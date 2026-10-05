# RV84: independent re-derivation review of I65 U4 grant 3 (producer composition at the caps)

**Candidate:** `R/I65/u4_g3_01/` committed at NUM `3260d7809e`. Its SHA256SUMS verifies 56/56. The packet cites source at NUM `5ae5fe4f0f`, and U1(a) at `59a5de2032`. I read source at `5ae5fe4f0f` for every packet citation. I read U1 as merged at `3260d7809e` for T11 and for the late-capture hook.

**Reviewer:** RV84, a TASK (Type 2) dispatched by ROOT under `BRIEFS/RV84_U4_G3_REVIEW.md`. I did not write the packet. I re-derived each item from source. I used the author's scripts and outputs only as leads, except for the inputs named as PACKET in `RX/rv84_compose.py`.

**ROOT's mid-task message** asked me to:
- cite RV83's R-1 (`R/REVIEW_RV83/u4_g2_02/REVIEW.md`), not duplicate it;
- look for other dropped call patterns;
- say whether any composed figure or the recursion claim changes materially.

§3 answers it.

**Abbreviations:**
- P, PP, FK (= FKS/retained), FKS, R, T3: as in the brief;
- RR = T3/ROOT_RULINGS_V1.md;
- G3 = R/I65/u4_g3_01;
- G3R = G3/_run_records;
- RX = R/REVIEW_RV84/u4_g3_01/_run_records;
- CJ = P/core/serialization/canonical_json/src;
- SJ = the cached serde_json-1.0.149 sources.

All source lines are at `5ae5fe4f0f` unless stated otherwise.

## Verdict: PASS

**Findings:** 0 BLOCKING, 7 SHOULD-FIX and 13 NOTE.

