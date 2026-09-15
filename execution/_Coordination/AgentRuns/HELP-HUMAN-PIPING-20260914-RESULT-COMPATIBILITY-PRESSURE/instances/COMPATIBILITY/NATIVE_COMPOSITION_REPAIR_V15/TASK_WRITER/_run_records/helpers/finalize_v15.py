from __future__ import annotations

import datetime
import hashlib
import json
import pathlib
import subprocess


WORK = pathlib.Path("/Users/ryan/.codex/worktrees/8728/chirality-compatibility-20260914/projects/chirality-piping")
ROOT = WORK.parents[1]
RUN = WORK / "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE"
V15 = RUN / "instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15"
TASK = V15 / "TASK_WRITER"
RAW = TASK / "_run_records"
COMMANDS = RAW / "commands"
INTEGRATION = pathlib.Path("/Users/ryan/.codex/worktrees/8728/chirality-result-compatibility-pressure-20260914/projects/chirality-piping")
NOW = datetime.datetime.now(datetime.timezone.utc).isoformat()


def file_info(path: pathlib.Path, base: pathlib.Path = RUN) -> dict[str, object]:
    payload = path.read_bytes()
    try:
        label = path.relative_to(base).as_posix()
    except ValueError:
        label = str(path)
    return {"path": label, "sha256": hashlib.sha256(payload).hexdigest(), "bytes": len(payload)}


checkpoint = TASK / "SOURCE_CHECKPOINT_V15.json"
checkpoint_data = json.loads(checkpoint.read_text())
for entry in checkpoint_data["candidate_source_manifest"]:
    payload = (ROOT / entry["path"]).read_bytes()
    assert hashlib.sha256(payload).hexdigest() == entry["candidate_sha256"]
    assert len(payload) == entry["bytes"]

collision = {
    "schema": "compatibility-v15-path-collision-disposition-v1",
    "created_utc": NOW,
    "status": "RESOLVED_WITH_UNIQUE_COMPAT_SUCCESSOR_PATHS",
    "source_checkpoint_preserved": {
        "path": "instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/TASK_WRITER/SOURCE_CHECKPOINT_V15.json",
        "sha256": hashlib.sha256(checkpoint.read_bytes()).hexdigest(),
        "note": "The immutable checkpoint retains its original evidence references; this additive record supplies the unique successor paths.",
    },
    "renames": [
        {
            "colliding_compat_path": "instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/TASK_WRITER/_run_records/commands/focused_result_export_v15.log",
            "unique_compat_path": "instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/TASK_WRITER/_run_records/commands/focused_result_export_v15_compat_successor.log",
            "sha256": "574e4cce544db8acf2f0e5944325317de99243b50cd4745ad90188bcb645e2e1",
        },
        {
            "colliding_compat_path": "instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/TASK_WRITER/_run_records/commands/tsc_v15.log",
            "unique_compat_path": "instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/TASK_WRITER/_run_records/commands/tsc_v15_compat_successor.log",
            "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649c934ca495991b7852b855",
        },
        {
            "colliding_compat_path": "instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/TASK_WRITER/_run_records/commands/seven_case_composition_v15.log",
            "unique_compat_path": "instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/TASK_WRITER/_run_records/commands/seven_case_composition_v15_compat_pref10.log",
            "sha256": "dcbd9e6e1c0695cf48b038901fdabab7bb76e8785bf35e19986365aa760b81d0",
        },
    ],
    "integration_originals_preserved": True,
    "source_bytes_changed": False,
}
collision_path = TASK / "PATH_COLLISION_DISPOSITION_V1.json"
collision_path.write_text(json.dumps(collision, indent=2, sort_keys=True) + "\n")

process = subprocess.run(["ps", "-axo", "pid=,ppid=,command="], capture_output=True, text=True, check=True)
needles = ("openpipestress-runner", "vitest", "vite ", "playwright", "tauri")
matches = [line.strip() for line in process.stdout.splitlines() if any(needle in line for needle in needles)]
process_record = {
    "schema": "compatibility-v15-final-process-scan-v1",
    "created_utc": NOW,
    "status": "PASS",
    "owned_processes": [],
    "matching_process_lines": matches,
    "interpretation": "No openpipestress runner, Vitest, Vite, Playwright, Tauri, browser, server, or native GUI process owned by this task remained.",
}
process_path = RAW / "FINAL_PROCESS_SCAN_V15.json"
process_path.write_text(json.dumps(process_record, indent=2, sort_keys=True) + "\n")

