# Result volume: summary first, detail on demand (design proposal "C")

**Status:** proposal for SWBPIPE Agent 0's approval. It changes nothing by itself. It is a
cross-lane change of what results are published and held where, so each phase lands only
after approval, with its own reviews, re-pricing and reconciliation. It serves RT-U4 (exit
criterion 9) and the usability of ordinary models. Basis: main at `cfd17e103` and the
measurements in `criterion-9-measurement.md` (§5, §6).

## 1. The problem, in numbers

Today one solve publishes one JSON envelope holding every published row. Every consumer reads
that envelope whole.

| Where | What was measured |
|---|---|
| **Engine** | The `linear-ex` rack publishes about 1,890 rows per member across its 12 case and combination sets. 93% of them are the five station rows per member and section basis. The worker's footprint grows by about 5.9 kB per row. At 1,000 nodes that is 1.9 M rows, 1.84 GB of JSON and a 10.6 GB peak. At 2,000 nodes it is 3.8 M rows, 3.68 GB and a 22.4 GB peak, against the 12 GiB per-solve budget. |
| **Engine stages, 1,000 nodes** | Per-case solves hold 1.4 GB. Combinations raise that to 4.1 GB, with a transient of 6.4 GB. Serialization adds the full string (doubling while it grows). The attestation parse-back adds about 4 GB; PR #39 removes it. |
| **Desktop** | WebContent grows to about 60–80× the result's bytes plus about 1 GB. A 0.46 GB result hangs it, and 0.92 GB or more crashes it. PR #51 refuses anything over 64 MiB, which today means linear-ex racks of about 36 nodes or more. |
| **Reader verdict** | Before the reader fixes: 239 s at 82 nodes, from a quadratic duplicate-id check, a per-station index scan, and one wasm call per row key on the results panel. The fixes bring it to 5–7 s natively. It is still linear in the rows, and still runs in wasm inside WebContent. |
| **Persistence** | Record Run and Save Project write the whole envelope as one SQLite TEXT value, whose default maximum is 1e9 bytes. |

