#!/usr/bin/env python3
"""Private regression of the DEL-17-04 MBF importer on CAEPIPE-exported MBFs (Design `caepipe-mbf-import.md` §18.3).

The exchange folder named by SWBPIPE_CAEPIPE_EXCHANGE holds, per correlation case, `caepipe_out/exported.mbf`
(CAEPIPE's MBF export after importing the case's writer MBF: CAEPIPE output, private) and the writer sidecar
`caepipe_in/id_map.json`. The check imports each exported MBF as unbound, rebinding the sidecar by CAEPIPE name
(every rebound id unconfirmed), with the case's project_invented material declaration, and compares the canonical
forms of the case's SWBPIPE model and the import within the writer's tolerances. Its classification is one of
`equal_within_writer_tolerance`, `differs` or `refused`.

Authored cases (validation/caepipe_correlation/authored/, invented hand-written MBFs) hold, per case, the authored
file as `caepipe_in/model.mbf` and CAEPIPE's export of it as `caepipe_out/exported.mbf`. The check imports both as
unbound, with the manifest's pressure-closure declaration where it has one. For a mapped case it compares the
authored import, as the reference, with the export's import within the writer's tolerances (the SI writer re-renders
the reference to derive them): `equal_within_writer_tolerance`, `differs`, or `refused` when the export's import is
refused. For a refused case it compares refusals: `refused_alike` when the export's import is refused with the
manifest's named code, `refused_differently` when it is refused otherwise, and `differs` when it imports.

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

# Reads its directory scans stand for, for CI selection (tools/ci/python_test_reads.py). Its only scan lists the private exchange folder an environment variable names, outside the project.
CI_READS = ()


ROOT = Path(__file__).resolve().parents[1]
for extra in (ROOT, ROOT / "tests"):
    if str(extra) not in sys.path:
        sys.path.insert(0, str(extra))

import caepipe_mbf_canonical as canon  # noqa: E402
from core.handoff.caepipe_external.harness import EXPORT_ROUTE, contained, contained_dir, write_private  # noqa: E402
from core.handoff.caepipe_mbf.importer import (  # noqa: E402
    IMPORT_PROFILE_ID, import_caepipe_mbf, inside_git_work_tree, load_sidecar)
from core.handoff.caepipe_mbf import render_caepipe_mbf_si  # noqa: E402
from tools.caepipe_correlation.run_pilot import (  # noqa: E402
    authored_mbf_bytes, load_authored_cases, load_cases, render_case_mbf)


ENV_VAR = "SWBPIPE_CAEPIPE_EXCHANGE"
RECORD_VAR = "SWBPIPE_CAEPIPE_IMPORT_RECORD"
CASES_VAR = "SWBPIPE_CAEPIPE_IMPORT_CASES"
OUTCOME_KIND = "swbpipe.caepipe_mbf_import_outcome"
CLASSIFICATIONS = ("equal_within_writer_tolerance", "differs", "refused")
AUTHORED_CLASSIFICATIONS = ("equal_within_writer_tolerance", "differs", "refused_alike", "refused_differently",
                            "refused")
LOAD_CASE_KINDS = {"Sustained (W+P)": "sustained", "Expansion (T1)": "expansion"}


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


def classify(case: Mapping[str, Any], exported: bytes, sidecar: Any) -> dict[str, Any]:
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
    outcome["fields"] = compared_fields(case["model"], writer, writer_text, result)
    return {**outcome, "classification": "differs" if outcome["fields"] else "equal_within_writer_tolerance"}


def compared_fields(model: Mapping[str, Any], writer: Any, writer_text: str, result: Any) -> list[str]:
    """The canonical fields of `model` (rendered by the writer as `writer`) that the import `result` does not
    reproduce within the writer's tolerances; empty when every field agrees."""
    kinds_a = {case_id: LOAD_CASE_KINDS[label] for case_id, label in writer.load_case_map.items()}
    kinds_i = {item["id"]: LOAD_CASE_KINDS.get(item["label"], "other") for item in result.model["load_cases"]}
    nu = {record["swbpipe_material"]: record["poisson_ratio"] for record in writer.material_map.values()}
    try:
        reference = canon.canonical(model, writer.node_map, kinds_a, oriented=True, writer_nu=nu)
        limits = canon.bounds(model, writer, canon.read_printed(writer_text), writer.node_map, reference)
        reference, limits = canon.unoriented(reference, limits)
        names = {swb_id: name for name, swb_id in result.id_map["node_map"].items()}
        imported = canon.canonical(result.model, names, kinds_i, oriented=False)
        failures = canon.compare(reference, imported, limits).failures
    except (KeyError, ValueError, ZeroDivisionError, StopIteration) as error:  # a structure the comparison cannot pair
        failures = [f"comparison not formed: {type(error).__name__}"]
    return [item.split(":")[0] for item in failures]


