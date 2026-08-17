//! Readline helper: operation-name completion for the interactive shell.

use rustyline::{
    completion::{Completer, Pair},
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
    Context, Helper,
};

use rxchef::runtime;

#[derive(Clone)]
pub(crate) struct ReplHelper {
    pub(crate) candidates: Vec<String>,
}

impl Helper for ReplHelper {}

impl Hinter for ReplHelper {
    type Hint = String;
}

impl Highlighter for ReplHelper {}

impl Validator for ReplHelper {}

impl Completer for ReplHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let start = line[..pos]
            .rfind(|character: char| character.is_whitespace() || character == '|')
            .map_or(0, |index| index + 1);
        let needle = line[start..pos].to_ascii_lowercase();
        let matches = self
            .candidates
            .iter()
            .filter(|candidate| candidate.to_ascii_lowercase().starts_with(&needle))
            .map(|candidate| Pair {
                display: candidate.clone(),
                replacement: candidate.clone(),
            })
            .collect();
        Ok((start, matches))
    }
}

pub(crate) fn repl_candidates() -> Vec<String> {
    let mut candidates = vec![
        "help".into(),
        "data".into(),
        "show".into(),
        "clear".into(),
        "list".into(),
        "exit".into(),
        "quit".into(),
    ];
    for name in runtime::operation_names(None) {
        candidates.push(runtime::canonical_identifier(&name));
        candidates.push(
            name.chars()
                .filter(|character| character.is_alphanumeric())
                .collect::<String>()
                .to_ascii_lowercase(),
        );
    }
    candidates.sort();
    candidates.dedup();
    candidates
}
