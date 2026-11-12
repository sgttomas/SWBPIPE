#!/usr/bin/env python3
"""Focused tests for the CAEPIPE sitting pack (tools/caepipe_correlation/sitting_pack.py, DEL-17-05): the plan and
its order, the pack's determinism and work-tree refusal, refused cases, variants, format samples and observations,
the intake of returned results by each route, sealed captures and the two-condition unseal, and the end-to-end path
prepare, ingest, offline screen. Invented inputs and invented result text only; nothing here runs the external
program or reads its output."""

from __future__ import annotations

import builtins
import contextlib
import hashlib
import io
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from typing import Callable

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import canonical_model_hash, print_csv  # noqa: E402
from core.handoff.caepipe_external import harness  # noqa: E402
from tools.caepipe_correlation import run_pilot, sitting_pack  # noqa: E402
from tests.test_caepipe_benchmark import MANIFEST, benchmark_manifest, fake_benchmark, invented_print  # noqa: E402,F401


def plan_file(tmp_path: Path, parts: list[dict], name: str = "plan.json") -> Path:
    path = tmp_path / name
    defaults = {"buys": "invented test part", "title": "Invented part", "estimate": None}
    path.write_text(json.dumps({"document_kind": sitting_pack.PLAN_KIND,
                                "parts": [defaults | part for part in parts]}), encoding="utf-8")
    return path


def build(manifest: Path, plan: Path | None = None, observations: Path | None = None, cases=None) -> dict:
    return sitting_pack.build_pack(manifest, sitting_pack.load_plan(plan),
                                   sitting_pack.load_observations(observations), cases)


def index_of(files: dict) -> dict:
    return json.loads(files["pack_index.json"])


def by_id(index: dict) -> dict:
    return {item["item_id"]: item for item in index["items"]}


def write(pack: Path, files: dict) -> Path:
    return sitting_pack.write_pack(pack, files)


# -- prepare ----------------------------------------------------------------------------------------------------------
def test_the_default_plan_gives_the_dry_run_and_samples_and_lists_empty_parts_with_reasons():
    index = index_of(build(MANIFEST))
    parts = {part["id"]: part for part in index["parts"]}
    assert [part["id"] for part in index["parts"]] == ["part-0", "part-1", "part-2", "part-3", "part-4"]
    assert parts["part-0"]["items"] == ["cpb-g1-05", "cpb-g6-02", "cpb-g8-01", "fs-limit-stop-gap",
                                        "fs-variable-spring-hanger", "fs-modal-straight-span"]
    for part_id in ("part-1", "part-2", "part-3", "part-4"):
        assert parts[part_id]["items"] == [] and parts[part_id]["empty_because"], part_id
    items = by_id(index)
    assert [items[i]["order"] for i in parts["part-0"]["items"]] == [1, 2, 3, 4, 5, 6]
    sample = items["fs-limit-stop-gap"]
    assert sample["item_kind"] == "format_sample" and sample["identity_file"] is None
    assert sample["mbf"]["copied_byte_for_byte"] is True
    modal = items["fs-modal-straight-span"]
    assert modal["run_variant_only"] and modal["variants"][0]["kind"] == "instruction"


def test_the_pack_is_deterministic_and_holds_the_writer_mbf_its_identity_file_and_settings(tmp_path):
    first, second = build(MANIFEST), build(MANIFEST)
    assert first == second
    write(tmp_path / "a", first)
    write(tmp_path / "b", second)
    names = sorted(str(p.relative_to(tmp_path / "a")) for p in (tmp_path / "a").rglob("*") if p.is_file())
    assert names == sorted(first)
    for name in names:
        assert (tmp_path / "a" / name).read_bytes() == (tmp_path / "b" / name).read_bytes(), name
    item = by_id(index_of(first))["cpb-g1-05"]
    model = next(m for m in run_pilot.load_benchmark(MANIFEST) if m["model_id"] == "cpb-g1-05")
    text = run_pilot.render_benchmark_mbf(model).text
    assert first["cases/cpb-g1-05/cpb-g1-05.mbf"] == text.encode("utf-8")
    assert item["mbf"]["sha256"] == hashlib.sha256(text.encode("utf-8")).hexdigest()
    identity = json.loads(first["cases/cpb-g1-05/cpb-g1-05.swbpipe-ids.json"])
    assert identity["mbf_sha256"] == item["mbf"]["sha256"]
    assert item["settings"]["piping_code"] == "B311-80" and item["settings"]["b31j"] == "off (OPCODE 0,0,0)"
    assert item["settings"]["gravity_m_per_s2"] == 9.80665 and item["settings"]["vertical_axis"] == "Y"
    assert item["settings"]["units"] == "SI" and item["settings"]["reference_temperature_c"] == "20"
    assert item["inexact_inputs"] == model["inexact_inputs"] and item["layout_row_count"] == 4
    assert json.loads(first["cases/cpb-g1-05/case.json"]) == item
    authored = (run_pilot.AUTHORED_DIR / "cpa-14-limit-stop-gap.mbf").read_bytes()
    assert first["samples/fs-limit-stop-gap/cpa-14-limit-stop-gap.mbf"] == authored
    assert not any(name.startswith("samples/fs-limit-stop-gap/") and name.endswith(".swbpipe-ids.json")
                   for name in first)


