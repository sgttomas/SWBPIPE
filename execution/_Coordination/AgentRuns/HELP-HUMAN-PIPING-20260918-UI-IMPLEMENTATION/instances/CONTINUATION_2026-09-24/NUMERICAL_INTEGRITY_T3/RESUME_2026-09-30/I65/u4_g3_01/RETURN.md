# I65 U4 G3: return

**G3 is complete and ready for review:** RV83 confirms the G2 repairs; RV84 reviews G3. Every G3 term and every RV83 repair is priced at the D1 caps, symbolically per layout atom, with illustrative ASSUMED numbers. Two domain proposals and one modelling choice need ROOT.

**The headline:** at the caps, the exact-block-selected branch fits M only with the proposed D1.10 and D1.11.

| Deliverable | File | Result |
|---|---|---|
| T05, the ordinary span per mode, with a monotonicity lemma for each family class and the new O-N row (S-4) | ORDINARY.md | without T25: 245.2 MB (sparse), 264.9 MB (dense). Six milestone cross-checks reproduce I54 exactly |
| T07 repair (S-1) | RESIDUALS_G3.md §T07 | 39.8 MB requested, 41.6 MB moving. Both identity copies, J3 capacity, the #1 construction law, the Response temporaries, three descriptor generations, and the owners G2 missed |
| T22 repair (S-2) | RESIDUALS_G3.md §T22 | I34's classes site by site; the `ceil_sqrt` total-function lemma; free·(free+1) = 37,056; a reproducible census (60/36 FK, 38/36 PP) |
| **T25, new** (B-1) | RESIDUALS_G3.md §T25 | **3.07 GB at ε = 6; 1.44 GB at ε = 2.** The canonical hash of the whole envelope dominates |
| T08 text (B-2, S-6) | TEXT.md | **complete** over 1,881 reachable functions and 2,563 sites. Total allocation volume 1.54 GB; D = 25,544 (envelope 11,030) |
| T11–T15 and T21 | COMPOSITION.md §2–3 | T11 0.2 MB (U1(a) OrdinarySeed 12 KB); T12 2.0 MB; T13 42.8 MB; T14 23.2 MB; T15 0.3 MB. Moving terms: one largest old backing per span |
| Per-mode tables, gates, and the `LateFacts`/`CompleteFacts` fields (the G2 API hand-off) | COMPOSITION.md | branch X: **5.54 GB > M at ε = 6**; **3.50 GB ≤ M at ε = 2**. Branch W (G3 part): 1.93 GB, leaving about 2.08 GB for G4 |
| T20 stack | STACK_INVENTORY.md | 18 self-recursive functions, no mutual recursion, nothing count-proportional; R = 64 MiB and k = 16 confirmed; the thread-local inventory shows nothing production-visible |
| G2 amendments, each pointing back to the line it replaces | G2_AMENDMENTS.md | B-3 and S-5 (single-line escaped identity, `option_env!`, G5 tests); S-3 (typed capacity caps); S-4 (no sections); N-1, N-12; NOTE dispositions; proposals D1.10 and D1.11 |

## What ROOT should decide

1. **D1.11: no control characters in input strings** (G2_AMENDMENTS §6). Without it, worst-case JSON escaping (6×) puts the selected-finalization branch over M at the caps, and `admit` would refuse every D1 invocation. That is safe, but useless. With it, branch X fits with about 0.51 GB to spare (ASSUMED). The milestone satisfies it: none of its 213 strings and keys has a control character.

2. **D1.10: no JSON-object provenance on primitive loads** (G2_AMENDMENTS §5). It removes the self-weight validation path from D1. Without it, the same text method gives about 10.6 GB. The milestone satisfies it. If ROOT declines, the remainder is a per-load bound of about 1–2 h.

3. **The text model.** At 1.54 GB, text is the second-largest term, because it counts every transient String as live. G4's T16/T17 hash the successor envelope by the same route as T25, which costs about 1.4 GB at ε = 2 against about 2.08 GB of branch-W headroom. I recommend a lifetime-aware text refinement before G4 closes its composition. Smaller m, g or l caps are the alternative lever.

4. **D-4 citation.** The ruling's T22 premise should point at RESIDUALS_G3.md §T22. The outcome is unchanged.

