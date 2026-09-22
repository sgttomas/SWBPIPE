# H2 — code-fix brief candidates

Run HELP-HUMAN-PIPING-20260921-RECONCILIATION, R3 integration, TASK H2 (parent:
HELP_HUMAN Agent 0). This is a proposal for the R4 gate. None of these briefs
is executed. Executing one needs an owner-steered production brief, then the
chirality-change PR path, then re-verification of the affected rows in a later
concordance. Ledger rows are not edited in place. Where a brief chooses a
"narrow the requirement" branch, that is an R5 record repair (H4), which needs
separate R5 authorization. Standard claim fence applies (F-PIP-2; claims
taxonomy per DEC-081). This file makes no certification, code-compliance,
professional-approval or engineering-acceptance claim.

## Scope and sources

The scope comes from `R3_INTEGRATION_TOPICS.md` (H2):

- every row routed CODE_FIX_CANDIDATE in `R3/CLASS_ASSIGNMENTS.csv`: T4A-C07 (3),
  T6-C01 (180), T6-C02 (116), T6-C03 (34) and T7-C06 (42), 375 rows in all;
- the CODE_FIX_CANDIDATE rows of `R3/TASKS/T8_ROWS.csv` (17) and
  `R3/TASKS/T12_UNREACHED.csv` (17);
