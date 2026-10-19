#!/bin/bash
# O4 declared-problem oracle: regenerate the J2 twins, run the product's dump entry and the
# headless runner on each twin in both solver modes, then run the oracle's self-test, the
# judged runs and the real-data controls. Then (R2b) the same for RF's PIN-OFF-345 twins and the
# frozen unit's judged-set documents, and (R2c) invented sensitive phi ~ 2 documents, each with a
# selection census. Generated files stay in scratch.
#
# Environment (defaults are the T3 host's):
#   O4DP_T3        T3 root holding tools/t3_cargo.sh and tools/t3_slot.sh  (the worktree's parent directory)
#   O4DP_SCRATCH   scratch directory for twins, runs and the report         ($O4DP_T3/scratch/o4-r2)
#   O4DP_TARGET    cargo target directory                                    ($O4DP_T3/targets/o4/r2)
#   O4DP_FROZEN    commit holding the frozen o4_retained_source unit         (d4dbf9187d)
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
PROJECT=$(cd "$HERE/../../../.." && pwd)
REPO=$(git -C "$PROJECT" rev-parse --show-toplevel)
T3=${O4DP_T3:-$(dirname "$REPO")}
[ -x "$T3/tools/t3_cargo.sh" ] && [ -x "$T3/tools/t3_slot.sh" ] || { echo "run.sh: set O4DP_T3 to the T3 root (tools/t3_cargo.sh, tools/t3_slot.sh)" >&2; exit 2; }
SCRATCH=${O4DP_SCRATCH:-$T3/scratch/o4-r2}
TARGET=${O4DP_TARGET:-$T3/targets/o4/r2}
FROZEN=${O4DP_FROZEN:-d4dbf9187d}
mkdir -p "$SCRATCH"

echo "== self-test (synthetic dumps)"
python3 -I -B "$HERE/oracle.py" --selftest

echo "== twins"
FROZEN_TWINS="$SCRATCH/frozen_source_twins_cases.json"
if git -C "$PROJECT" cat-file -e "$FROZEN:projects/chirality-piping/validation/references/o4_retained_source/source_twins_cases.json" 2>/dev/null; then
  git -C "$PROJECT" show "$FROZEN:projects/chirality-piping/validation/references/o4_retained_source/source_twins_cases.json" > "$FROZEN_TWINS"
  python3 -I -B "$HERE/twins.py" "$PROJECT" "$SCRATCH/twins" "$FROZEN_TWINS"
else
  echo "frozen unit $FROZEN not reachable; twins are not cross-checked against it"
  python3 -I -B "$HERE/twins.py" "$PROJECT" "$SCRATCH/twins"
fi

echo "== build (through the T3 cargo wrapper)"
( cd "$PROJECT/core/product_physics" && "$T3/tools/t3_cargo.sh" test --lib --no-run --target-dir "$TARGET" )
( cd "$PROJECT/core/runner/headless" && "$T3/tools/t3_cargo.sh" build --bin openpipestress-runner --target-dir "$TARGET" )
TEST_BIN=$(ls -t "$TARGET"/debug/deps/open_pipe_stress_product_physics-* | grep -E '/open_pipe_stress_product_physics-[0-9a-f]+$' | head -1)
RUNNER="$TARGET/debug/openpipestress-runner"

echo "== product runs (dump entry and headless runner, both modes; one T3 slot)"
rm -rf "$SCRATCH/runs"
( cd "$PROJECT/core/product_physics" && "$T3/tools/t3_slot.sh" python3 -I -B "$HERE/twins.py" run \
    "$SCRATCH/twins/index.json" "$SCRATCH/runs" "$TEST_BIN" "$RUNNER" "$PROJECT/core/product_physics" ) > "$SCRATCH/run.log" 2>&1
grep -c 'cli_rc=0' "$SCRATCH/run.log" | sed 's/^/runner runs with exit 0: /'

