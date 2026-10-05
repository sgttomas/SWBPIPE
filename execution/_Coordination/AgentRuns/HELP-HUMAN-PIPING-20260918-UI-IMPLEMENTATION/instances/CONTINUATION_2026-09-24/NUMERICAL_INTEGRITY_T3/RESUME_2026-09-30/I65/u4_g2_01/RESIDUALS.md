# G2 residual closure: T02, T03, T06, T07, T08 (ordinary templates) and T22

**Four of the six terms close at the D1 caps by source argument:**
- **T02 (raw backing):** the accepted formula, evaluated at the caps;
- **T06 (H_formation128):** heap-free by source;
- **T07 (deep legacy-exact):** bounded by its own exact-boundary limits together with the caps;
- **T22 (scalar admission):** every named site is checked and representable at the caps.

**Two close in part:**
- **T03 (nested typed owners):** the owner enumeration and census design are complete; the reads are implemented in G5.
- **T08 (text):** the ordinary template inventory and the spelling maxima are done; per-site multiplicity and the composite-Debug type bounds move to G3, as planned.

Each claim below cites its source at NUM `a2c26cc885`, which matches `dca3b65b3d` for every maintained file. Numbers marked ASSUMED use illustrative 64-bit layouts. They are not qualified values; G5 and G6 evaluate in-build (BUILD.md).

Arithmetic: `_run_records/caps_arithmetic.py`, output in `caps_arithmetic.out.json`.

---

## T02 Raw request backing: priced; evaluated at the caps

The accepted formula (I54 BOUND:36–44, RV75):

`R_raw = s(Value)·array_capacity_elements + key_capacity_bytes + string_capacity_bytes + max(Leaf, Internal)(String,Value) · (objects + ⌊object_entries/5⌋) + digest_capacity`

**At the caps.** Objects and entries are each ≤ raw values ≤ 16,384, which is conservative. The formula is linear and nondecreasing in every fact, so the cap value bounds every D1 input:
- **15,623,328 bytes** with the DWARF-observed 632/728 node sizes (ASSUMED layout);
- **15,780,608 bytes** with BUILD.md's upper formulas, Leaf_up 640 and Internal_up 736.

**Closing evidence:**
- the census facts are complete-or-refuse (D1.2);
- the node sizes are bound by the BUILD.md §4 witnesses;
- the D-6 witnesses guarantee the Map is a BTreeMap and that Number owns no heap (BUILD.md §2.3).

**Remaining:** none at G2 scope; G5 evaluates the formula in-build.

---

## T03 Nested typed owners: enumeration complete; census reads are G5

**The route** (plan T03): read actual lengths and capacities from the borrowed typed request at G-A, allocation-free, extending `borrowed_request_census` (retained_memory.rs:185–201). Construction-history laws are not used.

**How the typed request is built** (source_receipt.rs:109–131):
- it comes from `serde_json::from_value(raw.clone())`, so typed Strings are moved from cloned Value strings (capacity equal to length);
- typed Vecs come from exact-size sequences.

These are corroborating facts. The census reads actual capacities anyway, so a different construction path cannot invalidate it.

**The owner roster in D1.** Types are from PP/lib.rs:163–770, pressure_runtime.rs:25–48 and case_state/input.rs:35–70.

