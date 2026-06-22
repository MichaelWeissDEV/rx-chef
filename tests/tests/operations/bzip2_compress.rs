// Tests for the bzip2_compress operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations bzip2_compress::

use rxchef::operation::ArgValue;
use rxchef::operations::bzip2_compress::Bzip2Compress;
use rxchef::Operation;

#[test]
fn test_bzip2_compress_basic() {
    let op = Bzip2Compress;
    let input = b"Hello, world! This is a test string for bzip2 compression.".to_vec();
    let args = [ArgValue::Num(9.0), ArgValue::Num(30.0)]; // Best compression, default work factor
    
    let result = op.run(input.clone(), &args);
    
    // Should successfully compress the data
    assert!(result.is_ok(), "Bzip2 compression should succeed");
    if let Ok(compressed) = result {
        assert!(!compressed.is_empty(), "Compressed output should not be empty");
        // Note: For very small inputs, compressed data might not be smaller due to headers
        // Just verify it's not empty and compression succeeded
    }
}

#[test]
fn test_bzip2_compress_empty_input() {
    let op = Bzip2Compress;
    let args = [ArgValue::Num(9.0), ArgValue::Num(30.0)];
    let result = op.run(vec![], &args);
    
    // Should return error for empty input
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Please provide an input"));
    }
}

#[test]
fn test_bzip2_compress_different_block_sizes() {
    let op = Bzip2Compress;
    let input = b"Hello, world! This is a test string for bzip2 compression.".to_vec();
    
    // Test different block sizes
    for block_size in [1.0, 5.0, 9.0] {
        let args = [ArgValue::Num(block_size), ArgValue::Num(30.0)];
        let test_input = input.clone();
        let result = op.run(test_input, &args);
        
        assert!(result.is_ok(), "Bzip2 compression with block size {} should succeed", block_size);
        if let Ok(compressed) = result {
            assert!(!compressed.is_empty(), "Compressed output should not be empty");
        }
    }
}

#[test]
fn test_bzip2_compress_already_compressed() {
    let op = Bzip2Compress;
    // This is a simple test - in reality, already compressed data might not compress well
    let input = vec![0x42, 0x5A, 0x68, 0x39, 0x31, 0x41, 0x59, 0x26, 0x53, 0x59]; // BZh91AY&SY (bzip2 magic)
    let args = [ArgValue::Num(9.0), ArgValue::Num(30.0)];
    
    let result = op.run(input, &args);
    
    // Should still work, even if it doesn't compress well
    assert!(result.is_ok(), "Bzip2 compression should handle already compressed data");
}

#[test]
fn test_bzip2_compress_invalid_block_size() {
    let op = Bzip2Compress;
    let input = b"Hello, world!".to_vec();
    let args = [ArgValue::Num(100.0), ArgValue::Num(30.0)]; // Invalid block size
    
    let result = op.run(input, &args);
    
    // Should handle invalid block size gracefully (use best compression)
    assert!(result.is_ok(), "Bzip2 compression should handle invalid block size");
}
