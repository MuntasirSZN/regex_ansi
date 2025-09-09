//! regex_ansi: ANSI escape sequence matching.
//!
//! High‑fidelity Rust port of the JavaScript `ansi-regex` pattern.

use regex::Regex;
use std::sync::LazyLock;

// Pattern derived to align with ansi-regex JS semantics (v6 era):
// Matches:
// 1. OSC: ESC ] ... (terminated by BEL | ESC \\ | 0x9C) – non-greedy.
// 2. CSI / control sequences with parameter bytes and a final byte in valid range.
// 3. Other 2-byte escape sequences used by some terminals.
// This pattern intentionally does not attempt to validate every numeric range; it
// mirrors practical coverage of color/style + link sequences.
// The pattern is kept as a single constant string literal for compile-time embedding.

// Simplified & Rust-regex-compatible form (negated char class for OSC body):
//  - OSC: ESC ] then any bytes except BEL, ESC, 0x9C lazily until a terminator BEL | ESC \ | 0x9C
//  - CSI/other: same final-byte class as ansi-regex JS.
// Adjusted pattern for Rust `regex` crate limitations:
// - Replace nested `[[` consumption with an explicit character class that does
//   not prematurely introduce an unclosed set (Clippy previously flagged it).
// - Equivalent intent: match ESC or single-byte CSI, then zero+ of the allowed
//   parameter/intermediate bytes, then a final byte from the upstream class.
// Pattern components:
// 1. OSC: ESC ] ... (terminator BEL | ESC \\ | 0x9C) with lazy body so incomplete OSC doesn't match.
// 2. CSI (two forms): ESC [ ... final OR single 0x9B ... final.
// 3. VT52 & short escapes: ESC followed by a single char from allowed set.
// 4. Charset selection: ESC ( or ) then one of A B 0 1 2.
// 5. DEC line/screen alignment etc with '#'.
// These extra explicit branches ensure ESC A etc match while ESC ] (incomplete OSC) does not.
pub const ANSI_REGEX_PATTERN: &str = concat!(
    // OSC branch
    "(?:\\x1B\\][^\\x07\\x1B\\x9C]*?(?:\\x07|\\x1B\\\\|\\x9C))",
    "|",
    // CSI ESC[ ...
    "(?:\\x1B\\[[\\[\\]()#;?]*(?:[0-9]{1,4}(?:[;:][0-9]{0,4})*)?[0-9A-PR-TZcf-nq-uy=><~])",
    "|",
    // CSI single-byte 0x9B ...
    "(?:\\x9B[\\[\\]()#;?]*(?:[0-9]{1,4}(?:[;:][0-9]{0,4})*)?[0-9A-PR-TZcf-nq-uy=><~])",
    "|",
    // VT52 / short escapes (single final)
    // Added E (NEL), M (RI), c (reset), m (SGR reset), plus existing cursor & mode keys.
    "(?:\\x1B[ABCDHIKJSTZ=><sum78EMcNO])",
    "|",
    // Charset selection ESC (X or )X where X in A B 0 1 2
    "(?:\\x1B[()][AB012])",
    "|",
    // Hash sequences ESC # 3 4 5 6 8
    "(?:\\x1B#[34568])",
    "|",
    // Device status reports / queries: ESC [ 5 n etc (already covered by CSI) but bare 'ESC 5 n' appears in fixtures => add generic ESC [0-9]+[n] pattern fallback
    "(?:\\x1B[0-9]+n)"
);

static ANSI_REGEX_GLOBAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(ANSI_REGEX_PATTERN).expect("valid ANSI regex"));

// For first-match semantics we can reuse same pattern; users just use methods like find.
static ANSI_REGEX_FIRST: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(ANSI_REGEX_PATTERN).expect("valid ANSI regex"));

/// Return the compiled global-style ANSI regex (intended for finding all matches).
pub fn ansi_regex() -> &'static Regex {
    &ANSI_REGEX_GLOBAL
}

/// Return the compiled first-match ANSI regex (semantic helper; identical underlying pattern).
pub fn ansi_regex_first() -> &'static Regex {
    &ANSI_REGEX_FIRST
}

/// Return the raw ANSI regex pattern string.
pub fn pattern() -> &'static str {
    ANSI_REGEX_PATTERN
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_compiles() {
        let _ = ansi_regex();
    }

    #[test]
    fn matches_basic_csi() {
        let text = "\x1b[31mRed\x1b[0m";
        let re = ansi_regex();
        let parts: Vec<_> = re.find_iter(text).map(|m| m.as_str()).collect();
        assert_eq!(parts.len(), 2);
        assert!(parts[0].starts_with("\x1b[31m"));
    }

    #[test]
    fn osc_hyperlink_variants() {
        // OSC 8 ; ; url ST (terminated by BEL)
        let s1 = "\x1b]8;;https://example.com\x07label\x1b]8;;\x07";
        // ESC backslash termination
        let s2 = "\x1b]8;;https://example.com\x1b\\label\x1b]8;;\x1b\\";
        // 0x9C termination
        let s3 = format!(
            "\x1b]8;;https://example.com{}label\x1b]8;;{}",
            '\u{9C}', '\u{9C}'
        );
        for s in [s1, s2, s3.as_str()] {
            assert_eq!(ansi_regex().replace_all(s, ""), "label");
        }
    }

    #[test]
    fn no_false_positive_plain_brackets() {
        let t = "[not an escape]";
        assert!(ansi_regex().find(t).is_none());
    }

    #[test]
    fn incomplete_escape_left_intact() {
        let t = "\x1b"; // lone ESC
        assert!(ansi_regex().find(t).is_none());
    }

    #[test]
    fn large_text_performance_safety() {
        let mut s = String::new();
        for _ in 0..2_000 {
            s.push_str("Line \x1b[32mGREEN\x1b[0m end\n");
        }
        let stripped = ansi_regex().replace_all(&s, "");
        assert!(stripped.contains("GREEN"));
        assert!(!stripped.contains("\x1b[32m"));
    }
}
