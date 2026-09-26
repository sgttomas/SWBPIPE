"""Mutation run for T1_WP2_DESKTOP_READERS on a scratch copy (never in place).

Each mutant disables one refusal or guard of the desktop change: a reader
`require_`/`blocks`/`composite` condition becomes `true`, a `unique(...)` check
is removed, a `throw` becomes an expression statement, or a named guard is
replaced. The selected vitest files
run against the mutated copy; a mutant is killed when any test fails. Usage:
    NODE_BIN_DIR=<node 24 bin> python run_mutants.py <desktop-root-of-scratch-copy> <out.json> [group ...]
The run resumes from <out.json>: recorded mutants are skipped.
"""
from __future__ import annotations
import json, re, subprocess, sys, time
from pathlib import Path

DESK = Path(sys.argv[1])
OUT = Path(sys.argv[2])
GROUPS = set(sys.argv[3:])
R = "src/features/results/"
PARITY = [R + "loadReferenceEvidence.parity.test.ts"]
READERS = [R + "loadReferenceReaders.test.ts", R + "loadReferenceReaderCases.test.ts"]
LEDGER = [R + "loadReferenceSourceLedger.test.ts"]
SESSION = ["src/features/workspace/loadReference.resultsSessionState.test.ts"]
ROUND = ["src/services/loadReferenceRoundTrip.test.ts"]
RUN = ["src/services/loadReferenceAnalysisRun.test.ts"]
OUTPUT = [R + "loadReferenceOutputRefusal.test.tsx"]


def sites(text: str, pattern: str):
    return [m.start() for m in re.finditer(pattern, text)]


def call_mutants(path: str, func: str, tests: list[str], group: str):
    """`func(<cond>, "NAME")` -> `func(true, "NAME")` at each call site (not the definition)."""
    text = (DESK / path).read_text()
    out = []
    for start in sites(text, rf"(?<![\w.]){func}\("):
        if text[max(0, start - 9):start].endswith("function "):
            continue
        # find the matching close paren and the last top-level comma (the name argument)
        depth, i, last_comma = 0, start + len(func), None
        while True:
            ch = text[i]
            if ch in "([{": depth += 1
            elif ch in ")]}":
                depth -= 1
                if depth == 0: break
            elif ch == "," and depth == 1: last_comma = i
            elif ch in "\"'`":
                q = ch; i += 1
                while text[i] != q:
                    if text[i] == "\\": i += 1
                    i += 1
            i += 1
        if last_comma is None:
            continue
        name = text[last_comma + 1:i].strip()
        mutated = text[:start + len(func) + 1] + "true" + text[last_comma:]
        line = text.count("\n", 0, start) + 1
        out.append({"group": group, "file": path, "line": line, "id": f"{Path(path).stem}:{line}:{func}:{name}", "kind": f"{func}(true, {name})", "text": mutated, "tests": tests})
    return out


def remove_call_mutants(path: str, func: str, tests: list[str], group: str):
    """`func(<items>, "NAME")` -> `void 0` at each call site (not the definition)."""
    text = (DESK / path).read_text()
    out = []
    for start in sites(text, rf"(?<![\w.]){func}\("):
        if text[max(0, start - 9):start].endswith("function "):
            continue
        depth, i, last_comma = 0, start + len(func), None
        while True:
            ch = text[i]
            if ch in "([{": depth += 1
            elif ch in ")]}":
                depth -= 1
                if depth == 0: break
            elif ch == "," and depth == 1: last_comma = i
            elif ch in "\"'`":
                q = ch; i += 1
                while text[i] != q:
                    if text[i] == "\\": i += 1
                    i += 1
            i += 1
        name = text[last_comma + 1:i].strip()
        line = text.count("\n", 0, start) + 1
        out.append({"group": group, "file": path, "line": line, "id": f"{Path(path).stem}:{line}:{func}:{name}", "kind": f"{func}(...) removed", "text": text[:start] + "void 0" + text[i + 1:], "tests": tests})
    return out


