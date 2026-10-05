"""RV89 part 2: its own mutants of the profile forms, the phase composition and the gate caps.
One exact replacement at a time in a scratch copy of the part-2 core tree; `cargo test --lib retained_memory`
(one cargo job). KILLED / SURVIVED / COMPILE as in RV89 part 1. Every mutated file is restored first."""
import json, os, shutil, subprocess, sys
CAND, COPY, TARGET = sys.argv[1:4]
RM = "product_physics/src/retained_memory.rs"
SR = "loads/stress_recovery/src/elastic_extrema.rs"
W3S = '            (add(add(add(add(add(add(add(add(form(&FORMS[6], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), form(&FORMS[7], v)), t16(v)), max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[22], v))), // W3'
W3S_NEW = '            (add(add(add(add(add(add(add(form(&FORMS[6], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), t16(v)), max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[22], v))), // W3'
W4D = '            (add(add(add(add(add(add(add(add(add(form(&FORMS[5], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), form(&FORMS[9], v)), form(&FORMS[2], v)), t17(v)), max(max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[31], v)), form(&FORMS[30], v))), // W4'
W4D_NEW = '            (add(add(add(add(add(add(add(add(form(&FORMS[5], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), form(&FORMS[9], v)), t17(v)), max(max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[31], v)), form(&FORMS[30], v))), // W4'
MUTANTS = [
    ("Q01 W3 (sparse) drops the staged copy", RM, W3S, W3S_NEW),
    ("Q02 W4 (dense) drops the invocation Value", RM, W4D, W4D_NEW),
    ("Q03 T25 drops the carried case", RM, "add(form(&FORMS[42], v), max(max(max(max(max(max(max(form(&FORMS[37], v)", "add(Some(0), max(max(max(max(max(max(max(form(&FORMS[37], v)"),
    ("Q04 s(Value) bound to String's size", RM, "        (size_of::<Value>()) as u64, // s(Value)", "        (size_of::<String>()) as u64, // s(Value)"),
    ("Q05 ESTIMATES counts SourceUpper", RM, "if matches!(ATOM_BINDINGS[i], Binding::Estimate) {", "if matches!(ATOM_BINDINGS[i], Binding::SourceUpper) {"),
    ("Q06 Estimate gate exempts today's 42", RM, "    if estimates != 0 {", "    if estimates != 0 && estimates != 42 {"),
    ("Q07 node law: 11 edges", RM, "let internal = up(up(leaf, 8) + 12 * 8, a);", "let internal = up(up(leaf, 8) + 11 * 8, a);"),
    ("Q08 Shared<4> drops the (4,8) instantiation", RM, "(max_usize(fkr::SHARED_4_4, fkr::SHARED_4_8)) as u64", "(fkr::SHARED_4_4) as u64"),
    ("Q09 SR stride of the wrong type", SR, "pub const NODE_STRIDE: usize = std::mem::size_of::<Node>();", "pub const NODE_STRIDE: usize = std::mem::size_of::<Controls>();"),
    ("Q10 maximum skips the X phases", RM, "        while i < PHASES {", "        while i < 5 {"),
    ("Q11 G-C result-capacity cap is the length cap", RM, "            push_capacity(P_FINAL),", "            P_FINAL,"),
    ("Q12 G-B late cap is all of T11", RM, "profile_bytes(profile::F_T11).saturating_sub(profile_bytes(profile::F_T11_LATE_CAPTURE))],", "profile_bytes(profile::F_T11)],"),
]
def run():
    env = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
    p = subprocess.run(["cargo", "test", "--locked", "--offline", "--lib", "--target-dir", TARGET, "retained_memory"],
                       cwd=os.path.join(COPY, "product_physics"), env=env, capture_output=True, text=True, timeout=2400)
    out = p.stdout + p.stderr
    if "could not compile" in out or "error[E" in out:
        return "COMPILE", out[-600:]
    return ("KILLED" if p.returncode != 0 else "SURVIVED"), [l for l in out.splitlines() if l.startswith("test ") and "FAILED" in l][:4]
for name, rel, old, new in MUTANTS:
    if subprocess.run(["pgrep", "-f", "memguard.sh"], capture_output=True).returncode != 0:
        print(json.dumps({"mutant": name, "result": "STOPPED: memory guard not running"})); sys.exit(9)
    for f in (RM, SR):
        shutil.copyfile(os.path.join(CAND, f), os.path.join(COPY, f))
    path = os.path.join(COPY, rel)
    text = open(path).read()
    if text.count(old) != 1:
        print(json.dumps({"mutant": name, "result": "NOT APPLIED", "count": text.count(old)}), flush=True); continue
    open(path, "w").write(text.replace(old, new))
    result, detail = run()
    print(json.dumps({"mutant": name, "file": rel, "result": result, "detail": detail}), flush=True)
for f in (RM, SR):
    shutil.copyfile(os.path.join(CAND, f), os.path.join(COPY, f))
