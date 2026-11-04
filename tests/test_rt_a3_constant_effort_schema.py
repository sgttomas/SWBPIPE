"""RT-A3 increment 3: an exact (EX) envelope whose combination gate 4 withholds validates against the pressure-2
result schema.

The fixture is the producer's sparse CE-2 2*A envelope (`core/product_physics/tests/rt_a3_constant_effort_references.rs`
`ce2_withheld_envelope_fixture_is_current` keeps it current). Review of ac7fea603 found that
`results_pressure_2.v0.3.schema.json` did not list CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE among the combination
gate reasons, so such an envelope failed the schema; the negative control removes the one enum entry again.
"""
from copy import deepcopy
import json
from pathlib import Path

import pytest

from schema_validation import envelope_branches, schema_for_definition, validate_instance

ROOT = Path(__file__).resolve().parents[1]
SCHEMAS = ROOT / "schemas"
PRESSURE_SCHEMA_NAME = "results_pressure_2.v0.3.schema.json"
FIXTURE = ROOT / "fixtures/results/rt_a3_ce2_withheld_sparse.json"
GATE_4 = "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE"
PRESSURE_2 = "openpipestress.result_semantics/0.3.0/pressure-2"


def _pressure_schema():
    return json.loads((SCHEMAS / PRESSURE_SCHEMA_NAME).read_text(encoding="utf-8"))


def _envelope_definition():
    """The name of results.v0.3's ResultEnvelope branch for pressure-2, a $defs member of the pressure-2 schema."""
    import yaml

    results = yaml.safe_load((SCHEMAS / "results.v0.3.schema.yaml").read_text(encoding="utf-8"))
    refs = [b["$ref"] for b in results["$defs"]["ResultEnvelope"]["oneOf"] if b.get("$ref", "").startswith(PRESSURE_SCHEMA_NAME)]
    assert len(refs) == 1, refs
    name = refs[0].split("#/$defs/")[1]
    branch = next(b for b in envelope_branches(results)
                  if b.get("properties", {}).get("producer", {}).get("properties", {}).get("semantic_contract_id", {}).get("const") == PRESSURE_2)
    assert branch == _pressure_schema()["$defs"][name]
    return name


def _validate(pressure_schema, envelope):
    root = schema_for_definition(pressure_schema, _envelope_definition())
    if "$id" in pressure_schema:
        root["$id"] = pressure_schema["$id"]
    validate_instance(root, envelope, schema_label=PRESSURE_SCHEMA_NAME, instance_label="CE-2 2*A envelope")


def _fixture():
    document = json.loads(FIXTURE.read_text(encoding="utf-8"))
    envelope = document["envelope"]
    assert envelope["producer"]["semantic_contract_id"] == PRESSURE_2
    record = next(r for r in envelope["contract_evidence"]["exact_combinations"] if r["combination_id"] == document["combination_id"])
    assert record["gate"] == {"withheld": True, "reason": GATE_4}
    assert any(d["code"] == GATE_4 and d["affected_refs"] == [document["combination_id"]] for d in envelope["diagnostics"])
    return envelope


def _gate_reason_enums(value):
    """Every enum list in the schema that names a combination-withholding gate reason."""
    if isinstance(value, dict):
        for key, item in value.items():
            if key == "enum" and isinstance(item, list) and "COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE" in item:
                yield item
            else:
                yield from _gate_reason_enums(item)
    elif isinstance(value, list):
        for item in value:
            yield from _gate_reason_enums(item)


def test_every_pressure_2_gate_reason_enum_names_gate_4():
    enums = list(_gate_reason_enums(_pressure_schema()))
    assert enums
    for enum in enums:
        # Gate 4 directly follows gate 3. RT-A4 (#98) appends its own combination reasons after
        # them (unsigned operands, operand-mixed, operand withheld, nonlinear), so the pair is no
        # longer the enum's tail; its presence and order are what this guards.
        gate_3 = enum.index("COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE")
        assert enum[gate_3 : gate_3 + 2] == ["COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE", GATE_4]
        assert enum.count(GATE_4) == 1


def test_ce2_withheld_envelope_validates_against_the_pressure_2_schema():
    _validate(_pressure_schema(), _fixture())


def test_the_schema_without_gate_4_refuses_the_ce2_withheld_envelope():
    """Negative control: the schema as ac7fea603 had it (gate 4 absent from the reason enum) refuses the envelope."""
    old = deepcopy(_pressure_schema())
    removed = 0
    for enum in _gate_reason_enums(old):
        enum.remove(GATE_4)
        removed += 1
    assert removed >= 1
    with pytest.raises(AssertionError, match=GATE_4):
        _validate(old, _fixture())
