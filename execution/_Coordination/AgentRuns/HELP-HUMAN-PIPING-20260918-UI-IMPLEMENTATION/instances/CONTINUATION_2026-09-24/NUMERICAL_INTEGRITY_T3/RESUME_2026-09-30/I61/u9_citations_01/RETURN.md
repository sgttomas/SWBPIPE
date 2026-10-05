# I61 RETURN: U9 citations, stale code line pointers replaced by symbol names (comment-only)

**What changed:**
- **93 bare code-line citations reworded.** Every such citation in shipped comments (`PP:2896-2907`, `FC:358-379`, `FK/adaptive.rs:4349-4378`, `verify.rs:880` and the like) now names the cited function, method, type or constant instead of a line.
- **91 lines in 8 files edited**, each one line for one line. Every file keeps its line count.
- **14 citations kept pinned** in the package's `citations.json`, with anchor text verified at a pinned revision:
  - 11 in `retained_memory.rs`'s GENERATED PROFILE block;
  - 3 in the hash-pinned JSON corpus.
- **`COMP:66`** resolves to I51's COMPOSITION, per ROOT's ruling.

**Checks:**
- The extended `check_citations.py` passes on the reworded tree: 368 resolved, 0 ambiguous, 0 unresolved.
- At `e543c3d8f3` itself it fails, with exactly the 93.
- No Pass B rule key falls on an edited D1 production line.
- The suites match their pre-edit counts (§4).

**Status:** the work is uncommitted in WT/f2a-u7, on top of `e543c3d8f3`. The package `R/I61/u9_package_01/` is updated and resealed.

**Who and how:** I61 (TASK, Type 2), dispatched directly by ROOT, 2026-10-04 from about 21:57Z.
- No Git or index writes. Git reads used `GIT_OPTIONAL_LOCKS=0` (`diff`, `show`, `blame`, `ls-tree`, `cat-file`).
- The memory guard (PID 5387) was running.
- One Cargo job at a time, `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`, on my own targets.
- No machine paths in these records.

## 1. Scope: the 38 ROOT named, and 55 more of the same class

ROOT named 38 prefixed citations (`PP:`, `FC:`, `FK/…:`). The same class also appears without a prefix: `adaptive.rs:4545`, `verify.rs:880`, `source_receipt.rs:546` and the like. ROOT also asked the check to fail on any bare `FILE:line` code citation, so it covers both forms.

| Group | Count | Handling |
|---|---|---|
| In comments and docstrings | 93 (36 prefixed, 57 without a prefix) | Reworded to symbols (§2) |
| Generated, in `retained_memory.rs` lines 1051–2300 | 11 | **Kept and pinned** |
| Hash-pinned JSON data, in `fixtures/results/retained_precision_cases.json` | 3 | **Kept and pinned** |

**Why the generated citations are kept.** That block is written by `g5_profile.py` ("do not edit by hand"), and the package's copied generator must keep regenerating it byte for byte. Each of the 11 is recorded in `code_anchors` with its revision and anchor text: the cited struct or field at the profile basis (`2bb81ec1ea`, `cba3e9fda7`). Rewording them means changing the generator. That is ROOT's call; it is not done here.

**Why the JSON citations are kept.** They are data fields, not comments, and the corpus is snapshot-pinned (07k). They are pinned at their blame revisions (`cfda60403f`, `cc4dd61d67`); each anchor names the cited test or function line.

**Two bare line numbers found next to citations were also fixed:**
- `retained_memory.rs:2572` had `:4543`; it now reads "`ordinary_report` in lib.rs".
- `retained_precision.py:1197` had "assigned at 1195"; it now reads "assigned once every body completes".

## 2. Method: how each symbol was chosen

1. **The revision the line referred to.**
   - **Reader and test files** (the Python, Rust and TS readers and their tests): the producer CODE head the readers were audited against, `fc23cff95f`. This is the typed-trace producer that RV74 accepted, and its line numbers in `retained_product.rs` are the same as the coverage-seam head `c618675e84`'s.
   - **The check that chose it:** at `fc23cff95f`, `retained_product.rs:2896` is `fn enter`, `:2310` is `fn check`, `:2442` is the nonpositive-input check, `:2360` is `CoefficientRange` and `:1238` is the member id. Those are exactly what the comments describe.
   - **The blame revision was rejected.** For the readers, `git blame` gives the reader-branch commit, whose older CODE copy put `:2896` in the middle of another function.
   - **U4 files** (`retained_memory.rs`, `retained_memory_law_tests.rs`): their blame revision, G6 `2bb81ec1ea`.
   - **`retained_receipt.rs`:** `fc23cff95f` too, where its `:111–113` are `project`'s stage-consistency checks (checklist P2, P6, P11).
