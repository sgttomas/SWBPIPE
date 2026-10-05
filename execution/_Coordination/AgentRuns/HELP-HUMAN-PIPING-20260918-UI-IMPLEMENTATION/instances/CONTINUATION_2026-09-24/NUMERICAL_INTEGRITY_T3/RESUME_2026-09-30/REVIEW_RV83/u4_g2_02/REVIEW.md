# RV83 confirmation: are the G2 findings repaired in I65's G3?

**Candidate:** `R/I65/u4_g3_01/` at NUM `3260d7809e`. SHA256SUMS verifies 56/56.

**Basis:** G3 cites source at `5ae5fe4f0f`, so I read source at that revision (`git show`, read-only). Since then, the U1 merge has changed PP `lib.rs` (observer hooks only), `retained_product.rs`, the new `retained_wire.rs` and FK `final_case.rs`. Per the ruling, those belong to G4. None of the files behind my findings changed apart from `lib.rs`.

**Scope:** whether each G2 finding of mine (`R/REVIEW_RV83/u4_g2_01/REVIEW.md`) is genuinely repaired. RV84 re-derives G3's numbers and composition; I do not repeat that.

**Read:**
- RR "RV83 on U4 G2: FAIL; dispositions…" and "U4 G3 verified; D1.10 and D1.11 adopted; the margin rule";
- G2_AMENDMENTS.md, RESIDUALS_G3.md (T07, T22, T25), TEXT.md and ORDINARY.md;
- the run records each depends on.

**Abbreviations:** as in my G2 report. RX = R/REVIEW_RV83/u4_g2_02/_run_records; G3R = R/I65/u4_g3_01/_run_records.

## Verdict: NOT CONFIRMED

**Status of the nine G2 findings:**
- 7 fixed;
- 1 superseded by ruling and implemented as ruled (S-4);
- **1 not fixed: B-2,** with two narrow residuals:
  - R-1: the call graph drops two classes of method call. FKS/formation_check.rs is therefore unreached, though its text is priced elsewhere;
  - R-2: an exclusion argument for pressure_runtime.rs is false.

  Neither changes a composed figure materially. Both remedies are mechanical.

**New findings in this pass:** 0 BLOCKING, 3 SHOULD-FIX (R-1, R-2, R-3) and 3 NOTE.

## Per-finding status

