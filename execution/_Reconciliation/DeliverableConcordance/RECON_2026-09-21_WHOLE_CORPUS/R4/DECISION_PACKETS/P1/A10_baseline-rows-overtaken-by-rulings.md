# A10 — Baseline rows overtaken by rulings (non-DEL-01-01), and the release-label floor

PROPOSAL for the R4 gate (R3 integration, TASK P1). It decides nothing and changes no row.

Conventions: `F:` is `{FREEZE}/projects/chirality-piping/` at `00115c719`. `RUN/` is the run folder. `REG:` is `{REPO}/projects/chirality-piping/execution/_Coordination/_DECISIONS/_REGISTER.md`.

## 1. Decision

Five baseline-tier questions where a later ruling (or a lost source) overtook a deliverable's hold or principle, and a catch-up is not a no-decision repair under A2:
1. **D-68 against the DEL-00-05 accessibility hold.** How far does D-68 close it?
2. **SCA-004 against the DEL-00-07 export-format list.** Is SCA-004 the resolving ruling?
3. **DEC-058 for DEL-08-05 OQ-003 / OQ-005.** Confirm DEC-058 closes the release-scan parts.
4. **DEC-051 against the DEL-12-05 explicit-disclosure principle.** Amend the principle or reopen DEC-051?
5. **Release-label floor (PB-TBD-003).** Settle the replacement for the engineering-beta minimum that VALIDATION_STRATEGY §4 lost, then align DEL-09-05 RQG-007.

**Holder: OWNER** for all five. Item 3 is likely a confirmation, not a decision (T5B observation 5).

## 2. Background

- **Item 1.** DEL-00-05's ArchitectureBasis keeps "accessibility target details" OPEN as never ruled (`F:execution/PKG-00_…/1_Working/DEL-00-05_…/ArchitectureBasis.md:50`). D-68 (RULED 2026-09-15) adopted "bounded accessibility criteria" with the PKG07 workspace foundation (REG: row D-68, l.105; `F:execution/_Coordination/_DECISIONS/D-68_RULING_2026-09-15.md`).
- **Item 2.** DEL-00-07's ArchitectureBasis keeps "External format list and the per-format export contracts" TBD (`F:execution/PKG-00_…/1_Working/DEL-00-07_…/ArchitectureBasis.md:80`), while SCA-004 added PKG-17 export packages and formats (`F:execution/_Decomposition/SOFTWARE_DECOMP.md:33`). AB-00-07 at revision 0.12 still says TBD (W2 owner list).
- **Item 3.** DEL-08-05 OQ-08-05-003 says severity policy is "TBD; must be confirmed before CI guard implementation" (`F:execution/PKG-08_…/1_Working/DEL-08-05_…/ScopeOfWork.md:472`). DEC-058 (D-20 ruling) set release-artifact scan ownership and procedure (`SOFTWARE_DECOMP.md:649`). The CI-guard part stays open.
- **Item 4.** DEL-12-05 states an explicit-disclosure principle: private data leaving local control is a user-intent event (`F:execution/PKG-12_…/1_Working/DEL-12-05_…/ScopeOfWork.md:148`, `:452`). DEC-051 (D-24; `SOFTWARE_DECOMP.md:642`) adopted OPEN RESIDENCY: owner-configured agent visibility and networked model providers with no app-side egress guard. No provider channel is live at the freeze (T5B-C07).
- **Item 5.** DEL-09-05 RQG-007 ties the engineering-beta label to the release gate in `docs/VALIDATION_STRATEGY.md#4` (`F:execution/PKG-09_…/1_Working/DEL-09-05_…/ScopeOfWork.md:152`); section 4 ("Release gate", `F:docs/VALIDATION_STRATEGY.md:105`) lost its engineering-beta minimum condition on 2026-06-07 (c8748a04a, per T4A-C06; not re-derived here). PB-TBD-003 holds "Release-label vocabulary and final release policy language" for the human project authority (`F:docs/PROFESSIONAL_BOUNDARY.md:174`); release tooling defers labels to it (`F:tools/release/run_release_gate_records.py:25-30`).

