use rxchef::operation::ArgValue;
use rxchef::operations::snefru::SNEFRU;
use rxchef::Operation;

#[test]
fn test_snefru_256_vectors() {
    let operation = SNEFRU;
    assert!(!operation.is_broken());
    assert_eq!(
        operation.run(Vec::new(), &[]).unwrap(),
        b"8617f366566a011837f4fb4ba5bedea2b892f3ed8b894023d16ae344b2be5881"
    );
    assert_eq!(
        operation.run(b"hello".to_vec(), &[]).unwrap(),
        b"7c5f22b1a92d9470efea37ec6ed00b2357a4ce3c41aa6e28e3b84057465dbb56"
    );
}

#[test]
fn test_snefru_rejects_nonstandard_variants() {
    let operation = SNEFRU;
    assert!(operation
        .run(
            b"test".to_vec(),
            &[ArgValue::Num(128.0), ArgValue::Num(8.0)]
        )
        .is_err());
    assert!(operation
        .run(
            b"test".to_vec(),
            &[ArgValue::Num(256.0), ArgValue::Num(4.0)]
        )
        .is_err());
}
