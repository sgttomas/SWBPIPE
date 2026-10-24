"""B5O1: source binding of batch 5's additional positive carriers.

The domain is unchanged. In particular, no child backing is credited merely
because the joint feature is dormant. Existing D1 component/region refusals
establish the empty children; actual carrier layouts stay positively priced.
"""
import hashlib
import os
import re
import runpy

HELPERS = runpy.run_path(os.path.join(os.path.dirname(__file__), "kd5_text_model.py"))
PINS = {
    "core/product_physics/src/lib.rs": {
        "solve_load_case_observed": "5a70782515a2e66790c8d570cc13d49ead9d22614e0f99b288d2632e8fc4e061",
        # T4-U6/U8: D-5's stress rows, per-basis maxima, extrema and coverage (priced at their own sites and by the
        # U6 rules); its carriers, staging and prepared-pressure calls are unchanged. Epoch addbdc030e: the case
        # evidence's `pipe_sections` is `exact_pipe_sections(built)`, the same entries plus the realized-arc marker
        # (no realized arc on D1). O4 S5 (retained-pricing-rules.md §8.9, O4-SR): a selected case's maximum is one
        # composite_member_maximum call per basis where the two stress sections differ (the reduced one on its own
        # section) and SOURCE_MAXIMUM_BASES_DISTINCT is deleted; the warning pushes the error by clone. Carriers,
        # staging and the prepared-pressure calls are unchanged.
        # Pre-0.4 failure unification D3 (§8.14): the screen and withheld decline on both exact routes, the support-norm
        # cause String and the route-neutral cause records (PU-TX); carriers, staging and prepared-pressure calls unchanged.
        "solve_prepared_load_case": "96aab83751549846c8898defa59da887fdde529a74d231862b2ef454f109c4e8",
        "append_curved_bend_macro_element_results": "9ebc4f11d1a881b3310579118792190c4d4c9c81964a3a89eb04952f0312304c",
    },
    # B7b: branch E is 0.5.0 with {3.0.0, exact_pressure_v3, euler_bernoulli}; the
    # component, region, section and load-state refusals that empty the carriers' children are unchanged.
    # T4-U6/U8 (O4): namespace_branch refuses the Timoshenko exact contract (ShearModel) and family_clauses a nonzero
    # mill tolerance or allowance on branch E (SectionAllowance); both narrow admission only.
    # W1: family_clauses only refreshes its stage-1 comment to the implemented
    # EB member-load route; executable refusals and carrier-child premises match.
    # O4 S5 (§8.9, O4-AD): the T4-U6/U8 narrowings are removed: namespace_branch admits either shear model through
    # contract_is and family_clauses drops the branch-E section-allowance refusal. The component, region and
    # load-state refusals that empty the carriers' children are unchanged.
    "core/product_physics/src/retained_memory.rs": {
        "namespace_branch": "b1f8f02df1f10552a69c357ebc329fa801243addc94d595403ab9313f79cf805",
        "family_clauses": "e234ac88ff7a894707225c05d6c361c85492263749177d15984c654247ac2603",
    },
}
# B7b: the production prefix deletes v2's declaration, refusal and negative-pressure
# gate, adds the 0.5.0 contract/shear/corrosion refusals (diagnostics only) and lets
# PressureRegionInput decode member_connector_ids; admitted regions stay empty (E) or
# absent (L), so no carrier child is populated.
# T4-U6 (D-5): ExactPressurePipeState holds ExactSectionBases in place of one SourceAnnulus; every changed production
# line is in the per-region member body, the joint/factor-group terms or closure_source, all reached only through a
# pressure region (absent on L, explicitly empty on E). No carrier or staging changes.
# PP: validate_profile admits both shear models (one fewer refusal arm), selects_timoshenko is a predicate, and
# closure_source's straight operand identity begins with its member prefix (region path only).
# Epoch addbdc030e: assemble_pressure_case's per-region member body calls `super::realized_arc` and
# `super::mark_realized_arc` in place of the same inline expressions (region path only).
# T4-S5 (epoch): the prefix drops the arc withheld policy and reason (ARC_WITHHELD_REASON, exact_member_maximum_policy),
# reads 0.6.0 in two validate_profile texts and, on a realized arc only, adds bend_radius_m to the member geometry and
# the arc_stress_definitions evidence (region path only: absent on L, explicitly empty on E). No carrier, child or
# staging changes. solve_prepared_load_case's U4b rows, arc maxima and corrected displacement are priced at their own
# sites (U4B-ST, U5B-PD); family_clauses changes only its 0.6.0 comment.
# Model document 0.7.0 D3 (W-13 S1b): the prefix reads 0.7.0 in the same two validate_profile texts (same length);
# family_clauses changes only its 0.7.0 comment. No carrier, child or staging changes.
PRESSURE_PREFIX_PIN = "4541c94c88fc90988a7746e3f86622c574338b098778190457f97193e763bcc9"
LEGACY_PRESSURE_PREFIX_PIN = "d335b7c22bee4bf5832a4e2b9f82f63a77930aa2bbcedeab15369e62c205b017"
LEGACY_CASE_PIN = "273d31e03921bad5b4d28852b54b17ba7e70acd85d590b4213a3d5d8141aa00c"


