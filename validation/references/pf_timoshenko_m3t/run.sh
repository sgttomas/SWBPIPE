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
# Runner outputs are kept, so a rerun resumes; a cached evaluation is reused only when it is current
# (pft_compare.py --check-eval), and is recomputed otherwise. Aggregate with pft_report.py <set>/set.jsonl <run>/eval/c.*.own.jsonl,
# which checks the whole inventory (3,624 runs for the full set).
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
    if [ -e "$D/eval/$n.$v.jsonl" ]; then
      # reuse a cached evaluation only when it is current: its records' and sidecar's stamps (judging format,
      # comparator and reference sha256), its sidecar's sha256 of its records, of this chunk's runs and of its
      # expected models. Exit 3: current but its inventory failed, so the run fails; anything else: recompute
      $WRAP "$PY" -I -B "$HERE/pft_compare.py" --check-eval "$SET/set.jsonl" "$D/out/$n.out.jsonl" "$D/eval/$n.$v.jsonl" --ref "$v" --expect "$f"
      case $? in
        0) continue ;;
        3) echo "inventory failure $n $v (cached)" >&2; status=1; continue ;;
        *) echo "recomputing $n $v" >&2; rm -f "$D/eval/$n.$v.jsonl" "$D/eval/$n.$v.jsonl.inventory.json" ;;
      esac
    fi
    # --expect: this chunk's models; exit 3 is an inventory failure (missing, duplicate or unexpected runs): the
    # judged records are kept and the run fails
    $WRAP "$PY" -I -B "$HERE/pft_compare.py" "$SET/set.jsonl" "$D/out/$n.out.jsonl" "$D/eval/$n.$v.tmp" --ref "$v" \
      --expect "$f" --workers "${PFM3T_WORKERS:-6}"
    rc=$?
    case $rc in
      0) ;;
      3) echo "inventory failure $n $v" >&2; status=1 ;;
      *) echo "compare failed $n $v" >&2; status=1; continue ;;
    esac
    mv "$D/eval/$n.$v.tmp" "$D/eval/$n.$v.jsonl"
    mv "$D/eval/$n.$v.tmp.inventory.json" "$D/eval/$n.$v.jsonl.inventory.json"
  done
  echo "$(date -u +%T) done $n"
done
exit $status
