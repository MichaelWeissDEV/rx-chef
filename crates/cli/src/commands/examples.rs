//! Display verified recipes and practical usage examples: `rxchef examples`

pub(crate) fn cmd_examples() -> Result<(), String> {
    println!("rx-chef Usage Examples & Common Recipes");
    println!("========================================");

    let examples = [
        (
            "Base64 Decode & Hex Dump",
            "rxchef pipe 'from_base64' 'to_hex' --input 'SGVsbG8gV29ybGQ='",
            "Decodes Base64 string and displays hexadecimal representation of the raw bytes.",
        ),
        (
            "Deflate Compression & Base64",
            "echo -n 'High entropy compressed payload' | rxchef pipe 'deflate' 'to_base64'",
            "Compresses text with raw Deflate and encodes the resulting binary stream to Base64.",
        ),
        (
            "Cryptographic Hash (SHA256)",
            "rxchef run 'sha256' --input 'CyberChef'",
            "Computes SHA-256 digest of input text.",
        ),
        (
            "Fork & Process Lines Independently",
            "rxchef pipe 'fork,\\n,\\n' 'to_upper_case' 'merge' --input 'apple\\nbanana\\ncherry'",
            "Splits on newlines, transforms each subsection, and merges back with newlines.",
        ),
        (
            "Automatic Detection with Magic",
            "rxchef magic --input 'VGhlIGFuc3dlciBpcyA0Mg==' --decode",
            "Recursively probes encoding layers and outputs decoded result.",
        ),
        (
            "Interactive Terminal Workbench (TUI)",
            "rxchef_tui   (or rxchef tui)",
            "Launches interactive full-terminal workbench for building and debugging recipes.",
        ),
    ];

    for (title, cmd, desc) in examples {
        println!("\n▶ {}", title);
        println!("  Command:     {}", cmd);
        println!("  Description: {}", desc);
    }
    println!();
    Ok(())
}
