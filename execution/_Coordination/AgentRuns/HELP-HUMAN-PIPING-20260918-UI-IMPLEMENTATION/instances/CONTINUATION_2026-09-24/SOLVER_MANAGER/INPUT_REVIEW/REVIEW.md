# Independent input-contract review

TASK `/root/solver_manager/input_review` returns to `/root/solver_manager` using delegated-harness-native execution. This is fresh-context review by an agent that did not implement the candidate. Source review is complete; the frozen source was handed back to the manager before any subsequent repair. Only this INPUT_REVIEW evidence directory was written. No OS isolation, model diversity, owner review, native witness, engineering acceptance, or release is claimed.

**Disposition: not suitable for manager fan-in until R1–R3 are repaired and independently backchecked.** The existing positive changes for M04, M21 and M35 are well targeted; the support validation needs to follow actual assembled semantics. V1 is missing verification rather than a demonstrated numerical defect.

## Frozen candidate and scope

Base: `f7e8b467cb2db244f11fe49cede636140031b387`.

| Reviewed file | SHA-256 |
|---|---|
| `projects/chirality-piping/core/product_physics/src/lib.rs` | `1477faebbad58c5437382c835d25c2e7cb2feeaf24e2b2a165a5ecdd77af47b0` |
| `projects/chirality-piping/core/product_physics/src/validation.rs` | `d74e1d453ac902b6fd2a40edb5a69d1924672185194cdbaffedbef0024fdf611` |

Both hashes matched INPUT_CONTRACTS/CANDIDATE_RUN.json. SOURCE_DIFF.patch preserves the complete two-file diff reviewed. New validation benchmark Cargo dependency/tests belong to the manager's future M24 integration and are explicitly outside this review. All locations below refer to the frozen candidate, not a later repair.

## Confirmed actionable findings

### R1 — P1: constant-effort applied-force axes are mistaken for rigid constraints

Location: `projects/chirality-piping/core/product_physics/src/validation.rs:77–84` (the unconditional co-located support restraint comparison).

Trigger: a valid `constant_effort_support` and a separate nonlinear gap/lift-off/one-way support share a node and translational DOF. For example, use the existing constant-effort cantilever fixture with `restraints=["UY"]` and its valid constant-load/travel/source metadata, then add a distinct tip gap acting on UY. These are an applied force plus contact, not incompatible prescribed displacements.

Impact: new validation emits blocking `SUPPORT_LINEAR_NONLINEAR_DOF_CONFLICT`, so an otherwise valid, previously supported load/contact model returns MODEL_INCOMPLETE with no results.

Evidence: `build_model` explicitly excludes constant-effort records from linear boundary construction at lib.rs:3523–3524. `classify_constant_effort_consumption` (lib.rs:8361 onward) uses their single declared translational restraint as the force direction; `constant_effort_solve_dispositions` excludes only records carrying their own nonlinear field, and `add_constant_effort_support_loads` is used in the shared nonlinear force-vector path at lib.rs:1303. Existing `constant_effort_coexists_with_nonlinear_supports_and_nonlinear_field_precedence` covers coexistence, but not co-located same-axis coexistence. The new validator compares the raw field without this classification.

Remedy: derive overlap from actual rigid-boundary/spring semantics and exclude constant-effort force axes from rigid overlap. Add a co-located same-axis constant-effort/contact positive control, retaining explicit consumed-load evidence and contact equilibrium; keep genuinely conflicting rigid overlap negative controls.

### R2 — P1: separate positive springs in parallel with contact are unnecessarily blocked

Location: `projects/chirality-piping/core/product_physics/src/validation.rs:79–84` (both spring axis representations are treated as conflict).

Trigger: a supported positive linear spring is a separate support record from a nonlinear gap at the same node/DOF. The two forces act in parallel and remain separately attributable. This is distinct from placing two laws on one support identity.

Impact: new validation blocks a valid system already handled by the assembled solver, including a spring that deflects before a gap closes. It narrows the accepted model contract unnecessarily.