def test_prepare_refuses_a_directory_inside_a_git_work_tree_and_a_non_empty_one(tmp_path, capsys):
    repo = tmp_path / "repo"
    subprocess.run(["git", "init", "-q", str(repo)], check=True)
    assert sitting_pack.main(["prepare", str(repo / "pack")]) == 2
    assert "inside a Git work tree" in capsys.readouterr().err
    assert not (repo / "pack").exists()
    full = tmp_path / "full"
    full.mkdir()
    (full / "returned.csv").write_text("kept", encoding="utf-8")
    assert sitting_pack.main(["prepare", str(full)]) == 2
    assert "not an empty directory" in capsys.readouterr().err
    assert (full / "returned.csv").read_text(encoding="utf-8") == "kept"


def manifest_with_a_refused_model(tmp_path: Path) -> Path:
    path = benchmark_manifest(tmp_path / "set", count=3)
    manifest = json.loads(path.read_text(encoding="utf-8"))
    entry = manifest["models"][1]
    model_path = path.parent / entry["file"]
    document = json.loads(model_path.read_text(encoding="utf-8"))
    document["model"]["pressure_contract"] = {"version": "invented", "mode": "invented"}
    data = json.dumps(document).encode("utf-8")
    model_path.write_bytes(data)
    entry.update(file_sha256=hashlib.sha256(data).hexdigest(), model_hash=canonical_model_hash(document["model"]))
    path.write_text(json.dumps(manifest), encoding="utf-8")
    return path


def test_a_case_the_writer_refuses_is_listed_with_its_codes_and_given_no_folder(tmp_path):
    manifest = manifest_with_a_refused_model(tmp_path)
    files = build(manifest, plan_file(tmp_path, [{"id": "all", "kind": "benchmark", "select": {"all": True}}]))
    items = by_id(index_of(files))
    refused = items["cpb-g1-02"]
    assert refused["status"] == "refused" and "MBF-SI-PRESSURE-UNSUPPORTED" in refused["refusal_codes"]
    # The model carrying pressure_contract is selected by its member and its contract is carried in the index.
    assert refused["pressure_contract"] == {"version": "invented", "mode": "invented"}
    assert not any(name.startswith("cases/cpb-g1-02/") for name in files)
    assert "not to run (the writer refused it)" in files["RUN_SHEET.md"].decode()
    assert items["cpb-g1-01"]["status"] == items["cpb-g1-03"]["status"] == "written"


def test_parts_order_by_family_series_adjacent_smaller_first_and_place_each_case_once(tmp_path):
    manifest = benchmark_manifest(tmp_path / "set", count=60)
    plan = plan_file(tmp_path, [
        {"id": "first", "kind": "dry_run", "select": {"model_ids": ["cpb-g3-07", "cpb-x-99"]}},
        {"id": "nothing", "kind": "benchmark", "select": {"families": ["G99"]}},
        {"id": "rest", "kind": "benchmark", "select": {"families": ["G2", "G3"], "series": ["tee-ratio"]}},
    ])
    index = index_of(build(manifest, plan))
    parts = {part["id"]: part for part in index["parts"]}
    assert parts["first"]["items"] == ["cpb-g3-07"]
    assert parts["nothing"]["items"] == [] and "no case of the manifest matches" in parts["nothing"]["empty_because"]
    rest = parts["rest"]["items"]
    assert parts["rest"]["already_placed"] == {"cpb-g3-07": "first"} and "cpb-g3-07" not in rest
    assert rest[:16] == [f"cpb-g2-{n:02d}" for n in range(1, 17)]  # each k-series pair adjacent, in member order
    g3 = [item for item in rest if item.startswith("cpb-g3")]
    rows = {item["item_id"]: item["layout_row_count"] for item in index["items"]}
    assert [rows[item] for item in g3] == sorted(rows[item] for item in g3)  # smaller models first
    assert rest[-4:] == ["cpb-g5-01", "cpb-g5-02", "cpb-g5-03", "cpb-g5-04"]


