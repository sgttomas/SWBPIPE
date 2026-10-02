# I21 kernel mutants 26 — stopped at first survivor

**M05_UC_C_OVERLAP survived.** The mutant programme stopped immediately, as
required. NONE passed; four earlier mutants compiled and were killed by actual
numerical assertions. This is an incomplete required mutant programme, not a
passing checkpoint. The owner subsequently requested a graceful halt; no further
variant, witness, test repair or build was started.

The exact source is9086964a1fb656a76cda6d1002d8594efa636fdc, extracted by git archive
into owned scratch. No mutable maintained source was copied or consumed. The
complete core subtree, supplied Root/TASK/Piping instructions and selected proof
records are bound in `_run_records/INPUT_BINDING.json`. Native TASK
`/root/i21_kernel_resume` remained a direct child of ROOT `/root`; no delegation,
maintained edit, Git/index write, installation, guard change or performance run.
The new45-minute grant began2026-10-02 02:13:18 UTC; new-variant cutoff02:53:18,
hard end02:58:18. Completion is in `_run_records/VERIFICATION.json`.

The test set was fixed before the NONE control and unchanged throughout:

1. `cargo test --release --test k6c_envelope`: all nine existing arithmetic,
   owner-phase, descriptor/error and capacity regressions.
2. `cargo test --release --test k6b_w1 the_committed_counts_carry_this_codes_estimate -- --exact`:
   the existing exact-field/reference test over all33 H counts rows. It evaluates
   integer reference arithmetic, not33 model/count/solver runs.

All commands additionally used installed Rust1.97.1, auto-install0, locked/offline,
incremental0, -j4, testthreads2, the existing M5 guard and a separate target.
Each executed variant has its actual diff, original/mutant source hashes, fixed
test hashes, complete argv/environment, unabridged output, compile status,
assertion excerpt and test-binary hashes in its `_run_records/<id>` directory.
Every mutant binary hash differs from NONE. No compilation failure is a kill.

| Variant | Actual active formula mutation | Observed result |
|---|---|---|
| NONE | No change | Nine kernel tests and the H33 test passed. |
| M01_NL_ONE_VECTOR | nl_pass `3*f*w` to `2*f*w` | Kernel tests passed; H exact pass fields failed. This is one missing equal-width vector, not three independent kills. |
| M02_NL_ALL_THREE | nl_pass `3*f*w` to zero, preserving other shift alternatives | Kernel tests passed; H exact pass fields failed with different values from M01. This separately exercises the original omitted-three-vector contribution. |
| M03_DEAD_WORK | nl_pass `3*f*w` to `4*f*w`, wrongly retaining factor work there | Kernel Shift identity and H exact pass fields failed. This is an over-count identity failure, not a measured-underbound claim. |
| M04_OPTION_REPORT_PLUS8 | live five Option-backed report row arrays `5*q*w` to `5*q*(w+8)` | Kernel reference totals and H exact fields failed. Unrelated Wide owners and the unused compatibility constant were untouched. This is an over-count identity failure. |
| M05_UC_C_OVERLAP | active Uc `4*f*w+4*f` to `3*f*w+4*f` | Compiled; all nine kernel tests and the H33 exact-field test passed. **First survivor; stop.** |

For example, M01 changed the first H row's pass array from
`[157254,256966,456390]` to `[154374,252166,447750]` while its global maximum
remained22744300. M03 changed the axis fixture's Shift from
1030165/1032725 requested/moving to1030517/1033077. M04 changed the first H
row's max/sel128 from22744300/20739628 to22754820/20750148. These are test
assertions on source arithmetic; no observed heap value supplies a formula term.

M05's live site is the `vbuild` alternatives in
`H/src/k6/w1/envelope.rs`, immediately before `VerificationSharedBuild`:

    m*COEFFICIENT[i] + 16*bc
        + max(4*f*w + 4*f, 2*f*w + 2*bc*w)

The first4*f*w represents the accepted c+at+bt+ct overlap. The mutation removes
one f*w c owner; it retains the4f row map and later output alternative. The exact
patch and source-line excerpt are retained, not inferred from a description.

The masking diagnosis is independently checked on existing inputs only.
`VerificationSharedBuild` takes a maximum that also includes the same common
prefix plus144*m*w member-block construction. Across the six successful kernel
fixture policies and all33 H count rows, at48/80/144 bytes per Wide, that member
alternative already dominates the complete normal Uc alternative. All117 scalar
comparisons pass; lowering Uc therefore leaves this phase unchanged for those
inputs. The widened-member/directional alternative can only supply another
competitor. The selected tests also do not assert this Uc owner subalternative;
H's verify diagnostic is retained VerifyShared storage, not this construction
scratch. A direct phase assertion on these same dominated inputs would still
miss this mutation. This is not equivalence over the supported descriptor domain.
No suggested new member-free witness was constructed or verified.

Seven planned core variants remain **unexecuted**, preserved without invented
kills or silent waiver:

- M06_RES_EXACT: replace growing RES G16(B) by exact16B.
- M07_TOP_DROP: remove the TOP phase alternative.
- M08_HATCHECK_DROP: remove the distinct HATCHECK alternative.
- M09_SUMMARY_MISSING: omit the prepaid moved40B summary owner.
- M10_SUMMARY_DOUBLE: count that moved summary owner twice.
- M11_ACTIVE_OLD_DROP: omit the current realloc-old surcharge.
- M12_REBUILD_OLD_DROP: omit the original table during old-table/new-kept rebuild.

VR stale-port/caller mutations remain separately owned. No wider programme is
claimed complete. ROOT will decide any later narrow diagnosis/test repair after
reading this source and evidence; no automatic follow-on is authorized.

The normal source was restored after every trial. Final comparison of all572
archived files with the immutable tar found zero differences; tests never changed.
The target's last compiled artifacts are M05 and are explicitly diagnostic, not
a restored production binary. Cargo is idle and the build lane is released.
Only this additive evidence packet and owned scratch/target were written.
