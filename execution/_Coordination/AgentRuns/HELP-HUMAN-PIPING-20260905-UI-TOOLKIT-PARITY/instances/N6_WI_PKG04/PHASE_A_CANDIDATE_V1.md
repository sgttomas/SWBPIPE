# PKG-04 row 21 bounded boundary-authoring candidate v1

Status: DESIGN_CANDIDATE; derivative coordination evidence, not accepted decomposition truth.
RunID: HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY
InstanceID: N6_WI_PKG04; Parent: HELP_HUMAN; Role: WORKING_ITEMS Agent 1.
Scope: PKG-04 / DEL-04-03 / SOW-011 / OBJ-003 / OUT-001 / AC-001 / VER-001.
Authority: parent launch plus user instruction transcribed in run OWNER_DIRECTION.md; SCA-009 row 21; decomposition revision 0.12; approved DAG-010 pointer. Basis file hashes are in BASIS_HASHES.json. No lifecycle, acceptance, receipt, or Git closeout act.
Paths derive REPO_ROOT with git rev-parse --show-toplevel and WORKING_ROOT={REPO_ROOT}/projects/chirality-piping.

## Intake and ownership

Read root/project AGENTS.md, AGENT_WORKING_ITEMS.md, software-workflow.json, SOFTWARE_WORKFLOW_PROFILE.md, DEL-04-03 ScopeOfWork/context/status/references, SCA-009 annex, product support emission, linear support mechanics and analysis boundary schema. DEL-04-03 is SOW_V1, currently IN_PROGRESS with no Remaining entry. The absence of a Remaining entry is not row21 completion. The authoritative target includes explicit node DOFs, linear springs, rigid constraints and imposed displacement mechanics; it explicitly excludes arbitrary coordinate-policy extensions. Local historical context still names decomposition 0.7; run consumes the newer accepted parent basis without rewriting those surfaces.

DEL-02-03 analysis_boundary.schema.yaml separates solver, rule-check and human-acceptance authority. It does not define equipment physical boundary equations. No new physics is derived from its name. OPS-K-MECH-1/2, OPS-K-DATA-2, OPS-K-UNIT-1, OPS-K-SOLVER-1 and OPS-K-IP-1 constrain this candidate.

## Proposed existing-mechanics interface

Offer an Equipment/nozzle boundary form targeting one existing node. Require user label/equipment-nozzle reference and provenance, show **Global model axes** explicitly, and require explicit classification for Ux/Uy/Uz/Rx/Ry/Rz: Free, Rigid, or Linear spring. No default anchor, guessed stiffness, default nozzle load or limit is populated. Require at least one constrained/spring DOF; an all-free selection performs no mutation and explains that no restraint was created.

Build one atomic, previewable standard operation batch: one create_support record holding all rigid DOFs (omit if none); one family=spring support per spring DOF (omit free DOFs). Deterministic stable IDs are based on a user-entered boundary ID plus rigid or DOF suffix, checked for collision. Generated records retain label, node and explicit provenance identifying the equipment/nozzle reference and global DOF. This is a grouped authoring convenience over support records, not a novel solver SupportFamily. The model tree remains able to select and edit each created support record. Existing operation validation, references, canonical hash, atomic apply and undo are reused by both human and agent interfaces.

Spring values are strictly positive, finite, user supplied quantities. Ux/Uy/Uz use linear_stiffness (force/length); Rx/Ry/Rz use rotational_stiffness (moment/angle). Retain entered unit/value and require catalog-backed dimensional conversion at the current preview boundary. A zero elastic value is not silently changed to free. Reject duplicate conflicting DOF modes within the form and warn/block existing rigid overlap at the node as supported by operation validation. Multiple independent elastic supports at a node currently add stiffness; do not claim the form automatically consolidates pre-existing equipment restraints.

Underlying preview shape is one PreviewSupport with optional stiffness:{dof,value} per spring. product_physics build maps each spring to LinearSupport::spring; assembled preview adds it to the global diagonal. Rigid records map explicit DOFs to Anchor/Guide families. This bounds the feature to global, uncoupled linear compliance at the pipe node; it does not model equipment geometry, local shell flexibility, local coordinate bases, coupled 6x6 stiffness, or allowable load checks.

