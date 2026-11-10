"""A scheduled full run skips only an unchanged main and fails toward running."""
import os
from pathlib import Path
import subprocess
import sys

import pytest

import nightly_ci

ROOT = Path(__file__).resolve().parents[2]
A, B = "a" * 40, "b" * 40


def history(*runs):
    return {"total_count": len(runs), "workflow_runs": [
        {"head_sha": sha, "head_branch": branch, "conclusion": conclusion} for sha, branch, conclusion in runs]}


def test_last_successful_sha_is_the_newest_success_on_main():
    assert nightly_ci.last_successful_sha(history()) is None
    assert nightly_ci.last_successful_sha(history((B, "main", "failure"), (A, "main", "success"))) is None
    assert nightly_ci.last_successful_sha(history((A, "main", "cancelled"), (A, "main", "success"))) is None
    assert nightly_ci.last_successful_sha(history((B, "main", None), (A, "main", "success"))) == A
    assert nightly_ci.last_successful_sha(history((B, "feature", "success"), (A, "main", "success"))) == A
    assert nightly_ci.last_successful_sha(history((B, "main", "success"), (A, "main", "success"))) == B
    for malformed in ({}, None, [], "x", {"workflow_runs": None}, {"workflow_runs": ["x"]},
                      history(("HEAD", "main", "success"))):
        with pytest.raises(ValueError):
            nightly_ci.last_successful_sha(malformed)


def test_schedule_skips_only_when_main_is_unchanged_since_the_last_success():
    assert nightly_ci.decide("schedule", A, A)[0] is False
    assert nightly_ci.decide("schedule", B, A)[0] is True
    assert nightly_ci.decide("schedule", A, None)[0] is True
    # An unavailable history runs rather than skipping coverage.
    assert nightly_ci.decide("schedule", A, A, lookup_error="HTTPError")[0] is True


def test_the_newest_finished_run_decides_whatever_started_it():
    """Runs of main share one concurrency group, so a run that skipped carries
    the verdict of the finished run it was decided from."""
    def runs(*entries):
        return {"workflow_runs": [{"head_sha": sha, "head_branch": "main", "conclusion": conclusion, "event": event}
                                  for sha, conclusion, event in entries]}
    def skips(history):
        return not nightly_ci.decide("schedule", A, nightly_ci.last_successful_sha(history))[0]
    assert skips(runs((A, "success", "schedule")))
    assert skips(runs((A, "success", "workflow_dispatch"), (B, "success", "schedule")))
    # Runs recorded before the after-merge trigger was removed still count.
    assert skips(runs((A, "success", "push")))
    # A failed or cancelled run of the head does not let the schedule skip, so the suite runs nightly until one passes.
    for conclusion in ("failure", "cancelled", "timed_out"):
        assert not skips(runs((A, conclusion, "schedule"), (A, "success", "schedule")))
    # A success at an earlier commit does not cover a head that has moved.
    assert not skips(runs((B, "success", "schedule")))
    # Listing order is by creation: a re-run that fails keeps its place and still decides.
    rerun = {"workflow_runs": [
        {"head_sha": A, "head_branch": "main", "conclusion": "success", "event": "schedule", "updated_at": "2026-11-10T09:01:00Z"},
        {"head_sha": A, "head_branch": "main", "conclusion": "failure", "event": "schedule", "updated_at": "2026-11-10T11:00:00Z"}]}
    assert not skips(rerun)


def test_dispatch_always_runs_and_other_events_are_refused():
    assert nightly_ci.decide("workflow_dispatch", A, A)[0] is True
    for event in ("push", "pull_request", "merge_group", "release", ""):
        with pytest.raises(ValueError):
            nightly_ci.decide(event, A, None)
    with pytest.raises(ValueError):
        nightly_ci.decide("schedule", "HEAD", None)


def test_cli_writes_the_decision_and_fails_toward_running_without_history(tmp_path):
    def run(event):
        output = tmp_path / f"{event}.out"
        env = {k: v for k, v in os.environ.items() if k not in {"GH_TOKEN", "GITHUB_REPOSITORY"}}
        # No token or repository: the history lookup fails, so the schedule still runs.
        subprocess.run([sys.executable, str(ROOT / "tools/software_workflow/nightly_ci.py"), "--event", event,
                        "--sha", A, "--workflow", "nightly-full-ci.yml"],
                       env={**env, "GITHUB_OUTPUT": str(output)}, check=True, capture_output=True)
        return output.read_text()
    assert run("schedule") == "run=true\n"
    assert run("workflow_dispatch") == "run=true\n"


def test_any_history_lookup_failure_fails_toward_running(tmp_path, monkeypatch):
    import http.client
    failures = [http.client.IncompleteRead(b"{"), ValueError("bad JSON"), OSError("network"),
                AttributeError("x"), TypeError("x")]
    for failure in failures:
        def fetch(*_args, _failure=failure):
            raise _failure
        output = tmp_path / "out"
        output.write_text("")
        monkeypatch.setattr(nightly_ci, "fetch_history", fetch)
        monkeypatch.setenv("GITHUB_REPOSITORY", "o/r")
        monkeypatch.setenv("GH_TOKEN", "t")
        monkeypatch.setenv("GITHUB_OUTPUT", str(output))
        monkeypatch.setattr(sys, "argv", ["nightly_ci.py", "--event", "schedule", "--sha", A,
                                          "--workflow", "nightly-full-ci.yml"])
        assert nightly_ci.main() == 0
        assert output.read_text() == "run=true\n", type(failure).__name__
    for document in (None, []):
        monkeypatch.setattr(nightly_ci, "fetch_history", lambda *_a, _d=document: _d)
        output.write_text("")
        nightly_ci.main()
        assert output.read_text() == "run=true\n"
