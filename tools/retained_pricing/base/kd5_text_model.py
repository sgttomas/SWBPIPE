"""KD5T2: exhaustive fixed-label/text inventory, guarded by reviewed source pins.

Only the four certified record constructors produce the group labels consumed
by the pinned D1 case loop and pinned recovery finding. Their private group
helper copies those arguments. A changed producer/consumer stops for review;
no field is presumed fixed merely because a fixture showed it.
"""
import hashlib, os, re

PINS = {'core/product_physics/src/certified_publication.rs': {'group': '00d97e3e62b9533ee1bf49c5e8dad88d38028b9f29a173115182e31ce322a169', 'member_record': '201b9a9c9390794c15207731efe9f524d1c14f0c80a15afa02b1e3b601580547', 'support_record': '854169c4f86d9fc8c96aa642f8295c6f6ccf5e7c5f0ff9103044a2c70d8fb774', 'unavailable_record': 'dce5657004aeb2807d7c3fe6a8814c7963df73d73422a583e5465efd8ea6f407', 'connector_record': '195b0ec1b1d66c2512cd41541f8fc4ba6bfe08b2e7806a64057600f3974f2313'}, 'core/product_physics/src/formation_guard.rs': {'recovery_finding': '8182d4258b0b7ddd452ffb5df65619c1283abf3e055916e2bc220d233e4b8307'}, 'core/product_physics/src/lib.rs': {'solve_load_case_observed': '74c5cead7b08e7456f604bd026ccafd13f35516cfac172aef217fedc0d4c4894'}}

def function_body(text, name):
    hits = list(re.finditer(r"^(?:pub(?:\([a-z]+\))?\s+)?fn\s+" + re.escape(name) + r"\b", text, re.M))
    if len(hits) != 1:
        raise ValueError(f"KD5T2: expected one fn {name}, found {len(hits)}")
    end = text.find("\n}", hits[0].start())
    if end < 0:
        raise ValueError(f"KD5T2: no top-level end of fn {name}")
    return text[hits[0].start():end + 2]

def literals(text):
    return re.findall(r'"((?:\\.|[^"\\])*)"', text)

def literal_bytes(template):
    if "{{" in template or "}}" in template:
        raise ValueError("KD5T2: escaped format braces require review")
    return len(re.sub(r"\{[^{}]*\}", "", template).encode())

def inspect(snapshot):
    bodies = {}
    for file, functions in PINS.items():
        with open(os.path.join(snapshot, file), encoding="utf-8") as stream:
            source = stream.read()
        for name, digest in functions.items():
            body = function_body(source, name)
            if hashlib.sha256(body.encode()).hexdigest() != digest:
                raise ValueError(f"KD5T2: source pin changed: {file}:{name}; re-review the producer/consumer inventory")
            bodies[name] = body
    constructors = ("member_record", "support_record", "unavailable_record", "connector_record")
    labels = sorted(set(value for name in constructors for value in literals(bodies[name])))
    if not labels:
        raise ValueError("KD5T2: no fixed labels")
    templates = literals(bodies["recovery_finding"])
    def one(fragment):
        hits = [value for value in templates if fragment in value]
        if len(hits) != 1:
            raise ValueError(f"KD5T2: expected one template containing {fragment!r}, found {len(hits)}")
        return literal_bytes(hits[0])
    routes = [value for value in literals(bodies["solve_load_case_observed"]) if value.startswith("on ") and value.endswith(" route")]
    if not routes:
        raise ValueError("KD5T2: no fixed route strings")
    return {"labels": labels, "label_bytes": max(len(value.encode()) for value in labels),
            "item_numeric_literal": one(": q="), "item_unavailable_literal": one("member bound is unavailable"),
            "sentence_literal": one("S11-G recovery guard"), "route_bytes": max(len(value.encode()) for value in routes)}

