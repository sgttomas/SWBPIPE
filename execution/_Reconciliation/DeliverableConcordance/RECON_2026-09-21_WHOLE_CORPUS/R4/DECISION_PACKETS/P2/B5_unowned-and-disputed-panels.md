# B5 — Panels and helpers with no owner or a disputed owner

Packet for the R4 gate of run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
Writer: TASK P2. This is a proposal only. The path conventions are as in B1
and B4.

## 1. Decision

Six small ownership questions that T1 (decisions 3–6) and T3 left for the
owner. Each can be ruled separately:

1. **Rule-check run panel.** Who owns the only user path for running a rule
   pack?
2. **Attribution panels.** Who owns the validation-evidence review panel and
   the native package review panel? This needs a D-42/DEC-076-style
   attribution ruling.
3. **Hanger landing.** Confirm where the hanger schema, fixtures and library
   GUI land (annex row 23, DEC-103 item 5).
4. **Palette implementation home.** Where do the palette component and command
   catalog land under DEC-094?
5. **Preview fixtures.** Are the preview fixture family and the startup
   preview model product content or demo scaffolding / shared infrastructure?
6. **Shared GUI helper.** Is `core/gui/pkg02_boundary.py` assigned to a
   deliverable, or ruled shared infrastructure?

**Holder: OWNER.** WORKING_ITEMS (workflow: scope-change) follows where a
ruling assigns ownership.

## 2. Background

- **DEC-076** (`SD:667`; D-42, ruled 2026-07-15). This is the precedent for
  items 2 and 6. SURF-011 (build-readiness panel) went to DEL-10-04 per its
  embedded `deliverable_id`, not to the no-GUI DEL-09-05. The export-unit
  disclosure helper became shared desktop export infrastructure, with
  DEL-17-02 as contract owner only. No scope was expanded.
- **DEC-074 O3** (`SD:665`). Names `core/product_preview`, `tools/REGISTRY.md`
  and two coordination tools as shared governed infrastructure. It names the
  service, not the fixture folder (T1 F4).
- **DEC-094** (`SD:685`). DEL-07-09 is the single palette-surface owner, and
  its contract binds to the implemented taxonomy. Yet `_CONTEXT.md:37` says
  implementation lands in annex-named deliverables, and no annex row names a
  landing for `ToolkitPalette.tsx` / `capabilityCatalog.ts` (T7-C04 decision
  3; T3-G9).
- **DEC-103 item 5** (`SD:694`). Vendor hanger tables are a user-imported
  library class, never bundled. It says the landed `LibraryKind::Hanger` and
  `hanger.schema.yaml` are the extension that annex item 23 anticipated,
  "recordable in the DEL-07-09 coverage ledger". Annex row 23 (`AX:127`)
  names the landings. DEL-03-07's reverse answers read DEC-103 as routing the
  schema to DEL-07-09. T1 O-1 flags that reading as a possible misreading.
- **Code (freeze).**
  - The rule-check run panel ships. PKG-06 partly owns the runner
    (CAP-COREC-044, whose owner is itself an H1 item). DEL-07-03, 07-04,
    07-05, 07-09 and all of PKG-06 disclaim the panel (T1 F2).
  - `core/gui/pkg02_boundary.py:10` holds literal DimensionId sets. This is
    recorded by the FIRM resolution on DEL-02-02:SOW#CLM-020.
  - The native package panel builds its own review record instead of calling
    `core/handoff/native_json` (T1 O-6; RC-17-0086).

## 3. Options

