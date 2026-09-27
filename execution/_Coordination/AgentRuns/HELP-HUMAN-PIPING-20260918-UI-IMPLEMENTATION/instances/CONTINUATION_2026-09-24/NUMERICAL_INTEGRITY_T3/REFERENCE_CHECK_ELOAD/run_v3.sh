#!/bin/sh
# V3 pipeline (standard-library Python only). Usage, from REFERENCE_CHECK_ELOAD/:
#   sh run_v3.sh <out_dir> <scratch_dir>
# <out_dir> receives the JSON outputs and stdouts; <scratch_dir> receives the regeneration run of the
# author's script (it writes references_eload.json next to itself, so it runs on a copy, never in place).
set -e
OUT=${1:-_run_records}
SCR=${2:?usage: sh run_v3.sh <out_dir> <scratch_dir>}
REF=../REFERENCES_ELOAD
mkdir -p "$OUT" "$SCR/regen"
(cd "$REF" && sha256sum -c _run_records/SHA256SUMS) > "$OUT/package_sha256_check.txt"
python3 -c 'import sys; print(sys.version)' > "$OUT/PYTHON_VERSION.txt"
nice -n 19 python3 -B v3_derive.py  "$REF/references_eload.json" "$OUT/v3_values.json"  > "$OUT/v3_derive.stdout.txt"
nice -n 19 python3 -B v3_compare.py "$REF/references_eload.json" "$OUT/v3_values.json" "$OUT/v3_compare.json" > "$OUT/v3_compare.stdout.txt"
nice -n 19 python3 -B v3_cancel.py  "$REF/references_eload.json" "$OUT/v3_cancel.json"  > "$OUT/v3_cancel.stdout.txt"
nice -n 19 python3 -B v3_nc.py      "$REF/references_eload.json" "$OUT/v3_nc.json"      > "$OUT/v3_nc.stdout.txt"
nice -n 19 python3 -B v3_floor.py   "$REF/references_eload.json" "$OUT/v3_floor.json"   > "$OUT/v3_floor.stdout.txt"
nice -n 19 python3 -B v3_checks.py  "$REF/references_eload.json" "$OUT/v3_checks.json"  > "$OUT/v3_checks.stdout.txt"
# regeneration of the package, byte for byte, on a copy of the script
cp "$REF/references_eload.py" "$SCR/regen/references_eload.py"
(cd "$SCR/regen" && nice -n 19 python3 -B references_eload.py > stdout.txt 2> stderr.txt)
{
  echo "script sha256: $(sha256sum "$SCR/regen/references_eload.py" | cut -d' ' -f1)"
  echo "regenerated json sha256: $(sha256sum "$SCR/regen/references_eload.json" | cut -d' ' -f1)"
  cmp "$SCR/regen/references_eload.json" "$REF/references_eload.json" && echo "json: byte-identical to the package"
  cmp "$SCR/regen/stdout.txt" "$REF/_run_records/references_eload.stdout.txt" && echo "stdout: identical to _run_records/references_eload.stdout.txt"
  echo "stderr bytes: $(wc -c < "$SCR/regen/stderr.txt")"
} > "$OUT/regeneration.txt"
echo done
