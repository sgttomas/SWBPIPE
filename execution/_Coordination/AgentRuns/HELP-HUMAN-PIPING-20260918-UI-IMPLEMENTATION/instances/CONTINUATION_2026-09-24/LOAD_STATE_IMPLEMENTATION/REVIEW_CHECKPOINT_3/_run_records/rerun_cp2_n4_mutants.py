"""CP3 review: demonstrate the EXTENSION_1 kill targets (review N-4). The CP2
reviewer's mutants (anchors verbatim from REVIEW_CHECKPOINT_2/_run_records/
review2_mutations.py, M04/M19 re-anchored to the CP3 refactor) are applied one at
a time to a scratch copy of the FROZEN candidate and run against the extension
binary alone, then against the whole committed set. Bytes restored and
sha256-verified. usage: python3 rerun_cp2_n4_mutants.py <review2 script> <scratch core/product_physics> <target>"""
import hashlib, json, os, pathlib, subprocess, sys
src = open(sys.argv[1]).read()
ns = {}
exec(src[src.index('L, R, T, TH, IN'):src.index('def run_tests')], ns)
M = {k: ns['M'][k] for k in ["M06_pressure_ignores_resolved_pairs", "M14_material_points_not_canonicalized",
     "M15_order_check_disabled", "M16_secant_datum_coverage_restored", "M17_dilation_datum_sample_counted_consumed",
     "M22_operating_temperature_not_canonicalized", "M23_pressure_member_E_from_base_in_evidence_only",
     "M24_exact_endpoint_tolerance_snapping"]}
M["M04_eigenload_doubled_reanchored"] = ("src/lib.rs",
    "                axial_load: member.material.pair.elastic_modulus_pa()\n                    * section.area\n                    * member.strain.total_eigenstrain,",
    "                axial_load: 2.0 * member.material.pair.elastic_modulus_pa()\n                    * section.area\n                    * member.strain.total_eigenstrain,")
M["M19_evidence_pipe_materials_from_base_reanchored"] = ("src/lib.rs",
    "            evidence[\"pipe_materials\"] = load_state_pipe_materials(state);",
    "            let _ = load_state_pipe_materials(state);")
root = pathlib.Path(sys.argv[2]); env = dict(os.environ, CARGO_TARGET_DIR=sys.argv[3])
base = ["cargo", "+1.97.1", "test", "--locked", "--offline", "-j", "1"]
suites = {"extension_only": base + ["--test", "load_reference_state_runtime_extension"],
          "committed": base + ["--lib", "--test", "load_reference_state_runtime", "--test", "load_reference_state_runtime_extension", "--", "--skip", "review3_"]}
results = {}
for name, (rel, old, new) in M.items():
    path = root / rel
    original = path.read_bytes()
    text = original.decode()
    assert text.count(old) == 1, (name, text.count(old))
    path.write_text(text.replace(old, new))
    try:
        entry = {}
        for label, args in suites.items():
            run = subprocess.run(args, cwd=root, env=env, capture_output=True, text=True)
            out = run.stdout + run.stderr
            ce = "error[" in out or "could not compile" in out
            entry[label] = {"exit": run.returncode, "compile_error": ce, "killed": run.returncode != 0 and not ce,
                            "failed": [l.split(' ')[1] for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")]}
        results[name] = entry
        print(name, "extension_killed=", entry["extension_only"]["killed"], entry["extension_only"]["failed"], "committed_killed=", entry["committed"]["killed"], len(entry["committed"]["failed"]), flush=True)
    finally:
        path.write_bytes(original)
        assert hashlib.sha256(path.read_bytes()).hexdigest() == hashlib.sha256(original).hexdigest()
print(json.dumps(results, indent=1))
