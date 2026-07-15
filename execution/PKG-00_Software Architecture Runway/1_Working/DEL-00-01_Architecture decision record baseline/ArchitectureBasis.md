<!-- chirality-architecture-basis/v1 -->
# DEL-00-01 — Architecture decision record baseline — Architecture Basis

Classification: `ARCHITECTURE_BASIS_REFERENCE`
Consolidates: `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md` (four-document kit, consolidated 2026-07-15 per piping decision D-43; source bytes hash-bound in the package `CONSOLIDATION_MANIFEST.md` and preserved in git history)
Authority basis: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9, architecture-basis row AB-00-01
Dependency direction: one-way — project packages depend on this member; this member depends on no package deliverable (HUMAN-STEER-PKG00-EXCLUSION-001)

## Purpose

Decision surface for stack, runtime, GUI framework, solver-library, and packaging target choices. This member holds the decision structure — decision inventory, ADR template, human-ruling queue, and reconsideration triggers — that lets later packages consume architecture choices as cited rulings instead of package-local assumptions. It records decision structure and required evidence; the choices themselves are made by human ruling and codified in `SOFTWARE_DECOMP.md` §12 and the ADR surface. Identity: DEL-00-01, PKG-00 (Software Architecture Runway), type DOC_UPDATE, scope item SOW-056, objective OBJ-013.

The ADR index, template, and accepted baseline now exist (see Realized artifacts). Current upstream authority is `SOFTWARE_DECOMP.md` revision 0.9 with approved `DAG-007` graph coordination; later architecture decisions remain governed through the ADR process.

## Normative requirements

| ID | Requirement | Evidence |
|---|---|---|
| REQ-01-01 | Define an ADR index structure that can list proposed, accepted, superseded, and rejected decisions. | Acceptance review |
| REQ-01-02 | Define an ADR template with fields for context, options, decision status, consequences, source references, and human authority. | Acceptance review |
| REQ-01-03 | Record stack/runtime/framework/library/package choices as TBD unless a human ruling is cited. | Acceptance review |
| REQ-01-04 | Require every accepted architecture decision to identify affected packages and reconsideration triggers. | Acceptance review |
| REQ-01-05 | Keep all architecture decisions code-neutral and free of protected standards data. | Human review |

Required invariants (from `docs/CONTRACT.md`): `OPS-K-IP-1` (no protected standards text, tables, figures, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data in public artifacts); `OPS-K-DATA-2` (missing solve-required or rule-check-required values remain explicit findings, never silent defaults); `OPS-K-AUTH-1` (no certification, sealing, approval, authentication, or engineering-code-compliance claims); `OPS-K-MECH-1` (global analysis remains a 3D centerline/frame model; local FEA is a handoff path); `OPS-K-AGENT-1` (unknown facts become `TBD`); `OPS-K-AGENT-3` (Type 2 execution stays within sealed deliverable scope).

Decision-handling rules (retained from Guidance):
- Record a choice as `TBD` when no cited human ruling exists; as `PROPOSAL` only when explicitly framed for review.
- Do not convert a proposed architecture option into an accepted decision inside this member.
- Every accepted decision cites its human ruling; downstream packages cite the accepted architecture document or note the pending ruling.
- Treat diagnostics, provenance, units, and data-boundary checks as cross-cutting architecture obligations; preserve the distinction between mechanical calculation, user rule checking, and professional approval.

Boundary: this member authorizes no product implementation code and does not advance PKG-01 through PKG-17. Architecture outputs remain draft/proposal material until accepted by the human project authority. Downstream consumers use AB-00-01 through sealed briefs, accepted context injection, or governed review/dispatch surfaces; consumption does not make PKG-00 `ISSUED`.

## Resolved decisions (former TBD and human-ruling queue)

| Former TBD / ruling item | Resolution | Authority |
|---|---|---|
| Implementation stack and runtime | Rust core/application services, Tauri 2 desktop shell, TypeScript/React/Vite GUI, Three.js viewport | DEC-009, `SOFTWARE_DECOMP.md` §12 (SCA-001 Gate 3, human-approved 2026-04-30) |
| GUI framework | TypeScript/React/Vite with Three.js viewport (same ruling as stack) | DEC-009, `SOFTWARE_DECOMP.md` §12 |
| Numerical solver library | In-repo sparse skyline/profile direct solver, zero new numerical dependencies (D-03 Option C) | DEC-023, `SOFTWARE_DECOMP.md` §12 (ruled 2026-06-11); live-path adoption timing DEC-050 |
| Packaging targets | Physical container: multi-member archive package per PKG-17 manifest contracts (D-09 Option C), extension `.opsproj`; release matrix/installers/publication: D-06 Option O-A with both riders | DEC-028 and DEC-057, `SOFTWARE_DECOMP.md` §12 (ruled 2026-06-11 and 2026-07-04) |
| Storage format / persistence | MVP local storage profile: SQLite local store/index substrate, canonical JSON/JCS as domain truth, rebuildable FTS5 sidecars, no hosted DB or telemetry path | DEC-017, `SOFTWARE_DECOMP.md` §12 (SCA-003, 2026-05-17) |
| ADR numbering sequence beyond the baseline template | `ADR-NNNN` four-digit zero-padded from `ADR-0001`, one file per ADR named `ADR-NNNN_<slug>.md` | Numbering convention applied in `docs/architecture/adr/index.md` |

## Realized artifacts

| Anticipated artifact | Realized as | Status |
|---|---|---|
| `docs/architecture/adr/index.md` | `docs/architecture/adr/index.md` (four-state index: proposed/accepted/superseded/rejected, per REQ-01-01) | Exists |
| `docs/architecture/adr/template.md` | `docs/architecture/adr/template.md` (REQ-01-02 fields) | Exists |
| ADR baseline in use | `docs/architecture/adr/ADR-0001_operation_seam_engine_unification.md` (accepted) | Exists |

## Open holds and routed questions

- Exact dependency versions, component/state libraries, rule expression grammar/library, public API transport, CI provider, and coverage thresholds remain implementation-level TBDs under the DEC-012 boundary unless a sealed brief or later human ruling resolves them (DEC-009 codification note; DEC-012).
- Signing/notarization re-decision is deferred to register row D-06b (`NOT_PREPARED`); v0.1 ships unsigned per DEC-057.
- Licensing constraints for numerical library policy: DEC-023 selected a zero-dependency in-repo solver, mooting third-party solver licensing for the current basis; broader dependency-licensing policy has no dedicated ruling and remains open under DEC-012.

## Currency and provenance

Consolidated 2026-07-15 per piping decision D-43 from the four-document kit (`Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`) as reconciled by D-41 R5 T7 (PDU-054 currentness declarations, 2026-07-12): setup-era framing is folded into current accepted text, the package universe is stated as PKG-01 through PKG-17, and the authority pin is SOFTWARE_DECOMP revision 0.9. Prior wording is preserved in git history; `MEMORY.md` is retained unchanged. Lifecycle state remains governed by `_STATUS.md`; this reference document makes no lifecycle, review, acceptance, or issuance claim.
