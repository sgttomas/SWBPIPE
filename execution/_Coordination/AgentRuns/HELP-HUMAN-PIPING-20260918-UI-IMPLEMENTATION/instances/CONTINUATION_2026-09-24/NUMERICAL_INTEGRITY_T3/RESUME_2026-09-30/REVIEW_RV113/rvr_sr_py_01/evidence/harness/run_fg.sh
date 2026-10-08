#!/bin/bash
# RV113: the (f)/(g) probes on PY (I1, head), RS (b5cb7faaeb) and TS (7e47e51b5d); every job under the T3 lock.
WT=WT
S=$WT/scratch/rv113_rvr_01
PR=$S/py/fg/probes_fg.json
for v in i1 head; do
  P=$WT/rv113/py-$v/projects/chirality-piping
  $S/tools/run_py_job.sh fg_py_$v $P $S/tools/rv113_py_harness.py $P probes $PR $S/py/fg/py_${v}_fg.jsonl; echo "py $v rc=$?"
done
RE=$WT/rv113/fg-rs/projects/chirality-piping/core/reporting/result_export
RV113_PROBES=$PR RV113_PROBES_OUT=$S/py/fg/rs_fg.jsonl $S/tools/run_job.sh fg_rs $RE $WT/targets/rv113-fgrs test --locked --offline --test rv113_census -- rv113_probes --exact; echo "rs rc=$?"
DT=$WT/rv113/fg-ts/projects/chirality-piping/apps/desktop
RV113_PROBES=$PR RV113_PROBES_OUT=$S/py/fg/ts_fg.jsonl $S/tools/run_ts_job.sh fg_ts $DT vitest run src/features/results/rv113Census.test.ts; echo "ts rc=$?"
