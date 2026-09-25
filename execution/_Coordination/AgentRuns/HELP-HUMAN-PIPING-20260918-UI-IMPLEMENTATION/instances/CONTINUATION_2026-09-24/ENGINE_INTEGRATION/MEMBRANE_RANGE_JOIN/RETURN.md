# MEMBRANE_RANGE_JOIN — diagnosis and prospective test composition

Disposition: the observed join failure is an ordinary NUM applicability refusal,
not evidence that the bounded PHYS-R4 membrane repair reverted. Preserve the
guard, the original inputs and independent reference. The proposed composition
below is suitable for parent implementation and independent review; it has not
been compiled or executed by this TASK and does not close current range capability.

## Exact refusal boundary

The inherited `ENGINE_INTEGRATION/product-regressions-03.log` records both public
modes returning `MODEL_INCOMPLETE` with blocking
`NUMERICAL_INTEGRITY_UNRESOLVED` / `Range("arithmetic outside normal range")`.
It records 241 library tests and the preceding two extrema plus one grouping
integration tests passing, followed by the two membrane failures. The brief's
"first5publicchecks" wording is not supported by this log: three preceding
integration tests passed. Later integration executables were not reached.

Static trace at the inspected source hashes in `range_probe.json`:

1. `core/product_physics/src/lib.rs`, `solve_load_case`, calls
   `solve_preview_reduced_system` after preparing the ordinary RHS.
2. `solve_preview_reduced_system` calls
   `nonlinear_integration::structural_adapter::AssemblyEvidence::new` before
   calculating free DOFs or dispatching the selected backend.
3. `AssemblyEvidence::new` obtains each represented frame local stiffness and
   orientation and calls `frame_kernel::structural::transform_roundoff`.
4. The original x-aligned member with y-reference `[0,1,0]` has identity `T`.
   Local matrix entries are zero or normal. The first-stage products/sums are
   therefore in the method's normal range.
5. The second-stage allowance loop reaches zero-based `[UY,UY] = [1,1]`.
   With `g = gamma(24)`, both `inherited` and `second` equal `abs(K[1][1])`.
   `checked_value(g * (inherited/(1-g)+second) * (1+64*EPSILON))` rejects the
   resulting nonzero subnormal allowance. That call produces the exact observed
   error text; no solver backend or stress publisher has been reached.

The retained Python static arithmetic trace obtains:

| Quantity | Value |
|---|---:|
| Source wall area | `9.42477796076937839e-154` |
| Source I | `1.17809724509617196e-307` |
| Source J | `2.35619449019234393e-307` |
| Smallest nonzero stiffness magnitude (torsion) | `1.07099749554197451e-307` |
| First rejected allowance's K[1][1] | `1.4137166941154064e-306` |
| gamma(24) | `2.66453525910038280e-15` |
| First rejected allowance | `7.53450109907900980e-321`, binary64 bits `1525` |
| Smallest normal binary64 | `2.22507385850720138e-308` |

For this identity transform and a nonzero entry `k`, the modeled final allowance
is `g*(abs(k)/(1-g)+abs(k))*(1+64*EPSILON)`. Its nonzero normal boundary is
approximately `abs(k) = 4.17535075001861726e-294`; actual boundary classification
uses the rounded expression. This is an allowance range boundary, not the
stiffness representability threshold and not the later wall-force projection.
`checked_value` accepts zero and rejects subnormal/nonfinite values, so the claim
is specifically about this nonzero subnormal allowance; it is not a universal
lower bound for every possible rounded expression.

All-fixed support data cannot bypass the failure because assembly evidence runs
before free-DOF selection. Pressure, stress tolerance and the two solver modes
do not control this particular failure. Confidence is high from the source trace,
the distinct error string and Python arithmetic; no instrumented Rust trace was
performed under this assignment's heavy-lane restriction.

## Concrete proposed repair

`proposed_public.delta.patch` changes only the maintained test's applicability
assertions. The complete prospective file is
`proposed_public_pressure_membrane_range.rs`. Its original `anchor` and `fixture`
functions are byte-identical, and its input remains
`OD=4e-77, wall=1e-77, L=1, E=1, nu=0.1, p=4.7e-170`, both ends fixed, no primitive
mechanical/thermal load. Both public modes must explicitly return
`MODEL_INCOMPLETE`, the expected blocking numerical-range diagnostic, no mechanics
rows, no displacement/stress headline, and no accepted-state mutation.

`proposed_private_tests.rs` is a proposed private root child module. Parent can
install it as `core/product_physics/src/membrane_publication_range.rs` and add:

```rust
#[cfg(test)]
mod membrane_publication_range;
```

The private file carries the original request wiring verbatim. It prepares the
request through existing profile/input/support validation, section resolution,
unit normalization, material resolution, model construction and exact pressure
case construction. It does not call a solver or construct a solved envelope.
The explicit zero mechanical/thermal action vector is the fixed-displacement,
no-primitive-load constitutive premise from the preserved reference. Opposite
endpoint wall actions use the independently projected `f64::from_bits(1)` force.

The two proposed private tests separately exercise the repaired production
callsites:

- `append_exact_pressure_results`: unique case/entity/component/location/unit
  rows at all five stations, wall force bits `1`, membrane reference
  `3.1333333333333337e-171 Pa`, finite outputs, no blocking recovery diagnostic.
  It confirms that rounded-force redivision remains more than 67% away from the
  reference, demonstrating sensitivity to the original publisher defect.
