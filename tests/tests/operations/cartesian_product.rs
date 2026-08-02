// Tests for the cartesian_product operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations cartesian_product::

use rxchef::operation::ArgValue;
use rxchef::operations::cartesian_product::CartesianProduct;
use rxchef::Operation;

#[test]
fn test_cartesian_product_basic() {
    let op = CartesianProduct;
    let input = "1,2\n\n3,4".to_string(); // Two sets: [1,2] and [3,4]
    let args = [ArgValue::Str("\\n\\n".to_string()), ArgValue::Str(",".to_string())];
    
    let result = op.run(input.into_bytes(), &args);
    
    assert!(result.is_ok(), "Cartesian product should succeed");
    if let Ok(output) = result {
        let output_str = String::from_utf8_lossy(&output);
        // Expected: (1,3),(1,4),(2,3),(2,4)
        assert!(output_str.contains("(1,3)"), "Should contain (1,3)");
        assert!(output_str.contains("(1,4)"), "Should contain (1,4)");
        assert!(output_str.contains("(2,3)"), "Should contain (2,3)");
        assert!(output_str.contains("(2,4)"), "Should contain (2,4)");
    }
}

#[test]
fn test_cartesian_product_three_sets() {
    let op = CartesianProduct;
    let input = "1,2\n\n3,4\n\n5,6".to_string(); // Three sets: [1,2], [3,4], [5,6]
    let args = [ArgValue::Str("\\n\\n".to_string()), ArgValue::Str(",".to_string())];
    
    let result = op.run(input.into_bytes(), &args);
    
    assert!(result.is_ok(), "Cartesian product with three sets should succeed");
    if let Ok(output) = result {
        let output_str = String::from_utf8_lossy(&output);
        // Should have 2*2*2 = 8 combinations: (1,3,5),(1,3,6),(1,4,5),(1,4,6),(2,3,5),(2,3,6),(2,4,5),(2,4,6)
        assert!(output_str.contains("(1,3,5)"), "Should contain (1,3,5)");
        assert!(output_str.contains("(1,3,6)"), "Should contain (1,3,6)");
        assert!(output_str.contains("(2,4,6)"), "Should contain (2,4,6)");
        // Count the number of tuples (each starts with '(')
        let tuple_count = output_str.matches('(').count();
        assert_eq!(tuple_count, 8, "Should have 8 combinations");
    }
}

#[test]
fn test_cartesian_product_single_set() {
    let op = CartesianProduct;
    let input = "1,2,3".to_string(); // Only one set
    let args = [ArgValue::Str("\\n\\n".to_string()), ArgValue::Str(",".to_string())];
    
    let result = op.run(input.into_bytes(), &args);
    
    // Should return error for insufficient sets
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(e.to_string().contains("Incorrect number of sets"));
    }
}

#[test]
fn test_cartesian_product_custom_delimiters() {
    let op = CartesianProduct;
    let input = "a|b###c|d".to_string(); // Sets separated by ###, items by |
    let args = [ArgValue::Str("###".to_string()), ArgValue::Str("|".to_string())];
    
    let result = op.run(input.into_bytes(), &args);
    
    assert!(result.is_ok(), "Cartesian product with custom delimiters should succeed");
    if let Ok(output) = result {
        let output_str = String::from_utf8_lossy(&output);
        assert!(output_str.contains("(a,c)"), "Should contain (a,c)");
        assert!(output_str.contains("(a,d)"), "Should contain (a,d)");
        assert!(output_str.contains("(b,c)"), "Should contain (b,c)");
        assert!(output_str.contains("(b,d)"), "Should contain (b,d)");
    }
}

#[test]
fn test_cartesian_product_empty_sets() {
    let op = CartesianProduct;
    let input = "\n\n".to_string(); // Empty sets
    let args = [ArgValue::Str("\\n\\n".to_string()), ArgValue::Str(",".to_string())];
    
    let result = op.run(input.into_bytes(), &args);
    
    // Should handle empty sets gracefully
    assert!(result.is_ok(), "Should handle empty sets");
}

#[test]
fn test_cartesian_product_single_item_sets() {
    let op = CartesianProduct;
    let input = "a\n\nb\n\nc".to_string(); // Three sets with single items each
    let args = [ArgValue::Str("\\n\\n".to_string()), ArgValue::Str(",".to_string())];
    
    let result = op.run(input.into_bytes(), &args);
    
    assert!(result.is_ok(), "Cartesian product with single-item sets should succeed");
    if let Ok(output) = result {
        let output_str = String::from_utf8_lossy(&output);
        // Should have one combination: (a,b,c)
        assert!(output_str.contains("(a,b,c)"), "Should contain (a,b,c)");
    }
}
