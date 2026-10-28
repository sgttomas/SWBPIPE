"""Run the CAEPIPE correlation pilot (DEL-17-05 Design §3-§7) for every case, one case at a time.

From the project root:

    python3 -m tools.caepipe_correlation.run_pilot [--case ID ...] [--offline | --export-mbf | --export-only]
        [--runner PATH]
    python3 -m tools.caepipe_correlation.run_pilot --authored [--case ID ...] [--export-only]

Per case: the independent hand calculation, the SWBPIPE solve (headless runner),
the SI MBF (with the case's deliberate fault injection, if any), the live
CAEPIPE round trip through the configured virtual machine (skipped with
--offline, which reuses the CSV already stored in caepipe_out/), the parsed
CAEPIPE results, the three-way comparison record and the case manifest, all
under the private exchange folder named by SWBPIPE_CAEPIPE_EXCHANGE. A CSV is
compared only when its run record binds it to the current MBF: the host MBF hash
and the hash the guest reported for the MBF it solved both equal the current
MBF's hash.

After the selected cases, every hypothesis series in
validation/caepipe_correlation/series/ is evaluated from its members' stored
comparison records and written privately to series/<hypothesis_id>.json in the
exchange folder; every case whose declarations name an evaluated hypothesis and
whose stored record carries a different series status is then rebuilt offline
from its stored CSV, so its causes use the series status (Design §6, §7).

--export-mbf adds the export step after each case's live round trip: the guest
opens the case's solved model in a CAEPIPE instance it starts, writes it back to
MBF with File > Export to MBF... and stops that instance; the harness collects
caepipe_out/exported.mbf and binds it to the run record by hash (Design §3, §5).
--export-only runs only that step, from each case's stored solve, which its run
record must bind to the current MBF, and needs no runner. In the exchange folder it
writes the case's caepipe_out/ files (the export, its guest logs and the run
record's export entry), the harness copy of the guest script and run lock in
_harness/, and creates the case's layer folders if they are missing.

--authored runs the authored cases of validation/caepipe_correlation/authored/
instead: invented, hand-written MBF files listed in its manifest, each exercising
one importer mapping or one named refusal (DEL-17-04 Design §18.3). Per case the
live round trip imports (and solves) the authored MBF as written, and the export
step then writes the model back to caepipe_out/exported.mbf; with --export-only
only the export step runs. No SWBPIPE solve, CSV comparison or series step runs,
and no runner is needed.

Standard output carries operational status only: per case its id, route and
either the comparison record's path relative to the exchange folder or the
reason no record was written, per export step its route and either the exported
MBF's path or the reason none was written, and per series the path of its
record. Outcome,
cause and series status stay in the private records; no external-prover value or
derived statistic is printed. Exit status 0 when every selected case and every
rebuild produced a comparison record, or when the harness is not configured
(the reason is printed and nothing is written); 1 otherwise, including any export
step that wrote no exported MBF; 2 for a usage error.
"""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
from typing import Any, Callable, Mapping


PROJECT = Path(__file__).resolve().parents[2]
if str(PROJECT) not in sys.path:
    sys.path.insert(0, str(PROJECT))

from core.handoff.caepipe_external import (  # noqa: E402
    HarnessConfig,
    HarnessSkip,
    build_comparison_record,
    canonical_model_hash,
    case_paths,
    read_caepipe_print_csv,
    resolve_harness_config,
    run_caepipe_mbf_export,
    run_caepipe_round_trip,
)
from core.handoff.caepipe_external.correlation import evaluate_hypothesis_series, series_summary  # noqa: E402
from core.handoff.caepipe_external.harness import (  # noqa: E402
    BOUNDARY_NOTE,
    CASE_ID_PATTERN,
    CSV_ROUTES,
    EXPORT_ROUTE,
    EXPORTED_MBF,
    ContainmentError,
    contained,
    contained_dir,
    stored_run_record,
    write_private,
)
from core.handoff.caepipe_mbf import render_caepipe_mbf_si  # noqa: E402


