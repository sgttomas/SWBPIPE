"""I65 U4 G4: ORIGINS.json -- the Git blob id and SHA-256 of every source file, static input and
record this packet relies on (stdlib only; Git reads only, GIT_OPTIONAL_LOCKS=0).
Usage: python3 origins_g4.py <numerics repo root> > ORIGINS.json
"""
import hashlib, json, os, subprocess, sys

repo = sys.argv[1]
REV = "b1f80234dc"
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
P = "projects/chirality-piping/"
R = P + "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/"
def git(*a, binary=False):
    out = subprocess.run(["git", "-C", repo, *a], capture_output=True, env=env, check=True).stdout
    return out if binary else out.decode()
def at(path):
    data = git("show", f"{REV}:{path}", binary=True)
    return {"blob": git("rev-parse", f"{REV}:{path}").strip(), "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)}
SOURCES = ["core/product_physics/src/lib.rs", "core/product_physics/src/retained_wire.rs",
           "core/product_physics/src/retained_product.rs", "core/product_physics/src/retained_memory.rs",
           "core/product_physics/src/retained_receipt.rs", "core/product_physics/src/source_receipt.rs",
           "core/product_physics/src/source_receipt/source.rs", "core/product_physics/src/pressure_runtime.rs",
           "core/product_physics/src/self_weight.rs", "core/product_physics/Cargo.toml", "core/product_physics/Cargo.lock",
           "core/reporting/result_export/src/retained_precision.rs", "core/reporting/result_export/src/preview_physics_evidence.rs",
           "core/reporting/result_export/src/semantic_contract.rs", "core/reporting/result_export/src/source_blocks.rs",
           "core/reporting/result_export/Cargo.toml",
           "core/serialization/canonical_json/src/lib.rs", "core/serialization/canonical_json/src/binary64.rs",
           "core/solver/frame_kernel/src/structural/exact_boundary/functionals.rs",
           "core/solver/frame_kernel/src/structural/retained/origins.rs",
           "core/solver/frame_kernel/src/structural/retained/product_certificate.rs",
           "core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs",
           "core/solver/frame_kernel/src/structural/retained/adaptive.rs",
           "core/solver/nonlinear_integration/src/lib.rs", "core/loads/stress_recovery/src/lib.rs"]
STATICS = ["schemas/physics_source_recovery.schema.json", "schemas/retained_precision_mp_v2.schema.json",
           "fixtures/results/retained_precision_prepared_ordinary_v1.json",
           "fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json",
           "fixtures/results/semantic_contract_v0_2.json", "fixtures/results/semantic_contract_v0_3_precision_1.json",
           "fixtures/results/semantic_contract_v0_3_physics_1.json", "fixtures/results/semantic_contract_v0_3_load_reference_1.json",
           "fixtures/results/semantic_contract_v0_3_load_reference_source_1.json",
           "fixtures/results/semantic_contract_v0_3_preview_physics_1.json",
           "fixtures/results/semantic_contract_v0_3_physics_source_1.json",
           "fixtures/results/semantic_contract_v0_3_source_blocks_1.json", "schemas/source_block_recovery.schema.json"]
RECORDS = ["ROOT_RULINGS_V1.md", "RESUME_2026-09-30/I65/u4_g2_01/API.md", "RESUME_2026-09-30/I65/u4_g2_01/BUILD.md",
           "RESUME_2026-09-30/I65/u4_g2_01/DOMAIN.md", "RESUME_2026-09-30/I65/u4_g3_01/COMPOSITION.md",
           "RESUME_2026-09-30/I65/u4_g3_01/TEXT.md", "RESUME_2026-09-30/I65/u4_g3_01/STACK_INVENTORY.md",
           "RESUME_2026-09-30/I65/u4_g3_01/RESIDUALS_G3.md", "RESUME_2026-09-30/I65/u4_g3_01/ORDINARY.md",
           "RESUME_2026-09-30/I65/u4_g3_01/G2_AMENDMENTS.md",
           "RESUME_2026-09-30/REVIEW_RV83/u4_g2_02/REVIEW.md", "RESUME_2026-09-30/REVIEW_RV84/u4_g3_01/REVIEW.md",
           "RESUME_2026-09-30/I61/receipt_experiment_03/RETURN.md", "RESUME_2026-09-30/BRIEFS/I61_U3_FACADE_CAPTURE.md"]
out = {"basis": {"num_revision": REV, "full": git("rev-parse", REV).strip(),
                 "u1_merge": "4a13e369b9", "u3_grant1": "bee3dc07ca", "g3_basis": "5ae5fe4f0f"},
       "sources": {f"P/{p}": at(P + p) for p in SOURCES},
       "statics": {f"P/{p}": at(P + p) for p in STATICS},
       "records": {}}
for rec in RECORDS:
    path = R + rec
    try:
        out["records"][f"T3/{rec}"] = at(path)
    except subprocess.CalledProcessError:
        data = open(os.path.join(repo, path), "rb").read()
        out["records"][f"T3/{rec}"] = {"blob": None, "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data),
                                        "note": "not in the basis tree; read from the working tree"}
print(json.dumps(out, indent=1))