| Path (one entry per element) | Heap owners to read | D1 constraint |
|---|---|---|
| `request.materials: Vec<MaterialInput>` | Vec cap. Per element: `id`; `constitutive_basis: Option<String>`; `provenance: Option<String>`; the Quantity units of `elastic_modulus` and of the Option `poisson_ratio`, `shear_modulus` and `thermal_expansion_coefficient`; `temperature_points` Vec cap and, per point, `id`, `provenance`, the five Option<Quantity> units | ≤ 4 materials; ≤ 16 points |
| `model.schema_version`, `document_kind` | String caps | ∈ {"0.1.0","0.2.0"} |
| `model.project` | `id`; `units: Value` (census it with `borrowed_value_census`) | text cap; the Value is part of raw depth and count |
| `model.analysis_status` | three String caps | text cap |
| `model.nodes: Vec<PreviewNode>` | Vec cap (already a top-level fact); per element `id`, `provenance` | n ≤ 32 |
| `model.pipe_segments: Vec<PreviewPipe>` | per element `id`, `from`, `to`, `material`, `section_ref`, `provenance`; `section` = two Quantity units plus five Option<Quantity> units | m ≤ 32 |
| `model.sections: Vec<PreviewSection>` | per element `id`, `name`, `section_type`; `properties: BTreeMap<String, Quantity>` (len → node count by I54 COEFFICIENTS T2, nodes sized by Leaf_up/Internal_up(String,Quantity), BUILD.md §4); each key String cap and Quantity unit cap; `provenance: Value` (census) | ≤ 32 |
| `model.supports: Vec<PreviewSupport>` | per element `id`, `node`, `family`, `provenance`; `restraints` Vec cap plus each String cap; `stiffness: Option<SupportStiffnessInput>` = `dof` String plus Quantity unit | g ≤ 32; Σ restraints ≤ 192; `hanger` and `nonlinear` are None |
| `model.materials: Vec<MaterialInput>` | as `request.materials` | ≤ 4 |
| `model.load_cases: Vec<PreviewLoadCase>` | per case `id`, `provenance`; `primitive_loads` Vec cap and, per load, `id`, `category`, `direction`, `dimension`, `provenance`, the `target` node String and the magnitude unit | exactly 1 case; l ≤ 192; the case's Option and Authored fields as in D1.5 |
| `model.components`, `model.combinations` | Vec caps only | len 0, capacity still read |
| `model.material_expansion_laws: Vec<Authored<…>>` | Vec cap (built by `unzip`, PP/lib.rs:226–231) | every element `Absent`, so no child heap |
| `model.request_material_expansion_laws: Vec<usize>` | Vec cap | len 0 |
| `model.pressure_contract`, `reference_configurations`, case `pressure_regions`, `equivalent_static`, `analysis_state` | none, by D1 | must be None or Absent |

**The formula.** `R_typed = Σ_vec s(T)·cap + Σ_strings cap + Σ_maps nodes·max(Leaf_up, Internal_up) + Σ_Value R_raw-formula(Value facts)`.
- Element strides are taken in-build (BUILD.md §3).
- Typed string capacity is additionally bounded by raw string bytes plus key bytes when the request comes from the raw clone, but the census value is the one used.
- Top-level Vec facts are already in `BorrowedRequestFacts`.

**What G5 adds:**
- the nested walk;
- `max_string_bytes` and `max_key_bytes` for the 128-byte cap;
- `TypedIncomplete` if any expected owner cannot be read.

**Evidence for review:** check this roster against the type definitions cited. Any field present in source but missing here is a missing term.

---

## T06 H_formation128: heap 0 by source; stack goes to STACK_PLAN

**The term** (I54 REQUIRED_FACTS): the "local natural-number/width-helper heap and stack" of the 128-bit re-formation at FKS/formation_check.rs:228–268.

**The argument:**
1. **The helper owns no heap.** `WideArith` holds `{precision: u32, work: WorkCounter}` (FK/wide.rs:871–885), and `Wide<L>` is `{negative: bool, exponent: i64, significand: [u64; L]}` (FK/wide.rs:204–208). FK/wide.rs contains no `Vec`, `Box`, `String`, `format!`, `collect` or `clone()` of an owning type: a search of the whole file finds none. Every `WideArith` operation (`add`, `sub`, `mul`, `div`, `sqrt` at FK/wide.rs:897–925) returns `Wide2` by value.
2. **The accumulators own no heap.** `ExactAccumulator` is `{positive: Magnitude, negative: Magnitude}`, fixed inline arrays (FK/exact_sum.rs:45–48), and exact_sum.rs contains no heap tokens.
3. **The element helpers own no heap.** `frame_matrix`, `user_matrix`, `curved_matrix`, `rotate`, `chord_axes`, `quad`, `mul6`, `invert6`, `lift3`, `sub3`, `dot3`, `cross3`, `scale3` and `norm3` (FKS/formation_check.rs:464–873) operate on `Element = [[Wide2;12];12]`, `M6 = [[Wide2;6];6]` and `Vec3 = [Wide2;3]` (:460–462). From :298 on, the only heap tokens in the file are inside `body_scales` (:372–455).
4. **Every other allocation in `check`/`evaluate`** (:155–340) is already priced by I54 BOUND:73: `free_index`, `rows`, `truncated` and its children, `exponents`, `rhs`, the backend `solve` result, and `body_scales`.
5. **The error Strings** — `FormationCheck::unavailable(family.clone())`, the `format!` at :171–174, and `Failure::detail()` at :143–150 — are text, under T08.
6. **Curved and user-stiffness elements are outside D1** (no components).

