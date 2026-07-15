<!-- chirality-architecture-basis/v1 -->
# DEL-00-02 — Repository and module boundary architecture — Architecture Basis

Classification: `ARCHITECTURE_BASIS_REFERENCE`
Consolidates: `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md` (four-document kit, consolidated 2026-07-15 per piping decision D-43; source bytes hash-bound in the package `CONSOLIDATION_MANIFEST.md` and preserved in git history)
Authority basis: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9, architecture-basis row AB-00-02
Dependency direction: one-way — project packages depend on this member; this member depends on no package deliverable (HUMAN-STEER-PKG00-EXCLUSION-001)

## Purpose

Repository/module ownership boundaries and dependency direction: the layer map, module ownership table, dependency-direction rules, and adapter governance boundary that prevent later packages from making incompatible local choices about services, storage, diagnostics, GUI state, APIs, and acceptance gates. This member defines architecture contracts only; it does not itself create modules, source files, build scripts, package manifests, or implementation code. Identity: DEL-00-02, PKG-00 (Software Architecture Runway), type API_CONTRACT, scope items SOW-057 and SOW-062, objective OBJ-013.

The repository/module boundary kit and its executable gate now exist. Current upstream authority is `SOFTWARE_DECOMP.md` revision 0.9 with approved `DAG-007` graph coordination. PDU-007 retains the formal review hold on evidence sufficiency and module-boundary acceptance (see Open holds); this reference does not convert that hold into approval.

## Normative requirements

| ID | Requirement | Evidence |
|---|---|---|
| REQ-02-01 | Define the intended responsibilities for GUI, application services, domain core, solver, rules, reports, adapters, storage, schemas, validation, and tests. | Acceptance review |
| REQ-02-02 | Require dependencies to point inward toward domain contracts and not around unit, provenance, or data-boundary checks. | Acceptance review |
| REQ-02-03 | State that adapters and plugins may translate data but may not bypass governance, validation, or diagnostics contracts. | Acceptance review |
| REQ-02-04 | Record TBD implementation-layout decisions without inventing repository specifics. | Acceptance review |
| REQ-02-05 | Define review checks that detect package-local architecture drift before PKG-01 through PKG-17 proceed. | Human review |

REQ-02-01 through REQ-02-03 are satisfied by reference to the human-accepted AB-00-02 architecture-basis text in `SOFTWARE_DECOMP.md` (layer/module responsibilities for the eleven named layers; inward-pointing dependencies toward domain contracts; adapters/plugins translate but never bypass governance, validation, or diagnostics contracts).

Required invariants (from `docs/CONTRACT.md`): `OPS-K-IP-1` (no protected standards text, tables, figures, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data in public artifacts); `OPS-K-DATA-2` (missing solve-required or rule-check-required values remain explicit findings, never silent defaults); `OPS-K-AUTH-1` (no certification, sealing, approval, authentication, or engineering-code-compliance claims); `OPS-K-MECH-1` (global analysis remains a 3D centerline/frame model; local FEA is a handoff path); `OPS-K-AGENT-1` (unknown facts become `TBD`); `OPS-K-AGENT-3` (Type 2 execution stays within sealed deliverable scope).

Decision-handling rules (retained from Guidance):
- Record a choice as `TBD` when no cited human ruling exists; as `PROPOSAL` only when explicitly framed for review.
- Prefer explicit contracts over package-local assumptions; treat diagnostics, provenance, units, and data-boundary checks as cross-cutting architecture obligations.
- Preserve the distinction between mechanical calculation, user rule checking, and professional approval.

Boundary: this member authorizes no product implementation code and does not advance PKG-01 through PKG-17. Downstream consumers use AB-00-02 through sealed briefs, accepted context injection, or governed review/dispatch surfaces; consumption does not make PKG-00 `ISSUED`.

## Resolved decisions (former TBD and human-ruling queue)

