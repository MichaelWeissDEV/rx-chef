/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the CipherSaber2 Encrypt operation.
 * -----------------------------------------------------------------------------
 */

use rand::RngCore;

use crate::{
    operation::{ArgSchema, ArgValue, DataType, Operation, OperationError},
    operations::cipher_saber2_decrypt::ciphersaber2_crypt,
};

/// CipherSaber2 Encrypt operation
pub struct CipherSaber2Encrypt;

impl Operation for CipherSaber2Encrypt {
    fn name(&self) -> &'static str {
        "CipherSaber2 Encrypt"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "CipherSaber is a simple symmetric encryption protocol based on the RC4 stream \
        cipher. It gives reasonably strong protection of message confidentiality, yet \
        it's designed to be simple enough that even novice programmers can memorize the \
        algorithm and implement it from scratch. A random 10-byte IV is prepended to \
        the ciphertext output."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Encryption key",
                default_value: "",
                kind: crate::operation::ArgKind::Bytes,
                required: true,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: true,
            },
            ArgSchema {
                name: "Rounds",
                description: "Number of key schedule rounds (default 20)",
                default_value: "20",
                kind: crate::operation::ArgKind::UnsignedInteger,
                required: false,
                choices: &[],
                minimum: None,
                maximum: None,
                sensitive: false,
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::Bytes
    }

    /// Prepends a random initialisation vector.
    fn side_effects(&self) -> &'static [crate::operation::SideEffect] {
        use crate::operation::SideEffect;
        &[SideEffect::Random]
    }

    /// Equal inputs do not produce equal outputs.
    fn deterministic(&self) -> bool {
        false
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key = self.parse_key(args.first())?;
        let rounds = args.get(1).and_then(|a| a.as_usize()).unwrap_or(20);

        // Generate a random 10-byte IV
        let mut iv = [0u8; 10];
        rand::thread_rng().fill_bytes(&mut iv);

        Ok(encrypt_with_iv(&input, &key, rounds, iv))
    }
}

fn encrypt_with_iv(input: &[u8], key: &[u8], rounds: usize, iv: [u8; 10]) -> Vec<u8> {
    let ciphertext = ciphersaber2_crypt(&iv, key, rounds, input);
    let mut output = Vec::with_capacity(10 + ciphertext.len());
    output.extend_from_slice(&iv);
    output.extend_from_slice(&ciphertext);
    output
}

#[cfg(test)]
mod tests {
    use super::encrypt_with_iv;

    #[test]
    fn published_ciphersaber_vector_encrypts_exactly() {
        let expected = hex::decode("6f6d0babf3aa6719031530edb677ca74e0089dd0e7b8854356bb1448e37cdbefe7f3a84f4f5fb3fd").unwrap();
        let iv: [u8; 10] = expected[..10].try_into().unwrap();
        assert_eq!(
            encrypt_with_iv(b"This is a test of CipherSaber.", b"asdfg", 1, iv),
            expected
        );
    }
}

impl CipherSaber2Encrypt {
    fn parse_key(&self, arg: Option<&ArgValue>) -> Result<Vec<u8>, OperationError> {
        match arg {
            Some(ArgValue::Str(s)) => {
                if s.is_empty() {
                    Ok(vec![])
                } else if s.starts_with("0x") {
                    hex::decode(&s[2..]).map_err(|e| OperationError::InvalidArgument {
                        name: "Key".to_string(),
                        reason: e.to_string(),
                    })
                } else {
                    Ok(s.as_bytes().to_vec())
                }
            }
            Some(ArgValue::Bytes(b)) => Ok(b.clone()),
            _ => Ok(vec![]),
        }
    }
}
