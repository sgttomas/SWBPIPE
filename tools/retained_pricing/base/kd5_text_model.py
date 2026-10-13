"""KD5T2: exhaustive fixed-label/text inventory, guarded by reviewed source pins.

Only the four certified record constructors produce the group labels consumed
by the pinned D1 case loop and pinned recovery finding. Their private group
helper copies those arguments. A changed producer/consumer stops for review;
no field is presumed fixed merely because a fixture showed it.
"""
import hashlib, os, re

PINS = {'core/product_physics/src/certified_publication.rs': {'group': 'ec5db3c8e95c345590bf4472a4310f155259cb9ced56b58da3c0316b62b0f394', 'magnitude_group': '600d0d42c5dbb53c489b4bf86a1fba82fdcc8d00440e4f86a2cfb6e6d7bfd953', 'member_record': '201b9a9c9390794c15207731efe9f524d1c14f0c80a15afa02b1e3b601580547', 'support_record': '1b5b46b845578a05a14879b1bae657f832f48a5b2122b404350588490571fd44', 'unavailable_record': '79931f5662ae9277f7ddbc008799340757f0e581e29a053cbdd90ff7a0eaf1a6', 'connector_record': '9b6c0a2a68029af3f190eaa905693d0ad7b9f2bfd62ba19671921f16d9fe9674', 'straight_stress_spans': 'ab6bb7b081a367e4782fdb07fb115e587184e7468b3a310e8dbf38c76fa0ba61', 'fallback_circular_maximum': 'fc6978a4f0f6b82e4348388f34715c311ecccdb7e8b01537711cbd282e207ff2', 'fallback_product_up': '5476bd4c93f66d5aee68c465b7da0ee39ab1d123dc75825931f2502cf0b607bb'}, 'core/product_physics/src/formation_guard.rs': {'recovery_finding': 'c499ed59986310186a5825c2043363ca131a5b44d41755a7700e1f7d6c1f5f9b', 'member_group_decision': '76a37c824097b528c638214d913166d24ffa81698222ad9199a511f32f0d5e59'}, 'core/product_physics/src/preview_physics.rs': {'extrema_evidence': 'ac7bcd693cb6d4d4b41f8bbdc8f945b4e76fddc5362e74eb30b51c2785daf825'}, 'core/product_physics/src/lib.rs': {'solve_load_case_observed': '5a70782515a2e66790c8d570cc13d49ead9d22614e0f99b288d2632e8fc4e061', 'solve_prepared_load_case': '87e62f4554b7ef5cb1a270f6a81b6235770c77121134bf1d2f4ffe7cb63777d6'}}

PRODUCTION_PINS = {'core/product_physics/src/certified_publication.rs': '965647cc57fbbc336dc380d0f942222c9c636bc62048d91a59dee740575bd9a9', 'core/solver/frame_kernel/src/structural/certified_recovery.rs': '473277410897bbf21565d1a5e0134ea0be0f4b61c2c9a69d99b2e342f820c62c', 'core/solver/frame_kernel/src/exact_sum.rs': '35ae1c71195a95d88669c658965198e7dc2476cf32d948e4bfc62ef184e1051c'}

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
        if hashlib.sha256(source.encode()).hexdigest() != digest:
            raise ValueError(f"KD5 source/slot inventory pin changed: {file}; re-review owned layouts and caller/workspace unions")
    for file, functions in PINS.items():
        with open(os.path.join(snapshot, file), encoding="utf-8") as stream:
            source = stream.read()
        for name, digest in functions.items():
            body = function_body(source, name)
            if hashlib.sha256(body.encode()).hexdigest() != digest:
                raise ValueError(f"KD5T2: source pin changed: {file}:{name}; re-review the producer/consumer inventory")
            bodies[name] = body
    # B5K1: the pinned wrapper prepares pressure; the extracted mechanics
    # body still owns every route/maximum consumer literal.
    if "solve_prepared_load_case" in bodies:
        bodies["solve_load_case_observed"] = bodies["solve_prepared_load_case"]
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
STRESS_SUBJECT_SOURCE_PIN = "7df37b7604d7f201a265a2d4d6d0772c8e31b757ff1130d17ce8a17df70b87ff"
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

# B7a E14: this exact full-file source deletes the DEC-050/053 observation
# lane and its sole guard caller. No successor replaces its Iterator<usize>::max.
B7A_PRIMITIVE_LIB_SOURCE_PIN = "d1a3675f3c59630b69bd35de5896f1877264eb484eb980466ca057de315396db"


