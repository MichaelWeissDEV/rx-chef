//! OpenSSL-backed RSA helpers shared by the RSA and JWK operations.
//!
//! Keeping RSA implementation details in one place lets the public operations
//! support both PKCS#1 and PKCS#8 PEM without depending on an unmaintained
//! pure-Rust RSA implementation.

use openssl::{
    error::ErrorStack,
    hash::MessageDigest,
    pkey::{PKey, Private, Public},
    rsa::Rsa,
};

pub fn public_key_from_pem(pem: &str) -> Result<PKey<Public>, ErrorStack> {
    PKey::public_key_from_pem(pem.as_bytes())
        .or_else(|_| Rsa::public_key_from_pem(pem.as_bytes()).and_then(PKey::from_rsa))
}

pub fn private_key_from_pem(pem: &str) -> Result<PKey<Private>, ErrorStack> {
    PKey::private_key_from_pem(pem.as_bytes())
}

pub fn digest(name: &str) -> Option<MessageDigest> {
    match name {
        "SHA-1" => Some(MessageDigest::sha1()),
        "SHA-224" => Some(MessageDigest::sha224()),
        "SHA-256" => Some(MessageDigest::sha256()),
        "SHA-384" => Some(MessageDigest::sha384()),
        "SHA-512" => Some(MessageDigest::sha512()),
        "MD5" => Some(MessageDigest::md5()),
        _ => None,
    }
}
