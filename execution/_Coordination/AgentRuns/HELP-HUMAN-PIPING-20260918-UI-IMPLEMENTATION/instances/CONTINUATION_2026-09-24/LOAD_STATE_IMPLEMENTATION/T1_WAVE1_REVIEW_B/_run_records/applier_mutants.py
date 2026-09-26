"""REVIEW_B independent operation_applier mutants (scratch clone at the candidate commit)."""
import subprocess, sys, os
from pathlib import Path
CRATE = Path(sys.argv[1])
TARGET = sys.argv[2]
F = CRATE / 'src/load_state_authoring.rs'
MUTANTS = {
 'R01 no-op still writes': ("        writes: if after_display == current_display {\n            vec![]", "        writes: if false {\n            vec![]"),
 'R02 point_ref resolved on any material': ("            resolve_ref(&ids(material, \"temperature_points\"), \"point_ref\", point_ref)?;",
     "            let any: std::collections::HashSet<&str> = model[\"materials\"].as_array().into_iter().flatten().flat_map(|m| ids(m, \"temperature_points\")).collect();\n            resolve_ref(&any, \"point_ref\", point_ref)?;"),
 'R03 orphan check skipped for not_present removals': ("    refuse_orphans(model, object_type, target, owner, write.as_ref())?;", "    if write.is_some() { refuse_orphans(model, object_type, target, owner, write.as_ref())?; }"),
 'R04 support scan limited to first case': ("    for case in cases(model) {\n        for (index, state) in case", "    for case in cases(model).take(1) {\n        for (index, state) in case"),
 'R05 element pipe scan limited to first case': ("    for case in cases(model) {\n        for (index, element) in element_states(case) {\n            if element.get(\"pipe_ref\")", "    for case in cases(model).take(1) {\n        for (index, element) in element_states(case) {\n            if element.get(\"pipe_ref\")"),
 'R06 configuration orphan check only first case': ("    for case in cases(model) {\n        if object_type == \"Model\" {", "    for case in cases(model).take(1) {\n        if object_type == \"Model\" {"),
}
orig = F.read_text()
env = dict(os.environ, CARGO_TARGET_DIR=TARGET)
for name, (old, new) in MUTANTS.items():
    assert orig.count(old) == 1, (name, orig.count(old))
    F.write_text(orig.replace(old, new))
    try:
        p = subprocess.run(['cargo', '+1.97.1', 'test', '--locked', '--offline', '-j', '2', '--test', 'load_state_authoring', '--test', 'load_state_delete_control'],
                           cwd=CRATE, env=env, capture_output=True, text=True, timeout=1800)
        lines = [l for l in p.stdout.splitlines() if l.startswith('test result') or ' FAILED' in l or l.startswith('error')]
        print(f"{name}: {'KILLED' if p.returncode else 'SURVIVED'} {lines[-3:] if lines else p.stderr[-300:]}", flush=True)
    finally:
        F.write_text(orig)
