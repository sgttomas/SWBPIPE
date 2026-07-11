# NOTES — DEL-01-04 Professional responsibility and product-claims policy (W1)

Run: DELIVERABLE_CONCORDANCE_2026-07-11_1305, R2 wave W1. Frozen evidence tree
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`. Ledger:
`WAVES/W1/CLAIM_CONCORDANCE_DEL-01-04.csv` (18 rows).

**Run-level NormativeSource path alias (addendum 12, declared once):**
`KIT` = `execution/PKG-01_Governance, IP Boundary, and Professional
Responsibility/1_Working/DEL-01-04_Professional responsibility and
product-claims policy` (relative to `projects/chirality-piping/`).

**Requirement-ID mapping (dispatch note):** the deliverable's self-identifying
scheme is `DEL-01-04-R01`–`R10`; ledger ClaimIDs use the addendum-12 form —
R01→`DEL-01-04-REQ-001`, R02→`REQ-002`, R03→`REQ-003`, R04→`REQ-004`,
R05→`REQ-005`, R06→`REQ-006`, R07→`REQ-007`, R08→`REQ-008`, R09→`REQ-009`,
R10→`REQ-010`.

**Fence note (F-PIP-2 adjacency):** this deliverable is the policy home of the
professional-claims fence. Every certification/sealing/approval/compliance
phrase appearing in the ledger is a quotation or description of the policy's
own prohibition text, never a claim by this audit; this audit makes no
release-readiness, issuance, certification, sealing, professional-approval, or
code-compliance claim about anything.

## 1. Histograms (recounted from the CSV before writing)

Disposition histogram (18 rows):

| Disposition | Count |
|---|---|
| ALIGNED | 15 |
| STALE_SETUP_SPECIFICATION | 2 |
| ACCEPTED_DIVERGENCE | 1 |

ClaimType histogram (18 rows):

| ClaimType | Count |
|---|---|
| REQUIREMENT | 10 |
| DECLARED_STATE | 6 |
| ACCEPTANCE | 1 |
| EXCLUSION | 1 |

No REMAINING_WORK rows: the only `## Remaining` item is the seeded
`(gated: D-41)` bootstrap item, recorded verbatim solely in DECL-005's
`RecordedRemaining` per addendum 2 (its source/gate text therefore appears
only inside that cell; RemainingSource/GateOrStageConstraint on that row stay
`NONE_RECORDED` per the brief's "verbatim ONLY into RecordedRemaining"
reading). No IMPLEMENTED_UNMAPPED rows: both repo-level policy artifacts are
mapped to DEL-01-04 by front-matter `deliverable_id` and the Datasheet
Construction table; the enforcement surfaces cited as corroboration
(analysis_status schema, pkg02_boundary, reporting crates,
protected_content_linter) are mapped to their own PKG-02/PKG-07/PKG-08
deliverables and are outside this deliverable's unmapped orbit.

## 2. Self-flagged rows

- **DEL-01-04-ACC-001** — acceptance-grain judgment call: of the six
  Specification Verification-table checks, five restate R01/R02/R03/R05/R08
  (no mirrored ACCEPTANCE rows per the brief); I ledgered only "Human wording
  approval" as a distinct acceptance criterion because its pass condition
  (exact notice text accepted by the human project authority before repo-level
  publication) is a gate no single requirement row carries. Reviewer eyes
  welcome on whether zero or one ACC row is the intended addendum-12 grain.
