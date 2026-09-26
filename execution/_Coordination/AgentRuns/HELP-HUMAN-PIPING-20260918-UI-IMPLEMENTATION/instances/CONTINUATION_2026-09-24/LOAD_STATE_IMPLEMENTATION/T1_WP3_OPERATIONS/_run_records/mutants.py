"""Mutants removing each new refusal of the T1 WP3 load/reference-state operations.

Run from the operation_applier crate directory of a scratch copy made with
`git archive` plus the candidate files. Each mutant applies one textual edit,
runs `cargo test --test load_state_authoring`, records whether any test fails
(killed) and restores the file. Environment: CARGO_TARGET_DIR must be set.
"""
import json, os, subprocess, sys

M = "src/load_state_authoring.rs"
L = "src/lib.rs"
RA = "src/rich_authoring.rs"
PA = "src/pressure_authoring.rs"
MUTANTS = [
    ("M01-envelope", M, 'if unit != "none" || dimension != "dimensionless" {', 'if false {'),
    ("M02-schema-version", M, 'if model.get("schema_version").and_then(Value::as_str) != Some(LOAD_STATE_MODEL_VERSION) {', 'if false {'),
    ("M03-model-target", M, 'if model.pointer("/project/id").and_then(Value::as_str) != Some(target) {', 'if false {'),
    ("M04-before-value", M, 'if before != current_display {', 'if false {'),
    ("M05-explicit-null", M, 'if after_value.is_null() {', 'if false {'),
    ("M06-typed-parse-reference-configurations", M, '"Model" => validate_reference_configurations(model, after)?,', '"Model" => {}'),
    ("M07-typed-parse-expansion-laws", M, '"Material" => validate_expansion_laws(after)?,', '"Material" => {}'),
    ("M08-typed-parse-analysis-state", M, '_ => validate_analysis_state(model, owner, after, &after_value)?,', '_ => {}'),
    ("M09-typed-parse-from-value-not-text", M, '    serde_json::from_str(after).map_err(|e| {', '    serde_json::from_value(serde_json::from_str::<Value>(after).unwrap_or(Value::Null)).map_err(|e| {'),
    ("M10-unique-configuration-ids", M, '''    unique(
        configurations.iter().map(|c| c.id.as_str()),
        "reference configuration",
    )?;''', ''),
    ("M11-unique-law-ids", M, '    unique(laws.iter().map(law_id), "expansion law")', '    let _ = laws; Ok(())'),
    ("M12-member-pipe-ref", M, '            resolve_ref(&pipes, "pipe_ref", &member.pipe_ref)?;', ''),
    ("M13-reference-configuration-ref", M, '''    resolve_ref(
        &ids(model, "reference_configurations"),
        "reference_configuration_ref",
        &state.reference_configuration_ref,
    )?;''', ''),
    ("M14-element-pipe-ref", M, '        resolve_ref(&pipes, "pipe_ref", &element.pipe_ref)?;', ''),
    ("M15-material-ref", M, '''            .find(|m| m.get("id").and_then(Value::as_str) == Some(material_ref))
            .ok_or_else(|| {
                refuse(
                    "OP-LOAD-STATE-REFERENCE-UNRESOLVED",
                    format!("material_ref {material_ref} does not resolve in the current model"),
                )
            })?;''', '''            .find(|m| m.get("id").and_then(Value::as_str) == Some(material_ref))
            .unwrap_or(&model["materials"][0]);'''),
    ("M16-point-ref", M, '            resolve_ref(&ids(material, "temperature_points"), "point_ref", point_ref)?;', '            let _ = point_ref;'),
    ("M17-law-ref", M, '''            resolve_ref(
                &ids(material, "expansion_laws"),
                "expansion_law_ref",
                law_ref,
            )?;''', '            let _ = law_ref;'),
    ("M18-law-ref-any-material", M, '''                &ids(material, "expansion_laws"),''', '''                &model["materials"].as_array().unwrap().iter().flat_map(|m| ids(m, "expansion_laws")).collect(),'''),
    ("M19-support-ref", M, '        resolve_ref(&supports, "support_ref", &support.support_ref)?;', ''),
    ("M20-source-ref", M, '        resolve_ref(&primitives, "load_sources[].source_ref", &source.source_ref)?;', ''),
    ("M21-source-ref-any-case", M, '    let primitives = ids(case, "primitive_loads");', '    let _ = case; let primitives: HashSet<&str> = model["load_cases"].as_array().unwrap().iter().flat_map(|c| ids(c, "primitive_loads")).collect();'),
    ("M22-model-hash-required", L, '''    if load_state_authoring::owns(&object_type, &field_path)
        && claimed_model_hash.is_none_or(Value::is_null)
    {''', '''    if false {'''),
    ("M23-inverse-restores-absence", M, 'let (write, after_display) = if after == NOT_PRESENT {', 'let (write, after_display) = if false {'),
    ("M24-entity-target", M, '''    if found.next().is_some() {
        return Err(refuse(''', '''    if false {
        return Err(refuse('''),
    # Addendum 1: inbound-reference refusals.
    ("N01-orphan-reference-configuration-ref", M, '        "Model" => "reference_configurations",', '        "Model" => return Ok(()),'),
    ("N02-orphan-expansion-law-ref", M, '        "Material" => "expansion_laws",', '        "Material" => return Ok(()),'),
    ("N03-orphan-only-if-newly-unresolved", M, 'let lost = |id: &str| current.contains(id) && ', 'let lost = |id: &str| (current.contains(id) || true) && '),
    ("N04-orphan-law-scoped-to-selecting-material", M, """                != Some(target)
            {
                continue;
            }""", """                != Some(target)
            {
            }"""),
    ("N05-delete-pipe-load-state-references", L, '    references.extend(load_state_authoring::pipe_references(model, target_ref));', ''),
    ("N06-delete-pipe-configuration-members", M, '            if member.get("pipe_ref").and_then(Value::as_str) == Some(pipe_ref) {', '            if false {'),
    ("N07-delete-pipe-element-states", M, '            if element.get("pipe_ref").and_then(Value::as_str) == Some(pipe_ref) {', '            if false {'),
    ("N08-delete-support-support-states", L, '    references.extend(load_state_authoring::support_references(model, target_ref));', ''),
    ("N09-delete-primitive-load-sources", L, '    let references = load_state_authoring::source_references(load_case, primitive_id);', '    let references: Vec<String> = Vec::new();\n    let _ = (load_case, primitive_id);'),
    # Addendum 2: review B repairs. R01, R02 and R04 are review B's own mutants, verbatim.
    ("R01-no-op-still-writes", M, "        writes: if after_display == current_display {\n            vec![]", "        writes: if false {\n            vec![]"),
    ("R02-point-ref-resolved-on-any-material", M, "            resolve_ref(&ids(material, \"temperature_points\"), \"point_ref\", point_ref)?;",
     "            let any: std::collections::HashSet<&str> = model[\"materials\"].as_array().into_iter().flatten().flat_map(|m| ids(m, \"temperature_points\")).collect();\n            resolve_ref(&any, \"point_ref\", point_ref)?;"),
    ("R04-support-scan-first-case-only", M, "    for case in cases(model) {\n        for (index, state) in case", "    for case in cases(model).take(1) {\n        for (index, state) in case"),
    ("P01-pressure-profile-version-lock", PA, """            if model.get("schema_version").and_then(Value::as_str)
                == Some(open_pipe_stress_product_physics::LOAD_STATE_MODEL_VERSION)
            {""", "            if false {"),
    ("P02-temperature-points-orphan-check", RA, "            crate::load_state_authoring::refuse_point_orphans(model, current, &after)?;\n", ""),
    ("P03-point-orphan-scoped-to-selecting-material", M, """                != target
            {
                continue;
            }""", """                != target
            {
            }"""),
    ("P04-point-orphan-only-if-newly-unresolved", M, ".filter(|p| current.contains(p) && !next.contains(p))", ".filter(|p| { let _ = &current; !next.contains(p) })"),
    ("P05-exact-profile-includes-0-4-0", RA, 'let exact_profile = (model["schema_version"] == "0.3.0" || load_state)', 'let exact_profile = (model["schema_version"] == "0.3.0")'),
    ("P06-0-4-0-points-need-no-coefficient", RA, "    let point_thermal_field = if load_state && exact_profile {", "    let point_thermal_field = if false {"),
]

