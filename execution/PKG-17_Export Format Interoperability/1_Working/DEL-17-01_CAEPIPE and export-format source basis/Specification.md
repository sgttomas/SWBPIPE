# Specification: DEL-17-01 CAEPIPE and export-format source basis

## Normative Scope

DEL-17-01 shall define the source authority, target-behavior boundaries, unanswered-question register, and downstream consumption rules for PKG-17 export-format work.

This deliverable shall not implement exporters, parsers, harnesses, schemas, tests, GUI behavior, persistence behavior, or public API surfaces.

## Source Authority Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-01-REQ-001 | The deliverable shall use only public, official, project-owned, or otherwise admitted source references listed in `_REFERENCES.md` and `PKG-17/0_References/_REFERENCE_INDEX.md`. |
| DEL-17-01-REQ-002 | CAEPIPE-specific facts shall be grounded in official/public CAEPIPE documentation or explicitly marked `TBD`. |
| DEL-17-01-REQ-003 | PCF-specific facts shall distinguish documented translator behavior from OpenPipeStress export intentions. |
| DEL-17-01-REQ-004 | glTF/GLB facts shall be limited to review-geometry export semantics and shall not imply solver geometry equivalence. |
| DEL-17-01-REQ-005 | Later `DEL-17-*` deliverables shall consume this deliverable before making target-format claims. |

## Boundary Requirements

| Req ID | Requirement |
|---|---|
| DEL-17-01-REQ-010 | The source basis shall not include bundled CAEPIPE binaries, copied commercial examples, proprietary model files, reverse-engineered binary behavior, or license-bypass instructions. |
| DEL-17-01-REQ-011 | The source basis shall not include protected standards text, protected tables, proprietary formulas, standards-derived examples, material allowables, SIF/flexibility tables, or owner criteria. |
| DEL-17-01-REQ-012 | A CAEPIPE run, import, export, or parsed CSV result shall be described only as non-authoritative regression/handoff evidence unless a later human professional review process separately accepts it. |
| DEL-17-01-REQ-013 | Target solver options may be recorded as pass-through configuration, but shall not become OpenPipeStress local code-checking logic. |
| DEL-17-01-REQ-014 | Unknown or version-sensitive target behavior shall remain `TBD` and shall gate implementation claims in downstream deliverables. |

## Required Registers

| Register | Required Content |
|---|---|
| Source basis register | Source ID, source location, admitted use, boundary, downstream consumers. |
| Finding register | Source-grounded facts separated from assumptions and TBDs. |
| CAEPIPE question dossier | Questions for CAEPIPE developer/support clarification, current public evidence, affected deliverables, and gating impact. |
| TBD register | Unanswered target-format behavior, blocked deliverables, and closure route. |

## Downstream Consumption Rules

- `DEL-17-02` shall use this deliverable as the source authority for common export package/profile/ID-map/loss-report contracts.
- `DEL-17-04` shall not define a CAEPIPE MBF writer subset until its unsupported, omitted, approximated, delegated, and sidecar-mapped behaviors are traceable to this deliverable.
- `DEL-17-05` shall not treat CAEPIPE external runs as mandatory or bundled; the executable path and license remain user-owned.
- `DEL-17-07` shall treat PCF translation as conservative interoperability, not as the first deterministic validation backbone.
- `DEL-17-08` shall treat GLB/glTF as visual review geometry only.

## Acceptance Requirements

DEL-17-01 is acceptable when:

- the four-document kit exists and passes the local four-document check;
- source findings cite admitted source IDs;
- public facts, assumptions, and TBDs are separated;
- the CAEPIPE developer-team question dossier exists;
- no protected or proprietary source material is copied into the repository;
- no compatibility, release, code-compliance, or professional-acceptance claim is made.