**Result:** H_formation128 has heap = 0. Its stack (an `Element` is 144·s(Wide2) bytes) is in STACK_PLAN §2(d). **Remaining:** none at the heap level. Reviewer: confirm the search claims by reading FK/wide.rs and formation_check.rs.

---

## T07 Generic deep legacy-exact: closed under the exact-boundary limits and the caps

**The path.** `source_recovery::solve` (PP/source_recovery.rs:1434–1600) is entered for an in-domain case when the ordinary route needs legacy source recovery (PP/lib.rs:3678–3722). It runs, in order:
1. `prepare_sources`;
2. `exact::Context::prepare_with_budget`;
3. `solve_with_budget`;
4. `FunctionalPlan::new`;
5. `evaluate_functionals`;
6. one `project` per functional;
7. 2N DOF projections;
8. `retain`;
9. selected-output construction.

The named skew case is refused inside `prepare_sources` (I54 BOUND:97). Axis-aligned D1 cases can run the whole path, and a success makes exact-block select (D39), so W1 is bypassed. G-A's ordinary-span bound must still include this path.

**The three bounding facts:**
1. **Dimensions.** `prepare_sources` requires N ≤ 256 and source_count = 144m+s+l ≤ 16,384 (source_recovery.rs:507–524). `Context` requires n ≤ 256, contributions plus force terms ≤ 16,384, and every free connected block of order ≤ 2 (FKS/exact_boundary.rs:288–347, 428). At the D1 caps N ≤ 192 and source_count ≤ 4,832.
2. **Expansion length.** Every `Expansion` on this path is created by `Work::{add, sum, scalar, mul}` or is a clone of one. These refuse growth beyond `limits.expansion_terms` = 256:
   - `Work::add` checks `len ≥ limit` before adding (FKS/exact_boundary.rs:89–103);
   - `Work::mul` checks `len + 2 > limit` before each product (:116–141);
   - projection uses the same `Work` (:769–851).

   `source_recovery` passes `Limits { operations: case_limit, ..Default::default() }` (PP/lib.rs:3711–3718), so `expansion_terms` = 256.
3. **Expansion capacity.** `Expansion::add` rebuilds its terms in a fresh `next` Vec by push (FK structural.rs:732–755). Capacity is therefore PushCap(8, ≤256) ≤ 256, which is at most 2,048 child bytes. A clone's capacity equals its length. During one `add`, the old and new buffers coexist: at most +2,048 bytes for the active add.

**Owner roster at the peak phase** (step 9, where every earlier owner is still live; a conservative sum of everything at once). n = N; Fn = 42m + N + s + 6g functional descriptors (members 42 each, nodal N, springs, supports 6 each; source_recovery.rs:1110–1240); E = s(Expansion) + 2,048.

