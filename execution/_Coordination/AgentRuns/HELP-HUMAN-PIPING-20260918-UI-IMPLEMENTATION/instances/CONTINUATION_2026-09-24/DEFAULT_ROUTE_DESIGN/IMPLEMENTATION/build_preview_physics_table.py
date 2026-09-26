"""S1: build the preview-physics-1 semantic contract table deterministically.

Run from the repository root:
    python3 <this file> [--check]

Inputs are the frozen precision-1 and physics-1 tables. The output is
projects/chirality-piping/fixtures/results/semantic_contract_v0_3_preview_physics_1.json.
Standard library only. --check verifies the committed bytes instead of writing.
"""
import copy
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path("projects/chirality-piping/fixtures/results")
PRECISION = ROOT / "semantic_contract_v0_3_precision_1.json"
PHYSICS = ROOT / "semantic_contract_v0_3_physics_1.json"
OUT = ROOT / "semantic_contract_v0_3_preview_physics_1.json"

PREVIEW_ID = "openpipestress.result_semantics/0.3.0/preview-physics-1"
RETIRED = [
    "reaction_resultant",
    "open_formula_stress_summary",
    "component_user_stress_multiplier_review",
]
ARC_BASIS = "nominal_straight_beam_formula_on_arc_resultants"
ARC_COORDINATE = "arc_chord_frame"
INTENSIFIED_KIND = "component_equal_factor_intensified_bending_stress_v1"
INTENSIFIED_COMPONENT = "equal_factor_intensified_bending_stress"
INTENSIFIED_BASIS = "user_sif_times_member_section_bending_stress_v1"
ARC_STRESS_KINDS = [
    "element_local_axial_normal_stress",
    "element_local_bending_normal_stress_y",
    "element_local_bending_normal_stress_z",
    "element_local_torsional_shear_stress",
    "pipe_section_pressure_hoop_stress",
    "pipe_section_pressure_longitudinal_stress",
]
LINEAR_STATE_KINDS = [
    "global_nodal_displacement_x",
    "global_nodal_displacement_y",
    "global_nodal_displacement_z",
    "global_nodal_rotation_x",
    "global_nodal_rotation_y",
    "global_nodal_rotation_z",
    "element_local_axial_force",
    "element_local_shear_force_y",
    "element_local_shear_force_z",
    "element_local_torsional_moment",
    "element_local_bending_moment_y",
    "element_local_bending_moment_z",
    "element_local_axial_normal_stress",
    "element_local_bending_normal_stress_y",
    "element_local_bending_normal_stress_z",
    "element_local_torsional_shear_stress",
    "pipe_section_pressure_hoop_stress",
    "pipe_section_pressure_longitudinal_stress",
    "support_reaction_component_v2",
    "constant_effort_support_applied_load",
]
LIMITATIONS = [
    "Small-displacement, linear-elastic Euler-Bernoulli frame preview; numerical integrity does not establish physical correctness.",
    "Nonzero pressure is refused on this route, and so is legacy imposed_displacement.",
    "On straight members the normal-stress maximum is |N/A| + hypot(My,Mz)/Z, bounded over all statics intervals. Torsional shear is separate. No transverse shear, equivalent stress or code stress.",
    "On arcs: signed tangent-frame resultants and nominal stress components. Endpoint force rows are in the chord frame. No maximum, and no stress headline for a model containing an arc.",
    "Intensified measures are i*hypot(My,Mz)/Z with the user's scalar SIF at member ends adjacent to markers and branches, using member Z. They are not code stresses and are never combined. A flexibility factor never multiplies stress.",
    "Signed support actions for rigid restraints, springs, consuming constant effort and attributable nonlinear supports. Ambiguous attribution is withheld.",
    "Headlines cover all load cases. Mechanics combinations are withheld for nonlinear supports, constant effort with sum of factors other than 1, or mixed moduli. Subtraction is a labelled difference. No combination maxima and no code compliance.",
]


