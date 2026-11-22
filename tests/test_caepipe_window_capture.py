#!/usr/bin/env python3
"""The window print route (DEL-17-05; tools/caepipe_correlation/window_capture.py).

Every capture here is invented: an invented print in the documented layout, an invented saved model and run line, in
a temporary exchange folder. Nothing captured enters a fixture.
"""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import sys
from pathlib import Path

import pytest

# Reads its directory scans stand for, for CI selection (tools/ci/python_test_reads.py).
# Every directory scan here inspects temporary capture or exchange folders under tmp_path, outside the project.
CI_READS = ()

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external.benchmark import SUPPORT_STATE_NOT_DECLARED, UNMATCHED, WITHIN  # noqa: E402
from tests.test_caepipe_benchmark import csv_text, mirror_csv, solve  # noqa: E402
from tests.test_caepipe_correlation import fake_config  # noqa: E402
from tools.caepipe_correlation import benchmark_hand_entry, run_pilot  # noqa: E402
from tools.caepipe_correlation import window_capture as wc  # noqa: E402

MANIFEST = ROOT / "validation" / "caepipe_benchmark" / "manifest.json"
HAND = "hand-entry"


@pytest.fixture(autouse=True)
def _live_session_passed_through(monkeypatch):
    monkeypatch.setattr(run_pilot, "live_session", lambda config, emit=print: config, raising=False)


def run_entry(model_id: str = "cpb-n-04", series: str = HAND) -> dict:
    [entry] = run_pilot.load_benchmark(MANIFEST, [model_id], series)
    return entry


def declared(entry: dict) -> dict:
    return run_pilot.run_declarations(MANIFEST, entry["series_run"])[entry["model_id"]]


def never_solved(document):
    pytest.fail("solved")


def never_written(path, value):
    pytest.fail(f"written: {path}")


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


def stored(tmp_path: Path, entry: dict, **folder) -> tuple[Path, str, dict]:
    root = fake_config(tmp_path).exchange_root
    capture_id, record = wc.store_capture(entry, root, capture_folder(tmp_path, entry, **folder), identity=IDENTITY,
                                          declaration=declared(entry))
    return root, capture_id, record


def test_a_capture_is_stored_whole_by_content_once_and_its_folder_is_never_changed(tmp_path):
    entry = run_entry()
    root = fake_config(tmp_path).exchange_root
    folder = capture_folder(tmp_path, entry, extra={"screen-1.png": b"INVENTED IMAGE",
                                                    f"{entry['run_id']}.res": b"INVENTED RESULT FILE"})
    before = tree(folder)
    capture_id, record = wc.store_capture(entry, root, folder, identity=IDENTITY,
                                          declaration=declared(entry))
    assert tree(folder) == before  # copied, never moved or changed
    names = wc._names(entry)
    printed = (folder / names["print"]).read_bytes()
    assert capture_id == hashlib.sha256(printed).hexdigest()[:12]
    stored_dir = wc.capture_dir(root.resolve(), entry["run_id"], capture_id)
    assert tree(stored_dir) == {**before, wc.WINDOW_RECORD: (stored_dir / wc.WINDOW_RECORD).read_bytes()}
    roles = {name: item["role"] for name, item in record["files"].items()}
    assert roles[names["print"]] == "print" and roles[names["saved_model"]] == "saved_model"
    assert roles[names["instruction"]] == "instruction"
    assert roles["screen-1.png"] == roles[f"{entry['run_id']}.res"] == wc.KEPT_NOT_EXAMINED
    assert record["stored_at"] == IDENTITY and record["limits"] == wc.WINDOW_LIMITS
    assert record["window_instruction_sha256"] == entry["window_instruction"]["sha256"]
    assert record["checkout"]["writer_file"] == f"the writer's file at commit {IDENTITY['source_commit']} (the checkout was clean)"
    assert record["parse"]["status"] == "parsed"
    assert [item["title"] for item in record["parse"]["sections_not_read"]] == [
        "Invented unread section: Sustained (W+P)"]
    with pytest.raises(wc.CaptureRefused) as again:
        wc.store_capture(entry, root, folder, identity=IDENTITY,
                         declaration=declared(entry))
    assert again.value.route == "capture_stored"
    # A different print of the same run is a second capture beside the first.
    (folder / names["print"]).write_text(invented_print() + "\r\n", encoding="utf-8")
    second, _ = wc.store_capture(entry, root, folder, identity=IDENTITY,
                                 declaration=declared(entry))
    assert second != capture_id and wc.stored_captures(root, entry["run_id"]) == sorted([capture_id, second])
    assert not [path.name for path in stored_dir.parent.iterdir() if path.name.startswith(wc.INCOMING)]


