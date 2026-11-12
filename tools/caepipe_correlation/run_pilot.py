"""Run the CAEPIPE correlation pilot (DEL-17-05 Design §3-§7) for every case, one case at a time.

From the project root:

    python3 -m tools.caepipe_correlation.run_pilot [--case ID ...] [--offline | --export-mbf | --export-only]
        [--runner PATH]
    python3 -m tools.caepipe_correlation.run_pilot --authored [--case ID ...] [--export-only | --no-export]
    python3 -m tools.caepipe_correlation.run_pilot --authored --authored-dir PATH [--case ID ...]
    python3 -m tools.caepipe_correlation.run_pilot --benchmark MANIFEST [--case ID ...] [--batch N]
        [--batch-size N] [--runner PATH] (--offline | --freeze-file PATH --exclusive-file PATH [--pace-s S]
        [--liveness-wait-s S])

Per case: the independent hand calculation, the SWBPIPE solve (headless runner),
the SI MBF (with the case's deliberate fault injection, if any), the live
CAEPIPE round trip through the configured virtual machine (skipped with
--offline, which screens the CSV of the case's current run), the parsed
CAEPIPE results, the three-way comparison record and the case manifest, all
under the private exchange folder named by SWBPIPE_CAEPIPE_EXCHANGE. A CSV is
compared only when its run record binds it to the current MBF: the host MBF hash
and the hash the guest reported for the MBF it solved both equal the current
MBF's hash.

Nothing stored is changed (the harness module states the rule): a live step writes
a new run folder `cases/<id>/runs/<NNNNNN>/` and a screen a new folder
`cases/<id>/screens/<NNNNNN>/`, every file created exclusively; each becomes the
case's current one only once it is complete, through a new selection
`cases/<id>/selections/<NNNNNN>.json`, which states its source (a run folder, or
`already_stored`: the results already in the case's own folders). A case with no
selection is read where its results already are; a case whose selections cannot be
trusted, or whose source is sealed, is refused by name (route=selection_untrusted)
and never read elsewhere. A live step is refused under its own name while another
step holds the run lock, while a guest script of an ended run or an instance of the
program it did not start runs in the guest, and while the newest holder's run
recorded the saved settings' before-state without their comparison, until
`python3 -m tools.caepipe_correlation.clear_settings_stop` records a clearance.
The screen modes (--offline, the benchmark's --offline and the series step) need no
guest, no prlctl and no guest configuration.

After the selected cases, every hypothesis series in
validation/caepipe_correlation/series/ is evaluated from its members' stored
comparison records and written privately as the next new record
series/<hypothesis_id>/<NNNNNN>.json in the exchange folder; every case whose declarations name an evaluated hypothesis and
whose stored record carries a different series status is then rebuilt offline
from its stored CSV, so its causes use the series status (Design §6, §7).

--export-mbf adds the export step after each case's live round trip: the guest
opens the case's solved model in a CAEPIPE instance it starts, writes it back to
MBF with File > Export to MBF... and stops that instance; the harness collects
exported.mbf into the export's own new run folder and binds it there by hash (Design
§3, §5). --export-only runs only that step, from each case's current solve, which its
run record must bind to the current MBF, and needs no runner. In the exchange folder it
writes only the export's new run folder (the export, its guest logs and its run
record, which names the solve it read), the content-named copy of the guest script in
_harness/ when absent, and a new selection once the export is checked complete; it
takes and removes only its own run lock.

--authored runs the authored cases of validation/caepipe_correlation/authored/
instead: invented, hand-written MBF files listed in its manifest, each exercising
one importer mapping or one named refusal (DEL-17-04 Design §18.3). Per case the
live round trip imports (and solves) the authored MBF as written, and the export
step then writes the model back to caepipe_out/exported.mbf; with --export-only
only the export step runs. No SWBPIPE solve, CSV comparison or series step runs,
and no runner is needed. --authored-dir takes the cases from another directory
with the same manifest layout instead; it must lie inside the exchange folder,
with no symbolic link on the way, so private authored files stay private, and its
case ids must not name a tracked writer or authored case, whose stored results the
run would otherwise be mixed with. --no-export leaves out the export step; --authored-dir
runs never export (only the solve and its printed results are wanted there), and
need the configuration's settings_guard. Before the first case and before each round
trip no instance of the tool may be running that the run did not start. Each round
trip reads the tool's whole saved-settings key before and after and confirms that
the processes it started have ended; when the configuration enables it, a
difference confined to the recent-files subkey (or, with restore_scope "key", any
difference strictly inside the key) is written back and verified, and nothing else
is ever written. The run stops at the first case whose check is not clean, whose
round trip raised, or whose batch run left no CSV. These runs fall back to the GUI
print only with restore_scope "key", whose restore covers the settings that route
writes (Design §5).

--benchmark runs the generated benchmark models of a manifest written by
tools/caepipe_correlation/benchmark_models.py (DEL-17-05 Design §11, RT-V2): per
model the SWBPIPE solve, the MBF written with the model's explicit piping code,
the guarded batch round trip, the parsed CSV and the two-way comparison record
(comparison/benchmark_comparison.json), with the case manifest as the model's
run record, all in the exchange folder. Models run one at a time in serial
batches of --batch-size (default 4); --batch N runs only the N-th batch. A live
benchmark run needs the configuration's settings_guard, --freeze-file and
--exclusive-file. It paces the models --pace-s apart (default 60 s). Before each
model it stops if the freeze file exists, holds while the exclusive file names a
running process (up to 30 min), checks that the guest answers `cmd /c ver`
(once more after --liveness-wait-s, default 2 min) and that no instance of the tool is running. A model
whose round trip raised or left no CSV, with a clean settings check, is retried
once 2 min later. The run stops at the first unclean settings check, a failed
check before a model, or a round trip that raised twice; the GUI print fallback
is used only with restore_scope "key" (Design §5). --offline rebuilds the records
from the stored, bound CSVs.

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
from dataclasses import dataclass
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import time
from typing import Any, Callable, Mapping


PROJECT = Path(__file__).resolve().parents[2]
if str(PROJECT) not in sys.path:
    sys.path.insert(0, str(PROJECT))

from core.handoff.caepipe_external import (  # noqa: E402
    HarnessConfig,
    build_benchmark_record,
    HarnessSkip,
    build_comparison_record,
    canonical_model_hash,
    read_caepipe_print_csv,
    resolve_harness_config,
    run_caepipe_mbf_export,
    run_caepipe_round_trip,
)
from core.handoff.caepipe_external.correlation import evaluate_hypothesis_series, series_summary  # noqa: E402
from core.handoff.caepipe_external.harness import (  # noqa: E402
    CONFIG_NAME,
    check_guest_alive,
    check_no_foreign_instance,
    BOUNDARY_NOTE,
    SETTINGS_KEEP_GOING,
    CASE_ID_PATTERN,
    CSV_ROUTES,
    EXPORT_ROUTE,
    EXPORTED_MBF,
    REFUSED_ROUTES,
    RUNS_DIR,
    SCREEN_RECORD_KIND,
    SCREENS_DIR,
    CaseFolder,
    ContainmentError,
    ExistingPath,
    SelectionError,
    contained,
    current_case,
    exchange_tree,
    open_folder,
    outputs_sealed,
    publish,
    reserve_folder,
    solve_ineligible,
    stored_run_record,
)
from core.handoff.caepipe_mbf import render_caepipe_mbf_si  # noqa: E402


CASES_DIR = PROJECT / "validation" / "caepipe_correlation" / "cases"
AUTHORED_DIR = PROJECT / "validation" / "caepipe_correlation" / "authored"
AUTHORED_MANIFEST_KIND = "openpipestress.caepipe_correlation.authored_manifest"
SERIES_DIR = PROJECT / "validation" / "caepipe_correlation" / "series"
SERIES_DEFINITION_KIND = "openpipestress.caepipe_correlation.series_definition"
HAND_CALCS = PROJECT / "validation" / "caepipe_correlation" / "hand_calcs.py"
RUNNER_ENV = "OPENPIPESTRESS_RUNNER_BIN"
BENCHMARK_MANIFEST_KIND = "openpipestress.caepipe_benchmark.manifest"
BENCHMARK_MODEL_KIND = "openpipestress.caepipe_benchmark.model"
BENCHMARK_BATCH_SIZE = 4
# Live benchmark pacing: seconds between models, the wait before the one retry of a failed model or
# liveness check, and the longest hold for a pending exclusive job, polled every BENCHMARK_POLL_S.
BENCHMARK_PACE_S = 60.0
BENCHMARK_RETRY_WAIT_S = 120.0
BENCHMARK_EXCLUSIVE_WAIT_S = 1800.0
BENCHMARK_POLL_S = 30.0
BENCHMARK_RECORD = "benchmark_comparison.json"
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


def load_authored_cases(selected: list[str] | None = None, directory: Path = AUTHORED_DIR) -> list[dict[str, Any]]:
    """The authored cases of the manifest, checked: each id matches the pattern, is unique and names its MBF."""
    label = "authored/manifest.json" if directory == AUTHORED_DIR else "manifest.json"
    manifest = json.loads((directory / "manifest.json").read_text(encoding="utf-8"))
    if manifest.get("document_kind") != AUTHORED_MANIFEST_KIND:
        raise ValueError(f"{label}: document_kind must be {AUTHORED_MANIFEST_KIND}")
    cases: dict[str, dict[str, Any]] = {}
    for entry in manifest.get("cases") or []:
        case_id = entry.get("case_id")
        if not isinstance(case_id, str) or not CASE_ID_PATTERN.match(case_id) or case_id in cases:
            raise ValueError(f"{label}: case id {case_id!r} is invalid or repeated")
        if entry.get("file") != f"{case_id}.mbf" or not (directory / entry["file"]).is_file():
            raise ValueError(f"{label}: {case_id} must name its file {case_id}.mbf")
        cases[case_id] = entry
    if selected:
        unknown = sorted(set(selected) - set(cases))
        if unknown:
            raise KeyError(f"unknown authored case id(s): {', '.join(unknown)}")
        return [cases[case_id] for case_id in sorted(set(selected))]
    return [cases[case_id] for case_id in sorted(cases)]


def authored_mbf_bytes(case: Mapping[str, Any], directory: Path = AUTHORED_DIR) -> bytes:
    """The authored MBF exactly as stored (CRLF kept)."""
    return (directory / case["file"]).read_bytes()


def external_authored_dir(root: Path, path: str) -> Path:
    """The --authored-dir directory, checked: inside the exchange folder with no symbolic link on the way.

    Its manifest's case ids must not name a tracked writer or authored case: their case folders would be replaced.
    """
    directory = contained(root, Path(os.path.abspath(path)))
    if not directory.is_dir():
        raise ValueError(f"--authored-dir: {path} is not a directory")
    tracked = {case["case_id"] for case in load_cases()} | {case["case_id"] for case in load_authored_cases()}
    clashes = sorted(tracked & {case["case_id"] for case in load_authored_cases(directory=directory)})
    if clashes:
        raise ValueError(f"--authored-dir: case id(s) {', '.join(clashes)} name tracked cases")
    return directory


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


def runner_request(case_id: str, source_location: str | None = None) -> dict[str, Any]:
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
            "source_location": source_location or f"validation/caepipe_correlation/cases/{case_id}.json",
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
    body = {"request": runner_request(case["case_id"], case.get("source_location")),
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


def _json_bytes(value: Any) -> bytes:
    return (json.dumps(value, indent=2, sort_keys=True) + "\n").encode("utf-8")


def _now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def _file_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def bound_csv(root: Path, paths: Mapping[str, Path], record: Mapping[str, Any] | None,
              mbf_sha256: str) -> tuple[Path | None, str, str]:
    """(CSV, route, reason): the CSV only when the run record binds it to the current MBF, else None and why.

    `paths["root"]` is the folder holding the run's caepipe_out/: a run folder, or the case's own folder for a case
    with no selection (`current_case`). A run record that marks its outputs sealed binds nothing that is read."""
    if record is None:
        return None, "unbound_output", "no valid caepipe_out/run_record.json binds a CSV to this MBF"
    if outputs_sealed(record):
        return None, "sealed", "the run record marks its outputs sealed; they are not read"
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
    except SelectionError as error:
        emit(f"{case['case_id']}: route=selection_untrusted; {error}; no comparison record")
        return False
    except ContainmentError as error:
        emit(f"{case['case_id']}: route=containment_violation; {error}; no comparison record")
        return False


def _screen_read(root: Path, view: Any, csv_path: Path) -> dict[str, Any]:
    """What a screen read, by path and SHA-256: the run's record and the CSV it binds (the run is left as it is)."""
    record_path = contained(root, view.run / "caepipe_out" / "run_record.json")
    return {"run": view.source, "selection": view.selection,
            "files": {"caepipe_out/run_record.json": _file_sha256(record_path),
                      str(csv_path.relative_to(view.run)): _file_sha256(csv_path)}}


