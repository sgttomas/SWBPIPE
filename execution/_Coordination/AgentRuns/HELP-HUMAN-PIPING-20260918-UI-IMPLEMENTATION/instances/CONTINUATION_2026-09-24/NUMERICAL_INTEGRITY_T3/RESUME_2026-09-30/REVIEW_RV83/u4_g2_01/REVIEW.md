# RV83: independent re-derivation review of I65 U4 grant 2

**Candidate:** `R/I65/u4_g2_01/` at NUM `fe38ea55bc`. SHA256SUMS verifies 18/18. No maintained source, schema, fixture or app file changed between `dca3b65b3d` and `fe38ea55bc` (Git diff read), so the packet's citations at `a2c26cc885` apply at the reviewed revision.

**Reviewer:** RV83, TASK (Type 2) dispatched by ROOT under `BRIEFS/RV83_U4_G2_REVIEW.md`. I did not write the packet. I re-derived each item from source and used the author's scripts and outputs as leads only.

**Abbreviations:** as in the brief: P, PP = P/core/product_physics/src, FK = FKS/retained, FKS = P/core/solver/frame_kernel/src/structural, R, T3 and RR (= T3/ROOT_RULINGS_V1.md). FKL = P/core/solver/frame_kernel/src. RX = R/REVIEW_RV83/u4_g2_01/_run_records.

## Verdict: FAIL

**Findings:** 3 BLOCKING, 6 SHOULD-FIX and 13 NOTE.

Each BLOCKING finding sits under a claim that a ruling or G3 already relies on. Each has a bounded repair, and none reopens D-1, D-3, D-4b, D-6 = (a) or S-1. Most of the packet holds up under re-derivation:
- the T02 arithmetic;
- the derived counts at the caps;
- the BTree and hashbrown laws;
- the f64 spelling maxima;
- T06;
- the T03 roster;
- the D-4 quotations.

## Findings

