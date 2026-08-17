//! `rxchef serve` — persistent JSONL / JSON-RPC transport.

use std::io::{self, BufReader};

use crate::cli::ServeArgs;

pub(crate) fn cmd_serve(a: ServeArgs) -> Result<(), String> {
    if !a.stdio {
        return Err("only --stdio transport is currently supported".into());
    }
    let stdin = io::stdin();
    let stdout = io::stdout();
    rxchef::integration::serve_jsonl_with_limit(
        BufReader::new(stdin.lock()),
        stdout.lock(),
        a.max_request_bytes,
    )
}

// ─── Pipeline management ─────────────────────────────────────────────────────