def _write_screen_record(folder: CaseFolder, case_id: str, read: Mapping[str, Any], mode: str) -> None:
    folder.record("screen_record.json", {
        "document_kind": SCREEN_RECORD_KIND, "case_id": case_id, "mode": mode, "read": dict(read),
        "created": _now(), "boundary_note": BOUNDARY_NOTE,
        "note": "a screen writes only new files in its own folder; the run it read is unchanged"})


def _run_case(case: Mapping[str, Any], config: HarnessConfig, *, runner: str, offline: bool, hand_calcs: Any,
              emit: Callable[[str], None], round_trip: Callable[..., dict[str, Any]],
              hypothesis_series: Mapping[str, Mapping[str, Any]]) -> bool:
    """One case: live, a new run folder written by the harness and then completed and published here; offline, a
    new screen folder of the case's current run. Nothing stored is written in either."""
    case_id = case["case_id"]
    root = config.exchange_root.resolve()
    # A screen first resolves where the case is read (an untrusted selection or a linked folder stops it here).
    view = current_case(root, case_id) if offline else None
    analytic = hand_calcs.analytic_values(case_id)
    alternative = hand_calcs.alternative_analytic_values(case_id)

    body, output, error = solve_swbpipe(runner, case)
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
    inputs = {
        "swbpipe/request.json": _json_bytes(body),
        "swbpipe/result.json": _json_bytes(output),
        "caepipe_in/id_map.json": _json_bytes(mbf.sidecar(mbf_sha256)),
        "caepipe_in/mbf_render.json": _json_bytes({
            "profile_id": mbf.to_dict()["profile_id"], "status": mbf.status, "layout_row_count": mbf.layout_row_count,
            "diagnostics": mbf.diagnostics, "loss_report": mbf.loss_report, "assumptions": mbf.assumptions,
            "writer_text_sha256": _sha256(writer_text), "mbf_sha256": mbf_sha256,
            "fault_injection": dict(fault) if fault else None}),
    }

    def comparison(folder: CaseFolder, csv_path: Path, route: str, mode: str) -> tuple[dict[str, str], str]:
        parsed = read_caepipe_print_csv(csv_path)
        written = {"parsed/caepipe_results.json": folder.record("parsed/caepipe_results.json", parsed)["sha256"],
                   "parsed/parser_diagnostics.json": folder.record("parsed/parser_diagnostics.json", {
                       "document_kind": PARSER_DIAGNOSTICS_KIND,
                       **{key: parsed[key] for key in ("parser_version", "diagnostics", "sections_parsed",
                                                       "sections_skipped")}})["sha256"]}
        record = build_comparison_record(case, swbpipe_model=case["model"], swbpipe_result_payload=payload,
                                         mbf_result=mbf, mbf_text=mbf_text, parsed_caepipe=parsed, analytic=analytic,
                                         alternative_analytic=alternative, hypothesis_series=hypothesis_series)
        written["comparison/comparison.json"] = folder.record("comparison/comparison.json", record)["sha256"]
        written["case_manifest.json"] = folder.record("case_manifest.json", {
            "document_kind": MANIFEST_KIND,
            "case_id": case_id,
            "created": _now(),
            "mode": mode,
            "folder": folder.relative,
            "internal_model_hash": canonical_model_hash(case["model"]),
            "writer_text_sha256": _sha256(writer_text),
            "exported_model_hash": _sha256(mbf_text),
            "target_tool": record["target_tool"],
            "target_version": record["target_version"],
            "product_label": record["product_label"],
            "trial_first_launch_date": config.trial_first_launch_date,
            "route": route,
            "csv": str(csv_path.relative_to(root / "cases" / case_id)),
            "hypothesis_series_used": sorted(record["hypothesis_series"]),
            "layer_privacy": LAYER_PRIVACY,
            "boundary_note": BOUNDARY_NOTE,
        })["sha256"]
        return written, f"cases/{case_id}/{folder.relative}/comparison/comparison.json"

    if offline:
        # A screen of the case's current run: read only after the binding is checked; everything new goes into a new
        # screen folder, which becomes current once complete.
        csv_path, route, reason = bound_csv(root, {"root": view.run},
                                            stored_run_record(root, view.run / "caepipe_out"), mbf_sha256)
        if csv_path is None:
            emit(f"{case_id}: route={route}; {reason}; no comparison record")
            return False
        read = _screen_read(root, view, csv_path)
        with exchange_tree(config) as tree:
            screen = reserve_folder(tree, case_id, SCREENS_DIR)
            for relative, data in sorted(inputs.items()):
                screen.create(relative, data)
            screen.create("caepipe_in/model.mbf", mbf_text.encode("utf-8"))
            written, shown = comparison(screen, csv_path, route, "offline")
            _write_screen_record(screen, case_id, read, "offline")
            publish(tree, case_id, screen=screen.relative, also=written, expect_run=view.source,
                    reason="an offline screen of the current run")
        emit(f"{case_id}: route={route}; comparison record written: {shown}")
        return True

    record = round_trip(config, case_id, mbf_text, files=inputs)
    if record.get("route") in REFUSED_ROUTES:  # nothing stored was changed
        emit(f"{case_id}: route={record['route']}; {record.get('reason')}; no comparison record")
        return False
    run_root = root / "cases" / case_id / record["run_folder"]
    csv_path, route, reason = bound_csv(root, {"root": run_root}, record, mbf_sha256)
    if csv_path is None:
        emit(f"{case_id}: route={route}; {reason}; no comparison record")
        return False
    with exchange_tree(config) as tree:
        folder = open_folder(tree, case_id, record["run_folder"])
        written, shown = comparison(folder, csv_path, route, "live")
        # Published only now, with every file the run completes checked (`publish`).
        publish(tree, case_id, run=folder.relative, also=written, reason="a live solve with its comparison record")
    # Operational status only: the counts by outcome and cause stay in the private record's summary.
    emit(f"{case_id}: route={route}; comparison record written: {shown}")
    return True


