"""Re-author J2's twelve source requests into O4 twins (standard library only).

Each J2 request (`fixtures/product_preview/pressure_source/{physics_source,
load_reference_source}/<name>.request.json`) is copied unchanged except for
the contract's `shear_model` and each pipe section's `mill_tolerance` and
`corrosion_allowance`, as the frozen unit's README ("Source-control twins")
describes. Allowance classes (judged-set values, README table):

  zero: c_m = 0,        c_a = 0              (the original request, untouched)
  mill: c_m = 0.125*t,  c_a = 0
  ca:   c_m = 0,        c_a = 0.0015 m (0.25*t where t < 6 mm)
  both: c_m = 0.125*t,  c_a as ca

Every value is converted into the section's own length unit and must be
exactly a binary64 number there; the script refuses otherwise. Where the
frozen unit froze a twin of the same load case, the re-authored values are
cross-checked against its document (`source_twins_cases.json`, read-only, from
the frozen unit's commit through `git show`).

Usage: python3 -I twins.py <project-root> <out-dir> [<frozen-twins-cases.json>]
       python3 -I twins.py run <index.json> <runs-dir> <dump-test-binary> <openpipestress-runner> <product_physics-dir>
       python3 -I twins.py extended <project-root> <out-dir> <frozen-cases.json> <frozen-twins-cases.json>
       python3 -I twins.py census <index.json> <runs-dir> <census.json>
Writes <out-dir>/<family>-<name>+<variant>+<theory>.request.json (the
`{materials?, model}` request) and <out-dir>/index.json.
"""

import json
import os
import sys
from fractions import Fraction

REQUESTS = [
    ("physics_source", "n05"), ("physics_source", "n06"), ("physics_source", "n05_unicode"),
    ("physics_source", "n05_units"), ("physics_source", "fields"), ("physics_source", "mixed"),
    ("physics_source", "mixed_units"),
    ("load_reference_source", "n05"), ("load_reference_source", "n06"),
    ("load_reference_source", "eigen_motion"), ("load_reference_source", "fields"),
    ("load_reference_source", "mixed"),
]
VARIANTS = ["zero", "mill", "ca", "both"]
THEORIES = {"eb": "euler_bernoulli", "timo": "timoshenko_annulus_energy_v1"}
TO_M = {"m": Fraction(1), "mm": Fraction(1, 1000), "cm": Fraction(1, 100)}


def exact(x):
    return Fraction(x) if not isinstance(x, Fraction) else x


def as_binary64(q, what):
    f = float(q)
    if Fraction(f) != q:
        raise SystemExit(f"twins: {what} = {q} is not exactly a binary64 number")
    return f


def allowances(section, variant):
    unit = section["wall_thickness"]["unit"]
    if unit not in TO_M:
        raise SystemExit(f"twins: unknown length unit {unit}")
    t_m = exact(section["wall_thickness"]["value"]) * TO_M[unit]
    cm = Fraction(1, 8) * t_m if variant in ("mill", "both") else Fraction(0)
    if variant in ("ca", "both"):
        ca = Fraction(1, 4) * t_m if t_m < Fraction(6, 1000) else exact(0.0015)
    else:
        ca = Fraction(0)
    return unit, cm, ca, t_m


def reauthor(request, variant, theory):
    doc = json.loads(json.dumps(request))
    model = doc["model"]
    if variant == "zero" and theory == "eb":
        return doc, {}
    model["pressure_contract"]["shear_model"] = THEORIES[theory]
    values = {}
    for pipe in model["pipe_segments"]:
        section = pipe["section"]
        unit, cm, ca, t_m = allowances(section, variant)
        if variant != "zero":
            section["mill_tolerance"] = {"unit": unit, "value": as_binary64(cm / TO_M[unit], "mill_tolerance")}
            section["corrosion_allowance"] = {"unit": unit, "value": as_binary64(ca / TO_M[unit], "corrosion_allowance")}
        values[pipe["id"]] = (float(cm), float(ca))
    return doc, values


