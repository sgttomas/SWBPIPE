"""CP4 SF-1 mutants: each edit is applied in place, the fallback tests run,
and the original bytes are restored and sha256-verified. Run from the
product_physics crate root with CARGO_TARGET_DIR set."""
import hashlib, pathlib, subprocess, sys

MUTANTS = [
    ("SF1-M1 replay reservation removed", "src/lib.rs",
     "Some(_) => recovery.reserve_captured_replay(case_limit),", "Some(_) => Ok(recovery),"),
    ("SF1-M2 fallback republication removed", "src/lib.rs",
     "(Some(request), Some(cause)) => {", "(Some(request), Some(cause)) if false => {"),
    ("SF1-M3 withheld attempt still selects", "src/lib.rs",
     "Some(_) if source_budget.load_state_join_withheld.is_some() => Err(recovery.decline_withheld()),",
     "Some(_) if false => Err(recovery.decline_withheld()),"),
    ("SF1-M4 per-case finalization failure not recorded", "src/lib.rs",
     'source_budget.record_load_state_join_failure(format!("case {}: {}", load_case.id, error.0));', ""),
    ("SF1-M5 invocation receipt failure not recorded", "src/lib.rs",
     'source_budget.record_load_state_join_failure(format!("invocation receipt: {}", error.0));', ""),
    ("SF1-M6 reservation off by the whole replay", "src/source_recovery.rs",
     "if charged <= limit.saturating_sub(charged) {", "if charged <= limit {"),
]

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

for name, rel, old, new in MUTANTS:
    path = pathlib.Path(rel)
    original = path.read_bytes()
    digest = sha(path)
    text = original.decode()
    assert text.count(old) == 1, (name, "anchor count", text.count(old))
    path.write_text(text.replace(old, new))
    try:
        run = subprocess.run(["cargo", "+1.97.1", "test", "--locked", "--offline", "-j", "2", "--lib",
                              "source_receipt::load_state_fallback_tests"], capture_output=True, text=True)
        failed = [l for l in run.stdout.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
        outcome = "KILLED" if run.returncode != 0 else "SURVIVED"
        print(f"{name}: {outcome}; failing={[l.split()[1].split('::')[-1] for l in failed]}", flush=True)
    finally:
        path.write_bytes(original)
        assert sha(path) == digest, (name, "restore")
print("all restored and sha256-verified")
