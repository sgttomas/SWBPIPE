# Specification: DEL-11-03 Theory Notes - Classical to Modern Centerline Analysis

## Scope

This specification governs only the deliverable-local setup for `DEL-11-03` and the future production contract for `docs/theory/centerline_analysis.md`. It does not authorize edits outside the deliverable folder in this setup run.

The future theory note shall explain the classical flexibility lineage and modern 3D centerline/frame implementation approach for OpenPipeStress, while preserving protected-data, rule-check, professional-responsibility, and unit-awareness boundaries.

## Requirements

| ID | Requirement | Source |
|---|---|---|
| REQ-11-03-01 | The theory note shall explain centerline analysis as an educational mechanics concept and shall not assert code compliance, certification, sealing, approval, or professional reliance. | `docs/CONTRACT.md` `OPS-K-AUTH-1`; `INIT.md` |
| REQ-11-03-02 | The theory note shall distinguish mechanics solving from user-supplied rule checks and human professional acceptance. | `docs/CONTRACT.md` `OPS-K-MECH-2`, `OPS-K-DATA-1`; `INIT.md` |
| REQ-11-03-03 | The theory note shall frame routine global piping analysis as a 3D centerline/frame model and distinguish local shell/solid FEA as a handoff path. | `docs/CONTRACT.md` `OPS-K-MECH-1`; `INIT.md` |
| REQ-11-03-04 | The theory note shall use only public/permissive sources for historical and mechanics claims and shall cite source provenance. | `_CONTEXT.md`; `docs/CONTRACT.md` `OPS-K-IP-2` |
| REQ-11-03-05 | The theory note shall not copy or paraphrase protected standards text, examples, figures, tables, code-specific formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data. | `docs/CONTRACT.md` `OPS-K-IP-1`, `OPS-K-IP-3` |
| REQ-11-03-06 | Any technical quantities, units, or dimensional reasoning introduced later shall be unit-aware and shall not rely on silent defaults. | `docs/CONTRACT.md` `OPS-K-UNIT-1`, `OPS-K-DATA-2` |
| REQ-11-03-07 | Missing source support, exact source sections, and unresolved historical claims shall be marked `TBD` rather than invented. | `docs/CONTRACT.md` `OPS-K-AGENT-1`, `OPS-K-AGENT-2` |
| REQ-11-03-08 | Public examples, if any are introduced in future work, shall use invented non-code values and clear educational disclaimers. | `docs/CONTRACT.md` `OPS-K-RULE-1`; `SOW-033` notes |
| REQ-11-03-09 | The setup artifacts shall remain inside the assigned deliverable folder and shall not move content to `ISSUED`. | Human sealed brief; `AGENTS.md` dispatch rule |
| REQ-11-03-10 | Future citations shall carry at least source title, locator, source section, license or redistribution status, public/permissive disposition, and review notes. | `docs/CONTRACT.md` `OPS-K-IP-2`; `_SEMANTIC_LENSING.md` items `C-001` and `F-001` |

## Standards

No proprietary or protected standards text is incorporated by this setup run.

Applicable governing constraints are repository governance invariants, not engineering code clauses:

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-UNIT-1`
- `OPS-K-AUTH-1`
- `OPS-K-MECH-1`, `OPS-K-MECH-2`
- `OPS-K-AGENT-1..4`

Public/permissive theory sources for final prose are `TBD`.

## Verification

| Check | Method | Expected evidence |
|---|---|---|
| Four-document setup exists | File presence check | `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` present in this folder |
| Scope isolation | Path review | All created/modified setup artifacts remain under `DEL-11-03_Theory notes- classical to modern centerline analysis/` |
| Protected-data boundary | Content review | No copied standards formulas/examples/tables, material allowables, SIF/flexibility tables, or proprietary values |
| Non-certification boundary | Content review | No statement that software output certifies, seals, approves, authenticates, or declares engineering code compliance |
| Source gap handling | Content review | Unsupported public-source needs are marked `TBD` |
| Semantic setup | Artifact review | `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, and P3 run record exist and preserve lens-not-authority language |
| Dependency setup | Tool validation | `Dependencies.csv` passes v3.1 schema validation and `_DEPENDENCIES.md` counts match |
| Lifecycle gate | `_STATUS.md` review | `Current State` is `SEMANTIC_READY` only after setup artifacts and local validations pass |

Final theory-note coverage checklist for future production:

| Check | Required coverage |
|---|---|
| Scope and boundary | Educational purpose, public-source basis, no protected-data reproduction, and no compliance/certification claim. |
| Classical lineage | Only source-supported historical or conceptual statements; unsupported details remain `TBD`. |
| Centerline/frame abstraction | Conceptual explanation of nodes, elements, local/global frames, supports, loads, and mechanical results without protected code formulas. |
| Rule-check boundary | Clear separation between mechanics results, user rule checks, and human professional acceptance. |
| Limitations and FEA handoff | Distinction between routine global line-element analysis and local shell/solid FEA handoff. |
| References | Public/permissive sources with provenance and review disposition. |

Protected-content review checklist for future production:

| Content type | Required disposition |
|---|---|
| Standards text, figures, examples, and tables | Must not be copied or paraphrased into public prose. |
| Code-specific formulas, SIF/flexibility tables, and material allowables | Must not be included unless a future human-approved legal basis is recorded; current disposition is prohibited. |
| Proprietary commercial examples or benchmark data | Must not be included. |
| Invented examples | Permitted only when clearly non-code and non-engineering reliance. |
| Compliance or certification language | Must be rejected unless phrased as a user/human responsibility boundary. |

## Documentation

Required setup outputs for this run:

- `Datasheet.md`
- `Specification.md`
- `Guidance.md`
- `Procedure.md`
- `_SEMANTIC.md`
- `_SEMANTIC_LENSING.md`
- `Dependencies.csv`
- `_DEPENDENCIES.md`
- `_STATUS.md`
- `_run_records/*`

Future production output remains:

- `docs/theory/centerline_analysis.md` (not edited in this setup run)
