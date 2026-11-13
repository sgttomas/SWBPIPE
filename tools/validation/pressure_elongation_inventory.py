"""Inventory of committed pressure model inputs for the pressure-elongation switch (DEL-04-07).

Scans every Git-tracked JSON file for model documents under a `pressure_contract` with at
least one pressurised load case (`pressure_elongation.pressurised_case_ids`), and joins each
file to the curated GROUPS below: its maintained producer (and whether the producer takes
the switch), the readers and tests that pin its outputs, and the anchor route the harness
(`pressure_elongation_anchor.py`) uses. The result, `pressure_elongation_inventory.json`, is
also the re-production checklist for the fixture rebuild.

    python3 tools/validation/pressure_elongation_inventory.py           # write the inventory
    python3 tools/validation/pressure_elongation_inventory.py --check   # compare only

A scanned file that no group claims, or a group pattern that claims nothing, is an error:
a new pressure fixture must be classified here before the inventory is current.
"""
from __future__ import annotations

import argparse
import fnmatch
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))
from tools.validation import pressure_elongation as PE  # noqa: E402

OUTPUT = ROOT / "tools/validation/pressure_elongation_inventory.json"
SWITCH = "--pressure-elongation {absent,all,none}"


def _py(path: str, verify: str, write: str) -> dict:
    return {"kind": "python_generator", "path": path, "verify": verify, "write": write, "switch": SWITCH,
            "switch_plumbed": True}


def _writer(kind: str, path: str, write: str) -> dict:
    return {"kind": kind, "path": path, "write": write, "switch_plumbed": False,
            "switch_route": "copies its model from a committed source: re-produce that source with the switch, "
                            "then re-run this writer"}


HAND = {"kind": "none", "status": "hand_authored", "switch_plumbed": False,
        "switch_route": "no producer: the fixture rebuild edits the document (the helper's apply) and its "
                        "committed outputs are re-produced by their writers"}
FROZEN_RUN = {"kind": "none", "status": "frozen_run_record", "switch_plumbed": False,
              "switch_route": "frozen reference; its _run_records generator is a historical record, not a "
                              "maintained producer; the reading tests re-author a copy in memory"}
BASELINE = {"kind": "baseline"}

