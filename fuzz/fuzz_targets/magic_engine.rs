#![no_main]

use libfuzzer_sys::fuzz_target;
use rxchef::magic::{magic, MagicOptions};

fuzz_target!(|data: &[u8]| {
    let input = &data[..data.len().min(4096)];
    let options = MagicOptions {
        depth: 3,
        max_results: 8,
        max_candidates: 32,
        max_candidate_bytes: 16 * 1024,
        max_total_decoded_bytes: 64 * 1024,
        ..MagicOptions::default()
    };
    let _ = magic(input, &options);
});
