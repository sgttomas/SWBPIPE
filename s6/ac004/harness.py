"""RT-A3 S6 final composition (da0afeabb on #164) §8.60 proposal AC-004 window harness: admission identity check, then seven prebuilt processes once each.

Runs inside t3_exclusive.sh. No compilation, no retry. One internal deadline of 55 s on the monotonic clock
(time.monotonic; on macOS mach_absolute_time), started at runner entry, before admission; it bounds admission, the
seven processes and the post-run hashes. Stops at the first failed, missing (zero executed), multiple-test or
timed-out outcome; later tests are recorded NOT RUN. Zero executed is never a pass. The tests run from the frozen
read-only copies named below. CHECK_ONLY=1 runs admission only (no test); a real launch requires it unset.
"""
import hashlib, json, os, re, stat, subprocess, sys, time
from datetime import datetime, timezone

T0 = time.monotonic()  # runner entry: the single deadline's origin
REPO = '/Users/ryan/dev/chirality-t3/swbpipe-rt-a3-s6-final'
COPIES = '/Users/ryan/dev/chirality-t3/scratch/rt-a3-s6-final/ac004/bin/'
LIB = COPIES + 'open_pipe_stress_product_physics-f31c71322c8cc2bc'
CHAL = COPIES + 'retained_memory_challenge-0a8cb81bbc9e389a'
EXPECT = {
    LIB: "8f2b649d4bc6ffd4933aeda986e7a1c2bf4f649315edb57dc721e6a67a7d0c73",
    CHAL: "bba69229159f741484fc0af9ea90ab897232bddab51ee850118bef4d239213a5",
    REPO + "/core/product_physics/src/retained_memory.rs": "24141f216cbb54285dacf7b1419d20867aec71a1ba20ccd7984135ab7ec80081",
    REPO + "/core/product_physics/Cargo.lock": "b7ebb02aafa8cfe214c18bc64941381b33ab3f9295e0fc774ee0819e1e4f5653",
}
HEAD = '1a07aec6b732ffcf1b050626341780ff92888fbd'
CORE = 'e8e83ae9f1b7dbf7a9984eee1fa8d7a7688cad87'
RUSTC = ("1.97.1", "8bab26f4f68e0e26f0bb7960be334d5b520ea452")
FLAGS = ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_RUSTFLAGS", "RUSTDOCFLAGS")
MEMGUARD = "/Users/ryan/dev/chirality-t3/guard/memguard.sh"
DEADLINE_S = 55.0
W = "retained_memory::witness_tests::"
TESTS = [(LIB, W + f"{w}::{m}", True) for w in ("witness_headroom_w1_at_one_mebibyte",
                                              "witness_w2_deep_milestone_publishes", "witness_w_c2_publishes")
         for m in ("sparse", "dense")]
TESTS.append((CHAL, "retained_direct_peak_is_within_the_profiles_bounds", False))
RESULT = re.compile(r"test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored")


def now():
    return datetime.now(timezone.utc).isoformat(timespec="milliseconds")


def elapsed():
    return round(time.monotonic() - T0, 3)