def active(snapshot):
    file = "core/product_physics/src/lib.rs"
    source = open(os.path.join(snapshot, file), encoding="utf-8").read()
    pressure = open(os.path.join(snapshot, "core/product_physics/src/pressure_runtime.rs"), encoding="utf-8").read()
    pressure_digest = hashlib.sha256(pressure.split("#[cfg(test)]", 1)[0].encode()).hexdigest()
    if not re.search(r"^fn solve_prepared_load_case\b", source, re.M):
        case_digest = hashlib.sha256(HELPERS["function_body"](source, "solve_load_case_observed").encode()).hexdigest()
        if pressure_digest != LEGACY_PRESSURE_PREFIX_PIN or case_digest != LEGACY_CASE_PIN:
            raise ValueError("B5O1: prepared helper absent but producer/consumer is not the reviewed registered source; refusing historical pricing")
        return False
    if pressure_digest != PRESSURE_PREFIX_PIN:
        raise ValueError("B5O1: pressure producer/declaration source changed; re-review every owner, child and staging bound")
    for file, functions in PINS.items():
        source = open(os.path.join(snapshot, file), encoding="utf-8").read()
        for name, digest in functions.items():
            body = HELPERS["function_body"](source, name)
            if hashlib.sha256(body.encode()).hexdigest() != digest:
                raise ValueError(f"B5O1: carrier/caller source changed: {file}:{name}; re-review staging and owned children")
    return True


def descriptor_recovery_bytes(snapshot):
    """B5A1/C5: the fixed recovery label is charged inside the format literal."""
    file = "core/product_physics/src/lib.rs"
    source = open(os.path.join(snapshot, file), encoding="utf-8").read()
    name = "append_curved_bend_macro_element_results"
    body = HELPERS["function_body"](source, name)
    if hashlib.sha256(body.encode()).hexdigest() != PINS[file][name]:
        raise ValueError("B5A1: descriptor body changed; re-review literal ownership and formation")
    expected = "node_on_element_end_actions_from_assembled_stiffness_in_end_tangent_frame"
    templates = [value for value in HELPERS["literals"](body) if value.startswith("component_family=bend;")]
    if len(templates) != 1 or re.findall(r";recovery=([^;]+);", templates[0]) != [expected]:
        raise ValueError("B5A1: missing, changed or duplicate unconditional recovery literal")
    if len(expected.encode()) != 73 or HELPERS["literal_bytes"](templates[0]) != 631:
        raise ValueError("B5A1: current descriptor literal byte inventory changed")
    return 73
