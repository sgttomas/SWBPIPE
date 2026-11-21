#!/usr/bin/env python3
"""Generate the four-span hydrostatic test model a person opens in the desktop.

The model is the four-span model M4 of `validation/references/hydrostatic_test`, written as a
model document from that package's frozen inputs (`references.json`, "inputs" and
"models"), which this generator reads after checking them against the package's
SHA256SUMS. It reads no product code and writes nothing a product computes.

The document holds what a person needs to define the test, and nothing the product
generates or the person enters for the test:

- the exact pressure contract (3.0.0, exact_pressure_v3, Euler-Bernoulli);
- nodes a to e at the span length a, four pipes with the reference section and
  material, each assigned the condition set `cs:test`;
- the condition set `cs:test`, whose condition `HYDRO` states the test contents density;
- the five supports: the run's axial and torsional restraint at a, the guided end at e,
  the spring at b, the constant-effort support at c and the variable spring hanger at d;
- no load case. The self-weight generator creates the case (a person generates it with the
  desktop's self-weight control); the person then enters the case's pressure region, its
  hydrostatic test condition and its contents selection, and solves.

`core/model_operations/operation_applier/tests/hydrostatic_four_span_journey.rs` takes this
document through those steps and compares the solve with the package's held four-span
rows (HT-4S-TEST-HELD).

Usage:
    python3 -I fixtures/hydrostatic_test/generate_four_span_test_model.py           # write
    python3 -I fixtures/hydrostatic_test/generate_four_span_test_model.py --check   # compare only
"""
import argparse
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PACKAGE = ROOT / "validation" / "references" / "hydrostatic_test"
OUTPUT = HERE / "four_span_test_model.json"

MODEL_KEY = "M4"
SCHEMA_VERSION = "0.9.0"
SET_ID = "cs:test"
CONDITION_KEY = "HYDRO"
PROVENANCE = ("invented hydrostatic test input (validation/references/hydrostatic_test, model M4); "
              "not standard, supplier, commercial or project data")


def frozen_references():
    """The package's references.json, after checking it against its SHA256SUMS."""
    raw = (PACKAGE / "references.json").read_bytes()
    sums = (PACKAGE / "SHA256SUMS").read_text().splitlines()
    line = next((l for l in sums if l.endswith("  references.json")), None)
    if line is None or line[:64] != hashlib.sha256(raw).hexdigest():
        sys.exit("references.json does not match the package's SHA256SUMS")
    return json.loads(raw)


