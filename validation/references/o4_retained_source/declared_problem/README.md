# O4 declared-problem oracle for the retained-source route

This is the maintained oracle of DEL-04-08 Design
[`o4-source-extension.md`](../../../../execution/PKG-04/DEL-04-08/Design/o4-source-extension.md)
section 5, "A declared-problem oracle for the source route": RF's P1 method
made a maintained oracle. It judges the source route's selected cases from the
product's test-only dump of the private normalized payload and functional plan,
and from the published envelope of the same request and solver mode.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Every
model is invented. No code SIF, flexibility formula, table or allowable
appears (F-PIP-1), and no `domains/piping-design/` artifact was used (DEC-043).
These results do not establish acceptance.

## Files

| File | Role |
|---|---|
| `BOUND.md`, `bound.json` | The per-entry K_local bound and its machine form. Frozen and committed (`674a7df342`) before any dump, envelope or other product output was produced or opened. `oracle.py` re-derives every count from BOUND.md's formulas and refuses a `bound.json` that disagrees |
| `oracle.py` | Standard library only (`fractions`, `hashlib`, `json`, `struct`, `math`). The judge, the self-test and the real-data controls |
| `twins.py` | Re-authors J2's twelve source requests into twins; converts RF's PIN-OFF-345 census requests and the frozen unit's judged-set documents into product requests (R2b); runs the product's dump entry and the headless runner on them; writes the selection census |
| `run.sh` | Regenerates the twins and documents, builds through the T3 cargo wrapper, runs both product entries in both modes under one T3 slot, then runs the self-test, the oracle, the controls and the census. Twins, dumps, envelopes and reports stay in scratch |

## Independence

The oracle depends on no product formation code. Its author read none of
`core/product_physics/src/**`, `core/solver/**`, `core/reporting/**` or
`tests/verifiers/**`, with two exceptions allowed by the steer: the doc comment
of `with_source_dump` and its record format (`source_receipt/source.rs`, about
lines 550–600), and the test entry `o4_source_payload_dump`
(`source_receipt/o4_source_tests.rs`, lines 839–865), read only to learn how to
invoke it.

Inputs read:
- DEL-04-08 Design `o4-source-extension.md`: sections 3, 4.4 (the counting
  method) and 5. `rf-source-method-extension.md`: its summary and sections 1.1,
  2, 3, 7 and 10 (P1, the declared problem, projections, the receipt).
