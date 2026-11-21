"""Run the CAEPIPE correlation pilot (DEL-17-05 Design §3-§7) for every case, one case at a time.

From the project root:

    python3 -m tools.caepipe_correlation.run_pilot [--case ID ...] [--offline | --export-mbf | --export-only]
        [--runner PATH]
    python3 -m tools.caepipe_correlation.run_pilot --authored [--case ID ...] [--export-only | --no-export]
    python3 -m tools.caepipe_correlation.run_pilot --authored --authored-dir PATH [--case ID ...]
    python3 -m tools.caepipe_correlation.run_pilot --benchmark MANIFEST [--case ID ...] [--batch N]
        [--batch-size N] [--runner PATH] (--offline | --freeze-file PATH --exclusive-file PATH [--pace-s S]
        [--collect-only])

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

The export modes described below are held: the guest script returns
`export_held` and starts nothing until the export adopts the attempt's claim
and job. The export descriptions state the retained path, not currently
available behavior.

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
and no runner is needed. --authored-dir takes the cases from another directory
with the same manifest layout instead; it must lie inside the exchange folder,
with no symbolic link on the way, so private authored files stay private, and its
case ids must not name a tracked writer or authored case, whose case folders the
run would otherwise replace. --no-export leaves out the export step; --authored-dir
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
(comparison/<solve id>/benchmark_comparison.json), with the case manifest as the
model's run record, all in the exchange folder. Models run one at a time in serial
batches of --batch-size (default 4); --batch N runs only the N-th batch. A live
benchmark run needs the configuration's settings_guard, --freeze-file and
--exclusive-file. It paces the models --pace-s apart (default 60 s). Before each
model it stops if the freeze file exists, holds while the exclusive file names a
running process (up to 30 min), checks that the guest answers `cmd /c ver`
(once more after 2 min) and that no instance of the tool is running. A model
whose round trip raised or left no CSV, with a clean settings check, is retried
once 2 min later only when its program was never started. A model whose round
trip's attempt claimed and ended without its terminal report (route start_partial)
or whose success report is contradicted by its own nonzero dispatch reply (route
report_reply_contradiction), both outcome partial, and one whose two attempts were
each established as never started (route not_started) are never retried. The run
stops at the first unclean settings check, a failed check before a model, a round
trip that raised twice, a round trip whose start is unresolved (its case manifest
then stays at round_trip_started, and nothing is restored), a round trip whose
program ran and that raised or left no CSV (completed is never a ground for retry),
an outcome that cannot be written once the round trip has been called, or a
second attempted run that did not agree (partial or contradicted); the GUI
print fallback is used only with restore_scope "key" (Design §5). --offline builds the records
from the stored, bound CSVs.

The CAEPIPE side of a case is kept as collected: its print and the harness's run
record are never replaced. Every live attempt on a case whose stored run record
already names a collected print is refused by name (print_stored), so that the
print stays as it is; compare it again with --offline, or move the case folder
aside to run the program again. A run record that ended in harness_error after a
clean batch on the verified input still names its print (from the guest's report
and the attempt's digested outputs); one that ended in harness_error with outputs
collected but does not establish a clean batch refuses the live run as
print_unestablished. Where the run record holds a digest for the print, its bytes
must match it (print_digest_mismatch otherwise). A case manifest that is a
symbolic link is refused (manifest_link). SWBPIPE's side may be solved again at any revision: every
solve, live or offline, writes its request and result under
swbpipe/<solve id>/ and creates its comparison under comparison/<solve id>/,
never replacing one, where the
solve id names the UTC time, the SWBPIPE commit and whether the tree was dirty.
Each solve records the commit, whether its tree was clean, the runner binary's
SHA-256, the SHA-256 of the request and result, and the numerical standing the
product published for that solve; the comparison record carries it as
swbpipe_solve, beside the print it compared. The case manifest is the record of
the live run: its route, attempt and outcome, and the solve it used. It is
written before each attempt's round trip (outcome round_trip_started) and again
with the attempt's outcome. Offline, a case with no folder is refused as
no_stored_case. The one retry of a model reuses its own solve.

A capture-only run (one with no SWBPIPE counterpart yet: no SWBPIPE state equals
it, or the product has no headless entry for its analysis) is round-tripped like
any other run: its print is collected, bound and parsed (parsed/), and no SWBPIPE
solve or comparison is made (outcome "captured"). Offline it is parsed from the
stored, bound print. A run declared with a window instruction (a member entered,
or added records checked, at the target program's window) is refused by name
(window_run): the batch round trip cannot make that entry or check.

--collect-only (live only) stops each model once its CAEPIPE print is collected
and bound to the MBF (outcome "collected"): the print is not parsed and no
comparison is written, and the GUI print fallback is off. --offline then parses
and compares it.

The run phase's guest primitives are not established on the deployed guest
(DEL-17-05, Scope of work, Not met); a live run is attended by the agent or
person who starts it, one run at a time.

Standard output names, per case, its route and either the record written (with
the solve id and the comparison's counts by screening class) or the reason none
was written; per export step its route and either the exported MBF's path or the
reason none was written; and per series the path of its record. The private
records under the exchange folder hold everything else. Exit status 0 when every
selected case and every rebuild produced its record, or when the harness is not
configured (the reason is printed and nothing is written); 1 otherwise, including
any export step that wrote no exported MBF; 2 for a usage error.
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
import secrets
import shutil
import signal
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
    case_paths,
    parse_caepipe_print_bytes,
    read_caepipe_print_csv,
    resolve_harness_config,
    run_caepipe_mbf_export,
    run_caepipe_round_trip,
)
from core.handoff.caepipe_external.correlation import evaluate_hypothesis_series, series_summary  # noqa: E402
from core.handoff.caepipe_external.harness import (  # noqa: E402
    ATTEMPT_TOKEN_PATTERN,
    CONFIG_NAME,
    check_guest_alive,
    check_no_foreign_instance,
    exchange_share_letter,
    open_live_session,
    BOUNDARY_NOTE,
    SETTINGS_KEEP_GOING,
    CASE_ID_PATTERN,
    CSV_ROUTES,
    EXPORT_ROUTE,
    EXPORTED_MBF,
    REPORT_REPLY_CONTRADICTION,
    ContainmentError,
    StartUnresolved,
    disposition as start_disposition,
    contained,
    contained_dir,
    stored_run_record,
    write_private,
)
from core.handoff.caepipe_mbf import render_caepipe_mbf_si  # noqa: E402
from tools.caepipe_correlation import benchmark_models  # noqa: E402


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
    """Run `<runner> solve --explicit-local-private-intent --input -`; return (request body, output, error).

    A solve is clean only when the runner exits 0 and its output reports the mechanics solved. Any other exit is an
    error, whatever stdout holds, and its parsed output is kept: the runner exits 1 with a blocking diagnostic or a
    blocked export and 2 for unusable input (docs/validation_manual/headless_runner_reproduction.md), and a negative
    exit is the signal that ended it."""
    body = {"request": runner_request(case["case_id"], case.get("source_location")),
            "solve": {"preview_model": {"model": case["model"], "materials": []}}}
    completed = subprocess.run([runner, "solve", "--explicit-local-private-intent", "--input", "-"],
                               input=json.dumps(body), capture_output=True, text=True, check=False)
    try:
        output = json.loads(completed.stdout)
    except json.JSONDecodeError:
        return body, None, f"runner exit {completed.returncode}; stdout is not JSON"
    if completed.returncode != 0:
        return body, output, f"runner exit {completed.returncode}{_ended_by(completed.returncode)}; not a clean solve"
    mechanics = (((output.get("payload") or {}).get("mechanics_envelope") or {}).get("status") or {}).get("mechanics")
    if mechanics != "MECHANICS_SOLVED":
        return body, output, f"SWBPIPE mechanics status {mechanics!r} (runner exit {completed.returncode})"
    return body, output, ""


def _ended_by(returncode: int) -> str:
    """For a negative exit, the signal that ended the process, as subprocess reports it; otherwise nothing."""
    if returncode >= 0:
        return ""
    try:
        return f" (killed by signal {signal.Signals(-returncode).name})"
    except ValueError:
        return f" (killed by signal {-returncode})"


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


def _create_private_json(root: Path, path: Path, value: Any) -> str:
    """Create `path` inside the exchange folder, failing if it exists (O_EXCL), and return the SHA-256 of the bytes
    written. A benchmark run's SWBPIPE request and result are created this way, so they are never replaced."""
    data = (json.dumps(value, indent=2, sort_keys=True) + "\n").encode("utf-8")
    descriptor = os.open(contained(root, path), os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o666)
    with os.fdopen(descriptor, "wb") as handle:
        handle.write(data)
    return hashlib.sha256(data).hexdigest()


