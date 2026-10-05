#!/usr/bin/env python3
"""RV95: rerun the walker probe on synth2 (the string literal on its own line), at F' and under W3."""
import hashlib, json, os, subprocess, sys
T3 = "WT"
S = T3 + "/scratch/rv95_u9_01"; C2 = S + "/conf2"; P = T3 + "/rv95/projects/chirality-piping"
NI = P + "/core/solver/nonlinear_integration"; TG = T3 + "/targets/rv95/ni"; S11K = NI + "/src/s11k_tests.rs"
env = dict(os.environ, TMPDIR=S + "/tmp", CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2", RV95_TREE=C2 + "/synth2/src"); env.pop("RUSTFLAGS", None)
def run(label, cmd, cwd):
    if subprocess.run(["ps", "-p", "5387"], capture_output=True).returncode: sys.exit("MEMGUARD NOT RUNNING")
    r = subprocess.run(cmd, cwd=cwd, env=env, capture_output=True, text=True); open(f"{C2}/logs/{label}.log", "w").write(r.stdout + r.stderr); return r
res = {}
for tag, extra in (("plain", []), ("cfg_test", ["--cfg", "test"])):
    res["rustc2_" + tag] = run(f"rustc_synth2_{tag}", ["rustc", "--edition", "2021", "--crate-type", "lib", "--emit=metadata", *extra, "-o", f"{S}/tmp/synth2_{tag}.rmeta", C2 + "/synth2/src/lib.rs"], C2).returncode
base = open(S11K).read(); h0 = hashlib.sha256(base.encode()).hexdigest()
probe = open(C2 + "/zz_rv95_walker_probe.rs").read()
W3_OLD = "            let base = if inline.is_empty() {\n                file.parent().unwrap().to_path_buf()\n            } else {\n                inline.iter().fold(child_dir(file), |d, m| d.join(m))\n            };\n"
W3_NEW = "            let base = file.parent().unwrap().to_path_buf();\n"
try:
    for label, src in (("probe2_Fp", base + probe), ("probe2_W3", base.replace(W3_OLD, W3_NEW, 1) + probe)):
        assert label != "probe2_W3" or base.count(W3_OLD) == 1
        open(S11K, "w").write(src)
        r = run(label, ["cargo", "test", "--locked", "--offline", "--target-dir", TG, "--lib", "zz_rv95_walker_probe", "--", "--ignored", "--nocapture"], NI)
        res[label] = [l for l in (r.stdout + r.stderr).splitlines() if "RV95_WALKER_NON_TEST" in l or "panicked" in l or "No such file" in l][:3]
finally:
    open(S11K, "w").write(base)
assert hashlib.sha256(open(S11K).read().encode()).hexdigest() == h0
res["restored"] = True
json.dump(res, open(C2 + "/probe2.json", "w"), indent=1); print(json.dumps(res, indent=1))
