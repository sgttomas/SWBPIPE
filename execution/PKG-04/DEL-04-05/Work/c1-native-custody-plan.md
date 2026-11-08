# C1 native custody: implementation plan

**Serves:** RT-U4 (exit criterion 9) and the usability of ordinary models. **Design:** phase C1
of `result-volume-design.md` (§2, §3.1, §3.2, §4, §4.1; on `claude/rt-perf-c-design`),
approved by SWBPIPE Agent 0 on 2026-10-31 on three conditions:

- C1 changes no published byte and no engine code.
- The app-process peak and the usable size are measured before the PR leaves draft. *(Superseded
  for integration by Agent 0 on 2026-11-05; see slice 7, "Integration decision".)*
- C1 keeps today's v0.3 persistence, with exact-length refusals by name.

**Basis:** main at `e1a483b6c`. The integration decision and remaining measurement limits
are in slice 7.

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

### Slice 1: native verdict

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

### Slice 2: S projection

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
- the diagnostics presentation facts of the same held bytes (`diagnostics_presentation`; slice 4,
  "Native diagnostics presentation");
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

### Slice 3: paged detail and batched semantics

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

**Remaining for this slice:** the C2-copy interim for
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

#### Native diagnostics presentation (RT-A3 §4 on a native handle, 2026-11-05)

#138 classifies a result's diagnostics for the Diagnostics list and explanation
(`suppliedDiagnostics.ts` `presentedResultDiagnostics`; DEL-05-02 Work `combination-certificate.md`
§2). Its whole-value helpers read `results` and `retained_precision`, which a native handle guards.
The native handle is classified by the same code with three facts Rust reads of the held bytes
(`diagnostics_presentation.rs`) and S carries as `diagnostics_presentation`:

- **The facts.** Each is its whole-value helper on the same bytes:
  - `combination_identities`: `result_export::document_combination_identities`, the population
    `documentCombinationIdentities` documents as the same (the same ambiguous-identity exclusions);
  - `roster_checked`: `rosterCheckedByReader`: false on a retained successor; else the current reader
    of the source's route (`reader_verdict::route`) accepts the source alone
    (`reader_verdict::validate(reader, source, None, None)` on S's parse, current tables, never a
    recorded table). It is read only for a non-empty population. The fresh registration's
    model-bound validations are not reused: with a model, the readers read the certificate's roster
    with the model;
  - `certificate_attested`: `certificateAttestedByRetainedReader`. It is false for every native
    snapshot. No native custody holds a registered retained reader's acceptance of its bytes: a
    fresh solve never registers a retained successor natively, and a restored run never registers.
- **One snapshot.** The facts, S's diagnostics and the validation come from the one share S's
  command holds (bytes and recorded table, immutable). Their cache (`HeldResult::presentation`: a
  digest and a flag) is a field of that share, so no other job, restore or new solve of the same
  bytes reads it. Nothing is taken from a request, a stored claim, route metadata or a message.
- **Binding and fail-closed rule.** `binding` states the held bytes' length and SHA-256, the route,
  its current table and the recorded table the handle holds. The webview
  (`nativeResult.ts` `nativeDiagnosticsPresentation`) decodes the member strictly (exact keys,
  version 1, sorted non-empty ids, no roster flag without a population or together with the
  certificate flag). It then requires the length, route, table and recorded table to be the handle's
  own custody and the custody to be live. Otherwise there is no fact: every message is quoted as
  supplied text, through the same `suppliedDiagnosticMessage`, and nothing is attested. The
  whole-value path is unchanged; the native branch reads no guarded member and runs no reader on
  the webview handle. Any later classifier rule (for example #122's held-status quoting) applies to
  both branches, because both use the same map.
- **The report.** It reads the C2 full copy (`C2ResultGate`, `loadC2Full`), never the handle.
  `diagnosticsForSections` refuses a native handle by type (`NotNativeResult`), and its guarded rows
  throw at run time.
- **Memory.** S's command is not a ledger command. The population, the source-only validation's
  working memory and the streamed digest are S's working set and reader working memory (U3,
  unpriced). The header text is inside S's response, which is admitted at its exact length. The
  validation adds one source-only reader pass per held snapshot with combinations to S's first read.
