/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Fuzzy Match operation.
 * -----------------------------------------------------------------------------
 */

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

#[derive(Clone)]
struct Weights {
    sequential_bonus: i32,
    separator_bonus: i32,
    camel_bonus: i32,
    first_letter_bonus: i32,
    leading_letter_penalty: i32,
    max_leading_letter_penalty: i32,
    unmatched_letter_penalty: i32,
}

/// Fuzzy Match operation
pub struct FuzzyMatch;

impl Operation for FuzzyMatch {
    fn name(&self) -> &'static str {
        "Fuzzy Match"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Conducts a fuzzy search to find a pattern within the input based on weighted criteria."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        static SCHEMA: &[ArgSchema] = &[
            ArgSchema {
                name: "Search",
                description: "Pattern to search for",
                default_value: "",
            },
            ArgSchema {
                name: "Sequential bonus",
                description: "Bonus for adjacent matches",
                default_value: "15",
            },
            ArgSchema {
                name: "Separator bonus",
                description: "Bonus if match occurs after a separator",
                default_value: "30",
            },
            ArgSchema {
                name: "Camel bonus",
                description: "Bonus if match is uppercase and previous is lower",
                default_value: "30",
            },
            ArgSchema {
                name: "First letter bonus",
                description: "Bonus if the first letter is matched",
                default_value: "15",
            },
            ArgSchema {
                name: "Leading letter penalty",
                description: "Penalty applied for every letter in the input before the first match",
                default_value: "-5",
            },
            ArgSchema {
                name: "Max leading letter penalty",
                description: "Maxiumum penalty for leading letters",
                default_value: "-15",
            },
            ArgSchema {
                name: "Unmatched letter penalty",
                description: "Unmatched letter penalty",
                default_value: "-1",
            },
        ];
        SCHEMA
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::Html
    }

    fn run(&self, input: Vec<u8>, args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8(input)
            .map_err(|_| OperationError::InvalidInput("Invalid UTF-8".to_string()))?;

        if input_str.is_empty() {
            return Ok(b"No matches.".to_vec());
        }

        let search_str = args.first().and_then(|v| v.as_str()).unwrap_or("");

        if search_str.is_empty() {
            return Ok(b"No matches.".to_vec());
        }

        let weights = Weights {
            sequential_bonus: args.get(1).and_then(|v| v.as_f64()).unwrap_or(15.0) as i32,
            separator_bonus: args.get(2).and_then(|v| v.as_f64()).unwrap_or(30.0) as i32,
            camel_bonus: args.get(3).and_then(|v| v.as_f64()).unwrap_or(30.0) as i32,
            first_letter_bonus: args.get(4).and_then(|v| v.as_f64()).unwrap_or(15.0) as i32,
            leading_letter_penalty: args.get(5).and_then(|v| v.as_f64()).unwrap_or(-5.0) as i32,
            max_leading_letter_penalty: args.get(6).and_then(|v| v.as_f64()).unwrap_or(-15.0)
                as i32,
            unmatched_letter_penalty: args.get(7).and_then(|v| v.as_f64()).unwrap_or(-1.0) as i32,
        };

        let matches = fuzzy_match_global(search_str, &input_str, &weights);

        if matches.is_empty() {
            return Ok(b"No matches.".to_vec());
        }

        let mut result = String::new();
        let mut pos = 0;
        let mut hl_class = "hl1";

        let input_chars: Vec<char> = input_str.chars().collect();

        for (_found, _score, idxs) in matches {
            let match_ranges = calc_match_ranges(&idxs);

            for (i, &(start, length)) in match_ranges.iter().enumerate() {
                result.push_str(&escape_html(
                    &input_chars[pos..start].iter().collect::<String>(),
                ));
                if i == 0 {
                    result.push_str(&format!("<span class=\"{}\">", hl_class));
                }
                pos = start + length;
                result.push_str("<b>");
                result.push_str(&escape_html(
                    &input_chars[start..pos].iter().collect::<String>(),
                ));
                result.push_str("</b>");
            }
            result.push_str("</span>");
            hl_class = if hl_class == "hl1" { "hl2" } else { "hl1" };
        }

        result.push_str(&escape_html(&input_chars[pos..].iter().collect::<String>()));

        Ok(result.into_bytes())
    }
}

