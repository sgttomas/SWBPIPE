#!/bin/bash
# I65 U4 G7 (u4_g7_03), RV89 ADDENDUM_01 N-4 controls on mutated copies of g7_pass.sh (never the
# recorded script): one step forced to fail, the item 5 cargo stage cut (so a control ends at
# `finish`), and pass_<tag>/ pre-filled with a previous run's outputs, as a reused tag would leave them.
# Usage: I65_T=<WT> pass_b_controls_n4.sh <basis dir> <basis rev> <previous run dir (pass_<tag>)> <out dir>
set -u
T=${I65_T:?}; BASIS=$1; REV=$2; PREV=$3; OUT=$4; mkdir -p $OUT
R0=$T/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
NEW=$R0/I65/u4_g7_03/_run_records/g7_pass.sh; OLD=$R0/I65/u4_g7_02/_run_records/g7_pass.sh
S=$T/scratch/i65_u4_g7_01
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
FAILS=0
mutate() { # mutate <script> <out> <which step fails: text|noncand|controls>
  python3 - "$1" "$2" "$3" <<'PY'
import sys, re
src, dst, which = sys.argv[1:4]; t = open(src).read()
pat = {"text": "bash $RR/run_text_part2.sh pb $O",
       "noncand": "TB_NONCAND_OUT=$O/noncand.json python3 $O/sens_pb/text_budget.py",
       "controls": "python3 $REC/audit_controls_g7.py"}[which]
assert t.count(pat) == 1, (which, t.count(pat))
t = t.replace(pat, "false && " + pat)                                   # the step fails, as a crash would
i = t.index("# ---- item 5"); t = t[:i] + "finish\n"                    # cut cargo: the control ends here
open(dst, "w").write(t)
PY
}
prefill() { # a reused tag: the previous run's TEXT, sweep and controls outputs, plus a sentinel
  local O=$S/pass_$1; mkdir -p $O
  cp -R $PREV/sens_pb $PREV/sens_pb.summary.json $PREV/edges_pb.json $PREV/lexicon_pb.json $PREV/noncand.json $PREV/noncand_compare.out.json $PREV/ctl $O/ 2>/dev/null
  echo stale > $O/SENTINEL; }
run() { # run <tag> <script> ; prints the verdict line
  ( I65_T=$T bash $2 $BASIS $REV $1 > $OUT/$1.out 2>&1 ); echo $? > $OUT/$1.code; }
gate_of() { awk -F'\t' -v g=$2 '$1==g{print $2}' $S/pass_$1/verdict.tsv | head -1; }
row() { local ok=FAIL; [ "$2" = "$3" ] && ok=PASS || FAILS=$((FAILS+1)); printf "%-62s expect %-3s got %-3s %s\n" "$1" "$2" "$3" "$ok"; }
for which in text noncand controls; do
  mutate $NEW $OUT/new_$which.sh $which
  tag=n4_new_$which; prefill $tag; run $tag $OUT/new_$which.sh
  row "new, $which step fails, reused tag: exit" 6 $(cat $OUT/$tag.code)
  row "new, $which step fails: gate ${which}_run" 6 "$(gate_of $tag ${which}_run)"
  row "new, reused tag: the stale sentinel is gone" gone $([ -e $S/pass_$tag/SENTINEL ] && echo present || echo gone)
done
# before N-4 (u4_g7_02's script, same mutation): the stale TEXT outputs are read and the text gate passes
mutate $OLD $OUT/old_text.sh text; tag=n4_old_text; prefill $tag; run $tag $OUT/old_text.sh
row "u4_g7_02 script, TEXT chain fails, reused tag: gate text" 0 "$(gate_of $tag text)"
row "u4_g7_02 script: the stale sentinel survives" present $([ -e $S/pass_$tag/SENTINEL ] && echo present || echo gone)
echo "N-4 CONTROLS: $FAILS failing"
exit $([ $FAILS -eq 0 ] && echo 0 || echo 1)
