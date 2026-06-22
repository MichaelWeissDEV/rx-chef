// Tests for the haversine_distance operation.
// Run only these tests:
//   cargo test -p cyberchef-rust-tests --test operations haversine_distance::

use rxchef::operations::haversine_distance::HaversineDistance;
use rxchef::Operation;

#[test]
fn test_haversine_london_to_dc() {
    let op = HaversineDistance;
    // London: 51.487263,-0.124323  Washington DC: 38.9517,-77.1467
    let input = b"51.487263,-0.124323, 38.9517,-77.1467".to_vec();
    let result = op.run(input, &[]).expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    let dist: f64 = s.parse().expect("numeric");
    // Roughly 5900 km
    assert!(dist > 5_800_000.0 && dist < 6_100_000.0);
}
#[test]
fn test_haversine_same_point() {
    let op = HaversineDistance;
    let input = b"51.0, 0.0, 51.0, 0.0".to_vec();
    let result = op.run(input, &[]).expect("should succeed");
    let s = String::from_utf8(result).expect("valid utf8");
    let dist: f64 = s.parse().expect("numeric");
    assert!(dist.abs() < 0.001);
}
#[test]
fn test_haversine_invalid_input() {
    let op = HaversineDistance;
    let result = op.run(b"not valid".to_vec(), &[]);
    assert!(result.is_err());
}