def _write_fd(descriptor: int, data: bytes) -> None:
    with os.fdopen(descriptor, "wb") as handle:
        handle.write(data)


def _replace_private_json(root: Path, path: Path, value: Any) -> None:
    """Replace `path` inside the exchange folder by writing a new temporary file in the same folder and renaming it
    over the old one, so an interruption during the write leaves the earlier file whole. (Nothing is claimed about
    surviving a power loss.)"""
    target = contained(root, path)
    data = (json.dumps(value, indent=2, sort_keys=True) + "\n").encode("utf-8")
    temporary = contained(root, path.parent / f".{path.name}.{os.getpid()}.{secrets.token_hex(4)}.tmp")
    descriptor = os.open(temporary, os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o666)
    try:
        _write_fd(descriptor, data)
        os.replace(temporary, target)
    except BaseException:
        temporary.unlink(missing_ok=True)
        raise


def _file_sha256(path: str | Path) -> str | None:
    """The SHA-256 of a file's bytes, or None when it cannot be read."""
    try:
        return hashlib.sha256(Path(path).read_bytes()).hexdigest()
    except OSError:
        return None


def _source_identity() -> dict[str, Any]:
    """The commit this checkout of SWBPIPE is at and whether its tree is clean (None for either when Git cannot
    say), recorded with each benchmark run so the revision that produced its SWBPIPE result is identified."""
    # Inherited GIT_* variables (a hook's GIT_DIR, for instance) would make Git describe another repository.
    env = {key: value for key, value in os.environ.items() if not key.startswith("GIT_")}

    def git(*args: str) -> subprocess.CompletedProcess | None:
        try:
            return subprocess.run(["git", "-C", str(PROJECT), *args], capture_output=True, text=True, check=False,
                                  timeout=60, env=env)
        except (OSError, subprocess.SubprocessError):
            return None
    head, status = git("rev-parse", "HEAD"), git("status", "--porcelain")
    return {"source_commit": head.stdout.strip() if head is not None and head.returncode == 0 else None,
            "source_tree_clean": (status.stdout == "") if status is not None and status.returncode == 0 else None}


def _resolve_runner(runner: str) -> str:
    """The runner as the absolute real path that will be executed: a bare name is looked up on PATH, as
    subprocess does (one not found there is resolved against the working directory, where subprocess would not
    look), and a path is resolved; the same string is hashed and run."""
    path = Path(runner)
    if os.sep not in runner and not (os.altsep and os.altsep in runner):
        found = shutil.which(runner)
        if found:
            path = Path(found)
    return str(path.resolve())


def _now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def _print_of_record(record: Mapping[str, Any]) -> tuple[str | None, str, str, bool]:
    """(host path, route, reason, recovered): the print a harness run record names.

    A record whose route is a CSV route names its print in observed_csv_path.host. A record that ended in
    harness_error after the program ran clean (an error after the print was collected, such as a failed settings
    write-back) still establishes its print when its own facts show a clean batch on the verified input: start
    completed, exit status 0, input hash verified, a CSV route in the guest's report, an attempt token, and the
    print the guest named among the attempt's collected outputs with its digest. Such a print is `recovered`."""
    route = str(record.get("route") or "unknown_route")
    observed = record.get("observed_csv_path")
    host = observed.get("host") if isinstance(observed, Mapping) else None
    if route in CSV_ROUTES and isinstance(host, str) and host.startswith("caepipe_out/"):
        return host, route, "", False
    if route != "harness_error":
        return None, route, "the run record names no collected CAEPIPE CSV", False
    guest = record.get("guest_log") if isinstance(record.get("guest_log"), Mapping) else {}
    start = record.get("start") if isinstance(record.get("start"), Mapping) else {}
    digests = record.get("outputs_sha256") if isinstance(record.get("outputs_sha256"), Mapping) else {}
    token = record.get("attempt_token")
    named = guest.get("csv_path")
    name = named.replace("/", "\\").split("\\")[-1] if isinstance(named, str) else ""
    failed = [reason for holds, reason in (
        (start.get("state") == "completed", "its start is not recorded as completed"),
        (record.get("exit_status") == 0, "the batch's exit status is not 0"),
        (record.get("input_hash_verified") is True, "the input hash is not recorded as verified"),
        (guest.get("status") in CSV_ROUTES, "the guest reported no print route"),
        (isinstance(token, str) and bool(ATTEMPT_TOKEN_PATTERN.match(token)), "it names no attempt token"),
        (name.lower().endswith(".csv") and name in digests, "the print the guest named is not among its digested "
                                                            "outputs")) if not holds]
    if failed:
        return None, route, ("the run record ended in harness_error and does not establish a clean batch on the "
                             "verified input: " + "; ".join(failed)), False
    return f"caepipe_out/{token}/{name}", str(guest["status"]), "", True