def run(target):
    env = dict(os.environ)
    p = subprocess.run(["cargo", "+1.97.1", "test", "--locked", "--offline", "-j", "2", "--test", "load_state_authoring"],
                       capture_output=True, text=True, env=env)
    out = p.stdout + p.stderr
    failed = [l.split()[1] for l in out.splitlines() if l.startswith("test ") and l.rstrip().endswith("FAILED")]
    compiled = "could not compile" not in out
    return p.returncode, failed, compiled, out

results = []
only = sys.argv[1:]
for mid, path, old, new in MUTANTS:
    if only and mid not in only:
        continue
    src = open(path).read()
    if src.count(old) != 1:
        results.append({"id": mid, "status": "PATTERN_NOT_UNIQUE", "count": src.count(old)})
        print(mid, "PATTERN_NOT_UNIQUE", src.count(old), flush=True)
        continue
    open(path, "w").write(src.replace(old, new))
    try:
        code, failed, compiled, out = run(path)
    finally:
        open(path, "w").write(src)
    status = "KILLED" if code != 0 and compiled and failed else ("COMPILE_ERROR" if not compiled else ("SURVIVED" if code == 0 else "ERROR"))
    if status == "COMPILE_ERROR":
        print(out[-3000:])
    results.append({"id": mid, "file": path, "status": status, "failing_tests": failed})
    print(mid, status, failed, flush=True)
json.dump(results, open(os.environ.get("MUTANT_OUT", "mutants_result.json"), "w"), indent=1)
