#!/bin/bash
# PF M3-T run driver (kd5_timoshenko/run.sh adapted): runs the runner on every chunk of a run directory (typed
# and captured entries, both solver modes) and judges each chunk with pft_compare.py against the requested
# reference variants.
#   run.sh <runner binary> <set dir> <run dir> [variant...]
#     <set dir>  holds set.jsonl (pft_generate.py full)
#     <run dir>  holds chunks/c.NNN.in.jsonl (pft_generate.py split); out/ and eval/ are written there
#     variant    own (default) or twin (the twin theory's reference: the sensitivity control)
# Environment: PFM3T_PY (python, default python3); PFM3T_WRAP (a command prefix for each runner and compare
# job, for example a host's resource wrapper; default none); PFM3T_WORKERS (compare workers, default 6).
# Finished outputs are kept, so a rerun resumes.
set -u
[ $# -ge 3 ] || { sed -n '2,11p' "$0" >&2; exit 2; }
HERE=$(cd "$(dirname "$0")" && pwd -P)
BIN=$1
SET=$2
D=$3
shift 3
VARIANTS=${*:-own}
PY=${PFM3T_PY:-python3}
WRAP=${PFM3T_WRAP:-}
mkdir -p "$D/out" "$D/eval"
status=0
for f in "$D"/chunks/c.[0-9][0-9][0-9].in.jsonl; do
  n=$(basename "$f" .in.jsonl)
  if [ ! -s "$D/out/$n.out.jsonl" ]; then
    KD5_IN=$f KD5_OUT=$D/out/$n.out.tmp KD5_ENTRIES=typed,captured $WRAP "$BIN" || { echo "run failed $n" >&2; status=1; continue; }
    mv "$D/out/$n.out.tmp" "$D/out/$n.out.jsonl"
  fi
  for v in $VARIANTS; do
    case $v in
      own | twin) ;;
      *) echo "unknown variant $v" >&2; exit 2 ;;
    esac
    [ -s "$D/eval/$n.$v.jsonl" ] && continue
    $WRAP "$PY" -I -B "$HERE/pft_compare.py" "$SET/set.jsonl" "$D/out/$n.out.jsonl" "$D/eval/$n.$v.tmp" --ref "$v" \
      --workers "${PFM3T_WORKERS:-6}" || { echo "compare failed $n $v" >&2; status=1; continue; }
    mv "$D/eval/$n.$v.tmp" "$D/eval/$n.$v.jsonl"
  done
  echo "$(date -u +%T) done $n"
done
exit $status
