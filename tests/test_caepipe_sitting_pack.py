#!/usr/bin/env python3
"""Focused tests for the CAEPIPE sitting pack (tools/caepipe_correlation/sitting_pack.py, DEL-17-05): the plan and
its order, the pack's determinism, exclusive creation and work-tree refusal, refused cases, substitution variants,
format samples bound to their declaration, the authoritative index, the intake of returned results by each route,
sealed captures that nothing reads, create-only and contained writes in the exchange folder, interruption, and the
driver's refusals. Invented inputs and invented result text only; nothing here runs the external program or reads
its output."""

from __future__ import annotations

import builtins
import contextlib
import fcntl
import hashlib
import io
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
from typing import Callable

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import core.handoff.caepipe_external as caepipe_external  # noqa: E402
from core.handoff.caepipe_external import canonical_model_hash, print_csv  # noqa: E402
from core.handoff.caepipe_external import harness  # noqa: E402
from tools.caepipe_correlation import run_pilot, sitting_pack  # noqa: E402
from tests.test_caepipe_benchmark import MANIFEST, benchmark_manifest, fake_benchmark, invented_print  # noqa: E402,F401


REAL_CLASSIFY = sitting_pack.classify_case  # before any test patches it


def plan_file(tmp_path: Path, parts: list[dict], name: str = "plan.json") -> Path:
    path = tmp_path / name
    defaults = {"buys": "invented test part", "title": "Invented part", "estimate": None}
    path.write_text(json.dumps({"document_kind": sitting_pack.PLAN_KIND,
                                "parts": [defaults | part for part in parts]}), encoding="utf-8")
    return path


def build(manifest: Path, plan: Path | None = None, observations: Path | None = None, cases=None,
          samples: Path | None = None) -> dict:
    return sitting_pack.build_pack(manifest, sitting_pack.load_plan(plan), sitting_pack.load_observations(observations),
                                   cases, samples=sitting_pack.load_samples(samples))


def index_of(files: dict) -> dict:
    return json.loads(files["pack_index.json"])


def by_id(index: dict) -> dict:
    return {item["item_id"]: item for item in index["items"]}


def write(pack: Path, files: dict) -> Path:
    return sitting_pack.write_pack(pack, files, pack.with_name(pack.name + ".authority"))


PAUSE = [False]  # the instrumented reads ignore the test's own digests


def tree_digest(root: Path) -> dict[str, str]:
    """Every entry under `root` by relative path: a file's SHA-256 (computed here), a directory or link marked."""
    PAUSE[0] = True
    try:
        return _tree_digest(root)
    finally:
        PAUSE[0] = False


def _tree_digest(root: Path) -> dict[str, str]:
    digest = {}
    for path in sorted(root.rglob("*")):
        key = str(path.relative_to(root))
        if path.is_symlink():
            digest[key] = "link:" + os.readlink(path)
        elif path.is_dir():
            digest[key] = "dir"
        else:
            digest[key] = hashlib.sha256(path.read_bytes()).hexdigest()
    return digest


def git_status() -> str:
    return subprocess.run(["git", "-C", str(ROOT), "status", "--porcelain", "--untracked-files=all"],
                          capture_output=True, text=True, check=True).stdout


# -- invented format samples ----------------------------------------------------------------------------------------
INVENTED_SAMPLE = ("INVENTED SAMPLE\r\nInvented format sample\r\nVERSION\r\n15.10\r\nOPTIONS\r\nHGRA,SI\r\nOPTEMP\r\n20\r\n"
                   "MATERIAL\r\nM1,7833,0.3,1,1,CS,invented steel\r\nM1,20,200000,1.2e-05\r\nPIPE\r\n"
                   "P1,168.3,7.11,0,0,0,0,0,0\r\nLOADS\r\nL1,20,0,0\r\nLAYOUT\r\nF10,KA,MM1,PP1,CL=L1\r\nT20,X2500\r\n"
                   ).encode("utf-8")


def sample_declaration(directory: Path, entries: list[tuple[str, str, bytes]], *, sha: dict | None = None) -> Path:
    """A declaration beside invented sample files, each with its invented-provenance sidecar."""
    directory.mkdir(parents=True, exist_ok=True)
    samples = []
    for sample_id, name, data in entries:
        (directory / name).write_bytes(data)
        (directory / (Path(name).stem + ".provenance.json")).write_text(json.dumps(
            {"produced_by_caepipe": False, "origin": "invented for a test"}), encoding="utf-8")
        samples.append({"id": sample_id, "file": name, "sha256": (sha or {}).get(sample_id) or
                        hashlib.sha256(data).hexdigest(), "title": "Invented sample",
                        "prints": "the invented sections", "instruction": "Make the one invented change."})
    path = directory / "declaration.json"
    path.write_text(json.dumps({"document_kind": sitting_pack.SAMPLES_KIND, "samples": samples}), encoding="utf-8")
    return path


# -- prepare ----------------------------------------------------------------------------------------------------------
def test_the_default_plan_gives_the_dry_run_without_samples_and_lists_empty_parts_with_reasons():
    index = index_of(build(MANIFEST))
    parts = {part["id"]: part for part in index["parts"]}
    assert [part["id"] for part in index["parts"]] == ["part-0", "part-1", "part-2", "part-3", "part-4"]
    assert parts["part-0"]["items"] == ["cpb-g1-05", "cpb-g6-02", "cpb-g8-01"]
    assert not any(item["item_kind"] == "format_sample" for item in index["items"])
    for part_id in ("part-1", "part-2", "part-3", "part-4"):
        assert parts[part_id]["items"] == [] and parts[part_id]["empty_because"], part_id
    sitting_pack.validate_index(index)
    assert json.loads(sitting_pack.DEFAULT_SAMPLES.read_text(encoding="utf-8"))["samples"] == []


def test_the_pack_is_deterministic_holds_the_writer_mbf_and_its_authority(tmp_path):
    first, second = build(MANIFEST), build(MANIFEST)
    assert first == second
    write(tmp_path / "a", first)
    write(tmp_path / "b", second)
    names = sorted(str(p.relative_to(tmp_path / "a")) for p in (tmp_path / "a").rglob("*") if p.is_file())
    assert names == sorted(first)
    for name in names:
        assert (tmp_path / "a" / name).read_bytes() == (tmp_path / "b" / name).read_bytes(), name
    authority = tmp_path / "a.authority"
    index_bytes = (authority / "pack_index.json").read_bytes()
    assert index_bytes == first["pack_index.json"]
    assert (authority / "pack_index.sha256").read_text().split()[0] == hashlib.sha256(index_bytes).hexdigest()
    item = by_id(index_of(first))["cpb-g1-05"]
    model = next(m for m in run_pilot.load_benchmark(MANIFEST) if m["model_id"] == "cpb-g1-05")
    text = run_pilot.render_benchmark_mbf(model).text
    assert first["cases/cpb-g1-05/cpb-g1-05.mbf"] == text.encode("utf-8")
    identity = json.loads(first["cases/cpb-g1-05/cpb-g1-05.swbpipe-ids.json"])
    assert identity["mbf_sha256"] == item["mbf"]["sha256"] == hashlib.sha256(text.encode("utf-8")).hexdigest()
    assert item["settings"]["piping_code"] == "B311-80" and item["settings"]["gravity_m_per_s2"] == 9.80665
    assert json.loads(first["cases/cpb-g1-05/case.json"]) == item


