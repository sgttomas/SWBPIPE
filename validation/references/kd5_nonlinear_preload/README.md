# K-D5 nonlinear references under the variable spring preload law

This package supplements the frozen package `../kd5_nonlinear`, which stays unchanged. In
`../kd5_nonlinear/u1_spring_request.json`, a one-way support and a variable spring hanger act on the same
degree of freedom (node C, UZ). Its frozen oracle `../kd5_nonlinear/u1_spring_oracle.json` models the hanger as
a bare spring of its entered rate and leaves out the entered cold load. This package gives the expectations
under the variable spring law in DEL-04-03/REQ-014 and DEL-04-11/REQ-014. Under that law, the hanger is a spring
of rate k plus its cold load F_cold, a constant force on the structure along +UZ in every case. Its published
support force is F_cold − k·u, and each case also publishes F_cold as the support's applied force.

The frozen spring row is −k·u (`reference.py`, `published`: `rows[d % 6] = -k * u[d]`). In the frozen LIFTED
case it is −40.013… N for an upward displacement of +4.0013e-4 m. Like the one-way reaction `r = K·u − f`, it is
a force acting on the structure. F_cold − k·u extends that convention.

**Axis and sign of the cold load.** The cold load acts along the positive axis of the hanger's declared stiffness
DOF (+UZ here). The positive axis is the integrator's decision for both hanger families, read from DEL-04-03
REQ-014's convention for the constant-effort support and DEL-04-11 REQ-014's "through the constant-effort seam";
the words "positive axis" enter the spring's own sentence in both scopes with the preload landing.

This package is independent of the product and not of the frozen package: frame formation, state enumeration
and classification are the frozen generator's (`../kd5_nonlinear/reference.py`).

No maintained product test reads this package yet, because main does not yet implement the preload law;
the preload landing (DEL-04-11 REQ-014) points the product tests at it. Until then, its own check and the
tests of the reference-package table (`tests/reference_packages.py`) are what maintain it.

## Files

| File | What it is | Supplements |
| --- | --- | --- |
| `reference_preload.py` | Generator. It imports the frozen `../kd5_nonlinear/reference.py` for frame formation, enumeration, exact solve, classification and published rows. It adds only the cold load and the plain-spring record form. It reads no product code or output. | — |
| `u1_spring_law_oracle.json` | Item 1: the frozen `u1_spring_request.json`, unchanged, under the law. Each case carries `changed_from_stiffness_only`, which gives each changed quantity with its new value and the frozen stiffness-only value read from the frozen oracle. It also states the basis and the entered cold load. | Replaces `../kd5_nonlinear/u1_spring_oracle.json` as the expectation for `u1_spring_request.json` under the preload law. The frozen oracle remains the stiffness-only result. |
| `u1_spring_twin_request.json` | Item 2: the frozen request with the hanger record replaced by a plain spring record. The replacement has the same id, node, stiffness and DOF, and carries no hanger member or loads. Everything else is identical, and the generator asserts this. | Carries the original control: the converged active set with a parallel spring, pressed and lifted. Under the law, `u1_spring_request.json` no longer exercises it. |
| `u1_spring_twin_oracle.json` | Its oracle. The generator asserts that its PRESSED and LIFTED cases equal the frozen oracle's cases exactly, by computation. | Can take the place of `../kd5_nonlinear/u1_spring_oracle.json` for that control. |
| `u1_preload_request.json` | Item 3: the frozen request with the hanger and its cold load. Its two cases are PRESSED-PRELOAD (−1000 N in UZ at C) and LIFTED-PRELOAD (−200 N in UZ at C). | New control: a pressed one-way support beside a preloaded hanger. |
| `u1_preload_oracle.json` | Its oracle. | — |
| `SHA256SUMS` | Digests of the five generated JSON files. The generator writes it. | — |

The plain spring record follows the form used by committed requests such as
`../w16_nonlinear_motion/requests/b1_spring.json`, `../rt_a3_s6/requests/ls1_c5.json` and
`../t4_u8/requests/S-E-SPRING-EULER-BERNOULLI.request.json`. Its `family` is `"spring"`. Its `restraints` is
`[its DOF]`, which names the spring axis and is not a rigid restraint. Its `stiffness` is `{dof, value}` (schema
`$defs/Support`, `$defs/SupportStiffness`).

Each oracle keeps the frozen oracle format (`strict_states` with `standing`, `states`, `groups` and `signed`, plus
`weak_states`), so a test can compare it as it compared the frozen one. Each strict state adds the following:

- `margins`: the one-way support's DEC-067 measure, exact and as binary64 bounds. This is the reaction when the
  support is active and the lift when it is inactive.
- `hanger_force`: F_cold − k·u.
- `applied_force`: F_cold.
- `displacements`.

Each case adds `enumerated`, which lists every support state with its standing and its measure.

## Results

| Oracle | Case | Admissible state (strict, unique) | Margin | The other state |
| --- | --- | --- | --- | --- |
| law | PRESSED (−100 N) | one-way lifted | lift 1.3204e-3 m | active: reaction −330 N |
| law | LIFTED (+100 N) | one-way lifted | lift 2.1207e-3 m | active: reaction −530 N |
| twin | PRESSED | one-way active | reaction 100 N | inactive: lift −4.001e-4 m |
| twin | LIFTED | one-way lifted | lift 4.0013e-4 m | active: reaction −100 N |
| preload | PRESSED-PRELOAD | one-way active | reaction 570 N | inactive: lift −2.2808e-3 m |
| preload | LIFTED-PRELOAD | one-way lifted | lift 9.203e-4 m | active: reaction −230 N |

The generator fails unless every case has exactly one admissible state, that state is strict, and no state is
weak. It also checks that, in every case, the support forces exactly balance the case's own loads.

No oracle states an iteration count. Neither the frozen oracle nor `reference.py` states one, and this derivation
does not establish one. The twin's assembled system (K, f, support states and initial state) equals the frozen
request's exactly. Any premise that rests on that system therefore holds for the twin for the same reason it holds
for the frozen request.

## Regenerate and check

From this directory:

```sh
python3 reference_preload.py --check   # the default: regenerates into a temporary directory and compares every output byte for byte
python3 reference_preload.py --write   # replaces the outputs here
```

The generator reads `../kd5_nonlinear/reference.py`, `u1_spring_request.json` and `u1_spring_oracle.json` and
writes nothing there. It also turns off bytecode writing before importing them.

Membership: the files in the table above. `SHA256SUMS` lists the five generated JSON files. `README.md` and
`reference_preload.py` are outside it. Every oracle also binds its request by `request_sha256`. Item 1 binds the
frozen request, and binds the frozen oracle under `supersedes` or `reproduces`.
