#!/bin/bash
# M3 judge driver, both routes. It builds the runner itself from a clean checkout, runs it on every chunk (raw outputs
# only, fresh each time), and judges the whole collection once per variant with the binary's sha256, the checkout's
# head and the checkout passed to the judging pass (m3_report.py).
#   run.sh <route> <checkout> <set dir> <run dir> own [twin] [control]
#     <route>    pf (the PF M3-T set) or exact (the K-D5 M3-T set, kd5_timoshenko/)
#     <checkout> a clean git checkout whose committed runner and product are judged (runner/build.sh refuses a dirty one)
#     <set dir>  holds set.jsonl (the generator's full set) and, for control, controls.jsonl (exact route)
#     <run dir>  holds chunks/c.NNN.in.jsonl (the generator's split; each run with both entries) and
#                chunks/c.NNN.typed.in.jsonl (models whose reference declares the typed entry alone, run with it alone);
#                runner.bin, out/, report.*, eval.* are rewritten
#     own        required; twin (the twin theory's reference) and control (exact route: E perturbed by 2^-20) are
#                sensitivity controls
# Environment: M3J_PY (python, default python3); M3J_WRAP (a command prefix for the build, the runner and the judging
# pass, for example a host's resource wrapper); M3J_WORKERS (judging workers, default 6); M3J_TARGET (cargo target dir,
# default <run dir>/cargo-target); M3J_CARGO (cargo for build.sh); M3J_BUILD (the builder, default runner/build.sh; the
# self-test substitutes a synthetic one).
# Exit 0, with the final line OVERALL: PASS, only when the build and every runner invocation succeeded, the own
# judgement passed and every requested control completed with provenance and inventory passing and M3 failing.
# Otherwise the final line is OVERALL: FAIL.
set -u
finish() { [ "$1" = 0 ] && echo "OVERALL: PASS" || echo "OVERALL: FAIL"; exit "$1"; }
[ $# -ge 5 ] || { sed -n '2,20p' "$0" >&2; finish 2; }
HERE=$(cd "$(dirname "$0")" && pwd -P)
ROUTE=$1
CHECKOUT=$2
SET=$3
D=$4
shift 4
case $ROUTE in pf | exact) ;; *) echo "unknown route $ROUTE" >&2; finish 2 ;; esac
PY=${M3J_PY:-python3}
WRAP=${M3J_WRAP:-}
BUILD=${M3J_BUILD:-$HERE/runner/build.sh}
WANT_OWN=0
for v in "$@"; do
  case $v in
    own) WANT_OWN=1 ;;
    twin) ;;
    control) [ "$ROUTE" = exact ] || { echo "control is an exact-route variant" >&2; finish 2; } ;;
    *) echo "unknown variant $v" >&2; finish 2 ;;
  esac
done
[ $WANT_OWN = 1 ] || { echo "the own judgement is required" >&2; finish 2; }
mkdir -p "$D"
rm -rf "$D/out" "$D/runner.bin" "$D"/report.* "$D"/eval.*
mkdir -p "$D/out"
$WRAP "$BUILD" "$CHECKOUT" "${M3J_TARGET:-$D/cargo-target}" "$D/runner.bin" >&2 || { echo "build failed" >&2; finish 1; }
RUNNER_SHA=$(shasum -a 256 "$D/runner.bin" | cut -d' ' -f1)
HEAD_SHA=$(git -C "$CHECKOUT" rev-parse --verify HEAD^{commit}) || finish 1
echo "runner $RUNNER_SHA at $HEAD_SHA"
status=0
for f in "$D"/chunks/c.[0-9][0-9][0-9].in.jsonl "$D"/chunks/c.[0-9][0-9][0-9].typed.in.jsonl; do
  [ -e "$f" ] || continue
  n=$(basename "$f" .in.jsonl)
  case $n in *.typed) entries=typed ;; *) entries=typed,captured ;; esac
  KD5_IN=$f KD5_OUT=$D/out/$n.out.jsonl KD5_ENTRIES=$entries $WRAP "$D/runner.bin"
  rc=$?
  [ $rc = 0 ] || { echo "runner exited $rc on $n" >&2; status=1; }
  echo "$(date -u +%T) ran $n (exit $rc)"
done
for v in "$@"; do
  CTL=()
  [ "$v" = control ] && CTL=(--controls "$SET/controls.jsonl")
  $WRAP "$PY" -I -B "$HERE/m3_report.py" --route "$ROUTE" "$SET/set.jsonl" "$D"/out/c.*.out.jsonl --ref "$v" ${CTL[@]+"${CTL[@]}"} \
    --runner-sha256 "$RUNNER_SHA" --product-head "$HEAD_SHA" --checkout "$CHECKOUT" --workers "${M3J_WORKERS:-6}" \
    --eval-out "$D/eval.$v.jsonl" --json "$D/report.$v.json" > "$D/report.$v.log" 2>&1
  rc=$?
  echo "$v: $(tail -1 "$D/report.$v.log") (exit $rc)"
  if [ "$v" = own ]; then
    [ $rc = 0 ] || status=1
  else
    # a sensitivity control counts only from a completed report: provenance and inventory PASS, M3 FAIL
    "$PY" -I -B -c 'import sys
sys.path.insert(0, sys.argv[2]); import m3_json
j, p = m3_json.read_json(sys.argv[1])
ok = not p and isinstance(j, dict) and j.get("provenance_pass") is True and j.get("inventory", {}).get("pass") is True and j.get("m3_pass") is False
sys.exit(0 if ok else 1)' "$D/report.$v.json" "$HERE" || { echo "the $v sensitivity control did not complete as a control" >&2; status=1; }
  fi
done
finish $status
