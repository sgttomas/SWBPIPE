#!/bin/bash
# PF M3-T driver (refutation round 3): the runner writes raw outputs only, fresh on every invocation, and one judging
# pass (pft_report.py) judges the whole collection directly from them and the reference. Nothing is reused.
#   run.sh <runner binary> <set dir> <run dir> [variant...]
#     <set dir>  holds set.jsonl (pft_generate.py full)
#     <run dir>  holds chunks/c.NNN.in.jsonl (pft_generate.py split); out/ is emptied and rewritten, and each variant
#                writes report.<variant>.json and eval.<variant>.jsonl (outputs, never read back)
#     variant    own (default) or twin (the twin theory's reference: the sensitivity control, expected to fail)
# Environment: PFM3T_PY (python, default python3); PFM3T_WRAP (a command prefix for the runner and the judging pass,
# for example a host's resource wrapper; default none); PFM3T_WORKERS (judging workers, default 6).
# Exit 0 only when every runner invocation succeeded, the own judgement is OVERALL: PASS and a twin control fails.
set -u
[ $# -ge 3 ] || { sed -n '2,12p' "$0" >&2; exit 2; }
HERE=$(cd "$(dirname "$0")" && pwd -P)
BIN=$1
SET=$2
D=$3
shift 3
VARIANTS=${*:-own}
PY=${PFM3T_PY:-python3}
WRAP=${PFM3T_WRAP:-}
RUNNER_SHA=$(shasum -a 256 "$BIN" | cut -d' ' -f1)
rm -rf "$D/out"
mkdir -p "$D/out"
status=0
for f in "$D"/chunks/c.[0-9][0-9][0-9].in.jsonl; do
  [ -e "$f" ] || continue
  n=$(basename "$f" .in.jsonl)
  KD5_IN=$f KD5_OUT=$D/out/$n.out.jsonl KD5_ENTRIES=typed,captured $WRAP "$BIN" || { echo "runner failed $n" >&2; status=1; }
  echo "$(date -u +%T) ran $n"
done
for v in $VARIANTS; do
  case $v in
    own | twin) ;;
    *) echo "unknown variant $v" >&2; exit 2 ;;
  esac
  $WRAP "$PY" -I -B "$HERE/pft_report.py" "$SET/set.jsonl" "$D"/out/c.*.out.jsonl --ref "$v" --runner-sha256 "$RUNNER_SHA" \
    --workers "${PFM3T_WORKERS:-6}" --eval-out "$D/eval.$v.jsonl" --json "$D/report.$v.json" > "$D/report.$v.log"
  rc=$?
  tail -1 "$D/report.$v.log"
  case $v:$rc in
    own:0 | twin:1) ;;
    twin:0) echo "the twin sensitivity control did not fail" >&2; status=1 ;;
    *) status=1 ;;
  esac
done
exit $status
