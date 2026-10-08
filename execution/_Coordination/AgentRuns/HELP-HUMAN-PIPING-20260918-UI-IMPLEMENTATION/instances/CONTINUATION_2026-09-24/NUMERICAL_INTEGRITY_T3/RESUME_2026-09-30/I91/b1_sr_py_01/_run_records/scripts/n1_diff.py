"""I91 SR-PY: RV108 N1's differential, one Python tree per run (base 262bd687f0 or the SR-PY head).

Usage: n1_diff.py <P root> <corpus path> <out.json> [--reader]
Inputs, each keyed by a label that is the same in every run:
  A. `_source_contract` (check_receipt True and False) on
     - every 07m entry (17 bases, 294 mutations, 28 must-pass), raw and as the reader's G7 projection;
     - every JSON object in P/fixtures (recursively) that carries `schema_version`, and every such
       object one level down (a fixture's `source`, `envelope`, ...);
     - enum probes: on each 07m base's projection and both milestone projections, each enum field
       (numerical_quality.status; each case's solve_quality, structural_status, model_matrix_fidelity,
       accuracy_evidence) set to every vocabulary member and to "estimated", "", None, 0, 1.5, True,
       False, [], [member], {} and {member: 1}, and removed.
  B. the v0.3 packager's validator on a preview-physics-1 package view of each milestone with the
     same enum probes.
  C. with --reader: the retained reader (raw with and without the invocation, transport) on each 07m
     base and both milestones with the same enum probes applied to the successor and rehashed.
An outcome is "ok:<value>" or "<exception class>:<message>".
"""
import hashlib, json, math, sys
from copy import deepcopy
from pathlib import Path

PROOT, CORPUS, OUT = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
READER = "--reader" in sys.argv
sys.path.insert(0, str(PROOT))
from core.analysis_runs import compatibility as c  # noqa: E402
from core.analysis_runs import retained_precision as rp  # noqa: E402
sys.path.insert(0, str(Path(__file__).parent))
import census_lib as lib  # noqa: E402

VOCAB = {
    "status": ["not_assessed", "checks_passed", "sensitive", "unresolved", "failed"],
    "solve_quality": ["not_assessed", "checks_passed", "sensitive", "unresolved", "failed"],
    "structural_status": ["passive_model_basis", "physical_mechanism_witnessed", "negative_energy_witnessed", "numerically_unresolved"],
    "model_matrix_fidelity": ["represented_equations_retained", "assembly_loss_detected", "assembly_uncertainty", "not_assessed"],
    "accuracy_evidence": ["not_claimed", "reference_verified", "unresolved"],
}
REMOVE = object()


def values(field, full=True):
    member = VOCAB[field][0]
    extra = ["estimated", "", None, 0, 1.5, True, False, [], [member], {}, {member: 1}, REMOVE]
    return (VOCAB[field] if full else [VOCAB[field][-1]]) + extra


def label_of(v):
    return "REMOVE" if v is REMOVE else json.dumps(v, sort_keys=True)


def fields_of(env):
    out = [(["numerical_quality", "status"], "status")]
    cases = env.get("numerical_quality", {}).get("cases", [])
    for i in range(len(cases) if isinstance(cases, list) else 0):
        for key in ("solve_quality", "structural_status", "model_matrix_fidelity", "accuracy_evidence"):
            out.append((["numerical_quality", "cases", i, key], key))
    return out


def edit(env, path, value):
    env = deepcopy(env)
    parent = env
    for part in path[:-1]:
        parent = parent[part]
    if value is REMOVE:
        parent.pop(path[-1], None)
    else:
        parent[path[-1]] = deepcopy(value)
    return env


def run(fn, *args):
    try:
        result = fn(*deepcopy(args))
    except Exception as error:  # every class is an outcome to compare
        text = f"{error.gate}/{error.code}/{error.detail}" if isinstance(error, rp.RetainedPrecisionError) else str(error)
        return f"{type(error).__name__}:{text[:300]}"
    if isinstance(result, tuple):
        result = result[:2]
    return "ok:" + hashlib.sha256(json.dumps(result, sort_keys=True, default=str).encode()).hexdigest()[:24]


