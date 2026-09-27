# S11G_CHECK — V1 check of D1's S11-G formation-noise guard note

**Reviewer:** V1 (Type 2 TASK, independent reviewer), for the T3 WORKING_ITEMS manager, as ROOT ruled ("I4's F12 stop", its amendment, and "S11-G note: rulings, conditional on V1's check").
**Date:** 2026-09-27.

**Basis:**
- **Note.** `DESIGN_NUMERICS/S11G_GUARD.md` (sha256 `5fa3be8f…`) at `633460fb4`, and `_run_records/s11g_forecast.{py,json,stdout.json}` (`e97539b8…`, `e165a514…`).
- **Brief and rulings.** `TASK_BRIEFS/D1_S11G_GUARD_NOTE.md` with ROOT's addendum; `ROOT_RULINGS_V1.md` through `71f7fc6b9`.
- **Gate files.** `GATE/FORMATION_EXCEPTIONS.json` (`454bbc24…`, 14 rows) and `GATE/S11_EXCEPTIONS.json` (`1d8979f6…`, 221 triples in 18 cases).
- **Product sources, read-only.**
  - I4's uncommitted S11-F candidate, read in place and never written. Hashes as read:
    - `product_physics/src/lib.rs` `2441ecf2…`
    - `straight_pipe/src/lib.rs` `868a7cdd…`
    - `curved_bend/src/lib.rs` `1a41d6a4…`
    - `pressure_exact.rs` `b8563099…` and `pressure_exact/source_geometry.rs` `a0231707…`
    - `pressure_runtime.rs` `5a4f07f4…`
    - `source_recovery.rs` `f54d4ccd…`
    - FK `load_ledger.rs` `969b07f9…`
  - FK `lib.rs` (frame orientation).
  - I3's K-D5 PP diff (`product_physics/src/lib.rs` `999196ee…` as read).
  - I4's uncommitted `IMPLEMENTATION/S11F/_run_records/formation_rows/formation_rows.json` (`c548f519…`).

Line numbers below are `PP:n` in that S11-F candidate.

## Verdict

**NOT READY as written: one BLOCKING, four SHOULD-FIX.**

- **The load-row guard is right in design and coverage.**
  - It catches all 6 load-formation rows.
  - It demotes nothing among the 221 repaired triples, the passing frozen-reference rows, probe A and the committed fixtures.
  - The committed-byte forecast of zero is reproduced.
  - Its SHOULD-FIXes are specification repairs, not design changes.
- **The recovery-side decision does not survive ROOT's condition (b)(i).**
  - The INPLANE mechanism is not confined to synthetic scales or to the typed entry.
  - It occurs at realistic scale on inputs the captured entry accepts.
  - Under ROOT's own rule, **R-b ships in S11-G**.
  - The note must therefore add R-b, including R-b's committed-fixture and committed-byte forecast, which does not exist yet (B-1).

## ROOT's conditional points, answered plainly

**(a) Exact signed defect.**
- **The forecast script: yes.** Every defect in Parts A and A2 is computed in `Fraction`, and only the final value is rounded, for reporting. Part B is a bound screen, not a defect computation.
- **The specification: not quite (SF-1).**
  - The per-family accumulators A12 and A1 are exact.
  - The combination E = fl(round(A12)/12 + round(A1)) has binary64 intermediates: two roundings, a division and an addition.
  - Where exact-family and rounded-product defects cancel at a row, its absolute error is u·max(|A12|/12, |A1|), which is not relative to E.
  - The fix is small; see SF-1.
- **The stated-bound fallback is not conservative as written (SF-2).**
  - **Exact-pressure operands.** The source rounds at least five times. The half-ulp claim is false, and a γ₈-type bound is needed.
  - **Curved consistent vectors.** γ₆₄·‖f_source‖∞ ignores:
    - the cond(F) amplification of the F⁻¹ inversion;
    - cancellation in p_i = H·X + W_i;
    - the unspecified libm sin, cos and atan2.
  - **Materiality today.** None, because no committed model loads a realized curved span. Neither changes any forecast.

