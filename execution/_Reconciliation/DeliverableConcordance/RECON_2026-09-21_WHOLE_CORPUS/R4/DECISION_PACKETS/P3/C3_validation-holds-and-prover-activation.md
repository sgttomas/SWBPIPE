# C3 — Validation holds, external-prover activation, and the independent usability/security basis

Packet writer: TASK P3, R3 integration, run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
This is a proposal. It repairs, rules and changes nothing. It asserts nothing
about engineering adequacy: every row here is verified and not validated (A5),
and the records say so.

`RUN` = `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.
`F:` = the freeze `projects/chirality-piping/` at `00115c71931bcae79909602d653740d3bb72dfa1`.

## 1. Decision

Does the owner want to act now on any of three held validation bases, or keep
them held?
1. The **independent usability and security validation basis** for PDU-045,
   PDU-046 and PDU-049 (DEL-07-03, DEL-07-06), held under D-68.
2. **Activation of the external prover** for DEL-17-05 ACC-006, gated by DEC-080.
3. The **DEL-05-03 pressure reference model**, under an owner hold, which must
   lift before engineering can rule the model and its companion decisions.

The owner also sees the engineering validation holds on DEL-13-04, DEL-14-04
and DEL-14-05 (a W3 owner item). Those need no owner act: their holder is
ENGINEERING, through H3.

**Holders.** OWNER for items 1–3 (lifting or keeping a hold; DEC-080
activation is also reserved from agents by DEC-082's hard limits).
ENGINEERING for the named validation bases once a hold lifts, and for the
DEL-13-04/14-04/14-05 bases (H3). EXTERNAL where a procured tool or an
independent reviewer supplies the basis.

## 2. Background

- **A5** (`RUN/CONVENTIONS.md:91`): verification is not validation; tests are
  never promoted to validation.
- **D-68** (`_DECISIONS/_REGISTER.md:105`, ruled 2026-09-15): adopted the
  professional modelling workspace foundation and bounded accessibility
  criteria, with "no … independent-usability closure".
- **DEC-080** (D-47; `F:execution/_Decomposition/SOFTWARE_DECOMP.md:671`;
  register `:81`): external-prover correlation is the principal validation
  posture, with activation owner-gated on tool procurement; "no stage
  advancement or prover activation".
- **DEC-082** (D-49; register `:83`): the standing delegation excludes DEC-080
  prover, procurement and reproduction acts.
- **D-67** (register `:104`, 2026-09-14): adopted a private dormant pressure
  kernel; broader pressure runtime deferred. The DEL-05-03 pressure-reference
  investigation (2026-09-08) listed seven owner decisions (ledger Notes on
  `DEL-05-03:SOW#CLM-011/DEL-05-03-RQ-001.s01`).
- **What the code does.** The behaviours are implemented and verified by
  project tests (ledger Notes). The DEL-05-03 crate is called by
  `core/product_physics` for thin-wall membrane recovery (ledger Notes; no
  equation restated here, DEC-043). DEL-17-05 tests verify the bounded contract
  only; no live prover run exists.

## 3. Options

**Item 1 — independent usability/security basis (9 rows).**
- (a) Keep the D-68 hold. Rows stay VERIFIED_NOT_VALIDATED; no action.
- (b) Authorise and scope an independent usability and security validation
  tranche (who reviews, against which criteria, for which PDUs).
- Consequence of (b): an external or independent reviewer engagement; rows
  change on re-verification after the tranche. No record repair is due under
  either option: the records are accurate.

**Item 2 — external prover (1 row).**
- (a) Keep the DEC-080 gate closed.
- (b) Activate: authorise tool procurement and a live correlation run for ACC-006.
- Consequence of (b): an owner procurement and spend act, then a validation
  tranche. T6 observation O1 notes non-deferred record-field residue on sibling
  DEL-17-05 rows; that residue is H2 work under either option.

**Item 3 — DEL-05-03 pressure reference (2 rows).**
- (a) Keep the owner hold.
- (b) Lift the hold so engineering rules the pressure reference model and its
  companion decisions, followed by activation and validation through the
  physics-audit activation plan.

**Engineering holds (DEL-13-04 REQ-007; DEL-14-04 ×2; DEL-14-05 R01).** No owner
option: engineering names a vetted basis (benchmark, independent witness or
vetted source). The owner may only reprioritise. Recorded here so the W3 owner
item is accounted for.

## 4. Evidence and reliability

| Source | Reliability | Shows |
|---|---|---|
| `RUN/R3/TASKS/T7_CLASSES.md` T7-C07, T7-C08 | R3 proposal | Classes, holders, NO_ACTION reading |
| `RUN/R3/TASKS/T9_LIFECYCLE.md` T9-C11a/b/c; `T9_LIFECYCLE.csv` `STALE_VV:*` items | R3 proposal | Same rows as owner/engineering decisions |
| Sealed ledgers W1 PKG-07; W3 PKG-05, PKG-13, PKG-14, PKG-17 | Worker judgment, verifier-sampled | "Accurate; the hold stands at the freeze" |
| Register rows D-47, D-49, D-67, D-68 | Ruled | Holds and gates |
| `RUN/WAVES/W3/W3_ASSESSMENT.md:92` | Agent 0 | Engineering validation holds as owner item |

