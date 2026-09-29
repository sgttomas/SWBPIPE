# I17: implement slice V-K (the VP-ROBUST kernel lane, in the new `numerical_robustness` crate)

> This is an implementation TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host" and "Platform calibration", `I8R_K1_RESUME.md:24-50`) override `_COMMON.md`'s host section, and apply to you in full, with V-K's paths below in place of K1's.

## Roles

- ROOT (HELP_HUMAN) dispatches you directly, as a background subagent, and is your return path. There is no separate T3 manager on the Mac.
- Make no Git writes and no index operations. ROOT commits.
- Record the delegation mechanism in RETURN.

## Paths and bases used in this brief

- `P/`, `T3/`, `FK`, `SD`, `NI`, `SA` and `PP` are as in `I15_K6_IMPLEMENTATION.md` ("Paths and bases"). `K4R` is `FK/structural/retained/`, `K4T` is `P/core/solver/frame_kernel/tests/retained_k4/`, and `VR` is `P/validation/benchmarks/numerical_robustness/` (new).
- **Code is cited at K4's candidate head `7d8fa9c0e`** (PR #1054). ROOT records the actual base at spawn: main after K4 merges, plus K6b's export commit A0 cherry-picked (below). Re-locate every line on your base.
- **T3 records are cited at the numerics head,** read in `<wt>/numerics`.

## Purpose

`DESIGN.md` §6's V-K row: "V-K: VP-ROBUST kernel lane, after R1's references are frozen and K4; new `numerical_robustness/**`; §4.10 kernel lane, the zero-scale floor check." V-K builds the standing harness that checks W1a's kernel method against R1's frozen references, independently of K4's own tests, and records per-case outcomes and work for ROOT's W1 limits ("K4: Q5 amended": the limits come from K6, K6b and V-K's kernel-lane runs, with K4's work counts).

K4 has already compared RF-CHAIN, RF-SKEW, RF-WEAK, RF-FINITE, RF-MECH and RF-CANCEL, and RF-LARGE at 10 and 100 members, inside its own tests (K4 `RETURN.md` §12). V-K is the independent, standing version of that check, and it adds what K4 did not run:
- **RF-INVARIANCE** (25 cases), **RF-RANGE** (32) and **RF-ZERO** (4);
- **RF-LARGE at 1,000 and 10,000 members** (as examples, under the host rules);
- the §4.10 reporting and gate: `pass`, `fail`, `not_covered` and `pass_absolute_range`, and the enumerated `not_covered` list;
- the discrimination check and the seeded faults, with a kill matrix;
- Q7 of K4's brief: the equality of K4's RCM port with SD's.

### What V-K does not do

- No product-lane run: every request through PP is V-P's, after F2a.
- No change to K4's method, any published row, or any product path.
- No limit or threshold. It records work and outcomes; ROOT sets the limits.
- No edit of R1's references, ever. A disagreement is a finding.

## Scope

1. **The crate** (`DESIGN.md` §4.10 "Crate"): `VR`, with its own `Cargo.lock`, discovered by CI automatically. Its dependencies are `frame_kernel`, `sparse_direct`, and `serde_json` with `float_roundtrip`. Whether it depends on `product_physics` now or V-P adds it is Q1.
2. **The case adapters** (`VR/cases/`): from R1's `references.json` (`7b176dbb…`) to K4's `PrimitiveSource`, through the export. The adapter is reviewed code, written independently of `K4T`'s adapter (`K4T/models.rs`, K4 `RETURN.md` §12.1), and each model is checked against `references.py --model` (`80d473a7…`).
   - **CI omits `projects/*/execution/`** (`.github/workflows/piping-desktop-e2e.yml:188`), so no CI test may read R1's files there. How the crate carries R1's cases and expected values, pinned to R1's sha256, is Q2.
