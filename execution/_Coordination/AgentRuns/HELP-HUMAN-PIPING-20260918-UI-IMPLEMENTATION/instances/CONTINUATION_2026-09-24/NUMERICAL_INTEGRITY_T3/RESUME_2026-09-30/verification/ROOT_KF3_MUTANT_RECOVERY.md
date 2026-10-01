# ROOT bounded KF3 mutation-evidence recovery

Read-only recovery, no test/tool execution or source change. Checked the named
scratch/i19 and scratch/rv23 locations, the remaining KF3 worktree's exact KF3
records, top-level names containing i19/kf3/rv23, and preserved review pointers.
The initial broader worktree filename listing was noisy and was immediately
narrowed; no unrelated document bodies were read. No original I19 checkpoint-A
patch script or exact patches were recovered. Stop this search; do not invent
historical byte identity or start a host-tool recovery project.

This absence is already documented in REVIEW/KF3_REVIEW.md opening findings and
mutation section: RV23 reconstructed five I19 faults from CHECKPOINT_A section4
because their diffs were not recorded. The canonical surviving source is
REVIEW/_run_records/kf3_review/scripts/rv23_mut.py.txt, plus its confirmation
script and IMPLEMENTATION/KF3/_run_records/rv23/mutants/rv23_mut.py.txt.
CHECKPOINT_A.md section4 and RETURN.md section7 preserve all original fault
meanings and intended tests. Those historical bytes remain unchanged.

Potential semantic correspondence for a separately reviewed current replay:
- original M1: RV23-M1 and M1b cover refused Uc becoming zero at two sites;
- M2: RV23-M2 B=max;
- M3: RV23-M3b drops missing-bound stop;
- M4b/M4c: RV23-M5b/M5 drop solve_case_at/build_verify_shared close_stopped;
- M4d and M5–M8: explicitly retained I19-labelled reconstructions in RV23's script;
- M4a: the named build_shared close_stopped site needs an explicit current patch,
  checked against CHECKPOINT_A and frozen build_shared source, not an invented
  original-script claim.

These are recovery pointers/proposed semantic matches, not accepted equivalences,
new patches, executed kills or permission to retire any registered obligation.
ROOT's next grant may prepare faithful current-source patches with original IDs,
source-qualified fault descriptions and fresh independent mapping review. Broad
historical driver failure predicates cannot establish a kill: compilation and the
intended semantic assertion must be examined separately. No permanent driver or
host tool is changed by this recovery note.