**(b)(i) Is the INPLANE class reachable only at synthetic scales on the typed entry? No. This condition fails.**
1. **The mechanism does not need G.** After S11-F, the N2 row is the correctly rounded net of (1e80, 1e-8, −1e80), which is exactly fl(1e-8). The `AssembledForce` is therefore bit-identical to that of a plain model with 50 N at N1 and an authored nodal 1e-8 N at N2. That model is legal on the captured entry and publishes the same rows. The captured entry's refusal comes from the 1e80 authoring, not from the mechanism.
2. **It occurs at realistic scale.**
   - The case is RF-LARGE-CONT-n00100 (AX and ROT, both modes): a 50-span, 6 m continuous pipe beam with 96–192 N nodal loads and no cancellation or extreme parameters.
   - It publishes rows **at or above the DESIGN floor** (|q| ≥ 2⁻³⁴·S\*, with S\* the §4.1.6 coupled body scale formed from R1's exact values). Their realized row-relative error is 202–298× (AX) and 8628–25984× (ROT) the 1e-9 criterion. These are the reactions R.S14.UY/UZ/UX and R.S34.UY, and the translations u.C37.UX/UZ and u.C13.UZ.
   - That is the INPLANE mechanism: a small quantity in a large body, carried by binary64 solve noise.
   - These rows pass R1's class-scale criterion, which is why P1 lists the case as passing. The INPLANE rows are breaches only because RF-CANCEL uses R1's net-governed scale. Under one criterion (row-relative, or DESIGN's relative-above-floor contract), both sets fail. Under the other (class scale), both pass.
   - The same mechanism appears in the synthetic RF-WEAK-W-3D/AX-rho1e-08 (4–13×, above the floor).
3. **D1's note contains no scan of realistic or committed models for this mechanism.** Part B covers only the load-row guard; Part C covers only P1's passing frozen references.
   - P1's `realistic_fixture_survey` (`DETECTION/results.json`) reports row-wise noise estimates fe·scale/|q| above 1e-9 in 30 of its 114 committed-fixture case-modes. Examples:
     - `results/invented/result_export_v0_2.json` load:L-100, up to 2.4e5;
     - `physics_source/fields.request.json`, up to 573.
   - These are estimates, not measurements. They are consistent with (2).

**(b)(ii) Exactly which 16 case-modes R-b would falsely demote.**
- My own re-implementation of R-b, from the note's definition and P1's published u and Mb, reproduces D1's set exactly: 28 rows in 16 case-modes over 146 passing case-modes.
- **None of the 16 is a committed product model.** All are R1 frozen references.

| Case (both modes) | Class | R-b rows | Realized row-relative error of those rows | DESIGN floor | Reading |
|---|---|---|---|---|---|
| RF-INVARIANCE-LFRAME-BASE, -OFF-1e3, -OFF-1e6, -RELABEL (8 case-modes) | Synthetic (invariance test; dyadic 2⁻¹⁶-scaled loads) | Mb.M2.i / Mb.E1.j | 0.05–0.22 | above | Genuinely false: the rows are accurate |
| RF-WEAK-W-3D-rho1e-08, RF-WEAK-W-AX-rho1e-08 (4) | Synthetic (soft member, E×1e-8) | Mb.A01.j, Mb.A2.j | 4.1–13.3 | above | Row-relatively wrong (same mechanism) |
| RF-LARGE-CONT-n00100-AX, -ROT (4) | **Realistic scale** (continuous pipe beam) | Mb.B14.j, A15.i, A37.j, B37.i | 0.53–876 | below | Row-relatively wrong, but below the floor, so within DESIGN's absolute bound |

- So R-b's false demotions are **not all synthetic** (LARGE-CONT is realistic scale).
- But (i) fails, so **under ROOT's rule R-b ships in S11-G.** That consequence is B-1.

**(c) Case-level demotion and the `source_eligible` gate.** The gate is correct, and necessary where it can act. It is harmless elsewhere. Details:
- Retained-source recovery runs only when the ordinary solve is Sensitive or errors (`needs_source_recovery`, PP:2583-2586).
- Recovery supports only nodal loads plus the load-state eigen terms: it refuses "authored non-nodal load family" (`source_recovery.rs` ~l.590-600).
- So the gate acts only on a case whose ordinary solve is already Sensitive and whose fired formed terms are eigen load-state products.
- There, recovery would solve the same represented terms at higher precision and re-qualify a case whose load row is wrong. Blocking it is right.
- The test D1 gives for this, T10, is vacuous; see SF-3.

## The six required checks

**1. All claimed formation rows caught: yes, for the 6 UDL rows. The 8 INPLANE rows are not caught (see (b)).**

| Case | Represented net | Exact defect | Threshold | Statistic / threshold (mine; D1) | Fires |
|---|---|---|---|---|---|
| UDL-W1e5 | 0.49166666668606923 | 1.94e-11 | 4.917e-10 | 0.03946; 0.0395 | no |
| UDL-W1e8 | **0.4916666902601719** | 2.3594e-8 | 4.917e-10 | **47.987**; 47.99 | yes |
| UDL-W1e80 | 2.6328e64 | 2.6328e64 | 1e-17 | 2.63e81; 2.6e81 | yes |

- **Independent emulation.** I emulated SP from `spanned_uniform_equivalent_terms` in source operation order. It reproduces I4's post-S11-F ledger row for UDL-W1e8, (0.2, −33333333.333333325, 33333333.625000015), **bit for bit**, and the net 0.4916666902601719 matches ROOT's figure.
- **Restrained rows.** No restrained row fires in any of the three cases, even against the smaller free-row scale.
- **Every entry and mode.** The guard acts on the ledger before the solve, so it covers all 4 UDL-W1e8 rows (captured and typed, dense and sparse) and both UDL-W1e80 typed rows. The captured entry still refuses 1e80.
- **The INPLANE per-row F2 justification**, checked against I4's measured post-S11-F records and my own solve emulation:
  - **R-a, R-b and R-c.** My R-b reproduces D1's result. R-b catches all 8 rows, since B ≈ 3.6e-13 to 8.9e-13 ≫ 1e-9·1e-8 and 1e-8 > 2¹⁰·B. R-a is silent on them, as the note says.
  - **"The error is in u."** Only partly. In D1's emulation, exact recovery from the binary64 u leaves 145–2949×. In my product-faithful emulation (scaled dense Cholesky or RCM LDLᵀ with the residual gate), it leaves 28–2774×. Formation and u contribute comparable amounts.
  - **What holds.** The conclusion D1 needs stands: exact recovery alone does not clear the rows, and u at precision p is required.
  - **Per-row figures differ.** D1's per-row figures are emulation-dependent and do not match I4's measurement. I4 measured F-case Mb.M1.j at 3635 and Mb.M2.i at 628 (dense), the reverse of D1's labels. I4 measured M-case Mb.M2.i dense at 5767, against D1's emulated 1504. D1 did not emulate the sparse rows (I4: 2214 and 4346). See NOTE N-1.
  - **F2's stop rule does resolve them.**
    - F case: S\*_moment = max(50, L_b·50) = 100 N·m, so 2⁻⁶⁴·100 = 5.42e-18 < 1e-17 (margin 1.85). The floor is 2⁻³⁴·100 = 5.8e-9 < 1e-8, so the row is `relative_verified` (margin 1.7).
    - M case: 1.08e-18, with floor 1.16e-9.

**2. Nothing legitimate demoted: verified for the frozen references and committed fixtures, with one reachable false-demotion class outside both (SF-4).**
- **The 221 triples.** They sit in 18 cases, all nodal-only. Only RF-CANCEL-UDL-W1e5/-W1e8/-W1e80 carry element loads among R1's model cases. So E = B = 0 by construction.
- **Probe A.** At G = 1e8 and 1e80, the individual G-term defects are ±8.69e-9 and ±8.78e63, and they cancel exactly. The tip RZ signed sum is 1.85e-17 against 6e-10, a margin of 3.2e7. Tip UY defects are 0. Silent.
- **UDL-W1e5** is silent at 0.0395.
- **Committed fixtures.**
  - D1's forecast script reruns byte-identical, JSON and stdout.
  - My independent walker finds the same 164 entries in 98 files with formed loads.
  - The minimum screen margin is 1577.
- **SF-4 (outside both sets).** A fully collinear body under formed loads only fires. An example is a pure-thermal or pure-pressure straight run between anchors with free interior axial rows and irregular node spacing (`probe_thermal_skew`). Details are in SF-4.

**3. Committed-byte forecast: zero, reproduced for the load-row guard as specified.**
- Values, terms, `finish` and `ForceTerm` `Debug` are unchanged.
- `AssembledForce` derives `Debug` and would gain a field. I found no site in PP, source recovery or the receipt that prints it; T9 remains the guard (N-6).
- With R-b added (B-1), the forecast is **unknown**, and P1's survey suggests it may not be zero.

**4. Soundness of the departure from the brief (ROOT decision (a)): sound, with SF-1 and SF-2.**
- **Computable exactly in the product.**
  - SP's formula has exact coefficients after the scale-3 clearing of 1/3 and 2/3. The multi-factor products (q_local = T·q_g, L, L, bᵏ, Tᵀ) are exact expansions via FMA two-products.
  - `RoundedProduct` (fl(N·x)) is exact by one FMA.
  - These cover every straight ledger site; my site list matches the note's.
- **Signed cancellation cannot hide a real formation error at the row.**
  - For the Exact and RoundedProduct families, E is exactly (represented row net − intended row net) on the held operands. That is the row's actual formation error.
  - Cancelling defects therefore means the represented row is actually close to the intended row.
- **Two limits to state.**
  - N-3: the held-operand boundary makes errors formed before `axial_load`, before the SI conversion or before T invisible.
  - N-4: a row-wise load criterion is not a response-level guarantee under ill-conditioning.
- **Bound families.** Not conservative as written (SF-2).

**5. Case-level granularity and the gate:** see (c). This is correct, and ROOT has accepted it.

**6. Tests, mutations and the K-D5 boundary.**
- **Tests that do their job.**
  - T1–T5 carry real paths-differ preconditions.
  - T3 kills M4.
  - T4 kills M3.
  - T5 has nonzero individual G defects, and kills M4 and M6.
  - T6 kills M6.
  - T8 kills M2.
  - T1 and T2 kill M1, M5 and M9. Under M5 the thresholds become 0.033 and 3.3e70.
- **T10 is vacuous (SF-3).** It uses UDL-W1e8, whose ordinary solve is Passed, so recovery never runs. Recovery would refuse its element loads anyway. So neither M7 (drop the gate) nor M8 (demote before routing) is killed.
- **Mutations the plan does not have:**
  - the binary64 combination of E (SF-1);
  - the bound-family counts (SF-2);
  - the range-failure fallback paths (N-5).
- **K-D5 boundary: verified.** I3's actual PP diff edits only the body of `solve_preview_reduced_system`: its `assembly.solve` becomes `solve_with_formation_check`. S11-G edits other functions, the call site's `source_eligible` line, the ledger push sites and `append_integrity_report`. FK `structural.rs` and SA are K-D5's; FK `load_ledger.rs` and SP are S11-G's. No function is edited by both.

## Findings

| ID | Class | Finding | Fix |
|---|---|---|---|
| **B-1** | **BLOCKING** | ROOT's condition (b)(i) fails. The INPLANE mechanism is independent of G after S11-F, and is reachable on the captured entry. It occurs at realistic scale: RF-LARGE-CONT rows above the DESIGN floor are 202–25984× the row-relative criterion. The note has no realistic or committed scan for it. Under ROOT's rule, **R-b ships in S11-G**, so §4, §5, §7, §8 and §6.4 must change. **R-b's committed-fixture and committed-byte forecast does not exist.** P1's survey (30 of 114 committed case-modes with row-wise noise estimates above 1e-9) suggests R-b may fire on committed fixtures. That would collide with ROOT's "zero regeneration diff or stop" rule. | Add R-b to the note: rule, cost, write set, and behavioural tests with paths-differ preconditions, including an INPLANE row and an LFRAME-type accurate row. Add mutations. **Forecast R-b over the committed fixtures by script before implementation. If it fires on any, take that and the zero-diff rule to ROOT.** Record that R-b falsely demotes the 8 LFRAME case-modes, whose rows are accurate, and that it covers only member end bending (N-7). |
| SF-1 | SHOULD-FIX | E = fl(round(A12)/12 + round(A1)) has binary64 intermediates (ROOT condition (a)). Its error is absolute, not relative to E, when the families cancel. | Use one exact accumulator of 12·Σε: add −k·lo(a·b) as three `add_product(4k, lo)` calls, since 4k is exact. Then round once, or decide \|A\| > 12·(1e-9·max(\|n_int\|, S\*) − B) exactly. Add a mutation. |
| SF-2 | SHOULD-FIX | The bound fallbacks are not conservative as stated (ROOT condition (a)). **Exact-pressure operands:** `pressure_group_value` is fl(fl(fl(p·A_i)·c)·d), with A_i = fl(fl(π·r_i)·r_i) and c a correctly rounded sum. `Scaled::mul` rounds its mantissa, so there are at least 5 roundings, not half an ulp. **Curved consistent vectors:** X = −F⁻¹δ₀ carries cond(F)-amplified error (cond 348 for elbows, 1.7e4 for the small-angle bend in R5-4). p_i = H·X + W_i can cancel, and sin, cos and atan2 have no stated accuracy. So γ₆₄·‖f_source‖∞ is not a bound. No forecast changes: there are no committed curved-span loads, and the screen already charges 17·γ₁₆. | Exact pressure: γ₈·\|t\|, or an exact product chain. It is products only, so it is sound. Curved: a bound over intermediate magnitudes including cond₁(F) with a stated libm assumption, **or** fail closed (demote a case whose curved consistent-load term meets another term at a row), **or** re-form in `Wide` as K-D5 does for curved stiffness. State which. |
| SF-3 | SHOULD-FIX | T10 is vacuous. Recovery runs only for an ordinary Sensitive or error result (PP:2583-2586), and refuses element loads, so UDL-W1e8 never reaches it. M7 and M8 survive. | Use a case whose ordinary solve is Sensitive, whose loads are nodal plus load-state eigen terms, and whose eigen products fire the guard. First show that without the gate recovery is attempted and selected. |
| SF-4 | SHOULD-FIX | **A false-demotion class exists in ordinary models.** FK normalizes x = d·fl(1/\|d\|), so collinear members at irregular spacing get direction cosines that differ by an ulp. Example: along global x at stations 7.29 → 9.0, x = 0.9999999999999999. In a body whose free rows carry only such self-cancelling formed terms (a pure-thermal or pure-pressure case on a straight run between anchors, with free interior axial rows), the free-row S\* collapses to held-operand noise (about 1e-10·N). The guard then fires at 2e8–6e8× (`probe_thermal_skew`: three of four runs). The case's thermal stresses are exact, and the whole case loses Current. The committed and frozen sets contain no such body. A body with any bend has an N-level corner net and is safe. The free-row-only S\* that UDL-W1e80 requires is what exposes this, so no simple scale change separates the two. | Disclose this in the note for ROOT's decision: fail-closed availability loss on fully collinear formed-only bodies. Add a behavioural test pinning the chosen behaviour. |
| N-1 | NOTE | The INPLANE per-row figures in §2 and §5 are emulation-dependent. F-case labels are swapped relative to I4's measurement (dense: I4 M1.j 3635, M2.i 628). M-case M2.i measures 5767, against 1504 emulated. Sparse rows are not emulated. "The error is in u" should say "in u and in K_e·u, comparably; exact recovery alone leaves 28–2949×". | Update §2 and §5 from I4's records when they are committed (§11 item 1). |
| N-2 | NOTE | F2 resolves the INPLANE rows with modest margins: 1.85 on the stop rule and 1.7 on the floor in the F case. | None; recorded. |
| N-3 | NOTE | Held-operand boundary: `axial_load` (E·A·α·ΔT, p·A), SI unit conversion and T are treated as held. Formation error before them is invisible, for example equal intended thermal loads formed through different factorizations at a junction. | State this as W1's class, or extend the RoundedProduct chain upstream, where it is cheap because these are products. |
| N-4 | NOTE | A row-wise load criterion does not bound the response error under ill-conditioning. K-D5's ρ with f − E (§9's "later option") would give a response-level check. | Record for F2 or K-D5 follow-up. |
| N-5 | NOTE | There is no test of the range-failure paths: expansion overflow falling back to `Bounded`, a non-finite bound firing, and `SumError` firing. | Add one test, for example q near 1e307. |
| N-6 | NOTE | `AssembledForce` derives `Debug` and would gain a `formations` field. No printing site was found, but work units derived from text length have moved committed bytes before (S11-K). | T9 covers this; keep the field out of any `Debug` that reaches output. |
| N-7 | NOTE | R-b covers member end bending only. The same mechanism reaches reactions and displacements (LARGE-CONT R.S14/R.S34, u.C37). | State R-b's coverage limit; F2/W1a is the full repair. |

## What I ran

All runs used standard-library Python 3.11, single-threaded, `nice 19`, `PYTHONDONTWRITEBYTECODE=1`, from `T3/`. I did no Git writes, used no cargo, and built or ran no product code. Other agents' worktrees were read only; there, `git` was invoked with `GIT_OPTIONAL_LOCKS=0`.

| Run | Result |
|---|---|
| `DESIGN_NUMERICS/_run_records/s11g_forecast.py ../../../../../../.. <out>` | JSON and stdout byte-identical to D1's committed records |
| `REVIEW/_run_records/s11g_check/probe_s11g.py.txt <v1_dir> <out>` | Independent Parts A, A2, C and R (above). It imports nothing from D1; it uses only my own binary64 frame and stiffness emulation (`probe_d5_check.py`) |
| `probe_inplane.py.txt <v1_dir> <I4 formation_rows.json> <out>` | INPLANE in the S11-F state with my product-faithful solve: exact-recovery ratios 28–2774; I4's measured values listed |
| `probe_thermal_skew.py.txt <out>` | SF-4: 3 of 4 straight thermal runs fire (2e8–6e8×); the run with bit-identical direction cosines does not |
| Independent walker over committed JSON (inline) | 164 entries in 98 files with formed loads, matching D1 |

Records are in `REVIEW/_run_records/s11g_check/`. The scratch rerun and input hashes are in `rerun_hashes.txt`. `REVIEW/_run_records/SHA256SUMS` is refreshed.

---

## Delta check: S11-G note revision 2 (`a5137da0f`)

**Basis.**
- `DESIGN_NUMERICS/S11G_GUARD.md` revision 2 (sha256 `b414b88e…`).
- `_run_records/s11g_rb_forecast.{py,json,stdout.json}` (`2f5d2739…`, `0477c138…`, `0f3ead29…`).
- I4's input snapshot `_run_records/inputs/i4_formation_rows.json` (`c548f519…`).
- ROOT's conditional rulings, "S11-G note revision 2", at `1dc27ae80`.
- Product sources as read for the first check. PP:3075 lies in `solve_load_case`.

This check covers the delta only.

### Delta verdict

**NOT YET PASS: one BLOCKING (DB-1) and one SHOULD-FIX (DS-1). Both are text- or rule-level fixes. Everything else asked is confirmed.**
- **DB-1.** ROOT's point 2 requires that the SF-4 floor cannot be gamed. It can: I constructed a row where a real formation defect, 14,901× the criterion in the published displacement, is hidden by the floor. The fix is one rule change, verified below.
- **DS-1.** D1's claim that K-D5 would itself demote the INPLANE cases in the S11-F state does not hold. The 1.6× and 2.0× figures come from a pre-S11-F solve.
- After the DB-1 rule change, with a test and a mutation for it, and the DS-1 text correction, a diff check suffices for PASS.

### Answers to the manager's items

**1. R-b′.**
- **(a) All 8 INPLANE rows caught, each a Passed breach under today's predicate: confirmed.**
  - I recomputed B with my own code from R1's exact u, and took q from I4's measured post-S11-F values.
  - B/(1e-9·q) is 35527–88818 and q/B is 11259–28148, so both R-b clauses hold by margins of about 10⁴.
  - q/S\*_moment is 1.0e-10 in the F case (S\* = 100) and 5.0e-10 in the M case (S\* = 20), against the 2⁻³⁴ = 5.82e-11 floor. The margins are 1.72 and 8.6.
  - I4's ratios are 628–5767×. The ordinary solve is Passed: rcond 4.3e-3, and my emulation agrees.
- **(b) Committed forecast of 0 firings: confirmed for the committed envelopes, independently. Confirmed by byte-identical rerun only for the emulated models.**
  - My own walker, pairing and B (`delta_r2/probe_rb_committed`, nothing imported from D1) finds 62 envelopes: 87 case-envelopes with 248 straight end rows, and 3 skipped. That matches D1.
  - R-b′ fires on **0** of them.
  - R-b fires only at `load_reference_fallback_uz` end i, in both modes.
  - No other end comes within 10³ of R-b's first clause while resolved above noise.
  - D1's Part E, 96 committed models without an envelope, emulated exactly, reruns byte-identical with 0 firings. I did not re-emulate those models independently.
- **(c) The 12 false demotions are all synthetic: confirmed.**
  - My earlier independent R-b set, 16 case-modes, loses exactly the 4 RF-LARGE-CONT-n00100 case-modes under the floor, whose rows are below it.
  - That leaves 8 RF-INVARIANCE-LFRAME case-modes (B/(1e-9·q) = 1.035, borderline; rows accurate at 0.05–0.22×) and 4 RF-WEAK-W-3D/AX-rho1e-08 case-modes. All 12 are synthetic R1 constructions.
- **(d) R-b's one committed firing changes nothing: confirmed.**
  - The envelope's case quality is already `sensitive`.
  - q = 2e-6 is below 2⁻³⁴·S\* = 2⁻³⁴·1.79e5 = 1.04e-5, so R-b′ is silent anyway.
  - Under R-b, the no-op rule (§3.5) changes no verdict and no byte. T13 pins this.

**2. SF-4 floor.**
- **The collinear and pure-pressure runs are silent: confirmed** (`delta_r2/probe_sf4`). The worst stat/threshold is 6.2e-5, 3.2e-5 and 3.5e-5 for the thermal runs, and 7.4e-5 for the pressure-thrust run.
- **The 6 UDL catches are unchanged: confirmed.** P = 0 on every UDL row and on probe A.
- **Counterexample search: FOUND. The floor can be gamed (DB-1).**
  - **The model.** A UDL-W1e8-type fixed-end cancellation at a free translational row, S1.UY:
    - member A, L = 3, q_A = 100000000.1 N/m;
    - member B, L = 2, q_B = −150000000.15 N/m;
    - a nodal input of 1e-3 N;
    - S1's rotations restrained.
  - **The formation defect.** A's transverse fixed-end term carries a real defect of −1.49e-8, from fl(q_A·3). Against the intended net of 1.0e-3, that is **14901×** the criterion without the floor.
  - **The published error.** S1.UY is the body's only moving DOF, so the published displacement carries the same relative error, 1.5e-5.
  - **How the floor hides it.** Add two anchored thermal members along y meeting at S1, with axial_load N each. Their terms ±N cancel exactly, contribute nothing to the net, and set P = 2N.
    - With N = 1e6 the floor makes the statistic 0.0076 (silent).
    - With N = 1e4 it is 0.76 (still silent).
    - A second magnitude set is caught at 93× without the floor, and hidden at 0.00095 with it.
  - **Why D1's argument fails.** D1's soundness argument ("a self-equilibrated term's only defect is ≤ u·|t|") is true of those terms. But the floor raises the threshold for every defect at the row, including non-self-equilibrated ones.
  - **The fix, verified.** Apply the floor only to the self-equilibrated defect component. The row fires if |E_nonSE| + B > 1e-9·max(|n|, S\*), or if |E_SE| > 1e-9·max(|n|, S\*, 2⁻¹⁰·P).
    - This takes two exact accumulators instead of one. Each stays exact, and the decision stays exact.
    - It keeps the collinear and pressure runs silent (same figures), leaves the UDL catches unchanged, and catches all three counterexamples (14901×, 14901×, 93×).
    - SF-3's unreachability still holds, because self-equilibrated-only rows still cannot fire.
    - Add a test (T6b: the counterexample must fire) and a mutation (apply the floor to the whole E; killed by T6b).
- **Residual, disclosed as a NOTE (DN-4), not blocking.** A self-equilibrated-only junction with a genuine small net is hidden by any floor large enough to silence held-operand noise. Example: two collinear skew thermal members with N1 = 1e6 and N2 = 999999.999, giving a load-relative error of 126× the criterion. Every floor above about 1.1e-7·P hides all self-equilibrated-only defects, since each is ≤ u·P absolute. Such rows need N1 and N2 to agree to about 1e-9, which is contrived.

**3. SF-1: confirmed, with two NOTEs.**
- **Exactness.** The accumulator is exact:
  - 12·value and (12/scale)·c are exact two-products;
  - −12k·lo is three `add_product(4k, lo)` calls, each exact.
- **The decision.** It is exact: the sign of A, then a copy of A plus ∓12·(T − B) read by signum.
- **Conservative rounding.** T is rounded down, B up, and T − B down.
- **DN-2.** `lo = fma(a, b, −fl(a·b))` is exact only if a·b does not underflow (|a·b| above about 2⁻⁹⁶⁹). Add that to the range fallback. Also, the 12× scaling makes |value| > 1.5e308/12 overflow into `SumError`, which fires (fail-closed). T14 exercises this; it should be stated.
- **DN-3.** The binary64 constant 1e-9 is above 1e-9 by 6.2e-17 relative. For strict "rounded downward", use RD(1e-9), or define the criterion as fl(1e-9). This is immaterial.

**4. SF-2: confirmed conservative.**
- **Exact-pressure operands at γ₁₆.** My count from `source_bore_area_scaled` and `pressure_group_value` is about 9–10 relative roundings, all multiplicative:
  - r_i = od·0.5 − wall, correctly rounded once, and doubled by the square;
  - the π constant;
  - ×r_i twice;
  - ×p;
  - the coefficient's rounded exact sum;
  - ×coefficient;
  - ×direction.

  γ₁₆ covers that.
- **Curved consistent vectors as CannotBound.** Demoting is fail-closed, and so conservative by construction.

**5. SF-3: confirmed.**
- **The gate is unreachable end to end.** Source recovery admits only nodal inputs (zero defect) and load-state eigen pairs, which are self-equilibrated `RoundedProduct` terms. Under the floor, such rows reach at most 1.1e-4 of the threshold. That still holds under the DB-1 fix.
- **M7** is killed only at unit level (T10), as the note states. **DN-5:** add a source pin that the routing site actually calls the tested predicate.
- **M8** is killed by T1's `quality: Passed` report-text pin.
- **M7 + M8** is killed by T1's diagnostic-code-set pin (`SOURCE_BLOCK_RECOVERY_UNAVAILABLE` appears).

**6. NOTEs, tests, mutations and the K-D5 boundary.**
- **The INPLANE labels now match I4's records:** F Mb.M1.j 3635/2214, Mb.M2.i 628/4346; M Mb.M2.i 5767/4346, Mb.M2.j 628/628.
- **"The error is in u" is correctly qualified** (28–2949×).
- **The new tests and mutations:**
  - T6a, T11, T12, T13, T15 and T16 are sound, with paths-differ preconditions.
  - M11, M12 (the floor from all terms hides UDL-W1e8: 2⁻¹⁰·6.7e7 → T = 6.5e-5), M13, M15 and M16 are killed as stated.
  - **DN-1: one variant of M14 is not killed by T11.** The INPLANE members are axis-aligned, so T is a signed permutation, and Σ_c|T_kc||u_c| equals |Σ_c T_kc u_c| exactly. Replacing Σ|T||u| by |Tu| is invisible to T11. The signed-sum variant is killed. Add a skew-member unit test of `bending_formation_bound`.
- **The K-D5 boundary holds.** PP:3075 is in `solve_load_case`, where S11-G also edits the `source_eligible` line. K-D5 edits only the body of `solve_preview_reduced_system`. No function is shared.

**7. K-D5 on INPLANE (optional spot check): it does NOT demote in the S11-F state (DS-1).**
- **My figure.** With the S11-F force (the exact net at N2), K-D5's exact-residual EF gives 2·EF = 1.3e-6 to 3.2e-6 of the trigger, in both modes (`delta_r2/probe_kd5_inplane`: ρ = f − K_int·u exact, then K̃⁻¹ with the same factor, DESIGN-coupled S\*).
- **Where D1's figures come from.** D1's 1.60 and 2.00 come from `recal_d5.json`, whose `solve_product` uses the **binary64-folded (pre-S11-F) f** (`recal_d5.py` l.126-151). Its ρ uses the exact ledger f. So EF there measures the absorbed 1e-8 load on th.N2.RZ, which is the S11 defect that S11-F repairs, not the formation rows. `recal_d5.json`'s own `EF_ratio_coupled_with_folded_f` (1.0e-6 to 1.6e-6) is consistent with my S11-F figure.
- **The consequence.** After S11-F, R-b′ is the only catch for the INPLANE rows; there is no defence in depth.
- **What to correct.** §1, §5's K-D5 column and §6.4's "demotes (recal)", and ROOT ruling 5's premise. My emulation does not reproduce I4's published bits, so I3's merged-state run remains the final word. It should expect K-D5 silent.

### Delta findings

| ID | Class | Finding | Fix |
|---|---|---|---|
| **DB-1** | **BLOCKING** (ROOT point 2) | The SF-4 floor can be gamed. Self-equilibrated terms meeting and cancelling at a row raise the threshold for a non-self-equilibrated formation defect at the same row. Counterexample: 14901× the criterion in the published displacement, hidden at 0.0076 (N = 1e6) and at 0.76 (N = 1e4) | Apply the floor only to the self-equilibrated defect component (two exact accumulators). Add T6b, the counterexample, which must fire, and a mutation applying the floor to the whole E |
| **DS-1** | SHOULD-FIX | K-D5 does not demote the INPLANE cases in the S11-F state. D1's 1.6× and 2.0× come from a pre-S11-F folded-f solve | Correct §1, §5 and §6.4. Tell I3 to expect K-D5 silent on INPLANE after S11-F |
| DN-1 | NOTE | M14's \|Tu\| variant survives T11, because the members are axis-aligned | A skew-member unit test of B |
| DN-2 | NOTE | RoundedProduct exactness needs no underflow in the FMA. The 12× scaling overflows above about 1.5e307 (fires, fail-closed) | Add to the range fallback; state it |
| DN-3 | NOTE | The binary64 constant 1e-9 is above 1e-9 by 6.2e-17 relative | Use RD(1e-9), or define the criterion as fl(1e-9) |
| DN-4 | NOTE | Self-equilibrated-only junctions with a genuine small net are hidden by any noise-silencing floor (126× in the probe) | Disclose in the note and in the CHANGE_RECORD |
| DN-5 | NOTE | T10 pins the predicate, not its use at the routing site | Add a source pin |
| DN-6 | NOTE | The R-b′ floor margin on the F-case INPLANE rows is 1.72, and depends on the product computing S\*_moment = 100 from its published rows | None; recorded |

### What I ran (delta)

The same constraints apply: standard-library Python 3.11, `nice 19`, `PYTHONDONTWRITEBYTECODE=1`, from `T3/`, no Git writes, no cargo, other worktrees read only. The `<v1_dir>` holds V1's `probe_d5_check.py` and `probe_s11g.py`.

| Run | Result |
|---|---|
| `DESIGN_NUMERICS/_run_records/s11g_rb_forecast.py ../../../../../../.. DESIGN_NUMERICS/_run_records/inputs/i4_formation_rows.json <scratch>/out` | JSON and stdout byte-identical to D1's records |
| `delta_r2/probe_rb_committed.py.txt <v1_dir> ../../../../../../.. <out>` | Independent committed-envelope forecast: 87 case-envelopes and 248 end rows; R-b′ 0; R-b only at `load_reference_fallback_uz` end i |
| `delta_r2/probe_rbp_inplane.py.txt <v1_dir> <i4 snapshot> <out>` | R-b′ on the 8 INPLANE rows: all fire, with the margins above |
| `delta_r2/probe_sf4.py.txt <out>` | The floor on the collinear and pressure runs; the DB-1 counterexample; the split rule; the DN-4 junction |
| `delta_r2/probe_kd5_inplane.py.txt <v1_dir> <out>` | K-D5 EF on INPLANE in the S11-F state: 2·EF ≤ 3.2e-6 |

Records are in `REVIEW/_run_records/s11g_check/delta_r2/`. Scratch rerun hashes are in `rerun_hashes.txt`. `REVIEW/_run_records/SHA256SUMS` is refreshed.
