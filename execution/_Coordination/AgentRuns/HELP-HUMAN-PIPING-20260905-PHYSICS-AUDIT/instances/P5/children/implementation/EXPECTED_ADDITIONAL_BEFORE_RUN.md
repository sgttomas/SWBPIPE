# Independent test expectations frozen before new tests execute
Original statics and Euler–Bernoulli expressions, not product outputs. P9 frozen references remain unchanged.
- Cantilever L2 q100 full: tip 1000*200/(EI) mm, root Fy/Mz -200/-200, station j-side Vy/Mz at x1 +100/+50, tip end action zero.
- Partial x[1,2]: tip 1000*1025/(6EI), root -100/-150, midpoint +100/+50.
- Simply supported x[0,1] q100: reactions -75/-25, peak |M|=28.125 at x.75, section modulus I/.084. Uniform full maximum50 at x1. Reversing element preserves global mechanics and maximum.
- Point axial350/torsion350 give constant j-side field350, transverse350 gives shear350 and midpoint moment350. Fixed thermal j-side axial=-EA alpha deltaT everywhere.
- Equal/opposite two cases signed components and displacement/reaction norms cancel; subtraction reverses signed components and preserves nonnegative magnitude. Tiny .00035 N times2/1e6 agrees direct .0007/350 N within public rounding. Missing vector component withholds magnitude.
- Adjacent spans sum to full span; disjoint spans sum through signed component algebra; rotated geometry/load preserves scalar response.
- Existing axial wind lever-rule nodal shares preserve displacement/support actions, but physical station/end actions differ because member load must be recovered. Existing tests must no longer equate those fields.
