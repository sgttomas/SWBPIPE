"""I97 B2-C revision 02: regenerate DEF-C and PTABLE (records only; standard library only).

Imports the sealed v0 generator `b2c_statics.py` and the sealed r1 generator `b2c_statics_r1.py` unchanged, checks that
they still reproduce the sealed r1 statics (DEF-C r1, PTABLE r1) and the v0 SCHEMA text, then applies revision 02:
  DEF-C  rows.displacement_magnitude  option (ii): RN64, ties to even, of the exact 3-norm of the published components
         stages.observables           NC-2: the guard's formula, 64*eps*max(|p|,MIN_POSITIVE)
  PTABLE product_formation_definitions  DEF-C r2's H (nothing else changes)

Writes into <out_dir>:
  retained_precision_prepared_combination_v1.json          DEF-C r2
  semantic_contract_v0_3_preview_physics_retained_1.json   PTABLE r2
  b2c_statics_r2.out.json                                  hashes, controls and deltas

Usage: python b2c_statics_r2.py <P> <I96 record folder> <v0 b2c_statics.py> <r1 b2c_statics_r1.py> <v0 statics folder>
                                 <r1 statics folder> <out_dir>
"""
import copy
import importlib.util
import json
import sys
from pathlib import Path

DISPLACEMENT_MAGNITUDE = (
    "finish the same node's frozen global_nodal_displacement_x/y/z rows; RN64, ties to even, of the exact 3-norm "
    "sqrt(x^2+y^2+z^2) of their frozen raw values (mm): each square and their sum formed exactly, the square root "
    "rounded once to binary64, the same bits on every platform; finite only (a norm beyond binary64's range refuses "
    "the row), canonical +0, SI by the projection's mm rule; formed in the kernel's projection for combination owners "
    "only; still certify dual physical norm and the unchanged 64-epsilon combination magnitude guard (base G7 "
    "combination_magnitudes), which the observables stage checks; not H of a lane norm hull and not a nested binary64 "
    "hypot")
OBSERVABLES = (
    "base G7 combination_magnitudes on the frozen rows: one support-action row per support and component, and each "
    "displacement, force and moment magnitude p with |p-r| <= 64*eps*max(|p|,MIN_POSITIVE), r the binary64 "
    "hypot(hypot(x,y),z) of the same combination's published components; the case support coverage and guard; no "
    "preview case evidence, maximum or headline check; a failure is this combination's facade_certificate")


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def definition_r2(defc1):
    d = copy.deepcopy(defc1)
    assert d["rows"]["displacement_magnitude"].startswith("finish the same node's frozen global_nodal_displacement")
    assert "binary64 hypot(hypot(x,y),z) of their frozen raw values" in d["rows"]["displacement_magnitude"]
    d["rows"]["displacement_magnitude"] = DISPLACEMENT_MAGNITUDE
    assert "within 64 epsilon relative of binary64 hypot" in d["stages"]["observables"]
    d["stages"]["observables"] = OBSERVABLES
    return d


