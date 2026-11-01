# C1 native custody: implementation plan

**Serves:** RT-U4 (exit criterion 9) and the usability of ordinary models. **Design:** phase C1
of `result-volume-design.md` (§2, §3.1, §3.2, §4, §4.1; on `claude/rt-perf-c-design`),
approved by SWBPIPE Agent 0 on 2026-10-31 on three conditions:

- C1 changes no published byte and no engine code.
- The app-process peak and the usable size are measured before the PR leaves draft.
- C1 keeps today's v0.3 persistence, with exact-length refusals by name.

**Basis:** main at `e1a483b6c`. This is a working plan. Each slice lands as a commit on
`claude/rt-perf-c1-custody`, and the C1 PR stays in draft until slice 7's measurement.

## 1. Data flow, before and after

**Today:**
1. The worker publishes the envelope.
2. `solve_jobs` retains it (lease R).
3. `take_preview_mechanics_job_result` moves it to the webview (custody 2R).
4. `previewService` decodes it, runs `JSON.parse` and `rememberReceivedText`.
5. Every panel, overlay, reader call (wasm), record builder and save reads `result.results`.

**After C1:**
1. The worker is unchanged.
2. `solve_jobs` keeps the bytes in **native custody**: one `HeldResult` (bytes and lease
   together, shared by `Arc` with the native readers in flight).
3. The webview gets a **handle** `{job_id, S}`:
   - S is built natively in one pass from the parsed envelope, under a pool lease priced by
     the ingress prescan;
   - detail comes as pages by native query;
   - the verdict, the analysis-run preview, Record Run and Save Project are native,
     reading the held bytes.
4. The webview never holds the envelope's text or its rows.

## 2. Slices, in order

Each slice lists its files, its checks and the existing tests it moves.

**Order (2026-10-31).** Slice 5's native builder and preview land before slice 4's switch:
`workspaceSession.ts` awaits `buildAnalysisRunPreview(output, ...)` before it commits a result and
paints, so the switch needs the native preview to exist first. Record Run's and Save Project's
connection to the job handle (the rest of slice 5) follows the switch.

### Slice 1: native verdict (done; review fixes and rulings in §8)

**Files:**
- `src-tauri/src/solve_jobs.rs`: the retained result becomes `Arc<HeldResult>`, so the bytes
  and their result lease are released together by the last holder. `held_result` lends it
  to native readers, and the take refuses `PREVIEW-SOLVE-RESULT-BUSY` while one reads.
- New `src-tauri/src/native_verdict.rs`.
- `lib.rs` registers command `read_preview_mechanics_job_verdict`, with args
  `{jobId, modelJson?, invocationJson?}` under `command_ingress` (raw body, interactive
  lease).

**The command:**
1. On the async runtime it only checks the ingress header, reserves the raw body's copy
   (the envelope of its length, a lease capped as in §8) and copies it. Tauri lends the
   body only by reference, so the one input-sized step left there is that memcpy, under
   its reservation. No prescan, parse or materialization runs there.
2. The copy, its lease and a registry handle move into a blocking task
   (`spawn_blocking`). The arguments' admission, the diagnostics and the read all run
   there, so none of them occupies a runtime worker.
3. **Admission.** The args are prescanned, and the lease grows by their admission bound
   before they are parsed (`command_ingress::admit_raw`).
4. **Prescan of the texts.** The held text, the model text and the invocation text are
   prescanned, without allocating, in the reader's order.
   - A text the prescan refuses is refused with the reader's own message (§4).
   - That message comes from a diagnostic parse of each text up to the refused one. Each
     runs under a diagnostic lease of its priced allocations, admitted first (§8).
5. **The parse leases.** The args lease grows by each argument text's priced parse
   (`text_parse_bytes`). The held text's `Value` materialization M, from the prescan, is
   admitted as a pool lease before the parse allocates anything (§8). A refusal is
   `RESOURCE_UNAVAILABLE`, typed.
6. It calls `operation_applier::reader_dispatch_json`, the same function the wasm export
   `reader_dispatch_json` wraps, on the held text and the argument texts, and returns the
   outcome text unchanged.

**Tests:**
- Rust (`native_verdict_tests.rs`):
  - lease and pricing;
  - unknown, untaken and taken jobs;
  - refusal by name when M doesn't fit;
  - the take busy while a reader holds the result, and the bytes and lease released
    together after a dispose during a read;
  - commands held at admission (each with a 2 MiB model text) and then inside the reader,
    more than the runtime reports workers, while another async command completes. It
    fails when the work runs on the executor;
  - a refused parse lease allocates nothing near the parse and never reaches the reader;
  - a refused diagnostic lease comes before any allocation proportional to the texts,
    and the priced bounds cover the diagnostic parse;
  - the 12 GiB cap on the parse and diagnostic leases and on the args lease (admission
    and growth);
  - each parse refusal, and the `RawValue` exception, byte for byte (§4);
  - the differential (§4).
- vitest (`baseReaderWasmEquivalence.test.ts`): the wasm dispatch text equals the native
  outcome's serde text, byte for byte. The wasm engine's parse refusals are the same
  literals as the Rust test's, and it reads the `RawValue` texts the native command
  refuses (§4).
- The webview is not switched in this slice.

### Slice 2: S projection (implemented; independently reviewed, with tested repairs)

**Files:**
- `src-tauri/src/result_summary.rs` (the command, its admission and S's header),
  `result_summary_tables.rs` (the ports of the builders) and `result_planes.rs` (the planes and
  the binary layout).
- `src/services/resultSummary.ts`: the webview's reader of S. No panel reads it before slice 4.
- `lib.rs` registers command `read_preview_mechanics_job_summary`, raw args body
  `{jobId, modelJson, standing}` under `command_ingress`:
  - `modelJson` is the model text the panels show (the builders' `model`);
  - `standing` is `null` (no reader standing) or `{eligible, findings}`, the reader standing the
    session holds for the result (`resultsSessionState.ts`). The tables withhold exactly as
    `overlayMayShowRow` and `caseWithheldReason` do with that standing and the health below.
  - S is a projection under the caller's model and standing, not independent verification of
    either; slice 4 binds and invalidates it by job, model text and standing.
- **The response** is binary: the magic `SWBSUMv1`, the header's length as a little-endian u64,
  S's JSON header, zero padding to 8 bytes, then little-endian f64 planes at the offsets and
  counts of the header's descriptors (`planes: [{name, offset, count}]`, `planes_bytes`), with
  nothing after the last. The reader refuses any other layout, and any code, ordinal or offset
  the header does not admit, with `RESULT-SUMMARY-MALFORMED`.

**S carries:**
- the envelope's identity members, status, summary, numerical quality, formulation basis,
  professional boundary and diagnostics;
- `contract_evidence` without the per-row combination evidence: `exact_combinations[].rows`,
  `.recoveries` and `.missing_sources` are counted in `combination_evidence_counts`, not copied.
  Other top-level members are named in `omitted_members`, not copied;
- the row count and the distinct units, in first-seen order (each consumer sorts them as today);
- the numerical health: `numerical_health_json` with the standing's findings, `null` without a
  standing;
- the tables, ports of the builders, which stay their oracle until the switch:

  | Table | Builder |
  |---|---|
  | `results_panel` | `countResultFamilies`, `countResultFrames`, `navigatorCases` |
  | `support_load_cases`, `support_load_summaries` | `supportLoadSummary.ts` |
  | `displacement_extremes` | `buildDisplacementExtremes` (an absent slot is `null`) |
  | `stress_ratio` | `buildStressRatioTable` without a rule-check run; each group's rows are a run of the stress planes |
  | `component_bodies` | `componentBodies` |
  | `key_results` | `buildKeyResults` without a rule-check run, not historical, without `standingText` |
  | `withheld_support_state_rows` | `withheldSupportStateRows` |
  | `overlay_cases`, `colour_quantities` | `resultOverlayCases`, `availableResultColourQuantities` |

  A row's family is read as `reader_verdict::row_semantics` reads it (the table of
  `reader_verdict::route`, the signature of `semantic_contract::signature_in`), without an
  outcome per row. The frames other than element local, global and unstated are in
  `localeCompare` order only where that equals UTF-16 order (frames of `a`–`z` and `_`, which
  every producer frame is); otherwise `others_ordered` is false and the consumer sorts them.
- **Planes:**
  - `display.*`: every row of a viewport kind (the nodal translations, rotations and
    displacement magnitudes per set, the support reactions, the member and station quantities of
    `RESULT_COLOUR_QUANTITIES`) and the rows the intensified-bending rows name as sources. Per
    row: its ordinal in `results` (the exact row a pick opens), its value bit for bit (NaN when it
    is not a finite number) and the members the overlays read, as codes into S's value table;
    the sources as ordinals.
  - `stress.*`: the stress groups' admitted rows, with value, case, case label, location and unit.

**Not in S, because it is session state or a user's choice, not rows:** the rule-check run's
groups and ratios and `componentBodyNotices`, which look rows up by id (slice 3); `standingText`;
and the overlays' drawing for a chosen case, quantity and resolution (reaction arrows,
deformation, rotation glyphs, colour), which runs on the display planes.

