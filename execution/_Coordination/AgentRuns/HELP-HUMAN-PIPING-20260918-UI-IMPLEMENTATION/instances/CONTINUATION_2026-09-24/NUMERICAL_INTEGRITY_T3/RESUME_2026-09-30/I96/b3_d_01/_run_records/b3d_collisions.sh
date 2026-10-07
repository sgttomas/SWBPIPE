#!/bin/sh
# I96 B3-D collision check: fixed-string git grep at NUM's HEAD, read-only.
# Usage: b3d_collisions.sh <NUM> > b3d_collisions.out.txt
NUM="$1"
cd "$NUM" || exit 2
export GIT_OPTIONAL_LOCKS=0
echo "HEAD $(git rev-parse HEAD)"
echo "maintained tree vs main 2007709549 (P/core, P/fixtures, P/schemas, P/apps, P/tests): $(git diff --quiet 2007709549 HEAD -- projects/chirality-piping/core projects/chirality-piping/fixtures projects/chirality-piping/schemas projects/chirality-piping/apps projects/chirality-piping/tests && echo equal || echo DIFFERENT)"
for name in \
  'RP-PREPARED-EXACT-DUAL-v1' \
  'retained_precision_prepared_exact_v1' \
  'semantic_contract_v0_3_physics_retained_1' \
  'openpipestress.result_semantics/0.3.0/physics-retained-1' \
  'exact_straight_retained_w1a_v2' \
  'receipt_bindings' \
  'base_common_E_nu_derived_G' \
  'exact_straight_W1a' \
  'explicitly_empty_pressure_regions_and_no_pressure_primitives' \
  'ordinary_profile_prepared_formation' \
  'named_point_common_E_nu' \
  'interpolated_common_E_nu' \
  'pressure_primitives' \
  'constant_effort_supports' \
  'load_reference_states' \
  'retained_physics' \
  'PHYSICS_RETAINED' \
  'physics_retained' \
  'N_RETAINED_PHYSICS_OUTPUT' \
  'RETAINED_PHYSICS_OUTPUT_REFUSAL' \
  'EXACT_DEFINITION_ID' \
  'EXACT_DEFINITION_HASH' \
  'EXACT_TABLE_HASH' \
  'EXACT_CONTRACT_ID' \
  'retained_precision_exact_successor' \
  'BaseENu' \
  'retained_precision_formation_exact_v1' \
  'RETAINED_PRECISION_OUTPUT_NOT_YET_AVAILABLE' \
  ; do
  outside=$(git grep -F -l -e "$name" HEAD -- projects/chirality-piping ':!projects/chirality-piping/execution' | wc -l | tr -d ' ')
  inside=$(git grep -F -l -e "$name" HEAD -- projects/chirality-piping/execution | wc -l | tr -d ' ')
  repo=$(git grep -F -l -e "$name" HEAD -- . ':!projects/chirality-piping' | wc -l | tr -d ' ')
  echo "== $name | maintained(P outside execution): $outside | P/execution: $inside | rest of repo: $repo"
  git grep -F -n -e "$name" HEAD -- projects/chirality-piping ':!projects/chirality-piping/execution' | sed 's/^HEAD://' | cut -c1-220 | head -8
done
