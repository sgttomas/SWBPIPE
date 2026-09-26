# T3 P1: detection run on main (pre-change baseline)

This is the return of Type 2 TASK P1 for tranche T3, dated 2026-09-26. The brief is `T3/TASK_BRIEFS/P1_DETECTION.md`, at `45cfc92b1`, read with `_COMMON.md`. The run also covers the manager's in-run additions, relayed from ROOT:

- the settlement rule;
- re-evaluation against the frozen references;
- the M03-class search;
- the realistic-fixture survey;
- the row counts by kind.

It records what main publishes. It does not pass or fail main. **Every mismatch below is provisional** until the manager re-evaluates it.

## 1. Basis

| | |
|---|---|
| Product under test | main `c61a540ea`. `git archive c61a540ea projects/chirality-piping/{core,fixtures,schemas}` was extracted under `<scratch>`. No worktree product file was touched |
| Entry | `run_linear_static_preview_value_with_mode` (captured), in modes `sparse_interactive` and `dense_scrutiny`. Where capture refused, the typed `run_linear_static_preview_with_mode` was also run, as a secondary observation only (§6) |
| Standing | Main's own Rust `result_export::semantic_contract::numerical_use_standing_with_context(envelope, load-case refs, Some({request, solver_mode}))` |
| References, primary (frozen) | `c0f14201c` (`ROOT_SELECTION_REFERENCES.md` at `1d90276c3`). `references.py` sha256 `80d473a7351e92a2a903d233b9e633aac794aeff07f3ac41e40d25a0d94fc8a8`, `references.json` sha256 `7b176dbbf2296be02d8bca19c698ee56d0d5753751150a9175e0d3ad4cf89cc9` |
| References, cross-check (revision 1) | `3592032fa`. `references.py` sha256 `7fe459b12a02dfc75c9305fcab260ae8c8a528f07f4ac76a5be672547de99701`, `references.json` sha256 `57e3254e9ced0fa468945bbfde9d197eecf30bac3bd3e6e35f03739e54468a22`. The requests built from both revisions are byte-identical (222 of 222), and the comparison gives **0 verdict or ratio differences** between the revisions |
| Existing fixtures | `P/validation/benchmarks/numerical_integrity/fixtures.json` at `c61a540ea` (sha256 `c061d734…`) |
| Inputs | Every model input comes from `references.py` itself. The script is imported, its builders run, and each case's exact rational `defn` is rounded once per input with `float(Fraction)`. The printed JSON supplies only expected values, scales, RF-CANCEL recommended scales and negative controls. Section wall = (OD − ID)/2, rounded once. G is the stated G, or E/(2(1+ν)) rounded once. Springs along a global axis are authored as DOF springs |
| Criterion | Unchanged: `|obs − exp| ≤ 1e-9·max(|exp|, scale)`, evaluated exactly in rationals, with obs decoded exactly from its binary64 and published unit. RF-CANCEL uses the recommended (net-governed) column (ROOT_RULINGS_V2 §1). No expected value in any authorable case falls below the binary64 normal range (flag count 0). Such a value would have been compared absolutely against 1e-9·scale and flagged (V2 F8). Nonzero values below their scale are flagged `absolute_floor` per quantity |
| Toolchain | rustc/cargo 1.97.1 (`toolchain.txt`), `CARGO_INCREMENTAL=0`, `CARGO_TARGET_DIR=<t3-target>`, `--release --offline`. `--locked` was refused, because the probe root adds three path packages to the lockfile copied from `product_physics`. No registry version changed (`toolchain.txt`). Python 3.11.15, standard library only |
| Processes | One fresh process per case and mode, with `RLIMIT_AS` = 6 GiB (6442450944 bytes) set in the child. Timeout 600 s (1800 s for ≥ 1000 members). Wall time and peak RSS come from `wait4` |

**Mapping of published rows to reference keys.**

| Reference key | Product result it is read from |
|---|---|
| `u.*`, `th.*` | `result:disp:<node>:<dof>`. Displacements are published in mm and converted exactly |
| `R.*` | `support_reaction_component_v2` of the node's rigid support |
| `S.*` | Components of the spring support |
| `N`, `T` | −(end_i) and +(end_j) element-local actions. Both renderings are compared, and the quantity passes only if both do |
| `Mb.i`, `Mb.j` | hypot of the published local bending components, evaluated exactly |

`Mb.mid`, `tw` and `ext` are not product results. They are counted `not_published`, never as passes. The sign conventions are confirmed by the passing cases, for example every RF-ZERO, FIXTURE-N and CONT case at ratios of 1e-6 or less.

**Authoring correction during the run.** In the first pass, rigid supports that included rotations were authored without a family. Main infers such a support as a guide, which may not restrain rotations, and blocked it with `SUPPORT_INPUT_INVALID`. The committed N05 fixture authors partial rigid sets as `family: anchor`. The generator was corrected to do the same, and every case was re-run. Only the re-run is reported (`runs2`). The RF-SKEW finding in §3 is unaffected, because its supports are translations only and its request bytes did not change.

## 2. Summary by family (both modes)

A case is counted once per mode. The columns mean:

- **pass**: every compared quantity is within the criterion, or the case solved with no quantity reference (N03-RX).
- **mismatch**: at least one quantity breaches. Most mismatches are Sensitive and withheld; §3 lists every trusted one.
- **refused**: refused at capture, a blocked envelope, an error, or a required mechanism refusal.
- **resource**: a timeout or abort under `RLIMIT_AS`.
- **not_authorable**: the product cannot author the case. The reasons are in `results.json`.

| Family | Cases | Authorable | not_authorable | sparse: pass / mismatch / refused / resource | dense: pass / mismatch / refused / resource |
|---|---:|---:|---:|---|---|
| RF-CHAIN | 30 | 30 | 0 | 12 / 18 / 0 / 0 | 11 / 17 / 2 / 0 |
| RF-SKEW | 36 | 18 | 18 | 5 / 12 / 1 / 0 | 5 / 12 / 1 / 0 |
| RF-WEAK | 9 | 9 | 0 | 7 / 2 / 0 / 0 | 7 / 1 / 1 / 0 |
| RF-LARGE | 24 | 24 | 0 | 10 / 8 / 0 / 6 | 10 / 6 / 0 / 8 |
| RF-INVARIANCE | 25 | 19 | 6 | 11 / 8 / 0 / 0 | 14 / 5 / 0 / 0 |
| RF-RANGE | 32 | 32 | 0 | 1 / 1 / 30 / 0 | 1 / 1 / 30 / 0 |
| RF-ZERO | 4 | 4 | 0 | 4 / 0 / 0 / 0 | 4 / 0 / 0 / 0 |
| RF-FINITE | 6 | 6 | 0 | 6 / 0 / 0 / 0 | 6 / 0 / 0 / 0 |
| RF-MECH | 9 | 9 | 0 | 1 / 0 / 8 / 0 | 1 / 0 / 8 / 0 |
| RF-CANCEL | 41 | 41 | 0 | 17 / 13 / 11 / 0 | 17 / 13 / 11 / 0 |
| FIXTURE-N | 15 | 14 | 1 | 10 / 0 / 4 / 0 | 10 / 0 / 4 / 0 |
| FIXTURE-NP-A | 6 | 6 | 0 | 6 / 0 / 0 / 0 | 6 / 0 / 0 / 0 |
| FIXTURE-R | 6 | 4 | 2 | 3 / 0 / 1 / 0 | 3 / 0 / 1 / 0 |
| FIXTURE-NP | 3 | 0 | 3 | 0 / 0 / 0 / 0 | 0 / 0 / 0 / 0 |
| V1-CHECK-L | 2 | 2 | 0 | 0 / 0 / 2 / 0 | 0 / 0 / 2 / 0 |
| S11-PROBE-A | 4 | 4 | 0 | 2 / 2 / 0 / 0 | 2 / 2 / 0 / 0 |

Notes:

- **RF-SKEW, not_authorable 18.** T-PIN-AX, T-CANT-AX and A-CANT-AX need springs along the member axis (3,4,0) or (1,2,2). The product authors springs by global DOF only.
- **RF-INVARIANCE, not_authorable 6.**
  - PINTOR and LFRAME ROT-Q3 and ROT-Q9 need springs along a rotated axis.
  - PINTOR-UNITS-mm and LFRAME-UNITS-mm need N·mm moments and N·mm/rad springs. The units registry at `c61a540ea` has no such units, and converting them would approximate the case.
  - CONT4-UNITS-mm is authorable (mm, N, MPa) and passes.
- **FIXTURE, not_authorable 6.** N07 and NP-B, NP-C and NP-D are matrix-level. R02 and R03 inject a wrong matrix or load. R04, the prescribed displacement, is attempted and refused: `IMPOSED_DISPLACEMENT_REQUIRES_LOAD_STATE_MODEL`, since that model version has no load-state boundary motion. It is counted as refused and is the not-authorable settlement case (§5).
- **R01** publishes a residual-test candidate (r = 10), not the solution. Only its statically fixed reaction and axial force are compared.

## 3. Passed or Current-eligible results that breach the reference

Captured entry, both modes. Each row is one case and mode; every breaching quantity, with observed, expected, scale and ratio, is in `results.json` under `exceptions`. Counts: S11 class, 100 distinct (case, quantity) pairs in 15 cases (200 case, mode and quantity rows); other class, 4 distinct pairs in 1 case (6 rows).