Verified by this writer: the register rows. Known only from ledger Notes: the
seven pressure decisions, which PDUs are verified by which tests, and "O10" not
being defined anywhere (DEL-17-05 CLM-019 Notes).

**Tasks disagree on route; both views shown, none chosen.**

| Rows | T7 view | T9 view |
|---|---|---|
| DEL-07-03 ×3, DEL-07-06 ×6 | T7-C08 · NO_ACTION (records correct; owner holds exist) | T9-C11a · OWNER_DECISION (authorise and scope the basis) |
| DEL-17-05 CLM-019 | T7-C08 · NO_ACTION | T9-C11c · OWNER_DECISION (activation) |
| DEL-13-04, DEL-14-04 ×2, DEL-14-05 | T7-C07 · ENGINEERING_AUTHORITY | T9-C11b · ENGINEERING_AUTHORITY (agree) |

The difference is whether "keep the hold" needs a fresh owner act. Under both
views, option (a) changes nothing.

## 5. Affected claims

| Class / item | Class rows | Portion | Filter |
|---|---|---|---|
| T7-C08 | 10 | 10 (whole class) | `CLASS_ASSIGNMENTS.csv` `ClassID == T7-C08` |
| T7-C07 | 7 | 2 (owner-facing part: DEL-05-03) | `ClassID == T7-C07` and `DeliverableID == DEL-05-03` |
| T9 items | 3 items | 3 | `T9_LIFECYCLE.csv` `Item` starts `STALE_VV:VNV-` |

- **T7-C08 keys.** `DEL-07-03:SOW#CLM-007/PDU-049`, `DEL-07-03:SOW#CLM-012/DEL-07-03-R-011`,
  `DEL-07-03:SOW#CLM-016`, `DEL-07-06:SOW#CLM-008.r01`, `DEL-07-06:SOW#CLM-008.r03`,
  `DEL-07-06:SOW#CLM-012/DEL-07-06-RQ-004`, `DEL-07-06:SOW#CLM-017`,
  `DEL-07-06:SOW#CLM-018`, `DEL-07-06:SOW#completion-and-reliance-basis-epistemology/AC-001`,
  `DEL-17-05:SOW#CLM-019`. All ten also appear in `R3/NO_ACTION_ROWS.csv`
  (class route NO_ACTION), which Agent 0 records.
- **T7-C07 portion.** `DEL-05-03:CONTEXT#description`,
  `DEL-05-03:SOW#CLM-011/DEL-05-03-RQ-001.s01` (BaselineClass OWNER_HOLD).
  The other five T7-C07 rows (`DEL-02-02:STATUS#remaining/R05`,
  `DEL-13-04:SOW#CLM-017/DEL-13-04-REQ-007`, `DEL-14-04:SOW#CLM-008.r02`,
  `DEL-14-04:SOW#CLM-017.s02`, `DEL-14-05:STATUS#remaining/R01`) are
  ENGINEERING and go to H3; they are cited here, not claimed.
- **T9 items.** `STALE_VV:VNV-A_independent_usability_security` (9 keys, as
  T7-C08 minus DEL-17-05), `STALE_VV:VNV-B_engineering_validation_basis`
  (4 keys, H3), `STALE_VV:VNV-C_external_prover_activation` (1 key).
- **Packages / deliverables.** PKG-05, 07, 17 (owner items); PKG-02, 13, 14
  (engineering, cited). DEL-05-03, 07-03, 07-06, 17-05.
- **OtherCorrections.** None on these rows.

## 6. Risks

- **Undecided.** Low for the records, which already state the gap. The product
  risk is that verified behaviour (private-by-default, usability, contrast,
  pressure recovery, the prover contract) is relied on as if validated.
- **Option (b) items.** Procurement and independent engagement cost; activation
  may expose correlation differences that reopen accepted work. None of that is
  a defect finding today.

## 7. Recommended routing

No recommendation; owner's call on each hold. The evidence supports that
"keep the hold" needs no record repair, and that the engineering holds route to
H3 without an owner act.

## 8. On-ruling mechanism

- **Keep (a).** Nothing executes. Agent 0 records the confirmation in
  `R3_SYNTHESIS.md` or R4 closure; rows stay NO_ACTION.
- **Item 1(b).** An owner authorisation scoping an independent validation
  tranche, executed by an EXTERNAL or independent reviewer; rows change on
  re-verification in a later concordance.
- **Item 2(b).** An owner DEC-080 activation act (procurement, then a live
  correlation run), outside agent latitude (DEC-082); then a validation tranche.
- **Item 3(b).** An owner act lifting the DEL-05-03 hold; engineering rules the
  reference model and companions (H3); activation and validation through the
  physics-audit activation plan. Nothing becomes a code fix or record repair
  until that basis exists.

## 9. Dependencies

- **Depends on:** none.
- **Blocks:** H3 items for DEL-05-03 (BlockedOnPacket C3). H2 residue on
  DEL-17-05 record fields is not blocked (T6 O1).
- **Related:** B12 (release/QA/stage gates); C7 (acceptance-workflow cause is a
  separate reading).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
