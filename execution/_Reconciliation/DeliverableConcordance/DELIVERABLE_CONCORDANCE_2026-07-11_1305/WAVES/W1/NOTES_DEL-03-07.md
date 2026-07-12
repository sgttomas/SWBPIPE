# NOTES — DEL-03-07 Public/private library import provenance checker (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Encoding per `R1_CONVENTIONS.md`
(conventions 1-8 + addenda 1-13). Ledger: `CLAIM_CONCORDANCE_DEL-03-07.csv`
(17 data rows, 20 columns, RFC-4180 clean via `csv` round-trip).

**NormativeSource path alias (addendum 12, declared once):** `KIT/` =
`execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-07_Public-private library import provenance checker/`.
All non-KIT paths (`core/…`, `tests/…`, `fixtures/…`, `apps/…`,
`schemas/…`, `execution/…`, `plans/…`, `docs/…`) are repo-root-relative
under `projects/chirality-piping/` in the frozen tree.

## Requirement-ID normalization (addendum 12)

Deliverable scheme is `DEL-03-07-R*`. Mapping to the addendum-12 ClaimID form:

| Kit ID | ClaimID | ClaimClass |
|---|---|---|
| DEL-03-07-R1 | DEL-03-07-REQ-001 | GOVERNANCE |
| DEL-03-07-R2 | DEL-03-07-REQ-002 | GOVERNANCE |
| DEL-03-07-R3 | DEL-03-07-REQ-003 | SECURITY |
| DEL-03-07-R4 | DEL-03-07-REQ-004 | SECURITY |
| DEL-03-07-R5 | DEL-03-07-REQ-005 | SCHEMA |
| DEL-03-07-R6 | DEL-03-07-REQ-006 | INTEROP |
| DEL-03-07-R7 | DEL-03-07-REQ-007 | GOVERNANCE |

Seven requirement IDs, one REQ row each (convention 1: requirement rows always
take the substance disposition, never `STALE_SETUP_SPECIFICATION`).

## Histograms (recounted from the CSV before writing)

**Disposition histogram**

| Disposition | Count |
|---|---|
| ALIGNED | 17 |
| **Total** | **17** |

**ClaimType histogram**

| ClaimType | Count |
|---|---|
| REQUIREMENT | 7 |
| EXCLUSION | 4 |
| DECLARED_STATE | 6 |
| **Total** | **17** |

(Confidence, for orientation: HIGH 11, MEDIUM 6.)

## Row census rationale

- **7 REQUIREMENT rows** — one per current requirement ID (R1-R7).
- **4 EXCLUSION rows** — the four `## Scope` "It excludes" bullets: external
  import formats/parsers (EXC-001), legal license/redistribution conclusions
  (EXC-002), protected standards/vendor/derived content (EXC-003), global
  solver/rule-evaluator/GUI implementation (EXC-004).
- **6 DECLARED_STATE rows** — the four-document kit (Specification, Datasheet,
  Guidance, Procedure) + `_STATUS.md` + `MEMORY.md` (addendum 1 census). No
  deliverable-owned in-tree README exists in the kit folder; `core/library_import/README.md`
  is a product-code surface cited as ImplementationEvidence, not a declared-state
  kit surface (mirrors the DEL-03-03 exemplar's census).
- **0 REMAINING_WORK rows** — `_STATUS.md ## Remaining` carries only the seeded
  `(gated: D-41)` bootstrap item, recorded verbatim in the DECL-005 (`_STATUS`)
  row's `RecordedRemaining` and excluded from all residual/gate/selectability
  analysis (addendum 2). The Specification/Datasheet/MEMORY "remaining unresolved
  items" (external formats, source-catalog/legal policy, fixture-value authority,
  dependency satisfaction, review-finding disposition, lifecycle closure) are
  declared kit-surface TBDs, NOT `_STATUS` residuals, so they get no REM row
  (handled as surface notes per addendum 1; Guidance C1-C4 are declared open
  conflicts with no permitting ruling).
- **0 IMPLEMENTED_UNMAPPED rows** — every material surface in this deliverable's
  orbit is already deliverable-attributed in `IMPLEMENTATION_SURFACES.csv`
  (SURF-091 Rust crate `library_import_document` -> DEL-03-07; SURF-092 Python
  module -> DEL-03-07/DEL-10-02; SURF-054 import service, SURF-026 library panel,
  SURF-007 app shell — all mapped and documented in MEMORY). Nothing is unmapped
  or materially undocumented, so no UNMAP/IMPLEMENTED_UNDOCUMENTED rows (addendum 8).

## Convention-6 SECURITY encoding (F-PIP-1 fence-adjacent; Part C spot-check target)

