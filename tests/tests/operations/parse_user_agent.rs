// Tests for the parse_user_agent operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations parse_user_agent::
//
// Browser name and version are objectively derivable from the user-agent
// string itself, so those are asserted as exact values. Device type, vendor
// and engine are heuristics that depend on the parser's database: rx-chef and
// upstream CyberChef (which uses ua-parser-js) disagree on several of them,
// recorded as a REFERENCE_IMPLEMENTATION_DIFFERENCE in
// verification/differential-triage.json rather than "fixed" in either
// direction.

use rxchef::runtime::{self, RuntimeError};

fn parse(agent: &str) -> String {
    String::from_utf8(
        runtime::run_operation("Parse User Agent", agent.as_bytes().to_vec(), &[])
            .expect("parsing must succeed"),
    )
    .unwrap()
}

fn try_parse(agent: &[u8]) -> Result<Vec<u8>, RuntimeError> {
    runtime::run_operation("Parse User Agent", agent.to_vec(), &[])
}

/// Read one reported field, e.g. `field(&out, "Name")`.
fn field(output: &str, name: &str) -> String {
    output
        .lines()
        .find_map(|line| {
            let (key, value) = line.split_once(':')?;
            (key.trim() == name).then(|| value.trim().to_string())
        })
        .unwrap_or_else(|| panic!("no {name} field in:\n{output}"))
}

const CHROME: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                      (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
const FIREFOX: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0";
const IPHONE: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) \
                      AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1";

#[test]
fn test_parse_user_agent_reads_the_browser_token_from_the_string() {
    // "Chrome/120.0.0.0" is in the string; nothing heuristic about it.
    let output = parse(CHROME);
    assert_eq!(field(&output, "Name"), "Chrome");
    assert_eq!(field(&output, "Version"), "120.0.0.0");
}

#[test]
fn test_parse_user_agent_reads_firefox() {
    let output = parse(FIREFOX);
    assert_eq!(field(&output, "Name"), "Firefox");
    assert_eq!(field(&output, "Version"), "121.0");
}

#[test]
fn test_parse_user_agent_prefers_the_version_token_on_safari() {
    // Safari reports its own release in "Version/17.0", with Safari/604.1
    // being the WebKit build. The former is the browser version.
    let output = parse(IPHONE);
    assert_eq!(field(&output, "Version"), "17.0");
}

#[test]
fn test_parse_user_agent_detects_a_mobile_device() {
    let output = parse(IPHONE);
    assert_eq!(field(&output, "Type"), "mobile");
}

#[test]
fn test_parse_user_agent_reports_every_section() {
    let output = parse(CHROME);
    for section in ["Browser", "Device", "Engine", "OS"] {
        assert!(
            output.contains(section),
            "missing {section} section in:\n{output}"
        );
    }
}

#[test]
fn test_parse_user_agent_reports_unknown_for_an_unrecognised_agent() {
    let output = parse("curl/8.4.0");
    assert_eq!(field(&output, "Name"), "unknown");
    assert_eq!(field(&output, "Version"), "unknown");
}

#[test]
fn test_parse_user_agent_empty_input() {
    // Every field falls back to "unknown" rather than failing.
    let output = parse("");
    assert!(output.contains("unknown"), "unexpected output: {output}");
}

#[test]
fn test_parse_user_agent_handles_a_very_long_agent_string() {
    // Boundary: a pathological agent must not blow up or hang.
    let long = format!("Mozilla/5.0 ({}) Chrome/120.0.0.0", "x".repeat(20_000));
    let output = parse(&long);
    assert_eq!(field(&output, "Name"), "Chrome");
}

#[test]
fn test_parse_user_agent_accepts_binary_input_without_panicking() {
    // Agents arrive from untrusted sources; invalid UTF-8 must not panic.
    let binary: Vec<u8> = (0u8..=255).collect();
    let result = try_parse(&binary);
    assert!(
        result.is_ok() || result.is_err(),
        "the call must return rather than panic"
    );
}

#[test]
fn test_parse_user_agent_is_deterministic() {
    assert_eq!(parse(CHROME), parse(CHROME));
}
