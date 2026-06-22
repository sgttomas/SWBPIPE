#!/usr/bin/env python3
"""Focused regression checks for invented nonlinear support fixtures."""

import json
import re
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BENCHMARK_DIR = ROOT / "validation" / "benchmarks" / "nonlinear"
SOURCE_PATH = BENCHMARK_DIR / "src" / "lib.rs"
HAND_CALCS_DIR = ROOT / "validation" / "hand_calcs" / "nonlinear"
HAND_CALCS_README = HAND_CALCS_DIR / "README.md"
BENCHMARK_README = BENCHMARK_DIR / "README.md"
CONVERGENCE_OBSERVATION_NOTE = HAND_CALCS_DIR / "convergence_observations.md"
CONVERGENCE_POLICY_RECORD = BENCHMARK_DIR / "convergence_policy.dec046.json"
DEC_046_POLICY_REF = "DEC-046-CV-B-active-set-count-validation-v1"
FREE_DOF_FORCE_MOMENT_POLICY_RECORD = BENCHMARK_DIR / "free_dof_force_moment_policy.dec046.json"
DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF = (
    "DEC-046-CV-B-free-dof-force-moment-residual-validation-v1"
)

REQUIRED_FAMILIES = {
    "ActiveSet",
    "Gap",
    "LiftOff",
    "Friction",
    "NonConvergence",
}

REQUIRED_FIXTURE_NOTES = {
    "NL-ACTIVE-ONE-WAY-ORIGINAL": "active_set_one_way.md",
    "NL-GAP-CLOSURE-ORIGINAL": "gap_closure.md",
    "NL-LIFT-OFF-ORIGINAL": "lift_off.md",
    "NL-FRICTION-STICK-SLIDE-ORIGINAL": "friction_transition.md",
    "NL-NONCONVERGENCE-LIMIT-ORIGINAL": "unresolved_nonconvergence.md",
}

REQUIRED_ASSEMBLED_FIXTURE_NOTES = {
    "NL-ASSEMBLED-ONE-WAY-DEACTIVATE-ORIGINAL": "assembled_one_way_deactivation.md",
    "NL-ASSEMBLED-GAP-CLOSURE-ORIGINAL": "assembled_gap_closure.md",
    "NL-ASSEMBLED-LIFT-OFF-ORIGINAL": "assembled_lift_off.md",
    "NL-ASSEMBLED-FRICTION-STICK-ORIGINAL": "assembled_friction_sticking.md",
    "NL-ASSEMBLED-FRICTION-SLIDE-ORIGINAL": "assembled_friction_sliding.md",
    "NL-ASSEMBLED-FRICTION-DERIVED-NORMAL-ORIGINAL": "assembled_friction_derived_normal.md",
}

REQUIRED_UNIT_BASIS_LINES = {
    "Translational support displacement and clearance | `mm` | length",
    "Translational support reaction | `N` | force",
    "Rotational support reaction | `N-m` | moment",
    "Friction coefficient | `ratio` | dimensionless",
    "Active-set residual and iteration counts | `count` | dimensionless",
}

BOUNDARY_PHRASES = {
    "project-original-public-content",
    "invented support states",
    "not copied from protected standards",
    "commercial software examples",
    "proprietary data",
    "private data",
    "real project records",
}

FORBIDDEN_TERMS = {
    "AS" + "ME",
    "B" + "31",
    "B" + "31J",
    "allowable stress " + "table",
    "stress intensification factor " + "table",
    "vendor catalog " + "value",
    "real " + "se" + "cret",
    "cert" + "ified",
    "sea" + "led",
    "code-compliant",
    "professional approval",
    "licensed engineer approval",
    "suitable for professional reliance",
}

CANONICAL_DIMENSIONS_RE = re.compile(
    r"const\s+CANONICAL_DIMENSIONS:\s*&\[\&str\]\s*=\s*&\[(?P<body>.*?)\];",
    re.DOTALL,
)


def _canonical_dimensions(source: str) -> set[str]:
    match = CANONICAL_DIMENSIONS_RE.search(source)
    assert match is not None
    return set(re.findall(r'"([^"]+)"', match.group("body")))


def _normalized_text(text: str) -> str:
    return " ".join(text.split())


def test_nonlinear_benchmark_crate_runs_focused_regressions():
    result = subprocess.run(
        ["cargo", "test", "--quiet"],
        cwd=BENCHMARK_DIR,
        check=False,
        text=True,
        capture_output=True,
    )

    assert result.returncode == 0, result.stdout + result.stderr