| ID | Sev. | Where | Evidence | Remedy |
|---|---|---|---|---|
| B-1 | BLOCKING | R/I65/u4_g2_01/DOMAIN.md:3, :24; RESIDUALS.md:110, :138, :142 | An axis-aligned D1 case whose ordinary solve is `Sensitive` or fails enters source recovery (PP/lib.rs:3660–3722). If recovery succeeds, the case continues into SOURCE-BLOCKS-1 finalization. **Per case** (PP/lib.rs:4972 → source_receipt.rs:636–700), `check_input` → `check_input_with_physical` (source_receipt.rs:270–330) performs: `requested()`, which is `raw.clone()` plus `from_value` of the whole request (:136–139); `resolve_shared_sections`; `normalize_model_units`; `resolve_base`; `build_model`; `prepare_boundary`; dense `assemble_global_stiffness_with_user_elements`; primitive loads; application; and the force ledger. **Per invocation** (PP/lib.rs:2796–2798 → source_receipt.rs:869–1010), `requested()` runs at :875 (a whole typed request built only to test the schema version) and again at :965, and `serialized(envelope)` serializes the full envelope. None of these owners is priced. I54 BOUND's family case never selects (BOUND:97). T07 stops at the selected output and states "Remaining: none". DOMAIN.md's claim that D1 stays "inside the source paths I54/RV75 priced" is therefore false for this branch. | Add a named term for selected source-blocks finalization, with its owner roster at the caps, to G3, and point T07 at it. A field predicate cannot exclude these inputs, so do not try to remove them from D1. |
| B-2 | BLOCKING | RESIDUALS.md:148–156, :191; RR "U4 G2 verified" (G3 owes "all 707 sites") | My lexer (RX/rv83_text_sites.py) reproduces exactly 707 sites and the 909/56/10/4/2 placeholder split on I65's 14 files. **D1-reachable files with format or diag sites are missing from the inventory:**<br>– PP/validation.rs, 129 sites. `validate_model_inputs` is called unconditionally (PP/lib.rs:2296), and `validate_provenance` fires once per node, pipe, support, material, case and load;<br>– PP/pressure_runtime.rs, 16 (:2294);<br>– PP/case_state/resolve.rs, 14 (:2295);<br>– PP/self_weight.rs, 8 (:2320);<br>– PP/pressure_material.rs, 7 (:2339);<br>– PP/source_receipt/source.rs, 10. Its `failure_fields` (source_receipt.rs:771) is reached by the milestone's own failed recovery;<br>– PP/source_receipt/rows.rs, 7;<br>– P/core/units/src/lib.rs, 9: unit-error Display through `error.to_string()` in `normalize_quantity` (PP/lib.rs:8347–8384);<br>– FKS/formation_check.rs, 5. RESIDUALS:90 itself calls these sites T08 text;<br>– FKL/load_ledger.rs, 3 and FKL/exact_sum.rs, 3;<br>– canonical_json/src/lib.rs, 7;<br>– on the W1 side, PP/retained_product.rs (13) and FK/{source, wide, wide/multi}.rs (20).<br>**Altogether 365 sites** lie outside the inventory across the linked crates. Some are unreachable in D1: curved_bend, nonlinear_*, source_receipt/composite.rs and the bins.<br>**Non-template text owners are also outside a macro inventory:** PP/lib.rs alone has 416 `.to_string()` calls, plus the eagerly built `vec![id.clone(), ...]` refs. | Withdraw "the inventory is the complete list". G3 should select files from the D1 call graph and state that rule, extend the inventory to every reachable file, and count Display invocations (`to_string`) and literal text owners as text terms. ROOT should amend the G3 direction "for all 707 sites". |
| B-3 | BLOCKING | BUILD.md:34–55, :65–72 | The canonical identity is "newline-separated key=value lines", but it is emitted through a single `cargo:rustc-env=OPS_RETAINED_BUILD_IDENTITY=<text>` directive. Cargo parses build-script output line by line, and a rustc-env value cannot contain a newline. Only `rustc.release=…` reaches `env!`; the later lines are not directives and are dropped. Two outcomes follow:<br>– if G6 registers the full texts, every build is `Stale` and the permit is unreachable;<br>– if G6 registers the observed value, the check compares the rustc release only, so a mismatch in commit, target, opt-level, debug assertions or rustflags yields `Registered`. That is a path to a permit on a mismatch. | Use a single-line encoding with an escaping rule: for example, `;`-separated pairs with `%`-escaping of `;`, `=`, `%` and newline. Alternatively, emit one rustc-env per key and compare them jointly. Add a G5 test that the compiled value carries every key in order, and generate the registered texts with the same encoder. |
| S-1 | SHOULD-FIX | RESIDUALS.md:122–140; caps_arithmetic.py:111–150 | **The T07 formula is not a sound upper as written:**<br>(a) `Snapshot.identity` copies the identity JSON into the Context (FKS/exact_boundary.rs:466) and again into `RetainedResponse` (`self.context.source.clone()`, :1429). Neither copy is in a row: 2 × 1,797,412 B at the caps;<br>(b) the identity String comes from `serde_json::to_string`, which starts at 128 and doubles (J3; serde_json ser.rs:2217). Its capacity at the caps is therefore 2,097,152 B, not the length. The length constant is also +5, not +0 or +4;<br>(c) descriptors #1 are built in `prepare_sources` (PP/source_recovery.rs:1059–1240), before `descriptors_charge` (functionals.rs:427–476) runs inside `FunctionalPlan::new`, so the 16,384 cap does not bound #1. At the caps #1 counts about 41,152 units (802 per member, 4 per nodal descriptor, 4 per spring, about 14,592 for support rows). The deep path then refuses at step 4 while #1 is live. The per-unit stride also ignores capacity slack: push-built terms and offset Vecs with minimum capacity 4, product Vecs grown to 4 by `extend_from_slice`, and the outer Vec's PushCap. My estimate of #1 is about 1.73 MB, against the formula's 1.20 MB. "Failure paths are prefixes" (:140) holds structurally, but the success-path bound on #1 does not bound the failing prefix;<br>(d) the script omits the Response "6E" temporaries that the table lists.<br>**Totals (ASSUMED strides):** the table as written gives 31,669,316 B; adding (a) and (b) gives 35,563,880 B; the script gives 36,474,868 B. The published figure stays above my corrected sum only because the script counts descriptors 7× (3·2 + 1) where the table says 3. | Restate the rows with these owners and with J3 capacity. Bound #1 by its construction law: a per-member constant times m, plus support rows proportional to C, l and s, with slack. State the number of descriptor copies explicitly. Re-evaluate before G5 encodes T07; G3 may keep the published number meanwhile. |
| S-2 | SHOULD-FIX | RESIDUALS.md:195–212; D4_RECONCILIATION.md:42 | **T22 is not yet I34's closed scalar admission:**<br>(a) I34 API_PLAN:245 names "r*r". That is `ceil_sqrt` (FK/bound.rs:996–1006), which uses u128 widening rather than a checked stop. Its caller `2 * ceil_sqrt(n_c)` (bound.rs:1044) is an unchecked u64 multiply. Both are safe at the caps (14 and 28), but the table omits them and calls every site "a checked operation with a typed stop", and D4 §2 substitutes "n·n" for r*r;<br>(b) FK/source.rs:356–358 computes free·(free+1), which is 37,056 at the caps, not "free·n" (36,864);<br>(c) I34 DESIGN.md:255–265 lists further classes the table does not enumerate: the layout upper 7n+12m+6t+s+3d+r+2g, tracker held−before+after, scale/exponent/index products and factor 64·operations;<br>(d) the count of "57 CountRange stops" cannot be reproduced. Non-test FK/retained has 36 literal-tagged `CountRange("…")` constructions and 62 `CountRange(` tokens. | Enumerate I34 DESIGN.md:255–265 site by site, each with its check or representability argument. Re-point the D-4 ruling's T22 citation at the repaired table. The outcome does not change: input-derived scalars are tiny at the caps, and operation-derived ones are checked. |
| S-3 | SHOULD-FIX | DOMAIN.md:30–46; RESIDUALS.md:67–76 | The cap table caps typed **lengths** and raw **capacities**, but no typed capacity:<br>– String capacity (128 is a length cap);<br>– typed Vec capacities, including the length-0 components, combinations and expansion vectors whose capacity is "still read";<br>– section property trees;<br>– typed Value facts (`project.units`, section provenance).<br>A constant bound priced at the caps needs capacity caps. Today `from_value(raw.clone())` gives capacity = length, but the packet says it does not rely on construction. | Add typed capacity caps to D1.9: String ≤ 128, Vec ≤ its count cap, length-0 vectors capacity 0, typed Value facts within the raw caps. Alternatively, state that typed terms are priced from actual G-A facts. |
| S-4 | SHOULD-FIX | DOMAIN.md:3, :24, :40; RESIDUALS.md T03 | D1 admits up to 32 sections and leaves `pipe_segments[i].section_ref` unconstrained. The ordinary route then mutates the moved typed request (PP/lib.rs:2283–2339):<br>– `resolve_shared_sections` (:7357) builds a refs `vec!`, a `format!` id, a `matches` collect, a `section.clone()` and `od`/`wall` clones written into the model;<br>– `normalize_model_units` (:7514) replaces every `quantity.unit` with a fresh canonical String (:8379). It also builds a `format!` id and `vec![id.clone(), ..]` refs for each quantity, eagerly, even on success.<br>I54 BOUND has no row for this phase; CHECKPOINT_A:35 only names it. T03's `R_typed` is the G-A snapshot and does not bound the mutated tree. | Either require `section_ref` None and zero sections in D1 (the milestone has none), or name the normalization and shared-section phase as a T05 row for G3, with its old/new pairs. |
| S-5 | SHOULD-FIX | BUILD.md:67 | `env!("OPS_RETAINED_BUILD_IDENTITY")` fails to compile when the variable is absent, for example in builds or analyses that skip build scripts. The brief requires no path to a compile error. | Use `option_env!`, with `None` → `Stale`. |
| S-6 | SHOULD-FIX | RESIDUALS.md:158–183 | **Rows missing from the spelling table:**<br>– str/String Debug: `escape_debug` emits at most 6 bytes per input byte plus 2 quotes, so 770 B for a 128-byte string. Any composite Debug with String fields needs this row;<br>– `{:032x}` (FK/wide.rs:218, W1 side): 32 hex digits.<br>**The composite-Debug split is partial.** It omits, for example:<br>– PP/lib.rs:3756 `{failure:?}`, the `RecoveryFailure`, which the milestone reaches;<br>– :2809 and :5003 `actual_finalization_work={:?}`;<br>– :4635, :4651, :1143, :6004, :6619, :9770, :9791, :11291. | Add the two rows. G3 classifies all 56 `{:?}` sites, as it already owes. |
| N-1 | NOTE | RESIDUALS.md:81–93 | **T06 confirmed:** no heap on the H_formation128 path.<br>– FK/wide.rs has no Vec, Box, String, `format!`, `collect` or `clone()`. Its 15 `write!` sites are Display/Debug impls into a Formatter.<br>– The submodule wide/multi.rs (`pub(crate) mod multi`, wide.rs:126) is also heap-free, but the "whole file" search excludes it.<br>– `split_binary64` uses an inline `[f64;3]` (wide.rs:226, :409–414).<br>– After :298, formation_check.rs's heap tokens are all inside `body_scales`. | Name wide/multi.rs in the argument. |
| N-2 | NOTE | DOMAIN.md:52–63; RESIDUALS.md:23–29 | **Reproduced** (RX/rv83_caps.out.json):<br>– N 192; Q 1,472; P_final 2,115; C = Z 4,640; E 2,528; H 18,528; source_count 4,832; Fn 1,760;<br>– T02: 15,623,328 B at 632/728 and 15,780,608 B at 640/736.<br>Every formula substituted into is a nonnegative-coefficient polynomial, min/max or PushCap, and so is monotone nondecreasing. The cap derivations take s ≤ g from PP/lib.rs:6756–6772, which I confirmed. Figures that depend on s(Value)=32 and the node sizes are ASSUMED. | — |
| N-3 | NOTE | BUILD.md:110–123 | **Leaf_up 640 and Internal_up 736 for (String, Value) are sound for any field order.** Per-field padding to A bounds every ordering, and InternalNode is `#[repr(C)]` (btree/node.rs:43–111). An alignment-sorted layout reproduces the DWARF 632/728.<br>**The hashbrown law is confirmed:** NEON Group WIDTH 8 (hashbrown-0.17.1 control/group/neon.rs:10–21); `capacity_to_buckets` and `calculate_layout_for` (raw.rs:104–235) give `HashReq = up(s·b, max(a,8)) + b + 8`. The toolchain rmeta shows std bundles hashbrown 0.17.1. | — |
| N-4 | NOTE | RESIDUALS.md:158–170 | **f64 spellings confirmed** from core/fmt/float.rs:12–23, :87–103, :180–203 and numerically over 80,012 samples, including the extremes: Display 327, Debug and LowerExp 24, positional Debug 23.<br>**The format capacity bound ≤ max(8, 2·len) is confirmed** (core/fmt/mod.rs:754–806; raw_vec grow_amortized :502–524). | — |
| N-5 | NOTE | RESIDUALS.md:48–65 | **The T03 roster is complete for D1-admitted owners.** My own enumeration of PP/lib.rs:163–770 and case_state/input.rs:35–45 agrees with it. `Authored::Null` is distinct from `Absent`, so "Absent" in D1.3 and D1.5 correctly excludes an explicit null. | — |
| N-6 | NOTE | DOMAIN.md:85–88 | **Milestone recount** (RX/rv83_milestone.out.json):<br>– sha256 `2aa51bee…`;<br>– 150 values; 37 objects with 131 entries; 12 arrays with 18 elements; 82 strings of 1,039 B; 855 key bytes; depth 7; longest string 46 B;<br>– schema 0.1.0; n=2, m=1, g=4 (one rigid, three springs); 3 nodal moment loads; materials 1 and 0; no temperature points or sections.<br>Every D1 clause holds. Its three `concentrated_moment` loads raise the `LOAD_CATEGORY_PREVIEW_MAPPED` warning (PP/lib.rs:8473–8486), a D1 text term the milestone itself reaches. | — |
| N-7 | NOTE | DOMAIN.md:67–81 | The refusal map uses only kinds in the schema's precondition enum (schemas/retained_precision_mp_v2.schema.json:6478–6484). D1 does not use `capture`, because W1 capture faults arise after G-A. | — |
| N-8 | NOTE | STACK_PLAN.md | **R = 64 MiB and k = 16 are a defensible provisional proposal** on the stated observation, provided G3 finds no count-proportional recursion. Items for G3 are in §7 below. | — |
| N-9 | NOTE | BUILD.md:44–55, :86–94 | **The identity does not record** lto, codegen-units, overflow-checks or debuginfo. These change frames, not layouts, and `PROFILE` folds custom profiles into debug or release.<br>**It does not bind the maintained source revision,** so formula drift after a source change is caught only by review or G6.<br>**`unbounded_depth` and `raw_value` have no witness;** the parser's 128 limit is a reviewed record.<br>**The consumer-lock residual is acceptable** for Direct-only D1 as mitigated; a build script has no mechanical alternative. | State these in the G6 record. |
| N-10 | NOTE | D4_RECONCILIATION.md | **D-4 checks out:**<br>– C1:64 and :66 are quoted byte for byte;<br>– RR:5024, 5144, 5210, 5260, 5468 and 7157, I34 DESIGN.md:230–233, API_PLAN.md:304–306 and I52 RETURN:23–25 all hold;<br>– the ledger guard (PP/lib.rs:837–839, :880–896) and `Work::charge` (FKS/exact_boundary.rs:80–87) hold.<br>The reading applies C1 §2's own alternative and does not change the contract (§8), subject to S-2. | — |
| N-11 | NOTE | RESIDUALS.md:199–207 | Every cited T22 check exists at its cited line, and its value at the caps fits its width (≤ 37,056, far below `u32::MAX`). | — |
| N-12 | NOTE | DOMAIN.md:43 | The census counts the root at depth 0 (retained_memory.rs:111), so "depth ≤ 16" admits 17 levels. | G5 states the convention. |
| N-13 | NOTE | API.md; STACK_PLAN.md §1 | **API.md is consistent with D-5, decision 7 and design-to-budget** (§9). Separately, result_export's `OnceLock` schema and contract caches (physics_source.rs:39, load_reference.rs:334, semantic_contract.rs:5–155) allocate on first use, persist for the life of the process, and parse on the scoped thread. That belongs to G4 (T17) and to G3's stack inventory. | — |

