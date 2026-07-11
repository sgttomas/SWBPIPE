"""Focused tests for the DEL-09-05 release gate-outcome record emitter.

Fences under test (completion-plan row E8):

- governed-TBD discipline: TBD criteria never carry an invented value;
- no release label, readiness status, or F-PIP-2 claim is minted;
- no coverage floor is promoted regardless of the artifact count (DEC-060);
- the DEC-025 five-surface sweep plan is untouched;
- only clean-head commit-bound sweep artifacts qualify as gate evidence.
"""

from __future__ import annotations

import importlib.util
import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "release" / "run_release_gate_records.py"

EXPECTED_FAMILY_ORDER = ["solver", "rule_engine", "gui", "report_template", "mixed"]
SWEEP_SURFACES = [
    "cargo_crate_sweep",
    "python_pytest",
    "desktop_vitest",
    "desktop_playwright_e2e",
    "desktop_production_build",
]
COMMIT_A = "a" * 40
COMMIT_B = "b" * 40


def load_module():
    spec = importlib.util.spec_from_file_location(
        "run_release_gate_records", MODULE_PATH
    )
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def make_sweep_artifact(
    commit=COMMIT_A,
    dirty=False,
    surface_statuses=None,
    started="2026-07-11T00:00:00+00:00",
):
    statuses = dict.fromkeys(SWEEP_SURFACES, "pass")
    statuses.update(surface_statuses or {})
    return {
        "artifact": "openpipestress.evidence_sweep_summary",
        "schema_version": 2,
        "git": {
            "commit_hash": commit,
            "branch": "main",
            "status_capture_failed": False,
            "working_tree_dirty": dirty,
            "dirty_paths": ["x"] if dirty else [],
        },
        "started_utc": started,
        "surfaces": [
            {"surface_id": surface_id, "status": status}
            for surface_id, status in statuses.items()
        ],
        "overall_status": (
            "fail" if any(s != "pass" for s in statuses.values()) else "pass"
        ),
    }


def make_coverage_artifact(commit=COMMIT_A, dirty=False):
    return {
        "artifact": "openpipestress.coverage_telemetry",
        "git": {
            "commit_hash": commit,
            "status_capture_failed": False,
            "working_tree_dirty": dirty,
        },
    }


def write_json(path: Path, body: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(body), encoding="utf-8")


def clean_git_state(commit=COMMIT_B):
    return {
        "commit_hash": commit,
        "branch": "work",
        "status_capture_failed": False,
        "working_tree_dirty": False,
        "dirty_paths": [],
    }


def build_record(gates, tmp_path, family="solver", sweep_bodies=(), scan_bodies=(),
                 coverage_bodies=(), evaluated_commit=COMMIT_A):
    sweep_dir = tmp_path / "sweeps"
    scan_dir = tmp_path / "releases"
    coverage_dir = tmp_path / "coverage"
    for index, body in enumerate(sweep_bodies):
        write_json(sweep_dir / f"SWEEP_{index}.json", body)
    for index, body in enumerate(scan_bodies):
        write_json(scan_dir / f"SCAN_{index}.json", body)
    for index, body in enumerate(coverage_bodies):
        write_json(coverage_dir / f"COVERAGE_{index}.json", body)
    return gates.build_family_record(
        family,
        evaluated_commit,
        gates.select_sweep_evidence(sweep_dir, evaluated_commit),
        gates.select_scan_evidence(scan_dir, evaluated_commit),
        gates.coverage_floor_context(coverage_dir),
        clean_git_state(),
        {"platform": "test", "python": "3"},
        "2026-07-11T01:00:00+00:00",
    )


def test_family_catalog_is_the_five_gate_families():
    gates = load_module()
    assert gates.FAMILY_IDS == EXPECTED_FAMILY_ORDER
    for family in EXPECTED_FAMILY_ORDER:
        ids = [spec.criterion_id for spec in gates.family_criteria(family)]
        assert len(ids) == len(set(ids)), f"duplicate criterion ids in {family}"
        # Common section-3 criteria appear in every family record.
        assert "common.human_acceptance_or_waiver" in ids


def test_mixed_family_is_the_union_plus_waiver():
    gates = load_module()
    mixed_ids = {spec.criterion_id for spec in gates.family_criteria("mixed")}
    union_ids = set()
    for family in ("solver", "rule_engine", "gui", "report_template"):
        union_ids |= {spec.criterion_id for spec in gates.family_criteria(family)}
    assert mixed_ids == union_ids | {"mixed.waiver_and_risk_disposition"}


