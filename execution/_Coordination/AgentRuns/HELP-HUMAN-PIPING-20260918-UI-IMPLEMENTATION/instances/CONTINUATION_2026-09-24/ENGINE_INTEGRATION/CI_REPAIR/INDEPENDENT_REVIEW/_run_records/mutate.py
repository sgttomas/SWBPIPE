#!/usr/bin/env python3
"""Independent-review mutation runner. For one mutation: back up the target file, apply exactly one
replacement (or replace the whole file with a given Git blob), run one Playwright selection from
apps/desktop, restore the original bytes, and verify they equal the HEAD blob. Exits non-zero if
the restore check fails. Usage:
  mutate.py <name> <repo-relative file> <old|@blob:REV> <new> <playwright args...>
"""
import hashlib, os, subprocess, sys, datetime

name, rel, old, new, *pw = sys.argv[1:]
repo = subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip()
path = os.path.join(repo, rel)
desk = os.path.join(repo, "projects/chirality-piping/apps/desktop")
out = os.path.join(os.path.dirname(os.path.abspath(__file__)), f"mut-{name}.log")
orig = open(path, "rb").read()
head_blob = subprocess.check_output(["git", "rev-parse", f"HEAD:{rel}"], cwd=repo, text=True).strip()
def blob_id(data):
    return hashlib.sha1(b"blob %d\0" % len(data) + data).hexdigest()
assert blob_id(orig) == head_blob, "working file is not the HEAD blob before mutation"
if old.startswith("@blob:"):
    mutated = subprocess.check_output(["git", "show", f"{old[6:]}:{rel}"], cwd=repo)
    desc = f"whole file replaced by {old[6:]}:{rel}"
else:
    text = orig.decode()
    assert text.count(old) == 1, f"expected exactly one occurrence, found {text.count(old)}"
    mutated = text.replace(old, new).encode()
    desc = f"replace {old!r} -> {new!r}"
with open(out, "w") as log:
    log.write(f"mutation {name}: {rel}: {desc}\n")
    log.write(f"head {subprocess.check_output(['git','rev-parse','HEAD'], cwd=repo, text=True).strip()}; original blob {head_blob}\n")
    log.write(f"start {datetime.datetime.utcnow().isoformat()}Z; load {os.getloadavg()}\n")
    log.write("cmd: ../../node_modules/.bin/playwright test " + " ".join(pw) + "\n\n")
    log.flush()
    open(path, "wb").write(mutated)
    try:
        rc = subprocess.run(["../../node_modules/.bin/playwright", "test", *pw], cwd=desk, stdout=log, stderr=subprocess.STDOUT).returncode
    finally:
        open(path, "wb").write(orig)
    restored = open(path, "rb").read()
    ok = blob_id(restored) == head_blob
    log.write(f"\nplaywright_rc={rc} (non-zero means the mutation was caught)\n")
    log.write(f"restored_blob={blob_id(restored)} equals_HEAD={ok}\n")
    log.write(f"end {datetime.datetime.utcnow().isoformat()}Z\n")
print(name, "rc", rc, "restored", ok)
sys.exit(0 if ok else 3)
