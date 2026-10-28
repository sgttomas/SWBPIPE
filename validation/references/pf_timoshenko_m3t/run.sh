#!/bin/bash
# PF M3-T driver (refutation rounds 3 and 4). It builds the runner itself from a clean checkout, runs it on every
# chunk (raw outputs only, fresh each time), and judges the whole collection once per variant with the binary's
# sha256, the checkout's head and the checkout passed to the judging pass (pft_report.py).
#   run.sh <checkout> <set dir> <run dir> own [twin]
#     <checkout> a clean git checkout whose committed runner and product are judged (runner/build.sh refuses a dirty one)
#     <set dir>  holds set.jsonl (pft_generate.py full)
#     <run dir>  holds chunks/c.NNN.in.jsonl (pft_generate.py split); runner.bin, out/, report.*, eval.* are rewritten
#     own        required; twin, the twin theory's reference, is the sensitivity control
# Environment: PFM3T_PY (python, default python3); PFM3T_WRAP (a command prefix for the build, the runner and the
# judging pass, for example a host's resource wrapper); PFM3T_WORKERS (judging workers, default 6); PFM3T_TARGET
# (cargo target dir, default <run dir>/cargo-target); PFM3T_CARGO (cargo for build.sh); PFM3T_BUILD (the builder,
# default runner/build.sh; the self-test substitutes a synthetic one).
# Exit 0, with the final line OVERALL: PASS, only when the build and every runner invocation succeeded, the own
# judgement passed and, if requested, the twin control completed with provenance and inventory passing and M3 failing.
# Otherwise the final line is OVERALL: FAIL.
set -u
finish() { [ "$1" = 0 ] && echo "OVERALL: PASS" || echo "OVERALL: FAIL"; exit "$1"; }
[ $# -ge 4 ] || { sed -n '2,19p' "$0" >&2; finish 2; }
HERE=$(cd "$(dirname "$0")" && pwd -P)
CHECKOUT=$1
SET=$2
D=$3
shift 3
PY=${PFM3T_PY:-python3}
WRAP=${PFM3T_WRAP:-}
BUILD=${PFM3T_BUILD:-$HERE/runner/build.sh}
WANT_OWN=0
for v in "$@"; do
  case $v in
    own) WANT_OWN=1 ;;
    twin) ;;
    *) echo "unknown variant $v" >&2; finish 2 ;;
  esac
done
[ $WANT_OWN = 1 ] || { echo "the own judgement is required" >&2; finish 2; }
mkdir -p "$D"
rm -rf "$D/out" "$D/runner.bin" "$D"/report.* "$D"/eval.*
mkdir -p "$D/out"
$WRAP "$BUILD" "$CHECKOUT" "${PFM3T_TARGET:-$D/cargo-target}" "$D/runner.bin" >&2 || { echo "build failed" >&2; finish 1; }
RUNNER_SHA=$(shasum -a 256 "$D/runner.bin" | cut -d' ' -f1)
HEAD_SHA=$(git -C "$CHECKOUT" rev-parse --verify HEAD^{commit}) || finish 1
echo "runner $RUNNER_SHA at $HEAD_SHA"
status=0
for f in "$D"/chunks/c.[0-9][0-9][0-9].in.jsonl; do
  [ -e "$f" ] || continue
  n=$(basename "$f" .in.jsonl)
  KD5_IN=$f KD5_OUT=$D/out/$n.out.jsonl KD5_ENTRIES=typed,captured $WRAP "$D/runner.bin"
  rc=$?
  [ $rc = 0 ] || { echo "runner exited $rc on $n" >&2; status=1; }
  echo "$(date -u +%T) ran $n (exit $rc)"
done
for v in "$@"; do
  $WRAP "$PY" -I -B "$HERE/pft_report.py" "$SET/set.jsonl" "$D"/out/c.*.out.jsonl --ref "$v" --runner-sha256 "$RUNNER_SHA" \
    --product-head "$HEAD_SHA" --checkout "$CHECKOUT" --workers "${PFM3T_WORKERS:-6}" --eval-out "$D/eval.$v.jsonl" \
    --json "$D/report.$v.json" > "$D/report.$v.log" 2>&1
  rc=$?
  echo "$v: $(tail -1 "$D/report.$v.log") (exit $rc)"
  if [ "$v" = own ]; then
    [ $rc = 0 ] || status=1
  else
    # the twin control counts only from a completed report: provenance and inventory PASS, M3 FAIL
    "$PY" -I -B -c 'import json, sys
sys.path.insert(0, sys.argv[2]); import pft_compare as C
j = C.strict_loads(open(sys.argv[1]).read())
ok = j.get("provenance_pass") is True and j.get("inventory", {}).get("pass") is True and j.get("m3_pass") is False
sys.exit(0 if ok else 1)' "$D/report.twin.json" "$HERE" || { echo "the twin sensitivity control did not complete as a control" >&2; status=1; }
  fi
done
finish $status
