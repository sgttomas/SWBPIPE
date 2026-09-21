# DEL-00-06 notes — Diagnostics, warning, and result-envelope contract

Wave W2, package PKG-00, worker G2. Forward ledger 42 rows (28 required keys,
7 `.rNN` rows for the split resolved-decisions and realized-artifacts blocks,
7 `.sNN` sub-claims). Reverse file 387 answers. Frozen state
`00115c71931bcae79909602d653740d3bb72dfa1`.

## Path aliases

- `AB` = `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-06_Diagnostics, warning, and result-envelope contract/ArchitectureBasis.md` (cited only in `NormativeSource`/`ContextRefs` because of spaces).
- Diagnostic definitions: `schemas/results.v0.2.schema.yaml` L104-L165 (class enum L122-L134, severity L136-L143); `schemas/analysis_run.v0.1.schema.json` L237-L294 (enum L255-L268). `schemas/analysis_run.schema.json` is an exact-version dispatcher since `6bb26b118` (2026-09-14).
- Solve-to-export crossing: `core/runner/headless/src/result_envelope_binding.rs` L159-L182 (`export_mechanics_diagnostic`); solve `Diagnostic` struct at `core/product_physics/src/lib.rs` L710-L719.

## Judgment calls

- **REQ-06-02 (possible defect, owner item).** The one boundary crossing located in code, solve diagnostic into the 0.2 result-export envelope, replaces the solve diagnostic's `affected_refs` with a reference to the diagnostic's own id and supplies a fixed remediation string. REQ-06-02 says every field other than class is always kept across a boundary. Disposed IMPLEMENTED_DIFFERENTLY · POSSIBLE_DEFECT · PROJECT_BASELINE · BASELINE · OWNER, MEDIUM: code is evidence, not authority, and whether this is a defect or an accepted mapping needs a decision. The TP-VERIFY-013E pending crossings were not traced further.
- **Additive-class anchor (.s01, .r03).** Both name `schemas/analysis_run.schema.json` as the enum's home; it became a dispatcher, the enum is in the versioned 0.1 file, and the 0.2 record carries diagnostics only as source annotations. CONTRACT_VERSION_ADVANCED, LOCAL_DESIGN, finding group FG-DEL-00-06-03. D-67 (adopted by reference) flagged on `.s01`.
- **Severity-taxonomy hold (.s01, CP-10).** Severity is fixed to info/warning/blocking in the result and analysis-record schemas with no located ruling; the code-namespace half stays open. OWNER.
- **Localization hold (.s02).** ALIGNED: SOW-061 still records it TBD and no mechanism exists; English-only strings are not a settled policy.
- REQ-06-01: `operation_outcome` diagnostics omit class and provenance; judged within "where applicable" for operation validation findings.
- REQ-06-03: breadth judged by schema presence and contract tests per layer (MEDIUM).
- `.r03` of resolved decisions cites "rev 0.9 §8.2": accurate provenance (§8.2 exists in both revisions).
- `.r02` carries a GAP_WORDING_CHECKED clause explaining that it records the 2026-05-17 classification, not later crossing conformance.

## Canonical departures

None. CS rows inherited unchanged; CP-02 and CP-10 applied as written.

## Convention friction

- The ScopeLedger/SOFTWARE_DECOMP SOW-061 note still says "exact schema syntax remains TBD" while DEC-010 adopted JSON Schema 2020-12; that text belongs to the decomposition, so it is noted on `.r03` and not disposed here.
- CP-02 `AuthorityNeeded NO` versus the manifest hash binding: same friction as DEL-00-05.

## UNKNOWN rows

None.

## Reverse pass

17 CONSTRAINS, 2 COVERS, 368 NOT_MINE. The reverse pass confirmed the `.s01`/`.r03` finding (RC-00-0103 builds the 0.2 record with source-annotation diagnostics) and did not change any sealed view.

## Batch consistency

Batch over the four G2 ledgers: PASS, 0 findings. The STATUS#history body shared with DEL-00-08 takes the same treatment.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
