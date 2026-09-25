import json, subprocess, sys
sys.path.insert(0, 'tests')
import importlib.util
spec = importlib.util.spec_from_file_location("t", "tests/test_headless_runner_contract.py")
t = importlib.util.module_from_spec(spec); spec.loader.exec_module(t)
FINAL='core/runner/headless/target/debug/openpipestress-runner'
COMPAT='core/runner/headless/target/debug/headless_preview_runner'
cands = sys.argv[1:]
for p in cands:
    d = json.load(open(p))
    payload = d if 'model' in d and 'materials' in d else {"model": d, "materials": []}
    body = {"request": t.final_runner_request("solve"), "solve": {"preview_model": payload}}
    c = t.run_final(FINAL, "solve", body, "--explicit-local-private-intent")
    out = json.loads(c.stdout)
    me = (out.get('payload') or {}).get('mechanics_envelope') or {}
    diags = [x['code'] for x in (out.get('payload') or {}).get('diagnostics', [])]
    medi = [x['code'] for x in me.get('diagnostics', []) if x.get('severity')=='blocking']
    print(p, 'FINAL exit', c.returncode, 'blocked', out['blocked'], 'mech', (me.get('status') or {}).get('mechanics'), 'results', len(me.get('results',[])), 'diag', diags[:5], 'meblock', medi[:5])
    if 'model' not in d or 'materials' not in d:
        c2 = subprocess.run([COMPAT, p, '--explicit-local-private-intent'], capture_output=True, text=True)
        o2 = json.loads(c2.stdout) if c2.stdout else None
        me2 = (o2['payload'] or {}).get('mechanics_envelope') or {} if o2 else {}
        print('   COMPAT exit', c2.returncode, 'blocked', o2 and o2['blocked'], 'mech', (me2.get('status') or {}).get('mechanics'), 'results', len(me2.get('results',[])), c2.stderr[:200])
