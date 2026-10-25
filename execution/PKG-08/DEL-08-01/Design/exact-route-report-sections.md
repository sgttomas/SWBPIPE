# Exact-route report sections and report provenance (W-11)

**Basis:** `origin/main` at `25b7df82ae` (2026-10-25), after W-18a (a), (b1)
and (b2) (#1597, #1606, #1616), W-7 (#1581), #1613, #1621 and the W-12 merges
through #1624. Revision 2, design before implementation. Revision 1 (at
`67e9e43cc5`, 2026-10-24) was never reviewed; §1.1 lists what re-verification
corrected. Citations are at the basis head and are relative to
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
   §2.2 adds a composing seam that the producer itself uses.
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
| Rendered report | `outputPolicy.ts:85-88` (`EXACT_ADMITTED_SURFACES` = result-export, stress-neutral, rule-check), read by `LoadReferenceOutputGate` (`RenderedReportPanel.tsx:220`) and by the route-level check at `:171` | `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE` |
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
  bound exactly to its table (`preview_physics_evidence.rs:269`).
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
- **Fail closed.** A result whose producer limitations include a string the
  table lacks is refused for every report surface with
  `REPORT-EXACT-LIMITATION-UNREGISTERED: <identity> publishes a limitation this
  report has no reviewed statement for.` A producer text change therefore needs
  a reviewed table change before it reaches a report.

### 2.2 The coverage fixture and its writer

- **Seam.** `core/product_physics/src/lib.rs` gains
  `pub(crate) fn exact_formulation_basis(load_state: bool, connectors: bool,
  realized_bends: bool, selected: bool) -> FormulationBasis`. It composes exactly
  what the exact arm of `formulation_basis_for_model_with_source` composes today
  (§1.3 items 1-7), and that arm calls it. This refactor is behaviour-preserving;
  the existing corpus-currency tests and `source_receipt/pressure_source_context_tests.rs:174-230`
  pin it.
- **Writer.** A test in `core/product_physics/src/exact_route_limitations_tests.rs`,
  `exact_route_limitations_fixture_is_current`, enumerates the whole domain of
  the seam: `selected` ∈ {false (`pressure-2`), true (`pressure-source-1`)} ×
  `load_state` × `connectors` × `realized_bends`, 16 lists. It writes, in this
  order, `fixtures/reports/exact_route_limitations.json`:
  `{schema_version: "1.0.0", generator: "<test path>::exact_route_limitations_fixture_is_current",
  lists: [{semantic_contract_id, load_reference_states, objective_connectors,
  realized_bends, reachable, limitations}], strings: [<the 19 distinct strings,
  sorted by code point>]}`.
  `reachable` is false for the six `pressure-source-1` lists with a connector or
  a bend: the source family admits no components, so the producer never selects
  such a model. The table covers them anyway; they add no string.
- **Stale fails.** The test compares its output with the committed bytes and
  fails on any difference. `OPENPIPESTRESS_WRITE_EXACT_ROUTE_LIMITATIONS=<path>`
  writes the file instead, as the recorded-run witness does
  (`apps/desktop/src-tauri/src/analysis_runs_tests.rs:936`).
- **Against real envelopes.** The same test checks that every published list in
  the committed exact corpora (§1.3) equals one fixture list.
- **Table test.** `features/report/exactReportLimitations.test.ts` reads the
  fixture and checks that the table's digests are exactly the digests of
  `strings` (no missing and no extra entry), and that each statement equals its
  string, except the two source paragraphs (§2.3).
- The test is registered in `tools/hosted-ci-routing.json` with the
  product-physics unit tests.

### 2.3 The statement table

Each row is one producer string with its exact report statement. "Verbatim"
means the statement is the producer string byte for byte.

**S1. Identity, pressure-2** (`exact_admission.rs:85`; first entry of every pressure-2 list). Producer SHA-256 `5312cff975d5c51c429d3a8cdae60f1da31b8a6a54df2909d08d7894403a8eac`. Statement, verbatim:

