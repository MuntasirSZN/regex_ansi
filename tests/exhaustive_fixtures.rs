//! Exhaustive parity tests ported from chalk/ansi-regex test.js + ansi-codes fixtures.

use regex_ansi::ansi_regex;

// Consumption characters from upstream test (kept identical ordering)
const CONSUMPTION_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()_+1234567890-=[]{};'\":./>?,<\\|";

// Full fixture maps transcribed from fixtures/ansi-codes.js
fn vt52_codes() -> &'static [(&'static str, &'static str)] {
    &[
        ("A", "Cursor up"),
        ("B", "Cursor down"),
        ("C", "Cursor right"),
        ("D", "Cursor left"),
        ("H", "Cursor to home"),
        ("I", "Reverse line feed"),
        ("J", "Erase to end of screen"),
        ("K", "Erase to end of line"),
        ("S", "Scroll up"),
        ("T", "Scroll down"),
        ("Z", "Identify"),
        ("=", "Enter alternate keypad mode"),
        (">", "Exit alternate keypad mode"),
        ("1", "Graphics processor on"),
        ("2", "Graphics processor off"),
        ("<", "Enter ANSI mode"),
        ("s", "Cursor save"),
        ("u", "Cursor restore"),
    ]
}

fn ansi_compatible_codes() -> &'static [(&'static str, &'static str)] {
    &[
        ("[176A", "Cursor up Pn lines"),
        ("[176B", "Cursor down Pn lines"),
        ("[176C", "Cursor forward Pn characters (right)"),
        ("[176D", "Cursor backward Pn characters (left)"),
        ("[176;176H", "Direct cursor addressing"),
        ("[176;176f", "Direct cursor addressing"),
        ("7", "Save cursor and attributes"),
        ("8", "Restore cursor and attributes"),
        ("#3", "Change line double-height top"),
        ("#4", "Change line double-height bottom"),
        ("#5", "Single-width single-height"),
        ("#6", "Double-width single-height"),
        ("[176;176;176;176;176;176;176m", "Text Styles"),
        ("[176;176;176;176;176;176;176q", "Programmable LEDs"),
        ("[K", "Erase from cursor to end of line"),
        ("[0K", "Erase from cursor to end same"),
        ("[1K", "Erase from beginning to cursor"),
        ("[2K", "Erase line"),
        ("[J", "Erase from cursor to end of screen"),
        ("[0J", "Erase from cursor to end same"),
        ("[2J", "Erase entire screen"),
        ("[P", "Delete character"),
        ("[0P", "Delete character (0P)"),
        ("[2P", "Delete 2 characters"),
        ("(A", "UK charset G0"),
        (")A", "UK charset G1"),
        ("(B", "USASCII G0"),
        (")B", "USASCII G1"),
        ("(0", "Special graphics G0"),
        (")0", "Special graphics G1"),
        ("(1", "Alt char ROM G0"),
        (")1", "Alt char ROM G1"),
        ("(2", "Alt graphic ROM G0"),
        (")2", "Alt graphic ROM G1"),
        ("H", "Set tab"),
        ("[g", "Clear tab"),
        ("[0g", "Clear tab same"),
        ("[3g", "Clear all tabs"),
        ("[6n", "Cursor position report"),
        ("[176;176R", "Cursor position response"),
        ("[5n", "Status report"),
        ("[c", "Terminal ok"),
        ("[0c", "Terminal not ok"),
        ("[?1;176c", "Option present"),
        ("c", "Power-up reset"),
        ("#8", "Fill screen with E"),
        ("[2;176y", "Invoke tests"),
    ]
}

