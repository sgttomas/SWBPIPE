#!/usr/bin/env python3
"""K4 after RV19's review, evidence pass (not the kill): a mutant's edits
(this campaign's, `rv19_mut.py`, or K4's C-campaign definitions,
`../k4-mut/mutants.py`) on a clean copy of the
candidate (`rv19_mut.py`'s copy), plus a print-only insertion at the start of
the controls test, which lists every control whose outcome differs from GEN's
and every selected control whose claims `compare_honest` rejects (with GEN's
128-bit expectations and nothing skipped), before the test's own assertions
run. Appends to evidence.jsonl here."""
import json, os, shutil, subprocess, sys
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "k4-mut"))
import rv19_mut  # noqa: E402
from mutants import MUTANTS as C_MUTANTS  # noqa: E402

CTRL = "adaptive::method_tests::every_control_follows_gens_schedule_and_r7s_expectations_honestly"
MT = "tests/retained_k4/method_tests.rs"
ANCHOR = "    let gen = gen_outcomes();\n    let runs = run_controls();\n"
INSERT = ANCHOR + """    {
        let mut exp: BTreeMap<String, BTreeMap<String, models::Exact>> =
            all_models().into_iter().map(|m| (m.name, m.exact)).collect();
        for c in models::parse_combos(MODELS_5A3) {
            exp.insert(c.name, c.exact);
        }
        for (name, o) in &runs {
            let (sel, toks) = &gen[name];
            let got = o.selected.map_or("-".to_string(), |p| p.to_string());
            let moved = (&got, &o.tokens) != (sel, toks);
            let mut honest = String::new();
            if let Some(s) = &o.solve {
                let e = exp.get(name).cloned().unwrap_or_default();
                let (w, at, _) = models::compare_honest(s.publish(), s.selected_precision(), &e);
                if w > 1.0 {
                    honest = format!(" DISHONEST {w:e} at {at}");
                }
            }
            if moved || !honest.is_empty() {
                println!("EVIDENCE {name}: {got} {:?} (GEN {sel} {:?}){honest}", o.tokens, toks);
            }
        }
    }
"""


def main(name):
    spec = rv19_mut.MUTANTS.get(name) or C_MUTANTS[name]
    edits = list(spec["edits"])
    d = os.path.join(HERE, "ev-" + name)
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
    arch = subprocess.run(["git", "-C", rv19_mut.REPO, "archive", rv19_mut.BASE, rv19_mut.FK],
                          stdout=subprocess.PIPE, check=True).stdout
    subprocess.run(["tar", "-x", "-C", d], input=arch, check=True)
    for rel in rv19_mut.candidate_files():
        src = os.path.join(rv19_mut.REPO, rel)
        dst = os.path.join(d, rel)
        if os.path.exists(src):
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copyfile(src, dst)
    root = os.path.join(d, rv19_mut.FK)
    for (path, old, new, count) in edits + [(MT, ANCHOR, INSERT, 1)]:
        p = os.path.join(root, path)
        s = open(p).read()
        assert s.count(old) == count, (name, path)
        open(p, "w").write(s.replace(old, new))
    env = dict(rv19_mut.ENV, CARGO_TARGET_DIR=os.path.join(d, "target"), RUST_TEST_THREADS="1")
    log = os.path.join(d, "ctrl.log")
    with open(log, "w") as f:
        subprocess.run(["cargo", "test", "--offline", "--locked", "-j", "4", "--lib", "--",
                        "--exact", "structural::retained::" + CTRL, "--nocapture"],
                       cwd=root, env=env, stdout=f, stderr=subprocess.STDOUT)
    lines = [l[l.find("EVIDENCE ") + 9:].strip() for l in open(log, errors="replace") if "EVIDENCE " in l]
    with open(os.path.join(HERE, "evidence.jsonl"), "a") as f:
        f.write(json.dumps(dict(name=name, moved=lines)) + "\n")
    shutil.rmtree(os.path.join(d, "target"), ignore_errors=True)
    print(name, len(lines), flush=True)


if __name__ == "__main__":
    for n in sys.argv[1:]:
        main(n)
