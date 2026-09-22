# R3 integration — topic assignment

This file assigns every owner-decision candidate raised in R2 and R3 to
exactly one packet writer (P1–P3). It also defines the handoff scopes
(H1–H4). The candidates come from the R3 task outputs and the W2/W3 owner
lists. A candidate that fits no topic goes back to Agent 0 as UNASSIGNED.

**Sources.** `R3/TASKS/*` and `R3/CLASS_INDEX.csv`. The owner lists are
`WAVES/W2/W2_GATE_ASSESSMENT.md` ("Items for R3/R4") and
`WAVES/W3/W3_ASSESSMENT.md` ("Owner items added in W3"). Contested clusters
come from `R3/TASKS/T8_CLUSTERS.md`.

## P1 — authority, baseline and architecture (`R4/DECISION_PACKETS/P1/`)

| ID | Topic | Main sources |
|---|---|---|
| A1 | DEC-009 Rust core vs Python/TypeScript engines: port, permit or split. Includes the PKG-16 Python engines not yet in the cluster | T8 DEC-009; T7-C03; T4A-C06; T11 S-05; W3 |
| A2 | JSON hash basis and "JCS-compatible" labels (six readings, 13 deliverables; the DEL-17-03 AC-001 wording; the MBF label) | T7-C02; T11 T-01/R-01/SS-04; W2; W3 |
| A3 | Holds that code settled with no ruling (CP-10), about 12 topic rulings; whether CP-10 holds go to the owner or to record catch-up | T7-C01; T11 T-03; W2 |
| A4 | Rename and identity residue: one ruling. Covers text residue, default code identifiers, the four identifiers kept on 2026-09-18, and the extra identifiers T4B named (`openpipestress-projects.sqlite3`, `openpipestress_result_semantics_v0_2`, `open_pipe_stress_declared_expression`). Also picks the one route for `DEL-01-01:SOW` | T4B-C01; T11 S-06; W3 |
| A5 | PKG-00 SEMANTIC_READY: target lifecycle state and workflow (Direction 8), and restating the injection statements | T9-C07; T4B-C03; T5B-C04; T8 SR-1 |
| A6 | ISSUED DEL-01-01 change path: reissue, amend, or keep as history. Covers the DEC-081 Wave 2 edit after issue and the double-routed SOW row | T4A-C08; T4B-C02; T9-C05/C06/C08/C10; T5B-C07 (DEL-01-01 part) |
| A7 | The deleted export plan (restore, re-point or retire), and the missing N7 intake evidence | T8 EXPORT_PLAN; T4B-C04; T4A-C06; T9-C03 |
| A8 | Product posture (FOSS vs source-available), and governing-source conflicts or silences (pre-release legal review, palette landing source, secret provider) | W2; T7-C04 |
| A9 | The D-41 "current declaration" blocks: re-pin or retire (213 rows) | T4A-C02 |
| A10 | Baseline rows overtaken by rulings: D-68 vs the DEL-00-05 accessibility hold, SCA-004 vs the DEL-00-07 formats, DEC-058 for DEL-08-05, DEC-051 vs the DEL-12-05 disclosure principle, and the release-label floor PB-TBD-003 | T5B-C07 (non-DEL-01-01); T4A-C06; W2; W3 |

## P2 — ownership, scope and product path (`R4/DECISION_PACKETS/P2/`)

| ID | Topic | Main sources |
|---|---|---|
| B1 | Product solve path unowned: `core/product_physics` and the DEC-044 nonlinear loop | T1 (decision 1); T3 G2/G3; W2 ownership gaps |
| B2 | Desktop workspace shell unowned, including session state CAP-WSUI-011 and the operations UI | T2 G1/G4; T3 G4 |
| B3 | Runtime model-operation applier (outside the DEL-16-02 and DEL-16-03 scopes; DEC-020 seam) | T3 G1; T5A-C04 |
| B4 | Load-case and self-weight editors: DEC-094 vs the DEL-07-09 envelope, and the DEL-07-02/07-03 editor boundary (includes the W1 DEC-094/SCA-009 R-005 item) | T1 (decision 2); T5B-C09; W1 |
| B5 | Panels with no owner or a disputed owner: the rule-check run panel, the attribution panels (FEATB-031, FEATC-021), the hanger landing, the palette landing (VIEW-025/026), preview fixtures and the shared GUI helper | T1 (decisions 3–6); T3 |
| B6 | Convention for panels that name a deliverable whose SOW excludes GUI (the DEL-17-04 MBF panel, DEL-17-05) | T3 G5; T5A-C05 (MBF part); W3 |
| B7 | Canonical handoff path: wire the Python engines, port them, or treat the desktop previews as the product. Covers PKG-15 schema compliance and the prover packet field | T12-C01; W3 (PKG-15) |
| B8 | Whether model-state persistence and comparison enter the product | T12-C02 |
| B9 | PKG-13 product status (wire, hold or retire), including the Knowledge panel schema bypass and the constraint status that no engine produces | T12-C03; W3 (PKG-13) |
| B10 | Plugin and adapter runtime and grant model; PDU-034 quarantine taxonomy | T6-C04 D2/D3; T12-C04/C05 |
| B11 | Promotion of DEL-07-09 | T9 |
| B12 | The other held owner selections blocking implementation: human acceptance workflow OI-007, private-library storage roots and secret provider, telemetry routes, unit namespaces, result-envelope home and comparison schema, results-viewer scope, ADR vs decision log, the DEC-028 compatibility window, rule-pack combinations, adapter families, mechanics residuals, release/QA/stage gates, PDU-031, CF-001/CF-002 | T6-C04 D1, D4–D15; W3 |

