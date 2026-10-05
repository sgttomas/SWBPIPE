# RV84 confirmation: are the G3 findings repaired in I65's G4?

**What I checked.** RV84's G3 findings S-1 to S-7 and N-3 (`R/REVIEW_RV84/u4_g3_01/REVIEW.md`), against `R/I65/u4_g4_01/` committed at NUM `406e4c8916`. Its SHA256SUMS verifies 77/77.

**The basis.** G4 reads source at NUM `b1f80234dc`, and so do I. I extracted it with `git archive` to the scratch directory, for reading only.
- NUM later moved to `b4ab0159c3`, adding the l ≤ 128 addendum, the RV87 and G5 briefs, and a rulings entry. Those changes are records-only and add files.
- None of the files confirmed here changed.

**Who.** RV84, the same TASK (Type 2), under ROOT's mid-task message. Per that message:
- RV87 reviews T16–T19 and the l ≤ 128 re-run;
- RV83 confirms its own R-items;
- I did not repeat either.

**Abbreviations:**
- G4 = R/I65/u4_g4_01; G4R = G4/_run_records;
- RX = R/REVIEW_RV84/u4_g3_02/_run_records;
- PP, FK (= FKS/retained), FKS: as before.

Source lines are at `b1f80234dc`.

## Verdict: CONFIRMED

S-1 to S-7 and N-3 are each **fixed**, checked with my own probes. This pass has no new BLOCKING or SHOULD-FIX finding. It has **5 NOTEs** (C-N1 to C-N5). Each is under 0.15% of M, and none changes a phase maximum or a ruling.

