# R3 T7 — Defects, evidence and authority classes

T7 classifies the 213 divergent rows of `R3/CORPUS_CLAIMS.csv` whose effective CauseTag is POSSIBLE_DEFECT (77), AUTHORITY_UNCLEAR (69), EVIDENCE_NOT_LOCATED (42), VALIDATION_GAP (15), OTHER (9) or VERIFICATION_REMOVED (1). There are 11 classes, and code-fix candidates, engineering-authority items and owner decisions each get their own classes. Five classes need an owner decision, 100 rows in all. T7-C01 (50) covers declared-open holds that the code settled without a ruling (CP-10). T7-C02 (11) covers the JSON hash basis (JCS against sorted-compact). T7-C03 (7) covers the DEC-009 Rust-core baseline against the Python engines. T7-C04 (5) covers questions where the governing sources conflict or say nothing. T7-C05 (27) covers behaviour that contradicts the SOW text but may be intended, so the owner must rule on intent before any code fix. T7-C06 (42) holds the code and test fix candidates that need no ruling. T7-C07 (7) is engineering authority. T7-C08 (10) is held independent validation that the records state accurately. T7-C09 (31) and T7-C10 (14) are review-evidence gaps, split into IP/data boundaries and all other gaps. T7-C11 (9) is record text that was wrong when written. Every class below is a proposal for routing. None of them re-disposes a row, repairs anything or decides anything. This file describes records and evidence only. It makes no claim of certification, code compliance, professional approval or engineering acceptance (F-PIP-2; claims taxonomy per DEC-081).

Evidence conventions: `F:` means `{FREEZE}/projects/chirality-piping/` at `00115c71931bcae79909602d653740d3bb72dfa1`. Ledger keys refer to the sealed forward ledgers under `RUN/WAVES/`. "RC" means the row's `ResolutionClasses`.

## Class table

| Class | Name | Rows | Owning authority | Route |
|---|---|---|---|---|
| T7-C01 | CP-10 holds settled in code without a ruling | 50 | OWNER | OWNER_DECISION |
| T7-C02 | JSON hash-basis departure from AB-00-04 (JCS) | 11 | OWNER | OWNER_DECISION |
| T7-C03 | DEC-009 Rust-core baseline vs Python engines | 7 | OWNER | OWNER_DECISION |
| T7-C04 | Authority silent or in conflict (non-CP-10) | 5 | OWNER | OWNER_DECISION |
| T7-C05 | Behaviour contradicts SOW; owner rules intent before fix | 27 | OWNER | OWNER_DECISION |
| T7-C06 | Code/test fix candidates (no ruling needed) | 42 | REVIEW | CODE_FIX_CANDIDATE |
| T7-C07 | Engineering validation and reference-model authority | 7 | ENGINEERING | ENGINEERING_AUTHORITY |
| T7-C08 | Owner-held independent validation, accurately recorded | 10 | OWNER | NO_ACTION |
| T7-C09 | Protected-content / IP-data review record not located | 31 | REVIEW | REVIEW |
| T7-C10 | Other evidence not located (smallest checks) | 14 | REVIEW | REVIEW |
| T7-C11 | Record text inaccurate at authoring (OTHER) | 9 | NONE | R5_RECORD_REPAIR |

---

## T7-C01 — CP-10 holds settled in code without a ruling

**Description.** A deliverable declares an implementation choice open (TBD, hold, or "human ruling required"). The frozen code has since fixed that choice, and no ruling selecting it was found in the decision register or SOFTWARE_DECOMP §12. The records are consistent with each other. They are behind the code only because nobody has confirmed the choice. The row is not in doubt; what is missing is authority. The choices, in groups:
- **GUI component and state library.** React built-ins, no third-party library. DEL-00-05, DEL-07-01, DEL-07-02, DEL-07-03, DEL-07-05, DEL-07-08: 15 rows.
- **Private-data root.** The Tauri app-local data directory, `F:apps/desktop/src-tauri/src/lib.rs:548` `app_store_path`. DEL-12-01, DEL-12-02 (CLM-037): 8 rows.
- **Schema file layout and QuantityKind classification.** DEL-02-01, DEL-02-02 (non-frozen rows).
- **Split of the status axes.** DEL-02-03.
- **Redistribution and review vocabulary.** DEL-03-07, DEL-06-04.
- **Exact rule comparisons with no tolerance.** DEL-06-02.
- **Benchmark fixture schema.** DEL-09-01.
- **API and handoff schema placement and vocabulary.** DEL-10-01, DEL-10-03.
- **Unmatched-classification enum.** DEL-14-05.
- **Other items.** The package manager (DEL-00-02), the severity taxonomy (DEL-00-06), the undo mechanism (DEL-00-05) and REXC-CON-002 explicit local-private intent (DEL-12-02).

**Signature.** AUTHORITY_UNCLEAR · IMPLEMENTED_DIFFERENTLY · PROJECT_BASELINE · BaselineClass NONE · layer RECORD. CanonicalSituation CP-10 on 49 rows.

**Population.** 50 rows.
- **Packages (9).** PKG-00, 02, 03, 06, 07, 09, 10, 12, 14.
- **Deliverables (20).** DEL-00-02, 00-05, 00-06, 02-01, 02-02, 02-03, 03-07, 06-02, 06-04, 07-01, 07-02, 07-03, 07-05, 07-08, 09-01, 10-01, 10-03, 12-01, 12-02, 14-05.

**Owning authority.** OWNER.

**Route.** OWNER_DECISION.

**Decision required.** For each hold topic, the owner rules on one question: does the implemented choice close the hold? The options, as the evidence states them:
- (a) Confirm the implemented choice and record it as a ruling or as an accepted implementation-level choice. The Remaining text on DEL-00-02 .s03 offers this option explicitly.
- (b) Rule a different choice, which makes the code the thing to catch up.
- (c) Keep the hold explicitly open, with the code marked provisional.

Some holds are only partly settled, and those parts stay open whatever the owner rules:
- code-generation tooling (DEL-02-01, DEL-02-02);
- the diagnostic code namespace (DEL-00-06);
- lint tooling (DEL-00-02);
- the transport, FEA format and adapter (DEL-10-03).

