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
models. Expected values come only from the engine's own arithmetic. The runner (`runner/`) is harness,
not reference: it links the product crate by path and calls only `run_linear_static_preview_with_mode`
(typed) and `run_linear_static_preview_value_with_mode` (captured).

## Files

| File | Role |
|---|---|
| `kd5t_engine.py` | Exact reference engine: a public request in, every expected published row out (110-digit `decimal`; rational element controls in `fractions`). `--check` runs its self-check (58 checks). |
| `kd5t_generate.py` | Judged-set generator: `full <dir>` writes the set and controls, `freeze` writes the frozen subset, `split` makes runner inputs, `check` verifies everything. |
| `kd5t_compare.py` | Reconstructed M3 judging and coverage and the M3 verdict (acceptance clauses C0–C4, C6 and C8: the K-D5 M3 criterion and the soundness of its judging; and the information clauses), aware of the section basis and of the producer (`--producer u6\|pre-u6`). `--selftest <set>` exercises every rule and clause. |
| `kd5t_report.py` | Aggregates the comparison into the M3 tables and the verdict, by family, theory, placement or section variant, and by phi decade. |
| `kd5t_census.py` | Premise census: the Hager ratio from published gate values (`hager`), the probe-dump consumer (`probe`), and standings of the re-authored corpora (`standings`). |
| `kd5t_reauthor.py` | Re-authors the surviving RV30, realistic, RV29 and W1 corpora as 0.5.0 exact documents with either shear model. |
| `runner/` | The product-driving runner (harness): `Cargo.toml`, `Cargo.lock`, `src/main.rs` and `build.sh`. |
| `run.sh`, `census.sh` | Drivers: runner and comparison over a split set; re-authoring and slim runs of the census corpora. |
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
- **The stale-vector mutant is not observable on the judged set.** Every member load of the set is a
  full-span uniform intensity (authored, or self-weight and contents authored as intensities), and the
  other loads are nodal, axial (free strain, Poisson pairs, caps) or prescribed motions. For a full-span
  uniform load on a prismatic member the Timoshenko consistent vector is [wL/2, wL²/12, wL/2, −wL²/12],
  identical to the Euler–Bernoulli one (SHEAR_REFERENCE §4; Design §3.4), so a product that used the
  stale Euler–Bernoulli vector with Timoshenko K would publish the same bits as a correct one, in
  assembly and in recovery. The only load where the two vectors differ is a partial span, and partial
  spans cannot reach the exact route: their extents arise only from equivalent-static wind, which the
  exact route refuses (Design §3.4), and K-D5 records a Timoshenko frame with a partial-span primitive as
  unavailable (Design §4 item 4). The mutant is killed at element level by t4_u8's
  `timoshenko_distal_half_load_kills_the_stale_vector` (54/384 against 53/384). The product-level
  sensitivity control that remains is the Timoshenko product judged against the Euler–Bernoulli
  reference (`--ref twin` on the Timoshenko runs), which must breach at high φ.

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
| Lamé | Stress rows (the pressure-1 `stress` family). A group per (pipe, location, basis), judged against its own q, since they scale no certified resultant | K-D5 §3.2 "every published member row" (below); own scale |
| Scales | SF = max(force, moment/L_b), SM = max(moment, L_b·force) from the published ACT groups | K-D5 §3.3; cc |
| Reference scales | A force, moment, stress or Lamé group is also judged when it passes the guard and floor on max(published, reference): SF and SM formed from both the published and the reference values of the same groups, and max(q, t) for Lamé. A vanishing publication (all zero, or scaled below the guard) is therefore judged and breaches. Where the published scale passes the guard, any group judged on the larger scale is already judged on the published one, so nothing changes there | manager ruling (refuter finding F1) |
| Overflow | Ratios and magnitudes are display values, formed without exceptions (+inf beyond the binary64 range). Every decision is an exact rational predicate, so a judged group whose ratio overflows is a breach. A null (serde's non-finite) or non-finite published value is a C1 breach on a Passed case; a null evidence upper covers nothing (C2) | manager ruling (F2); K-D5 §3.3 "A nonfinite q … fails closed" |
| Displacement scales | S*_t = max(S_t, L_b·S_r) for translation groups, S*_r = max(S_r, S_t/L_b) for rotation groups; S_t and S_r the largest published translation and rotation components | K-D5 "The defect", §3.3, §4.1 (below) |
| Coverage | (U − \|axial\|)² ≥ My² + Mz² exactly, for every station sample of the same pipe and basis. The axial sample is the wall membrane stress where pressurized | K-D5 §3.2 ("the exact-pressure route uses wall axial stress") and §4; cc |
| Producer | `u6`: every stress row names its basis and both families are published. `pre-u6`: unsuffixed rows judged on the reduced (today's single) section | Design §§1, 2.1, 2.5 |
| Bodies | One connected body per model (per-body scales are not implemented; a second body is a harness error) | own |

Here "cc" is `compare_current.py`; L_b is the node bounding-box diagonal.

**Verdict.** Acceptance is the K-D5 M3 criterion, "0 judged member, section, support or stress
breaches on Passed cases and 0 strict coverage failures" (Design §4 item 5; K-D5 §4.3), together with
the soundness of the judging itself. Manager ruling for this epoch, on the refuter's findings F1 and
F3: "A Passed case on which an expected group is unpublished, or on which nothing is judged, is a
coverage failure." A run fails the verdict on any acceptance clause, and the set passes when no run
fails.

| Clause | Requirement | Source |
|---|---|---|
| C0 | Every run is judged: no reference or harness error | without it the criterion is not evaluated |
| C1 | 0 judged member, section, support or stress breaches on Passed cases (pipe-wall groups and Lamé rows included); no null or non-finite published value on a Passed case | K-D5 §4.3, §3.2, §3.3 |
| C2 | 0 strict coverage failures of the circular maxima | K-D5 §3.2, §4, §4.3 |
| C3 | On Passed cases, every displacement and rotation group within RD(1e-9) of its body's coupled scale of its kind, S*_t or S*_r | K-D5 per-DOF rule (below) |
| C4 | 0 typed/captured disagreements (integrity codes or breach counts) | K-D5 §4.3; b4 |
| C6 | On Passed cases, every reference row and every maximum of a basis the producer must publish is published (an expected group unpublished, for example dropped station rows, is a coverage failure) | coverage; manager ruling (F3) |
| C8 | On Passed cases, no vacuous pass: every group the reference itself would judge (reference scales, a factor-2 floor margin) is judged; a body with a nonzero reference has a judged group; a Passed run publishes rows | coverage; manager ruling (F1) |

Information clauses are this harness's own, stricter checks. K-D5 does not require them, so they are
counted and reported beside the verdict, not accepted on (manager ruling for this epoch: "a clause the
tool marks 'own and stricter' is reported information unless K-D5 requires it").

| Clause | Reported | Source |
|---|---|---|
| C3lit | C3 against the separate published scales (translations against the largest published translation, component or magnitude; rotations against the largest published rotation) | own, stricter literal reading |
| C4b | Typed and captured published rows differ | own |
| C5 | Refusal, DTO or runner error | own |
| C6i | Unmapped or duplicate product rows; on `u6`, a basis-free stress row; on `pre-u6`, a basis-suffixed row; on runs that are not Passed, unpublished rows or maxima and null or non-finite values | own; Design §2.1 |
| C7 | A maximum without same-basis evidence; on Passed cases an enclosure that misses the exact maximum by more than CRIT; a headline that is not its basis maximum | own; Design §§2.2, 2.5 |

**Lamé rows are in C1** (manager ruling on finding F4: "Lamé rows are stress rows. Include them in
C1's acceptance unless K-D5's text excludes those pipe-wall groups from the certified groups"). K-D5
does not exclude them; it does not name them at all. Neither K-D5 file contains "Lamé", "hoop" or
"radial". Its group text is general:

- §3.2: "The groups are every published member row, for straight members, realized arcs and connectors".
- §3.2: "Pipe-wall rows form their own groups."
- §3.2: "Stress rows are scaled at p from the certified resultants and judged through the resultant
  they scale. This covers the axial, bending and torsional components and the component
  stress-multiplier rows".
- Design §4: "The certificate bounds published member, section, support and pipe-wall groups".
- Design §2.1 lists `pipe_lame_radial_stress_v2` and `pipe_lame_hoop_stress_v2` among the rows "whose
  pressure-1 table family is `stress`".

Lamé rows scale no certified resultant (they are −p, 0 and the hoop terms of each basis's annulus), so
each Lamé group is judged against its own q, the strictest available scale.

**C3 and the K-D5 text it rests on** (`kd5-member-relative-bound.md`). K-D5 does not repair published
displacements and rotations; its per-DOF rule judges them against a body scale:

- The defect: "K-D5 bounds the forward error of u per DOF against max(|u_i|, S*_kind)."
- §3.2: "Displacement and rotation rows are published unchanged (u). K-D5's per-DOF rule judges them,
  as today."
- §3.3: "S*_g is the body's coupled scale over the published (repaired) rows. It is the moment scale
  for bending, torsion and support moment groups, and the force scale for axial, shear and support
  force groups (O-13's scale extension, which includes arcs)."
- §4.1: "The genuine evaluation judges each displacement or rotation group against its own q. […] All
  lie within D5C-1's 1e-9·S*. They are comparison artefacts of judging displacement groups without a
  body-scale floor; those rows are the solve's own and byte-identical to the baseline."
- §4.3: "The only judged breaches on Passed cases are the displacement and rotation noise groups of
  section 4.1 (RV30 224 case-modes, 508 groups; W1 212), every one within 1e-9 of its body's published
  displacement or rotation scale (worst 0.35)."
- The rulings, HELP_HUMAN 2026-10-12: "No displacement-noise floor is added; a floor would be a
  criterion change, which remains a possible later owner choice."

C3 is that rule: the scale of a displacement or rotation group is S*_kind, K-D5's coupled body scale
over the published rows, of the group's kind. K-D5 calls the coupled scales "the force scale" and "the
moment scale"; in the same way "its body's published displacement or rotation scale" is S*_t or S*_r.
K-D5 writes the coupling formula out for forces and moments (§3.3). The translation/rotation form
used here, S*_t = max(S_t, L_b·S_r) and S*_r = max(S_r, S_t/L_b), is the one T4-I6's independent
reference gives for K-D5's per-DOF rule (`validation/references/t4_i6/round_01/_run_records/r01_lib.py`,
`actual_ratio` and `kd5_trigger`: `tr, ro = max(st, lb * sr), max(sr, st / lb)`). The comparison is
per group (the 2-norm of the group's error), stricter than per DOF by at most √3, and S_t uses
components only.

- **A zero field in a moving body** (FF00: an axial-only run, true rotations exactly zero, published
  rotation noise 1.15e-15 rad) is judged against the body's coupled rotation scale, so its noise passes
  C3. The separate-scale reading fails it by a factor of 1e9 (C3lit).
- **An all-zero body.** No displacement-noise floor exists. A body whose published translations and
  rotations are all noise is judged against that noise and fails C3. A body whose rows are exactly zero
  passes.

`--selftest` passes 836 checks on the frozen subset. It covers:
- the predicates at their exact boundaries (CRIT strict, floor inclusive, guard);
- CRIT pinned from both sides (1 ± 1e-5) on the largest judged group of every group kind;
- C3 pinned from both sides: a component error of (1 ± 1e-5)·CRIT·S*_kind on translations and rotations;
- C3lit on rotations and translations scaled by 1 + 1e-6;
- rotation noise in an exactly zero rotation field (passes C3, raises C3lit), and an all-zero
  displacement field published as noise (fails C3) or as exact zeros (passes);
- F1: an all-zero publication (fails C1 on the reference scales, and C3), and every force, moment,
  stress and reaction row zero with correct displacements (fails C1 on the reference scales);
- F2: every action row scaled by 1e-300 (fails C1 with +inf ratios, no exception), a null and an
  infinite published value (C1 and C6), and a null evidence upper (C2);
- F3: one member's interior station rows dropped (C6);
- C8: a Passed run with no rows, and one gross support moment that lifts the published scales;
- coverage, headline and evidence mutants;
- unmapped rows (C6i), a dropped row (C6) and a producer mismatch;
- typed/captured row and code changes, and a refusal (C5 raised, no acceptance clause failed);
- a nominal/reduced swap;
- F4: nominal Lamé evaluated at the pressure bore, and reduced Lamé rows copied from nominal (C1);
- basis-free rows on `u6`, and reduced maxima without reduced evidence;
- a second body (C0).

**Measured verdict** (the saved runs on product `77cc26f426`, re-judged with this comparison; 964
models, both modes, typed and captured entries, 3,856 runs):

| Run set | Runs | Passed | Sensitive | Fail-closed | Verdict | Judged breaches on Passed (C1) | Worst C3, err/(c·S*_kind) |
|---|---|---|---|---|---|---|---|
| Timoshenko, own reference | 1,928 | 1,876 | 52 | 0 | PASS (all acceptance clauses 0) | 0 | 0.565 |
| Euler–Bernoulli, own reference | 1,928 | 1,804 | 124 | 0 | PASS (all acceptance clauses 0) | 0 | 0.742 |
| Timoshenko against the Euler–Bernoulli reference | 1,928 | | | | FAIL, 1,796 runs | 133,612 (C3 11,962) | |
| Euler–Bernoulli against the Timoshenko reference | 1,928 | | | | FAIL, 1,724 runs | 127,316 (C3 11,552) | |
| Euler–Bernoulli zero mill, E perturbed by 2^-20 | 1,368 | | | | FAIL, 1,220 runs | 86,864 (C3 7,194) | |

Every group judged on a Passed run is judged on the published scales: 0 groups are judged only on
the reference scales, 0 groups the reference judges are left unjudged, 0 rows are unpublished and
the worst judged Lamé group is 3.5e-7 of c. Information on the own references: C3lit 338 groups in
312 runs (172 Euler–Bernoulli, 166 Timoshenko), every other information clause 0. The worst
Euler–Bernoulli C3 over the whole set is 0.742 (`SHORT02|S3M1|utmskew|m0|eb`, typed, dense); the figure
"1.2e-4" in the message of commit `9bc482073d` is wrong. The earlier Euler–Bernoulli run set
(`w1eb`, 1,928 runs) gives the same counts and worst ratio.

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

## Runner (`runner/`)

A standalone crate: it joins no Cargo workspace (its own `[workspace]` root) and has its own
`Cargo.lock`; it changes no product build or lock file. Its one product dependency is
`../../../../core/product_physics`, the product of the checkout it sits in. The existing headless runner
(`openpipestress-runner solve`) does not suffice: it drives only the captured entry, and the judged set
needs the typed entry as well.

`runner/build.sh <checkout root> <target dir> <binary out>` builds it with `--locked`. Cargo is
`$KD5T_CARGO` (default `cargo`). For a checkout other than this one, for example a detached worktree of
a product commit, it first copies `Cargo.toml`, `Cargo.lock` and `src/main.rs` to the same path inside
that checkout. Inputs are lines `{tag, request}`. `KD5_IN`, `KD5_OUT`, `KD5_ENTRIES` and `KD5_MODES`
control a run; `KD5_SLIM` drops rows and evidence for standings-only runs.

The drivers take every path as an argument. `KD5T_PY` names the Python and `KD5T_WRAP` an optional
command prefix for each runner and comparison job (a host's resource wrapper).

## Reproduce

```
python3 -I -B kd5t_engine.py --check
python3 -I -B kd5t_generate.py check          # self-checks, frozen bytes, compare self-test, SHA256SUMS
python3 -I -B kd5t_generate.py full <set> 8
python3 -I -B kd5t_generate.py split <set>/set.jsonl <run>/chunks all 40     # or: eb, ts, hv, pending, stiffness_control
runner/build.sh <checkout> <target dir> <runner.bin>
./run.sh <runner.bin> <set> <run> own twin control     # writes <run>/out and <run>/eval
python3 -I -B kd5t_report.py <run>/eval/c.*.own.jsonl --by section --phi-table <set>/set.jsonl
python3 -I -B kd5t_census.py hager <set>/set.jsonl <run>/out/c.*.out.jsonl --out <census.jsonl>
KD5T_KEEP_INPUTS=1 ./census.sh <eb|ts> <runner.bin> <corpus dir> <census out>
python3 -I -B kd5t_census.py standings <census out>/*.out.jsonl
```

`freeze` regenerates the references for an intended change of them, never to absorb a product result.
