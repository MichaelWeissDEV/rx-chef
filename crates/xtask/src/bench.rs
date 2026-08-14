//! `cargo run -p xtask -- bench` — a small, dependency-light throughput
//! harness for representative rxchef operations.
//!
//! This is intentionally hand-rolled (`std::time::Instant`) rather than built
//! on `criterion`: the numbers only need to be good enough to sanity-check
//! "is this operation roughly O(n) and roughly this fast", and a plain
//! warmup + repeated-sample + median approach is sufficient for that without
//! adding a heavy dependency to the workspace. See
//! `docs/development/benchmarking.md` for how to read the output and how to
//! add a new case, and `docs/performance/methodology.md` for what is
//! deliberately out of scope (statistical significance testing, multi-machine
//! comparison, process-startup/I/O costs).
//!
//! IMPORTANT: this must be run against a release build. Debug builds carry
//! overflow checks, disabled inlining, and no LTO, so their timings are not
//! representative of anything a user would experience. `run` below refuses
//! to produce numbers under a debug build.

use std::time::Instant;
use std::{env, fs, path::PathBuf, process::Command};

use rxchef::magic::{magic, MagicOptions};
use rxchef::runtime;
use rxchef::scan::{scan_bytes, ScanOptions};

/// One warmup-then-sample measurement of a single operation.
struct Measurement {
    name: &'static str,
    category: &'static str,
    input_bytes: usize,
    warmup_iters: usize,
    sample_iters: usize,
    /// Wall-clock nanoseconds for each sample, in the order they were taken.
    samples_ns: Vec<f64>,
}

impl Measurement {
    fn sorted_ns(&self) -> Vec<f64> {
        let mut sorted = self.samples_ns.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted
    }

    fn median_ns(&self) -> f64 {
        median(&self.sorted_ns())
    }

    /// Median absolute deviation (ns) — a spread measure that, unlike
    /// stdev, is not dominated by a single slow outlier sample (GC pause,
    /// OS scheduler hiccup, thermal throttle blip, etc.).
    fn mad_ns(&self) -> f64 {
        let med = self.median_ns();
        let mut deviations: Vec<f64> = self.samples_ns.iter().map(|v| (v - med).abs()).collect();
        deviations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        median(&deviations)
    }

    fn min_ns(&self) -> f64 {
        self.sorted_ns().first().copied().unwrap_or(0.0)
    }

    fn max_ns(&self) -> f64 {
        self.sorted_ns().last().copied().unwrap_or(0.0)
    }

    fn p95_ns(&self) -> f64 {
        let sorted = self.sorted_ns();
        let index = ((sorted.len() as f64 * 0.95).ceil() as usize)
            .saturating_sub(1)
            .min(sorted.len().saturating_sub(1));
        sorted.get(index).copied().unwrap_or(0.0)
    }

    fn throughput_mib_s(&self) -> f64 {
        let seconds = self.median_ns() / 1e9;
        if seconds <= 0.0 || self.input_bytes == 0 {
            return 0.0;
        }
        (self.input_bytes as f64 / (1024.0 * 1024.0)) / seconds
    }
}

fn median(sorted: &[f64]) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let n = sorted.len();
    if n % 2 == 1 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    }
}

fn human_ns(ns: f64) -> String {
    if ns >= 1e9 {
        format!("{:.3} s", ns / 1e9)
    } else if ns >= 1e6 {
        format!("{:.3} ms", ns / 1e6)
    } else if ns >= 1e3 {
        format!("{:.3} us", ns / 1e3)
    } else {
        format!("{:.0} ns", ns)
    }
}

/// Deterministic, dependency-free byte generator (xorshift64*). Not
/// cryptographic; only used to produce reproducible, non-trivially-compressible
/// benchmark inputs without pulling in `rand` as an xtask dependency.
fn deterministic_bytes(len: usize, seed: u64) -> Vec<u8> {
    let mut state = seed | 1;
    let mut out = Vec::with_capacity(len);
    for _ in 0..len {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        // xorshift64* scramble so low bytes aren't a simple LFSR pattern.
        let scrambled = state.wrapping_mul(0x2545_F491_4F6C_DD1D);
        out.push((scrambled >> 24) as u8);
    }
    out
}

