"""Reviewer's own mutants (T1_WAVE1_REVIEW_A), run on a git-archive scratch copy.

Each anchor must occur exactly once; one mutant at a time; the file is restored
and byte-checked after each run. Usage: mutants.py <COPY_WORKING_ROOT> <LOG_DIR> [ids...]
"""
import hashlib, os, pathlib, subprocess, sys, time

root = pathlib.Path(sys.argv[1]).resolve()
logs = pathlib.Path(sys.argv[2]); logs.mkdir(parents=True, exist_ok=True)
only = set(sys.argv[3:])
RS = "core/reporting/result_export/src/"
PY = "core/analysis_runs/"
M = [
    ("RS-R7-WHOLE-PRESSURE", RS + "load_reference_source.rs",
     '"pressure": case_pressure(pressure, &exact["load_case_id"]),\n            "load_reference_state": record,',
     '"pressure": pressure,\n            "load_reference_state": record,'),
    ("RS-STANDING-EARLY-RETURN-REMOVED", RS + "semantic_contract.rs",
     '    if source["producer"]["semantic_contract_id"] == LOAD_REFERENCE_SOURCE_ID {\n        return "needs_recompute";\n    }\n',
     ''),
    ("RS-STANDING-BEFORE-VALIDATION", RS + "semantic_contract.rs",
     '    let Ok((_, version)) = for_source(source) else {',
     '    if source["producer"]["semantic_contract_id"] == LOAD_REFERENCE_SOURCE_ID {\n        return "needs_recompute";\n    }\n    let Ok((_, version)) = for_source(source) else {'),
    ("RS-PROJECT-KEEPS-RECORDS", RS + "load_reference.rs",
     '        o.remove("load_reference_states");', '        let _ = o;'),
    ("RS-PROJECT-STALE-PHYSICAL", RS + "load_reference_source.rs",
     '        case["physical_evidence_sha256"] = json!(domain_hash(\n            PHYSICS_SOURCE_CASE_DOMAIN,',
     '        let _unused = json!(domain_hash(\n            PHYSICS_SOURCE_CASE_DOMAIN,'),
    ("RS-J0-PROFILE-CLAUSE", RS + "load_reference_source.rs",
     '\n            && source["formulation_basis"]["profile_id"] == PROFILE,', ','),
    ("RS-S10B-ANY-TO-ALL", RS + "load_reference.rs",
     'records.iter().any(|r| is_selected(r)),', 'records.iter().all(|r| is_selected(r)),'),
    ("RS-S13-SELECTED-ID-DASHED", RS + "load_reference.rs",
     'format!("diagnostic:source-recovery:{case_id}:selected"),',
     'format!("diagnostic:source-recovery:{}:selected", case_id.replace(\':\', "-")),'),
    ("RS-DERIVATIVE-EVIDENCE-COPY", RS + "derivative.rs",
     '                    | crate::semantic_contract::LOAD_REFERENCE_ID\n                    | crate::semantic_contract::LOAD_REFERENCE_SOURCE_ID\n            )\n        ) {\n            e["contract_evidence"]',
     '                    | crate::semantic_contract::LOAD_REFERENCE_ID\n            )\n        ) {\n            e["contract_evidence"]'),
    ("PY-R7-WHOLE-PRESSURE", PY + "load_reference_source.py",
     '"pressure": _case_pressure(pressure, exact_case["load_case_id"]), "load_reference_state": record}',
     '"pressure": list(pressure), "load_reference_state": record}'),
    ("PY-STANDING-EARLY-RETURN-REMOVED", PY + "compatibility.py",
     '    if contract == LOAD_REFERENCE_SOURCE_CONTRACT_ID:\n', '    if False:\n'),
    ("PY-STANDING-BEFORE-VALIDATION", PY + "compatibility.py",
     '    try:\n        contract, _, _ = _source_contract(source)\n    except (ValueError, KeyError, TypeError, AttributeError):\n        return "unsupported"\n',
     '    if isinstance(source, Mapping) and isinstance(source.get("producer"), Mapping) and source["producer"].get("semantic_contract_id") == LOAD_REFERENCE_SOURCE_CONTRACT_ID:\n        return "needs_recompute"\n    try:\n        contract, _, _ = _source_contract(source)\n    except (ValueError, KeyError, TypeError, AttributeError):\n        return "unsupported"\n'),
    ("PY-J0-PROFILE-CLAUSE", PY + "load_reference_source.py",
     ' and lr._eq(lr._get(lr._get(source, "formulation_basis"), "profile_id"), PROFILE), "JOIN_IDENTITY")',
     ', "JOIN_IDENTITY")'),
    ("PY-S10B-ANY-TO-ALL", PY + "load_reference_evidence.py",
     'any(_is_selected(record) for record in records)', 'all(_is_selected(record) for record in records)'),
    ("PY-S13-SELECTED-ID-DASHED", PY + "load_reference_evidence.py",
     'f"diagnostic:source-recovery:{case_id}:selected"',
     'f"diagnostic:source-recovery:{case_id.replace(\':\', \'-\')}:selected"'),
    ("PY-ANALYSIS-RUN-EVIDENCE-COPY", PY + "compatibility.py",
     '    if contract_id in {PHYSICS_SOURCE_CONTRACT_ID, LOAD_REFERENCE_SOURCE_CONTRACT_ID}:\n        envelope["analysis_run"]["contract_evidence"]',
     '    if contract_id in {PHYSICS_SOURCE_CONTRACT_ID}:\n        envelope["analysis_run"]["contract_evidence"]'),
]
env = dict(os.environ)
env["PYTHONDONTWRITEBYTECODE"] = "1"
summary = []
for mid, rel, anchor, repl in M:
    if only and mid not in only:
        continue
    path = root / rel
    original = path.read_bytes()
    before = hashlib.sha256(original).hexdigest()
    text = original.decode()
    n = text.count(anchor)
    if n != 1:
        summary.append(f"{mid}: ANCHOR_COUNT={n} (not run)")
        continue
    path.write_text(text.replace(anchor, repl))
    t0 = time.time()
    if mid.startswith("RS-"):
        cmd = ["cargo", "+1.97.1", "test", "--locked", "--offline", "-j", "2",
               "--test", "load_reference_source_contract", "--test", "derivative_contract", "--test", "load_reference_contract"]
        cwd = root / "core/reporting/result_export"
    else:
        cmd = [sys.executable, "-m", "pytest", "-x", "-q", "-p", "no:cacheprovider",
               "tests/test_load_reference_source_readers.py"]
        cwd = root
    proc = subprocess.run(cmd, cwd=cwd, env=env, capture_output=True, text=True)
    out = proc.stdout + proc.stderr
    (logs / f"{mid}.log").write_text(f"$ {' '.join(cmd)}\nexit={proc.returncode}\n" + out[-6000:])
    path.write_bytes(original)
    assert hashlib.sha256(path.read_bytes()).hexdigest() == before
    killed = proc.returncode != 0
    compile_error = "error[E" in out or "could not compile" in out
    first = ""
    for line in out.splitlines():
        if ("panicked" in line or "FAILED" in line or "assert" in line) and not first:
            first = line.strip()[:220]
    summary.append(f"{mid}: {'KILLED' if killed else 'SURVIVED'}{' (COMPILE ERROR)' if compile_error else ''} [{time.time()-t0:.0f}s] {first}")
    print(summary[-1], flush=True)
(logs / "summary.txt").write_text("\n".join(summary) + "\n")
print("\n".join(summary))
