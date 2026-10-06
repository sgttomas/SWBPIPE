#!/bin/bash
# usage: run_py_auth.sh <cand|base> <log> tests...   (CLI authorities built by RV101 through the lock)
c=$1; log=$2; shift 2
cd WT/rv101/$c/projects/chirality-piping || exit 2
mkdir -p WT/scratch/rv101_t6s_01/tmp/pytest_auth_$c
OPENPIPESTRESS_CHECKED_JSON_BIN=WT/targets/rv101/checked-json/release/openpipestress_jcs_ijson OPENPIPESTRESS_UNITS_BIN=WT/targets/rv101/units-authority/release/openpipestress_units PYTHONDONTWRITEBYTECODE=1 TMPDIR=WT/scratch/rv101_t6s_01/tmp GIT_CEILING_DIRECTORIES=WT/rv101 PATH=WT/scratch/rv101_t6s_01/tools/shim:$PATH VENV/bin/python -m pytest -q -p no:cacheprovider -rA -n 4 --dist loadscope --basetemp WT/scratch/rv101_t6s_01/tmp/pytest_auth_$c "$@" > WT/scratch/rv101_t6s_01/logs/$log 2>&1
echo "rc=$?" >> WT/scratch/rv101_t6s_01/logs/$log
