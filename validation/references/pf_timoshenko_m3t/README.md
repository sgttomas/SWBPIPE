# PF M3-T: certificate soundness carried to the pressure-free route

This directory holds the judged-set generator, the exact reference engine, the M3 comparison with
its PF clauses, the premise-census wrapper and a small frozen subset for M3-T carried to the
pressure-free (PF) route's opt-in Timoshenko straight member: DEL-05-03 Design
[pf-timoshenko.md](../../../execution/PKG-05/DEL-05-03/Design/pf-timoshenko.md) §5, "Certificate
soundness (M3-T, carried to PF)", by the method of
[section-bases-and-timoshenko.md](../../../execution/PKG-05/DEL-05-03/Design/section-bases-and-timoshenko.md)
§4 item 5. The acceptance is the K-D5 M3 criterion of
[kd5-member-relative-bound.md](../../../execution/PKG-04/DEL-04-01/Design/kd5-member-relative-bound.md)
§4.3. It is the PF counterpart of [`../kd5_timoshenko/`](../kd5_timoshenko/README.md) (the exact
route's M3-T), whose tooling it reuses by import, unchanged. Standard claim fence applies (F-PIP-2;
claims taxonomy per DEC-081). Every model is invented.

**Status: built, self-checked and frozen by its author TASK (stage A), then refuted in round 1 by a
second TASK (Codex; Plan 01 §5). No product output has been compared with an expected value of this
set; the judging is stage B.** The refutation recomputed 16 models independently at 140 digits
(3,287 rows and 35 maxima, including Codex's case and an expected refusal) and found every frozen
value within its 45-digit rounding. It found 0 admission mismatches over the 906 models and the 41
ulp placements correct. It refuted the judging workflow, repaired here; no frozen value moved:

- a duplicated run record could replace an earlier failure, and missing or unexpected runs were
  invisible. Both tools now check the collection inventory (below);
- admission (C10) is a failing design-conformance verdict, separate from M3;
- the README counted 7 auxetic elements at φ = 2 exactly; there are 6.

**Round 2** (Codex, at `81f11c6be8`) reproduced the frozen bytes and checks, found exactly six φ = 2
elements, and confirmed C11ops against §§2.3–2.4 on 2,040 σ± and 930 τ rows. It refuted the
aggregation. Both findings are repaired here; no frozen value moved:

- the report trusted chunk sidecars, so a typed/captured disagreement split across comparisons
  disappeared. The report now recomputes the inventory and the typed/captured pairing over the whole
  collection, and sidecars are cross-checks only;
- cached evaluations from before the repair hid admission failures. Every record and sidecar is now
  stamped. `run.sh` and the report refuse or recompute a stale evaluation, and an absent
  `design_failures` fails.

Round 3 superseded these repairs (below).

**Round 3** (Codex, at `c4b7bac39b`) found the values standing (19,566 rows, 9 refusals, `1a6984ec…`,
130/720/11 checks) and round 2's reproductions closed, and refuted the judging again: stored pairing
operands could be forged, missing or null clause fields passed, a cached evaluation stayed reusable
after a request or runner change, and runner errors passed (kd5t_compare returns no failure for any
`error` key, null included). On Agent 0's direction the architecture changed rather than the cache
being hardened a third time; no frozen value moved:

- nothing intermediate is read back. `run.sh` writes raw runner outputs only, fresh on every
  invocation. One judging pass, `pft_report.py`, judges the whole collection directly from those raw
  outputs and the reference every time; evaluation records are an output, never an input. The
  evaluation cache, its stamps and sidecars, and `--check-eval` are gone;
- every raw record carries the sha256 of the request line it solved, the runner binary's sha256 and
  the product head it was built from. The pass fails a run of any other request under the same tag,
  and a collection with mixed runners or heads;
- a runner error (any `error` key, null included), a run with no outcome and a schema-invalid record
  fail C0 and the inventory; they are never skipped;
- the raw-output and reference schemas are validated exactly, so older formats and evaluation records
  are refused as input;
- a final `OVERALL:` line, with exit 0 only on PASS.

**Round 4** (Codex, at `b81867df9d`) found the values standing (19,566 publications, 9 refusals,
`1a6984ec…`, 737/130/12 checks), every round-3 reproduction closed and all 28 genuine probe records
valid, and refuted the judge's totality. Repaired here, on the principle that the judge is total and
fail-closed; no frozen value moved:

- **Outcomes.** Exactly one standing per load case, from a closed recognised set, or exactly one
  recognised refusal code; zero, several or unknown fail on schema. Before, the first code won, and a
  contradictory or fabricated code passed.
- **Strict JSON.** Duplicate keys, NaN and Infinity are refused in every JSON read: raw outputs, the
  reference, and the twin control's report in `run.sh`.
- **An established head.** `--product-head` (40 lowercase hex), `--runner-sha256` and `--checkout`
  are mandatory, and the head must resolve with `git rev-parse --verify <head>^{commit}`.
  - `run.sh` builds the runner itself from that checkout and passes the binary's sha256 and the
    head.
  - `build.sh` refuses a dirty build input: `core/`, `fixtures/`, `schemas/` and `validation/` (the
    runner sources and `Cargo.lock`). `apps/` is not linked.
- **Outputs never destroy inputs.** `--eval-out` and `--json` are created exclusively. A path that
  exists, or that aliases an input through a symlink or a hard link, is refused before anything is
  written.
- **The twin control** counts only from a completed report with provenance and inventory PASS and
  M3 FAIL. The driver's final line is `OVERALL: FAIL` whenever its exit is nonzero, a runner exiting 7
  after valid records included.
- **Totality.** Every nested field the judge reads is validated (summary headline, contract evidence,
  rows' model binding), and the rest is documented as unread. The reference is fully typed. An
  unforeseen exception is a named `OVERALL: FAIL`, never a traceback.
- **Regressions.** Every attack from rounds 1–4 is a committed regression case (below).

**Independence.** The engine, generator, comparison and census are standard-library Python. They
import and read no product code. They import `kd5t_engine.py`, `kd5t_generate.py`, `kd5t_compare.py`
and `kd5t_census.py` from `../kd5_timoshenko/` (read-only, no bytecode written there), which are
themselves product-independent. The inputs were:

- the designs: pf-timoshenko.md (all), section-bases-and-timoshenko.md §§1, 3 and 4, K-D5
  §§2, 3.2, 3.3 and 4.3, DEL-04-07 `u4b-u5b-implementation.md` §§2.3 and 2.4 (the normal-stress
  extremes and the Saint-Venant transverse shear), DEL-04-01 `global-end-actions.md` (the global end
  rows), and SHEAR_REFERENCE `CONTRACT.md` at `f2211ecef6`;
- `../pf_timoshenko/README.md` and its committed requests, for PF's request spelling and premises;
- `../kd5_timoshenko/` (README and tooling);
- the product's published rows on **Euler–Bernoulli probe models** (below), used only to establish
  the public format, units, sign conventions and section basis, never expected values. One
  Timoshenko probe was read for its envelope's format (kinds, diagnostics) only; its values were
  not read.

Expected values come only from the engine's own arithmetic. The runner (`runner/`) is harness, not
reference: it links the product crate by path and calls only `run_linear_static_preview_with_mode`
(typed) and `run_linear_static_preview_value_with_mode` (captured).

## Files

| File | Role |
|---|---|
| `pft_engine.py` | Exact PF reference engine: a PF request in, every expected published row out (110-digit `decimal`; ν, κ_E and φ exact rationals). `--check` runs its self-check (130 checks; one of them runs kd5t_engine's 58). |
| `pft_generate.py` | Judged-set generator: `full <dir>` writes the set, `freeze` the frozen subset, `split` runner inputs, `probes` the format probes, `precision` a re-solve at more digits, `composition` the counts, `check` verifies everything. |
| `pft_compare.py` | The judging library: kd5t_compare's reconstructed judge with a single-section adapter, the strict raw-output and reference schemas, the request binding, the inventory, the design-conformance clause C10 and the derived-row clauses C11, C11tau_b and C11ops. `--selftest <set>` exercises every rule and clause; `--probe-format` prints the format facts of the probes. |
| `pft_report.py` | The one judging pass: the whole collection of raw runner outputs judged against the reference, total and fail-closed, with kd5t_report's tables, the four verdicts (provenance, inventory, M3 and design conformance) and the final `OVERALL:` line. |
| `pft_census.py` | Premise census: kd5t_census with its engine bound to the PF reference (`hager`, `probe`, `standings`, `--selftest`). |
| `runner/` | The product-driving runner (harness): `Cargo.toml`, `Cargo.lock`, `build.rs`, `src/main.rs` (kd5's, plus each record's provenance) and `build.sh`. |
| `run.sh` | Driver: builds the runner from a clean checkout, runs it over every chunk (raw outputs only, fresh each time), then the judging pass. |
| `frozen/pft_frozen_subset.jsonl` | 100 models (50 designs in both theories), each line `{tag, meta, ref, request}`, with exact expected rows. |
| `SHA256SUMS` | Digests of the files above, except this README. |

## The reference engine (`pft_engine.py`)

- **Request.** Model document 0.8.0 with no `pressure_contract`. `model_settings.shear_model` absent
  or `euler_bernoulli` gives Euler–Bernoulli; `timoshenko_annulus_energy_v1` makes every straight
  member Timoshenko (pf-timoshenko.md §1). Every binary64 input is taken exactly.
- **PF's section basis (premise).** One section for every use: the annulus r_o = OD/2,
  r_i = OD/2 − (t − c_m), with A, I, J = 2I and Z = I/r_o exact (π by Machin). It was established
  from the design text and confirmed on the probes:
  - pf-timoshenko.md §2: "PF's stiffness section, computed by PF's existing arithmetic
    (`derive_pipe_section`): r_o = OD/2 and r_i = OD/2 − (t − c_m), with A its area";
  - `../pf_timoshenko/README.md`: "PF's one effective-wall section for every use";
  - the probes: stress rows carry no `section_basis`, and on probes with c_m > 0 every row
    (stiffness, stresses, maxima, τ) agrees with this annulus to 6e-16 (below).

  PF's own section arithmetic is binary64. The engine refuses a model whose binary64 t − c_m is
  inexact, and the generator only authors exact differences (c_m = t − fl(t − c₀), exact by
  Sterbenz). What remains is the product's ulp-level area arithmetic, as on the exact route.
- **Moduli and ν.** E and G are authored independently; EA and EI use E, GJ the authored G. An
  authored `poisson_ratio` is not consumed. ν = E/(2G) − 1 exactly.
- **κ_E** is SHEAR_REFERENCE's energy-matched annular factor of that ν on PF's annulus, in the
  moduli form of pf-timoshenko.md §2: with m = r_i/r_o, a = 1 + m², b = 1 − m² and t = 1 − 2G/E,
  κ_E = 6a²/((7 + t²)b² + 48m²). It is an exact rational, and the self-check proves it equal to
  kd5t_engine's (E, ν) form and to the CONTRACT closed form on a grid of 6 annuli × 9 values of
  E/(2G) down to 1e-160.
- **φ** = 12EI/(κ_E·G·A·L²) = 3E(r_o² + r_i²)/(κ_E·G·L²), an exact rational (I/A = (r_o² + r_i²)/4),
  with L² exact from the binary64 node coordinates.
- **Element, loads, supports and recovery.** kd5t_engine's, unchanged: the 12×12 frame (SHEAR_REFERENCE
  §3), the consistent full-span uniform vector (both theories), the held thermal pair ∓E·A·α·ΔT,
  end rows f = K·d − feq (node-on-element), j-side station cuts at 0, .25, .5, .75 and 1, the
  continuous circular maximum |N/A| + hypot(M_y, M_z)/Z, and the whole-body balance check on every
  solve. PF additions:
  - `guide` supports restrain their listed DOFs like anchors;
  - `imposed_displacement` primitives prescribe the named support DOF (W-16, partitioned
    elimination);
  - `weight` and `distributed_force` primitives are full-span intensities.
- **Derived rows** (published by PF; "no certificate", u4b-u5b-implementation.md §2.3, and
  "receive no certificate of their own", global-end-actions.md):
  - `element_global_end_force_v1` and `_moment_v1`: F = Rᵀf, M = Rᵀm of the end rows;
  - `element_local_normal_stress_extreme`: σ± = σ_N ± hypot(σ_y, σ_z) of the station's stress rows;
  - `element_local_transverse_shear_stress_maximum`: τ = |V|·c,
    c = ((3 + 2ν)r_o² + (1 + 2ν)r_i²)/(4(1 + ν)I), formed without 1 + ν (1 + ν = E/(2G)); V is the
    section shear at interior stations and the end-force shear at the ends. A member has τ rows
    only where PF's binary64 ν_d = RN(RN(E/RN(2G)) − 1) lies in (−1, 1/2); otherwise the case is
    expected to carry `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN` (§2.4).
- **Admission (design §2).** A Timoshenko document is expected **refused** with
  `PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE` when ν_d is not finite or outside (−1, 1/2), which is
  E/(2G) ≤ 2^-54 or ≥ 3/2. Each refused member also lists the binary64 representability failures of
  the moduli-form factor in the design's order (t·t overflow, D or κ_E not normal, A_s = κ_E·A,
  G·A_s, (G·A_s)·L² or φ not normal). Exact rows are frozen for these models as well, so that any
  publication can be judged.
- **Self-check (`--check`, 130 checks).**
  - kd5t_engine's self-check (58 checks, counted as one): SHEAR_REFERENCE S-A, S-B, S-C and S-E, rigid modes, pure curvature, cut(L) = f_j
    and the balance check with its negative control.
  - The moduli form against the (E, ν) and CONTRACT forms (108 grid points); t4_u8's
    κ_E(9/10, 3/10) = 16609827/33068756 from E and G = E/2.6.
  - The pf_timoshenko S-B annulus: ν = 23/77 exactly, and κ_E within 1e-15 of 1965660000/3913460969.
  - The auxetic control: ν = −5/9, κ_E = 30/49 and φ = 2 exactly.
  - Codex's case (below), against the guided-cantilever closed form and the energy form.
  - The admission edges: E/(2G) = 2^-54 refused, the next G below admitted; E/(2G) = 3/2 refused,
    the next E below admitted; t·t overflow named.
  - Euler–Bernoulli and Timoshenko tip deflections against their closed forms; the PF section with
    c_m > 0, and the refusal of an inexact t − c_m; the τ constant's two forms; the global end rows;
    an imposed displacement.
- **Codex's case, reproduced.** OD 0.2 m, wall 0.01 m, L 1 m, fixed root, tip restrained except UY,
  E = 1 GPa, G = 6755399441055744 GPa (E/(2G) = 1/(3·2^52) ≈ 7.4e-17, admitted), tip force 1e-18 N
  (binary64). φ = 2.69469524665317494e12 and
  **UY = 8.3139297980274503558e-12 m**, which is Codex's independent energy value
  8.31392979802745e-12 m to all its 15 digits. It equals F·L³(1 + φ)/(12EI) and
  F·(L³/(12EI) + L/(G·A_s)) to 1e-90.
- **Precision.** `precision` re-solves the near-cancellation, near-Sensitive and φ = 2 families
  (366 models, 57,120 rows) at 200 digits. The worst difference from the frozen 45-digit rows is
  3.9e-46 of the case's largest value of the same unit.

## The judged set (`full`)

- **Size.** 906 models: 453 Euler–Bernoulli (member absent) and their 453 Timoshenko twins.
- **Section variants.** `m0` (no mill tolerance) 782; `cm` (c_m ≈ 0.125·t, t − c_m exact) 124, on
  every third design of each family and two φ = 2 designs, at the origin and skewed. PF has no
  corrosion allowance.
- **Placements.** 448 at the origin, 188 at a UTM-scale offset (4.3e5, 7.3e6, 218 m) and 270 skewed
  at (5e6, 7.3e6, 1.2e6 m) (kd5_timoshenko's offsets and rotation).
- **Families** (models, both theories):

  | Family | Models | Content |
  |---|---|---|
  | Cantilevers | 164 | tip loads, self-weight and contents, uniform loads, thermal, tip and interior springs |
  | Fixed-fixed runs | 120 | point loads, self-weight, thermal, interior springs, prescribed motions of the far anchor (all six DOFs) |
  | L-lines with sharp corners | 104 | thermal, self-weight, tip and corner loads, a grounded spring, a guide, prescribed motions |
  | U, Z and 3D lines | 76 | thermal, self-weight, prescribed motions |
  | Skew multi-span runs | 76 | grounded UY and UZ springs, guides, lateral uniform loads, thermal, a prescribed motion |
  | φ = 2 cluster | 64 | element lengths 0 to ±3 ulps from the φ = 2 length; a two-element run and an L-line at it |
  | Near the Sensitive boundary | 132 | soft torsion and translation springs against stiff members, short members (10–50 mm between 4 m members, 4–30 mm corner legs) |
  | Near-cancellation | 170 | below |

  Self-weight and contents are authored `weight` intensities w = g(ρ_s·A + ρ_c·π·r_i²) on PF's
  section (ρ_s 7850, ρ_c 998 kg/m³, g 9.80665 m/s²).
- **φ (Timoshenko twins).** Outside the near-cancellation family, 1,155 elements from 1.0e-6
  (9.9994e-7) to 5.0e4: per decade from 1e-7 to 1e4, 3, 24, 55, 88, 98, 149, 229, 259, 161, 68, 12 and 9. The φ = 2
  cluster has 22 elements within 2.4e-15 of 2 at the origin and 10 within 2.0e-9 skewed; the
  auxetic designs add 6 at exactly 2 (the guided, free, two-element and fixed-fixed designs at the
  origin). The skewed free auxetic element is 1.99999999674…, because its rounded binary64
  coordinates move L. The exact φ and φ − 2 are in each line's `ref.phi` and
  `ref.phi_minus_2`.
- **The near-cancellation family** (Agent 0: r1's defect published 55.6% low while marked Passed).
  E = 1 GPa and G authored, so E/(2G) is exact from the two binary64 values. Unless stated, a
  guided cantilever: fixed root, tip restrained except UY (one free DOF), OD 0.2 m, wall 0.01 m
  (m = 9/10), L = 1 m, with nodal loads in all six directions and a uniform UY load.

  | Group | E/(2G) | Designs | Timoshenko expectation |
  |---|---|---|---|
  | Codex's case (tip force 1e-18 N only) | 1/(3·2^52) | 3 sections (m = 9/10, 1/3, 0.995), and skewed | rows |
  | Ordinary | 1.3, 1, 0.65, 0.5, 0.3 | 5 (+2 sections at 1) | rows |
  | Decades | 1e-1 … 1e-16 | 16 (+2 sections and skewed at 1e-16) | rows |
  | Below the admission edge | 1e-17, 1e-20, 1e-50, 1e-100, 1e-154 | 5 | refused (ν_d = −1) |
  | Beyond representability | 1e-200; G = 1e300 Pa (5e-292); G = 1e308 Pa (fl(2G) = ∞) | 3 | refused (ν_d; t·t overflow, D, κ_E, A_s, G·A_s) |
  | 2G/E within ulps of 1 (ν ≈ 0) | G = E/2 ± 0, 1, 2 ulps | 5 | rows |
  | Sterbenz edge E/(2G) = 1/2 (G = E) | ± 0, 1, 2 ulps | 5 (+2 sections) | rows |
  | Domain edge E/(2G) = 3/2 | G = E/3 ± 0, 1, 2 ulps | 5 | 3 refused (ν_d ≥ 1/2), 2 rows |
  | Admission edge E/(2G) = 2^-54 | G = 2^53·E ± 0, 1, 2 ulps | 5 (+2 sections) | 5 refused, 2 rows |
  | t·t overflow edge E/(2G) ≈ 7.458e-155 | ± 0, 1 ulp | 3 | refused (ν_d; A_s, or t·t, D, κ_E) |
  | Free and two-element cantilevers (6 and 12 free DOFs) | 1.3, 0.5, 1e-3, 1e-6, 1e-9, 1e-12, 1e-16 | 14 (+1 skewed) | rows (conditioning grows as 1/x) |
  | Auxetic φ = 2 point: OD 9/32, wall 3/32, L 7/32, E 8 GPa, G 9 GPa (ν = −5/9, κ_E = 30/49) | 4/9 | guided, free (and skewed), two-element, fixed-fixed | rows |

  19 Timoshenko models are expected refused. Their 19 Euler–Bernoulli twins are expected to solve,
  with no τ rows and `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN`.
- **Not in the set.** Realized arcs (out of scope), combinations (K-D5 judges case rows, not
  combinations, K-D5 §3.3), nonlinear supports, W-15 rigid components, generated self-weight and
  temperature-selected moduli.

## The frozen subset (`freeze`, `check`)

100 models, 50 designs in both theories: cantilevers 10, fixed-fixed 8, L-lines 8, U/Z/3D 6, skew
multi-span 6, φ = 2 cluster 8, near-Sensitive 8 and near-cancellation 46. They are 70 at the
origin, 4 at UTM and 26 skewed, and 6 with c_m > 0. 9 Timoshenko models are expected refused,
including Codex's case's neighbours at the admission edge and every representability group. That is
19,566 expected rows and maxima. `check` regenerates the subset in memory and compares it byte for
byte, runs the engine self-check and the comparison self-test, and verifies `SHA256SUMS`.

## The comparison (`pft_compare.py`)

**What PF publishes, from the format probes.**

- The same certified row kinds, units and signs as the exact route: global displacements (mm) and
  rotations (rad), `displacement_magnitude` (mm), the element-local forces and moments at five
  locations, the element-local axial, bending and torsional stresses (MPa), the support components
  and two magnitudes, the circular maximum `pipe_elastic_normal_stress_maximum_v2` (Pa) with
  `pipe_stress_extrema` evidence (`value_lower_pa`, `value_upper_pa`) in `contract_evidence`, and the
  `max_open_formula_stress` headline.
- **One section family.** Stress rows carry no `section_basis`. kd5t_compare's single-family
  producer (`pre-u6`: unsuffixed stress rows on one section) is exactly PF's case, so the comparison
  uses it, with PF's section under that producer's lookup key.
- **The standing and the gate values.** `NUMERICAL_INTEGRITY_CHECKS_PASSED` or `_SENSITIVE`, whose
  message carries the gate's `reciprocal_condition_estimate`, `scale_exponents` and
  `global_dof_map`, as on the exact route.
- **PF-only derived kinds**: the global end rows, σ± and τ_v,max (above).
- **Not published**: any group bound B, the Hager estimate est, ‖K̃_s‖₁ or ε_E.

**What can and cannot be judged.**

- **Can.** The K-D5 M3 criterion, through the decision, as on the exact route: a Passed case asserts
  that no judged group fired, so a judged group whose actual error exceeds RD(1e-9)·q refutes
  B ≥ error there. The coverage of the circular maxima (strict, exact). Every certified row kind
  above, typed against captured, both modes. A1's Hager ratio, from r̂ and the scale exponents with
  the engine's exact free-DOF stiffness (`pft_census.py hager`).
- **Cannot, from published values.** The bound B itself (B/err margins), and ε_E/‖K̃_s‖₁ against
  2^-40. These need an instrumented product build, as the exact route's census did
  (`../kd5_timoshenko/README.md`, "ε_E census, measured"); `pft_census.py probe` consumes such a
  dump unchanged.
- **Judged, but as information.** The derived rows carry no certificate, so they are not part of
  C1. C11 judges them against the ideal target at their body's coupled scale, and C11ops against the
  design's prescribed binary64 operations on the published operands, where the design prescribes
  them:
  - σ± are checked **bit for bit**: u4b-u5b-implementation.md §2.3 fixes the sequence
    h = norm2(σ_y, σ_z) (correctly rounded), RN(σ_N ± h), on the same station's published rows
    ("Every reader can recompute this bit for bit");
  - τ_v,max is checked to 2^-40 relative against §2.4's fixed sequence on the station's published
    shear rows and the model's E, G, OD, wall and c_m. The sequence includes ν_d = RN(RN(E/RN(2G)) − 1)
    and RN(1 + ν_d). Only I_b's binary64 order is not prescribed, so the exact I rounded once
    stands in for it;
  - the global end rows are checked against the ideal target only, because the rotation's binary64
    order is not prescribed.

**The judging pass** (`pft_report.py <set.jsonl> <raw output.jsonl>... --runner-sha256 <hex64>
--product-head <hex40> --checkout <dir>`). It reads only the reference set and the raw runner outputs,
and judges the whole collection in one invocation, every time. It may chunk internally for speed, but
only from the raw outputs. All of a model's records are judged together, so C4 and C4b come from the
canonical published fields of the raw records. `--eval-out` and `--json` write outputs, created
exclusively and never aliasing an input; nothing reads them back. Four verdicts follow, then
`OVERALL: PASS` only if all four pass, with exit 0 only then. The judge is total: anything it cannot
fully validate, including a usage error or an unforeseen exception, ends in a named `OVERALL: FAIL`.

1. **Provenance** (refutation rounds 3 and 4).
   - Every JSON input is read strictly (`pft_compare.strict_loads`): a duplicate key, NaN or Infinity
     is refused.
   - The reference set is validated exactly, every field the judge reads typed: line, `meta` and `ref`
     keys, sections, node coordinates, the case rows and maxima, the request's nodes, pipes, supports
     and load cases, and the twins when judging against them. With an invalid reference nothing is
     judged.
   - Every raw record must match `pft_compare.RAW_SCHEMA` exactly. That shape is taken from the probe
     outputs, and all 28 genuine probe records validate and bind. It covers:
     - required keys, types and no unknown keys;
     - the nested fields the judge reads (the summary headline; the contract evidence's cases and
       extrema);
     - exactly one outcome: one standing per load case from `STANDINGS` (DEL-07-05
       `numerical-health-indicator.md` §1's table; only `CHECKS_PASSED` is Passed), naming its case,
       or blocking diagnostics of exactly one code from `REFUSALS` (the PF refusals pf-timoshenko.md
       names) and no standing. A refusal publishes no rows; a standing publishes rows;
     - its binding to its model: rows of the model's cases and nodes, pipes or supports (the
       `linear_solver_mode_basis` row names `solver:linear_static_preview`), and evidence of its cases
       and pipes.

     `numerical_quality`, the summary's other members, and the evidence's `intensified_measures`,
     `stress_maximum_coverage` and `support_attribution` are not read. Older-format records and
     evaluation records are refused.
   - The runner records on every run:
     - `request_sha256`, the sha256 of the exact input line `{tag, request}` it solved. This must equal
       the sha256 of `pft_compare.request_line` formed from the reference request, so a changed request
       under the same tag fails;
     - `runner_sha256`, its own executable's sha256;
     - `product_head`, the product commit baked in at build.
   - `--runner-sha256` (64 lowercase hex), `--product-head` (40 lowercase hex) and `--checkout` are
     mandatory. The head must resolve with `git rev-parse --verify <head>^{commit}` in the checkout,
     and every record must carry exactly that runner and head. The pass prints both. These record
     fields are assertions of the local build, bound by `run.sh`; they are not authentication (see
     "Threat model").
2. **Inventory** (refuter P1). Every model × entry × mode exactly once, as a successful, schema-valid,
   bound record, and nothing else. The entries are typed and captured, and the modes
   SparseInteractive and DenseScrutiny. That is 3,624 runs for the full set and 400 for the frozen
   subset.
   - Unsuccessful records fail the inventory and C0; they are never skipped. They are: a runner error
     (any `error` key, null included, or an error beside a result, which is schema-invalid), and a
     schema-invalid or misbound record, which includes a run with no outcome or contradictory
     outcomes.
   - A run identity received more than once is rejected before judging: none of its records is judged,
     so a later correct record can never replace an earlier failure. A record of a model outside the
     set fails C0.
3. **M3** (the certificate): kd5t_compare's acceptance clauses C0–C4, C6, C8 and C9 (see
   `../kd5_timoshenko/README.md`, "The comparison and the M3 verdict"). It passes only with provenance
   and the inventory. The clauses apply to every run, whatever the admission expectation, so a Passed
   publication on a model the design refuses is judged against its exact rows like any other.
   kd5t_compare's own `judge_run` returns no failure for a run with an `error` key. The PF judge never
   passes such a run to it, and kd5's directory is not edited. By the criterion (K-D5 §4.3), a
   collection whose every run is Sensitive judges no rows and passes M3 (`bad_rows_sensitive_only`).
   The report prints the standings.
4. **Design conformance** (admission, separate from M3): C10, pf-timoshenko.md §2. It fails on an
   expected refusal that publishes or is refused under another code, on an admitted model that is
   refused, and on any unjudged record. It also requires provenance and the inventory.

Information clauses: kd5t_compare's C3lit, C4b, C5, C6i and C7, and:

| Clause | Reported | Source |
|---|---|---|
| C11 | On Passed cases: a derived group (global end force or moment per end, σ± pair, τ_v,max) beyond RD(1e-9) of its coupled scale in the row's unit (SF; SM; SM/Z; SF·c) and not the known τ defect; an expected derived row unpublished; the τ-domain warning missing or unexpected (a τ row where none is expected is C6i) | own; u4b-u5b-implementation.md §§2.3, 2.4; global-end-actions.md |
| C11tau_b | On Passed cases: a τ_v,max group beyond the ideal target's scale that equals §2.4's prescribed operations. This is the **known defect on main**: RN(1 + ν_d) cancels as E/(2G) → 0, so τ is off by up to the relative error of RN(1 + ν_d), a factor of 1.5 at Codex's case. Agent 0 has ruled it a separate correction, option (b) | Agent 0's ruling; §2.4 |
| C11ops | On Passed cases: σ± not bit for bit the prescribed operations, or τ_v,max not within 2^-40 of them | u4b-u5b-implementation.md §§2.3, 2.4 |

C11 uses the coupled scale, not each group's own q, because σ⁻ = σ_N − h and τ are binary64
functions of certified rows and may cancel legitimately.

**`--selftest` passes 775 checks on the frozen subset.** On ten models (Euler–Bernoulli and
Timoshenko cantilevers, a prescribed-motion run, an L-line with a guide, a skew multi-span with guides
and springs, a c_m > 0 model, a skewed UTM cantilever, Codex's case, the skewed auxetic model (φ ≈ 2)
and a two-element cantilever at E/(2G) = 1e-12) it checks:

- the reference published as itself, with σ± and τ formed by the prescribed operations, passes every
  clause. Its only information is C11tau_b, raised exactly on the models where RN(1 + ν_d) is off by
  more than 1e-12 (here Codex's case and E/(2G) = 1e-12);
- CRIT pinned from both sides (1 ± 1e-5) on the largest judged group of every group kind (all 14 PF
  kinds pinned);
- C3 pinned from both sides on translations and rotations;
- F1 (an all-zero publication), F2 (a null and an infinite value; a null evidence upper);
- C2 and C7 (an upper bound 0.1% low, a low headline, missing evidence);
- C6 (dropped station rows; a dropped row), C6i (an unknown row; a basis-suffixed stress row);
- C8 (no rows; a gross support moment);
- C9 (a second value of one identity in four orders; the same on a Sensitive run; a same-value
  repeat; a second maximum);
- C4 and C4b (typed/captured);
- C5 and the design-conformance verdict (a refusal of an admitted model fails C10; M3 passes);
- C11 from both sides (1 ± 1e-5 of the coupled scale) on each derived kind, and an unpublished
  derived row.

Beyond those ten models it checks:

- **r1's defect**: Codex's case with UY published 55.6% low on a Passed run fails C3. Its forces are
  φ-independent (a guided beam's end moments are −FL/2 for any φ), so only the displacement catches
  it;
- the sensitivity control: a φ = 12 Timoshenko model judged against its Euler–Bernoulli twin fails;
- rotation noise in an exactly zero rotation field (a guided tip) passes C3 and raises C3lit; an
  all-zero displacement field published as zeros passes and as noise fails C3;
- a second body (C0);
- three expected refusals, refused by name (pass), under another code (C10 fails design
  conformance), publishing their exact rows (C10 fails, M3 passes) and publishing wrong rows as
  Passed (C3);
- an out-of-domain Euler–Bernoulli twin: no τ rows and the warning (clean), the warning missing
  (C11), and a τ row present (C6i);
- the known τ defect and the operations: on Codex's case, τ by the prescribed operations is C11tau_b
  (not C11 or C11ops), and τ at the ideal target is C11ops (not C11); σ⁻ one ulp off the operations
  is C11ops on two models; the correctly rounded norm2 on hard cases;
- **every refutation round's attacks (1–4) as committed regression cases**, 101 in all, each printing a
  `TABLE round | attack | case | outcome` line. They run through the judging pass on raw synthetic
  records; through `run.sh`, with a synthetic builder whose runners are Python scripts binding their
  records to each input line, their own sha256 and the checkout's head, so no product can run; through
  `build.sh`, on a throwaway repository with cargo set to `false`; and through the CLI. The head is the
  self-test checkout's real `HEAD`. They comprise:
  - the positive controls, which pass: a small collection, the frozen subset (400 runs), two raw
    files, reordered and escaped keys, the product's solver-mode row, run.sh with and without the twin
    control, recovery from a partial or stale earlier output, and the CLI;
  - every attack, which fails for its stated reason (inventory, schema, provenance, M3, C4,
    outputs refused, or run.sh's exit and final line);
  - `forged_identical_provenance` and `bad_rows_sensitive_only`, which pass: the first is outside the
    model (see "Threat model"), the second is the criterion.

**Controls.** `--ref twin` judges each run against its twin theory's reference. The Timoshenko
product against the Euler–Bernoulli reference must fail at high φ. The stale-vector mutant is not
observable here, as on the exact route: every member load is full span, where the two vectors are
identical, and PF refuses Timoshenko partial spans by name (pf-timoshenko.md §3).

## Premise census (`pft_census.py`)

kd5t_census with its engine bound to pft_engine. `hager` gives κ₁(D·K·D)·r̂ from the published r̂
and scale exponents with the exact PF free-DOF stiffness (at most 72 free DOFs). `standings` gives
Passed, Sensitive, fail-closed and refused counts by shear model, with the EB → Timoshenko moves.
`probe` gives the ε_E figures from an instrumented build's dump. Its self-test passes on the format
probes (6 models, 0 failures), and on those probes diag(D·K·D) lies in [1, 4), confirming the
mapping.

## Runner and probes (`runner/`, `pft_generate.py probes`)

The runner is `../kd5_timoshenko/runner/` plus each record's provenance (refutation round 3): a
standalone crate (its own `[workspace]` root and `Cargo.lock`) whose one product dependency is
`../../../../core/product_physics`. `sha2` is the only added dependency, already in the product's
locked graph. Each record carries:

- `request_sha256`, the sha256 of the input line it parsed and solved;
- `runner_sha256`, its own executable's sha256;
- `product_head`, the product commit, which `build.sh` passes to `build.rs` as `PFM3T_PRODUCT_HEAD`.

`build.sh <checkout> <target dir> <binary out>` builds the checkout's own committed runner with
`--locked`; cargo is `$PFM3T_CARGO` (default `cargo`). It refuses (exit 3) when any build input is
dirty or untracked in git:

- `core/`, which holds the product crate and all its path dependencies;
- `fixtures/`, `schemas/` and `validation/`: files the product embeds, plus the runner sources and
  its `Cargo.lock`.

The runner links nothing under `apps/`. `PFM3T_ALLOW_DIRTY=1` builds anyway with the head marked
`-dirty`, which the judging pass refuses.

The round-3 runner was built through the host's cargo wrapper at `c4b7bac39b`, whose product is
#1663's `8d70bcceee` unchanged. On its 28 probe runs every record is schema-valid under the round-4
schema and binds to its model, and every `request_sha256` equals the judging pass's recomputation.
Inputs are lines `{tag, request}`, written by `pft_generate.py split` as `pft_compare.request_line`
forms them. `KD5_IN`, `KD5_OUT`, `KD5_ENTRIES` and `KD5_MODES` control a run.

`run.sh <checkout> <set dir> <run dir> own [twin]` proceeds as follows:

- it builds the runner itself from the checkout (`runner/build.sh`, or `$PFM3T_BUILD`), computes the
  binary's sha256 and reads the checkout's head;
- it empties `<run>/out` and runs the runner on every chunk, so it writes only raw outputs and reuses
  none;
- it runs the judging pass per variant with the runner, head and checkout. `report.<v>.json`,
  `report.<v>.log` and `eval.<v>.jsonl` are outputs.

It exits 0, with the final line `OVERALL: PASS`, only when the build and every runner invocation
succeeded and the own judgement passed. If a twin control is requested, its report must also have
completed with provenance and inventory PASS and M3 FAIL; the report is parsed strictly. Otherwise the
final line is `OVERALL: FAIL`.

**Probes.** Seven invented models outside the judged set, in both entries and both modes (28
runs): six Euler–Bernoulli probes and one Timoshenko probe read for format only. Every run was
Passed. The comparison of the Euler–Bernoulli rows was restricted to format:

- a two-element cantilever with a tip load in all six directions, a uniform load, self-weight and
  contents, at the origin and skewed at UTM scale, each with c_m = 0 and c_m > 0;
- an L-line with an anchor, a guide, grounded UY and RX springs, a thermal load and a prescribed
  six-DOF motion of the far anchor, at the origin and skewed.

What they established:

- **Format.** The row kinds, units and coordinate systems listed above, and the gate fields in the
  integrity message.
- **Units, signs and kinds.** Every probe row agrees with this engine's Euler–Bernoulli rows to at
  most 6e-16 of its kind's scale (`--probe-format`). That covers translations in mm, rotations in rad,
  element-local stresses in MPa and the maximum in Pa; node-on-element end rows; j-side stations;
  support-on-pipe reactions; the global end rows; σ±; and τ_v,max with ν = E/(2G) − 1.
- **PF's single section** t − c_m, on the c_m > 0 probes.
- **Envelope format.** The Timoshenko probe's envelope has the same kinds and diagnostics, by format
  only.

## Threat model

Inputs are the committed runner's output on a clean build at the stated head; the judge fails closed on malformed, contradictory, duplicated, missing or misbound input. It is not authentication against a rebuilt, lying runner.

### Outside the model

Findings that a later round places outside this model, each with its reason:

- **Raw hash and head fields are assertions: identical forged values pass** (round 4,
  `forged_identical_provenance`). A record's `runner_sha256` and `product_head` are what the runner
  says. The judge binds them to `--runner-sha256` and a head that is a commit of `--checkout`, and
  `run.sh` binds those to the binary it built from that clean checkout. A party who writes records
  with the same forged values, and states them, is a lying runner.

## Not covered

- Realized arcs (an Euler–Bernoulli arc between Timoshenko tangents), out of scope for this set.
- ε_E and the bound B, which PF does not publish (above).
- Combinations, nonlinear supports, constant-effort hangers, W-15 rigid components, generated
  self-weight from densities, temperature-selected or interpolated moduli, and the
  authored-ν warning. These are covered physically by `../pf_timoshenko/`, not by this judged set.
- Admission at the representability boundary within a few ulps: the generator places those models
  decades from the backstops except where stated. Every backstop lies below the 2^-54 edge, where ν_d
  already refuses.
- PF's binary64 section arithmetic (`derive_pipe_section`) is not reproduced bit for bit. The exact
  annulus differs from it by ulps, far inside RD(1e-9).
- Anything outside the threat model (above).

## Reproduce

```
python3 -I -B pft_engine.py --check
python3 -I -B pft_generate.py check                  # self-check, frozen bytes, compare self-test, SHA256SUMS
python3 -I -B pft_generate.py full <set> 8
python3 -I -B pft_generate.py precision <set>/set.jsonl
python3 -I -B pft_generate.py split <set>/set.jsonl <run>/chunks all 40     # or: eb, ts
./run.sh <clean checkout> <set> <run> own twin         # builds the runner, raw outputs in <run>/out, the judging pass per variant
python3 -I -B pft_report.py <set>/set.jsonl <run>/out/c.*.out.jsonl --runner-sha256 <hex64> --product-head <hex40> --checkout <dir> \
        --by theory --phi-table                         # the same pass by hand; add --ref twin for the sensitivity control (must fail M3)
python3 -I -B pft_census.py hager <set>/set.jsonl <run>/out/c.*.out.jsonl --out <census.jsonl>
python3 -I -B pft_census.py standings <run>/out/c.*.out.jsonl
python3 -I -B pft_generate.py probes <probes.in.jsonl>   # then the runner, then:
python3 -I -B pft_compare.py --probe-format <probes.in.jsonl> <probes.out.jsonl>
```

`freeze` regenerates the references for an intended change of them, never to absorb a product
result.