## 3. Options

As they stand in the evidence (T5B-C07 items 3–6; T4A-C06 (iii)):

| Item | Options | Consequence |
|---|---|---|
| 1 D-68 / DEL-00-05 | (a) fully closes; (b) closes in part, independent usability still held; (c) does not close, target left to DEL-07-06 | (a)/(b): R5 repair of the AB hold row; (c): hold stays, cite D-68 as partial context. C3 carries the independent usability basis |
| 2 SCA-004 / DEL-00-07 | (a) yes, SCA-004 resolves the format list: repair the hold; (b) no: the TBD stands and the rows read AUTHORITY_CONFLICT (OBSERVED note) | (a): R5 repair of AB-00-07; (b): AB-00-07 and SCA-004 need reconciling through the architecture path |
| 3 DEC-058 / DEL-08-05 | (a) confirm DEC-058 closes the release-scan parts, CI-guard severity still open; (b) do not confirm | (a): R5 repair of OQ-003/OQ-005 text, or H4 treatment as a no-decision catch-up; (b): rows stay open |
| 4 DEC-051 / DEL-12-05 | (a) amend the principle to carve out the owner-configured provider channel; (b) reopen DEC-051 | (a): R5 repair of two SOW rows; (b): a DEC-051 change through its own path, security-layer consequences |
| 5 PB-TBD-003 floor | (a) rule the replacement floor under PB-TBD-003, then align RQG-007 and checklist §8; (b) keep PB-TBD-003 open and mark RQG-007's cited condition as absent | (a): a DEC then R5 repair; (b): RQG-007 cannot be met or checked until ruled |

**Contested points kept visible.**
- Item 4: `DEL-12-05:SOW#CLM-033.r02` is CONTESTED on tier: INVARIANT with layer SECURITY under F8, or PROJECT_BASELINE as sealed.
- Item 2: `DEL-00-07:AB#open-holds-and-routed-questions.s02` is OBSERVED (AB-00-07 rev 0.12 still TBD argues for AUTHORITY_CONFLICT).
- Item 5: the CONTESTED resolutions on both DEL-09-05 rows propose cause AUTHORITY_UNCLEAR, with tier INVARIANT or PROJECT_BASELINE (two options).

## 4. Evidence and reliability

| Source | Shows | Reliability |
|---|---|---|
| AB/SOW lines above | The holds and principle as written | Deliverable records at the freeze, read by this task |
| D-68 (REG l.105), SCA-004 (`SOFTWARE_DECOMP.md:33`), DEC-058 (`:649`), DEC-051 (`:642`), PB-TBD-003 (`PROFESSIONAL_BOUNDARY.md:174`) | The overtaking rulings and the open hold | Governing sources |
| T5B-C07 (ii), T4A-C06 (iii) (`RUN/R3/TASKS/`) | 6 + 2 rows; options | R3 task proposals over verifier-checked ledgers |
| W2 list (export formats), W3 list (DEC-051 provider disclosure) | Owner items | Agent 0 assessments |
| VALIDATION_STRATEGY §4 change (c8748a04a) | Loss of the engineering-beta minimum | Ledger record; not re-derived (no git) |

## 5. Affected claims

**This packet's portion: 8 rows.**

| Class | Class rows | Portion | Filter |
|---|---|---|---|
| T5B-C07 (Authority OWNER; split class) | 22 | 6 | `ClassID == 'T5B-C07' and DeliverableID != 'DEL-01-01'` |
| T4A-C06 (Authority OWNER; split class) | 14 | 2 | `ClassID == 'T4A-C06' and DeliverableID == 'DEL-09-05'` |