Imposed equipment movement may use the existing explicit displacement primitive/load-case workflow and remain separately authored. Do not claim static Support records currently implement imposed movement: PreviewSupport does not contain it even though the linear_supports crate supports prescribed-displacement boundary data. Integrating equipment movement into the composite form would require a separately traced PKG-05 contract and validation; not necessary for this first bounded rigid/spring capability.

## Concrete seams and minimum implementation fence

- core/solver/linear_supports/src/lib.rs: SupportFamily, NodeDof/global_index, prepare_boundary and apply_linear_supports already provide required equations. **No solver source change is expected.**
- core/product_physics/src/lib.rs: PreviewSupport/SupportStiffnessInput and support emission around lines 312–332 and 3144–3208 already support single-DOF translational or rotational springs and explicit rigid DOFs. Verify serializer ingestion by the next PKG-16/07 wave; do not create a competing shape.
- PKG-16 owns create_support payload schema, rotational stiffness creation/field parity, multi-record atomic command integration and model validation. Current resolve_create_support only expressly admits linear_stiffness metadata, so present operation support must not be assumed complete.
- PKG-07 owns boundary form and operation draft builder. Source overlap belongs to that manager, not PKG-04.
- PKG-04 supplies mechanics contract and a focused original-fixture test/evidence if parent later authorizes source work; any product/source diff requires fresh read-only TASK+software-code-review over 100% frozen diff before sweep/push.
- Do not change PCF unsupported-nozzle claims: this internal support authoring does not establish PCF nozzle import/export semantics.

## Acceptance checks for downstream implementation

1. Rigid, mixed rigid+spring and rotational-spring forms produce exactly the expected standard support operations with explicit node/DOF/provenance, no unrequested fields and no alternate mutation route.
2. One global Ux translational spring and one global Rz rotational spring at the same node add only their respective stiffness diagonals; all free DOFs remain free. Existing linear_supports tests already cover rotational dimension rejection and diagonal addition; product test must cover submitted operation-to-preview adapter.
3. Missing/invalid units, negative/nonfinite/zero stiffness, unknown node, ID collision and conflicting within-form DOFs reject without partial mutation. Canonical preview/applied hashes and undo roundtrip cover the complete batch.
4. Global axes are visible in authoring and preview; local axes cannot be selected or silently treated as global. Human and agent payloads use identical validation.
5. Existing ordinary support/hanger/nonlinear cases remain unaffected. No new stiffness assumptions or equipment/nozzle compliance claim appears.

## Work graph and handoff

Posture MIXED; plan selection authority AGENT_0. A1 manager intake (this record, source read-only) -> A2 independent refutation after parent confirms slot -> parent contract disposition -> serialized PKG16/07 implementation only after existing-capability wave accepted -> fresh source review and registered checks. Shared product source ownership stays with PKG16/07. Allowed writes for this instance are this instance directory and a candidate-only PKG04 record; no product files, status/pointers, governed registers, receipts, commits or pushes.

Phase A evidence is not independently reviewed yet. No tests were run because this is a read-only design phase; findings derive from cited source/spec reads, not executed test claims. Model attribution: inherited runtime (specific model unavailable to this child); native descent role enforcement is instruction-asserted, not mechanism-proven. Child dispatch held pending parent slot confirmation; no simulated child completion.

Closure verdict: CANDIDATE_READY_FOR_PARENT_REVIEW, not row21/package closed. Authoritative decomposition remains unchanged; derivative packages are not regenerated for this design-only phase. No true engineering decision blocks the bounded global rigid/diagonal-spring form. Arbitrary local frames, coupled stiffness and equipment-specific allowable checks would be a future engineering-contract decision; they are not quietly implemented. Remaining blockers: independent refutation, PKG16 operation shape acceptance, PKG07 UI acceptance, existing-capability predecessor. Rerun if support payload, coordinate semantics, operation adapter or accepted scope changes. Next owner HELP_HUMAN routes contract to PKG16/07 and authorizes the serial source wave.
