# WORKING_ITEMS Run Record - TP-PMM-P4-ABSENTDOMAINS-001

Date: 2026-07-10
Agent: WORKING_ITEMS (bounded Type-2 documentation worker; owner-adopted tranche)
Deliverable: PKG-11 package level (see run-record placement note below)
Package: PKG-11 - Documentation, Examples, and Education
Tranche: TP-PMM-P4-ABSENTDOMAINS-001
Target stage: physical-model mechanics program P4 documentation tranche
(`plans/PLAN_2026-07-09_physical_model_mechanics.md`; ruling `DEC-069`,
packet `execution/_Coordination/_DECISIONS/D-37_RULING_2026-07-09.md`;
codified in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12)

## Scope

Execute the D-37/DEC-069 ruling: name the nine currently
absent-and-unacknowledged special analysis domains on the PRD non-goals
surface so their absence reads as a recorded scope decision rather than an
omission. Docs-only; wording chosen in-tranche per the delegation in the
ruling. The nine domains (authoritative slate from DEC-069): buried pipe /
soil springs; jacketed pipe; FRP / orthotropic materials; slug / two-phase
forces; snubbers; fatigue cycle counting; first-class nozzle-flexibility
objects; mitered-bend and stepped-reducer formulations; expansion-joint
hardware kinematics (tie rod / hinge / gimbal) beyond metadata.

## Files Touched

- `docs/PRD.md` §5 Non-Goals: appended a lead-in sentence citing DEC-069
  (D-37) plus nine bullets in the existing "will **not**" bullet style, one
  per domain. No other section of the file edited.
- `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` §7 Non-Goals: mirrored the
  same lead-in and the identical nine domain statements as numbered items
  11-19 in that section's existing "shall not" numbered style. No other
  section of the file edited.
- This run record (new).

## Implemented Evidence

- Wording decisions (in-tranche delegation):
  - All nine domains placed on the non-goals surface only; the Open
    Questions sections (v0.1 §25, v0.2 §27) were NOT used. Rationale:
    listing an item simultaneously as a non-goal and an open question would
    read as contradictory, and the lead-in sentence already records that
    naming "neither schedules nor forecloses future work", which carries
    the open-question-shaped nuance for items such as first-class
    nozzle-flexibility objects and fatigue cycle counting.
  - The expansion-joint bullet says the hardware KINEMATICS are not
    modeled "beyond the configuration metadata already specified for
    expansion joints", so it does not contradict v0.1 §11.3.6 ("Tie
    rod/hinge/gimbal/limit rod configuration metadata") or the v0.2 §12.3
    Expansion Joints metadata list. The phrase avoids a hard section
    number because the metadata list is §11.3.6 in v0.1 but §12.3 in v0.2,
    which lets the domain statements stay textually identical across both
    surfaces.
- Surface choice (recorded per tranche instruction; subject to owner
  review at merge): the D-37 ruling literally names `docs/PRD.md`, but
  that file's Forward Authority Note delegates forward authority to
  `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` (SCA-005/D-21/DEC-056);
  because the ruling's intent is that "absence reads as a recorded
  decision", the naming was mirrored with identical domain wording onto
  the forward-authority surface as well, so the recorded decision is
  visible where forward work actually reads scope.
- Run-record placement: no single PKG-11 deliverable owns `docs/PRD.md`
  (it is a project-level document; DEL-11-01..05 own guides, theory notes,
  examples, and onboarding). The package-level home
  `PKG-11_.../1_Working/_run_records/` has precedent for cross-deliverable
  documentation records (`TASK_RUN_2026-05-16_PKG02_DOWNSTREAM_AUDIT.md`),
  so this record is placed there.

## Checks

- `PYTHONDONTWRITEBYTECODE=1 python3 tools/practitioner_harness/harness.py
  self-check` from repo root: exit 0.
- Practitioner-harness pytest (`tools/practitioner_harness/`): pass.
- No cargo build/test and no DEC-025 sweep: docs-only tranche per
  `plans/PLAN_2026-07-09_physical_model_mechanics.md` §3 (sweep is for
  code tranches).

## Boundaries And Residuals

- Naming these domains is a scope statement only. No PRD
  functional-requirement change was made (no FR table touched in either
  PRD surface); promotion of any named domain into scope follows the
  governed SCOPE_CHANGE/SCA path (D-21/SCA precedent).
- The mirroring onto the v0.2 forward-authority surface is an in-tranche
  surface-choice interpretation of the ruling's intent and is flagged for
  owner review at merge.
- No lifecycle transition, no release-readiness, professional,
  certification, or code-compliance claim.