def test_green_sweep_yields_pass_for_sweep_derived_criteria(tmp_path):
    gates = load_module()
    record = build_record(gates, tmp_path, sweep_bodies=[make_sweep_artifact()])

    by_id = {c["criterion_id"]: c for c in record["criteria"]}
    assert by_id["solver.mechanics_benchmarks_pass"]["status"] == "pass"
    assert by_id["solver.tolerance_source_named_or_tbd"]["status"] == "pass"
    # Human-gated and governed-TBD criteria stay TBD even on a green sweep.
    assert by_id["common.human_acceptance_or_waiver"]["status"] == "TBD"
    assert record["status_counts"]["fail"] == 0
    assert record["inputs"]["sweep_evidence"]["selected"]["path"] == "SWEEP_0.json"


def test_failed_surface_yields_fail_and_not_run_yields_tbd(tmp_path):
    gates = load_module()
    sweep = make_sweep_artifact(
        surface_statuses={
            "cargo_crate_sweep": "fail",
            "python_pytest": "not_run",
            "desktop_vitest": "not_run",
            "desktop_playwright_e2e": "not_run",
            "desktop_production_build": "not_run",
        }
    )
    record = build_record(gates, tmp_path, family="mixed", sweep_bodies=[sweep])

    by_id = {c["criterion_id"]: c for c in record["criteria"]}
    assert by_id["solver.mechanics_benchmarks_pass"]["status"] == "fail"
    assert by_id["solver.mechanics_benchmarks_pass"]["tbd_reason"] is None
    gui = by_id["gui.missing_data_visibility"]
    assert gui["status"] == "TBD"
    assert gui["tbd_reason"] == "evidence_not_available"


def test_no_sweep_at_commit_yields_tbd_not_fail(tmp_path):
    gates = load_module()
    record = build_record(
        gates,
        tmp_path,
        sweep_bodies=[make_sweep_artifact(commit=COMMIT_B)],
        evaluated_commit=COMMIT_A,
    )

    by_id = {c["criterion_id"]: c for c in record["criteria"]}
    assert record["inputs"]["sweep_evidence"]["selected"] is None
    assert by_id["solver.mechanics_benchmarks_pass"]["status"] == "TBD"
    assert (
        by_id["solver.mechanics_benchmarks_pass"]["tbd_reason"]
        == "evidence_not_available"
    )
    assert record["status_counts"]["fail"] == 0


def test_dirty_sweep_artifacts_never_qualify_as_gate_evidence(tmp_path):
    gates = load_module()
    record = build_record(
        gates, tmp_path, sweep_bodies=[make_sweep_artifact(dirty=True)]
    )

    evidence = record["inputs"]["sweep_evidence"]
    assert evidence["artifacts_at_commit"] == 1
    assert evidence["clean_artifacts_at_commit"] == 0
    assert evidence["selected"] is None


def test_latest_clean_sweep_is_selected(tmp_path):
    gates = load_module()
    older = make_sweep_artifact(started="2026-07-10T00:00:00+00:00")
    newer = make_sweep_artifact(started="2026-07-11T02:00:00+00:00")
    record = build_record(gates, tmp_path, sweep_bodies=[newer, older])

    assert (
        record["inputs"]["sweep_evidence"]["selected"]["started_utc"]
        == "2026-07-11T02:00:00+00:00"
    )


def test_tbd_criteria_carry_reasons_and_no_invented_values(tmp_path):
    gates = load_module()
    record = build_record(gates, tmp_path, family="mixed",
                          sweep_bodies=[make_sweep_artifact()])

    for criterion in record["criteria"]:
        if criterion["status"] == "TBD":
            assert criterion["tbd_reason"] in {
                "human_gated",
                "governed_value_tbd",
                "not_evaluable_by_agent",
                "evidence_not_available",
            }
        # A criterion is a status record only: it never carries a numeric
        # threshold, measured value, or defaulted quantity.
        assert set(criterion.keys()) == {
            "criterion_id",
            "gate_doc_section",
            "text",
            "basis_kind",
            "sweep_surfaces",
            "governed_sources",
            "status",
            "tbd_reason",
            "notes",
        }


def test_governed_tbd_criterion_stays_tbd_on_green_sweep(tmp_path):
    gates = load_module()
    record = build_record(gates, tmp_path, family="gui",
                          sweep_bodies=[make_sweep_artifact()])

    by_id = {c["criterion_id"]: c for c in record["criteria"]}
    matrix = by_id["gui.browser_device_matrix_accessibility"]
    assert matrix["status"] == "TBD"
    assert matrix["tbd_reason"] == "governed_value_tbd"


def test_record_mints_no_release_label_or_readiness_claim(tmp_path):
    gates = load_module()
    for family in EXPECTED_FAMILY_ORDER:
        record = build_record(gates, tmp_path, family=family,
                              sweep_bodies=[make_sweep_artifact()])
        assert record["release_labels"]["minted"] is False
        assert "not a release" in record["boundary_note"].lower()
        serialized = json.dumps(record).lower()
        for forbidden in ("release-ready", "release ready", "release_ready"):
            assert forbidden not in serialized, (family, forbidden)


