# regex-ansi Rust Rewrite – Task Plan

Goal: Re‑implement the functionality of the current JavaScript module (`index.js`) in pure Rust while preserving exact observable behavior (pattern semantics, matches, flags, edge‑cases) and offering:
1. A Rust crate (`regex-ansi`) for native Rust consumers.
2. An optional Node.js binding that exposes the same JS API (default export: function(options?: {onlyFirst?: boolean}) => RegExpLike) so downstream JS users can swap seamlessly (drop‑in replacement), if desired.

---
## 1. Functional Parity Specification
- Extract the exact JS pattern produced today.
- Document all constituent parts:
  - OSC sequences: `ESC ] ... ST` (non-greedy up to first valid ST: BEL | ESC \ | 0x9C)
  - CSI & related sequences: `[\u001B\u009B][[\]()#;?]*(?:\d{1,4}(?:[;:]\d{0,4})*)?[\dA-PR-TZcf-nq-uy=><~]`
- Confirm character class ranges & exclusions (notably the final byte class and avoidance of numeric finals for excluded extended codes).
- Enumerate all test cases from `test.js` as normative examples.
- Define “onlyFirst” semantics: JS returns `new RegExp(pattern)` (no `g`) vs global `g` when not onlyFirst.

Deliverables:
- `SPEC.md` (generated later) summarizing grammar + rationale.

---
## 2. Rust Crate Scaffolding
- Create `Cargo.toml` with MIT license metadata matching original.
- Module layout:
  - `src/lib.rs` – public API.
  - `pattern.rs` – pattern assembly (string literal or builder).
  - `tests/` – mirrored tests.
- Feature flags:
  - `std` (default) vs potential `no_std` (stretch) – ensure pattern storage only.
  - `regex` vs `fancy-regex` (if backtracking needed) – evaluate necessity (likely standard `regex` is enough as pattern is literal alternation + quantified digits; verify).

---
## 3. Pattern Implementation Strategy
- Prefer compile-time constant `&'static str` to avoid runtime allocation.
- Validate Rust `regex` crate compatibility (it does not support `\u{XXXX}` inside sets directly the same way JS does; adapt to UTF-8 bytes / explicit escapes):
  - Replace `\u001B` with `\x1B`, `\u009B` with `\x9B`, `\u0007` with `\x07`, `\u005C` with `\\`, `\u009C` with `\x9C`.
- Ensure non-greedy OSC: Use `.*?` → In Rust `regex`, lazy quantifiers are supported (`.*?`). Confirm performance (bounded by early ST). If catastrophic backtracking risk appears, consider manual scan parser for OSC portion.
- Provide two APIs:
  - `fn regex_ansi() -> Regex` (global-style) – equivalence to JS default.
  - `fn regex_ansi_first() -> Regex` – for onlyFirst semantics.
  - Or a builder: `AnsiRegex::new().only_first(true).build()`.
- Consider returning a lightweight wrapper if Node binding wants raw string.

---
## 4. Testing (Rust Side)
- Port JS tests to Rust:
  - Recreate literals using `"\u{1B}[4m"` etc. or hex escapes.
  - Terminal link (OSC 8) cases with all three ST terminators.
  - Overconsumption checks: ensure replacement leaves expected tail.
  - Negative tests (no false positives for incomplete ESC sequences, bracketed text sans ESC, unsupported finals).
- Extended code tables: Convert `fixtures/ansi-codes.js` maps into `lazy_static!` / arrays; iterate to auto-generate tests (skip codes ending in digit as JS does).
- Property-based (stretch): use `proptest` to generate random strings with embedded valid sequences and assert round‑trip removal vs JS implementation reference output (optional if Node binding present to cross-call).

---
## 5. Node.js Binding (Optional but “exactly the same” implies)
- Use `napi-rs` (prefer) or `neon` for minimal overhead.
- Expose function `ansiRegex(options?)`:
  - Build pattern once (static) and cache two variants (with and without global flag semantics). Since JS RegExp object creation with flags must return genuine `RegExp`, consider returning a JS `RegExp` constructed from the pattern string (`new RegExp(pattern, onlyFirst? '' : 'g')`). Simpler: binding returns the pattern string; JS shim constructs RegExp (less native complexity).
