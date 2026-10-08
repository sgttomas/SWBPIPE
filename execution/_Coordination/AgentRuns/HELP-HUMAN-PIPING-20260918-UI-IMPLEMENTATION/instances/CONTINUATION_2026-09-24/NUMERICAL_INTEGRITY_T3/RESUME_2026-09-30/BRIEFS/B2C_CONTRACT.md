# B2-C: the B2 contract (combinations, mixed and preparation-only; documents and code reading only)

TASK (Type 2), a designer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is your return path, and you do not delegate. **You are a fresh instance.** Cite the records, and assume nothing beyond them.

## Why

B2 extends B1's multi-case retained transaction to model combinations (I93's plan, PLAN §1.2):
- a mechanics combination with at least one selected operand is retained;
- its operands are sourced from a selected solve, a prepared source, or one shared `OperandPreparation`;
- every other combination has a coverage entry and no W1 work.

Your contract is the text the kernel (B2-K), the producer (B2-P), admission (B2-A), the readers and 07o implement. A fresh reviewer (RV-C) reviews it, and ROOT selects it. Its draft statics land in J1's package with B3b's (decision 30's interim registration).

## The basis

- **The plan:**
  - `R/I93/b2b3_plan_01/PLAN.md` (`e1147dbd…`): §1.2.1–§1.2.2, §1.2.6–§1.2.8 and decisions 5–10 and 20–21;
  - `REVISION_01.md` (`63abb73f…`): §1.1 (S-1: the combination's own formation definition and `ProductAttempt`), §1.3 (one owner per file), §1.4 (J1), §2 (decision 9 re-ruled, and N-12), §5's B2-C outline, and decisions 28–31;
  - RV114's review, `R/REVIEW_RV114/b2b3_plan_01/REVIEW.md`;
  - RR "B2/B3 R1: …" and "I93's REVISION_01 accepted; …".
- **B0's contract:** `R/I78/b0_contract_01/DESIGN_v2.md` §4 (C3a's P1 text, with the reserved spellings `operand_preparations`, `OperandPreparation`, `operand_preparation_failure`, `retained_precision_operand_preparation_v1` and `combination_operand`) and decision 12. Also C1 §5–§6, C2, C3 and SC1 §2, by the plan's notation.
- **B1's transaction, T-1 to T-13, as RV-P round 2 confirmed it:** RV109's round 2 report `R/REVIEW_RV109/rvp_round2_01/REVIEW.md` (`206fd360…`) on `b1` at `603e238517`, and RR "RV109 passes SP in RV-P round 2; SF-1 and SF-2 go to I3's pinning step; B2-C dispatched as I97". B2's T-9b and T-9c sit after B1's T-9. Read B1's code on `b1` (read-only) for the stage names, the meter, the notice reservation and staging.
- **The kernel design,** as accepted: I94's `R/I94/b2_kd_01/DESIGN.md`, with RV115's review (`R/REVIEW_RV115/b2_kd_01/REVIEW.md`) and RR "RV115 (RV-K) accepts B2-KD with amendments; R-1 to R-11 ruled". These items are yours:
  - **N-4:** map `no_selected_operand` under operand validation, which now has four reasons;
  - **N-5:** state that a combination's origin refusal never becomes a capture error (`CaptureError::Origin`). It abandons the successor under decision 7. This is R-1's condition for reusing `MissingSelectedOrigin`;
  - **R-7:** the combination coverage rule. Confirm the published row set (7n + 50m + 8g; slots 0–19);
  - **R-8:** DEF-C's `scope.operand_equality` binds equal prepared section facts across operands, as well as the kernel's K primitives;
  - **R-10:** DEF-C reuses `retained_precision_formation_v1` as its hash domain (recommended), or you reserve a new one with reasons.
- **B3-D, final for J1** (RR "RV116 confirms B3-D's revision 01; B3-D is final for J1"):
  - I96's `R/I96/b3_d_01/DESIGN.md` with `REVISION_01.md`, and the statics in `statics/r1/`.
  - **`receipt_bindings` (B3D-8):** PTABLE's revision uses the same member as XTABLE, spelled and shaped identically: `canonicalization`, `method`, `projection_policy`, `work` {`case_limit`, `invocation_limit`} and `work_policy`, keyed by the receipt body's own paths. XTABLE's bytes are the reference.
  - **NA-1:** B3b's whole SCHEMA change is `statics/r1/SCHEMA_ENUM.diff` (the `definition_id` enum, N-8's title and `$comment`). Your SCHEMA diff must compose with it into one coherent J1 text. Propose that merged text.
