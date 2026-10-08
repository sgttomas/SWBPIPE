**piping(T3 B1): up to three load cases in one retained-precision invocation (C = 3 at S3), with the three readers and corpus 07n**

This PR brings B1 to main. The retained-precision route widens from one load case to up to three in one invocation, at D1's full model caps.

- **Admission.** D1.4 admits 1 to 3 load cases (at most 384 loads), with no combination or component. Pressure regions are still refused.
- **The producer.**
  - A case whose ordinary report already passed is not attempted. With none attempted, the exact ordinary bytes are published.
  - Otherwise the attempted cases are prepared, solved in one batch call, frozen and serialized per case, in request order.
  - A fallback publishes one notice per attempted case. One case runs through the same path, and every committed one-case successor is byte-identical.
- **A latent defect of main, repaired.** The producer wrote a load term's `constructor_ordinal` as its canonical position, where the readers expect the authored ordinal. A case whose loads were authored out of order fell back with one notice instead of publishing. It now publishes, and no committed byte changes.
- **The memory threshold.** The registered profile is regenerated at C = 3, and `threshold_bytes` is 11,274,289,152 B (10.5 GiB). The priced worst case is 0.8745 of it (dense).
- **The readers.** The Rust, Python and TypeScript retained readers accept the multi-case successor and agree on every gate and code, apart from the declared raw G7 codes and two declared Python-only transport differences (Python refuses arrays over 16,384 items; Python admits an unsafe `span_index` that Rust refuses at G7 and TypeScript at G1). A few malformed shapes the producer cannot emit are now refused alike.
- **Corpus 07n** is appended to 07m. It adds 9 cases, 240 mutations and 50 must-pass entries, for 26, 534 and 78 in all.
- **Also:** RV95 N-5's direct unit test of the 2^53−1 integer bound, and the witness, challenge and law-test re-pins.
- **Platform-exact tests.** B1's test inputs are spelled as binary64 bits rather than built from libm's `cos` and `sin`. Where the ordinary route's `f64::hypot` differs by one ulp on Linux glibc, two dense pins have exact glibc variants. Each platform must produce its own bytes exactly, and no production code changes.

## What stays closed

- **The Direct entry is in the registered dev/test build only.** Every other build publishes the ordinary bytes.
- **No product caller and no public activation.** Those are B8's.
- **No supported-machine statement,** which stays owner-held. The largest measured RSS at the caps is 206.5 MiB, and release wall time is at most 1.77 s per invocation. Behaviour on a 16 GB machine was not observed.
- **Not combinations (B2), pressure (B3), S-I2, F2b or F3.**
- **No kernel, schema, `Cargo.lock` or reviewed-input change.**

## Source and packaging

- **33 files under `projects/chirality-piping`** (3 added, 30 modified; +329,970 / −947, of which +283,214 is the corpus). They are byte-identical to the integration branch at `31eed8497f`; `source_equality.py` checks this.
  - `core/product_physics/`: admission, the n-case transaction, the wire, and their tests;
  - the Rust, Python and TypeScript retained readers and their tests;
  - `fixtures/results/`: corpus 07n, W-C2's two successors, and the carrier scope sentence.
- **This branch is cut from main `953d8c9446`** as one commit, `7f5f72912e`. B1's history stays on `codex/piping-t3-b1-20261007` and the integration branch.
- **Records committed:** `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/B1/`, holding the change record, this body, the citation index, `SHA256SUMS`, and copies of QUAL_B1, RSS_TIME and `registration.diff`. Source citations resolve through `citations.json` to commit-pinned URLs.

## Reviews

These are agent reviews, not personal review by the owner.

- **RV109 (producer):** PASS on each round, with its should-fixes repaired and confirmed.
- **RV112 and RV124 (admission and qualification):** PASS. RV124 confirmed M.
- **RV113 and RV120 (the readers and 07n):** PASS, with each repair round confirmed.
- **RV125 (the complete-diff review of this PR):** PASS, with 0 blocking, 0 should-fix and 7 notes. Its two should-fixes on the first cut (a misquoted ruling title in a comment, and the commit message's reviewer list) were repaired by the recut.
- **Pass B:** I107, with no stop; RV124 confirmed it.
- **RV125:** the complete-diff review of this PR (the merge record).

Details: `…/IMPLEMENTATION/B1/CHANGE_RECORD.md` §4.

## Gates

| Gate | Result |
|---|---|
| The full 40-manifest suite and the src-tauri suite, before the freeze | 0 changed outcomes; src-tauri 116 = 116 |
| The Direct-entry gates | Pass: pressure refused, coexistence holds, Stale byte-identical, no product caller |
| `source_equality.py` | Checks 1–3 and 5 pass on the code commit; check 4 needs this head |
| `check_citations.py` | Pass on the code commit: 98 resolved, 0 ambiguous, 0 unresolved |
| Pass B | Pass, with no stop; RV124 confirmed it |
| Hosted CI with the full-SHA dispatch, GEN-8, and the exact-head DEC-025 with the src-tauri suite | Run on this head before the merge, and recorded on the integration branch |

**Still open,** and routed: B3's pricing on B1's profile (SQ2), the DEF-O projection note (B2), and the combination and pressure rules of the readers (B2, B3). See CHANGE_RECORD §7.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