def main():
    p, i96, v0_gen, r1_gen, v0_statics, r1_statics, out = (Path(a) for a in sys.argv[1:8])
    out.mkdir(parents=True, exist_ok=True)
    v0 = load(v0_gen, "b2c_statics")
    r1 = load(r1_gen, "b2c_statics_r1")
    load(i96 / "_run_records/b3d_statics.py", "b3d_statics")
    b3 = load(i96 / "_run_records/revision_01/b3d_statics_r1.py", "b3d_statics_r1")
    res = p / "fixtures/results"
    defo_raw = (res / "retained_precision_prepared_ordinary_v1.json").read_bytes()
    defo = json.loads(defo_raw)
    ptable_raw = (res / "semantic_contract_v0_3_preview_physics_retained_1.json").read_bytes()
    schema_raw = (p / "schemas/retained_precision_mp_v2.schema.json").read_bytes()
    xtable = json.loads((i96 / "statics/r1/semantic_contract_v0_3_physics_retained_1.json").read_bytes())
    defc_name, table_name = ("retained_precision_prepared_combination_v1.json",
                             "semantic_contract_v0_3_preview_physics_retained_1.json")
    sealed_r1 = {n: (r1_statics / n).read_bytes() for n in (defc_name, table_name)}
    sealed_j1 = (v0_statics / "retained_precision_mp_v2.schema.json").read_bytes()
    report = {"inputs": {"DEF-O": v0.sha(defo_raw), "PTABLE (main)": v0.sha(ptable_raw),
                         "SCHEMA (main)": v0.sha(schema_raw),
                         "b2c_statics.py (v0, sealed)": v0.sha(v0_gen.read_bytes()),
                         "b2c_statics_r1.py (r1, sealed)": v0.sha(r1_gen.read_bytes()),
                         **{f"r1 statics/r1/{k}": v0.sha(v) for k, v in sealed_r1.items()},
                         "v0 statics/retained_precision_mp_v2.schema.json": v0.sha(sealed_j1)}}
    defo_h = v0.h(v0.FORMATION_DOMAIN, defo)

    # Controls: r1's definition and table, and v0's J1 SCHEMA text, reproduce byte for byte.
    defc1 = r1.definition_r1(v0.combination_definition(defo), defo)
    defc1_h = v0.h(v0.FORMATION_DOMAIN, defc1)
    t1 = json.loads(v0.ptable_revision(ptable_raw, defo_h, defc1_h, xtable))
    t1["accuracy_classification"]["scope"] = r1.SCOPE_R1
    j1, _ = v0.apply_j1(json.loads(schema_raw), b3.schema_r1)
    j1_raw = v0.pretty(j1)
    report["controls"] = {
        "r1_DEF_C_reproduces": v0.canonical(defc1) == sealed_r1[defc_name],
        "r1_PTABLE_reproduces": v0.pretty(t1) == sealed_r1[table_name],
        "v0_J1_SCHEMA_reproduces": j1_raw == sealed_j1,
        "DEF_O_H": defo_h, "r1_DEF_C_H": defc1_h,
    }
    assert all(v for k, v in report["controls"].items() if isinstance(v, bool))

    # DEF-C r2.
    defc2 = definition_r2(defc1)
    defc2_raw = v0.canonical(defc2)
    assert all(b < 128 for b in defc2_raw) and v0.floats(defc2) == 0
    defc2_h = v0.h(v0.FORMATION_DOMAIN, defc2)
    (out / defc_name).write_bytes(defc2_raw)
    report["definition"] = {
        "file": defc_name, "bytes": len(defc2_raw), "raw_sha256": v0.sha(defc2_raw),
        "H_retained_precision_formation_v1": defc2_h,
        "H_alternative_domain_retained_precision_combination_formation_v1": v0.h(
            "retained_precision_combination_formation_v1", defc2),
        "canonical_bytes": v0.canonical(json.loads(defc2_raw)) == defc2_raw, "ascii_only": True, "json_floats": 0,
        "operand_definition_sha256_is_H_DEF_O": defc2["inherits"]["operand_definition"]["sha256"] == defo_h,
        "paths_changed_from_r1": r1.paths_changed(defc1, defc2),
        "support_magnitude_equal_to_DEF_O": defc2["rows"]["support_magnitude"] == defo["rows"]["support_magnitude"],
        "rows.displacement_magnitude": defc2["rows"]["displacement_magnitude"],
        "stages.observables": defc2["stages"]["observables"],
    }

    # PTABLE r2: r1's table with DEF-C r2's H.
    t2 = json.loads(v0.ptable_revision(ptable_raw, defo_h, defc2_h, xtable))
    t2["accuracy_classification"]["scope"] = r1.SCOPE_R1
    table2 = v0.pretty(t2)
    (out / table_name).write_bytes(table2)
    report["table"] = {
        "file": table_name, "bytes": len(table2), "sha256": v0.sha(table2),
        "paths_changed_from_r1": r1.paths_changed(t1, t2),
        "product_formation_definitions": t2["product_formation_definitions"],
        "roundtrip_indent2_ascii": v0.pretty(json.loads(table2)) == table2,
    }
    report["schema"] = {"j1_merged_sha256": v0.sha(j1_raw), "unchanged_from_v0_statics": j1_raw == sealed_j1,
                        "contains_a_definition_hash": any(x in j1_raw.decode() for x in (defc1_h, defc2_h, defo_h))}
    report["reviewed_inputs_cascade"] = {
        "new_hashes": {"SCHEMA": v0.sha(j1_raw), "PTABLE": v0.sha(table2), "DEF-C": v0.sha(defc2_raw)},
        "moved_from_r1": {"PTABLE": [v0.sha(sealed_r1[table_name]), v0.sha(table2)],
                          "DEF-C": [v0.sha(sealed_r1[defc_name]), v0.sha(defc2_raw)]},
    }
    (out / "b2c_statics_r2.out.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps({k: report[k] for k in ("controls", "reviewed_inputs_cascade")}, indent=1, sort_keys=True))
    print(json.dumps({"DEF-C": {k: report["definition"][k] for k in ("raw_sha256", "H_retained_precision_formation_v1",
                                                                      "paths_changed_from_r1", "bytes")},
                      "PTABLE": {k: report["table"][k] for k in ("sha256", "paths_changed_from_r1")},
                      "SCHEMA": report["schema"]}, indent=1))


if __name__ == "__main__":
    main()
