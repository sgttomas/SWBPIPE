# MEMORY: DEL-17-01

## 2026-05-18 - SCA-004 PREPARATION scaffold
- Created minimum viable fileset from SOFTWARE_DECOMP revision 0.7.
- Populated local context, dependencies placeholder, references, semantic placeholder, and status.
- Production documents, semantic lensing, dependency extraction, implementation, release claims, and professional claims remain unperformed.

## 2026-05-18 - TP-EXPORT-001 source-basis population
- Populated the four-document kit: `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md`.
- Added deliverable-local evidence artifacts: `Source_Basis_Register.md` and `CAEPIPE_Question_Dossier.md`.
- Used only admitted public/official/project-owned references from `_REFERENCES.md` and `PKG-17/0_References/_REFERENCE_INDEX.md`.
- Remaining TBDs: CAEPIPE target version/profile, first MBF subset, MBF stable-ID carrying strategy, CSV parser subset, conservative PCF subset, and GLB/glTF identity sidecar policy.
- Boundary exclusions preserved: no proprietary examples, no protected standards data, no bundled commercial solver, no license-bypass guidance, no CAEPIPE compatibility overclaim, no release claim, and no code-compliance or professional-acceptance claim.

## 2026-05-18 - TP-EXPORT-003 canonical semantic remediation
- Replaced the PREPARATION `_SEMANTIC.md` placeholder with a `semantic-matrix-build` style matrix artifact.
- Added `_SEMANTIC_LENSING.md` using the setup-time `lens-register` pattern and applied warranted P3 enrichment to `Procedure.md`.
- Created `Dependencies.csv` v3.1 and updated `_DEPENDENCIES.md`.
- Updated `_STATUS.md` to `SEMANTIC_READY` as local document/semantic readiness only.
- Remaining TBDs are unchanged: CAEPIPE version/profile, first MBF record subset, MBF stable ID carrying, CSV result section stability, conservative PCF subset, and GLB/glTF review identity policy.

## 2026-05-18 - TP-EXPORT-004R sealed semantic/lensing/P3 rerun
- Regenerated `_SEMANTIC.md` as the deliverable-specific full canonical semantic matrix set: A, B, C, F, D, K, G, X, T, and E with interpretation work and audit PASS.
- Regenerated `_SEMANTIC_LENSING.md` with complete A/B/C/F/D/X/E lens coverage over the four documents and three warranted items: `C-001`, `D-001`, and `E-001`.
- Applied only warranted P3 items to `Datasheet.md` and `Procedure.md`; `Specification.md` and `Guidance.md` were reviewed and left unchanged.
- Added three run records under `_run_records/` for semantic matrix generation, lens-register generation, and four-documents `P3_ONLY`.
- `_STATUS.md` remains `SEMANTIC_READY`; semantic step appended a verification history entry and P3 made no status transition.
- Validation results after the sealed sequence: `validate_semantic_matrix.py` PASS, `validate_lens_register.py` PASS, `check_four_documents.sh` PASS, `check_min_viable_fileset.sh` PASS, `validate_dependencies_schema.py Dependencies.csv` PASS, and `git diff --check` PASS.
- Remaining TBDs are unchanged: CAEPIPE version/profile, first MBF record subset, MBF stable ID carrying strategy, CSV result section stability, conservative PCF subset, and GLB/glTF review identity policy.
- Boundary exclusions preserved: no proprietary examples, no protected standards data, no bundled commercial solver, no license-bypass guidance, no CAEPIPE compatibility overclaim, no release claim, and no code-compliance or professional-acceptance claim.
- Boundary exclusions for this TASK worker: no coordination files, validators, schemas, code, DAG/blocker queues, dependency files, or other deliverable folders were edited.

## 2026-05-18 - TP-EXPORT-004R parent fan-in review
- Parent reviewed sealed TASK outputs without authoring semantic or lensing content inline.
- Fan-in validation passed for `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, four-document kit, minimum viable fileset, `Dependencies.csv`, validator pytest coverage, validator py_compile, and `git diff --check`.
- Boundary scan found only negative guardrail language and no affirmative CAEPIPE compatibility, release, code-compliance, professional-acceptance, protected-data, proprietary-data, or reverse-engineering claim.

## 2026-05-23 - DEL-17-01 source-basis refinement
- Decisions/assumptions: treated DAG-005 as accepted graph-coordination foundation only; treated PKG-02 as accepted foundation if encountered; made no lifecycle, candidate-promotion, implementation, release, code-compliance, professional, or validation claim.
- Source pointers used: CAEPIPE public MBF import/export/batch/export-data pages; CAEPIPE PCF translator PDF; Khronos glTF 2.0 specification; `docs/CONTRACT.md` provenance and claim-strength invariants; `docs/IP_AND_DATA_BOUNDARY.md` public/private and protected-content policy; `docs/SPEC.md` deliverable/provenance/lifecycle sections; `docs/TYPES.md` Type 2 and TBD epistemic sections; `execution/_DAG/DAG-005/APPROVAL_RECORD.md` approval-condition and follow-up boundaries.
- Changed files: `Source_Basis_Register.md`, `CAEPIPE_Question_Dossier.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `MEMORY.md`, and `_run_records/TASK_RUN_2026-05-23_1408_DEL-17-01_source-basis.md`.
- Unresolved questions/TBDs: CAEPIPE version/profile and citation target; first MBF record-family subset; MBF stable-ID carrier versus sidecar-only policy; CSV result sections stable enough for DEL-17-05/DEL-17-06 parser scope; conservative PCF subset and translator-default warning rules; GLB/glTF review-geometry identity metadata and sidecar policy. `PLAN-EXPORT-INTEROP` section locations remain `location TBD` where the run did not narrow the plan heading.
- Downstream handoff: DEL-17-02 should consume the explicit protected/private boundary and stable-ID TBD; DEL-17-04 should consume MBF evidence and carry all version/subset/ID TBDs; DEL-17-05 and DEL-17-06 should carry CSV/batch harness TBDs without treating parsed results as professional acceptance; later PKG-17 exporters should cite admitted source IDs and report exported, omitted, approximated, delegated, unsupported, and TBD behavior.

## 2026-05-23 - Validation scope clarification
- Human ruling applied: blockers outside `/Users/ryan/ai-env/projects/chirality/projects/chirality-piping` and outside the approved deliverable write scope are bypassed as external-scope noise for this project tranche.
- Updated `Procedure.md` to make scoped `git diff --check -- <deliverable path>` the controlling diff-hygiene evidence.
- No outside-scope files were edited.
