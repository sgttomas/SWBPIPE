"""KD5T2: exhaustive fixed-label/text inventory, guarded by reviewed source pins.

Only the four certified record constructors produce the group labels consumed
by the pinned D1 case loop and pinned recovery finding. Their private group
helper copies those arguments. A changed producer/consumer stops for review;
no field is presumed fixed merely because a fixture showed it.
"""
import hashlib, os, re

PINS = {'core/product_physics/src/certified_publication.rs': {'group': 'ec5db3c8e95c345590bf4472a4310f155259cb9ced56b58da3c0316b62b0f394', 'magnitude_group': '600d0d42c5dbb53c489b4bf86a1fba82fdcc8d00440e4f86a2cfb6e6d7bfd953', 'member_record': '201b9a9c9390794c15207731efe9f524d1c14f0c80a15afa02b1e3b601580547', 'support_record': '1b5b46b845578a05a14879b1bae657f832f48a5b2122b404350588490571fd44', 'unavailable_record': '79931f5662ae9277f7ddbc008799340757f0e581e29a053cbdd90ff7a0eaf1a6', 'connector_record': '195b0ec1b1d66c2512cd41541f8fc4ba6bfe08b2e7806a64057600f3974f2313', 'straight_stress_spans': 'ab6bb7b081a367e4782fdb07fb115e587184e7468b3a310e8dbf38c76fa0ba61', 'fallback_circular_maximum': 'fc6978a4f0f6b82e4348388f34715c311ecccdb7e8b01537711cbd282e207ff2', 'fallback_product_up': '5476bd4c93f66d5aee68c465b7da0ee39ab1d123dc75825931f2502cf0b607bb'}, 'core/product_physics/src/formation_guard.rs': {'recovery_finding': '5a16ba3e4680f456ec998edbad20ef80a7fdf3edb0c85665bafac61dfb0c19b0', 'member_group_decision': '76a37c824097b528c638214d913166d24ffa81698222ad9199a511f32f0d5e59'}, 'core/product_physics/src/preview_physics.rs': {'extrema_evidence': 'ac7bcd693cb6d4d4b41f8bbdc8f945b4e76fddc5362e74eb30b51c2785daf825'}, 'core/product_physics/src/lib.rs': {'solve_load_case_observed': '4b96ef49a3ab5494bc79c3425adaf5b67e1fe072a1043653ba040c37f15871db'}}

PRODUCTION_PINS = {'core/product_physics/src/certified_publication.rs': '822cf28ac85c9a41856f7b1c8d8bace246d913044a7c3cbe8ce344186120e3a8', 'core/solver/frame_kernel/src/structural/certified_recovery.rs': 'ab2a378f57eb1922c84ab006f9d58f04bf5c37c3b553bed0f6fc910cf507e034', 'core/solver/frame_kernel/src/exact_sum.rs': '35ae1c71195a95d88669c658965198e7dc2476cf32d948e4bfc62ef184e1051c'}

# Join re-reviewed alternatives (the pressure-source join merged with K-D5). Each
# differs from its K-D5-reviewed pin only as stated; the K-D5 value stays
# accepted so the registered revision still prices. Keys: (file, function or
# None for the file-level pin).
JOIN_REVIEWED = {
    # Production prefix: only the four norm2/norm3 call paths, crate::correct_norm
    # -> the shared open_pipe_stress_correct_norm crate (the same functions).
    ("core/solver/frame_kernel/src/structural/certified_recovery.rs", None):
        "ae7855ec7bd9432018b09be40f25281f0bd13254e966eb51e232444592cbb483",
    # Only the removed not-joined diagnostic call; route strings, record labels
    # and attempt/clone consumers are unchanged.
    ("core/product_physics/src/lib.rs", "solve_load_case_observed"):
        "afa68f9aca96d5df206e0367776520e933777ceda87cb30a377029e6139cce52",
    # Non-test source: only the removed `#[path]` correct_norm module; the closed
    # StressFinding subject producer and its one copy site are unchanged.
    ("core/loads/stress_recovery/src/lib.rs", None):
        "7df37b7604d7f201a265a2d4d6d0772c8e31b757ff1130d17ce8a17df70b87ff",
}

