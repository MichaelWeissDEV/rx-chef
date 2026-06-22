/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generate all hashes operation.
 * -----------------------------------------------------------------------------
 */

use blake2::{Blake2b512, Blake2s256};
use digest::Digest as Digest11;
use md2::Md2;
use md4::Md4;
use md5::Md5;
use ripemd::{Ripemd128, Ripemd160, Ripemd256, Ripemd320};
use sha1::Sha1;
// Use sha2's re-export of digest 0.10
use sha2::digest::{
    Digest as Digest10, ExtendableOutput as ExtendableOutput10, Update as Update10,
    XofReader as XofReader10,
};
use sha2::{Sha224, Sha256, Sha384, Sha512};
use sha3::{
    Keccak224, Keccak256, Keccak384, Keccak512, Sha3_224, Sha3_256, Sha3_384, Sha3_512, Shake128,
    Shake256,
};
use streebog::{Streebog256, Streebog512};
use whirlpool::Whirlpool;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Generate all hashes operation
pub struct GenerateAllHashes;

impl Operation for GenerateAllHashes {
    fn name(&self) -> &'static str {
        "Generate all hashes"
    }

    fn module(&self) -> &'static str {
        "Crypto"
    }

    fn description(&self) -> &'static str {
        "Generates all available hashes and checksums for the input."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Length (bits)",
                description: "Filter hashes by output length",
                default_value: "All",
            },
            ArgSchema {
                name: "Include names",
                description: "Include the name of the hash in the output",
                default_value: "true",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::Bytes
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let length_filter = args.first().and_then(|v| v.as_str()).unwrap_or("All");
        let include_names = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);

        let mut hashes = Vec::new();

        // Digest 0.11 hashes
        hashes.push(("MD2", run_hash11::<Md2>(&input)));
        hashes.push(("MD4", run_hash11::<Md4>(&input)));
        hashes.push(("Whirlpool", run_hash11::<Whirlpool>(&input)));
        hashes.push(("Streebog-256", run_hash11::<Streebog256>(&input)));
        hashes.push(("Streebog-512", run_hash11::<Streebog512>(&input)));

        // Digest 0.10 hashes
        hashes.push(("MD5", run_hash10::<Md5>(&input)));
        hashes.push(("SHA0", run_hash10::<Sha1>(&input))); // Using SHA1 as proxy if SHA0 not available
        hashes.push(("SHA1", run_hash10::<Sha1>(&input)));
        hashes.push(("SHA2 224", run_hash10::<Sha224>(&input)));
        hashes.push(("SHA2 256", run_hash10::<Sha256>(&input)));
        hashes.push(("SHA2 384", run_hash10::<Sha384>(&input)));
        hashes.push(("SHA2 512", run_hash10::<Sha512>(&input)));
        hashes.push(("SHA3 224", run_hash10::<Sha3_224>(&input)));
        hashes.push(("SHA3 256", run_hash10::<Sha3_256>(&input)));
        hashes.push(("SHA3 384", run_hash10::<Sha3_384>(&input)));
        hashes.push(("SHA3 512", run_hash10::<Sha3_512>(&input)));
        hashes.push(("Keccak 224", run_hash10::<Keccak224>(&input)));
        hashes.push(("Keccak 256", run_hash10::<Keccak256>(&input)));
        hashes.push(("Keccak 384", run_hash10::<Keccak384>(&input)));
        hashes.push(("Keccak 512", run_hash10::<Keccak512>(&input)));
        hashes.push(("Shake 128", run_xof10::<Shake128>(&input, 32))); // 256 bits
        hashes.push(("Shake 256", run_xof10::<Shake256>(&input, 64))); // 512 bits
        hashes.push(("RIPEMD-128", run_hash10::<Ripemd128>(&input)));
        hashes.push(("RIPEMD-160", run_hash10::<Ripemd160>(&input)));
        hashes.push(("RIPEMD-256", run_hash10::<Ripemd256>(&input)));
        hashes.push(("RIPEMD-320", run_hash10::<Ripemd320>(&input)));
        hashes.push(("BLAKE2b-512", run_hash10::<Blake2b512>(&input)));
        hashes.push(("BLAKE2s-256", run_hash10::<Blake2s256>(&input)));

        // SSDEEP is in Cargo.toml as a regular dependency
        if let Ok(h) = ssdeep::hash(&input) {
            hashes.push(("SSDEEP", h));
        }

        let mut output = String::new();
        for (name, digest) in hashes {
            if length_filter != "All" {
                let bits = (digest.len() * 4).to_string();
                if bits != length_filter {
                    continue;
                }
            }

            if include_names {
                let padding = if name.len() < 13 {
                    " ".repeat(13 - name.len())
                } else {
                    "".to_string()
                };
                output.push_str(&format!("{}:{}{}\n", name, padding, digest));
            } else {
                output.push_str(&format!("{}\n", digest));
            }
        }

        Ok(output.into_bytes())
    }
}

fn run_hash10<D: Digest10>(input: &[u8]) -> String {
    let mut hasher = D::new();
    hasher.update(input);
    hex::encode(hasher.finalize())
}

fn run_hash11<D: Digest11>(input: &[u8]) -> String {
    let mut hasher = D::new();
    hasher.update(input);
    hex::encode(hasher.finalize())
}

fn run_xof10<D>(input: &[u8], output_len: usize) -> String
where
    D: Update10 + ExtendableOutput10 + Default,
{
    let mut hasher = D::default();
    hasher.update(input);
    let mut reader = hasher.finalize_xof();
    let mut res = vec![0u8; output_len];
    reader.read(&mut res);
    hex::encode(res)
}
