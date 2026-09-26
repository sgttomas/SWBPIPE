"""Reviewer's independent validation of the 30 joined carriers (T1_WAVE1_REVIEW_A).

Own registry (not tests/schema_validation.py): every schemas/* file is registered
under https://openpipestress.org/schemas/<filename> and under its own $id. Each
carrier must validate against its full carrier schema and match exactly one
top-level oneOf branch, expected index 5. Cross-carrier $refs into
results.v0.3.schema.yaml#/$defs/LoadReferenceSource* must resolve.
Usage: validate_carriers.py <WORKING_ROOT>
"""
import glob, json, pathlib, sys
import yaml
from jsonschema import Draft202012Validator
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT202012

root = pathlib.Path(sys.argv[1])
BASE = "https://openpipestress.org/schemas/"
resources = []
for p in sorted((root / "schemas").iterdir()):
    if p.suffix not in {".json", ".yaml"}:
        continue
    try:
        doc = json.loads(p.read_text())
    except Exception:
        continue
    if not isinstance(doc, dict):
        continue
    res = Resource.from_contents(doc, default_specification=DRAFT202012)
    resources.append((BASE + p.name, res))
    if isinstance(doc.get("$id"), str) and doc["$id"] != BASE + p.name:
        resources.append((doc["$id"], res))
registry = Registry().with_resources(resources)


def validator(uri):
    return Draft202012Validator({"$ref": uri}, registry=registry)


def branches(uri_prefix, instance, count):
    return [i for i in range(count) if validator(f"{uri_prefix}/{i}").is_valid(instance)]


RESULTS = BASE + "results.v0.3.schema.yaml"
RUN = BASE + "analysis_run.v0.3.schema.json"
RUN_DISPATCH = BASE + "analysis_run.schema.json"
SN = BASE + "stress_neutral_export.v0.3.schema.json"
results_n = len(json.loads((root / "schemas/results.v0.3.schema.yaml").read_text())["$defs"]["ResultEnvelope"]["oneOf"])
run_n = len(json.loads((root / "schemas/analysis_run.v0.3.schema.json").read_text())["$defs"]["AnalysisRun"]["oneOf"])
sn_n = len(json.loads((root / "schemas/stress_neutral_export.v0.3.schema.json").read_text())["oneOf"])

ok = fail = 0
for path in sorted(glob.glob(str(root / "fixtures/results/load_reference_source_*.json"))):
    inst = json.loads(pathlib.Path(path).read_text())
    name = pathlib.Path(path).name
    if name.endswith(".document.json"):
        full = validator(RESULTS).is_valid(inst)
        b = branches(RESULTS + "#/$defs/ResultEnvelope/oneOf", inst["result_envelope"], results_n)
    elif name.endswith(".analysis_run.json"):
        full = validator(RUN).is_valid(inst) and validator(RUN_DISPATCH).is_valid(inst)
        b = branches(RUN + "#/$defs/AnalysisRun/oneOf", inst["analysis_run"], run_n)
    elif name.endswith(".stress_neutral.json"):
        full = validator(SN).is_valid(inst)
        b = branches(SN + "#/oneOf", inst, sn_n)
    else:
        continue
    good = full and b == [5]
    ok += good; fail += not good
    print(("OK  " if good else "BAD ") + f"{name}: full={full} branches={b}")
    if not full:
        for e in list(validator(RESULTS if 'document' in name else RUN if 'analysis' in name else SN).iter_errors(inst))[:3]:
            print("    ", list(e.absolute_path)[:8], e.message[:200])
print(f"carriers ok={ok} bad={fail}")

# Relabel controls: each joined document relabelled to load-reference-1 / physics-source-1 must fail.
LR = "openpipestress.result_semantics/0.3.0/load-reference-1"
PS = "openpipestress.result_semantics/0.3.0/physics-source-1"
J = "openpipestress.result_semantics/0.3.0/load-reference-source-1"
bad_relabels = 0
for path in sorted(glob.glob(str(root / "fixtures/results/load_reference_source_*.json"))):
    text = pathlib.Path(path).read_text()
    uri = RESULTS if path.endswith("document.json") else RUN if path.endswith("analysis_run.json") else SN
    for other in (LR, PS):
        if validator(uri).is_valid(json.loads(text.replace(J, other))):
            bad_relabels += 1
            print("RELABEL VALIDATED", path, other)
print("relabel controls: validated-when-they-should-not =", bad_relabels)

# Pre-existing carriers keep their branch (LR 4, PS 3) on the candidate schemas.
for path in sorted(glob.glob(str(root / "fixtures/results/load_reference_*.json"))):
    if "load_reference_source_" in path:
        continue
    inst = json.loads(pathlib.Path(path).read_text())
    name = pathlib.Path(path).name
    if name.endswith(".document.json"):
        b = branches(RESULTS + "#/$defs/ResultEnvelope/oneOf", inst["result_envelope"], results_n); full = validator(RESULTS).is_valid(inst)
    elif name.endswith(".analysis_run.json"):
        b = branches(RUN + "#/$defs/AnalysisRun/oneOf", inst["analysis_run"], run_n); full = validator(RUN).is_valid(inst)
    elif name.endswith(".stress_neutral.json"):
        b = branches(SN + "#/oneOf", inst, sn_n); full = validator(SN).is_valid(inst)
    else:
        continue
    print(f"pre-existing {name}: full={full} branches={b}")