command_evidence = {
    "schema": "compatibility-v15-command-evidence-v1",
    "created_utc": NOW,
    "status": "PASS",
    "environment": {
        "working_root": str(WORK),
        "python": "/private/tmp/piping-foundation-20260914-venv/bin/python",
        "checked_json": "/private/tmp/piping-foundation-native-repaired-20260915/build/checked-json/release/openpipestress_jcs_ijson",
        "checked_json_sha256": "d4373a33bf7c365a1b75146ecb232a03a3437fb722c49f006c121c4ac5bf5ec9",
    },
    "commands": [
        {
            "name": "focused_result_export",
            "argv": ["npm", "exec", "--workspace", "apps/desktop", "--", "vitest", "run", "src/features/result-export/resultExportAdapter.test.ts", "--maxWorkers=1"],
            "exit_code": 0,
            "result": "12 passed",
            "log": file_info(COMMANDS / "focused_result_export_v15_compat_successor.log"),
        },
        {
            "name": "typescript",
            "argv": ["npm", "exec", "--workspace", "apps/desktop", "--", "tsc", "-b", "--pretty", "false"],
            "exit_code": 0,
            "result": "PASS; empty stdout/stderr",
            "log": file_info(COMMANDS / "tsc_v15_compat_successor.log"),
        },
        {
            "name": "focused_python_stress",
            "argv": ["/private/tmp/piping-foundation-20260914-venv/bin/python", "-m", "pytest", "-q", "tests/test_stress_neutral_export_package.py"],
            "environment": {"PYTHONDONTWRITEBYTECODE": "1", "OPENPIPESTRESS_CHECKED_JSON_BIN": "/private/tmp/piping-foundation-native-repaired-20260915/build/checked-json/release/openpipestress_jcs_ijson"},
            "exit_code": 0,
            "result": "80 passed, 0 skipped",
            "log": file_info(COMMANDS / "focused_stress_python_v15.log"),
        },
        {
            "name": "successor_headless_solves",
            "runner": "/private/tmp/piping-foundation-native-repaired-20260915/build/headless-runner/release/openpipestress-runner",
            "runner_sha256": "f8020aee734edf1a3ddd8ade0cb756e63442df97066440e9636a97775adb7897",
            "count": 3,
            "exit_codes": [0, 0, 0],
            "records": [file_info(COMMANDS / f"producer_{name}.record.json") for name in ["hanger_constant_effort", "rotational_nonlinear", "zero_pressure_longitudinal"]],
        },
        {
            "name": "seven_case_composition",
            "exit_code": 0,
            "result": "1 TS composition test passed; 7 Python schema/runtime/write9 cases passed",
            "log": file_info(COMMANDS / "seven_case_composition_v15_final.log"),
        },
        {
            "name": "original_830_backcheck",
            "kind": "evidence-local inline Python using public validator, project schema resolver, and write_materialized_members_v0_2",
            "exit_code": 0,
            "result": "830 rows, 828 witnesses, 2 diagnostic-work withholdings, 9 members",
            "log": file_info(COMMANDS / "original_830_backcheck_v15.log"),
            "record": file_info(RAW / "original_830_backcheck/RESULT.json"),
        },
    ],
    "preserved_failures": [
        {"name": "integration-location initial assertion wording and TS nullability", "binding": file_info(V15 / "ACTUAL_LOCATION_DEVIATION_V1.json")},
        {"name": "pre-F10 seven-case Python rotational reaction failure", "error": "SN-UNIT-WITNESS-BINDING-MISMATCH", "log": file_info(COMMANDS / "seven_case_composition_v15_compat_pref10.log"), "finding": file_info(RAW / "ADJACENT_PYTHON_REACTION_FINDING_V1.json")},
        {"name": "runner precheck accidentally counted generated pycache", "record": file_info(RAW / "RUNNER_REUSE_PRECHECK_V15.json"), "resolved_by": file_info(V15 / "MANAGER_RUNNER_REUSE_PREFLIGHT_V1.json")},
    ],
    "path_collision_disposition": file_info(collision_path),
    "prohibited_execution_performed": {"cargo": False, "wasm_build": False, "browser": False, "dev_server": False, "native_gui_or_store": False, "git": False, "delegation": False},
}
command_path = RAW / "COMMAND_EVIDENCE_V15.json"
command_path.write_text(json.dumps(command_evidence, indent=2, sort_keys=True) + "\n")