/// Mostly-printable-ish text, for operations (gzip, magic) where highly
/// compressible / structured input is more representative than pure noise.
fn deterministic_text(len: usize, seed: u64) -> Vec<u8> {
    const WORDS: &[&str] = &[
        "the",
        "quick",
        "brown",
        "fox",
        "jumps",
        "over",
        "lazy",
        "dog",
        "rxchef",
        "operation",
        "pipeline",
        "recipe",
        "magic",
        "scan",
        "entropy",
        "decode",
        "encode",
        "token",
        "cipher",
        "hash",
    ];
    let mut state = seed | 1;
    let mut out = Vec::with_capacity(len + 16);
    while out.len() < len {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let word = WORDS[(state as usize) % WORDS.len()];
        out.extend_from_slice(word.as_bytes());
        out.push(b' ');
    }
    out.truncate(len);
    out
}

fn run_case<F>(
    name: &'static str,
    category: &'static str,
    input_bytes: usize,
    warmup_iters: usize,
    sample_iters: usize,
    mut f: F,
) -> Result<Measurement, String>
where
    F: FnMut() -> Result<(), String>,
{
    for _ in 0..warmup_iters {
        f()?;
    }
    let mut samples_ns = Vec::with_capacity(sample_iters);
    for _ in 0..sample_iters {
        let start = Instant::now();
        f()?;
        samples_ns.push(start.elapsed().as_secs_f64() * 1e9);
    }
    Ok(Measurement {
        name,
        category,
        input_bytes,
        warmup_iters,
        sample_iters,
        samples_ns,
    })
}

const CODEC_WARMUP: usize = 5;
const CODEC_SAMPLES: usize = 20;
const ENGINE_WARMUP: usize = 3;
const ENGINE_SAMPLES: usize = 10;

fn build_cases(full: bool) -> Result<Vec<Measurement>, String> {
    let mut out = Vec::new();

    // ── Codecs: 1 MiB of deterministic noise ────────────────────────────────
    let codec_payload = deterministic_bytes(1 << 20, 0x9E37_79B9_7F4A_7C15);

    out.push(run_case(
        "To Hex",
        "codec",
        codec_payload.len(),
        CODEC_WARMUP,
        CODEC_SAMPLES,
        || {
            runtime::run_operation(
                "To Hex",
                codec_payload.clone(),
                &["None".to_string(), "0".to_string()],
            )
            .map(|_| ())
            .map_err(|error| error.to_string())
        },
    )?);

    if full {
        for (name, size) in [
            ("To Base64 (1 KiB)", 1 << 10),
            ("To Base64 (16 MiB)", 16 << 20),
        ] {
            let payload = deterministic_bytes(size, size as u64);
            out.push(run_case(
                name,
                "codec",
                payload.len(),
                CODEC_WARMUP,
                CODEC_SAMPLES,
                || {
                    runtime::run_operation("To Base64", payload.clone(), &[])
                        .map(|_| ())
                        .map_err(|error| error.to_string())
                },
            )?);
        }
        for (name, size) in [
            ("SHA2-256 (1 KiB)", 1 << 10),
            ("SHA2-256 (16 MiB)", 16 << 20),
        ] {
            let payload = deterministic_bytes(size, !(size as u64));
            out.push(run_case(
                name,
                "hash",
                payload.len(),
                CODEC_WARMUP,
                CODEC_SAMPLES,
                || {
                    runtime::run_operation("SHA2", payload.clone(), &[])
                        .map(|_| ())
                        .map_err(|error| error.to_string())
                },
            )?);
        }
    }

    out.push(run_case(
        "To Base64",
        "codec",
        codec_payload.len(),
        CODEC_WARMUP,
        CODEC_SAMPLES,
        || {
            runtime::run_operation("To Base64", codec_payload.clone(), &[])
                .map(|_| ())
                .map_err(|error| error.to_string())
        },
    )?);

    // ── Hash: SHA2-256 over 1 MiB ────────────────────────────────────────────
    out.push(run_case(
        "SHA2 (256)",
        "hash",
        codec_payload.len(),
        CODEC_WARMUP,
        CODEC_SAMPLES,
        || {
            runtime::run_operation("SHA2", codec_payload.clone(), &[])
                .map(|_| ())
                .map_err(|error| error.to_string())
        },
    )?);

    // ── Cipher: AES-256-CBC over 1 MiB ──────────────────────────────────────
    let aes_key = "hex:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
    let aes_iv = "hex:00112233445566778899aabbccddeeff";
    out.push(run_case(
        "AES Encrypt (CBC-256)",
        "cipher",
        codec_payload.len(),
        CODEC_WARMUP,
        CODEC_SAMPLES,
        || {
            runtime::run_operation(
                "AES Encrypt",
                codec_payload.clone(),
                &[
                    aes_key.to_string(),
                    aes_iv.to_string(),
                    "CBC".to_string(),
                    "Raw".to_string(),
                    "Hex".to_string(),
                ],
            )
            .map(|_| ())
            .map_err(|error| error.to_string())
        },
    )?);

    // ── Compressor: Gzip over 1 MiB of word-like text (more representative
    //    of real payloads than pure noise, which is already ~incompressible) ─
    let gzip_payload = deterministic_text(1 << 20, 0xD1B5_4A32_D192_ED03);
    out.push(run_case(
        "Gzip Compress",
        "compression",
        gzip_payload.len(),
        CODEC_WARMUP,
        CODEC_SAMPLES,
        || {
            runtime::run_operation("Gzip", gzip_payload.clone(), &[])
                .map(|_| ())
                .map_err(|error| error.to_string())
        },
    )?);

    // ── Magic engine: a small layered payload (base64, then hex) at the
    //    engine's default depth/intensive settings. Magic explores many
    //    candidate decode branches per byte, so its useful input sizes are
    //    much smaller than a raw codec's. ───────────────────────────────────
    let magic_secret = b"the quick brown fox jumps over the lazy dog 1234567890";
    let magic_b64 = runtime::run_operation("To Base64", magic_secret.to_vec(), &[])
        .map_err(|error| error.to_string())?;
    let magic_payload =
        runtime::run_operation("To Hex", magic_b64, &["None".to_string(), "0".to_string()])
            .map_err(|error| error.to_string())?;
    out.push(run_case(
        "Magic (depth 3, hex(base64(text)))",
        "engine",
        magic_payload.len(),
        ENGINE_WARMUP,
        ENGINE_SAMPLES,
        || {
            let opts = MagicOptions::default();
            let _ = magic(&magic_payload, &opts);
            Ok(())
        },
    )?);

    // ── Scan engine: a 256 KiB buffer with base64/hex tokens seeded through
    //    noise, decode enabled so the magic engine runs per-token. ─────────
    let scan_payload = build_scan_corpus(256 * 1024);
    out.push(run_case(
        "Scan (tokenize + decode)",
        "engine",
        scan_payload.len(),
        ENGINE_WARMUP,
        ENGINE_SAMPLES,
        || {
            let opts = ScanOptions {
                min_len: 12,
                decode: true,
                ..ScanOptions::default()
            };
            let _ = scan_bytes(&scan_payload, opts);
            Ok(())
        },
    )?);

    Ok(out)
}

