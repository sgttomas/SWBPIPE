# I75: the T6 slice's desktop work (T6S-3, T6S-5, then T6S-4)

TASK (Type 2). Read `BRIEFS/T6S_COMMON.md` first; it binds you. You are the TypeScript owner of the slice. Your plan sections are I74's PLAN §1.1–§1.3, §3 (CQ-1 to CQ-6, CQ-11) and §4 (T6S-3, -4, -5), as ruled.

## Order and checkpoint

1. **T6S-3,** the gate split.
2. **T6S-5,** stress-neutral.
3. **Checkpoint:** write `R/I75/t6s_01/CHECKPOINT_1.md` with SHA256SUMS and end your turn. ROOT verifies it, and continues you with I76's golden file hashes.
4. **T6S-4,** result export, against I76's goldens.
5. The final RETURN.

## T6S-3: the gate split and the output policy (decision 2, 12)

- **Split `loadReferenceOutputRefusal`'s use, not its meaning.** The two panels (Result Export, Stress-Neutral Export) gain an explicit successor admission. Every other surface (the 18 in `retainedPrecisionOutputRefusal.test.tsx`, the report package, the Rule-check panel) keeps the refusal.
- **An exhaustive per-route output policy,** `Record<SourceContract, …>` (or equivalent) that `tsc` checks for exhaustiveness and that names each surface. A new route fails to compile until it has an entry, and a route left unadmitted refuses.
- **Admission** only at `numerically_eligible` standing with the live native capture (`numericalResultStanding(result, model).eligible`). An ineligible successor refuses with a specific reason (`RETAINED_PRECISION_VALIDATION_REQUIRED` or `…NATIVE_CAPTURE_REQUIRED`, as the existing standing code names them).
- **Each panel gains one text-only summary line** from `classificationSummary(result, model)`: per case, "n verified only to an absolute bound; n uncovered" (D2 §4.9.9).
- **The refusal text** on the remaining surfaces is reworded so it no longer says every output is unavailable (decision 12). Display only; the existing assertions change only in the expected text.

## T6S-5: stress-neutral export (PLAN §1.3; decision 5, S-d)

- `buildStressNeutralExportPacket` and `validateStressNeutralExportPacket` gain the successor. They need:
  - the UTF-8 CSV policy (`usesUtf8Csv`);
  - the successor's semantic table path;
  - `contract_evidence`;
  - `retained_precision` copied whole;
  - `source_annotations`;
  - the receipt in the transport header (I67's F4);
  - transport validation through the existing `validateRetainedPrecisionTransport`.
- **Classified rows (S-d):** withhold each `absolute_verified` or `not_covered` row's unit-preservation witness with the dispositions `SN-UNIT-WITNESS-WITHHELD-RETAINED-PRECISION-ABSOLUTE-VERIFIED` and `…-NOT-COVERED`. They are info severity, carry D-U6-2's message text exactly as Rust's derivative states it, and are counted in the loss report's reason. The CSV row and its value stay unchanged.
- **The package must validate** under the existing schema's successor branch, with no schema change.

## T6S-4: result export (PLAN §1.2; decisions 4, 6, 11)

- `deriveResultDocument` and `validateResultDocument` gain Rust `derive_document`'s successor form:
  - `contract_evidence` copied;
  - `retained_precision` copied whole;
  - each `absolute_verified` or `not_covered` row disclosed, not valued, with D-U6-2's code and the exact Rust message (`RE/src/derivative.rs:38–61`).
- **Classes come from** the accepted reader run without an invocation (`validateRetainedPrecision(source)`), mirroring Rust `retained_row_classes`.
- **Validation mirrors Rust:** receipt equality, `RETAINED_PRECISION_DOWNGRADE_FORBIDDEN` for a receipt on any other identity, and exact class-code consistency.
- **b is formatted** as Rust's `{:e}` does: shortest round-trip digits, with `e+` normalized to `e` (CQ-1). Pin edge values: b ≥ 1 and a subnormal b.
- **Byte parity** with I76's Rust goldens for both pinned successors, using the same fixed desktop-shaped base and origin I76 records.
- **No standing token and no producer-origin claim** in the document. The desktop origin keeps `authentic_producer_available: false`.
- **The positive panel witness** uses test-built, hash-consistent manifest evidence, labelled in the test as such (CQ-11).

## Controls (T6S-4 and T6S-5 each)

- **Negative:** a copied successor; a moved model; an unregistered build's bytes; a load-reference route; a relabelled statement; a receipt on another identity; a forged class code; an edited disclosure message. Each refuses with its expected code.
- **Multi-case:** the corpus's synthetic two-case bases, read by id. `two_case_synthetic` is admitted when eligible; `two_case_facade_after_certificate_synthetic` refuses.
- **Mutants,** one set per new branch, each killed by an assertion: the gate admitting a non-panel surface; admission without the live capture; a dropped disclosure; a valued `absolute_verified` row; the `e+` exponent left unnormalized; a missing receipt in the transport header; a withheld-witness code with the wrong severity.
- **Unchanged routes:** every committed non-successor fixture exports byte-identically to the base `c1bfc460fc` through both builders.

## Suites

Run each on the base and the candidate, and compare them test by test:
- the desktop vitest suite in full;
- `tsc --noEmit` clean;
- the desktop lint the project's scripts define.

Explain every count change by an added test or a deliberate expectation change.

## Records

`NUM/R/I75/t6s_01/`: CHECKPOINT_1.md, then RETURN.md (changed files and hashes, controls, mutants, suite comparisons, anything to rule on) and SHA256SUMS.

## Budget

17–25 h across the checkpoint. Return at the checkpoint and at the end.
