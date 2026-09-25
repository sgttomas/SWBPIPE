import json, subprocess, hashlib
FINAL='core/runner/headless/target/debug/openpipestress-runner'
def run(body,*a):
    return subprocess.run([FINAL,'export-results',*a],input=json.dumps(body),capture_output=True,text=True)
def rebind(b, name, ver):
    e=b.get('export_results')
    if not e: return b
    sv=e['audit_manifest']['solver_version']; sv.update(solver_name=name,solver_version=ver,solver_build_ref=f'{name}@{ver}')
    for env in e['result_envelopes']:
        env.update(solver_name=name,solver_version=ver,solver_build_ref=f'{name}@{ver}')
    return b
for f in ['success','producer_blocked','missing_payload','binding_mismatch']:
    b=json.load(open(f'validation/witness/inputs/del1005_export_results_{f}_input.json'))
    for label,body in [('as-recorded',json.loads(json.dumps(b))),('rebound-0.2.0',rebind(json.loads(json.dumps(b)),'open_pipe_stress_product_physics','0.2.0'))]:
        c=run(body,'--explicit-local-private-intent')
        o=json.loads(c.stdout)
        p=o['payload'] or {}
        codes=[(d['code'],d['message'][:90]) for d in p.get('diagnostics',[])]
        rp=p.get('report_package')
        print(f, label, 'exit',c.returncode,'blocked',o['blocked'],'pkg', rp and len(rp['container_bytes']), rp and rp['container_sha256_hex'][:16], codes[:3])