def test_prepare_reserves_new_folders_only_and_refuses_a_work_tree(tmp_path, capsys):
    repo = tmp_path / "repo"
    subprocess.run(["git", "init", "-q", str(repo)], check=True)
    assert sitting_pack.main(["prepare", str(repo / "pack")]) == 2
    assert "inside a Git work tree" in capsys.readouterr().err and not (repo / "pack").exists()
    existing = tmp_path / "existing"
    existing.mkdir()  # even an empty folder is refused: the folder is reserved by creating it
    assert sitting_pack.main(["prepare", str(existing)]) == 2
    assert "already exists" in capsys.readouterr().err and list(existing.iterdir()) == []
    (tmp_path / "fresh.authority").mkdir()
    (tmp_path / "fresh.authority" / "kept").write_bytes(b"kept")
    assert sitting_pack.main(["prepare", str(tmp_path / "fresh")]) == 2
    assert "already exists" in capsys.readouterr().err
    assert (tmp_path / "fresh.authority" / "kept").read_bytes() == b"kept"
    assert list((tmp_path / "fresh").iterdir()) == []  # reserved, left empty, never deleted


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
    refused = by_id(index_of(files))["cpb-g1-02"]
    assert refused["status"] == "refused" and "MBF-SI-PRESSURE-UNSUPPORTED" in refused["refusal_codes"]
    assert not any(name.startswith("cases/cpb-g1-02/") for name in files)


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
    assert "no case of the manifest matches" in parts["nothing"]["empty_because"]
    rest = parts["rest"]["items"]
    assert parts["rest"]["already_placed"] == {"cpb-g3-07": "first"}
    assert rest[:16] == [f"cpb-g2-{n:02d}" for n in range(1, 17)]
    rows = {item["item_id"]: item["layout_row_count"] for item in index["items"]}
    g3 = [item for item in rest if item.startswith("cpb-g3")]
    assert [rows[item] for item in g3] == sorted(rows[item] for item in g3)
    assert rest[-4:] == ["cpb-g5-01", "cpb-g5-02", "cpb-g5-03", "cpb-g5-04"]


SETTINGS_TEXT = "TITLE\r\nVERSION\r\n15.10\r\nOPPRES\r\n0,0,0,1,0\r\nOPMISC\r\n9,8,7\r\nMATERIAL\r\nM1,20,1\r\nLAYOUT\r\nF10,KA\r\nT20,X3000\r\n"


def test_a_substitution_changes_only_the_data_line_of_an_allowed_options_record(tmp_path):
    text, old = sitting_pack.substitute_setting(SETTINGS_TEXT, "OPPRES", "0,1,0,1,0")
    assert old == "0,0,0,1,0" and text == SETTINGS_TEXT.replace("OPPRES\r\n0,0,0,1,0", "OPPRES\r\n0,1,0,1,0")
    assert sitting_pack.substitute_setting(SETTINGS_TEXT, "OPMISC", "9,8,6")[1] == "9,8,7"
    for heading in ("LAYOUT", "MATERIAL", "PIPE", "LOADS", "OPTIONS"):  # topology and data records: never
        with pytest.raises(sitting_pack.PackError):
            sitting_pack.substitute_setting(SETTINGS_TEXT, heading, "T20,X1")
        with pytest.raises(sitting_pack.PackError, match="heading must be one of"):
            sitting_pack.check_variant({"label": "x", "kind": "substitution", "heading": heading, "new_line": "1"}, "v")
    with pytest.raises(sitting_pack.PackError, match="occurs 2 times"):
        sitting_pack.substitute_setting(SETTINGS_TEXT.replace("OPMISC", "OPPRES"), "OPPRES", "1")
    repeated = SETTINGS_TEXT.replace("M1,20,1", "0,0,0,1,0")
    with pytest.raises(sitting_pack.PackError, match="data line under OPPRES occurs 2 times"):
        sitting_pack.substitute_setting(repeated, "OPPRES", "1")
    with pytest.raises(sitting_pack.PackError, match="one data line"):
        sitting_pack.substitute_setting(SETTINGS_TEXT, "OPPRES", "LAYOUT")
    with pytest.raises(sitting_pack.PackError, match="unknown member"):
        sitting_pack.check_variant({"label": "x", "kind": "substitution", "heading": "OPPRES", "new_line": "1",
                                    "old_line": "T20,X3000"}, "v")
    files: dict = {}
    entry = sitting_pack._variant_entry({"label": "on", "kind": "substitution", "heading": "OPPRES",
                                         "new_line": "0,1,0,1,0"}, "inv", SETTINGS_TEXT, "0" * 64, files, "cases/inv",
                                        "inv.swbpipe-ids.json")
    assert entry["mbf"]["sha256"] == hashlib.sha256(files["cases/inv/inv.on.mbf"]).hexdigest()
    assert "cases/inv/inv.on.swbpipe-ids.json" not in files and entry["identity_file_of_base"] == "inv.swbpipe-ids.json"
    assert "no node, element or support line changed" in entry["identity"]
    # The benchmark models carry no pressure options record: a plan substitution on them is refused.
    manifest = benchmark_manifest(tmp_path / "set", count=1)
    plan = plan_file(tmp_path, [{"id": "p", "kind": "benchmark", "select": {"all": True}, "variants": [
        {"label": "on", "kind": "substitution", "heading": "OPPRES", "new_line": "0,1,0,1,0"}]}])
    with pytest.raises(sitting_pack.PackError, match="OPPRES occurs 0 times"):
        build(manifest, plan)


