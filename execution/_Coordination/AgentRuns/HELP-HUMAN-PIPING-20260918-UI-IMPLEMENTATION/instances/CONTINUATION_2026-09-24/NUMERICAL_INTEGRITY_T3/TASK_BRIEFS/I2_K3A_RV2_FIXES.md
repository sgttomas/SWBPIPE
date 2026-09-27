# I2: K3a fixes for RV2's SHOULD-FIX findings (S1, S2) and cheap NOTEs

A focused follow-up for I2 (resumed). `_COMMON.md` and I2's original brief apply. This follows the S11-K precedent: ROOT has SHOULD-FIX findings fixed before merge. RV2 then backchecks only the delta.

## Basis

RV2's review: `T3/REVIEW/K3A_REVIEW.md` (`9c558f533` on the T3 branch, sha256 `11419d2c…`), with its run records in `REVIEW/_run_records/k3a_review/`, including the S1 input and `r2_distinguishing_inputs.txt`.

## Worktree

`/home/user/wt/k3a`, branch `codex/piping-k3a-20260926`, at head `43da7a24e` (main already merged in). Make no Git writes; the manager commits.

## Fixes

1. **S1: the tolerance is not a bound.**
   - State plainly in `wide.rs`'s documentation, `IMPLEMENTATION/K3A/RETURN.md` §3 and `CHANGE_RECORD.md` that **6 ulp is a regression tolerance for the committed vectors only**, and that the **accuracy contract is the proved 23.6 ulp** (at 53 ≤ p ≤ 128).
   - Add RV2's 6.08-ulp input to the generator's arctangent set. Then either raise the test tolerance to cover it while staying at or below 23.6, or have the test assert the proved bound for all vectors and the tolerance only for the regression set. Say which.
2. **S2: RV2's mutant R2 (the tail summed largest first) must be killed.**
   - Add at least one of the 13 inputs in `r2_distinguishing_inputs.txt` to the generator's arctangent set.
   - Regenerate `atan.txt` and the vector SHA256SUMS **with the generator only**, and check that `gen_wide_vectors.py --check` is byte-identical.
   - Re-run R2 in scratch (RV2's patch) and show it is now killed. Also re-run I2's M5 and M6, and RV2's R1, to show they are still killed.
3. **Cheap NOTEs** (records and wording only):
   - correct the test counts (19 tests added; base total 116);
   - disclose that the raw logs keep trailing blank lines (for `git diff --check`);
   - make the bound statement mention the `ExponentRange` refusals;
   - add the checked revisions to CHANGE_RECORD.
   Leave the "proof slack" and the "rounded error measure" NOTEs as recorded. RV2's AngleDomain note (angles within about 1e-19 of π) is K-D5's; the manager routes it to I3.

## Rules

- **No change to arithmetic behaviour** (`wide.rs` code paths). Only documentation, tests, generator input sets, regenerated vectors and records change. If a fix seems to need a code change, stop and tell the manager.
- Fixture identity must stay 112/112 (no product caller exists).
- `frame_kernel` stays green. No dependency or lockfile change.
- One cargo job at a time: RV2 is finished, so you alternate with I3 and I4. Check `pgrep -x cargo` and `pgrep -f 'python[0-9.]* .*run_evidence_sweep'` first. `RUSTUP_AUTO_INSTALL=0`, `RUSTUP_TOOLCHAIN=1.97.1`, your own `CARGO_TARGET_DIR`. **Never delete the authority targets** (built in this worktree by ROOT).
- No Git writes and no index operations.

## Return

Append an "RV2 fixes" section to `IMPLEMENTATION/K3A/RETURN.md` covering:
- the files changed;
- the new vectors, and the generator check;
- the mutation re-runs (R2 killed; M5, M6 and R1 still killed);
- the frame_kernel count.

Refresh SHA256SUMS and message the manager.