## 1. The D1 predicate (DOMAIN.md §1, §3)

**What each clause excludes, checked against source:**
- **D1.0:** `Entry` has only Direct and Headless (retained_memory.rs:308–311).
- **D1.3:**
  - `is_exact` requires 0.3.0 or 0.4.0 plus an exact contract (pressure_runtime.rs:77–83);
  - `is_load_state` requires the load-state version (case_state/mod.rs:39–41);
  - 0.1.0 and 0.2.0 differ only in `validate_profile`;
  - `Authored::Absent` excludes an explicit null.
- **D1.4:** retained_product.rs:194–201 and :1129–1131.
- **D1.5:** retained_product.rs:1137–1150; source_recovery.rs:604–612.
- **D1.6:** `support_hanger_type` falls back to the trimmed family (PP/lib.rs:12004–12027). Exact matching with `hanger` None therefore excludes hangers and constant-effort supports. Family None takes the rigid branch, which ignores any `stiffness` (:6826–6847). A "spring" support yields one `LinearSupport::spring` on one DOF, so s ≤ g.
- **D1.7:** Node targets with force or moment dimension produce only `nodal_force` (:8500–8515). The thermal loads (:10042–10060) and pressure-thrust loads (:10094–10107) need Element targets, so both stay empty.

**Fields the predicate does not constrain that change the ordinary path:**
- **`section_ref` and `sections`** (S-4), together with unit normalization, which mutates the moved typed request (S-4).
- **Typed capacities** (S-3).
- **The fields named in the brief:**
  - schema-version defaulting: none; `schema_version` is required on the wire (PP/lib.rs:207);
  - optional provenance Values: priced through the Value census;
  - the two material lists: request materials replace model materials when non-empty (:2286–2291), both capped at 4;
  - case-level options: all constrained by D1.5;
  - solver mode: a parameter, with both modes in D1.
