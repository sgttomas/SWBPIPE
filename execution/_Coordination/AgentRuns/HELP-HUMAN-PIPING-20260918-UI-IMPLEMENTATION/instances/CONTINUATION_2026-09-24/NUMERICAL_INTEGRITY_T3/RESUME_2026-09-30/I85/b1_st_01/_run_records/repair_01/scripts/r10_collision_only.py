#!/usr/bin/env python3
"""I85 B1-ST repair 1: R10 against the collision variant alone. In MUT, RV109's R10 edit in lib.rs
plus one test edit that removes the pin's capacity assertion (so only the collision variant can
refuse R10); PP's whole --lib suite, registered, through WT/tools/t3_cargo.sh; both files restored
and checked. Usage: r10_collision_only.py MUT_P LOG TARGET_DIR T3_CARGO TMPDIR MUTANTS_PY"""
import hashlib, os, re, subprocess, sys
M, LOG, TD, CARGO, TMP, MPY = sys.argv[1:7]
PP = os.path.join(M, "core/product_physics"); LIB = os.path.join(PP, "src/lib.rs"); TST = os.path.join(PP, "src/retained_facade_tests.rs")
src = open(MPY).read(); ns = {}
exec(src[src.index("MUTANTS = ["):src.index("]\nENV") + 1], {"LIB": LIB}, ns)
_, _, r10_old, r10_new = [m for m in ns["MUTANTS"] if m[0] == "R10_t4_after_reservation"][0]
cap_old = """        assert_eq!((envelope.diagnostics.capacity(), envelope.diagnostics.len()), (capacity, capacity),
            "{label} {mode:?}: T-4 precedes R-2: no diagnostics slot was reserved");
"""
edits = [(LIB, r10_old, r10_new), (TST, cap_old, "        let _ = capacity;\n")]
pristine = {p: open(p, "rb").read() for p, _, _ in edits}
for p, old, new in edits:
    t = pristine[p].decode(); assert t.count(old) == 1, p
    open(p, "wb").write(t.replace(old, new).encode())
env = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2", CARGO_TARGET_DIR=TD, TMPDIR=TMP)
env.pop("RUSTFLAGS", None); env.pop("CARGO_ENCODED_RUSTFLAGS", None)
with open(LOG, "w") as fh:
    rc = subprocess.run([CARGO, "test", "--locked", "--offline", "--lib", "--no-fail-fast"], cwd=PP, env=env, stdout=fh, stderr=subprocess.STDOUT).returncode
for p, data in pristine.items():
    open(p, "wb").write(data); assert open(p, "rb").read() == data
out = open(LOG, encoding="utf-8", errors="replace").read()
print("rc", rc, "compiled", "Running unittests" in out)
for t, a, m in re.findall(r"^thread '([^']+)' \(\d+\) panicked at ([^\n]+):\n([^\n]*)", out, re.M):
    print("P", t, "|", m[:200])
print(re.findall(r"^test result: .*$", out, re.M))
print({os.path.basename(p): hashlib.sha256(open(p, "rb").read()).hexdigest() for p in pristine})
