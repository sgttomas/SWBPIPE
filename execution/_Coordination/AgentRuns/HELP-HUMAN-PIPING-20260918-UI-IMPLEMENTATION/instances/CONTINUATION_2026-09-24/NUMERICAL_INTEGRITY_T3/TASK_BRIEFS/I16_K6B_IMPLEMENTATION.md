# I16: implement slice K6b (W1 observations: limbs, work, memory per precision and seconds per work unit, on the K6 harness)

> This is an implementation TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host" and "Platform calibration", `I8R_K1_RESUME.md:24-50`) override `_COMMON.md`'s host section, and apply to you in full, with K6b's paths below in place of K1's.

## Roles

- ROOT (HELP_HUMAN) dispatches you directly, as a background subagent, and is your return path. There is no separate T3 manager on the Mac.
- Make no Git writes and no index operations. ROOT commits.
- Record the delegation mechanism in RETURN.

## Paths and bases used in this brief

- `P/`, `T3/`, `H`, `FK`, `SD`, `NI`, `SA` and `PP` are as in `I15_K6_IMPLEMENTATION.md` ("Paths and bases"). `K4R` is `FK/structural/retained/`.
- **Code is cited at K4's candidate head `7d8fa9c0e`** (PR #1054), whose piping tree is main `7ac7b1c37` (K6) plus K4. ROOT records the actual base at spawn: main after K4 merges. Re-locate every line on your base.
- **T3 records are cited at the numerics head,** read in `<wt>/numerics`.

## Purpose

K6 measured the binary64 kernel path and left W1 to this slice (`I15_K6_IMPLEMENTATION.md` Q1(a); K6 `RETURN.md` §8.7, §13.1 "How K6b adds a W1 mode"). ROOT needs W1's figures to set the per-case and per-invocation limits before F2a merges ("K4: Q5 amended", `ROOT_RULINGS_V1.md`; K6 `RETURN.md` §13.5). K6b supplies, on RF-LARGE, observations only:

