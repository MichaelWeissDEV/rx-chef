// Tests for the mime_decoding operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations mime_decoding::

use rxchef::operations::mime_decoding::MIMEDecoding;
use rxchef::Operation;

#[test]
fn test_mime_decoding_base64() {
    let op = MIMEDecoding;
    let input = b"=?utf-8?B?SGVsbG8gV29ybGQ=?=".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "Hello World");
}
#[test]
fn test_mime_decoding_quoted_printable() {
    let op = MIMEDecoding;
    let input = b"=?iso-8859-1?Q?Keld_J=F8rn_Simonsen?=".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(String::from_utf8(result).unwrap(), "Keld Jørn Simonsen");
}
#[test]
fn test_mime_decoding_mixed() {
    let op = MIMEDecoding;
    let input = b"Subject: =?utf-8?Q?Test_Message?= and more".to_vec();
    let result = op.run(input, &[]).unwrap();
    assert_eq!(
        String::from_utf8(result).unwrap(),
        "Subject: Test Message and more"
    );
}