3. **The kernel lane** (§4.10 "Kernel lane"): the families RF-CHAIN, RF-SKEW, RF-WEAK, RF-LARGE, RF-INVARIANCE, RF-RANGE, RF-ZERO, RF-FINITE and RF-MECH through the kernel method, and RF-MECH and RF-LARGE also through the binary64 sparse gate for the parity checks. **RF-CANCEL** is included as K4 ran it, with the binding net-governed scale, except its UDL cases, which are W1b's. Cases flagged `needs_directional_spring` run with K4's kernel-only `DirectionalSpring`.
4. **What is compared** (§4.10 "What is compared"): nodal displacements and rotations; the six signed reaction components per support; axial force, torque and `hypot(My, Mz)` at the ends and stations; twist and extension derived as `T/k_t` and `N/k_a`, never differenced; the method evidence (the selected precision, the attempts, the outcome). RF-MECH must be refused with a witness or end unresolved, with no rows.
5. **The predicate:** `|obs − exp| ≤ 1e-9·max(|exp|, scale)` with R1's zero scales, unchanged. The represented basis where R1 marks it (`RF-SKEW-A-CANT-AX-122-r1e-12`, `RF-FINITE-THIRTIETHS-O1e6`, and every `finite_input` case).
6. **The zero-scale floor check and the reporting** (§4.10, V1-S8, F2, F8): R = 2^-34 and S\* by §4.1.6.1's kinds, variant F for twist and extension; the four outcomes, reported as three separate counts; `pass_absolute_range` for RF-LARGE-CONT-n10000, whose expected values lie below binary64's range and are parsed exactly from their decimal strings.
   - **The enumerated `not_covered` list** is committed with the harness: RF-WEAK 46, RF-CANCEL 3, RF-SKEW 2, as §4.10 lists and K4 reproduced. A new `not_covered` comparison, or one that leaves the list, blocks the gate until ROOT reviews it.
