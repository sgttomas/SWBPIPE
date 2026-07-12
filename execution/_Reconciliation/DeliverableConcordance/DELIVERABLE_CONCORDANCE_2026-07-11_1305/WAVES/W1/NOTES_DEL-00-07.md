# Notes — DEL-00-07 API boundary and adapter contract map (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen evidence tree
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`WAVES/W1/CLAIM_CONCORDANCE_DEL-00-07.csv` (16 rows).

## Deliverable character (orientation)

DEL-00-07 is a PKG-00 Software Architecture Runway deliverable of type
`API_CONTRACT`. It is **documentation-only**: its declared scope explicitly
excludes implementing APIs, plugins, importers/exporters, schemas, tests, or
GUI. Its "implementation" surface is the four-document kit plus the semantic
artifacts. Lifecycle is `IN_PROGRESS` following the 2026-07-11 D-40
administrative rebaseline (CHECKING -> IN_PROGRESS, DEC-072). No product code
is authored here; the API-boundary/adapter architecture basis it defines
(SCA-001 / AB-00-07) was realized downstream by other deliverables.

**Requirement ID scheme mapping (addendum 12 / dispatch note):** the scheme is
`REQ-07-*` and is **non-self-identifying** — the `07` denotes the deliverable
index (DEL-00-**07**) within PKG-00, not PKG-07. Requirement IDs REQ-07-01..05
are disambiguated in the ledger by the fixed ClaimID form
`DEL-00-07-<TYPE>-NNN` (e.g. REQ-07-01 -> `DEL-00-07-REQ-001`). Confirmed
against `DELIVERABLE_INVENTORY.csv` (RequirementIDScheme `REQ-07-*`,
RequirementIDs `REQ-07-01;...;REQ-07-05`).

**Run-level NormativeSource alias (addendum 12):** paths in the ledger are
relative to `projects/chirality-piping/` at the frozen SHA unless otherwise
qualified.

## 1. Histograms (recount from the CSV)

Disposition histogram (16 rows):
- ALIGNED — 12
- PARTIALLY_IMPLEMENTED — 1
- STALE_SETUP_SPECIFICATION — 3

ClaimType histogram (16 rows):
- REQUIREMENT — 5
- ACCEPTANCE — 3
- EXCLUSION — 2
- DECLARED_STATE — 6
- REMAINING_WORK — 0
- IMPLEMENTED_UNMAPPED — 0

Both reproduce exactly from the ledger (verified by re-parsing the CSV).

## Census derivation (why these rows and no others)

- **REQUIREMENT (5):** one per current requirement ID REQ-07-01..05
  (convention 1 substance disposition; never STALE_SETUP_SPECIFICATION).
- **ACCEPTANCE (3):** the Specification carries no ID'd verification table —
  only an `## Acceptance Criteria` bullet list. Per the addendum-12 grain rule
  (do not mirror requirement restatements), I created acceptance rows only for
  the bullets that add distinct, verifiable acceptance content: document-kit
  presence (bullet 1 -> ACC-001), semantic-artifact presence and non-authority
  (bullet 6 -> ACC-002), and the protected-data / no-compliance boundary
  (bullet 5 + OPS-K-IP-1/OPS-K-AUTH-1 -> ACC-003). Bullets 2, 3, 4 restate the
  cross-package exclusion (EXC-002), REQ-07-04 (TBD visibility), and the
  no-implementation exclusion (EXC-001) respectively, so they were folded, not
  mirrored. This grain call is self-flagged below.
- **EXCLUSION (2):** the product-implementation exclusion (EXC-001) and the
  cross-package PKG-01..12 non-advancement exclusion (EXC-002) are distinct
  durable scope boundaries.
- **DECLARED_STATE (6):** one per four-document kit surface (Specification,
  Datasheet, Guidance, Procedure) + `_STATUS.md` + `MEMORY.md` (addendum 1).
  No deliverable-owned in-tree README exists in the folder, so no README row.
