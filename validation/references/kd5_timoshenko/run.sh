#!/bin/bash
# M3-T run driver: runs the runner on every chunk of a run directory (typed and captured entries, both
# solver modes) and judges each chunk with kd5t_compare.py against the requested reference variants.
#   run.sh <runner binary> <set dir> <run dir> [variant...]
#     <set dir>  holds set.jsonl and controls.jsonl (kd5t_generate.py full)
#     <run dir>  holds chunks/c.NNN.in.jsonl (kd5t_generate.py split); out/ and eval/ are written there
#     variant    own (default), twin (the twin theory's reference) or control (E perturbed by 2^-20)
# Environment: KD5T_PY (python, default python3); KD5T_WRAP (a command prefix for each runner and
# compare job, for example a host's resource wrapper; default none); PRODUCER (u6 or pre-u6, default
# u6); KD5T_WORKERS (compare workers, default 6). Finished outputs are kept, so a rerun resumes.
set -u
[ $# -ge 3 ] || { sed -n '2,10p' "$0" >&2; exit 2; }
HERE=$(cd "$(dirname "$0")" && pwd -P)
BIN=$1
SET=$2
D=$3
shift 3
VARIANTS=${*:-own}
PY=${KD5T_PY:-python3}
WRAP=${KD5T_WRAP:-}
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
      own | twin) R=$v ;;
      control) R=control:$SET/controls.jsonl:pertE ;;
      *) echo "unknown variant $v" >&2; exit 2 ;;
    esac
    [ -s "$D/eval/$n.$v.jsonl" ] && continue
    $WRAP "$PY" -I -B "$HERE/kd5t_compare.py" "$SET/set.jsonl" "$D/out/$n.out.jsonl" "$D/eval/$n.$v.tmp" --ref "$R" \
      --producer "${PRODUCER:-u6}" --workers "${KD5T_WORKERS:-6}" || { echo "compare failed $n $v" >&2; status=1; continue; }
    mv "$D/eval/$n.$v.tmp" "$D/eval/$n.$v.jsonl"
  done
  echo "$(date -u +%T) done $n"
done
exit $status