CASES_DIR = PROJECT / "validation" / "caepipe_correlation" / "cases"
AUTHORED_DIR = PROJECT / "validation" / "caepipe_correlation" / "authored"
AUTHORED_MANIFEST_KIND = "openpipestress.caepipe_correlation.authored_manifest"
SERIES_DIR = PROJECT / "validation" / "caepipe_correlation" / "series"
SERIES_DEFINITION_KIND = "openpipestress.caepipe_correlation.series_definition"
HAND_CALCS = PROJECT / "validation" / "caepipe_correlation" / "hand_calcs.py"
RUNNER_ENV = "OPENPIPESTRESS_RUNNER_BIN"
MANIFEST_KIND = "openpipestress.caepipe_correlation.case_manifest"
PARSER_DIAGNOSTICS_KIND = "openpipestress.caepipe_print_csv.parser_diagnostics"
LAYER_PRIVACY = {
    "swbpipe": "SWBPIPE request and result from the invented public case; kept with the private run",
    "caepipe_in": "SWBPIPE-derived MBF and writer maps; no CAEPIPE output",
    "caepipe_out": "PRIVATE: CAEPIPE output (CSV, .res, .mod, exported.mbf) and the guest run logs",
    "parsed": "PRIVATE: derived from CAEPIPE output",
    "comparison": "PRIVATE: derived from CAEPIPE output",
}


def load_cases(selected: list[str] | None = None) -> list[dict[str, Any]]:
    cases = {}
    for path in sorted(CASES_DIR.glob("*.json")):
        case = json.loads(path.read_text(encoding="utf-8"))
        if case.get("case_id") != path.stem or not CASE_ID_PATTERN.match(path.stem):
            raise ValueError(f"{path.name}: case_id must equal the file stem and match {CASE_ID_PATTERN.pattern}")
        cases[path.stem] = case
    if selected:
        unknown = sorted(set(selected) - set(cases))
        if unknown:
            raise KeyError(f"unknown case id(s): {', '.join(unknown)}")
        return [cases[case_id] for case_id in sorted(set(selected))]
    return [cases[case_id] for case_id in sorted(cases)]


def load_authored_cases(selected: list[str] | None = None) -> list[dict[str, Any]]:
    """The authored cases of the manifest, checked: each id matches the pattern, is unique and names its MBF."""
    manifest = json.loads((AUTHORED_DIR / "manifest.json").read_text(encoding="utf-8"))
    if manifest.get("document_kind") != AUTHORED_MANIFEST_KIND:
        raise ValueError(f"authored/manifest.json: document_kind must be {AUTHORED_MANIFEST_KIND}")
    cases: dict[str, dict[str, Any]] = {}
    for entry in manifest.get("cases") or []:
        case_id = entry.get("case_id")
        if not isinstance(case_id, str) or not CASE_ID_PATTERN.match(case_id) or case_id in cases:
            raise ValueError(f"authored/manifest.json: case id {case_id!r} is invalid or repeated")
        if entry.get("file") != f"{case_id}.mbf" or not (AUTHORED_DIR / entry["file"]).is_file():
            raise ValueError(f"authored/manifest.json: {case_id} must name its file {case_id}.mbf")
        cases[case_id] = entry
    if selected:
        unknown = sorted(set(selected) - set(cases))
        if unknown:
            raise KeyError(f"unknown authored case id(s): {', '.join(unknown)}")
        return [cases[case_id] for case_id in sorted(set(selected))]
    return [cases[case_id] for case_id in sorted(cases)]


def authored_mbf_bytes(case: Mapping[str, Any]) -> bytes:
    """The authored MBF exactly as tracked (CRLF kept)."""
    return (AUTHORED_DIR / case["file"]).read_bytes()


def load_series() -> list[dict[str, Any]]:
    """The public hypothesis series definitions, checked against the case set."""
    known = {case["case_id"] for case in load_cases()}
    series = []
    for path in sorted(SERIES_DIR.glob("*.json")):
        definition = json.loads(path.read_text(encoding="utf-8"))
        if definition.get("document_kind") != SERIES_DEFINITION_KIND:
            raise ValueError(f"{path.name}: document_kind must be {SERIES_DEFINITION_KIND}")
        if definition.get("hypothesis_id") != path.stem or not CASE_ID_PATTERN.match(path.stem):
            raise ValueError(f"{path.name}: hypothesis_id must equal the file stem and match {CASE_ID_PATTERN.pattern}")
        unknown = sorted({member["case_id"] for member in definition.get("members") or []} - known)
        if unknown:
            raise ValueError(f"{path.name}: unknown member case id(s): {', '.join(unknown)}")
        series.append(definition)
    return series