def test_nonlinear_fixture_catalog_is_bounded_and_invented():
    source = SOURCE_PATH.read_text(encoding="utf-8")

    for family in REQUIRED_FAMILIES:
        assert family in source

    for phrase in BOUNDARY_PHRASES:
        assert phrase in source

    assert "PKG09-NONLINEAR-FIXTURE-UNITS-EXPLICIT-MM-N-NM" in source
    assert "unit catalog remains TBD" in source
    assert "tolerance_policy: None" in source
    assert "Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF)" in source
    tolerance_assignments = re.findall(
        r"^\s+tolerance_policy:\s*([^,\n]+)", source, re.MULTILINE
    )
    assert tolerance_assignments
    assert set(tolerance_assignments) == {
        "None",
        "Some(DEC_046_ACTIVE_SET_COUNT_POLICY_REF)",
    }

    lowered_source = source.lower()
    for term in FORBIDDEN_TERMS:
        assert term.lower() not in lowered_source


def test_nonlinear_fixture_notes_cover_each_public_original_fixture():
    source = SOURCE_PATH.read_text(encoding="utf-8")
    readme = HAND_CALCS_README.read_text(encoding="utf-8")

    for fixture_id, note_name in {
        **REQUIRED_FIXTURE_NOTES,
        **REQUIRED_ASSEMBLED_FIXTURE_NOTES,
    }.items():
        note_path = HAND_CALCS_DIR / note_name
        assert note_path.is_file(), note_name

        note = note_path.read_text(encoding="utf-8")
        source_location = f"validation/hand_calcs/nonlinear/{note_name}"

        assert fixture_id in source
        assert fixture_id in readme
        assert fixture_id in note
        assert source_location in source
        assert f"[{note_name}]({note_name})" in readme
        assert "## Provenance" in note
        assert "## Invented Inputs" in note
        assert "## Expected Values" in note

        normalized_note = _normalized_text(note)
        for phrase in BOUNDARY_PHRASES:
            assert phrase in normalized_note


def test_nonlinear_hand_calc_unit_basis_is_explicit_and_unresolved():
    readme = HAND_CALCS_README.read_text(encoding="utf-8")
    benchmark_readme = BENCHMARK_README.read_text(encoding="utf-8")

    for required_line in REQUIRED_UNIT_BASIS_LINES:
        assert required_line in readme

    assert "PKG09-NONLINEAR-FIXTURE-UNITS-EXPLICIT-MM-N-NM" in benchmark_readme
    assert "fixture-local basis" in benchmark_readme
    normalized_benchmark_readme = _normalized_text(benchmark_readme)
    assert "does not define project conversion constants" in normalized_benchmark_readme
    assert "canonical unit catalog, which remain `TBD`" in normalized_benchmark_readme
    assert "free-DOF force/moment equilibrium residuals" in readme
    assert "CI gate" in readme
    assert "remain `TBD`" in _normalized_text(readme)
    assert DEC_046_POLICY_REF in readme

    for note_name in REQUIRED_FIXTURE_NOTES.values():
        note = (HAND_CALCS_DIR / note_name).read_text(encoding="utf-8")
        assert "| Quantity |" in note
        assert "Canonical dimension" in note
        assert "Tolerance policy: `TBD`." in note

    for note_name in REQUIRED_ASSEMBLED_FIXTURE_NOTES.values():
        note = (HAND_CALCS_DIR / note_name).read_text(encoding="utf-8")
        assert "| Quantity |" in note
        assert "Canonical dimension" in note
        assert f"Tolerance policy: `{DEC_046_POLICY_REF}`." in note


def test_nonlinear_validation_artifacts_avoid_protected_and_claim_terms():
    scanned_paths = [
        SOURCE_PATH,
        BENCHMARK_README,
        HAND_CALCS_README,
        *(
            HAND_CALCS_DIR / note_name
            for note_name in [
                *REQUIRED_FIXTURE_NOTES.values(),
                *REQUIRED_ASSEMBLED_FIXTURE_NOTES.values(),
            ]
        ),
        CONVERGENCE_OBSERVATION_NOTE,
        CONVERGENCE_POLICY_RECORD,
        FREE_DOF_FORCE_MOMENT_POLICY_RECORD,
    ]

    for path in scanned_paths:
        lowered_text = path.read_text(encoding="utf-8").lower()
        for term in FORBIDDEN_TERMS:
            assert term.lower() not in lowered_text, f"{term!r} appears in {path}"


def test_nonlinear_dimension_allowlist_includes_force_per_length():
    source = SOURCE_PATH.read_text(encoding="utf-8")

    assert "force_per_length" in _canonical_dimensions(source)


def test_nonlinear_public_provenance_sources_exist_before_fixture_acceptance():
    source = SOURCE_PATH.read_text(encoding="utf-8")
    source_locations = {
        line.split('"')[1]
        for line in source.splitlines()
        if 'validation/hand_calcs/nonlinear/' in line and line.strip().startswith('"')
    }

    assert source_locations
    for source_location in source_locations:
        assert (ROOT / source_location).is_file(), source_location