def test_a_dirty_checkout_is_stated_as_the_working_trees_file_not_a_commits():
    assert "not that commit's" in wc.checkout({"source_commit": "1" * 40, "source_tree_clean": False})["writer_file"]
    assert "could not state" in wc.checkout({"source_commit": None, "source_tree_clean": None})["writer_file"]


@pytest.mark.parametrize("change, route", [
    ("mbf", "pack_file_differs"), ("instruction", "instruction_differs"), ("missing", "capture_incomplete"),
    ("no_instruction", "capture_incomplete"), ("link", "capture_folder"), ("directory", "capture_folder"),
    ("fifo", "capture_folder"), ("record_name", "reserved_name"), ("folder_link", "capture_folder")])
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
    elif change == "no_instruction":
        (folder / names["instruction"]).unlink()
    elif change == "link":
        os.symlink(folder / names["print"], folder / "linked.csv")
    elif change == "directory":
        (folder / "sub").mkdir()
    elif change == "fifo":
        os.mkfifo(folder / "pipe")
    elif change == "record_name":
        (folder / wc.WINDOW_RECORD).write_text("{}", encoding="utf-8")
    else:
        os.symlink(folder, tmp_path / "linked-folder")
        folder = tmp_path / "linked-folder"
    before = tree(root)
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.store_capture(entry, root, folder, identity=IDENTITY,
                         declaration=declared(entry))
    assert refused.value.route == route and tree(root) == before
    if change == "mbf":
        assert hashlib.sha256(b"AN OLDER FILE\r\n").hexdigest() in refused.value.reason


def test_an_interrupted_store_is_never_listed_and_never_bars_a_retry(tmp_path, monkeypatch):
    entry = run_entry()
    root = fake_config(tmp_path).exchange_root
    folder = capture_folder(tmp_path, entry)
    real = os.fdopen
    written = []

    def interrupted(descriptor, *args, **kwargs):  # the third file of the copy fails
        written.append(descriptor)
        if len(written) == 3:
            raise OSError("invented interruption")
        return real(descriptor, *args, **kwargs)
    monkeypatch.setattr(wc.os, "fdopen", interrupted)
    with pytest.raises(OSError):
        wc.store_capture(entry, root, folder, identity=IDENTITY,
                         declaration=declared(entry))
    monkeypatch.undo()
    assert wc.stored_captures(root, entry["run_id"]) == []
    window = root.resolve() / "cases" / entry["run_id"] / "caepipe_out" / "window"
    assert not [path for path in window.iterdir() if not path.name.startswith(wc.INCOMING)]
    # A partial directory left by a process that died is not listed either, and the retry stores the capture.
    (window / f"{wc.INCOMING}abandoned").mkdir()
    ((window / f"{wc.INCOMING}abandoned") / "partial.csv").write_bytes(b"PARTIAL")
    capture_id, _ = wc.store_capture(entry, root, folder, identity=IDENTITY,
                                     declaration=declared(entry))
    assert wc.stored_captures(root, entry["run_id"]) == [capture_id]


def test_a_store_that_loses_a_race_or_meets_another_print_under_its_id_is_refused_by_name(tmp_path, monkeypatch):
    entry = run_entry()
    root = fake_config(tmp_path).exchange_root
    folder = capture_folder(tmp_path, entry)
    first, _ = wc.store_capture(entry, root, folder, identity=IDENTITY,
                                declaration=declared(entry))
    target = wc.capture_dir(root.resolve(), entry["run_id"], first)
    # Another print whose digest begins with the same 12 hex digits (the stored record edited to say so).
    record = json.loads((target / wc.WINDOW_RECORD).read_text())
    record["files"][record["print"]]["sha256"] = first + "f" * 52
    os.chmod(target / wc.WINDOW_RECORD, 0o600)
    (target / wc.WINDOW_RECORD).write_text(json.dumps(record))
    with pytest.raises(wc.CaptureRefused) as collided:
        wc.store_capture(entry, root, folder, identity=IDENTITY,
                         declaration=declared(entry))
    assert collided.value.route == "capture_id_collision" and first + "f" * 52 in collided.value.reason
    # A concurrent store of the same print that lands first: this one ends in the named refusal, nothing replaced.
    other = tmp_path / "other"
    other.mkdir()
    folder2 = capture_folder(other, entry, print_text=invented_print() + "\r\n")
    real = os.rename

    def raced(source, destination):
        source = Path(source)
        twin = source.parent / "twin"
        shutil.copytree(source, twin)
        real(twin, destination)  # the other store renames into place first
        return real(source, destination)
    monkeypatch.setattr(wc.os, "rename", raced)
    with pytest.raises(wc.CaptureRefused) as lost:
        wc.store_capture(entry, root, folder2, identity=IDENTITY,
                         declaration=declared(entry))
    assert lost.value.route == "capture_stored"
    assert not [path.name for path in target.parent.iterdir() if path.name.startswith(wc.INCOMING)]