def load_hand_calcs() -> Any:
    spec = importlib.util.spec_from_file_location("caepipe_correlation_hand_calcs", HAND_CALCS)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


def runner_request(case_id: str) -> dict[str, Any]:
    """The headless runner request wrapper (shape of tests/test_headless_runner_contract.py final_runner_request)."""
    return {
        "request_id": f"caepipe-pilot-{case_id}",
        "operation": "solve",
        "operation_ref": {"ref_type": "api_operation", "ref_id": "solve"},
        "project_ref": {"ref_type": "project", "ref_id": case_id},
        "model_ref": {"ref_type": "model", "ref_id": case_id},
        "unit_system_ref": {"ref_type": "unit_system", "ref_id": "invented-si"},
        "load_basis_refs": [{"ref_type": "load_case", "ref_id": "LC1"}],
        "input_manifest_ref": {"ref_type": "audit_manifest", "ref_id": f"caepipe-pilot-{case_id}"},
        "requested_outputs": ["result_envelope", "audit_manifest", "diagnostics"],
        "privacy": {
            "local_only": True,
            "telemetry_allowed": False,
            "private_payload_redacted": True,
            "classification": "public_metadata",
        },
        "provenance": {
            "source_name": "invented CAEPIPE correlation pilot case",
            "source_location": f"validation/caepipe_correlation/cases/{case_id}.json",
            "source_license": "project invented",
            "contributor": "OpenPipeStress",
            "contributor_certification": "invented non-engineering example",
            "redistribution_status": "invented_non_engineering_example",
            "review_status": "pending",
        },
        "professional_boundary": {
            "human_review_required": True,
            "software_makes_compliance_claim": False,
            "software_makes_certification_claim": False,
            "software_makes_sealing_claim": False,
            "software_makes_approval_claim": False,
            "software_makes_authentication_claim": False,
        },
        "tbd_decisions": {
            "final_cli_command_syntax": "SETTLED_DEC_065",
            "package_scripts": "SETTLED_DEC_065",
            "process_invocation": "SETTLED_DEC_065",
            "network_access": "SETTLED_DEC_065",
            "filesystem_mutation_policy": "SETTLED_DEC_065",
            "ci_provider": "TBD",
            "release_matrix": "TBD",
            "public_transport_protocol": "TBD",
            "external_adapter_formats": "TBD",
            "physical_project_container": "TBD",
        },
    }


def solve_swbpipe(runner: str, case: Mapping[str, Any]) -> tuple[dict[str, Any], dict[str, Any] | None, str]:
    """Run `<runner> solve --explicit-local-private-intent --input -`; return (request body, output, error)."""
    body = {"request": runner_request(case["case_id"]),
            "solve": {"preview_model": {"model": case["model"], "materials": []}}}
    completed = subprocess.run([runner, "solve", "--explicit-local-private-intent", "--input", "-"],
                               input=json.dumps(body), capture_output=True, text=True, check=False)
    try:
        output = json.loads(completed.stdout)
    except json.JSONDecodeError:
        return body, None, f"runner exit {completed.returncode}; stdout is not JSON"
    mechanics = (((output.get("payload") or {}).get("mechanics_envelope") or {}).get("status") or {}).get("mechanics")
    if mechanics != "MECHANICS_SOLVED":
        return body, output, f"SWBPIPE mechanics status {mechanics!r} (runner exit {completed.returncode})"
    return body, output, ""


