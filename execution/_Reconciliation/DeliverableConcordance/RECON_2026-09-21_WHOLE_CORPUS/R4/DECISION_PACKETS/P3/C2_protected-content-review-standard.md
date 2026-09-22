# C2 — Protected-content review: records not located, and what counts as a record

Packet writer: TASK P3, R3 integration, run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
This is a proposal. It repairs, rules and changes nothing. Nothing here is
evidence that protected content exists, or that the boundary holds.

`RUN` = `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.
`F:` = the freeze `projects/chirality-piping/` at `00115c71931bcae79909602d653740d3bb72dfa1`.

## 1. Decision

Three questions; the first two are linked, the third is separate.
1. **Evidence standard.** When a claim names a protected-content, private-data
   or fixture-provenance review, does an agent's own reading or an automated
   denylist test satisfy it, or must a dated review record exist? (T11 T-02,
   R-02.)
2. **The review round.** Given the standard, authorise a bounded
   protected-content review of the named surfaces at current bytes, or defer to
   the DEC-058 release scan.
3. **PR #787's removed protected check (DEL-04-04 REQ-08).** Restore it, or
   rule the removal intended. This is a PROTECTED_CHECK on loop-identity
   diagnostics, not protected content; it is decided independently of Q2.

**Holder: OWNER** for questions 1 and 3 (question 1 is a corpus-wide evidence-standard reading that
T11 routes OWNER_DECISION). **WORKING_ITEMS (workflow: review)** executes the
reviews once the standard is set; the class authority of T7-C09 and T5B-C02 is
REVIEW. The DEC-058 scan owner and sole signatory stay as ruled (the owner).

## 2. Background

- **DEC-058** (D-20; `F:execution/_Decomposition/SOFTWARE_DECOMP.md:649`;
  `execution/_Coordination/_DECISIONS/_REGISTER.md:53`): an owner-owned release-artifact scan and a
  sole-signatory release gate, recorded as
  `SCAN_<candidate>_<utc>_<commit12>.json` with owner sign-off. The register
  notes nothing was scanned, cleared or published under the ruling.
- **At the freeze** no file under `F:validation/evidence/` follows the DEC-058
  scan-record pattern (`release_artifacts/` holds one
  `RELEASE_ARTIFACT_20260711T030046Z_8e436704b52b.json`; checked by listing).
- **IP boundary text.** `F:docs/IP_AND_DATA_BOUNDARY.md` is `status: draft`
  (line 4). Its §4 (line 58) lists required provenance fields; line 72 says
  records with unknown redistribution status or missing contributor
  certification are not acceptable as public data until a maintainer records a
  review disposition. Whether this draft counts as governing is a C6 item.
- **R2 rulings on the evidence standard.** The W1 FIRM resolution on
  `DEL-07-02:SOW#CLM-026` (`RUN/WAVES/W1/RESOLUTIONS.csv:5`) set
  UNKNOWN · EVIDENCE_NOT_LOCATED · INVARIANT · IP_DATA for a missing
  protected-content review. R0 review F8 (`RUN/R0_CALIBRATION/R0_REVIEW.md:219`)
  found ALIGNED rows asserting such reviews without evidence. These are run
  conventions, not owner rulings on the standard itself.
- **PR #787** (commit `b43cc00c4`) removed envelope-level tests proving that
  nonlinear loop identity and the `NONLINEAR_ASSEMBLED_LOOP` diagnostics reach
  the exported result (`RUN/WAVES/W2/PKG-04/PKG-04_VERIFICATION.md` §7 item 1).
  Under A2 a merged PR carries owner intent, but no record shows the removal of
  a PROTECTED_CHECK was intended.

## 3. Options

**Question 1 — evidence standard (T11 T-02).**
- (a) An agent reading or a denylist test satisfies a claim that names a
  protected-content review. The 29 EVIDENCE_NOT_LOCATED rows could be re-read
  and may align; the ALIGNED comparators stand.
- (b) A dated review record is required. The ALIGNED comparators
  (`DEL-07-05:SOW#CLM-013/VER-07-05-002`, `DEL-07-07:SOW#CLM-013/VER-07-07-004`,
  `DEL-03-07:SOW#CLM-017.r06`, `DEL-13-01:SOW#CLM-011.r02`,
  `DEL-10-03:SOW#CLM-013/DEL-10-03-REQ-07`, `DEL-10-03:SOW#CLM-015/REQ-07`,
  `DEL-15-04:SOW#CLM-012.r04`, `DEL-17-03:SOW#CLM-011.r07`) are under-assessed,
  and one review round closes both groups.
