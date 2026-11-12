"""A full run of main that did not pass yields one open `nightly` issue; a pass yields none."""
import json

import pytest

import nightly_report

A, B = "a" * 40, "b" * 40
REPO = "o/r"
TIMED_OUT = [{"annotation_level": "failure", "message": "The job has exceeded the maximum execution time of 1h30m0s"}]
CANCELLED = [{"annotation_level": "failure", "message": "The operation was canceled."}]


def needs(gate="success", run="true", suite="success"):
    return {"changed": {"result": gate, "outputs": {"run": run}}, "full-suite": {"result": suite, "outputs": {}}}


def issue(number, sha, **changes):
    return {"number": number, "state": "open", "labels": [{"name": "nightly"}],
            "body": f"{nightly_report.marker(sha)}\n\ntext", **changes}


def job(name, conclusion, steps=(), job_id=1, status="completed", took=10):
    return {"id": job_id, "name": name, "status": status, "conclusion": conclusion,
            "started_at": "2026-11-10T09:00:00Z", "completed_at": f"2026-11-10T09:{took:02d}:30Z",
            "html_url": f"https://github.com/{REPO}/actions/runs/7/job/{job_id}",
            "steps": [{"name": step, "conclusion": result} for step, result in steps]}


def test_only_a_suite_that_ran_and_passed_or_a_scheduled_skip_is_a_pass():
    assert nightly_report.outcome(needs())[0] is False
    assert nightly_report.outcome(needs(run="false", suite="skipped"))[0] is False
    for failing in (needs(suite="failure"), needs(suite="cancelled"), needs(suite="timed_out"),
                    # Selected coverage that never ran is not a pass.
                    needs(suite="skipped"), needs(run="false", suite="success"), needs(run=""),
                    needs(gate="failure", run="", suite="skipped"), needs(gate="cancelled", run="", suite="skipped"),
                    # Unreadable results report rather than stay silent.
                    None, [], "x", {}, {"changed": {"result": "success"}}, {"changed": None, "full-suite": None}):
        assert nightly_report.outcome(failing)[0] is True, failing


def test_a_failure_opens_one_issue_per_commit_and_comments_on_it_afterwards():
    assert nightly_report.decide(False, A, [issue(3, A)]) == ("none", None)
    assert nightly_report.decide(True, A, []) == ("open", None)
    assert nightly_report.decide(True, A, [issue(3, A)]) == ("comment", 3)
    assert nightly_report.decide(True, A, [issue(9, A), issue(4, A), issue(2, B)]) == ("comment", 4)
    # Another commit's issue, a closed or unlabelled one, a pull request or a malformed listing is not this commit's open issue.
    for other in ([issue(3, B)], [issue(3, A, state="closed")], [issue(3, A, labels=[])], [issue(3, A, pull_request={})],
                  [issue(True, A)], [issue("3", A)], [issue(3, A, body=None)], ["x", None], None, {"message": "Not Found"}):
        assert nightly_report.decide(True, A, other) == ("open", None), other


def test_a_stopped_job_is_named_as_timed_out_or_cancelled_and_not_as_a_failed_test():
    failed = job("Full suite of main / Python product contracts (2/4)", "failure", [("Checkout", "success"), ("Selected tests", "failure")])
    assert nightly_report.describe(failed, None) == ("failed", "failed in step `Selected tests` after 10.5 min")
    # GitHub reports a job stopped by its time limit as cancelled; its annotation tells the two apart.
    stopped = job("Full suite of main / Desktop Rust (src-tauri)", "cancelled", [("Cargo tests", None)], took=59)
    assert nightly_report.describe(stopped, TIMED_OUT) == (
        "timed out", "timed out after 59.5 min: the job reached its time limit; no step failed")
    assert nightly_report.describe(stopped, CANCELLED) == ("cancelled", "was cancelled after 59.5 min; no step failed")
    kind, sentence = nightly_report.describe(stopped, None)
    assert kind == "cancelled or timed out" and "could not be read" in sentence and "no step failed" in sentence
    assert nightly_report.describe(job("x", "timed_out"), [])[0] == "timed out"
    assert nightly_report.describe({"conclusion": "cancelled"}, []) == ("cancelled", "was cancelled; no step failed")
    assert nightly_report.describe(job("x", "startup_failure"), [])[0] == "other"


