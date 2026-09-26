# Checkpoint 5 — T1 wave 1: joined activation, typed operations, harness adapter and VP-STATIC package

- **Author:** the session-3 T1 WORKING_ITEMS manager, parent HELP_HUMAN (ROOT).
- **Branch:** `codex/piping-load-states-20260925`, from the CP4 head `500d88644`.
- **Paths:** WORKING_ROOT-relative. Machine records are in `_run_records/session4/`.
- **Status:** completed execution, not acceptance. No M10, M16 or M29 finding closes at this checkpoint.

## Authority

- **Owner and ROOT decisions D1–D4** (`OWNER_T1_DECISIONS_2026-09-26.md`):
  - D1: activate `load-reference-source-1` once its readers pass independent review; the owner approved the packager edit.
  - D2: plain input fields.
  - D3: no backwards compatibility; 0.4.0 is for new models.
  - D4: write scope. Granted now: WP1, the WP3 operations, WP5 and WP6. After T0R: WP2, the WP3 native fields and WP4.
- **Manager and ROOT rulings made during execution:** `T1_WAVE1_RULINGS.md` §1–10.
- **The wire of this wave:** `T1_WIRE_ADDENDUM.md`.

## What landed

| Commit | Work | Delegated to |
|---|---|---|
| `5ced47dec`, `bfb0fe3ce` | **WP3.** Typed authoring operations for `Model.reference_configurations`, `Material.expansion_laws` and `Load.analysis_state`, parsed with the product's public closed DTOs, on 0.4.0 models only. Inbound-reference refusals for replacements and deletes. `pressure_profile` is locked on 0.4.0. `temperature_points` may not orphan an `exact_point` `point_ref`. Pre-0.4 and 0.3.0 outcomes are byte-identical to goldens blessed from the base. | TASK T1_WP3_OPERATIONS |
| `bfef71b19`, `1ccca8b87` | **WP1 (D1).** Rust and Python readers for `load-reference-source-1`. Additive joined branches in the three 0.3 carrier schemas. The owner-approved `package_v0_3.py` edit (the host allowed it). 10 canonical documents, 10 AnalysisRun records and 10 stress-neutral packages from the committed joined raws. One declared standing edit (early `needs_recompute`). One D1-caused expectation change. | TASKs T1_WP1_JOINED_READERS and T1_WP1_JOINED_SCHEMAS; the packager edit and the stress-neutral outputs by the manager |
| `31dc7ce08`, `ab5919133` | **WP5.** A closed, manifest-driven VP-HARNESS adapter for the `load-reference-1` transport. Runner output is admitted up to the selected limit. The reader is re-pinned to the integrated bytes (`14e1750e…`). | TASK T1_WP5_HARNESS_ADAPTER |
| `91ec30630`, `6824b6b6b`, `c1e130818`, `cad59d01e` | **WP6.** 14 VP-STATIC cases over 12 independent analytical references (support motion, reference temperatures, cold spring), with 467 positive and 40 negative assertions. They passed an independent freeze (FREEZE; changes R1–R3 applied) and are admitted through `ADMISSION.json`. Records are at the admitted state. | TASKs T1_WP6_STATIC_CASES and T1_WP6_FREEZE; admission by the manager |

## Independent reviews

- **T1_WAVE1_REVIEW_A** (joined readers, schemas, packager): **CLEAR**, so the D1 gate is met. Its three notes were repaired in `1ccca8b87`. The backcheck is pending.
- **T1_WAVE1_REVIEW_B** (operations, adapter, admission): FINDINGS, with three Medium and six Low/Info, none blocking. All are dispositioned (`T1_WAVE1_RULINGS.md` §10) and repaired in `bfb0fe3ce`, `ab5919133` and `cad59d01e`. The backcheck is pending.

## Checks (targeted; the latest run of each)

| Check | Result |
|---|---|
| result_export | 69 |
| runner/headless | 64 |
| operation_applier | 193 |
| Python suites (joined and LR readers and schemas, results and AnalysisRun schemas, three stress-neutral suites, source-block schema contract, physics consumer and physics-source contract) | 1521 passed, 16 skipped, at `bfef71b19`. The WP1 follow-up author reports 1182 passed, 1 skipped on its seven files |
| Harness | adapter 55 OK, gate 31 OK, physics 42 OK |
| WP6 package | generator `--check` 28/0; admitted-state checker 4535/0; independent recompute 3154/0 |
| Mutants | WP1 50/50 plus the N-2 follow-up; schemas 25 killed, 1 equivalent (recorded); WP3 42/42; WP5 62 before the repair, and after the repair its own mutants plus REVIEW_B's (see its return); WP6 24/24; freeze checker 9/9 |

**Not yet run:** the recorded VP-STATIC comparisons (below), the full piping pytest sweep, the DEC-025 sweep, desktop, browser, native and hosted CI.

## VP-STATIC comparisons

The recorded development comparison runs after:

- REVIEW_A's backcheck of the reader bytes;
- ROOT's release of the host hold for T0R's timing-sensitive sweep;
- an incremental rebuild of the runner at the candidate head.

**Pre-review shakedown** (scratch only, not evidence; `T1_WAVE1_RULINGS.md` §9): 12 of the 14 cases solved with `checks_passed` standing, and all 441 of their assertions matched in both modes. The other two exposed the adapter output-limit defect, since repaired.

## Boundaries and open work

- **Joined results are `needs_recompute`.** They are never numerically eligible in T1. Invocation-based eligibility is open work for T3. M10, M16 and M29 are qualified on the ordinary `load-reference-1` route. Sensitive cases are withheld from Current on every route (M03).
- **After T0R lands on main:**
  - WP2: desktop types, native persistence and migration;
  - WP3 native fields: plain inputs and the read-only resolved-state block;
  - WP4: headless and CLI tests, including the joined path.
- **Native witnesses on the owner's Mac** (`T1_PLAN.md` §6) remain outstanding, and M10/M16/M29 cannot close without them. REVIEW_B added one item to native witness 8: the post-reservation fallback input.
- **Recorded limits, not repaired:** the inherited transport scope of joined validation; the schema cross-carrier `$ref` home; the operation-level inverse and explicit-null priors (documented).
- The T1 PR qualification (`T1_PLAN.md` WP7) is still to come.