| Former TBD / ruling item | Resolution | Authority |
|---|---|---|
| Implementation stack (precondition for layout/module-syntax choices) | Rust core/application services, Tauri 2 desktop shell, TypeScript/React/Vite GUI, Three.js viewport | DEC-009, `SOFTWARE_DECOMP.md` §12 (SCA-001 Gate 3, human-approved 2026-04-30) |
| Final repository layout names | Concrete `docs/architecture/module_boundaries.md` artifact permitted as a deferral; layout realized in practice by the implemented module tree (see Realized artifacts) | SOW-057 note ("concrete folder names may refine during implementation without changing layer responsibilities") + DEC-012 (SCA-001 TBD boundary); deferral adjudicated in D-41 W1 concordance (`NOTES_DEL-00-02.md`) |
| Storage substrate feeding the storage layer | MVP local storage profile: SQLite store/index substrate, canonical JSON/JCS domain truth, rebuildable retrieval sidecars | DEC-017, `SOFTWARE_DECOMP.md` §12 (SCA-003, 2026-05-17) |
| Solver module strategy | In-repo sparse skyline/profile direct solver, zero new numerical dependencies (D-03 Option C) | DEC-023, `SOFTWARE_DECOMP.md` §12 (ruled 2026-06-11) |
| Physical project package/container | Multi-member archive per PKG-17 manifest contracts (D-09 Option C), `.opsproj`; release matrix per D-06 Option O-A | DEC-028 and DEC-057, `SOFTWARE_DECOMP.md` §12 (supersedes the DEC-012 container portion) |
| Module-boundary decision recording | Governed through the ADR surface; accepted `ADR-0001_operation_seam_engine_unification.md` records the operation-seam/engine unification | ADR numbering convention per `docs/architecture/adr/index.md` |

## Realized artifacts

| Anticipated artifact | Realized as | Status |
|---|---|---|
| `docs/architecture/module_boundaries.md` | Never created — permitted deferral under the SOW-057 note and the DEC-012 TBD boundary; realized in practice by the implemented module tree: `core/`, `apps/desktop`, `api/`, `schemas/`, `tests/` | Deferred (permitted) |
| Architecture dependency rules | AB-00-02 dependency-direction and adapter-governance text in `SOFTWARE_DECOMP.md` (human-accepted); sibling `docs/architecture/*` contracts (owned by other PKG-00 members) | Exists by reference |

## Open holds and routed questions

- **D-41 R5 T6 PDU-007 formal-review hold on REQ-02-05.** The architecture review gate is defined, but the owning formal REVIEW disposition / evidence-sufficiency act has not occurred for `DEL-00-02-REQ-005`. Drift-detection review checks exist only as kit-hygiene / sufficiency-gate checks; no concrete check compares package-local choices against AB-00-02. PDU-007 therefore remains partially implemented at governance grain; the hold remains until the owning formal REVIEW disposition or evidence-sufficiency act occurs (mirrors `_STATUS.md ## Remaining`). No alignment, closure, readiness, acceptance, or issuance may be inferred. Evidence: `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T6-PDU007-HOLD.md`.
- **Routed-to-REVIEW boundary anomaly (record-hosting).** This documentation-only member's `_run_records/WORKING_ITEMS_RUN_2026-06-11_t3_wasm_enablement_*` files are the primary record for the TP-SEAM-WASM-001 module-boundary implementation work (wasm build seam: crate → browser asset; DEC-020 / ADR-0001), whose code landed outside this folder (`apps/desktop/...`, `core/model_operations/operation_applier`) via a separate Type 1 tranche. Routed `AuthorityNeeded=REVIEW` in the D-41 W1 concordance for whether hosting implementation run records under a doc-only architecture member is intended.
- **Still-open TBDs:** monorepo/package-manager selection and lint tooling have no cited human ruling and remain open under the DEC-012 boundary (kit `TBD and Human-Ruling Slots`); language-specific module syntax is constrained by the DEC-009 stack but has no dedicated ruling.

## Currency and provenance

Consolidated 2026-07-15 per piping decision D-43 from the four-document kit (`Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`) as reconciled by D-41 R5 T7 (PDU-054 currentness declarations, 2026-07-12): setup-era framing is folded into current accepted text, the package universe is stated as PKG-01 through PKG-17, and the authority pin is SOFTWARE_DECOMP revision 0.9. Prior wording is preserved in git history; `MEMORY.md` is retained unchanged. Lifecycle state remains governed by `_STATUS.md`; this reference document makes no lifecycle, review, acceptance, or issuance claim and does not lift the PDU-007 hold.
