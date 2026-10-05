"""RV92 confirmation: mutants against the whole-file seam guard (RV88 U6a N-3).
Each mutant edits product text in the mutant lane, runs the guard tests, and restores."""
import json, os, subprocess, sys
WT = "WT"
P = f"{WT}/rv92/mut2/projects/chirality-piping"
S = f"{WT}/scratch/rv92_u6f"
env = dict(os.environ, TMPDIR=f"{S}/tmp", CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2", CARGO_TARGET_DIR=f"{WT}/targets/rv92/re_mut2")
TAURI, RUNNER, PP, DER = "apps/desktop/src-tauri/src/lib.rs", "core/runner/headless/src/lib.rs", "core/product_physics/src/lib.rs", "core/reporting/result_export/src/derivative.rs"
SIG = "fn solver_result_row_value(envelope: &Value, result_id: &str) -> Result<Option<(f64, String)>, &'static str> {\n"

def after(anchor, text):
    return lambda s: s.replace(anchor, anchor + text, 1) if anchor in s else (_ for _ in ()).throw(SystemExit(f"anchor missing: {anchor[:40]}"))
def append(text):
    return lambda s: s + text
def inside_tests(text):  # inside src-tauri's `mod tests {`
    return after("#[cfg(test)]\nmod tests {\n", text)

MUTANTS = [
    ("G1 seam named in src-tauri solver_result_row_value (RV88's site)", TAURI, after(SIG, "    let _rv92 = open_pipe_stress_result_export::semantic_contract::class_binding_refusal;\n"), "killed"),
    ("G2 aliased seam import at the end of runner/headless lib.rs", RUNNER, append("\nuse open_pipe_stress_result_export::semantic_contract::retained_standing_from as rv92_alias;\n"), "killed"),
    ("G3 seam named in a product fn at the end of product_physics lib.rs", PP, append("\nfn rv92_g3() { let _ = open_pipe_stress_result_export::semantic_contract::classification_summary_from; }\n"), "killed"),
    ("G4 a fourth class_disclosure mention in derivative.rs product text", DER, append("\n// rv92: class_disclosure\n"), "killed"),
    ("G5 raw string holding '#[cfg(test)] mod x {' then a seam, in a product fn", TAURI, after(SIG, "    let _rv92s = r#\"#[cfg(test)] mod x {\"#; let _rv92 = open_pipe_stress_result_export::semantic_contract::class_binding_refusal;\n"), "killed"),
    ("G6 a line comment holding '#[cfg(test)] {' then a seam on the next line", TAURI, after(SIG, "    // #[cfg(test)] {\n    let _rv92 = open_pipe_stress_result_export::semantic_contract::class_binding_refusal;\n"), "killed"),
    ("G7 a #[cfg(test)] statement, then a seam on the next statement", TAURI, after(SIG, "    #[cfg(test)]\n    let _rv92_t = 1u8;\n    let _rv92 = open_pipe_stress_result_export::semantic_contract::class_binding_refusal;\n"), "killed"),
    ("G8 a seam in a product doc comment of runner lib.rs", RUNNER, append("\n/// see retained_standing_from\npub fn rv92_doc() {}\n"), "killed"),
    ("C1 control: a seam inside src-tauri's mod tests", TAURI, inside_tests("    #[allow(dead_code)]\n    fn rv92_c1() { let _ = open_pipe_stress_result_export::semantic_contract::class_binding_refusal; }\n"), "passes"),
    ("C2 control: a #[cfg(test)]-gated use of a seam in runner lib.rs", RUNNER, append("\n#[cfg(test)]\nuse open_pipe_stress_result_export::semantic_contract::retained_standing_from as rv92_c2;\n"), "passes"),
    ("C3 control: a gated test fn holding braces in strings and chars, then product text", RUNNER, append("\n#[cfg(test)]\nfn rv92_c3() { let _a = \"}{\"; let _b = '}'; let _c = retained_standing_from; }\npub fn rv92_c3_after() {}\n"), "passes"),
]

def run():
    r = subprocess.run(["cargo", "test", "--locked", "--offline", "--test", "retained_precision_carriers", "--", "u6_doc_hidden_seams_have_no_product_callers", "u6_product_text_drops_only_test_gated_items"],
                       cwd=f"{P}/core/reporting/result_export", env=env, capture_output=True, text=True)
    out = r.stdout + r.stderr
    return r.returncode, [l for l in out.splitlines() if l.startswith("test ") and ("FAILED" in l or " ok" in l)], [l for l in out.splitlines() if "panicked" in l or "assertion" in l][:3]

results = []
code, tests, why = run()
results.append({"mutant": "baseline (unmutated lane)", "expected": "passes", "result": "passes" if code == 0 else "fails", "tests": tests})
for name, rel, fn, expected in MUTANTS:
    path = f"{P}/{rel}"
    original = open(path).read()
    try:
        open(path, "w").write(fn(original))
        code, tests, why = run()
    finally:
        open(path, "w").write(original)
    got = "killed" if code != 0 else "passes"
    results.append({"mutant": name, "file": rel, "expected": expected, "result": got, "as_expected": got == expected, "tests": tests, "why": why})
    print(name, "->", got, "(expected", expected + ")", flush=True)
code, tests, why = run()
results.append({"mutant": "baseline after restore", "expected": "passes", "result": "passes" if code == 0 else "fails", "tests": tests})
json.dump(results, open(f"{S}/r2/guard_mutants.json", "w"), indent=1)
print(json.dumps([(r["mutant"], r["result"]) for r in results], indent=0))
