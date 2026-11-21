#!/usr/bin/env python3
"""The window print route (DEL-17-05; tools/caepipe_correlation/window_capture.py).

Every capture here is invented: an invented print in the documented layout, an invented saved model and run line, in
a temporary exchange folder. Nothing captured enters a fixture.
"""

from __future__ import annotations

import hashlib
import json
import os
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external.benchmark import UNMATCHED, WITHIN  # noqa: E402
from tests.test_caepipe_benchmark import csv_text, mirror_csv, solve  # noqa: E402
from tests.test_caepipe_correlation import fake_config  # noqa: E402
from tools.caepipe_correlation import benchmark_hand_entry, run_pilot  # noqa: E402
from tools.caepipe_correlation import window_capture as wc  # noqa: E402

MANIFEST = ROOT / "validation" / "caepipe_benchmark" / "manifest.json"
HAND = "hand-entry"


@pytest.fixture(autouse=True)
def _live_session_passed_through(monkeypatch):
    monkeypatch.setattr(run_pilot, "live_session", lambda config, emit=print: config, raising=False)


def run_entry(model_id: str = "cpb-n-04") -> dict:
    [entry] = run_pilot.load_benchmark(MANIFEST, [model_id], HAND)
    return entry


def invented_print() -> str:
    """A print in the documented layout with one invented section the parser does not read."""
    cases = {"Sustained (W+P)": {"anchors": {"10": ["0", "100", "0", "0", "0", "50"]},
                                 "displacements": {"10": ["0"] * 6, "20": ["0.000", "-1.250", "0", "0", "0", "0"]}}}
    return csv_text(cases) + "Invented unread section: Sustained (W+P),\r\nNode,Value,\r\n20,1,\r\n"


def capture_folder(tmp_path: Path, entry: dict, *, print_text: str | None = None, extra: dict | None = None) -> Path:
    folder = tmp_path / "capture" / entry["run_id"]
    folder.mkdir(parents=True)
    _, mbf_text = wc.run_mbf(entry)
    names = wc._names(entry)
    (folder / names["imported_mbf"]).write_bytes(mbf_text.encode("utf-8"))
    (folder / names["instruction"]).write_bytes(entry["window_instruction"]["text"].encode("utf-8"))
    (folder / names["saved_model"]).write_bytes(b"INVENTED SAVED MODEL\r\n")
    (folder / names["run_line"]).write_text("invented run line: made by hand\n", encoding="utf-8")
    (folder / names["print"]).write_text(print_text if print_text is not None else invented_print(), encoding="utf-8")
    for name, data in (extra or {}).items():
        (folder / name).write_bytes(data)
    return folder


def tree(path: Path) -> dict[str, bytes]:
    return {str(p.relative_to(path)): p.read_bytes() for p in sorted(path.rglob("*")) if p.is_file()}


IDENTITY = {"source_commit": "0" * 40, "source_tree_clean": True}


def test_a_capture_is_stored_whole_by_content_once_and_its_folder_is_never_changed(tmp_path):
    entry = run_entry()
    root = fake_config(tmp_path).exchange_root
    folder = capture_folder(tmp_path, entry, extra={"screen-1.png": b"INVENTED IMAGE",
                                                    f"{entry['run_id']}.res": b"INVENTED RESULT FILE"})
    before = tree(folder)
    capture_id, record = wc.store_capture(entry, root, folder, identity=IDENTITY)
    assert tree(folder) == before  # copied, never moved or changed
    names = wc._names(entry)
    printed = (folder / names["print"]).read_bytes()
    assert capture_id == hashlib.sha256(printed).hexdigest()[:12]
    stored = wc.capture_dir(root.resolve(), entry["run_id"], capture_id)
    assert tree(stored) == {**before, wc.WINDOW_RECORD: (stored / wc.WINDOW_RECORD).read_bytes()}
    roles = {name: item["role"] for name, item in record["files"].items()}
    assert roles[names["print"]] == "print" and roles[names["saved_model"]] == "saved_model"
    assert roles["screen-1.png"] == roles[f"{entry['run_id']}.res"] == wc.KEPT_NOT_EXAMINED
    assert record["stored_at"] == IDENTITY and record["limits"] == wc.WINDOW_LIMITS
    assert record["parse"]["status"] == "parsed"
    assert [item["title"] for item in record["parse"]["sections_not_read"]] == [
        "Invented unread section: Sustained (W+P)"]
    with pytest.raises(wc.CaptureRefused) as again:
        wc.store_capture(entry, root, folder, identity=IDENTITY)
    assert again.value.route == "capture_stored"
    # A different print of the same run is a second capture beside the first.
    (folder / names["print"]).write_text(invented_print() + "\r\n", encoding="utf-8")
    second, _ = wc.store_capture(entry, root, folder, identity=IDENTITY)
    assert second != capture_id and wc.stored_captures(root, entry["run_id"]) == sorted([capture_id, second])


