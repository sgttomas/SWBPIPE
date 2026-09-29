#!/usr/bin/env python3
"""K4 after RV19's review: one mutant from a clean copy of the candidate, with
its own target (deleted afterwards). Standard library only.

The candidate is uncommitted (ROOT commits), so a clean copy is the
`git archive` of the reviewed head's frame_kernel with every file the fixes
change (tracked or new, under frame_kernel) copied over it from the working
tree; nothing else of the working tree enters.

  python3 rv19_mut.py NAME...     NONE, RV19-1R, A1R, RV19-M2, RV19-M6

Each mutant is a list of exact source edits (each `old` must occur exactly as
many times as stated). Every K4 test of `structural::retained` runs except the
N5 arithmetic streams and the sum differentials (the arithmetic of `wide_sum`
the fixes do not touch) and K3's own; every failing test is recorded."""
import json, os, re, shutil, subprocess, sys, time
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.join(os.path.dirname(HERE), "k4")
BASE = "7d8fa9c0e"
FK = "projects/chirality-piping/core/solver/frame_kernel"
ENV = dict(os.environ, RUSTUP_TOOLCHAIN="1.97.1", RUSTUP_AUTO_INSTALL="0", CARGO_INCREMENTAL="0")
R = "src/structural/retained/"
MODULES = ["adaptive", "assemble", "bound", "combine", "directed", "factor", "ledger", "recover",
           "source", "verify", "wide_sum"]

MUTANTS = {
    "NONE": dict(what="the unmutated candidate", edits=[]),
    # ROOT's ruling on RV19-1 reverted: every row sets the stop rule's S*.
    "RV19-1R": dict(what="the RV19-1 fix reverted (rows the candidate cannot publish set S* again)",
                    edits=[(R + "adaptive.rs",
                            "            .map(|q| !q.is_zero() && q.to_binary64().value().is_none())\n",
                            "            .map(|_q| false)\n", 1)]),
    # Amendment A1 reverted: b = fl↑(2^-64·S*) for every absolute row.
    "A1R": dict(what="amendment A1 reverted (b = fl↑(2^-64·S*) below 2^-988 too)",
                edits=[(R + "adaptive.rs", "            bound_bits: row_bound(value, s_star).to_bits(),",
                        "            bound_bits: absolute_bound(s_star).to_bits(),", 1)]),
    # RV19's M2: ‖SĀS‖ = the ∞-norm only.
    "RV19-M2": dict(what="‖SĀS‖ taken as the ∞-norm only (RV19's M2)",
                    edits=[(R + "verify.rs", "            n.sas = wmax(&n.sas_one, &n.sas_inf);",
                            "            n.sas = n.sas_inf;", 1)]),
    # RV19's M6: a support group's E without its directional contributors.
    "RV19-M6": dict(what="a support group's E without its directional contributors (RV19's M6)",
                    edits=[(R + "verify.rs",
                            "                    if (offset..offset + 3).contains(&c) {\n"
                            "                        sum.add_wide(&directional_e[k][c - offset], false)?;\n"
                            "                    }\n", "                    let _ = (offset, k);\n", 1)]),
}


def run(cmd, cwd, env, log):
    with open(log, "w") as f:
        return subprocess.run(cmd, cwd=cwd, env=env, stdout=f, stderr=subprocess.STDOUT).returncode


def candidate_files():
    """frame_kernel files the fixes change against BASE (tracked or new)."""
    out = subprocess.run(["git", "--no-optional-locks", "-C", REPO, "diff", "--name-only", BASE, "--", FK],
                         stdout=subprocess.PIPE, check=True).stdout.decode().split()
    new = subprocess.run(["git", "--no-optional-locks", "-C", REPO, "ls-files", "--others",
                          "--exclude-standard", "--", FK], stdout=subprocess.PIPE, check=True).stdout.decode().split()
    return sorted(set(out) | set(new))


def main(name):
    spec = MUTANTS[name]
    d = os.path.join(HERE, name)
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
    arch = subprocess.run(["git", "-C", REPO, "archive", BASE, FK], stdout=subprocess.PIPE, check=True).stdout
    subprocess.run(["tar", "-x", "-C", d], input=arch, check=True)
    overlay = candidate_files()
    for rel in overlay:
        src = os.path.join(REPO, rel)
        dst = os.path.join(d, rel)
        if os.path.exists(src):
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copyfile(src, dst)
        elif os.path.exists(dst):
            os.remove(dst)
    root = os.path.join(d, FK)
    for (path, old, new, count) in spec["edits"]:
        p = os.path.join(root, path)
        s = open(p).read()
        n = s.count(old)
        assert n == count, "%s: %s: expected %d occurrences, found %d" % (name, path, count, n)
        open(p, "w").write(s.replace(old, new))
    target = os.path.join(d, "target")
    env = dict(ENV, CARGO_TARGET_DIR=target)
    t0 = time.time()
    rc = run(["cargo", "test", "--offline", "--locked", "-j", "4", "--lib", "--no-run"], root, env,
             os.path.join(d, "build.log"))
    rec = dict(name=name, what=spec["what"], overlay=len(overlay), build_rc=rc,
               build_s=round(time.time() - t0), failed=[], passed=0)
    if rc != 0:
        rec["result"] = "COMPILE-FAIL"
    else:
        deps = os.path.join(target, "debug", "deps")
        bins = [os.path.join(deps, f) for f in os.listdir(deps)
                if f.startswith("open_pipe_stress_frame_kernel-") and "." not in f
                and os.access(os.path.join(deps, f), os.X_OK)]
        assert len(bins) == 1, bins
        log = os.path.join(d, "tests.log")
        t1 = time.time()
        args = [bins[0]] + ["structural::retained::%s" % m for m in MODULES]
        args += ["--skip", "n5_stream", "--skip", "sum_differential", "--test-threads=2", "--nocapture"]
        run(args, root, dict(env, RUST_TEST_THREADS="2"), log)
        txt = open(log, errors="replace").read()
        rec["tests_s"] = round(time.time() - t1)
        rec["failed"] = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED", txt, re.M)))
        rec["passed"] = len(re.findall(r"^test (\S+) \.\.\. ok", txt, re.M))
        rec["why"] = {}
        for t in rec["failed"]:
            m = re.search(r"thread '%s' \([0-9]+\) panicked at [^\n]*\n([^\n]*)" % re.escape(t), txt)
            rec["why"][t] = m.group(1)[:400] if m else ""
        if name == "NONE":
            rec["result"] = "PASS" if not rec["failed"] else "NONE-FAILS"
        else:
            rec["result"] = "KILLED" if rec["failed"] else "SURVIVED"
    shutil.rmtree(target, ignore_errors=True)
    with open(os.path.join(HERE, "results.jsonl"), "a") as f:
        f.write(json.dumps(rec) + "\n")
    print(json.dumps(dict((k, rec.get(k)) for k in ("name", "result", "build_s", "tests_s", "passed", "failed"))))


if __name__ == "__main__":
    for n in sys.argv[1:]:
        main(n)
