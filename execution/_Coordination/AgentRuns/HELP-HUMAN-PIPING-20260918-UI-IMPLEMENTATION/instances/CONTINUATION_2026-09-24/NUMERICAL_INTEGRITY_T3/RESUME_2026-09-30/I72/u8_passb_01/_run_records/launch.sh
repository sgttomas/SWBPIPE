#!/bin/bash
# I72 U8-4: launch the whole Pass B as one job under the T3 cargo lock.
# Usage: I65_T=WT bash launch.sh   (the basis is a git archive of REV's projects/chirality-piping without
# execution/, at WT/scratch/i72_u8/basis; outputs WT/scratch/i72_u8/run.out.txt and pass_u8/)
set -u
T=${I65_T:?set I65_T to the worktree root}; HERE=$(cd "$(dirname "$0")" && pwd)
REV=bd6b4be2c33cc64edf3e273bc126083872d03e24; TAG=u8; B=$T/scratch/i72_u8/basis
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
echo "$(date -u '+%FT%TZ') WAIT pid=$$ cwd=$PWD args: I72 g7_pass.sh (whole Pass B) $B $REV $TAG" >> "$T/guard/cargo_jobs.log"
cd "$T/scratch/i72_u8"
env -u RUSTFLAGS I65_T="$T" CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 GIT_OPTIONAL_LOCKS=0 \
  /usr/bin/lockf -k "$T/guard/cargo_job.lock" /bin/bash "$HERE/i72_locked_job.sh" "$B" "$REV" "$TAG"
rc=$?; echo "launch: exit=$rc $(date -u '+%FT%TZ')"; exit $rc
