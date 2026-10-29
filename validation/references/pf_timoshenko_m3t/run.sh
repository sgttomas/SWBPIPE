#!/bin/bash
# M3-T run driver of the PF route: the shared M3 judge's driver (../m3_judge/run.sh). It builds the shared runner from
# a clean checkout, runs it on every chunk (typed and captured entries, both solver modes) and judges the whole
# collection once per variant with ../m3_judge/m3_report.py --route pf.
#   run.sh <checkout> <set dir> <run dir> own [twin]
#     <set dir>  holds set.jsonl (pft_generate.py full)
#     <run dir>  holds chunks/c.NNN.in.jsonl (pft_generate.py split)
# Environment: as ../m3_judge/run.sh (M3J_PY, M3J_WRAP, M3J_WORKERS, M3J_TARGET, M3J_CARGO).
exec "$(cd "$(dirname "$0")" && pwd -P)/../m3_judge/run.sh" pf "$@"
