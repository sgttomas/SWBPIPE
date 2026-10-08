"""I90 B1-SR-RS repair round 1: RV113's M20, M02 and M11 (its mutant schema's definitions), in a scratch archive
of the repair head (WT/scratch/i90_b1_sr_rs/repair_01/mut).

Each mutant is a list of exact (old, new) replacements in one file (each `old` must occur exactly
once). For each: apply, run one cargo job through WT/tools/t3_cargo.sh
(`test --locked --offline --no-fail-fast --test retained_precision_contract --lib`, so RE's
contract test and its lib tests), record every failing test, and restore the file's bytes.
Usage: python3 mutants.py [ids...]
"""
import json, os, re, subprocess, sys
WT = "WT"
S = f"{WT}/scratch/i90_b1_sr_rs"
RE = f"{S}/repair_01/mut/projects/chirality-piping/core/reporting/result_export"
RS = "src/retained_precision.rs"
SB = "src/source_blocks.rs"
P1_OPEN = '''        // P1: exactly one mode row, valued with the mode code (1 sparse, 2 dense).
        fail(
            modes.len() == 1'''
P24 = '''        fail(
            parity <= 1
                && (parity == 0 || mode == "dense_scrutiny")
                && (parity == 0 || o["w2"]["kind"] != "published"),
        )?;
    }'''
NR = '''            fail(c["product_attempt_ref"].is_null() && quality["solve_quality"] == "checks_passed")?;'''
def nr(extra):
    return [(RS, NR, NR.replace(')?;', '') + f'\n                && {extra})?;')]
def d38_drop(line):
    return [(RS, line, '')]
MUTANTS = {
    "M20 not_required: product_attempt_ref null dropped": [(RS, 'fail(c["product_attempt_ref"].is_null() && quality["solve_quality"] == "checks_passed")?;', 'fail(quality["solve_quality"] == "checks_passed")?;')],
    "M02 (4b) branch: and made or (a Run-bearing native failure routed to (4b))": [(RS, 'if st["native"] == "failed" && a["run_ref"].is_null() {', 'if st["native"] == "failed" || a["run_ref"].is_null() {')],
    "M11 P4 stricter: a parity row only beside W2 not_triggered": [(RS, '&& (parity == 0 || o["w2"]["kind"] != "published"),', '&& (parity == 0 || o["w2"]["kind"] == "not_triggered"),')],
}
def run(mid, edits):
    originals = {}
    for f, old, new in edits:
        path = f"{RE}/{f}"
        if f not in originals:
            originals[f] = open(path, encoding="utf-8").read()
        text = open(path, encoding="utf-8").read()
        assert text.count(old) == 1, (mid, f, text.count(old), old[:80])
        open(path, "w", encoding="utf-8").write(text.replace(old, new))
    log = f"{S}/repair_01/mutants/logs/{re.sub(r'[^A-Za-z0-9]+', '_', mid)[:60]}.log"
    os.makedirs(os.path.dirname(log), exist_ok=True)
    env = dict(os.environ, TMPDIR=f"{S}/tmp", CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2", CARGO_TARGET_DIR=f"{WT}/targets/i90-b1-sr-rs/mut-r1")
    env.pop("RUSTFLAGS", None); env.pop("CARGO_ENCODED_RUSTFLAGS", None)
    try:
        with open(log, "w") as out:
            rc = subprocess.run([f"{WT}/tools/t3_cargo.sh", "test", "--locked", "--offline", "--no-fail-fast",
                                 "--test", "retained_precision_contract", "--lib"], cwd=RE, env=env, stdout=out, stderr=subprocess.STDOUT).returncode
    finally:
        for f, text in originals.items():
            open(f"{RE}/{f}", "w", encoding="utf-8").write(text)
    body = open(log, encoding="utf-8").read()
    failed = sorted(set(re.findall(r"^test (\S+) \.\.\. FAILED", body, re.M)))
    passed = len(re.findall(r"^test \S+ \.\.\. ok", body, re.M))
    compiled = "could not compile" not in body
    status = "KILLED" if failed else ("BUILD-FAILED" if not compiled else "SURVIVED")
    return {"id": mid, "status": status, "rc": rc, "failed": failed, "passed": passed, "log": os.path.basename(log)}
if __name__ == "__main__":
    ids = sys.argv[1:] or list(MUTANTS)
    pristine = {f: open(f"{RE}/{f}", encoding="utf-8").read() for f in (RS, SB)}
    with open(f"{S}/repair_01/mutants/results.jsonl", "a") as results:
        for mid in ids:
            r = run(mid, MUTANTS[mid])
            print(json.dumps(r), flush=True)
            results.write(json.dumps(r) + "\n")
    for f, text in pristine.items():
        assert open(f"{RE}/{f}", encoding="utf-8").read() == text, f
    print("restored: pristine bytes")
