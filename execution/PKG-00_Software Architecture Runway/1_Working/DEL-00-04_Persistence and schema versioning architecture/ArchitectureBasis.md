<!-- chirality-architecture-basis/v1 -->
# DEL-00-04 — Persistence and schema versioning architecture — Architecture Basis

Classification: `ARCHITECTURE_BASIS_REFERENCE`
Consolidates: `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md` (four-document kit, consolidated 2026-07-15 per piping decision D-43; source bytes hash-bound in the package `CONSOLIDATION_MANIFEST.md` and preserved in git history)
Authority basis: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9, architecture-basis row AB-00-04
Dependency direction: one-way — project packages depend on this member; this member depends on no package deliverable (HUMAN-STEER-PKG00-EXCLUSION-001)

## Purpose

Defines the persistence, schema versioning, migration, canonicalization, and deterministic round-trip architecture for project files and persisted artifacts. This member is architecture-basis context: it constrains persistence contracts across the package universe (PKG-01 through PKG-17); it does not itself own schemas, migrations, serializers, database tables, or project-file readers. Scope item SOW-059; objective OBJ-013; type DATA_MODEL_CHANGE.

Per the D-41 R5 T7 PDU-055 current declaration, persistence versioning, migration, canonical round-trip behavior, and storage-boundary evidence now exist in the implemented slice; provider expansion and any still-recorded policy choices remain residual work.

## Normative requirements

| ID | Requirement |
|---|---|
| REQ-04-01 | Define persistence as deterministic, versioned, unit-aware, and provenance-preserving. |
| REQ-04-02 | Require every persisted artifact to declare schema version and migration status. |
| REQ-04-03 | Define canonicalization as a prerequisite for reproducible hashes and audit manifests (concrete algorithm now resolved — see Resolved decisions). |
| REQ-04-04 | Require round-trip checks for models, units, loads, rule-pack references, diagnostics, and provenance metadata. |
| REQ-04-05 | Separate public schemas from private/user-owned code data and proprietary libraries. |

Cross-cutting invariants carried from the kit: `OPS-K-IP-1` (no protected standards/code text, tables, formulas, or proprietary data in public artifacts), `OPS-K-DATA-2` (missing solve/rule-check values remain explicit findings, never silent defaults), `OPS-K-AUTH-1` (no certification, sealing, approval, or code-compliance claims for reliance), `OPS-K-MECH-1` (global analysis stays a 3D centerline/frame model; local FEA is a handoff path), `OPS-K-AGENT-1` (unknown facts become TBD), `OPS-K-AGENT-3` (Type 2 execution stays within sealed scope).

Guardrails: prefer explicit contracts over package-local assumptions; treat diagnostics, provenance, units, and data-boundary checks as cross-cutting obligations; software and agents must not claim code compliance or professional approval.

## Resolved decisions (former TBD and human-ruling queue)

| Former open item | Resolution | Authority |
|---|---|---|
| Physical project file format / storage backend | Local SQLite-backed project store/index; canonical JSON/JCS remains domain truth; rebuildable SQLite FTS5/BM25 retrieval sidecars; large files referenced in place by path/URI plus hash/metadata; no hosted DB, daemon, required network, cloud sync, telemetry, or direct plugin/adapter SQL access. | SCA-003; DEC-017 in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; AB-00-04 |
| Schema language | JSON Schema 2020-12 as the public schema/interchange baseline, with schema-first command/query/job/result envelopes. | SCA-001; DEC-010 in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 |
| Canonical hash algorithm | SHA-256 over JCS-like sorted-key canonical JSON, implemented in `apps/desktop/src/services/hashService.ts`; the envelope hash excludes the volatile storage summary and its own carrier field so it is recomputable from a restored envelope. | `execution/_Coordination/_DECISIONS/D-08_model_document_schema_migration.md` §2.7 |
| Migration framework / schema-version and migration policy | Two-track versioning: store schema via the SQLite `user_version` ledger; model documents via per-document semver with an explicit registered transform chain; migrate-in-memory-on-open with persist-on-save; explicit-operation evidence guarantees with no silent destructive rewrites. Refined by DEC-033: additive shape changes (new optional fields) bump the per-document minor version; patch is reserved for non-shape changes; breaking changes remain major/explicit-migration events. | D-08 packet; DEC-019 (2026-06-10) and DEC-033 (2026-06-12) in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 |

## Realized artifacts

The anticipated `docs/architecture/persistence_versioning.md` was never created. Its function was absorbed by:

| Artifact | Role | Ownership |
|---|---|---|
| `docs/architecture/persistence_contract.md` | Canonical persistence contract document | DEL-02-05 (not DEL-00-04) |
| `execution/_Coordination/_DECISIONS/D-08_model_document_schema_migration.md` | Migration-policy decision packet (accepted as proposed 2026-06-10) | Decision record (coordination surface) |
| `apps/desktop/src-tauri/src/model_document_migration.rs` | Implemented model-document migration chain | Implementation packages (not DEL-00-04) |
| `apps/desktop/src/services/hashService.ts` | Canonical-JSON SHA-256 hashing implementation (REQ-04-03) | Implementation packages (not DEL-00-04) |

## Open holds and routed questions

- Provider expansion and any still-recorded policy choices remain residual work per the PDU-055 declaration; surviving deliverable-local residuals are those recorded in `_STATUS.md ## Remaining` (currently empty) and the D-41 residual records.
- No former kit TBD (format, schema language, migration framework, hashing, storage backend) remains without a cited ruling.

## Currency and provenance

Consolidated 2026-07-15 per piping decision D-43 from the four-document kit (`Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`) as reconciled by D-41 R5 T7 (PDU-054/PDU-055 currentness declarations, 2026-07-12; PDU-055 cited claims `DEL-00-04-DECL-002`, `DEL-00-04-DECL-003`). Prior wording, including superseded setup-era framing and the revision 0.7 authority pins, is preserved in git history. `MEMORY.md` is retained unchanged as the dated deliverable memory. This document is reference context, not a production contract, and carries no lifecycle, review, release, professional-reliance, or code-compliance claim.
