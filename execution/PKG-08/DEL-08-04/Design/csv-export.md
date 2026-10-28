# CSV export of every results table (RT-R6)

**Basis:** `origin/main` at `389ca97d5f` (2026-10-28). Paths are relative to
`projects/chirality-piping/` unless they name a root path. Slices 2 and 3
(§5) are built, except the `ResultsPanel` row tables, whose value basis is
open (§2). It serves release-target row RT-R6 and
exit criterion 5 (`docs/RELEASE_TARGET_v1.0.md`). The commitments are
DEL-08-04-R14, AC-002 and VER-002 in this deliverable's `ScopeOfWork.md`.
This design records no acceptance, issuance, approval, compliance or lifecycle
standing (F-PIP-3).
Standard claim fence applies (claims taxonomy per DEC-081).

## 1. Decisions

HELP_HUMAN adopted the design on 2026-10-28 ("the design is ADOPTED") and
chose the recommended option in all four choices:
- **C1 (a): comparison tables are included.** RT-U5 leaves the comparison
  *envelope* export for later. CSV of the tables the comparison views show is
  part of RT-R6.
- **C2 (a): numbers are unrounded.** A value is the unrounded value in the
  column's displayed unit, written as the shortest decimal that reads back
  exactly. HELP_HUMAN: "Use the same unit-conversion function the panel uses,
  so CSV and screen can never disagree beyond display rounding. Add a
  round-trip test against that function."
- **C3 (a): a provenance preamble comes first.** Then a blank row, then the
  table. "The notice wording goes through the wording grant."
- **C4 (a): the encoding is UTF-8 with a byte-order mark and CRLF line
  endings.**