def test_a_batch_run_is_not_a_window_run(tmp_path):
    [entry] = run_pilot.load_benchmark(MANIFEST, ["cpb-g1-01"])
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.store_capture(entry, fake_config(tmp_path).exchange_root, tmp_path, identity=IDENTITY, declaration=None)
    assert refused.value.route == "not_a_window_run"


def test_a_dynamics_capture_is_stored_but_never_compared(tmp_path):
    """A tracked dynamics run (kind option_addition, declared screened: false; exit criterion 11 keeps its captures as
    references) is stored, and its comparison is refused by name with nothing solved or written."""
    entry = run_entry("cpb-dyn-c-n08-flat", "dynamics-comparable")
    assert entry["run_kind"] == "option_addition" and declared(entry)["screened"] is False
    root, capture_id, _ = stored(tmp_path, entry)
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=never_solved, write=never_written,
                           declaration=declared(entry))
    assert refused.value.route == "not_screened" and "headless modal" in refused.value.reason
    with pytest.raises(wc.CaptureRefused) as undeclared:
        wc.compare_capture(entry, root, capture_id, solve=never_solved, write=never_written, declaration=None)
    assert undeclared.value.route == "not_declared"


def test_a_print_that_does_not_parse_is_stored_and_its_comparison_is_refused_by_name(tmp_path, monkeypatch):
    entry = run_entry()
    root, capture_id, record = stored(tmp_path, entry, print_text="not a print\r\n")
    assert record["parse"]["status"] == "failed" and record["parse"]["reason"]
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=never_solved, write=never_written,
                           declaration=declared(entry))
    assert refused.value.route == "print_not_parsed"
    # A print that parsed when stored but not with the current parser is refused too.
    (tmp_path / "again").mkdir()
    root, capture_id, _ = stored(tmp_path / "again", entry)
    monkeypatch.setattr(wc, "parse_status", lambda data: ({"status": "failed", "reason": "invented parser change",
                                                           "sections_not_read": []}, None))
    with pytest.raises(wc.CaptureRefused) as now:
        wc.compare_capture(entry, root, capture_id, solve=never_solved, write=never_written,
                           declaration=declared(entry))
    assert now.value.route == "print_not_parsed" and "current parser" in now.value.reason


@pytest.mark.parametrize("change, route", [
    ("print", "print_digest_mismatch"), ("print_link", "stored_file_changed"), ("instruction", "stored_file_changed"),
    ("record_run", "record_run_differs"), ("record_print_path", "no_window_record")])
def test_a_stored_capture_changed_afterwards_is_refused_and_nothing_is_written(tmp_path, change, route):
    entry = run_entry()
    root, capture_id, record = stored(tmp_path, entry)
    target = wc.capture_dir(root.resolve(), entry["run_id"], capture_id)
    names = wc._names(entry)
    if change == "print":
        os.chmod(target / names["print"], 0o600)
        (target / names["print"]).write_text("changed", encoding="utf-8")
    elif change == "print_link":
        data = (target / names["print"]).read_bytes()
        (tmp_path / "elsewhere.csv").write_bytes(data)
        (target / names["print"]).unlink()
        os.symlink(tmp_path / "elsewhere.csv", target / names["print"])
    elif change == "instruction":
        os.chmod(target / names["instruction"], 0o600)
        (target / names["instruction"]).write_text("another instruction", encoding="utf-8")
    else:
        if change == "record_run":
            record["run_id"] = "cpb-n-01--stop-axial-closed"
        else:
            record["print"] = "../../../outside.csv"
        os.chmod(target / wc.WINDOW_RECORD, 0o600)
        (target / wc.WINDOW_RECORD).write_text(json.dumps(record), encoding="utf-8")
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=never_solved, write=never_written,
                           declaration=declared(entry))
    assert refused.value.route == route


