// Tests for the plist_viewer operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations plist_viewer::

use rxchef::operations::plist_viewer::PLISTViewer;
use rxchef::Operation;

#[test]
fn test_plist_viewer_basic() {
    let op = PLISTViewer;
    let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>Name</key>
	<string>John Doe</string>
	<key>Age</key>
	<integer>30</integer>
</dict>
</plist>"#.as_bytes().to_vec();
    let result = op.run(input, &[]).unwrap();
    let result_str = String::from_utf8(result).unwrap();
    assert!(result_str.contains("plist =>"));
    assert!(result_str.contains("Name => \"John Doe\""));
    assert!(result_str.contains("Age => 30"));
}