- **The largest gap is not a field.** It is the exact-selected branch's finalization (B-1), which in-domain inputs reach.

**The refusal map** matches the schema enum (N-7). **The milestone counts** reproduce (N-6).

## 2. Cap arithmetic

I re-derived the arithmetic with my own stdlib script (RX/rv83_caps.py, output in rv83_caps.out.json), working from the RESIDUALS text and the source rather than from `caps_arithmetic.py`.
- **Derived quantities and T02** reproduce exactly (N-2).
- **Monotonicity:**
  - each derived count is a polynomial with nonnegative coefficients, or a min, max or F(F+1)/2 of such;
  - PushCap(s,h) is nondecreasing in h;
  - T02 is linear in the census facts.
- **T07 does not reproduce from the table.** At the ASSUMED strides:
  - the table as written gives 31,669,316 B;
  - the script gives 36,474,868 B, because it carries 7 descriptor copies;
  - the owners I found missing add 3,894,564 B (S-1).
- **ASSUMED values:** every figure that depends on s(Value), Leaf, Internal, s(Expansion), s(Ratio), s(FD) or the other strides uses the same illustrative 64-bit values I65 used, so the totals can be compared. None is a qualified value.

## 3. T06 and T22

- **T06:** confirmed heap-free (N-1).
- **T22:**
  - every cited check exists, and its cap value fits its width (N-11);
  - the table omits I34's r*r (`ceil_sqrt`) and the further DESIGN classes, and mislabels FK/source.rs:358 (S-2).

