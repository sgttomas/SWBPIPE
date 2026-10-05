#!/bin/bash
# I66 U6a: copy run evidence into the records folder with placeholder paths.
set -u
WT=WT
REPO=REPO_ROOT
S=$WT/scratch/i66_u6a_slice_01
C=$WT/f2a-carriers
R=$WT/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I66/u6a_slice_01/_run_records
mkdir -p "${R:?}"
clean() { sed -e "s#$WT#WT#g" -e "s#$REPO#REPO_ROOT#g" -e "s#TMP ]*#TMP#g" "$1" > "$R/$2"; }
for f in run_rs.sh run_py.sh mutants.py zz_i66_sweep.rs lane_py_backout.py i66BackOut.test.ts lane_schema.py collect_records.sh; do clean "$S/$f" "$f"; done
for f in sweep_base.tsv sweep_base2.tsv sweep_base3.tsv sweep_cand.tsv sweep_existing.diff; do clean "$S/$f" "$f"; done
for f in base_result_export.outcomes cand_final.outcomes lane_py_backout.log lane_ts_backout.results lane_schema.log mutants_run.log mutants_final.json mutants_run_part1.log mutants_run_part2.log base_py.summary cand_py.summary runner_cand.outcomes pp_pins.outcomes sweep_compare.txt; do [ -f "$S/logs/$f" ] && clean "$S/logs/$f" "$f"; done
( cd "$C" && GIT_OPTIONAL_LOCKS=0 git diff 7e4f5a51dd -- projects/chirality-piping > "$R/candidate_tracked.diff" 2>/dev/null )
clean "$R/candidate_tracked.diff" candidate_tracked.diff.tmp && mv "$R/candidate_tracked.diff.tmp" "$R/candidate_tracked.diff"
( cd "$C" && for f in $(cat "$S/changed_files.txt") projects/chirality-piping/fixtures/results/retained_precision_carrier_cases.json; do shasum -a 256 "$f"; done ) | sed 's#projects/chirality-piping/#P/#' > "$R/changed_files_sha256.txt"
grep -rl "/Us""ers/" "$R" && echo "MACHINE PATHS REMAIN" || echo "no machine paths"