def test_the_instructions_put_the_harness_route_first_and_say_what_cannot_be_done_yet():
    files = build(MANIFEST)
    text = files["INSTRUCTIONS.md"].decode()
    flat = re.sub(r"\s+", " ", text.replace("> ", ""))
    assert text.index("## 1. The harness route") < text.index("## 2. The hand route")
    assert "start no live run that clears an output folder against the exchange folder at all" in flat
    assert flat.count("(measured 12 November 2026)") == 2
    assert "Never leave the pressure options line blank." in flat
    assert "a dry-run case is never run through a route that clears a stored result" in flat
    assert "cannot be screened yet: no existing reader uses the run that intake names" in flat
    assert ("There is no unsealing command: every route that parses or screens a result refuses a sealed "
            "capture") in flat
    assert "### 2.9 Rehearsing the hand route" in text and "Invented files never go into the real exchange" in flat
    for absent in ("no public page", "screening class", "--replace", "tolerance class", "unseal --"):
        assert absent not in text, absent
    assert "--authority" in text and "trial" not in text.lower()
    for source, url in sitting_pack.SOURCES.items():
        assert f"{source}, {url}" in text
    out = io.StringIO()
    with contextlib.redirect_stdout(out), pytest.raises(SystemExit):
        run_pilot.main(["--help"])
    help_text = re.sub(r"\s+", " ", out.getvalue())
    for _, line in sitting_pack.HARNESS_OPTION_HELP:
        assert line in help_text, line
    sheet = files["RUN_SHEET.md"].decode()
    assert "--case" not in sheet[sheet.index("## part-0"):sheet.index("## part-1")]


def test_a_declared_sample_is_copied_byte_for_byte_and_a_benchmark_model_never_is_one(fake_benchmark, tmp_path):
    manifest = fake_benchmark["manifest"]
    declaration = sample_declaration(tmp_path / "samples", [("fs-a", "fs-a.mbf", INVENTED_SAMPLE)])
    plan = plan_file(tmp_path, [{"id": "dry", "kind": "dry_run", "select": {}, "format_samples": [{"id": "fs-a"}]}])
    files = build(manifest, plan, samples=declaration)
    item = by_id(index_of(files))["fs-a"]
    assert files["samples/fs-a/fs-a.mbf"] == INVENTED_SAMPLE and item["identity_file"] is None
    assert item["outside_benchmark_set"] and item["designation"]["sha256"] == hashlib.sha256(INVENTED_SAMPLE).hexdigest()
    # Refusals: a benchmark model under any id or alias, an undeclared sample, a changed file, a plan that names more.
    document = next(m for m in run_pilot.load_benchmark(manifest) if m["model_id"] == "cpb-g1-02")
    writer_mbf = run_pilot.render_benchmark_mbf(document).text.encode("utf-8")
    model_file = (manifest.parent / "models" / "cpb-g1-02.json").read_bytes()
    for sample_id, name, data, message in (
            ("cpb-g1-02", "x.mbf", INVENTED_SAMPLE, "names a model of the benchmark manifest"),
            ("fs-alias", "alias.mbf", writer_mbf, "writer's MBF of benchmark model cpb-g1-02"),
            ("fs-model", "model.mbf", model_file, "model file of benchmark model cpb-g1-02")):
        directory = tmp_path / f"decl-{sample_id}"
        refused = sample_declaration(directory, [(sample_id, name, data)])
        plan = plan_file(directory, [{"id": "d", "kind": "dry_run", "select": {}, "format_samples": [{"id": sample_id}]}])
        with pytest.raises(sitting_pack.PackError, match=message):
            build(manifest, plan, samples=refused)
    plan = plan_file(tmp_path, [{"id": "d", "kind": "dry_run", "select": {}, "format_samples": [{"id": "fs-b"}]}], "u.json")
    with pytest.raises(sitting_pack.PackError, match="not in the format-sample declaration"):
        build(manifest, plan, samples=declaration)
    changed = sample_declaration(tmp_path / "changed", [("fs-a", "fs-a.mbf", INVENTED_SAMPLE)], sha={"fs-a": "0" * 64})
    plan = plan_file(tmp_path, [{"id": "d", "kind": "dry_run", "select": {}, "format_samples": [{"id": "fs-a"}]}], "c.json")
    with pytest.raises(sitting_pack.PackError, match="does not have the SHA-256 the declaration binds"):
        build(manifest, plan, samples=changed)
    aliased = plan_file(tmp_path, [{"id": "d", "kind": "dry_run", "select": {}, "format_samples": [
        {"id": "fs-a", "model_id": "cpb-g1-02"}]}], "a.json")
    with pytest.raises(sitting_pack.PackError, match="by its id only"):
        sitting_pack.load_plan(aliased)


def test_the_request_the_index_binds_is_the_one_run_pilot_sends():
    document = next(m for m in run_pilot.load_benchmark(MANIFEST) if m["model_id"] == "cpb-g1-05")
    sent = run_pilot.solve_swbpipe("/usr/bin/true", {"case_id": "cpb-g1-05", "model": document["model"],
                                                     "source_location": "validation/caepipe_benchmark/models/"
                                                                        "cpb-g1-05.json"})[0]
    assert sent == sitting_pack.benchmark_request(document)
    item = by_id(index_of(build(MANIFEST)))["cpb-g1-05"]
    assert item["swbpipe_request"]["canonical_sha256"] == canonical_model_hash(sent)


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
    path.chmod(0o600)
    path.write_text(text, encoding="utf-8")


@pytest.fixture
def sitting(fake_benchmark, tmp_path):
    """A prepared pack of invented cases and a sample (outside any work tree), its authority and the exchange folder."""
    state = fake_benchmark
    declaration = sample_declaration(tmp_path / "samples", [("fs-a", "fs-a.mbf", INVENTED_SAMPLE)])
    plan = plan_file(tmp_path, [
        {"id": "dry", "kind": "dry_run", "select": {"model_ids": ["cpb-g1-01", "cpb-g1-02"]},
         "case_overrides": {"cpb-g1-02": {"variants": [
             {"label": "by-hand", "kind": "instruction", "instruction": "Switch the invented option on."}]}},
         "format_samples": [{"id": "fs-a"}]},
        {"id": "sealed", "kind": "sealed_capture", "sealed_reason": "invented: SWBPIPE side not ready",
         "select": {"model_ids": ["cpb-g1-03"]}},
    ])
    pack = write(tmp_path / "pack", build(state["manifest"], plan, samples=declaration))
    for path in pack.rglob("*"):
        if path.is_file():
            path.chmod(0o600)  # a person's copy: writable
    state.update(pack=pack, authority=tmp_path / "pack.authority", root=state["config"].exchange_root.resolve(),
                 index=json.loads((pack / "pack_index.json").read_text(encoding="utf-8")), declaration=declaration,
                 plan=plan)
    state["offline"] = ["--benchmark", str(state["manifest"]), "--runner", state["runner"], "--offline"]
    return state


def ingest(state, **kwargs) -> bool:
    return sitting_pack.ingest_pack(state["pack"], state["root"], state["authority"], **kwargs)