# The headless runner's request metadata (invented; the same literal fields the
# runner's own CLI tests write). Only the project, model and load-case
# references follow the twin.
RUNNER_REQUEST = {
    "input_manifest_ref": {"ref_id": "o4-declared-problem-oracle", "ref_type": "audit_manifest"},
    "operation": "solve",
    "operation_ref": {"ref_id": "solve", "ref_type": "api_operation"},
    "privacy": {"classification": "public_metadata", "local_only": True,
                "private_payload_redacted": True, "telemetry_allowed": False},
    "professional_boundary": {"human_review_required": True, "software_makes_approval_claim": False,
                              "software_makes_authentication_claim": False,
                              "software_makes_certification_claim": False,
                              "software_makes_compliance_claim": False, "software_makes_sealing_claim": False},
    "provenance": {"contributor": "OpenPipeStress", "contributor_certification": "invented non-engineering example",
                   "redistribution_status": "invented_non_engineering_example", "review_status": "pending",
                   "source_license": "project invented",
                   "source_location": "validation/references/o4_retained_source/declared_problem/twins.py",
                   "source_name": "invented O4 declared-problem twin"},
    "request_id": "o4-declared-problem-oracle",
    "requested_outputs": ["result_envelope", "audit_manifest", "diagnostics"],
    "tbd_decisions": {"ci_provider": "TBD", "external_adapter_formats": "TBD",
                      "filesystem_mutation_policy": "SETTLED_DEC_065", "final_cli_command_syntax": "SETTLED_DEC_065",
                      "network_access": "SETTLED_DEC_065", "package_scripts": "SETTLED_DEC_065",
                      "physical_project_container": "TBD", "process_invocation": "SETTLED_DEC_065",
                      "public_transport_protocol": "TBD", "release_matrix": "TBD"},
    "unit_system_ref": {"ref_id": "invented-si", "ref_type": "unit_system"},
}
MODES = ["sparse_interactive", "dense_scrutiny"]


def run_products(index_path, runs, test_binary, runner, product_physics_dir):
    """Run the product's dump entry and the headless runner for every twin and mode."""
    import subprocess
    with open(index_path, encoding="utf-8") as fh:
        index = json.load(fh)
    os.makedirs(runs, exist_ok=True)
    for entry in index:
        with open(entry["request"], encoding="utf-8") as fh:
            payload = json.load(fh)
        stem = entry["twin"].replace("|", "+")
        request = json.loads(json.dumps(RUNNER_REQUEST))
        pid = payload["model"]["project"]["id"]
        request["project_ref"] = {"ref_id": pid, "ref_type": "project"}
        request["model_ref"] = {"ref_id": pid, "ref_type": "model"}
        request["load_basis_refs"] = [{"ref_id": c["id"], "ref_type": "load_case"} for c in payload["model"]["load_cases"]]
        cli_in = os.path.join(runs, stem + ".cli_input.json")
        with open(cli_in, "w", encoding="utf-8") as fh:
            json.dump({"request": request, "solve": {"preview_model": payload}}, fh, ensure_ascii=False)
        for mode in MODES:
            dump = os.path.join(runs, f"{stem}+{mode}.dump.json")
            env = dict(os.environ, SWBPIPE_O4_SOURCE_DUMP_REQUEST=entry["request"],
                       SWBPIPE_O4_SOURCE_DUMP_OUT=dump, SWBPIPE_O4_SOURCE_DUMP_MODE=mode)
            done = subprocess.run([test_binary, "o4_source_payload_dump", "--test-threads", "1"], env=env,
                                  cwd=product_physics_dir, capture_output=True, text=True)
            if done.returncode != 0 or not os.path.exists(dump):
                with open(dump + ".error.txt", "w", encoding="utf-8") as fh:
                    fh.write(done.stdout + "\n" + done.stderr)
            out = os.path.join(runs, f"{stem}+{mode}.envelope.json")
            done = subprocess.run([runner, "solve", "--input", cli_in, "--explicit-local-private-intent",
                                   "--solver-mode", mode], capture_output=True, text=True)
            with open(out, "w", encoding="utf-8") as fh:
                fh.write(done.stdout)
            with open(out + ".rc", "w", encoding="utf-8") as fh:
                fh.write(f"{done.returncode}\n{done.stderr}")
            print(f"run: {stem} {mode} cli_rc={done.returncode}", flush=True)


# ----------------------------------------------------------------------------- R2b: RF and judged-set documents

PROVENANCE = "invented_o4_declared_problem_oracle_no_library_data"
RF_CASES = ("RF-SKEW-T-PIN-OFF-345-r1e-08", "RF-SKEW-T-PIN-OFF-345-r1e-12")
DIRECTION = {("force", "x"): "global_x", ("force", "y"): "global_y", ("force", "z"): "global_z",
             ("moment", "x"): "rotation_x", ("moment", "y"): "rotation_y", ("moment", "z"): "rotation_z"}


