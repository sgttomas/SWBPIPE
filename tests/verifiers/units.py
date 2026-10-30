"""The unit conversions the retained verifier uses, in Python: each quantity to its
dimension's canonical SI unit.

A transcription of `core/units`'s closed catalog (symbol, dimension and transform
to canonical, with each factor and offset formed by the same binary64
expression), its symbol lookup (`unit_by_symbol`, including the legacy `C` and the
semantic-dimension bindings) and `convert_for_dimension`'s arithmetic: identity
returns the value's own bits; otherwise `value * factor + offset`, then the
canonical unit's inverse. `convert_quantities_to_canonical` has the batch
contract of `core/units/adapter.py`, without its subprocess.

Test-only and independent: it calls no crate the producer links.
`tests/test_verifier_independence.py` checks it against that crate's committed
witnesses, and the producer-solved successors' G8 replay checks the converted
bits the verifier forms.
"""
from __future__ import annotations

from collections.abc import Mapping, Sequence
import math
from typing import Any, NamedTuple

INCH = 0.0254
FOOT = 0.3048
POUND_MASS = 0.45359237
POUND_FORCE = 4.4482216152605
SQUARE_INCH = 0.00064516
PSI = POUND_FORCE / SQUARE_INCH
KSI = 1_000.0 * PSI
CELSIUS_ZERO_K = 273.15
FAHRENHEIT_ZERO_R = 459.67


class Unit(NamedTuple):
    symbol: str
    dimension: str
    factor: float
    offset: float = 0.0


# The crate's CATALOG, in its order (lookup takes the first match).
CATALOG = (
    Unit("kN/m", "force_per_length", 1_000.0),
    Unit("kN", "force", 1_000.0),
    Unit("kN*m", "moment", 1_000.0),
    Unit("GPa", "pressure", 1_000_000_000.0),
    Unit("bar", "pressure", 100_000.0),
    Unit("N/mm", "force_per_length", 1_000.0),
    Unit("kN/mm", "force_per_length", 1_000_000.0),
    Unit("1/degF", "thermal_expansion_coefficient", 9.0 / 5.0),
    Unit("1", "dimensionless", 1.0),
    Unit("m", "length", 1.0),
    Unit("mm", "length", 0.001),
    Unit("in", "length", INCH),
    Unit("ft", "length", FOOT),
    Unit("kg", "mass", 1.0),
    Unit("lb", "mass", POUND_MASS),
    Unit("s", "time", 1.0),
    Unit("K", "temperature", 1.0),
    Unit("degC", "temperature", 1.0, CELSIUS_ZERO_K),
    Unit("degF", "temperature", 5.0 / 9.0, FAHRENHEIT_ZERO_R * 5.0 / 9.0),
    Unit("degR", "temperature", 5.0 / 9.0),
    Unit("K", "temperature_interval", 1.0),
    Unit("degC", "temperature_interval", 1.0),
    Unit("degF", "temperature_interval", 5.0 / 9.0),
    Unit("degR", "temperature_interval", 5.0 / 9.0),
    Unit("rad", "angle", 1.0),
    Unit("deg", "angle", math.pi / 180.0),
    Unit("N", "force", 1.0),
    Unit("lbf", "force", POUND_FORCE),
    Unit("N/m", "force_per_length", 1.0),
    Unit("lbf/ft", "force_per_length", POUND_FORCE / FOOT),
    Unit("lbf/in", "force_per_length", POUND_FORCE / INCH),
    Unit("N*m", "moment", 1.0),
    Unit("lbf*in", "moment", POUND_FORCE * INCH),
    Unit("lbf*ft", "moment", POUND_FORCE * FOOT),
    Unit("Pa", "pressure", 1.0),
    Unit("kPa", "pressure", 1_000.0),
    Unit("MPa", "pressure", 1_000_000.0),
    Unit("psi", "pressure", PSI),
    Unit("ksi", "pressure", KSI),
    Unit("m^2", "area", 1.0),
    Unit("mm^2", "area", 1.0e-6),
    Unit("in^2", "area", SQUARE_INCH),
    Unit("m^3", "volume", 1.0),
    Unit("in^3", "volume", INCH * INCH * INCH),
    Unit("kg/m^3", "density", 1.0),
    Unit("lb/in^3", "density", POUND_MASS / (INCH * INCH * INCH)),
    Unit("N/m", "linear_stiffness", 1.0),
    Unit("N*m/rad", "rotational_stiffness", 1.0),
    Unit("m/s", "velocity", 1.0),
    Unit("m/s^2", "acceleration", 1.0),
    Unit("W/(m*K)", "thermal_conductivity", 1.0),
    Unit("J/(kg*K)", "specific_heat", 1.0),
    Unit("1/K", "thermal_expansion_coefficient", 1.0),
    Unit("1/degC", "thermal_expansion_coefficient", 1.0),
    Unit("m^4", "second_moment_area", 1.0),
    Unit("mm^4", "second_moment_area", 1.0e-12),
    Unit("in^4", "second_moment_area", INCH * INCH * INCH * INCH),
    Unit("m^3", "section_modulus", 1.0),
    Unit("mm^3", "section_modulus", 1.0e-9),
    Unit("in^3", "section_modulus", INCH * INCH * INCH),
    Unit("kg/m", "mass_per_length", 1.0),
    Unit("lb/ft", "mass_per_length", POUND_MASS / FOOT),
    Unit("m^3/m", "volume_per_length", 1.0),
)

