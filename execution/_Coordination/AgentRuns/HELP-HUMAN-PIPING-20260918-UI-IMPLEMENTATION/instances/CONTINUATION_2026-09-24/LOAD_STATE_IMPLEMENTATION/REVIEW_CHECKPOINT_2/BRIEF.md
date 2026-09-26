# TASK — independent review of load-state checkpoint 2

This is a fresh-context, non-author review. Its parent of record is the
session-2 WORKING_ITEMS load-state manager (SendMessage id `a3675abb28ada0834`).
ROOT spawns it on the manager's request and assigns the frozen candidate commit.
Report findings to the manager by SendMessage, and send the final report to
ROOT. Do not delegate further. Paths are WORKING_ROOT-relative.

## Candidate and basis

- **Candidate.** The commit ROOT names for the checkpoint-2 freeze on
  `codex/piping-load-states-20260925`. Review it as a detached snapshot in your
  own worktree. The review diff is `be1b9294a..<candidate>` for the files listed
  below; `be1b9294a` is the checkpoint-1 candidate that was already reviewed.
- **Engineering basis, which is immutable.** Read these with `git show`:
  - `9e8a55daecdeb9669131fd3e53c0e0303ee550d6:projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/CORRECTNESS_DESIGN/LOAD_REFERENCE_STATES/{DESIGN,INTERFACE,VERIFICATION,REFERENCES,HYDROSTATIC_CONTROL}.md`.
  - ROOT's selections are in `LOAD_STATE_IMPLEMENTATION/_run_records/ROOT_SELECTION.json`.
- **Implementation records.** In `LOAD_STATE_IMPLEMENTATION/`: `CHECKPOINT_2.md`,
  `CP2_WIRE.md` together with `CP2_WIRE_ADDENDUM_1.md`, `REVIEW_CHECKPOINT_1/RETURN.md`
  (the dispositions ROOT gave the manager), and `_run_records/session2/`.

## Files in scope

Manager-authored:
- `core/product_physics/src/lib.rs`, the diff only
- `core/product_physics/src/pressure_runtime.rs`, the diff only
- `core/product_physics/src/case_state/{mod,input,resolve,temperature,tests}.rs`
- the kernel repairs in `case_state/thermal.rs` (SF2 coverage and N1 consumed/consulted split) and its tests
- the new test in `case_state/material.rs` (N3)

TASK deliverables, which need an independence and meaningfulness check:
- `core/product_physics/tests/load_reference_state_runtime.rs`
- `tests/fixtures/load_reference_states/{reference_cases.json (additive extension), README.md}`

## What to judge

The judgement is whether these fit together to deliver the reviewed design's
first capability — M10 prescribed motion, M16 per-element material/thermal and
M29 axial fit — on the product route, with one resolved case consumed by
assembly, recovery and evidence, and with no old meaning or hash changed.
Focus on the following.

1. **Physics and consistency.**
   - Per-member E/ν with derived G reaches stiffness, pressure recovery and
     evidence alike.
   - The eigenstrain enters as `E_member·A_s·ε*` exactly once and is removed
     exactly once.
   - Prescribed values reach the structural solve, and the reduced-system
     coupling is correct.
   - The complete u includes g, and reactions come from the unreduced
     `K·u − f`.
   - The legacy observation lanes are consistent with the prescribed system.
   - Multiple cases rebuild their own stiffness.
2. **Source ledger.**
   - The effective case contains only the declared, factored primitives.
   - Unreferenced primitives are excluded, and a duplicate blocks.
   - No other code path re-reads the unfiltered case in a way that could
     double-consume or leak an excluded source. Check pressure, thermal
     legacy, self-weight, evidence and summary.
3. **No inherited eligibility.** For 0.4.0, retained-source recovery and
   receipt finalization are not reachable, and the not-joined record is
   truthful. Confirm that `source_receipt`/`composite` paths cannot run for
   0.4.0 inputs.
4. **Backward compatibility.**
   - Every pre-0.4 behaviour and serialized meaning is unchanged. That covers
     the `PreviewModel` manual `Deserialize` wire (material `expansion_laws`
     split), `is_exact` now accepting 0.4.0, `validate_profile`, the
     `solve_preview_reduced_system` signature change, and the observation
     force for old models.
   - Old documents that carry new fields block.
   - Consider the downstream crates that deserialize `PreviewModel`
     (operation_applier, self_weight_wasm, headless).
5. **Typed boundary and diagnostics.** Closed unions, deny-unknown, no silent
   defaults, no ambient temperature, no synthesized material identity and no
   reinterpreted legacy α.
6. **Repairs SF1–SF3 and N1–N5.** Recheck each against your own derivation or
   probes, and confirm the repairs themselves are right. SF3 in particular:
   check the exact-rational identity (shortest round-trip decimal, affine
   definitions and i128 range handling) and the order-preservation refusal.
   Confirm that no decision still depends on affine rounding and that no
   tolerance snapping was introduced.
7. **Tests.**
   - Are they meaningful? Mutation probes are welcome; run them on scratch
     copies only.
   - Are expected values independent of the implementation?
   - Protected criteria (relative 1e-9) must not be weakened.
8. **Records.** Check the accuracy of CHECKPOINT_2.md's claims and evidence.
   Machine-specific paths may appear only in `_run_records/`.

## Boundaries and resources

- **Writes.** Write only to `LOAD_STATE_IMPLEMENTATION/REVIEW_CHECKPOINT_2/`
  (RETURN.md and `_run_records/`). Probes and mutants go in scratch copies
  only. Never edit reviewed bytes. Make no Git writes.
- **Cargo.** Use `cargo +1.97.1 … --locked --offline -j 1` with your own
  `CARGO_TARGET_DIR` outside the repository. Delete your target when done:
  disk is limited.
- **No other lanes.** No browser, native, UI or npm.

## Return

Give a verdict: CLEAR, FINDINGS or BLOCKING. Present findings in a table with
columns for severity, location, concrete failure scenario and suggested repair.
Also include what you verified and how, raw logs, and limits. Write RETURN.md
in this directory.
