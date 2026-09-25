# Independent mass-reference and method-policy check

Current disposition: complete 23-file source review PASS carried through lock-only normalization and two authorized downstream edges and a source-equivalent TypeScript narrowing repair to patch `6a349262358bcfc8295cc18e7ec116be544f6738ee425dae51848b3447334c99`; see the [final backcheck](_run_records/FINAL_BACKCHECK.md). The maintained WASM build passed; UI/TypeScript checks remain pending. The following reference-stage record and later historical dispositions are retained in sequence. This is a TASK Type 2 return to `/root/authoring_manager`, executed through delegated-harness-native collaboration by `/root/authoring_manager/self_weight_review`. I did not implement product changes or delegate.

## Result and independent basis

For the existing uniform circular mass model, let outside diameter be D, effective wall t = wall minus the explicit mill deduction, and exterior insulation thickness d. Starting from source radii R = D/2, r = R−t, and S = R+d gives:

- Metal area: π(R²−r²) = πt(D−t).
- Contents area: πr² = π(D/2−t)².
- Insulation area: π(S²−R²) = πd(D+d).
- Mass per length: ρmetal Ametal + ρcontents Acontents + ρinsulation Ainsulation. Signed generated intensity is this mass multiplied by the explicit axis acceleration.

The factored areas are exact algebraic identities, not a thin-wall approximation. They do not change the physical mass recipe, the mill basis, geometry inputs, density sources, or M30. The bore calculation must use its source radius; deriving it by subtracting rounded wall area from rounded disk area would lose a small bore. A solid bore has exactly zero area. Zero insulation thickness and zero optional densities are valid zero contributions. A positive area that the implementation cannot represent must produce an explicit range failure, rather than a zero contribution masquerading as an ordinary optional absence. The source domain remains D > 0 and 0 < t ≤ D/2, with nonnegative optional inputs and a complete insulation pair.