def _stored_comparison(root: Path, case_id: str) -> dict[str, Any] | None:
    """The case's current comparison record: its selected screen's, else its run's (`current_case`); a case with no
    selection is read in its own folder. SelectionError when its selections cannot be trusted."""
    view = current_case(root, case_id)
    path = contained(root, view.comparison_root / "comparison" / "comparison.json")
    try:
        record = json.loads(path.read_text(encoding="utf-8")) if path.is_file() and not path.is_symlink() else None
    except (OSError, UnicodeDecodeError, json.JSONDecodeError):
        return None
    return record if isinstance(record, dict) else None


def _new_series_record(tree: Any, hypothesis: str, value: Mapping[str, Any]) -> str:
    """A series record as the next numbered new file `series/<hypothesis>/<NNNNNN>.json`; earlier ones are kept."""
    tree.ensure_dir(("series", hypothesis))
    for _ in range(1000):
        numbers = [int(name[:6]) for name in tree.listdir(("series", hypothesis))
                   if len(name) == 11 and name.endswith(".json") and name[:6].isdigit()]
        name = f"{max(numbers, default=0) + 1:06d}.json"
        try:
            tree.record(("series", hypothesis, name), {**value, "record": f"series/{hypothesis}/{name}"})
        except ExistingPath:
            continue
        return f"series/{hypothesis}/{name}"
    raise ContainmentError(f"no new series record could be written for {hypothesis}")