@pytest.mark.parametrize("change, route", [
    ("mbf", "pack_file_differs"), ("instruction", "instruction_differs"), ("missing", "capture_incomplete"),
    ("link", "capture_folder"), ("directory", "capture_folder")])
def test_a_capture_is_refused_before_anything_is_written(tmp_path, change, route):
    entry = run_entry()
    root = fake_config(tmp_path).exchange_root
    folder = capture_folder(tmp_path, entry)
    names = wc._names(entry)
    if change == "mbf":
        (folder / names["imported_mbf"]).write_bytes(b"AN OLDER FILE\r\n")
    elif change == "instruction":
        (folder / names["instruction"]).write_text("another instruction", encoding="utf-8")
    elif change == "missing":
        (folder / names["saved_model"]).unlink()
    elif change == "link":
        os.symlink(folder / names["print"], folder / "linked.csv")
    else:
        (folder / "sub").mkdir()
    before = tree(root)
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.store_capture(entry, root, folder, identity=IDENTITY)
    assert refused.value.route == route and tree(root) == before
    if change == "mbf":
        assert hashlib.sha256(b"AN OLDER FILE\r\n").hexdigest() in refused.value.reason


def test_a_batch_run_is_not_a_window_run(tmp_path):
    [entry] = run_pilot.load_benchmark(MANIFEST, ["cpb-g1-01"])
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.store_capture(entry, fake_config(tmp_path).exchange_root, tmp_path, identity=IDENTITY)
    assert refused.value.route == "not_a_window_run"


def test_a_print_that_does_not_parse_is_stored_and_its_comparison_is_refused_by_name(tmp_path):
    entry = run_entry()
    root = fake_config(tmp_path).exchange_root
    capture_id, record = wc.store_capture(entry, root, capture_folder(tmp_path, entry, print_text="not a print\r\n"),
                                          identity=IDENTITY)
    assert record["parse"]["status"] == "failed" and record["parse"]["reason"]
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=lambda d: pytest.fail("solved"),
                           write=lambda p, v: pytest.fail("written"))
    assert refused.value.route == "print_not_parsed"


def test_a_stored_print_changed_afterwards_is_refused_and_nothing_is_written(tmp_path):
    entry = run_entry()
    root = fake_config(tmp_path).exchange_root
    capture_id, record = wc.store_capture(entry, root, capture_folder(tmp_path, entry), identity=IDENTITY)
    (wc.capture_dir(root.resolve(), entry["run_id"], capture_id) / record["print"]).write_text("changed",
                                                                                                 encoding="utf-8")
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=lambda d: pytest.fail("solved"),
                           write=lambda p, v: pytest.fail("written"))
    assert refused.value.route == "print_digest_mismatch"


