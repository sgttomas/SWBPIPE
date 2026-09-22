# A9 — The D-41 "current declaration" blocks: re-pin or retire

PROPOSAL for the R4 gate (R3 integration, TASK P1). It decides nothing and changes no row.

Conventions: `F:` is `{FREEZE}/projects/chirality-piping/` at `00115c719`. `RUN/` is the run folder.

## 1. Decision

The D-41-era "current declaration" blocks (for example "D-41 R5 T7 PDU-055 current declaration") name SOFTWARE_DECOMP revision 0.8 and DAG-007 as current authority and delegate residuals to `_STATUS.md` Remaining. One treatment choice for all 213 rows: **re-pin** each block to revision 0.12 and DAG-010, or **retire** it at the next SOW catch-up (marking it superseded on history surfaces).

**Holder: OWNER**, as a single treatment choice inside the R5 authorisation (T4A-C02). After the choice the work is mechanical.

## 2. Background

- **Frozen basis.** SOFTWARE_DECOMP is at revision 0.12 (`F:execution/_Decomposition/SOFTWARE_DECOMP.md:6`); the latest DAG is DAG-010 (`F:execution/_DAG/_LATEST.md:3`).
- **The blocks.** Headings such as `F:execution/PKG-04_…/DEL-04-05_…/ScopeOfWork.md:26-28` ("D-41 R5 T7 PDU-055 current declaration"). They come from the D-41 concordance program (register row D-41; DEC-074 at `SOFTWARE_DECOMP.md:665`). 208 sit in `ScopeOfWork.md`; five sit on history surfaces: `DEL-04-06:MEMORY`, `DEL-11-04:MEMORY`, `DEL-09-05:MEMORY.s01`, `DEL-11-01:MEMORY.s01`, `DEL-12-01:STATUS#d-41-r5-t7-pdu-055-current-declaration` (T4A-C02).
- **How the ledgers read them.** CP-03 (169 rows) or CP-02 on the same blocks (44 rows, for example DEL-17-07/08/09 CLM-002 and CLM-010, "CP-03 block; treatment per CP-02"). They record the delegation clause as not relied on (A4), and they note the no-closure sentence is still accurate.
- **Selection mechanism changed.** Since 2026-09-19 Piping selects work through owner-steered work graphs, not `## Remaining` (`RUN/CONVENTIONS.md` C9). Remaining is not authority (A4, Direction 1).

## 3. Options

As in the ledgers' RemainingWork (T4A-C02):

1. **Re-pin (a).** Re-pin each declaration to revision 0.12 and DAG-010 (for example `DEL-04-05:SOW#CLM-010`, `DEL-14-05:SOW#CLM-009`).
   - Deliverables: 213 edits across 69 deliverables; the blocks stay as current declarations.
   - Consequence: keeps the Remaining delegation live, although Piping no longer selects work from Remaining (C9), so the block would direct readers to a mechanism that is not used.
2. **Retire (b).** Retire the declaration at the next SOW catch-up (for example `DEL-08-03:SOW#CLM-002`: "or retire it at the next SOW catch-up"). On the five MEMORY and STATUS units, retire means marking the undated declaration as superseded, not deleting it.
   - Deliverables: 208 SOW block removals plus five history annotations.
   - Consequence: removes text that the ledgers already treat as not relied on; the no-closure sentence (accurate) goes with it unless kept separately.
3. **Hybrid.** Retire the delegation clause and re-pin the rest (keep a dated "as of revision 0.12 / DAG-010" statement with the accurate no-closure sentence). This is not proposed by any ledger; it is listed because options 1 and 2 each drop one accurate element the ledgers note.

No code consequence under any option. No other packet's decision changes with this choice.

## 4. Evidence and reliability

| Source | Shows | Reliability |
|---|---|---|
| `SOFTWARE_DECOMP.md:6`; `_DAG/_LATEST.md:3` | Current basis revision 0.12, DAG-010 | Governing sources |
| C9, A4 (`RUN/CONVENTIONS.md`) | Remaining no longer selects work; is not authority | RULED run conventions |
| T4A-C02 (`RUN/R3/TASKS/T4A_CLASSES.md`) | 213 rows, both options with example keys | R3 task proposal over verifier-checked ledgers |
| Ledger RemainingWork on the example keys | Options (a) and (b) as written by workers | Worker text, verifier-checked |

## 5. Affected claims

**This packet's portion: 213 rows**, the whole of T4A-C02 (Authority "OWNER (treatment choice)"). Filter: `CLASS_ASSIGNMENTS.csv` `ClassID == 'T4A-C02'`.

- Signature: BASIS_POINTER_STALE · LOCAL_DESIGN · STALE_REVIEW_OR_EVIDENCE · NONE · RECORD; CP-03 169, CP-02 44.
- Surfaces: SOW 208 (BLOCK), MEMORY 2, MEMORY.s01 2, STATUS 1.
- Packages (rows): PKG-17 23, 08 22, 07 21, 09 20, 14 20, 11 19, 10 17, 06 16, 04 11, 02 10, 15 9, 13 7, 16 6, 12 5, 03 3, 05 3, 01 1 (17 packages, not PKG-00); 69 deliverables. Waves W1 27, W2 25, W3 161.
- Exceptions: six FIELD rows (VerificationClass only): `DEL-10-04:SOW#CLM-002`, `#CLM-010`, `#CLM-025`; `DEL-10-05:SOW#CLM-008`, `#CLM-016`, `#CLM-023`. No route change.

No `OtherCorrections` text changes the treatment question. No T8 cluster covers these rows.

**Adjacent rows (not claimed).** The same SOW files carry other revision pins (T4A-C03, R5) and, on 42+40 deliverables, the SEMANTIC_READY injection (A5). A single catch-up PR per package would touch all of them; the treatment chosen here should be applied in the same pass.

## 6. Risks

- **Undecided.** Readers take revision 0.8 and DAG-007 as governing authority, and the Remaining delegation points to a selection mechanism no longer used (T4A-C02). Any agent brief sealed from these SOWs inherits the stale authority.
- **Option 1.** Re-affirms a delegation to Remaining that conflicts with C9 and A4.
- **Option 2.** Loses the accurate no-closure sentence unless kept; the history surfaces must be annotated, not deleted.
- **Option 3.** A new text form that no ledger has checked; more editorial judgement per block.

## 7. Recommended routing

No recommendation; owner's call. The evidence notes only that option 1 keeps live a delegation that conflicts with the ruled conventions C9 and A4, and that under option 2 the five MEMORY/STATUS units should be annotated rather than edited in place (T4A-C02).

## 8. On-ruling mechanism

The R5 ruling names option 1, 2 or 3 for all 213 rows at once. A catch-up tranche then edits the `ScopeOfWork.md` files (plus the five MEMORY and STATUS units) through the chirality-change PR path, one PR per package or per tranche. No sibling disposition changes (C1). This is an R5 record repair; it needs the owner's separate R5 authorisation (D-73). Nothing executes until the owner acts.

## 9. Dependencies

- **Depends on:** none.
- **Interacts with:** A5 and T4A-C01/C03 (same files; one pass per package is efficient); A6 (DEL-01-01 is not in this class; PKG-01's one row is outside DEL-01-01, so no ISSUED edit arises).
- **Blocks:** H4 repair of the 213 rows (tranche marked `BlockedOnPacket = A9`).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No certification, code-compliance, professional-approval or engineering-acceptance claim is made.