/// Build a synthetic buffer of noise interspersed with base64/hex tokens, so
/// the scan engine has real work to tokenize and decode, at a predictable
/// total size.
fn build_scan_corpus(target_len: usize) -> Vec<u8> {
    let b64_token = "SGVsbG8gZnJvbSB0aGUgcnhjaGVmIGJlbmNobWFyayBoYXJuZXNzIQ==";
    let hex_token = "72786368656620697320612043796265724368656620706f7274";
    let mut out = Vec::with_capacity(target_len + 256);
    let mut state: u64 = 0xA5A5_5A5A_1234_5678;
    while out.len() < target_len {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        out.extend_from_slice(b"noise ");
        out.extend_from_slice(&(state.to_string()).into_bytes());
        out.extend_from_slice(b" ");
        if state % 2 == 0 {
            out.extend_from_slice(b64_token.as_bytes());
        } else {
            out.extend_from_slice(hex_token.as_bytes());
        }
        out.extend_from_slice(b" filler text between tokens keeps offsets moving ");
    }
    out.truncate(target_len);
    out
}

fn print_human(measurements: &[Measurement]) {
    println!(
        "{:<38} {:>10} {:>12} {:>12} {:>12} {:>12}",
        "operation", "input", "median", "min", "max", "MiB/s"
    );
    for m in measurements {
        println!(
            "{:<38} {:>10} {:>12} {:>12} {:>12} {:>12.2}",
            format!("{} [{}]", m.name, m.category),
            format!("{} B", m.input_bytes),
            human_ns(m.median_ns()),
            human_ns(m.min_ns()),
            human_ns(m.max_ns()),
            m.throughput_mib_s(),
        );
        println!(
            "  {} warmup / {} samples, MAD {}",
            m.warmup_iters,
            m.sample_iters,
            human_ns(m.mad_ns())
        );
    }
}

fn print_json(measurements: &[Measurement]) {
    let root = serde_json::json!({
        "debug_assertions": cfg!(debug_assertions),
        "results": measurement_values(measurements),
    });
    println!("{}", serde_json::to_string_pretty(&root).unwrap());
}

