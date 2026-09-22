# B6 — Convention for desktop panels that name a deliverable whose SOW excludes GUI

Packet for the R4 gate of run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
Writer: TASK P2. This is a proposal only. The path conventions are as in B1.

## 1. Decision

Several mounted desktop panels identify one deliverable, several by an
embedded `deliverable_id`, but that deliverable's SOW excludes GUI or does
not key the panel. The main cases are the DEL-17-04 MBF export panel and the
DEL-17-05 external-run panel. One convention is needed: are such panels
authorised under the named deliverable, treated as shared desktop
infrastructure consuming its contract, reassigned, or withdrawn? The packet
also rules on the concrete DEL-17-04 case (T5A-C05 decision 2).

**Holder: OWNER.** WORKING_ITEMS (workflow: scope-change) follows for a
reassignment or a lifted exclusion. The convention could be recorded as a DEC.
HELPS_HUMANS is not needed, because this is a product-scope rule and not a
method convention.

## 2. Background

- **DEC-076** (`SD:667`; D-42). The precedent pair.
  - SURF-011 went to the deliverable whose `deliverable_id` the panel embeds
    (DEL-10-04), not to the no-GUI DEL-09-05.
  - SURF-021 became shared desktop export infrastructure, with DEL-17-02 as
    contract owner only.
  - No scope was expanded in either case.
- **DEC-103 item 9** (`SD:694`). The vendor string must not appear in the
  product. The export is described only as a `.mbf` model batch file. This
  affects the MBF panel's text, but that is rename residue and belongs to A4.
- **Code (freeze).**
  - `apps/desktop/src/features/caepipe-mbf/CaepipeMbfExportPanel.tsx:186`
    carries `deliverable_id: "DEL-17-04"`. The panel is mounted in
    `apps/desktop/src/App.tsx:955` (checked).
  - T5A-C05 records that it was added 2026-06-08 (TP-MAC-50), and that no
    separate authorization was located.
  - `CaepipeExternalHarnessPanel` (DEL-17-05) and `NativePackagePanel`
    (sources DEL-17-03) are imported in `App.tsx:53` and `:86`.
- **Reverse answers are inconsistent across PKG-17**
  (`WAVES/W3/PKG-17/PKG-17_VERIFICATION.md:159`). DEL-17-07/08/09 answer
  CLAIMED_BY for their own panels, DEL-17-04/05 answer PARTIAL, and DEL-17-03
  answers NOT_MINE. The verifier asks R3 to "apply one treatment across the
  PKG-17 panels". Its §7 (line 187) lists the GUI-exclusion contradiction for
  DEL-17-04 and 17-05 as an R4 item.

## 3. Options

**Convention (T3-G5; PKG-17 verifier)**

| Option | What it means | Consequences |
|---|---|---|
| (a) Attribute to the named deliverable and lift its GUI exclusion | SURF-011 pattern | A scope change per deliverable (SOW text and keys). DEL-17-04, 17-05, 17-03, 15-02/03, 15-04, 10-02, 10-03 and 16-04 panels come into scope. The T3-G5 R5 record repairs become possible. |
| (b) Shared desktop infrastructure consuming the deliverable's contract | SURF-021 pattern | No scope change. The SOW exclusions stand. The panels have no deliverable-level acceptance owner (compare B2). |
| (c) Reassign to a PKG-07 GUI owner | e.g. the B2 shell owner, or a feature PKG-07 slice | A scope change. Couples to B2. |
| (d) Withdraw a panel | case by case | A code change. Loses the desktop preview surface. |

**DEL-17-04 case (T5A-C05 decision 2, as it stands):** (a) authorise the
mounted panel under DEL-17-04 and lift the exclusion; (b) assign it to another
owner through scope change; (c) withdraw it.

**DEL-17-05 case.** The same options. Its row `DEL-17-05:SOW#CLM-011` records
the no-code/no-GUI clauses as overtaken by a mounted panel. That row is
STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE, class T5A-C01, route R5 (H4), so
a text catch-up would silently ratify the panel. H4 should mark it
BlockedOnPacket B6.

## 4. Evidence and reliability

| Source | Class | Status |
|---|---|---|
| `SD:667` DEC-076; `SD:694` DEC-103 | GOVERNING | Read at the freeze |
| `CaepipeMbfExportPanel.tsx:186`; `App.tsx:53`, `:86`, `:955` | EVIDENCE (freeze) | Checked by this writer |
| `PKG-17_VERIFICATION.md:159`, `:187` | R2 verification report | Verifier finding |
| DEL-17-04 FG-DEL-17-04-02 rows; DEL-17-05 CLM-011 Notes | R2 sealed ledgers | Effective values |
| T3-G5; T5A-C05 decision 2 | R3 PROPOSAL | Classification only |
| "Added 2026-06-08 (TP-MAC-50); no authorization located" | R2 worker/T5A reading | Not rechecked here (no git use) |

