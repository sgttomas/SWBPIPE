#!/bin/bash
# RV98 (U8 Pass B confirmation): an independent re-run of u4_g7_06's g7_pass.sh gates on RV98's own
# git-archive extract of the U8 head, WITHOUT cargo (RV98 holds no cargo grant). Derived from I72's
# path-retargeted copy (itself u4_g7_06's g7_pass.sh plus path retargets). Differences from u4_g7_06:
#  (1) outputs under WT/scratch/rv98_u8_01/pass_rv98/;
#  (2) the three cargo steps are not run. The build gates (law, pp_outcomes, runner_outcomes, witnesses,
#      challenge) are evaluated by the SAME checks on I72's recorded build outputs (I72L, I72O below),
#      and price_delta reads I72's recorded law log;
#  (3) nothing else: tools, rules, references and the 11-entry reviewed table are read from u4_g7_06
#      (REC) and u4_g7_01 (REC_A); finish(), the VERDICT line and exit code, the early stops, the
#      text_summary gate, the delta-tool failure mapping and no-inventory stop, and guard() are kept.
# Usage: I65_T=<WT> rv98_g7_nobuild.sh <basis dir holding projects/chirality-piping> <basis rev> <tag>
set -u
T=${I65_T:?set I65_T to the worktree root}; BASIS=$1; REV=$2; TAG=$3; OVERLAY=
[[ "$TAG" =~ ^[A-Za-z0-9_]+$ ]] || { echo "tag must be [A-Za-z0-9_]+"; exit 1; }
R0=$T/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
REC=$R0/I65/u4_g7_06/_run_records; REC_A=$R0/I65/u4_g7_01/_run_records
REV87=$R0/REVIEW_RV87/u4_g6_02/_run_records/rv87_g6r_noncandidates.out.json
NUMREPO=$T/numerics; PASS_A_REV=ba1faa1c858ce3630a22767677310b1902a14b83; REGISTERED_REV=0c7827b6ad
I72L=$T/scratch/i72_u8/logs; I72O=$T/scratch/i72_u8/pass_u8   # I72's recorded build outputs (read only)
O=$T/scratch/rv98_u8_01/pass_$TAG; WK=$O/work; RR=$O/rr
rm -rf $O; mkdir -p $O/logs $O/tmp; : > $O/verdict.tsv
export GIT_OPTIONAL_LOCKS=0 TMPDIR=$O/tmp
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
say() { echo "== $* ($(date -u +%FT%TZ))"; }
gate() { local name=$1; shift; python3 $REC/pass_checks.py "$@" > $O/gate_$name.json; local c=$?
  printf "%s\t%s\n" "$name" "$c" >> $O/verdict.tsv; say "gate $name: code $c $(head -c 300 $O/gate_$name.json)"; return $c; }
finish() {
  local code=$(awk -F'\t' '$2==2||$2==3||$2==4||$2==5{print $2; exit}' $O/verdict.tsv)
  [ -z "$code" ] && code=$(awk -F'\t' '$2!=0{print 6; exit}' $O/verdict.tsv); [ -z "$code" ] && code=0
  local word=PASS; [ $code -ne 0 ] && word="STOP"; [ $code -eq 6 ] && word="DELTAS TO READ"
  echo "VERDICT $word exit=$code basis=$REV tag=$TAG overlay=${OVERLAY:-none} gates=$(awk -F'\t' '{printf "%s:%s ", $1, $2}' $O/verdict.tsv)" | tee $O/VERDICT.txt
  exit $code; }