> 3.0.0/exact_pressure_v3 under pressure-2 semantics. Admitted families: small-displacement homogeneous-isotropic straight circular pipe members (shear deformation as the contract declares: euler_bernoulli, or timoshenko_annulus_energy_v1 with the energy-matched annular shear factor, whose nodal rotations are section rotations; G is derived from E/nu) and linear restraints or springs; realized curved bends are Euler-Bernoulli under either shear model and, with expansion joints authored as objective connectors, are admitted only as the bend and connector limitations state. Section bases: stiffness and pipe metal mass use the nominal wall; contents mass and pressure end-cap areas use the reduced bore; every stress row is published on the nominal section and on the reduced section (wall minus mill tolerance and corrosion allowance) and names its basis, and each basis has its own maximum and headline; no governing basis is chosen. Each basis's Lamé stresses are those of its own annulus, while the axial membrane stress divides the one wall force by that basis's area. Every other component, support, combination or equivalent-static family is refused by name (EXACT_PRESSURE_FAMILY_NOT_ADMITTED) until a later contract revision admits it.

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
- **Exact admission.** For an exact source, `buildRenderableReportInput` takes a
  new argument, `admittedResultExport`: the document
  `buildCurrentResultExport` returned for the same session inputs. It checks that
  the document's producer identity and run equal the result's. Without it the
  builder throws `REPORT-EXACT-ADMISSION-REQUIRED: an exact-route report is built
  only from a result that Export Result JSON admits.` That admission is the
  result export's: eligible standing with the live native invocation, the bound
  input manifest and record (`resultExportAdapter.ts:258-293`). The package
  already builds that document (`reportPackageRequest.ts:179`); the panel builds
  it before the render. A pressure-free source needs no argument, so its path is
  unchanged.
- **Below the gate.** Tests reach these sections through
  `buildRenderableReportInput` directly and through
  `assembleExactReportBelowOutputGate`, a test-only export of the panel's build
  step, as W-18 (b2) did before opening.

### 3.2 The exact-route result block

`report_renderer::RenderableReportInput` gains
`#[serde(default, skip_serializing_if = "Option::is_none")] pub exact_route: Option<ExactRouteSection>`:

```rust
pub struct ExactRouteSection {
    pub semantic_contract_id: String,           // the result's producer identity
    pub numerical_standing: String,             // §3.2 standing line
    pub load_reference_states: Option<String>,  // §3.2 load/reference line
    pub headlines: Vec<ReportHeadline>,         // §3.3, nominal then reduced
}
pub struct ReportHeadline {
    pub basis: String,                 // "nominal_section" | "reduced_section"
    pub label: String,
    pub value_display: String,
    pub location_ref: Option<String>,
    pub source_ref: Option<String>,
}
```

The renderer emits it at the start of the Results section
(`report_renderer/src/lib.rs:710`), in `assemble_report_sections`, so the HTML
and the PDF carry it alike:
- subheading "Exact-Route Result";
- a meta paragraph "Semantic contract: {semantic_contract_id} · Numerical
  standing: {numerical_standing}";
- the load/reference paragraph, when present;
- subheading "Stress Headlines by Section Basis", a table with the columns
  "Section basis", "Stress maximum", "Location" and "Source", and the paragraph
  "Each section basis has its own maximum; no governing basis is chosen."

An absent `exact_route` emits nothing, so pressure-free output is byte-identical.

**Standing line.** Admission (§3.1) means the standing is always eligible, so
`numerical_standing` is the fixed text: "integrity checked (the Rust reader's
standing for these result bytes, with this session's live native invocation)".
"integrity checked" is the session's label (`LoadReferenceStatesBlock.tsx:16`).
The numerical-health lines the builder already adds on the generic route
(`renderableReportInput.ts:672-678`, which includes pressure-2) are unchanged.

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
(`bindings.rs:194`), nothing below runs on a package. It is built and unit-tested
behind that refusal so that opening changes only the admitted set:
- `expected_exact_route_section(source)`: identity, the fixed standing text, the
  load/reference line from `contract_evidence.load_reference_states` (distinct
  configurations, cases and histories in published order), and the headlines
  from `summary`, all with the §3.2-§3.3 texts;
