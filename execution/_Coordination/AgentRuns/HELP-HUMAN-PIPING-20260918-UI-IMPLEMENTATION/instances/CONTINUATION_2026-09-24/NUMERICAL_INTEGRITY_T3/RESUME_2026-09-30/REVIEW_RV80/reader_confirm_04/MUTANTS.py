#!/usr/bin/env python3
"""RV80 single-edit mutants of the Rust retained-precision reader (own copy only).
reader_confirm_04: run on READER abcb16fd27 (M45-M50 aimed at D32). reader_confirm_03: run on READER a894d9d0ba (M13/M19 re-indented for reason_table; M36-M44 added). reader_confirm_02: M01-M18 re-applied to READER b36739112a (M05, M13 re-expressed on the repaired code), plus M19-M34 on the repairs.

Each mutant is applied to a pristine copy of retained_precision.rs inside WT/rv80,
the candidate's own test command is run, and the file is restored and re-hashed.
"""
import hashlib, json, os, re, subprocess, sys
from pathlib import Path

WT = Path(sys.argv[1])
SRC = WT / 'rv80/projects/chirality-piping/core/reporting/result_export/src/retained_precision.rs'
LOG = WT / 'scratch/rv80_reader_confirm4/mutants'
LOG.mkdir(parents=True, exist_ok=True)
PRISTINE = SRC.read_bytes()
PRISTINE_SHA = hashlib.sha256(PRISTINE).hexdigest()
assert PRISTINE_SHA.startswith('bb713adcebc6'), PRISTINE_SHA