| Owner | Source | Bound |
|---|---|---|
| Sources completion (generic): descriptors #1 and the identity builder and JSON | source_recovery.rs:629–1240; `Identity` :462–501 | descriptors: Fn·(s(FD) + 2·128) + 16,384·max(s(Vec), 8, s(AffineTerm)), because `descriptors_charge` caps descriptors + products + atoms + terms ≤ 16,384 (functionals.rs:427–476). Identity: names ≤ 8 + n + 6l + 6g + r + s + 12m and bits ≤ 8 + 3n + 4l + 4g + r + 4N + 3s + 325m, tallied from the `identity.*` call sites listed in `_run_records`; JSON ≤ names·(6·128 + 3) + bits·21 |
| Context Snapshot #1 | exact_boundary.rs:251–262, 465–486 | Mat(f64,N,N) stiffness + 3 vectors + C·s(StiffnessContribution) + force terms with their source Strings + symmetry Mat(f64)+Mat(usize)+basis |
| Context k and f | :351–405 | Mat(Expansion,N,N) headers + 8·(4Z + 2C) children; f = N·s(Expansion) + 8·(4N + 2·force terms) |
| blocks and witnesses | :408–457 | ≤ F blocks of ≤ 2; witnesses ≤ F·(16 + 3·8·256 + 2,048) |
| new_charged transients | :320, 352, 381, 408 | ordered_k Mat(f64,N,N), seen and visited N each, the symmetry `sum` temporary |
| FunctionalPlan descriptors #2 | functionals.rs:309–322 | as #1 |
| Response | exact_boundary.rs:527–606 | 2N Ratios: 2N·s(Ratio) + 4N·2,048; reaction-loop temporaries ≤ 6E |
| FunctionalSet values | functionals.rs:550–606 | Fn·(s(Ratio) + 2·2,048); per-descriptor transient N·E + 12·2,048, dropped each iteration |
| projections | source_recovery.rs:1456–1475 | PushCap(s(QFP), Fn)·s(QFP) + PushCap(s(QP), 2N)·s(QP) |
| RetainedFunctionalSet | functionals.rs:751–824; exact_boundary.rs:1378–1452 | Snapshot #2 + witnesses #2 + 2N Ratios (clone, children ≤ 8·256) + 2N·s(RetainedProjection) + descriptors #3 + values #2 + Fn·s(RFP) |
| selected output | source_recovery.rs:1480–1600 | 16N + m·(s(MemberRecovery)+128) + s·(s(SpringAction)+128) + g·(s(SupportActions)+128) + summary |

**Total at the caps, everything live at once (ASSUMED layout):** 36,474,868 bytes, about 34.8 MiB (`caps_arithmetic.out.json`, `T07_…SUM`). Every term is monotone in N, m, g, s, l, C and Z. After `solve` returns, `SelectedSourceRecovery` (with the retained set) stays live in the ordinary suffix; G3 composes it there.

**Failure paths are prefixes of the success path.** Every step returns through `?` or `Attempt.result?` (source_recovery.rs:1440–1475), and the `Work` refusals stop before allocating. So the owners live at any failure are a subset of those live at the same point on the success path, and the peak above bounds them. The retained `RecoveryFailure` keeps only its stage, its error and its `WorkReport` (source_recovery.rs:397–404).

**Remaining:** none at the derivation level. The type strides are evaluated in-build (BUILD.md §3), and G3 places this term into the ordinary-span composition.

---

## T08 Text: ordinary templates and spelling maxima done; multiplicity to G3

**The inventory.** `_run_records/template_inventory.py` lexically extracts every `format!`, `write!`, `writeln!`, `format_args!` and `diag(…)` site in the non-test ordinary-route files. **707 sites:**
- PP lib.rs: 145 `diag` and 362 format calls;
- preview_physics: 17 and 33;
- formation_guard: 11;
- source_receipt: 11;
- source_recovery: 2;
- the FK, straight_pipe, stress_recovery, linear_supports, primitive_loads, sparse_direct and structural_adapter Display/format sites.

Each row records the literal template, its literal byte length and each placeholder's spec and argument (`template_inventory.out.json`). Placeholder specs: 909 `{}`, 56 `{:?}`, 10 `{:e}`, 4 `{:016x}`, 2 `{:x}`.

**Spelling maxima** (installed core 1.97.1, `core/fmt/float.rs`):

| Placeholder value | Maximum bytes | Source and reasoning |
|---|---|---|
| f64 Display `{}` | **327** | positional, never exponential (`float_to_decimal_display`, :87). Shortest digits (≤ 17) expanded: the subnormal 5e-324 renders as "0." + 323 zeros + "5" (326), plus sign |
| f64 Debug `{:?}` | **24** | exponential when x ≠ 0 and \|x\| < 1e-4, or \|x\| ≥ 1e16 (:12–23, 180–195). Otherwise positional with \|x\| < 1e16, which is ≤ 22 + sign. Exponential is "-d.dddddddddddddddde-308", 24. "NaN"/"inf"/"-inf" fit |
| f64 LowerExp `{:e}` | **24** | as exponential Debug |
| u64/usize; i64; i32; u32; u8 | 20; 20; 11; 10; 3 | decimal digits plus sign |
| `{:016x}` u64; `{:x}` | 16; ≤ 16 | hex |
| identifier or other input String | ≤ 128 | D1 text cap |
| `&'static str` literal argument | its literal length | from source |

