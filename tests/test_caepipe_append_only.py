#!/usr/bin/env python3
"""The append-only rule of the CAEPIPE pilot's live steps and screens (DEL-17-05 REQ-022, AC-012, VER-012), checked
through the real command.

Every property test enters through `run_pilot.main` (or the clearance command's `main`) with its real arguments, over
a populated invented exchange folder made in a temporary directory, with the guest substituted only where the
virtual machine tool is called: a fake `prlctl` first on PATH (tests/caepipe_fake_guest.py), whose file operations
are modelled as the real guest performs them (`copy /Y` overwrites through a link; the harness's create-new copy
never writes over a name). No step is replaced: the SWBPIPE solve (the runner executable), the MBF writer, the
harness, the parser, the comparison, the export (refused before it starts anything) and the series step all run. The
only other substitutions are the
benchmark's fixed retry wait (the pilot's `time`), a recording spy that calls through to the reader it watches, and,
in the crash, race and record-failure tests, a wrapper around one exclusive-layer operation that fails or races it
and otherwise calls it. An abrupt end is a real SIGKILL of the harness process, which runs as a child process.

The property: for every live path and screen, on success, refusal, error and crash, every file and folder that
existed before the command is still there with the same SHA-256 (a deletion fails the check as surely as a change),
and everything new lies in a new run, screen or series file of its own, a new selection, the content-named guest
script, the lock file (created once, empty) or a holder record.

What this cannot show: the substituted guest shows the decisions the harness makes and what it does with the guest's
answers; it does not run the guest script. Whether the real program's prompts are visible to the script's way of
looking for windows, and whether the guest's file system resolves links as reported, are shown only by a rehearsal in
the real guest, which is not part of this check.

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
import signal
import stat
import subprocess
import sys
import time
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
    STORED_SOURCE,
    ContainmentError,
    SelectionError,
    current_case,
)
from tests import caepipe_fake_guest as fake  # noqa: E402
from tests.caepipe_fake_guest import STATE_ENV, install, read_log  # noqa: E402
from tests.test_caepipe_benchmark import benchmark_manifest, invented_print  # noqa: E402
from tools.caepipe_correlation import clear_settings_stop, run_pilot  # noqa: E402


# Reads its directory scans stand for, for CI selection (tools/ci/python_test_reads.py). It lists only the invented
# exchange folder and guest folders each test makes in a temporary directory, outside the project.
CI_READS = ()

PARSER_FIXTURE = ROOT / "fixtures" / "caepipe_external" / "invented" / "print_to_file_results.csv"
RUNNER = os.environ.get(run_pilot.RUNNER_ENV)
HAVE_RUNNER = bool(RUNNER and Path(RUNNER).is_file())
needs_runner = pytest.mark.skipif(not HAVE_RUNNER, reason=f"no runner executable ({run_pilot.RUNNER_ENV}); the "
                                                          "SWBPIPE solve runs for real")
GUARD = {"key": "HKCU\\Software\\Invented Vendor\\Tool\\1.0", "recent_files_subkey": "Recent list"}
RESTORING_GUARD = {**GUARD, "restore_recent_files": True}
WRITER = "cpc-01-cantilever-force"   # stored in the legacy layout (tranche 1)
SELECTED = "cpc-07-cantilever-span-3m"  # stored in a run folder named by a selection
AUTHORED = "cpa-05-guide"            # stored in the legacy layout with an export beside its solve


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


def solve_folder(folder: Path, mbf: str | bytes, csv: str, *, marker: str, case_id: str | None = None,
                 run: str | None = None, sealed: bool = False) -> None:
    """A stored solve in `folder` (a case's own folder, or the run folder `run`): its MBF, a CSV, a solved model and
    a run record binding them, and a comparison record."""
    data = mbf.encode("utf-8") if isinstance(mbf, str) else mbf
    digest = put(folder / "caepipe_in" / "model.mbf", data)
    outputs = {"model_results.csv": put(folder / "caepipe_out" / "model_results.csv", csv),
               "model.mod": put(folder / "caepipe_out" / "model.mod", b"INVENTED STORED SOLVED MODEL " + marker.encode())}
    put(folder / "caepipe_out" / "run_record.json", json.dumps({
        "document_kind": RUN_RECORD_KIND, "route": "csv_from_gui_print", "mbf": {"sha256": digest},
        "guest_mbf_sha256": digest, "observed_csv_path": {"host": "caepipe_out/model_results.csv"},
        "outputs_collected": sorted(outputs), "outputs_sha256": outputs, "sealed": sealed,
        **({"step": "round_trip", "case_id": folder.parents[1].name, "run_folder": run, "input_hash_verified": True}
           if run else {})}))
    if case_id:
        put(folder / "comparison" / "comparison.json", json.dumps(stored_comparison(case_id, marker)))
        put(folder / "parsed" / "caepipe_results.json", json.dumps({"invented": marker}))
        put(folder / "case_manifest.json", json.dumps({"invented": marker}))


def selection(case_root: Path, number: int, **members) -> None:
    put(case_root / "selections" / f"{number:06d}.json", json.dumps({
        "document_kind": SELECTION_KIND, "case_id": case_root.name, "sequence": number,
        "run": STORED_SOURCE, "export": None, "screen": None, "verified": {},
        "previous": f"selections/{number - 1:06d}.json" if number > 1 else None, "reason": "invented", **members}))


def populate(root: Path) -> None:
    """Stored results as earlier tranches left them, and as this layout keeps them."""
    csv = PARSER_FIXTURE.read_text(encoding="utf-8")
    # Tranche 1: a tracked case in the legacy layout (its own layer folders, no selection).
    solve_folder(root / "cases" / WRITER, mbf_text(WRITER), csv, marker="legacy", case_id=WRITER)
    # A case with a selected run, and older legacy files beside it that no reader may use while a selection exists.
    case = root / "cases" / SELECTED
    solve_folder(case, mbf_text(SELECTED), csv, marker="legacy", case_id=SELECTED)
    solve_folder(case / "runs" / "000001", mbf_text(SELECTED), csv, marker="run 000001", case_id=SELECTED,
                 run="runs/000001")
    selection(case, 1, run="runs/000001")
    # An authored case with its export stored beside its solve (legacy layout).
    authored = next(item for item in run_pilot.load_authored_cases() if item["case_id"] == AUTHORED)
    folder = root / "cases" / AUTHORED
    solve_folder(folder, run_pilot.authored_mbf_bytes(authored), csv, marker="legacy authored")
    put(folder / "caepipe_out" / "exported.mbf", b"INVENTED\r\nSTORED EXPORT\r\n")
    # The harness's folder as an earlier version left it, and series records.
    put(root / "_harness" / f"guest-script-{'0' * 64}.ps1", "# an earlier invented guest script\r\n")
    put(root / "series" / "shear-deformation.json", json.dumps({"invented": "legacy series record"}))
    put(root / "series" / "shear-deformation" / "000001.json", json.dumps({"invented": "series record 1"}))


class World:
    """A temporary invented exchange folder, a guest work folder and the invented guest behind a fake prlctl."""

    def __init__(self, tmp_path: Path, monkeypatch, *, guard: dict | None = None, csv: str | None = None,
                 config: bool = True, guest_on_path: bool = True):
        self.tmp = tmp_path
        self.monkeypatch = monkeypatch
        self.root = tmp_path / "exchange"
        self.root.mkdir(parents=True)
        self.root.chmod(0o700)
        put(self.root / ".gitignore", "*\n")
        self.config = {"vm_name": "invented-vm", "guest_executable": "C:\\Invented\\tool.exe",
                       "guest_exchange_root": "Z:\\invented", "guest_work_root": "C:\\invented-work",
                       "license_and_environment_responsibility_acknowledged": True,
                       **({"settings_guard": guard} if guard else {})}
        if config:
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
        path = os.environ.get("PATH", "")
        self.environ = {ENV_VAR: str(self.root),
                        "PATH": f"{tmp_path / 'bin'}{os.pathsep}{path}" if guest_on_path else path}
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

    def main_in_process_of_its_own(self, *argv: str, timeout: float = 300) -> subprocess.CompletedProcess:
        """The command in a process of its own, which the substituted guest may kill (SIGKILL) as a crash would."""
        code = ("import sys; sys.path.insert(0, %r)\n"
                "from tools.caepipe_correlation import run_pilot\n"
                "raise SystemExit(run_pilot.main(sys.argv[1:]))\n" % str(ROOT))
        env = {**os.environ, **self.environ, STATE_ENV: str(self.state_path)}
        return subprocess.run([sys.executable, "-c", code, *argv], capture_output=True, text=True, env=env,
                              timeout=timeout, cwd=str(ROOT))

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
              r"_harness(/guest-script-[0-9a-f]{64}\.ps1|/round_trip\.lock|/holders(/[0-9]{6}\.json)?)?")


def assert_append_only(before: dict[str, str], after: dict[str, str], *, changed_by_others=(),
                       new_in_existing=()) -> list[str]:
    """Everything that existed is unchanged (a deletion fails as a change does); everything new lies in a new run
    or screen folder of its own, a new selection, a new series record, the content-named guest script, the lock file
    or a holder record. Returns the new paths."""
    for path, digest in before.items():
        if path in changed_by_others:
            continue
        assert after.get(path) == digest, f"{path} changed or went: {digest} -> {after.get(path)}"
    new = sorted(set(after) - set(before))
    for path in new:
        assert any(re.fullmatch(pattern, path) for pattern in NEW_PLACES), f"{path} is not a new place"
        folder = re.match(r"(cases/[a-z0-9-]+/(runs|screens)/[0-9]{6})/", path)
        assert folder is None or folder[1] not in before or path in new_in_existing, \
            f"{path} was written into an existing folder"
    if "_harness/round_trip.lock" in after:
        assert after["_harness/round_trip.lock"] == sha(b""), "the lock file holds content"
    return new


def selections_of(root: Path, case_id: str) -> list[str]:
    folder = root / "cases" / case_id / "selections"
    return sorted(item.name for item in folder.iterdir()) if folder.is_dir() else []


def current_runs(root: Path, cases) -> dict[str, tuple]:
    return {case: (current_case(root, case).run_relative, current_case(root, case).export_relative) for case in cases}


def run_record(root: Path, case_id: str, folder: str) -> dict:
    return json.loads((root / "cases" / case_id / folder / "caepipe_out" / "run_record.json").read_text("utf-8"))


def newest_run(root: Path, case_id: str) -> str:
    return "runs/" + max(path.name for path in (root / "cases" / case_id / "runs").iterdir())


def assert_guest_clean(world: World) -> list[dict]:
    log = world.log()
    assert [entry["violations"] for entry in log if entry["violations"]] == []
    return log


# Guest commands that delete, copy or start the program; none may come before the work directory is shown separate.
WRITING_KINDS = {"prepare", "solve", "stage_script", "collect", "export_prepare", "export", "export_stage_script",
                 "export_collect", "copy", "reg_write"}
READ_ONLY_KINDS = {"net_use", "exchange_check", "ver", "tasklist", "processes", "reg_query", "type"}


# -- the live paths --------------------------------------------------------------------------------------------------
# name: (needs the runner, the settings guard it needs, the case it writes, argv for a world, solve mode). The export
# and the print fallback through the program's window are refused before anything is started (their own tests below).
PATHS = {
    "writer_batch": (True, None, WRITER, lambda w: ["--case", WRITER], "batch"),
    "authored": (False, None, AUTHORED, lambda w: ["--authored", "--no-export", "--case", AUTHORED], "batch"),
    "authored_dir": (False, GUARD, "cx-01-invented",
                     lambda w: ["--authored", "--authored-dir", str(w.root / "authored-private")], "batch"),
    "benchmark": (True, GUARD, "cpb-g1-01", lambda w: w.benchmark(), "batch"),
}


def world_for(name: str, tmp_path: Path, monkeypatch) -> tuple[World, str, list[str]]:
    runner, guard, case_id, argv, mode = PATHS[name]
    if runner and not HAVE_RUNNER:
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
    assert view.run_relative == "runs/000001", out
    record = run_record(world.root, case_id, "runs/000001")
    assert record["route"] == "csv_from_batch"
    assert set(RUN_RECORD_MEMBERS) <= set(record)
    assert record["exchange_marker"] == "kept in the run folder"
    assert (world.root / "cases" / case_id / "runs" / "000001" / "exchange_marker.txt").is_file()
    # The documented batch pattern alone: the guest script is always told not to fall back to the program's window.
    assert all("-NoGuiFallback" in entry["command"] for entry in log if entry["kind"] == "solve")
    # Nothing in the guest deleted, copied or started anything before the work directory was shown separate.
    kinds = [entry["kind"] for entry in log]
    first_write = min((kinds.index(kind) for kind in WRITING_KINDS if kind in kinds), default=len(kinds))
    assert {"work_root", "work_directory"} <= set(kinds[:first_write]) or first_write == len(kinds)
    assert kinds.index("work_root") < kinds.index("work_directory")
    assert not any(entry["kind"] == "copy" or " del " in entry.get("command", "") for entry in log)
    assert (world.root / "_harness" / "round_trip.lock").read_bytes() == b""
    assert any(path.startswith(f"cases/{case_id}/runs/000001/") for path in new)


REFUSALS = {
    # name: (what changes in the world, the route recorded)
    "drive_absent": ("the guest lists no drive", "exchange_drive_unavailable"),
    "other_content": ("the configured drive shows another folder holding a copy of the stored files",
                      "exchange_drive_unavailable"),
    "two_matches": ("the exchange folder is shown at the configured letter and at another", "exchange_drive_ambiguous"),
    "work_on_shared_drive": ("the guest's work folder is on a drive it lists as shared", "work_folder_on_exchange_drive"),
    "work_under_root": ("the guest's work folder lies under the exchange root", "work_folder_on_exchange_drive"),
    "work_aliased": ("the guest's work root resolves into the exchange folder", "work_folder_on_exchange_drive"),
    "work_root_absent": ("the configured work root does not exist in the guest", "work_folder_on_exchange_drive"),
    "hash_reported_differently": ("the guest reports another model hash", "input_hash_mismatch"),
}


def refuse(world: World, refusal: str) -> None:
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
    elif refusal == "work_aliased":  # the work root's spelling is local; it resolves into the exchange folder
        world.state["work"] = {"C:\\invented-work": str(world.root / "cases")}
    elif refusal == "work_root_absent":  # nothing is made for it: not the root, not the step's directory
        world.state["work"] = {"C:\\invented-work": str(world.tmp / "absent-work-root")}
    elif refusal == "hash_reported_differently":
        world.state["solve"]["report_sha"] = "1" * 64
        world.state["export"]["report_mod_sha"] = "1" * 64
    world.save()


@pytest.mark.parametrize("refusal", sorted(REFUSALS))
@pytest.mark.parametrize("name", sorted(PATHS))
def test_a_refused_live_path_changes_nothing_stored_and_records_why(name, refusal, tmp_path, monkeypatch):
    world, case_id, argv = world_for(name, tmp_path, monkeypatch)
    refuse(world, refusal)
    cases = (case_id, WRITER, SELECTED, AUTHORED)
    runs = current_runs(world.root, cases)
    before = snapshot(world.root)
    code, out = world.main(*argv)
    assert code == 1, out
    after = snapshot(world.root)
    # The work root is resolved before anything is created for the step: an aliased or absent one leaves nothing.
    assert_append_only(before, after)
    assert not (world.tmp / "absent-work-root").exists()
    # No run or export is published (the series step after a refused case may still publish new screens).
    assert current_runs(world.root, cases) == runs
    log = assert_guest_clean(world)
    kinds = {entry["kind"] for entry in log}
    route = REFUSALS[refusal][1]
    record = run_record(world.root, case_id, "runs/000001")
    assert set(RUN_RECORD_MEMBERS) <= set(record)
    if route == "input_hash_mismatch":
        assert not kinds & {"collect", "export_collect"}
        assert (record.get("mbf_export") or record)["route"] == route
    else:
        # Refused before any guest command that deletes, copies or starts the program.
        assert not kinds & WRITING_KINDS, kinds
        assert record["route"] == route and route in REFUSED_ROUTES
        assert record["exchange_marker"] in ("kept in the run folder", "not created")
        if refusal in ("work_aliased", "work_root_absent"):
            assert record["work_folder_check"]["created"] is False and "work_directory" not in kinds


@pytest.mark.parametrize("variant", ["child_aliased", "destination_linked", "work_file_linked"])
def test_a_work_directory_that_reaches_the_exchange_folder_is_refused_before_anything_is_copied_or_started(
        variant, tmp_path, monkeypatch):
    world, case_id, argv = world_for("authored", tmp_path, monkeypatch)
    stored = world.root / "cases" / WRITER / "caepipe_out" / "model_results.csv"
    if variant == "child_aliased":  # the configured work root is a child path that is a link into the exchange folder
        (world.work / "linked").symlink_to(world.root / "cases" / WRITER, target_is_directory=True)
        world.config["guest_work_root"] = "C:\\invented-work\\linked"
        world.write_config()
    elif variant == "destination_linked":  # an output destination of the program shows in the new directory as a link
        world.state["fresh_dir_plant"] = [{"name": "model.mbf", "link_to": str(stored), "kind": "symlink"}]
    else:  # a work file linked to stored content
        world.state["fresh_dir_plant"] = [{"name": "model.mod", "link_to": str(stored), "kind": "hardlink"}]
    world.save()
    before = snapshot(world.root)
    code, out = world.main(*argv)
    assert code == 1, out
    assert_append_only(before, snapshot(world.root))  # nothing is made in the exchange folder in any variant
    record = run_record(world.root, case_id, "runs/000001")
    assert record["route"] == "work_folder_on_exchange_drive", record["route"]
    assert record["work_folder_check"]["created"] is (variant != "child_aliased")
    assert not {entry["kind"] for entry in world.log()} & WRITING_KINDS


def test_a_work_directory_that_looks_separate_but_is_not_is_refused_and_the_two_items_it_left_are_named(
        tmp_path, monkeypatch):
    # The residual case: the guest's file system reports a local final path for its work root, which in fact lies in
    # the exchange folder; only the content probe shows it. The step made exactly its fresh directory and the probe
    # there, names both in its refusal by their paths relative to the exchange folder, and removes neither.
    world = World(tmp_path, monkeypatch)
    stray = world.root / "invented-guest-work"
    stray.mkdir()
    world.state["work"] = {"C:\\invented-work": str(stray)}
    world.state["final_local"] = True
    world.save()
    before = snapshot(world.root)
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1 and "route=work_folder_on_exchange_drive" in out, out
    after = snapshot(world.root)
    record = run_record(world.root, AUTHORED, "runs/000001")
    check = record["work_folder_check"]
    left = check["left_in_exchange_folder"]
    assert check["status"] == "inside" and check["created"] is True
    assert re.fullmatch(rf"invented-guest-work/{AUTHORED}_run-000001-[0-9a-f]{{16}}", left["directory"])
    assert left["probe"] == f"{left['directory']}/{check['probe']}"
    assert left["directory"] in out and left["probe"] in out and "nothing removes them" in out
    # Exactly the directory and its probe are new outside the step's own places; both are still there.
    stray_new = {path for path in set(after) - set(before) if path.startswith("invented-guest-work/")}
    assert stray_new == {left["directory"], left["probe"]}
    assert_append_only(before, {path: digest for path, digest in after.items() if path not in stray_new})
    assert not {entry["kind"] for entry in world.log()} & WRITING_KINDS


def test_two_steps_never_share_a_work_directory(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    for _ in range(2):
        assert world.main("--authored", "--no-export", "--case", AUTHORED)[0] == 0
    directories = sorted(path.name for path in world.work.iterdir())  # directly in the work root, nothing else made
    assert len(directories) == 2 and all(re.fullmatch(rf"{AUTHORED}_run-00000[12]-[0-9a-f]{{16}}", name)
                                         for name in directories)
    for sequence in ("000001", "000002"):
        check = run_record(world.root, AUTHORED, f"runs/{sequence}")["work_folder_check"]
        assert check["status"] == "separate" and check["entries"] == [check["probe"]]


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


# -- the two routes that operate the program's windows: refused before anything is started ---------------------------
EXPORT_KINDS = {"export_prepare", "export", "export_stage_script", "export_collect"}
EXPORT_COMMANDS = {
    # name: (needs the runner, case, argv, whether a round trip runs first in the same command)
    "export_only": (False, WRITER, ["--export-only", "--case", WRITER], False),
    "export_only_selected": (False, SELECTED, ["--export-only", "--case", SELECTED], False),
    "authored_default": (False, AUTHORED, ["--authored", "--case", AUTHORED], True),
    "authored_export_only": (False, AUTHORED, ["--authored", "--export-only", "--case", AUTHORED], False),
    "export_mbf_flag": (True, WRITER, ["--case", WRITER, "--export-mbf"], True),
}


@pytest.mark.parametrize("name", sorted(EXPORT_COMMANDS))
def test_the_export_is_refused_by_name_before_anything_is_started_from_every_command(name, tmp_path, monkeypatch):
    runner, case_id, argv, round_trip_first = EXPORT_COMMANDS[name]
    if runner and not HAVE_RUNNER:
        pytest.skip(f"no runner executable ({run_pilot.RUNNER_ENV}); the SWBPIPE solve runs for real")
    world = World(tmp_path, monkeypatch)
    before = snapshot(world.root)
    exports_before = current_case(world.root, case_id).export_relative
    code, out = world.main(*argv)
    assert code == 1, out
    assert f"{case_id}: export route=window_route_unavailable" in out and "DEL-17-05 REQ-023" in out, out
    assert_append_only(before, snapshot(world.root))
    folder = newest_run(world.root, case_id)
    record = run_record(world.root, case_id, folder)
    assert record["step"] == "mbf_export" and record["route"] == "window_route_unavailable"
    assert record["mbf_export"]["route"] == "window_route_unavailable" and set(RUN_RECORD_MEMBERS) <= set(record)
    # A complete record in the step's new run folder and nothing else: no marker, no lock holder, no guest command.
    run = world.root / "cases" / case_id / folder
    assert sorted(path.relative_to(run).as_posix() for path in run.rglob("*") if path.is_file()) == \
        ["caepipe_out/run_record.json"]
    log = assert_guest_clean(world)
    assert not {entry["kind"] for entry in log} & EXPORT_KINDS
    assert sum(entry["kind"] == "work_directory" for entry in log) == (1 if round_trip_first else 0)
    if not round_trip_first:
        assert log == []
    if round_trip_first:  # the round trip itself ran and was published; only the export is refused
        assert current_case(world.root, case_id).run_relative == "runs/000001"
    assert current_case(world.root, case_id).export_relative == exports_before  # no export is published


def test_the_print_fallback_is_refused_by_name_before_anything_is_started(tmp_path, monkeypatch):
    # No command asks for it; asked for through the harness, it is refused before any guest command.
    world = World(tmp_path, monkeypatch)
    before = snapshot(world.root)
    config = harness.resolve_harness_config(world.environ)
    text = run_pilot.authored_mbf_bytes(next(item for item in run_pilot.load_authored_cases()
                                             if item["case_id"] == AUTHORED)).decode("ascii")
    record = harness.run_caepipe_round_trip(config, AUTHORED, text, gui_fallback=True)
    assert record["route"] == "window_route_unavailable" and "DEL-17-05 REQ-023" in record["reason"]
    assert world.log() == []
    assert_append_only(before, snapshot(world.root))
    stored = run_record(world.root, AUTHORED, record["run_folder"])
    assert stored["route"] == "window_route_unavailable" and set(RUN_RECORD_MEMBERS) <= set(stored)
    run = world.root / "cases" / AUTHORED / record["run_folder"]
    assert sorted(path.relative_to(run).as_posix() for path in run.rglob("*") if path.is_file()) == \
        ["caepipe_out/run_record.json"]


ROUND_TRIP_COMMANDS = {
    # name: (needs the runner, the settings guard, case, argv for a world)
    "authored": (False, None, AUTHORED, lambda w: ["--authored", "--no-export", "--case", AUTHORED]),
    "authored_dir": (False, GUARD, "cx-01-invented",
                     lambda w: ["--authored", "--authored-dir", str(w.root / "authored-private")]),
    "writer": (True, None, WRITER, lambda w: ["--case", WRITER]),
    "benchmark": (True, GUARD, "cpb-g1-01", lambda w: w.benchmark()),
}


@pytest.mark.parametrize("name", sorted(ROUND_TRIP_COMMANDS))
def test_a_batch_run_that_leaves_no_result_file_ends_by_name_with_no_fallback_from_every_command(
        name, tmp_path, monkeypatch):
    runner, guard, case_id, argv = ROUND_TRIP_COMMANDS[name]
    if runner and not HAVE_RUNNER:
        pytest.skip(f"no runner executable ({run_pilot.RUNNER_ENV}); the SWBPIPE solve runs for real")
    world = World(tmp_path, monkeypatch, guard=guard, csv=invented_print() if name == "benchmark" else None)
    world.state["solve"]["mode"] = "print"  # the batch route leaves no result file; the fallback would print one
    world.save()
    if name == "authored_dir":
        directory = world.root / "authored-private"
        put(directory / "cx-01-invented.mbf", b"INVENTED AUTHORED FIXTURE\r\ncx-01-invented\r\n")
        put(directory / "manifest.json", json.dumps({"document_kind": run_pilot.AUTHORED_MANIFEST_KIND,
                                                     "cases": [{"case_id": "cx-01-invented",
                                                                "file": "cx-01-invented.mbf"}]}))
    before = snapshot(world.root)
    code, out = world.main(*argv(world))
    assert code != 0, out
    assert_append_only(before, snapshot(world.root))
    assert "route=batch_no_csv" in out, out
    log = assert_guest_clean(world)
    solves = [entry for entry in log if entry["kind"] == "solve"]
    assert solves and all("-NoGuiFallback" in entry["command"] for entry in solves)
    record = run_record(world.root, case_id, "runs/000001")
    assert record["route"] == "batch_no_csv" and record["guest_log"]["gui"]["used"] is False
    assert current_case(world.root, case_id).run_relative is None  # never published


# -- collection: create-new copies, retries and collisions ----------------------------------------------------------
def test_a_name_appearing_at_a_collection_destination_after_the_check_is_never_written(tmp_path, monkeypatch):
    # After the harness found the new caepipe_out/ empty, a link to a stored file appears at a name the guest copies
    # to. The guest's create-new copy refuses that name; the stored file keeps its bytes; nothing is published.
    # (Were collection a `copy /Y`, as before, the substituted guest would write through the link, as the real one
    # does, and this test would fail.)
    world = World(tmp_path, monkeypatch)
    stored = world.root / "cases" / WRITER / "caepipe_out" / "model.mod"  # other bytes than the guest's model.csv
    out = world.root / "cases" / AUTHORED / "runs" / "000001" / "caepipe_out"
    world.state["actions"] = [{"before": "collect", "op": "link", "source": str(stored), "path": str(out / "model.csv")}]
    world.save()
    before = snapshot(world.root)
    code, msg = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1
    assert_append_only(before, snapshot(world.root), new_in_existing=())
    record = run_record(world.root, AUTHORED, "runs/000001")
    assert record["route"] == "collection_failed" and record["collection"]["files"]["model.csv"] == "present_differs"
    assert current_case(world.root, AUTHORED).run_relative is None


@pytest.mark.parametrize("fault, route", [
    ({"kind": "collect", "transport_after": True}, "csv_from_batch"),   # the retry finds its own complete copies
    ({"kind": "collect", "partial": "model.csv"}, "collection_failed"),  # the retry finds its own partial copy
    ({"kind": "collect", "collect_file": 2}, "collection_failed"),       # one file copied part-way, reported failed
])
def test_a_transport_failure_or_a_partial_guest_copy_never_overwrites(fault, route, tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    world.state["faults"] = [fault]
    world.save()
    before = snapshot(world.root)
    code, msg = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert_append_only(before, snapshot(world.root))
    record = run_record(world.root, AUTHORED, "runs/000001")
    assert record["route"] == route, msg
    assert (current_case(world.root, AUTHORED).run_relative == "runs/000001") is (route == "csv_from_batch")
    collects = [entry for entry in world.log() if entry["kind"] == "collect"]
    assert len(collects) == (2 if "partial" in fault or "transport_after" in fault else 1)


# -- screens and the series step -------------------------------------------------------------------------------------
@needs_runner
def test_screens_and_the_series_step_write_only_new_files(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    before = snapshot(world.root)
    code, out = world.main("--offline", "--case", WRITER, "--case", SELECTED)
    assert code == 0, out
    new = assert_append_only(before, snapshot(world.root))
    assert world.log() == []  # no guest
    first = json.loads((world.root / "cases" / WRITER / "selections" / "000001.json").read_text())
    assert first["run"] == STORED_SOURCE and first["screen"]  # the legacy source stated explicitly
    screen = current_case(world.root, SELECTED)
    assert screen.run_relative == "runs/000001" and screen.screen_relative is not None
    assert json.loads((screen.screen / "screen_record.json").read_text("utf-8"))["read"]["run"] == "runs/000001"
    assert "series/shear-deformation/000002.json" in new


@needs_runner
def test_every_screen_command_completes_a_screen_with_no_guest_command_and_no_guest_configuration(
        tmp_path, monkeypatch):
    # A stored benchmark run first, through the live command; then the configuration file and the guest command go.
    world = World(tmp_path, monkeypatch, guard=GUARD, csv=invented_print())
    assert world.main(*world.benchmark())[0] == 0
    (world.root / "harness_config.json").unlink()
    world.environ["PATH"] = os.pathsep.join(  # no prlctl anywhere on it (a host's own is left out, never run)
        entry for entry in os.environ.get("PATH", "").split(os.pathsep)
        if entry and not os.path.exists(os.path.join(entry, "prlctl")))
    assert shutil.which("prlctl", path=world.environ["PATH"]) is None
    guest_calls = len(world.log())
    before = snapshot(world.root)
    code, out = world.main("--offline", "--case", WRITER, "--case", SELECTED)
    assert code == 0, out
    code, out = world.main("--benchmark", str(world.tmp / "set" / "manifest.json"), "--case", "cpb-g1-01", "--offline")
    assert code == 0, out
    new = assert_append_only(before, snapshot(world.root))
    for case_id in (WRITER, SELECTED, "cpb-g1-01"):
        view = current_case(world.root, case_id)
        assert view.screen_relative is not None, case_id
        assert (view.screen / "screen_record.json").is_file()
        assert any((view.screen / "comparison").iterdir())
    assert any(re.fullmatch(r"series/shear-deformation/[0-9]{6}\.json", path) for path in new)
    assert len(world.log()) == guest_calls  # no guest command


# -- legacy reading and the one resolver -----------------------------------------------------------------------------
def test_a_case_without_a_selection_is_read_where_its_results_are(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    for case_id in (WRITER, AUTHORED):
        view = current_case(world.root, case_id)
        assert view.selection is None and view.source == STORED_SOURCE
        assert view.run == view.export == world.root / "cases" / case_id
    assert run_pilot._stored_comparison(world.root, WRITER)["marker"] == "legacy"


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


def rewrite(path: Path, **changes) -> None:
    data = json.loads(path.read_text())
    path.write_text(json.dumps({**data, **changes}))


def break_selections(case_root: Path, how: str) -> None:
    second = case_root / "selections" / "000002.json"
    if how == "gap":
        selection(case_root, 3, run="runs/000001")
    elif how == "sequence_mismatch":
        selection(case_root, 2, run="runs/000001")
        rewrite(second, sequence=3)
    elif how == "conflicting_previous":
        selection(case_root, 2, run="runs/000001")
        rewrite(second, previous=None)
    elif how == "other_case":
        selection(case_root, 2, run="runs/000001")
        rewrite(second, case_id="cpc-01-other")
    elif how == "no_source":
        selection(case_root, 2, run=None)
    elif how == "corrupt":
        put(second, "{ not json")
    elif how == "unreadable":
        selection(case_root, 2, run="runs/000001")
        second.chmod(0)
    elif how == "absent_run":
        selection(case_root, 2, run="runs/000009")
    elif how == "incomplete_run":
        (case_root / "runs" / "000002" / "caepipe_out").mkdir(parents=True)
        selection(case_root, 2, run="runs/000002")
    elif how == "run_of_another_case":
        solve_folder(case_root / "runs" / "000002", mbf_text(SELECTED), "x\n", marker="other", run="runs/000002")
        rewrite(case_root / "runs" / "000002" / "caepipe_out" / "run_record.json", case_id="cpc-99-other")
        selection(case_root, 2, run="runs/000002")
    elif how == "ineligible_run":
        solve_folder(case_root / "runs" / "000002", mbf_text(SELECTED), "x\n", marker="other", run="runs/000002")
        rewrite(case_root / "runs" / "000002" / "caepipe_out" / "run_record.json", route="timeout")
        selection(case_root, 2, run="runs/000002")
    elif how == "sealed_run":
        solve_folder(case_root / "runs" / "000002", mbf_text(SELECTED), "x\n", marker="sealed", run="runs/000002",
                     sealed=True)
        selection(case_root, 2, run="runs/000002")
    elif how == "verified_traversal":
        selection(case_root, 2, run="runs/000001", verified={"run": {"../../../etc/passwd": "0" * 64}})
    elif how == "stray":
        put(case_root / "selections" / "notes.txt", "stray")


SELECTION_FAULTS = ["gap", "sequence_mismatch", "conflicting_previous", "other_case", "no_source", "corrupt",
                    "unreadable", "absent_run", "incomplete_run", "run_of_another_case", "ineligible_run", "sealed_run",
                    "verified_traversal", "stray"]


@pytest.mark.parametrize("how", SELECTION_FAULTS)
def test_an_untrusted_selection_stops_every_reader_by_name_without_falling_back(how, tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    case_root = world.root / "cases" / SELECTED
    break_selections(case_root, how)
    try:
        with pytest.raises(SelectionError):
            current_case(world.root, SELECTED)
        with pytest.raises(SelectionError):
            run_pilot._stored_comparison(world.root, SELECTED)  # the series step's reader
        before = snapshot(world.root)
        if HAVE_RUNNER:
            code, out = world.main("--offline", "--case", SELECTED)
            assert code == 1 and f"{SELECTED}: route=selection_untrusted" in out
        after = snapshot(world.root)
        assert_append_only(before, after)
        assert {path for path in set(after) - set(before) if "/screens/" in path} == set()
    finally:
        for item in (case_root / "selections").iterdir():
            item.chmod(0o600)


def test_publication_validates_identity_outcome_source_and_verified_paths(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    case_root = world.root / "cases" / SELECTED
    with harness.exchange_tree(world.root) as tree:
        for how, folder in (("other case", "runs/000002"), ("other folder", "runs/000003"),
                            ("timeout", "runs/000004"), ("sealed", "runs/000005")):
            solve_folder(case_root / folder, mbf_text(SELECTED), "x\n", marker=how, run=folder, sealed=how == "sealed")
            record = case_root / folder / "caepipe_out" / "run_record.json"
            if how == "other case":
                rewrite(record, case_id="cpc-99-other")
            elif how == "other folder":
                rewrite(record, run_folder="runs/999999")
            elif how == "timeout":
                rewrite(record, route="timeout")
            with pytest.raises(ContainmentError):
                harness.publish(tree, SELECTED, run=folder, reason="invented")
        with pytest.raises(ContainmentError):
            harness.publish(tree, SELECTED, run="runs/000001", also={"../x": "0" * 64}, reason="invented")
        with pytest.raises(ContainmentError):
            harness.publish(tree, SELECTED, run="../runs/000001", reason="invented")
    assert selections_of(world.root, SELECTED) == ["000001.json"]  # nothing was published


# -- links, races and changes by others -----------------------------------------------------------------------------
def hard_link(source: Path, target: Path) -> None:
    target.parent.mkdir(parents=True, exist_ok=True)
    os.link(source, target)


def test_hard_links_from_stored_files_at_every_fixed_path_are_never_written_through(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    stored = world.root / "cases" / AUTHORED / "caepipe_out" / "model_results.csv"
    script = harness.GUEST_SCRIPT_NAME.format(sha256=sha(harness.GUEST_SCRIPT.read_bytes()))
    # The next run folder of the case, made in advance with a link at every name a step writes there, and the next
    # holder record: each a link to one stored file.
    run = world.root / "cases" / AUTHORED / "runs" / "000001"
    for relative in ("exchange_marker.txt", "run_lock_holder.json", "run_lock_end.json", "settings_before_state.json",
                     "caepipe_in/model.mbf", "caepipe_out/run_record.json", "caepipe_out/model.csv",
                     "caepipe_out/guest_stdout.txt"):
        hard_link(stored, run / relative)
    hard_link(stored, world.root / "_harness" / "holders" / "000001.json")
    before = snapshot(world.root)
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    # The link at holder 000001 is read as the newest holder record: incomplete (not a holder record), so refused.
    assert code == 1 and "route=run_lock_held" in out and "incomplete" in out, out
    assert_append_only(before, snapshot(world.root))
    # The lock file itself linked to a stored file: only ever opened read-only to lock it; never written.
    world_ = World(tmp_path / "lock", monkeypatch)
    hard_link(world_.root / "cases" / AUTHORED / "caepipe_out" / "model_results.csv",
              world_.root / "_harness" / "round_trip.lock")
    before = snapshot(world_.root)
    world_.main("--authored", "--no-export", "--case", AUTHORED)
    after = snapshot(world_.root)
    assert {path: after[path] for path in before} == before
    # The content-named guest script linked to a stored file: refused by name.
    world_ = World(tmp_path / "script", monkeypatch)
    hard_link(world_.root / "cases" / AUTHORED / "caepipe_out" / "model_results.csv", world_.root / "_harness" / script)
    before = snapshot(world_.root)
    code, out = world_.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1 and "route=exchange_check_failed" in out, out
    assert_append_only(before, snapshot(world_.root))
    assert world_.log() == []
    # A selection that is a link to a stored file is untrusted.
    world_ = World(tmp_path / "selection", monkeypatch)
    hard_link(world_.root / "cases" / AUTHORED / "caepipe_out" / "model_results.csv",
              world_.root / "cases" / AUTHORED / "selections" / "000001.json")
    before = snapshot(world_.root)
    with pytest.raises(SelectionError):
        current_case(world_.root, AUTHORED)
    code, out = world_.main("--authored", "--no-export", "--case", AUTHORED)
    # The round trip runs into its own new folder; publishing it stops on the linked selection, named.
    assert code == 1 and f"cases/{AUTHORED}/selections/000001.json is unreadable" in out, out
    assert not (world_.root / "cases" / AUTHORED / "selections" / "000002.json").exists()
    assert_append_only(before, snapshot(world_.root))


def test_a_link_made_in_the_steps_own_folder_during_the_step_is_never_written_through(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    stored = world.root / "cases" / AUTHORED / "caepipe_out" / "model_results.csv"
    out = world.root / "cases" / AUTHORED / "runs" / "000001" / "caepipe_out"
    world.state["actions"] = [{"before": "collect", "op": "link", "source": str(stored),
                               "path": str(out / "guest_stdout.txt")}]
    world.save()
    before = snapshot(world.root)
    code, msg = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1 and "route=containment_violation" in msg, msg
    assert_append_only(before, snapshot(world.root))
    assert current_case(world.root, AUTHORED).run_relative is None


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
            solve_folder(world.root / "cases" / AUTHORED / "runs" / "000009",
                         run_pilot.authored_mbf_bytes(next(item for item in run_pilot.load_authored_cases()
                                                           if item["case_id"] == AUTHORED)),
                         PARSER_FIXTURE.read_text(), marker="another command's run", run="runs/000009")
            selection(world.root / "cases" / AUTHORED, int(target[-1][:6]), run="runs/000009")
        return real_link(self, source, target)

    monkeypatch.setattr(exclusive.Tree, "reserve_dir", reserve_dir)
    monkeypatch.setattr(exclusive.Tree, "link_new", link_new)
    before = snapshot(world.root)
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 0, out
    assert_append_only(before, snapshot(world.root))
    assert list((world.root / "cases" / AUTHORED / "runs" / "000001").iterdir()) == []  # the other command's
    second = json.loads((world.root / "cases" / AUTHORED / "selections" / "000002.json").read_text())
    assert second["run"] == "runs/000002" and second["previous"] == "selections/000001.json"


def test_the_marker_stays_and_nothing_is_removed(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    marker = world.root / "cases" / AUTHORED / "runs" / "000001" / "exchange_marker.txt"
    # Another writer replaces the marker before the probe: the step is refused and the marker is left as it is.
    world.state["actions"] = [{"before": "net_use", "op": "write", "path": str(marker), "data": "another marker"}]
    world.save()
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1 and "route=exchange_check_failed" in out and "Z: answered differs" in out
    assert marker.read_text() == "another marker"
    assert run_record(world.root, AUTHORED, "runs/000001")["exchange_marker"] == "kept in the run folder"


def test_the_drive_check_reads_only_the_marker_on_each_letter_then_inputs_under_the_selected_root(tmp_path,
                                                                                               monkeypatch):
    world = World(tmp_path, monkeypatch)
    world.state["view"]["Y:\\invented"] = str(world.tmp / "elsewhere")
    world.state["listed"] = ["X:"]
    world.save()
    (world.tmp / "elsewhere").mkdir()
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 0, out
    checks = [fake._encoded(entry["command"])[1] for entry in world.log() if entry["kind"] == "exchange_check"]
    marker = "\\invented\\cases\\cpa-05-guide\\runs\\000001\\exchange_marker.txt"
    assert sorted(item["path"] for item in checks[0]) == [f"{letter}:{marker}" for letter in "XYZ"]
    assert sorted(item["path"] for item in checks[1]) == sorted([
        "Z:\\invented\\cases\\cpa-05-guide\\runs\\000001\\caepipe_in\\model.mbf",
        f"Z:\\invented\\_harness\\{harness.GUEST_SCRIPT_NAME.format(sha256=sha(harness.GUEST_SCRIPT.read_bytes()))}"])
    assert len(checks) == 2
    record = run_record(world.root, AUTHORED, "runs/000001")
    assert record["exchange_check"]["listing"]["letters_tested"] == ["X:", "Y:", "Z:"]
    assert record["exchange_check"]["letters_matched"] == ["Z:"]


@pytest.mark.parametrize("answers", [{"answer_twice": True}, {"answers": {"drive_y": ["unreadable drive_y"]}},
                                     {"answers": {"drive_y": ["sha256 drive_y " + "0" * 64]}}])
def test_a_conflicting_unanswered_or_differing_candidate_refuses_the_step(answers, tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    world.state["listed"] = ["Y:"]
    world.state.update(answers)
    world.save()
    before = snapshot(world.root)
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1 and "route=exchange_check_failed" in out, out
    assert_append_only(before, snapshot(world.root))
    assert not {entry["kind"] for entry in world.log()} & WRITING_KINDS


# -- the run lock and the three named stops -------------------------------------------------------------------------
HOLDER = """
import sys, time
sys.path.insert(0, %r)
from core.handoff.caepipe_external import harness
held = harness.hold_run_lock(%r, "cx-holder")
print("held" if not isinstance(held, dict) else held["route"], flush=True)
time.sleep(600)
"""


def test_a_held_lock_refuses_with_the_holders_record_and_a_killed_holder_releases_it(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    holder = subprocess.Popen([sys.executable, "-c", HOLDER % (str(ROOT), str(world.root))], stdout=subprocess.PIPE,
                              text=True)
    try:
        assert holder.stdout.readline().strip() == "held"
        before = snapshot(world.root)
        code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
        assert code == 1 and "route=run_lock_held" in out, out
        # What a reader needs: whose lock it is.
        assert "cases/cx-holder/runs/000001" in out and "held step" in out and f"process {holder.pid}" in out
        assert "on host" in out and "since" in out
        assert world.log() == []  # no guest command
        record = run_record(world.root, AUTHORED, "runs/000001")
        assert record["run_lock"]["state"] == "unfinished" and record["run_lock"]["holder"]["process_id"] == holder.pid
        assert len(record["run_lock"]["attempts"]) == 2  # one bounded wait, then the named refusal
    finally:
        holder.send_signal(signal.SIGKILL)
        holder.wait(timeout=30)
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 0, out  # the kernel released the lock; nothing had to be removed
    assert_append_only(before, snapshot(world.root))
    record = run_record(world.root, AUTHORED, "runs/000002")
    assert record["run_lock"]["predecessor"]["state"] == "unfinished"  # the killed holder ended unfinished


@pytest.mark.parametrize("platform", ["not_honoured", "no_flock"])
def test_a_file_system_that_does_not_honour_the_lock_refuses_the_step(platform, tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    if platform == "no_flock":
        monkeypatch.setattr(harness, "fcntl", None)
    else:  # every flock attempt succeeds, a second descriptor's included
        monkeypatch.setattr(harness, "fcntl", SimpleNamespace(LOCK_EX=2, LOCK_NB=4, flock=lambda fd, operation: None))
    before = snapshot(world.root)
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1 and "route=run_lock_unavailable" in out, out
    assert world.log() == []
    assert_append_only(before, snapshot(world.root))


@pytest.mark.parametrize("processes, route", [
    ([{"kind": "script", "pid": 7001}], "guest_script_running"),
    ([{"kind": "tool", "pid": 7002}], "program_instance_running"),
    (None, "guest_processes_unknown"),
])
def test_a_live_step_is_refused_while_a_guest_script_or_a_foreign_instance_runs(processes, route, tmp_path,
                                                                                monkeypatch):
    world = World(tmp_path, monkeypatch)
    if processes is None:
        world.state["processes_unreadable"] = True
    else:
        world.state["processes"] = processes
    world.save()
    before = snapshot(world.root)
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1 and f"route={route}" in out, out
    assert_append_only(before, snapshot(world.root))
    assert not {entry["kind"] for entry in world.log()} & (WRITING_KINDS | {"work_root", "work_directory"})


def test_a_window_shown_leaves_the_process_running_and_stops_the_next_step(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    world.state["solve"].update(mode="window", window_title="Invented Activation Prompt")
    world.save()
    before = snapshot(world.root)
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1
    record = run_record(world.root, AUTHORED, "runs/000001")
    assert record["route"] == "window_shown"
    assert record["guest_log"]["window_shown"]["windows"] == [{"title": "Invented Activation Prompt", "class": "#32770"}]
    assert record["guest_log"]["batch"]["stopped_own_process"] is False
    assert current_case(world.root, AUTHORED).run_relative is None  # never published
    # The harness sends the guest nothing that stops or answers a process.
    assert not any(re.search(r"taskkill|Stop-Process|\.Kill\(", entry.get("command", "")) for entry in world.log())
    # The process it left running stops the next step under its own name.
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1 and "route=program_instance_running" in out, out
    assert_append_only(before, snapshot(world.root))
    # A timeout with no window: stopped as before; the next step runs.
    world_ = World(tmp_path / "plain", monkeypatch)
    world_.state["solve"]["mode"] = "timeout_no_window"
    world_.save()
    code, out = world_.main("--authored", "--no-export", "--case", AUTHORED)
    assert run_record(world_.root, AUTHORED, "runs/000001")["route"] == "no_results_after_batch"
    world_.state["solve"]["mode"] = "batch"
    world_.save()
    assert world_.main("--authored", "--no-export", "--case", AUTHORED)[0] == 0


def test_the_guest_script_stops_a_process_only_through_stop_own_after_looking_for_its_windows():
    # What the substituted guest cannot show, read from the script itself: every stop goes through Stop-Own, which
    # looks for the process's visible windows first and stops it only through the object it started.
    script = harness.GUEST_SCRIPT.read_text(encoding="utf-8")
    assert "Stop-Process" not in script and "Remove-Item" not in script and "taskkill" not in script
    assert script.count(".Kill()") == 1
    body = script[script.index("function Stop-Own("):script.index("$script:windowShown = $null")]
    assert body.index("Visible-Windows") < body.index("return $false") < body.index(".Kill()")
    assert script.count("Stop-Own $") == 3  # the batch timeout, the GUI print's finally, the export's finally
    assert "Record-Owned" in script and '"{0} {1}" -f [int]$p.Id, $start' in script


def test_a_reused_process_id_with_another_start_time_is_not_ours_and_refuses_the_step(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch, guard=GUARD)
    world.state["solve"]["mode"] = "recycled"
    world.save()
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    record = run_record(world.root, AUTHORED, "runs/000001")
    owned = record["settings_guard"]["owned_processes"]
    assert owned["status"] == "foreign" and owned["foreign_pids"] == [fake.TOOL_PID] and "running" not in owned
    assert current_case(world.root, AUTHORED).run_relative is None
    # The session's own check sees it as an instance this run did not start.
    config = harness.resolve_harness_config(world.environ)
    found = harness.check_no_foreign_instance(config)
    assert found["status"] == "foreign_instance_running"
    assert found["foreign"] == [{"pid": fake.TOOL_PID, "start": fake.RECYCLED_START}]
    # The next step is refused: its predecessor's settings comparison was not clean (a foreign instance appeared).
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1 and "route=settings_not_restored" in out, out


SETTINGS_VALUES = [data for _, data in fake.REGISTRY_VALUES.values()]


def no_value_or_hash(texts: list[str]) -> None:
    secrets_ = SETTINGS_VALUES + [sha(value) for value in SETTINGS_VALUES]
    for text in texts:
        for secret in secrets_:
            assert secret not in text, f"{secret[:6]}... appears in an output"


def outputs_outside(root: Path, run_folder: Path) -> list[str]:
    """Every file of the exchange folder outside the run folders that recorded a before-state (`run_folder`
    included): the settings record and anything derived from it stay in its own run folder."""
    recording = {path.parent for path in root.rglob("settings_before_state.json")} | {run_folder}
    return [path.read_text(errors="replace") for path in root.rglob("*")
            if path.is_file() and not recording & set(path.parents)]


def test_a_step_killed_after_writing_the_before_state_stops_the_next_until_a_clearance_exists(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch, guard=RESTORING_GUARD)
    world.state["actions"] = [{"before": "solve", "op": "kill_parent"}]
    world.save()
    killed = world.main_in_process_of_its_own("--authored", "--no-export", "--case", AUTHORED)
    assert killed.returncode == -signal.SIGKILL, killed.stderr
    dead = world.root / "cases" / AUTHORED / "runs" / "000001"
    before_state = json.loads((dead / "settings_before_state.json").read_text())
    # The before-state: every member by name, kind and hash; values only inside the restore scope.
    by_name = {member["name"]: member for member in before_state["members"]}
    assert set(by_name) == {"Licence", "Use something", "1", "N"}
    assert "data" not in by_name["Licence"] and "data" not in by_name["Use something"]
    assert by_name["1"]["data"] == "0A0B0C0D0E0F1A1B" and by_name["Licence"]["sha256"] == sha("INVENTEDLICENCE7F3A9C2E")
    assert not (dead / "settings_check.json").exists()
    world.state["actions"] = []
    world.save()
    before = snapshot(world.root)
    commands_before = len(world.log())
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 1 and "route=settings_not_restored" in out, out
    assert f"cases/{AUTHORED}/runs/000001" in out and before_state["written"] in out
    for member in ("Licence", "Use something", "Recent list\\1"):
        assert member in out
    assert world.log()[commands_before:] == []  # refused before any guest command
    no_value_or_hash([out, killed.stdout, killed.stderr, *outputs_outside(world.root, dead)])
    # The clearance: names the run, who cleared it and why; refuses an empty why, another run and a second one.
    clear = ["--case", AUTHORED, "--run", "runs/000001", "--by", "invented operator"]
    log_before = len(world.log())
    assert clear_settings_stop.main(clear + ["--reason", "  "], environ=world.environ) != 0
    assert clear_settings_stop.main(["--case", AUTHORED, "--run", "runs/000002", "--by", "x", "--reason", "y"],
                                    environ=world.environ) == 1
    snapshot_before = snapshot(world.root)
    assert clear_settings_stop.main(clear + ["--reason", "settings restored by hand and compared; no instance runs"],
                                    environ=world.environ) == 0
    added = sorted(set(snapshot(world.root)) - set(snapshot_before))
    assert added == [f"cases/{AUTHORED}/runs/000001/settings_clearance.json"]
    assert_append_only(snapshot_before, snapshot(world.root), new_in_existing=added)
    assert clear_settings_stop.main(clear + ["--reason", "again"], environ=world.environ) == 1
    assert len(world.log()) == log_before  # the command issues no guest command
    # The next step proceeds and its record names the clearance it proceeded on.
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 0, out
    record = run_record(world.root, AUTHORED, newest_run(world.root, AUTHORED))
    assert record["run_lock"]["predecessor"]["proceeded_on_clearance"] == \
        f"cases/{AUTHORED}/runs/000001/settings_clearance.json"
    assert_append_only(before, snapshot(world.root), new_in_existing=added)


def test_a_completed_guarded_step_does_not_stop_the_next(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch, guard=RESTORING_GUARD)
    assert world.main("--authored", "--no-export", "--case", AUTHORED)[0] == 0
    check = json.loads((world.root / "cases" / AUTHORED / "runs" / "000001" / "settings_check.json").read_text())
    assert check["restored_or_unchanged"] is True
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    assert code == 0, out
    no_value_or_hash([out, *outputs_outside(world.root, world.root / "cases" / AUTHORED / "runs" / "000001")])


# -- records on early and partial failure ----------------------------------------------------------------------------
@pytest.mark.parametrize("failing", ["run_lock_holder.json", "exchange_marker.txt", "run_record.json"])
def test_every_step_that_reserved_a_run_folder_leaves_a_complete_record(failing, tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    real = exclusive.Tree.create

    def create(tree, parts, data):
        if parts[-1] == failing:
            payload = data.encode("utf-8") if isinstance(data, str) else data
            real(tree, parts, payload[:len(payload) // 2])  # partly written, then the write fails
            raise OSError(errno.EIO, f"injected failure writing {failing}")
        return real(tree, parts, data)

    monkeypatch.setattr(exclusive.Tree, "create", create)
    before = snapshot(world.root)
    code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
    monkeypatch.setattr(exclusive.Tree, "create", real)
    assert isinstance(code, OSError) or code == 1
    assert_append_only(before, snapshot(world.root))
    out_dir = world.root / "cases" / AUTHORED / "runs" / "000001" / "caepipe_out"
    name = "run_record_2.json" if failing == "run_record.json" else "run_record.json"
    record = json.loads((out_dir / name).read_text())
    assert record["route"] == "harness_error" and set(RUN_RECORD_MEMBERS) <= set(record)
    assert current_case(world.root, AUTHORED).run_relative is None


# -- crashes at every write boundary, each from the same starting world ---------------------------------------------
class Crash:
    """Fail the n-th exclusive-layer operation (a file creation, a folder reservation, a container directory or a
    link): `partial` writes the first half of a file first, as a crash in the middle of the write would leave it."""

    def __init__(self, monkeypatch, n: int, partial: bool):
        self.n, self.partial, self.count, self.at = n, partial, 0, None
        real = {name: getattr(exclusive.Tree, name) for name in ("create", "reserve_dir", "link_new", "ensure_dir")}

        def boundary(parts) -> bool:
            self.count += 1
            if self.count == self.n:
                self.at = "/".join(parts)
            return self.count == self.n

        def create(tree, parts, data):
            if boundary(parts):
                payload = data.encode("utf-8") if isinstance(data, str) else data
                if self.partial:
                    real["create"](tree, parts, payload[:len(payload) // 2])
                raise OSError(errno.EIO, f"injected crash at write boundary {self.n}")
            return real["create"](tree, parts, data)

        def simple(name):
            def wrapped(tree, *args):
                if boundary(args[-1] if name == "link_new" else args[0]):
                    raise OSError(errno.EIO, f"injected crash at write boundary {self.n}")
                return real[name](tree, *args)
            return wrapped

        monkeypatch.setattr(exclusive.Tree, "create", create)
        for name in ("reserve_dir", "link_new", "ensure_dir"):
            monkeypatch.setattr(exclusive.Tree, name, simple(name))


CRASH_PATHS = {
    "authored": (False, None, AUTHORED, lambda w: ["--authored", "--no-export", "--case", AUTHORED]),
    "authored_dir": (False, GUARD, "cx-01-invented",
                     lambda w: ["--authored", "--authored-dir", str(w.root / "authored-private")]),
    "export": (False, None, WRITER, lambda w: ["--export-only", "--case", WRITER]),  # refused: record only
    "writer": (True, None, WRITER, lambda w: ["--case", WRITER]),
    "screen": (True, None, SELECTED, lambda w: ["--offline", "--case", SELECTED]),
    "benchmark": (True, GUARD, "cpb-g1-01", lambda w: w.benchmark()),
    "benchmark_screen": (True, None, "cpb-g1-01", lambda w: [
        "--benchmark", str(w.tmp / "set" / "manifest.json"), "--case", "cpb-g1-01", "--offline"]),
}


def crash_world(name: str, tmp_path: Path, monkeypatch) -> tuple[World, str, list[str]]:
    runner, guard, case_id, argv = CRASH_PATHS[name]
    world = World(tmp_path, monkeypatch, guard=guard if name != "benchmark_screen" else GUARD,
                  csv=invented_print() if name.startswith("benchmark") else None)
    if name == "authored_dir":
        directory = world.root / "authored-private"
        put(directory / "cx-01-invented.mbf", b"INVENTED AUTHORED FIXTURE\r\ncx-01-invented\r\n")
        put(directory / "manifest.json", json.dumps({"document_kind": run_pilot.AUTHORED_MANIFEST_KIND,
                                                     "cases": [{"case_id": "cx-01-invented",
                                                                "file": "cx-01-invented.mbf"}]}))
    if name == "benchmark_screen":  # a stored benchmark run to screen
        assert world.main(*world.benchmark())[0] == 0
    return world, case_id, argv(world)


@pytest.mark.parametrize("partial", [False, True])
@pytest.mark.parametrize("name", sorted(CRASH_PATHS))
def test_a_crash_at_any_write_boundary_changes_nothing_and_a_later_command_proceeds(name, partial, tmp_path,
                                                                                   monkeypatch):
    if CRASH_PATHS[name][0] and not HAVE_RUNNER:
        pytest.skip(f"no runner executable ({run_pilot.RUNNER_ENV}); the SWBPIPE solve runs for real")
    n = 1
    while True:
        # Each injected fault starts from the same world: a fresh one, so every boundary of one command is reached.
        world, case_id, argv = crash_world(name, tmp_path / f"n{n}", monkeypatch)
        before = snapshot(world.root)
        crash = Crash(monkeypatch, n, partial)
        code, out = world.main(*argv)
        monkeypatch.undo()
        monkeypatch.setenv(STATE_ENV, str(world.state_path))
        monkeypatch.setattr(run_pilot, "time", SimpleNamespace(sleep=world.sleeps.append))
        assert_append_only(before, snapshot(world.root))
        if crash.count < n:  # this command has fewer boundaries than n: every boundary has failed once
            assert code == (1 if name == "export" else 0), out
            assert n > (3 if name == "export" else 8), n
            return
        # Absorbed by design: the holder's end record (the lock's release does not depend on it); and a live benchmark
        # retries a model once after its round trip raised.
        # The check record's failure is absorbed too: the step ends, and the next step is then refused by name.
        assert code != 0 or crash.at.endswith(("/run_lock_end.json", "/settings_check.json")) or (
            name == "benchmark" and 120.0 in world.sleeps), (n, crash.at, out)
        for selection_file in (world.root / "cases" / case_id / "selections").glob("*.json"):
            json.loads(selection_file.read_text())  # a selection that appears is complete
        script = world.root / "_harness" / harness.GUEST_SCRIPT_NAME.format(sha256=sha(harness.GUEST_SCRIPT.read_bytes()))
        assert not script.exists() or script.read_bytes() == harness.GUEST_SCRIPT.read_bytes(), n
        # A later command proceeds, with a new run or screen folder (the lock is free; no partial file blocks it),
        # except where the crash came between a guarded step's before-state and its comparison record: then the
        # later step is refused by name until a clearance exists, as it must be.
        code, out = world.main(*argv)
        between = crash.at.endswith(("/settings_before_state.json", "/settings_check.json")) or (
            "route=settings_not_restored" in out and any(
                (path / "settings_before_state.json").exists() and not (path / "settings_check.json").exists()
                for path in (world.root / "cases" / case_id / "runs").iterdir()))
        assert code == (1 if name == "export" else 0) or (between and "route=settings_not_restored" in out), \
            (n, crash.at, out)
        n += 1


def test_an_abrupt_end_at_each_guest_command_changes_nothing_and_leaves_no_lock(tmp_path, monkeypatch):
    # A real SIGKILL of the harness process just before each guest command of an authored round trip.
    kinds = ["net_use", "exchange_check", "processes", "work_root", "work_directory", "prepare", "solve", "collect"]
    for kind in kinds:
        world = World(tmp_path / kind, monkeypatch)
        world.state["actions"] = [{"before": kind, "op": "kill_parent"}]
        world.save()
        before = snapshot(world.root)
        killed = world.main_in_process_of_its_own("--authored", "--no-export", "--case", AUTHORED)
        assert killed.returncode == -signal.SIGKILL, (kind, killed.stderr)
        assert_append_only(before, snapshot(world.root))
        world.state["actions"] = []
        world.save()
        code, out = world.main("--authored", "--no-export", "--case", AUTHORED)
        assert code == 0, (kind, out)


# -- what is read: the sealed flag first ----------------------------------------------------------------------------
OPENS: list[tuple[str, tuple[str, ...]]] = []
RECORDING = [False]


def _audit(event: str, args: tuple) -> None:
    if RECORDING[0] and event == "open" and isinstance(args[0], (str, bytes, os.PathLike)):
        frames = []
        frame = sys._getframe(1)
        while frame is not None and len(frames) < 12:
            frames.append(frame.f_code.co_name)
            frame = frame.f_back
        OPENS.append((os.fsdecode(args[0]), tuple(frames)))


sys.addaudithook(_audit)
HASHING = {"_file_sha256", "file_sha256", "sha256_bytes"}


@contextlib.contextmanager
def recording_opens():
    OPENS.clear()
    RECORDING[0] = True
    try:
        yield OPENS
    finally:
        RECORDING[0] = False


def content_reads(opens, under: Path) -> list[str]:
    """Opens of a result file under `under` other than to hash it (a run record, where the sealed flag is read
    first, is not a result file)."""
    return [path for path, frames in opens if Path(path).is_relative_to(under) and not set(frames) & HASHING
            and not Path(path).name.startswith("run_record")]


def test_a_sealed_text_capture_is_hashed_never_read_and_never_published(tmp_path, monkeypatch):
    world = World(tmp_path, monkeypatch)
    world.state["solve"]["mode"] = "print_txt"
    world.save()
    calls = []
    real = harness._print_layout_problem
    monkeypatch.setattr(harness, "_print_layout_problem", lambda path: calls.append(path) or real(path))
    config = harness.resolve_harness_config(world.environ)
    with recording_opens() as opens:
        record = harness.run_caepipe_round_trip(config, AUTHORED, run_pilot.authored_mbf_bytes(next(
            item for item in run_pilot.load_authored_cases() if item["case_id"] == AUTHORED)).decode("ascii"),
            capture_only=True)
    captured = world.root / "cases" / AUTHORED / "runs" / "000001" / "caepipe_out"
    assert calls == []  # the recognition reader was never called (observed here, outside any helper)
    assert content_reads(opens, captured / "model.txt") == []
    assert record["route"] == "captured_unread" and record["sealed"] is True
    assert "model.txt" in record["outputs_sha256"]  # hashed
    with harness.exchange_tree(world.root) as tree, pytest.raises(ContainmentError):
        harness.publish(tree, AUTHORED, run="runs/000001", reason="invented")


def test_no_reader_opens_sealed_content(tmp_path, monkeypatch):
    from tests import test_caepipe_private_correlation as regression
    from tests import test_caepipe_private_mbf_import as importing
    world = World(tmp_path, monkeypatch)
    # A legacy case whose stored run record marks its outputs sealed: every reader stops before its content.
    sealed_case = world.root / "cases" / WRITER
    rewrite(sealed_case / "caepipe_out" / "run_record.json", sealed=True)
    put(sealed_case / "caepipe_out" / "exported.mbf", b"INVENTED SEALED EXPORT\r\n")
    selected = world.root / "cases" / SELECTED  # an unselected, sealed run beside the selected one
    solve_folder(selected / "runs" / "000002", mbf_text(SELECTED), "SEALED\n", marker="sealed", run="runs/000002",
                 sealed=True)
    parse_calls, classify_calls = [], []
    monkeypatch.setattr(run_pilot, "read_caepipe_print_csv",
                        lambda path, _real=run_pilot.read_caepipe_print_csv: parse_calls.append(path) or _real(path))
    monkeypatch.setattr(importing, "classify", lambda *a, **k: classify_calls.append(a) or pytest.fail("classified"))
    with recording_opens() as opens:
        with pytest.raises(SelectionError):
            current_case(world.root, WRITER)
        # The series step's reader.
        with pytest.raises(SelectionError):
            run_pilot._stored_comparison(world.root, WRITER)
        # The private import check reads the sealed flag before it opens the export.
        statuses = importing.check_exchange(world.root, record=False, emit=lambda line: None,
                                            cases=[tracked(WRITER)], authored=[])
        # The private regression copies only what the selection names, never a sealed or unselected run.
        copied = regression.copy_selected(world.root, SELECTED, tmp_path / "copy")
    assert statuses[0].status == "refused"
    assert parse_calls == [] and classify_calls == []
    assert content_reads(opens, sealed_case / "caepipe_out") == []
    assert content_reads(opens, selected / "runs" / "000002") == []
    assert copied == ["selections", "runs/000001"]
    assert not (tmp_path / "copy" / "cases" / SELECTED / "runs" / "000002").exists()
