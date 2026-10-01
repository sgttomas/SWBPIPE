# G1 runtime04 — correct G05 location, independently release every group

I22's runtime03 applied K4-M17 to the wrong identical-looking source context
and launched Cargo despite a visible postimage mismatch. This violated the
required prelaunch gate. Compilation was interrupted before any test; the
wrong source/launch/termination remain sealed, with no numerical credit.
G02–G04 intended semantic results/controls remain separately reviewable.

ROOT read the wrong-vs-frozen diff and created fresh G05_r1 from the immutable
E1 baseline, verifying all116 original files and distinct inodes. Existing
apply_patch used the distinguishing following Spent context to insert the
same frozen work.record line only in solve_case_at. Correct postimage:
a7a6f4060a1d1f8d06fe91cc002a5a26e8c20038b0f3612d359c77b2f8cb092a.
The old wrong G05 is preserved. New source/target:
scratch/i22/protected_g1/G05_r1 and a1-protected-target/g1/G05_r1.

Existing I22 may resume G05_r1 then G06–G54 only under these stricter controls:
BEFORE EACH Cargo launch, manager independently reads/verifies all exact
affected-file postimages, unchanged protected tests/lock, resolved source/
target/log paths and guard. Manager sends an explicit release for that one
group. A child-displayed hash without manager verification is not release.
Any mismatch stops before Cargo. No new runner/framework/host tool is needed.
Use existing apply_patch with enough function/context to identify the frozen
site; the hash, not apparent matching text or exit status, decides correctness.

Manager first verifies ROOT's G05_r1 complete source against frozen intended
patch and releases that one test. Do not reapply the patch. Keep logs in the
writable sibling runtime_04/logs, outside every source archive. Separate mkdir
and source checks must succeed before dependent commands. Each group retains
its exact filter and untouched-baseline control, all original semantic gates,
normal cleared environment, existing guard and one Cargo at a time.
Original14:10:19UTC deadline stays; no automatic extension.

Write only additive I22/protected_g1/runtime_04, manager/protected_g1/runtime_04,
and granted owned scratch/targets. No maintained edits, changed criteria,
alternate test/search, V-K/G2/runtime expansion, Git/index writes or delegation.
Return any new failure promptly. ROOT/RV29 will review G02–G04 and subsequent
results independently; this execution repair does not accept numerical truth.

