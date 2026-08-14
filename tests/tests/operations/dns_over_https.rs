// Tests for the DNS over HTTPS operation that do not require a network socket.
// Transport-independent URL/response behavior is unit-tested in the operation
// module; these tests exercise the public Operation contract.

use rxchef::operation::{ArgValue, OperationError};
use rxchef::operations::dns_over_https::DnsOverHttps;
use rxchef::Operation;

#[test]
fn empty_domain_is_an_empty_dns_document_without_network_access() {
    let operation = DnsOverHttps;
    let output = operation.run(Vec::new(), &[]).unwrap();
    assert_eq!(output, b"{}");
}

#[test]
fn invalid_resolver_is_rejected_before_network_access() {
    let operation = DnsOverHttps;
    let error = operation
        .run(
            b"example.com".to_vec(),
            &[
                ArgValue::Str("not a resolver URL".into()),
                ArgValue::Str("A".into()),
                ArgValue::Bool(false),
                ArgValue::Bool(false),
            ],
        )
        .unwrap_err();
    assert!(matches!(
        error,
        OperationError::InvalidArgument { ref name, .. } if name == "Resolver"
    ));
}
