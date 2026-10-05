# I74: plan the T6 successor-output slice (pulled forward by the owner)

TASK (Type 2), planning only. Dispatched by ROOT (HELP_HUMAN, Agent 0), who is your return path; you do not delegate. **You are a fresh instance.**

**Planning only:** no source edits, no Git writes, and no cargo, native or solver jobs. Reads only.

## Why

The owner decided the T6 conflict on 2026-10-05: **"Pull T6 slice forward"** (RR "I61's U8 plan ruled; the owner pulls T6's successor-output slice forward; …", decision 12).
- **The blocker:** public activation's checklist item 4 needs T6's successor outputs. Without them, activation waits, and through the owner's order so do S-I2, F2b and F3.
- **The slice** runs in parallel with F2a's breadth, so it is ready before B8 (activation).

## The basis

- **The checklist and the T6 items:** the merged package's `T3/IMPLEMENTATION/F2A_D1/CHANGE_RECORD.md` §4 (item 4) and §5.5 (T6):
  - the successor outputs replacing the explicit N-5 panel refusal;
  - the `results.schema.yaml` v0.3 dispatcher, which knows only precision-1;
  - RV95 N-5, a test of the 2^53−1 bound in `source_blocks::integer`.
- **The work graph's T6 row** (`P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md`) for T6's scope, groups and dependencies. Pull forward only what checklist item 4 needs, not all of T6.
- **The successor contract:**
  - M03-INTEGRITY-MP-v2;
  - the schema `P/schemas/retained_precision_mp_v2.schema.json`;
  - the readers' standing (Python `core/analysis_runs/retained_precision.py`, Rust `result_export`, TS `retainedPrecision*.ts`);
  - D-U7-4: TS requires the live native capture;
  - D-U7-6: no producer-origin claim;
  - the carrier case file `fixtures/results/retained_precision_carrier_cases.json` (v4).
- **The current refusals:** the explicit panel gate (RV91 N-5, U7 slice T); the stress-neutral packager and D-U6-9 refusing successors.
- **Breadth's coming shapes** (`R/I61/u8_plan_01/PLAN.md` §2): B1 brings multi-case receipts, B2 combinations, and B3 the exact-route identity `physics-retained-1`. The slice must not freeze a single-case assumption.

## Deliverable: `NUM/R/I74/t6_slice_plan_01/PLAN.md` plus SHA256SUMS (placeholder paths only)

1. **Scope.** The minimal set of T6 work that satisfies checklist item 4. For each output path (result export, stress-neutral export, the panels, the v0.3 dispatcher, RV95 N-5):
   - what it must do with a successor;
   - what standing it shows;
   - what it refuses;
   - what stays closed until activation.
2. **The fence.** The write set (export paths, panels, dispatcher, tests). Confirm it touches no PP file and no file in the D1 call graph (QUALIFICATION §11), so it needs no re-qualification. If any item needs such a file, flag it.
3. **The contract questions.** Anything the export of a successor needs that the contract does not yet say, such as how a successor's withheld rows or `not_covered` disclosures appear in an export. Give a recommendation for each; owner-facing ones are marked as such.
4. **Slices, owners, reviewers, order and estimates.** Include how it reaches main (its own PR) and how it stays consistent as B1–B3 widen the receipt.
5. **Interaction** with S-I2 and checklist item 1 (src-tauri's `qualify_rule_mechanics_with_context`), and with open draft PR #885, which touches `src-tauri/src/lib.rs`.

## Budget and return

3 hours. Return once, with the plan's sha256, the numbered decisions (each with a recommendation and its decider), and an estimate summary.
