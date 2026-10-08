#!/bin/bash
# RV113: the Python mutants, one locked job each (pytest on the B1/RV108 tests of the two retained test files,
# then the reviewer's probes v5 and fg). Usage: run_py_mutants.sh <selection: b1|full> <ids...> (NONE = control)
WT=WT
S=$WT/scratch/rv113_rvr_01
VENV=VENV
B=$WT/targets/rv113-pybins/release
P=$WT/rv113/py-mut/projects/chirality-piping
sel=$1; shift
for id in "$@"; do
  out=$S/py/mutants/runs/$sel/$id; mkdir -p "$out" "$S/py/tmp/mut_$id"
  if [ "$sel" = full ]; then K=(); else K=(-k "b1_ or rv108"); fi
  cd "$P" || exit 90
  echo "# queued $(date -u '+%FT%TZ') $id $sel" > "$out/run.log"
  /usr/bin/lockf -k "$WT/guard/cargo_job.lock" /usr/bin/env RV113_MUT="$([ "$id" = NONE ] || echo "$id")" TMPDIR="$S/py/tmp/mut_$id" \
    OPENPIPESTRESS_CHECKED_JSON_BIN="$B/openpipestress_jcs_ijson" OPENPIPESTRESS_BINARY64_JSON_BIN="$B/openpipestress_jcs_binary64" OPENPIPESTRESS_UNITS_BIN="$B/openpipestress_units" \
    /bin/bash -c '
      echo "# start $(date -u "+%FT%TZ") RV113_MUT=${RV113_MUT:-unset}"
      "$0" -m pytest -p no:cacheprovider --basetemp="$TMPDIR/basetemp" -q -rf --junitxml="$1/pytest.xml" tests/test_retained_precision_contract.py tests/test_retained_precision_carriers.py "${@:4}" > "$1/pytest.log" 2>&1; echo "pytest rc=$?"
      if [ "$3" != full ]; then
        "$0" "$2/tools/rv113_py_harness.py" "$PWD" probes "$2/ts/probes/probes_v5.json" "$1/probes_v5.jsonl" > "$1/probes.log" 2>&1; echo "probes v5 rc=$?"
        "$0" "$2/tools/rv113_py_harness.py" "$PWD" probes "$2/py/fg/probes_fg.json" "$1/probes_fg.jsonl" >> "$1/probes.log" 2>&1; echo "probes fg rc=$?"
      fi
      echo "# end $(date -u "+%FT%TZ")"' "$VENV/bin/python" "$out" "$S" "$sel" "${K[@]}" >> "$out/run.log" 2>&1
done
