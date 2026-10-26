# Exact-route report sections and report provenance (W-11)

**Basis:** `origin/main` at `25b7df82ae` (2026-10-25), after W-18a (a), (b1)
and (b2) (#1597, #1606, #1616), W-7 (#1581), #1613, #1621 and the W-12 merges
through #1624. Revision 5, design before implementation, on the same basis
(re-checked at `1bee2881cd`). Revision 4 (at `1bee2881cd`) had a third Codex
round, which found its source binding sound and reported two consistency
defects. This revision resolves them:
- the admitted-set variants get an explicit forwarding graph and a scan that
  verifies it (§5.5);
- before opening, the native render refuses exact identities at the native
  boundary (§3.6 step 0), so no production path reaches an exact output.

It also records that round's view of the render's working-memory provision
(§3.6). It changes §1.2, §3.1, §3.6, §5.1, §5.5, §6-§8 and Appendix A: the
G2 and G5 News, the O4 Old, and the O13 Old and New. Revision 4 resolved the
second Codex round (at `88073b7968`): the native boundary (§3.6), the runner
and test-seam findings (§3.5, §5.5) and the W-17 rule-outcome constraint
(§1.7). Revision 2 (at `2c37cfa09e`) had the first Codex round,
which revision 3 resolved. Revision 1 (at `67e9e43cc5`, 2026-10-24) was
never reviewed; §1.1 lists what re-verification corrected. Citations are at the basis head and are relative to
`projects/chirality-piping/`. W-18 (DEL-08-01 Design
`report-package-current-results.md`) is the package's own design; this design
adds what the exact routes need on top of it, and FR-REP-003 provenance on
every route. Standard claim fence applies (claims taxonomy per DEC-081).

## 0. Rulings and requirements

- **Ruling 10A (owner, 2026-10-21):** reports of exact-route results
  (`pressure-2`, `pressure-source-1`) stay refused until DEL-08-01 designs
  their limitation sections. DEL-04-07 Design `desktop-exact-outputs.md` §4.5
  (:211): "Opening them needs DEL-08-01's own design of those sections and of
  the PRD §19.3 notice's place." Opening returns to HELP_HUMAN and the owner
  (completeness map, W-11 row :541). **This design builds the sections behind
  the existing refusals. It opens nothing.** §5 states what opening would
  change.
- **Opening ruling (owner, 2026-10-26, through HELP_HUMAN):** "My two
  decisions are to approve your recommendations in both cases. Proceed
  accordingly." The approved recommendation opens both surfaces, the rendered
  report and the report package, for `pressure-2` and `pressure-source-1`,
  exactly as §5.1 lists them, once slices (b) and (c) are merged and the
  owner has seen the evidence of §5.5 (the rendered local-private example
  reports of each exact-route shape). §6.3 records the opening as built. The
  report JSON packet, the handoff preview and the other fifteen surfaces stay
  refused (§5.4).
- **FR-REP-001 (Must)**, PRD §19.2: inputs, versions, states, runs,
  assumptions, warnings, results and rule-pack references. **FR-REP-002:**
  mechanics, user-rule checks and human review kept distinct.
- **FR-REP-003 (Must):** "Report identifies sources or source notes for
  user-supplied data where available." PRD §20.2 names the seven fields of a
  provenance record (source name, location, license, contributor, contributor
  certification, redistribution status, review status).
- **Claims registry** (`docs/claims_registry.md` §1 BS-ACCEPT, §4): BS-ACCEPT
  text is on no product surface or report body (DEC-100, DEC-107). Where it is
  withdrawn, nothing is written in its place. Deliverable-specific exclusions
  are scope statements and stay ordinary prose. Structured metadata fields are
  data contracts, not prose. The PRD §19.3 notice is the report's composite
  anchor (§3) and is unchanged.
- **DEC-028:** one package version; the reader refuses others by name.
- **Ruling on #1639 (owner, 2026-10-26):** "C1 b, C2 a". C1 (b): in a
  local-private report only, a provenance record whose redistribution, review
  or privacy status is `unknown` or `TBD` renders with a non-blocking warning
  in the warnings section that names the record and the unresolved field and
  says the source's status is not recorded; in every other context (a public
  report, an absent context, and any later shared context such as W-18b's
  user-released report) it stays report-blocking. C2 (a): text the user
  authored in a record, a path typed in `source_location` included, stays as
  written in local-private outputs; no other context carries these records.

## 1. Where things stand

### 1.1 Corrections from revision 1

1. **The producer composes more than `exact_admission.rs`.** Revision 1 listed
   the strings of `pressure_v3_formulation_basis_with_source`
   (`core/product_physics/src/exact_admission.rs:72-115`). The published list
   is composed by `formulation_basis_for_model_with_source`
   (`core/product_physics/src/lib.rs:1186-1210`), which also pushes the
   realized-bend limitation (`bend_pressure.rs:35`, pushed at `lib.rs:1198-1200`)
   and appends two certified-recovery statements on every route
   (`lib.rs:1205-1208`). There are 19 distinct exact-route strings, not 16
   (§1.3).
2. **The fixture writer.** Writing the coverage fixture through
   `pressure_v3_formulation_basis_with_source` would miss those three strings.
   §2.2 generates the fixture from the public producer itself, on invented
   models.
3. **The renderer input.** `RenderableReportInput` is
   `report_renderer::RenderableReportInput`
   (`core/reporting/report_renderer/src/lib.rs:154-163`), not
   `report_generator`'s. `schemas/report_generator.schema.yaml` describes the
   generator document (`calculation_report`), not the renderer input; no schema
   describes `result_rows`, `report_context` or `user_rule_checks`. The struct
   has no `deny_unknown_fields`, so a field the Rust side does not know is
   dropped silently when the package deserializes the report
   (`report_package/src/wire.rs:286`). New fields therefore land in Rust first
   (slice (a), §6).
4. **The rendered report's refusal is route level.** `RenderedReportPanel.tsx:171`
   calls `loadReferenceOutputRefusal`, which returns the route's shared refusal
   whatever the surface (`loadReferenceOutputAvailability.ts:34-36`,
   `outputPolicy.ts:140-145`). Changing the policy cell alone would not open the
   render. Slice (b) moves the check to the surface level, which refuses
   identically while the cell refuses (§3.1).
5. **The package is open for pressure-free results and has three refusal
   points for exact identities** (§5.1): the output policy's `report-package`
   cell, the TypeScript identity check (`reportPackageRequest.ts:181-185`) and
   the Rust identity boundary (`report_package/src/document_form.rs:17,45-51`,
   `REPORT-PACKAGE-ROUTE-NOT-ADMITTED`). Revision 1 named only the cell and
   "the Rust identity list".
6. **Result rows are a cross-language contract.** Since W-18 (b2) the report's
   rows are `reportResultRows` (`features/report/reportResultRows.ts:19-27`),
   which the package's Rust producer re-derives
   (`report_package/src/bindings.rs:103-140`, checked at item 6, `:448-452`)
   against shared vectors (`fixtures/reports/invented/report_result_rows_parity.json`).
   A label suffix changes both sides and the vectors together (§3.4).
7. **Standing has no registered display form.** `numericalResultStanding`
   returns `{status, eligible, findings}` (`results/resultStanding.ts:26`). The
   session shows the label "integrity checked"
   (`solve/LoadReferenceStatesBlock.tsx:16`). The rendered report today checks
   no standing; the package's admission is `buildCurrentResultExport`
   (`reportPackageRequest.ts:179`; `result-export/resultExportAdapter.ts:252-295`).
8. **The load/reference disclosure is UI text only today.**
   `results/loadReferenceDisclosure.tsx:1-9` says its lines are never written
   into a document. The report needs its own template (§3.2).
9. **The pressure-free headline is dropped.** Revision 1 added the
   `max_open_formula_stress` headline to pressure-free reports. That changes
   the open package's rendered members for no W-11 requirement, so it is not in
   this design.
10. **Line numbers moved:** the limitation block is
    `renderableReportInput.ts:548-563`, the provenance notes `:538-542`, the
    component record `:178-194`; the per-basis headlines are
    `ResultsPanel.tsx:227-261`; the exact-route limitation check is
    `semantic_contract.rs:601-614`; `selectedResultRefs` is `ReportPanel.tsx:297-305`.
11. **The governance token** is the current GF-TOKEN form (registry §1).

### 1.2 Refusals today

| Surface | Where it refuses an exact result | Code |
|---|---|---|
| Rendered report | `outputPolicy.ts:85-88` (`EXACT_ADMITTED_SURFACES` = result-export, stress-neutral, rule-check), read by `LoadReferenceOutputGate` (`RenderedReportPanel.tsx:220`) and by the route-level check at `:171`. The native render (`src-tauri/src/lib.rs:1540-1563`) is route-blind today: its input names no source identity (`report_renderer/src/lib.rs:154-163`), and its only production caller is the panel, after that check (`RenderedReportPanel.tsx:171,197`) | `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` |
| Report package | the `report-package` cell (`reportPackageRequest.ts:60-62,168`; menu `App.tsx:233,373,760`); the TS identity check (`:181-185`); the Rust boundary (`document_form.rs:17,45-51`), called at `wire.rs:282`, `bindings.rs:194`, `lib.rs:241` and the reader `reader.rs:930` | the same code; then `REPORT-PACKAGE-ROUTE-NOT-ADMITTED` |
| Report JSON packet (`report`) | the cell, through `ReportPanel.tsx:107` | `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` |

The refusal texts list the admitting surfaces (`outputPolicy.ts:43,48`). The
native smoke pins the exact-route refusals (`apps/desktop/native-smoke/run.mjs:277-283`).

### 1.3 What the exact producers publish

`formulation_basis.limitations` are whole static strings, composed in this
order (`lib.rs:1186-1210`):

1. the identity sentence: `pressure-2` (`exact_admission.rs:85`) or
   `pressure-source-1` (`:83`);
2. six shared statements (`:94-99`): pressure, "Not modelled: …", contribution
   ledgers, line loads, support attribution, the normal-stress maximum's
   enclosure;
3. the source paragraph: `pressure-2` (`:90`) or `pressure-source-1` (`:88`);
4. five load/reference-state statements when the model is a load-state model
   (`:104-108`);
5. the connector limitation when a component carries an objective connector
   (`joint.rs:164`, pushed at `exact_admission.rs:112`);
6. the realized-bend limitation when a component is a curved macro element
   (`bend_pressure.rs:35`, pushed at `lib.rs:1199`);
7. two certified-recovery statements on every route (`lib.rs:1206-1207`).

Distinct exact-route strings: 2 + 6 + 2 + 5 + 1 + 1 + 2 = **19**. A list has 10
to 17 entries. The committed corpora hold eight of the shapes
(`fixtures/results/pressure_v3_straight_reader_corpus.json`, `…_arc_…`,
`…_connector_…`, and the `pressure_source_load_state_*.document.json`
documents); each published list matches this composition exactly (checked for
this revision).

**The acceptance-boundary sentence remains.** Both source paragraphs still end
with "No code compliance or professional acceptance is produced, and actual
numerical admission remains separate." (`exact_admission.rs:88,90`). No other
exact-route string contains "acceptance". The only non-ASCII character in the
19 strings is "é" (in "Lamé"), which the PDF emitter's WinAnsi encoding carries
(`pdf_emitter/src/lib.rs:620-656`).

### 1.4 How they are bound

- For the exact routes the reader checks only that the limitations are
  non-empty strings under the pressure profile
  (`result_export/src/semantic_contract.rs:601-614`). Preview-physics-2's are
  bound exactly to its table (`preview_physics_evidence.rs:269`). The report's
  closed statement table (§2.1, §3.5) binds the exact strings one by one, in
  TypeScript and in Rust.
- The tables' `supported_profile_limitations` (10 strings for pressure-2, 7 for
  pressure-source-1; `fixtures/results/semantic_contract_v0_3_pressure_2.json`,
  `…_pressure_source_1.json`) carry similar sentences ("Source evidence and
  numerical standing do not authenticate a producer or grant engineering
  acceptance."; "… never replace actual invocation custody or responsible
  engineering review."). They travel by reference, as they already do for the
  exports.

### 1.5 Headlines and rows

- An exact result publishes `max_open_formula_stress_nominal_section` and
  `…_reduced_section` (`types.ts:639-644`); `max_open_formula_stress` is null
  (no governing basis is chosen). The session shows them with the labels
  "Nominal section (nominal wall)" and "Reduced section (wall minus mill
  tolerance and corrosion allowance)" (`ResultsPanel.tsx:227-230`), the
  withheld text at `:250` and the sentence "Each section basis has its own
  maximum; no governing basis is chosen." (`:259`). The report has no headline
  section.
- Exact stress rows name their basis in `metadata.basis`, as
  `…; section_basis=nominal_section` or `…; section_basis=reduced_section`, and
  the reader requires that basis to equal the table signature's `source_basis`
  where declared (`source_basis_signature_policy`). Today's row label is
  `kind (component)`, so the two bases' rows have the same label.
