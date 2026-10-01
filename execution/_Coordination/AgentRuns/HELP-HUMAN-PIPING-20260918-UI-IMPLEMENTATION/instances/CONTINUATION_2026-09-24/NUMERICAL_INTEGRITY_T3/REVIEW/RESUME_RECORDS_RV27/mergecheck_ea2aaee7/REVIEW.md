# RV27 — final merge check at ea2aaee7

**PASS. No new actionable finding; RV27-S1 remains closed.** The prior full-diff
review and correction backcheck carry to candidate
`ea2aaee702624cd612eee23a7e1969fd3c670d3f` against new main
`a38617d08bfbff6ea0a15b1cb129a16cbc82b2fc`. Exact-final-head CI and Mac DEC-025
remain ROOT's required gates before merge; this check does not waive them.

The clean local candidate and live PR1068 head/base match those identities.
The merge's ordered parents are prior reviewed candidate
`de9f8d83bdaab01b2bb988b78791f608adda61e6` and new main `a38617d...`; their
merge base is the original `d01ad98a754698631f927709d08284c272de85e8`.

There are 322 reviewed changed paths and 52 incoming paths, with no overlap.
Every incoming path is under projects/chirality-app-v4/execution/. An independent
in-memory union of all file modes/blob identities from the two parents equals
the entire resulting Git tree; there is no unexplained resolution or merge-added
change. The combined merge diff is empty. No Git remerge/object-writing operation
was used to establish that result.

The complete PR raw diff and binary patch are identical to the previously
reviewed Piping diff. Both binary patches are 65,142,722 bytes with SHA256
`84188c71e562af315e15ebeecdbcfcc824132739d6cec8c5019e859cf0a136c4`.
Piping, tools, .github, Root AGENTS.md, agents, skills and workflows retain their
exact prior tree/blob identities. No numerical source, evidence or instruction
change is introduced by this merge.

The original review seal (17 entries), correction-backcheck seal (10 entries),
and preservation seal (293 entries) all verify and retain their original hashes.
The prior archive restoration and original/sanitized-copy checks therefore still
cover identical evidence. Earlier review records and seals were not edited.

Fresh GEN-8 passed on the exact final clean head: 1 passed, 10 deselected.
The original stdout/timing is evidence/gen8.log. Commands, complete PR raw diff,
incoming paths, per-tree identities and checks are retained in this additive
packet. No Rust, solver, DEC-025, Git/index mutation, host-tool change, delegation,
App-v4 design acceptance, personal owner-review claim or model-diversity claim
was made. All Git reads used GIT_OPTIONAL_LOCKS=0.

This is records/merge review coverage only. A1 remains open, F2a reliance held,
and K6c unaccepted. Subsequent candidate changes still require review coverage.
