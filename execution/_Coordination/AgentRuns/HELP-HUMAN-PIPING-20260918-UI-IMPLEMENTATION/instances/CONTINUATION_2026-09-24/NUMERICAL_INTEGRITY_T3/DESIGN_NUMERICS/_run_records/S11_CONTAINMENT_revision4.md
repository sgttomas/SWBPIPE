# V1-S11 — cancelled load contributions: exact ledger and exact recovery

D1 (TASK), 2026-09-26. **Revision 4** (narrow), with erratum I1 (the nonlinear typed entry points, §4.3 and §8.1; approved by the T3 manager for the S11-K implementer) and erratum R3B-5/N-4 (V1's BACKCHECK_R3 and ROOT's ruling: the site-test constant is keyed by function plus match count, §4.3 rule 8; S11-K's PR record carries the fixture-diff sizes, §8.3). Revision 3 (sha256 `561c7200…`, committed at `d6575c25e`) is kept as `_run_records/S11_CONTAINMENT_revision3.md`. Revision 2 (sha256 `4bec712c…`, committed at `4663cdbb6`) is kept as `_run_records/S11_CONTAINMENT_revision2.md`. Revision 1 (sha256 `616214b2…`, committed at `70f56b83e`) is kept as `_run_records/S11_CONTAINMENT_revision1.md`.

**Revision 4** answers V1's backcheck of revision 3, `T3/REVIEW/S11_BACKCHECK_R3.md` at `ef8fc4224` (sha256 `d9b077a8…`, verdict FINDINGS; both blockers resolved). It changes only the items in §0. V1 backchecks it together with `DESIGN.md` revision 3.

**Revision 3** answered V1's backcheck of revision 2, `T3/REVIEW/S11_BACKCHECK.md` at `61b228543` (sha256 `a3dfcd78…`, verdict BLOCKING on S11B-1 and S11B-2), ROOT's rulings on it (`T3/ROOT_RULINGS_V1.md` at `45cfc92b1`), and the manager's dispositions of S11B-1 to S11B-9. What changed is in §0.1. V1 backchecks revision 3 on those items only.

Revision 2 answered:
- V1's check of revision 1, `T3/REVIEW/S11_CHECK.md` at `56b651282` (sha256 `47b6fff2…`), verdict BLOCKING;
- ROOT's rulings on that check, and ROOT's adopted no-interim text, in `T3/ROOT_RULINGS_V1.md` at `2d07cad7f` (sha256 `d32b5589…`);
- the T3 manager's relay of S11-V1 to S11-V7 and ROOT's seven additions.

- **Basis.** Product source `c61a540ea`, unchanged at branch head `45cfc92b1` (no commit since `c61a540ea` touches `P/core` or `P/fixtures`). T1 is `f3270ea79`, read only with `git show`.
- **Paths.** `P/`, `PP`, `FK` and `SA` are as in `DESIGN.md`. `SP` means `P/core/solver/straight_pipe/src/lib.rs`. `CB` means `P/core/solver/curved_bend/src/lib.rs`.
- **Scope of work.** Read-only on product source. I ran standard-library Python probes and scans only (§12). The host is still held, so nothing was built and no product test ran. No Git write was made.

## 0. What changed in revision 4 (V1's backcheck of revision 3)

| Item | Change | Where |
|---|---|---|
| R3-1 | KS1–KS3 are bit-identical only where every prescribed value is zero. That holds on main today, but S11-K lands after T1 merges, when T1's support-motion fixtures (nonzero prescribed values) are committed. Those fixtures and every file derived from them are **pre-registered as expected diffs** under the stop rule, with their reason | §2.4, §4.6, §8.1, §8.3 |
| R3-2 | F8 uses (G, 0.3, −G) and cancelling thermal strains, and joins the precondition list, so M1h–M1k are killed. K4 gains an axial-effect case (G, n, −G), so M1b and the axial half of E3 are killed | §9 |
| R3-3 | §2.5's table becomes the site test's constant: every function with a floating-point compound assignment, `.sum` or `fold` in `PP`, `pressure_runtime.rs`, `self_weight.rs`, `SP`, `CB` and `load_case_algebra` must be named as an E-site, a declared formation, an integer or max, or an allow-listed observation. §4.3 limit 4 is corrected | §2.5, §4.3 |
| R3-N1 | The nonlinear influence solves' unit-force vectors (`nonlinear_integration/src/lib.rs:1332-1337`) are allow-listed as T5's | §4.3 |
| R3-N2 | KS1 and KS3 scale by the row's power of two **inside** the accumulator and then round once (`round_scaled(e)`), so a subnormal-range value is not rounded twice | §4.1.1, §4.6 |
| R3-N3 | V1's scan of the files §2.5 omitted is recorded (nothing further) | §2.5 |
| R3-N4 | The per-site mutants are relabelled M1a–M1o (15 E-sites) | §9 |

## 0.1 What changed in revision 3 (V1's backcheck of revision 2)

| Item | Change | Where |
|---|---|---|
| **S11B-1** (BLOCKING) | The prescribed-motion reduced right-hand side `f − Σ K_ic·g_c` is now exact at both `FK` sites (`FK/structural.rs:603-606`, `FK/lib.rs:870-877`), and so is the refinement residual's numerator on rows coupled to a nonzero prescribed value (`FK/structural.rs:697-760`), which would otherwise put the fold error back. Added as KS1–KS3 in the new §2.4; joins S11-K. The 0.4.0 test is V1's probe P model, with a mutation. ROOT: no block on T1; the no-interim ruling is extended to the 0.4.0 support-motion route; S11-K lands soon after T1 merges | §2.4, §4.6, §8.1, §9 |
| **S11B-2** (BLOCKING) | E15 `pressure_for_pipe` (`PP:10780-10802`) and E16 the expansion-joint thrust aggregate (`PP:8368-8382`) added, each an exact sum rounded once. A re-scan of `PP`, `pressure_runtime.rs`, `self_weight.rs`, `primitive_loads`, `FK`, `SA` and `sparse_direct` for any other published sum over more than one source finds none (§2.5) | §2.2, §2.5, §4.4 |
| S11B-3 | The ledger and `AssembledForce` move into `FK`. Every solve seam takes an `FK`-owned, ledger-built type (`StructuralSystem`, `reduce_system*`, `StructuralAssembly::solve`, the nonlinear entry points). The site test is extended to `FK`, `SA` and `nonlinear_integration` and catches copies, `iter_mut` and compound assignment on any binding derived from a force. Stated limits: generic linear-algebra kernels, two protected observation lanes, and T5's in-loop friction | §4.3 |
| S11B-4 | Accumulator spec completed: quantum 2^-2148, rounding at the binary64 subnormal quantum (no copy-bits shortcut), and a nonzero net that rounds to zero gives +0.0 at every site (a stated deviation from IEEE's −0.0 for a negative underflow). V1's probe X cases join K1 | §4.1.1 |
| S11B-5 | Required kill set G = 1e8 and 1e80, one mutant per E-site (M1a–M1p; relabelled M1a–M1o in revision 4) | §9 |
| S11B-6 | F4, F5, F6 and K4 assert as a precondition that the binary64 fold differs from the correctly rounded net. F5 restated: the producer fixes the order (nodal first, then eigen pairs), so the case is the interior node of two colinear hot members | §9 |
| S11B-7 | Erratum: the zero witness is removed from E3–E5 and K5; it stays only at `FK:554`, with V1's boundary wording. Evidence corrected: `multicase-dense_scrutiny` holds 5 −0.0 stress rows, all retained-path rows of selected cases. The E-site folds cannot produce −0.0 today, so +0.0 there changes no committed zero sign | §4.1.3, §9 |
| S11B-8 | The product's m_i is the audit's per-row operation count; the completeness limit is per row and recorded. The "acceptance does not depend on d_i" claim is scoped to paths where every load-like term goes through the accumulator | §5.1, §5.2 |
| S11B-9 | Counts corrected (245 values in 31 of the 37 files carrying intended-action rows). The formation item is answered in `R2_ERRATUM_F2.md` (K2a checks every intermediate) | §7 |

## 0.2 What changed in revision 2 (from revision 1)

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
   - **Recovery side (new).** Every post-solve sum that folds element loads is one exact sum of its individual terms, rounded once, through the same accumulator. This covers member end forces, station resultants, stress-extrema intensities, curved-bend intensities and sections, restrained reactions, load-case combinations, pressure stresses and expansion-joint thrust (E1–E16).
   - **Kernel side (revision 3).** Where the kernel combines the force with prescribed-motion terms `K_ic·g_c`, it does so through the same accumulator (§2.4).
   - With only the force side, V1's probe A publishes root shear 1.24e-9 off at G = 1e7. With both halves, every probe-A quantity is within 4.6e-16 of its own magnitude up to G = 1e80 (§5.3).
2. **Bypass is impossible by construction at every solve seam.** The case force vector is an `FK` type that only the ledger can build, and every solve entry point takes it. A copied or modified `Vec<f64>` cannot reach a solve. A source-scan test over `PP`, `FK`, `SA` and `nonlinear_integration` enumerates every place that combines a force with other terms. The limits are stated in §4.3. The M03 audit remains as a guard, with a stated floor (§5.1).
3. **One correctly rounded function.** It is `FK`'s new `exact_sum` accumulator, moved from `P/core/product_physics/src/pressure_sum.rs`: a fixed-point sum, rounded to nearest-even. An exact zero is +0.0. It replaces `Expansion::rounded()`, which is not correctly rounded, at all three `FK` sites.
4. **New finding (§7).** `Expansion::rounded()` feeds a published, byte-compared value: the M03 intended-action `ResidualRow` in the `Debug` structural report embedded in diagnostics. It publishes **−0.0** for an exact-zero residual, because the standard library's float `Sum` starts from −0.0. There are 245 such values in committed fixtures. Moving that site to +0.0 would change them, so ROOT adopted the zero witness there, and only there (D-S11-1, §4.1.3).
5. **Landing.**
   - **S11-K** is the first T3 slice. It is T1-disjoint and a full product slice with full gates (ROOT, D-S11-3). ROOT has raised its priority: it lands soon after T1 merges (after S11B-1). It is live in `SP` (exact recovery), in `load_case_algebra` (exact combination), and at the `FK` rounding sites. The exact prescribed-motion right-hand side is bit-identical on main, where every prescribed value is 0.0 (`PP:3371-3374`; imposed displacements are refused at `PP:1236`), and live on T1's 0.4.0 route once T1 merges, where T1's support-motion fixtures are pre-registered as expected diffs (§8.3). Everything else in it is dormant.
   - **S11-F** is the first facade slice after T1 merges. It covers the ledger, the switch to the typed seams, the `PP` recovery composition (including E15 and E16) and T1's three sites.
   - The ROOT text below governs: no interim containment before T1 merges. S11-K's live parts are repairs in T1-disjoint files, as ROOT's disposition of S11-V1 allows for `SP`. `load_case_algebra` and the `FK` sites were D1's additions, accepted by ROOT (D-S11-3).
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
| **E15** (rev. 3) | `PP:10780-10802` `pressure_for_pipe` (T1 `:11400-11418`) | `pressure += load.magnitude.value` over every pressure load on the pipe; feeds the published hoop and longitudinal pressure stresses at ends and stations (`PP:2558-2569`, then `recover_section_stress`) | S11-F |
| **E16** (rev. 3) | `PP:8368-8382` `append_expansion_joint_pressure_thrust_results` (T1 `:8974-8992`) | `entry.axial_load += load.axial_load` over the pressure loads on a joint; published as `expansion_joint_pressure_thrust_load_review` (`PP:8393-8396`) | S11-F |

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

### 2.4 Kernel side: the prescribed-motion right-hand side (S11B-1)

| # | Site | What is folded today | Slice |
|---|---|---|---|
| KS1 | `FK/structural.rs:603-606` `prepare_structural` | `b = force[i]; b -= fl(K_ij·u_j)` over every prescribed (j, u_j): the M03 solve's own right-hand side | S11-K |
| KS2 | `FK/lib.rs:870-877` `reduce_system_for_boundary` (behind `reduce_system` and `reduce_system_with_prescribed_displacements`) | `adjusted_force -= K·g` in the same way | S11-K |
| KS3 | `FK/structural.rs:697-760` `evaluate_original_residual`, on rows coupled to a nonzero prescribed value | `r = −f_i + Σ_j fl(K_ij·u_j)` over the full row, prescribed columns included. The refinement correction is solved from this residual (`:966-973`). If KS1 is exact but KS3 still folds, a refinement step puts the fold error back | S11-K |

- **Live where (corrected in revision 4, R3-1).** KS1–KS3 are bit-identical wherever every prescribed value is zero. On main today every prescribed value is 0.0 (`PP:3371-3374`; imposed displacements are refused at `PP:1236`). They are live on T1's 0.4.0 support-motion route (`StructuralSystem.prescribed` from `resolved.prescribed`). **S11-K lands after T1 merges** (ROOT), when T1's committed support-motion fixtures carry nonzero prescribed values (for example `load_reference/connected.request.json`: UX 0.5 mm and RZ 0.001 rad). Their original-residual rows (KS3) will almost surely change in the low bits, and their displacements may (KS1). They are pre-registered as expected diffs (§8.3).
- **Evidence.** V1's probe P (`T3/REVIEW/_run_records/s11_backcheck/`): two 3 m members, both ends settled by the same g, and a 0.0137 N·m moment at the middle node. Member moments are wrong by 1.21e-9 (g = 0.05 m) and 2.57e-9 (g = 0.20 m) on the body scale, with the guard at 3.4e-4 and 8.5e-5 of its target.
- **ROOT (after `61b228543`).** S11B-1 does not block T1's merge. The no-interim ruling is extended to T1's 0.4.0 support-motion route, with the added reopen trigger "P1 or T1's final review finds a Passed breach on a realistic settlement case". The exact right-hand side joins S11-K, which lands soon after T1 merges.
- **Not changed:** T1's legacy observation lane (`observation_force`, the same fold) feeds only the protected DEC-050/053 observations. It stays as it is (§4.3 limits).

### 2.5 Re-scan for other published sums over more than one source (S11B-2)

Searched, at `c61a540ea` and at T1 with `git show`: every `+=`, `-=`, `.sum()`, `.sum::<f64>()` and `fold(` in `PP`, `pressure_runtime.rs`, `self_weight.rs`, `source_recovery.rs`, `primitive_loads`, `FK/{lib.rs, structural.rs, structural/exact_boundary.rs}`, `SA` and `sparse_direct`.

| Found | Disposition |
|---|---|
| E1–E16, KS1–KS3 | In scope (§2.2, §2.4) |
| `PP:1517-1524` `.sum()` of per-case counts | Integers |
| `PP:3821`, `:3937`, `:3949`, `:3965`, `:10890`, `:10898` `fold(…, max)` | Maxima, not sums |
| `PP:7479` per-load local transform | Formation (one source) |
| `PP:7666` stress-bound signed sums | Declared (§2.2) |
| `PP:10879-10884` `multiply_matrix_vector` (`K·u`, used by E12) | The formed elastic term of E12 |
| `pressure_runtime.rs:1048` chord projection | Geometry, not loads |
| `self_weight.rs:478-484` | Declared same-sign formation (§2.2) |
| `source_recovery.rs:580` and `:761` | The fold check (§4.5) and the stiffness coverage fold (stiffness, not loads) |
| `primitive_loads/src/lib.rs:1405` `global_load_vector` | Replaced by the ledger in `PP`; forbidden there by the site test (§4.3) |
| `open_formula_summary_mpa` (`PP:8971-8995`), the membrane `fused_mul_add` (`pressure_exact/source_geometry.rs:135-165`), `stress_recovery::summarize_components`, the lumped lever rule (`primitive_loads/src/lib.rs:2188-2219`) | V1 checked these: two-term or one-source, or not published by `PP` |
| `FK/structural.rs:474-490` `contribution_differences` delta norm | A diagnostic norm of stiffness differences |
| `SA:870-871` scrutiny force plus applied friction | T5's open item (E14) |
| `sparse_direct/src/lib.rs:678`, `:728` | Permutations, not sums |
| `preview_physics.rs:777-778`; `pressure_exact*`; `membrane_publication_range.rs`; `source_receipt/{composite, endpoint_maximum}.rs`; `stress_recovery` | V1's scan (S11_BACKCHECK_R3, R3-N3): a combination-factor sum used only for a gate with a Σ|c|-scaled tolerance, a max fold, and no other sums outside tests |

No further published sum over more than one source was found. **This table is the site test's constant** (§4.3 rule 8, revision 4): each function named here, with its disposition, is listed in the test.

## 3. Options (revision 1's verdicts stand)

- **C1** (exact sum inside `global_load_vector` alone): **rejected.** It knocks out retained recovery, and through the composite that yields an invocation-level `Err` (`PP:1249-1253`) or a partial source-blocks-1 receipt.
- **C2** (a blocking load finding): **rejected.** It blocks the whole envelope (`PP:1430-1432`, `:1497-1499`).
- **C3-full: chosen, now defined as ledger plus exact recovery (§4).** Force and recovery sums are repaired. No case moves from Passed to Sensitive. No path to the composite `Err` is added, provided every site moves together (§4.5).
- **C3-detect: fallback only,** with or after D2's S-D. It keeps every value and demotes flagged cases. It is also the comparator of the invariant (§5.3).

## 4. The rule

### 4.1 One correctly rounded function (S11-V4; ROOT 2, 4, 5)

#### 4.1.1 Definition

`FK` gains two public modules: `exact_sum` (new file `FK/exact_sum.rs`) and, in revision 3, `load_ledger` (new file `FK/load_ledger.rs`, §4.3). It is dependency-free, and every crate involved already depends on `FK`: `primitive_loads`, `SP`, `CB`, `nonlinear_integration` and `product_physics`. `load_case_algebra` gains a direct `FK` path dependency.

- **`ExactAccumulator`** is a signed fixed-point integer accumulator.
  - It is generalized from the 34-limb accumulator of `pressure_sum::exact_sum` (`P/core/product_physics/src/pressure_sum.rs:1-125`, T1-disjoint).
  - **Quantum (S11B-4).** Its unit is 2^-2148, the quantum of an exact product of two binary64 values, not `pressure_sum`'s 2^-1074. The span runs from 2^-2148 to 2^2048, plus 64 carry bits: 4260 bits, held in 68 limbs of 64 bits (4352 bits). That is enough for 2^64 maximal products. A 106-bit significand product fits `u128`.
  - `add(x: f64)` adds x exactly, as the integer x·2^2148.
  - `add_product(a: f64, b: f64)` adds a·b exactly, through the 106-bit integer product of the significands placed at exponent e_a + e_b. No FMA underflow case exists.
  - **`round(&self) -> Result<f64, SumError>`** rounds the exact integer to binary64, to nearest with ties to even, **at the binary64 quantum of the result's binade, including the subnormal binade (quantum 2^-1074).** It does not reuse `pressure_sum`'s shortcut `highest < 52 → f64::from_bits(sign | a[0])` (`pressure_sum.rs:85-87`), which is exact only when the accumulator's quantum is 2^-1074. Examples from V1's probe X: 3·2^-1076 → 2^-1074; a tie at 2^-1075 → 0; (1.7e308)² − (1.7e308)² → exactly 0.
  - **`round_scaled(&self, e: i32)`** (revision 4, R3-N2) multiplies the exact integer by 2^e (an exact shift of the binary point) and then rounds once, with the same rules. `round()` is `round_scaled(0)`. Callers that scale a rounded sum by a power of two (KS1 and KS3's radix normalization) use it, so a value that would be subnormal before or after scaling is not rounded twice.
  - **A nonzero net that rounds to zero (S11B-4).** It returns **+0.0** at every site, whatever its sign. That is a stated deviation from IEEE 754's −0.0 for a negative underflow, chosen because the ledger, the bit-equality checks and every published sum then share one zero (ROOT, D-S11-1). The ledger records each such DOF in its evidence (`LedgerEvidence::underflowed_dofs`), because a load below 2^-1075 in SI units has been lost. Range handling stays W2's (`DESIGN.md` §4.7); this is not a refusal.
- **`exact_rounded_sum(values)`** and **`exact_rounded_dot(pairs)`** are thin wrappers. There is one rounding path.
- **Errors.** `SumError::{NonFinite, AccumulatorOverflow, NonRepresentable}` are unchanged from `pressure_sum`. A net outside the binary64 range is `NonRepresentable`, never ±∞. Each call site maps it to the diagnostic its current non-finite path already uses: `require_finite_mechanics` and `validate_finite_array`.
- **Exact zero.** An exact zero returns +0.0, for any operand signs and for the empty sum. `float(Fraction(0))` behaves the same way in the probes. Together with the underflow rule, **`round()` never returns −0.0.**
- **`pressure_sum::exact_sum`** becomes a wrapper over `exact_rounded_sum`. Its existing tests stay, byte for byte.

#### 4.1.2 Where it replaces something

| Site | Today | With `exact_sum` |
|---|---|---|
| The ledger, the recovery sums E1 to E13, E15 and E16, the kernel sites KS1 to KS3 (§2.4), and `source_recovery`/`source_receipt` (§4.5) | binary64 folds | `ExactAccumulator` |
| `FK/structural.rs:369-377` `exact_scalar` | `rounded()`, then an exact check of the difference | `round()`, then the same check. It is sound either way. It now withholds less often (§7) |
| `FK/structural.rs:406-414` absorbed-diagonal screen | `rounded()` before and after the contribution | `round()` before and after |
| `FK/structural.rs:554` intended-action residual `r` | `rounded()` | `round()`, with the zero rule of §4.1.3 |
| `FK/structural/exact_boundary.rs:361`, `:387` coverage checks | naive projection or ordered fold, compared with `!=` | also accept `round()` of the same terms (S11-K) |
| `exact_boundary.rs:218-229` `approximate_projection` (used at `:726`, `:1025`) | naive quotient | unchanged. It is documented as a proposal that is verified afterwards, so it makes no rounding claim |

`Expansion::rounded()` is removed, so no site can keep using it.

#### 4.1.3 Signed zero: +0.0 everywhere, one diagnostic witness (D-S11-1 as ruled; erratum S11B-7)

- **The rule.** `round()` returns +0.0 for every zero result (§4.1.1). The ledger, the force vector, every recovery sum E1–E16, the kernel sites KS1–KS3, every published result row, every `source_recovery` and `exact_boundary` comparison and every value at precision p use it.
- **The one exception: the zero witness at `FK/structural.rs:554`** (ROOT's D-S11-1 and its condition). The intended-action `ResidualRow` fields `residual` and `normalized_residual` keep **−0.0** for an exact-zero residual, written as a literal with a comment naming the committed bytes, not left to `Iterator::sum`.
  - Why: an exact-zero residual is the empty expansion, and today's `rounded()` is `iter().sum()`, whose neutral element on the pinned toolchain is −0.0. Committed fixtures hold 245 `normalized_residual: -0.0` values, in 31 of the 37 files that carry intended-action rows (for example `n05-dense_scrutiny.raw.json`, `ResidualRow { global_dof: 6, residual: -0.0, … normalized_residual: -0.0 …}`).
  - **Boundary (V1's wording, adopted).** The witness is never used in a value that is independently recomputed and compared (the fold and coverage checks, replay, `source_receipt/rows.rs` `compare`), and never in a receipt field. Digests over rendered published text are consistent by construction, because the text is hashed as produced and re-hashed from the same bytes. The two fields are consumed only by the `Debug` rendering in diagnostics (`PP:883`, `:2046`, `:2057`) and by the gate through `r.abs()`; `ResidualRow` has no `Serialize`, and `intended_residual_rows` is read elsewhere only for size accounting (`SA:765`).
- **Erratum to revision 2 (S11B-7).** Revision 2 also proposed the witness at E3, E4 and E5. That is withdrawn: those are published result values, bit-compared for selected cases against rows rebuilt from retained recovery (`source_receipt/rows.rs:306-307`, used at `:604` and `:625`), and consumed by E6. They use +0.0.
- **Why +0.0 at the E-sites changes no committed zero sign.** None of today's E-site folds can yield −0.0. A fold yields −0.0 only if its accumulator starts at −0.0 or every term is −0.0 and it starts from its first operand. Here:
  - E1, E2, E7, E9, E10, E13, E15 and E16 start from literal +0.0 accumulators (`[0.0; 12]`, `[0.0; 3]`, `let mut … = 0.0`);
  - the elastic term `K_e·u` starts from +0.0 (`SP:1354-1362`), so `local_forces` is never −0.0, and neither are E3 and E5 (`local − equivalent`, then `+= axial_load` only when nonzero) or the end actions that E4 and E6 start from; E4's `M_i + V·d` has d ≥ 0;
  - E8 and E11 are built from +0.0-started global forces.
  - The exception is E12: its formed `K·u` uses `multiply_matrix_vector` (`PP:10879-10884`, `Iterator::sum`, neutral −0.0). An unloaded, unmoved restrained DOF whose products are all −0.0 publishes −0.0 today and +0.0 after. The four committed −0.0 support components are spring actions `−k·u` (`independent-spring:Mx` in the `physics_source/mixed*` raws), not E12.
- **Evidence corrected (S11B-7).** `source_blocks/multicase-dense_scrutiny.raw.json` holds **5** −0.0 stress rows, not 7. All five are `end_i` stress components of the two selected cases (`retained_source_blocks_exact_v1`), produced by the retained path and verified by the receipt. They are not E-site outputs and not j-side negations. Across `P/fixtures`, 941 result values with a `kind` are −0.0, in 29 files: 937 member-action and stress rows and the 4 spring components. Since the E-site folds cannot yield −0.0 (above), these zeros are produced outside the E-site sums: by sign conventions and stress factors applied to the rounded actions (for example the j-side negation at `PP:7509-7519`), or by the retained path.
- **Any sign change is reported.** The committed-fixture diff and its stop rule (§8.3) report every sign-of-zero change to ROOT before any regeneration.

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

### 4.3 The force vector can be built only from the ledger (S11-V2; ROOT 3; revised for S11B-3)

- **Types, now in `FK` (`FK/load_ledger.rs`, S11-K).** `primitive_loads` already depends on `FK`, and re-exports them.
  - `LoadLedger` records `(source: String, dof, kind: Term | Product)` entries. It offers `push`, `push_product`, `terms()`, and `finish(n) -> Result<AssembledForce, LedgerError>`.
  - `AssembledForce` wraps a private `Vec<f64>` and its terms. It has no public constructor, no `From`/`Default`/`Deserialize`/`Clone` into a plain vector, and no `&mut` access. It offers `values(&self) -> &[f64]`, `get(dof)` and `terms(&self) -> &[ForceTerm]`.
  - `ReducedForce` is the reduced right-hand side. `FK`'s reduction functions alone build it, exactly (§2.4 KS2).
- **Every solve seam takes the typed force (S11-K defines them; S11-F wires the product).**

| Seam | Today | Revision 3 |
|---|---|---|
| `FK/structural.rs:16-30` `StructuralSystem.force` (read by `validate`, the audit, `prepare_structural`, `evaluate_original_residual`, `solve_structural_dense`, `factor_structural_*`, `finish_structural`, and the exact context) | `&[f64]` | `&AssembledForce`. The exact context takes its force terms from `force.terms()`, so coverage and solve see one source |
| `FK/lib.rs:820-880` `reduce_system`, `reduce_system_with_prescribed_displacements`, `reduce_system_for_boundary` | `&[f64]` in, `Vec<f64>` out | `&AssembledForce` in, `ReducedForce` out |
| `SA:252-289` `StructuralAssembly::solve` | `f: &[f64]` | `f: &AssembledForce` |
| `sparse_direct::structural::solve_structural_sparse` | takes `&StructuralSystem` | unchanged signature; inherits the typed force |
| Nonlinear public entry points `solve_active_set_frame`, `_with_mode`, `_with_mode_and_springs` (`nonlinear_integration/src/lib.rs:484`, `:490`, `:499`), which take the force inside `NonlinearFrameSolveInput.force` (erratum I1: revision 4 named `:2015`, `:2045`, `:2242`, which are private functions of the sparse-parity observation lane, allow-listed under limit 2) | `input.force: Vec<f64>` | dormant typed siblings taking `&AssembledForce`: each returns `InvalidInput` unless `input.force` equals `force.values()` bit for bit, then delegates |
| `source_recovery::Input.force` | `&[f64]` | `&AssembledForce` |
| Both receipt replays | `global_load_vector` | the ledger, through the live producer functions |

  A copied or modified `Vec<f64>` therefore cannot reach any solve: `values().to_vec()` compiles, but nothing that solves accepts the result.
- **Stated limits (where a type cannot reach, and why).**
  1. **Generic linear-algebra kernels** keep `&[f64]`: `FK::solve_dense`, `PreparedSystem::solve(rhs)`, `sparse_direct::{solve_symmetric_system, solve_symmetric_system_from_entries, factorize_ldlt}`. They solve refinement corrections, condition-estimate probes and test systems, not case forces, so a force type there would be wrong. The site test forbids any product call to them except in the allow-listed observation functions below.
  2. **Two protected observation lanes** read `values()` and fold in binary64 on purpose: main's sparse parity observation (`assemble_reduced_sparse_entry_system` → `solve_symmetric_system_from_entries`, published as `sparse_live_path_dense_parity_relative_delta`, DEC-050/053) and T1's `observation_force`. They report solver parity, not mechanics results. They stay as they are, allow-listed by function name.
  3. **In-loop friction** (`SA:870-871`, `nonlinear_integration/src/lib.rs:1642-1659`) adds applied friction forces inside the nonlinear loop, and the friction influence solves build unit-force vectors (`nonlinear_integration/src/lib.rs:1332-1337`, `unit_force[…] += 1.0`, solved through the generic kernels). Both are T5's (E14, SUP-16), allow-listed as such (revision 4, R3-N1).
  4. **Reading is always possible.** Rust cannot stop code reading `values()` into a new vector. The types stop such a vector reaching a solve. Rule 8 below makes every floating-point accumulation in the load-bearing files a named, reviewed entry, so a new published sum cannot appear silently; whether a newly listed entry is really a load sum remains a review judgment, which the list makes visible (corrected in revision 4, R3-3).
- **Enumerated site test (S11-F, strengthened).** A `PP` unit test reads, with `include_str!` and relative paths, `PP`, `source_recovery.rs`, `source_receipt.rs` and `source_receipt/*.rs`, `pressure_runtime.rs`, `self_weight.rs`, `FK/{lib.rs, structural.rs, structural/exact_boundary.rs, load_ledger.rs}`, `SA`, `nonlinear_integration/src/lib.rs` and `sparse_direct/src/structural.rs`. Outside `#[cfg(test)]` blocks it asserts:
  1. no `global_load_vector(` call;
  2. no `values()` followed by `.to_vec(`, `.to_owned(`, `.iter_mut(` or `.iter().copied()/.cloned()…collect`, except in allow-listed functions;
  3. no compound assignment (`+=`, `-=`) and no `iter_mut()` on any binding whose name contains `force`, `rhs` or `load` (case-insensitive), or that is bound from an expression mentioning `values()`, `force` or `rhs`. This catches `PP:1839-1840` (`force.iter_mut()`) and T1's `observation_force`, which are either rewritten or allow-listed;
  4. no product call to the generic kernels of limit 1 outside the allow-list;
  5. the functions that call `.push(`/`.push_product(` on a `LoadLedger` are exactly the producer list of §4.2;
  6. the functions in `FK`, `SA` and `nonlinear_integration` that combine a force with other terms are exactly KS1–KS3 (§2.4), `audit_intended_action` (already exact), the exact-context coverage check, and the allow-listed E14;
  7. the allow-list itself equals a constant in the test, so extending it is a visible edit;
  8. **(revision 4, R3-3) completeness by constant.** The test also reads `SP`, `CB` and `load_case_algebra/src/lib.rs`. In `PP`, `pressure_runtime.rs`, `self_weight.rs`, `SP`, `CB` and `load_case_algebra`, every function outside `#[cfg(test)]` that contains a compound assignment (`+=`, `-=`), `.sum`, `.sum::<f64>` or `fold(` must appear in the test's constant with one disposition: E-site (E1–E16), declared formation, integer (index and count arithmetic, which a source scan cannot tell apart from floats, so it is listed too), max, or allow-listed observation. The constant is §2.5's table plus §2.2's declared formations, by function name. A function that matches the pattern and is not in the constant fails the test, whatever its variable names. **Erratum R3B-5 (ROOT):** the constant is keyed by **function plus its count of matching lines**, so each accumulation is its own entry: a new sum inside an already-listed function (for example `solve_load_case`, 3 matches, or `run_linear_static_preview_captured`, 4) changes the count and fails the test until the constant is edited. V1's probe G counts 59 matching lines in 31 `PP` functions outside tests at `c61a540ea`; the constant records each function's count at the slice's base. This catches an E15-class sum (`let mut pressure = 0.0; pressure += …`), which rule 3's name-based check would not.
  
  **Limit of the test itself.** It is a source scan, not a type check. It backs the types where the types cannot reach (limits 1–4), and a reviewer sees any change to its lists.
- **`global_load_vector`** stays for non-product callers, documented as "binary64 fold; not for solve input". The test forbids it in the scanned files.
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
| E15 (`PP`, rev. 3) | each pressure load's `magnitude.value` on the pipe; the pressure stresses are then formed from the once-rounded net pressure, as today |
| E16 (`PP`, rev. 3) | each pressure load's `axial_load` on the joint |

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

### 4.6 The kernel right-hand side and refinement residual (S11B-1)

- **KS1 (`prepare_structural`) and KS2 (`reduce_system_for_boundary`).** For each free row i, the reduced right-hand side is one `ExactAccumulator` sum: the ledger's terms for DOF i (from `force.terms()`), plus `add_product(−K_ic, g_c)` for every prescribed (c, g_c), rounded once. `prepare_structural` then applies its exact radix scaling (`radix_scale`) to the rounded value, as today.
- **KS3 (`evaluate_original_residual`), on rows coupled to a nonzero prescribed value.** The numerator is one exact sum: the ledger's terms for DOF i negated, plus `add_product(K_ij, u_j)` over the full row, rounded once and then radix-normalized, as today. Its denominator and allowance are unchanged. Rows with no nonzero prescribed coupling keep today's binary64 evaluation, bit for bit, because their only load operand is the ledger's single net and the rest is the structural action that M03 already audits.
  - Why KS3 is needed: the refinement correction is solved from these residual rows (`FK/structural.rs:966-973`). If KS1 were exact but KS3 still folded f with the gross `K_ic·g_c`, a refinement step would put the fold error back.
- **Where the bits do not change:** every row whose prescribed values are all zero. There the exact sum of [ledger terms, ±0 products] equals today's value bit for bit. That includes +0.0, because `round()` never returns −0.0 and today's `b − (±0)` from +0.0 is +0.0. That is every row on main today. **Where they change:** rows coupled to a nonzero prescribed value, which after T1 merges includes T1's committed support-motion fixtures (§8.3's pre-registered list).
- **Scaling (R3-N2).** `prepare_structural` radix-normalizes its right-hand side by `exponents[r]`, and `evaluate_original_residual` normalizes by the row exponent. KS1 and KS3 use `round_scaled` with that exponent instead of rounding and then calling `radix_scale`. For normal values the result is bit-identical to rounding then scaling; it differs only where rounding-then-scaling would round twice in the subnormal range.
- **The exact context** (`exact_boundary.rs`) already treats `K_fc·g` exactly and checks only force coverage (V1), so it needs no change beyond §4.1.2.

## 5. Guard, scale and invariant

### 5.1 The guard and its floor (S11-V2)

- **The predicate is M03's intended-action predicate,** unchanged in form, with the load term replaced by the ledger's exact per-DOF sum:
  - `ratio_i = |Σ_j K_ij·u_j − f_i^exact| / d_i`, with `d_i = |f_i^exact| + Σ_j |K_ij|·|u_j|`;
  - flagged when `ratio_i > 64·γ(m_i)`;
  - applied to free and restrained rows.
- **Floor, stated.** The guard flags a net-load error e_i only when `e_i > 64·γ(m_i)·d_i`. Below that it is silent. A published quantity governed by that DOF's net then carries a relative error of at most `64·γ(m_i)·(d_i/|f_i|)`. It is guaranteed within 1e-9 only when the row amplification is `d_i/|f_i| ≤ 1e-9/(64·γ(m_i))`.
  - **The product's m_i (S11B-8)** is the audit's own per-row operation count, `residual.operations + denominator_operations + 2` (`FK/structural.rs:552`). It depends on the row's expansion work, so the completeness limit is per row. The `LoadFidelityReport` records m_i, the target and the completeness limit for every row it lists.
  - **Example from the product.** A committed report row has m_i = 92, target 6.54e-13 (`n05-dense_scrutiny.raw.json`), so the guard is complete up to an amplification of about 1.5e3.
  - **The probes use a smaller count,** m = 2k + 2, which gives target 4.26e-14 and a limit of about 2.3e4 (D1's probe A) or 1.2e4 (V1's probe P). These are emulations; the product's per-row m_i governs.
  - Probe A's tip rows have amplifications of 6 and 18.
  - V1's probe B (a 1e6 N·m moment at the same node, amplification about 2e7) is beyond the floor, which is why it passes.
- **Why the floor is acceptable.** Under C3-full the guard never has to carry 1e-9. Every ledger-routed and recovery-routed load, and every kernel combination of the force with prescribed-motion terms (§4.6), is exact by construction (§4.3, §4.4). The guard's job is to catch mutations and future bypass defects, and it catches gross ones (a missing producer) regardless of the floor.

### 5.2 Relation to R1's net-governed scale for RF-CANCEL (R1 finding 4)

- **R1's recommended RF-CANCEL scale** (`T3/REFERENCES/README.md` §4 and the RF-CANCEL notes) is net-governed: each value is compared relative to its response to the net alone.
- **The row scale d_i** is a runtime detection scale, not an acceptance scale.
- **Under C3-full the acceptance does not depend on d_i — on paths where every load-like term goes through the accumulator (S11B-8).** Load-like terms are the ledger's terms and the prescribed-motion products `K_ic·g_c`. On such paths the ledger gives the correctly rounded net, and recovery sums the load terms exactly. V1's probe P was a path where this did not hold (amplification 2.6e7 to 1.1e8); §4.6 closes it. So every net-governed RF-CANCEL value meets `|obs − exp| ≤ 1e-9·max(|exp|, scale)`, up to the solve's ordinary accuracy. Probe A's errors are at most 4.6e-16 of the value's own magnitude.
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
   - 37 committed fixtures contain intended-action rows, and fixture tests compare them byte for byte. The 245 −0.0 values sit in 31 of those 37 files (V1's count).
   - **The gate decision is protected:** the ratio adds `g/(1 − g)` with g = γ(operations), and the operation count includes the expansion's operations. The published value can differ from the correctly rounded residual in the last bits.
   - **The zero is −0.0.** An exact-zero residual is the empty expansion, and the standard library's float `Sum` starts from −0.0. That gives the 245 `-0.0` values of §4.1.3, next to +0.0 in the ordinary residual rows. The published zero sign is therefore a standard-library behaviour rather than a design choice, and pinning the toolchain is what keeps it stable.
- **Finding for the map (N-S11-R).** A published, byte-compared value depends on a non-correctly-rounded sum and on the standard library's float-sum neutral element.
- **Disposition in S11-K (D-S11-2, ruled).** Replace it with `round()` plus the zero witness at `:554` only (D-S11-1, §4.1.3). ROOT records N-S11-R as low severity. The committed-fixture diff shows whether any nonzero residual changes. For short, nonoverlapping expansions it is expected not to change, but that is unproven until the run.

## 8. Slices, serialization and disclosure

### 8.1 S11-K (the first T3 slice; T1-disjoint; ROOT: land soon after T1 merges)

- **Write set:**
  - `FK/exact_sum.rs` and `FK/load_ledger.rs` (new), and `FK/lib.rs` (their module declarations);
  - `FK/lib.rs:820-880`: `reduce_system*` exact right-hand side (KS2), and new typed entry points taking `&AssembledForce` and returning `ReducedForce`, beside today's `&[f64]` ones;
  - `FK/structural.rs`: `rounded()` replaced at `:369-377`, `:406-414` and `:554`; the audit with exact per-DOF force terms; the exact right-hand side in `prepare_structural` `:603-606` (KS1); the exact numerator in `evaluate_original_residual` `:697-760` on prescribed-coupled rows (KS3); and a typed `StructuralSystem` constructor, beside today's layout;
  - `FK/structural/exact_boundary.rs:361`, `:387`;
  - `SA`: `AssemblyEvidence::with_force_terms`, and a typed `StructuralAssembly::solve` beside today's;
  - `nonlinear_integration/src/lib.rs`: dormant typed siblings of the public entry points `solve_active_set_frame`, `_with_mode` and `_with_mode_and_springs` (`:484`, `:490`, `:499`); each requires `input.force` to equal `force.values()` bit for bit, returns `InvalidInput` otherwise, and delegates (erratum I1; ROOT serializes this crate with T5);
  - `P/core/loads/primitive_loads/src/lib.rs`: re-exports of the ledger types;
  - `SP`: E1 to E4, E6, and `equivalent_nodal_load_terms_with_spans`;
  - `CB`: `arc_section_resultant_terms`;
  - `P/core/loads/load_case_algebra/src/lib.rs` and its `Cargo.toml` (E13, `FK` dependency);
  - `P/core/product_physics/src/pressure_sum.rs` (wrapper).
- **Why the typed seams are added beside the old ones.** S11-K stays T1-disjoint, so `PP` must compile unchanged. S11-F switches `PP` to the typed seams and removes the `&[f64]` product entry points (they stay for tests, as `#[cfg(test)]` or `pub(crate)`).
- **Live effect:**
  - `SP`: exact recovery sums in every product path that recovers straight members;
  - combinations;
  - the `FK` rounding sites;
  - KS1–KS3 only where a prescribed value is nonzero: nowhere on main today, and T1's support-motion fixtures once T1 has merged (pre-registered, §8.3).
  - Everything else is dormant until S11-F.
- **Gates (D-S11-3, ruled).** A full product slice as its own PR to main: independent review, hosted CI including the surface-4 dual-viewport dispatch, a clean DEC-025 sweep, and the fixture stop rule (§8.3).
- **Serialization.** S11-K → K2a → K1 → K2b → K5 (`DESIGN.md` §6). S11-K, K1, K2b and K5 write `SA` and `FK/structural.rs`; S11-K and K2a both write `FK/lib.rs` (S11-K its module declarations and `reduce_system*`, K2a `local_stiffness`), so K2a merges after S11-K. K3 runs in parallel.

### 8.2 S11-F (the first facade slice after T1 merges, ahead of F1)

- **Write set:**
  - `PP`: the ledger at every §4.2 producer; the switch to the typed seams; E5, E7 to E12, E15 and E16; the Sensitive mapping; the site test (§4.3), which reads `PP`, `FK`, `SA`, `nonlinear_integration` and `sparse_direct` sources;
  - `P/core/product_physics/src/pressure_runtime.rs` (push group operands);
  - `source_recovery.rs` and `source_receipt.rs` at T1's three sites (§4.5), and T1's 0.4.0 prescribed-motion wiring onto the typed seams (KS1–KS3 through `StructuralSystem`);
  - removal of the `&[f64]` product entry points added beside the typed ones in S11-K.
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
  - **The expected result is "unchanged", except the pre-registered list.** Committed force DOFs carry at most two contributions (§2.1 R10). The E-site folds cannot produce −0.0 today, so +0.0 there changes no committed zero sign (§4.1.3), and the witness keeps the 245 residual −0.0 values at `FK:554`. KS1–KS3 are bit-identical wherever every prescribed value is zero (§4.6).
  - **Pre-registered expected diffs (revision 4, R3-1).** S11-K lands after T1 merges, so these T1 files are expected to change in low bits, because their requests carry nonzero prescribed motions (KS1: displacements may change; KS3: the original-residual rows in the `Debug` text will almost surely change): `fixtures/product_preview/load_reference/connected-{dense_scrutiny,sparse_interactive}.raw.json`; `fixtures/product_preview/load_reference_source/eigen_motion-{dense_scrutiny,sparse_interactive}.raw.json`; and the derived `fixtures/results/load_reference_connected_{dense,sparse}.{analysis_run,document,stress_neutral}.json` and `fixtures/results/load_reference_source_eigen_motion_{dense,sparse}.{analysis_run,document,stress_neutral}.json`. The diff tool also flags any other committed request with a nonzero prescribed value, and every file derived from it, as belonging to this list. The stop still applies: the slice reports these diffs to the T3 manager with the reason, and ROOT decides before the actual producer regenerates them. **Erratum N-4:** the size of each change (bytes changed, and the largest relative change per quantity kind) is recorded in S11-K's PR record as well as in the report to ROOT. Any diff outside the list is an unexpected stop.
  - **Stop rule.** `pipe:P-120` in `load:L-100` has four-term end-force sums (E5). Committed Debug residuals may also differ from correct rounding (§7). If the diff shows any committed byte change, the slice stops and reports each change to the T3 manager with its site and reason, **before** any fixture is regenerated. Derived fixtures are regenerated only by the actual producer, after ROOT's decision. Frozen references and historical raws never change.

## 9. Tests and mutations

Tests are cited as K*n* (the S11-K list) and F*n* (the S11-F list). The kernel sites of §2.4 are KS1–KS3, and `DESIGN.md`'s kernel slices are K1–K6, K2a and K2b.

**Precondition rule (S11B-6).** Every test that is meant to kill a fold mutation first asserts, inside the test, that the binary64 fold of its terms (in the producer's own order) differs from the correctly rounded net. Otherwise the test could pass vacuously. This applies to K4, K11, F4, F5, F6, F8, F9 and F10.

**S11-K.**
1. `exact_rounded_sum` and `exact_rounded_dot` equal the correctly rounded `Fraction` value for:
   - seeded random terms and products;
   - V1's probe D, {2^-110, 2^-53, 1} → 1.0000000000000002 (the naive sum gives 1.0);
   - the ≥3-term cancellations of §5.3, and the thermal pair (1.3, 4.1e7, −4.1e7) → 1.3;
   - ties, overflow-adjacent values and subnormal nets (`DESIGN.md` §4.11);
   - **V1's probe X cases (S11B-4):** 3·2^-1076 → 2^-1074; a tie at 2^-1075 → +0.0; (1.7e308)² − (1.7e308)² → +0.0; −2^-1080 → **+0.0** (the stated underflow rule, not IEEE's −0.0); 1.3 + 1.35·4.1e7 − 1.35·4.1e7 as exact products → 1.3.
2. Zero: [1e8, −1e8], [−0.0], [−0.0, −0.0] and [] all give +0.0 (bits). `round()` never returns −0.0.
3. `pressure_sum`'s existing tests pass unchanged through the wrapper.
4. `SP`: probe A as a unit test at **G ∈ {1e8, 1e80}** (the required kill set; S11B-5) in the orders (G, n, −G) and (n, G, −G), with the precondition. End forces, midspan and quarter stations, and E3, each within 1e-9 of the exact value (`Fraction` reference in the test). G = 1e7 is kept as an informative case, not a kill requirement (its root-shear margin is only 1.24). For one load, E1 is bit-identical to today.
   - **Axial-effect case (revision 4, R3-2).** One member with three axial-effect loads (thermal and thrust `StraightPipeAxialEffect`s of G, n and −G, G ∈ {1e8, 1e80}, n = 0.3 N), in the orders (G, n, −G) and (n, G, −G), with the precondition. E2 and the axial half of E3 are each within 1e-9 of the exact value.
5. **`SP` zero sign (rewritten, S11B-7):** a load-free station, a load-free end force and a station whose load terms cancel exactly all publish +0.0; a station of a member with a nonzero end action is bit-identical to today. No witness applies at E3–E5.
6. `load_case_algebra`: A + B − A2 with (1e80, 1e-8) → 1e-8 (the fold gives 0). 1.3 + 1.35·4.1e7 − 1.35·4.1e7 → 1.3 (the fold gives 1.2999999970197678).
7. `FK`:
   - the audit with force terms flags every probe-A G ≥ 1e5 row and V1's check L, and passes every control;
   - a restrained root row with (1e8, 0.3, −1e8) flags;
   - the `Debug` report is byte-identical when nothing is flagged;
   - `exact_scalar`, the absorbed-diagonal screen and `:554` are byte-identical on every existing `FK` test, including −0.0 at `:554` for an exact-zero residual (the witness).
8. `exact_boundary` accepts the correctly rounded sum and refuses a corrupted force.
9. `CB`: `arc_section_resultant_terms` sums exactly to the section value of the combined inputs when that value is exact. Controls use single loads (bit-identical).
10. The committed-fixture diff (§8.3).
11. **Kernel prescribed motion (S11B-1), in `FK`:** V1's probe P model (two 3 m members, both ends settled by g ∈ {0.05, 0.20} m, a 0.0137 N·m moment at the middle node), through `solve_structural_dense`, the profile path and `reduce_system_with_prescribed_displacements`, with the precondition that the binary64 fold of `f − Σ K_ic·g_c` differs from the exact value. The reduced right-hand side equals the correctly rounded exact value, and the recovered member moments are within 1e-9 of the exact answer. A forced-refinement variant (one refinement step) stays within 1e-9, which exercises KS3. With all prescribed values 0.0, every existing `FK` test is bit-identical.
12. The typed seams: `StructuralSystem`, `reduce_system*`, `StructuralAssembly::solve` and the nonlinear entry points accept only `AssembledForce`/`ReducedForce`; a doc-test marked `compile_fail` (standard rustdoc, no new dependency) shows that a `Vec<f64>` is refused.

**S11-F** (both modes, through `run_linear_static_preview_value_with_mode`).
1. The RF-CANCEL references, as ROOT ruled (`ROOT_RULINGS_V2.md` §1): every case meets R1's predicate with the binding net-governed scale. The binary64-fold negative controls fail in the orders in which they differ.
2. Probe A through the product (a single-element model; **G = 1e8 and 1e80**): end forces, stations and extrema within 1e-9. **The invariant test:** the same model's C3-detect verdict, computed from the ledger's `terms()` folded in order and passed to `FK`'s audit entry point, flags. The product publishes Passed with every quantity within 1e-9.
3. The realistic thermal case: a colinear hot run with a 4.1e7 N thermal pair and a 1.3 N co-axial nodal load at an interior node. Ordinary route and 0.4.0 eigen route. The precondition is asserted.
4. Retained source: an N05-class Sensitive case with (1e8, 0.3, −1e8) at the tip is selected, and its answer equals the exact-term answer. The precondition is asserted.
5. **V1's 0.4.0 test, restated (S11B-6).** The authored order cannot be chosen: the producer folds nodal loads first and then the eigen pairs (`source_receipt.rs:218-219`; `source_recovery.rs:1270`). The unfavourable order (1.3, +N, −N) therefore arises at the **interior node of two colinear hot members** carrying a 1.3 N co-axial nodal load, with N = 4.1e7 N from the two eigen pairs. The test asserts the precondition, then checks that the retained join is selected, finalization succeeds, and the published force equals the correctly rounded net.
6. A multi-case pre-0.4 exact-route invocation: selected case A, plus case B carrying the cancelling loads and an element load (so out of retained scope). **No `Err`, and no blocked envelope.** The precondition is asserted for case B.
7. The strengthened site test of §4.3.
8. **Curved (revised in revision 4, R3-2):** a bend carrying three uniform loads (G, 0.3, −G) N/m, G ∈ {1e8, 1e80}, in that authored order, and a thermal case with three cancelling thermal strains (ε, ε_n, −ε) on the span, so that E8's pre-summed strain is exercised. The precondition is asserted for each (the order (G, −G, 0.3) is Sterbenz-exact and would kill nothing). Recovery (end forces and arc stations) equals the exact per-load answer within 1e-9.
9. **0.4.0 support motion (S11B-1):** V1's probe P model authored on T1's 0.4.0 route (settlements g ∈ {0.05, 0.20} m). Member moments within 1e-9 of the exact answer on the body scale, with the precondition asserted.
10. **E15 and E16 (S11B-2):** a pipe with pressure loads (P, 0.3 Pa, −P) at P = 1e8 and 1e80 Pa, and an expansion joint carrying the matching thrust loads, with the precondition. The published hoop and longitudinal stresses and `expansion_joint_pressure_thrust_load_review` equal the values from the exact net.

**Mutations that must fail** (required kill set G = 1e8 and 1e80; one mutant per site, S11B-5; labels corrected in revision 4).

| # | Mutation | Killed by |
|---|---|---|
| M1a–M1o | Restore the binary64 fold at one E-site each (15 sites): M1a E1, M1b E2, M1c E3, M1d E4, M1e E6 (in `SP`); M1f E5, M1g E7, M1h E8, M1i E9, M1j E10, M1k E11, M1l E12 (in `PP`); M1m E13 (`load_case_algebra`); M1n E15, M1o E16 (`PP`) | M1a, M1c, M1d, M1e: K4; M1b and E3's axial half: K4's axial-effect case; M1m: K6; M1f, M1g, M1l: F2; M1h–M1k: F8; M1n, M1o: F10 |
| M2 | Restore `mechanical` then `corrected` as two folds in `PP` (E5) | F2 |
| M3 | Restore the ledger fold | F1, F2's invariant check, and the audit (the case turns Sensitive with `LOAD_CONTRIBUTION_ABSORBED`) |
| M4 | Add a `force[i] +=` producer, a `values().to_vec()` copy handed to a solve, or a `global_load_vector` call in `PP` | fails to compile at the typed seams (K12); F7 |
| M5 | Push the curved thermal `value` pre-summed | F7 (the listed function pushes `push_product`) and F8 |
| M6 | Replace `round()` with an ascending naive sum | K1 (probe D) |
| M7 | Return −0.0 for an exact ledger zero, or IEEE's −0.0 for a negative underflow | K1, K2 |
| M8 | Leave `source_receipt.rs:320` on `global_load_vector` | F6 (`Err`) |
| M9 | Leave `source_receipt.rs:218-219` on the fold | F5 (the join is not selected) |
| M10 | Drop restrained rows from the audit | K7 |
| M11 | Restore the combination fold | K6 |
| M12 | Restore the binary64 fold at KS1 (`prepare_structural`) | K11, F9 |
| M13 | Restore the binary64 fold at KS2 (`reduce_system_for_boundary`) | K11 |
| M14 | Restore the binary64 numerator at KS3 (`evaluate_original_residual`) | K11's forced-refinement variant |
| M15 | Round `ExactAccumulator` subnormal results by copying bits (the `pressure_sum` shortcut) | K1 (probe X) |

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
5. **Protected observation lanes (revision 3).** Main's sparse parity observation and T1's `observation_force` keep their binary64 folds, allow-listed by name (§4.3 limit 2). They publish solver-parity observations, not mechanics results.
6. **The refinement residual on rows without nonzero prescribed coupling** keeps today's binary64 evaluation (§4.6). Its only load operand is the ledger's net; its accuracy is M03's, audited exactly by the intended-action check.

## 11. ROOT's decisions (recorded; D1's options are in revision 2 §11)

| ID | Ruling (`T3/ROOT_RULINGS_V1.md`, at `4862a72a9` and `45cfc92b1`) |
|---|---|
| D-S11-1 | The zero witness, only for published diagnostic renderings (`FK:554`); the ledger, the force vector and every bit-compared path use +0.0. Revision 3 applies it (§4.1.3) |
| D-S11-2 | `Expansion::rounded()` replaced at all three `FK` sites, under the fixture stop rule |
| D-S11-3 | S11-K is a full product slice with full gates, as its own PR to main |
| D-S11-4 | No in-band marker; disclosure in the change records |
| S11B-1 | Does not block T1's merge; the no-interim ruling extends to T1's 0.4.0 support-motion route, with the added settlement reopen trigger; the exact right-hand side joins S11-K, which lands soon after T1 merges |

## 12. Run records (`_run_records/`)

- **`probe_s11_rev2.py` → `probe_s11_rev2.stdout.json`.**
  - It rebuilds V1's probe A (read from `T3/REVIEW/_run_records/s11/`, not imported) on D1's binary64 element, imported unchanged from `probe_skew_precision.py`.
  - Checks: A2 and INV (the invariant), MUT, ORD, SUM, CMB (combinations), FLR (guard amplification) and ZW (zero witness). ZW is now informative only, since the witness is withdrawn at the E-sites (§4.1.3).
- **`scan_element_loads.py` → `scan_element_loads.stdout.json`.**
  - Every committed JSON under `P/` outside `execution/`, run from `P/` with argument `.`.
  - It finds 183 load cases in 117 files, at most three element loads on one element, and nine element-case pairs with two or more.
- **Revision 1 records kept:** `probe_s11_audit.*`, `scan_load_fold.*`.
- **Revision 3 evidence from V1** (not re-run here): probes P (prescribed motion) and X (accumulator reference) in `T3/REVIEW/_run_records/s11_backcheck/probe_s11_backcheck.*`.
- **Revision 3 source reading:** `FK/structural.rs:16-30`, `:470-500`, `:540-612`, `:697-760`, `:958-980`; `FK/lib.rs:815-885`; `SA:252-289`, `:860-895`; `PP:1800-1870`, `:3338-3380`, `:10776-10805`, `:8360-8400`, `:10876-10900`; `SP:1310-1365`, `:256-270`; and the re-scan of §2.5.
- **Other evidence, read with `grep` and a standard-library JSON walk:** the committed −0.0 counts (§4.1.3, §7) and the `ResidualRow` contexts. They can be reproduced with `grep -rho "normalized_residual: -0\.0[,} ]" P/fixtures`, and by walking every JSON file under `P/fixtures` for objects with a `kind` and a `value` of −0.0 (941 in 29 files).
- Python 3.11.15, run at `nice 19`, each under 2 s.
- **Not done:** no product build or run (the host is held); the fixture diff itself (S11-K and S11-F).