fn common_codes() -> &'static [(&'static str, &'static str)] {
    &[
        ("[176A", "Move cursor up n lines"),
        ("[176B", "Move cursor down n lines"),
        ("[176C", "Move cursor right n lines"),
        ("[176D", "Move cursor left n lines"),
        ("[176;176H", "Move cursor to v,h"),
        ("[176;176f", "Move cursor to v,h"),
        ("[176;176r", "Set top and bottom lines"),
        ("[176;176R", "Response cursor at v,h"),
        ("[?1;1760c", "Response terminal type code n"),
        ("[20h", "Set new line mode"),
        ("[?1h", "Set cursor key application"),
        ("[?3h", "Set columns 132"),
        ("[?4h", "Set smooth scrolling"),
        ("[?5h", "Set reverse video"),
        ("[?6h", "Set origin relative"),
        ("[?7h", "Set auto-wrap"),
        ("[?8h", "Set auto-repeat"),
        ("[?9h", "Set interlacing"),
        ("[20l", "Set line feed mode"),
        ("[?1l", "Set cursor key normal"),
        ("[?2l", "Set VT52"),
        ("[?3l", "Set columns 80"),
        ("[?4l", "Set jump scrolling"),
        ("[?5l", "Set normal video"),
        ("[?6l", "Set origin absolute"),
        ("[?7l", "Reset auto-wrap"),
        ("[?8l", "Reset auto-repeat"),
        ("[?9l", "Reset interlacing"),
        ("N", "Single shift 2"),
        ("O", "Single shift 3"),
        ("[m", "SGR reset"),
        ("[0m", "SGR reset explicit"),
        ("[1m", "Bold on"),
        ("[2m", "Low intensity on"),
        ("[4m", "Underline on"),
        ("[5m", "Blink on"),
        ("[7m", "Reverse video on"),
        ("[8m", "Invisible on"),
        ("[9m", "Strikethrough on"),
        ("[22m", "Bold off"),
        ("[23m", "Italics off"),
        ("[24m", "Underline off"),
        ("[27m", "Inverse off"),
        ("[29m", "Strikethrough off"),
        ("[30m", "FG black"),
        ("[31m", "FG red"),
        ("[32m", "FG green"),
        ("[33m", "FG yellow"),
        ("[34m", "FG blue"),
        ("[35m", "FG magenta"),
        ("[36m", "FG cyan"),
        ("[37m", "FG white"),
        ("[39m", "FG default"),
        ("[40m", "BG black"),
        ("[41m", "BG red"),
        ("[42m", "BG green"),
        ("[43m", "BG yellow"),
        ("[44m", "BG blue"),
        ("[45m", "BG magenta"),
        ("[46m", "BG cyan"),
        ("[47m", "BG white"),
        ("[49m", "BG default"),
        ("[H", "Cursor home"),
        ("[;H", "Cursor home variant"),
        ("[f", "hvhome"),
        ("[;f", "hvhome variant"),
        ("M", "Scroll window down"),
        ("E", "Next line"),
        ("H", "Set tab"),
        ("[g", "Clear tab"),
        ("[0g", "Clear tab same"),
        ("[3g", "Clear all tabs"),
        ("[K", "Clear line right"),
        ("[0K", "Clear line right same"),
        ("[1K", "Clear line left"),
        ("[2K", "Clear entire line"),
        ("[J", "Clear screen down"),
        ("[0J", "Clear screen down same"),
        ("[1J", "Clear screen up"),
        ("[2J", "Clear entire screen"),
        ("[c", "Identify terminal"),
        ("[0c", "Identify terminal alt"),
        ("c", "Reset terminal"),
        ("[2;1y", "Test"),
        ("[2;2y", "Confidence loopback"),
        ("[2;9y", "Repeat power up"),
        ("[2;10y", "Repeat loopback"),
        ("[0q", "LED off"),
        ("[1q", "LED1 on"),
        ("[2q", "LED2 on"),
        ("[3q", "LED3 on"),
        ("[4q", "LED4 on"),
    ]
}

