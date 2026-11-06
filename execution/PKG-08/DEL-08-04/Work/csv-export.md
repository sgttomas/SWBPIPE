# CSV export of every results table (RT-R6)

**Basis:** `origin/main` at `389ca97d5f` (2026-10-28). Paths are relative to
`projects/chirality-piping/` unless they name a root path. Slices 2 and 3
(§5) are built. It serves release-target row RT-R6 and
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
| Result row tables, one per group, and the endpoint pair (`features/results/ResultsPanel.tsx`, two `<table>` elements) | the panel's row groups (`resultSemantics.ts`, `resultReader.ts`) and the endpoint pair of the result interpretation; adapter `resultRowsCsv.ts` | one file per group, every row the filters admit across all pages (`results-<group>`), and the endpoint pair (`results-endpoint-pair`) |
| Displacement extremes (`DisplacementExtremesPanel.tsx`) | `displacementExtremes.ts`; adapter `displacementExtremesCsv.ts` | every page of the current filter (`displacement-extremes`) |
| Stress and ratio table (`StressRatioTablePanel.tsx`) | `stressRatioTable.ts`; adapter `stressRatioTableCsv.ts` | one file per group, every page under the current mode and threshold, with the user-rule outcome columns (`stress-ratio`) |
| Support load summary (`SupportLoadSummaryPanel.tsx`) | `supportLoadSummary.ts`; adapter `supportLoadSummaryCsv.ts` | the shown support with its maximum and minimum rows (`support-loads`), and every support in one file, one row per support and case (`support-loads-all`) |
| Weight summary and centre of gravity (`WeightSummaryPanel.tsx`, two `<table>` elements) | `weightSummary.ts`; adapter `weightSummaryCsv.ts` | the weight table (`weight`) and the centre-of-gravity table (`weight-cog`) |
| Run and state comparisons (`features/comparison/RunComparisonView.tsx`, `StateComparisonView.tsx`) | `runComparison.ts` and `stateComparison.ts`; adapters `runComparisonCsv.ts` (which also holds the view's row list and filter) and `stateComparisonCsv.ts` | every page of the rows the filters match (`run-comparison`, `state-comparison`) |

Each exporting `<table>` carries `data-csv-export` naming its kinds, and its
panel renders an "Export CSV" control (`CsvExportControl.tsx`). The guard
(`csvTableCoverage.test.ts`) fails when a `<table>` under `features/results` or
`features/comparison` lacks the marker. All nine are exported.

**The `ResultsPanel` tables.** The two `ResultsPanel` tables show values
through the session's display-unit converter (`useDisplayQuantity`,
`convertDisplayQuantities`). Under SI or US display units, the Rust unit
engine (`core/units` `convert`) applies factors such as 0.0254,
4.4482216152605, 0.001 and pi/180 in binary64.
- **Resolution.** HELP_HUMAN engineering-judgment resolution, 2026-10-28:
  these tables export the panel's own converted value. The reason, verbatim:
  "converting stored values to display units in binary64 is routine
  engineering practice. π/180 has no exact decimal, so binary64 is the
  practical resolution and doesn't degrade the output. It also keeps C2's
  intent that the CSV and the screen agree."
- **The value.** The CSV writes the converter's binary64 output, the value
  the readout shows before any rounding, as its shortest round-trip decimal.
  It is obtained through `useDisplayQuantityResolver`, with the same
  preference, converter and request. Under the default "entered" display
  units the conversion is the identity, so the value is the stored value,
  exact. Each row also keeps its entered value and unit, and the readout's
  status.