## Things ROOT should know

- **A call-graph defect was found and fixed in this grant.** Multi-segment path calls (`a::B::f(`) were silently dropped, which hid T25's finalize path and T07's `exact::Context::…` calls from the text reachability. After the fix:
  - the reachable functions rose from 1,665 (Direct root, before the fix) to 1,881 (both roots, after), and TAV from 0.73 to 1.54 GB;
  - no new recursion cycle appeared (still 18);
  - every reached file is in the macro inventory (73 of 93).

  The method is still lexical; RV84 should spot-check it.

- **NUM moved during the grant** to `2a645453ad`. U1 grants 1–2 and U2 were merged at 23:52 MDT (`4a13e369b9`), after every source-reading run here. All citations are at `5ae5fe4f0f`, with U1(a) read from `59a5de2032` as instructed. The grant-2 delta was checked for G3 terms:
  - `ProductCapture.invocation_digest` (+64 B) is now in T11;
  - the new `ProductProofWork.anchor` is an `Arc::clone`, so it adds no heap;
  - the serializer (retained_wire.rs) is T16, which belongs to G4.

- **Every number depends on illustrative strides** (`g3lib.py`), except two DWARF-derived node sizes. G5 evaluates the same symbolic expressions in-build. The fit conclusions are therefore conditional on the real layouts not differing materially from the ASSUMED ones.

## Remainders (precise)

1. **T25's source-commitment coefficients.** These are the `function()` and `recipes()` object, entry and string counts per descriptor and per row (source.rs). They come from a reading, not a field-by-field trace, and RV84 should re-derive them. They are not the peak stage.
2. **The self-weight path**, only if D1.10 is declined: about 1–2 h.
3. **G5:** the in-build evaluation of every atom; the gate arithmetic at M−1, M and M+1; the witnesses W1–W7. W3, the selected T25 path, is new and important.
4. **G4:** T16–T19 within branch W's headroom, and confirmation that the reader's deepest schema chain is witnessed.
5. **Stated residuals:** the panic-hook stack (S1 evidence); a lexical call graph rather than a compiler one.

## Re-estimate (the 18-hour rule)

G3 ran from 22:00 to about 00:00 MDT, roughly 2 hours of wall-clock time against an 8–12 h budget. No remainder above is forced by the time rule; they are open by scope or by decision.

## Execution record

**Who.** TASK I65 (Type 2) under ROOT, with no descendants.

**Memory guard.** `memguard.sh`, PID 5387, was running at the start, during the work and at the seal.

**Not run.** No Cargo, rustc, solver, native or DEC-025 job. Git reads only (`show`, `log`, `diff --stat`, `status`, `cat-file`), all with `GIT_OPTIONAL_LOCKS=0`. U1(a) and the NUM revisions were read through `git show`, never from the f2a-serializer working tree.

**Run.** Stdlib Python under `_run_records/`:
- the call graph (`callgraph.py`, `loopscan.py`);
- text (`text_lexicon.py`, `text_budget.py`, `composite_text.py`, `t08_closure.py`, `explain_mult.py`). The extension inventory `template_inventory_ext.out.json` is G2's `template_inventory.py`, run over the 79 files its `files` field lists;
- the term evaluators (`g3lib.py`, `t07_repair.py`, `t22_sites.py`, `t25_caps.py`, `ordinary_caps.py`, `producer_caps.py`, `compose_caps.py`);
- `schema_depth.py`, `alloc_sites.py` and `origins.py`, which writes `ORIGINS.json`: 40 sources and 16 records at `5ae5fe4f0f`, and 2 U1(a) files.

**Writes.** Only R/I65/u4_g3_01/. A search found no machine path anywhere in the packet. Two disclosures:
- Earlier in the grant, two intermediate JSON outputs (`tb_all.json` and `tb_scratch.json`) were written to the system temp directory, outside the fence. They were deleted, and all later intermediates stayed inside the fence.
- One measurement (text without D1.10) used temporary rule files inside `_run_records`, which were deleted after their figure was recorded. That run started after the NUM merge, so its approximate 10.6 GB is indicative only.

**Not touched.** R/REVIEW_RV83/.
