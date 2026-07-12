# NOTES — DEL-06-04 Private rule-pack lifecycle and checksum handling (W3 discovery pilot)

Frozen source tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`
(read-only worktree `.claude-worktrees/piping-frozen-551f84ef6`). Binding set:
`R1_CONVENTIONS.md` (conventions 1–8 + addenda 1–13 + named repairs) plus the
W1 and W2 fan-in calibration items in the W3 pilot brief.

**NormativeSource path alias (addendum 12, declared once):**
`{DELROOT}` = `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-04_Private rule-pack lifecycle and checksum handling`.
Crate paths are written in full (`core/rules/rule_pack_lifecycle`,
`core/rules/rule_pack_document`).

## 1. Histograms (recount from CSV; both reproduce exactly)

**Disposition histogram (20 rows):**
- ALIGNED — 13
- PARTIALLY_IMPLEMENTED — 3
- DOCUMENTED_UNIMPLEMENTED — 1
- STALE_SETUP_SPECIFICATION — 3

**ClaimType histogram (20 rows):**
- REQUIREMENT — 12
- EXCLUSION — 1
- DECLARED_STATE — 7
- ACCEPTANCE — 0
- REMAINING_WORK — 0
- IMPLEMENTED_UNMAPPED — 0

AuthorityNeeded: NO — 13; OWNER — 7 (all seven OWNER rows are the
convention-6 SECURITY sufficiency deferrals; see §4).

## 2. Census decisions

- **Requirement rows (12):** R-06-04-001..012, re-verified against the frozen
  Specification requirements table (12 IDs, matches the R1 inventory). Each
  takes the substance disposition (convention 1); requirement rows never take
  `STALE_SETUP_SPECIFICATION`. ClaimIDs use the addendum-12 fixed form
  `DEL-06-04-REQ-NNN`; the native `R-06-04-NNN` IDs are recorded in
  `NormativeSource`.
- **Acceptance rows (0):** the Specification Verification table (V-06-04-001..008)
  checks setup-artifact hygiene (four documents exist, semantic matrix/lensing
  exist, `Dependencies.csv` validates, no protected content / compliance
  claims). These restate setup gates / other requirements rather than adding
  distinct acceptance thresholds at addendum-12 grain, so no mirrored
  ACCEPTANCE rows were created.
- **Exclusion rows (1):** `EXC-001` for the stable IP scope exclusion (no
  ASME/proprietary/protected rule-pack content; no private payloads in public
  artifacts). The storage/encryption/access deferral is already a requirement
  (R-06-04-011), so it is not duplicated as an exclusion.
- **Declared-state rows (7):** one per four-document kit surface (Specification,
  Datasheet, Guidance, Procedure) plus `_STATUS.md`, `MEMORY.md`, and the one
  deliverable-owned in-tree README (`core/rules/rule_pack_lifecycle/README.md`,
  a DECL surface per W2 calibration item 10 even though self-identifying).
- **Remaining-work rows (0):** the only `## Remaining` item is the seeded
  `(gated: D-41)` bootstrap, which goes verbatim into the `_STATUS.md` surface
  row's `RecordedRemaining` only and is excluded from residual/gate/selectability
  analysis (addendum 2). MEMORY "Remaining TBDs" are durable notes, not the
  `## Remaining` work-discovery surface; those residuals are carried in the
  relevant requirement rows' `RemainingWork` cells, not as standalone rows.
- **Implemented-unmapped rows (0):** every material surface in DEL-06-04's orbit
  carries a deliverable mapping in the R1 `IMPLEMENTATION_SURFACES.csv`
  (SURF-120 `rule_pack_lifecycle`, SURF-119 `rule_pack_document`, SURF-005
  `src-tauri`, SURF-205 `rule_pack.schema.yaml`, SURF-061 `rulePackService.ts`,
  SURF-143/144 examples). None are unmapped; the desktop/SQLite surfaces are
  owned/mapped by sibling deliverables (DEL-06-01/06-02 and persistence).

## 3. Deliverable posture

DEL-06-04 is a setup deliverable whose four documents describe a *future*
implementation contract, but a bounded implementation slice now exists in the
frozen tree: crate `core/rules/rule_pack_lifecycle` (lifecycle metadata,
`ChecksumRecord`, `LifecycleFindingCode`, `AuditManifestEntry`,
`ProfessionalBoundary`; commit `ad270f6`), its production consumer
`core/rules/rule_pack_document` (RFC 8785 / JCS canonicalization binding
`grammar_version`, DEC-022), plus desktop Tauri commands and SQLite store v10
`local_rule_packs`. `_STATUS.md` is IN_PROGRESS (affirmed 2026-06-17). This is
why the requirement rows carry live implementation/verification evidence while
the setup-era declarative surfaces (Specification, Datasheet, Procedure) are
STALE.