`source_block_recovery` stays omitted as the design requires. Slice 3's recovery query serves
its `body.status` to `ResultsPanel` and its evidence to `PhysicsSourceEvidencePanel` in bounded
pages; slice 4 switches those panels to it, without restoring a full-envelope fetch.

**Tests:**
- Rust (`result_summary_tests.rs`):
  - each of the 45 cases of `fixtures/results/result_summary_cases.json` equals its committed
    response in `result_summary_outcomes.json`, byte for byte (`RESULT_SUMMARY_OUTCOMES_WRITE=1`
    rewrites it), and each case with a findings standing reads the same through the command on a
    retained job;
  - per-row combination evidence is counted from the borrowed source before any copy;
    increasing the omitted payload does not create a row-scale temporary;
  - the planes against the rows, bit for bit; the layout, little-endian; every truncation and
    each malformed layout refused by name;
  - S's JSON part on a scaled invented rack: 12 times the station rows add less than 12 KiB, and
    twice the members about double it;
  - the parse lease refused before any allocation near the parse, typed and never reaching the
    projection, and refused under pressure; the response lease asked at the response's exact
    length after the counting pass, a refusal allocating nothing of the planes' size; unknown,
    untaken and taken jobs and malformed arguments refused; the take busy while a summary reads,
    and a disposal during a read leaving the read whole and the bytes and lease released
    together after it; more summaries held at admission and in the projection than the runtime
    has workers, while another async command completes.
- vitest (`resultSummaryParity.test.ts`): the cases file equals its generator
  (`test-support/resultSummaryCases.ts`: the result fixtures the builders' tests, the demos and
  the native replay use, each with its model, and 19 invented cases for withheld and
  non-eligible standings, missing members, tied and signed extrema, mixed units, other frames,
  row order, duplicate ids and the per-row combination evidence). For each case, every table
  equals its builder's output over the full rows on the same model and standing, the numerical
  health equals the wasm engine's, and the reaction, colour (every quantity, case and
  resolution), deformation and rotation overlays on the rows the display planes give back equal
  the same overlays on the full rows. The reader's refusals are checked on truncated, foreign and
  patched responses.

**Limits stated for review:**
- The numerical health parses the held text itself (`numerical_health_json` takes text; its
  `&Value` entry is private to the engine crate). S's own parse follows it, under the same lease,
  so the peak is one parse but the time is two. Sharing the parse needs a public entry over a
  parsed `Value` in `operation_applier`: an engine-lane change, outside C1.
- Unproven by parity: frame names outside `a`–`z` and `_` (flagged, not ordered); a family
  outside the semantic tables' vocabulary (the tables have none); `source_result_refs` that is not
  an array; model ids that are not strings.

### Slice 3: paged detail and batched semantics (implemented; independently reviewed, with tested repairs)

**Files:**
- `src-tauri/src/result_detail.rs` (the index and the four commands) and `result_detail_tests.rs`.
- `src/services/resultDetail.ts`: the commands' types, decoders and adapters. No panel, session or
  agent calls them before slice 4. `src/test-support/resultDetailEmulation.ts` answers them over the
  full rows with the TypeScript readers, for the parity test and slice 4's replay.
- `HeldResult` holds the index's slot; `lib.rs` registers the four commands (raw args bodies under
  `command_ingress`).

**The index** (a refinement of "built in the same pass as S": S parses into a `Value`, which keeps no
byte offsets, and S is built per model and standing, while the index is not). The first detail query
on a held result builds it, once:
1. the prescan of the held text (allocation-free);
2. one parse into a `Value`, under the parse lease of M, only to read the route's table, because
   `reader_verdict::route` reads the whole source; the `Value` is dropped before anything else
   allocates;
3. three passes that build nothing, through serde_json's borrowed `RawValue`: the last `results`
   member (as `Value` keeps the last of a duplicated key), a count of its elements, and their
   `(start, end)` ranges into an index allocated at its exact capacity, each row prescanned for its
   priced parse.

The index holds 16 bytes per row, the table, the largest row's priced parse and the scan
allowances. It lives in the `HeldResult` with its own lease and is released with the bytes (disposal,
take or session reset). Concurrent first queries wait for one build; a refused or panicked build
leaves none.

**Commands:**
- `query_preview_mechanics_job_rows {jobId, filter, offset, limit}` returns `{total, ordinals,
  rows}`, the rows as byte ranges of the held text, in publication order. `limit` is 1–1,000.
  - A page stops early (`truncated`) rather than exceed 64 MiB, today's whole-result limit, so it
    always holds at least one row, and paging on from `offset + returned` reaches every row. A row
    too large by itself is refused by name (`RESULT-DETAIL-RESPONSE-LIMIT`).
  - The filter has the panel's members (text, family, frame, case, kind, selected entity), the
    agent's (kinds and entities, each any of a list) and the plan's (set as `basis_ref.ref_id`,
    station as `metadata.location`, ids). `"all"` and empty lists admit every row, as today.
  - Each member reads the row as the TypeScript does: family through the route's table, as S's
    counts read it; the text search's members, truthiness, `String` and number text,
    `trim`/`toLowerCase` and substring.
  - Where the TypeScript filter throws (a `null` row, a `source_result_refs` that is not iterable),
    the query is refused by name (`RESULT-DETAIL-FILTER-TYPE-ERROR`).
  - An unfiltered page reads no row. A filtered one parses one row at a time.
- `lookup_preview_mechanics_job_rows {jobId, ordinals?, ids?, kinds?, match}`:
  - rows by exact ordinal;
  - by id or kind, the first (`results.find`) or last (`new Map(results.map(...))`) row.
  - Every row with an id is the query with `filter.ids`.
  - At most 1,000 keys, in one response within the limit.
- `read_preview_mechanics_job_row_semantics {jobId, ordinals}`: one `row_semantics_each` over up to
  1,000 rows with the held source, the table `readerRowSemantics` reads. It parses the held text
  each call, under the parse lease of M, as the wasm engine parses the source text each call. The
  public reader entries take the source, not its table; a table-level public entry would be an
  engine-lane change.
- `query_preview_mechanics_job_source_recovery {jobId, offset, limit, raw?}`:
  - what `ResultsPanel` and `PhysicsSourceEvidencePanel` read from `source_block_recovery`: its
    presence and kinds, the recorded status, `receipt_sha256`, `receipt_version`,
    `publication_sha256`, the recorded and physical case counts, the distinct-id and coverage
    agreement;
  - a page of the recorded cases' facts (id, selected method, outcome) and, with `raw`, each case's
    exact recorded text.
  - It parses `source_block_recovery` and `contract_evidence` alone, found by the borrowing pass,
    each under its prescan's lease. These are recorded labels; they grant no eligibility.
- The row, lookup and recovery responses are binary: `SWBDETv1`, the header's length (u64 LE), the
  JSON header, then the texts it lists by length.

