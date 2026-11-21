#!/usr/bin/env python3
"""Keep one standing issue for drift found by the retained pricing audit on main.

The last job of retained-pricing-audit.yml runs this on every run
(docs/CI_STRATEGY.md, "Retained pricing audit"). On main:

The audit has two jobs, the Python tool tests and the Rust registration tests
in the registered build identity; it fails when either fails and passes when
both pass.

- a failed audit opens the standing issue, labelled `pricing-drift`, or, when one
  is open, adds a comment naming the commit, the failing test count and the run;
- a passed audit closes an open standing issue with a comment naming the commit
  and the run;
- any other outcome (cancelled, skipped, unreadable) reports nothing, since it
  says nothing about drift.

A run of another ref reports nothing. The label is not `nightly`: drift is
repaired at the solver-core freeze before a release candidate, so it does not
take the triage an open `nightly` issue asks for before each merge.

Every value comes from the environment, the test reports or the API and is
placed in the issue by this script, never in a shell command.
"""
from __future__ import annotations

from pathlib import Path
import json
import os
import re
import sys
import xml.etree.ElementTree as ElementTree

sys.path.insert(0, str(Path(__file__).resolve().parent))
from nightly_report import MAIN_REF, REPOSITORY, SHA, api, label_names, text  # noqa: E402

LABEL = "pricing-drift"
MARKER = "<!-- retained-pricing-audit-drift -->"
TITLE = "Retained pricing audit: main's priced source has drifted from the registered profile"
MAX_PAGES = 10


JOBS = ("audit", "rust-audit")
CARGO_RESULT = re.compile(r"^test result: \w+\. \d+ passed; (\d+) failed;", re.M)


def combined(needs) -> str:
    """"failure" when an audit job failed, "success" when every one passed, otherwise what was read."""
    try:
        results = [needs[job]["result"] for job in JOBS]
    except (KeyError, TypeError):
        return "unreadable"
    if "failure" in results:
        return "failure"
    if all(result == "success" for result in results):
        return "success"
    return ",".join(str(result) for result in results)


def failing_count(directory: Path) -> int | None:
    """Failures plus errors over the JUnit reports and the Cargo test logs (`*.cargo.log`) in `directory`, or None
    when none can be read."""
    total, read = 0, False
    for path in sorted(directory.glob("**/*.cargo.log")) if directory.is_dir() else []:
        try:
            found = CARGO_RESULT.findall(path.read_text(errors="replace"))
        except OSError:
            continue
        total += sum(int(count) for count in found)
        read = read or bool(found)
    for path in sorted(directory.glob("**/*.xml")) if directory.is_dir() else []:
        try:
            root = ElementTree.parse(path).getroot()
        except (ElementTree.ParseError, OSError):
            continue
        suites = [root] if root.tag == "testsuite" else root.findall("testsuite")
        for suite in suites:
            try:
                total += int(suite.get("failures", "0")) + int(suite.get("errors", "0"))
                read = True
            except ValueError:
                continue
    return total if read else None


def decide(result: str, issues) -> tuple[str, int | None]:
    """("none" | "open" | "comment" | "close", issue number) from the audit job's result and the open issues."""
    numbers = sorted(issue["number"] for issue in issues if isinstance(issue, dict)
                     and "pull_request" not in issue and issue.get("state") == "open"
                     and isinstance(issue.get("number"), int) and not isinstance(issue["number"], bool)
                     and isinstance(issue.get("body"), str) and MARKER in issue["body"]
                     and LABEL in label_names(issue)) if isinstance(issues, list) else []
    standing = numbers[0] if numbers else None
    if result == "failure":
        return ("comment", standing) if standing else ("open", None)
    if result == "success" and standing:
        return "close", standing
    return "none", None


def render_failure(sha: str, count: int | None, run_url: str) -> str:
    failing = "could not be read from the reports" if count is None else str(count)
    return "\n".join([
        f"The retained pricing audit failed on `main` at {sha}.", "",
        f"- Failing audit tests: {failing}", f"- Run: {run_url}", "",
        "The audit's tests stop where the priced source differs from what the pricing tool's pins and the "
        "registered profile were reviewed against. Re-registration is done at the solver-core freeze before a "
        "release candidate (DEL-04-08 REQ-004). Until then, no local debug run of this source is retained-route "
        "evidence.", ""])


def main() -> int:
    try:
        result = combined(json.loads(os.environ.get("RESULTS", "")))
    except ValueError:
        result = "unreadable"
    if os.environ.get("GITHUB_REF") != MAIN_REF:
        print(f"Not a run of main; the audit ended as {text(result)} and nothing is reported.")
        return 0
    sha, repository = os.environ["GITHUB_SHA"], os.environ["GITHUB_REPOSITORY"]
    if not SHA.fullmatch(sha) or not REPOSITORY.fullmatch(repository):
        raise ValueError("Unexpected commit or repository identity")
    run_id, attempt = int(os.environ["GITHUB_RUN_ID"]), int(os.environ.get("GITHUB_RUN_ATTEMPT") or "1")
    server = os.environ.get("GITHUB_SERVER_URL", "https://github.com")
    run_url = f"{server}/{repository}/actions/runs/{run_id}/attempts/{attempt}"
    if result not in ("failure", "success"):
        print(f"The audit ended as {text(result)}; nothing is reported.")
        return 0
    issues = []
    for page in range(1, MAX_PAGES + 1):
        batch = api("GET", f"/repos/{repository}/issues?state=open&labels={LABEL}&per_page=100&page={page}")
        if not isinstance(batch, list):
            raise ValueError("Malformed issue listing")
        issues += batch
        if len(batch) < 100:
            break
    action, number = decide(result, issues)
    if action == "none":
        print("The audit passed and no drift issue is open.")
        return 0
    if action == "close":
        api("POST", f"/repos/{repository}/issues/{number}/comments",
            {"body": f"The retained pricing audit passed on `main` at {sha} ({run_url}). Closing.\n"})
        api("PATCH", f"/repos/{repository}/issues/{number}", {"state": "closed", "state_reason": "completed"})
        print(f"Closed {LABEL} issue #{number}: the audit passed at {sha}.")
        return 0
    body = render_failure(sha, failing_count(Path(os.environ.get("JUNIT_DIR", "audit-junit"))), run_url)
    if action == "comment":
        api("POST", f"/repos/{repository}/issues/{number}/comments", {"body": body})
        print(f"Commented on {LABEL} issue #{number} for {sha}.")
        return 0
    created = api("POST", f"/repos/{repository}/issues", {"title": TITLE, "body": MARKER + "\n\n" + body, "labels": [LABEL]})
    if not isinstance(created, dict) or LABEL not in label_names(created):
        raise RuntimeError(f"The issue was created without the {LABEL} label")
    print(f"Opened {LABEL} issue #{created.get('number')} for {sha}.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
