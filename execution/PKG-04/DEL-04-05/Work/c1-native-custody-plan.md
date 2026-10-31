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

### Slice 2: S projection (native, one pass)

**Files:**
- New `src-tauri/src/result_summary.rs` and `result_planes.rs`.
- Command `read_preview_mechanics_job_summary`, which returns S: JSON plus the planes as
  one binary IPC response (a length-prefixed JSON header, then little-endian f64 planes).

**S carries:**
- status, summary, numerical quality, formulation basis, professional boundary;
- diagnostics;
- `contract_evidence` headers (combination per-row evidence excluded);
- the row count and the distinct units;
- the native numerical health (`numerical_health_json`'s projection on the same parse);
- the tables each panel builds today:

  | Panel | Builder today |
  |---|---|
  | `DisplacementExtremesPanel` | `displacementExtremes.ts` |
  | `SupportLoadSummaryPanel` | `supportLoadSummary.ts` |
  | `StressRatioTablePanel` | `stressRatioTable.ts` |
  | `WeightSummaryPanel` | `componentBody.ts`, weight rows |
  | Key Results | `keyResults.ts` |
  | `ResultsPanel` family counts, frame counts and navigator cases | `countResultFamilies`, `countResultFrames`, `navigatorCases` |
  | viewport reactions and deformation | `reactionOverlay.ts`, `deformationOverlay.ts` row selection |

- **Planes:** nodal translations and rotations per set, and per member and station, the
  quantities `resultColourOverlay.ts` colours by (`isDrawableRow`, `membersOf`), with the
  row id of each cell so a pick opens the full row.

**Rule:** S's tables are computed by ports of the TS builders, so each TS builder stays the
oracle until the switch.

**Tests:**
- For every result fixture the TS builder tests use (the corpus `fixtures/results/*`, the
  demo results, the `nativeMechanicsReplay` fixtures), the TS builder over the full rows
  equals the same builder fed S.
- That runs as a vitest leg over committed native S outputs, generated by a Rust test in
  check mode, as `base_reader_outcomes` does.
- The planes are checked bit for bit against the rows.
- Coverage: S's JSON part is bounded by members × sets, supports and nodes, not by rows. A
  test checks it on a scaled invented rack.

### Slice 3: paged detail and batched semantics

**Files:**
- `src-tauri/src/result_detail.rs`: an index of row byte ranges in the held text, built in
  the same pass as S (one `(start, end)` pair per row).
- Command `query_preview_mechanics_job_rows {jobId, filter:{set, kind, entity, station,
  text}, offset, limit}` returns `{total, rows: [exact row texts]}`.
- Command `read_preview_mechanics_job_row_semantics {jobId, rowIds}` returns one batched
  `reader_verdict::row_semantics` call per page, from the same table as
  `readerRowSemantics`.
- A row returned is the byte range of the held text, so it is byte-identical to the
  envelope's row (§2.2 of the design).

**Tests:**
- Concatenating the pages of the unfiltered query reproduces `results` exactly.
- Each filter equals the TS filter over the full rows.
- The semantics equal `readerRowSemantics` per row on the corpus (`rows.sha256` of
  `base_reader_outcomes.json`).

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

### Slice 5: native analysis-run preview, Record Run and Save Project (v0.3, exact bytes)

**The record builder:** `buildAnalysisRunV03` (`analysisRunCompatibility.ts`) is ported to
Rust in the desktop crate (`analysis_run_v03_build.rs`). It reads the held bytes and the
small inputs (the manifest, the rule-pack exclusions and the load basis) and returns the
record text. The webview keeps only the record's header (run id, hashes, statuses and counts).

**The proof:** for every fixture `analysis_run.json` in `fixtures/results/`, and for the
records `analysisRunCompatibility.test.ts` builds, the TS builder and the native builder give
byte-identical records. The TS builder stays as the oracle in tests until C2.

**Persistence:**
- `record_analysis_run` and `save_local_project` take the job handle, not the result and
  record texts, and read the held bytes, so they store exactly today's bytes.
- **Exact lengths:** before writing, each computes the exact stored length of the result
  text and of the record that embeds it. It compares each with
  `sqlite3_limit(SQLITE_LIMIT_LENGTH, -1)` on the connection, through rusqlite's `limits`
  feature (a feature of a dependency already present, no new crate).
- **Refusals by name:**
  - `RUN-RECORD-RESULT-TOO-LARGE {result_bytes, record_bytes, limit_bytes}`: nothing is
    written;
  - `PROJECT-SAVE-RESULT-TOO-LARGE`: the model is saved with `mechanics_result_json =
    'null'` and `analysis_run_json = 'null'`, and the reply states that the result wasn't
    saved.

**Tests:**
- At the limit and one byte above, with the limit lowered on a test connection, for both
  refusals.
- Below the limit, stored bytes equal the take path's bytes (a regression against today's
  store).
- Persistence gets the independent persistence review.

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
| Workspace preview and binding (`workspaceSession.ts`) | `buildAnalysisRunPreview(output)`, `model_ref`, `run_id`, row count, row lookup | native record preview (slice 5); S's header fields; by-id page |
| Design workspace, export review, export unit disclosure, run audit, report lint (counts and units) | `results.length`, units | S |
| Reports (`reportResultRows.ts`, `ReportPanel.tsx`, `renderableReportInput.ts`) | every row | C2; in C1 full rows up to 64 MiB, refused by name above (slice 4) |
| Comparisons (`runComparison.ts`, `withinRunComparison.ts`, `ComparisonPanel.tsx`) | every row | C2; in C1 full rows up to 64 MiB, refused by name above (slice 4) |
| Rule checks (`ruleCheckService.ts`, `RuleCheckRunPanel.tsx`, `DeclarationsEditor.tsx`) | every row | C2; in C1 full rows up to 64 MiB, refused by name above (slice 4) |
| Stress-neutral export, handoff, local FEA, native package, headless runner panels | every row | C2; in C1 full rows up to 64 MiB, refused by name above (slice 4) |
| JSON export (`resultExportAdapter.ts`) and Save Result JSON (`save_local_result_json`) | the envelope | Save Result JSON writes the held bytes natively (slice 5); the derivative is C2 |
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
- **Results groups and endpoint pairs:** the table pages, and the CSV pages through the
  native query in chunks. Each chunk is formatted by today's TS formatter and appended
  through a native file writer (the `native_result_download` pattern), so the webview never
  holds the whole group.
- **Comparisons:** C2; in C1 full rows up to 64 MiB, refused by name above (slice 4).

**Tests:**
- The guard stays at 9.
- A new test checks that the paged CSV of each results group equals today's in-memory CSV
  on the fixtures.

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
   (id vectors, maps over rows) and S's construction are O(rows) and run under that lease
   unpriced, as `command_ingress`'s residual already states for command working memory.
   Slice 7 measures them; if they're material, the lease adds a working-memory factor, as
   the report package's does.
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
   - DEL-04-07: its memory-budget decision is not edited (Agent 0). §8 states how the C1
     leases fit it, and a proposed factual note goes to the owner in the PR body;
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
- **Not priced:** the readers' working memory (§7, item 3), and the copies Tauri makes of
  the body before the command runs (as for every command).
