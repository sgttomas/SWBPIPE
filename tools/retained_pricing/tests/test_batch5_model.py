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
    (PRESSURE, "    #[serde(skip)]", "    #[serde(default)]"),
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
