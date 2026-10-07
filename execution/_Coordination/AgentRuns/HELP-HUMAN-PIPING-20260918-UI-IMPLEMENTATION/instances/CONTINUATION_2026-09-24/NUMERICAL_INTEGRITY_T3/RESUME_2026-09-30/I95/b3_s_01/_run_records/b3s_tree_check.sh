#!/bin/bash
# I95 B3-S: the TEXT chain's inputs are tree-identical between U8's basis bd6b4be2c3 (to which I82 line-mapped
# the rules) and main 2007709549 (this study's basis): the 15 crate_dirs.txt source trees, the three static
# schemas, PP's Cargo.toml and build.rs (I82's b1_tree_check.sh list), and each g4_caps.py STATIC_FILES entry.
# fixtures/results as a whole differs (B6's corpus 07m, carrier cases and derivative fixtures), so its chain
# statics are checked one by one. Git reads only. Usage: I95_WT=<WT> b3s_tree_check.sh
T=${I95_WT:?}; export GIT_OPTIONAL_LOCKS=0; cd $T/numerics
A=bd6b4be2c33cc64edf3e273bc126083872d03e24; B=2007709549e9701b302e0eb62a1474d82acc1c40
for d in core/product_physics/src core/solver/frame_kernel/src core/reporting/result_export/src core/solver/nonlinear_integration/src core/loads/primitive_loads/src core/solver/straight_pipe/src core/loads/stress_recovery/src core/serialization/canonical_json/src core/solver/sparse_direct/src core/solver/diagnostics/src core/units/src core/solver/linear_supports/src core/solver/nonlinear_supports/src core/loads/load_case_algebra/src core/solver/curved_bend/src schemas/physics_source_recovery.schema.json schemas/retained_precision_mp_v2.schema.json schemas/source_block_recovery.schema.json core/product_physics/Cargo.toml core/product_physics/build.rs fixtures/results/retained_precision_prepared_ordinary_v1.json fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json fixtures/results/semantic_contract_v0_2.json fixtures/results/semantic_contract_v0_3_precision_1.json fixtures/results/semantic_contract_v0_3_physics_1.json fixtures/results/semantic_contract_v0_3_load_reference_1.json fixtures/results/semantic_contract_v0_3_load_reference_source_1.json fixtures/results/semantic_contract_v0_3_preview_physics_1.json fixtures/results/semantic_contract_v0_3_physics_source_1.json fixtures/results/semantic_contract_v0_3_source_blocks_1.json; do
  a=$(git rev-parse $A:projects/chirality-piping/$d); b=$(git rev-parse $B:projects/chirality-piping/$d)
  [ "$a" = "$b" ] && echo "same $a $d" || echo "DIFF $d"; done
echo "fixtures/results files that differ (none is a chain static):"
git diff --name-only $A $B -- projects/chirality-piping/fixtures/results | sed 's#^projects/chirality-piping/##'
