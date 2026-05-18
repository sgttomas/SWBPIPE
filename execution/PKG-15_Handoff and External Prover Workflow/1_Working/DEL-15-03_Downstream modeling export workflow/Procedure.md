# Procedure: DEL-15-03 Downstream modeling export workflow

## Purpose

Define the conservative procedure for producing and validating the generic downstream modeling export workflow for DEL-15-03.

## Prerequisites

| Prerequisite | Source / status |
|---|---|
| Deliverable scope, package, artifacts, and architecture basis reviewed. | `_CONTEXT.md` |
| Invariants for data boundary, unit handling, professional boundary, and agent output reviewed. | `docs/CONTRACT.md` |
| Protected-content and public/private data boundary reviewed. | `docs/IP_AND_DATA_BOUNDARY.md` |
| Approved DAG-002 mirror rows preserved as ACTIVE evidence. | `_DEPENDENCIES.md`; `Dependencies.csv` |
| Upstream architecture-basis context available for DEL-00-01, DEL-00-02, DEL-00-03, DEL-00-04, DEL-00-06, DEL-00-07, and DEL-00-08. | Local `Dependencies.csv` |
| Upstream interop/handoff/security/model context identified for DEL-15-01, DEL-15-02, DEL-10-02, DEL-10-03, DEL-12-02, DEL-13-04, and DEL-14-05. | Local `Dependencies.csv` |

## Steps

1. Confirm the work remains inside DEL-15-03 and SOW-074. If implementation needs target-specific commercial parsers, commercial examples, professional approval states, or cross-package rewrites, stop and escalate.
2. Identify the upstream handoff package schema and target-mapping contracts available to the implementation. If they are not implementation-ready, record the missing inputs as TBD rather than inventing schema fields.
3. Define the generic exporter boundary: input model/result context, output handoff package envelope, validation result, warnings, unresolved assumptions, target mapping metadata, unsupported-target flags, and hash/provenance handling.
4. Implement the exporter using the accepted architecture basis applicable to this backend slice. Exact module paths and dependency versions are TBD until implementation starts.
5. Ensure exported payloads preserve unit metadata, entity IDs, references, diagnostics/warnings, assumptions, and hash metadata required by the upstream handoff package contract.
6. Add unsupported-target handling that records explicit findings or flags when the target fixture lacks a supported mapping. Do not silently drop or coerce unsupported behavior.
7. Create an invented target fixture. Record fixture provenance and protected-content review status; do not use commercial-tool examples or protected standards-derived examples.
8. Add export validation tests for schema compliance, unit manifest presence, required identity/reference fields, warnings/assumptions, unsupported-target flags, and professional-boundary wording.
9. Run applicable validation gates. At setup time, exact commands are TBD; expected gates include schema validation, test suite coverage for the exporter, and protected-content/provenance review.
10. Record unresolved assumptions, upstream contract gaps, unsupported features, and any dependency conflicts in deliverable-local evidence.

## Verification

| Check | Expected result |
|---|---|
| Scope check | Work remains a generic handoff export workflow for DEL-15-03. |
| Schema check | Output validates against the governing handoff package schema once available. |
| Unit check | Unit-bearing exported values carry explicit unit metadata or a documented dimensionless/TBD classification. |
| Warning/assumption check | Missing data and unsupported target behavior are explicit findings, not silent defaults. |
| Boundary check | Output does not claim certification, sealing, approval, authentication, professional acceptance, or code compliance. |
| Fixture check | Invented target fixture has provenance and does not use protected or commercial-tool example data. |
| Dependency check | Approved DAG-002 mirror rows remain preserved; no dependency extraction refresh reclassifies them without human approval. |

## Records

- Exporter implementation path: TBD.
- Export validation test path: TBD.
- Invented target fixture path and provenance: TBD.
- Handoff schema version or source: TBD until upstream contract is available.
- Target mapping taxonomy/source: TBD until upstream contract is available.
- Dependency validation evidence: `Dependencies.csv` schema validation output.
- Boundary review evidence: TBD.