def sha(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


class QueryFailed(Exception):
    pass


def run(*cmd):
    """An admission query; a nonzero exit or a launch failure refuses admission (fail closed)."""
    try:
        return subprocess.run(list(cmd), capture_output=True, text=True, check=True).stdout.strip()
    except (subprocess.CalledProcessError, OSError) as error:
        raise QueryFailed(f"{' '.join(cmd)}: {error}") from error


def read_only(path):
    return not (os.stat(path).st_mode & (stat.S_IWUSR | stat.S_IWGRP | stat.S_IWOTH))


def main():
    record = {"runner_entry": now(), "deadline_s": DEADLINE_S, "results": []}
    check_only = os.environ.get("CHECK_ONLY")
    # Children never see W1_MEMBER_CAP_OUT (cleared); its inherited presence is recorded.
    child_env = {k: v for k, v in os.environ.items() if k != "W1_MEMBER_CAP_OUT"}
    flags = {k: (k in os.environ) for k in FLAGS}  # presence, not value: a flag set to "" is present
    hashes = {p: sha(p) for p in EXPECT}
    try:
        rustc = run("rustc", "-vV")
        head = run("git", "-C", REPO, "rev-parse", "HEAD")
        core = run("git", "-C", REPO, "rev-parse", "HEAD:core")
        status = run("git", "-C", REPO, "status", "--porcelain", "--untracked-files=all")
    except QueryFailed as error:
        record["admission_refused"] = True
        record["query_failed"] = str(error)
        record["end"] = now()
        print(json.dumps(record, indent=1))
        return 3
    check = {
        "head": head,
        "core": core,
        "clean_whole_worktree": status == "",
        "hashes": hashes,
        "flags_present": flags,
        "rustc": [l for l in rustc.splitlines() if l.startswith(("release:", "commit-hash:"))],
        "memguard_live": subprocess.run(["pgrep", "-f", MEMGUARD], capture_output=True).returncode == 0,
        "copies_read_only": all(read_only(p) for p in (COPIES, LIB, CHAL)),
        "w1_member_cap_out_inherited": "W1_MEMBER_CAP_OUT" in os.environ,
        "check_only": check_only,
    }
    record["admission_check"] = check
    ok = (check["head"] == HEAD and check["core"] == CORE and check["clean_whole_worktree"]
          and all(hashes[p] == v for p, v in EXPECT.items()) and all(k not in os.environ for k in FLAGS)
          and check["rustc"] == [f"commit-hash: {RUSTC[1]}", f"release: {RUSTC[0]}"]
          and check["memguard_live"] and check["copies_read_only"] and elapsed() < DEADLINE_S)
    record["admission_seconds"] = elapsed()
    if not ok:
        record["admission_refused"] = True
        record["end"] = now()
        print(json.dumps(record, indent=1))
        return 3
    record["admission_verified"] = now()
    if check_only:
        record["check_only_no_test_run"] = True
        record["end"] = now()
        print(json.dumps(record, indent=1))
        return 0
    stopped = None
    for binary, name, ignored in TESTS:
        entry = {"test": name, "binary": binary.rsplit("/", 1)[1]}
        remaining = DEADLINE_S - (time.monotonic() - T0)
        if stopped or remaining <= 0:
            entry["outcome"] = "NOT RUN" + (f" (stopped after {stopped})" if stopped else " (deadline)")
            record["results"].append(entry)
            stopped = stopped or "deadline"
            continue
        cmd = [binary, name, "--exact", "--test-threads=1"] + (["--ignored"] if ignored else [])
        s = time.monotonic()
        try:
            p = subprocess.run(cmd, cwd=REPO + "/core/product_physics", capture_output=True, text=True,
                               timeout=remaining, env=child_env)
            entry["rc"] = p.returncode
            m = RESULT.search(p.stdout)
            passed, failed = (int(m.group(2)), int(m.group(3))) if m else (None, None)
            entry["executed_passed"], entry["executed_failed"] = passed, failed
            entry["summary"] = [l for l in p.stdout.splitlines() if l.startswith("test result") or " ... " in l]
            if p.returncode == 0 and passed == 1 and failed == 0:
                entry["outcome"] = "pass"
            elif passed == 0 and failed == 0:
                entry["outcome"] = "MISSING (zero executed)"
            elif (passed or 0) + (failed or 0) > 1:
                entry["outcome"] = "MULTIPLE TESTS"
            else:
                entry["outcome"] = "FAIL"
            if entry["outcome"] != "pass":
                entry["stdout_tail"] = p.stdout[-2000:]
                entry["stderr_tail"] = p.stderr[-2000:]
        except subprocess.TimeoutExpired:
            entry["outcome"] = "KILLED (deadline)"
        entry["seconds"] = round(time.monotonic() - s, 2)
        if entry["outcome"] != "pass":
            stopped = name
        record["results"].append(entry)
    post = {p: sha(p) for p in (LIB, CHAL)}
    record["post_run_hashes"] = post
    record["post_run_hashes_unchanged"] = all(post[p] == EXPECT[p] for p in post)
    record["harness_seconds"] = elapsed()
    record["within_deadline"] = record["harness_seconds"] <= DEADLINE_S
    record["end"] = now()
    print(json.dumps(record, indent=1))
    return 0 if not stopped and record["post_run_hashes_unchanged"] and record["within_deadline"] else 1


if __name__ == "__main__":
    sys.exit(main())