- **Evidence.** `diagnostics_presentation_tests.rs` checks each fact against its definition over the
  certificate corpus, its claims and forgeries, the retained successors and a result without
  combinations. It also checks that the cache is the share's own and that S carries its share's
  facts: fresh, restored under the tenth registry variant, and disposed.
  `resultSummaryParity.test.ts` checks the facts against the whole-value helpers over the S corpus.
  `DiagnosticsPanel.native.supplied.test.tsx` checks the native list, explanation and interpretation
  against the whole value's on recorded Rust replies. It also checks the fail-closed and no-read
  cases and the report's refusal.

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
  an argument's (`text_parse_bytes`) and admitted first as an interactive lease in the command's
  running total (§8, "One running total per command"; a refusal is the pool's typed one), not JSON; the parsed model
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
  unchanged). Result/record values and their table and explicit index records are measured
  from the texts' counted lengths: the table row's record (SQLite's record format, `record_size`) and each measured index key's record
  (bounded with an eight-byte rowid). Until store schema v14 a project's search text and its search
  row's content record were measured too (Refusal changes, store schema v14).
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
- **Lifetimes and Record Run's readback (RT-U4, 2026-11-04; Agent 0's P1/P2 decision).** Stored
  bytes, canonical hashes, storage and schema versions, historical selection and the one
  transaction committed after the response are unchanged, and no new value is stored. Refusal texts
  and their order are unchanged **except** for the stated exceptions below ("Refusal changes"). P1
  ends each row-scale allocation at its last use:
  - Record Run writes the recorded revision while the parse M is alive, ends M and the builder's
    leases, then writes the result text from the held text. The two texts, their leases and the
    lease of SQLite's statement copies end as soon as the run row is inserted, before its labels
    and the readback.
  - Save, Create and the restored re-save upsert the project row with SQLite's copies leased for
    the statement, then end the result and record texts, in the same transaction. Until store
    schema v14 they then built the search text from its slots, ended the record text, and deleted
    and inserted the search row under its own statement lease, the replaced search row measured
    before the upsert.
  - Every readback of a recorded run (`summary_from_row`: Record Run's under P1, lists, reads,
    reopen) frees the loaded record once it is parsed. The record hash
    (`run_record_hash_within`) is streamed from the parse with no projection clone, serialized text,
    checked parse or canonical text. A projection the checked path could refuse or read
    differently (a number outside `openpipestress_jcs_ijson_v1`, nesting 127 deep, the `RawValue`
    token key) still takes the checked path itself, so its hash and refusals are that path's;
    `record_projection`'s omission rule and duplicate-checksum refusal come first, as before.
  - The recorded-run reopen takes the record text the verification loaded and the result parse
    the checks read (`NativeRunLoad::record_text`, `result_value`) rather than loading or parsing
    either again. The release of the current result before a reopen is not part of this change.

  P2 (a separate commit, the native Record Run only): before anything is written, the readback's
  checks are applied to the staged record text with the columns about to be written. These are the
  prescan, then one streamed pass that builds no record tree, giving the projection hash and the
  members the checks read exactly as the parse gives them, then the identity checks and the status,
  with the readback's refusal texts. After the INSERT, in the same transaction, the stored record
  must be exactly the staged text: its storage class, its length and its SHA-256, streamed through
  SQLite's blob reader (rusqlite `blob`, no new package). The identity, time, hash and label
  columns are then read and checked as before against the staged record's members. The text's
  digest is ephemeral and is not the record's `analysis_run_record` hash. A staged text the pass
  cannot stream as the parse reads it gets the full readback: another key order, a repeated key, a
  number outside the profile, nesting 127 deep, the `RawValue` key, another shape or a parse error.
  Stored bytes that are not the staged text get the full readback's refusal, or
  `ANALYSIS-RUN-INTEGRITY` (the stored record is not the record written) where the full readback
  would accept them. Either way every row rolls back. Every other reader keeps full validation.
  A staged record failing its checks is now refused before the INSERT with the text the readback
  gave after it.

  **Refusal changes** (each still rolls back every row and releases every lease; reconciled with
  Agent 0's decision, which required the byte-fidelity check and named its bounded exceptions):
  - *New text, P2 only:* stored bytes the full readback would accept in full (its record, column,
    status and label checks all pass) but that are not the staged bytes, such as added
    whitespace, are refused `ANALYSIS-RUN-INTEGRITY: … the stored record is not the record
    written`. The full readback runs to its end first, so any refusal it gives (a malformed label
    list, a column mismatch) stands with its own text (review of #133, finding 1;
    `p2_changed_bytes_with_a_malformed_label_list_keep_the_old_label_refusal`).
  - *Earlier staged refusal, P2 only:* a staged record failing the readback's checks (which only
    a broken builder produces) is refused before the run is written, so before every later
    write-stage failure: the state insertion, the between-rows hooks, the label insertion and
    SQLite's row-size backstop. Its text is the readback's.
  - *Memory refusals, P1:* each statement's copies are now leased per statement and texts end
    earlier, so under memory pressure a different reservation may be the one refused, and at a
    different point. In particular a save's search-row reservation is now admitted after the
    project row is upserted; its refusal rolls the transaction back, so nothing is written. The
    refusal is `STORE-READ-MEMORY-UNAVAILABLE` naming that reservation. (Store schema v14 removed
    that reservation; below.)
  - *Memory refusals, one running total (RT-U4, 2026-11-05; §8, "One running total per command"):*
    each C1 command's live leases are now counted together against its budget, so a command can be
    refused where each lease alone fit. A named reservation over the budget is refused
    `STORE-READ-MEMORY-UNAVAILABLE: <what> needs <total> bytes in all, above the per-invocation
    budget of <budget> bytes; nothing more was read or written.` (the reopen's own reservations end
    `nothing more was read.`), where it was the pool's per-lease `ingress_exceeds_budget` inside the
    `which refused them (…)` form; a typed lease's refusal stays the pool's `ingress_exceeds_budget`,
    its `required_bytes` now the command's total. A pool refusal keeps its text. Inside a
    transaction every such refusal still rolls back every row.
  - *Store schema v14 (RT-U4, 2026-11-05; Agent 0's decision (A), the unused FTS sidecar
    retired):* `local_project_fts` had no product reader. Migration v14 drops it with its FTS5
    shadow tables (`DROP TABLE IF EXISTS`; no row-by-row delete, no VACUUM), and no save writes
    a search row. Canonical columns, hashes, other storage versions and historical selection are
    unchanged. The changes:
    - Save, Create and the restored re-save no longer reserve "the project's search text" or
      "SQLite's copies of the project's search row", so neither `STORE-READ-MEMORY-UNAVAILABLE`
      refusal naming them occurs. After the project row's statement the save's only reservation
      is its response's, admitted once the write's leases have ended, so the command's running
      total no longer rises after its writes. A budget refusal after the writes cannot occur;
      the pool's refusal of the response still can, and it rolls back as before
      (`record_run_and_save_refused_after_their_writes_roll_back`).
    - The save's length check no longer measures "search text" or "search row" against
      `SQLITE_LIMIT_LENGTH`. The project row's record is always longer than either, because it
      holds every slot the search text holds, plus the result slot, both hashes, the id, the
      name and a longer header. So the same saves fall back to the model-only save or are
      refused `PROJECT-SAVE-TOO-LARGE`. A save whose search text was the first measured length
      over the limit, with every slot alone within it, now names `project row` and its bytes
      (`PROJECT-SAVE-RESULT-TOO-LARGE`'s `refused`, or `PROJECT-SAVE-TOO-LARGE`'s
      `project_row_bytes=` and text) where it named `search text`.
    - Opening the store: each migration step and its `user_version` now commit in one write
      transaction (BEGIN IMMEDIATE … COMMIT). A failing v1–v13 step still refuses the open, and
      now its partial writes roll back with it. A failing v14 step no longer refuses the open:
      it is rolled back whole, and the store stays at v13 with the sidecar intact. The open goes
      on with `pending_migration_count` 1 and `migration_status`
      `migration_pending_store_schema_v13_of_v14: store-v14-retire-project-fts-sidecar failed and
      was rolled back (<error>); …`, and the next open retries the step. Meanwhile saves do not
      touch the sidecar. The read-only stored-document export (`open_store_read_only`) refuses
      such a store `STORE-NOT-CURRENT` until a later open migrates it, as it refuses any store
      older than the build's target. A v13 build refuses a v14 store `STORE-SCHEMA-NEWER` before
      writing (`a_v13_build_refuses_a_v14_store_by_name_and_writes_nothing`).
    - After a successful v14 migration, the project summary in every create, save, open and
      re-save reply reports `store_schema_version` and `store_schema_target_version` 14, the v14
      migration status, and `fts_indexed: false`. While v14 is deferred, it reports version 13,
      target 14 and the pending status (above).
    Evidence: `analysis_runs_tests.rs`
    (`v14_retires_the_fts_sidecar_of_a_v13_store_and_keeps_every_canonical_byte`,
    `a_failed_v14_step_rolls_back_whole_is_reported_pending_and_retried_on_the_next_open`, the
    v13-build refusal), the `lib.rs` migration tests (fresh store, required-step rollback), and
    the parity corpus. That corpus removes the 42 search-row keys and re-derives the 28 project
    replies (`REDERIVED_AT_V14`), each first checked to differ from its pinned digest only in the
    v14 summary members. No stored column's digest moved.
  Evidence: `p1_stored_bytes_hashes_and_replies_are_the_pre_change_implementations` (every stored
  column and reply of Record Run, Create/Save, the recorded-run read, the reopen and the re-save on
  every persistence case, against digests generated at `c8865f384`), the `p1_*` and `p2_*` tests in
  `analysis_runs_tests.rs` and `native_persistence_tests.rs` (hash and stream parity with the
  checked path and the parse; tampering with each identity column and the record bytes after the
  INSERT, missing labels, failures between rows and a refused finish, each rolling back every row;
  allocation by phase from the per-thread counting allocator, `stored_read::phase_probe`).
