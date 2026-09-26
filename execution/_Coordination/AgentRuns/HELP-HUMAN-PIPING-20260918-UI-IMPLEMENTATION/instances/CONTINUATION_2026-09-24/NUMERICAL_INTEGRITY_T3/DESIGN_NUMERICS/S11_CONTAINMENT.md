# V1-S11 — cancelled load contributions: exact ledger and exact recovery

D1 (TASK), 2026-09-26. **Revision 2.** Revision 1 (sha256 `616214b2…`, committed at `70f56b83e`) is kept as `_run_records/S11_CONTAINMENT_revision1.md`.

This revision answers:
- V1's check of revision 1, `T3/REVIEW/S11_CHECK.md` at `56b651282` (sha256 `47b6fff2…`), verdict BLOCKING;
- ROOT's rulings on that check, and ROOT's adopted no-interim text, in `T3/ROOT_RULINGS_V1.md` at `2d07cad7f` (sha256 `d32b5589…`);
- the T3 manager's relay of S11-V1 to S11-V7 and ROOT's seven additions.

- **Basis.** Product source `c61a540ea`, unchanged at branch head `2d07cad7f` (the diff under `P/core` and `P/fixtures` is empty). T1 is `f3270ea79`, read only with `git show`.
- **Paths.** `P/`, `PP`, `FK` and `SA` are as in `DESIGN.md`. `SP` means `P/core/solver/straight_pipe/src/lib.rs`. `CB` means `P/core/solver/curved_bend/src/lib.rs`.
- **Scope of work.** Read-only on product source. I ran standard-library Python probes and scans only (§12). The host is still held, so nothing was built and no product test ran. No Git write was made.

## 0. What changed from revision 1

| Item | Revision 1 | Revision 2 | Where |
|---|---|---|---|
| S11-V1 (BLOCKING), ROOT 2 | The ledger covered the force vector only | Every recovery-side load sum is exact and rounded once, using the ledger's accumulator and rounding. Fourteen sums are enumerated, going beyond V1's list. Probe A becomes a test, with a mutation restoring the fold that must fail | §2.2, §4.4, §9 |
| ROOT 1 | — | The invariant "never newly silent", stated, bounded and tested on probe A in five loadings and three orders | §5.3 |
| S11-V2, ROOT 3 | The audit was the guard | The force vector can be built only from the ledger (private type), and a test enumerates every accumulation site. The guard is defence in depth, and its floor is stated | §4.3, §5.1 |
| S11-V3, ROOT 3 | Granularity undefined | Contribution granularity is defined per producer. Pre-summing producers push individual products | §4.2 |
| S11-V4, ROOT 4, ROOT 5 | `exact_rounded_sum` named, not specified | One correctly rounded accumulator (from `pressure_sum::exact_sum`), +0.0 for an exact zero, used at every site. T1's three sites named. V1's 0.4.0 test added. `Expansion::rounded()` usage stated, **with a new finding** | §4.1, §4.5, §7 |
| ROOT 5 (signed zero) | — | +0.0 for every new and ledger value. **Refinement proposed:** where a replaced expression publishes −0.0 today, keep that zero, because committed fixtures contain 245 such values | §4.1.3, D-S11-1 |
| S11-V5 | Nonlinear loop "repaired too" | Friction terms and the absent in-loop guard are recorded as open with T5 | §10 |
| S11-V6, ROOT 7 | "Three or more contributions" presented as a narrowing | Exposure is governed by the gross-to-net ratio and the order of addition. ROOT's adopted text is quoted | §2.3 |
| S11-V7, ROOT 6 | Fixture byte-identity claimed | Bit changes in real user models are disclosed. The fixture diff is the evidence, and it has a stop rule where committed recovery sums could change | §8.3 |
| R1 finding 4 | — | The row scale is related to R1's net-governed RF-CANCEL scale | §5.2 |

## 1. Answer in brief

1. **The repair has two halves, and it needs both.**
   - **Force side (the ledger).** Every force contribution goes, term by term, into one per-case exact ledger. The solve's force vector is each DOF's correctly rounded net.
   - **Recovery side (new).** Every post-solve sum that folds element loads is one exact sum of its individual terms, rounded once, through the same accumulator. This covers member end forces, station resultants, stress-extrema intensities, curved-bend intensities and sections, restrained reactions and load-case combinations.
   - With only the force side, V1's probe A publishes root shear 1.24e-9 off at G = 1e7. With both halves, every probe-A quantity is within 4.6e-16 of its own magnitude up to G = 1e80 (§5.3).
2. **Bypass is impossible by construction.** The case force vector is a type that only the ledger can build. A source-scan test enumerates every accumulation site, so a producer that skips the ledger fails to compile or fails that test. The M03 audit remains as a guard, with a stated floor (§5.1).
3. **One correctly rounded function.** It is `FK`'s new `exact_sum` accumulator, moved from `P/core/product_physics/src/pressure_sum.rs`: a fixed-point sum, rounded to nearest-even. An exact zero is +0.0. It replaces `Expansion::rounded()`, which is not correctly rounded, at all three `FK` sites.
4. **New finding (§7).** `Expansion::rounded()` feeds a published, byte-compared value: the M03 intended-action `ResidualRow` in the `Debug` structural report embedded in diagnostics. It publishes **−0.0** for an exact-zero residual, because the standard library's float `Sum` starts from −0.0. There are 245 such values in committed fixtures. Moving that site to +0.0 would change them, so D1 proposes keeping the existing zero there (D-S11-1).
5. **Landing.**
   - **S11-K** goes first once the host is released. It is T1-disjoint. It is live in `SP` (exact recovery), in `load_case_algebra` (exact combination) and at the `FK` rounding sites; everything else in it is dormant.
   - **S11-F** is the first facade slice after T1 merges. It covers the ledger, the `PP` recovery composition and T1's three sites.
   - The ROOT text below governs: no interim containment before T1 merges. S11-K's live parts are repairs in T1-disjoint files, as ROOT's disposition of S11-V1 allows for `SP`. `load_case_algebra` and the `FK` sites are D1's additions (D-S11-3).
