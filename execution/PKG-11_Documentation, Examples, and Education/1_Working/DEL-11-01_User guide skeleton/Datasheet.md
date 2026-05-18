# Datasheet: DEL-11-01 User guide skeleton

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-11-01 |
| Deliverable name | User guide skeleton |
| Package ID | PKG-11 |
| Package name | Documentation, Examples, and Education |
| Deliverable type | DOC_UPDATE |
| Scope item | SOW-033 |
| Objectives | OBJ-001; OBJ-011 |
| Context envelope | M |
| Lifecycle artifact | Draft setup document, not the repository user guide |

## Attributes

| Attribute | Value |
|---|---|
| Primary purpose | Define a user guide structure for project setup, modeling, solving, rule checks, reports, and limitations. |
| Target artifact | `docs/user_guide/index.md` is the anticipated downstream artifact, but this setup run does not edit it. |
| Guide posture | Help users understand an open, auditable piping stress workflow without hiding missing data, private-data requirements, or professional responsibility limits. |
| Required sections | Product scope, setup, project creation, model building, solving, rule checks, results review, reports, limitations, troubleshooting, glossary. |
| Data boundary | Public guide content must not include protected standards text, protected examples, proprietary tables, private rule packs, private libraries, or owner data. |
| Rule-check boundary | User rule checks are computations using user-supplied or lawful private rule packs; they are not professional authentication. |
| Professional boundary | The software assists analysis and reporting; competent human review remains required before project reliance. |
| Implementation maturity | Exact packaging, public API transport, import/export formats, solver library, expression grammar, and physical project container remain `TBD`. |

## Conditions

| Condition | Source basis | Effect on the guide skeleton |
|---|---|---|
| User documentation is in scope for PKG-11. | `docs/_Registers/Deliverables.csv` row DEL-11-01; `docs/_Registers/ScopeLedger.csv` row SOW-033 | The skeleton covers the user-facing workflow, not source code or examples. |
| Open mechanics and protected standards data must remain separated. | `INIT.md`; `docs/DIRECTIVE.md` sections 1-4; OPS-K-IP-1 through OPS-K-IP-3 | Guide sections must describe user/private data requirements without copying or paraphrasing protected standards content. |
| Mechanics solve, user rule check, and professional approval are separate states. | `docs/TYPES.md` sections 4 and 6; OPS-K-AUTH-1 | The guide outline must explain statuses and warnings without implying automatic code compliance. |
| Primary global analysis is a 3D centerline/frame model. | `docs/DIRECTIVE.md` section 3; `docs/SPEC.md` sections 1 and 4 | Modeling sections emphasize centerline nodes, elements, components, supports, loads, and unit-aware fields. |
| Reports must be auditable and boundary-aware. | `docs/SPEC.md` section 8; OPS-K-REPORT-1 and OPS-K-AUTH-1 | Reporting sections include provenance, warnings, hashes/checksums, limitations, and human review notice. |
| Future implementation details are unresolved. | `_CONTEXT.md` Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 11 | Setup/install and advanced integration content uses `TBD` rather than inventing product behavior. |

## Construction

The user guide skeleton is organized as a documentation outline. Section content is intentionally skeletal until product features, UI behavior, reports, examples, packaging, and release gates mature.

| Guide section | Purpose | Initial state |
|---|---|---|
| Scope and authority boundary | Explain OpenPipeStress as decision-support software with open mechanics and user-supplied design-basis data. | Skeleton required |
| Installation and project setup | Reserve setup instructions for supported builds, local-first storage, project files, units, and privacy posture. | `TBD` where packaging is unresolved |
| Creating a project | Cover unit selection, project metadata, model identity, provenance expectations, and private rule-pack/library references. | Skeleton required |
| Building the centerline model | Cover nodes, elements, pipe runs, components, supports/restraints, load cases, combinations, and missing-data warnings. | Skeleton required |
| Solving mechanics | Explain mechanics-only solve status, solver diagnostics, singular/nonconverged states, and result envelopes. | Skeleton required |
| Running user rule checks | Explain private rule packs, required inputs, unit checks, missing rule data, checksums, and rule-check statuses. | Skeleton required |
| Reviewing results | Reserve structure for displacements, rotations, forces, moments, reactions, equipment loads, stresses, ratios, warnings, and assumptions. | Skeleton required |
| Generating reports | Cover input manifest, model hash, solver version, rule-pack checksum, provenance, limitations, and public-template prohibitions. | Skeleton required |
| Limitations and professional responsibility | State global centerline vs local FEA handoff limits, validation status, known risks, and human review requirements. | Skeleton required |
| Troubleshooting and warnings | Organize solve blockers, rule-check blockers, provenance warnings, assumption warnings, nonlinear warnings, and IP-boundary warnings. | Skeleton required |
| Glossary and status vocabulary | Define user-facing terms from `docs/TYPES.md` without creating new engineering authority. | Skeleton required |

## References

- `INIT.md`
- `AGENTS.md`
- `docs/README.md`
- `docs/DIRECTIVE.md`
- `docs/CONTRACT.md`
- `docs/TYPES.md`
- `docs/SPEC.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
- `docs/_Decomposition/SOFTWARE_DECOMP.md`
- `docs/_Registers/Deliverables.csv`
- `docs/_Registers/ScopeLedger.csv`
- `docs/_Registers/ContextBudgetQA.csv`