## 5. Affected claims

- **Class T5A-C05** (17 rows; split between **B6** and **C1**). B6's portion is
  the MBF part, **3 rows**, FG-DEL-17-04-02, all IMPLEMENTED_DIFFERENTLY ·
  DOC_BEHIND_CODE · LOCAL_DESIGN:
  - `DEL-17-04:SOW#CLM-003`;
  - `DEL-17-04:SOW#CLM-008`;
  - `DEL-17-04:SOW#CLM-015`.

  Filter: `CLASS_ASSIGNMENTS.csv` `ClassID == T5A-C05 and DeliverableID ==
  DEL-17-04`. The other 14 rows (DEL-12-01 ×4, DEL-15-03 ×8, DEL-16-02 ×1,
  DEL-17-06 ×1) are C1's.
- **Capabilities, T3-G5 (7, `T3_OWNERSHIP.csv`, PARTIAL_UNOWNED_REMAINDER):**

  | Capability | Deliverable | Note |
  |---|---|---|
  | CAP-FEATC-002 | DEL-10-02 | |
  | CAP-FEATC-003 | DEL-16-04 | |
  | CAP-FEATC-007 | DEL-17-05 | |
  | CAP-FEATC-008 | DEL-17-04 | |
  | CAP-FEATC-014 | DEL-15-04 | LOW; its unit-policy part has no named owner |
  | CAP-FEATC-015 | DEL-15-02/03 | |
  | CAP-FEATC-019 | DEL-10-03 | FR-025 export post-beta by DEC-078 |
- **Rows cross-referenced, not in my portion:**
  - `DEL-17-05:SOW#CLM-011` (T5A-C01, H4; recommend BlockedOnPacket B6).
  - CAP-FEATC-021 (native package panel) is decided in B5 item 2b. The
    convention here should apply to it consistently.
- **OtherCorrections.** None on the three claimed rows.
- **Packages.** PKG-17, PKG-15, PKG-10, PKG-16.

## 6. Risks

- **Undecided.**
  - An unauthorised GUI surface sits against a shall-not (T5A-C05 risk).
  - Preview panels with synthesized or fixed packets stay unowned: FEATC-007
    synthesizes rows from invented values, and FEATC-014 and FEATC-015 are
    fixed-packet previews (T3-G5).
  - PKG-17 answers stay inconsistent.
  - A text-only catch-up on DEL-17-05 CLM-011 would ratify the panel without a
    ruling.
- **(a).** Widens several contract-only SOWs at once, which a context-envelope
  check may reject.
- **(b).** No acceptance owner for mounted panels.
- **(c).** Depends on B2.
- **(d).** Removes product surfaces that other rows cite as evidence.

## 7. Recommended routing

No recommendation; owner's call. The evidence does support one point: one
convention, applied to every panel in T3-G5 and to CAP-FEATC-021 (B5), avoids
the per-package inconsistency the PKG-17 verifier recorded. DEC-076 already
offers both patterns (a) and (b).

## 8. On-ruling mechanism

- **Convention.** A DEC by owner ruling that states the rule.
- **(a).** A scope-change handoff per affected deliverable (SOW GUI clause and
  keys). Then R5 record repair of the T3-G5 reverse answers and of the three
  DEL-17-04 rows and DEL-17-05 CLM-011.
- **(b).** R5 record repair only. The reverse answers read COVERS or CONSTRAINS,
  and the SOW exclusions stand.
- **(c).** A scope-change handoff to the named PKG-07 owner.
- **(d).** A CODE_FIX_CANDIDATE brief (H2) removing the mount, under a
  production brief.
- R5 needs separate authorization. Nothing executes until the owner acts.

## 9. Dependencies

- **Coordinates with.**
  - B5 item 2b (native package panel).
  - B2 (option c).
  - B7 (whether desktop previews are the product record; a ruling of B7
    option (c) makes these panels the product surface).
  - A4 (rename residue in the MBF panel's identifiers and text).
- **Blocks.** H4 row DEL-17-05:SOW#CLM-011 and T3-G5 H1/R5 items.
- **Depends on.** None, though a B7 ruling first would simplify the choice.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
packet makes no certification, code-compliance, professional-approval or
engineering-acceptance claim.