def test_a_capture_made_from_another_file_than_the_writers_now_is_refused(tmp_path, monkeypatch):
    entry = run_entry()
    root = fake_config(tmp_path).exchange_root
    capture_id, _ = wc.store_capture(entry, root, capture_folder(tmp_path, entry), identity=IDENTITY)
    real = wc.run_mbf
    monkeypatch.setattr(wc, "run_mbf", lambda e: (real(e)[0], real(e)[1] + "* a later writer\r\n"))
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=lambda d: pytest.fail("solved"),
                           write=lambda p, v: pytest.fail("written"))
    assert refused.value.route == "pack_file_differs" and IDENTITY["source_commit"] in refused.value.reason


@pytest.mark.parametrize("model_id", ["cpb-n-04", "cpb-h-01"])
def test_a_mirror_capture_compares_the_base_and_lists_the_hand_entered_member_unmatched(tmp_path, model_id):
    """SWBPIPE's side is the model with the member; the print mirrors SWBPIPE's own solve through the base file's
    identity map, so every bound row is within the profile or unwitnessed, and the member's reaction rows are listed
    unmatched with the cause by name, the printed section not read."""
    entry = run_entry(model_id)
    root = fake_config(tmp_path).exchange_root
    model = benchmark_hand_entry.hand_entry_model(entry)
    payload = solve(model)
    _, text = mirror_csv(entry, payload)
    capture_id, _ = wc.store_capture(entry, root, capture_folder(tmp_path, entry, print_text=text), identity=IDENTITY)
    written = {}
    solved = []

    def fake_solve(document):
        solved.append(document)
        return solve(document["model"]), "", {"solve_id": f"invented-{len(solved)}"}
    path, comparison = wc.compare_capture(entry, root, capture_id, solve=fake_solve,
                                          write=lambda p, v: written.setdefault(p, v) and "")
    assert solved[0]["model"] == model and written[path] is comparison
    assert comparison["caepipe_print"]["route"] == wc.WINDOW_ROUTE and comparison["caepipe_print"]["capture_id"] == \
        capture_id
    assert comparison["window_capture"]["limits"] == wc.WINDOW_LIMITS
    members = {member["id"] for member in entry["hand_entry"]["member"]}
    assert set(comparison["window_capture"]["members_not_compared"]) == members
    member_rows = [row for row in comparison["results"] if row["entity"] in members]
    assert member_rows and all(row["classification"] == UNMATCHED and row["unmatched_cause"] == "print_section_not_read"
                               for row in member_rows)
    others = [row for row in comparison["results"] if "values" in row and row["entity"] not in members]
    assert others and all(row["classification"] == WITHIN or row.get("resolution_cause") == "no_witness"
                          for row in others), [row for row in others if row["classification"] != WITHIN][:3]
    anchor = next(block for block in comparison["stated_bases"]["bases"] if block["name"] == "finite_anchor_stiffness")
    assert anchor["applied"] and any(row["entity"] in members for row in anchor["results"])  # listed there too


def test_the_command_stores_and_compares_a_capture_and_offline_compares_every_stored_one(tmp_path, monkeypatch,
                                                                                        capsys):
    entry = run_entry()
    config = fake_config(tmp_path)
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    monkeypatch.setattr(run_pilot, "solve_swbpipe", lambda runner, case: (
        {"invented": True}, {"payload": {"mechanics_envelope": {"results": []}}}, ""))
    runner = tmp_path / "runner"
    runner.write_text("invented", encoding="utf-8")
    folder = capture_folder(tmp_path, entry)
    argv = ["--benchmark", str(MANIFEST), "--series", HAND, "--case", "cpb-n-04", "--runner", str(runner)]
    assert run_pilot.main(argv + ["--window-capture", str(folder)], environ={}) == 0
    out = capsys.readouterr().out
    assert "window capture" in out and "route=window_capture; comparison written" in out
    assert run_pilot.main(argv + ["--offline"], environ={}) == 0
    root = config.exchange_root.resolve()
    assert len(list((root / "cases" / entry["run_id"] / "comparison").iterdir())) == 2
    with pytest.raises(SystemExit):
        run_pilot.main(argv[:4] + ["--window-capture", str(folder)], environ={})  # no --case