This is a SECURITY-class deliverable (public/private boundary, protected-content
handling, private-data governance). Two behavior claims are SECURITY-class with
owner-deferred sufficiency review and carry the convention-6 encoding **exactly**:

- **DEL-03-07-REQ-003** (public/private separation, OPS-K-PRIV-1/OPS-K-DATA-1)
- **DEL-03-07-REQ-004** (protected-content quarantine, OPS-K-IP-1/3, OPS-K-GOV-4)

Both use `ValidationEvidence = NONE_FOUND — sufficiency review deferred,
owner-gated`, remain `ALIGNED` (no `VERIFIED_NOT_VALIDATED` downgrade on that
ground, per convention 6), `Confidence=MEDIUM`, `AuthorityNeeded=OWNER`, and are
self-flagged for the Part C reviewer spot-check. Rationale: the behavior is
implemented and unit+parity-tested (verification-class), and the requirement
wording itself scopes sufficiency (legal/protected-content adequacy) to human
review — so the implemented behavior is aligned with the current requirement,
while the security/IP *sufficiency* audit (does the metadata-declared boundary
actually stop all leakage / detect protected content beyond a self-declared
flag) is the owner-gated deferral. `SourceReliability=REVIEWED` on both because
DEC-036 (2026-06-13 human ruling) governs the private-data/quarantine
refuse-to-store posture; the deferred piece is the sufficiency review, encoded in
the validation cell, not the reliability ladder.

REQ-001 (provenance-metadata gate) and REQ-007 (no-legal-claims boundary) were
read as GOVERNANCE, not SECURITY — they are IP/rights-governance and
professional-boundary claims that are fully implemented (metadata presence gate;
no legal-acceptance path). This keeps the SECURITY encoding scoped to the two
genuine protection-behavior claims. See self-flags for the REQ-001 borderline.

## Self-flagged rows

- **DEL-03-07-REQ-002** — ClaimClass GOVERNANCE vs WORKFLOW judgment. The R0b
  convention-5 rubric routes diagnostic/warning behavior to WORKFLOW absent
  numeric-mechanics/report content; I chose GOVERNANCE because the substance is
  the no-silent-default IP/data-boundary invariant (OPS-K-DATA-2), the diagnostic
  being the mechanism. Reviewer eyes on the class.
- **DEL-03-07-REQ-003** — SECURITY-class row; Part C convention-6 encoding
  spot-check target (see section above). Also flags the REQ-001 borderline: R1
  could be read SECURITY (access-gate on public data) rather than GOVERNANCE.
- **DEL-03-07-REQ-004** — SECURITY-class row; Part C convention-6 encoding
  spot-check target. Protected-content DETECTION rests on a self-declared
  metadata flag (`redistribution_status=='protected_suspected'` /
  `review_status=='quarantined'`), not content analysis — sufficiency is
  owner/legal-gated (Guidance C1-C3 TBD).
- **DEL-03-07-REQ-005** — ClaimClass SCHEMA vs MECHANICS judgment for a code-side
  unit-metadata presence check. Chose SCHEMA (data/unit contract, OPS-K-UNIT-1);
  no numeric computation occurs so MECHANICS felt over-strong.
- **DEL-03-07-REQ-007** — verification gap: no test isolates the
  `REVIEW_REQUIRED`-only outcome. The review-needed status is verified via
  `test_public_material_import_with_tbd_rights_is_rejected_for_review`
  (REJECTED + IMPORT_REVIEW_REQUIRED) and the private-local case plus the code
  path (L274-283, L338-339); disposition kept ALIGNED at MEDIUM.
- **DEL-03-07-EXC-004** — GUI-exclusion vs documented GUI-surfacing tension.
  The Specification excludes GUI implementation, yet later app-integration C3
  tranches added `LibraryManagerPanel` (SURF-026) + import service (SURF-054),
  logged under DEL-03-07's MEMORY. Read as cross-package app-integration
  surfacing (shared surfaces attributed to PKG-07 GUI deliverables too), not a
  DEL-03-07 scope expansion — exclusion held ALIGNED, but flagged for review.
- **DEL-03-07-DECL-002** — Datasheet cites SOFTWARE_DECOMP "revision 0.7" as
  Decomposition Basis; frozen basis is 0.8 (through DEC-072). A
  decomposition-revision citation lag (the 2026-06-04 authority refresh set the
  0.7-era basis; SCA-005/D-21 -> 0.8 landed 2026-07-02 and the kit was not
  re-refreshed), not a slice-description staleness and not a `_STATUS` residual.
  Same lag noted (not separately flagged) on DECL-001's Standards table.
- **DEL-03-07-DECL-006** — MEMORY historical-TBD + GUI-attribution drift.
  Open Items (external formats, legal/license + accepted public source catalogs,
  UI/editor GUI, downstream adapter interop) persist and are not on `_STATUS`;
  addendum 1 makes this a surface note, never a staleness disposition. The
  2026-06-13 GUI-slice entry ties to the EXC-004 tension.

