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
        "solve_prepared_load_case": "ec460ec2eb0e06a2e35fba545ad93656d16099974250a664397d63b7f27edae0",
        "append_curved_bend_macro_element_results": "9ebc4f11d1a881b3310579118792190c4d4c9c81964a3a89eb04952f0312304c",
    },
    # B7b: branch E is 0.5.0 with {3.0.0, exact_pressure_v3, euler_bernoulli}; the
    # component, region, section and load-state refusals that empty the carriers' children are unchanged.
    "core/product_physics/src/retained_memory.rs": {
        "namespace_branch": "b1f8f02df1f10552a69c357ebc329fa801243addc94d595403ab9313f79cf805",
        "family_clauses": "6f85658b105d1d38c0fa23d94aba52572c81273956764d4824371bc7c4cf2d3a",
    },
}
# B7b: the production prefix deletes v2's declaration, refusal and negative-pressure
# gate, adds the 0.5.0 contract/shear/corrosion refusals (diagnostics only) and lets
# PressureRegionInput decode member_connector_ids; admitted regions stay empty (E) or
# absent (L), so no carrier child is populated.
PRESSURE_PREFIX_PIN = "642e67b895762cf2bcede5dc471f66fe8c5705b70a2658fb82aeef65a6f6d6d1"
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