- DEL-05-03 Design `section-bases-and-timoshenko.md` (U8 §3.1's fixed-order κ̂,
  U8 §3.3's formation chain, the D-5 bases).
- `schemas/physics_source_recovery.schema.json`, for the receipt's public shape.
- The frozen unit `../` at `d4dbf9187d`: its README and
  `source_twins_cases.json` (the twins' allowance values and the scope table).
  Its generators and `oracle.json` were not opened.
- The twelve J2 request inputs
  `fixtures/product_preview/pressure_source/{physics_source,load_reference_source}/*.request.json`.
- R2b, input only: `fixtures/results/j7_rf_census_inputs.json`, using only the
  RF-SKEW-T-PIN-OFF-345 r1e-08 and r1e-12 requests, decoded by the rule of
  `expand` (lines 1–92 of `tools/fixtures/retained_precision_corpus.py`,
  re-implemented in `twins.py`; the module is not imported); and the frozen
  unit's `cases.json` documents (inputs only; `oracle.json` was not opened).
- To obtain the published envelopes (the dump entry writes only the dump), the
  headless runner's command-line plumbing: `core/runner/headless/Cargo.toml`,
  `src/bin/headless_preview_runner.rs`, the argument and input parts of
  `src/bin/openpipestress-runner.rs`, and the request-metadata builder of
  `tests/load_reference_cli.rs`. None of these forms, solves or reads a source.
- The dumps and envelopes the product produced in scratch.

Every expected value comes from this file's own arithmetic on the declared
operands: the exact solve, statics, the exact reference K_local of U8 §3.3 and
the counting rule of §4.4.

## What is judged

Per selected source case (`retained_source_blocks_exact_v1`):

- **(a) Commitments.** sha256 of each dump text equals the record's hash and
  the receipt's `normalized_source_sha256` / `functional_plan_sha256`; the
  dump's `publication_sha256` equals the receipt's; the domains, the payload's
  case id and the plan's binding of the payload hash agree. The oracle so
  judges exactly the committed source.
- **(b) The declared problem, exactly.** K is the sum over frames of the exact
  congruence Tᵀ·K_local·T of the declared binary64 K_local and T (for a signed
  permutation, exactly ±K_local; the declared stiffness terms must bind it),
  plus the springs; f is every declared force term, including the resolved
  eigenloads; prescribed DOFs take their declared motions. It is solved in
  `Fraction`s; the free-row residual must be exactly zero. Every functional's
  exact value is then computed **by statics in this file**, not from the plan's
  coefficients: nodal components (m to mm exactly), end actions
  K_local·T·u_e − T·f_eq, j-side station actions from the i-end action
  (N = −f₀, V = −f₁,₂, T = −f₃, M_y = −f₄ − x·f₂, M_z = −f₅ + x·f₁ at
  x = s·L), support actions (rigid: Σ frame forces − loads on the DOF; spring:
  −k(u − g); structural zero otherwise). Asserted:
  - the plan's own lowering, evaluated exactly, equals statics, and every plan
    factor resolves to its payload bits;
  - every published projection interval contains the exact value, its value
    lies in it, |value − exact| ≤ its absolute bound, `exact_zero` and
    `exact_identity` hold exactly, and the published row carries the value;
  - every station and endpoint action interval, and every support-norm
    component and norm interval, contains the exact value.
- **(c) Stress checks and maxima, per basis.** Each check's parameters are bit
  for bit the payload's leaves of its own basis (nominal: the formation area,
  torsion constant and `recovery_section`; reduced: `reduced_recovery_section`)
  and pass BOUND.md 4.3's annulus screen for that basis's wall (t, or
  fl(fl(t − c_m) − c_a)). Its action interval contains the exact functional.
  The published row recomputes from the action and those parameters within
  ρ_ops (two roundings for N/A and M/Z, three for T·r_o/J, each then over
  `pa_per_mpa`). The reduced check shares functional and action with its
  nominal twin; the reduced section is present exactly when the walls differ.
  Each basis's endpoint maximum interval contains the exact
  max(|N|/A + √(M_y² + M_z²)/Z) over both ends, with that basis's A and Z.
- **(d) K_local against BOUND.md.** Every declared entry is within
  ρ_N·S of the exact matrix re-formed from the formation inputs, structural
  zeros exactly ±0, with each frame's theory equal to the request's.
- **Supplementary screens (BOUND.md section 4).** The shear-area leaf equals
  RN(κ̂·A) with κ̂ in U8 §3.1's fixed order, bit for bit; the stiffness section
  is the nominal annulus; the length leaf matches the coordinates; an eigenload
  is E·A_nominal·ε within ρ₂ and enters as (−P, +P) on the member's axial DOFs.
- **(e)** A per-case verdict, the counts of every check, and the worst margins.

The **self-test** (`oracle.py --selftest`) builds hand-made dumps, envelopes
and requests: four consistent ones (Timoshenko and Euler–Bernoulli, with zero
and nonzero allowance) that must pass, and sixteen mutants that must each
fail, among them: a payload altered after commitment, a receipt hash differing,
a projection interval excluding the exact value, a declared load changed under
the same receipt, a plan lowering differing from statics, nominal parameters on
a reduced check, a stress row that does not recompute, a maximum below the
exact maximum, a missing reduced section, and K_local formed wrongly but
consistently (k_θθ with 4 + 2φ, k_vv at twice the bound, a stray structural
nonzero, an Euler–Bernoulli matrix on a Timoshenko frame, a flipped x–z
coupling sign) so that only the bound can see it, a shear area from 5/6, and a
frame theory differing from the request. One more consistent mutant (k_vv at
half the bound) must pass. Result: 21 of 21 as expected.

The **real-data controls** (`--controls`) must each fail: every Timoshenko
dump against its Euler–Bernoulli twin's envelope (96 of 96 failed), every
declared Timoshenko K_local against the Euler–Bernoulli reference of its own
leaves (96 of 96 exceed the bound), and every nonzero-allowance dump against
its zero-allowance twin's envelope (144 of 144 failed).

## Coverage

- **Twins.** Each of J2's twelve requests (`physics_source`: n05, n06,
  n05_unicode, n05_units, fields, mixed, mixed_units; `load_reference_source`:
  n05, n06, eigen_motion, fields, mixed) in eight forms: the original
  Euler–Bernoulli zero-allowance request (harness validation), its Timoshenko
  twin, and Euler–Bernoulli and Timoshenko twins with `mill`
  (c_m = 0.125·t), `ca` (c_a = 0.0015 m) and `both`. Only `shear_model`,
  `mill_tolerance` and `corrosion_allowance` change. The 77 allowance pairs
  that the frozen unit also froze agree with it. `eigen_motion` (the
  load/reference-state family the frozen unit left out of scope) is
  re-authored by the same rule. 96 requests, each in `sparse_interactive` and
  `dense_scrutiny`: 192 runs.
