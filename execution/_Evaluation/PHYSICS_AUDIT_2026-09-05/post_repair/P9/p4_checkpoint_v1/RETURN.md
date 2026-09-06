# P9 P4 checkpoint

Compiled existing independent suite: 2 PASS /8 FAIL, exit101 expected incomplete repair. Both solver modes reach all assertions in the inactive-gap parallel-spring test and the closed axial-stop selected-state test. This verifies spring displacement/action, signed joint balance, contact state/reaction and ordinary selected node/element coherence for those two original fixtures.

Sparse cantilever reaction350N and pure-moment force0N now pass before later station failures; neither reaches dense mode. Station failures remain expected R08: cantilever quarter shear175 vs350N, axial quarter175 vs350N, pure moment midspan0 vs100Nm. P5 remaining expected failures: opposite SUM magnitude .812046 vs0mm, tiny scaled0 vs.406023368mm, SECOND source refers FIRST, full UDL .116007 vs.087005007mm, partial .058003 vs.038064691mm. No unexpected defect identified.

No oracle, source or test edit; no mutation. Current suite units checked. Root-granted isolated slot released. Source under fresh P4 review, not yet root accepted. Source before/after hashes bound and drift report explicitly retained. Derivative checkpoint only; no lifecycle or full R10 closure. Await final P5 source for full both-mode rerun and negative controls.

Source drift count: 0.
