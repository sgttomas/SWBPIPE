# I62, I63 and I64: repairing the readers after review RV78–RV81 (snapshot 07)

Three TASKs (Type 2) share this brief. ROOT (HELP_HUMAN, Agent 0) resumes each one directly, with its existing context. ROOT is the return path, and none delegates.

| Author | Owns |
|---|---|
| I62 | The shared corpus and schema; the Python reader |
| I63 | The Rust reader |
| I64 | The TypeScript reader |

## The basis

- **The decisions:** in `T3/ROOT_RULINGS_V1.md`, the ruling "Reader review RV78–RV81: consolidated ruling and the repair wave", D1–D15. Each item below refers to them.
- **The reviews:** `R/REVIEW_RV78/`, `R/REVIEW_RV79/`, `R/REVIEW_RV80/` and `R/REVIEW_RV81/`, each in `reader_review_01/`.
- **The probes:** RV78's `PROBES.json` holds exact edits and expectations for most of the B1, S1 and S2 relations. Build your reader-local tests and the shared mutations from them, checking each against the decision. Neither the probes nor the reviews are oracles.
- **The candidate:** READER at `6b607fd01f`. All readers pass snapshot 06d there.

## The order

1. **I62, checkpoint A (45 minutes).** I62 writes `R/I62/review_repair_07/CHECKPOINT_A.md` with the native and contract facts below, then returns. It makes no change in READER.
2. **I63 and I64, phase 1 (90 minutes, in parallel with step 1).** Each repairs its reader for every decision that does not wait on checkpoint A, and adds reader-local tests. Neither touches the shared files.
3. **ROOT** rules on checkpoint A and commits phase 1.
4. **I62, phase B (2 hours).** I62 builds snapshot 07 and repairs Python.
5. **I63 and I64, phase 2.** Each adopts snapshot 07 and the decisions ruled at step 3.
6. **The four reviewers** confirm their findings on the repaired head.

## I62, checkpoint A: facts for ROOT to rule on

For each item, give the native or contract citation, and a proposed rule with its gate and code.
- **D1, empty inventory:** can a CaseSource member inventory, or a complete old list, be empty in an emittable receipt?
- **D6a, ordinary diagnostic list:** does the producer's ordinary `diagnostic_refs` contain only diagnostics whose `affected_refs` include the case? Cite the projection and routing code.
- **D6b, routing statuses:** which ordinary `solve_quality` statuses route a case to retained precision under I30? In particular, does `not_assessed`?
- **D8, the accounting class:**
  - list every schema shape carrying an accounting event, a fault or a lost flag, with its native emission condition;
  - propose R1′–R4, starting from RV78-S2;
  - say whether R3 can bind to the owning trace;
  - list the corpus entries each rule would move, including `prefix_attached_old_input_unbound` and the rebase it needs.
- **D9a, Refusal variants:** is there a contract or native source for the Refusal variants `work_accounting` and `count_range`?
- **D9b, "no source":** which encoding does the producer's `retained_receipt` projection emit for "no source" on an unavailable case?
- **D3, the native check table:** every native G5 check in the three readers, with its ATTEMPT or WORK code. Flag any check where the readers differ, and any 06d entry whose expected code would change under the D3 convention.
- **D12, the checklist:** correct N11, recount the IDs, and update the statuses of the 13 IDs RV79 disputes.

## Phase 1 (I63 and I64): decisions that need no checkpoint

Implement the decisions below against your reader, with a reader-local test for each relation, built from RV78's probe edits where one exists. Where your reader already complies, confirm it with a test, not a claim.

- **D1:**
  - member ids are `0..len−1`, with prepared and new as prefixes;
  - sourced complete old coverage equals the source's member map;
  - unsourced complete old coverage is non-empty and equals any CaseSource's member count;
  - the `captured_prefix` split: members at G3; source, run and result at G5 PRODUCT_ATTEMPT;
  - Run origin checks belong in G5 class 1.
- **D2:** the G0 union. Shape defects outside it wait for G1.
- **D3:**
  - class order;
  - the class-1 convention: native WORK predicates are deferred to the end of class 1.
- **D4 a–e.**
- **D5 a–e.**
- **D6c and D6d.**
- **D6b's `checks_passed` exclusion.** The `not_assessed` part waits for checkpoint A.
- **D7.**
- **D13** pins, and tests that kill your review's surviving mutants where the rule is implemented.
- **D14 and D15.**

**I63 (Rust):** RV80's findings and RV78 B1/S1 rows R-6a, T-1, T-2 and T-4e. Make `OUTCOMES` list all 178 mutations.

**I64 (TypeScript):** RV81's B1, B2, S1 and S2, its N2 (code ATTEMPT), and the D3 convention.

**Stop and report if:**
- any 06d shared entry's outcome changes;
- a decision conflicts with native code you can cite;
- you need a path outside your fence.

The 06d corpus must still pass in full at the end of phase 1.

## Phase B (I62): snapshot 07 and Python

ROOT grants this after ruling on checkpoint A and committing phase 1.
- **Shared files:** D9 and D11, plus the D8 rules and rebase as ruled.
- **Python:** D1–D7, D10 and D13. Snapshot files and SHA256SUMS follow the 06d pattern.

## Write fences (READER only)

| Author | May write |
|---|---|
| I62 | `P/schemas/retained_precision_mp_v2.schema.json`, `P/fixtures/results/retained_precision_cases.json`, `P/core/analysis_runs/retained_precision.py`, `P/tests/test_retained_precision_contract.py`, `P/tests/test_retained_precision_schema.py` |
| I63 | `P/core/reporting/result_export/src/retained_precision.rs`, `P/core/reporting/result_export/tests/retained_precision_contract.rs`, and `src/lib.rs` for wiring only |
| I64 | `P/apps/desktop/src/features/results/retainedPrecision.ts` and `.test.ts` |

## Commands

As in the 06d grants, with one change: **Cargo now uses the default toolchain.** Do not set `DEVELOPER_DIR`; the Xcode licence is accepted.

## Host and limits

- The memory guard must be running.
- **Never:** Git writes or index operations (Git reads only, with `GIT_OPTIONAL_LOCKS=0`), installs, new tooling, or native, solver or DEC-025 jobs.
- `IMPLEMENTATION_COMPLETE` and `SUMMARY_COVERAGE_COMPLETE` stay false.

## Output

**Evidence** goes in `NUM/R/I6x/review_repair_07/`: RETURN.md, outcome files and SHA256SUMS, with placeholder paths only. Bulk goes in `WT/scratch/i6x_review_repair_07/`.

**RETURN.md** holds:
- the changed files, with sha256;
- each decision, with its status (implemented, already compliant, or waiting) and the test that pins it;
- the commands, with results;
- every remaining known difference from the other readers, or "none".

**End your turn** with a concise status for ROOT.