## 4. T03 owner roster

My own enumeration of the `LinearStaticPreviewRequest` type tree finds no missing nested owner among those D1 admits (N-5). The gap is in the caps, not the roster: typed capacities have no cap (S-3).

## 5. T08 text inventory

- **The count reproduces.** My lexer gives PP/lib.rs 362 `format!` + 145 `diag(` sites, the same as I65, and 707 sites over I65's 14 files with the same placeholder split.
- **The file set is incomplete** (B-2).
- **The spelling maxima reproduce** (N-4), except for the missing str Debug and `{:032x}` rows (S-6).
- **The 332-site pre-filter is treated as a lead only** (RESIDUALS:191), as the brief requires.

## 6. BUILD.md: D-6, T01 and T10

**The fail-closed shape is right:**
- a read failure gives "unavailable";
- comparison is byte-exact;
- a false witness gives `Stale`;
- witnesses are `const bool` values, never assertions.

**The design does not yet meet the brief's criteria:**
- the newline-separated canonical text cannot survive a rustc-env directive (B-3);
- `env!` adds a compile-error path (S-5).

**The identity covers every fact a layout stride or container law depends on,** with these exceptions:
- the facts that depend on serde_json's version, which the reviewed lock covers;
- a few profile knobs that affect only stack frames (N-9).

