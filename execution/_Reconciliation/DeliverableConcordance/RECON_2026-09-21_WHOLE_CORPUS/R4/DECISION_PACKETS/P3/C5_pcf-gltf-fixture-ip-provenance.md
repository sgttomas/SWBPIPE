# C5 — IP and provenance: the PCF fixture's dimensional values, and the PCF and glTF fixtures' missing provenance records

Packet writer: TASK P3, R3 integration, run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
This is a proposal. It repairs, rules and changes nothing. It does not restate
the fixture's values or the published table entry, and it is not evidence that
protected content was copied.

`RUN` = `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.
`F:` = the freeze `projects/chirality-piping/` at `00115c71931bcae79909602d653740d3bb72dfa1`.

## 1. Decision

1. **Boundary interpretation.** The invented PCF fixture pairs a nominal-size
   label with outside-diameter and wall-thickness values that coincide with one
   entry of a published dimensional table. Does the IP boundary's rule against
   copied dimensional tables reach single coinciding values, and if so, keep,
   replace or quarantine the fixture?
2. **Provenance.** The PCF and glTF fixtures carry no fixture-provenance or
   contributor-certification record. Who creates those records, to which
   template, and does the missing record make the fixtures non-public data under
   the boundary text until a maintainer disposition exists?

**Holders.** Item 1: OWNER (or the maintainer the owner names) for the
interpretation of the boundary rule; T7 notes the review may escalate to
OWNER_DECISION if it cannot settle the rule. Item 2: WORKING_ITEMS (workflow:
review) for the maintainer review disposition once C2 sets the record standard;
OWNER only if the boundary text's status is in question (C6).

## 2. Background

- **Boundary text.** `F:docs/IP_AND_DATA_BOUNDARY.md` (`status: draft`, line 4)
  lists dimensional or rating tables copied from standards among excluded
  content (line 52). Line 72 says records with unknown redistribution status or
  missing contributor certification are not acceptable as public data until a
  maintainer records a review disposition. §4 (line 58) lists the required
  provenance fields. No ruling located settles whether single values fall under
  the table rule (T7-C09 description).
- **SOW requirements.** DEL-17-07 REQ-040 (fixtures invented or
  redistribution-safe with documented provenance) and REQ-041 (fixtures do not
  copy protected tables and similar content), at the DEL-17-07 `ScopeOfWork.md`
  lines 227 and 228 (ledger NormativeSource). CLM-045 carries the same check for
  examples.
- **What is on disk at the freeze** (listed by this writer):
  `F:fixtures/pcf_export/invented/` holds `model.pcf`, `pcf_export_package.json`
  and `source_pcf_payload.json`; `F:fixtures/review_geometry/invented/` holds
  `model.gltf`, `review_geometry_export_package.json` and
  `source_centerline_payload.json`. Neither folder holds a provenance or
  contributor-certification record.
- **Prior rulings.** DEC-058 (D-20) governs the release-time scan, which has not
  run (see C2). No ruling on these fixtures was located.

## 3. Options

**Item 1 — dimensional coincidence.**
- (a) Single coinciding values fall under the table rule. Replace the values with
  clearly invented ones and re-run the fixture tests; the row then closes on
  re-verification.
- (b) Single values do not fall under the rule when no table is reproduced.
  Record that interpretation and the review disposition; the fixture stays.
- (c) Quarantine the fixture from public use until a maintainer review settles
  it (the boundary text's quarantine route, line 84).
- Consequences: (a) is an H2 fixture change touching the PCF export tests and
  package hashes; (b) is a boundary-interpretation record, possibly a boundary
  text amendment; (c) blocks the PCF invented example until reviewed.

**Item 2 — provenance records.**
- (a) Create fixture-provenance records to the §4 template for both fixture sets
  (fixture ID, contributor certification, redistribution status, review
  disposition), with a maintainer review disposition.
- (b) Treat the fixtures as project-authored and record only a maintainer
  disposition, without per-fixture certification.
- Consequences: (a) is evidence and record work, then re-verification of the
  DEL-17-07 and DEL-17-08 rows; (b) needs an owner reading that project-authored
  fixtures are exempt from the template, which the draft text does not state.
- **Who makes the records.** Under (a) or (b) the maintainer disposition is
  written by the review WORKING_ITEMS runs (§1 holders); no option in the
  evidence assigns it elsewhere.
- **Public status until a record exists.** This part follows C6 M5. If the
  draft boundary text governs, line 72 makes the fixtures non-public data until a
  maintainer disposition exists: (i) keep them in the public tree pending the
  disposition, with the gap recorded; or (ii) quarantine them (line 84) until
  it exists. If the draft is evidence only, no public-status consequence follows
  from the missing record alone.

## 4. Evidence and reliability

| Source | Reliability | Shows |
|---|---|---|
| `RUN/WAVES/W3/PKG-17/PKG-17_VERIFICATION.md` §7 item 3 | Verifier report | Both items; maintainer review needed |
| `RUN/WAVES/W3/W3_ASSESSMENT.md:91` | Agent 0 | Owner item |
| Sealed ledgers `WAVES/W3/PKG-17/DEL-17-07`, `DEL-17-08` | Worker judgment, verifier-sampled | Dispositions and RemainingWork |
| `RUN/R3/TASKS/T7_CLASSES.md` T7-C09 | R3 proposal | AUTHORITY_UNCLEAR reading of REQ-041 and CLM-045 |
| `F:docs/IP_AND_DATA_BOUNDARY.md` lines 4, 52, 58, 72, 84 | Draft policy (verified by this writer) | Rule text and draft status |
| Freeze listing of `fixtures/pcf_export/invented/` and `fixtures/review_geometry/invented/` | Verified by this writer | No provenance record on disk |

Known only from the worker and verifier reading: the coincidence itself. This
writer did not compare the values with any table and does not reproduce either.

## 5. Affected claims

This topic comes from the W3 owner list; the topic file assigns it no class.
Its rows sit in classes that a packet or handoff already counts, so C5 claims
**no class portion** and lists its rows by key. Handoffs carry
`BlockedOnPacket C5` on them.

| Key | Effective | Class (counted by) |
|---|---|---|
| `DEL-17-07:SOW#CLM-017/DEL-17-07-REQ-041` | UNKNOWN · AUTHORITY_UNCLEAR · INVARIANT · REVIEW | T7-C09 (C2) |
| `DEL-17-07:SOW#CLM-045` | UNKNOWN · AUTHORITY_UNCLEAR · INVARIANT · REVIEW | T7-C09 (C2) |
| `DEL-17-07:SOW#CLM-017/DEL-17-07-REQ-040` | PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · REVIEW | T6-C05 (H3) |
| `DEL-17-07:SOW#CLM-007` | same | T6-C05 (H3) |
| `DEL-17-07:SOW#CLM-025` | same | T6-C05 (H3) |
| `DEL-17-07:SOW#CLM-031` | same | T6-C05 (H3) |
| `DEL-17-07:SOW#CLM-034` | same | T6-C05 (H3) |
| `DEL-17-07:SOW#completion-and-reliance-basis-epistemology/AC-001` | same | T6-C05 (H3) |
| `DEL-17-07:SOW#production-and-verification-method-praxeology/VER-001` | same | T6-C05 (H3) |
| `DEL-17-08:SOW#production-and-verification-method-praxeology/VER-001` | same | T6-C05 (H3) |
| `DEL-17-07:SOW#CLM-021` | PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · LOCAL_DESIGN · NO | T6-C01 (H2) |

