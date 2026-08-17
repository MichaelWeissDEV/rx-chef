//! Shared alphabet-range expansion for the Base-N operations.
//!
//! `To Base64`, `From Base64`, `To Base85`, `From Base85` and the Base32 pair
//! each carried their own expansion. They were not equivalent: `To Base64`
//! only performed a literal substring replacement for the standard alphabet,
//! so it rejected `A-Za-z0-9-_` — an alphabet `From Base64` accepted and
//! upstream CyberChef supports — with "Must be 64 chars". Encoding and
//! decoding therefore disagreed about which alphabets exist.
//!
//! This is a direct port of upstream's `Utils.expandAlphRange`, which is the
//! single definition every Base-N operation there shares.

/// Expand `a-z` style ranges and `\-` escapes into a literal character list.
///
/// The rules, matching upstream exactly:
///
/// * `X-Y` expands to every code point from `X` to `Y` inclusive, provided the
///   character before `-` is not a backslash.
/// * `\-` is a literal hyphen.
/// * Everything else is taken literally, including a trailing `-`.
///
/// Expansion is deliberately pure: it does not strip padding characters or
/// validate length, because those rules differ per codec and belong to the
/// caller.
pub fn expand_alphabet(alphabet: &str) -> String {
    let chars: Vec<char> = alphabet.chars().collect();
    let mut expanded = String::with_capacity(chars.len());
    let mut index = 0;

    while index < chars.len() {
        let has_range = index + 2 < chars.len() && chars[index + 1] == '-' && chars[index] != '\\';
        if has_range {
            let start = chars[index] as u32;
            let end = chars[index + 2] as u32;
            // A descending range yields nothing rather than panicking.
            for code in start..=end.max(start) {
                if code >= start && code <= end {
                    if let Some(character) = char::from_u32(code) {
                        expanded.push(character);
                    }
                }
            }
            index += 3;
            continue;
        }
        if index + 2 < chars.len() && chars[index] == '\\' && chars[index + 1] == '-' {
            expanded.push('-');
            index += 2;
            continue;
        }
        expanded.push(chars[index]);
        index += 1;
    }

    expanded
}

/// Expand an alphabet and drop the padding character, for codecs whose
/// alphabet argument includes `=` but whose symbol table must not.
pub fn expand_alphabet_without_padding(alphabet: &str) -> String {
    expand_alphabet(alphabet).replace('=', "")
}

#[cfg(test)]
mod tests {
    use super::{expand_alphabet, expand_alphabet_without_padding};

    #[test]
    fn expands_the_standard_base64_alphabet() {
        assert_eq!(
            expand_alphabet("A-Za-z0-9+/="),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="
        );
    }

    #[test]
    fn expands_the_url_safe_alphabet() {
        // Regression: `To Base64` used to reject this alphabet outright.
        let expanded = expand_alphabet("A-Za-z0-9-_");
        assert_eq!(expanded.chars().count(), 64);
        assert!(expanded.ends_with("-_"));
    }

    #[test]
    fn expands_a_fully_custom_alphabet() {
        let expanded = expand_alphabet("./0-9A-Za-z");
        assert_eq!(expanded.chars().count(), 64);
        assert!(expanded.starts_with("./0123456789A"));
    }

    #[test]
    fn a_trailing_hyphen_is_literal() {
        assert_eq!(expand_alphabet("abc-"), "abc-");
    }

    #[test]
    fn an_escaped_hyphen_is_literal() {
        assert_eq!(expand_alphabet("a\\-z"), "a-z");
    }

    #[test]
    fn characters_without_ranges_pass_through() {
        assert_eq!(expand_alphabet("!@#$%"), "!@#$%");
        assert_eq!(expand_alphabet(""), "");
    }

    #[test]
    fn single_character_range_yields_one_character() {
        assert_eq!(expand_alphabet("a-a"), "a");
    }

    #[test]
    fn descending_range_yields_nothing() {
        assert_eq!(expand_alphabet("z-a"), "");
    }

    #[test]
    fn padding_is_removed_only_by_the_padding_aware_helper() {
        assert!(expand_alphabet("A-Za-z0-9+/=").ends_with('='));
        assert_eq!(
            expand_alphabet_without_padding("A-Za-z0-9+/=")
                .chars()
                .count(),
            64
        );
    }

    #[test]
    fn base85_alphabet_expands_to_85_symbols() {
        assert_eq!(expand_alphabet("!-u").chars().count(), 85);
    }
}