- **W-7 (#1581)** added `element_global_end_force_v1` and
  `element_global_end_moment_v1` rows to pressure-2 and preview-physics-2 (the
  pressure-source-1 table has none). Their basis
  (`rotated_from_published_element_local_end_actions_v1;axial_is_pipe_wall_action`)
  names no section basis, they are not stress rows, and the reader classifies
  them. They take no suffix and change no headline.

### 1.6 Provenance

- `provenance_notes` holds the session, component and spring-hanger records
  (`renderableReportInput.ts:538-542`). Materials, sections, supports' records
  and library references are absent (completeness map :204).
- Model 0.7.0 gives materials, sections, components and supports an optional
  `provenance_record` with the seven §20.2 fields, `reviewed_by`, `reviewed_on`,
  `verification_note` and an optional `library_ref`
  `{library_kind, library_id, library_version?, record_id?}` (`types.ts:15-27`;
  DEL-02-01 Design `model-document-0.7.0.md` §3.5). Its `review_status` is
  source-data review only (`types.ts:12-14`).
- W-13 S2b (#1566) sets `library_ref` when a library record is applied. #1595
  made the open-time store check follow the save screen's walk: every
  `library_ref` at any depth, owned by its nearest `id`
  (`services/libraryImportService.ts:440-469`). Findings
  `MODEL-LIBRARY-REF-UNRESOLVED` and `MODEL-LIBRARY-REF-VERSION-CHANGED`
  (`:410-411`) are published to a session store
  (`features/library/libraryRefFindings.ts:20-45`) and shown only while the
  entity still carries the reference they were raised on.

### 1.7 Other merges checked

- **#1613** projects the run record's rule-pack exclusion diagnostics into the
  result export (`resultExportAdapter.ts:63-67`). An open exact package's
  result member would carry them. The rendered report reads
  `result.diagnostics` only (`renderableReportInput.ts:139-160`), on every
  route; see §9.
- **#1621** prices the package request by regions (DEL-04-07 Design
  `memory-budget-decision.md` :43-45). W-11 adds no request field: everything
  it adds is inside `report`, an opaque `Value` region, priced in C_V. The
  region table is unchanged. The committed witness request grows in slice (c),
  so that design's measured figures (:49-52) are re-measured there.
- **W-12 (#1589 … #1624)** changed the package's state/run and comparison
  sections (DEL-08-06), which are route-independent. No W-11 section depends
  on them. The handoff state reference is `state:<model>:preview` on every
  route (`stateComparisonHandoffSections.ts:109`, required by
  `bindings.rs:327`); it is an identifier, not a route statement. Within-run
  comparison pairs rows by id, so each basis pairs with itself.
- **W-17 (planned; HELP_HUMAN constraint).** Rule-pack 0.5.0 makes rule-check
  outcomes per (check, load case or combination). A W-17 slice will add each
  outcome's load case to `userRuleChecks` (`renderableReportInput.ts`) and to
  `UserRuleCheckOutcome` (`report_renderer/src/lib.rs:135`), pending the
  owner's O-A ruling on the package version under DEC-028. **No W-11 section,
  check, test, fixture or parity vector reads the shape or count of the
  rule-check outcomes**: item 9 and the exact-section vectors read no
  `user_rule_checks`, item 7 (`bindings.rs:454-490`) is unchanged and reads the
  aggregate and the pack reference only, and the W-11 witness inputs carry no
  rule-check run (§5.5). Slice (c) regenerates
  `validation/witness/inputs/del1005_export_results_success_input.json`, which
  does carry W-18's rule-check run. It regenerates that file in whatever shape
  main has, so W-17 and (c) may land in either order, and the later one
  regenerates it.

## 2. Exact-route limitation sections

### 2.1 A closed statement table

`features/report/exactReportLimitations.ts` holds a frozen table of 19 entries:
`{ producer_sha256, statement }`.
- **Matched exactly.** A producer string matches the entry whose
  `producer_sha256` is the SHA-256 of its UTF-8 bytes. Keying by digest keeps
  the omitted sentence out of desktop product source; the producer text lives
  in the producer and in the fixture.
- **Statements are copied verbatim**, except the two source paragraphs, which
  omit their last sentence (§2.3, DEC-107). Nothing replaces it. Numerical
  standing is stated by its own line (§3.2).
- **One literal per statement.** GEN-13 does not reassemble sentences split
  across concatenated literals (`tools/validation/validate_claims_language.py`
  docstring), so each statement is one string literal.
- **Fail closed, in both languages.** A result whose producer limitations
  include a string the table lacks is refused for every report surface with
  `REPORT-EXACT-LIMITATION-UNREGISTERED: <identity> publishes a limitation this
  report has no reviewed statement for.` Rust applies the same mapping (§3.5):
  the table test writes the table as `fixtures/reports/exact_report_statements.json`,
  and `report_renderer` embeds that file. No Rust path copies a producer string
  or strips a suffix. A producer text change therefore needs a reviewed table
  change before it reaches a report on either side.

### 2.2 The coverage fixture and its writer

- **Producer, not a seam.** No product source changes: `lib.rs` and
  `exact_admission.rs`, whose bytes retained pricing pins, are unchanged. The
  fixture is generated from the public producer the desktop and the runner use
  (`run_linear_static_preview_value_with_mode`), on invented models, so it holds
  exactly what the producer publishes.
- **Writer.** An integration test, `core/product_physics/tests/exact_route_limitations.rs`
  `exact_route_limitations_fixture_is_current`, runs one invented model for each
  shape the producer emits, 10 lists: `pressure-2` × `load_state` ×
  `connectors` × `realized_bends` (8), and `pressure-source-1` with neither a
  connector nor a bend, without and with the load/reference-state group (2).
  The models are committed invented requests, or copies of them patched inline
  with an added connector tail. Each envelope must state its intended shape:
  its identity, its load/reference-state evidence, its connector records and
  its curved-bend count. The test writes, in this order,
  `fixtures/reports/exact_route_limitations.json`:
  `{schema_version: "1.0.0", generator: "<test path>::exact_route_limitations_fixture_is_current",
  lists: [{semantic_contract_id, load_reference_states, objective_connectors,
  realized_bends, limitations}], strings: [<the 19 distinct strings, sorted by
  code point>]}`. The 10 lists carry all 19 strings.
- **Six shapes are never emitted.** The producer does not emit
  `pressure-source-1` for a model with a connector or a realized bend. Source
  recovery declines each by name (`source_recovery.rs:807-814`, with the
  component-family refusal at `:874-880` behind them), and the case publishes on
  the ordinary `pressure-2` route (`lib.rs:3198-3214`). The fixture therefore
  has no such list (revision 5 listed 16, six of them unreachable). The guard
  `pressure_source_1_is_never_emitted_with_a_connector_or_a_realized_bend` (same
  file) adds a connector, a bend or both to the retained-source control, without
  and with the load/reference-state group. Each of the six asks for source
  recovery. The test asserts the named decline, a `pressure-2` envelope with
  that shape's fixture list, no `pressure-source-1` string, and, on a load-state
  case, the not-joined diagnostic. If the producer ever emits such a list, the
  guard fails, and the fixture and the table must grow.
- **Stale fails.** The test compares its output with the committed bytes and
  fails on any difference. `OPENPIPESTRESS_WRITE_EXACT_ROUTE_LIMITATIONS=<path>`
  writes the file instead, as the recorded-run witness does
  (`apps/desktop/src-tauri/src/analysis_runs_tests.rs:936`).
- **Against real envelopes.** The same test checks that every published list in
  the committed exact corpora (§1.3) equals one fixture list.
- **Recovery once.** The same file has `certified_recovery_statements_enter_once`.
  The recovery pair is the last two statements of a produced list. For each of
  the 10 produced lists, each committed exact corpus list, a committed
  preview-physics-2 envelope and the producer's run of a pressure-free model, it
  checks that each recovery statement occurs exactly once and that the pair is
  last. The public `preview_formulation_basis()` carries neither statement.
- **Table test.** `features/report/exactReportLimitations.test.ts` reads the
  fixture and checks that the table's digests are exactly the digests of
  `strings` (no missing and no extra entry), and that each statement equals its
  string, except the two source paragraphs (§2.3). It also writes the table as
  `fixtures/reports/exact_report_statements.json`:
  `{schema_version: "1.0.0", generator: "<test path>", entries: [{producer_sha256,
  statement}]}`, sorted by digest. The test fails if the file is stale;
  `OPENPIPESTRESS_WRITE_EXACT_REPORT_STATEMENTS=<path>` writes it instead. This
  file is the table's only Rust form (§3.5).
- The test is registered in `tools/hosted-ci-routing.json` with the
  product-physics unit tests.

### 2.3 The statement table

Each row is one producer string with its exact report statement. "Verbatim"
means the statement is the producer string byte for byte.

**S1. Identity, pressure-2** (`exact_admission.rs:85`; first entry of every pressure-2 list). Producer SHA-256 `bb333a816c02a45e582042aea6110478e337e799e939dc8e756e551e7f9904bd`. Statement, verbatim:

> 3.0.0/exact_pressure_v3 under pressure-2 semantics. Admitted families: small-displacement homogeneous-isotropic straight circular pipe members (shear deformation as the contract declares: euler_bernoulli, or timoshenko_annulus_energy_v1 with the energy-matched annular shear factor, whose nodal rotations are section rotations; G is derived from E/nu) and linear restraints or springs; realized curved bends are Euler-Bernoulli under either shear model and, with expansion joints authored as objective connectors, are admitted only as the bend and connector limitations state; valve, flange, rigid and specialty components are admitted as their mapped straight member with the user's DEC-045 stiffness scale, an equivalent pipe of the mapped section whose stress rows are those of that reference section (marked component_body), not a body stress, and reducers only when their mapped member lies outside every pressure region (EXACT_PRESSURE_REDUCER_IN_REGION otherwise). Section bases: stiffness and pipe metal mass use the nominal wall; contents mass and pressure end-cap areas use the reduced bore; every stress row is published on the nominal section and on the reduced section (wall minus mill tolerance and corrosion allowance) and names its basis, and each basis has its own maximum and headline; no governing basis is chosen. Each basis's Lamé stresses are those of its own annulus, while the axial membrane stress divides the one wall force by that basis's area. Every other component, support, combination or equivalent-static family is refused by name (EXACT_PRESSURE_FAMILY_NOT_ADMITTED) until a later contract revision admits it.

**S2. Identity, pressure-source-1** (`exact_admission.rs:83`; first entry of every pressure-source-1 list). Producer SHA-256 `e582703b4491dcce94dd3e25391cab2658d837d67f97ba32cc5a78b9fcbea2c0`. Statement, verbatim:

> 3.0.0/exact_pressure_v3 under pressure-source-1 semantics. Admitted families: small-displacement homogeneous-isotropic straight circular pipe members (shear deformation as the contract declares: euler_bernoulli, or timoshenko_annulus_energy_v1 with the energy-matched annular shear factor, whose nodal rotations are section rotations; G is derived from E/nu) and linear restraints or springs; an expansion joint authored as an objective connector is admitted only as the connector limitation states. Section bases: stiffness and pipe metal mass use the nominal wall; contents mass and pressure end-cap areas use the reduced bore; every stress row is published on the nominal section and on the reduced section (wall minus mill tolerance and corrosion allowance) and names its basis, and each basis has its own maximum and headline; no governing basis is chosen. Each basis's Lamé stresses are those of its own annulus, while the axial membrane stress divides the one wall force by that basis's area. The selected retained-source response uses the contract's shear model and publishes its stress rows on both section bases. Every other component, support, combination or equivalent-static family is refused by name (EXACT_PRESSURE_FAMILY_NOT_ADMITTED) until a later contract revision admits it.

**S3. Pressure** (`exact_admission.rs:94`; every list). Producer SHA-256 `3e406fe7447bc73a133a81ff45404ebd76e0de07d1f0fc2714bcc95882088814`. Statement, verbatim:

> Pressure is a signed internal differential with zero external pressure increment, uniform within each explicit case-scoped collinear equal-bore region with explicit closure-transfer paths. A negative differential is admitted, but external-pressure stability and collapse are not assessed.

**S4. Not modelled** (`exact_admission.rs:95`; every list). Producer SHA-256 `0cd182f3d07b99b573b7ba3b68d9a672d232a0f5c365e2072555a49fe5ce9a22`. Statement, verbatim:

> Not modelled: steady-flow momentum and transient pressure loads; bend opening under pressure (Bourdon effect); pressure stiffening of flexibility factors and stress intensification; ovalization.

**S5. Contribution ledgers** (`exact_admission.rs:96`; every list). Producer SHA-256 `a61ae24cdd3c5325cf87fb20b02f682a91bbc0f57e8e5aeed0559f8f6bb862d9`. Statement, verbatim:

> Mechanical, pressure-eigen and cap-transfer contributions remain distinct; rounded cap/eigen ledgers are observational and do not replace source-grouped pressure assembly.

**S6. Line loads** (`exact_admission.rs:97`; every list). Producer SHA-256 `daa48a4c191547020a5894f475950a2b0bdc34f551d95659aae5cce72ee6871d`. Statement, verbatim:

> Structural line loads act as entered; contents density does not imply hydrostatic pressure head or a coupled static-fluid pressure/weight state.

**S7. Support attribution** (`exact_admission.rs:98`; every list). Producer SHA-256 `d47820118782fbf0e16a105f01fa8cdc1506ee99cc3ec7e251c5e936598ee5fc`. Statement, verbatim:

> Signed support actions are attributed only for admitted linear restraints and springs; ambiguous coincident rigid attribution is refused.

**S8. Normal-stress maximum enclosure** (`exact_admission.rs:99`; every list). Producer SHA-256 `f324475fea7267ad3c04b1f0d697d82e81c87f2dadeca853fa4e271e42795821`. Statement, verbatim:

> The circular normal-stress maximum bounds the supplied binary64 polynomial coefficient enclosures; solution error is separate, and incomplete coverage withholds the headline.

**S9. Source paragraph, pressure-2** (`exact_admission.rs:90`; every pressure-2 list). Producer SHA-256 `fada4069d170093e9e74ef785926ecb362dbccf94104e6b93fa7f602a31aed72`. Statement, the producer string without its last sentence, "No code compliance or professional acceptance is produced, and actual numerical admission remains separate." Nothing replaces it (DEC-107):

> Every member section declares its corrosion allowance and the contract declares its shear model. This envelope publishes ordinary structural recovery. Retained-source recovery is available only for straight members, linear restraints or springs, nodal force and moment loads and explicitly empty pressure regions; an authored load/reference-state group includes resolved eigenloads and prescribed motions. Other source families are declined by name.

**S10. Source paragraph, pressure-source-1** (`exact_admission.rs:88`; every pressure-source-1 list). Producer SHA-256 `4fc7d74460d35680e520131b2cffaa9f957d05231223b607c387b86a402b21a4`. Statement, the producer string without its last sentence, "No code compliance or professional acceptance is produced, and actual numerical admission remains separate." Nothing replaces it (DEC-107):

> Every member section declares its corrosion allowance and the contract declares its shear model. This pressure-source-1 envelope carries a selected retained-source response. The source family is straight members, linear restraints or springs, nodal force and moment loads and explicitly empty pressure regions; an authored load/reference-state group includes resolved eigenloads and prescribed motions. Other source families are declined by name. Other cases keep their ordinary recovery and standing.

**S11. Load/reference states 1, resolved cases** (`exact_admission.rs:104`; load-state models). Producer SHA-256 `c7d91ccf111fc974b8574f9de46442623780eee98c725dea3efcf68900e03204`. Statement, verbatim:

> One resolved case per load case supplies each member's selected E/nu with derived G, actual/selected/installation temperatures and explicit expansion definition; each case is solved independently.

**S12. Load/reference states 2, eigenstrain** (`exact_admission.rs:105`; load-state models). Producer SHA-256 `8f837617c5259422681368fdf8c0b62bd9fd88359e3747f766679fb25ffbeb59`. Statement, verbatim:

> Thermal and fit reference strain compose as lambda_fit*lambda_thermal-1 and enter once as an axial eigenstrain; uniform member temperature only, no gradients, finite strain, inelasticity or fit-up joints.

**S13. Load/reference states 3, prescribed motion** (`exact_admission.rs:106`; load-state models). Producer SHA-256 `62df2a3b0908cfbe3d067ff0e523fce53713a4f4b31474d41af684113fa7cd05`. Statement, verbatim:

> Global rigid support translations/rotations are prescribed absolute boundary values through the partitioned solve; reactions come from the unreduced equations. Spring base motion, device preload/reference, inactive or locked supports are not provided.

**S14. Load/reference states 4, applied loads** (`exact_admission.rs:107`; load-state models). Producer SHA-256 `d15de32c2e645c34fbdec18dea8fde381461f1a85aa72d5d6d74b3ada936aff5`. Statement, verbatim:

> Ordinary applied loads are exactly the case's declared source ledger with explicit factors; unreferenced stored primitives are excluded. Hydrostatic head, contents-weight state and per-case mass selection are not provided.

**S15. Load/reference states 5, history** (`exact_admission.rs:108`; load-state models). Producer SHA-256 `d67bef81dd0df58e72fe799467f916ff5c1d28c3fabadb64450d799fedc0bfa4`. Statement, verbatim:

> History is independent equilibrium only; no installation, contact, friction or predecessor history is represented.

**S16. Objective connector** (`joint.rs:164`; models with an objective connector). Producer SHA-256 `9a039e682885cdd1e6d14633173960dfd6a466df3abd6722cd3ecf959556ee0d`. Statement, verbatim:

> Objective connectors (expansion joints, objective_connector 1.0.0): symmetric-midpoint, small-displacement, small-rotation linear elastic law with the authored 6x6 work matrix, frame, offsets and installed reference q_ref, replacing exactly one straight span (replaces_span); untied hardware or explicit bilateral linear elastic tie rods (explicit_axial_rods_v1) between the connector's end nodes with rigid offsets, each with its own stiffness and reference extension. A pressurized joint (constant_effective_area_reference_v1) is a coaxial edge of one equal-bore pressure region between straight members, with zero attachment offsets, and carries the dead pressure load p(Ae - Ai) at its attachments, Ae being the authored effective area; unequal bore or pressure, offset faces and joints at region terminals or beside realized arcs are refused. Ideal inextensible rods, collective constraints, tension-only or gapped rods, nuts and stops are refused. No finite rotation, follower or geometric stiffness, pressure stiffening, squirm or stability, temperature law or stop is modelled; the replaced span carries no load, member result or self-weight. Connector actions are published as generalized and end-node elastic actions, with each tie-rod force, the joint pressure force pAe and the effective axial force S = g_x + sum T_r (n_r.Q_x) - Pe published separately; they are outside the straight-member recovery coverage.

**S17. Realized bend** (`bend_pressure.rs:35`; models with a curved macro element). Producer SHA-256 `e3cdc39260cf67ee730a608000cd3b182e65167c197095ff6dbceb49f1da3023`. Statement, verbatim:

> Realized curved bends (curved_bend_macro_element with the user's bend radius and flexibility factor k, one k for both bending planes) are admitted under pressure: each owns the term K_b*u_free(eps_p) - c_b, with eps_p = (1-2nu)pAi/(E As) (the straight Lame mean Poisson term on the arc) and c_b its own end caps along the end tangents; on an arc N_w = N_el + pAi and S = N_el, and shear and moments are elastic. A region changes direction only at realized bends: a bend-end direction change up to alpha_tan = 1e-3 rad is carried exactly by the remainder pAi(t_in - t_out); a larger change (a mitre) and a straight-straight kink are refused. Geometry-only (chord) bends carry no pressure and are refused. On arcs the hoop is the toroidal membrane cut average (bore_load_transferred_toroidal_membrane_v1) and the radial values are -p at the bore and 0 outside, the transverse shear is the straight-annulus Saint-Venant field driven by the tangent-frame shears (local_straight_annulus_on_arc), and the normal-stress maximum is bounded along the arc; ovalization, hoop bending from moments, bending boundary layers, tangent-end effects and the curvature correction of the shear field are not represented; the excluded effects above apply to every bend result.

**S18. Certified recovery 1** (`lib.rs:1206`; every list (also on preview-physics-2)). Producer SHA-256 `47a8910e8e8dcb0f60c3d3fe9ca400c91822859293c8331b221d2e913c586f84`. Statement, verbatim:

> Member actions, section actions, stresses and support reactions of a load case reported NUMERICAL_INTEGRITY_CHECKS_PASSED are recovered at 128-bit precision from the formation check's corrected displacement and rounded once; they are not binary64 recomputations from the solve's own displacement, from which they may differ by about 1e-12 of the body's action scale (at most about 6e-9 in the evaluation sets).

**S19. Certified recovery 2** (`lib.rs:1207`; every list (also on preview-physics-2)). Producer SHA-256 `eb043ebf9b3c8186a16574237e69ec69b977bb17c4546ef434c9dfe0b55ccfce`. Statement, verbatim:

> The recovery is checked against the formation check's re-formed system (stiffness and loads formed at 128-bit precision from the binary64 inputs). Wherever the formation check produced its member bound for a load case, including a case that the member recovery check then reports as sensitive, each published nodal displacement component is the corrected displacement (the solve's displacement plus the check's two correction terms, summed exactly) rounded once to binary64 in SI units and then converted to the published unit as every other row is, and each displacement magnitude is the correctly rounded norm of the node's three published translation components; where there is no such bound, or where any component of the case cannot be formed, every published displacement of the case is the solve's own, and a selected retained source publishes its recovery's nodal values. Support reactions balance the re-formed loads, not the solve's binary64 load vector.


### 2.4 In the report

- **The technical-preview limitation stays first** on every route
  (`renderableReportInput.ts:548-563`).
- **One entry per producer string**, in the producer's order, in
  `report_sections.limitations[]`:
  - `limitation_id`: `limitation:<run>:exact:<n>`, n from 1;
  - `source`: `{ref_type: "semantic_contract", ref_id: <identity>}`;
  - `affected_scope`: `{ref_type: "analysis_run", ref_id: <run>}`;
  - `statement`: the table's statement;
  - `effect`: the technical-preview entry's `effect`;
  - `provenance`: the session provenance (`renderableReportInput.ts:121-132`).
- **A reference entry last:** `limitation:<run>:exact:profile-reference`, same
  shape, with the statement "The profile limitations of the semantic table
  {table_id} (sha256 {table_sha256}) are bound by reference and are not restated
  in this report." `{table_id}` and `{table_sha256}` are the run's
  `reproducibility.semantic_contract` (`id`, `sha256`), which the admission
  already requires to equal `sourceSemanticBinding(result)`
  (`resultExportAdapter.ts:268`).
- **Notices.** The PRD §19.3 notice keeps its place: the renderer emits it in
  the professional-boundary section (`report_renderer/src/lib.rs:972-993`), and
  no exact-route text is added beside it. The local-private notice (W-3) is
  unchanged. There is no new notice.
- **Protected-content linter.** "Not modelled" carries "stress
  intensification" and the bend statement "flexibility factor"; both are
  standards-signal tokens (`protected_content_linter/src/lib.rs:490-514`). They
  raise only a warning, and only within a few lines of a numeric grid
  (`:645-668`); no exact statement carries a blocking phrase (`:689-698`). The
  render test pins that an exact report has no blocking finding.

## 3. Other exact-route sections

### 3.1 Admission

- **The rendered report** (slice (b)): `RenderedReportPanel.tsx:171` changes
  from `loadReferenceOutputRefusal(result)` to
  `surfaceOutputRefusal(result, model, "rendered-report")`. While the cell
  refuses, both return the same reason for every route: the policy's `reason`
  for a gated route, `OUTPUT_ROUTE_NOT_REGISTERED_REFUSAL` for a missing entry,
  null for an ungated one (`outputPolicy.ts:140-178`). Behaviour is unchanged.
- **Exact admission is bound to the admitted bytes.** Equal identities and runs
  do not show that Export Result JSON admitted these result bytes, so the report
  binds the admission itself. `features/report/exactReportAdmission.ts` exports
  `admitExactReportSource({model, result, analysisRun, inputManifest})`. It
  calls `buildCurrentResultExport` with those inputs. That call requires
  eligible standing with the live native invocation and the bound input
  manifest and record, and re-checks them at its end
  (`resultExportAdapter.ts:252-295`). The function returns `{document,
  binding}`. Each binding field is read from the document and recomputed from
  the inputs. Any difference refuses with
  `REPORT-EXACT-ADMISSION-BINDING-MISMATCH: <field>`.

  | Field | From the document | Recomputed from the inputs |
  |---|---|---|
  | `semantic_contract_id` | `result_envelope.producer.semantic_contract_id` | `result.producer.semantic_contract_id`, which is pressure-2 or pressure-source-1 |
  | `run_ref` | `result_envelope.run_ref.ref_id` | `result.run_id` |
  | `result_envelope_id` | `result_envelope.envelope_id` | `result-envelope:<run>` |
  | `received_result_sha256` | `reproducibility.source_origin_bindings[0].received_carrier_checksum.value` | `resultDigest(result)` |
  | `model_sha256` | `reproducibility.model_hash.value` | `canonicalSha256Hex(model)` |
  | `input_manifest_ref`, `input_manifest_sha256` | `reproducibility.audit_manifest_ref.ref_id`; the run record's input-manifest hash | `inputManifest.manifest_ref.ref`, `inputManifest.manifest_sha256` |
  | `derivative_sha256` | `reproducibility.derivative_hash.value` | the document's self-hash, checked by its builder |
  | `native_job_id` | none | `deliveredMechanicsResultJob(result)`, non-null (`previewService.ts:437`) |
  | `received_bytes_sha256` | none | the SHA-256 of `receivedResultBytes(result)`, non-null (`previewService.ts:129`) |

  The last two carry the live invocation's custody across IPC: they name the
  job that delivered the result and the bytes it delivered.
  `receivedResultBytes` is non-null only while the result is unchanged since it
  was read from those bytes.
- **Recorded, not forgeable.** The returned admission is recorded in a
  module-private `WeakMap` with the four input objects and their checked JSON
  text. `buildRenderableReportInput` takes it as a new argument,
  `exactAdmission`. For an exact source the builder requires all of these:
  - a recorded admission for the same four objects, by identity, with unchanged
    text;
  - `hasNativeMechanicsInvocation(result, model, solver_mode)` is still true;
  - the standing is still eligible;
  - `deliveredMechanicsResultJob(result)` is still the binding's job.

  A missing or unrecorded admission refuses with
  `REPORT-EXACT-ADMISSION-REQUIRED: an exact-route report is built only from a
  result that Export Result JSON admits.` This covers a copy, a JSON round trip
  and an object built by hand. A recorded admission offered for other objects,
  or whose conditions no longer hold, refuses with
  `REPORT-EXACT-ADMISSION-STALE: the result Export Result JSON admitted is no
  longer this session's live result.` The builder checks at its start and again
  after its last await, as `buildCurrentResultExport` does (`:293`). Its callers
  repeat the check immediately before IPC: the panel before
  `renderCalculationReport`, and the package request before `save_report_package`.
  The package request uses the admission's document as its result member
  (`reportPackageRequest.ts:179`), so the package and the report name one
  admission. A pressure-free source takes no admission, so its path is
  unchanged.
- **Same-run substitution.** Suppose a result keeps the run id, identity and
  producer but differs in a byte. It is another object with another digest, so
  it matches neither the recorded objects nor `received_result_sha256`, and it
  refuses.
- **Asynchronous retirement.** The session may drop, replace or dispose the
  result, reset custody, or edit the model during the build. The invocation,
  job or text check then fails at the next check point, and nothing reaches
  IPC.
- **The native render refuses exact identities before opening (§3.6 step
  0).** `render_calculation_report` refuses every exact-route input (one that
  carries `exact_route`, an exact identity in an identity field or exact
  limitation entries; free text is not a marker) unless its `exact_route` names an identity in the render's
  admitted set, which is empty until opening, with
  `REPORT-RENDER-ROUTE-NOT-ADMITTED`. The package's Rust
  boundary does the same at item 0. Before opening, no production path
  therefore renders or packages an exact-route report, even with every webview
  check bypassed. §3.6 states what the native render can and cannot see.
- **The native host binds the source (§3.6).** `render_calculation_report`
  (`src-tauri/src/lib.rs:1540`) and `save_report_package` (`:58`) refuse an
  exact input unless all of these hold:
  - the named job's result is still in the app's custody;
  - the result and model texts submitted with the input are the bytes that job
    delivered and received, by digests the host recorded itself;
  - the native reader's standing for those bytes is numerically eligible;
  - the input's source is that result, and, for a render, its rows and exact
    sections are re-derived from it.

  A released job refuses with `REPORT-EXACT-CUSTODY-RELEASED: the result this
  report was admitted for is no longer in the app's custody.` The save checks
  custody again after its dialog returns and before the write. The host still
  keeps no result bytes after delivery. For an exact render or save it re-reads
  the texts the webview submits, under a priced parse, after checking their
  digests against its own. The link from the delivered bytes to the request's
  source value is therefore enforced natively, not only in the webview.
- **Below the gate.** Tests reach these sections through
  `buildRenderableReportInput` directly and through
  `assembleExactReportBelowOutputGate`, a test-only export of the panel's build
  step, as W-18 (b2) did before opening. Both take an admission from
  `admitExactReportSource` under the native-capture test seams that the existing
  result-export tests use. The native render of an exact input runs only
  through the desktop crate's test-only witness set (§5.5), and the package's
  assembly only through `report_package`'s.

### 3.2 The exact-route result block

`report_renderer::RenderableReportInput` gains
`#[serde(default, skip_serializing_if = "Option::is_none")] pub exact_route: Option<ExactRouteSection>`:

```rust
pub struct ExactRouteSection {
    pub semantic_contract_id: String,           // the result's producer identity
    pub admission: ExactAdmissionBinding,       // §3.1; the standing line is derived from it
    pub load_reference_states: Option<String>,  // §3.2 load/reference line
    pub headlines: Vec<ReportHeadline>,         // §3.3, nominal then reduced
}
pub struct ExactAdmissionBinding {              // the §3.1 binding, field for field
    pub semantic_contract_id: String,
    pub run_ref: String,
    pub result_envelope_id: String,
    pub received_result_sha256: String,
    pub model_sha256: String,
    pub input_manifest_ref: String,
    pub input_manifest_sha256: String,
    pub derivative_sha256: String,
    pub native_job_id: String,
    pub received_bytes_sha256: String,
}
pub struct ReportHeadline {
    pub basis: String,                 // "nominal_section" | "reduced_section"
    pub label: String,
    pub value_display: String,
    pub location_ref: Option<String>,
    pub source_ref: Option<String>,
}
```

The three new structs are `#[serde(deny_unknown_fields)]`; the enclosing
struct still drops unknown top-level fields (§1.1 item 3). The renderer emits
the block at the start of the Results section
(`report_renderer/src/lib.rs:710`), in `assemble_report_sections`, so the HTML
and the PDF carry it alike:
- subheading "Exact-Route Result";
- a meta paragraph "Semantic contract: {semantic_contract_id} · Numerical
  standing: {the standing line}";
- the load/reference paragraph, when present;
- subheading "Stress Headlines by Section Basis", a table with the columns
  "Section basis", "Stress maximum", "Location" and "Source", and the paragraph
  "Each section basis has its own maximum; no governing basis is chosen."

An absent `exact_route` emits nothing, so pressure-free output is byte-identical.

**Standing line.** The input carries no standing text; the renderer composes
the line from the admission binding. When `exact_route` is present, the
renderer first checks the binding against the report's own records. Any
difference refuses with `REPORT-EXACT-ADMISSION-BINDING-MISMATCH: <field>`:
- the two `semantic_contract_id` values are equal;
- `run_ref` is the run of `report_sections.run_ref`;
- `received_result_sha256` is the checksum of the received-result entry of
  `calculation_report.audit_manifest_refs`, whose reference is `result_envelope`,
  `result_envelope_id`;
- `model_sha256` is `calculation_report.model_input_summary.model_hash.value`;
- `input_manifest_ref` is
  `calculation_report.model_input_summary.input_manifest_ref.ref_id`;
- every digest is 64 lowercase hex characters and every id is non-empty.

It then writes: "integrity checked (the Rust reader's standing for the result
Export Result JSON admitted in this session: received result sha256
{received_result_sha256}, model sha256 {model_sha256}, input manifest
{input_manifest_ref} sha256 {input_manifest_sha256})". "integrity checked" is
the session's label (`LoadReferenceStatesBlock.tsx:16`).

Each consumer checks the binding as far as its inputs reach:
- the desktop recorded it at admission and re-checked it live before IPC
  (§3.1);
- the native host bound the source and model to the bytes its solve job
  delivered and received, read them for eligible standing, bound the
  result and model digests to them and, for a render, re-derived the rows and
  exact sections from them (§3.6);
- the renderer matched it to the report's own records (above);
- in a package, the bindings re-derive it from the source and its document
  (§3.5).

A pressure-free report has no `exact_route` and no standing line. On a
pressure-free report, the numerical-health lines the builder already adds on the
generic route (`renderableReportInput.ts:672-678`) are unchanged. An exact
report carries no numerical-health line, whatever the panel supplies (§5.2,
§6.1).

**Load/reference line.** `loadReferenceDisclosure.tsx` gains the surface
`"rendered-report"` with the tail "Their published limitations are listed under
Limitations." The report line is therefore: "This result includes resolved
load/reference states (reference configuration {configurations}; load cases
{cases}; history {histories}). Their published limitations are listed under
Limitations." The identifiers come from the published records as for the other
surfaces (`:42-57`). The module comment changes to say that this one line is
written into the rendered report. A `pressure-source-1` result with
load/reference states is never eligible (`resultStanding.ts:23`), so it never
reaches a report.

### 3.3 Headlines

- One entry per section basis, nominal then reduced, from
  `summary.max_open_formula_stress_nominal_section` and `…_reduced_section`.
- `label`: "Nominal section (nominal wall)" and "Reduced section (wall minus
  mill tolerance and corrosion allowance)". The labels move to one exported
  constant shared with `ResultsPanel.tsx:227-230`.
- `value_display`: `` `${value} ${unit}` ``, the result rows' form, which the
  Rust side reproduces with `canonical_number` (`bindings.rs:114-117`).
  `location_ref` and `source_ref` are the headline's `location_ref` and
  `result_ref`.
- A null key (incomplete coverage) is withheld: `value_display` "withheld: a
  stress maximum on this basis is not available for every member in every load
  case", the session's text (`ResultsPanel.tsx:250`), and both refs null, shown
  as "not available". A withheld headline is never a value.
- An absent or malformed key on an exact result refuses the report with
  `REPORT-EXACT-HEADLINE-UNREADABLE: the <basis> headline is missing or not
  readable.`

### 3.4 Result rows

- **Rule.** Split `metadata.basis` on ";" and trim each token. Exactly one token
  `section_basis=nominal_section` adds the suffix " [nominal section]" to the
  label; exactly one `section_basis=reduced_section` adds " [reduced section]".
  No `section_basis=` token adds nothing. Any other `section_basis=` value, or
  more than one, refuses: `REPORT-ROW-SECTION-BASIS-UNREADABLE: <row id>`
  (TypeScript) and `REPORT-PACKAGE-REPORT-ROWS-MISMATCH` (Rust).
- **Example (invented ids):** `element_local_axial_normal_stress
  (axial_normal_stress) [reduced section]`.
- **Why the metadata.** The rule runs only after the reader admitted the whole
  source (§3.1), and the reader requires each declared basis to equal its
  signature's (§1.5), so `metadata.basis` is the classified basis. Reading it
  keeps the projection pure on both sides. A row the reader does not classify
  never reaches the rule: the admission refuses it, as the result export does.
- **Where.** `reportResultRows` (`reportResultRows.ts:19-27`) and
  `bindings::expected_report_result_rows` (`bindings.rs:103-140`) apply the same
  rule; `report_result_rows_parity.json` gains exact-route vectors (both bases, a
  W-7 global end-force row, a row without metadata, the two refusals). No
  preview-physics-2 row has a `section_basis` token (its table has none), so
  pressure-free rows and the existing vectors are unchanged.

### 3.5 Package bindings for the exact sections (dormant)

While the Rust boundary refuses exact identities at item 0
(`bindings.rs:194`), nothing below runs on a production package. It is built
and tested behind that refusal, in unit tests and through the test-only
assembly of §5.5, so that opening changes only the admitted set.

**The closed table in Rust.** `core/reporting/report_renderer/src/exact_statements.rs`
embeds `fixtures/reports/exact_report_statements.json` with `include_str!`, as
`report_package/src/schema_eval.rs:47-62` embeds the schemas. It parses the
file once and requires exactly 19 entries, unique lowercase 64-hex digests and
non-empty statements. Any other content refuses every exact section. The module
exposes `exact_statement_for(producer_sha256) -> Option<&'static str>` and
`is_exact_statement(text) -> bool`. There is no other Rust copy. The TypeScript
table is the reviewed source, which GEN-13 scans as desktop product source (§7);
the JSON is its generated form (§2.2).
- **The renderer** (every report, rendered or packaged) refuses an input whose
  `limitation:<run>:exact:<n>` entries are present without `exact_route`, or
  whose statements are not table statements (`is_exact_statement`). The refusal
  is `REPORT-EXACT-LIMITATION-UNREGISTERED`.

**The runner profile is refused directly.** Only the desktop builds exact
sections, because only the desktop has the Export Result JSON admission (§3.1)
and the native source binding (§3.6). The absence of a runner-built
`exact_route` is not relied on. Directly after item 0, `check_request` refuses
a request whose `export_profile_id` is `RUNNER_PROFILE` (`bindings.rs:36`) and
whose result document has an exact identity
(`document_form::producer_contract_id`, `document_form.rs:39`). The refusal is
`REPORT-PACKAGE-EXACT-RUNNER-PROFILE: the runner profile does not package
pressure-2 or pressure-source-1 results; only the desktop builds their
sections.` The check reads only the profile and the identity, so it refuses
whatever the report carries. The reader applies the same check to the
manifest's `export_profile_id` and the result member, beside its item-0 check
(`reader.rs:930`). Before opening, item 0 refuses these first. The runner check
is exercised under the witness set (§5.5, §7).

**Item 9 of the package bindings**, after item 8, applies to a document with an
exact identity:
- An exact identity requires the report's `exact_route`. A preview-physics-2
  document whose report carries `exact_route` refuses with
  `REPORT-PACKAGE-EXACT-SECTIONS-MISMATCH`.
- `expected_exact_route_section(source, document, audit)`:
  - the identity;
  - the admission binding, re-derived from the request:
    - `semantic_contract_id`, `run_ref` and `result_envelope_id` from the
      document;
    - `received_result_sha256` as `digest(source)`, which item 5 already binds
      to the origin carrier and the run hashes;
    - `model_sha256` as the document's model hash (item 5);
    - `input_manifest_ref` and `input_manifest_sha256` as the audit
      input-manifest hash's `payload_ref` and `value`;
    - `derivative_sha256` as the document's `reproducibility.derivative_hash.value`
      (item 3 checks the self-hash);
    - `native_job_id` and `received_bytes_sha256` checked for form only. They
      are custody facts: the shared producer has no custody, and the native
      save binds the request's source and model to that job's bytes before
      item 0 (§3.6);
  - the load/reference line from `contract_evidence.load_reference_states`
    (distinct configurations, cases and histories, in published order);
  - the headlines from `summary`;
  - all with the §3.2-§3.3 texts.
- `expected_exact_limitations(source, run)`: for each `formulation_basis.limitations`
  string, in order, the SHA-256 of its UTF-8 bytes must name a table entry, and
  that entry's statement is the expected statement. No string is copied and no
  suffix is stripped. An unknown digest refuses with
  `REPORT-EXACT-LIMITATION-UNREGISTERED: <identity> publishes a limitation this
  report has no reviewed statement for.` The reference entry (§2.4) follows,
  with the table binding of the source's route (`reader_verdict::route` and
  `binding`, `reader_verdict.rs:88,122`).