def run_series(config: HarnessConfig, *, runner: str, hand_calcs: Any, emit: Callable[[str], None] = print,
               series_definitions: list[Mapping[str, Any]] | None = None,
               cases: list[Mapping[str, Any]] | None = None) -> dict[str, bool]:
    """Evaluate every hypothesis series and rebuild the records whose series status changed.

    Returns {case id: rebuilt} for each case rebuilt offline. Each series record is a new numbered file
    (`series/<hypothesis>/<NNNNNN>.json`), and each rebuild a new screen of the case (`run_case(offline=True)`):
    nothing stored is changed. The series records are private (Design §8).
    """
    root = config.exchange_root.resolve()
    cases = list(cases if cases is not None else load_cases())
    definitions = list(series_definitions if series_definitions is not None else load_series())
    by_id = {case["case_id"]: case for case in cases}
    statuses: dict[str, dict[str, Any]] = {}
    try:
        with exchange_tree(config) as tree:
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
                result.update(definition=f"validation/caepipe_correlation/series/{hypothesis}.json", created=_now(),
                              boundary_note=BOUNDARY_NOTE)
                relative = _new_series_record(tree, hypothesis, result)
                result["record"] = relative
                statuses[hypothesis] = result
                emit(f"series {hypothesis}: series record written: {relative}")
    except SelectionError as error:
        emit(f"series: route=selection_untrusted; {error}; no further series record")
        return {"series": False}
    except ContainmentError as error:
        emit(f"series: containment_violation; {error}; no further series record")
        return {"series": False}

    rebuilt: dict[str, bool] = {}
    for case in cases:
        named = sorted({str(item.get("hypothesis")) for item in case.get("declared_differences") or []
                        if item.get("hypothesis") in statuses})
        if not named:
            continue
        try:
            stored = _stored_comparison(root, case["case_id"])
        except ContainmentError as error:
            emit(f"rebuild for series: {case['case_id']}: route=selection_untrusted; {error}; no comparison record")
            rebuilt[case["case_id"]] = False
            continue
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
    """Export the case's current solved model into a new run folder when its run record binds it to the MBF, and
    publish the export once it is checked complete."""
    case_id = case["case_id"]
    mbf, _, mbf_text = render_case_mbf(case)
    if mbf.status != "written":
        emit(f"{case_id}: export route=not_run; MBF refused; no exported MBF")
        return False
    return _export_and_publish(config, case_id, _sha256(mbf_text), emit=emit, export=export)


def _export_and_publish(config: HarnessConfig, case_id: str, mbf_sha256: str, *, emit: Callable[[str], None],
                        export: Callable[..., dict[str, Any]]) -> bool:
    try:
        entry = export(config, case_id, mbf_sha256)
        route = entry["route"]
        if route == EXPORT_ROUTE:
            with exchange_tree(config) as tree:
                publish(tree, case_id, export=entry["run_folder"], expect_run=(entry.get("solve_run") or {}).get("source"),
                        reason="an export of the current run")
    except ContainmentError as error:
        emit(f"{case_id}: export route=containment_violation; {error}; no exported MBF")
        return False
    if route == EXPORT_ROUTE:
        emit(f"{case_id}: export route={route}; exported MBF written: "
             f"cases/{case_id}/{entry['run_folder']}/caepipe_out/{EXPORTED_MBF}")
        return True
    where = f"cases/{case_id}/{entry['run_folder']}/caepipe_out/run_record.json" if entry.get("run_folder") else \
        f"cases/{case_id}"
    reason = entry.get("reason") or f"the guest export did not complete ({where})"
    emit(f"{case_id}: export route={route}; {reason}; no exported MBF")
    return False


class RunStopped(RuntimeError):
    """A guarded round trip ended in a state the run must not continue from; no further case runs."""


def _guard_stop(config: HarnessConfig, case_id: str, settings: Mapping[str, Any] | None,
                emit: Callable[[str], None]) -> None:
    """Raise RunStopped unless the round trip's saved-settings check (configuration `settings_guard`) is clean."""
    if config.settings_guard is None:
        return
    status = (settings or {}).get("status")
    if status not in SETTINGS_KEEP_GOING:
        emit(f"{case_id}: settings check {status or 'missing'}; "
             f"{(settings or {}).get('reason', 'no settings check recorded')}; stopped")
        raise RunStopped(case_id)