1. **limbs per entry** by precision, and the storage counts K4's evidence reports;
2. **work units** (K4's limb-multiply equivalents) by stage and precision, including the verification pass and revision 5a.3's shift (R7 §6.7: "their cost on large models is for K6b and V-K to measure");
3. **peak memory per precision** (Q1 below);
4. **seconds per work unit,** by interleaved runs (A, B, A, B; `_COMMON.md:32`);
5. **the export** of K4's `retained` API from FK, which this slice, as the first consumer outside FK, adds (`ROOT_RULINGS_V1.md`, "Design text made stale by K1, K2b and K3" item 9; K4 `RETURN.md` §16 "The export list").

### What K6b does not do

- It proposes no limit, threshold or ceiling. ROOT sets the limits from K6, K6b, V-K and K4's work counts.
- It does not change K4's method, any published row, or any product path. PP, SA, NI, SD and the fixtures are untouched.
- It runs no product-level measurement (V-P) and nothing on Linux.
- It does not run V-K's reference comparisons (the R1 families beyond RF-LARGE, the zero-scale floor check, the seeded faults).

## Scope

### 1. The export (checkpoint A0, its own commit)

- Raise to `pub`, and re-export from `FK/structural.rs` beside the private `mod retained;`, **exactly the items in K4 `RETURN.md` §16's export list, including "Revision 5a.3 adds to the export list"**, and nothing else. What that list excludes stays `pub(crate)`.
- `#[allow(dead_code)] // <consumer>` markers on items that now have a public path may be removed; nothing else in `K4R` changes.
- **A0 is a separate commit on your branch, with no other change.** V-K (I17) builds on the same commit, cherry-picked, so both PRs carry an identical export and whichever merges second merges main cleanly. Report A0 as soon as it compiles with FK's suite passing, before the rest of A.
- **Scan:** after A0, no crate other than `H` (and, later, V-K's `numerical_robustness`) names any exported item. PP, SA, NI, SD, the runners and the apps do not. Record the scan.

### 2. The W1 mode in `k6_observe`

Following K6 `RETURN.md` §13.1 "How K6b adds a W1 mode":
- a `Mode` variant (Q2 names it), with its own `materializes_n2` answer (no);
- **an adapter from K6's models to K4's `PrimitiveSource`** (`K6Model` → `SourceParts`), the RF-LARGE families only. The adapter is reviewed code. Its equality with R1's model is checked through K6's canonical model bytes and `references.py --model`, as K6 did;
- a staged sequence over K4's public entry (`solve_case`, and `solve_cases` where a model has several cases), under K6's allocator and stage observer. K4's entry is monolithic, so the stages the binary can time are the call as a whole plus what K4's evidence reports per attempt (Q1);
- the counts line gains limbs per entry by precision, K4's `StorageCounts`, and work by stage and precision from each `AttemptRecord`'s `StageWork`;
- `counts::admission_estimate_bytes` gains a W1 estimate, derived from K4's storage (profile entries × limbs × 8 B per precision, plus what the code keeps alive at the peak), as K6 derived E_adm;
- the runner's `MODES`, `TIERS` and `estimate_key` extended. The allocator, the JSONL kinds, the runner, the watchdog and the admission rule are reused unchanged.

### 3. What each W1 run records

- The outcome (`Selected` at p, `Refused` or `Unresolved`, with the reason), and the attempts' precisions, roles and outcomes.
- For a selected case: the published rows' digest, their classes, and **the comparison with R1's expected values under the unchanged predicate** at the sizes where K4 compared them (10 and 100 members), and at larger sizes where the reference exists. A published row outside its claim is a **stop**.
- Work by stage and precision, the storage counts, the heap peaks (whole call, and per precision where Q1 allows), the stage times, and the process RSS and footprint through the runner.

### 4. Models, sizes and the ascent (Q3)

- **RF-LARGE,** all three families and both orientations, at 10, 100 and 1,000 members, ascending under K6's rule (the previous size recorded first, the admission re-evaluated).
- **10,000 members only as ROOT approves** from the 1,000-member figures and the projected time (Q3).
- **The DEC-053 nine:** only those W1a covers (straight frames and global-axis springs); list the rest as refused by W1a's coverage, with the reason (Q4).
- **The ceiling runs of K6** (`K6-CEIL-*`, `K6-GRID-*`) are out of scope.

### 5. Seconds per work unit (Q5)

- Time comes from interleaved runs in one slot: for each model, the W1 run and K6's sparse binary64 run alternate A, B, A, B, so load drift affects both.
- The figure is seconds per work unit per stage where K4's evidence separates them, and for the call as a whole otherwise, with its load recorded. It is an observation, not a claim.

## ROOT rulings for this slice

Given at spawn, before the plan:
- **A0 carries the export for both K6b and V-K** (above). Whichever PR merges first carries it to main.
- **Observation only.** The constraints of `I15_K6_IMPLEMENTATION.md` ("Constraints") apply unchanged, with W1 added: no test or record asserts a time or memory bound; the claims RETURN may make are growth fits of heap and RSS against members per family and precision, and the ratios of measured heap to the W1 estimate.
- **Host:** as in K6's brief §6, with `<wt>/k6b-target`, `<wt>/scratch/i16` and `<wt>/k6b-mut/`. Observation runs are `--release` from a `git archive` of the exact commit, one observation process at a time, in a slot ROOT grants.

## Open questions for your checkpoint-0 plan (options and a recommendation for each)

- **Q1: peak memory per precision.** K4's `solve_case` runs the whole schedule in one call. Options: (a) the whole call's heap peak, with the selected precision and the attempts, and the per-precision storage derived from K4's `StorageCounts` and checked against measured peaks at small sizes; (b) a stage observer inside `K4R`, behind a feature K6b adds; (c) something else. Say which gives per-precision peaks without changing FK beyond A0. ROOT's prior: (a); (b) needs its own ruling, since it edits `K4R`.
- **Q2:** the mode's name and CLI, the counts-line fields, and the JSONL additions.
- **Q3:** the W1 schedule: every (model, size) with its estimate, admission and reason, and a projected slot time. State the per-case limit (`CaseLimit`) and invocation meter you run under: large enough never to stop a run, recorded, and reported if reached.
- **Q4:** which of the DEC-053 nine W1a covers, and why.
- **Q5:** the interleaving design: pairs, order, repeats, and what is divided by what.
- **Q6:** how the adapter is checked independently of K4's test adapter (`K4T/models.rs`) and of K6's generator.
- **Q7:** anything in K4's evidence that cannot be observed through the export list, and what you propose instead (no edit beyond A0 without a ruling).

## Write set

- `FK/structural.rs` and `K4R/**`: A0's visibility changes and the `pub use` only.
- `H/**`: the W1 mode, the adapter, counts, admission, runner extensions, tests, and `H/observations/k6b/`.
- `P/tests/test_performance_harness_runner.py` only if the runner's tests need it.
- `T3/IMPLEMENTATION/K6B/`.

Anything else is a stop.

## Required tests

- The export: a test in `H` that builds a `PrimitiveSource` and solves it through the public path (proving the export is sufficient); FK's full suite unchanged and passing.
- The adapter: canonical bytes against K6's model and `references.py --model` on the RF-LARGE sizes used; `PrimitiveSource::encoding` determinism.
- The W1 mode: determinism of counts, work and outcomes across two runs; equality of the published rows with K4's own `solve_case` on the same source; the unchanged predicate against R1 at 10 and 100 members.
- The runner: the W1 estimate and admission; the extended `MODES` and `TIERS`.
- Debug tests stay at 100 members or fewer.

## Mutants

At C, with the NONE control first, each from a clean `git archive` with its own target: the adapter (a dropped member, a swapped node order, a wrong section property), the work accounting (a stage dropped, a precision mislabelled), the W1 estimate, the runner's W1 admission, and the export (an item missing from the list). Report survivors and equivalences.

## Gates (ROOT runs the PR)

As in `I15_K6_IMPLEMENTATION.md` ("Gates"), with:
- FK's full suite, K4's suite, and `H`'s debug suite with its time;
- the scan of Scope 1;
- **T9 and the both-entry gate: not run,** provided the scan shows no product crate names an exported item (no published byte can change);
- an independent reviewer who re-derives the W1 estimate, re-checks the adapter against `references.py`, re-runs the mutants and traces every number to the raw JSONL.

## Checkpoints

End your turn at each one with a status for ROOT: the changed files, the results, and any stop.
- **0: the plan,** before any code, with Q1–Q7.
- **A0: the export commit.**
- **A1: the W1 mode and adapter,** with the tests, a warning-free non-test build, and `H`'s debug suite time.
- **A2: the runner and `--plan`'s W1 schedule,** for ROOT's approval. No W1 run above 100 members before ROOT approves it.
- **B: the observation runs** in ROOT's slots, in the approved order, with the packet and fits. ROOT rules before C.
- **C: mutants. D: CHANGE_RECORD, RETURN, `_run_records/` and SHA256SUMS.**

**Stop and report** on any of these: an edit outside the write set, or in `K4R` beyond A0; a published W1 row outside its claim, or a disagreement with K4's own `solve_case`; an adapter model that differs from `references.py --model`; a watchdog kill or heap-cap abort on an admitted run; a run outside the approved schedule; a SIGKILL from the memory guard; a surviving mutant.

## Return

As in `I15_K6_IMPLEMENTATION.md` ("Return"), with an **"Interface for ROOT's W1 limits, V-K and F2a"** section: per family and size, the selected precision, work by stage and precision, storage, heap and RSS, and seconds per work unit with its load; the W1 estimate and its measured ratios, with the sizes covered and not covered; and what ROOT still needs that K6b did not measure.
