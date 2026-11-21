#!/usr/bin/env python3
"""Keep one standing issue for drift found by the retained registration drift check on main.

The last job of retained-drift-check.yml runs this on every run (docs/CI_STRATEGY.md, "Retained registration drift
check"). The check has two parts, the pricing tool's tests in four shards and the registered build's Rust tests; it
fails when either fails and passes when both pass. On main:

- a failed check opens the standing issue, labelled `registration-drift`, or, when one is open, adds a comment naming
  the commit, the failing test count and the run. The count is unknown, never zero, when a report is missing or
  incomplete: a shard that crashed before writing its results, or a Rust job that did not compile;
- a passed check closes the open issue only when its commit is, or descends from, the last failed commit the issue
  records and its run was created after that failure's run; a re-run of an older green run closes nothing;
- any other outcome (cancelled, skipped, unreadable) reports nothing, since it says nothing about drift.

A green run shows that the checked revision still matches the committed registration pins, so the registered bound
holds by that registration; it runs no four-point pricing of any revision. A run of another ref reports nothing. The
label is not `nightly`: drift is repaired at the solver-core freeze before a release candidate, so it does not take the
triage an open `nightly` issue asks for before each merge.

Every value comes from the environment, the test reports or the API and is placed in the issue by this script, never
in a shell command.
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

LABEL = "registration-drift"
MARKER = "<!-- retained-registration-drift -->"
FAILURE = re.compile(r"<!-- retained-drift-check-failed ([0-9a-f]{40}) run=(\d+) created=([0-9TZ:.+-]+) -->")
TITLE = "Retained registration drift: main no longer matches the committed registration pins"
JOBS = ("python", "rust")
SHARDS = 4
CARGO_RESULT = re.compile(r"^test result: \w+\. \d+ passed; (\d+) failed;", re.M)
CARGO_RUNNING = re.compile(r"^\s*Running ", re.M)
MAX_PAGES = 10
PASSED_SENTENCE = ("The drift check passed: this revision matches the committed registration pins, so the registered "
                   "bound holds by that registration; it priced no revision.")


def combined(needs) -> str:
    """"failure" when a part failed, "success" when every part passed, otherwise what was read."""
    try:
        results = [needs[job]["result"] for job in JOBS]
    except (KeyError, TypeError):
        return "unreadable"
    if "failure" in results:
        return "failure"
    if all(result == "success" for result in results):
        return "success"
    return ",".join(str(result) for result in results)


def junit_failures(path: Path) -> int | None:
    try:
        root = ElementTree.parse(path).getroot()
    except (ElementTree.ParseError, OSError):
        return None
    suites = [root] if root.tag == "testsuite" else root.findall("testsuite")
    if not suites:
        return None
    try:
        return sum(int(suite.get("failures", "0")) + int(suite.get("errors", "0")) for suite in suites)
    except ValueError:
        return None


def cargo_failures(path: Path) -> int | None:
    """The failures in a Cargo test log, or None unless every test binary it started reported a result: a log that
    stops at a compile error, or before a binary's result, has no count."""
    try:
        log = path.read_text(errors="replace")
    except OSError:
        return None
    results, running = CARGO_RESULT.findall(log), len(CARGO_RUNNING.findall(log))
    if not results or running != len(results):
        return None
    return sum(int(count) for count in results)


def failing_count(directory: Path, result: str) -> int | None:
    """Failures plus errors over the four shards' JUnit reports and the Rust log, or None when any report is missing
    or incomplete, or when a failed check reads as having no failing test."""
    counts = [junit_failures(directory / f"shard-{shard}.xml") for shard in range(1, SHARDS + 1)]
    counts.append(cargo_failures(directory / "rust.cargo.log"))
    if any(count is None for count in counts):
        return None
    total = sum(counts)
    return None if result == "failure" and total == 0 else total


def standing_issue(issues) -> int | None:
    numbers = sorted(issue["number"] for issue in issues if isinstance(issue, dict)
                     and "pull_request" not in issue and issue.get("state") == "open"
                     and isinstance(issue.get("number"), int) and not isinstance(issue["number"], bool)
                     and isinstance(issue.get("body"), str) and MARKER in issue["body"]
                     and LABEL in label_names(issue)) if isinstance(issues, list) else []
    return numbers[0] if numbers else None


def last_failure(texts) -> tuple[str, int, str] | None:
    """The latest failure an issue records (its commit, run id and run creation time), by creation time."""
    found = [(sha, int(run), created) for body in texts if isinstance(body, str) for sha, run, created in FAILURE.findall(body)]
    return max(found, key=lambda item: item[2]) if found else None


