"""RV88 (U6c review): successor-shaped AnalysisRun and stress-neutral documents,
made by RV88's own generic relabel of every committed preview-physics-1 document
the exact v0.3 schema accepts (identity, table sha and, for stress-neutral, the
profile replaced everywhere), with the real milestone receipt added. Each
positive is paired with single-change negatives. Run in both lanes.
Usage: python rv88_successor_shapes.py <lane P> <u6a slice dir> <out.tsv>"""
import json
import sys
from copy import deepcopy
from pathlib import Path

P, SLICE, OUT = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
sys.path.insert(0, str(P))
sys.path.insert(0, str(P / "tests"))
from tests.schema_validation import validate_instance  # noqa: E402

PREVIEW_ID = "openpipestress.result_semantics/0.3.0/preview-physics-1"
SUCC_ID = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
PREVIEW_SHA = "ae55503d44a4750714a35c423623e38cf4132099134097193024d1635bfbc88a"
SUCC_SHA = "c74742ce6a936384e00986006e6a0b2e6bb11f190451e876eed9ffa11903c6a8"
PREVIEW_PROFILE, SUCC_PROFILE = "product_preview_mechanics_v1", "product_preview_retained_w1a_v2"
RECEIPT = json.loads((SLICE / "cand_sparse_interactive_receipt_from_derivative.json").read_text())
OTHER = json.loads((SLICE / "cand_dense_scrutiny_receipt_from_derivative.json").read_text())
EXACT = {"analysis_run": json.loads((P / "schemas/analysis_run.v0.3.schema.json").read_text()),
         "stress_neutral": json.loads((P / "schemas/stress_neutral_export.v0.3.schema.json").read_text())}
DISPATCH = {"analysis_run": json.loads((P / "schemas/analysis_run.schema.json").read_text()),
            "stress_neutral": json.loads((P / "schemas/stress_neutral_export.schema.json").read_text())}


def ok(schema, doc):
    try:
        validate_instance(schema, doc, instance_label="rv88")
        return "valid"
    except AssertionError as e:
        return "invalid:" + " | ".join(str(e).splitlines()[1:2])[:140]


def kind(v):
    if isinstance(v, dict) and isinstance(v.get("schema_version"), str):
        if "analysis_run" in v and "run_contract_status" in v:
            return "analysis_run"
        if "csv_text" in v and "manifest" in v:
            return "stress_neutral"
    return None


def walk(v, out):
    k = kind(v)
    if k:
        out.append((k, v))
    if isinstance(v, dict):
        for c in v.values():
            walk(c, out)
    elif isinstance(v, list):
        for c in v:
            walk(c, out)


def relabel(doc, profile):
    text = json.dumps(doc).replace(PREVIEW_ID, SUCC_ID).replace(PREVIEW_SHA, SUCC_SHA)
    if profile:
        text = text.replace(f'"{PREVIEW_PROFILE}"', f'"{SUCC_PROFILE}"')
    return json.loads(text)


def holder(k, d):
    return d["analysis_run"] if k == "analysis_run" else d


def projection(raw):
    p = deepcopy(raw)
    p.pop("retained_precision")
    p["producer"]["semantic_contract_id"] = PREVIEW_ID
    p["formulation_basis"]["profile_id"] = PREVIEW_PROFILE
    for r in p["results"]:
        r.pop("recovery_method", None)
    return p


