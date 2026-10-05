"""I65 U4 G3: SHA-256 of every source and record this grant cites, read from Git objects
(read-only; GIT_OPTIONAL_LOCKS=0). Sources are hashed at NUM 5ae5fe4f0f (the revision every
source-reading run used) and the U1(a) files at 59a5de2032.
Usage: python3 origins.py <numerics repo root> > ORIGINS.json
"""
import hashlib, json, os, subprocess, sys
repo = sys.argv[1]
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
P = "projects/chirality-piping"
R = P + "/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30"
NUM = "5ae5fe4f0f"
U1A = "59a5de2032"
src = [
    "core/product_physics/src/lib.rs", "core/product_physics/src/source_recovery.rs",
    "core/product_physics/src/source_receipt.rs", "core/product_physics/src/source_receipt/rows.rs",
    "core/product_physics/src/source_receipt/source.rs", "core/product_physics/src/source_receipt/composite.rs",
    "core/product_physics/src/retained_product.rs", "core/product_physics/src/retained_memory.rs",
    "core/product_physics/src/self_weight.rs", "core/product_physics/src/validation.rs",
    "core/product_physics/src/pressure_runtime.rs", "core/product_physics/src/pressure_material.rs",
    "core/product_physics/src/case_state/resolve.rs", "core/product_physics/src/preview_physics.rs",
    "core/product_physics/src/formation_guard.rs", "core/product_physics/src/historical_pressure_reference.rs",
    "core/solver/frame_kernel/src/lib.rs", "core/solver/frame_kernel/src/load_ledger.rs",
    "core/solver/frame_kernel/src/structural.rs", "core/solver/frame_kernel/src/structural/exact_boundary.rs",
    "core/solver/frame_kernel/src/structural/exact_boundary/functionals.rs",
    "core/solver/frame_kernel/src/structural/formation_check.rs",
    "core/solver/frame_kernel/src/structural/retained/source.rs", "core/solver/frame_kernel/src/structural/retained/bound.rs",
    "core/solver/frame_kernel/src/structural/retained/adaptive.rs", "core/solver/frame_kernel/src/structural/retained/factor.rs",
    "core/solver/frame_kernel/src/structural/retained/wide.rs", "core/solver/frame_kernel/src/structural/retained/verify.rs",
    "core/solver/frame_kernel/src/structural/retained/seeded.rs", "core/solver/frame_kernel/src/structural/retained/origins.rs",
    "core/solver/frame_kernel/src/structural/retained/directed/certificate.rs",
    "core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs",
    "core/solver/nonlinear_integration/src/structural_adapter.rs", "core/solver/linear_supports/src/lib.rs",
    "core/loads/primitive_loads/src/lib.rs", "core/loads/stress_recovery/src/elastic_extrema.rs",
    "core/solver/straight_pipe/src/lib.rs", "core/units/src/lib.rs", "core/serialization/canonical_json/src/lib.rs",
    "fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json",
]
records = [
    "I65/u4_plan_01/PLAN.md", "I65/u4_g2_01/RESIDUALS.md", "I65/u4_g2_01/STACK_PLAN.md", "I65/u4_g2_01/API.md",
    "I65/u4_g2_01/_run_records/caps_arithmetic.py", "REVIEW_RV83/u4_g2_01/REVIEW.md",
    "I54/direct_ordinary_bound_02/BOUND.md", "I54/direct_ordinary_bound_02/W2_RETURN.md",
    "I54/direct_ordinary_bound_02/CHECKPOINT_A.md", "I54/direct_ordinary_bound_correction_03/ADDENDUM.md",
    "I54/direct_container_profile_01/COEFFICIENTS.md", "I51/public_producer_admission_05/COMPOSITION.md",
    "I29/f2a_preparation_counts_p1/COUNTS_AND_OWNERSHIP.md", "I29/f2a_kernel_ownership_p3/ENVELOPE.md",
    "I34/f2a_work_exactness_design_01/DESIGN.md", "I34/f2a_work_exactness_api_02/API_PLAN.md",
]
def blob(rev, path):
    return subprocess.run(["git", "-C", repo, "show", f"{rev}:{path}"], capture_output=True, env=env, check=True).stdout
out = {"num_revision": NUM, "u1a_commit": U1A, "sources": {}, "u1a_sources": {}, "records": {}}
for s in src:
    out["sources"]["P/" + s] = hashlib.sha256(blob(NUM, f"{P}/{s}")).hexdigest()
for s in ["core/product_physics/src/retained_product.rs", "core/product_physics/src/lib.rs"]:
    out["u1a_sources"]["P/" + s] = hashlib.sha256(blob(U1A, f"{P}/{s}")).hexdigest()
for r in records:
    out["records"]["R/" + r] = hashlib.sha256(blob(NUM, f"{R}/{r}")).hexdigest()
print(json.dumps(out, indent=1))
