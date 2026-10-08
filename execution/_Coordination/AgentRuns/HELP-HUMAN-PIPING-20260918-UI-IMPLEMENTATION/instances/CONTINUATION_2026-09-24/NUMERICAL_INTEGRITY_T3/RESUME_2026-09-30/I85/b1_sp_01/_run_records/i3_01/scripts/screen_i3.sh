#!/bin/bash
# I85 B1-SP I3: screen the I3 record (I3_01.md, SHA256SUMS.i3_01, _run_records/i3_01/) with the
# strict pattern and the machine's host name (and `.local`); list links, `build` folders and .gz
# files; and run `git status --ignored` on the folder (read-only: GIT_OPTIONAL_LOCKS=0).
WT=WT
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I85/b1_sp_01
H=$(hostname); D=${H#*.}
# The strict pattern, written in pieces so that this script, copied into the records, does not match itself.
PAT='~''/|/Us''ers/|/pri''vate/|\.cla''ude/worktrees|swbpipe-''control-layer'
cd $R
T="I3_01.md SHA256SUMS.i3_01 _run_records/i3_01"
echo "strict pattern hits: $(grep -rlE "$PAT" $T 2>/dev/null | wc -l | tr -d ' ')"
grep -rnE "$PAT" $T 2>/dev/null | cut -c1-200 | head -20
echo "host name hits (the full name, and its domain part): $(grep -rliF -e "$H" -e "$D" $T 2>/dev/null | wc -l | tr -d ' ')"
echo ".local hits: $(grep -rlF '.local' $T 2>/dev/null | wc -l | tr -d ' ')"
grep -rnF '.local' $T 2>/dev/null | cut -c1-200 | head -10
echo "links: $(find $T -type l 2>/dev/null | wc -l | tr -d ' '); build folders: $(find $T -type d -name build 2>/dev/null | wc -l | tr -d ' '); gz files: $(find $T -name '*.gz' 2>/dev/null | wc -l | tr -d ' ')"
echo "git status --ignored:"; cd $WT/numerics && GIT_OPTIONAL_LOCKS=0 git status --short --ignored --untracked-files=all -- $R/I3_01.md $R/SHA256SUMS.i3_01 $R/_run_records/i3_01 2>/dev/null | awk '{print $1}' | sort | uniq -c | sed 's/^ */  /'; echo "  (?? = untracked, to be committed by ROOT; !! = ignored)"