- **Selection.** The source route selected every source load case in all 192
  runs (192 cases). It did not select `case:ordinary-pressure` of `mixed`,
  `mixed_units` and `load_reference_source/mixed` (48 run-cases): the ordinary
  attempt's outcome is `checks_passed`, and the product states
  "retained_source_attempt=not_required_ordinary_checks_passed" (the
  load-reference diagnostic `LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED`); the
  originals behave the same way.
- **Load/reference-state features.** `eigen_motion`'s `case:join` (16 runs) is
  judged with its resolved eigenload E·A·ε (thermal and fit strains combined),
  prescribed motions UY and RZ on `anchor` and UX on the `line_stop`
  (`stop`), which the payload declares as prescribed DOFs, and the spring.
- **What the corpus does not reach.** Every J2 request has one straight member
  (D = 0.2 m, t = 0.01 m, L = 2 m along x, E = 200 GPa, ν = 0.25) with an
  identity transform. So there are only two distinct declared K_local
  matrices, one Euler–Bernoulli and one Timoshenko (φ = 0.0676), no congruence
  member, no length from coordinates (every frame declares a length leaf, so
  ℓ = 0), and no φ near 2. Most requests load only torsion; `fields` loads all
  six components. RF's PIN-OFF-345 twins (a (3,4,0) congruence member) were
  not run: they are not J2 requests, and their census inputs were outside this
  task's reading list.

## Results (on `8c1966e8f7` plus this unit)

**192 of 192 cases PASS, 0 FAIL.** By family: `physics_source` 112,
`load_reference_source` 80. By shear model: Euler–Bernoulli 96, Timoshenko 96.
By allowance class: zero 48, mill 48, ca 48, both 48. By mode:
`sparse_interactive` 96, `dense_scrutiny` 96. Every request: 16 of 16.
The Euler–Bernoulli zero-allowance originals (24 cases) pass every rule, which
validates the harness, the row mapping and the K_local layout.

Checks made: 10,464 projections (each for containment, value, absolute bound
and published row; 9,136 basis assertions), 12,960 plan-versus-statics
functionals, 5,760 station and 1,152 endpoint actions, 2,400 support-norm
components and 800 norms, 7,680 stress checks, 384 per-basis maxima, 192
K_local matrices (96 Timoshenko), 96 shear-area leaves and 16 eigenloads.

Worst margins:
- **Projections.** Largest |value − exact|/absolute bound: 0.333
  (`fields|zero|eb`, bending z at end j). Largest |value − exact|/|exact|:
  1.81e-16. Smallest relative distance of an exact value from an outward
  interval endpoint: 3.3e-16 (`eigen_motion|zero|timo`, tip RZ).
- **Actions.** Largest relative error of a published action value: 2.06e-16;
  smallest endpoint slack 3.6e-16.
- **Stress rows.** Every published stress row equals, bit for bit, the
  natural-order recomputation from its action and parameters; largest relative
  error against the exact functional's stress: 2.31e-16.
- **K_local against the bound** (largest |declared − exact|/(ρ_N·S)):
  Timoshenko 0.172 overall; k_axial 0.172 (N = 2), k_θθ 0.131 (N = 18), k_vθ
  0.129 (N = 12), k_vv 0.119 (N = 13), k_θθ' 0.084 (N = 18), k_torsion 0.018
  (N = 2). Euler–Bernoulli 0.172 overall (k_axial; k_vθ 0.120, k_vv 0.096).

No finding. Nothing failed.

## R2b: RF congruence twins and the frozen judged set

