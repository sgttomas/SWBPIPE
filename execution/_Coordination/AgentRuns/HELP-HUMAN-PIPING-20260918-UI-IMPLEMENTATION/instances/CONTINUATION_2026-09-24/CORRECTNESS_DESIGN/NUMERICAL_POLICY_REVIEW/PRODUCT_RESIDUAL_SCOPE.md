# Product-preview residual scope — independent backcheck

Date: 2026-09-24. TASK /root/numerical_policy_review, delegated-harness-native child of ROOT, resumed on a new bounded assignment. No descendants, source/test expectation edits, numerical executions, tests, builds or native/browser application actions occurred. ROOT supplied the historical Git snapshots; this executor inspected and hashed them without running Git. This is policy-scope/reference review, not complete-candidate software review.

**Disposition: the product-preview zero residual policies are real adopted bounded criteria. Do not dismiss them as unsupported because the benchmark JSONs exclude product preview. The exact-zero assertion was applied to a quantized published value and never demonstrated an exactly zero raw residual. A technically justified, explicitly versioned M03 successor is recommended; changing the protected preview acceptance criterion still needs its owning D-19/DEC-046 disposition before it is declared accepted.**

Ordinary implementation of full-precision evidence and a separate M03 integrity gate is already authorized. That does not authorize silently changing the protected zero constants, replacing this failing assertion with an arbitrary epsilon, or continuing to claim raw DEC-046 zero-limit compliance for a nonzero value. The only authority hold identified here concerns superseding the protected acceptance on the affected preview surface. It is not a reason to stop independent kernel work or demand permission for routine numerical choices.

## 1. Adopted scope and provenance

The owning decision is [D-19](../../../../../../_DECISIONS/D-19_release_convergence_tolerance_policy.md), codified as DEC-046 in Piping SOFTWARE_DECOMP §12, not the unrelated D-46 decision file. D-19 §7 accepts a class-tiered convergence record, separate from verification tolerances, and says:

> Fixture-local overrides may only tighten; loosening a governed convergence value or raising an iteration cap is a new governance event.

The quoted obligation is confirmed by the actual DEC-046 row. It does not mean every historical zero observation is scientifically sound, but it does prevent silently editing one accepted predicate solely to get a pass.

The historical evidence is specific:

| Source | What it establishes |
|---|---|
| Commit 00effc54b1adc99fab71a7c7bccc5c23ed49fe68, selected source diff acquired by ROOT | Adds product policy ID DEC-046-CV-B-product-preview-free-dof-force-moment-residual-v1 and force/moment absolute limit constants 0.0; upgrades product metadata from observed/TBD to accepted. |
| Original TP-R4-D9-FREEDOFRESIDPOLICY-001 run record at that commit | Objective explicitly promotes validation **and product-preview** surfaces where evidence was recorded as zero. Its boundary explicitly includes the current invented product-preview dense-loop surface and excludes broader non-seed/release/sparse thresholds. |
| Original PLAN_COMPLETION_LOG at that commit | Independently records the product-preview promotion and accepted metadata for the invented dense-loop surface. The log identifies itself as history, not decision authority; the underlying authority is DEC-046. |
| VERIFICATION_2026-06-21_r4_exit_gap.md, product-preview row and Blocking Gaps item 1 | Explicitly records accepted preview active-count, free force/moment, work and general-energy policies. Limits the preview policies to the then-current invented dense-loop surface. |
| TP-R4-D9-WORKPOLICY-001 and TP-R4-D9-GENERALENERGYPOLICY-001, 2026-06-22 run records and completion-log entries | Subsequent bounded promotions include the current invented product-preview work/general-energy surface; they are not confined to the benchmark seed JSONs. |
| D-75/DEC-113 and D-76/DEC-114, 2026-09-23 | Their unit corrections explicitly preserve product-preview records and limits. They neither withdraw this preview policy nor fix its precision premise. |

The benchmark force/moment/work/general-energy JSONs explicitly exclude product-preview thresholds because those records govern a different seed surface. Their absence of preview entries is **not evidence of absent preview authority**. Conversely, the preview policy's real existence does not make it a general nonlinear/contact acceptance standard for all authored models.