def throw_mutants(path: str, pattern: str, tests: list[str], group: str):
    """`throw X(` -> `void X(` at each site."""
    text = (DESK / path).read_text()
    out = []
    for start in sites(text, pattern):
        line = text.count("\n", 0, start) + 1
        snippet = text[start:text.find("\n", start)].strip()[:90]
        mutated = text[:start] + "void " + text[start + len("throw "):]
        out.append({"group": group, "file": path, "line": line, "id": f"{Path(path).stem}:{line}:throw", "kind": snippet, "text": mutated, "tests": tests})
    return out


def replace_mutant(group, mid, path, old, new, tests):
    text = (DESK / path).read_text()
    assert text.count(old) == 1, (mid, text.count(old))
    return {"group": group, "file": path, "line": text[:text.index(old)].count("\n") + 1, "id": mid, "kind": f"replace {old[:70]!r}", "text": text.replace(old, new), "tests": tests}


def build():
    m = []
    lr = R + "loadReferenceEvidence.ts"
    lrs = R + "loadReferenceSourceEvidence.ts"
    # R: every load-reference-1 reader refusal (pre-pass shared with the joined reader).
    m += call_mutants(lr, "require_", PARITY + READERS, "R")
    m += throw_mutants(lr, r"throw fail\(", PARITY + READERS, "R")
    m += throw_mutants(lr, r"throw new LoadReferenceError\(", PARITY + READERS, "R")
    m += throw_mutants(lr, r'throw new MalformedAccess\(', PARITY + READERS, "R")
    # J: every joined reader refusal, including each ported ledger check.
    m += call_mutants(lrs, "require_", PARITY + LEDGER, "J")
    m += throw_mutants(lrs, r"throw fail\(", PARITY + LEDGER, "J")
    m += throw_mutants(lrs, r"throw new LoadReferenceError\(", PARITY + LEDGER + READERS, "J")
    m += call_mutants(lrs, "blocks", LEDGER + PARITY, "L")
    m += call_mutants(lrs, "composite", LEDGER + PARITY, "L")
    m += remove_call_mutants(lrs, "unique", LEDGER + PARITY, "L")
    # D: dispatch and standing.
    q = R + "numericalResultQuality.ts"
    m.append(replace_mutant("D", "joined-early-return", q, 'return { contract, status: "needs_recompute" as const, eligible: false, findings: loadReferenceSourceStanding(source).findings };', 'void loadReferenceSourceStanding;', READERS + SESSION))
    m.append(replace_mutant("D", "lr-standing-reader", q, 'try { validateLoadReferenceEvidence(source, model ?? undefined); }', 'try { void validateLoadReferenceEvidence; }', READERS + SESSION))
    m.append(replace_mutant("D", "lr-header-profile-evidence", q, ': loadReference ? f?.profile_id === LOAD_REFERENCE_PROFILE && evidenceObject', ': loadReference ? true', READERS + PARITY))
    m.append(replace_mutant("D", "joined-header-profile-evidence", q, ': joined ? f?.profile_id === LOAD_REFERENCE_SOURCE_PROFILE && evidenceObject', ': joined ? true', READERS + PARITY))
    m.append(replace_mutant("D", "joined-header-receipt-shape", q, 'joined ? !loadReferenceSourceReceiptShape(source.source_block_recovery)', 'joined ? false', READERS + PARITY))
    m.append(replace_mutant("D", "joined-registration-binding", lrs, 'if (!registration || !current || current.text !== registration.text || current.negativeZeros !== registration.negativeZeros) {', 'if (!registration) {', READERS))
    m.append(replace_mutant("D", "joined-receipt-shape-fn", lrs, "return schemaShape(shaped, physicsSourceSchema, physicsSourceSchema);\n  } catch { return false; }", "return true;\n  } catch { return false; }", READERS + PARITY))
    m.append(replace_mutant("D", "native-registration-lr-reader", "src/services/previewService.ts", 'if (sourceContract(source) === "load_reference") validateLoadReferenceEvidence(source, capture.invocation.request.model);', '', SESSION))
    # P: lossless-projection and retention guards.
    m.append(replace_mutant("P", "browser-040-retention", "src/services/projectService.ts", "if (documentVersion.every((part, index) => part === loadReference[index])) {", "if (false) {", ROUND))
    m.append(replace_mutant("P", "analysis-run-joined-evidence", "src/services/analysisRunCompatibility.ts", 'if (route === "physics_source" || route === "load_reference_source") record.analysis_run.contract_evidence', 'if (route === "physics_source") record.analysis_run.contract_evidence', RUN + ROUND))
    m.append(replace_mutant("P", "analysis-run-joined-receipt", "src/services/analysisRunCompatibility.ts", 'if (route === "source_blocks" || route === "physics_source" || route === "load_reference_source") record.analysis_run.source_block_recovery', 'if (route === "source_blocks" || route === "physics_source") record.analysis_run.source_block_recovery', RUN))
    m.append(replace_mutant("P", "lr-projection-copy", lr, "const projected = structuredClone(source) as Json;", "const projected = source as Json;", READERS + PARITY))
    m.append(replace_mutant("P", "joined-validate-copy", lrs, "await guardedJoined(() => validate(structuredClone(source), true));", "await guardedJoined(() => validate(source, true));", READERS + PARITY))
    # O: output refusals.
    o = R + "loadReferenceOutputAvailability.ts"
    m.append(replace_mutant("O", "shared-output-refusal", o, "return isLoadReferenceRoute(source) ? LOAD_REFERENCE_OUTPUT_REFUSAL : null;", "return null;", OUTPUT))
    m.append(replace_mutant("O", "stress-neutral-builder-refusal", "src/features/stress-neutral/StressNeutralExportPanel.tsx", "  refuseLoadReferenceOutput(args.result);\n", "", OUTPUT))
    m.append(replace_mutant("O", "result-export-builder-refusal", "src/features/result-export/resultExportAdapter.ts", "  refuseLoadReferenceOutput(result);\n", "", OUTPUT))
    return [x for x in m if not GROUPS or x["group"] in GROUPS]