def run_authored_case(case: Mapping[str, Any], config: HarnessConfig, *, export_only: bool = False,
                      emit: Callable[[str], None] = print,
                      round_trip: Callable[..., dict[str, Any]] = run_caepipe_round_trip,
                      export: Callable[..., dict[str, Any]] = run_caepipe_mbf_export,
                      directory: Path = AUTHORED_DIR, no_export: bool = False, gui_fallback: bool = True) -> bool:
    """Import (and solve) one authored MBF as written, then export the model back to caepipe_out/exported.mbf.

    With `no_export` the round trip alone runs. With `settings_guard` configured, the saved-settings check is
    enforced before anything else is looked at: any status but `unchanged`, an exception during the round trip,
    or (with `gui_fallback=False`) a batch run that left no CSV raises RunStopped, so no further case runs.
    """
    case_id = case["case_id"]
    data = authored_mbf_bytes(case, directory)
    mbf_sha256 = hashlib.sha256(data).hexdigest()
    guarded = config.settings_guard is not None
    try:
        if not export_only:
            try:
                record = (round_trip(config, case_id, data.decode("ascii")) if gui_fallback else
                          round_trip(config, case_id, data.decode("ascii"), gui_fallback=False))
            except Exception as error:
                if not guarded:
                    raise
                emit(f"{case_id}: round trip raised {type(error).__name__}: {error}")
                _guard_stop(config, case_id, getattr(error, "settings_guard", None), emit)
                emit(f"{case_id}: the round trip did not end cleanly; stopped")
                raise RunStopped(case_id) from error
            if record.get("route") in REFUSED_ROUTES:  # refused before anything ran or was cleared
                emit(f"{case_id}: route={record['route']}; {record.get('reason')}; no exported MBF")
                return False
            # The guard comes first: a rejected hash or any other early return still stops the run on it.
            _guard_stop(config, case_id, record.get("settings_guard"), emit)
            if not gui_fallback and record.get("route") == "batch_no_csv":
                emit(f"{case_id}: route=batch_no_csv; the GUI fallback is refused for this run; stopped")
                raise RunStopped(case_id)
            if record.get("mbf", {}).get("sha256") != mbf_sha256 or record.get("guest_mbf_sha256") != mbf_sha256:
                emit(f"{case_id}: route={record.get('route')}; the guest did not report importing this MBF; "
                     "no exported MBF")
                return False
            ineligible = solve_ineligible(record)
            if ineligible:  # recorded in its run folder, never published, and no export reads it
                emit(f"{case_id}: route={record.get('route')}; {ineligible}; not published; no exported MBF")
                return False
            # The run becomes current only once it is checked complete (`publish`); the export then reads it.
            with exchange_tree(config) as tree:
                publish(tree, case_id, run=record["run_folder"], reason="an authored round trip")
            settings = record.get("settings_guard")
            emit(f"{case_id}: route={record.get('route')}; round trip recorded: "
                 f"cases/{case_id}/{record['run_folder']}/caepipe_out/run_record.json"
                 + (f"; settings {settings['status']}" if settings else ""))
            if no_export:
                return True
    except ContainmentError as error:
        emit(f"{case_id}: route=containment_violation; {error}; no exported MBF")
        return False
    return _export_and_publish(config, case_id, mbf_sha256, emit=emit, export=export)


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
        if export_mbf and not results[case["case_id"]]:
            # A refused or failed case is not followed by its export step.
            emit(f"{case['case_id']}: export route=not_run; the case's live round trip did not complete; "
                 "no exported MBF")
            results[f"{case['case_id']} export"] = False
        elif export_mbf:
            results[f"{case['case_id']} export"] = export_case(case, config, emit=emit, export=export)
    for key, ok in run_series(config, runner=runner, hand_calcs=hand_calcs, emit=emit).items():
        results[key] = results.get(key, True) and ok
    return results


def load_benchmark(manifest_path: Path, selected: list[str] | None = None) -> list[dict[str, Any]]:
    """The benchmark models of a generator manifest, each checked against the manifest's hashes (Design §11.1)."""
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if manifest.get("document_kind") != BENCHMARK_MANIFEST_KIND:
        raise ValueError(f"{manifest_path.name}: document_kind must be {BENCHMARK_MANIFEST_KIND}")
    tracked = {case["case_id"] for case in load_cases()} | {case["case_id"] for case in load_authored_cases()}
    models: dict[str, dict[str, Any]] = {}
    for entry in manifest.get("models") or []:
        model_id = entry.get("model_id")
        if not isinstance(model_id, str) or not CASE_ID_PATTERN.match(model_id) or model_id in models:
            raise ValueError(f"{manifest_path.name}: model id {model_id!r} is invalid or repeated")
        if model_id in tracked:
            raise ValueError(f"{manifest_path.name}: model id {model_id} names a tracked case")
        if entry.get("file") != f"models/{model_id}.json":
            raise ValueError(f"{manifest_path.name}: {model_id} must name its file models/{model_id}.json")
        data = (manifest_path.parent / entry["file"]).read_bytes()
        if hashlib.sha256(data).hexdigest() != entry.get("file_sha256"):
            raise ValueError(f"{manifest_path.name}: models/{model_id}.json differs from the manifest's file_sha256")
        document = json.loads(data.decode("utf-8"))
        if (document.get("document_kind") != BENCHMARK_MODEL_KIND or document.get("model_id") != model_id
                or canonical_model_hash(document.get("model")) != entry.get("model_hash")):
            raise ValueError(f"{manifest_path.name}: models/{model_id}.json is not the model the manifest names")
        if not isinstance(document.get("piping_code"), str):
            raise ValueError(f"{manifest_path.name}: {model_id} names no piping code; the benchmark writes one explicitly")
        models[model_id] = document
    if selected:
        unknown = sorted(set(selected) - set(models))
        if unknown:
            raise KeyError(f"unknown benchmark model id(s): {', '.join(unknown)}")
        return [models[model_id] for model_id in sorted(set(selected))]
    return [models[model_id] for model_id in sorted(models)]


def benchmark_batches(models: list[Mapping[str, Any]], size: int) -> list[list[Mapping[str, Any]]]:
    return [list(models[start:start + size]) for start in range(0, len(models), size)]


RECORDED, NOT_RUN, RETRY = "recorded", "not_run", "retry"


@dataclass(frozen=True)
class BenchmarkStops:
    """Host files checked before each live benchmark model: `freeze` present stops the run; `exclusive` naming a
    running process holds it (polled, up to BENCHMARK_EXCLUSIVE_WAIT_S, then stops)."""

    freeze: Path
    exclusive: Path


def _exclusive_pending(path: Path) -> bool:
    """Whether `path` names a running process by its id (a missing, unreadable or stale file is not pending)."""
    try:
        pid = int(path.read_text(encoding="ascii").strip())
    except (OSError, UnicodeDecodeError, ValueError):
        return False
    if pid <= 0:
        return False
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        return False
    except PermissionError:
        return True
    return True


def run_benchmark_model(entry: Mapping[str, Any], config: HarnessConfig, *, runner: str, offline: bool,
                        emit: Callable[[str], None] = print,
                        round_trip: Callable[..., dict[str, Any]] = run_caepipe_round_trip,
                        gui_fallback: bool = False, attempt: int = 1) -> tuple[str, str, str]:
    """One benchmark model: solve, write, guarded batch round trip (or the stored bound CSV), parse, compare.

    Returns (status, route, reason): `recorded`, `not_run` (SWBPIPE not solved, MBF refused, containment, or no
    stored bound CSV offline), or `retry` (the round trip raised, or left no bound CSV, with a clean settings check).
    Raises RunStopped when the round trip's saved-settings check is not clean, or when the batch run left no CSV and
    the GUI fallback is off."""
    model_id = entry["model_id"]
    try:
        return _run_benchmark_model(entry, config, runner=runner, offline=offline, emit=emit, round_trip=round_trip,
                                    gui_fallback=gui_fallback, attempt=attempt)
    except ContainmentError as error:
        emit(f"{model_id}: route=containment_violation; {error}; no benchmark record")
        return NOT_RUN, "containment_violation", str(error)


