# Oracle erratum V1 — before behavioral repair completion

Preserve ORACLE_FROZEN.md immutable. Manager caught an incorrect mixed-seed sentence there and in the child's first test freeze. The accepted final equilibrium and both-inactive recovered reaction oracle remain correct.

Root Inactive / tip Active axial seed is initially nonsingular because tip UX fixes the axial rigid-body mode. For F=[0,10] N, tip constraint .20m gives u=[.20,.20]m, reactions [0,-10]N; root inactive gap penetrates and engages. Next all-active solve gives [-15,+5]N, releasing tip, and then root-only equilibrium [.05,.15]m, [-10,0]N. This path uses no singular recovery.

For F=[0,-10] N, the same mixed seed gives u=[.20,.20]m, reactions [0,+10]N: tip releases and root engages simultaneously. Root-only next trial u=[.05,-.05]m, reactions [+10,0]N releases root; following unconstrained solve is later singular and must fail without recovery. Thus release sequence tip then root then singular holds, while first changed_supports is [root,tip], not merely [tip]. Only both-Inactive in this two-gap model guarantees first singular solve and recovered all-active reaction witness.

Source: elementary K=100[[1,-1],[-1,1]] N/m algebra, independent of repaired output; corroborated initial baseline test failure at first reaction assertion in baseline-kernel.txt. Child instructed to preserve initial freeze plus record corrected frozen test bytes and prove both-Inactive baseline failure specifically. No eligibility or classifier changes implied.
