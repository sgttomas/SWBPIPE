#!/usr/bin/env python3
"""The append-only rule of the CAEPIPE pilot's live steps and screens (DEL-17-05), checked through the real command.

Every test here enters through `run_pilot.main` with its real arguments, over a populated invented exchange folder
made in a temporary directory, with the guest substituted only where the virtual machine tool is called: a fake
`prlctl` first on PATH (tests/caepipe_fake_guest.py). No step is replaced: the SWBPIPE solve (the runner executable),
the MBF writer, the harness, the parser, the comparison, the export and the series step all run. The only other
substitutions are the benchmark's fixed retry wait (the pilot's `time`, so that a retry does not wait 120 s) and,
in the crash and race tests, a wrapper around one exclusive-layer operation that fails it or races it and otherwise
calls it.

The property: for every live path and screen, on success, refusal and error, every file and folder that existed
before the command is still there with the same SHA-256 (a deletion fails the check as surely as a change), and
everything new lies in a new run, screen or series file of its own, or a new selection. The invented guest also
checks its side: a `mkdir` or `del` on a shared drive, or a `copy` onto an existing file there, is a violation.

Nothing here reads any real exchange folder, starts any virtual machine or runs any external program.
"""

from __future__ import annotations

import contextlib
import errno
import hashlib
import io
import json
import os
from pathlib import Path
import re
import shutil
import stat
import sys
from types import SimpleNamespace

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import exclusive, harness  # noqa: E402
from core.handoff.caepipe_external.correlation import ALTERNATIVE_CHECK  # noqa: E402
from core.handoff.caepipe_external.harness import (  # noqa: E402
    ENV_VAR,
    REFUSED_ROUTES,
    RUN_RECORD_KIND,
    RUN_RECORD_MEMBERS,
    SELECTION_KIND,
    SelectionError,
    current_case,
)
from tests.caepipe_fake_guest import STATE_ENV, install, read_log  # noqa: E402
from tests.test_caepipe_benchmark import benchmark_manifest, invented_print  # noqa: E402
from tools.caepipe_correlation import run_pilot  # noqa: E402


PARSER_FIXTURE = ROOT / "fixtures" / "caepipe_external" / "invented" / "print_to_file_results.csv"
RUNNER = os.environ.get(run_pilot.RUNNER_ENV)
needs_runner = pytest.mark.skipif(not RUNNER or not Path(RUNNER).is_file(),
                                  reason=f"no runner executable ({run_pilot.RUNNER_ENV}); the SWBPIPE solve runs for real")
GUARD = {"key": "HKCU\\Software\\Invented Vendor\\Tool\\1.0", "recent_files_subkey": "Recent list"}
WRITER = "cpc-01-cantilever-force"   # stored in the legacy layout (tranche 1)
SELECTED = "cpc-07-cantilever-span-3m"  # stored in a run folder named by a selection
AUTHORED = "cpa-05-guide"            # stored in the legacy layout with an export beside its solve
GUEST_STEPS_AFTER_ROOT = {"prepare", "solve", "stage_script", "collect", "export_prepare", "export",
                          "export_collect", "type", "reg_write"}


# -- the invented exchange folder ------------------------------------------------------------------------------------
def tracked(case_id: str) -> dict:
    return next(case for case in run_pilot.load_cases() if case["case_id"] == case_id)


def mbf_text(case_id: str) -> str:
    return run_pilot.render_case_mbf(tracked(case_id))[2]


def sha(data: bytes | str) -> str:
    return hashlib.sha256(data.encode("utf-8") if isinstance(data, str) else data).hexdigest()


def put(path: Path, data: bytes | str) -> str:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = data.encode("utf-8") if isinstance(data, str) else data
    path.write_bytes(payload)
    return sha(payload)


def stored_comparison(case_id: str, marker: str) -> dict:
    """An invented comparison record of a cantilever member, of the shape the series step reads."""
    from validation.caepipe_correlation import hand_calcs
    return {"document_kind": "invented", "marker": marker, "exported_model_hash": sha(mbf_text(case_id)),
            "tolerance_basis": {ALTERNATIVE_CHECK: {"abs": {"m": 5e-7}}}, "hypothesis_series": {},
            "quantities": [{"id": "n1_uy", "unit": "m",
                            "values": {"swbpipe": hand_calcs.analytic_values(case_id)["n1_uy"]["value"],
                                       "caepipe": -1e-3},
                            "checks": {"analytic_vs_swbpipe": {"outcome": "matched_within_tolerance"}}}]}


def solve_folder(folder: Path, mbf: str | bytes, csv: str, *, marker: str, step: str | None = "round_trip",
                 case_id: str | None = None) -> None:
    """A stored solve in `folder` (a case's own folder, or a run folder): its MBF, a CSV, a solved model and a run
    record binding them, and a comparison record."""
    data = mbf.encode("utf-8") if isinstance(mbf, str) else mbf
    digest = put(folder / "caepipe_in" / "model.mbf", data)
    outputs = {"model_results.csv": put(folder / "caepipe_out" / "model_results.csv", csv),
               "model.mod": put(folder / "caepipe_out" / "model.mod", b"INVENTED STORED SOLVED MODEL " + marker.encode())}
    put(folder / "caepipe_out" / "run_record.json", json.dumps({
        "document_kind": RUN_RECORD_KIND, **({"step": step} if step else {}), "route": "csv_from_gui_print",
        "mbf": {"sha256": digest}, "guest_mbf_sha256": digest,
        "observed_csv_path": {"host": "caepipe_out/model_results.csv"},
        "outputs_collected": sorted(outputs), "outputs_sha256": outputs}))
    if case_id:
        put(folder / "comparison" / "comparison.json", json.dumps(stored_comparison(case_id, marker)))
        put(folder / "parsed" / "caepipe_results.json", json.dumps({"invented": marker}))
        put(folder / "case_manifest.json", json.dumps({"invented": marker}))


