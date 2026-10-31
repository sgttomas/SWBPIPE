#!/bin/bash
# DEL-04-05 release timing window (RT-U4, criterion 9): ONE exclusive invocation that runs
# release_runner.py --run and then --packet for a named plan subset. Observation only.
#
# Usage (run it under the host's exclusive-job wrapper; it builds and generates nothing):
#   release_window.sh <models_dir> <out_root> <release_timing binary> [subset]
#     subset: window (default: the runs sized for one window of at most 15 min), all, linear,
#             nonlinear, nonlinear-pf, nonlinear-ex, criterion,
#             or comma-separated run ids (see: release_runner.py --plan --subset <subset>).
#   Environment: RELEASE_WINDOW_BUDGET_S (default 900: the whole window stays within 15 min;
#   a run whose estimate exceeds the remaining budget is skipped and logged, and each timeout
#   is capped at the remaining budget), RELEASE_WINDOW_WRITE_RESULTS=1 (keep result bytes;
#   about 1.5 GB per 2,000-node result), RELEASE_WINDOW_MODE (sparse, default; or dense).
# Prepare first: release_runner.py --generate --models <models_dir> (the rack models and their
# manifest) and a release build of the driver (core/solver/performance_harness/release).
# Each run's elapsed time is printed as it ends; records, packet.json and window.log go to a new
# <out_root>/window-<UTC time>/ folder.
set -u
[ $# -ge 3 ] || { sed -n '2,17p' "$0" >&2; exit 2; }
HERE=$(cd "$(dirname "$0")" && pwd)
MODELS=$1
ROOT=$2
BIN=$3
SUBSET=${4:-window}
BUDGET=${RELEASE_WINDOW_BUDGET_S:-900}
MODE=${RELEASE_WINDOW_MODE:-sparse}
[ -x "$BIN" ] || { echo "release_window: driver binary not found or not executable" >&2; exit 2; }
[ -f "$MODELS/manifest.json" ] || { echo "release_window: no manifest.json in the models folder (run --generate)" >&2; exit 2; }
OUT=$ROOT/window-$(date -u +%Y%m%dT%H%M%SZ)
mkdir -p "$OUT" || exit 2
START=$(date +%s)
log() { echo "$(date -u '+%FT%TZ') +$(( $(date +%s) - START ))s $*" | tee -a "$OUT/window.log"; }

log "START subset=$SUBSET mode=$MODE budget=${BUDGET}s"
python3 -u "$HERE/release_runner.py" --plan --subset "$SUBSET" --mode "$MODE" --budget-s "$BUDGET" | tee -a "$OUT/window.log"
WRITE=()
[ "${RELEASE_WINDOW_WRITE_RESULTS:-0}" = 1 ] && WRITE=(--write-results)
python3 -u "$HERE/release_runner.py" --run --subset "$SUBSET" --mode "$MODE" --budget-s "$BUDGET" \
  --models "$MODELS" --out "$OUT" --binary "$BIN" ${WRITE[@]+"${WRITE[@]}"} 2>&1 | tee -a "$OUT/window.log"
RC=${PIPESTATUS[0]}
log "RUNS DONE rc=$RC"
python3 -u "$HERE/release_runner.py" --packet --out "$OUT" | tee -a "$OUT/window.log"
log "WINDOW DONE; records and packet.json in $(basename "$OUT") under the out root"
exit "$RC"