# Real preview-physics-1 records and packages from the lane's own builders (base
# APIs), from the reader projection of each milestone; plus records of the other
# receipt-free identities, for the existing-branch receipt probe.
from core.analysis_runs.compatibility import build_analysis_run  # noqa: E402
from core.handoff.stress_neutral import package_v0_3 as sn  # noqa: E402
from tests.test_stress_neutral_physics_source import arguments  # noqa: E402
generated = []
for mode in ("sparse_interactive", "dense_scrutiny"):
    raw = json.loads((P / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())["source"]
    src = projection(raw)
    record = build_analysis_run(src, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:rv88"}, input_manifest_hash="5" * 64)
    generated.append((f"GEN/{mode}/analysis_run", "analysis_run", record))
    try:
        package = sn.build_stress_neutral_export_package_v0_3(source_envelope=src, analysis_record=record, **arguments(src, record))
        generated.append((f"GEN/{mode}/stress_neutral", "stress_neutral", package))
    except Exception as e:  # noqa: BLE001
        print("package build failed", mode, e)
others = []
for rel in ("fixtures/product_preview/physics_source/n05-sparse_interactive.raw.json", "fixtures/product_preview/source_blocks/n05-sparse_interactive.raw.json",
            "fixtures/results/physics_connected_mechanics_sparse.json", "fixtures/results/preview_physics_connected_sparse.json",
            "fixtures/product_preview/load_reference_source/n05-sparse_interactive.raw.json", "fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json"):
    try:
        src = json.loads((P / rel).read_text())
        record = build_analysis_run(src, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:rv88"}, input_manifest_hash="6" * 64)
        others.append((rel, record))
    except Exception as e:  # noqa: BLE001
        others.append((rel, f"build failed: {type(e).__name__}: {str(e)[:80]}"))

rows = []
for rel, record in others:
    if isinstance(record, str):
        rows.append(f"OTHER/{rel}\tanalysis_run\t-\tbuild\t{record}\t-")
        continue
    with_receipt = deepcopy(record); with_receipt["analysis_run"]["retained_precision"] = deepcopy(RECEIPT)
    with_null = deepcopy(record); with_null["analysis_run"]["retained_precision"] = None
    for name, d in (("plain", record), ("receipt", with_receipt), ("null", with_null)):
        rows.append(f"OTHER/{rel}\tanalysis_run\t-\t{name}\texact={ok(EXACT['analysis_run'], d)}\tdispatch={ok(DISPATCH['analysis_run'], d)}")
for path in [None] + sorted((P / "fixtures").rglob("*.json")) + sorted((P / "tests").rglob("*.json")):
    if path is None:
        found = [(k, d) for _, k, d in generated]
    else:
        try:
            found = []
            walk(json.loads(path.read_text()), found)
        except Exception:  # noqa: BLE001
            continue
    for i, (k, doc) in enumerate(found):
        if PREVIEW_ID not in json.dumps(doc) or ok(EXACT[k], doc) != "valid":
            continue
        key = f"{path.relative_to(P)}[{i}]" if path is not None else generated[i][0]
        for profile in (False, True):
            s = relabel(doc, profile)
            holder(k, s)["retained_precision"] = deepcopy(RECEIPT)
            for name, d in [("positive", s)] + [
                ("no_receipt", (lambda x: (holder(k, x).pop("retained_precision"), x)[1])(deepcopy(s))),
                ("null_receipt", (lambda x: (holder(k, x).__setitem__("retained_precision", None), x)[1])(deepcopy(s))),
                ("receipt_extra", (lambda x: (holder(k, x)["retained_precision"].__setitem__("rv88", 1), x)[1])(deepcopy(s))),
                ("receipt_policy_v1", (lambda x: (holder(k, x)["retained_precision"]["body"].__setitem__("policy", "M03-INTEGRITY-MP-v1"), x)[1])(deepcopy(s))),
                ("other_mode_receipt", (lambda x: (holder(k, x).__setitem__("retained_precision", deepcopy(OTHER)), x)[1])(deepcopy(s))),
                ("preview_sha", json.loads(json.dumps(s).replace(SUCC_SHA, PREVIEW_SHA))),
                ("receipt_on_preview", (lambda x: (holder(k, x).__setitem__("retained_precision", deepcopy(RECEIPT)), x)[1])(deepcopy(doc))),
            ]:
                rows.append(f"{key}\t{k}\tprofile={profile}\t{name}\texact={ok(EXACT[k], d)}\tdispatch={ok(DISPATCH[k], d)}")
OUT.write_text("\n".join(rows) + "\n")
print(len(rows), "rows")
