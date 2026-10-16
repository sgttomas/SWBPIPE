# M3-T: independent evaluation of the K-D5 certified gate with the Timoshenko straight member

This directory holds the judged-set generator, the exact reference engine, the
reconstructed M3 comparison and a small frozen subset for M3-T, part 5 of section 4 of
[section-bases-and-timoshenko.md](../../../execution/PKG-05/DEL-05-03/Design/section-bases-and-timoshenko.md).
The acceptance is the K-D5 M3 criterion of
[kd5-member-relative-bound.md](../../../execution/PKG-04/DEL-04-01/Design/kd5-member-relative-bound.md)
(section 4.3): on Passed cases, 0 judged member, section, support or stress breaches, and 0 strict
`max_station_checks` coverage failures. Standard claim fence applies (F-PIP-2; claims taxonomy per
DEC-081). Every model is invented.

**Independence.** The engine, generator, comparison and census are standard-library Python. They import
and read no product code. The public request and response format came from the schemas, committed
request fixtures, the surviving harness and the product's published rows on probe models. Expected
values come only from the engine's own arithmetic. The scratch runner links the product crate by path
and calls only `run_linear_static_preview_with_mode` (typed) and
`run_linear_static_preview_value_with_mode` (captured).

## Files

| File | Role |
|---|---|
| `kd5t_engine.py` | Exact reference engine: a public request in, every expected published row out (110-digit `decimal`; rational element controls in `fractions`). `--check` runs its self-check. |
| `kd5t_generate.py` | Judged-set generator: `full <dir>` writes the set and controls, `freeze` writes the frozen subset, `split` makes runner inputs, `check` verifies everything. |
| `kd5t_compare.py` | Reconstructed M3 judging and coverage, aware of the section basis. `--selftest <set>` exercises its rules on synthetic runs. |
| `kd5t_report.py` | Aggregates the comparison into the M3 tables, optionally by family, theory, placement or mill, and by phi decade. |
| `kd5t_census.py` | Premise census: the Hager ratio of the K-D5 gate against the exact inverse norm. |
| `frozen/kd5t_frozen_subset.jsonl` | 60 models (30 designs in both theories), each line `{tag, meta, ref, request}`, with exact expected rows. |
| `SHA256SUMS` | Digests of the files above, except this README. |

## The reference engine

- **Element.** A straight two-node 3D frame. The local frame is x = chord, y = `y_reference` projected
  normal to x, z = x × y. Axial EA/L and torsion GJ/L use the nominal section, with G = E/(2(1+ν)).
  The two bending planes follow the existing frame convention (v = u_y, θ = r_z; v = u_z, θ = −r_y).
  Each is Euler–Bernoulli (φ = 0) or the SHEAR_REFERENCE element at `f2211ecef6` (CONTRACT §3):
  φ = 12EI/(K_s L²), K_s = κ_E·G·A_n, with κ_E in the cancellation-free form of Design §3.1,
  formed exactly from the authored binary64 OD, wall and ν on the nominal section.
- **D-5 bases (Design §1).** The nominal section has wall t. The reduced section has
  t_r = fl(fl(t − c_m) − c_a), the Design's fixed binary64 order. The pressure bore is
  r_i,r = D/2 − t_r and Ai = π r_i,r². A, I, J = 2I and Z = I/r_o of each basis are exact.
- **Loads.**
  - Nodal forces and moments.
  - Full-span uniform loads, with the consistent vector [wL/2, wL²/12, wL/2, −wL²/12] for both
    theories (SHEAR_REFERENCE §4).
  - Axial free strain, from α·ΔT or an explicit load-state strain, as the end pair ∓E·A_n·ε.
  - Factored load sources, entered as the exact product of their binary64 operands.
  - Pressure, on regions of collinear straight members only. Caps of p·Ai act outward at
    `transfers_to_wall` terminals. Kink forces of p·Ai·Σ(outward tangents) act at interior region
    nodes. The Poisson free strain is −2ν·p·Ai/(E·A_n). This is the wall formulation of T4-I8 and
    T4-U4a, with Ai on the pressure bore and A_n the stiffness area (Design §1).
- **Supports.** Anchors may restrain any set of DOFs, with prescribed values from a load-reference
  `boundary_motion`. Grounded linear springs act on one DOF.
- **Recovery.**
  - End rows are the node-on-element actions f = K·d − feq.
  - Station rows are the j-side section action at 0, .25, .5, .75 and 1, by statics from the i-end
    action plus the load integral.
  - Reactions are support-on-pipe: Σ f_e − P at anchors and −k·u at springs.
  - Stresses are formed on each basis:
    - σ = N/A_b, M_y/Z_b and M_z/Z_b, and τ = T·r_o/J_b;
    - the membrane stress N_w/A_b;
    - Lamé radial −p and 0, with outer hoop 2p·π·r_i,b²/A_b and inner hoop that plus p;
    - the continuous maximum over the member of |N_w/A_b| + hypot(M_y, M_z)/Z_b.
  - On pressurized members, N_w is the section force, S = N_w − p·Ai, and the endpoint wall actions
    are node-on-element.