# Each group: id, path patterns (fnmatch on the repository path), producer, pinned_by, anchor.
GROUPS = [
    # ---- maintained Python producers (switch plumbed) -----------------------------------------
    {"id": "t4_u4a", "paths": ["validation/references/t4_u4a/requests.json"],
     "producer": _py("validation/references/t4_u4a/u4a_generate.py",
                     "cd validation/references/t4_u4a && python3 -I -B u4a_generate.py --pressure-elongation all",
                     "cd validation/references/t4_u4a && python3 -I -B u4a_generate.py --write --pressure-elongation all"),
     "pinned_by": ["validation/references/t4_u4a/oracle.json (requests_sha256)", "validation/references/t4_u4a/SHA256SUMS",
                   "core/product_physics/src/u4a_tests.rs", "core/product_physics/src/u4b_tests.rs",
                   "core/product_physics/tests/preview_physics_runtime.rs"],
     "anchor": BASELINE},
    {"id": "t4_u5a", "paths": ["validation/references/t4_u5a/requests/*.request.json"],
     "producer": _py("validation/references/t4_u5a/generators/u5a_reference.py",
                     "cd validation/references/t4_u5a/generators && python3 -I -B u5a_reference.py --pressure-elongation all",
                     "cd validation/references/t4_u5a/generators && python3 -I -B u5a_reference.py --write --pressure-elongation all"),
     "pinned_by": ["validation/references/t4_u5a/oracle.json (request sha256)", "validation/references/t4_u5a/SHA256SUMS",
                   "core/runner/headless/tests/untied_joint_pressure_references.rs",
                   "core/product_physics/src/b7b_admission_tests.rs", "core/product_physics/tests/common/t4_u5b.rs"],
     "anchor": BASELINE},
    {"id": "t4_u6", "paths": ["validation/references/t4_u6/requests/*.request.json"],
     "producer": _py("validation/references/t4_u6/generators/u6_reference.py",
                     "cd validation/references/t4_u6 && python3 -I -B generators/u6_reference.py --pressure-elongation all",
                     "cd validation/references/t4_u6 && python3 -I -B generators/u6_reference.py --write --pressure-elongation all"),
     "pinned_by": ["validation/references/t4_u6/oracle.json (request sha256)", "validation/references/t4_u6/SHA256SUMS",
                   "core/product_physics/tests/t4_u6_section_bases.rs", "core/product_physics/src/timoshenko_product_tests.rs"],
     "anchor": BASELINE},
    {"id": "t4_u8", "paths": ["validation/references/t4_u8/requests/*.request.json"],
     "producer": _py("validation/references/t4_u8/generators/u8_reference.py",
                     "cd validation/references/t4_u8/generators && python3 -I -B u8_reference.py --pressure-elongation all",
                     "cd validation/references/t4_u8/generators && python3 -I -B u8_reference.py --write --pressure-elongation all"),
     "pinned_by": ["validation/references/t4_u8/product_cases.json (request sha256)", "validation/references/t4_u8/SHA256SUMS",
                   "core/product_physics/tests/t4_u8_timoshenko_references.rs"],
     "anchor": BASELINE},
    {"id": "rt_a3_s6", "paths": ["validation/references/rt_a3_s6/requests/*.json"],
     "producer": _py("validation/references/rt_a3_s6/generators/rt_a3_s6_reference.py",
                     "cd validation/references/rt_a3_s6 && python3 -I -B generators/rt_a3_s6_reference.py --pressure-elongation all",
                     "cd validation/references/rt_a3_s6 && python3 -I -B generators/rt_a3_s6_reference.py --write --pressure-elongation all"),
     "pinned_by": ["validation/references/rt_a3_s6/references.json (request_sha256)", "validation/references/rt_a3_s6/SHA256SUMS",
                   "core/product_physics/src/rt_a3_s6_tests.rs", "core/product_physics/tests/rt_a3_s6_reference_system.rs",
                   "core/product_physics/tests/exact_admission_seam.rs", "core/product_physics/src/modal/tests_state.rs",
                   "tests/test_rt_a3_nonlinear_reader.py"],
     "anchor": BASELINE},
    {"id": "reference_system", "paths": ["fixtures/reference_system/reference_system.json"],
     "producer": _py("fixtures/reference_system/generate_reference_system.py",
                     "python3 fixtures/reference_system/generate_reference_system.py --check --pressure-elongation all",
                     "python3 fixtures/reference_system/generate_reference_system.py --pressure-elongation all"),
     "pinned_by": ["tests/test_reference_system_fixture.py", "tools/validation/test_reference_system_ledger.py",
                   "core/product_physics/tests/rt_a3_s6_reference_system.rs",
                   "core/product_physics/tests/rt_a3_reducer_2b_reference_system.rs"],
     "anchor": BASELINE},
    # The CAEPIPE benchmark set: its producer writes each model's default series without the setting by design (the
    # pressure-elongation-on series adds it in memory, naming the case that carries pressure), so its switch is not
    # plumbed; the anchor compares `--switch all` with the pre-change engine's solve of the committed document.
    {"id": "caepipe_benchmark", "paths": ["validation/caepipe_benchmark/models/cpb-p*.json"],
     "producer": {"kind": "python_generator", "path": "tools/caepipe_correlation/benchmark_models.py",
                  "verify": "python3 -m tools.caepipe_correlation.benchmark_models --check",
                  "write": "python3 -m tools.caepipe_correlation.benchmark_models", "switch_plumbed": False,
                  "switch_route": "the tracked model is the default series, without model_settings.pressure_elongation "
                                  "by design; the pressure-elongation-on series names the case that carries pressure "
                                  "in memory"},
     "pinned_by": ["validation/caepipe_benchmark/manifest.json", "tests/test_caepipe_benchmark.py",
                   "core/handoff/caepipe_external/benchmark.py", "tools/caepipe_correlation/run_pilot.py"],
     "anchor": BASELINE},
    # ---- Rust and TypeScript writers (switch not plumbed; they copy committed sources) --------
    {"id": "t4_u5b_requests", "paths": ["core/runner/headless/tests/t4_u5b_requests/*.request.json"],
     "producer": _writer("rust_builder", "core/product_physics/tests/common/t4_u5b.rs",
                         "the shared builder from validation/references/t4_u5b/cases and the t4_u5a base requests; "
                         "core/runner/headless/tests/t4_u5b_requests/bindings.json binds the request sha256"),
     "pinned_by": ["core/runner/headless/tests/t4_u5b_requests/bindings.json", "core/runner/headless/tests/tied_joint_references.rs",
                   "core/product_physics/tests/connector_readers.rs", "fixtures/results/pressure_v3_connector_reader_corpus.json"],
     "anchor": BASELINE},
    # The cancellation-screen exception's halfway-force case: the test builds its model in code without the setting by
    # design (the exception's family excludes pressure elongation), so its committed envelope is the not-included
    # mechanics; the anchor compares `--switch all` with the pre-change engine's solve of the committed request.
    {"id": "cap_remainder_halfway_force", "paths": ["fixtures/results/pressure_v3_cap_remainder_halfway_force.json"],
     "producer": {"kind": "rust_test", "path": "core/product_physics/tests/pressure_grouping_limits.rs",
                  "write": "SWBPIPE_WRITE_CAP_REMAINDER_FIXTURE=1 with "
                           "a_force_halfway_between_two_shortest_decimals_is_read_in_either_spelling",
                  "switch_plumbed": False,
                  "switch_route": "the model is built in code without model_settings.pressure_elongation by design"},
     "pinned_by": ["core/product_physics/tests/pressure_grouping_limits.rs",
                   "tests/test_pressure_cancellation_exception_readers.py"],
     "anchor": BASELINE},
    {"id": "arc_reader_corpus", "paths": ["fixtures/results/pressure_v3_arc_reader_corpus.json"],
     "producer": _writer("rust_test", "core/product_physics/tests/pressure_arc_readers.rs",
                         "shared_arc_reader_corpus_is_current (models from validation/references/t4_i7)"),
     "pinned_by": ["tests/test_pressure_arc_readers.py", "tests/test_qualification_exact_pressure.py",
                   "tests/test_section_bases_readers.py", "core/product_physics/tests/exact_route_limitations.rs",
                   "core/reporting/result_export/tests/section_bases.rs", "core/reporting/report_package/tests/exact_sections_parity.rs"],
     "anchor": {"kind": "inline", "case_depth": 2, "envelope": "envelope", "mode_member": "solver_mode"}},
    {"id": "combination_reader_corpus", "paths": ["fixtures/results/pressure_v3_combination_reader_corpus.json"],
     "producer": _writer("rust_test", "core/product_physics/tests/w16_exact_combinations.rs",
                         "shared_combination_reader_corpus_is_current"),
     "pinned_by": ["tests/test_exact_combination_verifier.py", "tests/test_pressure_connector_readers.py",
                   "tests/test_qualification_exact_pressure.py", "tests/base_reader_corpus.py",
                   "core/reporting/result_export/tests/base_reader_outcomes.rs"],
     "anchor": {"kind": "inline", "case_depth": 1, "envelope": "envelope", "mode_member": "solver_mode"}},
    {"id": "combination_certificate_corpus", "paths": ["fixtures/results/combination_certificate_corpus.json"],
     "producer": _writer("rust_test", "core/product_physics/tests/combination_certificate.rs", "CERT_WRITE_CORPUS=<path>"),
     "pinned_by": ["tests/test_combination_certificate.py", "core/reporting/result_export/tests/combination_certificate_contract.rs",
                   "apps/desktop/src-tauri/src/analysis_run_v03_build_tests.rs"],
     "anchor": BASELINE},
    {"id": "arc_tangent_above_one", "paths": ["fixtures/results/pressure_arc_tangent_above_one_sparse.json"],
     "producer": _writer("rust_test", "core/product_physics/tests/arc_tangent_direction_magnitude.rs",
                         "its write branch rewrites the fixture"),
     "pinned_by": ["core/product_physics/tests/arc_tangent_direction_magnitude.rs", "tests/test_pressure_arc_readers.py"],
     "anchor": {"kind": "inline", "case_depth": 1, "envelope": "envelope", "mode": "sparse_interactive"}},
    {"id": "l_line_authoring", "paths": ["fixtures/model_operations/pressure_v3_l_line_authoring.json"],
     "producer": _writer("rust_test", "core/model_operations/operation_applier/tests/l_line_authoring.rs",
                         "the_desktop_l_line_fixture_is_current"),
     "pinned_by": ["core/model_operations/operation_applier/tests/l_line_authoring.rs",
                   "core/runner/headless/tests/pressure_export.rs"],
     "anchor": {"kind": "inline", "case_depth": 1, "envelope": "sparse_result", "mode": "sparse_interactive"}},
    {"id": "report_witness_inputs", "paths": ["fixtures/reports/invented/exact_report_witness_inputs/*.request.json"],
     "producer": _writer("rust_test", "core/product_physics/tests/exact_route_limitations.rs",
                         "exact_report_witness_inputs_are_current (request and its sparse raw)"),
     "pinned_by": ["core/product_physics/tests/exact_route_limitations.rs",
                   "apps/desktop/src/features/report/exactReportBelowGate.test.tsx"],
     "anchor": {"kind": "raw_files", "files_from": "{stem}-{mode}.raw.json"}},
    {"id": "report_package_requests", "paths": ["fixtures/reports/invented/exact_report_package_requests/pressure_2_*.json"],
     "producer": _writer("ts_test", "apps/desktop/src/features/report/exactReportBelowGate.test.tsx",
                         "its write mode, from request_path and raw_path"),
     "pinned_by": ["core/runner/headless/tests/del1005_package_witness.rs", "core/reporting/report_package/tests/user_release_package.rs",
                   "apps/desktop/src-tauri/src/report_render_exact_tests.rs", "tests/security/test_redaction_export_controls.py"],
     "anchor": {"kind": "raw_files", "files_from": "raw_path"}},
    {"id": "user_release_exact", "paths": ["fixtures/reports/invented/user_release_exact/pressure_2_*.json"],
     "producer": _writer("ts_test", "apps/desktop/src/features/report/userReleaseExact.parity.test.ts",
                         "its write mode (model from core/product_physics/tests/fixtures/exact_pressure_connected_request.json)"),
     "pinned_by": ["core/reporting/report_package/tests/user_release_package.rs",
                   "core/reporting/report_package/tests/user_release_derivation.rs", "tests/test_semantic_contract_tables.py"],
     "anchor": {"kind": "raw_files", "files": {"sparse_interactive": "fixtures/results/pressure_connected_mechanics_sparse.json"}}},
    {"id": "desktop_historical_reopen", "paths": ["apps/desktop/src/test-support/native-reopen/historical_reopen_cases.json"],
     "producer": _writer("ts_test", "apps/desktop/src/test-support/historicalReopenCases.ts", "HISTORICAL_REOPEN_CASES_WRITE=1"),
     "pinned_by": ["apps/desktop/src/services/historicalReopenParity.test.ts", "apps/desktop/src-tauri/src/stored_custody_tests.rs",
                   "tests/test_semantic_contract_tables.py"],
     "anchor": BASELINE},
    {"id": "desktop_native_reopen", "paths": ["apps/desktop/src/test-support/native-reopen/native_reopen_re*.json"],
     "producer": _writer("rust_and_ts_tests", "apps/desktop/src-tauri/src/stored_custody_tests.rs",
                         "native_reopen_replies.json by stored_custody_tests.rs; native_reopen_requests.json by "
                         "nativeReopen.session.test.tsx (NATIVE_REOPEN_RECORD=1)"),
     "pinned_by": ["apps/desktop/src/test/nativeReopenEmulation.ts", "apps/desktop/native-smoke/perf.test.mjs",
                   "tools/serialization/semantic_contract_tables.py"],
     "anchor": BASELINE},
    {"id": "desktop_native_replay", "paths": ["apps/desktop/src/test-support/native-replay/native_persistence_replies.json"],
     "producer": _writer("rust_test", "apps/desktop/src-tauri/src/native_persistence_tests.rs", "NATIVE_PERSISTENCE_REPLIES_WRITE=1"),
     "pinned_by": ["apps/desktop/src/test/nativePersistenceReplay.ts", "apps/desktop/src/test/nativePersistenceFake.ts",
                   "apps/desktop/native-smoke/perf.test.mjs"],
     "anchor": BASELINE},
    # ---- hand-authored inputs with committed result envelopes ---------------------------------
    {"id": "exact_pressure_connected", "paths": ["core/product_physics/tests/fixtures/exact_pressure_connected_request.json"],
     "producer": HAND,
     "pinned_by": ["fixtures/results/pressure_connected_mechanics_{sparse,dense}.json (examples/exact_pressure_connected.rs)",
                   "apps/desktop/src-tauri/src/native_verdict_tests.rs", "core/product_physics/tests/exact_admission_seam.rs",
                   "tests/base_reader_corpus.py", "tests/test_analysis_run_compatibility.py"],
     "anchor": {"kind": "raw_files", "example": "exact_pressure_connected",
                "files": {"sparse_interactive": "fixtures/results/pressure_connected_mechanics_sparse.json",
                          "dense_scrutiny": "fixtures/results/pressure_connected_mechanics_dense.json"}}},
    {"id": "exact_pressure_authoring_model", "paths": ["fixtures/model_operations/exact_pressure_authoring_model.json"],
     "producer": HAND,
     "pinned_by": ["fixtures/results/pressure_connected_ui_mechanics_{sparse,dense}.json",
                   "apps/desktop/src-tauri/src/native_verdict_tests.rs", "apps/desktop/native-smoke/run.mjs",
                   "tests/test_headless_runner_contract.py", "tests/base_reader_corpus.py"],
     "anchor": {"kind": "raw_files", "example": "exact_pressure_connected",
                "files": {"sparse_interactive": "fixtures/results/pressure_connected_ui_mechanics_sparse.json",
                          "dense_scrutiny": "fixtures/results/pressure_connected_ui_mechanics_dense.json"}}},
    {"id": "physics_thermal_ui_model", "paths": ["fixtures/model_operations/physics_thermal_ui_model.json"],
     "producer": HAND,
     "pinned_by": ["fixtures/results/pressure_thermal_ui_mechanics_{sparse,dense}.json",
                   "apps/desktop/src-tauri/src/native_verdict_tests.rs", "tests/base_reader_corpus.py"],
     "anchor": {"kind": "raw_files", "example": "exact_pressure_connected",
                "files": {"sparse_interactive": "fixtures/results/pressure_thermal_ui_mechanics_sparse.json",
                          "dense_scrutiny": "fixtures/results/pressure_thermal_ui_mechanics_dense.json"}}},
    {"id": "u4b_l90_p_model", "paths": ["core/product_physics/tests/fixtures/u4b_l90_p_model.json"],
     "producer": HAND,
     "pinned_by": ["fixtures/results/pressure_u4b_arc_sparse.json (exact_pressure_connected -- sparse --model)",
                   "core/product_physics/src/u4b_tests.rs", "tests/test_u4b_review_tampers.py"],
     "anchor": {"kind": "raw_files", "example": "exact_pressure_connected",
                "files": {"sparse_interactive": "fixtures/results/pressure_u4b_arc_sparse.json"}}},
    {"id": "product_preview_pressure_requests",
     "paths": ["fixtures/product_preview/pressure/load_reference/*.request.json",
               "fixtures/product_preview/pressure_source/physics_source/*.request.json",
               "fixtures/product_preview/pressure_source/load_reference_source/*.request.json"],
     "producer": HAND,
     "pinned_by": ["fixtures/product_preview/**/<stem>-{sparse_interactive,dense_scrutiny}.raw.json "
                   "(core/runner/headless load_reference_route_tests.rs, load_reference_cli.rs, tests/physics_source_connected.rs)",
                   "apps/desktop/src-tauri/src/native_verdict_tests.rs", "tests/test_load_reference_readers.py",
                   "tests/test_load_reference_source_readers.py", "tests/test_base_reader_verifier.py"],
     "anchor": {"kind": "raw_files", "files_from": "{stem}-{mode}.raw.json"}},
    {"id": "shared_terminal_junction", "paths": ["core/product_physics/tests/fixtures/shared_terminal_junction.request.json"],
     "producer": HAND,
     "pinned_by": ["core/product_physics/tests/shared_terminal_junction.rs"],
     "anchor": BASELINE},
    # ---- frozen references (no maintained producer) ---------------------------------------------
    {"id": "t4_i7", "paths": ["validation/references/t4_i7/u2_document_sketches.json"],
     "producer": FROZEN_RUN,
     "pinned_by": ["validation/references/t4_i7/SHA256SUMS", "core/product_physics/tests/t4_u2_pressure_references.rs",
                   "core/product_physics/tests/pressure_arc_readers.rs", "tools/validation/qualification_exact_pressure.py"],
     "anchor": BASELINE},
    {"id": "t4_i8", "paths": ["validation/references/t4_i8/rebuilt_reference_cases.json"],
     "producer": FROZEN_RUN,
     "pinned_by": ["validation/references/t4_i8/SHA256SUMS", "core/product_physics/tests/t4_u2_pressure_references.rs",
                   "core/product_physics/tests/t4_u6_section_bases.rs", "core/product_physics/tests/t4_u8_timoshenko_references.rs",
                   "validation/qualification/fixtures/exact_pressure/MANIFEST.json (generate_package.py)"],
     "anchor": BASELINE},
    {"id": "persistence_records", "paths": ["fixtures/persistence/recorded_*/*.json"],
     "producer": {"kind": "none", "status": "frozen_persistence_record", "switch_plumbed": False,
                  "switch_route": "not re-produced (a frozen record)"},
     "pinned_by": ["apps/desktop/src-tauri persistence reopen tests", "tests/test_model_document_schema.py"],
     "anchor": {"kind": "excluded", "reason": "frozen persistence record; left untouched"}},
]

