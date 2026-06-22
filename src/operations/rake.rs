/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the RAKE operation.
 * -----------------------------------------------------------------------------
 */

use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// RAKE (Rapid Keyword Extraction) operation
pub struct RAKE;

impl Operation for RAKE {
    fn name(&self) -> &'static str {
        "RAKE"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Rapid Keyword Extraction (RAKE)<br><br>RAKE is a domain-independent keyword extraction algorithm in Natural Language Processing.<br><br>The list of stop words are from the NLTK python package"
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Word Delimiter (Regex)",
                description: "Word Delimiter (Regex)",
                default_value: r"\s",
            },
            ArgSchema {
                name: "Sentence Delimiter (Regex)",
                description: "Sentence Delimiter (Regex)",
                default_value: r"\.\s|\n",
            },
            ArgSchema {
                name: "Stop Words",
                description: "Stop Words (comma separated)",
                default_value: "i,me,my,myself,we,our,ours,ourselves,you,you're,you've,you'll,you'd,your,yours,yourself,yourselves,he,him,his,himself,she,she's,her,hers,herself,it,it's,its,itsef,they,them,their,theirs,themselves,what,which,who,whom,this,that,that'll,these,those,am,is,are,was,were,be,been,being,have,has,had,having,do,does',did,doing,a,an,the,and,but,if,or,because,as,until,while,of,at,by,for,with,about,against,between,into,through,during,before,after,above,below,to,from,up,down,in,out,on,off,over,under,again,further,then,once,here,there,when,where,why,how,all,any,both,each,few,more,most,other,some,such,no,nor,not,only,own,same,so,than,too,very,s,t,can,will,just,don,don't,should,should've,now,d,ll,m,o,re,ve,y,ain,aren,aren't,couldn,couldn't,didn,didn't,doesn,doesn't,hadn,hadn't,hasn,hasn't,haven,haven't,isn,isn't,ma,mightn,mightn't,mustn,mustn't,needn,needn't,shan,shan't,shouldn,shouldn't,wasn,wasn't,weren,weren't,won,won't,wouldn,wouldn't",
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
        let input_str = String::from_utf8_lossy(&input).to_lowercase();
        let input_str = input_str.trim();

        if input_str.is_empty() {
            return Ok(Vec::new());
        }

        let word_delim_regex = args.first().and_then(|a| a.as_str()).unwrap_or(r"\s");
        let sent_delim_regex = args.get(1).and_then(|a| a.as_str()).unwrap_or(r"\.\s|\n");
        let stop_words_str = args.get(2).and_then(|a| a.as_str()).unwrap_or("");

        let word_regex = Regex::new(word_delim_regex)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid word regex: {}", e)))?;
        let sent_regex = Regex::new(sent_delim_regex)
            .map_err(|e| OperationError::InvalidInput(format!("Invalid sentence regex: {}", e)))?;

        let mut stop_words: HashSet<String> = stop_words_str
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .collect();
        stop_words.insert("".to_string());

        let sentences: Vec<&str> = sent_regex.split(input_str).collect();
        let mut tokens_list: Vec<String> = Vec::new();
        let mut word_frequencies: HashMap<String, usize> = HashMap::new();
        let mut phrases: Vec<Vec<String>> = Vec::new();

        for sent in sentences {
            let split_sent: Vec<&str> = word_regex.split(sent).collect();
            let mut start_index = 0;

            for i in 0..split_sent.len() {
                let token = split_sent[i];
                if stop_words.contains(token) {
                    let phrase: Vec<String> = split_sent[start_index..i]
                        .iter()
                        .filter(|&&s| !s.is_empty())
                        .map(|&s| s.to_string())
                        .collect();
                    if !phrase.is_empty() {
                        phrases.push(phrase);
                    }
                    start_index = i + 1;
                } else {
                    let token_string = token.to_string();
                    if !token_string.is_empty() {
                        *word_frequencies.entry(token_string.clone()).or_insert(0) += 1;
                        if !tokens_list.contains(&token_string) {
                            tokens_list.push(token_string);
                        }
                    }
                }
            }
            let phrase: Vec<String> = split_sent[start_index..]
                .iter()
                .filter(|&&s| !s.is_empty())
                .map(|&s| s.to_string())
                .collect();
            if !phrase.is_empty() {
                phrases.push(phrase);
            }
        }

        // Deduplicate phrases
        let mut unique_phrases = Vec::new();
        let mut seen_phrases = HashSet::new();
        for phrase in phrases {
            if seen_phrases.insert(phrase.clone()) {
                unique_phrases.push(phrase);
            }
        }
        let phrases = unique_phrases;

        if tokens_list.is_empty() {
            return Ok(Vec::new());
        }

        let num_tokens = tokens_list.len();
        let mut word_degree_matrix = vec![vec![0usize; num_tokens]; num_tokens];
        let token_to_index: HashMap<String, usize> = tokens_list
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i))
            .collect();

        for phrase in &phrases {
            for word1 in phrase {
                for word2 in phrase {
                    if let (Some(&idx1), Some(&idx2)) =
                        (token_to_index.get(word1), token_to_index.get(word2))
                    {
                        word_degree_matrix[idx1][idx2] += 1;
                    }
                }
            }
        }

        let mut degree_scores = vec![0.0f64; num_tokens];
        for i in 0..num_tokens {
            let mut word_degree = 0usize;
            for j in 0..num_tokens {
                word_degree += word_degree_matrix[j][i];
            }
            let freq = *word_frequencies.get(&tokens_list[i]).unwrap_or(&1) as f64;
            degree_scores[i] = word_degree as f64 / freq;
        }

        let mut phrase_scores: Vec<(f64, String)> = phrases
            .iter()
            .map(|phrase| {
                let mut score = 0.0;
                for token in phrase {
                    if let Some(&idx) = token_to_index.get(token) {
                        score += degree_scores[idx];
                    }
                }
                (score, phrase.join(" "))
            })
            .collect();

        phrase_scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut output = String::from("Scores: , Keywords: \n");
        for (score, phrase) in phrase_scores {
            output.push_str(&format!("{}, {}\n", score, phrase));
        }

        Ok(output.trim_end().as_bytes().to_vec())
    }
}
