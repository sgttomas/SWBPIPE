#!/usr/bin/env python3
"""RV95 addendum 02 runs (only after ROOT's all-clear): rustc on the synthetic tree, the walker probe,
nonlinear_integration's full suite on F', and the walker/pin mutants. One cargo job at a time."""
import hashlib, json, os, subprocess, sys
T3 = "WT"
S = T3 + "/scratch/rv95_u9_01"; C2 = S + "/conf2"; P = T3 + "/rv95/projects/chirality-piping"
NI = P + "/core/solver/nonlinear_integration"; TG = T3 + "/targets/rv95/ni"
S11K = NI + "/src/s11k_tests.rs"; PPLIB = P + "/core/product_physics/src/lib.rs"
env = dict(os.environ, TMPDIR=S + "/tmp", CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2"); env.pop("RUSTFLAGS", None)
os.makedirs(S + "/tmp", exist_ok=True); os.makedirs(C2 + "/logs", exist_ok=True)
def guard():
    if subprocess.run(["ps", "-p", "5387"], capture_output=True).returncode: sys.exit("MEMGUARD NOT RUNNING")
def run(label, cmd, cwd, extra=None):
    guard(); e = dict(env, **(extra or {}))
    r = subprocess.run(["perl", "-e", "alarm shift; exec @ARGV", "1800", *cmd], cwd=cwd, env=e, capture_output=True, text=True)
    open(f"{C2}/logs/{label}.log", "w").write(r.stdout + r.stderr); return r
def cargo(label, *args, extra=None):
    return run(label, ["cargo", "test", "--locked", "--offline", "--target-dir", TG, *args], NI, extra)
def outcomes(text):
    return sorted(l.strip() for l in text.splitlines() if l.startswith("test ") and l.rstrip().endswith(("ok", "FAILED", "ignored")))
res = {}
# 1. rustc resolves the synthetic tree, without and with cfg(test)
for tag, extra in (("plain", []), ("cfg_test", ["--cfg", "test"])):
    r = run(f"rustc_synth_{tag}", ["rustc", "--edition", "2021", "--crate-type", "lib", "--emit=metadata", *extra,
             "-o", f"{S}/tmp/synth_{tag}.rmeta", C2 + "/synth/src/lib.rs"], C2)
    res[f"rustc_{tag}"] = r.returncode
# 2. full suite on F' (pristine copy)
h0 = {f: hashlib.sha256(open(f, "rb").read()).hexdigest() for f in (S11K, PPLIB)}
r = cargo("ni_full_Fp", "--no-fail-fast"); res["ni_full_rc"] = r.returncode
o = outcomes(r.stdout + r.stderr); res["ni_full"] = {"ok": sum(x.endswith(" ok") for x in o), "failed": sum(x.endswith("FAILED") for x in o), "ignored": sum(x.endswith("ignored") for x in o)}
open(f"{C2}/logs/ni_full_Fp.outcomes", "w").write("\n".join(o) + "\n")
# 3. the walker probe on the synthetic tree (appended scratch test), then mutants
def mutate(path, old, new):
    src = open(path).read(); assert src.count(old) == 1, (path, old[:60], src.count(old)); open(path, "w").write(src.replace(old, new, 1)); return src
probe = open(C2 + "/zz_rv95_walker_probe.rs").read()
base_s11k = open(S11K).read(); open(S11K, "w").write(base_s11k + probe)
try:
    r = cargo("probe_Fp", "--lib", "zz_rv95_walker_probe", "--", "--ignored", "--nocapture", extra={"RV95_TREE": C2 + "/synth/src"})
    res["probe_Fp"] = [l for l in (r.stdout + r.stderr).splitlines() if "RV95_WALKER_NON_TEST" in l or "panicked" in l]
    MUT = [
     ("W1", "walker: enclosing inline modules ignored for plain declarations", S11K,
      "        let dir = inline.iter().fold(child_dir(file), |d, m| d.join(m));\n", "        let dir = child_dir(file);\n"),
     ("W2", "K2b site pin reverted to the wrapper `fn solve_load_case`", S11K,
      '    ("lib.rs", "fn solve_load_case_observed", "ForceScale", 1),\n', '    ("lib.rs", "fn solve_load_case", "ForceScale", 1),\n'),
     ("W3", "walker: #[path] inside inline modules resolved from the file's directory", S11K,
      "            let base = if inline.is_empty() {\n                file.parent().unwrap().to_path_buf()\n            } else {\n                inline.iter().fold(child_dir(file), |d, m| d.join(m))\n            };\n",
      "            let base = file.parent().unwrap().to_path_buf();\n"),
     ("W4", "PP: the wrapper `solve_load_case` names ForceScale (text only; the test reads PP's source)", PPLIB,
      "    solve_load_case_observed(model, built, materials, stiffness, restrained_dofs, spring_entries,\n",
      "    let _ = ForceScale::UNSCALED; solve_load_case_observed(model, built, materials, stiffness, restrained_dofs, spring_entries,\n"),
     ("W5", "walker: test-ness from the file path instead of the declaration (grant2 treated as non-test)", S11K,
      "            let child_test = test || !non_test.contains(&child);\n", "            let child_test = test && !non_test.contains(&child);\n"),
    ]
    res["mutants"] = []
    for mid, what, path, old, new in MUT:
        src = mutate(path, old, new)
        try:
            r = cargo(f"mut_{mid}", "--lib", "s11k_tests", "--", "--test-threads=2")
            o = outcomes(r.stdout + r.stderr)
            failed = [x for x in o if x.endswith("FAILED")]
            pr = None
            if mid == "W3":
                pr = cargo("mut_W3_probe", "--lib", "zz_rv95_walker_probe", "--", "--ignored", "--nocapture", extra={"RV95_TREE": C2 + "/synth/src"})
                pr = [l for l in (pr.stdout + pr.stderr).splitlines() if "RV95_WALKER_NON_TEST" in l or "panicked" in l][:2]
            compile_err = "could not compile" in (r.stdout + r.stderr)
            res["mutants"].append({"id": mid, "what": what, "status": "COMPILE_ERROR" if compile_err else ("KILLED" if failed else "SURVIVED"),
                                   "failed": failed, "probe": pr})
        finally:
            open(path, "w").write(src)
        print(json.dumps(res["mutants"][-1])[:400], flush=True)
finally:
    open(S11K, "w").write(base_s11k)
for f, h in h0.items(): assert hashlib.sha256(open(f, "rb").read()).hexdigest() == h, f
res["restored"] = True
json.dump(res, open(C2 + "/after_allclear.json", "w"), indent=1)
print(json.dumps({k: v for k, v in res.items() if k != "mutants"}, indent=1))
