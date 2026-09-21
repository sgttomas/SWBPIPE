# Gate wave 2 assessment (Agent 0)

**Inputs.**
- The five package verification reports: `PKG-0N/PKG-0N_VERIFICATION.md`.
- The DEL-03-07 rerun report, `PKG-03/PKG-03_RERUN1_VERIFICATION.md`.
- Agent 0's rechecks, recorded in `RUN_STATE.jsonl`.
- The recorded resolutions, `RESOLUTIONS.csv` (122 rows).

The gate is defined in `WAVE_PLAN.md`. It uses the wave 1 conditions,
judged per package.

## Result: MET

| Package | Verdict | Firm false alignment (sampled ALIGNED normative) |
|---|---|---|
| PKG-00 | ACCEPT WITH CONTESTED ROWS | 4.0% (1/25) |
| PKG-01 | ACCEPT WITH CONTESTED ROWS | 0.0% (0/50) |
| PKG-02 | ACCEPT WITH CONTESTED ROWS | 4.3% (3/69) |
| PKG-03 | ACCEPT WITH CONTESTED ROWS, after one rerun cycle | 1.5% (1/68) |
| PKG-04 | ACCEPT WITH CONTESTED ROWS | 0.0% (0/47) |

Across the wave the rate is 5 of 259, or 1.9%. Wave 1 was 4.4%.

**The PKG-03 figure.** The first verifier's rate was 2.9% (2 of 68).
- It returned `RERUN DEL-03-07`. The cause was CLM-021, an F1 row whose
  correct tier is INVARIANT.
- A fresh manager and worker re-encoded DEL-03-07. The first-run files are
  kept byte-identical in `superseded_1/`.
- A fresh verifier then found 0 of 14 aligned normative rows wrong. In the
  PKG-03 sample, the re-verified DEL-03-07 slice replaces the first one.
- The rerun counts towards the gate (`WAVE_PLAN.md`, Verifier reruns).

**Condition by condition.**
1. **Verdicts.** Every package returned ACCEPT WITH CONTESTED ROWS.
2. **Firm false alignment.** Every package is at or below 5%. PKG-02 is the
   closest, at 4.3%. Counting a row the worker flagged itself, which falls
   outside the verifier's sample, it would be 5.7%.
3. **Shared-situation conflicts.** Batch mode with `--resolutions` returns
   0 findings for the 31 W2 ledgers and 0 for the 13 W1 ledgers. The
   resolutions come from these sources:
   - verifier-resolved pairs: five CP-04 variant rows and two CP-03
     non-conflict rows (`RESOLVED_PAIR`);
   - the corpus-wide split of the "PKG-00 at SEMANTIC_READY"
     architecture-basis sub-claims. That split is invisible to batch mode,
     because minted `.sNN` sub-claims have no body hash. Its disposition is
     resolved by an **Agent 0 reading** of the F3 exception, not a ruling:
     a readiness state is treated as a review state and stays
     `STALE_REVIEW_OR_EVIDENCE`. Three verifiers recommended this reading
     (PKG-03 W1, PKG-04 W-1, the DEL-03-07 rerun), and it overrides 11
     sealed rows that applied F3's origin test literally. The rows are
     recorded as `AGENT_READING`, 23 in W2 and 9 in W1, and the reading goes
     to the owner with the canonical table for confirmation.
   - The cause of that split (`RECORD_DRIFT` against
     `SCOPE_REDIRECTED_BY_RULING`) is left as one contested cluster, SR-1,
     for a single R3 resolution.
4. **Validation.** All 44 ledgers pass single mode. The 31 W2 ledgers also
   pass the Part F checks (`--notes-gap`).

## What Part F changed, and what it did not

- **Fewer firm errors.** The pooled firm false-alignment rate fell from 4.4%
  to 1.9%.
- **F1 is still broken sometimes.** Four PKG-02 rows and one DEL-03-07 row
  were marked ALIGNED while deferring an unmet element to another row. The
  gap-wording check lists such rows only when their Notes use gap words, so
  it catches the pattern but does not guarantee it is gone.
- **F7 markers are often missing.** Workers disclosed 72 missing
  `PRODUCT_CALLER: NONE` markers in PKG-02, plus others in PKG-03 and
  PKG-04. No row's disposition was wrong as a result. The marker cannot be
  checked mechanically.

## Changes before the rolling queue (tooling only; no convention change)

The validator was changed after the W2 verifiers finished, and before any
further dispatch:
- **CP-04 variants.** Batch mode compares CP-04 rows within the three
  variants `CANONICAL_SITUATIONS.md` defines (by tier and baseline class). Any
  other pair falls into the default group and is compared there. This removes
  five false flags.
- **Resolved pairs.** A row recorded as `RESOLVED_PAIR`, with a verifier
  report as its source, is exempt from body and pattern comparison.
- **Separators.** Spaces around a `;` in evidence columns are rejected (Part D).
- **Paths with spaces.** Evidence tokens may contain spaces when they
  resolve to a path at the freeze, because deliverable folder names contain
  spaces. Before this change, workers could cite deliverable-local evidence
  only in ContextRefs.

With the changed validator, all 44 ledgers still pass.

## Items for R3/R4 (not gate conditions)

These are collected for the owner's attention. They do not block
scale-out:

**Owner rulings needed.**
- Product posture: "free and open-source" (PRD v0.4, README) against
  "source-available noncommercial" (DIRECTIVE, licence). DEL-01-01
  `CLM-009.s01` is AUTHORITY_CONFLICT.
- The DEC-081 edit to the ISSUED DEL-01-01 SOW (AC-001), recorded as
  ACCEPTED_DIVERGENCE, needs confirmation. Also decide how rename residue
  in ISSUED text is routed; today it is routed twice.
- Export formats: AB-00-07 at revision 0.12 says "TBD", while the accepted
  SCA-004 names them.
- Holds that code settled with no ruling (CP-10): package manager, state
  library, undo/redo storage, severity taxonomy, and the DEL-02-03, 02-01,
  02-02 and 03-07 vocabulary choices.
- The DEL-03-07 R-005-style question of whether a draft policy
  (`IP_AND_DATA_BOUNDARY.md` §4) counts as governing.

**Code-change candidates.**
- The Python persistence service hashes with sorted compact JSON rather
  than JCS (AB-00-04, DEC-010, DEC-017, SPEC).
- The unit check accepts bare numbers (Python and Rust; OPS-K-UNIT-1).
- The section calculator: accepts a mass-dimension "length", drops
  provenance, and omits the corrosion allowance in the product routine.
- The result-envelope binding drops affected objects and hard-codes the
  former product name.
- A stale sparse-solver remediation message.

**Verification and evidence gaps.**
- Merged PR #787 removed a protected check (nonlinear loop identity).
- There is no current protected-content review of the solver crates.
- Unit safety exists only at metadata grain.

**Ownership gaps.**
- `nonlinear_integration` and `curved_bend`.
- The product section and mass routine.
- Routing gaps: CAP-COREC-053, CAP-SHELL-032/048 and CAP-PHYS-039 were not
  routed to their likely owners.

**Convention questions recorded as contested clusters.**
- The SEMANTIC_READY cause (SR-1).
- Whether a schema-only SOW is stale once product physics lands.
- Whether envelope round-trip tests satisfy AB-00-04.
- CP-09 applied to VER-001.
- Import-checker CONSTRAINS against COVERS.

## Next

1. Independent review of the validator change and the two resolutions
   files.
2. Show the owner this result and the canonical situation table, then
   start the rolling queue at 16 live agents (`WAVE_PLAN.md`).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
