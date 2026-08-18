use rxchef::operations::has160::HAS160Op;
use rxchef::Operation;

#[test]
fn test_has160_vectors() {
    let op = HAS160Op;
    assert!(!op.is_broken());
    assert_eq!(
        op.run(Vec::new(), &[]).unwrap(),
        b"307964ef34151d37c8047adec7ab50f4ff89762d"
    );
    assert_eq!(
        op.run(b"abc".to_vec(), &[]).unwrap(),
        b"975e810488cf2a3d49838478124afce4b1c78804"
    );
}

#[test]
fn test_has160_invalid_rounds_zero() {
    let operation = HAS160Op;
    let input = b"test".to_vec();
    use rxchef::operation::ArgValue;
    let result = operation.run(input, &[ArgValue::Num(0.0)]);
    assert!(result.is_err());
}

#[test]
fn test_has160_invalid_rounds_too_large() {
    let operation = HAS160Op;
    let input = b"test".to_vec();
    use rxchef::operation::ArgValue;
    let result = operation.run(input, &[ArgValue::Num(81.0)]);
    assert!(result.is_err());
}