**The composition holds up under re-derivation.** No ruling moves:
- **The branch structure is right:** X (exact-block selected, T25 inside G-A's span) and W (W1 runs).
- **Branch X needs D1.11 (ε = 2).** With today's D1 it is over M by about 1.4–1.5 GB, on my numbers as well as the packet's.
- **The packet's T25 is a conservative upper bound** of its peak stage. My live-set model of the peak gives 1.35 GB (ε = 2), against the packet's 1.44 GB.

**The defects are omissions of the kind the packet itself calls loose, each bounded and small against M:**
- two more call-graph and loop-rule holes in the text bound (S-1, S-2), beyond RV83's R-1;
- an unpriced escape temporary in the hash route (S-3);
- T25's commitment-stage coefficients (S-4);
- branch X's later phases missing from the stated admission formula (S-5);
- the gate-fact hand-off (S-6);
- an unstated U3 premise about how many ordinary runs an invocation makes (S-7).

**They do matter for the margin rule.** By the packet's own byte classes they add up to about 131 MB to branch X. The dense, ε = 2 branch then sits at 0.907 M against the adopted 0.9 M rule. With my tighter T25 it sits at 0.881 M (N-4).

## Findings

| ID | Sev. | Where | Evidence | Remedy |
|---|---|---|---|---|
| S-1 | SHOULD-FIX | G3R/callgraph.py:40 (`CALL` lookbehind `[^A-Za-z0-9_:.]`); TEXT.md:14, :151 ("complete") | **A third dropped call pattern, besides RV83 R-1:** a call written right after a single `:`, as in compact `json!({"k":f(x)})` or `S{a:f(x)}`. The lookbehind excludes `:` so that it does not split `a::b`, and so drops these calls. RX/rv84_colon_calls.py finds 30 absent caller→callee edges on reached functions. **On the T25 path, PP/source_receipt/source.rs `commitment` and `function` lose these:**<br>– `matrix` (:5), `matrix12` (:11) and `products` (:17), the `bits` strings: N² aggregate, 3·144·m frame and every atom;<br>– `lowered_products` (:46), one `format!("{path}/{i}/{j}")` per atom, at most 16,384;<br>– `recipes` (:153), six `format!`s per checked row;<br>– `unit` (:29);<br>– the `functional_id` and `bits` calls inside `function` (:150).<br>The edge `audit_intended_action → physical_residual_record` (FKS/structural.rs:938) is also lost; it carries no text. **TAV hidden** (RX/rv84_colon_quantify.out.json): 84.8 MB at the packet's byte classes, of which `{path}`, classed as 2,173 B static, gives 72.6 MB. At source spellings it is 9.1 MB. | Accept `:` but not `::` before a call name. Re-run TEXT together with RV83 R-1's repair in G4. |
| S-2 | SHOULD-FIX | G3R/loop_bounds.json:363 (`components\|…`), :373 (`wind\|…`), :403 (`span\.start_fraction\|intensity`) | **Zero-bound rules match by substring, and the first match wins. They zero real D1 loops:**<br>– PP/lib.rs:12350 and :12443, `in local_components` (4 entries each; endpoint and station stress);<br>– :10987, `in components` (6 nodal displacement components);<br>– retained_product.rs:2000, `components.into_iter()` (3);<br>– lib.rs:9727, `boundaries.windows(2)`. The `wind` rule matches "windows"; the true bound is 1;<br>– :9753, `zip(&intensity)` (3 accumulators).<br>Their rows report multiplicity 0 (RX/rv84_zero_loops.out.txt). **TAV missed at the packet's classes: 35.9 MB** (RX/rv84_zero_quantify.out.json). The other zero hits I read are genuinely empty in D1: components, combinations, nonlinear, curved, thermal, pressure and load-state. | Anchor each zero rule to the D1-empty collection it means (`model.components`, `.exposed_spans`, …), and order specific rules before general ones. Add a self-check that lists every zero-matched header with the collection D1 forces empty. |
| S-3 | SHOULD-FIX | G3R/t25_caps.py:74–78 (hash route); RESIDUALS_G3.md:108–114 | **The canonical render allocates a temporary for every string and every key:** `serde_json::to_string(text)` at CJ/lib.rs:132 and :155. Its capacity is at most max(128, 2·(ε·L + 2)). The route prices this as a constant 2·max(24, RID) = 2,048 B, which scales neither with ε nor with the longest string L. **The publication's longest string** is the integrity message, at most 2,549,385 B (G3R/composite_text.caps.json `message_int`). So the temporary is 10.2 MB at ε = 2 and 30.6 MB at ε = 6. For the payload, whose longest string is `source_identity` at ≤ 1,797,413 B, it is 7.2 / 21.6 MB. **In I1 this is absorbed** by `body_vf.tree()`, which is not yet live when `hash(publication)` runs (source_receipt.rs:1105; json! inserts entries in order), and by T25.9's dropped temporaries. G4 reuses the route for T16/T17. | Add `max(128, 2·(ε·L_max + 2))` per hashed Value, with L_max a stated text atom. Price the serde_json parser scratch, ≤ 2·L_max, in the parse phase. |
| S-4 | SHOULD-FIX | RESIDUALS_G3.md:102, :136; G3R/t25_caps.py:122–131, :69 | **Remainder 1, T25's commitment coefficients, re-derived field by field** (RX/rv84_t25_s4.py). `payload_vf` omits three things:<br>– the `source_identity` string, ≤ 1,797,413 B (source.rs:376);<br>– `stiffness_aggregate_bits`: N² = 36,864 strings in N + 1 arrays;<br>– the per-atom factor objects that `function()` builds, `{kind, source_path, bits}` (≤ 16,384 atoms; FKS/exact_boundary/functionals.rs:409–476 counts atoms in the 16,384 limit).<br>`parsed()` also uses 4·obj as a stand-in for 4·#arrays, and #arrays exceeds #objects in the payload and functions.<br>**S4 core at the 16,384-unit limit:** 0.563 GB (ε = 2) and 0.957 GB (ε = 6), against the packet's 0.460 / 0.732 GB with the carried outputs removed. **The peak is unchanged:** I1 still exceeds S4 by at least 2.2×. | Replace `payload_vf`, `functions_vf` and `plan_vf` with field-level facts. Use the array count in `parsed()`. State L_max for S-3. |
| S-5 | SHOULD-FIX | COMPOSITION.md:32, :36; RR "U4 G3 verified" ("`admit` must cover `max(E_X, E_W + G4)`") | **E_X stops at T25.** On branch X these phases still follow:<br>– the receipt stays in the envelope (`into_wire` deep-copies the body, source_receipt.rs:1112; it is retained, PP/lib.rs:2801);<br>– U3's pre-reserved fallback notice (T18) is live for the whole invocation;<br>– the Direct completion and output transfer of that envelope (T19) runs on X too.<br>Numerically these lie below I1, because the hash-route transients of about 1.1 GB drop first, but the formula as recorded has no X-side G4 term. **The `max` itself is right as a design condition.** Under plan §2.3(b) it is sufficient. Soundness would survive a G-A that checked only the ordinary span, because G-C can refuse to the completed ordinary result. With cap-priced constants, though, E_W + G4 > M would refuse every W1 run, so the condition is needed for W1 to be reachable. | Record admission as max over all phases: E_X(T25), E_X(completion) = O + Text + T11 + retained receipt + T18 + T19_X, and E_W + G4_W. G4 prices T18/T19 on both branches. |
| S-6 | SHOULD-FIX | COMPOSITION.md:76–102 (§4); PP/lib.rs:5009–5012; retained_product.rs:3060–3062 | (a) **G-B's placement cannot read most of `LateFacts`.** G-B is the late-capture hook `prepared_case_source`, whose arguments are `source_selected, model, built, materials, case, restrained, springs, application, thermal, pressure`. No results, diagnostics or retained errors are passed, so `case_rows`, `case_row_text_bytes`, `diagnostics`, `diagnostic_text_bytes` and `retained_error_text_bytes` are unreadable there without a hook change. That change is in PP/lib.rs, I61's integration site under D-5.<br>(b) **`CompleteFacts` names an owner that no longer exists at G-C.** `source_cases` and `source_case_row_json_census` (COMPOSITION.md:98) refer to the local at PP/lib.rs:2615 (:2619 at `3260d7809e`), which is consumed or dropped before the ordinary owner returns. The envelope has no such field (lib.rs:814–832).<br>(c) **No gate reads the late capture's own capacities after it is made.** G-B reads before it, and `CompleteFacts` has no `observation_bytes` or late-capture bytes.<br>(d) **COMPOSITION.md:84 labels the bound "2·Text(diag_total)",** but 155,625,688 = 2·Text(diag_env). The number is the right one. | Name the hook extension, with its owner, in API.md. Drop `source_cases`. Add `observation_bytes` and `late_capture_bytes` to `CompleteFacts`. Fix the label. |
| S-7 | SHOULD-FIX | G3R/text_budget.caps.out.json `function_multiplicity`; G3R/loop_bounds.json `fn_cap` (`solve_load_case_observed ≤ cases`) | **TAV mixes two premises about U3.** It takes the union of both roots, so M(`run_linear_static_preview_observed`) = 2 and route-level text is counted twice. Meanwhile `solve_load_case_observed` is capped at `cases` = 1, and M(`check_input_with_physical`) = 1, which is right only if an invocation makes **one** ordinary run. `prepare_observed` itself runs the ordinary route (retained_product.rs:3265–3269). If U3 also ran the plain route, the per-case subtree would be under-counted by 2×. If it runs once, the route level is over-counted. | Make "exactly one ordinary run per invocation" a G4 budget item for U3 (design-to-budget). The route-level double count then becomes a tightening lever. Otherwise lift the per-case caps to 2. |
| N-1 | NOTE | RV83 R-1; RX/rv84_augment_scc.py, rv84_newly_reached.py | **I did not repeat RV83's enumeration.** As an independent check, I fanned every `.name(` out by name within the crate-dependency closure:<br>– reach rises from 1,881 to 1,982 functions (+101);<br>– the only text rows in the newly reached functions are FKS/formation_check.rs's 5, which agrees with RV83;<br>– other patterns probed show no D1 text effect (§3). | — |
| N-2 | NOTE | TEXT.md:44 | **"`?` conversions construct no text" is false** for `impl From<&str>` and `From<String> for CaptureError` (retained_product.rs:2988–2996). Every `ok_or("…")?` on the W1 path allocates the literal on its error path. The bytes are negligible: one literal per error. | Correct the sentence; no figure moves. |
| N-3 | NOTE | STACK_INVENTORY.md:13, :35 | **"No mutual recursion" is unverified on a repaired graph.** My augmented graph adds 17 cyclic components (RX/rv84_augment_scc.out.txt). The path I traced through the 50-node SCC is a name collision: `WorkTotal::add` (wide_sum.rs:158–180) fans out to the Enclosure `add` (product_certificate.rs:343) (RX/rv84_cycle_path.out.txt). I found no genuine mutual recursion, but G4's adjudicated graph must settle the claim. R = 64 MiB and k = 16 are unaffected on present evidence. | G4 re-runs the SCCs on the repaired graph. |
| N-4 | NOTE | COMPOSITION.md:5–12 | **The headline figures reproduce** (RX/rv84_compose.py). With the packet's inputs, my arithmetic gives X = 3,500,163,917 / 5,543,257,845 (sparse, ε = 2 / 6) and W = 1,930,160,927, within 0.01%. My independent I1 live set gives:<br>– T25 = 1,345,957,094 (ε = 2) and 2,999,736,142 (ε = 6);<br>– X (dense, ε = 2) = 3,426,506,985 = 0.851 M.<br>**Which figures are ASSUMED:** everything except TAV and R rests on ASSUMED strides: O, T25, T11–T15, and especially Node(String,Value) = 736 and s(Value) = 32.<br>**Sensitivity:** adding S-1 to S-3 at the packet's byte classes (+131 MB) puts the packet's dense ε = 2 branch X at 3,651 MB = 0.907 M. With my T25 it is 0.881 M. | ROOT: the 0.9 M margin for branch X is about 104 MB (dense, ε = 2) on the packet's numbers. |
| N-5 | NOTE | G2_AMENDMENTS.md §5–§6; RR "U4 G3 verified" | **D1.11 caps expansion at 2:**<br>– SJ ser.rs:2135–2165 escapes only 0x00–0x1F, `"` and `\`;<br>– CJ delegates every string and key to `serde_json::to_string` (lib.rs:132, :155);<br>– 0x7F is escaped by neither, so its place in the clause is harmless (it matters only to Debug);<br>– no product literal on the graph carries a control character (the `\x01` bytes are binary K4 encodings, not JSON).<br>**D1.10 holds:** only an object provenance yields a `method` (self_weight.rs:796–799), and D1.7 excludes Element targets. The per-load parse transient is RV83 R-3. | — |
| N-6 | NOTE | RESIDUALS_G3.md:44, :125 | **A tightening lever.** T25 is reachable only when `descriptors_charge` passes (≤ 16,384 units), but #1 holds 41,760 units at the full cap vector. So T25 is priced at counts that cannot select, and the payload uses 41,760 units. Pricing T25 on the selection-feasible region (smaller m) would shrink P and D_env. | Option for ROOT's sensitivity table. |
| N-7 | NOTE | RESIDUALS_G3.md:64 | **The T22 census reproduces** for FK (60 / 36 / 9 / 30 / 5) and for retained_product.rs's `CountRange` (38 / 36) and `u32::try_from` (8). **"26 `checked_mul`" spans four PP files:** retained_product.rs 15, case_state/temperature.rs 8, source_recovery.rs 2, retained_memory.rs 1. The table values reproduce:<br>– p = 4,640 (d is directional springs, 0 in D1; FK/source.rs:443–452);<br>– q = 1,472; encoding 36,742; free·(free+1) = 37,056; 64m = 24,704.<br>The `ceil_sqrt` total-function lemma checks against bound.rs:996–1006 and :1044. | Fix the label. |
| N-8 | NOTE | ORDINARY.md:51–58, §3 | **The six I54 milestone sub-expressions reproduce in my own arithmetic:** 4,712; 4,796; 4,712 and 7,016; 2,992; 12,752 (I54 RETURN:12–14; correction_03 RETURN:8–9). Every primitive the lemmas use is nondecreasing, including the 0→s·48 sort-scratch step and `G(h)`. O-N's 1,664 B reproduces; RV83 R-3 adds the parse term. | — |
| N-9 | NOTE | RESIDUALS_G3.md §T07 | **The T07 arithmetic reproduces:** identity names 1,992, bits 12,456, e = 1,797,413; #1 units 41,760. F1 and H4 (2·2,048 B per Ratio under `expansion_terms` 256) and the three-generation count check against source. T07 is 1% of M, so no materiality question remains. | — |
| N-10 | NOTE | COMPOSITION.md §2 T11 | **U1's `OrdinarySeed`** (retained_product.rs:149–200 at `3260d7809e`) matches T11.4's roster, and 11,968 B reproduces. `InitialSeed::Report.code` is not priced separately; it sits inside the 4·RID slack. The +64 B `invocation_digest` is confirmed (:97, :308). | — |
| N-11 | NOTE | G3R/compose_caps.py:37 | **W's moving term** adds an undocumented 2 MiB candidate and does not enumerate T12–T15's old backings. O-without-T25's largest old backing is the 8,388,608 B maximum helper (my re-evaluation of `ordinary_caps`' candidates). No T13 owner approaches the 16.8 MB it would need to exceed that, so the figure holds. | List T12–T15 candidates in G4's T21. |
| N-12 | NOTE | STACK_INVENTORY.md §1, §4 | **The thread-local inventory reproduces exactly** from a non-test grep of the 15 crates. ROWS sits in `mod tracker_hook` (adaptive.rs:5186–5187), not `hooks`; `seeded` is gated at retained/mod.rs:46. `assemble_sparse_stiffness` recursion ≤ 2 (sparse.rs:600–612) and `retained_basis` ≤ 2 (self_weight.rs:745–751) are confirmed. | — |
| N-13 | NOTE | TEXT.md §1–§4 | **The sampling stated in §3 holds.** Of 30 random reachable functions, 29 have site counts that match the inventory exactly. The 30th is canonical_json's nested visitor, which is excluded by file. About 30 more functions and 18 exclusions were checked by hand. The top-ten multiplicities are over-counts: exclusive `match` arms are counted together in `validate_final_metadata`, and support × restraint products are counted per support. RV83 R-2 (`validate_profile`) stands. | — |

## 1. The composition and the fit (COMPOSITION.md)

**The branch structure is right.**
- After exact-block arbitration a D1 case is either:
  - **selected:** `source_selected` at PP/lib.rs:2614; `preview` is None at :2616; finalize runs at :2788–2811; and `observer.finish` refuses at retained_product.rs:1502–1507 because `source_block_recovery.is_some()`;
  - **or not selected,** which is branch W.
- **A selected case whose finalization fails stays on X.** The envelope then carries the source-blocks contract id with no receipt, and the dispatcher returns `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` (lib.rs:2221–2226). W1 never runs after T25.
- **In D1 the publication holds no `contract_evidence`:** `preview` is None on X and `is_exact` is false. So `finalize_for` passes its check at source_receipt.rs:913–917 and reaches `serialized(envelope)`.

**`max(E_X, E_W + G4)` is right as a design condition, but incomplete** (S-5).

**Double counting, all conservative:**
- O keeps the preview rewrite and final copy, which X never runs;
- T25's I1 includes `body_vf.tree()` and T25.9's dropped temporaries;
- TAV counts route-level text twice (S-7);
- the maximum helper appears in both O and T14.

**Omissions:**
- S-3, the escape temporary;
- S-5, X's later phases;
- S-1 and S-2, holes in TAV.

**T21 is sound.** The invocation is single-threaded, so at any instant at most one reallocation is in flight, and the requested sum uses final capacities. Requested plus the largest single old backing therefore bounds every instant, provided the candidates span the whole branch:
- for X it is the publication text's last growth, e;
- for W it is 8,388,608 (N-11).

**My re-derivation** (RX/rv84_compose.out.json):

| Mode, ε | X: packet's T25 | X: RV84's I1 live set | W, G3 part | G4 headroom |
|---|---|---|---|---|
| sparse, 6 | 5,543,257,845 | 5,468,921,577 | 1,930,160,927 | 2,096,370,913 |
| sparse, 2 | 3,500,163,917 | 3,406,796,537 | 1,930,160,927 | 2,096,370,913 |
| dense, 6 | 5,562,968,293 | 5,488,632,025 | 1,949,871,375 | 2,076,660,465 |
| dense, 2 | 3,519,874,365 | 3,426,506,985 | 1,949,871,375 | 2,076,660,465 |

My X figures for the packet's T25 differ from COMPOSITION.md only through my own count of the publication facts. My publication has 66,312 slots and 102,086,498 string bytes, against the packet's 66,528 and 102,171,746, because it holds no preview part. TAV, O, T11–T15 and the carried case outputs are PACKET inputs. S-1 and S-2 together add up to about 121 MB to both X and W at the packet's byte classes.

## 2. T25: selected source-blocks finalization

**The peak stage, I1** (`hash(publication)` inside the body `json!`), from source:
- **Live at I1** (source_receipt.rs:867–1105):
  - the moved `cases`;
  - `publication`;
  - the typed `request`, with its raw clone already consumed;
  - `ids`, `wire`, `accounted` and `observations`;
  - the partly built body map.
- **The hash route** (source_receipt.rs:30–37 → CJ/lib.rs:39–43):
  - the `json!` wrapper's deep copy;
  - the `to_string` text, ≤ max(128, 2e) (Vec::with_capacity(128), then doubling);
  - the checked-parse tree: `Vec::new()` + push arrays, with keys owned and cloned into a per-frame `HashSet`;
  - the canonical output, ≤ max(8, 2J), with J ≤ the same e formula;
  - **and the per-string temporary** (S-3).
- **The parser's scratch and seen sets are dropped** before the render (`checked_parse` returns first), so the render phase is the peak.

**The publication facts:**
- `ResultItem` has 3 objects and ≤ 15 entries per row, with 128 B of keys in total (lib.rs:1977–2018);
- `Diagnostic` has 6 entries (lib.rs:2022–2032);
- `evidence_refs` holds ≤ 1 per case (lib.rs:1910);
- strings = P·Text(row) + Text(diag_env) + ≤ 6 KB of scaffolding;
- Text(row) = 11,474: ≤ 4 source refs. The 5-ref rows come only from components (lib.rs:11607), which D1 excludes.

**The ε factor.**
- e ≤ ε·strings + keys + 8·values + 24·numbers.
- ECMA number spelling reaches 25 characters, and `false` needs 9 bytes in an object, but the 8-per-value overhead absorbs both.
- The publication's keys are static field names, so factor 1 is right for them.
- D1.11's ε = 2 is verified (N-5).

**The commitment coefficients (remainder 1)** are re-derived in S-4 and RX/rv84_t25_s4.py. They are incomplete, but they do not set the peak.

## 3. Text (TEXT.md, T08)

**Other call patterns** (ROOT's request). Beyond RV83 R-1, I probed the following:

| Pattern | Finding |
|---|---|
| Colon-adjacent calls | Dropped (S-1) |
| Trait objects | Only `dyn FnMut` (numeric closures, FK adaptive.rs:3288; nonlinear_integration structural_adapter.rs:970) and `dyn Error`. No text |
| Closures passed to user functions | 17 `Fn`/`FnMut`/`FnOnce` parameters (RX/rv84_patterns.out.json). Each invocation inside the callee's loop is not multiplied. The only one carrying text is `load_row_finding`'s `dof_label` (formation_guard.rs:301, :315), which D1 never reaches: it returns at `has_formation_records()`, and the edge is excluded |
| `macro_rules!` | All 7 are defined inside fns (`run`, `assign`, `merge`, `delta`; `layout` is test-only) or generate impls (`core_width`). Their calls are attributed to the enclosing fn, and invocation counts are not multiplied. The FK retained text under them is ≤ 4 sites (N-13 sample) |
| `impl Trait`, operator, `Deref`, `Drop`, `Ord`, `PartialEq` impls | No text or recursion (`Ratio::eq` compares bits) |
| `?` From conversions | N-2 |
| Serde attribute functions (`serialize_with`, `deserialize_with`) | Named only in string literals, so invisible to the graph. `serialize_finite_f64` allocates only on failure; `closed_quantity` is load-state, excluded |
| The custom `Deserialize for PreviewModel` (lib.rs:228) | Allocates typed Vecs (T03), not text |

**Materiality.**
- **RV83 R-1 plus S-1 plus S-2** add about 0.12 GB of TAV at the packet's byte classes. At source spellings S-1 is 9.1 MB; I did not re-spell S-2's sites.
- **The fit is unchanged:** X at ε = 2 stays under M by more than 0.37 GB.
- **The 0.9 M margin is tight** (N-4).
- **The recursion claim** is open pending G4 (N-3).

**The lexicon.**
- A grep of the reached files for `String::from`, `repeat`, `to_*case`, `collect::<String>`, `serde_json::to_string`, `with_capacity` and `into_owned` (RX/rv84_lexicon_gaps.out.txt) finds outside the inventory only:
  - RV83 R-N3's `collect::<String>`;
  - the identity JSON, priced by T07;
  - load-state and composite sites, which D1 does not reach.

**The exclusions I checked against source** (18 or more):
- `resolve_shared_sections`, :7357–7361;
- `add_uniform_element_loads`, :9891 (empty for nodal loads);
- `build_thermal_element_loads`, :10051;
- `build_nonlinear_supports`, :6299, and its two field-diagnostic helpers (only called at :6362–6436);
- `validate_spring_hangers`, validation.rs:818, and its 3 helpers;
- `append_exact_pressure_results`, :4851–4855;
- `append_equivalent_static_generated_loads`, :8708;
- the 17 self-weight functions under D1.10;
- the edge `load_row_finding → decide_row`;
- `edge_once` for `normalize_quantity → unit_conversion_diag` (three mutually exclusive returns, :8357–8384) and for `…captured → …captured_once` (the load-state fallback only);
- the caps on `blocked_envelope`, `SupportFinding::new` (≤ g + r per `prepare_boundary`, with M = 4) and `add_restrained_dof`;
- the files composite.rs, pressure_material.rs and case_state/.

**Two are wrong:**
- `validate_profile`, which is RV83 R-2;
- the `canonical_json` file exclusion. It rests on the hash route, which lacks the S-3 temporary.

**The top ten.** The multiplicities I checked are over-counts (N-13). Among the top sites, `normalize_quantity` (M = 5,244) counts four normalizations, and its `unit_conversion_diag` site runs at most once per call.

## 4. T05 and the O-N row

N-8 covers the six milestone sub-expressions against I54, and the monotonicity lemma. O-N is otherwise RV83 R-3.

`normalize_vector_quantity` (lib.rs:8389) clones the unit three times and the refs twice. It is called only for components (:7974), so it has multiplicity 0 in D1, correctly. **Ruling S-2 out for O:** none of its zero hits touches an O family.

## 5. T07 and T22, as repaired

**T07:** N-9.

**T22:** N-7. The site table's checks exist and are typed stops, except the stated `ceil_sqrt` total-function lemma. The D-4 citation re-point to RESIDUALS_G3 §T22 is safe.

## 6. T11–T15 and T21

**T11:** N-10. The late capture is suppressed on X (retained_product.rs:3075), so counting T11 whole on X is conservative.

**T12–T15:** I did not re-derive their I29/I51 rosters. I spot-checked the counts: Q = 1,472; P_final = 2,115; E_src = 36,742; and the 262,144·s(Node) helper is shared with O. T13 is the retained-superset alternative, as the packet says.

**T21:** §1 and N-11.

## 7. STACK_INVENTORY.md

- **The recursion list (18)** reproduces from G3R's graph.
- **Two bounded recursions** are confirmed in source (N-12).
- **The derive and std recursion bounds** are sound: Value depth ≤ 18, BTree height ≤ 7, and a serde_json parser limit of 128.
- **The thread-local finding** reproduces (N-12).
- **"No mutual recursion"** depends on the graph that RV83 R-1, S-1 and S-2 affect (N-3).

**R/k:** about 34 frames, plus 18 Value levels, plus about 36 reader levels, at ≤ 16 KiB per frame, is under 1.5 MiB. That leaves 40× headroom against 64 MiB and 2.7× at the R/16 witness. The 16 KiB-per-frame figure is an assumption, which the per-identity witness checks. Confirmed as provisional.

## 8. COMPOSITION §4: LateFacts and CompleteFacts

**The bounds reproduce:**
- 2·R0·Text(row) = 43,394,668;
- 2·P·Text(row) = 48,535,020;
- 97·16,384 = 1,589,248;
- 2·Text(diag_env) = 155,625,688.

**Every listed fact is a len or capacity read over borrowed owners,** and `contract_evidence` uses the fixed-frame census (retained_memory.rs:95–166). They are therefore allocation-free **where the owner is in scope**.

**Sufficiency and placement fail in four places** (S-6):
- G-B's hook does not receive the rows, diagnostics or errors;
- `source_cases` is gone at G-C;
- the late capture is never measured after it is made;
- the label is mixed up.

**Neither list can cover the bulk of live text** (TAV). That is acceptable, because the gates are a cross-check of the derivation, not the bound itself.

## For ROOT

1. **The margin rule.** At the packet's byte classes, S-1, S-2 and S-3 put branch X (dense, ε = 2) at about 0.907 M, against the adopted 0.9 M. With RV84's tighter I1 live set it is 0.881 M. G4 should apply the rule to the repaired TAV and the corrected T25 together. If it trips, the sensitivity-table levers apply:
   - N-6, pricing T25 on the region where selection is feasible;
   - S-7, the route-level double count.
2. **S-5 amends the wording of the G3 ruling.** Admission is a maximum over phases, including X's own completion (T18, T19) and the receipt it retains.
3. **S-6(a) needs a PP/lib.rs hook change** at the late-capture call site. Under D-5 that is the single integration owner's (I61's), so it should be named in API.md before G5.
4. **S-7 is a U3 budget item** under design-to-budget: one ordinary run per invocation.

## Execution record

**Who.** RV84, TASK (Type 2) under ROOT, with no descendants and no delegation.

**Memory guard.** `memguard.sh` (PID 5387) was running at the start, checked with `pgrep -fl memguard`.

**Not run.** No Cargo, rustc, solver, native or DEC-025 job; no install and no new tooling.

**Git.** Reads only, all with `GIT_OPTIONAL_LOCKS=0`: `rev-parse`, `log`, `status`, `diff --stat`, and `show` (the merged `retained_product.rs`). `archive` of `5ae5fe4f0f`'s `core/` was piped to tar into the scratch directory, for reading only.

**Run.** Stdlib Python, in RX:
- the composition and T25: `rv84_compose.py`, `rv84_t25_s4.py`;
- the call graph: `rv84_augment_scc.py`, `rv84_chain_calls.py`, `rv84_colon_calls.py`, `rv84_newly_reached.py`, `rv84_new_edges_text.py`, `rv84_cycle_path.py`;
- text: `rv84_sample_sites.py`, `rv84_zero_loops.py`, `rv84_zero_quantify.py`, `rv84_colon_quantify.py`, `rv84_lexicon_gaps.py`, `rv84_patterns.py`.

Each takes G3R and/or the P root at `5ae5fe4f0f` as arguments, and each writes its output next to itself. I also read the installed serde_json 1.0.149 source.

**Writes.** Only RX, this file, SHA256SUMS, and the scratch directory `WT/scratch/rv84_u4_g3_01/`. Nothing went to the system temp directory. No machine path appears in this directory.

**Not touched.** G3, R/REVIEW_RV83/, and I61's and I65's working files.

**Time.** About 45 minutes against the 4-hour box. Nothing in the brief is left unfinished. I did not re-derive the I29/I51 rosters for T12–T15 or the I54 O families beyond the six cross-checks and spot checks; this is stated here.