def closes(failure, passing_created: str, relation: str | None) -> bool:
    """A pass closes the issue only when its run was created after the recorded failure's and its commit is, or
    descends from, the failed commit (`relation` is the comparison's status from the failed to the passing commit)."""
    if failure is None or not passing_created:
        return False
    return passing_created > failure[2] and relation in ("identical", "ahead")


def decide(result: str, issues) -> tuple[str, int | None]:
    """("none" | "open" | "comment" | "consider-close", issue number) from the check's result and the open issues."""
    standing = standing_issue(issues)
    if result == "failure":
        return ("comment", standing) if standing else ("open", None)
    if result == "success" and standing:
        return "consider-close", standing
    return "none", None


def render_failure(sha: str, count: int | None, run_url: str, run_id: int, created: str) -> str:
    failing = "unknown (a report is missing or incomplete)" if count is None else str(count)
    return "\n".join([
        f"<!-- retained-drift-check-failed {sha} run={run_id} created={created} -->",
        f"The retained registration drift check failed on `main` at {sha}.", "",
        f"- Failing tests: {failing}", f"- Run: {run_url}", "",
        "The check's tests stop where `main` differs from what the pricing tool's pins and the registered profile were "
        "reviewed against; it priced no revision. Re-registration is done at the solver-core freeze before a release "
        "candidate (DEL-04-08 REQ-004). Until then, no local debug run of this source is retained-route evidence.", ""])


def list_pages(path: str) -> list:
    items = []
    for page in range(1, MAX_PAGES + 1):
        batch = api("GET", f"{path}{'&' if '?' in path else '?'}per_page=100&page={page}")
        if not isinstance(batch, list):
            raise ValueError("Malformed listing")
        items += batch
        if len(batch) < 100:
            break
    return items


def main() -> int:
    try:
        result = combined(json.loads(os.environ.get("RESULTS", "")))
    except ValueError:
        result = "unreadable"
    if os.environ.get("GITHUB_REF") != MAIN_REF:
        print(f"Not a run of main; the drift check ended as {text(result)} and nothing is reported.")
        return 0
    sha, repository = os.environ["GITHUB_SHA"], os.environ["GITHUB_REPOSITORY"]
    if not SHA.fullmatch(sha) or not REPOSITORY.fullmatch(repository):
        raise ValueError("Unexpected commit or repository identity")
    run_id, attempt = int(os.environ["GITHUB_RUN_ID"]), int(os.environ.get("GITHUB_RUN_ATTEMPT") or "1")
    server = os.environ.get("GITHUB_SERVER_URL", "https://github.com")
    run_url = f"{server}/{repository}/actions/runs/{run_id}/attempts/{attempt}"
    if result not in ("failure", "success"):
        print(f"The drift check ended as {text(result)}; nothing is reported.")
        return 0
    action, number = decide(result, list_pages(f"/repos/{repository}/issues?state=open&labels={LABEL}"))
    if action == "none":
        print("The drift check passed and no drift issue is open.")
        return 0
    # The run's own creation time; a re-run keeps its run's, so it cannot pass for a later run.
    created = api("GET", f"/repos/{repository}/actions/runs/{run_id}").get("created_at", "")
    if action == "consider-close":
        issue = api("GET", f"/repos/{repository}/issues/{number}")
        comments = list_pages(f"/repos/{repository}/issues/{number}/comments")
        failure = last_failure([issue.get("body")] + [comment.get("body") for comment in comments if isinstance(comment, dict)])
        relation = None
        if failure is not None:
            relation = api("GET", f"/repos/{repository}/compare/{failure[0]}...{sha}").get("status")
        if not closes(failure, created, relation):
            print(f"The drift check passed at {sha}, but not on a later run descending from issue #{number}'s last "
                  "recorded failure; the issue stays open.")
            return 0
        api("POST", f"/repos/{repository}/issues/{number}/comments",
            {"body": f"{PASSED_SENTENCE} Commit {sha}, run {run_url}. Closing.\n"})
        api("PATCH", f"/repos/{repository}/issues/{number}", {"state": "closed", "state_reason": "completed"})
        print(f"Closed {LABEL} issue #{number}: the drift check passed at {sha}.")
        return 0
    body = render_failure(sha, failing_count(Path(os.environ.get("REPORT_DIR", "drift-reports")), result),
                          run_url, run_id, created)
    if action == "comment":
        api("POST", f"/repos/{repository}/issues/{number}/comments", {"body": body})
        print(f"Commented on {LABEL} issue #{number} for {sha}.")
        return 0
    created_issue = api("POST", f"/repos/{repository}/issues", {"title": TITLE, "body": MARKER + "\n\n" + body, "labels": [LABEL]})
    if not isinstance(created_issue, dict) or LABEL not in label_names(created_issue):
        raise RuntimeError(f"The issue was created without the {LABEL} label")
    print(f"Opened {LABEL} issue #{created_issue.get('number')} for {sha}.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
