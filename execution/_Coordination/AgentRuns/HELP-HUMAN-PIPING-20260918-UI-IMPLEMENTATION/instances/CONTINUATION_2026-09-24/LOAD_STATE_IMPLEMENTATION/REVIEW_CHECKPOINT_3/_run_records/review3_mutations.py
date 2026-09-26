"""CP3 independent review: reviewer-authored mutants of the retained-source join.

Applied one at a time to a scratch copy of core/product_physics (never the
snapshot). Each mutant is run against the committed suites (lib without the
reviewer probes, load_reference_state_runtime, load_reference_state_runtime_extension,
physics_source_runtime) and then against the reviewer probes alone. Bytes are
restored and sha256-verified after every mutant.
usage: python3 review3_mutations.py <scratch core/product_physics> <target dir>
"""
import hashlib, json, os, pathlib, subprocess, sys

root = pathlib.Path(sys.argv[1])
env = dict(os.environ, CARGO_TARGET_DIR=sys.argv[2])
R, L, C, S = "src/source_recovery.rs", "src/lib.rs", "src/source_receipt/composite.rs", "src/source_receipt.rs"
M = {
    "K1_replay_builds_with_base_materials": [(S,
        "            build_model_for_members(&model, &materials, Some(&resolved.pairs), &mut diagnostics)\n                .ok_or_else(|| bad(\"captured member-pair build unavailable\"))?;",
        "            build_model_for_members(&model, &materials, None, &mut diagnostics)\n                .ok_or_else(|| bad(\"captured member-pair build unavailable\"))?;")],
    "K2_replay_prescribed_zero": [(S,
        "            .map(|&d| (d, resolved.prescribed.get(&d).copied().unwrap_or(0.0)))",
        "            .map(|&d| (d, 0.0))")],
    "K3_replay_omits_eigen_loads": [(S,
        "        add_thermal_equivalent_loads(&mut force, &eigen, &built.pipes, &HashMap::new());",
        "        let eigen: Vec<ThermalElementLoad> = { let _ = eigen; Vec::new() };")],
    "K4_live_eigen_fold_sign_flipped": [(R,
        "            folded_force[i_base + axis] -= value;\n            folded_force[j_base + axis] += value;",
        "            folded_force[i_base + axis] += value;\n            folded_force[j_base + axis] -= value;")],
    "K5_identity_omits_resolver_evidence": [(R,
        "    identity.name(&evidence, budget)?;\n    Ok(eigen_axial)",
        "    let _ = evidence;\n    Ok(eigen_axial)")],
    "K6_pressure_region_check_removed": [(R,
        "            .is_some_and(Vec::is_empty)\n        {\n            return Err(unsupported(\n                \"exact source recovery requires explicitly empty pressure regions\",",
        "            .is_some_and(|_| true)\n        {\n            return Err(unsupported(\n                \"exact source recovery requires explicitly empty pressure regions\",")],
    "K7_prescribed_ownership_loop_removed": [(R,
        "    if let Some(state) = load_state {\n        for (&dof, &value) in &state.prescribed {",
        "    if let Some(state) = load_state.filter(|_| false) {\n        for (&dof, &value) in &state.prescribed {")],
    "K8_eigen_id_collision_check_removed": [(R,
        "        if authored_loads.contains_key(source.as_str()) {",
        "        if false && authored_loads.contains_key(source.as_str()) {")],
    "K9_ordinary_case_record_not_bound_in_joined_envelope": [(C,
        "        if load_record != load_records.map(|records| &records[index]) {",
        "        if false && load_record != load_records.map(|records| &records[index]) {")],
    "K10_identity_omits_member_strain_split": [(R,
        "            strain.thermal_strain,\n            strain.thermal_stretch,\n            strain.fit_strain,\n            strain.fit_stretch,\n            strain.total_eigenstrain,\n        ]);",
        "            strain.total_eigenstrain,\n        ]);")],
    "K11_identity_omits_eigen_local_axis": [(R,
        "        identity.scalars([load.axial_load]);\n        identity.scalars(local_x);",
        "        identity.scalars([load.axial_load]);")],
    "K12_frame_area_check_removed": [(R,
        "            || !bits(frame.section.area, section.area)\n            || !bits(section.area, geometry.wall_area_m2())",
        "")],
}
committed = ["test", "--locked", "--offline", "-j", "1", "--lib", "--test", "load_reference_state_runtime",
             "--test", "load_reference_state_runtime_extension", "--test", "physics_source_runtime", "--", "--skip", "review3_"]
probes = ["test", "--locked", "--offline", "-j", "1", "--lib", "--", "review3_"]
results = {}
only = os.environ.get("ONLY")
for name, edits in M.items():
    if only and name not in only.split(","):
        continue
    originals = {}
    try:
        for rel, old, new in edits:
            path = root / rel
            originals.setdefault(path, path.read_bytes())
            text = path.read_text()
            assert text.count(old) == 1, (name, rel, text.count(old))
            path.write_text(text.replace(old, new))
        entry = {}
        for label, args in (("committed", committed), ("probes", probes)):
            run = subprocess.run(["cargo", "+1.97.1", *args], cwd=root, env=env, capture_output=True, text=True)
            out = run.stdout + run.stderr
            entry[label] = {
                "exit": run.returncode,
                "compile_error": "error[" in out or "could not compile" in out,
                "failed_tests": [l for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")],
            }
        entry["killed_by_committed"] = entry["committed"]["exit"] != 0 and not entry["committed"]["compile_error"]
        entry["killed_by_probes"] = entry["probes"]["exit"] != 0 and not entry["probes"]["compile_error"]
        results[name] = entry
        print(name, "committed_killed=", entry["killed_by_committed"], len(entry["committed"]["failed_tests"]),
              "probes_killed=", entry["killed_by_probes"], len(entry["probes"]["failed_tests"]),
              "compile_error=", entry["committed"]["compile_error"], flush=True)
    finally:
        for path, original in originals.items():
            path.write_bytes(original)
            assert hashlib.sha256(path.read_bytes()).hexdigest() == hashlib.sha256(original).hexdigest()
print(json.dumps(results, indent=1))
