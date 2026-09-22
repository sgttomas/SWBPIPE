# A8 — Product posture, and governing sources in conflict or silent

PROPOSAL for the R4 gate (R3 integration, TASK P1). It decides nothing and changes no row.

Conventions: `F:` is `{FREEZE}/projects/chirality-piping/` at `00115c719`. `RUN/` is the run folder.

## 1. Decision

Four questions where the governing sources disagree or say nothing, and no code has settled the matter (T7-C04):
1. **Product posture.** Is the product "free and open-source" (PRD) or "source-available noncommercial" (DIRECTIVE, ScopeLedger, licence)?
2. **Pre-release legal review.** Is the review PRD v0.1 §17.5 required still required, and on what basis?
3. **Palette code landing.** Which governing source decides where the palette code lands?
4. **Secret provider and encrypted storage** (DEL-12-04 CF-001, CF-002): rule or confirm they stay TBD.

**Holder: OWNER** for questions 1 and 2. Questions 3 and 4 are the same choices that **B5** (palette landing, VIEW-025/026) and **B12** (secret provider selection) present; to keep one decision per topic, this packet does not restate their options and records only how the T7-C04 rows follow those rulings. If the owner treats question 3 as reassigning ownership, it goes to **WORKING_ITEMS (workflow: scope-change)** (T7-C04).

## 2. Background

- **Posture.** The PRD calls SWBPIPE "a proposed free and open-source, local-first, analysis-grade piping design engine" (`F:docs/PRD.md:24`); the project `F:README.md:10` uses the same phrase. The DIRECTIVE says "Source-available noncommercial" (`F:docs/DIRECTIVE.md:73`) and records the PolyForm Noncommercial selection (`:107`); `F:LICENSE.md` carries `SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0` (l.3). ScopeLedger SOW-001 also says source-available noncommercial (T7-C04). DEL-01-01 was issued on 2026-06-03 with the licence selection approved (`F:execution/PKG-01_…/1_Working/DEL-01-01_…/_STATUS.md:14`). The PRD elsewhere keeps open-source framing (for example `F:docs/PRD.md:38`, `:354`).
- **Legal review.** PRD v0.1 §17.5 required a pre-release legal review; PRD v0.4 did not carry it forward (T7-C04). DEL-01-01 `CLM-018.s02` still names the legal review process as open under DEC-079 (T9-C05).
- **Palette.** DEC-094 makes DEL-07-09 the owner of the palette surface (`F:execution/_Decomposition/SOFTWARE_DECOMP.md:685`); no annex row says where `ToolkitPalette.tsx` and `capabilityCatalog.ts` land, and DEL-07-09's `_CONTEXT` says it never receives implementation (T7-C04; T1 decision 2 notes the same envelope text for the editors).
- **Secret provider.** DEL-12-04 CF-001 and CF-002 defer the secret provider and encrypted storage; one element of CF-002, the package/container, is stale because SCA-003 settled it (T7-C04).

## 3. Options

**Question 1 (posture)**, as in T7-C04:
- **1a. Source-available noncommercial.** Align the PRD (and README) with the DIRECTIVE, ScopeLedger and licence. Consequence: a PRD revision; the ISSUED DEL-01-01 CLM-009.s01 then matches its governance surfaces and needs no posture change; the DEL-01-01 `CLM-003` CONTESTED reading becomes ALIGNED "if the owner treats the PRD sentence as the stale side".
- **1b. Free and open-source.** Change the licence and the governance surfaces to match the PRD. Consequence: a licence change (outside this run and outside agent authority), DIRECTIVE and ScopeLedger revisions, and an ISSUED-text change on DEL-01-01 through A6.

**Question 2 (legal review):**
- **2a. Still required.** State its basis (a DEC or PRD revision) and owner; `DEL-01-03:STATUS#remaining/R02` then cites it.
- **2b. Retired.** Record the retirement; R02 is restated as closed.

**Question 3 (palette landing):** the options (DEL-07-09, DEL-07-01 or DEL-07-02) are presented in **B5**. Whatever B5 rules, `DEL-07-09:PALETTE_ORGANIZATION_CONTRACT#organization-and-ownership` is repaired to cite that ruling; if the ruling amends DEL-07-09's envelope, that goes to scope change.

**Question 4 (secret provider):** the selection is presented in **B12** (T6-C04 D-items). Whatever B12 rules (select, or confirm TBD), the two DEL-12-04 rows cite it, and the stale package/container element of CF-002 is repaired against SCA-003 regardless.

## 4. Evidence and reliability

