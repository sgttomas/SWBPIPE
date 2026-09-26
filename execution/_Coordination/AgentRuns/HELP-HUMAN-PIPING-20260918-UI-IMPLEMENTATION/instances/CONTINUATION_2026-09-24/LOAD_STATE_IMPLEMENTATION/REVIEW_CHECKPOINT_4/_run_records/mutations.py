"""CP4 reviewer mutants, run on a git-archive scratch copy of the frozen
candidate (never the checkout). Each mutant is applied, the named suites run,
and the original bytes are restored and sha256-verified.
usage: python3 r4_mutations.py <reviewer review3_mutations.py> <scratch core/product_physics> <target>
"""
import ast, hashlib, json, os, pathlib, subprocess, sys

review3 = pathlib.Path(sys.argv[1]).read_text()
root = pathlib.Path(sys.argv[2])
env = dict(os.environ, CARGO_TARGET_DIR=sys.argv[3])
R, L, C, S = "src/source_recovery.rs", "src/lib.rs", "src/source_receipt/composite.rs", "src/source_receipt.rs"
K = None
for node in ast.parse(review3).body:
    if isinstance(node, ast.Assign) and any(getattr(t, "id", None) == "M" for t in node.targets):
        K = eval(compile(ast.Expression(node.value), "review3", "eval"), {}, {"R": R, "L": L, "C": C, "S": S})
assert K and len(K) == 12

base = ["test", "--locked", "--offline", "-j", "2"]
JOIN = base + ["--lib", "--", "source_receipt::load_state_join_tests"]
FALLBACK = base + ["--lib", "--", "source_receipt::load_state_fallback_tests"]
LIB = base + ["--lib", "--", "--skip", "review4_probes"]
ALL = base + ["--", "--skip", "review4_probes"]

M = {
    # K8/K9 verbatim from the CP3 reviewer's table.
    "K8": (K["K8_eigen_id_collision_check_removed"], [("join", JOIN), ("all", ALL)]),
    "K9": (K["K9_ordinary_case_record_not_bound_in_joined_envelope"], [("join", JOIN), ("all", ALL)]),
    # The manager's six SF-1 mutants, re-anchored to the frozen (rustfmt) bytes.
    "SF1-M1 reservation removed": ([(L, "Some(_) => recovery.reserve_captured_replay(case_limit),", "Some(_) => Ok(recovery),")], [("fallback", FALLBACK)]),
    "SF1-M2 fallback republication removed": ([(L, "(Some(request), Some(cause)) => {", "(Some(request), Some(cause)) if false => {")], [("fallback", FALLBACK)]),
    "SF1-M3 withheld attempt still selects": ([(L, "Some(_) if source_budget.load_state_join_withheld.is_some() => {\n                Err(recovery.decline_withheld())", "Some(_) if false => {\n                Err(recovery.decline_withheld())")], [("fallback", FALLBACK)]),
    "SF1-M4 per-case failure not recorded": ([(L, 'source_budget.record_load_state_join_failure(format!(\n                            "case {}: {}",\n                            load_case.id, error.0\n                        ));', "")], [("fallback", FALLBACK)]),
    "SF1-M5 invocation failure not recorded": ([(L, 'source_budget.record_load_state_join_failure(format!(\n                            "invocation receipt: {}",\n                            error.0\n                        ));', "")], [("fallback", FALLBACK)]),
    "SF1-M6 reservation c<=L": ([(R, "if charged <= limit.saturating_sub(charged) {", "if charged <= limit {")], [("fallback", FALLBACK)]),
    # Reviewer additions.
    "R1 pre-0.4 guards removed (wrapper + invocation record)": ([
        (L, "&& case_state::is_load_state(&request.model)\n", "\n"),
        (L, "                    if joined_load_state {\n                        source_budget.record_load_state_join_failure", "                    if true {\n                        source_budget.record_load_state_join_failure"),
    ], [("fallback", FALLBACK), ("lib", LIB)]),
    "R2 pre-0.4 guards removed (wrapper + per-case record)": ([
        (L, "&& case_state::is_load_state(&request.model)\n", "\n"),
        (L, "                    if load_state.is_some() {\n                        source_budget.record_load_state_join_failure", "                    if true {\n                        source_budget.record_load_state_join_failure"),
    ], [("lib", LIB), ("all", ALL)]),
    "R3 fallback ledger keeps first-run charge": ([(L, "            load_state_join_withheld: Some(cause),\n            ..Default::default()", "            load_state_join_withheld: Some(cause),\n            charged: self.charged,\n            failed_charged: self.failed_charged,\n            attempts: self.attempts,\n            ..Default::default()")], [("fallback", FALLBACK), ("join", JOIN)]),
    "R4 declined attempt debits no work": ([(R, "            work: self.summary.work,\n        }\n    }", "            work: exact::WorkReport { charged: 0, rejected: 0, limit: self.summary.work.limit },\n        }\n    }")], [("fallback", FALLBACK)]),
    "R5 withheld cause omitted from diagnostic": ([(L, '(Some(_), Some(cause)) => format!("; the invocation publishes every case on its ordinary route because its selected join could not finalize: {cause}"),', '(Some(_), Some(_cause)) => String::new(),')], [("fallback", FALLBACK)]),
    "R6 reservation strict c<L-c": ([(R, "if charged <= limit.saturating_sub(charged) {", "if charged < limit.saturating_sub(charged) {")], [("fallback", FALLBACK), ("join", JOIN)]),
    "R7 fallback first failure overwritten by later": ([(L, "self.load_state_join_failure.get_or_insert(cause);", "self.load_state_join_failure = Some(cause);")], [("fallback", FALLBACK), ("join", JOIN)]),
}

only = os.environ.get("ONLY")
out = {}
for name, (edits, suites) in M.items():
    if only and not any(name.startswith(o) for o in only.split(",")):
        continue
    originals = {}
    try:
        for rel, old, new in edits:
            path = root / rel
            originals.setdefault(path, (path.read_bytes(), hashlib.sha256(path.read_bytes()).hexdigest()))
            text = path.read_text()
            assert text.count(old) == 1, (name, rel, text.count(old))
            path.write_text(text.replace(old, new))
        entry = {}
        for label, args in suites:
            run = subprocess.run(["cargo", "+1.97.1", *args], cwd=root, env=env, capture_output=True, text=True)
            text = run.stdout + run.stderr
            compile_error = "could not compile" in text
            failed = [l.split()[1] for l in text.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
            entry[label] = {"exit": run.returncode, "compile_error": compile_error,
                            "killed": run.returncode != 0 and not compile_error, "failed": failed,
                            "results": [l for l in text.splitlines() if l.startswith("test result:")]}
        out[name] = entry
        print(name, {k: ("KILLED" if v["killed"] else ("COMPILE_ERROR" if v["compile_error"] else "SURVIVED"), len(v["failed"])) for k, v in entry.items()}, flush=True)
        for k, v in entry.items():
            for t in v["failed"]:
                print("   ", k, t, flush=True)
    finally:
        for path, (original, digest) in originals.items():
            path.write_bytes(original)
            assert hashlib.sha256(path.read_bytes()).hexdigest() == digest
print("ALL RESTORED AND SHA256-VERIFIED")
print(json.dumps(out, indent=1))
