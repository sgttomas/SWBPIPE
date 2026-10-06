#!/bin/bash
# usage: run_py.sh <cand|base> <log> tests...
c=$1; log=$2; shift 2
cd WT/rv101/$c/projects/chirality-piping || exit 2
mkdir -p WT/scratch/rv101_t6s_01/tmp/pytest_$c
OPENPIPESTRESS_CHECKED_JSON_BIN=/nonexistent/rv101-dummy-checked-json OPENPIPESTRESS_UNITS_BIN=/nonexistent/rv101-dummy-units PYTHONDONTWRITEBYTECODE=1 TMPDIR=WT/scratch/rv101_t6s_01/tmp GIT_CEILING_DIRECTORIES=WT/rv101 VENV/bin/python -m pytest -q -p no:cacheprovider -rA --basetemp WT/scratch/rv101_t6s_01/tmp/pytest_$c "$@" > WT/scratch/rv101_t6s_01/logs/$log 2>&1
echo "rc=$?" >> WT/scratch/rv101_t6s_01/logs/$log
