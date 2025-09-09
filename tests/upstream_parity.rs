use regex_ansi::{ansi_regex, pattern};

// NOTE: This is an approximated port of chalk/ansi-regex test coverage.
// Without the remote fixture files locally, we enumerate representative
// sequences across each category exercised by the original tests.

fn sgr_base_codes() -> Vec<String> {
    let mut v = Vec::new();
    // Reset + common styles
    for code in [0, 1, 2, 3, 4, 7, 8, 9] {
        v.push(format!("\x1b[{}m", code));
    }
    // 30-37 standard fg
    for code in 30..=37 {
        v.push(format!("\x1b[{}m", code));
    }
    // 90-97 bright fg
    for code in 90..=97 {
        v.push(format!("\x1b[{}m", code));
    }
    // 40-47 standard bg
    for code in 40..=47 {
        v.push(format!("\x1b[{}m", code));
    }
    // 100-107 bright bg
    for code in 100..=107 {
        v.push(format!("\x1b[{}m", code));
    }
    // Reset color, default fg/bg
    for code in [39, 49] {
        v.push(format!("\x1b[{}m", code));
    }
    v
}

fn sgr_extended_codes() -> Vec<String> {
    let mut v = Vec::new();
    // 256-color (indexed) examples
    for idx in [0u16, 1, 15, 16, 88, 160, 231, 255] {
        v.push(format!("\x1b[38;5;{}m", idx));
    }
    for idx in [0u16, 1, 160, 255] {
        v.push(format!("\x1b[48;5;{}m", idx));
    }
    // Truecolor examples
    v.push("\x1b[38;2;0;0;0m".to_string());
    v.push("\x1b[38;2;255;128;64m".to_string());
    v.push("\x1b[48;2;12;34;56m".to_string());
    v.push("\x1b[48;2;255;255;255m".to_string());
    v
}

fn other_control_sequences() -> Vec<&'static str> {
    vec![
        // Cursor movement & misc (typical finals in allowed class)
        "\x1b[2J",     // clear screen
        "\x1b[0J",     // erase below
        "\x1b[1J",     // erase above
        "\x1b[2K",     // erase line
        "\x1b[10A",    // cursor up 10
        "\x1b[10;20H", // cursor position
        "\x1b[?25l",   // hide cursor
        "\x1b[?25h",   // show cursor
        "\x1b7",       // save cursor (ESC 7)
        "\x1b8",       // restore cursor (ESC 8)
    ]
}

fn osc_sequences() -> Vec<String> {
    vec![
        // Title setting (OSC 0/2 style) with BEL termination
        "\x1b]0;My Title\x07".to_string(),
        // Hyperlink (OSC 8) BEL termination
        "\x1b]8;;https://example.com\x07label\x1b]8;;\x07".to_string(),
        // Hyperlink ESC backslash termination
        "\x1b]8;;https://example.com\x1b\\label\x1b]8;;\x1b\\".to_string(),
        // Hyperlink 0x9C termination
        format!(
            "\x1b]8;;https://example.com{}label\x1b]8;;{}",
            '\u{9C}', '\u{9C}'
        ),
    ]
}

#[test]
fn pattern_smoke_compiles() {
    let _ = ansi_regex();
}

#[test]
fn matches_all_sgr_base_codes() {
    let re = ansi_regex();
    for seq in sgr_base_codes() {
        assert!(re.is_match(&seq), "missed base SGR: {seq:?}");
    }
}

#[test]
fn matches_all_sgr_extended_codes() {
    let re = ansi_regex();
    for seq in sgr_extended_codes() {
        assert!(re.is_match(&seq), "missed extended SGR: {seq:?}");
    }
}

#[test]
fn matches_other_control_sequences() {
    let re = ansi_regex();
    for seq in other_control_sequences() {
        assert!(re.is_match(seq), "missed control sequence: {seq:?}");
    }
}

#[test]
fn matches_osc_sequences() {
    let re = ansi_regex();
    for seq in osc_sequences() {
        assert!(re.is_match(&seq), "missed OSC: {seq:?}");
    }
}

#[test]
fn strips_from_mixed_string() {
    let re = ansi_regex();
    let mut s = String::new();
    s.push_str("Start ");
    s.push_str("\x1b[31mRED\x1b[0m and ");
    s.push_str("\x1b[38;5;160mIDX\x1b[0m mid ");
    s.push_str("\x1b]0;Title\x07tail");
    let cleaned = re.replace_all(&s, "");
    assert_eq!(cleaned, "Start RED and IDX mid tail");
}

#[test]
fn does_not_match_incomplete_osc() {
    let re = ansi_regex();
    let incomplete = "\x1b]8;;https://example.com"; // missing ST terminator
    assert!(
        re.find(incomplete).is_none(),
        "incomplete OSC sequence should not match"
    );
}

#[test]
fn does_not_consume_plain_text() {
    let re = ansi_regex();
    let text = "Hello [31m world (not really an escape)";
    assert!(re.find(text).is_none());
}

#[test]
fn large_randomish_buffer_performance_smoke() {
    // Build a buffer with many sequences; ensure replacing them works and count matches
    let re = ansi_regex();
    let mut buf = String::with_capacity(200_000);
    for i in 0..5000 {
        buf.push_str(&format!(
            "Line {} \x1b[32mGREEN\x1b[0m segment \x1b[38;5;160mX\x1b[0m\n",
            i
        ));
    }
    let matches: usize = re.find_iter(&buf).count();
    assert!(matches > 10_000, "expected many matches, got {matches}");
    let cleaned = re.replace_all(&buf, "");
    assert!(cleaned.contains("GREEN"));
    assert!(!cleaned.contains("\x1b[32m"));
}

#[test]
fn pattern_constant_stable_snapshot() {
    // If this changes, it should be an intentional semver-impacting decision.
    // Snapshot invariant: key CSI introducers and OSC pieces present.
    assert!(pattern().contains("\\x1B\\["));
    assert!(pattern().contains("\\x9B"));
    assert!(pattern().contains("\\x1B\\]"));
}