# Consumers that hold no model document of their own but re-derive from the entries above.
DERIVED = [
    {"path": "validation/qualification/fixtures/exact_pressure/MANIFEST.json",
     "producer": "validation/qualification/fixtures/exact_pressure/generate_package.py",
     "from": ["t4_i7", "t4_i8", "t4_u4a", "t4_u4b", "t4_u5b", "t4_u6"],
     "reader": "tools/validation/qualification_exact_pressure.py; tests/test_qualification_exact_pressure.py"},
    {"path": "fixtures/results/semantic_contract_v0_3_pressure_2.json",
     "producer": "tools/serialization/semantic_contract_tables.py",
     "from": ["the pressure-2 identity (pressure-3 successor)"],
     "reader": "tests/test_semantic_contract_tables.py; core/analysis_runs/compatibility.py"},
    {"path": "fixtures/results/pressure_*.{analysis_run,document,stress_neutral}.json",
     "producer": "core/runner/headless/tests/pressure_export.rs, physics_source_connected.rs and the Python builders",
     "from": ["product_preview_pressure_requests", "exact_pressure_connected"],
     "reader": "tests/test_load_reference_readers.py; tests/test_results_schema.py; apps/desktop exactOutputTwins.test.tsx"},
    {"path": "fixtures/results/pressure_v3_connector_reader_corpus.json",
     "producer": "core/product_physics/tests/connector_readers.rs",
     "from": ["t4_u5b_requests"], "reader": "tests/test_pressure_connector_readers.py"},
]

