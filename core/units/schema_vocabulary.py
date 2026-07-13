"""Read-only Python access to the PKG-02 canonical unit schema vocabulary."""

from __future__ import annotations

import json
from functools import lru_cache
from pathlib import Path


UNITS_SCHEMA_PATH = Path(__file__).resolve().parents[2] / "schemas" / "units.schema.yaml"


@lru_cache(maxsize=1)
def canonical_dimension_ids() -> frozenset[str]:
    """Return DimensionId values from the accepted schema, without a mirror list."""

    schema = json.loads(UNITS_SCHEMA_PATH.read_text(encoding="utf-8"))
    values = schema["$defs"]["DimensionId"]["enum"]
    if not isinstance(values, list) or not values or not all(
        isinstance(value, str) and value for value in values
    ):
        raise ValueError("units.schema.yaml DimensionId enum is missing or malformed")
    if len(values) != len(set(values)):
        raise ValueError("units.schema.yaml DimensionId enum contains duplicates")
    return frozenset(values)