| G2 finding | Status | Evidence (my own probes) |
|---|---|---|
| **B-1:** exact-selected finalization unpriced | **Fixed** | T25 (RESIDUALS_G3.md:87–136) prices the chain I flagged, and I checked its roster against source at the basis:<br>– **per case:** `check_input` → `check_input_with_physical` (source_receipt.rs:270–366). That covers `requested()`, O-N, `build_model`, dense stiffness, loads, application, force and prescribed/free, plus `replay_against` (Sources, Context, Response and values #2). Then `check_binding_against`, `rows::bind`, `source::commitment` and the case outputs (:636–700);<br>– **per invocation:** `finalize_for` (:869–1060): `serialized(envelope)`, `requested()` ×2, the two assessed-quality Values, the ids and accounted sets, per-case actual-rows clones and serializations, the wire, the body `json!` with `hash(publication)`, `hash(body)` and `into_wire`.<br>T07's "Remaining: none" and DOMAIN.md:3's claim are withdrawn. The numbers and the self-declared S4 residual are RV84's. |
| **B-2:** T08 inventory incomplete | **Not fixed** (residuals R-1 and R-2) | **The method is fixed.** Sites are now taken over the D1 call graph (1,881 functions, 2,563 sites), and the lexicon covers `to_string` (561 rows), `into`, text `clone`, `join`, `push_str`, `replace`, `to_owned` and the `Diagnostic {` literals.<br>**My file list against TEXT.md:**<br>– validation.rs: 263 rows reached, 76 with positive multiplicity, 76.1 MB;<br>– source_receipt/source.rs: 19 positive; rows.rs: 31 positive;<br>– the units Display: 10 positive, with UnitError ≤ 611 B;<br>– load_ledger.rs: 6 positive; exact_sum.rs: 3 positive;<br>– case_state/resolve.rs: excluded correctly. `validate_document` returns without text when `carries` is false (resolve.rs:66–90);<br>– pressure_material.rs: excluded correctly. `resolve_base` returns at :66–68 when `!is_exact`;<br>– self_weight.rs: excluded correctly under D1.10 (§D1.10 below).<br>**Two listed files fail the criterion:**<br>– **FKS/formation_check.rs** is unreached: 6 rows, multiplicity 0 (R-1). Its detail String is priced only incidentally, through T05's `Text(formation_detail)` (G3R/ordinary_caps.py:199, :238);<br>– **pressure_runtime.rs**'s `validate_profile` is excluded on a false call-path argument (R-2). |
| **B-3:** newline identity through one rustc-env | **Fixed** (design) | G2_AMENDMENTS §1 specifies:<br>– one `v1;key=value;…` line with `%`-escaping (`%`, `;`, `=`, and every byte < 0x21 or ≥ 0x7F), so it is printable ASCII with no separator ambiguity;<br>– `v1;unavailable` on any read failure;<br>– byte-exact comparison, with `Registered` only when the value is present, equal and the witnesses hold, and `Stale` otherwise;<br>– one encoder shared by `build.rs`, the crate and the G6 registration texts;<br>– three G5 tests: keys in order, a 256-byte round trip, and absent → `Stale`. |
| **S-1:** T07 formula unsound | **Fixed** | G3R/t07_repair.caps.out.json has all the missing rows:<br>– C1 and H1 carry both `Snapshot.identity` copies (constant 2,123,045 includes the 1,797,413 B identity);<br>– A4 prices the identity at J3 capacity (3,594,826);<br>– A3 prices descriptors #1 by their construction law: 41,760 units at the caps, outer capacity 3,072, 1.88 MB at ASSUMED strides against my independent 1.73 MB estimate;<br>– D2 prices the Response temporaries;<br>– E1 and H3 price #2 and #3 by the 16,384-unit law, so exactly three generations are live;<br>– the length constant is +5.<br>The owners G2 omitted are now rows: `Sources.assembly`, `force_terms`, the prepare_sources locals and the caller's `to_dense`. |
| **S-2:** T22 not closed | **Fixed** | RESIDUALS_G3 §T22:<br>– enumerates I34 DESIGN.md:255–265 site by site;<br>– replaces the `ceil_sqrt` and `2·ceil_sqrt` rows with a total-function lemma, which I checked: `(n as f64).sqrt() as u64` saturates, the u128 squares cannot overflow, r ≤ 2^32 and 2r ≤ 2^33;<br>– relabels source.rs:356–358 as free·(free+1) = 37,056;<br>– shows `held−before+after` checked, "tracker capacity" (adaptive.rs:872–876, confirmed).<br>I reproduce the count of 60 `CountRange(` tokens, 36 of them labelled, under the "before the first cfg(test) mod" convention. My G2 figure of 62 included source.rs's test module. The D-4 citation is re-pointed by ruling. |
| **S-3:** no typed capacity caps | **Fixed** | G2_AMENDMENTS §2 caps every typed owner in my G2 roster:<br>– Strings ≤ 128;<br>– nodes, pipes and supports ≤ 32;<br>– restraints ≤ 192 each and in total;<br>– materials ≤ 4; points ≤ 16; cases ≤ 1; loads ≤ 192;<br>– the length-0 vectors at capacity 0;<br>– expansion-law vector ≤ 4, all elements `Absent`;<br>– `project.units` facts within the raw caps.<br>The census reads actual capacities. The depth convention (N-12) is stated. |
| **S-4:** sections and normalization | **Superseded by ruling** ("both"), and implemented | D1.3 now requires zero sections and `section_ref` None everywhere (G2_AMENDMENTS §3). O-N (ORDINARY.md; G3R/ordinary_caps.py:103–107) prices one quantity's transients at a time: the `format!` id, the refs Vec and the replacement unit String, which fits T_resident's 128-byte per-String cap. The no-section `resolve_shared_sections` emits no text. New residual R-3: the provenance-parse transient is not in O-N. |
| **S-5:** `env!` compile error | **Fixed** | `option_env!`, with `None` → `Stale` (G2_AMENDMENTS §1), plus the G5 test `absent_identity_is_stale`. |
| **S-6:** spelling rows and composite Debug | **Fixed** | TEXT.md §3 and G3R/text_args.json:<br>– `ident_debug` 770 (6·len+2);<br>– `{:032x}` 32;<br>– SHA-256 `{:x}` 64;<br>– `error_display` and `error_debug` 8,192 each.<br>Every composite-Debug site I named now carries a size: lib.rs:3756 16,456; :2809 and :5003 16,411; :4635 and :4651 812; :1143 175; :6004 38,032; :6619 8,302; :9791 8,192. |

**NOTE dispositions** (G2_AMENDMENTS §7) are accepted. N-1 now names wide/multi.rs (§4).

## New findings

| ID | Sev. | Where | Evidence | Remedy |
|---|---|---|---|---|
| R-1 | SHOULD-FIX | G3R/callgraph.py:353–363 (method resolution) and :41 (an unused `METHOD` regex); TEXT.md:46–54 | **The call graph is not an over-approximation for two classes of method call:**<br>(a) **A receiver whose parameter has a bare generic type,** such as `equations: &R` or `prepared: &P` in `finish_checked_factor` (FKS/structural.rs:1687–1791), resolves to a "type" named `R` that owns no methods. The edge is dropped instead of fanning out.<br>(b) **A chained call** such as `f(x).g(`, `x?.g(` or `x.unwrap().g(` matches neither `CALL` (its lookbehind excludes `.`) nor the receiver regex, so it adds no edge. The `METHOD` regex that would catch it is defined but never used.<br>**My probes, read-only over G3's own `callgraph_edges.json`:**<br>– RX/rv83_missed_edges.py: 24 generic-receiver calls hide 64 functions;<br>– RX/rv83_chained_calls.py: 21 chained-call names hide 44 functions.<br>**The text consequence is small.** The only hidden D1 text rows are FKS/formation_check.rs's 5 `format!` sites, reached through `equations.formation_check` at structural.rs:1761. A further 3 are in result_export, which is G4's. The 15 dropped calls whose targets are reachable by other paths carry no text below them, so no text multiplicity is undercounted. T05 already prices the formation detail String, as `Text(formation_detail)`, once and twice more in the W2 tail.<br>**The structural consequence is not small.** TEXT.md:54's "never under-counts" is false. STACK_INVENTORY's "18 self-recursive, no mutual recursion" was computed over the same graph, which misses these 108 functions. | Fan out on generic and unknown receivers; use `METHOD` for chained calls. Re-run TEXT and the STACK inventory, then compare. Route to RV84, which samples the graph, and to I65 at G4. |
| R-2 | SHOULD-FIX | G3R/loop_bounds.json `fn_zero` "pressure_runtime.rs:111:validate_profile" (and `build_pressure_case_with_members`, which relies on it); TEXT.md:75 | **The exclusion says** the only text site reachable in D1 is guarded by `pressure_contract.is_some()`.<br>**At the basis that is false.** The non-exact arm of the case loop (pressure_runtime.rs:207–225) calls `problem(…, "PRESSURE_MODEL_REAUTHOR_REQUIRED", …)` for every load whose `category == "pressure"` and whose magnitude is nonzero. D1.7 constrains the dimension but leaves the category free, so a nodal force with category "pressure" is in D1 and reaches it. Each firing builds a joined refs String, a `format!` id, `diag`'s copies, a refs Vec and a `source` String: about 2 KB per load, about 0.4 MB at l = 192.<br>The diagnostic is blocking, and `run_linear_static_preview_observed` returns `blocked_envelope` at the first `has_blocking` check (lib.rs:2316–2317 at the basis), before any later phase. The prefix's text is therefore far below TAV. But the stated call-path argument is wrong, and these sites have multiplicity 0. | Either price the 3–4 sites with multiplicity l (drop the exclusion), or add `category != "pressure"` to D1.7. That is ROOT's choice; it is a refusal under `source_family`. |
| R-3 | SHOULD-FIX | G2_AMENDMENTS.md:115; RR "U4 G3 verified" (D1.10: "its per-load parse attempt stays priced in O-N"); G3R/ordinary_caps.py:103–107 | **O-N has no term for the per-load provenance parse.** Its formula (7·s(String) + 1,496) covers only the normalization transients.<br>**The transient is also not "a serde_json::Error of about 40 B".** A provenance that is valid JSON but not an object is admitted by D1.10 and parses to a `Value` (self_weight.rs:796–799), which lives to the end of that load's iteration. Examples are `[{"":0},{"":0},…]`, `[[[[0]]]]` and `"…"`. From ≤ 128 bytes that tree is bounded by about 26 BTree nodes (≤ 16.7 KB at Leaf_up 640), plus array backings ≤ 4·arrays + 2·elements slots (≤ 12.2 KB at s(Value) 32), plus ≤ 256 B of text. That is under about 30 KB, one at a time. | Add a stated parse-transient term to O-N, for example `R_raw` evaluated at a 128-byte document plus `ErrorImpl`, and correct the "about 40 B" text. |
| R-N1 | NOTE | G2_AMENDMENTS.md:40–45 | **The build script `include!`s `src/build_identity.rs` but declares only `rerun-if-changed=build.rs`.** Cargo should rebuild and rerun the script through its dep-info, but stating the file is cheap. | Add `cargo:rerun-if-changed=src/build_identity.rs`. |
| R-N2 | NOTE | G2_AMENDMENTS.md:45 | **"Cannot read … any key" needs one clarification:** an empty value is a valid value, not a read failure. `CARGO_CFG_TARGET_ENV` is empty on aarch64-apple-darwin. | State that "unavailable" means the variable is absent; "empty" is a value. |
| R-N3 | NOTE | PP/lib.rs:1757 (basis), `range_scaling_evidence_line` | `.collect::<String>()` builds a String from the `NAMED` `format!` pieces. That form is not in the lexicon. Its pieces are counted, and the final `format!` repeats the content, so the bound is not at risk. | Add `collect::<String>` to the lexicon for completeness. |

## D1.10: is the self-weight module unreachable?

**Yes, at the basis.**
- `validate_applied_self_weight` (lib.rs:2320) is the module's only entry on the Direct path. I searched every non-test PP source file.
- It calls `inspect_applied_self_weight`, which for each load:
  - parses the provenance with `serde_json::from_str::<Value>(p).ok()` (self_weight.rs:796–799);
  - reads `p["method"].as_str()`, which is `None` for any non-object `Value`;
  - computes `deterministic_identity`, which is false for a Node target (D1.7);
  - reaches `continue` (:825), so every function after it is unreached.
- D1.10's test, "the first byte after ASCII whitespace is not `{`", is at least as strict as serde_json's own whitespace rule (space, tab, LF, CR). Rust's `is_ascii_whitespace` additionally skips form feed, which can only refuse more. A non-object top-level value can never yield a method.
- **The residual is cost, not reachability:** the parse transient (R-3).

## Execution record

- **Who and when:** RV83, TASK (Type 2) under ROOT, with no descendants. Started 00:00 MDT and finished within the 2-hour box.
- **Memory guard:** PID 5387, running at the start.
- **Not run:** Cargo, rustc, solver, native and DEC-025 jobs. Git use was reads only (`rev-parse`, `log`, `status`, `diff --stat`, `show`), with `GIT_OPTIONAL_LOCKS=0`.
- **Run:** stdlib Python over G3's committed records and over source read with `git show`.
- **Writes:** RX/ and this REVIEW.md, plus WT/scratch/rv83_u4_g2_01/. Nothing went to the system temp directory.
- **One slip,** corrected at once. A malformed shell redirect created an empty file, `_probe_placeholder`, inside my sealed `REVIEW_RV83/u4_g2_01/`. I deleted it within the same minute. That packet's SHA256SUMS re-verifies 10/10, and Git shows it unchanged.
- **Run records** (RX/), all covered by SHA256SUMS:
  - `rv83_missed_edges.py` and its output: generic receivers, plus dropped calls to otherwise-reachable targets;
  - `rv83_chained_calls.py` and its output;
  - `ORIGINS.json`: sha256 of the G3 files relied on, and source blob ids at `5ae5fe4f0f`.