Portion keys:
- Item 1: `DEL-00-05:AB#open-holds-and-routed-questions.s03`.
- Item 2: `DEL-00-07:AB#open-holds-and-routed-questions.s02` (OBSERVED).
- Item 3: `DEL-08-05:SOW#CLM-033/OQ-08-05-003`, `DEL-08-05:SOW#CLM-033/OQ-08-05-005` (PROJECT_BASELINE, RULED_CRITERION, AuthorityNeeded NO).
- Item 4: `DEL-12-05:SOW#CLM-011.r02`; `DEL-12-05:SOW#CLM-033.r02` (CONTESTED).
- Item 5: `DEL-09-05:SOW#CLM-012/RQG-007`, `DEL-09-05:SOW#CLM-029` (both CONTESTED).

T5B-C07's other 16 rows are A6's; T4A-C06's other rows are A1's (7) and A7's (5).

**Rows on other routes (not claimed).**
- `DEL-10-05:SOW#CLM-004.r04` (T5B-C06, R5): FIRM correction cites SCA-004 and OI-004 (`SOFTWARE_DECOMP.md` L572); consistent with item 2 option (a).
- `DEL-08-04:SOW#CLM-013/V-7` (T6-C03, CODE_FIX_CANDIDATE, Authority REVIEW): FIELD correction adds the DEC-058 scan tool `tools/release/run_release_candidate_scan.py` as evidence; context for item 3.

**Rows known only from `OtherCorrections`** (not divergent): `DEL-00-07:AB#normative-requirements/REQ-07-04` (CONTESTED; "Right values: unresolved": PARTIALLY_IMPLEMENTED · SCOPE_REDIRECTED_BY_RULING · PROJECT_BASELINE · OWNER if SCA-004 governs the export side; ALIGNED and open if AB-00-07's TBD governs). It follows item 2.

**Packages and deliverables.** PKG-00 (DEL-00-05, 00-07), PKG-08 (DEL-08-05), PKG-09 (DEL-09-05), PKG-12 (DEL-12-05).

## 6. Risks

- **Undecided.** Baseline records and the decision register disagree on five subjects. RQG-007 cites a condition that no longer exists, so no release candidate can be checked against it. The DEL-12-05 principle and DEC-051 stay in tension on a security-layer subject (no provider channel is live yet, so the exposure is latent).
- **Item 1 (a).** May over-read D-68, which bounded criteria for one implementation plan; independent usability may still be open (C3).
- **Item 2 (a).** SCA-004 names formats for PKG-17; whether it answers AB-00-07's per-format contracts is a reading.
- **Item 4 (b).** Reopening DEC-051 reverses a tier-0 residency ruling.
- **Item 5 (b).** Leaves the engineering-beta label ungated.

## 7. Recommended routing

- Item 3: the evidence supports a confirmation (DEC-058's scan ruling is explicit and T5B observation 5 finds it unambiguous); owner confirms or declines.
- Items 1, 2, 4, 5: no recommendation; owner's call.

## 8. On-ruling mechanism

- **Items 1–3.** An owner ruling or confirmation recorded in the register, then an R5 record repair of the named ArchitectureBasis or SOW rows citing it. For item 2 option (b), an architecture-basis amendment reconciling AB-00-07 with SCA-004 through the architecture-decision path.
- **Item 4.** Option (a): an R5 record repair of the two DEL-12-05 rows carving out the owner-configured channel. Option (b): a DEC-051 change through its own decision path (and the tier-0 ruling it adopts), then repair.
- **Item 5.** A DEC under PB-TBD-003 (owner's decision record), then an R5 repair of RQG-007, `CLM-029`, and checklist §8; or, under (b), an R5 note that the cited condition is absent pending PB-TBD-003.

Nothing executes until the owner acts. R5 needs separate authorisation.

## 9. Dependencies

- **Depends on:** none.
- **Interacts with:** C3 (independent usability basis, item 1); A7 (export formats and the deleted plan are separate questions on PKG-17); B12 (release/QA/stage gates, item 5 is the label floor only).
- **Blocks:** H4 repair of the 8 portion rows; the `DEL-00-07` REQ-07-04 reading.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No certification, code-compliance, professional-approval or engineering-acceptance claim is made.
