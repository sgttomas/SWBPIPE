# NOTES — DEL-06-01 Rule-pack schema (W3, D-41 concordance)

Deliverable: **DEL-06-01** (PKG-06, IN_PROGRESS). Frozen source SHA
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`CLAIM_CONCORDANCE_DEL-06-01.csv` (19 rows, 20 columns, RFC-4180 / CRLF).

NormativeSource path alias (declared once per addendum 12): kit files are
under `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-01_Rule-pack schema/`;
product surfaces (`schemas/`, `tests/`, `examples/`, `docs/`, `core/`) are
repo-relative under `projects/chirality-piping/`. ClaimID uses the addendum-12
fixed form `DEL-06-01-<TYPE>-NNN`; the native requirement ID (`REQ-06-01-00N`)
is carried in `NormativeSource`.

## 1. Histograms (recomputed from the CSV)

Disposition histogram:
- ALIGNED — 15
- STALE_SETUP_SPECIFICATION — 4
- (total 19)

ClaimType histogram:
- REQUIREMENT — 12
- DECLARED_STATE — 6
- EXCLUSION — 1
- (total 19)

Supporting counts: ClaimClass {SCHEMA 7, DOCUMENTATION 6, GOVERNANCE 5,
SECURITY 1}; AuthorityNeeded {NO 14, OWNER 5}; Confidence {HIGH 15, MEDIUM 4}.
No ACCEPTANCE, REMAINING_WORK, or IMPLEMENTED_UNMAPPED rows (rationale below).

## 2. Census reasoning

- **Requirements (12):** REQ-06-01-001..012, re-verified against the frozen
  Specification.md requirements table and the R1 `DELIVERABLE_INVENTORY.csv`
  row (scheme `REQ-06-01-*`; IDs 001–012). One substance-disposition row each
  (convention 1 — requirement rows never take STALE). All 12 are implemented
  in `schemas/rule_pack.schema.yaml` (SURF-205) and covered by
  `tests/test_rule_pack_schema.py`; all ALIGNED.
- **Acceptance (0):** the Specification "Verification" section is
  future-implementation prose and the requirements table's "Verification
  approach" column merely restates each requirement; no discrete acceptance/
  verification criteria at addendum-12 grain, so no mirrored ACCEPTANCE rows.
- **Exclusion (1):** EXC-001 captures the durable product-boundary portion of
  the Scope sentence (no sandboxed evaluator [DEL-06-02], no protected/
  proprietary rule content, no engineering code-compliance claim). The same
  sentence's setup-era "not yet implemented" portions are overtaken and are
  ledgered on the Specification declared-state row (DECL-001), following the
  R0b exemplar split (DEL-07-05 C01/C02).
- **Declared-state (6):** one per four-document kit surface (Specification,
  Datasheet, Guidance, Procedure) plus `_STATUS.md` and `MEMORY.md` (both
  carry current-state declarations). No deliverable-owned in-tree README
  exists (folder listing confirms), so no README DECL row.
- **Remaining-work (0):** `_STATUS.md ## Remaining` carries ONLY the seeded
  `(gated: D-41)` bootstrap item, which per addendum 2 goes verbatim into the
  `_STATUS.md` surface row's `RecordedRemaining` and never its own row, and is
  excluded from residual/gate/selectability analysis. The MEMORY "Open Items"
  deferrals (evaluator/completeness/private-lifecycle/public-example) are
  scope boundaries homed at DEL-06-02/03/04/05, not DEL-06-01 residuals.
- **Implemented-unmapped (0):** every material surface in this deliverable's
  orbit is already mapped. `schemas/rule_pack.schema.yaml` (SURF-205),
  `examples/rule_packs` (SURF-144), and the `core/rules/*` crates
  (SURF-117/118/119/120) all list DEL-06-01 among their owners in R1
  `IMPLEMENTATION_SURFACES.csv`; the desktop rule-pack features (SURF-043/044)
  are owned by DEL-07-03. Nothing unmapped in orbit.

## 3. Bootstrap and _STATUS handling

The bootstrap item is transcribed byte-exact into DECL-005 `RecordedRemaining`
(including `§§`, the en-dash in `6–8`, and `(gated: D-41)`). Per calibration
items 5/11 (W2-uniform exclusion variant), the `GateOrStageConstraint` and
`RemainingSource` cells for that row stay `NONE_RECORDED` (not annotated with
the bootstrap gate); `SelectableUnderCurrentLoop=NO` (no non-bootstrap
residual). DECL-005 disposition is ALIGNED (IN_PROGRESS status accurate;
History cross-checks the two LifecycleCorrection Decision_Logs and the D-41
register row).