2. **The symbol.** Resolve the cited file at that revision. For each cited line or range, name the enclosing definition; or, when the range starts on a definition boundary, the definition it starts. A range spanning several definitions is written as "`first` through `last`".
3. **Checks on the result.**
   - Every named symbol still exists in the cited file at `e543c3d8f3` ("missing 0").
   - **Three were adjusted by hand,** each marked in `EDITS.json`:
     - `retained_precision.py:312`: the range ran into the tests module, so the K4SRC/K4STF encoders are named;
     - `retained_precision.rs:2323`: the bare second range "3460-3555" is now named as `PreparedCase::project_candidate`;
     - the two bare line numbers in §1.

**The rewording form.** `PP:2896-2907` becomes ``retained_product.rs `AdapterWork::enter` ``, and `FK/adaptive.rs:4349-4378` becomes ``adaptive.rs `terminal` ``. The surrounding sentence is unchanged.

**Every before/after pair**, with its citation, the symbols and the revision used, is in `EDITS.json`. The full diff is `_run_records/candidate.diff`: 8 files, +91/−91.

## 3. Comment-only and line-neutral

- **Line counts are unchanged** in all 8 files: `git diff --numstat` shows insertions equal to deletions per file, and every replacement is one line for one line.
- **Every edited line is a comment or inside one:**
  - `//`, `///`, `/* */` and JSDoc lines in Rust and TS;
  - in Python, `#` comments and docstrings. For the one line with code (`retained_precision.py:1446`), the change starts after its `#`. The 10 docstring continuation lines were confirmed inside their triple-quoted docstrings.
- **Pass B rule keys:**
  - Method: every key naming `retained_memory.rs` or result_export's `retained_precision.rs` in I65's `u4_g7_04` rule files, mapped from Pass A's basis (`ba1faa1c…`) to `e543c3d8f3`.
  - **No key falls on, or within 2 lines of, an edited line.** That is 2 keys against `retained_memory.rs`'s 2 edited lines, and 25 keys against `retained_precision.rs`'s 22 edited lines. The premise pins at `:4252`, `:4253` and `:4305` are not near any edit.
- **One note for I65's full Pass B:** `retained_memory_law_tests.rs:1238` is a qualification-test file. Its single comment-only hunk needs a reviewed entry, per RV89 N-5, or Pass B exits 6 as designed.

## 4. Suites, on WT/f2a-u7 with the edits

| Suite | Result | Before (RR, e543c3d8f3) |
|---|---|---|
| Python retained (3 files) | **463 passed** | 463 |
| result_export (all targets, `--no-fail-fast`) | **171 passed, 0 failed** | 171 |
| PP `cargo test --lib --no-run` and `--tests --no-run` | **both exit 0** | compiles |
| vitest (whole desktop suite) | **138 files, 3,552 passed** | 3,552 |
| tsc `--noEmit` | **exit 0, no output** | clean |

**Every count equals its pre-edit value**, as expected for a comment-only change. The suites ran from 22:05Z to 22:13:01Z (`_run_records/suites_summary.txt`). Full logs are in WT/scratch/i61_u9_citations_01/logs; the runner is `_run_records/run_suites.sh`. After the runs, `git status` in WT/f2a-u7 lists only the 8 edited files, plus the untracked `node_modules` link.

## 5. The check, extended (package `check_citations.py`)

- **The new class `code_line`** fails on any bare `FILE:line` code citation in the source set (prefixed or not) unless the index's `code_anchors` pins it. A pinned entry records the revision and the anchor text of each cited line, and the check compares that text at that revision.
- **`--head WORKTREE`** checks an uncommitted checkout against the base.
- **Results:**
  - Reworded tree: resolved 368 (65 record/RR, 289 document, 14 pinned code lines), ambiguous 0, unresolved 0. PASS.
  - At `e543c3d8f3` itself: unresolved 93, the same 93 reworded here. FAIL. This is the natural negative control.
- **`COMP`'s rule** records ROOT's ruling: COMP in PP's U3 facade code means I51's COMPOSITION, because the comment came in with U3 grant 1b (`4b31bbf23a`).

## 6. Files

- `EDITS.json`: 91 lines; before, after, citations, symbols, revision and hand notes.
- `_run_records/candidate.diff`, `rewrite.py` (the planner), `run_suites.sh`, `suites_summary.txt`.
- `SHA256SUMS`.