- `exact_straight_summary_extrema`: the actual mechanical-action/pressure-state
  coefficient path, checking witness lower/upper, global upper and the midpoint
  estimator against the same independent reference. Constant axial pressure and
  zero bending justify the same maximum at every station. The tolerance remains
  `1e-9 * abs(reference)` for every stress assertion.

This is lower-level publication/extrema evidence. It does not prove current
public solve success, correct solver-to-publisher action selection, public maximum
row wiring or complete-case summary wiring. Existing admitted public pressure,
geometry, extrema and maximum-coverage controls remain necessary for those
connected behaviors. No historical test mode, ordinary-NUM bypass or exact-boundary
namespace is introduced. Parent should keep this direct-test labeling in the
source and evidence, and obtain independent review of the prospective applicability
disposition together with the final actual diff.

After parent integration and release of the heavy lane, bounded verification is
the product library with filter `membrane_publication_range` and the
`pressure_membrane_range` integration test, followed by the affected admitted
pressure/geometry/extrema/maximum-coverage controls and the remaining scheduled
full-candidate gates. A useful sensitivity check is to separately restore the old
publisher redivision and old pressure-free extrema constant calculation in a
temporary candidate and confirm the corresponding private assertion fails;
restore actual source before final tests. That mutation check is proposed, not
performed. The original historical public red/green remains the real existing
defect/repair runtime witness.

## Preserved provenance and execution boundary

Actual actor `/root/physics_resume/membrane_range_join`, parent
`/root/physics_resume`, delegated-harness-native TASK. No descendants. Full TASK
role and assignment came from this directory's `BRIEF.md`; instruction origins
and SHA-256 hashes are in `range_probe.json`. Read Root/Piping AGENTS,
Piping `loop/LOOP_INIT.md` and the selected `software-defect-diagnosis` skill.
Writes were confined to this directory. No product source writes, Git, Cargo,
Rust, native/browser execution or public-product execution occurred.

`ORIGINAL_pressure_membrane_range.rs` preserves the original test bytes with
SHA-256 `53b461e708f092cd9b35717f9aa242ddfd288e2f742c61456fbf46fa0ec620ea`.
The historical source packet was inspected read-only in the supplied
`/private/tmp/piping-pressure-stress-20260924` checkout:

- `PHYSICS_MANAGER/INDEPENDENT_REVIEW/FREEZE_03_BACKCHECK/independent_arithmetic_probe.json`:
  SHA-256 `23db606adf7fe61f4df6031ac6b11b8285b88b12d6dfe381a04faaabd13c349c`.
- `PHYSICS_MANAGER/MEMBRANE_PUBLICATION/red-02.log`:
  SHA-256 `618e937089db6aeadbca224dc0dbbb0916cca9e43b0c856bc62e6ea55cfd93c3`.
- `PHYSICS_MANAGER/MEMBRANE_PUBLICATION/green-01.log`:
  SHA-256 `4086f6863bef956dce8916ed7a7ce4f305ff21f1532e152bda0cf19576ca0a8c`.
- `MEMBRANE_PUBLICATION/REFERENCE_BINDING_02.json` is the operative corrected
  binding; `MEMBRANE_BACKCHECK/RETURN.md` independently closes the bounded R4
  source repair and historical manager red/green. Earlier red-01 remains a
  separate failed-admission record, not a public stress proof.

Those historical packet paths are currently absent from the joined checkout.
The parent must preserve/integrate their canonical records and chain when it
integrates this disposition; this derivative return is not a replacement.
Manager public execution remains attributed to that manager. Independent Fraction
re-evaluation here confirms the existing reference; the diagnostic Scaled
transcription is implementation tracing and is not used as the physics oracle.

Run `python3 probe_range.py` from this directory to reproduce `range_probe.json`.
Observed exit 0; reference/hash assertions passed, projected force bits remained
`1`, normal local stiffness assertions passed, and first nonzero subnormal
allowance was `[1,1]`. `OUTPUT_HASHES.json` binds the delivered files. Rust source
drafts have only been inspected statically; compile/runtime/review remain parent
work, and any later source changes require affected binding/check refresh.

A final unchanged-source assertion failed because `lib.rs` and the maintained
`pressure_membrane_range.rs` had changed concurrently after the diagnostic
snapshot; the parent was notified. All other inspected source/instruction/log
hashes still matched. `draft_static_checks.json` records both identities. The
original `range_probe.json` and proposed patch basis are preserved rather than
silently rebound. These subsequent bytes are not reviewed by this diagnosis;
the parent owns implementation and candidate verification. The parent subsequently
confirmed by harness message that it applied `proposed_public.delta.patch`, copied
the private module to `src/membrane_publication_range.rs`, and added the
`cfg(test)` module declaration. It reported no test execution while NUM owns the
heavy lane. That is parent-reported integration, not a runtime or independent
review result from this TASK.

## Remaining numerical capability

No range capability repair is proposed here. Admitting the unchanged public
fixture would require an independently reviewed NUM method that represents or
scales the small transformation allowances with justified error accounting,
then checks every subsequent applicable range boundary and the actual public
case. Dropping or flushing the allowance, skipping the gate for all-fixed models,
changing E/geometry/pressure, or bypassing NUM would not establish that capability.
Source-block recovery remains separately owned and qualified. This test
composition neither completes the physics/group undertaking nor qualifies
source-load/subnormal-force forward accuracy, solver conditioning, engineering
fitness, native behavior, release or downstream reliance.