def test_job_lines_list_only_completed_unsuccessful_jobs_and_survive_unread_annotations():
    def annotations(entry):
        if entry["id"] == 4:
            raise OSError("network")
        return TIMED_OUT
    jobs = [job("a", "success"), job("b", "skipped"), job("c", None, status="in_progress"), "x",
            job("d `e`\n@f", "failure", [("s", "failure")], job_id=2), job("g", "cancelled", job_id=3), job("h", "cancelled", job_id=4)]
    lines, kinds = nightly_report.job_lines(jobs, annotations, f"https://github.com/{REPO}/")
    assert [line.split(" ", 2)[1] for line in lines] == ["`d", "`g`", "`h`"]
    # A name cannot leave its code span, so it can neither format nor mention.
    assert lines[0].startswith("- `d 'e' @f` failed in step `s` after 10.5 min ([job](https://github.com/o/r/actions/runs/7/job/2))")
    assert "timed out" in lines[1] and "was cancelled or timed out" in lines[2]
    assert kinds == {"failed": 1, "timed out": 1, "cancelled or timed out": 1}
    # A link outside the repository is dropped.
    assert "[job]" not in nightly_report.job_lines([job("d", "failure")], annotations, "https://github.com/other/")[0][0]


def test_the_merged_pull_request_is_the_one_whose_merge_commit_is_the_head():
    pulls = [{"number": 7, "merge_commit_sha": B, "merged_at": "t"}, {"number": 8, "merge_commit_sha": A, "merged_at": None},
             {"number": 9, "merge_commit_sha": A, "merged_at": "t"}]
    assert nightly_report.merged_pull_request(pulls, A) == 9
    for none in ([], None, {"message": "x"}, pulls[:2], [{"number": "9", "merge_commit_sha": A, "merged_at": "t"}]):
        assert nightly_report.merged_pull_request(none, A) is None


class Api:
    """The API as the report uses it, recording every write."""

    def __init__(self, issues=(), jobs=(), pulls=(), fail=(), flaky=0):
        self.issues, self.jobs, self.pulls, self.fail = list(issues), list(jobs), list(pulls), set(fail)
        self.flaky = flaky
        self.calls, self.writes = [], []

    def __call__(self, method, path, payload=None):
        self.calls.append((method, path))
        for name in self.fail:
            if name in path and method == "GET":
                raise OSError("unavailable")
        if method == "POST":
            self.writes.append((path, payload))
            if path.endswith("/issues"):
                created = {"number": 50 + len(self.writes), "state": "open", "body": payload["body"],
                           "labels": [{"name": name} for name in payload["labels"]]}
                self.issues.append(created)
                return created
            return {"id": 1}
        if "/jobs?" in path:
            return {"jobs": self.jobs}
        if "/annotations" in path:
            return TIMED_OUT
        if path.endswith("/pulls?per_page=100"):
            return self.pulls
        if "/issues?" in path:
            assert "state=open" in path and "labels=nightly" in path
            if self.flaky:
                self.flaky -= 1
                raise OSError("unavailable")
            page = int(path.rsplit("page=", 1)[1])
            return [entry for entry in self.issues if entry["state"] == "open"][(page - 1) * 100:page * 100]
        raise AssertionError(path)


def report(monkeypatch, tmp_path, api, results, event="schedule", ref="refs/heads/main", sha=A, attempt="1"):
    for name, value in {"RESULTS": results if isinstance(results, str) else json.dumps(results), "GITHUB_REF": ref,
                        "GITHUB_SHA": sha, "GITHUB_REPOSITORY": REPO, "GITHUB_RUN_ID": "7", "GITHUB_RUN_ATTEMPT": attempt,
                        "GITHUB_EVENT_NAME": event, "GH_TOKEN": "t",
                        "GITHUB_SERVER_URL": "https://github.com"}.items():
        monkeypatch.setenv(name, value)
    monkeypatch.setattr(nightly_report, "api", api)
    assert nightly_report.main() == 0
    return api


def test_a_passing_or_skipped_run_reports_nothing_and_calls_nothing(monkeypatch, tmp_path):
    for results in (needs(), needs(run="false", suite="skipped")):
        for event in ("schedule", "workflow_dispatch"):
            api = report(monkeypatch, tmp_path, Api(issues=[issue(3, A)]), results, event=event)
            assert api.calls == []