- both are compared with the report, refusing with
  `REPORT-PACKAGE-EXACT-SECTIONS-MISMATCH`;
- **One projection for both commands.** These functions are public in a new
  `report_package::exact_sections` module, beside `expected_report_result_rows`.
  The source-derived part, `expected_exact_route_core(source)`, covers the
  identity, `run_ref`, `result_envelope_id`, `received_result_sha256`, the
  load/reference line and the headlines. Item 9 adds the document- and
  audit-derived fields, and the native render re-derives with the same core
  (§3.6);
- shared vectors `fixtures/reports/invented/exact_report_sections_parity.json`,
  checked by vitest against the builder and by `report_package` against these
  functions;
- tests:
  - an invented unknown string, in a source and document whose hashes are
    rebound coherently, refuses with `REPORT-EXACT-LIMITATION-UNREGISTERED`;
  - a changed source paragraph refuses likewise. Two cases are tested: S9 with
    one word changed, and S9 published without its last sentence (each is a new
    digest);
  - a same-run substitute source refuses with
    `REPORT-PACKAGE-EXACT-SECTIONS-MISMATCH`. It changes one row value, rebinds
    its hashes and keeps the report's binding;
  - each binding field mutated refuses likewise;
  - the fixture's strings ending in the omitted sentence are exactly the two
    source paragraphs;
  - a runner-profile request with an exact identity refuses with
    `REPORT-PACKAGE-EXACT-RUNNER-PROFILE`, both without `exact_route` and with
    a coherent `exact_route` copied from the desktop witness. A witness package
    whose manifest is rewritten to the runner profile, with its package digests
    recomputed, refuses on read with the same code;
  - **the shared checks alone cannot see a coherent substitute.** The
    direct-native substitute request (§7, slice (b)) passes items 0-9 under the
    witness set. The native binding (§3.6) is what refuses it.

The TypeScript check at `reportPackageRequest.ts:181-185` reads a new exported
constant, `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS`, which a test keeps equal to
Rust's `ADMITTED_SEMANTIC_CONTRACTS` through the parity vectors. Both stay
`[preview-physics-2]`.

### 3.6 The native source binding

Revision 3's native check bound a job id to a raw-byte digest. Nothing native
linked those bytes to the source value in a request. A direct IPC request could
keep a live job id and its genuine digest, substitute the source, and rebind
its document, report and hashes coherently. Custody then passed, and the
comparisons of §3.2 and item 9 established only internal consistency. A
standalone render had no source-derived check at all. This section closes both
at the native boundary.

**The choice: resubmission with a priced parse.** Two options were weighed.
- *A digest computed before delivery.* The solve-job result path would parse
  each completed result and record its canonical digest. That puts a parse,
  priced at the result's materialization bound, on every solve. It would also
  run for pressure-free results and for results never reported. It adds a
  refusal point to DEL-04-07's publication path, which today never parses a
  result: the parent keeps only raw bytes. It also leaves two gaps. The
  standalone render still needs the source sent again to re-derive its rows and
  sections. Pressure-source-1 standing needs the solved model, which the host
  never parses either.
- *Resubmission.* The webview sends the exact texts it received and sent. The
  host checks their SHA-256 against digests it recorded in place, then parses
  them under the command's lease. The solve path gains two in-place hashes and
  no parse, allocation or lease change. The parse falls only on an exact render
  or save, which the user starts. The same parse serves the render's
  re-derivation and binds the model.

This design takes resubmission. The canonical-source digest is native-owned: the
host computes `digest(source)` from text whose raw digest it recorded itself.

**What the host records.** `SolveJobRecord` (`solve_jobs.rs:213`) gains
`custody_digests: Option<CustodyDigests>`, where `CustodyDigests` is
`{intake_sha256: [u8; 32], result_sha256: [u8; 32], solver_mode: PreviewSolverMode}`.
- `intake_sha256` is the SHA-256 of the raw model body that `start` admits
  (`:424`). It is hashed in place, before the body is copied for the worker.