def _benchmark_manifest(folder: CaseFolder, entry: Mapping[str, Any], *, offline: bool, outcome: str, route: str,
                        attempt: int, **extra: Any) -> str:
    """The model's private run record for this attempt (the case manifest): a new `case_manifest.json` in the
    attempt's own run or screen folder, never anywhere else. Returns its SHA-256."""
    return folder.record("case_manifest.json", {
        "document_kind": MANIFEST_KIND,
        "case_id": entry["model_id"],
        "created": _now(),
        "mode": "benchmark_offline" if offline else "benchmark_live",
        "outcome": outcome,
        "route": route,
        "attempt": attempt,
        "benchmark_model": f"validation/caepipe_benchmark/models/{entry['model_id']}.json",
        "family": entry.get("family"),
        "generator": entry.get("generator"),
        "piping_code": entry["piping_code"],
        "internal_model_hash": canonical_model_hash(entry["model"]),
        **extra,
        "folder": folder.relative,
        "layer_privacy": LAYER_PRIVACY,
        "boundary_note": BOUNDARY_NOTE,
    })["sha256"]


def _run_benchmark_model(entry: Mapping[str, Any], config: HarnessConfig, *, runner: str, offline: bool,
                         emit: Callable[[str], None], round_trip: Callable[..., dict[str, Any]], gui_fallback: bool,
                         attempt: int) -> tuple[str, str, str]:
    """One benchmark model. Live: the harness writes a new run folder; this attempt's manifest is a new file in it
    (never the stored one), and the run is published only with its comparison. Offline: a new screen of the model's
    current run. A model whose SWBPIPE solve or MBF render fails writes nothing."""
    model_id = entry["model_id"]
    root = config.exchange_root.resolve()

    def outcome(status: str, route: str, reason: str, run_folder: str | None = None) -> tuple[str, str, str]:
        if run_folder:
            with exchange_tree(config) as tree:
                folder = open_folder(tree, model_id, run_folder)
                if not folder.exists("case_manifest.json"):
                    _benchmark_manifest(folder, entry, offline=offline, outcome="no_record", route=route,
                                        attempt=attempt, reason=reason)
        if status == NOT_RUN:
            emit(f"{model_id}: route={route}; {reason}; no benchmark record")
        return status, route, reason

    body, output, error = solve_swbpipe(runner, {
        "case_id": model_id, "model": entry["model"],
        "source_location": f"validation/caepipe_benchmark/models/{model_id}.json"})
    if error:
        return outcome(NOT_RUN, "not_run", f"SWBPIPE not solved ({error})")
    mbf = render_caepipe_mbf_si(entry["model"], title=entry["model_id"],
                                reference_temperature_c=entry.get("reference_temperature_c", 20.0),
                                piping_code=entry["piping_code"])
    if mbf.status != "written":
        codes = sorted({item["code"] for item in mbf.diagnostics if item["severity"] == "error"})
        return outcome(NOT_RUN, "not_run", f"MBF refused ({', '.join(codes)})")
    mbf_text = mbf.text
    mbf_sha256 = _sha256(mbf_text)
    inputs = {
        "swbpipe/request.json": _json_bytes(body),
        "swbpipe/result.json": _json_bytes(output),
        "caepipe_in/id_map.json": _json_bytes(mbf.sidecar(mbf_sha256)),
        "caepipe_in/mbf_render.json": _json_bytes({
            "profile_id": mbf.to_dict()["profile_id"], "status": mbf.status, "layout_row_count": mbf.layout_row_count,
            "piping_code": entry["piping_code"], "diagnostics": mbf.diagnostics, "loss_report": mbf.loss_report,
            "assumptions": mbf.assumptions, "mbf_sha256": mbf_sha256}),
    }

    def record_benchmark(folder: CaseFolder, csv_path: Path, route: str) -> tuple[dict[str, str], str]:
        parsed = read_caepipe_print_csv(csv_path)
        written = {"parsed/caepipe_results.json": folder.record("parsed/caepipe_results.json", parsed)["sha256"],
                   "parsed/parser_diagnostics.json": folder.record("parsed/parser_diagnostics.json", {
                       "document_kind": PARSER_DIAGNOSTICS_KIND,
                       **{key: parsed[key] for key in ("parser_version", "diagnostics", "sections_parsed",
                                                       "sections_skipped")}})["sha256"]}
        comparison = build_benchmark_record(entry, swbpipe_result_payload=output["payload"], mbf_result=mbf,
                                            mbf_text=mbf_text, parsed_caepipe=parsed)
        relative = f"comparison/{BENCHMARK_RECORD}"
        written[relative] = folder.record(relative, comparison)["sha256"]
        case_root = root / "cases" / model_id
        written["case_manifest.json"] = _benchmark_manifest(
            folder, entry, offline=offline, outcome="recorded", route=route, attempt=attempt,
            exported_model_hash=mbf_sha256, target_tool=comparison["target_tool"],
            target_version=comparison["target_version"], product_label=comparison["product_label"],
            trial_first_launch_date=config.trial_first_launch_date, csv=str(csv_path.relative_to(case_root)),
            comparison=f"{folder.relative}/{relative}")
        return written, f"cases/{model_id}/{folder.relative}/{relative}"

    if offline:
        view = current_case(root, model_id)
        csv_path, route, reason = bound_csv(root, {"root": view.run},
                                            stored_run_record(root, view.run / "caepipe_out"), mbf_sha256)
        if csv_path is None:
            return outcome(NOT_RUN, route, reason)
        read = _screen_read(root, view, csv_path)
        with exchange_tree(config) as tree:
            screen = reserve_folder(tree, model_id, SCREENS_DIR)
            for relative, data in sorted(inputs.items()):
                screen.create(relative, data)
            screen.create("caepipe_in/model.mbf", mbf_text.encode("utf-8"))
            written, shown = record_benchmark(screen, csv_path, route)
            _write_screen_record(screen, model_id, read, "benchmark_offline")
            publish(tree, model_id, screen=screen.relative, also=written, expect_run=view.source,
                    reason="an offline benchmark screen of the current run")
        emit(f"{model_id}: route={route}; benchmark record written: {shown}")
        return RECORDED, route, ""

    try:
        record = round_trip(config, model_id, mbf_text, gui_fallback=gui_fallback, files=inputs)
    except Exception as error:
        emit(f"{model_id}: round trip raised {type(error).__name__}: {error}")
        _guard_stop(config, model_id, getattr(error, "settings_guard", None), emit)
        return outcome(RETRY, "round_trip_raised", f"the round trip raised {type(error).__name__}",
                       getattr(error, "run_folder", None))
    run_folder = record.get("run_folder")
    if record.get("route") in REFUSED_ROUTES:
        # Refused with nothing stored changed: the attempt's manifest is a new file in its own run folder, and the run
        # stops (no later model would reach the exchange folder either).
        outcome(RETRY, record["route"], str(record.get("reason")), run_folder)
        emit(f"{model_id}: route={record['route']}; {record.get('reason')}; stopped")
        raise RunStopped(model_id)
    # The guard comes first: any other outcome still stops the run on it.
    _guard_stop(config, model_id, record.get("settings_guard"), emit)
    if not gui_fallback and record.get("route") == "batch_no_csv":
        outcome(RETRY, "batch_no_csv", "the batch run left no CSV and the GUI fallback is off", run_folder)
        emit(f"{model_id}: route=batch_no_csv; the GUI fallback is off for this run; stopped")
        raise RunStopped(model_id)
    csv_path, route, reason = bound_csv(root, {"root": root / "cases" / model_id / run_folder}, record, mbf_sha256)
    if csv_path is None:
        return outcome(RETRY, route, reason, run_folder)
    with exchange_tree(config) as tree:
        folder = open_folder(tree, model_id, run_folder)
        written, shown = record_benchmark(folder, csv_path, route)
        publish(tree, model_id, run=folder.relative, also=written, reason="a live benchmark solve with its record")
    # Operational status only: the classification counts stay in the private record.
    emit(f"{model_id}: route={route}; benchmark record written: {shown}")
    return RECORDED, route, ""


