# A6 — ISSUED DEL-01-01: change path

PROPOSAL for the R4 gate (R3 integration, TASK P1). It decides nothing and changes no row or state.

Conventions: `F:` is `{FREEZE}/projects/chirality-piping/` at `00115c719`. `RUN/` is the run folder. `DEL-01-01` is the Governance baseline deliverable (PKG-01).

## 1. Decision

DEL-01-01 is ISSUED (2026-06-03). Its SOW text was true at issuance and has since been overtaken by rulings, edited once after issuance (DEC-081 Wave 2), and carries rename residue and stale pins. The owner decides one change path for every DEL-01-01 edit: **reissue, amend on the ISSUED change path, or keep the text as issuance history**, and confirms whether the DEC-081 Wave 2 edit validly reached the ISSUED SOW. This packet also owns the single route for `DEL-01-01:SOW`, including its rename residue.

**Holder: OWNER**, with the change carried by **WORKING_ITEMS (workflow: scope-change)**: the reconciliation profile routes any change to an ISSUED deliverable through the governed scope-change process.

## 2. Background

- **Issuance.** `F:execution/PKG-01_…/DEL-01-01_…/_STATUS.md:3` reads ISSUED; l.14 records the human approval (licence selection, RF-001, CHECKING to ISSUED) and that remaining governance TBDs "may be resolved later or not at human discretion". l.15 records the 2026-07-16 DEC-081 claims-language edit to `ScopeOfWork.md` (D-48 Wave 2, commit `8fac6631a`).
- **Rulings that overtook the text.** DEC-027 (2026-06-11; sole maintainer and release authority, quorum one; `F:execution/_Decomposition/SOFTWARE_DECOMP.md:618`); DEC-057 and DEC-089 (v0.1 unsigned with checksum and release-record attestation; `:648`, `:680`); DEC-081 (claims language; `:672`, which says its execution is "not an SCA — no PRD text or scope-item change"); DEC-105 (maturity sentence retired; `:696`); DEC-101 (rename; `:692`). Still open: the legal review process (DEC-079, `CLM-018.s02`) and the maturity sentence (T9-C05).
- **Run rules.** C6(d) and R0 ruling item 4: an ISSUED claim true at issuance but overtaken is LIFECYCLE_REASSESSMENT_REQUIRED with a FindingGroup, and goes to R4 as one item on the ISSUED change path (`RUN/CONVENTIONS.md` C6(d)). C6(e) / ruling item 3: rename residue in SOWs is a finding. C6(g): ACCEPTED_DIVERGENCE needs a named ruling or an A3a record. The profile: "`ISSUED` deliverables (currently DEL-01-01): read-only in discovery; any change routes through the governed scope-change process" (`{REPO}/projects/chirality-piping/docs/RECONCILIATION_PROFILE.md:84-85`).
- **What is inconsistent today.** The same deliverable is recorded two ways: BaselineClass ISSUED and AuthorityNeeded OWNER on SOW pin rows, but NONE/NO on its four `_CONTEXT.md` pin rows and on the MAINTAINERS.md row (T4A observation 1). `DEL-01-01:SOW` is routed to R4 twice, through FG-DEL-01-01-02 and through the rename class (T4B observation 4; T9-C06).

## 3. Options

As they stand in the evidence (T9-C05, C06, C10; T5B-C07 items 1–2; T4B-C02 item 1; T4A-C08):

1. **Reissue.** Revise the ISSUED SOW with the DEC-027 / DEC-057 / DEC-089 / DEC-081 / DEC-105 values, current pins (revision 0.12, DAG-010), the rename, the representation residue and the status field, then reissue through the lifecycle and scope-change workflows.
   - Deliverables: one controlled revision of DEL-01-01 covering all 30 portion rows; a new issuance record and parity.
   - Code: none. MAINTAINERS.md (T4A-C07, H2) can ride the same revision if the owner treats it as ISSUED content.
   - Other packets: A8's posture ruling (CLM-009.s01) must come first or ride along, or the reissued text will carry a known conflict.
2. **Amend on the ISSUED change path** without full reissue: a scope-change amendment that restates the affected rows, keeping ISSUED with the amendment recorded.
   - Same row coverage as option 1; lighter lifecycle handling.
3. **Keep as issuance history.** Leave the ISSUED text unchanged, add a pointer to the later rulings, and record the drift. No ruling currently supports accepted divergence for this drift (C6(g)), so this needs an owner record under A3a. T4A-C08 notes that no ledger proposes this for the pins; it is the do-nothing alternative.

**The DEC-081 Wave 2 edit (T9-C10; T5B-C07 item 2), a confirmation:**
- **(a) Confirm** that DEC-081 validly reached the ISSUED SOW; keep ISSUED with the edit recorded; AC-001 stays ACCEPTED_DIVERGENCE.
- **(b) Route** the six-line change through the ISSUED change path (it joins option 1 or 2).