The fixes in flight (#39, #49, #51 and the reader bundle) make the failure honest and the
verdict fast. None of them changes the basic shape: every consumer holds every row. Criterion 9
(2,000 nodes, ≤ 10 s linear static end to end, provisional) and ordinary models above about
36 nodes need that shape to change.

## 2. What must hold

1. **Every station number stays available and verifiable.** Nothing published today is
   dropped or replaced by a recomputation. Derivable values such as σ±, τ and magnitudes stay
   published as they are, with the same ids, kinds, units, metadata and bits. Only where they
   are held changes (Agent 0, 2026-10-31: option D is not applied on its own).
2. **The same row is the same row.** A detail row read from the store is byte-identical to the
   row in today's envelope. Readers, reports and comparisons that work per row keep their
   per-row semantics.
3. **The webview never holds the full row set.** Its memory is bounded by what is on screen
   (summary tables, one page of detail, and numeric arrays for the viewport), not by the
   model's size.
4. **The engine never holds the full row set either** (phase C3). Rows stream to the store
   case by case and member chunk by member chunk. Peak memory is bounded by a chunk and the
   solve, not by rows × cases.
5. **Attestation, run records and reports bind everything.** One digest covers the summary
   and a Merkle root over the detail chunks. Any detail row can be proven part of the run.
6. **Old records still open.** Stored runs and projects saved before C stay readable with
   their original hashes. Migration is explicit and never rewrites historic evidence.

## 3. The shape

### 3.1 Two tiers

- **Summary (S).** A small JSON document. In the webview it is held as an object; natively, as
  bytes. It carries:
  - the envelope's status, summary, numerical quality, formulation basis, professional
    boundary and diagnostics, with repeated warnings aggregated (§3.6);
  - `contract_evidence`, except the per-row combination evidence, which moves to D (§3.5);
  - **summary tables:**
    - per case and combination, the governing value of each published stress and force kind
      with its member, station and basis (the "extremes" table);
    - per member, its governing stress per case;
    - support reactions;
    - nodal displacement extremes;
  - **viewport planes,** as columnar binary arrays rather than JSON rows: nodal translations
    and rotations per case (6 × nodes × cases float64), and the per-member, per-station
    stress measures the colour map uses (quantities × stations × members × cases, float64).
    At 2,000 nodes and 12 sets that is about 1.2 MB for displacements and about 10 MB for a
    dozen colour quantities. That's two orders of magnitude below today's text;
  - **the detail manifest:** chunk ids, row counts, the id range and the kinds in each chunk,
    each chunk's sha256, and the Merkle root.

  At 2,000 nodes, S is expected to be about 15–30 MB, dominated by the planes. Its JSON part
  is bounded by members × sets plus supports and nodes, not by stations × kinds × bases.
- **Detail (D).** The full set of published rows, exactly as today. They are split into chunks
  by set (load case or combination) and by member range, each about 4–16 MB of canonical
  rows, plus the per-row combination evidence for those rows. D is held natively, in the
  app's private result store (§3.4), never in the webview.

### 3.2 Where it is read

- **Verdict, natively.**
  - The summary reader checks S on its own: its internal coverage, and that every summary
    reference names a row id inside the manifest's ranges.
  - The detail reader runs today's per-row checks chunk by chunk, streaming, in native Rust
    and not in wasm:
    - station recomputation;
    - pressure 5-station × 2-basis coverage;
    - global end coverage;
    - combination re-derivation, with operand chunks of the same member range read together.
  - Cross-chunk invariants hold by construction and are checked through the manifest:
    - **id uniqueness:** ids include set and member, and chunk id ranges are disjoint;
    - **evidence references:** they resolve through the id ranges.
  - The verdict is the conjunction of both readers. It carries the same codes, refusals and
    diagnostics as today, proven equal on the reader corpus by running both shapes.
- **Display.**
  - The webview receives S. Tables and the viewport work from S.
  - The results table becomes a paged query over D through native IPC, with a filter (set,
    member, kind, station) and a page of N rows with full metadata. Row semantics (labels,
    units) come from one batched native call per page.
  - **CSV:** the summary tables export as today. "Export all detail rows" streams natively
    from D to a file, with no webview copy.

### 3.3 Engine streaming (phase C3)

- **Sink.** `product_physics` publishes through a row sink instead of a growing
  `Vec<ResultItem>`. A case's station rows are produced per member already (`u4b_rows`), so
  the sink writes them to the open chunk, closes the chunk at its size, and records its hash.
- **Combinations** are per-key algebra over operand sets. They run per member range: read
  that range's operand chunks, combine, and write the range's combination chunk. Memory is
  operands × one chunk. The per-row combination evidence goes into the same chunk.
- **Summary rows and planes** are folded from the stream as it passes: governing values, then
  extremes, then planes. They are never rebuilt from the full set.
- **Worker protocol.** The worker writes D's chunks into a job directory the parent creates,
  private, under the app's data directory. It sends S over the pipe as today. `d_S` covers S,
  and S carries the manifest root. The parent verifies each chunk's hash as it registers the
  chunk. The result lease covers S plus a bounded chunk buffer; chunks on disk aren't leased
  memory.
- **Serialization doubling goes away,** because chunks are written with known bounds and S is
  small.
- **Headless runner and CLI.** They write S plus a chunk directory. A `--single-envelope`
  option reassembles today's envelope for tools that need it, refusing above a stated size.

### 3.4 Persistence

- **Store.**
  - Projects and stored runs gain a chunk table: `(store_id, chunk_id, sha256, bytes BLOB)`.
    Each value stays well under SQLite's 1e9 limit.
  - `mechanics_result_json` holds S.
  - Chunks are content-addressed by sha256 and shared between a project's result and the runs
    recorded from it, so Record Run doesn't copy the detail.
- **Run record.**
  - The analysis-run record moves to v0.4. Today's per-row `ResultRef` with its row checksum
    is replaced by S's hash and the Merkle root, plus the per-chunk hashes.
  - **A per-row proof is derivable:** the chunk hash, then the root. The verifier checks every
    chunk hash against the root. A run's integrity check therefore still covers every row,
    without the record holding one entry per row.
- **Migration.**
  - Records and projects saved before C, with a single envelope in `result_json` or
    `mechanics_result_json`, open read-only under their original hashes and verifiers. The
    v0.3 verifier stays.
  - A large legacy result opens through the same native path as C1 (§4), so it never reaches
    the webview whole.
  - A user can convert a legacy project to S + D explicitly. The converted copy gets new
    hashes, and the legacy record is kept.
  - No historic evidence is rewritten.

### 3.5 Per-row combination evidence

It is audit evidence, about 16% of today's envelope bytes, so it stays available. It moves
from S's `contract_evidence` into D, beside the combination rows it evidences. S keeps each
combination's record header (operands, gate, standing, counts). That evidence belongs to the
row it explains: it is fetched with the row, exported with the detail, and covered by the same
chunk hash.

### 3.6 Repeated warnings (option E)

A diagnostic raised once per load per case, such as `LOAD_CATEGORY_PREVIEW_MAPPED`, about 10k
times at 2,000 nodes, becomes one diagnostic per case and code. It lists the affected refs
(complete, in a field the reader checks) and carries a count. Ids stay unique, and
`affected_refs` keeps every load. This changes report and run bytes, so it lands in its own
small PR (approved to go early). It is independent of the tiers.

### 3.7 Minimal publication for derived solves

Internal derived solves, such as hanger sizing's 2n + 3 one-case solves (RT-A11), request a
tier holding only nodal, support and support-state values. Agent 0 approved it in principle,
on three conditions:

1. **Internal only.** It is never a user-visible result, never recorded as a run and never
   attested. Its numbers reach the person only through the requesting analysis's own result,
   with that result's basis block.
2. **Bit for bit.** Its nodal, support and support-state values are identical to a full-tier
   solve of the same document, because it skips publication work, not computation. A
   regression compares the two tiers on the same models.
3. **No user selection.** Neither a model document nor the UI can select it; only an internal
   caller.

## 4. Phases, ordered by the usable size each one raises

| Phase | What lands | Who and where | Usable size after it |
|---|---|---|---|
| **C0** (in flight) | #51 refusal at 64 MiB; reader bundle (dedupe, batched semantics, health key) and `Index::any`; #39 attestation digest; #49 combination keys and the append-peak fix | this lane | No hang or crash. The verdict is fast natively, but the webview limit is still 64 MiB (about 36 nodes, full case set) |
| **C1: native custody** | The webview stops taking the envelope. The app process keeps the solve's bytes (it already holds them until the take). It runs the readers natively, computes S's tables and planes from the envelope with one streaming pass, and serves detail pages by paged IPC from an in-process index. Record Run and Save Project write S plus content-addressed chunks, cut from the envelope. There is no engine change; the published envelope is byte-identical | this lane, with DEL-07-xx and DEL-08-04 reviews | Bounded by the worker budget, not the webview: about 1,000 nodes, full case set, today's engine. The 64 MiB refusal moves to "over the worker budget", which the worker cap already refuses truthfully |
| **C2: records on S + D** | analysis-run v0.4 (§3.4); reports (S's tables by default, detail for selected members, checksums binding S, the root and the chunks used); comparisons (summary in the webview, detail natively by row identity); rule-pack station selectors resolved natively against D; `swbpipe_read_results` (summary plus paged detail with metadata); stress-neutral export streamed from D | this lane with the owners of DEL-08-01, DEL-08-04, PKG-14, the rules and RT-E1 | unchanged size; every consumer is on the new shape |
| **C3: engine streaming** | the row sink, chunked combinations, worker chunk protocol (§3.3); the minimal derived tier (§3.7) | this lane in solver files under the solver-repair extension, with RT-A3 agreeing the combination part | 2,000 nodes and beyond within the 12 GiB budget. The engine peak becomes solve plus chunk, not rows × cases |
| **C4: retire the single envelope** | The legacy single-envelope path stays only for opening old records and for the CLI's explicit `--single-envelope`. The refusal's interim wording goes | this lane | — |

**Why C1 comes first.** It changes no published byte and no engine code, yet it removes the
60–80× webview growth that dominates today. The native readers already exist; the report path
uses them. It lifts the usable size from about 36 nodes to about 1,000 for models whose solve
fits the budget. It also makes the next steps local to the desktop: C2's consumers move one at
a time behind the same native custody.

**Fit with other lanes:**
- **0.9.0 S1 (#50):** C touches no model-document surface and can land before or after it.
- **The reference system:** it gains no new fields.
- **RT-A3's 1b and stage P:** they continue in the solve. C3's sink touches only publication,
  and is agreed with RT-A3 because `u4b_rows` and `exact_combinations` are its neighbours.
- **The direction-magnitude fix:** it lands before the reader bundle wherever they share
  reader code.

## 5. Every consumer, and what changes

| Consumer | Today | With C | Phase |
|---|---|---|---|
| Readers (Rust/wasm, `reader_verdict.rs`, `physics_evidence.rs`, `preview_physics_evidence.rs`, `u4b_evidence.rs`) | whole envelope, in wasm in the webview | natively; S reader plus chunked D reader; same codes and refusals, proven equal on the corpus | C1, then C2 for the D reader |
| Results panel and table (`ResultsPanel.tsx`) | all rows in memory, client-side paging | S tables, plus a paged native detail query | C1 |
| Viewport colour and deformation (`resultColourOverlay.ts`, `PipeViewport.tsx`) | scans `result.results` | the columnar planes in S | C1 |
| Numerical health and standing | full-text wasm calls | native verdict summary in S | C1 |
| Reports (DEL-08-01; `reportResultRows.ts`, `report_package` bindings) | one report row per envelope row, each checksummed | S's tables by default; station detail for selected members; checksums bind S, the root and the chunks used | C2 |
| CSV for every table (RT-R6, DEL-08-04; `csvTableCoverage.test.ts`) | 9 tables, from in-memory rows | the same 9 tables from S, plus a native "all detail rows" stream; the coverage guard covers both | C1 (tables), C2 (detail stream) |
| JSON export (DEL-08-04 derivative, `derivative.rs`) | 1:1 row accounting over the envelope | the same accounting over S + D, streamed natively | C2 |
| Comparisons (PKG-14; `runComparison.ts`, `engine.py`) | rows mapped by identity in memory | summary comparison in the webview; detail comparison natively by row identity, chunk by chunk | C2 |
| Rule packs (`rule_solver_selection.rs`, `run_rule_checks`) | envelope sent over IPC; rows qualified in place | selectors resolved natively against D's index; envelope not sent | C2 |
| Analysis-run record (`analysisRunCompatibility.ts`, `compatibility.py`, `analysis_runs.rs`) | one `ResultRef` and checksum per row, plus the whole-envelope hash | v0.4: S hash, Merkle root, chunk hashes; v0.3 kept for old records | C2 |
| Stored runs and project save (SQLite) | one TEXT value of the whole envelope | S plus content-addressed chunk blobs, shared | C1 |
| Stress-neutral export (`package.py`) and headless runner | every row | streamed from D | C2 |
| Embedded agent `swbpipe_read_results` | 50-row pages of id/kind/value/unit, no metadata | summary tables plus paged detail with metadata | C2 |
| MBF handoff | doesn't read results | unchanged | — |
| Solve attestation (`d_S`) | over the whole result text | over S, which carries the root; no change for derived solves, which aren't attested | C3 (worker) |

## 6. Risks and how each is closed

- **Verdict drift between the shapes.** Both shapes run on the reader corpus and the racks
  until C4; every code, refusal and diagnostic must be equal.
- **Chunk boundaries splitting a check's inputs.** Chunks are by set and member range. Every
  per-row check reads only its member's rows and its operands' same-member rows, so a member's
  rows never span chunks.
- **Persistence and migration.** Old records keep their verifier. Conversion is explicit and
  copies; it never edits. A persistence change gets its targeted regression and independent
  review.
- **Pricing.** C1 and C2 are desktop-side. C3 touches priced publication code, so it gets a
  full re-price, an independently reviewed registration and the AC-004 witnesses.
- **Scope.** Each phase is its own PR or bundle, with an xhigh review where it touches readers,
  attestation or what a person sees.

## 7. Decisions needed

1. Approve the two-tier shape (§3) and the order C1 → C2 → C3 → C4.
2. Approve C1 as the next unit of work after C0, desktop-only, with no published byte changed.
3. Approve where the per-row combination evidence lives (§3.5).
4. Approve the v0.4 run record with a Merkle root replacing per-row refs (§3.4). It's a
   persistence and schema change; the owner may want to see it.
5. Confirm E's aggregated diagnostic shape (§3.6) before its PR.
