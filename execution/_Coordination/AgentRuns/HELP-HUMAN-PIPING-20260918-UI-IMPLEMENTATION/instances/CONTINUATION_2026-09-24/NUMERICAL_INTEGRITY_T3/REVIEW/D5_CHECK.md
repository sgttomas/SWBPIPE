# T3 V1: narrow check of D1's D5_TRIGGER

Type 2 TASK V1, 2026-09-26. The T3 manager requested this check. ROOT's pre-acceptance of option O1 depends on it.

- **Scope.** Read-only. Standard-library Python, single-threaded, at `nice 19`. No cargo, no build, no Git write. T1 was read only through `git show f3270ea79`.
- **Input.**
  - `T3/DESIGN_NUMERICS/D5_TRIGGER.md` at `7dc2655f9` (sha256 `9ac9c12a…`, verified), with its run records `probe_d5_trigger`, `sweep_d5_r1`, `scan_rcond`, `cond_realistic` and `ef_realistic`.
  - ROOT's rulings at the end of `T3/ROOT_RULINGS_V1.md` (`77adc860e`).
  - R1's frozen references at `c0f14201c`.
  - The later update (`0d14db3b…`, at `e94af71f2`) is covered in §6.
- **Line numbers.**
  - Product code: main `c61a540ea`. `FK` = `P/core/solver/frame_kernel/src/`. `SA` = `P/core/solver/nonlinear_integration/src/structural_adapter.rs`. `PP` = `P/core/product_physics/src/lib.rs`.
  - `D5:n` is a line of D5_TRIGGER.

## 1. Verdict: **BLOCKING**

**The blocking item.** The trigger (a2) as specified computes EF = K⁻¹(ΔK·u + r) from two inputs (D5:73, D5:185-186):
- ΔK is formed element by element;
- r is the product's published binary64 original residual.

That estimator misses breaches that are Passed on main, and I demonstrate it. The missed cases are in the formation class and the solve class, at ordinary magnitudes, which is exactly what D-5 exists to stop.

**The fix is a small change to the specification.** Compute the residual of the *intended* system exactly, in one exact sum, EF = K⁻¹(f − K_int·u), where K_int is:
- every frame element re-formed from its primitives, local coefficients included;
- the springs;
- the prescribed coupling;
- the load ledger.

With that residual, EF equals the actual error to 1e-7 relative in every case I ran, and nothing is missed.

**The rest of D1's case holds:**
- (a2) is the right option.
- Rejecting (a1), (b) and (c) is well founded.
- The committed-fixture claim is reproduced exactly.
- The realistic-model conclusion holds.
- K-D5 is T1-disjoint.

ROOT's three questions are answered in §2 to §4, and the findings are in §5.

## 2. Q1: why D1's emulation is 4–8 times smaller than the product (probe A)

I rebuilt 122 on an emulation of main's actual path, as the code reads:
- the frame with `normalize`, which scales by 1/|v| (`FK/lib.rs:1146-1152`), where D1's probe divides by |v|;
- `local_stiffness` and the two-stage `multiply_transpose_left` (`FK/lib.rs:697-745`, `:1061-1080`);
- springs added with `+=` (`PP:1386-1390`);
- `prepare_structural`, with radix scaling and the symmetric average a_ij/2 + a_ji/2 (`FK/structural.rs:572-648`);
- dense `cholesky` in natural order (`:1010-1062`), or the RCM profile LDLᵀ (`sparse_direct/src/lib.rs:505`, `FK/structural.rs:1165-1246`);
- `estimate_rcond`, the residual check and refinement (`:796-988`).

D1's probe instead uses a textbook LDLᵀ in natural order.

| 122, k_X = 144 | cond (product estimator) | Dense ratio | Sparse ratio | Represented matrix solved exactly (formation only) |
|---|---|---|---|---|
| y_ref (1,0,0), and also (0,1,−1) | 3.837e7 (rcond 2.606e-8) | **2.413** | **1.214** | 1.035 |
| y_ref (1,1,1) | 3.837e7 | 2.400 | 1.214 | 0.983 |
| y_ref (0,0,1) | 3.837e7 | 1.586 | 2.042 | 0.175 |
| y_ref (0,1,0) | 3.837e7 | 1.579 | 1.214 | 0.175 |
| y_ref (2,−2,1) | 3.837e7 | 0.726 | 2.042 | 0.627 |
| P1 (relayed) | rcond 2.6065e-8 | 2.43 | 1.21 | — |

