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
| `kd5t_compare.py` | Routes the judging to the shared M3 judge, `../m3_judge/` (route `exact`; refutation round 5). Its `--selftest` runs that judge's self-test, whose exact view is judged on the frozen subset; `kd5t_generate.py check` calls it. |
| `kd5t_census.py` | Premise census: the Hager ratio from published gate values (`hager`), the probe-dump consumer (`probe`), and standings of the re-authored corpora (`standings`). |
| `kd5t_reauthor.py` | Re-authors the surviving RV30, realistic, RV29 and W1 corpora as 0.5.0 exact documents with either shear model. |
| `runner/` | The product-driving runner (harness): `Cargo.toml`, `Cargo.lock`, `src/main.rs` and `build.sh`. |
| `run.sh`, `census.sh` | Drivers: `run.sh` calls `../m3_judge/run.sh exact` (build from a clean checkout, raw outputs, one judging pass per variant); `census.sh` re-authors and runs the census corpora slim with `runner/`. |
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
  assembly and in recovery. The only load where the two vectors differ is a partial span. This judged
  set contains only full-span member loads and cannot observe the stale partial-span vector.
  Exact-route partial-span wind is covered by `../w16_timoshenko_partial_wind/`. The mutant is killed at
  element level by t4_u8's
  `timoshenko_distal_half_load_kills_the_stale_vector` (54/384 against 53/384). The product-level
  sensitivity control that remains is the Timoshenko product judged against the Euler–Bernoulli
  reference (`--ref twin` on the Timoshenko runs), which must breach at high φ.

## The comparison and the M3 verdict (`kd5t_compare.py`, now `../m3_judge/`)

**Since refutation round 5 this set is judged by the shared M3 judge, `../m3_judge/`, route `exact`.**
Its validator turns each raw record into a closed, typed evidence view: both D-5 bases, units from the
pressure-2 contract, case-bound and unique identities, and lo ≤ hi. Its comparison reads only that
view. The rules below are ported there unchanged in meaning. The changes the view makes are listed in
`../m3_judge/README.md` ("The rules"): a repeated row identity, a basis-free stress row or a wrong
unit is refused rather than counted as information, and nonvacuity applies per family. The measured
verdicts in this section were judged with the earlier `kd5t_compare.py` at its epoch head on model
document 0.5.0. They stand as history; the re-judge below replaces them as the current record.

**Re-judge at current main (model document 0.8.0).** Main refuses 0.5.0 documents
(`MODEL_SCHEMA_VERSION_RETIRED`), so the set was re-authored as model document 0.8.0 with the same
expected rows (964 references, 268,694 rows). It was run at product head `9875952df7` with runner
sha256 `c0e7d413aa2a168053c4b5cbfe5e270d25e7927a5d5cf4390ec0db7728dc16c2`: 3,856 records, 3,840
under pressure-2 and 16 under pressure-source-1. The shared judge at `6c80e54528` (reports and records identical at `94498f8e0d`) judged them against
the committed pins: set `0e8573d7…`, controls `a3b26032…`, frozen subset `42ee244f…`
(`../m3_judge/REFERENCE_SHA256SUMS`; `m3_reproduce.py` regenerates all three).

| Run set | Recorded (0.5.0) | Re-judged (0.8.0, `9875952df7`) |
|---|---|---|
| Timoshenko, own reference | 1,876 Passed, 52 Sensitive; PASS; worst C3 0.565 | 1,876, 52; PASS; worst C3 3.95e-7 |
| Euler–Bernoulli, own reference | 1,804 Passed, 124 Sensitive; PASS; worst C3 0.742 | 1,804, 124; PASS; worst C3 2.53e-7 |
| Timoshenko against the Euler–Bernoulli reference | FAIL, 1,796 runs; C1 133,612; C3 11,962 | the same |
| Euler–Bernoulli against the Timoshenko reference | FAIL, 1,724 runs; C1 127,316; C3 11,552 | the same |
| Euler–Bernoulli zero mill, E perturbed by 2^-20 | FAIL, 1,220 runs; C1 86,864; C3 7,194 | FAIL, 1,220 runs; C1 86,864; C3 7,202 |
| Information, own references | C3lit 338 groups in 312 runs; every other clause 0 | C3lit 232 groups in 208 runs; C4b 16; C4x 10; every other clause 0 |