| Case | Family | Mode | Identity | Quality | Standing | Mismatch / compared | Worst ratio (at) | Relative error at worst | rcond | fe estimate |
|---|---|---|---|---|---|---|---|---|---|---|
| RF-SKEW-T-CANT-OFF-122-r1e-04 | RF-SKEW | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 2 / 28 | 1.21395 (th.N1.RX) | 1.21e-09 | 2.61e-08 | 1.86e-06 |
| RF-SKEW-T-CANT-OFF-122-r1e-04 | RF-SKEW | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 4 / 28 | 2.4263 (th.N1.RX) | 2.43e-09 | 2.61e-08 | 2.06e-06 |
| RF-CANCEL-F-G1e7-GnG | RF-CANCEL | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 2.48353 (R.N0.UY) | 2.48e-09 | 0.00426 | 3.47e-12 |
| RF-CANCEL-F-G1e7-GnG | RF-CANCEL | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 2.48353 (Mb.M1.j) | 2.48e-09 | 0.00426 | 3.01e-12 |
| RF-CANCEL-F-G1e7-nGG | RF-CANCEL | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 2.48353 (R.N0.UY) | 2.48e-09 | 0.00426 | 3.47e-12 |
| RF-CANCEL-F-G1e7-nGG | RF-CANCEL | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 2.48353 (Mb.M1.j) | 2.48e-09 | 0.00426 | 3.01e-12 |
| RF-CANCEL-F-G1e8-GnG | RF-CANCEL | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 9.93411 (Mb.M2.i) | 9.93e-09 | 0.00426 | 3.32e-12 |
| RF-CANCEL-F-G1e8-GnG | RF-CANCEL | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 9.93411 (R.N0.UY) | 9.93e-09 | 0.00426 | 3.32e-12 |
| RF-CANCEL-F-G1e8-nGG | RF-CANCEL | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 9.93411 (Mb.M2.i) | 9.93e-09 | 0.00426 | 3.32e-12 |
| RF-CANCEL-F-G1e8-nGG | RF-CANCEL | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 9.93411 (R.N0.UY) | 9.93e-09 | 0.00426 | 3.32e-12 |
| RF-CANCEL-F-G1e8-GnG-ORTHO | RF-CANCEL | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 6 / 32 | 9.93411 (R.N0.UY) | 9.93e-09 | 0.00426 | 3.32e-12 |
| RF-CANCEL-F-G1e8-GnG-ORTHO | RF-CANCEL | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 6 / 32 | 9.93411 (R.N0.UY) | 9.93e-09 | 0.00426 | 3.32e-12 |
| RF-CANCEL-F-G1e8-GnG-INPLANE | RF-CANCEL | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 2 / 32 | 9.93401 (Mb.M1.j) | 9.93e-09 | 0.00426 | 3.16e-12 |
| RF-CANCEL-F-G1e8-GnG-INPLANE | RF-CANCEL | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 2 / 32 | 9.93401 (Mb.M1.j) | 9.93e-09 | 0.00426 | 3.47e-12 |
| RF-CANCEL-M-G1e7-GnG | RF-CANCEL | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 2.48353 (R.N0.RZ) | 2.48e-09 | 0.00426 | 3.16e-12 |
| RF-CANCEL-M-G1e7-GnG | RF-CANCEL | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 2.48353 (R.N0.RZ) | 2.48e-09 | 0.00426 | 3.02e-12 |
| RF-CANCEL-M-G1e7-nGG | RF-CANCEL | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 2.48353 (R.N0.RZ) | 2.48e-09 | 0.00426 | 3.16e-12 |
| RF-CANCEL-M-G1e7-nGG | RF-CANCEL | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 2.48353 (R.N0.RZ) | 2.48e-09 | 0.00426 | 3.02e-12 |
| RF-CANCEL-M-G1e8-GnG | RF-CANCEL | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 9.93411 (Mb.M2.j) | 9.93e-09 | 0.00426 | 3e-12 |
| RF-CANCEL-M-G1e8-GnG | RF-CANCEL | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 9.93411 (Mb.M2.j) | 9.93e-09 | 0.00426 | 3.32e-12 |
| RF-CANCEL-M-G1e8-nGG | RF-CANCEL | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 9.93411 (Mb.M2.j) | 9.93e-09 | 0.00426 | 3e-12 |
| RF-CANCEL-M-G1e8-nGG | RF-CANCEL | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 9 / 32 | 9.93411 (Mb.M2.j) | 9.93e-09 | 0.00426 | 3.32e-12 |
| RF-CANCEL-M-G1e8-GnG-ORTHO | RF-CANCEL | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 5 / 32 | 9.93411 (th.N2.RZ) | 9.93e-09 | 0.00426 | 3.63e-12 |
| RF-CANCEL-M-G1e8-GnG-ORTHO | RF-CANCEL | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 5 / 32 | 9.93411 (th.N2.RZ) | 9.93e-09 | 0.00426 | 3.33e-12 |
| RF-CANCEL-M-G1e8-GnG-INPLANE | RF-CANCEL | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 2 / 32 | 9.9342 (Mb.M2.j) | 9.93e-09 | 0.00426 | 3.01e-12 |
| RF-CANCEL-M-G1e8-GnG-INPLANE | RF-CANCEL | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 2 / 32 | 9.93415 (Mb.M2.j) | 9.93e-09 | 0.00426 | 3e-12 |
| RF-CANCEL-UDL-W1e8 | RF-CANCEL | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 1 / 41 | 46.4714 (th.S1.RZ) | 4.65e-08 | 0.8 | 4.61e-15 |
| RF-CANCEL-UDL-W1e8 | RF-CANCEL | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 1 / 41 | 46.4714 (th.S1.RZ) | 4.65e-08 | 0.8 | 4.61e-15 |
| S11-PROBE-A-G1e7 | S11-PROBE-A | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 6 / 6 | 3.25963 (th.tip.RZ) | 3.26e-09 | 0.0612 | 1.04e-13 |
| S11-PROBE-A-G1e7 | S11-PROBE-A | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 6 / 6 | 3.25963 (th.tip.RZ) | 3.26e-09 | 0.0612 | 1.15e-13 |
| S11-PROBE-A-G1e8 | S11-PROBE-A | sparse_interactive | preview-physics-1 | checks_passed | numerically_eligible | 6 / 6 | 22.3517 (th.tip.RZ) | 2.24e-08 | 0.0612 | 1.15e-13 |
| S11-PROBE-A-G1e8 | S11-PROBE-A | dense_scrutiny | preview-physics-1 | checks_passed | numerically_eligible | 6 / 6 | 22.3517 (th.tip.RZ) | 2.24e-08 | 0.0612 | 1.15e-13 |

### 3.1 Outside S11: the M03 class (ROOT's search)

The search covered every R1 family, the fixture cases and the supplementary cases, in both modes. **Exactly one case** is Passed or eligible, breaches, and contains no cancellation: **`RF-SKEW-T-CANT-OFF-122-r1e-04`**, in both modes.

The model is invented:

- one N-section member (0,0,0)→(1,2,2), with L = 3 m;
- N0 translations rigid;
- global rotational springs at N0: RX k = 144 N·m/rad (k/a₁ ≈ 1e-4), and RY, RZ 1e6 N·m/rad;
- a tip moment of (0.0048, 0.0096, 0.0096) N·m.

Published figures (M03-INTEGRITY-v1, Hager–Higham estimate; the full text is in `results.json` under `identity.integrity_diagnostics`):

| Mode | Factorization | rcond | Smallest pivot | Residual rows (guarded_ratio max; target) | Breaching quantities (ratio) | Relative error at worst |
|---|---|---|---|---|---|---|
| sparse | positive skyline LDL | 2.6065e-8 | 5.36e-7 | all passed (≈2.5e-15; 1.56e-13) | th.N0.RX 1.214, th.N1.RX 1.214 | 1.21e-9 |
| dense | positive dense Cholesky | 2.6065e-8 | 3.43e-5 | all passed (≈2.5e-15; 1.56e-13) | th.N0.RX 2.426, th.N1.RX 2.426, u.N1.UY 1.716, u.N1.UZ 1.716 | 2.43e-9 |

The values breach against the published rotation class scale, 3.3337e-5. For example, th.N0.RX is expected 3.3333333333e-5, and the dense run publishes 3.3333333252e-5, an absolute error of about 8.1e-14 rad against a criterion of 3.33e-14.

Input rounding is excluded: R1's `finite_input` for the case is 1.0e-15, so its basis is intended.

**Sibling `RF-SKEW-T-CANT-OFF-345-r1e-04`.** It passes in both modes (max ratios 0.0065 and 0.011), with rcond 1.1569e-6. It is the same variant with a different skew, and not only a different direction:

- axis (3,4,0)/5 with L = 5 m, against (1,2,2)/3 with L = 3 m;
- k = 86.4 N·m/rad against 144 N·m/rad (each k/a₁ = 1e-4);
- tip moment (0.005184, 0.006912, 0) N·m against (0.0048, 0.0096, 0.0096) N·m.

The 345 axis lies in the XY plane, so the stiff RZ spring is normal to it. The (1,2,2) axis loads all three springs, which makes its condition about 45 times worse.