def error_codes(result: Any) -> list[str]:
    return sorted({item["code"] for item in result.diagnostics if item["severity"] == "error"})


def classify_authored(case: Mapping[str, Any], authored: bytes, exported: bytes) -> dict[str, Any]:
    """The private outcome of importing one authored MBF and CAEPIPE's export of it (never printed)."""
    declaration = {"*": {"origin": "project_invented", "source_name": f"invented authored case {case['case_id']}"}}
    options = {"pressure_closure": case["pressure_closure"]} if case.get("pressure_closure") else {}

    def read(data: bytes, label: str) -> Any:
        return import_caepipe_mbf(data, material_provenance=declaration, source_label=f"{case['case_id']}/{label}",
                                  **options)

    reference, result = read(authored, "caepipe_in/model.mbf"), read(exported, "caepipe_out/exported.mbf")
    outcome: dict[str, Any] = {"document_kind": OUTCOME_KIND, "case_id": case["case_id"], "case_kind": "authored",
                               "importer_profile": IMPORT_PROFILE_ID,
                               "authored_mbf_sha256": hashlib.sha256(authored).hexdigest(),
                               "exported_mbf_sha256": hashlib.sha256(exported).hexdigest(),
                               "authored_refusal_codes": error_codes(reference),
                               "refusal_codes": error_codes(result), "fields": []}
    expected = case["importer_expectation"]
    if expected["status"] == "refused":
        if reference.status == "imported" or expected["code"] not in outcome["authored_refusal_codes"]:
            raise AssertionError(f"{case['case_id']}: the authored MBF no longer meets its manifest expectation")
        if result.status == "imported":
            return {**outcome, "classification": "differs"}
        alike = expected["code"] in outcome["refusal_codes"]
        return {**outcome, "classification": "refused_alike" if alike else "refused_differently"}
    if reference.status != "imported":
        raise AssertionError(f"{case['case_id']}: the authored MBF no longer meets its manifest expectation")
    if result.status != "imported":
        return {**outcome, "classification": "refused"}
    writer = render_caepipe_mbf_si(reference.model, title=case["case_id"])
    if writer.status != "written":
        return {**outcome, "fields": ["reference not rendered by the writer"], "classification": "differs"}
    outcome["fields"] = compared_fields(reference.model, writer, writer.text, result)
    return {**outcome, "classification": "differs" if outcome["fields"] else "equal_within_writer_tolerance"}