- **The statics at NUM's maintained tree:** PTABLE, DEF-O, SCHEMA and CORPUS, by the plan's notation.
- **The T6S consistency notes** (I74 PLAN §4.3), and decision 21's B2 part (tests only, N-8).

## What the contract must give (`CONTRACT.md`, plus draft statics as records)

1. **C3a's completions** (PLAN §1.2.2): placement, record point, work, gate placement and codes.
2. **The combination transaction,** T-9b and T-9c:
   - dispositions;
   - the `MechanicsCombinationCall` on the one meter;
   - `CombinationSource`;
   - Group imports (the first selected operand's group, per S-2);
   - the Run and its own freeze;
   - the failure split (decision 7);
   - staging, diagnostics and `execution_order`.

   Each step needs its exact place relative to B1's confirmed T-1 to T-13.
3. **The combination's formation definition** (decision 28; REVISION §1.1 item 1), `RP-PREPARED-COMBINATION-DUAL-v1` (suggested), at `P/fixtures/results/retained_precision_prepared_combination_v1.json`:
   - the draft JSON;
   - its H under R-10's domain;
   - a collision check.

   DEF-O's bytes stay unchanged.
4. **The combination's `ProductAttempt`** (REVISION §1.1 item 2): its owner kind, its stages, its order in `product_attempts[]`, and `product_attempt_ref`.
5. **R-COMB-1's representation** (decision 8): reader-derived, or a new receipt member. Give both with a recommendation; ROOT selects. Name DN §4.2.
6. **SCHEMA's `$defs`:**
   - B2's diff against main (PLAN §1.2.2 and REVISION §1.1 item 3: the `definition_id` enums, `owner_ref.kind`, and `Body.sources` admitting `CombinationSource`);
   - the merged J1 text with B3b's `SCHEMA_ENUM.diff` (NA-1).
7. **PTABLE's one revision** (REVISION §2):
   - the scope extension;
   - `product_formation_definitions` and `formation_warrant` gaining DEF-C;
   - `receipt_bindings` as above.

   The contract id, profile, rows, inherited hash and every other member stay unchanged.
8. **G0 per N-12 and decision 31,** in all three readers: the table read, the constants kept as a cross-check, and the failure code. Also the 07o mutations and the reader-local unit tests it implies.
9. **The D1.4 text and the cap rows** (decision 10): c ≤ 3; C_eq ≤ 3; at most 3 terms; at most 3 range operands; components 0. Also N-4's reason that C1's 60B limit stays non-binding.
10. **Gate and code placement** for every new failure, with the 07o first failures stated per reader as far as the present code allows (SC2 establishes them).
11. **The decisions,** each with alternatives and a recommendation, and **a collision log** of every new name for ROOT to reserve.
12. **Estimates** for B2-A, B2-P, the B2 readers and B2's part of SC2, refining the plan's.

**The draft statics** (records only, ready for J1's package):
- the DEF-C JSON;
- PTABLE's revision;
- B2's SCHEMA diff;
- the merged J1 SCHEMA text.

Write them with a generator script in `_run_records/`, run twice byte-identical, as I96's `b3d_statics.py` does, so a reviewer can rebuild every hash.

## Rules

- **Documents and code reading only.** You may run read-only Python with VENV against committed files.
- **Not allowed:** cargo, native jobs, installs, and Git writes. Read Git with `GIT_OPTIONAL_LOCKS=0`.
- **Writes:** absolute paths only.
  - Scratch goes in `WT/scratch/<id>_b2_c/`.
  - Records hold placeholder paths only.
  - No record folder is named `build`.
  - Run `git status --ignored` on your record folder before returning.

## Output

- **The record:** `R/<id>/b2_c_01/CONTRACT.md`, with the draft statics, `_run_records/` and SHA256SUMS.
- **Budget:** 11–18 h.
- **End your turn with:**
  - CONTRACT.md's sha256;
  - the transaction in brief;
  - DEF-C and the `ProductAttempt` in brief;
  - R-COMB-1's recommendation;
  - the statics' hashes;
  - the names to reserve;
  - the estimates;
  - anything ROOT must rule on.
