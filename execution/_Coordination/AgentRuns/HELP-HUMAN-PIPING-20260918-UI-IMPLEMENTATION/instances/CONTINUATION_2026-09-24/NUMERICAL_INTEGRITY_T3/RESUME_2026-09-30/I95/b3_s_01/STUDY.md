# I95 B3-S: pricing the exact route at C = 3 (Python only; records only)

TASK (Type 2), I95, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC.

**The brief:** `R/BRIEFS/B3S_EXACT_PRICING.md`, sha256 `9968db1d3e1a108eac2d53f27344eed9b603cb15900d8c2f0fe2c5a6d01c8751`, verified before reading.

**What I ran.** Python 3.13 from VENV over a `git archive` snapshot of main `2007709549` in `WT/scratch/i95_b3_s/`, using I82's chain unchanged plus one patcher of my own (`b3s_exact_chain.py`). No cargo build was needed and none was run; no native job, install or Git write. Git reads used `GIT_OPTIONAL_LOCKS=0`. Nothing went to the system temp directory. §9 has the execution record.

**Notation.** WT, NUM, P, PP, T, R, RR and VENV as in the dispatch.
- c: load cases; z: combinations; C_eq = c + z.
- E+R = E_mov,max + R, in-build bytes, as I82 (in-build atoms from I72's law record).
- **5 % M:** the smallest 256 MiB multiple M with E+R + 5 %·TAV ≤ 0.9 M in both modes. TAV is the binding branch's text: TAV_W when a W phase binds, TAV_X when an X phase binds.
- **S3** is I82's price for B1's target: one tier, D1's caps, C = 3 (I82 ADDENDUM_01 §1).
- **The exact route** is model 0.3.0 with `{2.0.0, exact_straight_pressure_v2}` and every case's `pressure_regions` explicitly `[]` (PLAN §1.4 item 3; I write this clause "D1.5-exact"). Sections stay absent. D1.4 and D1.6–D1.9 are unchanged, and 0.4.0 stays excluded.
- **The four chain variants** (`b3s_exact_chain.py` over I82's `mc_chain`):

  | Variant | Graph | D1.3 rules rebound | D1.5-exact credits |
  |---|---|---|---|
  | **erc** (the main figure) | exact route only: legacy-only branches zeroed (§2.4) | yes | yes (§2.3) |
  | er | exact route only | yes | no |
  | urc | union of both routes (one form set) | yes | yes |
  | ur | union of both routes | yes | no |

## 0. Findings in brief

1. **Decision 26's answer: the exact route fits within B1's M.**
   - At C = 3 on the exact route's own forms (erc), E+R is **9,900,151,888 B dense** and 9,841,020,544 B sparse. W3 binds in both modes.
   - **The 5 % M is 10.5 GiB** (11,274,289,152 B), the same as S3's. The text-error budget there is **5.95 % dense** and 7.37 % sparse; X1 has 5.89 % of TAV_X.
   - **No route caps are needed, and M need not rise.** At 12 GiB the budget would be 40.9 %.
   - The answer rests on two conditions ROOT must rule on (item 4).
2. **The E+R table at D1's model caps** (bytes; binding phase; 5 % M):

   | c | S3 (I82, reproduced) | **erc** | er | urc | ur |
   |---|---|---|---|---|---|
   | 1 | 3,595,488,734 / 3,575,778,286 W3; 4.0 GiB | **3,643,220,328 / 3,623,509,880 W3; 4.0** | 3,725,822,892 / 3,706,112,444 W3; 4.0 | 4,524,695,105 / 4,504,984,657 X1; 5.0 | 4,889,870,109 / 4,870,159,661 X1; 5.25 |
   | 2 | 6,592,539,324 / 6,553,118,428 W3; 7.0 | **6,688,918,758 / 6,649,497,862 W3; 7.25** | 6,909,912,301 / 6,870,491,405 X1; 7.5 | 8,843,993,088 / 8,804,572,192 X1; 9.5 | 9,591,078,160 / 9,551,657,264 X1; 10.25 |
   | **3** | **9,747,725,678 / 9,688,594,334 W3; 10.5** | **9,900,151,888 / 9,841,020,544 W3; 10.5** | 10,427,541,576 / 10,368,410,232 X1; 11.25 | 13,774,963,459 / 13,715,832,115 X1; 14.75 | 14,906,539,503 / 14,847,408,159 X1; 16.0 |

   Each cell reads dense / sparse, then the binding phase and the 5 % M in GiB.
3. **Against S3, the exact route at C = 3 costs +152,426,210 B** in both modes (erc minus S3).
   - +133,152,650 B is TEXT and its diagnostics knock-on. D grows from 41,412 to 51,570, and D_env from 22,911 to 23,895.
   - +19,273,560 B is the census: the exact contract evidence in place of the preview tree (`erc_nocensus`, §4).
   - **The binding terms** (W3 dense): TAV_W 4,149,552,618 (−40,143,720); T16 at P2 3,741,259,650 (+152,250,605); O_base 816,626,020; T16_moving 528,505,545 (+23,049,926); T12_T15 323,428,560; STAGED 267,820,013.
   - **X1 is only 32,235,846 B behind W3** (9,867,916,042): it carries the composite finalization's replays.
   - **The priced worst-case heap** of one W1 invocation is 9,833,043,024 B (9.16 GiB) against S3's 9.02 GiB: 57 % of a 16 GiB machine and 29 % of a 32 GiB one.
4. **The two conditions** (§7):
   - **(a) Route-exclusive forms.** If B3b's G5 prices one TEXT graph over both routes without excluding the legacy-only branches on the exact route (ur, urc), C = 3 needs 14.75–16.0 GiB, above the owner's 12 GiB. X1 then binds, because both routes' source-block finalizations and the 0.4.0 replays are summed. **G5 must price per route** (nine mirror rules, §2.4) and take the maximum over routes, as PLAN §3.2 item 1 allows.
   - **(b) Two D1.5-exact zero rules** (§2.3). With no pressure region, every cap, eigen and group term list is empty, so `exact_sum` returns `Ok(+0.0)` and the per-DOF `problem()` sites in `build_pressure_case_with_members` and `finish_source_groups` cannot run. Without these credits (er), C = 3 needs **11.25 GiB** (X1 10,427,541,576 B; 3.95 % at 11 GiB). That is still within the owner's 12 GiB, and ROOT may select it.
5. **C_eq = 3 with c + z ≤ 3.** On the exact route z = 0: `validate_profile`'s exact arm makes every combination a blocking `EXACT_PRESSURE_COMBINATION_UNSUPPORTED` (PLAN §0 item 4). So the exact route's C_eq = 3 point is its c = 3 point.
   - Over both routes, the registered maximum at C_eq = 3 is max(I93's legacy HIGH 9,749,149,984 B, erc 9,900,151,888 B) = **erc**. That is **10.5 GiB with 5.95 %**.
   - B3b's D1 must refuse z > 0 on the exact route, so that the exact forms need not price combination text (§7, ruling 4).
6. **The tree at c = 1.**
   - On main `2007709549`, I82's chain reproduces I82's c = 1, 2 and 3 trees byte for byte. Step 0 also equals I65's Pass A, so main's B6 and SI1b change no chain input.
   - **The exact route's c = 1 tree (erc) differs from I82's `d1_c1` in 24 forms and 6 text atoms.** E+R is +47,731,594 B dense.
     - Largest form deltas: TAV_X +99.7 MB, T16_P2 +59.8 MB, T25_I1 +57.9 MB, T17_V2_hash +57.9 MB, TAV_W −31.0 MB.
     - Text atoms: D 14,734 → 16,652; D_env 9,361 → 9,757; Text(diag_env) 68,720,236 → 73,364,299.
7. **Rules rebound: 6 removed, 5 restated at zero, 1 branch exclusion, and 38 rules added** for newly reached code: 32 loop rules, 4 error-path rules, 1 zero edge and 1 per-call bound. erc adds 2 credits and 9 mirror rules. Each is listed with its multiplicity in §2.
8. **The census.** Every committed exact-route output with empty regions publishes 7n + 51m + 8g rows per case (+ at most one dense-parity row, `sparse_live_path_dense_parity_relative_delta`, in dense), within the chain's P_final = 7n + 51m + 8g + 3. The exact contract evidence at D1's caps (about 0.3 MB per case) is substituted componentwise for the preview tree.

## 1. Basis and reproduction

**Code basis:** main `2007709549e9701b302e0eb62a1474d82acc1c40` (B6 merged). NUM's maintained tree equals it.
- **The tree check** (`b3s_tree_check.out.txt`): the 15 chain crate source trees, the three static schemas, PP's `Cargo.toml` and `build.rs`, and all ten `g4_caps.py` statics are identical to U8's `bd6b4be2c3`, where I82 line-mapped the rules (30 of 30).
- `fixtures/results` differs only in four files that are not chain statics: the corpus, the carrier cases and the two derivative fixtures (B6's 07m). Main's other changes since then (SI1b's `core/rules`, reader tests and the Python reader) are outside the chain's crate sources.

**Step 0** (`b3s_text_base.sh`, I82's `b1_text_base.sh` with only the scratch rebound; `base_reproduction.txt`):
- 8 of 8 non-TEXT outputs are identical to I65's G7 Pass A, including `profile_tree.json`, which is also identical to I82's `d1_c1`.
- The four text budgets differ in line numbers only: 0 changed rows of 2,814 each.
- The linemap remapped 0 rules, with 0 unmapped.

**I82's multi-case chain** (`b1_mc_chain.py`, unchanged) gives the same `I82_PATCHES.json`. At c = 1, 2 and 3, its trees are byte-identical to I82's `d1_c1`, `d1_c2` and `d1_c3`, and the evaluator gives I82's E+R exactly (`sweep.log`). **S3 is reproduced: 9,747,725,678 / 9,688,594,334 B.**

## 2. The method: I82's chain with the D1.3 rules rebound

`b3s_exact_chain.py` copies I82's `mc_chain` and changes only rules, plus three mechanical patches (§2.5). Every removal and patch must match exactly as stated, or it fails. The full diffs are in `chain_diff/`, and each variant's rule log is in `rebinds/`.

### 2.1 The D1.3 zero rules, rebound

Multiplicities are function multiplicities M in erc at c = 1 / c = 3, for branch W (the one that binds) and branch X.

| # | Rule (I65's) | Rebound to | Multiplicity on the exact route |
|---|---|---|---|
| 1 | fn_zero `append_exact_pressure_results` ("D1.3 has no exact pressure case") | **Kept at 0, restated:** its only call (PP `lib.rs:5373-5389`) needs a pipe state, and `build_pressure_case_with_members` makes one only per region member. Every region list is empty | 0 |
| 2 | file_zero `source_receipt/composite.rs` | **Removed.** `composite_support_norms` and `composite_member_maximum` are made X-only (row 13) | W: `ordinary_physics` 1 / 3. X: `composite_support_norms` 32 / 96, `composite_member_maximum` 64 / 192, `composite_exact` 1 / 3, `validate_publication` 1 / 1, `physical_source_built` 3 / 9. composite.rs TEXT: W 4.3 / 39.0 MB, X 34.2 / 244.3 MB |
| 3 | file_zero `pressure_material.rs` | **Removed:** `resolve_base` (`lib.rs:2434`) no longer returns early, and `resolve_case` (`lib.rs:9634`) runs under `is_exact` | W: `resolve_base` 1 / 1, `resolve_case` 1 / 3, `failure` 222 / 602. X: 3 / 7, 3 / 9, 648 / 1,880. TEXT: W 5.8 / 15.6 MB, X 16.8 / 48.7 MB |
| 4 | edge_zero `build_pressure_case_with_members` → 51 callees | **Removed.** Callees inside the region loop take that loop's 0 (row 9) | `build_pressure_case_with_members`: W 1 / 3 (one per case), X 7 / 21 |
| 5 | edge_zero `validate_profile` → `check_suffixes` | **Removed.** The exact arm calls it on the pipe, node, support and case id lists | `check_suffixes`: W 8 / 16, X 36 / 100 |
| 6 | edge_per_call `validate_profile` → `problem` = L | **Kept at L, restated** for the exact arm: only `EXACT_PRESSURE_REQUIRES_REGION` can fire, at most once per load whose category or dimension is pressure (`pressure_runtime.rs:186-191`) | L per call. `validate_profile`: W 2 / 4, X 9 / 25 |
| 7 | edge_zero `for_source` → `validate_physics_evidence` and → `forbid_load_reference_evidence` | **Removed:** physics-retained-1's G7 runs physics-1's base validator, the `PHYSICS_ID` arm (`semantic_contract.rs:440-443`) | W: `validate_physics_evidence` 1, `physics_source::validate_maximum` 32 / 96. TEXT: physics_evidence.rs 0.15 / 0.45 MB; physics_source.rs 89.3 / 267.9 MB (no error-path credit taken; §8) |
| 8 | loop `exact\.assembled_operands` = 0 | **Kept at 0, restated:** operands are pushed only inside the region traversal | 0 |
| 9 | loops `traversal\|terminals\|…groups` and `pressure\|region\|aggregates` = 0 | **Kept at 0, restated** under D1.5-exact: every loop they match iterates regions, region members or terminals, `pipe_states`, or pressure-thrust aggregates (components, D1.4) | 0 |
| 10 | the other `for_source` and `for_source_metadata` zero edges (load-reference, physics-source-1, source-blocks-1, `is_retained`) | **Unchanged** | 0 |
| 11 | fn_zero `resolve_shared_sections`, case_state's file_zero and fn_zero, edge_once `…_captured_once`, loops `state\.load_sources` and `state\.members` | **Unchanged:** the exact route keeps sections absent and 0.4.0 excluded | 0 |
| 12 | (new) edge_zero `composite::physical_source` → `captured_load_state_case` | Added: that branch runs only under `is_load_state` (`composite.rs:415-417`), and D1.3's load-state clause is kept | 0 |
| 13 | (new) branch W excludes `composite_support_norms`, `composite_member_maximum` and `composite_exact` | Added: each runs only with a selected source (`lib.rs:4748-4750`, `:5268-5270`, `:5490-5496`). Under T-3 (c), a selected case publishes the exact ordinary bytes, so W1 never reaches them | W: 0 |
| 14 | (new) edge_per_call `check_suffixes` → `problem` = max(n, m, g, c) | Added: the loop's real collection (at most one collision per id), in place of the generic "entity id iterator" rule's max(l, L) | ≤ 32 per call |

**The new loop rules.** 32 are appended after every existing rule, so they apply only to headers that nothing else matched; every header they cover had been unmapped.
- **physics_evidence.rs** (G7's physics-1 validator): diagnostics and evidence references D; cases c; sections m; rows Pall; supports g; id lists Pall; vectors 6n; GEOMETRY_KEYS 9.
  - The assembly `groups` loop is 0 (no region).
  - The region-nested loops (members, applied loads, terminals, components 6, stations 5) carry multiplicity 0.
- **composite.rs** (X): rows Pall; members m; support magnitudes 2; STATIONS 5; cases c; `[f64; 3]` 3.
- **physics_source.rs** (X): endpoints 2; functionals per endpoint 3.

**The error-path rule** (text_args `site_from`): `physics_evidence.rs:38`, `:48`, `:53` and `:57` are allocated only on failure, so at most once per `validate_physics_evidence` call. This is I65's G4 rule for the preview validator (`preview_physics_evidence.rs:90`, `:100`). The only `.ok()` that swallows such an Err (`:755`) is inside the region loop.

### 2.2 How the problem() sites are priced

Each `pressure_runtime::problem` call costs about 44.6 KB of requested text in I65's chain:
- the diagnostic, 10,240 B;
- the `refs.join` under the join rule, at (g + m)·130 B;
- the id format, about 2.3 KB;
- four reference copies.

These are I65's rules, kept unchanged. So `problem`'s multiplicity, and the D it feeds, drive most of the exact route's TEXT. M(`problem`) in erc is: W 520 / 2,072; X 2,360 / 12,968. In er (W) it is 3,233 at c = 3.

### 2.3 The two D1.5-exact credits (`--credits`; erc and urc)

| # | Rule | Reason |
|---|---|---|
| C-1 | edge_per_call `build_pressure_case_with_members` → `problem` = mats | Of its `problem()` sites only `EXACT_PRESSURE_MATERIAL_AMBIGUOUS` can fire, once per selected material (`pressure_runtime.rs:461-468`). The rest cannot:<br>• `pressure_regions` is `Some`, so `REGIONS_REQUIRED` never fires;<br>• the region loop is empty;<br>• every cap and eigen term list is empty, so `exact_sum` returns `Ok(+0.0)` (`pressure_sum.rs:16-22`) and the per-DOF `_ => problem` (`:748-761`) never runs;<br>• every vector is +0.0, so the nonfinite check (`:764-775`) never fires |
| C-2 | edge_zero `finish_source_groups` → `problem` | `groups` is empty, so its two in-loop sites never run. Every `by_dof` list is empty, so the per-DOF `Err(_)` never runs. max_rhs = 0 with every list empty gives screen = 0, so the cancellation site never runs (`:845-907`) |

They remove the per-DOF sites that I65's method would price at N = 192 per call. They are sound by code reading. They are new zero rules, so they need review as RV83 and RV84 reviewed G4's.

### 2.4 The route-exclusive forms (`--exact-only`; erc and er)

With `is_exact` true, these legacy branches cannot run. The edge_zero rules:

| Caller → callee | Why |
|---|---|
| `solve_load_case_observed` → `source_receipt::exact` | A selected exact case takes `composite_exact` (`lib.rs:5493-5496`) |
| `solve_load_case_observed` → `FinalizedSourceBlockCase::ordinary` | An unselected exact case takes `ordinary_physics` (`:5505-5508`) |
| `run_linear_static_preview_observed` → `finalize` | `composite` = source_selected && is_exact (`:2838`), so the call goes to `finalize_composite` (`:2890-2891`) |
| `run_linear_static_preview_observed` → `preview_physics::render`, `::evidence`; `solve_load_case_observed` → `::evidence` | The preview exists only when !is_exact (`:2711`, `:2878-2881`, `:4673-4674`) |
| `for_source` → `validate_preview_physics_evidence` | G7 takes the `PHYSICS_ID` arm |

Plus fn_zero `captured_load_state_case` and `check_load_state_input`: the 0.4.0 replays, and 0.3.0 cannot be a load-state document.

**Without these** (ur, urc), the chain sums both routes' selected finalizations, the 0.4.0 replays and both base validators into one invocation. So X1 binds at 13.8–14.9 GB at C = 3.

### 2.5 Mechanical patches

- `sens.py` evaluates every TEXT variant even after an incomplete one. I65's `complete and tb(..)` stopped at the first.
- `t08_closure.py` accepts a run whose **only** open items are `id-unaudited` (§8).
- The census substitution (§3).

No arithmetic, law, atom or evaluator changed. I82's `b1_eval.py` and the law record are used as committed.

## 3. The census: an exact-route fixture at D1's caps

From `fixtures/product_preview/physics_source/` (`b3s_census.py`; `b3s_census.out.json`):
- **Rows.** n05, n05_units, n05_unicode, n06 and fields (2 nodes, 1 pipe, 2 supports, empty regions) publish 81 rows per case sparse and 81 or 82 dense. That is 7n + 51m + 8g, plus at most one dense-parity row (`sparse_live_path_dense_parity_relative_delta`) in dense. In mixed, the empty-region case is the same; its region case (110/111) is outside B3b.
  - So the chain's P_final = 7n + 51m + 8g + 3 bounds the exact route, and **P is unchanged**.
- **Contract evidence.** On this route, `contract_evidence` is `{connector [], exact_cases, pressure []}` (`lib.rs:2841`) and no preview tree is built (`:4673`).
  - I measured one fields case. Per member: `pipe_materials`, `pipe_sections`, `pipe_stress_extrema` and an unavailable-pipe id. Per DOF: the three 6n RHS vectors. Per node: `node_order`. The rest is per case.
  - Every string is taken at max(its length, 300 B): identifiers are ≤ 128 B, and the elastic-maximum result id is < 300 B.
  - **At n = m = 32**, per case: 1,900 array slots, 419 objects, 2,073 entries, 226,200 string bytes, 21,561 key bytes and 2,275 numbers. The preview tree has 160, 67, 553, 66,816, 22,120 and 320.
- The chain's four per-case preview-tree forms (`ordinary_caps` PREVIEW, `g4_caps` ENV and PREVIEW_T, `t25_g4` env_vf) take the **componentwise maximum** of the two trees as functions of (n, m, g). No credit is taken for the absent preview tree.
- **Its price at C = 3:** +19,273,560 B in both modes.

## 4. Results in detail

**At C = 3** (`b3s_report.out.json`):

| | S3 | **erc** | er |
|---|---|---|---|
| E+R dense / sparse | 9,747,725,678 / 9,688,594,334 | **9,900,151,888 / 9,841,020,544** | 10,427,541,576 / 10,368,410,232 |
| Binding phase | W3 | W3 (X1 9,867,916,042 / 9,808,784,698) | X1 (W3 10,169,268,460) |
| Smallest M (dense) | 10,830,806,309 | 11,000,168,765 | 11,586,157,307 |
| **5 % M** | 10.5 GiB | **10.5 GiB** | 11.25 GiB |
| Margin and budget at 10.5 GiB (dense) | 399,134,558 B, 9.53 % | **246,708,348 B, 5.95 %** (sparse 7.37 %) | −280,681,340 B |
| At 11 GiB (dense) | 21.06 % | 17.59 % | 3.95 % |
| At 12 GiB (dense) | 44.12 % | 40.88 % | 22.80 % |
| TAV_W / TAV_X | 4,189,696,338 / 3,946,452,254 | 4,149,552,618 / 4,738,629,798 | 4,217,048,514 / 5,125,816,332 |
| D / D_env | 41,412 / 22,911 | 51,570 / 23,895 | 59,697 / 25,056 |

The 5 % rule holds for every phase of erc at 10.5 GiB, not only the binding one; `five_pct_M_every_phase` is 10.5 GiB.

**W3's terms, dense** (erc minus S3; they sum to E+R):

| Term | S3 | erc | Δ |
|---|---|---|---|
| TAV_W | 4,189,696,338 | 4,149,552,618 | −40,143,720 |
| T16 (P2: body `json!` + hash(publication)) | 3,589,009,045 | 3,741,259,650 | +152,250,605 |
| O_base_dense | 812,438,740 | 816,626,020 | +4,187,280 |
| T12_T15 | 323,428,560 | 323,428,560 | 0 |
| STAGED | 254,737,894 | 267,820,013 | +13,082,119 |
| STATICS + T11 + T19 + NOTICE | 5,850,618 | 5,850,618 | 0 |
| T16_moving | 505,455,619 | 528,505,545 | +23,049,926 |
| R | 67,108,864 | 67,108,864 | 0 |

- **T16 grows** with the envelope's diagnostics (D_env +984 at C = 3) and the evidence tree.
- **TAV_W falls** because the preview render and preview validator leave the W graph (§2.4), more than offsetting the pressure_runtime, pressure_material, physics validator and composite sites that join it.
- **Next behind W3:** X1 by 32 MB. Its TAV_X grows by 792 MB: the composite publication replays each case's physical build (`normalized_case` → `validate_profile`, unit normalization, `build_model_for_members`, `build_pressure_case`).

## 5. Comparison with S3, and C_eq = 3

- **The exact route at C = 3 costs 1.6 % more than S3** (+152.4 MB) and keeps S3's 10.5 GiB with a 5.95 % budget, against S3's 9.53 %.
- **B1's G5 headroom statement changes.** S3 alone absorbs up to 44 % of TAV_W within 12 GiB; with the exact route it is 40.9 %.
- **At C_eq = 3 over both routes,** the maximum is the exact route's 9,900,151,888 B. I93's legacy combination price (HIGH) is 9,749,149,984 B. One 10.5 GiB registration covers both, at 5.95 %.

## 6. Decision 26

**The exact route fits at M ≤ 12 GiB.**
- **With route-exclusive forms and the two credits (erc):** it fits **at B1's expected 10.5 GiB, with 5.95 %**. No route caps are needed.
- **Without the credits (er):** it needs **11.25 GiB**, which is within ROOT's 12 GiB. Route caps are then optional, priced:

  | Exact-route cap (er unless stated) | Dense E+R | 5 % M |
  |---|---|---|
  | c ≤ 3 (none) | 10,427,541,576 | 11.25 GiB |
  | c ≤ 2 | 6,909,912,301 | 7.5 GiB, so B1's 10.5 GiB holds with 99 % |
  | c ≤ 2 (erc) | 6,688,918,758 | 7.25 GiB |

- **Without route-exclusive forms (ur, urc):** it does not fit within 12 GiB at C = 3 (14.75–16.0 GiB). An exact-route cap of c ≤ 2 would not help either, because a single union form set prices every invocation, legacy ones included, at the union's price. So the route split, not a cap, is the remedy.

## 7. For ROOT to rule

1. **Route-exclusive pricing for B3b's G5.** SQ2's TEXT runs per route, with the mirror rules of §2.4 reviewed on the frozen code, and `cap_priced_maximum` takes the maximum over the route form sets (PLAN §3.2 item 1).
   - This is a structural requirement: without it C = 3 needs 14.75–16.0 GiB.
   - It also adds a profile dimension: route-specific forms, as I82's tiers would have.
2. **The two D1.5-exact credits (C-1, C-2),** for review with the mirror rules, RV83/RV84-style. They decide 10.5 GiB (erc) against 11.25 GiB (er).
3. **M.** With 1 and 2, B1's 10.5 GiB holds for B3b. Without 2, 11.25 GiB, within ROOT's authority. No owner decision is needed in either case.
4. **D1 on the exact route refuses z > 0** (B3-D, B3b-A), so the exact forms never price combination text. The ordinary route already blocks such combinations, and this keeps the W1 domain aligned with it.
5. **The identifier audit** for the 32 newly reached identifier placeholders (`b3s_open_items.out.json`; 14 in W) is G5's work. They are priced here at their argument rules' class values.

## 8. Limits: what is emulated, and what G5 on the real code replaces

- **B3b's code does not exist.** I priced today's call graph with B3b's route assumed.
  - The W1 producer's exact capture and preparation, B3-D's definition, the `<physics-retained>` reader branch and G8's `actual_materials` are not in the graph.
  - The retained W1 path is priced as today's (preview-route) producer and receipt.
  - The exact precommit is priced as today's physics-1 base validator.
  - G5 on the frozen B3b code replaces all of this.
- **The multi-case scaling is I82's emulation** (STUDY §2.4's limits).
- **No liveness or sharing credit was taken** (I65's method), and P is the legacy bound.
- **The census is one fixture's structure scaled to D1's caps,** with strings at 300 B. The extrema evidence, present only for a selected source, is counted on every case.
- **I took no credit for physics_source.rs's error-path sites in W** (89 / 268 MB at c = 1 / 3). Like physics_evidence's, they allocate only on failure, so G5 could credit them, after checking that no caller swallows an Err. Taking it would lower erc's W3, not raise it.
- **The D fixpoint is I65's:** one D from the full graph bounds the diagnostic loops of every branch. X-branch diagnostics therefore raise W's preview and integrity loops. This is not changed here.
- **The 32 id-unaudited placeholders** keep `text_complete` false in every exact variant. There are no unmapped loops and no other unclassified argument.
- **The X branch is emulated as T25 plus TEXT.** The composite finalization's typed owners (T25's forms) are the legacy source-block ones, with the evidence tree maxed. X1 is 32 MB below W3, so a G5 X1 above W3 would bind.
- **No machine claim** (D-7's non-claims). The measured RSS and time for the exact route's cap-maximal input stay SQ2's (PLAN §3.1).

## 9. Execution record, read list and records

**Execution.**
- I95, TASK, no descendants; 2026-10-07.
- Python 3.13 from VENV.
- Step 0 took about 20 s; one point 15–70 s; the final sweep (15 points) 3 min 47 s, plus one attribution point.
- Writes went to this folder and `WT/scratch/i95_b3_s/` only. The scratch (`snap/` 83 MB, the chain copies, `runs/`) is disposable, and `_run_records/` recreates it (`RUN.md`).

**Read** (sha256):

| Input | sha256 |
|---|---|
| The brief | `9968db1d…8751` |
| `NUM/AGENTS.md`, `NUM/agents/AGENT_TASK.md` | — |
| I93 PLAN.md (§0, §1.4, §3, §5), REVISION_01.md | `e1147dbd…1238a`, `63abb73f…bba0c` |
| I82 STUDY.md, ADDENDUM_01.md | `d8b18220…7188`, `7c155ceb…0ce2` |
| I82 `b1_eval.py`, `b1_mc_chain.py`, `b1_mc_run.sh`, `b1_text_base.sh`, `b1_snapshot.sh` | `c404e8db…c74b`, `a885ff93…719d`, `b8a2d43c…fed6`, `63375d94…4e40`, `896c7d5f…b171` |
| I82 `profile_trees/d1_c1.json`, `d1_c3.json` | `148cc1c8…f82f`, `0a1d5257…4d35` |
| I72's law record | `cbf34c52…313d` |
| I65 u4_g7_06 `chain/` (per-file sum list), `g7_linemap.py`, `text_row_diff.py`; RETURN.md | `8761920d…470a`, `993a2306…ce42`, `73e20e0a…bb3d` |
| I65 DOMAIN.md (§1–§2) | — |
| RR (`2cc4819c…63cd`): "Owner decision: M's practical limit is 12 GiB…", "I82's addendum…", "B2/B3 R1…", "I93's REVISION_01 accepted…", "SP returned before I3…" | — |
| Fixtures: `physics_source/fields.request.json`, `fields-sparse_interactive.raw.json`, `n05`, `n06`, `mixed` requests, and every physics_source raw output | `7f8ff9d5…6002`, `820ed6df…e244`, `332319ee…4b00`, `5551f164…e931`, `a6236084…4abf` |

**Code read at `2007709549`:**
- PP `pressure_runtime.rs` (`is_exact`, `validate_profile`, `check_suffixes`, `build_pressure_case_with_members`, `finish_source_groups`), `pressure_sum.rs`;
- PP `lib.rs`: the `is_exact` sites, `solve_load_case_observed` (exact pressure, preview record, source finalization), `append_exact_pressure_results`, the envelope assembly and finalize choice, `append_modulus_basis_record`;
- PP `source_receipt.rs` (`finalize*`, `check_input*`, `captured_load_state_case`), `source_receipt/composite.rs` (entries, `physical_source`, `ordinary_physics`);
- RE `semantic_contract.rs` (`for_source`), `physics_evidence.rs` (helpers, validator loops), `physics_source.rs` (the maximum checks), `preview_physics_evidence.rs` (helpers).

**Records** (`_run_records/`, placeholder paths only):

| File | What |
|---|---|
| `b3s_snapshot.sh`, `b3s_tree_check.sh` (+ `.out.txt`), `b3s_text_base.sh`, `base_reproduction.txt` | The basis and step 0 |
| `b3s_exact_chain.py`, `b3s_build_chains.sh`, `chain_diff/`, `rebinds/` | The rebinding: the patcher, the four variants (and `erc_nocensus`), and each one's full diff and rule log |
| `b3s_census.py`, `b3s_census.out.json` | §3 |
| `b3s_run.sh`, `b3s_sweep.sh`, `sweep.log`, `profile_trees/` | Every point and its profile tree |
| `b3s_report.py`, `b3s_report.out.json` | §0, §4–§6 tables, the c = 1 tree change, the attribution |
| `b3s_rules_mult.py`, `b3s_rules_mult.out.json` | §2's multiplicities and per-file TEXT |
| `b3s_open_items.py`, `b3s_open_items.out.json` | The identifier-audit items per run |
| `b3s_text_delta.py` | The per-function delta tool used while building the rules |
| `RUN.md` | The commands |
