"""RV97: per mutant, the failing tests and each failing test's assertion message (from the panic
lines in the log). Usage: mutant_summary.py MUTANT_DIR"""
import json, re, sys
from pathlib import Path
d = Path(sys.argv[1])
res = json.loads((d / "mutants.json").read_text())
for m in res:
    log = (d / f"mutant_{m['id']}.log").read_text(errors="replace")
    msgs = {}
    for name, msg in re.findall(r"thread '([^']+)' \(\d+\) panicked at [^\n]*\n([^\n]*)", log):
        msgs.setdefault(name, msg.strip()[:220])
    short = lambda t: t.replace("retained_facade_tests::", "")
    print(f"{m['id']}: {m['what']}")
    print(f"   compile_error={m['compile_error']} killed={m['killed']} failed={len(m['failed'])} only_u8={m['only_u8']}")
    for t in m["failed"]:
        print(f"   - {short(t)}: {msgs.get(t, '(no panic line found)')}")