def test_a_capture_made_from_another_file_or_instruction_than_the_runs_now_is_refused(tmp_path, monkeypatch):
    entry = run_entry()
    root, capture_id, _ = stored(tmp_path, entry)
    # The run now declares another instruction (a changed gap or load entered by hand, the base file unchanged).
    text = entry["window_instruction"]["text"] + "\nAn invented later step."
    later = {**entry, "window_instruction": {"text": text, "sha256": hashlib.sha256(text.encode()).hexdigest()}}
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(later, root, capture_id, solve=never_solved, write=never_written,
                           declaration=declared(entry))
    assert refused.value.route == "instruction_differs" and later["window_instruction"]["sha256"] in \
        refused.value.reason
    real = wc.run_mbf
    monkeypatch.setattr(wc, "run_mbf", lambda e: (real(e)[0], real(e)[1] + "* a later writer\r\n"))
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=never_solved, write=never_written,
                           declaration=declared(entry))
    assert refused.value.route == "pack_file_differs" and IDENTITY["source_commit"] in refused.value.reason


def rewrite_record(target: Path, record: dict) -> None:
    os.chmod(target / wc.WINDOW_RECORD, 0o600)
    (target / wc.WINDOW_RECORD).write_text(json.dumps(record), encoding="utf-8")


def test_a_run_not_declared_with_a_window_instruction_is_never_compared(tmp_path):
    """The window route compares only a window run, whatever capture id it is handed."""
    [batch] = run_pilot.load_benchmark(MANIFEST, ["cpb-g1-01"])
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(batch, fake_config(tmp_path).exchange_root, "0" * 12, solve=never_solved,
                           write=never_written, declaration={"screened": True})
    assert refused.value.route == "not_a_window_run"


def test_an_instruction_whose_text_is_not_its_declared_digest_is_refused(tmp_path):
    """The run's declared instruction digest is checked against the run's text, not only against the record."""
    entry = run_entry()
    root, capture_id, _ = stored(tmp_path, entry)
    crafted = {**entry, "window_instruction": {**entry["window_instruction"], "text": "another instruction"}}
    for instruction in (crafted["window_instruction"], {**entry["window_instruction"], "sha256": ""}):
        with pytest.raises(wc.CaptureRefused) as refused:
            wc.compare_capture({**entry, "window_instruction": instruction}, root, capture_id, solve=never_solved,
                               write=never_written, declaration=declared(entry))
        assert refused.value.route == "instruction_differs"


@pytest.mark.parametrize("change", ["file_not_a_mapping", "file_without_digest", "files_not_a_mapping",
                                    "print_role", "other_file_malformed"])
def test_a_malformed_window_record_is_refused_by_name(tmp_path, change):
    entry = run_entry()
    root, capture_id, record = stored(tmp_path, entry)
    target = wc.capture_dir(root.resolve(), entry["run_id"], capture_id)
    names = wc._names(entry)
    if change == "file_not_a_mapping":
        record["files"][names["print"]] = "invented"
    elif change == "file_without_digest":
        del record["files"][names["imported_mbf"]]["sha256"]
    elif change == "files_not_a_mapping":
        record["files"] = [names["print"], names["instruction"], names["imported_mbf"]]
    elif change == "print_role":
        record["files"][names["print"]]["role"] = wc.KEPT_NOT_EXAMINED
    else:
        record["files"][names["saved_model"]] = None
    rewrite_record(target, record)
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=never_solved, write=never_written,
                           declaration=declared(entry))
    assert refused.value.route == "no_window_record"


def test_a_capture_stored_under_another_id_than_its_prints_is_refused(tmp_path):
    """Every digest in the record agrees with the stored files, but the directory's capture id is not the print's."""
    entry = run_entry()
    root, capture_id, record = stored(tmp_path, entry)
    target = wc.capture_dir(root.resolve(), entry["run_id"], capture_id)
    other = "f" * 12 if capture_id != "f" * 12 else "e" * 12
    moved = target.parent / other
    os.rename(target, moved)
    rewrite_record(moved, {**record, "capture_id": other})
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, other, solve=never_solved, write=never_written, declaration=declared(entry))
    assert refused.value.route == "print_digest_mismatch"


@pytest.mark.parametrize("role, field", [("imported_mbf", "mbf_sha256"),
                                         ("instruction", "window_instruction_sha256")])
