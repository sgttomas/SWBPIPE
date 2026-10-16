# M3-T: independent evaluation of the K-D5 certified gate with the Timoshenko straight member

This directory holds the judged-set generator, the exact reference engine, the
reconstructed M3 comparison with its verdict, the premise-census tooling and a small frozen
subset for M3-T, part 5 of section 4 of
[section-bases-and-timoshenko.md](../../../execution/PKG-05/DEL-05-03/Design/section-bases-and-timoshenko.md).
The acceptance is the K-D5 M3 criterion of
[kd5-member-relative-bound.md](../../../execution/PKG-04/DEL-04-01/Design/kd5-member-relative-bound.md)
(section 4.3). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Every model is
invented.

**Independence.** The engine, generator, comparison, census and re-authoring tool are standard-library
Python. They import and read no product code. The public request and response format came from the
schemas, committed request fixtures, the surviving harness and the product's published rows on probe
models. Expected values come only from the engine's own arithmetic. The scratch runner links the
product crate by path and calls only `run_linear_static_preview_with_mode` (typed) and
`run_linear_static_preview_value_with_mode` (captured).

## Files

| File | Role |
|---|---|
| `kd5t_engine.py` | Exact reference engine: a public request in, every expected published row out (110-digit `decimal`; rational element controls in `fractions`). `--check` runs its self-check (58 checks). |
| `kd5t_generate.py` | Judged-set generator: `full <dir>` writes the set and controls, `freeze` writes the frozen subset, `split` makes runner inputs, `check` verifies everything. |
| `kd5t_compare.py` | Reconstructed M3 judging and coverage and the M3 verdict (clauses C0–C8), aware of the section basis and of the producer (`--producer u6\|pre-u6`). `--selftest <set>` exercises every rule and clause. |
| `kd5t_report.py` | Aggregates the comparison into the M3 tables and the verdict, by family, theory, placement or section variant, and by phi decade. |
| `kd5t_census.py` | Premise census: the Hager ratio from published gate values (`hager`), the probe-dump consumer (`probe`), and standings of the re-authored corpora (`standings`). |
| `kd5t_reauthor.py` | Re-authors the surviving RV30, realistic, RV29 and W1 corpora as 0.5.0 exact documents with either shear model. |
| `frozen/kd5t_frozen_subset.jsonl` | 64 models (32 designs in both theories), each line `{tag, meta, ref, request}`, with exact expected rows. |
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
- **Self-check (`--check`, 58 checks).**
  - SHEAR_REFERENCE S-A rational controls: tip force 5/12 and 1/2, tip couple, full uniform load and
    root actions.
  - S-E interior spring, rational for both theories.
  - S-E physical companion through the 3D engine, against the 12-digit decimals.
  - S-C κ limits, the cancellation-free form and Cowper equality at ν = 0.
  - S-B κ_E = 16609827/33068756, κ_C, and the L = 1, 2 and 10 m tip deflections.
  - Six rigid modes, the pure-curvature patch and cut(L) = f_j.
  - Whole-body force and moment balance on every solve. The tolerances use coupled scales, so
    couple-only cases are checked too; a negative control perturbs a reaction moment.
- **Not covered.**
  - Generated self-weight. Self-weight and contents are authored intensities w = g(ρ_s·A_n + ρ_c·Ai)
    on the D-5 bases, so the product's D-5 mass basis itself is not exercised. Generated-self-weight
    models will be added when the PP owner supplies the exact request spelling.
  - Partial-span loads, which cannot reach the exact route.
  - Arcs, connectors and nonlinear supports.

## The judged set (`full`)

- **Size.** 964 models: 482 Euler–Bernoulli and their 482 Timoshenko twins.
- **Section variants.**

  | Variant | Models | Placements | Mill tolerance and allowance |
  |---|---|---|---|
  | `m0` | 684 | all three | both zero |
  | `mill` | 140 | origin and skewed | 0.125·t (rounded) and 0.5 or 1.5 mm |
  | `cm` | 70 | origin | mill tolerance only |
  | `ca` | 70 | origin | allowance only |

  The non-zero variants are applied to every third design of each family.
- **Placements.** 458 models at the origin, 212 at a UTM-scale offset (4.3e5, 7.3e6, 218 m) and 294
  skewed at (5e6, 7.3e6, 1.2e6 m).
