import json, importlib.util
spec = importlib.util.spec_from_file_location("t", "tests/test_headless_runner_contract.py")
t = importlib.util.module_from_spec(spec); spec.loader.exec_module(t)
FINAL='core/runner/headless/target/debug/openpipestress-runner'
m=json.load(open('fixtures/product_preview/invented_preview_model.json'))
body={"request": t.final_runner_request("solve"), "solve":{"preview_model":{"model":m,"materials":[]}}}
for args in (["--explicit-local-private-intent"], []):
    c=t.run_final(FINAL,"solve",body,*args)
    o=json.loads(c.stdout)
    print('ARGS',args,'exit',c.returncode,'blocked',o['blocked'], 'payload keys', list(o['payload'].keys()) if o['payload'] else None)
    if o['payload']:
        p=o['payload']
        print(' command',p['command'],'op',p['operation'])
        print(' diagnostics',p['diagnostics'])
        for k in ('request_validation','result_validation'):
            v=p.get(k); print(' ',k, json.dumps(v)[:800])
        rr=p.get('runner_result'); print(' runner_result.analysis_status', rr and rr['analysis_status'], 'job', rr and rr['job']['state'], 'diag', rr and rr['diagnostics'], 'result_refs', rr and len(rr['result_refs']))
        me=p['mechanics_envelope']; print(' mech', me['status'], len(me['results']), sorted({(d['code'],d['severity'],d['source']) for d in me['diagnostics'] if d['severity']=='blocking'}))
        print(' other keys', {k: (type(v).__name__) for k,v in p.items()})
    print(' summary', o['summary'])