## 4. SECURITY / convention-6 encoding (Part C spot-check target)

This is a SECURITY/privacy/integrity deliverable. Convention-6 encoding
(W1 calibration item 2) — `ValidationEvidence=NONE_FOUND — sufficiency review
deferred, owner-gated`, `AuthorityNeeded=OWNER` — is applied to the seven
SECURITY-class requirement rows whose sufficiency review is recorded as
deferred/owner-gated:

- R-06-04-002 (private-by-default): marking/export-gate implemented; full
  privacy enforcement deferred to PKG-12.
- R-06-04-003 (protected-content exclusion): export gate + heuristic quarantine
  implemented; protected-content-detection sufficiency owner-gated.
- R-06-04-004 (JCS payload hash): metadata in the lifecycle crate + real
  RFC 8785 canonicalization in `rule_pack_document`; integrity-mechanism
  sufficiency owner-gated.
- R-06-04-005 (manifest hashes): representation implemented; non-JSON/binary
  partitioning recorded TBD.
- R-06-04-006 (checksum identity / no volatile fields): implemented; integrity
  sufficiency owner-gated.
- R-06-04-011 (storage/encryption/access deferred to PKG-12): deferral honored;
  security sufficiency owner/PKG-12-gated by construction.
- R-06-04-012 (no-bypass / schema-first envelope): non-bypassable controls
  implemented; result-envelope integration downstream; no-bypass sufficiency
  owner-gated.

Per the brief and convention 6, the pilot **does not assert integrity- or
privacy-mechanism sufficiency**; the em-dash marker routes that judgment to the
owner. Convention 6 also forbids a `VERIFIED_NOT_VALIDATED` downgrade on that
ground, so these rows keep their substance disposition (ALIGNED/PARTIALLY).
No unit test of checksum behavior was promoted to engineering validation
(verification != validation). Confidence on owner-gated-validation rows is
capped at MEDIUM (addendum 13).

Finding `PKG06-04-PKG02-001` (Review_Findings.csv) — the lifecycle crate
labels caller bytes `CallerSuppliedJcsBytesUnverified` rather than proving JCS —
was human-dispositioned `ACCEPT_AS_IS`/`RESOLVED` (2026-06-05). That closes the
labeling finding but does not settle integrity-mechanism sufficiency, which
remains the owner-gated question above.

## 5. rev-0.7 authority-pointer drift (W1 calibration item 1) — owner-calibration caveat

The deliverable's `_CONTEXT.md` and `_REFERENCES.md` cite
`SOFTWARE_DECOMP.md` **revision 0.7** and `DAG-006` as current basis, and the
dated MEMORY entry 2026-06-04 (`TP-AUTHORITY-REFRESH-0_7-DAG006`) repeats it.
The frozen decomp header is revision **0.8** (`status: current_basis`) and the
live DAG pointer is `DAG-007` (RUN_BASIS). This is the same pointer drift
adjudicated STALE-side in W1. Here it does **not** attach to any addendum-1
census DECL row: `_CONTEXT.md`/`_REFERENCES.md` are not census surfaces, and the
MEMORY occurrence is inside a *dated* log entry (historical per addendum 1, note
only). It is therefore recorded here as the once-per-notes owner-calibration
caveat rather than as a staleness disposition. `AuthorityNeeded=NO` (pure
pointer drift; no overtaken TBD register on a census surface).

## 6. Self-flagged rows

- **DEL-06-04-REQ-002** — class (SECURITY vs GOVERNANCE/IP) and grain judgment
  (default-private *marking* implemented vs end-to-end privacy enforcement
  deferred to PKG-12); OWNER routing.
- **DEL-06-04-REQ-003** — encoded at crate-contribution grain; the requirement
  is a project-wide IP-boundary invariant broader than this crate; OWNER
  routing.
- **DEL-06-04-REQ-004** — ALIGNED depends on cross-crate composition: the
  lifecycle crate records unverified caller-supplied metadata, while real JCS
  canonicalization lives in the consumer `rule_pack_document`. Grain judgment.
- **DEL-06-04-REQ-005** — PARTIALLY_IMPLEMENTED (partition TBD, no permitting
  ruling so not ACCEPTED_DIVERGENCE, addendum 11); OWNER-vs-ENGINEERING routing
  resolved to OWNER under convention 6 (checksum/integrity).
- **DEL-06-04-REQ-007** — PARTIALLY_IMPLEMENTED because 'rule-check-required
  data gaps' is out of the lifecycle crate's boundary and homed to DEL-06-03;
  the other five diagnostic families are implemented.
- **DEL-06-04-REQ-010** — DOCUMENTED_UNIMPLEMENTED within this slice (no numeric
  evaluation); substance homed to DEL-02-02 unit system + DEL-06-02/06-03. Flag
  for reviewer: the value marks absence-in-this-deliverable, not a program gap.
