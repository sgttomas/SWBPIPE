# Report package for current results (W-18)

**Basis:** `origin/main` at `9361129700` (2026-10-24), after W-3 (#1553, the
local-private rendered report) and W-19 (#1585, the rule-check headline).
Citations are at that head unless marked as the unmerged W-6a branch
(`claude/piping-reports-w6a-20261024` at `cb87128b69`, PR #1588; its screening
code is unchanged from `c38e1b63cb`). Revision 4, after three independent Codex
reviews. Design before implementation.
Standard claim fence applies (claims taxonomy per DEC-081).

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
  (round-1 review). The determinism test exercises the producer
  (`tests/test_headless_runner_contract.py:652`). Slice (a) stops committing
  that generated output: no test or tool reads it, and the determinism test and
  reproduction step C11 produce it.
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
- **Retired, by slice.**
  - In (a), the shared producer's fresh-result refusal and its metadata tokens
    (`wire.rs:659-704,711-718`) are retired. The native command keeps the same
    code as its temporary hold (§2.6, "Slice (a) hold").
  - In (b1), that hold is removed.
  - In (b2), the TypeScript gate's `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE` and
    `N_REPORT` (`knownSemanticLimitations.ts:27,36`), the test seam
    `assembleReportPackageRequestBelowAvailabilityGate` and their tests are
    retired.

  The texts that cite them change as §5 lists.

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
- **Not a member.** The received result carrier (the document's source) travels
  in the request, and the model's canonical text travels as a separate command
  argument. Both are validation and screening inputs only (§2.3, §2.6); neither
  is written to the package.
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
- item 0 is the route preflight;
- items 1-3 are source-independent and shared with the reader;
- items 4-8 need the source and the request.

0. **Route preflight.** The producer reads `result_envelope.producer.semantic_contract_id`
   first. If the id is outside the package's admitted set, `[PREVIEW_PHYSICS_ID]`
   only, it returns `REPORT-PACKAGE-ROUTE-NOT-ADMITTED`. That includes an
   unknown id and a missing one. So the exact routes stay closed at both
   boundaries, and an unknown id is not reported as a form error, which the
   schema's producer enum would otherwise produce first (`results.v0.3.schema.yaml:989-995`).
   Route admission is the producer's check. It is distinct from the header
   dispatch in item 2.
1. **Document form: the complete 0.3.0 contract.** A new source-independent
   validator, `report_package::document_form::validate_result_export_form(doc)`,
   replaces what `validate_result_envelope` gave the 1.0.0 path
   (`result_export/src/lib.rs:397-479`; called at
   `report_package/src/lib.rs:220-239`). `validate_document` explicitly leaves
   whole-schema validation to a separate gate (`derivative.rs:388-389`), so
   this validator provides it. It does four things:
   - it checks that the document is `guard_json`-clean (`derivative.rs:70-90`);
   - it evaluates `schemas/results.v0.3.schema.yaml` from its root with JSON
     Schema 2020-12 semantics, with the evaluator described below;
   - it checks the invariants the schema cannot state, carried over from
     `validate_result_envelope`:
     - `analysis_status` holds `MECHANICS_SOLVED` and `HUMAN_REVIEW_REQUIRED`
       and at most one rule token;
     - every provenance record is complete;
     - `solver_version` members are non-empty, with the build stamp as in
       item 5;
     - every magnitude is finite;
     - `schema_version` and `result_envelope.schema_version` are both `0.3.0`
       (`RESULT_EXPORT_VERSION`, `semantic_contract.rs:458`), and
       `deliverable_id` is `DEL-08-04`;
   - through the schema, it enforces every wrapper member and every envelope
     member (`run_ref`, `solver_version`, `unit_system_ref`, `load_basis_refs`,
     `result_sets`, `row_accounting`, `source_annotations`, `review_evidence`,
     `row_disclosures`, `diagnostics`, `provenance`, `analysis_status`,
     `downstream_use`, `professional_boundary`), the boundary flags as
     constants, and `deliverable_id`/`deterministic_ordering: true`.

   The reviewer's header-only counterexample
   (`scratch/rep-w18-review2/reader-header-only-counterexample.json`) has an
   admitted header and a valid self-hash but no `result_sets`,
   `row_accounting`, `professional_boundary` or `run_ref`. This validator
   refuses it.

   **The evaluator (slice (a)).** The existing checker cannot do this
   unchanged:
   - `source_blocks::shape_in` (`source_blocks.rs:70-169`) has no `contains`,
     `dependentRequired`, `if`/`then`/`else`, `not` or `uniqueItems`. The root
     schema uses the first five (`results.v0.3.schema.yaml:33,216,523,931,1277,2017,2154,2300,3837`)
     and its closure uses `uniqueItems` (`results_pressure_2.v0.3.schema.json:629`);
   - it applies `required` only inside an object branch with `properties`
     (`:122-144`);
   - it refuses objects without a `properties` map (`:133-137`), which
     `RawSourceAnnotationMetadata` deliberately allows (schema `:2178-2187`);
   - it adds an implicit 16384-item cap (`:148`);
   - it supplies no resources (`:70-71`), and it resolves only fragment
     references (`:80`), not whole resources (schema `:1366`).

   A new module, `report_package::schema_eval`, is therefore written (in
   `report_package` with `document_form`, so the pinned `result_export/src/lib.rs`
   is unchanged):
   - It evaluates the complete root schema and its transitively referenced
     repository resources with JSON Schema 2020-12 semantics.
   - It uses a closed, explicit local resource catalog and never loads a schema
     from the network. The catalog is `results.v0.3.schema.yaml`,
     `results_pressure_2.v0.3.schema.json`, `physics_source_recovery.schema.json`,
     `retained_precision_mp_v2.schema.json` and `load_reference_state.schema.json`,
     embedded with `include_str!`, which is the reviewer's measured closure
     (`scratch/rep-w18-review3/schema-resource-closure.json`).
   - It supplies `contains`, `dependentRequired`, `if`/`then`/`else`, `not` and
     `uniqueItems`.
   - It resolves both fragment and whole-resource references. An unknown
     reference or keyword fails closed.
   - It enforces `required` independently of `properties` or an explicit
     `type`.
   - Object schemas without `properties` admit their unconstrained members
     unless another keyword forbids them, so `RawSourceAnnotationMetadata`
     keeps preserving arbitrary JSON objects.
   - Schema validation adds no implicit `maxItems` limit. Any independent
     package resource limit is named separately and tested.
   - The keyword census covers the transitive closure and separates
     annotations (`description`, `title`, `$comment`, and `x-rp-encoding`,
     which the retained schema's `:5` defines as a G2 encoding note) from
     assertion and applicator keywords. `$comment` is ignored; `$schema`,
     `$id`, `$defs` and `$ref` receive their core and resource handling. The
     closure has 32 keywords (round-4 review census).
   - Coverage includes type unions and applicability, required-only schemas,
     both conditional branches, overlapping combinators, numeric and compound
     equality, exact bounds, both regex patterns, unconstrained metadata, array
     cardinality and uniqueness, and cross-resource reference bases.
   - Positive and negative behavioural tests exercise every supported
     constraint. A list of keyword names alone is not evidence of correct
     evaluation.
   - The existing source-block checker and its consumers keep their current
     behaviour, unless a separately reviewed change establishes compatibility.

   **Independent oracle.** A Draft 2020-12 validator already exists in the
   repository: the piping tests' `tests/schema_validation.py` (`:25-28`,
   `validate_instance` `:133-145`) uses Python `jsonschema.Draft202012Validator`
   with the same local registry (`:78-87`). The evaluator's committed
   vectors, `fixtures/results/schema_eval_vectors.json`, hold one valid and
   one invalid instance per keyword and per probe in
   `scratch/rep-w18-review3/shape-probes.json`, plus the counterexample and
   real derived documents. A pytest, `tests/test_schema_eval_parity.py`,
   validates every vector with that helper and asserts that the expected
   verdicts equal the Rust evaluator's verdicts. A Rust test asserts the same
   expected verdicts from the same file. In its routed lane the parity test
   requires `jsonschema` and fails if it is missing; it does not skip
   (`schema_validation.py:197`).

   One disagreement class is admitted, by name and case by case:
   `ecma_dollar_before_final_line_feed`. It covers a `pattern` of the form
   `^[0-9a-f]{N}$` against a string whose last character is a line feed.
   JSON Schema 2020-12 patterns use the ECMA-262 dialect, where `$` without
   the `m` flag matches only at the end of input. Python's `re`, the oracle's
   engine, also matches `$` before a final line feed. The evaluator keeps the
   ECMA-262 verdict, invalid (coordinator decision, W-18a review round 1).
   Each entry records its string's pointer and pattern. For each entry the
   test checks four things: the expected verdict is invalid, the oracle's is
   valid, the same case without the line feed is valid to the oracle, and the
   pattern refuses the string under an end-of-input anchor. Any other
   disagreement fails the test. The vectors include the round-1 reviewer's
   eight line-feed probes: the model and derivative hashes of the runner,
   pressure-2 and retained documents, and two 16-digit fragments.

   Two evaluator properties are also bound by tests:
   - Numeric equality is exact. Integers compare as integers, and an integral
     binary64 compares exactly with an integer, never by rounding the integer.
   - `uniqueItems` does near-linear work. Items are bucketed by a hash
     consistent with JSON equality and compared only within a bucket. A unit
     test counts the comparisons over 100 000 distinct items.

   The provenance completeness invariant (item 1) applies only at the
   instance paths where the schema applies `$defs/Provenance`. The evaluator
   reports those paths, so a member named `provenance` inside unconstrained
   raw annotation metadata is not read as a provenance record.

   **Considered and not chosen: a Rust validator crate.** The only Rust
   JSON Schema crate in the repository is `jsonschema = "=0.58.5"` in App v4
   (`projects/chirality-app-v4/app/src-tauri/Cargo.toml:27`). It is in no
   SWBPIPE Cargo workspace. Adopting it would add a new third-party dependency
   tree to the product, with its lockfile, licence and CI-digest changes. The
   round-4 review prefers the crate for conformance coverage, but finds the
   bounded evaluator defensible. The manager chose the evaluator to keep the
   product's dependency tree unchanged. Oracle parity supplies regression
   evidence; implementation tests and independent review must cover every
   construct used by the five embedded schemas.
2. **Header dispatch.** `for_export_metadata(doc.result_envelope)`
   (`semantic_contract.rs:464-477`) accepts the transported header, and
   `semantic_contract_ref` names the producer's contract.
3. **Self-hash.** The derivative hash verifies over the document without its own
   hash (`derivative.rs:760-773`).
4. **Source validity.** `for_source(source)` (`semantic_contract.rs:626`)
   selects the same table as item 2 (pointer-equal `&'static Value`).
   `derivative::validate_document(doc, source)` (`derivative.rs:390`) then
   checks:
   - the copied producer, numerical quality, formulation basis and contract
     evidence;
   - `semantic_contract_ref`, the origin and carrier hash;
   - row accounting, annotations and unit witnesses.

   Row accounting is a producer-only check: the reader has no source.
5. **Identity and hash bindings.** `d` is the document, `e` its
   `result_envelope`, `a` the audit manifest, `r` the report input, `h` the
   handoff record set. `digest` is the SHA-256 of the RFC 8785 bytes, as
   `derivative::digest` computes it (`derivative.rs:92-97`). Each hash record
   is checked for its algorithm, payload reference type and id, and value. Its
   basis label must be one that describes how that digest was computed
   (§2.4); a bare value match is not enough.
   - **Actual model:** these all name the same model id:
     - `e.model_ref` (type `model_payload`);
     - `source.model_ref`;
     - `a.model_hash.payload_ref`;
     - `source_model_ref` (type `model`);
     - `r.report_sections.model_ref` and
       `r.calculation_report.model_input_summary.model_ref` (type `model`);
     - `…model_input_summary.project_ref` (type `project`).

     A runner request alias is bound separately (§2.7). The model-hash value
     is the same in `a.model_hash`, `e.reproducibility.model_hash`,
     `…model_input_summary.model_hash`, and `h`'s `model_state_record`
     `hash_refs[0]`.
   - **Run, package group:** these all name the same analysis run:
     - `source_basis_refs[0]` = `e.run_ref` = `r.report_sections.run_ref`
       (each `analysis_run`);
     - `h`'s `analysis_run_record` `run_ref` (`AnalysisRun`);
     - `r.calculation_report.audit_manifest_refs[0].ref` (`analysis_run`).
   - **Run, tied to the source, by origin.** This ties the group to the source.
     Without it, a coherent relabelling of the whole group could pass.
     - For `received_current_dimension_absent` (the desktop), `e.run_ref.ref_id`
       equals `source.run_id`.
     - For `attested_headless_producer`, `e.run_ref.ref_id` equals the origin's
       `request_run_ref.ref_id` (type `runner_run`), and `source.run_id` stays
       bound to the origin's `mechanics_run_ref` by `validate_document`
       (`derivative.rs:467-475`; the producer sets both,
       `result_envelope_binding.rs:272`). The analysis-run and mechanics-run
       identities are not collapsed.
   - **Received result:** `digest(source)` (the carrier digest, available
     here) equals all of these:
     - the `received_result` entry of `e.reproducibility.run_hashes`, with
       payload `ResultEnvelope:result-envelope:<run>`;
     - the origin's `received_carrier_checksum`;
     - `r.calculation_report.audit_manifest_refs[1].checksum`, with ref
       `result_envelope:result-envelope:<run>`;
     - the matching entry of `h`'s `analysis_run_record` `hash_refs`.

     The `analysis_run_record` hash cannot be recomputed here, because the
     analysis record is not in the request. It is checked for equality only,
     across `e.run_hashes`, `r.audit_manifest_refs[0]` and `h`.
   - **Rows:** `r.calculation_report.result_export_refs` lists the source rows
     in order: ref `Result:<row id>`, with `checksum.value` equal to
     `digest(row)`. The same holds for `h`'s `analysis_run_record`
     `result_refs`.
   - **Load basis:** `source_basis_refs[1..]` = `e.load_basis_refs` = `h`'s
     `analysis_run_record` `load_basis_refs`, each as a duplicate-free set of
     (type, id).
   - **Current state and run sections in `h`.** Each of the two current
     sections occurs exactly once.
     - The `model_state_record` has `state_ref`
       `{ModelState, state:<actual model id>:preview}`, `section_id`
       `state-run:state:<actual model id>:preview`, and a `hash_refs[0]` payload
       ref naming that state.
     - The `analysis_run_record` has `state_ref` naming the same state and
       `section_id` `state-run:<analysis run id>`
       (`stateComparisonHandoffSections.ts:107-120,144-148`).
     - Comparison sections keep their own separately labelled sources.
   - **Solver:** these all equal the linked identity (as the audit manifest's
     already must, `wire.rs:310-318`):
     - `e.solver_version.solver_name`/`solver_version`;
     - `source.producer.component_name`/`component_version`;
     - `r.report_context.solver_name`/`solver_version`.

     `e.solver_version.solver_build_ref` = `r.report_context.solver_build_ref`.
     That build stamp is `<name>@<version>` for the desktop profile, or the
     headless producer's
     `headless_preview_producer; solver_component=<name>@<version>[; nonlinear_component=…]`
     for the runner profile (`result_envelope_binding.rs:299-311`).
   - **Input manifest:** `e.reproducibility.audit_manifest_ref.ref_id` =
     `a.manifest_id`, the runner's existing rule moved here
     (`openpipestress-runner.rs:680-685`).
     - `a.input_manifest_hash` (payload ref and value) equals the
       `input_manifest_refs[0]`/`input_manifest_hashes[0]` in `h`'s
       `analysis_run_record` `reproducibility`.
     - `…model_input_summary.input_manifest_ref.ref_id` equals that payload
       ref.
     - When the origin class is `received_current_dimension_absent` (the
       desktop), the origin's `qualification_ref` is
       `current_manifest:<that payload ref>`.

     The input manifest itself is not in the request, so its digest is bound,
     not recomputed.
   - **Unit system and display policy:** `a.unit_system_ref` =
     `e.unit_system_ref.ref_id`. On the desktop profile, `e.unit_system_ref`
     is `{unit_system, <actual model id>:units}`.
     `…model_input_summary.unit_system_ref` is the DEC-018 display-policy
     reference `{unit_system, unit-system:dec-018-si-dual-display}`. The unit
     display summary is the complete projection of `renderableReportInput.ts:56-59,108-117`:
     - `storage_convention` is `entered_units_preserved`;
     - `quantity_display_policy` is "display result-row values with their
       explicit units; no report-time conversion";
     - `conversion_performed` is `false`;
     - `result_units` is the sorted distinct non-empty source-row unit list;
     - `model_units` equals the parsed model's `project.units` map, desktop
       only (§2.6 item 5), compared by keys and values in any property order.

     The renderer prints these verbatim (`report_renderer/src/lib.rs:636-665`),
     and the completeness checker does not restrict them
     (`report_generator/src/lib.rs:219-227`). These checks assert that the
     recorded units are preserved. They do not assert a conversion.
   - **Status:** the document's rule tokens are exactly
     `[rule_check_aggregate]` when the aggregate is present, and a subset of
     `[solve_rule_check_status]` otherwise. These sets are equal:
     - `r.calculation_report.analysis_status`, with `HUMAN_REVIEW_REQUIRED`;
     - `h`'s `analysis_run_record` `analysis_status`;
     - `e.analysis_status`.
6. **Displayed content: the report-row contract.** The renderer prints its
   inputs verbatim (`report_renderer/src/lib.rs:625-685,710-738`). The report
   validator checks completeness, not equality with the source
   (`report_generator/src/lib.rs:537-596`). So the producer derives the
   expected rows from the validated source and requires equality
   (`REPORT-PACKAGE-REPORT-ROWS-MISMATCH`). The five-field source-row
   projection is the shared report-row contract. Document composition stays
   with `report_renderer`, and nothing here reimplements HTML or PDF
   rendering. The contract (today `renderableReportInput.ts:671-676`):
   - one row per source row, in order;
   - `row_id` is `row:<id>`;
   - `label` is `<kind> (<metadata.component>)`, or `<kind>` when there is no
     metadata;
   - `case_ref` is `<basis type>:<basis id>`, or `TBD`;
   - `quantity_display` is `<value> <unit>`;
   - `source_ref` is `result:<id>`.

   Two pure helpers implement it:
   - **TypeScript.** `reportResultRows(result)` is extracted from
     `renderableReportInput.ts:671-676` into
     `apps/desktop/src/features/report/reportResultRows.ts` and called by
     `buildRenderableReportInput`. The value text is JavaScript's own
     `${value}`.
   - **Rust.** `report_package::bindings::expected_report_result_rows(source)`
     is used by the producer check and by the fixture generator. The value text
     is `open_pipe_stress_canonical_json::canonical_number`
     (`canonical_json/src/lib.rs:400`), the ECMAScript Number-to-String form,
     after `guard_json`, never Rust's `Display`, whose exponent and
     negative-zero text differ.

   **Row parity.** Both helpers are checked against the same committed
   vectors, `fixtures/reports/invented/report_result_rows_parity.json` (input
   rows and expected rows):
   - the vitest `reportResultRows.parity.test.ts` runs in `desktop-test`;
   - the Rust `report_package/tests/report_rows_parity.rs` runs in the
     report-package lane;
   - both are registered in `tools/hosted-ci-routing.json`.

   The vectors cover:
   - metadata present and absent;
   - basis present and absent;
   - source order;
   - Unicode ids and units;
   - `-0`;
   - small exponent notation (`1e-7`, `1.5e-6`, `5e-324`); large exponent
     forms (from `1e21`) are integer-valued above 2^53 − 1, so `guard_json`
     refuses them before any row is formed, and the vectors record that;
   - integer-valued binary64 values.

   A change to the projection updates the contract, both helpers and the
   vectors together.

   The package passes no numerical-health rows (`renderableReportInput.ts:486-490`),
   so the row lists match one for one. This is what keeps the rendered rows
   equal to the result member's source values.

   Handoff content: the handoff record set's `section_set_id` names the run,
   and the current sections are bound as in item 5. Comparison sections keep
   their own labelled `run_context` (left and right) and the DEL-08-06
   assembler's content under its parity fixture. The desktop native layer adds
   the model-text checks (§2.6 item 5).
7. **Rule-check content (§2.5).**
   - `rule_check_aggregate` is present exactly when `r.user_rule_checks` is,
     and equals its `aggregate_status`.
   - `r.calculation_report.rule_pack_refs` has one entry when it is present,
     none otherwise, and its `rule_pack_id` equals `r.user_rule_checks.rule_pack_id`.
   - `a.rule_pack_refs` is the §2.4 projection of that entry, field for field.
8. **Context.** `r.report_context.export_context` is `local_private` (both export
   profiles are local-private routes). This makes the renderer emit the
   local-private notice.

Then the assembler runs: renderer and linter gates, audit validation, the result
member's form validation (item 1, as a blocking reason), and `export_blocked`
(`report_package/src/lib.rs:203-240`). Its input type changes from
`result_envelopes: &[ResultEnvelope]` to the validated documents
(`:127-128,220-240`).

**The reader.** `check_member_document` checks only the deliverable id and file
name (`reader.rs:951-962`), so an old or malformed member inside a relabelled
2.0.0 package would pass. The reader runs in this order:
1. **Every package, blocked or not:** every result member must pass items 1-3,
   that is the complete 0.3.0 form, the transported-header dispatch and the
   self-hash. A failure is a reader refusal, not a blocked-package diagnosis.
2. **An unblocked package, in addition:**
   - the member's producer id is in the package's admitted set (the route
     admission of item 0, applied after form);
   - the audit member's `manifest_id` equals the member's
     `audit_manifest_ref.ref_id`;
   - the audit member's model-hash value equals the member's.

These are equality checks, not signatures, not source authentication and not
row accounting. The reader has no source, so row accounting stays the
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
    rendered report still shows the checksum as declared;
  - under either supported pair, the checksum's `payload_ref.ref_id` must be
    the reference's own `rule_pack_id`; a checksum naming another payload is
    refused with `REPORT-PACKAGE-RULE-CHECK-BINDING-MISMATCH`, so projection
    never relabels another pack's checksum (review round 1, B1).

  Considered and not chosen for the checksum record: recording a
  desktop-computed document digest instead, because the audit value would then
  differ from the report's.

  **Declared, not verified.** A `rule_pack_payload_jcs` record carries the
  pack's own declared digest:
  - the package does not receive the pack's private payload bytes and does not
    recompute it;
  - W-3 projects declared evidence even for failed or incomplete runs;
  - carrying it is therefore not an independent check of its value or basis.

  The `rfc8785_jcs` fallback is the desktop's digest of the loaded document. It
  is computed by the desktop, not re-verified by Rust. The audit record says what
  each value is, not that the package verified it.

  Only safe metadata travels (R-08-01-008, REXC-REQ-009): no formulas, limits or
  rule values. The audit validator still blocks `protected_suspected` and
  unredacted private packs (`audit_manifest/src/lib.rs:444-466`).

**Checksum subjects** (review round 2). There is one rule: every checksum or
hash record names, in `payload_ref`, the subject it is attached to, by type
and by id, and that subject derives from the record's container. A record whose
location has no derivation is refused.

The rule is applied in four places:
- the result member: `document_form::check_document_subjects`, at the
  producer, the writer and the reader (unblocked packages);
- the audit manifest: `audit_manifest::validate_manifest`
  (`PayloadSubjectMismatch`), and `document_form::check_audit_subjects` at
  the reader;
- the calculation report: `report_generator::validate_report`
  (`REPORT-CHECKSUM-SUBJECT-MISMATCH`), which blocks the render and so the
  package;
- the remaining report and handoff records: `bindings`.

In the result member, the schema's own checksum types (`$defs/Checksum`,
`$defs/ScopedChecksum`) locate the records, through the evaluator's marking.
The round-2 review inventoried 640 references in 23 shapes;
`every_checksum_record_kind_refuses_another_subject` probes each kind for
both profiles, changing the type alone, the id alone, and both (review
round 3). In the table, E is the result member's `envelope_id`, R is
its `run_ref`, and C is the carrier: `attested_headless_producer:E` for the
runner origin, `received_current_carrier:<origin mechanics run>` for the
desktop origin.

| Record kind | Subject |
|---|---|
| member `reproducibility.derivative_hash` | `derivative_document:E` |
| member `unit_preservation_witnesses[].target_row_checksum` | `derivative_document:E` |
| member `reproducibility.model_hash` | `model_payload:<e.model_ref>` |
| member `reproducibility.request_hash` | runner only: `runner_request:<id>`, where the origin's `request_run_ref` is `run:headless-preview:<id>` (the producer's run naming, `headless/src/lib.rs`); otherwise refused |
| member `reproducibility.run_hashes[]` | `AnalysisRun:R` or `ResultEnvelope:result-envelope:R` (`bindings` also requires both) |
| member `raw_source_hashes[]` | C; the list is exactly `[original_producer_checksum]` when the producer is authentic, else empty |
| member origin `received_carrier_checksum` and `original_producer_checksum` | C |
| member `row_accounting[]`, `row_disclosures[]` and `unit_preservation_witnesses[]` carrier and producer row checksums | C (`validate_document` binds the rows to the origin's reference; this binds that reference) |
| member `rule_pack_refs[].checksum` | `rule_pack:<that entry's rule_pack_id>`, or `rule_pack_document:<id>` under `rfc8785_jcs` |
| audit `assets[].hash` | the entry's `asset_id`, with the entry's `payload_kind` (the audit's references are strings; the payload kind is their type) |
| audit `rule_pack_refs[].checksum` | `rule_pack_id`, or `rule_pack_document:<id>` under `rfc8785_jcs`, with payload kind `rule_pack_reference` |
| audit `model_hash`, `input_manifest_hash` | no container subject; `bindings` binds them to the source model and the handoff input manifest, and the reader binds the model hash value to the result member |
| report `model_input_summary.model_hash` | the summary's `model_ref` |
| report `result_export_refs[]`, `audit_manifest_refs[]` and `report_section_refs[]` checksums | the entry's own `ref` |
| report `rule_pack_refs[].checksum` | that entry's `rule_pack_id`, typed `rule_pack` or `rule_pack_document` (bindings further tie the type to the basis) |
| handoff `state_run_sections[]` hash refs, row hash refs and input-manifest hashes | the section's state, run, row and manifest (`bindings`) |

A `report_section_refs[]` entry's own `ref` is a free reference: nothing in
the package derives which section record it must name. The rule binds its
checksum to the entry, and its value is declared, not verified, like the
other report checksums. The reader sees no calculation report, only the
rendered members, so the report-side rules apply at the producer.

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
  the awaited build outlives the render closure.
- **Placement of the last guard.** The new preparation awaits (canonical text and
  SHA-256, §2.6) run before the native call. A check only before entering the
  service would miss a basis change during them. The service is split:
  - `prepareReportPackageSave(controlled, model)` (async) returns the request
    text, its SHA-256 and the model text;
  - `invokeReportPackageSave(prepared, isCurrent)` then calls
    `invokeWithIngress("save_report_package", …, { stillCurrent: isCurrent })`.

  `invokeWithIngress` evaluates `stillCurrent` synchronously immediately before
  each `invoke`, including every headroom resend (`nativeIngress.ts:73-77`). The
  last guard therefore follows every preparation await with no await between it
  and the native invocation. A retired basis throws `IngressRequestRetiredError`,
  which the handler treats as stale. `isCurrent` is also still evaluated after
  the build and after the save returns, as today (`workspaceSession.ts:2609,2620`).
- **Invalidation under the same aggregate.** `handleRuleCheckAggregate` returns
  early when the aggregate is unchanged (`workspaceSession.ts:1059-1060`), before
  `invalidateReportPackageComputedState` (`:1066`, body `:1679-1685`). A changed
  pack or changed outcomes under the same aggregate would leave a stale package
  route. The run comparison moves above the early return:
  `if (nextRun !== previousRun) invalidateReportPackageComputedState();`. Presence
  equivalence alone cannot catch a mismatched pack or status. The checks are:
  - object identity here;
  - the id, status and aggregate equalities in TypeScript (above);
  - their Rust re-check (§2.3 item 7).
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
   local-store lookup. Both screens run over **both** parsed values: the request
   and the model.
   - **Generic recursive screen** (`screen_value_dispositions`), over the request
     `Value` (the report input, result document, source carrier, audit manifest
     and handoff records) and over the model `Value`. Every object at any depth
     is judged as the library import judges one: the hanger rule generalized,
     where own fields win, else its `provenance`'s, and a provenance-versus-own
     conflict judges the provenance alone. It blocks on:
     - `redistribution_status` `protected_suspected` or `rejected`;
     - `review_status` `quarantined` or `rejected`;
     - `privacy_classification` `protected_suspected`, matching the webview's
       rule (`redactionExportControls.ts:437-442`).

     Every `library_ref` it meets, in either value, is resolved against the
     store as W-6a resolves it. A package request has no `/project/id` (W-6a
     `model_document_save.rs:388`), so the resolver takes the project id from
     the validated model text's `/project/id`.
   - **Store-resolved model screen.** This is W-6a's
     `screen_model_document_dispositions` (W-6a `model_document_save.rs:382-447`)
     over the model: every `provenance_record`, every `rule_pack_refs[]` entry
     and every `library_ref`, with each referenced stored library and record
     judged by the import's own per-kind rule. The model screen is needed because
     the report input relabels component provenance as
     `private_only`/`pending`/`private_project_data`
     (`renderableReportInput.ts:177-193`), so no screen of the request alone can
     see a model record's own disposition. The generic screen is needed because
     W-6a's screen does not judge `privacy_classification` or the disposition
     members of arbitrary model objects (W-6a :230-240,335-374).
   - W-6a's model-save caller keeps its own policy unchanged.

   Refusals: a blocked disposition is `PROTECTED_CONTENT_BLOCKED`, naming the
   record and its path; a store read error is
   `REPORT-PACKAGE-SAVE-LIBRARY-STORE-UNAVAILABLE`; an unresolved reference is a
   receipt warning (`MODEL-LIBRARY-REF-UNRESOLVED`).
5. **Raw context and model-text bindings.**
   - `/report/report_context/export_context` is `local_private` and
     `/export_profile_id` is `desktop_local_private_report_package_1`.
   - The model text's `project.id` equals the request's actual model id (§2.3
     item 5).
   - `report_title` is `<project.name> — Calculation Report`.
   - `…unit_display_summary.model_units` equals the model's `project.units`
     (`renderableReportInput.ts:108-117`).
   - `report_context.software_name` is `app.package_info().name`, and
     `software_version` is `app.package_info().version.to_string()`. The
     compiled Tauri context takes these from `tauri.conf.json` `productName` and
     `version`, as the desktop builder does (`renderableReportInput.ts:16,379-380`).
     `admit_report_package` receives these expected values as arguments, so it
     stays testable without a window.
6. **Typed conversion and the shared producer** (§2.3), then `export_blocked`
   (`lib.rs:224-229`), then the picker and the atomic write (`:232-310`).
   Admission is a pure `admit_report_package(...)` function, so its tests need
   no window.

**Slice (a) hold.** The TypeScript gate is not an IPC boundary
(`reportPackageRequest.ts:27-30,218-221`). After (a), the shared producer
admits current requests, and the native command calls it directly
(`lib.rs:199-236`; `report_package_bridge.rs:3-18`). (a) therefore adds a
native hold, local to the desktop command:
- `save_report_package_decoded` first returns
  `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE: The report package is unavailable
  for fresh results.` with stage `assemble`. This happens before evidence use,
  package assembly, the picker or any write.
- Every request the 2.0.0 producer could admit is a current result, so the hold
  is unconditional.
- It does not close the shared producer or the headless runner.
- A command-path regression in `lib.rs`'s tests,
  `report_package_save_is_held_until_native_screening`, submits an otherwise
  valid 2.0.0 request (the derived fixture) with clean caller-supplied evidence
  through `save_report_package_args` and `save_report_package_decoded`. It
  asserts the code, that no picker was requested (a counting picker seam), and
  that the sandbox directory stays empty.
- **Removal:** (b1) deletes the hold and its test and installs the screened
  admission of items 1-6 above, with the TypeScript gate still closed until
  (b2).
- Merging (a) and (b1) atomically would be an equally sound alternative. With
  (a) landing first, the hold is required.

**Sharing with W-6a.**
- W-18's (b1) starts after #1588 (head `cb87128b69`) merges.
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

**The webview side.** `saveReportPackage`
(`reportPackageSaveService.ts:74-111`) becomes the two calls of §2.5:
- the preparation computes `requestCanonicalText = canonicalJsonString(controlled.payload)`
  and its SHA-256, asserts the controlled payload is canonically equal to the
  request, and produces `canonicalJsonString(model)`;
- the invocation then sends them under the session's `stillCurrent` guard.

### 2.7 The headless runner (shared producer consumer)

- **Request.** `export_results` carries the §2.3 request. The runner's
  local-private gates stay (`--explicit-local-private-intent`). It decodes the raw
  payload to `Value` once (`openpipestress-runner.rs:458`) and uses the shared
  `wire` path.
- **Runner bindings** (`validate_export_results_bindings`, `:583-689`), rewritten
  over the documents. The shared §2.3 checks run first.
  - **Model identity and request alias.** `source_model_ref` (type `model`) is
    the actual model identity, bound to `e.model_ref` (type `model_payload`) by
    §2.3 item 5. The runner's own `runner.model_ref` (the request reference, as
    supplied) must equal the origin's `request_model_ref` (type and id) when that
    is non-null; the headless producer always sets it
    (`result_envelope_binding.rs:271-272`). Only when it is null does it equal
    the actual identity. In the alias case there is no second equality to the
    actual identity: the alias is disclosed in `request_alias_disclosure` and
    accepted. The runner's former rules binding the request model to `source_model_ref` and
    to the audit model-hash payload ref (`openpipestress-runner.rs:605-617`) are
    replaced by this one, since both name the actual model.
  - `runner.input_manifest_ref.ref_id` equals `a.input_manifest_hash.payload_ref`.
  - `runner.unit_system_ref.ref_id` equals `a.unit_system_ref` and
    `e.unit_system_ref.ref_id`.
  - `runner.load_basis_refs` equals `e.load_basis_refs` as a set.
- **Fixture transition (decided, O-2).** A Rust generator test,
  `core/reporting/report_package/tests/witness_inputs.rs`, writes the inputs
  when an update variable is set and otherwise asserts them byte for byte.
  1. **Inputs.**
     - **The model.** The success model `M` is
       `core/product_physics/tests/fixtures/preview_physics_invented_model.json`
       (`project:invented-loop-01`, load cases `load:L-100`, `load:L-200`): the
       committed joint-free, pressure-free derivative of the original demo,
       which the headless tests already solve (`core/runner/headless/src/lib.rs:1139-1147`).
       `fixtures/product_preview/invented_preview_model.json` stays a refusal
       control: its legacy joint gives `MODEL_INCOMPLETE` and no document even
       without pressure (`core/runner/headless/tests/preview_physics_admission.rs:192-228`;
       `tests/test_headless_runner_contract.py:32-37`).
     - **The input manifest.** A committed invented input manifest,
       `fixtures/reports/invented/del1005_input_manifest.json`, names `M`.
     - **The request.** A runner request `Q` has:
       - `model_ref` `{model, project:invented-loop-01}`;
       - `input_manifest_ref` naming that manifest;
       - `unit_system_ref` `<model id>:units`;
       - `load_basis_refs` `M`'s two load cases;
       - the witness's invented, test-labelled provenance and boundary.
     - **The alias fixture.** It differs only in `Q.model_ref`
       (`{model, invented-model-alias}`) and repeats every later step from
       its own `Q`, so its request hash and origin stay coherent.
  2. **Solve once.** The generator solves `M` once through the real headless
     producer, `run_preview_model_value_with_rule_check(Q, {"model": M, "materials": []}, None)`
     (`core/runner/headless/src/lib.rs:721`). It keeps the mechanics carrier
     `S` and the producer-derived document `D` (`result_envelope_document`,
     `:685-695`). It asserts that `S` is `MECHANICS_SOLVED` and that `D` is
     present.

     The two run identities stay explicit:
     - `S.run_id` names the mechanics run;
     - `D.result_envelope.run_ref` names the headless analysis run,
       `run:headless-preview:<request id>` (`lib.rs:859`), which equals the
       origin's `request_run_ref` id (`result_envelope_binding.rs:272`).
  3. **Normalize `D` into a package base.** `D` is not yet a package fixture:
     - its `run_hashes` hold only the headless `result_envelope` checksum
       (`lib.rs:955-963`; copied at `result_envelope_binding.rs:333-383`), not
       the `analysis_run_record` and `received_result` pair that §2.3 item 5
       requires;
     - `derive_document` leaves `run_hashes` untouched (`derivative.rs:337-384`).

     The generator therefore:
     - builds an invented, explicitly test-labelled analysis-record payload for
       the package run, which records the mechanics run separately. It computes
       its `analysis_run_record` digest over the RFC 8785 bytes of
       `analysis_record_projection` (`core/analysis_runs/compatibility.py:73-80`),
       which removes the record's own `analysis_run_record` entry from
       `analysis_run.hashes`. That record shape and its self-hash exclusion rule
       are committed with the fixture, as
       `fixtures/reports/invented/del1005_analysis_record.json`, not inferred
       from a digest value;
     - computes `digest(S)`, `digest(row)` for every source row, and the input
       manifest's digest;
     - normalizes a clone of `D` as the package base:
       - the two package run-hash records carry exactly the references,
         algorithms, bases and values that §2.3 requires;
       - `audit_manifest_ref` names the generated audit member;
       - the statuses are exactly `MECHANICS_SOLVED`, `RULE_INPUTS_INCOMPLETE`
         and `HUMAN_REVIEW_REQUIRED`;
       - the solver stamp is the producer's;
     - leaves the producer's origin, the mechanics carrier and the model
       unchanged;
     - removes the inherited `derivative_hash` and calls `derive_document` on
       this normalized base with `M`, `S`, the original origin and `Q`. This
       creates the final package document, whose derivative hash is computed
       once.

     No field of that final document changes afterwards. This is an explicit
     invented fixture transformation of real producer output. It does not
     assert that the headless producer already emits desktop analysis-record
     hashes, and it changes no live result-export producer.
  4. **Matching members from the same basis.**
     - The audit manifest has:
       - `manifest_id` = the document's `audit_manifest_ref`;
       - `model_hash` = the document's value under `rfc8785_jcs`;
       - `input_manifest_hash` = the manifest's digest;
       - `unit_system_ref` = `Q`'s;
       - the linked solver.
     - The renderer input uses the §2.3 item 6 helper on `S`, plus the
       model-summary references, row digests, input-manifest evidence, statuses
       and solver stamp from the same basis, with `report_context`
       `local_private`.
     - The handoff record carries the same states, run, hashes and load basis.
     - `source_model_ref` and `source_basis_refs` come from the document.

     The generator checks the final request against every shared §2.3 binding
     before writing it.
  5. **Variants.** `generate_del1005_export_results_inputs.py` reads the
     Rust-written success input and derives the binding-mismatch,
     producer-blocked and missing-payload inputs from it by one edit each.
     Only the missing-payload input, which carries no document, is committed;
     the contract tests make the other two edits in memory, and the alias
     input is committed as its leaf replacements over the success input. The
     generated failure witnesses are regenerated in 2.0.0 form, with their
     codes unchanged. The generated success output, which no test reads and
     which embeds the whole package, is not committed; the determinism test
     and reproduction step C11 produce it.
  6. **Tests.** The success, mismatch, producer-blocked, missing-payload,
     determinism and zip-exact tests (`tests/test_headless_runner_contract.py:652`
     and its neighbours) exercise the fixture unchanged in what they assert.
     `export_input(…, linked_solver_identity)` no longer rewrites the solver
     identity. It asserts that the committed identity equals the linked one, so
     a product version bump fails until the generator is re-run. There is no
     live identity rewrite.

  What this witness is: versioned input. Python's `zipfile` and `hashlib` and
  repeated process output check archive encoding, integrity and determinism
  independently of the Rust producer. It is not an independent numerical or
  physical-validity witness and is not described as one.
- **Considered and not selected:** a named refusal of `export-results` in place
  of the transition.

### 2.8 Exact routes (W-11)

Nothing opens here. When the owner opens exact-route reports (W-11), the
package needs:
- the policy cell;
- the identity added to the admitted set (§2.3 item 0);
- W-11's rendered limitation sections.

The exact-route document already exists (`resultExportAdapter.ts:82-83,88`).

## 3. A shareable report path: the DEL-12-02 check (finding; option B decided, §7)

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

**The owner chose option B; W-18b designs it after W-6a and W-18a. The
alternatives below record the considered scope** (§7). This design builds none of
them.
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
  - **identity and order:** pressure-2, pressure-source-1, the retained
    identities, an unknown id and a missing id are each refused with
    `REPORT-PACKAGE-ROUTE-NOT-ADMITTED` by the preflight, before any form
    error. A document whose `semantic_contract_ref` differs from its producer's
    is refused. At the reader, a malformed member of a blocked package is
    refused, and an unblocked package with a non-admitted but well-formed
    member is refused;
  - **tamper:** a value edit with a recomputed derivative hash is refused by
    `validate_document`; a stale derivative hash is refused by the reader too;
  - **form, at producer and reader:**
    - the reviewer's header-only counterexample is refused;
    - each of these mutations is refused, with the derivative hash recomputed
      so only the form check can catch it:
      - `software_makes_compliance_claim: true`;
      - `human_review_required: false`;
      - `deterministic_ordering: false`;
      - a missing wrapper member (`export_format_status`, `objectives`);
      - a missing envelope member (`result_sets`, `row_accounting`, `run_ref`);
      - an unknown analysis status;
      - two rule tokens;
      - incomplete provenance;
    - the schema-keyword census test, over the transitive closure, with
      annotations separated;
  - **evaluator (`report_package::schema_eval`):**
    - a positive and a negative case for each assertion and applicator keyword
      in the closure, including `contains`, `dependentRequired`,
      `if`/`then`/`else`, `not` and `uniqueItems`;
    - `required` without `properties`;
    - an unconstrained object, and `RawSourceAnnotationMetadata` with arbitrary
      metadata, accepted;
    - an unbounded array longer than 16384, accepted;
    - fragment and whole-resource references resolved, and an unknown
      reference refused;
    - the existing `source_blocks` consumers unchanged;
    - the Python Draft 2020-12 oracle parity
      (`tests/test_schema_eval_parity.py`) over the same vectors;
  - **cross-member substitution:** one test per §2.3 item 5 binding, including
    the model-summary `model_ref` and `project_ref` and the `solver_build_ref`.
    Also: a source carrier from another run, an audit manifest from another
    model, a swapped `manifest_id`, a hash record with the right value but the
    wrong reference or basis, and a handoff record with another run's or
    model's hashes;
  - **run, state and unit policy:** each of these, changed alone, is refused:
    - a coherent relabelling of every package run reference on the desktop
      profile while `source.run_id` stays unchanged;
    - on the runner profile, `e.run_ref` not equal to `request_run_ref`;
    - each current state reference and section id, and a duplicated or
      missing current section;
    - `conversion_performed: true`, `storage_convention` and
      `quantity_display_policy`;
    - the model-summary `unit_system_ref`, and the desktop `e.unit_system_ref`;
  - **row parity:** the TypeScript and Rust row helpers both match
    `report_result_rows_parity.json`;
  - **displayed content:** one `result_rows[].quantity_display` replaced by an
    unrelated safe value, and a row reordered, dropped or added, are refused;
    `result_export_refs` with a wrong row checksum is refused; a comparison
    section's own left and right runs are accepted;
  - **runner alias:** the alias fixture is admitted, with the request alias
    bound to `request_model_ref` and the actual model bound to the audit; a
    request model matching neither is refused;
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
  - **model-only content:** a hash-bound model carrying, with every other status
    clear:
    - a `protected_suspected` `privacy_classification` in a component's
      provenance record;
    - a direct or nested disposition member on an arbitrary model object;
    - a quarantined `provenance_record`.

    Each is refused, although the report input relabels that provenance;
  - **request-only library reference:** a `library_ref` that occurs only in the
    request is resolved with the model's project id. It is refused when the
    stored record is quarantined or the store fails;
  - evidence for other bytes; a non-canonical text; a model text not matching
    the audit hash; a model whose library record is quarantined in the store;
    a store read error;
  - a model text whose `project.id`, name or units differ from the request's,
    and a forged `software_name` or `software_version`;
  - **slice (a) hold:** `report_package_save_is_held_until_native_screening`,
    as specified in §2.6; it exists in (a) only and is replaced in (b1);
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
    during the awaited build, or during the service's canonicalization and hash
    awaits, publishes nothing and never calls the native command. The guard is
    asserted to run in `invokeWithIngress`'s `stillCurrent`, including on a
    headroom resend;
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

## 5. Reconciliation, per slice

Each text is changed in the PR of the slice that makes it true, so every merge
head stays truthful. Commitment and acceptance texts are drafted exactly in
Appendix A for the manager's wording route; this design does not apply them.
Where one paragraph spans slices, the earlier slice leaves it unchanged if it
stays true, or makes only the factual change its own work establishes.

- **Slice (a): producer, reader, version and checksum bases.**
  - A4, DEL-02-05 ScopeOfWork :214: the reader's 2.0.0 version and the
    result-member form check. A4's "DEL-08-04 0.3.0 schema" is applied only
    with the full-schema evaluator and its oracle parity in the same PR.
  - A6, DEL-04-08 AC-006 :240: the Rust boundary refuses every identity but
    preview-physics-2.
  - A7, DEL-04-08 VER-006 :311: the Rust identity test. The named test
    `current_result_package` and its `tools/hosted-ci-routing.json`
    registration are in the same PR.
  - DEL-08-02 ScopeOfWork :116: (a) adds only the basis fact: "`validate_manifest`
    accepts `rfc8785_jcs` and `rule_pack_payload_jcs` in their defined slots
    alongside `project_local_deterministic_json`." Its desktop-refusal sentence
    stays true until (b2), which applies the final A3. (a) does not reword A3,
    which is reviewed exact text.
  - REQ-006 (c) :219 stays as it is: the desktop still refuses with the
    fresh-result code, and the added Rust refusal is covered by A6.
  - DEL-08-02 and DEL-08-04 `deliverable.yaml` code and check pointers (the
    bases; the form validator).
  - CI routing (`tools/hosted-ci-routing.json`) for the reporting and runner
    sources and tests (~:2494-2496 and the `report_package` tests list), the new
    `schema_eval` module, the fixtures and vectors, the schema-parity and
    row-parity tests, and changed manifest digests (e.g. `:290`).
  - The native hold (§2.6) needs no text change: every desktop text still says
    the package is refused.
  - `docs/COMPLETENESS_MAP.md`: the W-18 row (:543) records (a) only.
- **Slice (b1): native route declarations.**
  - The `DREP-PACKAGE-SAVE-009` declarations:
    - its route tables `core/security/local_first_storage/route_control.py:28`
      and `redactionExportControls.ts:145`, where the evidence form changes;
    - the package-save entry in `apps/desktop/SMOKE.md` (:9881);
    - DEL-12-02 text naming the native package checks.
  - CI routing for the Tauri sources (~:2357-2372, `export_disposition_screen.rs`).
  - The (a) hold and its test are removed; the desktop gate stays closed in (b1).
- **Slice (b2): desktop opening.**
  - A1, DEL-08-01 ScopeOfWork :81; `deliverable.yaml` pointers.
  - A2, DEL-08-06 ScopeOfWork :118.
  - A3, DEL-08-02 ScopeOfWork :116 (final).
  - A5, DEL-04-08 REQ-006 (c) :219.
  - DEL-04-07 Design `desktop-exact-outputs.md` §4.5 (:212): the report package
    refuses exact identities by policy and Rust identity, not by the
    fresh-result code.
  - User guide `docs/user_guide/index.md`:
    - :109-111 — the bullet becomes "Save Report Package… saves a local
      private package for a pressure-free result after its own local-private
      selection; exact-route results are refused with
      `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`.";
    - :148 — "and the report package refuses every current result" becomes
      "and the report package is saved locally for a pressure-free result";
    - :393-395 — the same clause change.
  - `docs/COMPLETENESS_MAP.md` :97, :543, :586.
  - (b2) opens the desktop gate only with (b1) in its ancestry.
- **W-18b (option B):** the DEL-12-02 classification, in its own design.

## 6. Implementation slices and size

| Slice | Content | Size | Persistence / data boundary |
|---|---|---|---|
| **(a) Rust 2.0.0 transition** | `wire` request 2.0.0, `bindings`, the 0.3.0 form validator, assembler input, reader member checks, version 2.0.0; `audit_manifest` bases; headless runner request, bindings and the generated witness. Every direct consumer of the old types moves with it, not only the bridge: the Tauri bridge and ingress pricing (`command_ingress.rs:552-568,976-1181`), and the `report_package` test helpers and reader tests (e.g. `tests/common/mod.rs:228-251`, `tests/reader.rs:211-226`). The repaired schema evaluator (`schema_eval`) with its vectors and Python oracle parity; the row-parity helper and vectors; the native hold in `save_report_package` with its test (§2.6). The desktop stays closed, by the TypeScript gate and the native hold. Its §5 texts | L-XL (~2.5k lines with tests, plus regenerated witnesses and vectors) | **Persistence:** the `.opsproj` format and its reader. Numerical meaning untouched |
| **(b1) Native admission and screening** | Canonical-text command, evidence binding, shared `export_disposition_screen.rs` extracted from W-6a, both screens over request and model, model-text bindings, `admit_report_package`, forged-evidence, model-only and read-back tests. It replaces the (a) hold. Starts after #1588 merges. Its §5 texts | M-L (~1.1k) | **Data boundary and file output** |
| **(b2) Desktop wiring** | `reportPackageRequest.ts` rewrite using the extracted `reportResultRows` helper, gate and menu, rule-run threading and guards, the split save service, retired constants, vitest, native smoke step. Its §5 texts | M-L (~1k) | Data boundary (webview evidence) |

(b1) and (b2) may land as one PR if review prefers. The order is (a), then
(b1), then (b2), which opens the desktop route.

## 7. Decided items and the wording route

- **O-1, decided by the owner (2026-10-24):** "I accept your recommendations for
  each of these three: G10, W-12, W-18." This is option B, a user release of the
  user's own project data, in v0.1, sequenced after W-6a and this package
  (W-18a).
  - The release is recorded **per report** (HELP_HUMAN, 2026-10-24, option (1)).
    It is made at report time by an explicit selection and the user's statement
    of the redistribution right (`docs/IP_AND_DATA_BOUNDARY.md` §6).
  - It is recorded in that report's `report_context` and the package manifest,
    and bound to the model hash and result-set identity the report covers.
  - The selection does not persist: it is not stored in the session, the project
    or the model document, and the model document does not change. The per-report
    release record is kept in that report's output. A report without the
    selection stays local-private exactly as described here.
  - The design is W-18b, in its own section or design after this one. It needs
    the DEL-12-02 classification (wording route), targeted regression checks
    and Codex scrutiny. This design does not build it.
- **O-2, decided (manager): the fixture transition** (§2.7). It keeps the
  determinism and zip-exact witnesses, and no existing success expectation
  becomes a refusal.
- **O-3, decided (manager): wait for #1588.** Slice (b1) builds on W-6a's merged
  screening module; nothing is copied.
- **O-4: the wording route.** Appendix A's A1–A7 old → new texts are reviewed by
  Codex as exact text and go to HELP_HUMAN before they are applied. Each is
  applied in the PR of the slice that makes it true (§5). Codex round 2 approved
  A1, A2, A5 and A6 as written. A3, A4 and A7 carry its exact corrections; A4
  depends on the §2.3 form validator.

## Appendix A. Wording-route drafts (old → new; not applied)

**A1. DEL-08-01 ScopeOfWork :81** (current-implementation paragraph, last clause)
- Old: "and Save Report Package (`reportPackageUnavailableReason` in `reportPackageRequest.ts`, then `save_report_package` through `apps/desktop/src-tauri/src/report_package_bridge.rs`) refuses every current result with `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`."
- New: "and Save Report Package (`buildReportPackageRequest` in `reportPackageRequest.ts`, then `save_report_package` in `apps/desktop/src-tauri/src/lib.rs` and the shared producer `core/reporting/report_package/src/wire.rs`) saves a version 2.0.0 package for a Current pressure-free (preview-physics-2) result under `local_private` with the package's own explicit intent: its result member is the DEL-08-04 0.3.0 document that Export Result JSON saves, its rendered members carry the rule-pack references and user-rule outcomes, and the native command screens dispositions from content before the save dialog. Every other identity is refused (exact routes with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; owner ruling 10A, 2026-10-21)."

**A2. DEL-08-06 ScopeOfWork :118**
- Old: "Desktop report-package request (current, after B7b). `apps/desktop/src/features/report/reportPackageRequest.ts` assembles the state/run and comparison sections (`stateComparisonHandoffSections.ts`, kept equal to the Python assembler `core/reporting/state_comparison_handoff_sections/engine.py` by a shared parity fixture) only below its availability gate. The gate refuses preview-physics-2, its retained successor, pressure-2 and pressure-source-1 with T0R's `REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE` (the exact identities since epoch 8; DEL-04-07 Design `desktop-exact-outputs.md` §4.5). Below the gate, which only its test seam reaches directly, assembly accepts only a preview-physics-2 source with a 0.3.0 analysis record, so no current result yields a desktop report package. B7b removed the physics-1 and source-blocks reasons with their retired identities."
- New: "Desktop report-package request (current, after W-18). `apps/desktop/src/features/report/reportPackageRequest.ts` assembles the state/run and comparison sections (`stateComparisonHandoffSections.ts`, kept equal to the Python assembler `core/reporting/state_comparison_handoff_sections/engine.py` by a shared parity fixture) for a Current preview-physics-2 result. The output policy's `report-package` cell refuses its retained successor and, under owner ruling 10A, pressure-2 and pressure-source-1; the package's Rust boundary admits only preview-physics-2 (DEL-08-01 Design `report-package-current-results.md`)."

**A3. DEL-08-02 ScopeOfWork :116** (current-implementation paragraph, second sentence)
- Old: "On the desktop, `apps/desktop/src/features/report/reportPackageRequest.ts` assembles the audit manifest, model hash and input-manifest hash for the Save Report Package request (`save_report_package` through `apps/desktop/src-tauri/src/report_package_bridge.rs`); that request is refused for every current result (`REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`)."
- New: "On the desktop, `apps/desktop/src/features/report/reportPackageRequest.ts` assembles the audit manifest for the Save Report Package request (`save_report_package` in `apps/desktop/src-tauri/src/lib.rs`) for a Current pressure-free result. The manifest is identified by the run's input-manifest reference and records the model and input-manifest hashes under `rfc8785_jcs`. When a rule-check run is present, it carries a supported declared `sha256`/`JCS` rule-pack checksum as `rule_pack_payload_jcs`, without recomputing it, or the loaded document's SHA-256 digest as `rfc8785_jcs`; unsupported checksum bases refuse the package. `validate_manifest` accepts those bases in their defined slots alongside `project_local_deterministic_json`."

**A4. DEL-02-05 ScopeOfWork :214**
- Old: "(`schema_version 1.0.0` today)"
- New: "(`schema_version 2.0.0` today)"
- Old: "It then verifies the archive form, the manifest against the version 1.0.0 form and every member against the manifest inventory and hashes;"
- New: "It then verifies the archive form, the manifest against the version 2.0.0 form and every member against the manifest inventory and hashes; for each result member it also checks the DEL-08-04 0.3.0 schema, transported semantic-contract header and derivative hash, and, for an unblocked package, the admitted identity plus equality of the audit-manifest id and model-hash value;"

**A5. DEL-04-08 ScopeOfWork :219** (REQ-006 (c), first bullet)
- Old: "and the report package refuses it as it refuses every fresh identity (`REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE`)."
- New: "and the report package refuses it with the same code at the desktop output policy and with `REPORT-PACKAGE-ROUTE-NOT-ADMITTED` at its Rust boundary (owner ruling 10A)."

**A6. DEL-04-08 ScopeOfWork :240** (AC-006)
- Old: "The desktop output policy refuses each named identity with its stated code."
- New: "The desktop output policy refuses each named identity with its stated code, and the report package's Rust boundary refuses every identity except preview-physics-2."

**A7. DEL-04-08 ScopeOfWork :311** (VER-006, `desktop-test` bullet), and one bullet added after it
- Old: "`apps/desktop/src/features/report/reportPackageRequest.test.ts` and `apps/desktop/src/services/ruleCheckService.test.ts`;"
- New: "`apps/desktop/src/features/report/reportPackageRequest.test.ts` and `apps/desktop/src/services/ruleCheckService.test.ts`;" (unchanged), followed by this added line, with two leading spaces to match its sibling bullets:
  - the report package's identity refusals: `core/reporting/report_package/tests/current_result_package.rs`, exercised by `cargo test --manifest-path core/reporting/report_package/Cargo.toml --test current_result_package` and registered in `tools/hosted-ci-routing.json`;
