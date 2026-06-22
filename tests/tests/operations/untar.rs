// Tests for the untar operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations untar::

use rxchef::operations::untar::Untar;
use rxchef::Operation;
use tar::Builder;

#[test]
fn test_untar_empty() {
    let op = Untar;
    let result = op.run(vec![], &[]).unwrap();
    assert_eq!(result, b"No files found in tarball.");
}
#[test]
fn test_untar_single_file() {
    let op = Untar;
    let mut builder = Builder::new(Vec::new());
    let mut header = tar::Header::new_gnu();
    header.set_size(5);
    header.set_path("test.txt").unwrap();
    header.set_cksum();
    builder.append(&header, b"hello".as_ref()).unwrap();
    let input = builder.into_inner().unwrap();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("File: test.txt"));
    assert!(result_str.contains("hello"));
}
#[test]
fn test_untar_multiple_files() {
    let op = Untar;
    let mut builder = Builder::new(Vec::new());
    let mut h1 = tar::Header::new_gnu();
    h1.set_size(5);
    h1.set_path("a.txt").unwrap();
    h1.set_cksum();
    builder.append(&h1, b"aaaaa".as_ref()).unwrap();
    let mut h2 = tar::Header::new_gnu();
    h2.set_size(5);
    h2.set_path("b.txt").unwrap();
    h2.set_cksum();
    builder.append(&h2, b"bbbbb".as_ref()).unwrap();
    let input = builder.into_inner().unwrap();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8_lossy(&result);
    assert!(result_str.contains("File: a.txt"));
    assert!(result_str.contains("aaaaa"));
    assert!(result_str.contains("File: b.txt"));
    assert!(result_str.contains("bbbbb"));
}
