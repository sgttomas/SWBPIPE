"""Apply one mutation at a time in place, run case_state tests, restore bytes."""
import hashlib, pathlib, subprocess, sys, json
root = pathlib.Path(sys.argv[1])  # core/product_physics
env = dict(**__import__("os").environ, CARGO_TARGET_DIR=sys.argv[2])
mutants = {
  "SF1_integral_last_segment_only": ("src/case_state/thermal.rs",
     "        terms.extend(product(width, average)?.map(|x| sign * x));",
     "        terms = product(width, average)?.map(|x| sign * x).to_vec();"),
  "SF1_dilation_last_segment_only": ("src/case_state/thermal.rs",
     "        terms.extend(product(delta, ratio)?.map(|v| sign * v));",
     "        terms = product(delta, ratio)?.map(|v| sign * v).to_vec();"),
  "SF2_restore_secant_datum_coverage": ("src/case_state/thermal.rs",
     "                CoefficientData::Table(points) => {\n                    validate_table(points)?;",
     "                CoefficientData::Table(points) => {\n                    validate_table(points)?;\n                    covered(points, datum)?;"),
  "SF3_disable_identity_canonicalization": ("src/case_state/temperature.rs",
     "            .map(|(_, binary)| *binary)",
     "            .map(|(_, binary)| *binary + if quantity.unit == \"K\" { 0.0 } else { crate::case_state::temperature::affine_rounding_probe(quantity) })"),
}
results = {}
for name, (rel, old, new) in mutants.items():
    path = root / rel
    original = path.read_bytes()
    text = original.decode()
    assert text.count(old) == 1, name
    if name.startswith("SF3"):
        # Mutant: return the ordinary binary64 affine conversion of THIS
        # quantity instead of the class representative.
        new = "            .map(|_| crate::case_state::temperature::ordinary_binary64(quantity))"
        text = text.replace(old, new) + "\n#[allow(dead_code)]\npub(crate) fn ordinary_binary64(q: &Quantity) -> f64 {\n    let from = open_pipe_stress_units::unit_by_symbol(&q.unit, Dimension::Temperature).unwrap();\n    let to = open_pipe_stress_units::canonical_unit(Dimension::Temperature).unwrap();\n    open_pipe_stress_units::convert_for_dimension(q.value, Dimension::Temperature, from, to).unwrap()\n}\n"
    else:
        text = text.replace(old, new)
    path.write_text(text)
    try:
        run = subprocess.run(["cargo", "+1.97.1", "test", "--locked", "--offline", "-j", "2", "--lib", "case_state"],
                             cwd=root, env=env, capture_output=True, text=True)
        out = run.stdout + run.stderr
        failed = [l for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
        results[name] = {"exit": run.returncode, "killed": run.returncode != 0,
                         "failed_tests": failed, "compile_error": "error[" in out}
    finally:
        path.write_bytes(original)
        assert hashlib.sha256(path.read_bytes()).hexdigest() == hashlib.sha256(original).hexdigest()
print(json.dumps(results, indent=1))
