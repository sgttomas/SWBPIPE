#!/bin/bash
# I82: the TEXT chain's inputs are tree-identical between U8's basis (bd6b4be2c3, I72's Pass B) and
# the code basis (main d8c88774d0): the 15 crate_dirs.txt source trees, fixtures/results and the three
# static schemas. Git reads only. Usage: I82_WT=<WT> b1_tree_check.sh
T=${I82_WT:?}; export GIT_OPTIONAL_LOCKS=0; cd $T/numerics
A=bd6b4be2c33cc64edf3e273bc126083872d03e24; B=d8c88774d0a73bc99fe9c6e906db6296162f8953
for d in core/product_physics/src core/solver/frame_kernel/src core/reporting/result_export/src core/solver/nonlinear_integration/src core/loads/primitive_loads/src core/solver/straight_pipe/src core/loads/stress_recovery/src core/serialization/canonical_json/src core/solver/sparse_direct/src core/solver/diagnostics/src core/units/src core/solver/linear_supports/src core/solver/nonlinear_supports/src core/loads/load_case_algebra/src core/solver/curved_bend/src fixtures/results schemas/physics_source_recovery.schema.json schemas/retained_precision_mp_v2.schema.json schemas/source_block_recovery.schema.json core/product_physics/Cargo.toml core/product_physics/build.rs; do
  a=$(git rev-parse $A:projects/chirality-piping/$d); b=$(git rev-parse $B:projects/chirality-piping/$d)
  [ "$a" = "$b" ] && echo "same $a $d" || echo "DIFF $d"; done