fn other_codes() -> &'static [(&'static str, &'static str)] {
    &[
        ("7", "Save cursor position and attributes"),
        ("8", "Restore cursor position and attributes"),
        ("=", "Alternate keypad mode"),
        (">", "Numeric keypad mode"),
        ("(A", "UK G0"),
        (")A", "UK G1"),
        ("(B", "US G0"),
        (")B", "US G1"),
        ("(0", "G0 special chars"),
        (")0", "G1 special chars"),
        ("(1", "G0 alt char ROM"),
        (")1", "G1 alt char ROM"),
        ("(2", "G0 alt char + spec"),
        (")2", "G1 alt char + spec"),
        ("#3", "Double-height top"),
        ("#4", "Double-height bottom"),
        ("#5", "Single width height"),
        ("#6", "Double width single height"),
        ("#8", "Screen alignment display"),
        ("5n", "Device status report"),
        ("0n", "Terminal OK"),
        ("3n", "Terminal not OK"),
        ("6n", "Get cursor position"),
    ]
}

fn urxvt_codes() -> &'static [(&'static str, &'static str)] {
    &[
        ("[5~", "Prior"),
        ("[6~", "Next"),
        ("[7~", "Home"),
        ("[8~", "End"),
        ("[A", "Up"),
        ("[B", "Down"),
        ("[C", "Right"),
        ("[D", "Left"),
        ("[3;5;5t", "C-M-q"),
        ("[3;5;606t", "C-M-y"),
        ("[3;1605;5t", "C-M-e"),
        ("[3;1605;606t", "C-M-c"),
        ("]710;9x15bold\u{0007}", "font"),
    ]
}

#[test]
fn upstream_sample_sequences_match() {
    let re = ansi_regex();
    // Core examples from test.js header
    for sample in [
        "foo\x1b[4mcake\x1b[0m",
        "\x1b[4mcake\x1b[0m",
        "\x1b[0m\x1b[4m\x1b[42m\x1b[31mfoo\x1b[39m\x1b[49m\x1b[24mfoo\x1b[0m",
        // ESC m (implicit SGR reset) now explicitly supported in pattern via short escape branch
        "foo\x1bmfoo",
    ] {
        assert!(re.is_match(sample), "expected match for {sample:?}");
    }
}

#[test]
fn ls_style_sequence() {
    assert!(ansi_regex().is_match("\x1b[00;38;5;244m\x1b[m\x1b[00;38;5;33mfoo\x1b[0m"));
}

#[test]
fn reset_fg_bg_italics_strike_underline_combo_extract() {
    let re = ansi_regex();
    let s = "foo\x1b[0;33;49;3;9;4mbar";
    let caps: Vec<_> = re.find_iter(s).map(|m| m.as_str()).collect();
    assert_eq!(caps[0], "\x1b[0;33;49;3;9;4m");
}

#[test]
fn clear_tabs_sequence() {
    let s = "foo\x1b[0gbar"; // ESC [0g in JS is represented as ESC [0g but here pattern matches ESC [0g; upstream writes 0g without '[' due to parsing? Actually test has \u001B[0g
    let re = ansi_regex();
    let first = re.find(s).unwrap().as_str();
    assert_eq!(first, "\x1b[0g");
}

#[test]
fn clear_line_cursor_right() {
    assert_eq!(first_match("foo\x1b[Kbar"), Some("\x1b[K"));
}
#[test]
fn clear_screen() {
    assert_eq!(first_match("foo\x1b[2Jbar"), Some("\x1b[2J"));
}

#[test]
fn only_first_behavior() {
    let re = ansi_regex();
    let all: Vec<_> = re.find_iter("foo\x1b[4mcake\x1b[0m").collect();
    assert!(all.len() >= 2);
}