| Source | Shows | Reliability |
|---|---|---|
| `F:docs/PRD.md:24`; `F:README.md:10` | "free and open-source" | Governing source (PRD) and a project document; read by this task |
| `F:docs/DIRECTIVE.md:73`, `:107`; `F:LICENSE.md:3` | Source-available noncommercial; PolyForm Noncommercial | Governing sources; read by this task |
| `F:execution/_Decomposition/SOFTWARE_DECOMP.md:685` (DEC-094) | Palette ownership | Governing source |
| T7-C04 (`RUN/R3/TASKS/T7_CLASSES.md`) | Five rows, four decisions | R3 task proposal over verifier-checked ledgers |
| W2 owner list (`RUN/WAVES/W2/W2_GATE_ASSESSMENT.md`, "Owner rulings needed") | Posture conflict | Agent 0 assessment |
| PRD v0.1 §17.5 legal-review clause | Earlier requirement | From ledger Notes only (not re-read here) |

## 5. Affected claims

**This packet's portion: 5 rows**, the whole of T7-C04 (Authority OWNER). Filter: `ClassID == 'T7-C04'`.

| Key | Question | Signature |
|---|---|---|
| `DEL-01-01:SOW#CLM-009.s01` | 1 | AUTHORITY_CONFLICT · AUTHORITY_UNCLEAR · PROJECT_BASELINE · ISSUED |
| `DEL-01-03:STATUS#remaining/R02` | 2 | UNKNOWN · AUTHORITY_UNCLEAR · PROJECT_BASELINE |
| `DEL-07-09:PALETTE_ORGANIZATION_CONTRACT#organization-and-ownership` | 3 (follows B5) | UNKNOWN · AUTHORITY_UNCLEAR · PROJECT_BASELINE |
| `DEL-12-04:SOW#CLM-028/DEL-12-04-CF-001` | 4 (follows B12) | UNKNOWN · AUTHORITY_UNCLEAR · LOCAL_DESIGN |
| `DEL-12-04:SOW#CLM-028/DEL-12-04-CF-002` | 4 (follows B12) | UNKNOWN · AUTHORITY_UNCLEAR · LOCAL_DESIGN |

Packages PKG-01, 07, 12; deliverables DEL-01-01, 01-03, 07-09, 12-04.

**Rows known only from `OtherCorrections`** (not divergent): `DEL-01-01:SOW#CLM-003` (CONTESTED: candidate AUTHORITY_CONFLICT sharing the CLM-009.s01 finding, or ALIGNED if the PRD sentence is the stale side).

**Cross-reference.** Any edit to `DEL-01-01:SOW#CLM-009.s01` or `#CLM-003` is ISSUED text and goes through A6's change path.

## 6. Risks

- **Undecided.** The public posture and the licence contradict each other in issued text, the PRD and the project README; contributors and users may rely on the wrong one. A release-time legal gate stays ambiguous. Palette code has no owning deliverable. Secret-handling defaults stay undefined.
- **1a.** A PRD revision; low product risk.
- **1b.** A licence change with legal consequences outside this run; ISSUED-text change.
- **2a.** Adds a release gate that must be owned and scheduled.
- **2b.** Removes a legal check the earlier PRD required; the owner carries that choice.

## 7. Recommended routing

No recommendation on substance; owner's call. One factual point: the licence file, the DIRECTIVE, ScopeLedger SOW-001 and the ISSUED DEL-01-01 approval record all agree with each other, and only the PRD and README differ. That locates the conflict; it does not decide which side the owner wants.

## 8. On-ruling mechanism

- **Question 1.** 1a: a PRD revision through the PRD change path (scope-change bundle if the owner treats it as PRD scope) and a README edit; R5 record repair of `CLM-009.s01` if needed via A6. 1b: a licence change by the owner, DIRECTIVE and ScopeLedger revisions, and an ISSUED change on DEL-01-01 through A6.
- **Question 2.** An owner record (DEC or PRD revision), then an R5 record repair of `DEL-01-03:STATUS#remaining/R02`.
- **Questions 3–4.** The B5 and B12 rulings, then R5 record repair of the three rows (CF-002's SCA-003 element in any case); an ownership reassignment goes through scope change.

Nothing executes until the owner acts. R5 needs separate authorisation.

## 9. Dependencies

- **Depends on:** B5 (question 3) and B12 (question 4).
- **Interacts with:** A6 (ISSUED DEL-01-01 text; posture should be ruled before any reissue).
- **Blocks:** H4 repair of the 5 rows; A6's reissue option (posture).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No certification, code-compliance, professional-approval, engineering-acceptance or legal-advice claim is made.
