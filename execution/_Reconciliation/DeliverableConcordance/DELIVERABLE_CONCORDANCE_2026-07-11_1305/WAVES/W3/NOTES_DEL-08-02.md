# NOTES — DEL-08-02 Audit manifest and model hash (W3 discovery pilot)

Deliverable: DEL-08-02 (PKG-08 Reporting, Audit, and Reproducibility), IN_PROGRESS.
Frozen source SHA: `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Ledger: `CLAIM_CONCORDANCE_DEL-08-02.csv` (18 rows).

This is a **setup-plus-bounded-implementation** deliverable: the four-document
kit is setup-era (future-tense, "no code here"), but a real implemented crate
`core/reporting/audit_manifest` (crate `open_pipe_stress_audit_manifest`, 995-line
`src/lib.rs`, 13 unit tests) exists and is hardened. The requirement substance is
therefore dispositioned against implementation, and the setup-era kit prose that
declares no implementation is dispositioned STALE where overtaken.

## 1. Histograms (recomputed from the CSV)

**Disposition histogram**
- ALIGNED — 15
- STALE_SETUP_SPECIFICATION — 3
- (total 18)

**ClaimType histogram**
- REQUIREMENT — 10
- DECLARED_STATE — 7
- EXCLUSION — 1
- (total 18)

Row census: 10 REQ (R1–R10, one per current requirement ID, re-verified against
`Specification.md` at the frozen SHA), 1 EXCLUSION (professional/certification
scope exclusion), 7 DECLARED_STATE (four-doc kit ×4 + `_STATUS.md` + `MEMORY.md` +
the deliverable-owned crate `README.md`). No ACCEPTANCE rows (the Specification
V-1..V-7 verification table merely restates requirements — brief census rule). No
REMAINING_WORK rows (see §4). No IMPLEMENTED_UNMAPPED rows (see §4).

## 2. Self-flagged rows

- **DEL-08-02-REQ-002** (R2, JCS canonicalization) — **grain judgment (calibration
  item 6).** The requirement asks for "JCS-compatible canonicalization." I ruled
  ALIGNED **at project/hash-seam contract grain**: the project's true RFC 8785/JCS
  renderer is `core/serialization/canonical_json` (SURF-129), consumed at every
  hash seam per the 2026-06-11 H5 unification, and `hashService.ts` (SURF-053) is
  the canonical JCS hashing service. This crate itself deliberately hashes
  `ProjectLocalDeterministicJson` (sorted-key serialization, caller-supplied number
  strings) and only *names* the shared renderer in its doc comment. A this-crate
  grain reading would take PARTIALLY_IMPLEMENTED / IMPLEMENTED_DIFFERENTLY. There is
  an **open human disposition** on exactly this point (`Review_Findings.csv`
  DEL-08-02-PKG02-001, WARNING, TECHNICALLY_ADDRESSED_PENDING_HUMAN, HumanDisposition
  TBD), so I routed AuthorityNeeded=OWNER and capped Confidence at MEDIUM.
- **DEL-08-02-REQ-008** (R8, protected/private data exclusion) — **SECURITY
  convention-6 encoding + Part C spot-check candidate.** ClaimClass=SECURITY;
  ValidationEvidence carries the exact em-dash marker
  `NONE_FOUND — sufficiency review deferred, owner-gated`; AuthorityNeeded=OWNER;
  Disposition ALIGNED (mechanism implemented + unit-tested, following the W1 SECURITY
  precedent DEL-02-04-REQ-013 / DEL-03-07-REQ-003 where owner-gated sufficiency does
  not by itself downgrade to VERIFIED_NOT_VALIDATED). Per the dispatch instruction:
  the hash-determinism unit tests are **verification**, not audit-sufficiency
  validation — the "does the metadata-declared boundary sufficiently prevent
  protected/private embedding across all public-template paths" question is the
  owner-gated deferral.
- **DEL-08-02-REQ-009** (R9, state distinction) — **interpretive call.** The
  no-compliance/no-approval half is enforced by hard `ProfessionalBoundary` defaults
  plus a blocking finding and is directly tested. The tri-state distinction
  (mechanics solved / user-rule checked / human acceptance) is realized via *distinct
  manifest fields* (model+solver hashes / `rule_pack_refs` / `professional_boundary`
  + `human_review_required`) rather than a named state enum. I read that as
  satisfying the requirement at manifest-structure grain (ALIGNED, MEDIUM).
- **DEL-08-02-DECL-001 / -002 / -004** (Specification / Datasheet / Procedure) —
  **STALE routing + OWNER.** These three setup docs declare the deliverable is
  documentation-only with future implementation; that premise is overtaken by the
  implemented+tested crate (widened addendum 4). I routed AuthorityNeeded=OWNER
  (kit-refresh repair needing an owner-authorized tranche recorded in `## Remaining`)
  following the DEL-02-05-DECL-001 precedent, because the drift is substantive
  overtaken framing (and, for the Specification, an overtaken canonicalization
  "library/API TBD"), not mere authority-pointer drift.