def selection(case_root: Path, number: int, **members) -> None:
    put(case_root / "selections" / f"{number:06d}.json", json.dumps({
        "document_kind": SELECTION_KIND, "case_id": case_root.name, "sequence": number,
        "run": None, "export": None, "screen": None, **members, "verified": {},
        "previous": f"selections/{number - 1:06d}.json" if number > 1 else None, "reason": "invented"}))


def populate(root: Path) -> None:
    """Stored results as earlier tranches left them, and as this layout keeps them."""
    csv = PARSER_FIXTURE.read_text(encoding="utf-8")
    # Tranche 1: a tracked case in the legacy layout (its own layer folders, no selection).
    solve_folder(root / "cases" / WRITER, mbf_text(WRITER), csv, marker="legacy", step=None, case_id=WRITER)
    # A case with a selected run, and older legacy files beside it that no reader may use while a selection exists.
    case = root / "cases" / SELECTED
    solve_folder(case, mbf_text(SELECTED), csv, marker="legacy", step=None, case_id=SELECTED)
    solve_folder(case / "runs" / "000001", mbf_text(SELECTED), csv, marker="run 000001", case_id=SELECTED)
    selection(case, 1, run="runs/000001")
    # An authored case with its export stored beside its solve (legacy layout).
    authored = next(item for item in run_pilot.load_authored_cases() if item["case_id"] == AUTHORED)
    folder = root / "cases" / AUTHORED
    solve_folder(folder, run_pilot.authored_mbf_bytes(authored), csv, marker="legacy authored", step=None)
    put(folder / "caepipe_out" / "exported.mbf", b"INVENTED\r\nSTORED EXPORT\r\n")
    # The harness's folder as an earlier version left it, and series records.
    put(root / "_harness" / f"guest-script-{'0' * 64}.ps1", "# an earlier invented guest script\r\n")
    put(root / "series" / "shear-deformation.json", json.dumps({"invented": "legacy series record"}))
    put(root / "series" / "shear-deformation" / "000001.json", json.dumps({"invented": "series record 1"}))


class World:
    """A temporary invented exchange folder, a guest work folder and the invented guest behind a fake prlctl."""

    def __init__(self, tmp_path: Path, monkeypatch, *, guard: bool = False, csv: str | None = None):
        self.tmp = tmp_path
        self.root = tmp_path / "exchange"
        self.root.mkdir(parents=True)
        self.root.chmod(0o700)
        put(self.root / ".gitignore", "*\n")
        self.config = {"vm_name": "invented-vm", "guest_executable": "C:\\Invented\\tool.exe",
                       "guest_exchange_root": "Z:\\invented", "guest_work_root": "C:\\invented-work",
                       "license_and_environment_responsibility_acknowledged": True,
                       **({"settings_guard": GUARD} if guard else {})}
        self.write_config()
        populate(self.root)
        self.work = tmp_path / "guest-c"
        self.work.mkdir()
        self.state_path = tmp_path / "guest-state.json"
        self.state = {"view": {"Z:\\invented": str(self.root)}, "listed": [],
                      "work": {"C:\\invented-work": str(self.work)},
                      "solve": {"mode": "batch", "csv": csv or PARSER_FIXTURE.read_text(encoding="utf-8")},
                      "export": {}, "actions": [], "log": str(tmp_path / "guest-log.jsonl")}
        self.save()
        monkeypatch.setenv(STATE_ENV, str(self.state_path))
        install(tmp_path / "bin")
        self.environ = {ENV_VAR: str(self.root), "PATH": f"{tmp_path / 'bin'}{os.pathsep}{os.environ.get('PATH', '')}"}
        if RUNNER:
            self.environ[run_pilot.RUNNER_ENV] = RUNNER
        self.sleeps: list[float] = []
        monkeypatch.setattr(run_pilot, "time", SimpleNamespace(sleep=self.sleeps.append))

    def write_config(self) -> None:
        (self.root / "harness_config.json").write_text(json.dumps(self.config), encoding="utf-8")

    def save(self) -> None:
        self.state_path.write_text(json.dumps(self.state), encoding="utf-8")

    def log(self) -> list[dict]:
        return read_log(self.state)

    def main(self, *argv: str):
        out = io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(out):
            try:
                code = run_pilot.main(list(argv), environ=self.environ)
            except Exception as error:  # an injected crash escaping the command: kept for the test to examine
                code = error
        return code, out.getvalue()

    def benchmark(self, *extra: str) -> list[str]:
        manifest = self.tmp / "set" / "manifest.json"
        if not manifest.exists():
            benchmark_manifest(self.tmp / "set", count=1)
        return ["--benchmark", str(manifest), "--case", "cpb-g1-01", "--freeze-file", str(self.tmp / "FREEZE"),
                "--exclusive-file", str(self.tmp / "exclusive.pending"), "--pace-s", "0", *extra]


