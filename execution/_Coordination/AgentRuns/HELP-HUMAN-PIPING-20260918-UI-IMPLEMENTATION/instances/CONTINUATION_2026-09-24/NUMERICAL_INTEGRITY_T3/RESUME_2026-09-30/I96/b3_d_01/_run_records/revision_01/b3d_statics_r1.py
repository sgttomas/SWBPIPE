"""I96 B3-D revision 01: regenerate the draft statics (records only; standard library only).

Imports the original generator `b3d_statics.py` (unchanged, beside this file) and applies
revision 01's changes on top of it, so the delta is explicit:
  S-2   DEF-E's `evidence` member and `rows.maximum` scoped to the owner case's exact_cases
        entry; the `scope.excludes` token `pressure_regions` renamed `nonempty_pressure_regions`
        (RV116 S-2, optional part, adopted).
  N-8   SCHEMA: one coherent `title` and `$comment` beside the `definition_id` enum.
  N-7   The `formulation_basis.profile_id` enums of the results and stress-neutral carrier
        schemas gain `exact_straight_retained_w1a_v2` (diffs only; lane T installs).

Writes into <out_dir>:
  retained_precision_prepared_exact_v1.json, semantic_contract_v0_3_physics_retained_1.json,
  retained_precision_mp_v2.schema.b3b.json (scratch), SCHEMA_ENUM.diff, CARRIER_PROFILE_ENUMS.diff,
  b3d_statics_r1.out.json

Usage: python b3d_statics_r1.py <P> <out_dir>
"""
import difflib
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import b3d_statics as v0  # the original generator, unchanged

EXACT_PROFILE = "exact_straight_retained_w1a_v2"
PREVIEW_PROFILE = "product_preview_retained_w1a_v2"


def exact_definition_r1(defo):
    d = v0.exact_definition(defo)
    d["evidence"] = {
        "pipe_sections": (
            "in the owner case's contract_evidence.exact_cases entry (matched by load_case_id) only: regenerate "
            "pipe_sections[].As_m2,I_m4,J_m4,Z_m3 as the prepared A,I,J,Z bits; outside_diameter_m, "
            "effective_wall_thickness_m, ro_m, ri_m, Ai_m2, geometry_basis, pipe_id and order unchanged"),
        "pipe_stress_extrema": (
            "in the owner case's entry only: the maximum row family's eight regenerated numeric fields; "
            "every other key unchanged"),
        "unchanged": (
            "pressure [] and connector []; in the owner case's entry, load_case_id, profile_mode, material_basis, "
            "pipe_materials, pressure_rhs_assembly and stress_maximum_coverage (complete) byte-identical to the "
            "ordinary exact envelope; the entry of every case that is not selected byte-identical to the ordinary "
            "exact envelope; no recovery_method member"),
    }
    old = ("regenerate the eight numeric fields of contract_evidence.exact_cases[].pipe_stress_extrema;")
    new = ("regenerate the eight numeric fields of pipe_stress_extrema in the owner case's "
           "contract_evidence.exact_cases entry (matched by load_case_id);")
    assert d["rows"]["maximum"].count(old) == 1
    d["rows"]["maximum"] = d["rows"]["maximum"].replace(old, new)
    ex = d["scope"]["excludes"]
    assert ex.count("pressure_regions") == 1
    d["scope"]["excludes"] = ["nonempty_pressure_regions" if x == "pressure_regions" else x for x in ex]
    return d


def schema_r1(sraw):
    s = json.loads(sraw)
    assert s["$defs"]["ProductAttempt"]["properties"]["definition_id"] == {"const": v0.ORDINARY_ID}
    s["$defs"]["ProductAttempt"]["properties"]["definition_id"] = {"enum": [v0.ORDINARY_ID, v0.EXACT_ID]}
    assert s["title"] == "Standalone ordinary prepared C1/C2/C3 receipt"
    s["title"] = "Standalone prepared C1/C2/C3 receipt"
    prefix = ("Ordinary-prepared scope only; exact and prepared-combination admission require their later "
              "selected extension.")
    assert s["$comment"].startswith(prefix)
    s["$comment"] = ("Prepared ordinary and exact formations, selected by ProductAttempt.definition_id; the exact "
                     "route's constraints are reader gates (G0, G5b, G8), not schema branches; prepared-combination "
                     "admission requires its later selected extension." + s["$comment"][len(prefix):])
    return s


