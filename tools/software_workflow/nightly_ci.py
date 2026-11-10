#!/usr/bin/env python3
"""Decide whether a full run of main has a commit to verify.

A scheduled run is skipped only when the most recently finished run of the
full-run workflow on main succeeded on main's current head. A run that skipped
the suite also concludes success, and that carries the verdict forward soundly:
runs of main share one concurrency group, so a skip is only ever decided from a
finished run that succeeded on the same head. A manual dispatch always runs. An
unavailable or malformed run history fails toward running.
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
    """Head SHA of the most recently finished run on `branch` if it succeeded, else None.

    Runs are ordered by when they last finished, because GitHub lists them by
    creation and a re-run keeps its place.
    """
    if not isinstance(history, dict):
        raise ValueError("Malformed workflow run history")
    runs = history.get("workflow_runs")
    if not isinstance(runs, list):
        raise ValueError("Malformed workflow run listing")
    for run in runs:
        if not isinstance(run, dict):
            raise ValueError("Malformed workflow run entry")
        if not isinstance(run.get("updated_at", ""), str):
            raise ValueError("Malformed run time in workflow run listing")
    for run in sorted(runs, key=lambda entry: entry.get("updated_at", ""), reverse=True):
        if run.get("head_branch") != branch or run.get("conclusion") is None:
            continue
        if run.get("conclusion") != "success":
            return None
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
        raise ValueError(f"Unsupported full-run event {event!r}")
    if lookup_error:
        return True, f"Last successful run unavailable ({lookup_error}); running the full suite"
    if last_success is None:
        return True, "The newest completed full run on main did not succeed, or there is none; running the full suite"
    if last_success == sha:
        return False, f"main is unchanged at {sha} since the last successful full run; skipped"
    return True, f"main moved from {last_success} to {sha}; running the full suite"


def fetch_history(api: str, repository: str, workflow: str, token: str) -> dict:
    query = urllib.parse.urlencode({"branch": BRANCH, "status": "completed", "exclude_pull_requests": "true", "per_page": "100"})
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
        except Exception as exc:  # any lookup failure (truncated read, malformed JSON, ...) fails toward running
            error = f"{type(exc).__name__}: {exc}"
    run, reason = decide(args.event, args.sha, last, error)
    print(reason)
    if os.getenv("GITHUB_OUTPUT"):
        with open(os.environ["GITHUB_OUTPUT"], "a") as stream:
            stream.write(f"run={'true' if run else 'false'}\n")
    if os.getenv("GITHUB_STEP_SUMMARY"):
        with open(os.environ["GITHUB_STEP_SUMMARY"], "a") as stream:
            stream.write(f"### Full suite of main: {'runs' if run else 'skipped'}\n\n{reason}.\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