def test_a_record_whose_file_digests_disagree_with_its_own_fields_is_refused(tmp_path, role, field):
    """A crafted record: the stored file replaced and its listed digest updated, the top-level digest left as the
    run's. The stored bytes match the listing, and the top-level field matches the run; neither is enough."""
    entry = run_entry()
    root, capture_id, record = stored(tmp_path, entry)
    target = wc.capture_dir(root.resolve(), entry["run_id"], capture_id)
    name = wc._names(entry)[role]
    os.chmod(target / name, 0o600)
    (target / name).write_bytes(b"AN INVENTED OTHER FILE\r\n")
    record["files"][name]["sha256"] = hashlib.sha256(b"AN INVENTED OTHER FILE\r\n").hexdigest()
    assert record[field] != record["files"][name]["sha256"]
    rewrite_record(target, record)
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=never_solved, write=never_written,
                           declaration=declared(entry))
    assert refused.value.route == "no_window_record"
    # The listing and the field both rewritten to the other file: the stored bytes are then checked against the run.
    record[field] = record["files"][name]["sha256"]
    rewrite_record(target, record)
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=never_solved, write=never_written,
                           declaration=declared(entry))
    assert refused.value.route == ("pack_file_differs" if role == "imported_mbf" else "instruction_differs")


def test_a_linked_folder_below_the_exchange_root_lists_no_capture_outside_it(tmp_path):
    """A case folder that is a symbolic link to a directory outside the exchange folder, holding what looks like a
    complete capture: nothing in it is listed."""
    entry = run_entry()
    root = fake_config(tmp_path).exchange_root.resolve()
    outside = tmp_path / "outside"
    capture = outside / "caepipe_out" / "window" / ("a" * 12)
    capture.mkdir(parents=True)
    (capture / wc.WINDOW_RECORD).write_text("{}", encoding="utf-8")
    (root / "cases").mkdir(exist_ok=True)
    os.symlink(outside, root / "cases" / entry["run_id"])
    assert wc.stored_captures(root, entry["run_id"]) == []
    # The same under a linked caepipe_out folder.
    os.unlink(root / "cases" / entry["run_id"])
    (root / "cases" / entry["run_id"]).mkdir()
    os.symlink(outside / "caepipe_out", root / "cases" / entry["run_id"] / "caepipe_out")
    assert wc.stored_captures(root, entry["run_id"]) == []


def without_operating_class(entry: dict) -> dict:
    static = {case: cls for case, cls in entry["screening_classes"]["static"].items() if case != "operating"}
    return {**entry, "screening_classes": {**entry["screening_classes"], "static": static}}


def test_a_compared_case_with_no_declared_class_is_refused_before_anything_is_solved(tmp_path):
    entry = without_operating_class(run_entry())
    root, capture_id, _ = stored(tmp_path, entry)
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=never_solved, write=never_written,
                           declaration=declared(entry))
    assert refused.value.route == "class_not_declared" and "operating" in refused.value.reason


def test_the_record_never_screens_a_case_under_a_derived_class_in_place_of_a_missing_declared_one(tmp_path,
                                                                                                    monkeypatch):
    """Past the command's own check (removed here), the record itself refuses: no derived class stands in."""
    monkeypatch.setattr(wc, "compared_cases", lambda mbf: [])
    with pytest.raises(ValueError, match="operating"):
        mirror_compare(tmp_path, "cpb-n-04", alter=without_operating_class)


def test_a_nonlinear_case_with_no_declared_support_state_has_an_unmatched_row_saying_so(tmp_path):
    def no_states(entry):
        return {**entry, "hand_entry": {**entry["hand_entry"], "expected_status": {}}}
    *_, comparison, _ = mirror_compare(tmp_path, "cpb-n-04", alter=no_states)
    nonlinear = [status["case"] for status in comparison["load_cases"] if status["screening_class"] == "nonlinear"]
    rows = [row for row in comparison["results"] if row.get("unmatched_cause") == SUPPORT_STATE_NOT_DECLARED]
    assert nonlinear and sorted(row["case"] for row in rows) == sorted(nonlinear)
    assert all(row["kind"] == "support_state" and row["classification"] == UNMATCHED for row in rows)


def test_the_unread_member_text_cites_no_page_outside_its_register_boundary():
    """The Limit Stop page is admitted for support-type semantics only; where its results are displayed is outside
    that boundary, so the unread member is stated without it."""
    for text in (*wc.UNREAD_MEMBER.values(), wc.UNREAD_OTHER):
        assert "sstcae" not in text and "http" not in text
    assert wc.UNREAD_MEMBER["nonlinear"] == ("the parser does not read the printed loads or support state of a "
                                             "nonlinear member entered at the window")