def test_a_substitution_variant_is_its_own_mbf_without_an_identity_file_and_needs_its_line_once(tmp_path):
    manifest = benchmark_manifest(tmp_path / "set", count=1)
    plan = plan_file(tmp_path, [{"id": "p", "kind": "benchmark", "select": {"all": True}, "variants": [
        {"label": "opt-on", "kind": "substitution", "old_line": "0,0,0", "new_line": "1,0,0"},
        {"label": "by-hand", "kind": "instruction", "instruction": "Switch the invented option on."}]}])
    files = build(manifest, plan)
    item = by_id(index_of(files))["cpb-g1-01"]
    substitution, instruction = item["variants"]
    base = files["cases/cpb-g1-01/cpb-g1-01.mbf"].decode()
    changed = files["cases/cpb-g1-01/cpb-g1-01.opt-on.mbf"]
    assert changed.decode() == base.replace("OPCODE\r\n0,0,0\r\n", "OPCODE\r\n1,0,0\r\n")
    assert substitution["mbf"]["sha256"] == hashlib.sha256(changed).hexdigest() != item["mbf"]["sha256"]
    assert substitution["writer_output"] is False and substitution["identity_file_of_base"] == "cpb-g1-01.swbpipe-ids.json"
    assert "cases/cpb-g1-01/cpb-g1-01.opt-on.swbpipe-ids.json" not in files
    assert instruction["mbf"]["sha256"] == item["mbf"]["sha256"] and instruction["result_file"] == \
        "cpb-g1-01.by-hand.print.csv"
    absent = plan_file(tmp_path, [{"id": "p", "kind": "benchmark", "select": {"all": True}, "variants": [
        {"label": "x", "kind": "substitution", "old_line": "NO SUCH LINE", "new_line": "1"}]}], "absent.json")
    with pytest.raises(sitting_pack.PackError, match="occurs 0 times"):
        build(manifest, absent)


def test_the_instructions_put_the_harness_route_first_and_quote_run_pilots_own_help():
    text = build(MANIFEST)["INSTRUCTIONS.md"].decode()
    assert text.index("## 1. The harness route") < text.index("## 2. The hand route")
    assert "Do not run sealed captures, format samples or variants with this command." in text
    for source, url in sitting_pack.SOURCES.items():
        assert f"{source}, {url}" in text
    assert "trial" not in text.lower()
    out = io.StringIO()
    with contextlib.redirect_stdout(out), pytest.raises(SystemExit):
        run_pilot.main(["--help"])
    help_text = re.sub(r"\s+", " ", out.getvalue())
    for _, line in sitting_pack.HARNESS_OPTION_HELP:
        assert line in help_text, line


# -- a returned pack ------------------------------------------------------------------------------------------------
def guest_json(sha: str, stem: str, status: str = "csv_from_batch") -> str:
    return ("Invented banner line\r\n" + json.dumps({"schema": "openpipestress.caepipe_guest_round_trip.v1",
                                                     "mbf_sha256": sha.upper(), "status": status,
                                                     "csv_path": f"C:\\invented\\{stem}.csv",
                                                     "batch": {"exit_code": 0}}) + "\r\n")


def fill_run_line(folder: Path, route: str) -> None:
    path = folder / "run_line.txt"
    text = path.read_text(encoding="utf-8").replace("date: ", "date: 2026-11-21").replace(
        "(guest_script, manual_batch or manual_print): ", f"(guest_script, manual_batch or manual_print): {route}", 1)
    path.write_text(text, encoding="utf-8")


@pytest.fixture
def sitting(fake_benchmark, tmp_path):
    """A prepared pack of invented cases (outside any work tree), the exchange folder and the offline command."""
    state = fake_benchmark
    plan = plan_file(tmp_path, [
        {"id": "dry", "kind": "dry_run", "select": {"model_ids": ["cpb-g1-01", "cpb-g1-02"]},
         "case_overrides": {"cpb-g1-02": {"variants": [
             {"label": "by-hand", "kind": "instruction", "instruction": "Switch the invented option on."}]}}},
        {"id": "sealed", "kind": "sealed_capture", "sealed_reason": "invented: SWBPIPE side not ready",
         "select": {"model_ids": ["cpb-g1-03"]}},
    ])
    pack = write(tmp_path / "pack", build(state["manifest"], plan))
    state.update(pack=pack, root=state["config"].exchange_root.resolve(),
                 index=json.loads((pack / "pack_index.json").read_text(encoding="utf-8")))
    state["offline"] = ["--benchmark", str(state["manifest"]), "--runner", state["runner"], "--offline"]
    return state