Evidence: `core/solver/nonlinear_integration/src/lib.rs:438–459` adds the spring diagonal to the active-set stiffness. The active contact prescribes its entered clearance at lines 1386–1421, and lines 1507–1511 recover reactions from that augmented stiffness. Product publication uses `-k*u` for the separate spring row (lib.rs:1472–1480) and the nonlinear residual for the contact row (1482–1494). An independent axial control with the existing 1 m annulus fixture, E=200 GPa, k=1 MN/m, F=100 kN and gap=0.05 mm gives u=0.05 mm after closure, root reaction -35405.749206 N, spring reaction -50 N and contact reaction -64544.250794 N. These sum to -100 kN. Before contact, u=0.141020853 mm, so closure is physically required. REFERENCE_CALCULATIONS.json records the arithmetic; no solver was run by this reviewer.

Remedy: preserve separate spring/contact coexistence while rejecting actual incompatible rigid constraints. Keep same-record overlap blocked until its support-law and combined reaction semantics are intentionally supported; simply deleting every overlap check is not sufficient. Add inactive and active gap states and signed equilibrium/component checks in both modes.

### R3 — P1: nominal spring family can still bypass the ignored-stiffness guard

Location: `projects/chirality-piping/core/product_physics/src/validation.rs:43–46` (`spring_family` does not follow the constant-effort precedence used by the builder).

Trigger: a valid constant-effort hanger payload with `hanger.hanger_type="constant_effort_support"`, `family="spring"`, a positive top-level stiffness (e.g. UY, 100000 N/m), and matching UY restraints. The support configuration form exposes family, hanger type and stiffness independently, so this contradictory saved record is reachable, not merely an impossible Rust object.

Impact: the new validator classifies this as a consumed spring and produces no unused-stiffness diagnostic. The builder then treats it as constant-effort and drops the entered spring stiffness. The model can solve using only its constant force, leaving the M19 silent-loss class open.

Evidence: `is_constant_effort_support` uses `support_hanger_type`, which prefers the hanger type over family (lib.rs:8268–8290). `build_model` returns None for all such records at 3523–3524, before its spring constructor. `validate_spring_hangers` validates the constant-effort hanger's own type/required fields but never crosschecks `family="spring"`; the new `spring_family` expression is nevertheless true solely from that family. `SupportConfigurationForm.tsx:194–249` supplies all these independently editable fields. This is a pre-existing contradictory-classification hole directly left open by the new M19 guard, not claimed as a new regression introduced by it.

Remedy: reject contradictory effective-family/hanger combinations at solve readiness or use one consistent classification shared by validation and construction. Do not clear or reinterpret authored fields silently. Add this conflicting payload as a blocking negative control and retain ordinary spring, variable hanger and constant-effort positives.

## Required verification action

V1 — `lib.rs:15363–15417`: new typed moments are tested only on an X-aligned member. All three signed axes have independent analytical values, so these are stronger than alias comparisons, but they do not meet the requested coordinate-rotation check. The existing rotated uniform-wind test at lib.rs:16161 exercises translational distributed load, not the new typed moment route.

Add a skew member, e.g. tangent `n=(1,2,2)/3`, L=2 m, a non-collinear explicit orientation vector, and one or more moments authored through `rotation_x/y/z`. For the circular annulus the independent global oracle is `theta = L*((M-(M·n)n)/(EI) + (M·n)n/(GJ))`; tip translation is `u = L²/(2EI)*(M × n)`. REFERENCE_CALCULATIONS.json supplies one numeric companion. Check global component signs/magnitudes, zero resultant force, and root/member moment equilibrium within the available result contract. Current scalar force-reaction rows cannot establish full signed moment-reaction publication; that programme limitation must remain explicit.

Other evidence limits: the provenance test compares an untouched pre-solve clone with its original string, so that assertion alone does not prove persistence; native edit/save/reopen remains a separate outstanding witness. No schema/persistence code is changed in this diff. The guide/contact positive control checks active clearance but not its combined force resultant under simultaneous Y/Z load; backchecking repaired support classification should include attribution/equilibrium. These are coverage limitations, not additional confirmed implementation defects.

