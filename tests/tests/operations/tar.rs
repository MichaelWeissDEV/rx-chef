// Tests for the tar operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations tar::

use rxchef::operation::ArgValue;
use rxchef::operations::tar::Tar;
use rxchef::Operation;
use tar::Archive;

#[test]
fn test_tar_basic() {
    let op = Tar;
    let input = b"Hello World".to_vec();
    let args = [ArgValue::Str("test.txt".to_string())];
    let result = op.run(input.clone(), &args).unwrap();
    let mut archive = Archive::new(&result[..]);
    let mut entries = archive.entries().unwrap();
    let mut entry = entries.next().unwrap().unwrap();
    assert_eq!(entry.path().unwrap().to_str().unwrap(), "test.txt");
    assert_eq!(entry.header().size().unwrap(), input.len() as u64);
    let mut content = Vec::new();
    std::io::Read::read_to_end(&mut entry, &mut content).unwrap();
    assert_eq!(content, input);
}
#[test]
fn test_tar_filename() {
    let op = Tar;
    let input = b"data".to_vec();
    let args = [ArgValue::Str("mydata.bin".to_string())];
    let result = op.run(input, &args).unwrap();
    let mut archive = Archive::new(&result[..]);
    let mut entries = archive.entries().unwrap();
    let entry = entries.next().unwrap().unwrap();
    assert_eq!(entry.path().unwrap().to_str().unwrap(), "mydata.bin");
}
#[test]
fn test_tar_empty_input() {
    let op = Tar;
    let input = b"".to_vec();
    let args = [ArgValue::Str("empty.txt".to_string())];
    let result = op.run(input, &args).unwrap();
    let mut archive = Archive::new(&result[..]);
    let mut entries = archive.entries().unwrap();
    let entry = entries.next().unwrap().unwrap();
    assert_eq!(entry.header().size().unwrap(), 0);
}