def return_guest(state, case_id: str, content: str | bytes | None = None) -> Path:
    folder = state["pack"] / "cases" / case_id
    item = by_id(state["index"])[case_id]
    data = invented_print().encode("utf-8") if content is None else (
        content.encode("utf-8") if isinstance(content, str) else content)
    (folder / f"{case_id}.csv").write_bytes(data)
    (folder / f"{case_id}.guest.json").write_text(guest_json(item["mbf"]["sha256"], case_id), encoding="utf-8")
    fill_run_line(folder, "guest_script")
    return folder


def only_run(state, case_id: str) -> Path:
    runs = sorted((state["root"] / "cases" / case_id / "sitting_runs").iterdir())
    assert len(runs) == 1
    return runs[0]


def test_a_guest_script_result_is_kept_whole_in_a_new_run_and_binds_after_the_pack_is_gone(sitting, capsys):
    folder = return_guest(sitting, "cpb-g1-01")
    root, item = sitting["root"], by_id(sitting["index"])["cpb-g1-01"]
    before = tree_digest(root)
    assert ingest(sitting, check=True, selected=["cpb-g1-01"])
    assert tree_digest(root) == before  # the check writes nothing
    checked = capsys.readouterr().out
    assert "cpb-g1-01: benchmark case; checked; route=csv_from_batch; binding=guest_reported_hash" in checked
    assert "0 error diagnostic(s)" in checked
    assert ingest(sitting, selected=["cpb-g1-01"])
    run = only_run(sitting, "cpb-g1-01")
    assert re.fullmatch(r"\d{8}T\d{12}Z-[0-9a-f]{12}", run.name)
    record = json.loads((run / "caepipe_out" / "run_record.json").read_text(encoding="utf-8"))
    assert record["route"] == "csv_from_batch" and record["sitting"]["binding"] == "guest_reported_hash"
    assert record["mbf_profile"]["profile_id"] == "caepipe-mbf-si-pilot-1"
    assert "TBD-17-04-010" in record["mbf_profile"]["open_questions"]
    assert record["mbf_profile"]["run_open_questions"] == ["TBD-17-01-004"]
    intake = json.loads((run / sitting_pack.INTAKE_RECORD).read_text(encoding="utf-8"))
    selection = json.loads((root / "cases" / "cpb-g1-01" / "run_selection" / f"{run.name}.json").read_text())
    assert selection["run"] == f"sitting_runs/{run.name}" and "No existing reader consumes" in selection["rule"]
    whole = tree_digest(root)
    assert ingest(sitting, selected=["cpb-g1-01"])
    assert "already_placed" in capsys.readouterr().out and tree_digest(root) == whole
    # The returned pack is removed: everything needed to use the run is in the run folder.
    shutil.rmtree(sitting["pack"])
    assert not folder.exists()
    for key in ("mbf", "identity_file", "writer_report"):
        retained = record["retained_inputs"][key]
        assert hashlib.sha256((run / retained["path"]).read_bytes()).hexdigest() == item[key]["sha256"]
    assert intake["returned_files"]["cpb-g1-01.csv"]["sha256"] == hashlib.sha256(
        (run / "returned" / "cpb-g1-01.csv").read_bytes()).hexdigest()
    csv_path, route, _ = run_pilot.bound_csv(root, {"root": run}, record, item["mbf"]["sha256"])
    assert csv_path == run / "caepipe_out" / "model.csv" and route == "csv_from_batch"


def test_a_result_obtained_by_hand_is_recorded_under_its_own_route_and_never_bound(sitting):
    folder = sitting["pack"] / "cases" / "cpb-g1-02"
    (folder / "cpb-g1-02.print.csv").write_text(invented_print(), encoding="utf-8")
    (folder / "cpb-g1-02.by-hand.print.csv").write_text(invented_print(sustained_uy="-5"), encoding="utf-8")
    fill_run_line(folder, "manual_print")
    assert ingest(sitting, selected=["cpb-g1-02"])
    run = only_run(sitting, "cpb-g1-02")
    record = json.loads((run / "caepipe_out" / "run_record.json").read_text(encoding="utf-8"))
    assert record["route"] == "sitting_manual_print" and record["guest_mbf_sha256"] is None
    assert record["sitting"]["binding"] == "person_attested"
    assert run_pilot.bound_csv(sitting["root"], {"root": run}, record, record["mbf"]["sha256"])[0] is None
    variant = json.loads((run / sitting_pack.INTAKE_RECORD).read_text(encoding="utf-8"))["variants"]["by-hand"]
    assert variant["returned"] == ["returned/cpb-g1-02.by-hand.print.csv"] and variant["screened"] is False


def test_a_stored_run_stays_byte_identical_and_a_changed_mbf_is_refused_without_a_write(sitting, capsys):
    root = sitting["root"]
    case = root / "cases" / "cpb-g1-01"
    for relative, data in (("caepipe_out/run_record.json", b'{"invented": "stored run"}\n'),
                           ("caepipe_out/model.csv", b"invented stored print\r\n"),
                           ("caepipe_in/model.mbf", b"INVENTED STORED MBF\r\n")):
        (case / relative).parent.mkdir(parents=True, exist_ok=True)
        (case / relative).write_bytes(data)
    stored = tree_digest(case)
    folder = return_guest(sitting, "cpb-g1-01")
    mbf = folder / "cpb-g1-01.mbf"
    original = mbf.read_bytes()
    mbf.chmod(0o600)
    mbf.write_bytes(original + b"x")
    before = tree_digest(root)
    assert not ingest(sitting, selected=["cpb-g1-01"])
    assert "mbf_changed" in capsys.readouterr().out and tree_digest(root) == before
    mbf.write_bytes(original)
    assert ingest(sitting, selected=["cpb-g1-01"])
    after = tree_digest(case)
    assert {key: value for key, value in after.items() if key in stored} == stored
    assert only_run(sitting, "cpb-g1-01").is_dir()


# -- the authoritative index ----------------------------------------------------------------------------------------
def rewrite_index(state, edit) -> None:
    """Edit the returned pack's index (the person's copy)."""
    path = state["pack"] / "pack_index.json"
    index = json.loads(path.read_text(encoding="utf-8"))
    edit(index)
    path.write_text(json.dumps(index), encoding="utf-8")