## 4. SECURITY encoding (convention 6 / calibration item 2)

REQ-06-01-007 (formula slots without arbitrary executable code) is the schema's
user-supplied-code trust boundary and is encoded SECURITY. The schema
implements the boundary (frozen DEC-022 declarative-AST `ExpressionNode`, no
text parser, `arbitrary_code_execution_allowed` constrained false, evaluator
refusal markers `unsupported_form`/`unsafe_host_access` deliberately not
authorable) and a negative fixture rejects executable-formula flags — so the
row is **ALIGNED at schema/contract grain**. But the schema's own
`FormulaDeclaration` description states that monotone-table / dimension-algebra
/ unit-match and host-access sufficiency are **enforced by the sandboxed
evaluator (core/rules/expression_evaluator, DEL-06-02), not by this schema**.
The whole-system injection/sandbox sufficiency of the trust boundary is
therefore an owner-gated SECURITY review deferral. Applied exactly:
`ValidationEvidence=NONE_FOUND — sufficiency review deferred, owner-gated`
(em-dash marker) and `AuthorityNeeded=OWNER`; no VERIFIED_NOT_VALIDATED
downgrade on that ground. Confidence MEDIUM (forward-looking-gap grain call
per calibration item 6 — grain stated in-row). Row self-flagged below.

## 5. Rev-0.7 authority-pointer drift (calibration item 1) — owner caveat

The kit cites `SOFTWARE_DECOMP revision 0.7` (Specification Standards table,
Datasheet References, `_CONTEXT.md`, `_REFERENCES.md`) while the frozen decomp
header is `revision: 0.8, status: current_basis`. This pointer drift is
recorded in-row on DECL-001 (Specification) and DECL-002 (Datasheet). Those
two surfaces are adjudicated STALE_SETUP_SPECIFICATION on the dominant
ground — the setup-era "does not implement the schema/docs/examples" prose is
overtaken by the implemented slice — and additionally carry overtaken TBD/
future content, so `AuthorityNeeded=OWNER` (not the pointer-drift-only NO
variant). **Owner-calibration caveat (recorded once, per calibration item 1):**
the rev-0.7→0.8 pointer drift is a pure setup-era authority-pointer lag and by
itself would route `AuthorityNeeded=NO`; it is folded into the OWNER-routed
R5 repair candidate here only because these surfaces independently carry
overtaken not-implemented prose and TBD registers.

## 6. Evidence-execution log

- **Re-executed side-effect-free (addendum 9):**
  `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest <frozen>/tests/test_rule_pack_schema.py
  -p no:cacheprovider --rootdir=<scratch>` → **5 passed** (0.27s). Run from a
  scratch cwd outside the frozen tree; no `CARGO_TARGET_DIR` needed (pure
  pytest). `git -C FROZEN status --porcelain` **empty before and after**.
  This is the primary verification leg cited on the requirement/exclusion rows.
- **Cited-recorded (not re-executed):** the DEC-025 five-surface sweep
  `validation/evidence/sweeps/SWEEP_20260711T040758Z_e648462f1d05.json`
  (surface `python_pytest=pass`) bound to commit `e648462f1` (ancestor of the
  frozen SHA). Carried with the addendum-10 qualifier `content-identical at
  frozen SHA 551f84ef6be656f1603ce0acfa5e3935aa9683c7 (diff empty over tests/,
  schemas/, fixtures/, examples/)` — the R1 `VERIFICATION_INDEX.csv` PY-65 row
  already establishes this content-identity for the test/schema paths (I did
  not independently re-run a `git diff` across the fuller path list PY-65
  quotes, so I narrowed my in-row qualifier to the four paths that carry the
  rule-pack test and its inputs, and appended `not re-executed at frozen SHA
  551f84ef6`).
- The `core/rules/rule_pack_document`/`rule_pack_lifecycle` cargo suites
  (RUST-20/21, recorded pass, CONTENT_IDENTICAL) belong to the DEL-06-02/04
  seam and were not cited as DEL-06-01 verification legs.

## 7. SourceReliability keying (addendum 6 / calibration item 13)