- **REMAINING_WORK (0):** the only recorded `## Remaining` item is the seeded
  `(gated: D-41)` bootstrap, which per addendum 2 is copied verbatim only into
  the `_STATUS.md` surface row's RecordedRemaining (DECL-005) and gets no row
  of its own and no residual/gate/selectability analysis. No other
  evidence-backed residual is recorded (the REQ-07-05 handoff gap and the
  unproduced anticipated artifacts are within-deliverable documentation
  residuals, noted on their rows, not recorded in `## Remaining`).
- **IMPLEMENTED_UNMAPPED (0):** the material API-boundary/adapter surfaces in
  the tree — `api/api_boundary_contract.yaml` (SURF-002),
  `core/adapters/framework` (SURF-067),
  `schemas/adapter_framework.schema.yaml` (SURF-173) — are all attributed in
  `IMPLEMENTATION_SURFACES.csv` to downstream deliverables (DEL-02-04,
  DEL-10-02, DEL-10-04, and solver/load kits), not to DEL-00-07. None is
  unmapped, and none is in this documentation deliverable's authored orbit, so
  no unmapped rows.

## 2. Self-flagged rows

- **DEL-00-07-REQ-005** — Judgment call: PARTIALLY_IMPLEMENTED vs ALIGNED. The
  requirement enumerates handoff obligations for storage, reports, private
  libraries, local FEA export, and external automation. The kit addresses only
  the private-library boundary role and the local-FEA handoff path
  (OPS-K-MECH-1); storage/reports/external-automation handoff obligations are
  not concretely defined, and AB-00-07 does not restate them. Marked
  PARTIALLY_IMPLEMENTED. A reviewer applying the deliverable's own
  SEMANTIC_READY completion model (runway-grain prose + visible TBDs) could
  argue ALIGNED.
- **DEL-00-07-REQ-001..004 (representative: REQ-001)** — Judgment call on
  "define" satisfaction for a runway deliverable. These were marked ALIGNED
  because the kit states each obligation at architecture-runway grain and the
  accepted basis (SCA-001 / AB-00-07) intentionally keeps public transport and
  concrete formats TBD. The concrete named output artifact
  `docs/architecture/api_boundary_map.md` and a standalone adapter contract map
  do not exist in the tree; a stricter reviewer could read the "define the map"
  requirements as PARTIALLY_IMPLEMENTED on that basis. Confidence held at
  MEDIUM to reflect this and the pending human review gate.
- **DEL-00-07-DECL-002** — The Datasheet declares two `Outputs Expected`
  artifacts (`docs/architecture/api_boundary_map.md`; adapter contract map)
  that are absent tree-wide at the frozen SHA. Judged ALIGNED (declaration of a
  planning kit not overtaken by any implemented slice; the Procedure completion
  condition does not require those artifacts for SEMANTIC_READY, and the
  deliverable is IN_PROGRESS) rather than a REMAINING_STATE_MISMATCH. Reviewer
  eyes welcome on that ALIGNED-vs-mismatch line.
- **SourceReliability = UNVERIFIED on all requirement/acceptance/exclusion
  rows** — Judgment call. A human review (PKG00_LOCK_REVIEW_2026-05-11_2218)
  did cover DEL-00-07, which could argue REVIEWED per addendum 6. But D-40 /
  DEC-072 administratively reset CHECKING -> IN_PROGRESS and preserved that
  review only as a historical checking basis, so the content is currently
  pre-acceptance (agent-generated kit awaiting the Specification's Human Review
  Gate). I encoded UNVERIFIED. See friction note 4.
- **DEL-00-07-DECL-001 / DECL-002 / DECL-004** — rev-0.7 authority-pointer
  drift, originally kept ALIGNED with in-row notes; re-encoded to
  `STALE_SETUP_SPECIFICATION` in the fan-in repair below per the W1
  verification adjudication under addendum 4 (see
  `## Fan-in repair (fable re-run)`). AuthorityNeeded stays NO on all three
  rows: the decomposition's own §13 gate posture sanctions the state as
  permitted-pending-refresh "by their owning workflows", so no per-row
  authority routing is needed beyond the disposition.

