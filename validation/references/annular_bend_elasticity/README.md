# Annular bend elasticity reference

The evidence for LIM-021 in `docs/RELEASE_LIMITATIONS_v1.0.md` and for the application-range sentence of DEL-03-03's
scope. This package compares the realized bend's element, a thin curved beam, with linear three-dimensional
elasticity of a curved bar of annular section. It also compares the element with the same element with transverse
shear added, and with a fuller candidate. Every figure that LIM-021 or this README states is a key of `figures.json`.
The package's check recomputes each one, and the generated block below maps each LIM-021 figure to its key.

**Not part of the required result.** This package's check is not part of the required `harness` result. It runs
in its own workflow, `.github/workflows/annular-bend-reference.yml`. A pull request that touches the package, or the
model documents it reads, is merged only after that workflow's result has been read.

## Files

| File | What it is |
|---|---|
| `generators/annular_reference.py` | Builds every figure and checks them (see Running) |
| `generators/torus_elasticity.py` | The elasticity reference: the interior section compliance of an annular curved bar |
| `generators/frame_solver.py` | The frame solver, an importable module (see The frame solver) |
| `generators/rectangular_bar.py` | The exact plane-stress solution of a rectangular curved bar |
| `requirements.txt` | The package's own pins (numpy and scipy) |
| `compliance.json` | The elastic interior section compliance of each grid geometry, with its mesh and mesh change |
| `figures.json` | Every figure: inputs, figures and limit checks |
| `SHA256SUMS` | The digests of the files above, written by the generator |

## The reference

*What it is.* An interior-compliance beam reference for a freely ovalizing annular bend. The bend is treated as a
segment of a torus. Each interior state of a curved bar has stresses that are constant around the torus or vary as
its first harmonic. Its displacement is a periodic field of the same harmonic, plus the angle around the torus
times a rigid motion (a dislocation of the closed torus). For each such rigid motion, prescribed with unit amplitude,
the periodic field minimizes the strain energy. That is a two-dimensional finite-element problem on the section,
solved with nine-node isoparametric quadrilaterals and a four-by-four Gauss rule. The four problems give the six
section states. The section's compliance per unit arc length follows from their resultants and their cross energy.
It includes transverse shear and its distribution, curvature, Poisson effects and free ovalization of the section.

*What it leaves out.* The boundary layers at a bend's ends, the restraint of ovalization by tangents or flanges, and
the particular three-dimensional fields of a distributed load. It is a beam reference for the bend's interior, not
a three-dimensional calculation of a finite bend with its tangents.

*How it is checked* (Limit checks, in the generated block):
- against the exact plane-stress solution, on a thin rectangular plate section, for the normal and shear energies
  under a radial end force;
- in the straight limit, at a large bend radius, against the Timoshenko straight beam with the energy-matched annular
  shear factor, for the diagonal and the off-diagonal entries;
- the energy-matched annular shear factor itself, reproduced from the flexure solution of a straight elastic tube;
- the work of each prescribed dislocation on a cut, against the strain energy in the volume, and the reciprocity of
  the cut work;
- the matched in-plane bending factor against a converged series for an inextensional thin ring section, and
  against the thin-shell estimate;
- the change of every grid compliance between the grid mesh and a coarser one.

The grid-mesh measure is a normalized difference between two meshes, not a bound on the error against the continuum.

*Its standing.* This project's own reference. It was read independently before it was packaged; that reading
repeated the plate, straight-limit and thin-shell checks and added the shear-factor, cut-work and thin-ring checks
kept here. The package's own review is that of the pull request that adds it.

## The frame solver

`generators/frame_solver.py` imports only the Python standard library and numpy. The elasticity reference
(`torus_elasticity.py`) also needs scipy. Given a model document in this repository's benchmark shape, the solver
returns, for each load case, node displacements and rotations, support reactions and member end actions:

```python
import frame_solver as fs
result = fs.solve(document, straight="timoshenko", arc="thin_shear", anchor_stiffness=None)
```

