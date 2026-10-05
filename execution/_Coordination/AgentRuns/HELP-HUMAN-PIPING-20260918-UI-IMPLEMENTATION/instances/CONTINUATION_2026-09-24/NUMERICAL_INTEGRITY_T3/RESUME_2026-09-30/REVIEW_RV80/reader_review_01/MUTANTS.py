#!/usr/bin/env python3
"""RV80 single-edit mutants of the Rust retained-precision reader (own copy only).

Each mutant is applied to a pristine copy of retained_precision.rs inside WT/rv80,
the candidate's own test command is run, and the file is restored and re-hashed.
"""
import hashlib, json, os, re, subprocess, sys
from pathlib import Path

WT = Path(sys.argv[1])
SRC = WT / 'rv80/projects/chirality-piping/core/reporting/result_export/src/retained_precision.rs'
LOG = WT / 'scratch/rv80_reader_review/mutants'
LOG.mkdir(parents=True, exist_ok=True)
PRISTINE = SRC.read_bytes()
PRISTINE_SHA = hashlib.sha256(PRISTINE).hexdigest()
assert PRISTINE_SHA.startswith('bd20dd9a8f88'), PRISTINE_SHA

M = [
 ('M01', 'fail-closed hold', 'const IMPLEMENTATION_COMPLETE: bool = false;', 'const IMPLEMENTATION_COMPLETE: bool = true;'),
 ('M02', 'N17 case-before-invocation budget scope',
  '                        !case_over\n                            && (inv_over', '                        (inv_over'),
 ('M03', 'I57 feasibility: positive force floor ORed into positive',
  '            positive[2] |= floor[0];', '            positive[2] |= false && floor[0];'),
 ('M04', 'I57 p512 charge = force/moment stop',
  'let charge = if p == 512 { [stop[2], stop[3]] } else { estimate };', 'let charge = estimate;'),
 ('M05', 'R3 work_accounting fault containment',
  '.is_some_and(|f| f & !seen == 0));', '.is_some());'),
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
 ('M13', 'P8: native error run_ref equals the case Run',
  '                        pf(e["run_ref"] == run["id"])?;\n', ''),
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
]

env = dict(os.environ, DEVELOPER_DIR='/Library/Developer/CommandLineTools', CARGO_BUILD_JOBS='4',
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