# Committed references whose pressurised product models are built in code, not committed as model
# JSON: the harness cannot reach them; their anchor is the engine test suite run with the setting.
IN_CODE = [
    {"reference": "validation/references/t4_u4b", "built_by": "core/product_physics/src/u4b_tests.rs"},
    {"reference": "validation/references/t4_u5b", "built_by": "core/product_physics/tests/common/t4_u5b.rs (committed as t4_u5b_requests)"},
    {"reference": "validation/references/t4_i12", "built_by": "core/product_physics/src/joint_pressure_tests.rs, f1b_tests.rs, source_recovery.rs"},
    {"reference": "validation/references/w15_rigid_scaling", "built_by": "core/product_physics/tests/w15_rigid_scaling_benchmarks.rs"},
    {"reference": "validation/references/o4_retained_source", "built_by": "core/product_physics/src/o4_m3_judging_tests.rs, retained_facade_tests.rs"},
    {"reference": "validation/references/w16_exact_combinations", "built_by": "core/product_physics/tests/w16_exact_combinations.rs"},
    {"reference": "validation/references/w16_exact_equivalent_static", "built_by": "core/product_physics/tests/w16_exact_equivalent_static.rs"},
    {"reference": "validation/references/combination_certificate", "built_by": "core/product_physics/tests/combination_certificate.rs"},
]


