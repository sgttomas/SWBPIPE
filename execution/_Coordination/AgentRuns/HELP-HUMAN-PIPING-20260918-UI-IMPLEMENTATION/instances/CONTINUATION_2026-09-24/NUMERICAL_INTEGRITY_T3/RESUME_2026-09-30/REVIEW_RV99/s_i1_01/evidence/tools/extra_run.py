"""Run rv99_extra.json through RV99's harness+checker on the candidate and on
named mutants (patch text from mut_rv99.py)."""
import hashlib, os, pathlib, re, subprocess, sys
S = pathlib.Path(__file__).parent
C = S.parents[1] / "rv99/projects/chirality-piping"
VENV = "<VENV>"
src = (S / "mut_rv99.py").read_text()
ns = {}
exec(src.split("\nALL = {}")[0].replace("rr = pathlib.Path(sys.argv[1])", "rr = pathlib.Path(%r)" % sys.argv[1]), ns)
MUT = dict(ns["I73_EE"]); MUT.update(ns["I73_RP"]); MUT.update(ns["RV99"])
cases = sys.argv[2]
for name in ["candidate"] + sys.argv[3:]:
    path = original = before = None
    if name != "candidate":
        rel, patches = MUT[name]
        path = C / rel; original = path.read_text(); before = hashlib.sha256(original.encode()).hexdigest()
        text = original
        for old, new in patches:
            assert text.count(old) == 1; text = text.replace(old, new)
        path.write_text(text)
    try:
        out = S / f"extra_{name}.json"
        env = dict(os.environ, RV99_CASES=str(S / cases), RV99_OUT=str(out))
        rc = subprocess.run([str(S / "rc.sh"), str(C / "core/rules/rule_check_runner"), "cand", f"extra_{name}",
                             "test", "--locked", "--offline", "--release", "--test", "rv99_harness"], env=env).returncode
        chk = subprocess.run([f"{VENV}/bin/python", "-B", str(S / "rv99_check.py"), str(S / cases), str(out),
                              str(C / "core/analysis_runs")], capture_output=True, text=True)
        (S / f"logs/check_extra_{name}.txt").write_text(chk.stdout + chk.stderr)
        m = re.search(r"VIOLATIONS: (\{.*\})", chk.stdout)
        print(name, "harness_rc=", rc, "violations=", m.group(1) if m else chk.stderr[-400:])
    finally:
        if path is not None:
            path.write_text(original)
            assert hashlib.sha256(path.read_text().encode()).hexdigest() == before
