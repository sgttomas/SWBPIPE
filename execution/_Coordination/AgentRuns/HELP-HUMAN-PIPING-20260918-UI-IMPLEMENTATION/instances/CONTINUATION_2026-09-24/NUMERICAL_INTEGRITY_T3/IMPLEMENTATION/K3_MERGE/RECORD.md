# K3 merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1041.
  - ROOT (HELP_HUMAN) merged it on 2026-09-28 as `57617b0fbfa6e59a48f329320cce52849602b833`: a merge commit with `--match-head-commit 2511f5a3c`, under the owner's standing Git authorization.
  - Main was `98b1723b1`, an ancestor of the head, and the merge state was clean.
- **Candidate head:** `2511f5a3c73fd95f1c27e28140224bd708058d84`, on branch `codex/piping-k3-20260928`.
- **Where it ran:** the owner's Mac (`aarch64-apple-darwin`, rustc 1.97.1), all at opt-level 0. ROOT dispatched the implementer (I11) and the reviewer (RV12) directly. A drafting TASK wrote the brief, and ROOT reviewed it and ruled on its questions.
- **Scope:** the rest of W1's arithmetic. The work sits in `retained/wide/multi.rs`: `Wide<L>` for L = 4, 8 and 16; the conversion to binary64 with explicit outcomes; and K4's arithmetic (widening and narrowing, TwoSum and TwoProduct, an exact-integer constructor, and per-width work counts).
  - It is kernel only, with no product caller.
  - K3a's `Wide<2>`, which K-D5 publishes through, is byte-identical.

## The chain (base main `6e18505e3`; its piping tree equals `eb52114e9`'s [outside `execution/`, that is in `core`, `fixtures`, `validation` and `schemas` (RV13-N2)])

| Commit | Content |
|---|---|
| `74add6078` | Checkpoint A: `multi.rs`, the `wide.rs` edits, and 42 tests with vectors |
| `8cacbfaf4` | `[profile.test] opt-level = 1` in `FK/Cargo.toml` (ROOT's Q7), **later withdrawn** |
| `9aee9854c` | The profile guard test (overflow checks and debug assertions on) |
| `664ef5c5e` | Records |
| `de719cbdc` | A merge of main `98b1723b1` (the skew M03 pin) |
| `e83e22356` | **The profile withdrawn.** `FK/Cargo.toml` is back to main's bytes (ROOT reversed Q7, `ffc9ea275`) |
| `b7e93650e` | Records addendum 1 (Q7 reversed) |
| `e62837f7e` | RV12's test gaps: K4-API value assertions and a tail-decides-tie class (tests only) |
| `2511f5a3c` | Records addendum 2 (RV12's findings) |

## Gates

- **Independent review, RV12** (`REVIEW/K3_REVIEW.md`; numerics `b477547f7`, then `e63325363`):
  - PASS at `b7e93650e`: 0 BLOCKING, 2 SHOULD-FIX, 7 NOTE.
  - PASS on the delta check at `2511f5a3c`: S2 and N1 resolved, and S1 stated. There is one optional NOTE, D1: two stale "43" counts in CHANGE_RECORD, lines 76 and 90, which should read 45. It is corrected here, not in the hash-bound file.
  - RV12's own oracle, independent of I11's generator, found 0 mismatches in 1,023,720 operations and 379,062 conversions.
- **Hosted CI:**
  - pull_request run **36413698754** on `2511f5a3c`: success. Its "Numerical cargo suite" took **about 11.9 minutes** (11:07:15Z–11:19:09Z) against its 45-minute budget. This is the figure the Q7 ruling asked for.
  - pull_request run **36404664521** and the full-SHA dispatch **36404663005** (target_base `98b1723b1…`) on `b7e93650e`: success. They stand for the head, since `e62837f7e` and `2511f5a3c` change only tests and records.
  - The final PR state: 12 checks passing, 0 failing.
- **T9 (Mac-only):** 112 of 112 byte-identical, base against candidate (I11's RETURN). RV12 checked the final head's outputs against ROOT's Mac main hashes: 112 of 112. [RV12's T9 ran at `b7e93650e`; the final head changes only FK tests (RV13-N2).]
- **DEC-025,** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28), on `b7e93650e`; the evidence is in `dec025/`:
  1. **The sweep invocation** failed at the cargo surface on product_physics's platform test `t13`, and the later surfaces were recorded as `not_run` (fail-fast). The summary is `SWEEP_20260928T093842Z_b7e93650ea11.json`, sanitized; the sha256 of the original is `b3eeb3515a482ee4a3ad1fbe13124f27c32b87db7056a0c31894683dc329a42d`.
  2. **All 39 manifests** were run with `--no-fail-fast`. Against the Mac baseline of main `98b1723b1`, only frame_kernel changes (184 → 227, K3's tests), and the failing tests are exactly the three Mac platform tests (`suites_vs_baseline.txt`).
  3. **Surfaces 2, 3 and 5** (`surfaces.txt`):
     - pytest: 3023 passed, 32 skipped;
     - the production build passed;
     - **desktop vitest failed one test under host load:** `App.deadControls.test.tsx`, "Next result page … produced no observable DOM change", with a load average above 8 while two reviewers and pytest ran. K3 changes no TypeScript and no desktop input.
  - **The vitest surface was re-run on a quiet host at the head `2511f5a3c`** (`dec025/vitest_rerun/`, load average about 3 at the start) [and 11.91 at the end, 11:22:00Z, per its `meta.txt`. The head, `2511f5a3c`, is established by the sweep worktree's reflog, not by `meta.txt`. The earlier "above 8" loads are not in committed evidence (RV13-N3)]: `build:wasm:desktop` passed, and `test:desktop` gave 134 files and 2822 of 2822 tests. No timeout was raised and no test was skipped.
  - The evidence at `b7e93650e` stands for `2511f5a3c`, which changes only FK tests and records. FK's suite at the head is 229 passed (I11 and RV12, independently).
  - Machine paths are sanitized to `<WORKTREE>`, `<VENV>`, `<wt>`, `<home>` and `<tmp>`.
- **The gate:** not run. There is no product caller (the K3 rulings).

## Findings and routed items

- **Q7 (ROOT's error, recorded):** the test profile rested on a false premise, that opt-level cannot change results. Constant-folded `powi` changed the skew pin's figure. The profile was withdrawn, and the lesson is on the T3-close list: tests must not depend on compile-time evaluation of functions of unspecified precision.
- **For K4's brief** (RETURN addendum 2; RV12's S1 and N2–N5):
  - the correctly rounded exact multi-term sum primitive;
  - `AttemptWork` double counting via `Clone`;
  - `from_integer`'s flat cost;
  - netting `ExactAccumulator`'s split magnitudes;
  - 10^6-operation streams at K4's working precisions (128, 192, 320, 576);
  - the Q8 ceiling question (a p + 64 residual beyond 1024 bits).
- **The M17 note:** K-D5's tests do not detect a tie-rule change at 2^-128. K3a's path is guarded by K3a's suite and K3's L = 2 cross-check.
- **Dead-code labels** name real consumers. The "K3a API, no caller yet" items are reviewed at T3 close.