**What this shows:**
- **The gap is explained.** With y_ref (1,0,0), the emulation reproduces P1's figures to three digits in both modes, and P1's rcond to four.
  - It also reproduces the sign: in dense mode th.N0.RX comes out *below* the exact value (u_exact − u = +8.05e-14, where P1's relayed values give +8.1e-14). In sparse mode it comes out above.
  - The y_ref P1's adapter used should be recorded; (1,0,0) and (0,1,−1) both fit.
- **The breach is largely solve error, not formation error.**
  - The formation-only error is 0.17–1.04 of the criterion, depending on y_ref. The rest is the factorization's own error, and it depends on the mode: dense and sparse differ by a factor of two on the same matrix.
  - D1's reading, "most of the error is formation, not the solve" (D5:51), holds for D1's emulation but not for the product.
- **Implication for thresholds.**
  - D1's sweep and `ef_realistic` compute EF from formation only, K⁻¹(ΔK·u) (the `sweep_d5_r1.py` and `ef_realistic.py` docstrings). Against the product, formation-only EF under-reads the actual error by up to 12 times (0.175 against 2.042).
  - The factor 8 therefore covers 122 only narrowly, and only for some y_refs.
  - The design relies on the residual term to carry the solve error (D5:76, "The residual term covers solve error"). A binary64 residual does not do that reliably (§4).

## 3. Q2: can the re-formation itself be wrong in the band? (probe A, EF variants)

The re-formation can share each step below with main's binary64 formation. I computed EF each way against the exact first-order error.

| Shared step | Effect | Evidence |
|---|---|---|
| The product's rounded frame T (re-forming T^T K T exactly with the binary64 axes) | **Harmless.** A consistent rotation keeps the rigid-mode null space to O(u²) | EF equals the exact error to four digits in all 20 122-variants (`EF_Tshared`) |
| Binary64 section terms and coordinates (the represented inputs) | **Harmless.** A consistent perturbation of the section keeps the null space. R1 uses the represented basis where inputs are inexact | By construction; D1's `represented` flags |
| **Binary64 local coefficients** (reusing `local_stiffness`'s 12EI/L³, 6EI/L², 4EI/L and 2EI/L) | **Biased low.** Rounding those coefficients breaks the local rigid-mode identity 12 − 24 + 12 = 0 at first order. This is the whole formation error of an axis-aligned bending-soft member | Axis (3,0,0), k_Y = 2.2, cond 5.7e7, Passed: actual 0.931, and `EF_Lshared` 0.296, three times low |
| **Element-level ΔK** (leaving out spring addition, assembly sums and the symmetric average) | **Misses** absorption of a soft spring into a stiff diagonal | §4, probe C: 108 of 230 Passed breaches missed |
| **The binary64 published residual** in place of an exact one | **Misses** solve error. In the Passed band the true residual of a backward-stable solve is the same size as binary64 evaluation noise | §4, probe D: 28 of 105 missed; EF/actual as low as 1e-16 |
| The Wide re-formation's own rounding (about 2^-106) | Negligible: ≤ cond·2^-100 ≈ 1e-22 relative in the band | Analytic |

So D1's "from its binary64 primitives" (D5:186) must explicitly include the local coefficients, formed in `Wide` from E, G, A, I, J and L. The residual must be exact and taken over the assembled intended system (D5C-1).

## 4. Q3: can (a2) miss a formation-class breach that the 51 cases do not represent?

**The 51 emulated breaches are all at cond > 6.7e7, so all are Sensitive on main** (D5:103). None of them is in the band where (a2) must act. The one product case in that band is 122. So the claim of 0 false negatives rests on cases outside the target population.

**Classes I constructed in the Passed band (probes C and D), all single-member and product-authorable:**
- **(i) A soft spring partly absorbed by a stiff diagonal.**
  - The model: an axis-aligned member, whose element formation is exact, and a root spring k_X ≈ 0.05 N·m/rad added to G·J/L ≈ 7e5 with a loss of up to 1.2e-9.
  - Main publishes **230 Passed breaches** over the (L, k) grid, at ratios up to 3.5.
  - The product's contribution audit refuses only full absorption (`FK/structural.rs:406-411`). Its normwise amplification check fires only at 1 (`:894-901`).
  - Element-level ΔK plus the binary64 residual misses **108 of 230** (EF of about 1e-15).
  - With ΔK taken over the assembled matrix, all 230 fire, but EF/actual falls to 0.31.
  - The exact residual catches all 230, with EF/actual = 1.000.
- **(ii) Solve error only.**
  - The model: the same geometry with an exactly representable spring sum, so formation and assembly are exact.
  - Main publishes **105 Passed breaches**, at ratios up to 3.9, mostly dense Cholesky near cond 6.6e7.
  - Here ΔK is zero, so EF with the binary64 residual is noise: it misses **28 of 105**, with EF/actual down to 1.2e-16.
  - The exact residual catches all 105.

**The other items ROOT asked about:**

| Item | With the exact residual | As specified |
|---|---|---|
| Other geometries, several soft modes | Covered: EF is the first-order error vector, and first order is exact to ‖K⁻¹ΔK‖ ≲ 6.7e7·n·u ≪ 1 | Depends on noise (§3) |
| Near-mechanisms (cond close to 6.7e7) | Covered for the same reason; above 6.7e7 the case is Sensitive | — |
| Sparse path | Covered if EF is formed with the factor and u of the same mode. Dense and sparse errors differ by a factor of 2 on 122 | Same |
| Springs off the global axes | Kernel lane only (the product authors global-axis springs), so they are covered as contributions | — |
| **Curved bends, user stiffness elements, expansion-joint stiffness** | **Not re-formed** by `formation_check.rs` (D5:186 names only frame elements). Their formation error is invisible, so a fallback is needed (D5C-2) | Same gap |
| **Member end actions and reactions** | EF checks nodal quantities only (D5:185). Member actions recovered in binary64 from u can lose accuracy by about u·‖u_e‖/‖deformation‖. For the formation class the rigid mode carries no force (D5:67), but recovery cancellation is a separate class that EF does not see. The backstop gate covers R1's member comparisons only | Same (NOTE N-1) |
| Load formation (Tᵀ applied to element loads, thermal) | Covered only if the residual uses exact ledger terms (after S11-K). Otherwise load-formation rounding is outside EF | Same (NOTE N-2) |

## 5. Findings

**D5C-1 (BLOCKING): the specified EF misses Passed breaches. Compute the intended residual exactly.**
- **Evidence:** §4 (i) and (ii), and §3; probes C and D.
- **Required change.** Define
  - EF = K̃⁻¹·ρ, with ρ = f − K_int·u, one `ExactAccumulator` sum per free row, rounded once;
  - K_int = the Wide re-formation of every frame element from primitives (local coefficients included), plus the springs and every other stiffness contribution SA supplies, plus prescribed K_fc·u_c on T1's 0.4.0 route;
  - f = the load ledger's exact terms.
  - Do not use the published binary64 residual, and do not use element-level ΔK.
  - The product already builds exact contribution sums for its intended-action audit (`FK/structural.rs:395-411`, `:513-571`), so K-D5 extends that path.
- **Tests to add to K-D5:**
  - a solve-error-only control (probe D's class);
  - an absorbed-spring control (probe C's class);
  - an axis-aligned bending-soft control near the criterion.
- **Mutations to add**, each of which must fail one of those controls:
  - EF with the binary64 residual;
  - EF with element-level ΔK;
  - EF with binary64 local coefficients.

**D5C-2 (SHOULD-FIX): coverage of stiffness contributions that are not frame elements.**
- Curved bend macro-elements, user stiffness elements and expansion-joint stiffness enter through SA's contributions (`SA:88-163`), but `formation_check.rs` re-forms frame elements only.
- **Required change:** re-form each of them in Wide, or add their per-entry formation bound (`transform_roundoff`-style, `FK/structural.rs:1248-1285`) to ρ as an absolute term, or demote a case in which such an element carries a soft mode. State which, and add a curved-element control.

**D5C-3 (SHOULD-FIX): the byte-identity claim.**
- `StructuralReport` derives `Debug` (`FK/structural.rs:103-125`), and committed raws render it.
- Adding an EF field, even `None`, changes every committed raw, which contradicts D5:189 and D5:195 ("non-demoted `Debug` reports stay byte-identical"; fixture diff expected unchanged).
- **Required change:** keep EF out of the derived `Debug` struct, as a separate evidence record emitted only on demotion, or give it a hand-written `Debug`. Add a test that a non-demoted report is byte-identical.

**D5C-4 (SHOULD-FIX): calibration on the product-faithful path.**
- The factor 8, the 7 false positives in the band (D5:89) and the (a1) comparisons were derived from formation-only EF on D1's textbook LDLᵀ. Recompute them with the D5C-1 estimator on the product's algorithms: product normalize, symmetric average, Cholesky or RCM profile.
- With an exact residual EF ≈ actual, so false positives are cases whose actual error exceeds 1/8 of the criterion. They are no longer an artefact of estimator noise.
- ROOT's condition "thresholds from product data; 122 a required true positive" then holds by construction. 122 fires in every y_ref variant I ran (EF_exact equals the actual error, 0.73–2.41).

**D5C-5 (SHOULD-FIX): the named S11 exceptions.**
- D5:118 lists them as "RF-CANCEL's G ≥ 1e7 cases, by name", but gives no names. ROOT's ruling fixes the number at 12.
- **Required change:** the gate lists the 12 exceptions as (case, quantity) pairs, not as whole cases, so that a breach of another class inside an RF-CANCEL case is not masked. The list must match P1's frozen-reference results, and a test must require it to be empty once S11-F merges.

**NOTEs**
- **N-1.** Member actions and reactions are outside EF (§4). Ask P1's survey to include member and reaction comparisons for Passed-band cases, and state the limit in K-D5. W1's recovery before rounding addresses it after F2.
- **N-2.** Load formation is outside EF unless ρ uses exact ledger terms. Before S11-F the folded f is used, which is consistent with the RF-CANCEL exceptions. After S11-K the ledger is there to use.
- **N-3.** `ef_realistic` reports formation-only EF, so "actual error about 1e-15" (D5:21, D5:152) is not the actual error. On the product-faithful path, the actual first-order error for M9 and M11 is 5e-5 to 1.5e-4 of the criterion (1.5e-13 relative, probe B), so the conclusion holds.
  - The product's estimator gives cond 2.6e6 for M11 (D1 reports 2.3e7) and 3.2e5 for M9 (D1: 5.6e5).
  - So "(a1) would demote M11 falsely" (D5:154) is not supported with the product's estimator. This does not affect the choice of (a2).
- **N-4.** The emulated 122 error under D1's textbook LDLᵀ (0.3–0.65) should not be quoted as the product's. P1's y_ref should be recorded with its results.

## 6. The other checks

- **Committed models.** I reran `scan_rcond.py` on `P/fixtures` at main, whose fixtures are unchanged at the T3 branch head. The output is **identical** to D1's `scan_rcond_main.json`: 56 reports, 30 of them Passed. The largest Passed cond is 7.7e4, from the invented demo envelopes. **None is in 1e6–6.7e7.** D1's update (`0d14db3b…`) says those demo envelopes are historical outputs. That leaves the claim unchanged, and if anything stronger.
- **Realistic models.** All of D1's figures lie between 1.2e3 and 5.6e5, apart from the skew line. For the two models I rebuilt, the product estimator gives lower figures than D1's (N-3), and the actual errors are ≤ 1.5e-13 relative. **Claim confirmed**, with the corrections in N-3.
- **K-D5 write set.**
  - `FK/structural.rs`, the new `FK/structural/formation_check.rs` and `SA`. T1 (`f3270ea79`) changed no file under `P/core/solver/`, so K-D5 is **T1-disjoint**.
  - S11-K also writes `FK/structural.rs` and `SA`. D1's order (K-D5 after K3, around K1 and K2b) puts K-D5 after S11-K, as it must be.
  - Byte-identity when nothing is demoted needs D5C-3.
- **The named S11 exceptions.** ROOT's reading (12 RF-CANCEL breaches, removed at S11-F) is consistent with the S11 no-interim ruling. They need D5C-5's granularity.
- **D5_TRIGGER update (`0d14db3b…`, `e94af71f2`).** It adds the O1 pre-acceptance, the provenance of the demo envelopes, P1's fe survey (54 false positives, which supports rejecting fe) and the pressure-route headline note. **None of it changes the EF definition or any conclusion above.** D5C-1 to D5C-5 stand.

## 7. Run records (`T3/REVIEW/_run_records/d5_check/`)

The probes import each other by module name. To run them, copy each `*.py.txt` to `*.py` in one folder.

| Probe | What it holds |
|---|---|
| A: `probe_d5_check.py.txt` → `probe_d5_check.stdout.json` | 122 under ten y_refs; 345 under two; the absorbed-spring case; the bending-soft series; all EF variants |
| B: `probe_d5_realistic.py.txt` → `probe_d5_realistic.stdout.json` | M9 and M11 on the product-faithful path |
| C: `search_q3c.py.txt` → `search_q3c.stdout.txt`, and `search_q3d.py.txt` → `search_q3d.stdout.json` | The absorbed-spring class: the search, then the misses |
| D: `search_q3e.py.txt` → `search_q3e.stdout.json` | The solve-error-only class |

- The emulation is not a product run. It follows the product code as read at `c61a540ea`. It matches P1's relayed 122 figures (2.413 against 2.43, 1.214 against 1.21, rcond 2.606e-8 against 2.6065e-8), which is the check that it is faithful.
- Python 3.11.15, `nice 19`, `PYTHONDONTWRITEBYTECODE=1`. Hashes are in `T3/REVIEW/_run_records/SHA256SUMS`.