def add_profile(doc, ascii_out):
    hits = []

    def walk(v, path):
        if isinstance(v, dict):
            for k, x in v.items():
                if (k == "profile_id" and isinstance(x, dict) and isinstance(x.get("enum"), list)
                        and PREVIEW_PROFILE in x["enum"] and "exact_straight_pressure_v2" in x["enum"]):
                    assert EXACT_PROFILE not in x["enum"]
                    x["enum"].append(EXACT_PROFILE)
                    hits.append("/".join(path + [k, "enum"]))
                else:
                    walk(x, path + [k])
        elif isinstance(v, list):
            for i, x in enumerate(v):
                walk(x, path + [str(i)])
    walk(doc, [])
    return hits, (json.dumps(doc, indent=2, ensure_ascii=ascii_out) + "\n").encode()


def udiff(a, b, name):
    return "".join(difflib.unified_diff(a.decode().splitlines(True), b.decode().splitlines(True),
                                        f"a/P/{name}", f"b/P/{name}", n=3))


def main():
    p, out = Path(sys.argv[1]), Path(sys.argv[2])
    out.mkdir(parents=True, exist_ok=True)
    res = p / "fixtures/results"
    defo_raw = (res / "retained_precision_prepared_ordinary_v1.json").read_bytes()
    defo = json.loads(defo_raw)
    v0_defe = v0.exact_definition(defo)
    report = {"controls": {
        "defo_canonical_bytes": v0.canonical(defo) == defo_raw,
        "defo_H": v0.h("retained_precision_formation_v1", defo),
        "v0_definition_raw_sha256": v0.sha(v0.canonical(v0_defe)),
        "v0_definition_H": v0.h("retained_precision_formation_v1", v0_defe),
    }}
    d = exact_definition_r1(defo)
    raw = v0.canonical(d)
    assert all(b < 128 for b in raw) and v0.floats(d) == 0
    (out / "retained_precision_prepared_exact_v1.json").write_bytes(raw)
    h = v0.h("retained_precision_formation_v1", d)
    report["definition"] = {
        "bytes": len(raw), "raw_sha256": v0.sha(raw), "H_retained_precision_formation_v1": h,
        "preparation_subobject_identical_to_DEF_O": d["preparation"] == defo["preparation"],
        "paths_changed_from_v0": sorted(
            f"{k}.{kk}" for k in d for kk in (d[k] if isinstance(d[k], dict) else [None])
            if isinstance(d[k], dict) and d[k].get(kk) != v0_defe.get(k, {}).get(kk)),
        "evidence": d["evidence"], "rows.maximum": d["rows"]["maximum"], "scope.excludes": d["scope"]["excludes"],
    }
    p1 = (res / "semantic_contract_v0_3_physics_1.json").read_bytes()
    table = v0.physics_retained_table(p1, h)
    (out / "semantic_contract_v0_3_physics_retained_1.json").write_bytes(table)
    report["table"] = {"bytes": len(table), "sha256": v0.sha(table),
                       "product_formation_definitions": json.loads(table)["product_formation_definitions"],
                       "roundtrip_indent2_ascii": (json.dumps(json.loads(table), indent=2, ensure_ascii=True) + "\n").encode() == table}
    sraw = (p / "schemas/retained_precision_mp_v2.schema.json").read_bytes()
    snew = (json.dumps(schema_r1(sraw), indent=2, ensure_ascii=True) + "\n").encode()
    (out / "retained_precision_mp_v2.schema.b3b.json").write_bytes(snew)
    (out / "SCHEMA_ENUM.diff").write_text(udiff(sraw, snew, "schemas/retained_precision_mp_v2.schema.json"))
    report["schema"] = {"main_sha256": v0.sha(sraw), "b3b_only_sha256": v0.sha(snew)}
    diffs, carriers = [], {}
    for name, ascii_out in (("schemas/results.v0.3.schema.yaml", True),
                            ("schemas/stress_neutral_export.v0.3.schema.json", False)):
        a = (p / name).read_bytes()
        assert (json.dumps(json.loads(a), indent=2, ensure_ascii=ascii_out) + "\n").encode() == a
        hits, b = add_profile(json.loads(a), ascii_out)
        diffs.append(udiff(a, b, name))
        carriers[name] = {"main_sha256": v0.sha(a), "profile_enum_only_sha256": v0.sha(b), "enum_paths": hits}
    (out / "CARRIER_PROFILE_ENUMS.diff").write_text("".join(diffs))
    report["carriers"] = carriers
    (out / "b3d_statics_r1.out.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps(report, indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