@pytest.mark.parametrize("stale, route", [("mbf", "pack_file_differs"), ("instruction_text", "instruction_differs"),
                                          ("instruction_sha", "instruction_differs")])
def test_a_stale_manifest_declaration_is_refused_in_store_and_in_replay(tmp_path, stale, route):
    """The run and the writer agree; only the selected manifest run declaration is stale. Neither the store nor the
    replay passes over it."""
    entry = run_entry()
    good = declared(entry)
    bad = json.loads(json.dumps(good))
    if stale == "mbf":
        bad["mbf"]["sha256"] = "0" * 64
    elif stale == "instruction_text":
        bad["window_instruction"]["text"] += "\nAn invented later step."
    else:
        bad["window_instruction"]["sha256"] = "0" * 64
    root = fake_config(tmp_path).exchange_root
    before = tree(root)
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.store_capture(entry, root, capture_folder(tmp_path, entry), identity=IDENTITY, declaration=bad)
    assert refused.value.route == route and tree(root) == before
    (tmp_path / "again").mkdir()
    root, capture_id, _ = stored(tmp_path / "again", entry)
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=never_solved, write=never_written, declaration=bad)
    assert refused.value.route == route


def test_a_manifest_whose_window_instruction_is_not_the_runs_is_refused_at_load(monkeypatch):
    real = run_pilot.benchmark_models.series_document

    def drifted(document, series):
        out = real(document, series)
        if out.get("window_instruction"):
            text = out["window_instruction"]["text"] + "\nAn invented later step."
            out["window_instruction"] = {"text": text, "sha256": hashlib.sha256(text.encode("utf-8")).hexdigest()}
        return out
    monkeypatch.setattr(run_pilot.benchmark_models, "series_document", drifted)
    with pytest.raises(ValueError, match="window instruction"):
        run_pilot.load_benchmark(MANIFEST, ["cpb-n-04"], HAND)


def test_a_capture_the_command_refuses_makes_no_case_folder(tmp_path, monkeypatch, capsys):
    entry = run_entry()
    config = fake_config(tmp_path)
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    monkeypatch.setattr(run_pilot, "solve_swbpipe", lambda runner, case: pytest.fail("solved"))
    runner = tmp_path / "runner"
    runner.write_text("invented", encoding="utf-8")
    folder = capture_folder(tmp_path, entry)
    (folder / wc._names(entry)["saved_model"]).unlink()
    argv = ["--benchmark", str(MANIFEST), "--series", HAND, "--case", "cpb-n-04", "--runner", str(runner)]
    assert run_pilot.main(argv + ["--window-capture", str(folder)], environ={}) == 1
    assert "route=capture_incomplete" in capsys.readouterr().out
    assert not (config.exchange_root.resolve() / "cases" / entry["run_id"]).exists()


def static_classes(value):
    def alter(entry):
        if value == "absent":
            return {key: item for key, item in entry.items() if key != "screening_classes"}
        static = {**entry["screening_classes"]["static"], "operating": value}
        return {**entry, "screening_classes": {**entry["screening_classes"], "static": static}}
    return alter


@pytest.mark.parametrize("value", ["absent", None, ["nonlinear"], "linear"])
def test_a_window_case_without_a_valid_declared_static_class_is_refused_before_solving(tmp_path, value):
    entry = static_classes(value)(run_entry())
    root, capture_id, _ = stored(tmp_path, entry)
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=never_solved, write=never_written,
                           declaration=declared(entry))
    assert refused.value.route == "class_not_declared"


@pytest.mark.parametrize("value", [None, ["nonlinear"]])
def test_the_record_refuses_a_declared_class_that_is_not_a_static_class_name(tmp_path, monkeypatch, value):
    """Past the command's own check (removed here): a declared class of None or of another type is refused by the
    record itself, never replaced by a derived class."""
    monkeypatch.setattr(wc, "compared_cases", lambda mbf: [])
    with pytest.raises(ValueError, match="operating"):
        mirror_compare(tmp_path, "cpb-n-04", alter=static_classes(value))


@pytest.mark.parametrize("change", ["saved_model_name", "saved_model_role", "run_line_missing", "run_line_text",
                                    "parse", "checkout"])