- `result_sha256` is the SHA-256 of the result bytes. It is hashed in place in
  the job's supervisor thread, after the UTF-8 check (`:635`) and before
  `publish` (`:668`) takes the registry lock. Revision 3 hashed in
  `take_result`, under that lock; nothing is hashed there now.
- `solver_mode` is the admitted mode.

This is bounded digest metadata, 65 bytes per job. No result or model bytes are
retained for it, and no lease changes. A new query, `custody_binding(job_id)`,
replaces revision 3's `custody_digest`. It returns a copy only while the
result is delivered and its custody lease is held. A retained, untaken result
has no binding. `dispose_result` (`:756`) and `dispose_all_results` (`:784`)
take the digests together with the lease, and pruning a record (`:261`) removes
them. The digests are therefore released with custody.

**What the webview submits.** An exact render and an exact save carry these
texts beside their existing arguments; a pressure-free input carries none:
- `exact_result_text`: `receivedResultBytes(result)`, the bytes the job
  delivered (`previewService.ts:129`);
- `exact_intake_text`: the model text `startPreviewMechanicsJob` sent
  (`previewService.ts:290`). A new query, `deliveredIntakeText(result)`,
  re-serializes it with `checkedJsonText` from the result's native
  registration, and returns null without one;
- for the render only, `input_manifest_text`: the canonical text of the
  admission's input manifest. The save already carries one.

The panel and the request builder read the texts at their pre-IPC re-check
(§3.1).

**Step 0: the render refuses exact identities until opening.** Ruling 10A
keeps exact-route report outputs refused until the owner opens them. The
package enforces this natively at its Rust identity boundary (item 0,
`document_form.rs:17,45-51`). The render gets the same boundary:
- `apps/desktop/src-tauri/src/report_render_exact.rs` (new) holds
  `RENDER_ADMITTED_EXACT_CONTRACTS: [&str; 0] = []`, the exact identities
  whose `exact_route` the native render admits. It also holds
  `render_calculation_report_args_in`, the render's exact path (§5.5), and
  the step itself, `check_render_route_admitted`. The step lands in slice
  (a), with the renderer's `exact_route` (§6).
- The step runs first for every render input, before step 1, any parse of
  the texts, any custody query, any lease growth and the renderer's decode of
  the input. It reads, in place, the `input` argument that
  `command_ingress::admit` already priced and decoded, and allocates nothing
  before it decides.
- It recognizes an exact-route input only by structured markers at defined
  paths of the input, and never reads free text: labels, notes, statements,
  descriptions, provenance strings and other user-supplied values are not
  route evidence. The desktop copies a load-case label verbatim, so a
  pressure-free report whose label equals an exact identity renders as it
  does today (Codex review round 2 of #1632). The markers are:
  - a non-null `exact_route`;
  - an exact identity (`PRESSURE_ID`, `PRESSURE_SOURCE_ID` or
    `PHYSICS_RETAINED_ID`) at an identity field:
    `exact_route.semantic_contract_id`,
    `exact_route.admission.semantic_contract_id`, or the `ref_id` of a
    `report_sections.limitations[].source` whose `ref_type` is
    `semantic_contract` (§2.4);
  - an exact limitation entry: a `report_sections.limitations[].limitation_id`
    of the form `limitation:…:exact:…`, the renderer's predicate (§3.5).

  Such an input passes only when its `exact_route.semantic_contract_id` is in
  the admitted set and every exact identity at its identity fields is too.
  Otherwise it refuses with
  `REPORT-RENDER-ROUTE-NOT-ADMITTED: the rendered report admits no exact-route
  identity; found <identity>.` The found text is the identity, or names an
  `exact_route` without one, or the first exact limitation entry of an input
  without `exact_route`. Opening adds `PRESSURE_ID` and `PRESSURE_SOURCE_ID`,
  and the message then names that set (§5.1).
- Before opening, a genuine exact render input from a live job, with its
  genuine texts, therefore refuses here. The rest of this section runs on a
  render only under the desktop crate's test-only witness set (§5.5).
- An input with no marker has no `exact_route`, so the renderer emits no
  exact block, standing line or headlines for it (§3.2), and it has no exact
  limitation entry. Its copied text is ordinary report text. This is the
  route-blind render below.
- The render input names its source identity only in `exact_route` and in
  its exact limitation entries' sources (§1.2, §2.4).
  An input that carries the exact texts without `exact_route` refuses with
  `REPORT-EXACT-SECTIONS-MISMATCH: exact_route` (below). One that carries
  exact limitation entries without it refuses at step 0, and the renderer's
  own check stands behind it (§3.5). These refusals hold in every build.

**What the host checks.** The checks apply to a render input with
`exact_route` that passes step 0, and to a save whose result document has an
exact identity or whose report carries `exact_route`. They run in this order,
before every other check of the input:
1. `custody_binding(native_job_id)` exists. Otherwise the command refuses with
   `REPORT-EXACT-CUSTODY-RELEASED` (§3.1).
2. Hashed in place, without allocating: SHA-256(`exact_result_text`) equals
   `result_sha256` and the binding's `received_bytes_sha256`, and
   SHA-256(`exact_intake_text`) equals `intake_sha256`. Otherwise the command
   refuses with `REPORT-EXACT-SOURCE-NOT-DELIVERED: <result|model>: the
   submitted text is not the bytes this app's solve job delivered or
   received.`
3. The lease grows by the priced parse (below). Both texts are then parsed and
   guarded (`guard_json`), giving `source` and `model`.
4. The standing is computed natively: `reader_verdict::standing(source,
   Some(model), Some(invocation))` (`result_export/src/reader_verdict.rs:391`).
   The `invocation` is `{request: {model, materials: []}, solver_mode}`, built
   from the recorded mode in the form of `NativeMechanicsInvocation`
   (`previewService.ts:98-101`). It must be eligible. Otherwise the command
   refuses with `REPORT-EXACT-STANDING-NOT-ELIGIBLE: the native reading of the
   delivered result is not numerically eligible.`
5. The binding's `received_result_sha256` is `digest(source)`
   (`derivative.rs:92`), and its `model_sha256` is `digest(model)`. Otherwise
   the command refuses with `REPORT-EXACT-SOURCE-NOT-DELIVERED`.

Then each command finishes its own checks:
- **Save.** `digest(request.source_results[0])` must equal `digest(source)`,
  and the model text's SHA-256 must equal `digest(model)` (the text is
  canonical, `report_package_save.rs:612-613`). Otherwise the save refuses with
  `REPORT-EXACT-SOURCE-NOT-DELIVERED`. The existing model, input-manifest and
  audit checks (`:612-633`) and the shared producer follow, so items 0-9 run on
  the delivered source and the solved model. Before opening, a genuine exact
  request passes steps 1-5 and then refuses at item 0. After the dialog the
  save repeats step 1 and compares the binding with the digests of step 2,
  which it keeps.
- **Render** (after step 0 admits the identity, which before opening only the
  witness set does). The input manifest text is parsed as the save parses its own
  (`parse_canonical`, `:368`). Its SHA-256 must be the binding's
  `input_manifest_sha256`, and its model must be the bound model
  (`check_manifest_model`, `:502`). The host then re-derives, from the bound
  source and with the functions item 9 uses (§3.5):
  - the result rows (`expected_report_result_rows`);
  - the exact limitation entries and the profile reference;
  - the `exact_route` section (`expected_exact_route_core`), with
    `model_sha256`, `native_job_id` and `received_bytes_sha256` from steps 2-5.

  The render receives no result document, so `input_manifest_ref` and
  `derivative_sha256` are checked for form and against the report's own
  records (§3.2). Any difference refuses with `REPORT-EXACT-SECTIONS-MISMATCH:
  <section>`. The renderer's own checks (§3.2, §3.5) follow. An input that
  carries the texts without `exact_route` refuses with
  `REPORT-EXACT-SECTIONS-MISMATCH: exact_route`. The render does its exact work
  on a blocking worker thread, as the save does (`lib.rs:87`).