# KD5T3: this is one closed private constructor's allocation site, not
# a type-wide claim about the public StressFinding.subject_id field.
STRESS_SUBJECT_SOURCE_PIN = "4526e43a3dca37a265516ad5556a07efb9f25693fca6ee8e065a8211637c47c1"
STRESS_SUBJECTS = ("analysis_status", "area", "axial_force", "axial_normal_range",
    "bending_moment_y", "bending_moment_z", "bending_normal_y_range", "bending_normal_z_range",
    "first_state", "second_state", "section_modulus_y", "section_modulus_z",
    "torsion_constant", "torsion_radius", "torsional_moment", "torsional_shear", "torsional_shear_range")

def inspect_stress_subjects(snapshot):
    file = "core/loads/stress_recovery/src/lib.rs"
    with open(os.path.join(snapshot, file), encoding="utf-8") as stream:
        source = stream.read().split("#[cfg(test)]", 1)[0]
    if hashlib.sha256(source.encode()).hexdigest() != STRESS_SUBJECT_SOURCE_PIN:
        raise ValueError("KD5T3: stress subject producer/forwarding source changed; re-review the closed caller inventory")
    sites = [i + 1 for i, line in enumerate(source.splitlines()) if "subject_id: subject_id.into()," in line]
    if len(sites) != 1:
        raise ValueError(f"KD5T3: expected one StressFinding subject copy: {sites}")
    return {"file": file, "line": sites[0], "subjects": list(STRESS_SUBJECTS),
            "max_utf8_bytes": max(len(value.encode()) for value in STRESS_SUBJECTS)}

# KD5O13: the three-frame FK directed-comparison workspace, independent
# of PP fallback exact-product staging and PP certificate callers.
DIRECTED_HELPER_PINS = {
    "nonnegative_parts": "a0ffb3fbb4dc0657023b94d61d9ce43ed68341e3e8faec2a2fb521600e9ea35e",
    "compare_nonnegative_product": "2be2d814849b0d8c1dd47d6d9a836cbeb5fb5cf3d29bfec2c82227649ee1fb8f",
    "product_upward": "fef93d9c5b919388fc63f599c94f83eee36b01f9482a88e387ffd0acd5837941",
    "product_downward": "a4cc23b84fdff0dd6006dfc7e8dfdf279864ed8b470079e34f47ac1db481f90c",
    "quotient_downward": "718019f27189b9ed7cebbf4375c4924c8824d5352fb4b6024137f8dd12ae2cc3",
}

# Known original wrappers are identical at basis2243be380b, registered
# B3 and U2 snapshots. Comparator absence alone is never a history proof.
LEGACY_DIRECTED_WRAPPER_PINS = {
    "product_upward": "06d0575424d34adb6055e8b4b42375e4a675e0b58d0c87c851c093d2b850f3ae",
    "product_downward": "301ada5902141c20c00d5aa4503f03e7fc4005980ec299c224a85ae98881e037",
    "quotient_downward": "153d1a54be95416eb09f59fbc635aac1a3e90653fa4c805d3b1e9a297544b198",
}

def inspect_directed_helpers(snapshot):
    file = "core/solver/frame_kernel/src/load_ledger.rs"
    with open(os.path.join(snapshot, file), encoding="utf-8") as stream:
        source = stream.read()
    if "fn compare_nonnegative_product(" not in source:
        for name, digest in LEGACY_DIRECTED_WRAPPER_PINS.items():
            if hashlib.sha256(function_body(source, name).encode()).hexdigest() != digest:
                raise ValueError(f"KD5O13: comparator absent but {name} is not a pinned original wrapper; refusing historical zero pricing")
        if re.search(r"^fn\s+nonnegative_parts\b", source, re.M):
            raise ValueError("KD5O13: comparator absent with new decomposition helper; refusing historical zero pricing")
        return False  # known original wrapper bodies, not name absence
    for name, digest in DIRECTED_HELPER_PINS.items():
        if hashlib.sha256(function_body(source, name).encode()).hexdigest() != digest:
            raise ValueError(f"KD5O13: directed helper source changed: {name}; re-review the three-frame slot census")
    return True