def primitive_lib_minmax_callers(snapshot):
    with open(os.path.join(snapshot, "core/product_physics/src/lib.rs"), "rb") as stream:
        raw = stream.read()
    if hashlib.sha256(raw).hexdigest() != B7A_PRIMITIVE_LIB_SOURCE_PIN:
        raise ValueError("KD5C4: B7a E14 full-file source changed; re-audit retired lane and current primitive owners")
    source = raw.decode("utf-8")
    if re.search(r"^\s*(?:pub(?:\([a-z]+\))?\s+)?(?:const\s+)?fn\s+observation_lane_profile\b", source, re.M):
        raise ValueError("KD5C4: the retired observation lane definition returned")
    return ["build_curved_bend_macro_elements", "straight_summary_extrema"]


# E13 uses the same exact current lib.rs binding as C4; remove one
# complete decision preimage, never permit an arbitrary replaced key.
def retire_combination_reaction_audit(snapshot, audits):
    primitive_lib_minmax_callers(snapshot)
    retired = ("core/product_physics/src/lib.rs:13085",
               {"row.id": {"bytes": 1024, "source": "RES"}})
    if audits.count(retired) != 1:
        raise ValueError("B7a E13: exact retired reaction audit preimage changed")
    current = list(audits)
    current.remove(retired)
    return current


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
    ("pressure_exact.rs", "eigenload_pair", ("mul",), "ce624c341c68c90e445856b617fe49fbcb81c7e5591c95e8e174f1d9a94413e9"),
    # B7a E14/C5: row slots and arc metadata changed; the sole divided
    # receiver remains frame_kernel::CertifiedRow, not FallbackError.
    ("preview_physics.rs", "render", ("divided",), "1a0bdf5476a3d20f0fcf6a46a992725b6c49205a859b2b89fbb303a22e06da5f"),
    ("retained_memory.rs", "strings", ("add",), "b34bc5c8a6f6863d21fd93b87baae4f33e47467d1f639086650fe6a22bd1a585"),
    # The N06 capture adds pending error capacity positively. Every add
    # receiver remains this module's Bytes accumulator, not FallbackError.
    ("retained_memory.rs", "retained_error_text", ("add",), "9f62063575e9e1eb566d12bb66b0601b4a9f39fd37c552cb0ec721168082f3e9"),
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
# KD5C5b (T4 batch 4, U2 H-2): the realized-arc pressure callers. Their
# receivers are FK's CertifiedRow (`wall_row.divided`) and Scaled
# (`fluid.mul`, `Scaled::from_f64(..).mul`), never the private FallbackError.
# Only a snapshot whose bend_pressure.rs defines the arc pressure rows has them.
FALLBACK_OUTSIDE_CALLERS_U2 = (
    ("bend_pressure.rs", "append_arc_pressure_results", ("divided",), "bbd3035a888811f15632e2c6bdaf72ba3ec2fcacecd33c1d07920a44f36cb280"),
    ("pressure_exact/source_geometry.rs", "arc_pressure_strain", ("mul",), "db9c1caeb47d37a3864bfb926a41cc37c6a6a713ae1c7c99b3d823fdb30ffd4a"),
)
# B5C1: the new effective-area expression's receiver is Scaled. Only
# subtract the unrelated private FallbackError::mul candidate, keeping
# every actual Scaled target and all original conservative candidates.
FALLBACK_OUTSIDE_CALLERS_B5 = (
    ("pressure_runtime.rs", "finish_source_groups", ("mul",), "c1a88878b2f10038e0b1f446f0c0fbf02b34bd1fba84cef3cd2770e98ee624fa"),
)

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
    # B7a E11 retired the unused interior Lamé caller; its product surface
    # stress path is already the separately pinned source_geometry caller.
    # 16 surviving K-D5 pairs and the join's J7 census caller.
    callers, expected = FALLBACK_OUTSIDE_CALLERS, 17
    bend_pressure = os.path.join(snapshot, "core/product_physics/src/bend_pressure.rs")
    if os.path.exists(bend_pressure):
        with open(bend_pressure, encoding="utf-8") as stream:
            if re.search(r"^pub\(super\) fn append_arc_pressure_results\(", stream.read(), re.M):
                callers, expected = callers + FALLBACK_OUTSIDE_CALLERS_U2, expected + len(FALLBACK_OUTSIDE_CALLERS_U2)
    pressure_source = os.path.join(snapshot, "core/product_physics/src/pressure_runtime.rs")
    if os.path.exists(pressure_source):
        with open(pressure_source, encoding="utf-8") as stream:
            if re.search(r"^fn assemble_pressure_case\b", stream.read(), re.M):
                callers, expected = callers + FALLBACK_OUTSIDE_CALLERS_B5, expected + len(FALLBACK_OUTSIDE_CALLERS_B5)
    for relative, name, methods, digest in callers:
        caller_file = "core/product_physics/src/" + relative
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