- **DEL-01-04-DECL-002** and **DEL-01-04-DECL-004** — staleness materiality
  call: Datasheet and Procedure present revision 0.7 / DAG-006 as the current
  approved basis, while the frozen tree carries revision 0.8 (SCA-005) and
  DAG-007 (approved 2026-06-16), and the repo-level policy docs themselves
  cite DAG-007. I read this as post-alignment drift under the widened
  convention-4 definition. Counter-reading: SOFTWARE_DECOMP v0.8's own §13
  posture ("downstream surfaces may be stale relative to revision 0.8 until
  refreshed by their owning workflows") could support ACCEPTED_DIVERGENCE;
  I did not adopt that because the v0.8 text is a descriptive warning, not a
  named ruling that *permits* these two surfaces' deferral (addendum 11 bar).
- **DEL-01-04-EXC-001** — the Specification exclusion says "this current-basis
  refresh does not edit repo-level policy files", yet repo-level policy files
  were edited the same day by the separately human-approved
  PKG-01-NONISSUED-GOVERNANCE-REFRESH tranche. I read the exclusion as scoped
  to the current-basis refresh itself (which stayed deliverable-local) and
  ledgered ALIGNED; flagged in case a reviewer reads the exclusion wider.
- **DEL-01-04-REQ-002** — PB §6 uses prose concept labels ("Mechanics
  solved"), not the literal TYPES.md tokens; I treated the one-to-one concept
  mapping as satisfying the R02 cross-check.

## 3. Evidence-execution log

Re-executed (side-effect-free, read-only text scans/reads inside the frozen
tree; `git -C FROZEN status --porcelain` empty BEFORE and AFTER every batch,
final check empty at close):

- Prohibited-term scan (`certif|seal|approv|authentic|endors|complian`,
  case-insensitive) over `docs/PROFESSIONAL_BOUNDARY.md` and
  `docs/report_notice_template.md` at frozen SHA: 16 + matching lines in PB,
  all in negated/prohibition-list context (§1 self-notice, §3 "must not
  state" column, §5 prohibited list, §2 license-notice negation); template
  matches all inside the "does not provide" Required Notice sentence and the
  license-notice negation. No affirmative software/agent authority claim.
- `CODE_COMPLIANT` token scan over both docs: zero hits.
- Status-vocabulary scan: all four §6 concepts present in PB; the four tokens
  (`MECHANICS_SOLVED`, `USER_RULE_CHECKED`, `HUMAN_REVIEW_REQUIRED`,
  `HUMAN_APPROVED_FOR_PROJECT`) defined in `docs/TYPES.md`;
  `schemas/analysis_status.schema.yaml` AnalysisStatusVocabulary enum matches.
- R03 notice-content scan: user-supplied-data and competent-human-review
  sentences present in the template Required Notice.
- Front-matter reads: both docs `status: draft`, `deliverable_id: DEL-01-04`.
- Schema/enforcement reads: `analysis_status.schema.yaml`
  `$defs.ProfessionalBoundary` const-false claim flags +
  `human_review_required: const true` + `HumanAcceptanceRecord` requires
  `bound_hashes`; `core/gui/pkg02_boundary.py` `forbidden_software_claims`
  list and `human_acceptance_invalidates_on_hash_change=True`;
  `core/reporting/report_sections` / `report_generator` `preserves_boundary()`
  blocking paths (read only, cited as corroboration owned by other
  deliverables).
- Anchor resolution: SOW-034/SOW-064 rows in `docs/_Registers/ScopeLedger.csv`;
  Deliverables.csv row DEL-01-04; OPS-K-AUTH-1/-2, OPS-K-MECH-2,
  OPS-K-REPORT-1/-2, OPS-K-GOV-3, OPS-K-AGENT-1/-4, OPS-K-IP-1 in
  `docs/CONTRACT.md`; both LifecycleCorrection Decision_Log paths;
  `_LATEST.md` → DAG-007; SOFTWARE_DECOMP revision 0.8.
- `git log` over both policy docs at the frozen tree: post-migration commits
  `294c0f1ba` and `1b32c8096` touch both docs; the 2026-06-04 formal review
  therefore binds a pre-`1b32c8096` content state.

Cited as recorded (not re-executed):

- `KIT/_REVIEW.md` formal review 2026-06-04 (PASS_WITH_WARNINGS; prohibited
  claim scan, status vocabulary scan, report-notice completeness, human gate
  trace, protected-content screen, design-engine boundary all PASS), run
  record `TASK_RUN_2026-06-04_DEL-01-04_formal-review.md` — overtaken for the
  frozen doc content (see git log above), so every row also cites my
  bind-current re-executed scan; marker `not re-executed at frozen SHA
  551f84ef6` carried in the cells.
- 2026-06-04 human deferral ruling
  (`TP-PKG01-CHECKING-TRANSITION-DEL-01-04_2026-06-04.md`;
  `Review_Findings.csv` DEL-01-04-REV-W001 DEFERRED_BY_HUMAN_RULING).
- MEMORY.md write-scope/boundary attestations for the 2026-05/2026-06 passes.

No builds or test suites were run; nothing this deliverable claims requires
one (DOC_UPDATE), so no CARGO_TARGET_DIR/pytest sandboxing was needed.

## 4. Convention friction notes

- **Addendum 1 census vs. product-doc surfaces:** the two repo-level policy
  docs carry declared-state front matter (`status: draft`) but are the
  deliverable's *implementation* artifacts, not kit surfaces or READMEs. The
  census gives them no DECLARED_STATE row; their draft posture is instead
  audited under REQ-006/REQ-009. A future addendum could say explicitly
  whether `deliverable_id`-stamped product docs get surface rows.
- **Bootstrap-item columns:** addendum 2 says the item goes "verbatim ONLY
  into RecordedRemaining"; I left RemainingSource and GateOrStageConstraint
  `NONE_RECORDED` on DECL-005 rather than copying the item's internal
  source/gate suffix into those columns. Worth one sentence in a future
  convention so W-wave ledgers stay uniform.
- **R1 IMPLEMENTATION_SURFACES gap (index observation, not a defect I can
  repair):** `docs/report_notice_template.md` is SURF-142, but
  `docs/PROFESSIONAL_BOUNDARY.md` has no row in
  `IMPLEMENTATION_SURFACES.csv` despite being a mapped, front-matter-stamped
  implementation artifact of DEL-01-04. Flagged for the wave reviewer /R3;
  it does not affect this ledger (the doc is fully mapped, so no UNMAP row).
- **Acceptance grain** (see self-flag on ACC-001).

## 5. Boundary-compliance statement

- Writes confined to exactly two files:
  `WAVES/W1/CLAIM_CONCORDANCE_DEL-01-04.csv` and `WAVES/W1/NOTES_DEL-01-04.md`
  (plus a scratch build script outside both trees). No edits to any
  `_STATUS.md`, register, product file, DAG, or lifecycle surface.
- Frozen tree strictly read-only: `git -C FROZEN status --porcelain` empty
  before and after all evidence work.
- F-PIP-1..5 held: no release-readiness, issuance, certification, sealing,
  professional-approval, or code-compliance claim is made in either output;
  forbidden phrases appear only as attributed quotations of the policy's
  prohibition text.
- All dispositions are agent judgments; authority routed via
  `AuthorityNeeded` (single OWNER route on ACC-001). No lifecycle transition
  recorded or applied; no DEFERRED_AGENT_WORKFLOW matters arose.
- Owner suspension treated as run-level caveat only; all
  `SelectableUnderCurrentLoop` cells are mechanical (`NO` everywhere — no
  non-bootstrap recorded item exists).