def test_prepare_ingest_offline_screen_for_a_guest_script_result(sitting, capsys):
    folder = sitting["pack"] / "cases" / "cpb-g1-01"
    item = by_id(sitting["index"])["cpb-g1-01"]
    (folder / "cpb-g1-01.csv").write_text(invented_print(), encoding="utf-8")
    (folder / "cpb-g1-01.guest.json").write_text(guest_json(item["mbf"]["sha256"], "cpb-g1-01"), encoding="utf-8")
    fill_run_line(folder, "guest_script")
    # The dry run's check writes nothing and reads the print in place.
    before = sorted(p.relative_to(sitting["root"]) for p in sitting["root"].rglob("*"))
    assert sitting_pack.ingest_pack(sitting["pack"], None, check=True, selected=["cpb-g1-01"])
    assert sorted(p.relative_to(sitting["root"]) for p in sitting["root"].rglob("*")) == before
    checked = capsys.readouterr().out
    assert "cpb-g1-01: benchmark case; checked; route=csv_from_batch; binding=guest_reported_hash" in checked
    assert "0 error diagnostic(s)" in checked
    assert sitting_pack.ingest_pack(sitting["pack"], sitting["root"], selected=["cpb-g1-01"])
    out = sitting["root"] / "cases" / "cpb-g1-01" / "caepipe_out"
    record = json.loads((out / "run_record.json").read_text(encoding="utf-8"))
    assert record["route"] == "csv_from_batch" and record["sitting"]["binding"] == "guest_reported_hash"
    assert record["mbf"]["sha256"] == record["guest_mbf_sha256"] == item["mbf"]["sha256"]
    intake = json.loads((out / sitting_pack.INTAKE_RECORD).read_text(encoding="utf-8"))
    assert intake["bound"] is True and intake["document_kind"] == sitting_pack.INTAKE_KIND
    assert (out / "model.csv").read_text(encoding="utf-8") == invented_print()
    # Ingesting the same pack again changes nothing.
    assert sitting_pack.ingest_pack(sitting["pack"], sitting["root"], selected=["cpb-g1-01"])
    assert "already_placed" in capsys.readouterr().out
    assert run_pilot.main(sitting["offline"] + ["--case", "cpb-g1-01"], environ={}) == 0
    assert "cpb-g1-01: route=csv_from_batch; benchmark record written" in capsys.readouterr().out
    manifest = json.loads((out.parent / "case_manifest.json").read_text(encoding="utf-8"))
    assert manifest["outcome"] == "recorded"


def test_a_result_obtained_by_hand_is_recorded_under_its_own_route_and_not_compared(sitting, capsys):
    folder = sitting["pack"] / "cases" / "cpb-g1-02"
    (folder / "cpb-g1-02.print.csv").write_text(invented_print(), encoding="utf-8")
    (folder / "cpb-g1-02.by-hand.print.csv").write_text(invented_print(sustained_uy="-5"), encoding="utf-8")
    fill_run_line(folder, "manual_print")
    assert sitting_pack.ingest_pack(sitting["pack"], sitting["root"], selected=["cpb-g1-02"])
    case = sitting["root"] / "cases" / "cpb-g1-02"
    record = json.loads((case / "caepipe_out" / "run_record.json").read_text(encoding="utf-8"))
    assert record["route"] == "sitting_manual_print" and record["guest_mbf_sha256"] is None
    assert record["sitting"]["binding"] == "person_attested"
    assert run_pilot.bound_csv(sitting["root"], {"root": case}, record, record["mbf"]["sha256"])[0] is None
    variant = json.loads((case / "variants" / "by-hand" / sitting_pack.STORED_RECORD).read_text(encoding="utf-8"))
    assert list(variant["files"]) == ["cpb-g1-02.by-hand.print.csv"]
    assert run_pilot.main(sitting["offline"] + ["--case", "cpb-g1-02"], environ={}) == 1
    assert "cpb-g1-02: route=unbound_output" in capsys.readouterr().out


def test_ingest_refuses_a_case_whose_mbf_changed_and_a_guest_hash_that_differs(sitting, capsys):
    folder = sitting["pack"] / "cases" / "cpb-g1-01"
    mbf = folder / "cpb-g1-01.mbf"
    mbf.write_bytes(mbf.read_bytes().replace(b"OPTEMP\r\n20", b"OPTEMP\r\n21"))
    (folder / "cpb-g1-01.csv").write_text(invented_print(), encoding="utf-8")
    fill_run_line(folder, "manual_batch")
    assert not sitting_pack.ingest_pack(sitting["pack"], sitting["root"], selected=["cpb-g1-01"])
    assert "mbf_changed" in capsys.readouterr().out
    assert not (sitting["root"] / "cases" / "cpb-g1-01").exists()
    other = sitting["pack"] / "cases" / "cpb-g1-02"
    (other / "cpb-g1-02.csv").write_text(invented_print(), encoding="utf-8")
    (other / "cpb-g1-02.guest.json").write_text(guest_json("0" * 64, "cpb-g1-02"), encoding="utf-8")
    assert sitting_pack.ingest_pack(sitting["pack"], sitting["root"], selected=["cpb-g1-02"])
    out = sitting["root"] / "cases" / "cpb-g1-02" / "caepipe_out"
    assert json.loads((out / "run_record.json").read_text(encoding="utf-8"))["route"] == "input_hash_mismatch"
    assert not (out / "model.csv").exists()


