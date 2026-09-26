# S0′ — independent refutation of the revision-2 references

HELP_HUMAN (ROOT) records this check. The same fresh-context checker as S0 derived every value from the case definitions and the product's documented laws, using the standard library only and no code from `references.py`. It read the author's numbers only after deriving its own. Its scripts and outputs are archived unchanged in [_run_records/](_run_records/). `s0p_compare.py` loads `s0p_derive.py` from the checker's scratch directory; point it at this folder to rerun it.

**Verdict: DISCREPANCIES — one labelling error in a negative control. No expected value is wrong.** Of 257 scalars compared, 256 agree at 1e-9, with a largest difference of 4.6e-15. The checker also confirmed three things:

- the revision-1 keys and values are byte-unchanged, and the re-serialized hash matches the S0-checked revision-1 output;
- the S0 archive still verifies;
- the new negative controls fail on their targets.

## To fix in revision 3

1. **REF-M05-COMB-2 `negative_control_sum_Nm = 1500`.** Over the labelled cases A1 (tip Fy) and A2 (tip Fz), the sum is 2000. The 1500 value belongs to REF-M05-COMB's A1/T pair. Either relabel the control or use 2000. The envelope value, 1000, is correct either way.
2. **REF-I-L "including axial" control.** Say which variant it is: axial added unintensified (13.3823 MPa) or intensified (13.4257 MPa).
3. **REF-I-T coincidence.** The header value 1.3·500/Z equals the value for the unreferenced continuation b-e. Add an Fx load at c, which gives a-b a bending component that b-e does not see.
4. **REF-B2 frames.** The tangent-frame "section at b" names no force row. Also freeze the chord-frame cut face, ((X+1000)/√2, (X−1000)/√2, 0, 0, 0, −0.2(1000−X)), noting that the product's end_i rows are its negation. Also confirm in S2 that the product accepts an arc span with no adjacent straight pipes.
5. **Pin the nonlinear-support JSON.**
   - NL-C3 uses the `gap` behaviour with `closes_when: positive_displacement`, `gap: 0.0004` and dof UY. It is not `one_way`, which has no gap field.
   - SPRING-GAP needs `closes_when`.
   - ATTR needs `active_when`.
6. **REF-M33-TIE.** Assert the tie-break only when |u_p| and |u_q| are bit-identical in both orders and both modes. Otherwise report "tie not exercised", rather than pass or fail. (This matches review finding SF-D.)
7. **The ATTR global-balance exemption.** State whether revision 1's rule, that every reference checks global balance from published rows, still applies. If it does, ATTR needs an explicit exemption, because both devices are withheld.
8. **Detection list §9.2.** On main, I-L1 also mismatches (i·k·σ = 14.14 against 13.09 MPa), and so does I-T. Add both.
9. **X1-SUPPORTS control target.** Describe it accurately: it catches DOF-slot mapping errors only.
10. **REF-M14-TH.** Give the moment components their own zero scale.