def test_intake_takes_every_designation_from_the_authority_and_refuses_a_changed_index(sitting, reads):
    return_guest(sitting, "cpb-g1-03", LOUD)
    watch_returned(reads, sitting["pack"] / "cases" / "cpb-g1-03", sitting["root"])

    def unseal_by_editing(index):
        for item in index["items"]:
            if item["item_id"] == "cpb-g1-03":
                item["capture"] = "screened"
    rewrite_index(sitting, unseal_by_editing)
    before = tree_digest(sitting["root"])
    with pytest.raises(sitting_pack.PackError, match="differs from the authoritative index"):
        ingest(sitting)
    with pytest.raises(sitting_pack.PackError, match="differs from the authoritative index"):
        ingest(sitting, check=True)
    assert tree_digest(sitting["root"]) == before and reads.parser_calls == [] and reads.content_reads() == []
    # An authority whose own digest does not match is refused; so is one whose paths are not plain.
    digest = sitting["authority"] / "pack_index.sha256"
    good = digest.read_text()
    digest.chmod(0o600)
    digest.write_text("0" * 64 + "  pack_index.json\n")
    with pytest.raises(sitting_pack.PackError, match="does not have the SHA-256"):
        sitting_pack.load_authority(sitting["authority"])
    digest.write_text(good)
    for bad in ({"folder": "../outside"}, {"folder": "/absolute"}, {"item_id": "../x"},
                {"mbf": {"file": "../x.mbf", "sha256": "0" * 64}}, {"item_kind": "unknown"},
                {"capture": "stored_only"}):
        index = json.loads((sitting["authority"] / "pack_index.json").read_text())
        target = next(item for item in index["items"] if item["item_id"] == "cpb-g1-01")
        target.update(bad)
        with pytest.raises(sitting_pack.PackError):
            sitting_pack.validate_index(index)


# -- sealed captures: instrumented reads ----------------------------------------------------------------------------
LOUD = b"\x00\xff\xfeNOT A PRINT \x80\x81 Caepipe? Displacements\x00"
RESERVED_NAMES = ("capture_record.json", "sitting_intake.json", "stored_output.json", "run_record.json",
                  "pack_index.json", "guest_log.json")


class Reads:
    """Every read-only open of a watched file (by io.open, builtins.open or os.open, a dir_fd-relative one resolved
    to its full path), with whether it happened inside a hashing pass; and every call of a parser or guest-report
    entry point at any alias, each of which fails the test at once."""

    def __init__(self):
        self.log: list[dict] = []
        self.parser_calls: list[str] = []
        self.hashing = 0
        self.watched: Callable[[Path], bool] = lambda path: False

    def content_reads(self) -> list[dict]:
        return [entry for entry in self.log if not entry["hashing"]]

    def per_file(self) -> dict[str, int]:
        counts: dict[str, int] = {}
        for entry in self.log:
            counts[str(entry["path"])] = counts.get(str(entry["path"]), 0) + 1
        return counts


@pytest.fixture
def reads(monkeypatch):
    state = Reads()
    real_io_open, real_os_open = io.open, os.open

    def note(path: Path) -> None:
        if not PAUSE[0] and state.watched(path):
            state.log.append({"path": path, "hashing": state.hashing > 0})

    def watched_io_open(file, mode="r", *args, **kwargs):
        if isinstance(file, (str, os.PathLike)) and "r" in mode and "+" not in mode:
            note(Path(os.path.abspath(os.fspath(file))))
        return real_io_open(file, mode, *args, **kwargs)

    def watched_os_open(path, flags, mode=0o777, *, dir_fd=None):
        if (flags & os.O_ACCMODE) == os.O_RDONLY and not flags & os.O_DIRECTORY:
            name = os.fspath(path)
            if dir_fd is not None and not os.path.isabs(name):
                base = fcntl.fcntl(dir_fd, fcntl.F_GETPATH, bytes(1024)).split(b"\0", 1)[0].decode()
                name = os.path.join(base, name)
            note(Path(os.path.abspath(name)))
        return real_os_open(path, flags, mode, dir_fd=dir_fd)

    monkeypatch.setattr(io, "open", watched_io_open)
    monkeypatch.setattr(builtins, "open", watched_io_open)
    monkeypatch.setattr(os, "open", watched_os_open)
    for owner, name in ((sitting_pack, "stream_sha256"), (sitting_pack.Tree, "copy")):
        real = getattr(owner, name)

        def hashing(*args, _real=real, **kwargs):
            state.hashing += 1
            try:
                return _real(*args, **kwargs)
            finally:
                state.hashing -= 1
        monkeypatch.setattr(owner, name, hashing)

    def forbidden(name):
        def call(*args, **kwargs):
            state.parser_calls.append(name)
            raise AssertionError(f"{name} was called")
        return call
    for owner, name in ((print_csv, "read_caepipe_print_csv"), (print_csv, "parse_caepipe_print_csv"),
                        (caepipe_external, "read_caepipe_print_csv"), (caepipe_external, "parse_caepipe_print_csv"),
                        (run_pilot, "read_caepipe_print_csv"), (harness, "_print_layout_problem"),
                        (harness, "last_json_object"), (sitting_pack, "last_json_object"),
                        (sitting_pack, "parse_run_line"), (sitting_pack, "_guest_result"),
                        (sitting_pack, "classify_case"), (sitting_pack, "_parser_counts")):
        monkeypatch.setattr(owner, name, forbidden(f"{owner.__name__}.{name}"))
    return state


def watch_returned(reads: Reads, folder: Path, root: Path) -> None:
    """Watch every file that came back in `folder` (not the pack's own inputs) and everything in a payload."""
    folder = folder.resolve()
    inputs = {"case.json"} | {p.name for p in folder.iterdir() if p.suffix == ".mbf" or p.name.endswith(
        (".swbpipe-ids.json", ".writer-report.json"))}
    returned_names = {p.name for p in folder.iterdir() if p.name not in inputs}
    root = root.resolve()

    def watched(path: Path) -> bool:
        path = path.resolve() if path.exists() else path
        if path.parent == folder and path.name in returned_names:
            return True
        return path.is_relative_to(root) and sitting_pack.PAYLOAD in path.parts
    reads.watched = watched