def bound_csv(root: Path, paths: Mapping[str, Path], record: Mapping[str, Any] | None,
              mbf_sha256: str) -> tuple[Path | None, str, str]:
    """(CSV, route, reason): the CSV only when the run record binds it to the current MBF, else None and why. Where the
    record holds a digest for the print (outputs_sha256), the print's bytes must match it; an older record without
    one is accepted on its two MBF hashes alone."""
    if record is None:
        return None, "unbound_output", "no valid caepipe_out/run_record.json binds a CSV to this MBF"
    if (record.get("mbf") or {}).get("sha256") != mbf_sha256:
        return None, "stale", "the stored CAEPIPE output was produced from a different MBF"
    route = str(record.get("route") or "unknown_route")
    if record.get("guest_mbf_sha256") != mbf_sha256:
        return None, (route if route == "input_hash_mismatch" else "unbound_output"), \
            "the guest did not report solving this MBF (guest MBF hash absent or different)"
    host, route, reason, _ = _print_of_record(record)
    if host is None:
        return None, route, reason
    csv_path = contained(root, paths["root"] / host)
    if not csv_path.is_file():
        return None, route, "the CAEPIPE CSV named by the run record is missing"
    digests = record.get("outputs_sha256")
    name = Path(host).name
    if isinstance(digests, Mapping) and name in digests and \
            hashlib.sha256(csv_path.read_bytes()).hexdigest() != digests[name]:
        return None, "print_digest_mismatch", f"the print's bytes do not match the run record's digest for {name}"
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
        # Checked before caepipe_in/ is rewritten, so the MBF that produced a stale output is kept, and before
        # anything is solved, so a case with no bound CSV is left without a SWBPIPE request or result.
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
            emit(f"series {hypothesis}: series record written: {relative}; status {result.get('status')}")
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
            # The guard comes first: a rejected hash or any other early return still stops the run on it.
            _guard_stop(config, case_id, record.get("settings_guard"), emit)
            if not gui_fallback and record.get("route") == "batch_no_csv":
                emit(f"{case_id}: route=batch_no_csv; the GUI fallback is refused for this run; stopped")
                raise RunStopped(case_id)
            if record.get("mbf", {}).get("sha256") != mbf_sha256 or record.get("guest_mbf_sha256") != mbf_sha256:
                emit(f"{case_id}: route={record.get('route')}; the guest did not report importing this MBF; "
                     "no exported MBF")
                return False
            settings = record.get("settings_guard")
            emit(f"{case_id}: route={record.get('route')}; round trip recorded: "
                 f"cases/{case_id}/caepipe_out/run_record.json"
                 + (f"; settings {settings['status']}" if settings else ""))
            if no_export:
                return True
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


def load_benchmark(manifest_path: Path, selected: list[str] | None = None,
                   series: str | None = None) -> list[dict[str, Any]]:
    """The benchmark models of a generator manifest, each checked against the manifest's hashes (DEL-17-05 AC-007).

    A model whose manifest entry declares `runs` is run only in a named series: without `series` it is refused by
    name; with it, its document is that run's (benchmark_models.series_document: the state's model, its closed forms
    and inexact inputs, its OPPRES line and a `run_id` distinct per run). With `series` and no selection, the models
    declaring that run are taken; a selected model without it is refused. A model without runs, with no `series`, is
    loaded as before."""
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if manifest.get("document_kind") != BENCHMARK_MANIFEST_KIND:
        raise ValueError(f"{manifest_path.name}: document_kind must be {BENCHMARK_MANIFEST_KIND}")
    tracked = {case["case_id"] for case in load_cases()} | {case["case_id"] for case in load_authored_cases()}
    models: dict[str, dict[str, Any]] = {}
    declared_runs: dict[str, list[str]] = {}
    lacking: set[str] = set()
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
        runs = entry.get("runs")
        if runs is not None or series is not None:
            declared = {run.get("series"): run for run in runs or [] if isinstance(run, dict)}
            if series is None:
                declared_runs[model_id] = sorted(declared)
            elif series not in declared:
                lacking.add(model_id)
            else:
                document = benchmark_models.series_document(document, series)
                if document["run_id"] != declared[series].get("run_id"):
                    raise ValueError(f"{manifest_path.name}: {model_id} names another run id for {series}")
        models[model_id] = document
    if selected:
        unknown = sorted(set(selected) - set(models))
        if unknown:
            raise KeyError(f"unknown benchmark model id(s): {', '.join(unknown)}")
    chosen = sorted(set(selected)) if selected else sorted(set(models) - (lacking if series else set()))
    refused = [f"{model_id} (runs: {', '.join(declared_runs[model_id])})" for model_id in chosen
               if model_id in declared_runs]
    if refused:
        raise ValueError(f"{manifest_path.name}: these models declare runs; name one with --series: "
                         f"{'; '.join(refused)}")
    if series and set(chosen) & lacking:
        raise ValueError(f"{manifest_path.name}: no {series} run is declared for: "
                         f"{', '.join(sorted(set(chosen) & lacking))}")
    if series and not chosen:
        raise ValueError(f"{manifest_path.name}: no model declares a {series} run")
    return [models[model_id] for model_id in chosen]


def benchmark_run_id(entry: Mapping[str, Any]) -> str:
    """The case id a benchmark document is run and recorded under: its run's id, else its model id."""
    return entry.get("run_id") or entry["model_id"]


WINDOW_RUN = "window_run"


def window_refusal(entry: Mapping[str, Any], emit: Callable[[str], None]) -> tuple[str, str, str]:
    """A run declared with a window instruction (a member entered, or added records checked, at the target program's
    window) cannot be made by the batch round trip, so it is refused by name before anything is solved or written."""
    reason = ("this run is declared with a window instruction (a member entered, or added records checked, at the "
              "target program's window), which the batch round trip cannot make; run it through the program's window")
    emit(f"{benchmark_run_id(entry)}: route={WINDOW_RUN}; {reason}; not run")
    return NOT_RUN, WINDOW_RUN, reason


