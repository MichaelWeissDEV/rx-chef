// Tests for the avro_to_json operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations avro_to_json::

use rxchef::operation::ArgValue;
use rxchef::operations::avro_to_json::AvroToJSON;
use rxchef::Operation;

#[test]
fn test_avro_to_json_upstream_small_container() {
    // Fixed Avro object-container fixture and expected record from CyberChef's
    // upstream AvroToJSON.mjs suite at commit 2e048b029085.
    let input = hex::decode("4f626a0104166176726f2e736368656d6196017b2274797065223a227265636f7264222c226e616d65223a22736d616c6c222c226669656c6473223a5b7b226e616d65223a226e616d65222c2274797065223a22737472696e67227d5d7d146176726f2e636f646563086e756c6c004e0247632e3702e5b75cdab9a62f1541020e0c6d796e616d654e0247632e3702e5b75cdab9a62f1541").unwrap();
    let output = AvroToJSON.run(input, &[ArgValue::Bool(true)]).unwrap();
    assert_eq!(output, b"{\n    \"name\": \"myname\"\n}");
}

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
    assert!(
        result.is_err(),
        "Should return error for invalid Avro data even with force_json=false"
    );
}
