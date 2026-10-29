# Calculation report section composer (RT-R4)

**Basis:** `origin/main` at `b24256155d` (2026-10-28). Paths are relative to
`projects/chirality-piping/` unless they name a root path. Design before
implementation; nothing here is built. It serves release-target row RT-R4
("Calculation report with a section composer", Must) and exit criterion 4
("Shareable report") in `docs/RELEASE_TARGET_v1.0.md`, and it touches RT-R1,
RT-R3, RT-R5 and RT-R6. It builds on this deliverable's
`report-package-current-results.md` (W-18), `exact-route-report-sections.md`
(W-11) and `report-user-release.md` (W-18b), and reuses the table set of
DEL-08-04 Design `csv-export.md`. This design records no acceptance, issuance,
approval, compliance or lifecycle standing (F-PIP-3).
Standard claim fence applies (claims taxonomy per DEC-081).

## 1. Scope and gap

**What the report does today, on both routes** (pressure-free
`preview-physics-2`; exact `pressure-2`, `pressure-source-1`):
- **One fixed document.** `report_renderer` assembles eight governed sections
  in a fixed order (`SECTION_ORDER`, `report_renderer/src/lib.rs:47-56`;
  `assemble_report_sections`, `:936`). They are Model Input Summary, Load Cases,
  Results, Warnings/Assumptions/Provenance, Audit Manifest, Rule Pack References,
  Limitations and Professional Boundary Notice. A frame of title, meta line,
  status banner, local-private or user-release notice and blocked banner comes
  first. The HTML and the PDF (`pdf_emitter`) consume the same assembled
  sections. The print view is the canonical HTML plus a banner that names its
  hash (`derived_print_view`, `:2428`).
- **Results are one flat table.** It has one row per source row, in source
  order, in the five-field projection (`reportResultRows.ts:47`;
  `bindings::expected_report_result_rows`, `bindings.rs:145`). On an exact
  result the table is preceded by the exact block: the semantic contract, the
  standing line and the stress headlines (W-11 §3.2). The "Referenced result
  export" table lists every source row with its checksum.
- **The generator contract requires all eight kinds**
  (`REQUIRED_SECTION_KINDS`, `report_generator/src/lib.rs:12`;
  `validate_template_slots`, `:773`). The desktop declares all eight, each as
  `required: true` (`renderableReportInput.ts:1016-1029`).
- **Three boundaries re-derive the full row list:** the package producer
  (W-18 §2.3 item 6), the exact native render (`rederive`,
  `report_render_exact.rs:253`) and the user-release derivation (W-18b §2.6).
- **Not present.** Nothing lets the user choose content, order or case
  selection. Results have no per-family tables, extremes or sorted views. The
  report lists no load combinations, model input tables (geometry, supports,
  materials, loads), weight/CoG or support-load tables.

**What the composer adds.** The user chooses which optional content sections
appear, their order within their governed parent section, and per section the
case selection and the view. The governed sections and the frame cannot be
removed. A composed report stays bound to one exact result, states its
composition and shows how many result rows it carries.

**Criterion 4.** The criterion needs the §19.3 notice and a reproducible
canonical hash on a report the user releases, saves and prints. The composer
keeps the notice in every composition (§2.4). It makes the composition a
recorded report setting, so the same inputs, versions and composition give the
same bytes (R-08-01-010). The criterion's "either route" is outside the
composer: route 010 refuses exact results under scope R (W-18b §0, §10). See D-6.

**Checks that establish the gap closed** (each named again in its slice, §6):