def test_a_sealed_capture_is_copied_once_under_its_names_and_never_read_or_parsed(sitting, reads, capsys):
    folder = return_guest(sitting, "cpb-g1-03", LOUD)
    for name in RESERVED_NAMES:
        (folder / name).write_bytes(b'{"document_kind": "' + name.encode() + b'", "invented": true}')
    root = sitting["root"]
    watch_returned(reads, folder, root)
    status_before = git_status()
    assert ingest(sitting, selected=["cpb-g1-03"])
    assert "cpb-g1-03: sealed capture; stored" in capsys.readouterr().out
    assert reads.parser_calls == [] and reads.content_reads() == []
    sources = {str(p.resolve()): 1 for p in folder.iterdir() if reads.watched(p)}
    assert {key: value for key, value in reads.per_file().items() if key in sources} == sources
    attempts = sorted((root / "cases" / "cpb-g1-03" / "sealed").iterdir())
    assert len(attempts) == 1
    attempt = attempts[0]
    assert sorted(p.name for p in attempt.iterdir()) == ["capture_record.json", "payload"]
    record = json.loads((attempt / "capture_record.json").read_text(encoding="utf-8"))
    assert record["document_kind"] == sitting_pack.CAPTURE_KIND and record["status"] == "sealed"
    assert record["files"]["cpb-g1-03.csv"] == {"bytes": len(LOUD), "sha256": hashlib.sha256(LOUD).hexdigest(),
                                                "role": "returned"}
    for name in RESERVED_NAMES:  # stored as payload, never taken for a record
        assert (attempt / "payload" / name).read_bytes().startswith(b'{"document_kind": "' + name.encode())
        assert record["files"][name]["role"] == "returned"
    assert record["mbf_sha256"] == by_id(sitting["index"])["cpb-g1-03"]["mbf"]["sha256"]
    assert attempt.is_relative_to(root) and harness._inside_git(root, os.environ) is False
    assert git_status() == status_before
    assert not (root / "cases" / "cpb-g1-03" / "caepipe_out").exists()
    # A re-intake hashes each returned file once and writes nothing; a different capture is refused.
    before = tree_digest(root)
    reads.log.clear()
    assert ingest(sitting, selected=["cpb-g1-03"])
    assert "already_sealed" in capsys.readouterr().out and reads.content_reads() == []
    assert {key: value for key, value in reads.per_file().items() if key in sources} == sources
    assert tree_digest(root) == before
    (folder / "cpb-g1-03.csv").write_bytes(LOUD + b"x")
    assert not ingest(sitting, selected=["cpb-g1-03"])
    assert "never replaced" in capsys.readouterr().out
    assert tree_digest(root) == before and reads.parser_calls == []


def test_nothing_opens_a_sealed_item_on_any_other_route(sitting, reads, tmp_path, capsys):
    folder = return_guest(sitting, "cpb-g1-03", LOUD)
    root = sitting["root"]
    watch_returned(reads, folder, root)
    assert ingest(sitting, selected=["cpb-g1-03"])
    capsys.readouterr()
    # --check on the sealed item (and the sample) opens nothing.
    sample = sitting["pack"] / "samples" / "fs-a"
    (sample / "fs-a.csv").write_bytes(LOUD)
    watch_returned(reads, folder, root)
    reads.log.clear()
    assert ingest(sitting, check=True, selected=["cpb-g1-03", "fs-a"])
    out = capsys.readouterr().out
    assert "cpb-g1-03: sealed capture; present; not opened" in out and "fs-a: format sample; present; not opened" in out
    assert reads.content_reads() == [] and reads.parser_calls == []
    # A later pack that names the same case as an ordinary screened case: refused, by check and by intake.
    later = plan_file(tmp_path, [{"id": "later", "kind": "dry_run", "select": {"model_ids": ["cpb-g1-03"]}}],
                      "later.json")
    later_pack = write(tmp_path / "later", build(sitting["manifest"], later, samples=sitting["declaration"]))
    for path in later_pack.rglob("*"):
        path.chmod(0o700 if path.is_dir() else 0o600)
    state = dict(sitting, pack=later_pack, authority=tmp_path / "later.authority",
                 index=json.loads((later_pack / "pack_index.json").read_text()))
    return_guest(state, "cpb-g1-03", LOUD)
    (later_pack / "cases" / "cpb-g1-03" / "cpb-g1-03.print.csv").write_bytes(LOUD)
    watch_returned(reads, later_pack / "cases" / "cpb-g1-03", root)
    before = tree_digest(root)
    for check in (True, False):
        result = ingest(state, check=check)
        out = capsys.readouterr().out
        assert "sealed" in out, out
        assert result is check  # the check reports it unopened; intake refuses it
    assert tree_digest(root) == before and reads.content_reads() == [] and reads.parser_calls == []
    # An index that names the same MBF under another id is refused by the MBF hash of the capture record.
    other = json.loads((later_pack / "pack_index.json").read_text())
    item = next(entry for entry in other["items"] if entry["item_id"] == "cpb-g1-03")
    item.update(item_id="cpb-x-03", folder="cases/cpb-x-03")
    (later_pack / "cases" / "cpb-g1-03").rename(later_pack / "cases" / "cpb-x-03")
    data = json.dumps(other).encode("utf-8")
    (later_pack / "pack_index.json").write_bytes(data)
    renamed = tmp_path / "renamed.authority"
    renamed.mkdir()
    (renamed / "pack_index.json").write_bytes(data)
    (renamed / "pack_index.sha256").write_text(hashlib.sha256(data).hexdigest() + "  pack_index.json\n")
    watch_returned(reads, later_pack / "cases" / "cpb-x-03", root)
    assert not sitting_pack.ingest_pack(later_pack, root, renamed)
    assert "a sealed capture of the same MBF" in capsys.readouterr().out
    assert tree_digest(root) == before and reads.content_reads() == [] and reads.parser_calls == []


def test_the_offline_screen_and_the_private_checks_pass_over_a_sealed_capture(sitting, reads, capsys):
    folder = return_guest(sitting, "cpb-g1-03", LOUD)
    root = sitting["root"]
    assert ingest(sitting, selected=["cpb-g1-03"])
    reads.watched = lambda path: "sealed" in path.parts
    reads.log.clear()
    monkey_reads = reads  # the parser entry points stay forbidden throughout
    run_pilot.main(sitting["offline"], environ={})
    assert "cpb-g1-03: route=sealed_capture" in capsys.readouterr().out
    # The live benchmark run refuses it before it starts the case, and the pilot's case runs refuse a sealed case.
    entry = next(m for m in run_pilot.load_benchmark(sitting["manifest"]) if m["model_id"] == "cpb-g1-03")
    status, route, _ = run_pilot.run_benchmark_model(entry, sitting["config"], runner="/invented/runner",
                                                     offline=False, emit=lambda line: None)
    assert (status, route) == (run_pilot.NOT_RUN, "sealed_capture")
    pilot = run_pilot.load_cases()[0]
    sealed = root / "cases" / pilot["case_id"] / "sealed" / "invented-attempt" / "payload"
    sealed.mkdir(parents=True)
    (sealed / "model.csv").write_bytes(LOUD)
    lines = []
    assert run_pilot.run_case(pilot, sitting["config"], runner="/invented/runner", offline=True, hand_calcs=None,
                              emit=lines.append) is False
    assert "route=sealed_capture" in lines[0]
    from tests import test_caepipe_private_correlation as private
    assert private.stored_cases(sitting["config"]) == []
    from tests import test_caepipe_private_mbf_import as private_import
    private_import.check_exchange(root, record=False, emit=lambda line: None)
    assert monkey_reads.log == [] and monkey_reads.parser_calls == []
    del folder