About 12 topic-level rulings would close all 50 rows. The GUI library ruling alone closes 15.

**On-ruling mechanism.** The owner records the ruling in the decision register and SOFTWARE_DECOMP §12, for example as a DEC entry. The ruling authorises R5_RECORD_REPAIR of the listed SOW, AB and CONTEXT rows, so they cite it and drop the TBD. It also authorises catching up the matching governance text: SPEC §4.4 for the private-data root, and the DEL-00-05 ArchitectureBasis hold. Under option (b), a CODE_FIX_CANDIDATE brief follows.

**Risk if unrepaired.** Implementation choices that were never ruled become de facto baseline, and the deliverables keep declaring holds that the product no longer honours. Later work may treat the code as authorised. It may also reopen choices already built into many surfaces: 15 rows depend on the GUI library choice alone.

**Representative keys.**
- `DEL-00-05:AB#open-holds-and-routed-questions.s01`. The GUI state library is declared OPEN under DEC-009 §8.2. The code has no third-party state or component library, and state lives in React `useState` hooks (ledger Notes).
- `DEL-12-01:SOW#CLM-033/LFSP-OI-002`. The OS private-data roots are declared TBD. `F:apps/desktop/src-tauri/src/lib.rs:548` resolves the store from the app-local data directory.
- `DEL-06-02:SOW#CLM-015.r04`. Comparison tolerances are declared TBD. The evaluator compares exactly, and the DEC-022 corpus pins that behaviour. No ruling on comparison tolerance was found.

**Exceptions kept visible.**
- `DEL-03-07:SOW#CLM-026.r01` is CONTESTED. The resolution says the row should cite `docs/IP_AND_DATA_BOUNDARY.md` §4 (a draft), which defines most of the vocabulary. Whether a draft policy counts as governing decides both the CP-10 treatment and the owner routing.
- `DEL-07-01:SOW#CLM-005.r05` is FIRM. The effective AuthorityNeeded is NO. The FIRM correction says CP-10 and OWNER, and to share the CP-10 finding. It is classed here on that basis.

---

## T7-C02 — JSON hash-basis departure from AB-00-04 (JCS)

**Description.** AB-00-04, DEC-010 and DEC-017 set the JSON payload hash basis: canonical JSON with JCS-compatible (RFC 8785) canonicalisation. The paths below depart from that basis in three ways.
- **Python persistence.** It hashes sorted-key compact ASCII JSON labelled SORTED_COMPACT_JSON, and says it is not JCS. See `F:core/project_persistence/service.py`.
- **The audit_manifest and report package.** They hash and label with "project_local_deterministic_json", and the wire format accepts no RFC 8785 label. See `F:core/reporting/report_package/src/lib.rs:464` and `wire.rs:251`. This holds even where the product computed the hash with RFC 8785 (DEL-08-02).
- **The exporters.** They hash with `json.dumps(sort_keys, compact)` (DEL-17-02 REQ-007).

Meanwhile the desktop Rust path and the canonical_json crate use RFC 8785. The D-41 R5 T2A relabelling is a record. It is not a baseline amendment.

**Signature.** POSSIBLE_DEFECT or AUTHORITY_UNCLEAR · IMPLEMENTED_DIFFERENTLY · PROJECT_BASELINE (10). One row is AUTHORITY_UNCLEAR · UNKNOWN · LOCAL_DESIGN. BaselineClass is FROZEN_CONTRACT on the 5 DEL-02-02 rows and NONE on the other 6. Layers BASELINE;RECORD.

**Population.** 11 rows.
- **Packages.** PKG-02, 08, 17.
- **Deliverables.** DEL-02-02 (5), DEL-08-02 (4), DEL-17-02 (1), DEL-17-09 (1).

**Owning authority.** OWNER.

**Route.** OWNER_DECISION.

**Decision required.** The owner chooses among three options:
- (a) Move the Python persistence, audit-manifest and exporter hashing, and their labels, to the RFC 8785 basis. This is a code change.
- (b) Amend AB-00-04 / DEC-010 / DEC-017 to permit the sorted-compact basis on the named paths, with accurate labels.
- (c) A split: keep the persistence basis, but make the product audit-manifest labels accurate. The DEL-08-02 rows ask for this as an "R4 code-change candidate".

**On-ruling mechanism.** Under (a) or (c), the ruling authorises a CODE_FIX_CANDIDATE brief covering four surfaces:
- the audit_manifest Canonicalization enum and the report_package wire labels;
- the exporter hash functions;
- the persistence service;
- the DEL-17-04 MBF label noted below.

The DEL-02-02 and DEL-08-02 SOW text is then repaired under R5. Under (b), the baseline amendment goes through the architecture-decision path, because a FROZEN_CONTRACT change is an owner baseline event, and R5_RECORD_REPAIR follows.

**Risk if unrepaired.** Hash records carry labels that do not describe the bytes. In one case the label is "project_local" and the hash is actually RFC 8785. Cross-surface hash comparisons can then fail or mislead, and a frozen contract stays breached with nothing recorded.

**Representative keys.**
- `DEL-02-02:SOW#CLM-014/U-008`. This is FROZEN_CONTRACT: the Python persistence is not JCS.
- `DEL-08-02:SOW#CLM-004.r02`. The product computes RFC 8785 hashes, then labels them project_local_deterministic_json (`F:core/reporting/report_package/src/wire.rs:251`).
- `DEL-17-02:SOW#CLM-018/DEL-17-02-REQ-007`. The exporters hash with sorted-compact. DEC-028 binds the native package to REQ-007.

**Note.** `DEL-17-09:SOW#CLM-018` is AUTHORITY_UNCLEAR · UNKNOWN · LOCAL_DESIGN with AuthorityNeeded NO. It asks whether "all DEL-17-09-produced JSON checksum records" includes the desktop SDK preview hash, which is labelled rfc8785_jcs. It is placed here because the hash-basis ruling settles which label is correct. The row is not itself an owner item. There are no contested, field or observed rows in this class.

---

## T7-C03 — DEC-009 Rust-core baseline vs Python engines

