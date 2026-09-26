#!/usr/bin/env python3
"""CP2 independent review mutation runner. Operates ONLY on a scratch copy of
core/ (argv[1] = scratch core/product_physics, argv[2] = scratch target dir).
Each mutant: exact single-occurrence replacement, run lib tests, the runtime
suite and the reviewer probes, restore bytes, verify sha256. `--dry` only
checks that every mutation site occurs exactly once."""
import hashlib
import json
import os
import pathlib
import re
import subprocess
import sys

root = pathlib.Path(sys.argv[1])
target = sys.argv[2]
dry = "--dry" in sys.argv
only = [a for a in sys.argv[3:] if not a.startswith("--")]
env = dict(os.environ, CARGO_TARGET_DIR=target)
L, R, T, TH, IN = "src/lib.rs", "src/case_state/resolve.rs", "src/case_state/temperature.rs", "src/case_state/thermal.rs", "src/case_state/input.rs"
PR = "src/pressure_runtime.rs"
M = {
    "M01_complete_u_omits_g": (L,
        "        for &(dof, value) in &prescribed {\n            displacements[dof] = value;\n        }\n",
        ""),
    "M02_observation_lane_uses_uncoupled_force": (L,
        "    let observation_force = if load_state.is_some() {",
        "    let observation_force = if false {"),
    "M03_legacy_reduced_system_zero_boundary": (L,
        "    let reduced = if load_state.is_some() {",
        "    let reduced = if false {"),
    "M04_eigenload_doubled_in_assembly_and_recovery": (L,
        "                    axial_load: member.material.pair.elastic_modulus_pa()\n                        * section.area\n                        * member.strain.total_eigenstrain,",
        "                    axial_load: 2.0 * member.material.pair.elastic_modulus_pa()\n                        * section.area\n                        * member.strain.total_eigenstrain,"),
    "M05_eigenload_not_removed_in_recovery": (L,
        "                &mechanical,\n                pipe_index,\n                &thermal_loads,",
        "                &mechanical,\n                pipe_index,\n                if load_state.is_some() { &[] } else { &thermal_loads },"),
    "M06_pressure_ignores_resolved_pairs": (L,
        "        load_case,\n        load_state.map(|state| &state.pairs),\n        diagnostics,",
        "        load_case,\n        None,\n        diagnostics,"),
    "M07_case_stiffness_reused_from_case_0": (L,
        "            let state_index = if case_index == 0 {",
        "            let state_index = if true {"),
    "M08_unfiltered_case_applied": (L,
        "    let load_case = load_state.map_or(load_case, |state| &state.effective_case);",
        "    let load_case = load_case;"),
    "M09_factor_ignored": (R,
        "        included.magnitude.value *= source.factor;",
        "        included.magnitude.value *= 1.0;"),
    "M10_G_not_derived_from_pair": (L,
        "                (pair.elastic_modulus_pa(), pair.shear_modulus_pa())",
        "                (pair.elastic_modulus_pa(), pair.elastic_modulus_pa() / 2.0)"),
    "M11_duplicate_source_admitted": (R,
        "        if !referenced.insert(source_ref) {",
        "        if false && !referenced.insert(source_ref) {"),
    "M12_not_joined_guard_removed": (L,
        "    if load_state.is_some() {\n        // New state inputs never inherit",
        "    if false {\n        // New state inputs never inherit"),
    "M13_prescribed_sign_flipped": (R,
        "            prescribed.insert(global, value);",
        "            prescribed.insert(global, -value);"),
    "M14_material_points_not_canonicalized": (R,
        "                    Ok(canonical) => point.temperature = Some(canonical),",
        "                    Ok(_canonical) => {}"),
    "M15_order_check_disabled": (T,
        "        if self.classes.windows(2).any(|pair| pair[0].1 >= pair[1].1) {",
        "        if false {"),
    "M16_secant_datum_coverage_restored": (TH,
        "    let secant = matches!(kind, PathKind::Secant);",
        "    let secant = false;"),
    "M17_dilation_datum_sample_counted_consumed": (TH,
        "            if sample(points, datum, &mut consulted)? != 0.0 {",
        "            if sample(points, datum, &mut used)? != 0.0 {"),
    "M18_legacy_thermal_guard_removed": (R,
        "            .any(|load| load.id == source_ref && load.category == \"thermal\")",
        "            .any(|load| false && load.id == source_ref && load.category == \"thermal\")"),
    "M19_evidence_pipe_materials_from_base": (L,
        "            evidence[\"pipe_materials\"] = serde_json::json!(state.members.iter()",
        "            if false { evidence[\"pipe_materials\"] = serde_json::json!(\"x\") }\n            let _unused = serde_json::json!(state.members.iter()"),
    "M20_fit_ignored_on_resolve": (R,
        "        let strain = match thermal::resolve_strain(&thermal_input, &fit_input) {",
        "        let strain = match thermal::resolve_strain(&thermal_input, &FitInput::None) {"),
    "M21_version_mismatch_guard_removed": (R,
        "        if carries {\n            block(diagnostics, \"LOAD_STATE_CONTRACT_VERSION_MISMATCH\"",
        "        if false && carries {\n            block(diagnostics, \"LOAD_STATE_CONTRACT_VERSION_MISMATCH\""),
    "M22_operating_temperature_not_canonicalized": (R,
        "            .map(|temperature| identity.canonical(temperature))",
        "            .map(|temperature| Ok::<_, String>(crate::Quantity { value: normalized(temperature, Dimension::Temperature).unwrap(), unit: \"K\".into() }))"),
    "M24_exact_endpoint_tolerance_snapping": ("src/case_state/material.rs",
        "                table.iter().find(|(t, _)| *t == requested)",
        "                table.iter().find(|(t, _)| (*t - requested).abs() <= 1e-8 * requested.abs())"),
    "M23_pressure_member_E_from_base_in_evidence_only": (PR,
        "            let material = if let Some(Some(&pair)) = resolved_pair {",
        "            let material = if let (false, Some(Some(&pair))) = (true, resolved_pair) {"),
}


