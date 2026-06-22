/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Public Key from Private Key operation.
 * -----------------------------------------------------------------------------
 */

use k256::SecretKey as K256SecretKey;
use p256::SecretKey as P256SecretKey;
use regex::Regex;
use rsa::{
    pkcs1::DecodeRsaPrivateKey,
    pkcs8::{DecodePrivateKey, EncodePublicKey, LineEnding},
    RsaPrivateKey,
};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};
// Removed redundant and failing spki import

/// Public Key from Private Key operation
pub struct PubKeyFromPrivKeyOp;

impl Operation for PubKeyFromPrivKeyOp {
    fn name(&self) -> &'static str {
        "Public Key from Private Key"
    }

    fn module(&self) -> &'static str {
        "PublicKey"
    }

    fn description(&self) -> &'static str {
        "Extracts the Public Key from a Private Key."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let mut output = String::new();

        // Regex to find PEM blocks
        let re =
            Regex::new(r"(?s)-----BEGIN (?P<type>[A-Z ]+)-----.*?-----END [A-Z ]+-----").unwrap();

        for caps in re.captures_iter(&input_str) {
            let full_pem = caps.get(0).unwrap().as_str();

            // Try RSA PKCS#8
            if let Ok(priv_key) = RsaPrivateKey::from_pkcs8_pem(full_pem) {
                let pub_key = priv_key.to_public_key();
                if let Ok(pub_pem) = pub_key.to_public_key_pem(LineEnding::LF) {
                    output.push_str(&pub_pem);
                    continue;
                }
            }

            // Try RSA PKCS#1
            if let Ok(priv_key) = RsaPrivateKey::from_pkcs1_pem(full_pem) {
                let pub_key = priv_key.to_public_key();
                if let Ok(pub_pem) = pub_key.to_public_key_pem(LineEnding::LF) {
                    output.push_str(&pub_pem);
                    continue;
                }
            }

            // Try P256 EC (PKCS#8)
            if let Ok(secret_key) = P256SecretKey::from_pkcs8_pem(full_pem) {
                let pub_key = secret_key.public_key();
                if let Ok(pub_pem) = pub_key.to_public_key_pem(LineEnding::LF) {
                    output.push_str(&pub_pem);
                    continue;
                }
            }

            // Try K256 EC (PKCS#8)
            if let Ok(secret_key) = K256SecretKey::from_pkcs8_pem(full_pem) {
                let pub_key = secret_key.public_key();
                if let Ok(pub_pem) = pub_key.to_public_key_pem(LineEnding::LF) {
                    output.push_str(&pub_pem);
                    continue;
                }
            }
        }

        Ok(output.into_bytes())
    }
}
