#!/bin/bash
# I82 B1-S step 0: reproduce the c = 1 TEXT chain at the code basis (main d8c88774d0), Python only.
# Copies u4_g7_06's chain (the tools I72's U8 Pass B ran), carries its line-keyed rules from Pass A
# (ba1faa1c85) to the basis with I65's g7_linemap.py, regenerates the template inventory, and runs
# run_text_part2.sh (callgraph, lexicon, sens.py at l <= 128, eps 2). No cargo, no Git writes.
# Usage: I82_WT=<WT> b1_text_base.sh   (writes only under WT/scratch/i82_b1_study/)
set -euo pipefail
T=${I82_WT:?}; S=$T/scratch/i82_b1_study
R0=$T/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
REC=$R0/I65/u4_g7_06/_run_records
BASIS=d8c88774d0a73bc99fe9c6e906db6296162f8953; PASS_A=ba1faa1c858ce3630a22767677310b1902a14b83
# The rules are line-mapped to U8's basis bd6b4be2c3 (I72's mapping), not to BASIS: BASIS adds
# core/rules/*/src/lib.rs edits (S-I1), which make g7_linemap's short `lib.rs:N` keys ambiguous.
# Every one of the 15 crate_dirs.txt source trees and the three static schemas are tree-identical
# between bd6b4be2c3 and BASIS (checked by b1_tree_check.sh), so the mapped rules apply to BASIS.
MAP_TO=bd6b4be2c33cc64edf3e273bc126083872d03e24
export GIT_OPTIONAL_LOCKS=0 TMPDIR=$S/tmp PATH=$T/../../projects/chirality-piping/.venv/bin:$PATH
W=$S/snap/projects/chirality-piping; RR=$S/rr; O=$S/base
rm -rf $RR $O $S/rules; mkdir -p $RR $O $S/rules
cp $REC/chain/* $RR/
python3 $REC/g7_linemap.py $T/numerics $PASS_A $MAP_TO $S/rules $REC/chain/callgraph_rules.g4.json $REC/chain/loop_bounds.g4.json $REC/chain/text_args.g4.json $REC/chain/sens.py $REC/premise_pins.json > $S/linemap.out.json
cp $S/rules/callgraph_rules.g4.json $S/rules/loop_bounds.g4.json $S/rules/text_args.g4.json $S/rules/sens.py $RR/
python3 - $W $REC/chain/crate_dirs.txt $RR <<'PY'
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
( cd $W && bash $RR/run_text_part2.sh base $O > $O/text.log 2>&1 )
A=$R0/I65/u4_g7_01/_run_records/pass_a/text_g7
for f in composite_text.caps.json g4_caps.caps.eps2.out.json ordinary_caps.caps.out.json producer_caps.caps.out.json profile_tree.json t07_repair.caps.out.json t08_closure.caps.log t25_g4.caps.eps2.out.json text_budget.caps.out.json text_budget_W.caps.out.json text_budget_X.caps.out.json text_budget_env.caps.out.json; do
  cmp -s $A/$f $O/sens_base/$f && echo "identical $f" || echo "DIFFERS $f"; done
cat $O/sens_base.summary.json; echo
