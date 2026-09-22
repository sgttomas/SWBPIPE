# B4 — Landing of the load-case, support and self-weight editors (DEC-094 vs the DEL-07-09 envelope) and the DEL-07-02/07-03 editor boundary

Packet for the R4 gate of run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
Writer: TASK P2. This is a proposal only. The path conventions are as in B1.
`AX` is `execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md`
at the freeze.

## 1. Decision

1. **Editor landing.** Which deliverable holds the implemented load-case and
   support editor surface? That surface is the Load Case Manager, load-case
   intents and combination authoring, plus the self-weight panel. DEC-094
   re-points the DEL-07-03 R-005/R-006 landing to DEL-07-09, but DEL-07-09's
   accepted envelope excludes editor implementation. This is the W1 authority
   item that the W1 gate ruling sent to R4.
2. **Editor boundary.** Where do entity-creation forms and direct table
   editing sit, DEL-07-02 or DEL-07-03?

**Holder: OWNER.** If ownership moves between deliverables, WORKING_ITEMS
(workflow: scope-change) follows.

## 2. Background

**Earlier decisions.**
- **DEC-094** (`SD:685`; SCA-009 Gates 1–2, 2026-08-20).
  - Adds DEL-07-09 (envelope L) as the single palette-surface owner, and routes
    palette commands through PKG-16.
  - Re-points "DEL-07-03-R-005/R-006 ownership landing" to DEL-07-09.
  - Changes no existing requirement.
- **DEL-07-09 `_CONTEXT.md:37`** (freeze). "Contract/coverage slice only". It
  says implementation lands in the annex-named deliverables and that editor
  implementation does not land in DEL-07-09.
- **DEL-07-03.**
  - It records R-005 as ACCEPTED_DIVERGENCE (T1, citing
    `ScopeOfWork.md:131`).
  - Its CLM-041 forbids taking the editor without an accepted binding (T1,
    citing `ScopeOfWork.md:552-560`).
- **Annex rows** (freeze):
  - Row 17 (`AX:116`) lands the section editor surface in DEL-07-03.
  - Row 22 (`AX:126`), automatic self-weight case generation, names DEL-05-01
    for the engine and no GUI landing.
  - Rows 3 and 17 put the material and section editors in DEL-07-03.
  - Rows 2, 12 and 13 put support creation in DEL-07-02 (T3-G6).
  - Row 5 puts load GUI in DEL-07-02's inspector forms (T3-G6).
- **W1 gate ruling** (`WAVES/W1/W1_GATE_RULING.md`, Effect item 5). It says the
  DEC-094/SCA-009 R-005 landing is "Not ruled here" and sends it to R4 as an
  authority item.
- **DEL-07-02 CLM-034.** The PROJECT_BASELINE boundary rationale calls the
  inspector a convenience surface (`…/DEL-07-02_…/ScopeOfWork.md:389-397`, via
  T5B-C09). Creation forms and direct table editing landed under DEL-07-02 in
  PRs #832–#834.

**What the code does now.** The Load Case Manager ships, and no PKG-07
deliverable claims it (W1 assessment, "Not in this remedy"). The load GUI is
built in the Load Cases manager panel, not in the inspector forms that annex
row 5 names (CAP-FEATB-024 note).

## 3. Options

**Question 1: editor landing (T1 F2 decision 2)**

| Option | Consequences |
|---|---|
| (a) Amend DEL-07-09's envelope so the editor lands there | Makes DEC-094's re-point executable. Envelope L may need re-sizing ("split if it expands", `_CONTEXT.md:36`). DEL-07-09 stops being coverage-only. Interacts with B2 (iii-b) and B11. |
| (b) Rebind to DEL-07-03 through the change path | Restores R-005/R-006 to DEL-07-03. The DEC-094 re-point needs amending, and DEL-07-03's accepted divergence and CLM-041 are revised. The C7 reading of the six contested T6-C09 rows becomes "replace". |
| (c) CREATE a load-case and support editor deliverable | Scope change: new node, SOW and keys. DEC-094's re-point is re-pointed again to the new owner. |

The self-weight panel (CAP-FEATC-031) follows the same choice. Its candidates
are the editor owner chosen here or DEL-05-01 (T1).

**Question 2: editor boundary (T5B-C09 decision 2)**

| Option | Consequences |
|---|---|
| (a) DEL-07-02 holds creation forms and table editing | DEL-07-02's SOW records it (R5 record repair of CLM-034 and the related T3-G6 keying gaps CAP-FEATC-034, CAP-FEATB-024). |
| (b) DEL-07-03 holds them, consistent with annex rows 3 and 17 | Ownership moves. That is a scope-change handoff. |

## 4. Evidence and reliability

