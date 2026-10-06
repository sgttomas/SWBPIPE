"""RV98: compare delta inventories row by row (file, lines, class, fingerprint, reason, reviewed)."""
import json, sys
def rows(p):
    d = json.load(open(p)); return d, [json.dumps(r, sort_keys=True) for r in d["rows"]]
a_lab, a, b_lab, b = sys.argv[1:5]
da, ra = rows(a); db, rb = rows(b)
print(f"{a_lab}: files {da['files']}, rows {da['hunks_and_files']}, classes {da['classes']}")
print(f"{b_lab}: files {db['files']}, rows {db['hunks_and_files']}, classes {db['classes']}")
from collections import Counter
ca, cb = Counter(ra), Counter(rb)
added = list((cb - ca).elements()); removed = list((ca - cb).elements())
print(f"rows only in {b_lab}: {len(added)}; rows only in {a_lab}: {len(removed)}")
for r in added: print("  +", r)
for r in removed: print("  -", r)