def _window_run(entry: Mapping[str, Any]) -> bool:
    return bool(entry.get("capture_only") and entry.get("window_instruction"))


def benchmark_batches(models: list[Mapping[str, Any]], size: int) -> list[list[Mapping[str, Any]]]:
    return [list(models[start:start + size]) for start in range(0, len(models), size)]


RECORDED, NOT_RUN, RETRY = "recorded", "not_run", "retry"
# The route under which each settled start that has run the program, and does not agree, is recorded.
ATTEMPTED_ROUTES = {"partial": "start_partial", "contradicted": REPORT_REPLY_CONTRADICTION}


def round_trip_disposition(record: Mapping[str, Any] | None = None,
                           error: BaseException | None = None) -> dict[str, Any]:
    """The one disposition function: what a round trip handed back about its attempt's start, through the
    harness's one rule table (`disposition`). From an error, only the disposition the harness carried on it (a
    StartUnresolved is unresolved); from a record, its `start.state`. Anything absent or unusable is unresolved,
    which is never a ground for retry or restoration. Both of run_benchmark_model's handlers consult it."""
    if error is not None:
        carried = getattr(error, "disposition", None)
        start = "unresolved" if isinstance(error, StartUnresolved) else (
            carried.get("start") if isinstance(carried, Mapping) else None)
    else:
        held = (record or {}).get("start")
        start = held.get("state") if isinstance(held, Mapping) else None
    return start_disposition(start)
# A model whose round trip's attempt claimed and is quiescent but left no terminal report (route start_partial): an
# attempted partial run, terminal, excluded from comparison, and never retried by any loop.
PARTIAL = "partial"
# The case manifest's state written before each attempt's round trip; the attempt's outcome replaces it. A manifest
# left in it shows only that no outcome of that attempt was written (an interrupt, an unresolved start or a stop).
ROUND_TRIP_STARTED = "round_trip_started"
# A collect-only run ends here: the CAEPIPE print collected and bound, nothing parsed or compared.
COLLECTED = "collected"
# A capture-only run ends here: its print collected, bound and parsed; no SWBPIPE counterpart, so no comparison.
CAPTURED = "captured"
# The SWBPIPE solve of each case folder made in this process, so that the one retry of a model reuses its own solve.
_SOLVED_THIS_PROCESS: dict[str, tuple[Any, Any, str, dict[str, Any]]] = {}
# What the comparison record says of the standing it copies from SWBPIPE's result.
STANDING_NOTE = ("as the product published it for this solve; under LIM-026 (docs/RELEASE_LIMITATIONS_v1.0.md) a "
                 "checks_passed quality does not rest on a verified inverse-norm bound")


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
                        gui_fallback: bool = False, attempt: int = 1,
                        collect_only: bool = False) -> tuple[str, str, str]:
    """One benchmark model: solve, write, guarded batch round trip (or the stored bound CSV), parse, compare.

    Returns (status, route, reason): `recorded`; `captured` (a capture-only run: its print collected, bound and
    parsed, no comparison); `collected` (collect-only); `not_run` (SWBPIPE not solved, MBF refused, a print already
    stored, a window run, containment before the round trip was called, no stored bound CSV offline, or route
    `not_started`: the round trip established, twice, that its attempt never started); `partial` (route
    `start_partial` or `report_reply_contradiction`: an attempted run that does not agree; never retried); or `retry`
    (only when the program was never started: the round trip raised, or left no bound CSV, with a clean settings
    check). Raises RunStopped when the round trip's saved-settings check is not clean, when the batch run left no CSV
    and the GUI fallback is off, when the start is unresolved, when the program has run and either the round trip
    raised or it left no bound CSV (completed is never a ground for retry), and when any outcome cannot be written
    once the round trip has been called, so that a retry or a stop is never lost. A stop after the round trip raised
    writes no outcome, so the case manifest stays at round_trip_started."""
    model_id = benchmark_run_id(entry)
    held = {"disposition": start_disposition("not_dispatched")}  # nothing called yet; updated around the round trip
    try:
        return _run_benchmark_model(entry, config, runner=runner, offline=offline, emit=emit, round_trip=round_trip,
                                    gui_fallback=gui_fallback, attempt=attempt, collect_only=collect_only, held=held)
    except RunStopped:
        raise
    except Exception as error:
        settled = held["disposition"]
        # Once the round trip has been called, a failure to write what it led to stops the list: the outcome would
        # otherwise read as not run, losing the retry or the stop it called for. Where the program may be running or
        # has run, the same holds whatever was called.
        if held.get("round_trip_called") or not settled["retry"]:
            note = f"its outcome could not be recorded ({type(error).__name__}: {error})"
            emit(f"{model_id}: route=start_{settled['start']}; {note}; stopped")
            raise RunStopped(f"{model_id} (start_{settled['start']}; {note})") from error
        if not isinstance(error, ContainmentError):
            raise
        emit(f"{model_id}: route=containment_violation; {error}; no benchmark record")
        return NOT_RUN, "containment_violation", str(error)


def _benchmark_manifest(root: Path, paths: Mapping[str, Path], entry: Mapping[str, Any], *, offline: bool,
                        outcome: str, route: str, attempt: int, solve: Mapping[str, Any] | None = None,
                        **extra: Any) -> None:
    """The model's private run record (the case manifest), written before each live attempt's round trip and with its
    outcome. `swbpipe_solve` names the solve the attempt used (none for a capture-only run). The file is replaced
    whole, through a temporary file in the same folder, so an interrupted write leaves the earlier one readable."""
    _replace_private_json(root, paths["manifest"], {
        "document_kind": MANIFEST_KIND,
        "case_id": benchmark_run_id(entry),
        "created": _now(),
        "mode": "benchmark_offline" if offline else "benchmark_live",
        **({key: entry.get(key) for key in ("series_run", "oppres_line")} if "series_run" in entry else {}),
        **({"capture_only": True} if entry.get("capture_only") else {}),
        "outcome": outcome,
        "route": route,
        "attempt": attempt,
        "benchmark_model": f"validation/caepipe_benchmark/models/{entry['model_id']}.json",
        "family": entry.get("family"),
        "generator": entry.get("generator"),
        "piping_code": entry["piping_code"],
        "internal_model_hash": canonical_model_hash(entry["model"]),
        "swbpipe_solve": dict(solve) if solve else None,
        **extra,
        "layer_privacy": LAYER_PRIVACY,
        "boundary_note": BOUNDARY_NOTE,
    })