| Item | Options as they stand in the evidence | Consequences |
|---|---|---|
| 1. Rule-check run panel (CAP-FEATB-008/009; CAP-WSUI-035 follows) | (a) DEL-07-04; (b) DEL-06-02 (runner partial owner); (c) CREATE | Each is a scope change for a GUI surface no SOW names. CAP-WSUI-035 (aggregate rule-check status, best existing DEL-05-04) goes with the chosen owner. The desktop commands CAP-SHELL-040/041 → DEL-06-03 are H1 routing-gap items. |
| 2a. Validation evidence review (CAP-FEATB-031) | (a) DEL-09-04; (b) DEL-10-04 (DEC-076 precedent); no single embedded identity (packet refs DEL-09-04, 09-05, 10-04; `ValidationEvidencePanel.tsx:142` per T1) | A DEC-076-style attribution. No PKG-09 key claims a GUI panel, so (a) needs a GUI scope addition. |
| 2b. Native package review (CAP-FEATC-021) | (a) shared desktop infrastructure (DEC-076 precedent); (b) a PKG-07 owner; (c) DEL-17-03 by lifting its GUI exclusion (links B6) | The parallel builder may diverge from the owned writer (T1 O-6). The canonical-path question is B7. |
| 3. Hanger landing (CAP-DATA-006, CAP-DATA-051, CAP-FEATC-016) | (a) Confirm annex row 23: schema DEL-03-02, fixtures DEL-03-07, library GUI DEL-07-03; (b) read DEC-103 item 5 as routing to DEL-07-09 (DEL-03-07's reading) | (a) Then R5 record repair of the reverse answers. (b) Conflicts with DEL-07-09's coverage-only envelope (compare B4 (1a)). |
| 4. Palette home (CAP-VIEW-025/026) | (a) DEL-07-09; (b) DEL-07-01; (c) DEL-07-02 (T7-C04 decision 3) | (a) Needs the DEL-07-09 envelope amendment (as in B4 (1a)). (b)/(c) Are SOW amendments plus a DEC or annex note recording the landing. |
| 5. Preview fixtures (CAP-DATA-058, CAP-SHELL-008) | (a) Confirm DATA-058 as shared governed infrastructure by analogy to DEC-074 O3; (b) assign it (DEL-13-01 cites the design-knowledge half); for SHELL-008: product default content vs demo scaffolding | (a) Then NO_ACTION. If product content, it needs an owner and IP/provenance review. CAP-PHYS-034 (NON_DELIVERABLE, NO_ACTION) shares the FIELD resolution. |
| 6. Shared GUI helper (CAP-COREB-030) | (a) Assign to DEL-07-04; (b) DEC-076-style shared-infrastructure ruling | (a) Scope change. (b) The FIRM defect (literal DimensionId set) then has no deliverable owner. It stays visible on DEL-02-02 CLM-020 (T6 class, H2). |

## 4. Evidence and reliability

| Source | Class | Status |
|---|---|---|
| `SD:665`, `SD:667`, `SD:685`, `SD:694`; `AX:127`; DEL-07-09 `_CONTEXT.md:37` | GOVERNING | Read at the freeze |
| DEL-02-02:SOW#CLM-020 FIRM; DEL-13-01 REQ-13-01-011 FIELD | R2 adopted resolutions | Adopted |
| RC-17-0086 and the DEL-03-07 reverse reasons | R2 sealed ledgers | Via T1 |
| `ValidationEvidencePanel.tsx:142`, `pkg02_boundary.py:10` | EVIDENCE (freeze) | Cited by T1 and the FIRM resolution; not re-read here |
| T1 F2/F3/F4, O-1, O-6; T2-G6; T3-G9; T7-C04 decision 3 | R3 PROPOSAL | Classification only |

## 5. Affected claims

No class in `CLASS_INDEX.csv` is in B5's portion. The portion is
capability-level (13 capabilities):
- **Item 1:** CAP-FEATB-008, CAP-FEATB-009 (T1, PRODUCT_UNOWNED);
  CAP-WSUI-035 (T2, PRODUCT_UNOWNED, LOW).
- **Item 2:** CAP-FEATB-031, CAP-FEATC-021 (T1, PRODUCT_UNOWNED).
- **Item 3:** CAP-DATA-006, CAP-DATA-051 (T1, ROUTING_GAP), CAP-FEATC-016 (T1,
  ROUTING_GAP).
- **Item 4:** CAP-VIEW-025, CAP-VIEW-026 (T3-G9, PARTIAL_UNOWNED_REMAINDER).
- **Item 5:** CAP-DATA-058 (T1, NON_DELIVERABLE, OWNER_DECISION to confirm),
  CAP-SHELL-008 (T2, PRODUCT_UNOWNED, LOW).
- **Item 6:** CAP-COREB-030 (T1, PRODUCT_UNOWNED, LOW).

Rows cross-referenced, not in my portion:
- **`DEL-07-09:PALETTE_ORGANIZATION_CONTRACT#organization-and-ownership`**
  (UNKNOWN · AUTHORITY_UNCLEAR). It is in class T7-C04, which topic **A8**
  takes ("palette landing source"). B5 and A8 therefore touch the same
  decision. See the coverage note in INDEX.md.
- **`DEL-13-01:SOW#CLM-009/REQ-13-01-011`** (T7-C05; the preview
  design-knowledge exception). This is topic **C1**. Item 5 decides the
  fixtures' status, and C1 decides the requirement's intent.
- **`DEL-02-02:SOW#CLM-020`** (FIRM; AuthorityNeeded NO). A code-fix
  candidate in H2.
- **CAP-COREB-046** (hanger import validation, T3-G9, DEL-03-07 by scope
  extension). An H1 item that follows item 3.
- **Rows known only from OtherCorrections.** The FIELD resolution on
  DEL-13-01 REQ-13-01-011 (VerificationEvidence re-pointed) touches
  CAP-DATA-058 and CAP-PHYS-034.
- **Deliverables touched.** DEL-07-01/02/03/04/09, DEL-06-02/03, DEL-05-04,
  DEL-09-04, DEL-10-04, DEL-17-03, DEL-03-02, DEL-03-07, DEL-13-01, DEL-02-02.

## 6. Risks

- **Undecided.**
  - The only user path to run rule checks has no owner.
  - Two review panels embed or build packets without a responsible
    deliverable. One may diverge from the owned writer.
  - The hanger landing rests on a possibly misread ruling.
  - The palette code has no owning deliverable.
  - Shipped preview fixtures carry an unclear product status.
- **By item.**
  - Items 2 and 6 under the shared-infrastructure option leave defects without
    a deliverable owner.
  - Items 3(b) and 4(a) collide with DEL-07-09's accepted envelope (see B4).
  - Item 5, if the fixtures are product content, needs a provenance record
    (IP review, compare C5).

## 7. Recommended routing

- **Item 3.** The evidence leans one way. The DEC-103 text says only
  "recordable in the DEL-07-09 coverage ledger". Annex row 23 names DEL-03-02
  and DEL-03-07, and DEL-07-09 is coverage-only. Confirming option (a) is
  consistent with all three records. This is a reading, not a ruling. The
  owner decides.
- **Items 1, 2, 4, 5 and 6.** No recommendation; owner's call.

## 8. On-ruling mechanism

- **Assignment to an existing deliverable** (items 1, 2, 4, 6(a)). A
  scope-change handoff amending the named SOW and issuing keys. Then R5 record
  repair of the reverse answers.
- **CREATE** (item 1(c)). A scope-change handoff with a new DAG node.
- **Shared-infrastructure classification** (items 2b(a), 5(a), 6(b)). A DEC by
  owner ruling (DEC-076 pattern). R5 records the status. No scope change.
- **Hanger confirmation** (item 3(a)). The ruling is recorded in the decision
  log. R5 record repair of DEL-03-02, DEL-03-07 and DEL-07-03 reverse answers
  and keys (H1 KEY_ISSUE/ASSIGN).
- **Palette under (a).** A DEL-07-09 envelope amendment through scope-change,
  consistent with B4.
- R5 needs separate authorization. Nothing executes until the owner acts.

## 9. Dependencies

- **Coordinates with.**
  - A8 (palette landing source row).
  - B4 (DEL-07-09 envelope).
  - B6 (the native package panel against DEL-17-03's GUI exclusion).
  - B7 (canonical native-JSON path).
  - C1 (preview design-knowledge exception).
  - C5 (fixture provenance).
- **Blocks.** H1 items for the 13 capabilities, CAP-COREB-046, and
  CAP-SHELL-040/041 (desktop rule-check commands).
- **Depends on.** None.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
packet makes no certification, code-compliance, professional-approval or
engineering-acceptance claim.