@pytest.mark.parametrize("event", ["schedule", "workflow_dispatch"])
@pytest.mark.parametrize("suite", ["failure", "cancelled", "timed_out", "skipped"])
def test_a_failed_cancelled_or_timed_out_run_opens_one_issue_then_comments(monkeypatch, tmp_path, event, suite):
    jobs = [job("Full run change check", "success"),
            job("Full suite of main / Desktop Rust (src-tauri)", "cancelled", job_id=2, took=59),
            job("Full suite of main / Python product contracts (2/4)", "failure", [("Selected tests", "failure")], job_id=3),
            job("Full run report", None, status="in_progress", job_id=4)]
    api = Api(jobs=jobs, pulls=[{"number": 220, "merge_commit_sha": A, "merged_at": "t"}])
    report(monkeypatch, tmp_path, api, needs(suite=suite), event=event)
    [(path, payload)] = api.writes
    assert path == "/repos/o/r/issues" and payload["labels"] == ["nightly"]
    assert payload["title"] == f"Full run of main did not pass at {A[:12]} (merge of #220)"
    body = payload["body"]
    assert f"- Commit: {A}\n" in body and "- Merged pull request: #220\n" in body
    assert "- Run: https://github.com/o/r/actions/runs/7/attempts/1\n" in body
    assert "Jobs that did not succeed (1 failed, 1 timed out):" in body
    assert "- `Full suite of main / Desktop Rust (src-tauri)` timed out after 59.5 min: the job reached its time limit; no step failed" in body
    assert "- `Full suite of main / Python product contracts (2/4)` failed in step `Selected tests` after 10.5 min" in body
    assert "Full run report" not in body and "Full run change check" not in body
    # A rerun and another trigger's run of the same commit comment on that issue; another commit opens its own.
    report(monkeypatch, tmp_path, api, needs(suite="cancelled"), event="schedule", attempt="2")
    assert [path for path, _ in api.writes] == ["/repos/o/r/issues", "/repos/o/r/issues/51/comments"]
    assert "/attempts/2" in api.writes[1][1]["body"]
    report(monkeypatch, tmp_path, api, needs(suite="failure"), sha=B)
    assert [path for path, _ in api.writes][2:] == ["/repos/o/r/issues"]
    assert "none found for this commit" in api.writes[2][1]["body"]
    # Once the issue is closed, a new failure of that commit opens a new one.
    api.issues[0]["state"] = "closed"
    report(monkeypatch, tmp_path, api, needs(suite="failure"))
    assert [path for path, _ in api.writes][3:] == ["/repos/o/r/issues"]


def open_for(sha, number):
    return {"number": number, "state": "open", "body": nightly_report.marker(sha), "labels": [{"name": "nightly"}]}


def test_the_open_issue_for_a_commit_is_found_on_a_later_page_and_after_a_failed_read(monkeypatch, tmp_path):
    monkeypatch.setattr(nightly_report, "RETRY_DELAYS", (0, 0))
    others = [open_for(format(n, "040x"), n) for n in range(1, 101)]
    api = report(monkeypatch, tmp_path, Api(issues=[*others, open_for(A, 200)]), needs(suite="failure"))
    [(path, _)] = api.writes
    assert path == "/repos/o/r/issues/200/comments"
    many = [open_for(format(n, "040x"), n) for n in range(1, 1001)]
    api = report(monkeypatch, tmp_path, Api(issues=[open_for(A, 2000), *many]), needs(suite="failure"))
    [(path, _)] = api.writes
    assert path == "/repos/o/r/issues/2000/comments"
    api = report(monkeypatch, tmp_path, Api(issues=[open_for(A, 200)], flaky=2), needs(suite="failure"))
    [(path, _)] = api.writes
    assert path == "/repos/o/r/issues/200/comments"


def test_unreadable_results_and_failed_lookups_still_open_the_issue(monkeypatch, tmp_path):
    monkeypatch.setattr(nightly_report, "RETRY_DELAYS", (0, 0))
    api = report(monkeypatch, tmp_path, Api(fail={"/jobs?", "/pulls?", "/issues?"}), "not json")
    [(path, payload)] = api.writes
    assert path == "/repos/o/r/issues"
    body = payload["body"]
    assert "the job results could not be read" in body and "The job list could not be read (OSError)" in body
    assert "Open `nightly` issues could not be listed (OSError)" in body
    assert "- Merged pull request: none found for this commit\n" in body
    # An unread annotation says so and does not call the job a failed test.
    api = report(monkeypatch, tmp_path, Api(jobs=[job("j", "cancelled")], fail={"/annotations"}), needs(suite="cancelled"))
    assert "- `j` was cancelled or timed out after 10.5 min (GitHub's reason could not be read); no step failed" in api.writes[0][1]["body"]


def test_a_failure_off_main_opens_nothing_and_a_dropped_label_or_failed_write_fails_the_job(monkeypatch, tmp_path):
    api = report(monkeypatch, tmp_path, Api(), needs(suite="failure"), event="workflow_dispatch", ref="refs/heads/feature")
    assert api.calls == []

    class Unlabelled(Api):
        def __call__(self, method, path, payload=None):
            created = super().__call__(method, path, payload)
            return {**created, "labels": []} if method == "POST" else created
    with pytest.raises(RuntimeError):
        report(monkeypatch, tmp_path, Unlabelled(), needs(suite="failure"))

    def refused(method, path, payload=None):
        if method == "POST":
            raise OSError("403")
        return {"jobs": []} if "/jobs?" in path else []
    with pytest.raises(OSError):
        report(monkeypatch, tmp_path, refused, needs(suite="failure"))
