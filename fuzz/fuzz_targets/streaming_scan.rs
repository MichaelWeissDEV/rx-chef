#![no_main]

use libfuzzer_sys::fuzz_target;
use rxchef::scan::{ScanOptions, Scanner};

fuzz_target!(|data: &[u8]| {
    let options = ScanOptions {
        min_len: 1,
        max_len: 4096,
        max_findings: 32,
        ..ScanOptions::default()
    };
    let mut scanner = Scanner::new(options);
    let split = data.len() / 2;
    let mut findings = Vec::new();
    scanner.push(&data[..split], &mut |finding| findings.push(finding));
    scanner.push(&data[split..], &mut |finding| findings.push(finding));
    scanner.finish(&mut |finding| findings.push(finding));
});
