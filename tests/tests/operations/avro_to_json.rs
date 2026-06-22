// Tests for the avro_to_json operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations avro_to_json::

use rxchef::operation::ArgValue;
use rxchef::operations::avro_to_json::AvroToJSON;
use rxchef::Operation;

#[test]
fn test_avro_to_json_simple() {
    let op = AvroToJSON;
    
    // For now, just test that the operation handles invalid Avro data gracefully
    // In a real test, we would need proper Avro binary data
    let invalid_avro_data = vec![0x4f, 0x62, 0x6a, 0x01]; // Partial/invalid Avro
    
    let args = [ArgValue::Bool(true)]; // Force Valid JSON
    let result = op.run(invalid_avro_data, &args);
    
    // Should return an error for invalid Avro data
    assert!(result.is_err(), "Should return error for invalid Avro data");
}

#[test]
fn test_avro_to_json_empty_input() {
    let op = AvroToJSON;
    let args = [ArgValue::Bool(true)];
    let result = op.run(vec![], &args);
    
    // Should return an error for empty input
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Please provide an input"));
    }
}

#[test]
fn test_avro_to_json_invalid_avro() {
    let op = AvroToJSON;
    let invalid_data = vec![0x00, 0x01, 0x02, 0x03]; // Not valid Avro
    let args = [ArgValue::Bool(true)];
    let result = op.run(invalid_data, &args);
    
    // Should return an error for invalid Avro data
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Error parsing Avro"));
    }
}

#[test]
fn test_avro_to_json_force_json_false() {
    let op = AvroToJSON;
    
    // Test with invalid Avro data
    let invalid_avro_data = vec![0x4f, 0x62, 0x6a, 0x01]; // Partial/invalid Avro
    
    let args = [ArgValue::Bool(false)]; // Don't force valid JSON
    let result = op.run(invalid_avro_data, &args);
    
    // Should still return error for invalid Avro data
    assert!(result.is_err(), "Should return error for invalid Avro data even with force_json=false");
}
