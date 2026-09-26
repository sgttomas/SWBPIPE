"""T1 integration (manager lane, after the owner-approved package_v0_3.py edit
for load-reference-source-1, D1): build the ten joined stress-neutral 0.3
packages from the committed joined raws and the WP1 AnalysisRun carriers, in
the same way as the CP3 load-reference-1 outputs. Each package is validated
against the stress_neutral_export v0.3 schema and the package validator (alone
and with source and analysis); its nine materialized members round-trip; a
load-reference-1 relabel and a physics-source-1 relabel must not validate.
Writes compact carriers fixtures/results/load_reference_source_<name>_<mode>.stress_neutral.json.
Run from WORKING_ROOT: python <this> <WORKING_ROOT> [--check]."""
import hashlib, json, pathlib, sys, tempfile
from copy import deepcopy
root = pathlib.Path(sys.argv[1]).resolve()
write = "--check" not in sys.argv
sys.path.insert(0, str(root)); sys.path.insert(0, str(root / "tests"))
from core.analysis_runs.compatibility import _source_contract, build_analysis_run_v0_3, LOAD_REFERENCE_SOURCE_CONTRACT_ID
from core.handoff.stress_neutral import package_v0_3 as sn
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
from test_stress_neutral_export_package import source_payload
from schema_validation import validate_instance
schema = json.loads((root / "schemas/stress_neutral_export.v0.3.schema.json").read_text())
RELABELS = {"load-reference-1": ("openpipestress.result_semantics/0.3.0/load-reference-1", None),
            "physics-source-1": ("openpipestress.result_semantics/0.3.0/physics-source-1", None)}
def arguments(raw, analysis):
    args = source_payload()
    args["source_result_ref"] = {"object_type": "ResultEnvelope", "ref": f"result-envelope:{raw['run_id']}"}
    args["source_run_ref"] = {"object_type": "AnalysisRun", "ref": raw["run_id"]}
    args["source_model_ref"] = {"object_type": "Model", "ref": raw["model_ref"]}
    args["source_hashes"] = deepcopy(analysis["analysis_run"]["hashes"])
    args["reproducibility_refs"] = [deepcopy(args["source_run_ref"])]
    _, _, table = _source_contract(raw)
    args["result_rows"] = [sn._source_row_projection(raw, row, table) for row in raw["results"]]
    args["stable_id_map"] = [{"canonical_ref": row["canonical_ref"],
        "export_ref": {"object_type": "StressNeutralRow", "ref": row["result_id"]},
        "mapping_status": "mapped", "loss_category": "exported"} for row in args["result_rows"]]
    return args
report = {"package_v0_3_sha256": hashlib.sha256((root / "core/handoff/stress_neutral/package_v0_3.py").read_bytes()).hexdigest(), "outputs": {}}
for name in ["eigen_motion", "fields", "mixed", "n05", "n06"]:
    for mode, short_mode in [("sparse_interactive", "sparse"), ("dense_scrutiny", "dense")]:
        raw_path = root / f"fixtures/product_preview/load_reference_source/{name}-{mode}.raw.json"
        raw = json.loads(raw_path.read_text())
        before = deepcopy(raw)
        analysis = build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": f"manifest:load-reference-source-{name}-{short_mode}"}, input_manifest_hash="1" * 64)
        stored = json.loads((root / f"fixtures/results/load_reference_source_{name}_{short_mode}.analysis_run.json").read_text())
        assert analysis == stored, f"{name}-{mode}: AnalysisRun differs from the WP1 carrier"
        packet = sn.build_stress_neutral_export_package_v0_3(source_envelope=raw, analysis_record=analysis, **arguments(raw, analysis))
        assert raw == before
        identity, digest, _ = _source_contract(raw)
        assert identity == LOAD_REFERENCE_SOURCE_CONTRACT_ID
        assert packet["semantic_contract"] == {"id": identity, "sha256": digest}
        assert packet["contract_evidence"] == raw["contract_evidence"]
        assert packet["source_block_recovery"] == raw["source_block_recovery"]
        assert packet["source_carrier_checksum"]["value"] == canonical_sha256_checked_v1(raw)
        assert [a["source_row"] for a in packet["source_annotations"]] == raw["results"]
        label = f"{name}-{mode}"
        validate_instance(schema, packet, instance_label=label)
        sn.validate_stress_neutral_export_package_v0_3(packet)
        sn.validate_stress_neutral_export_package_v0_3(packet, source_envelope=raw, analysis_record=analysis)
        with tempfile.TemporaryDirectory() as tmp:
            written = sn.write_materialized_members_v0_3(packet, tmp)
            assert [p.name for p in written] == sn.MEMBERS
            assert sn.read_materialized_members_v0_3(tmp, source_envelope=raw, analysis_record=analysis) == packet
        for relabel_name, (other, _) in RELABELS.items():
            relabel = json.loads(json.dumps(packet).replace(LOAD_REFERENCE_SOURCE_CONTRACT_ID, other))
            try:
                validate_instance(schema, relabel, instance_label=f"{label} as {relabel_name}")
                raise SystemExit(f"{label}: {relabel_name} relabel validated")
            except AssertionError:
                pass
        data = (json.dumps(packet, separators=(",", ":"), ensure_ascii=False) + "\n").encode()
        out = root / f"fixtures/results/load_reference_source_{name}_{short_mode}.stress_neutral.json"
        if write:
            out.write_bytes(data)
        assert out.read_bytes() == data, f"{label}: stored package differs"
        report["outputs"][out.name] = {"sha256": hashlib.sha256(data).hexdigest(), "rows": len(packet["result_rows"]), "raw_sha256": hashlib.sha256(raw_path.read_bytes()).hexdigest()}
print(json.dumps(report, indent=1))
