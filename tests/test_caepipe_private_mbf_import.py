#!/usr/bin/env python3
"""Private regression of the DEL-17-04 MBF importer on CAEPIPE-exported MBFs (Design `caepipe-mbf-import.md` §18.3).

The exchange folder named by SWBPIPE_CAEPIPE_EXCHANGE holds, per correlation case, `caepipe_out/exported.mbf`
(CAEPIPE's MBF export after importing the case's writer MBF: CAEPIPE output, private) and the writer sidecar
`caepipe_in/id_map.json`. The check imports each exported MBF as unbound, rebinding the sidecar by CAEPIPE name
(every rebound id unconfirmed), with the case's project_invented material declaration, and compares the canonical
forms of the case's SWBPIPE model and the import within the writer's tolerances. Its classification is one of
`equal_within_writer_tolerance`, `differs` or `refused`.

All outcomes, field names and refusal codes stay in `cases/<id>/imported/import_outcome.json` inside the exchange
folder, written only on an explicit recording run (SWBPIPE_CAEPIPE_IMPORT_RECORD=1). Standard output and assertion
messages carry operational status only: the case id, checked or skipped and why, and the private record's path.
The check fails when a case's classification moves from the recorded one, naming only the case and the path.

The public tests below exercise the skip and the operational-only output on an invented exchange folder whose
"exported" MBF is the project's own writer output; they hold no CAEPIPE output.
"""

from __future__ import annotations

from dataclasses import dataclass
import hashlib
import json
import os
from pathlib import Path
import sys
from typing import Any, Callable, Mapping

import pytest


ROOT = Path(__file__).resolve().parents[1]
for extra in (ROOT, ROOT / "tests"):
    if str(extra) not in sys.path:
        sys.path.insert(0, str(extra))

import caepipe_mbf_canonical as canon  # noqa: E402
from core.handoff.caepipe_external.harness import EXPORT_ROUTE, contained, contained_dir, write_private  # noqa: E402
from core.handoff.caepipe_mbf.importer import IMPORT_PROFILE_ID, import_caepipe_mbf, inside_git_work_tree  # noqa: E402
from tools.caepipe_correlation.run_pilot import load_cases, render_case_mbf  # noqa: E402


ENV_VAR = "SWBPIPE_CAEPIPE_EXCHANGE"
RECORD_VAR = "SWBPIPE_CAEPIPE_IMPORT_RECORD"
OUTCOME_KIND = "swbpipe.caepipe_mbf_import_outcome"
CLASSIFICATIONS = ("equal_within_writer_tolerance", "differs", "refused")


@dataclass
class CaseStatus:
    case_id: str
    status: str  # checked | recorded | skipped | moved
    detail: str  # operational only: the reason for a skip or the private record's path


def exchange_root(environ: Mapping[str, str]) -> Path | str:
    """The configured exchange folder, or the reason the check skips."""
    configured = environ.get(ENV_VAR)
    if not configured:
        return f"{ENV_VAR} is not set; the private MBF import regression needs the exchange folder"
    root = Path(configured).expanduser().resolve()
    if not root.is_dir() or not any(root.iterdir()):
        return f"the exchange folder named by {ENV_VAR} is missing or empty"
    if inside_git_work_tree(root):
        return f"the exchange folder named by {ENV_VAR} lies inside a Git work tree; private data stays outside"
    return root


def classify(case: Mapping[str, Any], exported: bytes, sidecar: Mapping[str, Any]) -> dict[str, Any]:
    """The private outcome of importing one exported MBF (never printed)."""
    declaration = {"*": {"origin": "project_invented", "source_name": f"invented correlation case {case['case_id']}"}}
    result = import_caepipe_mbf(exported, sidecar=sidecar, material_provenance=declaration, rebind_by_node_number=True,
                                source_label=f"{case['case_id']}/caepipe_out/exported.mbf")
    outcome: dict[str, Any] = {"document_kind": OUTCOME_KIND, "case_id": case["case_id"], "importer_profile": IMPORT_PROFILE_ID,
                               "exported_mbf_sha256": hashlib.sha256(exported).hexdigest(),
                               "refusal_codes": sorted({item["code"] for item in result.diagnostics
                                                        if item["severity"] == "error"}),
                               "fields": []}
    if result.status != "imported":
        return {**outcome, "classification": "refused"}
    writer, writer_text, _ = render_case_mbf(case)
    labels = {"Sustained (W+P)": "sustained", "Expansion (T1)": "expansion"}
    kinds_a = {case_id: labels[label] for case_id, label in writer.load_case_map.items()}
    kinds_i = {item["id"]: labels.get(item["label"], "other") for item in result.model["load_cases"]}
    nu = {record["swbpipe_material"]: record["poisson_ratio"] for record in writer.material_map.values()}
    reference = canon.canonical(case["model"], writer.node_map, kinds_a, oriented=True, writer_nu=nu)
    limits = canon.bounds(case["model"], writer, canon.read_printed(writer_text), writer.node_map, reference)
    reference, limits = canon.unoriented(reference, limits)
    names = {swb_id: name for name, swb_id in result.id_map["node_map"].items()}
    try:
        imported = canon.canonical(result.model, names, kinds_i, oriented=False)
        failures = canon.compare(reference, imported, limits).failures
    except (KeyError, ValueError, ZeroDivisionError) as error:  # a structure the comparison cannot pair
        failures = [f"comparison not formed: {type(error).__name__}"]
    outcome["fields"] = [item.split(":")[0] for item in failures]
    return {**outcome, "classification": "differs" if failures else "equal_within_writer_tolerance"}