def run(mutant):
    path = DESK / mutant["file"]
    original = path.read_text()
    path.write_text(mutant["text"])
    try:
        t0 = time.time()
        try:
            # A mutant whose test run never terminates (for example an endless
            # loop) is detected, not passed: the run is stopped after 900 s.
            proc = subprocess.run(["npx", "vitest", "run", "--bail", "1", *mutant["tests"]], cwd=DESK, capture_output=True, text=True, timeout=900, env={**__import__("os").environ, "PATH": __import__("os").environ.get("NODE_BIN_DIR", "") + ":" + __import__("os").environ["PATH"]})
        except subprocess.TimeoutExpired:
            return {"killed": True, "seconds": round(time.time() - t0, 1), "summary": ["non-terminating test run stopped after 900 s"]}
        tail = [l for l in proc.stdout.splitlines() if "Tests " in l or "Test Files" in l]
        return {"killed": proc.returncode != 0, "seconds": round(time.time() - t0, 1), "summary": tail[-1:] if tail else proc.stderr[-300:]}
    finally:
        path.write_text(original)


def main():
    mutants = build()
    results = json.loads(OUT.read_text()) if OUT.exists() else []
    done = {r["id"] for r in results}
    print(f"{len(mutants)} mutants", flush=True)
    for mutant in mutants:
        if mutant["id"] in done:
            continue
        r = run(mutant)
        record = {k: mutant[k] for k in ("group", "id", "file", "line", "kind", "tests")} | r
        results.append(record)
        OUT.write_text(json.dumps(results, indent=1) + "\n")
        print(("KILLED  " if r["killed"] else "SURVIVED") + f" {mutant['id']}  {mutant['kind']}", flush=True)


if __name__ == "__main__":
    main()
