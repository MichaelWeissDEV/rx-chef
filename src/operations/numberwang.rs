/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Numberwang operation.
 * -----------------------------------------------------------------------------
 */

use rand::seq::SliceRandom;
use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Numberwang operation
pub struct Numberwang;

const DID_YOU_KNOW: &[&str] = &[
    "Numberwang, contrary to popular belief, is a fruit and not a vegetable.",
    "Robert Webb once got WordWang while presenting an episode of Numberwang.",
    "The 6705th digit of pi is Numberwang.",
    "Numberwang was invented on a Sevenday.",
    "Contrary to popular belief, Albert Einstein always got good grades in Numberwang at school. He once scored ^4$ on a test.",
    "680 asteroids have been named after Numberwang.",
    "Archimedes is most famous for proclaiming \"That's Numberwang!\" during an epiphany about water displacement he had while taking a bath.",
    "Numberwang Day is celebrated in Japan on every day of the year apart from June 6.",
    "Biologists recently discovered Numberwang within a strand of human DNA.",
    "Numbernot is a special type of non-Numberwang number. It is divisible by 3 and the letter \"y\".",
    "Julie once got 612.04 Numberwangs in a single episode of Emmerdale.",
    "In India, it is traditional to shout out \"Numberwang!\" instead of checkmate during games of chess.",
    "There is a rule on Countdown which states that if you get Numberwang in the numbers round, you automatically win. It has only ever been invoked twice.",
    "\"Numberwang\" was the third-most common baby name for a brief period in 1722.",
    "\"The Lion King\" was loosely based on Numberwang.",
    "\"A Numberwang a day keeps the doctor away\" is how Donny Cosy, the oldest man in the world, explained how he was in such good health at the age of 136.",
    "The \"number lock\" button on a keyboard is based on the popular round of the same name in \"Numberwang\".",
    "Cambridge became the first university to offer a course in Numberwang in 1567.",
    "Schrdinger's Numberwang is a number that has been confusing dentists for centuries.",
    "\"Harry Potter and the Numberwang of Numberwang\" was rejected by publishers -41 times before it became a bestseller.",
    "\"Numberwang\" is the longest-running British game show in history; it has aired 226 seasons, each containing 19 episodes, which makes a grand total of 132 episodes.",
    "The triple Numberwang bonus was discovered by archaeologist Thomas Jefferson in Somerset.",
    "Numberwang is illegal in parts of Czechoslovakia.",
    "Numberwang was discovered in India in the 12th century.",
    "Numberwang has the chemical formula Zn4SO2(HgEs)3.",
    "The first pack of cards ever created featured two \"Numberwang\" cards instead of jokers.",
    "Julius Caesar was killed by an overdose of Numberwang.",
    "The most Numberwang musical note is G#.",
    "In 1934, the forty-third Google Doodle promoted the upcoming television show \"Numberwang on Ice\".",
    "A recent psychology study found that toddlers were 17% faster at identifying numbers which were Numberwang.",
    "There are 700 ways to commit a foul in the television show \"Numberwang\". All 700 of these fouls were committed by Julie in one single episode in 1473.",
    "Astronomers suspect God is Numberwang.",
    "Numberwang is the official beverage of Canada.",
    "In the pilot episode of \"The Price is Right\", if a contestant got the value of an item exactly right they were told \"That's Numberwang!\" and immediately won 5.7032.",
    "The first person to get three Numberwangs in a row was Madonna.",
    "\"Numberwang\" has the code U+46402 in Unicode.",
    "The musical note \"Numberwang\" is between D# and E.",
    "Numberwang was first played on the moon in 1834.",
];

impl Operation for Numberwang {
    fn name(&self) -> &'static str {
        "Numberwang"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Based on the popular gameshow by Mitchell and Webb."
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

        let output = if input_str.is_empty() {
            "Let's play Wangernumb!".to_string()
        } else {
            let re = Regex::new(
                r"(?i)(f0rty-s1x|shinty-six|filth-hundred and neeb|-??\d+(\.\d+)?i?([a-z]?)%?)",
            )
            .unwrap();
            if let Some(caps) = re.captures(&input_str) {
                let matched = caps.get(0).unwrap().as_str();
                if caps.get(3).map_or(false, |m| !m.as_str().is_empty()) {
                    format!("{}! That's AlphaNumericWang!", matched)
                } else {
                    format!("{}! That's Numberwang!", matched)
                }
            } else {
                "Sorry, that's not Numberwang. Let's rotate the board!".to_string()
            }
        };

        let mut rng = rand::thread_rng();
        let fact = DID_YOU_KNOW.choose(&mut rng).unwrap_or(&"");

        Ok(format!("{}\n\nDid you know: {}", output, fact).into_bytes())
    }
}