# canonical_unit(dimension), as a catalog entry; TBD has none.
CANONICAL = {
    "dimensionless": ("1", "dimensionless"), "slope": ("1", "dimensionless"),
    "length": ("m", "length"), "displacement": ("m", "length"), "mass": ("kg", "mass"), "time": ("s", "time"),
    "temperature": ("K", "temperature"), "temperature_interval": ("K", "temperature_interval"),
    "angle": ("rad", "angle"), "rotation": ("rad", "angle"), "force": ("N", "force"),
    "force_per_length": ("N/m", "force_per_length"), "moment": ("N*m", "moment"),
    "pressure": ("Pa", "pressure"), "stress": ("Pa", "pressure"), "area": ("m^2", "area"), "volume": ("m^3", "volume"),
    "density": ("kg/m^3", "density"), "linear_stiffness": ("N/m", "linear_stiffness"),
    "rotational_stiffness": ("N*m/rad", "rotational_stiffness"), "velocity": ("m/s", "velocity"),
    "acceleration": ("m/s^2", "acceleration"), "thermal_conductivity": ("W/(m*K)", "thermal_conductivity"),
    "specific_heat": ("J/(kg*K)", "specific_heat"),
    "thermal_expansion_coefficient": ("1/K", "thermal_expansion_coefficient"),
    "second_moment_area": ("m^4", "second_moment_area"), "section_modulus": ("m^3", "section_modulus"),
    "mass_per_length": ("kg/m", "mass_per_length"), "volume_per_length": ("m^3/m", "volume_per_length"),
}
DIMENSIONS = frozenset(CANONICAL) | {"TBD"}
# unit_supports_dimension: a unit of the first dimension serves the second.
_SERVES = {("length", "displacement"), ("pressure", "stress"), ("angle", "rotation"), ("dimensionless", "slope"),
           ("area", "volume_per_length"), ("force_per_length", "linear_stiffness")}


def _supports(unit_dimension: str, requested: str) -> bool:
    return unit_dimension == requested or (unit_dimension, requested) in _SERVES


def unit_by_symbol(symbol: str, dimension: str) -> Unit:
    if symbol == "C" and dimension in ("temperature", "temperature_interval"):
        symbol = "degC"
    for unit in CATALOG:
        if unit.symbol == symbol and unit.dimension == dimension:
            return unit
    for unit in CATALOG:
        if unit.symbol == symbol and _supports(unit.dimension, dimension):
            return unit
    raise ValueError(f"UNITS_CONVERSION_REFUSED: {symbol} is not in the catalog for {dimension}")


def _canonical(dimension: str) -> Unit:
    symbol, unit_dimension = CANONICAL[dimension]
    return next(unit for unit in CATALOG if unit.symbol == symbol and unit.dimension == unit_dimension)


def to_canonical(value: float, unit: str, dimension: str) -> float:
    """One quantity in its dimension's canonical unit (the bridge's per-quantity step)."""
    if dimension not in DIMENSIONS:
        raise ValueError("UNITS_DIMENSION_UNSUPPORTED")
    source = unit_by_symbol(unit, dimension)
    if dimension not in CANONICAL:
        raise ValueError("UNITS_CANONICAL_UNIT_UNAVAILABLE")
    target = _canonical(dimension)
    value = float(value)
    if not math.isfinite(value):
        raise ValueError(f"UNITS_CONVERSION_REFUSED: quantity must be finite, got {value}")
    if source is target:
        return value
    result = ((value * source.factor + source.offset) - target.offset) / target.factor
    if not math.isfinite(result):
        raise ValueError("UNITS_CONVERSION_NONFINITE")
    return result


def convert_quantities_to_canonical(quantities: Sequence[Mapping[str, Any]]) -> list[dict[str, Any]]:
    """`core/units/adapter.py`'s batch contract: `{id, value, unit, dimension}` in, the
    same with the canonical value and unit symbol out, in order."""
    rows = []
    ids = set()
    for item in quantities:
        if not isinstance(item, Mapping) or set(item) != {"id", "value", "unit", "dimension"}:
            raise ValueError("UNITS-REQUEST-SHAPE")
        if any(not isinstance(item[key], str) or not item[key] for key in ("id", "unit", "dimension")) or item["id"] in ids:
            raise ValueError("UNITS-REQUEST-IDENTITY")
        try:
            finite = type(item["value"]) in (int, float) and math.isfinite(item["value"])
        except OverflowError:
            finite = False
        if not finite:
            raise ValueError("UNITS-REQUEST-NONFINITE")
        ids.add(item["id"])
        rows.append(item)
    out = []
    for item in rows:
        value = to_canonical(item["value"], item["unit"], item["dimension"])
        out.append({"id": item["id"], "value": value, "unit": _canonical(item["dimension"]).symbol, "dimension": item["dimension"]})
    return out