**Description.** DEC-009 (SOFTWARE_DECOMP §12) adopts Rust core/application services as the implementation baseline. Four deliverables implement their engine or contract builder only as Python modules under `core/`:
- DEL-13-03: the validator;
- DEL-13-04: the transform and adapter;
- DEL-14-04: the comparison engine;
- DEL-15-02: `F:core/handoff/target_mapping/`.

No ruling was found that permits Python core slices. DEC-025 registers pytest as a gate surface, which is not the same thing. A merged PR cannot amend a PROJECT_BASELINE (A2). The rows differ on cause: 4 are POSSIBLE_DEFECT, 2 are AUTHORITY_UNCLEAR IMPLEMENTED_DIFFERENTLY, and 1 is AUTHORITY_UNCLEAR UNKNOWN. That split is exactly the contested point.

**Signature.** POSSIBLE_DEFECT or AUTHORITY_UNCLEAR · IMPLEMENTED_DIFFERENTLY or UNKNOWN · PROJECT_BASELINE · BaselineClass NONE · layers BASELINE or RECORD;BASELINE.

**Population.** 7 rows, all CONTESTED.
- **Packages.** PKG-13, 14, 15.
- **Deliverables.** DEL-13-03 (2), DEL-13-04 (2), DEL-14-04 (2), DEL-15-02 (1).

**Owning authority.** OWNER.

**Route.** OWNER_DECISION.

**Decision required.** One project-wide answer. The owner chooses among:
- (a) Python domain engines and contract builders under `core/` are permitted within the DEC-009 baseline, for example as a reference or contract layer. Record that permission.
- (b) The named engines must be ported to Rust.
- (c) Permit them per deliverable, case by case.

The R2 notes on DEL-15-02 and DEL-14-04 both state that one project-wide answer settles these rows.

**On-ruling mechanism.** Under (a) or (c), the owner records an architecture-decision amendment to DEC-009 / SOFTWARE_DECOMP §12, then the CONTEXT and SOW architecture-basis rows get R5_RECORD_REPAIR. Under (b), the ruling authorises CODE_FIX_CANDIDATE port briefs, sized per engine. Either way, this class overlaps T8's DEC-009 cluster, and T8's reading shapes the R4 packet.

**Risk if unrepaired.** The baseline stays breached across several packages with no record of it. It also stays unclear whether new core work may be written in Python, and the answer affects the product call paths too: DEL-15-02 is also PRODUCT_CALLER NONE elsewhere.

**Representative keys.**
- `DEL-13-03:SOW#CLM-004.r05`. The engine is a stdlib-only Python module (POSSIBLE_DEFECT).
- `DEL-14-04:SOW#CLM-004.r07`. The engine is Python, verified by pytest (AUTHORITY_UNCLEAR).
- `DEL-15-02:CONTEXT#architecture-basis-injection.s03`. Python only, with no Rust counterpart (UNKNOWN).

**Contested rows (all 7).** DEL-13-03 CLM-004.r05 and CONTEXT .s03; DEL-13-04 CLM-014.s01 and CONTEXT .s03; DEL-14-04 CLM-004.r07 and CONTEXT .s02; DEL-15-02 CONTEXT .s03. The resolution text on these rows reads "DEC-009 cluster: cause/tier/routing split; Agent 0 settles corpus-wide", or "cause POSSIBLE_DEFECT vs AUTHORITY_UNCLEAR".

---

## T7-C04 — Authority silent or in conflict (non-CP-10)

**Description.** These are owner questions where the governing sources disagree or say nothing, and where no code has settled the matter. That is what separates them from CP-10.

**Signature.** AUTHORITY_UNCLEAR · UNKNOWN (4) or AUTHORITY_CONFLICT (1) · PROJECT_BASELINE (3) or LOCAL_DESIGN (2) · BaselineClass NONE, except ISSUED on the DEL-01-01 row.

**Population.** 5 rows.
- **Packages.** PKG-01, 07, 12.
- **Deliverables.** DEL-01-01, DEL-01-03, DEL-07-09, DEL-12-04 (2).

**Owning authority.** OWNER.

**Route.** OWNER_DECISION. The palette-landing question could instead go to SCOPE_CHANGE_HANDOFF, if the owner treats it as reassigning ownership.

**Decisions required. There are four.**
1. **Product posture wording**, `DEL-01-01:SOW#CLM-009.s01`, which is ISSUED. PRD v0.4 L24 says "free and open-source". ScopeLedger SOW-001, DIRECTIVE L73 and the selected PolyForm Noncommercial licence say "source-available noncommercial". The owner picks one and aligns the PRD with the governance surfaces.
2. **Pre-release legal review**, `DEL-01-03:STATUS#remaining/R02`. It was required by PRD v0.1 §17.5, which PRD v0.4 did not carry forward. The owner either confirms that the review is still required, with its basis, or retires it.
3. **Palette code landing**, `DEL-07-09:PALETTE_ORGANIZATION_CONTRACT#organization-and-ownership`. DEC-094 makes DEL-07-09 the owner of the palette surface. But no annex row says where `ToolkitPalette.tsx` and `capabilityCatalog.ts` land, and `_CONTEXT` says DEL-07-09 never receives implementation. The options are DEL-07-09, DEL-07-01 or DEL-07-02. T1–T3 capability ownership may bear on this.
4. **Secret provider and encrypted storage**, and the remaining CF-002 deferrals: `DEL-12-04:SOW#CLM-028/DEL-12-04-CF-001` and `CF-002`. The owner either rules or confirms that they stay TBD. One element of CF-002, the package/container, is stale, because SCA-003 settled it.

**On-ruling mechanism.** Each ruling is recorded in the register or the PRD, and the rows then get R5_RECORD_REPAIR. For the posture ruling, a PRD revision path applies, and the ISSUED DEL-01-01 block needs re-issue under whatever change path is authorised. Decision 3 may instead go through scope-change as an ownership assignment.

**Risk if unrepaired.** The public posture and the licence wording contradict each other in issued text and in the project README. A release-time legal gate stays ambiguous. Palette code has no owning deliverable. And the secret-handling defaults stay undefined.

**Representative keys.** DEL-01-01 CLM-009.s01, DEL-01-03 R02 and DEL-07-09 organization-and-ownership, as above.

