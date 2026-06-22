/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the RC6 Decrypt operation.
 * -----------------------------------------------------------------------------
 */

use num_bigint::{BigInt, Sign};
use num_traits::{One, ToPrimitive, Zero};

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError, Utils};

/// RC6 Decrypt operation
pub struct RC6Decrypt;

impl Operation for RC6Decrypt {
    fn name(&self) -> &'static str {
        "RC6 Decrypt"
    }

    fn module(&self) -> &'static str {
        "Ciphers"
    }

    fn description(&self) -> &'static str {
        "RC6 is a symmetric key block cipher derived from RC5. It was designed by Ron Rivest, Matt Robshaw, Ray Sidney, and Yiqun Lisa Yin to meet the requirements of the AES competition, and was one of the five finalists.<br><br>RC6 is parameterised as RC6-w/r/b where w is word size in bits (any multiple of 8 from 8-256), r is the number of rounds (1-255), and b is the key length in bytes. The standard AES submission uses w=32, r=20. Common word sizes: 8, 16, 32 (standard), 64, 128.<br><br><b>IV:</b> The Initialisation Vector should be 4*w/8 bytes (e.g. 16 bytes for w=32). If not entered, it will default to null bytes.<br><br><b>Padding:</b> In CBC and ECB mode, the PKCS#7 padding scheme is used."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Key",
                description: "Key",
                default_value: "",
            },
            ArgSchema {
                name: "IV",
                description: "IV",
                default_value: "",
            },
            ArgSchema {
                name: "Mode",
                description: "Mode",
                default_value: "CBC",
            },
            ArgSchema {
                name: "Input",
                description: "Input format",
                default_value: "Hex",
            },
            ArgSchema {
                name: "Output",
                description: "Output format",
                default_value: "Raw",
            },
            ArgSchema {
                name: "Padding",
                description: "Padding scheme",
                default_value: "PKCS5",
            },
            ArgSchema {
                name: "Word Size",
                description: "Word size in bits (8-256)",
                default_value: "32",
            },
            ArgSchema {
                name: "Rounds",
                description: "Number of rounds (1-255)",
                default_value: "20",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let key_arg = args
            .get(0)
            .ok_or(OperationError::InvalidInput("Missing Key".to_string()))?;
        let iv_arg = args
            .get(1)
            .ok_or(OperationError::InvalidInput("Missing IV".to_string()))?;
        let mode = args.get(2).and_then(|a| a.as_str()).unwrap_or("CBC");
        let input_type = args.get(3).and_then(|a| a.as_str()).unwrap_or("Hex");
        let output_type = args.get(4).and_then(|a| a.as_str()).unwrap_or("Raw");
        let padding = args.get(5).and_then(|a| a.as_str()).unwrap_or("PKCS5");
        let word_size = args.get(6).and_then(|a| a.as_i64()).unwrap_or(32) as usize;
        let rounds = args.get(7).and_then(|a| a.as_i64()).unwrap_or(20) as usize;

        if word_size < 8 || word_size > 256 || word_size % 8 != 0 {
            return Err(OperationError::InvalidInput(format!(
                "Invalid word size: {}. Must be a multiple of 8 between 8 and 256.",
                word_size
            )));
        }

        if rounds < 1 || rounds > 255 {
            return Err(OperationError::InvalidInput(format!(
                "Invalid number of rounds: {}. Must be between 1 and 255.",
                rounds
            )));
        }

        let key = Utils::convert_to_byte_array(key_arg)?;
        let iv = Utils::convert_to_byte_array(iv_arg)?;
        let input_bytes = if input_type == "Hex" {
            hex::decode(
                String::from_utf8_lossy(&input)
                    .replace(" ", "")
                    .replace("\n", ""),
            )
            .map_err(|e| OperationError::InvalidInput(format!("Invalid hex input: {}", e)))?
        } else {
            input
        };

        let block_size = 4 * (word_size / 8);
        let actual_iv = if iv.is_empty() {
            vec![0u8; block_size]
        } else if iv.len() != block_size && mode != "ECB" {
            return Err(OperationError::InvalidInput(format!(
                "Invalid IV length: {} bytes. RC6-{} uses an IV length of {} bytes.",
                iv.len(),
                word_size,
                block_size
            )));
        } else {
            iv
        };

        if (mode == "ECB" || mode == "CBC") && input_bytes.len() % block_size != 0 {
            return Err(OperationError::InvalidInput(format!(
                "Invalid ciphertext length: {} bytes. Must be a multiple of {}.",
                input_bytes.len(),
                block_size
            )));
        }

        let rc6 = RC6::new(&key, rounds, word_size);
        let mut output = Vec::new();
        match mode {
            "ECB" => {
                for chunk in input_bytes.chunks(block_size) {
                    output.extend(rc6.decrypt_block(chunk));
                }
                output = remove_padding(&output, padding, block_size)?;
            }
            "CBC" => {
                let mut prev_block = actual_iv;
                for chunk in input_bytes.chunks(block_size) {
                    let decrypted = rc6.decrypt_block(chunk);
                    let mut block = vec![0u8; block_size];
                    for i in 0..block_size {
                        block[i] = decrypted[i] ^ prev_block[i];
                    }
                    output.extend(&block);
                    prev_block = chunk.to_vec();
                }
                output = remove_padding(&output, padding, block_size)?;
            }
            "CFB" => {
                let mut prev_block = actual_iv;
                for chunk in input_bytes.chunks(block_size) {
                    let encrypted = rc6.encrypt_block(&prev_block);
                    for i in 0..chunk.len() {
                        output.push(chunk[i] ^ encrypted[i]);
                    }
                    prev_block = chunk.to_vec();
                    if prev_block.len() < block_size {
                        prev_block.extend(vec![0u8; block_size - prev_block.len()]);
                    }
                }
            }
            "OFB" => {
                let mut iv_block = actual_iv;
                for chunk in input_bytes.chunks(block_size) {
                    iv_block = rc6.encrypt_block(&iv_block);
                    for i in 0..chunk.len() {
                        output.push(chunk[i] ^ iv_block[i]);
                    }
                }
            }
            "CTR" => {
                let mut counter = actual_iv;
                for chunk in input_bytes.chunks(block_size) {
                    let encrypted = rc6.encrypt_block(&counter);
                    for i in 0..chunk.len() {
                        output.push(chunk[i] ^ encrypted[i]);
                    }
                    increment_counter(&mut counter);
                }
            }
            _ => {
                return Err(OperationError::InvalidInput(format!(
                    "Unsupported mode: {}",
                    mode
                )))
            }
        }

        if output_type == "Hex" {
            Ok(hex::encode(output).into_bytes())
        } else {
            Ok(output)
        }
    }
}