def test_format_samples_and_observations_are_stored_with_their_designation_and_never_read(fake_benchmark, tmp_path,
                                                                                         reads, capsys):
    state = fake_benchmark
    (tmp_path / "obs").mkdir()
    (tmp_path / "obs" / "invented-input.mbf").write_bytes(b"INVENTED\r\n")
    observations = tmp_path / "obs" / "observations.json"
    observations.write_text(json.dumps({"document_kind": sitting_pack.OBSERVATIONS_KIND, "items": [
        {"id": "obs-b", "rank": 2, "title": "Second", "action": "Invented action.", "bring_back": ["b.txt"]},
        {"id": "obs-a", "rank": 1, "title": "First", "action": "Invented action.", "input": "invented-input.mbf",
         "bring_back": ["a.txt", "a2.txt"]}]}), encoding="utf-8")
    declaration = sample_declaration(tmp_path / "samples", [("fs-a", "fs-a.mbf", INVENTED_SAMPLE)])
    plan = plan_file(tmp_path, [{"id": "dry", "kind": "dry_run", "select": {}, "format_samples": [{"id": "fs-a"}]},
                                {"id": "obs", "kind": "observations"}])
    pack = write(tmp_path / "pack", build(state["manifest"], plan, observations, samples=declaration))
    for path in pack.rglob("*"):
        path.chmod(0o700 if path.is_dir() else 0o600)
    (pack / "samples" / "fs-a" / "fs-a.csv").write_bytes(LOUD)
    (pack / "samples" / "fs-a" / "stored_output.json").write_bytes(b'{"invented": "a returned file"}')
    (pack / "observations" / "obs-a" / "a.txt").write_bytes(LOUD + b"a")
    root = state["config"].exchange_root.resolve()
    watch_returned(reads, pack / "samples" / "fs-a", root)
    assert sitting_pack.ingest_pack(pack, root, tmp_path / "pack.authority")
    assert reads.parser_calls == [] and reads.content_reads() == []
    out = capsys.readouterr().out
    assert "obs-b: format observation; not_returned" in out and "not returned: a2.txt" in out
    attempt = next((root / "format_samples" / "fs-a").iterdir())
    assert sorted(p.name for p in attempt.iterdir()) == ["payload", "stored_output.json"]
    stored = json.loads((attempt / "stored_output.json").read_text(encoding="utf-8"))
    assert stored["files"]["fs-a.csv"] == {"bytes": len(LOUD), "sha256": hashlib.sha256(LOUD).hexdigest(),
                                           "role": "returned"}
    assert stored["designation"]["sha256"] == hashlib.sha256(INVENTED_SAMPLE).hexdigest()
    assert (attempt / "payload" / "fs-a.mbf").read_bytes() == INVENTED_SAMPLE
    assert (attempt / "payload" / "stored_output.json").read_bytes() == b'{"invented": "a returned file"}'
    observed = json.loads(next((root / "observations" / "obs-a").iterdir()).joinpath("stored_output.json").read_text())
    assert list(observed["files"]) == ["a.txt"] and observed["missing"] == ["a2.txt"]
    assert not (root / "observations" / "obs-b").exists()


# -- create-only and contained writes -------------------------------------------------------------------------------
def test_a_write_onto_an_existing_path_is_refused_by_name_and_leaves_it_untouched(tmp_path):
    base = tmp_path / "base"
    (base / "dir").mkdir(parents=True)
    (base / "dir" / "kept").write_bytes(b"kept")
    source = tmp_path / "source"
    source.write_bytes(b"new")
    with sitting_pack.Tree(base, "test") as tree:
        with pytest.raises(sitting_pack.ExistingPath, match="dir/kept already exists"):
            tree.create(("dir", "kept"), b"replacement")
        with pytest.raises(sitting_pack.ExistingPath):
            tree.copy(("dir", "kept"), source)
        with pytest.raises(sitting_pack.ExistingPath):
            tree.reserve_dir(("dir",))
        with pytest.raises(sitting_pack.PackError):
            tree.create(("dir", "../escape"), b"x")
    assert (base / "dir" / "kept").read_bytes() == b"kept" and not (tmp_path / "escape").exists()


def test_a_parent_swapped_for_a_link_mid_intake_cannot_redirect_a_write(sitting, tmp_path, monkeypatch, capsys):
    return_guest(sitting, "cpb-g1-01")
    outside = tmp_path / "outside"
    outside.mkdir()
    real = sitting_pack.Tree.reserve_dir

    def swap_after_attempt(self, parts):
        result = real(self, parts)
        if parts[-2:-1] == ("sitting_runs",):
            runs = self.path(parts[:-1])
            runs.rename(runs.with_name("sitting_runs.moved"))
            (outside / parts[-1]).mkdir()
            runs.symlink_to(outside)
        return result
    monkeypatch.setattr(sitting_pack.Tree, "reserve_dir", swap_after_attempt)
    assert not ingest(sitting, selected=["cpb-g1-01"])
    assert "symbolic link" in capsys.readouterr().out
    assert [p.name for p in outside.rglob("*")] == [next(outside.iterdir()).name]  # only the test's own folder
    assert list(next(outside.iterdir()).iterdir()) == []


def test_a_static_link_or_a_nested_repository_in_the_exchange_folder_refuses_the_write(sitting, tmp_path, capsys):
    return_guest(sitting, "cpb-g1-01")
    root = sitting["root"]
    outside = tmp_path / "outside"
    outside.mkdir()
    (root / "cases").symlink_to(outside)
    assert not ingest(sitting, selected=["cpb-g1-01"])
    assert "symbolic link" in capsys.readouterr().out and list(outside.iterdir()) == []
    (root / "cases").unlink()
    (root / "cases").mkdir()
    subprocess.run(["git", "init", "-q", str(root / "cases")], check=True)
    before = tree_digest(root)
    return_guest(sitting, "cpb-g1-03", LOUD)
    assert not ingest(sitting, selected=["cpb-g1-01", "cpb-g1-03"])
    out = capsys.readouterr().out
    assert out.count("holds .git") == 2 and tree_digest(root) == before


def count_writes(monkeypatch, fail_after: int | None) -> list[int]:
    """Count Tree's writes (new directories, new files, copies); with `fail_after`, raise after that many."""
    done = [0]
    for name in ("create", "copy", "reserve_dir"):
        real = getattr(sitting_pack.Tree, name)

        def counted(self, *args, _real=real, **kwargs):
            if fail_after is not None and done[0] >= fail_after:
                raise RuntimeError("invented interruption")
            done[0] += 1
            return _real(self, *args, **kwargs)
        monkeypatch.setattr(sitting_pack.Tree, name, counted)
    return done


@pytest.mark.parametrize("case_id, record", [("cpb-g1-01", sitting_pack.INTAKE_RECORD),
                                             ("cpb-g1-03", sitting_pack.CAPTURE_RECORD)])
