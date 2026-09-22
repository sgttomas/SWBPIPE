# R4 decision packets — P2 (ownership, scope and product path)

Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration. Writer: TASK P2
(parent HELP_HUMAN Agent 0). These are proposals only. Nothing executes until
the owner acts, and R5 needs separate authorization.

| ID | Question | Holder | File |
|---|---|---|---|
| B1 | Who owns the product solve path: `core/product_physics`, the DEC-044 nonlinear loop, and the product section and mass routine (PHYS-007)? | OWNER, then WORKING_ITEMS (workflow: scope-change) | `B1_product-solve-path-owner.md` |
| B2 | Who owns the desktop workspace shell, session state (CAP-WSUI-011) and the operations UI? | OWNER, then WORKING_ITEMS (workflow: scope-change) | `B2_desktop-workspace-shell.md` |
| B3 | Who owns runtime model-operation application, and which contract governs runtime edits (SS-02)? | OWNER | `B3_runtime-operation-applier.md` |
| B4 | Where do the load-case, support and self-weight editors land (DEC-094 against the DEL-07-09 envelope), and where is the DEL-07-02/07-03 editor boundary? | OWNER | `B4_load-case-editor-landing.md` |
| B5 | Who owns the rule-check run panel, the two attribution panels, the hanger landing, the palette implementation home, the preview fixtures and the shared GUI helper? | OWNER | `B5_unowned-and-disputed-panels.md` |
| B6 | What is the convention for panels that name a deliverable whose SOW excludes GUI (DEL-17-04 MBF, DEL-17-05)? | OWNER | `B6_panels-vs-no-gui-sows.md` |
| B7 | Which handoff and export path is canonical: wire the Python engines, port them, or treat the desktop previews as the product (PKG-15/17)? | OWNER | `B7_canonical-handoff-path.md` |
| B8 | Do model-state persistence and comparison (PKG-14, DEL-08-06) enter the product? | OWNER | `B8_model-state-and-comparison.md` |
| B9 | What is PKG-13's product status (wire, hold or retire), including the Knowledge panel schema bypass and the unowned desktop constraint status? | OWNER | `B9_pkg13-product-status.md` |
| B10 | Should the plugin and adapter runtime, the grant model and the PDU-034/PDU-004 quarantine taxonomy be selected or held? | OWNER | `B10_plugin-adapter-runtime-and-grants.md` |
| B11 | Should lifecycle review or promotion of DEL-07-09 be opened? | OWNER (lifecycle workflow) | `B11_del-07-09-promotion.md` |
| B12 | The other held owner selections: T6-C04 D1 and D4–D15, CF-001/CF-002 and PDU-031 | OWNER | `B12_other-held-owner-selections.md` |

## Portions of split classes

| Class | Class rows | P2 portion | Filter or keys |
|---|---|---|---|
| T6-C04 | 87 | B10: 20 (D2 14, D3 6); B12: 67 (D1, D4–D15) | The key lists are in B10 §5 and B12 §5. Together they cover the class with no overlap (script-checked). |
| T5A-C05 | 17 | B6: 3 | `ClassID == T5A-C05 and DeliverableID == DEL-17-04`. The other 14 rows are C1's. |
| T5B-C09 | 5 | B4: 1 (`DEL-07-02:SOW#CLM-034`) | This class is **not** on the topic file's split list. The other 4 rows are DEL-11-01 and match C4. |

Other classes wholly in P2: T5A-C04 (19; B3; route R5, carried by H4 with
BlockedOnPacket B3). T12 clusters in P2 (these are task clusters, not
`CLASS_INDEX` classes):
- T12-C01 213 (B7);
- T12-C02 95 (B8);
- T12-C03 112 (B9);
- T12-C04 57 and T12-C05 27 (B10; one key shared with T6-C04 is counted once;
  103 distinct B10 keys).

Capabilities claimed once across B1–B6: 101. T9 item: 1 (B11). T11 items: 3
(B3).

## Coverage notes for Agent 0

1. **CF-001/CF-002.** The rows `DEL-12-04:SOW#CLM-028/DEL-12-04-CF-001` and
   `-CF-002` are in T7-C04. A8 takes that class ("secret provider"), and the
   topic text also names CF-001/CF-002 under B12. B12 drafts the decision but
   does not count the rows.
2. **Palette landing.** The row
   `DEL-07-09:PALETTE_ORGANIZATION_CONTRACT#organization-and-ownership` is in
   T7-C04 (A8, "palette landing source"). B5 item 4 decides the same landing
   for CAP-VIEW-025/026. It should be ruled with A8.
3. **PB-TBD-003.** The row `DEL-09-05:STATUS#remaining/R01` (T6-C04 D14, in
   B12) is the same subject as A10's release-label floor (T4A-C06 rows
   DEL-09-05 CLM-012/RQG-007 and CLM-029). It should be ruled with A10.
4. **T5B-C09.** Split between B4 and C4 in substance, but not listed as a
   split class.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