# -- sealed captures ------------------------------------------------------------------------------------------------
LOUD = b"\x00\xff\xfeNOT A PRINT \x80\x81 Caepipe? Displacements\x00"
GUEST_NAME, PRINT_NAME = "cpb-g1-03.guest.json", "cpb-g1-03.csv"


class Reads:
    """Every open of a watched file, with whether it happened inside a hashing pass and whether the unseal record
    existed then; and every call of a parser entry point (each fails the test at once)."""

    def __init__(self):
        self.log: list[dict] = []
        self.parser_calls: list[str] = []
        self.hashing = 0
        self.watched: Callable[[Path], bool] = lambda path: False
        self.record: Path | None = None
        self.forbid_parsers = True  # any parser call on a watched file (or on text) fails at once

    def content_reads(self) -> list[dict]:
        return [entry for entry in self.log if not entry["hashing"]]


@pytest.fixture
def reads(monkeypatch):
    """Instrument the file-open path (io.open, which pathlib uses, and builtins.open) and the parser entry points."""
    state = Reads()
    real_open = io.open

    def watched_open(file, mode="r", *args, **kwargs):
        if isinstance(file, (str, os.PathLike)) and "r" in mode and "+" not in mode:
            path = Path(os.fspath(file)).absolute()
            if state.watched(path):
                state.log.append({"path": path, "hashing": state.hashing > 0,
                                  "record_exists": bool(state.record and state.record.exists())})
        return real_open(file, mode, *args, **kwargs)

    monkeypatch.setattr(io, "open", watched_open)
    monkeypatch.setattr(builtins, "open", watched_open)
    for name in ("stream_sha256", "_copy_hashed"):
        real = getattr(sitting_pack, name)

        def hashing(*args, _real=real, **kwargs):
            state.hashing += 1
            try:
                return _real(*args, **kwargs)
            finally:
                state.hashing -= 1
        monkeypatch.setattr(sitting_pack, name, hashing)

    def parser_entry(name, real):
        def call(*args, **kwargs):
            paths = [Path(os.fspath(a)).absolute() for a in args if isinstance(a, os.PathLike)]
            if any(state.watched(path) for path in paths) or not paths:
                state.parser_calls.append({"name": name, "record_exists": bool(state.record
                                                                               and state.record.exists())})
                if state.forbid_parsers:
                    raise AssertionError(f"{name} was called on a watched file or on text")
            return real(*args, **kwargs)
        return call
    for module, name in ((print_csv, "read_caepipe_print_csv"), (print_csv, "parse_caepipe_print_csv"),
                         (harness, "_print_layout_problem"), (sitting_pack, "last_json_object"),
                         (sitting_pack, "parse_run_line"), (harness, "last_json_object")):
        monkeypatch.setattr(module, name, parser_entry(name, getattr(module, name)))
    return state


def return_sealed(sitting, content: bytes = LOUD) -> tuple[Path, dict]:
    folder = sitting["pack"] / "cases" / "cpb-g1-03"
    item = by_id(sitting["index"])["cpb-g1-03"]
    (folder / PRINT_NAME).write_bytes(content)
    (folder / GUEST_NAME).write_text(guest_json(item["mbf"]["sha256"], "cpb-g1-03"), encoding="utf-8")
    fill_run_line(folder, "guest_script")
    return folder, item


def watch_capture(reads: Reads, *folders: Path) -> None:
    names = {PRINT_NAME, GUEST_NAME, "run_line.txt"}
    roots = [folder.absolute() for folder in folders]
    reads.watched = lambda path: path.name in names and any(path.is_relative_to(r) for r in roots)


