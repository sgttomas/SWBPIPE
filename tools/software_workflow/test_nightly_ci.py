"""A scheduled full run skips only an unchanged main and fails toward running; a push always runs."""
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


def test_a_successful_after_merge_run_of_the_same_head_lets_the_schedule_skip():
    """The after-merge run is a push run of this same workflow, so the one history
    lookup holds it: the newest success on main, whatever event started it."""
    def runs(*entries):
        return {"workflow_runs": [{"head_sha": sha, "head_branch": "main", "conclusion": conclusion, "event": event}
                                  for sha, conclusion, event in entries]}
    after_merge = nightly_ci.last_successful_sha(runs((A, "success", "push"), (B, "success", "schedule")))
    assert nightly_ci.decide("schedule", A, after_merge)[0] is False
    # A failed or cancelled after-merge run of the head does not: the newest success is an older commit.
    for conclusion in ("failure", "cancelled", "timed_out", None):
        older = nightly_ci.last_successful_sha(runs((A, conclusion, "push"), (B, "success", "push")))
        assert nightly_ci.decide("schedule", A, older)[0] is True
    # A later failure of the same head outweighs its earlier success.
    later_failure = nightly_ci.last_successful_sha(runs((A, "failure", "workflow_dispatch"), (A, "success", "push")))
    assert nightly_ci.decide("schedule", A, later_failure)[0] is True
    # A re-run that fails keeps its place in the listing, and still decides over a later skip.
    rerun = {"workflow_runs": [
        {"head_sha": A, "head_branch": "main", "conclusion": "success", "event": "schedule", "updated_at": "2026-11-10T09:01:00Z"},
        {"head_sha": A, "head_branch": "main", "conclusion": "failure", "event": "push", "updated_at": "2026-11-10T11:00:00Z"}]}
    assert nightly_ci.decide("schedule", A, nightly_ci.last_successful_sha(rerun))[0] is True
    # A scheduled run is never evidence, even when it finishes after the failed re-run.
    rerun["workflow_runs"][0]["updated_at"] = "2026-11-10T12:00:00Z"
    assert nightly_ci.decide("schedule", A, nightly_ci.last_successful_sha(rerun))[0] is True
    skip_only = runs((A, "success", "schedule"))
    assert nightly_ci.decide("schedule", A, nightly_ci.last_successful_sha(skip_only))[0] is True
    # Nor does a success of an earlier merge when a later merge has moved main.
    assert nightly_ci.decide("schedule", A, nightly_ci.last_successful_sha(runs((B, "success", "push"))))[0] is True


def test_dispatch_and_push_always_run_and_other_events_are_refused():
    assert nightly_ci.decide("workflow_dispatch", A, A)[0] is True
    # The after-merge run never skips, even when that head already passed or history is unreadable.
    assert nightly_ci.decide("push", A, A)[0] is True
    assert nightly_ci.decide("push", A, None, lookup_error="HTTPError")[0] is True
    for event in ("pull_request", "merge_group", "release", ""):
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
    assert run("push") == "run=true\n"


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