def bound_export(root: Path, case_dir: Path, exported: bytes) -> bool:
    """True when the run record's export entry names a successful export with these exact bytes."""

    try:
        record = json.loads(contained(root, case_dir / "caepipe_out" / "run_record.json").read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return False
    entry = record.get("mbf_export") if isinstance(record, dict) else None
    if not isinstance(entry, dict) or entry.get("route") != EXPORT_ROUTE:
        return False
    exported_mbf = entry.get("exported_mbf")
    return isinstance(exported_mbf, dict) and exported_mbf.get("sha256") == hashlib.sha256(exported).hexdigest()


def check_exchange(root: Path, *, record: bool, emit: Callable[[str], None] = print,
                   cases: list[Mapping[str, Any]] | None = None) -> list[CaseStatus]:
    statuses = []
    root = root.resolve()
    for case in cases if cases is not None else load_cases():
        case_id = case["case_id"]
        case_dir = contained(root, root / "cases" / case_id)
        exported_path = contained(root, case_dir / "caepipe_out" / "exported.mbf")
        sidecar_path = contained(root, case_dir / "caepipe_in" / "id_map.json")
        record_path = contained(root, case_dir / "imported" / "import_outcome.json")
        relative = record_path.relative_to(root).as_posix()
        if not exported_path.is_file():
            statuses.append(CaseStatus(case_id, "skipped", "no caepipe_out/exported.mbf"))
        elif not bound_export(root, case_dir, exported_path.read_bytes()):
            statuses.append(CaseStatus(case_id, "skipped", "caepipe_out/exported.mbf is not bound to a successful "
                                                           "export by caepipe_out/run_record.json"))
        elif not sidecar_path.is_file():
            statuses.append(CaseStatus(case_id, "skipped", "no caepipe_in/id_map.json writer sidecar"))
        else:
            outcome = classify(case, exported_path.read_bytes(), json.loads(sidecar_path.read_text(encoding="utf-8")))
            if record:
                contained_dir(root, record_path.parent)
                write_private(root, record_path, json.dumps(outcome, indent=2, sort_keys=True) + "\n")
                statuses.append(CaseStatus(case_id, "recorded", relative))
            elif not record_path.is_file():
                statuses.append(CaseStatus(case_id, "skipped", f"no recorded classification at {relative}; "
                                                               f"record with {RECORD_VAR}=1"))
            else:
                recorded = json.loads(record_path.read_text(encoding="utf-8")).get("classification")
                moved = recorded != outcome["classification"]
                statuses.append(CaseStatus(case_id, "moved" if moved else "checked", relative))
        emit(f"{statuses[-1].case_id}: {statuses[-1].status}; {statuses[-1].detail}")
    return statuses


def test_private_exported_mbfs_keep_their_recorded_classification():
    root = exchange_root(os.environ)
    if isinstance(root, str):
        pytest.skip(root)
    statuses = check_exchange(root, record=os.environ.get(RECORD_VAR) == "1")
    if all(status.status == "skipped" for status in statuses):
        pytest.skip("no correlation case in the exchange folder has an exported MBF and a recorded classification")
    moved = [f"{status.case_id} ({status.detail})" for status in statuses if status.status == "moved"]
    assert not moved, f"import classification moved from the private record: {', '.join(moved)}"


# --- public tests of the skip and the operational-only output (invented data only) -----------------------------
def test_the_check_skips_with_its_reason_when_no_exchange_folder_is_configured(tmp_path):
    assert ENV_VAR in exchange_root({})
    assert "missing or empty" in exchange_root({ENV_VAR: str(tmp_path / "absent")})
    assert "missing or empty" in exchange_root({ENV_VAR: str(tmp_path)})


def invented_exchange(tmp_path: Path, case: Mapping[str, Any]) -> Path:
    """An invented exchange folder whose 'exported' MBF is the project's own writer output for the case."""
    root = tmp_path / "exchange"
    writer, _, mbf_text = render_case_mbf(case)
    data = mbf_text.encode("ascii")
    for layer, name, content in (("caepipe_in", "id_map.json", json.dumps({
            **{key: value for key, value in writer.to_dict().items() if key.endswith("_map")},
            "mbf_sha256": hashlib.sha256(data).hexdigest()})), ("caepipe_out", "exported.mbf", mbf_text),
            ("caepipe_out", "run_record.json", json.dumps({"mbf_export": {
                "route": EXPORT_ROUTE, "exported_mbf": {"sha256": hashlib.sha256(data).hexdigest()}}}))):
        path = root / "cases" / case["case_id"] / layer
        path.mkdir(parents=True, exist_ok=True)
        (path / name).write_bytes(content.encode("ascii"))
    return root


def test_record_check_and_move_print_operational_status_only(tmp_path):
    case = next(item for item in load_cases() if item["case_id"] == "cpc-04-l-bend-thermal")
    root = invented_exchange(tmp_path, case)
    assert exchange_root({ENV_VAR: str(root)}) == root.resolve()
    lines: list[str] = []
    record = f"cases/{case['case_id']}/imported/import_outcome.json"
    assert [status.status for status in check_exchange(root, record=False, emit=lines.append, cases=[case])] == ["skipped"]
    assert [status.status for status in check_exchange(root, record=True, emit=lines.append, cases=[case])] == ["recorded"]
    stored = json.loads((root / record).read_text(encoding="utf-8"))
    assert stored["classification"] == "equal_within_writer_tolerance" and stored["document_kind"] == OUTCOME_KIND
    assert [status.status for status in check_exchange(root, record=False, emit=lines.append, cases=[case])] == ["checked"]
    stored["classification"] = "refused"
    (root / record).write_text(json.dumps(stored), encoding="utf-8")
    statuses = check_exchange(root, record=False, emit=lines.append, cases=[case])
    assert [status.status for status in statuses] == ["moved"]
    assert lines == [
        f"{case['case_id']}: skipped; no recorded classification at {record}; record with {RECORD_VAR}=1",
        f"{case['case_id']}: recorded; {record}",
        f"{case['case_id']}: checked; {record}",
        f"{case['case_id']}: moved; {record}",
    ]
    for line in lines:  # no classification, field, code or value leaves the private record
        assert not any(word in line for word in (*CLASSIFICATIONS, "MBF-IMPORT", "sha256"))


def test_a_case_without_an_exported_mbf_is_skipped(tmp_path):
    case = next(item for item in load_cases() if item["case_id"] == "cpc-01-cantilever-force")
    root = invented_exchange(tmp_path, case)
    (root / "cases" / case["case_id"] / "caepipe_out" / "exported.mbf").unlink()
    lines: list[str] = []
    assert [status.status for status in check_exchange(root, record=True, emit=lines.append, cases=[case])] == ["skipped"]
    assert lines == [f"{case['case_id']}: skipped; no caepipe_out/exported.mbf"]
    assert not (root / "cases" / case["case_id"] / "imported").exists()


def bind(exported: Path) -> None:
    """Record an invented successful export of exactly these bytes, as the harness does."""

    (exported.parent / "run_record.json").write_text(json.dumps({"mbf_export": {
        "route": EXPORT_ROUTE, "exported_mbf": {"sha256": hashlib.sha256(exported.read_bytes()).hexdigest()}}}),
        encoding="utf-8")


def test_an_edited_export_is_classified_privately_as_refused_or_differs(tmp_path):
    case = next(item for item in load_cases() if item["case_id"] == "cpc-01-cantilever-force")
    root = invented_exchange(tmp_path, case)
    exported = root / "cases" / case["case_id"] / "caepipe_out" / "exported.mbf"
    exported.write_bytes(exported.read_bytes().replace(b"\r\nPIPE\r\n", b"\r\nSOILS\r\nS1,0,1\r\nPIPE\r\n"))
    bind(exported)
    check_exchange(root, record=True, emit=lambda line: None, cases=[case])
    stored = json.loads((root / "cases" / case["case_id"] / "imported" / "import_outcome.json").read_text(encoding="utf-8"))
    assert stored["classification"] == "refused" and "MBF-IMPORT-SECTION-UNSUPPORTED" in stored["refusal_codes"]
    exported.write_bytes(exported.read_bytes().replace(b"\r\nSOILS\r\nS1,0,1", b"").replace(b"X6000", b"X6100"))
    bind(exported)
    check_exchange(root, record=True, emit=lambda line: None, cases=[case])
    stored = json.loads((root / "cases" / case["case_id"] / "imported" / "import_outcome.json").read_text(encoding="utf-8"))
    assert stored["classification"] == "differs" and stored["fields"]



@pytest.mark.parametrize("label, export_entry", [
    ("no export entry", None),
    ("a refused export", {"route": "not_bound", "reason": "invented"}),
    ("another file's hash", {"route": EXPORT_ROUTE, "exported_mbf": {"sha256": "0" * 64}}),
])
def test_an_export_the_run_record_does_not_bind_is_skipped(tmp_path, label, export_entry):
    case = next(item for item in load_cases() if item["case_id"] == "cpc-04-l-bend-thermal")
    root = invented_exchange(tmp_path, case)
    record = root / "cases" / case["case_id"] / "caepipe_out" / "run_record.json"
    record.write_text(json.dumps({} if export_entry is None else {"mbf_export": export_entry}), encoding="utf-8")
    statuses = check_exchange(root, record=True, emit=lambda line: None, cases=[case])
    assert [status.status for status in statuses] == ["skipped"], label
    assert "not bound" in statuses[0].detail
    assert not (root / "cases" / case["case_id"] / "imported").exists()
