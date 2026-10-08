#!/usr/bin/env python3
"""I85 B1-SP I3: which kills at 105e1a78c6 carry to the head 03f55e7178. The commit between them
changes only `b1_sp_sf2_selected_not_first_and_two_selected_pins` (checked here from the diff's
hunks: every changed line lies inside that function). A mutant killed at 105e1a78c6 by at least
one other test is therefore still killed at the head; a mutant killed only by the SF-2 test is
re-run at the head (RV109's six and O1 are all re-run there, for their killing assertions). Run in WT/b1 (read-only git: GIT_OPTIONAL_LOCKS=0).
Usage: carry_over.py <mutants.json at 105e1a78c6>"""
import json, os, re, subprocess, sys
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
F = "projects/chirality-piping/core/product_physics/src/retained_facade_tests.rs"
diff = subprocess.run(["git", "diff", "-U0", "105e1a78c6", "03f55e7178", "--", "."], capture_output=True, text=True, env=env).stdout
files = re.findall(r"^\+\+\+ b/(\S+)", diff, re.M)
head = subprocess.run(["git", "show", f"03f55e7178:{F}"], capture_output=True, text=True, env=env).stdout.split("\n")
start = next(i for i, l in enumerate(head) if l.startswith("fn b1_sp_sf2_selected_not_first_and_two_selected_pins("))
end = next(i for i in range(start + 1, len(head)) if head[i] == "}")
spans = [(int(a), int(b or 1)) for a, b in re.findall(r"^@@ -\S+ \+(\d+)(?:,(\d+))? @@", diff, re.M)]
inside = all(start + 1 <= a and a + max(n, 1) - 1 <= end + 1 for a, n in spans)
print(f"files changed: {files}; hunks: {len(spans)}; all inside the SF-2 test (lines {start + 1}-{end + 1}): {inside}")
SF2 = "b1_sp_sf2_selected_not_first_and_two_selected_pins"; T13 = "t13_committed_fallback_uz_is_byte_identical"
carry, rerun = [], []
for r in json.load(open(sys.argv[1])):
    others = [t for t in r["failed_tests"] if SF2 not in t and T13 not in t]
    (carry if r["killed"] and others else rerun).append(r["id"])
print(f"kill carries (another test fails too): {len(carry)}")
for m in carry: print(f"  {m}")
print(f"must be re-run at the head (killed only by the SF-2 test): {len(rerun)}")
for m in rerun: print(f"  {m}")