def accepted(file, name, actual, pinned):
    """The K-D5-reviewed digest, or the join's re-reviewed alternative for that key."""
    return actual == pinned or JOIN_REVIEWED.get((file, name)) == actual

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
    for file, digest in PRODUCTION_PINS.items():
        with open(os.path.join(snapshot, file), encoding="utf-8") as stream:
            source = stream.read().split("#[cfg(test)]", 1)[0]
        if not accepted(file, None, hashlib.sha256(source.encode()).hexdigest(), digest):
            raise ValueError(f"KD5 source/slot inventory pin changed: {file}; re-review owned layouts and caller/workspace unions")
    for file, functions in PINS.items():
        with open(os.path.join(snapshot, file), encoding="utf-8") as stream:
            source = stream.read()
        for name, digest in functions.items():
            body = function_body(source, name)
            if not accepted(file, name, hashlib.sha256(body.encode()).hexdigest(), digest):
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
    with open(os.path.join(snapshot, "core/product_physics/src/lib.rs"), encoding="utf-8") as stream:
        consumer_source = stream.read()
    sign_values = []
    for name in ("STRAIGHT_ENDPOINT_SECTION_SIGN_CONVENTION", "CURVED_BEND_SECTION_SIGN_CONVENTION"):
        hits = re.findall(r"const " + name + r': &str = "([^"\\]+)";', consumer_source)
        if len(hits) != 1:
            raise ValueError(f"KD5A7: expected one unescaped sign constant {name}")
        sign_values += hits
    comparison_reasons = literals(bodies["member_group_decision"])
    coefficient_basis = "j_side_section_equilibrium_binary64_enclosures"
    enclosure_scope = "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate"
    for producer in ("extrema_evidence", "solve_load_case_observed"):
        values = literals(bodies[producer])
        if coefficient_basis not in values or enclosure_scope not in values:
            raise ValueError(f"KD5T2: current maximum literal absent in {producer}")
    return {"labels": labels, "label_bytes": max(len(value.encode()) for value in labels),
            "comparison_reason_bytes": max(len(value.encode()) for value in comparison_reasons),
            "sign_bytes": max(len(value.encode()) for value in sign_values),
            "item_comparison_literal": one("member comparison unavailable"),
            "coefficient_basis_bytes": len(coefficient_basis.encode()), "enclosure_scope_bytes": len(enclosure_scope.encode()),
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
    if not accepted(file, None, hashlib.sha256(source.encode()).hexdigest(), STRESS_SUBJECT_SOURCE_PIN):
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

# KD5C5: only exact outside-caller candidates into the private non-escaping
# FallbackError owner are removed; every other resolver candidate is retained.
FALLBACK_METHOD_PINS = {
    "add": "6cf32ee08ae1324229b3332932b16be33f710e3db38f38799285b70a170b555d",
    "mul": "b7a13d3aa95f1a81250596c2894aa53c08261ef50678482907ca925eb89fd375",
    "divided": "42236c96706c6fd58796004eca2d9791bd37e011f0e77c1ee482673d74a501d8",
}
FALLBACK_OUTSIDE_CALLERS = (
    ("annulus_geometry.rs", "source_wall_area_scaled", ("mul",), "19e9559e151064359ff00db6a27c90c64ec619ef465f6dc90d077795850821f7"),
    ("annulus_geometry.rs", "source_bore_area_scaled", ("mul",), "13e243a2122d8717d5741c8698206bf8dac7d6673e91a5ffa4052252649e8a61"),
    ("annulus_geometry.rs", "source_insulation_area_m2", ("mul",), "41ec087e427844f47f407ed550d9124e6cfe773e98ab61a99335e08a47a68b41"),
    ("lib.rs", "exact_straight_summary_extrema", ("add",), "c1905b2780be30a7d74ef74e2f5826c02f01ed9b6c92908bfe0a5e36d3f120e1"),
    ("lib.rs", "append_exact_pressure_results", ("divided",), "c65eecfd90235e5adf464f7149a377ac79ae9474cd99a2933405120b4b9d5f77"),
    ("pressure_exact.rs", "from_radii", ("mul",), "30bf50858d1c5a2223b5fa9c4f5a4d2643c87d26875e394770b6d69d89230340"),
    ("pressure_exact.rs", "lame_at_radius", ("mul",), "4cc46083ebb6e0a4ae0b1fdd58b9219cefd77f44ac81ca6021b77845c7cded64"),
    ("pressure_exact.rs", "eigenload_pair", ("mul",), "ce624c341c68c90e445856b617fe49fbcb81c7e5591c95e8e174f1d9a94413e9"),
    ("preview_physics.rs", "render", ("divided",), "eb43e10bf8292eaf21706a887b4b28d6d616fc9216ef29ef86339dfad0a3f4db"),
    ("retained_memory.rs", "strings", ("add",), "b34bc5c8a6f6863d21fd93b87baae4f33e47467d1f639086650fe6a22bd1a585"),
    ("retained_memory.rs", "retained_error_text", ("add",), "f50e721160819efd5fc3c5b4fb3dae56c70989cf4cea9d3dd75f6a546507a666"),
    ("retained_memory.rs", "complete_observations", ("add",), "8f65af0bdd6c83d559966214eb77b308bea940f917ebb210e70d27e9bfef2fc2"),
    ("pressure_exact/source_geometry.rs", "from_od_wall", ("add", "mul"), "0c2ac197402cbbfb8ceae54cb1c18acaefbcb1a26f658c11f88b88d33e1e1df9"),
    ("pressure_exact/source_geometry.rs", "pressure_group_value", ("mul",), "ed2e3f79a1b883da9a7b6e9ff6a5cf05c03937b6a89a77ec681c40594e6689e5"),
    ("pressure_exact/source_geometry.rs", "surface_stresses", ("add",), "98a1e66fcdfeea1249ec0ed2d6aaaf28cacb8708761aa7a723500d67bf8765a9"),
    ("case_state/temperature.rs", "exact_kelvin", ("mul",), "4b7c432e3c502bfd1fd0f32a43e73a93c1bed6710bd44e2958ee497d10afa02d"),
    # The join's J7 census (#[cfg(test)] only): both `.mul(` receivers are the
    # validation crate's exact rational `Exact` (`e`, and `r1_exact::tolerance`'s
    # result), whose own `Exact::mul` is called; never FallbackError::mul.
    ("source_recovery/j7_rf_census.rs", "method_row_holds", ("mul",), "0b2cf6a2031afa16a70e0842301a52b87b88cdcf60c61a5dc83b1f0788d407b8"),
)
JOIN_ONLY_CALLER_FILES = {"source_recovery/j7_rf_census.rs"}

def _indented_function(text, name):
    hits = list(re.finditer(r"^([ \t]*)(?:pub(?:\([a-z]+\))?\s+)?(?:const\s+)?fn\s+" + re.escape(name) + r"\b", text, re.M))
    if len(hits) != 1:
        raise ValueError(f"KD5C5: expected one fn {name}, found {len(hits)}")
    hit = hits[0]
    end = re.search(r"^" + re.escape(hit.group(1)) + r"}", text[hit.end():], re.M)
    if not end:
        raise ValueError(f"KD5C5: no same-indent end of fn {name}")
    return text[hit.start():hit.end() + end.end()], text.count("\n", 0, hit.start()) + 1

def inspect_fallback_boundary(snapshot):
    file = "core/product_physics/src/certified_publication.rs"
    with open(os.path.join(snapshot, file), encoding="utf-8") as stream:
        source = stream.read().split("#[cfg(test)]", 1)[0]
    if hashlib.sha256(source.encode()).hexdigest() != PRODUCTION_PINS[file]:
        raise ValueError("KD5C5: private owner/production/escape boundary changed; re-review the exact target exclusions")
    if len(re.findall(r"^struct FallbackError \{", source, re.M)) != 1 or len(re.findall(r"^impl FallbackError \{", source, re.M)) != 1:
        raise ValueError("KD5C5: private struct/impl inventory changed")
    targets = {}
    for name, digest in FALLBACK_METHOD_PINS.items():
        body, line = _indented_function(source, name)
        if not body.startswith("    fn " + name + "(") or hashlib.sha256(body.encode()).hexdigest() != digest:
            raise ValueError(f"KD5C5: private method/body changed: {name}")
        targets[name] = f"{file}:{line}:{name}"
    inventory = []
    expected = 17
    for relative, name, methods, digest in FALLBACK_OUTSIDE_CALLERS:
        caller_file = "core/product_physics/src/" + relative
        if relative in JOIN_ONLY_CALLER_FILES:
            # A join-only caller is bound only where the join's file exists; a
            # K-D5 snapshot keeps its exact original 17-pair inventory.
            if not os.path.exists(os.path.join(snapshot, caller_file)):
                continue
            expected += len(methods)
        with open(os.path.join(snapshot, caller_file), encoding="utf-8") as stream:
            text = stream.read()
        body, line = _indented_function(text, name)
        if hashlib.sha256(body.encode()).hexdigest() != digest:
            raise ValueError(f"KD5C5: outside caller/receiver body changed: {caller_file}:{name}")
        if caller_file == file:
            raise ValueError("KD5C5: an inside-owner caller cannot be excluded")
        inventory.extend({"caller": f"{caller_file}:{line}:{name}", "name": method,
                          "exclude_targets": [targets[method]]} for method in methods)
    if len(inventory) != expected or len({(r["caller"], r["name"]) for r in inventory}) != expected:
        raise ValueError("KD5C5: exact outside caller/method inventory changed")
    return inventory
