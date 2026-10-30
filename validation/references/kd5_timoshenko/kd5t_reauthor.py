"""Re-author surviving K-D5 evaluation corpora as current exact-contract documents (premise census input).

Section-bases Design section 4 item 5: the premise census runs "on the surviving RV30 and realistic
input corpora re-authored with timoshenko_annulus_energy_v1 (straight tangents Timoshenko, arcs
Euler-Bernoulli)" against their Euler-Bernoulli re-authoring. This tool performs the re-authoring
only (standard library; no product code); it reads corpus lines {tag, request, ...} and writes
lines {tag, request} (model format 0.5.0, `3.0.0/exact_pressure_v3`).

Re-authoring rules (each a document change, stated so that a reader can undo it):
- schema_version 0.5.0; pressure_contract {version 3.0.0, mode exact_pressure_v3, shear_model};
  a pressure-free document becomes an exact-contract document with explicitly empty pressure
  regions in every load case.
- every pipe section gains corrosion_allowance 0 m (authored, not defaulted); mill_tolerance is kept.
- a material given as E and G becomes homogeneous_isotropic_E_nu_v1 with nu = fl(E/(2G) - 1) and no
  G; E/nu materials are unchanged. (The exact contract derives G from E and nu.)
- nodal load directions UX..RZ are written global_x..rotation_z; values are unchanged.
- realized bends (components of kind bend) are unchanged; they stay Euler-Bernoulli under either
  shear model (Design section 3.6).
- the tag gains "|reauth-eb" or "|reauth-ts".

Usage: python3 -I -B kd5t_reauthor.py <eb|ts> <out.jsonl> <corpus.jsonl> [<corpus.jsonl> ...]
"""

from __future__ import annotations

import copy
import json
import sys

SHEAR = {"eb": "euler_bernoulli", "ts": "timoshenko_annulus_energy_v1"}
DIRS = {"UX": "global_x", "UY": "global_y", "UZ": "global_z", "RX": "rotation_x", "RY": "rotation_y", "RZ": "rotation_z"}


def reauthor(request, theory):
    req = copy.deepcopy(request)
    m = req["model"]
    m["schema_version"] = "0.5.0"
    m["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": SHEAR[theory]}
    for p in m["pipe_segments"]:
        sec = p["section"]
        if "corrosion_allowance" not in sec:
            sec["corrosion_allowance"] = {"value": 0.0, "unit": "m"}
    for mat in m["materials"]:
        if "poisson_ratio" not in mat:
            E = mat["elastic_modulus"]
            G = mat.pop("shear_modulus")
            if E["unit"] != G["unit"]:
                raise ValueError("E and G in different units")
            mat["poisson_ratio"] = {"value": float(E["value"]) / (2.0 * float(G["value"])) - 1.0, "unit": "1"}
            mat["constitutive_basis"] = "homogeneous_isotropic_E_nu_v1"
            for tp in mat.get("temperature_points", []):
                if "shear_modulus" in tp:
                    raise ValueError("temperature points with G are outside this re-authoring")
    for lc in m["load_cases"]:
        lc.setdefault("pressure_regions", [])
        for pl in lc.get("primitive_loads", []):
            pl["direction"] = DIRS.get(pl["direction"], pl["direction"])
    return req


def main():
    theory, out_path, inputs = sys.argv[1], sys.argv[2], sys.argv[3:]
    n = 0
    with open(out_path, "w") as f:
        for path in inputs:
            for line in open(path):
                x = json.loads(line)
                f.write(json.dumps({"tag": x["tag"] + "|reauth-" + theory, "request": reauthor(x["request"], theory)},
                                   sort_keys=True, separators=(",", ":")) + "\n")
                n += 1
    print(f"{n} documents re-authored ({SHEAR[theory]}) -> {out_path}")


if __name__ == "__main__":
    main()
