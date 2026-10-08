#!/bin/sh
# I97 B2-C revision 01: collision check of the names revision 01 introduces or adds to the reservation table
# (read-only Git; records only).
# Usage: sh b2c_collisions_r1.sh <NUM>   (prints one line per name: name|form|files outside P/execution|first files)
set -u
NUM="$1"
P=projects/chirality-piping
export GIT_OPTIONAL_LOCKS=0
check() {
  form="$1"; name="$2"
  files=$(git -C "$NUM" grep -l -F -e "$name" HEAD -- "$P" ":!$P/execution" | sed "s#^HEAD:$P/##")
  count=$(printf '%s' "$files" | grep -c . || true)
  first=$(printf '%s\n' "$files" | head -4 | tr '\n' ' ')
  echo "$name|$form|$count|$first"
}
echo "# NUM HEAD $(git -C "$NUM" rev-parse HEAD)"
# N-14: the CombinationReason tag and its member (already in v0's SCHEMA text; added to the reservation table)
check quoted '"count_range"'
check quoted '"name"'
# 07o base ids (corpus) and witness labels
for n in b2_c1_range_mechanics W-CB4a W-CB4b W-CB1z b2_base_withheld b2_pre_source_refusal \
         b2_operand_preparation_failure b2_operand_source_unavailable; do check bare "$n"; done
# Suggested internal names (lanes P and A; not wire values)
for n in w1_combinations_admitted combination_observables CombinationObservables; do check bare "$n"; done
