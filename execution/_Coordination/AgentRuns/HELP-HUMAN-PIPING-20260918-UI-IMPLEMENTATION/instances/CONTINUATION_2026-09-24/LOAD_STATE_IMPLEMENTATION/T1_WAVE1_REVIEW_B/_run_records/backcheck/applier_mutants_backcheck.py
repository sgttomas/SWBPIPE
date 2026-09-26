"""REVIEW_B backcheck: R01/R02/R04 re-run plus F1/F2/F9 mutants on a git-archive scratch copy of the head."""
import subprocess, sys, os
from pathlib import Path
CRATE = Path(sys.argv[1])
env = dict(os.environ, CARGO_TARGET_DIR=sys.argv[2], CARGO_INCREMENTAL='0')
L, P, R = 'src/load_state_authoring.rs', 'src/pressure_authoring.rs', 'src/rich_authoring.rs'
MUTANTS = [
 ('R01 no-op still writes', L, "        writes: if after_display == current_display {\n            vec![]", "        writes: if false {\n            vec![]"),
 ('R02 point_ref resolved on any material', L, "            resolve_ref(&ids(material, \"temperature_points\"), \"point_ref\", point_ref)?;",
   "            let any: std::collections::HashSet<&str> = model[\"materials\"].as_array().into_iter().flatten().flat_map(|m| ids(m, \"temperature_points\")).collect();\n            resolve_ref(&any, \"point_ref\", point_ref)?;"),
 ('R04 support scan limited to first case', L, "    for case in cases(model) {\n        for (index, state) in case", "    for case in cases(model).take(1) {\n        for (index, state) in case"),
 ('B01 F1 version lock removed', P, "== Some(open_pipe_stress_product_physics::LOAD_STATE_MODEL_VERSION)\n", "== Some(\"never-a-version\")\n"),
 ('B02 F1 lock only for downgrades (no-op admitted)', P, "            if model.get(\"schema_version\").and_then(Value::as_str)\n                == Some(open_pipe_stress_product_physics::LOAD_STATE_MODEL_VERSION)\n",
   "            if model.get(\"schema_version\").and_then(Value::as_str)\n                == Some(open_pipe_stress_product_physics::LOAD_STATE_MODEL_VERSION) && !after.contains(\"0.4.0\")\n"),
 ('B03 F2 point orphan call removed', R, "            crate::load_state_authoring::refuse_point_orphans(model, current, &after)?;\n", ""),
 ('B04 F2 orphan check against the new list only (already-unresolved refused)', L, ".filter(|p| current.contains(p) && !next.contains(p))", ".filter(|p| { let _ = &current; !next.contains(p) })"),
 ('B05 F2 not scoped to the selecting material', L, "                != target\n            {\n                continue;\n            }\n            if let Some(point)", "                == Some(\"never-a-material\")\n            {\n                continue;\n            }\n            if let Some(point)"),
 ('B06 F2 orphan check only in the first case', L, "    for case in cases(model) {\n        for (index, element) in element_states(case) {\n            if element\n                .pointer(\"/material_selection/material_ref\")\n                .and_then(Value::as_str)\n                != target",
   "    for case in cases(model).take(1) {\n        for (index, element) in element_states(case) {\n            if element\n                .pointer(\"/material_selection/material_ref\")\n                .and_then(Value::as_str)\n                != target"),
 ('B07 F9 0.4.0 exact points still demand alpha', R, "    let point_thermal_field = if load_state && exact_profile {", "    let point_thermal_field = if false {"),
 ('B08 F9 0.4.0 not the exact profile', R, "(model[\"schema_version\"] == \"0.3.0\" || load_state)", "(model[\"schema_version\"] == \"0.3.0\")"),
 ('B09 F9 leaks to pre-0.4 (alpha never demanded)', R, "    let point_thermal_field = if load_state && exact_profile {", "    let point_thermal_field = if exact_profile {"),
]
for name, rel, old, new in MUTANTS:
    f = CRATE / rel; orig = f.read_text()
    if orig.count(old) != 1:
        print(f'{name}: NOT APPLIED (pattern count {orig.count(old)})', flush=True); continue
    f.write_text(orig.replace(old, new))
    try:
        p = subprocess.run(['cargo', '+1.97.1', 'test', '--locked', '--offline', '-j', '2', '--test', 'load_state_authoring',
                            '--test', 'load_state_delete_control', '--test', 'exact_authoring', '--test', 'contract_corpus'],
                           cwd=CRATE, env=env, capture_output=True, text=True, timeout=1800)
        failed = [l.split()[1] for l in p.stdout.splitlines() if l.startswith('test ') and l.endswith('FAILED')]
        err = 'compile error' if 'error[' in p.stderr else ''
        print(f"{name}: {'KILLED' if p.returncode else 'SURVIVED'} {err} {failed}", flush=True)
    finally:
        f.write_text(orig)
