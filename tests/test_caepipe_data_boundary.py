#!/usr/bin/env python3
"""Targeted data-boundary guard for the CAEPIPE correlation pilot (DEL-17-05 Design §8).

CAEPIPE outputs and everything derived from them stay in the private exchange
folder. These checks look at the files Git tracks or would add (tracked plus
untracked, not ignored): exchange-style paths, CAEPIPE-headed files, parsed,
comparison and run-record JSON (by document kind or private-layer file name),
unless declared invented by a provenance sidecar; and at the pilot command's
unconfigured path. For the MBF importer (DEL-17-04 Design `caepipe-mbf-import.md`
§18.4): every tracked MBF needs an invented-provenance sidecar, as does every import
report or private import outcome, and no file sits under an exchange-style
`cases/<id>/imported/` path.
"""

from __future__ import annotations

import json
import os
from pathlib import Path
import subprocess
import sys

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import HarnessSkip, resolve_harness_config  # noqa: E402
from core.handoff.caepipe_external.benchmark import BENCHMARK_RECORD_KIND  # noqa: E402
from core.handoff.caepipe_external.correlation import COMPARISON_RECORD_KIND, SERIES_RECORD_KIND  # noqa: E402
from core.handoff.caepipe_external.harness import ENV_VAR, RUN_RECORD_KIND  # noqa: E402
from core.handoff.caepipe_external.print_csv import DOCUMENT_KIND as PARSED_CSV_KIND  # noqa: E402
from core.handoff.caepipe_mbf.importer import REPORT_KIND as IMPORT_REPORT_KIND  # noqa: E402
from tools.caepipe_correlation.run_pilot import MANIFEST_KIND, PARSER_DIAGNOSTICS_KIND  # noqa: E402


FORBIDDEN_SEGMENTS = {"caepipe-oracle", "caepipe_out", "oracle_cases"}
# The exchange folder's `_harness/` holds the harness's copy of the guest script and its run lock. The project's
# unrelated `_harness/adapter.yaml` (the agent harness adapter) is not an exchange path, so the segment is
# refused only together with the exchange file names.
FORBIDDEN_HARNESS_FILES = {"Invoke-CaepipeRoundTrip.ps1", "round_trip.lock"}
# Artefacts of the private exchange layers (Design §5): CAEPIPE output or derived from it.
PRIVATE_LAYER_NAMES = {"caepipe_results.json", "parser_diagnostics.json", "comparison.json", "run_record.json",
                       "guest_log.json", "guest_stdout.txt", "guest_stderr.txt", "case_manifest.json",
                       "import_outcome.json", "export_guest_log.json", "export_guest_stdout.txt",
                       "export_guest_stderr.txt", "benchmark_comparison.json"}
# The private MBF import regression's outcome record (tests/test_caepipe_private_mbf_import.py).
IMPORT_OUTCOME_KIND = "swbpipe.caepipe_mbf_import_outcome"
PRIVATE_DOCUMENT_KINDS = {PARSED_CSV_KIND, PARSER_DIAGNOSTICS_KIND, COMPARISON_RECORD_KIND, SERIES_RECORD_KIND,
                          RUN_RECORD_KIND, MANIFEST_KIND, IMPORT_REPORT_KIND, IMPORT_OUTCOME_KIND,
                          BENCHMARK_RECORD_KIND}
# The target tool's output markers: the leading label of its results files and its section titles.
OUTPUT_MARKERS = {
    "file_label": b"Caepipe",
    "section_titles": (
        "Loads on Anchors in Global Coordinates",
        "Loads on Anchors in Local Coordinates",
        "Element forces in global coordinates",
        "Pipe element forces (local)",
        "Loads on restraints",
        ": Displacements",
        "Min/Max Displacements",
    ),
}


def repository_files() -> tuple[Path, list[str]]:
    top = subprocess.run(["git", "-C", str(ROOT), "rev-parse", "--show-toplevel"], capture_output=True, text=True,
                         check=False)
    if top.returncode != 0:
        pytest.skip("not inside a Git work tree")
    repo = Path(top.stdout.strip())
    listed = subprocess.run(["git", "-C", str(repo), "ls-files", "--cached", "--others", "--exclude-standard", "-z"],
                            capture_output=True, check=True)
    return repo, [item for item in listed.stdout.decode("utf-8").split("\0") if item]