| Item | Status | My probe |
|---|---|---|
| S-1: calls after a single `:` | **Fixed** (R1h) | RX/rv84c_graph_probes.py. Every T25 edge I listed now exists: `commitment` → `matrix`, `matrix12`, `products`, `recipes`; `function` → `lowered_products`, `functional_id`, `bits`, `unit`. Over the 2,698 reached functions, only 3 colon calls lack an edge, and all 3 are local closures (`body`, recover.rs:103; `row`, adaptive.rs:4174; `equal`, physics_source.rs:468). The rows are now priced: source.rs:51 at multiplicity 16,384 × 129 B (my source spelling), source.rs TAV 71.2 MB. The 314 chained-call misses are std iterator/f64 methods, `cfg(test)` / `mutation-controls` blocks (retained_product.rs:3694; assemble.rs:706–744), or closures. I checked `value`, `cmp_value`, `abs`, `get`, `first`, `work_summary` and `prepared_lane_work` against source |
| S-2: broad zero rules | **Fixed** | RX/rv84c_zero_rules.py. Every loop I cited now has positive multiplicity: lib.rs:12614 (896 = 224 × 4), :12707 and :12718 (1,024), :11248–11260 (384 = 64 × 6), :10029 `windows(2)` (96 × 1), :10015 `zip(&intensity)` (288 = 96 × 3), retained_product.rs:2168. `ambiguous_supports` (preview_physics.rs:202–224) is bounded by r (192). I read the 40 headers that still zero a reached row. All iterate collections that D1 empties: components, combinations, nonlinear, curved, thermal, pressure (with regions, terminals and traversal), load-state, wind, distributed spans, hanger and constant-effort. The `modulus_basis_record` filter at lib.rs:9755 is also empty: D1.5 emits no record row (lib.rs:9720, :4156). The packet's `zero_matched_headers` has 56 entries, not the stated 57 (C-N5) |
| S-3: the per-string temporary | **Fixed** | G4R/t25_g4.py:93–99 and g4_caps.py:79 add max(128, 2·(ε·L + 2)) plus the parser scratch 2·L at each hashed Value's L. `parsed()` now uses 6 × slots; Σ max(4, 2h) ≤ 6h holds for every non-empty array. Two L atoms are slightly low (C-N1, about 0.3 MB) |
| S-4: T25 commitment coefficients | **Fixed** | `payload_vf` adds `source_identity`, N² aggregate bits and force bits; `functions_vf` adds one `{kind, source_path, bits}` per atom (≤ 16,384). G4's S4 core is 637.5 MB (ε = 2) and 1,001.5 MB (ε = 6), above my field-level 563.2 and 956.5 MB at the true 16,384-unit limit (RX of u4_g3_01, `rv84_t25_s4.out.json`). The surplus comes from pricing the payload at #1's 41,760 units. Term `dof` numbers are still missing (C-N2) and absorbed. I1 (1.42 GB) stays the peak |
| S-5: admission over every phase | **Fixed** | COMPOSITION_G4 §1 and g4_caps.py:310–339. The admission bound is `max` over X1, X2, W1–W5 (:339), each requested + moving + R. I traced both branches through caller completion in source (lib.rs:2243–2263, 2884–2960). Every phase is present, including X's completion (receipt, reserve, T19) and W's publication, precommit and transfer. Fallback exits are prefixes of these. Three residual owners are omitted or misnamed in individual phases, together ≤ 5.4 MB (C-N3) |
| S-6: gate facts and hook | **Fixed** | API_G4 against the source. G-B's `LateFacts` now holds only what `prepared_case_source` receives (retained_product.rs:3222–3244), plus `capture`. Rows, diagnostics and errors move to G-C. `source_cases` is dropped (the local at lib.rs:2679 is consumed or dropped first). `observation_bytes` and `late_capture_bytes` are added at G-C. The label is now 2·Text(diag_env). The hook change is assigned to I61 (D-5); the shared borrows are compatible with `self.permit.as_ref()` at `a634ac8b53`. One fact's owner is misnamed (C-N4) |
| S-7: one ordinary run | **Fixed** | Root is the Direct entry only. `fn_cap` sets `run_linear_static_preview_observed` = 1; M(`check_input_with_physical`) = 1 and M(`solve_load_case_observed`) = 1. Source shows exactly one ordinary run: `admit` → `permitted_dispatch`, else `ordinary_dispatch` (lib.rs:2256–2261); `ordinary_dispatch` runs only if the reserved thread did not (:2899); `permitted_run` runs either the Domain `ordinary_dispatch` (:2930) or its own observed run (:2934). The load-state second run is outside D1. TAV_X and TAV_W replace the double-counted total |
| N-3: recursion | **Fixed** | RX/rv84c_collisions.py covers **all 17** of my G3 candidate cycles (not a sample). In G4's graph each member's same-named call goes to std (Vec, slice, integer and f64 methods; `self.values.is_empty()` and the like) or to the distinct real method: `WorkTotal::add` at work.rs:80 from wide_sum.rs `reserve`; Wide `neg` from source_residual.rs:22; `FrameElement::length`/`local_stiffness`/`global_stiffness` (FK lib.rs:586/607/611, `impl FrameElement` :562) from straight_pipe :445/:458/:462; `BodyEvidence::geometry` :1258; Wide `widen` multi.rs:770. None has a self-edge. **Rules checked against source:** 13 of the 24 new ones (52–62, 64–70), all correct. Rules 64–69 are right because a `Type::method` path resolves to the inherent method before the trait's. **The self-loops** (RX/rv84c_selfloops.py): all 22 are genuine self-calls, direct or as a fn value (`.all(safe_integers)`, `.any(floats)`, `for_each(normalize)`), and 19 are reachable. Each recurses on a Value child (depth ≤ 18) or a schema node (≤ 36), or is flag-bounded: `assemble_sparse_stiffness` re-enters with `SparseAssemblyOptions::new()` (sparse.rs:605–612), and `retained_basis` recurses only on a LEGACY-method provenance (self_weight.rs:745–751). The 3 unreachable ones are canonical_json binary64 walkers at depth+1 ≤ 128. A blunt fan-out of every `.name(` on G4's graph produces only collision SCCs (`len`, `is_empty`, `widen`, and a 752-node `collect`/`max` blob), so it is not evidence of recursion |

## New NOTEs (this pass)