## P3 — product intent, evidence, validation and method (`R4/DECISION_PACKETS/P3/`)

| ID | Topic | Main sources |
|---|---|---|
| C1 | Code contradicts the SOW; rule on intent. Covers DEL-15-03 redaction (PR #307), diagnostic class and remediation, bare numbers at the import gate, the preview design-knowledge exception, the DEC-026 tolerance, DEL-16-02 runtime schema validation, the DEL-12-01 path-class vocabulary and the DEL-17-06 table inventory | T7-C05; T5A-C05 (non-MBF); W2; W3 |
| C2 | Protected-content review: records not located, and what counts as a record. Also PR #787's removed protected check and the absence of protected-content reviews | T7-C09; T5B-C02; T11 T-02/R-02; T7-C06 (DEL-04-04 row); W2 |
| C3 | Validation holds, activation of the external prover, and the independent usability/security basis | T9-C11; T7-C08; T7-C07 (owner-facing part) |
| C4 | Scope of the DEL-11-01 user-guide exception; the DEL-11-04 professional-boundary list vs DEC-081/DEC-107 | W3 |
| C5 | IP and provenance: the PCF fixture matches a published table; the PCF and glTF fixtures have no provenance record | W3 |
| C6 | Method items for owner or HELPS_HUMANS adoption:<br>- the rerun-rule reading (PKG-08, PKG-15);<br>- whether PKG-12's post-gate 9.5% should come to the owner;<br>- the `.sNN` batch blind spot;<br>- the routing-design gap (`R3/ROUTING_GAPS.csv`);<br>- the DEL-15-02 tier hint;<br>- a corpus-level F7 ruling;<br>- the tier for "Still TBD item since ruled";<br>- a reverse-pass tie-break;<br>- the diagnostics boundary split (T11 SS-01 is its engineering part; see H3) | W3; T11; T12 obs 2; T5B obs 2; T3 O-6 |
| C7 | The readings Agent 0 can record without a ruling, presented for owner confirmation:<br>- SR-1 cause RECORD_DRIFT;<br>- unit-vocabulary tier follows the gap;<br>- F1 reaches CONTEXT rows;<br>- the scope-item test for in-scope REQ tier;<br>- acceptance-workflow cause DEFERRED_BY_RULING | T8 |

## Handoffs

| ID | Scope | Output folder |
|---|---|---|
| H1 | Scope change. The T1–T3 capabilities routed SCOPE_CHANGE_HANDOFF or classified PRODUCT_UNOWNED, ROUTING_GAP, PARTIAL_UNOWNED_REMAINDER, UNKEYED_SCOPE_GAP or DUPLICATE; T6-C07; the VIEW-017 duplicate. Cite packets B1–B12 where an owner choice comes first | `R3/SCOPE_CHANGE_HANDOFF/` |
| H2 | Code-fix candidates. Every row routed CODE_FIX_CANDIDATE in `CLASS_ASSIGNMENTS.csv` (T4A-C07, T6-C01/C02/C03, T7-C06), plus the T12 CODE_FIX_CANDIDATE rows. Mark briefs blocked by packets (for example the DEL-15-02 defaults wait on B7) | `R3/CODE_FIX_BRIEF_CANDIDATES/` |
| H3 | Engineering and review register. Every row or class routed ENGINEERING_AUTHORITY or REVIEW, across `CLASS_ASSIGNMENTS.csv` and the T1–T3, T8, T9, T11 and T12 CSVs | `R3/ENGINEERING_AUTHORITY/` |
| H4 | R5 record-repair tranche proposal. Every row routed R5_RECORD_REPAIR in `CLASS_ASSIGNMENTS.csv` (2,099), plus T9 and T11 record-repair items. Mark rows blocked by packets A4, A5, A6, A7 or A9 | `R4/R5_TRANCHE_PROPOSAL/` |

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