All 12 requirement rows and the exclusion row are keyed `UNVERIFIED`: the
weakest load-bearing leg is the verification suite, which is project-original
agent-generated test evidence with a recorded (agent) sweep pass and no human
disposition covering the test record. The schema *members* are human-ratified
(DEC-022/DEC-038/DEC-039) and finding PKG06-01-PKG02-001 has a human
ACCEPT_AS_IS disposition, but those cover the implementation/finding legs, not
every load-bearing leg — so no row is lifted to REVIEWED. DECLARED_STATE prose
rows are `NOT_APPLICABLE` per addendum 6.

## 8. Convention-friction notes

- **ClaimClass split on a schema deliverable.** §6 has both `SCHEMA` and
  domain classes. I used SCHEMA for structural field/format requirements
  (001,002,003,005,006,008,010), GOVERNANCE for IP/redistribution/professional-
  boundary/agent-authority policy requirements (004,009,011,012, and EXC-001),
  and SECURITY for the user-supplied-code trust boundary (007). REQ-004
  (redistribution) and REQ-005 (checksum integrity) each have a defensible
  alternate class (SCHEMA vs GOVERNANCE; SCHEMA vs SECURITY) — I keyed them to
  the dominant concern (IP classification → GOVERNANCE; schema-field +
  JCS-hash-correctness, which the JCS recomputation test validates → SCHEMA,
  not a deferred-sufficiency SECURITY row).
- **Guidance staleness grain.** Guidance is advisory "should" prose and makes
  no false hard current-state claim, but under widened addendum 4 its OI-006
  "expression grammar/library TBD" is partly overtaken (DEC-022 froze the
  grammar; DEC-037 deferred only the writable text syntax) and its "examples,
  when later authorized" framing is overtaken by the existing invented demo.
  I adjudicated STALE (consistent with the DEL-07-05 C04 exemplar) at MEDIUM
  confidence — self-flagged.
- **MEMORY undated-block rule (calibration item 9).** The undated Boundary
  Decisions / Open Items blocks are current declarations; their DEL-06-02/03/
  04/05 deferrals and grammar/library TBD still hold, so the surface is
  ALIGNED-with-note rather than STALE. The one tension (Open Items /
  Boundary Decisions say public examples not added / assigned to DEL-06-05,
  while `examples/rule_packs/invented_demo.yaml` exists) is reconciled in-file
  by the dated 2026-06-05 hardening entry describing the demo as a schema
  validation fixture (co-owned with DEL-06-05 per SURF-144) — the note names
  that correcting entry. Self-flagged.

## 9. Self-flagged rows

- **DEL-06-01-REQ-007** — SECURITY-class, forward-looking-gap grain call:
  ALIGNED at schema/contract grain with owner-gated SECURITY sufficiency
  deferral (em-dash marker, AuthorityNeeded=OWNER). Reviewer eyes on the
  grain choice (schema-boundary vs whole-system sandbox sufficiency) and on
  the SECURITY-vs-SCHEMA class assignment.
- **DEL-06-01-DECL-003** (Guidance) — STALE vs ALIGNED-with-note is a judgment
  call on advisory prose; chose STALE (MEDIUM) on the partly-overtaken OI-006
  grammar-freeze and the existing invented example.
- **DEL-06-01-DECL-006** (MEMORY) — ALIGNED-with-note under calibration item 9;
  the public-example-vs-invented_demo distinction (fixture vs DEL-06-05
  shipped content) is the judgment call; reviewer confirmation welcome.

## 10. Boundary-compliance statement

All fences held. Discovery was read-only: the only writes are the two W3
output files (`CLAIM_CONCORDANCE_DEL-06-01.csv`, `NOTES_DEL-06-01.md`) plus the
scratch generator `build_csv_DEL-06-01.py` outside the repo. No `_STATUS.md`,
register, DAG, product, or cross-project file was touched; no lifecycle
transition applied (`LIFECYCLE_REASSESSMENT_REQUIRED` not used; STALE repair
routes are recorded as agent-judgment R5 candidates via AuthorityNeeded, not
owner/engineering rulings). No F-PIP-1..5 release/issuance/certification/
sealing/professional-approval/code-compliance claim is made anywhere. The
frozen evidence worktree porcelain was empty before and after the single
re-execution and remains empty at completion. No STOP-worthy contradiction
found (the D-41 gate-state and rev-0.7→0.8 pointer are run-level codified
mechanics per RUN_BASIS, not conflicts).
