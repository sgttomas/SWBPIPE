# C4 — The DEL-11-01 user-guide exception, and the DEL-11-04 professional-boundary list against DEC-081/DEC-107

Packet writer: TASK P3, R3 integration, run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
This is a proposal. It repairs, rules and changes nothing.

`RUN` = `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.
`F:` = the freeze `projects/chirality-piping/` at `00115c71931bcae79909602d653740d3bb72dfa1`.

## 1. Decision

Two claims-and-scope questions from the W3 owner list
(`RUN/WAVES/W3/W3_ASSESSMENT.md:85-86`).
1. **DEL-11-01.** How far does the R18 "guide-only SOW exception", with
   DEC-107 (iii), reach? Does it cover the SOW's read-only text on
   `docs/user_guide/index.md`, including the guide edits made before either act?
2. **DEL-11-04 R-DEL-11-04-002.** Is the requirement to add a certification,
   approval, sealing and code-compliance non-claim to the public fixture notices
   still wanted, or was it overtaken by the DEC-081/DEC-107 instruction not to
   compose ad-hoc prohibition lists?

**Holder: OWNER** for both. Item 1 is a confirmation of reach under A3a (an
owner record already exists). Item 2 is a claims-language reading the
conventions do not settle.

## 2. Background

**Item 1.**
- The DEL-11-01 SOW treats `docs/user_guide/index.md` as read-only for the
  deliverable and limits writes to the deliverable folder
  (`F:execution/PKG-11_…/DEL-11-01_…/ScopeOfWork.md:61`, `:249`, per T5B-C09).
- The guide was created 2026-05-09 and edited 2026-06-07 under DEL-11-01.
- An A3a owner-direction record
  (`…/HELP-HUMAN-PIPING-20260725-CANDIDATE-BRIEFS-R18/instances/WI-PKG11-DEL1101-EXECUTION/OWNER_ADOPTION.md`)
  names a four-path fence and a guide-only SOW exception. The PKG-11 verifier
  recomputed its hash and found it matches the bound hash
  (`RUN/WAVES/W3/PKG-11/PKG-11_VERIFICATION.md` §6, worker-raised items).
- DEC-107 (`F:execution/_Decomposition/SOFTWARE_DECOMP.md:698`) item (iii)
  directs edits to the user guide and leaves the live SOW untouched.
- Both acts postdate the first guide edits. The four rows are
  ACCEPTED_DIVERGENCE · SCOPE_GREW_BY_DIRECTION · OWNER.

**Item 2.**
- The requirement asks that fixture notices state non-suitability for reliance,
  certification, approval, sealing or code-compliance claims (PRD §21.2).
  Both invented fixtures under `F:examples/models/invented/` state invented,
  non-code, educational and not-for-reliance, but not the other four nouns
  (PKG-11 verifier W-2).
- `F:docs/claims_registry.md:219` (DEC-081, extended by DEC-107) tells agents
  not to compose ad-hoc prohibition lists, and line 99 retires multi-noun
  prohibition litanies on product surfaces.
- DEC-081 Wave 2 (commit `8fac6631a`) edited this very SOW row, only appending
  "(PRD §21.2)", and kept the requirement (PKG-11 verifier W-2).
- The row is CONTESTED (no right values stated) and FIELD (AuthorityNeeded
  NO → OWNER, because RemainingWork offers two courses).

## 3. Options

**Item 1.**
- (a) Confirm reach: the exception and DEC-107 cover the read-only text and the
  pre-exception edits. The four rows stand as accepted divergence; no repair.
- (b) Direct a SOW catch-up so the SOW records the guide as in scope. This is an
  R5 record repair that DEC-107's leave-the-SOW-untouched instruction currently
  prevents, so it also needs an owner instruction lifting that part of DEC-107.
- (c) Confirm reach from the R18 record onward only; treat the 2026-05-09 and
  2026-06-07 edits as history with no further action. (Implied by the verifier's
  dating; not proposed by a task.)

**Item 2.**
- (a) The requirement stands. Add the non-claim to both fixture notices and the
  fixture test. This composes the kind of list the claims registry retires.
- (b) The requirement was overtaken by DEC-081/DEC-107. Amend the requirement to
  cite the governed boundary statement (the registry's canonical texts) instead
  of a list. The row becomes a stale requirement
  (SCOPE_REDIRECTED_BY_RULING · PROJECT_BASELINE, the verifier's candidate).
- (c) Narrow the requirement to the nouns the fixtures already carry (the
  ledger's second RemainingWork option).
- Consequences: (a) is a code-and-fixture change (H2); (b) and (c) are SOW
  amendments followed by R5.

## 4. Evidence and reliability

| Source | Reliability | Shows |
|---|---|---|
| `RUN/WAVES/W3/PKG-11/PKG-11_VERIFICATION.md` W-2, D-2, §6, §7 items 1–2 | Verifier report | Both questions, the hash check, the DEC-081 Wave 2 edit |
| `RUN/R3/TASKS/T5B_CLASSES.md` T5B-C09 | R3 proposal | DEL-11-01 confirmation under A3a |
| `RUN/R3/TASKS/T6_CLASSES.md` T6-C03 | R3 proposal | DEL-11-04 row as protected-subject remainder |
| `F:docs/claims_registry.md:99`, `:219` | Governing text (DEC-081/DEC-107), verified by this writer | The authoring instruction |
| DEC-107 row, `SOFTWARE_DECOMP.md:698` | Ruled, verified line | Guide edits and SOW untouched |

Known only from the verifier's reading: the guide's creation and edit dates,
the fixture notice contents, and the owner record's hash match. This writer did
not open the owner record.

## 5. Affected claims

| Class | Class rows | Portion | Filter |
|---|---|---|---|
| T5B-C09 | 5 | 4 (DEL-11-01 part) | `CLASS_ASSIGNMENTS.csv` `ClassID == T5B-C09` and `DeliverableID == DEL-11-01` |
| T6-C03 | 34 | 1 key decided here; class route stays with H2/H3 | `ClaimKey == DEL-11-04:SOW#CLM-011/R-DEL-11-04-002` |

