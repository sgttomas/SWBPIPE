"""Replay PR828 observations for scheduling only; does not run or qualify tests."""
import argparse
import copy
import gzip
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tarfile
import tempfile
import types

BASE = "fd195cf4287e84572a12183169478a6f7ddf6a92"
HEAD = "127677e1fff8b6c68044dfe0ed8475e3d1dd63d4"
CONT = Path("projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/ROOT/CONTINUATION_2026-09-19_CODEX")
TOOL = "projects/chirality-piping/tools/ci/e2e_plan.py"
HINTS = "projects/chirality-piping/tools/ci/e2e_duration_hints.json"
SKIP_WEIGHT = 0.05  # Positive scheduling floor; not an observed test duration.

args_parser = argparse.ArgumentParser(description=__doc__)
args_parser.add_argument("--repo-root", type=Path, required=True)
args_parser.add_argument("--output", type=Path, required=True)
args_parser.add_argument("--report", type=Path, required=True)
args = args_parser.parse_args()
root = args.repo_root.resolve()

def git_bytes(path):
    return subprocess.check_output(["git", "show", f"{BASE}:{path}"], cwd=root)

def key(row):
    return " › ".join([row["project"], row["file"], *row["title_path"]])

archive = root / CONT / "_run_records/B3B_C3_PUBLICATION_PREPARATION/evidence.tar.gz"
assert hashlib.sha256(archive.read_bytes()).hexdigest() == "69fe6130ea1842d2ebb628fa951edfeb3cb10fbc28a0daad5dfc13fccdc5a22b"
with tarfile.open(archive) as bundle:
    collection = json.load(bundle.extractfile("collection/full-collection/collection.json"))
    plan = json.load(bundle.extractfile("collection/plan.json"))
source = collection["selected"]
by_key = {key(row): row for row in source}
assert len(source) == len(by_key) == 435
log_path = root / CONT / "_run_records/PR828_CI/desktop-ci.log.gz"
log_bytes = gzip.decompress(log_path.read_bytes())
log_hash = hashlib.sha256(log_bytes).hexdigest()
assert log_hash == "d0b62e4856b7fd3e703b90c8ca1edb3cce952bcedd3b0fbc0de5039547425f5a"
passed = re.compile(r"\[(chromium-[^]]+)\] › (e2e/[^:]+):\d+:\d+ › (.+) \(([\d.]+)(ms|s|m)\)\s*$")
skipped = re.compile(r"\s-\s+\d+ \[(chromium-[^]]+)\] › (e2e/[^:]+):\d+:\d+ › (.+)\s*$")
durations = {}
skip_keys = set()

def normalize(match):
    value = " › ".join(match.group(1, 2, 3))
    if value.endswith(" @explicit-viewport"):
        value = value.removesuffix(" @explicit-viewport")
        assert "@explicit-viewport" in by_key[value]["tags"]
    assert value in by_key, value
    return value

for line in log_bytes.decode().splitlines():
    match = passed.search(line)
    if match:
        identity = normalize(match)
        assert identity not in durations
        durations[identity] = float(match[4]) * {"ms": 0.001, "s": 1, "m": 60}[match[5]]
    else:
        match = skipped.search(line)
        if match:
            identity = normalize(match)
            assert identity not in skip_keys
            skip_keys.add(identity)
assert len(durations) == 415 and len(skip_keys) == 20
assert not (durations.keys() & skip_keys)
assert durations.keys() | skip_keys == by_key.keys()
old = json.loads(git_bytes(HINTS))
new = copy.deepcopy(old)
new["seconds"].update(durations)
new["seconds"].update({identity: SKIP_WEIGHT for identity in skip_keys})
new["seconds"] = dict(sorted(new["seconds"].items()))
new["basis"] = {
    "repository": "sgttomas/chirality", "pr": 828, "run": 35538747925,
    "head": HEAD, "log_sha256": log_hash,
    "meaning": "Observed successful source durations; known skips use a positive scheduling floor, not a measured duration. Scheduling hints only, never acceptance limits or coverage selection. New/unobserved identities retain the conservative fallback.",
    "passed_duration_count": len(durations), "observed_skip_count": len(skip_keys),
    "skipped_scheduling_weight_seconds": SKIP_WEIGHT,
    "prior_duration_basis": old["basis"],
    "retained_prior_only_entries": len(set(old["seconds"]) - set(by_key)),
}
# Execute only the exact previously reviewed partition algorithm on its frozen source bodies.
module = types.ModuleType("historical_e2e_plan")
module.__file__ = str(root / TOOL)
exec(compile(git_bytes(TOOL), TOOL, "exec"), module.__dict__)
report = {
    "kind": "offline-scheduling-replay-not-test-execution", "source": BASE,
    "published_head": HEAD, "hosted_run": 35538747925, "log_sha256": log_hash,
    "source_identities": len(source), "observed_passes": len(durations),
    "observed_skips": len(skip_keys), "skip_floor_is_measured_duration": False,
    "unknown_seconds_unchanged": old["unknown_seconds"] == new["unknown_seconds"] == 30,
    "results": {},
}
with tempfile.TemporaryDirectory(prefix="swbpipe-ci-scheduling-") as temp:
    temp_root = Path(temp)
    for file in {row["file"] for row in source}:
        path = temp_root / module.DESKTOP / file
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(git_bytes(str(Path(module.DESKTOP) / file)))
    for label, hints in [("previous", old), ("updated", new)]:
        module.duration_hints = lambda current=hints: current
        parts = module.assign_partitions(plan, source, temp_root)
        ids = [row["id"] for rows in parts.values() for row in rows]
        assert len(ids) == len(set(ids)) == len(source)
        assert set(ids) == {row["id"] for row in source}
        report["results"][label] = {
            "counts": {name: len(rows) for name, rows in parts.items()},
            "replayed_observed_seconds": {name: round(sum(durations.get(key(row), 0) for row in rows), 3) for name, rows in parts.items()},
            "omitted": 0, "duplicated": 0,
        }
new_bytes = (json.dumps(new, indent=2, ensure_ascii=False) + "\n").encode()
report["updated_hints_sha256"] = hashlib.sha256(new_bytes).hexdigest()
report["limitations"] = "One-run scheduling estimate; skipped cases contribute zero replayed test duration. Does not predict setup, contention or future duration, and is not a new CI/test/performance pass. Confirm scheduling effect in the next normal applicable CI run."
args.output.write_bytes(new_bytes)
args.report.parent.mkdir(parents=True, exist_ok=True)
args.report.write_text(json.dumps(report, indent=2) + "\n")
print(json.dumps(report, indent=2))
