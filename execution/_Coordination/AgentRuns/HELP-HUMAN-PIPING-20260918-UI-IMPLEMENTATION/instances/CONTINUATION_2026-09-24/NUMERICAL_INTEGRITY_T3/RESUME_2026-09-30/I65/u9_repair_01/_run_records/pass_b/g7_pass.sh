#!/bin/bash
# I65 U4 G7 Pass B (repaired after RV89 G7 S-1 and RV87 G7 SF-1/N-1; hardened after RV89 ADDENDUM_01 N-4 and
# N-5 in u4_g7_03: a reused tag starts from an empty pass_<tag>/, and the TEXT chain, the section 11 run
# and the controls run are gates; the qualification's own test files need a reviewed entry): repeat items 1-5 of the G7 brief
# mechanically on a new basis, against Pass A (u4_g7_01), and end with ONE verdict and its exit code.
# Usage: I65_T=<WT> g7_pass.sh <basis dir holding projects/chirality-piping> <basis rev> <tag> [overlay.diff]
#   overlay.diff (self-test only): a patch applied to the copy after the tree check (e.g. Pass A's
#   basis plus the adopted T17_V4 line); the verdict names it.
# Reads its tools and rules from REC (this folder) and Pass A's references from REC_A. Writes only
# under WT/scratch/i65_u4_g7_01/pass_<tag>/ and WT/targets/i65_g7/work*-pass_<tag>. Read-only for every
# source file; Git reads only (GIT_OPTIONAL_LOCKS=0); one cargo job at a time; memguard checked.
# Exit: 0 PASS; 2 the copy is not the revision's tree; 3 the entry differs from the registered one,
#   or the build is not Registered, or a law test fails; 4 a rule line (including the dead-branch
#   premise pins) was edited; 5 a production hunk needs a reviewed entry; 6 deltas to read (statics,
#   TEXT incomplete/scc/self-recursion/differences, FORMS, sweep, controls, outcomes); 9 memguard.
set -u
T=${I65_T:?set I65_T to the worktree root}; BASIS=$1; REV=$2; TAG=$3; OVERLAY=${4:-}
[[ "$TAG" =~ ^[A-Za-z0-9_]+$ ]] || { echo "tag must be [A-Za-z0-9_]+"; exit 1; }
R0=$T/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
REC=$R0/I65/u9_repair_01/_run_records/pass_b; REC_A=$R0/I65/u4_g7_01/_run_records
REV87=$R0/REVIEW_RV87/u4_g6_02/_run_records/rv87_g6r_noncandidates.out.json
NUMREPO=$T/numerics; PASS_A_REV=ba1faa1c858ce3630a22767677310b1902a14b83; REGISTERED_REV=0c7827b6ad
O=$T/scratch/i65_u4_g7_01/pass_$TAG; WK=$O/work; RR=$O/rr
G7L=$T/scratch/i65_u4_g7_01/logs
# N-4: a reused tag never reads a previous run's outputs: pass_<tag>/ is emptied, and so are this
# tag's cargo logs and outcome lists
rm -rf $O; rm -f $G7L/pass_${TAG}_pp.log $G7L/pass_${TAG}_pp.outcomes $G7L/pass_${TAG}_runner.log $G7L/pass_${TAG}_runner.outcomes $G7L/pass_${TAG}_challenge.log $G7L/pass_${TAG}_witness_*.log
mkdir -p $O/logs $O/tmp $G7L; : > $O/verdict.tsv
export GIT_OPTIONAL_LOCKS=0 TMPDIR=$O/tmp
guard() { pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }; }
say() { echo "== $* ($(date -u +%FT%TZ))"; }
gate() { # gate <name> <check args...>: run one check, record its code
  local name=$1; shift; python3 $REC/pass_checks.py "$@" > $O/gate_$name.json; local c=$?
  printf "%s\t%s\n" "$name" "$c" >> $O/verdict.tsv; say "gate $name: code $c $(head -c 300 $O/gate_$name.json)"; return $c; }
finish() { # the one verdict: the first stop code, else 6 on any delta, else 0
  local code=$(awk -F'\t' '$2==2||$2==3||$2==4||$2==5{print $2; exit}' $O/verdict.tsv)
  [ -z "$code" ] && code=$(awk -F'\t' '$2!=0{print 6; exit}' $O/verdict.tsv); [ -z "$code" ] && code=0
  local word=PASS; [ $code -ne 0 ] && word="STOP"; [ $code -eq 6 ] && word="DELTAS TO READ"
  echo "VERDICT $word exit=$code basis=$REV tag=$TAG overlay=${OVERLAY:-none} gates=$(awk -F'\t' '{printf "%s:%s ", $1, $2}' $O/verdict.tsv)" | tee $O/VERDICT.txt
  exit $code; }