6. **A detected loss** still makes the case Sensitive, never refused. It carries `NUMERICAL_INTEGRITY_SENSITIVE` plus the new warning `LOAD_CONTRIBUTION_ABSORBED` (§6). Under the design this happens only when a mutation or a future defect bypasses the ledger.

## 2. Reach

### 2.1 Force side (revision 1's table, confirmed by V1)

| # | Finding at `c61a540ea` |
|---|---|
| R1 | Nodal loads are folded in binary64 in contribution order by `LoadApplication::global_load_vector` (`P/core/loads/primitive_loads/src/lib.rs:1401-1409`), called at `PP:1810` |
| R2 | Every `PP` site that accumulates into the force vector: `:1840` (exact-pressure totals), `:7759`, `:7797` (element equivalents, one load at a time), `:8000-8001`, `:8068-8069` (thermal and thrust axial pairs), `:8035-8036`, `:8040` (curved thrust and wall loads), `:8086-8090` (curved thermal, pre-summed), `:9808` (constant effort). T1 adds its eigen equivalents through `add_thermal_equivalent_loads` |
| R3 | The M03 audit starts from `system.force[i]` (`FK/structural.rs:513-538`) |
| R4 | Retained source carries each nodal load, and on T1 each eigen term, as an exact term (`source_recovery.rs:584-588`; T1 `:1270-1287`) |
| R5 | `source_recovery.rs:580-595` (T1 `:609-667`) re-folds in binary64 and compares bits with the actual force |
| R6 | Receipt replay rebuilds the force with `global_load_vector` (main `source_receipt.rs:154`; T1 `:320`, plus T1's 0.4.0 replay `:218-219`). Replay is in-run only (called from `source_receipt.rs:472` and `composite.rs:692`), so no historical artifact is re-verified by it |
| R7 | The nonlinear loop consumes the same vector. Restrained-DOF loads enter the published reaction directly (`PP:2131-2140`) |
| R9 | A binary64 fold of one or two terms is correctly rounded, so only a sum of three or more terms can differ. That holds for recovery sums as well |
| R10 | Committed models reach at most two contributions per force DOF (D1's nodal scan and V1's all-producer scan). **Recovery sums are different:** `fixtures/product_preview/invented_preview_model.json` (and its result export) puts three element loads on `pipe:P-120` in `load:L-100` (weight −190 N/m, pressure 1.2e6 Pa, thermal 12.5 °C) and two in `load:L-200`. Nine element-case pairs carry two or more element loads (`_run_records/scan_element_loads.*`). Their end-force sums have up to four terms |

### 2.2 Recovery side: every load sum after the solve (S11-V1: "enumerate every recovery-side load sum")

"Load sum" means a sum in which two or more operands carry load contributions, or a load contribution meets the elastic term. Each row becomes one exact sum rounded once (§4.4).

| # | Site | What is folded today | Slice |
|---|---|---|---|
| E1 | `SP:568-585` `equivalent_nodal_loads_with_spans` (via `add_spanned_uniform_equivalent_load` `:1167-1217` and `add_point_equivalent_load` `:1219-1253`) | Every load's fixed-end terms, with `+=` into one `[f64; 12]` | S11-K |
| E2 | `SP:649-661` `equivalent_local_axial_effect_loads` | Axial effects, `+=`/`-=` | S11-K |
| E3 | `SP:1150-1164` `apply_load_and_axial_corrections` → `subtract_equivalent_loads` (`:1364`) twice | local − load equivalent − axial equivalent, two roundings | S11-K |
| E4 | `SP:934-968` `station_resultants_from_i_end_with_spans` (via `accumulate_spanned_uniform_station_resultants` `:1255-1284`, `accumulate_point_station_resultants` `:1286-1309`), and the `recover_station_*` family `:1006-1148` | i-end action, `V·d`, and every load's station term | S11-K |
| E5 | `PP:2340-2341` `mechanical = local_forces − equivalent`, then `corrected_local_forces_for_axial_effects` `PP:8109-8128` | Elastic end force, minus E1, plus thermal and thrust axial loads (`.sum::<f64>()` at `:8116-8120` and in `pressure_thrust_for_pipe` `:8347-8353`) | S11-F |
| E6 | `PP:2430`, `:2496`, `:7490-7519` `straight_section_resultants` | E4 from the published end forces | S11-K (the sum is in `SP`) |
| E7 | `PP:7551-7562` extrema spans | `w[axis] += force_per_length` over the loads active on a sub-span | S11-F |
| E8 | `PP:8166-8175` `recover_curved_bend_local_forces` | `thermal_strain` pre-summed over loads, then K·(d − u_free) | S11-F |
| E9 | `PP:8178-8201` same function | Global end forces minus the equivalent of the **summed** intensity, minus the radial-pressure equivalent of the **summed** thrust | S11-F (with `CB` terms API in S11-K) |
| E10 | `PP:8226-8254` `curved_bend_uniform_intensity_by_pipe` | Intensities of all uniform loads on a curved span. The force side forms one equivalent per load (`PP:7746-7760`); recovery uses the sum | S11-F |
| E11 | `CB` `arc_section_resultants_with_radial_pressure` (called from `curved_bend_section_resultants`, `PP:8267-8308`) | End-j force, summed intensity and summed thrust, in one section evaluation | S11-F (with `CB` terms API in S11-K) |
| E12 | `PP:2135-2139` restrained reactions | `K·u − force`, with the folded force | S11-F |
| E13 | `P/core/loads/load_case_algebra/src/lib.rs:340` `evaluate_linear_combination` | `value += fl(c·q)` over combination terms | S11-K |
| E14 | `nonlinear_integration/src/lib.rs:1642-1659` | Friction forces added to force and reactions | Open with T5 (§10) |

**Declared formation steps (not load sums).** Each forms one contribution from one source's own inputs. Conditioning is stated, and none is a cross-source sum.
- **Fixed-end and station coefficients** in `SP` (`b³ − a³` and similar geometric polynomials, the lever integral at `SP:1268-1269`). Each multiplies one load magnitude once. Short partial spans lose relative accuracy in the coefficient. That is a formation-accuracy item recorded in §10, not an absorption.
- **The self-weight mass** (`P/core/product_physics/src/self_weight.rs:475-485`) is a same-sign sum of metal, contents and insulation masses. Its relative error is at most 2u.
- **Curved consistent equivalents** inside `CB` (`consistent_uniform_nodal_loads`, `consistent_radial_pressure_nodal_loads`) form one source's vector from its own magnitude.
- **The stress bound's signed sums** (`PP:7662-7670`). The maximum of |A|+|B|+|C| is attained by the same-sign pattern, which does not cancel.
- **Transforms and elastic dot products** (`SP:1320`, `:1348`, `:1358`; `K_e·u`; the chord rotation at `PP:8203-8214`). These are rotations or stiffness actions, not load contributions. Their accuracy belongs to W1 (`DESIGN.md` §4.1.5 recovers them at precision p). They are kept bit-identical here so that S11 changes nothing else.

### 2.3 Exposure (S11-V6; ROOT's adopted text governs)

> No interim measure before T1 merges. The error is at most about half a unit in the last place of the gross load on the DOF, so it reaches 1e-9 of the net only when gross exceeds net by about 1e7 in an unfavourable order. The fix repairs rather than contains. Reopens if T1's merge slips materially or P1 finds a Passed breach in a realistic model.

- **The count is not the filter.** Three or more contributions per DOF, or per recovery sum, are normal in real models. Examples: two loaded spans meeting a nodal load; pipe, contents and insulation loads on one member; the two thermal pair terms at any interior node of a hot run.
- **What governs exposure** is the gross-to-net ratio and the order of addition.
  - An error needs a small term added **before** a cancelling pair, carrying bits below ulp(gross).
  - Orders in which the pair cancels first are exact (Sterbenz). So are integer-valued small terms below 2^53.
- **A realistic breach.** A 4.1e7 N thermal pair (E·A·α·ΔT for a large-bore hot line) after a 1.3 N co-axial load folds to 1.2999999970197678 N. That is a relative error of 2.29e-9, about 3e-9 N absolute (probe `SUM`). The exact sum gives 1.3.
- **Whole-response loss** needs a ratio of 2^53 or more.
- **Probe A in R1's three orders** (§5.3): (G, n, −G) and (n, G, −G) breach at G ≥ 1e7; (G, −G, n) is exact at every G.

## 3. Options (revision 1's verdicts stand)

- **C1** (exact sum inside `global_load_vector` alone): **rejected.** It knocks out retained recovery, and through the composite that yields an invocation-level `Err` (`PP:1249-1253`) or a partial source-blocks-1 receipt.
- **C2** (a blocking load finding): **rejected.** It blocks the whole envelope (`PP:1430-1432`, `:1497-1499`).
- **C3-full: chosen, now defined as ledger plus exact recovery (§4).** Force and recovery sums are repaired. No case moves from Passed to Sensitive. No path to the composite `Err` is added, provided every site moves together (§4.5).
- **C3-detect: fallback only,** with or after D2's S-D. It keeps every value and demotes flagged cases. It is also the comparator of the invariant (§5.3).

## 4. The rule

### 4.1 One correctly rounded function (S11-V4; ROOT 2, 4, 5)

#### 4.1.1 Definition

`FK` gains a public module `exact_sum` (new file `FK/exact_sum.rs`). It is dependency-free, and every crate involved already depends on `FK`: `primitive_loads`, `SP`, `CB`, `nonlinear_integration` and `product_physics`. `load_case_algebra` gains a direct `FK` path dependency.

- **`ExactAccumulator`** is a signed fixed-point integer accumulator.
  - It is generalized from the 34-limb accumulator of `pressure_sum::exact_sum` (`P/core/product_physics/src/pressure_sum.rs:1-125`, T1-disjoint).
  - It is widened so that exact products of two binary64 values fit: exponent span 2^-2148 to 2^2048 plus carry room, 68 limbs of 64 bits.
  - `add(x: f64)` adds x exactly.
  - `add_product(a: f64, b: f64)` adds a·b exactly, through the 106-bit integer product of the significands. No FMA underflow case exists.
  - `round(&self) -> Result<f64, SumError>` rounds to nearest, ties to even. `pressure_sum`'s `project` does the same.
- **`exact_rounded_sum(values)`** and **`exact_rounded_dot(pairs)`** are thin wrappers. There is one rounding path.
- **Errors.** `SumError::{NonFinite, AccumulatorOverflow, NonRepresentable}` are unchanged from `pressure_sum`. A net outside the binary64 range is `NonRepresentable`, never ±∞. Each call site maps it to the diagnostic its current non-finite path already uses: `require_finite_mechanics` and `validate_finite_array`.
- **Exact zero.** An exact zero returns +0.0, for any operand signs and for the empty sum. `float(Fraction(0))` behaves the same way in the probes.
- **`pressure_sum::exact_sum`** becomes a wrapper over `exact_rounded_sum`. Its existing tests stay, byte for byte.

#### 4.1.2 Where it replaces something

| Site | Today | With `exact_sum` |
|---|---|---|
| The ledger, the recovery sums E1 to E13, and `source_recovery`/`source_receipt` (§4.5) | binary64 folds | `ExactAccumulator` |
| `FK/structural.rs:369-377` `exact_scalar` | `rounded()`, then an exact check of the difference | `round()`, then the same check. It is sound either way. It now withholds less often (§7) |
| `FK/structural.rs:406-414` absorbed-diagonal screen | `rounded()` before and after the contribution | `round()` before and after |
| `FK/structural.rs:554` intended-action residual `r` | `rounded()` | `round()`, with the zero rule of §4.1.3 |
| `FK/structural/exact_boundary.rs:361`, `:387` coverage checks | naive projection or ordered fold, compared with `!=` | also accept `round()` of the same terms (S11-K) |
| `exact_boundary.rs:218-229` `approximate_projection` (used at `:726`, `:1025`) | naive quotient | unchanged. It is documented as a proposal that is verified afterwards, so it makes no rounding claim |

`Expansion::rounded()` is removed, so no site can keep using it.

#### 4.1.3 Signed zero: ROOT's +0.0, and one proposed refinement (D-S11-1)

- **Ledger.** +0.0 always. Today's fold starts from a +0.0 accumulator (`global_load_vector`), so it never yields −0.0 either. No byte changes.
- **New quantities.** +0.0.
- **Replaced expressions that publish −0.0 today.** Two kinds exist:
  - `FK/structural.rs:554`. An exact-zero residual is an empty expansion, and `rounded()` is `iter().sum()`, whose neutral element on the pinned toolchain is −0.0. Committed fixtures hold 245 `normalized_residual: -0.0` values in the intended-action rows (for example, `n05-dense_scrutiny.raw.json`, `ResidualRow { global_dof: 6, residual: -0.0, … normalized_residual: -0.0 …}`).
  - Recovery sums that start from their first operand: stations from the i-end action (E4), and `local − equivalent` (E3, E5). These pass a −0.0 operand through when every other term is zero. Committed raws hold −0.0 member stresses (7 in `multicase-dense_scrutiny.raw.json`). Most probably come from the j-side negation after the sum (`PP:7509-7519`, for example `-section.axial_force` at `:7512`), but the source alone does not prove that.
- **Proposed refinement (the zero witness).** At these sites, an exact zero publishes the zero that the replaced binary64 expression gives, when that expression is itself zero. When the expression is nonzero (a repaired absorption), the exact zero publishes +0.0.
  - At `FK:554` the witness is the constant −0.0 for an empty expansion. It is written as a literal with a comment naming the committed bytes, not left to `Iterator::sum`.
  - At E3, E4 and E5 the witness is the existing expression, evaluated alongside.
- **Effect.** Any value whose fold was already correct keeps its bits, including the sign of zero. The rounding function itself stays single, and only the zero's sign is taken from the witness.
- **If ROOT keeps +0.0 everywhere,** the 245 residual strings (and any recovery −0.0 that passes through a sum) change sign in S11-K and S11-F's fixture diffs. They would then be regenerated only by the actual producer, with the change disclosed.

### 4.2 Contribution granularity (S11-V3; ROOT 3)

**Definition.** A contribution is one binary64 value that one producer forms from one source's inputs for one DOF, by a product, a quotient or a closed-form coefficient times the source's magnitude. The value is formed exactly as today, so the represented contribution is unchanged. Any addition across sources is the ledger's. So is an addition across several load-proportional products of one source.

- **Rounded formed terms** are pushed with `push(source, dof, value)`.
- **Load-proportional products that today feed a within-source sum** are pushed exactly with `push_product(source, dof, a, b)`.

| Producer (site) | Pushed | Note |
|---|---|---|
| Nodal loads (`primitive_loads`) | one term per load | as today |
| Straight element equivalents (`PP:7789-7798`) | one term per (load, DOF) | the `SP` formula for one load is formation |
| Curved uniform equivalents (`PP:7746-7760`) | one term per (load, DOF) from `consistent_uniform_nodal_loads` of that load alone | as today |
| Straight thermal and thrust pairs (`PP:7996-8001`, `:8064-8069`) | fl(P·x_a) per axis | formation: one product |
| Curved thrust caps and wall vector (`PP:8033-8041`) | fl(P·t_a) per axis; one term per wall slot | as today |
| **Curved thermal** (`PP:8078-8091`) | **`push_product(K_rc, fl(ε·chord_c))` per nonzero column**, instead of the pre-summed `value` | V1's S11-V3 example. The equivalent becomes the exact K·u_free of the represented u_free |
| **Exact pressure** (`PP:1838-1842`, from `pressure_runtime.rs:800-846`) | **each group operand**, instead of the group's pre-summed `assembled_loads` total | the groups are already exact, but pushing their totals would pre-sum |
| Constant effort (`PP:9805-9812`) | one term per application | as today |
| T1 eigen equivalents (`add_thermal_equivalent_loads`; `source_recovery.rs:1270-1287` at T1) | fl(P·x_a) per axis, as the straight thermal pair | both sides push the same terms |

Recovery sums use the same granularity (§4.4), so force and recovery see the same represented contributions.

### 4.3 The force vector can be built only from the ledger (S11-V2; ROOT 3)

- **Types (in `primitive_loads`, S11-K).**
  - `LoadLedger` records `(source: String, dof, kind: Term | Product)` entries. It offers `push`, `push_product`, `terms()`, and `finish(n) -> Result<AssembledForce, LedgerError>`.
  - `AssembledForce` wraps a private `Vec<f64>`. It has no public constructor, no `From`/`Default`/`Deserialize`, and no `&mut` access. It offers `values(&self) -> &[f64]` and `terms(&self) -> &[ForceTerm]`.
- **Seams that take it (S11-F).**
  - `PP`'s case-solve helpers take `&AssembledForce` in place of `&[f64]`. They hand `values()` to `FK`, sparse and nonlinear, and `terms()` to `AssemblyEvidence::with_force_terms`.
  - `source_recovery::Input.force` becomes `&AssembledForce`.
  - Both receipt replays build through the ledger, using the same producer functions as the live path.
  - A producer that skips the ledger therefore does not compile.
- **Enumerated site test (S11-F).** A `PP` unit test reads `lib.rs`, `source_recovery.rs`, `source_receipt.rs`, `pressure_runtime.rs` and `self_weight.rs` with `include_str!`. Outside `#[cfg(test)]` blocks it asserts:
  1. no `global_load_vector(` call;
  2. no compound assignment into an indexed force (the regex `force\w*\s*\[[^\]]*\]\s*[-+]?=`);
  3. the set of functions that call `.push(`/`.push_product(` on a `LoadLedger` equals the enumerated producer list of §4.2, by function name;
  4. every function with a `&mut LoadLedger` parameter is in that list.
  
  Adding a producer therefore means editing the list, which a reviewer sees.
- **`global_load_vector`** stays for non-product callers, documented as "binary64 fold; not for solve input". The test forbids it in `PP`.
- **The guard is defence in depth.** It flags the mutations and defects the construction does not prevent: a producer that pushes a wrong or pre-summed value, or a site added inside a listed function. Its floor is in §5.1.

### 4.4 Exact recovery sums (S11-V1; ROOT 2)

**Rule.** Each published recovered quantity is one `ExactAccumulator` sum over all its terms, rounded once. The terms are:
- the formed elastic term, bit-identical to today's `K_e·u` (or, curved, the `K·d` products, which are pushed exactly);
- minus every load's equivalent term (§4.2 granularity);
- plus every axial-effect term (each thermal load and each pressure-thrust load individually);
- for stations, the published i-end action, fl(V·d), and each load's station term.

| Sum | Terms of the one exact sum |
|---|---|
| E1 (`SP`) | the per-load fixed-end terms. For one load it equals today's value bit for bit, which is why the force side (`PP:7789`, one load per call) is unchanged |
| E2, E3 (`SP`) | `local_i`, −each load term, −each axial term. One rounding replaces two |
| E4, E6 (`SP`) | `M_i`, fl(V·d), each load's fl(w·lever) or fl(F·lever). Likewise for N, V and T |
| E5 (`PP`) | `local_i`, −each load's E1 term, +each thermal `axial_load` and +each thrust `axial_load` on UX rows. This replaces `mechanical` and `corrected` together. `SP` gains `equivalent_nodal_load_terms_with_spans` returning one `[f64; 12]` per load |
| E7 (`PP`) | each active load's `force_per_length` |
| E8, E9 (`PP`) | the exact products K_rc·d_c; −K_rc·fl(ε_l·chord_c) for each thermal load l; −each uniform load's own equivalent (per load, as on the force side); −each thrust load's radial-pressure equivalent. The chord rotation that follows stays a formed transform |
| E10 (`PP`) | removed. Recovery uses per-load equivalents, as the force side does |
| E11 (`PP` with `CB`) | linearity: the section value is the exact sum of `CB`'s section function applied to the end-j force alone, to each load's intensity alone, and to each thrust alone. `CB` gains `arc_section_resultant_terms` (S11-K, dormant until S11-F) |
| E12 (`PP`) | the formed `K·u` term, and −each ledger term of that DOF |
| E13 (`load_case_algebra`) | `add_product(c_i, q_i)` per term, so the combination is exact. That is the binary64 counterpart of `DESIGN.md` §4.1.1's exact `RetainedCombination::form` |

- **What stays rounded, and why.** Stations consume the published, once-rounded end force, as today. Section statics (M_i − V·x + w·x²/2) can cancel physically, and that is W1's recovery accuracy (`DESIGN.md` §4.1.5, at precision p), not load absorption. S11 makes the load part of every sum exact and leaves the formed terms bit-identical.
- **Relation to `DESIGN.md`.** These are the binary64 path's repairs. The F-slices then recover at precision p with the same accumulator, and supersede them.

### 4.5 T1's sites (S11-V4), all in S11-F

| Site at T1 | Change | If missed |
|---|---|---|
| `source_recovery.rs:609-667`, including `close_load_state`'s eigen fold `:1270-1274` | `folded_force` becomes a `LoadLedger` over the same nodal and eigen terms. The comparison is `finish()`'s bits against `input.force.values()` | 0.4.0 cases with eigen and nodal terms on a shared DOF lose selection (declared, a regression) |
| `source_receipt.rs:218-219` (0.4.0 replay: `global_load_vector` + `add_thermal_equivalent_loads`) | build through the ledger with the live producer functions | joined finalization fails, and SF-1 republishes ordinarily (declared, and it knocks out a correct result) |
| `source_receipt.rs:320` (pre-0.4 replay) | build through the ledger | **pre-0.4 finalization fails and reaches `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")`, which ROOT forbids** |
| main `exact_boundary.rs:361`, `:387` | also accept `round()` of the same terms (S11-K) | selection fails where the exact net differs from the fold |

Exact sums do not depend on order, and correct rounding is unique. The four sites therefore agree bit for bit when they hold the same term set. That holds within retained-source scope: nodal terms on main, plus the eigen terms on T1. Replay is in-run (§2.1 R6), so historical receipts are not re-verified.

## 5. Guard, scale and invariant

### 5.1 The guard and its floor (S11-V2)

- **The predicate is M03's intended-action predicate,** unchanged in form, with the load term replaced by the ledger's exact per-DOF sum:
  - `ratio_i = |Σ_j K_ij·u_j − f_i^exact| / d_i`, with `d_i = |f_i^exact| + Σ_j |K_ij|·|u_j|`;
  - flagged when `ratio_i > 64·γ(m_i)`;
  - applied to free and restrained rows.
- **Floor, stated.** The guard flags a net-load error e_i only when `e_i > 64·γ(m_i)·d_i`. Below that it is silent. A published quantity governed by that DOF's net then carries a relative error of at most `64·γ(m_i)·(d_i/|f_i|)`. It is guaranteed within 1e-9 only when the row amplification is `d_i/|f_i| ≤ 1e-9/(64·γ(m_i))`.
  - In a committed report, m_i = 92 gives a target of 6.54e-13 (`n05-dense_scrutiny.raw.json`), so the guard is complete up to an amplification of about 1.5e3.
  - Probe A's tip rows have amplifications of 6 and 18.
  - V1's probe B (a 1e6 N·m moment at the same node, amplification about 2e7) is beyond the floor, which is why it passes.
- **Why the floor is acceptable.** Under C3-full the guard never has to carry 1e-9. Every ledger-routed and recovery-routed load is exact by construction (§4.3, §4.4). The guard's job is to catch mutations and future bypass defects, and it catches gross ones (a missing producer) regardless of the floor.

### 5.2 Relation to R1's net-governed scale for RF-CANCEL (R1 finding 4)

- **R1's recommended RF-CANCEL scale** (`T3/REFERENCES/README.md` §4 and the RF-CANCEL notes) is net-governed: each value is compared relative to its response to the net alone.
- **The row scale d_i** is a runtime detection scale, not an acceptance scale.
- **Under C3-full the acceptance does not depend on d_i.** The ledger gives the correctly rounded net, and recovery sums the load terms exactly. So every net-governed RF-CANCEL value meets `|obs − exp| ≤ 1e-9·max(|exp|, scale)`, up to the solve's ordinary accuracy. Probe A's errors are at most 4.6e-16 of the value's own magnitude.
- **On the guard path only** (a mutation), d_i matters. The guard flags every net-governed RF-CANCEL failure when `d_i/|f_i| ≤ 1e-9/(64·γ(m_i))` (§5.1).
  - For R1's nodal cases (a tip DOF of a cantilever) and UDL cases (the shared-node rotation row, whose Σ|K||u| is net-governed), the amplification is small.
  - A case like V1's probe B exceeds it. R1's net-governed scale would fail it. The guard would not flag it, but the construction prevents it.
- **Under the class (norm) scale,** R1 notes the loss can hide. The design does not rely on that scale for RF-CANCEL.

### 5.3 The invariant: never newly silent (ROOT 1)

- **Statement.** For every case that C3-detect would flag as load-contribution loss, C3-full publishes each quantity either within 1e-9 of its own magnitude, or flagged.
- **Why it holds, for the load-contribution class.**
  - C3-full's force is the exact net, and its recovery sums are exact.
  - The only remaining load-dependent errors are the formed terms' own roundings. Each is within a few u of that term.
  - Any unrouted producer still meets the same audit C3-detect uses, over the same exact terms.
- **Boundary.** Errors of other classes are neither introduced nor detected by S11. Examples are a structurally ill-conditioned recovery or cancellation in section statics. A C3-detect flag incidentally covers them. Under C3-full they keep the standing they have in any Passed case today, which is W1's concern. The invariant is claimed for the class S11 addresses.
- **Test, probe A** (`_run_records/probe_s11_rev2.*`). One 2 m cantilever element carries three uniform local-y loads. C3-detect is emulated as a solve on the binary64 fold with the exact audit. C3-full is emulated as the exact ledger solve with exact recovery. The outputs are root shear, root moment, midspan shear, midspan moment and the extrema intensity:

| Loads (N/m, authored order) | C3-detect | C3-full worst error (own magnitude) | Mutation: recovery fold restored | Killed at 1e-9 |
|---|---|---|---|---|
| (1e5, 0.3, −1e5) | flags (37.9 × target) | 4.6e-16 | 3.9e-11 | no |
| (1e6, 0.3, −1e6) | flags (607) | 4.6e-16 | 6.2e-10 | no |
| (1e7, 0.3, −1e7) | flags (9.7e3) | 4.6e-16 | 6.8e-9 | **yes** |
| (1e8, 0.3, −1e8) | flags (3.9e4) | 4.6e-16 | 4.0e-8 | **yes** |
| (1e80, 1e-8, −1e80) | flags (2.3e13) | 2.0e-16 | 2.3 | **yes** |
| (1e7 / 1e8, −G, 0.3) | passes | 4.6e-16 | 4.6e-16 | no (the fold is exact) |
| (0.3, 1e7 / 1e8, −G) | flags | 4.6e-16 | 6.8e-9 / 4.0e-8 | **yes** |

Result: the invariant holds in every row. The product tests therefore use G = 1e7, 1e8 and 1e80, where the mutation is killed.

## 6. What a detected loss does (unchanged from revision 1 §4)

- **Kernel.** `finish_checked_factor` returns `Ok` with `SolveQuality::Sensitive`. A separate `LoadFidelityReport` records, per flagged row: DOF, exact and actual values as bit strings, ratio, target and source ids. The `Debug`-published report is byte-unchanged when nothing is flagged.
- **Facade.** `NUMERICAL_INTEGRITY_SENSITIVE`, plus the new warning `LOAD_CONTRIBUTION_ABSORBED`. Its `affected_refs` are the case and load ids (ROOT's F-1 amendment).
- **Standing** is today's standing: envelope-level Current is withheld, rows are kept, and the envelope is not blocked.
- **Never a refusal,** because a blocking diagnostic blocks the whole envelope.

## 7. `Expansion::rounded()` on main (ROOT 4): new finding

**Where it is used** (`FK/structural.rs:366-368`, `pub(crate)`, `self.terms.iter().copied().sum()`):
1. **`exact_scalar` (`:369-377`)**, used by `FK/rigid_body.rs:240` (the published rigid-motion witness) and tests.
   - It is sound: it verifies the candidate exactly and returns `None` otherwise.
   - A naive miss withholds a witness that could be published. Nothing wrong is published.
2. **The absorbed-diagonal screen (`:406-414`).** A naive-rounding artifact can decide "absorbed" (a spurious `NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED`) or miss one. A miss is still covered by the intended-action audit, which uses exact contribution sums.
3. **The intended-action residual (`:554`): published and byte-compared.**
   - `r` becomes `ResidualRow.normalized_residual`, and through `physical_residual_record` the row's `residual`. It also enters the gate ratio.
   - The rows are rendered with `{:?}` inside diagnostic messages (`PP:883`, `:2046`, `:2057`).
   - 37 committed fixtures contain them, and fixture tests compare them byte for byte.
   - **The gate decision is protected:** the ratio adds `g/(1 − g)` with g = γ(operations), and the operation count includes the expansion's operations. The published value can differ from the correctly rounded residual in the last bits.
   - **The zero is −0.0.** An exact-zero residual is the empty expansion, and the standard library's float `Sum` starts from −0.0. That gives the 245 `-0.0` values of §4.1.3, next to +0.0 in the ordinary residual rows. The published zero sign is therefore a standard-library behaviour rather than a design choice, and pinning the toolchain is what keeps it stable.
- **Finding for the map (N-S11-R).** A published, byte-compared value depends on a non-correctly-rounded sum and on the standard library's float-sum neutral element.
- **Disposition in S11-K.** Replace it with `round()` plus the zero witness (D-S11-1). The committed-fixture diff shows whether any nonzero residual changes. For short, nonoverlapping expansions it is expected not to change, but that is unproven until the run.

## 8. Slices, serialization and disclosure

### 8.1 S11-K (first, once the host is released; T1-disjoint)

- **Write set:**
  - `FK/exact_sum.rs` (new) and `FK/lib.rs` (module);
  - `FK/structural.rs`: `rounded()` replaced at `:369-377`, `:406-414` and `:554`; the audit with exact per-DOF force terms through a new entry point; `StructuralSystem` layout unchanged;
  - `FK/structural/exact_boundary.rs:361`, `:387`;
  - `SA`: `AssemblyEvidence::with_force_terms`;
  - `P/core/loads/primitive_loads/src/lib.rs`: `LoadLedger`, `AssembledForce`, `ForceTerm`;
  - `SP`: E1 to E4, E6, and `equivalent_nodal_load_terms_with_spans`;
  - `CB`: `arc_section_resultant_terms`;
  - `P/core/loads/load_case_algebra/src/lib.rs` and its `Cargo.toml` (E13, `FK` dependency);
  - `P/core/product_physics/src/pressure_sum.rs` (wrapper).
- **Live effect:**
  - `SP`: exact recovery sums in every product path that recovers straight members;
  - combinations;
  - the `FK` rounding sites.
  - Everything else is dormant until S11-F.
- **Serialization.** S11-K → K1 → K2 → K5, because all four write `SA` and `FK/structural.rs` (`DESIGN.md` §6). K3 runs in parallel.

### 8.2 S11-F (the first facade slice after T1 merges, ahead of F1)

- **Write set:**
  - `PP`: the ledger at every §4.2 producer; `AssembledForce` seams; E5, E7 to E12; the Sensitive mapping; the site test;
  - `P/core/product_physics/src/pressure_runtime.rs` (push group operands);
  - `source_recovery.rs` and `source_receipt.rs` at T1's three sites (§4.5).
- **Behaviour:** C3-full.

### 8.3 Bit changes and disclosure (S11-V7; ROOT 6)

- **Where bits change in real user models.**
  - Any force DOF or recovery sum with three or more terms whose binary64 fold was not correctly rounded, in the last bits.
  - Wherever a load was absorbed, by the full repair.
  - Curved thermal equivalents, which become exact products (§4.2).
  - Combinations with three or more terms, or with non-unit factors.
  - Three or more terms per sum are normal in user models (§2.3), so such changes are widespread in the last bits.
- **Disclosure.**
  - S11-K's and S11-F's change records (the PR body under `.agents/skills/chirality-change/SKILL.md`) state the above plainly: which quantities move, why, by how much in the non-cancelling case (at most one rounding of the gross), and that no case changes status unless it was absorbing a load.
  - Release notes repeat it if a release carries the slices.
  - **No in-band marker** in S11-K or S11-F. Any new envelope field would change every committed envelope. The method identity changes in-band at the F-slices' receipt profile (`DESIGN.md` §5). See D-S11-4.
- **Fixture diff.** Each slice runs every committed request under `P/fixtures/**` through base and candidate, in both modes (T0R and D2 practice). It attaches the diff and a summary by output kind.
  - **The expected result is "unchanged".** Committed force DOFs carry at most two contributions (§2.1 R10). Where they do change, the zero witness keeps the sign of zero.
  - **Stop rule.** `pipe:P-120` in `load:L-100` has four-term end-force sums (E5). Committed Debug residuals may also differ from correct rounding (§7). If the diff shows any committed byte change, the slice stops and reports each change to the T3 manager with its site and reason, **before** any fixture is regenerated. Derived fixtures are regenerated only by the actual producer, after ROOT's decision. Frozen references and historical raws never change.

## 9. Tests and mutations

**S11-K.**
1. `exact_rounded_sum` and `exact_rounded_dot` equal the correctly rounded `Fraction` value for:
   - seeded random terms;
   - V1's probe D, {2^-110, 2^-53, 1} → 1.0000000000000002 (the naive sum gives 1.0);
   - the ≥3-term cancellations of §5.3, and the thermal pair (1.3, 4.1e7, −4.1e7) → 1.3;
   - ties, overflow-adjacent values and subnormal nets (`DESIGN.md` §4.11).
2. Zero: [1e8, −1e8], [−0.0], [−0.0, −0.0] and [] all give +0.0 (bits).
3. `pressure_sum`'s existing tests pass unchanged through the wrapper.
4. `SP`: probe A as a unit test at G ∈ {1e7, 1e8, 1e80} in the orders (G, n, −G) and (n, G, −G). End forces, midspan and quarter stations, and E3, each within 1e-9 of the exact value (`Fraction` reference in the test). For one load, E1 is bit-identical to today.
5. `SP` zero witness: a load-free station whose i-end axial action is −0.0 publishes −0.0. Under ROOT's literal rule, +0.0.
6. `load_case_algebra`: A + B − A2 with (1e80, 1e-8) → 1e-8 (the fold gives 0). 1.3 + 1.35·4.1e7 − 1.35·4.1e7 → 1.3 (the fold gives 1.2999999970197678).
7. `FK`:
   - the audit with force terms flags every probe-A G ≥ 1e5 row and V1's check L, and passes every control;
   - a restrained root row with (1e8, 0.3, −1e8) flags;
   - the `Debug` report is byte-identical when nothing is flagged;
   - `exact_scalar`, the absorbed-diagonal screen and `:554` are byte-identical on every existing `FK` test.
8. `exact_boundary` accepts the correctly rounded sum and refuses a corrupted force.
9. `CB`: `arc_section_resultant_terms` sums exactly to the section value of the combined inputs when that value is exact. Controls use single loads (bit-identical).
10. The committed-fixture diff (§8.3).

**S11-F** (both modes, through `run_linear_static_preview_value_with_mode`).
1. The RF-CANCEL references, once V2 has checked them: every case meets R1's predicate with R1's recommended scales. The binary64-fold negative controls fail in the orders in which they differ.
2. Probe A through the product (a single-element model; G = 1e7, 1e8, 1e80): end forces, stations and extrema within 1e-9. **The invariant test:** the same model's C3-detect verdict, computed from the ledger's `terms()` folded in order and passed to `FK`'s audit entry point, flags. The product publishes Passed with every quantity within 1e-9.
3. The realistic thermal case: a colinear hot run with a 4.1e7 N thermal pair and a 1.3 N co-axial nodal load at an interior node. Ordinary route and 0.4.0 eigen route.
4. Retained source: an N05-class Sensitive case with (1e8, 0.3, −1e8) at the tip is selected, and its answer equals the exact-term answer.
5. **V1's 0.4.0 test:** an eigen pair plus a nodal load at a shared node, with values whose fold is not correctly rounded (eigen terms ±4.1e7·x_a and a 1.3 N nodal load, in that order). The retained join is selected, finalization succeeds, and the published force equals the correctly rounded net.
6. A multi-case pre-0.4 exact-route invocation: selected case A, plus case B carrying the cancelling loads and an element load (so out of retained scope). **No `Err`, and no blocked envelope.**
7. The enumerated site test of §4.3.
8. Curved: a bend with two uniform loads (G, −G) plus 0.3 on the span, and a thermal case. Recovery equals the exact per-load answer.

**Mutations that must fail.**

| # | Mutation | Killed by |
|---|---|---|
| M1 | Restore the binary64 fold in `SP` E1 or E4 | K4 (G ≥ 1e7) |
| M2 | Restore `mechanical` then `corrected` as two folds in `PP` (E5) | F2 |
| M3 | Restore the ledger fold | F1, F2's invariant check, and the audit (the case turns Sensitive with `LOAD_CONTRIBUTION_ABSORBED`) |
| M4 | Add a `force[i] +=` producer, or call `global_load_vector` in `PP` | fails to compile (private `AssembledForce`); F7 |
| M5 | Push the curved thermal `value` pre-summed | F7 (the listed function pushes `push_product`) and F8 |
| M6 | Replace `round()` with an ascending naive sum | K1 (probe D) |
| M7 | Return −0.0 for an exact ledger zero | K2 |
| M8 | Leave `source_receipt.rs:320` on `global_load_vector` | F6 (`Err`) |
| M9 | Leave `source_receipt.rs:218-219` on the fold | F5 (the join is not selected) |
| M10 | Drop restrained rows from the audit | K7 |
| M11 | Restore the combination fold | K6 |

## 10. Open items

1. **Nonlinear loop (S11-V5), owned by T5.**
   - Applied sliding-friction forces are added in binary64 to the loop's force and to the reported reactions (`nonlinear_integration/src/lib.rs:1642-1659`, called at `:754`, `:868`, `:1296`, `:1416`).
   - The loop has no in-loop load guard.
   - The base force it receives is the ledger's.
   - Open until T5 routes friction terms through an exact sum and places the audit inside the loop's linearized solves.
2. **Short partial-extent coefficients.** `SP`'s spanned formulas evaluate differences such as `b³ − a³` directly, so a span fraction of 1e-8 loses about eight digits in the coefficient.
   - A factored form, (b − a)(b² + ab + a²), is accurate but changes full-span bytes unless the full span keeps today's expression.
   - Not S11's class, and not in S11-K. It is proposed as a W1 item with its own fixture diff.
3. **Formed-term accuracy** (elastic dot products, transforms, section statics) stays as today on the binary64 path. W1 and the F-slices cover it.
4. **Non-product crates.** `P/core/loads/user_loads` builds nodal loads from `SP` equivalents (`:700`, `:828`, `:921`). It is not on the product path (`PP` does not depend on it), so it is not in the site test.

## 11. Decisions for ROOT

| ID | Question | Options | D1 recommends |
|---|---|---|---|
| D-S11-1 | Zero sign at replaced expressions that publish −0.0 today | (a) the zero witness (§4.1.3), which leaves committed bytes unchanged; (b) +0.0 everywhere, with 245 residual strings and any passed-through recovery −0.0 regenerated and disclosed | (a). The ledger and all new values stay +0.0 |
| D-S11-2 | `FK` sites of `Expansion::rounded()` | (a) replace at all three in S11-K, with the fixture diff's stop rule; (b) replace only `:369-377` and `:406-414`, and keep `:554` until the F-slices change the report anyway | (a), ROOT's "one function at every site" |
| D-S11-3 | T1-disjoint live repairs before T1 merges beyond `SP` (`load_case_algebra`, the `FK` sites) | (a) in S11-K; (b) move to S11-F | (a). They are repairs, not containment, in files T1 does not touch |
| D-S11-4 | An in-band marker for the summation rule | (a) none until the F-slices' receipt profile; (b) a new field now, which changes every committed envelope | (a) |

## 12. Run records (`_run_records/`)

- **`probe_s11_rev2.py` → `probe_s11_rev2.stdout.json`.**
  - It rebuilds V1's probe A (read from `T3/REVIEW/_run_records/s11/`, not imported) on D1's binary64 element, imported unchanged from `probe_skew_precision.py`.
  - Checks: A2 and INV (the invariant), MUT, ORD, SUM, CMB (combinations), FLR (guard amplification) and ZW (zero witness).
- **`scan_element_loads.py` → `scan_element_loads.stdout.json`.**
  - Every committed JSON under `P/` outside `execution/`, run from `P/` with argument `.`.
  - It finds 183 load cases in 117 files, at most three element loads on one element, and nine element-case pairs with two or more.
- **Revision 1 records kept:** `probe_s11_audit.*`, `scan_load_fold.*`.
- **Other evidence, read with `grep`:** the committed −0.0 counts (§4.1.3, §7) and the `ResidualRow` contexts. The commands are in the D1 transcript, and the counts can be reproduced with `grep -rho "normalized_residual: -0\.0[,} ]" P/fixtures`.
- Python 3.11.15, run at `nice 19`, each under 2 s.
- **Not done:** no product build or run (the host is held); no test of the zero-witness sites against product output; the fixture diff itself (S11-K and S11-F).
