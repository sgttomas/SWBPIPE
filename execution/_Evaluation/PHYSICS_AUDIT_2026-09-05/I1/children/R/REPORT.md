# Recovery, combinations, and numerical publication audit — I1-R

## Basis and method

Frozen source: `2be412ccea62bdc4bd96deb082c46d7a792076ea`; accepted decomposition 0.12 / SCA-009 / DAG-010 as supplied by parent. This derivative evidence packet implements the sealed I1-R brief and evidence-write supplement. It does not amend decomposition, adopt a numerical policy, or authorize engineering reliance. TASK Agent 2; no children; actual model not exposed; nondelegation instruction+config asserted.

The production subject was read-only. Reviewed the product adapter's solve-to-recovery-to-publication path, stress recovery, result algebra consumption, analysis-run wrappers, native/headless seams, typed export validation, and frontend result interpretation. Source hashes are bound in `_run_records/SOURCE_BINDING.json`; grouped function/branch coverage and remaining gaps are in `COVERAGE.csv`. Existing complete Rust suite evidence is owned by V1, reported by the manager as 971 tests / 37 manifests passing; it was not rerun or independently certified here. Important existing tests were inspected for oracle independence and current-tier assumptions.

Independently authored dimensional/free-body oracles were frozen before execution in `EXPECTED_BEFORE_RUN.json` and `EXPECTED_SUPPLEMENT_BEFORE_RUN.json`. V1's accepted early criteria corroborate them; no expected scalar or tolerance was fitted to an observed result. Numerical differences are compared against the known six-decimal serialization grid only to distinguish gross discrepancies from rounding; that grid is not an engineering acceptance threshold.

Executed 40 current-source product calls (20 fixture files × two solver modes, including deliberately preserved invalid fixture drafts), 14 actual provisional headless CLI calls, and three actual Python analysis-run wrapper calls. Initial headless calls without the explicit local-private flag correctly blocked exposure. The second seven calls used the flag for these agent-authored invented inputs; six exited successfully, overflow exited one. Every exposed headless mechanics envelope exactly equals the corresponding product envelope. Native GUI was not invoked; the thin driver calls the same public function and deserialization shape as the native command, which establishes shared code reachability, not actual packaged GUI execution. No physics WASM route exists in the inspected browser fallback.

## Confirmed findings

### I1-R-01 — Selected nonlinear state does not drive ordinary recovery (W2, high)

`solve_load_case` recovers ordinary nodal components, reactions, maxima, pipe forces, and stresses from the linear vector. `append_nonlinear_support_loop_results` separately solves and publishes selected support rows but returns no selected vector to its caller. The independent zero-gap axial example converges with support displacement zero and reaction −350 N, yet the ordinary node reports 0.000989 mm and pipe end actions ±350 N. Both ends have zero displacement in the selected state, with the load balanced at the gap; the elastic pipe should carry no axial deformation force in this simple model.

The unchanged prior investigation fixture, rerun against current source, reports ordinary UY at N140 = 0.332485 mm versus nonlinear UY = 0; ordinary UZ at N130 = −3.530824 mm versus nonlinear UZ = −5.469167 mm. Removing nonlinear supports leaves all 784 comparable non-nonlinear, non-mode result values unchanged. This count includes review/evidence quantities and is not a claim of 784 independently assessed physical responses. The small gap example is the direct analytical witness; the larger fixture is integration corroboration. Both modes reproduce the issue; actual headless CLI propagates the simple witness.

Repair boundary: propagate one explicitly selected state through every ordinary consumer, including constant-effort movement warnings, summary maxima, component modifiers and combinations. Preserve linear/nonlinear evidence as separately identified diagnostic states if useful. No convergence policy change is implied.

### I1-R-02 — Straight station interpolation mixes opposing end-action conventions (W5, high)

For pure 350 N axial force, 350 N m torque, and 350 N transverse tip force, the appropriate constant interior resultant must have magnitude 350 throughout an unloaded straight span. Product quarter/mid/three-quarter rows instead give −175, 0, +175. Axial and torsional stresses likewise vanish at midspan. Both solver modes reproduce this, and the headless CLI carries the axial error.

The cause is direct interpolation between the two nodal action vectors, whose opposing signs describe action on opposite element ends. The product must convert to one section convention before interpolation or equilibrium recovery. This is an incorrect constant-field representation even under the documented interpolation tier; it does not require adopting arbitrary station locations or exact distributed diagrams. The existing `tp_phys_008_thermal_pressure_axial_effects.md` independently declares constant axial station force despite opposite endpoint actions, corroborating the contract mismatch. Fixed thermal restraint also emits endpoint ±120 MPa but midspan zero.

### I1-R-03 — Linear combinations add nonlinear magnitude summaries (high)

Two equal/opposite transverse load cases sum to zero signed displacement components but a `displacement_magnitude` of 0.812046 mm. Their `reaction_resultant` sum is 1565.247584 N while the net loading cancels. Each component row is correctly added; the derived norm is incorrectly treated as another linear scalar. Open-formula stress summaries are explicitly excluded, showing an existing analogous guard, but displacement and reaction magnitudes are not excluded. Both modes and actual headless CLI reproduce the inconsistency.