**Reading.** Main's Sensitive boundary is rcond < √eps ≈ 1.49e-8. The case is Passed at rcond 2.6e-8, which is a condition number of about 3.8e7. A forward error near cond·u (about 8e-9) is then admissible under the gate, and that is far above 1e-9.

**Near misses.** These are Passed runs with a ratio above 0.1. All sit at rcond 3e-8 to 1.1e-7:

- RF-CHAIN-T and -A n03-r1e-06: up to 0.42;
- RF-LARGE-CHAIN-n00010-ROT: 0.23;
- RF-INVARIANCE-LFRAME, BASE, OFF and RELABEL variants: up to 0.20.

No other breach reaches 1e-7 relative. None is on a multi-member realistic model. The 122 case is single-member.

### 3.2 S11 (RF-CANCEL and P1's own probe A)

All of these are the predicted S11 outcomes (D1 §7.2) at R1's synthetic ratios. The manager and ROOT have assessed them as not meeting the realistic-model reopen trigger.

- **Nodal cancellation, F and M.** GnG and nGG breach at G = 1e7 (ratio 2.48) and G = 1e8 (9.93). Main matches R1's `NC-FLOAT-SUM-GnG` and `-nGG` fold controls in every such case. GGn passes at every G (max 3.8e-6), and G = 1e5 and 1e6 pass (max 0.0097 and 0.155).
- **G = 1e8 ORTHO and INPLANE** breach at 9.93. INPLANE fails only at Mb at N1, the value the net governs.
- **`RF-CANCEL-UDL-W1e8`** breaches only at th.S1.RZ, the net-governed rotation of the shared node, with ratio 46.47:

| Quantity | Expected | Main publishes (both modes) | R1 fold controls |
|---|---|---|---|
| th.S1.RZ | 2.275405069889944947747e-8 | 2.2754051756312433e-8 | NC-FLOAT-SUM-A-node-B and NC-FLOAT-SUM-A-B-node, both **distinct** from main (R1 fold values 2.2754050722e-8 (A-node-B, R1 ratio 1.01) and 2.2754050756e-8 (A-B-node, R1 ratio 2.53); main is 46.5 times the criterion from the reference, which implies a net moment at S1 about 4.6e-8 relative high) |

  So main's fold order is neither of R1's two modelled orders.
- **S11 probe A.** This is P1's own model, not a frozen reference: a 2 m N-section cantilever with global-y uniform loads (G, 0.3, −G) N/m, compared with the textbook closed form under the exact net w = 0.3 N/m, with EI = 1719500·PI_Q and scale = |exp|. G = 1e7 breaches on all 6 of 6 quantities (tip deflection 3.00, tip rotation 3.26, root shear, moment and reactions 2.48). G = 1e8 breaches on all 6 (up to 22.35). G = 1e5 and 1e6 pass. The tip values breach too, so on main the solve's force is folded as well.

## 4. Expected outcomes (D1 revision 2 §7.2, `S11_MAP.md`): confirmed or refuted

| Expectation | Observed on main |
|---|---|
| RF-CHAIN, k/a ≈ 1e-4 passes | **Confirmed.** Passed and pass in all six chains, max ratio 0.012 |
| RF-CHAIN, 1e-6 (continuity) | Passes, but n05 and n10 are already Sensitive and withheld. n03 is Passed at ratio up to 0.42 (near miss) |
| RF-CHAIN, 1e-8 and below: Sensitive misses or unresolved; order > 2, no exact-block recovery | **Confirmed.** Sensitive, needs_recompute, no recovery selected. Ratios run from 17 to 5.2e5 (up to 1.5e8 in dense). Two dense runs are unresolved. The sparse runs match `NC-LOST-SOFT` in 22 cases |
| RF-SKEW and FINITE soft cases miss at 1e-8 and below, exact-block unavailable | **Confirmed.** Every authorable 1e-8 and 1e-12 skew case is Sensitive, or unresolved and blocked. The represented-basis cases are compared on represented inputs. RF-FINITE passes everywhere (max 0.058) |
| **RF-SKEW at 1e-4 passes (continuity control)** | **Refuted for T-CANT-OFF-122-r1e-04**: Passed and eligible, with a breach (§3.1). The other five 1e-4 skew variants pass |
| RF-WEAK likely passes | **Confirmed** for W-AX and W-3D at every ρ (max 1.1e-5). W-L at 1e-8 and 1e-12 is Sensitive, or unresolved in dense |
| RF-LARGE | See §5. n10 passes. n100 CHAIN and TREE are Sensitive with misses. CONT passes at every size run |
| RF-RANGE and PHYS-R4: range refusals at the extremes | **Confirmed, in several forms.** See the RF-RANGE list below this table |
| **LEF-small: underflowed GJ becomes 0, or a spurious mechanism or wrong value** | **Refuted.** Main refuses earlier, with `PIPE_ELEMENT_INPUT_INVALID` "degenerate axis definition: element length". No zero, mechanism or wrong value is published |
| LEF-large: non-finite formation error | Not reached. It is refused at capture (|x| ≥ 2^53) |
| RF-MECH refused with a witness | **Confirmed** for LINE122-TORQUE and PERP, LINE345-RZ (the count trap), DISC-CHAIN100 and DISC-CHAIN100-SPRING (`NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM`). LINE122-UNLOADED is refused earlier as `LOAD_INPUT_MISSING`. K0 is refused as input (a spring with k = 0 needs explicit stiffness), plus the ground-DOF precheck. The companion LINE345-RX solves and passes, and `NC-FALSE-MECHANISM` is distinct from it. LINE-IN-CHAIN1000: see §5 |
| RF-CANCEL: GnG and nGG Passed and Current with the fold's answer, within 1e-9 at 1e5 and 1e6, failing at 1e7 and 1e8; GGn exact; net lost at 1e80 | **Confirmed** on the captured route at 1e5 through 1e8. The 1e80 cases are **refused at capture** (V1-S5), so the loss is not observable on the captured route. On the typed entry the net is lost (§6) |
| S11 probe A Passed with wrong actions at G ≥ 1e7 | **Confirmed**, and the solve's displacements are wrong too (§3.2) |
| Capture refusal of |x| ≥ 2^53 (V1-S5) | **Confirmed**: `CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT` in 11 RF-CANCEL cases, 19 RF-RANGE cases and both V1 check L cases |
| RF-ZERO and well-conditioned RF-INVARIANCE pass | **Confirmed.** RF-ZERO passes (max 7.8e-5). CONT4, including rotated and mm variants, passes (max 2.1e-4). LFRAME passes, as a near miss up to 0.20. PINTOR is Sensitive (withheld) with misses up to 3.3. TREE100 is Sensitive with misses up to 171 |
| Existing N01–N09, R, NP-A | N01, N08 and N09 pass (≤ 1.3e-6). N05, N06 and the NP-A ulp series are recovered by source-blocks-1: Sensitive or unresolved quality, **eligible via the receipt**, and pass (≤ 1.5e-7). N02-loaded, N03-RZ and N04 are refused as mechanisms. N02-unloaded (R07) is refused as `LOAD_INPUT_MISSING`. N03-RX solves Passed. R05 and R06 pass |

RF-RANGE outcomes, grouped:

- E−1000 and THIN-B are `NUMERICAL_INTEGRITY_UNRESOLVED`, which is PHYS-R4-like.
- L−240 and LEF-small are refused with `PIPE_ELEMENT_INPUT_INVALID` (a degenerate element length).
- Every +240, +960, SIM and LEF-large vector, and THIN-A, is refused at capture (`CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT`).
- F−960:
  - CHAIN reproduces its base case exactly, Sensitive with the same ratios (16.9 and 97.5);
  - SKEW is unresolved;
  - CONT is Passed and passes.

