#!/usr/bin/env python3
"""K4 checkpoint C: one mutant from a clean `git archive` copy of the candidate,
with its own target (deleted afterwards). Standard library only.

  python3 mut.py NAME            run one mutant (NONE: no edit)
  python3 mut.py --list          list the mutants

Each mutant is a list of exact source edits (each `old` must occur exactly as
many times as stated) and its intended killing tests, run one by one. When no
intended test fails, the rest of K4's fast tests are run; a mutant that no test
kills is a SURVIVOR. Results append to results.jsonl."""
import json, os, re, shutil, subprocess, sys, time
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.join(os.path.dirname(HERE), "k4")
COMMIT = "8f8023a20"
FK = "projects/chirality-piping/core/solver/frame_kernel"
ENV = dict(os.environ, RUSTUP_TOOLCHAIN="1.97.1", RUSTUP_AUTO_INSTALL="0", CARGO_INCREMENTAL="0")
sys.path.insert(0, HERE)
from mutants import MUTANTS, ALL_TESTS, SLOW  # noqa: E402

def run(cmd, cwd, env, log):
    with open(log, "w") as f:
        return subprocess.run(cmd, cwd=cwd, env=env, stdout=f, stderr=subprocess.STDOUT).returncode

def main(name):
    spec = MUTANTS[name]
    d = os.path.join(HERE, name)
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
    arch = subprocess.run(["git", "-C", REPO, "archive", COMMIT, spec.get("archive", FK)],
                          stdout=subprocess.PIPE, check=True).stdout
    subprocess.run(["tar", "-x", "-C", d], input=arch, check=True)
    root = os.path.join(d, FK)
    diffs = []
    for (path, old, new, count) in spec["edits"]:
        p = os.path.join(root, path)
        s = open(p).read()
        n = s.count(old)
        assert n == count, "%s: %s: expected %d occurrences, found %d" % (name, path, count, n)
        t = s.replace(old, new)
        open(p, "w").write(t)
        diffs.append(path)
    target = os.path.join(d, "target")
    env = dict(ENV, CARGO_TARGET_DIR=target)
    t0 = time.time()
    rc = run(["cargo", "test", "--offline", "--locked", "-j", "4", "--lib", "--no-run"], root, env, os.path.join(d, "build.log"))
    build_s = time.time() - t0
    rec = dict(name=name, build_rc=rc, build_s=round(build_s), killed_by=[], ran=[], survivor=False)
    if rc != 0:
        rec["result"] = "COMPILE-FAIL"
    else:
        deps = os.path.join(target, "debug", "deps")
        bins = [os.path.join(deps, f) for f in os.listdir(deps)
                if f.startswith("open_pipe_stress_frame_kernel-") and "." not in f and os.access(os.path.join(deps, f), os.X_OK)]
        assert len(bins) == 1, bins
        binary = bins[0]
        def test(t):
            log = os.path.join(d, "test_%s.log" % t.split("::")[-1][:60])
            t1 = time.time()
            r = run([binary, "--exact", "structural::retained::" + t, "--test-threads=1"], root,
                    dict(env, RUST_TEST_THREADS="1"), log)
            txt = open(log, errors="replace").read()
            ok = ("1 passed" in txt) and r == 0
            assert "1 passed" in txt or "1 failed" in txt, "%s: test %s did not run" % (name, t)
            why = ""
            if not ok:
                m = re.search(r"panicked at [^\n]*\n([^\n]*)", txt)
                why = m.group(1)[:300] if m else txt[-300:]
            rec["ran"].append(dict(test=t, ok=ok, s=round(time.time() - t1, 1), why=why))
            return ok
        for t in spec["tests"]:
            if not test(t):
                rec["killed_by"].append(t)
        for it in spec.get("itests", []):
            log = os.path.join(d, "itest_%s.log" % it)
            t1 = time.time()
            r = run(["cargo", "test", "--offline", "--locked", "-j", "4", "--test", it], root,
                    dict(env, RUST_TEST_THREADS="2"), log)
            txt = open(log, errors="replace").read()
            ok = r == 0
            m = re.search(r"panicked at [^\n]*\n([^\n]*)", txt)
            rec["ran"].append(dict(test="itest::" + it, ok=ok, s=round(time.time() - t1, 1),
                                   why="" if ok else (m.group(1)[:300] if m else txt[-300:])))
            if not ok:
                rec["killed_by"].append("itest::" + it)
        if name == "NONE":
            for t in ALL_TESTS:
                if t not in spec["tests"]:
                    if not test(t):
                        rec["killed_by"].append(t)
        elif not rec["killed_by"]:
            for t in ALL_TESTS:
                if t in spec["tests"] or t in SLOW:
                    continue
                if not test(t):
                    rec["killed_by"].append(t)
                    break
        if name == "NONE":
            rec["result"] = "PASS" if not rec["killed_by"] else "NONE-FAILS"
        else:
            rec["result"] = "KILLED" if rec["killed_by"] else "SURVIVED"
            rec["survivor"] = not rec["killed_by"]
    rec["expect"] = spec.get("expect", "kill")
    rec["what"] = spec["what"]
    shutil.rmtree(target, ignore_errors=True)
    with open(os.path.join(HERE, "results.jsonl"), "a") as f:
        f.write(json.dumps(rec) + "\n")
    print(json.dumps(dict((k, rec[k]) for k in ("name", "result", "build_s", "killed_by"))))

def check_edits():
    """Applies every mutant's edits to the candidate's files in memory."""
    cache = {}
    for name, spec in MUTANTS.items():
        files = {}
        for (path, old, new, count) in spec["edits"]:
            if path not in files:
                if path not in cache:
                    cache[path] = subprocess.run(["git", "-C", REPO, "show", "%s:%s/%s" % (COMMIT, FK, path)],
                                                 stdout=subprocess.PIPE, check=True).stdout.decode()
                files[path] = cache[path]
            n = files[path].count(old)
            if n != count:
                print("BAD %s %s: %d (want %d)" % (name, path, n, count))
            files[path] = files[path].replace(old, new)
    print("checked", len(MUTANTS))


if __name__ == "__main__":
    if sys.argv[1] == "--check-edits":
        check_edits()
    elif sys.argv[1] == "--list":
        for k, v in MUTANTS.items():
            print(k, "|", v["what"])
    else:
        for n in sys.argv[1:]:
            try:
                main(n)
            except Exception as e:  # recorded, never skipped silently
                with open(os.path.join(HERE, "results.jsonl"), "a") as f:
                    f.write(json.dumps(dict(name=n, result="HARNESS-ERROR", error=str(e)[:500])) + "\n")
                print(json.dumps(dict(name=n, result="HARNESS-ERROR", error=str(e)[:200])))
