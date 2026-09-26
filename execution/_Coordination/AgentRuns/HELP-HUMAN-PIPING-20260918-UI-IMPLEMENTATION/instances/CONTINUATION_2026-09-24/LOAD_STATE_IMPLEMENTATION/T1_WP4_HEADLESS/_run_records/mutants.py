"""T1 WP4 mutation run on a scratch copy (git archive of the base plus the WP4
test files). Never run in place.

Usage: python mutants.py <scratch-copy-root> <private-target-dir> <log-file>

Each mutant is an exact, single-occurrence source replacement. After applying
it, the WP4 lanes are built and run (the library module
`load_reference_route_tests` and the CLI integration target
`load_reference_cli`); the mutant is killed when that run fails. The file is
restored before the next mutant. A mutant that does not compile is reported as
such, not as killed.
"""
from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(sys.argv[1]) / "projects/chirality-piping"
TARGET = sys.argv[2]
LOG = Path(sys.argv[3])
HEADLESS = ROOT / "core/runner/headless"
LIB = "core/runner/headless/src/lib.rs"
BINDING = "core/runner/headless/src/result_envelope_binding.rs"
CLI = "core/runner/headless/src/bin/openpipestress-runner.rs"
PRODUCT = "core/product_physics/src/lib.rs"
RESOLVE = "core/product_physics/src/case_state/resolve.rs"
INPUT = "core/product_physics/src/case_state/input.rs"
FLIP = ("if mode==PreviewSolverMode::SparseInteractive {PreviewSolverMode::DenseScrutiny} "
        "else {PreviewSolverMode::SparseInteractive}")

MUTANTS = [
    # Identity routing and the export gate.
    ("R1 binding builds documents for non-eligible standing", BINDING,
     'Some(&evidence.actual_invocation)) != "numerically_eligible" {',
     'Some(&evidence.actual_invocation)) == "__mutant_never__" {'),
    ("P1 0.4.0 formulation basis not routed to load-reference", PRODUCT,
     "if case_state::is_load_state(model) {\n        return load_state_formulation_basis();",
     "if false && case_state::is_load_state(model) {\n        return load_state_formulation_basis();"),
    ("P1b blocked 0.4.0 envelope leaves the exact namespace", PRODUCT,
     "let exact_namespace = pressure_runtime::is_exact(&model) || case_state::is_load_state(&model);",
     "let exact_namespace = pressure_runtime::is_exact(&model);"),
    ("P2 joined publication labelled load-reference-1", PRODUCT,
     "envelope.formulation_basis = joined_load_state_formulation_basis();\n            LOAD_REFERENCE_SOURCE_SEMANTIC_CONTRACT_ID",
     "envelope.formulation_basis = joined_load_state_formulation_basis();\n            LOAD_REFERENCE_SEMANTIC_CONTRACT_ID"),
    # Receipt and invocation binding at the runner seam.
    ("R2 runner solves in the other mode", LIB,
     "run_linear_static_preview_value_with_mode(solve_payload.clone(), mode)",
     f"run_linear_static_preview_value_with_mode(solve_payload.clone(), {FLIP})"),
    ("R3 runner forwards a payload other than the retained one", LIB,
     "run_linear_static_preview_value_with_mode(solve_payload.clone(), mode)",
     "run_linear_static_preview_value_with_mode({let mut p=solve_payload.clone(); p[\"mutant_extra\"]=serde_json::json!(true); p}, mode)"),
    ("R4 runner captures the wrong mode label", LIB,
     'let actual_invocation = serde_json::json!({"request": &solve_payload, "solver_mode": mode.as_str()});',
     f'let actual_invocation = serde_json::json!({{"request": &solve_payload, "solver_mode": ({FLIP}).as_str()}});'),
    ("R5 binding drops invocation-request equals payload", BINDING,
     '|| evidence.actual_invocation["request"] != evidence.solve_payload',
     "|| false"),
    ("R6 binding drops the closed solver-mode set", BINDING,
     '|| !matches!(evidence.actual_invocation["solver_mode"].as_str(), Some("sparse_interactive" | "dense_scrutiny"))',
     "|| false"),
    ("R6b binding drops the invocation digest check", BINDING,
     "|| digest(&evidence.actual_invocation)? != evidence.invocation_digest",
     "|| false"),
    # Refusals at the runner seam.
    ("R7 CLI typed input gate bypassed", CLI,
     "let _preview_request: LinearStaticPreviewRequest =\n        match serde_json::from_value(exact_solve_payload.clone()) {",
     "let _preview_request: Option<LinearStaticPreviewRequest> =\n        match serde_json::from_value(exact_solve_payload.clone()).map(Some).or_else(|_: serde_json::Error| Ok::<_, serde_json::Error>(None)) {"),
    ("R8 runner reports MECHANICS_SOLVED for blocked envelopes", LIB,
     'if mechanics.status.mechanics == "MECHANICS_SOLVED" {\n        analysis_status.push(AnalysisStatus::MechanicsSolved);',
     'if true {\n        analysis_status.push(AnalysisStatus::MechanicsSolved);'),
    ("R9 runner drops the SOURCE_NOT_SOLVED gate", LIB,
     'if mechanics.status.mechanics!="MECHANICS_SOLVED" {return Err("SOURCE_NOT_SOLVED".into());}',
     ""),
    ("R10 CLI exit ignores result validation", CLI,
     "&& !result_validation.has_blocking_diagnostics();",
     ";"),
    ("P3 explicit-null refusal not emitted", RESOLVE,
     "for path in &nulls {",
     "for path in nulls.iter().take(0) {"),
    ("P4 pre-0.4 version-mismatch refusal not emitted", RESOLVE,
     'if carries {\n            block(diagnostics, "LOAD_STATE_CONTRACT_VERSION_MISMATCH"',
     'if false && carries {\n            block(diagnostics, "LOAD_STATE_CONTRACT_VERSION_MISMATCH"'),
    ("P5 unknown analysis_state fields accepted", INPUT,
     "#[serde(deny_unknown_fields)]\npub struct AnalysisStateInput {",
     "pub struct AnalysisStateInput {"),
]

