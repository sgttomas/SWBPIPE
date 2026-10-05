#!/bin/bash
# G6 repair: build the audit table from the latest run, insert it into text_args, rerun TEXT (whole).
# Record note: while this ran, D (_run_records) held the repaired text_budget.py and
# text_args.g4.json; they are kept here in _run_records_g6r/, and _run_records/ holds the sealed G6
# originals again. To reproduce: copy _run_records/, overlay _run_records_g6r/{text_budget.py,
# text_args.g4.json}, run with D pointing at that copy. Run three times (g6r1, g6r2, g6r): the
# table converged at the third (FID 2,233 -> 188, ENTRY 8,352; the third table equals the second).
set -euo pipefail
S=WT/scratch/i65_u4_g6_01; D=WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I65/u4_g6_01/_run_records; SNAP=WT/scratch/i65_u4_g5_01/snap_1e32/projects/chirality-piping
caps="$1"; tag="$2"
cd $S && python3 id_table.py $S "$caps" id_audit_table.json id_audit_rows.md $D/text_args.g4.json
python3 - <<'PY'
import json
S="WT/scratch/i65_u4_g6_01"; D="WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I65/u4_g6_01/_run_records"
ta=json.load(open(D+"/text_args.g4.json")); ta["id_audit"]=json.load(open(S+"/id_audit_table.json"))
json.dump(ta,open(D+"/text_args.g4.json","w"),indent=1)
PY
pgrep -f memguard.sh >/dev/null || { echo "memguard not running"; exit 3; }
rm -rf $S/p4/sens_$tag
cd $SNAP && bash $D/run_text_part2.sh $tag $S/p4 > $S/p4/$tag.log 2>&1 || true
python3 -c "
import json; t=json.load(open('$S/p4/sens_$tag/text_budget.caps.out.json')); print('complete',t['complete'],'TAV',t['total_text_requested_bytes']); [print(u) for u in t['unclassified_args'][:30]]"
