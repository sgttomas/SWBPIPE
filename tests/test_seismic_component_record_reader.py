"""DEL-05-01 (issue 450 item 4): the Python package reader of an exact case's ``equivalent_static``
record with its optional ``component_forces`` (``tests/verifiers/pressure_3.equivalent_static_record``,
the Rust ``physics_evidence::equivalent_static_record``). The reader checks the closed shape and that
each load identifier is in the record's inventory once and is the seismic component identifier formed
from the entry's own fields (case, axis, component); it claims nothing about the model (the
model-bound check is Rust's, tested in ``core/product_physics/tests/seismic_component_weight.rs``).
The record below has the shape the producer writes for the valve of
``validation/references/seismic_component_weight`` under a g-factor along global X."""
import copy

import pytest

from tests.verifiers.pressure_3 import equivalent_static_record

CASE = "case:s"
PREFIX = f"{CASE}:generated:seismic:global-x:component:15:component:valve"
FORCE, MY, MZ = f"{PREFIX}:force", f"{PREFIX}:moment:y", f"{PREFIX}:moment:z"


def bits(value: float) -> str:
    import struct
    return struct.pack(">d", value).hex()


def record() -> dict:
    return {
        "mass_method": "source_od_nominal_wall_pressure_bore_areas/v1",
        "generated_load_ids": [f"{CASE}:generated:seismic:global-x:element-0", FORCE, MY, MZ],
        "component_forces": [{
            "component_ref": "component:valve", "node_ref": "C", "axis": "global_x",
            "g_factor": bits(0.3), "weight_N": bits(2350.0),
            "center_of_gravity_m": [bits(-0.175), bits(0.12), bits(0.125)],
            "force": {"load_id": FORCE, "value_N": bits(705.0)},
            "moments": [{"load_id": MY, "axis": "global_y", "value_N_m": bits(0.125 * 705.0)},
                        {"load_id": MZ, "axis": "global_z", "value_N_m": bits(-0.12 * 705.0)}],
        }],
    }


def test_the_record_with_and_without_component_forces_is_admitted():
    equivalent_static_record(CASE, record())
    earlier = record()
    del earlier["component_forces"]
    equivalent_static_record(CASE, earlier)


def _set(path, value):
    def edit(r):
        target = r
        for key in path[:-1]:
            target = target[key]
        target[path[-1]] = value
    return edit


def _also_in_inventory(path, value):
    """Set an identifier and list it in the inventory, so that only its form is wrong."""
    def edit(r):
        _set(path, value)(r)
        r["generated_load_ids"].append(value)
    return edit


def _wind_case(r):
    """Issue 450 round 1 finding 3: a wind-only generating case whose record carries a component entry
    naming one of its own wind primitives, with no moments. Every identifier is in the inventory."""
    wind = f"{CASE}:generated:wind:element-0"
    r["generated_load_ids"] = [wind]
    r["component_forces"][0]["force"]["load_id"] = wind
    r["component_forces"][0]["moments"] = []


@pytest.mark.parametrize("what, edit", [
    ("extra record key", _set(["extra"], 1)),
    ("empty list", _set(["component_forces"], [])),
    ("extra entry key", _set(["component_forces", 0, "extra"], 1)),
    ("decimal value", _set(["component_forces", 0, "force", "value_N"], "705")),
    ("uppercase hex", _set(["component_forces", 0, "weight_N"], bits(2350.0).upper())),
    ("two offset coordinates", _set(["component_forces", 0, "center_of_gravity_m"], [bits(0.0), bits(0.0)])),
    ("local axis", _set(["component_forces", 0, "axis"], "local_x")),
    ("moment about its own axis", _set(["component_forces", 0, "moments", 0, "axis"], "global_x")),
    ("two moments about one axis", _set(["component_forces", 0, "moments", 1, "axis"], "global_y")),
    ("id outside the inventory", _set(["component_forces", 0, "force", "load_id"], f"{CASE}:generated:seismic:global-x:other")),
    ("id twice", _set(["component_forces", 0, "moments", 1, "load_id"], MY)),
    ("force id of another axis", _also_in_inventory(["component_forces", 0, "force", "load_id"],
                                                    f"{CASE}:generated:seismic:global-y:component:15:component:valve:force")),
    ("moment id of another moment axis", _also_in_inventory(["component_forces", 0, "moments", 0, "load_id"], f"{PREFIX}:moment:x")),
    ("component length not the byte length", _also_in_inventory(["component_forces", 0, "force", "load_id"],
                                                                f"{CASE}:generated:seismic:global-x:component:14:component:valve:force")),
    ("another component's id", _also_in_inventory(["component_forces", 0, "force", "load_id"],
                                                  f"{CASE}:generated:seismic:global-x:component:16:component:flange:force")),
    ("a wind generated id with empty moments", _wind_case),
])
def test_a_malformed_component_record_is_refused(what, edit):
    forged = copy.deepcopy(record())
    edit(forged)
    with pytest.raises(ValueError, match="SOURCE_PHYSICS_EVIDENCE_INVALID: equivalent_static"):
        equivalent_static_record(CASE, forged)


def test_values_are_not_bound_to_a_model_here():
    """A changed bit is admitted by the package reader: binding to the model is the model-bound check's."""
    forged = record()
    forged["component_forces"][0]["force"]["value_N"] = bits(705.0000000000001)
    equivalent_static_record(CASE, forged)
