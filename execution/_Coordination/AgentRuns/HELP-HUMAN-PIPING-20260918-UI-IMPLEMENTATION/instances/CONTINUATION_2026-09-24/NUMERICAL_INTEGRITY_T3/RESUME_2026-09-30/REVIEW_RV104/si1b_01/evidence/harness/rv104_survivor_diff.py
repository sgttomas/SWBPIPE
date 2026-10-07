#!/usr/bin/env python3
"""RV104 scratch (not repository content): runs RV104's evaluator and runner
differential harnesses on each named surviving mutant (WT/rv104/mut), through
the T3 cargo lock, and restores the file afterwards. Usage: WT NAME..."""
import os, subprocess, sys
from pathlib import Path
WT = Path(sys.argv[1]); NAMES = sys.argv[2:]
S = WT / "scratch/rv104_si1b_01"; P = WT / "rv104/mut/projects/chirality-piping"
LIB = P / "core/rules/expression_evaluator/src/lib.rs"
src = (S / "harness/rv104_mutants.py").read_text().split("def cargo")[0]
src = src.replace("WT = Path(sys.argv[1])", "WT = Path(%r)" % str(WT)).replace("OUT = Path(sys.argv[2])", "OUT = None").replace("ONLY = sys.argv[3:]", "ONLY = []")
ns = {}; exec(src, ns); MUTANTS = ns["MUTANTS"]; ORIGINAL = LIB.read_text()
tests = P / "core/rules/rule_check_runner/tests"
for h in ("rv104_ee_diff.rs", "rv104_run_diff.rs"):
    (tests / h).write_text((S / "harness" / h).read_text())
try:
    for name in NAMES:
        text = ORIGINAL
        for old, new in MUTANTS[name]:
            assert text.count(old) == 1; text = text.replace(old, new)
        LIB.write_text(text)
        for kind, test, var in (("ee", "rv104_ee_diff", "RV104_EE_OUT"), ("run", "rv104_run_diff", "RV104_RUN_OUT")):
            env = dict(os.environ, CARGO_TARGET_DIR=str(WT / "targets/rv104-mut"), TMPDIR=str(S / "tmp"))
            env[var] = str(S / f"dumps/{kind}_mut_{name}.txt")
            with open(S / f"logs/survivor_{kind}_{name}.log", "w") as f:
                rc = subprocess.call([str(WT / "tools/t3_cargo.sh"), "test", "--offline", "--locked", "--test", test],
                                     cwd=P / "core/rules/rule_check_runner", env=env, stdout=f, stderr=subprocess.STDOUT)
            print(name, kind, "rc", rc, flush=True)
finally:
    LIB.write_text(ORIGINAL)
    for h in ("rv104_ee_diff.rs", "rv104_run_diff.rs"):
        (tests / h).unlink()