def tracked_json() -> list[str]:
    out = subprocess.run(["git", "grep", "-l", "pressure_contract", "--", "*.json", ":!execution/"], cwd=ROOT,
                         capture_output=True, text=True, check=True).stdout
    return sorted(out.split())


def scan() -> dict[str, list[dict]]:
    """{path: [{pointer, schema_version, pressurised_load_cases}]} over files with a pressurised document."""
    found = {}
    for path in tracked_json():
        container = json.loads((ROOT / path).read_text(encoding="utf-8"))
        documents = [{"pointer": pointer, "schema_version": doc.get("schema_version"),
                      "pressurised_load_cases": PE.pressurised_case_ids(doc)}
                     for pointer, doc in PE.documents(container) if PE.pressurised_case_ids(doc)]
        if documents:
            found[path] = documents
    return found


def _raw_files(anchor: dict, path: str, container) -> tuple[dict | None, dict | None]:
    """(files, per_document) for a raw_files anchor templated by `files_from`."""
    pattern = anchor.get("files_from")
    if pattern is None:
        return anchor.get("files"), None
    if pattern == "raw_path":
        return {container.get("solver_mode", "sparse_interactive"): container["raw_path"]}, None
    folder, name = path.rsplit("/", 1)
    stem = name[: -len(".request.json")]
    files = {}
    for mode in ("sparse_interactive", "dense_scrutiny"):
        candidate = f"{folder}/" + pattern.format(stem=stem, mode=mode)
        if (ROOT / candidate).exists():
            files[mode] = candidate
    return files or None, None


