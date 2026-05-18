"""Pipe section and mass-property calculations from user-entered values.

The module does not provide dimensional tables, material defaults, unit
conversion constants, or catalog lookups. It requires explicit, compatible unit
metadata and reports diagnostics when inputs are insufficient.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import pi
from typing import Any


LENGTH_DIMENSIONS = {"length"}
DENSITY_DIMENSIONS = {"density"}
DIAGNOSTIC_SOURCE = "core/section_properties/calculator.py"


@dataclass(frozen=True)
class Quantity:
    magnitude: float
    unit: str
    dimension: str
    provenance: dict[str, Any]


@dataclass(frozen=True)
class SectionDiagnostic:
    code: str
    diagnostic_class: str
    severity: str
    field: str
    source: str
    affected_object: str
    message: str
    remediation: str
    provenance: dict[str, Any]


@dataclass(frozen=True)
class PipeSectionInput:
    outside_diameter: Quantity | None
    wall_thickness: Quantity | None
    corrosion_allowance: Quantity | None = None
    material_density: Quantity | None = None
    contents_density: Quantity | None = None
    insulation_thickness: Quantity | None = None
    insulation_density: Quantity | None = None


@dataclass(frozen=True)
class PipeSectionResult:
    properties: dict[str, Quantity]
    diagnostics: tuple[SectionDiagnostic, ...]

    @property
    def accepted(self) -> bool:
        return not any(item.severity == "blocking" for item in self.diagnostics)


def calculate_pipe_section_properties(inputs: PipeSectionInput) -> PipeSectionResult:
    """Calculate pipe section properties using explicit compatible units only."""

    diagnostics: list[SectionDiagnostic] = []
    od = _require_quantity(
        inputs.outside_diameter,
        field="outside_diameter",
        expected_dimensions=LENGTH_DIMENSIONS,
        diagnostics=diagnostics,
    )
    wall = _require_quantity(
        inputs.wall_thickness,
        field="wall_thickness",
        expected_dimensions=LENGTH_DIMENSIONS,
        diagnostics=diagnostics,
    )
    corrosion = _optional_quantity(
        inputs.corrosion_allowance,
        field="corrosion_allowance",
        expected_dimensions=LENGTH_DIMENSIONS,
        diagnostics=diagnostics,
    )
    insulation = _optional_quantity(
        inputs.insulation_thickness,
        field="insulation_thickness",
        expected_dimensions=LENGTH_DIMENSIONS,
        diagnostics=diagnostics,
    )

    length_inputs = [
        item
        for item in (
            ("outside_diameter", od),
            ("wall_thickness", wall),
            ("corrosion_allowance", corrosion),
            ("insulation_thickness", insulation),
        )
        if item[1] is not None
    ]
    length_unit = _single_unit(length_inputs, diagnostics=diagnostics)

    material_density = _optional_quantity(
        inputs.material_density,
        field="material_density",
        expected_dimensions=DENSITY_DIMENSIONS,
        diagnostics=diagnostics,
    )
    contents_density = _optional_quantity(
        inputs.contents_density,
        field="contents_density",
        expected_dimensions=DENSITY_DIMENSIONS,
        diagnostics=diagnostics,
    )
    insulation_density = _optional_quantity(
        inputs.insulation_density,
        field="insulation_density",
        expected_dimensions=DENSITY_DIMENSIONS,
        diagnostics=diagnostics,
    )
    density_unit = _single_unit(
        [
            item
            for item in (
                ("material_density", material_density),
                ("contents_density", contents_density),
                ("insulation_density", insulation_density),
            )
            if item[1] is not None
        ],
        diagnostics=diagnostics,
        required=False,
    )

    if diagnostics:
        return PipeSectionResult({}, tuple(diagnostics))

    assert od is not None
    assert wall is not None
    assert length_unit is not None

    corrosion_value = corrosion.magnitude if corrosion else 0.0
    insulation_value = insulation.magnitude if insulation else 0.0
    effective_wall = wall.magnitude - corrosion_value
    inside_diameter = od.magnitude - 2.0 * effective_wall

    if effective_wall <= 0:
        diagnostics.append(
            _blocking(
                "SECTION_CALCULATION_INPUT_INVALID",
                "corrosion_allowance",
                "Corrosion allowance leaves no positive effective wall thickness.",
                "Provide wall thickness greater than corrosion allowance.",
            )
        )
    if inside_diameter < 0:
        diagnostics.append(
            _blocking(
                "SECTION_CALCULATION_INPUT_INVALID",
                "wall_thickness",
                "Wall thickness and corrosion allowance produce a negative inside diameter.",
                "Provide dimensions where inside diameter is zero or positive.",
            )
        )
    if insulation_value < 0:
        diagnostics.append(
            _blocking(
                "SECTION_CALCULATION_INPUT_INVALID",
                "insulation_thickness",
                "Insulation thickness cannot be negative.",
                "Provide zero or positive insulation thickness.",
            )
        )
    if diagnostics:
        return PipeSectionResult({}, tuple(diagnostics))

    od2 = od.magnitude**2
    id2 = inside_diameter**2
    od4 = od.magnitude**4
    id4 = inside_diameter**4

    metal_area = pi / 4.0 * (od2 - id2)
    moment_of_inertia = pi / 64.0 * (od4 - id4)
    section_modulus = moment_of_inertia / (od.magnitude / 2.0)
    torsional_constant = pi / 32.0 * (od4 - id4)
    contents_volume_per_length = pi / 4.0 * id2

    properties = {
        "inside_diameter": _quantity(inside_diameter, length_unit, "length"),
        "metal_area": _quantity(metal_area, f"{length_unit}^2", "area"),
        "cross_section_area": _quantity(metal_area, f"{length_unit}^2", "area"),
        "moment_of_inertia": _quantity(
            moment_of_inertia,
            f"{length_unit}^4",
            "second_moment_area",
        ),
        "section_modulus": _quantity(
            section_modulus,
            f"{length_unit}^3",
            "section_modulus",
        ),
        "torsional_constant": _quantity(
            torsional_constant,
            f"{length_unit}^4",
            "second_moment_area",
        ),
        "contents_volume_per_length": _quantity(
            contents_volume_per_length,
            f"{length_unit}^2",
            "volume_per_length",
        ),
    }

    mass_parts: dict[str, float] = {}
    if material_density and density_unit:
        mass_parts["metal_mass_per_length"] = metal_area * material_density.magnitude
    if contents_density and density_unit:
        mass_parts["contents_mass_per_length"] = (
            contents_volume_per_length * contents_density.magnitude
        )
    if insulation and insulation_density and density_unit:
        insulation_od = od.magnitude + 2.0 * insulation.magnitude
        insulation_area = pi / 4.0 * (insulation_od**2 - od2)
        mass_parts["insulation_mass_per_length"] = (
            insulation_area * insulation_density.magnitude
        )

    if mass_parts and density_unit:
        mass_unit = f"{density_unit}*{length_unit}^2"
        for key, value in mass_parts.items():
            properties[key] = _quantity(value, mass_unit, "mass_per_length")
        properties["mass_per_length"] = _quantity(
            sum(mass_parts.values()),
            mass_unit,
            "mass_per_length",
        )

    return PipeSectionResult(properties, ())


def quantity_from_mapping(value: dict[str, Any] | None) -> Quantity | None:
    """Build a calculator quantity from schema-like value metadata."""

    if not isinstance(value, dict):
        return None
    unit = value.get("unit") or value.get("unit_ref")
    dimension = value.get("dimension") or value.get("dimension_id")
    magnitude = value.get("magnitude")
    provenance = value.get("provenance")
    if magnitude is None or unit is None or dimension is None or provenance is None:
        return None
    return Quantity(
        magnitude=float(magnitude),
        unit=str(unit),
        dimension=str(dimension),
        provenance=provenance,
    )


def _require_quantity(
    value: Quantity | None,
    *,
    field: str,
    expected_dimensions: set[str],
    diagnostics: list[SectionDiagnostic],
) -> Quantity | None:
    if value is None:
        diagnostics.append(
            _blocking(
                "SECTION_DIMENSION_MISSING",
                field,
                f"Required {field} is missing.",
                "Provide the user-entered value with unit and provenance metadata.",
            )
        )
        return None
    _validate_quantity(value, field, expected_dimensions, diagnostics)
    return value


def _optional_quantity(
    value: Quantity | None,
    *,
    field: str,
    expected_dimensions: set[str],
    diagnostics: list[SectionDiagnostic],
) -> Quantity | None:
    if value is None:
        return None
    _validate_quantity(value, field, expected_dimensions, diagnostics)
    return value


def _validate_quantity(
    value: Quantity,
    field: str,
    expected_dimensions: set[str],
    diagnostics: list[SectionDiagnostic],
) -> None:
    if value.unit == "" or value.dimension == "":
        diagnostics.append(
            _blocking(
                "SECTION_UNIT_MISSING",
                field,
                f"{field} is missing explicit unit or dimension metadata.",
                "Carry unit and dimension metadata through the calculation boundary.",
            )
        )
    if not value.provenance:
        diagnostics.append(
            _blocking(
                "SECTION_PROVENANCE_MISSING",
                field,
                f"{field} is missing explicit provenance metadata.",
                "Carry source provenance through the calculation boundary.",
            )
        )
    if value.dimension not in expected_dimensions:
        diagnostics.append(
            _blocking(
                "SECTION_DIMENSION_INCONSISTENT",
                field,
                f"{field} has dimension {value.dimension}, which is incompatible.",
                "Provide a dimensionally compatible quantity.",
            )
        )
    if value.magnitude <= 0 and field not in {"corrosion_allowance", "insulation_thickness"}:
        diagnostics.append(
            _blocking(
                "SECTION_CALCULATION_INPUT_INVALID",
                field,
                f"{field} must be positive.",
                "Provide a positive user-entered value.",
            )
        )
    if value.magnitude < 0:
        diagnostics.append(
            _blocking(
                "SECTION_CALCULATION_INPUT_INVALID",
                field,
                f"{field} cannot be negative.",
                "Provide zero or positive value where the field is optional.",
            )
        )


def _single_unit(
    values: list[tuple[str, Quantity | None]],
    *,
    diagnostics: list[SectionDiagnostic],
    required: bool = True,
) -> str | None:
    present = [(field, value) for field, value in values if value is not None]
    if not present:
        return None
    unit = present[0][1].unit
    mismatched = [field for field, value in present if value.unit != unit]
    if mismatched:
        diagnostics.append(
            _blocking(
                "SECTION_DIMENSION_INCONSISTENT",
                ",".join(mismatched),
                "Section calculator does not convert between units in this deliverable.",
                "Normalize compatible values before calculation or wait for approved unit conversion support.",
            )
        )
    if required and unit == "":
        diagnostics.append(
            _blocking(
                "SECTION_UNIT_MISSING",
                present[0][0],
                "Required unit metadata is empty.",
                "Provide explicit unit metadata.",
            )
        )
    return unit


def _quantity(magnitude: float, unit: str, dimension: str) -> Quantity:
    return Quantity(
        magnitude=magnitude,
        unit=unit,
        dimension=dimension,
        provenance={
            "source_name": "OpenPipeStress section property calculator",
            "source_location": "core/section_properties/calculator.py",
            "source_license": "project_source",
            "contributor": "OpenPipeStress",
            "contributor_certification": "calculated from user-entered dimensions; no catalog value supplied",
            "redistribution_status": "derived_from_user_input",
            "review_status": "calculated",
        },
    )


def _blocking(
    code: str,
    field: str,
    message: str,
    remediation: str,
) -> SectionDiagnostic:
    return SectionDiagnostic(
        code=code,
        diagnostic_class="SOLVE_BLOCKING",
        severity="blocking",
        field=field,
        source=DIAGNOSTIC_SOURCE,
        affected_object=field,
        message=message,
        remediation=remediation,
        provenance={
            "source_name": "OpenPipeStress section property calculator",
            "source_location": DIAGNOSTIC_SOURCE,
            "source_license": "project_source",
            "contributor": "OpenPipeStress",
            "contributor_certification": "calculator diagnostic generated from explicit input validation",
            "redistribution_status": "derived_from_user_input",
            "review_status": "calculated",
        },
    )