Every view was admitted, and provenance, inventory and nonvacuity pass on all three references. The
differences are expected product changes:

- **#1397 (`03f39bb49d`, Euler–Bernoulli) and #1468 (`fa019d9cef`, Timoshenko).** Retained-source
  recovery now succeeds for the SOFTRX05/06 near-sensitive models. Only the captured entry attempts it
  (DEL-04-08 ScopeOfWork CLM-003), so 16 captured runs publish pressure-source-1 while their typed
  pairs publish pressure-2. Their rows differ (C4b 16). Ten of these pairs also differ in breach
  counts on Sensitive cases: C4x, information under Agent 0's C4 ruling (`../m3_judge/README.md`,
  "The rules"), each entry judged against its own reference. The report lists the ten: SOFTRX05
  `utm` (eb and ts, sparse) and SOFTRX06 `o` and `utm` (eb and ts, both modes).
- **#1461 (`65cf0f42dd`, corrected nodal displacement).** The control's C3 count rises from 7,194 to
  7,202; C3lit falls from 312 runs to 208 (102 Euler–Bernoulli, 106 Timoshenko; recorded 172 and
  166); and the worst own-reference C3 on Passed runs falls from 0.742 to 3.95e-7. The earlier
  `kd5t_compare.py` reproduces the recorded numbers on the epoch records and these numbers on the
  0.8.0 records.

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
| C9 | On Passed cases, no row identity published more than once with different values. The identity is the judging key: case, kind, entity, location, component and, for stress rows, the basis. Only one publication of an identity can be judged, so without C9 the verdict would depend on row order. Each conflicting identity counts once, whatever the order | judging soundness; REFUTE3 residual; manager: "Make a second, different value for the same row identity an acceptance failure" |