guard
rm -rf $WK; mkdir -p $WK; cp -R $BASIS/projects $WK/
git -C $NUMREPO ls-tree -r $REV -- projects/chirality-piping | grep -v "projects/chirality-piping/execution/" | awk '$2=="blob"{print $3"\t"$4}' > $O/tree_blobs.tsv
gate tree tree $O/tree_blobs.tsv $WK || finish
W=$WK/projects/chirality-piping; P=$W/core/product_physics; RM=$P/src/retained_memory.rs
git -C $NUMREPO show $REGISTERED_REV:projects/chirality-piping/core/product_physics/src/retained_memory.rs > $O/registered_retained_memory.rs
gate entry entry $RM $O/registered_retained_memory.rs || finish
guard
# (2) no cargo: the law gate reads I72's recorded law log against THIS extract's entry
gate law law $I72O/logs/law.log $RM || finish
python3 $REC/statics_list.py $W $REC/chain/crate_dirs.txt $REC/reference/statics_a2.json > $O/statics.json
gate statics statics $O/statics.json
mkdir -p $RR; cp $REC/chain/* $RR/
python3 $REC/g7_linemap.py $NUMREPO $PASS_A_REV $REV $O/rules $REC/chain/callgraph_rules.g4.json $REC/chain/loop_bounds.g4.json $REC/chain/text_args.g4.json $REC/chain/sens.py $REC/premise_pins.json > $O/linemap.out.json
lm=$?; printf "linemap\t%s\n" $([ $lm -eq 0 ] && echo 0 || echo 4) >> $O/verdict.tsv; say "linemap: exit $lm"; [ $lm -eq 0 ] || finish
cp $O/rules/callgraph_rules.g4.json $O/rules/loop_bounds.g4.json $O/rules/text_args.g4.json $O/rules/sens.py $RR/
gate premise premise $O/rules/premise_pins.json $W || finish
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
guard
( cd $W && bash $RR/run_text_part2.sh pb $O > $O/logs/text.log 2>&1 ); tc=$?
printf "text_run\t%s\n" $([ $tc -eq 0 ] && echo 0 || echo 6) >> $O/verdict.tsv; say "TEXT chain exit $tc"
D=$(python3 -c "import json;print(json.load(open('$O/sens_pb.summary.json'))['D'])" 2>/dev/null || echo 0); echo $D > $O/D.txt; say "D from the run: $D"
[ "$D" -gt 0 ] 2>/dev/null || { printf "text_summary\t6\n" >> $O/verdict.tsv; say "the TEXT chain produced no summary"; }
python3 $REC/delta_inventory2.py $NUMREPO $PASS_A_REV $REV $W $REC/chain/crate_dirs.txt $O/edges_pb.json $O/sens_pb/loop_bounds.g4.json $REC/delta_reviewed.json $O/delta_inventory.json > $O/delta.out.txt
dc=$?; [ $dc -ne 0 ] && [ $dc -ne 5 ] && [ $dc -ne 6 ] && dc=6
printf "delta\t%s\n" $dc >> $O/verdict.tsv; say "delta: exit $dc $(head -1 $O/delta.out.txt)"
[ $dc -eq 6 ] && [ ! -s $O/delta_inventory.json ] && finish
[ $dc -eq 5 ] && finish
gate text text $O pb $REC_A/pass_a/text_g7 $REC/text_row_diff.py
gate forms forms $RM $REC_A/pass_a/text_g7/profile_tree.json $RR/g5_profile.py $O
python3 $REC/price_delta.py $O/sens_pb/profile_tree.json $I72O/logs/law.log > $O/price_delta.out.json
( cd $W && env G4_CAPS='{"l": 128}' TB_LEXICON=$O/lexicon_pb.json TB_COMPOSITE=$O/sens_pb/composite_text.caps.json TB_D=$D TB_NONCAND_OUT=$O/noncand.json python3 $O/sens_pb/text_budget.py $W $O/sens_pb/template_inventory_head.out.json $O/edges_pb.json $O/sens_pb/loop_bounds.g4.json $O/sens_pb/text_args.g4.json run_linear_static_preview_value_with_retained_direct caps > $O/whole_nc.out.json ); nc=$?
printf "noncand_run\t%s\n" $([ $nc -eq 0 ] && echo 0 || echo 6) >> $O/verdict.tsv; say "section 11 run exit $nc"
python3 $REC/noncand_compare.py $REV87 $O/noncand.json > $O/noncand_compare.out.json
gate noncand noncand $O/noncand_compare.out.json
( cd $W && TB_D_RUN=$D python3 $REC/audit_controls_g7.py $O/sens_pb $W $O/edges_pb.json $O/lexicon_pb.json $O/ctl > $O/controls.out.txt ); cr=$?
printf "controls_run\t%s\n" $([ $cr -eq 0 ] && echo 0 || echo 6) >> $O/verdict.tsv; say "controls run exit $cr"
gate controls controls $O/ctl/audit_controls_g7.out.json 12
# (2) no cargo: item 5's gates on I72's recorded outputs
gate pp_outcomes outcomes $REC/reference/a_pp.outcomes $I72L/pass_u8_pp.outcomes
gate runner_outcomes outcomes $REC/reference/a_runner.outcomes $I72L/pass_u8_runner.outcomes
w=$(grep -c "^== witness_.* exit=0" $I72O/logs/cargo.out); printf "witnesses\t%s\n" $([ "$w" = 9 ] && echo 0 || echo 6) >> $O/verdict.tsv; say "witnesses: $w/9 exit 0"
c=$(grep -c "^test result: ok. 1 passed" $I72L/pass_u8_challenge.log); printf "challenge\t%s\n" $([ "$c" = 1 ] && echo 0 || echo 6) >> $O/verdict.tsv; say "challenge: $c"
finish
