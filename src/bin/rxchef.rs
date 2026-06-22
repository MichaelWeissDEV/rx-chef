/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.1.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Minimal binary entry point for the rxchef library crate.
 * -----------------------------------------------------------------------------
 */

fn main() {
    let operation_count = rxchef::runtime::operation_names(None).len();
    println!(
        "rxchef library binary\n\n\
This package exposes the rxchef Rust library with {operation_count} registered operations.\n\
Use the frontend binaries for work:\n\
  rxchef_cli     command-line interface\n\
  rxchef_tui     interactive terminal interface\n\
  rxchef_gui     native desktop GUI\n\
  rxchef_webgui  local browser GUI\n\
  rxchef_server  HTTP API server"
    );
}