- Package layout:
  - `npm/` folder with `index.js` acting as facade: If native available load pattern from native; else fallback to pure JS (current implementation) for portability.
- Provide prebuild script for common targets (stretch task).

---
## 6. Benchmarking
- Benchmarks comparing JS vs Rust removal throughput on large strings with many sequences.
- Use `criterion` for Rust; for JS use node script. Document results in `BENCHMARKS.md`.

---
## 7. Validation Against Original JS
- Snapshot: For a corpus of test strings (aggregate from all tests + synthetic), run both implementations and diff:
  - All matches array order identical.
  - Replacement (`str.replace(regex, '')`) equivalence.
- Provide `verify_parity.js` script calling Rust via CLI (emit JSON) OR Node binding directly.

---
## 8. Documentation
- Update README (Rust section): installation (`cargo add regex-ansi`), examples, caution on untrusted input timeouts (point to `regex` crate `Regex::find_iter` deterministic behavior) and parity statement.
- Add crate-level docs including grammar explanation.
- Note licensing attribution to original author & repository.

---
## 9. CI & Tooling
- GitHub Actions workflows:
  - Rust: fmt check, clippy (deny warnings), tests.
  - JS: existing tests still pass (if binding added, add a matrix with/without native build).
- Optional: prebuild workflow for publishing Node binary artifacts.

---
## 10. Publishing Steps
- Reserve crate name (verify availability).
- Tag version `0.1.0` (semantic parity baseline).
- For Node: bump minor when switching default implementation to native (if not immediate).

---
## 11. Risk & Edge Considerations
- Rust `regex` vs JS engine differences: ensure no unintended Unicode class expansions; pattern is plain ASCII / explicit bytes.
- Potential backtracking in OSC portion if extremely long strings lacking ST; mitigate by manual scan fallback (stretch) if benchmarks show slowdown.
- Avoid exposing unsafe APIs; no allocations in hot path beyond regex engine.

---
## 12. Stretch / Future Enhancements
- Add function to strip ANSI codes directly (`strip_ansi(&str) -> Cow<str>`), using manual scanner for maximum speed.
- Add `no_std` (alloc only) variant with feature gating.
- WASM build (publish to npm as alternative to native bindings).
- Compile-time generation of test vectors from a single CSV / JSON spec.

---
## Task Breakdown (Implemented)
1. Extract & freeze current JS pattern (DONE – `ANSI_REGEX_PATTERN`).
2. Create Rust crate scaffold (DONE – lib crate, removed binary main).
3. Implement constant pattern & public API (DONE – functions + docs).
4. Port representative tests (DONE – see `src/lib.rs` tests module).
5. Extended dynamic tests (DEFERRED – not yet ported).
6. Property-based tests (DEFERRED – optional).
7. Node binding strategy (DEFERRED – Rust only for now).
8. Parity verification script (DEFERRED – future when integrating JS harness).
9. Benchmarks (DONE – criterion bench stub `benches/throughput.rs`).
10. Documentation (DONE – README + SPEC.md).
11. CI workflow (DONE – `.github/workflows/ci.yml`).
12. Publish prep (PARTIAL – metadata present, awaiting real repository URL & crates.io publish).

---
## Acceptance Criteria (Current Status)
- Core pattern constant + lazy compilation (MET).
- Representative tests pass (MET).
- Overconsumption avoidance (MET via tests & scanner logic).
- Extended parity / exhaustive tests (OUTSTANDING – future work).
- Property tests (OUTSTANDING – optional).
- Bench stub present (MET).
- Documentation & spec added (MET).

---
## Open Questions (To Clarify Before Implementation)
- Is Node binding required immediately, or is a Rust-only deliverable acceptable as Phase 1? (Answer Rust only)
- Should crate name mirror npm (`regex-ansi`) or follow Rust snake_case (`regex_ansi`)? (`regex_ansi`)
- Appetite for `strip_ansi` helper inclusion in initial release? (i will create another rust crate for that :))

Implementation completed for core scope; outstanding deferred items listed above.