def bound_export(root: Path, case_dir: Path, exported: bytes, *, solved: str | None = None) -> bool:
    """True when the run record's export entry names a successful export with these exact bytes and, with `solved`,
    the export is of the model imported from the MBF with that SHA-256 (an authored case's file)."""

    try:
        record = json.loads(contained(root, case_dir / "caepipe_out" / "run_record.json").read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return False
    entry = record.get("mbf_export") if isinstance(record, dict) else None
    if not isinstance(entry, dict) or entry.get("route") != EXPORT_ROUTE:
        return False
    exported_mbf = entry.get("exported_mbf")
    if solved is not None and (entry.get("solved_mbf_sha256") != solved or (record.get("mbf") or {}).get("sha256") != solved):
        return False
    return isinstance(exported_mbf, dict) and exported_mbf.get("sha256") == hashlib.sha256(exported).hexdigest()


def check_exchange(root: Path, *, record: bool, emit: Callable[[str], None] = print,
                   cases: list[Mapping[str, Any]] | None = None,
                   authored: list[Mapping[str, Any]] | None = None) -> list[CaseStatus]:
    """Check (or record) every writer case and every authored case that has a bound export."""
    statuses = []
    root = root.resolve()
    writer_cases = cases if cases is not None else load_cases()
    authored_cases = authored if authored is not None else ([] if cases is not None else load_authored_cases())
    for kind, case in [("writer", item) for item in writer_cases] + [("authored", item) for item in authored_cases]:
        case_id = case["case_id"]
        case_dir = contained(root, root / "cases" / case_id)
        exported_path = contained(root, case_dir / "caepipe_out" / "exported.mbf")
        sidecar_path = contained(root, case_dir / "caepipe_in" / "id_map.json")
        record_path = contained(root, case_dir / "imported" / "import_outcome.json")
        relative = record_path.relative_to(root).as_posix()
        authored_bytes = authored_mbf_bytes(case) if kind == "authored" else b""
        if not exported_path.is_file():
            statuses.append(CaseStatus(case_id, "skipped", "no caepipe_out/exported.mbf"))
        elif not bound_export(root, case_dir, exported_path.read_bytes(),
                              solved=hashlib.sha256(authored_bytes).hexdigest() if kind == "authored" else None):
            statuses.append(CaseStatus(case_id, "skipped", "caepipe_out/exported.mbf is not bound to a successful "
                                                           "export by caepipe_out/run_record.json"))
        elif kind == "writer" and not sidecar_path.is_file():
            statuses.append(CaseStatus(case_id, "skipped", "no caepipe_in/id_map.json writer sidecar"))
        else:
            outcome = (classify_authored(case, authored_bytes, exported_path.read_bytes()) if kind == "authored"
                       else classify(case, exported_path.read_bytes(), load_sidecar(sidecar_path)))
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


def selected_cases(environ: Mapping[str, str]) -> tuple[list[Mapping[str, Any]], list[Mapping[str, Any]]] | str:
    """(writer cases, authored cases): all of them, or only those SWBPIPE_CAEPIPE_IMPORT_CASES names (comma-separated
    ids), so that a recording run can re-record named cases alone; the reason when it names an unknown id."""
    writer, authored = load_cases(), load_authored_cases()
    named = {item.strip() for item in environ.get(CASES_VAR, "").split(",") if item.strip()}
    if not named:
        return writer, authored
    unknown = sorted(named - {case["case_id"] for case in writer + authored})
    if unknown:
        return f"{CASES_VAR} names unknown case id(s): {', '.join(unknown)}"
    return [case for case in writer if case["case_id"] in named], [case for case in authored if case["case_id"] in named]


def test_private_exported_mbfs_keep_their_recorded_classification():
    root = exchange_root(os.environ)
    if isinstance(root, str):
        pytest.skip(root)
    selected = selected_cases(os.environ)
    if isinstance(selected, str):
        pytest.fail(selected)
    statuses = check_exchange(root, record=os.environ.get(RECORD_VAR) == "1", cases=selected[0], authored=selected[1])
    if all(status.status == "skipped" for status in statuses):
        pytest.skip("no correlation case in the exchange folder has an exported MBF and a recorded classification")
    moved = [f"{status.case_id} ({status.detail})" for status in statuses if status.status == "moved"]
    assert not moved, f"import classification moved from the private record: {', '.join(moved)}"


# --- public tests of the skip and the operational-only output (invented data only) -----------------------------
def test_the_case_selection_names_writer_and_authored_cases():
    writer, authored = selected_cases({})
    assert len(writer) == len(load_cases()) and len(authored) == len(load_authored_cases())
    writer, authored = selected_cases({CASES_VAR: "cpc-04-l-bend-thermal, cpa-05-guide"})
    assert [case["case_id"] for case in writer] == ["cpc-04-l-bend-thermal"]
    assert [case["case_id"] for case in authored] == ["cpa-05-guide"]
    assert "cpa-99-absent" in selected_cases({CASES_VAR: "cpa-05-guide,cpa-99-absent"})


def test_the_check_skips_with_its_reason_when_no_exchange_folder_is_configured(tmp_path):
    assert ENV_VAR in exchange_root({})
    assert "missing or empty" in exchange_root({ENV_VAR: str(tmp_path / "absent")})
    assert "missing or empty" in exchange_root({ENV_VAR: str(tmp_path)})


def invented_exchange(tmp_path: Path, case: Mapping[str, Any]) -> Path:
    """An invented exchange folder whose 'exported' MBF is the project's own writer output for the case."""
    root = tmp_path / "exchange"
    writer, _, mbf_text = render_case_mbf(case)
    data = mbf_text.encode("ascii")
    for layer, name, content in (("caepipe_in", "id_map.json", json.dumps(
            writer.sidecar(hashlib.sha256(data).hexdigest()))), ("caepipe_out", "exported.mbf", mbf_text),
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


# --- authored cases: public checks of the invented fixtures and of the classification on invented data -------------
AUTHORED = {case["case_id"]: case for case in load_authored_cases()}


@pytest.mark.parametrize("case_id", sorted(AUTHORED))
def test_every_authored_fixture_meets_its_manifest_expectation(case_id):
    case = AUTHORED[case_id]
    path = ROOT / "validation" / "caepipe_correlation" / "authored" / case["file"]
    provenance = json.loads(path.with_name(f"{case_id}.provenance.json").read_text(encoding="utf-8"))
    assert provenance["produced_by_caepipe"] is False and "invented" in provenance["origin"]
    data = authored_mbf_bytes(case)
    assert data.startswith(b"INVENTED AUTHORED FIXTURE\r\n") and b"\n" not in data.replace(b"\r\n", b"")
    options = {"pressure_closure": case["pressure_closure"]} if case.get("pressure_closure") else {}
    result = import_caepipe_mbf(data, material_provenance={"*": {"origin": "project_invented", "source_name": "x"}},
                                source_label=case_id, **options)
    expected = case["importer_expectation"]
    assert result.status == expected["status"], result.diagnostics
    if expected["status"] == "refused":
        assert expected["code"] in error_codes(result)


def test_the_authored_set_covers_each_mapping_and_each_named_refusal():
    refused = {case["importer_expectation"].get("code") for case in AUTHORED.values()}
    assert refused >= {None, "MBF-IMPORT-SUPPORT-UNSUPPORTED", "MBF-IMPORT-TBD-17-04-010",
                       "MBF-IMPORT-MATERIAL-TEMPERATURE-DEPENDENT"}
    text = b"".join(authored_mbf_bytes(case) for case in AUTHORED.values()).decode("ascii")
    for construct in ("HGRA,SI,Z", ",CG", "STIFF=", "CLS(0,0)", "CK=", "TRAK(", "GGAP=", "MU=", "CLS(-", "CLS(0,)",
                      "DV(1,1,0)", "CNOD=", "CVS", "CCS=", "CUS(", "KR=", "ROTK(0,50,0)"):
        assert construct in text, construct
    assert any(case.get("pressure_closure") for case in AUTHORED.values())


def authored_exchange(tmp_path: Path, case: Mapping[str, Any], exported: bytes) -> Path:
    """An invented exchange folder holding the authored file and an invented 'export' of it, bound as the harness
    binds a real one."""
    root = tmp_path / "exchange"
    case_dir = root / "cases" / case["case_id"]
    for layer in ("caepipe_in", "caepipe_out"):
        (case_dir / layer).mkdir(parents=True, exist_ok=True)
    authored = authored_mbf_bytes(case)
    (case_dir / "caepipe_in" / "model.mbf").write_bytes(authored)
    rebind(case_dir / "caepipe_out" / "exported.mbf", exported, hashlib.sha256(authored).hexdigest())
    return root


def rebind(path: Path, exported: bytes, solved: str) -> None:
    path.write_bytes(exported)
    (path.parent / "run_record.json").write_text(json.dumps({"mbf": {"sha256": solved}, "mbf_export": {
        "route": EXPORT_ROUTE, "solved_mbf_sha256": solved,
        "exported_mbf": {"sha256": hashlib.sha256(exported).hexdigest()}}}), encoding="utf-8")


def recorded_class(root: Path, case_id: str, case: Mapping[str, Any]) -> str:
    check_exchange(root, record=True, emit=lambda line: None, cases=[], authored=[case])
    stored = json.loads((root / "cases" / case_id / "imported" / "import_outcome.json").read_text(encoding="utf-8"))
    assert stored["case_kind"] == "authored" and stored["document_kind"] == OUTCOME_KIND
    return stored["classification"]


def test_a_mapped_authored_case_is_classified_privately(tmp_path):
    case = AUTHORED["cpa-11-z-vertical-supports"]
    authored = authored_mbf_bytes(case)
    root = authored_exchange(tmp_path, case, authored)
    lines: list[str] = []
    record = "cases/cpa-11-z-vertical-supports/imported/import_outcome.json"
    assert [item.status for item in check_exchange(root, record=True, emit=lines.append, cases=[],
                                                   authored=[case])] == ["recorded"]
    assert [item.status for item in check_exchange(root, record=False, emit=lines.append, cases=[],
                                                   authored=[case])] == ["checked"]
    assert lines == [f"cpa-11-z-vertical-supports: recorded; {record}", f"cpa-11-z-vertical-supports: checked; {record}"]
    assert recorded_class(root, case["case_id"], case) == "equal_within_writer_tolerance"
    exported = root / "cases" / case["case_id"] / "caepipe_out" / "exported.mbf"
    solved = hashlib.sha256(authored).hexdigest()
    rebind(exported, authored.replace(b"CK=300", b"CK=310"), solved)
    assert recorded_class(root, case["case_id"], case) == "differs"
    rebind(exported, authored.replace(b"CK=300", b"CK=300,GGAP=4"), solved)
    assert recorded_class(root, case["case_id"], case) == "refused"


def test_a_refused_authored_case_compares_refusals_by_the_named_code(tmp_path):
    case = AUTHORED["cpa-22-skewed-rotational-stiffness"]
    authored = authored_mbf_bytes(case)
    root = authored_exchange(tmp_path, case, authored)
    assert recorded_class(root, case["case_id"], case) == "refused_alike"
    exported = root / "cases" / case["case_id"] / "caepipe_out" / "exported.mbf"
    solved = hashlib.sha256(authored).hexdigest()
    rebind(exported, authored.replace(b"CKR=30,DV(0,1,0)", b"CK=400,DV(1,1,0)"), solved)
    assert recorded_class(root, case["case_id"], case) == "refused_differently"
    rebind(exported, authored.replace(b"CKR=30", b"CK=30"), solved)
    assert recorded_class(root, case["case_id"], case) == "differs"


def test_an_authored_export_of_another_file_is_skipped(tmp_path):
    case = AUTHORED["cpa-05-guide"]
    root = authored_exchange(tmp_path, case, authored_mbf_bytes(case))
    exported = root / "cases" / case["case_id"] / "caepipe_out" / "exported.mbf"
    rebind(exported, exported.read_bytes(), "0" * 64)  # bound to a solve of some other MBF
    statuses = check_exchange(root, record=True, emit=lambda line: None, cases=[], authored=[case])
    assert [item.status for item in statuses] == ["skipped"] and "not bound" in statuses[0].detail
    assert not (root / "cases" / case["case_id"] / "imported").exists()