def build(refs):
    inputs = refs["inputs"]
    spec = refs["models"][MODEL_KEY]

    def value(key, unit):
        entry = inputs[key]
        if entry["unit"] != unit:
            sys.exit(f"input {key} is in {entry['unit']}, expected {unit}")
        return {"value": entry["value"], "unit": unit}

    spans = spec["spans"]
    span = inputs[spec["span_length_input"]]["value"]
    letters = [chr(ord("a") + i) for i in range(spans + 1)]
    nodes = [{"id": f"node:{n}", "label": f"Node {n.upper()}", "position": {"x": i * span, "y": 0.0, "z": 0.0},
              "provenance": PROVENANCE} for i, n in enumerate(letters)]
    section = {
        "outside_diameter": value("D", "m"), "wall_thickness": value("t", "m"),
        "mill_tolerance": value("c_m", "m"), "corrosion_allowance": value("c_a", "m"),
        "material_density": value("rho_m", "kg/m^3"), "contents_density": value("rho_c_section", "kg/m^3"),
        "insulation_thickness": value("t_i", "m"), "insulation_density": value("rho_i", "kg/m^3"),
    }
    pipes = [{"id": f"pipe:{a}-{b}", "label": f"Pipe {a.upper()}-{b.upper()}", "from": f"node:{a}", "to": f"node:{b}",
              "section": section, "material": "material:ht", "condition_set_ref": SET_ID,
              "y_reference": {"x": 0.0, "y": 1.0, "z": 0.0}, "provenance": PROVENANCE}
             for a, b in zip(letters, letters[1:])]

    first, last = letters[0], letters[-1]
    supports = [
        {"id": f"support:{first}", "label": "End A: axial and torsional restraint", "node": f"node:{first}",
         "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX"], "provenance": PROVENANCE},
        {"id": f"support:{last}", "label": f"End {last.upper()}: guided", "node": f"node:{last}",
         "family": "guide", "restraints": ["UY", "UZ"], "provenance": PROVENANCE},
    ]
    interior = {
        "spring": ("k_4s_b", None),
        "constant_effort": (None, "Fce_4s_c"),
        "variable_spring": ("kv_4s_d", "Fc_4s_d"),
    }
    for node, item in spec["interior_supports"].items():
        if item["dof"] != "uz":
            sys.exit(f"interior support at {node} acts on {item['dof']}, expected uz")
        family = item["family"]
        rate, load = interior[family]
        record = {"id": item["support"], "node": node, "restraints": ["UZ"], "provenance": PROVENANCE}
        if family == "spring":
            record.update(label="Spring", family="spring", stiffness={"dof": "UZ", "value": value(rate, "N/m")})
        elif family == "constant_effort":
            record.update(label="Constant-effort support", family="constant_effort_support",
                          hanger={"hanger_type": "constant_effort_support", "constant_load": value(load, "N"),
                                  "travel_range": {"value": 0.05, "unit": "m"}, "source_reference": "invented"})
        else:
            # Installed and hot loads are review metadata the record requires; they carry the cold load.
            cold = value(load, "N")
            record.update(label="Variable spring hanger", family="variable_spring_hanger",
                          hanger={"hanger_type": "variable_spring_hanger",
                                  "stiffness": {"dof": "UZ", "value": value(rate, "N/m")},
                                  "installed_load": cold, "cold_load": cold, "hot_load": cold,
                                  "travel_range": {"value": 0.05, "unit": "m"}, "source_reference": "invented"})
        supports.append(record)

    return {
        "schema_version": SCHEMA_VERSION,
        "document_kind": "openpipestress.product_preview.model",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
        "data_boundary": {
            "private_data_policy": "no_private_project_material_rule_pack_or_owner_data_bundled",
            "professional_boundary": "test_object_not_a_design_human_review_required_no_code_compliance_or_professional_approval",
            "protected_source_policy": "no_standard_supplier_catalogue_commercial_example_or_project_content_copied",
            "public_examples_policy": "invented_test_object_hydrostatic_test_references_committed_in_public_repository_no_provenance_review_disposition_recorded",
        },
        "project": {
            "id": "project:hydrostatic-four-span",
            "name": "Hydrostatic test, four spans (invented)",
            "description": "Invented four-span hydrostatic test model (validation/references/hydrostatic_test, model M4); a test object, not a design",
            "units": {"length": "m", "force": "N", "moment": "N*m", "angle": "rad", "pressure": "Pa", "stress": "Pa",
                      "temperature": "degC"},
        },
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "nodes": nodes,
        "pipe_segments": pipes,
        "supports": supports,
        "components": [],
        "materials": [{"id": "material:ht", "label": "Invented steel", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                       "elastic_modulus": value("E", "Pa"), "poisson_ratio": value("nu", "1"), "provenance": PROVENANCE}],
        "condition_sets": [{"id": SET_ID, "label": "Test contents", "provenance": PROVENANCE,
                            "conditions": [{"key": CONDITION_KEY, "contents_density": value("rho_c_test", "kg/m^3")}]}],
        "load_cases": [],
        "combinations": [],
        "diagnostics": [],
    }


def text(document):
    return json.dumps(document, indent=2, sort_keys=True, ensure_ascii=False, allow_nan=False) + "\n"


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--check", action="store_true", help="compare with the committed file; write nothing")
    args = parser.parse_args(argv)
    wanted = text(build(frozen_references()))
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != wanted:
            print(f"{OUTPUT.relative_to(ROOT)} is not what the generator writes; run it without --check", file=sys.stderr)
            return 1
        print(f"{OUTPUT.relative_to(ROOT)} is current")
        return 0
    OUTPUT.write_text(wanted)
    print(f"wrote {OUTPUT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