guard
# ---- 0. a fresh copy, checked blob for blob against the revision; an optional self-test overlay
rm -rf $WK; mkdir -p $WK; cp -R $BASIS/projects $WK/
git -C $NUMREPO ls-tree -r $REV -- projects/chirality-piping | grep -v "projects/chirality-piping/execution/" | awk '$2=="blob"{print $3"\t"$4}' > $O/tree_blobs.tsv
gate tree tree $O/tree_blobs.tsv $WK || finish
if [ -n "$OVERLAY" ]; then ( cd $WK && patch -p1 < $OVERLAY ) > $O/overlay.log 2>&1 || { echo "overlay failed"; exit 1; }; say "overlay applied: $OVERLAY"; fi
W=$WK/projects/chirality-piping; P=$W/core/product_physics; RM=$P/src/retained_memory.rs
# ---- item 2 / S-1(c): the entry against the registered commit's, byte for byte; the law tests gate
git -C $NUMREPO show $REGISTERED_REV:projects/chirality-piping/core/product_physics/src/retained_memory.rs > $O/registered_retained_memory.rs
gate entry entry $RM $O/registered_retained_memory.rs || finish
guard
( cd $P && env CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=2 perl -e 'alarm shift; exec @ARGV' 3600 cargo test --locked --offline --lib --target-dir $T/targets/i65_g7/work-pass_$TAG retained_memory -- --nocapture --test-threads=1 > $O/logs/law.log 2>&1 )
gate law law $O/logs/law.log $RM || finish
python3 $REC/statics_list.py $W $REC/chain/crate_dirs.txt $REC/reference/statics_a2.json > $O/statics.json
gate statics statics $O/statics.json
# ---- the rules, carried by the line map (the premise pins are rule lines); the premise gate
mkdir -p $RR; cp $REC/chain/* $RR/
python3 $REC/g7_linemap.py $NUMREPO $PASS_A_REV $REV $O/rules $REC/chain/callgraph_rules.g4.json $REC/chain/loop_bounds.g4.json $REC/chain/text_args.g4.json $REC/chain/sens.py $REC/premise_pins.json > $O/linemap.out.json
lm=$?; printf "linemap\t%s\n" $([ $lm -eq 0 ] && echo 0 || echo 4) >> $O/verdict.tsv; say "linemap: exit $lm"; [ $lm -eq 0 ] || finish
cp $O/rules/callgraph_rules.g4.json $O/rules/loop_bounds.g4.json $O/rules/text_args.g4.json $O/rules/sens.py $RR/
gate premise premise $O/rules/premise_pins.json $W || finish
# ---- TEXT on the basis (inventory regenerated; D read from the run)
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
# ---- item 1 / S-1(a): the production delta, every hunk classified; unreviewed live hunks stop
python3 $REC/delta_inventory2.py $NUMREPO $PASS_A_REV $REV $W $REC/chain/crate_dirs.txt $O/edges_pb.json $O/sens_pb/loop_bounds.g4.json $REC/delta_reviewed.json $O/delta_inventory.json > $O/delta.out.txt
dc=$?; [ $dc -ne 0 ] && [ $dc -ne 5 ] && [ $dc -ne 6 ] && dc=6   # a tool failure (e.g. no edges) is a delta to read
printf "delta\t%s\n" $dc >> $O/verdict.tsv; say "delta: exit $dc $(head -1 $O/delta.out.txt)"
[ $dc -eq 6 ] && [ ! -s $O/delta_inventory.json ] && finish   # no inventory: nothing below can be read
[ $dc -eq 5 ] && finish                                           # 6 (qualification tests to read) continues
# ---- TEXT gates, FORMS, the section 11 sweep, the controls
gate text text $O pb $REC_A/pass_a/text_g7 $REC/text_row_diff.py
gate forms forms $RM $REC_A/pass_a/text_g7/profile_tree.json $RR/g5_profile.py $O
python3 $REC/price_delta.py $O/sens_pb/profile_tree.json $O/logs/law.log > $O/price_delta.out.json
( cd $W && env G4_CAPS='{"l": 128}' TB_LEXICON=$O/lexicon_pb.json TB_COMPOSITE=$O/sens_pb/composite_text.caps.json TB_D=$D TB_NONCAND_OUT=$O/noncand.json python3 $O/sens_pb/text_budget.py $W $O/sens_pb/template_inventory_head.out.json $O/edges_pb.json $O/sens_pb/loop_bounds.g4.json $O/sens_pb/text_args.g4.json run_linear_static_preview_value_with_retained_direct caps > $O/whole_nc.out.json ); nc=$?
printf "noncand_run\t%s\n" $([ $nc -eq 0 ] && echo 0 || echo 6) >> $O/verdict.tsv; say "section 11 run exit $nc"
python3 $REC/noncand_compare.py $REV87 $O/noncand.json > $O/noncand_compare.out.json
gate noncand noncand $O/noncand_compare.out.json
( cd $W && TB_D_RUN=$D python3 $REC/audit_controls_g7.py $O/sens_pb $W $O/edges_pb.json $O/lexicon_pb.json $O/ctl > $O/controls.out.txt ); cr=$?
printf "controls_run\t%s\n" $([ $cr -eq 0 ] && echo 0 || echo 6) >> $O/verdict.tsv; say "controls run exit $cr"
gate controls controls $O/ctl/audit_controls_g7.out.json 12
# ---- item 5: witnesses, the challenge, PP (all targets) and runner/headless; outcomes against Pass A
guard
I65_T=$T bash $REC/run_cargo_a.sh pass_$TAG $WK > $O/logs/cargo.out 2>&1
grep -E '^== |passed|I65_G5_CHALLENGE |I65_G5_WITNESS' $O/logs/cargo.out > $O/cargo_summary.txt
gate pp_outcomes outcomes $REC/reference/a_pp.outcomes $G7L/pass_${TAG}_pp.outcomes
gate runner_outcomes outcomes $REC/reference/a_runner.outcomes $G7L/pass_${TAG}_runner.outcomes
w=$(grep -c "^== witness_.* exit=0" $O/logs/cargo.out); printf "witnesses\t%s\n" $([ "$w" = 9 ] && echo 0 || echo 6) >> $O/verdict.tsv; say "witnesses: $w/9 exit 0"
c=$(grep -c "^test result: ok. 1 passed" $G7L/pass_${TAG}_challenge.log); printf "challenge\t%s\n" $([ "$c" = 1 ] && echo 0 || echo 6) >> $O/verdict.tsv; say "challenge: $c"
finish
