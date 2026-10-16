#!/bin/bash
# Premise-census corpora: re-authors each surviving corpus chunk as 0.5.0 exact documents with the given
# shear model (kd5t_reauthor.py) and runs it slim (standings only) with the runner.
#   census.sh <eb|ts> <runner binary> <corpus dir> <out dir> [set...]
#     <corpus dir>  holds <set>/chunks/c.NNN, the surviving RV30, realistic, RV29 and W1 input chunks
#     set           default: real rv29 w1 rv30A rv30B rv30C rv30D
# Environment: KD5T_PY and KD5T_WRAP as for run.sh; KD5T_KEEP_INPUTS (set: keep the re-authored
# inputs, which kd5t_census.py hager reads). Finished outputs are kept, so a rerun resumes.
set -u
[ $# -ge 4 ] || { sed -n '2,8p' "$0" >&2; exit 2; }
HERE=$(cd "$(dirname "$0")" && pwd -P)
TH=$1
BIN=$2
C=$3
OUT=$4
shift 4
SETS=${*:-real rv29 w1 rv30A rv30B rv30C rv30D}
PY=${KD5T_PY:-python3}
WRAP=${KD5T_WRAP:-}
mkdir -p "$OUT"
status=0
for set in $SETS; do
  for f in "$C/$set"/chunks/c.[0-9][0-9][0-9]; do
    n=$set.$(basename "$f")
    [ -s "$OUT/$n.out.jsonl" ] && continue
    $WRAP "$PY" -I -B "$HERE/kd5t_reauthor.py" "$TH" "$OUT/$n.in.jsonl" "$f" >/dev/null || { echo "reauthor failed $n" >&2; status=1; continue; }
    KD5_IN=$OUT/$n.in.jsonl KD5_OUT=$OUT/$n.out.tmp KD5_ENTRIES=typed,captured KD5_SLIM=1 $WRAP "$BIN" || { echo "run failed $n" >&2; status=1; continue; }
    mv "$OUT/$n.out.tmp" "$OUT/$n.out.jsonl"
    [ -n "${KD5T_KEEP_INPUTS:-}" ] || rm -f "$OUT/$n.in.jsonl"
    echo "$(date -u +%T) done $n"
  done
done
exit $status
