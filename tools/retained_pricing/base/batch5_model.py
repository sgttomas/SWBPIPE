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
        "solve_prepared_load_case": "b8aaad6f6965068f336453331e20c97ffc9514524eb388b828b01d7ee4c67745",
    },
    "core/product_physics/src/retained_memory.rs": {
        "namespace_branch": "9b5a3838fb24726927df79203438ef1fefaa8506b6aa33e74c389b4c3a5b297b",
        "family_clauses": "0432a122cff1359e0a372c8584ea4003404845af06c5eeba8d8ebfebc1d06da1",
    },
}
PRESSURE_PREFIX_PIN = "205f686417d787f570e72aa3ea1a3230863779955faa7d4c439aad39075bf861"
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