**The uppers re-derive:** BTree 640/736 and the hashbrown law (N-3).

**The consumer-lock residual is acceptable as stated** (N-9).

## 7. STACK_PLAN under S1

**The recursion classes the brief lists, against the plan:**
- derive-generated recursion, Value Drop and Clone, sort, and the reader's `$ref` following are named or deferred to G3;
- the claim "BTreeMap … operations are iterative" misses `clone_subtree`, which recurses but only to the tree height.

**The thread-local scan holds.** Every `thread_local!` found in PP and FK is `cfg(test)` or behind `mutation-controls`:
- PP/lib.rs:2989, the dense-ceiling override;
- historical_pressure_reference.rs, a `cfg(test)` module (PP/lib.rs:105–106);
- source_receipt/composite.rs:1167;
- FK adaptive.rs:5159 and :5190, and verify.rs:1243;
- seeded.rs, gated `cfg(any(test, mutation-controls))`.

Std's per-thread state does differ across the hop: RandomState keys, and the thread name in the panic hook's output. Neither matters if outputs do not depend on HashMap order, and HashMap order is already random per process.

**Panic and spawn handling.** The spawn-failure handling is sound. A panic keeps its payload, but the hook has already run on the scoped thread, so "the caller sees the same panic" holds for the payload only.

