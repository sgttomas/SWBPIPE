# Pressure-thrust clarification

The user first supplied a correction to pass on, then clarified that it came from another agent and asked whether it was true. It is assessed here as a technical claim, not recorded as an owner adoption of a pressure formulation or reference. The earlier reviewed assessment and original report remain unchanged.

The correction is sound as a qualified modelling instruction: for steady internal pressure in a continuous welded line with an intact axial load path, a bend alone is not a reason to add an independent unbalanced external pA-type nodal thrust load. The report’s blanket instruction to apply thrust “at changes of direction” must not be used as the repair specification.

It is too absolute if read as denying pressure forces on an isolated elbow. Fluid pressure has a resultant on that elbow’s wetted surface; a correct free-body diagram also includes the forces carried by the adjacent pipe walls and welded connections. External restraint demand depends on that complete load path and boundary conditions. Pressure-driven extension or bend opening can still produce restraint reactions. Untied expansion joints, interrupted axial force paths and transient/momentum loads require separate treatment.

This distinction also does not prohibit consistent pressure equivalent-load formulations. A formulation may explicitly carry wall/cap pressure loading with matching wall/effective-force recovery; deleting only the bend load term can break that consistency just as adding an extra thrust can double-count it. The existing four-case reference and closure-topology work remains necessary.

Primary-source checks:

- [Bentley AutoPIPE: pressure thrust on an anchor](https://bentleysystems.service-now.com/community?id=kb_article&sysparm_article=KB0025326) distinguishes steady pressure extension from unbalanced thrust across an untied flexible joint or fluid transient.
- [AISI/STI-SPFA Welded Steel Pipe Design Manual, 2007](https://stispfa.org/wp-content/uploads/2022/11/Welded-Steel-Pipe-10.10.07.pdf), printed pp21 and50, explicitly distinguishes thrust restraint carried by welded pipe from external restraint for joints that cannot carry that longitudinal load. It also discusses pressure-resultant forces at direction changes; this is why an unqualified “no force at a bend” statement is misleading.
- [Hexagon CAESAR II Bourdon-pressure documentation](https://docs.hexagonppm.com/r/en-US/CAESAR-II-Users-Guide/Version-12/334808) identifies pressure elongation and bend-opening effects as explicit modelling behavior.

ROOT forwarded this qualified clarification directly to /root/solver_elements and /root/solver_stress. No solver source, oracle, protected criterion, engineering acceptance or runtime activation changed.
