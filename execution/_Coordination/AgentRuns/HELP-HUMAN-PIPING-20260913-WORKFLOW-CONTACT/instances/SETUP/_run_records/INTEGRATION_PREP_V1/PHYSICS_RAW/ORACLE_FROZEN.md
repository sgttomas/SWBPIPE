# Independent analytical oracle — frozen before source edits

Accepted upstream: copied PARENT_BRIEF.md SHA256 c8c0d3c538a04e132588c1d4ceff6d9db757d13a782e2f4b49f613febf53b292; APPROVED_PLAN.md and OWNER_DIRECTION.md. This derivative oracle is not decomposition authority.

Original invented two-node axial system, stiffness K=100[[1,-1],[-1,1]] N/m, external force F=[0,10] N, authored positive gap boundaries a=[0.05,0.20] m. Active constraints prescribe u=a; reactions R=Ku-F=[-15,+5] N. Negative reaction retains positive-gap contact; positive reaction releases it. Tip release leaves u0=.05 m and 100(u1-u0)=10 N, hence u1=.15 m and R=[-10,0] N. Both-inactive seed is singular, and mixed root-inactive/tip-active seed tries both-active once; successful trial counts completed iteration1. All-active seed is admitted but does not need recovery. Seed order and solve mode cannot alter equilibrium.

Reverse F=[0,-10] N: both-active reactions [-15,+25] N release tip; root-only solution u=[.05,-.05] m, R=[+10,0] N releases root; following unconstrained solve is singular and fails without another rescue.

F=[0,0] N zero reaction on a closed boundary follows the existing classifier zero convention; do not change that law. A valid one-contact fixture with stable recovered contact can converge at cap1; a changing support boundary cannot converge under tolerance>=1. Invalid rotational/state/clearance/duplicate/base-overlap classes cannot trigger recovery. Positive ground springs remain intact. Necessary preflight counts valid potential contacts for all seeds, but it does not establish solvability.

Product regression must remove base UX restraint from a fully otherwise-restrained root, supply eligible UX contact, pass necessary preflight and bypass only exact singular preliminary linear solve to execute nonlinear. Assertions use independent analytical values, never repaired output. Published values tolerance .5e-6+1e-10 declared units.

No source changes existed at freeze: HEAD b2c133d7aef38034d10fed2b9b7ef64f517f2ba0, clean worktree.
