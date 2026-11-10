#!/usr/bin/env python3
"""Decide whether the nightly full run has a new main commit to verify.

A scheduled run proceeds only when main's head differs from the head of the
last successful run of the nightly workflow on main. A manual dispatch always
runs. An unavailable or malformed run history fails toward running.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import urllib.parse
import urllib.request

BRANCH = "main"
SHA = re.compile(r"[0-9a-f]{40}|[0-9a-f]{64}")


def last_successful_sha(history: dict, branch: str = BRANCH) -> str | None:
    """Head SHA of the newest successful run on `branch` in a workflow-runs listing (newest first)."""
    runs = history.get("workflow_runs")
    if not isinstance(runs, list):
        raise ValueError("Malformed workflow run listing")
    for run in runs:
        if not isinstance(run, dict):
            raise ValueError("Malformed workflow run entry")
        if run.get("conclusion") == "success" and run.get("head_branch") == branch:
            sha = run.get("head_sha")
            if not isinstance(sha, str) or not SHA.fullmatch(sha):
                raise ValueError("Malformed head SHA in workflow run listing")
            return sha
    return None


def decide(event: str, sha: str, last_success: str | None, lookup_error: str | None = None) -> tuple[bool, str]:
    if not SHA.fullmatch(sha):
        raise ValueError(f"Not a commit SHA: {sha!r}")
    if event == "workflow_dispatch":
        return True, "Manual dispatch always runs the full suite"
    if event != "schedule":
        raise ValueError(f"Unsupported nightly event {event!r}")
    if lookup_error:
        return True, f"Last successful run unavailable ({lookup_error}); running the full suite"
    if last_success is None:
        return True, "No earlier successful nightly run on main; running the full suite"
    if last_success == sha:
        return False, f"main is unchanged at {sha} since the last successful nightly run; skipped"
    return True, f"main moved from {last_success} to {sha}; running the full suite"


def fetch_history(api: str, repository: str, workflow: str, token: str) -> dict:
    query = urllib.parse.urlencode({"branch": BRANCH, "status": "success", "exclude_pull_requests": "true", "per_page": "20"})
    url = f"{api}/repos/{repository}/actions/workflows/{urllib.parse.quote(workflow, safe='')}/runs?{query}"
    request = urllib.request.Request(url, headers={"Accept": "application/vnd.github+json",
                                                   "Authorization": f"Bearer {token}",
                                                   "X-GitHub-Api-Version": "2022-11-28"})
    with urllib.request.urlopen(request, timeout=30) as response:
        return json.load(response)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--event", required=True)
    parser.add_argument("--sha", required=True)
    parser.add_argument("--workflow", required=True, help="This workflow's file name")
    args = parser.parse_args()
    last, error = None, None
    if args.event == "schedule":
        try:
            history = fetch_history(os.environ.get("GITHUB_API_URL", "https://api.github.com"),
                                    os.environ["GITHUB_REPOSITORY"], args.workflow, os.environ["GH_TOKEN"])
            last = last_successful_sha(history)
        except (OSError, KeyError, ValueError) as exc:
            error = f"{type(exc).__name__}: {exc}"
    run, reason = decide(args.event, args.sha, last, error)
    print(reason)
    if os.getenv("GITHUB_OUTPUT"):
        with open(os.environ["GITHUB_OUTPUT"], "a") as stream:
            stream.write(f"run={'true' if run else 'false'}\n")
    if os.getenv("GITHUB_STEP_SUMMARY"):
        with open(os.environ["GITHUB_STEP_SUMMARY"], "a") as stream:
            stream.write(f"### Nightly full suite: {'runs' if run else 'skipped'}\n\n{reason}.\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