| Id | Check |
|---|---|
| K1 | Every governed section, the frame notices and the required blocks render under every composition (Rust render test over every subset of the catalogue, and a sample of orders) |
| K2 | An input without a composition renders byte-identical HTML and PDF; every existing fixture, package witness and `del1005` witness is unchanged |
| K3 | Composed rows are a subset of the bound rows, each shown at most once; the per-section and total row counts are correct; selection parity between TypeScript and Rust over committed vectors |
| K4 | The same input and composition give the same bytes; the composition and its SHA-256 are printed in the Audit Manifest section |
| K5 | Each `REPORT-COMPOSITION-*` refusal (§2.5) fires on its input and on no other |
| K6 | A composed exact input passes the native custody binding and `rederive`; its standing line and headlines are present whatever the composition |
| K7 | A composed local-private package passes producer item 6 and is read back by the reader |
| K8 | A composed user-released package derives equal (§4.4); its notice and record rows are present; the reach scan passes with unchanged pins |
| K9 | The pre- and post-render linter gates run over composed sections; GEN-13 is clean |
| K10 | The suggested names follow DEL-07-01 `export-file-names.md` (§4.3) |

## 2. The composer model

### 2.1 Granularity

The eight governed sections keep their kinds, titles and order. Composable
sections are **subsections of a governed parent** (`<h3>` in the HTML, a
`Subheading` block in the assembled model, so the PDF matches). So the
generator contract and its required-kind check do not change, and no
composition can move the notice ahead of or behind anything (D-1).

### 2.2 Catalogue

Required content (always present; not offered as a choice):