- **Families.**

  | Family | Models |
  |---|---|
  | Cantilevers | 196 |
  | Fixed-fixed runs | 144 |
  | L-lines with sharp corners | 124 |
  | U, Z and 3D lines | 92 |
  | Skew multi-span runs on grounded springs | 92 |
  | φ = 2 cluster | 80 |
  | Pressure regions (collinear chains) | 104 |
  | Near the Sensitive boundary | 132 |

  Near the Sensitive boundary means soft torsion or translation springs and short members, tuned so
  that the gate's r̂ straddles 2^-26.
- **Load content.** Springs, prescribed motions (load-reference state, factor 1.1 on some),
  self-weight (with contents on the pressure bore for the larger sections), thermal strain and
  pressure.
- **Exact collinearity of skewed pressure chains.** These sit on a dyadic grid along (2, 3, 6)/7 at a
  dyadic offset, so they are exactly collinear in binary64. The product's collinearity guard is
  64·ε·span.
- **φ (Timoshenko twins, 1,476 elements).** From 1.0e-6 to 5.0e4. 56 elements lie within 2.3e-15 of
  2 at the origin (±3 ulps of the φ = 2 length) or within 4.1e-9 at offsets. On the earlier 824-model
  set the figures were 1,252 elements over the same range. The exact φ and φ − 2 are in each line's
  `ref.phi` and `ref.phi_minus_2`.
- **Controls (`controls.jsonl`).** For every Euler–Bernoulli zero-mill model, the reference with E
  scaled by 1 + 2^-20 (a perturbed stiffness).
- **The stale-vector mutant.** It has no product-level control here. Every exact-route member load is
  full-span, and for a full-span load the stale Euler–Bernoulli vector and the Timoshenko vector
  coincide (SHEAR_REFERENCE §4). Partial-span loads cannot reach the exact route (Design §3.4). The
  mutant is killed at element level by t4_u8's `timoshenko_distal_half_load_kills_the_stale_vector`
  (54/384 against 53/384).

## The comparison and the M3 verdict (`kd5t_compare.py`)

The surviving `compare_current.py` (sha256 `4b21fa99…`, T3 scratch `pr_kd5_eval_current/`) imports
RV30's `compare.py` and I41's `ref_ext.py`, which are lost. Its own logic is kept. The lost pieces
are rebuilt as follows, and the module docstring cites a source for every rule or marks it as this
harness's own (stricter) addition.