def test_a_sealed_capture_is_hashed_once_and_stored_under_its_names_and_never_read_or_parsed(sitting, reads, capsys):
    folder, item = return_sealed(sitting)
    root = sitting["root"]
    sealed = root / "sealed" / "cpb-g1-03"
    watch_capture(reads, folder, root)
    status_before = git_status()
    assert sitting_pack.ingest_pack(sitting["pack"], root, selected=["cpb-g1-03"])
    assert "cpb-g1-03: sealed capture; sealed" in capsys.readouterr().out
    assert reads.parser_calls == [] and reads.content_reads() == []
    # One hashing pass per returned file (the copy that hashes), nothing more.
    per_file = {}
    for entry in reads.log:
        per_file[entry["path"].name] = per_file.get(entry["path"].name, 0) + 1
    assert per_file == {PRINT_NAME: 1, GUEST_NAME: 1, "run_line.txt": 1}
    record = json.loads((sealed / sitting_pack.CAPTURE_RECORD).read_text(encoding="utf-8"))
    assert record["status"] == "sealed" and record["reason"] == "invented: SWBPIPE side not ready"
    assert record["files"][PRINT_NAME] == {"bytes": len(LOUD), "sha256": hashlib.sha256(LOUD).hexdigest(),
                                           "role": "returned"}
    guest_bytes = (folder / GUEST_NAME).read_bytes()
    assert record["files"][GUEST_NAME]["sha256"] == hashlib.sha256(guest_bytes).hexdigest()
    assert record["files"]["cpb-g1-03.mbf"]["role"] == "pack_input"
    assert (sealed / PRINT_NAME).read_bytes() == LOUD  # collected under the name it was printed with
    # The records live in the exchange folder, outside every Git work tree; the repository is untouched.
    assert sealed.is_relative_to(root) and harness._inside_git(root, os.environ) is False
    assert git_status() == status_before
    assert not (root / "cases" / "cpb-g1-03").exists()
    reads.log.clear()
    assert sitting_pack.ingest_pack(sitting["pack"], root, selected=["cpb-g1-03"])
    assert "already_sealed" in capsys.readouterr().out and reads.content_reads() == []
    (folder / PRINT_NAME).write_bytes(LOUD + b"x")
    assert not sitting_pack.ingest_pack(sitting["pack"], root, selected=["cpb-g1-03"])
    assert "never replaced" in capsys.readouterr().out
    assert (sealed / PRINT_NAME).read_bytes() == LOUD and reads.parser_calls == []


def test_the_offline_screen_and_the_regression_check_pass_over_a_sealed_capture(sitting, reads, capsys):
    return_sealed(sitting)
    root = sitting["root"]
    assert sitting_pack.ingest_pack(sitting["pack"], root, selected=["cpb-g1-03"])
    watch_capture(reads, root / "sealed")
    reads.log.clear()
    # The offline screen of every model: the sealed case has no stored run, so nothing is compared or read.
    run_pilot.main(sitting["offline"], environ={})
    assert "cpb-g1-03: route=unbound_output" in capsys.readouterr().out
    # The private regression check's walk and the private MBF import check's walk of the exchange folder.
    from tests import test_caepipe_private_correlation as private
    assert private.stored_cases(sitting["config"]) == []
    from tests import test_caepipe_private_mbf_import as private_import
    private_import.exchange_root({harness.ENV_VAR: str(root)})
    assert reads.log == [] and reads.parser_calls == []


def swbpipe_result(tmp_path: Path, model_id: str, model: dict, solved: bool = True, tag: str = "") -> Path:
    directory = tmp_path / f"swbpipe-{model_id}-{solved}{tag}"
    directory.mkdir()
    (directory / "request.json").write_text(json.dumps({"request": run_pilot.runner_request(model_id),
                                                        "solve": {"preview_model": {"model": model}}, "tag": tag}),
                                            encoding="utf-8")
    status = "MECHANICS_SOLVED" if solved else "MECHANICS_REFUSED"
    (directory / "result.json").write_text(json.dumps({"payload": {"mechanics_envelope": {"status": {
        "mechanics": status}}}, "tag": tag}), encoding="utf-8")
    return directory