**Routing of `DEL-01-01:SOW` rename residue (T9-C06; T4B-C01 item 4):**
- **(a)** CP-04 as written with BaselineClass ISSUED, repaired inside A4's rename ruling through the ISSUED change path;
- **(b)** fold it into the C6(d) ISSUED group (and extend C6(d) to the five ISSUED pin rows). Either way this packet is its one route.

**Sub-question on grouping (T5B-C07 item 1; FIELD resolutions).** `CLM-004.r05` rests on DEC-081 and DEC-105, not on DEC-027 / DEC-057 / DEC-089; it can be a separate group or FG-DEL-01-01-01 with the extension disclosed. `CLM-014` adds DEC-081 to its basis.

## 4. Evidence and reliability

| Source | Shows | Reliability |
|---|---|---|
| `DEL-01-01/_STATUS.md:3`, `:14`, `:15` | ISSUED; approval terms; post-issuance edit | Lifecycle record |
| DEC-027/057/081/089/101/105 rows in `SOFTWARE_DECOMP.md` | The overtaking rulings | Governing sources |
| Profile `:84-85`; C6(d)/(e)/(g) | ISSUED change routes through scope change; one R4 item | Governing run instruments (RULED) |
| T9-C05, C06, C08, C10 (`RUN/R3/TASKS/T9_LIFECYCLE.*`) | The three linked ISSUED items and the proposed single packet | R3 task proposal |
| T5B-C07 (i), T4B-C02, T4A-C08, T4B-C01 item 4 | Row groups, options | R3 task proposals over verifier-checked ledgers |
| W2 owner list (`RUN/WAVES/W2/W2_GATE_ASSESSMENT.md`, "Owner rulings needed") | DEC-081 edit confirmation; double routing of rename residue | Agent 0 assessment |
| RECON-I0-PKG01 parity at conversion (T9-C10) | Parity PASS at the 2026-07-14 conversion; OUT-001 binds the pre-DEC-081 bytes (CP-09) | Run record (evidence); no human disposition of the post-DEC-081 bytes located |

## 5. Affected claims

**This packet's portion: 30 rows**, all DEL-01-01 (PKG-01), all Authority OWNER by class.

| Class | Class rows | Portion | Filter |
|---|---|---|---|
| T4A-C08 Pins on the ISSUED DEL-01-01 | 9 | 9 (whole) | `ClassID == 'T4A-C08'` |
| T4B-C02 ISSUED and baseline-bearing record drift | 5 | 4 | `ClassID == 'T4B-C02' and DeliverableID == 'DEL-01-01'` |
| T5B-C07 PROJECT_BASELINE claims overtaken (split class) | 22 | 16 | `ClassID == 'T5B-C07' and DeliverableID == 'DEL-01-01'` |
| T4B-C01 Rename and identity residue | 86 | 1 | `ClaimKey == 'DEL-01-01:SOW'` |

Portion keys:
- T4A-C08: `DEL-01-01:SOW#CLM-002.r09`, `#CLM-002.r10`, `#CLM-007`, `#CLM-013.s01`, `#CLM-017` (BaselineClass ISSUED); `DEL-01-01:CONTEXT`, `CONTEXT#package-reference`, `CONTEXT#decomposition-reference`, `CONTEXT#architecture-basis-injection` (BaselineClass NONE).
- T4B-C02: `DEL-01-01:SOW#CLM-002.r11` (status field IN_PROGRESS; also T9-C08), `#CLM-018.s01`, `#CLM-019.r06`, `#CLM-020` (four-document residue, declared CANONICAL_DEPARTURE to ISSUED).
- T5B-C07: FG-DEL-01-01-01, 15 rows (also T9-C05): `DEL-01-01:SOW#CLM-004.r04`, `.r05`, `.r06`, `#CLM-005.r02`, `.r03`, `.r04`, `.r05`, `#CLM-006`, `#CLM-013.s02`, `#CLM-014`, `#CLM-018.s02`, `#CLM-023`, `#CLM-024.s04`, `#CLM-025.r02`, `#CLM-027/C-01-01-002`; and `DEL-01-01:SOW#completion-and-reliance-basis-epistemology/AC-001` (ACCEPTED_DIVERGENCE, DEC-081 Wave 2).
- T4B-C01: `DEL-01-01:SOW` (also T9-C06; CONTESTED).

T5B-C07's other 6 rows are A10's portion. T4B-C02's fifth row (DEL-17-03 AC-001) is A2's. T4B-C01's other 85 rows are A4's. T4B-C02 and T4B-C01 are not listed as split classes in the topic file; reported to Agent 0.

**Subject-keyed item (T9).** `LIFECYCLE_STATE:ISSUED` (T9-C10). T9-C05, C06 and C08 (DEL-01-01 part) are row-keyed and coincide with portion keys above.