`oracle.py`, `BOUND.md` and `bound.json` are unchanged from `25e2689576` and
`674a7df342`. `twins.py` gains two converters:
- **RF PIN-OFF-345.** Each census request (model 0.3.0, contract without a
  shear model, sections without allowances) becomes model 0.6.0 with an
  explicit `shear_model`, `mill_tolerance` and `corrosion_allowance`, in the
  same eight forms as the J2 twins; all 16 allowance pairs agree with the
  frozen RF twins. One (3,4,0) member of L = 5 m, pinned translations at both
  ends, a soft RX spring at N0 and two moments at N1.
- **Judged set.** Each of the frozen unit's 172 `cases.json` documents (33
  designs, every one inside the source family and D1) becomes one model 0.6.0
  request per README "Binding": one load case `case`, rigid supports as
  anchors on exactly their DOFs, grounded springs on one DOF, explicit section
  lengths including `mill_tolerance`, and empty pressure regions.

### Selection census (both modes)

| Documents | Runs | Selected | Product's stated reason where not selected |
|---|---|---|---|
| RF PIN-OFF-345 r1e-08, r1e-12, all 8 twins | 32 | 32 | — (level `complete_identified_declared_member_congruence`) |
| CONTRAST-STIFF (zero, mill, both; EB and Timoshenko) | 12 | 12 | — |
| SHORT-MEMBER Euler–Bernoulli (zero, ca, both) | 6 | 6 | — |
| SHORT-MEMBER Timoshenko | 6 | 0 | ordinary `NUMERICAL_INTEGRITY_CHECKS_PASSED` (quality Passed); no source attempt, envelope pressure-2 |
| SOFT-RUN r1e-08, r1e-12 | 20 | 0 | `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`: `Exact(UnsupportedBlock { order: 10 })` at Preparation (ordinary Sensitive) |
| SOFT-TORSION r1e-08, r1e-12 | 16 | 0 | `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`: `UnsupportedBlock { order: 7 }`; in `dense_scrutiny`, r1e-12 also ends `MODEL_INCOMPLETE` (`NUMERICAL_INTEGRITY_UNRESOLVED`, nonpositive pivot) |
| RFLIKE-SKEWT-PIN-OFF-345 r1e-08, r1e-12 (tee) | 48 | 0 | `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`: `UnsupportedBlock { order: 10 }`; in `dense_scrutiny`, r1e-12 also ends `MODEL_INCOMPLETE` (`NUMERICAL_INTEGRITY_UNRESOLVED`) |
| All other 25 designs, including SKEW345-*, SKEW122-* ((1,2,2) congruence), PROP-PHI2-EXACT-A/B, PROP-PHI2-NEAR-X/-345-OFF, CAPS-LOOP-32 | 236 | 0 | ordinary `NUMERICAL_INTEGRITY_CHECKS_PASSED` (quality Passed); the source route is not attempted and the envelope is pressure-2 |

So 50 of 376 runs reach the source route. No document is declined by family
name: every non-selection is either the ordinary route passing or the exact
block search refusing an order above 4.

### Results

**All 50 selected cases PASS, 0 FAIL** (RF 32, judged set 18).
- RF: Euler–Bernoulli 16, Timoshenko 16; zero, mill, ca and both 8 each;
  each mode 16. The real-data controls fail as required (cross-theory 16 of
  16, Euler–Bernoulli reference on Timoshenko K 16 of 16, cross-allowance 24
  of 24).
- Judged set: CONTRAST-STIFF 12 (Euler–Bernoulli 6, Timoshenko 6), SHORT-MEMBER
  6 (Euler–Bernoulli); zero 6, mill 4, ca 2, both 6; each mode 9.
- Worst projection margins: |value − exact|/absolute bound 0.381 (RF) and
  0.403 (SHORT-MEMBER); relative error at most 2.16e-16; smallest endpoint
  slack 2.46e-16 relative.
- K_local share of the bound, largest per role:

  | Role (N) | RF Timoshenko (φ = 0.0108) | RF Euler–Bernoulli | CONTRAST-STIFF Timoshenko (φ 1.6e-4 to 28.1) | Judged-set Euler–Bernoulli |
  |---|---|---|---|---|
  | k_axial (2) | 0.059 | 0.059 | 0.288 | 0.288 |
  | k_torsion (2) | 0.225 | 0.225 | 0.356 | 0.356 |
  | k_vv (13 / 5) | 0.041 | 0.009 | 0.040 | 0.226 |
  | k_vθ (12 / 4) | 0.057 | 0.314 | 0.043 | 0.120 |
  | k_θθ (18 / 3) | 0.026 | 0.012 | 0.054 | 0.087 |
  | k_θθ' (18 / 3) | 0.051 | 0.012 | 0.043 | 0.087 |

