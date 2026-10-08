#!/usr/bin/env python3
"""I85 B1-SP I3: the mutant batch's table from mutants_sp.py's mutants.json. For each mutant:
compiled, killed, the failing tests (t13, which fails at the base too, is not a kill), and for
RV109's six (RV2_*) and the ordinal mutant (O1_*) the panic of the SF-2 or ordinal test, which
names the assertion that killed it (file:line and its message).
Usage: mutant_table.py <mutants.json>"""
import json, sys
rows = json.load(open(sys.argv[1]))
T13 = "s11g_tests::t13_committed_fallback_uz_is_byte_identical"
FOCUS = {"RV2_": ["b1_sp_sf2_selected_not_first_and_two_selected_pins"], "O1_": ["b1_sp_constructor_ordinal_is_the_authored_index"]}
killed = sum(1 for r in rows if r["killed"])
print(f"{len(rows)} mutants; compiled {sum(1 for r in rows if r['compiled'])}; killed {killed}; survived {len(rows) - killed}")
for r in rows:
    tests = [t.split("::")[-1] for t in r["failed_tests"] if T13 not in t]
    print(f"{r['id']}: compiled={r['compiled']} killed={r['killed']} failing={len(tests)} ({r['seconds']} s)")
    for t in tests: print(f"    {t}")
    for prefix, names in FOCUS.items():
        if r["id"].startswith(prefix):
            for p in r["panics"]:
                if any(n in p["test"] for n in names):
                    print(f"    -> {p['test'].split('::')[-1]} panicked at {p['at']}: {p['message']}")