M = [
 ('M01', 'fail-closed hold', 'const IMPLEMENTATION_COMPLETE: bool = false;', 'const IMPLEMENTATION_COMPLETE: bool = true;'),
 ('M02', 'N17 case-before-invocation budget scope',
  '                        !case_over\n                            && (inv_over', '                        (inv_over'),
 ('M03', 'I57 feasibility: positive force floor ORed into positive',
  '            positive[2] |= floor[0];', '            positive[2] |= false && floor[0];'),
 ('M04', 'I57 p512 charge = force/moment stop',
  'let charge = if p == 512 { [stop[2], stop[3]] } else { estimate };', 'let charge = estimate;'),
 ('M05', "R3' fault containment in the owner's statuses",
  '.is_some_and(|f| f & !statuses(owner) == 0)', '.is_some()'),
 ('M06', 'G5c relative-class boundary >= to >',
  'n.abs() >= 2f64.powi(-34) * sc', 'n.abs() > 2f64.powi(-34) * sc'),
 ('M07', 'G3 run-id contiguity dropped',
  'runs.keys().copied().eq(0..runs.len() as u64)', 'true'),
 ('M08', 'N10 idle exhaustion >= to > (schedule replay)',
  'af(u(&r["invocation_before"]) >= u(&b["work"]["invocation_limit"])\n                    && *terminal',
  'af(u(&r["invocation_before"]) > u(&b["work"]["invocation_limit"])\n                    && *terminal'),
 ('M09', 'I57 data fact: nonzero free nodal term implies has_data',
  "&& f(&t[\"value\"]) != 0.0\n        }) {", "&& f(&t[\"value\"]) != 0.0 && false\n        }) {"),
 ('M10', 'A1 small bound: +2^-1074 term dropped', 'limbs[0] = 1;', 'limbs[0] = 0;'),
 ('M11', 'I57 s3: Ready/certificate/G5a passed requires non-null coverage',
  '        pf(!coverage.is_null())?;\n    }\n    if coverage.is_null()', '        pf(true)?;\n    }\n    if coverage.is_null()'),
 ('M12', 'I57 item 4: verification bound non-null iff has_data',
  'x["value"].is_null() != has_data[i]', 'true'),
 ('M13', 'P8/D4d: native error run_ref equals the case Run',
  '\n                && e["run_ref"] == run["id"])?;', ')?;'),
 ('M14', 'C3:165 material basis agrees with the ordinary attempt',
  '\n            && a["material_basis_ref"] == ordinary["material_basis_ref"])?;', ')?;'),
 ('M15', 'C3:147 source preparation back-reference',
  '\n                && s["preparation"]["attempt_ref"] == json!(ai))?;', ')?;'),
 ('M16', 'N5 terminal(): budget scope payload dropped',
  '"budget" => unresolved("budget", &["scope"]),', '"budget" => unresolved("budget", &[]),'),
 ('M17', 'G5b p512 floor equality removed',
  'sf(s["floor"][bi][NAMES[j + 2]] == bits(phi))?;', 'sf(true)?;'),
 ('M18', 'G7 bare code: detail folded into code',
  '        detail: (code != text).then_some(text),\n        code,', '        detail: None,\n        code: text,'),
 ('M19', 'D4d: native with a selected Run invalid',
  '\n                && run["kernel_terminal"]["kind"] != "selected"\n                && e["run_ref"]', '\n                && e["run_ref"]'),
 ('M20', 'D1: old member ids 0..len (G3)',
  '&& old.iter().map(|m| u(&m["member"])).eq(0..old.len() as u64)', ''),
 ('M21', 'D3: native WORK deferred to the end of class 1',
  '    let wf = |ok: bool| -> VResult {\n        if !ok {\n            work_ok.set(false);\n        }\n        Ok(())\n    };',
  '    let wf = |ok: bool| -> VResult {\n        if !ok {\n            work_ok.set(false);\n        }\n        need(ok, "G5", "WORK_MISMATCH")\n    };'),
 ('M22', 'D5d: stop-rule quantity resolves in the layout', 'af(layout.iter().any(|row| {', 'af(true || layout.iter().any(|row| {'),
 ('M23', 'D5b: escalating failed verification has no pass',
  'af(record["verification"].is_null()\n                            && u(&record["work"]["verification_lme"]) == 0\n                            && record["verification_shared_build_ref"].is_null())?;', 'af(true)?;'),
 ('M24', 'D5a: candidate verification_lme = 0',
  '\n                        && u(&record["work"]["verification_lme"]) == 0\n                        && matches!(', '\n                        && matches!('),
 ('M25', 'D6b: selected quality excludes checks_passed', '"sensitive" | "unresolved" | "failed"', '"sensitive" | "unresolved" | "failed" | "checks_passed"'),
 ('M26', 'D4c: cause case has its own attempt',
  'pf(!c["product_attempt_ref"].is_null()\n                && c["product_attempt_ref"] == c["reason"]["cause"]["product_attempt_ref"])?;', 'pf(true)?;'),
 ('M27', 'D1: captured_prefix null source/run/unavailable at G5',
  'pf(a["source_ref"].is_null()\n                && a["run_ref"].is_null()\n                && a["result"]["kind"] == "unavailable")?;', 'pf(true)?;'),
 ('M28', 'D8 kernel scope: work_accounting anywhere in a Run/build/group',
  '.any(|o| o.get("tag").is_some_and(|t| t == "work_accounting")))?;', '.any(|o| false && o.get("tag").is_some_and(|t| t == "work_accounting")))?;'),
 ('M29', 'D8 R4: SectionError accounting needs non-exact member work', 'r4 &= statuses(&m["work"]) != 0;', 'r4 &= true;'),
 ('M30', "D8 R2': OperationalError accounting", '.any(|e| e["kind"] == "accounting" && e.get("event").is_none());', '.any(|e| false && e["kind"] == "accounting" && e.get("event").is_none());'),
 ('M31', 'D2: receipt_version exactly 1 at G0', 'unsupported(uint(&b["receipt_version"]) == Some(1))?;', 'unsupported(true)?;'),
 ('M32', 'D1: unsourced complete old count = CaseSource member count', '.all(|s| list(&s["id_maps"]["members"]).len() == old.len()),', '.all(|_| true),'),
 ('M33', 'D6a: untyped diagnostic_refs resolve', 'fail(refs.insert(text(id)) && ds.iter().any(|d| d["id"] == *id))?;', 'fail(refs.insert(text(id)))?;'),
 ('M34', 'D18: G5b echoed section terms positive', 'section(left[k] == right[k] && f(&right[k]) > 0.0)?;', 'section(left[k] == right[k])?;'),
 ('M35', 'D1: run origin owner is the case (class 1)', '\n                && *oi == json!({"kind":"case","index":ci})', ''),
 ('M36', 'D27 reverted: idle rule reads the running meter', '&& u(&r["invocation_before"]) >= u(&b["work"]["invocation_limit"])\n                    && r["kernel_terminal"]["reason"]["tag"] == "budget"', '&& current >= u(&b["work"]["invocation_limit"])\n                    && r["kernel_terminal"]["reason"]["tag"] == "budget"'),
 ('M37', 'D28 narrowed back to stop_rule', '"stop_rule" | "verification_estimate" | "charge" | "publication_enclosure"', '"stop_rule"'),
 ('M38', 'D29 general empty body inventory check dropped', 'fail(!list(&s["body_membership"]).is_empty())?;', 'fail(true)?;'),
 ('M39', 'D21 verification shared build as pass evidence dropped', '\n                            && record["verification_shared_build_ref"].is_null())?;', ')?;'),
 ('M40', 'D19 unavailable half dropped', 'pf(c["status"] == "unavailable"\n                && c["reason"]["cause"]["kind"] == "prepared_product_failure"\n                && u(&c["reason"]["cause"]["product_attempt_ref"]) == ai as u64)?;', 'pf(true)?;'),
 ('M41', 'D19 Ready half dropped', 'pf(c["status"] == "selected"\n                || (c["status"] == "unavailable"\n                    && c["reason"]["cause"]["kind"] == "receipt_failure"))?;', 'pf(true)?;'),
 ('M42', 'D20 selected case without a C3 attempt', '        if c["status"] == "selected" {\n            pf(!c["product_attempt_ref"].is_null())?;\n        }\n', ''),
 ('M43', 'D22 reverted: dangling attempt source_ref fails G3', 'if let Ok(s) = at(&b["sources"], &a["source_ref"], "G3", "COVERAGE_MISMATCH") {', 'if let Some(s) = (!a["source_ref"].is_null()).then(|| at(&b["sources"], &a["source_ref"], "G3", "COVERAGE_MISMATCH")).transpose()? {'),
 ('M44', 'D5b escalating failed verification: report null dropped (D21 third indicator)', 'af(record["verification"].is_null()\n                            && u(&record["work"]["verification_lme"]) == 0\n                            && record["verification_shared_build_ref"].is_null())?;', 'af(u(&record["work"]["verification_lme"]) == 0\n                            && record["verification_shared_build_ref"].is_null())?;'),
 ('M45', 'D32 normalization disabled', 'if !floats(&source["retained_precision"]) {', 'if true || !floats(&source["retained_precision"]) {'),
 ('M46', 'D32 normalization also converts -0', '                    && x.abs() <= SAFE as f64\n                    && !(*x == 0.0 && x.is_sign_negative())', '                    && x.abs() <= SAFE as f64'),
 ('M47', 'D32 normalization range guard dropped', '                    && x.abs() <= SAFE as f64\n', '\n'),
 ('M48', 'D32 at G0: receipt_version back to the host type test', 'unsupported(uint(&b["receipt_version"]) == Some(1))?;', 'unsupported(b["receipt_version"].as_u64() == Some(1) && b["receipt_version"].is_u64())?;'),
 ('M49', 'D32 at G0: limits back to as_u64', 'unsupported(uint(&b["work"][key]) == Some(want))?;', 'unsupported(b["work"][key].as_u64() == Some(want))?;'),
 ('M50', 'D32 normalization widened to the whole statement', '    if let Some(r) = owned.get_mut("retained_precision") {\n        normalize(r);\n    }', '    normalize(&mut owned);'),
 ('M51', 'D33 dropped', 'af(reason["tag"] != "verification_estimate"\n                        || matches!(text(&reason["kind"]), "force" | "moment"))?;', 'af(true)?;'),
 ('M52', 'D31 back to 0.2.0/0.3.0', 'matches!(text(&model["schema_version"]), "0.1.0" | "0.2.0" | "0.3.0")', 'matches!(text(&model["schema_version"]), "0.2.0" | "0.3.0")'),
]

