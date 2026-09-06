# Authored observation coverage

| Test | Repair | Independent observation |
|---|---|---|
| actual_cantilever_recovery_and_force_norm | R07/R08/R10 | Tip EB displacement, force norm350N, actual root moment700Nm and three moment/shear cuts |
| pure_moment_does_not_become_force | R07/R10 | Zero force norm, actual constant100Nm moment |
| inactive_gap_preserves_parallel_spring_and_its_action | R06/R07/R10 | u=F/(k+ks), ks*u, signed endpoint/spring joint balance, inactive state, zero contact reaction, selected final displacement |
| closed_axial_stop_selects_same_state_for_nodes_and_elements | R06/R10 | active contact, u0, R=-350N, signed endpoint balance, zero ordinary displacement and axial field |
| axial_and_torsion_fields_are_constant | R08/R10 | Three constant force/torque stations and independently derived section stresses |
| opposite_signed_sum_recomputes_magnitudes | R09/R10 | Opposite signed inputs cancel components, displacement norm and force norm |
| tiny_primitive_retains_precision_until_combination_publication | R09/R10 | Tiny load scaled1e6 agrees with independently derived350N tip response |
| derived_links_resolve_to_the_owning_primitive_case | R09/R10 | Actual metadata-only component result sources resolve within second primitive case |
| full_distributed_load_assembly_and_cut_recovery | R11/R10 | qL4/(8EI) tip, qL anchor, quadratic moment/linear shear cuts |
| partial_wind_assembly_and_cut_recovery | R11/R10 | Integrated point-load flexibility on[.5,1.5]m, q100N/m, equilibrium and cuts |

All tests contain sparse and dense invocations. The red V3 checkpoint stops each test at its first sparse failure; no dense pass is claimed. Final acceptance requires both modes to reach all assertions after P4/P5 repairs. R10 mutations are parent-coordinated and remain unexecuted. Existing force-only correction should pass; force+moment mixing reintroduction and actual station zeroing should fail. The audit's historical baseline remains immutable.

No full signed rigid-support vector exists in the consumed interface, so joint equilibrium uses actual signed beam end action, independently derived spring action and known applied force. Existing nonlinear final displacement/reaction/state rows are asserted; trial displacement is not falsely claimed. Fixtures cover stable frictionless contact; no friction, pressure, connector or new engineering policy is adopted. Exact contact equality and kernel invalid-input tests remain P4 scope. This does not claim complete arbitrary model correctness.
