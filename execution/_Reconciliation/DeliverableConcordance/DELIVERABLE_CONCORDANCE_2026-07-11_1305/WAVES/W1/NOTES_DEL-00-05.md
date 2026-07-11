# NOTES — DEL-00-05 "GUI state and interaction architecture" (PKG-00, W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305 · R2 wave W1 · frozen SHA
551f84ef6be656f1603ce0acfa5e3935aa9683c7. Ledger:
`CLAIM_CONCORDANCE_DEL-00-05.csv` (16 rows, 20 columns, RFC-4180 clean).

## Deliverable character (orients every disposition)

DEL-00-05 is an **architecture-documentation-only** PKG-00 runway deliverable
(Type `UX_UI_SLICE`). It produces a four-document kit + semantic artifacts; it
owns **no implementation, schema, or test surface of its own** (Datasheet Scope
Boundary; Specification "No implementation code … are created"). This is the key
contrast with the GUI exemplar DEL-07-05, whose Specification's setup prose was
overtaken by an *owned* implemented slice. Here there is no owned slice, so the
Specification/Procedure/_STATUS/MEMORY declared-state surfaces are **not** stale
merely because a React desktop app exists downstream — that app is owned by
PKG-07 deliverables, outside DEL-00-05's orbit.

The requirement rows (REQ-05-01..05) are therefore **documentation
obligations**, and the architecture *basis* behind them was human-selected by
**SCA-001** (2026-04-30; ScopeLedger SOW-060 note: "durable/transient split,
service-command mutation route, and scoped undo/redo; exact state library
remains TBD") and classified **LOCK_AS_BASIS** by the PKG00 lock review
(`execution/_Reconciliation/PKG00LockReview/PKG00_LOCK_REVIEW_2026-05-11_2218`,
row RECONCILIATION-PKG00-05). Current lifecycle is IN_PROGRESS after the D-40
administrative rebaseline (CHECKING → IN_PROGRESS; "not an invalidation of prior
work").

## Requirement-ID scheme mapping (REQ-05-* → addendum-12 ClaimID)

Non-self-identifying scheme. Specification requirement IDs map one-to-one:
- REQ-05-01 → DEL-00-05-REQ-001 (state separation)
- REQ-05-02 → DEL-00-05-REQ-002 (service-command mutation route)
- REQ-05-03 → DEL-00-05-REQ-003 (scoped undo/redo + diagnostics preservation)
- REQ-05-04 → DEL-00-05-REQ-004 (selection + property-inspector, framework-agnostic)
- REQ-05-05 → DEL-00-05-REQ-005 (GUI warning-class preservation)

Run-level NormativeSource alias (addendum 12): `Specification.md` =
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-05_GUI state and
interaction architecture/Specification.md` (and siblings for the other kit
files), all at the frozen SHA.

## Disposition histogram (reproduces from the CSV)

- ALIGNED — 13
- PARTIALLY_IMPLEMENTED — 1
- STALE_SETUP_SPECIFICATION — 2
- Total — 16

## ClaimType histogram (reproduces from the CSV)

- REQUIREMENT — 5
- ACCEPTANCE — 3
- EXCLUSION — 2
- DECLARED_STATE — 6
- REMAINING_WORK — 0
- IMPLEMENTED_UNMAPPED — 0
- Total — 16

Census notes: no REMAINING_WORK rows — the only `## Remaining` entry is the
seeded `(gated: D-41)` bootstrap item, recorded verbatim in the _STATUS surface
row's `RecordedRemaining` and excluded from all residual/gate/selectability
analysis (addendum 2). No IMPLEMENTED_UNMAPPED rows — DEL-00-05 has no code
orbit; the desktop GUI-state implementation is PKG-07-owned and out of scope
(addendum 8). No deliverable-owned README exists (no README declared-state row).
`SelectableUnderCurrentLoop=NO` on every row (no non-bootstrap residual;
addendum 12 default).

## Principal finding: framework/viewport TBDs overtaken by DEC-009

The Datasheet "TBD and Human-Ruling Slots" and the Guidance "Human-Ruling Queue"
still present **GUI framework** and **viewport engine/technology** as open
human-ruling items. **DEC-009** (human-approved 2026-04-30; SOFTWARE_DECOMP.md
§12) adopted React (GUI framework) and Three.js (viewport), realized at the
frozen SHA in `apps/desktop/package.json` (react ^19.0.0, three ^0.181.0). This
is post-decision declaration drift → `STALE_SETUP_SPECIFICATION` on both surface
rows (DECL-002 Datasheet, DECL-003 Guidance), routed `AuthorityNeeded=OWNER` as
R5 repair candidates. The **state-management library, undo/redo storage, and
accessibility** TBDs remain genuinely open (no dedicated state-mgmt dependency in
apps/desktop; consistent with DEC-012's remaining-TBD boundary and the lock
review's "exact component/state-management library remains intentionally
unresolved"). The staleness is partial and named per-item, not blanket.

Secondary gap folded into DECL-002: the Datasheet/_CONTEXT "Anticipated
Artifacts" `docs/architecture/gui_state_model.md` was **never produced**
(`docs/architecture/` at the frozen SHA contains no such file); the kit + semantic
files are the deliverable's only outputs.

## Self-flagged rows

- **DEL-00-05-REQ-003** — ALIGNED (MEDIUM). Judgment call: SCA-001 decides and
  the lock review confirms "scoped undo/redo," but REQ-05-03's specific clause
  "preserve diagnostics when an edit changes solve readiness" is not elaborated
  as an explicit undo/redo rule — only implied by Guidance's general
  cross-cutting-diagnostics principle. Reviewer may prefer PARTIALLY_IMPLEMENTED.
- **DEL-00-05-REQ-004** — PARTIALLY_IMPLEMENTED (MEDIUM), AuthorityNeeded=OWNER.
  Judgment call: the kit defines a "selection model" role but contains **no**
  property-inspector architecture content, while the requirement names both.
  Property-inspector is the subject of DEL-07-02, so this could instead read as
  an ALIGNED delegation if the owner confirms inspector-behavior architecture is
  delegated out of DEL-00-05. Routed to OWNER for the scope boundary.
- **DEL-00-05-REQ-005** — ALIGNED (MEDIUM). Judgment call: DEL-00-05 states the
  GUI warning-*preservation* principle, but the enumerated six warning classes
  live in DEL-00-06 (SOW-061) and are not enumerated/cross-referenced here.
  Treated as present-but-thin (ALIGNED) rather than absent; reviewer may prefer a
  cross-reference requirement.
- **DEL-00-05-DECL-002 / DEL-00-05-DECL-003** — STALE_SETUP_SPECIFICATION (HIGH).
  Judgment call flagged for visibility: the framework/viewport decision (DEC-009)
  is owned by DEL-00-01 (SOW-056), not DEL-00-05; the staleness is DEL-00-05's
  *un-updated deferral*, not a wrong decision. Confident it is drift (the app is
  built on React/Three.js) but the cross-deliverable ownership is worth a
  reviewer's eye.
- **DEL-00-05-ACC-003** — ALIGNED (MEDIUM). Judgment call: read the "TBD visible
  and routed to human ruling" acceptance criterion at production grain (were the
  TBDs surfaced and routed, not silently chosen — yes). The subsequent
  non-update after DEC-009 is dispositioned as staleness on the surface rows, not
  as an acceptance failure here, to avoid double-counting.

## Evidence-execution log

- **Re-executed:** none. DEL-00-05 has no runnable surface (no code, schemas, or
  tests) — all evidence is direct read-only inspection of the frozen tree plus
  citation of recorded records. No build/test tooling was invoked, so no
  `CARGO_TARGET_DIR`/`PYTHONDONTWRITEBYTECODE` redirection was needed for
  discovery reads (the CSV-builder script ran only in the working run folder, not
  the frozen tree).
- **Cited-as-recorded:** P3 consistency sweep (_run_records/TASK_RUN_2026-04-30_0930.md);
  SEMANTIC_READY setup (_0910/_0920); PKG00 lock review (2026-05-11); D-40 ruling
  (2026-07-11); SCA-001 Decision_Log + SOFTWARE_DECOMP §12 (DEC-008/009/012);
  ScopeLedger SOW-060 row.
- **Frozen-tree porcelain:** `git -C <frozen> status --porcelain` empty both
  before and after all reads (see boundary statement).
- H4 note: the GUI-class H4 desktop evidence posture (Playwright e2e / Vitest,
  SMOKE.md-only exception) does **not** attach to this deliverable — it governs
  GUI *implementation* deliverables. DEL-00-05 is documentation; its
  "no automated tests" state is by nature, not a recorded exception.

## Convention-friction notes

1. **Documentation-deliverable requirement rows.** Conventions 1/§7 are written
   for implemented behavioral claims; here "implementation evidence" is
   architecture prose and "verification" is document review. I applied the
   substance dispositions (ALIGNED/PARTIALLY_IMPLEMENTED) to the documentation
   obligations and kept staleness on the declared-state surface rows only
   (convention 1). Flagging that the ALIGNED/PARTIAL boundary for a *thin but
   present* architecture obligation (REQ-05-03/05) is a genuine judgment the
   convention set does not sharply resolve.
2. **SourceReliability for basis-backed requirement rows.** Addendum 6 gives
   REVIEWED only with a named human ruling covering the record. SCA-001's gates
   (DEC-008/012, human-approved) cover REQ-05-01/02/03 and the diagnostics basis
   for REQ-05-05 → REVIEWED. REQ-05-04's property-inspector portion has no such
   ruling and is agent-authored pending disposition → UNVERIFIED. Prose
   declared-state rows are NOT_APPLICABLE per addendum 6 (this overrides the
   pre-addenda exemplar, which used REVIEWED on _STATUS).
3. **Cross-deliverable staleness ownership.** The framework/viewport TBD
   staleness originates from a decision (DEC-009) owned by another deliverable
   (DEL-00-01). Convention 1 reserves STALE for declared-state rows; I applied it
   to DEL-00-05's own Datasheet/Guidance surfaces (which carry the stale prose)
   and routed the repair to OWNER — no cross-project or cross-deliverable edit is
   proposed (fences).

## Boundary-compliance statement

- All fences held. Discovery was read-only; writes are confined to the two W1
  output files (`CLAIM_CONCORDANCE_DEL-00-05.csv`, `NOTES_DEL-00-05.md`) under the
  working run folder. No frozen-tree write (even git-ignored); no lifecycle
  transition (`STALE_SETUP_SPECIFICATION` recorded as a disposition, never
  applied; no `LIFECYCLE_REASSESSMENT_REQUIRED` used); no DAG mutation; no
  cross-project or cross-deliverable edit; no `_STATUS.md`/register/product edit.
- No release-readiness, issuance, certification, sealing, professional-approval,
  or code-compliance claim anywhere (F-PIP-1..5). All dispositions are agent
  judgments; authority is routed via `AuthorityNeeded` (OWNER on the three R5/
  scope-boundary rows), never phrased as owner or engineering rulings.
- Frozen-tree porcelain clean before and after (empty `git status --porcelain`).
