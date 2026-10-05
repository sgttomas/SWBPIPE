#!/bin/bash
# I65 U4 G5 part 2: the T08 text run at the G5 code basis (NUM 1e323058f3, git-archive snapshot),
# on the R-4 call graph, then the whole cap-priced chain at l <= 128, eps = 2 (sens.py).
# The rules are G4's (with R-4's one added loop rule), line-mapped b1f80234dc -> 1e323058f3 by
# linemap.py (linemap_*.out.json), plus the part-2 rule changes the record lists.
# Usage (from the snapshot's projects/chirality-piping): run_text_part2.sh <tag> <scratch dir>
set -euo pipefail
H="$(cd "$(dirname "$0")" && pwd)"
tag="$1"; out="$2"; mkdir -p "$out"
ROOT=run_linear_static_preview_value_with_retained_direct
env CG_REPAIR=1 CG_EXTRA_DEPS="core/product_physics:core/reporting/result_export" CG_RULES="$H/callgraph_rules.g4.json" \
  CG_EDGES_OUT="$out/edges_$tag.json" CG_AUDIT_OUT="$out/audit_$tag.json" python3 "$H/callgraph_g5.py" . \
  "$out/cg_$tag.out.json" $ROOT $(cat "$H/crate_dirs.txt") > /dev/null
python3 "$H/text_lexicon.py" . "$out/edges_$tag.json" $ROOT "$out/lexicon_$tag.json" > /dev/null
python3 "$H/sens.py" "$out/sens_$tag" "$(pwd)" "$out/edges_$tag.json" "$out/lexicon_$tag.json" 2 '{"l": 128}' > "$out/sens_$tag.summary.json"
