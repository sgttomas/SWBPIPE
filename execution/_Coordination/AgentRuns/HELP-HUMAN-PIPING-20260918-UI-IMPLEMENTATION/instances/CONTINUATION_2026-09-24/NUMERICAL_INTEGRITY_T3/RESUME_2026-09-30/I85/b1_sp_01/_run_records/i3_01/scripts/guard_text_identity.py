#!/usr/bin/env python3
"""I85 B1-SP I3: the guards' text at the base (2ba2f81863) and the head. The two outside-src guard
files are compared whole; each in-src guard test is compared from its `fn` line to the next
top-level `fn`/attribute line. Run in WT/b1 (read-only git: GIT_OPTIONAL_LOCKS=0).
Usage: guard_text_identity.py <base> <head>"""
import hashlib, os, re, subprocess, sys
base, head = sys.argv[1:3]
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
PP = "projects/chirality-piping/core/product_physics"
def show(rev, f): return subprocess.run(["git", "show", f"{rev}:{f}"], capture_output=True, text=True, env=env, check=True).stdout
def body(text, name):
    m = re.search(rf"^fn {name}\(", text, re.M)
    assert m, name
    rest = text[m.start():]
    end = re.search(r"\n(?=fn |#\[|/// |// ----)", rest[1:])
    return rest[: end.start() + 1] if end else rest
ok = True
for f in [f"{PP}/tests/s11f_site_test.rs", f"{PP}/tests/retained_precision_admission.rs"]:
    a, b = show(base, f), show(head, f)
    same = a == b; ok &= same
    print(f"{os.path.basename(f)}: whole file identical={same} sha256={hashlib.sha256(b.encode()).hexdigest()}")
for f, names in [(f"{PP}/src/retained_facade_tests.rs", ["u3_n9_single_parse_custody", "u3_permitted_outputs_keep_the_report_and_gate_order",
                  "u3_capture_permit_is_linear", "u3g2_no_permit_path_runs_once_without_a_copy"]),
                 (f"{PP}/src/retained_wire_tests.rs", ["u1_serializer_reads_no_legacy_work_field"])]:
    a, b = show(base, f), show(head, f)
    for n in names:
        try:
            x, y = body(a, n), body(b, n)
        except AssertionError:
            print(f"{n}: not found in {os.path.basename(f)}"); ok = False; continue
        same = x == y; ok &= same
        print(f"{os.path.basename(f)}::{n}: identical={same} ({len(y.splitlines())} lines) sha256={hashlib.sha256(y.encode()).hexdigest()}")
print("ALL GUARD TEXT IDENTICAL" if ok else "DIFFERENCE")
sys.exit(0 if ok else 1)
