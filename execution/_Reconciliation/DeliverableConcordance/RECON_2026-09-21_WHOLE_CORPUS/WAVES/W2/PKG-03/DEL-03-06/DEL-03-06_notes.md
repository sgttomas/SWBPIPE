# DEL-03-06 — worker notes (W2, PKG-03, group G2)

Expansion joint component model. Forward ledger: 73 rows (71 required keys, 2
`.sNN` sub-claims; no optional `.rNN` block split). Frozen state `00115c719`.
Agent judgments only; nothing here is an owner ruling.

## Path aliases

- Project-root tokens resolve under `projects/chirality-piping/`; project
  documents are cited as `projects/chirality-piping/…`.
- SOW-STAGE2 parity/claim-map records are repository-root AgentRuns paths.
- Tests are cited as `<file>::<test name>`.
- Deliverable-local records sit under a folder name with spaces and commas;
  they are cited in `ContextRefs` and described in Notes.
- Suite-level pass status: `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json`
  (not rerun; A6).

## Judgment calls

1. **SOW SURFACE → CP-04** (rename residue): CLM-025 says "preserving
   OpenPipeStress boundaries" in current voice after SCA-010. Recorded once on
   the SURFACE row (C1); CLM-025 is judged on its substance (`ALIGNED`). CP-04
   names `STALE_REVIEW_OR_EVIDENCE`; I kept it although the text's origin is
   7bee9ae41 (F3 would otherwise say STALE_SETUP), because the pattern row
   fixes the class and the residue arose from the later rename.
2. **Per-axis stiffness and solver-mapping TBDs overtaken (FG-DEL-03-06-01).**
   DEC-045 (2026-06-20) chose a dedicated EJ stiffness macro-element consuming
   only user-entered axial/lateral/angular/torsional stiffness, with pressure
   thrust on the load side; the 2026-06-21/22 D4 tranches implemented it in
   product physics (tested: macro-element review rows, pressure-thrust load
   evidence). Rows that keep the per-axis mapping or solver semantics as TBD
   (CLM-004, CLM-005, CLM-013, CLM-027, CLM-028) or limit evidence to the
   schema slice (CLM-007, CLM-011, CLM-015/R-008) are `STALE_REVIEW_OR_EVIDENCE
   · DOC_BEHIND_CODE` (origins 1b62eb5b8). Movement-limit and hardware
   taxonomy TBDs still hold. AC-001's "unresolved solver mappings" stays true
   of the component-library contract itself, so it is `ALIGNED`.
3. **PDU-055 declarations (CLM-002, CLM-010, CLM-018, FG-DEL-03-06-02)** →
   CP-02 fields (pins SOFTWARE_DECOMP 0.8 and DAG-007). Their delegation of
   residuals to `## Remaining` is not relied on (A4); that section is empty
   while live TBDs remain in the SOW. `STATUS#remaining` is pre-typed
   NON_NORMATIVE and left `NOT_ASSESSED`.
4. **R-008 → PARTIALLY_IMPLEMENTED · PROJECT_BASELINE · BASELINE · REVIEW**,
   consistent with DEL-03-05 R07 (basis AB-00-02/07). R-005 is `ALIGNED`
   because product validation and the applier check EJ units (applier dimension
   rejection is tested) and the claim names no branch-style test demand.
5. **Parity:** no PASS record matches the frozen SOW → matrix OUT-001 and
   VER-001 `STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN` (CP-09).
6. **STATUS → CP-05** (Last Updated 2026-07-12 older than the 2026-07-16
   history entry).
7. **CONTEXT** treated as in DEL-03-04/05; Context Envelope note is pure sizing
   → NON_NORMATIVE.
8. **CLM-020 → CP-01, STALE_SETUP_SPECIFICATION** (names Datasheet.md,
   Specification.md, Guidance.md; origin 7bee9ae41). CLM-023 records list has
   no four-document residue.

## Canonical departures

None. CS rows inherited unchanged.

## Convention friction

- CP-04's fixed class versus F3's origin test (judgment call 1).
- The no-spaces rule keeps run records out of evidence columns; they are in
  `ContextRefs`.

## Smallest checks for UNKNOWN rows

None; this ledger has no `UNKNOWN` row.

## Reverse pass

376 capabilities answered: PARTIAL 3 (RC-03-0073, RC-03-0109, RC-03-0294),
COVERS 4 (RC-03-0072, RC-03-0107, RC-03-0167, RC-03-0321), NOT_MINE 369. The
generic caller-supplied macro-element (RC-03-0075) is answered NOT_MINE with a
specific reason: this deliverable supplies stiffness data, and its SOW places
solver interpretation with downstream solver deliverables. The reverse pass
did not change my view of any sealed row.

## Batch consistency

`--batch` over DEL-03-04, DEL-03-05 and DEL-03-06: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9): since
2026-09-19 Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