- **Evidence** (`native_persistence_tests.rs`, `result_js_text_tests.rs`, the TypeScript oracle
  `persistence_forms.json` from `persistenceFormsCases.ts`): on every preview case with a record
  (preview-physics, pressure, pressure-source and retained routes; aggregates; rule-pack
  exclusions) the native run, labels, snapshot and project rows (and, before store schema v14, the
  search row) equal the submitted
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
- **On main at `3983f51cf` (S2, RT-A3 2a/2b, hangers, MBF export, modal I1, `Index::any`;
  merged at `9df2d8abb`).** RT-A3 2b rebound the pressure-2 table's current digest
  (`d46e1963…` to `a2db16a5…`; rows, signatures and numbers unchanged) in its pin consumers.
  C1's native mirror of `RECORDED_RUN_TABLE_BASES` (`analysis_run_v03_validate.rs`
  `recorded_run_semantic_binding`) is one such consumer, written on this branch, so its
  `current` column takes the same digest as `analysisRunCompatibility.ts` and
  `compatibility.py`; without it a 0.7- or 0.8-era recorded run would bind differently natively
  than in the webview. The C1 corpora were regenerated from their generators, each after the
  one it reads: builder corpus, persistence forms, historical reopen cases (expected findings
  unchanged; only derived hashes), summary outcomes (the regenerated pressure sources'
  limitation text only), native replay requests (rebuilt from empty: 426 requests against
  the previous 451. Of the 100 removed keys, 74 are re-keyed in the same command classes, matched
  by 74 new keys, because pre-2b pressure hashes moved. The other 26 are operation-absent,
  model-absent `read_preview_mechanics_job_verdict` requests, 22 of them on file-backed sources,
  that the current webview tests no longer issue since `route_header` replaced the first bare
  dispatch; six remain unchanged and one is new, seven total. The native command's bare verdict stays covered
  on all 52 file-backed corpus inputs by `native_verdict_tests.rs`
  (`native_verdicts_are_the_committed_reader_outcomes_on_the_file_backed_corpus`, and the
  route-header test's command-against-`reader_dispatch_json` equality at line 740)) and outcomes, persistence replies, reopen requests (rebuilt from
  empty, same seven commands) and replies. The reach-scan pins are the union of both sides
  (main's `open_pipe_stress_caepipe_mbf` shell dependency, `hanger_sizing` runner dependency and
  `size_hangers_cli` target; C1's handlers); `app_invoke_handler`'s digest is unchanged because
  main added `mod mbf_export` but no handler. No CI routing digest was stale. The new families
  do not reach C1 custody: the MBF export and hanger view are not on the session's result path,
  modal I1 is a separate crate, and S2/RT-A3 publish through the same pressure-2 identity the
  native verdict reads from the core contract. `Index::any`'s map is inside the compulsory
  readers C1 calls; no C1 gate changed.
- **On main at `5e12ca44e` (RT-A3 #99's recorded pressure-2 bases, dynamics I2a/I2b; merged
  at `4cc0e117d`).** The independent review of the `3983f51cf` integration blocked on a
  persistence gap. RT-A3 2a (#73) and 2b (#80) each moved pressure-2's table digest
  (`d46e1963…` to `032e861f…` to `a2db16a5…`) without keeping the older digest as a
  recorded basis, so runs recorded under the first two were refused with
  `ANALYSIS_SEMANTIC_CONTRACT_MISMATCH`. #99 adds both as recorded bases of the current table
  in `analysisRunCompatibility.ts` and `compatibility.py`. It also adds their verbatim
  fixtures, `fixtures/persistence/recorded_model_0_9_0_pre_2a` and `…_pre_2b`, each holding a
  record, its source and its request model.
  - **Mirror.** The native `RECORDED_RUN_TABLE_BASES` (`analysis_run_v03_validate.rs`)
    gains the same two rows with the same id and current digest. It is now module-level,
    and `recorded_run_semantic_binding` reads it. Its doc comment is the TypeScript
    twin's. `the_recorded_table_bases_are_the_typescript_and_python_tables`
    (`stored_custody_tests.rs`) parses both twins' source text and requires all three
    tables to be equal, row for row and in order (four rows). No such test existed for the
    0.7/0.8 rows.
  - **Regression on the verbatim fixtures.** Before this step, the 0.7.0/0.8.0 recorded-basis
    fixtures had no native path. The native reopen saw only the synthetic
    `saved-variant:pressure:recorded-0.8-table-basis` case.
    `historicalReopenCases.ts` now adds main's four verbatim recorded-basis fixtures
    (`RECORDED_BASIS_FIXTURES`: 0.7.0, 0.8.0, pre-2a, pre-2b) to
    `historical_reopen_cases.json`:
    - each as a recorded revision, built as `recordedRunModel080.test.ts` and
      `recordedRunModel090Disclosure.test.ts` build it;
    - the two of a current model document also as a saved project's run. A project of a
      retired model document is refused for editing before its run is read.

    A case names its fixture directory instead of copying it. A recorded revision is the
    verbatim record plus `record_patches`, which the generator checks reproduce it. The
    native leg reads the same files, and the fixtures are never rewritten. Exactly:
    - **All six new cases** reach native findings validation in process
      (`the_native_findings_are_the_typescript_contexts`, 195 cases).
    - **Only the two saved cases** (pre-2a, pre-2b) enter the store, reopen and re-save loop
      of `historical_cases_are_resaved_with_their_findings_unchanged`.
    - **Through SQLite**, `recorded_basis_fixtures_reopen_natively_with_the_webviews_findings`
      stores all four fixtures' verbatim record and source as a project's run and opens each
      with the native open. The two 0.9.0 runs are restored with the webview's findings.
      The 0.7.0/0.8.0 projects are refused for editing (retired model documents), exactly
      as the legacy open refuses them, so their runs are not read through the store.
    - **The four recorded revisions** are checked in process only (`recorded_run_findings`
      and `recorded_run_validation`, which the native read calls). None is recorded into a
      store and loaded through SQLite as a recorded revision.

    Expected findings: each recorded revision has none, and each saved run has
    `HISTORICAL_INPUT_MANIFEST_MISSING` (the saved project carries no input manifest).
    Neither form has a contract mismatch.
  - **Discrimination.** With the two native rows removed (a temporary edit, reverted), all
    four tests fail. The two #99 fixtures gain `ANALYSIS_SEMANTIC_CONTRACT_MISMATCH` in the
    store's native open, the in-process findings (4 of 195 cases differ) and the re-save,
    and the table test finds 2 rows against 4.
  - **Corpora.** Each was regenerated from its generator, after the one it reads. Only
    `historical_reopen_cases.json` changed: 6 cases were added, and the 189 existing cases
    are unchanged in content and order. The other corpora are byte-identical:
    - builder corpus, persistence forms and summary outcomes;
    - the native replay requests, rebuilt from empty (426 requests) with their outcomes;
    - the persistence replies;
    - the reopen requests (rebuilt from empty, 7 requests) and replies.

    No fixture, registration, pricing tool or TypeScript/Python table was edited. The
    operation-applier and self-weight Wasm engines were rebuilt (stale against the merged
    source). No CI routing digest was stale.
- **On main at `bbf384dec` (RT-A4 #98: absolute sums, SRSS, nested combinations, §8.46 and the
  §10.4 historical read; merged at `731fb44e8`), then at `aa794eb68` (#102's region binding and
  `Index::any` determinism, §8.47, and #97's modal I2b2; merged at `480201bee`).** #98 named C1's
  mirror of its historical read as an explicit dependency.
  - **Conflicts (seven, resolved from both sides).**
    - `HistoryPanel.tsx`: the run check shows #98's historical reader findings and keeps C1's
      restored-record diagnostics.
    - `HistoricalRunContext.tsx`: #98's scoped reopen findings with C1's whole-value return type.
    - `ResultsPanel.tsx`: C1's exported standalone health, taking #98's table.
    - `knownSemanticLimitations.ts`: one signature, `(source, recordedTableRead, { hasIntensified })`.
    - `resultStanding.ts`: #98's `recordedResultStanding` beside C1's native-aware standing.
    - `user_release_reach_scan.rs`: C1's `app_invoke_handler` pin (main left the macro unchanged)
      with #98's re-pinned headless macros.
    - `hosted-ci-routing.json`: both sides' scanned fixtures. Every runtime-read digest and scan
      list recomputed from each merged tree matches.
  - **The historical read, natively.**
    - `analysis_run_v03_validate.rs` ports #98's scopes (`reopenFindings`: findings,
      `check_findings`, `recorded_table_read`, `historical_reader_findings`). Under a record
      binding a `recorded_tables::VARIANTS` member, the reopen reads through
      `rv::dispatch_recorded`, `rv::retained_recorded` and the record validator's recorded read
      (`validate_v03(…, recorded_read)`: recorded dispatch route, `rv::validate_recorded`, the
      variant's table for row interpretation, the variant binding as contract).
      `RECORDED_RUN_TABLE_BASES` takes #98's six rows (current `10455bf3…`).
    - The restored handle holds that variant (`RestoredSource::recorded_table`). The host
      establishes it from the stored record at restoration, never from a request.
    - S's family counts, the detail index and row semantics on that handle read under it
      (`result_summary::route_table`, `rv::row_semantics_each_recorded`). A refused binding
      refuses every row, never falling back.
    - The reopen reply carries the recorded dispatch (no model) for display classification.
    - Every C1 native reader entry runs inside `recorded_tables::entry_scope`.
    - The webview's `displayRoute`, `knownSemanticNotices`, `resultRowLabel` and
      `NativeResultsPanel` classify a native handle under its held table only (any other table
      is refused). The re-save reply and the session's post-re-save context carry the scopes.
    - Current admission is unchanged. The bare verdict and every fresh path read the current
      tables; a restored handle stays unbound to any standing and is never Current or Solve.
  - **#98's current display rules, natively.** S's support-load, navigator, overlay,
    colour-quantity, stress-label and extremes tables port `derivedRowNote`, `combinationNotes`
    and `isOneSolvedState` (absolute sum or SRSS unsigned, solved state plus derived bounds, a
    range envelope by sign convention). The summary corpus gains invented RT-A4 rows.
  - **Regressions.**
    - Registry equality, native (`VARIANTS`) against TypeScript against Python
      (`the_recorded_pre_rt_a4_registry_is_the_typescript_and_python_registry`), and the bases
      test at six rows.
    - The reopen corpus (206 cases, every case's scopes compared): the five pre-RT-A4 PF
      variants, an unknown-digest control, main's recorded revision, and the `pre_ce` and
      `pre_rt_a4` recorded-basis fixtures.
    - The connecting path: `a_run_recorded_before_rt_a4_reopens_through_the_native_commands`
      (real store; native open, S, semantics and bare verdict; unchanged re-save byte-equal to
      the submitted path and to the stored texts; disposal), with
      `nativeReopenPreRtA4.session.test.tsx` rendering `HistoricalRunPanel` from those
      commands' replies.
  - **Fail-without.** Each check was a temporary edit, reverted.
    - Without the native recorded read, three tests fail and 6 of 206 cases differ.
    - A dropped base row fails the bases test and two reopen cases.
    - A diverging TypeScript registry digest fails the registry test.
    - A missing absolute-sum note gives six summaries that differ.
    - A context ignoring the scopes, or `displayRoute` ignoring the held table, fails the
      connecting TypeScript test.
  - **Corpora.** Each was regenerated from its generator. The native replay requests were
    rebuilt from empty: 426 requests, the same count per command, re-keyed because #98 changed
    fixture bytes. The reopen requests were rebuilt from empty (the same seven commands). On
    `480201bee` every corpus regenerates byte-identical.
  - **Resolved.** `recorded_tables.rs`'s module documentation said "the desktop's native
    custody makes no recorded read". With RT-A4's agreement, `43cea6384` corrects it (native
    recorded reads run synchronously inside `entry_scope` on their worker thread) and re-pins
    `RT-A4-HS`.
- **Open matter: overlapping writes to one store (raised for a product decision; no
  repair in this run's binary).** Each store write opens its own connection with a 5 s busy
  timeout (`lib.rs:864-872`) and writes in one `BEGIN IMMEDIATE` transaction (`lib.rs:1080`;
  native Save `native_persistence.rs:885`, native Record Run `native_persistence.rs:566`,
  restored-handle Save `stored_save.rs:326`). The store is in WAL mode (`lib.rs:896`).
  - **The tests.** The former
    `the_persistence_commands_run_off_the_async_executor_and_reply_with_a_json_body` sent about
    38 Record Runs at one store and could exhaust the real 5 s timeout under full-suite load.
    It is now two tests, with no product change and no longer timeout:
    - the executor test gives each writer its own store, so it measures only off-executor
      progress;
    - `overlapping_record_runs_on_one_store_commit_whole_or_are_refused_busy` states the
      store's contract. Every overlapping write either commits completely or is refused as
      `STORE-SQL-ERROR: SQLite DatabaseBusy (…)`, and no other outcome is accepted. Exactly
      the committed runs are stored (no lost success), each reads back whole, no run lacks
      its state, and `quick_check` is `ok`. A second phase forces a refusal: another
      connection holds the write lock past the timeout. That command is refused, never
      reported as success; every table is byte-for-byte as before; and the same request then
      commits.

    Agent 0's disposition: a named, atomic `DatabaseBusy` is a legitimate bounded contention
    outcome, and arbitrary concurrent writes are not required to all succeed.
  - **Ordinary workflows (read-only analysis, 2026-11-03).**
    - **Autosave** writes recovery files beside the store under a per-slot `flock`. It never
      opens the store (`autosave.rs:17-24`), so it cannot contend with store writes.
    - **Within one window, project writes are single-flight.** Save, Save As (and its
      follow-on save), Record Run, Save Model State, label edits and deletions all take the
      session's project-operation owner (`workspaceSession.ts:2361-2373`; Save at `:3279`;
      history operations through `begin()` at `useModelStateHistory.ts:296-310`, Record Run
      at `:571`). A second request is refused
      at once with a named busy message and is not queued (`useModelStateHistory.ts:298-301`).
      These workflows never overlap each other's store writes in one session.
    - **On refusal, edits and retry eligibility are preserved.** A failed Save shows
      `Save failed: …` (`workspaceSession.ts:3438`) and leaves the session model untouched.
      The landed base is set only on success (`:3427`), and the owner is released in
      `finally` (`:3441`), so the user can retry. A failed Record Run shows
      `Not recorded: …` (`useModelStateHistory.ts:603`) and returns `false`; the dialog
      stays open with its labels (`HistoryPanel.tsx:358`). A native refusal is a rejected
      invoke, so it reaches only these catch paths and is never reported as success.
    - **Overlap that remains possible.**
      - The library and rule-pack imports have their own per-panel in-flight flags
        (`LibraryManagerPanel.tsx:251`, `RulePackManagerPanel.tsx:106`), not the project
        owner. Their store writes (`save_local_library` through `upsert_library`,
        `lib.rs:4379`; `save_local_rule_pack`, `lib.rs:4110-4113`) can therefore run while a
        Save or Record Run holds the write lock. A Save of a large held result may hold that
        lock for longer than 5 s (not measured), and then the import is refused
        `DatabaseBusy`. The refusal is named and atomic, and
        the panel's catch shows it, and its `finally` clears the flag for a retry
        (`LibraryManagerPanel.tsx:545-548`). When the import holds the (short)
        lock, the Save waits for it.
      - Two app processes on one data directory can overlap too: there is no
        single-instance guard, and `tauri.conf.json` declares one window per process. A
        different project id then gets the same bounded busy outcome. The same project id
        is additionally guarded by `PROJECT-BASE-MOVED` (`lib.rs:1455-1481`).
    - **Conclusion.** No ordinary single-window workflow can lose a Save or Record Run to
      store contention or misreport a refusal as success. The exposure is bounded: an
      import overlapping a long Save, or a second process, can see a named, atomic,
      retryable refusal.
  - **Product decision (owner).** Whether overlapping writes to one store must all succeed
    (a bounded per-store queue with cancellation semantics) is a later, targeted repair. It
    is not part of the frozen run's binary.
- **Remaining:** the independent persistence, permissions and numerical review of this increment;
  the C2 interim for comparisons with the current run and reference-inspection retention of a fresh
  result; slice 7's measurement. (Saved-result native reopening: slice 6 status.)

### Slice 6: saved and historical results open into native custody

- Opening a project or stored run reads `mechanics_result_json` or `result_json` natively
  into a `HeldResult`, under the same lease rule, and returns a handle. Its verdict, S and
  pages then work as for a solve.
- `HistoricalRunContext` and the model-state comparison take the handle.
- After this slice the wasm reader path is used only by tests.

#### Slice 6 status (connected on `claude/rt-perf-c1-custody`, 2026-11-01)

**Connected.** `stored_custody.rs`, `analysis_run_v03_validate.rs` (reusing the slice-5 builder's readers and writers by explicit import),
`services/nativeReopen.ts`; `solve_jobs.rs` gains the restored origin.

- **Commands** (raw args bodies under `command_ingress`, JSON body replies, Tauri delivers them
  parsed; the decoders refuse text, bytes and other shapes, `NATIVE-REOPEN-MALFORMED`):
  - `open_local_project_in_native_custody {projectId?}`: `open_local_project`'s members (the same
    store open, model-document refusals and serde messages) without `mechanics_result` and
    `analysis_run`, plus `stored_run`: `none`; `restored` (handle, origin, stored source with the
    SHA-256 of the texts held, lengths, findings, record header); or `refused`
    (`STORED-RESULT-RESTORE-REFUSED`, the lengths, the pool's or retention's reason) with the model
    open for editing.
  - `read_analysis_run_in_native_custody {projectId, recordingId}`: `read_analysis_run`'s verified
    store read (every integrity refusal, one read transaction) without its 64 MiB limit; the reply is
    the summaries, the manifest's canonical text and `restored`.
  - `read_restored_result_record_all {jobId}`: the stored record's bytes for a C2 consumer, at or
    under 64 MiB only (`RESULT-DETAIL-WEBVIEW-LIMIT` above), as `read_preview_mechanics_job_rows_all`
    copies the result.
- **Origin** (`solve_jobs::ResultOrigin`). A restored record (`restored-stored-result-N`) holds the
  stored result and record as two `HeldResult`s with result leases; it has no digests, attestation,
  worker, timestamp or acting cancellation token. `custody_binding` answers only for a solve (no
  exact-route custody); the native Record Run and Save Project refuse it whatever the request names
  (`NATIVE-PERSISTENCE-CUSTODY`), and the fresh record preview refuses it
  (`ANALYSIS-RUN-PREVIEW-READ-FAILED`). The webview's `NativeCustody.origin` is `fresh_job` or a
  restored origin; a restored `NativeResult` has no capture or registration, S read with no model
  and no standing (as the historical `ResultsPanel` read the rows), and a standing binding no model
  text matches (`NATIVE_VERDICT_NOT_BOUND`). The session never makes it the current result.
- **Findings.** `buildHistoricalRunContext`/`buildRecordedRunContext` and the record validator they
  call (`validateAnalysisRunV03`, `verifyAnalysisRunRecord`, `validateRecordedBinding`, S1's recorded
  pressure-2 table bases for 0.7/0.8-era records) ported over the stored values, reusing the builder's
  readers, table interpretation, load-basis rules and canonical writer, in the TypeScript's order. The
  TypeScript stays the oracle: `historical_reopen_cases.json` (181 cases: every record of the builder
  corpus as a saved run, 30+ variants per route of rows, checksums, versions, references, shapes,
  unsafe integers, duplicate ids, the 0.8-era basis and an unknown basis; recorded runs and their
  variants) equals its generator (`historicalReopenParity.test.ts`), and the native findings, run name,
  envelope payload hash and model hash equal every case (`stored_custody_tests.rs`). The standing
  behind the standing findings is never sent.
- **The record header** is the stored record without `result_refs`, `diagnostics`, `contract_evidence`,
  `source_block_recovery` and `retained_precision`, with their counts, the carried members and the
  record's rule-pack reference diagnostics; the complete wrapper, escaped keys and matching
  diagnostics are measured before allocation, and above 4 MiB it is omitted with its length. It is a
  projection of the stored record, never a rebuilt one.
- **Webview.** The session opens through the native command on the desktop (`openStoredProject`):
  the historical panel's `ResultsPanel` routes the restored handle to `NativeResultsPanel` (S, pages,
  semantics, navigator, streamed CSV). The open adopts the native envelope payload hash for the
  integrity cell; a refused run is not hashed (`hash_recompute_unavailable`), and a save in place
  without a new result is refused by name, the stored run kept. A save of a project showing its
  reopened run is the native save continuity below (no whole copy at any size). A reference inspection retains the
  restored run by its handle, never a clone. A restored handle is disposed once nothing shows or
  retains it, and when the session ends; a superseded, failed or refused open disposes what it
  received. The History panel's run check reads natively, shows the findings and the header's
  rule-pack references, and releases the handle at once (it shows no rows). Run comparisons keep the
  bounded whole-value `read_analysis_run` (C2 interim, #51's refusal above 64 MiB unchanged).
- **#51 preserved on the legacy reads**: `open_local_project`, `read_analysis_run`, the comparison
  admission and the save guard keep the stored-length withholding (lowered-seam test); the native
  reopen does not withhold for length alone.
- **Evidence.** `stored_custody_tests.rs`: the oracle corpus; every persistence case saved natively
  (the native Save, then a session reset) reopens with the stored bytes and record held exactly, its
  findings those of the legacy open's values (no mismatch finding on a fresh save), S's row count and
  the pages' total reading the restored bytes, the fresh persistence routes and preview refused with
  nothing written, the take refused, the C2 copies exact, the open writing nothing (the store file's
  digest), the members as `open_local_project` returns them; the same for every native Record Run; the
  legacy reads withholding at a lowered limit while the native reopen holds; a pool refusal (open:
  model editable, run refused, nothing held or written; read: the pool's named refusal); a reset during
  the read refusing the registration with nothing held; retention full; a corrupt stored result
  failing the open with the legacy open's exact message; disposal during a reader's share releasing
  both leases only at the last drop, once. `nativeReopen.session.test.tsx` runs the session on the
  committed actual replies (`native_reopen_replies.json`: the session's own native Save and Record Run
  requests on a real store, a reset, the open, every request the historical panel sends on the handle,
  the recorded run's read, and a refused open), request for request: the restored origin, the
  findings, no current result, S and the first page shown, no legacy open, full copy or take, one
  disposal at the end; the recorded run's check. The actual open/read commands are held at
  admission with more futures than runtime workers while another async task completes; all
  leases end afterward (`reopen_admission_and_store_reads_run_off_the_async_executor`). `resultTooLarge.session.test.tsx` shows the refused
  open and the refused save in place. Tests written before slice 6 that mock the legacy open reach
  the native path through a unit bridge (`test/nativeReopenEmulation.ts`: findings from the TypeScript
  oracle, rows from the actual commands' recorded replies); it is no proof of the reopen.
- **Save continuity** (`stored_save.rs`; already-authorized C1 persistence, not a C2 consumer: the
  approved C2 exception names reports, comparisons, rules, derivative exports and handoffs, never
  re-saving an editable reopened project). Save or Create of a project showing its reopened saved run
  runs `save_restored_result_local_project {jobId, expectedSource, envelopeBasis, mode, projectId,
  projectName, model, editorIntents, proposal, selectedReviewTarget, confirmedRevision, expectedBase}`
  (raw args body; JSON body reply). No result, record, model hash or envelope hash is sent.
  - **Stored as the legacy session stored it.** Before, the session copied the run whole, `JSON.parse`d
    it and sent it back; `save_local_project` stored serde_json's reading of `JSON.stringify`. The
    command writes exactly that: the held texts parsed, read with JavaScript's numbers
    (`analysis_run_v03_build::js_numbers`: binary64 values, a negative zero `0`, the JavaScript
    number text), written by the builder's stored writer (serde_json's member order). The record is
    the stored record (its `created_at`, `model_state_ref`, recorded revision and claims as stored),
    never rebuilt. The held texts, their source rows, schema and model versions are not rewritten.
  - **Hashes the host holds.** The stored project's model hash evidence and its envelope evidence with
    the payload hash recomputed at reopen are held with the handle (`solve_jobs::RestoredBasis`), not
    taken from the request. The envelope hash is computed over what is stored (with JavaScript's
    numbers, as `computeProjectEnvelopeHash` hashed `JSON.stringify`); when it equals the payload hash of
    the basis the session names, that basis's stored evidence is written (the legacy unchanged
    Historical save). The session names its basis (the reopen's, or a committed re-save's from its
    reply); only a basis the host established for the handle is accepted.
  - **Binding** (`NATIVE-PERSISTENCE-CUSTODY`, nothing written): a restored saved project run still
    held (a solve, a restored recorded run, an unknown, disposed or reset handle are refused; no session
    path saves a recorded run into a project); the stored source the session names (kind, project,
    recording, both SHA-256) is the handle's; the held texts still hash to it (in place); the envelope
    basis is established. The fresh Record Run, Save Project, record preview and exact-route custody
    still refuse a restored handle; nothing grants it intake, publication, worker, attestation or
    Current facts. The destination is the existing save contract: model document checks, the
    replacement guard (expected base / confirmed revision), SQLite's exact lengths, the model-only save
    (`PROJECT-SAVE-RESULT-TOO-LARGE`, successful, no findings) and the withheld-run guard before it, the
    no-save rollback (`PROJECT-SAVE-TOO-LARGE`); `native_persistence::stage_save`/`finish_save` are
    shared with the fresh save, whose replies are byte-identical.
  - **Reply and session.** The fresh save's reply plus `restored`: the handle (still held), its stored
    source and the findings of the project as stored (those the native reopen of it gives), and the
    envelope payload hash of what was stored. The session keeps the same history object's handle and
    source (historical, never current), refreshes its findings, model hash and envelope basis from the
    reply as the legacy session rebuilt them from its save's reply, checks the stored envelope claim
    against the evidence computed over what was stored, and keeps the history unchanged after a
    model-only save or a refusal. The save disposes nothing.
  - **Evidence.** `stored_save_tests.rs`: the oracle is the unchanged submitted path given what the
    legacy session sent (`result_js_text`'s `JSON.stringify` writer, matched to the TypeScript's texts;
    `computed_project_envelope_hash`'s independent RFC 8785 writer). Every persistence case saved
    natively, reset, reopened and re-saved (unchanged with the stored evidence kept, edited with a
    computed envelope hash and `HISTORICAL_MODEL_HASH_MISMATCH`, Create with the confirmed revision,
    again from the reply's basis with identical rows) stores the submitted path's project row (and, before store schema v14, search row)
    byte for byte (but the save times), the same summary, model hash and envelope hash, and the reply's
    findings are the native reopen's of the stored project; stored texts in JavaScript's form (as Record
    Run writes a result: member order, negative zeros) and in a raw form (integral floats, `-0.0`,
    unsorted keys, escapes, non-ASCII, integers beyond binary64) are stored normalized as the legacy
    session stored them, never copied; the persistence forms' edge texts read the same through the
    stored writer; every saved case of the TypeScript reopen oracle (0.7/0.8-era and unknown bases,
    broken checksums and references, retired and unsupported record versions) re-saved unchanged with
    the TypeScript's findings still; SQLite's row boundary on the actual store (saved at the row's size;
    one byte lower, the result and record each fitting, the model-only save with its envelope hash and no
    findings; below the model-only row, nothing written), the withheld-run guard and its confirmed
    Replace; the old display limit lowered (the C2 copies refused, the re-save exact); each binding
    refusal with the store file unchanged, a request's own model or envelope hash ignored; the moved base,
    missing base and stale confirmation; the pool refusing a parse (named, nothing written, the handle
    held); a disposal and a reset while a bound save is pending (it completes from its share, the leases
    released once when the share drops); concurrent saves (one write lock; a moved base refused); the
    actual command off the async executor. The session runs on the actual reply
    (`native_reopen_replies.json`, `nativeReopen.session.test.tsx`): Save after the reopen and the
    panel's reads, the request exactly, no legacy open, save, whole copy or take, the history's handle
    and source kept with the reply's findings, verified integrity, no disposal until the session ends;
    a refused and a model-only re-save keep the history. Older tests reach the re-save through the unit
    bridge (`nativeReopenEmulation.ts`, the legacy save calls), no proof of it.
- **Permissions.** Four ordinary `app_invoke_handler` entries (no `mod`, no `$parameter ::`, no
  scanned name), re-pinned in `user_release_reach_scan.rs`; no capability or guard change.

**Limits and residuals (for slice 7 and review).**
- The open still loads the project's small columns (model, intents, proposal, target, hashes) as
  `open_local_project` does, unmetered; SQLite's page cache and journal (and its FTS state before
  store schema v14); the validator's
  working memory (one row's values, the duplicate-id fingerprints within the fixed allowance); and
  Tauri's delivery copies of the replies after return are unpriced. The held leases cover the texts'
  envelopes; SQLite's copies during the load are a separate lease ended after the load.
- The recorded read parses the result twice (the store's verified read, then the findings) under
  separate leases, one at a time; the store's read reservations stay held while the findings run
  (conservative).
- Run listing (`list_analysis_runs`) still verifies each run's record under its own reservation, and
  run comparisons retain the bounded whole-value read under the approved C2 exception (measurement
  and review inputs, not a new optimization task).
- The re-save parses each held text once (M each) and holds the stored writer's output texts with
  the parses until the statements; the validator's working memory for the reply's findings is the
  stated residual. Historical model and envelope hashes read every number as JavaScript's
  binary64 value before RFC 8785 writing, including exact-digit stored integers outside binary64's
  exact range. The stored bytes and checked-profile hash rules are unchanged. An unchanged re-save
  therefore preserves the stored evidence, including advisory mismatch findings. The TypeScript
  oracle includes raw wide integers in models and results; the re-save's legacy oracle derives its
  prior payload independently from the stored fields rather than the native findings.
  Recorded-run model and manifest hashes use the same JavaScript binary64 reading; the raw wide
  integer oracle cases currently cover saved-project paths, not recorded-run paths.
- A stored record already at the parser's maximum container depth can exceed that depth when the
  header wrapper is re-parsed. The project then opens model-editable with its run refused; a recorded
  read fails. Stored bytes remain unchanged. Current writers do not produce such deeply nested
  records; this compatibility edge is a source-review observation, not an executed regression.
- The unused bounded whole-record read (`read_restored_result_record_all`) releases its copy lease
  when the response is built; a future consumer must account for the webview copy's lifetime before
  adopting it. No production reopen or re-save path uses that read.
- Recorded-manifest member checks reuse the store's `manifest_members` after the store's own read has
  checked them; their TypeScript `TypeError` points are therefore unreachable here and not emulated.

**Remaining (slice 6).** Independent numerical, persistence and permissions recheck cleared both
blocking findings at `55fd8f4fc`; the exact-head hosted run remains required, then slice 7's
separately authorized measurement. Review and checks establish no memory fit or acceptance.

### Slice 7: measurement and the PR

**Integration decision (Agent 0, 2026-11-05).** This supersedes the earlier blanket hold of #59
for complete usability and memory-fit evidence: the draft-until-measured condition above, and
this slice's "leaves draft only after that window". #59 may integrate on its implementation,
reviews and checks, with residuals U1, U3 and U4 explicit.
- *Residuals.* U1 was SQLite/FTS5 search-row memory, with bounded evidence in #137 (store schema
  v13). Store schema v14 retires the unconsumed sidecar (Refusal changes above). That removes
  the search row and its FTS5 memory, so U1 is narrowed to SQLite's own memory for the
  canonical writes (the project row's upsert, the run and state inserts) and COMMIT. That memory
  is unpriced beyond the per-statement copies the ledger leases, and unmeasured at v14. U3 is
  working memory and the page cache.
  U4 is that there is no App-wide cap. DEL-04-07 `memory-budget-decision.md` records the
  owner's 2026-10-24 acceptance of the SQLite and command-working-memory residuals.
- *What the decision does not waive.* Implementation defects, connected-path evidence,
  independent persistence and permission scrutiny, required exact-head checks, source and
  binding identity, and the existing per-surface C2 limits all still apply.
- *What it does not establish.* It is not a resource-safety, large-model, usable-size,
  16 GiB or release qualification. The 500- and 2,000-node and publication performance
  criteria stay unestablished, and the projections below stay out of user-facing text.
- *Connected evidence.* One 82-node native pass at the `4427c3ec0` runtime
  (`c1-slice7-run-proposal.md`). P1–P3 are covered by their tests, not by a connected run.
- *History kept.* The paragraphs below keep their dated history as recorded.

**Integrated-main executable prepared; no run authorized (2026-11-03).** After the
route-header run, main `3983f51cf` was merged (`9df2d8abb`; reconciliation `4d0bb1858`,
slice 5's "On main at `3983f51cf`"). The maintained recipe bound clean `4d0bb1858` into a
separate `app-c1-main-4d0bb1858/` without launching it: App/worker SHA-256
`2edc74f73e55403de3f0deaf27610e869bf024387456f1557603cfcfba494fc2`, 48,855,840 B, frontend
`sha256:df95f9bfe9d1d858ac408e5a4a70c81d5e1999883c0ea1ec4c689f542e35b58a`.

**That candidate is superseded, not qualified.** Its compiled native table lacked #99's
recorded bases, so it was never run. Main `5e12ca44e` was then merged at `4cc0e117d`, and the
native mirror and verbatim-fixture regression followed (slice 5's "On main at `5e12ca44e`").
The maintained recipe bound clean code head `2470f0108` into a new
`app-c1-main-2470f0108/` without launching it:
- App/worker SHA-256
  `bc8e18fc15def4a42b59d7f7819ae1b90fa831d5c9edd607b97f34d045a0e134`, 48,854,176 B;
- frontend `sha256:41739f12d9511ee688b060aff66c37ac8b5b6043a374a237e27b81459d4c154f`;
- three Wasm engines, two of them rebuilt for the merged source.

Its runtime includes #95's `Index::any` map inside the compulsory readers. The five earlier
candidates (the superseded one kept as history) and the 30 closure-hashed packet files are
unchanged. The proposal gives the one-500 command with fresh `evidence/n0500-c1-main1`
under unchanged limits. It also gives the journal observations that would show whether
registration's `validate` now exits, or which later phase stays pending. It promises no
speed or fit; every gate stays compulsory.

**Approved metadata-only header implementation.** Agent0 selected the distinct
route_header contract for C1 native/frontend surfaces only. The first await now
requests metadata via unchanged public rv route/binding/error APIs; old full
dispatch/reader replies remain unchanged and compulsory later registration,
standing, summary, full-verdict/run-preview/session/report/store gates remain.
Strict new types/cache and final full-binding agreement bar metadata promotion.
The proposal records exact old/new behavior, negative/session/native regressions
and the CLEAR gpt-6-astra/medium numerical/permissions review at c21ec1048, with
exact-head CI green. Full source parse
remains and no 300 s/performance claim follows. Core/bridge/clone-projection and
pricing source/pins remain on hold; RT-A3's exact pin/dependency facts are retained.

**Separate executable prepared; no run authorized.** The maintained release
recipe through normal slot/memguard bound clean reviewed c21ec1048 into separate
`app-route-header/`, without launching it: App/worker SHA-256
`5212d86b839c8babcadb918835759804a95bfc53647585f211313f61fbaf0e53`,
47,960,208 B, frontend `sha256:54cd4e5121ea2862328bc878be6c3bbbe8ca995d90fddfda8bc29e412ffb59af`.
All three earlier bound copies and 25 closure-hashed packet files are unchanged.
The proposal binds Wasm/scripts/plan/model/Python and gives the exact future
one-500 command with fresh `evidence/n0500-route-header1`, unchanged limits and
current journal. Existing small-fixture connecting tests pass (29 TS, 4 Rust).
No executable prerequisite remains, but a new owner run decision and current
resource clearance do. No launch/sampler/profile/large replay occurred; full
registration/standing may still stall. No B1000/retry/2,000 scope follows.

**Reader-phase attempt executed/closed (2026-11-02).** The single newly approved
attempt on clean `cf8129cd1` failed its 300 s first-paint gate but fulfilled the
missing observation: durable first-header call 1 entered native `validate` after
source prescan/admission and its sole source parse exited. Later registration,
standing, summary, serialization/drop and P3 were not reached. The wrapper held
04:00:57–04:06:00 UTC (303 s from one-second logs), owned processes/locks closed,
no held exception/overflow; all packets preserved. No retry/B/third execution
scope is implied. The proposal returns an exact distinct route_header contract
and consumer graph for an implementation decision, plus a separate exact-parity
clone-projection proposal under the existing shared-core/RT-A3/D3 write hold.
At that proposal return neither was implemented; the route header was then
approved/implemented as recorded above. The clone remains held, and full
reader/registration/standing authority is unchanged.

**Authorized diagnostic preparation only.** The next candidate adds fixed,
bounded, durable checkpoints to the four existing session awaits and native
command prescan/admission/parse/validate/drop boundaries. It leaves shared core
readers/operation-applier and all proposed optimizations untouched. Diagnostic
smoke-only wrapper expansion is checked for existing Value-reader reply/error
parity; ordinary builds keep the original wrapper. Summary health remains an
explicitly combined private-core phase. The proposal names retention, incomplete
exit semantics, overhead, checks and the distinct app-reader-phases binding.
Focused diagnostic review cleared `d4a880680`; the proposal records its
nonblocking limits, including that event overflow/invalidity stops the whole
window and may leave only partial journal evidence. Exact-head CI remains
required. No further native attempt is authorized by preparation or review/CI.

**Bounded offline timeout diagnosis.** The observed `reader_verdict_start` mark
covers header dispatch, registration, standing and summary. Four small maintained
replay freezes reproduce the same mark at each await; the failed packet cannot
identify the active native parse/validate/drop phase. Source establishes repeated
full-result parsing/pressure validation and combination projection's full-tree
clone-before-delete, but no measured time/allocation attribution. The proposal
records the exact call chain, 11 passing connection tests and the small existing
combination-corpus check, the minimum missing boundary observation, and a
conditional route-only/projection repair plan. No core/product algorithm,
numerical/permission contract, timeout or resource policy was changed; shared
reader edits still require Agent0 coordination and appropriate scrutiny.

**Repaired A500 execution update (2026-11-02).** The newly approved single
attempt on clean `eb77d0067` passed fresh startup telemetry but failed the
300 s first-paint wait, last stage `reader_verdict_start`. The worker published
918,638,964 B after 73.378 s. No table/usability/persistence/retirement probes or
separate reader/parse allocation profile were reached. The wrapper acquired
01:58:29 and closed rc=1 at 02:03:32 UTC (303 s from one-second timestamps);
owned processes and lock custody closed, with no held exception. No retry or
B1000 is authorized. The proposal records exact partial physical observations
and failed packet/closure locations; these are not usable-size, reader-working
attribution, memory-fit, C1-complete or acceptance evidence. Both packets remain
preserved for the owner and diagnosis consumer. #59 remains draft/unmerged.

**A500 execution update (2026-11-01).** The owner-approved attempt at clean
`1e1453a29` failed/closed at initial host telemetry, with no solve, reader-profile
or usable-size evidence. The exclusive wrapper recorded 19:34:58–19:34:59 UTC
(one-second precision); the supervisor window was 547 ms. All owned processes
and locks closed; partial packet/private custody is preserved. That attempt is
consumed, there is no retry, and B1000 is blocked. The proposal records the
confirmed clock-origin initialization defect, offline repair and newly enforced
fresh host admission before App launch. The latter needs focused scrutiny and
exact-head CI before a new one-run 500-node proposal can return for approval;
no new execution authority is inferred. No C1 completion or memory fit follows.

The concrete offline preparation is `c1-slice7-run-proposal.md`, with the maintained
`apps/desktop/native-smoke/perf-plan-c1.json` input. It binds the clean a37d release
App to the reviewed 55fd runtime, names the two generated racks and conditional
intervals, and tracks P1's independently clear safeguards at `dee93a72`, P2's
offline connected interaction probes (38 source-mode tests), and P3's
feature-gated scalar reader/parse attribution. The instrumented App is bound at clean `284c71f48` in that proposal; P2/P3
connected scrutiny cleared `92983690e`; the proposal records parent follow-ups
to its nonblocking notes. Exact-head checks remain required. The a37d binary cannot run P3. No App
or measurement window has been launched or authorized by that preparation.

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
| Diagnostics list and explanation (`DiagnosticsPanel.tsx`, `resultInterpretation.ts`, `suppliedDiagnostics.ts`) | the combination population and the readers' acceptance (RT-A3 §4) | S's `diagnostics_presentation`, Rust's facts of the held bytes (slice 4, "Native diagnostics presentation") |
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
7. **The session reset under IPC redelivery** (RT-U4 owner source assessment, 2026-11-05, on
   main's C1 source; source reading only: no App, native or timing run).
   - **Background.** In Tauri 2.11.1, any failure of a command's `ipc://` fetch, or of reading
     its reply, switches the page to wry's `window.ipc.postMessage` for good and re-sends the
     same message (`scripts/ipc-protocol.js`). Nothing deduplicates the two deliveries. On
     that path a byte payload reaches Rust as a JSON body (`scripts/process-ipc-message-fn.js`,
     `src/ipc/protocol.rs`).
   - **The command.** `dispose_all_preview_mechanics_results` (`dispose_all_results`) carries
     no page identity. The page sends it once (`beginSessionResultCustodyReset`), and every
     solve start awaits it. `reset_epoch` refuses only a restoration that spans a reset
     (RESTORED-RESULT-SESSION-RESET), not work registered before a reset that arrives late.
   - **(1) Same page, the original delivery executes first** (e.g. it ran, but reading its
     reply failed). The repeat executes before the page's reset resolves, so before any start.
     It finds nothing of this page and is safe. At most, a restoration already in flight is
     refused by name, and it can be retried.
   - **(2) Same page, the fallback copy executes first and the original later.** After the
     fallback the page's byte payloads arrive as JSON. Every command that creates C1 work or
     custody therefore refuses by name before admission: the solve start
     (PREVIEW-SOLVE-RAW-BODY-REQUIRED), and model checks, Record Run, Save and the
     reopen-into-custody reads (INGRESS-RAW-BODY-REQUIRED). The late original finds no work of
     this page. The page needs a reload before C1 works again. This is the raw-transport
     availability limit DEL-04-07 already records (`memory-budget-decision.md`: a postMessage
     fallback would materialize a `Value` first, and `start_preview_mechanics_job` refuses a
     JSON body).
   - **(3) A reset delivered late across a navigation.** This case is **conditional and not
     reproduced**. A reset sent by a page that is gone could execute after a later page's own
     reset and its start or reopen. It requires that page's request to reach Rust after the
     later page's requests, and it has not been shown on WKWebView. It does not depend on
     duplication: one late delivery, repeated or not, has the same effect.
     - **Result integrity: fail-closed.** The current page's queued or running jobs are
       abandoned (the worker is killed, and the job ends cancelled without publishing). Its
       native-custody handles then refuse by name. Its CSV stream ends with its partial file
       removed. A restoration in flight is refused by epoch. No stale or substituted value is
       presented.
     - **Accounting: a possible under-count.** If the current page already holds webview
       copies (a delivered result, or a C2 full copy of at most 64 MiB), their native custody
       leases are released while the webview still holds the bytes. The pool then counts less
       than is resident until the page releases them.
   - **Status.** No behaviour change follows from this assessment. It is not a demonstrated
     defect, not an accepted risk, and not covered by any qualification. It does not extend
     the owner's 2026-10-24 acceptance in DEL-04-07 (`memory-budget-decision.md`). A keyed or
     ordered reset would need reachability evidence or a design need first.

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
- **The analysis-run builder's leases** (slice 5), under the same rules and within the command's
  running total (below; the persistence commands share them). Nothing is cached; each
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
- **One running total per command** (DEL-14-01 Design §3, repair r4: the budget bounds the
  command's live leases together, starting from its argument lease; RT-U4, 2026-11-05; no new
  budget or rule). The analysis-run preview, Record Run, Save or Create Project, the restored
  re-save, the project reopen and the recorded-run reopen each keep one ledger
  (`memory_pool::CommandLedger`, through `stored_read::NativeLeaseMemory`):
  - **Budget.** min(the budget the argument lease was atomically admitted under, 12 GiB), fixed for
    the command. Every later lease and growth of the command is admitted by the pool under that
    cap, with its class, pressure, fresh headroom, aggregate pool, residency credit and re-sampling
    unchanged; a budget changed while it runs applies to later commands, and real headroom or pool
    changes still refuse.
  - **Accounting.** The argument lease is seeded once and its drop removes its charge; its growth
    by the arguments' bound and each argument's priced parse is a delta. Each lease's delta is
    reserved atomically in the ledger before the pool admission it precedes, as a provisional
    charge that counts until admitted and is undone on a pool refusal or unwinding; growth,
    shrinking and release are exact (nothing wraps or saturates), concurrent admissions of one
    command cannot oversubscribe it, and a lease may drop on any thread.
  - **Refusal.** Over the budget, by name before the allocation it guards (slice 5, "Refusal
    changes"); every lease ends on every path, unwinding included.
  - **Not counted.** The leases the command did not admit: the held fresh result H and an existing
    restored handle's texts (counted in the pool and its headroom as before), shared through
    `Arc`, never charged twice.
  - **The reopen's handoff.** The leases backing a reopened run's held texts are handed over whole
    (never released and admitted again) and leave the command's total only when the registry
    establishes the handle (`HeldResult::restored_in_custody`, `restore_with_basis`); every other
    lease stays counted until it drops. A handle whose reply is then refused, fails to serialize or
    unwinds is disposed, so none is left that no webview knows; a reset in between already
    disposed it. P1(e), releasing H before a reopen, is not part of this.
  - **Evidence.** `native_ledger_tests.rs` (each command traced at every admission: every pool
    admission of the command's thread is a ledger delta; the actual crossing step refused one byte
    lower, by name, nothing written; budget identity; the handoff; rollback after the writes;
    unwinding; no stranded handle) and `memory_pool.rs` `ledger_tests` (seed, exact growth and
    shrink, refused deltas undone, pending admissions counted, racing admissions of one ledger,
    cross-thread drops, the handoff, underflow).
  - **Not claimed.** This bounds the leases the commands take; it is no fit claim and no App-wide
    cap. U1 (SQLite's memory for the canonical upserts and COMMIT, unpriced; before store schema
    v14 also FTS5's tokenizer hash and the second old-content copy), U3 (builder,
    reader and validator working memory; SQLite's page cache) and U4 (no App-wide cap) remain.
- **The persistence commands' leases** (slice 5's persistence), under the same rules; every lease
  is admitted before what it covers is allocated, within the command's running total above (the
  store's own reads lease through the same `NativeLeaseMemory`, one adaptive lease per
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
    result text and transient record (only when the result is saved), the small slots and the
    envelope evidence (and, before store schema v14, the search text);
  - SQLite's statement copies (it copies every bound text) and its records, each held while its
    statement runs (before store schema v14, also the search row a save replaced, which FTS5
    read).

  The parse M, the record built on it and the builder's leases end together before the
  statements run; Record Run's result text is written after them. Each text ends with its last use
  (RT-U4, Persistence above): Record Run's two texts and the statement copies at the run row's
  INSERT; a save's result and record texts after the project row (before store schema v14, its
  record text once the search text was built). The response is leased at its exact length until it is handed to Tauri. Record Run's
  read-back loads no record: the staged checks before the INSERT and the stored bytes' digest after
  it allocate nothing of the record's size, and the labels, identity columns and status are metered
  by its own reservations (a stored text that is not the staged one is loaded, under its own
  reservation, by the full readback). Unpriced, as for the submitted paths: SQLite's page cache and
  journal (and, before store schema v14, FTS5's in-memory index of the search text), the staged pass's members (the record's
  non-row-scale identity members, as the header record) and Tauri's delivery copies of the small
  reply.
- **The reopen's leases** (slice 6), under the same rules and within the command's running total
  (above; the held leases leave it when the handle is established): the args lease as every command's; from
  the measured lengths, before anything of the run is loaded, the held result's and record's leases
  (each its text's envelope, adaptive, capped) and SQLite's copies of both (ended after the load); each
  parse's materialization M (result, record, manifest) admitted before the parse, a recorded run's
  record loaded once and its result parsed once (the verified read's own, RT-U4); the capped header's
  temporary serialized text and its parse M, both admitted before allocation, with the header parse
  lease held through reply serialization; the reply's exact length. The held leases become result
  leases and live with the bytes in the registry until disposal
  or reset (or the last reader's share). A restored result counts toward retention
  (`MAX_RETAINED_RESULTS`), and a restoration begun before a session reset is refused at registration.
  Unpriced: as listed in slice 6's limits.
- **The re-save's leases** (slice 6's save continuity), under the same rules and within the
  command's running total (above): the raw body's copy and
  the arguments' lease as the fresh save's; each held text's parse M (result, record) admitted before
  the parse; the working lease (the fixed allowance and the canonical writer's scratch); each output
  text at its exact length; SQLite's statement copies; the reply's exact length. The held texts are
  the handle's result leases, shared, credited nothing more; the parses and the working lease end
  before the statements; a disposal or reset during the save releases the texts when its share drops.
- **Not priced:** the readers' working memory and S's working set (§7, item 3), and the copies
  Tauri makes of the body before the command runs, plus the returned response buffer and
  Tauri's copies during delivery after return, including concurrent replies (§7, item 3).
