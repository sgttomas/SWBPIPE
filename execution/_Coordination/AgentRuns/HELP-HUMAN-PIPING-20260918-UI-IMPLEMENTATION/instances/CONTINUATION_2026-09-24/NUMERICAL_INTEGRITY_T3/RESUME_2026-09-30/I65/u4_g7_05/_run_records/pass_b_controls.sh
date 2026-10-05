#!/bin/bash
# I65 U4 G7 Pass B controls (ROOT item 7): each new stop fired on a mutated copy, through the same
# gate the pass uses, plus the self-test runs. Prints one line per control: name, expected code,
# got code, PASS/FAIL; exits 1 if any control fails.
# Usage: I65_T=<WT> pass_b_controls.sh <out dir> <self-test run dir (pass_<tag>, Pass A basis + T17_V4 line)>
set -u
T=${I65_T:?}; OUT=$1; ST=$2; mkdir -p $OUT
R0=$T/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30
REC=$R0/I65/u4_g7_03/_run_records; RA=$R0/I65/u4_g7_01/_run_records
PA_W=$ST/work/projects/chirality-piping; RM=$PA_W/core/product_physics/src/retained_memory.rs
PASS_A_REV=ba1faa1c858ce3630a22767677310b1902a14b83
export GIT_OPTIONAL_LOCKS=0
pgrep -f memguard.sh >/dev/null || { echo "MEMGUARD NOT RUNNING"; exit 9; }
FAILS=0
ck() { python3 $REC/pass_checks.py "$@" > $OUT/last.json; echo $?; }
row() { local ok=FAIL; [ "$2" = "$3" ] && ok=PASS || FAILS=$((FAILS+1)); printf "%-52s expect %s got %s %s\n" "$1" "$2" "$3" "$ok"; }
# tree (2)
mkdir -p $OUT/tree/projects; printf 'x\n' > $OUT/tree/projects/f.txt
printf '%s\t%s\n' "$(printf 'x\n' | git hash-object --stdin)" "projects/f.txt" > $OUT/tree/ok.tsv
row "tree: identical copy" 0 $(ck tree $OUT/tree/ok.tsv $OUT/tree)
printf 'y\n' > $OUT/tree/projects/f.txt; row "tree: one file mutated" 2 $(ck tree $OUT/tree/ok.tsv $OUT/tree)
printf 'x\n' > $OUT/tree/projects/f.txt; printf 'z\n' > $OUT/tree/projects/extra.txt; row "tree: one file added" 2 $(ck tree $OUT/tree/ok.tsv $OUT/tree)
# entry (3)
git -C $T/numerics show 0c7827b6ad:projects/chirality-piping/core/product_physics/src/retained_memory.rs > $OUT/reg.rs
row "entry: self-test basis vs 0c7827b6ad" 0 $(ck entry $RM $OUT/reg.rs)
sed 's/threshold_bytes: 4_026_531_840/threshold_bytes: 8_053_063_680/' $RM > $OUT/thr.rs; row "entry: threshold doubled" 3 $(ck entry $OUT/thr.rs $OUT/reg.rs)
sed 's/rustc.release=1.97.1;/rustc.release=1.97.2;/' $RM > $OUT/id.rs; row "entry: identity edited" 3 $(ck entry $OUT/id.rs $OUT/reg.rs)
sed 's/TypeLayout { size: 56, align: 8 }/TypeLayout { size: 64, align: 8 }/' $RM > $OUT/lay.rs; row "entry: a reader layout edited" 3 $(ck entry $OUT/lay.rs $OUT/reg.rs)
# law (3)
L=$ST/logs/law.log
row "law: self-test log" 0 $(ck law $L $RM)
sed 's/0 failed; 9 ignored/1 failed; 9 ignored/' $L > $OUT/law_failed.log; row "law: one failed test" 3 $(ck law $OUT/law_failed.log $RM)
sed 's/I65_G5_IDENTITY \(.*\)opt_level=0;/I65_G5_IDENTITY \1opt_level=1;/' $L > $OUT/law_stale.log; row "law: compiled identity Stale (opt_level=1)" 3 $(ck law $OUT/law_stale.log $RM)
sed 's/I65_G5_READER_LAYOUT Validation size=56/I65_G5_READER_LAYOUT Validation size=64/' $L > $OUT/law_lay.log; row "law: a compiled reader layout differs" 3 $(ck law $OUT/law_lay.log $RM)
grep -v "the_registered_profile_is_the_only_permit_source" $L > $OUT/law_missing.log; row "law: a registered test absent" 3 $(ck law $OUT/law_missing.log $RM)
: > $OUT/law_empty.log; row "law: empty log (build failure)" 3 $(ck law $OUT/law_empty.log $RM)
# premise (4)
mkdir -p $OUT/prem/core/reporting/result_export/src; cp $PA_W/core/reporting/result_export/src/retained_precision.rs $OUT/prem/core/reporting/result_export/src/
row "premise: as reviewed" 0 $(ck premise $REC/premise_pins.json $OUT/prem)
sed -i '' 's#json!("openpipestress.result_semantics/0.3.0/preview-physics-1");#json!(crate::retained_precision::CONTRACT_ID);#' $OUT/prem/core/reporting/result_export/src/retained_precision.rs
row "premise: the projection keeps the successor id" 4 $(ck premise $REC/premise_pins.json $OUT/prem)
cp $PA_W/core/reporting/result_export/src/retained_precision.rs $OUT/prem/core/reporting/result_export/src/
sed -i '' 's#crate::semantic_contract::for_source(&projected).map_err(base_error)?;#crate::semantic_contract::for_source(source).map_err(base_error)?;#' $OUT/prem/core/reporting/result_export/src/retained_precision.rs
row "premise: validate passes the unprojected source" 4 $(ck premise $REC/premise_pins.json $OUT/prem)
# linemap (4 in the pass): rule lines inside edited hunks (Pass A basis -> 0c7827b6ad, which lacks U6)
python3 $REC/g7_linemap.py $T/numerics $PASS_A_REV 0c7827b6ad $OUT/lm $REC/chain/loop_bounds.g4.json $REC/premise_pins.json > $OUT/lm.out.json; c=$?
row "linemap: rule lines in edited hunks (pass maps 1->4)" 1 $c
python3 $REC/g7_linemap.py $T/numerics $PASS_A_REV $PASS_A_REV $OUT/lm0 $REC/chain/loop_bounds.g4.json $REC/premise_pins.json > $OUT/lm0.out.json; c=$?
row "linemap: unchanged revision" 0 $c
# delta (5): F5 (0c7827b6ad -> Pass A basis) is live and unreviewed; the final basis's hunks with an empty table
echo '{"entries": []}' > $OUT/empty_reviewed.json
python3 $REC/delta_inventory2.py $T/numerics 0c7827b6ad $PASS_A_REV $PA_W $REC/chain/crate_dirs.txt $ST/edges_pb.json $ST/sens_pb/loop_bounds.g4.json $OUT/empty_reviewed.json $OUT/delta_u6.json > $OUT/delta_u6.txt; c=$?
row "delta: U6's F5 live hunk, no reviewed entry" 5 $c
grep -q '"live"' $OUT/delta_u6.json && grep -q "g5_ordinary" $OUT/delta_u6.json && echo "   (F5 classified live in g5_ordinary)"
python3 $REC/delta_inventory2.py $T/numerics $PASS_A_REV $PASS_A_REV $PA_W $REC/chain/crate_dirs.txt $ST/edges_pb.json $ST/sens_pb/loop_bounds.g4.json $OUT/empty_reviewed.json $OUT/delta_none.json > /dev/null; c=$?
row "delta: no change" 0 $c
# statics (6)
python3 - $REC/reference/statics_a2.json $OUT/statics_added.json <<'PY'
import json, sys
ref = json.load(open(sys.argv[1])); rows = ref["rows"][1:]
json.dump({"rows": ref["rows"], "added": [[ref["rows"][0]["file"], ref["rows"][0]["text"]]], "removed": []}, open(sys.argv[2], "w"))
PY
row "statics: one added" 6 $(ck statics $OUT/statics_added.json)
python3 $REC/statics_list.py $PA_W $REC/chain/crate_dirs.txt $REC/reference/statics_a2.json > $OUT/statics_same.json; row "statics: self-test basis" 0 $(ck statics $OUT/statics_same.json)
# forms (6): Pass A's basis without the adopted T17_V4 line
git -C $T/numerics show $PASS_A_REV:projects/chirality-piping/core/product_physics/src/retained_memory.rs > $OUT/rm_ba1f.rs
row "forms: Pass A basis without the T17_V4 line" 6 $(ck forms $OUT/rm_ba1f.rs $RA/pass_a/text_g7/profile_tree.json $REC/chain/g5_profile.py $OUT)
row "forms: with the line (self-test basis)" 0 $(ck forms $RM $RA/pass_a/text_g7/profile_tree.json $REC/chain/g5_profile.py $OUT)
# text (6): a run directory with one row's bytes changed, and one with the run incomplete
mkdir -p $OUT/tx/sens_m; cp $ST/sens_pb/*.json $ST/sens_pb/*.log $OUT/tx/sens_m/ 2>/dev/null; cp $ST/sens_pb.summary.json $OUT/tx/sens_m.summary.json
row "text: the self-test run" 0 $(ck text $OUT/tx m $RA/pass_a/text_g7 $REC/text_row_diff.py)
python3 - $OUT/tx/sens_m/text_budget_W.caps.out.json <<'PY'
import json, sys
p = sys.argv[1]; t = json.load(open(p)); r = next(x for x in t["rows"] if x["mult"] > 0); r["bytes"] += 1; r["req"] += 2 * r["mult"]; json.dump(t, open(p, "w"))
PY
row "text: one W row's bytes raised" 6 $(ck text $OUT/tx m $RA/pass_a/text_g7 $REC/text_row_diff.py)
cp $ST/sens_pb/text_budget_W.caps.out.json $OUT/tx/sens_m/
python3 - $OUT/tx/sens_m/text_budget.caps.out.json <<'PY'
import json, sys
p = sys.argv[1]; t = json.load(open(p)); t["complete"] = False; t["unclassified_args"] = [[["scc", "x y"], 1]]; json.dump(t, open(p, "w"))
PY
row "text: an incomplete run (scc)" 6 $(ck text $OUT/tx m $RA/pass_a/text_g7 $REC/text_row_diff.py)
# noncand (6)
python3 - $ST/noncand_compare.out.json $OUT/noncand_new.json <<'PY'
import json, sys
d = json.load(open(sys.argv[1])); d["new_noncandidates"] = [{"site": "x", "expr": "end"}]; json.dump(d, open(sys.argv[2], "w"))
PY
row "noncand: the self-test sweep" 0 $(ck noncand $ST/noncand_compare.out.json)
row "noncand: one new non-candidate" 6 $(ck noncand $OUT/noncand_new.json)
# controls (6)
python3 - $ST/ctl/audit_controls_g7.out.json $OUT/ctl_fail.json <<'PY'
import json, sys
d = json.load(open(sys.argv[1])); d[3]["pass"] = False; json.dump(d, open(sys.argv[2], "w"))
PY
row "controls: the self-test's 12" 0 $(ck controls $ST/ctl/audit_controls_g7.out.json 12)
row "controls: one failing" 6 $(ck controls $OUT/ctl_fail.json 12)
# outcomes (6)
awk 'NR==5{sub(/ ok$/," FAILED")}1' $REC/reference/a_pp.outcomes > $OUT/pp_flip.outcomes
row "outcomes: one test flipped" 6 $(ck outcomes $REC/reference/a_pp.outcomes $OUT/pp_flip.outcomes)
row "outcomes: identical" 0 $(ck outcomes $REC/reference/a_pp.outcomes $REC/reference/a_pp.outcomes)
# D read from the run (S-1(d)): the sweep and controls take the run's D; a different D changes the run
echo "D the self-test read from its run (S-1(d); used by the sweep and the controls): $(cat $ST/D.txt)"
echo "CONTROLS: $FAILS failing"
exit $([ $FAILS -eq 0 ] && echo 0 || echo 1)
