# Independent decimal-gap reference check

**Confirmed:** under the selected exact represented-binary64 source convention, the captured magnitude-5 case must finish **Inactive**, for both senses, either initial seed and either solver mode. Its existing Active expectation is a reference conflict, not a true exact-touching control. Magnitude 4 remains Inactive and magnitude 6 remains Active. The existing 1e-14 displacement/reaction assertions need no relaxation.

This return supports ROOT's prospective oracle disposition under the existing owner delegation. It does not change the test, source-number convention, contact law or acceptance authority, and creates no additional human approval reservation.

## Bound source and role

Independent TASK Type2 /root/solver_manager/kernel_review, parent /root/solver_manager, delegated-harness-native, unchanged gpt-6-astra xhigh dispatch. Full Root AGENTS, TASK, Piping AGENTS and LOOP_INIT were reread. [Origins and enforcement](_run_records/review/ORIGINS.json) record actual paths, hashes and runtime. No descendants, source/test/Git edit, Cargo, Rust test, Node, native or browser execution. Writes are limited to this REVIEW.md and the owned review records.

The captured [original test excerpt](_run_records/original_test_excerpt.rs.txt), SHA-256 6c95983e36c141d5d911cfcbfe3135deaafddc0810463dfadf4856d43ebbdd28, occurs verbatim in the actor-preserved [nonlinear baseline](../NONLINEAR_INTEGRATION/_run_records/before/lib.rs), SHA-256 e21c812accb1ab2eb4cd2c9e4509cd25ba38982b3e649aafaeaa64ea9cb3f009. The actor's BASIS.json confirms that identity. The named test begins at baseline line 4055; its magnitude-state assertion is at lines 4083–4089. two_node_axial_problem begins at line 2907.

The actual nonlinear_supports Gap law was read and verified byte-identical to commit 63b9a56cbcc40fd2ba7e1a3c2f2555214aa6d8d9; a copy is retained [here](_run_records/review/nonlinear_supports_unchanged_source.rs). The previously frozen frame-kernel source establishes the FrameSection argument order and EA/L axial term. [Source check](_run_records/review/SOURCE_CHECK.json) binds all inputs and captured excerpts. The actively edited live nonlinear lib.rs was not read or qualified.

## Source and arithmetic proof

The fixture places nodes one unit apart on the x axis, uses FrameSection::new(100,40,1,1,1,1), and fixes every DOF except the tip axial coordinate. The root is fixed. The frozen frame formula therefore gives k=EA/L=100 exactly; these inputs and the identity axial transformation introduce no coefficient ambiguity. The test assigns exact integer force magnitudes 4, 5 and 6, with physical sense s∈{−1,+1}.

NonlinearSupport::gap receives and stores an f64 clearance. Its represented value has bits 0x3fa999999999999a and equals g=3602879701896397/2^56. The exact released response at magnitude 5 is 5/k=1/20, so:

- g−1/20=1/(5·2^56)>0.
- If Active is imposed, u=s*g and r=k*u−s*5, giving s*r=100*g−5=5/2^54>0.

The unchanged Gap law closes a released support when s*u>=g; the exact released response therefore stays Inactive. An active Gap bears only when s*r<=0, including exactly zero. The positive normalized reaction above would pull the node onto the stop, so an Active seed releases. Its next released solve remains Inactive. A zero-dimensional free system at the active seed cannot bypass this reaction test.

The gap excess is approximately 2.7756e-18 in displacement units and the pulling reaction approximately 2.7756e-16 in force units. Neither is mathematically zero. Negative sense reverses physical displacement/reaction signs while retaining these normalized inequalities.

## Cases, seeds and quantity checks

[Independent check](_run_records/review/check_reference.py) decodes IEEE bits and derives the equations with Fraction. It imports no production code or submitted generator. [Exact output](_run_records/review/EXACT_REVIEW.json) retains all traces and independently confirms the manager's packet.

| Force magnitude | Exact final state | Initially Inactive | Initially Active | Exact selected tip reaction |
|---|---|---|---|---|
| 4 | Inactive | stays Inactive | releases, then stays Inactive | 0 |
| 5 | Inactive | stays Inactive | releases, then stays Inactive | 0 |
| 6 | Active | closes, then stays Active | stays Active | s·(−1+5/2^54) |

Both senses and both mode labels give 24 backend-independent reference cases; eight magnitude-5 expectations conflict with the captured assertion. Each reference trace converges within two ordinary iterations, inside the unchanged fixture cap eight. Mode labels identify required equivalent expectations; **no production backend was executed here**.

The selected reference binary64 projections are ±0.04 at magnitude 4, ±0.05 at magnitudes 5 and 6, zero tip reaction at 4/5, and a reaction within approximately 2.23e-16 of ∓1 at 6. They satisfy the captured 1e-14 quantity checks. Those assertions must remain and still pass in actual candidate execution.

The historical Active result is explainable by rounded arithmetic: fl(5/100) has the same bits as stored g, although the exact response is smaller; fl(100*g)−5 is zero, although the exact original action is positive. Consequently a published displacement can equal the stored gap while the exact represented-source state is Inactive. Published-value equality or a rounded reaction is not an exact contact oracle. Treating decimal spelling “0.05” as exact rational 1/20 would change the supplied source convention; it is not what the f64 field stores.

## True equality controls

The exact-touching Active convention is retained and independently checked:

1. An independent single-gap control with k=100, g=1/8 and F=25/2 has exact released u=s/8 and exactly zero active reaction. Both senses and either seed converge to Active; eight mode-labelled reference cases are recorded.
2. The captured maintained audit_coupled_exact_contacts_converge_from_all_seeds uses two axial spans, F=25/2 and gaps (1/8,1/4). Its free matrix [[200,−100],[−100,100]] has first minor 200 and determinant 10000. Exact rational elimination gives u=(s/8,s/4) and both contact reactions zero. All four seeds, both senses and both mode labels finish Active/Active; sixteen reference cases are recorded.

No fixture replacement or contact-law amendment is needed. The decimal-spelled example should remain visible as a near-boundary represented-source control alongside these true dyadic ties.

## Prospective disposition supported by this review

For this captured input set only, the correct final-state oracle is **4→Inactive, 5→Inactive, 6→Active**. A prospective correction should retain the same k, literal gap, force values, both senses/seeds/modes, convergence checks and 1e-14 quantity comparisons. It must not install a production magnitude-5 threshold, epsilon, reaction floor or new decimal-number interpretation. Preserve the captured assertion and past observations as historical evidence.

ROOT owns the prospective disposition already assigned by the owner; this independent result supplies its warrant. No test/source amendment was made here. Subsequent amended candidate review and actual Rust execution must establish that the implementation satisfies the correct reference. This check does not qualify the ongoing nonlinear half-diff, new backend, broader contact/affine behavior or product release.

