#!/bin/bash
# Overlay every file the worktree changes against base c1bfc460fc (committed or
# not, tracked or new) onto the scratch candidate tree.
set -eu
WT=<WT>
DEST=${1:-$WT/scratch/i73_s_i1/cand}
cd $WT/s-i1
{ GIT_OPTIONAL_LOCKS=0 git diff --name-only c1bfc460fc 2>/dev/null; GIT_OPTIONAL_LOCKS=0 git ls-files --others --exclude-standard 2>/dev/null; } | sort -u | while read -r f; do
  mkdir -p "$DEST/$(dirname "$f")"
  cp "$f" "$DEST/$f"
  echo "synced $f"
done