**Other DEL-01-01 rows on other routes (not claimed; any edit to them still touches the ISSUED deliverable).**

| Key | Class (route) | Owner of the decision |
|---|---|---|
| `DEL-01-01:SOW#CLM-009.s01` | T7-C04 (OWNER_DECISION) | A8 (posture) |
| `DEL-01-01:CONTEXT#architecture-basis-injection.s01` | T5B-C04 (OWNER_DECISION) | A5 (SEMANTIC_READY) |
| `DEL-01-01:SOW#CLM-024.s01` (MAINTAINERS.md) | T4A-C07 (CODE_FIX_CANDIDATE) | H2; ISSUED path if treated as ISSUED content |
| `DEL-01-01:CONTEXT#architecture-basis-injection.s02` | T5B-C05 (R5) | H4 |
| `DEL-01-01:MEMORY`, `DEL-01-01:STATUS` | T4B-C06 (R5) | H4 |
| `DEL-01-01:SOW#CLM-011/AC-01-01-04` | T4B-C08 (R5) | H4 |
| `DEL-01-01:SOW#CLM-011/AC-01-01-05`, `#CLM-019` | T6-C01 (CODE_FIX_CANDIDATE) | H2 |
| `DEL-01-01:SOW#output-and-evaluation-matrix/OUT-001`, `#production-and-verification-method-praxeology/VER-001` | T5B-C01 (REVIEW) | H3 |

H2, H3 and H4 should mark these rows `BlockedOnPacket = A6` for their DEL-01-01 text, because a non-owner tranche could otherwise edit the ISSUED deliverable by default (T4A-C08 risk).

**Rows known only from `OtherCorrections`:** the FIELD corrections on `CLM-004.r05` and `CLM-014` (basis and grouping); the CONTESTED `DEL-01-01:SOW` routing; and the non-divergent `DEL-01-01:SOW#CLM-003` (CONTESTED, tied to the posture conflict; A8).

## 6. Risks

- **Undecided.** The ISSUED governance baseline keeps asserting TBD roles that rulings fixed, so the ISSUED record and the decision register disagree at baseline tier; current SOW bytes have no issuance or parity record; one row can be ruled twice, inconsistently; a C01-style catch-up could edit DEL-01-01's `_CONTEXT.md` by default.
- **Option 1.** Heaviest process; reissuing before A8 would carry the posture conflict into new issued text.
- **Option 2.** Leaves the ISSUED record as a stack of amendments; still needs a parity record for the new bytes.
- **Option 3.** Keeps known-false TBD text in the issued baseline; needs an owner record under A3a, and MAINTAINERS L42 carries the same lag (T9-C05).
- **DEC-081 (a).** Sets a precedent that a governed-loop edit can reach ISSUED text without scope change, against the profile wording. **(b)** Adds the edit to the change bundle.

## 7. Recommended routing

The evidence supports one procedural recommendation: rule all DEL-01-01 edits as **one** ISSUED-path decision (T9-C10 proposal; T4A observation 1; T4B observation 4), so the 30 portion rows and the other DEL-01-01 rows above are not edited by two paths. Between reissue, amend and keep-as-history, and on the DEC-081 confirmation: no recommendation; owner's call.

## 8. On-ruling mechanism

- **Option 1 or 2.** An R4 owner ruling authorises an ISSUED change for DEL-01-01 through the scope-change workflow (WORKING_ITEMS, workflow: scope-change), per the profile and C6(d) ruling item 4. Under option 1 the lifecycle workflow handles reissue. The text repair is then carried out under that change's authority (R5 or R6), together with the MAINTAINERS.md brief (T4A-C07) if the owner includes it, and with A8's posture ruling applied to CLM-009.s01.
- **Option 3.** An owner record (A3a) naming the drift and the pointer to the rulings; an R5 edit of the non-ISSUED surfaces only (`_STATUS.md` history pointer), leaving the SOW unchanged.
- **DEC-081 (a).** Owner confirmation recorded; AC-001 row closed as ACCEPTED_DIVERGENCE under that confirmation. **(b)** The edit joins the option 1/2 change bundle.
- **`DEL-01-01:SOW` rename.** Carried by whichever option is chosen; A4's rename wording applies to the text.

Nothing executes until the owner acts. D-73 permits no write in this run; R5 needs separate authorisation.

## 9. Dependencies

- **Depends on:** A8 (posture) should be ruled before or with a reissue; A4 supplies the rename wording.
- **Interacts with:** A5 (the DEL-01-01 injection sub-claim), H2 (MAINTAINERS.md), H3 (CP-09 parity for OUT-001/VER-001 after any change).
- **Blocks:** every H2/H3/H4 edit of DEL-01-01 text listed above; T4A-C07's MAINTAINERS.md brief if treated as ISSUED content.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No certification, code-compliance, professional-approval or engineering-acceptance claim is made.
