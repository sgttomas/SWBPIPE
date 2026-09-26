# T3 V3: independent refutation of the RF-ELOAD references

This is the return of Type 2 TASK V3 for tranche T3, dated 2026-09-26. The brief is `T3/TASK_BRIEFS/V3_ELOAD_REFUTATION.md` (sha256 `bf38bab3…`), read with `_COMMON.md`. The references under test are `T3/REFERENCES_ELOAD/**`:

- candidate first committed at `2633c67fb`;
- reviewed at `f5a315280`, which adds the README-only checksum note;
- worktree head during the run: `cdf95af61`, then `f5a315280`, then `3b2fbe022`. The later commits do not touch `REFERENCES_ELOAD/`.

Hashes reviewed:

| File | sha256 |
|---|---|
| `references_eload.json` | `c20eb4c4662c172e8e29437b728346f18d57dc1a20fd1435821ddfd543c3aaba` |
| `references_eload.py` | `6bfd8b8b4f939b41bcfbe3db10d4a19bec374a47cc0cbd2c08dd35df903ebd10` |
| `README.md` (the version with the line-28 checksum note) | `0eb40a6cf71cdba85639030eabb51f02cb86ceeca0f47ba69209d4d44693f8ad` |
| `_run_records/SHA256SUMS` | `ad8497d7f1e88f6c448a02e81a40fbea502165f0e41cf303c7e1b7fe46d52d46` |

All five entries of `SHA256SUMS` verify from the `REFERENCES_ELOAD/` root. I did not author RF-ELOAD, I did not advise its author, and I have not read the T3 designs in depth.

## Verdict: FINDINGS

No expected value is wrong. There is 1 SHOULD-FIX, and nothing is BLOCKING or REFUTED.

- **Values.** All 2111 expected values and all 382 represented values agree with V3's third-route derivation. The largest difference is within 4.63e-40 of `max(|exp|, class scale)`.
- **Scales and bases.** These are all reproduced:
  - all 288 class scales, with their derivations;
  - all 1275 zero-valued keys, with their scales;
  - all 48 `finite_input` figures and basis decisions;
  - all 370 cancellation rows: `recommended_scale`, `gross_scale` and `governed_by`.
- **Negative controls.** All 213 were rebuilt from V3's own defect models:
  - every `discriminates` flag agrees;
  - 206 worst ratios agree to 1e-4;
  - the other 7 differ only in magnitude, because the control text underspecifies the defect (F4, F5, F6).
- **Regeneration.** It is byte for byte.
- **The SHOULD-FIX (F1).** A plausible W1b defect in the D-14 class passes every case: the generated intensity is formed exactly and then rounded once to binary64 before it enters the ledger. This is `NC-FLOAT-SUM` in CANCEL-SEIS. It does not discriminate there because of where these particular inputs round, not for any structural reason.

## 1. Method (a third route)

Both of the author's routes are replaced. The scripts are `v3_core.py` and `v3_derive.py`. Everything is exact `fractions`. π comes from my own Chudnovsky series at 170 digits, and it differs by 8.5e-171 from a separate Gauss–Legendre evaluation at 200 digits.

- **Element stiffness: exact flexibility inversion.**
  - The 6×6 end-j flexibility of the member clamped at i is formed from the closed-form cantilever compliances, in coordinate-free global form, and inverted exactly.
  - The other blocks come from the contragredient equilibrium transform Γ: K = [[Γ K Γᵀ, Γ K], [K Γᵀ, K]].
  - This replaces route B's closed-form projector blocks. There is no tree integration and no force method (route A).
