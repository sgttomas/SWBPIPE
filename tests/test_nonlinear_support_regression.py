#!/usr/bin/env python3
"""Focused regression checks for invented nonlinear support fixtures."""

import re
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BENCHMARK_DIR = ROOT / "validation" / "benchmarks" / "nonlinear"
SOURCE_PATH = BENCHMARK_DIR / "src" / "lib.rs"
HAND_CALCS_DIR = ROOT / "validation" / "hand_calcs" / "nonlinear"
HAND_CALCS_README = HAND_CALCS_DIR / "README.md"
BENCHMARK_README = BENCHMARK_DIR / "README.md"

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
    tolerance_assignments = re.findall(
        r"^\s+tolerance_policy:\s*([^,\n]+)", source, re.MULTILINE
    )
    assert tolerance_assignments
    assert set(tolerance_assignments) == {"None"}

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
    assert "release tolerances" in readme
    assert "CI gate" in readme
    assert "remain `TBD`" in readme

    for note_name in [
        *REQUIRED_FIXTURE_NOTES.values(),
        *REQUIRED_ASSEMBLED_FIXTURE_NOTES.values(),
    ]:
        note = (HAND_CALCS_DIR / note_name).read_text(encoding="utf-8")
        assert "| Quantity |" in note
        assert "Canonical dimension" in note
        assert "Tolerance policy: `TBD`." in note


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


def test_assembled_global_loop_seed_keeps_tbd_policy_visible():
    source = SOURCE_PATH.read_text(encoding="utf-8")

    assert "assembled_fixture_inventory" in source
    assert "solve_active_set_frame" in source
    assert "DEC-046-CV-B-assembled-validation-seed-TBD" in source
    assert "TolerancePolicyTbd" in source
    for fixture_id in REQUIRED_ASSEMBLED_FIXTURE_NOTES:
        assert fixture_id in source