## Evidence-execution log

- **Re-executed (side-effect-free, addendum 9):**
  `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q -p no:cacheprovider
  tests/test_library_import_provenance.py` -> `7 passed in 0.02s`.
  `git -C <FROZEN> status --porcelain` empty **before and after** (0 lines both
  times). No `CARGO_TARGET_DIR` needed (pure-Python, stdlib-only). This is
  VERIFICATION_INDEX PY-36, whose recorded pass is at ancestor
  `e648462f1d0521e26df15d04a988391343018886` and is marked CONTENT_IDENTICAL at
  the frozen SHA (diff empty over the listed paths); I cite the re-execution plus
  the content-identical marker.
- **Cited as recorded (NOT re-executed):** VERIFICATION_INDEX RUST-02, crate
  `open_pipe_stress_library_import_document` `cargo test` 18/18
  (7 Python-parity cases over the same invented fixtures + 11 unit tests), with
  the marker `not re-executed at frozen SHA 551f84ef6` and the RUST-02
  content-identical qualifier. A cargo build would require a redirected target
  dir and I did not need to re-run it to bind the parity claim, so I left it
  recorded-only (addendum 9 conservatism).
- **DecisionBasis resolvability (convention 7):** DEC-036 (SOFTWARE_DECOMP.md §12
  L614, frozen), DEC-018 (§12 L596; ruling record
  `D-01_unit_catalog_acceptance.md` present), DEC-012 (§12), the Gate A human
  disposition (Review_Findings.csv rows + MEMORY 2026-06-05), and the K-CONFLICT-1
  LifecycleCorrection Decision_Log — all resolve to artifacts in the frozen tree
  and govern their cited claims. NONE_FOUND used where no accepted decision
  selects the implementation basis (REQ-001/002/007, EXC-002/003/004).
- Frozen decomposition confirmed at revision 0.8 through DEC-072 (kit cites 0.7 —
  the DECL-001/002 citation-lag note). DEC-073/0.8-on-main is the live register
  state per RUN_BASIS; ruling-after-freeze mechanics, not re-derived here.

## Convention friction notes

- **Diagnostic-behavior class (convention 5/7).** The binding set routes generic
  diagnostic/warning behavior to WORKFLOW but adds no explicit slot for
  IP/data-governance diagnostics that are the *substance* of a governance
  requirement (R2). I encoded by substance (GOVERNANCE) and self-flagged; a
  reviewer convention on "diagnostic mechanism of a governance invariant" would
  remove the ambiguity across PKG-03/PKG-01.
- **SECURITY sufficiency-deferral disposition.** Convention 6 fixes the
  ValidationEvidence string and forbids the VERIFIED_NOT_VALIDATED downgrade but
  does not name the disposition. I used ALIGNED (the requirement wording itself
  scopes sufficiency to human review, so implemented+verified behavior aligns
  with the current requirement) with AuthorityNeeded=OWNER + MEDIUM + self-flag,
  rather than ACCEPTED_DIVERGENCE (addendum 11 needs a ruling that *permits a
  deferred state*; none permits the sufficiency deferral specifically — it is
  baked into the requirement, not a divergence from it). Flagged for the Part C
  spot-check to confirm this reading.
- **GUI exclusion vs app-integration surfacing.** The evidence-reconciled
  Specification (2026-06-05) predates the C3 GUI slice (2026-06-13); the binding
  set has no explicit rule for a later cross-package surface that a still-current
  exclusion names. Handled via the surface-attribution/addendum-8 lens (shared
  cross-package surface, documented) and self-flagged on EXC-004/DECL-006.

## Boundary-compliance statement

- All fences held. Discovery was READ-ONLY outside the two output files
  (`CLAIM_CONCORDANCE_DEL-03-07.csv`, `NOTES_DEL-03-07.md`, both under
  `RUN/WAVES/W1/`). A generator script was written only to the session scratchpad
  (outside both the RUN and FROZEN trees).
- No lifecycle transition applied; no DAG mutation; no cross-project edit; no edit
  to any `_STATUS.md`, register, or product file.
- F-PIP-1..5 respected: no release-readiness, issuance, certification, sealing,
  professional-approval, or code-compliance claim appears in the ledger or notes.
  Dispositions are agent judgments routed via `AuthorityNeeded`, never phrased as
  owner or engineering rulings.
- Frozen evidence tree clean: `git -C <FROZEN> status --porcelain` empty before
  and after all reads and the sandboxed pytest re-execution; HEAD verified at
  `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
- No agent-workflow redesign proposed; no STOP-worthy contradiction found.