def test_an_interrupted_intake_is_left_in_place_and_a_later_one_is_a_new_attempt(sitting, monkeypatch, tmp_path,
                                                                                  case_id, record):
    folder = return_guest(sitting, case_id, None if case_id == "cpb-g1-01" else LOUD)
    item = by_id(sitting["index"])[case_id]
    base = "sitting_runs" if record == sitting_pack.INTAKE_RECORD else "sealed"

    def take_in(root: Path):
        with sitting_pack.Tree(root, "test exchange") as tree:
            if record == sitting_pack.INTAKE_RECORD:
                return sitting_pack.intake_case(folder, item, tree, pack_index_sha256="0" * 64)
            return sitting_pack.seal_capture(folder, item, tree, pack_index_sha256="0" * 64)

    probe = tmp_path / "probe"
    probe.mkdir()
    with monkeypatch.context() as patch:
        total = count_writes(patch, None)
        take_in(probe)
    for boundary in range(1, total[0]):
        root = tmp_path / f"exchange-{boundary}"
        root.mkdir()
        with monkeypatch.context() as patch:
            count_writes(patch, boundary)
            with pytest.raises(RuntimeError):
                take_in(root)
        partial = tree_digest(root)
        outcome = take_in(root)
        assert outcome["status"] in ("placed", "stored", "already_placed"), (boundary, outcome)
        after = tree_digest(root)
        assert {key: value for key, value in after.items() if key in partial} == partial, boundary
        attempts = sorted((root / "cases" / case_id / base).iterdir())
        complete = [a for a in attempts if (a / record).is_file()]
        assert len(complete) == 1, boundary
        named = json.loads((complete[0] / record).read_text())["earlier_partial_attempts"]
        assert named == [f"cases/{case_id}/{base}/{a.name}" for a in attempts if a not in complete], boundary
        if record == sitting_pack.INTAKE_RECORD:  # an interruption before the selection: the retry adds it, new
            assert (root / "cases" / case_id / "run_selection" / f"{complete[0].name}.json").is_file(), boundary


# -- the driver and the command line ------------------------------------------------------------------------------
def test_the_driver_refuses_rather_than_removes_or_replaces():
    text = sitting_pack.DRIVER.read_text(encoding="utf-8")
    assert 'Join-Path $dir.FullName "harness_config.json"' in text and "Refuse" in text
    assert 'Join-Path $packRoot.FullName "pack_index.json"' in text
    assert '@(".csv", ".txt", "_results.csv", ".res", ".mod", ".guest.json")' in text
    assert "[IO.FileMode]::CreateNew" in text and "-NoGuiFallback" in text
    for removed in ("Remove-Item", "WriteAllText", "Set-Content", "Out-File", "Add-Content", "Copy-Item", "Move-Item"):
        assert removed not in text, removed
    assert "has not been executed by SWBPIPE's tests" in text


def test_the_command_line_has_no_replace_and_no_unseal_and_needs_the_authority(capsys):
    for argv in (["ingest", "/invented/pack", "--authority", "/invented/a", "--replace"], ["unseal", "--case", "x"],
                 ["ingest", "/invented/pack"]):
        with pytest.raises(SystemExit):
            sitting_pack.main(argv)
        capsys.readouterr()
    assert not hasattr(sitting_pack, "unseal_capture")


def test_the_reading_functions_refuse_a_sealed_case_themselves(sitting, reads, monkeypatch):
    # classify_case itself runs; the readers it would reach (run line, guest report, parser) stay forbidden.
    monkeypatch.setattr(sitting_pack, "classify_case", REAL_CLASSIFY)
    folder = return_guest(sitting, "cpb-g1-03", LOUD)
    root = sitting["root"]
    assert ingest(sitting, selected=["cpb-g1-03"])
    screened = dict(by_id(sitting["index"])["cpb-g1-03"], capture="screened")
    watch_returned(reads, folder, root)
    reads.log.clear()
    with sitting_pack.Tree(root, "the exchange folder") as tree:
        for call in (lambda: sitting_pack.classify_case(folder, screened, tree),
                     lambda: sitting_pack.check_case(folder, screened, tree),
                     lambda: sitting_pack.intake_case(folder, screened, tree, pack_index_sha256="0" * 64)):
            outcome = call()
            assert outcome["status"] == "refused" and "sealed in the exchange folder" in outcome["reason"]
        with pytest.raises(sitting_pack.PackError, match="only a screened benchmark case"):
            sitting_pack.classify_case(folder, by_id(sitting["index"])["cpb-g1-03"], tree)
    assert reads.content_reads() == [] and reads.parser_calls == []


def test_the_hand_route_can_be_rehearsed_end_to_end_with_invented_files(fake_benchmark, tmp_path, monkeypatch,
                                                                        capsys):
    """The hand route alone, from the generated pack: a script stands in for the program and places invented files
    under the names the pack gives; ingest --check and ingest take them in, stored and hashed."""
    state = fake_benchmark
    monkeypatch.setattr(sitting_pack, "resolve_harness_config", lambda environ, require_vm_tools: state["config"])
    pack, authority = tmp_path / "rehearsal", tmp_path / "rehearsal.authority"
    plan = plan_file(tmp_path, [{"id": "dry", "kind": "dry_run", "select": {"model_ids": ["cpb-g1-01"]}}])
    assert sitting_pack.main(["prepare", str(pack), "--manifest", str(state["manifest"]), "--plan", str(plan),
                              "--samples", str(sample_declaration(tmp_path / "samples", []))]) == 0
    instructions = (pack / "INSTRUCTIONS.md").read_text(encoding="utf-8")
    assert "`<stem>.print.csv`, `<stem>.mod` and `<stem>.res`" in instructions
    case = json.loads((pack / "cases" / "cpb-g1-01" / "case.json").read_text(encoding="utf-8"))
    folder = pack / "cases" / "cpb-g1-01"
    invented = {}
    for name in case["results"]["manual_print"]:  # the names the pack gives for the route
        data = f"invented {name}\r\n".encode("utf-8")
        (folder / name).write_bytes(data)
        invented[name] = hashlib.sha256(data).hexdigest()
    fill_run_line(folder, "manual_print")
    assert sitting_pack.main(["ingest", str(pack), "--authority", str(authority), "--check"], environ={}) == 0
    assert "route=sitting_manual_print; binding=person_attested" in capsys.readouterr().out
    assert sitting_pack.main(["ingest", str(pack), "--authority", str(authority)], environ={}) == 0
    assert "route=sitting_manual_print; binding=person_attested" in capsys.readouterr().out
    run = next((state["config"].exchange_root / "cases" / "cpb-g1-01" / "sitting_runs").iterdir())
    record = json.loads((run / sitting_pack.INTAKE_RECORD).read_text(encoding="utf-8"))
    for name, sha in invented.items():
        assert record["returned_files"][name]["sha256"] == sha
        assert hashlib.sha256((run / "returned" / name).read_bytes()).hexdigest() == sha
