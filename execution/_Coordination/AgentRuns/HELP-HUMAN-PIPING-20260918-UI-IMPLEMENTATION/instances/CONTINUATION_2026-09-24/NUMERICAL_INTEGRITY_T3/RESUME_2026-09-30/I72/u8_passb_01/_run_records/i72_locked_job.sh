#!/bin/bash
# I72 U8-4: the one locked job. Runs under the T3 host lock, started as
#   /usr/bin/lockf -k WT/guard/cargo_job.lock /bin/bash <this> <basis dir> <basis rev> <tag>
# with I65_T=WT in the environment and RUSTFLAGS unset. Logs START and END to WT/guard/cargo_jobs.log in
# t3_cargo_run.sh's format (the caller logs WAIT), then runs the retargeted g7_pass.sh beside this file.
# It does not call WT/tools/t3_cargo.sh (that would wait on the lock this job holds).
set -u
T=${I65_T:?set I65_T to the worktree root}; LOG=$T/guard/cargo_jobs.log; HERE=$(cd "$(dirname "$0")" && pwd)
echo "$(date -u '+%FT%TZ') START pid=$$ cwd=$PWD args: I72 g7_pass.sh (whole Pass B) $*" >> "$LOG"
bash "$HERE/g7_pass.sh" "$@"; rc=$?
echo "$(date -u '+%FT%TZ') END rc=$rc pid=$$ cwd=$PWD" >> "$LOG"
exit $rc