- `expected_exact_limitations(source, run)`: the §2.4 entries from
  `formulation_basis.limitations`, each statement equal to its producer string
  except that a string ending in the omitted sentence has it removed, plus the
  reference entry;
- both compared with the report, refusing with
  `REPORT-PACKAGE-EXACT-SECTIONS-MISMATCH`;
- shared vectors `fixtures/reports/invented/exact_report_sections_parity.json`,
  checked by vitest against the builder and by `report_package` against these
  functions;
- a test that the fixture's strings ending in the omitted sentence are exactly
  the two source paragraphs.

The TypeScript check at `reportPackageRequest.ts:181-185` reads a new exported
constant, `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS`, which a test keeps equal to
Rust's `ADMITTED_SEMANTIC_CONTRACTS` through the parity vectors. Both stay
`[preview-physics-2]`.

## 4. Report provenance: materials, sections, supports and library records (FR-REP-003, every route)

### 4.1 Records

**With a `provenance_record` r** (material, section, component or support), the
report record is:

| Field | Value |
|---|---|
| `source_name` | `r.source_name` |
| `source_location` | `r.source_location`, else "not recorded" |
| `source_license` | `r.source_license`, else "not recorded" |
| `contributor` | `r.contributor`, else "not recorded" |
| `contributor_certification` | `r.contributor_certification`, else "not recorded" |
| `redistribution_status` | `r.redistribution_status`, else "unknown" |
| `review_status` | `r.review_status`, else "pending" |
| `privacy_classification` | "private_project_data" |

`reviewed_by`, `reviewed_on` and `verification_note` are not carried: they are
not §20.2 fields, and `reviewed_by` names a person. `library_ref` goes to §4.2.

A `review_status` of `accepted` is the user's own source-data review
(`types.ts:12-14`), copied as a data field. No rendered table shows it, the
software never sets or advances it, and the screener treats it as it treats
`pending` (`redactionExportControls.ts:442-444`). If HELP_HUMAN prefers, the
report can write `pending` for every record instead; only the screener's input
changes.

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
- `user_supplied_values`, after today's entries: `value_id`
  `material-provenance:<id>`, `section-provenance:<id>` or
  `support-provenance:<id>`; `value_category` `material_provenance`,
  `section_provenance` or `support_provenance`; `source` `{material|section|support,
  <id>}`; `quantity` null; `provenance` the record; `privacy_classification` and
  `review_status` from the record; `required_for` ["reporting", "human_review"];
  `missing_data_finding` true when there is neither a record `source_name` nor a
  non-empty `provenance`.