| Source | Class | Status |
|---|---|---|
| `SD:685` DEC-094; `AX:116`, `AX:126` | GOVERNING | Read at the freeze |
| DEL-07-09 `_CONTEXT.md:36-37` | GOVERNING (accepted envelope) | Read at the freeze |
| `W1_GATE_ASSESSMENT.md` "Not in this remedy"; `W1_GATE_RULING.md` Effect 5 | R2 gate record / owner ruling | Owner ruling explicitly did not decide this |
| `WAVES/W1/RESOLUTIONS.csv` rows for DEL-07-03 (6 CONTESTED, "Awaits R4 authority reading: DEC-094 vs SCA-009 annex") | R2 adopted resolutions | Adopted |
| DEL-07-03 SOW lines 131, 552-560; DEL-07-02 SOW 389-397 | GOVERNING (deliverable scope) | Cited by T1/T5B, not re-read here |
| T1 F2, O-2; T5B-C09 | R3 PROPOSAL | Classification only |

## 5. Affected claims

- **Class T5B-C09** (5 rows, OWNER, OWNER_DECISION). This class is **split in
  substance but not listed as split in the topic file.** B4 claims **1 row**:
  `DEL-07-02:SOW#CLM-034` (IMPLEMENTED_UNDOCUMENTED · SCOPE_GREW_BY_DIRECTION
  · PROJECT_BASELINE). The other 4 rows are DEL-11-01 (FG-DEL-11-01-01, the
  user-guide exception). They match topic C4 and are not claimed here.
  Filter: `ClassID == T5B-C09 and DeliverableID == DEL-07-02`.
- **Capabilities (T1_UNMAPPED.csv, PRODUCT_UNOWNED, 4):** CAP-FEATB-022, 023,
  025 and CAP-FEATC-031.
- **Rows that depend on this ruling but belong to C7** (class T6-C09, not
  claimed): `DEL-07-03:SOW#CLM-012/DEL-07-03-R-005`,
  `DEL-07-03:SOW#CLM-005.r04`, `DEL-07-03:SOW#CLM-008/DEL-07-03-R-005`,
  `DEL-07-03:SOW#CLM-017`, `DEL-07-03:SOW#CLM-018` and
  `DEL-07-03:SOW#CLM-012/DEL-07-03-R-002`. These are the six W1 CONTESTED rows. Their
  candidate reading is `UNKNOWN · AUTHORITY_UNCLEAR · PROJECT_BASELINE ·
  RECORD · OWNER`, and it is known only from `OtherCorrections` in
  `WAVES/W1/RESOLUTIONS.csv`. C7 confirms or replaces them. Their substance
  turns on Question 1 here.
- **Related keying items (H1/H4, not claimed).** CAP-FEATC-034 and
  CAP-FEATB-024 (DEL-07-02 UNKEYED), CAP-VIEW-029 (routing gap → DEL-07-03),
  CAP-PHYS-028 (self-weight planning, DEL-05-01 UNKEYED).
- **Packages and deliverables.** PKG-07 (DEL-07-02, 07-03, 07-09), PKG-05
  (DEL-05-01, 05-02).

## 6. Risks

- **Undecided.** As recorded, no deliverable can hold the load-case or support
  editor (T1 O-2), and it ships unowned. The six DEL-07-03 rows stay contested.
  For DEL-07-02 and DEL-07-03, ownership of shipped editing surfaces stays
  ambiguous.
- **(1a).** DEL-07-09 loses its coverage-only character. That interacts with
  its lifecycle question (B11) and its envelope L.
- **(1b).** Needs an amendment of a ruled DEC-094 clause.
- **(1c).** A new node and DAG edges.
- **(2b).** Moves shipped work between deliverables. Their histories need R5
  catch-up.

## 7. Recommended routing

No recommendation; owner's call. The evidence shows that DEC-094 as written
cannot be executed without either option (1a) or an amendment of DEC-094.
Doing nothing is not a stable state.

## 8. On-ruling mechanism

- **(1a).** A scope-change handoff amending DEL-07-09's context envelope and
  SOW, then R5 record repair of DEL-07-09 and DEL-07-03 records.
- **(1b).** A new DEC amending the DEC-094 re-point clause, then a scope-change
  handoff restoring R-005/R-006 in DEL-07-03 (change path for its accepted
  divergence rows), then R5.
- **(1c).** A scope-change handoff creating the deliverable. A DEC records the
  re-point.
- **(2a).** R5 record repair of DEL-07-02 CLM-034 and the related keying
  items.
- **(2b).** A scope-change handoff moving creation and table editing to
  DEL-07-03.
- C7's reading of the six T6-C09 rows is recorded after this ruling. R5 needs
  separate authorization.

## 9. Dependencies

- **Blocks.**
  - C7 (T6-C09 confirm or replace).
  - H1 items CAP-FEATB-022/023/025, CAP-FEATC-031, CAP-FEATC-034,
    CAP-FEATB-024.
  - H4 rows on DEL-07-02 CLM-034 if (2a).
- **Related.**
  - B2 (DEL-07-09 as operations-UI owner, (iii-b)).
  - B5 (palette landing under the same DEC-094).
  - B11 (DEL-07-09 lifecycle).
  - B1 (self-weight engine keying).
- **Depends on.** None.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
packet makes no certification, code-compliance, professional-approval or
engineering-acceptance claim.