- (c) A tiered standard: a review record for INVARIANT (IP_DATA) claims and a
  worker reading for LOCAL_DESIGN guide claims (the five PKG-11 T5B-C02 rows
  already read this way under F8). This option is implied by T5B-C02's
  tiering, not stated as an option by any task.
- Consequences: (a) conflicts with the W1 FIRM resolution and R0 F8, so those
  run conventions would need re-reading; (b) and (c) need a review round.

**Question 2 — the review round.**
- (a) Authorise one bounded protected-content and private-data review per
  fixture surface (about 11 surfaces for T7-C09; the crates, fixtures, guides
  and SOWs for T5B-C02), each writing a dated record binding the reviewed
  hashes.
- (b) Defer reviews to the DEC-058 release scan. The rows stay open until
  release.
- Consequences: (a) produces evidence only and does not change the IP
  boundary. (b) leaves INVARIANT rows resting on self-declaration and keyword
  screening until release.

**Question 3 — DEL-04-04 REQ-08 (PR #787).**
- (a) Restore the envelope-binding test through H2.
- (b) Rule the PR #787 removal intended and amend DEL-04-04 REQ-08.
- Consequences: (a) is a test-only change; (b) is a SOW amendment of a
  PROTECTED_CHECK requirement. Either combines freely with Q2(a) or Q2(b).

## 4. Evidence and reliability

| Source | Reliability | Shows |
|---|---|---|
| `RUN/R3/TASKS/T7_CLASSES.md` T7-C09 | R3 proposal | 31 rows, 11 surfaces, the DEL-17-07 exception |
| `RUN/R3/TASKS/T5B_CLASSES.md` T5B-C02 | R3 proposal | 12 overtaken-review rows |
| `RUN/R3/TASKS/T11_METHOD.md` T-02, R-02 | R3 proposal (hand-directed screen) | Split standard across 11 deliverables; shared fixtures judged both ways |
| `RUN/R3/TASKS/T9_LIFECYCLE.md` SRE-2 | R3 proposal | 7 INVARIANT overtaken rows (same as T5B-C02 INVARIANT part) |
| `RUN/WAVES/W2/PKG-04/PKG-04_VERIFICATION.md` §7 items 1–2 | Verifier report | PR #787 removal; no current solver-crate review |
| `RUN/WAVES/W3/W3_ASSESSMENT.md:90` | Agent 0 | Six PKG-14 review records not located |
| Freeze listing of `validation/evidence/` | Verified by this writer | No DEC-058-pattern scan record |

Known only from worker notes and self-reviews: that the fixture values are
invented, and the dates of the last agent self-reviews (DEL-06-05 2026-04-30 and
2026-06-05; DEL-06-01 2026-06-12, overtaken by the 2026-06-14 demo changes).
None of these is a review record.

## 5. Affected claims

| Class | Class rows | Rows discussed (not a claimed portion; route carrier H3/H2) | Filter |
|---|---|---|---|
| T7-C09 | 31 | 31 (whole class; carrier H3) | `CLASS_ASSIGNMENTS.csv` `ClassID == T7-C09` |
| T5B-C02 | 12 | 12 (whole class; carrier H3) | `ClassID == T5B-C02` |
| T7-C06 | 42 | 1 key decided here; class route stays with H2 | `ClaimKey == DEL-04-04:SOW#CLM-010/DEL-04-04-REQ-08` |

- **T7-C09 by surface.** Persistence (DEL-02-05 ×3), branch/component
  (DEL-03-04 ×1, DEL-03-05 ×1), benchmark (DEL-04-05 `CLM-013`, DEL-09-02 ×2),
  rule-pack schema and demo (DEL-06-01 ×6, OBSERVED), evaluator (DEL-06-02 ×3),
  checker (DEL-06-03 ×1, DEL-06-05 ×1 FIRM), desktop UI (DEL-07-02 ×4, FIELD ×3
  and FIRM ×1), model-state / analysis-run / state-diff (DEL-14-01 ×3,
  DEL-14-02 ×2, DEL-14-03 ×1 — the six W3 PKG-14 records), PCF (DEL-17-07
  `CLM-017/DEL-17-07-REQ-041`, `CLM-045`).
- **The two DEL-17-07 rows** are AUTHORITY_UNCLEAR. Their substance (whether a
  single published dimensional value falls under the protected-table rule) is
  decided in **C5**. C2 decides only the review-record standard for them.
  Class accounting stays here.
- **T5B-C02.** INVARIANT (7): `DEL-04-01:SOW#CLM-012/DEL-04-01-REQ-012`,
  `DEL-04-01:SOW#CLM-020`, `DEL-04-02:SOW#CLM-021`, `DEL-04-03:SOW#CLM-012`,
  `DEL-04-05:SOW#CLM-012/DEL-04-05-RQ-004`, `DEL-05-01:SOW#CLM-016/REQ-05-01-003`,
  `DEL-05-01:SOW#production-and-verification-method-praxeology/VER-001` (FIRM).
  LOCAL_DESIGN (5): `DEL-11-01:SOW#CLM-013.r03`, `DEL-11-01:SOW#CLM-020.r06`,
  `DEL-11-02:SOW#CLM-013`, `DEL-11-02:SOW#CLM-020.r03`, `DEL-11-03:SOW#CLM-014.r03`.
- **T9 item.** `STALE_VV:SRE-2_protected_content_review_overtaken`
  (`R3/TASKS/T9_LIFECYCLE.csv`, route REVIEW): its 7 rows are the T5B-C02
  INVARIANT rows above, so this packet covers it without adding rows.
- **Rows outside any class** (ALIGNED comparators, question 1 option b): the
  eight keys in §3. They are not divergent and carry no class.
- **Rows known only from `OtherCorrections`.** FIRM `DEL-06-05:SOW#CLM-020.r05`
  (AuthorityNeeded → REVIEW; sealed Notes say "Holds at the freeze", empty
  BaselineClass). FIRM `DEL-07-02:SOW#CLM-026` and FIELD ×3 on DEL-07-02
  (AuthorityNeeded → REVIEW). FIRM `DEL-05-01` VER-001 (AuthorityNeeded → REVIEW;
  shared FindingGroup with `CLM-016/REQ-05-01-003`).
- **Packages.** PKG-02, 03, 04, 05, 06, 07, 09, 11, 14, 17 (+ the eight
  comparators in PKG-03, 07, 10, 13, 15, 17).
- **Related, not claimed.** T6-C03 (protected-subject implementation remainder,
  34 rows) goes to H2/H3.

## 6. Risks

- **Undecided.** The IP/data boundary (INVARIANT) rests on self-declaration and
  keyword screening. A protected value in a public fixture would go undetected
  until the DEC-058 release scan. The same fixtures are judged ALIGNED and
  UNKNOWN in different ledgers (T11 R-02), so the corpus reads inconsistently.
  A protected check stays removed without a recorded intent.
- **Option 1(a).** Weakens a boundary INVARIANT's evidence to an agent's own
  reading, which R0 F8 already found unreliable.
- **Option Q2(b).** Defers every INVARIANT review to a release that has no date.

## 7. Recommended routing

- Question 1: no recommendation; owner's call. The evidence shows the split
  exists; it does not settle which standard the owner wants.
- Question 2: no recommendation; owner's call.
- Question 3: the evidence supports restoring or re-ruling the DEL-04-04 check
  before any record repair, because the row is VERIFICATION_REMOVED on a
  PROTECTED_CHECK. Whether to restore or amend is the owner's call.

## 8. On-ruling mechanism

- **Q1 ruling.** Recorded as an owner reading in the register (or a convention
  amendment through HELPS_HUMANS if it is to bind later runs). Under (a), a later
  concordance re-reads the 29 rows; under (b)/(c), the comparators are
  re-verified after the review round. No ledger is patched (F6).
- **Q2(a) reviews.** A review under WORKING_ITEMS (workflow: review), once per
  surface, writing a dated record binding reviewed hashes; it uses the DEC-058
  scan owner and signatory where the release path applies. Rows change on
  re-verification. Any problem found becomes a separate repair.
- **Q2(b).** Nothing executes now; the rows wait for the DEC-058 release scan.
- **Q3(a).** An H2 test-restoration brief under a production brief,
  chirality-change PR path with independent review. **Q3(b):** SOW amendment of
  DEL-04-04 REQ-08 through the change path, then R5.
- Nothing executes until the owner acts. R5 needs separate authorization.

## 9. Dependencies

- **Depends on:** C6 (whether the draft `IP_AND_DATA_BOUNDARY.md` §4 governs).
- **Blocks:** C5's review step for the PCF fixture; H3 review items for T7-C09
  and T5B-C02 (BlockedOnPacket C2); the H2 DEL-04-04 brief (BlockedOnPacket C2).
- **Related:** A8 (pre-release legal review is a separate governing-source
  question).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
