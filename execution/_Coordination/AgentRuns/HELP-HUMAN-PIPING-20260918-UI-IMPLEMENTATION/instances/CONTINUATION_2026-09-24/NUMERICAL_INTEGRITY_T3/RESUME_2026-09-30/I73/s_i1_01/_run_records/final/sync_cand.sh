#!/bin/bash
# Overlay the worktree's changed and new files onto the scratch candidate tree.
set -eu
WT=<WT>
cd $WT/s-i1
for f in $(GIT_OPTIONAL_LOCKS=0 git status --porcelain --untracked-files=all 2>/dev/null | awk '{print $2}'); do
  mkdir -p "$WT/scratch/i73_s_i1/cand/$(dirname "$f")"
  cp "$f" "$WT/scratch/i73_s_i1/cand/$f"
  echo "synced $f"
done
