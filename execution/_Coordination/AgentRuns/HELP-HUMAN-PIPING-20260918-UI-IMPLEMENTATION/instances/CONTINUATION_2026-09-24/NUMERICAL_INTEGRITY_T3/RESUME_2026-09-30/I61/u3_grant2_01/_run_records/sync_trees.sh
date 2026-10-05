#!/bin/bash
# Copy the candidate's changed files from the worktree into the scratch trees (cand, mut).
set -eu
T=WT; S=$T/scratch/i61_u3_grant2_01; W=$T/f2a-memory/projects/chirality-piping/core
for d in cand mut; do
  C=$S/$d/projects/chirality-piping/core/product_physics/src
  for f in lib.rs retained_product.rs retained_facade_tests.rs retained_wire_tests.rs retained_tests_hooks/grant2.rs; do cp $W/product_physics/src/$f $C/$f; done
done
printf '\n#[cfg(test)]\nmod zz_i61_u3g2_sweep;\n' >> $S/cand/projects/chirality-piping/core/product_physics/src/lib.rs
diff -rq -x target $W $S/mut/projects/chirality-piping/core && echo "== mut equals the worktree core tree"
diff -rq -x target -x zz_i61_u3g2_sweep.rs $W $S/cand/projects/chirality-piping/core | sed "s#$T#WT#g"