fn fuzzy_match_global(pattern: &str, str: &str, weights: &Weights) -> Vec<(bool, i32, Vec<usize>)> {
    let mut results = Vec::new();
    let mut str_curr_index = 0;

    let pattern_chars: Vec<char> = pattern.chars().collect();
    let str_chars: Vec<char> = str.chars().collect();

    loop {
        let mut matches = vec![0; 256];
        let res = fuzzy_match_recursive(
            &pattern_chars,
            &str_chars,
            0,
            str_curr_index,
            None,
            &mut matches,
            256,
            0,
            &mut 0,
            10,
            weights,
        );

        if res.0 {
            let idxs = res.2[..pattern_chars.len()].to_vec();
            str_curr_index = idxs.last().unwrap() + 1;
            results.push((res.0, res.1, idxs));
        } else {
            break;
        }
    }
    results
}

fn fuzzy_match_recursive(
    pattern: &[char],
    str: &[char],
    mut pattern_cur_index: usize,
    mut str_curr_index: usize,
    src_matches: Option<&[usize]>,
    matches: &mut [usize],
    max_matches: usize,
    mut next_match: usize,
    recursion_count: &mut i32,
    recursion_limit: i32,
    weights: &Weights,
) -> (bool, i32, Vec<usize>) {
    let mut out_score = 0;

    *recursion_count += 1;
    if *recursion_count >= recursion_limit {
        return (false, out_score, matches.to_vec());
    }

    if pattern_cur_index == pattern.len() || str_curr_index == str.len() {
        return (false, out_score, matches.to_vec());
    }

    let mut recursive_match = false;
    let mut best_recursive_matches = Vec::new();
    let mut best_recursive_score = 0;
    let mut first_match = true;

    while pattern_cur_index < pattern.len() && str_curr_index < str.len() {
        if pattern[pattern_cur_index].to_lowercase().to_string()
            == str[str_curr_index].to_lowercase().to_string()
        {
            if next_match >= max_matches {
                return (false, out_score, matches.to_vec());
            }

            if first_match {
                if let Some(src) = src_matches {
                    matches.copy_from_slice(src);
                }
                first_match = false;
            }

            let mut recursive_matches = matches.to_vec();
            let mut rec_count = *recursion_count;
            let (matched, recursive_score, rec_matches_result) = fuzzy_match_recursive(
                pattern,
                str,
                pattern_cur_index,
                str_curr_index + 1,
                Some(matches),
                &mut recursive_matches,
                max_matches,
                next_match,
                &mut rec_count,
                recursion_limit,
                weights,
            );
            *recursion_count = rec_count;

            if matched {
                if !recursive_match || recursive_score > best_recursive_score {
                    best_recursive_matches = rec_matches_result;
                    best_recursive_score = recursive_score;
                }
                recursive_match = true;
            }

            matches[next_match] = str_curr_index;
            next_match += 1;
            pattern_cur_index += 1;
        }
        str_curr_index += 1;
    }

    let matched = pattern_cur_index == pattern.len();

    if matched {
        out_score = 100;

        let mut penalty = weights.leading_letter_penalty * (matches[0] as i32);
        if penalty < weights.max_leading_letter_penalty {
            penalty = weights.max_leading_letter_penalty;
        }
        out_score += penalty;

        let unmatched = (str.len() - next_match) as i32;
        out_score += weights.unmatched_letter_penalty * unmatched;

        for i in 0..next_match {
            let curr_idx = matches[i];

            if i > 0 {
                let prev_idx = matches[i - 1];
                if curr_idx == prev_idx + 1 {
                    out_score += weights.sequential_bonus;
                }
            }

            if curr_idx > 0 {
                let neighbor = str[curr_idx - 1];
                let curr = str[curr_idx];

                let neighbor_str = neighbor.to_string();
                let curr_str = curr.to_string();

                if neighbor_str != neighbor_str.to_uppercase()
                    && curr_str != curr_str.to_lowercase()
                {
                    out_score += weights.camel_bonus;
                }

                if neighbor == '_' || neighbor == ' ' {
                    out_score += weights.separator_bonus;
                }
            } else {
                out_score += weights.first_letter_bonus;
            }
        }

        if recursive_match && (!matched || best_recursive_score > out_score) {
            return (true, best_recursive_score, best_recursive_matches);
        } else {
            return (true, out_score, matches.to_vec());
        }
    }

    (false, out_score, matches.to_vec())
}

fn calc_match_ranges(matches: &[usize]) -> Vec<(usize, usize)> {
    if matches.is_empty() {
        return Vec::new();
    }

    let mut ranges = Vec::new();
    let mut start = matches[0];
    let mut curr = start;

    for &m in matches {
        if m == curr || m == curr + 1 {
            curr = m;
        } else {
            ranges.push((start, curr - start + 1));
            start = m;
            curr = m;
        }
    }

    ranges.push((start, curr - start + 1));
    ranges
}

fn escape_html(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#039;")
}
