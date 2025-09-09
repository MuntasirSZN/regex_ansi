regex_ansi
=================================

High‑fidelity Rust port of the popular JavaScript `ansi-regex` pattern. Supplies a battle‑tested regular expression that matches ANSI escape sequences (CSI, OSC, etc.).

Example
-------
```rust
use regex_ansi::ansi_regex;

fn main() {
    let text = "\x1b[31mError:\x1b[0m something failed";
    let cleaned = ansi_regex().replace_all(text, "");
    assert_eq!(cleaned, "Error: something failed");
}
```

Features
--------
- Constant, pre‑audited pattern identical in semantics to `ansi-regex@6` (JS).
- Two ready regex constructors: global style (`ansi_regex`) and first match style (`ansi_regex_first`).
- No unsafe code.

API
---
- `pub fn ansi_regex() -> &'static regex::Regex`
- `pub fn ansi_regex_first() -> &'static regex::Regex`
- `pub fn pattern() -> &'static str` – raw pattern string.

Performance Notes
-----------------
- The OSC portion is implemented using a negated character class for linear behavior.

License
-------
Licensed under MIT. Derivative work conceptually based on the pattern logic from the original JavaScript project.
