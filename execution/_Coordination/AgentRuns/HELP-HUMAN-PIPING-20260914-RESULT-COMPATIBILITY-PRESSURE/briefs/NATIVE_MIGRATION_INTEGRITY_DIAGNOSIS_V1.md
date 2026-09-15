# Ephemeral Agent 2 — native migration integrity diagnosis V1

Role: bounded read-only generalist / Type 2 reporting to HELP_HUMAN. Continue the existing legacy_profile_refutation instance at gpt-6-astra/high. No delegation. This is an analytical integration diagnosis, not implementation, review of the complete candidate, or acceptance.

Resolve REPO_ROOT with git rev-parse --show-toplevel in the supplied integration checkout; WORKING_ROOT is REPO_ROOT/projects/chirality-piping. The supplied integration checkout is /Users/ryan/.codex/worktrees/8728/chirality-result-compatibility-pressure-20260914. Candidate HEAD is 6bb26b118fc038c451b97d3b86e3ae27d8ba8e91; read-only coordination evidence is being added without product changes.

## Purpose

Identify the smallest correct repair for a real first-save native integrity defect while preserving existing historical evidence and the unchanged model/persistence hash profiles and wire contracts.

The early actual native witness uses the maintained bundled 0.1 model. Native create/save applies the existing 0.1→0.2 model-document normalization, but persists the incoming pre-migration model and project-envelope hash claims unchanged. The migrated stored model has a different canonical hash. The migration ledger also names the old claim as post_migration_model_hash. App create/save handlers do not adopt the returned normalized model. Reopen truthfully reports model/envelope mismatches; a fresh solve/save then matches. New analysis 0.2, raw result and every row checksum match throughout. The early witness is BLOCKED despite successful preservation and fresh recovery.

Read the actual synthetic evidence under projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/NATIVE/EARLY_NATIVE_V1, especially store/after_first_save.row.json, validation/after_first_save_model_envelope_hashes.json, validation/after_fresh_solve_model_envelope_hashes.json and the source bindings. Do not read user models or normal application stores.

## Declared context

Read root/project AGENTS.md, current HELP_HUMAN/TASK instructions as relevant to this ephemeral non-delegating role; the approved tranche activation/briefs; D67; existing legacy preservation interpretation records; and the actual source paths:
- apps/desktop/src/App.tsx create/save/open and shared model replacement/currentness guards, plus related tests.
- apps/desktop/src/services/projectService.ts and tests, hashService.ts, HistoricalRunContext.tsx.
- apps/desktop/src-tauri/src/lib.rs prepare_model_document_for_persist, create/save/open and focused persistence/hash tests; model_document_migration.rs.
- Existing canonical JSON crate and operation-applier hash projections as necessary.
- Existing native/persistence DTOs and historical-preservation tests. No broad unrelated source read.

All listed product paths are WORKING_ROOT-relative. You may read the compatibility worktree current source only to identify relevant already pending changes; do not treat its unsealed source as an accepted candidate.

## Constraints

Preserve advertised historical hashes/labels as received. A new preservation or post-migration checksum cannot verify an old claim. An unchanged reopened save preserves historical result/analysis data. No missing historical input manifest may be fabricated. New-format Historical records remain Historical on reopen.

Model/persistence hash algorithms and projections are unchanged. No new model version, database/storage migration, transport command/request fields, operation API, cross-session history, product pressure activation or broad persistence redesign.

Any newly asserted checksum must correspond to its declared actual payload. A persistence response that changes the in-session model must not leave Current results or pending operations bound to the old basis. Preserve atomic/late-response/busy guards and the same operation semantics.

Do not propose suppressing truthful findings, silently overwriting incompatible history, or treating fresh recovery as verification of the failed first save. Distinguish exact original received metadata from newly computed evidence and say where each can live in the existing contracts.

## Outputs and permitted writes

Write only projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/MIGRATION_DIAGNOSIS_V1/**. Source, test, governance, Git and application state are read-only. No builds, GUI/browser control or product test execution; native is finishing its exclusive lease. Pure bounded analysis over the already captured synthetic bytes is permitted.

Return:
1. Reproduction/source trace and precise invariant violation.
2. Recommended minimal correct behavior for initial create/save that normalizes, unchanged Historical save, already migrated Historical open/save, and delayed responses.
3. Exact proposed product/test paths and a few meaningful regression triggers.
4. Any true contract decision needed from Agent 0, with recommendation and reasoning. Do not ask the user.
5. Source/context/output hashes and calibrated limitations.

No further orchestration layer. Root will issue a separate source release to the sole compatibility writer if a repair is accepted.
