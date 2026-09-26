"""Join Rust and Python probe outcomes by id and flag every difference."""
import json, sys
def load(p):
    return {json.loads(l)["id"]: json.loads(l) for l in open(p) if l.strip()}
rows = []
for py_path, rs_path in zip(sys.argv[1::2], sys.argv[2::2]):
    py, rs = load(py_path), load(rs_path)
    for k in sorted(set(py) | set(rs)):
        a, b = py.get(k, {}), rs.get(k, {})
        for col in ("dispatch", "validator", "transport", "standing"):
            if col in a or col in b:
                same = a.get(col) == b.get(col)
                rows.append(f"{'SAME' if same else 'DIFF'} | {k} | {col} | py={a.get(col)} | rs={b.get(col)}")
print("\n".join(rows))
print("differences:", sum(r.startswith("DIFF") for r in rows), "of", len(rows))
