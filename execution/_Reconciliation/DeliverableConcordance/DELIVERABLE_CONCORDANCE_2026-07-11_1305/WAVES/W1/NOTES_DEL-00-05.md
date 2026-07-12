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

- ALIGNED — 11
- PARTIALLY_IMPLEMENTED — 1
- STALE_SETUP_SPECIFICATION — 4
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
- **DEL-00-05-DECL-001 / DEL-00-05-DECL-004** — STALE_SETUP_SPECIFICATION (HIGH),
  re-encoded at fan-in (see repair section). The rev-0.7-vs-0.8 authority-pointer
  drift is encoded to the STALE side per the W1 verifier's addendum-4
  adjudication; flagged because the encoding side is subject to the pending
  owner calibration on this corpus-wide pattern.

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
   for REQ-05-05 → REVIEWED (SCA-001 is codified in the current_basis
   decomposition and untouched by the D-40 lifecycle reset). REQ-05-04's
   property-inspector portion has no such ruling and is agent-authored pending
   disposition → UNVERIFIED. Prose declared-state rows are NOT_APPLICABLE per
   addendum 6 (this overrides the pre-addenda exemplar, which used REVIEWED on
   _STATUS). ACC/EXC rows: originally REVIEWED on the strength of the PKG00 lock
   review; re-encoded UNVERIFIED at fan-in (see repair section) — the lock
   review is an agent-authored recommend-only record, and the only human
   acceptance in its chain (the 2026-06-04 CHECKING move) was administratively
   reversed by D-40/DEC-072 on the codified ground of "absence of a current
   accepted basis," so no named human ruling/disposition currently covers the
   cited records at the frozen SHA.
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

## Fan-in repair (fable re-run)

W1 fan-in verification (`W1_VERIFICATION_PKG-00.md`) found this ledger
DEFECTIVE: the rev-0.7-vs-0.8 authority-pointer drift (identical generated kit
lines at Specification.md:32, Datasheet.md:34, Procedure.md:8) was entirely
uncaptured — no disposition and no note on DECL-001/DECL-004, and DECL-002's
existing STALE row omitted the rev-pointer element. Owner-ruled repair
protocol: a fable pilot re-runs the defective rows and owns the changed claims.

**Independent re-verification before re-encoding (all reads at frozen SHA
551f84ef6; porcelain empty before and after):**

- Specification.md line 32 carries "…SOFTWARE_DECOMP.md revision 0.7, the
  SCA-001/SCA-003/SCA-004 architecture-basis records…"; Datasheet.md line 34
  carries "…revision 0.7 for package and deliverable authority"; Procedure.md
  line 8 carries "…revision 0.7 is the current basis" — confirmed verbatim.
- Frozen `execution/_Decomposition/SOFTWARE_DECOMP.md` header: `revision: 0.8`,
  `status: current_basis`; §13 gate posture states downstream surfaces "may be
  stale relative to revision 0.8 until refreshed by their owning workflows" —
  confirmed. The verifier's addendum-4 adjudication (STALE side; sanction
  informs immateriality/AuthorityNeeded, not the controlled disposition) is
  supported by this evidence; no disagreement.
- PKG00 lock review (`PKG00_LOCK_REVIEW_2026-05-11_2218`): agent-authored,
  recommend-only ("This record is recommend-only"; lock rows effective "after
  human acceptance of this recommendation"). D-40/DEC-072 reversed the only
  human acceptance in its chain (2026-06-04 CHECKING move) citing "absence of
  a current accepted basis," preserving the review as historical evidence
  only. Addendum-6 ladder therefore resolves the ACC/EXC cells to UNVERIFIED
  (agent-generated evidence pending human disposition), matching the DEL-00-07
  encoding the verifier endorsed — harmonization supported; no disagreement.

**Rows changed (old → new):**

| ClaimID | Field | Old | New |
|---|---|---|---|
| DEL-00-05-DECL-001 | Disposition | ALIGNED | STALE_SETUP_SPECIFICATION |
| DEL-00-05-DECL-001 | RemainingWork | NONE_OBSERVED | rev-0.7→0.8 authority-pointer refresh (immaterial; decomp §13 sanction) |
| DEL-00-05-DECL-002 | DeclaredState/Evidence/RemainingWork | rev-pointer element absent | Required Source Basis rev-0.7 pointer added to the drift record and repair list (Disposition unchanged: STALE_SETUP_SPECIFICATION) |
| DEL-00-05-DECL-004 | Disposition | ALIGNED | STALE_SETUP_SPECIFICATION |
| DEL-00-05-DECL-004 | Confidence | MEDIUM | HIGH |
| DEL-00-05-ACC-001/002/003, EXC-001/002 | SourceReliability | REVIEWED | UNVERIFIED |

DECL-001/DECL-004 keep `AuthorityNeeded=NO` (refresh is sanctioned
owning-workflow work per decomp §13; consistent with the verified-SOUND
DEL-00-03/08 encodings). No disposition changed on any ACC/EXC row.

**Owner-calibration caveat (rev-drift rows DECL-001/DECL-002/DECL-004):** the
STALE side follows the W1 verifier's addendum-4 adjudication of a
corpus-wide pattern (every kit refreshed to 0.7). The verifier's report asks
the owner/orchestrator to calibrate the pattern before W2; if the owner
instead calibrates it to ALIGNED-with-note, DECL-001 and DECL-004 flip back to
ALIGNED (the drift facts now recorded in-row are correct under either ruling,
and DECL-002 remains STALE on its independent DEC-009 grounds). These
dispositions remain agent judgments, not owner or engineering rulings.

**Contract/hygiene:** header unchanged (20 columns, byte-for-byte); 16 rows;
RFC-4180 CRLF preserved; histograms above recounted from the re-encoded CSV.
Writes confined to this ledger's two W1 files. Frozen-tree porcelain empty
before and after all repair reads.