def _swbpipe_standing(output: Mapping[str, Any] | None) -> dict[str, Any]:
    """The mechanics status and the numerical quality SWBPIPE's result published for its solve, per load case."""
    envelope = ((output or {}).get("payload") or {}).get("mechanics_envelope") or {}
    quality = envelope.get("numerical_quality")
    quality = quality if isinstance(quality, Mapping) else {}
    cases = [{key: case[key] for key in ("load_case_id", "case_id", "solve_quality") if key in case}
             for case in quality.get("cases") or [] if isinstance(case, Mapping)]
    return {"status": envelope.get("status"), "numerical_quality": quality.get("status"), "cases": cases,
            "note": STANDING_NOTE}


def _new_solve_folder(root: Path, swbpipe_dir: Path, identity: Mapping[str, Any]) -> tuple[str, Path]:
    """A new folder for one SWBPIPE solve under swbpipe/, named by the UTC time, the commit and a dirty tree. An id
    whose solve folder or comparison folder already exists is never reused, so no earlier comparison is replaced."""
    commit = identity.get("source_commit")
    name = (datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ") + "-"
            + (commit[:12] if isinstance(commit, str) and commit else "unknown-commit")
            + ("-dirty" if identity.get("source_tree_clean") is False else ""))
    for number in range(1, 1000):
        solve_id = name if number == 1 else f"{name}-{number}"
        folder = contained(root, swbpipe_dir / solve_id)
        if os.path.lexists(folder) or os.path.lexists(swbpipe_dir.parent / "comparison" / solve_id):
            continue
        try:
            folder.mkdir()
        except FileExistsError:
            continue
        return solve_id, folder
    raise RuntimeError(f"no free solve folder for {name}")


def _solve_benchmark(root: Path, paths: Mapping[str, Path], entry: Mapping[str, Any],
                     runner: str) -> tuple[Any, Any, str, dict[str, Any]]:
    """Solve SWBPIPE's side of one benchmark run afresh, in a new solve folder: (request, output, error, solve)."""
    # The runner is resolved once and hashed, and the checkout identified, before the solve; that same path is run.
    resolved = _resolve_runner(runner)
    identity = _source_identity()
    runner_sha256 = _file_sha256(resolved)
    solve_id, folder = _new_solve_folder(root, paths["swbpipe"], identity)
    body, output, error = solve_swbpipe(resolved, {
        "case_id": benchmark_run_id(entry), "model": entry["model"],
        "source_location": f"validation/caepipe_benchmark/models/{entry['model_id']}.json"})
    solve = {"solve_id": solve_id, **identity, "runner_sha256": runner_sha256, "solved": _now(),
             "request": f"swbpipe/{solve_id}/request.json",
             "request_sha256": _create_private_json(root, folder / "request.json", body),
             "result": f"swbpipe/{solve_id}/result.json" if output is not None else None,
             "result_sha256": (_create_private_json(root, folder / "result.json", output) if output is not None
                               else None),
             "standing": _swbpipe_standing(output) if output is not None else None}
    return body, output, error, solve


def _stored_print(root: Path, case_root: Path) -> tuple[str, str] | None:
    """(route, reason) refusing a live run of a case whose stored run record holds a print, else None. A live round
    trip replaces the flat run record, which holds the print's attempt token and digest, so the record is protected:
    print_stored when it establishes a print (recovered after a harness error included), and print_unestablished when
    it ended in harness_error with outputs collected but its facts do not establish a clean batch."""
    if not os.path.isdir(case_root / "caepipe_out"):
        return None
    record = stored_run_record(root, case_root / "caepipe_out")
    if record is None:
        return None
    host, _, reason, recovered = _print_of_record(record)
    if host is not None and contained(root, case_root / host).is_file():
        return "print_stored", (f"this case already holds a collected print ({host}"
                                + (", recovered from a run record that ended in harness_error" if recovered else "")
                                + ") named by its run record, which a live run would replace; compare it again with "
                                  "--offline, or move the case folder aside to run the program again")
    if record.get("route") == "harness_error" and record.get("outputs_collected"):
        return "print_unestablished", (f"this case's run record holds collected outputs, but {reason[0].lower()}"
                                       f"{reason[1:]}; a live run would replace that record, so it is not run: "
                                       "examine the case folder, or move it aside to run the program again")
    return None


def _counts(summary: Mapping[str, Any]) -> str:
    """The comparison's counts by screening class, as printed on standard output."""
    by_class = summary.get("by_classification") or {}
    return ", ".join(f"{name} {by_class[name]}" for name in sorted(by_class)) or "no results"