| Piece | Rule | Source |
|---|---|---|
| CRIT | RD(1e-9) = 0x1.12e0be826d694p-30; a breach is Σ(v − t)² > CRIT²·Σv² | K-D5 §3.3; cc `breach` |
| FLOOR, guard | 2^-34 and 2^-988; a group is judged iff S ≥ 2^-988 and max(q², t²)·conv² ≥ FLOOR²·S² | K-D5 §3.3; cc `judged` |
| `group_of` | Per location: \|N\|, hypot(V_y, V_z), \|T\|, hypot(M_y, M_z). Pipe-wall kinds are their own groups. Stress groups are keyed by basis. Supports: force vector, moment vector and the two magnitudes. Displacement, rotation and displacement magnitude | K-D5 §3.2; cc `ACT` and `conv`; analysis scripts |
| Lamé | A group per (pipe, location, basis), judged against its own q | own, stricter |
| Scales | SF = max(force, moment/L_b), SM = max(moment, L_b·force) from the published ACT groups | K-D5 §3.3; cc |
| Coverage | (U − \|axial\|)² ≥ My² + Mz² exactly, for every station sample of the same pipe and basis. The axial sample is the wall membrane stress where pressurized | K-D5 §3.2 and §4; cc. Membrane sample: own, stricter |
| Producer | `u6`: every stress row names its basis and both families are published. `pre-u6`: unsuffixed rows judged on the reduced (today's single) section | Design §§1, 2.1, 2.5 |

Here "cc" is `compare_current.py`.

**Verdict.** A run fails on any clause, and the set passes when no run fails.

| Clause | Requirement | Source |
|---|---|---|
| C0 | No reference or harness error | own |
| C1 | 0 judged member, section, support, stress or Lamé breaches on Passed cases | K-D5 §4.3 |
| C2 | 0 strict coverage failures | K-D5 §4.3 |
| C3 | On Passed cases, every displacement and rotation group within RD(1e-9) of its body's published scale (translations against the largest published translation, rotations against the largest published rotation) | K-D5 §4.3, literal. The scale coupled through L_b is reported for information |
| C4, C4b | 0 typed/captured disagreements (integrity codes or breach counts), and identical published rows | K-D5 §4.3 and b4; row identity own |
| C5 | No refusal, DTO or runner error | own |
| C6 | 0 unmapped or unpublished rows; on `u6`, no basis-free stress row and both maxima per pipe; on `pre-u6`, no basis-suffixed row | own; Design §2.1 |
| C7 | Same-basis evidence for every maximum; on Passed cases the evidence enclosure contains the exact maximum to within CRIT; headlines equal their basis maximum | own; Design §§2.2, 2.5 |
| C8 | No vacuous pass: every group the reference itself would judge (reference scales, a factor-2 floor margin) is judged | own |

`--selftest` passes 653 checks on the frozen subset. It covers:
- the predicates at their exact boundaries (CRIT strict, floor inclusive, guard);
- CRIT pinned from both sides (1 ± 1e-5) on the largest judged group of every group kind;
- C3 on rotations and translations scaled by 1 + 1e-6;
- C8 on an all-zero publication;
- coverage, headline and evidence mutants;
- unmapped and dropped rows, and a producer mismatch;
- typed/captured row and code changes, and a refusal;
- a nominal/reduced swap;
- nominal Lamé evaluated at the pressure bore;
- basis-free rows on `u6`, and reduced maxima without reduced evidence.

## Premise census (`kd5t_census.py`, `kd5t_reauthor.py`)

**`hager`.** κ₁(D·K·D)·r̂ from the gate's published r̂ and scale exponents D, with the engine's exact
free-DOF stiffness (straight models). diag(D·K·D) lies in [1, 4), which confirms the mapping.

**`probe`.** This consumes the dump of an instrumented product build, in the format given in the
module docstring. The runner exports `KD5_ACTIVE_TAG`, `KD5_ACTIVE_ENTRY` and `KD5_ACTIVE_MODE` so the
probe can label its lines. The consumer reports:
- ε_E/‖K̃_s‖₁ against the 2^-40 go/no-go and the 2^-36 pass condition;
- 2^10·est·ε_E;
- the Hager ratio ‖K̃_s⁻¹‖₁/est, from the exact inverse of a dumped K̃_s or, for straight models, of the
  engine's D·K·D.

`--selftest` validates this arithmetic on synthetic dumps. The probe itself is product
instrumentation. It is not built here, because the M3-T reference author does not read or write the
product implementation (Design §7).

**`standings`.** Counts Passed, Sensitive, fail-closed and refused for the re-authored corpora by
shear model, and the EB → Timoshenko moves by tag stem.

## Runner (scratch, untracked)

The runner is `Cargo.toml`, `src/main.rs` and `build.sh`. It was copied from the T3 scratch
`b4-finish/cand` with the `retained_direct` arm dropped.

`build.sh` symlinks the Git worktree that holds `$KD5T_PP` and builds through `t3_cargo.sh` with
`CARGO_TARGET_DIR` under the T3 targets tree. Inputs are lines `{tag, request}`. `KD5_IN`,
`KD5_OUT`, `KD5_ENTRIES` and `KD5_MODES` control a run; `KD5_SLIM` drops rows and evidence for
standings-only runs.

## Reproduce

```
python3 -I -B kd5t_engine.py --check
python3 -I -B kd5t_generate.py check          # self-checks, frozen bytes, compare self-test, SHA256SUMS
python3 -I -B kd5t_generate.py full <dir> 8
python3 -I -B kd5t_generate.py split <dir>/set.jsonl <runs>/chunks hv 40     # or: eb, ts, pending, stiffness_control, all
# run the runner on each chunk, then
python3 -I -B kd5t_compare.py <dir>/set.jsonl <runs.jsonl> <eval.jsonl> --producer u6 [--ref twin | --ref control:<dir>/controls.jsonl:pertE]
python3 -I -B kd5t_report.py <eval.jsonl>... --by section [--phi-table <dir>/set.jsonl]
python3 -I -B kd5t_census.py hager <dir>/set.jsonl <runs.jsonl>... --out <census.jsonl>
python3 -I -B kd5t_reauthor.py <eb|ts> <out.jsonl> <corpus chunk>...
python3 -I -B kd5t_census.py standings <runs.jsonl>...
```

`freeze` regenerates the references for an intended change of them, never to absorb a product result.