def test_assembled_global_loop_seed_uses_governed_policy():
    source = SOURCE_PATH.read_text(encoding="utf-8")
    benchmark_readme = BENCHMARK_README.read_text(encoding="utf-8")
    hand_calc_readme = HAND_CALCS_README.read_text(encoding="utf-8")
    observation_note = CONVERGENCE_OBSERVATION_NOTE.read_text(encoding="utf-8")
    policy_record = json.loads(CONVERGENCE_POLICY_RECORD.read_text(encoding="utf-8"))
    force_moment_policy_record = json.loads(
        FREE_DOF_FORCE_MOMENT_POLICY_RECORD.read_text(encoding="utf-8")
    )

    assert "assembled_fixture_inventory" in source
    assert "assembled_convergence_observations" in source
    assert "assembled_force_displacement_residual_observations" in source
    assert "ConvergenceObservation" in source
    assert "ForceDisplacementResidualObservation" in source
    assert "observed_iteration_count" in source
    assert "free_dof_force_moment_threshold_policy" in source
    assert DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF in source
    assert "governed_convergence_policy_entries" in source
    assert "governed_free_dof_force_moment_policy_entries" in source
    assert "solve_active_set_frame" in source
    assert "DEC_046_ACTIVE_SET_COUNT_POLICY_REF" in source
    assert "ConvergencePolicyStatus::Accepted" in source
    for fixture_id in REQUIRED_ASSEMBLED_FIXTURE_NOTES:
        assert fixture_id in source
        assert fixture_id in observation_note

    assert CONVERGENCE_OBSERVATION_NOTE.name in benchmark_readme
    assert CONVERGENCE_OBSERVATION_NOTE.name in hand_calc_readme
    assert CONVERGENCE_POLICY_RECORD.name in benchmark_readme
    assert FREE_DOF_FORCE_MOMENT_POLICY_RECORD.name in benchmark_readme
    assert "## Provenance" in observation_note
    assert "## Invented Inputs" in observation_note
    assert "## Active-Set Expected Values" in observation_note
    assert "## Force/Displacement Residual Observations" in observation_note
    assert f"Tolerance policy: `{DEC_046_POLICY_REF}`." in observation_note
    assert f"Free-DOF force/moment residual policy: `{DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF}`." in observation_note
    assert "changed-support-count residual" in benchmark_readme
    normalized_observation_note = _normalized_text(observation_note).lower()
    assert "force/displacement residual" in normalized_observation_note
    assert "threshold policy" in normalized_observation_note
    assert DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF.lower() in normalized_observation_note
    assert "displacement and reaction deltas remain observation-only" in normalized_observation_note

    assert policy_record["record_id"] == DEC_046_POLICY_REF
    assert policy_record["decision_ref"] == "DEC-046"
    assert policy_record["residual_basis"]["name"] == "active_set_changed_support_count"
    assert policy_record["residual_basis"]["unit"] == "count"
    assert policy_record["residual_basis"]["dimension"] == "dimensionless"
    assert [entry["nonlinear_class"] for entry in policy_record["entries"]] == [
        "one_way",
        "gap",
        "lift_off",
        "friction",
    ]
    for entry in policy_record["entries"]:
        assert entry["policy_ref"] == DEC_046_POLICY_REF
        assert entry["relative_residual_tolerance"] == 0.0
        assert entry["absolute_residual_floor"] == 0.0
        assert entry["max_iterations"] == 4
        assert entry["evidence_fixture_ids"]

    assert force_moment_policy_record["record_id"] == DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF
    assert force_moment_policy_record["decision_ref"] == "DEC-046"
    assert force_moment_policy_record["status"] == "accepted_for_current_assembled_validation_seed"
    assert force_moment_policy_record["force_residual_basis"]["name"] == "free_dof_force_residual"
    assert force_moment_policy_record["force_residual_basis"]["unit"] == "N"
    assert force_moment_policy_record["moment_residual_basis"]["name"] == "free_dof_moment_residual"
    assert force_moment_policy_record["moment_residual_basis"]["unit"] == "N-m"
    assert [entry["nonlinear_class"] for entry in force_moment_policy_record["entries"]] == [
        "one_way",
        "gap",
        "lift_off",
        "friction",
    ]
    for entry in force_moment_policy_record["entries"]:
        assert entry["policy_ref"] == DEC_046_FREE_DOF_FORCE_MOMENT_POLICY_REF
        assert entry["force_absolute_limit"] == 0.0
        assert entry["moment_absolute_limit"] == 0.0
        assert entry["evidence_fixture_ids"]
