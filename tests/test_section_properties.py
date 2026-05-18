#!/usr/bin/env python3
"""Checks for pipe section and mass-property calculations."""

from math import isclose, pi
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from core.section_properties.calculator import (  # noqa: E402
    PipeSectionInput,
    Quantity,
    calculate_pipe_section_properties,
    quantity_from_mapping,
)


PROVENANCE = {
    "source_name": "Invented section property test",
    "source_location": "tests/test_section_properties.py",
    "source_license": "project_test",
    "contributor": "OpenPipeStress",
    "contributor_certification": "invented non-engineering test value",
    "redistribution_status": "TBD",
    "review_status": "pending",
}

CANONICAL_OUTPUT_DIMENSIONS = {
    "length",
    "area",
    "second_moment_area",
    "section_modulus",
    "volume_per_length",
    "mass_per_length",
}


def q(value, unit="m", dimension="length"):
    return Quantity(value, unit, dimension, PROVENANCE)


def codes(result):
    return {finding.code for finding in result.diagnostics}


def test_calculates_pipe_section_properties_from_user_dimensions():
    result = calculate_pipe_section_properties(
        PipeSectionInput(
            outside_diameter=q(10.0),
            wall_thickness=q(1.0),
        )
    )

    assert result.accepted is True
    props = result.properties
    assert isclose(props["inside_diameter"].magnitude, 8.0)
    assert isclose(props["metal_area"].magnitude, pi / 4.0 * (10.0**2 - 8.0**2))
    assert isclose(
        props["moment_of_inertia"].magnitude,
        pi / 64.0 * (10.0**4 - 8.0**4),
    )
    assert isclose(
        props["section_modulus"].magnitude,
        props["moment_of_inertia"].magnitude / 5.0,
    )
    assert props["cross_section_area"].dimension == "area"
    assert props["cross_section_area"].unit == "m^2"
    assert props["moment_of_inertia"].dimension == "second_moment_area"
    assert props["torsional_constant"].dimension == "second_moment_area"
    assert props["section_modulus"].dimension == "section_modulus"
    assert props["contents_volume_per_length"].dimension == "volume_per_length"
    assert {item.dimension for item in props.values()} <= CANONICAL_OUTPUT_DIMENSIONS
    assert (
        props["cross_section_area"]
        .provenance["contributor_certification"]
        .startswith("calculated from user-entered")
    )


def test_corrosion_allowance_reduces_effective_wall():
    result = calculate_pipe_section_properties(
        PipeSectionInput(
            outside_diameter=q(10.0),
            wall_thickness=q(1.0),
            corrosion_allowance=q(0.25),
        )
    )

    assert result.accepted is True
    assert isclose(result.properties["inside_diameter"].magnitude, 8.5)


def test_mass_per_length_uses_only_supplied_densities():
    result = calculate_pipe_section_properties(
        PipeSectionInput(
            outside_diameter=q(10.0),
            wall_thickness=q(1.0),
            material_density=q(2.0, "kg/m^3", "density"),
            contents_density=q(3.0, "kg/m^3", "density"),
            insulation_thickness=q(0.5),
            insulation_density=q(4.0, "kg/m^3", "density"),
        )
    )

    assert result.accepted is True
    props = result.properties
    metal_area = pi / 4.0 * (10.0**2 - 8.0**2)
    contents_area = pi / 4.0 * 8.0**2
    insulation_area = pi / 4.0 * (11.0**2 - 10.0**2)
    expected = metal_area * 2.0 + contents_area * 3.0 + insulation_area * 4.0
    assert isclose(props["mass_per_length"].magnitude, expected)
    assert props["mass_per_length"].dimension == "mass_per_length"


def test_missing_dimensions_are_blocking_findings_not_defaults():
    result = calculate_pipe_section_properties(
        PipeSectionInput(outside_diameter=q(10.0), wall_thickness=None)
    )

    assert result.accepted is False
    assert result.properties == {}
    assert "SECTION_DIMENSION_MISSING" in codes(result)
    diagnostic = result.diagnostics[0]
    assert diagnostic.diagnostic_class == "SOLVE_BLOCKING"
    assert diagnostic.source == "core/section_properties/calculator.py"
    assert diagnostic.affected_object == "wall_thickness"
    assert diagnostic.provenance["source_location"] == "core/section_properties/calculator.py"


def test_missing_provenance_is_blocking_finding():
    result = calculate_pipe_section_properties(
        PipeSectionInput(
            outside_diameter=Quantity(10.0, "m", "length", {}),
            wall_thickness=q(1.0),
        )
    )

    assert result.accepted is False
    assert result.properties == {}
    assert "SECTION_PROVENANCE_MISSING" in codes(result)


def test_mixed_units_are_rejected_without_hidden_conversion():
    result = calculate_pipe_section_properties(
        PipeSectionInput(
            outside_diameter=q(10.0, "in", "length"),
            wall_thickness=q(1.0, "mm", "length"),
        )
    )

    assert result.accepted is False
    assert result.properties == {}
    assert "SECTION_DIMENSION_INCONSISTENT" in codes(result)


def test_invalid_geometry_is_rejected():
    result = calculate_pipe_section_properties(
        PipeSectionInput(
            outside_diameter=q(10.0),
            wall_thickness=q(0.5),
            corrosion_allowance=q(0.5),
        )
    )

    assert result.accepted is False
    assert "SECTION_CALCULATION_INPUT_INVALID" in codes(result)


def test_schema_like_quantity_mapping_requires_unit_metadata():
    assert (
        quantity_from_mapping(
            {
                "magnitude": 10.0,
                "unit": "m",
                "dimension": "length",
                "provenance": PROVENANCE,
            }
        )
        == q(10.0)
    )
    assert quantity_from_mapping({"magnitude": 10.0, "dimension": "length"}) is None
    assert (
        quantity_from_mapping({"magnitude": 10.0, "unit": "m", "dimension": "length"})
        is None
    )


if __name__ == "__main__":
    test_calculates_pipe_section_properties_from_user_dimensions()
    test_corrosion_allowance_reduces_effective_wall()
    test_mass_per_length_uses_only_supplied_densities()
    test_missing_dimensions_are_blocking_findings_not_defaults()
    test_missing_provenance_is_blocking_finding()
    test_mixed_units_are_rejected_without_hidden_conversion()
    test_invalid_geometry_is_rejected()
    test_schema_like_quantity_mapping_requires_unit_metadata()