def _benchmark_gate(model_id: str, config: HarnessConfig, stops: BenchmarkStops, *,
                    liveness: Callable[[HarnessConfig], dict[str, Any]],
                    precondition: Callable[[HarnessConfig], dict[str, Any]], sleep: Callable[[float], None],
                    emit: Callable[[str], None], liveness_wait_s: float = BENCHMARK_RETRY_WAIT_S) -> str | None:
    """Before each live model: the freeze file, a pending exclusive job, the guest's liveness (checked again once
    after `liveness_wait_s`) and the no-foreign-instance precondition. None when the model may start, else why
    the run stops."""
    if stops.freeze.exists():
        return "the freeze file is present"
    waited = 0.0
    while _exclusive_pending(stops.exclusive):
        if waited >= BENCHMARK_EXCLUSIVE_WAIT_S:
            return f"an exclusive job was still pending after {waited:g} s"
        if waited == 0.0:
            emit(f"{model_id}: an exclusive job is pending; waiting for it (up to {BENCHMARK_EXCLUSIVE_WAIT_S:g} s)")
        sleep(BENCHMARK_POLL_S)
        waited += BENCHMARK_POLL_S
    alive = liveness(config)
    if alive["status"] != "alive":
        emit(f"{model_id}: liveness {alive['status']}; {alive.get('reason', '')}; "
             f"checking again in {liveness_wait_s:g} s")
        sleep(liveness_wait_s)
        alive = liveness(config)
        if alive["status"] != "alive":
            return f"liveness {alive['status']}; {alive.get('reason', '')}"
    if stops.freeze.exists():
        return "the freeze file is present"
    pre = precondition(config)
    if pre["status"] != "clear":
        return f"{pre['status']}; {pre.get('reason', '')}"
    return None