## Reference qualification and current-source check

The end-couple reference independently follows equilibrium and curvature: no transverse force means zero shear, constant moment; integrating curvature gives signed rotation ML/EI. Circular torsion gives TL/GJ, with exact annular `I=pi*(Do^4-Di^4)/64` and `J=2I`. The test's dimensions are consistent, its implied isotropic Poisson ratio is 0.2987, and its approximately 0.0002175 rad bending and 0.0002825 rad torsional rotations are in the small-rotation regime. E and G are explicit synthetic inputs, not claimed material-standard values. The 1e-6 rad assertion permits result-envelope rounding (at most 0.5e-6 rad); it has not replaced or weakened an existing protected threshold.

Current primary-source retrieval on 2026-09-24 checked [TU Delft's Timoshenko equations](https://interactivetextbooks.citg.tudelft.nl/computational-modelling/structural_linear/timoshenko_md.html), [TU Delft's circular-shaft torsion summary](https://ocw.tudelft.nl/course-readings/summary-key-formulas-torsion-2/), and [MIT's Roylance shear/torsion derivation](https://ocw.mit.edu/courses/3-11-mechanics-of-materials-fall-1999/0e0845a9e3abe430080eaffb0c5015ba_MIT3_11F99_torsion.pdf) (document dated June 23, 2000; publication age is not a claim of recent theory). These are primary teaching sources, not the prohibited unreviewed OCR corpus.

Comparison/inference: Timoshenko admits transverse shear deformation but its constitutive relation gives zero shear strain when this control's shear force is zero, leaving the rotation oracle unchanged. The circular annulus has the circular-section torsion solution without noncircular warping corrections; the MIT derivation specifically integrates over inner/outer radii. Current higher-fidelity shear, shell/ovalization, geometric/material nonlinear, or dynamic formulations do not supersede this bounded linear static direction-ingestion control merely by being more general. They remain necessary for claims outside its envelope. This is a bounded applicability check, not an exhaustive literature review or a pressure/code-stress validity claim.

## Checks, evidence, and limits

- Read the full Root AGENTS, TASK role, Piping AGENTS, LOOP_INIT and software-code-review skill from primary checkout; byte equality with implementation-worktree copies was verified. Actual origins/hashes are in ORIGINS.json. No other full role instructions were activated.
- Reviewed the complete frozen diff and traced affected validation, builders, shared load assembly, nonlinear spring/constraint handling, support/result publication and authoring field reachability.
- Scoped `validate_change_scope.py` passed for the explicitly selected two-file subset; it does not certify the manager's other working files. `git diff --check` passed for those two files.
- Reconstructed the preserved tests-only baseline in memory from Git base plus PRE_FIX_TESTS.patch. The reconstructed lib.rs SHA-256 and unchanged validation.rs hash exactly match BASELINE_RUN.json. Raw baseline log records four failed intended regressions. The later tests add controls beyond that baseline; not every final assertion was observed failing independently.
- Inspected supplied final log: 180 unit tests passed, zero failed, zero doc tests. This is parent-run candidate evidence, not reviewer-executed validation. No Cargo, build, native, browser, npm or CAEPIPE execution was performed by this reviewer, respecting the shared CPU lane restriction.
- Independent lightweight Python arithmetic supplies realistic magnitudes and equilibrium for the reference/control cases; it is not execution of production code. STATIC_CHECKS.json and REFERENCE_CALCULATIONS.json preserve the results.
- Two historical tests that required silent hydrotest-pressure omission are prospectively replaced with explicit blocking and an ordinary-pressure positive control. This implements the authorized interim containment; no protected numerical oracle or tolerance was weakened. Full hydrotest mechanics remains outstanding.

After repair, rerun affected checks under the manager's authorized CPU lane and obtain independent review coverage of the changed candidate. This review cannot establish full programme completion, downstream/native acceptance, engineering validity, or release.