def _suffix(path: str, paths: list[str]) -> str:
    """The path below the group's common directory, without `.json`: a unique, readable entry id."""
    common = Path(*Path(paths[0]).parts[:-1])
    for other in paths[1:]:
        while not str(Path(other)).startswith(str(common) + "/"):
            common = common.parent
    return str(Path(path).relative_to(common)).removesuffix(".json")


def build() -> dict:
    found = scan()
    entries, claimed, unused = [], set(), []
    for group in GROUPS:
        paths = sorted(p for p in found if any(fnmatch.fnmatch(p, pattern) for pattern in group["paths"]))
        if not paths:
            unused.append(group["id"])
        for path in paths:
            if path in claimed:
                raise SystemExit(f"{path} is claimed by two groups")
            claimed.add(path)
            anchor = {k: v for k, v in group["anchor"].items() if k != "files_from"}
            if group["anchor"]["kind"] == "raw_files":
                container = json.loads((ROOT / path).read_text(encoding="utf-8"))
                files, _ = _raw_files(group["anchor"], path, container)
                if files is None:
                    anchor = dict(BASELINE)
                else:
                    anchor["files"] = files
            entries.append({
                "id": group["id"] if len(paths) == 1 else f"{group['id']}/{_suffix(path, paths)}",
                "group": group["id"], "path": path, "producer": group["producer"],
                "pinned_by": group["pinned_by"], "anchor": anchor, "documents": found[path]})
    unclaimed = sorted(set(found) - claimed)
    if unclaimed or unused:
        raise SystemExit("inventory out of date: unclassified " + ", ".join(unclaimed)
                         + ("; groups matching nothing: " + ", ".join(unused) if unused else ""))
    counts: dict[str, dict] = {}
    for entry in entries:
        c = counts.setdefault(entry["group"], {"files": 0, "documents": 0, "pressurised_load_cases": 0})
        c["files"] += 1
        c["documents"] += len(entry["documents"])
        c["pressurised_load_cases"] += sum(len(d["pressurised_load_cases"]) for d in entry["documents"])
    return {
        "document_kind": "swbpipe.pressure_elongation_inventory",
        "version": 1,
        "purpose": "Every committed model input under a pressure_contract with a pressurised load case, by producer and "
                   "pinning reader: the DEL-04-07 regression anchor's scope and the "
                   "fixture re-production checklist. Generated by tools/validation/pressure_elongation_inventory.py.",
        "setting": "model_settings.pressure_elongation.operating_load_cases",
        "switch": {"helper": "tools/validation/pressure_elongation.py", "values": list(PE.MODES), "default": PE.DEFAULT,
                   "environment": PE.ENV},
        "pressurised_case_rule": "a pressure region with a nonzero pressure, or condition_use.pressure resolving to a "
                                 "nonzero condition pressure on a pipe of one of the case's regions",
        "counts": counts,
        "entries": entries,
        "derived_consumers": DERIVED,
        "in_code_constructions": IN_CODE,
    }


def dumps(value) -> str:
    return json.dumps(value, indent=1, ensure_ascii=False) + "\n"


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="compare with the committed inventory; write nothing")
    args = parser.parse_args(argv)
    text = dumps(build())
    if args.check:
        current = OUTPUT.read_text(encoding="utf-8") if OUTPUT.exists() else ""
        if current != text:
            print(f"{OUTPUT.relative_to(ROOT)} is not current; run without --check")
            return 1
        print(f"{OUTPUT.relative_to(ROOT)} is current")
        return 0
    OUTPUT.write_text(text, encoding="utf-8")
    print(f"wrote {OUTPUT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