## Fan-in repair (fable re-run)

W1 fan-in verification (`W1_VERIFICATION_PKG-00.md`, DEL-00-07 section:
DEFECTIVE) ruled the rev-0.7 authority-pointer drift rows DECL-001, DECL-002,
and DECL-004 on the wrong side under the addendum-4 adjudication (drift noted
on all three rows but kept ALIGNED; STALE_SETUP_SPECIFICATION is the
convention-correct side). Owner-ruled repair protocol: defective ledgers are
re-run by a fable pilot who owns the changed claims. This section records that
re-run.

Independent re-verification against the frozen tree (before re-encoding):

- `Specification.md` line 32: "Upstream authority is
  …SOFTWARE_DECOMP.md revision 0.7, the SCA-001/SCA-003/SCA-004
  architecture-basis records, and approved …DAG-007…" — confirmed present at
  frozen SHA 551f84ef6.
- `Datasheet.md` line 33: "…SOFTWARE_DECOMP.md revision 0.7 for package and
  deliverable authority." — confirmed.
- `Procedure.md` line 8: "…SOFTWARE_DECOMP.md revision 0.7 is the current
  basis." — confirmed.
- Frozen `execution/_Decomposition/SOFTWARE_DECOMP.md` header:
  `revision: 0.8`, `status: current_basis` — confirmed; §13 gate posture
  states downstream surfaces "may be stale relative to revision 0.8 until
  refreshed by their owning workflows" — confirmed.

Disagreements with the verifier adjudication: NONE. The Procedure line
declares a fact about the current authority basis that is false at the frozen
SHA; the corpus's own accepted language classifies the state as *stale*
(permitted-pending-refresh); addendum 4's widened definition ("the surface's
declaration no longer describes the frozen implemented slice … one controlled
value") covers this post-alignment drift, and the §13 sanction properly
informs AuthorityNeeded/immateriality notes, not the controlled disposition.

Rows changed (old -> new):

- DEL-00-07-DECL-001: ALIGNED -> STALE_SETUP_SPECIFICATION (drift facts
  expanded in-row; AuthorityNeeded stays NO; Confidence stays HIGH).
- DEL-00-07-DECL-002: ALIGNED -> STALE_SETUP_SPECIFICATION (drift facts
  expanded in-row; anticipated-artifact content retained; AuthorityNeeded
  stays NO; Confidence stays MEDIUM).
- DEL-00-07-DECL-004: ALIGNED -> STALE_SETUP_SPECIFICATION (drift facts
  expanded in-row; AuthorityNeeded stays NO; Confidence stays HIGH).

No other row changed. Histograms above are recounted post-repair
(ALIGNED 15 -> 12; STALE_SETUP_SPECIFICATION 0 -> 3; ClaimType histogram
unchanged). This encoding matches the peer repairs applied to
DEL-00-02/04/05/06.

