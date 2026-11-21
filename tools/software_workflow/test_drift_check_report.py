"""The retained registration drift check keeps one standing `registration-drift` issue on main: opened by a failure,
updated by later failures, closed only by a later pass on a descendant of the last failure; nothing else reports."""
import json

import drift_check_report as report

A, B, C = "a" * 40, "b" * 40, "c" * 40
EARLY, FAIL_TIME, LATE = "2026-11-20T08:30:00Z", "2026-11-21T08:30:00Z", "2026-11-22T08:30:00Z"
CARGO_OK = ("     Running unittests src/lib.rs (target/debug/deps/x)\n"
            "test result: ok. 1220 passed; 0 failed; 91 ignored; 0 measured; 0 filtered out\n"
            "     Running tests/retained_precision_admission.rs (target/debug/deps/y)\n"
            "test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\n")


def needs(python="success", rust="success"):
    return json.dumps({"python": {"result": python, "outputs": {}}, "rust": {"result": rust, "outputs": {}}})


def issue(number, **changes):
    return {"number": number, "state": "open", "labels": [{"name": report.LABEL}],
            "body": f"{report.MARKER}\n\ntext", **changes}


def reports(tmp_path, failures=(0, 0, 0, 0), cargo=CARGO_OK):
    for shard, count in enumerate(failures, start=1):
        if count is not None:
            (tmp_path / f"shard-{shard}.xml").write_text(f'<testsuites><testsuite failures="{count}" errors="0" tests="9"/></testsuites>')
    if cargo is not None:
        (tmp_path / "rust.cargo.log").write_text(cargo)
    return tmp_path


def test_the_check_fails_when_either_part_fails_and_passes_when_both_pass():
    assert report.combined(json.loads(needs())) == "success"
    assert report.combined(json.loads(needs(rust="failure"))) == "failure"
    assert report.combined(json.loads(needs(python="failure", rust="cancelled"))) == "failure"
    assert report.combined(json.loads(needs(rust="cancelled"))) == "success,cancelled"
    assert report.combined({"python": {"result": "success"}}) == "unreadable"
    assert report.combined(None) == "unreadable"


def test_a_failure_opens_the_standing_issue_or_updates_it():
    assert report.decide("failure", []) == ("open", None)
    assert report.decide("failure", [issue(9), issue(4)]) == ("comment", 4)


def test_a_pass_considers_closing_only_an_open_standing_issue():
    assert report.decide("success", [issue(4)]) == ("consider-close", 4)
    assert report.decide("success", []) == ("none", None)
    for result in ("cancelled", "skipped", "unreadable", ""):
        assert report.decide(result, [issue(4)]) == ("none", None)


def test_only_an_open_labelled_issue_with_the_marker_is_the_standing_one():
    others = [issue(1, state="closed"), issue(2, labels=[{"name": "nightly"}]), issue(3, body="text"),
              issue(5, pull_request={}), issue(True), "junk"]
    assert report.decide("failure", others) == ("open", None)
    assert report.decide("failure", None) == ("open", None)
    assert report.LABEL != "nightly"


def test_a_pass_closes_only_on_a_later_run_of_the_failed_commit_or_a_descendant():
    body = report.render_failure(A, 3, "u", 7, FAIL_TIME)
    failure = report.last_failure([body])
    assert failure == (A, 7, FAIL_TIME)
    assert report.closes(failure, LATE, "ahead")
    assert report.closes(failure, LATE, "identical")
    # A re-run of an older green run keeps its run's creation time, and an older commit is behind the failure.
    assert not report.closes(failure, EARLY, "behind")
    assert not report.closes(failure, EARLY, "ahead")
    assert not report.closes(failure, LATE, "behind")
    assert not report.closes(failure, LATE, "diverged")
    assert not report.closes(failure, LATE, None)
    assert not report.closes(None, LATE, "ahead")
    assert not report.closes(failure, "", "ahead")


def test_the_last_recorded_failure_is_the_latest_by_run_creation():
    first = report.render_failure(A, 1, "u", 7, FAIL_TIME)
    later = report.render_failure(B, 2, "u", 8, LATE)
    assert report.last_failure([later, first, None, "no marker"]) == (B, 8, LATE)
    assert report.last_failure(["text"]) is None


def test_failing_count_sums_every_report(tmp_path):
    assert report.failing_count(reports(tmp_path, (3, 0, 2, 0)), "failure") == 5
    failed_rust = CARGO_OK.replace("ok. 1220 passed; 0 failed", "FAILED. 1216 passed; 4 failed")
    assert report.failing_count(reports(tmp_path, (0, 0, 0, 0), failed_rust), "failure") == 4


