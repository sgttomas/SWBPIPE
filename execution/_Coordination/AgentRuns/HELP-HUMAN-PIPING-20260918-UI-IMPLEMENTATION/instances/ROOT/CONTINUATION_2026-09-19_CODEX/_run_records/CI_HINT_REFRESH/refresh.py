"""Reproduce the bounded PR832 duration refresh; no browser/test execution."""
import argparse
import copy
import gzip
import hashlib
import json
import math
from pathlib import Path
import re
import subprocess
import tempfile
import types


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--apply", action="store_true")
    args = parser.parse_args()
    root = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip())
    out = Path(__file__).resolve().parent
    records = out.parent
    candidate = "8077032231ded1cc5d206becd03a18e43e6f3bb2"
    prior = "c9f19195214b2e097980dde5e0d5fccbfba8e34c"
    hint_path = "projects/chirality-piping/tools/ci/e2e_duration_hints.json"
    code_path = "projects/chirality-piping/tools/ci/e2e_plan.py"

    def git_bytes(ref, path):
        return subprocess.check_output(["git", "show", f"{ref}:{path}"], cwd=root)

    code = git_bytes(candidate, code_path)
    module = types.ModuleType("pr832_scheduler")
    module.__file__ = str(root / code_path)
    exec(compile(code, module.__file__, "exec"), module.__dict__)
    source_bytes = gzip.decompress((records / "B4_2_PUBLICATION_CHECKS/collection/source.stdout.json.gz").read_bytes())
    source = module.collected_tests(json.loads(source_bytes))
    module.validate_source(source)
    plan_bytes = gzip.decompress((records / "B4_2_PUBLICATION_CHECKS/ci-plan.json.gz").read_bytes())
    plan = json.loads(plan_bytes)
    assert plan["mode"] == "full"
    run_bytes = (records / "PR832_CI/corrected-run.json").read_bytes()
    run = json.loads(run_bytes)
    assert run["headSha"] == candidate and run["conclusion"] == "success"
    raw_log = gzip.decompress((records / "PR832_CI/corrected-run.log.gz").read_bytes())
    assert sha(raw_log) == "5350a46b8e76e788f1b96f39abd046b1b59560db2d5f6451aede3c1259d396ae"

    def key(row):
        return " › ".join([row["project"], row["file"], *row["title_path"]])

    reporter = {}
    for row in source:
        canonical = key(row)
        reported = canonical + (" " + " ".join(row["tags"]) if row["tags"] else "")
        assert reported not in reporter
        reporter[reported] = canonical
    passed = re.compile(r"✓\s+\d+\s+\[([^\]]+)\]\s+›\s+(e2e/[^:]+):\d+:\d+\s+›\s+(.+)\s+\(([\d.]+)(ms|s|m)\)$")
    skipped = re.compile(r"-\s+\d+\s+\[([^\]]+)\]\s+›\s+(e2e/[^:]+):\d+:\d+\s+›\s+(.+)$")
    durations, skips = {}, set()
    for line in raw_log.decode().splitlines():
        match = passed.search(line)
        if match:
            project, file, title, value, unit = match.groups()
            canonical = reporter[" › ".join([project, file, title])]
            assert canonical not in durations
            durations[canonical] = float(value) * {"ms": .001, "s": 1, "m": 60}[unit]
        else:
            match = skipped.search(line)
            if match:
                canonical = reporter[" › ".join(match.groups())]
                assert canonical not in skips
                skips.add(canonical)
    keys = {key(row) for row in source}
    assert len(source) == 455 and len(durations) == 435 and len(skips) == 20
    assert set(durations) | skips == keys and not set(durations) & skips

    old_bytes = git_bytes(prior, hint_path)
    old = json.loads(old_bytes)
    assert old["unknown_seconds"] == 30
    new = copy.deepcopy(old)
    new["seconds"].update(durations)
    rounded_zero = {key for key, value in durations.items() if value == 0}
    assert len(rounded_zero) == 1 and all(old["seconds"][key] == .001 for key in rounded_zero)
    new["seconds"].update({key: old["seconds"][key] for key in rounded_zero})
    new["seconds"].update({item: .05 for item in skips})
    new["seconds"] = dict(sorted(new["seconds"].items()))
    new["basis"] = {
        "repository": "sgttomas/chirality", "pr": 832, "run": 35567286692,
        "head": candidate, "log_sha256": sha(raw_log),
        "collection_sha256": sha(source_bytes),
        "meaning": "Observed successful source durations, resolved to canonical collection identities including separately reported tags. Existing skips and one pass rounded to0ms use positive scheduling floors, not measured durations. Scheduling hints only, never acceptance limits or coverage selection; new/unobserved identities retain the conservative fallback.",
        "passed_duration_count": len(durations), "observed_skip_count": len(skips),
        "skipped_scheduling_weight_seconds": .05,
        "rounded_zero_pass_count": len(rounded_zero),
        "rounded_zero_scheduling_weight_seconds": .001,
        "prior_duration_basis": old["basis"],
        "retained_prior_only_entries": len(set(old["seconds"]) - keys),
    }
    assert all(type(value) in (int, float) and math.isfinite(value) and value > 0
               for value in [new["unknown_seconds"], *new["seconds"].values()])
    body_hashes = {}
    with tempfile.TemporaryDirectory(prefix="swbpipe-pr832-hints-") as directory:
        historical = Path(directory)
        for file in {row["file"] for row in source}:
            path = str(Path(module.DESKTOP) / file)
            body = git_bytes(candidate, path)
            target = historical / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(body)
            body_hashes[path] = sha(body)
        module.duration_hints = lambda: old
        before = module.assign_partitions(plan, source, historical)
        module.duration_hints = lambda: new
        after = module.assign_partitions(plan, source, historical)
    identities = lambda parts: {module.test_key(row) for rows in parts.values() for row in rows}
    assert identities(before) == identities(after)
    assert sum(map(len, before.values())) == sum(map(len, after.values())) == 455
    assert before["barrier"] == after["barrier"]
    observed_sums = lambda parts: {name: round(sum(durations.get(key(row), 0) for row in rows), 3) for name, rows in parts.items()}
    new_bytes = (json.dumps(new, ensure_ascii=False, indent=2) + "\n").encode()
    report = {
        "kind": "historical scheduling replay; not executed future timing",
        "head": candidate, "priorHintsCommit": prior,
        "sourceInventory": 455, "observedPassDurations": 435, "existingSkips": 20,
        "newlyMeasuredKeys": len(set(durations) - set(old["seconds"])),
        "unchangedFallbackSeconds": 30, "unchangedSkipWeightSeconds": .05,
        "retainedPriorOnlyKeys": new["basis"]["retained_prior_only_entries"],
        "roundedZeroPassesUsingExistingPositiveWeight": sorted(rounded_zero),
        "identicalCoverageAndBarrier": True,
        "oldBinRecordedDurationSums": observed_sums(before),
        "newBinRecordedDurationSums": observed_sums(after),
        "newBinCounts": {name: len(rows) for name, rows in after.items()},
        "bindings": {"log": sha(raw_log), "collection": sha(source_bytes), "plan": sha(plan_bytes), "run": sha(run_bytes), "selector": sha(code), "priorHints": sha(old_bytes), "refreshedHints": sha(new_bytes)},
        "historicalSourceBodies": dict(sorted(body_hashes.items())),
        "limit": "Replay sums recorded test durations only, excluding setup/runner/reporting variance. Materials and new Node scenarios were absent from PR832 and retain fallback weights. Actual later CI timing remains unobserved.",
    }
    (out / "REPLAY.json").write_text(json.dumps(report, indent=2) + "\n")
    if args.apply:
        (root / hint_path).write_bytes(new_bytes)
    print(json.dumps({"applied": args.apply, "observedPassDurations": 435, "existingSkips": 20, "newlyMeasuredKeys": report["newlyMeasuredKeys"], "before": report["oldBinRecordedDurationSums"], "after": report["newBinRecordedDurationSums"], "hintSha256": sha(new_bytes)}, indent=2))


if __name__ == "__main__":
    main()