**Tests:**
- Rust (`result_detail_tests.rs`, 12 tests):
  - unfiltered paging at limits 1, 7 and 1,000 rebuilds `results` byte for byte over 92 sources
    (the 52 file-backed corpus statements and the 40 cases' results), in order;
  - bounds and shapes: zero, past-end and huge offsets, limit 0 and 1,001, invalid arguments,
    empty, absent and non-array results, a duplicated member, whitespace and exponents kept;
  - the response limit, with pages still reaching every row;
  - the committed outcomes: 2,700 queries, 200 lookups and 58 recovery pages of
    `fixtures/results/result_detail_cases.json`, through the commands on retained jobs. Each must
    equal `result_detail_outcomes.json` (`RESULT_DETAIL_OUTCOMES_WRITE=1` rewrites it), and each row
    and case text must be its source token;
  - the batched semantics give `base_reader_outcomes.json`'s `rows.sha256` on every row of the
    corpus statements that pin it, which the wasm engine's `readerRowSemanticsJson` is also pinned
    to;
  - refusals by name where the TypeScript throws;
  - the index admitted before it allocates, refused allocating nothing of its size, and its lease
    held with the bytes through dispose, take and a read in flight; one build under concurrent
    first queries; a panicked build leaving none; query leases ending with the query; the
    executor (more commands held at admission and read than the runtime has workers, while
    another completes).
- vitest (`resultDetailParity.test.ts`):
  - the cases file equals its generator (`test-support/resultDetailCases.ts`: the summary cases'
    results, two retained-source pressure results, and 10 invented results for search over every
    member type, JavaScript number text, Unicode case mapping, duplicate ids, throwing rows, empty
    and absent results, a duplicated member, whitespace and incoherent recovery evidence);
  - every outcome equals the TypeScript readers over the full rows: `rowsUnderFilters` with its kind
    and selected entity, `paginateResults` on the panel's pages, the agent's `readResults`
    predicate, `find` and `Map` lookups, and `PhysicsSourceEvidencePanel`'s own rendering on the
    composite results;
  - the decoders refuse every other layout by name.

**Limits stated for review:**
- The parity's agent leg copies `readResults`' predicate (it is inline in `embeddedAgentSession.ts`).
  A change there would not fail it; slice 4's switch replaces that predicate with the query.
- Each filtered query and lookup parses every row, one at a time. Each semantics batch parses the
  whole source. Both are linear in the rows per call, as today's webview reads are. Slice 7 measures
  them; an index of the filtered members, or a public table-level semantics entry, is a later
  refinement.
- `toLowerCase` against Rust's `to_lowercase`: the same Unicode default case mapping, including
  final sigma. A character whose mapping differs between the two engines' Unicode versions would
  differ; the parity covers Greek, Turkish dotted I, ß and emoji.
- JavaScript can send a lone surrogate as an escaped JSON string, and its UTF-16 substring
  search can match half an astral character. Native serde refuses that query as
  `INGRESS-ARGS-INVALID`; slice 4 must display the typed refusal, rather than an empty page.

The recovery page cap counts the complete serialized header (receipt fields, job id and
lengths) and exact case text bytes before including a case. A page truncates before the cap;
an oversized singleton case is refused. Invalid filter-list bounds are checked before the
first source parse/index build. The semantics command uses Tauri's raw byte response body,
with a native response-body test and an ArrayBuffer adapter test covering that connection.

### Slice 4: the webview switch

**Files:**
- `previewService.ts`: no take. A completed job yields a `HeldResultHandle`
  (`{job_id, summary, planes}`), and `dispose_preview_mechanics_result` ends native custody.
- `workspaceSession.ts` and `WorkspaceSessionContext.tsx` hold the handle in place of
  `MechanicsResult`.
- `resultReader.ts`: the verdict, the route and the binding come from the slice 1 command.
  - **Cache key.** A verdict is cached by the job together with the exact model text and
    invocation text it was read with: the key is (job id, model text's SHA-256 or none,
    invocation text's SHA-256 or none), the texts `readerSourceText` gives.
  - **Invalidation.** An edited model is another model text, so it misses the cache and
    reads again. Disposing the job drops all of its entries.
  - The wasm path stays for saved and historical values until slice 6.
- Panels and overlays read S. The results table pages (§3).
  - The native semantics command returns raw JSON bytes (an ArrayBuffer through Tauri). The
    verdict command returns JSON through Tauri and is delivered as an already parsed value;
    its slice-4 adapter must accept that actual delivery type.
  - Clamp a past-end table page to the last page from the query's total, as `paginateResults`
    does; the low-level native query intentionally returns an empty page past the end.
  - S's cache key is (job id, model-text SHA-256, exact standing, including findings). An
    edited model or changed standing invalidates S; disposing the job drops its entries.
    The header's job id alone does not establish the model or standing binding.
- **Every C1 surface reads S or a native page, never the full rows** (design §2(3)). That
  includes the row lookups by id or kind (interpretation, Knowledge, Toolkit, the workspace's
  `findResult`) and the embedded agent's current `swbpipe_read_results` pages, through
  native paged adapters over the slice 3 query.

**Consumers that move only in C2,** by Agent 0's ruling of 2026-10-31, an interim exception
to design §2(3):
- **Who:** reports, comparisons, rule checks, stress-neutral and JSON derivative exports,
  and the handoff panels (handoff, local FEA, native package, headless runner).
- **Up to 64 MiB:** they keep full rows, as today, through
  `read_preview_mechanics_job_rows_all`. It is allowed only at or under PR #51's 64 MiB
  limit, and each such panel loads lazily.
- **Above 64 MiB:** each is refused by name (`RESULT-DETAIL-WEBVIEW-LIMIT`, stating size and
  limit) until C2. The refusal names what the person can still do: view the results (the
  summary, tables, viewport and paged detail work), plus #51's advice:

  > remove the load cases and combinations this review does not need (Load Cases
  > section); or solve the cases in smaller groups, each in its own copy of the model
  > document (File > Save Model Document…), with the combinations in a copy holding only
  > the cases they use; or split a large model into parts.
- **If C2 doesn't land for v1.0,** this refusal becomes a LIM entry in the release
  limitations list, as #51's refusal would.

**Tests:**
- `nativeMechanicsReplay.ts` emulates the new commands from the same fixtures.
- The App, session and panel tests switch to the handle.
- A test pins that no code path calls `take_preview_mechanics_job_result` any longer.
- A test pins that no C1 surface calls `read_preview_mechanics_job_rows_all`; only the
  listed C2 consumers do.
- The refusal's text, with its sizes and both kinds of advice, at 64 MiB and one byte
  above.

#### Slice 4 status (connected increment on `claude/rt-perf-c1-custody`, 2026-10-31)

**Connected.** A desktop solve now starts with `x-swbpipe-result-custody: native`
(`solve_jobs::ResultCustody`; absent or `webview` keeps every earlier caller's take and its 64 MiB
refusal at publication). A native-custody result has no displayable limit at publication (the
worker's capped grant and the result lease's pool admission bound it, unchanged), is never taken
(`PREVIEW-SOLVE-RESULT-NATIVE-CUSTODY`) and stays one `HeldResult` until disposal or reset.
`previewService.readNativeFreshResult` reads it natively: the header dispatch, the registration
(`validateCapturedSource`'s readers, run natively through `read_preview_mechanics_job_verdict` with
its new optional `reader` argument, which calls `operation_applier::reader_validate_json` under the
same admission), the standing for the solve's model (with the captured invocation for a registered
pressure-source statement, as `numericalResultStanding` reads it), S for that standing, the
recorded recovery facts for pressure-source, and the native record preview for the session's
manifest. The session commits a `NativeResult` and a `NativeAnalysisRun` (`services/nativeResult.ts`)
and never takes, parses, fingerprints, serializes or stores the envelope or the `ResultRef` array.

- **The handle.** `NativeResult` carries S's envelope members; its type has no `results`,
  `source_block_recovery` or `retained_precision`, so every full-row consumer had to be routed
  explicitly (the compiler listed them), and at run time those members, `JSON.stringify`,
  `structuredClone` and reader text throw `NATIVE-RESULT-ROWS-GUARDED`. `NativeAnalysisRun` is the
  preview's record header, guarded the same way for its row-scale members.
- **Delivery kinds.** The verdict arrives parsed (a JSON body); S, rows, lookups, semantics, recovery
  and the preview arrive as ArrayBuffers. `decodeNativeVerdict` refuses text and bytes.
- **Binding.** Verdicts are cached by (job, reader or dispatch, exact model text, exact invocation
  text); S by (job, exact model text, exact standing); previews by (job, manifest, aggregate,
  exclusions). Exact texts are the keys (stronger than their digests). Disposal marks the job
  dead and drops its entries; panels discard replies for another result.
- **C1 surfaces moved:** solve panel, solve proof, job audit, run audit, report lint, design
  workspace and export review counts and units (S's `row_count`, `units`; the record's
  `result_ref_count`, one `result_row` checksum per ResultRef); key results; displacement, support
  and stress tables (S; stress rows keyed internally, their ids resolved by lookup for selection
  and CSV); the viewport's reactions, colour, deformation and rotation (S's display rows,
  `summaryDisplayRows`, moved from the parity test into `resultSummary.ts`; the legend's row id
  resolved by lookup); the results panel (`NativeResultsPanel`: S's counts, frames and cases, one
  native page per view clamped from the query's total, batched page semantics, typed refusals,
  by-id detail and endpoint pair); Knowledge, Toolkit and diagnostic by-id and linked-row lookups;
  the load/reference-state admission; the result and diagnostic selections; the embedded agent's
  `swbpipe_read_results` (the native query with the agent filter, output unchanged).
- **C2 interim.** `C2ResultGate` gives reports, the rendered report, comparisons (on request),
  rule checks and the declarations editor's selector offers (on request), result JSON and
  stress-neutral exports, local FEA, headless runner, native package and handoff panels a full copy
  (`read_preview_mechanics_job_rows_all`, at or under 64 MiB, its custody lease held until release or
  disposal) registered by the existing readers on the exact copied bytes, and the full record built
  on it by the oracle builder and checked against the native preview's checksums. Above the limit
  they show `RESULT-DETAIL-WEBVIEW-LIMIT` with the sizes, what still works and #51's advice.

