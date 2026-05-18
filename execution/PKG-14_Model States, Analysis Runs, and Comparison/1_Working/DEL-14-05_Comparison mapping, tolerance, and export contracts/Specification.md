# Specification: DEL-14-05 Comparison mapping, tolerance, and export contracts

## Scope

This deliverable defines API/data contracts for comparison mappings, unmatched classifications, tolerance profiles, and comparison exports for model-state and analysis-run comparisons.

In scope:

- comparison mapping schema;
- tolerance profile schema;
- comparison exporters for CSV, JSON, and report-section use;
- explicit `TBD` handling for unresolved tolerance defaults, mapping workflows, exact field names, and export layout.

Out of scope:

- implementation of the model-state comparison engine owned by DEL-14-03;
- implementation of the analysis-run comparison engine owned by DEL-14-04;
- comprehensive commercial prover result ingestion;
- external validation or professional approval;
- protected standards text, protected tables, proprietary values, private rule-pack payloads, or code-specific tolerance defaults.

Sources: `_CONTEXT.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md#DEL-14-05`; `execution/_Decomposition/SOFTWARE_DECOMP.md#PKG-14`; `docs/IP_AND_DATA_BOUNDARY.md#public-repository-must-not-contain`.

## Requirements

| Req ID | Requirement | Source |
|---|---|---|
| DEL-14-05-R001 | The contract shall support deterministic comparison of two model states and/or two analysis runs using stable IDs. | `execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-073` |
| DEL-14-05-R002 | The mapping schema shall represent manual mappings where automatic stable-ID alignment is insufficient. | `_CONTEXT.md#Description`; `execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-073` |
| DEL-14-05-R003 | The contract shall represent unmatched classifications for compared entities or results that lack an accepted counterpart. Specific classification values are TBD. | `_CONTEXT.md#Description`; `_CONTEXT.md#Anticipated-Artifacts` |
| DEL-14-05-R004 | The tolerance profile schema shall support unit-normalized result deltas and tolerance profiles without silently supplying default engineering values. Numeric defaults are TBD. | `execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-073`; `execution/_Decomposition/SOFTWARE_DECOMP.md#OI-014`; `docs/CONTRACT.md#OPS-K-DATA-2` |
| DEL-14-05-R005 | Export contracts shall preserve explicit unit and dimensional metadata for unit-bearing values or produce blocking diagnostics when required unit metadata is missing. | `docs/CONTRACT.md#OPS-K-UNIT-1`; `docs/SPEC.md#result-export-format` |
| DEL-14-05-R006 | JSON export semantics shall align with the schema-first result-envelope baseline. Exact deliverable-specific JSON fields are TBD. | `_CONTEXT.md#Architecture-Basis-Injection`; `docs/SPEC.md#result-export-format` |
| DEL-14-05-R007 | CSV export semantics shall be specified without embedding protected standards data, proprietary engineering values, private project data, or private rule-pack payloads. Exact columns are TBD. | `_CONTEXT.md#Description`; `docs/IP_AND_DATA_BOUNDARY.md#public-repository-must-not-contain`; `docs/SPEC.md#result-export-format` |
| DEL-14-05-R008 | Report-section export semantics shall preserve diagnostics, provenance, hashes or audit-manifest references, analysis statuses, limitations, and professional-boundary notices. Layout is TBD. | `docs/SPEC.md#result-export-format`; `docs/SPEC.md#report-boundary`; `docs/CONTRACT.md#OPS-K-REPORT-1` |
| DEL-14-05-R009 | The contracts shall not claim certification, sealing, approval, authentication, external validation, or code compliance. | `docs/CONTRACT.md#OPS-K-AUTH-1`; `execution/_Decomposition/SOFTWARE_DECOMP.md#SOW-073` |
| DEL-14-05-R010 | Public artifacts shall not bundle protected standards text, protected tables, code-specific acceptance criteria, proprietary values, or private user data. | `docs/CONTRACT.md#OPS-K-IP-1`; `docs/IP_AND_DATA_BOUNDARY.md#public-repository-must-not-contain` |
| DEL-14-05-R011 | The contract shall remain compatible with model-state records, analysis-run records, result export envelopes, and unit-system contracts identified as upstream dependency evidence. | `Dependencies.csv`; `_DEPENDENCIES.md` |

## Standards

| Standard or governance source | Applicability | Status |
|---|---|---|
| JSON Schema 2020-12 | Schema-first contract basis where JSON contracts are materialized. | Source: `_CONTEXT.md#Architecture-Basis-Injection`. |
| Chirality/OpenPipeStress lifecycle vocabulary | Deliverable state and draft/proposal status. | Source: `docs/TYPES.md#Lifecycle-states`; `docs/CONTRACT.md#OPS-K-AGENT-4`. |
| OpenPipeStress unit boundary | Unit-aware, dimensionally checked values and exports. | Source: `docs/CONTRACT.md#OPS-K-UNIT-1`; `docs/SPEC.md#unit-system-and-dimensional-analysis`. |
| OpenPipeStress IP/data boundary | Protected/private data exclusion for public artifacts. | Source: `docs/IP_AND_DATA_BOUNDARY.md`. |
| Professional responsibility boundary | Software output is decision support and not professional approval. | Source: `docs/CONTRACT.md#OPS-K-AUTH-1`; `docs/DIRECTIVE.md#Human-authority`. |

No external engineering standard text is locally provided as an authoritative source for this deliverable. Any code- or owner-standard-derived tolerance or acceptance rule is `TBD` unless later supplied through a governed private/user data path.

## Verification

| Requirement | Verification approach |
|---|---|
| DEL-14-05-R001 | Determinism tests comparing equivalent input states/runs once DEL-14-03 and DEL-14-04 interfaces exist. Current status: TBD. |
| DEL-14-05-R002 | Schema validation for manual mapping records, including stable source/target references and evidence/provenance fields. Current status: TBD. |
| DEL-14-05-R003 | Schema validation for unmatched classifications once enum values are human-approved or source-defined. Current status: TBD. |
| DEL-14-05-R004 | Unit-aware tolerance-profile validation with missing default values treated as explicit findings. Current status: TBD. |
| DEL-14-05-R005 | Unit/dimension validation for exported comparison values. Current status: TBD. |
| DEL-14-05-R006 | JSON Schema validation against the deliverable schema after the schema body is created. Current status: TBD. |
| DEL-14-05-R007 | CSV parse/round-trip checks after column set and ordering are defined. Current status: TBD. |
| DEL-14-05-R008 | Report-section fixture checks for provenance, diagnostics, hashes, limitations, and professional-boundary notices. Current status: TBD. |
| DEL-14-05-R009 | Protected wording/professional-claim review checking for prohibited approval/compliance language. Current status: TBD. |
| DEL-14-05-R010 | Protected-content/private-data lint or review gate. Current status: TBD. |
| DEL-14-05-R011 | Dependency closure review against the approved local DAG-002 mirror. Current status: mirror present; closure not independently reclassified by this deliverable. |

## Documentation

Required records for later implementation work:

- comparison mapping schema documentation;
- tolerance profile schema documentation;
- JSON export contract documentation;
- CSV export contract documentation;
- report-section export contract documentation;
- unresolved `TBD` register for tolerance defaults, mapping workflows, exact fields, and export layout;
- dependency evidence record preserving the approved DAG-002 mirror rows.