fn remove_padding(
    input: &[u8],
    padding: &str,
    block_size: usize,
) -> Result<Vec<u8>, OperationError> {
    if input.is_empty() {
        return Ok(Vec::new());
    }
    match padding {
        "PKCS5" | "PKCS7" => {
            let pad_byte = input[input.len() - 1];
            if pad_byte == 0 || pad_byte as usize > block_size {
                return Err(OperationError::InvalidInput(
                    "Invalid PKCS5 padding".to_string(),
                ));
            }
            for i in 0..pad_byte as usize {
                if input[input.len() - 1 - i] != pad_byte {
                    return Err(OperationError::InvalidInput(
                        "Invalid PKCS5 padding".to_string(),
                    ));
                }
            }
            Ok(input[..input.len() - pad_byte as usize].to_vec())
        }
        "BIT" => {
            for i in (0..input.len()).rev() {
                if input[i] == 0x80 {
                    return Ok(input[..i].to_vec());
                } else if input[i] != 0 {
                    return Err(OperationError::InvalidInput(
                        "Invalid BIT padding".to_string(),
                    ));
                }
            }
            Err(OperationError::InvalidInput(
                "Invalid BIT padding".to_string(),
            ))
        }
        "ZERO" | "NO" | "RANDOM" => Ok(input.to_vec()),
        _ => Err(OperationError::InvalidInput(format!(
            "Unsupported padding: {}",
            padding
        ))),
    }
}