def test_a_window_record_without_its_saved_model_run_line_parse_or_checkout_is_refused(tmp_path, change):
    entry = run_entry()
    root, capture_id, record = stored(tmp_path, entry)
    target = wc.capture_dir(root.resolve(), entry["run_id"], capture_id)
    names = wc._names(entry)
    if change == "saved_model_name":
        record["saved_model"] = "another.mod"
    elif change == "saved_model_role":
        record["files"][names["saved_model"]]["role"] = wc.KEPT_NOT_EXAMINED
    elif change == "run_line_missing":
        del record["files"][names["run_line"]]
    elif change == "run_line_text":
        record["run_line_text"] = None
    else:
        record[change] = [change]
    rewrite_record(target, record)
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(entry, root, capture_id, solve=never_solved, write=never_written,
                           declaration=declared(entry))
    assert refused.value.route == "no_window_record"


@pytest.mark.parametrize("name", ["../outside.csv", "sub/print.csv", "sub\\print.csv", "print\0.csv", ".", "..",
                                  wc.WINDOW_RECORD])
def test_a_declared_capture_file_name_that_is_not_a_single_name_is_refused_before_opening(tmp_path, name):
    entry = run_entry()
    root, capture_id, _ = stored(tmp_path, entry)
    run = {**entry["hand_entry"]["run"], "expected_print_file": name}
    renamed = {**entry, "hand_entry": {**entry["hand_entry"], "run": run}}
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.compare_capture(renamed, root, capture_id, solve=never_solved, write=never_written,
                           declaration=declared(entry))
    assert refused.value.route == "capture_name_refused"
    (tmp_path / "again").mkdir()
    with pytest.raises(wc.CaptureRefused) as refused:
        wc.store_capture(renamed, root, capture_folder(tmp_path / "again", entry), identity=IDENTITY,
                         declaration=declared(entry))
    assert refused.value.route == "capture_name_refused"


def test_a_file_read_from_an_open_directory_is_a_single_name_never_a_path(tmp_path):
    (tmp_path / "inner").mkdir()
    (tmp_path / "secret.txt").write_bytes(b"OUTSIDE")
    descriptor = os.open(tmp_path / "inner", os.O_RDONLY | os.O_DIRECTORY)
    try:
        for name in ("../secret.txt", "..", ""):
            with pytest.raises(wc.CaptureRefused):
                wc._read_regular(descriptor, name, "invented_route")
    finally:
        os.close(descriptor)


def mirror_compare(tmp_path: Path, model_id: str, monkeypatch=None, *, raising_bases: bool = False, alter=None):
    entry = run_entry(model_id)
    if alter is not None:
        entry = alter(entry)
    root = fake_config(tmp_path).exchange_root
    model = benchmark_hand_entry.hand_entry_model(entry)
    payload = solve(model)
    _, text = mirror_csv(entry, payload)
    capture_id, _ = wc.store_capture(entry, root, capture_folder(tmp_path, entry, print_text=text), identity=IDENTITY,
                                     declaration=declared(entry))
    written, solved = {}, []

    def fake_solve(document):
        solved.append(document)
        return solve(document["model"]), "", {"solve_id": f"invented-{len(solved)}"}

    def write(path, value):
        assert path not in written
        written[path] = value
        return ""
    if raising_bases:
        def raising(*args, **kwargs):
            raise RuntimeError("invented failure of the bases")
        monkeypatch.setattr(wc.stated_bases, "compare_bases", raising)
        with pytest.raises(RuntimeError):
            wc.compare_capture(entry, root, capture_id, solve=fake_solve, write=write, declaration=declared(entry))
        return entry, written, solved, capture_id
    path, comparison = wc.compare_capture(entry, root, capture_id, solve=fake_solve, write=write,
                                          declaration=declared(entry), identity=IDENTITY)
    return entry, written, solved, capture_id, path, comparison, model