Owner-calibration caveat (per the verifier's flip note): if the owner later
calibrates this corpus-wide rev-0.7 pattern to ALIGNED-with-note instead, the
flip back is mechanical — all drift facts remain in-row and the three ClaimIDs
are named here.

## 3. Evidence-execution log

- **Re-executed:** none. This is a documentation-only deliverable; there is no
  side-effect-free product test whose pass would bind these claims (the
  acceptance evidence is document review / direct inspection). No build,
  pytest, or cargo invocation was run.
- **Side-effect-free reads only:** file reads, directory listings, and `grep`
  over the frozen tree; a tree-wide `find` for `api_boundary_map.md`
  (absent) and `git rev-parse` / `git status --porcelain`.
- **Porcelain:** `git -C <FROZEN> status --porcelain` was empty before work
  began and empty after all reads (HEAD confirmed
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7`). No writes into the frozen tree.
- **Fan-in repair pass (fable re-run):** re-executed only side-effect-free
  reads (`git status --porcelain`, `git rev-parse HEAD`, `grep`/`sed` over the
  frozen kit and SOFTWARE_DECOMP) to independently re-verify the rev-0.7 lines
  and the frozen 0.8/`current_basis` header plus the §13 sanction before
  re-encoding. Porcelain EMPTY before and after the repair pass; frozen HEAD
  confirmed `551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Writes confined to the
  two output files.
- **Cited-as-recorded (not re-executed):** PKG00_LOCK_REVIEW_2026-05-11_2218
  (human review, later administratively reset by D-40/DEC-072), marked
  `not re-executed at frozen SHA 551f84ef6`. All DecisionBasis values resolve
  to artifacts present in the frozen tree: SCA-001 and AB-00-07 in
  `execution/_Decomposition/SOFTWARE_DECOMP.md`; D-40 ruling record in
  `execution/_Coordination/_DECISIONS/D-40_RULING_2026-07-11.md`; DEC-072 in
  SOFTWARE_DECOMP.md section 12. No ATTESTED-only DecisionBasis was needed.

## 4. Convention friction notes

1. **Acceptance-row grain with no ID'd verification table.** The addendum-12
   guidance ("acceptance rows only at addendum-12 grain; verification tables
   that merely restate requirements do not get mirrored ACCEPTANCE rows")
   assumes an ID'd verification table. This deliverable has only prose
   acceptance-criteria bullets. I read the rule's intent as "capture distinct
   verifiable acceptance content, do not double-count requirement
   restatements," yielding 3 acceptance rows. A firmer rule for
   bulleted-acceptance-criteria deliverables would remove ambiguity.
2. **"Define" satisfaction for runway documentation deliverables.** Conventions
   1/4 tell requirement rows to take the substance disposition and reserve
   STALE_SETUP_SPECIFICATION for declared-state rows, but for a documentation
   deliverable "substance" is the prose itself. When the accepted basis
   deliberately keeps specifics TBD (AB-00-07), the line between ALIGNED
   (obligation stated at runway grain) and PARTIALLY_IMPLEMENTED (concrete
   artifact absent) is a judgment the binding set does not resolve. I applied a
   consistent rule (ALIGNED when the kit states the obligation and routes
   specifics to accepted TBDs; PARTIALLY only where the enumerated obligation
   is materially uncovered — REQ-07-05) and flagged it.
3. **Anticipated-artifact absence vs residual.** Datasheet `Outputs Expected`
   artifacts that were never produced are not a recorded `## Remaining` item
   and, under the deliverable's own SEMANTIC_READY completion model, not
   required for the current lifecycle stage. The binding set has no explicit
   controlled encoding for "declared anticipated output not produced, but not
   overdue"; I recorded it as an ImplementationEvidence/RemainingWork note on
   the requirement and Datasheet-surface rows rather than manufacture a
   REMAINING_STATE_MISMATCH or REMAINING_WORK row.
4. **REVIEWED vs UNVERIFIED after an administratively-vacated human review**
   (addendum 6). A recorded human review that a later administrative ruling
   preserves only as historical/re-entry evidence (not current acceptance) is
   not cleanly one or the other. I chose UNVERIFIED to match the current
   pre-acceptance IN_PROGRESS posture; addendum 6 could be sharpened for this
   "reviewed-then-reset" case.

## 5. Boundary-compliance statement

- Discovery was read-only outside the two W1 output files. No `_STATUS.md`,
  register, decomposition, product, or DAG file was modified; no lifecycle
  transition was applied (none proposed; the deliverable stays IN_PROGRESS);
  no cross-project edit.
- No F-PIP-1..5 claim appears in the outputs: ACC-003 verifies the *absence*
  of any code-compliance / professional-approval / certification / sealing
  claim and asserts none itself; no release-readiness or issuance claim is
  made.
- Dispositions are recorded as agent judgments, not owner or engineering
  rulings; no agent-workflow redesign is proposed (none arose;
  DEFERRED_AGENT_WORKFLOW unused).
- Frozen tree clean: `git status --porcelain` empty before and after; HEAD
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7`; all writes confined to
  `WAVES/W1/CLAIM_CONCORDANCE_DEL-00-07.csv` and `WAVES/W1/NOTES_DEL-00-07.md`.