- **φ ≈ 2: no row reached.** The four PROP-PHI2 designs (φ = 2 exactly, and
  2 + 1.4e-16 and 2 − 3.3e-11) pass the ordinary checks in every twin and mode,
  so the product never forms or commits their declared problem, and there is
  no dump to judge. The same holds for the (1,2,2) congruence members of
  SKEW122-CANT, -FRAME-OFF and -YREF-Z. Reaching them needs either a
  sensitive document containing such a member or a product hook that attempts
  the source route on a passing case; both are outside this unit's inputs.

## R2c: invented sensitive documents at φ ≈ 2, (3,4,0) and (1,2,2)

The φ = 2 and (1,2,2) documents of the judged set pass the ordinary checks, so
the source route never forms them (R2b). R2c closes that gap with invented
single-member documents modelled on RF PIN-OFF-345 (`twins.py phi2`). The
oracle, `BOUND.md` and `bound.json` are unchanged.

**Layouts.**
- `rf` (RF PIN-OFF-345's own layout): translations pinned at both ends, a soft
  RX spring at N0 and two moments at N1. For a member in the plane z = 0 the
  free rotations form an order-4 block.
- `zhinge`, for members with all three direction components nonzero: under
  the `rf` layout their six rotations form one block, and the product refuses
  it (`UnsupportedBlock { order: 6 }`, kept as the `P122-RF-CONTROL` design).
  In `zhinge`, N0 is pinned with RX and RY restrained; N1 has UZ, RX and RY
  restrained; there is a soft RZ spring at N0 and one RZ moment at N1. The
  rigid rotation about global Z through N0 is held only by the spring. The
  free DOFs RZ0, RZ1, UX1 and UY1 form one block of order 4, and k_θθ' enters
  its RZ0–RZ1 coupling.

**Springs and operands.** k/a = 1e-8 and 1e-12, with a = G·J/L and k rounded
to a power of two. Loads are powers of two, and E is chosen so that
G = E/(2(1+ν)) is exact. With full-mantissa springs and loads, and with a second
(force) load in `zhinge`, the attempts selected but ended
`SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` (`Exact(Budget)` at the
finalization replay, charged 7,999,9xx of 8,000,000). These choices bring them
under the limit.

**φ values** (φ_annular, the exact annulus with κ_E, in rationals; π cancels):

| Design | Layout | Direction, placement | Section | φ_annular − 2 | Declared φ* − 2 (from the payload's leaves) |
|---|---|---|---|---|---|
| P345-NEAR | rf | (3,4,0), origin | SA | +9.98e-16 | +1.09e-15 |
| P345-PLUS11 / -MINUS11 | rf | (3,4,0), origin | SA | +1.00e-11 / −1.00e-11 | +1.00e-11 / −1.00e-11 |
| P345-OFF | rf | (3,4,0), offset (1024, 2048, 16) | SA | +5.78e-12 | +5.78e-12 |
| P122-NEAR | zhinge | (1,2,2), origin | SA | −5.5e-17 | +3.88e-16 |
| P122-PLUS11 / -MINUS11 | zhinge | (1,2,2), origin | SA | +1.00e-11 / −1.00e-11 | +1.00e-11 / −1.00e-11 |
| P122-OFF | zhinge | (1,2,2), offset (100, 200, 300) | SA | +2.42e-14 | +2.43e-14 |
| PEXACT-A, -B | zhinge | (39,9,6), (353,23,8) (the frozen unit's) | SA, SB | 0 exactly | — (Timoshenko not selected) |
| PEXACT-C | rf | (866,1925,0), origin | SC | 0 exactly | +1.32e-16 |
| PEXACT-D | rf | (373,3880,0), origin | SD | 0 exactly | — (Timoshenko not selected) |

SA and SB are the frozen unit's φ = 2 sections. No section was found that puts
φ = 2 exactly on a (3,4,0) or (1,2,2) member: that needs L² = r_o²·Q with Q
a rational square. The search covered ν = n/2^k (k ≤ 12, −1 < ν < ½) and
r_i/r_o = a/b (b < 200), and found none. A planar member only needs Q to be a
sum of two rational squares, which is common. So SC (ν = 1/16,
r_i/r_o = 3/4) and SD (ν = 1/16, r_i/r_o = 2/3) give φ = 2 exactly on skew
members in the plane z = 0, which carry the `rf` layout. Each design has the
zero, mill, ca and both allowances, its Euler–Bernoulli twin, both k/a values
and both modes: 416 runs.

**Selection.** 336 of 416 runs select. Not selected:
- P122-RF-CONTROL (32 runs): `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`,
  `UnsupportedBlock { order: 6 }`. At 1e-12 in `dense_scrutiny`, the
  Euler–Bernoulli twins also end `MODEL_INCOMPLETE`
  (`NUMERICAL_INTEGRITY_UNRESOLVED`).
- PEXACT-A, -B and -D Timoshenko (48 runs): selected, then
  `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` with `Exact(Budget)` at the
  finalization replay. Selection alone charges 4.36M (A) and 4.73M (B) of the
  8M case limit, which the replay then exceeds. PEXACT-D at 1e-12 in
  `dense_scrutiny` also ends `NUMERICAL_INTEGRITY_UNRESOLVED`. Their
  Euler–Bernoulli twins select.

**Results.** **336 of 336 selected cases PASS, 0 FAIL.**
- By layout: `rf` 176, `zhinge` 160. By shear model: Euler–Bernoulli 192,
  Timoshenko 144 (P345 64, P122 64, PEXACT-C 16). Each allowance class 84;
  each mode 168.
- Projections: largest |value − exact|/absolute bound 0.433; largest relative
  error 2.33e-16; smallest endpoint slack 2.49e-16 relative.
- K_local share of the bound, largest per role. Timoshenko: k_axial 0.572
  (N = 2), k_torsion 0.283, k_vv 0.159, k_θθ' 0.144, k_θθ 0.094, k_vθ 0.082.
  Euler–Bernoulli: k_axial 0.572, k_vθ 0.548, k_vv 0.348, k_θθ and k_θθ'
  0.286, k_torsion 0.283.
- **k_θθ' near φ = 2** (absolute bound, N = 18,
  S = (φ* + |2 − φ*|)·EI/(L(1 + φ*))). No declared k_θθ' was exactly zero;
  each design's matrix is the same in every allowance twin and mode:

  | Design | Declared k_θθ' | Exact k_θθ'* | S | Share of bound |
  |---|---|---|---|---|
  | PEXACT-C (φ_annular = 2) | +1.069e-9 | −3.179e-10 (opposite sign) | 4.82e6 | 0.144 |
  | P345-NEAR | −6.777e-8 | −8.316e-8 | 1.53e8 | 0.051 |
  | P122-NEAR | −3.388e-8 | −2.958e-8 | 1.53e8 | 0.014 |
  | P122-OFF | −1.830e-6 | −1.851e-6 | 1.53e8 | 0.071 |
  | P345-OFF | −4.408e-4 | −4.408e-4 | 1.53e8 | 0.010 |
  | P345-PLUS11 / -MINUS11 | ∓7.63e-4 | ∓7.63e-4 | 1.53e8 | 0.042 / 0.032 |
  | P122-PLUS11 / -MINUS11 | ∓7.63e-4 | ∓7.63e-4 | 1.53e8 | 0.060 / 0.070 |

  At φ ≈ 2 the declared entry carries no correct relative digits, and at
  PEXACT-C not even its sign. It still lies well within the absolute bound
  that BOUND.md derives for the cancelling entry.

## Reproduce

```
./run.sh                          # everything: twins, builds, 984 product runs, self-test, oracle, controls, census (about 20 min)
python3 -I oracle.py --selftest   # the self-test alone
python3 -I oracle.py --dump D.json --envelope E.json --request R.json --theory timo --allowance both
```

`run.sh` uses `tools/t3_cargo.sh` (with its own target directory) and
`tools/t3_slot.sh` from the T3 host; `O4DP_T3`, `O4DP_SCRATCH`,
`O4DP_TARGET` and `O4DP_FROZEN` override its defaults. The envelopes come from
the headless runner's `solve` with `--solver-mode`; each is bound to its dump
by the publication commitment and both source hashes, which the oracle checks.