@pytest.mark.parametrize("model_id", ["cpb-n-01", "cpb-n-04", "cpb-h-01"])
def test_a_mirror_capture_compares_the_base_under_the_declared_class_and_lists_the_member_unmatched(tmp_path,
                                                                                                   model_id):
    """SWBPIPE's side is the model with the member, under the declared inexact inputs and screening class; the print
    mirrors SWBPIPE's own solve through the base file's identity map. The member's reaction rows, and a nonlinear
    case's required support-state match, are unmatched by name (the printed section is not read), and each declared
    closed form of the member stays beside SWBPIPE's value."""
    entry, written, solved, capture_id, path, comparison, model = mirror_compare(tmp_path, model_id)
    assert solved[0]["model"] == model and written[path] is comparison
    assert solved[0]["inexact_inputs"] == entry["declared_inexact_inputs"] and solved[0]["inexact_inputs"]
    declared_classes = entry["screening_classes"]["static"]
    for status in comparison["load_cases"]:
        assert status["screening_class"] == declared_classes[status["case"]]
        assert status["screening_class_basis"] == "declared by the run"
        if status["screening_class"] == "nonlinear":
            assert status["support_state"]["match"] == "not_available"
            assert status["support_state"]["expected"] == {
                support: states[status["case"]] for support, states in entry["hand_entry"]["expected_status"].items()}
    assert comparison["caepipe_print"]["route"] == wc.WINDOW_ROUTE and comparison["caepipe_print"]["capture_id"] == \
        capture_id
    window = comparison["window_capture"]
    assert window["limits"] == wc.WINDOW_LIMITS and window["declaration"]["screened"] is False
    members = {member["id"] for member in entry["hand_entry"]["member"]}
    assert set(window["members_not_compared"]) == members
    member_rows = [row for row in comparison["results"] if row["entity"] in members]
    assert member_rows and all(row["classification"] == UNMATCHED and row["unmatched_cause"] == "print_section_not_read"
                               for row in member_rows)
    states = [row for row in member_rows if row["kind"] == "support_state"]
    assert bool(states) == (model_id.startswith("cpb-n"))
    member_forms = [form for form in entry["third_values"] if form.get("support") in members]
    for form in member_forms:
        [row] = [row for row in member_rows if row["kind"] == "support_force" and row["case"] == form["load_case"]
                 and row["component"] == form["component"]]
        assert row["values"]["closed_form"] == form["value"] and row["closed_form"]["closed_form_vs_swbpipe"] == "agree"
    assert bool(member_forms) == (model_id == "cpb-n-01")
    others = [row for row in comparison["results"] if "values" in row and row["entity"] not in members]
    assert others and all(row["classification"] == WITHIN or row.get("resolution_cause") == "no_witness"
                          for row in others), [row for row in others if row["classification"] != WITHIN][:3]
    # The stated bases are written after the plain record, beside it.
    bases = written[path.parent / wc.STATED_BASES_RECORD]
    assert list(written) == [path, path.parent / wc.STATED_BASES_RECORD]
    assert comparison["stated_bases"]["record"] == wc.STATED_BASES_RECORD and "bases" not in comparison["stated_bases"]
    anchor = next(block for block in bases["bases"] if block["name"] == "finite_anchor_stiffness")
    assert anchor["applied"] and any(row["entity"] in members for row in anchor["results"])  # listed there too


def test_no_basis_can_cost_the_plain_comparison(tmp_path, monkeypatch):
    entry, written, solved, _ = mirror_compare(tmp_path, "cpb-n-04", monkeypatch, raising_bases=True)
    [path] = written
    assert path.name == "benchmark_comparison.json" and written[path]["results"]


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
    root = config.exchange_root.resolve()
    before = tree(root)
    # The batch selection holds offline: an out-of-range batch does nothing.
    assert run_pilot.main(argv + ["--offline", "--batch-size", "1", "--batch", "2"], environ={}) == 2
    assert "--batch 2 is not one of the 1 batches" in capsys.readouterr().out and tree(root) == before
    assert run_pilot.main(argv + ["--offline", "--batch-size", "1", "--batch", "1"], environ={}) == 0
    comparisons = list((root / "cases" / entry["run_id"] / "comparison").iterdir())
    assert len(comparisons) == 2 and all((path / wc.STATED_BASES_RECORD).is_file() for path in comparisons)
    with pytest.raises(SystemExit):
        run_pilot.main(argv[:4] + ["--window-capture", str(folder)], environ={})  # no --case


def test_the_command_stores_a_dynamics_capture_and_announces_no_comparison(tmp_path, monkeypatch, capsys):
    entry = run_entry("cpb-dyn-c-n08-flat", "dynamics-comparable")
    config = fake_config(tmp_path)
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    monkeypatch.setattr(run_pilot, "solve_swbpipe", lambda runner, case: pytest.fail("solved"))
    runner = tmp_path / "runner"
    runner.write_text("invented", encoding="utf-8")
    argv = ["--benchmark", str(MANIFEST), "--series", "dynamics-comparable", "--case", "cpb-dyn-c-n08-flat",
            "--runner", str(runner)]
    assert run_pilot.main(argv + ["--window-capture", str(capture_folder(tmp_path, entry))], environ={}) == 1
    out = capsys.readouterr().out
    assert "window capture" in out and "route=not_screened" in out and "comparison written" not in out
    assert not (config.exchange_root.resolve() / "cases" / entry["run_id"] / "comparison").exists() or not any(
        (config.exchange_root.resolve() / "cases" / entry["run_id"] / "comparison").iterdir())