- **DEL-08-02-DECL-003** (Guidance) — **ALIGNED, not STALE.** Guidance is advisory
  ("should") design principles that the implementation demonstrably follows, and its
  one conditional ("do not choose a container/hashing library without a later
  implementation brief or human decision") was satisfied — implementation proceeded
  under bounded Type 2 briefs / run records. A stricter reading of its future-tense
  prose could take STALE_SETUP_SPECIFICATION; flagged for reviewer eyes.
- **DEL-08-02-DECL-005** (MEMORY) — **ALIGNED-with-note under calibration item 9.**
  The undated Verification header block says "9 focused tests"; this is corrected
  **in the same file** by the dated 2026-06-06 "Audit manifest hardening" entry
  ("passed 13 focused tests"), which matches the frozen crate. Item 9's in-file
  correction path therefore permits ALIGNED-with-note rather than STALE. The dated
  2026-06-04 entry citing SOFTWARE_DECOMP revision 0.7 / DAG-006 is a historical log
  entry (addendum 1), noted not dispositioned.

## 3. Evidence-execution log

- **Re-executed (side-effect-free, calibration item 12 byte-identical out-of-tree
  copy):** `cargo test` on `core/reporting/audit_manifest`. The crate is standalone
  (no workspace root, empty `[dependencies]`, `Cargo.lock` gitignored and absent).
  In-place `cargo test` would write `target/` and `Cargo.lock` into the frozen tree
  (forbidden even git-ignored, addendum 9), so I copied the crate to scratch,
  verified byte-identity with `diff -r` (empty), ran
  `CARGO_TARGET_DIR=<scratch> cargo test --offline` (offline because no committed
  lock and zero dependencies). **Result: 13 passed / 0 failed / 0 ignored** (plus 0
  doc-tests). Frozen-tree `git status --porcelain` **empty before and after** the
  copy and the test run. This satisfies all four item-12 conditions (byte-identity,
  external target dir, porcelain empty before/after, in-row disclosure).
- **Cited as recorded (not re-executed in place):** VERIFICATION_INDEX row RUST-09
  binds the same 13-test crate to sweep `SWEEP_20260711T040758Z_e648462f1d05.json`
  at ancestor commit `e648462f1d0521e26df15d04a988391343018886` with the addendum-10
  qualifier `content-identical at frozen SHA
  551f84ef6be656f1603ce0acfa5e3935aa9683c7 (diff empty over core/, ...)`. Because I
  re-ran the byte-identical frozen source directly, the ledger states "re-executed at
  frozen SHA 551f84ef6" and cites RUST-09 as the corroborating recorded pass.
- **Not re-executed:** the shared `core/serialization/canonical_json` RFC 8785/JCS
  conformance suite (out of this crate's scope; referenced for the R2 contract-grain
  disposition, not re-run) and the desktop `hashService.ts` vitest.

## 4. Convention-friction / boundary reasoning notes

- **REMAINING_WORK census.** `_STATUS.md ## Remaining` (method §6 "sole current
  work-discovery surface") carries only the seeded `(gated: D-41)` bootstrap item,
  which is recorded verbatim in the DECL-006 `RecordedRemaining` cell and excluded
  from all residual/gate/selectability analysis (addendum 2). `MEMORY.md`'s
  "Remaining TBDs" (physical container, persistence adapter, report/API/CLI
  integration, PKG-12 private storage/redaction) are **downstream deferrals owned by
  other deliverables** (DEL-02-05, DEL-08-01, DEL-10-05, PKG-12), not this
  deliverable's `_STATUS` residuals — so no REMAINING_WORK rows were created. The
  full-JCS deferral noted in the 2026-06-06 MEMORY entry was subsequently resolved at
  the engine/hash seams by the H5 unification (shared `canonical_json` crate).
- **Bootstrap `_STATUS` cell scoping (calibration item 11).** DECL-006 uses the
  exclusion variant: `GateOrStageConstraint`/`RemainingSource` = NONE_RECORDED scoped
  to non-bootstrap content; the bootstrap item is *not* annotated into gate/source
  cells. `SelectableUnderCurrentLoop=NO` (no non-bootstrap recorded item; addendum 12).
- **Bootstrap byte-exactness (calibration item 5).** The `RecordedRemaining` cell was
  verified byte-for-byte against `_STATUS.md` including the `§§6–8` en-dash (an initial
  hyphen substitution was caught and corrected before finalizing).
- **IMPLEMENTED_UNMAPPED census.** Every material surface in this deliverable's orbit
  is already mapped to DEL-08-02 in IMPLEMENTATION_SURFACES.csv: the audit_manifest
  crate (SURF-106), the shared canonical_json crate (SURF-129), `hashService.ts`
  (SURF-053), `usePackageHash.ts` (SURF-064). No unmapped surface remains, so no
  IMPLEMENTED_UNMAPPED rows.
- **Cross-reference discipline (dispatch instruction).** DEL-05-04 acceptance-record
  hash-binding cross-references exist (schemas/headless_runner.schema.yaml, SURF-187,
  attributes both), but only THIS deliverable's concordance facts are recorded here.
- **SourceReliability leg-keying (W2 calibration item 13).** All REQ/EXC rows are
  UNVERIFIED: the load-bearing evidence legs are agent-generated unit tests and
  agent/self-check reviews (`_REVIEW.md` is Review Type SELF_CHECK/AGENT_CHECK). The
  2026-06-06 human-approved Gate 5 covered a *lifecycle transition* (later reset to
  IN_PROGRESS), not the correctness of the tests as verification/validation, so it
  does not lift these rows to REVIEWED. DECL prose rows are NOT_APPLICABLE (addendum 6).
- **Selectability / owner-suspension.** `SelectableUnderCurrentLoop` is mechanical
  (DAG/lifecycle/gate) and NO on every row (no recorded selectable item). The
  owner-suspension declaration is a run-level caveat, not encoded per-row.
- **Pending-human findings.** Two `Review_Findings.csv` entries are
  TECHNICALLY_ADDRESSED_PENDING_HUMAN with HumanDisposition=TBD: PKG02-001 (JCS
  overclaim removal — load-bearing for R2, routed OWNER, Confidence MEDIUM per
  addendum 13) and PKG02-002 (DEL-02-02 unit prerequisite metadata — INFO, metadata
  only, not load-bearing for R6's implemented `unit_system_ref`, so R6 stays
  AuthorityNeeded=NO / HIGH with the finding noted).

## 5. Boundary-compliance statement

All fences held. Discovery was read-only outside the two output files
(`CLAIM_CONCORDANCE_DEL-08-02.csv`, `NOTES_DEL-08-02.md`, both under `RUN/WAVES/W3/`).
No lifecycle transition was applied (no LIFECYCLE_REASSESSMENT_REQUIRED used; all
dispositions are agent judgments routed via AuthorityNeeded, never phrased as owner
or engineering rulings). No DAG mutation, no cross-project edit, no edit to any
`_STATUS.md`/register/product file. No release-readiness, issuance, certification,
sealing, professional-approval, or code-compliance claim appears anywhere in the
outputs (F-PIP-1..5); the EXC-001 row records the deliverable's own professional
boundary as a scope exclusion, asserting no certification. The frozen evidence
worktree `git status --porcelain` was verified **empty before and after** every read
and the out-of-tree test re-execution; all writes were confined to the two W3 output
files in the working run folder.