#[test]
fn terminal_link_variants() {
    let sts = ["\x07", "\x1b\\", "\u{009C}"];
    let re = ansi_regex();
    for st in sts {
        let open = format!("\x1b]8;k=v;https://example-a.com/?a_b=1&c=2#tit%20le{}", st);
        let close = format!("\x1b]8;;{}", st);
        let value = format!("{}click{}", open, close);
        let matches: Vec<_> = re.find_iter(&value).map(|m| m.as_str()).collect();
        assert_eq!(matches, vec![open.as_str(), close.as_str()]);
        // plus variant
        let plus_open = format!("\x1b]8;;https://www.example.com/?q=hello+world{}", st);
        let plus_close = format!("\x1b]8;;{}", st);
        let plus_val = format!("{}hello{}", plus_open, plus_close);
        let plus_matches: Vec<_> = re.find_iter(&plus_val).map(|m| m.as_str()).collect();
        assert_eq!(plus_matches, vec![plus_open.as_str(), plus_close.as_str()]);
        assert_eq!(re.replace_all(&plus_val, ""), "hello");
    }
}

#[test]
fn colon_separated_sequences() {
    let samples = [
        "\x1b[38:2:68:68:68:48:2:0:0:0m",
        "\x1b[4:0m",
        "\x1b[4:1m",
        "\x1b[4:2m",
        "\x1b[4:3m",
        "\x1b[4:4m",
        "\x1b[4:5m",
        "\x1b[38:5:123m",
        "\x1b[48:5:200m",
        "\x1b[58:5:200m",
        "\x1b[38:2::12:34:56m",
        "\x1b[48:2::200:201:202m",
        "\x1b[58:2::255:0:0m",
        "\x1b[38:6::255:0:0:128m",
        "\x1b[48:6::0:0:0:64m",
    ];
    let _re = ansi_regex();
    for s in samples {
        assert_eq!(first_match(s).unwrap(), s);
    }
}

#[test]
fn colon_sequences_no_overconsume() {
    let samples = [
        "\x1b[4:5mX",
        "\x1b[38:5:123mX",
        "\x1b[58:2::255:0:0mX",
        "\x1b[38:2::12:34:56mX",
        "\x1b[48:2::200:201:202mX",
    ];
    let re = ansi_regex();
    for s in samples {
        let m = re.find(s).unwrap().as_str();
        assert_eq!(s.strip_suffix('X').unwrap(), m);
        assert_eq!(re.replace(s, ""), "X");
    }
}

#[test]
fn negative_no_esc_bracketed() {
    let samples = [
        "[38:2:68:68:68m",
        "[4:5m",
        "some [0m text",
        "plain [58:2::255:0:0m words",
    ];
    let re = ansi_regex();
    for s in samples {
        assert!(re.find(s).is_none(), "false positive {s}");
    }
}

#[test]
fn negative_incomplete_csi() {
    assert!(ansi_regex().find("\x1b[").is_none());
}

#[test]
fn negative_unsupported_final() {
    assert!(ansi_regex().find("pre\x1b`post").is_none());
}

#[test]
fn exhaustive_code_sets_basic() {
    let re = ansi_regex();
    for &(code, _) in vt52_codes() {
        check_code(re, code);
    }
    for &(code, _) in ansi_compatible_codes() {
        check_code(re, code);
    }
    for &(code, _) in common_codes() {
        check_code(re, code);
    }
    for &(code, _) in other_codes() {
        check_code(re, code);
    }
    for &(code, _) in urxvt_codes() {
        check_code(re, code);
    }
}

fn check_code(re: &regex::Regex, code: &str) {
    // Skip if ends in digit (upstream test logic)
    if code
        .chars()
        .last()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        return;
    }
    let esc = format!("\x1b{}", code);
    let wrapped = format!("hel{}lo", esc);
    assert!(re.is_match(&wrapped), "did not match {code}");
    let first = re.find(&wrapped).unwrap().as_str();
    assert_eq!(first, esc, "overconsumed {code}");
    assert_eq!(re.replace(&wrapped, ""), "hello");
    for ch in CONSUMPTION_CHARS.chars() {
        let sample = format!("{}{}", esc, ch);
        let m = re.find(&sample).unwrap().as_str();
        assert_eq!(m, esc, "overconsume {code} before {ch}");
        assert_eq!(re.replace(&sample, ""), ch.to_string());
    }
}

fn first_match(s: &str) -> Option<&str> {
    ansi_regex().find(s).map(|m| m.as_str())
}