No separate human act approving each zero scalar was found in this bounded history. The implementation run invokes DEC-046's authority to promote measured entries, and subsequent project records treat that promotion as accepted. The current project instruction to preserve adopted/protected criteria therefore applies; an agent cannot erase the scope by calling the promotion “metadata only.”

## 2. The original zero test measured the rounded carrier

At original commit 00effc54:

- Product source lines 54–57 define the preview force/moment policy and zero limits.
- append_nonlinear_scalar_result, around line 1445, stores value: round6(value).
- The raw NonlinearResidualObservation force/moment values are passed to that helper around lines 1530–1555.
- mixed_nonlinear_preview_bundle_converges_and_emits_each_support_state, around lines 6201–6209, asserts the **returned ResultItem value** equals 0.0.
- round6, around line 5531, computes round(value*1_000_000)/1_000_000 and canonicalizes zero.

Thus the old passing assertion only established that the published residual rounded to zero. In the ordinary finite range, positive residuals below roughly 0.5e-6 in that row's unit could satisfy it. It did not establish a raw equilibrium residual of exactly zero. The promoted metadata nevertheless described the quantity as free-DOF force/moment equilibrium with a zero threshold; it did not declare a quantized measurement operator as the policy basis.

The promotion diff leaves the prior zero assertion and rounding path in place while changing the metadata/limits to accepted. That is a flawed measurement-to-claim inference. It is enough to reject any claim that the old green test proves exact raw equilibrium. It is not enough to infer that the old raw residual was exactly today's value: this reviewer did not execute the historical model/solver.

In the current numerical branch the producer source at hash 4f78091a3bf865f76c2121d25d1d7e0e35c1bbc0b8a144a21d2456144e34bd09 retains full scalar values. The supplied product_full_repaired.log reports 189 passes and this one failure:

    mixed_nonlinear_preview_bundle_converges_and_emits_each_support_state
    assertion at src/lib.rs:11613
    left: 5.684341886080802e-14
    right: 0.0

The failing row is the free-DOF force residual, in N. It is **not** the signed Coulomb friction-law comparison. The nearby assert_coulomb_force_balance_n helper concerns a separate nonzero force relation under the analytical comparison policy and cannot justify editing this exact-zero residual criterion.

The reported value is approximately 2^-44 N and rounds to zero through the historical round6 path. Its magnitude is consistent with ordinary floating-point cancellation, but the scalar alone does not prove that explanation or establish acceptable equilibrium. The affected original row, denominator, selected force/support state and evaluation allowance are needed to establish the new numerical quality claim. No new pass is asserted here.

## 3. Current implementation does not enforce the displayed zero limits as nonlinear stopping tests

In the inspected product source, the force/moment/work/general-energy constants feed emitted metadata and test expectations. Their usages do not compare the raw residuals against those constants before mechanics success. The assembled nonlinear loop explicitly states that its governed residual is the classifier's changed-support count and that callers bind the reported force/displacement/work observations to policies.

The current converged expression in nonlinear_integration additionally checks blocked status, recovered-path state constancy, deferred sliding-force application, and tangential/derived-normal branch admissibility. Those are material checks and must be preserved. It does not make a zero raw free-force/work comparison. The reported free-DOF work measure is max_i |r_i*u_i|, and the “general-energy” record names that same source measure. It is neither total strain energy nor a separate global energy balance proof.

Consequences:

1. Correct full-precision publication exposes a pre-existing gap between accepted metadata, quantized assertions and actual raw enforcement.
2. That enforcement gap does not make the adopted criterion disposable.
3. A per-row M03 linear-equilibrium check on the selected state closes one missing integrity check. It does not prove all nonlinear constitutive/complementarity conditions, convergence of a future different iteration method, or physical accuracy.
4. The exact zero changed-support count remains meaningful: it is a discrete count. Do not conflate its zero tolerance with a continuous floating-point equilibrium residual.

## 4. Independent numerical reason for a successor

An exact mathematical residual need not vanish for a correctly rounded finite solution. The harmless scalar problem 3*x=1 has no binary64 solution with exactly zero mathematical residual: the correctly rounded x is 6004799503160661/2^54, so 3*x-1=-2^-54. A working-precision product may round 3*x back to 1 and report zero, while a more accurate residual calculation reveals the nonzero residual. A policy that equates displayed/computed zero with mathematical exactness can therefore reward less accurate observation.

