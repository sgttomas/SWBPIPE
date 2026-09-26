# ROOT selection of the T0R design

HELP_HUMAN (ROOT), 2026-09-26, under the owner's delegated correctness authority.

**Selected:** [DESIGN.md](DESIGN.md) revision 3, with the references in `references.py` and `_run_records/references.stdout.txt` as frozen by this selection, together with the two binding amendments below. Implementation follows DESIGN.md and these amendments. Where they differ, the amendments win.

## Basis

- The first draft was independently reviewed ([REVIEW/RETURN.md](REVIEW/RETURN.md), BLOCKING). It was revised to [ROOT_RULINGS.md](ROOT_RULINGS.md).
- Revision 2 was backchecked ([REVIEW/BACKCHECK_R2.md](REVIEW/BACKCHECK_R2.md), FINDINGS, none blocking). It was revised again.
- Revision 3 got a narrow backcheck, recorded below. All mapped items are resolved, with one new should-fix (F-1).
- The references were refuted independently three times:
  - [S0](REFERENCE_CHECK/RETURN.md): 102 values;
  - [S0′](REFERENCE_CHECK/REVISION_2/RETURN.md): 257 values;
  - [S0″](REFERENCE_CHECK/REVISION_3/RETURN.md): 50 changed values, REFERENCES CONFIRMED.
- The owner decisions it relies on:
  - [SIF formulas](../OWNER_SIF_DECISION_2026-09-26.md);
  - the report-package outage until T6 (in ROOT_RULINGS.md);
  - [T1 decisions](../OWNER_T1_DECISIONS_2026-09-26.md).

## Narrow backcheck of revision 3 (recorded by ROOT)

The same fresh-context reviewer read the design read-only and ran one read-only probe. It confirmed that the following are resolved:

- **SF-A:** rendering happens at PP:1496, before the headlines, combinations and `HIGH_DISPLACEMENT_REVIEW`.
- **SF-B:** `rule_binding_refusal` covers every binding site. Both `src-tauri` binding paths go through `solver_result_row_value`, the only production caller of `rule_check_runner` is `src-tauri`, and headless has no solver-result binding.
- **SF-C and SF-D.**
- **SF-E:** a pre-0.4 request-value check that returns a blocked envelope, adds no enum variant and solves nothing.
- **Notes:** NOTE-1, 2, 5 and 6.
- **S0′ items:** 1 to 10, with the changed values re-derived by hand.

It found that the remaining T1 overlap in `lib.rs` is adjacent only, apart from two trivial same-line conflicts: the `LoadCaseSolve` early-return literals, and the statement placed before T1's `if source_selected` line. It also raised the following:

- **F-1 (SHOULD-FIX, new in revision 3).** §5.6's completeness check requires every diagnostic `affected_refs` entry to resolve to a row or a model entity. The product's existing non-blocking diagnostics carry other reference classes. The probe on the invented demo model found:
  - `SPRING_HANGER_USER_DATA_REVIEWED` and `CONSTANT_EFFORT_USER_DATA_REVIEWED` → `"hanger"`;
  - `NONLINEAR_SUPPORT_LOOP_CONVERGED` → `"DEC-046"`;
  - the intensification diagnostic → user source-reference strings.

  With the check as written, a correct envelope for any model with a hanger or a nonlinear support would be refused.
- **N-A (NOTE).** It is ambiguous whether the source-blocks-1 notice on Current exports is a document field or a UI or manifest label.
- **N-B (NOTE).** T1's uncommitted wave adds an early `needs_recompute` return for `LOAD_REFERENCE_SOURCE_ID` inside `numerical_use_standing_with_context`, a standing function ROOT had asked T1 to leave alone.

## Binding amendments

1. **F-1.**
   - The completeness check applies to result-namespace references: every `affected_refs` entry that begins with `result:`, and every summary `result_ref`. Those must resolve to an emitted row.
   - Other reference classes are allowed and are not resolved by the readers: model entity ids, field or category tokens, decision ids, and user source-reference strings.
   - The tamper tests change to match. A dangling `result:` reference is refused. The hanger, nonlinear and intensification diagnostics on the invented demo model are accepted, and a positive control proves it.
2. **N-A.** The source-blocks-1 notice on Current exports is a UI or export-manifest label, never a new field in an exported results or stress-neutral document. Existing source-blocks-1 export bytes, schemas and fixtures stay unchanged.

## On N-B (T1)

The early `needs_recompute` for `load-reference-source-1` is conservative and correct in meaning: the joined identity is not Current until T1's readers and qualification land. ROOT accepts it as a declared, minimal T1 edit, and has asked the T1 manager to declare it in T1's records. At the T0R/T1 merge, the two edits to the standing functions are combined by hand, and both sets of standing tests must pass.

## What happens next

A T0R WORKING_ITEMS manager implements slices S1 to S6 as one atomic PR, to [DESIGN.md](DESIGN.md) §10 and these amendments. The references are frozen: no implementation slice may change `references.py` or its output. A mismatch goes back to ROOT. The PR needs:

- a complete-diff independent review;
- hosted CI, including the full dual-viewport dispatch for surface 4;
- a clean DEC-025 sweep.

The native witness on the owner's Mac is recorded as outstanding if it is not yet available. Containment in T0R closes no finding group.

## Record custody

At integration, ROOT renamed the design probe's `_run_records/design_probe/Cargo.toml` and `src/main.rs` to `.txt`, so that tools that discover Cargo manifests cannot treat this record as a crate. The bytes are unchanged, and `_run_records/SHA256SUMS` was updated to the new names and verifies. To rebuild the probe, drop the suffixes. DESIGN.md is byte-identical to the backchecked revision 3.
