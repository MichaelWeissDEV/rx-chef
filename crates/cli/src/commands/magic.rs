//! `rxchef magic` — recursive detect-and-decode.

use crate::cli::MagicArgs;
use crate::{input, output};

pub(crate) fn cmd_magic(a: MagicArgs) -> Result<(), String> {
    use rxchef::magic::{magic, MagicOptions};

    let input = input::load_input_from(a.input, a.input_file, &[])?.bytes;
    let crib = match &a.crib {
        Some(c) => {
            Some(regex::Regex::new(c).map_err(|e| format!("invalid crib regex '{c}': {e}"))?)
        }
        None => None,
    };
    let opts = MagicOptions {
        depth: a.depth,
        crib,
        intensive: a.intensive,
        max_results: 20,
        max_candidates: a.max_candidates,
        max_candidate_bytes: a.max_candidate_bytes,
        max_total_decoded_bytes: a.max_total_decoded_bytes,
    };
    let results = magic(&input, &opts);

    // --decode: emit only the winning plaintext, raw. Nothing else on stdout.
    if a.decode {
        return match results.first() {
            Some(m) => output::write_output(&m.data, a.hex),
            None => Err("magic: no decoding found".into()),
        };
    }

    if a.json {
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
        return Ok(());
    }

    if results.is_empty() {
        println!("No candidate decodings found.");
        if !input.is_empty() {
            eprintln!(
                "hint: try --intensive for aggressive decoders, or --depth for deeper chains"
            );
        }
        return Ok(());
    }

    println!("{} candidate decoding(s), best first:\n", results.len());
    for (i, m) in results.iter().enumerate() {
        let recipe = format_recipe(&m.recipe);
        let crib_tag = if m.matched_crib { "  ✓crib" } else { "" };
        println!(
            "{:>2}. {}  [entropy {:.2}]{}",
            i + 1,
            recipe,
            m.entropy,
            crib_tag
        );
        println!("    {}", m.preview);
    }
    eprintln!("\nRe-run one recipe with:  rxchef pipe \"...\" --input <data>");
    Ok(())
}

pub(crate) fn format_recipe(steps: &[rxchef::magic::RecipeStep]) -> String {
    steps
        .iter()
        .map(|s| {
            if s.args.is_empty() {
                s.op.clone()
            } else {
                format!("{}({})", s.op, s.args.join(","))
            }
        })
        .collect::<Vec<_>>()
        .join(" → ")
}

// ─── Scan ─────────────────────────────────────────────────────────────────────
