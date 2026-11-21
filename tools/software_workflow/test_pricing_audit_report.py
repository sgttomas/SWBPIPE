"""The retained pricing audit keeps one standing `pricing-drift` issue on main: opened by a failure, updated by later
failures, closed by a pass; nothing else reports."""
import json

import pricing_audit_report as report

A = "a" * 40


def issue(number, **changes):
    return {"number": number, "state": "open", "labels": [{"name": report.LABEL}],
            "body": f"{report.MARKER}\n\ntext", **changes}


def needs(python="success", rust="success"):
    return json.dumps({"audit": {"result": python, "outputs": {}}, "rust-audit": {"result": rust, "outputs": {}}})


def test_the_audit_fails_when_either_job_fails_and_passes_when_both_pass():
    assert report.combined(json.loads(needs())) == "success"
    assert report.combined(json.loads(needs(rust="failure"))) == "failure"
    assert report.combined(json.loads(needs(python="failure", rust="cancelled"))) == "failure"
    assert report.combined(json.loads(needs(rust="cancelled"))) == "success,cancelled"
    assert report.combined({"audit": {"result": "success"}}) == "unreadable"
    assert report.combined(None) == "unreadable"


def test_a_failure_opens_the_standing_issue_or_updates_it():
    assert report.decide("failure", []) == ("open", None)
    assert report.decide("failure", [issue(9), issue(4)]) == ("comment", 4)


def test_a_pass_closes_an_open_standing_issue_and_otherwise_reports_nothing():
    assert report.decide("success", [issue(4)]) == ("close", 4)
    assert report.decide("success", []) == ("none", None)


def test_other_outcomes_say_nothing_about_drift():
    for result in ("cancelled", "skipped", "", None):
        assert report.decide(result, [issue(4)]) == ("none", None)


def test_only_an_open_labelled_issue_with_the_marker_is_the_standing_one():
    others = [issue(1, state="closed"), issue(2, labels=[{"name": "nightly"}]), issue(3, body="text"),
              issue(5, pull_request={}), issue(True), "junk"]
    assert report.decide("failure", others) == ("open", None)
    assert report.decide("failure", None) == ("open", None)


def test_the_label_is_not_the_nightly_triage_label():
    assert report.LABEL != "nightly"


def test_failing_count_sums_failures_and_errors_over_the_shard_reports(tmp_path):
    (tmp_path / "one.xml").write_text('<testsuites><testsuite failures="3" errors="1" tests="9"/></testsuites>')
    (tmp_path / "two.xml").write_text('<testsuite failures="2" errors="0" tests="5"/>')
    (tmp_path / "broken.xml").write_text("<testsuite")
    assert report.failing_count(tmp_path) == 6
    (tmp_path / "rust.cargo.log").write_text("test result: FAILED. 900 passed; 4 failed; 1 ignored; 0 measured\n"
                                             "test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured\n")
    assert report.failing_count(tmp_path) == 10
    (tmp_path / "rust.cargo.log").unlink()
    assert report.failing_count(tmp_path / "missing") is None
    (tmp_path / "one.xml").unlink()
    (tmp_path / "two.xml").unlink()
    assert report.failing_count(tmp_path) is None


def test_the_failure_comment_names_the_commit_count_and_run():
    body = report.render_failure(A, 7, "https://github.com/o/r/actions/runs/1/attempts/1")
    assert A in body and "Failing audit tests: 7" in body and "/actions/runs/1/attempts/1" in body
    assert "could not be read" in report.render_failure(A, None, "u")


def test_a_run_of_another_ref_reports_nothing(monkeypatch, capsys):
    monkeypatch.setenv("GITHUB_REF", "refs/heads/release-candidate")
    monkeypatch.setenv("RESULTS", needs(python="failure"))
    monkeypatch.setattr(report, "api", lambda *a, **k: (_ for _ in ()).throw(AssertionError("no API call")))
    assert report.main() == 0
    assert "Not a run of main" in capsys.readouterr().out


def test_main_opens_comments_and_closes_through_the_api(monkeypatch):
    calls = []
    standing = []

    def api(method, path, payload=None):
        calls.append((method, path, payload))
        if method == "GET":
            return standing
        return {"number": 11, "labels": [{"name": report.LABEL}]}

    monkeypatch.setattr(report, "api", api)
    for name, value in {"GITHUB_REF": "refs/heads/main", "GITHUB_SHA": A, "GITHUB_REPOSITORY": "o/r",
                        "GITHUB_RUN_ID": "1", "RESULTS": needs(rust="failure"), "JUNIT_DIR": "/nonexistent"}.items():
        monkeypatch.setenv(name, value)
    assert report.main() == 0
    assert calls[-1][:2] == ("POST", "/repos/o/r/issues") and calls[-1][2]["labels"] == [report.LABEL]
    assert calls[-1][2]["body"].startswith(report.MARKER)
    standing.append(issue(11))
    calls.clear()
    assert report.main() == 0
    assert calls[-1][:2] == ("POST", "/repos/o/r/issues/11/comments")
    monkeypatch.setenv("RESULTS", needs())
    calls.clear()
    assert report.main() == 0
    assert calls[-1] == ("PATCH", "/repos/o/r/issues/11", {"state": "closed", "state_reason": "completed"})