def _run_benchmark_model(entry: Mapping[str, Any], config: HarnessConfig, *, runner: str, offline: bool,
                         emit: Callable[[str], None], round_trip: Callable[..., dict[str, Any]], gui_fallback: bool,
                         attempt: int, collect_only: bool = False,
                         held: dict[str, Any] | None = None) -> tuple[str, str, str]:
    held = {} if held is None else held
    model_id = benchmark_run_id(entry)
    capture = bool(entry.get("capture_only"))
    if _window_run(entry):
        return window_refusal(entry, emit)
    if collect_only and offline:
        raise ValueError("a collect-only run is live: offline there is nothing to collect")
    root = config.exchange_root.resolve()
    if not CASE_ID_PATTERN.match(model_id):
        raise ValueError(f"case id {model_id!r} must match {CASE_ID_PATTERN.pattern}")
    case_root = root / "cases" / model_id
    if offline and not os.path.lexists(case_root):
        reason = "offline, this case has no stored folder"
        emit(f"{model_id}: route=no_stored_case; {reason}; no benchmark record")
        return NOT_RUN, "no_stored_case", reason
    if os.path.islink(case_root / "case_manifest.json"):
        reason = "this case's manifest is a symbolic link; nothing is solved or written through it"
        emit(f"{model_id}: route=manifest_link; {reason}; not run")
        return NOT_RUN, "manifest_link", reason
    if not offline:
        # On every live attempt: a live round trip replaces the harness's flat run record, so a case whose record
        # holds a print is not run again; the print and its run record are kept, and compared again offline.
        stored = _stored_print(root, case_root)
        if stored is not None:
            route, reason = stored
            emit(f"{model_id}: route={route}; {reason}; not run")
            return NOT_RUN, route, reason
    # Only the retry (attempt 2) of a model reuses the solve its own first attempt made in this process.
    solved = _SOLVED_THIS_PROCESS.get(str(case_root)) if attempt > 1 else None
    paths = case_paths(config, model_id)
    solve: dict[str, Any] = {}

    def outcome(status: str, route: str, reason: str, **extra: Any) -> tuple[str, str, str]:
        if not offline:
            _benchmark_manifest(root, paths, entry, offline=offline,
                                outcome="partial" if status == PARTIAL else "no_record", route=route,
                                attempt=attempt, reason=reason, solve=solve, **extra)
        if status in (NOT_RUN, PARTIAL):
            emit(f"{model_id}: route={route}; {reason}; no benchmark record")
        return status, route, reason

    def stop(route: str, reason: str) -> None:
        # Every stop of this function: its outcome is recorded and the stop is then raised. The stop always wins: if
        # that write fails, RunStopped is still raised, so no caller can turn the stop into another outcome, and the
        # failure is named in the stop's message and in an emitted line. An interrupt propagates as itself.
        try:
            outcome(RETRY, route, reason)
        except Exception as failure:
            note = f"its outcome could not be recorded ({type(failure).__name__}: {failure})"
            emit(f"{model_id}: route={route}; {note}; stopped")
            raise RunStopped(f"{model_id} ({route}; {note})") from failure
        raise RunStopped(model_id)

    def guard_stop(settings: Any, *, raised: bool = False) -> None:
        try:
            _guard_stop(config, model_id, settings, emit)
        except RunStopped:
            stop("settings_stop", "the round trip raised, and its settings check was not clean or not recorded; the "
                                  "run stopped" if raised else "the round trip's settings check was not clean; the run "
                                                               "stopped")

    mbf = render_caepipe_mbf_si(entry["model"], title=entry["model_id"],
                                reference_temperature_c=entry.get("reference_temperature_c", 20.0),
                                piping_code=entry["piping_code"])
    if mbf.status != "written":
        codes = sorted({item["code"] for item in mbf.diagnostics if item["severity"] == "error"})
        return outcome(NOT_RUN, "not_run", f"MBF refused ({', '.join(codes)})")
    mbf_text = benchmark_models.run_mbf_text(entry, mbf.text)  # a series run's OPPRES line substituted
    mbf_sha256 = _sha256(mbf_text)
    if offline:
        # Checked before caepipe_in/ is rewritten, so the MBF that produced a stale output is kept, and before
        # anything is solved, so a case with no bound CSV gets no new solve.
        csv_path, route, reason = bound_csv(root, paths, stored_run_record(root, paths["caepipe_out"]), mbf_sha256)
        if csv_path is None:
            return outcome(NOT_RUN, route, reason)
    if capture or collect_only:
        output, error = None, ""  # no SWBPIPE counterpart, or nothing compared in this run: no solve
    elif solved is None:
        _, output, error, made = _solve_benchmark(root, paths, entry, runner)
        solve.update(made)
        _SOLVED_THIS_PROCESS[str(case_root)] = (None, output, error, dict(made))
    else:
        _, output, error, made = solved
        solve.update(made)
    if error:
        return outcome(NOT_RUN, "not_run", f"SWBPIPE not solved ({error})")
    in_dir = paths["caepipe_in"]
    write_private(root, in_dir / "model.mbf", mbf_text.encode("utf-8"))
    _write_json(root, in_dir / "id_map.json", mbf.sidecar(mbf_sha256))
    _write_json(root, in_dir / "mbf_render.json", {
        "profile_id": mbf.to_dict()["profile_id"], "status": mbf.status, "layout_row_count": mbf.layout_row_count,
        "piping_code": entry["piping_code"], "diagnostics": mbf.diagnostics, "loss_report": mbf.loss_report,
        "assumptions": mbf.assumptions, "mbf_sha256": mbf_sha256})
    if not offline:
        # Before every attempt's round trip: an interrupt from here on leaves this state until the outcome replaces it.
        _benchmark_manifest(root, paths, entry, offline=offline, outcome=ROUND_TRIP_STARTED, route=ROUND_TRIP_STARTED,
                            attempt=attempt, solve=solve)
        held["disposition"] = round_trip_disposition()  # absent while the round trip runs: unresolved
        held["round_trip_called"] = True
        try:
            record = round_trip(config, model_id, mbf_text, gui_fallback=gui_fallback)
        except Exception as error:
            settled = held["disposition"] = round_trip_disposition(error=error)
            if not settled["retry"]:
                # Never a retry: the program may still be running (unresolved) or has run (completed, partial,
                # contradicted). No outcome is written, so the case manifest stays at round_trip_started, and the
                # run stops.
                emit(f"{model_id}: route=start_{settled['start']}; the round trip raised {type(error).__name__}: "
                     f"{error}; stopped")
                raise RunStopped(f"{model_id} (start_{settled['start']})") from error
            emit(f"{model_id}: round trip raised {type(error).__name__}: {error}")
            guard_stop(getattr(error, "settings_guard", None), raised=True)
            return outcome(RETRY, "round_trip_raised", f"the round trip raised {type(error).__name__}")
        settled = held["disposition"] = round_trip_disposition(record=record)
        # The guard comes first: any other outcome still stops the run on it.
        guard_stop(record.get("settings_guard"))
        if settled["start"] == "unresolved":  # a returned record naming no usable start
            emit(f"{model_id}: route=start_unresolved; the round trip's record names no usable start; stopped")
            raise RunStopped(f"{model_id} (start_unresolved)")
        if not gui_fallback and record.get("route") == "batch_no_csv":
            emit(f"{model_id}: route=batch_no_csv; the GUI fallback is off for this run; stopped")
            stop("batch_no_csv", "the batch run left no CSV and the GUI fallback is off")
        if settled["start"] in ATTEMPTED_ROUTES:
            reason = ("the attempt claimed and is quiescent but left no terminal report; an attempted partial run, "
                      "not retried") if settled["start"] == "partial" else (
                "the attempt's success report is contradicted by its own nonzero dispatch reply; an attempted run "
                "that does not agree, not retried and never compared")
            return outcome(PARTIAL, ATTEMPTED_ROUTES[settled["start"]], reason)
        if settled["start"] == "not_started":
            return outcome(NOT_RUN, "not_started", "two attempts were each established as never started; nothing "
                                                   "was made")
        csv_path, route, reason = bound_csv(root, paths, record, mbf_sha256)
        if csv_path is None:
            if settled["retry"]:
                return outcome(RETRY, route, reason)
            # Completed: the program has run, so it is never retried; the list stops.
            emit(f"{model_id}: route={route}; {reason}; the program has run, so it is never retried; stopped")
            stop(route, f"{reason}; the program has run, so it is never retried")
        if collect_only:
            _benchmark_manifest(root, paths, entry, offline=offline, outcome=COLLECTED, route=route, attempt=attempt,
                                solve=solve, exported_model_hash=mbf_sha256, parser_status="not_parsed",
                                trial_first_launch_date=config.trial_first_launch_date,
                                csv=str(csv_path.relative_to(paths["root"])))
            emit(f"{model_id}: route={route}; collected and bound; not parsed or compared (compare it with --offline)")
            return COLLECTED, route, ""
    print_bytes = contained(root, csv_path).read_bytes()
    parsed = parse_caepipe_print_bytes(print_bytes)
    _write_json(root, paths["parsed"] / "caepipe_results.json", parsed)
    _write_json(root, paths["parsed"] / "parser_diagnostics.json", {
        "document_kind": PARSER_DIAGNOSTICS_KIND,
        **{key: parsed[key] for key in ("parser_version", "diagnostics", "sections_parsed", "sections_skipped")}})
    csv = str(csv_path.relative_to(paths["root"]))
    stored_record = stored_run_record(root, paths["caepipe_out"]) or {}
    token = stored_record.get("attempt_token")
    caepipe_print = {"csv": csv, "route": route, "sha256": hashlib.sha256(print_bytes).hexdigest(),
                     **({"attempt_token": token} if isinstance(token, str) and ATTEMPT_TOKEN_PATTERN.match(token)
                        else {}),
                     **({"run_record_route": stored_record.get("route")}
                        if stored_record.get("route") not in (None, route) else {})}
    if capture:
        if not offline:
            _benchmark_manifest(root, paths, entry, offline=offline, outcome=CAPTURED, route=route, attempt=attempt,
                                exported_model_hash=mbf_sha256, parser_status="parsed",
                                trial_first_launch_date=config.trial_first_launch_date, csv=csv,
                                parsed="parsed/caepipe_results.json")
        parsed_path = (paths["parsed"] / "caepipe_results.json").relative_to(root)
        emit(f"{model_id}: route={route}; print captured and parsed: {parsed_path}; sections parsed "
             f"{len(parsed.get('sections_parsed') or [])}; no SWBPIPE counterpart, so no comparison")
        return CAPTURED, route, ""
    comparison = build_benchmark_record(entry, swbpipe_result_payload=output["payload"], mbf_result=mbf,
                                        mbf_text=mbf_text, parsed_caepipe=parsed)
    comparison["swbpipe_solve"] = dict(solve)
    comparison["caepipe_print"] = caepipe_print
    comparison_path = contained_dir(root, paths["comparison"] / solve["solve_id"]) / BENCHMARK_RECORD
    _create_private_json(root, comparison_path, comparison)  # created, never replacing an earlier comparison
    if not offline:
        _benchmark_manifest(root, paths, entry, offline=offline, outcome="recorded", route=route, attempt=attempt,
                            solve=solve, exported_model_hash=mbf_sha256, target_tool=comparison["target_tool"],
                            target_version=comparison["target_version"], product_label=comparison["product_label"],
                            trial_first_launch_date=config.trial_first_launch_date, csv=csv,
                            comparison=str(comparison_path.relative_to(paths["root"])))
    emit(f"{model_id}: route={route}; benchmark record written: {comparison_path.relative_to(root)}; "
         f"solve {solve['solve_id']}; {_counts(comparison['summary'])}")
    return RECORDED, route, ""