fn increment_counter(counter: &mut [u8]) {
    for i in 0..counter.len() {
        counter[i] = counter[i].wrapping_add(1);
        if counter[i] != 0 {
            break;
        }
    }
}

struct RC6 {
    s: Vec<BigInt>,
    rounds: usize,
    w: usize,
    mask: BigInt,
    lgw: u32,
    lg_mask: BigInt,
}

impl RC6 {
    fn new(key: &[u8], rounds: usize, w: usize) -> Self {
        let p_256 = BigInt::from_bytes_be(
            Sign::Plus,
            &hex::decode("b7e151628aed2a6abf7158809cf4f3c762e7160f38b4da56a784d9045190cfef")
                .unwrap(),
        );
        let q_256 = BigInt::from_bytes_be(
            Sign::Plus,
            &hex::decode("9e3779b97f4a7c15f39cc0605cedc8341082276bf3a27251f86c6a11d0c18e95")
                .unwrap(),
        );

        let p = (p_256 >> (256 - w)) | BigInt::one();
        let q = (q_256 >> (256 - w)) | BigInt::one();

        let mask = (BigInt::one() << w) - BigInt::one();
        let lgw = (w as f64).log2().floor() as u32;
        let lg_mask = (BigInt::one() << lgw) - BigInt::one();

        let bytes_per_word = w / 8;
        let b = key.len();
        let c = std::cmp::max((b + bytes_per_word - 1) / bytes_per_word, 1);

        let mut padded_key = key.to_vec();
        while padded_key.len() < c * bytes_per_word {
            padded_key.push(0);
        }

        let mut l = Vec::new();
        for i in (0..padded_key.len()).step_by(bytes_per_word) {
            let mut word = BigInt::zero();
            for j in 0..bytes_per_word {
                word |= BigInt::from(padded_key[i + j]) << (j * 8);
            }
            l.push(word);
        }

        let t = 2 * rounds + 4;
        let mut s = vec![BigInt::zero(); t];
        s[0] = p;
        for i in 1..t {
            s[i] = (&s[i - 1] + &q) & &mask;
        }

        let mut a = BigInt::zero();
        let mut b_val = BigInt::zero();
        let mut i = 0;
        let mut j = 0;
        let v = 3 * std::cmp::max(c, t);

        for _ in 0..v {
            s[i] = rol(
                &((&s[i] + &a + &b_val) & &mask),
                &BigInt::from(3u8),
                w,
                &lg_mask,
                &mask,
            );
            a = s[i].clone();
            l[j] = rol(
                &((&l[j] + &a + &b_val) & &mask),
                &(&a + &b_val),
                w,
                &lg_mask,
                &mask,
            );
            b_val = l[j].clone();
            i = (i + 1) % t;
            j = (j + 1) % c;
        }

        RC6 {
            s,
            rounds,
            w,
            mask,
            lgw,
            lg_mask,
        }
    }