- **Element loads: exact ODE solution.**
  - The fixed-fixed state is found by solving EI v'''' = q⊥ and EA w'' = −q_ax exactly, as Macaulay-bracket polynomials under the four clamped conditions.
  - Its end reactions are the fixed-end forces.
  - Eigenstrain and thrust enter the same fixed state as a constant axial force.
  - There is no Hermite shape-function integration (route B) and no Simpson integration (route A).
- **Recovery: superposition.** The fixed-fixed internal fields are taken from the ODE solution. The displacement-state fields come from statics from **end j**. Route B uses K_e u_e − f_eq and statics from end i.
- **Checks built into the solver.**
  - A self-check asserts that end-j statics reproduce the end-i actions.
  - Global force and moment equilibrium is asserted exactly in every solve: 96 base solves plus every defect solve.
- **Inputs.**
  - Every magnitude is recomputed from the case's `inputs` strings: sections, local-to-global intensities, ε\*, F_p and the generated intensities. Only the topology is taken from `model`.
  - The model's derived fields (`EA_over_pi`, `eps_exact`, `F_p_over_pi`, `global_intensity`, `generated_intensities`) are asserted equal to V3's values, and all of them are.
  - I did not reuse the author's helper functions. From `references_eload.py` I read only its list of `def` names, via grep. I ran it on a copy only to check regeneration (§8).

## 2. Per-family agreement

The comparisons below are V3 against the package (`v3_compare.py`). "Normalized" means |V3 − exp| / max(|exp|, class scale).

| Family | Cases | Values | ≤ 5e-40 | Max normalized | Represented values (≤ 5e-40) |
|---|---|---|---|---|---|
| UDL | 11 | 438 | 438 | 3.39e-40 | |
| TH | 8 | 353 | 353 | 4.63e-40 | |
| PT | 5 | 192 | 192 | 1.44e-40 | |
| CE | 4 | 181 | 181 | 3.23e-40 | |
| GEN | 6 | 289 | 289 | 2.84e-40 | 289/289 (max 3.19e-40) |
| PM | 5 | 220 | 220 | 3.58e-40 | |
| CANCEL | 6 | 270 | 270 | 1.85e-40 | 93/93 (max 1.48e-40) |
| COMB | 3 | 168 | 168 | 1.56e-40 | |

The brief asked for these to be re-derived in full, and they were:

- every cancellation case: CANCEL-FEM ×3, CANCEL-SEIS ×3 and CE-CANCEL ×2;
- every generated case, on both intended and represented inputs;
- every prescribed-motion case.

These also agree:

- The per-quantity `represented_vs_intended_per_quantity` tables.
- The `generated_intensities`: the intended exact value, `intended_over_pi`, the represented exact product, and the binary64 product, which is bit-identical to `repr`.
- `nonzero_below_class_scale`, up to exact ties.

## 3. Definitions (brief item 2)

Each definition was implemented independently from the manager's answers (`RF_ELOAD_DEFINITIONS.md`, `1f236a2d…`, accepted by ROOT). Every definition the package adopts is supported by those answers:

- **Thrust.** N = EA·ext/L − pA. The cap pair is −P_fixed; fixed-fixed gives −F_p and a free end gives 0 (Q1).
- **Effective wall.** One effective wall is used for A, I, J, the thrust area and the mass (Q6). EA/π = 334687500 for section G. m′/π = 20.24296875.
- **Wind.** Wind acts per unit member length along a global axis, with no projection (Q4). GEN-WIND-SKEW-PROP carries 87.88 N/m along +Y on e = (0,3,4)/5.
- **Consistent partial-span loading.** My exact ODE fixed state reproduces every partial case (Q5).
- **ε\*.**
  - Legacy: α·ΔT.
  - 0.4.0: λ_fit·λ_thermal − 1, with λ(T) = 1 + α_sec(T)(T − T_datum) and λ_fit = 1 + ΔL/L_member. This is T1's `signed_fit_states` and `thermal_datum_ratio` (Q7).
- **Constant effort.** A positive force along +axis in every solved case, with no stiffness (Q10).
- **Combinations.** Algebraic (Q11). A ± B superposed equals the direct solve exactly. Mb is the magnitude of the combined moment vector.
- **Prescribed motion.** R = K·d − f, with rotations in radians (Q8).
- **Other answers.** The local frame (Q3), the opposing fixed-end moments (Q9) and the self-weight drop (Q2 and ROOT's ruling) also match.

The hand calcs were reproduced with V3's solver (`v3_checks.py`):

- tp_phys_006: u_y = −7/500, θ_z = −13/3000, V_i = 4 and M_i = 8.
- tp_phys_008: N = −12. The local end forces are +12 at i and −12 at j.
- `constant_effort_support_applied_load`: u = 5/24, R_y = −3 and M_z = −30.
- `fixed_fixed_thermal_axial`: N = −5.4.
- `tp_pmm_p3_occloadgen`: m′ matches to 1.1e-15. The hand calc's figure is itself the binary64 product, which is the D-14 hazard's magnitude.

No definition is unsupported. F10 records two points for the harness mapping.

## 4. Scales (brief item 3)

**The author's both-zero translation rule is sound, not merely convenient (F7).**

- **The brief's premise.** R1 has no F·L³/(3EI) term. R1's code (`REFERENCES/references.py` lines 1028–1029) refuses a region in which translations and rotations are both zero. The author's branch is therefore a new extension, not a dropped term.
- **Where it applies.** Only 3 cases: TH-FF-LEG-AX, TH-FF-RES-122 and PT-FF-AX.
- **Why it is harmless there.** In all three, every translation and rotation row sits at a restrained DOF. That makes it `input_derived` under D1 rule 2a, with an exact zero that any implementation reproduces.
- **What the scale should be.** The derived scale is the axial motion that releasing the restraint would show: εL = 5.4e-3 m, or F_p L/EA. That is exactly the author's value.
  - An F·L³/(3EI) term is physically inapplicable, because no transverse load exists.
  - It would also inflate the scale 2600× (14.2 m in TH-FF-LEG-AX).
- **Too small or too large?** Neither. A released axial restraint fails by 1e9×, and the listed controls act on N and R.
- **Untested branch.** The conditional moment·L²/(2EI) branch is never exercised in this package.

**V1-S8 floor (`v3_floor.py`).**

- **How it was evaluated.** Per D1 §4.1.6 and §4.1.6.1:
  - revision 5a.1, sha256 `13c1a7d5…`; revision 5a.2, `fb62ef4a…`, is text-only and changes no quantity;
  - one body per case, with L_b the bounding-box diagonal;
  - restrained and prescribed rows treated as input_derived;
  - twist and extension given per-member kinds.
- **Thresholds and row sets.** R = 2^-34 and R = 2^-64/1e-9 were both tried. There are two row sets:
  - A: the package's quantities only;
  - B: A plus the element-local end shears and bending components.
- **Result:**
  - no nonzero expected value is below R·S\*;
  - no comparison scale is below it.
  - The closest approach is 4.9e-5·S\* (CE-NODAL `u.N1.UX`), about 8.5e5 times above the floor.

**Cancellation scales.**

- **Rows.** CE-CANCEL has 15 net-governed rows and 35 zero rows. CANCEL-FEM has 1 net-governed row (`th.S1.RZ`), 15 mixed rows and 43 zero rows. CANCEL-SEIS has 8 net-governed rows and 23 zero rows. These are recomputed from V3's own net-alone, other-loads and gross-alone solves.
- **Recommended scale.** It never exceeds the class scale. In CANCEL-FEM's 15 mixed rows it falls below |exp|, where the comparison is exactly relative.
- **Gross scale.** It hides every float-sum and product control, in all 8 cases.
- **The shared-node rotation.** Its net-governed scale is applied consistently:
  - the JSON row gives 2.152e-8 rad, which equals |exp| and equals the class scale, since it is the only nonzero rotation;
  - every control evaluation uses it;
  - README finding 4 agrees.
  - The gross value (2.08 rad) is never substituted. The same class scale serves as the zero scale for `th.S1.RX` and `th.S1.RY` (exact zeros) and, times L_c, for the translations.
- **One NOTE (F9)** on the review-only gross column.

## 5. The represented-input basis (brief item 4)

**The threshold.** Decoding moves the CANCEL-SEIS net by 1.42347e-11 at 1e5, 1.12034e-9 at 1e7 and 1.27288e-8 at 1e8, in `finite_input` terms. So G1e7 and G1e8 correctly take the represented basis under R1's RF-FINITE rule.

**Every case.** The rule was applied to every case. I recomputed all 48 `finite_input` figures. Apart from G1e7 and G1e8, CANCEL-SEIS-G1e5 is at 1.4e-11, and the other 45 cases are at most 3.4e-16.

**Margin at 1e7.** The margin at 1e7 is thin (1.12). In a sweep of 400 values of D, each written to 20 significant digits, only 38 % would need the represented basis at 1e7, but 100 % would at 1e8.

**The D-14 hazard control still discriminates against the represented basis.** `NC-BIN64-PRODUCT` fails by 9.09051× at 1e7 and 90.9051× at 1e8; V3 agrees with the author to 6 digits. At 1e5 it gives 0.0994708.

**What the basis presumes (F3).** The represented basis presumes that generation reads the binary64 operands. This matches D1 §4.2: "all binary64 operands lifted exactly". A generation that is exact from the authored decimals would fail G1e8 by 8.57× and pass G1e7 at 0.857.

## 6. Negative controls (brief item 5)

### 6.1 Listed controls

All 213 were rebuilt (`v3_nc.py` and `v3_cancel.py`):

- 163 in the non-cancellation, non-COMB families;
- 44 in the cancellation cases;
- 6 in COMB.

Every discriminating control fails under the stated scale: the class scale, or the recommended scale in the cancellation cases. All 190 discriminating flags and all 23 non-discriminating flags are reproduced. The magnitude differences are all explained by definitions:

- `NC-LEVER-RULE` (4 cases) also lumps full-span loads 50/50 (F4).
- The TH-SERIAL `NC-ALPHA-TIMES-INTERVAL` and `NC-SUBTRACT-DILATIONS` controls also drop the fit (F5).
- COMB-DIFF `NC-MAG-SUM` is Mb_A − Mb_B (F6).
- In CANCEL-SEIS G1e7 and G1e8, `NC-FLOAT-SUM` reproduces only when w_s is read as the *represented* intensity (F1).

### 6.2 The 23 non-discriminating controls

The bound is the worst-case ratio to the criterion.

| # | Case | Control | Ratio | Reason, checked | Decision |
|---|---|---|---|---|---|
| 1 | TH-SPRING-LEG-r1e-06 | NC-LOST-SOFT | 0 | fl(EA/L) + 200 is exact: the ulp is 2.98e-8, and 200 is a multiple of it. The worst case over any k is 7.5e-11 of k, a bound of 0.075. The control is identical to the reference | **Retire** (F8). r1e-08 (537×) and r1e-12 carry it |
| 2–3 | CE-CANCEL-G1e5 | FLOAT-SUM GmnGp, nGmGp | 0.0097 | The ulp(1e5)/2 bound is 0.024 | Keep, labelled "lossy order within the criterion at G = 1e5". It anchors the low end, like RF-CANCEL G1e5 |
| 4–5 | CE-CANCEL-G1e5, -G1e8 | FLOAT-SUM GmGpn | 3.7e-8 | The gross cancels first, exactly. The residual is the decoding of 0.3 | Keep, labelled "benign order (exact)" |
| 6–11 | GEN-SEIS-CANT-AX, -CONT2-Q9, -WIND-LFRAME-FF, GEN-WIND-MARKED-CONT2, -SUBSPAN, -SKEW-PROP | NC-BIN64-PRODUCT | 1.1e-7 to 1.0e-6 | The product is about 1e-15 relative; with no cancellation it cannot be observed | Keep, labelled "D-14 not observable without cancellation". Not a mutation test (§7) |
| 12–14 | CANCEL-FEM-G1e5, -G1e7, -G1e8 | FLOAT-SUM ABn | 2.9e-8 | A + B is exact (Sterbenz). The residual is fl(−0.2) and one rounding | Keep, labelled "benign order (exact)" |
| 15–16 | CANCEL-FEM-G1e5 | FLOAT-SUM AnB, nAB | 0.0075 | The bound is 0.0094 | Keep, labelled "within the criterion at this ratio" |
| 17–18 | CANCEL-FEM-G1e7 | FLOAT-SUM AnB, nAB | 0.48 | The bound is 0.60, so it is structurally non-discriminating | Keep, labelled "within the criterion at this ratio" |
| 19–21 | CANCEL-SEIS-G1e5 | BIN64-PRODUCT, FLOAT-SUM-BIN64, FLOAT-SUM | 0.099, 0.105, 0.0026 | The bound at r = 1e5 is about 0.1 | Keep, labelled "within the criterion at r = 1e5" |
| 22 | CANCEL-SEIS-G1e7 | NC-FLOAT-SUM | 0.024 | Rounded-once generation. The bound is half an ulp(w)/net = 0.57, so it is structurally non-discriminating | Keep, with a corrected label (F1) |
| 23 | CANCEL-SEIS-G1e8 | NC-FLOAT-SUM | 0.24 | **Not structural.** fl(w_rep) happens to lie 0.021 ulp from w_rep; the bound is 5.70 | **Re-scale** (F1) |

### 6.3 Defects I constructed that the author did not list

A defect counts as caught when at least one value in some case fails the criterion.

| Family | Defect (V3) | Caught |
|---|---|---|
| UDL | X-EXTENT-FROM-J: partial fractions measured from end j | 5/7. It is invisible in CONT2-AX and CONT2-Q9 because [0.25, 0.75] is symmetric, so it is not a defect for those inputs |
| UDL | X-LEFT-HANDED-LOCAL-Z (z = y × x) | 2/2 |
| UDL | X-STATIONS-FROM-J (the Mb stations mirrored) | 11/11 |
| UDL | X-FEM-MOMENTS-ONLY-OMITTED: consistent end forces, no end moments | 11/11 |
| UDL, PM | X-REACTION-WITHOUT-F (R = K·d) | 11/11 and 1/1 |
| TH | X-EPS-DATUM-AS-INSTALL (ε = λ(T) − 1) | 4/4 |
| TH | X-FIT-TOTAL-LENGTH (λ_fit over the chain length) | 1/1 |
| TH | X-EIGEN-E-OF-FIRST-MEMBER | 1/2. In SERIAL only M1 is heated, so there is no effect |
| PT | X-THRUST-MEAN-DIAMETER, X-CAP-AT-J-ONLY | 5/5 and 5/5 |
| CE | X-CE-WRONG-AXIS, X-CE-DOUBLE | 1/1 and 4/4 |
| GEN | X-NO-CONTENTS-MASS, X-G-FACTOR-Y-Z-SWAPPED, X-CONTENTS-NOMINAL-ID | 3/3 each |
| GEN | X-INSULATION-ONE-SIDE, X-SUBSPAN-HULL, X-EXTENT-FROM-J | 4/4, 1/1 and 1/1 |
| PM | X-PM-ROTATION-TIMES-L, X-PM-FIRST-COMPONENT-ONLY | 2/2 and 4/5. In PM-SINGLE-FF there is only one component |
| CANCEL | X-OVERWRITE: a contribution assigned instead of accumulated | 8/8 |
| CANCEL | X-DECIMAL-EXACT-GENERATION (with D decoded) | Caught at G1e8 (8.57×); passes at G1e7 (0.857) |
| COMB | X-DIFFERENCE-OF-MAGNITUDES, X-B-MOMENT-ROWS-SIGN | 2/2 each |
| **CANCEL** | **X-ROUNDED-ONCE: the exact product of the binary64 inputs, rounded to binary64, then summed exactly** | **0/8: undetected (F1)** |

## 7. The D-14 hazard: recommendation

**The binary64 product.** Only CANCEL-SEIS-G1e7 and -G1e8 can tell it apart from exact generation (9.09× and 90.9×).

- Without cancellation it is 1e-7 to 1e-6 of the criterion. No reference case with a non-cancelling load can observe it. This confirms the author's finding 2.
- At 1e5 the bound is about 0.1.

**The rounded-once variant.** This is exact generation stored as binary64 before the ledger. D1 §4.2 exists to prevent exactly this, because it forms the intensity at p. The package catches it nowhere:

- at 1e7 it is structurally below the criterion (bound 0.57);
- at 1e8 it could be caught (bound 5.70), but the chosen g-factor places w_rep 0.021 ulp from a binary64 value, so it gives 0.24.

**Recommendation.**

1. **Keep** CANCEL-SEIS-G1e7 and -G1e8 on the represented basis as the reference-level D-14 gate.
2. **Re-scale G1e8** so that the rounded-once defect is caught as well (F1). With g_factor.Z = −0.23 and D recomputed to 20 digits:
   - NC-FLOAT-SUM gives 9.44×;
   - NC-BIN64-PRODUCT gives 89.6×;
   - `finite_input` is 8.55e-9, so the basis stays represented.

   In a g-factor search from −0.200 to −0.300 in steps of 0.005, 18 of 21 values give 2.2× to 9.4×. Only −0.200 (0.24), −0.225 (0.95) and −0.255 (0.23) are lucky.
3. **Test D-14 at kernel level too.** A unit test in F3 should assert that each `GeneratedUniform` source enters the ledger as the product of its lifted binary64 operands at p, with no binary64 intermediate. The reason is that no non-cancelling reference case can observe D-14, and at 1e7 only the product defect is visible, not the rounded-once one.
4. **State in the README** that the represented basis presumes generation from binary64 operands, as D1 §4.2 says (F3).

## 8. Sterbenz, formulation and reproducibility (brief items 6–8)

**Sterbenz.** In CANCEL-SEIS every two-term binary64 sum is exact. The Sterbenz condition y/2 ≤ x ≤ 2y holds, and exactness was checked with `Fraction`. It holds for both the binary64 product plus fl(D) and fl(w) plus fl(D).

The listed three-term orders lose the net:

- In CE-CANCEL, GmnGp and nGmGp fail by 9.93411× at 1e8 and give 0.0097 at 1e5. GmGpn is exact.
- In CANCEL-FEM, AnB and nAB fail by 7.69092× at 1e8 and give 0.481 at 1e7 and 0.0075 at 1e5. ABn is exact. The binary64 fixed-end moments w·L·L/12.0 are exact for all three G.

The claim "the loss comes from the product and input rounding, not the final sum" is true of the sum. It does not explain `NC-FLOAT-SUM`'s pass at 1e8 (F1).

**Formulation.**

- Euler–Bernoulli with no shear and small displacement: my flexibility has no shear term and reproduces everything.
- Section constants for N, G and B, with J = 2I, are exact.
- Units are SI.
- Q3 = [(1,2,−2), (2,1,2), (2,−2,−1)]/3 and Q9 = [(1,8,−4), (8,1,4), (4,−4,−7)]/9 (columns) are orthonormal with det +1, and are reconstructed from the case data. V3's unrotated solves rotate exactly onto the Q3 and Q9 cases, and every invariant is equal.
- Equilibrium is exact in every solve.

**Reproducibility.**

- `references_eload.py` was copied unchanged to `<scratch>` and run: exit 0, 3 s, empty stderr.
- The JSON is byte-identical to the package (`c20eb4c4…`), and the stdout is identical to `_run_records/references_eload.stdout.txt`.
- The record is `_run_records/regeneration.txt`.

## 9. Findings

| ID | Severity | Case and quantity | Evidence | Required change |
|---|---|---|---|---|
| F1 | SHOULD-FIX | CANCEL-SEIS-G1e8 (and G1e7): NC-FLOAT-SUM, all 8 net rows | (a) The control's intensity is fl(w_rep), the rounded *represented* product. That is the only reading that reproduces 0.0240477 and 0.240477. Read as fl(intended w_s), the control discriminates (1.16× and 11.6×). (b) fl(w_rep) lies 0.021 ulp from w_rep, so rounded-once generation, the defect D1 §4.2 exists to prevent, passes at 0.24. The bound at 1e8 is 5.70×. (c) README finding 3 credits the pass to Sterbenz; that is true of the sum, but this pass comes from the input coincidence | Re-scale G1e8 (for example g_factor.Z = −0.23, which gives NC-FLOAT-SUM 9.44×, BIN64 89.6× and a represented basis at 8.55e-9), or add such a variant. Reword NC-FLOAT-SUM: "w_s on the case's basis". Reword finding 3: at 1e7 the rounded-once defect is structurally below the criterion (0.57); at 1e8 it is detectable |
| F3 | NOTE | CANCEL-SEIS-G1e7/-G1e8 basis | The represented basis rejects decimal-exact generation at 1e8 (8.57×). It is consistent with D1 §4.2 ("all binary64 operands lifted exactly") and with R1's RF-FINITE rule | Say in the README that the basis presumes generation from binary64 operands |
| F4 | NOTE | NC-LEVER-RULE in UDL-CONT2-AX, -CONT2-Q9, -LFRAME-3D and PM-ELOAD-CONT2 | The ratios (1e9, 1e9, 2.02e8, 3.28e8) reproduce only if full-span loads are lumped 50/50 as well. With the lever rule on partial loads only, the ratios are 9.5e8, 8.9e8, 4.3e7 and 5.2e7, which still discriminate | Reword: "every element load lever-ruled (50/50 at full span)" |
| F5 | NOTE | TH-SERIAL-RES-FIT: NC-ALPHA-TIMES-INTERVAL and NC-SUBTRACT-DILATIONS | 2.05e8 and 2.16e8 reproduce only with the fit dropped as well. Composed with the fit, the ratios are 1.06e7 and 1.40e5, which still discriminate | Compose with the fit, or say that the fit is dropped |
| F6 | NOTE | COMB-DIFF: NC-MAG-SUM and NC-WRONG-DIFFERENCE | MAG-SUM there is Mb_A − Mb_B, giving 2.26e7; the true sum gives 8.53e7. WRONG-DIFFERENCE leaves Mb correct, so 19 values fail rather than 28 | Wording only |
| F7 | NOTE | Both-zero translation zero scale (TH-FF-LEG-AX, TH-FF-RES-122, PT-FF-AX) | This is an extension; R1 refuses the case. It is sound (the axial motion scale, §4), acts only on input_derived exact zeros, and hides no control. The moment branch is never exercised | State the justification. Do not call it a dropped R1 term |
| F8 | NOTE | TH-SPRING-LEG-r1e-06: NC-LOST-SOFT | The defect model is identical to the reference: the addition is exact, and the bound is 0.075 | Retire it from this case |
| F9 | NOTE | CANCEL-FEM `gross_scale` | Gross means span A alone. For span-B rows (R.S2.\*, Mb.M2.\*) it is 7.5e6 against an |exp| of 7.5e7. It is review-only, and no conclusion changes | Optionally, the larger of each span's response |
| F10 | NOTE | Harness mapping | (a) README §3 says "N, T, Mb, tw and ext do not change when i and j are swapped". That holds only if the station labels mirror too (i↔j, q1↔q3); stations are keyed from authored end i. (b) The tp_phys_008 station table prints +12 where the package's tension-positive N is −12. The convention differs, and the physics agrees | Harness notes; no change to the package |

(There is no F2. It was folded into F1.)

## 10. Questions for the manager

1. **F1, the form of the fix.** Should the author change CANCEL-SEIS-G1e8's `gen.g_factor.Z` from −0.2 to −0.23? Under that change:
   - D becomes 143.44076231504138435;
   - the rounded-once control fails by 9.44×;
   - the binary64 product fails by 89.6×;
   - the basis stays represented (8.55e-9).

   Or should the author keep G1e8 and add a variant case with those inputs? Either way the author regenerates the package, and the change should be refuted again.
2. **F3, the reading of D-14.** Does ROOT confirm that D-14's "exact from the user's inputs" means exact from the binary64 operands, as D1 §4.2 says? The frozen represented basis of G1e7 and G1e8 depends on that reading.

## 11. What I did not check

- **Product code.** I did not read product source (`P/core/**`, `P/apps/**`), product tests or fixtures. I did not read the author's solution methods; from `references_eload.py` I read only the `def` names, via grep.
- **What I saw before deriving.** While surveying the JSON schema, before V3's derivation had run, I saw these package figures:
  - the expected rows of UDL-CANT-PART-GLOB-AX, CE-CANCEL-G1e8, CANCEL-FEM-G1e8 and CANCEL-SEIS-G1e8;
  - their controls;
  - the models' derived fields.

  None of them was used as an input. The derived fields were only asserted against.
- **The product's row set.** D1's floor was evaluated from reference quantities, plus V3's local end components (row set B). It was not evaluated from the product's actual published row set.
- **Controls checked only by value.** Failing-value counts of the controls were compared only where they are printed above.
- **Curved bends** and the other exclusions in README §8 are out of scope.
- **I ran:**
  - `sh run_v3.sh _run_records <scratch>` (about 10 s, single-threaded, `nice -n 19`, Python 3.11.15, standard library only);
  - the author's script on a copy in `<scratch>`;
  - `sha256sum`.
- **I did not run** any cargo, npm or product build or test.
- **Writes.** I made no Git write, and I wrote nothing outside `T3/REFERENCE_CHECK_ELOAD/**` apart from `<scratch>`.
- **No change to protected material.** I proposed no tolerance, and I did not edit any frozen reference, fixture or criterion.

## 12. Record

**Read** (sha256):

| File | sha256 |
|---|---|
| `AGENTS.md` | `c8ce87ef…` |
| `agents/AGENT_TASK.md` | `1a13a5b0…` |
| `T3/TASK_BRIEFS/_COMMON.md` | `892a2e4e…` |
| `V3_ELOAD_REFUTATION.md` | `bf38bab3…` |
| `R1_ADDENDUM_ELOAD.md` | `12524b3a…` |
| `MANAGER_NOTES/RF_ELOAD_DEFINITIONS.md` | `1f236a2d…` |
| `ROOT_RULINGS_V1.md` (the D-14 and V1-S8 lines) | `6b35be8c…` |
| `ROOT_RULINGS_V2.md` (the RF-CANCEL lines) | `4addbdbc…` |
| `ROOT_SELECTION_REFERENCES.md` (grep only) | `6c7a6f0e…` |
| `REFERENCES/README.md` (§4) | `5a89bad9…` |
| `REFERENCES/references.py` (lines 1026–1037 only, the zero-scale rule) | `80d473a7…` |
| `REFERENCE_CHECK/RETURN.md` | `d2f8538f…` |
| `REFERENCE_CHECK/v2_floor.py` | `4a50026b…` |
| `REVIEW/RETURN.md` (the V1-S8 lines) | `75535fed…` |
| `DESIGN_NUMERICS/DESIGN.md` (§4.1.6, §4.1.6.1, §4.2 D-14; read at `13c1a7d5…`, now `fb62ef4a…`, text-only) | — |
| H/`tp_phys_005` | `2ce5852f…` |
| H/`tp_phys_006` | `fe7b3588…` |
| H/`tp_phys_008` | `7e262f37…` |
| H/`fixed_fixed_thermal_axial` | `631512d5…` |
| H/`constant_effort_support_applied_load` | `6d819331…` |
| H/`imposed_displacement_spring` | `7007b544…` |
| H/`tp_pmm_p3_occloadgen_equivalent_static` | `4b602c33…` |
| H/`tp_pmm_p3_subspan_wind_exposure` | `065a8b15…` |
| T1 `generate_references.py`, read with `git show f3270ea79:` (the prescribed-motion and fit/thermal sections) | `42993028…` |

**Files** (the hashes are in `SHA256SUMS`, with paths relative to `REFERENCE_CHECK_ELOAD/`):

- Scripts: `v3_core.py`, `v3_derive.py`, `v3_compare.py`, `v3_cancel.py`, `v3_nc.py`, `v3_floor.py`, `v3_checks.py` and `run_v3.sh`.
- Outputs in `_run_records/`:
  - `v3_values.json`, `v3_compare.json`, `v3_cancel.json`, `v3_nc.json`, `v3_floor.json` and `v3_checks.json`;
  - the matching `*.stdout.txt`;
  - `regeneration.txt`, `package_sha256_check.txt` and `PYTHON_VERSION.txt`.

## Addendum: delta check of revision 1 (2026-09-26)

This addendum re-checks only the delta, as ROOT ruled in the last sections of `ROOT_RULINGS_V1.md` (at `35677f223`).

**Revision 1** is committed at `b6927f783`. The worktree head during the check was `b6927f783`. The revision-1 `_run_records/SHA256SUMS` verifies from the `REFERENCES_ELOAD/` root, all 8 entries.

| File | sha256 |
|---|---|
| `references_eload.json` | `8240765095481e08ca3153eb89ef7b5828018615f3dd774ff3eed5d7af4549c9` |
| `references_eload.py` | `3601db34fd17df3d2397bb8e1152faac90d04c58913b5fc54517017f36581f87` |
| `README.md` | `7a0c1aadde41f41222d1d50055e48d481067108c46f7f31a1505203fbb162c83` |
| `_run_records/SHA256SUMS` | `cc16cea55a9deeec8b26c034d1844442b255050ce447f9babe43cbd68f12c319` |
| `_run_records/SHA256SUMS.revision0` | `ad8497d7…`, the revision-0 sums, unchanged |

The revision-0 JSON used for comparison was taken from `git show f5a315280:…`, sha256 `c20eb4c4…`.

### Delta verdict: CONFIRMED

There are two wording NOTEs, N1 and N2 below. Neither needs a change before selection. I also correct one of my own revision-0 findings, F6.

### 1. The new case RF-ELOAD-CANCEL-SEIS-G1e8-R, by V3's own route

I ran the full V3 pipeline against revision 1: `sh run_v3.sh _run_records/rev1 <scratch>`.

**Values:** 31/31 expected values, all on the represented basis, agree within 2.34e-40 of max(|exp|, scale). The 31 represented values agree too. The keys match, including the 23 zeros.

**Case fields:** these are reproduced:
- the class scales and the zero scales;
- `finite_input`: 8.55448e-9 at R.N0.UZ, which means the represented basis;
- the per-quantity table and the generated intensities;
- all 31 cancellation rows (8 net, 23 zero) for `recommended_scale`, `gross_scale` and `governed_by`.

**D:** D = 143.44076231504138435 is −w_s(1 − 1e-8) to 20 digits, computed independently.

**The 4 controls,** rebuilt with V3's own models:

| Control | V3 result | Package result |
|---|---|---|
| NC-FLOAT-SUM | 9.43619× | 9.43619× |
| NC-BIN64-PRODUCT | 89.6350× | 89.6350× |
| NC-FLOAT-SUM-BIN64 | 89.6350× | 89.6350× |
| NC-NET-DROPPED | 1e9× | 1e9× |

All four discriminate under the recommended scale and under the class scale. None discriminates under the gross scale, except NET-DROPPED.

**Other checks on the new case:**
- The generated intensity w_rep lies 0.476 ulp from a binary64 value, so the rounded-once defect sits near its worst case (the bound is 9.9×).
- Sterbenz holds for both two-term sums.
- V1-S8: no row is below the floor, for either value of R or either row set.
- The package regenerates byte for byte on a copy: JSON `82407650…`, and stdout identical to `_run_records/references_eload.stdout.txt` (`_run_records/rev1/regeneration.txt`).

### 2. Byte identity of the pre-existing values

This is my own check, `v3_delta.py`. It is independent of `preserve_check_rev1.py`, which I did not run and did not read.

**Rows:** all 48 revision-0 cases are present. Every row is string-identical:
- 2111 expected rows;
- 382 `expected_represented` rows;
- 382 `represented_vs_intended_per_quantity` rows.

**Other case fields:** every other field is JSON-identical: inputs, model, classes, `zero_valued`, `finite_input`, `generated_intensities`, cancellation, notes and the rest. The only exception is the allowed new `cancellation.gross_scale_status`, in the three CANCEL-FEM cases.

**Top level:** only `summary` changed. It now reads 49 cases, 2142 values, 413 represented values, and 216 controls of which 194 discriminate, which is consistent with the changes.

**Controls:**
- no numerical field of any surviving control changed: `discriminates`, the counts, the worst key, the ratio, the observed and expected values, and the class and gross flags;
- 1 control was removed (NC-LOST-SOFT at TH-SPRING-LEG-r1e-06);
- none was added in the pre-existing cases;
- 16 `defect` texts were changed;
- 22 `label` fields were added.

### 3. Changed and retired controls

**F8.** NC-LOST-SOFT was retired from r1e-06 only; it stays at r1e-12 and RES-r1e-08.

**F4 and F5.** The new texts state exactly the defects the package computes. NC-LEVER-RULE applies to every element load, so full-span loads are lumped 50/50. In TH-SERIAL, ALPHA-TIMES-INTERVAL and SUBTRACT-DILATIONS replace ε\* entirely and drop the fit. My earlier rebuilds reproduce the package only under these readings (RETURN §6.1).

**F6 (my correction).** The computed COMB-DIFF control is **|Mb_A − Mb_B|**, and the author's text is correct.
- With the absolute value: 2.25838e7 at Mb.M1.i, 9 values failing. This matches the package exactly.
- Without it (signed Mb_A − Mb_B): 1.16222e8 at Mb.M2.j, 10 values failing, because Mb_A = 0 < Mb_B there.
- Revision 0's texts already said "absolute difference" and, for NC-WRONG-DIFFERENCE, "bending magnitudes left correct". My F6 misread them: I had checked only COMB-SUM's text.
- So F6 should not have been raised, and my RETURN §9 and ROOT's ruling text ("Mb_A − Mb_B") should read |Mb_A − Mb_B|. The revision-1 text "Mb_A + Mb_B" in COMB-SUM and "|Mb_A − Mb_B|" in COMB-DIFF matches the computation.

**F1.** The new NC-FLOAT-SUM text says the control rounds w_s formed exactly on the case's basis: the intended product on the intended basis, and the decoded-binary64 product on the represented basis. That matches what I reproduce:
- 0.00260101 at G1e5, which is fl(intended w_s);
- 0.0240477 at G1e7 and 0.240477 at G1e8, which is fl(w_rep);
- 9.43619 at G1e8-R.

README finding 3 is reworded correctly. It gives the bound at 1e7 as 0.57 and at 1e8 as 5.70, and correctly credits the G1e8 pass to the input coincidence rather than to Sterbenz.

**Labels.** The 22 labels are exactly my §6.2 decisions: the 21 kept controls, plus G1e8's benign-rounding label, which points to G1e8-R. Every non-discriminating control carries a label, and no labelled control discriminates.

### 4. README

- **F7.** §4 calls the both-zero rule an extension that R1 refuses, and names the 3 cases.
- **F10.** The harness notes in §3 are correct: station labels mirror under an i/j swap, and tp_phys_008 prints +12 where the package's N is −12.
- **D-14.** §2 and finding 1 state "exact from the binary64 inputs as the document stores them" (D1 §4.2, ROOT). The 8.57× figure is reported correctly.

### NOTEs

- **N1 (F9 wording).** `gross_scale_status` says span-B rows show "about one tenth of |expected|". The actual ratios of gross scale to |exp| are:

  | Row | Ratio |
  |---|---|
  | R.S2.UY | 0.1 |
  | R.S2.RZ | 0.2 |
  | Mb.M2.i | 0.4 |
  | Mb.M2.q1 | 2.0 |
  | Mb.M2.mid | 0.2 |
  | Mb.M2.q3 | 0.4 |
  | Mb.M2.j | 0.2 |

  Mb.M2.q1's gross scale is larger than |exp|. The column is review-only, so nothing binding is affected. Optional: reword it as "0.1 to 2 times |expected|".
- **N2 (README §4 wording).** The both-zero bullet still carries the parenthetical "(for example the fixed-fixed thermal, thrust and prescribed-free cases)". The same sentence says the rule applies only in the three named cases, and none of them is a prescribed-motion case. Optional: delete "and prescribed-free".

**Remaining magnitude differences.** My rebuilt LEVER-RULE and SERIAL ratios still differ from the package's in magnitude, as in RETURN §6.1. That is because `v3_nc.py` keeps my original reading. The package's figures reproduce under the reading its new text states (checked in revision 0), so this is not a finding.

### Record for the delta

- **New files:** `v3_delta.py` and `_run_records/rev1/*`. The latter holds the full pipeline outputs against revision 1, plus `v3_delta.json` and its stdout.
- **Revision-0 outputs:** the top-level `_run_records/*` files are the revision-0 run and are unchanged. `run_v3.sh` always runs against the current package.
- **Read:**
  - the last sections of `ROOT_RULINGS_V1.md` (`git show 9d9e8bfc7 35677f223`);
  - the revision-1 README (`diff` against revision 0);
  - the revision-1 JSON.
- **Not read:** `preserve_check_rev1.py`, and the author's code beyond running it on a copy.
- **Not done:** no Git write, no cargo or npm, and nothing written outside `T3/REFERENCE_CHECK_ELOAD/**` and `<scratch>`.