def test_unsealing_refuses_each_missing_condition_by_name_and_admits_both(sitting, tmp_path):
    return_sealed(sitting, invented_print().encode("utf-8"))
    root = sitting["root"]
    assert sitting_pack.ingest_pack(sitting["pack"], root, selected=["cpb-g1-03"])
    models = {m["model_id"]: m["model"] for m in run_pilot.load_benchmark(sitting["manifest"])}
    good = swbpipe_result(tmp_path, "cpb-g1-03", models["cpb-g1-03"])
    calls = []

    def unseal(**kwargs):
        return sitting_pack.unseal_capture(root, "cpb-g1-03", intake=lambda *a, **k: calls.append(1) or {},
                                           **({"tolerance_class": "exact", "swbpipe_result": good} | kwargs))

    assert sitting_pack.TOLERANCE_CLASSES_WITH_VALUE == ("exact", "named_inexact", "nonlinear", "modal")
    assert unseal(tolerance_class=None)["reason"] == "tolerance_class_missing"
    assert unseal(tolerance_class="spectrum")["reason"] == "tolerance_class_without_value"
    assert unseal(tolerance_class="invented")["reason"] == "tolerance_class_unknown"
    assert unseal(swbpipe_result=None)["reason"] == "swbpipe_result_missing"
    assert unseal(swbpipe_result=tmp_path / "absent")["reason"] == "swbpipe_result_missing"
    other = swbpipe_result(tmp_path, "cpb-g1-03", models["cpb-g1-04"])
    assert unseal(swbpipe_result=other)["reason"] == "swbpipe_result_mismatch"
    unsolved = swbpipe_result(tmp_path, "cpb-g1-03", models["cpb-g1-03"], solved=False)
    assert unseal(swbpipe_result=unsolved)["reason"] == "swbpipe_result_not_solved"
    sealed = root / "sealed" / "cpb-g1-03"
    assert calls == [] and sorted(p.name for p in sealed.iterdir() if p.name.startswith(("unseal", "kept"))) == []
    assert not (root / "cases" / "cpb-g1-03" / "caepipe_out" / "run_record.json").exists()

    for modal in ("modal", "nonlinear", "named_inexact"):
        assert modal in sitting_pack.TOLERANCE_CLASSES_WITH_VALUE
    outcome = sitting_pack.unseal_capture(root, "cpb-g1-03", tolerance_class="exact", swbpipe_result=good)
    assert outcome["status"] == "unsealed" and outcome["placement"]["route"] == "csv_from_batch"
    record = json.loads((sealed / sitting_pack.UNSEAL_RECORD).read_text(encoding="utf-8"))
    capture = json.loads((sealed / sitting_pack.CAPTURE_RECORD).read_text(encoding="utf-8"))
    result_bytes = (good / "result.json").read_bytes()
    assert record["tolerance_class"] == "exact" and record["capture_sha256"] == capture["capture_sha256"]
    assert record["model_hash"] == capture["model_hash"] == canonical_model_hash(models["cpb-g1-03"])
    assert record["swbpipe_result"]["result_sha256"] == hashlib.sha256(result_bytes).hexdigest()
    assert (sealed / sitting_pack.KEPT_RESULT).read_bytes() == result_bytes
    # A second attempt, with a different (later) SWBPIPE result, replaces neither the record nor the kept result.
    kept_record = (sealed / sitting_pack.UNSEAL_RECORD).read_bytes()
    later = swbpipe_result(tmp_path, "cpb-g1-03", models["cpb-g1-03"], tag="later")
    assert unseal(swbpipe_result=later)["reason"] == "already_unsealed"
    assert (sealed / sitting_pack.UNSEAL_RECORD).read_bytes() == kept_record
    assert (sealed / sitting_pack.KEPT_RESULT).read_bytes() == result_bytes and calls == []


def test_the_unseal_record_exists_before_any_byte_of_the_capture_is_read_but_to_hash_it(sitting, tmp_path, reads):
    return_sealed(sitting, invented_print().encode("utf-8"))
    root = sitting["root"]
    assert sitting_pack.ingest_pack(sitting["pack"], root, selected=["cpb-g1-03"])
    sealed = root / "sealed" / "cpb-g1-03"
    models = {m["model_id"]: m["model"] for m in run_pilot.load_benchmark(sitting["manifest"])}
    good = swbpipe_result(tmp_path, "cpb-g1-03", models["cpb-g1-03"])
    watch_capture(reads, sealed)
    reads.record = sealed / sitting_pack.UNSEAL_RECORD
    reads.forbid_parsers = False  # unsealing may read and parse, but only once the record exists
    reads.log.clear()
    outcome = sitting_pack.unseal_capture(root, "cpb-g1-03", tolerance_class="modal", swbpipe_result=good)
    assert outcome["status"] == "unsealed" and outcome["placement"]["route"] == "csv_from_batch"
    content = reads.content_reads()
    assert content, "the placement read the capture"
    first = reads.log.index(content[0])
    # Before the first content read: only hashing passes, and the record already exists at that read.
    assert first > 0 and all(entry["hashing"] for entry in reads.log[:first])
    assert not any(entry["record_exists"] for entry in reads.log[:1])
    assert all(entry["record_exists"] for entry in content)
    assert reads.parser_calls and all(call["record_exists"] for call in reads.parser_calls)


def test_the_unseal_command_refuses_without_a_class(sitting, capsys, monkeypatch):
    monkeypatch.setattr(sitting_pack, "resolve_harness_config", lambda environ, require_vm_tools: sitting["config"])
    assert sitting_pack.main(["unseal", "--case", "cpb-g1-03"], environ={}) == 1
    assert "tolerance_class_missing" in capsys.readouterr().out