- Reproducing filter: sealed forward ledgers of DEL-17-07 and DEL-17-08 where
  Notes, ClaimSummary or RemainingWork mention fixture provenance or dimensional
  values, restricted to `Divergent == YES` in `CORPUS_CLAIMS.csv` (11 rows).
- Only one glTF-side row (`DEL-17-08` VER-001) carries the provenance gap; the
  DEL-17-08 ledger mints no separate fixture-provenance requirement row.
- **Packages / deliverables.** PKG-17: DEL-17-07, DEL-17-08.
- **OtherCorrections.** None on these rows. No resolution rows.

## 6. Risks

- **Undecided.** If the coincidence falls under the rule, a public fixture
  carries protected data until the release scan. Missing provenance leaves both
  fixture sets unacceptable as public data by the boundary text's own terms,
  while they are already in the public tree and in test paths.
- **Option 1(a).** Changes package hashes and test expectations.
- **Option 1(b).** Sets an interpretation that other fixtures will rely on.
- **Option 2(b).** Exempts project-authored fixtures by reading, not by rule.

## 7. Recommended routing

No recommendation; owner's call on item 1. On item 2, the evidence supports
that a provenance record and maintainer disposition are what the SOW rows ask
for (their RemainingWork says so); the form of that record waits on C2.

## 8. On-ruling mechanism

- **1(a).** An H2 fixture-replacement brief under a production brief,
  chirality-change PR path with independent review; the DEL-17-07 rows change on
  re-verification.
- **1(b).** An owner or maintainer interpretation recorded in the register; if it
  should bind generally, a boundary-text amendment through the governance path.
- **1(c).** A quarantine record under the boundary text; the fixture leaves public
  paths through H2.
- **Public status (i)/(ii).** (i) no action beyond the review; (ii) a quarantine
  record and removal from public paths through H2.
- **2(a)/(b).** A review under WORKING_ITEMS (workflow: review) writing the
  provenance records and disposition; rows change on re-verification.
- Nothing executes until the owner acts. R5 needs separate authorization.

## 9. Dependencies

- **Depends on:** C2 (review-record standard); C6 (whether the draft
  `IP_AND_DATA_BOUNDARY.md` governs).
- **Blocks:** the H3 review of the T6-C05 DEL-17-07/08 rows listed in §5 and
  the H2 T6-C01 row `DEL-17-07:SOW#CLM-021` (BlockedOnPacket C5). C5 blocks no
  other DEL-17-07 row, including no Remaining item.
- **Related:** A7 (export plan; PCF source-basis refs); H2 PCF code items (zero
  coordinate, missing disclosure file) are separate.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
