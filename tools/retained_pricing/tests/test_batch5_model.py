"""Batch 5 producer, owned-layout and domain-premise refusal controls."""
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
MODEL = runpy.run_path(str(PRICING / "base/batch5_model.py"))
LIB = "core/product_physics/src/lib.rs"
PRESSURE = "core/product_physics/src/pressure_runtime.rs"
MEMORY = "core/product_physics/src/retained_memory.rs"


def test_current_positive_carrier_source_is_bound():
    assert MODEL["active"](SOURCE)


@pytest.mark.parametrize("file,old,new", [
    (PRESSURE, "    pub assembled_operand_bounds: Vec<f64>,", "    pub assembled_operand_bounds: Vec<Vec<f64>>,"),
    # B7b: member_connector_ids is a current-format field (B7a: `#[serde(skip)]`).
    (PRESSURE, '    #[serde(default, deserialize_with = "super::deserialize_present_value")]\n    pub member_connector_ids',
     "    #[serde(default)]\n    pub member_connector_ids"),
    (PRESSURE, "    let mut cap_terms =", "    let extra_owner = vec![0.0; 1024];\n    let mut cap_terms ="),
    (PRESSURE, "    let mut by_dof =", "    let extra_owner = vec![Vec::<f64>::new(); 1024];\n    let mut by_dof ="),
    (LIB, "fn solve_prepared_load_case(", "fn renamed_prepared_load_case("),
    (LIB, "    solve_prepared_load_case(model,built", "    changed_prepared_load_case(model,built"),
    (MEMORY, "if !m.components.is_empty()", "if false && !m.components.is_empty()"),
    (MEMORY, "regions.capacity() != 0", "regions.capacity() > 1024"),
])
def test_changed_owner_or_empty_child_premise_refuses(tmp_path, file, old, new):
    for relative in (LIB, PRESSURE, MEMORY):
        target = tmp_path / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / relative, target)
    target = tmp_path / file
    source = target.read_text()
    assert old in source
    target.write_text(source.replace(old, new, 1))
    with pytest.raises(ValueError):
        MODEL["active"](tmp_path)


def test_unconditional_descriptor_keeps_the_positive_73_byte_literal_charge():
    assert MODEL["descriptor_recovery_bytes"](SOURCE) == 73
    body = MODEL["HELPERS"]["function_body"]((SOURCE / LIB).read_text(), "append_curved_bend_macro_element_results")
    template, = [value for value in MODEL["HELPERS"]["literals"](body) if value.startswith("component_family=bend;")]
    assert MODEL["HELPERS"]["literal_bytes"](template) == 558 + 73
    assert template.count(";recovery=node_on_element_end_actions_from_assembled_stiffness_in_end_tangent_frame;") == 1
    assert "recovery_treatment" not in body


@pytest.mark.parametrize("change", ["missing", "changed", "duplicate"])
def test_unconditional_descriptor_literal_changes_refuse(tmp_path, change):
    target = tmp_path / LIB
    target.parent.mkdir(parents=True)
    source = (SOURCE / LIB).read_text()
    literal = ";recovery=node_on_element_end_actions_from_assembled_stiffness_in_end_tangent_frame;"
    assert source.count(literal) == 1
    replacement = {"missing": ";", "changed": ";recovery=end_forces_from_assembled_stiffness_in_chord_frame;",
                   "duplicate": literal + literal}[change]
    target.write_text(source.replace(literal, replacement, 1))
    with pytest.raises(ValueError, match="descriptor body changed"):
        MODEL["descriptor_recovery_bytes"](tmp_path)