    fn encrypt_block(&self, block: &[u8]) -> Vec<u8> {
        let bytes_per_word = self.w / 8;
        let mut words = Vec::new();
        for i in (0..block.len()).step_by(bytes_per_word) {
            let mut word = BigInt::zero();
            for j in 0..bytes_per_word {
                word |= BigInt::from(block[i + j]) << (j * 8);
            }
            words.push(word);
        }

        let mut a = words[0].clone();
        let mut b = words[1].clone();
        let mut c = words[2].clone();
        let mut d = words[3].clone();

        b = (&b + &self.s[0]) & &self.mask;
        d = (&d + &self.s[1]) & &self.mask;

        for i in 1..=self.rounds {
            let t = rol(
                &((&b * (&(&b << 1) + BigInt::one())) & &self.mask),
                &BigInt::from(self.lgw),
                self.w,
                &self.lg_mask,
                &self.mask,
            );
            let u = rol(
                &((&d * (&(&d << 1) + BigInt::one())) & &self.mask),
                &BigInt::from(self.lgw),
                self.w,
                &self.lg_mask,
                &self.mask,
            );
            a = (rol(&(a ^ &t), &u, self.w, &self.lg_mask, &self.mask) + &self.s[2 * i])
                & &self.mask;
            c = (rol(&(c ^ &u), &t, self.w, &self.lg_mask, &self.mask) + &self.s[2 * i + 1])
                & &self.mask;

            let temp = a;
            a = b;
            b = c;
            c = d;
            d = temp;
        }

        a = (&a + &self.s[2 * self.rounds + 2]) & &self.mask;
        c = (&c + &self.s[2 * self.rounds + 3]) & &self.mask;

        let mut output = Vec::new();
        for word in &[a, b, c, d] {
            for j in 0..bytes_per_word {
                output.push(((word >> (j * 8)) & BigInt::from(0xFFu8)).to_u8().unwrap());
            }
        }
        output
    }

    fn decrypt_block(&self, block: &[u8]) -> Vec<u8> {
        let bytes_per_word = self.w / 8;
        let mut words = Vec::new();
        for i in (0..block.len()).step_by(bytes_per_word) {
            let mut word = BigInt::zero();
            for j in 0..bytes_per_word {
                word |= BigInt::from(block[i + j]) << (j * 8);
            }
            words.push(word);
        }

        let mut a = words[0].clone();
        let mut b = words[1].clone();
        let mut c = words[2].clone();
        let mut d = words[3].clone();

        c = (&c - &self.s[2 * self.rounds + 3] + (&BigInt::one() << self.w)) & &self.mask;
        a = (&a - &self.s[2 * self.rounds + 2] + (&BigInt::one() << self.w)) & &self.mask;

        for i in (1..=self.rounds).rev() {
            let temp = d;
            d = c;
            c = b;
            b = a;
            a = temp;

            let u = rol(
                &(&d * (&(&d << 1) + BigInt::one()) & &self.mask),
                &BigInt::from(self.lgw),
                self.w,
                &self.lg_mask,
                &self.mask,
            );
            let t = rol(
                &(&b * (&(&b << 1) + BigInt::one()) & &self.mask),
                &BigInt::from(self.lgw),
                self.w,
                &self.lg_mask,
                &self.mask,
            );

            c = (ror(
                &((&c - &self.s[2 * i + 1] + (&BigInt::one() << self.w)) & &self.mask),
                &t,
                self.w,
                &self.lg_mask,
                &self.mask,
            )) ^ &u;
            a = (ror(
                &((&a - &self.s[2 * i] + (&BigInt::one() << self.w)) & &self.mask),
                &u,
                self.w,
                &self.lg_mask,
                &self.mask,
            )) ^ &t;
        }

        d = (&d - &self.s[1] + (&BigInt::one() << self.w)) & &self.mask;
        b = (&b - &self.s[0] + (&BigInt::one() << self.w)) & &self.mask;

        let mut output = Vec::new();
        for word in &[a, b, c, d] {
            for j in 0..bytes_per_word {
                output.push(((word >> (j * 8)) & BigInt::from(0xFFu8)).to_u8().unwrap());
            }
        }
        output
    }
}

fn rol(x: &BigInt, n: &BigInt, w: usize, lg_mask: &BigInt, mask: &BigInt) -> BigInt {
    let shift = (n & lg_mask).to_usize().unwrap() % w;
    ((x << shift) | (x >> (w - shift))) & mask
}

fn ror(x: &BigInt, n: &BigInt, w: usize, lg_mask: &BigInt, mask: &BigInt) -> BigInt {
    let shift = (n & lg_mask).to_usize().unwrap() % w;
    ((x >> shift) | (x << (w - shift))) & mask
}