| Parent | Required content |
|---|---|
| Frame | Title; meta line; status banner; local-private notice or user-release notice and record rows; blocked banner; the closing note on derived copies |
| Model Input Summary | Project and model identity, unit system and unit display summary, coordinate systems, **model hash**, input-manifest reference (today's table) |
| Load Cases | Today's load-case table |
| Results | The display-rule statement; on an exact result the exact block (semantic contract, **standing line**, stress headlines on both section bases); the row-coverage line (§2.6); the "Referenced result export" table of every row and checksum |
| Warnings, Assumptions and Provenance | All of it: diagnostics, assumptions, user-supplied values, provenance notes, library records (R-08-01-007; FR-REP-003) |
| Audit Manifest | Envelope references; **Software, Solver And Run** (the solver and engine stamp); the Report Composition table (§2.6) when composed |
| Rule Pack References | References and user-rule outcomes whenever a rule-check run is recorded; otherwise its "none" statements |
| Limitations | The technical-preview limitation, the exact-route limitations, the TBDs |
| Professional Boundary Notice | The PRD §19.3 notice and the sign-off table |

Composable sections. These are the RT-R1 results surfaces and the model input.
Every value shown is a bound row or a projection of the bound model:

| Id | Parent | Content | Options | Slice |
|---|---|---|---|---|
| `result_rows_all` | Results | Today's flat table (the default) | cases | C2 |
| `result_displacements` | Results | Translation and rotation rows | cases; view `all` or `extremes` (per case and component, the largest magnitude) | C2 |
| `result_forces_moments` | Results | Element force and moment rows | cases | C2 |
| `result_stresses` | Results | Stress rows, with their section-basis suffix on an exact result | cases; view `all` or `largest` (by magnitude, with a limit of 1-1000) | C2 |
| `result_support_loads` | Results | Support reaction rows, grouped by support, then case | cases | C2 |
| `result_other_rows` | Results | Every row of a kind outside the family table, for example the `spring_hanger_user_input_review` rows | cases | C2 |
| `load_combinations` | Load Cases | Combinations as authored: id, label, method, terms and factors | none | C5 |
| `input_geometry` | Model Input Summary | Nodes, elements and components, as entered | none | C5 |
| `input_materials_sections` | Model Input Summary | Materials and sections, as entered | none | C5 |
| `input_supports` | Model Input Summary | Supports and restraints, as entered | none | C5 |
| `input_loads` | Model Input Summary | Applied loads per case, as entered | none | C5 |

**Families.** A closed table maps each row `kind` to one family, so the
families partition the rows. It is a committed fixture,
`fixtures/reports/invented/report_composition_families.json`, which both the
TypeScript and the Rust code read. A kind not in the table falls to
`result_other_rows`, so no row can be lost by classification.

**Joins later, through the same mechanism** (not in v1):
- weight and CoG (RT-R1), which need a Rust mirror of `weightSummary.ts` and
  touch numerical meaning;
- the modal section (DEL-04-09 M3; this deliverable's `DEL-04-09` need);
- the hanger table (RT-A11);
- comparisons (FR-REP-004; DEL-08-06 sections).

### 2.3 Order, inclusion, options

- **The composition record** is `report_composition`, an optional member of
  `RenderableReportInput`, `#[serde(default, skip_serializing_if =
  "Option::is_none")]`. Its new structs are `deny_unknown_fields`, and the
  shared object-form check covers them (`object_form.rs:485`):

  ```json
  { "composition_version": "1.0.0",
    "sections": [ { "section_id": "result_stresses",
                    "cases": "all",
                    "view": { "kind": "largest", "limit": 25 } } ] }
  ```
- **Inclusion.** A listed section is included and an unlisted one is omitted.
  Each id is listed at most once, and only composable ids may be listed.
- **Order.** Within each parent, the list order. Parents keep the governed order.
- **Cases.** The value is `"all"` or a non-empty list of `case_ref` values, as
  the rows carry them (`<basis type>:<basis id>`). A row whose `case_ref` is
  `TBD` is shown only under `"all"`.
- **Views.** These are display orderings and selections of bound rows. They
  compute no new number. Magnitude compares the row's value after the report
  display rule's exact step, which shifts `Pa` to `MPa` by decimals
  (`csv-export.md` §3). It ranks within one display unit, breaks ties by source
  order, and treats `-0` as `0`.
- **Overlap.** `result_rows_all` excludes the family sections, so each row is
  shown at most once (§5 relies on this).
- **Units.** v1 has one report-wide display rule, the existing one (Pa shown as
  MPa, six significant digits). There are no per-section units (D-3).
- **No free text.** The composition carries no title, heading or note, so
  composition adds no wording to the report.
- **Default.** The composer opens on `result_rows_all` alone. While the user
  leaves that unchanged, the desktop sends no composition, so the output is
  today's bytes (K2).

### 2.4 Required content cannot be removed

The catalogue is closed. Required content is not in it, and a composition that
names a governed kind or an unknown id is refused. The renderer emits the frame
and the governed sections before and around any composed subsection, so no
composition reaches them. The same holds on every route because every route
renders through `assemble_report_sections`.

### 2.5 Refusals

These are blocking reasons of the render, with export blocked. Each also
refuses the package.

| Code | When |
|---|---|
| `REPORT-COMPOSITION-VERSION-UNSUPPORTED` | `composition_version` other than `1.0.0` |
| `REPORT-COMPOSITION-SECTION-UNKNOWN` | an id outside the composable catalogue, governed kinds included |
| `REPORT-COMPOSITION-SECTION-DUPLICATE` | an id listed twice |
| `REPORT-COMPOSITION-OPTION-INVALID` | an option the section does not take, a limit outside 1-1000, or an empty case list |
| `REPORT-COMPOSITION-CASE-UNKNOWN` | a `case_ref` that no bound row carries (names it) |
| `REPORT-COMPOSITION-ROWS-OVERLAP` | `result_rows_all` together with any family section |

A family section with no rows under its cases renders the statement "No result
rows of this kind for the selected cases." It is not a refusal.

### 2.6 Binding to one exact result

- **The composer selects; it never supplies.** The renderer input keeps the
  full bound row list, and each boundary keeps its equality check unchanged:
  - producer item 6;
  - the exact native render's custody binding and `rederive`;
  - the user-release derivation.

  The renderer applies the composition to those rows inside
  `assemble_report_sections`. So every shown value is a bound row, and the model
  hash, the run (`report_sections.run_ref`), the received-result checksum, the
  per-row checksums and, on an exact result, the standing line are the
  report's own, as today.
- **Row contract.** Selection needs each row's kind, entity and value. The
  five-field projection gains three fields:
  - `kind`, the source `kind`;
  - `entity_ref`, the source `entity_ref` or `null`;
  - `value_text`, the canonical number text the display rule starts from.

  Both helpers and `report_result_rows_parity.json` change together (W-18
  §2.3 item 6), so every boundary binds the new fields. The renderer prints
  them only in composed sections, so K2 holds.
- **Disclosure.** When composed, the Results section states "Result rows shown:
  n of N." Each composed subsection states its cases and view. The Audit
  Manifest section gains a "Report Composition" table: every catalogue id,
  `included` or `omitted`, its options, and the SHA-256 of the composition's
  RFC 8785 bytes. Every row stays listed with its checksum in "Referenced
  result export", so omitting a row from view never removes it from the binding.
- **Stale results.** The composition carries no result identity. It is applied
  to whatever result the report is built for, and every existing staleness
  guard keeps deciding:
  - the panel's render invalidation;
  - the save basis's `isCurrent` refs (W-18 §2.5), which gain the composition,
    so a change during the awaited build retires the save;
  - the native custody binding on an exact result;
  - the solve attestation on route 010.

  A composition kept across a re-solve whose cases no longer occur is refused
  with `REPORT-COMPOSITION-CASE-UNKNOWN`. It is never applied as an empty filter.

## 3. Persistence

| Option | Consequence |
|---|---|
| **(a) Session only (recommended for v1)** | The composition lives in the workspace session, is kept across re-solves and is lost on quit. It needs no file, no native command and no model-document change. The printed Report Composition table and digest let a user recreate it |
| (b) App-level presets (later slice C6) | Named compositions in a capped native file under app local data, read under a memory reservation (the `autosave-settings.json` pattern, `autosave_commands.rs:200`). Two new commands, so the `app_invoke_handler` pin moves (§5); persistence review |
| (c) In the project or model document | Not recommended. The composition would enter the model hash that binds the result: recomposing would change the model's identity and make the result stale. It is a model-document change (Agent 0). The transform format's `ReportSettings` booleans (`physical_transform_model.schema.yaml:1639`) could turn off the professional-boundary notice, so they are not reused |

**Recommendation: (a).** The model document is unchanged and nothing comes to
Agent 0 under (a) or (b). W-18b's user-release selection stays non-persistent
whatever is chosen here (W-18b §0).

## 4. Rendering and output

### 4.1 Paths

| Path | How the composition reaches it |
|---|---|
| Standalone render, pressure-free | The webview builds the input with `report_composition` and calls `render_calculation_report`. The renderer applies it |
| Standalone render, exact | The same, through the native exact binding (W-11 §3.6). `rederive` compares the full rows, which are unchanged by composition |
| Local-private package (009), both routes | `buildReportPackageRequest` passes the session composition into the renderer input. The producer runs item 6 on the full rows. `assemble_full_report_package` (`pdf_emitter/src/lib.rs:205`) renders the HTML and PDF from the same assembled sections |
| User-released package (010), pressure-free under R | §4.4 |
| Print / PDF | The print view stays the canonical HTML with its hash banner, so it carries the composition. The hash-bound PDF is the package member. A standalone PDF stays as today (not hash-bound) |
| Headless runner | Offers no composition. Its requests carry none, and its witnesses are unchanged (K2) |

### 4.2 What each context allows

| | Local-private (render; package 009) | User-released (package 010) |
|---|---|---|
| Routes | Pressure-free and exact | Pressure-free only, under R (W-18b §0) |
| Result sections | All of §2.2 | All of §2.2, as projections of the attested S rows |
| Model-input sections (C5) | All | All, derived natively from M. R already refuses library- and rule-pack-referenced models |
| Rule outcomes | Required whenever a run is recorded | None under R: the section states none (W-18b §2.1) |
| Provenance entries and library records | As today | Absent under R (W-18b §2.6) |
| Composition | Caller content, checked by the renderer and object form | An input C to the native derivation (§4.4) |
| Required disclosures | §2.2 | §2.2 plus the user-release notice and record rows. No composition can remove them |

### 4.3 File names (DEL-07-01 `export-file-names.md`, #1711)

- **Canonical HTML.** Today it is `openpipestress-[local-private-]report-<12
  hex>.html` (`RenderedReportPanel.tsx:380,388`); it becomes
  `swbpipe-report-run-<n>.html`.
- **Local-private package.** The chooser's suggestion becomes
  `swbpipe-report-run-<n>.opsproj`.
- **User-released package.** The suggestion becomes
  `swbpipe-report-shareable-run-<n>.opsproj`, with a new kind row
  `report-shareable` added in that PR.
- **`package_id` is unchanged.** It is manifest content, and changing it would
  change the package bytes. Only the name passed to the chooser
  (`lib.rs:207`) changes.
- **The composition does not enter the name.**
- **Smoke pins.** A rename that would change the owner-held
  `apps/desktop/native-smoke/expected.json` stops and is reported.
- **Finding.** The OS print-to-PDF name comes from the document title,
  `{project name} — Calculation Report`. That name is outside #1711's reach,
  since the desktop does not suggest it. It is left as is and noted for the
  RT-U2 walkthrough.

### 4.4 The user-released report

- The derivation becomes `E(M, S, m, I, R, H, U, C)` (W-18b §2.6).
  - C is form-checked against the closed catalogue and copied into the derived
    renderer input.
  - Every other member is still derived.
  - Whole-request equality therefore covers C, and C can only select among the
    attested rows and the projections of M.
- The record stays at `record_version` `1.0.0`:
  - The Report Composition table is in the HTML and PDF members, and the
    manifest's `content_binding` hashes those members, so the composition the
    user released is bound.
  - `scope.not_carried` keeps its constants. Sections the user omitted are
    disclosed in the table, not in the record.
- The dialog's line "It carries the model summary, the load cases and this
  solve's results." must change to name the user's sections. That text takes
  the wording route; the exact New is drafted in C4, not here.
- Tests:
  - `user_release_composed_keeps_required_disclosures`;
  - derivation vectors with compositions;
  - a forged C (an unknown id, a governed kind, an extra key) refused at native
    admission.
- D-4.

## 5. Native commands, memory and the reach scan

- **No new native command in C1-C5.** The composition rides inside the
  existing arguments of `render_calculation_report` and `save_report_package`.
  `app_invoke_handler` (`lib.rs:5282`; pinned at
  `user_release_reach_scan.rs:537-541`) is unchanged, so it needs no re-pin. C6
  (presets) adds two commands. Its PR re-pins the macro after review that it
  adds no `mod`, no `$parameter ::` and no scanned name, as #1665, #1717 and
  #1718 did.
- **The reach scan otherwise.**
  - No new document function: `render_calculation_report`, `derived_print_view`
    and `assemble_full_report_package` keep their signatures.
  - A new `report_renderer` module (`composer.rs`) is reached from
    `assemble_report_sections` only.
  - It uses only listed derives, serde keys and macro paths. If it needs an
    unlisted one, the list change is reviewed in that slice.
  - The scan runs unchanged in C2 and C4 (K8).
- **Memory.**
  - The composition is small: at most 11 entries, and case lists no longer than
    the bound case set. It is priced by the existing ingress lease of each
    command (`command_ingress::admit`).
  - The overlap rule (§2.3) shows each row at most once. So the composed row
    text is at most today's row table, and composition adds only the coverage
    line and the composition table. That stays within the save's W M and the
    exact render's W M_r (W-11 §3.6), which are measured, not proven.
  - C2 re-measures both on the witness inputs with every row section included.
  - C5's model projections are bounded by the model text, which is already
    priced on the exact render and the save. C5 measures them.
  - Any excess is priced by lease growth before assembly. No DEL-04-07 limit,
    cap, factor or admission rule is loosened.
- **No new `unsafe`.** The reporting crates keep `#![forbid(unsafe_code)]`, and
  `src-tauri` its `deny`.

## 6. Slices

Each slice is one PR.

| Slice | Content | Tests | Review gate |
|---|---|---|---|
| **C1 Contract** | The `ReportComposition` types (Rust, TypeScript), the catalogue and family fixture, the three row-contract fields in both helpers and the vectors, and object-form coverage. Nothing is visible | Row parity (new fields), object-form refusal of a positional composition, K2 on every witness | Codex; targeted regression (rows are bound content) |
| **C2 Renderer** | `composer.rs`: selection, views, coverage line, Report Composition table, the §2.5 refusals, HTML and PDF alike | K1, K2, K3 (selection vectors in both languages), K4, K5, K6, K9; working-memory re-measurement | Codex and **independent review**: the renderer serves the release path |
| **C3 Desktop, local private** | Composer UI in the rendered-report panel: the catalogue with checkboxes, keyboard reordering, options, and required items shown as such. Session state; render and package 009 wiring; save-basis refs; the #1711 renames for the HTML and the package | vitest (UI, guards, names), K7, a native smoke step | Codex and **independent review** (file writes) |
| **C4 User-released composition** | C in the route-010 derivation (Rust, TypeScript, vectors), the native check of C, the dialog text (wording route), the shareable name | K8, `user_release_composed_keeps_required_disclosures`, forged-C refusals, reach scan | Codex and **independent review** (release path, data boundary). Needs D-4 |
| **C5 Model-input sections** | `load_combinations` and the four `input_*` sections: Rust projection of M at the native bindings and in the derivation, the TypeScript mirror, vectors | Projection parity, the library-refusal interplay on 010, memory measurement, K9 | Codex and **independent review** (data boundary on 010) |
| C6 Presets (only if D-2 chooses (b)) | Native presets file and two commands; `app_invoke_handler` re-pin | Size cap, malformed and oversized file, reservation, re-pin test | Codex and **independent review** (persistence) |

The order is C1, C2, C3; then C4 and C5 in either order. C1-C3 deliver RT-R4 for
local-private reports on both routes.

## 7. Boundaries

- **No compliance claims.**
  - Section titles are mechanics words: "Stresses", "Support loads",
    "Displacements".
  - No section, view or option names a code, an allowable or a governing
    case.
  - The stress view ranks by magnitude only and carries no ratio or limit
    column.
  - The exact headlines keep "no governing basis is chosen".
- **No protected standards content.** The catalogue titles and the fixed
  statements are public template text, and the linter gates run on every
  composed section (K9). Model-input sections show only the user's own entered
  values. Rule packs stay at safe metadata (R-08-01-008).
- **Outcome wording follows RT-R3.** RT-R3's "within / exceeds user limit" is
  not yet confirmed; outcomes still read pass/fail. The composer adds no
  outcome text and no outcome column. Outcomes stay in the required Rule Pack
  References section through `user_rule_check_status_display`
  (`report_renderer/src/lib.rs:601`). When the RT-R3 wording decision is made,
  that function and its TypeScript and CSV constants change in the RT-R3 PR,
  and composed reports follow unchanged.
- **Claim fence.**
  - Composition carries no free text.
  - The §19.3 notice, the local-private notice, the user-release notice and the
    sign-off table are required.
  - A composed report is a decision-support artifact, like an uncomposed one.
    Composing it, saving it or sharing it records no acceptance, issuance or
    release by anyone but the user's own data statement under W-18b.

## 8. Open decisions for HELP_HUMAN / Agent 0

**Ruled by Agent 0, 2026-10-28.** The table keeps the proposals; where a ruling differs, the ruling governs.

- **D-1:** yes. The governed frame is fixed; only subsections are composed.
- **D-2:** yes, session-only for v1. A saved, reusable composition is a remaining item (Should): it would be a
  model-document change.
- **D-3:** yes, no per-section units in v1.
- **D-4:** yes, composition on the user-released route; the record stays 1.0.0, the composition is bound by
  `content_binding`. C4's independent review treats it as a permissions surface: a composition cannot drop a required
  notice or block and cannot reach unbound rows.
- **D-5:** one independent review on the exact Old/New, then Agent 0's confirmation. C1 starts once D-5 is confirmed.
- **D-6:** with the owner. C2 to C5 proceed regardless, since the composer does not depend on it.

| Id | Decision | Recommendation | Consequence | Blocks |
|---|---|---|---|---|
| D-1 | Composer granularity | Governed frame fixed; composable subsections only (§2.1) | The generator contract and the required-content guarantee are unchanged. Users cannot remove Load Cases or Limitations, or reorder governed sections | C2 |
| D-2 | Persistence | (a) session only for v1; (b) presets as a later slice if users ask for them | No model-document change, no file and no new command now. Users recompose after a restart | C3 (as built), C6 |
| D-3 | Per-section units | None in v1: one report-wide display rule, the existing one | Reports cannot show US units yet. A later report-wide SI/US option would use `core/units` in all three derivations, with a `unit_basis` statement as in the CSV. It changes numerical meaning, so it needs targeted regression checks and independent scrutiny | none of C1-C6 |
| D-4 | Composition on the user-released route | Admit it: C enters the derivation; the record stays 1.0.0; the composition is bound by `content_binding` (§4.4) | Shareable reports can be focused. The dialog text takes the wording route. The alternative, route 010 refusing any composition, keeps shareable reports full-length | C4 |
| D-5 | The commitment text (below) | Adopt R-08-01-011 through the wording grant with Codex | Gives RT-R4 an acceptance anchor in this deliverable | C1 |
| D-6 | Criterion 4's "either route" | Owner decision on exact-route results under user release (W-18b §10); not part of the composer | Until then a shareable report exists for pressure-free results only. Exact composed reports stay local private | criterion 4, not a C-slice |

**D-5, ScopeOfWork Old/New** (`execution/PKG-08/DEL-08-01/ScopeOfWork.md`,
CLM-011, line 143). The Old occurs exactly once, verbatim, as one line. The New
keeps that line and adds one row after it. This file does not edit the ScopeOfWork.

Old:

```text
> | R-08-01-010 | Report reproducibility tests shall verify deterministic output for the same model/result/rule-pack inputs, versions, and report settings, subject to explicitly documented volatile fields. | SPEC.md section 9; CONTRACT.md OPS-K-REPORT-1 | Future tests compare normalized report outputs and audit manifests. |
```

New:

```text
> | R-08-01-010 | Report reproducibility tests shall verify deterministic output for the same model/result/rule-pack inputs, versions, and report settings, subject to explicitly documented volatile fields. | SPEC.md section 9; CONTRACT.md OPS-K-REPORT-1 | Future tests compare normalized report outputs and audit manifests. |
> | R-08-01-011 | The report generator shall let the user compose a report from a closed catalogue of optional content sections, choosing which appear, their order within their governed section, and their case selection and view. The governed sections, the model identity and hash, the software, solver and run identification, the warnings, limitations and rule-pack references, and the professional-review notice shall not be removable. Every shown value shall be a bound result row or a projection of the bound model, and the report shall state its composition and how many result rows it shows. | `docs/RELEASE_TARGET_v1.0.md` RT-R4; PRD §19.1, §19.2 | Renderer tests show every governed section and notice under every composition and byte-identical output without one; selection parity between the desktop and Rust (DEL-08-01 Design `report-section-composer.md`). |
```
