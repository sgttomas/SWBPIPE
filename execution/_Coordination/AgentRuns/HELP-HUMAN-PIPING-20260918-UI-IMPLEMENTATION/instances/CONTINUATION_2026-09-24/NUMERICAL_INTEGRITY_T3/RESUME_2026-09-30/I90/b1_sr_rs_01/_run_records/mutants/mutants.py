"""I90 B1-SR-RS: the mutant programme, in a scratch archive of the head (WT/scratch/i90_b1_sr_rs/mut).

Each mutant is a list of exact (old, new) replacements in one file (each `old` must occur exactly
once). For each: apply, run one cargo job through WT/tools/t3_cargo.sh
(`test --locked --offline --no-fail-fast --test retained_precision_contract --lib`, so RE's
contract test and its lib tests), record every failing test, and restore the file's bytes.
Usage: python3 mutants.py [ids...]
"""
import json, os, re, subprocess, sys
WT = "WT"
S = f"{WT}/scratch/i90_b1_sr_rs"
RE = f"{S}/mut/projects/chirality-piping/core/reporting/result_export"
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
    # G8: the per-case loop.
    "G8-1 P1-P4 for case 0 only": [(RS, P1_OPEN, P1_OPEN.replace('        fail(', '        if i == 0 {\n        fail(', 1)),
                                   (RS, P24, P24[:-6] + '\n        }\n    }')],
    "G8-2 P1-P4 for selected cases only (B1's widening undone)": [(RS, P1_OPEN, P1_OPEN.replace('        fail(', '        if b["cases"][i]["status"] == "selected" {\n        fail(', 1)),
                                   (RS, P24, P24[:-6] + '\n        }\n    }')],
    "G8-3 P1 dropped": [(RS, P1_OPEN, P1_OPEN.replace('modes.len() == 1', 'true || modes.len() == 1'))],
    "G8-4 P1 'exactly one' dropped (first row's value only)": [(RS, P1_OPEN, P1_OPEN.replace('modes.len() == 1', '!modes.is_empty()'))],
    "G8-5 P2 dropped": [(RS, '            parity <= 1\n', '            true\n')],
    "G8-6 P3 dropped": [(RS, '                && (parity == 0 || mode == "dense_scrutiny")\n', '')],
    "G8-7 P4 dropped": [(RS, '                && (parity == 0 || o["w2"]["kind"] != "published"),\n', ',\n')],
    "G8-8 the old parity rule restored (exactly one iff dense)": [(RS, P24, '''        fail(parity == usize::from(mode == "dense_scrutiny"))?;
    }''')],
    # G5: the not_required rule's three dropped conjuncts, each restored.
    "G5-1 initial.kind == report restored": nr('o["initial"]["kind"] == "report"'),
    "G5-2 initial.outcome == checks_passed restored": nr('o["initial"]["outcome"] == "checks_passed"'),
    "G5-3 w2.kind == not_triggered restored": nr('o["w2"]["kind"] == "not_triggered"'),
    # R-D38: the relaxed check restored, and each (4b) conjunct dropped.
    "D38-1 the run_ref requirement restored (no (4b))": [(RS, '''            if st["native"] == "failed" && a["run_ref"].is_null() {
                pf(d38_capture_before_run(c, a, ai))?;
            } else {
                pf(!a["run_ref"].is_null() && st["preparation"] == "completed")?;
            }''', '''            pf(!a["run_ref"].is_null() && st["preparation"] == "completed")?;''')],
    "D38-2 (4b) source equality dropped": d38_drop('\n        && a["source_ref"] == c["source_ref"]'),
    "D38-3 (4b) source non-null dropped": d38_drop('\n        && !a["source_ref"].is_null()'),
    "D38-4 (4b) result unavailable dropped": [(RS, '    a["result"]["kind"] == "unavailable"\n        && a["result"]["error"]["kind"] == "capture"', '    a["result"]["error"]["kind"] == "capture"')],
    "D38-5 (4b) error kind capture dropped": d38_drop('\n        && a["result"]["error"]["kind"] == "capture"'),
    "D38-6 (4b) preparation completed dropped": [(RS, '        && st["preparation"] == "completed"\n        && STAGE8[2..]', '        && STAGE8[2..]')],
    "D38-7 (4b) later stages not_entered dropped": [(RS, '''        && STAGE8[2..]
            .iter()
            .chain(&["observables", "g5a"])
            .all(|k| st[*k] == "not_entered")\n''', '')],
    "D38-8 (4b) case unavailable dropped": [(RS, '        && c["status"] == "unavailable"\n        && c["reason"]["cause"]["kind"] == "prepared_product_failure"', '        && c["reason"]["cause"]["kind"] == "prepared_product_failure"')],
    "D38-9 (4b) cause kind dropped": d38_drop('\n        && c["reason"]["cause"]["kind"] == "prepared_product_failure"'),
    "D38-10 (4b) cause attempt dropped": d38_drop('\n        && u(&c["reason"]["cause"]["product_attempt_ref"]) == ai as u64'),
    "D38-11 (4b) reason code dropped": d38_drop('\n        && c["reason"]["code"] == "source_unavailable"'),
    "D38-12 (4b) reason phase dropped": d38_drop('\n        && c["reason"]["phase"] == "preparation"'),
    # RV95 N-5.
    "S1 integer bound removed (RV95's S1)": [(SB, '        .filter(|n| *n <= 9_007_199_254_740_991)\n', '')],
    "S1b integer bound off by one": [(SB, '*n <= 9_007_199_254_740_991', '*n < 9_007_199_254_740_991')],
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
    log = f"{S}/mutants/logs/{re.sub(r'[^A-Za-z0-9]+', '_', mid)[:60]}.log"
    os.makedirs(os.path.dirname(log), exist_ok=True)
    env = dict(os.environ, TMPDIR=f"{S}/tmp", CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2", CARGO_TARGET_DIR=f"{WT}/targets/i90-b1-sr-rs/mut")
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
    with open(f"{S}/mutants/results.jsonl", "a") as results:
        for mid in ids:
            r = run(mid, MUTANTS[mid])
            print(json.dumps(r), flush=True)
            results.write(json.dumps(r) + "\n")
    for f, text in pristine.items():
        assert open(f"{RE}/{f}", encoding="utf-8").read() == text, f
    print("restored: pristine bytes")