**Negative controls** (every control that publishes wrong values, evaluated at its listed keys with the case's scale):

| Family | NC evaluations (both modes) | main matches the wrong answer | partially | distinct from the wrong answer | not evaluable |
|---|---:|---:|---:|---:|---:|
| RF-CHAIN | 240 | 22 | 11 | 123 | 84 |
| RF-SKEW | 144 | 4 | 1 | 88 | 51 |
| RF-WEAK | 72 | 0 | 0 | 41 | 31 |
| RF-LARGE | 152 | 0 | 0 | 106 | 46 |
| RF-INVARIANCE | 84 | 0 | 0 | 70 | 14 |
| RF-RANGE | 184 | 0 | 0 | 8 | 176 |
| RF-ZERO | 18 | 0 | 0 | 18 | 0 |
| RF-FINITE | 24 | 0 | 0 | 12 | 12 |
| RF-MECH | 52 | 0 | 0 | 52 | 0 |
| RF-CANCEL | 322 | 48 | 0 | 80 | 194 |
| V1-CHECK-L | 4 | 0 | 0 | 0 | 4 |

The RF-CANCEL matches are the float-sum controls on the breaching GnG and nGG cases. Every non-cancel match (22 RF-CHAIN and 4 RF-SKEW) is on a Sensitive, withheld result. None is on a trusted one. "Not evaluable" means either that the reference publishes no wrong values for that control (non-discriminating, or outcome-type), or that the case was refused.

## 5. Memory and runtime (first M32 observation; observations, not limits)

These are fresh processes, one per case and mode, with `RLIMIT_AS` = 6 GiB. Peak RSS comes from `wait4` (`ru_maxrss`). The host has 15 GiB RAM and no swap, and another T3 task shared it.

| Case | Members | Mode | Outcome | Wall s | Peak RSS MiB | Solve s (in-process) |
|---|---:|---|---|---:|---:|---:|
| RF-LARGE-CHAIN-n00010-AX | 10 | sparse_interactive | pass | 0.0485 | 34.4 | 0.011 |
| RF-LARGE-CHAIN-n00010-AX | 10 | dense_scrutiny | pass | 0.0473 | 34.4 | 0.011 |
| RF-LARGE-CHAIN-n00010-ROT | 10 | sparse_interactive | pass | 0.0477 | 34.4 | 0.011 |
| RF-LARGE-CHAIN-n00010-ROT | 10 | dense_scrutiny | pass | 0.0477 | 34.4 | 0.014 |
| RF-LARGE-TREE-n00010-AX | 10 | sparse_interactive | pass | 0.0486 | 34.4 | 0.016 |
| RF-LARGE-TREE-n00010-AX | 10 | dense_scrutiny | pass | 0.0511 | 34.4 | 0.012 |
| RF-LARGE-TREE-n00010-ROT | 10 | sparse_interactive | pass | 0.0292 | 34.6 | 0.012 |
| RF-LARGE-TREE-n00010-ROT | 10 | dense_scrutiny | pass | 0.0475 | 34.4 | 0.012 |
| RF-LARGE-CONT-n00010-AX | 10 | sparse_interactive | pass | 0.049 | 34.4 | 0.012 |
| RF-LARGE-CONT-n00010-AX | 10 | dense_scrutiny | pass | 0.0484 | 34.4 | 0.012 |
| RF-LARGE-CONT-n00010-ROT | 10 | sparse_interactive | pass | 0.0479 | 34.4 | 0.012 |
| RF-LARGE-CONT-n00010-ROT | 10 | dense_scrutiny | pass | 0.0435 | 34.4 | 0.012 |
| RF-LARGE-CHAIN-n00100-AX | 100 | sparse_interactive | mismatch | 0.3518 | 49.6 | 0.201 |
| RF-LARGE-CHAIN-n00100-AX | 100 | dense_scrutiny | mismatch | 0.7872 | 49.9 | 0.645 |
| RF-LARGE-CHAIN-n00100-ROT | 100 | sparse_interactive | mismatch | 0.3515 | 49.8 | 0.205 |
| RF-LARGE-CHAIN-n00100-ROT | 100 | dense_scrutiny | mismatch | 0.7378 | 50.1 | 0.602 |
| RF-LARGE-TREE-n00100-AX | 100 | sparse_interactive | mismatch | 0.3432 | 50.5 | 0.205 |
| RF-LARGE-TREE-n00100-AX | 100 | dense_scrutiny | mismatch | 0.844 | 50.5 | 0.697 |
| RF-LARGE-TREE-n00100-ROT | 100 | sparse_interactive | mismatch | 0.3982 | 52.6 | 0.241 |
| RF-LARGE-TREE-n00100-ROT | 100 | dense_scrutiny | mismatch | 0.807 | 52.6 | 0.639 |
| RF-LARGE-CONT-n00100-AX | 100 | sparse_interactive | pass | 0.3745 | 52.3 | 0.191 |
| RF-LARGE-CONT-n00100-AX | 100 | dense_scrutiny | pass | 0.5787 | 51.6 | 0.393 |
| RF-LARGE-CONT-n00100-ROT | 100 | sparse_interactive | pass | 0.493 | 52.7 | 0.294 |
| RF-LARGE-CONT-n00100-ROT | 100 | dense_scrutiny | pass | 0.6112 | 51.7 | 0.434 |
| RF-LARGE-CHAIN-n01000-AX | 1000 | sparse_interactive | mismatch | 19.2245 | 3605.2 | 17.586 |
| RF-LARGE-CHAIN-n01000-AX | 1000 | dense_scrutiny | mismatch | 487.0382 | 3605.2 | 485.044 |
| RF-LARGE-CHAIN-n01000-ROT | 1000 | sparse_interactive | mismatch | 18.8612 | 3606.5 | 16.945 |
| RF-LARGE-CHAIN-n01000-ROT | 1000 | dense_scrutiny | timeout | 1800.2591 | 3606.7 |  |
| RF-LARGE-TREE-n01000-AX | 1000 | sparse_interactive | mismatch | 16.8072 | 3608.7 | 15.092 |
| RF-LARGE-TREE-n01000-AX | 1000 | dense_scrutiny | timeout | 1800.4501 | 3609.1 |  |
| RF-LARGE-TREE-n01000-ROT | 1000 | sparse_interactive | mismatch | 16.2946 | 3617.2 | 14.5 |
| RF-LARGE-TREE-n01000-ROT | 1000 | dense_scrutiny | mismatch | 451.3726 | 3617.2 | 449.535 |
| RF-LARGE-CONT-n01000-AX | 1000 | sparse_interactive | pass | 16.9031 | 3372.8 | 14.86 |
| RF-LARGE-CONT-n01000-AX | 1000 | dense_scrutiny | pass | 203.8639 | 3373.0 | 202.037 |
| RF-LARGE-CONT-n01000-ROT | 1000 | sparse_interactive | pass | 15.2203 | 3377.2 | 13.141 |
| RF-LARGE-CONT-n01000-ROT | 1000 | dense_scrutiny | pass | 214.8905 | 3377.2 | 213.06 |
| RF-LARGE-CHAIN-n10000-AX | 10000 | sparse_interactive | memory_refused | 13.2451 | 6140.4 |  |
| RF-LARGE-CHAIN-n10000-AX | 10000 | dense_scrutiny | memory_refused | 9.7748 | 6140.3 |  |
| RF-LARGE-CHAIN-n10000-ROT | 10000 | sparse_interactive | memory_refused | 8.3058 | 6140.4 |  |
| RF-LARGE-CHAIN-n10000-ROT | 10000 | dense_scrutiny | memory_refused | 9.719 | 6140.4 |  |
| RF-LARGE-TREE-n10000-AX | 10000 | sparse_interactive | memory_refused | 10.8033 | 6140.4 |  |
| RF-LARGE-TREE-n10000-AX | 10000 | dense_scrutiny | memory_refused | 10.6032 | 6140.3 |  |
| RF-LARGE-TREE-n10000-ROT | 10000 | sparse_interactive | memory_refused | 10.314 | 6140.2 |  |
| RF-LARGE-TREE-n10000-ROT | 10000 | dense_scrutiny | memory_refused | 10.815 | 6140.0 |  |
| RF-LARGE-CONT-n10000-AX | 10000 | sparse_interactive | memory_refused | 9.9488 | 6140.3 |  |
| RF-LARGE-CONT-n10000-AX | 10000 | dense_scrutiny | memory_refused | 11.8015 | 6140.2 |  |
| RF-LARGE-CONT-n10000-ROT | 10000 | sparse_interactive | memory_refused | 11.5703 | 6140.0 |  |
| RF-LARGE-CONT-n10000-ROT | 10000 | dense_scrutiny | memory_refused | 13.2934 | 6140.1 |  |
| RF-MECH-DISC-CHAIN100 | 103 | sparse_interactive | refused_blocked | 0.0517 | 44.8 | 0.038 |
| RF-MECH-DISC-CHAIN100 | 103 | dense_scrutiny | refused_blocked | 0.0491 | 44.8 | 0.03 |
| RF-MECH-DISC-CHAIN100-SPRING | 103 | sparse_interactive | refused_blocked | 0.0525 | 44.8 | 0.029 |
| RF-MECH-DISC-CHAIN100-SPRING | 103 | dense_scrutiny | refused_blocked | 0.0659 | 44.8 | 0.029 |
| RF-MECH-LINE-IN-CHAIN1000 | 1005 | sparse_interactive | refused_blocked | 2.2215 | 1697.4 | 2.082 |
| RF-MECH-LINE-IN-CHAIN1000 | 1005 | dense_scrutiny | refused_blocked | 2.1768 | 1697.4 | 2.065 |

- **Limits.** The 6 GiB `RLIMIT_AS` and the 600 s and 1800 s wall limits belong to P1's observation harness. They are not product or test limits, and they were not raised to force completion.
- **memory_refused.** Every n = 10000 case, in both modes, aborted after 8–13 s at a peak RSS of about 6.0 GiB: "memory allocation of 480048 bytes failed". 480048 bytes = 60006 × 8, one dense row of the 60006-DOF system, so the process got through part of the dense row-by-row allocation.
- **timeout.** RF-LARGE-CHAIN-n01000-ROT and RF-LARGE-TREE-n01000-AX in dense_scrutiny were killed at 1800 s. They had reached a peak RSS of about 3.6 GiB and were still solving. Their sparse runs completed in about 17–19 s. The other four dense n = 1000 runs completed in 204–487 s.
- These outcomes are observations of what main does under the harness limits, not failures of the reference.
- Reading: n = 1000 needs about 3.5–3.7 GB peak in either mode (D1 §2.1 estimated ≥ 3.8 GB). Sparse solves in about 15–20 s, and dense takes 200–500 s or more.
- **Settlement (S11B-1).** Main cannot author a support settlement on this route. The R04 prescribed-displacement request is refused with `IMPOSED_DISPLACEMENT_REQUIRES_LOAD_STATE_MODEL`, so it is recorded as not_authorable. T1's 0.4.0 route was not built.

## 6. Secondary observation: typed entry where capture refused

These results come from `run_linear_static_preview_with_mode`, the historical typed entry, and are **not** the primary route. The typed entry is reachable from `core/runner/headless/src/lib.rs:806` (`run_preview_in_memory_mode`). Its comment says it "cannot capture authored JSON or mint QualifiedPreviewEvidence". The Rust standing function returns `numerically_eligible` when P1 supplies an invocation, but P1 does not believe these results can become Current through the headless binding. The M03 quality is still `checks_passed`.

| Case | Mode | Outcome | Quality | Standing (with a P1-supplied invocation) | pass / mismatch | Worst ratio |
|---|---|---|---|---|---|---|
| RF-RANGE-CHAIN-L+240 | sparse_interactive | solved | sensitive | needs_recompute | 52 / 12 | 16.9494 |
| RF-RANGE-CHAIN-L+240 | dense_scrutiny | solved | sensitive | needs_recompute | 52 / 12 | 97.4792 |
| RF-RANGE-CHAIN-E+960 | sparse_interactive | solved | sensitive | needs_recompute | 52 / 12 | 16.9494 |
| RF-RANGE-CHAIN-E+960 | dense_scrutiny | solved | sensitive | needs_recompute | 52 / 12 | 97.4792 |
| RF-RANGE-CHAIN-F+960 | sparse_interactive | solved | sensitive | needs_recompute | 52 / 12 | 16.9494 |
| RF-RANGE-CHAIN-F+960 | dense_scrutiny | solved | sensitive | needs_recompute | 52 / 12 | 97.4792 |
| RF-RANGE-CHAIN-LEF-large | sparse_interactive | refused_blocked | not_assessed | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-CHAIN-LEF-large | dense_scrutiny | refused_blocked | not_assessed | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-CHAIN-SIM-a | sparse_interactive | refused_blocked | not_assessed | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-CHAIN-SIM-a | dense_scrutiny | refused_blocked | not_assessed | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-CHAIN-SIM-b | sparse_interactive | solved | sensitive | needs_recompute | 52 / 12 | 16.9494 |
| RF-RANGE-CHAIN-SIM-b | dense_scrutiny | solved | sensitive | needs_recompute | 52 / 12 | 97.4792 |
| RF-RANGE-SKEW-L+240 | sparse_interactive | solved | sensitive | needs_recompute | 16 / 12 | 16932.2 |
| RF-RANGE-SKEW-L+240 | dense_scrutiny | solved | sensitive | needs_recompute | 14 / 14 | 13184.6 |
| RF-RANGE-SKEW-E+960 | sparse_interactive | refused_blocked | unresolved | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-SKEW-E+960 | dense_scrutiny | refused_blocked | unresolved | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-SKEW-F+960 | sparse_interactive | refused_blocked | sensitive | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-SKEW-F+960 | dense_scrutiny | refused_blocked | sensitive | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-SKEW-LEF-large | sparse_interactive | refused_blocked | not_assessed | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-SKEW-LEF-large | dense_scrutiny | refused_blocked | not_assessed | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-SKEW-SIM-a | sparse_interactive | refused_blocked | not_assessed | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-SKEW-SIM-a | dense_scrutiny | refused_blocked | not_assessed | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-SKEW-SIM-b | sparse_interactive | solved | sensitive | needs_recompute | 16 / 12 | 16932.2 |
| RF-RANGE-SKEW-SIM-b | dense_scrutiny | solved | sensitive | needs_recompute | 14 / 14 | 13184.6 |
| RF-RANGE-CONT-L+240 | sparse_interactive | solved | checks_passed | numerically_eligible | 127 / 0 | 6.28969e-07 |
| RF-RANGE-CONT-L+240 | dense_scrutiny | solved | checks_passed | numerically_eligible | 127 / 0 | 7.89256e-07 |
| RF-RANGE-CONT-E+960 | sparse_interactive | solved | checks_passed | numerically_eligible | 127 / 0 | 6.28969e-07 |
| RF-RANGE-CONT-E+960 | dense_scrutiny | solved | checks_passed | numerically_eligible | 127 / 0 | 7.89256e-07 |
| RF-RANGE-CONT-F+960 | sparse_interactive | refused_blocked | checks_passed | numerically_eligible | 0 / 0 | 0.0 |
| RF-RANGE-CONT-F+960 | dense_scrutiny | refused_blocked | checks_passed | numerically_eligible | 0 / 0 | 0.0 |
| RF-RANGE-CONT-LEF-large | sparse_interactive | refused_blocked | not_assessed | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-CONT-LEF-large | dense_scrutiny | refused_blocked | not_assessed | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-CONT-SIM-a | sparse_interactive | refused_blocked | not_assessed | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-CONT-SIM-a | dense_scrutiny | refused_blocked | not_assessed | needs_recompute | 0 / 0 | 0.0 |
| RF-RANGE-CONT-SIM-b | sparse_interactive | solved | checks_passed | numerically_eligible | 127 / 0 | 6.28969e-07 |
| RF-RANGE-CONT-SIM-b | dense_scrutiny | solved | checks_passed | numerically_eligible | 127 / 0 | 7.89256e-07 |
| RF-RANGE-THIN-A | sparse_interactive | solved | checks_passed | numerically_eligible | 22 / 0 | 4.36479e-07 |
| RF-RANGE-THIN-A | dense_scrutiny | solved | checks_passed | numerically_eligible | 22 / 0 | 7.98467e-07 |
| RF-CANCEL-F-G1e80-GnG | sparse_interactive | solved | checks_passed | numerically_eligible | 23 / 9 | 1000000000.0 |
| RF-CANCEL-F-G1e80-GnG | dense_scrutiny | solved | checks_passed | numerically_eligible | 23 / 9 | 1000000000.0 |
| RF-CANCEL-F-G1e80-GGn | sparse_interactive | solved | checks_passed | numerically_eligible | 32 / 0 | 4.32226e-06 |
| RF-CANCEL-F-G1e80-GGn | dense_scrutiny | solved | checks_passed | numerically_eligible | 32 / 0 | 3.66052e-06 |
| RF-CANCEL-F-G1e80-nGG | sparse_interactive | solved | checks_passed | numerically_eligible | 23 / 9 | 1000000000.0 |
| RF-CANCEL-F-G1e80-nGG | dense_scrutiny | solved | checks_passed | numerically_eligible | 23 / 9 | 1000000000.0 |
| RF-CANCEL-F-G1e80-GnG-ORTHO | sparse_interactive | solved | checks_passed | numerically_eligible | 26 / 6 | 1000000000.0 |
| RF-CANCEL-F-G1e80-GnG-ORTHO | dense_scrutiny | solved | checks_passed | numerically_eligible | 26 / 6 | 1000000000.0 |
| RF-CANCEL-F-G1e80-GnG-INPLANE | sparse_interactive | solved | checks_passed | numerically_eligible | 30 / 2 | 999998000.0 |
| RF-CANCEL-F-G1e80-GnG-INPLANE | dense_scrutiny | solved | checks_passed | numerically_eligible | 30 / 2 | 999996000.0 |
| RF-CANCEL-M-G1e80-GnG | sparse_interactive | solved | checks_passed | numerically_eligible | 23 / 9 | 1000000000.0 |
| RF-CANCEL-M-G1e80-GnG | dense_scrutiny | solved | checks_passed | numerically_eligible | 23 / 9 | 1000000000.0 |
| RF-CANCEL-M-G1e80-GGn | sparse_interactive | solved | checks_passed | numerically_eligible | 32 / 0 | 2.99877e-06 |
| RF-CANCEL-M-G1e80-GGn | dense_scrutiny | solved | checks_passed | numerically_eligible | 32 / 0 | 3.32965e-06 |
| RF-CANCEL-M-G1e80-nGG | sparse_interactive | solved | checks_passed | numerically_eligible | 23 / 9 | 1000000000.0 |
| RF-CANCEL-M-G1e80-nGG | dense_scrutiny | solved | checks_passed | numerically_eligible | 23 / 9 | 1000000000.0 |
| RF-CANCEL-M-G1e80-GnG-ORTHO | sparse_interactive | solved | checks_passed | numerically_eligible | 27 / 5 | 1000000000.0 |
| RF-CANCEL-M-G1e80-GnG-ORTHO | dense_scrutiny | solved | checks_passed | numerically_eligible | 27 / 5 | 1000000000.0 |
| RF-CANCEL-M-G1e80-GnG-INPLANE | sparse_interactive | solved | checks_passed | numerically_eligible | 30 / 2 | 999999000.0 |
| RF-CANCEL-M-G1e80-GnG-INPLANE | dense_scrutiny | solved | checks_passed | numerically_eligible | 30 / 2 | 999999000.0 |
| RF-CANCEL-UDL-W1e80 | sparse_interactive | solved | checks_passed | numerically_eligible | 40 / 1 | 2.63281e+81 |
| RF-CANCEL-UDL-W1e80 | dense_scrutiny | solved | checks_passed | numerically_eligible | 40 / 1 | 2.63281e+81 |
| V1-CHECK-L-MIXED | sparse_interactive | solved | sensitive | needs_recompute | 17 / 11 | 1999990000.0 |
| V1-CHECK-L-MIXED | dense_scrutiny | solved | sensitive | needs_recompute | 17 / 11 | 1999950000.0 |
| V1-CHECK-L-ONLY | sparse_interactive | solved | sensitive | needs_recompute | 21 / 7 | 1000000000.0 |
| V1-CHECK-L-ONLY | dense_scrutiny | solved | sensitive | needs_recompute | 21 / 7 | 1000000000.0 |

**`RF-CANCEL-UDL-W1e80` on the typed entry:** th.S1.RZ is published as **1.2184480799204924e+57 rad** against an expected 4.627942515030396503893e-16 rad (ratio 2.63e81), with checks_passed. The residue of the gross fixed-end moments (about 3.3e79 N·m) is published as a response. ROOT has noted the resulting ordering constraint: D2's capture fix must not land before the S11 product slice.

## 7. Realistic-fixture survey (ROOT's addition)

**Scope.** The survey ran every committed preview request, and every committed preview model document (wrapped), under `P/fixtures/**` at `c61a540ea`. That is 51 distinct requests after deduplication by canonical hash, each on the captured entry in both modes.

**Figures.**

- fe = (1/rcond)·(max guarded_ratio + 2^-53), read from the published M03 text. **It is an estimate from published figures, not a bound.**
- The row-wise column counts nonzero rows q with fe·scale/|q| > 1e-9, where scale is the largest |value| of q's class in the case: translation, rotation, support force or moment, member force or moment. The literal form `fe > 1e-9·scale/|q|` is first met at the class-maximum row, where it equals `fe > 1e-9`. The row-wise count is dominated by roundoff-level near-zero rows.
- "Rows by kind" gives per-case counts for D1's floor classification: displacement, component_reaction, reaction_resultant, open_formula_stress_summary, member_action, stress, and the support magnitude rows. `headline` marks the case that holds the max_stress headline. Runs on the exact profile (physics-1 or physics-source-1) are marked `exact route`. Of these, the ordinary pressure cases are the two physics-1 model fixtures and the physics-source `case:ordinary-pressure` cases. None of them publishes reaction_resultant or open_formula_stress_summary rows, so the headline and the reactions there come from the stress and component rows.

**T1 fixtures, not_authorable.** These need the 0.4.0 load-state route (T1, `f3270ea79`); T1's code was not built. The files were listed read-only:

- `fixtures/model_operations/load_reference_authoring_control.json`
- `fixtures/model_operations/load_reference_delete_control.json`
- `fixtures/product_preview/load_reference/{connected,pressure}.request.json`
- `fixtures/product_preview/load_reference_source/{eigen_motion,fields,mixed,n05,n06}.request.json`

**Summary.**

- No Passed committed fixture case would demote under `fe > 1e-9`. The largest Passed fe is 2.85e-12, in the physics-source ordinary-pressure case, at rcond 1.06e-3.
- Every Sensitive case already has fe of about 3e-4 to 1e-3 (rcond about 1e-11). Its published values are either retained-source recoveries (eligible through the receipt) or withheld.
- **The same trigger on the R1 references.** Across the 192 trusted runs:
  - it catches RF-SKEW-T-CANT-OFF-122-r1e-04 (fe about 2e-6);
  - it flags 54 runs that pass (fe 4e-9 to 1.7e-6);
  - it misses only the S11 load-vector cases (fe about 3e-12), which a solve-error estimate cannot see.

| Fixture | Load case | Mode | Outcome | Producer | Quality | Standing | rcond | min pivot | max guarded_ratio (target) | fe estimate | Demote fe>1e-9 | Row-wise rows >1e-9 | Rows by kind |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| fixtures/model_operations/exact_pressure_authoring_model.json | case:closed-pressure | sparse_interactive | solved (exact route) | physics-1 | checks_passed | numerically_eligible | 0.037 | 0.754 | 2.93e-15 (1.85e-13) | 8.21e-14 | False | 0 | component_reaction 6, displacement 14, member_action 25, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 41 |
| fixtures/model_operations/exact_pressure_authoring_model.json | case:six-component-load | sparse_interactive | solved (exact route) | physics-1 | checks_passed | numerically_eligible | 0.037 | 0.754 | 6.96e-15 (4.41e-13) | 1.91e-13 | False | 0 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 21; headline |
| fixtures/model_operations/exact_pressure_authoring_model.json | case:closed-pressure | dense_scrutiny | solved (exact route) | physics-1 | checks_passed | numerically_eligible | 0.037 | 0.251 | 2.94e-15 (1.85e-13) | 8.23e-14 | False | 0 | component_reaction 6, displacement 14, member_action 25, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 41 |
| fixtures/model_operations/exact_pressure_authoring_model.json | case:six-component-load | dense_scrutiny | solved (exact route) | physics-1 | checks_passed | numerically_eligible | 0.037 | 0.251 | 6.31e-15 (3.98e-13) | 1.73e-13 | False | 0 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 21; headline |
| fixtures/model_operations/physics_thermal_ui_model.json | case:closed-pressure | sparse_interactive | solved (exact route) | physics-1 | checks_passed | numerically_eligible | 0.037 | 0.754 | 2.91e-15 (1.85e-13) | 8.15e-14 | False | 0 | component_reaction 6, displacement 14, member_action 25, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 41 |
| fixtures/model_operations/physics_thermal_ui_model.json | case:six-component-load | sparse_interactive | solved (exact route) | physics-1 | checks_passed | numerically_eligible | 0.037 | 0.754 | 6.96e-15 (4.41e-13) | 1.91e-13 | False | 0 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 21; headline |
| fixtures/model_operations/physics_thermal_ui_model.json | case:closed-pressure | dense_scrutiny | solved (exact route) | physics-1 | checks_passed | numerically_eligible | 0.037 | 0.251 | 2.95e-15 (1.85e-13) | 8.27e-14 | False | 0 | component_reaction 6, displacement 14, member_action 25, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 41 |
| fixtures/model_operations/physics_thermal_ui_model.json | case:six-component-load | dense_scrutiny | solved (exact route) | physics-1 | checks_passed | numerically_eligible | 0.037 | 0.251 | 6.31e-15 (3.98e-13) | 1.73e-13 | False | 0 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 21; headline |
| fixtures/model_operations/precision_connected_ui_model.json | case:six-component-load | sparse_interactive | solved | preview-physics-1 | checks_passed | numerically_eligible | 0.037 | 0.754 | 6.92e-15 (4.41e-13) | 1.9e-13 | False | 0 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 21; headline |
| fixtures/model_operations/precision_connected_ui_model.json | case:six-component-load | dense_scrutiny | solved | preview-physics-1 | checks_passed | numerically_eligible | 0.037 | 0.251 | 6.93e-15 (4.41e-13) | 1.9e-13 | False | 0 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 21; headline |
| fixtures/product_preview/invented_dec092_temperature_g_request.json | load:L-DEC092-EXACT | sparse_interactive | solved | preview-physics-1 | checks_passed | numerically_eligible | 0.0612 | 0.716 | 2.9e-15 (1.85e-13) | 4.92e-14 | False | 0 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 21 |
| fixtures/product_preview/invented_dec092_temperature_g_request.json | load:L-DEC092-INTERPOLATED | sparse_interactive | solved | preview-physics-1 | checks_passed | numerically_eligible | 0.0612 | 0.707 | 2.89e-15 (1.85e-13) | 4.91e-14 | False | 0 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 21 |
| fixtures/product_preview/invented_dec092_temperature_g_request.json | load:L-DEC092-BASE | sparse_interactive | solved | preview-physics-1 | checks_passed | numerically_eligible | 0.037 | 0.754 | 2.9e-15 (1.85e-13) | 8.13e-14 | False | 0 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 21; headline |
| fixtures/product_preview/invented_dec092_temperature_g_request.json | load:L-DEC092-EXACT | dense_scrutiny | solved | preview-physics-1 | checks_passed | numerically_eligible | 0.0612 | 0.955 | 2.95e-15 (1.85e-13) | 5e-14 | False | 0 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 21 |
| fixtures/product_preview/invented_dec092_temperature_g_request.json | load:L-DEC092-INTERPOLATED | dense_scrutiny | solved | preview-physics-1 | checks_passed | numerically_eligible | 0.0612 | 0.942 | 2.95e-15 (1.85e-13) | 5e-14 | False | 0 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 21 |
| fixtures/product_preview/invented_dec092_temperature_g_request.json | load:L-DEC092-BASE | dense_scrutiny | solved | preview-physics-1 | checks_passed | numerically_eligible | 0.037 | 0.251 | 2.94e-15 (1.85e-13) | 8.22e-14 | False | 0 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 21; headline |
| fixtures/product_preview/numerical_sensitive_torsion_model.json | case | sparse_interactive | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.16e-14 (7.39e-13) | 0.0011 | True | 8 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/numerical_sensitive_torsion_model.json | case | dense_scrutiny | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.02e-14 (6.54e-13) | 0.000977 | True | 8 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/source_blocks/multicase-dense_scrutiny.request.json | case | sparse_interactive | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.16e-14 (7.39e-13) | 0.0011 | True | 8 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20 |
| fixtures/product_preview/source_blocks/multicase-dense_scrutiny.request.json | case:signed-companion | sparse_interactive | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.16e-14 (7.39e-13) | 0.0011 | True | 15 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/source_blocks/multicase-dense_scrutiny.request.json | case | dense_scrutiny | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.02e-14 (6.54e-13) | 0.000977 | True | 8 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20 |
| fixtures/product_preview/source_blocks/multicase-dense_scrutiny.request.json | case:signed-companion | dense_scrutiny | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.02e-14 (6.54e-13) | 0.000977 | True | 15 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/source_blocks/n05-dense_scrutiny.request.json | case | sparse_interactive | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.16e-14 (7.39e-13) | 0.0011 | True | 8 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/source_blocks/n05-dense_scrutiny.request.json | case | dense_scrutiny | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.02e-14 (6.54e-13) | 0.000977 | True | 8 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/source_blocks/n06-dense_scrutiny.request.json | case | sparse_interactive | solved | source-blocks-1 | unresolved | numerically_eligible | - | - | - | - | - | - | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/source_blocks/n06-dense_scrutiny.request.json | case | dense_scrutiny | solved | source-blocks-1 | unresolved | numerically_eligible | - | - | - | - | - | - | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/source_blocks/ui/multicase-dense_scrutiny.request.json | case | sparse_interactive | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.16e-14 (7.39e-13) | 0.0011 | True | 8 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20 |
| fixtures/product_preview/source_blocks/ui/multicase-dense_scrutiny.request.json | case:signed-companion | sparse_interactive | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.16e-14 (7.39e-13) | 0.0011 | True | 15 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/source_blocks/ui/multicase-dense_scrutiny.request.json | case | dense_scrutiny | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.02e-14 (6.54e-13) | 0.000977 | True | 8 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20 |
| fixtures/product_preview/source_blocks/ui/multicase-dense_scrutiny.request.json | case:signed-companion | dense_scrutiny | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.02e-14 (6.54e-13) | 0.000977 | True | 15 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/source_blocks/ui/n05-dense_scrutiny.request.json | case | sparse_interactive | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.16e-14 (7.39e-13) | 0.0011 | True | 8 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/source_blocks/ui/n05-dense_scrutiny.request.json | case | dense_scrutiny | solved | source-blocks-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.02e-14 (6.54e-13) | 0.000977 | True | 8 | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/source_blocks/ui/n06-dense_scrutiny.request.json | case | sparse_interactive | solved | source-blocks-1 | unresolved | numerically_eligible | - | - | - | - | - | - | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/source_blocks/ui/n06-dense_scrutiny.request.json | case | dense_scrutiny | solved | source-blocks-1 | unresolved | numerically_eligible | - | - | - | - | - | - | component_reaction 12, displacement 14, member_action 30, open_formula_stress_summary 1, reaction_resultant 2, stress_other 20; headline |
| fixtures/product_preview/physics_source/fields.request.json | case | sparse_interactive | solved (exact route) | physics-source-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 9.58e-15 (6.11e-13) | 0.000916 | True | 43 | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21; headline |
| fixtures/product_preview/physics_source/fields.request.json | case | dense_scrutiny | solved (exact route) | physics-source-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.09e-14 (6.96e-13) | 0.00104 | True | 43 | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21; headline |
| fixtures/product_preview/physics_source/mixed.request.json | case | sparse_interactive | solved (exact route) | physics-source-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 9.58e-15 (6.11e-13) | 0.000916 | True | 8 | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21 |
| fixtures/product_preview/physics_source/mixed.request.json | case:ordinary-pressure | sparse_interactive | solved (exact route) | physics-source-1 | checks_passed | numerically_eligible | 0.00106 | 0.0064 | 2.89e-15 (1.85e-13) | 2.85e-12 | False | 0 | component_reaction 12, displacement 14, member_action 25, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 41; headline |
| fixtures/product_preview/physics_source/mixed.request.json | case | dense_scrutiny | solved (exact route) | physics-source-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.09e-14 (6.96e-13) | 0.00104 | True | 8 | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21 |
| fixtures/product_preview/physics_source/mixed.request.json | case:ordinary-pressure | dense_scrutiny | solved (exact route) | physics-source-1 | checks_passed | numerically_eligible | 0.00106 | 0.00637 | 2.89e-15 (1.85e-13) | 2.85e-12 | False | 0 | component_reaction 12, displacement 14, member_action 25, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 41; headline |
| fixtures/product_preview/physics_source/mixed_units.request.json | case | sparse_interactive | solved (exact route) | physics-source-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 9.58e-15 (6.11e-13) | 0.000916 | True | 8 | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21 |
| fixtures/product_preview/physics_source/mixed_units.request.json | case:ordinary-pressure | sparse_interactive | solved (exact route) | physics-source-1 | checks_passed | numerically_eligible | 0.00106 | 0.0064 | 2.89e-15 (1.85e-13) | 2.85e-12 | False | 0 | component_reaction 12, displacement 14, member_action 25, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 41; headline |
| fixtures/product_preview/physics_source/mixed_units.request.json | case | dense_scrutiny | solved (exact route) | physics-source-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.09e-14 (6.96e-13) | 0.00104 | True | 8 | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21 |
| fixtures/product_preview/physics_source/mixed_units.request.json | case:ordinary-pressure | dense_scrutiny | solved (exact route) | physics-source-1 | checks_passed | numerically_eligible | 0.00106 | 0.00637 | 2.89e-15 (1.85e-13) | 2.85e-12 | False | 0 | component_reaction 12, displacement 14, member_action 25, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 41; headline |
| fixtures/product_preview/physics_source/n05.request.json | case | sparse_interactive | solved (exact route) | physics-source-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 9.58e-15 (6.11e-13) | 0.000916 | True | 8 | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21; headline |
| fixtures/product_preview/physics_source/n05.request.json | case | dense_scrutiny | solved (exact route) | physics-source-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.09e-14 (6.96e-13) | 0.00104 | True | 8 | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21; headline |
| fixtures/product_preview/physics_source/n05_unicode.request.json | case:温度:α | sparse_interactive | solved (exact route) | physics-source-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 9.58e-15 (6.11e-13) | 0.000916 | True | 8 | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21; headline |
| fixtures/product_preview/physics_source/n05_unicode.request.json | case:温度:α | dense_scrutiny | solved (exact route) | physics-source-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.09e-14 (6.96e-13) | 0.00104 | True | 8 | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21; headline |
| fixtures/product_preview/physics_source/n05_units.request.json | case | sparse_interactive | solved (exact route) | physics-source-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 9.58e-15 (6.11e-13) | 0.000916 | True | 8 | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21; headline |
| fixtures/product_preview/physics_source/n05_units.request.json | case | dense_scrutiny | solved (exact route) | physics-source-1 | sensitive | numerically_eligible | 1.06e-11 | 9.54e-11 | 1.09e-14 (6.96e-13) | 0.00104 | True | 8 | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21; headline |
| fixtures/product_preview/physics_source/n06.request.json | case | sparse_interactive | solved (exact route) | physics-source-1 | unresolved | numerically_eligible | - | - | - | - | - | - | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21; headline |
| fixtures/product_preview/physics_source/n06.request.json | case | dense_scrutiny | solved (exact route) | physics-source-1 | unresolved | numerically_eligible | - | - | - | - | - | - | component_reaction 12, displacement 14, member_action 30, support_reaction_force_magnitude_v2 2, support_reaction_moment_magnitude_v2 2, stress_other 21; headline |
| fixtures/results/invented/result_export_v0_2.json#producer_cases[1].model | load:L-100 | sparse_interactive | solved | preview-physics-1 | checks_passed | numerically_eligible | 0.0155 | 0.337 | 1.3e-14 (8.24e-13) | 8.43e-13 | False | 8 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 20 |
| fixtures/results/invented/result_export_v0_2.json#producer_cases[1].model | load:L-100 | dense_scrutiny | solved | preview-physics-1 | checks_passed | numerically_eligible | 0.0155 | 0.123 | 1.29e-14 (8.24e-13) | 8.4e-13 | False | 8 | component_reaction 6, displacement 14, member_action 30, support_reaction_force_magnitude_v2 1, support_reaction_moment_magnitude_v2 1, stress_other 20 |
| fixtures/results/invented/result_export_v0_2.json#producer_cases[5].model | load:G-TIP | sparse_interactive | solved | preview-physics-1 | unresolved | needs_recompute | 0.111 | 0.659 | 5.78e-15 (3.69e-13) | 5.3e-14 | False | 0 | component_reaction 30, displacement 21, member_action 60, support_reaction_force_magnitude_v2 5, support_reaction_moment_magnitude_v2 5, stress_other 42; headline |
| fixtures/results/invented/result_export_v0_2.json#producer_cases[5].model | load:G-TIP | dense_scrutiny | solved | preview-physics-1 | unresolved | needs_recompute | 0.111 | 1.32 | 5.78e-15 (3.69e-13) | 5.3e-14 | False | 0 | component_reaction 30, displacement 21, member_action 60, support_reaction_force_magnitude_v2 5, support_reaction_moment_magnitude_v2 5, stress_other 42; headline |
| fixtures/results/invented/result_export_v0_2.json#producer_cases[6].model | load:G-TIP | sparse_interactive | solved | preview-physics-1 | unresolved | needs_recompute | 0.0139 | 0.659 | 8.23e-15 (5.26e-13) | 6.01e-13 | False | 2 | component_reaction 66, displacement 42, member_action 150, support_reaction_force_magnitude_v2 11, support_reaction_moment_magnitude_v2 11, stress_other 105; headline |
| fixtures/results/invented/result_export_v0_2.json#producer_cases[6].model | load:G-TIP | dense_scrutiny | solved | preview-physics-1 | unresolved | needs_recompute | 0.0139 | 0.528 | 8.29e-15 (5.26e-13) | 6.05e-13 | False | 3 | component_reaction 66, displacement 42, member_action 150, support_reaction_force_magnitude_v2 11, support_reaction_moment_magnitude_v2 11, stress_other 105; headline |

Runs that did not solve (both modes each, grouped by blocking code):

- `LOAD_COMBINATION_TERMS_EMPTY`: 5 fixtures (case_01_accept_set_field_material_elastic_modulus.json#base_model; case_39_block_unit_metadata_missing.json#base_model; case_44_block_quantity_object_missing.json#base_model; case_58_accept_create_combination_subtraction.json#base_model; case_67_accept_set_field_component_expansion_joint_stiffness.json#base_model)
- `PIPE_ORIENTATION_INPUT_MISSING`: 6 fixtures (case_05_accept_update_load_combination_term_factor.json#base_model; case_13_accept_delete_combination_term.json#base_model; case_32_block_term_factor_dimension_mismatch.json#base_model; case_50_accept_delete_primitive_load.json#base_model; case_55_block_delete_pipe_run_referenced.json#base_model; case_57_block_delete_node_referenced.json#base_model)
- `LOAD_INPUT_MISSING, NODE_INPUT_MISSING, PIPE_INPUT_MISSING`: 6 fixtures (case_45_accept_create_support.json#base_model; case_46_accept_create_material.json#base_model; case_47_accept_create_section.json#base_model; case_48_accept_delete_support.json#base_model; case_51_accept_delete_combination.json#base_model; r2_from_blank_rehearsal.json#blank_model)
- `IMPOSED_DISPLACEMENT_REQUIRES_LOAD_STATE_MODEL`: 1 fixtures (case_49_block_delete_support_referenced.json#base_model)
- `NODE_INPUT_MISSING, PIPE_INPUT_MISSING`: 2 fixtures (case_52_accept_delete_load_case.json#base_model; case_53_block_delete_load_case_referenced.json#base_model)
- `LOAD_INPUT_MISSING`: 1 fixtures (case_54_accept_delete_pipe_run.json#base_model)
- `LOAD_INPUT_MISSING, PIPE_INPUT_MISSING`: 1 fixtures (case_56_accept_delete_node.json#base_model)
- `Err: request: missing field `project``: 2 fixtures (case_76_accept_set_field_material_temperature_point_shear_modulus.json#base_model; case_77_block_set_field_material_temperature_point_shear_modulus_missing_point.json#base_model)
- `PRESSURE_MODEL_REAUTHOR_REQUIRED`: 3 fixtures (invented_preview_model.json; result_export_v0_2.json#producer_cases[0].model; result_export_v0_2.json#producer_cases[2].model)
- `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED`: 1 fixtures (dense_scrutiny.request.json)
- `PRESSURE_MODEL_REAUTHOR_REQUIRED, PREVIEW_DOCUMENT_KIND_INVALID`: 1 fixtures (result_export_v0_2.json#producer_cases[3].model)
- `NONLINEAR_SUPPORT_NONCONVERGENCE, SOLVER_SYSTEM_BLOCKED`: 1 fixtures (result_export_v0_2.json#producer_cases[4].model)

## 8. What was not run, and limits

- **RF-LARGE.** The sizes and modes that ended in memory_refused or timeout are listed in §5. The n = 10000 cases could not run under the 6 GiB limit, and no larger limit was tried on the shared host.
- **Not authorable.** The 30 cases in §2 were not run. That includes 18 skew and 4 invariance cases with directional springs, 2 cases in N·mm units, and 6 matrix-level fixture cases. None was approximated.
- **V1 check L.** It is authored from V1's `pin_case` description, with expected values from `references.py`'s own tree integration (supplementary; not frozen). It is refused at capture, and is Sensitive on the typed entry.
- **Not published.** `Mb.mid`, `tw` and `ext` are not product results and were not compared, apart from being counted as not_published. `tw` and `ext` could be derived from published nodal values, but that derivation is itself the NC-SUBTRACT-ROUNDED defect model, so it was not used.
- **Probe A** is P1's own textbook reference, not a frozen one.
- **Not done.** No product, test, fixture, reference or work-graph file was changed. No Git write was made. Nothing outside `T3/DETECTION/**` was written in the repository. T1's worktree, branch and code were not touched. Build output was deleted from `<t3-target>` at the end (P1's own `t3_p1_probe` and `t3_p1_survey` artifacts only).

## 9. Records in this folder

- `results.json`: primary against revision 2. It holds per case, mode and quantity:
  - `[key, class, expected, observed, scale, ratio, status, governed_by, flags]`;
  - identity, quality, recovery, standing, and the M03 integrity text. For large models, the pivot and residual lists are elided above 20,000 characters, keeping rcond and the summary figures;
  - negative-control outcomes, the typed secondary observations, and fe estimates;
  - the header with both reference hashes and the revision cross-check;
  - the realistic-fixture survey.
- `probe/Cargo.toml.txt`, `probe/main.rs.txt`, `survey_probe/Cargo.toml.txt`, `survey_probe/main.rs.txt`: the Rust probes, with the export path as `<export>`.
- `scripts/gen.py.txt`, `run.py.txt`, `compare.py.txt`, `survey.py.txt`, `tables.py.txt`: the generator, driver, comparator, survey and tables.
- `logs/run.log`, `logs/survey.log`: console logs of the reported run.
- `toolchain.txt`, `SHA256SUMS`.

## 10. Addendum: follow-up requested by the manager after `516c34c1a`

This is measurement only, under the same rules as the main run: the frozen references `c0f14201c`, fresh processes, `RLIMIT_AS` 6 GiB, no raised limits and no product writes. The probe was rebuilt from the recorded `probe/main.rs.txt`, and the binary's sha256 is unchanged (`61cbb76f…`). The script is `scripts/run_typed.py.txt`, and its log is `logs/run_typed.log`.

**1. Typed entry on the captured S11-class cases.** I ran `run_linear_static_preview_with_mode` in both modes on the 15 cases of the captured `s11_class` list, plus `RF-SKEW-T-CANT-OFF-122-r1e-04`. The results are recorded in `results.json` as `modes.<mode>.typed_on_captured` and `exceptions.typed_on_captured`. The earlier `typed_secondary` list is unchanged.

- All 32 runs publish preview-physics-1 with checks_passed. Main's standing function, given a P1-supplied invocation, returns numerically_eligible. The §6 caveat about that standing on the typed route still applies.
- In every run the published result values, numerical_quality and producer are **bit-identical** to the captured entry's.
- The breaching triples are therefore the same as on the captured entry:

| Class | Rows (case, mode, quantity) | Distinct (case, quantity) | Cases |
|---|---:|---:|---:|
| S11 class | 200 | 100 | 15 |
| S11 class, frozen-reference cases only | 176 | 88 | 13 |
| Other class (RF-SKEW-T-CANT-OFF-122-r1e-04) | 6 | 4 | 1 |

**2. The interim counts of 106 triples in 13 cases (captured) and 168 in 22 cases (typed).** P1 did not produce these counts: none of my messages or records contains them. I cannot reproduce them from any P1 run or with any counting unit I tried. Every count P1 can produce from its runs, frozen revision 2, re-run authoring:

| Selection | Rows (case, mode, quantity) | Distinct (case, quantity) | Cases |
|---|---:|---:|---:|
| Captured, Passed or eligible, S11 class | 200 | 100 | 15 |
| Captured, same, frozen-reference cases only | 176 | 88 | 13 |
| Captured, every Passed or eligible breach | 206 | 104 | 16 |
| Typed (capture-refused cases), Passed or eligible | 104 | 52 | 9 |
| Typed (capture-refused cases), any mismatch, including Sensitive | 288 | 146 | 17 |
| Typed, any mismatch, capped at 12 per case and mode (as the list was before I widened it) | 284 | n/a | 17 |

- The 13 cases in the interim captured count do match the frozen-reference case count (13). Its triple count of 106 matches no unit here.
- The typed interim count of 22 cases exceeds every typed selection P1 has. Typed runs exist for only 32 cases, of which 23 solved and 17 have any mismatch.
- The pinned list should be re-pinned from `exceptions` in this file. P1's counting units are defined there: rows = (case, mode, quantity), pairs = distinct (case, quantity).

**3. Frozen references against P1's probes.** `S11-PROBE-A-G1e5`, `-G1e6`, `-G1e7` and `-G1e8` are **P1's own probes**, not frozen-reference cases. Their reference is P1's textbook closed form (§3.2). The same holds for `V1-CHECK-L-*`, whose values are P1's application of `references.py`'s routines, and for the FX-* fixture cases, which rest on `fixtures.json`, not on R1. With the probe-A cases excluded, the captured S11-class breaches are **88 (case, quantity) pairs in 13 cases** (176 rows counting mode), which confirms the manager's figure. `results.json` carries these counts as `exceptions.<entry>.s11_class_frozen_reference_only_counts`.
