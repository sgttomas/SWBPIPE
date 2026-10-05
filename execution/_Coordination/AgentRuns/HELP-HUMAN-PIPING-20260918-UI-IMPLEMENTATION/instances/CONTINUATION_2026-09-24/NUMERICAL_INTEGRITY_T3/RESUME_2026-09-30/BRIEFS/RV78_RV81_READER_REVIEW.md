# RV78–RV81: independent review of the three retained-precision readers and their shared corpus

Four TASKs (Type 2), independent reviewers dispatched directly by ROOT (HELP_HUMAN, Agent 0) as background subagents. ROOT is the return path. None delegates. **None of you wrote any of this code or corpus, and none relies on the authors' tests as oracles.**

| Reviewer | Reviews |
|---|---|
| RV78 | The shared contract artefacts and corpus; the joint parity run |
| RV79 | The Python reader |
| RV80 | The Rust reader |
| RV81 | The TypeScript reader |

## Why there are four

The three readers were built to match one shared corpus. If the corpus expects the wrong gate or code, or encodes a receipt the producer could never emit, all three readers can agree and all be wrong. So RV78 checks the corpus against the contract and the native code, independently of the readers. RV79–RV81 each check one reader's complete source against the contract, not merely against the corpus.

## The candidate

- **READER** = `WT/f2a-readers`, branch `codex/piping-f2a-readers-20261003`. Its exact head is given in your dispatch prompt.
- **Everything there is unaccepted WIP:** the readers, the receipt schema, the corpus and the tests. The authors are I62 (shared corpus and Python), I63 (Rust) and I64 (TypeScript); their RETURNs are under `R/I62/`, `R/I63/` and `R/I64/`.
- **The shared snapshot under review is 06d** (`R/I62/coverage_shared_python_01/SHARED_SNAPSHOT_06D.json`): 15 cases, 178 mutations and 18 must-pass entries.

## The basis (read what your role needs)

- **The selected contract:**
  - C1 `R/I32/f2a_wire_c1/WIRE_CONTRACT.md`;
  - C2 `R/I32/f2a_wire_c2/CONTRACT_DELTA.md`;
  - C3 `R/I52/prepared_public_contract_02/C3_DELTA.md` with `DEFINITION.json`;
  - F1 `R/I52/prepared_public_contract_correction_03/ADDENDUM.md`;
  - the seam addenda `R/I52/reader_contract_seams_06`, `_correction_07` and `_g4_08`.
- **The coverage completion:** `R/I57/summary_coverage_01/ADDENDUM.md`.
- **The G5 checklist:** `R/I62/coverage_shared_python_01/READER_AUDIT_PLAN.md` Part 1.
- **ROOT's rulings** in `T3/ROOT_RULINGS_V1.md`, from "Resumption by the next ROOT; coverage implementation planned" to the end. They settle many contract readings, with citations. Where you believe a ruling misreads the contract or the native code, say so with the citation; that is a finding, not a defect in your review.
- **Native code** for producer facts: `P/core/solver/frame_kernel/src/structural/retained/` and `P/core/product_physics/src/`, at NUM's head.

## RV78: the shared artefacts, the corpus and parity

1. **The schema** (`P/schemas/retained_precision_mp_v2.schema.json`) against C1, C3 and I57: closed shapes, required members, `summary_coverage`, and the WorkAccounting tension that ROOT recorded.
2. **The corpus.** Sample at least 40 mutations and every must-pass entry, weighted toward the 06a–06d additions. For each, establish independently:
   - **the expected first gate and code,** from the contract's gate order and within-gate order (C1 §6, C3:294–304);
   - **native fidelity:** is the base, or the edited receipt, something the producer could emit, or a correct rejection of something it couldn't? Check the synthetic labels and the storage-only trigger rule;
   - **rehash integrity.**
   
   Check also that the 15 cases validate under an independent reading.
3. **The joint parity run.** Run all three readers' test commands on the same READER head, and tabulate per-entry outcomes from each reader's outcome files or test output. Report any entry where the readers disagree, and any where all three agree but you believe the expectation is wrong.
4. **Coverage of the checklist:** which of the 42 IDs is pinned by a shared entry, and which only by reader-local tests or deferred. Is any important gap missing a shared pin?

## RV79, RV80, RV81: one reader each, complete source

Review your reader's complete retained-precision source (not only recent deltas) against the contract basis, the I57 coverage rules and the rulings:
1. **Gate order and first-failure codes** for G0 to G8, including within-gate precedence, gate-major order across cases, and per-reader G7 base codes, with detail kept separate.
2. **The G5 checklist:** every ID, against the reader's actual code. Confirm or dispute the author's status table.
3. **Arithmetic.** Check the exact and rational helpers, upward rounding, `phi_512`/`e_hat`, the extent, and the checked counters, using your own independent oracle (exact rationals), not the reader's own functions.
4. **Coverage (I57):** the feasibility rule, the estimate/charge rederivation, rosters, data facts, and unavailable attempts.
5. **Fail-closed behaviour:** eligibility must be impossible while the completeness flag is false. Look for any path that reports success without running every gate. Look for public-API exposure (Python's API should stay disabled).
6. **Mutation testing:** apply at least six single-edit mutants of your own to the reader, each from a clean copy, and report whether the existing tests kill each one.

## All reviewers: host, method and output

- **Your own copy:** build or run from a `git archive` of the READER head into `WT/rv<NN>/`, with your own targets for Cargo (`WT/targets/rv<NN>`). Delete the copy afterwards; keep the logs in `WT/scratch/rv<NN>_reader_review/`.
- **Cargo:** `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`. Set `DEVELOPER_DIR=/Library/Developer/CommandLineTools` in your cargo environment (the Xcode licence is unaccepted) and disclose it.
- **Python:** `VENV/bin/python -m pytest` with `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` set, as the I62 brief shows.
- **TypeScript:** the existing `node_modules` link, with no install.
- **The memory guard** must be running.
- **Never:** Git writes, index operations, new tooling, or native, solver or DEC-025 jobs.
- **The report:** `NUM/R/REVIEW_RV<NN>/reader_review_01/REVIEW.md`, containing:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path:line, evidence and remedy;
  - a short section per review item.
  
  Add your evidence files and a SHA256SUMS. Use placeholder paths (`WT`, `P`, `VENV`) in committed text.
- **Time box:** 2 hours from your first tool call. Report anything unfinished, rather than silently narrowing.
- **End your turn** with a concise status: the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