| ID | Sev. | Where | Evidence | Remedy |
|---|---|---|---|---|
| C-N1 | NOTE | G4R/t25_g4.py:56, :58, :186–187; g4_caps.py:44 | **Two L atoms are slightly low.**<br>(a) **L_PUB** = `message_int` = 2,549,385. The reached text sites that build the final integrity message are larger: PP lib.rs:1137, after the formation-check and range-scaling lines are appended (:1130–1141), is 2,599,962 B, and formation_guard.rs:487 is the same size (`largest_single_site_bytes`).<br>(b) **T25's receipt body** uses L_ROW = 2,173. The body carries diagnostic ids, ≤ L_DIAGID = 2,330 (source_receipt.rs:1048 `diagnostic_ref`), which g4_caps already uses for the U1 body.<br>The effect is about 0.3 MB in the hash route. | Set L_PUB to the larger of the two, and the T25 body's L to the larger of 2,173 and 2,330 |
| C-N2 | NOTE | G4R/t25_g4.py:155 | **Per-term `dof` numbers are unpriced.** The S-4 addition to `functions_vf` has no `nums` for the per-term `dof` (≤ 16,384), so up to 16,384·24 B of text is missing, about 1.6 MB in the route. It is absorbed: the payload is priced at 41,760 units, and G4's S4 exceeds my field-level S4 by 74 MB (ε = 2). | Add `nums = 16,384` |
| C-N3 | NOTE | COMPOSITION_G4.md:21–27; G4R/g4_caps.py:310–325 | **Three phase-level owners:**<br>(a) **T19's spawn heap** (`Arc<ScopeData>`, the thread `Arc`, and the `Arc<Packet>` that holds s(output)) is allocated by `on_reserved_stack` (lib.rs:2894, :2907–2918) **before** the observed ordinary run. It is live in X1 and W1–W4, but only X2 and W5 count it (≤ 10,240 B).<br>(b) **The 13 reader statics are process-lifetime** (PUBLICATION_READER.md:113). After the first permitted invocation they are live in every phase of later invocations, but only W4 and W5 count them (5.4 MB, 0.13% of M).<br>(c) **X2 prices the retained receipt** with the U1 successor's `BODY` facts, not T25's receipt body. This is conservative: every fact is larger. `RECEIPT_X` (:301) is computed and unused. | Add T19 to every phase. Either add STATICS to every phase or state that M excludes process-global caches after first use. Name the receipt owner |
| C-N4 | NOTE | API_G4.md:80 | **`retained_error_text_bytes` names the wrong owner.** It reads "the maximum and record error Strings in `capture`", but `ProductCapture` holds no maximum or record error Strings (retained_product.rs:100–140). Its error Strings are `error`, `observable_error` and `g5a_error`. The ordinary run's member and recovery record errors are dropped before G-C, or are envelope diagnostics already covered by `envelope_diagnostic_text_bytes`. | Name the fields G5 reads; keep the bound |
| C-N5 | NOTE | G3_REPAIRS.md:237 | **The header count is off by one.** The text says the self-check lists 57 headers; `text_budget.caps.out.json` `zero_matched_headers` has 56. | Correct the count |

## For ROOT

**Nothing needs a ruling.** C-N1 to C-N4 are small corrections that G5 can absorb when it encodes the expressions. C-N3(b) is a question of wording for D-7's non-claims: whether process-global caches count against M after first use. It is cheapest to settle in the G6 record.

## Execution record

**Who.** RV84, TASK (Type 2) under ROOT. No delegation.

**Memory guard.** PID 5387, running at the start and at the end (`pgrep -fl memguard`).

**Not run.** No Cargo, rustc, solver, native or DEC-025 job, and no install.

**Git.** Reads only, all with `GIT_OPTIONAL_LOCKS=0`: `rev-parse`, `log`, `status`, `diff --stat` and `--name-status`. `archive` of `b1f80234dc`'s `core/` was piped to tar in the scratch directory.

**Run.** Stdlib Python in RX:
- `rv84c_graph_probes.py` (colon, chain and blunt-SCC probes);
- `rv84c_zero_rules.py`;
- `rv84c_collisions.py`;
- `rv84c_selfloops.py`.

Each takes G4R and the P root at `b1f80234dc` as arguments, and writes its output beside itself. The S-4 comparison reuses RX of u4_g3_01 (`rv84_t25_s4.out.json`).

**Writes.** Only RX, this file, SHA256SUMS, and `WT/scratch/rv84_u4_g3_01/`. Nothing went to the system temp directory, and no machine path appears here.

**Not touched.** G4, R/REVIEW_RV83/, and RV87's and I61's files.

**Time.** About 15 minutes against the 2-hour box.