**Exceptions.** None. There are no contested, field or observed rows.

---

## T7-C05 — Behaviour contradicts SOW; owner rules intent before any fix

**Description.** In each case, the frozen code departs from an explicit SOW requirement. The effective AuthorityNeeded is OWNER on 25 of the 27 rows, because the departure may be intentional, or may need a baseline or ruled-criterion change. So a code fix waits on an owner ruling about intent. Six subjects:
1. **DEL-15-03, 11 rows. Export redaction.** Since PR #307 (2026-07-22), the exporter routes output through `control_route_export` with route REXC-CORE-001 (`F:core/handoff/exporter/workflow.py:131`). Much of the content comes back as `[REDACTED]`, even for invented public fixtures:
   - model_hash;
   - units-manifest values;
   - entity IDs, library refs and rule refs;
   - assumption IDs and warning codes.

   The SOW requires that content to be preserved.
2. **DEL-08-03 (6) and DEL-08-04 (2). Diagnostic class and remediation.** The desktop adapter derives each diagnostic's class from its severity and substitutes a fixed remediation string, instead of carrying the producer's value or marking it TBD. The functions are `diagnosticsForSections` and `diagnosticClass`, and `buildCurrentResultExport` fixes every class to ASSUMPTION_WARNING.
3. **DEL-00-06 REQ-06-02 (1). affected_object not preserved.** The solve-to-export crossing replaces `affected_refs` with a reference to the diagnostic itself (`F:core/runner/headless/src/result_envelope_binding.rs:168`). The requirement says every field except class is kept.
4. **DEL-03-07 (3). Bare numbers at the import gate.** The unit gate inspects only `magnitude`-shaped values, so a bare number in a material, section or component record can be accepted (OPS-K-UNIT-1).
5. **DEL-13-01 REQ-13-01-011 (1). Preview design-knowledge data.** The desktop preview path consumes design-knowledge data that is not shaped to the schema and is not validated against it. The question is whether to accept this as a preview-only exception.
6. **DEL-09-01 (3). A loosened fixture tolerance.** The MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL fixture uses a fixture-local relative tolerance of 5.0e-7, where the DEC-026 analytic seed is 1e-9. DEC-026 treats loosening as a governance event.

**Signature.** POSSIBLE_DEFECT · IMPLEMENTED_DIFFERENTLY (22) or PARTIALLY_IMPLEMENTED (5) · tiers: PROJECT_BASELINE 16, INVARIANT 10, LOCAL_DESIGN 1 · BaselineClass NONE, except RULED_CRITERION on DEL-09-01 (3) · layers BASELINE, BASELINE;VALIDATION or BASELINE;RECORD.

**Population.** 27 rows.
- **Packages.** PKG-00, 03, 08, 09, 13, 15.
- **Deliverables.** DEL-00-06, DEL-03-07, DEL-08-03, DEL-08-04, DEL-09-01, DEL-13-01, DEL-15-03.

**Owning authority.** OWNER.

**Route.** OWNER_DECISION.

**Decisions required, with the options in the evidence.**
1. **Export redaction (DEL-15-03).** Is downstream_tool redaction of invented or public handoff content intended?
   - If yes, amend the SOW (REQ-002/004/005, AC-001) through the change path.
   - If no, fix the export route classification. That is a code fix.
2. **Diagnostic class and remediation (DEL-08-03, DEL-08-04, DEL-00-06).** Must producer values be carried through, with TBD where the producer has none? Or is adapter derivation accepted, with the requirement refined?
3. **Bare numbers (DEL-03-07).** Must the import gate reject bare numeric values, or schema-validate those record kinds?
4. **Preview path (DEL-13-01).** Must it carry schema-valid records, or is it an accepted exception?
5. **Tolerance (DEL-09-01).** Adopt 5.0e-7 into the governed DEC-026 record with its reason, or require the witness configuration that meets 1e-9.

**On-ruling mechanism.** The owner ruling goes to the register, or to the DEC-026 record for decision 5. Depending on the answer, it authorises either a CODE_FIX_CANDIDATE brief (exporter route, desktop diagnostic adapter, runner binding, import gate) or an SOW amendment through the scope-change or change path, followed by R5_RECORD_REPAIR.

**Risk if unrepaired.** Export packages drop hashes and units that the handoff contract promises. Diagnostics carry classes and remediation text that did not come from the producer, which contradicts the "do not invent" principles. Unitless numbers can pass the import gate (INVARIANT). And a protected-check tolerance is loosened without the governance event that DEC-026 requires.

**Representative keys.**
- `DEL-15-03:SOW#CLM-011/DEL-15-03-REQ-004`. Unit metadata is redacted in the export payload (INVARIANT).
- `DEL-08-03:SOW#CLM-012/DEL-08-03-REQ-002`. Class is derived from severity, and no class column is rendered.
- `DEL-09-01:SOW#CLM-024.r05`. The fixture-local 5.0e-7 against the DEC-026 value of 1e-9.

**Exceptions kept visible.**
- **FIELD**, sealed RULED_CRITERION: `DEL-09-01:SOW#CLM-015.r03`, `CLM-023` and `CLM-024.r05`. The field resolution says BaselineClass should be PROTECTED_CHECK. That correction is not applied, but it makes these rows 100%-sampled.
- **FIELD**, AuthorityTier PROJECT_BASELINE to INVARIANT: `DEL-15-03:SOW#CLM-006.r04`, `CLM-011/REQ-002`, `CLM-019` and `AC-001`.
- **FIELD**, VerificationEvidence correction: `DEL-13-01:SOW#CLM-009/REQ-13-01-011`.
- **OBSERVED**: `DEL-03-07:SOW#CLM-021.s02`. The cause is POSSIBLE_DEFECT on the rerun and PARTIAL_SLICE for the first verifier.
- **AuthorityNeeded REVIEW**: the two DEL-08-04 rows. They are placed here because they share their substance with DEL-08-03, which is AuthorityNeeded OWNER (see the R3 observations).

---

## T7-C06 — Code/test fix candidates (no ruling needed)

