//! Output rendering and writing.
//!
//! Every command emits its result through this module so binary output, hex
//! output, and JSON envelopes behave identically everywhere.

use std::fs;
use std::io::{self, IsTerminal, Write};

use crate::cli::OutputFormat;
use crate::steps::{RunResult, Step};

pub(crate) fn write_output(output: &[u8], hex: bool) -> Result<(), String> {
    write_formatted_output(
        output,
        if hex {
            OutputFormat::Hex
        } else {
            OutputFormat::Auto
        },
        None,
    )
}

pub(crate) fn write_formatted_output(
    output: &[u8],
    format: OutputFormat,
    output_file: Option<&std::path::Path>,
) -> Result<(), String> {
    if let Some(path) = output_file {
        return atomic_write(path, output);
    }
    if format == OutputFormat::Json {
        return write_json_output(output);
    }

    let stdout = io::stdout();
    let mut out = stdout.lock();
    match format {
        OutputFormat::Raw => return write_bytes(&mut out, output),
        OutputFormat::Text => {
            let text = std::str::from_utf8(output)
                .map_err(|error| format!("output is not valid UTF-8: {error}"))?;
            return write_bytes(&mut out, text.as_bytes());
        }
        OutputFormat::Hex => return write_output_raw(output, true, &mut out),
        OutputFormat::Base64 => {
            use base64::{engine::general_purpose, Engine as _};
            return write_bytes(
                &mut out,
                general_purpose::STANDARD.encode(output).as_bytes(),
            );
        }
        OutputFormat::Json => unreachable!("JSON handled before locking stdout"),
        OutputFormat::Auto => {}
    }

    // Redirects and pipes are byte transports. TTY presentation must never
    // alter the data a downstream process receives.
    if !io::stdout().is_terminal() {
        return write_bytes(&mut out, output);
    }

    let safe_text = std::str::from_utf8(output).ok().filter(|text| {
        text.chars()
            .all(|character| !character.is_control() || matches!(character, '\n' | '\r' | '\t'))
    });
    if let Some(text) = safe_text {
        write_bytes(&mut out, text.as_bytes())?;
        if !output.ends_with(b"\n") {
            write_bytes(&mut out, b"\n")?;
        }
    } else {
        write_output_raw(output, true, &mut out)?;
        eprintln!("rxchef: binary output shown as hex; use a pipe/redirect for exact bytes");
    }
    Ok(())
}

pub(crate) fn atomic_write(path: &std::path::Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("invalid output path '{}'", path.display()))?;
    let temporary = parent.join(format!(".{file_name}.rxchef-{}.tmp", std::process::id()));
    let result = (|| {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|error| format!("cannot create '{}': {error}", temporary.display()))?;
        file.write_all(bytes)
            .map_err(|error| format!("cannot write '{}': {error}", temporary.display()))?;
        file.flush()
            .map_err(|error| format!("cannot flush '{}': {error}", temporary.display()))?;
        file.sync_all()
            .map_err(|error| format!("cannot sync '{}': {error}", temporary.display()))?;
        fs::rename(&temporary, path).map_err(|error| {
            format!(
                "cannot replace '{}' with completed output: {error}",
                path.display()
            )
        })
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

pub(crate) fn write_json_output(output: &[u8]) -> Result<(), String> {
    use base64::{engine::general_purpose, Engine as _};
    let json = serde_json::json!({
        "schema_version": 1,
        "success": true,
        "output": String::from_utf8_lossy(output),
        "output_base64": general_purpose::STANDARD.encode(output),
        "output_len": output.len(),
        "output_is_utf8": std::str::from_utf8(output).is_ok(),
    });
    let mut encoded = serde_json::to_vec(&json).map_err(|error| error.to_string())?;
    encoded.push(b'\n');
    write_bytes(&mut io::stdout().lock(), &encoded)
}

pub(crate) fn write_json_pipe_output(
    result: &RunResult,
    trace_steps: Option<&[Step]>,
) -> Result<(), String> {
    use base64::{engine::general_purpose, Engine as _};
    let mut json = serde_json::json!({
        "schema_version": 1,
        "success": true,
        "output": String::from_utf8_lossy(&result.final_output),
        "output_base64": general_purpose::STANDARD.encode(&result.final_output),
        "output_len": result.final_output.len(),
        "output_is_utf8": std::str::from_utf8(&result.final_output).is_ok(),
    });
    if let Some(steps) = trace_steps {
        let step_arr: Vec<_> = result
            .steps
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let op_name = steps.get(i).map(|st| st.op.as_str()).unwrap_or("?");
                serde_json::json!({
                    "step": i + 1,
                    "op": op_name,
                    "output": &s.output_preview,
                    "output_bytes": s.output_bytes,
                    "error": s.error,
                })
            })
            .collect();
        json["steps"] = serde_json::json!(step_arr);
    }
    let mut encoded = serde_json::to_vec(&json).map_err(|error| error.to_string())?;
    encoded.push(b'\n');
    write_bytes(&mut io::stdout().lock(), &encoded)
}

pub(crate) fn write_output_raw<W: Write>(
    output: &[u8],
    hex: bool,
    w: &mut W,
) -> Result<(), String> {
    if hex {
        for (i, b) in output.iter().enumerate() {
            if i > 0 && i % 16 == 0 {
                w.write_all(b"\n").map_err(|e| e.to_string())?;
            } else if i > 0 {
                w.write_all(b" ").map_err(|e| e.to_string())?;
            }
            write!(w, "{:02x}", b).map_err(|e| e.to_string())?;
        }
        w.write_all(b"\n").map_err(|e| e.to_string())
    } else {
        match std::str::from_utf8(output) {
            Ok(s) => w.write_all(s.as_bytes()).map_err(|e| e.to_string()),
            Err(_) => {
                // Binary: hex dump
                for (i, b) in output.iter().enumerate() {
                    if i > 0 && i % 16 == 0 {
                        w.write_all(b"\n").map_err(|e| e.to_string())?;
                    } else if i > 0 {
                        w.write_all(b" ").map_err(|e| e.to_string())?;
                    }
                    write!(w, "{:02x}", b).map_err(|e| e.to_string())?;
                }
                w.write_all(b"\n").map_err(|e| e.to_string())
            }
        }
    }
}

pub(crate) fn write_bytes<W: Write>(writer: &mut W, bytes: &[u8]) -> Result<(), String> {
    match writer.write_all(bytes) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

// ─── Project ──────────────────────────────────────────────────────────────────