This is about a mechanics **linear sum** of compatible signed states. A range/envelope selecting a magnitude from independently solved states may be meaningful and must not be indiscriminately removed. Repair by recomputing derived magnitudes from combined components where complete, or withholding those derived rows with a clear diagnostic. Reaction component availability and the separate reaction-dimension defect owned by C must be resolved together. The generic algebra crate faithfully performs scalar arithmetic; the product adapter owns quantity semantics.

### I1-R-04 — Nonfinite publication can still claim mechanics solved (high robustness)

A finite authored axial load of 1e308 N produces eight null numerical result values after Rust nonfinite values pass through `round6`/serialization. The mechanics status remains `MECHANICS_SOLVED` with no blocking mechanics diagnostic. Both modes reproduce this. This is a robustness counterexample, not a realistic piping design case or a proposed numerical threshold.

The actual headless wrapper catches invalid typed-export values and exits one with `HEADLESS_RUNNER_RESULT_ENVELOPE_PRODUCTION_FAILED`, while its embedded mechanics envelope retains the solved/null contradiction. That downstream protection is real and must not be omitted from the finding. The manager independently found a transverse version; consolidate rather than double count. A bounded fix must reject nonfinite numerical output before solved status and before any combination `expect` relying on finiteness.

### I1-R-07 — Alternate-case derived-result sources point to the default case (medium provenance)

The actual current larger fixture contains 20 alternate-load-case component multiplier source links whose target belongs to the first load case. For example, the L200 component-C110 multiplier references L100 end-j axial/bending stress IDs. Product code qualifies each later-case result's ID but not its embedded `source_result_refs`. The result values need not be wrong for their trace to be wrong. `SOURCE_REF_CHECK.json` records every mismatched edge. Remap same-case source references along with result IDs, then validate referenced basis identities before export.

## Reproduced limitations and conflicts requiring calibration

### I1-R-05 — Straight distributed-load response remains a lumped/interpolated approximation (W5)

For q=100 N/m and L=2 m, the product reports midpoint shear 0 N and moment −100 N m; free-body section magnitudes are 100 N and 50 N m. Root/tip recovered shears are −100/+100 N instead of the continuous-load end actions −200/0 N. Tip displacement is 0.116007 mm versus Euler–Bernoulli 0.087005007466... mm (the oracle file carries the unrounded value).

Source explicitly uses 50/50 nodal force shares, no fixed-end moments, and endpoint interpolation. Project records declare this approximate tier and defer exact internal diagrams. Therefore distinguish the confirmed constant-field sign defect above from this documented approximation: the observation does not itself authorize a new distributed-load formulation. The parent must map upgrade requirements or obtain an Owner decision for consistent loads and section equilibrium. The lower straight-pipe APIs already expose richer recovery, but kernel existence is not product integration. `partial.json` is a preserved duplicate uniform control, not evidence of partial-span execution; partial primitive input is not exposed by this preview schema, while partial generated-wind transfer is owned by C.

### I1-R-06 — Pressure force/stress meaning conflicts across existing evidence (high concern, disputed physical basis)

Under an independently stated closed-end, free axial pipe interpretation, pressure 1 MPa over internal diameter 0.154 m produces pAi = 18626.502843 N of wall tension, requiring nonzero longitudinal wall stress. The actual product extends by 0.052609 mm but reports zero axial force/stress everywhere and suppresses the separate pressure-longitudinal rows.

The source combines pressure thrust and thermal axial corrections in `corrected_local_forces_for_axial_effects`, then suppresses longitudinal pressure whenever thrust is active. DEL-05-03 MEMORY says the suppression is appropriate **when represented through axial-normal stress**. However, the existing TP-PHYS-008 hand calculation explicitly subtracts pressure equivalent loads like thermal eigenloads; that is evidence of a conflicting established force-reference convention, not a clean universal oracle. The curved macro-span uses a different complete pressure system and recovers wall tension.

Preserve this as a reproduced physical-basis conflict, not an unqualified solver-kernel defect. Before repair, explicitly decide whether straight reported resultants include pressure wall tension, how end-cap load transfer is represented, and where longitudinal stress enters. Reconcile the hand calculation and trace the same meaning through recovery and summary. No unilateral new pressure formulation is authorized by this packet.

### W7 — Reused labels are real; whole-contract identity is richer

Changed length and material values reuse `run:preview-linear-static-001` and the project reference while changing numerical results. The provisional headless CLI also uses a fixed request/run label, but binds path-based request metadata and result checksums. The frontend wrapper verifies a current-session input manifest, records its exact hash, and hashes result values/envelope; Python wrapper experiments confirm supplied model-bound input hashes and result hashes distinguish the three cases.

Consequently, a constant run label alone does **not** prove that the whole application cannot distinguish models. Raw mechanics envelopes lack an exact input/settings hash and durable model-state identity; current wrappers partly supply that context. Full durable run-history semantics remain a known integration gap. The Python experiment uses an explicitly audit-supplied fixture digest, not a claim that it generated the authentic frontend InputManifest.