- `straight`: `"eb"` (Euler–Bernoulli), `"timoshenko"` (with the energy-matched annular shear factor) or `"model"`
  (the document's own shear model).
- `arc`: `"thin"` (DEL-03-03's element: the thin curved beam with the user's factors on its two bending terms),
  `"thin_shear"` (the same, with transverse shear added, unscaled by the factors) or `"candidate"` (the same, with
  the curved bar's coupling of axial force with in-plane moment added unscaled; the factor keeps its meaning).
- `anchor_stiffness`: six finite stiffnesses that make every anchor elastic.
- `k_in` and `k_out` override the document's factor; `compliance` gives the arc a section compliance (used for the
  elasticity reference); `arc_polyline` replaces each arc by straight members. The module docstring states every
  option, the output and the conventions. The same options are available on the command line.

## The comparisons

*Formulations.* In every comparison with elasticity, the straight pipe is the Timoshenko member, as the Timoshenko
selection makes it. The reference frame is the same frame with the bend given its elastic interior compliance. The
compared elements are the thin arc with shear and the candidate, each against the thin arc.

*Factors.* Five choices of the flexibility factor are compared:
- the thin-shell estimate in both planes. This was one sensitivity case, not a statement of what any code
  prescribes;
- the benchmark document's own factor;
- factors matched separately in each plane, reproducing the bend's elastic pure-bending flexibility relative to the
  thin beam;
- the out-of-plane match used in both planes;
- a factor of one.

A factor is a dimensionless ratio of section flexibilities in its plane. Matching it reproduces the pure-bending
response alone, not the section's shape, its coupled axial extension, or a finite bend with tangents.

*Measures.*
- Heated L-runs: the largest componentwise difference of the twelve anchor reaction components, divided by the
  largest reference component, with moments divided by the leg length, so that forces and moments share one scale.
- Benchmark cases: forces and moments, over every support, are each divided by their own largest reference
  component, and the larger of the two percentages is taken.
- Counts compare the absolute errors of the two elements. A difference within the tie band is a tie, and every
  count asserts that no deciding difference lies within the check's margin of the band.
- The isolated bend: one end is held and a force is applied at the other, against the arc with the elastic
  compliance. An entry is counted where the absolute percentage error changed by more than the screen when shear was
  added.

*The model change with shear.* In heated L-runs, the thin arc with shear is compared with the thin arc. The two are
bend formulations in the same frame solver, with factors of one and five, not an evaluation of any other program.
The measure takes its sign from the change in magnitude of the largest reaction component of the thin arc, and its
size from the largest change of any component, so the two need not refer to the same component.

*What the figures do not show.* No bound or dependable sign for any individual reaction, and no cutoff in bend
radius, wall ratio or leg length. No share of every frame reaction's error assigned to ovalization: the section
results show that ovalization beyond the factor is a major omission in tight, thin-walled bends, not a decomposition.
They also do not show end restraint, pressure stiffening, stresses, modal or spectrum results, or the curved-bar
terms out of the bend's plane.

## The candidate after v1.0

The candidate adds transverse shear and the curved bar's coupling of axial force with in-plane moment to the thin
arc. It keeps the user's in-plane factor on the existing bending term and still omits ovalization's larger coupling.
Its counts are in the generated block, beside those of the thin arc with shear. They are case counts, not error
magnitudes and not a basis for selecting an implementation. Its prerequisites:
- The factor keeps its meaning, the bend's pure in-plane bending flexibility relative to the thin beam, so no
  separate bending correction is added to a matched term, which would count it twice. Each of the three factor rules
  examined, recalibrated, reproduces the same section bending flexibility with the same coupling, so the choice rests
  on the meaning of the input, not on these counts.
- An admission bound on the in-plane factor. The candidate's axial–bending block is positive definite only for an
  in-plane factor above the square of r_g/R, where r_g is the section's radius of gyration. Today the input rule
  admits any finite positive factor.
- This reference, maintained with its review.
- The same formation in each place the product forms an arc's flexibility: the solve's element and its load
  vectors, the formation check and the certificate.

## Running

From this directory, with the packages of `requirements.txt` installed:

```sh
python3 -I -B generators/annular_reference.py           # check: recomputes and compares; writes nothing
python3 -I -B generators/annular_reference.py --write   # rewrites the outputs, this README's block and SHA256SUMS
```

The check compares every figure with `figures.json` at the relative tolerance `RTOL` (counts exactly), every limit
check with its bound, and every compliance with `compliance.json`. It compares the generated block below byte for
byte, the LIM-021 figures with their keys, and `SHA256SUMS` with the package's files. It fails if a model document
matching `MODELS_GLOB` is not the set `figures.json` records, if a number in LIM-021 has no key, or if a number in this
README's prose falls outside the block. The workflow, `software-workflow.json` and the generator name the model
documents by the same pattern, `cpb-g[234]-*.json` under `validation/caepipe_benchmark/models/`. The documents are
invented inputs; nothing of any other program's results is read.

## Generated figures

The block below is written by `generators/annular_reference.py --write` and compared byte for byte by its check.

<!-- generated by generators/annular_reference.py: begin -->
### Figures stated in LIM-021

| Text in LIM-021 | Key in figures.json |
|---|---|
| `two equal legs and a 90° bend` | an input of the samples |
| `R/D from 1 to 10, D/t from 5 to 100 and factors of 1 or 5` | an input of the samples |
| `by 1.0 to 4.2% of the largest reaction with legs of 3 diameters, 0.07 to 0.49% with legs of 10 and less than 0.05% with legs of 30` | `shear_change.legs3.range`, `shear_change.legs10.range`, `shear_change.legs30.range` |
| `38 / 5 / 32 of 75 heated L-runs and 14 / 19 / 35 of 68 load cases` | `counts.thin_shear.code.L_thermal`, `counts.thin_shear.code.benchmark`, `samples.L_runs`, `samples.benchmark_cases` |
| `34 invented benchmark systems` | `samples.benchmark_systems` |
| `28 / 15 / 25 of the 68` | `counts.thin_shear.document.benchmark` |
| `(74 / 1 / 0 and 43 / 15 / 10)` | `counts.thin_shear.unit.L_thermal`, `counts.thin_shear.unit.benchmark` |
| `max(1, 1.65/h)` | an input of the samples |
| `A factor of 1` | an input of the samples |
| `within 0.005 percentage points` | an input of the samples |
| `2.5, 12.8 and 70.5 times` | `section.R1.Dt5.coupling_over_plane_section`, `section.R1.Dt20.coupling_over_plane_section`, `section.R1.Dt100.coupling_over_plane_section` |
| `at D/t of 5, 20 and 100` | an input of the samples |
| `0.930 to 0.893 of the Timoshenko member's for D/t from 5 to 100` | `straight_limit.L4D.Dt5.euler_bernoulli_over_timoshenko`, `straight_limit.L4D.Dt100.euler_bernoulli_over_timoshenko` |
| `a member 4 diameters long` | an input of the samples |
| `nearer than the present element in 16, as near in 16 and further in 36 of the 68 cases` | `counts.candidate.document.benchmark` |

### Inputs

| Input | Value |
|---|---|
| Section grid | R/D 1, 1.5, 3, 5, 10; D/t 5, 10, 20, 50, 100; D = 1 m; E = 2e+11 Pa; Poisson's ratio 0.3 |
| Grid meshes (around x through the wall) | D/t 5: 96 x 3; D/t 10: 96 x 3; D/t 20: 144 x 2; D/t 50: 192 x 2; D/t 100: 288 x 2 |
| Heated L-runs | 75: the grid with legs of 3, 10, 30 D; uniform rise of 100 degC, alpha 1e-5 per degC; anchored at both ends |
| Benchmark systems | 34 documents matching `validation/caepipe_benchmark/models/cpb-g[234]-*.json` (16 with 1 bend; 12 with 2 bends; 6 with 4 bends), 68 load cases |
| Tie | absolute errors within 0.005 percentage points |
| Isolated-bend screen | absolute percentage error changed by more than 0.1 percentage points |
| Isolated-bend angles | 15 degrees, 30 degrees, 45 degrees, 90 degrees, 180 degrees |

### Shear added to the thin arc, against the thin arc (heated L-runs)

Largest componentwise change over the largest reaction component, moments divided by the leg length, per cent; factors 1 and 5, D/t over the grid. The largest reaction decreased in magnitude in 150 of 150 runs.

| R | legs 3 D | legs 10 D | legs 30 D |
|---|---|---|---|
| 1 D | 1.227 to 2.659 | 0.069 to 0.163 | 0.004 to 0.007 |
| 1.5 D | 1.495 to 3.340 | 0.087 to 0.223 | 0.005 to 0.011 |
| 3 D | 1.821 to 4.203 | 0.126 to 0.348 | 0.008 to 0.019 |
| 5 D | 1.695 to 4.186 | 0.157 to 0.436 | 0.011 to 0.028 |
| 10 D | 0.983 to 3.083 | 0.183 to 0.492 | 0.015 to 0.043 |

### Against elasticity: the thin arc with transverse shear added (unscaled) against the thin arc

Counts nearer / tied / further; largest gain and loss in percentage points.

| Factor | Heated L-runs (75) | Point force normal to the L-runs | Benchmark cases (68) |
|---|---|---|---|
| thin-shell estimate max(1, 1.65/h) in both planes | 38 / 5 / 32 (+3.66, -2.92) | 35 / 21 / 19 (+0.91, -0.28) | 14 / 19 / 35 (+0.44, -3.17) |
| the benchmark document's own factor (the L-runs have none) | none | none | 28 / 15 / 25 (+4.77, -2.83) |
| matched to pure bending separately in each plane | 18 / 5 / 52 (+2.89, -3.07) | 44 / 16 / 15 (+0.86, -0.12) | 7 / 24 / 37 (+0.61, -3.19) |
| the out-of-plane match used in both planes | 28 / 5 / 42 (+2.89, -2.46) | 44 / 16 / 15 (+0.86, -0.12) | 11 / 23 / 34 (+0.63, -2.74) |
| 1 in both planes | 74 / 1 / 0 (+8.31, -0.00) | 45 / 20 / 10 (+1.60, -0.04) | 43 / 15 / 10 (+4.77, -0.48) |

### Against elasticity: the candidate against the thin arc

Counts nearer / tied / further; largest gain and loss in percentage points.

| Factor | Heated L-runs (75) | Point force normal to the L-runs | Benchmark cases (68) |
|---|---|---|---|
| thin-shell estimate max(1, 1.65/h) in both planes | 47 / 1 / 27 (+2.61, -1.85) | 35 / 21 / 19 (+0.91, -0.28) | 31 / 20 / 17 (+0.41, -2.22) |
| the benchmark document's own factor (the L-runs have none) | none | none | 16 / 16 / 36 (+1.50, -2.24) |
| matched to pure bending separately in each plane | 63 / 0 / 12 (+2.65, -1.32) | 44 / 16 / 15 (+0.86, -0.12) | 31 / 23 / 14 (+0.55, -2.22) |
| the out-of-plane match used in both planes | 47 / 1 / 27 (+2.66, -0.74) | 44 / 16 / 15 (+0.86, -0.12) | 20 / 24 / 24 (+0.85, -1.68) |
| 1 in both planes | 20 / 0 / 55 (+4.46, -6.65) | 45 / 20 / 10 (+1.60, -0.04) | 13 / 17 / 38 (+1.74, -3.98) |

### The thin arc's own distance from elasticity, per cent

| Factor | Heated L-runs | Benchmark cases |
|---|---|---|
| thin-shell estimate max(1, 1.65/h) in both planes | 0.118 to 33.842 | 0.009 to 6.155 |
| the benchmark document's own factor (the L-runs have none) | none | 0.119 to 104.680 |
| matched to pure bending separately in each plane | 0.062 to 33.835 | 0.004 to 5.526 |
| the out-of-plane match used in both planes | 0.014 to 34.099 | 0.004 to 3.464 |
| 1 in both planes | 0.118 to 158.073 | 0.119 to 352.747 |

The smallest matched factor in either plane, over the grid and the benchmark sections: 1.0042.

### The isolated bend

Force-loaded tip-flexibility entries (tangential, radial and normal tip force) whose absolute percentage error changed by more than 0.1 percentage points when shear was added: with factors matched separately in each plane, shear was nearer in 275 of 369; with the thin-shell estimate, in 275 of 372. A radial tip force on a 90-degree bend, matched factors: shear further at 25 of 25 geometries, by 0.1 to 26.1 percentage points. Signed percentage error of the radial tip flexibility against elasticity, thin arc / with shear:

| R | D/t 5 | D/t 10 | D/t 20 | D/t 50 | D/t 100 |
|---|---|---|---|---|---|
| 1 D | +2.0 / +28.1 | +15.0 / +33.2 | +23.6 / +33.3 | +29.5 / +33.6 | +31.4 / +33.5 |
| 1.5 D | -2.0 / +11.9 | +3.6 / +14.7 | +8.4 / +14.1 | +11.0 / +13.3 | +11.8 / +12.9 |
| 3 D | -1.2 / +2.9 | -0.7 / +3.7 | +1.2 / +4.0 | +2.3 / +3.4 | +2.6 / +3.2 |
| 5 D | -0.5 / +1.1 | -0.5 / +1.3 | -0.0 / +1.5 | +0.7 / +1.4 | +0.9 / +1.2 |
| 10 D | -0.1 / +0.3 | -0.2 / +0.3 | -0.1 / +0.4 | +0.1 / +0.4 | +0.2 / +0.3 |

### The section grid

Elastic interior section flexibilities: the axial-moment coupling over the plane-section value 1/(E A R), the axial flexibility over 1/(E A), the matched factors, the in-plane factor over 1.65/h (h = t R / r_m^2), and the shear flexibilities over the straight pipe's with the energy-matched factor.

| R/D | D/t | coupling | axial | k_in | k_out | h | k_in h / 1.65 | shear in plane | shear out of plane |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 5 | 2.490 | 1.342 | 1.482 | 1.340 | 1.250 | 1.1228 | 0.9063 | 0.9682 |
| 1 | 10 | 5.626 | 2.581 | 3.205 | 2.802 | 0.494 | 0.9593 | 1.0699 | 1.1521 |
| 1 | 20 | 12.831 | 6.725 | 7.379 | 6.641 | 0.222 | 0.9910 | 1.4071 | 1.5782 |
| 1 | 50 | 34.314 | 21.519 | 19.779 | 18.451 | 0.083 | 0.9985 | 1.9921 | 2.3435 |
| 1 | 100 | 70.498 | 49.660 | 40.427 | 38.590 | 0.041 | 1.0000 | 2.5495 | 3.0837 |
| 1.5 | 5 | 1.993 | 1.096 | 1.217 | 1.171 | 1.875 | 1.3834 | 0.9693 | 0.9898 |
| 1.5 | 10 | 3.820 | 1.477 | 2.139 | 2.003 | 0.741 | 0.9601 | 1.0755 | 1.0966 |
| 1.5 | 20 | 8.594 | 3.575 | 4.867 | 4.605 | 0.332 | 0.9805 | 1.3605 | 1.4109 |
| 1.5 | 50 | 23.031 | 12.253 | 13.213 | 12.725 | 0.125 | 1.0006 | 1.9484 | 2.0687 |
| 1.5 | 100 | 46.712 | 29.130 | 26.970 | 26.290 | 0.061 | 1.0006 | 2.5080 | 2.6979 |
| 3 | 5 | 1.672 | 1.014 | 1.054 | 1.046 | 3.750 | 2.3964 | 0.9941 | 0.9981 |
| 3 | 10 | 2.248 | 1.047 | 1.312 | 1.293 | 1.481 | 1.1778 | 1.0283 | 1.0305 |
| 3 | 20 | 4.274 | 1.407 | 2.337 | 2.295 | 0.665 | 0.9417 | 1.1557 | 1.1605 |
| 3 | 50 | 11.607 | 4.822 | 6.589 | 6.507 | 0.250 | 0.9980 | 1.6110 | 1.6282 |
| 3 | 100 | 23.532 | 11.884 | 13.493 | 13.367 | 0.122 | 1.0012 | 2.0929 | 2.1239 |
| 5 | 5 | 1.601 | 1.004 | 1.020 | 1.017 | 6.250 | 3.8621 | 0.9980 | 0.9994 |
| 5 | 10 | 1.837 | 1.009 | 1.114 | 1.109 | 2.469 | 1.6675 | 1.0110 | 1.0116 |
| 5 | 20 | 2.720 | 1.072 | 1.529 | 1.519 | 1.108 | 1.0266 | 1.0651 | 1.0658 |
| 5 | 50 | 6.912 | 2.319 | 3.820 | 3.798 | 0.416 | 0.9643 | 1.3458 | 1.3492 |
| 5 | 100 | 14.243 | 6.181 | 8.106 | 8.072 | 0.204 | 1.0025 | 1.7617 | 1.7693 |
| 10 | 5 | 1.571 | 1.001 | 1.005 | 1.004 | 12.500 | 7.6128 | 0.9995 | 0.9998 |
| 10 | 10 | 1.654 | 1.001 | 1.029 | 1.028 | 4.938 | 3.0790 | 1.0029 | 1.0030 |
| 10 | 20 | 1.900 | 1.006 | 1.138 | 1.136 | 2.216 | 1.5283 | 1.0174 | 1.0175 |
| 10 | 50 | 3.452 | 1.164 | 1.890 | 1.887 | 0.833 | 0.9543 | 1.1161 | 1.1163 |
| 10 | 100 | 7.066 | 2.357 | 3.905 | 3.900 | 0.408 | 0.9660 | 1.3624 | 1.3633 |

At D/t 50 and 100 the matched in-plane factor departs from 1.65/h by at most 0.25% for h from 0.041 to 0.250, and by -4.6 to -3.4% for h from 0.408 to 0.833. Over the grid the coupling ratio runs from 1.57 to 70.50; at R = D the shear ratios run 0.906 to 2.550 in plane and 0.968 to 3.084 out of plane while the matched in-plane factor runs 1.48 to 40.43; at D/t 5 and 10 over every R the shear ratios run 0.9063 to 1.0755 in plane and 0.9682 to 1.1521 out of plane.

### The straight limit

Euler-Bernoulli over Timoshenko transverse tip flexibility of a straight cantilever four diameters long, 1/(1 + 3 E I / (G A_s L^2)), D = 0.3 m: D/t 5: 0.929965; D/t 15: 0.904307; D/t 100: 0.893302.

### The rectangular bar

Total strain energy of elementary beam theory against the exact plane-stress solution, per cent (thin bending without shear / with shear): shear further from exact at 11 of 11 sampled h/R.

| h/R | without shear | with shear |
|---|---|---|
| 0.02 | -0.00 | +0.01 |
| 0.05 | -0.02 | +0.05 |
| 0.1 | -0.07 | +0.19 |
| 0.2 | -0.26 | +0.77 |
| 0.3 | -0.57 | +1.74 |
| 0.4 | -0.96 | +3.10 |
| 0.6 | -1.89 | +7.02 |
| 0.8 | -2.70 | +12.67 |
| 1 | -2.95 | +20.34 |
| 1.2 | -2.01 | +30.74 |
| 1.6 | +8.56 | +68.12 |

### Limit checks

| Check | Bound | Observed |
|---|---|---|
| `frame_solver.polyline_192_vs_thin_shear` | at most 1e-05 | 3.02e-06 |
| `frame_solver.quarter_circle_closed_form` | at most 1e-13 | within the bound |
| `frame_solver.straight_limit.thin_shear_vs_timoshenko` | at most 1e-05 | 2.07e-07 |
| `frame_solver.straight_limit.thin_vs_euler_bernoulli` | at most 1e-05 | 1.93e-07 |
| `frame_solver.straight_timoshenko_closed_form` | at most 1e-13 | within the bound |
| `margin.bend_alone.code` | at least 1e-06 | within the bound |
| `margin.bend_alone.fit` | at least 1e-06 | within the bound |
| `margin.counts.candidate.code.L_outplane` | at least 1e-06 | within the bound |
| `margin.counts.candidate.code.L_thermal` | at least 1e-06 | within the bound |
| `margin.counts.candidate.code.benchmark` | at least 1e-06 | within the bound |
| `margin.counts.candidate.document.benchmark` | at least 1e-06 | within the bound |
| `margin.counts.candidate.fit.L_outplane` | at least 1e-06 | within the bound |
| `margin.counts.candidate.fit.L_thermal` | at least 1e-06 | within the bound |
| `margin.counts.candidate.fit.benchmark` | at least 1e-06 | within the bound |
| `margin.counts.candidate.oop.L_outplane` | at least 1e-06 | within the bound |
| `margin.counts.candidate.oop.L_thermal` | at least 1e-06 | within the bound |
| `margin.counts.candidate.oop.benchmark` | at least 1e-06 | within the bound |
| `margin.counts.candidate.unit.L_outplane` | at least 1e-06 | within the bound |
| `margin.counts.candidate.unit.L_thermal` | at least 1e-06 | within the bound |
| `margin.counts.candidate.unit.benchmark` | at least 1e-06 | within the bound |
| `margin.counts.thin_shear.code.L_outplane` | at least 1e-06 | within the bound |
| `margin.counts.thin_shear.code.L_thermal` | at least 1e-06 | within the bound |
| `margin.counts.thin_shear.code.benchmark` | at least 1e-06 | within the bound |
| `margin.counts.thin_shear.document.benchmark` | at least 1e-06 | within the bound |
| `margin.counts.thin_shear.fit.L_outplane` | at least 1e-06 | within the bound |
| `margin.counts.thin_shear.fit.L_thermal` | at least 1e-06 | within the bound |
| `margin.counts.thin_shear.fit.benchmark` | at least 1e-06 | within the bound |
| `margin.counts.thin_shear.oop.L_outplane` | at least 1e-06 | within the bound |
| `margin.counts.thin_shear.oop.L_thermal` | at least 1e-06 | within the bound |
| `margin.counts.thin_shear.oop.benchmark` | at least 1e-06 | within the bound |
| `margin.counts.thin_shear.unit.L_outplane` | at least 1e-06 | within the bound |
| `margin.counts.thin_shear.unit.L_thermal` | at least 1e-06 | within the bound |
| `margin.counts.thin_shear.unit.benchmark` | at least 1e-06 | within the bound |
| `reference.annular_shear_integral` | at most 1e-12 | within the bound |
| `reference.cut_reciprocity` | at most 1e-08 | 2.29e-09 |
| `reference.cut_work_vs_volume_energy` | at most 1e-06 | 6.12e-07 |
| `reference.mesh_change_on_grid` | at most 0.002 | 1.67e-03 |
| `reference.plane_stress_rectangular_bar` | at most 1e-05 | 6.61e-06 |
| `reference.straight_limit.Dt100.diagonal` | at most 0.0022 | 2.11e-03 |
| `reference.straight_limit.Dt100.off_diagonal` | at most 0.001 | 4.91e-04 |
| `reference.straight_limit.Dt20.diagonal` | at most 0.00012 | 1.12e-04 |
| `reference.straight_limit.Dt20.off_diagonal` | at most 0.001 | 5.05e-04 |
| `reference.straight_limit.Dt5.diagonal` | at most 0.00012 | 9.46e-05 |
| `reference.straight_limit.Dt5.off_diagonal` | at most 0.001 | 4.29e-04 |
| `reference.thin_shell_series` | at most 0.012 | 1.09e-02 |
<!-- generated by generators/annular_reference.py: end -->
