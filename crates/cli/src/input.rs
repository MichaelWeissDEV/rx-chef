//! Input selection: literal text, a file, or stdin.

use std::collections::HashMap;
use std::fs;
use std::io::{self, IsTerminal, Read};
use std::path::PathBuf;

pub(crate) struct LoadedInput {
    pub(crate) bytes: Vec<u8>,
    pub(crate) supplied: bool,
}

pub(crate) fn load_input_from(
    text: Option<String>,
    file: Option<PathBuf>,
    trailing_args: &[String],
) -> Result<LoadedInput, String> {
    if let Some(t) = text {
        return Ok(LoadedInput {
            bytes: t.into_bytes(),
            supplied: true,
        });
    }
    if let Some(p) = file {
        let b = fs::read(&p).map_err(|e| format!("cannot read '{}': {}", p.display(), e))?;
        return Ok(LoadedInput {
            bytes: b,
            supplied: true,
        });
    }
    if !trailing_args.is_empty() {
        return Ok(LoadedInput {
            bytes: trailing_args[0].as_bytes().to_vec(),
            supplied: true,
        });
    }
    if !io::stdin().is_terminal() {
        let mut buf = Vec::new();
        io::stdin()
            .read_to_end(&mut buf)
            .map_err(|e| format!("stdin read error: {e}"))?;
        return Ok(LoadedInput {
            bytes: buf,
            supplied: true,
        });
    }
    Ok(LoadedInput {
        bytes: Vec::new(),
        supplied: false,
    })
}

// ─── Output ───────────────────────────────────────────────────────────────────

pub(crate) fn parse_set_vars(raw: &[String]) -> Result<HashMap<String, String>, String> {
    raw.iter()
        .map(|kv| {
            let (key, value) = kv
                .split_once('=')
                .ok_or_else(|| format!("invalid --set value '{}': expected KEY=value", kv))?;
            let k = key.to_uppercase();
            let v = value.to_string();
            if k.is_empty() {
                Err(format!("invalid --set value '{}': expected KEY=value", kv))
            } else {
                Ok((k, v))
            }
        })
        .collect()
}

// ─── Input loading ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::parse_set_vars;

    #[test]
    fn variable_values_may_contain_equals() {
        let vars = parse_set_vars(&["TOKEN=a=b=c".into()]).unwrap();
        assert_eq!(vars["TOKEN"], "a=b=c");
    }

    #[test]
    fn variable_overrides_require_key_value_separator() {
        let error = parse_set_vars(&["KEY".to_string()]).unwrap_err();
        assert!(error.contains("expected KEY=value"));
        assert_eq!(parse_set_vars(&["KEY=".to_string()]).unwrap()["KEY"], "");
    }
}
