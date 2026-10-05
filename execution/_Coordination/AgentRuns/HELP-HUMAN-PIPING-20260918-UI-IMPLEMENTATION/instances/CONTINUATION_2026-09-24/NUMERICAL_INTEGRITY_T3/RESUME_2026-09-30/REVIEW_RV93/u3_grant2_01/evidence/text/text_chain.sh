#!/bin/bash
# RV93: the G7 TEXT chain (g7_pass.sh steps 3/4, TEXT and the enforced audit only) on one tree.
# usage: text_chain.sh <tree dir holding projects/chirality-piping> <tag>. Python stdlib only; no cargo.
set -u
WT=WT
S=$WT/scratch/rv93_u3_grant2_01
REC=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I65/u4_g7_01/_run_records
REV87=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/REVIEW_RV87/u4_g6_02/_run_records/rv87_g6r_noncandidates.out.json
TREE=$1; TAG=$2; O=$S/text_$TAG; RR=$O/rr; rm -rf $O; mkdir -p $RR $O/tmp; export TMPDIR=$O/tmp
W=$TREE/projects/chirality-piping
cp $REC/chain/* $RR/
python3 - $W $REC/crate_dirs.txt $RR <<'PY'
import os, sys, subprocess
W, crates, RR = sys.argv[1:4]
files = []
for c in open(crates).read().split():
    for d, _, fs in os.walk(os.path.join(W, c)):
        if "/tests" in d or "/bin" in d: continue
        files += [os.path.relpath(os.path.join(d, f), W) for f in fs if f.endswith(".rs") and not f.endswith("_tests.rs") and f != "tests.rs"]
out = subprocess.run(["python3", RR + "/template_inventory.py", "."] + sorted(files), cwd=W, capture_output=True, text=True, check=True).stdout
open(RR + "/template_inventory_head.out.json", "w").write(out)
PY
( cd $W && bash $RR/run_text_part2.sh pb $O > $O/text.log 2>&1 ); echo "text exit=$?"
cat $O/sens_pb.summary.json | head -c 400; echo
( cd $W && env G4_CAPS='{"l": 128}' TB_LEXICON=$O/lexicon_pb.json TB_COMPOSITE=$O/sens_pb/composite_text.caps.json TB_D=14734 TB_NONCAND_OUT=$O/noncand.json python3 $O/sens_pb/text_budget.py $W $O/sens_pb/template_inventory_head.out.json $O/edges_pb.json $O/sens_pb/loop_bounds.g4.json $O/sens_pb/text_args.g4.json run_linear_static_preview_value_with_retained_direct caps > $O/whole_nc.out.json ); echo "whole_nc exit=$?"
python3 $REC/noncand_compare.py $REV87 $O/noncand.json > $O/noncand_compare.out.json; python3 -c "import json;print('noncand:',json.load(open('$O/noncand_compare.out.json'))['verdict'])"
