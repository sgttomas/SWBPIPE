#!/usr/bin/env python3
"""Temporary one-site mutation harness.
usage: mutate.py <log> <target-rel-to-WORKING_ROOT> <old-file> <new-file> <expect: fail|pass> -- <test files...>
Applies exactly one replacement (old must occur once), runs the focused vitest
files, restores the original bytes by copy and verifies sha256 against both the
backup and the HEAD blob (for tracked product files) or the pre-mutation bytes."""
import hashlib, os, shutil, subprocess, sys, datetime
WR = "/home/user/wt/engine/projects/chirality-piping"
log, target, oldf, newf, expect = sys.argv[1:6]
tests = sys.argv[7:]
old = open(oldf).read(); new = open(newf).read()
path = os.path.join(WR, target)
sha = lambda b: hashlib.sha256(b).hexdigest()
orig = open(path, "rb").read()
backup = path + ".mutation-backup"
shutil.copyfile(path, backup)
head = subprocess.run(["git", "-C", WR, "show", f"HEAD:projects/chirality-piping/{target}"], capture_output=True)
head_sha = sha(head.stdout) if head.returncode == 0 else None
text = orig.decode()
assert text.count(old) == 1, f"old snippet occurs {text.count(old)} times"
mutated = text.replace(old, new).encode()
out = []
out.append(f"date: {datetime.datetime.utcnow().isoformat()}Z")
out.append(f"target: {target}")
out.append(f"sha256 before: {sha(orig)}")
out.append(f"sha256 HEAD blob: {head_sha}")
out.append("--- replaced (exactly once) ---\n" + old + "\n--- with ---\n" + new + "\n---")
open(path, "wb").write(mutated)
out.append(f"sha256 mutated: {sha(mutated)}")
env = dict(os.environ, PATH="/opt/node24/bin:" + os.environ["PATH"])
cmd = ["npm", "test", "--workspace", "apps/desktop", "--", "--run", *tests]
out.append("command (from WORKING_ROOT): " + " ".join(cmd))
try:
    r = subprocess.run(cmd, cwd=WR, capture_output=True, text=True, env=env, timeout=900)
    body = r.stdout + r.stderr
    rc = r.returncode
finally:
    shutil.copyfile(backup, path)
    os.remove(backup)
restored = open(path, "rb").read()
keep = [l for l in body.splitlines() if "getContext" not in l and not l.lstrip().startswith("at ") and not l.lstrip().startswith("\x1b[36m") and not l.lstrip().startswith("\x1b[33m") and not l.lstrip().startswith("\x1b[0m")]
out.append("--- vitest output (jsdom canvas noise and stack/DOM dump lines elided) ---")
out.extend(keep)
out.append(f"exit={rc}")
out.append(f"sha256 restored: {sha(restored)}")
ok_restore = sha(restored) == sha(orig) and (head_sha is None or head_sha == sha(orig))
diff = subprocess.run(["git", "-C", WR, "diff", "--stat", "--", target], capture_output=True, text=True).stdout.strip()
out.append(f"git diff --stat after restore: {diff or '(empty)'}")
out.append(f"restored == before: {sha(restored) == sha(orig)}; before == HEAD blob: {head_sha == sha(orig) if head_sha else 'n/a (untracked)'}")
verdict = ("FAIL-as-expected" if rc != 0 else "UNEXPECTED-PASS") if expect == "fail" else ("PASS-as-expected" if rc == 0 else "UNEXPECTED-FAIL")
out.append(f"verdict: {verdict}")
open(log, "w").write("\n".join(out) + "\n")
print(f"{verdict} rc={rc} restore_ok={ok_restore} diff={diff or 'empty'}")
sys.exit(0 if ok_restore else 3)