Information clauses are this harness's own, stricter checks. K-D5 does not require them, so they are
counted and reported beside the verdict, not accepted on (manager ruling for this epoch: "a clause the
tool marks 'own and stricter' is reported information unless K-D5 requires it").

| Clause | Reported | Source |
|---|---|---|
| C3lit | C3 against the separate published scales (translations against the largest published translation, component or magnitude; rotations against the largest published rotation) | own, stricter literal reading |
| C4b | Typed and captured published rows differ | own |
| C5 | Refusal, DTO or runner error | own |
| C6i | Unmapped or duplicate product rows (a repeat with the same value included); on `u6`, a basis-free stress row; on `pre-u6`, a basis-suffixed row; on runs that are not Passed, unpublished rows or maxima, null or non-finite values and conflicting identities (C9's) | own; Design §2.1 |
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

`--selftest` passes 948 checks on the frozen subset. It covers:
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
- C9: a second value of one row identity, one ulp off or doubled, placed after or before the original
  (each fails C9 once; the doubled value placed first also fails C1, placed after it passes C1, which is
  the order dependence C9 removes); the same conflict on a Sensitive run (C6i only); a same-value repeat
  (C6i only); and a second, different maximum row of one pipe and basis (C9);
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

Both run sets were re-judged with C9 against the own, twin and control references (11,104 judged
records). No verdict, acceptance-failure count or information count changed on any record, and no row
identity is published twice.

**Product freshness (epoch head `addbdc030e`).** Between `77cc26f426` and `addbdc030e` the product
changed only in a byte-preserving receipt-hash serialization and the realized-arc evidence marker.
The runner was built against a detached `addbdc030e` and run on four chunks: cantilevers with every
section variant, the φ = 2 cluster, pressure chains with mill tolerance and allowance, and
near-Sensitive soft-spring and short-member models. That is 160 models (80 Euler–Bernoulli and 80
Timoshenko) and 640 runs, both modes and both entries, Passed and Sensitive. Every record equals the
saved `77cc26f426` record in every field except timing: rows, diagnostics, numerical quality, summary
and contract evidence. The verdict above therefore stands for `addbdc030e`.

**Product freshness (epoch head `1ef3487611`).** The whole judged set was rerun on a detached
`1ef3487611`, uninstrumented and with the census probe, 3,856 runs each. Every record of both runs
equals the saved `77cc26f426` record in every field except timing (1,078,632 rows). Judged with C9
against the own, twin and control references, the uninstrumented run gives the saved runs' judgement
in every field of every record. The verdict above therefore stands for `1ef3487611`.

**The bit-identical-to-main control (Design §4 item 5).** "The Euler–Bernoulli twins with zero mill
tolerance are bit-identical to main and must show 0 breaches."

- **Set.** The 342 Euler–Bernoulli models of section variant `m0`: mill tolerance and allowance both 0.
  That is 1,368 runs, both modes and both entries.
- **Runs.** On main `2b642c39fb` and on the epoch head `1ef3487611`. The committed runner builds against
  main unchanged (`build.sh`, `--locked`), so no main-compatible variant was needed.
- **Method, row for row.** The byte envelopes differ by design: the epoch publishes the reduced family,
  the basis tokens, the id segments and the per-basis headlines, and its limitations differ. So:
  - Each epoch row is stripped of its basis suffix: the final `:nominal_section`, `:reduced_section`,
    `:nominal-section` or `:reduced-section` id segment, and the `; section_basis=…` token in
    `metadata.basis`.
  - The stripped row must be one of main's rows. It must equal that row in kind, unit, entity, case,
    location, component and stripped metadata, and in value bit for bit (the binary64 repr, sign of zero
    included).
  - Every main row must be matched. The epoch rows without their reduced twins must be main's rows in
    main's order.
  - Main's `max_open_formula_stress` value must equal both per-basis headlines. Every other summary
    field must be equal, and so must the K-D5 integrity code and message, the other diagnostic codes and
    the numerical-quality record (basis-stripped).
- **Result: bit-identical on all 1,368 runs.**
  - Main's 281,892 rows are matched by 374,244 epoch rows: 189,540 non-stress rows one to one, and
    92,352 main stress rows each by its nominal and its reduced twin.
  - The 2,736 per-basis headlines equal main's headline.
  - There is no difference of any kind.
- **Main judged by this harness.** The same 1,368 main runs, judged with `--producer pre-u6` against the
  own reference: 1,246 Passed, 122 Sensitive, 0 judged breaches, 0 C3 groups, and every acceptance
  clause 0.
- **Main on the other Euler–Bernoulli variants (information).** These are the D-5 changes, not control
  failures:
  - The 140 mill-only runs are Passed on main and fail the D-5 reference (7,064 judged breaches): main
    forms stiffness on t − c_m, D-5 on the nominal wall t.
  - The 420 runs with a nonzero allowance are refused on main by name
    (`EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED`).

## Premise census (`kd5t_census.py`, `kd5t_reauthor.py`)

**`hager`.** κ₁(D·K·D)·r̂ from the gate's published r̂ and scale exponents D, with the engine's exact
free-DOF stiffness (straight models with at most 72 free DOFs). diag(D·K·D) lies in [1, 4), which
confirms the mapping.

**`probe`.** This consumes the dump of an instrumented product build, in the format given in the
module docstring. The runner exports `KD5_ACTIVE_TAG`, `KD5_ACTIVE_ENTRY` and `KD5_ACTIVE_MODE` so the
probe can label its lines. The consumer reports:
- ε_E/‖K̃_s‖₁ against the 2^-40 go/no-go and the 2^-36 pass condition;
- 2^10·est·ε_E;
- the Hager ratio ‖K̃_s⁻¹‖₁/est and the A1 margin 2^10·est/‖K̃_s⁻¹‖₁. The inverse is that of a dumped
  K̃_s or, for straight models without a dump, of the engine's D·K·D. A dumped K̃_s has its binary64
  entries taken exactly and is inverted by one Gauss–Jordan elimination in 110-digit decimal. At these
  conditions that is exact to far more digits than the ratio needs; the self-test checks it against the
  exact rational inverse (relative 1e-80).

`--selftest` validates this arithmetic on synthetic dumps (6 models). The probe itself is product
instrumentation, so it is not kept here: the reference side (engine, generator, comparison) still reads
no product code (Design §7).

**`standings`.** Counts Passed, Sensitive, fail-closed and refused for the re-authored corpora by
shear model, and the EB → Timoshenko moves by tag stem.

**ε_E census, measured (Design §4 item 5; K-D5 §2.1).** The product does not publish ε_E, so it was
measured with an instrumented build that was never committed.

- **Build.** A detached scratch worktree of the epoch head `1ef3487611` with a probe in FK
  `core/solver/frame_kernel/src/structural/formation_check.rs` (193 added lines, 3 changed). The
  solver is unchanged between `addbdc030e`, where the first census ran, and `1ef3487611`.
  - `epsilon_e` also keeps the row and column sums of each of its three parts (element formation,
    assembly rounding, projection change) and returns the same value.
  - `evaluate`, right after step 7, writes one JSON line per K-D5 evaluation to `$KD5_PROBE_OUT`: the
    gate's (r̂, est, ‖K̃_s‖₁), ε_E, its three parts, 2^10·est·ε_E and the outcome. Lines are labelled
    with the runner's `KD5_ACTIVE_*`.
  - `check_at` writes a line for an evaluation that ends before step 7 (a D5C-1 demotion). The product
    forms no ε_E there, so the probe forms it with the same `epsilon_e`.
  - For the A1 census, `check_at` also adds to the typed entry's line the scale exponents and the
    prepared scaled symmetric matrix K̃_s that the factor sees (`projected(r, c)`, free-DOF order), when
    the model has at most 160 free DOFs. Every evaluated model has at most 156. On all 91,236 dumped
    matrices the exact 1-norm equals the gate's floating-point ‖K̃_s‖₁ to 4.4e-16 relative, so the dump
    is the gate's matrix.
  - Nothing the probe computes reaches the result.
  - The patch is scratch evidence (`t4-u6u8-m3t4-ev/kd5_probe_formation_check_ks.diff`, sha256
    `f22561c5…`). It is the census's earlier probe (`6dedc3fa…`) plus the matrix dump.
- **No published byte changed.**
  - Judged set: all 3,856 instrumented runs equal the uninstrumented `1ef3487611` runs and the saved
    runs in every field except timing (1,078,632 rows, the diagnostics, summaries and evidence).
  - RV30, full records: chunk `rv30A c.000` (Euler–Bernoulli) and chunk `rv30D c.000` (Timoshenko),
    3,200 runs with 1,244,224 rows. The instrumented records equal the uninstrumented `1ef3487611`
    records in every field except timing.
  - Corpora, standings only: all 196,216 instrumented runs, every chunk of RV30, realistic, RV29 and W1
    in both theories, equal round 1's uninstrumented `77cc26f426` runs of the same re-authored inputs
    (diagnostics, numerical quality and summary).
- **Sets.**
  - The judged set, Euler–Bernoulli and Timoshenko.
  - The realistic, RV29, W1 and RV30 corpora re-authored by `kd5t_reauthor.py` with either shear model.
    The Timoshenko form makes straight tangents Timoshenko and keeps arcs Euler–Bernoulli. Re-authoring
    is deterministic: the realistic, RV29 and W1 inputs equal round 1's kept inputs byte for byte.
  - RV30 is the surviving RV30 input corpus (`b4-finish/eval/norule/rv30A`–`rv30D` in T3 scratch, 57
    chunks), 22,560 documents: A 7,968, B 7,056, C 1,584 and D 5,952. Every one has a realized bend; none
    has a nonzero mill tolerance.
  - Each set is run in both modes and both entries.
- **What is counted.** Every run has at most one K-D5 evaluation. Runs that the gate demotes
  (r̂ < 2^-26) have none, and neither do runs the exact route refuses. Every Passed run reached step 7.
  The ε_E figures exclude evaluations with no free DOF, where ε_E = ‖K̃_s‖₁ = 0 (W1, 216 per theory;
  judged set, 12 per theory).

ε_E/‖K̃_s‖₁ (go: every evaluation ≤ 2^-40 = 9.09e-13; no-go above 2^-36):

| Set | Shear model | Runs | Passed | Sensitive | Fail-closed | K-D5 evaluations | Median | p99 | Max | Max / 2^-40 | Max 2^10·est·ε_E |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Judged set | Euler–Bernoulli | 1,928 | 1,804 | 124 | 0 | 1,832 | 1.2e-16 | 7.1e-16 | 8.1e-16 | 8.9e-4 | 2.2e-5 |
| Judged set | Timoshenko | 1,928 | 1,876 | 52 | 0 | 1,888 | 1.4e-16 | 6.1e-16 | 6.7e-16 | 7.3e-4 | 2.2e-5 |
| Realistic | Euler–Bernoulli | 1,968 (4 refused) | 1,846 | 118 | 0 | 1,868 | 2.1e-15 | 6.3e-15 | 6.5e-15 | 7.2e-3 | 2.7e-4 |
| Realistic | Timoshenko | 1,968 (4 refused) | 1,842 | 122 | 0 | 1,868 | 2.0e-15 | 6.3e-15 | 6.5e-15 | 7.2e-3 | 2.7e-4 |
| RV29 | Euler–Bernoulli | 212 | 184 | 28 | 0 | 200 | 1.3e-15 | 4.9e-13 | 4.9e-13 | 0.541 | 1.5e-4 |
| RV29 | Timoshenko | 212 | 188 | 24 | 0 | 200 | 1.3e-15 | 4.9e-13 | 4.9e-13 | 0.542 | 1.5e-4 |
| W1 | Euler–Bernoulli | 5,688 | 5,344 | 344 | 0 | 5,344 | 2.9e-16 | 8.3e-16 | 1.1e-15 | 1.2e-3 | 2.4e-5 |
| W1 | Timoshenko | 5,688 | 5,560 | 128 | 0 | 5,560 | 2.8e-16 | 8.0e-16 | 1.1e-15 | 1.2e-3 | 2.1e-5 |
| RV30 | Euler–Bernoulli | 90,240 (1,056 refused) | 81,726 | 7,458 | 0 | 82,188 | 1.5e-15 | 1.9e-14 | 7.1e-14 | 0.078 | 5.3e-4 |
| RV30 | Timoshenko | 90,240 (1,056 refused) | 81,486 | 7,698 | 0 | 81,980 | 1.5e-15 | 1.9e-14 | 7.1e-14 | 0.078 | 6.5e-4 |

Every figure except RV30's equals the first census's on `addbdc030e`. The W1 medians now exclude the
evaluations with no free DOF.

**RV30 in detail.**

- **Refusals.** The 1,056 refused runs in each theory are 264 pressurized documents whose pressure
  region is not collinear. The exact route refuses them by name (`PRESSURE_REGION_NONCOLLINEAR`).
- **Distribution** of ε_E/‖K̃_s‖₁ over the evaluations:

  | Theory | [1e-16, 1e-15) | [1e-15, 1e-14) | [1e-14, 1e-13) | p90 | p99.9 |
  |---|---|---|---|---|---|
  | Euler–Bernoulli | 20,216 | 59,780 | 2,192 | 3.5e-15 | 4.4e-14 |
  | Timoshenko | 20,048 | 59,744 | 2,188 | 3.5e-15 | 4.4e-14 |

  No value exceeds 2^-40 = 9.09e-13.
- **The maximal case** is the same document in both theories: `B|L|10-3|t7|R0.2286|rho0.08|k5|w|o-xy`,
  an L-line with a bend.
  - The value is 7.08e-14 (Euler–Bernoulli 7.0779e-14, Timoshenko 7.0776e-14), 0.078 of the threshold.
    The element-formation part dominates: ε_E = 8.47e-13 with ‖K̃_s‖₁ = 12.0, against assembly 4.7e-16
    and projection 1.1e-16.
  - The K-D5 outcome is step 7 completed (the member-bound inputs exist). The case is Passed in both
    theories, with 2^10·est·ε_E = 2.2e-6.
- **Neumann factor.** The largest 2^10·est·ε_E is 5.3e-4 (Euler–Bernoulli) and 6.5e-4 (Timoshenko).
- **Fail-closed.** None.
- **Against K-D5 §2.1.** K-D5 §2.1 recorded RV30 figures of max 1.8e-13 and 2^10·est·ε_E 0.010 over
  41,310 evaluations. Those came from I44's probe at `98dba87714`, on the corpus as then authored.
  - This census measures the implementation's own ε_E on `1ef3487611`, with the corpus re-authored as
    0.5.0 exact documents. It has 41,094 Euler–Bernoulli case-modes.
  - Its maxima are lower. I44's evidence (`t4_i44/`) is not in the tree or the T3 scratch, so the
    difference cannot be traced case by case.
  - On realistic, RV29 and W1 the two measurements agree (the bullet below).

**Verdict: go, on every set and shear model.**

- No evaluation exceeds 2^-40 and none exceeds 2^-36, so there is no Passed or fail-closed case above
  the threshold.
- The largest value is RV29 `k10000`, an arc beside a stiff spring: 4.93e-13, 0.542 of the threshold.
  That case is Passed. The Timoshenko re-authoring keeps that arc Euler–Bernoulli.
- The Euler–Bernoulli maxima reproduce K-D5 §2.1's measurements: realistic 6.5e-15 and 2.7e-4, RV29
  4.9e-13, W1 1.1e-15.
- The Timoshenko maxima equal the Euler–Bernoulli ones to two digits on each corpus. The judged set's
  Timoshenko maximum is below its Euler–Bernoulli one.
- In every worst case the element-formation part dominates.
- No fail-closed case occurs.

Standings against the Euler–Bernoulli baseline (`kd5t_census.py standings`, by run, all sets):

- 90,348 runs stay Passed: RV30 81,190 and the other sets 9,158, as in the first census.
- 604 move from Sensitive to Passed:
  - 276 from the gate: W1 216, judged set 54, RV30 6;
  - 328 from D5C-1: RV30 290, judged set 22, realistic 10, RV29 6.
- 556 move from Passed to Sensitive:
  - 364 by D5C-1: RV30 346, realistic 12, judged set 4, RV29 2;
  - 190 by the gate (r̂ < 2^-26), all RV30;
  - 2 by the realistic recovery guard (R-b′), both after step 7.

  These moves are fail-safe. None ends fail-closed.
- Moves between Sensitive standings: 8 from the gate to D5C-1 (judged set 2, RV30 6), 22 from D5C-1
  to the gate (RV30), and 8 from the load-row guard to the gate (RV30).

**A1, the Hager ratio, measured (Design §4 item 5; K-D5 §2).** A1 is the premise ‖K̃_s⁻¹‖ ≤ 2^10·est.
K-D5 measured the Hager ratio ‖K̃_s⁻¹‖₁/est on Euler–Bernoulli sets only, finding [1, 7.17], a margin of
at least 140.

- **Round 1** (`kd5t_census.py hager`, saved `77cc26f426` runs). This is the engine's exact D·K·D for
  straight models with at most 72 free DOFs, over every typed run with a gate value, including
  gate-demoted runs. It covers:
  - the judged set, 958 runs per theory;
  - W1, 2,358 per theory (378 runs had more than 72 free DOFs);
  - RV29's straight models, 28 per theory;
  - no realistic model (each has a bend).
- **Epoch head** (`kd5t_census.py probe` on the dumped K̃_s of `1ef3487611`). This covers every K-D5
  evaluation with a free DOF, typed entry, on every set, arcs included. It is the matrix A1 is about.
- **Agreement.** Where both measure the same run, they agree. The judged set's maxima are 1.78305823598
  (Euler–Bernoulli) and 2.21151517551 (Timoshenko) in both, and W1's maximum is 4.6444798255 in both.

Hager ratio ‖K̃_s⁻¹‖₁/est on the epoch head (A1 margin = 2^10/ratio; A1 holds when the margin is at
least 1):

| Set | Shear model | Evaluations measured | Ratio range | Median | Smallest A1 margin | A1 failures |
|---|---|---|---|---|---|---|
| Judged set | Euler–Bernoulli | 910 | 0.99999999902 – 1.783 | 1.0 | 574 | 0 |
| Judged set | Timoshenko | 938 | 0.99999999912 – 2.212 | 1.0 | 463 | 0 |
| Realistic | Euler–Bernoulli | 934 | 0.99999999819 – 1.688 | 1.0 | 607 | 0 |
| Realistic | Timoshenko | 934 | 0.99999999906 – 1.666 | 1.0 | 615 | 0 |
| RV29 | Euler–Bernoulli | 100 | 0.99999999806 – 1.0000000010 | 1.0 | 1,024 | 0 |
| RV29 | Timoshenko | 100 | 0.99999999914 – 1.0000000007 | 1.0 | 1,024 | 0 |
| W1 | Euler–Bernoulli | 2,564 | 0.99999999976 – 4.644 | 1.0 | 220 | 0 |
| W1 | Timoshenko | 2,672 | 0.99999999962 – 4.612 | 1.0 | 222 | 0 |
| RV30 | Euler–Bernoulli | 41,094 | 0.99999999823 – 2.582 | 1.0 | 397 | 0 |
| RV30 | Timoshenko | 40,990 | 0.99999999868 – 2.577 | 1.0 | 397 | 0 |

- **Coverage.** 91,236 evaluations are measured, and 0 fail A1.
  - The overall range is [0.99999999806, 4.644], and the smallest A1 margin is 220.5. That case is W1
    `L-corner|t24|R0.2286|sub1|w|o-xy`, Euler–Bernoulli, 6 free DOFs, Passed.
  - The smallest Timoshenko margins are 463 on the judged set, 615 on realistic, 1,024 on RV29, 222 on
    W1 and 397 on RV30.
  - On realistic, RV29, W1 and RV30, the Timoshenko margin equals or exceeds its Euler–Bernoulli twin's.
    The worst W1 case is the same L-corner document in both theories.
  - On the judged set the Timoshenko margin is lower, 463 against 574. Its worst case,
    `PR04|S4M1|utmskew|m0|ts`, has a ratio of 2.21, against 1.78 for the Euler–Bernoulli worst. That is
    still 3.3 times K-D5's measured margin of 140.
- **Ratios below 1.** The ratio falls below 1 by at most 2e-9. The deficit grows with the condition:
  the 22 deficits above 1e-9 all have r̂ ≤ 1.9e-8 (2^-26 = 1.5e-8), and every deficit above 1e-12 has
  r̂ ≤ 3.7e-5. This is the estimate's own floating-point error, since its solves through the factor carry
  about κ·u.
- **Not measured.** Evaluations with no free DOF have no matrix (W1 108 typed per theory; judged set 6).
  Gate-demoted runs have no K-D5 evaluation, so A1 is not used on them. Round 1 measured those on its
  straight models, and they fall in the same range: round 1's smallest margin over all its runs,
  gate-demoted ones included, is 220.

## Runner (`runner/`)

The M3 runs use the shared runner, `../m3_judge/runner/`, which records each run's request, runner and
product provenance; this one remains for the census's slim runs (`census.sh`).

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
runner/build.sh <checkout> <target dir> <runner.bin>   # the census runner (census.sh, slim runs)
./run.sh <clean checkout> <set> <run> own twin control   # ../m3_judge/run.sh exact: builds the shared runner, raw outputs, one judging pass per variant
python3 -I -B ../m3_judge/m3_report.py --route exact <set>/set.jsonl <run>/out/c.*.out.jsonl --runner-sha256 <hex64> \
        --product-head <hex40> --checkout <dir> --by section   # the same pass by hand
python3 -I -B kd5t_census.py hager <set>/set.jsonl <run>/out/c.*.out.jsonl --out <census.jsonl>
KD5T_KEEP_INPUTS=1 ./census.sh <eb|ts> <runner.bin> <corpus dir> <census out>
python3 -I -B kd5t_census.py standings <census out>/*.out.jsonl
python3 -I -B kd5t_census.py probe <probe dump.jsonl> --workers 8 --out <records.jsonl>   # instrumented build only
python3 -I -B kd5t_census.py --selftest <set>/set.jsonl <run>/out/c.000.out.jsonl
```

`freeze` regenerates the references for an intended change of them, never to absorb a product result.