- **T5B-C09 portion keys.** `DEL-11-01:SOW#CLM-010.s02`, `DEL-11-01:SOW#CLM-014.s02`,
  `DEL-11-01:SOW#CLM-017`, `DEL-11-01:SOW#CLM-018.r05` (FG-DEL-11-01-01).
  The fifth T5B-C09 row, `DEL-07-02:SOW#CLM-034`, is the DEL-07-02/07-03 editor
  boundary and belongs to **B4**. The topic file does not list T5B-C09 as a split
  class; this is reported to Agent 0.
- **DEL-11-04 row.** Effective PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT ·
  NONE · CLAIMS, AuthorityNeeded NO.
- **OtherCorrections.** CONTESTED (candidate stale requirement,
  SCOPE_REDIRECTED_BY_RULING · PROJECT_BASELINE) and FIELD (AuthorityNeeded
  OWNER) on the DEL-11-04 row; `RUN/WAVES/W3/RESOLUTIONS.csv` holds the key twice
  (CONTESTED and FIELD), as W3 consistency notes.
- **Packages / deliverables.** PKG-11: DEL-11-01, DEL-11-04.

## 6. Risks

- **Undecided.** Item 1: the SOW contradicts its own artifact, and a later
  record repair could either erase a sanctioned exception or ratify edits made
  before it. Item 2: an agent may carry out the first RemainingWork option and
  put a retired litany into public fixtures, or leave a kept requirement unmet.
- **Item 1(b).** Conflicts with DEC-107 (iii) unless the owner lifts that part.
- **Item 2(a).** Re-introduces the pattern DEC-081 was ruled to remove.
- **Item 2(b).** Retires a requirement DEC-081 Wave 2 visibly kept.

## 7. Recommended routing

- Item 1: no recommendation; owner's call. The owner record exists and is
  verified; only its reach is open.
- Item 2: no recommendation; owner's call. The evidence points both ways (the
  registry instruction against the Wave 2 retention).

## 8. On-ruling mechanism

- **Item 1(a)/(c).** An owner confirmation recorded at R4; rows stay
  ACCEPTED_DIVERGENCE. No R5 work.
- **Item 1(b).** An owner instruction amending DEC-107 (iii) scope, then an R5
  record repair of the DEL-11-01 SOW through the ordinary change path.
- **Item 2(a).** An H2 fixture-notice and test brief under a production brief,
  chirality-change PR path with independent review.
- **Item 2(b)/(c).** SOW amendment of R-DEL-11-04-002 through the change path,
  then R5 record repair. If the owner wants the reading to bind other SOW
  prohibition lists, a claims-registry amendment through HELPS_HUMANS.
- Nothing executes until the owner acts. R5 needs separate authorization.

## 9. Dependencies

- **Depends on:** none.
- **Blocks:** H2 and H3 handling of `DEL-11-04:SOW#CLM-011/R-DEL-11-04-002`
  (BlockedOnPacket C4); any H4 repair of the four DEL-11-01 rows.
- **Related:** B4 (the other T5B-C09 row); A4 (rename residue in PKG-11 guides,
  PKG-11 verifier §7 item 3).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