def apply_fault_injection(mbf_text: str, fault: Mapping[str, Any]) -> str:
    """Multiply the elastic modulus of every MATERIAL temperature row by the case's factor (deliberate fault)."""
    if fault.get("target") != "mbf" or "material_modulus_factor" not in fault:
        raise ValueError(f"unsupported fault injection {dict(fault)!r}")
    factor = float(fault["material_modulus_factor"])
    lines = mbf_text.split("\r\n")
    inside, changed = False, 0
    for index, line in enumerate(lines):
        if line == "MATERIAL":
            inside = True
        elif line.isupper() and line.isalpha():
            inside = False
        elif inside:
            fields = line.split(",")
            if len(fields) == 4:  # name, temperature, E, expansion coefficient
                fields[2] = format(float(fields[2]) * factor, ".10g")
                lines[index] = ",".join(fields)
                changed += 1
    if not changed:
        raise ValueError("fault injection found no MATERIAL temperature rows")
    return "\r\n".join(lines)


def _sha256(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def _write_json(root: Path, path: Path, value: Any) -> None:
    write_private(root, path, json.dumps(value, indent=2, sort_keys=True) + "\n")


def _now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def bound_csv(root: Path, paths: Mapping[str, Path], record: Mapping[str, Any] | None,
              mbf_sha256: str) -> tuple[Path | None, str, str]:
    """(CSV, route, reason): the CSV only when the run record binds it to the current MBF, else None and why."""
    if record is None:
        return None, "unbound_output", "no valid caepipe_out/run_record.json binds a CSV to this MBF"
    if (record.get("mbf") or {}).get("sha256") != mbf_sha256:
        return None, "stale", "the stored CAEPIPE output was produced from a different MBF"
    route = str(record.get("route") or "unknown_route")
    if record.get("guest_mbf_sha256") != mbf_sha256:
        return None, (route if route == "input_hash_mismatch" else "unbound_output"), \
            "the guest did not report solving this MBF (guest MBF hash absent or different)"
    host = (record.get("observed_csv_path") or {}).get("host")
    if route not in CSV_ROUTES or not isinstance(host, str) or not host.startswith("caepipe_out/"):
        return None, route, "the run record names no collected CAEPIPE CSV"
    csv_path = contained(root, paths["root"] / host)
    if not csv_path.is_file():
        return None, route, "the CAEPIPE CSV named by the run record is missing"
    return csv_path, route, ""


def render_case_mbf(case: Mapping[str, Any]) -> tuple[Any, str, str]:
    """(writer result, writer text, exported MBF text with the case's fault injection applied)."""
    mbf = render_caepipe_mbf_si(case["model"], title=case.get("title", case["case_id"]),
                                reference_temperature_c=case.get("reference_temperature_c", 20.0))
    if mbf.status != "written":
        return mbf, "", ""
    fault = case.get("fault_injection")
    return mbf, mbf.text, apply_fault_injection(mbf.text, fault) if fault else mbf.text


def run_case(case: Mapping[str, Any], config: HarnessConfig, *, runner: str, offline: bool, hand_calcs: Any,
             emit: Callable[[str], None] = print,
             round_trip: Callable[..., dict[str, Any]] = run_caepipe_round_trip,
             hypothesis_series: Mapping[str, Mapping[str, Any]] | None = None) -> bool:
    try:
        return _run_case(case, config, runner=runner, offline=offline, hand_calcs=hand_calcs, emit=emit,
                         round_trip=round_trip, hypothesis_series=hypothesis_series or {})
    except ContainmentError as error:
        emit(f"{case['case_id']}: route=containment_violation; {error}; no comparison record")
        return False


def _run_case(case: Mapping[str, Any], config: HarnessConfig, *, runner: str, offline: bool, hand_calcs: Any,
              emit: Callable[[str], None], round_trip: Callable[..., dict[str, Any]],
              hypothesis_series: Mapping[str, Mapping[str, Any]]) -> bool:
    case_id = case["case_id"]
    root = config.exchange_root.resolve()
    paths = case_paths(config, case_id)
    analytic = hand_calcs.analytic_values(case_id)
    alternative = hand_calcs.alternative_analytic_values(case_id)

    body, output, error = solve_swbpipe(runner, case)
    _write_json(root, paths["swbpipe"] / "request.json", body)
    if output is not None:
        _write_json(root, paths["swbpipe"] / "result.json", output)
    if error:
        emit(f"{case_id}: route=not_run; SWBPIPE not solved ({error})")
        return False
    payload = output["payload"]

    mbf, writer_text, mbf_text = render_case_mbf(case)
    if mbf.status != "written":
        codes = sorted({item["code"] for item in mbf.diagnostics if item["severity"] == "error"})
        emit(f"{case_id}: route=not_run; MBF refused ({', '.join(codes)})")
        return False
    fault = case.get("fault_injection")

    mbf_sha256 = _sha256(mbf_text)
    if offline:
        # Checked before caepipe_in/ is rewritten, so the MBF that produced a stale output is kept.
        csv_path, route, reason = bound_csv(root, paths, stored_run_record(root, paths["caepipe_out"]), mbf_sha256)
        if csv_path is None:
            emit(f"{case_id}: route={route}; {reason}; no comparison record")
            return False
    in_dir = paths["caepipe_in"]
    write_private(root, in_dir / "model.mbf", mbf_text.encode("utf-8"))
    _write_json(root, in_dir / "id_map.json", mbf.sidecar(mbf_sha256))
    _write_json(root, in_dir / "mbf_render.json", {
        "profile_id": mbf.to_dict()["profile_id"], "status": mbf.status, "layout_row_count": mbf.layout_row_count,
        "diagnostics": mbf.diagnostics, "loss_report": mbf.loss_report, "assumptions": mbf.assumptions,
        "writer_text_sha256": _sha256(writer_text), "mbf_sha256": mbf_sha256,
        "fault_injection": dict(fault) if fault else None,
    })

    if not offline:
        csv_path, route, reason = bound_csv(root, paths, round_trip(config, case_id, mbf_text), mbf_sha256)
        if csv_path is None:
            emit(f"{case_id}: route={route}; {reason}; no comparison record")
            return False

    parsed = read_caepipe_print_csv(csv_path)
    _write_json(root, paths["parsed"] / "caepipe_results.json", parsed)
    _write_json(root, paths["parsed"] / "parser_diagnostics.json", {
        "document_kind": PARSER_DIAGNOSTICS_KIND,
        **{key: parsed[key] for key in ("parser_version", "diagnostics", "sections_parsed", "sections_skipped")}})
    record = build_comparison_record(case, swbpipe_model=case["model"], swbpipe_result_payload=payload,
                                     mbf_result=mbf, mbf_text=mbf_text, parsed_caepipe=parsed, analytic=analytic,
                                     alternative_analytic=alternative, hypothesis_series=hypothesis_series)
    comparison_path = paths["comparison"] / "comparison.json"
    _write_json(root, comparison_path, record)
    _write_json(root, paths["manifest"], {
        "document_kind": MANIFEST_KIND,
        "case_id": case_id,
        "created": _now(),
        "mode": "offline" if offline else "live",
        "internal_model_hash": canonical_model_hash(case["model"]),
        "writer_text_sha256": _sha256(writer_text),
        "exported_model_hash": _sha256(mbf_text),
        "target_tool": record["target_tool"],
        "target_version": record["target_version"],
        "product_label": record["product_label"],
        "trial_first_launch_date": config.trial_first_launch_date,
        "route": route,
        "csv": str(csv_path.relative_to(paths["root"])),
        "hypothesis_series_used": sorted(record["hypothesis_series"]),
        "layer_privacy": LAYER_PRIVACY,
        "boundary_note": BOUNDARY_NOTE,
    })
    # Operational status only: the counts by outcome and cause stay in the private record's summary (Design §8).
    emit(f"{case_id}: route={route}; comparison record written: {comparison_path.relative_to(root)}")
    return True


def _stored_comparison(root: Path, case_id: str) -> dict[str, Any] | None:
    path = contained(root, root / "cases" / case_id / "comparison" / "comparison.json")
    try:
        record = json.loads(path.read_text(encoding="utf-8")) if path.is_file() else None
    except (OSError, UnicodeDecodeError, json.JSONDecodeError):
        return None
    return record if isinstance(record, dict) else None


def run_series(config: HarnessConfig, *, runner: str, hand_calcs: Any, emit: Callable[[str], None] = print,
               series_definitions: list[Mapping[str, Any]] | None = None,
               cases: list[Mapping[str, Any]] | None = None) -> dict[str, bool]:
    """Evaluate every hypothesis series and rebuild the records whose series status changed.

    Returns {case id: rebuilt} for each case rebuilt offline. The series records are private (Design §8).
    """
    root = config.exchange_root.resolve()
    cases = list(cases if cases is not None else load_cases())
    definitions = list(series_definitions if series_definitions is not None else load_series())
    by_id = {case["case_id"]: case for case in cases}
    statuses: dict[str, dict[str, Any]] = {}
    try:
        for definition in definitions:
            hypothesis = definition["hypothesis_id"]
            member_ids = [member["case_id"] for member in definition.get("members") or []]
            current = {}
            for case_id in member_ids:
                if case_id in by_id:
                    mbf, _, mbf_text = render_case_mbf(by_id[case_id])
                    if mbf.status == "written":
                        current[case_id] = _sha256(mbf_text)
            records = {case_id: _stored_comparison(root, case_id) for case_id in member_ids}
            result = evaluate_hypothesis_series(definition, records, current_mbf_sha256=current)
            relative = Path("series") / f"{hypothesis}.json"
            result.update(definition=f"validation/caepipe_correlation/series/{hypothesis}.json", created=_now(),
                          record=str(relative), boundary_note=BOUNDARY_NOTE)
            contained_dir(root, root / "series")
            _write_json(root, root / relative, result)
            statuses[hypothesis] = result
            emit(f"series {hypothesis}: series record written: {relative}")
    except ContainmentError as error:
        emit(f"series: containment_violation; {error}; no series record")
        return {"series": False}

    rebuilt: dict[str, bool] = {}
    for case in cases:
        named = sorted({str(item.get("hypothesis")) for item in case.get("declared_differences") or []
                        if item.get("hypothesis") in statuses})
        if not named:
            continue
        stored = _stored_comparison(root, case["case_id"])
        if stored is None:
            continue
        wanted = {hypothesis: series_summary(statuses[hypothesis]) for hypothesis in named}
        if all((stored.get("hypothesis_series") or {}).get(hypothesis) == wanted[hypothesis] for hypothesis in named):
            continue
        rebuilt[case["case_id"]] = run_case(case, config, runner=runner, offline=True, hand_calcs=hand_calcs,
                                            emit=lambda line: emit(f"rebuild for series: {line}"),
                                            hypothesis_series=statuses)
    return rebuilt


def export_case(case: Mapping[str, Any], config: HarnessConfig, *, emit: Callable[[str], None] = print,
                export: Callable[..., dict[str, Any]] = run_caepipe_mbf_export) -> bool:
    """Export the case's stored solved model to caepipe_out/exported.mbf when its run record binds it to the MBF."""
    case_id = case["case_id"]
    mbf, _, mbf_text = render_case_mbf(case)
    if mbf.status != "written":
        emit(f"{case_id}: export route=not_run; MBF refused; no exported MBF")
        return False
    try:
        entry = export(config, case_id, _sha256(mbf_text))
    except ContainmentError as error:
        emit(f"{case_id}: export route=containment_violation; {error}; no exported MBF")
        return False
    route = entry["route"]
    if route == EXPORT_ROUTE:
        emit(f"{case_id}: export route={route}; exported MBF written: cases/{case_id}/caepipe_out/{EXPORTED_MBF}")
        return True
    reason = entry.get("reason") or f"the guest export did not complete (cases/{case_id}/caepipe_out/run_record.json)"
    emit(f"{case_id}: export route={route}; {reason}; no exported MBF")
    return False


def run_authored_case(case: Mapping[str, Any], config: HarnessConfig, *, export_only: bool = False,
                      emit: Callable[[str], None] = print,
                      round_trip: Callable[..., dict[str, Any]] = run_caepipe_round_trip,
                      export: Callable[..., dict[str, Any]] = run_caepipe_mbf_export) -> bool:
    """Import (and solve) one authored MBF as written, then export the model back to caepipe_out/exported.mbf."""
    case_id = case["case_id"]
    data = authored_mbf_bytes(case)
    mbf_sha256 = hashlib.sha256(data).hexdigest()
    try:
        if not export_only:
            record = round_trip(config, case_id, data.decode("ascii"))
            if record.get("mbf", {}).get("sha256") != mbf_sha256 or record.get("guest_mbf_sha256") != mbf_sha256:
                emit(f"{case_id}: route={record.get('route')}; the guest did not report importing this MBF; "
                     "no exported MBF")
                return False
            emit(f"{case_id}: route={record.get('route')}; round trip recorded: "
                 f"cases/{case_id}/caepipe_out/run_record.json")
        entry = export(config, case_id, mbf_sha256)
    except ContainmentError as error:
        emit(f"{case_id}: route=containment_violation; {error}; no exported MBF")
        return False
    route = entry["route"]
    if route == EXPORT_ROUTE:
        emit(f"{case_id}: export route={route}; exported MBF written: cases/{case_id}/caepipe_out/{EXPORTED_MBF}")
        return True
    reason = entry.get("reason") or f"the guest export did not complete (cases/{case_id}/caepipe_out/run_record.json)"
    emit(f"{case_id}: export route={route}; {reason}; no exported MBF")
    return False


def run_cases(cases: list[Mapping[str, Any]], config: HarnessConfig, *, runner: str, offline: bool, hand_calcs: Any,
              emit: Callable[[str], None] = print,
              round_trip: Callable[..., dict[str, Any]] = run_caepipe_round_trip, export_mbf: bool = False,
              export: Callable[..., dict[str, Any]] = run_caepipe_mbf_export) -> dict[str, bool]:
    """Run the selected cases (each followed by its export step with `export_mbf`), then the series step.

    Returns {case id, "<case id> export" or "series": success}.
    """
    results = {}
    for case in cases:
        results[case["case_id"]] = run_case(case, config, runner=runner, offline=offline, hand_calcs=hand_calcs,
                                            emit=emit, round_trip=round_trip)
        if export_mbf:
            results[f"{case['case_id']} export"] = export_case(case, config, emit=emit, export=export)
    for key, ok in run_series(config, runner=runner, hand_calcs=hand_calcs, emit=emit).items():
        results[key] = results.get(key, True) and ok
    return results


def main(argv: list[str] | None = None, environ: Mapping[str, str] = os.environ) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--case", action="append", dest="cases", metavar="ID", help="run only this case (repeatable)")
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--offline", action="store_true", help="reuse the CAEPIPE CSV stored in caepipe_out/")
    mode.add_argument("--export-mbf", action="store_true",
                      help="after each case's round trip, export the solved model to caepipe_out/exported.mbf")
    mode.add_argument("--export-only", action="store_true",
                      help="only export each case's stored, bound solved model to caepipe_out/exported.mbf")
    parser.add_argument("--authored", action="store_true",
                        help="run the authored MBF cases of validation/caepipe_correlation/authored/ (with --export-only "
                             "only their export step)")
    parser.add_argument("--runner", help=f"openpipestress-runner executable (default: ${RUNNER_ENV})")
    args = parser.parse_args(argv)
    if args.authored and (args.offline or args.export_mbf):
        parser.error("--authored takes only --case and --export-only")
    try:
        cases = load_authored_cases(args.cases) if args.authored else load_cases(args.cases)
        load_series()
    except (KeyError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 2
    config = resolve_harness_config(environ, require_vm_tools=not args.offline)
    if isinstance(config, HarnessSkip):
        print(f"CAEPIPE harness skipped: {config.reason}")
        return 0
    if args.authored:
        for warning in config.warnings:
            print(f"warning: {warning}")
        return 0 if all([run_authored_case(case, config, export_only=args.export_only) for case in cases]) else 1
    if args.export_only:
        for warning in config.warnings:
            print(f"warning: {warning}")
        return 0 if all([export_case(case, config) for case in cases]) else 1
    runner = args.runner or environ.get(RUNNER_ENV)
    if not runner or not Path(runner).is_file():
        print(f"error: no runner executable (--runner or ${RUNNER_ENV})", file=sys.stderr)
        return 2
    for warning in config.warnings:
        print(f"warning: {warning}")
    hand_calcs = load_hand_calcs()
    results = run_cases(cases, config, runner=runner, offline=args.offline, hand_calcs=hand_calcs,
                        export_mbf=args.export_mbf)
    return 0 if all(results.values()) else 1


if __name__ == "__main__":
    raise SystemExit(main())