def expand_dag(document):
    """The corpus DAG decoder: the same rule as `expand` in tools/fixtures/retained_precision_corpus.py
    (lines 1-92): scalars stay, [0, refs...] is an array, [1, key, value, ...] an object."""
    if document.get("format") != "retained-precision-corpus-dag/v1":
        raise SystemExit("twins: unsupported corpus storage format")
    nodes = document["nodes"]

    def value(index):
        node = nodes[index]
        if not isinstance(node, list):
            return node
        if node[0] == 0:
            return [value(ref) for ref in node[1:]]
        return {nodes[node[i]]: value(node[i + 1]) for i in range(1, len(node), 2)}
    return value(document["root"])


def length(value):
    return {"unit": "m", "value": value}


def judged_request(doc, project_id, title):
    """A frozen-unit case document (cases.json `document`) as a model 0.6.0 product request (README "Binding")."""
    mat = doc["material"]
    model = {
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "professional_acceptance": "not_provided",
                            "rule_check": "not_performed_user_rule_inputs_missing"},
        "combinations": [], "components": [], "diagnostics": [],
        "data_boundary": {"private_data_policy": "no_private_project_data",
                          "professional_boundary": "technical_preview_requires_human_engineering_review",
                          "protected_source_policy": "no_bundled_protected_owner_or_standards_data",
                          "public_examples_policy": "invented_or_cleared_data_only"},
        "document_kind": "openpipestress.product_preview.model",
        "load_cases": [{
            "id": "case", "kind": "primitive_user_load", "label": title, "pressure_regions": list(doc["pressure_regions"]),
            "primitive_loads": [{
                "category": f"concentrated_{l['kind']}", "dimension": l["kind"], "direction": DIRECTION[(l["kind"], l["direction"])],
                "id": l["id"], "magnitude": {"unit": l["unit"], "value": l["value"]}, "provenance": PROVENANCE,
                "target": {"node": l["node"], "type": "node"}} for l in doc["loads"]],
            "provenance": PROVENANCE, "status": "user_entered"}],
        "materials": [{"constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                       "elastic_modulus": {"unit": "Pa", "value": mat["elastic_modulus_Pa"]}, "id": mat["id"],
                       "label": "Invented material", "poisson_ratio": {"unit": "1", "value": mat["poisson_ratio"]},
                       "provenance": PROVENANCE}],
        "nodes": [{"id": n["id"], "label": f"Invented {n['id']}", "provenance": PROVENANCE,
                   "position": {"x": n["xyz_m"][0], "y": n["xyz_m"][1], "z": n["xyz_m"][2]}} for n in doc["nodes"]],
        "pipe_segments": [{"from": m["from"], "id": m["id"], "label": f"Invented {m['id']}", "material": mat["id"],
                           "provenance": PROVENANCE, "to": m["to"],
                           "section": {"outside_diameter": length(m["outside_diameter_m"]), "wall_thickness": length(m["wall_thickness_m"]),
                                       "mill_tolerance": length(m["mill_tolerance_m"]), "corrosion_allowance": length(m["corrosion_allowance_m"])},
                           "y_reference": {"x": m["y_reference"][0], "y": m["y_reference"][1], "z": m["y_reference"][2]}}
                          for m in doc["members"]],
        "pressure_contract": {"mode": "exact_pressure_v3", "shear_model": doc["shear_model"], "version": "3.0.0"},
        "project": {"description": "Invented O4 declared-problem oracle document; no external project or library data.",
                    "id": project_id, "name": title,
                    "units": {"angle": "rad", "force": "N", "length": "m", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
        "schema_version": "0.6.0",
        "supports": [],
    }
    for sup in doc["supports"]:
        if sup["kind"] == "rigid":
            model["supports"].append({"family": "anchor", "id": sup["id"], "node": sup["node"], "provenance": PROVENANCE,
                                      "restraints": list(sup["restraints"])})
        elif sup["kind"] == "spring":
            model["supports"].append({"family": "spring", "id": sup["id"], "node": sup["node"], "provenance": PROVENANCE,
                                      "restraints": [sup["dof"]],
                                      "stiffness": {"dof": sup["dof"], "value": {"unit": sup["unit"], "value": sup["stiffness"]}}})
        else:
            raise SystemExit(f"twins: unsupported support kind {sup['kind']}")
    return {"materials": [], "model": model}


def rf_twin(request, variant, theory, frozen):
    """An RF census request re-authored with an explicit shear model and explicit allowances (model 0.6.0)."""
    doc = json.loads(json.dumps(request))
    model = doc["model"]
    model["schema_version"] = "0.6.0"
    model["pressure_contract"] = {"mode": "exact_pressure_v3", "shear_model": THEORIES[theory], "version": "3.0.0"}
    values = {}
    for pipe in model["pipe_segments"]:
        section = pipe["section"]
        unit, cm, ca, _ = allowances(section, variant)
        section["mill_tolerance"] = {"unit": unit, "value": as_binary64(cm / TO_M[unit], "mill_tolerance")}
        section["corrosion_allowance"] = {"unit": unit, "value": as_binary64(ca / TO_M[unit], "corrosion_allowance")}
        values[pipe["id"]] = (float(cm), float(ca))
    if frozen is not None:
        for pid, pair in values.items():
            if pid in frozen and tuple(frozen[pid]) != pair:
                raise SystemExit(f"twins: RF {pid} allowances {pair} differ from the frozen twin {frozen[pid]}")
    return doc


def extended(root, out, frozen_cases_path, frozen_twins_path):
    """Write <out>/rf/index.json (RF PIN-OFF-345 twins) and <out>/judged/index.json (the frozen judged set)."""
    with open(os.path.join(root, "fixtures/results/j7_rf_census_inputs.json"), encoding="utf-8") as fh:
        census = expand_dag(json.load(fh))
    frozen_twins = {}
    with open(frozen_twins_path, encoding="utf-8") as fh:
        for case in json.load(fh)["cases"]:
            if case["design"].startswith("RF-"):
                frozen_twins[(case["design"].split("|")[0], case["variant"], case["theory"])] = {
                    m["id"]: (m["mill_tolerance_m"], m["corrosion_allowance_m"]) for m in case["document"]["members"]}
    rf_dir = os.path.join(out, "rf")
    os.makedirs(rf_dir, exist_ok=True)
    index, checked = [], 0
    for case in census["cases"]:
        if case["id"] not in RF_CASES:
            continue
        for theory in THEORIES:
            for variant in VARIANTS:
                frozen = frozen_twins.get((case["id"], variant, theory))
                checked += frozen is not None
                doc = rf_twin(case["request"], variant, theory, frozen)
                twin = f"{case['id']}|{variant}|{theory}"
                path = os.path.join(rf_dir, twin.replace("|", "+") + ".request.json")
                with open(path, "w", encoding="utf-8") as fh:
                    json.dump(doc, fh, ensure_ascii=False, sort_keys=True)
                index.append({"twin": twin, "family": "rf_census", "name": case["id"], "variant": variant, "theory": theory,
                              "source": "fixtures/results/j7_rf_census_inputs.json", "request": path, "load_cases": ["case"]})
    with open(os.path.join(rf_dir, "index.json"), "w", encoding="utf-8") as fh:
        json.dump(index, fh, ensure_ascii=False, indent=1)
    print(f"twins: wrote {len(index)} RF twins; {checked} cross-checked against the frozen twins")
    with open(frozen_cases_path, encoding="utf-8") as fh:
        judged = json.load(fh)["cases"]
    jdir = os.path.join(out, "judged")
    os.makedirs(jdir, exist_ok=True)
    index = []
    for case in judged:
        doc = judged_request(case["document"], f"invented:o4-judged:{case['id']}", f"O4 judged set {case['id']}")
        path = os.path.join(jdir, case["id"].replace("|", "+") + ".request.json")
        with open(path, "w", encoding="utf-8") as fh:
            json.dump(doc, fh, ensure_ascii=False, sort_keys=True)
        index.append({"twin": case["id"], "family": "judged_" + ("source_control" if case["source_control_style"] else "other"),
                      "name": case["design"], "variant": case["variant"], "theory": case["theory"], "tags": case["tags"],
                      "source": "validation/references/o4_retained_source/cases.json", "request": path, "load_cases": ["case"]})
    with open(os.path.join(jdir, "index.json"), "w", encoding="utf-8") as fh:
        json.dump(index, fh, ensure_ascii=False, indent=1)
    print(f"twins: wrote {len(index)} judged-set requests")


def census(index_path, runs, out_path):
    """The selection census: per run, whether the source route selected, and the product's stated reason if not."""
    with open(index_path, encoding="utf-8") as fh:
        index = json.load(fh)
    rows = []
    for entry in index:
        stem = entry["twin"].replace("|", "+")
        for mode in MODES:
            row = {"twin": entry["twin"], "name": entry["name"], "variant": entry["variant"], "theory": entry["theory"], "mode": mode}
            with open(os.path.join(runs, f"{stem}+{mode}.envelope.json"), encoding="utf-8") as fh:
                text = fh.read()
            try:
                doc = json.loads(text)
            except ValueError:
                row["outcome"] = "runner output is not JSON"
                rows.append(row)
                continue
            env = doc.get("payload", {}).get("mechanics_envelope") or {}
            row["contract"] = (env.get("producer") or {}).get("semantic_contract_id")
            row["mechanics"] = (env.get("status") or {}).get("mechanics")
            sbr = env.get("source_block_recovery")
            cases = (sbr or {}).get("body", {}).get("cases", [])
            sel = [c for c in cases if c["selected_method"] == "retained_source_blocks_exact_v1"]
            row["selected"] = bool(sel)
            if sel:
                row["outcome"] = "selected"
            else:
                reasons = []
                for c in cases:
                    oa = c.get("ordinary_attempt") or {}
                    reasons.append(f"receipt: {c['selected_method']}; ordinary {oa.get('outcome')}")
                for dg in env.get("diagnostics", []) + doc.get("payload", {}).get("diagnostics", []):
                    code = dg.get("code", "")
                    msg = dg.get("message", "")
                    if "INTEGRITY" in code:
                        q = msg.split("quality: ", 1)[1].split(",", 1)[0] if "quality: " in msg else "?"
                        reasons.append(f"{code} (quality {q})")
                    elif any(w in code for w in ("SOURCE", "RETAINED", "EXACT_PRESSURE", "REFUSED", "UNSUPPORTED", "DECLINE")) \
                            or dg.get("severity") == "blocking":
                        reasons.append(f"{code}: {msg[:300]}")
                row["outcome"] = "not selected"
                row["reasons"] = reasons
            rows.append(row)
    with open(out_path, "w", encoding="utf-8") as fh:
        json.dump(rows, fh, ensure_ascii=False, indent=1)
    sel = sum(r.get("selected", False) for r in rows)
    print(f"census: {sel} of {len(rows)} runs selected the source route; written to {out_path}")


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "extended":
        extended(*sys.argv[2:6])
        return
    if len(sys.argv) > 1 and sys.argv[1] == "census":
        census(*sys.argv[2:5])
        return
    if len(sys.argv) > 1 and sys.argv[1] == "run":
        run_products(*sys.argv[2:7])
        return
    root, out = sys.argv[1], sys.argv[2]
    frozen = {}
    if len(sys.argv) > 3:
        for case in json.load(open(sys.argv[3], encoding="utf-8"))["cases"]:
            src = case["source"]["file"]
            frozen[(src, case["variant"], case["theory"])] = {
                m["id"]: (m["mill_tolerance_m"], m["corrosion_allowance_m"]) for m in case["document"]["members"]}
    os.makedirs(out, exist_ok=True)
    index = []
    checked = 0
    for family, name in REQUESTS:
        rel = f"fixtures/product_preview/pressure_source/{family}/{name}.request.json"
        with open(os.path.join(root, rel), encoding="utf-8") as fh:
            request = json.load(fh)
        for theory in THEORIES:
            for variant in VARIANTS:
                doc, values = reauthor(request, variant, theory)
                twin = f"{family}-{name}|{variant}|{theory}"
                key = (rel, variant, theory)
                if key in frozen and values:
                    for pid, pair in values.items():
                        if pid in frozen[key] and tuple(frozen[key][pid]) != pair:
                            raise SystemExit(f"twins: {twin} {pid} allowances {pair} differ from the frozen twin {frozen[key][pid]}")
                        checked += 1
                path = os.path.join(out, twin.replace("|", "+") + ".request.json")
                with open(path, "w", encoding="utf-8") as fh:
                    json.dump(doc, fh, ensure_ascii=False, sort_keys=True)
                index.append({"twin": twin, "family": family, "name": name, "variant": variant,
                              "theory": theory, "source": rel, "request": path,
                              "load_cases": [c["id"] for c in doc["model"]["load_cases"]]})
    with open(os.path.join(out, "index.json"), "w", encoding="utf-8") as fh:
        json.dump(index, fh, ensure_ascii=False, indent=1)
    print(f"twins: wrote {len(index)} requests; {checked} member allowance pairs agree with the frozen twins")


if __name__ == "__main__":
    main()
