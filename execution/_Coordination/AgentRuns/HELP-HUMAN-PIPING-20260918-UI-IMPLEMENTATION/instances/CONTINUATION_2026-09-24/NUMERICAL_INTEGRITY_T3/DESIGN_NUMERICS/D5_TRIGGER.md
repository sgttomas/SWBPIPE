# D-5 — when W1 runs: the Passed-boundary trigger

D1 (TASK), 2026-09-26. This is a narrow addendum to `DESIGN.md` revision 3, which is committed at `9377f32db`. It replaces D-5 in `DESIGN.md` §9 and the trigger of §4.3. It answers ROOT's ruling on P1's skew Passed breach (`T3/ROOT_RULINGS_V1.md` at `d84e66bff`), ROOT's product-impact request, and the manager's relay of P1's early data.

- **Basis.** Product source `c61a540ea`, T1 `f3270ea79` (read with `git show`), and R1's frozen references at `c0f14201c`.
- **P1's figures.** Those quoted here come from the manager's relay of P1's early report. `T3/DETECTION/results.json` was not yet committed, so every P1 figure is marked **P1 (relayed)** and should be re-checked against that file.
- **Scope of work.** Standard-library Python emulation and scans only (§9). Nothing was built, no product code ran, and no Git write was made.

## 1. Answer in brief

1. **Why 122 fails and 345 passes.** It is formation error, amplified by conditioning.
   - The binary64 element matrix of the 3D-skew member (1,2,2)/3 does not have the soft mode in its null space. That mode is the rigid rotation about global X, held only by the 144 N·m/rad spring.
   - The in-plane member (3,4,0)/5 breaks that null space 26 times less. Its tip moves only along global Z, whose transform entries are exact.
   - Main's problem has condition about 3.8e7 for 122 and 8.6e5 for 345, P1 (relayed). Its forward error is then about 0.57·cond·u for 122, and it sits inside main's Passed band, because main's Sensitive boundary is rcond < √eps, that is cond > 6.7e7.
   - The emulation reproduces the class and the ordering (§2), though not the product's exact constant.
2. **The Passed band is bounded.** Main calls a case Passed only if cond ≤ 6.7e7. So the silent error of this class is at most a small multiple of 6.7e7·u ≈ 7.4e-9 relative to the quantity's class scale. ROOT's reopen trigger at 1e-7 sits above that.
3. **Recommendation.** Option **(a2)**: a first-order formation-error trigger, EF.
   - It re-forms each element more precisely, takes ΔK = K_rep − K_int, and makes one extra solve with the existing factor, EF = K⁻¹(ΔK·u + r).
   - It routes the case when 8·EF exceeds 1e-9 of any published nodal quantity's comparison scale.
   - In the emulation it fires on every breach it can see, apart from RF-CANCEL, which is S11's class and is fixed by the ledger. It has 7 false positives in the Passed band out of 154 cases.
   - It does not fire on any committed product model or on any invented realistic model, including a skew line at cond 2.3e7 whose actual error is about 1e-15.
   - It costs about one extra solve plus an extended-precision formation pass.
   - It can land early, as a kernel-local Passed→Sensitive demotion, once K3's `Wide` arithmetic exists (slice **K-D5**, T1-disjoint). After W1 lands, the same estimate routes to W1 instead of demoting.
4. **Rejected as the main trigger:**
   - **(a1)**, a condition-number trigger, because it demotes correct skew models;
   - **(b)**, "always run W1 for skew", because it misses axis-aligned formation error and reroutes a third of the covered cases;
   - **(c)**, "always run W1", because it multiplies runtime on every covered case and changes the identity of every covered result.
5. **ROOT's hard requirement.** "Passed must not be publishable for a case the reference suite shows breaching." It is met by (a2) plus a **mechanical backstop**: a VP-ROBUST gate test that fails if any R1 case is published Passed while one of its covered comparisons fails (§6).
   - RF-CANCEL's Passed breaches remain until S11-F, under ROOT's standing S11 no-interim ruling. D-5 cannot fix them, because they are load-fold errors, not formation errors. I flag this conflict for ROOT (§6).
6. **Interim containment.** None before K-D5, consistent with ROOT's ruling of `d84e66bff` (no interim; reopens at 1e-7 or a realistic multi-member model). Three owner-level options are set out in §8.

## 2. Why 122 fails and 345 passes (shown)

