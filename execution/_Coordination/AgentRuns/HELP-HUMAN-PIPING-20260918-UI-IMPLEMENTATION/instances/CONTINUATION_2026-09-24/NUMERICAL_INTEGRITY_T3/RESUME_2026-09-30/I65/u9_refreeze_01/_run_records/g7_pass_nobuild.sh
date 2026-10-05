#!/bin/bash
# I65 U9 refreeze: the no-build part of Pass B (u4_g7_06's g7_pass.sh, with every cargo/rustc step
# removed, for a host running DEC-025). Gates: tree, entry, statics, linemap, premise, text_run, delta,
# text, forms, noncand_run, noncand, controls_run, controls. Not run (they build): law, the PP and
# runner/headless outcomes, the witnesses, the challenge; price_delta reads the given law record.
# Differences from g7_pass.sh, and nothing else: (1) the copy is a fresh `git archive <rev>` straight
# into pass_<tag>/work (no separate basis dir); (2) no cargo; (3) price_delta uses LAWREC.
# Usage: I65_T=<WT> g7_pass_nobuild.sh <rev> <tag> <law record of a build of identical inputs>
set -u
T=${I65_T:?}; REV=$1; TAG=$2; LAWREC=$3
[[ "$TAG" =~ ^[A-Za-z0-9_]+$ ]] || { echo "tag must be [A-Za-z0-9_]+"; exit 1; }
R0=$T/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
REC=$R0/I65/u4_g7_06/_run_records; REC_A=$R0/I65/u4_g7_01/_run_records
REV87=$R0/REVIEW_RV87/u4_g6_02/_run_records/rv87_g6r_noncandidates.out.json
NUMREPO=$T/numerics; PASS_A_REV=ba1faa1c858ce3630a22767677310b1902a14b83; REGISTERED_REV=0c7827b6ad
O=$T/scratch/i65_u4_g7_01/pass_$TAG; WK=$O/work; RR=$O/rr
rm -rf $O; mkdir -p $O/logs $O/tmp $WK; : > $O/verdict.tsv
export GIT_OPTIONAL_LOCKS=0 TMPDIR=$O/tmp
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
say() { echo "== $* ($(date -u +%FT%TZ))"; }
gate() { local name=$1; shift; python3 $REC/pass_checks.py "$@" > $O/gate_$name.json; local c=$?
  printf "%s\t%s\n" "$name" "$c" >> $O/verdict.tsv; say "gate $name: code $c $(head -c 240 $O/gate_$name.json)"; return $c; }
git -C $NUMREPO archive --format=tar $REV -- projects/chirality-piping ':(exclude)projects/chirality-piping/execution' | tar -x -C $WK
git -C $NUMREPO ls-tree -r $REV -- projects/chirality-piping | grep -v "projects/chirality-piping/execution/" | awk '$2=="blob"{print $3"\t"$4}' > $O/tree_blobs.tsv
gate tree tree $O/tree_blobs.tsv $WK
W=$WK/projects/chirality-piping; P=$W/core/product_physics; RM=$P/src/retained_memory.rs
git -C $NUMREPO show $REGISTERED_REV:projects/chirality-piping/core/product_physics/src/retained_memory.rs > $O/registered_retained_memory.rs
gate entry entry $RM $O/registered_retained_memory.rs
python3 $REC/statics_list.py $W $REC/chain/crate_dirs.txt $REC/reference/statics_a2.json > $O/statics.json
gate statics statics $O/statics.json
mkdir -p $RR; cp $REC/chain/* $RR/
python3 $REC/g7_linemap.py $NUMREPO $PASS_A_REV $REV $O/rules $REC/chain/callgraph_rules.g4.json $REC/chain/loop_bounds.g4.json $REC/chain/text_args.g4.json $REC/chain/sens.py $REC/premise_pins.json > $O/linemap.out.json
lm=$?; printf "linemap\t%s\n" $([ $lm -eq 0 ] && echo 0 || echo 4) >> $O/verdict.tsv; say "linemap: exit $lm"
cp $O/rules/callgraph_rules.g4.json $O/rules/loop_bounds.g4.json $O/rules/text_args.g4.json $O/rules/sens.py $RR/
gate premise premise $O/rules/premise_pins.json $W
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
( cd $W && bash $RR/run_text_part2.sh pb $O > $O/logs/text.log 2>&1 ); tc=$?
printf "text_run\t%s\n" $([ $tc -eq 0 ] && echo 0 || echo 6) >> $O/verdict.tsv; say "TEXT chain exit $tc"
D=$(python3 -c "import json;print(json.load(open('$O/sens_pb.summary.json'))['D'])" 2>/dev/null || echo 0); echo $D > $O/D.txt; say "D from the run: $D"
python3 $REC/delta_inventory2.py $NUMREPO $PASS_A_REV $REV $W $REC/chain/crate_dirs.txt $O/edges_pb.json $O/sens_pb/loop_bounds.g4.json $REC/delta_reviewed.json $O/delta_inventory.json > $O/delta.out.txt
dc=$?; printf "delta\t%s\n" $dc >> $O/verdict.tsv; say "delta: exit $dc $(head -1 $O/delta.out.txt)"
gate text text $O pb $REC_A/pass_a/text_g7 $REC/text_row_diff.py
gate forms forms $RM $REC_A/pass_a/text_g7/profile_tree.json $RR/g5_profile.py $O
python3 $REC/price_delta.py $O/sens_pb/profile_tree.json $LAWREC > $O/price_delta.out.json
( cd $W && env G4_CAPS='{"l": 128}' TB_LEXICON=$O/lexicon_pb.json TB_COMPOSITE=$O/sens_pb/composite_text.caps.json TB_D=$D TB_NONCAND_OUT=$O/noncand.json python3 $O/sens_pb/text_budget.py $W $O/sens_pb/template_inventory_head.out.json $O/edges_pb.json $O/sens_pb/loop_bounds.g4.json $O/sens_pb/text_args.g4.json run_linear_static_preview_value_with_retained_direct caps > $O/whole_nc.out.json ); nc=$?
printf "noncand_run\t%s\n" $([ $nc -eq 0 ] && echo 0 || echo 6) >> $O/verdict.tsv
python3 $REC/noncand_compare.py $REV87 $O/noncand.json > $O/noncand_compare.out.json
gate noncand noncand $O/noncand_compare.out.json
( cd $W && TB_D_RUN=$D python3 $REC/audit_controls_g7.py $O/sens_pb $W $O/edges_pb.json $O/lexicon_pb.json $O/ctl > $O/controls.out.txt ); cr=$?
printf "controls_run\t%s\n" $([ $cr -eq 0 ] && echo 0 || echo 6) >> $O/verdict.tsv
gate controls controls $O/ctl/audit_controls_g7.out.json 12
echo "NOBUILD GATES $(awk -F'\t' '{printf "%s:%s ", $1, $2}' $O/verdict.tsv)" | tee $O/GATES.txt