**Pricing** (DEL-04-07 `memory-budget-decision.md`, ingress and "Report-package
request pricing"). Hashing allocates nothing. Each added parse is priced before
allocation by `command_ingress::text_parse_bytes` (`command_ingress.rs:118`),
the rule the save already applies to its model and input-manifest texts:
- a prescan that allocates nothing gives M;
- the price is M, plus two envelopes of the text's length for the canonical
  re-serialization, plus the 4 KiB allowance.

The commands apply it as follows:
- **Save.** `parse_lease_bytes` (`report_package_save.rs:354`) adds the two
  texts' terms to its sum. `check_text_sizes` (`:331`) caps them. The result
  text's cap is `MAX_REQUEST_TEXT_BYTES`, since the source is a member of the
  request. The intake text's cap is `MAX_MODEL_TEXT_BYTES`. Steps 4-5 and the
  comparisons run under the held lease as the command's working memory, within
  the existing W M provision.
- **Render.** The lease from `command_ingress::admit` (`lib.rs:1544`) already
  prices the args body, which now holds the three texts as strings. Before step
  3, the lease grows by:
  - the three texts' `text_parse_bytes`;
  - W M_r, where M_r is the result text's materialization bound and W is
    `REPORT_PACKAGE_WORKING_MEMORY_FACTOR` (1).

  W M_r is the render's working-memory provision for the standing, the
  re-derivation and the render of an exact input, as W M is the save's. It is
  measured for the witness inputs (§7) and is not a proven bound. The texts get
  the save's caps, and the manifest text gets `MAX_INPUT_MANIFEST_TEXT_BYTES`.
  Before opening, no production render reaches this growth (step 0).
- **Owner risk acceptance.** Codex's third review round found that W M_r
  needs no further owner risk acceptance. The provision is additive, and it
  falls within the existing command-working-memory residual that DEL-04-07
  already carries. The qualification stands: W M_r is measured for the
  witness inputs, not proven. The DEL-04-07 updates land in slice (b) (§8).
- **Refusals.** A refused growth refuses the command with its typed
  `ResourceRefusal`, as the save's parse growth does.

Every term adds to a lease. **No DEL-04-07 limit, cap, factor or admission rule
is loosened.** The memory-budget design gains these terms in slice (b) (§8).
Wry still buffers each body before any check; that is the design's existing
residual.

**What this enforces, and what it does not.**
- **Enforced.** After the checks, an exact report's source and model are the
  bytes that a solve job of this app delivered and received, while that job's
  result is in custody. The native host has read them at the native reader's
  eligible standing. A standalone render's rows and exact sections are derived
  from that result.
- **Input manifest.** The host keeps no record of the input manifest a session
  used. The render therefore binds the manifest digest to a manifest whose
  model is the solved model, not to the session's manifest.
- **Derivative digest.** The derivative digest belongs to the Export Result
  JSON document. Only the package carries that document, and item 9 re-derives
  the digest from it.
- **Desktop checks.** The admission of §3.1 (Export Result JSON for these
  objects in this session) stays a desktop check.
- **The native identity gate.** Before opening, the native render refuses
  every exact identity at step 0, as the package's Rust boundary does at item
  0. The policy cells (§1.2) are the webview's gates. Before opening, no
  production path reaches an exact output. The webview refuses at the cells.
  The two native boundaries refuse every input that carries an exact
  identity in an identity field or another exact-route marker, even when a
  webview check is bypassed. At opening, the cells, the two native
  admitted sets and the TypeScript identity list change together (§5.1).
  Steps 1-5 and the commands' own checks then bind what an admitted exact
  input claims.
- **A route-blind input.** The native render cannot recognize an input that
  carries no step 0 marker and no exact section: no `exact_route`, no exact
  identity in an identity field, no exact texts and no exact limitation
  entries. Its free text may equal an exact identity or copy an exact
  statement. Such an input has no exact sections, standing line or
  headlines, so it is not an exact-route report.
  The render renders it as it renders any input today (§1.2). This is the
  route-blind render that exists today, and W-11 does not widen it. A native
  check would need every render, pressure-free included, to resubmit its
  source. The app's own production caller sends such an input only for a
  result its cell admits (`RenderedReportPanel.tsx:171`).

## 4. Report provenance: materials, sections, supports and library records (FR-REP-003, every route)

### 4.1 Records

**With a `provenance_record` r** (material, section, component or support), the
report record is:

| Field | Value |
|---|---|
| `source_name` | `r.source_name` when non-empty after trimming, else "not recorded" |
| `source_location` | `r.source_location`, else "not recorded" |
| `source_license` | `r.source_license`, else "not recorded" |
| `contributor` | `r.contributor`, else "not recorded" |
| `contributor_certification` | `r.contributor_certification`, else "not recorded" |
| `redistribution_status` | `r.redistribution_status`, else "unknown" |
| `review_status` | `r.review_status`, else "pending" |
| `privacy_classification` | "private_project_data" |

`reviewed_by`, `reviewed_on` and `verification_note` are not carried: they are
not §20.2 fields, and `reviewed_by` names a person. `library_ref` goes to §4.2.

**`review_status` is copied (this design's choice).** A `review_status` of
`accepted` is the user's own source-data review (`types.ts:12-14`), and the
report copies it as source-review data. No rendered table shows it, the
software never sets or advances it, and the screener treats it as it treats
`pending` (`redactionExportControls.ts:442-444`). The report does not write
`pending` for every record. A blanket `pending` would also overwrite `rejected`
and `quarantined`, which the screener blocks (§4.3), so those records would
reach a report. Copying keeps the blocks. One normalization applies, at value
level only. A user-supplied value that is a missing-data finding carries
`pending` where its record says `accepted`, so
`REPORT_SECTION_MISSING_VALUE_MARKED_ACCEPTED` (`report_sections/src/lib.rs:534-540`)
cannot fire on a copied review. `rejected` and `quarantined` are kept, and the
record inside the value keeps its copied status.

**One provenance predicate.** An entity has provenance when its
`provenance_record.source_name` or its `provenance` note is non-empty after
trimming. The predicate applies alike to components, materials, sections and
supports. It decides `missing_data_finding` (its negation), and which entities
get a `*_PROVENANCE_MISSING` diagnostic. `componentHasProvenance`
(`renderableReportInput.ts:196-198`) becomes this predicate, and the component
values and diagnostics (`:200-236`) use it. A component with a structured record
and no note therefore gets neither a missing-data flag nor a warning.

**Without one**, a material or section gets the component pattern
(`renderableReportInput.ts:162-194`):
- `source_name` "{label} material provenance" (materials' `label`) or
  "{name} section provenance" (sections' `name`);
- `source_location` "material.provenance={provenance}" or
  "section.provenance={provenance}", with an object-valued section `provenance`
  as its RFC 8785 text; "material provenance missing" or "section provenance
  missing" when empty;
- `source_license` "user_supplied_or_private", `contributor`
  "user_local_session", `contributor_certification` "User-local material
  provenance metadata; redistribution remains private or pending until
  separately cleared." (or "section"), `redistribution_status` "private_only",
  `review_status` "pending", `privacy_classification` "private_project_data".

**Where they go.**
- `provenance_notes`: session, then components and spring hangers as today (a
  component with a record uses the mapped record in place of
  `componentProvenanceRecord`), then materials, sections and the supports that
  carry a record, in model order. Appending keeps today's order.
- `user_supplied_values`, after today's entries:
  - `value_id` `material-provenance:<id>`, `section-provenance:<id>` or
    `support-provenance:<id>`;
  - `value_category` `report_metadata`, the schema's existing category
    (`schemas/report_sections.schema.yaml:605-617`). The specific identity is
    in `value_id` and `source`, and no new category is added;
  - `source` `{material|section|support, <id>}`;
  - no `quantity` member. The schema's `ValueQuantity` is an object, and the
    Rust field is an `Option`;
  - `provenance` the record, and `privacy_classification` from the record;
  - `review_status` from the record, as normalized above;
  - `required_for` ["reporting", "human_review"];
  - `missing_data_finding` true when the entity has no provenance by the
    predicate.

  A component's existing entry keeps its id, category and form. Only its record
  and its flag follow the predicate.
- Diagnostics, for the entities the predicate finds without provenance:
  - `COMPONENT_PROVENANCE_MISSING` (`:220-236`), whose message becomes
    "Component {label} has neither a provenance_record nor a component.provenance
    note; report provenance is incomplete for this component.";
  - `MATERIAL_PROVENANCE_MISSING` ("Material {label} has neither a
    provenance_record nor a provenance note; report provenance is incomplete
    for this material.", remediation "Enter material source/provenance before
    relying on this report.");
  - `SECTION_PROVENANCE_MISSING`, likewise;
  - all with class `PROVENANCE_WARNING` and severity warning.

The renderer's Provenance Notes table shows source, location, license and
contributor (`report_renderer/src/lib.rs:794-831`); its User-Supplied Values
table shows value, category, quantity, source and the missing-data flag. Neither
shows `review_status`. The other fields drive the screener (§4.3).

### 4.2 Library references

- `RenderableReportInput` gains `library_references:
  Option<Vec<LibraryReference>>` with `LibraryReference { reference_id,
  library_kind, library_id, library_version, record_id, entity_refs: Vec<String>,
  provenance }`.
- The builder takes the model's references from `libraryRefSites`
  (`libraryImportService.ts:440`), the walk the open-time check and the save
  screen use. One entry per distinct `(library_kind, library_id,
  library_version, record_id)`, in first-occurrence order, `reference_id`
  `library-ref:<n>`; an absent version or record id is "not recorded";
  `entity_refs` are the owning ids, sorted and unique; `provenance` is the
  session provenance.
- The renderer adds, after Provenance Notes, subheading "Library Records" and a
  table with the columns "Library kind", "Library", "Version", "Record" and
  "Referenced by". None or an empty list emits nothing.
- **Findings.** The builder takes a new argument, `libraryRefFindings`; its
  callers pass `currentLibraryRefFindings(model)` (`libraryRefFindings.ts:37`).
  Each finding becomes a `report_sections.diagnostics` entry: `code` the
  finding's, `class` `PROVENANCE_WARNING`, `severity` warning, `source`
  `{diagnostic_source, <LIBRARY_REF_CHECK_SOURCE>}` (`:15`), `affected_object`
  `{model_entity, <entity_ref>}`, `message` the finding's, `remediation`
  "Compare the library reference with the local library store before relying on
  this report.", and the session provenance. Resolution is the desktop's
  existing store check; the report adds no second resolution rule.
- Safe metadata only: no library payload or record snapshot enters the report
  (R-08-01-008, REXC-REQ-009). `library_version` is the recorded
  "sha256:<hex>" digest.

### 4.3 Disposition

A record with `protected_suspected`, `quarantined` or `rejected` is refused in
every context, as the package save refuses the model that carries it. A
structured source (a `provenance_record`, or an object- or array-valued note)
keeps the blocking dispositions it holds at any depth, and a malformed
disposition member refuses the build. Private records (`private_only` and every
other non-public class) render only in a local-private report. A record whose
redistribution, review or privacy status is `unknown` or `TBD` renders in a
local-private report with a non-blocking warning naming the record and the
field, and blocks in every other context (ruling "C1 b", §0). No redaction
applies to authored record text: the webview screener's concrete-path rule
(`redaction-controls/redactionExportControls.ts:438`) judges key names only
(`CONCRETE_PATH_DETAIL_KEYS`) and never reads a `source_location` value, so
source text the user authored, a typed path included, stays as written in the
local-private report and package and reaches no other output (ruling "C2 a",
§0): a render refused for its provenance records returns no document body. §6.2 states where each rule is enforced.

### 4.4 What changes on the open pressure-free route

FR-REP-003 applies on every route, so slice (c) changes today's pressure-free
rendered report and the package's rendered members whenever the model has
materials, sections, records or library references. The renderer's new
`library_references` is absent without references, and `exact_route` is absent
on this route. Committed witnesses that contain materials or sections are
regenerated in that slice (§7). The package version stays 2.0.0: no member form
changes.

## 5. What opening would change (for the owner)

### 5.1 The refusal points

After slices (a)-(c), opening is these edits and nothing else:

| # | Place | Change |
|---|---|---|
| 1 | `outputPolicy.ts:85` `EXACT_ADMITTED_SURFACES` | add `rendered-report` and `report-package` (the `pressure` and `pressure_source` entries both use `exactRouteSurfaces`, `:94,100`) |
| 2 | `outputPolicy.ts:43,48` refusal texts | "only the result JSON and stress-neutral exports, the rule check, the rendered report and the report package admit a numerically eligible result" (otherwise unchanged); the module comments `:13-31` and `loadReferenceOutputAvailability.ts:10-15` |
| 3 | `report_package/src/document_form.rs:17` `ADMITTED_SEMANTIC_CONTRACTS` | add `PRESSURE_ID` and `PRESSURE_SOURCE_ID`; message `:48` names the admitted set |
| 4 | `src-tauri/src/report_render_exact.rs` `RENDER_ADMITTED_EXACT_CONTRACTS` (§3.6 step 0) | the native render's admitted set: add `PRESSURE_ID` and `PRESSURE_SOURCE_ID`; the `REPORT-RENDER-ROUTE-NOT-ADMITTED` message names the admitted set |
| 5 | `reportPackageRequest.ts` `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS` (§3.5) | the same three ids as item 3 |
| 6 | native smoke `driver.js` `exactRouteGates` (`:548-576`) | after the exact solve, render the report under the local-private selection with `renderLocalPrivateReport` (`:510`), and save and re-read the package with `saveReportPackage` (`:177`) |
| 7 | native smoke `run.mjs:277-283` | the two exact report checks become admission checks of what the driver did: the render's canonical HTML hash and passed export gates, and the saved package's read-back with its exact-identity result member. Changing these assertions without item 6 would show only that the controls are offered, not that admission succeeds |
| 8 | tests pinning the refusals | `outputPolicy.test.ts`, `reportPackageRequest.test.ts:117-163`, `report_package/tests/reader.rs:1175`, `tests/current_result_package.rs` and the W-11 still-refused tests (§7). The production-entry-point refusal tests of §5.5 become admission tests. So do the item-0 cases of the direct-native save tests and the production render's step-0 refusal test (§7, slice (b)). The test-only `EXACT_WITNESS_ADMITTED` and `EXACT_RENDER_WITNESS_ADMITTED` are removed, since the production sets now equal them; the forwarding-graph scans (§5.5) then assert that no test-only set exists. The direct-native render tests and the exact render's whole-command memory check then run through `render_calculation_report_args`, and the exact witness save's whole-command memory check through assembly (§7) |
| 9 | commitment, design and guide texts | Appendix A.2, O1-O14 |

The report JSON packet keeps refusing; the cell for `report` is unchanged. No
package version change is needed: the manifest and member forms are unchanged,
and a build from before opening refuses an exact member by name
(`REPORT-PACKAGE-ROUTE-NOT-ADMITTED`), which DEC-028 allows.

### 5.2 An opened exact-route report

In the renderer's fixed order (`report_renderer/src/lib.rs:33-42`):
- model input summary and load cases, unchanged;
- results: the exact-route block (identity, standing line, load/reference line
  where present, both headlines), then the rows with their basis suffixes;
  an exact report carries no numerical-health line (§6.1);
- warnings, assumptions and provenance: the result diagnostics, provenance and
  library-finding warnings, the FR-REP-003 values and notes, Library Records;
- audit manifest, and rule-pack references with user-rule outcomes, unchanged;
- limitations: technical preview, the producer's statements (§2.3) in order,
  the profile reference;
- the PRD §19.3 notice and the human review record, and the local-private
  notice.

### 5.3 An opened exact-route package (2.0.0)

- `package_manifest.json`, unchanged form;
- `canonical_rendered_report_html` and `…_pdf`: §5.2, under `local_private`,
  `generatedAt: null`, with the session's rule-check run, and without
  numerical-health lines (the package passes none);
- `audit_manifest.json`, unchanged (DEL-08-02);
- `result_export_<envelope id>.json`: the DEL-08-04 0.3.0 exact-route document
  Export Result JSON saves (`load_case` load bases, `contract_evidence` with any
  `load_reference_states`, the rule-pack exclusion diagnostics of #1613, and the
  producer's `formulation_basis.limitations` as published, the source paragraph
  whole: it is a data member, not a report body);
- `state_comparison_handoff_sections`: the state/run sections and, when chosen,
  the within-run comparison (DEL-08-06).

The bindings check the exact sections against the source (§3.5). Before the
dialog, the native save binds the request's source and model to the bytes its
solve job delivered and received, and it checks custody again before the write
(§3.6). The producer and the reader refuse a runner-profile exact package by
its profile (§3.5).

### 5.4 What stays refused

- The **report JSON packet** (`report`, `ReportPanel.tsx`) picks fixed demo ids
  and one headline (`selectedResultRefs`, `:297-305`). Its retirement or
  redesign is a separate item.
- The **handoff preview** and the other fifteen surfaces
  (`desktop-exact-outputs.md` §4.5). The model handoff (W-6b) needs no result.

### 5.5 Evidence for the return, and the test-only assembly

The witness shapes are pressure-2 with and without load/reference states, with
a connector and with a bend, and pressure-source-1. Each is built from the
committed exact corpora below the gate.
- The coverage fixture, the statement table and their tests, and the claims
  checks (§7).
- **A rendered local-private exact report** (HTML and PDF) of each shape. It
  comes from `report_renderer` directly, a library call in a test. The
  library has no route refusal. Its production callers are the desktop's
  native render and the package's assembly (`report_package/src/lib.rs:212`),
  and both sit behind native identity boundaries.
- **A complete 2.0.0 package of each shape, through a test-only assembly.**
  The production boundary refuses exact identities at item 0, so the witness
  cannot pass through it.
- **The admitted-set type.** A new module, `report_package/src/admitted.rs`,
  defines `pub(crate) struct AdmittedSet(&'static [&'static str])`. Its field
  is private to the module, so no other code can construct a value. The
  module defines two values:
  - `PRODUCTION`, which wraps `ADMITTED_SEMANTIC_CONTRACTS` and is compiled in
    every build;
  - `#[cfg(test)] EXACT_WITNESS_ADMITTED`, which holds the three identities
    and exists only in the crate's own test build.

  In a non-test build `PRODUCTION` is therefore the only `AdmittedSet` there
  is. The compiler guarantees that every variant below receives the
  production set there, whoever calls it.
- **The forwarding graph.** The admitted set reaches item 0 only along these
  edges. Every `_in` variant is `pub(crate)` or private, so no dependent can
  call it. It takes `admitted: AdmittedSet` as its last parameter.

  | Variant (file) | Allowed non-test callers | Argument |
  |---|---|---|
  | `document_form::check_route_admitted_in` (`document_form.rs`) | `check_route_admitted`; `wire::assemble_wire_request_in`; `bindings::check_request_in`; `assemble_report_package_container_in`; `reader::verify_members` | `PRODUCTION` from the first, `admitted` from the others |
  | `wire::assemble_wire_request_in` (`wire.rs`) | `assemble_wire_request` | `PRODUCTION` |
  | `bindings::check_request_in` (`bindings.rs`) | `bindings::check_request`; `wire::assemble_wire_request_in` | `PRODUCTION` from the first, `admitted` from the second |
  | `assemble_report_package_container_in` (`lib.rs`) | `assemble_report_package_container`; `wire::assemble_wire_request_in` | `PRODUCTION` from the first, `admitted` from the second |
  | `reader::read_report_package_container_in` (`reader.rs`) | `read_report_package_container` | `PRODUCTION` |
  | `reader::verify_members` (`reader.rs`, private; gains the parameter) | `reader::read_report_package_container_in` | `admitted` |

  These edges mirror today's calls of `check_route_admitted`: at
  `wire.rs:282`, at `bindings.rs:194` from `wire.rs:285`, at `lib.rs:241` from
  `wire.rs:304`, and at `reader.rs:930` in `verify_members` (`:223`). The
  graph's production roots are the five public entry points:
  `check_route_admitted`, `assemble_wire_request`, `bindings::check_request`,
  `assemble_report_package_container` and `read_report_package_container`.
  Each passes `PRODUCTION` and nothing else. An internal edge passes its own
  `admitted` parameter, unchanged. The runner-profile check (§3.5) reads no
  set and is outside the graph. The desktop (`report_package_bridge.rs:11`,
  `native_smoke.rs:146`) and the headless runner
  (`openpipestress-runner.rs:518`) call only public roots.
- **The scan.** `admitted_set_forwarding_graph_is_closed`, in
  `report_package/src/admitted_tests.rs` (a `#[cfg(test)]` module), reads every
  `src/**/*.rs` file through `CARGO_MANIFEST_DIR`. It scans only non-test
  source. A non-test source is each file with every column-0 `#[cfg(test)]`
  item removed: the item runs to its first column-0 `}` line, or to the end of
  its line if that line ends with `;`. A file that a `#[cfg(test)] mod
  <name>;` declaration names is removed whole. An indented `#[cfg(test)]` is
  kept, so anything it calls counts as a non-test call. The scan fails closed
  if an item's end is not found. It asserts:
  1. each variant's set of enclosing functions with a call site equals its
     row's allowed callers, exactly. The enclosing function of a call is the
     nearest preceding `fn <name>` line at a lesser indentation;
  2. a root's call passes `admitted::PRODUCTION` as its last argument (each
     file imports `crate::admitted`). An internal edge's call passes the
     identifier `admitted`. The argument is read from the call's balanced
     parentheses;
  3. `AdmittedSet(` occurs in non-test source only in `admitted.rs`, in the
     struct and in `PRODUCTION`'s definition;
  4. `EXACT_WITNESS_ADMITTED` occurs only in its `#[cfg(test)]` definition and
     in the witness module;
  5. no variant is declared `pub fn`;
  6. a variant's name occurs in non-test source only in its definition and in
     direct calls (`name(`). An alias, a function value, a `use` of it or a
     re-export is a finding, so the set cannot leave the graph through a name
     the scan does not follow.

  The rule is `report_package/src/forwarding_graph_scan.rs`, a `#[cfg(test)]`
  module that the desktop render's scan includes by path. The same module has
  a negative test of the scanner on invented sources. A sixth caller, a root
  passing `admitted`, an internal edge passing `PRODUCTION`, a witness-set use
  outside the witness module, an alias of a variant in another function (with
  every edge intact), a variant passed as a function value and a re-export
  each produce a finding.
- **The witness.** `report_package/src/exact_package_witness_tests.rs` is a
  `#[cfg(test)]` module. It must be in the crate, because `#[cfg(test)]` items
  are visible neither to `tests/` integration tests nor to dependents. It
  calls `assemble_wire_request_in` with `EXACT_WITNESS_ADMITTED` for each
  witness request. That call forwards the witness set along the graph to
  `check_request_in`, `assemble_report_package_container_in` and their item-0
  checks, so every item-0 check on that path admits the witness. It then reads
  the package back with `read_report_package_container_in` and the same set,
  and checks every member.
- **No conditional path.** There is no cargo feature. **This design narrows
  the claim, rather than compiling the variants conditionally.** The `_in`
  variants are compiled in every build, release included, because production
  calls them; that is why the witness exercises the production code.
  Conditional compilation would give the release a path the witness never
  runs. What no non-test build compiles is the widened set: the value
  `EXACT_WITNESS_ADMITTED` and the witness module, its only user.
- **The native render's witness set**, by the same rule (§3.6 step 0).
  `apps/desktop/src-tauri/src/report_render_exact.rs` defines
  `pub(crate) struct RenderAdmittedSet(&'static [&'static str])`, with a
  module-private field. It has two values: `PRODUCTION`, which wraps
  `RENDER_ADMITTED_EXACT_CONTRACTS` (empty), and `#[cfg(test)]
  EXACT_RENDER_WITNESS_ADMITTED`, which holds `PRESSURE_ID` and
  `PRESSURE_SOURCE_ID`. The graph:

  | Function (file) | Allowed non-test callers | Argument |
  |---|---|---|
  | `render_calculation_report_args` (`lib.rs`; production root) | `render_calculation_report` (the command, `lib.rs:1540`) | none |
  | `report_render_exact::render_calculation_report_args_in` | `render_calculation_report_args` | `report_render_exact::PRODUCTION` |
  | `report_render_exact::check_render_route_admitted` (step 0) | `report_render_exact::render_calculation_report_args_in` | `admitted` |
  | `render_calculation_report_value` (`lib.rs:1553`, private) | `report_render_exact::render_calculation_report_args_in`, after step 0 and, for an exact input, steps 1-5 and the re-derivation | none |

  `render_calculation_report_args_in` and `check_render_route_admitted` take
  `admitted: RenderAdmittedSet` as their last parameter, and the first runs
  step 0 against it first.
  `render_admitted_set_forwarding_graph_is_closed`, in
  `src-tauri/src/report_render_exact_tests.rs`, applies the same scan rule to
  the desktop crate's `src/**/*.rs`. That rule handles the crate's scattered
  `#[cfg(test)]` functions (`lib.rs:154,1848,2243,5172`). It asserts the four
  rows above and the analogues of assertions 2-6. `EXACT_RENDER_WITNESS_ADMITTED`
  may occur only in its definition and in `report_render_exact_tests.rs`. The
  direct-native render tests (§7) call `render_calculation_report_args_in`
  with `EXACT_RENDER_WITNESS_ADMITTED`. The desktop's package save calls only
  `report_package`'s public roots, so it receives the production set in every
  build.
- **Every production entry point still refuses.** The same test passes each
  witness request to `assemble_wire_request`, `bindings::check_request` and
  `assemble_report_package_container`, and the witness package bytes to
  `read_report_package_container`. Each refuses with
  `REPORT-PACKAGE-ROUTE-NOT-ADMITTED`. The headless runner's package path
  (`core/runner/headless/tests/del1005_package_witness.rs`) refuses the same
  requests likewise. The desktop's `admit_report_package`
  (`src-tauri/src/report_package_save.rs:591`) refuses them at the native
  binding first, because their jobs are not live
  (`REPORT-EXACT-CUSTODY-RELEASED`). With a live test job and its genuine
  texts it refuses at item 0 (§7, slice (b)). The production native render,
  `render_calculation_report_args`, refuses a genuine exact render input from
  a live test job with its genuine texts at step 0, with
  `REPORT-RENDER-ROUTE-NOT-ADMITTED` (§7, slice (b)). Slice (a) already pins
  its refusal of a coherent invented exact input (§7).
- **Witness requests.** `buildReportPackageRequest` becomes the cell check
  followed by `assembleReportPackageRequest(input, admittedContractIds)`.
  Production passes `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS`. The assembly step
  is exported only under the test-only name
  `assembleReportPackageRequestBelowOutputGate`, as in §3.1; the Rust boundary
  still refuses whatever it produces. vitest calls it with the three
  identities, under the native-capture test seams. It writes the requests to
  `fixtures/reports/invented/exact_report_package_requests/` and fails if they
  are stale; `OPENPIPESTRESS_WRITE_EXACT_PACKAGE_REQUESTS=<dir>` writes them.
  Their job ids and byte digests are invented, and the shared producer checks
  only their form (§3.5). The direct-native tests set them to a live test job's
  (§7). The same vitest checks that `buildReportPackageRequest` refuses each
  witness input.
  - **The substitute.** The same vitest also writes
    `pressure_2_straight_substitute.overlay.json`, leaf replacements over the
    pressure-2 straight witness request that the tests apply (`leafOverlay`),
    and checks that the overlay reproduces exactly the substitute rebuilt by
    the same builders from a copy of the pressure-2 straight source with one
    row value changed, so that every hash in the document, report and handoff
    records is rebound. It keeps the genuine request's `native_job_id` and
    `received_bytes_sha256` (§7).
- **No rule-check run.** The witness inputs carry no rule-check run
  (`userRuleChecks` absent and no aggregate), so no W-11 fixture holds
  rule-check outcomes (§1.7).

## 6. Slices and size

| Slice | Content | Size | Persistence / data boundary |
|---|---|---|---|
| **(a) Rust producers, renderer and the statement table** | The fixture writer on the public producer, its guard and the committed fixture (§2.2); no product source changes. `exactReportLimitations.ts` and its test, which writes `exact_report_statements.json`, and the Rust table that embeds it (§3.5); the TypeScript module is imported by nothing until (b). The renderer's `exact_route` (with the admission-binding check and the composed standing line, §3.2), its exact-limitation check, and `library_references`, in `assemble_report_sections`, so HTML and PDF both emit them. Render and emit tests, including byte-identity when absent. In `report_package`: the row suffix in `expected_report_result_rows`, the dormant item 9 and the public `exact_sections` projection (§3.5), the runner-profile refusal in the producer and the reader (§3.5), the `AdmittedSet` type, the crate-private admitted-set variants on their forwarding graph, and the graph scan with its negative test (§5.5), and both parity vector files. The native render's step 0 (`report_render_exact.rs`, `RENDER_ADMITTED_EXACT_CONTRACTS` empty, `REPORT-RENDER-ROUTE-NOT-ADMITTED`) with its witness set and graph scan (§3.6, §5.5), moved here from (b) because the renderer's `exact_route` makes exact reports reachable through the unchanged native render command (Codex review round 1 of #1632). Hosted-CI routing | L (~1.9k lines with tests, plus the generated fixtures and vectors; revision 4: ~1.8k) | None. Numerical meaning untouched. Package and report bytes unchanged for every current input. The native render refuses every exact-route input at step 0 |
| **(b) Desktop exact sections and admission, behind the gate** | `exactReportAdmission.ts` (§3.1); the exact path of `buildRenderableReportInput` (admission, limitations, profile reference, binding, load/reference line, headlines); the row suffix in `reportResultRows`; `RenderedReportPanel.tsx:171` at surface level, the panel's admission and its pre-IPC re-check; the request builder's split, its pre-IPC re-check and `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS`; the native source binding (§3.6): the job's custody digests and `custody_binding`, `deliveredIntakeText`, the resubmitted texts, their priced parses, the native standing, the save's source and model equalities and the render's re-derivation; the `rendered-report` disclosure template; still-refused tests, including the production native render's refusal of a genuine exact result; the witness and substitute requests, the in-crate witness and production-refusal tests (§5.5) and the direct-native substitution tests (§7). The DEL-04-07 memory-budget updates (§8). Texts A.1 G2 and G5 | L (~2.4k, plus the witness requests; revision 4: ~2.2k) | None before opening. The webview cells refuse exact results, and the native render (step 0) and the package's Rust boundary (item 0) refuse every input that carries an exact identity in an identity field, so no production path reaches an exact output. The job registry keeps 65 bytes of digest metadata per job, released with custody. The admission and the native binding are an output gate with new priced parses, so they get targeted regression checks and independent scrutiny |
| **(c) FR-REP-003 on every route** | Material, section, component and support records under one provenance predicate; library references and finding diagnostics; callers pass the findings; the schema check of representative records (§7); regenerated witnesses and witness requests; the memory-budget figures re-measured. Texts A.1 G1, G3 and G4 | M (~0.8k, plus regenerated witnesses) | **Data boundary**: new private content in the open pressure-free report and package |

(a) comes first, because Rust drops unknown report fields (§1.1 item 3) and
item 9 reads the statement table. (b) and (c) follow in either order and may
share a PR. Opening (§5.1) is an owner act after them. It is about 0.3k lines
with its tests, the native-smoke driver and the texts (revision 5 adds the
native render's admitted set, a few lines).

### 6.1 Slice (b) as built

Slice (b) follows §3.1-§3.6 and §5.5 with these differences, each fail-closed:
- **Handoff load bases.** The exact document names its load bases `load_case` (DEL-04-07 Design `desktop-exact-outputs.md` §4.2), while the handoff run section keeps the record's `LoadCase`. Item 8's "handoff load basis equals the document's" (`bindings.rs`) would refuse every coherent exact package, so it compares under that one mapping for exact identities only. Pressure-free packages are unchanged; item 0 still refuses exact identities first.
- **Save order.** The save cannot know a request is exact before it parses it, so steps 1-5 run after the evidence, size and screening-payload checks and the request parse, and before every content check (model, input manifest, screen, context, rule packs, producer). A pressure-free request that carries exact texts refuses with `REPORT-EXACT-SECTIONS-MISMATCH: exact_route`, as the render does.
- **No numerical-health line in an exact report** (Codex review round 1 of #1637). The panel's numerical-health lines are derived in the webview from the reader's findings and the session's verdict hold; the native render cannot re-derive them from the delivered result without that logic. The builder therefore adds none to an exact report, whatever the panel supplies, and the render's re-derivation requires the rows to equal `expected_report_result_rows` exactly, so a fabricated line refuses (`REPORT-EXACT-SECTIONS-MISMATCH: result_rows`). The package carries none either: item 6 already requires its rows to equal the source's projection. A native re-derivation of these lines is a separate item; pressure-free reports keep them.
- **Admission snapshot** (same review). `admitExactReportSource` takes the four inputs' checked text before its first await and re-checks it, with the live invocation, standing, custody and received bytes, after its last await, before it returns the admission; an edit made after Export Result JSON admitted the inputs makes the admission stale.
- **`model_sha256` mutated in a render input** refuses at step 5 (`REPORT-EXACT-SOURCE-NOT-DELIVERED: model`), as §3.6 orders it; §7 states this.
- **Same-run substitution in the builder.** A recorded admission offered for other objects refuses as stale; an unrecorded admission refuses as required (§3.1, §7).
- **Render thread.** The render command runs every input on a blocking worker thread, so that an exact input's binding is off the main thread; a pressure-free input renders byte-identically there.
- **Witness shapes.** The five witness requests are pressure-2 straight (the straight corpus's input), pressure-2 with load/reference states, pressure-2 with an objective connector (T4-U5b AS-F, the connector corpus's pressurized tied joint and its smallest connector input), pressure-2 with a realized bend (U2-L-ANCH-PTW-K2 0.3.0, the arc corpus's input) and pressure-source-1. The connector and bend corpora commit no request model a desktop session can solve, so `core/product_physics/tests/exact_route_limitations.rs` (`exact_report_witness_inputs_are_current`, test code only) writes their re-authored requests and the bytes the desktop's solve worker writes for them (`fixtures/reports/invented/exact_report_witness_inputs/`, stale fails). Each shape is assembled by the desktop below the gate, bound natively with a live job, rendered (HTML and print view) under the render's witness set, and assembled (HTML and PDF members) and read back under the package's. The witness requests embed the producer output number for number: the vitest writes them with a number-preserving writer (`apps/desktop/src/test/losslessJson.ts`) that takes the source result, and each document member copied from it (its `source_block_recovery` receipt among them), from the delivered bytes, so the corpus-wide receipt check (`result_export/tests/hash_corpus.rs`) holds, and its staleness check uses the same writer. The substitute is committed as leaf replacements over the straight witness's request (`pressure_2_straight_substitute.overlay.json`); it changes one nodal rotation row, which no section-basis pair or magnitude row repeats, so its standing stays eligible.
- **Shared projection.** The TypeScript projection is `features/report/exactReportSections.ts` (the headline labels move there and `ResultsPanel.tsx` imports them); item 9's limitation comparison is the public `exact_sections::check_exact_limitation_entries`, which the native render reuses. The parity vectors also carry `admitted_semantic_contracts`, which `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS` must equal.

### 6.2 Slice (c) as built

Slice (c) follows §4 with these differences, each fail-closed:
- **The render route's disposition blocks are the renderer's.** The webview screener's rule (`redactionExportControls.ts:438-444`) never sees a report record's statuses on DREP-IPC-003: the route projection (`controlRouteExport`, `projectRouteValue`) turns every record into opaque leaves classified by path, so a `quarantined`, `rejected` or `protected_suspected` record passes the local-private control (`renderedReport.test.tsx`, "carries each record's disposition verbatim"). Before this slice no report record carried a status other than `pending`. The renderer therefore refuses export (`report_renderer` `provenance_disposition_blocking_reasons`; the blocking values are the package save's `SCREENED_MEMBERS`) for: a report-section record, a user-supplied value or an assumption whose own `review_status` is `quarantined` or `rejected` (`REPORT_PROVENANCE_DISPOSITION_BLOCKED`); and a library reference's record with those statuses or failing the section validation's own provenance rules, incomplete or `protected_suspected`, and outside local-private also `unknown` or `TBD` (`report_sections::validate_provenance_record`, judged in the report's context as below). A report-section record with `protected_suspected` was already refused by the section validation (`REPORT_SECTION_PROTECTED_CONTENT_SUSPECTED`). Each reason names the record by its collection position and the member only (`user_supplied_values[3]`, `assumptions[0]`, `library_references[2].provenance`), and says to review that record's disposition in the model. Ids, labels and source text are user-authored (the model schema admits any non-blank identity), so none enters a reason (Codex review round 3 of #1639); the panel resolves a position to its entity locally from the input it sent, as display text only (`provenanceReasonEntities.ts`). The refusal covers the HTML, its print view, the PDF and the package's rendered members, in every export context; a refused render shows no preview, save or print. The package save also refuses these records first, in its structural screen of the request and the model (`report_package_save.rs`).
- **Structured sources are screened before they are flattened** (Codex review round 1 of #1639). An object-valued section note is written into the record's `source_location` as RFC 8785 text, and a mapped `provenance_record` copies only its seven text fields, so a disposition inside either would otherwise be lost while the package save, which screens the model at every depth, refuses it. The builder screens each `provenance_record` and each non-string note at every depth with the package save's `SCREENED_MEMBERS` (`renderableReportInput.ts` `screenDispositions`, `withSourceDispositions`): a `quarantined` or `rejected` value anywhere becomes the record's `review_status`, `protected_suspected` anywhere its `redistribution_status`, so the renderer refuses it; a disposition member whose value is an array or object refuses the build with `REPORT-PROVENANCE-DISPOSITION-MALFORMED`, naming the entity and member, as the package save refuses it as malformed. A top-level `redistribution_status` of `rejected` is copied, and the renderer's input refuses it (`RENDER-INPUT-INVALID`) as the package's report decode does.
- **Private records render only under `local_private`** (same review). A direct call of the native render command bypasses the webview's context control, so the renderer refuses any report record whose privacy class or redistribution status is not public when the report's export context is absent or not `local_private` (`REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE`; `report_render_exact_tests.rs` `the_native_render_refuses_private_records_outside_local_private`). Invented public fixtures still render without a context.
- **A provenance refusal is metadata-only** (Codex review round 2 of #1639). A render refused for its provenance records, that is with any `REPORT_PROVENANCE_CONTEXT_NOT_LOCAL_PRIVATE` or `REPORT_PROVENANCE_DISPOSITION_BLOCKED` reason, a library reference's section-rule refusal, or a report-section record refused as `REPORT_SECTION_PROTECTED_CONTENT_SUSPECTED`, returns before anything is assembled: `content_withheld` true, `html` empty (its SHA-256 is the empty document's), no lint findings, each section diagnostic with its code and severity and a fixed withheld message, and the blocking reasons (codes, positions and members; no id, label or record text), which are all the package manifest and the headless runner's output carry of it (`authored_ids_never_enter_a_withheld_outcome`, `a_refused_package_carries_no_authored_id`, `the_native_render_response_names_no_authored_id`, headless `refused_package_ids.rs`). The PDF emitter emits no bytes for it (`pdf_from_render`), the full package assembly therefore carries empty members (and refuses as blocked), and the native command derives no print view (`render_calculation_report_value`). A rendered outcome serializes without the member, so every other outcome is unchanged. Other blocked renders (report validation, report context, exact-route, lint) keep their diagnostic document as before. One existing behaviour changes: a report-section record with `protected_suspected`, blocked before this slice with its document returned, is now refused metadata-only, because its content is suspected protected. The panel already shows no preview, save or print for any blocked render; it now receives no body either (`report_renderer` `a_typed_path_stays_local`, `pdf_emitter` `the_desktop_builder_records_block_the_pdf_by_disposition_and_context`, `report_render_exact_tests.rs` `the_native_render_refuses_private_records_outside_local_private`).
- **A renderer input that does not decode** is refused with its JSON path and the error kind only, `RENDER-INPUT-INVALID: <kind> at <path>` (`report_renderer` `decode_renderable_report_input`, used by the native render), never with the offending value or key: a typed path given as a disposition value is not echoed (Codex review round 3 of #1639, `an_undecodable_input_names_its_path_and_kind_not_its_value`). The decode moves each top-level member out and decodes it as the whole decode did, so the input is never copied; before that it decodes the provenance-bearing elements by reference one at a time, holding at most one element's copy, to place an error down to a disposition member (the measured render peaks are unchanged). A valid input decodes exactly as before (`the_value_free_decode_equals_the_whole_decode`). The message text changes for every undecodable input; only its `RENDER-INPUT-INVALID` prefix was pinned. The package path is treated the same: the report's decode (`REPORT-PACKAGE-REPORT-DTO-INVALID: <kind> at report.<path>`, through the same decode), the request's typed decode in the native save and the headless runner (`<kind> at <member>`, `wire::decode_report_package_request` and `report_package_save::typed_request`), the exact-route section's decode in the package bindings (kind only), and the runner's CLI input decode (kind, line and column for a data error); each reads a positional (array) form as serde does.
- **Unresolved records warn in local-private only** (ruling "C1 b", §0). The section validation takes the report's context (`report_sections::ProvenanceContext`, `validate_report_sections_in`; the renderer derives it from `report_context.export_context`, `provenance_context`). Under `local_private` a record whose redistribution status is `unknown` or `TBD`, or whose review or privacy status is `TBD`, gives a non-blocking `REPORT_SECTION_PROVENANCE_UNRESOLVED` warning, class `PROVENANCE_WARNING`, which the renderer appends to the warnings section's Diagnostics table (`unresolved_provenance_warnings`): it names the record (`user_supplied_value <value id>`, `provenance <source name>` or the library reference and its owners) and each unresolved field, and says the source's status is not recorded. In every other context, a public report, an absent context and any later shared context, the same record stays report-blocking; `validate_report_sections` without a context is that strict form, so no caller gets the relaxation without the local-private context. The package renders its members under `local_private` (its bindings refuse any other context), so a local-private package shows the same warning. Which records are unresolved: a structured `provenance_record` (on a material, section, component or support) that records no redistribution status maps to `unknown` (§4.1), whatever the entity kind; a legacy `provenance` note without a record keeps the `private_only` fallback. Both the record editor's "Not recorded" choice (`ModelDataAnnotations.tsx:154,161`) and a library apply over values whose previous provenance is missing (`libraryApply.ts:202-212`, which records `unknown`) produce such a record. `quarantined`, `rejected` and `protected_suspected` still block in every context.
- **Concrete paths.** The screener's concrete-path rule (`:438`) judges key names only (`CONCRETE_PATH_DETAIL_KEYS`); it never reads a `source_location` value. Source text the user authored in a record or note, a path included, therefore stays as written in a local-private report and package, as the component note's text already does and as the model document save writes it (ruling "C2 a", §0). No value-level path rule is added. No other output carries the records: the webview's public render is blocked before IPC, the report packet carries none, and the native render refuses private record content outside `local_private` with a metadata-only outcome (no document, print view or PDF bytes) (`a_typed_path_stays_local`, `the_native_render_refuses_private_records_outside_local_private`, `renderedReport.test.tsx` "keeps a typed source_location path as written locally").
- **The predicate on an object-valued note.** A section whose `provenance` is an object has provenance only when the object has a member; `{}` is "section provenance missing" and a missing-data finding.
- **Absent members.** Every mapped record member is copied trimmed and only when non-empty after trimming. A library reference's absent or empty kind or library id is "not recorded", like its version and record id.
- **No support diagnostic.** §4.1 names component, material and section diagnostics only; a support record without provenance carries its missing-data flag and no diagnostic.
- **Witnesses.** The five exact-route witness requests (`fixtures/reports/invented/exact_report_package_requests/`) each gain their model's material record and value, and with them the report-sections checksum; nothing else in them changes, and the substitute overlay is unchanged. `del1005_export_results_success_input.json` does not change: the Rust test basis (`report_package/tests/common/package_basis.rs`) builds its report with invented provenance and no desktop records, so §1.7's regeneration is a no-op. The shared component projection (`component_provenance_cross_layer_projection.json`) changes the missing component's diagnostic message, written by its vitest's new write mode (`OPENPIPESTRESS_WRITE_COMPONENT_PROVENANCE_PROJECTION`).
- **Citations moved.** The component record and predicate are `renderableReportInput.ts` `componentProvenanceRecord` and `hasProvenance`, `libraryRefSites` is `libraryImportService.ts:440-457`, and the renderer's provenance tables are in `assemble_report_sections` (`report_renderer/src/lib.rs:846-950`).

### 6.3 Opening as built (§5.1)

The opening follows §5.1 items 1-9 under the ruling of §0, with these
differences, each fail-closed:
- **The headless runner stays refused** (§5.4: `headless-runner` is one of
  the fifteen refused surfaces). Before opening, its package path refused
  exact identities only through the shared producer's item 0, which now admits
  them under the desktop profile. The runner's `export-results` therefore
  refuses a result document with an exact identity under any export profile,
  before its other bindings, with the code it gave before opening
  (`REPORT-PACKAGE-ROUTE-NOT-ADMITTED: the headless runner packages no
  exact-route result; …`; `openpipestress-runner.rs`,
  `del1005_package_witness.rs` `the_runner_package_path_refuses_the_exact_witness_requests`).
  The shared root still refuses them under the runner profile
  (`REPORT-PACKAGE-EXACT-RUNNER-PROFILE`). Why: §5.4 keeps `headless-runner`
  refused, and only the desktop has the Export Result JSON admission and the
  native source binding (§3.5, §3.6); the refusal keeps the code the runner
  gave before opening, and the test pins it by driving the runner binary on
  each witness request and the same-run substitute, under both profiles.
- **No test-only set remains.** `EXACT_WITNESS_ADMITTED` and
  `EXACT_RENDER_WITNESS_ADMITTED` are removed, and the scans' assertion 4 now
  checks that no set other than `PRODUCTION` is constructed or declared in any
  source, test code included (the scans' own invented sources excepted). The
  in-crate witness module stays in the crate and calls the public roots only;
  the test-only names `assembleExactReportBelowOutputGate` and
  `assembleReportPackageRequestBelowOutputGate` are removed (their callers use
  `assembleRenderedReport` and `buildReportPackageRequest`). The witness test
  file keeps its name, which the committed witness requests record as their
  generator; only the requests' `description` changed.
- **Route refusals that stay pinned** use an identity outside the admitted set:
  physics-retained-1 at the render's step 0, the package's item 0 (producer and
  reader, the reader's with the committed exact-successor derivative document,
  which is well formed) and the desktop identity check.
- **An exact render is local private natively** (Codex review round 1 of
  #1648, blocking). With genuine custody and texts, a render input whose
  provenance flags were relabelled public rendered unblocked under
  `public_report` or with no context, because the renderer's private-record
  refusal trusts those flags. The native render therefore requires
  `report_context.export_context == "local_private"` for every input with
  `exact_route`, after step 0 and before the source binding, and otherwise
  refuses the command (`REPORT-RENDER-EXACT-CONTEXT-NOT-LOCAL-PRIVATE`; no
  body and no print view, the lease not grown; the message names no input
  value) (`report_render_exact.rs` `check_exact_render_context`). The native
  save (`report_package_save.rs` `check_context`) and the shared producer
  (bindings item 8) already refuse any context but `local_private`
  (`REPORT-PACKAGE-REPORT-CONTEXT-NOT-LOCAL-PRIVATE`, no package), and the
  headless runner refuses exact identities outright. The regression
  (`report_render_exact_tests.rs`
  `an_exact_render_or_save_outside_local_private_refuses_whatever_its_provenance_flags`)
  covers pressure-2 and pressure-source-1 with relabelled provenance under
  `public_report`, another context, no export context and no report
  context, through the decoded args and the raw body, and the save and the
  shared producer.
- **Native inputs are object form at every depth** (Codex review rounds 2
  and 3 of #1648, blocking). serde, and the renderer's value-free decode, read
  a struct written as a JSON array by field order (its positional form),
  while step 0, the exact context check, the exact detection, the save's
  disposition screen and the source binding read members by key. Round 2
  found a positional render input carrying an exact section past all of them;
  round 3 found a positional provenance record (for example
  `calculation_report.provenance` with `review_status` `quarantined`) hiding
  its disposition from the save's screen while the renderer read it, so an
  unblocked package was produced. A path list of members cannot close this,
  and round 3's gap was live on the pressure-free route too, so the one
  shared check landed against main first (#1650) and is described in
  `report-package-current-results.md` §2.6: `report_renderer::object_form`
  compares the typed decode with the original JSON, following the decoder
  exactly. The opening's earlier path-list checks are removed; the exact
  route uses the same check at the same points, each before any keyed policy
  check:
  - the native render, before step 0 and so before the exact context check
    and the source binding (`report_render_exact.rs` `check_object_form`,
    `REPORT-RENDER-INPUT-POSITIONAL`; no body or print view, the lease not
    grown);
  - the native save, after its canonical parse and before its exact
    detection, source binding, screen and context checks
    (`report_package_save.rs` `check_request_object_form`,
    `REPORT-PACKAGE-SAVE-REQUEST-POSITIONAL`);
  - the shared producer, after item 0 and before the audit conversion and the
    bindings (`wire::assemble_wire_request_in`,
    `REPORT-PACKAGE-REQUEST-POSITIONAL`); the headless runner also refuses
    exact identities outright (above).

  The audit of the other native commands, the check's constraint on serde
  aliases and the remaining items (among them the stored-model recovery
  export's keyed screen over malformed stored rows, W-17's queued check) are
  recorded there and are not changed here. Pricing (DEL-04-07
  `memory-budget-decision.md`): the check's by-reference decode is held only
  during the check and is priced within the existing leases; on the exact
  witness the whole render command peaks at 6,642,400 bytes within its
  12,099,728-byte lease and the whole exact save at 40,620,336 within
  110,845,552, each less the buffered body's envelope (counting-allocator
  figures for these inputs, not proven bounds;
  `the_genuine_exact_render_and_save_stay_within_their_leases`). Exact-route
  regressions (`report_render_exact_tests.rs`, pressure-2 and
  pressure-source-1, decoded args and raw body):
  `a_positional_provenance_record_refuses_the_render_and_the_save_at_every_depth`
  (quarantined, rejected and protected-suspected records at every provenance
  depth of the witness report, the calculation report's own included, render
  and save; in object form the save's screen refuses them), `a_positional_render_input_refuses_before_any_check`,
  `a_positional_save_request_refuses_before_any_check` and the
  legitimate-arrays control
  `genuine_inputs_with_every_array_field_pass_the_object_form_check`; the
  producer's
  `positional_members_refuse_and_a_positional_request_assembles_the_same_package`
  and the headless binary test's positional variants. The pressure-free
  regressions and the check's own tests are §2.6's.
- **The exact save's memory check** now runs through the shared producer's
  assembly of the admitted package: priced peak 45,915,856 bytes within its
  110,845,552-byte lease, and 40,620,336 re-measured with the object-form
  check (the render's 6,642,400 within 12,099,728); DEL-04-07
  `memory-budget-decision.md` records them.
- **Deferred readability items (observations on the §5.5 evidence, not
  changed).** Each was checked against a pressure-free report of today (the
  committed pressure-free result `fixtures/results/preview_physics_connected_sparse.json`
  and the headless runner's pressure-free package of
  `del1005_export_results_success_input.json`); Codex review round 1 of #1648
  classified them as readability, not correctness, and suggested the
  improvement given for each:
  1. the Diagnostics table prints the numerical-integrity evidence as a raw
     debug dump (`StructuralReport { … PivotEvidence { … } }`), most of each
     PDF's length. **Pre-existing:** the pressure-free result's
     `NUMERICAL_INTEGRITY_CHECKS_PASSED` message carries the same dump, and the
     renderer carries result diagnostics on both routes. *Improvement:* a
     readable summary with the detail accessible;
  2. limitation ids and scopes name the run `run:preview-linear-static-001`.
     **Pre-existing:** the product producer names every desktop run so
     (`product_physics/src/lib.rs:3278`), and the pressure-free report's
     technical-preview entry carries it; the exact entries (`…:exact:<n>`) add
     more ids with the same run name. *Improvement:* the producer's run
     naming;
  3. values are printed with every digit. **Pre-existing** for result rows
     (`reportResultRows.ts:48`, `${value} ${unit}`, e.g. 28.462881029847143 MPa
     on the pressure-free route). **New** on exact reports: the stress
     headlines, which quote the producer's summary maxima in Pa (e.g.
     41354908.75465696 Pa); a pressure-free rendered report has no headline.
     *Improvement:* a reviewed display-rounding and unit policy;
  4. the pressure-source-1 example (synthetic n05) shows 0 Pa for both
     headlines, and its standing line says "integrity checked" while
     Diagnostics carries `NUMERICAL_INTEGRITY_SENSITIVE`. **New:** the standing
     line and the headlines exist only on exact reports, and the
     numerical-health line, which on a pressure-free report states the
     ordinary solve's numerical-health state, is omitted from exact reports
     (§6.1). Codex found it numerically correct: n05 is pure torsion (1e-8 N·m
     about RX), so the normal-stress maxima are zero while the torsional shear
     is not, and the warning describes the ordinary attempt while the
     retained-source response is the qualified one. *Improvement:* label the
     headlines explicitly as normal-stress maxima, and distinguish the
     retained-source eligibility from the ordinary solve's health next to
     them.
- **Remaining item (Codex review round 3 of #1648, not changed).** A direct
  call of the low-level `pdf_emitter` library can emit a relabelled exact
  report under `public_report`; its production callers (the package assembly
  and the native commands) enforce the local-private context boundary.
- **Remaining FR-REP-003 item (Codex review round 1 of #1648, not changed).**
  `supportSubjects` (`renderableReportInput.ts:381`) gives a record only for a
  support that carries a `provenance_record`, so a support that carries only a
  legacy `provenance` note has no record in the report.
- **Texts.** The same rewording of item 2 also applies to the pressure
  authoring panel's mode line (`PressureAuthoringPanel.tsx`), which repeated
  the refusal's list of admitting surfaces. Appendix A.2 adds O15 for the
  user guide's limitations list, which would otherwise say reports are only for
  pressure-free results.

## 7. Verification

**Slice (a).**
- Rust integration (`core/product_physics/tests/exact_route_limitations.rs`):
  - the fixture is current;
  - every committed exact corpus list equals a fixture list;
  - the recovery statements enter once (§2.2);
  - no `pressure-source-1` list with a connector or a realized bend is emitted (§2.2);
  - the existing `pressure_source_context_tests.rs` and corpus-currency tests
    pass unchanged.
- vitest `features/report/exactReportLimitations.test.ts`:
  - coverage is exactly the fixture;
  - the digest keying;
  - verbatim statements and the source-paragraph omission;
  - `exact_report_statements.json` is current;
  - fail-closed `REPORT-EXACT-LIMITATION-UNREGISTERED`.
- **Claims check**, same file. No statement or §3 text contains any of these:
  the BS-ACCEPT canonical text or a short variant (whitespace-normalized,
  case-folded, copied from registry §1); the omitted sentence; three or more
  GEN-13 litany terms; "acceptance". Every statement is WinAnsi-encodable.
- Rust unit (`report_renderer/src/exact_statements.rs`):
  - the embedded table parses, with 19 entries;
  - its digests are the coverage fixture's strings' digests;
  - each statement equals its string, except S9 and S10, whose statement is the
    string without the omitted sentence;
  - malformed table content refuses every exact section.
- Rust integration (`core/reporting/report_renderer/tests/render.rs`,
  `core/reporting/pdf_emitter/tests/emit.rs`):
  - the exact block and Library Records render in HTML and PDF;
  - the standing line is composed from the binding;
  - each binding field that disagrees with the report's records refuses with
    `REPORT-EXACT-ADMISSION-BINDING-MISMATCH`;
  - an exact limitation statement outside the table, or exact limitation
    entries without `exact_route`, refuse with
    `REPORT-EXACT-LIMITATION-UNREGISTERED`;
  - an unknown field in the new structs refuses;
  - the existing invented inputs render byte-identical;
  - an exact report has no blocking linter finding.
- Rust integration (`core/reporting/report_package/tests/report_rows_parity.rs`,
  a new `tests/exact_sections_parity.rs`):
  - the suffix rule;
  - the dormant item 9 against the vectors, with the §3.5 tests (unknown
    string, changed source paragraph, same-run substitute, each binding field).
    The runner-profile tests need the witness set, so they are in the in-crate
    witness module (slice (b));
  - `tests/current_result_package.rs` still refuses an exact document at
    item 0.
- Rust unit (`report_package/src/admitted_tests.rs`):
  `admitted_set_forwarding_graph_is_closed` and the scanner's negative test
  (§5.5).
- Rust unit (`src-tauri/src/report_render_exact_tests.rs`), step 0 (§3.6):
  - the production native render refuses a coherent invented exact input, which
    the witness set renders unblocked with "integrity checked", with
    `REPORT-RENDER-ROUTE-NOT-ADMITTED` and no HTML, through
    `render_calculation_report_args` and from the raw body through
    `command_ingress::admit` as the command does;
  - the refusal precedes the renderer's decode of the input;
  - each structured marker refuses, through the production root and the raw
    body: an `exact_route` without an identity or not an object, an exact
    identity at `exact_route.semantic_contract_id`,
    `exact_route.admission.semantic_contract_id` or a `semantic_contract`
    limitation source, and exact limitation entries without `exact_route`;
  - free text is not a marker: a pressure-free input whose load-case label
    and other free-text fields equal each exact identity renders
    byte-identically to the unchanged renderer call;
  - an input that avoids every marker (the coherent exact input without
    `exact_route`, its limitation ids and sources renamed) renders with no
    exact block, standing line or headlines, and exact limitation entries
    without `exact_route` block export in the renderer itself;
  - a pressure-free input renders byte-identically to the unchanged renderer
    call, with `exact_route` absent or null, and the admitting decision
    allocates nothing;
  - `render_admitted_set_forwarding_graph_is_closed` and the scanner's negative
    test on invented desktop sources (§5.5).

**Slice (b).**
- vitest `features/report/exactReportAdmission.test.ts`:
  - each binding field equals its document value and its recomputed value, and
    a disagreeing field refuses with `REPORT-EXACT-ADMISSION-BINDING-MISMATCH`;
  - **same-run substitution**: a second result object with the same run id,
    identity and producer and one changed row value, offered with the first
    result's admission, refuses with `REPORT-EXACT-ADMISSION-STALE`;
  - a JSON copy of an admission refuses with `REPORT-EXACT-ADMISSION-REQUIRED`;
  - **asynchronous retirement**: the result disposed
    (`disposeDeliveredMechanicsResult`), custody reset or the model edited
    while the builder awaits refuses at its end check;
  - in the panel and the package request, retirement after the build sends no
    IPC (spied).
- Rust unit (`src-tauri/src/solve_jobs.rs`), the custody digests (§3.6):
  - `intake_sha256` is the SHA-256 of the admitted body, `result_sha256` that
    of the delivered bytes, and `solver_mode` the admitted mode;
  - `custody_binding` is none for an untaken job, after `dispose_result`, after
    `dispose_all_results`, after pruning and for an unknown job;
  - the two hashes allocate nothing (counting allocator), and `take_result`
    hashes nothing.
- **Direct-native substitution** (`report_package_save.rs` and `lib.rs`
  tests), the regression check for the revision-3 finding. A test registry job
  delivers the pressure-2 straight corpus envelope through the test launcher,
  from that corpus's model, and is taken into custody. The genuine witness
  request and render input are loaded with `native_job_id` and
  `received_bytes_sha256` set to the live job's. The substitute (§5.5) keeps
  those two values. Every other caller-controlled hash is rebound coherently:
  - in the result document: the row checksums, the received carrier checksum,
    the run hashes and the derivative self-hash;
  - in the report: the rows and the `audit_manifest_refs` and
    `result_export_refs` checksums;
  - in the handoff records: the row hashes;
  - in the audit: the analysis-record, model and input-manifest hashes;
  - in the `exact_route` binding: `received_result_sha256`, `model_sha256`,
    `input_manifest_ref`, `input_manifest_sha256` and `derivative_sha256`.

  `admit_report_package`, and `render_calculation_report_args_in` with
  `EXACT_RENDER_WITNESS_ADMITTED` (§5.5), each refuse:
  1. the substitute with the genuine result text, with
     `REPORT-EXACT-SOURCE-NOT-DELIVERED: result` at step 5. A variant keeps
     the genuine `exact_route` binding and is refused at the save's source
     equality and, as `REPORT-EXACT-SECTIONS-MISMATCH`, at the render's
     re-derivation;
  2. the substitute with its own text as `exact_result_text`, with the same
     code at step 2, because its SHA-256 is not the delivered bytes';
  3. a substituted model (one support stiffness changed, with the model,
     manifest and audit hashes rebound) with the genuine intake text, with
     `REPORT-EXACT-SOURCE-NOT-DELIVERED: model`; with its own text as
     `exact_intake_text`, at step 2;
  4. the genuine input after `dispose_result`, and after
     `dispose_all_results`, with `REPORT-EXACT-CUSTODY-RELEASED`;
  5. a delivered pressure-source-1 result with load/reference states, from its
     corpus model, with `REPORT-EXACT-STANDING-NOT-ELIGIBLE`.

  Then:
  - the genuine request passes the native binding and refuses at item 0 with
    `REPORT-PACKAGE-ROUTE-NOT-ADMITTED`;
  - the genuine render input renders under `EXACT_RENDER_WITNESS_ADMITTED`;
  - the genuine texts with the render input's rows, one exact limitation
    statement or one headline changed refuse with
    `REPORT-EXACT-SECTIONS-MISMATCH: <section>`; with `model_sha256` changed
    they refuse at step 5 with `REPORT-EXACT-SOURCE-NOT-DELIVERED: model`;
  - a fabricated row appended to the genuine input (a numerical-health id with
    a stress statement and invented references) refuses with
    `REPORT-EXACT-SECTIONS-MISMATCH: result_rows`; in a save it refuses at
    item 0, and at item 6 under the witness set;
  - the texts without `exact_route` refuse likewise;
  - the save refuses when custody ends during its dialog, and writes nothing.
- **The production native render refuses a genuine exact result**
  (`src-tauri/src/report_render_exact_tests.rs`). This is the regression
  check for the revision-4 gate finding. The live test job and the genuine
  pressure-2 render input and texts are those above. The input is passed
  through `render_calculation_report_args`, which supplies the production
  set, and from the raw body through `command_ingress::admit` as the command
  does. Both refuse at step 0 with `REPORT-RENDER-ROUTE-NOT-ADMITTED`. The
  lease does not grow, neither text is parsed, and the job's custody binding
  is unchanged after the refusal. An `exact_route` naming pressure-source-1, preview-physics-2 or an
  invented identity refuses likewise. Under the witness set, the last two
  still refuse at step 0. A pressure-free input without `exact_route` renders
  byte-identically under either set.
- **Pricing** (same files):
  - `parse_lease_bytes` with the two texts is the former sum plus their
    `text_parse_bytes`;
  - the render's growth is the three texts' `text_parse_bytes` plus W M_r;
  - oversize texts refuse with the save's `TOO_LARGE` before any parse;
  - a whole-command counting-allocator check of the genuine exact render, from
    the raw body through `command_ingress::admit` and
    `render_calculation_report_args_in` with the witness set, has a peak
    within its lease;
  - the same check on the genuine exact save, to its item-0 refusal, has a
    peak within its lease;
  - a refused growth refuses with its `ResourceRefusal` and parses nothing.
- vitest `services/previewService.test.ts`: `deliveredIntakeText` equals the
  body `startPreviewMechanicsJob` sent (spied invoke). It is null for a copy,
  an unregistered result and a disposed one.
- vitest `features/report/renderedReport.test.tsx` (below the gate):
  - the exact block, the binding, headlines (values and withheld) and row
    suffixes;
  - the load/reference line;
  - limitation entries and ids;
  - `REPORT-EXACT-ADMISSION-REQUIRED` and `REPORT-EXACT-HEADLINE-UNREADABLE`.
- vitest `features/report/reportResultRows.parity.test.ts`: the exact vectors.
- **The test-only assembly** (§5.5), in `report_package/src/exact_package_witness_tests.rs`:
  - each witness shape assembles through `assemble_wire_request_in` and reads
    back through `read_report_package_container_in`, both under
    `EXACT_WITNESS_ADMITTED`. Passing shows that the set reaches every item-0
    check on both paths through the forwarding graph;
  - every production entry point refuses with `REPORT-PACKAGE-ROUTE-NOT-ADMITTED`;
  - the runner's package path refuses the same requests, and
    `admit_report_package` refuses them at the native binding;
  - the runner-profile refusals in the producer and the reader (§3.5);
  - the substitute request passes items 0-9 (§3.5);
  - the forwarding-graph scan of slice (a) still passes with the witness
    module present (§5.5).
- **Still refused**:
  - vitest `features/results/outputPolicy.test.ts`: for `pressure` and
    `pressure_source`, `report`, `rendered-report` and `report-package` refuse
    with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`, and `EXACT_ADMITTED_SURFACES` is
    exactly result-export, stress-neutral and rule-check;
  - `renderedReport.test.tsx`: an exact result shows the refusal and no render
    control;
  - `reportPackageRequest.test.ts`: the exact refusals stand,
    `buildReportPackageRequest` refuses each witness input, and
    `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS` is `[preview-physics-2]`;
  - the production native render refuses a genuine exact result (above), and
    `RENDER_ADMITTED_EXACT_CONTRACTS` is empty.
- The native smoke's exact checks (`run.mjs:277-283`) pass unchanged.

**Slice (c).**
- vitest `renderedReport.test.tsx` and `reportPackageRequest.test.ts`, as
  targeted regression checks with independent scrutiny, since this is a data
  boundary:
  - the records with and without `provenance_record`, and the support records;
  - a component with a record and no note has no missing-data flag and no
    `COMPONENT_PROVENANCE_MISSING`;
  - a record marked `accepted` gives a value that is not a missing-data finding,
    so `REPORT_SECTION_MISSING_VALUE_MARKED_ACCEPTED` does not fire under
    `validate_report_sections`;
  - an `accepted` record with a blank source name and no note gives a `pending`
    value, and its record keeps `accepted`;
  - library references and findings;
  - screener blocks for `protected_suspected`, `quarantined` and `rejected` in
    both contexts.
- **Schema check of representative desktop records.** vitest builds
  `report_sections` from an invented model and writes it to
  `fixtures/reports/invented/report_sections_provenance_records.json` (stale
  fails). The model has a material, a section, a component and a support, each
  with and without a record, and records marked `accepted`, `rejected` and
  `quarantined`. A new pytest, `tests/test_report_sections_provenance_records.py`,
  validates every `provenance_notes` entry against `$defs/Provenance`, and every
  `user_supplied_values` entry against `$defs/UserSuppliedValue`
  (`schemas/report_sections.schema.yaml`). It uses `jsonschema` 4.26.0
  (`requirements-dev.txt`). The pre-existing divergence (§9) is named and
  excluded by value-id prefix (`component-provenance:` and `spring-hanger:`)
  for `value_category` and `quantity` only; those entries' records are still
  validated. Any new divergence fails.
- pytest `tests/test_report_protected_content_linter.py` and
  `tests/test_report_sections_contract.py`.
- The regenerated witnesses and the counting-allocator tests of the package
  request (`report_package_text_parse_stays_within_its_priced_growth`).

**Every slice.**
- GEN-13: `python3 tools/validation/validate_claims_language.py --repo-root .`
  (also `tests/test_claims_language_surface.py`). It scans
  `exactReportLimitations.ts`, which is non-test desktop source.
- `python3 -m tools.deliverables --project projects/chirality-piping check`.
- No W-11 test, fixture or vector asserts the shape or count of the
  rule-check outcomes (§1.7).
- Review: an independent Codex round per slice. These get specific scrutiny:
  the statement table, its Rust form, the claims check and the forwarding
  graph with its scan in (a); the admission binding, the native render's step
  0 and its witness set, the native source binding and its pricing in (b);
  the screener, records and schema check in (c).

**This revision** ran the GEN-13 scan functions (`_scan_file`,
`_scan_retired_texts`) over the 19 statements and 93 other texts it proposes,
treated as desktop product source, together and one by one: no finding. The
other texts are the 13 refusal messages and 61 quoted texts of §2.3-§5.5,
including §3.6's step 0 refusal, and the 19 New texts of Appendix A, including
the revised G2, G5 and O13. The repository run of GEN-13 is clean. Each Old
text in Appendix A was checked again for one byte-exact occurrence, the
G texts at the basis head and O4 and O13 after A.1 G2 and G5 are applied.

## 8. Reconciliation

- **DEL-08-01:** this design; ScopeOfWork :81 by A.1 G1 and G2;
  `deliverable.yaml` code and check pointers for the new files.
- **DEL-08-03:** ScopeOfWork :99 by A.1 G3; pointers.
- **DEL-04-07 Design** `desktop-exact-outputs.md` §4.5 by A.1 G5, and at
  opening by A.2 O13; its refusal stands until the owner opens it.
  `memory-budget-decision.md` figures in (c). The native source binding
  (§3.6) touches DEL-04-07's solve-job registry and ingress pricing. In slice
  (b), `memory-budget-decision.md` gains three things: the job's digest
  metadata, the exact save's and render's text terms, and the render's W M_r
  provision. None loosens a limit. Codex's third round found that the W M_r
  provision needs no further owner risk acceptance: it is additive and within
  the existing command-working-memory residual. It stays a measured provision,
  not a proven bound (§3.6). DEL-04-07's `deliverable.yaml` gains the check
  pointers. The native render's step 0 (§3.6) refuses before any of these
  terms, so before opening a production render never reaches them.
- **The user guide** §10 by A.1 G4, and at opening by A.2 O9-O12 and O14.
- **The completeness map:** FR-REP-001 and FR-REP-003 rows (:202,:204), gap 1
  and the W-11 row (:541), by HELP_HUMAN.

## 9. Findings outside W-11

These are existing gaps. Each is separate from W-11 and from the others.

- **Pressure-free limitations.** The pressure-free report also carries none of
  its producer's published limitations; preview-physics-2's nine are bound to
  its table and include scope statements such as "No equivalent stress or code
  stress." A separate item, if wanted.
- **Run-record diagnostics (#1613).** The rendered report omits the run
  record's rule-pack exclusion diagnostics that #1613 put into the exports, on
  every route. A separate FR-REP-001 item.
- **Schema divergence.** Two things in the desktop's existing
  `user_supplied_values` are outside `schemas/report_sections.schema.yaml`:
  - their categories (`component_provenance:<kind>`, `spring_hanger:<kind>`)
    are outside the `value_category` enum;
  - their `quantity: null` is not a `ValueQuantity`.

  The Rust struct accepts both. This predates W-11. W-11's entries use
  `report_metadata` and omit `quantity` (§4.1), and slice (c)'s schema check
  (§7) fails on any new divergence.
- **Error text that echoes request values (Codex review round 4 of #1639).**
  Two refusal paths format request content into their messages. Both predate
  W-11, and both are outside the provenance routes that slice (c) made
  value-free.
  - `report_package/src/wire.rs:205` and its neighbouring audit-token
    converters interpolate the value supplied: a hash algorithm of `C:/x.pdf`
    returns that path through the native save and the headless stdout.
  - `src-tauri/src/export_disposition_screen.rs:452` builds diagnostic
    pointers from arbitrary keys. A key `C:/x.pdf` under `report` returns
    `/request/report/C:~1x.pdf/review_status`.

  The fix would use fixed field names, error kinds and positional placeholders.
  It is a remaining item under the owner's finishing ruling of 2026-10-26.

## Appendix A. Wording-route drafts

Each Old text below occurs exactly once in its file at the basis head (Python
`count == 1`), starting on the line given. They go through HELP_HUMAN and the
owner's wording route before they are applied. The A.1 texts are applied, each
in the slice and PR marked on it; the A.2 texts are not.

### A.1 Texts the implementation changes behind the gate (applied)

**G1. DEL-08-01 ScopeOfWork :81** (current-implementation paragraph; provenance clause; slice (c); `execution/PKG-08/DEL-08-01/ScopeOfWork.md`; applied in slice (c), #1639)

Old:

```text
the result coordinate system, or states "not recorded";
```

New:

```text
the result coordinate system, or states "not recorded"; it also records the provenance of materials, sections, components and supports and the model's library references, with the library store's findings on them (FR-REP-003);
```

**G2. DEL-08-01 ScopeOfWork :81** (current-implementation paragraph; rendered-report clause; slice (b); `execution/PKG-08/DEL-08-01/ScopeOfWork.md`; applied in slice (b), #1637)

Old:

```text
exact-route results are refused for the rendered report (`loadReferenceOutputRefusal`; owner ruling 10A, 2026-10-21);
```

New:

```text
exact-route results are refused for the rendered report by its output-policy cell (`surfaceOutputRefusal` on `rendered-report`) and, natively, by the render's identity check (`REPORT-RENDER-ROUTE-NOT-ADMITTED`; owner ruling 10A, 2026-10-21), and their sections (DEL-08-01 Design `exact-route-report-sections.md`) are built behind those refusals;
```

**G3. DEL-08-03 ScopeOfWork :99** (current-implementation paragraph; slice (c); `execution/PKG-08/DEL-08-03/ScopeOfWork.md`; applied in slice (c), #1639)

Old:

```text
supplies the result diagnostics and a session provenance record classified `private_project_data` / `private_only`;
```

New:

```text
supplies the result diagnostics, a session provenance record classified `private_project_data` / `private_only`, the component and spring-hanger records, a record for each material and section and for each support that carries a `provenance_record` (from that record where present, else from the entity's `provenance` note; a component's record is likewise taken from its `provenance_record` where present), and the model's library references, with the library store's `MODEL-LIBRARY-REF-UNRESOLVED` and `MODEL-LIBRARY-REF-VERSION-CHANGED` findings as provenance warnings;
```

**G4. User guide `docs/user_guide/index.md` :400** (§10, last sentence of the first paragraph; slice (c); `docs/user_guide/index.md`; applied in slice (c), #1639)

Old:

```text
materials, sections and libraries are not yet in it.
```

New:

```text
it also records the provenance of materials, sections, components and supports and lists the library records the model references, with the library store's findings on them.
```

**G5. DEL-04-07 Design `desktop-exact-outputs.md` §4.5 :209** (second sub-bullet of the first bullet; slice (b); `execution/PKG-04/DEL-04-07/Design/desktop-exact-outputs.md`; applied in slice (b), #1637)

Old:

```text
  - The renderable report's only limitation is a fixed technical-preview statement (`features/report/renderableReportInput.ts:343-357`). It carries none of the exact identity's published limitations, such as the D-3 exclusions.
```

New:

```text
  - The renderable report builds the exact identity's limitation sections, stress headlines and standing line behind this refusal (DEL-08-01 Design `exact-route-report-sections.md`). Until the owner opens them, none of them reaches an output: the `rendered-report` and `report-package` cells refuse an exact result, and the native render and the package's Rust boundary refuse every input that carries an exact identity in an identity field (`REPORT-RENDER-ROUTE-NOT-ADMITTED`, `REPORT-PACKAGE-ROUTE-NOT-ADMITTED`).
```


### A.2 Texts only opening would change (applied at opening)

O4 is stated against the text as it reads after A.1 G2, and O13 against the
text as it reads after A.1 G5. Their Olds were checked for one occurrence in
those texts (revision 5 re-checked both: `count == 1` after G2 and after G5).
At opening (after slices (b) and (c), #1637 and #1639), every Old O1-O14 was
checked again (`count == 1` in its file), none had drifted, and each New was
applied as drafted, with the placeholder replaced by the owner's words of §0.
O15 was added at opening.

**The placeholders stay placeholders.** "[the owner's opening decision, quoted
with its date]" (O1, O4, O13) holds the place of the owner's actual words and
the date of the decision. It is replaced only by those words, quoted, when the
owner makes the decision. No agent writes, paraphrases or anticipates them, and
no text here is applied before then.

**O1. DEL-04-08 ScopeOfWork :219** (REQ-006 (c), first bullet; `execution/PKG-04/DEL-04-08/ScopeOfWork.md`)

Old:

```text
Its result JSON export, stress-neutral export and rule check admit it, and each takes the reader's verdict and standing. Every other desktop output surface refuses it (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`), and the report package refuses it with the same code at the desktop output policy and with `REPORT-PACKAGE-ROUTE-NOT-ADMITTED` at its Rust boundary (owner ruling 10A).
```

New:

```text
Its result JSON export, stress-neutral export, rule check, rendered report and report package admit it, and each takes the reader's verdict and standing; the rendered report and the package carry the exact-route sections of DEL-08-01 Design `exact-route-report-sections.md`, and the package's Rust boundary admits it with preview-physics-2 ([the owner's opening decision, quoted with its date]). Every other desktop output surface refuses it (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`).
```

**O2. DEL-04-08 ScopeOfWork :219** (REQ-006 (c), first bullet, load/reference sentence; `execution/PKG-04/DEL-04-08/ScopeOfWork.md`)

Old:

```text
when eligible it is admitted, the two exports carry its resolved load/reference states, and the admitting panels disclose them.
```

New:

```text
when eligible it is admitted, the two exports carry its resolved load/reference states, the rendered report and the package state them with their published limitations, and the admitting panels disclose them.
```

**O3. DEL-04-08 ScopeOfWork :240** (AC-006; `execution/PKG-04/DEL-04-08/ScopeOfWork.md`)

Old:

```text
and the report package's Rust boundary refuses every identity except preview-physics-2.
```

New:

```text
and the report package's Rust boundary refuses every identity except preview-physics-2, pressure-2 and pressure-source-1.
```

**O4. DEL-08-01 ScopeOfWork :81** (current-implementation paragraph, rendered-report clause as A.1 G2 leaves it; after A.1 G2; `execution/PKG-08/DEL-08-01/ScopeOfWork.md`)

Old:

```text
exact-route results are refused for the rendered report by its output-policy cell (`surfaceOutputRefusal` on `rendered-report`) and, natively, by the render's identity check (`REPORT-RENDER-ROUTE-NOT-ADMITTED`; owner ruling 10A, 2026-10-21), and their sections (DEL-08-01 Design `exact-route-report-sections.md`) are built behind those refusals;
```

New:

```text
an exact-route result is rendered with its sections (DEL-08-01 Design `exact-route-report-sections.md`) only after Export Result JSON admits it in the desktop, and only when the native host finds that the result and model texts submitted with the report are the exact bytes one of its solve jobs delivered and received, that the job's result is still in the app's custody, that the Rust reader's standing for that result and model, computed by the native host, is numerically eligible, and that the report's result and model digests, result rows and exact sections are those of that result and model; the report's standing line is composed from those digests and from the digest of an input manifest whose model is that model ([the owner's opening decision, quoted with its date]);
```

**O5. DEL-08-01 ScopeOfWork :81** (current-implementation paragraph, package clause; `execution/PKG-08/DEL-08-01/ScopeOfWork.md`)

Old:

```text
saves a version 2.0.0 package for a Current pressure-free (preview-physics-2) result under
```

New:

```text
saves a version 2.0.0 package for a Current pressure-free (preview-physics-2) or eligible exact-route (pressure-2, pressure-source-1) result under
```

**O6. DEL-08-01 ScopeOfWork :81** (current-implementation paragraph, last sentence; `execution/PKG-08/DEL-08-01/ScopeOfWork.md`)

Old:

```text
Every other identity is refused (exact routes with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; owner ruling 10A, 2026-10-21).
```

New:

```text
Every other identity is refused.
```

**O7. DEL-08-06 ScopeOfWork :118** (desktop report-package request paragraph; `execution/PKG-08/DEL-08-06/ScopeOfWork.md`)

Old:

```text
for a Current preview-physics-2 result. The output policy's `report-package` cell refuses its retained successor and, under owner ruling 10A, pressure-2 and pressure-source-1; the package's Rust boundary admits only preview-physics-2 (DEL-08-01 Design `report-package-current-results.md`).
```

New:

```text
for a Current preview-physics-2, pressure-2 or pressure-source-1 result. The output policy's `report-package` cell refuses the retained successors; the package's Rust boundary admits those three identities (DEL-08-01 Designs `report-package-current-results.md` and `exact-route-report-sections.md`).
```

**O8. DEL-08-02 ScopeOfWork :116** (current-implementation paragraph, second sentence; `execution/PKG-08/DEL-08-02/ScopeOfWork.md`)

Old:

```text
(`save_report_package` in `apps/desktop/src-tauri/src/lib.rs`) for a Current pressure-free result.
```

New:

```text
(`save_report_package` in `apps/desktop/src-tauri/src/lib.rs`) for a Current pressure-free result or an eligible exact-route result.
```

**O9. User guide `docs/user_guide/index.md` :91** ("What the app refuses or does not offer today", first bullet; `docs/user_guide/index.md`)

Old:

```text
- **Reports are local-private only, and pressure-free only.**
```

New:

```text
- **Reports are local-private only.**
```

**O10. User guide `docs/user_guide/index.md` :106** (same bullet, exact-route sub-bullet (:106-108); `docs/user_guide/index.md`)

Old:

```text
  - Every report surface refuses exact-route results with
    `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` (owner ruling 10A, 2026-10-21;
    `execution/PKG-04/DEL-04-07/Design/desktop-exact-outputs.md`).
```

New:

```text
  - On an exact-route result (pressure-2 or pressure-source-1) that Export
    Result JSON admits, **Render report** adds the producer's limitations,
    the stress headline of each section basis, the numerical standing and,
    where present, the resolved load/reference states. The report JSON packet
    still refuses exact-route results with
    `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`.
```

**O11. User guide `docs/user_guide/index.md` :109** (same bullet, package sub-bullet (:109-111); `docs/user_guide/index.md`)

Old:

```text
  - Save Report Package… saves a local private package for a pressure-free
    result after its own local-private selection; exact-route results are
    refused with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`.
```

New:

```text
  - Save Report Package… saves a local private package for a pressure-free
    result, or for an exact-route result that Export Result JSON admits,
    after its own local-private selection.
```

**O12. User guide `docs/user_guide/index.md` :393** (§10, first sentence (:393-396); `docs/user_guide/index.md`)

Old:

```text
The desktop app renders a report only as a local private document for a
pressure-free result, after the local-private selection; exact-route results
are refused, and the report package is saved locally for a pressure-free result (see
```

New:

```text
The desktop app renders a report only as a local private document, after the
local-private selection, for a pressure-free result or an exact-route result
that Export Result JSON admits, and the report package is saved locally for
the same results (see
```

**O13. DEL-04-07 Design `desktop-exact-outputs.md` §4.5 :205** (from the opening line through the report-package bullet, as A.1 G5 leaves them (:205-211); the final "other fifteen surfaces" bullet is kept; after A.1 G5; `execution/PKG-04/DEL-04-07/Design/desktop-exact-outputs.md`)

Old:

```text
**These stay refused in this unit** (ruling 10A, §8).

- **Report JSON and rendered report** (`report`, `rendered-report`). Their builders are generic.
  - `selectedResultRefs` picks the single `max_open_formula_stress` headline (`features/report/ReportPanel.tsx:279-298`), but an exact result carries one headline per section basis.
  - The renderable report builds the exact identity's limitation sections, stress headlines and standing line behind this refusal (DEL-08-01 Design `exact-route-report-sections.md`). Until the owner opens them, none of them reaches an output: the `rendered-report` and `report-package` cells refuse an exact result, and the native render and the package's Rust boundary refuse every input that carries an exact identity in an identity field (`REPORT-RENDER-ROUTE-NOT-ADMITTED`, `REPORT-PACKAGE-ROUTE-NOT-ADMITTED`).

  Opening them needs DEL-08-01's own design of those sections and of the PRD §19.3 notice's place. They keep refusing with the reworded text (§5.3).
- **Report package.** Since W-18 (DEL-08-01 Design `report-package-current-results.md`) it saves a Current preview-physics-2 result. It refuses the exact identities by policy and Rust identity, not by the fresh-result code: the policy entry `report-package: refused` gives their routes' refusal (`PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`, through `reportPackageUnavailableReason` in `reportPackageRequest.ts`), and the package's Rust boundary admits only preview-physics-2 (`REPORT-PACKAGE-ROUTE-NOT-ADMITTED`).
```

New:

```text
**These stayed refused in this unit** (ruling 10A, §8). The rendered report and the report package were opened later under DEL-08-01 Design `exact-route-report-sections.md` ([the owner's opening decision, quoted with its date]); the report JSON packet and the other fifteen surfaces still refuse.

- **Report JSON** (`report`) stays refused with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`; its builder still selects one headline and fixed demo references.
- **Rendered report** (`rendered-report`) admits an exact-route result only when Export Result JSON admits it, with the sections of DEL-08-01 Design `exact-route-report-sections.md`. Its native render admits the pressure-2 and pressure-source-1 identities and binds each exact input to the result and model its solve job delivered and received. The PRD §19.3 notice keeps its existing place.
- **Report package** (`report-package`) saves a Current preview-physics-2 result or an eligible pressure-2 or pressure-source-1 result. Its TypeScript and Rust identity checks admit those three identities; its exact sections are checked against the source as DEL-08-01 Design `exact-route-report-sections.md` specifies.
```

**O14. User guide `docs/user_guide/index.md` :148** ("Reports and result envelopes" row of the architecture table; `docs/user_guide/index.md`)

Old:

```text
exact-route results are refused, and the report package is saved locally for a pressure-free result
```

New:

```text
eligible exact-route results can be rendered and packaged, and the report JSON packet still refuses them
```

**O15. User guide `docs/user_guide/index.md` :482** (§14, the app's refusals and gaps; added at opening; `docs/user_guide/index.md`)

Old:

```text
[Current Desktop App](#current-desktop-app-010): reports only as local private
  documents and only for pressure-free results,
```

New:

```text
[Current Desktop App](#current-desktop-app-010): reports only as local private
  documents, and the report JSON packet only for pressure-free results,
```