- Diagnostics, mirroring `COMPONENT_PROVENANCE_MISSING` (`:220-236`):
  `MATERIAL_PROVENANCE_MISSING` ("Material {label} has neither a
  provenance_record nor a provenance note; report provenance is incomplete for
  this material.", remediation "Enter material source/provenance before relying
  on this report.") and `SECTION_PROVENANCE_MISSING` likewise, class
  `PROVENANCE_WARNING`, severity warning.

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

The redaction screener blocks a record with `protected_suspected`,
`quarantined` or `rejected` in every context
(`redaction-controls/redactionExportControls.ts:439-444`), as for every report.
`private_only` and `unknown` records render in a local-private report. A
concrete path in a `source_location` is redacted (`:438`), which the lossless
route then refuses; that is the existing rule.

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
| 4 | `reportPackageRequest.ts` `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS` (§3.5) | the same three ids |
| 5 | native smoke `run.mjs:277-283` | the two exact report checks become admission checks |
| 6 | tests pinning the refusals | `outputPolicy.test.ts`, `reportPackageRequest.test.ts:117-163`, `report_package/tests/reader.rs:1175`, `tests/current_result_package.rs`, the W-11 still-refused tests (§7) |
| 7 | commitment, design and guide texts | Appendix A.2 |

The report JSON packet keeps refusing; the cell for `report` is unchanged. No
package version change is needed: the manifest and member forms are unchanged,
and a build from before opening refuses an exact member by name
(`REPORT-PACKAGE-ROUTE-NOT-ADMITTED`), which DEC-028 allows.

### 5.2 An opened exact-route report

In the renderer's fixed order (`report_renderer/src/lib.rs:33-42`):
- model input summary and load cases, unchanged;
- results: the exact-route block (identity, standing line, load/reference line
  where present, both headlines), then the rows with their basis suffixes, then
  the numerical-health lines on pressure-2 (generic route) when the panel
  supplies them;
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
The bindings check the exact sections against the source (§3.5).

### 5.4 What stays refused

- The **report JSON packet** (`report`, `ReportPanel.tsx`) picks fixed demo ids
  and one headline (`selectedResultRefs`, `:297-305`). Its retirement or
  redesign is a separate item.
- The **handoff preview** and the other fifteen surfaces
  (`desktop-exact-outputs.md` §4.5). The model handoff (W-6b) needs no result.

### 5.5 Evidence for the return

The coverage fixture and table test; the claims checks (§7); a rendered
local-private exact report (HTML and PDF) and a 2.0.0 package, each built from
the committed exact corpora below the gate, for pressure-2 with and without
load/reference states, with a connector and a bend, and for pressure-source-1.

## 6. Slices and size

| Slice | Content | Size | Persistence / data boundary |
|---|---|---|---|
| **(a) Rust producers and renderer** | The `exact_formulation_basis` seam, the fixture writer and the committed fixture (§2.2). The renderer's `exact_route` and `library_references` fields in `assemble_report_sections`, so HTML and PDF both emit them, with render and emit tests, including byte-identity when absent. In `report_package`: the row suffix in `expected_report_result_rows`, the dormant exact-section checks (§3.5) and both parity vector files. Hosted-CI routing | M-L (~1.3k lines with tests, plus the generated fixture and vectors) | None. Numerical meaning untouched. Package and report bytes unchanged for every current input |
| **(b) Desktop exact sections, behind the gate** | `exactReportLimitations.ts` and its test; the exact path of `buildRenderableReportInput` (admission, limitations, profile reference, standing, load/reference line, headlines); the row suffix in `reportResultRows`; `RenderedReportPanel.tsx:171` at surface level and the panel's admission; `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS`; the `rendered-report` disclosure template; still-refused tests. Texts A.1 G2 and G5 | M-L (~1.2k) | None while the cells refuse; exact outputs stay unreachable |
| **(c) FR-REP-003 on every route** | Material, section, component and support records; library references and finding diagnostics; callers pass the findings; regenerated witnesses; the memory-budget figures re-measured. Texts A.1 G1, G3 and G4 | M (~0.7k, plus regenerated witnesses) | **Data boundary**: new private content in the open pressure-free report and package |

(a) comes first, because Rust drops unknown report fields (§1.1 item 3). (b) and
(c) follow in either order and may share a PR. Opening (§5.1) is an owner act
after them, about 0.2k lines with its tests and texts.

## 7. Verification

**Slice (a).**
- Rust unit (`core/product_physics/src/exact_route_limitations_tests.rs`): the
  fixture is current; every committed exact corpus list equals a fixture list;
  the existing `pressure_source_context_tests.rs` and corpus-currency tests pass
  unchanged.
- Rust integration (`core/reporting/report_renderer/tests/render.rs`,
  `core/reporting/pdf_emitter/tests/emit.rs`): the exact block and Library
  Records render in HTML and PDF; the existing invented inputs render
  byte-identical; an exact report has no blocking linter finding.
- Rust integration (`core/reporting/report_package/tests/report_rows_parity.rs`,
  a new `tests/exact_sections_parity.rs`): the suffix rule and the dormant
  checks against the vectors; `tests/current_result_package.rs` still refuses
  an exact document at item 0.

**Slice (b).**
- vitest `features/report/exactReportLimitations.test.ts`: coverage exactly the
  fixture; the digest keying; verbatim statements; the source-paragraph
  omission; fail-closed `REPORT-EXACT-LIMITATION-UNREGISTERED`.
- **Claims check**, same file: no statement or §3 text contains the BS-ACCEPT
  canonical text or a short variant (whitespace-normalized, case-folded, copied
  from registry §1), the omitted sentence, three or more GEN-13 litany terms, or
  "acceptance"; every statement is WinAnsi-encodable.
- vitest `features/report/renderedReport.test.tsx` (below the gate): the exact
  block, headlines (values and withheld), row suffixes, standing and
  load/reference lines, limitation entries and ids,
  `REPORT-EXACT-ADMISSION-REQUIRED`, `REPORT-EXACT-HEADLINE-UNREADABLE`.
- vitest `features/report/reportResultRows.parity.test.ts`: the exact vectors.
- **Still refused**, vitest `features/results/outputPolicy.test.ts`: for
  `pressure` and `pressure_source`, `report`, `rendered-report` and
  `report-package` refuse with `PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE`, and
  `EXACT_ADMITTED_SURFACES` is exactly result-export, stress-neutral and
  rule-check. `renderedReport.test.tsx`: an exact result shows the refusal and
  no render control. `reportPackageRequest.test.ts`: the exact refusals stand
  and `REPORT_PACKAGE_ADMITTED_CONTRACT_IDS` is `[preview-physics-2]`.
- The native smoke's exact checks (`run.mjs:277-283`) pass unchanged.

**Slice (c).**
- vitest `renderedReport.test.tsx` and `reportPackageRequest.test.ts`: the
  records with and without `provenance_record`, the support records, library
  references and findings, and screener blocks for `protected_suspected`,
  `quarantined` and `rejected` in both contexts. This is a data boundary, so
  these are targeted regression checks with independent scrutiny.
- pytest `tests/test_report_protected_content_linter.py` and
  `tests/test_report_sections_contract.py`.
- The regenerated witnesses and the counting-allocator tests of the package
  request (`report_package_text_parse_stays_within_its_priced_growth`).

**Every slice.**
- GEN-13: `python3 tools/validation/validate_claims_language.py --repo-root .`
  (also `tests/test_claims_language_surface.py`). It scans
  `exactReportLimitations.ts`, which is non-test desktop source.
- `python3 -m tools.deliverables --project projects/chirality-piping check`.
- Review: an independent Codex round per slice. The statement table and the
  claims check get specific scrutiny in (b), the screener and records in (c).

**This revision** ran the GEN-13 scan functions (`_scan_file`,
`_scan_retired_texts`) over the 19 statements and the 18 other texts it proposes
(§2.4, §3, §4, §5.1), treated as desktop product source: no finding. It also
checked each Old text in Appendix A for one byte-exact occurrence.

## 8. Reconciliation

- **DEL-08-01:** this design; ScopeOfWork :81 by A.1 G1 and G2;
  `deliverable.yaml` code and check pointers for the new files.
- **DEL-08-03:** ScopeOfWork :99 by A.1 G3; pointers.
- **DEL-04-07 Design** `desktop-exact-outputs.md` §4.5 by A.1 G5; its refusal
  stands until the owner opens it. `memory-budget-decision.md` figures in (c).
- **The user guide** §10 by A.1 G4.
- **The completeness map:** FR-REP-001 and FR-REP-003 rows (:202,:204), gap 1
  and the W-11 row (:541), by HELP_HUMAN.

## 9. Findings outside W-11

- **Pressure-free limitations.** The pressure-free report also carries none of
  its producer's published limitations; preview-physics-2's nine are bound to
  its table and include scope statements such as "No equivalent stress or code
  stress." A separate item, if wanted.
- **Run-record diagnostics.** The rendered report omits the run record's
  rule-pack exclusion diagnostics that #1613 put into the exports, on every
  route. A separate FR-REP-001 item.
- **Schema divergence.** The desktop's `user_supplied_values` categories
  (`component_provenance:<kind>`, `spring_hanger:<kind>`, and §4.1's new ones)
  are outside `schemas/report_sections.schema.yaml`'s `value_category` enum. The
  Rust struct accepts any string, and no test validates the desktop input
  against the schema. This predates W-11.

## Appendix A. Wording-route drafts

Each Old text below occurs exactly once in its file at the basis head (Python
`count == 1`), starting on the line given. They go through HELP_HUMAN and the
owner's wording route before they are applied.

### A.1 Texts the implementation changes behind the gate (not applied)

**G1. DEL-08-01 ScopeOfWork :81** (current-implementation paragraph; provenance clause; slice (c); `execution/PKG-08/DEL-08-01/ScopeOfWork.md`)

Old:

```text
the result coordinate system, or states "not recorded";
```

New:

```text
the result coordinate system, or states "not recorded"; it also records the provenance of materials, sections, components and supports and the model's library references, with the library store's findings on them (FR-REP-003);
```

**G2. DEL-08-01 ScopeOfWork :81** (current-implementation paragraph; rendered-report clause; slice (b); `execution/PKG-08/DEL-08-01/ScopeOfWork.md`)

Old:

```text
exact-route results are refused for the rendered report (`loadReferenceOutputRefusal`; owner ruling 10A, 2026-10-21);
```

New:

```text
exact-route results are refused for the rendered report by its output-policy cell (`surfaceOutputRefusal` on `rendered-report`; owner ruling 10A, 2026-10-21), and their sections (DEL-08-01 Design `exact-route-report-sections.md`) are built behind that refusal;
```

**G3. DEL-08-03 ScopeOfWork :99** (current-implementation paragraph; slice (c); `execution/PKG-08/DEL-08-03/ScopeOfWork.md`)

Old:

```text
supplies the result diagnostics and a session provenance record classified `private_project_data` / `private_only`;
```

New:

```text
supplies the result diagnostics, a session provenance record classified `private_project_data` / `private_only`, the component and spring-hanger records, a record for each material and section and for each support that carries a `provenance_record` (from that record where present, else from the entity's `provenance` note; a component's record is likewise taken from its `provenance_record` where present), and the model's library references, with the library store's `MODEL-LIBRARY-REF-UNRESOLVED` and `MODEL-LIBRARY-REF-VERSION-CHANGED` findings as provenance warnings;
```

**G4. User guide `docs/user_guide/index.md` :400** (§10, last sentence of the first paragraph; slice (c); `docs/user_guide/index.md`)

Old:

```text
materials, sections and libraries are not yet in it.
```

New:

```text
it also records the provenance of materials, sections, components and supports and lists the library records the model references, with the library store's findings on them.
```

**G5. DEL-04-07 Design `desktop-exact-outputs.md` §4.5 :209** (second sub-bullet of the first bullet; slice (b); `execution/PKG-04/DEL-04-07/Design/desktop-exact-outputs.md`)

Old:

```text
  - The renderable report's only limitation is a fixed technical-preview statement (`features/report/renderableReportInput.ts:343-357`). It carries none of the exact identity's published limitations, such as the D-3 exclusions.
```

New:

```text
  - The renderable report builds the exact identity's limitation sections, stress headlines and standing line behind this refusal (DEL-08-01 Design `exact-route-report-sections.md`); none of them reaches an output while the cells refuse.
```


### A.2 Texts only opening would change (for the opening decision; not applied)

O4 is stated against the text as it reads after A.1 G2. Its Old was checked
for one occurrence in that text. "[the owner's opening decision, quoted with
its date]" is replaced by the owner's words when the decision is made.

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
exact-route results are refused for the rendered report by its output-policy cell (`surfaceOutputRefusal` on `rendered-report`; owner ruling 10A, 2026-10-21), and their sections (DEL-08-01 Design `exact-route-report-sections.md`) are built behind that refusal;
```

New:

```text
an exact-route result is rendered with its sections (DEL-08-01 Design `exact-route-report-sections.md`) only when Export Result JSON would admit it, at the Rust reader's numerically eligible standing with the live native invocation ([the owner's opening decision, quoted with its date]);
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

**O13. DEL-04-07 Design `desktop-exact-outputs.md` §4.5 :205** (opening line; `execution/PKG-04/DEL-04-07/Design/desktop-exact-outputs.md`)

Old:

```text
**These stay refused in this unit** (ruling 10A, §8).
```

New:

```text
**These stayed refused in this unit** (ruling 10A, §8). The rendered report and the report package were opened later under DEL-08-01 Design `exact-route-report-sections.md` ([the owner's opening decision, quoted with its date]); the report JSON packet and the other fifteen surfaces still refuse.
```

