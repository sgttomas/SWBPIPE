#!/bin/bash
# I65 U4 G5, RV83 R-4: rebuild the D1 call graph with the R-4 resolver repairs at G4's basis
# (NUM b1f80234dc, git-archive snapshot), then rerun TEXT (the whole cap-priced chain at
# l <= 128, ε = 2) and the recursion inventory on it. G4's committed scripts are used
# unchanged; the only overlays are callgraph_g5.py and loop_bounds.r4.json (one added rule).
# Usage (from the snapshot's projects/chirality-piping): run_r4.sh <G4 _run_records> <scratch dir>
set -euo pipefail
H="$(cd "$(dirname "$0")" && pwd)"
G4="$1"; out="$2"; mkdir -p "$out/tools"
cp "$G4"/*.py "$G4"/callgraph_rules.g4.json "$G4"/text_args.g4.json "$G4"/template_inventory_head.out.json "$G4"/crate_dirs.txt "$out/tools/"
cp "$H/loop_bounds.r4.json" "$out/tools/loop_bounds.g4.json"
ROOT=run_linear_static_preview_value_with_retained_direct
graph() {  # <tag> <extra env>
  env PYTHONPATH="$G4" CG_REPAIR=1 CG_EXTRA_DEPS="core/product_physics:core/reporting/result_export" CG_RULES="$G4/callgraph_rules.g4.json" \
    CG_EDGES_OUT="$out/edges_$1.json" CG_AUDIT_OUT="$out/audit_$1.json" ${2:-} python3 "$H/callgraph_g5.py" . \
    "$out/cg_$1.out.json" $ROOT $(cat "$G4/crate_dirs.txt") > /dev/null
}
graph r4
graph r4_g4mode CG_R4=0
graph r4_implicit CG_IMPLICIT=1
graph r4_general CG_R4_GENERAL=1
PYTHONPATH="$G4" python3 "$G4/text_lexicon.py" . "$out/edges_r4.json" $ROOT "$out/lexicon_r4.json" > /dev/null
python3 "$out/tools/sens.py" "$out/sens_r4" "$(pwd)" "$out/edges_r4.json" "$out/lexicon_r4.json" 2 '{"l": 128}' > "$out/sens_r4.summary.json"
# The general rebinding rule, measured only (its loose name fan-out has no adjudication rules yet).
PYTHONPATH="$G4" python3 "$G4/text_lexicon.py" . "$out/edges_r4_general.json" $ROOT "$out/lexicon_r4_general.json" > /dev/null
python3 "$out/tools/sens.py" "$out/sens_r4_general" "$(pwd)" "$out/edges_r4_general.json" "$out/lexicon_r4_general.json" 2 '{"l": 128}' > "$out/sens_r4_general.summary.json" || true