def test_a_missing_or_incomplete_report_is_an_unknown_count_never_zero(tmp_path):
    # A shard that crashed before writing its report.
    (tmp_path / "a").mkdir()
    assert report.failing_count(reports(tmp_path / "a", (0, None, 0, 0)), "failure") is None
    # A Rust job that did not compile, or stopped before a test binary's result.
    (tmp_path / "b").mkdir()
    assert report.failing_count(reports(tmp_path / "b", cargo="error[E0425]: cannot find value `x`\n"), "failure") is None
    (tmp_path / "c").mkdir()
    assert report.failing_count(reports(tmp_path / "c", cargo=CARGO_OK.rsplit("test result", 1)[0]), "failure") is None
    (tmp_path / "d").mkdir()
    assert report.failing_count(reports(tmp_path / "d", cargo=None), "failure") is None
    # A failed check whose reports show no failing test is unknown, not zero.
    (tmp_path / "e").mkdir()
    assert report.failing_count(reports(tmp_path / "e"), "failure") is None
    assert report.failing_count(tmp_path / "e", "success") == 0
    (tmp_path / "f").mkdir()
    (tmp_path / "f" / "shard-1.xml").write_text("<testsuite")
    assert report.failing_count(reports(tmp_path / "f", (None, 1, 1, 1)), "failure") is None


def test_the_failure_comment_records_the_commit_count_and_run():
    body = report.render_failure(A, 7, "https://github.com/o/r/actions/runs/1/attempts/1", 1, FAIL_TIME)
    assert A in body and "Failing tests: 7" in body and "/actions/runs/1/attempts/1" in body
    assert "priced no revision" in body
    assert "unknown" in report.render_failure(A, None, "u", 1, FAIL_TIME)


def test_a_run_of_another_ref_reports_nothing(monkeypatch, capsys):
    monkeypatch.setenv("GITHUB_REF", "refs/heads/release-candidate")
    monkeypatch.setenv("RESULTS", needs(python="failure"))
    monkeypatch.setattr(report, "api", lambda *a, **k: (_ for _ in ()).throw(AssertionError("no API call")))
    assert report.main() == 0
    assert "Not a run of main" in capsys.readouterr().out


def run_main(monkeypatch, tmp_path, result, sha, created, standing, comments, relation):
    calls = []

    def api(method, path, payload=None):
        calls.append((method, path, payload))
        if method == "GET" and "/issues?state=open" in path:
            return standing
        if method == "GET" and "/actions/runs/" in path:
            return {"created_at": created}
        if method == "GET" and path.endswith("/comments?per_page=100&page=1"):
            return comments
        if method == "GET" and "/compare/" in path:
            return {"status": relation}
        if method == "GET" and "/issues/" in path:
            return standing[0] if standing else {}
        return {"number": 11, "labels": [{"name": report.LABEL}]}

    monkeypatch.setattr(report, "api", api)
    for name, value in {"GITHUB_REF": "refs/heads/main", "GITHUB_SHA": sha, "GITHUB_REPOSITORY": "o/r",
                        "GITHUB_RUN_ID": "9", "RESULTS": result, "REPORT_DIR": str(tmp_path)}.items():
        monkeypatch.setenv(name, value)
    assert report.main() == 0
    return calls


def test_main_opens_comments_and_closes_through_the_api(monkeypatch, tmp_path):
    calls = run_main(monkeypatch, tmp_path, needs(rust="failure"), A, FAIL_TIME, [], [], None)
    assert calls[-1][:2] == ("POST", "/repos/o/r/issues") and calls[-1][2]["labels"] == [report.LABEL]
    opened = calls[-1][2]["body"]
    assert opened.startswith(report.MARKER) and "Failing tests: unknown" in opened
    standing = [issue(11, body=opened)]
    calls = run_main(monkeypatch, tmp_path, needs(python="failure"), B, LATE, standing, [], None)
    assert calls[-1][:2] == ("POST", "/repos/o/r/issues/11/comments")
    later = calls[-1][2]["body"]
    # A re-run of an older green run closes nothing.
    calls = run_main(monkeypatch, tmp_path, needs(), C, EARLY, standing, [{"body": later}], "behind")
    assert not any(method == "PATCH" for method, _, _ in calls)
    # A later pass on a commit that descends only from the first failure, not the last, closes nothing.
    calls = run_main(monkeypatch, tmp_path, needs(), C, "2026-11-23T08:30:00Z", standing, [{"body": later}], "diverged")
    assert not any(method == "PATCH" for method, _, _ in calls)
    # A later pass descending from the last failure closes it, saying what green shows.
    calls = run_main(monkeypatch, tmp_path, needs(), C, "2026-11-23T08:30:00Z", standing, [{"body": later}], "ahead")
    assert any(path == f"/repos/o/r/compare/{B}...{C}" for _, path, _ in calls)
    assert calls[-2][2]["body"].startswith(report.PASSED_SENTENCE)
    assert calls[-1] == ("PATCH", "/repos/o/r/issues/11", {"state": "closed", "state_reason": "completed"})