## Additional review dispositions and gaps

- Curved recovery subtracts arc-consistent distributed/radial wall loads, transforms chord end actions to global, then evaluates arc section equilibrium. Those paths avoid the straight interpolation method. Existing tests commonly reuse the same curved kernel for expected outputs; independent arc formulation verification belongs to M1/V1. This packet does not independently validate every arc case.
- Stress kernel formulas N/A, My/Zy, Mz/Zz, Tr/J and optional thin-wall membrane components are explicit. Missing/nonfinite inputs empty components and suppress summary. Product stress values are recovered at full precision and rounded for publication, but combinations consume already-rounded rows: small-component cancellation or large factors can amplify quantization. No new precision policy or exhaustive bound was established here.
- Stress summary adds absolute bending components, excludes torsional shear from its scalar normal-stress summary, and considers only fixed endpoints/three stations. It is not a von Mises or true continuous extremum. Summary maxima at envelope level represent the first load case, not all cases. These definitions require clear consumption contracts; neither a new stress criterion nor all-case envelope is adopted here.
- Temperature/modulus records are carried; mechanics combinations reject incompatible bases through upstream validation, while result-state subtraction and range envelopes preserve operand records. Source inspection plus existing tests cover these gates; no new independent mixed-temperature numerical witness was run by R.
- Nonlinear state-metadata differences can cause combination row mismatch; convergence failures are warnings in the loop appender and actual mechanics status can remain solved. Kernel cycling/convergence policy is owned by M1; this packet establishes the publication seam and does not adopt a convergence threshold.
- Component modifier rows distinguish geometric-review sif×flexibility from macro-bend sif-only behavior; actual same-case reference remapping remains defective as recorded above. Stress derivation is not independent validation of user-entered factors.
- Frontend `invokeOrFixture` swallows native invocation errors and may return the bundled solved fixture for an exactly unchanged example; edited models block. Async job fallback exposes browser-fixture status. This is a static provenance concern requiring a separate native-error injection check before claiming user-visible misrepresentation; no such GUI test was executed here.
- Typed result export validates finite values and metadata before the headless publication succeeds; serialization itself is a separate public function and does not implicitly validate. Result interpretation displays supplied values, basis and source links without recalculating physics. Storage persists the caller payload; full immutable history is outside this audit slice. No user database was touched.

## Recommendations and decision queue

First repair selected-state propagation, constant-field section signs, magnitude-combination semantics, nonfinite publication, and same-case source traces, each with frozen independent regressions and whole-diff review. Coordinate reaction recovery with C. Keep pressure interpretation and any upgrade from lumped to consistent distributed loads as explicit design/requirement decisions, preserving the unresolved existing evidence. Review rounding bounds and fallback provenance separately after those blockers; do not dilute high-priority defects into an aggregate score.

## Handoff and closure

R's bounded baseline investigation is complete with explicit gaps; engine correctness is not established. Required outputs are present, original failed/schema-draft/blocked runs are preserved, expected values predate their actual calls, and no production source was changed. Manager fan-in and fresh whole-baseline review remain required. All reports are derivative of the named source/authority basis. Re-run unchanged witnesses and source-ref checks after repairs; rebuild actual headless/native consumers for integration changes. No lifecycle, scope, professional acceptance or engineering hold is closed.

## Live deliverable calibration supplement

The current DEL-05-03, DEL-05-05 and DEL-04-02 `_STATUS.md` files all remain IN_PROGRESS and have empty Remaining sections; an empty section is not acceptance or a declaration that integration is complete. DEL-05-03 ScopeOfWork AC-001 requires preserving sign/station handling and VER-001 requires resultant-to-component/station parity. Its current CLM declarations (lines 63, 225 and 326–328) explicitly leave final service/envelope ownership and production integration unresolved, assigning station generation upstream. Thus constant-field sign repair maps to an existing correctness obligation, while complete product diagram integration remains residual.

DEL-05-05 ScopeOfWork OUT-001 and R7 (line 175) cover full/partial-span oriented straight-pipe equivalent nodal recovery in the lower user-load module. The same current document (lines 54, 159 and 222–223) explicitly leaves final result-envelope/API/persistence/GUI/CLI/report integration TBD. It supplies an existing capability and potential implementation route, but is not evidence that the product currently consumes it or that a new integration tranche is already closed. DEL-04-02 status records bounded producer/export binding and preserved runtime/policy residuals. Manager should activate the owning existing integration obligation with a concrete source/acceptance brief rather than infer a new scope amendment from the observed approximation alone.

Typed producer coverage was completed through every production function family in `result_envelope_binding.rs` (lines 1–584). The mapping explicitly discloses unsupported kinds/units rather than coercing them, and validates both typed and serialized envelopes. Its `model_hash` field currently carries the **runner request metadata checksum**, not the actual model bytes. Result-envelope checksums remain separate. It also emits empty trace chains instead of converting product `source_result_refs`; those refs remain in the carried mechanics envelope. These are provenance limitations of the typed subset, not a claim all metadata disappeared. The direct/native envelope and current-session frontend wrapper have distinct contracts and must not be conflated.