- **Units, signs and kinds.** These are the product's published ones: mm, rad, N, N·m, MPa for
  element-local stresses and Pa for `pipe_*` stresses. They were established from its sign-convention
  texts and checked on probe models (EB, zero mill tolerance) to about 1e-16.
- **Self-check (`--check`, 56 checks).**
  - SHEAR_REFERENCE S-A rational controls: tip force 5/12 and 1/2, tip couple, full uniform load and
    root actions.
  - S-E interior spring, rational for both theories.
  - S-E physical companion through the 3D engine, against the 12-digit decimals.
  - S-C κ limits, the cancellation-free form and Cowper equality at ν = 0.
  - S-B κ_E = 16609827/33068756, κ_C, and the L = 1, 2 and 10 m tip deflections.
  - Six rigid modes, the pure-curvature patch, cut(L) = f_j, and global force balance on every solve.
- **Not covered.**
  - Generated self-weight. No public exact-route request field for densities could be established;
    the exact route applies "structural line loads as entered" (results.v0.3 limitations). Self-weight
    and contents are therefore authored intensities w = g(ρ_s·A_n + ρ_c·Ai) on the D-5 bases. The
    product applies the authored value, so the D-5 mass basis itself is not exercised here.
  - Partial-span loads, which cannot reach the exact route.
  - Arcs, connectors and nonlinear supports.

## The judged set (`full`)

- **Size.** 824 models: 412 Euler–Bernoulli and their 412 Timoshenko twins.
  - 140 of them (70 + 70) have nonzero mill tolerance (0.125·t rounded) and allowance (0.5 or
    1.5 mm).
  - Placements: 318 at the origin, 212 at a UTM-scale offset (4.3e5, 7.3e6, 218 m) and 294 skewed at
    (5e6, 7.3e6, 1.2e6 m).
- **Families.**

  | Family | Models |
  |---|---|
  | Cantilevers | 164 |
  | Fixed-fixed runs | 120 |
  | L-lines with sharp corners | 104 |
  | U, Z and 3D lines | 76 |
  | Skew multi-span runs on grounded springs | 76 |
  | φ = 2 cluster | 64 |
  | Pressure regions (collinear chains) | 88 |
  | Near the Sensitive boundary | 132 |

  Near the Sensitive boundary means soft torsion or translation springs and short members, tuned so
  that the gate's r̂ straddles 2^-26.
- **Load content.** 216 models carry springs, 86 prescribed motions (load-reference state, factor 1.1
  on some), 384 self-weight (with contents on the pressure bore for the larger sections), 240 thermal
  strain and 88 pressure.
- **Exact collinearity of skewed pressure chains.** These sit on a dyadic grid along (2, 3, 6)/7 at a
  dyadic offset, so they are exactly collinear in binary64. The product's collinearity guard is
  64·ε·span.
- **φ (Timoshenko twins, 1,235 elements).** From 4.0e-6 to 1.1e7. 44 elements lie within
  |φ − 2| ≤ 2.3e-15 at the origin (±3 ulps of the φ = 2 length), or within 2e-9 at the offset. The
  exact φ and φ − 2 are in each line's `ref.phi` and `ref.phi_minus_2`.
- **Controls (`controls.jsonl`).** For every Euler–Bernoulli zero-mill model, the reference with E
  scaled by 1 + 2^-20 (a perturbed stiffness).

## The comparison (`kd5t_compare.py`)

The surviving `compare_current.py` (sha256 `4b21fa99…`, T3 scratch `pr_kd5_eval_current/`) imports
RV30's `compare.py` and I41's `ref_ext.py`, which are lost. Its own logic is kept. The lost pieces
are rebuilt as follows; the module docstring cites each one.

| Piece | Rule | Source |
|---|---|---|
| CRIT | RD(1e-9), the largest binary64 ≤ 1e-9; a breach is Σ(v − t)² > CRIT²·Σv² | K-D5 §3.3 ("exceeds(B, q) is today's exact B > RD(1e-9)·q"); compare_current's `breach` |
| FLOOR, guard | 2^-34 and 2^-988; a group is judged iff S ≥ 2^-988 and max(q², t²)·conv² ≥ FLOOR²·S² | K-D5 §3.3 predicate; compare_current's `judged` |
| `group_of` | Per location: \|N\|, hypot(V_y, V_z), \|T\|, hypot(M_y, M_z). Pipe-wall kinds are their own groups. Stress groups are `s_axial`, `s_bending`, `s_torsion` and membrane, each keyed by basis. Supports: force vector, moment vector and the two magnitudes. Displacement, rotation and displacement magnitude | K-D5 §3.2; compare_current's `ACT` and `conv`; the analysis scripts' group names |
| Scales | SF = max(force, moment/L_b), SM = max(moment, L_b·force), L_b the node bounding-box diagonal. Stress groups use SF or SM; displacement and rotation groups use their own q | K-D5 §3.3 (coupled S*); compare_current |
| `SKIP` | The solver-mode record. The maximum is checked by coverage. Lamé rows form an extension group outside the criterion | compare_current |
| Coverage | U = evidence `value_upper_pa`, of the same `section_basis` from U6. Every station sample of that pipe and basis must satisfy (U − \|axial\|)² ≥ My² + Mz² exactly. The axial sample is the wall membrane stress where pressurized | K-D5 §3.2 and §4 ("exact-pressure route uses wall axial stress"); compare_current's `circular_public_sample_covered`, which ignored membrane: this is the stricter form |
| Bases | A row with `; section_basis=X` is judged on basis X. A row without it uses the reduced section, which is today's single section t − c_m whenever today's producer admits the document | Design §§2.1, 2.5 |