def test_coverage_floor_is_never_promoted_even_when_count_is_met(tmp_path):
    gates = load_module()
    below = build_record(
        gates, tmp_path / "below",
        coverage_bodies=[make_coverage_artifact()],
    )
    context = below["coverage_telemetry_context"]
    assert context["clean_head_artifacts"] == 1
    assert context["distinct_clean_head_commits"] == 1
    assert context["artifact_count_prerequisite_met"] is False
    assert context["promotion_performed"] is False

    many = [make_coverage_artifact(commit=COMMIT_A) for _ in range(3)] + [
        make_coverage_artifact(commit=COMMIT_B) for _ in range(3)
    ]
    met = build_record(gates, tmp_path / "met", coverage_bodies=many)
    context = met["coverage_telemetry_context"]
    assert context["artifact_count_prerequisite_met"] is True
    # Meeting the count NEVER promotes: a new owner-ruled decision is required.
    assert context["promotion_performed"] is False
    assert "owner-ruled" in context["promotion_prerequisites"]
    assert "none exist" in context["numeric_floors"]


def test_dirty_coverage_artifacts_do_not_count_toward_promotion(tmp_path):
    gates = load_module()
    record = build_record(
        gates, tmp_path,
        coverage_bodies=[make_coverage_artifact(dirty=True)],
    )
    assert record["coverage_telemetry_context"]["clean_head_artifacts"] == 0


def test_scan_record_at_commit_stays_tbd_human_gated(tmp_path):
    gates = load_module()
    scan = {
        "artifact": "openpipestress.release_protected_content_scan_record",
        "git": {"commit_hash": COMMIT_A},
    }
    record = build_record(gates, tmp_path, scan_bodies=[scan],
                          sweep_bodies=[make_sweep_artifact()])

    by_id = {c["criterion_id"]: c for c in record["criteria"]}
    disposition = by_id["common.scan_disposition_recorded"]
    assert disposition["status"] == "TBD"
    assert disposition["tbd_reason"] == "human_gated"
    assert "SCAN_0.json" in disposition["notes"]


def test_records_validate_against_their_own_schema(tmp_path):
    gates = load_module()
    for family in EXPECTED_FAMILY_ORDER:
        record = build_record(gates, tmp_path, family=family,
                              sweep_bodies=[make_sweep_artifact()])
        assert gates.validate_record(record) == [], family


def test_schema_rejects_a_minted_release_label(tmp_path):
    gates = load_module()
    record = build_record(gates, tmp_path, sweep_bodies=[make_sweep_artifact()])
    record["release_labels"]["minted"] = True
    assert gates.validate_record(record), "a minted label must fail validation"


def test_schema_rejects_a_tbd_criterion_without_reason(tmp_path):
    gates = load_module()
    record = build_record(gates, tmp_path, sweep_bodies=[make_sweep_artifact()])
    tbd = next(c for c in record["criteria"] if c["status"] == "TBD")
    tbd["tbd_reason"] = None
    assert gates.validate_record(record), "TBD without a reason must fail"


def test_evaluation_is_deterministic_for_fixed_inputs(tmp_path):
    gates = load_module()
    first = build_record(gates, tmp_path / "a", sweep_bodies=[make_sweep_artifact()])
    second = build_record(gates, tmp_path / "b", sweep_bodies=[make_sweep_artifact()])
    # Same commit + same artifact set -> identical criteria and counts
    # (paths under inputs differ only by the temp directory).
    assert first["criteria"] == second["criteria"]
    assert first["status_counts"] == second["status_counts"]


def test_record_filename_binds_evaluated_commit_and_flags_emitting_tree():
    gates = load_module()
    dirty_git = {
        "commit_hash": COMMIT_B,
        "status_capture_failed": False,
        "working_tree_dirty": True,
    }
    name = gates.record_filename(
        "solver", "2026-07-11T01:00:00+00:00", COMMIT_A, dirty_git
    )
    assert name == f"GATE_SOLVER_20260711T010000Z_{'a' * 12}-dirty.json"


def test_family_selection_accepts_known_ids_and_rejects_unknown():
    gates = load_module()
    assert gates.select_families("gui") == ["gui"]
    assert gates.select_families("mixed,solver") == ["solver", "mixed"]
    assert gates.select_families("solver,nope") is None
    assert gates.select_families("") is None


def test_dec025_sweep_plan_is_unmodified_by_gate_records():
    """The five-surface gate is read as evidence, never changed."""
    gates = load_module()
    sweep = gates._sweep_module()
    surfaces = [surface.surface_id for surface in sweep.build_sweep_plan()]
    assert surfaces == SWEEP_SURFACES
