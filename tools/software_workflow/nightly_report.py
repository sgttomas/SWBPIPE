#!/usr/bin/env python3
"""Report a full run of main that did not pass as a GitHub issue labelled `nightly`.

The last job of nightly-full-ci.yml runs this on every run. A run that passed,
or a scheduled run that skipped an unchanged main, reports nothing. Any other
outcome on main (a failed, cancelled or timed-out job, or results that cannot
be read) opens an issue naming the commit, the merged pull request, the jobs
that did not succeed and the run, or adds a comment to the open issue for the
same commit. The decision to report uses the workflow's job results alone;
lookups that only add detail fail soft.

Every value comes from the environment, the event file or the API and is
placed in the issue by this script, never in a shell command.
"""
from __future__ import annotations

from collections import Counter
from datetime import datetime
from itertools import count
import json
import os
import re
import time
import urllib.request

LABEL = "nightly"
MAIN_REF = "refs/heads/main"
SHA = re.compile(r"[0-9a-f]{40}|[0-9a-f]{64}")
REPOSITORY = re.compile(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+")
PASSED = frozenset({"success", "skipped", "neutral"})
# GitHub's annotation on a job stopped by `timeout-minutes`. Such a job's
# conclusion is `cancelled`, as for a cancelled run, so the conclusion alone
# cannot tell the two apart.
TIMEOUT_ANNOTATION = "exceeded the maximum execution time"
MERGE_SUBJECT = re.compile(r"Merge pull request #([0-9]{1,9}) from ")
TRIGGERS = {"push": "after-merge run (push to `main`)", "schedule": "scheduled nightly run",
            "workflow_dispatch": "manual dispatch"}
MAX_JOB_PAGES = 5
RETRY_DELAYS = (2, 5)  # seconds between attempts to read the open issues


def outcome(needs) -> tuple[bool, str]:
    """(failed, summary) from the workflow's `needs` results. Only a suite that
    ran and succeeded, or a scheduled skip of an unchanged main, passes."""
    try:
        gate, suite = needs["changed"], needs["full-suite"]
        gate_result, suite_result = gate["result"], suite["result"]
        run = (gate.get("outputs") or {}).get("run")
    except (KeyError, TypeError, AttributeError):
        return True, "the job results could not be read"
    if gate_result == "success" and run == "true" and suite_result == "success":
        return False, "the full suite passed"
    if gate_result == "success" and run == "false" and suite_result == "skipped":
        return False, "main is unchanged since the last successful full run; the suite was skipped"
    return True, f"the change check ended as {text(gate_result)} and the full suite as {text(suite_result)}"


def marker(sha: str) -> str:
    return f"<!-- nightly-full-run {sha} -->"


def decide(failed: bool, sha: str, issues) -> tuple[str, int | None]:
    """("none" | "open" | "comment", issue number): a failure comments on the
    oldest open `nightly` issue for the same commit, and otherwise opens one."""
    if not failed:
        return "none", None
    numbers = [issue["number"] for issue in issues if isinstance(issue, dict)
               and "pull_request" not in issue and issue.get("state") == "open"
               and isinstance(issue.get("number"), int) and not isinstance(issue["number"], bool)
               and isinstance(issue.get("body"), str) and marker(sha) in issue["body"]
               and LABEL in label_names(issue)] if isinstance(issues, list) else []
    return ("comment", min(numbers)) if numbers else ("open", None)


def label_names(issue: dict) -> set[str]:
    labels = issue.get("labels")
    return {label.get("name") if isinstance(label, dict) else label for label in labels} if isinstance(labels, list) else set()


def text(value) -> str:
    """A value shown in Markdown as inert code: one line, no backticks, bounded."""
    return "`" + (" ".join(str(value).replace("`", "'").split())[:200] or "?") + "`"


def minutes(job: dict) -> float | None:
    try:
        started, completed = (datetime.fromisoformat(job[key].replace("Z", "+00:00")) for key in ("started_at", "completed_at"))
        return max((completed - started).total_seconds(), 0.0) / 60
    except (KeyError, TypeError, ValueError, AttributeError):
        return None


def describe(job: dict, annotations) -> tuple[str, str]:
    """(kind, sentence) for a job that did not succeed. `annotations` is the
    job's annotation list, or None when it was not read."""
    conclusion = job.get("conclusion")
    steps = job.get("steps") if isinstance(job.get("steps"), list) else []
    failed_steps = [text(step.get("name")) for step in steps if isinstance(step, dict) and step.get("conclusion") == "failure"]
    took = minutes(job)
    after = "" if took is None else f" after {took:.1f} min"
    step_note = f"; failed in step {', '.join(failed_steps)}" if failed_steps else "; no step failed"
    timed_out = conclusion == "timed_out" or any(
        isinstance(note, dict) and TIMEOUT_ANNOTATION in str(note.get("message")) for note in annotations or [])
    if timed_out:
        return "timed out", f"timed out{after}: the job reached its time limit{step_note}"
    if conclusion == "cancelled":
        if annotations is None:
            return "cancelled or timed out", f"was cancelled or timed out{after} (GitHub's reason could not be read){step_note}"
        return "cancelled", f"was cancelled{after}{step_note}"
    if conclusion == "failure":
        return "failed", "failed" + (f" in step {', '.join(failed_steps)}" if failed_steps else "") + after
    return "other", f"ended as {text(conclusion)}{after}{step_note}"


def job_lines(jobs: list, annotations_of, link_prefix: str) -> tuple[list[str], Counter]:
    """One Markdown line per completed job that did not succeed, and a count by kind."""
    lines, kinds = [], Counter()
    for job in jobs:
        if not isinstance(job, dict) or job.get("status") != "completed" or job.get("conclusion") in PASSED:
            continue
        annotations = None
        if job.get("conclusion") != "failure":
            try:
                annotations = annotations_of(job)
                if not isinstance(annotations, list):
                    annotations = None
            except Exception:  # the reason is detail; its absence is stated in the line
                annotations = None
        kind, sentence = describe(job, annotations)
        kinds[kind] += 1
        url = job.get("html_url")
        link = f" ([job]({url}))" if isinstance(url, str) and url.startswith(link_prefix) and not re.search(r"[\s()<>]", url) else ""
        lines.append(f"- {text(job.get('name'))} {sentence}{link}")
    return lines, kinds


def merged_pull_request(pulls, sha: str) -> int | None:
    """The pull request whose merge commit is `sha`, from the commit's pull-request listing."""
    for pull in pulls if isinstance(pulls, list) else []:
        if (isinstance(pull, dict) and pull.get("merge_commit_sha") == sha and pull.get("merged_at")
                and isinstance(pull.get("number"), int) and not isinstance(pull["number"], bool)):
            return pull["number"]
    return None


def merge_subject_pull_request(event, sha: str) -> int | None:
    """The pull request a push's head commit message names ("Merge pull request #N from ...")."""
    try:
        commit = event["head_commit"]
        if commit["id"] != sha:
            return None
        match = MERGE_SUBJECT.match(commit["message"])
    except (KeyError, TypeError):
        return None
    return int(match.group(1)) if match else None


def render(sha: str, event: str, pull: int | None, pull_note: str, summary: str, run_url: str,
           lines: list[str], kinds: Counter, notes: list[str]) -> tuple[str, str]:
    title = f"Full run of main did not pass at {sha[:12]}" + (f" (merge of #{pull})" if pull else "")
    merged = f"#{pull}{pull_note}" if pull else "none found for this commit"
    counts = ", ".join(f"{count} {kind}" for kind, count in sorted(kinds.items()))
    body = [marker(sha), "", f"A full run of `main` did not pass: {summary}.", "",
            f"- Commit: {sha}", f"- Merged pull request: {merged}",
            f"- Trigger: {TRIGGERS.get(event, 'unrecognised event')}", f"- Run: {run_url}", ""]
    if lines:
        body += [f"Jobs that did not succeed ({counts}):", "", *lines[:60]]
        if len(lines) > 60:
            body.append(f"- and {len(lines) - 60} more; see the run")
    else:
        body.append("No unsuccessful job was listed; see the run.")
    body += ["", *notes] if notes else []
    return title, "\n".join(body) + "\n"


def api(method: str, path: str, payload: dict | None = None):
    request = urllib.request.Request(
        os.environ.get("GITHUB_API_URL", "https://api.github.com") + path, method=method,
        data=None if payload is None else json.dumps(payload).encode("utf-8"),
        headers={"Accept": "application/vnd.github+json", "Authorization": f"Bearer {os.environ['GH_TOKEN']}",
                 "Content-Type": "application/json", "X-GitHub-Api-Version": "2022-11-28"})
    with urllib.request.urlopen(request, timeout=30) as response:
        return json.load(response)


def run_jobs(repository: str, run_id: int, attempt: int) -> list:
    jobs = []
    for page in range(1, MAX_JOB_PAGES + 1):
        batch = api("GET", f"/repos/{repository}/actions/runs/{run_id}/attempts/{attempt}/jobs?per_page=100&page={page}")["jobs"]
        jobs += batch
        if len(batch) < 100:
            break
    return jobs


def open_issues(repository: str) -> list:
    """Every open issue with the label; a read that keeps failing raises."""
    issues = []
    for page in count(1):
        for delay in (*RETRY_DELAYS, None):
            try:
                batch = api("GET", f"/repos/{repository}/issues?state=open&labels={LABEL}&per_page=100&page={page}")
                if not isinstance(batch, list):
                    raise ValueError("Malformed issue listing")
                break
            except Exception:
                if delay is None:
                    raise
                time.sleep(delay)
        issues += batch
        if len(batch) < 100:
            return issues
    raise AssertionError("unreachable")


def main() -> int:
    try:
        needs = json.loads(os.environ.get("RESULTS", ""))
    except ValueError:
        needs = None
    failed, summary = outcome(needs)
    if not failed:
        print(f"Nothing to report: {summary}.")
        return 0
    if os.environ.get("GITHUB_REF") != MAIN_REF:
        print(f"Not a run of main; no issue for: {summary}.")
        return 0
    sha, repository = os.environ["GITHUB_SHA"], os.environ["GITHUB_REPOSITORY"]
    if not SHA.fullmatch(sha) or not REPOSITORY.fullmatch(repository):
        raise ValueError("Unexpected commit or repository identity")
    run_id, attempt = int(os.environ["GITHUB_RUN_ID"]), int(os.environ.get("GITHUB_RUN_ATTEMPT") or "1")
    server = os.environ.get("GITHUB_SERVER_URL", "https://github.com")
    run_url = f"{server}/{repository}/actions/runs/{run_id}/attempts/{attempt}"
    notes, lines, kinds = [], [], Counter()
    try:
        lines, kinds = job_lines(
            run_jobs(repository, run_id, attempt),
            lambda job: api("GET", f"/repos/{repository}/check-runs/{int(job['id'])}/annotations?per_page=50"),
            f"{server}/{repository}/")
    except Exception as exc:  # the issue is still filed, with the run link
        notes.append(f"The job list could not be read ({type(exc).__name__}); see the run.")
    pull, pull_note = None, ""
    try:
        pull = merged_pull_request(api("GET", f"/repos/{repository}/commits/{sha}/pulls?per_page=100"), sha)
    except Exception:
        pass
    if pull is None:
        try:
            with open(os.environ["GITHUB_EVENT_PATH"], encoding="utf-8") as stream:
                pull = merge_subject_pull_request(json.load(stream), sha)
            pull_note = " (named by the merge commit message)"
        except (KeyError, OSError, ValueError):
            pass
    try:
        issues = open_issues(repository)
    except Exception as exc:  # after the retries, a possible duplicate is better than an unreported failure
        issues = []
        notes.append(f"Open `{LABEL}` issues could not be listed ({type(exc).__name__}); this may repeat one.")
    action, number = decide(True, sha, issues)
    title, body = render(sha, os.environ.get("GITHUB_EVENT_NAME", ""), pull, pull_note, summary, run_url, lines, kinds, notes)
    if action == "comment":
        api("POST", f"/repos/{repository}/issues/{number}/comments", {"body": body})
        print(f"Commented on open {LABEL} issue #{number} for {sha}.")
        return 0
    created = api("POST", f"/repos/{repository}/issues", {"title": title, "body": body, "labels": [LABEL]})
    if not isinstance(created, dict) or LABEL not in label_names(created):
        raise RuntimeError(f"The issue was created without the {LABEL} label")
    print(f"Opened {LABEL} issue #{created.get('number')} for {sha}.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
