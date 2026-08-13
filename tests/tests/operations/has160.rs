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
