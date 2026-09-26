"""CP4 review repairs: SF-1 mutants anchored to the committed (rustfmt) bytes.
Supersedes cp4_sf1_mutations.py, whose M3-M5 anchors matched only pre-rustfmt
bytes (CP4 review N-1). Each edit is applied in place, the fallback tests
run, and the original bytes are restored and sha256-verified. Every anchor
must occur exactly once. Run from core/product_physics with CARGO_TARGET_DIR set."""
import hashlib, os, pathlib, subprocess

L, R = "src/lib.rs", "src/source_recovery.rs"
MUTANTS = [
    ("SF1-M1 screen removed", L,
     ".reserve_captured_replay(case_limit)\n                .and_then(|recovery| {",
     ".reserve_captured_replay(usize::MAX)\n                .and_then(|recovery| {"),
    ("SF1-M2 fallback republication removed", L,
     "(Some(request), Some(cause)) => {", "(Some(request), Some(cause)) if false => {"),
    ("SF1-M3 withheld attempt still selects", L,
     "if source_budget.load_state_join_withheld.is_some() {\n                        // ROOT CP3 SF-1: the invocation publishes ordinarily.",
     "if false {\n                        // ROOT CP3 SF-1: the invocation publishes ordinarily."),
    ("SF1-M4 per-case finalization failure not recorded", L,
     'source_budget.record_load_state_join_failure(format!(\n                            "case {}: {}",\n                            load_case.id, error.0\n                        ));', ""),
    ("SF1-M5 invocation receipt failure not recorded", L,
     'source_budget.record_load_state_join_failure(format!(\n                            "invocation receipt: {}",\n                            error.0\n                        ));', ""),
    ("SF1-M6 screen off by the whole replay", R,
     "if charged <= limit.saturating_sub(charged) {", "if charged <= limit {"),
    ("N3-M7 withheld decided before the case's own screen", L,
     ".reserve_captured_replay(case_limit)\n                .and_then(|recovery| {\n                    if source_budget.load_state_join_withheld.is_some() {",
     ".reserve_captured_replay(if source_budget.load_state_join_withheld.is_some() { usize::MAX } else { case_limit })\n                .and_then(|recovery| {\n                    if source_budget.load_state_join_withheld.is_some() {"),
    ("N2-M8 fallback ledger reset", L,
     "            charged: self.charged,\n            failed_charged: self.failed_charged,\n            publication_charged: self.publication_charged,\n            rejected: self.rejected,\n            attempts: self.attempts,\n",
     "            charged: 0,\n            failed_charged: 0,\n            publication_charged: 0,\n            rejected: 0,\n            attempts: 0,\n"),
]

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

ONLY = os.environ.get("ONLY")
for name, rel, old, new in MUTANTS:
    if ONLY and not name.startswith(ONLY):
        continue
    path = pathlib.Path(rel)
    original = path.read_bytes()
    digest = sha(path)
    text = original.decode()
    assert text.count(old) == 1, (name, "anchor count", text.count(old))
    path.write_text(text.replace(old, new))
    try:
        run = subprocess.run(["cargo", "+1.97.1", "test", "--locked", "--offline", "-j", "2", "--lib",
                              "source_receipt::load_state_fallback_tests"], capture_output=True, text=True)
        failed = [l.split()[1].split("::")[-1] for l in run.stdout.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
        compile_error = "error[" in run.stderr
        outcome = "KILLED" if run.returncode != 0 else "SURVIVED"
        print(f"{name}: {outcome}; compile_error={compile_error}; failing={failed}", flush=True)
    finally:
        path.write_bytes(original)
        assert sha(path) == digest, (name, "restore")
print("all restored and sha256-verified")