**The two cases** (R1's models):
- **122:** one N-section member (0,0,0)→(1,2,2), L = 3 m. Root translations are rigid. Root rotational springs are k_X = 144, k_Y = k_Z = 1e6 N·m/rad. The tip moment is (0.0048, 0.0096, 0.0096) N·m, a pure torque of 0.0144 N·m along the member.
- **345:** (0,0,0)→(3,4,0), L = 5 m, k_X = 86.4, and tip moment (0.005184, 0.006912, 0).
- In both, the soft mode is the rigid rotation of the whole member about global X, resisted only by k_X. The exact element has that rotation in its null space, so the exact answer is θ_X = T_X/k_X.

**The emulation.** `probe_d5_trigger.py` uses D1's binary64 element: binary64 local matrix, binary64 frame, two-stage Tᵀ(K T), binary64 assembly, and binary64 LDL. It is the same code as `probe_skew_precision.py`, imported unchanged.

| Case | Ordinary binary64 (ratio to criterion) | Represented matrix solved exactly (formation only) | Artificial stiffness of the soft rotation, r·K_rep·r / k_X |
|---|---|---|---|
| 122, y_ref (0,0,1) | 0.41 | 0.28 | 2.85e-10 |
| 122, y_ref (0,1,0) | 0.65 | 0.28 | 2.85e-10 |
| 345, y_ref (0,0,1) | 0.010 | 0.0097 | 1.08e-11 |
| 345, y_ref (0,1,0) | 0.032 | 0.021 | 2.16e-11 |
| 122 geometry, k_X = 1e6 / 1e4 / 100 / 1.44 | 0 / 0.012 / 1.67 / 44 | 0 / 0.004 / 0.41 / 28.7 | 4.1e-14 / 4.1e-12 / 4.1e-10 / 2.85e-8 |
| Axis-aligned sibling (5,0,0), k_X = 144 | 0.0006 | 0 | 0 |

**What this shows:**
- **Most of the error is formation, not the solve.** The represented matrix solved exactly already carries most of it.
- **It scales as 1/k_X.** The artificial stiffness is a fixed property of the represented element, about 4.1e-8 N·m/rad for 122, so its relative effect is that value divided by k_X.
- **The 3D-skew member breaks the null space about 26 times more than the in-plane member.** For 345 with y_ref (0,0,1), local z is exactly global Z. The rigid rotation about X then moves the tip only along Z, a direction whose transform entries are exact 0 and 1. For 122, all nine direction cosines are nonzero and inexact (1/3, 2/3). So the rounded products in Tᵀ(K T) mix every stiffness family into the soft mode.
  - The attribution to single families is not separable: summing the rounded products of one family alone gives exactly zero.
- **The axis-aligned member has no such defect.** This is not a general "skew is bad" effect, though. The same emulation shows axis-aligned formation defects in bending-soft chains (RF-CHAIN-A, RF-INVARIANCE-PINTOR; §5).

**Against the product, P1 (relayed):**
- 122 publishes rcond 2.6065e-8 (cond about 3.8e7), smallest pivot 5.36e-7 (sparse) and 3.43e-5 (dense), and guarded ratios about 2.5e-15 against a target of 1.56e-13. Refinement does not help. The observed error is 2.4e-9, that is ratio 2.43 dense and 1.21 sparse.
- 345 publishes rcond 1.1569e-6 (cond 8.6e5), about 45 times better conditioned.
- The emulation gives cond 5.7e7 for 122 and 7.9e5 for 345, which agrees on conditioning. But its 122 error, 0.3–0.65, is **about 4 to 8 times smaller than the product's**. The source of the gap is not identified from reading. Candidates are the product's frame and y reference choice in P1's adapter, and its local-matrix and transform operation order.
- So the thresholds below are set against product data (the ratio actual/(cond·u) = 0.57 for 122), with margin. The emulation is used for the class and for relative counts, not for constants.

**Quantity kinds at risk:**
- nodal rotations and translations along the soft mode (th.RX at both nodes; the tip u.UY and u.UZ, which the rigid rotation moves);
- the spring action that is proportional to that rotation (the RX spring moment k_X·θ_X), with the same relative error.

The rigid mode produces no member force, so member actions are not at risk from this class. P1's mismatches are th.N1.RX, th.N0.RX, u.N1.UY and u.N1.UZ, P1 (relayed).

## 3. The options

| | (a1) condition trigger | **(a2) formation-error trigger (recommended)** | (b) W1 for skew models | (c) W1 wherever it applies |
|---|---|---|---|---|
| **Rule** | Route when cond_est·u·c > 1e-9, c = 1 (cond > 9.0e6) | Route when 8·EF > 1e-9·max(\|q\|, scale) for any published nodal quantity, with EF = \|K⁻¹(ΔK·u + r)\|, ΔK from an extended-precision re-formation and r the published original residual | Route every case with a member not parallel to a global axis | Route every case in W1's coverage |
| **Inputs today** | `reciprocal_condition_estimate` is already computed (`FK/structural.rs:893`) | Needs extended-precision element formation (K3's `Wide`, or a double-double routine) plus one solve with the existing factor | Geometry only | — |
| **Extra runtime** | None | One element pass in extended precision (O(members), cheap next to the solve) plus one solve pair (O(nnz L), like one refinement step): about 1 to 2 times the ordinary solve | W1 on every skew case: a 128-bit solve plus 256-bit verification, estimated at 50 to 400 times the binary64 factorization (estimate; K6 measures it) | The same W1 cost on every covered case |
| **False negatives** | None in the emulation (§5). In principle cond·u can under-estimate if the formation perturbation is large relative to u; the product constant for 122 is 0.57 | None at factor 8 in the emulation. At factor 1 it misses RF-SKEW-A-CANT-OFF-122-r1e-08 (1.47 against EF 0.82). The residual term covers solve error. The risk is second-order terms, which only matter when EF is already far above 1e-9 | Every axis-aligned formation breach: bending-soft chains, torsion chains, PINTOR. They are not Passed on main today, but (b) gives no protection if one is | None within coverage, up to W1's own floor |
| **False positives** | Heavy on correct skew and chain models: 6 to 18 of 154 R1 cases in main's Passed band, depending on c. The invented skew line M11 (cond 2.3e7, actual about 1e-15) fires | 7 of 154 R1 cases in the Passed band, all near the criterion (actual 0.07 to 0.41). No committed or invented realistic model | Every correct skew model | Every correct model |
| **Before W1** | Demotes Passed→Sensitive (withholds Current) | Demotes Passed→Sensitive, in K-D5 | Would demote every skew model: not viable | Not available |
| **After W1** | Routes to W1 | Routes to W1 | Routes to W1 | Every covered result changes identity and receipt; every committed covered fixture is regenerated |

## 4. Reroute fractions over R1's frozen cases (216 cases at `c0f14201c`)

| Option | R1 cases routed to W1 or demoted | Notes |
|---|---|---|
| (c) | **207** (kernel lane) / **185** product-authorable | All except the 9 RF-MECH cases (refused before any method runs); the 22 directional-spring cases are kernel lane only; the 3 UDL cases are in W1b |
| (b) | **66** (kernel lane) / **44** product-authorable | Cases with at least one member not parallel to a global axis, including the generated ROT models |
| (a1), c = 1 | Emulated: fires on 61 of 154; **6 in main's Passed band** (cond ≤ 6.7e7) | The other 55 are already Sensitive or unresolved on main and route to W1 through the existing trigger |
| (a1), c = 8 | Emulated: 71 of 154; **16 in the Passed band** | |
| **(a2)**, factor 8 | Emulated: 62 of 154; **7 in the Passed band** | The 7 are RF-CHAIN-T-n03-r1e-06, RF-CHAIN-A-n05-r1e-06, RF-SKEW-T-CANT-OFF-122-r1e-04 (a product breach, so a true positive there) and four RF-INVARIANCE-LFRAME variants (actual 0.07 to 0.29) |

- **The emulation covers 154 cases.** The 62 it skips are 22 directional-spring, 16 too large for the dense Python emulation (n ≥ 100 members, including the n1000 and n10000 models), 6 generated, 6 that fail binary64 LDL (Sensitive or unresolved on main), 3 UDL and 9 RF-MECH.
- **Pending (P1):** product rcond and residuals for all main-authorable cases, and P1's search for further Passed breaches of this class. P1 (relayed) has found none outside RF-CANCEL and S11 probe A in any small case; RF-LARGE n1000 was still running.

## 5. Evidence that (a2) meets ROOT's requirement on R1

`sweep_d5_r1.py` emulates main's binary64 path on each of the 154 cases and compares every published nodal translation and rotation with R1's expected value under the unchanged predicate and R1's class scales. It evaluates three estimators on the same factor:
- **EB** (bound-based, from the product's own `transform_roundoff` form);
- **EF** (first-order, exact ΔK);
- the equilibrated **condition number** (main's rcond target).

- **Emulated breaches: 63 cases.**
  - 12 are RF-CANCEL (S11). No formation estimator sees them: EF is about 1e-6, EB about 3e-3.
  - The other 51 are formation-class cases at cond > 6.7e7, which are Sensitive on main in the emulation.
  - EF×8 fires on **every one of those 51** (0 false negatives). EB and (a1) at any threshold tried also fire on all 51.
- **EF tracks the actual error** within about ×0.3 to ×5 across the families (for example RF-CHAIN-A-n05-r1e-10: actual 2.2e3, EF 2.2e3; RF-CHAIN-A-n03-r1e-08: actual 25, EF 4.8). That is why a factor of 8 is used.
- **The product 122 case.** In the product, EF is formed from the product's own K_rep and u, so it measures the product's formation error directly, and the residual term adds its solve error. In the emulation EF = 0.285 and 8·EF = 2.3 > 1, so it fires; the product's actual ratio is 2.43.
- **What the sweep cannot show:**
  - member, reaction and spring-action quantities;
  - the skipped 62 cases;
  - the product constant.
  
  So the requirement is also enforced mechanically on the product, by the backstop in §6.

## 6. Meeting ROOT's requirement: the backstop

- **VP-ROBUST gate test, new: "no Passed breach".** For every R1 case in the product lane, if any covered comparison fails, the case's published M03 outcome must not be `Passed`. That means `Sensitive` or `unresolved` on main, or selected by W1 after F2. It is one assertion per case, and it runs on every candidate, including K-D5's.
- **Negative control:** disable the trigger. RF-SKEW-T-CANT-OFF-122-r1e-04 must then fail the gate.
- **RF-CANCEL conflict, flagged.** P1 (relayed, ROOT at `11d24caf5`) confirms that main publishes Passed breaches in RF-CANCEL (ratios 2.48 to 22.4), and ROOT's S11 no-interim ruling accepts them until S11-F. The D-5 trigger cannot catch them: they are load-fold errors, and a formation estimator sees only about 1e-6 of the criterion. So until S11-F lands, the gate test must list RF-CANCEL's G ≥ 1e7 cases as **known S11 exceptions**, by name, with ROOT's ruling cited. After S11-F they must pass. ROOT should confirm that this reading of its direction is intended.

## 7. Product impact

**Committed product models.** Main publishes its rcond and residual rows in the `Debug` report text. `scan_rcond.py` reads them from every committed fixture on main, and from T1's fixtures through `git show`.

| Model (distinct) | Quality | Published rcond | cond | P1's fe = (1/rcond)(max guarded ratio + u) | cond·u |
|---|---|---|---|---|---|
| Invented preview model (`preview_physics_invented_*`, 4 cases) | Passed | 1.3e-5 | 7.7e4 | **1.7e-9 to 2.4e-9** | 8.5e-12 |
| Invented model, precision variant (`invented_mechanics_result_precision_1_*`) | Passed | 3.3e-5 | 3.0e4 | 7.7e-10 to 9.9e-10 | 3.4e-12 |
| Unicode-id model | Passed | 6.1e-4 | 1.6e3 | 3.8e-11 | 1.8e-13 |
| physics_source mixed (pressure case) | Passed | 1.06e-3 | 948 | 2.9e-12 | 1.1e-13 |
| Connected and thermal models (5 fixtures) | Passed | 0.037 | 27 | 1.9e-13 | 3e-15 |
| T1 load_reference connected | Passed | 0.75 | 1.33 | 9.7e-15 | 1.5e-16 |
| T1 load_reference pressure | Passed | 1.0 | 1 | 1.1e-16 | 1.1e-16 |
| N05 and multicase family (source_blocks, physics_source, T1's load_reference_source) | **Sensitive** already | 1.06e-11 | 9.4e10 | — | — |

- **No committed Passed model is in the cond band 1e6 to 6.7e7.** The worst is the demo model at 7.7e4, and the demo model is axis-aligned.
- **P1's fe formula would demote the demo model** (fe 1.7e-9 to 2.4e-9 > 1e-9). The guarded ratio is an allowance-inflated bound: it includes `g/(1 − g)` with g = γ(m), about 2.5e-14 here, not the actual backward error. Used as a trigger it gives false positives on ordinary models. (a1) with u in place of the guarded ratio, and (a2), do not fire on it.

**Invented realistic models** (`cond_realistic.py` and `ef_realistic.py`; emulated equilibrated cond; 200 mm steel pipe):

| Model | cond |
|---|---|
| M2: 30 m run, anchors at both ends, 4 spring hangers | 2.2e3 |
| M3: one anchor, 3 hangers, free end | 5.4e3 |
| M1: anchor, rests, guide, free end | 2.1e4 |
| M4: L plus riser with hangers | 1.0e5 |
| M5 and M6: a 45° leg | 5.6e4 and 5.0e5 |
| M7: M6 plus a soft rotational stabiliser (1e4 to 1 N·m/rad) | 4.9e5 |
| M9 and M10: 60 m, 30 members, hangers every 6 m | 5.6e5 and 1.6e5 |
| **M11: M9 on a skew line (2,1,0)** | **2.3e7** |
| M8: 3 m branch stub | 1.2e3 |

- For M11 and M9, EF (the actual formation error, first order) is **9e-16 relative for translations and 4e-15 for rotations**, far below 1e-9.
- ROOT's concern holds for conditioning: a realistic skew line with hangers and a guide reaches cond 2.3e7, inside the band. But its results are accurate. The load path does not drive the soft direction against a rounding-level defect.
- So **(a1) would demote M11 falsely** at c = 1 or c = 8, **(b) would demote or route M5, M6, the three M7 variants and M11**, and **(a2) routes none**.

**Demotions or reroutes of committed and invented models:**

| Option | Before W1 (Passed→Sensitive, so Current withheld envelope-wide today) | After W1 (routed to W1) |
|---|---|---|
| (a1) with c·cond·u, c = 1 | committed 0; invented 1 (M11) | committed 0; invented 1 |
| (a1) with P1's fe (guarded ratio) | committed: the demo model (2 fixture sets, 4 cases each); invented: every model with cond above about 4e4 (M4, M5, M6, M7, M9, M10, M11), assuming guarded ratios near 2.5e-14 as in the committed reports | the same |
| **(a2)** | committed 0 (expected; not computed on the product); invented 0 | committed 0; invented 0 |
| (b) | committed 0 (the demo model is axis-aligned; the other committed models were not checked for skew); invented 6 (M5, M6, the three M7 variants, M11) | invented 6 |
| (c) | n/a | every covered committed and invented case |

## 8. Owner-level framing

The data does not show broad demotion under (a2). It does under (a1) with P1's fe, and under (b). If ROOT takes D-5 to the owner:

1. **O1: precise trigger, early (recommended).** Land (a2) as the kernel slice K-D5, after K3, as a Passed→Sensitive demotion.
   - Until then no interim measure, per ROOT's `d84e66bff` ruling. Main's silent error in this class stays within a small multiple of 6.7e7·u ≈ 7e-9 relative.
   - Once W1 lands, the same estimate routes to W1.
   - **Cost:** 7 correct R1 cases in the Passed band are withheld until W1 (emulated); no committed model is affected.
2. **O2: immediate coarse containment.** Change main's Sensitive boundary now from rcond < √eps (cond > 6.7e7) to cond > 9e6, a one-constant kernel change.
   - It catches 122 today.
   - **Cost:** correct skew and chain models in the band lose Current until W1 (6 R1 cases emulated, and realistic skew lines such as M11). Any such model in the field loses Current envelope-wide, which is a product-semantics change the owner should accept knowingly.
3. **O3: W1 for everything.** Adopt (c) when W1 lands.
   - It gives the strongest guarantee within coverage.
   - **Cost:** runtime on every covered case (estimated 50 to 400 times the binary64 factorization), and every covered result changes identity and receipt, so every committed covered fixture is regenerated. It still needs a D-5 trigger for W1c and later families until they are covered.

## 9. The early slice K-D5 (O1)

- **Where.** Kernel-local and T1-disjoint, after K3 (`FK/structural/retained/wide.rs`).
- **Write set:**
  - `FK/structural.rs`: in `finish_checked_factor`, after the residual passes, compute EF and demote `Passed` to `Sensitive` when 8·EF > 1e-9·max(|q|, S*_kind) for any nodal quantity. S*_kind is the body scale of §4.1.6.1, computed from the binary64 solution.
  - new `FK/structural/formation_check.rs`: re-forms each frame element's Tᵀ K T from its binary64 primitives in `Wide<2>`, forms ΔK·u exactly through the `exact_sum` accumulator, and rounds once.
  - `SA`: passes each element's primitives (end coordinates, section terms, y reference) to FK beside the existing `transform_roundoff` evidence. SA already has them from `FrameElement`.
- **No `PP` change.** Sensitive handling already exists: `NUMERICAL_INTEGRITY_SENSITIVE` and retained-source recovery for in-scope signed-permutation cases.
- **The report.** EF goes in a separate evidence line, printed only when it demotes, as W2's b is (`DESIGN.md` §4.7). Non-demoted `Debug` reports stay byte-identical.
- **Tests:**
  - 122 demotes and 345 does not;
  - RF-CHAIN r1e-04 continuity controls do not demote;
  - the invented M11 does not demote;
  - every existing suite is byte-identical;
  - the committed-fixture diff is expected unchanged (no committed Passed model has EF near 1e-9);
  - the "no Passed breach" gate (§6).
- **Mutation:** drop the trigger, and the gate test fails on 122.
- **False-positive cost:** the demoted case, and today its whole invocation, loses Current until W1. That is 7 emulated R1 cases, all within a factor of 15 of the criterion.
- **Gates:** a full product slice as its own PR (as S11-K and K2a): independent review, hosted CI including the surface-4 dispatch, a clean DEC-025 sweep, and the fixture stop rule.
- **Serialization:** S11-K → K2a → K1 → K2b → K5, with K3 in parallel and **K-D5 after K3**. K-D5 also writes `FK/structural.rs` and `SA`, so it is serialized after K1 and before K2b, or after K5 if K3 is late.
- **After W1 (F2):** the demotion becomes routing. A case whose EF fires runs W1, and is published selected or unresolved, never Passed.

## 10. P1 expectations and negative controls

- **P1 (added to `DESIGN.md` §7.2).** RF-SKEW-T-CANT-OFF-122-r1e-04 is expected **Passed with a breach** on main: th.RX at both nodes and the tip u.UY and u.UZ, ratio about 2.4 dense and 1.2 sparse. That is the confirmed finding. RF-SKEW-T-CANT-OFF-345-r1e-04 is expected Passed and within the criterion (about 0.01).
- **Negative controls (added to `DESIGN.md` §7.3):**
  - (23) with the D-5 trigger disabled, 122 is published Passed and the "no Passed breach" gate fails;
  - (24) with the trigger computed from the bound EB instead of EF, M11 and the RF-CHAIN r1e-04 continuity controls demote, which is recorded as the false-positive control;
  - (25) with the factor 8 removed, RF-SKEW-A-CANT-OFF-122-r1e-08 is missed (emulated).
- **V-K and V-P:** the same cases through the kernel and product lanes.

## 11. Decision for ROOT

**D-5 (revised).** Choose one:
- **O1:** (a2) as K-D5, early, then routing to W1;
- **O2:** O1 plus an immediate coarse boundary change (cond > 9e6) now;
- **O3:** (c) when W1 lands.

In every option, the backstop gate "no Passed breach" is added, with RF-CANCEL listed as known S11 exceptions until S11-F. **D1 recommends O1.**

## 12. Run records (`_run_records/`)

| Record | What it holds |
|---|---|
| `probe_d5_trigger.py` → `probe_d5_trigger.stdout.json` | §2 |
| `sweep_d5_r1.py` → `sweep_d5_r1.json` and `sweep_d5_r1.stdout.json` | §4 and §5. Run from `T3/` with `REFERENCES/references.json` and a member limit of 12 |
| `scan_rcond.py` → `scan_rcond_main.json` (main's `P/fixtures`) and `scan_rcond_t1.json` (T1's changed fixtures, extracted with `git show f3270ea79` into a scratch folder) | §7 |
| `cond_realistic.py` → `cond_realistic.stdout.json`, and `ef_realistic.py` → `ef_realistic.stdout.json` | §7 |

- Everything is standard-library Python 3.11.15 at `nice 19`. The sweep and the realistic runs take a few minutes; the others take seconds.
- **Limits:**
  - the emulation does not match the product's constant for 122 (4 to 8 times smaller error);
  - M03's verdict is not emulated;
  - member and reaction quantities are not compared in the sweep;
  - P1's figures are relayed and pending `T3/DETECTION/results.json`;
  - W1's runtime multiple is an estimate until K6 measures it.