7. **The discrimination check:** each of R1's discriminating negative controls must fail the same predicate; a control that does not is reported as non-discriminating, never dropped.
8. **RF-RANGE** (§4.7, §4.10): LEF-small and LEF-large "must be solved"; a named range refusal is recorded as a failure. **Whether W1's source admits LEF-small is V-K's question** (`ROOT_RULINGS_V1.md`, "Design text made stale by K1, K2b and K3" item 10). If K4's `PrimitiveSource` refuses a RF-RANGE case, record the refusal with its `SourceError` and report it as a finding for ROOT; do not change K4.
9. **The seeded faults** (§4.10, §7.3): behind a `frame_kernel` feature `mutation-controls`, `#[cfg(any(test, feature = "mutation-controls"))]`, enabled only by `VR`'s mutation run. A CI-run test checks that no product manifest enables it. Each fault must fail at least one comparison; the kill matrix is recorded. The fault list is Q4.
10. **RCM equality** (K4's brief Q7): `retained::factor::reverse_cuthill_mckee` equals SD's RCM on every adapted model's adjacency.
11. **Per-case records for ROOT's limits:** for every case, the outcome, the selected precision, the attempts, and the work by stage and precision from K4's evidence, committed as JSON with hashes under `VR/observations/`. Time is recorded with its load, as an observation only.
12. **Scale runs** as examples, not tests (§4.10): RF-LARGE at 1,000 and 10,000 members, under K6's runner and the ascent rule (Q5).

## ROOT rulings for this slice

Given at spawn, before the plan:
- **The export** is K6b's checkpoint A0 (`I16_K6B_IMPLEMENTATION.md` Scope 1), exactly K4 `RETURN.md` §16's list. ROOT cherry-picks that commit onto your branch at spawn, or as soon as it exists. You make no other visibility change in FK; a needed item outside the list is a stop.
- **The seeded-fault feature** is V-K's only other FK change: the feature in `FK`'s `Cargo.toml` and the `cfg`-gated fault sites. With the feature off, FK's code must be byte-identical in effect (the reviewer checks).
- **Host:** as in `I15_K6_IMPLEMENTATION.md` §6, with `<wt>/vk-target`, `<wt>/scratch/i17` and `<wt>/vk-mut/`. K6b (I16) runs timed observations in ROOT's slots; during them you build and run nothing heavy. Your own scale runs also take slots ROOT grants.

## Open questions for your checkpoint-0 plan (options and a recommendation for each)

- **Q1:** `product_physics` as a dependency now (§4.10's list) or added by V-P. What it costs in CI build time.
- **Q2:** how `VR` carries R1's cases and expected values so CI can run them: a generated, committed cases file with R1's sha256 pinned and a `--check` regeneration (as K4's generator), or another way. Include how RF-LARGE-CONT-n10000's sub-range decimals are carried exactly.
- **Q3:** which comparisons run as CI tests and which as examples, with the numerical job's 45-minute budget in mind (`.github/workflows/piping-desktop-e2e.yml:176`). State the projected debug suite time.
- **Q4:** the seeded-fault list, mapped to §7.3's items and to R7 §7's mutants where they apply, with the comparison each is expected to fail.
- **Q5:** the scale-run schedule: every RF-LARGE (family, orientation, size) with K6b's W1 estimate (or yours, derived the same way), its admission, and a projected slot time. 10,000 members only as ROOT approves.
- **Q6:** how the adapter is checked independently of `K4T/models.rs`, and where the two are expected to differ.
- **Q7:** RF-INVARIANCE's relabelling and permutation checks: what "the same answers" means per quantity, given R1's references.
- **Q8:** anything V-K needs that the export list does not provide (no edit without a ruling).

## Write set

- `VR/**` (new).
- `FK/Cargo.toml` (the feature) and the `cfg`-gated fault sites in `K4R` and FK.
- A CI-run test that no product manifest enables `mutation-controls` (in `VR` or in P's tools; say where).
- `T3/IMPLEMENTATION/VK/`.

Anything else is a stop. R1's `REFERENCES/**` is read-only.

## Required tests

- The adapter against `references.py --model`, by canonical bytes or exact values, on every adapted case.
- The kernel lane over every in-scope case, with the report's counts pinned: passes, absolute-range passes and not-covered, and zero failures.
- The `not_covered` set equal to the committed list; the discrimination check; RF-MECH's refusals.
- The seeded faults: the kill matrix, each fault killed (run under the feature, in `VR`'s mutation run).
- RCM equality.
- The feature guard: no product manifest enables `mutation-controls`.

## Gates (ROOT runs the PR)

As in `I15_K6_IMPLEMENTATION.md` ("Gates"), with:
- FK's full suite with the feature off, K4's suite, and `VR`'s suite with its time;
- DEC-025: the sweep gains `VR`'s manifest; every other suite is unchanged against the Mac baseline;
- **T9 and the both-entry gate: not run,** provided the scans show no product path changes (the export is unused by product crates; the feature is off in every product manifest);
- an independent reviewer who re-checks the adapter against `references.py`, re-derives the floor check and the `not_covered` list, re-runs the seeded faults, and checks that the harness cannot pass a wrong answer (a probe of their own).

## Checkpoints

End your turn at each one with a status for ROOT: the changed files, the results, and any stop.
- **0: the plan,** before any code, with Q1–Q8.
- **A1: the crate, the adapters and the kernel lane** at CI scale, with the report and the floor check.
- **A2: the seeded faults and the kill matrix; RCM equality; the feature guard.**
- **B: the scale runs** in ROOT's slots, in the approved order. ROOT rules before C.
- **C: mutants of the harness itself** (a comparison dropped, a scale wrong, a `not_covered` misreported), with the NONE control first.
- **D: CHANGE_RECORD, RETURN, `_run_records/` and SHA256SUMS.**

**Stop and report** on any of these: a failing comparison on a covered row (never tune it; record and report); a `not_covered` set different from the committed list; a discriminating control that passes; an adapter model that differs from `references.py --model`; a RF-RANGE refusal (a finding, not a stop, but report it at once); an edit outside the write set; a watchdog kill or heap-cap abort; a SIGKILL from the memory guard; a surviving seeded fault.

## Return

As in `I15_K6_IMPLEMENTATION.md` ("Return"), with an **"Interface for ROOT's W1 limits, V-P and F2a"** section: per family, the outcomes and selected precisions, and the work by stage and precision (distribution, maximum, and the cases at the maximum); the RF-RANGE findings; the kill matrix; and how V-P extends `VR` to the product lane.