**File names** follow DEL-07-01 Design `export-file-names.md` (#1711):
`swbpipe-<kind>[-<qualifier>].csv`, with no project or model identity. This
design does not restate that rule.

**Commitment text.** HELP_HUMAN decided the R14, AC-002 and VER-002 text under
the owner's 2026-10-27 delegation, after three Codex wording rounds. No
governed scope change is needed, because DEL-08-04 already owns RT-R6.

## 2. Tables

The desktop results tables on `389ca97d5f`, each with the view model its
adapter reads:

| Table (component) | View model | Export |
|---|---|---|
| Result row tables, one per group, and the endpoint pair (`features/results/ResultsPanel.tsx`, two `<table>` elements) | the panel's row groups (`resultSemantics.ts`, `resultReader.ts`) | **open**: see below |
| Displacement extremes (`DisplacementExtremesPanel.tsx`) | `displacementExtremes.ts`; adapter `displacementExtremesCsv.ts` | every page of the current filter (`displacement-extremes`) |
| Stress and ratio table (`StressRatioTablePanel.tsx`) | `stressRatioTable.ts`; adapter `stressRatioTableCsv.ts` | one file per group, every page under the current mode and threshold, with the user-rule outcome columns (`stress-ratio`) |
| Support load summary (`SupportLoadSummaryPanel.tsx`) | `supportLoadSummary.ts`; adapter `supportLoadSummaryCsv.ts` | the shown support with its maximum and minimum rows (`support-loads`), and every support in one file, one row per support and case (`support-loads-all`) |
| Weight summary and centre of gravity (`WeightSummaryPanel.tsx`, two `<table>` elements) | `weightSummary.ts`; adapter `weightSummaryCsv.ts` | the weight table (`weight`) and the centre-of-gravity table (`weight-cog`) |
| Run and state comparisons (`features/comparison/RunComparisonView.tsx`, `StateComparisonView.tsx`) | `runComparison.ts` and `stateComparison.ts`; adapters `runComparisonCsv.ts` (which also holds the view's row list and filter) and `stateComparisonCsv.ts` | every page of the rows the filters match (`run-comparison`, `state-comparison`) |

Each exporting `<table>` carries `data-csv-export` naming its kinds, and its
panel renders an "Export CSV" control (`CsvExportControl.tsx`). The guard
(`csvTableCoverage.test.ts`) fails when a `<table>` under `features/results` or
`features/comparison` lacks the marker. Its only named exception is the two
`ResultsPanel` tables. Their values come from the session's display-unit
converter (`useDisplayQuantity`, `convertDisplayQuantities`). In the Rust unit
engine (`core/units` `convert`), that converter converts SI and US units in
binary64 through factors such as 0.0254, 4.4482216152605 and pi/180. So the
exact step of §3 does not cover these tables, and their basis is HELP_HUMAN's
open choice.

Later results tables join when they are built, through the same serializer.
Examples are the RT-A11 hanger table and the RT-R2 frequency and participation
tables. Model and input tables are not results tables: bulk change, the model
tree, the library, units and split into parts.

## 3. File form

- **Serializer.** One pure function, `features/result-export/csvTable.ts`,
  turns `{preamble, columns, rows}` into text. Every table goes through it.
- **Adapters.** Each table has an adapter beside its view model. The adapter
  reads the view model the panel renders. It recomputes nothing and reads
  nothing from the page. It therefore stays inside the governed result reader
  and keeps R9 and R12 (no bypass).
- **Preamble.** Key,value rows, column A the key and column B the value, in
  this order:
  - `notice`: the registry's BS-VALID canonical text, verbatim
    (`docs/claims_registry.md` §1). It is the one exported constant
    `CSV_PREAMBLE_NOTICE` in `features/result-export/csvTable.ts`, pinned by
    the GEN-13 anchor `MISSING_CSV_NOTICE` in the root
    `tools/validation/validate_claims_language.py`;
  - `export_context` = `local_private`;
  - `released_report` = `false`;
  - `table`: the CSV kind slug from `export-file-names.md`;
  - `software_version`: the application version (`tauri.conf.json`), as the
    report states it;
  - `model_hash`: `sha256:<hex>`. For a results table, the canonical SHA-256
    of the model the panel renders with, as the report computes it. For a
    weight table, that of the model its summary was computed for;
  - `result_hash`: `sha256:<hex>`, the `received_result` hash of the result's
    own analysis-run record (`analysisResultHashScope`). It is taken only
    from the current solved run's record, and only when the record's run id is
    the result's;
  - `analysis_route`: the result's `producer.semantic_contract_id`;
  - `case_or_filter`;
  - `unit_basis`;
  - zero or more `rule_pack` rows, with value `<id> <version> <checksum>`,
    only where the table has rule outcomes.

  - **Tokens.** HELP_HUMAN ruled, 2026-10-28, that a hash or route a result
    does not record is `not_recorded`, and that a table with no result states
    `not_applicable`. An export is never refused for either. The tokens are
    not hex and not `sha256:`-shaped, so the hash check (`isCsvHashValue`)
    never reads one as a hash. The model-only tables (`weight`, `weight-cog`)
    state `result_hash,not_applicable` and `analysis_route,not_applicable`.
  - **Two-sided tables.** The run and state comparisons state the suffixed
    keys in place of `model_hash`, `result_hash` and `analysis_route`, in this
    order: `model_hash_a`, `model_hash_b`, `result_hash_a`, `result_hash_b`,
    `analysis_route_a`, `analysis_route_b`. Side `a` is the view's Left side
    and `b` its Right side: the earlier run or state, then the later or
    current one. `case_or_filter` names the pair and the view's filters. For a
    recorded run, the model hash is the saved state's `model_hash`. For the
    current run, it is the hash of the model in its input manifest. A saved
    state has no result, so its `result_hash_*` and `analysis_route_*` are
    `not_applicable`.

  Then exactly one empty row, then the table's header row. These are machine
  keys, and no other prose is written. HELP_HUMAN confirmed the rows under the
  wording grant: "(d). Use BS-VALID alone as the `notice` row, verbatim, with
  all the machine-key rows as in r2." A required value that is missing is
  refused, never filled; the adapter states it with its reason (R11).
- **Columns.**
  - Rows keep their existing stable identifiers (result, node or element,
    case key) next to their labels (R10).
  - A numeric column states its unit in the header, for example `UX [mm]`.
  - A column whose rows mix kinds gets its own unit column.
- **Values.** A value is the number returned by the table's own conversion
  function for the column's displayed unit, unrounded. It is written as the
  shortest decimal that parses back to that number exactly.
  - **The exact step.** HELP_HUMAN ruled, 2026-10-28, that the report's
    display rule (`reportQuantityDisplay.ts`) is split into an exact step,
    `exactDisplayQuantity`, and a rounding step, `roundedDisplayText`.
    `displayQuantityParts` is their composition, byte-identical to the rule
    before the split. A pin test (`reportQuantityDisplay.exact.test.ts`)
    checks this against the earlier rule, and the shared parity vectors
    check it against the Rust producer.
  - **The conversions.** The rule performs one conversion: a value stored in
    `Pa` is shown in `MPa`, a decimal shift of six places. Every other unit is
    shown as stored, the identity. No displayed unit of these panels uses a
    factor that is not a power of ten. The step works on the value's
    canonical text as decimal digits, with no binary64 arithmetic, so it is
    exact for every conversion the panels perform. A later conversion with
    another factor multiplies by that factor's exact decimal in the same way.
  - **Writing the value.** The CSV writes the shortest binary64 decimal of
    the exact result when that decimal reads back as the exact result.
    Otherwise it writes the exact decimal text itself (`csvDecimal`), for
    example `123.45678912345679` for 123456789.12345679 Pa.
  - **Stored values.** The comparison views convert nothing, so their values
    are written as stored, in the rows' own units.
  - **Dimensionless values.** A dimensionless value (a ratio, a count) is
    labelled `[1]`.
- **Absent values.** An absent value is never a blank number. Its numeric
  cell, and its per-row unit cell, carry the table's stated reason, such as
  "withheld", "not emitted", "none" or "Reaction attribution withheld by the
  solver". The table's text constants are shared with the panel, so the two
  cannot drift. A numerical-health column carries the row's badge.
  Missing values and metadata keep their diagnostics (R6, R11).
- **Rule outcomes.** These use the text constants the table shows, so the
  RT-R3 wording reaches the CSV with no separate change. Rule packs appear by
  identifier, version and checksum only (R8).
- **Formula neutralization.** A text cell that begins with `=`, `+`, `-`, `@`,
  TAB, CR, LF or a full-width `＝`, `＋`, `－` or `＠` gets a leading
  apostrophe. Numbers are written as numbers, so a
  negative value is unchanged. Labels can come from an imported MBF, so they
  are treated as untrusted.
- **Determinism.** Identical source data, provenance and table settings give
  identical bytes. There is no timestamp. Rows keep the table's order, with an
  identifier tie-break.

## 4. Saving

- **Route.** The governed local-first route `DOTH-CSV-011` is added to all
  three route catalogues:
  - Python, `core/security/local_first_storage/route_control.py`;
  - TypeScript, `features/redaction-controls/redactionExportControls.ts`;
  - Rust, `apps/desktop/src-tauri/src/native_result_download.rs`.
- **Admission.** The route is admitted only for `local_private` output, with
  explicit user intent.
- **Native save.** The native command gets:
  - a `data:text/csv;charset=utf-8,` prefix;
  - a UTF-8 check;
  - the `swbpipe-<kind>-` name check from `export-file-names.md`.

  The request states `lossless_required=false`, because CSV is a derived view.
  Native admission accepts that only on this route.
- **Browser runtime.** It keeps today's anchor download.
- **Not shareable.** The CSV is local-private, like the result JSON. Sharing
  goes through the user-released report (RT-R5).
- **UI.** Each table panel gets an "Export CSV" control, reachable by
  keyboard. The status line names the saved file.

## 5. Slices and checks

1. **Docs.** This design and the ScopeOfWork text.
2. **Serializer, guard and route.**
   - The serializer and the formula guard.
   - `DOTH-CSV-011` in the three catalogues.
   - Checks:
     - serializer tests: preamble and layout, units, identifiers, absent
       values, byte-order mark, CRLF, neutralization and determinism;
     - route-catalogue parity tests;
     - native-save tests: admission with explicit local-private intent, and
       rejection without it or for any other audience.
   - Codex review, plus independent scrutiny for a persistence and permission
     route (Root `AGENTS.md`).
3. **Adapters and controls.**
   - An adapter and an export control for every table in §2.
   - Checks:
     - per-table round-trip tests, comparing each parsed numeric field
       exactly with the table's conversion-function value in the displayed
       unit;
     - adapter tests for AC-001 and the R9/R12 boundary;
     - a guard test that lists every `<table` under `features/results` and
       `features/comparison`, and fails when one lacks CSV export.
   - Built: `csvTables.test.ts` (round trip and AC-001), `csvTableCoverage.test.ts`
     (guard), `CsvExportControl.test.tsx` (controls), and
     `reportQuantityDisplay.exact.test.ts` (the split is byte-identical).
     The `ResultsPanel` tables wait on their value basis (§2).
   - Codex review.

A rename that would touch `apps/desktop/native-smoke/expected.json` stops and
goes to HELP_HUMAN.
