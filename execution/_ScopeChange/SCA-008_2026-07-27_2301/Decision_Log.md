# Piping SCA-008 Decision Log

| Date | Gate / act | Actor | Decision | State |
|---|---|---|---|---|
| 2026-07-27 | Gate 1 | Ryan Tufts | Confirmed package `cfa2ba572ff0d6a65ce08cf85427dd124a5d8e3eb42be482d7b431d504bf79ed` on basis `2979e3ff85a3bc8bc106c4cd01c63c5a2c7b4bc1`; preserved PRD R7, client status, topology, and history; carried COV-230/231 into Gate 2. | `CONFIRMED` |
| 2026-07-27 | Gate 2 | Ryan Tufts | Accepted corrected impact SHA-256 `c3f975a624bd52ead0914de44215b15fcbc558c4ef25b815097297ebb9f888f2`, the unchanged topology and envelope distribution, preservation envelope, and exact Gate-3 repair posture. | `ACCEPTED` |
| 2026-07-27 | Gate 3 | Ryan Tufts | Approved candidate manifest `e42f7bddabffa3b18db09dab9aef9710a1f55302df9a8cd774e87b6070786c72` and artifact manifest `528e3a0637c154aa90003da14c782438065b1499ea4d99844eded2d450758f3f`; opened Gate 4 only. | `APPROVED` |
| 2026-07-27 | Gate 4 | Ryan Tufts | Approved candidate manifest `2dd43e683027ab6704117ab363cc9d07c35fcf1a6a4f30a83f67727f5dfe206e` and artifact manifest `6bfaca0ae007cfa2690c2bf765203b49a601056156597c79117527bfdefc5047`; admitted the current-main rescan and pre-Gate-5 audit; opened Gate 5 only. | `APPROVED` |
| 2026-07-27 | Gate 5 preflight | SCOPE_CHANGE | Reproduced accepted manifests, current basis `7b0be4d8772a16e5a4774a17988479587d00acca`, four replacement preimages, sixteen required absences, D-58 authority inputs, topology, and the paired DEL-16-04 status/memory read. | `PASS` |
| 2026-07-27 | Gate 5 pre-change audit | AUDIT_DECOMP | Completed all 12 checks with 2 known active-SCA-007 blockers, 228 warnings, and 1 information finding; baseline declared eligible for pre-change use. | `ADMITTED` |
| 2026-07-27 | Gate 5 staged assembly | SCOPE_CHANGE | Assembled the exact prospective 20-path state in a disposable staging root. | `PASS` |
| 2026-07-28 | Gate 5 staged audit pass 1 | AUDIT_DECOMP | Completed all 12 checks with 0 blockers, 229 warnings, and 1 information finding; confirmed complete active SCA-008 state and clearance of the former COV-230/COV-231 blocker conditions. | `PASS_WITH_WARNINGS` |
| 2026-07-28 | Gate 5 staged post-state refresh | SCOPE_CHANGE | Replaced the provisional post-change coverage member with the exact pass-1 summary SHA-256 `49a4ecc255d950e48dfe04ea8fee593fbfc151baae867a410fbd3bac79a9a7fa` and refreshed only staged status/evidence fields. | `PASS_2_PENDING` |
| 2026-07-28 | Gate 5 staged audit pass 2 | AUDIT_DECOMP | Revalidated all 12 checks with 0 blockers, 229 warnings, and 1 information finding; confirmed 13/13 snapshot completeness, honest handoff state, unchanged topology, and stable clearance of the former COV-230/COV-231 blocker conditions. | `PASS_WITH_WARNINGS` |
| 2026-07-28 | Gate 5 live application | SCOPE_CHANGE | Applied exactly the validated twenty-path state pointer-last in the isolated worktree. | `APPLIED` |
| 2026-07-28 | Gate 5 final live audit pass 1 | AUDIT_DECOMP | Revalidated the byte-identical live-state mirror with 0 blockers, 229 warnings, and 1 information finding; confirmed 13/13 active snapshot completeness, unchanged topology/lifecycle, and clearance of the former COV-230/COV-231 blocker conditions. | `PASS_WITH_WARNINGS` |
| 2026-07-28 | Gate 5 closure backcheck | SCOPE_CHANGE | Reproduced the exact twenty-path quarantine, postimages, pointer and notice identities, SCA-007 immutability, reference and topology invariants, and clean candidate whitespace. | `PASS` |
| 2026-07-28 | Gate 5 final live audit pass 2 | AUDIT_DECOMP | Revalidated all 12 checks with 0 blockers, 229 warnings, and 1 information finding; confirmed 13/13 snapshot completeness, honest pending-confirmation state, unchanged topology and lifecycle, and stable clearance of the former COV-230/COV-231 blocker conditions. Artifact manifest SHA-256 `24db682b1a054317de38a418ad54cb59a2acc22c53cad3cce5942ad3e08a459b`; coverage summary SHA-256 `d35f339c937101927ae55e1b6a02a0f25dc0b98c6e0bf3718fea2d9a084d4eb1`. | `PASS_WITH_WARNINGS` |
| 2026-07-28 | Gate 5 owner confirmation | Ryan Tufts | Confirmed the post-change state and accepted `CLOSED_FOR_SCOPE_CHANGE_ONLY`; accepted decomposition revision 0.11, DEC-091, the lifecycle-neutral DEL-16-04 correction, the complete snapshot, blocker clearance, SCA-007 preservation, routed notices, frozen downstream reruns, and `ReadyForNextPhase=REGEN_ONLY`. | `ACCEPTED` |

The actual Gate-1 through Gate-4 owner rulings are transcribed in
`ACCEPTANCE_RECORD.md`. Application and closure entries may advance only from
deterministic staged and live audit evidence.