def snapshot(root: Path) -> dict[str, str]:
    """Every path below `root`: a file's SHA-256, `dir`, or a link's target; nothing followed."""
    found = {}
    for path in sorted(root.rglob("*")):
        mode = path.lstat().st_mode
        relative = path.relative_to(root).as_posix()
        if stat.S_ISLNK(mode):
            found[relative] = "link " + os.readlink(path)
        elif stat.S_ISDIR(mode):
            found[relative] = "dir"
        else:
            found[relative] = sha(path.read_bytes()) if os.access(path, os.R_OK) else "unreadable"
    return found


NEW_PLACES = (r"cases/[a-z0-9-]+(/(runs|screens|selections))?",
              r"cases/[a-z0-9-]+/(runs|screens)/[0-9]{6}(/.*)?",
              r"cases/[a-z0-9-]+/selections/[0-9]{6}\.json",
              r"series(/[a-z0-9-]+(/[0-9]{6}\.json)?)?",
              r"_harness(/guest-script-[0-9a-f]{64}\.ps1)?")


def assert_append_only(before: dict[str, str], after: dict[str, str], *, changed_by_others=()) -> list[str]:
    """Everything that existed is unchanged (a deletion fails as a change does); everything new lies in a new run
    or screen folder of its own, a new selection, a new series record, or the content-named guest script. Returns
    the new paths."""
    for path, digest in before.items():
        if path in changed_by_others:
            continue
        assert after.get(path) == digest, f"{path} changed or went: {digest} -> {after.get(path)}"
    new = sorted(set(after) - set(before))
    for path in new:
        assert any(re.fullmatch(pattern, path) for pattern in NEW_PLACES), f"{path} is not a new place"
        folder = re.match(r"(cases/[a-z0-9-]+/(runs|screens)/[0-9]{6})/", path)
        assert folder is None or folder[1] not in before, f"{path} was written into an existing folder"
    return new


def selections_of(root: Path, case_id: str) -> list[str]:
    folder = root / "cases" / case_id / "selections"
    return sorted(item.name for item in folder.iterdir()) if folder.is_dir() else []


def current_runs(root: Path, cases) -> dict[str, tuple]:
    return {case: (current_case(root, case).run_relative, current_case(root, case).export_relative) for case in cases}


def run_record(root: Path, case_id: str, folder: str) -> dict:
    return json.loads((root / "cases" / case_id / folder / "caepipe_out" / "run_record.json").read_text("utf-8"))


def assert_guest_clean(world: World) -> list[dict]:
    log = world.log()
    assert [entry["violations"] for entry in log if entry["violations"]] == []
    return log


# -- the live paths --------------------------------------------------------------------------------------------------
# name: (needs the runner, needs settings_guard, the case it writes, argv for a world, solve mode)
PATHS = {
    "writer_batch": (True, False, WRITER, lambda w: ["--case", WRITER], "batch"),
    "writer_print": (True, False, WRITER, lambda w: ["--case", WRITER], "print"),
    "export": (False, False, WRITER, lambda w: ["--export-only", "--case", WRITER], "batch"),
    "authored": (False, False, AUTHORED, lambda w: ["--authored", "--case", AUTHORED], "batch"),
    "authored_dir": (False, True, "cx-01-invented",
                     lambda w: ["--authored", "--authored-dir", str(w.root / "authored-private")], "batch"),
    "benchmark": (True, True, "cpb-g1-01", lambda w: w.benchmark(), "batch"),
}


def world_for(name: str, tmp_path: Path, monkeypatch) -> tuple[World, str, list[str]]:
    runner, guard, case_id, argv, mode = PATHS[name]
    if runner and not (RUNNER and Path(RUNNER).is_file()):
        pytest.skip(f"no runner executable ({run_pilot.RUNNER_ENV}); the SWBPIPE solve runs for real")
    world = World(tmp_path, monkeypatch, guard=guard, csv=invented_print() if name == "benchmark" else None)
    world.state["solve"]["mode"] = mode
    world.save()
    if name == "authored_dir":
        directory = world.root / "authored-private"
        put(directory / "cx-01-invented.mbf", b"INVENTED AUTHORED FIXTURE\r\ncx-01-invented\r\n")
        put(directory / "manifest.json", json.dumps({"document_kind": run_pilot.AUTHORED_MANIFEST_KIND,
                                                     "cases": [{"case_id": "cx-01-invented",
                                                                "file": "cx-01-invented.mbf"}]}))
    return world, case_id, argv(world)


@pytest.mark.parametrize("name", sorted(PATHS))
def test_a_live_path_succeeds_writing_only_new_files_and_publishes_only_a_complete_run(name, tmp_path, monkeypatch):
    world, case_id, argv = world_for(name, tmp_path, monkeypatch)
    before = snapshot(world.root)
    code, out = world.main(*argv)
    assert code == 0, out
    new = assert_append_only(before, snapshot(world.root))
    log = assert_guest_clean(world)
    view = current_case(world.root, case_id)
    if name == "export":
        # The legacy solve is read where it is; the export is a new run folder, published with the solve unchanged.
        assert view.run_relative is None and view.export_relative == "runs/000001"
        assert run_record(world.root, case_id, "runs/000001")["mbf_export"]["route"] == "mbf_exported"
        assert "solve" not in {entry["kind"] for entry in log}
    elif name == "authored":
        assert (view.run_relative, view.export_relative) == ("runs/000001", "runs/000002")
        assert selections_of(world.root, case_id) == ["000001.json", "000002.json"]
    else:
        assert view.run_relative == "runs/000001", out
        record = run_record(world.root, case_id, "runs/000001")
        assert record["route"] == ("csv_from_gui_print" if name == "writer_print" else "csv_from_batch")
        assert set(RUN_RECORD_MEMBERS) <= set(record)
        assert record["exchange_marker"] == "removed" and not (
            world.root / "cases" / case_id / "runs" / "000001" / "exchange_marker.txt").exists()
    assert not (world.root / "_harness" / "round_trip.lock").exists()
    assert any(path.startswith(f"cases/{case_id}/runs/000001/") for path in new)


