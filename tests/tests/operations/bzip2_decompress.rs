// Tests for the bzip2_decompress operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bzip2_decompress::

use rxchef::operation::ArgValue;
use rxchef::operations::bzip2_decompress::Bzip2Decompress;
use rxchef::Operation;

#[test]
fn test_bzip2_decompress_basic() {
    let op = Bzip2Decompress;
    
    // Create some test data and compress it first
    let test_data = b"Hello, world! This is a test string for bzip2 decompression.";
    let compress_op = rxchef::operations::bzip2_compress::Bzip2Compress;
    let compressed = compress_op.run(test_data.to_vec(), &[ArgValue::Num(9.0), ArgValue::Num(30.0)]);
    
    assert!(compressed.is_ok(), "Should be able to compress test data first");
    
    let args = [ArgValue::Bool(false)]; // Don't use low-memory algorithm
    let result = op.run(compressed.unwrap(), &args);
    
    // Should successfully decompress the data
    assert!(result.is_ok(), "Bzip2 decompression should succeed");
    if let Ok(decompressed) = result {
        assert!(!decompressed.is_empty(), "Decompressed output should not be empty");
        // Decompressed data should match original
        assert_eq!(decompressed, test_data, "Decompressed data should match original");
    }
}

#[test]
fn test_bzip2_decompress_empty_input() {
    let op = Bzip2Decompress;
    let args = [ArgValue::Bool(false)];
    let result = op.run(vec![], &args);
    
    // Should return error for empty input
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Please provide an input"));
    }
}

#[test]
fn test_bzip2_decompress_invalid_data() {
    let op = Bzip2Decompress;
    let invalid_data = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]; // Not valid bzip2
    let args = [ArgValue::Bool(false)];
    
    let result = op.run(invalid_data, &args);
    
    // Should return error for invalid bzip2 data
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Bzip2 decompression failed"));
    }
}

#[test]
fn test_bzip2_decompress_low_memory() {
    let op = Bzip2Decompress;
    
    // Create some test data and compress it first
    let test_data = b"Hello, world! This is a test string for bzip2 decompression.";
    let compress_op = rxchef::operations::bzip2_compress::Bzip2Compress;
    let compressed = compress_op.run(test_data.to_vec(), &[ArgValue::Num(9.0), ArgValue::Num(30.0)]);
    
    assert!(compressed.is_ok(), "Should be able to compress test data first");
    
    let args = [ArgValue::Bool(true)]; // Use low-memory algorithm
    let result = op.run(compressed.unwrap(), &args);
    
    // Should successfully decompress with low-memory algorithm
    assert!(result.is_ok(), "Bzip2 decompression with low-memory should succeed");
    if let Ok(decompressed) = result {
        assert_eq!(decompressed, test_data, "Decompressed data should match original");
    }
}

#[test]
fn test_bzip2_decompress_roundtrip() {
    let op = Bzip2Decompress;
    let compress_op = rxchef::operations::bzip2_compress::Bzip2Compress;
    
    // Test data of various sizes
    let test_cases = vec![
        b"Short".to_vec(),
        b"This is a medium length test string for compression.".to_vec(),
        vec![0u8; 1024], // 1KB of zeros
    ];
    
    for test_data in test_cases {
        // Compress
        let compressed = compress_op.run(test_data.clone(), &[ArgValue::Num(5.0), ArgValue::Num(30.0)]);
        assert!(compressed.is_ok(), "Compression should succeed");
        
        // Decompress
        let args = [ArgValue::Bool(false)];
        let decompressed = op.run(compressed.unwrap(), &args);
        assert!(decompressed.is_ok(), "Decompression should succeed");
        
        // Verify roundtrip
        assert_eq!(decompressed.unwrap(), test_data, "Roundtrip should preserve data");
    }
}
