"""The length of every guest command the CAEPIPE harness sends, for each of the 56 batch runs (DEL-17-05).

Every guest command the harness builds is at most `GUEST_COMMAND_LIMIT` characters, measured as the prlctl call's argv
joined with single spaces (`harness._guest_command_length`). This test runs, for each of the 56 runs of the batch (the
benchmark manifest's families P1 to P7 in the pressure-default and pressure-elongation-on series, and S1, S2 and S3),
complete round trips against the fake guest that take the longest-path branches: a first dispatch that never arrives,
so the host fences it and dispatches again; a restore that writes back the recent-files values exactly, the literal
value through the registry API, one write lost in transport and settled by its read-back before it is issued again;
and a restore whose write fails, which reads the key once more. The chunk read at the largest offset a collection can
reach is built by the same builder. Every command is recorded with its length, and every one, the host's fence and
the registry API's writes included, is asserted within the limit.

The configuration is invented, with paths longer than the design's examples: the lengths are a forecast for the
configured paths, which the harness checks itself before every send. Nothing here is a capture or a measurement.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import harness  # noqa: E402
from core.handoff.caepipe_external.harness import HarnessConfig, SettingsGuard, run_caepipe_round_trip  # noqa: E402
from tests import test_caepipe_correlation as correlation  # noqa: E402
from tests.caepipe_fake_guest import classify, write_call  # noqa: E402

MANIFEST = ROOT / "validation" / "caepipe_benchmark" / "manifest.json"
PRESSURE_FAMILIES = ("P1", "P2", "P3", "P4", "P5", "P6", "P7")
PRESSURE_SERIES = ("pressure-default", "pressure-elongation-on")
SHEAR_FAMILIES = ("S1", "S2", "S3")

# Invented, and longer than the design's examples. (The fake guest's VM name is used: 11 characters.)
EXCHANGE = "Z:\\invented-swbpipe-caepipe-exchange"
WORK = "C:\\Invented Work Root\\swbpipe-runs"
EXE = "C:\\Program Files (x86)\\Invented Vendor\\Invented Tool 12.00\\tool.exe"
KEY = "HKCU\\Software\\Invented Vendor\\Invented Tool\\12.00"
PRINTED_KEY = "HKEY_CURRENT_USER\\Software\\Invented Vendor\\Invented Tool\\12.00"
GUARD = SettingsGuard(key=KEY, recent_files_subkey="Recent list", restore_scope="key")
PRINT_NAME = "model_results.csv"  # a print name longer than the batch's own


def batch_runs() -> list[str]:
    """The 56 run ids of the batch, from the benchmark manifest by family."""
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    runs = []
    for entry in manifest["models"]:
        if entry["family"] in PRESSURE_FAMILIES:
            runs += [run["run_id"] for run in entry.get("runs") or [] if run.get("series") in PRESSURE_SERIES]
        elif entry["family"] in SHEAR_FAMILIES:
            runs.append(entry["model_id"])
    return runs


def _utf16_hex(text: str) -> str:
    return (text + "\0").encode("utf-16-le").hex().upper()


def _step_name(command: str) -> str:
    """The step a guest command belongs to, by its form."""
    kind = classify(command)
    if kind == "reg_query":
        return "settings_reading" if command.endswith(" /s") and '"HKCU' in command else (
            "subtree_readback" if command.endswith(" /s") else "value_readback")
    if kind == "reg_write":
        if command.startswith("reg add") and " /v " in command:
            return "value_write"
        if command.startswith("reg add"):
            return "key_create"
        return "value_delete" if " /v " in command else "key_delete"
    if kind == "literal":
        return {"regset": "regset", "regdelete": "regdelete"}[write_call(command)["op"]]
    if kind == "regread":
        return "literal_readback"
    return {"chunk": "chunk_read", "cim": "launcher_query", "job": "job_query"}.get(kind, kind)


def _config(root: Path) -> HarnessConfig:
    root.mkdir(parents=True, exist_ok=True)
    (root / ".gitignore").write_text("*\n", encoding="utf-8")
    return HarnessConfig(exchange_root=root, vm_name="invented-vm", guest_executable=EXE, guest_exchange_root=EXCHANGE,
                         guest_work_root=WORK, prlctl="/invented/prlctl", acknowledged=True,
                         trial_first_launch_date=None, settings_guard=GUARD)


def _guest(run_id: str, literal: str) -> correlation.FakeGuest:
    guest = correlation.FakeGuest()
    recent = PRINTED_KEY + "\\Recent list"
    earlier = f"{WORK}\\{run_id}\\{'e' * 32}\\model.mbf"  # a long path the list held before the run
    guest.values = {(PRINTED_KEY + "\\Options", "Use something"): ("REG_BINARY", "01"),
                    (recent, "N"): ("REG_BINARY", "02000000"),
                    (recent, "1"): ("REG_BINARY", _utf16_hex(earlier)),
                    (recent, "2"): ("REG_BINARY", _utf16_hex(earlier.replace("e", "d"))),
                    (recent, literal): ("REG_BINARY", _utf16_hex(earlier.replace("e", "c")))}
    guest.empty_keys = set()
    guest.outputs = {PRINT_NAME: b"Invented,print\r\n", "model.res": b"RES", "model.mod": b"INVENTED\r\n"}
    return guest


def _solve(run_id: str, literal: str):
    def solve(guest, argv, config):
        recent = PRINTED_KEY + "\\Recent list"
        this = f"{WORK}\\{run_id}\\{'f' * 32}\\model.mbf"
        guest.values[(recent, "3")] = guest.values[(recent, "2")]  # the list shifts; one value appears
        guest.values[(recent, "2")] = guest.values[(recent, "1")]
        guest.values[(recent, "1")] = ("REG_BINARY", _utf16_hex(this))
        guest.values[(recent, literal)] = ("REG_BINARY", _utf16_hex(this.replace("f", "a")))
        guest.values[(recent, "N")] = ("REG_BINARY", "03000000")
        return correlation.subprocess.CompletedProcess(argv, 0, json.dumps({
            "status": "csv_from_batch", "mbf_sha256": correlation.MBF_SHA, "owned_pids": list(guest.owned),
            "csv_path": f"{WORK}\\{run_id}\\{PRINT_NAME}"}), "")
    return solve


def command_lengths(tmp_path: Path, monkeypatch) -> list[dict]:
    """Every guest command of the longest-path runs, for each batch run: {run, variant, step, length, command}."""
    monkeypatch.setattr(correlation, "INVENTED_KEY", PRINTED_KEY)
    monkeypatch.setattr(harness, "_backoff", lambda seconds: None)
    literal = EXE.rsplit("\\", 1)[0] + "\\"
    rows: list[dict] = []
    for run_id in batch_runs():
        for variant in ("exact_restore", "failed_restore"):
            config = _config(tmp_path / variant / run_id)
            guest = _guest(run_id, literal)
            guest.plan = {1: {"reply": "undelivered"}}  # the first dispatch never arrives: fenced, then a second
            if variant == "exact_restore":
                # The writes go in the reading's order: values 1, 2, the literal, N, then value 3's delete. The first
                # is lost in transport before it runs (read back as left, issued again); the literal's reply is lost
                # after it ran (read back through regread as intended).
                guest.flaky = {"write": 1}
                guest.write_results = {3: (255, "", harness.TRANSPORT_LINES[0])}
                guest.applied_anyway = {3}
            else:
                guest.write_results = {1: (1, "", "ERROR: Access is denied.")}
            runner = guest.runner(_solve(run_id, literal), config)
            sent: list[str] = []

            def recording(argv, _runner=runner, _sent=sent, **kwargs):
                _sent.append(argv[6])
                return _runner(argv, **kwargs)
            try:
                run_caepipe_round_trip(config, run_id, correlation.MBF_TEXT, runner=recording, gui_fallback=False)
            except harness.RestoreWriteFailed:
                assert variant == "failed_restore"
            names = {_step_name(command) for command in sent}
            assert {"attempt", "fence", "launcher_query", "job_query", "chunk_read", "regset" if variant ==
                    "exact_restore" else "settings_reading"} <= names, (run_id, variant, sorted(names))
            if variant == "exact_restore":
                assert {"value_write", "value_delete", "regset", "literal_readback"} <= names, sorted(names)
            for command in sent:
                rows.append({"run": run_id, "variant": variant, "step": _step_name(command),
                             "length": harness._guest_command_length(config.vm_name, command),
                             "command": command})
        # The chunk read at the largest offset a collection can reach, by the collection's own builder.
        largest = harness._read_command(config, "chunk", {
            "path": harness._win(WORK, run_id, "f" * 32, PRINT_NAME),
            "offset": harness.COLLECT_MAX_FILE_BYTES - 1, "length": harness.COLLECT_CHUNK_BYTES})
        rows.append({"run": run_id, "variant": "largest_offset", "step": "chunk_read",
                     "length": harness._guest_command_length(config.vm_name, largest), "command": largest})
    return rows


def test_the_batch_is_the_56_runs_of_its_families():
    runs = batch_runs()
    assert len(runs) == 56 and len(set(runs)) == 56


def test_every_guest_command_of_every_batch_run_is_within_the_limit(tmp_path, monkeypatch):
    rows = command_lengths(tmp_path, monkeypatch)
    for row in rows:  # every command, without exception
        assert row["length"] <= harness.GUEST_COMMAND_LIMIT and " -EncodedCommand " not in row["command"], row
    assert {row["run"] for row in rows} == set(batch_runs())
