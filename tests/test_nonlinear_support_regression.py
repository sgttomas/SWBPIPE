#!/usr/bin/env python3
"""Focused regression checks for invented nonlinear support fixtures."""

import re
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
BENCHMARK_DIR = ROOT / "validation" / "benchmarks" / "nonlinear"
SOURCE_PATH = BENCHMARK_DIR / "src" / "lib.rs"

REQUIRED_FAMILIES = {
    "ActiveSet",
    "Gap",
    "LiftOff",
    "Friction",
    "NonConvergence",
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
}

CANONICAL_DIMENSIONS_RE = re.compile(
    r"const\s+CANONICAL_DIMENSIONS:\s*&\[\&str\]\s*=\s*&\[(?P<body>.*?)\];",
    re.DOTALL,
)


def _canonical_dimensions(source: str) -> set[str]:
    match = CANONICAL_DIMENSIONS_RE.search(source)
    assert match is not None
    return set(re.findall(r'"([^"]+)"', match.group("body")))


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

    assert "project-original-public-content" in source
    assert "invented support states" in source
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
