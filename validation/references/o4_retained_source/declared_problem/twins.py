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


def main():
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