def declared_invented(path: Path) -> bool:
    """True when `path` has a provenance sidecar declaring it invented and not produced by CAEPIPE."""
    provenance = path.with_name(path.stem + ".provenance.json")
    try:
        declared = json.loads(provenance.read_text(encoding="utf-8"))
    except (FileNotFoundError, UnicodeDecodeError, json.JSONDecodeError):
        return False
    return (isinstance(declared, dict) and declared.get("produced_by_caepipe") is False
            and "invented" in str(declared.get("origin", "")))


def private_artefact_reason(path: Path) -> str | None:
    """Why `path` looks like a private exchange-layer artefact (parsed, comparison, run record), or None.

    Invented fixtures with a provenance sidecar declaring them invented are allowed.
    """
    reason = None
    if path.name in PRIVATE_LAYER_NAMES:
        reason = f"private-layer artefact name {path.name}"
    elif path.suffix == ".json":
        try:
            data = path.read_bytes()
        except (IsADirectoryError, FileNotFoundError, PermissionError):
            return None
        if b"openpipestress.caepipe" in data or b"swbpipe.caepipe" in data:
            try:
                kind = json.loads(data).get("document_kind")
            except (UnicodeDecodeError, json.JSONDecodeError, AttributeError):
                kind = None
            if kind in PRIVATE_DOCUMENT_KINDS:
                reason = f"private document_kind {kind}"
    if reason is None or declared_invented(path):
        return None
    return reason


def test_no_private_layer_artefact_is_tracked():
    repo, files = repository_files()
    offending = [f"{name}: {reason}" for name in files
                 if (reason := private_artefact_reason(repo / name)) is not None]
    assert offending == []


def test_private_artefact_detector_on_synthetic_files(tmp_path):
    def write(name: str, value) -> Path:
        path = tmp_path / name
        path.write_text(json.dumps(value) if not isinstance(value, str) else value, encoding="utf-8")
        return path

    assert private_artefact_reason(write("renamed_parse.json", {"document_kind": PARSED_CSV_KIND})) is not None
    assert private_artefact_reason(write("renamed_cmp.json", {"document_kind": COMPARISON_RECORD_KIND})) is not None
    assert private_artefact_reason(write("renamed_run.json", {"document_kind": RUN_RECORD_KIND})) is not None
    assert private_artefact_reason(write("renamed_diag.json", {"document_kind": PARSER_DIAGNOSTICS_KIND,
                                                                "diagnostics": []})) is not None
    assert private_artefact_reason(write("renamed_manifest.json", {"document_kind": MANIFEST_KIND})) is not None
    # The private series evaluation shares its file name with the public series definition; its kind decides.
    assert private_artefact_reason(write("shear-deformation.json", {"document_kind": SERIES_RECORD_KIND})) is not None
    for name in sorted(PRIVATE_LAYER_NAMES):
        assert private_artefact_reason(write(name, {"anything": 1})) is not None, name
    assert private_artefact_reason(write("renamed_import.json", {"document_kind": IMPORT_REPORT_KIND})) is not None
    assert private_artefact_reason(write("renamed_outcome.json", {"document_kind": IMPORT_OUTCOME_KIND})) is not None
    assert private_artefact_reason(write("other.json", {"document_kind": "openpipestress.caepipe_other"})) is None
    assert private_artefact_reason(write("broken.json", "openpipestress.caepipe {not json")) is None
    assert private_artefact_reason(write("list.json", ["openpipestress.caepipe"])) is None
    invented = write("invented_cmp.json", {"document_kind": COMPARISON_RECORD_KIND})
    write("invented_cmp.provenance.json", {"produced_by_caepipe": False, "origin": "invented for a test"})
    assert private_artefact_reason(invented) is None
    unclear = write("unclear_cmp.json", {"document_kind": COMPARISON_RECORD_KIND})
    write("unclear_cmp.provenance.json", {"produced_by_caepipe": True, "origin": "invented"})
    assert private_artefact_reason(unclear) is not None


