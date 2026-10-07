#!/bin/bash
# RV109 R3′: the interim one-case continuation, on the SP copy as committed (LOAD_CASES = 1), then on a
# reviewer-only build of the same copy with caps::LOAD_CASES = 3 (simulating I2). Restored afterwards.
set -u
WT=WT
S=$WT/scratch/rv109_rvp_01
X=$S/r3p
J=$S/runjob.sh
SP_SHA=56c5579f07b1cf3203c178e10add51bba0b24a58
SPD=$WT/rv109/sp/projects/chirality-piping/core/product_physics
D=$SPD/src
cp $X/probe/zz_rv109_probe.rs $D/zz_rv109_probe.rs
cp $X/probe/zz_rv109_rev.sp.rs $D/zz_rv109_rev.rs
grep -q 'mod zz_rv109_probe' $D/retained_memory_witness_tests.rs || printf '\n#[cfg(test)]\n#[path = "zz_rv109_probe.rs"]\nmod zz_rv109_probe;\n' >> $D/retained_memory_witness_tests.rs
$J r3p_interim_c1 $SPD rv109-sp-probe 0 test --locked --offline --lib zz_rv109_interim -- --ignored --nocapture --test-threads=1
cp $D/retained_memory.rs $X/logs/retained_memory.pristine.rs
python3 - "$D/retained_memory.rs" <<'PY'
import sys
p=sys.argv[1]; t=open(p).read()
old="    pub(crate) const LOAD_CASES: usize = 1;"
assert t.count(old)==1, "LOAD_CASES line"
open(p,"w").write(t.replace(old,"    pub(crate) const LOAD_CASES: usize = 3;"))
PY
$J r3p_interim_c3 $SPD rv109-sp-probe 0 test --locked --offline --lib zz_rv109_interim -- --ignored --nocapture --test-threads=1
cp $X/logs/retained_memory.pristine.rs $D/retained_memory.rs
cd $WT/b1 && GIT_OPTIONAL_LOCKS=0 git show $SP_SHA:projects/chirality-piping/core/product_physics/src/retained_memory_witness_tests.rs > $D/retained_memory_witness_tests.rs
rm -f $D/zz_rv109_probe.rs $D/zz_rv109_rev.rs
shasum -a 256 $SPD/src/*.rs $SPD/src/retained_tests_hooks/grant2.rs | sed 's|/.*/src/|PP/|' > $X/logs/sp_restored2_src.sha256
echo INTERIM_DONE $(date -u '+%FT%TZ') > $X/logs/interim.done