- **`unit_basis`.** It is one value, in a fixed text form:
  - for entered units, `display units entered; values as stored, exact`;
  - otherwise, `display units <SI|US>; converted in binary64 by the session's
    display converter (core/units: stored unit to its canonical unit, then to
    the display unit); each formula below is approximate, from the
    converter's outputs for 0 and 1 stored unit`, then one entry per
    conversion, `<dimension> <stored>-><display> approximately
    v*<factor>[+<offset>]`, then `as entered: <dimension> <unit>, ...` for
    readouts left as entered.

  The entries are sorted by dimension, stored unit and display unit, and
  separated by `; `.
  - **Why approximate.** The converter performs two steps in binary64: the
    stored unit to its canonical unit, then the canonical unit to the display
    unit. Its unit definitions (`core/units` transforms) are not exposed to
    the webview; the catalogue command carries only prose, and only on the
    desktop. So the formula cannot restate the converter's operations, and
    it is labelled approximate. Applied in binary64 it is close to the CSV
    value, but it need not reproduce that value bit for bit. For example,
    `1000 degC` to `degF` is `1832`, while the formula gives
    `1832.0000000000043` (Codex review of #1726).
  - A value is never called exact where a factor was applied.
- **The other seven tables** keep the exact step of §3.

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
  - **The session converter.** The two `ResultsPanel` tables are the
    exception: they write the session converter's binary64 value (§2).
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
- **Streamed save (C1 native custody, DEL-04-05 `c1-native-custody-plan.md` §5).** A results
  group of a result held natively is written in parts, with no row ceiling, through the commands
  `begin_local_csv_stream`, `append_local_csv_stream`, `finish_local_csv_stream` and
  `abort_local_csv_stream` (`apps/desktop/src-tauri/src/native_csv_stream.rs`). They reuse the
  single save's code in `native_result_download.rs`: its evidence rules for this route, its file
  name check, its exclusive creation with ` (n)` collision suffixes, its partial-file cleanup and
  its receipt.
  - **Per chunk.** Every chunk (the first holds the BOM) carries its own screening and
    local-first evidence, taken by `controlRouteExport` on that chunk with the user's explicit
    intent, the controlled payload exactly the chunk. A text payload's route decision reads its
    path, not its characters, so each chunk's decision is the whole file's; it is still taken
    and checked for every chunk sent (`csvSave.ts` `csvChunkEvidence`).
  - **Binding and order.** A stream is bound to the calling webview and to the held result it
    began on; chunks are numbered from 0 and must arrive in order, non-empty and at most 4 MiB
    each (the webview sends at most 256 Ki UTF-16 units); finish states the chunk and byte counts
    written. At most four streams are open; each holds a file handle and counters, never text.
  - **Ending.** A stream ends exactly once: finish (the receipt), abort, any refused request
    (wrong webview, order, size, evidence or BOM), a write or flush failure, or the disposal of
    its result or the session reset. Each end removes the partial file when it is still the file
    the stream created, and states the cleanup. No request names a path or another job's bytes.
  - **The text.** It is the CSV the full rows give through the unchanged serializer and adapter
    (`serializeCsvTable` is `serializeCsvHead` then `serializeCsvRows`; the group adapter's
    columns depend only on a layout decided over every row). The rows are read twice by bounded
    native pages: the first pass keeps the distinct units and one row per conversion, which fix
    the header and the unit basis; the second writes each page under them
    (`resultGroupCsvStream.ts`). At most one page of rows and one chunk of text are held in the
    webview. The 200,000-row interim refusal is retired.
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
   - Built:
     - `csvTables.test.ts`: round trip and AC-001;
     - `csvTableCoverage.test.ts`: the guard;
     - `CsvExportControl.test.tsx`: the controls;
     - `reportQuantityDisplay.exact.test.ts`: the split is byte-identical;
     - `resultRowsCsv.test.tsx`: the `ResultsPanel` tables, one regression
       per conversion factor, with the panel's converter as the oracle.
   - Codex review.
4. **Streamed group CSV for a result held natively (C1, RT-U4).**
   - `resultGroupCsvStream.test.ts`: the parts equal `serializeCsvTable(resultGroupCsv(...))`
     byte for byte at page sizes 1 to 1,000, entered and converted units, one and mixed units,
     ties, quoting, formula guards, negative zero and non-finite values; a changed page fails
     by name; 250,001 rows stream with one page outstanding and one page of text per part.
   - `csvSaveStream.test.ts`: per-chunk screening and evidence, order, splitting without
     truncation, refusals before sending, abort with the native cleanup stated, receipts.
   - `NativeResultsPanel.native.test.tsx`: every group of the replay fixture, written through
     the stream from the actual Rust page and semantics replies, equals the browser panel's CSV
     of the full rows.
   - `native_csv_stream_tests.rs`: exact bytes and receipt; every evidence mutation refused per
     chunk with the partial file removed; order, size, counts and job refusals ending the stream
     once; webview binding; abort, disposal and reset; at most four streams; existing entries
     never replaced; write, flush, identity and cleanup failures stated truthfully; requests
     carry no path.

A rename that would touch `apps/desktop/native-smoke/expected.json` stops and
goes to HELP_HUMAN.