**Interim, not slice 5's persistence (now narrowed).** Save Project, Create Project and Record Run
of a native result went through the C2 copy and record; since slice 5's persistence (below) they go
from the job handle. Run comparisons with the current run and reference-inspection retention of a
native result still go through the C2 copy and record: at or under 64 MiB by today's path, refused
by name above it.

**Representation finding for slice 5 ("exact bytes").** Today's stores do not write the producer's
bytes: Record Run stores `jsonTextKeepingNegativeZero` of the webview's parsed result
(`analysisRunService.recordAnalysisRun`), and Save Project stores `serde_json::to_string` of the
value parsed from `JSON.stringify` of it (`write_project_rows`), and the project envelope hash the
session computes covers that result and record. So "store exactly today's bytes" from the held
bytes means the native writer must emit those two forms (the `Style::Stored` writer for Save; a
JavaScript-number writer keeping negative zeros, in the source's key order, for Record Run) and
compute the envelope hash natively; it cannot write the held bytes verbatim without changing what
is stored. This is stated for the slice-5 persistence review, not decided here. *Implemented as
stated (slice 5's persistence, below): today's two forms are written natively and the envelope
hash computed natively over what is stored; the held publication bytes are never rewritten.*

**Connection repairs (2026-10-31, same branch).** Defects found in the connected increment, each
repaired with a test:

- The C2 gate read the session's loader as an effect dependency; the loader is a new function on
  every session render and each load published a new copy state, so a mounted gate reloaded
  without end (the second solve of `App.keyResults` hung inside `act`). The gate now reads the
  loader when a load starts, and binds each request and outcome to the result and record it was
  made on; the session publishes the copy only when it changes.
- The Solve section is open when a solve lands, and its rule checks loaded the full copy at first
  paint. They now load on request (one gate). `App.nativeCustody.test.tsx` pins first paint: no
  take, no full copy and no TypeScript full record with Solve open and Results and Report dormant;
  one copy and one full record per job on request or section opening; none after a second solve
  until asked.
- A delivered copy that is not kept (unreadable, or its registration refused) now releases its
  custody lease (`release_preview_mechanics_job_full_copy`), so a later request can copy again.
- A native record's full record is built once on the job's copy and shared by the C2 consumers
  (each gate and save rebuilt it).
- Exact-route reports (W-11) of a native result were refused: the copy named no delivering job
  (`REPORT-EXACT-ADMISSION-BINDING-MISMATCH`), and `solve_jobs::custody_binding` answered only for
  a taken result (`REPORT-EXACT-CUSTODY-RELEASED`). The copy now names its native job while that
  job is held, and `custody_binding` also answers for a native-custody result while it is held,
  with the digests recorded at publication; disposal ends both. This changes DEL-08-01's §3.6
  custody rule (amended there). Agent 0 assigned the corresponding narrow DEL-04-07
  custody-binding/session-refusal reconciliation; it is included here and quoted old/new in #59,
  without changing identity checks, admission classes, budgets or caps.
- Create and Save Project after a reference inspection with no result failed (a TypeError): the
  session's run was skipped whenever a retained reference run existed, then read through. It is
  read again whenever the retained run lacks the result or record, as before C1.
- A refused native lookup in Knowledge showed the panel without its computed record; it now says
  the lookup was refused. The comparison builders take the record identity they read
  (`ComparedRun`), so the session passes a native record header without a cast.

The App and session tests were moved to the handle: counts and units through `resultRowCount` and
`resultUnits`; full rows, full records and saved bytes through the session's `loadC2Full` (as the
C2 consumers read them); refusals and failures that met the take now meet the first native read;
identity waits replace deep comparisons of guarded objects. Native replay: 235 requests from the
session and panel tests, each answered by its actual Rust command (`native_replay_tests`); sources
are committed fixtures by path, and the one re-serialized source is kept once. At `9950fd495`:
`tsc -b` clean; the full desktop vitest 7,349 of 7,351 pass, one todo, one failure
(`App.resultsNavigator`, below); the desktop build passes.

**Navigator and group CSV (connected, 2026-10-31, same branch).**

- **Navigator.** `NativeResultsPanel` steps the kind, case and support-or-element axes as
  `ResultsPanel` does (DEL-07-05 `results-navigator.md`). Kinds and cases are S's; the entity list
  is the new detail command `query_preview_mechanics_job_navigator {jobId, filter, probe}`
  (`result_detail.rs`): each distinct `entity_ref` of the rows the filters admit, other than
  "Selected entity only", once in first-publication order, with its first row's position among
  those rows and its ordinal, the admitted total, and a probed ordinal's admission and position. A
  step selects the first published row (its own id, by an ordinal lookup) and opens its native
  page. The webview holds the entity list (it scales with the entities), never the rows; the
  panel keeps the lists of its last four filters. A step made while its list is read waits for
  it; a reply for a superseded step, filter or result is discarded. A filtered `entity_ref` that is
  not a string is refused by name (`RESULT-DETAIL-NAVIGATOR-ENTITY-TYPE`), a stated difference
  from the full-row list, which would hold that value. A refused query now shows only its typed
  refusal, never also "no rows match" (a defect of the connected panel, repaired with a test).
- **Leases.** The navigator takes the args lease as every detail query, the index, a working lease
  of the fixed allowance and the largest row's priced parse, grown under the pool (by half again
  at least) before each new entity's text is copied, and the response lease of its exact length.
  It parses every row, one at a time, as a filtered query does.
- **Group CSV.** The 200,000-row interim and its in-webview assembly are gone. Each group's CSV is
  written through the native stream (`native_csv_stream.rs`, DEL-08-04 `csv-export.md` §4):
  `begin`/`append`/`finish`/`abort_local_csv_stream`, built on `native_result_download.rs`'s
  evidence rules, names, exclusive creation, collision suffixes, cleanup and receipt; each chunk
  screened by `controlRouteExport` with explicit intent and its evidence checked natively; bound to
  the webview and the held job; ordered, non-empty chunks of at most 4 MiB; at most four streams;
  ended exactly once, with the partial file removed, by finish, abort, any refused request, a disk
  failure, the job's disposal or the session reset. The text is byte for byte the formatters' CSV
  over all the group's rows (two bounded passes: units and conversions first, then rows), with at
  most one page of rows and one chunk of text in the webview. The endpoint pair keeps its
  single-save path for the complete matching end-row table. The endpoint read pages every
  station result and applies the legacy force/moment/stress family gate. The guard still counts
  nine tables at eleven render sites.
- **Checks.** `result_detail_tests.rs` (the navigator over every corpus source and filter member,
  probes, refusals, priced growth, leases); `native_csv_stream_tests.rs` (exact bytes, per-chunk
  evidence, order, size, counts, job, webview, abort, disposal, reset, collisions, disk failures);
  `NativeResultsPanel.native.test.tsx` (the list at filter boundaries, paged stepping, stale and
  disposed replies, lone surrogates, every group's streamed CSV against the browser panel's, all on
  the actual Rust replies); `resultGroupCsvStream.test.ts` (byte parity at page sizes 1 to 1,000,
  converted units, a changed page refused, 250,001 rows with one page outstanding);
  `csvSaveStream.test.ts`; `App.resultsNavigator.test.tsx` passes, waiting for the native list.

**Save Result JSON remains the existing derivative export (Agent 0, 2026-11-01).**
The existing `result-export-link` (`DOTH-JSON-001`, `save_local_result_json`) writes
`buildCurrentResultExport`'s derivative over the model, result rows, run record and manifest,
with full-document path/label screening, lossless materialization and an exact canonical
payload. It does not write the producer's result envelope.

C1 preserves that document and screening through the already-approved bounded C2 path:
at or under 64 MiB the existing derivative export remains available; above it the named
`RESULT-DETAIL-WEBVIEW-LIMIT` refusal remains. The earlier promise that Save Result JSON
would write the held bytes natively was incorrect and is superseded by this reconciliation
with `result-volume-design.md` §4/C2's consumer map and DEL-08-04. C1 adds no raw-envelope
output route, does not reuse `DOTH-JSON-001` for a different schema, and does not port the
derivative or its screen into C1. That port remains C2 work. C1 does not remove every export
size refusal. This resolves conflicting plan wording; it adopts no new output, schema or
permission. The exact old/new text and rationale are in #59 for the connected review.

