#!/bin/sh
# Regenerate every frozen file of this unit (judged set and source twins) and check it byte for byte
# (no writes), then verify SHA256SUMS.
#   ./run.sh            check (about a minute on 12 workers)
#   ./run.sh --workers 1
# Standard-library Python only; reads no product code or output.
set -eu
cd "$(dirname "$0")"
python3 -I -B generators/o4_reference.py "$@"
python3 -I -B generators/o4_source_twins.py "$@"
shasum -a 256 -c SHA256SUMS