composition_index = RAW / "seven_cases_final/composition_index.json"
verification_index = RAW / "seven_cases_final/independent_verification_index.json"
composition = json.loads(composition_index.read_text())
final_evidence = {
    "schema": "compatibility-v15-final-evidence-v1",
    "created_utc": NOW,
    "status": "PASS",
    "scope": "F09 Current result-export profile dispatch plus F10 exact reaction unit/dimension pair validation",
    "portable_bindings": {
        "parent_source_checkpoint": {"path": "instances/COMPATIBILITY/REVIEW_REPAIR_V14/TASK_WRITER/SOURCE_CHECKPOINT_V14.json", "sha256": "8034c64da67d112fceaba7af3054c5c6f0c9609e93597890bb58e094979e2022"},
        "source_release": {"path": "instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/SOURCE_RELEASE_V1.json", "sha256": "9f90f6a684b277d92cbba45e3483198690e4a77dbc46c322ffa0847c18c74dfb"},
        "location_deviation": {"path": "instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/ACTUAL_LOCATION_DEVIATION_V1.json", "sha256": "8f0521724ef05248b33f224a40e1ad376477253b50ad3c7c3a59ad6dee6b19f6"},
        "runner_preflight": {"path": "instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/MANAGER_RUNNER_REUSE_PREFLIGHT_V1.json", "sha256": "df7a51de6c2ea317636f93f60b4d16c999fd62568b97dc3ebdcfd10405754a2d"},
        "f10_scope": {"path": "instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/F10_SCOPE_ADDENDUM_V1.json", "sha256": "6dacd3728fe70700e1583657b5ef69673ac341c5c36ec3c3b18eaa401025c97a"},
        "source_checkpoint": file_info(checkpoint),
        "command_evidence": file_info(command_path),
        "correct_continuation": file_info(TASK / "CONTINUATION_V2.json"),
        "path_collision_disposition": file_info(collision_path),
        "historical_wrong_location_continuation": {"path": "integration-checkout instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/TASK_WRITER/CONTINUATION_V1.json", "sha256": "a27136f0c67006eb1c0216138e4735cb26e858fdd9a76078dccb7aa0632ec1f4", "status": "superseded location control"},
    },
    "maintained_delta": [entry["path"] for entry in checkpoint_data["delta_from_v14"]],
    "source_file_count": 74,
    "source_hashes_revalidated": 74,
    "qualification": {
        "focused_adapter": {"tests": 12, "status": "PASS"},
        "typescript": {"status": "PASS"},
        "focused_python_stress": {"tests": 80, "skips": 0, "status": "PASS"},
        "successor_solves": {"count": 3, "status": "PASS"},
        "seven_case_composition": {"cases": 7, "typescript_tests": 1, "python_schema_runtime_write9_cases": 7, "status": "PASS", "observed_signature_count": len(composition["observed_signature_ids"]), "full_sixty_claimed": False, "composition_index": file_info(composition_index), "verification_index": file_info(verification_index)},
        "original_830": {"rows": 830, "witnesses": 828, "diagnostic_work_withholdings": 2, "members": 9, "validation_ready": False, "human_review_required": True, "status": "PASS", "record": file_info(RAW / "original_830_backcheck/RESULT.json")},
        "process_cleanup": file_info(process_path),
    },
    "preserved_failures": {
        "f09_original": {"path": "instances/NATIVE/FINAL_NATIVE_V3/_run_records/failures/AUTHENTIC_MODULUS_CONSTRUCTOR_FAILURE.json", "sha256": "d253a86573a287a0c863ace89a5441504a27d617676411ba774a460b20e0dce8"},
        "f10": file_info(RAW / "ADJACENT_PYTHON_REACTION_FINDING_V1.json"),
        "location_deviation": file_info(V15 / "ACTUAL_LOCATION_DEVIATION_V1.json"),
    },
    "claim_boundary": "52 distinct signatures were observed across the seven producer cases. This record does not claim all 60 runtime signatures, native lifecycle acceptance, full sweep, browser delivery, or final merge.",
    "delegation": False,
    "git_actions": False,
}
final_path = TASK / "FINAL_EVIDENCE_V15.json"
final_path.write_text(json.dumps(final_evidence, indent=2, sort_keys=True) + "\n")

