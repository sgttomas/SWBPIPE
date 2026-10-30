"""Current ordinary maximum declarations have one scope in every transport branch."""
import copy
import json
from pathlib import Path

import jsonschema
import pytest

PROJECT = Path(__file__).resolve().parents[1]
BASIS = "j_side_section_equilibrium_binary64_enclosures"
SCOPE = "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate"
OLD_BASIS = "j_side_section_equilibrium_binary64"
OLD_SCOPE = "supplied_binary64_polynomial_coefficients; solution and coefficient formation error are separate"
# B7b: the load/reference-state evidence (and its maximum) moved from load_reference_state.schema.json, which
# keeps only the authored definitions and the state record, to pressure-2's PressureContractEvidence in
# results_pressure_2.v0.3; the ordinary maximum transport check moves with it.
SCHEMAS = ("results.v0.3.schema.yaml", "results_pressure_2.v0.3.schema.json",
           "physics_source_recovery.schema.json", "stress_neutral_export.v0.3.schema.json")


def test_the_authored_load_reference_schema_carries_no_maximum_transport():
    assert "coefficient_basis" not in (PROJECT / "schemas/load_reference_state.schema.json").read_text()


def ordinary_branches(node):
    if isinstance(node, dict):
        if node.get("properties", {}).get("coefficient_basis", {}).get("const") == BASIS:
            yield node
        for child in node.values():
            yield from ordinary_branches(child)
    elif isinstance(node, list):
        for child in node:
            yield from ordinary_branches(child)


@pytest.mark.parametrize("schema_name", SCHEMAS)
def test_each_ordinary_maximum_transport_refuses_old_or_mixed_scope(schema_name):
    schema = json.loads((PROJECT / "schemas" / schema_name).read_text())
    maximum = dict(pipe_id="pipe", result_id="maximum", station_fraction=0.5,
                   span_index=0, local_fraction=0.5, value_lower_pa=1.0,
                   value_upper_pa=1.0, global_upper_bound_pa=1.0,
                   certified_gap_pa=0.0, subdivisions=0,
                   approximation="piecewise_quadratic_straight_section_statics",
                   coefficient_basis=BASIS, enclosure_scope=SCOPE)
    branches = list(ordinary_branches(schema))
    assert branches, "current ordinary maximum branch is missing"
    for branch in branches:
        validator = jsonschema.Draft202012Validator({"$defs": schema["$defs"], **branch})
        # T4-U6 (D-5, §2.5): an exact-contract maximum names its section basis; the preview one has none.
        maximum.pop("section_basis", None)
        if "section_basis" in branch.get("required", []):
            maximum["section_basis"] = "nominal_section"
            assert not validator.is_valid(dict(maximum, section_basis="thin_wall")), schema_name
        validator.validate(maximum)
        for field, value in (("coefficient_basis", OLD_BASIS),
                             ("enclosure_scope", OLD_SCOPE),
                             ("enclosure_scope", "arbitrary nonempty scope")):
            invalid = copy.deepcopy(maximum)
            invalid[field] = value
            assert not validator.is_valid(invalid), (schema_name, field, value)