- **DEL-06-04-REQ-012** — class SECURITY vs GOVERNANCE and PARTIALLY grain
  (controls implemented; result-envelope integration downstream).
- **DEL-06-04-DECL-002** (Datasheet) and **DEL-06-04-DECL-004** (Procedure) —
  STALE judgment: Datasheet's 'without creating implementation files' and
  Procedure's SEMANTIC_READY expectation are overtaken by the implemented slice
  and the current IN_PROGRESS state.
- **DEL-06-04-DECL-003** (Guidance) — kept ALIGNED rather than STALE: treated as
  forward-looking guidance/principles for an IN_PROGRESS deliverable with no
  falsifiable current-state declaration. Contrast with Spec/Datasheet/Procedure.
- **DEL-06-04-DECL-006** (MEMORY) — ALIGNED-with-note under W2 calibration
  item 9: the undated 'Remaining TBDs' drift ('API transport remains downstream')
  is corrected in-file by the dated 2026-06-12 TP-C2-RPLIFE-001 entry.

## 7. Evidence-execution log

- **Re-executed:** nothing rebuilt or re-tested. Verification rests on the
  recorded crate-sweep pass (`validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`,
  `cargo_crate_sweep=pass`) at ancestor commit `e648462f1d05`, cited with the
  `not re-executed at frozen SHA 551f84ef6` marker.
- **Read-only git checks I did run** (frozen worktree): `git rev-parse HEAD`
  (= 551f84ef6...), `git merge-base --is-ancestor e648462f1d05 551f84ef6` (YES),
  and `git diff --stat e648462f1d05 551f84ef6 -- core/rules/rule_pack_lifecycle
  core/rules/rule_pack_document` (**empty**). This substantiates the addendum-10
  qualifier `content-identical at frozen SHA 551f84ef6be656f1603ce0acfa5e3935aa9683c7
  (diff empty over core/rules/rule_pack_lifecycle | core/rules/rule_pack_document)`
  written per crate in the ledger (calibration item 4: qualifier only for paths
  actually diffed). `git status --porcelain` on the frozen worktree was empty
  before and after every operation.
- I did not use the byte-identical out-of-tree re-execution pattern (calibration
  item 12); the recorded sweep pass plus content-identical qualifier was
  sufficient and lower-risk.

## 8. Convention-friction notes

- **rev-0.7 drift has no census home** (see §5): the authority-pointer drift
  lives only in non-census kit surfaces and a dated MEMORY entry, so it cannot
  attach to an addendum-1 DECL row. Handled as the once-per-notes caveat.
- **Cross-crate ALIGNED for R-06-04-004/006**: the requirement is satisfied by
  the lifecycle crate + its consumer `rule_pack_document` jointly. Both are in
  DEL-06-04's orbit (R1 SURF-119/120 attributions; MEMORY names
  `rule_pack_document` as the first production consumer), so citing the
  consumer's real JCS canonicalization for these rows is in-scope.
- **OWNER-routing density**: seven OWNER rows may read as heavy against W2
  calibration item 14, but each is a genuine SECURITY sufficiency deferral that
  the brief explicitly routes to the owner via convention 6; item 14's
  "router-not-work-queue" caution is respected (no ENGINEERING used to mean
  "do the remaining work"; TBDs routed once at their owning claim).
- **DOCUMENTED_UNIMPLEMENTED vs homing** for R-06-04-010: §7 offers no
  "satisfied-elsewhere" disposition, so the value marks absence in this
  deliverable's slice with the home recorded in `RemainingWork`.

## 9. Boundary-compliance statement

- Writes confined to the two W3 output files
  (`WAVES/W3/CLAIM_CONCORDANCE_DEL-06-04.csv`, `WAVES/W3/NOTES_DEL-06-04.md`).
- No frozen-tree writes; frozen worktree `git status --porcelain` empty before
  and after all operations (verified).
- No lifecycle transition, DAG mutation, cross-project edit, or edit to any
  `_STATUS.md`/register/product file. `LIFECYCLE_REASSESSMENT_REQUIRED` not used
  (no lifecycle change proposed or applied).
- No release-readiness, issuance, certification, sealing, professional-approval,
  or code-compliance claim (F-PIP-1..5). Reporting that R-06-04-009's crate
  *rejects* professional/compliance claims is a factual evidence observation,
  not such a claim.
- Dispositions are agent judgments routed via `AuthorityNeeded`, not owner or
  engineering rulings. `SelectableUnderCurrentLoop` is mechanical (all NO: no
  non-bootstrap recorded residual, addendum 12); the owner suspension is a
  run-level caveat, not encoded per row.
