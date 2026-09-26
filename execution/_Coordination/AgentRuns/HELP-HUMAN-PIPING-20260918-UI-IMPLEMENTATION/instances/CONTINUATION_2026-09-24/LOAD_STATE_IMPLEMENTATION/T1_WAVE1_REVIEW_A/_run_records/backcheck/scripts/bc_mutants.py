"""Backcheck mutants on the N-2 check (Python), on a git-archive copy of 1ccca8b87.
Usage: bc_mutants.py <COPY_WORKING_ROOT> <LOG_DIR>"""
import hashlib, os, pathlib, subprocess, sys
root = pathlib.Path(sys.argv[1]).resolve(); logs = pathlib.Path(sys.argv[2]); logs.mkdir(parents=True, exist_ok=True)
F = "core/analysis_runs/load_reference_evidence.py"
A = 'any(isinstance(r, str) and r in selected_ids for r in d["affected_refs"])'
M = [
    ("PY-N2-CHECK-REMOVED", '_require(not any(_eq(_get(d, "code"), UNAVAILABLE)', '_require(True or not any(_eq(_get(d, "code"), UNAVAILABLE)'),
    ("PY-N2-FIRST-REF-ONLY", A, 'any(isinstance(r, str) and r in selected_ids for r in d["affected_refs"][:1])'),
    ("PY-N2-ALL-REFS", A, 'all(isinstance(r, str) and r in selected_ids for r in d["affected_refs"])'),
    ("PY-N2-ANY-CASE-NOT-SELECTED", 'selected_ids = [case_id for case_id, record in zip(record_ids, records) if _is_selected(record)]', 'selected_ids = list(record_ids)'),
    ("PY-N2-APPLIED-TO-LOAD-REFERENCE-TOO", '        if joined:\n            _require(sum(1 for d in diagnostics if _eq(_get(d, "code"), SELECTED)) == selected, "JOIN_SELECTED_DIAGNOSTIC")\n',
     '        if joined:\n            _require(sum(1 for d in diagnostics if _eq(_get(d, "code"), SELECTED)) == selected, "JOIN_SELECTED_DIAGNOSTIC")\n        if True:\n            selected_ids = list(record_ids)\n            _require(not any(_eq(_get(d, "code"), UNAVAILABLE) for d in diagnostics), "JOIN_SELECTED_UNAVAILABLE_DIAGNOSTIC")\n        if joined:\n'),
]
env = dict(os.environ, PYTHONDONTWRITEBYTECODE="1")
out = []
for mid, anchor, repl in M:
    p = root / F; orig = p.read_bytes(); h = hashlib.sha256(orig).hexdigest(); t = orig.decode()
    n = t.count(anchor)
    if n != 1:
        out.append(f"{mid}: ANCHOR_COUNT={n}"); continue
    p.write_text(t.replace(anchor, repl))
    tests = ["tests/test_load_reference_source_readers.py"] + (["tests/test_load_reference_readers.py"] if "LOAD-REFERENCE" in mid else [])
    r = subprocess.run([sys.executable, "-m", "pytest", "-x", "-q", "-p", "no:cacheprovider", *tests], cwd=root, env=env, capture_output=True, text=True)
    p.write_bytes(orig); assert hashlib.sha256(p.read_bytes()).hexdigest() == h
    (logs / f"{mid}.log").write_text(f"exit={r.returncode}\n" + (r.stdout + r.stderr)[-4000:])
    first = next((l for l in (r.stdout).splitlines() if l.startswith("FAILED")), "")
    out.append(f"{mid}: {'KILLED' if r.returncode else 'SURVIVED'} {first[:200]}")
    print(out[-1], flush=True)
(logs / "summary.txt").write_text("\n".join(out) + "\n")