def run_tests():
    cmd = ["cargo", "+1.97.1", "test", "--locked", "--offline", "-j", "1", "--lib",
           "--test", "load_reference_state_runtime", "--test", "review2_probes", "--test", "review2_probes_b", "--no-fail-fast", "--", "--test-threads", "2"]
    # Merge streams: cargo's "Running" headers are on stderr, test lines on stdout.
    run = subprocess.run(cmd, cwd=root, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    out = run.stdout
    groups, current = {}, None
    for line in out.splitlines():
        m = re.match(r"\s+Running (\S+)", line)
        if m:
            current = "lib" if "unittests" in m.group(1) else pathlib.Path(m.group(1)).stem
            groups.setdefault(current, [])
        m = re.match(r"test (\S+) \.\.\. FAILED", line)
        if m and current:
            groups[current].append(m.group(1))
    return run.returncode, "error[" in out or "could not compile" in out, groups, out


results = {}
for name, (rel, old, new) in M.items():
    if only and name not in only:
        continue
    path = root / rel
    original = path.read_bytes()
    text = original.decode()
    count = text.count(old)
    if dry:
        results[name] = {"sites": count}
        continue
    assert count == 1, (name, count)
    path.write_text(text.replace(old, new))
    try:
        code, compile_error, groups, out = run_tests()
        logdir = pathlib.Path(target).parent / "mutant_logs"
        logdir.mkdir(exist_ok=True)
        (logdir / f"{name}.log").write_text(out)
        failed = {k: v for k, v in groups.items() if v}
        results[name] = {
            "exit": code,
            "compile_error": compile_error,
            "killed_by_reviewed_tests": bool(failed.get("lib")) or bool(failed.get("load_reference_state_runtime")),
            "killed_by_review_probes_only": bool(failed.get("review2_probes") or failed.get("review2_probes_b")) and not (failed.get("lib") or failed.get("load_reference_state_runtime")),
            "failed": failed,
        }
        if compile_error:
            results[name]["compile_output_tail"] = out[-3000:]
    finally:
        path.write_bytes(original)
        assert hashlib.sha256(path.read_bytes()).hexdigest() == hashlib.sha256(original).hexdigest()
    print(name, json.dumps(results[name])[:600], flush=True)
print(json.dumps(results, indent=1))
