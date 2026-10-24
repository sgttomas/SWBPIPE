# Report package for current results (W-18)

**Basis:** `origin/main` at `9361129700` (2026-10-24), after W-3 (#1553, the
local-private rendered report) and W-19 (#1585, the rule-check headline).
Citations are at that head unless marked as the unmerged W-6a branch
(`claude/piping-reports-w6a-20261024` at `c38e1b63cb`, PR #1588). Revision 2,
after an independent Codex review. Design before implementation.
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## 0. Rulings and requirements

- **FR-REP-001 (Must):** "Report includes inputs, versions, states, runs,
  assumptions, warnings, results, and rule-pack references." PRD §19.2 lists the
  content; §22.6 names the full report package.
- **FR-REP-004 (Should):** comparison summaries in reports. The package carries
  the within-run comparison sections today (DEL-08-06).
- **DEL-12-02 REXC-REQ-006:** local-private exports may retain private values
  only with explicit local-private intent and a warning or audit note.
- **DEC-028:** the package is a multi-member archive; one package version, older
  versions refused by name with no migration (owner, 2026-10-10: "Close: one
  version").
- **Ruling 10A (owner, 2026-10-21):** exact-route results stay refused for
  reports and the report package until DEL-08-01 designs their limitation
  sections. Opening them returns to the owner (W-11). This design does not open
  them.
- **Scope of this design:** Save Report Package (local-private) for a Current
  pressure-free result (route `preview_physics`, identity
  `openpipestress.result_semantics/0.3.0/preview-physics-2`,
  `semantic_contract.rs:125`). The shareable path is a finding only (§3).

## 1. Where things stand

- **The gate.** `reportPackageUnavailableReason` refuses the pressure-free route
  and every fresh identity with `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`
  (`reportPackageRequest.ts:27-32`; texts `knownSemanticLimitations.ts:27,36`;
  "until T6", `reportPackageRequest.ts:22`). Save Report Package… is therefore
  disabled for every result the desktop produces (`App.tsx:218,354,732-734,1212`).
- **What has and has not been produced.** No qualified desktop Current package
  has been produced. The shared Rust producer is not idle: the headless runner's
  `export-results` calls it (`core/runner/headless/src/bin/openpipestress-runner.rs:518`),
  and the committed success witness
  `validation/witness/generated/del1005_export_results_success.json` holds a
  hash-valid 1.0.0 package whose one result member is a 0.1.0 envelope
  (reviewer's parse, `scratch/rep-w18-review/witness-check.json`). The
  determinism test exercises it (`tests/test_headless_runner_contract.py:652`).
- **Why the desktop path was closed.** Below the gate, assembly transcodes the
  result into the typed DEL-08-04 envelope (`reportPackageRequest.ts:340-376`)
  through `projectReceivedReportResults` (`:120-205`):
  - force, moment and section-property rows without legacy metadata are dropped
    into `REPORT_SOURCE_EVIDENCE_DISCLOSED` diagnostics (`:149-152,358-363`);
  - every other family, stress included, is kept, with `metadata: null` whenever
    its metadata is outside the legacy vocabulary (`reportMetadata`, `:65-75,188`),
    so a current stress row loses its component, location and basis;
  - the Rust boundary refuses rows whose metadata carries current tokens
    (`wire.rs:659-718`). The desktop never sends them (the legacy lists exclude
    them), so this is defence for other callers.
- **The current result document already exists.** `buildCurrentResultExport`
  (`resultExportAdapter.ts:233-276`) produces the DEL-08-04 0.3.0 document for a
  Current result and is what Export Result JSON saves, as `JSON.stringify` of the
  document (`ResultExportPanel.tsx:53,78,169-171`). Its identity is carried in
  `result_envelope.producer.semantic_contract_id` and `semantic_contract_ref`
  (`resultExportAdapter.ts:85`). Rust has the same derivation and an
  independent validator (`core/reporting/result_export/src/derivative.rs:120,390`,
  derivative hash `:760-773`).
- **Rule checks.** The package refuses after any rule check
  (`reportPackageRequest.ts:266-270`; `wire.rs:719-724`). Its renderer call
  passes no `ruleCheckRun` (`reportPackageRequest.ts:277-284`), so the W-3
  rule-pack references and outcomes (`renderableReportInput.ts:637,685`) never
  reach it; its audit manifest and envelope hard-code `rule_pack_refs: []`
  (`reportPackageRequest.ts:374,405`).
- **Data boundary.** The save is local-private: route `DREP-PACKAGE-SAVE-009`,
  its own intent control (`workspaceSession.ts:2572-2632`;
  `reportRedactionProjector.ts:58-68`). The native command checks only
  caller-supplied evidence: route id and counts (`src-tauri/src/lib.rs:213-219`)
  and booleans (`:64-86`). Nothing native screens content; the typed decode runs
  first (`:203-205`). W-6a found and repaired the same gap for the model document.

## 2. Design

### 2.1 Availability

- **Route gate: the output policy's `report-package` cell.**
  `reportPackageUnavailableReason` becomes
  `surfaceRouteRefusal(result, "report-package")` (`outputPolicy.ts:159-164`),
  synchronous, as today's call site needs (`App.tsx:218`):
  - `preview_physics` is ungated (`outputPolicy.ts:94`); the builder's checks
    apply.
  - `pressure` and `pressure_source` keep `refused`, with
    `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` (`:93,99`; ruling 10A).
  - `retained_preview_physics` and `retained_physics` keep `refused` (`:110,123`).
  - `unsupported` falls to the builder's contract check.
- **Admission: the result-export builder's.** Assembly calls
  `refuseSurfaceOutput(result, model, "report-package")` first, since
  `buildCurrentResultExport` gates on the `result-export` cell, which admits
  exact routes (`resultExportAdapter.ts:239`; `outputPolicy.ts:84`). It then calls
  `buildCurrentResultExport` for the same session inputs and asserts the
  document's identity is preview-physics-2 (`REPORT-PACKAGE-ROUTE-NOT-ADMITTED`).
  A builder refusal is shown with its own code.
- **Menu.** Enablement stays synchronous: the route gate plus the presence
  checks (`App.tsx:354`). There is no asynchronous precheck. A builder refusal is
  shown in the status line after the attempt (`App.tsx:734`).
- **Retired.** `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE` and `N_REPORT`
  (`knownSemanticLimitations.ts:27,36`; `wire.rs:659-704,711-718`), the test seam
  `assembleReportPackageRequestBelowAvailabilityGate` and their tests; the texts
  that cite them change as §5 lists.

### 2.2 Members (package version 2.0.0)

| Role | Member | Source |
|---|---|---|
| `package_manifest` | `package_manifest.json` | Unchanged form, `schema_version` `2.0.0` |
| `canonical_rendered_report_html`, `canonical_rendered_report_pdf` | as today | `buildRenderableReportInput` under `local_private`, `generatedAt: null`, **with the session's `ruleCheckRun`** (§2.5) |
| `audit_manifest` | `audit_manifest.json` | DEL-08-02, assembled as §2.4 maps it |
| `result_export` | `result_export_<envelope id>.json` | The 0.3.0 document from `buildCurrentResultExport`, unchanged |
| `state_comparison_handoff_sections` | as today | `buildStateComparisonHandoffSections`, unchanged |

- **Canonical equality.** The member bytes are the JCS form of the same document
  Export Result JSON saves. `JCS(parse(saved file))` equals the member, and the
  derivative hash is the same. The member is not recomputed or re-labelled.
- **Version.** The `result_export` member's form changes, so
  `PACKAGE_MANIFEST_SCHEMA_VERSION` moves to `2.0.0` (`report_package/src/lib.rs:71`)
  and the reader refuses `1.0.0` by name (`OlderVersion`, `reader.rs:86`), as
  DEC-028 rules.
- **Not a member.** The received result carrier (the document's source) and the
  model's canonical text travel in the request as validation and screening
  inputs only (§2.3, §2.6). Neither is written to the package.
- **Retired from the package path.** `projectReceivedReportResults`,
  `reportMetadata`, `reportSolverIdentity` and the 1.0.0 envelope in
  `reportPackageRequest.ts:65-205,340-376`; the typed result-envelope DTOs and
  their conversion in `wire.rs:87-197,372-648`. The typed
  `result_export::ResultEnvelope` stays: the headless runner still builds its
  base document from it
  (`result_envelope_binding.rs:280-411`).

### 2.3 The shared producer: request 2.0.0 and its checks

`report_package::wire` (used by the Tauri bridge and the headless runner) takes a
new request. Changes from `wire.rs:8-20`:
- `result_envelopes: Vec<ResultEnvelopeDto>` becomes
  `result_documents: Vec<Value>` (exactly one) and
  `source_results: Vec<Value>` (the carrier each document derives from, same
  order);
- `report`, `state_comparison_handoff_records` stay `Value`;
- `audit_manifest` keeps its DTO with the §2.4 vocabulary;
- `rule_check_aggregate`, `solve_rule_check_status` stay.

Before assembly, a new `report_package::bindings` module checks, in this order,
each with a named `REPORT-PACKAGE-…` code:
1. **Document form.** `schema_version` and `result_envelope.schema_version` are
   both `0.3.0` (`RESULT_EXPORT_VERSION`, `semantic_contract.rs:458`);
   `deliverable_id` is `DEL-08-04`.
2. **Identity and table.** `for_source(source)` (`semantic_contract.rs:626`) and
   `for_export_metadata(doc.result_envelope)` (`:464-477`) select the same table
   (pointer-equal `&'static Value`). The source's
   `producer.semantic_contract_id` is in the package's admitted set,
   `[PREVIEW_PHYSICS_ID]` only. Any other identity is refused with
   `REPORT-PACKAGE-ROUTE-NOT-ADMITTED`, so the exact routes stay closed at both
   boundaries.
3. **Document validity.** `derivative::validate_document(doc, source)`
   (`derivative.rs:390`) covers:
   - the copied producer, numerical quality, formulation basis and contract
     evidence;
   - `semantic_contract_ref`, the origin and carrier hash, row accounting,
     annotations, unit witnesses;
   - the derivative hash (`:760-773`).
4. **Cross-member bindings.** `d` is the document, `e` its `result_envelope`,
   `a` the audit manifest, `r` the report input:
   - **model:** `e.model_ref.ref_id` = `source.model_ref` = `a.model_hash.payload_ref`
     = `source_model_ref.ref_id` = `r.report_sections.model_ref.ref_id`;
     `a.model_hash.value` = `e.reproducibility.model_hash.value` =
     `r.calculation_report.model_input_summary.model_hash.value`;
   - **run:** `source_basis_refs[0]` = `e.run_ref` =
     `r.report_sections.run_ref`;
   - **run hashes:** the report's `audit_manifest_refs` checksum values equal
     `e.reproducibility.run_hashes`' `analysis_run_record` and `received_result`
     values;
   - **load basis:** `source_basis_refs[1..]` equals `e.load_basis_refs` as a
     duplicate-free set;
   - **solver:** `e.solver_version.solver_name`/`solver_version` and
     `source.producer.component_name`/`component_version` equal the linked
     identity, as the audit manifest's already must (`wire.rs:310-318`);
   - **input manifest:** `e.reproducibility.audit_manifest_ref.ref_id` =
     `a.manifest_id`, the runner's existing rule moved here
     (`openpipestress-runner.rs:680-685`). When the origin class is
     `received_current_dimension_absent` (the desktop), the origin's
     `qualification_ref.ref_id` = `a.input_manifest_hash.payload_ref` =
     `r.calculation_report.model_input_summary.input_manifest_ref.ref_id`;
   - **unit system:** `a.unit_system_ref` = `e.unit_system_ref.ref_id`;
   - **status:** the document's rule tokens
     (`RULE_INPUTS_INCOMPLETE`/`USER_RULE_CHECKED`/`USER_RULE_FAILED` in
     `e.analysis_status`) are exactly `[rule_check_aggregate]` when the
     aggregate is present, and a subset of `[solve_rule_check_status]`
     otherwise. `r.calculation_report.analysis_status` equals
     `e.analysis_status` with `HUMAN_REVIEW_REQUIRED`.
5. **Rule-check content (§2.5).**
   - `rule_check_aggregate` is present exactly when `r.user_rule_checks` is,
     and equals its `aggregate_status`.
   - `r.calculation_report.rule_pack_refs` has one entry when it is present,
     none otherwise, and its `rule_pack_id` equals `r.user_rule_checks.rule_pack_id`.
   - `a.rule_pack_refs` is the §2.4 projection of that entry, field for field.
6. **Context.** `r.report_context.export_context` is `local_private` (both export
   profiles are local-private routes). This makes the renderer emit the
   local-private notice.

Then the assembler runs as today: renderer and linter gates, audit and member
validation, `export_blocked` (`report_package/src/lib.rs:203-240`). Its input
type changes from `result_envelopes: &[ResultEnvelope]` to the validated
documents (`:127-128,220-240`).

**The reader.** `check_member_document` checks only the deliverable id and file
name (`reader.rs:951-962`), so an old member inside a relabelled 2.0.0 package
would pass. For every package, blocked or not:
- the result member's two schema versions are `0.3.0`;
- `for_export_metadata` accepts its envelope;
- its derivative hash verifies.

For an unblocked package, in addition:
- its identity is in the admitted set;
- the audit member's `manifest_id` equals the member's `audit_manifest_ref.ref_id`;
- its model-hash value equals the member's.

The reader still has no source, so row accounting against the source stays the
producer's check. Its module text (`reader.rs:30-37`) is updated to say so.

### 2.4 Audit manifest: identity and checksum bases (DEL-08-02)

- **Mapping decision (audit manifest id).** The 0.3.0 document names the
  input-manifest id in `audit_manifest_ref` (`resultExportAdapter.ts:231,271`),
  while assembly names the audit manifest `audit-manifest:<run>`
  (`reportPackageRequest.ts:388`). The package keeps the document unchanged and
  sets the audit manifest's `manifest_id` to the input-manifest reference
  `inputManifest.manifest_ref.ref`. So the result member stays canonically equal
  to Export Result JSON's document, and the derivative goldens that build from
  `currentResultDocumentBase` are untouched. Precedent: the headless witness
  already names its audit manifest by its input manifest
  (`del1005_export_results_success_input.json`, `manifest_id` =
  `input_manifest_hash.payload_ref` = `manifest-1`). What differs from the 1.0.0
  path: only the audit member's `manifest_id`.
- **Unit system.** The audit manifest's `unit_system_ref` takes the document's
  `<model id>:units` (`resultExportAdapter.ts:231`), not the DEC-018 display ref
  (`reportPackageRequest.ts:34,404`). The report keeps the DEC-018 ref as its
  display policy (`renderableReportInput.ts:56-59`).
- **Truthful bases.** The audit vocabulary has only
  `project_local_deterministic_json` and `none`
  (`audit_manifest/src/lib.rs:71-74`; wire `wire.rs:249-260`), and its slots
  require the first (`:291-321,415-425,555-579`). The digests the desktop holds
  are computed differently:
  - the model hash is SHA-256 over RFC 8785 bytes (`canonicalSha256Hex`; the
    document labels it `openpipestress_jcs_ijson_v1`, the same bytes after the
    I-JSON guard, `resultExportAdapter.ts:15,33`);
  - the input-manifest hash is labelled `rfc8785_jcs` by the analysis record
    (`resultExportAdapter.ts:247,256`);
  - a pack's declared `rule_pack_checksum` is labelled `JCS`: RFC 8785 bytes of
    the payload excluding `checksums`, grammar-version-bound
    (`core/rules/rule_pack_document/src/lib.rs:756-782`;
    `schemas/rule_pack.schema.yaml` `ChecksumSet`);
  - W-3's fallback is `rfc8785_jcs` over the whole loaded document
    (`renderableReportInput.ts:428-434`).

  The 1.0.0 request labelled the first two `project_local_deterministic_json`
  (`reportPackageRequest.ts:389-402`), a basis they were not computed on. It never
  shipped (gated); this revision corrects it.
- **DEL-08-02 change.** `Canonicalization` gains two values:
  - `Rfc8785Jcs` (`rfc8785_jcs`): SHA-256 of the RFC 8785 bytes of the whole
    named payload;
  - `RulePackPayloadJcs` (`rule_pack_payload_jcs`): a pack's declared
    `rule_pack_checksum` under the rule-pack schema's definition, carried as
    declared and not recomputed.

  Slot rules:
  - model and input-manifest slots accept `project_local_deterministic_json` or
    `rfc8785_jcs`;
  - the rule-pack slot accepts those two or `rule_pack_payload_jcs`;
  - assets are unchanged.

  The crate still records and does not compute JCS (`lib.rs:1-7,268-272` stay
  true). The serializer and wire decoder change with it
  (`report_package/src/lib.rs:557-560`; `wire.rs:249-260`). This bears on
  DEL-08-02-R2 ("JCS-compatible canonicalization") and R5 (rule-pack checksum
  capture). Its "Current implementation" text (:116) changes (Appendix A3). The
  existing `project_local_deterministic_json` records, such as the headless
  witness, stay valid.
- **Safe-reference projection** (report `rule_pack_refs[0]` → audit
  `RulePackAuditRef`):
  - `rule_pack_id`, `version`→`rule_pack_version`, `source_notice` and
    `redistribution_status` are copied; `private_payload_redacted: true`;
  - checksum `sha256` + `JCS` (payload `rule_pack`) →
    `{sha256, rule_pack_payload_jcs, rule_pack_reference, <rule_pack_id>, value}`;
  - checksum `sha256` + `rfc8785_jcs` (payload `rule_pack_document`, the W-3
    fallback) → `{sha256, rfc8785_jcs, rule_pack_reference,
    rule_pack_document:<id>, value}`;
  - any other declared pair (`sha512`, `CALLER_SUPPLIED_JCS_BYTES_UNVERIFIED`,
    `NONE`, `TBD`, "not recorded") is refused with
    `REPORT-PACKAGE-RULE-PACK-CHECKSUM-BASIS-UNSUPPORTED`, naming the pair. The
    rendered report still shows the checksum as declared. Considered and not
    chosen: recording a desktop-computed document digest instead, because the
    audit value would then differ from the report's.

  Only safe metadata travels (R-08-01-008, REXC-REQ-009): no formulas, limits or
  rule values. The audit validator still blocks `protected_suspected` and
  unredacted private packs (`audit_manifest/src/lib.rs:444-466`).

### 2.5 The W-3 content and stale-work guards (desktop)

- **Input.** `ReportPackageRequestInput` (`reportPackageRequest.ts:207-215`)
  gains `ruleCheckRun: SessionRuleCheckRun | null`
  (`renderableReportInput.ts:73-76`). The caller passes the session's
  `ruleCheckReportRun`, which is exposed only while its aggregate is current
  (`resultsSessionState.ts:111-120`). Assembly passes it to
  `buildRenderableReportInput` (today's call `:277-284` omits it) and checks:
  - aggregate present ⇔ run present, and
    `run.result.aggregate_status === ruleCheckAggregate`;
  - the analysis record's one rule token (`resultExportAdapter.ts:265`) equals
    the aggregate when present, else `result.status.rule_check`;
  - `rulePackReference`'s pack-identity check stands
    (`renderableReportInput.ts:409-412`).

  `REPORT-PACKAGE-RULE-PACK-BINDING-UNAVAILABLE` (`reportPackageRequest.ts:266-270`)
  is retired.
- **Save basis.** `handleSaveReportPackage` captures `ruleCheckAggregate` but not
  the run, and its `isCurrent` checks neither (`workspaceSession.ts:2575-2596`).
  The basis gains the run, and `isCurrent` gains
  `ruleCheckAggregateRef.current === basis.ruleCheckAggregate` and
  `ruleCheckReportRunRef.current === basis.ruleCheckRun`. These are refs, because
  the awaited build outlives the render closure. `isCurrent` is evaluated after
  each await, as today (`:2609,2620`), and before the native call.
- **Invalidation under the same aggregate.** `handleRuleCheckAggregate` returns
  early when the aggregate is unchanged (`workspaceSession.ts:1059-1060`), before
  `invalidateReportPackageComputedState` (`:1063`, body `:1679-1685`). A changed
  pack or changed outcomes under the same aggregate would leave a stale package
  route. The run comparison moves above the early return:
  `if (nextRun !== previousRun) invalidateReportPackageComputedState();`. Presence
  equivalence alone cannot catch a mismatched pack or status. The checks are:
  - object identity here;
  - the id, status and aggregate equalities in TypeScript (above);
  - their Rust re-check (§2.3 item 5).
- **W-19 preserved.** The package's HTML and PDF come from the same renderer input
  through `pdf_emitter::assemble_full_report_package` (`report_package/src/lib.rs:205`).
  The aggregate headline therefore reads "INCOMPLETE — n check(s) failed; m
  check(s) blocked" whenever any check is incomplete
  (`report_renderer/src/lib.rs:317-332`), and `completeness_status` is
  `incomplete` (`renderableReportInput.ts:438-445`). Nothing in the package
  re-derives the headline.

### 2.6 Native admission and screening (Tauri)

Today the command decodes the typed request first (`lib.rs:181-205`), then
checks supplied evidence (`:207-219`), then assembles and opens the picker
(`:222-236`). The new order, all before the picker:
1. **Arguments.** `{requestCanonicalText, modelCanonicalText, screening,
   localFirstEvidence}`, decoded `ArgsDecoding::Moved` as W-6a's save is
   (W-6a `lib.rs:1813-1835`). `REPORT_PACKAGE_VEC_EXPANSION` and its pricing tests
   (`command_ingress.rs:552-568,976-1181`) are replaced by the moved-string rule
   plus a priced parse of the text, re-established by the counting-allocator test.
2. **Evidence shape.** `screening` takes W-6a's `ModelDocumentSaveScreening` form
   (W-6a `model_document_save.rs:68-84`, `deny_unknown_fields`):
   - route `DREP-PACKAGE-SAVE-009`, `local_private`, explicit intent, not blocked;
   - not withheld, lossless, exact payload match, zero blocking;
   - the decision and finding counts the receipt reports;
   - `payload_sha256`.

   The local-first check is unchanged (`lib.rs:64-86`). This remains defence in
   depth: intent is the user's webview choice, and Rust can only require its
   evidence (W-6a `model_document_save.rs:163-165`).
3. **Size cap**, then **canonical binding.** Parse `requestCanonicalText` to a
   `Value` and refuse unless:
   - `canonical_json(value) == text`;
   - `sha256:<hex>` of the text equals `screening.payload_sha256`
     (`REPORT-PACKAGE-SAVE-EVIDENCE-UNBOUND`), as W-6a binds its evidence
     (`model_document_save.rs:455-497`).

   Parse `modelCanonicalText` likewise and refuse unless its SHA-256 equals the
   raw request's `/audit_manifest/model_hash/value`
   (`REPORT-PACKAGE-SAVE-MODEL-UNBOUND`).
4. **Disposition screening of content**, on a blocking worker with the
   local-store lookup:
   - **Request `Value`, recursively.** It covers the report input, the result
     document, the source carrier, the audit manifest and the handoff records.
     Every object judged as the library import judges one (the hanger rule
     generalized: own fields win, else its `provenance`'s, and a
     provenance-versus-own conflict judges the provenance alone) blocks on:
     - `redistribution_status` `protected_suspected` or `rejected`;
     - `review_status` `quarantined` or `rejected`;
     - `privacy_classification` `protected_suspected`, matching the webview's
       rule (`redactionExportControls.ts:437-442`).

     Any `library_ref` met is resolved against the store as W-6a resolves it.
   - **The model text.** W-6a's `screen_model_document_dispositions` (W-6a
     `model_document_save.rs:382-447`): every `provenance_record`, every
     `rule_pack_refs[]` entry and every `library_ref`, store-resolved. It is
     needed because the report input re-labels component provenance
     `private_only`/`pending` (`renderableReportInput.ts:177-193`), so a
     quarantined source record is not visible in the request.

   Refusals: a blocked disposition is `PROTECTED_CONTENT_BLOCKED`, naming the
   record; a store read error is `REPORT-PACKAGE-SAVE-LIBRARY-STORE-UNAVAILABLE`;
   an unresolved reference is a receipt warning (`MODEL-LIBRARY-REF-UNRESOLVED`).
5. **Raw context.** `/report/report_context/export_context` is `local_private` and
   `/export_profile_id` is `desktop_local_private_report_package_1`.
6. **Typed conversion and the shared producer** (§2.3), then `export_blocked`
   (`lib.rs:224-229`), then the picker and the atomic write (`:232-310`).
   Admission is a pure `admit_report_package(...)` function, so its tests need
   no window.

**Sharing with W-6a.**
- W-18's desktop slice starts after #1588 merges.
- It extracts the disposition code from `model_document_save.rs` into a
  crate-private `export_disposition_screen.rs`: `blocking_dispositions`,
  `stored_item_blocking`, `collect_objects`, `DispositionScreen`,
  `screen_model_document_dispositions`, `LibraryLookup` (W-6a :230-447), plus the
  new generic `screen_value_dispositions`.
- Both saves call that module. W-6a's parity tests
  (`disposition_verdicts_match_the_library_import`, W-6a :1122) move with it,
  unchanged.
- The privacy rule lives only in the generic walker, so the model-document
  verdicts stay exactly the import's.
- The lookup closure (W-6a `lib.rs:1826-1836`) becomes one shared helper.
- If #1588 changes in review, the extraction follows its merged form.

**The webview side.** Before the call, `saveReportPackage`
(`reportPackageSaveService.ts:74-111`) does three things:
- computes `requestCanonicalText = canonicalJsonString(controlled.payload)` and
  its SHA-256;
- asserts the controlled payload is canonically equal to the request;
- sends `canonicalJsonString(model)`.

### 2.7 The headless runner (shared producer consumer)

- **Request.** `export_results` carries the §2.3 request. The runner's
  local-private gates stay (`--explicit-local-private-intent`). It decodes the raw
  payload to `Value` once (`openpipestress-runner.rs:458`) and uses the shared
  `wire` path.
- **Runner bindings** (`validate_export_results_bindings`, `:583-689`), rewritten
  over the documents. The shared §2.3 checks run first. The runner adds:
  - `runner.model_ref` equals `source_model_ref`, and also the document origin's
    `request_model_ref` when present (an alias is disclosed there,
    `result_envelope_binding.rs:271-272`), else `e.model_ref`'s id;
  - `runner.input_manifest_ref.ref_id` equals `a.input_manifest_hash.payload_ref`;
  - `runner.unit_system_ref.ref_id` equals `a.unit_system_ref` and
    `e.unit_system_ref.ref_id`;
  - `runner.load_basis_refs` equals `e.load_basis_refs` as a set.
- **Fixture transition.** Python cannot derive a 0.3.0 document. A Rust test in
  `core/reporting/report_package/tests/` derives one with `derive_document` and
  writes or asserts a committed input,
  `fixtures/reports/invented/del1005_current_result_document.json`. It uses:
  - the committed preview-physics-2 carrier
    `fixtures/results/preview_physics_invented_sparse.json`;
  - the base `fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json`
    (as `result_export/tests/derivative_contract.rs:12-31` does);
  - the linked solver identity, so a product version bump fails the test and
    regenerates the file.

  `generate_del1005_export_results_inputs.py` embeds that document and carrier
  and adds `report_context` (local-private). The four `del1005_export_results_*`
  inputs and their generated witnesses are regenerated in 2.0.0 form. The success
  witness becomes a 2.0.0 package with a 0.3.0 member, and the mismatch and
  blocked witnesses keep their codes. The determinism test
  (`tests/test_headless_runner_contract.py:652`) is unchanged in what it asserts;
  `export_input(…, linked_solver_identity)` must no longer rewrite the document's
  solver identity, because that would break its derivative hash.
- **Named-refusal alternative** (open item O-2): until a runner unit lands,
  `export-results` refuses `HEADLESS_RUNNER_EXPORT_RESULTS_VERSION_RETIRED` and the
  success witness becomes a refusal witness. Package determinism would then rest
  on the Rust assembler tests only.

### 2.8 Exact routes (W-11)

Nothing opens here. When the owner opens exact-route reports (W-11), the
package needs:
- the policy cell;
- the identity added to the admitted set (§2.3 item 2);
- W-11's rendered limitation sections.

The exact-route document already exists (`resultExportAdapter.ts:82-83,88`).

## 3. A shareable report path: the DEL-12-02 check (finding only)

**What the product does today.**
- Every session value is classified `private_project_data` / `private_only`
  (`renderableReportInput.ts:120-131`).
- In a public or shared context, private values get `redact_value`
  (`redactionExportControls.ts:466-473`; Python `core/security/redaction/controls.py:419`).
- The render route requires lossless materialization, so any redaction blocks
  it (`redactionExportControls.ts:963-967`). That is why the public render is
  `blocked` with no blocking finding (#1553's native probe).
- The `shared_model` and `downstream_tool` contexts exist
  (`redactionExportControls.ts:20-21,130-131`), but they redact private values as
  `public_report` does (`controls.py:419`), so neither gives an unredacted path.

**What the boundary allows.**
- Shared and public exports shall not silently include private project data
  "without documented rights" (REXC-REQ-002, DEL-12-02 ScopeOfWork :176).
  "Silent unredacted export to a shareable/public location" is the named
  anti-pattern (:470).
- `docs/IP_AND_DATA_BOUNDARY.md:99` already permits intentional export "with
  documented redistribution rights".
- Rule-pack detail in shared reports is limited to safe metadata (REXC-REQ-009,
  :183).
- What is missing is a runtime way for a user to record that basis for their own
  session's data.

**Options (reserved for the owner; this design builds none):**
- **(A) Redacted shareable report.** Within DEL-12-02 as written: a non-lossless
  route and a renderer that shows redaction markers. Because every session value
  is private, it would carry no project values and no results. It would carry
  only the notice, the structure, the software identity and the rule-pack safe
  metadata. M.
- **(B) User-documented release of project data.** This puts
  `IP_AND_DATA_BOUNDARY.md:99` into practice. The user records that the session's
  own project data is released for sharing, and the export records it. DEL-12-02's
  vocabulary gains one runtime classification; the default exclusion is
  unchanged. Then:
  - project values and results are included;
  - private rule-pack and library values stay redacted to safe metadata;
  - the protected-content linter and the disposition blocks still apply;
  - the notice says the user released the data. M.
- **(C) No shareable path in v0.1.** The local-private report and package are
  the product's reports. The document's notice ("not for sharing or
  publication") stays. No work.

It does not block §2.

## 4. Verification

Persistence, file output and the data boundary change, so each slice gets
targeted regression tests and an independent review on its own head. Pytest runs
under `t3_slot.sh` (or with prebuilt binaries from that head); cargo runs only
through the T3 wrappers.

- **Rust, `report_package` (new `tests/current_result_package.rs`) and `wire`:**
  - a 2.0.0 package from the derived invented document assembles, reads back,
    verifies its member hashes and is byte-identical on a second assembly
    (determinism);
  - **mixed version:** a 1.0.0 container is refused by name; a 2.0.0 manifest
    with a 0.1.0 result member, re-hashed so the inventory is consistent, is
    refused by the reader; a mixed `schema_version` / `result_envelope.schema_version`
    member is refused;
  - **identity:** pressure-2, pressure-source-1, the retained identities and an
    unknown id are each refused with `REPORT-PACKAGE-ROUTE-NOT-ADMITTED`; a
    document whose `semantic_contract_ref` differs from its producer's is
    refused;
  - **tamper:** a value edit with a recomputed derivative hash is refused by
    `validate_document`; a stale derivative hash is refused by the reader too;
  - **cross-member substitution:** one test per §2.3 item 4 binding, plus a
    source carrier from another run, an audit manifest from another model, and
    a swapped `manifest_id`;
  - **rule check:** aggregate without run, run without aggregate, a pack id
    mismatch between outcomes and reference, an aggregate differing from the
    document's token, and each unsupported checksum pair;
  - **W-19:** a package with one failed and one incomplete check shows the
    INCOMPLETE headline in the HTML member and in the PDF text.
- **Rust, `audit_manifest`:** the two new bases are accepted in their slots and
  refused elsewhere; the existing `project_local_deterministic_json` and `none`
  cases are unchanged.
- **Rust, Tauri (`admit_report_package`, sandbox directory):**
  - **forged evidence:** clean evidence over a request carrying a quarantined
    provenance at depth (in the report, the document and the carrier), a
    `protected_suspected` privacy class, or a rejected rule-pack ref; each is
    refused before the picker, and the sandbox directory stays empty;
  - evidence for other bytes; a non-canonical text; a model text not matching
    the audit hash; a model whose library record is quarantined in the store;
    a store read error;
  - a `public_report` report context under local-private evidence; a forged
    profile id; the existing evidence-shape refusals;
  - **native save and read-back:** admitted bytes written to the sandbox are
    read by `read_report_package_container`, and the result member's JCS equals
    the document's.
- **vitest:**
  - the gate follows the policy per route;
  - the result member is canonically equal to `buildCurrentResultExport`'s
    document;
  - the audit `manifest_id`, unit system and bases follow §2.4;
  - the package after a rule check carries the references in the report and
    the audit projection;
  - **same-aggregate changed outcomes:** a new run under an unchanged aggregate
    clears the computed package state;
  - **asynchronous staleness:** a save whose result, aggregate or run changes
    during the awaited build publishes nothing and never calls the native
    command;
  - each builder refusal is surfaced with its code.
- **pytest:** `tests/test_headless_runner_contract.py` (determinism, failures,
  output file) and the reader tests, on the regenerated 2.0.0 witnesses.
- **Native smoke:** Save Report Package… on the solved demo through
  `native_smoke::take_chosen_path`. The harness then checks:
  - the saved file's SHA-256 equals the receipt's;
  - the members hash-match the inventory;
  - the result member's JCS equals that of the Export Result JSON document the
    same session saves.

  These need no new pinned values; the owner-held
  `apps/desktop/native-smoke/expected.json` is not changed.
- **Preserved:** the derivative goldens (the T6S golden built from
  `currentResultDocumentBase`, `retained_precision_derivative_golden.rs`) and the
  W-3 rendered-report tests, unchanged.
- **Review:** an independent Codex round per slice on its implementation head.

## 5. Reconciliation

Commitment and acceptance text: exact drafts in Appendix A, for the manager's
wording route; not applied by this design.
- **DEL-08-01 ScopeOfWork :81** (A1); `deliverable.yaml` code and check pointers.
- **DEL-08-06 ScopeOfWork :118** (A2).
- **DEL-08-02 ScopeOfWork :116** (A3), plus R2/R5 bearing noted in §2.4.
- **DEL-02-05 ScopeOfWork :214** (A4): the reader's current version and member
  form.
- **DEL-04-08 REQ-006 (c) :219, AC-006 :240, VER-006 :309-315** (A5-A7).

Implementation edits (not commitment text):
- **DEL-04-07 Design** `desktop-exact-outputs.md` §4.5 (:212): the report package
  refuses exact identities by policy and Rust identity, not by the fresh-result
  code.
- **User guide** `docs/user_guide/index.md`:
  - :109-111 — the bullet becomes "Save Report Package… saves a local private
    package for a pressure-free result after its own local-private selection;
    exact-route results are refused with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`.";
  - :148 — "and the report package refuses every current result" becomes "and
    the report package is saved locally for a pressure-free result";
  - :393-395 — the same clause change.
- **Completeness map** `docs/COMPLETENESS_MAP.md` :97, :543 (W-18 row), :586.
- **DEL-12-02:** the §3 finding as an open item, after the owner's choice.
- **CI routing** `tools/hosted-ci-routing.json`:
  - add the new files to the lanes that list their siblings: the Tauri sources
    (~:2357-2372, `export_disposition_screen.rs`), the reporting sources and
    tests (~:2494-2496 and the `report_package` tests list), and the new
    fixture;
  - refresh the digests of changed manifests (e.g. `:290` if
    `report_package/Cargo.toml` gains a dependency);
  - this is implementation, done in each slice.

## 6. Implementation slices and size

| Slice | Content | Size | Persistence / data boundary |
|---|---|---|---|
| **(a) Rust 2.0.0 transition** | `wire` request 2.0.0, `bindings`, assembler input, reader member checks, version 2.0.0; `audit_manifest` bases; headless runner request, bindings and witness transition; the Tauri bridge and ingress pricing adapted mechanically to the new types. The desktop stays gated, since its TypeScript gate is unchanged in (a) | L (~1.5k lines with tests, plus regenerated witnesses) | **Persistence:** the `.opsproj` format and its reader. Numerical meaning untouched |
| **(b1) Native admission and screening** | Canonical-text command, evidence binding, shared `export_disposition_screen.rs` extracted from W-6a, model screening, `admit_report_package`, forged-evidence and read-back tests. Starts after #1588 merges | M-L (~1k) | **Data boundary and file output** |
| **(b2) Desktop wiring** | `reportPackageRequest.ts` rewrite, gate and menu, rule-run threading and guards, retired constants, webview canonical text, vitest, native smoke step, §5 docs | M-L (~1k) | Data boundary (webview evidence) |

(b1) and (b2) may land as one PR if review prefers. The order is (a), then (b1)
and (b2).

## 7. Open items

- **O-1 (owner): the shareable path.** §3 options A, B or C. If none is chosen,
  the product has no shareable report and the notice stays.
- **O-2 (manager, owner if it changes a check): the runner transition.**
  - Recommended: the fixture transition (§2.7). It keeps the determinism and
    zip-exact witness.
  - Alternative: a named refusal. This changes an existing success witness and
    test expectation into a refusal, so it needs the owner's agreement.
- **O-3 (manager): sequencing on W-6a.** (b1) depends on #1588. If #1588 is
  delayed, the alternatives are:
  - (i) wait;
  - (ii) land (b1) with its own copy and dedupe in a follow-up. This is not
    recommended: it duplicates the import-parity logic.
- **O-4 (manager wording route): Appendix A.** A5-A7 change commitment and
  acceptance text, so the owner accepts them.

## Appendix A. Wording-route drafts (old → new; not applied)

**A1. DEL-08-01 ScopeOfWork :81** (current-implementation paragraph, last clause)
- Old: "and Save Report Package (`reportPackageUnavailableReason` in `reportPackageRequest.ts`, then `save_report_package` through `apps/desktop/src-tauri/src/report_package_bridge.rs`) refuses every current result with `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`."
- New: "and Save Report Package (`buildReportPackageRequest` in `reportPackageRequest.ts`, then `save_report_package` in `apps/desktop/src-tauri/src/lib.rs` and the shared producer `core/reporting/report_package/src/wire.rs`) saves a version 2.0.0 package for a Current pressure-free (preview-physics-2) result under `local_private` with the package's own explicit intent: its result member is the DEL-08-04 0.3.0 document that Export Result JSON saves, its rendered members carry the rule-pack references and user-rule outcomes, and the native command screens dispositions from content before the save dialog. Every other identity is refused (exact routes with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; owner ruling 10A, 2026-10-21)."

**A2. DEL-08-06 ScopeOfWork :118**
- Old: "Desktop report-package request (current, after B7b). `apps/desktop/src/features/report/reportPackageRequest.ts` assembles the state/run and comparison sections (`stateComparisonHandoffSections.ts`, kept equal to the Python assembler `core/reporting/state_comparison_handoff_sections/engine.py` by a shared parity fixture) only below its availability gate. The gate refuses preview-physics-2, its retained successor, pressure-2 and pressure-source-1 with T0R's `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE` (the exact identities since epoch 8; DEL-04-07 Design `desktop-exact-outputs.md` §4.5). Below the gate, which only its test seam reaches directly, assembly accepts only a preview-physics-2 source with a 0.3.0 analysis record, so no current result yields a desktop report package. B7b removed the physics-1 and source-blocks reasons with their retired identities."
- New: "Desktop report-package request (current, after W-18). `apps/desktop/src/features/report/reportPackageRequest.ts` assembles the state/run and comparison sections (`stateComparisonHandoffSections.ts`, kept equal to the Python assembler `core/reporting/state_comparison_handoff_sections/engine.py` by a shared parity fixture) for a Current preview-physics-2 result. The output policy's `report-package` cell refuses its retained successor and, under owner ruling 10A, pressure-2 and pressure-source-1; the package's Rust boundary admits only preview-physics-2 (DEL-08-01 Design `report-package-current-results.md`)."

**A3. DEL-08-02 ScopeOfWork :116** (current-implementation paragraph, second sentence)
- Old: "On the desktop, `apps/desktop/src/features/report/reportPackageRequest.ts` assembles the audit manifest, model hash and input-manifest hash for the Save Report Package request (`save_report_package` through `apps/desktop/src-tauri/src/report_package_bridge.rs`); that request is refused for every current result (`REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`)."
- New: "On the desktop, `apps/desktop/src/features/report/reportPackageRequest.ts` assembles the audit manifest for the Save Report Package request (`save_report_package` in `apps/desktop/src-tauri/src/lib.rs`) for a Current pressure-free result. The manifest is identified by the run's input-manifest reference, records the model and input-manifest hashes under `rfc8785_jcs`, and records the rule-pack checksum under the basis it was computed on (`rule_pack_payload_jcs` for a pack's declared checksum, `rfc8785_jcs` for a document digest). `validate_manifest` accepts those bases alongside `project_local_deterministic_json`."

**A4. DEL-02-05 ScopeOfWork :214**
- Old: "(`schema_version 1.0.0` today)"
- New: "(`schema_version 2.0.0` today)"
- Old: "It then verifies the archive form, the manifest against the version 1.0.0 form and every member against the manifest inventory and hashes;"
- New: "It then verifies the archive form, the manifest against the version 2.0.0 form, every member against the manifest inventory and hashes, and each result member's form (a DEL-08-04 0.3.0 document whose derivative hash verifies; on an unblocked package, of an admitted identity and bound to the audit manifest member);"

**A5. DEL-04-08 ScopeOfWork :219** (REQ-006 (c), first bullet)
- Old: "and the report package refuses it as it refuses every fresh identity (`REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`)."
- New: "and the report package refuses it with the same code at the desktop output policy and with `REPORT-PACKAGE-ROUTE-NOT-ADMITTED` at its Rust boundary (owner ruling 10A)."

**A6. DEL-04-08 ScopeOfWork :240** (AC-006)
- Old: "The desktop output policy refuses each named identity with its stated code."
- New: "The desktop output policy refuses each named identity with its stated code, and the report package's Rust boundary refuses every identity except preview-physics-2."

**A7. DEL-04-08 ScopeOfWork :311** (VER-006, `desktop-test` bullet), and one bullet added after it
- Old: "`apps/desktop/src/features/report/reportPackageRequest.test.ts` and `apps/desktop/src/services/ruleCheckService.test.ts`;"
- New: "`apps/desktop/src/features/report/reportPackageRequest.test.ts` and `apps/desktop/src/services/ruleCheckService.test.ts`;" followed by a new bullet: "- the report package's identity refusals, `core/reporting/report_package/tests/current_result_package.rs` (its routed lane);"