# -- format samples and observations --------------------------------------------------------------------------------
def test_format_samples_and_observations_are_stored_and_hashed_never_parsed(fake_benchmark, tmp_path, reads,
                                                                         capsys):
    state = fake_benchmark
    (tmp_path / "obs").mkdir()
    (tmp_path / "obs" / "invented-input.mbf").write_bytes(b"INVENTED\r\n")
    observations = tmp_path / "obs" / "observations.json"
    observations.write_text(json.dumps({"document_kind": sitting_pack.OBSERVATIONS_KIND, "items": [
        {"id": "obs-b", "rank": 2, "title": "Second", "action": "Invented action.", "bring_back": ["b.txt"]},
        {"id": "obs-a", "rank": 1, "title": "First", "action": "Invented action.", "input": "invented-input.mbf",
         "bring_back": ["a.txt", "a2.txt"]}]}), encoding="utf-8")
    plan = plan_file(tmp_path, [
        {"id": "dry", "kind": "dry_run", "select": {}, "format_samples": [
            {"id": "fs-a", "title": "Invented", "purpose": "invented sections", "authored": "cpa-18-variable-spring-hanger"},
            {"id": "fs-b", "title": "Invented", "purpose": "invented sections", "model_id": "cpb-g1-01",
             "variant": {"label": "modal-3", "kind": "instruction", "instruction": "Invented change."}}]},
        {"id": "obs", "kind": "observations"}])
    pack = write(tmp_path / "pack", build(state["manifest"], plan, observations))
    index = json.loads((pack / "pack_index.json").read_text(encoding="utf-8"))
    assert [part["items"] for part in index["parts"]] == [["fs-a", "fs-b"], ["obs-a", "obs-b"]]
    items = by_id(index)
    assert items["fs-b"]["outside_benchmark_set"] and items["fs-b"]["mbf"]["file"] == "fs-b.mbf"
    assert (pack / "observations" / "obs-a" / "invented-input.mbf").read_bytes() == b"INVENTED\r\n"
    returned_files = {pack / "samples" / "fs-a" / "cpa-18-variable-spring-hanger.csv": LOUD,
                      pack / "samples" / "fs-b" / "fs-b.modal-3.print.csv": LOUD + b"b",
                      pack / "observations" / "obs-a" / "a.txt": LOUD + b"a"}
    for path, data in returned_files.items():
        path.write_bytes(data)
    names = {path.name for path in returned_files}
    root = state["config"].exchange_root.resolve()
    reads.watched = lambda path: path.name in names
    assert sitting_pack.ingest_pack(pack, root)
    assert reads.parser_calls == [] and reads.content_reads() == []
    assert len(reads.log) == len(names) * 1  # one hashing pass each
    out = capsys.readouterr().out
    assert "obs-b: format observation; not_returned" in out and "not returned: a2.txt" in out
    stored = json.loads((root / "format_samples" / "fs-a" / sitting_pack.STORED_RECORD).read_text(encoding="utf-8"))
    assert stored["files"]["cpa-18-variable-spring-hanger.csv"] == {"bytes": len(LOUD),
                                                                   "sha256": hashlib.sha256(LOUD).hexdigest()}
    assert (root / "format_samples" / "fs-b" / "fs-b.modal-3.print.csv").read_bytes() == LOUD + b"b"
    observed = json.loads((root / "observations" / "obs-a" / sitting_pack.STORED_RECORD).read_text(encoding="utf-8"))
    assert list(observed["files"]) == ["a.txt"] and observed["missing"] == ["a2.txt"]
    assert not (root / "observations" / "obs-b").exists()


def test_a_format_sample_cannot_take_a_benchmark_id_and_the_index_decides_what_is_one(fake_benchmark, tmp_path,
                                                                                      capsys):
    state = fake_benchmark
    clash = plan_file(tmp_path, [{"id": "dry", "kind": "dry_run", "select": {}, "format_samples": [
        {"id": "cpb-g1-02", "title": "Invented", "purpose": "invented", "model_id": "cpb-g1-01",
         "variant": {"label": "v", "kind": "instruction", "instruction": "Invented change."}}]}], "clash.json")
    with pytest.raises(sitting_pack.PackError, match="names a model of the benchmark manifest"):
        build(state["manifest"], clash)
    plan = plan_file(tmp_path, [
        {"id": "dry", "kind": "dry_run", "select": {"model_ids": ["cpb-g1-01"]}, "format_samples": [
            {"id": "fs-a", "title": "Invented", "purpose": "invented", "authored": "cpa-14-limit-stop-gap"}]}])
    pack = write(tmp_path / "pack", build(state["manifest"], plan))
    index_path = pack / "pack_index.json"
    index = json.loads(index_path.read_text(encoding="utf-8"))
    for item in index["items"]:
        if item["item_id"] == "fs-a":
            item["item_id"], item["folder"] = "cpb-g1-01", "samples/cpb-g1-01"  # an edited index
    index_path.write_text(json.dumps(index), encoding="utf-8")
    (pack / "samples" / "cpb-g1-01").mkdir()
    (pack / "samples" / "cpb-g1-01" / "x.csv").write_bytes(LOUD)
    root = state["config"].exchange_root.resolve()
    assert not sitting_pack.ingest_pack(pack, root)
    assert "it is not treated as one" in capsys.readouterr().out
    assert not (root / "format_samples").exists()


def git_status() -> str:
    return subprocess.run(["git", "-C", str(ROOT), "status", "--porcelain", "--untracked-files=all"],
                          capture_output=True, text=True, check=True).stdout