- the six W3 product and code items (`WAVES/W3/W3_ASSESSMENT.md`, "Owner items
  added in W3").

The basis is read-only. It covers effective values from `R3/CORPUS_CLAIMS.csv`,
the class and route from `CLASS_ASSIGNMENTS.csv`/`CLASS_INDEX.csv`, and the
RemainingWork, ImplementationEvidence and Notes from the sealed `_forward.csv`
ledgers (no `superseded_<n>/`). Where a FIRM or FIELD `OtherCorrections` entry
restates RemainingWork, the briefs use it and mark it `OC:`. Freeze line
citations were re-read at `00115c719` as code reading only. H2 ran no builds,
tests or git commands.

## Coverage

The deduplicated total is **380 claim rows + 6 items = 386 rows** in
`CODE_FIX_ROWS.csv`.

- All 375 class rows are present, and each class count matches
  `CLASS_INDEX.csv`.
- All 17 T8 rows are present. 12 of them are also code-fix class rows. The other five
  enter only through T8:
  - the three DEL-03-07 rows of class T7-C05, whose class route is
    OWNER_DECISION;
  - the two NOT_DIVERGENT rows `DEL-14-01:SOW#CLM-024` and
    `DEL-14-02:SOW#CLM-024`.
- All 17 T12 rows are present, and all of them are also code-fix class rows.
- The six W3 items are ITEM rows (`W3-PC-01`..`06`). Each also maps to claim
  rows in the same brief.

A script checked this reconciliation (`CODE_FIX_ROWS.csv` against
`CLASS_ASSIGNMENTS.csv`, `CLASS_INDEX.csv`, `T8_ROWS.csv`,
`T12_UNREACHED.csv`, `T8_ROUTE_DISAGREEMENTS.csv` and every CFB file). Results:

- no duplicate keys;
- every class portion sums to its class total;
- every CFB file lists exactly its CSV rows.

The classes here belong wholly to H2. None of them is a packet split class.

## How to read `BlockedOnPacket`

- **A topic ID (A1–A10, B1–B12, C1–C7)** means the row waits on that R4
  decision packet. That is the case when:
  - the row's T8 or T12 reading needs an owner or review decision;
  - the topic file names the block, as for the DEL-15-02 defaults behind B7;
  - the ledger RemainingWork waits on a selection that a packet carries.
- **`H3[<class>]`** means the H3 engineering-and-review register item for that
  class, T6-C03 or T7-C06. Both classes have owning authority REVIEW, so an
  independent review must come before any repair is relied on. Agent 0 maps
  `H3[<class>]` to the H3 item ID at integration (`H3_TOKEN_MAP.csv`: ER-21, ER-22).
- **An H3 item ID (`ER-20`)** is used directly where a row waits on a named
  H3 reviewer choice (DEL-12-01 LFSP-REQ-011).
- A dependency marked "context" in a brief is related but not blocking.

Blocked rows: **155 of 380 claim rows**, plus 5 of the 6 items (revised after the RV6/RV7 reviews; previously 140).

| Blocker | Rows |
|---|---|
| H3[T7-C06] | 42 + 5 items |
| H3[T6-C03] | 34 |
| B7 | 30 |
| B10 | 19 |
| C7 | 16 |
| A2 | 12 |
| C1 | 10 |
| A1 | 9 |
| B8 | 9 |
| B12 | 6 |
| B9 | 5 |
| C6 | 5 |
| A7 | 4 |
| A6 | 3 |
| B3 | 3 |
| C4 | 2 |
| A3 | 1 |
| C2 | 1 |
| C5 | 1 |
| ER-20 | 1 |

A row can carry more than one blocker. Across the briefs:

- 18 are wholly blocked;
- 18 are unblocked;
- 18 are partly blocked. Their unblocked rows can go ahead separately.

17 briefs touch a protected subject: an INVARIANT tier or an IP_DATA, CLAIMS
or SECURITY layer.

**Holders.** WORKING_ITEMS executes a selected brief, through the
chirality-change PR path under an owner-steered production brief. Review is
WORKING_ITEMS (workflow: review). The decisions named in `BlockedOnPacket`
stay with the holder of each packet (see each packet's Decision section).
Where a row's RemainingWork offers to narrow a PROJECT_BASELINE or INVARIANT
requirement, or to "obtain a ruling", the brief lists that alternative. That
alternative belongs to the owner, and no brief takes it. Where a row offers to
narrow a LOCAL_DESIGN requirement (for example DEL-11-05 REQ-11-05-07 in
CFB-03, DEL-07-04 CLM-013 and CLM-022 in CFB-27), the narrowing is a
deliverable-local Scope of Work edit: its holder is WORKING_ITEMS (workflow:
scope-change), and it runs only as a separately authorized R5 record repair
(H4), not under a code-fix brief.

## Briefs by area

### Architecture and governance records

| CFB | Change | Rows | Classes | BlockedOnPacket (rows) | Protected subject |
|---|---|---:|---|---|---|
| [CFB-01](CFB-01_pkg00-architecture-records.md) | PKG-00 architecture-basis records: layer responsibilities, transaction boundaries, job control, migration status | 12 | T6-C02 12 | — | no |
| [CFB-02](CFB-02_del0101-issued-residue.md) | DEL-01-01 ISSUED residue: MAINTAINERS.md authority pointer and closeout protected-content statement | 3 | T4A-C07 1, T6-C01 2 | A6 (3) | no |
| [CFB-03](CFB-03_contributor-governance-docs.md) | Contributor guide dead link and governance tooling residue | 4 | T4A-C07 2, T6-C01 2 | — | no |

### Domain schemas, persistence and hashing

| CFB | Change | Rows | Classes | BlockedOnPacket (rows) | Protected subject |
|---|---|---:|---|---|---|
| [CFB-04](CFB-04_domain-units-schema-binding.md) | Load and unit schema binding to the DEC-018 units catalog | 4 | T6-C01 3, T6-C02 1 | B12 (1) | no |
| [CFB-05](CFB-05_analysis-boundary-diagnostic.md) | Analysis-boundary Diagnostic code and acceptance-reference staleness | 3 | T6-C01 1, T6-C02 1, T6-C03 1 | A3 (1), B12 (2), H3[T6-C03] (1) | yes |
| [CFB-06](CFB-06_plugin-manifest-fixture.md) | Plugin manifest fixture conformance and manifest hashing tests | 5 | T6-C01 2, T6-C03 2, T7-C06 1 | B10 (2), H3[T6-C03] (2), H3[T7-C06] (1) | yes |
| [CFB-07](CFB-07_jcs-canonicalization.md) | Route Python persistence and model-state hashes through the project JCS profile | 12 | T6-C02 12 | A1 (1), A2 (12), B8 (3) | no |
| [CFB-08](CFB-08_project-package-roundtrip.md) | Project package: populated rule-pack reference round trip, explicit migrate, compatibility window | 8 | T6-C01 6, T6-C02 2 | B12 (1) | no |
| [CFB-09](CFB-09_material-component-library-records.md) | Material and library persistence round trip; component fixture coverage | 4 | T6-C01 1, T6-C02 3 | C6 (2) | no |

### Physics inputs, solver and diagnostics

| CFB | Change | Rows | Classes | BlockedOnPacket (rows) | Protected subject |
|---|---|---:|---|---|---|
| [CFB-10](CFB-10_physics-input-unit-provenance.md) | Component and product-physics inputs: unit-against-dimension and redistribution checks | 4 | T6-C03 4 | C1 (2), H3[T6-C03] (4) | yes |
| [CFB-11](CFB-11_import-gate-bare-numerics.md) | Import gate: reject bare numeric values for unit-bearing fields | 3 | T7-C05 3 | C1 (3) | yes |
| [CFB-12](CFB-12_solver-mechanics-verification.md) | Mechanics solver verification: restore envelope-binding tests and add boundary tests | 6 | T6-C01 2, T6-C02 1, T6-C03 2, T7-C06 1 | C2 (1), H3[T6-C03] (2), H3[T7-C06] (1) | yes |
| [CFB-13](CFB-13_producer-diagnostic-envelope.md) | AB-00-06 diagnostic fields on producer diagnostics | 11 | T6-C01 2, T6-C02 9 | — | no |
| [CFB-14](CFB-14_command-result-envelopes.md) | Storage, rule-pack and library commands: diagnostics envelope conformance | 4 | T6-C02 4 | B10 (1), C1 (4) | no |

### Rule engine, manifests and reporting

| CFB | Change | Rows | Classes | BlockedOnPacket (rows) | Protected subject |
|---|---|---:|---|---|---|
| [CFB-15](CFB-15_rule-evaluator-safety.md) | Rule evaluator and completeness checker: protected-content, bypass and bounds tests | 9 | T6-C01 1, T6-C03 8 | B10 (3), C1 (1), H3[T6-C03] (8) | yes |
| [CFB-16](CFB-16_manifest-asset-hashing.md) | Non-JSON asset manifest hashing and asset-entry fields | 5 | T6-C01 5 | — | no |
| [CFB-17](CFB-17_rule-pack-library-refs.md) | Populate rule-pack and library references through the product report, export and analysis-run path | 29 + 1 item | T6-C01 7, T6-C02 22 | — | no |
| [CFB-18](CFB-18_version-stamps.md) | Software/application and solver version stamps in manifests and rendered reports | 7 | T6-C02 7 | — | no |
| [CFB-19](CFB-19_report-rendering.md) | Report rendering: diagnostic class, provenance notes, assumption source and warning fixtures | 9 | T6-C01 5, T6-C02 4 | — | no |
| [CFB-20](CFB-20_result-export-writer.md) | Result export and parity: fixture protected-content scan, unit-conversion witnesses, parity refresh | 4 | T6-C01 3, T6-C03 1 | H3[T6-C03] (1) | yes |
| [CFB-21](CFB-21_linter-claim-phrases.md) | Protected-content linter: product-neutral prohibited-claim phrases | 3 | T7-C06 3 | H3[T7-C06] (3) | yes |
| [CFB-22](CFB-22_linter-detection-breadth.md) | Protected-content linter: detection beyond synthetic markers; CI and privacy boundary | 8 | T6-C01 3, T6-C02 1, T6-C03 4 | B10 (1), H3[T6-C03] (4) | yes |

### Benchmarks

| CFB | Change | Rows | Classes | BlockedOnPacket (rows) | Protected subject |
|---|---|---:|---|---|---|
| [CFB-23](CFB-23_benchmark-runner-records.md) | Benchmark and regression runner records; fixture provenance index; GUI gate evidence | 10 | T6-C01 10 | — | no |
| [CFB-24](CFB-24_benchmark-unit-basis.md) | Benchmark quantities bound to the DEC-018 project unit basis | 6 | T6-C02 6 | — | no |

### Adapters and desktop GUI

| CFB | Change | Rows | Classes | BlockedOnPacket (rows) | Protected subject |
|---|---|---:|---|---|---|
| [CFB-25](CFB-25_adapter-framework.md) | Adapter framework: envelope rule-pack references, invalid-input encoding, live-path conditions | 12 | T6-C01 12 | B10 (7), B7 (2) | no |
| [CFB-26](CFB-26_desktop-panel-fixed-values.md) | Desktop panels showing fixed packet values | 3 | T7-C06 3 | H3[T7-C06] (3) | no |
| [CFB-27](CFB-27_gui-editor-viewer-tests.md) | Model tree and viewer: rule-check negative tests, IP-boundary warning tests, deferred categories | 5 | T6-C01 5 | — | no |

### User documentation

| CFB | Change | Rows | Classes | BlockedOnPacket (rows) | Protected subject |
|---|---|---:|---|---|---|
| [CFB-28](CFB-28_user-docs.md) | User guide, theory note and examples: missing slots and non-claim notices | 8 | T6-C01 7, T6-C03 1 | C4 (2), H3[T6-C03] (1) | yes |

### Local-first storage, privacy and telemetry

| CFB | Change | Rows | Classes | BlockedOnPacket (rows) | Protected subject |
|---|---|---:|---|---|---|
| [CFB-29](CFB-29_local-store-export-redistribution.md) | Local-first store and export: provenance survival and redistribution metadata in export decisions | 5 | T6-C01 3, T6-C03 2 | B10 (1), ER-20 (1), H3[T6-C03] (2) | yes |
| [CFB-30](CFB-30_telemetry-config.md) | Telemetry: product config default-false opt-in and field-class rejection test | 4 | T6-C01 3, T6-C02 1 | — | yes |
| [CFB-31](CFB-31_private-library-guard.md) | Private-library guard integration into product export, report and share seams | 12 | T6-C01 6, T6-C02 3, T6-C03 3 | B10 (2), B12 (2), H3[T6-C03] (3) | yes |

### Constraints, model state and comparison

| CFB | Change | Rows | Classes | BlockedOnPacket (rows) | Protected subject |
|---|---|---:|---|---|---|
| [CFB-32](CFB-32_constraint-knowledge-unit-schema.md) | Constraint and design-knowledge schemas: bind Parameter.value to value_kind quantity; add force_per_length | 10 | T7-C06 10 | C7 (10), H3[T7-C06] (10) | yes |
| [CFB-33](CFB-33_constraint-evaluation.md) | Constraint conflict evaluation and product caller for the validation engine | 2 | T6-C01 1, T6-C02 1 | A1 (2), B9 (2) | no |
| [CFB-34](CFB-34_model-state-save-compare.md) | Model-state product save/list/open and comparison through the application-service boundary | 6 | NOT_DIVERGENT 1, T6-C01 1, T6-C02 4 | A1 (3), B8 (6), C7 (1) | no |
| [CFB-35](CFB-35_model-state-privacy-screening.md) | Model-state external references: privacy and protected-content screening | 2 | T6-C03 2 | H3[T6-C03] (2) | yes |
| [CFB-36](CFB-36_analysis-run-completeness.md) | Analysis-run records: producer dimensions, diagnostic breadth, versioned result contract | 5 | NOT_DIVERGENT 1, T6-C02 4 | C7 (1) | no |
| [CFB-37](CFB-37_comparison-mapping.md) | Comparison mapping: non-element object refs and catalog-based normalization | 4 | T6-C01 4 | — | no |
| [CFB-38](CFB-38_comparison-exporters.md) | Comparison CSV/JSON exporters for the comparison_review contracts | 6 | T6-C01 6 | — | no |
| [CFB-39](CFB-39_report-section-export.md) | Report-section export references bound through the reporting owner | 6 | T6-C01 6 | — | no |

### Handoff and external prover

| CFB | Change | Rows | Classes | BlockedOnPacket (rows) | Protected subject |
|---|---|---:|---|---|---|
| [CFB-40](CFB-40_target-mapping-defaults.md) | Target-mapping contract: remove silent defaults, carry caller provenance, check unit dimensions | 17 | T6-C01 1, T6-C03 3, T7-C06 13 | B7 (17), C6 (3), H3[T6-C03] (3), H3[T7-C06] (13) | yes |
| [CFB-41](CFB-41_handoff-export-path.md) | Product export path emitting schema-valid handoff packages | 9 | T6-C02 9 | B7 (9) | no |
| [CFB-42](CFB-42_external-prover-metadata.md) | External-prover metadata from the product surface; comparison link kind | 5 | T6-C01 4, T6-C02 1 | B7 (2) | no |

### Runtime model operations

| CFB | Change | Rows | Classes | BlockedOnPacket (rows) | Protected subject |
|---|---|---:|---|---|---|
| [CFB-43](CFB-43_runtime-constraint-stage.md) | Constraint-validation stage on the runtime operation route | 3 | T6-C02 3 | A1 (3), B3 (3), B9 (3) | no |
| [CFB-44](CFB-44_decision-history-contract.md) | Durable hash-bound decision/history contract; touching-path check preservation | 6 | T6-C02 5, T6-C03 1 | H3[T6-C03] (1) | yes |

### Export formats

| CFB | Change | Rows | Classes | BlockedOnPacket (rows) | Protected subject |
|---|---|---:|---|---|---|
| [CFB-45](CFB-45_export-profile-fields.md) | Export profiles: align to the common field list and ID-map carrier values | 12 | T6-C01 12 | C7 (2) | no |
| [CFB-46](CFB-46_native-json-token.md) | Native JSON loss category token: TBD vs tbd | 4 + 1 item | T7-C06 4 | H3[T7-C06] (4) | no |
| [CFB-47](CFB-47_mbf-loss-diagnostic.md) | MBF export: make MBF-LOSS-REPORT-MISSING reachable; sidecar identity coverage | 3 + 1 item | T7-C06 3 | H3[T7-C06] (3) | no |
| [CFB-48](CFB-48_mbf-pass-through.md) | MBF profile pass-through options and open questions | 4 | T6-C01 4 | — | no |
| [CFB-49](CFB-49_caepipe-harness-records.md) | CAEPIPE harness run-record fields (non-live parts) | 18 | T6-C01 18 | — | no |
| [CFB-50](CFB-50_stress-neutral-records.md) | Stress-neutral CSV: contract location, traceability table, owning workflows | 3 | T6-C01 3 | — | no |
| [CFB-51](CFB-51_pcf-silent-fallbacks.md) | PCF export silent fallbacks: zero coordinates, millimetre assumption, missing disclosure member | 4 + 3 items | T7-C06 4 | H3[T7-C06] (4) | no |
| [CFB-52](CFB-52_pcf-family-coverage.md) | PCF profile: per-family classification and component mappings beyond straight pipe | 11 | T6-C01 11 | A7 (4), C5 (1), C7 (1) | no |
| [CFB-53](CFB-53_gltf-profile.md) | glTF review-geometry profile: GLB or JSON profile, per-family coverage, transform and consumer-risk policy | 10 | T6-C01 10 | — | no |
| [CFB-54](CFB-54_plugin-sdk-admission.md) | Plugin SDK admission contract: external-execution policy, registry states, checklist categories | 8 | T6-C01 8 | B10 (2), C7 (1) | no |

## Rows with two views (T8 against class route)

Thirteen rows in `R3/T8_ROUTE_DISAGREEMENTS.csv` touch this route. Both views
are shown. H2 does not choose between them.

| Rows | T8 route | Class route | Held in | Brief |
|---|---|---|---|---|
| `DEL-14-01:SOW#CLM-005`; `DEL-14-03:SOW#CLM-004`, `SOW#CLM-006`, `CONTEXT#architecture-basis-injection.s03` (DEC-009 cluster) | OWNER_DECISION | CODE_FIX_CANDIDATE (T6-C02) | A1 | CFB-07, CFB-34 |
| `DEL-03-07:SOW#CLM-003.r06`, `SOW#CLM-009.r05`, `SOW#CLM-021.s02` (unit vocabulary) | CODE_FIX_CANDIDATE (REVIEW) | OWNER_DECISION (T7-C05) | C1 | CFB-11 |
| `DEL-14-01:SOW#CLM-024`, `DEL-14-02:SOW#CLM-024` (F1 on CONTEXT) | CODE_FIX_CANDIDATE | not divergent | C7 | CFB-34, CFB-36 |
| `DEL-17-02:SOW#CLM-019/DEL-17-02-REQ-014`, `SOW#CLM-023/DEL-17-02-REQ-053`, `DEL-17-07:SOW#CLM-016/DEL-17-07-REQ-034`, `DEL-17-09:SOW#CLM-013/DEL-17-09-REQ-010` (tier of in-scope REQs, CP-11) | NO_ACTION | CODE_FIX_CANDIDATE (T6-C01) | C7 | CFB-45, CFB-52, CFB-54 |

T12 readings that differ from the code-fix class route (both views shown in the briefs):

| Rows | T12 route | Class route | Held in | Brief |
|---|---|---|---|---|
| `DEL-14-01:SOW#CLM-004`, `SOW#CLM-005`, `SOW#CLM-011.r04` (T12-C02) | OWNER_DECISION | CODE_FIX_CANDIDATE (T6-C02) | B8 (with A2; CLM-005 also A1) | CFB-07 |
| `DEL-14-01:SOW#CLM-011.r01` (T12-C02) | OWNER_DECISION | CODE_FIX_CANDIDATE (T6-C02) | B8 | CFB-34 |
| `DEL-14-01:SOW#CLM-024` (T12-C02) | OWNER_DECISION | not divergent (T8 K4: CODE_FIX) | B8, C7 | CFB-34 |
| `DEL-13-03:SOW#CLM-005.r04` (T12-C03) | OWNER_DECISION | CODE_FIX_CANDIDATE (T6-C01) | B9 (with A1) | CFB-33 |
| `DEL-13-01:SOW#CLM-005.r05` (T12-C03) | REVIEW | CODE_FIX_CANDIDATE (T7-C06) | ER-22 (H3[T7-C06]), C7 | CFB-32 |

Other open readings are recorded in the briefs:

- **Unit-vocabulary rows (CFB-32).** The 8 DEL-13-02 rows are CONTESTED. T8
  reads them CODE_FIX with REVIEW, and so does the class route. C7 carries the
  tier reading.
- **DEL-15-02 tier-hint rows (CFB-40).** The CLM-005 Units, Missing-values and
  Provenance rows go to C6 (W3 departure 5).
- **DEL-02-05 CONTESTED rows (CFB-07).** Four rows split between FROZEN_CONTRACT
  and NONE; A2 decides.
- **DEL-03-01 CONTESTED rows (CFB-09).** Two rows where evidence is judged both
  ways.
- **DEL-08-05 AC-001 (CFB-22).** CONTESTED.
- **DEL-03-04 RQ-005 (CFB-10).** CONTESTED.
- **DEL-11-04 R-DEL-11-04-002 (CFB-28).** CONTESTED;FIELD; C4 decides.

## W3 product and code items

| Item | W3 statement (summarised) | Brief | Claim rows it maps to |
|---|---|---|---|
| W3-PC-01 | Analysis-run records write empty rule-pack and library references | CFB-17 | DEL-14-02 refs rows, DEL-08-0x and DEL-06-04 refs rows |
| W3-PC-02 | PCF writes 0 for a missing coordinate | CFB-51 | DEL-17-07 CLM-006, REQ-021, CLM-030 |
| W3-PC-03 | Desktop treats non-m section units as millimetres (worker-raised; not verifier-confirmed; a freeze code reading of `PcfExportPanel.tsx:701` agrees) | CFB-51 | DEL-17-07 CLM-006, REQ-021, CLM-043 |
| W3-PC-04 | `unit_system_disclosure.json` missing from PCF packages | CFB-51 | DEL-17-07 CLM-030 |
| W3-PC-05 | MBF loss-report diagnostic cannot fire | CFB-47 | DEL-17-04 REQ-009, CLM-011.r10, CLM-023 |
| W3-PC-06 | Native JSON token `TBD` vs contract `tbd` | CFB-46 | DEL-17-02 CLM-012, REQ-054, CLM-043; DEL-17-03 REQ-004 |

Some W3 "product and code" bullets are not among the six H2 items. They sit
elsewhere:

- **DEL-15-02 silent defaults.** Covered by the T7-C06 rows in CFB-40, which
  is blocked on B7.
- **DEL-15-03 redaction.** C1.
- **JCS labels.** A2. The code side is in CFB-07.
- **Knowledge panel and constraint status.** B9.
- **Handoff schema compliance.** B7. The code side is in CFB-41 and CFB-42.
- **GUI panels against SOW exclusions.** B6.

## Owner decisions first reported UNASSIGNED (now placed in the gate index)

H2's first return reported these as UNASSIGNED. `R4/R4_GATE_INDEX.md` §4 now
places them; the affected rows carry the placement token.

1. **U6, command envelope exemption** (placed with C1). Whether storage,
   rule-pack and library commands are exempt from the diagnostics envelope.
   Rows: `DEL-00-06:AB#normative-requirements/REQ-06-03`,
   `DEL-00-03:AB#normative-requirements/REQ-03-02`, `DEL-06-04:SOW#CLM-004.r05`
   and `SOW#CLM-010/R-06-04-012`. All carry C1. Brief: CFB-14.
2. **U5, narrowing INVARIANT restatements** (placed with C1).
   `DEL-03-08:SOW#CLM-026` and `SOW#CLM-011/DEL-03-08-RQ-004` (CFB-10), and
   `DEL-06-01:SOW#CLM-011/REQ-06-01-011` (CFB-15). All carry C1.
3. **U7, PROJECT_BASELINE "narrow or defer by ruling" rows outside A2, B3, B7
   and B8** (placed with C6, M2). A scan of the unblocked PROJECT_BASELINE,
   INVARIANT and FROZEN_CONTRACT rows leaves exactly two:
   `DEL-03-01:SOW#CLM-011/REQ-03-01-007` and
   `DEL-03-01:SOW#production-and-verification-method-praxeology/VER-001`
   (CFB-09). Both carry C6.
4. **U8, DEL-11-03 deferred theory scopes** (placed with C4). Owner discharge
   of the deferred scopes (`DEL-11-03:SOW#CLM-010.s02`, RF-11-03-C-003 human
   DEFER). The row carries C4. Brief: CFB-28.

## Disagreements left open

- **DEC-009 rows.** T8 route and class route differ (see the two-views table).
- **DEL-03-07.** T8 route and class route differ (see the two-views table).
- **F1 rows.** T8 route and class route differ (see the two-views table).
- **CP-11 rows.** T8 route and class route differ (see the two-views table).
- **T12 rows.** Seven rows where the T12 reading differs from the class route (see the T12 table).
