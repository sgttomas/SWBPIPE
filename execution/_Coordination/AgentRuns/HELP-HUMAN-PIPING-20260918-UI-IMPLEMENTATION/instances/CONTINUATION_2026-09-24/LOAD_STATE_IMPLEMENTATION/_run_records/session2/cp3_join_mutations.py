"""CP3: apply one mutant at a time in place (possibly several edits), run the
case_state and load_state_tests suites, restore bytes and verify sha256."""
import hashlib, pathlib, subprocess, sys, json, os
root = pathlib.Path(sys.argv[1])  # core/product_physics
env = dict(os.environ, CARGO_TARGET_DIR=sys.argv[2])
M = {
 "N1_M02_observation_uncoupled_force": [("src/lib.rs", "    let observation_force = if load_state.is_some() {", "    let observation_force = if false {")],
 "N1_M03_zero_boundary_reduced": [("src/lib.rs", "    let reduced = if load_state.is_some() {", "    let reduced = if false {")],
 "J1_member_end_offset_omitted": [("src/source_recovery.rs", "                0 if eigen != 0.0 => vec![vec![eigen]],\n                6 if eigen != 0.0 => vec![vec![-eigen]],", "                0 if false => vec![vec![eigen]],\n                6 if false => vec![vec![-eigen]],")],
 "J2_member_end_offset_sign_flipped": [("src/source_recovery.rs", "                0 if eigen != 0.0 => vec![vec![eigen]],\n                6 if eigen != 0.0 => vec![vec![-eigen]],", "                0 if eigen != 0.0 => vec![vec![-eigen]],\n                6 if eigen != 0.0 => vec![vec![eigen]],")],
 "J3_eigen_force_terms_unidentified": [("src/source_recovery.rs", "            if value != 0.0 {\n                force_terms.push(", "            if false {\n                force_terms.push(")],
 "J4_prescribed_owner_zero": [("src/source_recovery.rs", "                    state.prescribed.get(&global).copied().unwrap_or(0.0)", "                    0.0")],
 "J5_captured_replay_skipped": [("src/source_receipt.rs", "        let replay = self.captured_load_state_case(&input.load_case.id)?;\n        let LoadStateReplay {", "        if input.load_state.is_some() { return Ok(None); }\n        let replay = self.captured_load_state_case(&input.load_case.id)?;\n        let LoadStateReplay {")],
 "J6_case_record_binding_skipped": [("src/source_receipt/composite.rs", "        if record != Some(&expected) {", "        if false && record != Some(&expected) {")],
 "J7_eigen_closure_skipped": [("src/source_recovery.rs", "        return Err(mismatch(\n            \"actual eigen element loads differ from the resolved case\",\n        ));", "        let _ = ();")],
 "J8_member_pair_formation_unchecked": [("src/source_recovery.rs", "            return Err(mismatch(\n                \"resolved member pair/section did not reach formation\",\n            ));", "            let _ = ();")],
 "J9_ownership_unchecked": [("src/source_recovery.rs", "        (Some(state), true) => Some(state),\n        _ => return Err(mismatch(\"resolved load/reference-state ownership\")),", "        (Some(state), _) => Some(state),\n        _ => None,")],
 "J10_joined_semantics_not_selected": [("src/lib.rs", "        envelope.producer.semantic_contract_id = if joined_load_state {", "        envelope.producer.semantic_contract_id = if false {")],
 "SFA_fit_none_unit_variant": [("src/case_state/input.rs", "    NoFit {},", "    NoFit,"), ("src/case_state/resolve.rs", "FitReferenceInput::NoFit {} =>", "FitReferenceInput::NoFit =>")],
 "SFA_quantity_open": [("src/case_state/input.rs", "#[derive(Deserialize)]\n#[serde(deny_unknown_fields)]\nstruct ClosedQuantity {", "#[derive(Deserialize)]\nstruct ClosedQuantity {")],
 "N2_null_as_absent": [("src/case_state/input.rs", "            None => Self::Null,", "            None => Self::Absent,")],
 "N2_request_laws_ignored": [("src/lib.rs", "            if record.expansion_laws.is_authored() {\n                model.request_material_expansion_laws.push(index);", "            if false {\n                model.request_material_expansion_laws.push(index);")],
 "N5_missing_section_dropped": [("src/lib.rs", "            let section = built.sections.get(&member.pipe_id).ok_or_else(|| member.pipe_id.clone())?;", "            let Some(section) = built.sections.get(&member.pipe_id) else { return Ok(ThermalElementLoad { element_index: member.pipe_index, axial_load: 0.0, thermal_strain: 0.0 }); };")],
}
results = {}
only = os.environ.get('ONLY')
for name, edits in M.items():
    if only and name not in only.split(','):
        continue
    originals = {}
    try:
        for rel, old, new in edits:
            path = root / rel
            originals.setdefault(path, path.read_bytes())
            text = path.read_text()
            assert text.count(old) == 1, (name, rel, text.count(old))
            path.write_text(text.replace(old, new))
        run = subprocess.run(["cargo", "+1.97.1", "test", "--locked", "--offline", "-j", "2", "--lib", "--", "case_state::", "load_state_tests"],
                             cwd=root, env=env, capture_output=True, text=True)
        out = run.stdout + run.stderr
        failed = [l for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
        results[name] = {"exit": run.returncode, "killed": run.returncode != 0, "failed_tests": failed, "compile_error": "error[" in out}
        print(name, results[name]["killed"], len(failed), results[name]["compile_error"], flush=True)
    finally:
        for path, original in originals.items():
            path.write_bytes(original)
            assert hashlib.sha256(path.read_bytes()).hexdigest() == hashlib.sha256(original).hexdigest()
print(json.dumps(results, indent=1))