def _stops_hold(stops: BenchmarkStops, label: str, *, sleep: Callable[[float], None],
                emit: Callable[[str], None]) -> str | None:
    """The host's stop files, checked with no guest call: the freeze file present stops; a pending exclusive job is
    waited for (up to BENCHMARK_EXCLUSIVE_WAIT_S), then stops. None when neither holds."""
    if stops.freeze.exists():
        return "the freeze file is present"
    waited = 0.0
    while _exclusive_pending(stops.exclusive):
        if waited >= BENCHMARK_EXCLUSIVE_WAIT_S:
            return f"an exclusive job was still pending after {waited:g} s"
        if waited == 0.0:
            emit(f"{label}: an exclusive job is pending; waiting for it (up to {BENCHMARK_EXCLUSIVE_WAIT_S:g} s)")
        sleep(BENCHMARK_POLL_S)
        waited += BENCHMARK_POLL_S
    return None


def _benchmark_gate(model_id: str, config: HarnessConfig, stops: BenchmarkStops, *,
                    liveness: Callable[[HarnessConfig], dict[str, Any]],
                    precondition: Callable[[HarnessConfig], dict[str, Any]], sleep: Callable[[float], None],
                    emit: Callable[[str], None]) -> str | None:
    """Before each live model: the freeze file, a pending exclusive job, the guest's liveness (checked again once
    after BENCHMARK_RETRY_WAIT_S) and the no-foreign-instance precondition. None when the model may start, else why
    the run stops."""
    held = _stops_hold(stops, model_id, sleep=sleep, emit=emit)
    if held is not None:
        return held
    alive = liveness(config)
    if alive["status"] != "alive":
        emit(f"{model_id}: liveness {alive['status']}; {alive.get('reason', '')}; "
             f"checking again in {BENCHMARK_RETRY_WAIT_S:g} s")
        sleep(BENCHMARK_RETRY_WAIT_S)
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
                  sleep: Callable[[float], None] | None = None, collect_only: bool = False) -> int:
    """Run the models one at a time, in serial batches; the exit status of main.

    A live run paces the models `pace_s` apart, passes `_benchmark_gate` before each attempt, and retries a model
    once, BENCHMARK_RETRY_WAIT_S later, when its round trip raised or left no bound CSV with a clean settings check.
    A model that still left no CSV is recorded as such and the run goes on; a round trip that raised twice, a gate
    that fails, an unclean settings check, an unresolved start, or a second attempted run that did not agree (partial or contradicted) in this run stops the
    run. A partial start (`partial`) and a start established as never made (`not_started`) are never retried.
    The GUI fallback is on only when the settings guard restores the whole key, whose restore covers the settings that route writes (Design §5)."""
    windows = [entry for entry in models if _window_run(entry)]
    if windows:
        for entry in windows:
            window_refusal(entry, emit)
        return 2
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
    # A collect-only run never prints through the program's window.
    gui_fallback = bool(not collect_only and config.settings_guard is not None
                        and config.settings_guard.restores_whole_key)
    results, started, partials = [], False, 0
    for number, members in chosen:
        emit(f"batch {number}/{len(batches)}: {', '.join(benchmark_run_id(entry) for entry in members)}")
        for entry in members:
            model_id = benchmark_run_id(entry)
            status, route, reason = NOT_RUN, "not_run", ""
            try:
                for attempt in (1, 2):
                    if not offline:
                        if started:
                            sleep(pace_s if attempt == 1 else BENCHMARK_RETRY_WAIT_S)
                        stop = _benchmark_gate(model_id, config, stops, liveness=alive, precondition=check,
                                               sleep=sleep, emit=emit)
                        if stop is not None:
                            emit(f"stopped before {model_id}: {stop}")
                            return 1
                        started = True
                    status, route, reason = run_benchmark_model(entry, config, runner=runner, offline=offline,
                                                                emit=emit, round_trip=round_trip,
                                                                gui_fallback=gui_fallback, attempt=attempt,
                                                                collect_only=collect_only)
                    if status != RETRY:
                        break
                    if attempt == 1:
                        emit(f"{model_id}: route={route}; {reason}; retrying once in {BENCHMARK_RETRY_WAIT_S:g} s")
            except RunStopped as stopped:
                emit(f"stopped after {stopped}: the guarded round trip did not end cleanly; no further model ran")
                return 1
            if status == PARTIAL:  # an attempted run that does not agree: start_partial or report_reply_contradiction
                partials += 1
                if partials > 1:
                    emit(f"stopped after {model_id}: a second attempted run that did not agree in this run (partial "
                         "or contradicted); no further model ran")
                    return 1
            if status == RETRY:
                if route == "round_trip_raised":
                    emit(f"stopped after {model_id}: the round trip raised again after one retry; no further model ran")
                    return 1
                emit(f"{model_id}: route={route}; {reason}; no benchmark record after one retry")
            results.append(status in ((COLLECTED,) if collect_only else (RECORDED, CAPTURED)))
    return 0 if all(results) else 1