**String capacity** for a formatted owner follows I54 COEFFICIENTS J9: initial reserve from the literal length, then V3 growth. So a formatted String's capacity is ≤ max(8, 2·length), the convention RV75 accepted for these templates.

**The composite Debug sites** need a type-structure bound before multiplicity. These are the 56 `{:?}` placeholders.
- **The ones that carry vectors:**
  - PP lib.rs:1117 (the full `StructuralReport` and `integrity_dof_map(model)` in one diagnostic, which I54 REQUIRED_FACTS also flagged);
  - :1266 and :1270 (`row.sources`);
  - :1321 and :1827 (`integrity_dof_map`);
  - :3972 (`recovery.summary()`);
  - formation_guard.rs:234 and :244.

  Each is a Debug of finite, non-recursive types whose Vec lengths are bounded by the caps: F, N or the contribution count. Its maximum is Σ (field names + punctuation + element maxima from the table above).
- **The ones that are unreachable in D1:**
  - PP lib.rs:4043 and :4054 (`iteration.structural_report`): the nonlinear iteration path, since nonlinear supports are excluded;
  - structural_adapter.rs:2921 (nonlinear `states`).

**Moves to G3** (the plan's G3 row evaluates T05 at the caps):
- the per-site multiplicity: how many times each site can fire for one D1 invocation, from the loop structure enclosing it;
- the D1 reachability filter for each site;
- the composite-Debug structure bounds above;
- the live-text sum per ordinary phase.

A heuristic pre-filter gives G3 a starting split (`_run_records/template_d1_prefilter.py`, output `template_d1_prefilter.out.json`): **375 sites are D1 candidates and 332 are marked excluded**, by diagnostic code or enclosing-function family (pressure, nonlinear, components, combinations, hangers, thermal, wind/seismic, load state, expansion, bends, imposed, basis selection). G3 confirms each exclusion by call path; the pre-filter proves nothing by itself. The inventory is the complete list G3 must cover. A site G3 does not cover is a missing term, never zero.

---

## T22 Scalar admission: every named site is checked; all are representable at the caps

I34 API_PLAN.md:244–245 names the sites: "6n, offsets, residual m/64m, tracker sequences and r*r". Each is a checked operation with a typed stop, so an out-of-range value is a typed refusal, never a wrap. The caps make each one representable in D1:

| Site | Check | Value at the D1 caps | Width |
|---|---|---|---|
| 6n | `nodes.len().checked_mul(6)` and `u32::try_from(nodes)` (retained_product.rs:179–186); `checked_mul(DOF_PER_NODE)` (source_recovery.rs:507–512) | 192 | usize; u32 |
| member stations 3m; supports | `u32::try_from(m)…checked_mul(3)`; `u32::try_from(supports)` (retained_product.rs:181–185) | 96; 32 | u32 |
| source offsets and lengths | `u32::try_from(len)`, ID lengths and child counts (FK/source.rs:389–440) | ≤ 128 bytes; ≤ 192 children | u32 |
| pattern n·n; free·n | `checked_mul` (FK/source.rs:454, :358) | 36,864 | usize |
| support components 6g | `checked_mul(6)` (FK/product_certificate/final_case.rs:1132, 1180) | 192 | usize |
| residual m and 64m | `checked_mul(64)` with `CountRange("residual multiplier")` / `("pivot multiplier")` (FK/adaptive.rs:1551, 1671, 1722; FK/factor.rs:460) | m is an operation count, not an input count; the check stops it before any wrap | u64 |
| tracker sequence | `offered.checked_add(1)` with `CountRange("tracker sequence")` (FK/adaptive.rs:716–720) | ≤ the number of offers, bounded by the schedule | u64 |
| the rest of the retained kernel | 57 `CountRange` stops across FK/retained (a search count) | — | — |

The work counters themselves are covered by checked/sticky custody (D4_RECONCILIATION.md §2), not by this table.

**Remaining:** none for D1. The table is the "closed pre-execution scalar admission" that D-4 relies on.