def run_benchmark(models: list[Mapping[str, Any]], config: HarnessConfig, *, runner: str, offline: bool,
                  batch_size: int = BENCHMARK_BATCH_SIZE, batch: int | None = None,
                  emit: Callable[[str], None] = print,
                  round_trip: Callable[..., dict[str, Any]] = run_caepipe_round_trip,
                  precondition: Callable[[HarnessConfig], dict[str, Any]] | None = None,
                  liveness: Callable[[HarnessConfig], dict[str, Any]] | None = None,
                  stops: BenchmarkStops | None = None, pace_s: float = BENCHMARK_PACE_S,
                  sleep: Callable[[float], None] | None = None,
                  liveness_wait_s: float = BENCHMARK_RETRY_WAIT_S) -> int:
    """Run the models one at a time, in serial batches; the exit status of main.

    A live run paces the models `pace_s` apart, passes `_benchmark_gate` before each attempt, and retries a model
    once, BENCHMARK_RETRY_WAIT_S later, when its round trip raised or left no bound CSV with a clean settings check.
    A model that still left no CSV is recorded as such and the run goes on; a round trip that raised twice, a gate
    that fails, or an unclean settings check stops the run. The GUI fallback is on only when the settings guard
    restores the whole key, whose restore covers the settings that route writes (Design §5)."""
    batches = benchmark_batches(models, batch_size)
    if batch is not None:
        if not 1 <= batch <= len(batches):
            emit(f"error: --batch {batch} is not one of the {len(batches)} batches")
            return 2
        chosen = [(batch, batches[batch - 1])]
    else:
        chosen = list(enumerate(batches, start=1))
    if not offline and stops is None:
        raise ValueError("a live benchmark run needs its stop files")
    check = precondition or check_no_foreign_instance
    alive = liveness or check_guest_alive
    sleep = sleep or time.sleep
    gui_fallback = bool(config.settings_guard is not None and config.settings_guard.restores_whole_key)
    results, started = [], False
    for number, members in chosen:
        emit(f"batch {number}/{len(batches)}: {', '.join(entry['model_id'] for entry in members)}")
        for entry in members:
            model_id = entry["model_id"]
            status, route, reason = NOT_RUN, "not_run", ""
            try:
                for attempt in (1, 2):
                    if not offline:
                        if started:
                            sleep(pace_s if attempt == 1 else BENCHMARK_RETRY_WAIT_S)
                        stop = _benchmark_gate(model_id, config, stops, liveness=alive, precondition=check,
                                               sleep=sleep, emit=emit, liveness_wait_s=liveness_wait_s)
                        if stop is not None:
                            emit(f"stopped before {model_id}: {stop}")
                            return 1
                        started = True
                    status, route, reason = run_benchmark_model(entry, config, runner=runner, offline=offline,
                                                                emit=emit, round_trip=round_trip,
                                                                gui_fallback=gui_fallback, attempt=attempt)
                    if status != RETRY:
                        break
                    if attempt == 1:
                        emit(f"{model_id}: route={route}; {reason}; retrying once in {BENCHMARK_RETRY_WAIT_S:g} s")
            except RunStopped as stopped:
                emit(f"stopped after {stopped}: the guarded round trip did not end cleanly; no further model ran")
                return 1
            if status == RETRY:
                if route == "round_trip_raised":
                    emit(f"stopped after {model_id}: the round trip raised again after one retry; no further model ran")
                    return 1
                emit(f"{model_id}: route={route}; {reason}; no benchmark record after one retry")
            results.append(status == RECORDED)
    return 0 if all(results) else 1


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
    parser.add_argument("--authored-dir", metavar="PATH",
                        help="with --authored, take the cases from this directory's manifest.json (inside the exchange "
                             "folder) instead of the tracked ones; implies --no-export and needs settings_guard")
    parser.add_argument("--no-export", action="store_true",
                        help="with --authored, run the round trip only (no export step)")
    parser.add_argument("--runner", help=f"openpipestress-runner executable (default: ${RUNNER_ENV})")
    parser.add_argument("--benchmark", metavar="MANIFEST",
                        help="run the generated benchmark models of this manifest (two-way comparison, guarded "
                             "round trips, paced one at a time)")
    parser.add_argument("--batch", type=int, metavar="N", help="with --benchmark, run only the N-th batch")
    parser.add_argument("--batch-size", type=int, default=BENCHMARK_BATCH_SIZE, metavar="N",
                        help=f"with --benchmark, models per serial batch (default {BENCHMARK_BATCH_SIZE})")
    parser.add_argument("--freeze-file", metavar="PATH",
                        help="with a live --benchmark run (required): a host file whose presence stops the run")
    parser.add_argument("--exclusive-file", metavar="PATH",
                        help="with a live --benchmark run (required): a host file naming the process id of a pending "
                             "exclusive job, which holds the run")
    parser.add_argument("--pace-s", type=float, default=BENCHMARK_PACE_S, metavar="S",
                        help=f"with a live --benchmark run, seconds between models (default {BENCHMARK_PACE_S:g})")
    parser.add_argument("--liveness-wait-s", type=float, default=BENCHMARK_RETRY_WAIT_S, metavar="S",
                        help="with a live --benchmark run, seconds before the guest's liveness is checked once more "
                             f"(default {BENCHMARK_RETRY_WAIT_S:g})")
    args = parser.parse_args(argv)
    benchmark_only = (args.batch is not None or args.batch_size != BENCHMARK_BATCH_SIZE or args.freeze_file
                      or args.exclusive_file or args.pace_s != BENCHMARK_PACE_S
                      or args.liveness_wait_s != BENCHMARK_RETRY_WAIT_S)
    if args.benchmark and (args.authored or args.authored_dir or args.no_export or args.export_mbf
                           or args.export_only):
        parser.error("--benchmark takes only --case, --offline, --batch, --batch-size, --freeze-file, "
                     "--exclusive-file, --pace-s, --liveness-wait-s and --runner")
    if not args.benchmark and benchmark_only:
        parser.error("--batch, --batch-size, --freeze-file, --exclusive-file, --pace-s and --liveness-wait-s need "
                     "--benchmark")
    if args.batch_size < 1:
        parser.error("--batch-size must be at least 1")
    if not (args.pace_s >= 0.0):
        parser.error("--pace-s must be a non-negative number")
    if not (0.0 <= args.liveness_wait_s < float("inf")):
        parser.error("--liveness-wait-s must be a non-negative number")
    if args.benchmark and not args.offline and not (args.freeze_file and args.exclusive_file):
        parser.error("a live --benchmark run needs --freeze-file and --exclusive-file")
    if args.benchmark:
        return benchmark_main(args, environ)
    if args.authored and (args.offline or args.export_mbf):
        parser.error("--authored takes only --case and --export-only")
    if (args.authored_dir or args.no_export) and not args.authored:
        parser.error("--authored-dir and --no-export need --authored")
    if args.export_only and (args.authored_dir or args.no_export):
        parser.error("--export-only is not allowed with --authored-dir or --no-export")
    try:
        cases = [] if args.authored_dir else load_authored_cases(args.cases) if args.authored else load_cases(args.cases)
        load_series()
    except (KeyError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 2
    config = resolve_harness_config(environ, require_vm_tools=not args.offline)
    if isinstance(config, HarnessSkip):
        print(f"CAEPIPE harness skipped: {config.reason}")
        return 0
    if args.authored:
        directory = AUTHORED_DIR
        if args.authored_dir:
            # Checked only once the exchange folder is known: the directory must lie inside it.
            if config.settings_guard is None:
                print(f"error: --authored-dir runs need settings_guard in {CONFIG_NAME}", file=sys.stderr)
                return 2
            try:
                directory = external_authored_dir(config.exchange_root, args.authored_dir)
                cases = load_authored_cases(args.cases, directory)
            except (ContainmentError, KeyError, OSError, ValueError) as error:
                print(f"error: {error}", file=sys.stderr)
                return 2
        for warning in config.warnings:
            print(f"warning: {warning}")
        if args.authored_dir:
            # Session precondition: no instance of the tool may be running that this run did not start.
            pre = check_no_foreign_instance(config)
            if pre["status"] != "clear":
                print(f"stopped before the first case: {pre['status']}; {pre.get('reason', '')}")
                return 1
        results = []
        try:
            for case in cases:
                results.append(run_authored_case(case, config, export_only=args.export_only, directory=directory,
                                                 no_export=args.no_export or bool(args.authored_dir),
                                                 gui_fallback=not args.authored_dir
                                                 or config.settings_guard.restores_whole_key))
        except RunStopped as stopped:
            print(f"stopped after {stopped}: the guarded round trip did not end cleanly; no further case ran")
            return 1
        return 0 if all(results) else 1
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


def benchmark_main(args: argparse.Namespace, environ: Mapping[str, str]) -> int:
    try:
        models = load_benchmark(Path(args.benchmark), args.cases)
    except (KeyError, OSError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 2
    config = resolve_harness_config(environ, require_vm_tools=not args.offline)
    if isinstance(config, HarnessSkip):
        print(f"CAEPIPE harness skipped: {config.reason}")
        return 0
    if not args.offline and config.settings_guard is None:
        print(f"error: --benchmark runs need settings_guard in {CONFIG_NAME}", file=sys.stderr)
        return 2
    runner = args.runner or environ.get(RUNNER_ENV)
    if not runner or not Path(runner).is_file():
        print(f"error: no runner executable (--runner or ${RUNNER_ENV})", file=sys.stderr)
        return 2
    for warning in config.warnings:
        print(f"warning: {warning}")
    stops = None if args.offline else BenchmarkStops(freeze=Path(args.freeze_file),
                                                     exclusive=Path(args.exclusive_file))
    return run_benchmark(models, config, runner=runner, offline=args.offline, batch_size=args.batch_size,
                         batch=args.batch, stops=stops, pace_s=args.pace_s, liveness_wait_s=args.liveness_wait_s)


if __name__ == "__main__":
    raise SystemExit(main())