env = dict({k: v for k, v in os.environ.items() if k != 'DEVELOPER_DIR'}, CARGO_BUILD_JOBS='4',
           RUST_TEST_THREADS='2', CARGO_NET_OFFLINE='true', CARGO_TARGET_DIR=str(WT / 'targets/rv80'))
cmd = ['cargo', 'test', '--locked', '--offline', '--manifest-path',
       str(WT / 'rv80/projects/chirality-piping/core/reporting/result_export/Cargo.toml'),
       '--test', 'retained_precision_contract', '--', '--test-threads=2']
only = set(sys.argv[2:])
results = []
for mid, what, old, new in M:
    if only and mid not in only:
        continue
    text = PRISTINE.decode()
    n = text.count(old)
    if n != 1:
        results.append({'id': mid, 'what': what, 'error': f'pattern count {n}'})
        print(mid, 'PATTERN COUNT', n, flush=True)
        continue
    SRC.write_text(text.replace(old, new))
    msha = hashlib.sha256(SRC.read_bytes()).hexdigest()
    p = subprocess.run(cmd, cwd=WT / 'rv80', env=env, capture_output=True, text=True, timeout=1200)
    out = p.stdout + p.stderr
    (LOG / f'{mid}.log').write_text(out)
    SRC.write_bytes(PRISTINE)
    assert hashlib.sha256(SRC.read_bytes()).hexdigest() == PRISTINE_SHA
    failed = re.findall(r'^test (\S+) \.\.\. FAILED', out, re.M)
    summary = re.findall(r'test result: .*', out)
    compiled = 'error[' not in out and 'could not compile' not in out
    verdict = 'KILLED' if (p.returncode != 0 and compiled) else ('COMPILE_ERROR' if not compiled else 'SURVIVED')
    r = {'id': mid, 'what': what, 'old': old, 'new': new, 'mutant_sha256': msha, 'exit': p.returncode,
         'verdict': verdict, 'failed_tests': failed, 'summary': summary}
    results.append(r)
    print(mid, verdict, failed, summary, flush=True)
(LOG / ('results_' + ('_'.join(sorted(only)) if only else 'all') + '.json')).write_text(json.dumps(results, indent=1))
print('restored', hashlib.sha256(SRC.read_bytes()).hexdigest())
