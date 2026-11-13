"""Inventory of committed pressure model inputs for the pressure-elongation switch (DEL-04-07).

Scans every Git-tracked file under the scanned roots (`core`, `validation`, `tests`, `tools`, `fixtures`, `apps`;
`execution/` is outside them) whose bytes name `pressure_regions`, and requires each to be claimed exactly once:

- a data file (`.json`, or `.jsonl` read line by line) holding a model document under a `pressure_contract` with a
  pressurised load case (`pressure_elongation.pressurised_case_ids`), by one of the curated GROUPS below: its
  maintained producer (and whether the producer takes the switch), the readers and tests that pin its outputs, and
  the anchor route the harness (`pressure_elongation_anchor.py`) uses. Documents are also read where a file holds
  them as JSON text in a string, or in the retained-precision DAG storage;
- a code file (any other type) that builds a pressurised document which is solved, by an IN_CODE entry stating what
  its assertions rest on and whether its pressurised cases are named;
- any other scanned file (code that builds no solved pressurised document, or data with no pressurised document),
  by an EXEMPT entry with a one-line reason.

The result, `pressure_elongation_inventory.json`, is also the re-production checklist for the fixture rebuild.

    python3 tools/validation/pressure_elongation_inventory.py           # write the inventory
    python3 tools/validation/pressure_elongation_inventory.py --check   # compare only

An unclaimed file, a file claimed twice, a claim that matches no file still naming `pressure_regions` (stale), an
exemption without a reason, or an exemption of a data file that holds a pressurised document is an error: a new
pressure input must be classified here before the inventory is current.
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
from tools.fixtures import retained_precision_corpus as CORPUS  # noqa: E402
from tools.validation import pressure_elongation as PE  # noqa: E402

OUTPUT = ROOT / "tools/validation/pressure_elongation_inventory.json"
SWITCH = "--pressure-elongation {absent,all,none}"
SCAN_ROOTS = ("core", "validation", "tests", "tools", "fixtures", "apps")
TOKEN = "pressure_regions"
DATA_SUFFIXES = (".json", ".jsonl")
# What an IN_CODE entry's assertions rest on, and whether its pressurised cases are named.
EXPECTATIONS = ("closed_form", "reference_file", "product_output", "no_response_value")
CASES_NAMED = ("all", "some", "none")


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
    # ---- data whose documents the scan reads inside other storage (anchor excluded: the harness reads plain JSON) --
    # JSON Lines, one judged model per line. Pressurised (12 models, one nonzero region each) and naming no setting,
    # while the exact rows kd5t_engine.py froze beside each request include the free strain; the M3 route that judges
    # them is not run by any hosted job. Naming the cases belongs to the M3 restoration; nothing is switched on here.
    {"id": "kd5_timoshenko_frozen", "paths": ["validation/references/kd5_timoshenko/frozen/kd5t_frozen_subset.jsonl"],
     "producer": {"kind": "python_generator", "path": "validation/references/kd5_timoshenko/kd5t_generate.py",
                  "verify": "cd validation/references/kd5_timoshenko && python3 -I -B kd5t_generate.py check",
                  "write": "cd validation/references/kd5_timoshenko && python3 -I -B kd5t_generate.py freeze",
                  "status": "awaiting_m3_restoration", "switch_plumbed": False,
                  "switch_route": "the requests name no model_settings.pressure_elongation while their exact rows include "
                                  "the free strain -2 nu p A_i/(E A_n) (kd5t_engine.py); the M3 restoration names the cases"},
     "pinned_by": ["validation/references/m3_judge/REFERENCE_SHA256SUMS", "validation/references/m3_judge/m3_regress.py",
                   "validation/references/m3_judge/m3_reproduce.py", "validation/references/m3_judge/m3_selftest.py",
                   "validation/references/kd5_timoshenko/kd5t_compare.py"],
     "anchor": {"kind": "excluded", "reason": "JSON Lines frozen reference awaiting the M3 restoration"}},
    # Documents held as JSON text inside recorded requests and cases; every pressurised case is named.
    {"id": "desktop_native_replay_requests", "paths": ["apps/desktop/src/test-support/native-replay/native_replay_requests.json"],
     "producer": _writer("ts_test", "apps/desktop/scripts/native-replay-merge.mjs",
                         "NATIVE_REPLAY_RECORD=<dir> over the webview tests (src/test/nativeReplayStore.ts), then the merge"),
     "pinned_by": ["apps/desktop/src-tauri/src/native_replay_tests.rs", "apps/desktop/src/test/nativeReplayStore.ts"],
     "anchor": {"kind": "excluded", "reason": "documents held as JSON text inside recorded requests"}},
    {"id": "desktop_analysis_run_v03_cases", "paths": ["apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json"],
     "producer": _writer("ts_test", "apps/desktop/src/test-support/analysisRunV03Cases.ts", "ANALYSIS_RUN_V03_CASES_WRITE=1 with analysisRunV03Parity.test.ts"),
     "pinned_by": ["apps/desktop/src-tauri/src/analysis_run_v03_build_tests.rs", "apps/desktop/src/services/analysisRunV03Parity.test.ts"],
     "anchor": {"kind": "excluded", "reason": "documents held as JSON text (canonical bytes and intake text)"}},
    # The MBF writer's inputs and the importer's outputs, compared between the Python and Rust ports; nothing solves
    # them, so whether a case is named does not reach a result.
    {"id": "caepipe_mbf_parity_vectors", "paths": ["fixtures/caepipe_mbf/invented/parity/import_vectors.json",
                                                   "fixtures/caepipe_mbf/invented/parity/writer_vectors.json"],
     "producer": {"kind": "python_generator", "path": "tools/fixtures/caepipe_mbf_writer_vectors.py",
                  "write": "python3 tools/fixtures/caepipe_mbf_writer_vectors.py harvest (and tools/fixtures/caepipe_mbf_import_vectors.py harvest)",
                  "switch_plumbed": False, "switch_route": "MBF writer inputs and importer outputs; never solved"},
     "pinned_by": ["tests/test_caepipe_mbf_writer_parity.py", "tests/test_caepipe_mbf_import_parity.py",
                   "core/handoff/caepipe_mbf_engine/tests/writer_parity.rs", "core/handoff/caepipe_mbf_engine/tests/parity.rs"],
     "anchor": {"kind": "excluded", "reason": "MBF parity vectors (JSON text); not solved"}},
    # Retained-precision DAG storage (tools/fixtures/retained_precision_corpus.py), read expanded; every pressurised
    # case of its invocation requests is named.
    {"id": "pressure_source_reader_corpus", "paths": ["fixtures/results/pressure_source_reader_corpus.json"],
     "producer": _writer("rust_test", "core/product_physics/tests/pressure_source_join/consequences.rs",
                         "SWBPIPE_PRESSURE_SOURCE_FIXTURES_WRITE=1"),
     "pinned_by": ["tests/test_pressure_source_corpus.py", "core/product_physics/src/source_recovery/j7_rf_census.rs",
                   "apps/desktop/src/features/results/pressureSourceCorpusParity.test.ts"],
     "anchor": {"kind": "excluded", "reason": "documents inside the retained-precision DAG storage"}},
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

# Code files that build a pressurised document which is solved: the expectations their assertions rest on
# (EXPECTATIONS: an independent closed form, a reference file, the product's own output, or no response value at all)
# and whether their pressurised cases are named in model_settings.pressure_elongation (CASES_NAMED). Where cases are
# not named and an assertion rests on an independent value, the detail says why that value does not depend on the free
# pressure strain, or names the open matter.
IN_CODE = [
    {"paths": ["core/product_physics/src/cap_remainder_publication_tests.rs"],
     "expectations": "no_response_value", "cases_named": "none",
     "detail": "anchored closed chains at 2 MPa; asserts exception admission, record, warning and blocking codes; "
               "unnamed by design (exception family excludes elongation)"},
    {"paths": ["core/product_physics/src/case_state/tests.rs"],
     "expectations": "closed_form", "cases_named": "all",
     "detail": "anchored-anchored closed straight; N_wall = 2 nu p Ai - E As eps closed form; run() names every "
               "pressurised case"},
    {"paths": ["core/product_physics/src/f1b_tests.rs"],
     "expectations": "product_output", "cases_named": "none",
     "detail": "sparse-wiring parity against the product's own assembled bases over a declared subset including "
               "load_reference connected.request and t4_i12 connector; PHYS-R4 pressurised branch never called"},
    {"paths": ["core/product_physics/src/joint_pressure_tests.rs"],
     "reference": "validation/references/t4_i12",
     "expectations": "closed_form", "cases_named": "all",
     "detail": "t4_i12-derived U5a joint controls; closed-form rhs and q; model_settings names case:A"},
    {"paths": ["core/product_physics/src/kd5_pressure_closure_tests.rs"],
     "expectations": "closed_form", "cases_named": "all",
     "detail": "load closure against binary64 laws; named via with_pressurised_cases_named"},
    {"paths": ["core/product_physics/src/membrane_publication_range.rs"],
     "expectations": "reference_file", "cases_named": "all",
     "detail": "direct PHYS-R4 publisher against frozen Fraction reference; named"},
    {"paths": ["core/product_physics/src/o4_m3_judging_tests.rs"],
     "reference": "validation/references/o4_retained_source",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "o4 frozen references; SETTING names every pressurised case"},
    {"paths": ["core/product_physics/src/pressure_runtime.rs"],
     "expectations": "closed_form", "cases_named": "all",
     "detail": "product source; inline tests assemble region ledgers against closed forms, naming every pressurised "
               "case"},
    {"paths": ["core/product_physics/src/source_receipt/load_state_fallback_tests.rs"],
     "expectations": "product_output", "cases_named": "none",
     "detail": "1e-5 Pa sensitive case; expected rows from the uncaptured typed route"},
    {"paths": ["core/product_physics/src/source_receipt/load_state_tests.rs"],
     "expectations": "no_response_value", "cases_named": "none",
     "detail": "witness(true) asserts route, profile and not_joined record only"},
    {"paths": ["core/product_physics/src/timoshenko_product_tests.rs"],
     "expectations": "no_response_value", "cases_named": "none",
     "detail": "t4_i7 mixed L line solved; asserts section and region evidence only"},
    {"paths": ["core/product_physics/src/u4b_tests.rs"],
     "reference": "validation/references/t4_u4b",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "t4_u4b references; named"},
    {"paths": ["core/product_physics/tests/arc_tangent_direction_magnitude.rs"],
     "expectations": "product_output", "cases_named": "all",
     "detail": "rack bend in a pressurised region; asserts the published direction magnitude and reader admission; "
               "model_settings names case:pressure"},
    {"paths": ["core/product_physics/tests/combination_certificate.rs"],
     "reference": "validation/references/combination_certificate",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "combination_certificate frozen references; every pressurised case named"},
    {"paths": ["core/product_physics/tests/common/frozen_rows.rs"],
     "reference": "validation/references/t4_u6, validation/references/t4_i8, validation/references/t4_u8",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "shared comparison with t4_u6/t4_i8/t4_u8 frozen rows; names every pressurised case"},
    {"paths": ["core/product_physics/tests/connector_readers.rs"],
     "expectations": "product_output", "cases_named": "none",
     "detail": "t4_i12 and t4_u5b connector documents solved; reader admission and the committed connector reader "
               "corpus"},
    {"paths": ["core/product_physics/tests/exact_admission_seam.rs"],
     "expectations": "product_output", "cases_named": "none",
     "detail": "X0/Y0 and patched twins; admission codes, the committed pressure_v3 reader corpus and product-vs-"
               "product w15 scaling"},
    {"paths": ["core/product_physics/tests/exact_route_limitations.rs"],
     "expectations": "product_output", "cases_named": "some",
     "detail": "committed limitation fixture; variants solved not included, included and no effect, each stated"},
    {"paths": ["core/product_physics/tests/exact_route_refusals.rs"],
     "expectations": "no_response_value", "cases_named": "none",
     "detail": "refusal codes, emission order and signed-zero pressure publication"},
    {"paths": ["core/product_physics/tests/f1b_w2_runtime.rs"],
     "expectations": "closed_form", "cases_named": "all",
     "detail": "force-scaling exact references; the pressurised PHYS-R4 fixture names case:source-section"},
    {"paths": ["core/product_physics/tests/global_end_actions_runtime.rs"],
     "expectations": "closed_form", "cases_named": "none",
     "detail": "nodal equilibrium of published rows and authored loads, and local axes from geometry: hold whether "
               "or not elongation is included"},
    {"paths": ["core/product_physics/tests/load_reference_state_runtime_extension.rs"],
     "expectations": "reference_file", "cases_named": "all",
     "detail": "reference_cases.json and stated derivations; solve helper names every pressurised case"},
    {"paths": ["core/product_physics/tests/physics_source_runtime.rs"],
     "expectations": "no_response_value", "cases_named": "none",
     "detail": "pressurised mixed and n05 variants assert route identity and evidence counts only"},
    {"paths": ["core/product_physics/tests/pressure_arc_readers.rs"],
     "expectations": "product_output", "cases_named": "all",
     "detail": "writes the committed arc reader corpus; t4_i7 sketches named; the connector variant carries no "
               "region"},
    {"paths": ["core/product_physics/tests/pressure_elongation_on_path.rs"],
     "expectations": "product_output", "cases_named": "all",
     "detail": "the regression anchor: committed pre-switch envelopes against solves naming every pressurised case"},
    {"paths": ["core/product_physics/tests/pressure_elongation_switch.rs"],
     "expectations": "closed_form", "cases_named": "some",
     "detail": "the switch controls: closed forms for both settings; unnamed cases are judged against the not-"
               "included closed form"},
    {"paths": ["core/product_physics/tests/pressure_grouping_limits.rs"],
     "expectations": "closed_form", "cases_named": "some",
     "detail": "admission guard; named via CASE in its builder; the cap remainder fixture and the without-elongation"
               " controls are judged against the not-included statics"},
    {"paths": ["core/product_physics/tests/pressure_membrane_range.rs"],
     "expectations": "no_response_value", "cases_named": "none",
     "detail": "the subnormal PHYS-R4 fixture is refused NUMERICAL_INTEGRITY_UNRESOLVED; no rows"},
    {"paths": ["core/product_physics/tests/pressure_runtime.rs"],
     "expectations": "reference_file", "cases_named": "all",
     "detail": "frozen FROZEN_RUNTIME_EXPECTATIONS; solve helper names every pressurised case"},
    {"paths": ["core/product_physics/tests/pressure_section_geometry.rs"],
     "expectations": "reference_file", "cases_named": "all",
     "detail": "frozen section expectations; solve helper names every pressurised case"},
    {"paths": ["core/product_physics/tests/pressure_source_join/consequences.rs"],
     "expectations": "no_response_value", "cases_named": "none",
     "detail": "genuine producer outputs read by the pressure-source readers; reader statements only"},
    {"paths": ["core/product_physics/tests/rt_a3_constant_effort_references.rs"],
     "reference": "validation/references/rt_a3_constant_effort",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "rt_a3_constant_effort references; named"},
    {"paths": ["core/product_physics/tests/rt_a3_nonlinear_references.rs"],
     "reference": "validation/references/rt_a3_nonlinear",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "rt_a3_nonlinear references; named"},
    {"paths": ["core/product_physics/tests/rt_a3_reducer_2b_reference_system.rs"],
     "reference": "validation/references/reference_system",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "reference_system weight totals (equilibrium) and product consistency relations; the fixture names "
               "every pressurised case"},
    {"paths": ["core/product_physics/tests/rt_a3_reducer_2b_references.rs"],
     "reference": "validation/references/rt_a3_reducer_2b",
     "expectations": "reference_file", "cases_named": "some",
     "detail": "rt_a3_reducer_2b references with the case named; unnamed variants judged against the not-included "
               "unbalanced-thrust model"},
    {"paths": ["core/product_physics/tests/rt_a3_reducer_references.rs"],
     "reference": "validation/references/rt_a3_reducer",
     "expectations": "reference_file", "cases_named": "some",
     "detail": "rt_a3_reducer references with the case named; unnamed variants judged against the not-included "
               "unbalanced-thrust model"},
    {"paths": ["core/product_physics/tests/rt_a3_s6_reference_system.rs"],
     "reference": "validation/references/reference_system",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "reference_system weight totals (equilibrium), RS-SAM motion and subtractions; the fixture names "
               "every pressurised case"},
    {"paths": ["core/product_physics/tests/rt_a3_tee_references.rs"],
     "reference": "validation/references/rt_a3_tee",
     "expectations": "reference_file", "cases_named": "some",
     "detail": "rt_a3_tee references with the case named; unnamed variants judged against the not-included "
               "unbalanced-thrust model"},
    {"paths": ["core/product_physics/tests/shared_terminal_junction.rs"],
     "expectations": "closed_form", "cases_named": "all",
     "detail": "closed-form cap statics at the anchored junction; the committed request names case:pressure"},
    {"paths": ["core/product_physics/tests/spring_hanger_preload_warning.rs"],
     "expectations": "no_response_value", "cases_named": "none",
     "detail": "warning codes and messages only"},
    {"paths": ["core/product_physics/tests/t4_u2_pressure_references.rs"],
     "reference": "validation/references/t4_i7, validation/references/t4_i8",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "t4_i7/t4_i8 frozen references; named"},
    {"paths": ["core/product_physics/tests/w15_rigid_scaling_benchmarks.rs"],
     "reference": "validation/references/w15_rigid_scaling",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "w15_rigid_scaling references; the pressure cantilever is named"},
    {"paths": ["core/product_physics/tests/w16_exact_combinations.rs"],
     "reference": "validation/references/w16_exact_combinations",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "w16_exact_combinations references; case:A/B/C named"},
    {"paths": ["core/model_operations/operation_applier/tests/exact_authoring.rs"],
     "expectations": "closed_form", "cases_named": "all",
     "detail": "closed-end free axial strain closed form at the tip; the elongation intent names case:closed-"
               "pressure before the solve"},
    {"paths": ["core/model_operations/operation_applier/tests/l_line_authoring.rs"],
     "reference": "validation/references/t4_i7",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "authored L line against T4-I7 L-line reference; case:u2 named"},
    {"paths": ["core/model_operations/operation_applier/tests/library_apply.rs"],
     "expectations": "closed_form", "cases_named": "all",
     "detail": "tip torsional rotation of the exact authoring fixture (named) with an inserted E/nu/G; the pressure-"
               "free twin drops regions and setting"},
    {"paths": ["core/model_operations/operation_applier/tests/pipe_reconnect.rs"],
     "expectations": "no_response_value", "cases_named": "none",
     "detail": "pressurised region documents solved for status and PRESSURE_REGION_INTERNAL_BRANCH only; the physics"
               " section's closed forms are pressure-free"},
    {"paths": ["core/runner/headless/tests/tied_joint_references.rs"],
     "reference": "validation/references/t4_u5b",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "t4_u5b references through typed, captured and headless consumers; the shared builder names every "
               "pressurised case"},
    {"paths": ["core/runner/headless/tests/untied_joint_pressure_references.rs"],
     "reference": "validation/references/t4_u5a",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "t4_u5a references; the committed requests name every pressurised case"},
    {"paths": ["core/solver/performance_harness/rack/rack_model.py", "tests/test_performance_rack_model.py"],
     "expectations": "no_response_value", "cases_named": "none",
     "detail": "RT-U4 performance racks: rack_model.py builds them (the exact profiles carry operating-case regions)"
               " and the test solves small racks through the runner for status only; the harness times them; no "
               "response value is judged"},
    {"paths": ["apps/desktop/src-tauri/src/lib.rs"],
     "expectations": "no_response_value", "cases_named": "all",
     "detail": "desktop source; an inline test solves the committed exact authoring fixture (which names its case) "
               "for status, contract id and case count only"},
    {"paths": ["apps/desktop/src-tauri/src/report_render_exact_tests.rs"],
     "expectations": "no_response_value", "cases_named": "none",
     "detail": "anchored closed chain admitted under the cancellation-screen exception; asserts the report quotes "
               "the warning; unnamed by design (exception family excludes elongation)"},
    {"paths": ["tests/test_caepipe_benchmark.py"],
     "expectations": "closed_form", "cases_named": "some",
     "detail": "CAEPIPE benchmark tranche 2: the committed default series (no setting) is judged against the "
               "elongation-off closed forms, the in-memory on series names the pressure case and is judged against "
               "the elongation-on closed forms"},
    {"paths": ["tests/test_caepipe_mbf_round_trip.py"],
     "expectations": "product_output", "cases_named": "none",
     "detail": "source and bound import solved and compared with each other (product against product), and the "
               "route's admission of imported regions"},
    {"paths": ["tests/test_reference_system_fixture.py"],
     "reference": "validation/references/reference_system",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "reference system solved on EX through the runner; EQ-1/2/3/5 against the independent weight record "
               "and recorded bases; the fixture names every pressurised case"},
    {"paths": ["tests/test_rt_a3_nonlinear_reader.py"],
     "reference": "validation/references/rt_a3_nonlinear",
     "expectations": "no_response_value", "cases_named": "all",
     "detail": "Python and Rust readers' verdicts on runner-solved rt_a3_nonlinear cases; each document named "
               "through pressure_elongation.apply(all)"},
    {"paths": ["validation/benchmarks/stress/src/lib.rs"],
     "reference": "validation/hand_calcs/stress/pressure_hoop_longitudinal.md",
     "expectations": "closed_form", "cases_named": "some",
     "detail": "DEL-09-02 stress benchmark: restrained closed-end case named (expectation 2 nu p Ai includes the "
               "free strain); its not-named twin and the cantilever cases unnamed, whose expectations do not depend "
               "on the free strain; expectations from validation/hand_calcs/stress/pressure_hoop_longitudinal.md"},
    {"paths": ["validation/qualification/fixtures/exact_pressure/generate_package.py"],
     "reference": "validation/references/t4_i7, validation/references/t4_i8",
     "expectations": "reference_file", "cases_named": "all",
     "detail": "builds the VP-STATIC exact_pressure_1 requests from t4_i7/t4_i8 with PE.apply; the committed "
               "MANIFEST and the tests use all"},
    {"paths": ["validation/references/kd5_timoshenko/kd5t_generate.py"],
     "reference": "validation/references/kd5_timoshenko",
     "expectations": "reference_file", "cases_named": "none",
     "detail": "M3-T judged-set generator (and producer of the frozen subset); its pressurised models name no "
               "setting while kd5t_engine's expected rows include the free strain; judged by m3_judge through "
               "run.sh, which no hosted job runs; awaiting the M3 restoration"},
    {"paths": ["validation/references/kd5_timoshenko/kd5t_reauthor.py"],
     "expectations": "no_response_value", "cases_named": "none",
     "detail": "premise-census re-authoring of archived corpora; census.sh solves them for standings only"},
    {"paths": ["validation/references/o4_retained_source/declared_problem/twins.py"],
     "expectations": "no_response_value", "cases_named": "all",
     "detail": "declared-problem study tool: re-authors J2's source requests (the pressurised one names its case) "
               "and records product runs for the O4 census; asserts no response value"},
]

# Every other scanned file, with the reason it needs no group or IN_CODE entry. Patterns are fnmatch patterns over the
# repository path (`*` crosses `/`); an exemption may not take a data file that holds a pressurised document.
EXEMPT = [
    # ---- code that builds no solved pressurised document -------------------------------------------------------------
    {"reason": "product source: reads, validates or routes pressure regions; the field is named in product code only",
     "paths": ["core/product_physics/src/case_state/resolve.rs", "core/product_physics/src/exact_admission.rs",
               "core/product_physics/src/joint.rs", "core/product_physics/src/lib.rs",
               "core/product_physics/src/retained_memory.rs", "core/product_physics/src/retained_product.rs",
               "core/product_physics/src/source_receipt/composite.rs", "core/product_physics/src/validation.rs"]},
    {"reason": "product source; its inline tests give source preparation documents with regions only to show it declines "
               "them, and solve nothing",
     "paths": ["core/product_physics/src/source_recovery.rs"]},
    {"reason": "gives the composite adapter a missing, zero or nonzero region only to show it refuses them; nothing "
               "reaches mechanics",
     "paths": ["core/product_physics/src/source_receipt/tests.rs"]},
    {"reason": "runner source; its inline tests remove a closure or the regions and assert the refusal, with no rows",
     "paths": ["core/runner/headless/src/lib.rs", "core/runner/headless/src/bin/openpipestress-runner.rs"]},
    {"reason": "authoring source (operation applier): reads regions for references, routing and messages; solves nothing",
     "paths": ["core/model_operations/operation_applier/src/lib.rs", "core/model_operations/operation_applier/src/pipe_reconnect.rs",
               "core/model_operations/operation_applier/src/pressure_authoring.rs"]},
    {"reason": "authoring tests: transport and refusal of operations; the one solve has empty regions and no contract and "
               "is refused PREVIEW_CONTRACT_VERSION_MISMATCH",
     "paths": ["core/model_operations/operation_applier/src/b7b_transport_tests.rs",
               "core/model_operations/operation_applier/tests/pipe_delete_dependents.rs"]},
    {"reason": "desktop source and types: authoring, validation and package panels; no solve",
     "paths": ["apps/desktop/src/features/caepipe-mbf/mbfImport.ts", "apps/desktop/src/features/geometry-tools/geometryDraft.ts",
               "apps/desktop/src/features/model-settings/modelSettingsIntents.ts",
               "apps/desktop/src/features/native-package/NativePackagePanel.tsx",
               "apps/desktop/src/features/pressure-authoring/PressureAuthoringPanel.tsx",
               "apps/desktop/src/features/project-validation/ProjectValidationPanel.tsx",
               "apps/desktop/src/services/embeddedAgentSession.ts", "apps/desktop/src/services/projectService.ts",
               "apps/desktop/src/types.ts"]},
    {"reason": "desktop UI tests: authoring through the wasm operation engine or over committed results; no product "
               "solve (lLineAuthoring's solve is a mocked native job returning a committed envelope)",
     "paths": ["apps/desktop/src/features/bend-authoring/BendAuthoringForms.test.tsx",
               "apps/desktop/src/features/bend-authoring/lLineAuthoring.test.tsx",
               "apps/desktop/src/features/caepipe-mbf/mbfImport.test.ts",
               "apps/desktop/src/features/load-cases/referenceSystemFills.engine.test.tsx",
               "apps/desktop/src/features/model-settings/ModelSettingsPressureElongation.test.tsx",
               "apps/desktop/src/features/native-package/NativePackagePanel.test.ts",
               "apps/desktop/src/features/pressure-authoring/PressureAuthoringPanel.test.tsx",
               "apps/desktop/src/features/pressure-authoring/b7bAdmission.test.tsx",
               "apps/desktop/src/features/workspace/referenceEntryJourney.session.test.tsx"]},
    {"reason": "CAEPIPE MBF import and export code and their tests: regions are written, read, compared or asserted "
               "absent; nothing is solved",
     "paths": ["core/handoff/caepipe_mbf/importer.py", "core/handoff/caepipe_mbf/si_writer.py",
               "core/handoff/caepipe_mbf_engine/src/*", "core/handoff/caepipe_mbf_engine/tests/import_review.rs",
               "core/handoff/caepipe_mbf_engine/tests/sidecar_format6.rs", "apps/desktop/src-tauri/src/mbf_import_tests.rs",
               "tests/caepipe_mbf_canonical.py", "tests/test_caepipe_mbf_importer.py", "tests/test_caepipe_mbf_si_writer.py",
               "tools/fixtures/caepipe_mbf_writer_vectors.py"]},
    {"reason": "readers, verifiers and their tests over committed outputs; nothing is solved",
     "paths": ["core/reporting/result_export/src/physics_evidence.rs", "core/reporting/result_export/src/physics_source.rs",
               "core/reporting/result_export/src/retained_precision.rs",
               "core/reporting/result_export/tests/physics_source_contract.rs",
               "core/reporting/result_export/tests/retained_precision_contract.rs",
               "tests/verifiers/physics_source.py", "tests/verifiers/retained_precision.py",
               "tests/test_physics_source_contract.py", "tests/test_retained_precision_b3.py",
               "tests/test_reference_system_records.py"]},
    {"reason": "schema and synthetic structural tests; nothing is solved",
     "paths": ["tests/test_model_document_0_9_0_members.py", "tests/test_model_schema.py",
               "tests/test_qualification_physics_structure.py", "tools/validation/qualification_physics_structure.py"]},
    {"reason": "the VP-HARNESS adapter: binds the exact_pressure package's requests, built and named by "
               "generate_package.py (claimed), and reads the runner's output",
     "paths": ["tools/validation/qualification_exact_pressure.py"]},
    {"reason": "retained-route pricing: static analysis of product source and committed outputs; nothing is solved",
     "paths": ["tools/retained_pricing/base/batch5_model.py", "tools/retained_pricing/lib/census_exact.py",
               "tools/retained_pricing/rules/o10_rules.py", "tools/retained_pricing/rules/sq2_decisions_E.py"]},
    {"reason": "the switch itself and its shared test oracles (naming helpers and the not-included unbalanced-thrust "
               "model); the anchor's tests use a stub solver; their callers are claimed",
     "paths": ["tools/validation/pressure_elongation.py", "tools/validation/test_pressure_elongation_anchor.py",
               "core/product_physics/tests/common/pressure_elongation.rs"]},
    # ---- tests whose documents carry no pressurised case ---------------------------------------------------------------
    {"reason": "every region list it builds is empty, absent or at zero pressure, so no case carries pressure",
     "paths": ["core/model_operations/operation_applier/tests/condition_set_authoring.rs",
               "core/product_physics/src/d5_mass_product_tests.rs", "core/product_physics/src/modal/tests.rs",
               "core/product_physics/src/retained_facade_tests.rs", "core/product_physics/src/retained_memory_law_tests.rs",
               "core/product_physics/src/retained_memory_witness_tests.rs",
               "core/product_physics/src/source_receipt/load_state_join_tests.rs",
               "core/product_physics/src/source_recovery/j7_rf_census.rs",
               "core/product_physics/tests/common/b1_sq_inputs.rs", "core/product_physics/tests/common/w15c_bench.rs",
               "core/product_physics/tests/connector_pp_paths.rs", "core/product_physics/tests/elastic_extrema_runtime.rs",
               "core/product_physics/tests/load_reference_state_runtime.rs",
               "core/product_physics/tests/rt_a3_nonlinear_k1_twins.rs",
               "core/product_physics/tests/rt_a3_reducer_2b_pf_controls.rs", "core/product_physics/tests/rt_a4_combinations.rs",
               "core/product_physics/tests/silent_zero_guard_runtime.rs",
               "core/product_physics/tests/stress_maximum_coverage.rs",
               "core/product_physics/tests/support_reactions_runtime.rs", "core/product_physics/tests/w15_component_body.rs",
               "core/product_physics/tests/w15_component_weight_benchmarks.rs",
               "core/product_physics/tests/w15c_eccentric_reducer.rs", "core/product_physics/tests/w16_imposed_motion.rs",
               "core/product_physics/tests/w9_solver_benchmarks.rs", "tests/test_rt_a4_exact_constant_effort_seam.py",
               "tests/test_w15_component_body_readers.py", "tools/fixtures/j7_rf_census_inputs.py",
               "validation/references/kd5_member_bound/notice_reference.py",
               "validation/references/kd5_member_bound/two_body_reference.py"]},
    {"reason": "regions appear only in decode tests; its solves use t4_u5a J3-0, whose one region carries zero pressure",
     "paths": ["core/product_physics/src/b7b_admission_tests.rs"]},
    {"reason": "it replaces the pressurised case of physics_source/mixed.request.json with an empty-region case and "
               "removes the setting, so no solved case carries pressure",
     "paths": ["core/product_physics/tests/w16_exact_equivalent_static.rs",
               "core/product_physics/tests/w16_timoshenko_partial_wind.rs"]},
    # ---- producers, independent references and records -------------------------------------------------------------------
    {"reason": "producer of a data group above; its committed documents are claimed by that group",
     "paths": ["fixtures/reference_system/generate_reference_system.py", "tools/caepipe_correlation/benchmark_models.py",
               "validation/references/rt_a3_s6/generators/rt_a3_s6_reference.py",
               "validation/references/t4_u4a/u4a_generate.py", "validation/references/t4_u5a/generators/u5a_reference.py",
               "validation/references/t4_u6/generators/u6_reference.py",
               "validation/references/t4_u8/generators/u8_reference.py",
               "core/product_physics/tests/common/t4_u5b.rs"]},
    {"reason": "independent reference generator or engine (imports no product code); the test that builds and solves "
               "its documents is claimed",
     "paths": ["validation/references/kd5_timoshenko/kd5t_engine.py",
               "validation/references/o4_retained_source/generators/o4_reference.py",
               "validation/references/o4_retained_source/generators/o4_source_twins.py",
               "validation/references/rt_a3_constant_effort/generators/rt_a3_constant_effort_reference.py",
               "validation/references/rt_a3_nonlinear/generators/rt_a3_nonlinear_reference.py",
               "validation/references/rt_a3_tee/generators/rt_a3_tee_reference.py",
               "validation/references/t4_i12/generators/u3_reference.py",
               "validation/references/t4_i12/generators/u3_round02.py",
               "validation/references/t4_u4b/generators/u4b_reference.py",
               "validation/references/t4_u5b/generators/u5b_reference.py"]},
    {"reason": "independent reference generator (imports no product code) that states pressure elongation on or off per "
               "case; no product test solves its cases yet",
     "paths": ["validation/references/exact_difference_basis/generators/exact_difference_basis_reference.py",
               "validation/references/exact_load_state_equivalent_static/generators/exact_load_state_reference.py"]},
    {"reason": "historical run record of a frozen reference (groups t4_i7 and t4_i8)",
     "paths": ["validation/references/t4_i7/_run_records/u2_generate.py",
               "validation/references/t4_i8/_run_records/check_reference_json.py",
               "validation/references/t4_i8/_run_records/derive_rebuilt_references.py"]},
    {"reason": "documentation",
     "paths": ["validation/references/*/README.md"]},
    # ---- data files with no pressurised document ----------------------------------------------------------------------
    {"reason": "names the field only in text (tool descriptions, diagnostic messages, operation names, edit paths, "
               "writer notes); holds no model document",
     "paths": ["validation/caepipe_sitting_pack/records/cases/*/*.writer-report.json",
               "apps/desktop/src-tauri/resources/agent-capability-catalog.json", "fixtures/diagnostics/exact_route_codes.json",
               "fixtures/model_operations/agent_w9a_capture_outcomes.json", "fixtures/model_operations/agent_w9a_journey.json",
               "fixtures/product_preview/invented_mechanics_result_preview_physics_1_*.json",
               "fixtures/results/retained_precision_prepared_exact_v1.json", "tools/retained_pricing/base/loop_bounds.g4.json",
               "validation/references/rt_a3_nonlinear/references.json", "validation/references/t4_u5a/oracle.json",
               "validation/references/t4_u6/oracle.json"]},
    {"reason": "retained-route inputs: each edit sets a case's region list to empty",
     "paths": ["fixtures/results/retained_precision_rv120_b3_inputs.json"]},
    {"reason": "the reference system's authoring table: the field appears as an authorable field name only",
     "paths": ["fixtures/reference_system/authoring.json"]},
    {"reason": "model documents (plain, as JSON text or in DAG storage) whose every region list is empty, so no case "
               "carries pressure",
     "paths": ["core/product_physics/src/source_recovery/j7_i12_request.json",
               "core/product_physics/tests/fixtures/model_checks/load_state_fan_out_base.json",
               "core/product_physics/tests/fixtures/pre04_failure_unification/pexact_a_r1e-08_zero_timo.request.json",
               "core/product_physics/tests/fixtures/rf_source_method/order_four_sensitive.request.json",
               "core/product_physics/tests/fixtures/rt_a3_nonlinear/b1_loaded_inactive_reaction.json",
               "core/product_physics/tests/fixtures/silent_zero/*.request.json",
               "core/reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json",
               "core/runner/headless/tests/t4_u5b_requests/MO-F.request.json",
               "core/runner/headless/tests/t4_u5b_requests/MO-T.request.json",
               "fixtures/model_operations/contract_corpus/case_8[234]_*.json",
               "fixtures/operating_conditions/preview_parity.json",
               "fixtures/persistence/recorded_pre_rt_a4_pf/physics_retained_1/*/retained_precision_exact_successor_sparse_interactive.json",
               "fixtures/product_preview/pressure/load_reference/connected.request.json",
               "fixtures/product_preview/pressure_source/load_reference_source/eigen_motion.request.json",
               "fixtures/product_preview/pressure_source/load_reference_source/fields.request.json",
               "fixtures/product_preview/pressure_source/load_reference_source/n0*.request.json",
               "fixtures/product_preview/pressure_source/o4/o4_source_controls.json",
               "fixtures/product_preview/pressure_source/physics_source/fields.request.json",
               "fixtures/product_preview/pressure_source/physics_source/n0*.request.json",
               "fixtures/reports/invented/exact_report_package_requests/pressure_source_1.json",
               "fixtures/reports/invented/user_release_exact/pressure_source_1.json",
               "fixtures/results/j7_rf_census_inputs.json", "fixtures/results/modal_*.request.json",
               "fixtures/results/retained_precision_cases.json", "fixtures/results/retained_precision_*_successor_*.json",
               "validation/qualification/fixtures/first_static/*.json", "validation/qualification/fixtures/load_reference/*.json",
               "validation/references/kd5_member_bound/request.json",
               "validation/references/kd5_member_bound/two_body_request.json",
               "validation/references/rt_a3_s6/requests/ls1_*.json", "validation/references/t4_i12/u3_reference_cases.json",
               "validation/references/t4_u8/requests/LOAD-STATE-PER-CASE-NU-*.request.json",
               "validation/references/t4_u8/requests/S-*.request.json"]},
    {"reason": "a model document whose one region carries zero pressure",
     "paths": ["validation/references/t4_u5a/requests/J3-0.request.json"]},
    {"reason": "o4 reference cases in the reference's own shape, every region list empty; o4_m3_judging_tests.rs builds "
               "the product documents (claimed)",
     "paths": ["validation/references/o4_retained_source/cases.json",
               "validation/references/o4_retained_source/source_twins_cases.json"]},
    {"reason": "reference case inputs that give pressure as a plain number, not model documents; the test that builds "
               "and solves documents from them names the case (rt_a3_constant_effort_references.rs, "
               "rt_a3_tee_references.rs, u4b_tests.rs; claimed)",
     "paths": ["validation/references/rt_a3_constant_effort/references.json", "validation/references/rt_a3_tee/references.json",
               "validation/references/t4_u4b/cases.json"]},
    {"reason": "reference case inputs that give pressure as a plain number and state pressure elongation on or off per "
               "case; no product test solves them yet",
     "paths": ["validation/references/exact_difference_basis/references.json",
               "validation/references/exact_load_state_equivalent_static/references.json"]},
    {"reason": "the inventory itself, its tests and its documentation",
     "paths": ["tools/validation/README.md", "tools/validation/pressure_elongation_inventory.py",
               "tools/validation/pressure_elongation_inventory.json",
               "tools/validation/test_pressure_elongation_inventory.py"]},
]


class InventoryError(Exception):
    """The claims do not cover the scanned files exactly once."""


def scanned_files(root: Path = ROOT) -> list[str]:
    """Every tracked file under SCAN_ROOTS whose bytes name TOKEN, sorted."""
    done = subprocess.run(["git", "grep", "-l", "-F", TOKEN, "--", *SCAN_ROOTS], cwd=root, capture_output=True, text=True)
    if done.returncode not in (0, 1):
        raise InventoryError(f"git grep failed: {done.stderr.strip()}")
    return sorted(path for path in done.stdout.splitlines() if path)


def is_data(path: str) -> bool:
    return path.endswith(DATA_SUFFIXES)


def _texts(value, pointer: str = ""):
    """(pointer, parsed) for each string value naming TOKEN that is itself a JSON object or array."""
    if isinstance(value, dict):
        for key, item in value.items():
            yield from _texts(item, f"{pointer}/{PE._escape(key)}")
    elif isinstance(value, list):
        for index, item in enumerate(value):
            yield from _texts(item, f"{pointer}/{index}")
    elif isinstance(value, str) and TOKEN in value:
        try:
            parsed = json.loads(value)
        except ValueError:
            return
        if isinstance(parsed, (dict, list)):
            yield pointer, parsed


def container_documents(container, texts: tuple = ()) -> tuple[list[dict], list[dict]]:
    """(plain, held): {pointer, schema_version, pressurised_load_cases} for each pressurised document in `container`, in
    document order, and for each held as JSON text in one of its strings, where `pointer` names the outermost string
    and `text_pointers` the path inside it (each inner string, then the document)."""
    plain, held = [], []
    for pointer, doc in PE.documents(container):
        cases = PE.pressurised_case_ids(doc)
        if cases:
            where = {"pointer": texts[0], "text_pointers": [*texts[1:], pointer]} if texts else {"pointer": pointer}
            (held if texts else plain).append({**where, "schema_version": doc.get("schema_version"),
                                               "pressurised_load_cases": cases})
    for pointer, parsed in _texts(container):
        inner, deeper = container_documents(parsed, (*texts, pointer))
        held.extend(inner + deeper)
    return plain, held


def data_documents(path: str, text: str) -> tuple[list[dict], list[dict]]:
    """(plain, held) pressurised documents of one data file. Plain documents are those the anchor harness reads by
    JSON pointer. Held documents are in JSON text, in a `.jsonl` line (each record carries its `line`) or in a
    retained-precision DAG definition, read expanded (each record carries its `storage`)."""
    if path.endswith(".jsonl"):
        held = []
        for number, line in enumerate(text.splitlines(), 1):
            if line.strip():
                plain, inner = container_documents(json.loads(line))
                held.extend({"line": number, **record} for record in plain + inner)
        return [], held
    container = json.loads(text)
    if isinstance(container, dict) and container.get("format") == CORPUS.FORMAT:
        plain, inner = container_documents(CORPUS.expand(container))
        return [], [{**record, "storage": CORPUS.FORMAT} for record in plain + inner]
    return container_documents(container)


def scan(root: Path = ROOT) -> tuple[list[str], dict[str, tuple[list[dict], list[dict]]]]:
    """(every scanned file, {data file: (plain, held) pressurised documents}); a data file may hold none."""
    files = scanned_files(root)
    return files, {path: data_documents(path, (root / path).read_text(encoding="utf-8")) for path in files if is_data(path)}


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


def _matches(path: str, patterns: list[str]) -> bool:
    return any(fnmatch.fnmatchcase(path, pattern) for pattern in patterns)


def claims(files: list[str], data: dict[str, tuple[list[dict], list[dict]]], groups: list[dict], in_code: list[dict],
           exempt: list[dict]) -> tuple[dict[str, list[str]], list[str]]:
    """({file: the claims that take it}, problems). A group takes the data files with a pressurised document that
    its patterns match; an IN_CODE entry takes the code files it names exactly; an exemption takes the files its
    patterns match. A claim, or one of its patterns, that takes nothing is stale."""
    pressurised = {path for path, (plain, held) in data.items() if plain or held}
    scanned = set(files)
    taken: dict[str, list[str]] = {path: [] for path in files}
    problems = []
    for group in groups:
        for pattern in group["paths"]:
            if not any(fnmatch.fnmatchcase(path, pattern) for path in pressurised):
                problems.append(f"stale: group {group['id']} path {pattern} matches no data file with a pressurised document")
        for path in sorted(p for p in pressurised if _matches(p, group["paths"])):
            taken[path].append(f"group {group['id']}")
    for entry in in_code:
        label = f"IN_CODE {', '.join(entry.get('paths') or ['(no paths)'])}"
        if not entry.get("paths"):
            problems.append(f"{label}: names no file")
        if entry.get("expectations") not in EXPECTATIONS:
            problems.append(f"{label}: expectations must be one of {', '.join(EXPECTATIONS)}")
        if entry.get("cases_named") not in CASES_NAMED:
            problems.append(f"{label}: cases_named must be one of {', '.join(CASES_NAMED)}")
        if not str(entry.get("detail") or "").strip():
            problems.append(f"{label}: no detail of what its assertions rest on")
        for path in entry.get("paths") or []:
            if path not in scanned:
                problems.append(f"stale: IN_CODE path {path} is not a tracked file naming {TOKEN}")
            elif is_data(path):
                problems.append(f"IN_CODE path {path} is a data file; claim it with a group or an exemption")
            else:
                taken[path].append(f"IN_CODE {path}")
    for index, entry in enumerate(exempt):
        patterns = entry.get("paths") or []
        label = f"exemption #{index} ({patterns[0] if patterns else 'no paths'}{f' and {len(patterns) - 1} more' if len(patterns) > 1 else ''})"
        if not isinstance(entry.get("reason"), str) or not entry["reason"].strip():
            problems.append(f"{label}: no reason")
        for pattern in patterns:
            if not any(fnmatch.fnmatchcase(path, pattern) for path in files):
                problems.append(f"stale: exemption path {pattern} matches no tracked file naming {TOKEN}")
        for path in files:
            if _matches(path, patterns):
                if path in pressurised:
                    problems.append(f"{label}: {path} holds a pressurised document; claim it with a group")
                else:
                    taken[path].append(label)
    for path in files:
        if not taken[path]:
            problems.append(f"unclaimed: {path}")
        elif len(taken[path]) > 1:
            problems.append(f"claimed twice: {path} ({'; '.join(taken[path])})")
    return taken, problems


def build(groups: list[dict] | None = None, in_code: list[dict] | None = None, exempt: list[dict] | None = None,
          root: Path = ROOT, scanned: tuple | None = None) -> dict:
    """The inventory from the claims (default: this module's GROUPS, IN_CODE and EXEMPT) over `scanned` (default: a
    fresh `scan(root)`); InventoryError lists every problem."""
    groups = GROUPS if groups is None else groups
    in_code = IN_CODE if in_code is None else in_code
    exempt = EXEMPT if exempt is None else exempt
    files, data = scan(root) if scanned is None else scanned
    taken, problems = claims(files, data, groups, in_code, exempt)
    entries = []
    for group in groups:
        paths = sorted(p for p, (plain, held) in data.items() if (plain or held) and _matches(p, group["paths"]))
        for path in paths:
            documents, held = data[path]
            if group["anchor"]["kind"] != "excluded" and not documents:
                problems.append(f"group {group['id']}: {path} holds no document the harness can read (only JSON Lines, "
                                "JSON text or DAG storage); its anchor must be excluded")
            anchor = {k: v for k, v in group["anchor"].items() if k != "files_from"}
            if group["anchor"]["kind"] == "raw_files":
                container = json.loads((root / path).read_text(encoding="utf-8"))
                files_for, _ = _raw_files(group["anchor"], path, container)
                if files_for is None:
                    anchor = dict(BASELINE)
                else:
                    anchor["files"] = files_for
            entry = {"id": group["id"] if len(paths) == 1 else f"{group['id']}/{_suffix(path, paths)}",
                     "group": group["id"], "path": path, "producer": group["producer"],
                     "pinned_by": group["pinned_by"], "anchor": anchor, "documents": documents}
            if held:
                entry["held_documents"] = held
            entries.append(entry)
    if problems:
        raise InventoryError("inventory out of date:\n  " + "\n  ".join(problems))
    counts: dict[str, dict] = {}
    for entry in entries:
        c = counts.setdefault(entry["group"], {"files": 0, "documents": 0, "pressurised_load_cases": 0})
        c["files"] += 1
        c["documents"] += len(entry["documents"])
        c["pressurised_load_cases"] += sum(len(d["pressurised_load_cases"]) for d in entry["documents"])
        if entry.get("held_documents"):
            c["held_documents"] = c.get("held_documents", 0) + len(entry["held_documents"])
            c["held_pressurised_load_cases"] = (c.get("held_pressurised_load_cases", 0)
                                                + sum(len(d["pressurised_load_cases"]) for d in entry["held_documents"]))
    kinds = {"group": 0, "in_code": 0, "exemption": 0}
    for path in files:
        kind = taken[path][0].split(" ", 1)[0]
        kinds[{"group": "group", "IN_CODE": "in_code", "exemption": "exemption"}[kind]] += 1
    return {
        "document_kind": "swbpipe.pressure_elongation_inventory",
        "version": 2,
        "purpose": "Every tracked file under the scanned roots that names pressure_regions, each claimed once: a data file "
                   "with a pressurised document by its group (producer, pinning readers, anchor route), a code file "
                   "that builds a solved pressurised document by an in-code entry (what its assertions rest on, whether "
                   "its cases are named), any other by a reasoned exemption. The DEL-04-07 regression anchor's scope "
                   "and the fixture re-production checklist. Generated by tools/validation/pressure_elongation_inventory.py.",
        "setting": "model_settings.pressure_elongation.operating_load_cases",
        "switch": {"helper": "tools/validation/pressure_elongation.py", "values": list(PE.MODES), "default": PE.DEFAULT,
                   "environment": PE.ENV},
        "pressurised_case_rule": "a pressure region with a nonzero pressure, or condition_use.pressure resolving to a "
                                 "nonzero condition pressure on a pipe of one of the case's regions",
        "scan": {"roots": list(SCAN_ROOTS), "token": TOKEN, "data_suffixes": list(DATA_SUFFIXES), "files": len(files),
                 "claimed_by": kinds},
        "counts": counts,
        "entries": entries,
        "derived_consumers": DERIVED,
        "in_code_constructions": in_code,
        "exemptions": [{"reason": entry["reason"], "paths": entry["paths"],
                        "files": [p for p in files if _matches(p, entry["paths"])]} for entry in exempt],
    }


def dumps(value) -> str:
    return json.dumps(value, indent=1, ensure_ascii=False) + "\n"


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="compare with the committed inventory; write nothing")
    args = parser.parse_args(argv)
    try:
        text = dumps(build())
    except InventoryError as error:
        print(error, file=sys.stderr)
        return 1
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
