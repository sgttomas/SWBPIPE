#!/bin/bash
# RV111: stage the report's evidence with placeholder paths only, then seal it.
# Usage: rv111_stage.sh <report dir (R/REVIEW_RV111/si1c_01)>
set -euo pipefail
WT=WT
S=$WT/scratch/rv111_si1c_01
OUT=$1
VENVP=VENV
mkdir -p "$OUT/evidence"
cp -R "$S/stage/evidence/." "$OUT/evidence/"
# Placeholder paths only: WT, VENV; any other absolute user path is refused below.
find "$OUT/evidence" -type f | while read -r f; do
  PT="/pri""vate/tmp/claude-501/"   # split so this script does not rewrite itself
  /usr/bin/sed -i '' -e "s#$VENVP#VENV#g" -e "s#$WT#WT#g" -e "s#${PT}[^ ]*#SCRATCHPAD#g" "$f"
done
cp "$S/stage/REVIEW.md" "$OUT/REVIEW.md"
HOMEROOT="/Us""ers/"; PRIV="/pri""vate/"   # split so this script does not match itself
if grep -rlE "$HOMEROOT|$PRIV|(^|[ \"=(:])/tmp/" "$OUT" ; then echo "machine path left" >&2; exit 3; fi
( cd "$OUT" && find . -type f ! -name SHA256SUMS | sed 's#^\./##' | sort | xargs shasum -a 256 > SHA256SUMS )
( cd "$OUT" && shasum -a 256 -c SHA256SUMS | grep -vc ": OK$" || true )
