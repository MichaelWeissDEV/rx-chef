// Tests for the rot8000 operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations rot8000::

use rxchef::operations::rot8000::ROT8000;
use rxchef::Operation;

/// Build ROT8000 translation table
fn build_rot8000_table() -> Vec<(char, char)> {
    // Transition points: (codepoint, is_start_of_valid_range)
    // false means start of invalid range, true means start of valid range
    let transitions: &[(u32, bool)] = &[
        (33, true),
        (127, false),
        (161, true),
        (5760, false),
        (5761, true),
        (8192, false),
        (8203, true),
        (8232, false),
        (8234, true),
        (8239, false),
        (8240, true),
        (8287, false),
        (8288, true),
        (12288, false),
        (12289, true),
        (55296, false),
        (57344, true),
    ];

    // Build sorted list of valid codepoints in BMP (0..0x10000)
    let bmp_size: u32 = 0x10000;
    let mut valid_list: Vec<u32> = Vec::new();
    let mut curr_valid = false;
    let mut trans_iter = transitions.iter().peekable();

    for cp in 0..bmp_size {
        // Check if transition happens at this codepoint
        if let Some(&&(t_cp, t_val)) = trans_iter.peek() {
            if cp == t_cp {
                curr_valid = t_val;
                trans_iter.next();
            }
        }
        if curr_valid {
            valid_list.push(cp);
        }
    }

    let total = valid_list.len();
    let rotate_num = total / 2;

    // Build mapping: valid_list[i] -> valid_list[(i + rotate_num) % total]
    let mut table: Vec<(char, char)> = Vec::with_capacity(total);
    for (i, &cp) in valid_list.iter().enumerate() {
        let target_cp = valid_list[(i + rotate_num) % total];
        if let (Some(from_c), Some(to_c)) = (char::from_u32(cp), char::from_u32(target_cp)) {
            table.push((from_c, to_c));
        }
    }
    table
}

#[test]
fn test_rot8000_involution() {
    // ROT8000 applied twice must return the original string
    let op = ROT8000;
    let input = "Hello, World! 123".as_bytes().to_vec();
    let once = op.run(input.clone(), &[]).unwrap();
    let twice = op.run(once, &[]).unwrap();
    assert_eq!(twice, input);
}
#[test]
fn test_rot8000_table_size() {
    let table = build_rot8000_table();
    // Must be even (rotated by half)
    assert!(table.len().is_multiple_of(2));
    assert!(!table.is_empty());
}
#[test]
fn test_rot8000_ascii_printable_rotated() {
    // Printable ASCII chars (33..=126) are in the valid set and should be rotated
    let op = ROT8000;
    let input = b"!".to_vec();
    let result = op.run(input.clone(), &[]).unwrap();
    // '!' should map to something different
    assert_ne!(result, input);
}
#[test]
fn test_rot8000_non_bmp_unchanged() {
    // Codepoints >= 0x10000 (surrogate area is invalid) pass through
    // Newline \n (0x0A) is not in valid set, should be unchanged
    let op = ROT8000;
    let input = b"\n".to_vec();
    let result = op.run(input.clone(), &[]).unwrap();
    assert_eq!(result, input);
}