Reported apart from the criterion:
- Displacement and rotation groups, with their ratio to the coupled body scale (K-D5 §4.3, worst
  0.35).
- Lamé groups.
- Whether each maximum's enclosure contains the exact continuous maximum.
- Unmapped product rows and unpublished reference rows.
- Typed/captured disagreements: integrity codes or breach counts, as in `analyze_b4.py`, plus
  published-row identity.

## Premise census (`kd5t_census.py`)

The gate's r̂, its scale exponents D and the DOF map are read from the NUMERICAL_INTEGRITY message.
The Hager ratio is ‖K̃_s⁻¹‖₁/est = κ₁(D·K·D)·r̂, formed from the engine's exact free-DOF stiffness.
diag(D·K·D) lies in [1, 4), which confirms the mapping. A1 is the claim ratio ≤ 2^10.

The product does not expose ε_E or ‖K̃_s‖₁. ε_E/‖K̃_s‖₁ cannot be measured from outside: it needs the
product's own binary64 element contributions in assembly order.

## Runner (scratch, untracked)

The runner is `Cargo.toml`, `src/main.rs` and `build.sh`. It was copied from the T3 scratch
`b4-finish/cand` with the `retained_direct` arm dropped.

`build.sh` symlinks the Git worktree that holds `$KD5T_PP` (default: this worktree's
`core/product_physics`). It builds through `t3_cargo.sh` with `CARGO_TARGET_DIR` under the T3
targets tree. Inputs are lines `{tag, request}`; `KD5_IN`, `KD5_OUT`, `KD5_ENTRIES` and `KD5_MODES`
control a run. Each output line carries the rows with metadata, the diagnostics (the K-D5 standing),
`numerical_quality`, `summary` and `contract_evidence`.

## Reproduce

```
python3 -I -B kd5t_engine.py --check
python3 -I -B kd5t_generate.py check          # self-checks, frozen bytes, compare self-test, SHA256SUMS
python3 -I -B kd5t_generate.py full <dir> 8
python3 -I -B kd5t_generate.py split <dir>/set.jsonl <runs>/chunks hv 40     # or: pending, all
# run the runner on each chunk, then
python3 -I -B kd5t_compare.py <dir>/set.jsonl <runs.jsonl> <eval.jsonl> --ref own        # judged
python3 -I -B kd5t_compare.py ... --ref twin                                             # theory swapped
python3 -I -B kd5t_compare.py ... --ref control:<dir>/controls.jsonl:pertE                 # perturbed E
python3 -I -B kd5t_report.py <eval.jsonl>... --by family [--phi-table <dir>/set.jsonl]
python3 -I -B kd5t_census.py <dir>/set.jsonl <runs.jsonl>... --out <census.jsonl>
```

`--write`-style regeneration (`freeze`) is for an intended change of the references, never to absorb
a product result.

## Harness validation on today's producer (main behaviour, worktree base `b3f4da7661`)

The 342 Euler–Bernoulli models with zero mill tolerance and allowance were run: 1,368 case-modes over
both modes and both entries.
- 1,246 Passed, 96 Sensitive at the gate and 26 Sensitive by D5C-1.
- 125,988 judged groups.
- 0 judged member, section, support or stress breaches on Passed cases. The worst err/c is 2.2e-6.
- 0 strict coverage failures over 4,112 maxima.
- 0 unmapped or unpublished rows and 0 typed/captured disagreements.
- 180 displacement and rotation noise groups, all within 1e-9 of the coupled body scale (worst
  0.30).
- Hager ratio at most 1.78.

Sensitivity controls on the same runs:
- Against the Timoshenko twin's reference, 674 of the 1,246 Passed case-modes have judged member
  breaches, and every Passed case-mode breaches except axial-only pressure models, whose two
  references coincide.
- Against the perturbed-E reference, 594 have judged member breaches.

The Timoshenko and nonzero-allowance models are refused today, by name
(`EXACT_PRESSURE_SHEAR_MODEL_UNSUPPORTED`, `EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED`). They are
generated, referenced and frozen, ready for the T4-U6/U8 producer.