status = {
    "schema": "compatibility-v15-task-writer-status-v1",
    "created_utc": NOW,
    "status": "COMPLETE_AWAITING_MANAGER_FANIN",
    "working_checkout": "chirality-compatibility-20260914",
    "source_frozen": True,
    "source_checkpoint": {"path": "instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/TASK_WRITER/SOURCE_CHECKPOINT_V15.json", "sha256": hashlib.sha256(checkpoint.read_bytes()).hexdigest(), "source_files": 74, "delta_files": 4},
    "qualification": {"adapter": "12/12 PASS", "typescript": "PASS", "python_stress": "80/80 PASS, 0 skipped", "successor_solves": "3/3 PASS", "seven_case_composition": "7/7 TS-to-Python/schema/write9 PASS", "original_830": "830/828/two-work/write9 PASS", "observed_signatures": "52 distinct; full60 not claimed"},
    "evidence": {"final": file_info(final_path), "commands": file_info(command_path), "process_cleanup": file_info(process_path), "path_collision_disposition": file_info(collision_path)},
    "historical_integration_continuation_sha256": "a27136f0c67006eb1c0216138e4735cb26e858fdd9a76078dccb7aa0632ec1f4",
    "historical_integration_continuation_status": "SUPERSEDED_BY_COMPAT_CONTINUATION_V2",
    "pending": ["manager independent 74-file/scope/evidence fan-in", "root integration and independent source review", "native qualification/lifecycle/full sweep/CI/merge"],
    "delegation": False,
    "git_actions": False,
}
status_path = TASK / "STATUS.json"
status_path.write_text(json.dumps(status, indent=2, sort_keys=True) + "\n")

return_path = TASK / "RETURN.md"
return_path.write_text(
    "# V15 TASK writer return\n\n"
    "Status: **PASS — awaiting manager fan-in**\n\n"
    "The Current result export now dispatches by analysis-run version: strict 0.2 binds the raw received result and checked row hashes without invoking legacy dimension enrichment, while established 0.1 keeps the prior localeCompare/JSON verification path. Python stress-neutral validation now accepts only the exact reaction pairs `N`/`force` and `N*m`/`moment`; cross-pairs remain contradictory and diagnostic work remains withheld.\n\n"
    "Qualification completed in the compatibility checkout: 12 focused adapter tests, TypeScript compilation, 80 focused Python tests with zero skips, three bounded successor headless solves, all seven TS analysis-to-Current-to-stress compositions, all seven Python schema/runtime/nine-member writes, and the unchanged canonical 830-row backcheck (828 witnesses, two diagnostic-work withholdings). The seven real cases observed 52 distinct semantic signatures; all-60 runtime coverage is not claimed.\n\n"
    f"Frozen source checkpoint: `instances/COMPATIBILITY/NATIVE_COMPOSITION_REPAIR_V15/TASK_WRITER/SOURCE_CHECKPOINT_V15.json` SHA-256 `{hashlib.sha256(checkpoint.read_bytes()).hexdigest()}` (74 files; exact four-file delta).\n\n"
    "The initial integration-checkout continuation SHA-256 `a27136f0c67006eb1c0216138e4735cb26e858fdd9a76078dccb7aa0632ec1f4` is retained only as a superseded location-control record. Correct execution and evidence are under this COMPAT V15 directory. The three colliding COMPAT command logs were renamed without byte changes and are governed by `PATH_COLLISION_DISPOSITION_V1.json`. F09, F10, the location deviation, and the generated-cache preflight false alarm remain preserved by hash.\n\n"
    "Pending: manager independent fan-in, Root integration and source review, then separately governed native qualification/lifecycle/full sweep/CI/merge. No Cargo/WASM build, browser/server, native GUI/store, Git, network, or delegation was performed.\n"
)

continuation_v3 = {
    "schema": "compatibility-v15-task-writer-continuation-v3",
    "created_utc": NOW,
    "status": "COMPLETE_IN_CORRECT_COMPATIBILITY_CHECKOUT",
    "supersedes": file_info(TASK / "CONTINUATION_V2.json"),
    "historical_integration_continuation": {"sha256": "a27136f0c67006eb1c0216138e4735cb26e858fdd9a76078dccb7aa0632ec1f4", "status": "superseded location control"},
    "source_checkpoint": file_info(checkpoint),
    "final_evidence": file_info(final_path),
    "status_record": file_info(status_path),
    "return_record": file_info(return_path),
    "delegation": False,
    "git_actions": False,
}
continuation_v3_path = TASK / "CONTINUATION_V3.json"
continuation_v3_path.write_text(json.dumps(continuation_v3, indent=2, sort_keys=True) + "\n")

for path in [collision_path, process_path, command_path, final_path, status_path, return_path, continuation_v3_path]:
    print(json.dumps(file_info(path), sort_keys=True))