def build():
    precision = json.loads(PRECISION.read_text(encoding="utf-8"))
    physics = json.loads(PHYSICS.read_text(encoding="utf-8"))
    physics_by_id = {row["signature_id"]: row for row in physics["rows"]}
    rows = []
    for row in precision["rows"]:
        if row["kind"] in RETIRED:
            continue
        if row["kind"] in ARC_STRESS_KINDS:
            # The arc variant precedes the generic variant. Every reader takes
            # the first variant whose source_basis is absent or equal to the
            # row's metadata.basis, in table order.
            arc = copy.deepcopy(row)
            arc["signature_id"] = "preview-physics-arc-" + row["signature_id"].rsplit("-", 1)[-1]
            arc["source_basis"] = ARC_BASIS
            rows.append(arc)
        rows.append(copy.deepcopy(row))
    for signature_id in [f"supported-source-{n:03d}" for n in range(60, 69)]:
        rows.append(copy.deepcopy(physics_by_id[signature_id]))
    rows.append(
        {
            "signature_id": "preview-physics-intensified-bending",
            "kind": INTENSIFIED_KIND,
            "unit": "Pa",
            "component": INTENSIFIED_COMPONENT,
            "source_physical_semantic_dimension": "stress",
            "derivative_target_dimension": "stress",
            "category": "physical_quantity",
            "family": "stress",
            "canonical_disposition": "exported_quantity",
            "legacy_declared_dimension": None,
            "legacy_run_creation_admission": "throws_ANALYSIS_RUN_RESULT_DIMENSION_UNDECLARED",
            "governing_ratio_eligible": False,
        }
    )
    vocabulary = copy.deepcopy(precision["canonical_metadata_vocabulary"])
    for token in ["Fx", "Fy", "Fz", "Mx", "My", "Mz", "force_magnitude", "moment_magnitude",
                  "maximum_absolute_normal_stress", INTENSIFIED_COMPONENT]:
        if token not in vocabulary["component"]["enum"]:
            vocabulary["component"]["enum"].append(token)
    vocabulary["coordinate_system"]["enum"].append(ARC_COORDINATE)
    vocabulary["location"]["enum"].append("governing_station")
    for token in ["recovered_from_assembled_support_law", ARC_BASIS, INTENSIFIED_BASIS]:
        vocabulary["basis"]["enum"].append(token)
    table = {
        "schema_version": "0.3.0",
        "source_signature_count": len(rows),
        "source_kind_count": len({row["kind"] for row in rows}),
        "canonical_metadata_vocabulary": vocabulary,
        "rows": rows,
        "hash_vectors": copy.deepcopy(precision["hash_vectors"]),
        "semantic_contract_id": PREVIEW_ID,
        "source_schema_version": precision["source_schema_version"],
        "source_producer": copy.deepcopy(precision["source_producer"]),
        "metadata_policy": "preserve_source_producer_numerical_quality_formulation_basis_contract_evidence",
        "reserved_inactive_successors": [
            "openpipestress.result_semantics/0.3.0/reactions-1",
            "openpipestress.result_semantics/0.3.0/pressure-1",
            "openpipestress.result_semantics/0.3.0/stress-1",
        ],
        "inherited_semantic_contract_sha256": hashlib.sha256(PRECISION.read_bytes()).hexdigest(),
        "retired_source_kinds": RETIRED,
        "source_basis_signature_policy": "where_declared_require_exact_metadata_basis_first_match_in_table_order",
        "formulation_profile_id": "product_preview_mechanics_v1",
        "contract_evidence_policy": "closed_preview_case_extrema_coverage_attribution_intensification_and_combination_gate_binding_v1",
        "combination_policy": {
            "policy_id": "PREVIEW-PHYSICS-1-COMBINATION-v1",
            "mechanics_gates": [
                {"code": "NONLINEAR_COMBINATION_REQUIRES_SOLVE",
                 "condition": "the model has any nonlinear support record, including one inactive in every case"},
                {"code": "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE",
                 "condition": "the model has a consuming constant-effort support and |sum(factors) - 1| > 64*epsilon*max(1, sum(|factors|))"},
                {"code": "COMBINATION_MODULUS_BASIS_MIXED",
                 "condition": "the mechanics operands have different modulus-basis keys"},
            ],
            "gate_precedence": "first gate in list order supplies the single withheld reason",
            "gated_forms": ["mechanics"],
            "linear_state_kinds": LINEAR_STATE_KINDS,
            "recomputed_magnitude_kinds": [
                "displacement_magnitude",
                "support_reaction_force_magnitude_v2",
                "support_reaction_moment_magnitude_v2",
            ],
            "result_state_subtraction": "signed difference of solved states over linear_state_kinds, with recomputed magnitudes; no equilibrium claim",
            "range_envelope": "mode selection per row over linear_state_kinds and recomputed_magnitude_kinds; not a simultaneous state",
            "never_combined": "every other kind, including pipe_elastic_normal_stress_maximum_v2, " + INTENSIFIED_KIND + ", nonlinear support kinds, reviews, records, counts, codes and residuals",
        },
        "supported_profile_limitations": LIMITATIONS,
    }
    return json.dumps(table, indent=2, ensure_ascii=True) + "\n"


def main():
    text = build()
    if "--check" in sys.argv:
        if OUT.read_text(encoding="utf-8") != text:
            raise SystemExit("preview-physics-1 table differs from its generator")
        print("OK", hashlib.sha256(OUT.read_bytes()).hexdigest())
        return
    OUT.write_text(text, encoding="utf-8")
    print(hashlib.sha256(OUT.read_bytes()).hexdigest())


if __name__ == "__main__":
    main()