def live_session(config: HarnessConfig, emit: Callable[[str], None] = print) -> HarnessConfig | None:
    """Open a live run (`open_live_session`) before its first guest step and before anything is staged: the guest's
    liveness, asked again a stated number of times, then the guest's own drive letter for the exchange share, which
    the run uses in place of the configured one (in memory; the configuration file is not changed). The configuration
    the run uses, or None when it stops, with the reason emitted."""
    session, record = open_live_session(config, liveness=check_guest_alive, share=exchange_share_letter)
    if session is None:
        emit(f"stopped before the first guest step: {record['status']}; {record.get('reason', '')}")
        return None
    share = session.exchange_share
    if share["differs_from_configured"]:
        emit(f"exchange share {share['remote']} is at {share['letter']} in the guest, not the configured "
             f"{share['configured_root']}; this run uses {share['letter']} (the configuration file is unchanged)")
    return session


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
    parser.add_argument("--series", metavar="NAME",
                        help="with --benchmark, run each model's declared run of this series (required for a model "
                             "that declares runs)")
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
    parser.add_argument("--collect-only", action="store_true",
                        help="with a live --benchmark run, stop each model once its CAEPIPE print is collected and "
                             "bound: nothing is solved, parsed or compared (compare it later with --offline)")
    args = parser.parse_args(argv)
    benchmark_only = (args.batch is not None or args.batch_size != BENCHMARK_BATCH_SIZE or args.freeze_file
                      or args.exclusive_file or args.pace_s != BENCHMARK_PACE_S or args.series is not None
                      or args.collect_only)
    if args.benchmark and (args.authored or args.authored_dir or args.no_export or args.export_mbf
                           or args.export_only):
        parser.error("--benchmark takes only --case, --series, --offline, --batch, --batch-size, --freeze-file, "
                     "--exclusive-file, --pace-s, --collect-only and --runner")
    if not args.benchmark and benchmark_only:
        parser.error("--series, --batch, --batch-size, --freeze-file, --exclusive-file, --pace-s and --collect-only "
                     "need --benchmark")
    if args.collect_only and args.offline:
        parser.error("--collect-only is a live run; offline there is nothing to collect")
    if args.batch_size < 1:
        parser.error("--batch-size must be at least 1")
    if not (args.pace_s >= 0.0):
        parser.error("--pace-s must be a non-negative number")
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
        config = live_session(config)
        if config is None:
            return 1
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
        config = live_session(config)
        if config is None:
            return 1
        return 0 if all([export_case(case, config) for case in cases]) else 1
    runner = args.runner or environ.get(RUNNER_ENV)
    if not runner or not Path(runner).is_file():
        print(f"error: no runner executable (--runner or ${RUNNER_ENV})", file=sys.stderr)
        return 2
    for warning in config.warnings:
        print(f"warning: {warning}")
    if not args.offline:
        config = live_session(config)
        if config is None:
            return 1
    hand_calcs = load_hand_calcs()
    results = run_cases(cases, config, runner=runner, offline=args.offline, hand_calcs=hand_calcs,
                        export_mbf=args.export_mbf)
    return 0 if all(results.values()) else 1


def benchmark_main(args: argparse.Namespace, environ: Mapping[str, str]) -> int:
    try:
        models = load_benchmark(Path(args.benchmark), args.cases, args.series)
    except (KeyError, OSError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 2
    windows = [entry for entry in models if _window_run(entry)]
    for entry in windows:
        window_refusal(entry, print)
    if windows:
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
    if stops is not None:
        # Before the session's first guest call: the batch argument, then the host's stop files, so no guest call is
        # made for an out-of-range batch or while a stop is in force (an exclusive measurement window included).
        batches = benchmark_batches(models, args.batch_size)
        if args.batch is not None and not 1 <= args.batch <= len(batches):
            print(f"error: --batch {args.batch} is not one of the {len(batches)} batches")
            return 2
        held = _stops_hold(stops, "before the session", sleep=time.sleep, emit=print)
        if held is not None:
            print(f"stopped before the first guest step: {held}")
            return 1
        config = live_session(config)
        if config is None:
            return 1
    return run_benchmark(models, config, runner=runner, offline=args.offline, batch_size=args.batch_size,
                         batch=args.batch, stops=stops, pace_s=args.pace_s, collect_only=args.collect_only)


if __name__ == "__main__":
    raise SystemExit(main())