ENV = dict(os.environ, CARGO_TARGET_DIR=TARGET, CARGO_INCREMENTAL="0",
           CARGO_PROFILE_DEV_DEBUG="0", CARGO_PROFILE_TEST_DEBUG="0")
CARGO = ["cargo", "+1.97.1", "test", "--locked", "--offline", "-j", "2"]


def run(args):
    return subprocess.run(CARGO + args, cwd=HEADLESS, env=ENV, capture_output=True, text=True)


def lanes():
    """0 = pass, 1 = test failure, 2 = compile failure."""
    build = run(["--no-run"])
    if build.returncode != 0:
        return 2, build.stderr[-3000:]
    outputs = []
    for args in (["--lib", "load_reference_route_tests"], ["--test", "load_reference_cli"]):
        result = run(args)
        outputs.append(result.stdout[-4000:])
        if result.returncode != 0:
            return 1, "\n".join(outputs)
    return 0, "\n".join(outputs)


def log(line):
    with LOG.open("a") as handle:
        handle.write(line + "\n")
    print(line, flush=True)


def main():
    code, output = lanes()
    log(f"BASELINE (unmutated scratch copy): {'pass' if code == 0 else 'FAIL'}")
    if code != 0:
        log(output)
        return 1
    for label, rel, find, replace in MUTANTS:
        path = ROOT / rel
        original = path.read_text()
        count = original.count(find)
        if count != 1:
            log(f"{label}: NOT APPLIED (pattern count {count})")
            continue
        path.write_text(original.replace(find, replace, 1))
        try:
            code, output = lanes()
        finally:
            path.write_text(original)
        verdict = {0: "SURVIVED", 1: "KILLED", 2: "DOES NOT COMPILE"}[code]
        failing = [line for line in output.splitlines() if line.endswith("FAILED") and line.startswith("test ")]
        log(f"{label} [{rel}]: {verdict}" + (f" by {failing}" if failing else ""))
        if code == 2:
            log(output)
    return 0


if __name__ == "__main__":
    sys.exit(main())
