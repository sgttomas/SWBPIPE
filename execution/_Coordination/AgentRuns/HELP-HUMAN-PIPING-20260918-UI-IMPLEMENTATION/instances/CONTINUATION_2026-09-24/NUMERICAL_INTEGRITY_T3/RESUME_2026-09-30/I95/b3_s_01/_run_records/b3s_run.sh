#!/bin/bash
# I95 B3-S: one point. I82's b1_mc_run.sh with the scratch rebound and the chain selectable:
# CHAIN=mc_chain (I82's multi-case chain, unchanged) or CHAIN=ex_chain (b3s_exact_chain.py's exact-route rebinding).
# Runs the chain's sens.py at one cap vector on the basis snapshot, reusing step 0's call graph and lexicon,
# then evaluates the profile tree with the registered build's in-build atoms (I82's b1_eval.py, unchanged).
# Usage: I95_WT=<WT> CHAIN=<chain> b3s_run.sh <tag> '<G4_CAPS json>'   (Python only; writes WT/scratch/i95_b3_s/runs/<chain>/<tag>)
set -euo pipefail
T=${I95_WT:?}; S=$T/scratch/i95_b3_s; TAG=$1; CAPS=$2; CH=${CHAIN:?}
R0=$T/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
REC=$R0/I82/b1_cap_study_01/_run_records
LAW=$R0/I72/u8_passb_01/_run_records/runs/u8/law_record.txt
export TMPDIR=$S/tmp PATH=$T/../../projects/chirality-piping/.venv/bin:$PATH PYTHONDONTWRITEBYTECODE=1
O=$S/runs/$CH/$TAG; rm -rf $O; mkdir -p $O
W=$S/snap/projects/chirality-piping
EDGES=${EDGES:-$S/base/edges_base.json}
( cd $W && python3 $S/$CH/sens.py $O/work "$W" $EDGES $S/base/lexicon_base.json 2 "$CAPS" > $O/summary.json )
python3 -c "
import json, sys
sys.path.insert(0, '$REC')
import b1_eval
t = json.load(open('$O/work/profile_tree.json')); a = b1_eval.load_atoms('$LAW')
miss = sorted({x for f in t['forms'].values() for x in f if x != '1' and x not in a and not x.startswith('Text(')})
r = b1_eval.evaluate(t, a)
out = {'caps': json.loads('$CAPS'), 'missing_atoms': miss, 'summary': json.load(open('$O/summary.json')),
       'text_atoms': t['text_atoms'],
       'modes': {m: {'phase': r[m]['phase'], 'E_mov_max': r[m]['E_mov_max'], 'E_mov_plus_R': r[m]['E_mov_plus_R'],
                     'phases': r[m]['phases'], 'components': r[m]['components']} for m in r}}
json.dump(out, open('$O/inbuild.json', 'w'), indent=1)
print(json.dumps({'chain': '$CH', 'tag': '$TAG', 'caps': out['caps'], 'missing': miss, 'D': out['summary']['D'], 'D_converged': out['summary'].get('D_converged'),
                  'complete': out['summary']['text_complete'],
                  'sparse': [r['sparse']['phase'], r['sparse']['E_mov_plus_R']], 'dense': [r['dense']['phase'], r['dense']['E_mov_plus_R']]}))
"
