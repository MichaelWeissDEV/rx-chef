/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Generate Lorem Ipsum operation.
 * -----------------------------------------------------------------------------
 */

use rand::Rng;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Generate Lorem Ipsum operation
///
/// Generates varying-length lorem ipsum placeholder text.
pub struct GenerateLoremIpsum;

static WORD_LIST: &[&str] = &[
    "ad",
    "adipisicing",
    "aliqua",
    "aliquip",
    "amet",
    "anim",
    "aute",
    "cillum",
    "commodo",
    "consectetur",
    "consequat",
    "culpa",
    "cupidatat",
    "deserunt",
    "do",
    "dolor",
    "dolore",
    "duis",
    "ea",
    "eiusmod",
    "elit",
    "enim",
    "esse",
    "est",
    "et",
    "eu",
    "ex",
    "excepteur",
    "exercitation",
    "fugiat",
    "id",
    "in",
    "incididunt",
    "ipsum",
    "irure",
    "labore",
    "laboris",
    "laborum",
    "Lorem",
    "magna",
    "minim",
    "mollit",
    "nisi",
    "non",
    "nostrud",
    "nulla",
    "occaecat",
    "officia",
    "pariatur",
    "proident",
    "qui",
    "quis",
    "reprehenderit",
    "sint",
    "sit",
    "sunt",
    "tempor",
    "ullamco",
    "ut",
    "velit",
    "veniam",
    "voluptate",
];

const SENTENCE_LENGTH_MEAN: f64 = 15.0;
const SENTENCE_LENGTH_STD_DEV: f64 = 9.0;
const PARAGRAPH_LENGTH_MEAN: f64 = 5.0;
const PARAGRAPH_LENGTH_STD_DEV: f64 = 2.0;

fn random_length<R: Rng>(rng: &mut R, mean: f64, std_dev: f64) -> usize {
    loop {
        let v = (rng.gen::<f64>() * 2.0 - 1.0)
            + (rng.gen::<f64>() * 2.0 - 1.0)
            + (rng.gen::<f64>() * 2.0 - 1.0) * std_dev
            + mean;
        let n = v.round() as i64;
        if n > 0 {
            return n as usize;
        }
    }
}

fn get_words<R: Rng>(rng: &mut R, count: usize) -> Vec<String> {
    let mut words = Vec::with_capacity(count);
    let mut prev: Option<usize> = None;
    while words.len() < count {
        let idx = loop {
            let i = rng.gen_range(0..WORD_LIST.len());
            if Some(i) != prev {
                break i;
            }
        };
        words.push(WORD_LIST[idx].to_string());
        prev = Some(idx);
    }
    words
}

fn format_sentence<R: Rng>(rng: &mut R, mut words: Vec<String>) -> String {
    if !words.is_empty() && rng.gen::<f64>() < 0.35 {
        let pos = rng.gen_range(0..words.len());
        words[pos].push(',');
    }
    let mut s = words.join(" ");
    // Capitalize first character (ASCII only)
    if let Some(first) = s.chars().next() {
        let upper: String = first.to_uppercase().collect();
        s = upper + &s[first.len_utf8()..];
    }
    s.push('.');
    s
}

fn generate_paragraphs<R: Rng>(rng: &mut R, count: usize) -> String {
    let mut paragraphs: Vec<String> = Vec::new();
    while paragraphs.len() < count {
        let para_len = random_length(rng, PARAGRAPH_LENGTH_MEAN, PARAGRAPH_LENGTH_STD_DEV);
        let mut sentences: Vec<String> = Vec::new();
        while sentences.len() < para_len {
            let sent_len = random_length(rng, SENTENCE_LENGTH_MEAN, SENTENCE_LENGTH_STD_DEV);
            let words = get_words(rng, sent_len);
            sentences.push(format_sentence(rng, words));
        }
        paragraphs.push(sentences.join(" "));
    }
    // Replace start of first paragraph with "Lorem ipsum dolor sit amet"
    replace_start(&mut paragraphs[0]);
    paragraphs.join("\n\n")
}

fn generate_sentences<R: Rng>(rng: &mut R, count: usize) -> String {
    let mut sentences: Vec<String> = Vec::new();
    while sentences.len() < count {
        let sent_len = random_length(rng, SENTENCE_LENGTH_MEAN, SENTENCE_LENGTH_STD_DEV);
        let words = get_words(rng, sent_len);
        sentences.push(format_sentence(rng, words));
    }
    let mut result = sentences.join(" ");
    replace_start(&mut result);
    result
}

fn generate_words<R: Rng>(rng: &mut R, count: usize) -> String {
    let words = get_words(rng, count);
    let mut result = words.join(" ");
    replace_start(&mut result);
    result
}

fn replace_start(s: &mut String) {
    let lorem_start = "Lorem ipsum dolor sit amet";
    let words: Vec<&str> = s.splitn(6, ' ').collect();
    if words.len() > 5 {
        let rest_start = words[..5].iter().map(|w| w.len()).sum::<usize>() + 5;
        *s = format!("{} {}", lorem_start, &s[rest_start..]);
    } else {
        let lorem_words: Vec<&str> = lorem_start.split(' ').collect();
        let take = words.len().min(5);
        *s = lorem_words[..take].join(" ");
        s.push('.');
    }
}

impl Operation for GenerateLoremIpsum {
    fn name(&self) -> &'static str {
        "Generate Lorem Ipsum"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Generate varying length lorem ipsum placeholder text. Length type: Paragraphs, Sentences, Words, Bytes."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Length",
                description: "Number of units to generate",
                default_value: "3",
            },
            ArgSchema {
                name: "Length in",
                description: "Unit type: Paragraphs, Sentences, Words, Bytes",
                default_value: "Paragraphs",
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

    fn run(&self, _input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let length = args.first().and_then(|a| a.as_usize()).unwrap_or(3);
        let length_type = args.get(1).and_then(|a| a.as_str()).unwrap_or("Paragraphs");

        if length < 1 {
            return Err(OperationError::InvalidArgument {
                name: "Length".to_string(),
                reason: "Length must be greater than 0".to_string(),
            });
        }

        let mut rng = rand::thread_rng();
        let result = match length_type {
            "Paragraphs" => generate_paragraphs(&mut rng, length),
            "Sentences" => generate_sentences(&mut rng, length),
            "Words" => generate_words(&mut rng, length),
            "Bytes" => {
                let words_needed = (length / 3).max(1);
                let text = generate_words(&mut rng, words_needed);
                // Keep generating until we have enough bytes
                let mut buf = text;
                while buf.len() < length {
                    buf.push(' ');
                    buf.push_str(&generate_words(&mut rng, 5));
                }
                buf.truncate(length);
                buf
            }
            other => {
                return Err(OperationError::InvalidArgument {
                    name: "Length in".to_string(),
                    reason: format!("Unknown length type: {}", other),
                })
            }
        };

        Ok(result.into_bytes())
    }
}