def exchange_style(name: str) -> bool:
    parts = name.split("/")
    if FORBIDDEN_SEGMENTS & set(parts):
        return True
    if "_harness" in parts[:-1] and parts[-1] in FORBIDDEN_HARNESS_FILES:
        return True
    # The importer's private layer: cases/<case id>/imported/... (DEL-17-05 Design §5).
    return any(part == "imported" and index >= 2 and parts[index - 2] == "cases" for index, part in enumerate(parts[:-1]))


def test_no_exchange_style_path_is_tracked():
    _, files = repository_files()
    assert [name for name in files if exchange_style(name)] == []


def test_exchange_style_detector_on_synthetic_paths():
    assert exchange_style("x/cases/cpc-01/imported/import_outcome.json")
    assert exchange_style("cases/cpc-01/imported/model.json")
    assert exchange_style("a/caepipe_out/exported.mbf")
    assert not exchange_style("docs/imported/notes.md")
    assert not exchange_style("cases/imported")


def test_tracked_mbf_files_are_declared_invented():
    repo, files = repository_files()
    offending = [name for name in files if name.lower().endswith(".mbf") and not declared_invented(repo / name)]
    assert offending == []


def test_mbf_detector_on_synthetic_files(tmp_path):
    plain = tmp_path / "model.mbf"
    plain.write_text("INVENTED\r\n", encoding="utf-8")
    assert not declared_invented(plain)
    (tmp_path / "model.provenance.json").write_text(json.dumps({"produced_by_caepipe": False, "origin": "invented"}),
                                                    encoding="utf-8")
    assert declared_invented(plain)


def test_caepipe_headed_files_are_declared_invented():
    repo, files = repository_files()
    offending = []
    for name in files:
        path = repo / name
        try:
            with path.open("rb") as handle:
                head = handle.read(16)
        except (IsADirectoryError, FileNotFoundError, PermissionError):
            continue
        if not head.lstrip(b"\xef\xbb\xbf").startswith(OUTPUT_MARKERS["file_label"]):
            continue
        if not declared_invented(path):
            offending.append(name)
    assert offending == []


def test_no_caepipe_section_titles_under_the_public_case_folder():
    repo, files = repository_files()
    prefix = str((ROOT / "validation" / "caepipe_correlation").relative_to(repo)) + "/"
    offending = []
    for name in files:
        if not name.startswith(prefix):
            continue
        text = (repo / name).read_text(encoding="utf-8", errors="replace")
        offending.extend(f"{name}: {title}" for title in OUTPUT_MARKERS["section_titles"] if title in text)
    assert offending == []


def test_harness_refuses_an_exchange_folder_inside_a_git_work_tree(tmp_path):
    repo = tmp_path / "repo"
    subprocess.run(["git", "init", "-q", str(repo)], check=True)
    folder = repo / "exchange"
    folder.mkdir()
    (folder / ".gitignore").write_text("*\n", encoding="utf-8")
    result = resolve_harness_config({ENV_VAR: str(folder), "PATH": os.environ.get("PATH", "")})
    assert isinstance(result, HarnessSkip)
    assert "inside a Git work tree" in result.reason


def test_pilot_command_without_configuration_writes_nothing(tmp_path):
    def status() -> str:
        return subprocess.run(["git", "-C", str(ROOT), "status", "--porcelain", "--untracked-files=all"],
                              capture_output=True, text=True, check=True).stdout

    env = {key: value for key, value in os.environ.items() if key != ENV_VAR}
    env["PYTHONDONTWRITEBYTECODE"] = "1"
    before = status()
    completed = subprocess.run([sys.executable, "-m", "tools.caepipe_correlation.run_pilot"], cwd=ROOT, env=env,
                               capture_output=True, text=True, check=False)
    assert completed.returncode == 0, completed.stderr
    assert "CAEPIPE harness skipped" in completed.stdout and ENV_VAR in completed.stdout
    assert status() == before