def projected(source):
    base = deepcopy(source)
    base.pop("retained_precision", None)
    base["producer"]["semantic_contract_id"] = c.PREVIEW_PHYSICS_CONTRACT_ID
    base["formulation_basis"]["profile_id"] = "product_preview_mechanics_v1"
    for row in base.get("results", []):
        if isinstance(row, dict):
            row.pop("recovery_method", None)
    return base


def dispatch_pair(rows, label, env):
    rows[label + "|sc"] = run(lambda s: c._source_contract(s), env)
    rows[label + "|sc_transport"] = run(lambda s: c._source_contract(s, check_receipt=False), env)


def main():
    data = json.loads(CORPUS.read_text())
    bases = {f["id"]: f for f in data["cases"]}
    milestones = {}
    for mode in ("sparse_interactive", "dense_scrutiny"):
        doc = json.loads((PROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())
        milestones["milestone_" + mode] = {"source": doc["source"], "invocation": doc["invocation"]}
    rows = {}
    # A1: the corpus, raw and projected.
    entries = [("base:" + f["id"], deepcopy(f["source"])) for f in data["cases"]]
    for kind in ("mutations", "must_pass"):
        for entry in data[kind]:
            source, _ = lib.apply_entry(bases[entry["base"]], entry)
            entries.append((kind + ":" + entry["id"], source))
    for label, source in entries:
        dispatch_pair(rows, "A1|raw|" + label, source)
        if isinstance(source.get("producer"), dict) and isinstance(source.get("formulation_basis"), dict):
            dispatch_pair(rows, "A1|projected|" + label, projected(source))
    # A2: every fixture envelope.
    for path in sorted((PROOT / "fixtures").rglob("*.json")):
        try:
            doc = json.loads(path.read_text())
        except Exception:
            continue
        rel = str(path.relative_to(PROOT))
        candidates = [("", doc)] + ([(k, v) for k, v in doc.items() if isinstance(v, dict)] if isinstance(doc, dict) else [])
        for key, env in candidates:
            if isinstance(env, dict) and "schema_version" in env:
                dispatch_pair(rows, f"A2|{rel}|{key}", env)
    # A3: enum probes on every projection.
    envs = [(k, projected(f["source"])) for k, f in bases.items()] + [(k, projected(m["source"])) for k, m in milestones.items()]
    for name, env in envs:
        for path, field in fields_of(env):
            for v in values(field):
                dispatch_pair(rows, f"A3|{name}|{'.'.join(map(str, path))}|{label_of(v)}", edit(env, path, v))
    # B: the v0.3 packager's validator on a preview-physics-1 package view.
    from core.handoff.stress_neutral import package_v0_3 as sn
    sys.path.insert(0, str(PROOT / "tests"))
    from tests.test_stress_neutral_physics_source import arguments
    for name, m in milestones.items():
        base = projected(m["source"])
        record = c.build_analysis_run(base, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:u6b"}, input_manifest_hash="1" * 64)
        packet = sn.build_stress_neutral_export_package_v0_3(source_envelope=base, analysis_record=record, **arguments(base, record))
        rows[f"B|{name}|unedited"] = run(sn.validate_stress_neutral_export_package_v0_3, packet)
        for path, field in fields_of(packet):
            for v in values(field):
                rows[f"B|{name}|{'.'.join(map(str, path))}|{label_of(v)}"] = run(sn.validate_stress_neutral_export_package_v0_3, edit(packet, path, v))
    # C: the retained reader on rehashed successors.
    if READER:
        succ = [(k, f["source"], f["invocation"]) for k, f in bases.items()] + [(k, m["source"], m["invocation"]) for k, m in milestones.items()]
        for name, source, invocation in succ:
            for path, field in fields_of(source):
                for v in values(field, full=False):
                    edited = edit(source, path, v)
                    probe = lib.apply_mutation(edited, {"edits": [], "rehash": "all"})
                    key = f"C|{name}|{'.'.join(map(str, path))}|{label_of(v)}"
                    rows[key + "|raw"] = run(rp.validate_retained_precision, probe, invocation)
                    rows[key + "|raw_noinv"] = run(rp.validate_retained_precision, probe, None)
                    rows[key + "|transport"] = run(rp.validate_retained_precision_transport, probe)
    OUT.write_text(json.dumps({"corpus_sha256": hashlib.sha256(CORPUS.read_bytes()).hexdigest(), "rows": rows}, indent=0, sort_keys=True))
    print(len(rows))


main()