[NIST SP 811 chapter 4](https://www.nist.gov/pml/special-publication-811/nist-guide-si-chapter-4-two-classes-si-units-and-si-prefixes), tables 2–3, confirms area in m², density in kg/m³, acceleration in m/s², and force in kg·m/s². Hence mass is kg/m and intensity is N/m. This check uses the unit relationships, not historical base-kilogram definitions. [MIT's multiple-force-member treatment](https://web.mit.edu/4.441/1_lectures/1_lecture16/1_lecture16.html) supports combining a uniform load through its resultant and midpoint first moment. I separately derive a straight horizontal cantilever's support reaction as −(qL+P) and support moment as −(qL²/2+PL). Neither identity requires a production stiffness or mass helper.

These references apply to the invented uniform, straight, small-displacement test member. New constitutive, large-deformation, layered/curved, or pressure-force theories do not improve this particular conservation and geometry check without changing its assumptions; none is introduced here. This does not qualify those capabilities or engineering use.

## Seven-case expectations

[`SEVEN_MASS_REFERENCES.json`](_run_records/SEVEN_MASS_REFERENCES.json) records exact Fraction ratios for every represented input, all three area factors, every component mass, and 100-digit Decimal expectations. Pi is calculated independently with Machin's identity. [`reference_arithmetic.py`](_run_records/reference_arithmetic.py) imports no product code. Its input values are the seven Rust probe assignments, including `.5−2^-30` and `2^-55`, reconstructed as the same binary64 inputs. All 21 disk-area versus factored-area equalities are exact rational equalities. These displayed values are rounded for readability; they are not new pass tolerances.

| Probe | Expected mass, kg/m | Expected q at −1 m/s², N/m |
|---|---:|---:|
| ordinary | 2.8274333882308141 | −2.8274333882308141 |
| thin_1e9 | 3.1415926504482008e−9 | −3.1415926504482008e−9 |
| thin_1e12 | 3.1415926535866516e−12 | −3.1415926535866516e−12 |
| sub_ulp_wall | 8.7196712450215795e−17 | −8.7196712450215795e−17 |
| thin_insulation | 3.1415926535929348 | −3.1415926535929348 |
| sub_ulp_insulation | 0.087196712450215800 | −0.087196712450215800 |
| small_bore_contents | 2.7248972640692437e−6 | −2.7248972640692437e−6 |

The manager's existing four wall-area exact-rational references agree exactly with this independent calculation; see [`PRIOR_ARITHMETIC_BACKCHECK.json`](_run_records/PRIOR_ARITHMETIC_BACKCHECK.json). An illustrative Python evaluation of the legacy expression gives relative error about 2.82e−8 for thin_1e9, 2.21e−5 for thin_1e12 and thin_insulation, zero metal mass for sub_ulp_wall, and zero insulation mass for sub_ulp_insulation. That illustration is explicitly not a Rust production run or final acceptance check. The small-bore case checks preservation of a positive contents contribution; its legacy expression happens to be accurate for these represented inputs.

The original density baseline remains an actual failure: `../_run_records/checks/baseline.log` records retained q = −19.792033717615706 N/m, reaction 50.584067 N after density doubling, and independent expected reaction 90.16813487046282 N. The corresponding first moment is 101.16813487046282 N·m. That stale-load discrepancy is separate from the thin-geometry cancellation defect.

## Legacy and successor policy

The manager's follow-up proposal is sound in principle: keep `compute_pipe_mass_per_length` unchanged for legacy reconstruction; make new self-weight v2 use the sole PHYSICS-owned source-area helper; require an explicit stable `mass_method` discriminant in new v2 evidence. Preserve historical v1 meaning and pre-stable unmerged v2 artifacts.

The concrete implementation and its checks must enforce these consequences:

1. Dispatch the recorded method before verifying retained mass. Re-evaluating v1 with the new stable formula cannot establish original v1 validity. The initially inspected source called the active helper at `self_weight.rs:616` and `:818`; those call sites require the explicit dispatch that the manager described.
2. A known unchanged managed v1 payload needs an explicit method refresh even when its dependency projection is unchanged. Otherwise it remains on the inaccurate calculation indefinitely. The diagnostic should identify the outdated method; it must not falsely say physical inputs changed.
3. Exact current-versus-retained payload comparison precedes replacement. A changed or unverifiable v1 payload is eligible only for explicit preservation as manual ownership. No numerical tolerance, nearest-neighbor value, convergence through repeated JSON transport, or recomputed new-method hash can prove manual intent or justify replacing it.
4. The historical algorithm and its documented transport may support exact v1 recognition. They must not be broadened to accept merely close values. This restriction is especially relevant while the known typed/JSON floating-point transport issue is being repaired by its assigned owner.
5. New v2 evidence requires the recognized stable mass-method tag and internally consistent source inputs, mass, signed acceleration, and snapshot. Missing/unknown tags and the pre-stable v2 experiment must not be silently interpreted as stable v2 or reclassified as v1.
6. Refresh retains the historical generation evidence and records the actual successor method. Pure solve/open/save must neither relabel the original method nor manufacture a refresh source hash. Manual preservation retains every physical payload field, keeps the original generation provenance, and makes its fixed ownership visible.
7. Source/helper range errors remain explicit input or range diagnostics. They must not be confused with ordinary dependency drift or a user's manual payload edit. Deterministic identity and method-family detection are only grounds to inspect lineage; they are not sufficient evidence of valid managed state.

The initial `INTEGRATION_CONTRACT.md` still states same-formula v2 and needs this successor policy incorporated before freeze. This is a documentation/candidate dependency already reported to the manager, not a finding against a final candidate that has not yet been supplied.

## Execution evidence and limits

The first probe compiled and ran, then failed its strict direct-versus-wire Value equality at `mass_probe/src/main.rs:22` on thin_1e9. `before.result.json` has exit 101; `before.json` is empty; raw stderr retains the differing results. Thus this initial probe does not contain seven successful outcomes. The manager has acknowledged this and is preserving the failed source/log while changing the diagnostic probe to retain both outcomes without suppressing the parity-failure flag. The independent expected values above are available before that rerun or production effect.

No product tests, builds, native/browser workflows, external Git operations, or implementation edits were performed by this reviewer. Files initially inspected were mutable preparation source, not a frozen candidate. The complete review must receive the final diff/allowlist and explicit stable-method API, the shared PHYSICS reference/implementation handoff, rerun probe outcomes, and actual-candidate targeted checks. The seven cases do not by themselves cover mill deduction, invalid domains, positive-area range failures, solid bore, optional zero inputs, method migration, transport, source guards, atomicity, manual preservation, or save/reopen. Those remain implementation/backcheck coverage obligations, not newly introduced numerical acceptance criteria.

Actual origins, hashes, role/parentage, and read limits are recorded in [`REFERENCE_ORIGINS.json`](_run_records/REFERENCE_ORIGINS.json); initial source snapshots are retained under `_run_records`. Source-qualified skill applied: project `.agents/skills/software-code-review/SKILL.md`. Root brief's included WORKING_ITEMS text was read as supplied context; it did not change this reviewer's Type 2 role.

## Subsequent manual-preservation source precheck

The manager requested a separate bounded source check while the shared stable helper was pending. [MANUAL_PRESERVATION_PRECHECK.md](_run_records/MANUAL_PRESERVATION_PRECHECK.md) records three actionable P2 findings: a historical-pipe equality guard rejects valid retargeted manual preservation; inspection requires the original current pipe before recognizing a modified payload; and the applier validates the absolute mill deduction as dimensionless. A changed valid global direction has no original-axis equality guard in the inspected applier. The mass-reference result above remains valid; correction backchecks and complete frozen-candidate review remain outstanding.

## Complete candidate review

The subsequent 19-file frozen candidate received [complete independent diff review](_run_records/FROZEN_CANDIDATE_REVIEW.md). The three earlier findings were repaired and supported by connected/applier evidence. All source hashes and patch scope matched. A new P1 defect holds the candidate: direct imported managed-refresh apply lacks the planner's exact historical-payload eligibility gate and can overwrite a manually modified magnitude. The manager accepted the finding and is preparing a shared source-owned guard. The seven mass-reference values remain independently confirmed; UI runtime verification and revised-candidate backcheck remain pending.

## Final source backcheck

The [final backcheck](_run_records/FINAL_BACKCHECK.md) supersedes the prior source holds for the reviewed 21-file candidate. Exact before/candidate ownership inspection, gravity/request-intent binding, and whole-case completion now close all actionable review findings. The final 37 affected locked Rust checks and their 21-source/patch/log bindings pass. Source is ready for manager integration and conditional validation; UI/native qualification and final merge checks remain outside this reviewer claim.

## Cargo normalization backcheck

[Lock-only backcheck](_run_records/LOCK_NORMALIZATION_BACKCHECK.md) confirms that the maintained WASM build changed only Cargo.lock package/dependency ordering and whitespace. All 44 package records remain identical; source review PASS carries to the normalized freeze.

## Downstream lock closure

[Two downstream lock edges](_run_records/DOWNSTREAM_LOCK_BACKCHECK.md) independently pass. All prior source bytes are unchanged; the 23-file source candidate is clear for allocated runtime/UI/TypeScript verification.

## UI/runtime and TypeScript repair

[Runtime evidence and narrowing repair](_run_records/UI_RUNTIME_TSC_BACKCHECK.md) are independently checked: maintained WASM passes and 25 mixed UI/transport tests passed on the prior freeze. The original tsc failure remains historical; the one-file repair passes static review pending tsc/two-suite rerun.
