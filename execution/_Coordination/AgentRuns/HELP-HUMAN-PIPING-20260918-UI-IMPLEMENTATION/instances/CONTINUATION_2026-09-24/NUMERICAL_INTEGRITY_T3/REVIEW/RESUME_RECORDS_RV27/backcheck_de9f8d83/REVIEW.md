# RV27 — correction backcheck at de9f8d83

**PASS. RV27-S1 is closed by the same independent reviewer.** No new actionable
finding. The records slice is suitable for fan-in at candidate
`de9f8d83bdaab01b2bb988b78791f608adda61e6`, subject to the remaining required
exact-final-head CI/DEC-025 gates and any subsequent-delta review.

This additive backcheck covers the complete two-file delta from
`90b6bcbbf64b13975211038bf3f33bb87273e646`. Together with the sealed initial
full-diff review, it covers PR1068's records candidate against
`d01ad98a754698631f927709d08284c272de85e8`. The clean local HEAD and live PR head
both matched the new full candidate identity.

The work graph's current row now calls the source-only wave initial and explicitly
records I22's subsequent G0 one-probe-build authorization. Its state paragraph
likewise says this records candidate includes the build-only grant and contains
no solver-case grant or numerical acceptance. The ruling retains the original
words with an explicit historical-status correction bracket and appends the
RV27 finding/disposition with the correct original review hash. The correction
preserves grant versus actual execution and does not import later G1/B activity.

Only ROOT_RULINGS_V1.md and WORK_GRAPH.md changed. Product source, validation,
briefs, audit, preserved packet, and all old manifest blobs are unchanged. The
initial review seal is unchanged at
`6d46d3170ca77f3015e0cad5ece55757d5070f566cf59a37aae5a07e91ff8715`, with all
17 entries verifying. The preserved packet seal remains
`729d342640301294dfd31e02659a0a443231f627858aca85c57ee5b7b8192b97`, with all
293 entries verifying. The original full archive/hash/reconstruction checks
therefore retain coverage of identical evidence bytes.

Fresh GEN-8 on the exact clean new candidate passed; the unmodified captured
stdout and timing are in evidence/gen8.log. No Rust, solver, DEC-025, source
repair, Git/index mutation, delegation or host-tool work was performed. All
Git reads used GIT_OPTIONAL_LOCKS=0. Writes were confined to this additive
backcheck subtree and the assigned scratch script. The initial sealed packet
was not edited. No owner-review or model-diversity claim is made.

A1 remains open; F2a reliance remains held; K6c remains unaccepted. This is a
records consistency confirmation, not a numerical/design acceptance or proof
that remaining merge gates have completed. Later candidate changes need review.