REFUSALS = {
    # name: (what changes in the world, the route recorded)
    "drive_absent": ("the guest lists no drive", "exchange_drive_unavailable"),
    "other_content": ("the configured drive shows another folder holding a copy of the stored files",
                      "exchange_drive_unavailable"),
    "two_matches": ("the exchange folder is shown at the configured letter and at another", "exchange_drive_ambiguous"),
    "work_on_shared_drive": ("the guest's work folder is on a drive it lists as shared", "work_folder_on_exchange_drive"),
    "work_under_root": ("the guest's work folder lies under the exchange root", "work_folder_on_exchange_drive"),
    "hash_reported_differently": ("the guest reports another model hash", "input_hash_mismatch"),
}


def refuse(world: World, refusal: str, name: str) -> None:
    if refusal == "drive_absent":
        world.state["view"] = {}
    elif refusal == "other_content":
        other = world.tmp / "other-share"
        shutil.copytree(world.root, other, symlinks=True)
        world.state["view"] = {"Z:\\invented": str(other)}
    elif refusal == "two_matches":
        world.state["view"] = {"Z:\\invented": str(world.root), "Y:\\invented": str(world.root)}
    elif refusal == "work_on_shared_drive":
        world.state["listed"] = ["C:"]
    elif refusal == "work_under_root":
        world.config["guest_work_root"] = "Z:\\invented\\work"
        world.write_config()
    elif refusal == "hash_reported_differently":
        world.state["solve"]["report_sha"] = "1" * 64
        world.state["export"]["report_mod_sha"] = "1" * 64
    world.save()


@pytest.mark.parametrize("refusal", sorted(REFUSALS))
@pytest.mark.parametrize("name", sorted(PATHS))
def test_a_refused_live_path_changes_nothing_stored_and_records_why(name, refusal, tmp_path, monkeypatch):
    world, case_id, argv = world_for(name, tmp_path, monkeypatch)
    refuse(world, refusal, name)
    cases = (case_id, WRITER, SELECTED, AUTHORED)
    runs = current_runs(world.root, cases)
    before = snapshot(world.root)
    code, out = world.main(*argv)
    assert code == 1, out
    assert_append_only(before, snapshot(world.root))
    # No run or export is published (the series step after a refused case may still publish new screens).
    assert current_runs(world.root, cases) == runs
    log = assert_guest_clean(world)
    kinds = {entry["kind"] for entry in log}
    route = REFUSALS[refusal][1]
    record = run_record(world.root, case_id, "runs/000001")
    assert set(RUN_RECORD_MEMBERS) <= set(record)
    if route == "input_hash_mismatch":
        # The guest worked in its own folder; nothing was collected from it and the run is not published.
        assert not kinds & {"collect", "export_collect"}
        assert (record.get("mbf_export") or record)["route"] == route
    else:
        # Refused before any guest command but the drive listing and the marker probe (and the benchmark's
        # liveness and process checks before it).
        assert kinds <= {"net_use", "exchange_check", "ver", "tasklist"}, kinds
        assert record["route"] == route and route in REFUSED_ROUTES
        assert record["exchange_marker"] == "removed"
    assert not (world.root / "_harness" / "round_trip.lock").exists()