echo "== oracle"
REQUIRED="$HERE/required_selection.json"
python3 -I -B "$HERE/oracle.py" --index "$SCRATCH/twins/index.json" --runs "$SCRATCH/runs" \
    --out "$SCRATCH/report.json" --controls --required "$REQUIRED" --required-set j2 > "$SCRATCH/oracle.log"
echo "report: $SCRATCH/report.json"

echo "== R2b: RF PIN-OFF-345 twins and the frozen judged set"
FROZEN_CASES="$SCRATCH/frozen_cases.json"
git -C "$PROJECT" show "$FROZEN:projects/chirality-piping/validation/references/o4_retained_source/cases.json" > "$FROZEN_CASES"
python3 -I -B "$HERE/twins.py" extended "$PROJECT" "$SCRATCH/ext" "$FROZEN_CASES" "$FROZEN_TWINS"
echo "== R2c: invented sensitive phi ~ 2 documents"
python3 -I -B "$HERE/twins.py" phi2 "$SCRATCH/ext"
for SET in rf judged phi2; do
  rm -rf "$SCRATCH/ext/$SET/runs"
  ( cd "$PROJECT/core/product_physics" && "$T3/tools/t3_slot.sh" python3 -I -B "$HERE/twins.py" run \
      "$SCRATCH/ext/$SET/index.json" "$SCRATCH/ext/$SET/runs" "$TEST_BIN" "$RUNNER" "$PROJECT/core/product_physics" ) > "$SCRATCH/ext/$SET.run.log" 2>&1
  python3 -I -B "$HERE/twins.py" census "$SCRATCH/ext/$SET/index.json" "$SCRATCH/ext/$SET/runs" "$SCRATCH/ext/$SET/census.json"
done
# Each batch fails on any FAIL case, any error other than a recorded non-selection, a vacuous batch or
# control set, or any committed required (document, mode, case) that does not select and pass.
python3 -I -B "$HERE/oracle.py" --index "$SCRATCH/ext/rf/index.json" --runs "$SCRATCH/ext/rf/runs" \
    --out "$SCRATCH/ext/rf/report.json" --controls --required "$REQUIRED" --required-set rf > "$SCRATCH/ext/rf/oracle.log"
python3 -I -B "$HERE/oracle.py" --index "$SCRATCH/ext/judged/index.json" --runs "$SCRATCH/ext/judged/runs" \
    --out "$SCRATCH/ext/judged/report.json" --required "$REQUIRED" --required-set judged > "$SCRATCH/ext/judged/oracle.log"
python3 -I -B "$HERE/oracle.py" --index "$SCRATCH/ext/phi2/index.json" --runs "$SCRATCH/ext/phi2/runs" \
    --out "$SCRATCH/ext/phi2/report.json" --required "$REQUIRED" --required-set phi2 > "$SCRATCH/ext/phi2/oracle.log"
python3 -I -B - "$SCRATCH" <<'PY'
import json, sys
root = sys.argv[1]
for name, path in (("j2", "report.json"), ("rf", "ext/rf/report.json"), ("judged", "ext/judged/report.json"), ("phi2", "ext/phi2/report.json")):
    doc = json.load(open(f"{root}/{path}"))
    s = doc["summary"]
    counts = s["counts"].get("all=all", {"PASS": 0, "FAIL": 0})
    ctl = doc.get("controls")
    ctl_text = "" if not ctl else "; controls " + ", ".join(f"{k} {v['failed_as_required']} failed/{v['passed_wrongly']} passed" for k, v in ctl.items() if k not in ("vacuous", "checks_that_failed"))
    print(f"{name}: {s['runs']} runs, {s['cases']} cases PASS {counts['PASS']} FAIL {counts['FAIL']}, not selected {s['not_selected_runs']}, "
          f"required {s['required']['count']} missing {s['required']['missing']}, verdict {s['verdict']}{ctl_text}")
PY