A physical-unit residual floor inferred from round6 is also unsuitable. For example, omit the entire load F=1e-8 N in a stable scalar problem and return x=0. The true represented equation residual is -1e-8 N, which round6 publishes as zero, yet its componentwise relative backward error is 1. This independently shows why retaining hidden quantization is weaker than an original-equation scaled check. It is not a test execution or a statement that the current fixture omits a load.

The earlier independently reviewed M03 policy remains appropriate: preserve the actual original equations and check free-row residual quality relative to |K|*|u|+|f|, with explicit evaluation uncertainty and range handling. Current [LAPACK 3.12.1 DPORFS](https://www.netlib.org/lapack/explore-html/d7/dfd/group__porfs_gaff660c485d1ff77c2c5c85a6bbf81900.html) uses original-matrix componentwise backward error and safe-minimum handling. It does not demand exact residual zero.

The current [PETSc SNES convergence documentation](https://petsc.org/release/manualpages/SNES/SNESConvergedDefault/) separates residual, change-of-iterate and failure criteria, while its [reduced-space active-set method](https://petsc.org/release/manualpages/SNES/SNESVINEWTONRSLS/) retains bound/residual-sign conditions alongside reduced linear solves. These primary sources were checked on 2026-09-24; neither supplies a piping-specific Coulomb tolerance or proves this app's nonlinear method. The broader 2019/2022 numerical-reference qualification in this directory remains applicable and is not repeated as a fresh software validation.

## 5. Concrete recommended successor package

Prepare a new prospective policy ID, for example M03-PRODUCT-PREVIEW-EQUILIBRIUM-v1, with an explicit DEC-046 predecessor link and the exact new metric/basis. Do not reuse the old accepted ID for different arithmetic or claim the old numerical limits are unchanged. The owning disposition should cover the following limited change:

1. **Publish truthfully.** Retain full finite raw force, moment and work observations; preserve historical envelopes and their hashes. Display formatting is separate. Preserve an explicitly named historical quantization/compatibility witness if needed, without using it to establish current physics.
2. **Use original-equation equilibrium.** On the actual final selected active state compute r=K_original*u-f_same_state, including imposed values and actual affine/sliding loads. Use the independently specified M03 guarded componentwise ratio and per-row target 64*gamma(m_i), with u_roundoff=2^-53, actual count definitions and range controls. The multiplier is the recorded engineering screen, not a universal accuracy theorem.
3. **Keep separate quantities.** Continue emitting force and moment residual magnitudes in N and N*m with node/DOF mapping. Do not compare a mixed-dimensional global norm to a guessed 1e-9 or reuse the Coulomb assertion helper.
4. **Derive work evidence rather than guess a joule floor.** If the accepted free-row bound is |r_i| <= tau_i*d_i, then |u_i*r_i| <= |u_i|*tau_i*d_i, with the computation's own allowance. Retain max |u_i*r_i| as corroborating evidence and label the historical general-energy alias accurately. Work alone cannot accept equilibrium, because u_i=0 can hide a bad row. No new total/modal/physical energy acceptance is implied.
5. **Preserve nonlinear checks.** Keep changed-support tolerance/floor 0, cap 4, existing admissibility and recovery rules, and the protected displacement/reaction-delta limits. Require the selected-state M03 checks in addition to the applicable nonlinear checks. Changing any contact law, admissibility tolerance or convergence method remains its own scoped work.
6. **Preserve other protected surfaces.** Do not alter benchmark seed/multi-support zero limits, DEC-026 analytical accuracy, DEC-053 observations, release policy, D-75/D-76 unit decisions or old evidence through this product-preview successor.
7. **Separate policy status from observed compliance.** New carrier metadata must identify the actual evaluated policy and pass/fail/uncertainty result. An accepted policy ID is not itself evidence that this solve satisfied it. Do not attach unchanged “accepted zero threshold” semantics to a positive raw residual and imply compliance.

This is not a recommendation to choose an epsilon slightly larger than 5.684e-14. The successor's normalized metric, uncertainty treatment and failure cases are independently grounded. Its thresholds and implemented count/arithmetic still need candidate evidence; the one current failure does not calibrate them.

## 6. Owning decision and immediate safe actions

**A material owning decision remains before declaring a positive raw residual compliant in place of the protected preview zero criterion.** The applicable Piping AGENTS instruction says an explicit ruling/adopted requirement/protected criterion requires its owning decision before reversal, and D-19/DEC-046 explicitly makes loosening a governed convergence value a governance event. The current correctness activation authorizes engineering repairs and independently justified reference replacement, but also preserves existing method-specific limits against silent loosening. This record is not itself an owner act accepting this particular protected-policy supersession.

The correct governance act is narrow: adopt the versioned raw-equilibrium successor for the historically quantized product-preview surface, acknowledge the invalid raw-zero inference, preserve its historical record, and leave the listed other protected axes unchanged. It is not a request for the owner to decide numerical linear algebra from scratch or approve an arbitrary tolerance. ROOT should carry the concrete successor and evidence to the owning decision; this TASK does not send the user an approval question.

Before that final disposition, authorized work can and should continue:

- Retain the failed run and immutable current source identity.
- Prepare the new M03 metric, full-precision transport and associated negative controls in isolated scope without marking the old protected assertion waived/passed.
- Capture the failing fixture's final-state original row/action, free map, force/moment raw values, denominator/count/evaluation allowance, and active/contact evidence in both modes. This requires the solver manager's execution authority; it was not run here.
- Compare raw and historical quantized values side by side. Show the old assertion can hide a deliberately omitted tiny load and that the successor rejects it.
- Include a same-state solve mutation and a wrong-support-state/contact mutation: the first must fail equilibrium; the second must fail the applicable nonlinear/admissibility condition even if its linear system is solved accurately.
- Present the explicitly versioned successor with those observations for the owning disposition, then update producer metadata, tests and connected consumers together and obtain complete-candidate review.

Do not manufacture exact zero by rounding, clamping, snapping small residuals, changing the fixture load, or forcing a floating-point solution solely to satisfy this assertion. Do not silently downgrade accepted v1 metadata to “observation only” to sidestep the protected criterion. The old failed predicate remains a failed protected witness until it is repaired on its actual basis or prospectively superseded through its owner.

## 7. Return and limits

Confirmed: real bounded product-preview policy; original rounded-carrier premise; current raw force-residual failure; distinction from Coulomb; current lack of raw-limit stopping enforcement; scientifically justified prospective successor.

Not established: exact raw residual of the historical commit, benign-roundoff classification of the current row without its term data, a new candidate pass, new nonlinear tolerance acceptance, or complete product correctness. No historical record, constant, test expectation or source file was changed by this task. Origin/hash custody is in [_run_records/PRODUCT_RESIDUAL_SCOPE_BASIS.md](_run_records/PRODUCT_RESIDUAL_SCOPE_BASIS.md).

## 8. Subsequent authority clarification

ROOT asked the same independent reviewer to distinguish a non-delegable reservation from the general requirement for an owning disposition/governance event, in light of the owner's live direction to fix all findings and use expert judgment to decide correctness. The reviewer returned this clarification in the same delegated task on 2026-09-24:

> I found no explicit non-delegable reservation covering this prospective measured successor despite the live delegation of expert correctness decisions.
>
> D-19/DEC-046 requires a recorded governance event; Piping AGENTS requires an owning disposition. My inference that these necessarily require another owner prompt was too strong. ROOT can record the qualified successor as expert selection under the existing delegation, with independent evidence/review and preserved historical limits—without implying personal owner approval of its formula.

The factual scope, rounding and numerical findings above remain unchanged. The required disposition is still explicit; a fresh owner prompt is not an additional prerequisite under the actual delegation. ROOT will record any qualified prospective selection and its evidence as an expert decision under that authority. At this checkpoint the affected row and negative-control evidence remain in preparation: no old zero assertion is waived, no arbitrary epsilon is selected, and no successor pass is claimed. This addendum preserves the earlier advisory interpretation rather than silently rewriting it.
