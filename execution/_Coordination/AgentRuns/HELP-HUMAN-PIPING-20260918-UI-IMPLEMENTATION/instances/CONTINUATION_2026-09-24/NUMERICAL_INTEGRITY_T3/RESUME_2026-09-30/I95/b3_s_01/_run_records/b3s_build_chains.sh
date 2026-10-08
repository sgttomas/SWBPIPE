#!/bin/bash
# I95 B3-S: build the four exact-route chain variants from I82's multi-case chain (Python only).
#   ur   union of both routes, D1.3 rules rebound (no new credit)
#   urc  union, rebound + the D1.5-exact credits
#   er   exact-route-only forms (legacy-only branches zeroed), rebound
#   erc  exact-route-only forms, rebound + credits (the main figure)
# Usage: I95_WT=<WT> b3s_build_chains.sh
set -euo pipefail
T=${I95_WT:?}; S=$T/scratch/i95_b3_s
export TMPDIR=$S/tmp PATH=$T/../../projects/chirality-piping/.venv/bin:$PATH PYTHONDONTWRITEBYTECODE=1
X="python3 $S/tools/b3s_exact_chain.py $S/mc_chain"; C="--census $S/out/b3s_census.out.json"; EO="--exact-only --edges $S/base/edges_base.json"
$X $S/ur $C
$X $S/urc $C --credits
$X $S/er $C $EO
$X $S/erc $C --credits $EO