fn measurement_values(measurements: &[Measurement]) -> Vec<serde_json::Value> {
    measurements
        .iter()
        .map(|m| {
            serde_json::json!({
                "name": m.name,
                "operation": m.name,
                "category": m.category,
                "case": format!("{}-bytes", m.input_bytes),
                "input_bytes": m.input_bytes,
                "warmup_iters": m.warmup_iters,
                "sample_iters": m.sample_iters,
                "median_ns": m.median_ns(),
                "p95_ns": m.p95_ns(),
                "min_ns": m.min_ns(),
                "max_ns": m.max_ns(),
                "mad_ns": m.mad_ns(),
                "throughput_mib_s": m.throughput_mib_s(),
                "throughput_bytes_per_sec": m.throughput_mib_s() * 1024.0 * 1024.0,
            })
        })
        .collect()
}

/// Public `cargo xtask bench-docs` entry point. It relaunches this task in a
/// release profile so the documented command cannot accidentally record debug
/// timings.
pub fn run_docs(args: &[String]) -> Result<(), String> {
    let mode = if args.iter().any(|arg| arg == "--full") {
        "--full"
    } else {
        "--quick"
    };
    if !cfg!(debug_assertions) {
        return run_docs_internal(&[mode.to_string()]);
    }
    let root =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").map_err(|e| e.to_string())?).join("../..");
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let status = Command::new(cargo)
        .current_dir(root)
        .args([
            "run",
            "--release",
            "-p",
            "xtask",
            "--",
            "bench-docs-internal",
            mode,
        ])
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err("release benchmark process failed".into())
    }
}

pub fn run_docs_internal(args: &[String]) -> Result<(), String> {
    if cfg!(debug_assertions) {
        return Err("benchmark documentation must run in release mode".into());
    }
    let mode = if args.iter().any(|arg| arg == "--full") {
        "full"
    } else {
        "quick"
    };
    let measurements = build_cases(mode == "full")?;
    let root =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").map_err(|e| e.to_string())?).join("../..");
    let output = root.join(
        if env::consts::OS == "linux" && env::consts::ARCH == "x86_64" {
            "benchmarks/results/linux-x86_64.json"
        } else {
            "benchmarks/results/host-unverified.json"
        },
    );
    fs::create_dir_all(output.parent().unwrap()).map_err(|e| e.to_string())?;
    let document = serde_json::json!({
        "schema_version": 1,
        "environment": {
            "commit": command_output("git", &["rev-parse", "HEAD"]),
            "rustc": command_output("rustc", &["--version"]),
            "os": env::consts::OS,
            "arch": env::consts::ARCH,
            "cpu": cpu_name(),
        },
        "profile": "release",
        "suite": mode,
        "disclaimer": "Reference measurement; hardware dependent; not a runtime guarantee.",
        "results": measurement_values(&measurements),
    });
    let encoded = serde_json::to_string_pretty(&document).map_err(|e| e.to_string())?;
    fs::write(&output, format!("{encoded}\n")).map_err(|e| e.to_string())?;
    println!("wrote {}", output.display());
    Ok(())
}

fn command_output(program: &str, args: &[&str]) -> String {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|output| !output.is_empty())
        .unwrap_or_else(|| "not detected".to_string())
}

fn cpu_name() -> String {
    if env::consts::OS == "macos" {
        command_output("sysctl", &["-n", "machdep.cpu.brand_string"])
    } else {
        fs::read_to_string("/proc/cpuinfo")
            .ok()
            .and_then(|contents| {
                contents
                    .lines()
                    .find_map(|line| line.strip_prefix("model name\t: ").map(str::to_owned))
            })
            .unwrap_or_else(|| command_output("uname", &["-m"]))
    }
}

/// Entry point for `cargo run -p xtask -- bench [--json]`.
pub fn run(args: &[String]) -> Result<(), String> {
    if cfg!(debug_assertions) {
        return Err(
            "xtask bench refuses to run in a debug build (debug_assertions is enabled). \
             Debug-build timings are not representative — overflow checks, no LTO, and \
             disabled inlining all skew the numbers. Run with --release: \
             `cargo run -p xtask --release -- bench`"
                .to_string(),
        );
    }

    let json = args.iter().any(|a| a == "--json");

    let measurements = build_cases(args.iter().any(|argument| argument == "--full"))?;

    if json {
        print_json(&measurements);
    } else {
        print_human(&measurements);
    }

    Ok(())
}
