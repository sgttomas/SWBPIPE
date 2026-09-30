#!/usr/bin/env python3
"""K4 checkpoint C, evidence pass (not the kill): a mutant's edits plus a
print-only insertion at the start of the controls test, which lists every
control whose outcome differs from GEN's and every selected control that is
not honest, before the test's own assertions run. Appends to evidence.jsonl."""
import json, os, re, shutil, subprocess, sys
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from mutants import MUTANTS, CTRL  # noqa: E402
from mut import REPO, COMMIT, FK, ENV  # noqa: E402
MT = "tests/retained_k4/method_tests.rs"
ANCHOR = "    let gen = gen_outcomes();\n    let runs = run_controls();\n"
INSERT = ANCHOR + """    {
        let mut exp: BTreeMap<String, BTreeMap<String, f64>> =
            all_models().into_iter().map(|m| (m.name, m.expect)).collect();
        for c in models::parse_combos(MODELS_5A3) {
            exp.insert(c.name, c.expect);
        }
        for (name, o) in &runs {
            let (sel, toks) = &gen[name];
            let got = o.selected.map_or("-".to_string(), |p| p.to_string());
            let moved = (&got, &o.tokens) != (sel, toks);
            let mut honest = String::new();
            if let (Some(s), Some(e)) = (&o.solve, exp.get(name)) {
                if !e.is_empty() {
                    let (w, at, _) = models::compare_honest(s.publish(), s.selected_precision(), e);
                    if w > 1.0 {
                        honest = format!(" DISHONEST {w:e} at {at}");
                    }
                }
            }
            if moved || !honest.is_empty() {
                println!("EVIDENCE {name}: {got} {:?} (GEN {sel} {:?}){honest}", o.tokens, toks);
            }
        }
    }
"""

def main(name):
    spec = MUTANTS[name]
    d = os.path.join(HERE, "ev-" + name)
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
    arch = subprocess.run(["git", "-C", REPO, "archive", COMMIT, FK], stdout=subprocess.PIPE, check=True).stdout
    subprocess.run(["tar", "-x", "-C", d], input=arch, check=True)
    root = os.path.join(d, FK)
    # The tightened claim check (ROOT's ruling at C): the working tree's models.rs.
    if os.environ.get("K4_TIGHT") == "1":
        for f in ("models.rs", "method_tests.rs", "recover_tests.rs"):
            shutil.copy(os.path.join(REPO, FK, "tests/retained_k4", f),
                        os.path.join(root, "tests/retained_k4", f))
    edits = list(spec["edits"]) + [(MT, ANCHOR, INSERT, 1)]
    for (path, old, new, count) in edits:
        p = os.path.join(root, path)
        s = open(p).read()
        assert s.count(old) == count, (name, path)
        open(p, "w").write(s.replace(old, new))
    env = dict(ENV, CARGO_TARGET_DIR=os.path.join(d, "target"), RUST_TEST_THREADS="1")
    log = os.path.join(d, "ctrl.log")
    with open(log, "w") as f:
        subprocess.run(["cargo", "test", "--offline", "--locked", "-j", "4", "--lib", "--",
                        "--exact", "structural::retained::" + CTRL, "--nocapture"],
                       cwd=root, env=env, stdout=f, stderr=subprocess.STDOUT)
    lines = [l[l.find("EVIDENCE ") + 9:].strip() for l in open(log, errors="replace") if "EVIDENCE " in l]
    out = "evidence_tight.jsonl" if os.environ.get("K4_TIGHT") == "1" else "evidence.jsonl"
    with open(os.path.join(HERE, out), "a") as f:
        f.write(json.dumps(dict(name=name, moved=lines)) + "\n")
    shutil.rmtree(os.path.join(d, "target"), ignore_errors=True)
    print(name, len(lines), flush=True)

if __name__ == "__main__":
    for n in sys.argv[1:]:
        main(n)