**Description.** In each case, the code or a test departs from an unambiguous requirement, and the ledger's RemainingWork already names a bounded repair that needs no owner or engineering ruling. The effective AuthorityNeeded is NO on 26 rows and REVIEW on 16. Subjects:
- **DEL-15-02 (13). Silent defaults in the target-mapping contract.** `build_target_mapping_contract` fills in absent record inputs without reporting them:
  - it defaults mapping_status to "mapped" (`F:core/handoff/target_mapping/contract.py:239`);
  - it substitutes the engine's own provenance;
  - it defaults absent refs.

  In addition, its closed Reference objects cannot carry reference-level provenance status (FG-01, FG-03). All 13 rows are PRODUCT_CALLER NONE: the function is imported only by tests.
- **DEL-13-02 (8, CONTESTED) and DEL-13-01 (2). Unit rules in the constraint and design-knowledge schemas.**
  - `Parameter.value` is not tied to `value_kind` quantity.
  - `Quantity.dimension` omits the accepted `force_per_length`.
  - The test compares against a local copy of the vocabulary instead of the canonical one.
- **DEL-08-05 (3). Claim phrases keyed on the former product name.** The prohibited-claim phrases match only claims that name the former product, for example `F:core/reporting/protected_content_linter/src/lib.rs:639`. After the SCA-010 rename, claims naming the current product are not matched.
- **DEL-17-02 (3) and DEL-17-03 (1). A loss-category token mismatch.** The native JSON loss category is `TBD` in `F:core/handoff/native_json/package.py:26`, while the contract token is `tbd`. Either side's text could be the one that catches up.
- **DEL-17-04 (3). An unreachable diagnostic.** `MBF-LOSS-REPORT-MISSING` cannot fire, because an empty loss report is replaced by a default entry.
- **DEL-17-07 (4). PCF silent fallbacks.** An absent node coordinate is written as 0. A non-m section unit is treated as mm. And `unit_system_disclosure.json` is not written.
- **DEL-07-08 (2). A fixed evidence block.** DesignWorkspacePanel shows a fixed `core_contract_evidence` count block (`F:apps/desktop/src/features/design-workspace/DesignWorkspacePanel.tsx:204`).
- **DEL-10-04 (1). A hard-coded field.** The BuildReadinessPanel packet hard-codes `bundle_active: false` (`BuildReadinessPanel.tsx:116`), while `tauri.conf.json:27` has `"active": true`.
- **DEL-02-04 (1). A non-conforming fixture.** The published invented plugin manifest fixture does not conform to its canonical schema.
- **DEL-04-04 (1). A removed protected check.** Envelope-binding tests were removed by b43cc00c4 (PR #787). This is a test to restore.

**Signature.** POSSIBLE_DEFECT · PARTIALLY_IMPLEMENTED (33) or IMPLEMENTED_DIFFERENTLY (8) · tiers: INVARIANT 25, LOCAL_DESIGN 14, PROJECT_BASELINE 3. One row is VERIFICATION_REMOVED · PARTIALLY_IMPLEMENTED · INVARIANT · PROTECTED_CHECK.

**Population.** 42 rows.
- **Packages (8).** PKG-02, 04, 07, 08, 10, 13, 15, 17.
- **Deliverables (12).** DEL-02-04, 04-04, 07-08, 08-05, 10-04, 13-01, 13-02, 15-02, 17-02, 17-03, 17-04, 17-07.

**Owning authority.** REVIEW. The candidate briefs go through review. No ruling is needed.

**Route.** CODE_FIX_CANDIDATE.

**On-ruling mechanism.** An R4 acceptance of the class authorises WORKING_ITEMS to prepare one bounded code or test brief per subject. There are about 10 briefs, each with the negative test named in the ledger RemainingWork. Every brief goes through the chirality-change PR path, with independent review. Once a fix lands, the affected ledger rows are re-verified in a later concordance. They are not edited in place. Two subjects need a choice at brief time:
- **DEL-17-02/03.** Align the code to `tbd`, or record the variance in the DEL-17-02 contract. That is a record repair instead.
- **DEL-13-02.** The contested cluster must be settled first (T8 unit-vocabulary cluster). The pending human disposition of PKG13-DEL-13-02-PKG02-001 is also part of that row's RemainingWork.

**Risk if unrepaired.**
- Silent defaults and fabricated provenance break OPS-K-DATA-2. This is latent while the contract has no product caller.
- Unitless or mis-unit values pass schema checks (OPS-K-UNIT-1).
- The claims-language guard misses claims that name the current product. This touches the professional boundary.
- The product panels show static or false packet values.
- A protected check stays removed.

**Representative keys.**
- `DEL-15-02:SOW#CLM-011.r07`. A record with no mapping_status is emitted as "mapped" (`F:core/handoff/target_mapping/contract.py:239`).
- `DEL-08-05:SOW#CLM-011/DEL-08-05-REQ-006`. The phrase list names only the former product (`F:core/reporting/protected_content_linter/src/lib.rs:639`).
- `DEL-10-04:SOW#CLM-013.s02`. The packet says `bundle_active: false` while the config says `"active": true`.

**Exceptions kept visible.**
- **CONTESTED (8).** The DEL-13-02 rows CLM-003.r13, CLM-009, CLM-010/R-13-02-005, CLM-012/R-13-02-005, CLM-013, CLM-018, CLM-019.r04 and CLM-024. The resolution text reads "unit-vocabulary cluster: Agent 0 settles tier/disposition/AuthorityNeeded corpus-wide". If Agent 0 settles the cluster as an owner item, these rows move to an owner route.
- **FIRM, `DEL-13-01:SOW#CLM-005.r05`.** The correction sets AuthorityNeeded to REVIEW and adds the force_per_length remaining work. The verifier found LOCAL_DESIGN · NO also defensible.
- **FIRM, `DEL-15-02:SOW#CLM-020`.** The verifier offers two options: re-dispose the block row, or split step 2.
- **FIELD, `DEL-10-04:SOW#CLM-013.s02`.** A correction to the VerificationClass.
- **PROTECTED_CHECK, `DEL-04-04:SOW#CLM-010/DEL-04-04-REQ-08`.** The only VERIFICATION_REMOVED row in the corpus slice. It is 100%-sampled.

---

## T7-C07 — Engineering validation and reference-model authority

**Description.** These are open engineering decisions or validation bases. In every case the records state accurately that verification exists and validation does not (A5: tests are not promoted to validation). One row has no witness at all.
- **DEL-05-03 (2). The pressure reference model.** The thin-wall membrane is set against an exact-annulus reference. The investigation of 2026-09-08 lists seven decisions, and D-67 adopted only a dormant kernel.
- **DEL-02-02 R05. An independent conversion witness.** An independent witness for conversion and normalisation is required under DEC-018.
- **DEL-13-04 REQ-007. The 3D frame target.** Its suitability has no validation basis.
- **DEL-14-04 (2) and DEL-14-05 R01. Tolerance suitability.** There is no validation basis for tolerance suitability or for comparison mechanics. The section-property oracle does not qualify.

**Signature.** VALIDATION_GAP · VERIFIED_NOT_VALIDATED (4) or DOCUMENTED_UNIMPLEMENTED (1), and AUTHORITY_UNCLEAR · ENGINEERING_AUTHORITY_REQUIRED (2) · all INVARIANT · BaselineClass OWNER_HOLD on the DEL-05-03 rows, NONE otherwise · layer VALIDATION.

**Population.** 7 rows.
- **Packages.** PKG-02, 05, 13, 14.
- **Deliverables.** DEL-02-02, DEL-05-03 (2), DEL-13-04, DEL-14-04 (2), DEL-14-05.

**Owning authority.** ENGINEERING. The DEL-05-03 pressure decision is an owner/engineering decision under an owner hold.

**Route.** ENGINEERING_AUTHORITY.

**On-ruling mechanism.** Engineering names a vetted validation basis for each subject: a benchmark, an independent witness or a vetted source. For DEL-05-03, the owner lifts the D01–D06 hold, then engineering rules the pressure reference model and its six companion decisions. Activation and validation then go through the physics-audit activation plan. Nothing here is a code fix or a record repair until that basis exists.

**Risk if unrepaired.** Pressure stress recovery, unit conversions and comparison tolerances would stay verified but not validated. If they are relied on beyond verification, engineering adequacy has not been established. This class records that fact; it asserts nothing about adequacy.

**Representative keys.**
- `DEL-05-03:SOW#CLM-011/DEL-05-03-RQ-001.s01`. The pressure reference model is under an owner-held decision.
- `DEL-14-04:SOW#CLM-017.s02`. PDU-047 is held, and the section-property oracle is not a validation basis.
- `DEL-02-02:STATUS#remaining/R05`. No independent conversion witness was found.

**Exceptions.** None. There are no contested, field or observed rows.

---

## T7-C08 — Owner-held independent validation, accurately recorded

**Description.** Private-by-default behaviour (PDU-049), usability and contrast (PDU-045/046, held under D-68), and live external-prover validation (PDU-050, gated by DEC-080) are all verified by the project. Each is held, by an owner hold or gate, pending an independent validation basis. The ledger found the declarations accurate. The Divergent flag reflects the underlying state (C6(b)), not a record error.

**Signature.** VALIDATION_GAP · VERIFIED_NOT_VALIDATED · INVARIANT · BaselineClass OWNER_HOLD (9) or NONE (1, DEL-17-05) · layers VALIDATION or VALIDATION;SECURITY.

**Population.** 10 rows.
- **Packages.** PKG-07, 17.
- **Deliverables.** DEL-07-03 (3), DEL-07-06 (6), DEL-17-05 (1).

**Owning authority.** OWNER.

**Route.** NO_ACTION. The records are correct, and the holds are existing owner holds. No R4 decision is needed unless the owner chooses to lift a hold.

**On-ruling mechanism.** None is needed. If the owner chooses to lift a hold (D-68, PDU-049 or DEC-080 activation), that is a separate owner authorisation, and an independent validation tranche follows. The rows then change on re-verification.

**Risk if left as is.** Low for the records. The product risk is that behaviour relied on stays verified but not validated. The records already make that visible.

**Representative keys.**
- `DEL-07-03:SOW#CLM-012/DEL-07-03-R-011`. Private-by-default is implemented, and it stays VNV (PDU-049).
- `DEL-07-06:SOW#CLM-008.r03`. The independent usability basis is absent, held under D-68.
- `DEL-17-05:SOW#CLM-019`. ACC-006 stays VNV behind DEC-080. The note says "O10" is not defined in any governing record.

**Exceptions.** None. There are no contested, field or observed rows.

---

## T7-C09 — Protected-content / IP-data review record not located

**Description.** Each of these SOW rows names a protected-content, private-data or fixture-provenance review as its verification element, or as a retained record. The frozen code side generally holds: the values are invented and self-declared, and there is a keyword denylist. What is missing is the review record itself. No DEC-058 scan record exists under `validation/evidence/releases/` at the freeze. An agent's own reading is not a protected-content review (R0 review F8). This is an absence of a record, not evidence of a breach. The class also includes the two DEL-17-07 rows (AUTHORITY_UNCLEAR). There, a fixture's nominal-size OD and wall values coincide with one entry of a published dimensional table, and the boundary text does not settle whether single values fall under the protected-table rule. This file does not restate those values.

**Signature.** EVIDENCE_NOT_LOCATED · UNKNOWN · INVARIANT (29), and AUTHORITY_UNCLEAR · UNKNOWN · INVARIANT (2) · BaselineClass NONE (30) or empty (1) · layers IP_DATA or IP_DATA;RECORD.

**Population.** 31 rows.
- **Packages (8).** PKG-02, 03, 04, 06, 07, 09, 14, 17.
- **Deliverables (14).** DEL-02-05, 03-04, 03-05, 04-05, 06-01, 06-02, 06-03, 06-05, 07-02, 09-02, 14-01, 14-02, 14-03, 17-07.

**Owning authority.** REVIEW.

**Route.** REVIEW.

**On-ruling mechanism.** A bounded protected-content and private-data review under the review workflow. It runs once per fixture surface, not once per row. There are about 11 surfaces:
- the persistence fixtures;
- the branch and component fixtures, including the C-120 and C-130 values;
- the benchmark fixtures;
- the rule-pack schema and invented demo;
- the evaluator fixtures;
- the checker fixtures;
- the desktop UI fixtures and screenshots;
- the two post-SELF_CHECK stress fixtures;
- the model-state, analysis-run and state-diff fixtures;
- the PCF fixture.

Each review records a disposition. The DEL-17-07 dimensional-value question may need an owner or maintainer interpretation of the boundary's rule for single values. If the review cannot settle it, it escalates to OWNER_DECISION. When a review finds a problem, that becomes a separate repair. The rows change on re-verification.

**Risk if unrepaired.** The IP/data boundary (INVARIANT) rests only on self-declaration and keyword screening. A protected value in a public fixture would go undetected until release. DEC-058 scan evidence is absent at the freeze.

**Representative keys.**
- `DEL-06-01:SOW#CLM-011/REQ-06-01-007`. The rule-pack schema and invented demo have no protected-content review record.
- `DEL-09-02:SOW#CLM-005.r07`. Two fixtures were added after the 2026-06-06 SELF_CHECK and never reviewed.
- `DEL-17-07:SOW#CLM-017/DEL-17-07-REQ-041`. The dimensional-table coincidence (AUTHORITY_UNCLEAR).

**Exceptions kept visible.**
- **OBSERVED (6).** DEL-06-01 CLM-004.r08, CLM-011/REQ-06-01-007, CLM-013, CLM-014, CLM-020 and CLM-021.s01. Agent boundary self-reviews do exist (the DEL-06-05 task runs of 2026-04-30 and 2026-06-05, and the DEL-06-01 run of 2026-06-12), but the 2026-06-14 demo changes overtook them. STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN is also defensible. The sealed values stand.
- **FIELD (3), AuthorityNeeded REVIEW.** DEL-07-02 CLM-015/RQ-008, CLM-018.r08 and CLM-025.r06.
- **FIRM, `DEL-07-02:SOW#CLM-026`.** The correction sets AuthorityNeeded to REVIEW, with RemainingWork as for RQ-008.
- **FIRM, `DEL-06-05:SOW#CLM-020.r05`.** The correction sets AuthorityNeeded to REVIEW and adds a remaining-work search. The sealed Notes say "Holds at the freeze", and BaselineClass is empty (see the R3 observations).

---

## T7-C10 — Other evidence not located (smallest checks)

**Description.** These rows could not be decided from the frozen records. Each has a named smallest check: locate the record, or trace or run something read-only. None touches the IP boundary. Subjects:
- **Architecture no-bypass (4).** Trace every import, adapter, plugin and status-write path to its schema or unit validation: DEL-02-01 REQ-10, DEL-02-02 U-002 and CLM-035, DEL-02-03 R10.
- **Native re-observation (4).** Re-observe short-panel scrolling and the inspector AX omission on the post-SWBPIPE-B shell: DEL-07-01 R05, DEL-07-02 R04, DEL-07-06 R04, DEL-07-09 R04.
- **Undefined referents.** "Provider expansion" (DEL-00-04) and the PDU-037 matrix (DEL-02-02 R01) are named but never defined.
- **Missing provenance notes.** The reducer, flange and valve provenance notes are missing (DEL-03-05 CLM-014).
- **A no-transmission test.** No test for lint or scan runs making no transmission was found (DEL-08-05 CLM-013.r08, INVARIANT, security).
- **A validator run.** Run the dependency validator against the frozen Dependencies.csv (DEL-15-02 CLM-021.r07).
- **Run-record hosting.** Where TP-SEAM-WASM-001 run records are hosted (DEL-00-02 .s02). Governing sources are silent, and the question is routed to REVIEW.

**Signature.** EVIDENCE_NOT_LOCATED · UNKNOWN · LOCAL_DESIGN (8), PROJECT_BASELINE (4) or INVARIANT (1), and AUTHORITY_UNCLEAR · UNKNOWN · LOCAL_DESIGN (1) · BaselineClass NONE · layers RECORD, BASELINE or SECURITY.

**Population.** 14 rows.
- **Packages.** PKG-00, 02, 03, 07, 08, 15.
- **Deliverables (12).** DEL-00-02, 00-04, 02-01, 02-02, 02-03, 03-05, 07-01, 07-02, 07-06, 07-09, 08-05, 15-02.

**Owning authority.** REVIEW.

**Route.** REVIEW.

**On-ruling mechanism.** Each smallest check is run under the review workflow as a read-only investigation. The native re-observation is the exception: it needs a separately authorised runtime observation. Each result either confirms the row, which then routes to R5_RECORD_REPAIR or NO_ACTION, or turns up a defect, which gets a new CODE_FIX_CANDIDATE. The no-bypass trace can be done once for all four rows.

**Risk if unrepaired.** Four risks:
- The architecture no-bypass rule (AB-00-02, AB-00-07) stays unconfirmed on the product import paths.
- Accessibility statements may no longer describe the shipped shell.
- Two Remaining statements name things nobody can identify.
- A privacy INVARIANT rests on structure alone, with no test behind it.

**Representative keys.**
- `DEL-02-01:SOW#CLM-011/REQ-02-01-10`. No evidence was found that every product import path validates.
- `DEL-07-02:STATUS#remaining/R04`. The observations predate the SWBPIPE shell work of 2026-09-18 to 09-21.
- `DEL-08-05:SOW#CLM-013.r08`. Only structural evidence exists for no transmission.

**Exceptions kept visible.**
- **CONTESTED, `DEL-03-05:SOW#CLM-014`.** By analogy with CP-08, this could be STALE_REVIEW_OR_EVIDENCE · RECORD_DRIFT. The smallest check has already been run.
- **FIRM, `DEL-02-02:SOW#CLM-035`.** The correction sets RemainingWork to U-002's smallest check.
- **RESOLVED_PAIR, `DEL-07-06:STATUS#remaining/R04`.** AuthorityNeeded NO; RemainingWork as for DEL-07-02 R04.

---

## T7-C11 — Record text inaccurate at authoring (OTHER)

**Description.** The SOW construction notes name fields, properties or tokens that the schema or code never had. The schema history at 7bee9ae41 and 101dcb420 already shows the same property sets. The text was wrong when it was written. It was not overtaken by later code, and it is not a rename.
- **DEL-15-01 (5).** Mis-describes the `model_hash`, `units_manifest` and `entity_ids` slots.
- **DEL-15-02 (3).** Mis-describes the mapping-record, behaviour-flag and `source_context` fields.
- **DEL-01-02 (1).** Names the TYPES label `UNKNOWN_SOURCE` where the checklist says `unknown`.

**Signature.** OTHER · IMPLEMENTED_DIFFERENTLY · LOCAL_DESIGN · BaselineClass NONE · layer RECORD · AuthorityNeeded NO.

**Population.** 9 rows.
- **Packages.** PKG-01, 15.
- **Deliverables.** DEL-01-02 (1), DEL-15-01 (5), DEL-15-02 (3).

**Owning authority.** NONE. No ruling is implicated, per the ledger Notes.

**Route.** R5_RECORD_REPAIR.

**On-ruling mechanism.** At the deliverable's next catch-up, under an R5 record-repair ruling, the text is aligned with the schema. The alternative the ledger offers is to extend the schema. That would be a CODE_FIX_CANDIDATE, and it is a deliverable-level choice.

**Risk if unrepaired.** Readers and implementers would work from field names that do not exist, and a later "fix" could bring the schema into line with wrong text.

**Representative keys.**
- `DEL-15-01:SOW#CLM-006.r02`. The Checksum definition has no provenance property.
- `DEL-15-02:SOW#CLM-006.r03`. `source_entity_ref` and `target_field_ref` never existed (git log -S).
- `DEL-01-02:SOW#CLM-010.r05`. `UNKNOWN_SOURCE` against the checklist's `unknown`.

**Exceptions.** None. There are no contested, field or observed rows.

---

## Owner-decision candidates (summary for R4)

1. **T7-C01.** About 12 confirm-or-rule decisions on CP-10 holds. The largest are the GUI component and state library (15 rows) and the private-data root (8 rows).
2. **T7-C02.** The JSON hash basis: move to RFC 8785, amend AB-00-04 for the sorted-compact paths, or take the split option.
3. **T7-C03.** DEC-009 and Python core engines: one project-wide answer (overlaps T8).
4. **T7-C04.** Product posture wording (ISSUED); the pre-release legal review; where the palette code lands; the secret provider.
5. **T7-C05.** Five decisions on intent before any fix: export redaction (11 rows), carrying the producer's diagnostic class and remediation (9), bare numerics at the import gate (3), the preview design-knowledge exception (1), and the expansion-loop tolerance under DEC-026 (3).

## Coverage

- **Population.** `R3/CORPUS_CLAIMS.csv`, filtered to `Divergent == YES` and effective `CauseTag` in {POSSIBLE_DEFECT, VALIDATION_GAP, VERIFICATION_REMOVED, EVIDENCE_NOT_LOCATED, AUTHORITY_UNCLEAR, OTHER}. That gives 213 rows: POSSIBLE_DEFECT 77, AUTHORITY_UNCLEAR 69, EVIDENCE_NOT_LOCATED 42, VALIDATION_GAP 15, OTHER 9, VERIFICATION_REMOVED 1. This matches R3_PLAN.
- **Output.** `T7_CLASSES.csv` has 213 body rows plus `#END`.
- **Class counts.** C01 50, C02 11, C03 7, C04 5, C05 27, C06 42, C07 7, C08 10, C09 31, C10 14, C11 9. They sum to 213.
- **Check run.** A read-only Python script (csv module) that:
  - re-derives the population from CORPUS_CLAIMS;
  - confirms that the CSV keys equal the population set exactly, with no duplicates and none missing;
  - confirms that every key's DeliverableID matches CORPUS_CLAIMS;
  - confirms that the `#END` count equals the body count.

  The script reported: population 213, CSV body 213, unique 213, missing 0, extra 0, DeliverableID mismatches 0.
- **Ledger join.** All 213 keys were found in the sealed forward ledgers, excluding `superseded_*`, for Notes and RemainingWork.
- **Excluded inputs.** No `RESOLUTIONS_DRAFT*.csv` or merged-draft file was read. Effective values and OtherCorrections were taken from CORPUS_CLAIMS. No other task's files were read.

## R3 observations

These are observations only. None is a correction.

1. **Inconsistent AuthorityNeeded on one substance.** DEL-08-03 (6 rows, AuthorityNeeded OWNER, tier PROJECT_BASELINE) and DEL-08-04 CLM-011.r05 and CLM-025.r04 (REVIEW) describe the same desktop behaviour: the diagnostic class is derived from severity, and a fixed remediation string is substituted. DEL-00-06 REQ-06-02 (OWNER) raises the parallel runner-side case. T7 classes all three together in C05 so that one ruling covers them. DEL-08-03's RemainingWork calls itself an "R4 code-change candidate", which suggests that REVIEW together with CODE_FIX_CANDIDATE would also be defensible.
2. **An unrecorded mislabel in DEL-17-04.** The DEL-17-02 REQ-007 Notes record that the MBF package (DEL-17-04) labels sorted-compact bytes "JCS_compatible_json_payload_hash". No T7-population row in DEL-17-04 carries this. Any C02 hash-basis ruling or code fix should include that label.
3. **A disposition that contradicts its Notes.** `DEL-06-05:SOW#CLM-020.r05` has effective disposition UNKNOWN / EVIDENCE_NOT_LOCATED and an empty BaselineClass. Its sealed Notes say "Holds at the freeze", and its sealed RemainingWork is NONE. Only the FIRM correction supplies a review action. The empty BaselineClass is the only blank in this population.
4. **One class choice duplicated across rows.** DEL-13-01 CLM-005.r05 (C06) and the DEL-13-02 unit cluster (C06, contested) both need `force_per_length` added to `Quantity.dimension`. The C05 row DEL-13-01 REQ-13-01-011 concerns a different subject. If Agent 0 settles the contested unit cluster as an owner item, CLM-005.r05 should follow it for consistency.
5. **A cross-task dependency.** The DEL-07-09 palette landing (C04) depends on capability ownership, which T1–T3 examine. The R4 packet may want to pair them.
