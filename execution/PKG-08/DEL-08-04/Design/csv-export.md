# CSV export of every results table (RT-R6)

**Basis:** `origin/main` at `389ca97d5f` (2026-10-28). Paths are relative to
`projects/chirality-piping/` unless they name a root path. Slice 2 (§5: the
serializer, the formula guard and the route) is built; the adapters and
controls of slice 3 are not. It serves release-target row RT-R6 and
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
| Result row tables, one per group (`features/results/ResultsPanel.tsx`) | the panel's row groups (`resultSemantics.ts`, `resultReader.ts`) | one file per group |
| Displacement extremes (`DisplacementExtremesPanel.tsx`) | `displacementExtremes.ts` | every page of the current filter |
| Stress and ratio table (`StressRatioTablePanel.tsx`) | `stressRatioTable.ts` | one file per group, with the user-rule outcome columns |
| Support load summary (`SupportLoadSummaryPanel.tsx`) | `supportLoadSummary.ts` | the shown support, and every support in one file (one row per support and case) |
| Weight summary and centre of gravity (`WeightSummaryPanel.tsx`) | `weightSummary.ts` | the weight table and the centre-of-gravity table |
| Run and state comparisons (`features/comparison/RunComparisonView.tsx`, `StateComparisonView.tsx`) | the views' comparison rows | one file per shown table |

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
  - `software_version`;
  - `model_hash`;
  - `result_hash`;
  - `analysis_route`: one or more routes, in one row, space-separated;
  - `case_or_filter`;
  - `unit_basis`;
  - zero or more `rule_pack` rows, with value `<id> <version> <checksum>`,
    only where the table has rule outcomes.

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
- **Absent values.** An absent value is never a blank number. A status column
  carries the table's stated reason, such as "withheld", "No result" or
  "Mechanics not solved". A numerical-health column carries the row's badge.
  Missing values and metadata keep their diagnostics (R6, R11).
- **Rule outcomes.** These use the text constants the table shows, so the
  RT-R3 wording reaches the CSV with no separate change. Rule packs appear by
  identifier, version and checksum only (R8).
- **Formula neutralization.** A text cell that begins with `=`, `+`, `-`, `@`,
  TAB or CR gets a leading apostrophe. Numbers are written as numbers, so a
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
   - Codex review.

A rename that would touch `apps/desktop/native-smoke/expected.json` stops and
goes to HELP_HUMAN.