@needs_runner
def test_a_guest_that_fails_mid_step_leaves_an_unpublished_run_and_nothing_changed(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    world.state["solve"]["mode"] = "garbage"  # the guest script stops without a report, from the share and locally
    world.save()
    before = snapshot(world.root)
    code, out = world.main("--case", WRITER)
    assert code == 1
    assert_append_only(before, snapshot(world.root))
    assert current_case(world.root, WRITER).run_relative is None
    assert run_record(world.root, WRITER, "runs/000001")["route"] == "no_guest_json"
    kinds = [entry["kind"] for entry in assert_guest_clean(world)]
    assert "stage_script" in kinds and "collect" not in kinds


def test_the_export_without_a_bound_solve_asks_no_guest_and_changes_nothing(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    (world.root / "cases" / WRITER / "caepipe_out" / "model.mod").unlink()  # the legacy solve binds no model now
    before = snapshot(world.root)
    code, out = world.main("--export-only", "--case", WRITER)
    assert code == 1 and "export route=not_bound" in out
    assert_append_only(before, snapshot(world.root))
    assert world.log() == [] and selections_of(world.root, WRITER) == []
    record = run_record(world.root, WRITER, "runs/000001")
    assert record["route"] == "not_bound" and set(RUN_RECORD_MEMBERS) <= set(record)


# -- screens and the series step -------------------------------------------------------------------------------------
@needs_runner
def test_screens_and_the_series_step_write_only_new_files(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    before = snapshot(world.root)
    code, out = world.main("--offline", "--case", WRITER, "--case", SELECTED)
    assert code == 0, out
    new = assert_append_only(before, snapshot(world.root))
    assert world.log() == []  # no guest
    # Each screen is a new folder; the legacy case is read where it is, the selected case from its run.
    assert current_case(world.root, WRITER).screen_relative is not None
    screen = current_case(world.root, SELECTED)
    assert screen.run_relative == "runs/000001" and screen.screen_relative is not None
    read = json.loads((screen.screen / "screen_record.json").read_text("utf-8"))["read"]
    assert read["run"] == "runs/000001"
    # The series step adds the next numbered record; the legacy record and record 1 are unchanged.
    assert "series/shear-deformation/000002.json" in new


@needs_runner
def test_an_offline_benchmark_screen_writes_only_new_files(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch, guard=True, csv=invented_print())
    assert world.main(*world.benchmark())[0] == 0
    before = snapshot(world.root)
    code, out = world.main("--benchmark", str(world.tmp / "set" / "manifest.json"), "--case", "cpb-g1-01",
                           "--offline")
    assert code == 0, out
    assert_append_only(before, snapshot(world.root))
    assert current_case(world.root, "cpb-g1-01").screen_relative == "screens/000001"


# -- legacy reading and the one resolver -----------------------------------------------------------------------------
def test_a_case_without_a_selection_is_read_where_its_results_are(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    for case_id in (WRITER, AUTHORED):
        view = current_case(world.root, case_id)
        assert view.selection is None and view.run == view.export == world.root / "cases" / case_id
    assert run_pilot._stored_comparison(world.root, WRITER)["marker"] == "legacy"
    # The export step reads the legacy authored solve where it is and exports it into a new run folder.
    before = snapshot(world.root)
    code, out = world.main("--authored", "--export-only", "--case", AUTHORED)
    assert code == 0, out
    assert_append_only(before, snapshot(world.root))
    record = run_record(world.root, AUTHORED, "runs/000001")
    assert record["solve_run"]["folder"] is None and record["mbf_export"]["mod"]["path"] == "./caepipe_out/model.mod"
    assert current_case(world.root, AUTHORED).export_relative == "runs/000001"


@needs_runner
def test_while_a_selection_exists_no_reader_uses_the_legacy_files(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    (world.root / "cases" / SELECTED / "caepipe_out" / "model_results.csv").write_text("not a print\n")
    assert run_pilot._stored_comparison(world.root, SELECTED)["marker"] == "run 000001"
    code, out = world.main("--offline", "--case", SELECTED)
    assert code == 0, out
    view = current_case(world.root, SELECTED)
    read = json.loads((view.screen / "screen_record.json").read_text("utf-8"))["read"]
    run_csv = world.root / "cases" / SELECTED / "runs" / "000001" / "caepipe_out" / "model_results.csv"
    assert read["files"]["caepipe_out/model_results.csv"] == sha(run_csv.read_bytes())
    code, out = world.main("--export-only", "--case", SELECTED)
    assert code == 0, out
    record = run_record(world.root, SELECTED, current_case(world.root, SELECTED).export_relative)
    assert record["solve_run"]["folder"] == "runs/000001"
    assert record["mbf_export"]["mod"]["path"] == "runs/000001/caepipe_out/model.mod"


def break_selections(case_root: Path, how: str) -> None:
    if how == "gap":
        selection(case_root, 3, run="runs/000001")
    elif how == "sequence_mismatch":
        selection(case_root, 2, run="runs/000001")
        data = json.loads((case_root / "selections" / "000002.json").read_text())
        (case_root / "selections" / "000002.json").write_text(json.dumps({**data, "sequence": 3}))
    elif how == "conflicting_previous":
        selection(case_root, 2, run="runs/000001")
        data = json.loads((case_root / "selections" / "000002.json").read_text())
        (case_root / "selections" / "000002.json").write_text(json.dumps({**data, "previous": None}))
    elif how == "other_case":
        selection(case_root, 2, run="runs/000001")
        data = json.loads((case_root / "selections" / "000002.json").read_text())
        (case_root / "selections" / "000002.json").write_text(json.dumps({**data, "case_id": "cpc-01-other"}))
    elif how == "corrupt":
        put(case_root / "selections" / "000002.json", "{ not json")
    elif how == "unreadable":
        selection(case_root, 2, run="runs/000001")
        (case_root / "selections" / "000002.json").chmod(0)
    elif how == "absent_run":
        selection(case_root, 2, run="runs/000009")
    elif how == "incomplete_run":
        (case_root / "runs" / "000002" / "caepipe_out").mkdir(parents=True)
        selection(case_root, 2, run="runs/000002")
    elif how == "stray":
        put(case_root / "selections" / "notes.txt", "stray")


@needs_runner
@pytest.mark.parametrize("how", ["gap", "sequence_mismatch", "conflicting_previous", "other_case", "corrupt",
                                 "unreadable", "absent_run", "incomplete_run", "stray"])
def test_an_untrusted_selection_stops_every_reader_by_name_without_falling_back(how, tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    case_root = world.root / "cases" / SELECTED
    break_selections(case_root, how)
    try:
        with pytest.raises(SelectionError):
            current_case(world.root, SELECTED)
        before = snapshot(world.root)
        # A screen stops before writing anything; the legacy files beside it are not read.
        code, out = world.main("--offline", "--case", SELECTED)
        assert code == 1 and f"{SELECTED}: route=selection_untrusted" in out
        # The export step records the refusal in a new run folder of its own and asks no guest.
        code, out = world.main("--export-only", "--case", SELECTED)
        assert code == 1 and "export route=selection_untrusted" in out
        assert world.log() == []
        after = snapshot(world.root)
        assert_append_only(before, after)
        assert {path for path in set(after) - set(before) if "/screens/" in path} == set()
    finally:
        for item in (case_root / "selections").iterdir():
            item.chmod(0o600)


# -- links, races and changes by others -----------------------------------------------------------------------------
def hard_link(source: Path, target: Path) -> None:
    target.parent.mkdir(parents=True, exist_ok=True)
    os.link(source, target)


@needs_runner
def test_hard_links_from_stored_files_at_every_fixed_path_are_never_written_through(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    stored = world.root / "cases" / WRITER / "comparison" / "comparison.json"
    script = harness.GUEST_SCRIPT_NAME.format(sha256=sha(harness.GUEST_SCRIPT.read_bytes()))
    # Where this command will write fixed or next-numbered names: the next run folder of the case (made in advance,
    # with a link at every name the step writes), the next series record and the next selection of a case without
    # one are all links to one stored file.
    run = world.root / "cases" / WRITER / "runs" / "000001"
    for relative in ("exchange_marker.txt", "run_lock.json", "caepipe_in/model.mbf", "caepipe_in/id_map.json",
                     "caepipe_out/run_record.json", "caepipe_out/model.csv", "caepipe_out/guest_stdout.txt",
                     "comparison/comparison.json", "case_manifest.json"):
        hard_link(stored, run / relative)
    hard_link(stored, world.root / "series" / "shear-deformation" / "000002.json")
    before = snapshot(world.root)
    code, out = world.main("--case", WRITER)
    assert code == 0, out
    assert_append_only(before, snapshot(world.root))
    assert current_case(world.root, WRITER).run_relative == "runs/000002"
    assert (world.root / "series" / "shear-deformation" / "000003.json").is_file()
    # The guest script's content name and the run lock: a link to a stored file there refuses the step by name.
    for name, route in ((script, "exchange_check_failed"), ("round_trip.lock", "run_lock_held")):
        world_ = World(tmp_path / name, monkeypatch)
        hard_link(world_.root / "cases" / WRITER / "comparison" / "comparison.json", world_.root / "_harness" / name)
        before = snapshot(world_.root)
        code, out = world_.main("--case", WRITER)
        assert code == 1 and f"route={route}" in out, out
        assert_append_only(before, snapshot(world_.root))
        assert run_record(world_.root, WRITER, "runs/000001")["route"] == route
        assert world_.log() == [] if route == "run_lock_held" else True
    # A link to a stored file made in the step's own new run folder while the guest works, at a name the step writes
    # next: the step's file is never written through it.
    world_ = World(tmp_path / "during", monkeypatch)
    stored_ = world_.root / "cases" / WRITER / "comparison" / "comparison.json"
    out_dir = world_.root / "cases" / WRITER / "runs" / "000001" / "caepipe_out"
    world_.state["actions"] = [{"before": "collect", "op": "link", "source": str(stored_),
                                "path": str(out_dir / "guest_stdout.txt")}]
    world_.save()
    before = snapshot(world_.root)
    code, out = world_.main("--case", WRITER)
    assert code == 1 and "route=containment_violation" in out, out
    assert_append_only(before, snapshot(world_.root))
    assert current_case(world_.root, WRITER).run_relative is None
    # A selection that is a link to a stored file is untrusted: the case is not read and nothing is published.
    world_ = World(tmp_path / "selection", monkeypatch)
    hard_link(world_.root / "cases" / WRITER / "comparison" / "comparison.json",
              world_.root / "cases" / WRITER / "selections" / "000001.json")
    before = snapshot(world_.root)
    code, out = world_.main("--case", WRITER)
    assert code == 1 and "route=selection_untrusted" in out, out
    assert_append_only(before, snapshot(world_.root))


@needs_runner
def test_files_appearing_or_changing_during_the_step_are_left_as_the_other_writer_left_them(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    legacy = world.root / "cases" / WRITER / "comparison" / "comparison.json"
    out_dir = world.root / "cases" / WRITER / "runs" / "000001" / "caepipe_out"
    world.state["actions"] = [
        # Another writer changes a stored file while the guest solves, and puts a file where this step collects.
        {"before": "solve", "op": "write", "path": str(legacy), "data": "changed by another writer"},
        {"before": "solve", "op": "write", "path": str(out_dir / "model.csv"), "data": "put there by another"},
    ]
    world.save()
    before = snapshot(world.root)
    code, out = world.main("--case", WRITER)
    assert code == 1, out
    after = snapshot(world.root)
    rel_legacy = legacy.relative_to(world.root).as_posix()
    assert_append_only(before, after, changed_by_others={rel_legacy})
    assert legacy.read_text() == "changed by another writer"  # not reverted, not rewritten
    assert (out_dir / "model.csv").read_text() == "put there by another"
    # The step stopped before collecting into a folder that was not empty; the guest copied nothing; nothing is
    # published; the exceptional end is recorded.
    kinds = [entry["kind"] for entry in assert_guest_clean(world)]
    assert "collect" not in kinds and current_case(world.root, WRITER).run_relative is None
    record = run_record(world.root, WRITER, "runs/000001")
    assert record["route"] == "harness_error" and set(RUN_RECORD_MEMBERS) <= set(record)

    # The run record's name taken by another writer during the step: the step's record cannot be written; it says
    # so, leaves the other file as it is and publishes nothing.
    world = World(tmp_path / "record", monkeypatch)
    taken = world.root / "cases" / WRITER / "runs" / "000001" / "caepipe_out" / "run_record.json"
    world.state["actions"] = [{"before": "collect", "op": "write", "path": str(taken), "data": "another record"}]
    world.save()
    before = snapshot(world.root)
    code, out = world.main("--case", WRITER)
    assert code == 1 and "route=containment_violation" in out, out
    assert_append_only(before, snapshot(world.root))
    assert taken.read_text() == "another record" and current_case(world.root, WRITER).run_relative is None


@needs_runner
def test_races_for_run_folders_and_selections_take_the_next_number(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    real_reserve, real_link = exclusive.Tree.reserve_dir, exclusive.Tree.link_new
    raced = []

    def reserve_dir(self, parts):  # another command takes the same run folder first
        if parts[-2] == "runs" and not raced:
            raced.append(parts)
            os.mkdir(self.path(parts))
        return real_reserve(self, parts)

    def link_new(self, source, target):  # another command publishes the same selection number first
        if target[-2] == "selections" and len(raced) == 1:
            raced.append(target)
            solve_folder(world.root / "cases" / WRITER / "runs" / "000009", mbf_text(WRITER),
                         PARSER_FIXTURE.read_text(), marker="another command's run")
            selection(world.root / "cases" / WRITER, int(target[-1][:6]), run="runs/000009")
        return real_link(self, source, target)

    monkeypatch.setattr(exclusive.Tree, "reserve_dir", reserve_dir)
    monkeypatch.setattr(exclusive.Tree, "link_new", link_new)
    before = snapshot(world.root)
    code, out = world.main("--case", WRITER)
    assert code == 0, out
    assert_append_only(before, snapshot(world.root), changed_by_others=set())
    assert (world.root / "cases" / WRITER / "runs" / "000001").is_dir()
    assert list((world.root / "cases" / WRITER / "runs" / "000001").iterdir()) == []  # the other command's
    selections = selections_of(world.root, WRITER)
    assert selections[:2] == ["000001.json", "000002.json"]
    second = json.loads((world.root / "cases" / WRITER / "selections" / "000002.json").read_text())
    assert second["run"] == "runs/000002" and second["previous"] == "selections/000001.json"


def test_the_marker_and_the_lock_are_removed_only_while_they_hold_this_steps_content(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    marker = world.root / "cases" / AUTHORED / "runs" / "000001" / "exchange_marker.txt"
    # Another writer replaces the marker before the probe; in a second run, the lock during the solve.
    world.state["actions"] = [{"before": "net_use", "op": "write", "path": str(marker), "data": "another marker"}]
    world.save()
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1 and "route=exchange_drive_unavailable" in out
    assert marker.read_text() == "another marker"
    record = run_record(world.root, AUTHORED, "runs/000001")
    assert record["exchange_marker"].startswith("left: ")
    world = World(tmp_path / "lock", monkeypatch)
    lock = world.root / "_harness" / "round_trip.lock"
    world.state["actions"] = [{"before": "solve", "op": "write", "path": str(lock), "data": "another lock"}]
    world.save()
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 0, out
    assert lock.read_text() == "another lock"  # left: it is not this step's
    run_lock = json.loads((world.root / "cases" / AUTHORED / "runs" / "000001" / "run_lock.json").read_text())
    assert run_lock["outcome"] == "left: its content is not the content this step wrote"


def test_the_drive_check_reads_only_the_marker_on_each_letter_then_inputs_under_the_selected_root(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    world.state["view"]["Y:\\invented"] = str(world.tmp / "elsewhere")
    world.state["listed"] = ["X:"]
    world.save()
    (world.tmp / "elsewhere").mkdir()
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 0, out
    import base64
    checks = []
    for entry in world.log():
        if entry["kind"] == "exchange_check":
            script = base64.b64decode(entry["command"].rsplit(" ", 1)[1]).decode("utf-16-le")
            checks.append(json.loads(base64.b64decode(re.search(r"FromBase64String\('([^']*)'\)", script)[1])))
    marker = "\\invented\\cases\\cpa-05-guide\\runs\\000001\\exchange_marker.txt"
    assert sorted(item["path"] for item in checks[0]) == [f"{letter}:{marker}" for letter in "XYZ"]
    assert sorted(item["path"] for item in checks[1]) == sorted([
        "Z:\\invented\\cases\\cpa-05-guide\\runs\\000001\\caepipe_in\\model.mbf",
        f"Z:\\invented\\_harness\\{harness.GUEST_SCRIPT_NAME.format(sha256=sha(harness.GUEST_SCRIPT.read_bytes()))}"])
    assert len(checks) == 2
    record = run_record(world.root, AUTHORED, "runs/000001")
    assert record["exchange_check"]["listing"]["letters_tested"] == ["X:", "Y:", "Z:"]
    assert record["exchange_check"]["letters_matched"] == ["Z:"]


def test_duplicate_or_conflicting_answers_are_never_a_match(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    world.state["answer_twice"] = True
    world.save()
    before = snapshot(world.root)
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1 and "route=exchange_drive_unavailable" in out
    assert_append_only(before, snapshot(world.root))
    assert run_record(world.root, AUTHORED, "runs/000001")["exchange_check"]["marker_answers"] == {"Z:": "conflicting"}


# -- crashes at every write boundary ---------------------------------------------------------------------------------
class Crash:
    """Fail the n-th exclusive-layer operation (a file creation, a folder reservation or a selection link): `partial`
    writes the first half of a file first, as a crash in the middle of the write would leave it."""

    def __init__(self, monkeypatch, n: int, partial: bool):
        self.n, self.partial, self.count, self.at = n, partial, 0, None
        real_create, real_reserve, real_link = exclusive.Tree.create, exclusive.Tree.reserve_dir, \
            exclusive.Tree.link_new

        def boundary(parts) -> bool:
            self.count += 1
            if self.count == self.n:
                self.at = "/".join(parts)
            return self.count == self.n

        def create(tree, parts, data):
            if boundary(parts):
                payload = data.encode("utf-8") if isinstance(data, str) else data
                if self.partial:
                    real_create(tree, parts, payload[:len(payload) // 2])
                raise OSError(errno.EIO, f"injected crash at write boundary {self.n}")
            return real_create(tree, parts, data)

        def reserve_dir(tree, parts):
            if boundary(parts):
                raise OSError(errno.EIO, f"injected crash at write boundary {self.n}")
            return real_reserve(tree, parts)

        def link_new(tree, source, target):
            if boundary(target):
                raise OSError(errno.EIO, f"injected crash at write boundary {self.n}")
            return real_link(tree, source, target)

        monkeypatch.setattr(exclusive.Tree, "create", create)
        monkeypatch.setattr(exclusive.Tree, "reserve_dir", reserve_dir)
        monkeypatch.setattr(exclusive.Tree, "link_new", link_new)


def crash_everywhere(world: World, monkeypatch, argv: list[str], case_id: str, partial: bool) -> int:
    """Run the command once per write boundary, failing that boundary; returns how many boundaries there were."""
    n = 1
    while True:
        selections = selections_of(world.root, case_id)
        before = snapshot(world.root)
        crash = Crash(monkeypatch, n, partial)
        code, out = world.main(*argv)
        monkeypatch.undo()
        monkeypatch.setenv(STATE_ENV, str(world.state_path))
        monkeypatch.setattr(run_pilot, "time", SimpleNamespace(sleep=world.sleeps.append))
        after = snapshot(world.root)
        assert_append_only(before, after)
        if crash.count < n:  # this run had fewer boundaries than n: every boundary has failed once
            assert code == 0, out
            return n - 1
        # The one write whose failure the step absorbs by design: the lock's outcome record (`run_lock.json`); the
        # outcome is still returned, with a note that its record was not written.
        assert code != 0 or crash.at.endswith("/run_lock.json"), (n, crash.at, out)
        new_selections = sorted(set(selections_of(world.root, case_id)) - set(selections))
        # A crash before the selection's link leaves no selection; a selection that appears is complete.
        for name in new_selections:
            json.loads((world.root / "cases" / case_id / "selections" / name).read_text())
        # The lock and the guest script are written whole before they are linked to their names: a crash leaves
        # neither a lock nor a partial script behind, so the next command is never stuck.
        assert not (world.root / "_harness" / "round_trip.lock").exists(), n
        script = world.root / "_harness" / harness.GUEST_SCRIPT_NAME.format(sha256=sha(harness.GUEST_SCRIPT.read_bytes()))
        assert not script.exists() or script.read_bytes() == harness.GUEST_SCRIPT.read_bytes(), n
        n += 1


@needs_runner
@pytest.mark.parametrize("partial", [False, True])
def test_a_crash_at_any_write_boundary_of_a_live_step_changes_nothing_and_a_later_command_proceeds(
        partial, tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    boundaries = crash_everywhere(world, monkeypatch, ["--case", WRITER], WRITER, partial)
    assert boundaries >= 15
    # The run that completed last is the one published; every crashed run is a folder of its own, never selected.
    runs = sorted(item.name for item in (world.root / "cases" / WRITER / "runs").iterdir())
    assert current_case(world.root, WRITER).run_relative == f"runs/{runs[-1]}"
    assert len(runs) >= boundaries - 1
    assert_guest_clean(world)


@needs_runner
@pytest.mark.parametrize("partial", [False, True])
def test_a_crash_at_any_write_boundary_of_a_screen_changes_nothing_and_a_later_screen_proceeds(
        partial, tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    boundaries = crash_everywhere(world, monkeypatch, ["--offline", "--case", SELECTED], SELECTED, partial)
    assert boundaries >= 8
    view = current_case(world.root, SELECTED)
    assert view.run_relative == "runs/000001" and view.screen_relative is not None


# -- the capture-only place ------------------------------------------------------------------------------------------
def test_outputs_a_record_marks_sealed_are_never_read(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    out = world.root / "cases" / WRITER / "caepipe_out"
    record = json.loads((out / "run_record.json").read_text())
    (out / "run_record.json").write_text(json.dumps({**record, "sealed": True}))
    csv_path, route, _ = run_pilot.bound_csv(world.root, {"root": out.parent}, {**record, "sealed": True},
                                             record["mbf"]["sha256"])
    assert (csv_path, route) == (None, "sealed")
    assert harness.solved_model_binding({**record, "sealed": True}, record["mbf"]["sha256"], out / "model.mod")
    before = snapshot(world.root)
    code, msg = world.main("--export-only", "--case", WRITER)
    assert code == 1 and "export route=not_bound" in msg and "sealed" in msg
    assert world.log() == []
    assert_append_only(before, snapshot(world.root))