**R = 64 MiB with a witness at R/16 is defensible as a provisional, measured-evidence proposal** (N-8).

**G3 must add:**
- the per-chain recursion inventory, including `clone_subtree`;
- serde's buffering for `#[serde(flatten)]` (`MaterialRecordWire`) and for the internally tagged `LoadTargetInput`, which is depth-bounded;
- the first-use OnceLock schema parse on the scoped thread (N-13);
- debug-build frames in `json!`- and `format!`-heavy functions;
- driftsort's on-stack scratch;
- the stack used by the panic and backtrace path;
- a statement that the witness binary is `cfg(test)`, while its identity text matches the production build.

## 8. D4_RECONCILIATION.md

The quotations and citations hold (N-10).

**The reading applies C1 §2's own alternative.** C1:64's last sentence allows a checked/sticky seam "after its own bounded design, maintained write-set and review". The record meets that:
- the design is I34, reviewed by RV46;
- the API is API-02, selected at RR:5260;
- the source is `fdae294643b`, accepted at RR:5468 after RV51.

**One premise is not closed.** The reading leans on "closed pre-execution scalar admission" (I34 API_PLAN:304, which RR:5210 requires), and T22 does not yet close it (S-2). That is a citation repair. It is not a challenge to the D-4 or D-4b outcome.

## 9. API.md

API.md is consistent with three things:
- **D-5:** U4 owns `retained_memory.rs`; U3 owns the dispatch; I61 owns `PP/lib.rs`.
- **Decision 7:** no test permit. The `cfg(test)` R override is a test knob, like the dense-ceiling precedent, not a permit.
- **Design-to-budget:** through `budgets()`.

**Suggestions for G5:**
- make G-B before G-C a typestate, by consuming the permit into a late token;
- avoid running the census twice, once in `assess` and again in `admit`;
- confirm that the moved request, the borrowed capture and the result are `Send`/`Sync` for the scoped thread;
- map spawn failure explicitly in the API.

## For ROOT

1. **B-1:** price selected source-blocks finalization as a new G3 term, and say so in the G3 direction.
2. **B-2:** replace "all 707 sites" with "every text site on the D1 call graph". The 707-site inventory becomes a starting list.
3. **B-3 and S-5:** amend the D-6 design (single-line encoding; `option_env!`) before G5.
4. **S-2:** the D-4 ruling cites T22. Point that citation at the repaired table. The outcome is unchanged.
5. **S-4:** choose between restricting D1 to `section_ref` None and pricing the shared-section and normalization phase in G3.

## Execution record

**Who and when.** RV83, TASK (Type 2) under ROOT, with no descendants. Started 22:00 MDT and finished within the 3-hour box. Nothing in the brief was left unfinished. Within T07 I re-checked the rows named in S-1 and the Context, block, witness and Response rows against source; I did not audit every remaining row exhaustively.

**Memory guard.** `memguard.sh`, PID 5387, was running at the start.

**What was not run:** no Cargo, rustc, solver, native or DEC-025 job. Git use was reads only (`rev-parse`, `log`, `status`, `diff --stat`), all with `GIT_OPTIONAL_LOCKS=0`.

**What was run:** stdlib Python only. I read installed std sources from the toolchain's rustdoc HTML, and hashbrown 0.17.1 and serde_json 1.0.149 from the cargo registry cache. The rustc identity came from the toolchain's version_info.html and channel manifest; rustc was not executed.

**Writes:** RX/ and REVIEW.md, plus WT/scratch/rv83_u4_g2_01/. One intermediate JSON was briefly written to the system temp directory and deleted at once; that was outside my write fence. The run records hold no machine paths.

**Run records** (RX/):
- `rv83_caps.py` and its output: the arithmetic;
- `rv83_text_sites.py`: the lexer and site extraction, with outputs `rv83_text_sites_summary.out.json` and `rv83_text_sites_pp_lib.out.json`;
- `rv83_milestone.py` and its output;
- `rv83_rsrc.py`: the rustdoc source reader;
- `ORIGINS.json`: source hashes at `fe38ea55bc`.

All are covered by `SHA256SUMS`.