**Remaining for this slice:** saved-result native reopening (slice 6), and the C2-copy interim for
run comparisons with the current run and reference-inspection retention. Native Record Run and
Save/Create Project are connected (slice 5's persistence, below). Navigator
stepping and bounded streamed group CSV are implemented. The native-read refusal is now a
session-step refusal, distinct from a legacy take refusal, under Agent 0's narrow DEL-04-07
reconciliation. Independent numerical, persistence and permissions scrutiny of the connected
implementation and governing text remains owed. Current large-model memory fit is unmeasured;
no App invocation or measurement follows from the source-mode tests.

**Recovered verification basis:** `1e56ccdff` has a passing desktop build and focused corrected
CSV test. The full desktop run at its parent has 7,368 passing tests and 1 todo; the full Rust
lib run has 1,055 passing tests and 13 ignored, with no failures. The final change only corrects
the test's display-unit preference from `si` to `SI`. Main's subsequent routing/fixture-only
updates are reconciled from the combined selected tree; affected integration checks and the
exact-head hosted harness remain required before integration.

**Connected-review repairs (after `2f493e1db`).** C2 session and gate states drop the old full
copy when the source is replaced or cleared, including dormant/explicit gates; they do not merely
hide an unleased reference. Endpoint-pair reads page through every matching station row and apply
the selected row's authoritative force/moment/stress family gate. Registration resource refusals
propagate as session-step failures; discarded jobs drop cached readings and caller-model captures;
a refused C2 re-registration releases its copy. Reference inspection checks model/session/result/
record/run generations after awaiting a copy so a newer action wins. Refused diagnostic queries
are visible, and record-size messages name the record. The new regressions and the already-required
independent recheck cover these repairs; native persistence/reopen and measurements remain pending.

**Nonblocking stream edges recorded by the connected review.** Under the existing local
containment model, a panic between stream checkout and checkin can leak a checked-out stream slot
and leave a partial file; abort/reset only poison that checked-out slot, so cleanup is not established; a begin racing disposal is retired on the next operation/reset rather
than immediately; and begin currently accepts an untaken legacy held job. These remain recorded
limits, not claims of tested recovery or owner risk acceptance. The review did not classify them
as blockers for this increment. Its full-copy reply and lone-surrogate replay exceptions remain
stated evidence limits; no actual App or memory result is implied.

### Slice 5: native analysis-run preview, Record Run and Save Project (v0.3, exact bytes)

**The record builder** (implemented and independently reviewed, with focused repairs; the
fresh-native connection now uses its preview header): `buildAnalysisRunV03` and its caller `buildAnalysisRunPreview` are ported to
Rust in the desktop crate (`analysis_run_v03_build.rs`). They read the held bytes and the small
inputs (the manifest evidence, the GUI rule-check aggregate and the rule-pack exclusions; the load
basis is the manifest's model's, as today) and build the record without a record tree.

- **Command** `read_preview_mechanics_job_analysis_run_preview`, raw args body `{jobId,
  inputManifest: {manifest_ref, manifest_sha256, canonical_bytes}, ruleCheckAggregate,
  rulePackExclusions}`. The manifest goes without its parsed copy; its canonical bytes fix it, and
  `verifyCurrentSessionInputManifest`'s checks run on them. The response is the header's JSON as
  a raw byte body (an ArrayBuffer through Tauri), read by `src/services/analysisRunPreview.ts`.
- **The header:** run identity, both checksums, the statuses, the rule-check status, the record's
  exact stored length, `result_ref_count`, `producer_diagnostic_count`, `carried_members`, the
  appended rule-pack diagnostics whole, and every record member except `result_refs`, the
  producer's `diagnostics` and the copied `contract_evidence`, `source_block_recovery` and
  `retained_precision`. Its size does not grow with the rows.
- **The bytes route** for the persistence that follows: `run_record_bytes_within(registry, raw,
  copy, max_bytes)`, the same checks, then the record's stored bytes. The exact length is computed
  first and checked against the caller's bound; over it, `ANALYSIS-RUN-RECORD-TOO-LARGE` is
  returned with nothing allocated. At or under it, a buffer of the exact length is leased and
  written, and its lease is returned with the bytes. It is not a webview command. The
  persistence commands use its `record_bytes` under SQLite's measured limit (Persistence, below).
- **Passes** over the one leased parse of the held text (M):
  1. the readers: `reader_verdict::route`, `binding_of` and the route's `validate`, or the retained
     reader (`retained_precision::validate`);
  2. the record contract's row checks, one row at a time;
  3. the received-result checksum, the canonical text streamed into SHA-256;
  4. the record checksum: the canonical projection streamed, each row's checksum streamed, each
     result ref written from the borrowed row;
  5. the stored length, counted;
  6. the validator's remaining checks. Duplicate ids are found with 16-byte entries per row (8-byte hash plus 8-byte row index),
     leased at their exact size and checked against the actual ids when hashes agree.

  The bytes route is a seventh pass, writing into the exact buffer.
- **Byte identity, and what is compared.** The stored form is `serde_json::to_string` of the
  `Value` the command ingress parses from the webview's `JSON.stringify` of the TypeScript record.
  That is exactly what `save_local_project` stores in `analysis_run_json`, and Record Run's recorded
  revision is the same serializer over that `Value`. So each number is JavaScript's binary64 text
  read back by serde_json (`-0` → `0`, `1.0` → `1`), and keys are in serde_json's (UTF-8) order.
  Checksums are `openpipestress_jcs_ijson_v1` over the same values.
- **Refusals in the TypeScript order,** each the TypeScript's message byte for byte. Where the
  TypeScript throws a `TypeError`, the native refusal is `ANALYSIS-RUN-INPUT-SHAPE-INVALID: <member>`.
  The row phase follows `Promise.all`: an interpretation refusal in any row comes before a
  checked-profile refusal in an earlier row.
- **Stated differences.**
  - A record nested past serde_json's depth is refused with `CHECKED-JSON-INVALID: recursion
    limit exceeded`, without the wasm engine's column.
  - Arguments are typed at ingress (`manifest_sha256` text, `ruleCheckAggregate` text or null,
    exclusions with text fields), as every native command's are.
  - A held text serde_json cannot read is `SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED`, as the dispatch
    reads it. The `RawValue` token key keeps slice 1's refusal.
  - The retained reader reads the held value rewritten in place to JavaScript's numbers, with
    negative zeros kept, as `validateRetainedPrecision` hands the wasm engine. The corpus's four
    integral-float receipt cases read the same either way, so the cases do not show this emulation
    to be necessary.

**The proof:** `apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json` holds
122 cases with the unchanged TS builders' outcomes (records as `JSON.stringify` writes them,
refusals, `TypeError`s). It is generated by `test-support/analysisRunV03Cases.ts` and byte-checked
by `analysisRunV03Parity.test.ts`. It lives outside `fixtures/` for the reason the desktop-shaped
records do: the publication corpus requires serde-equal receipts there.

The cases:
- every fixture `analysis_run.json` carrier's source (18);
- the records and refusals `analysisRunCompatibility.test.ts` builds;
- the retained successors;
- invented rule-pack, manifest, producer, checked-profile and `TypeError` cases;
- 21 preview cases with session manifests.

In all: 54 records, 62 refusals and 6 `TypeError` points.

`analysis_run_v03_build_tests.rs` requires the native stored bytes to equal `serde_json::to_string`
of each committed record, and both checksums to equal the record's own. Builder cases run the
builder; preview cases run the command and the bytes route on a retained job. The TypeScript record
written through the store's own `upsert_project` reads back as the native bytes, and
`recorded_revision` of the two is the same text. A one-character change to a committed record fails
the test. The TS builder stays the oracle until C2.

**Persistence** (connected on `claude/rt-perf-c1-custody`, 2026-11-01; `native_persistence.rs`,
`result_js_text.rs`, `services/nativePersistence.ts`; the submitted commands are unchanged for taken,
restored and browser-preview results):
- **Commands.** Two new async commands take the job handle and small arguments, raw args body:
  `record_preview_mechanics_job_analysis_run {jobId, inputManifest, ruleCheckAggregate,
  rulePackExclusions, expectedRecord: {run_id, record_sha256, received_sha256}, intakeText, request:
  {expected_base, state_id, new_state, run_labels}}` and `save_preview_mechanics_job_local_project
  {…same six…, mode: "create"|"save", projectId, projectName, model, editorIntents, proposal,
  selectedReviewTarget, modelHash, confirmedRevision, expectedBase}`. No result, record, result
  text, envelope hash or C2 copy is sent. Both replies are JSON bodies (Tauri delivers them
  parsed): Record Run's is `record_analysis_run`'s outcome; Save's is `{summary, proposal,
  selected_review_target, model_hash, project_envelope_hash, model_document_version,
  stored_model_hash, result: {outcome: "saved", run_id, result_bytes, record_bytes} |
  {outcome: "not_saved", code, message, result_bytes, record_bytes, row_bytes, limit_bytes,
  refused: {what, bytes}}}`, never the model, result or record. The session adopts the reply
  with its own model (the stored model's natively computed hash must be the request's) and keeps
  the native solve whether or not its result was saved.
- **Binding.** The job must be a native-custody solve still held, its held bytes must hash to its
  publication digest, and the manifest must name its solver mode (`NATIVE-PERSISTENCE-CUSTODY`);
  the record rebuilt from the held bytes and the request's inputs must be the previewed one
  (`NATIVE-PERSISTENCE-RECORD-STALE`). Record Run then applies the store's own checks to the
  rebuilt record's members that are not row-scale (`analysis_runs::checked_native_record`, the
  submitted path's validators factored, not copied) and its transaction (`plan_native_run`,
  `write_native_run`). Save applies the replacement guard, the withheld-run guard and the model
  document checks unchanged.
- **Intake binding** (`bound_intake`, after the custody checks and before the build). Both
  requests carry `intakeText`, the model text the job was sent (`previewService.deliveredIntakeText`:
  `checkedJsonText` of the captured model, exactly what `startPreviewMechanicsJob` sent; null once
  the result is not held or its registration does not hold), as the exact-route report re-sends
  its intake; the registry still keeps only the digest. Refused `NATIVE-PERSISTENCE-CUSTODY`, in
  order and before anything is written: the text absent or null; its exact bytes' SHA-256 not the
  intake digest recorded at admission (hashed in place, nothing allocated); its parse, priced as
  an argument's (`text_parse_bytes`) and admitted first as an interactive lease capped at
  min(configured budget, 12 GiB) (a refusal is the pool's typed one), not JSON; the parsed model
  not the manifest's `model_basis.model_payload` as JSON data with JavaScript's numbers
  (`same_js_data`: binary64 values, `-0` equal to `0`, members in any order; exactly equality of
  the two RFC 8785 texts). A manifest whose model differs under the same project id, sealed and
  accepted by the builder, is therefore refused; the session's member order and number texts are
  not compared. A non-string `intakeText` is the arguments' typed refusal. The parse and its lease
  end before the held result is parsed. Before this binding the manifest bound to the result only
  through the builder's model-ref and load-basis checks.
- **Today's forms.** Record Run's `result_json` is `jsonTextKeepingNegativeZero(JSON.parse(bytes))`,
  written from the held bytes in JavaScript's member order (array-index keys first, repeated keys
  at their first position with their last value), its numbers by `Number::toString` with negative
  zeros kept; `run_record_json` is the recorded revision (`Revision`: `recorded_revision`'s members
  in the record's stored form, its hash the canonical projection's). Save's `mechanics_result_json`
  is the builder's stored writer over the held value (serde_json's reading of `JSON.stringify`),
  `analysis_run_json` the transient record's stored bytes, and the envelope hash
  `computeProjectEnvelopeHash`'s over the stored values, streamed.
- **Exact lengths, before allocation and before writes.** Inside the write transaction the writing
  connection's `SQLITE_LIMIT_LENGTH` is read (rusqlite `limits`, no new package; the lock file is
  unchanged). Result/record values and their table, explicit index and search records are measured
  from the texts' counted lengths: the table row's record (SQLite's record format, `record_size`), each measured index key's record
  (bounded with an eight-byte rowid) and, for a project, the search text and its content record.
  The run-hash autoindex is bounded by the measured run row; label/snapshot rows retain the
  existing whole-transaction rollback backstop. A result and a record that each fit can make a row
  that does not. Above the limit Record Run
  refuses `RUN-RECORD-RESULT-TOO-LARGE (result_bytes, record_bytes, row_bytes, limit_bytes)` with
  nothing written and no output allocated (its plan writes nothing); Save falls back to the
  approved model-only save (`mechanics_result_json` and `analysis_run_json` `null`, the envelope
  hash over what was stored, the reply and summary message saying the result was not saved,
  `PROJECT-SAVE-RESULT-TOO-LARGE`), never over a stored run withheld for its size without the
  user's confirmed Replace; above the limit without the result it refuses `PROJECT-SAVE-TOO-LARGE`.
  A row SQLite still refuses (`SQLITE_TOOBIG`) is refused by name and the transaction rolled back,
  a new snapshot included. Save uses `PROJECT-SAVE-TOO-LARGE` for this no-save backstop, distinct
  from a successful model-only save's `PROJECT-SAVE-RESULT-TOO-LARGE` result code.
- **Evidence** (`native_persistence_tests.rs`, `result_js_text_tests.rs`, the TypeScript oracle
  `persistence_forms.json` from `persistenceFormsCases.ts`): on every preview case with a record
  (preview-physics, pressure, pressure-source and retained routes; aggregates; rule-pack
  exclusions) the native run, labels, snapshot, project and search rows equal the submitted
  path's byte for byte (but the project's two save times), with the same outcome, and the
  envelope hashes the TypeScript's; the edge texts (array-index and repeated keys, negative
  zeros, integral floats, number boundaries, escapes, non-ASCII and supplementary-plane keys,
  infinities) are written as JavaScript writes them; SQLite writes each row at the computed size
  and refuses it one byte lower; Record Run's boundaries (result, record, row ±1) refuse with
  exact lengths, no writes and no output reservation or allocation; Save's row boundary gives the
  model-only save and its oracle envelope hash; the withheld-run guard and confirmed Replace;
  binding refusals, and the intake binding (`persistence_binds_the_solves_intake_to_the_manifests_model`:
  each job started on the session's text, `intake_text` in the builder corpus, for Record Run and Save;
  absent, null, other bytes of the same model, one byte more, another job, a swapped and a nudged
  manifest model under the same project id, the intake before the record's identity, the mode before
  the intake, a typed non-string refusal, a disposed job, each with nothing written; the same model in
  another layout binds when the job was started on it); every lease ends. The session's main case runs on the actual commands'
  replies request for request (`native_persistence_replies.json`), the others through a unit
  fake that hands each test's fake store today's submitted request rebuilt with the TypeScript
  paths (`test/nativePersistenceFake.ts`).
- **On main with S1 (model document 0.9.0, merged at `136f3e086`).** The C1 corpora were
  regenerated from their generators on the combined tree: the builder corpus and persistence forms
  (manifests of the 0.9.0 models, the pressure-2 table's current digest from the core semantic
  contract, which the native builder reads directly), the summary and detail outcomes (Rust-written,
  TypeScript-checked), the native replay requests (rebuilt from every request the 60 replay test
  files send; the pressure-source requests recorded against the 0.8-era raw sources are gone) with
  their outcomes, and the persistence replies. The native summary's support families now include
  S1's `snubber` (`result_summary_tables.rs`, as `displacementExtremes.ts`), pinned by a snubber
  support in the summary corpus. No S1 source fixture, schema, core or pricing test was edited;
  historical 0.8 controls and the recorded-run table compatibility are S1's, unchanged.
- **Remaining:** the independent persistence, permissions and numerical review of this increment;
  saved-result native reopening (slice 6); the C2 interim for comparisons with the current run
  and reference-inspection retention; slice 7's measurement.

### Slice 6: saved and historical results open into native custody

- Opening a project or stored run reads `mechanics_result_json` or `result_json` natively
  into a `HeldResult`, under the same lease rule, and returns a handle. Its verdict, S and
  pages then work as for a solve.
- `HistoricalRunContext` and the model-state comparison take the handle.
- After this slice the wasm reader path is used only by tests.

### Slice 7: measurement and the PR

- The P2 kit samples the app process, its worker and WebContent at 500 and 1,000 nodes
  (`linear-ex`).
- The PR leaves draft only after that window. Until then "about 1,000 nodes" is a projection
  and appears in no user-facing text.
- The interim 64 MiB refusal (#51) becomes "over the worker budget" only for the S path. It
  stays for the C2-pending consumers (slice 4).
- The window reports the native verdict's own working memory (the hashed id set, the
  evidence indexes, the semantics batch) as a measured peak, separately from the leased
  parse (Agent 0, 2026-10-31). If it is a material share of the app peak, C1 gains a bound
  or a lease for it before the PR leaves draft, and the proposed DEL-04-07 note does not
  call it an unpriced residual. That note goes to the owner only after this measurement.

## 3. Which webview features read `result.results`, and their replacement

| Feature (file) | What it reads | C1 replacement |
|---|---|---|
| Results table (`ResultsPanel.tsx`) | groups, counts, frames, navigator cases, rows | S's counts and cases, plus paged rows (slice 3) |
| Displacement extremes (`displacementExtremes.ts`) | nodal rows | S table |
| Support loads (`supportLoadSummary.ts`) | support rows | S table |
| Stress ratio (`stressRatioTable.ts`) | station and ratio rows | S table |
| Weight and component body (`componentBody.ts`) | body rows | S table |
| Key results (`keyResults.ts`, `useKeyResultsSession.ts`) | governing rows, contact counts | S table |
| Interpretation (`resultInterpretation.ts`), known limitations (`knownSemanticLimitations.ts`) | lookup by id, kind presence | a by-id page and S's kind set |
| Viewport (`resultColourOverlay.ts`, `deformationOverlay.ts`, `reactionOverlay.ts`) | all drawable rows | S planes, S reaction table, row ids from the planes |
| Solve panel, solve proof, job audit (`SolvePanel.tsx`, `solveProof.ts`, `solveJobAudit.ts`) | `results.length`, units | S's row count and units |
| Workspace preview and binding (`workspaceSession.ts`) | `buildAnalysisRunPreview(output)`, `model_ref`, `run_id`, row count, row lookup | native record preview (slice 5's builder, implemented ahead of slice 4); S's header fields; by-id page |
| Design workspace, export review, export unit disclosure, run audit, report lint (counts and units) | `results.length`, units | S |
| Reports (`reportResultRows.ts`, `ReportPanel.tsx`, `renderableReportInput.ts`) | every row | C2; in C1 full rows up to 64 MiB, refused by name above (slice 4) |
| Comparisons (`runComparison.ts`, `withinRunComparison.ts`, `ComparisonPanel.tsx`) | every row | C2; in C1 full rows up to 64 MiB, refused by name above (slice 4) |
| Rule checks (`ruleCheckService.ts`, `RuleCheckRunPanel.tsx`, `DeclarationsEditor.tsx`) | every row | C2; in C1 full rows up to 64 MiB, refused by name above (slice 4) |
| Stress-neutral export, handoff, local FEA, native package, headless runner panels | every row | C2; in C1 full rows up to 64 MiB, refused by name above (slice 4) |
| JSON export (`resultExportAdapter.ts`) and Save Result JSON (`save_local_result_json`) | every row (the derivative document) | C2; in C1 full rows up to 64 MiB, refused by name above (Save Result JSON writes the derivative, not the envelope: slice 4 status) |
| Embedded agent (`embeddedAgentSession.ts`) | filtered rows | C2 (catalog v5); C1 serves its current pages from the detail query, unchanged output |
| Knowledge and Toolkit lookups (`KnowledgePanel.tsx`, `ResultQuantity.tsx`) | rows by id or kind | native by-id and by-kind page (slice 3), no full fetch |
| Analysis-run record (`analysisRunCompatibility.ts`) | every row | native builder (slice 5) |

## 4. Verdicts stay identical

- **The same code.** The wasm export `reader_dispatch_json` is
  `operation_applier::reader_dispatch_json`, wrapped. The native command calls that same
  function on the held text, and the slice 2 summary reuses its parse.
- **The serde_json feature sets differ in one feature.**
  - Both targets have `float_roundtrip` and neither has `preserve_order` or
    `arbitrary_precision`, so an ordinary JSON text parses to the same `Value`, and writes
    back to the same text, on both.
  - The desktop crate also enables `raw_value`. Under it, an object key equal to
    serde_json's private `RawValue` token is read as an embedded raw value; the wasm engine
    reads it as an ordinary key.
  - The native command therefore refuses any text holding that key through the prescan:
    the held result, and the caller's model and invocation texts alike. It returns
    `WASM-ENGINE-INPUT-JSON-INVALID: <label> is not valid JSON: reserved RawValue token key
    at byte <n>`, where the wasm engine would read the text. This is the one permitted
    difference.
  - The engine never publishes the key, and a solve's model body holding it is refused at
    the solve's own ingress. Tests: `a_raw_value_token_key_is_the_one_refused_exception`
    (Rust) and the wasm leg reading the same three texts.
- **Texts that aren't JSON get the reader's text, byte for byte.**
  - The prescan refuses them without allocating. The refusal is then serde_json's own
    error, from a parse into a discarding type through `Value`'s parser path (no tree is
    built), formatted as `operation_applier`'s `parse_reader_input` formats it.
  - The order is the reader's: source, model, invocation. A text the prescan admits but
    serde_json refuses (a number beyond binary64) before a refused later text is reported
    first, as the reader would.
  - Tests: nine cases with full-byte literals in Rust (`NOT_JSON_CASES`, also checked
    against `reader_dispatch_json`'s own error) and the same literals through the wasm
    engine.
- **The proof on the reader corpus**, three legs, all in the ordinary checks:
  1. `result_export/tests/base_reader_outcomes.rs` pins the native `reader_verdict` outcome
     of all 561 corpus inputs (`fixtures/results/base_reader_outcomes.json`).
  2. `baseReaderWasmEquivalence.test.ts` runs the wasm engine on the same 561 inputs. Its
     dispatch text must equal the native outcome's serde text **byte for byte** (added in
     slice 1; previously compared by value). The TS builder `baseReaderInputs` gives the
     inputs.
  3. The desktop's `native_verdict_tests.rs` puts each file-backed corpus statement in a
     real `SolveJobRegistry` as a retained result. That's the 52 base statements with their
     models and invocations, including the two demo results. Each goes through the command
     path, with argument admission and the pool lease. The returned text must equal the
     committed outcome's serde text, byte for byte. An `#[ignore]` leg reads all 561 inputs
     from `BASE_READER_INPUTS_OUT` (`inputs.jsonl`, written by leg 1) and checks the same.
- **Refusals and diagnostics:**
  - A header refusal and the standing findings are the outcome text itself, so they're
    covered by the byte equality. Parse refusals are covered above.
  - The command adds only the `RawValue` exception and ingress and resource refusals,
    which are typed `RESOURCE_UNAVAILABLE` and `INVALID_REQUEST`, as for every native
    command.
  - Diagnostic parses also require adaptive pool admission. Under critical pressure or
    insufficient headroom, even a malformed text can return `RESOURCE_UNAVAILABLE`
    before its JSON diagnostic is computed. Native consumers must preserve that typed
    resource refusal rather than report a parse failure; the JSON diagnostic becomes
    available when memory permits the diagnostic parse.
  - Diagnostics move to S in slice 2, copied from the parsed envelope and checked equal to
    `result.diagnostics`.

## 5. CSV for all 9 tables

The guard `csvTableCoverage.test.ts` counts 9 marked tables:
- 2 comparisons;
- displacement extremes;
- support loads;
- stress ratio;
- weight and centre of gravity (2);
- the results groups;
- endpoint pairs.

**How each keeps working:**
- **S-backed tables** (extremes, support loads, stress ratio, weight, centre of gravity):
  their CSV is built from the same table rows as today, because S carries those rows.
  Unchanged.
- **Results groups:** the table pages, and the CSV pages through the native query in chunks.
  Each chunk is formatted by today's TS formatter and appended through the native CSV stream
  (`native_csv_stream.rs`, on `native_result_download`'s code), so the webview never holds the
  whole group (built: slice 4 status).
- **Endpoint pairs:** all legacy-matching end rows, read through bounded query pages and saved
  by the existing single-save route; the first 1,000 candidates are not treated as a complete set.
- **Comparisons:** C2; in C1 full rows up to 64 MiB, refused by name above (slice 4).

**Tests:**
- The guard stays at 9.
- `NativeResultsPanel.native.test.tsx` checks that the streamed CSV of each results group equals
  the browser panel's in-memory CSV on the replay fixture; `resultGroupCsvStream.test.ts` checks
  the parts against the formatters at every page size.

## 6. Tests that pin the old flow

They're updated in the slice that changes the flow, never deleted without a replacement.

**Rust:**
- `solve_jobs.rs`: take, custody 2R, retention and custody counts, dispose and
  `dispose_all_results`. These move in slice 4, when custody becomes native and the take
  retires. Slice 1 keeps all of them.
- `memory_pool.rs` interactive credit for Result and Custody reservations.

**TypeScript:**
- `previewService.test.ts`, `exactResultBytes.test.ts`, `test/nativeMechanicsReplay.ts`
  (the take and dispose emulation);
- `App.test.tsx`, `App.keyResults.test.tsx`, `App.resultsNavigator.test.tsx`;
- `solveJobResource.session.test.tsx`, `currentResultUnitPolicy.test.tsx`;
- `pressureSourceBridge.test.ts`, `pressureSourceIntegration.test.tsx`,
  `retainedPrecisionIntegration.test.tsx`;
- `ruleCheckService.test.ts`, `ruleCheckLoadReferenceDisclosure.test.tsx`;
- `resultReader.test.ts`, `baseReaderWasmEquivalence.test.ts`;
- `analysisRunCompatibility.test.ts`;
- the table builder tests (`displacementExtremes`, `supportLoadSummary`, `stressRatioTable`,
  `keyResults`, `resultColourOverlay`);
- #51's refusal tests, once #51 merges.

## 7. Open questions and risks for the manager

1. **Interim limit for C2-pending consumers (slice 4): agreed by Agent 0 (2026-10-31).**
   They keep full rows up to 64 MiB and are refused by name above it until C2, with the
   refusal text and LIM condition in slice 4.
2. **The run record is O(rows) too.** The v0.3 record holds one `ResultRef`, with its
   checksum, per row: about 1 kB per row, larger than the row. At about 1,000 nodes it would
   exceed SQLite's 1e9 length well before the result does, so Record Run is refused there
   until C2's v0.4. The analysis-run preview must therefore stay native (slice 5), not be
   held in the webview.
3. **Pricing residual.** The prescan prices the `Value` parse only. The readers' working sets
   (id vectors, maps over rows) run under that lease unpriced, as `command_ingress`'s residual
   already states for command working memory. S's response construction is leased at its exact
   length until the command returns. The returned buffer and Tauri's delivery copies, including
   concurrent replies, remain an explicit slice-7 residual after that return. S's
   other working set is bounded by entities and sets, not rows (the tables' `Value`s, about
   1.4 kB per listed displacement or support value, the value table and the header text), plus
   the ids the intensified-bending rows name. Slice 7 measures them; if they're material, the
   lease adds a working-memory factor, as the report package's does. Slice 3's detail queries
   price their row parses, the semantics' source parse and their responses (§8); unpriced are
   `route`'s and `row_semantics_each`'s working memory, one row's search text, and the delivery
   copies after return.
4. **Admission class: confirmed by Agent 0 (2026-10-31),** pressure-gated through the
   shared pool, on two conditions now implemented in slice 1 (§8):
   - the lease is sized before any parse allocation;
   - each lease is capped at the 12 GiB default.

   A lease needing more than 12 GiB, or memory outside the pool, is a policy change and
   goes to Agent 0.
5. **Concurrency with the reader bundle.** Slice 1 calls `reader_dispatch_json` and
   `reader_verdict` and edits neither. The reader bundle's verdict-identical changes are
   covered by the same differential.
6. **Reconciliation:**
   - DEL-04-05 (this plan, and the measurement in slice 7);
   - DEL-07-07 (desktop result custody: the declared source list gains
     `native_verdict.rs` and the slice 2–3 files);
   - DEL-08-04 (CSV and exports: slice 4's paged CSV, and the interim 64 MiB limit for
     exports);
   - DEL-04-07: the narrow custody-binding/session-refusal reconciliation assigned by Agent 0
     is included; all other adopted memory policy is preserved. §8 states how C1 leases fit it,
     and the separate proposed factual note remains in the PR body for owner adoption;
   - DEL-14-0x (the run record, slice 5).

## 8. C1's leases under the adopted pool policy

These are the shared-pool policy of DEL-04-07 `memory-budget-decision.md` (owner-adopted
2026-10-11) as C1 uses it. C1 adds no budget and changes no pool rule.

**Every lease the verdict command takes is pool-admitted before what it covers is
allocated.** None exceeds the configured per-invocation budget, nor, whatever budget is
configured, the owner-adopted 12 GiB default. The cap is checked under the pool lock.

- **The args lease** (`MemoryPool::acquire_interactive_capped`). One interactive lease:
  - on the executor, reserved for the raw body's copy (the envelope of its length) before
    the copy;
  - in the blocking task, grown by the args' admission bound (`command_ingress::lease_bytes`)
    before they are parsed;
  - grown again by each argument text's priced parse (`text_parse_bytes`) before the
    reader parses it.

  The capped budget is kept on the lease, so every growth (`Lease::grow_to`) is refused
  `ingress_exceeds_budget` above it, under the pool lock.
- **A diagnostic lease per diagnostic parse** (`MemoryPool::acquire_result_parse`). It is
  taken only when a text is refused, before each text up to it is parsed to find the
  reader's message. It is sized from the text before the parse allocates:
  - for a text the prescan admitted: the prescan's allowances (scratch, the float and
    error allowances, the array transient and the wrapper);
  - for the refused text: the same allowances from its length alone, as if the whole text
    were one escaped string or number.

  A refusal comes before any allocation proportional to the texts.
- **The verdict's parse lease** (`MemoryPool::acquire_result_parse`). One pool lease of
  the `Value` materialization M that the allocation-free prescan computes from the held
  bytes, admitted before the parse allocates anything.
- **Admission class.** The diagnostic and parse leases are adaptive: the pool, a fresh
  headroom verdict and normal pressure, re-sampled within the existing window, as result
  and custody leases are. The args lease is interactive, as every command's is.
- **The held result's lease.** It is the existing result lease R, admitted when the job
  publishes, unchanged in size and admission, and not taken by this command.
  - C1 only changes who holds it. The record and each native reader in flight share the
    bytes and their lease (`HeldResult`), so they're released together by the last
    holder, and the pool never counts held bytes as released.
  - R stays within the worker's capped grant, which is bounded by the per-invocation
    budget.
- **No new per-invocation budget.** No lease above 12 GiB, and no memory outside the pool.
  A model that would need either is refused by name.
- **The summary's leases** (slice 2), under the same rules: the args lease as the verdict's
  (the copy, the args' admission bound, the model text's priced parse); the diagnostic leases
  of `native_verdict::prescan_in_order`; one parse lease of M that covers the numerical health's
  parse of the held text and then S's own parse, one after the other; and a response lease
  (`acquire_result_parse`, its own cap) of the response's exact length, admitted after the
  counting pass and before the response is allocated. The response lease covers construction
  and ends when the command returns; it does not cover the returned buffer during delivery.
  Each is capped as above.
- **The detail's leases** (slice 3), under the same rules:
  - the args lease as the verdict's;
  - building the index: the diagnostic leases of `prescan_in_order`; the parse lease of M while the
    route is read; then the scan lease (the prescan's allowances) over the borrowing passes, with
    the index lease, 16 bytes per row and the index's own allocation, admitted before the index
    is allocated.
  - **The index lease persists with the index** and is released with the held bytes by the last
    holder. It stays ingress-purpose, so interactive admissions never credit it as resident.
  - each query's working lease, before it allocates: the largest row's priced parse for a scan
    that reads rows; each requested row's plus M for the semantics; the two members' prescan
    bounds for the recovery, with the receipt's again for the recorded values a page copies; twice
    a lookup's requested key bytes; and a fixed 4 MiB allowance for the header and the selection,
    which the 1,000-row and 1,000-key bounds limit;
  - a response lease of the response's exact length before it is allocated, ending when the
    command returns.
- **The analysis-run builder's leases** (slice 5), under the same rules. Nothing is cached; each
  call reads the held bytes with its arguments, so no result crosses a job, manifest, rule status
  or exclusion change. The leases:
  - the args lease, grown by the manifest text's priced parse;
  - the parse lease of M, held while the tree is read;
  - a working lease: a fixed 4 MiB allowance plus the canonical writer's key-order scratch along
    the deepest path of the source and manifest;
  - the fingerprint/index lease, 16 bytes per row (8-byte hash plus 8-byte row index), exact;
  - the response lease of the header's exact length;
  - for the bytes route, the record lease at its exact length, held by the caller with the bytes.

  Unpriced: the readers' working memory, the writers' per-number and per-escape scratch, the load
  basis, statuses and rule-pack diagnostics (scaling with the model and arguments, not enforced within the fixed 4 MiB allowance), and
  the delivery copies. Slice 7 must include these model-sized lists when evaluating residuals.
- **The persistence commands' leases** (slice 5's persistence), under the same rules; every lease
  is admitted before what it covers is allocated and capped at the per-invocation budget and the
  12 GiB default (the store's own reads lease through `NativeLeaseMemory`, one adaptive lease per
  reservation). Held together:
  - the raw body's copy and the args lease (the decoded arguments, the model, the re-sent intake
    text and the manifest's priced parse);
  - the intake's parse (the intake binding), admitted at its priced parse before it is parsed and
    ended once compared, before the held result is parsed;
  - the builder's leases above (M, working, fingerprints), with the canonical manifest and model
    payload texts the store's checks build, and SQLite's working set for the history tables;
  - the JS-text writer's scratch (one object's member list per nesting level along the heaviest
    path, with growth, and the largest decoded string), admitted before counting and held through
    writing; a refused output still admits this counting scratch, but not the output texts;
  - each output text at its exact length: Record Run's result text and recorded revision; Save's
    result text and transient record (only when the result is saved), the small slots, the
    envelope evidence and the search text;
  - SQLite's statement copies (it copies every bound text) and its records, and for a save the
    search row it replaces (FTS5 reads it), held while the statements run.

  The parse M, the record built on it and the builder's leases end together before the
  statements run; the result and record texts stay leased until the commit. The response is
  leased at its exact length until it is handed to Tauri; Record Run's read-back of the stored
  record (`find_run_with`) is metered by its own reservations. Unpriced, as for the submitted
  paths: SQLite's page cache and journal, FTS5's in-memory index of the search text, and Tauri's
  delivery copies of the small reply.
- **Not priced:** the readers' working memory and S's working set (§7, item 3), and the copies
  Tauri makes of the body before the command runs, plus the returned response buffer and
  Tauri's copies during delivery after return, including concurrent replies (§7, item 3).
