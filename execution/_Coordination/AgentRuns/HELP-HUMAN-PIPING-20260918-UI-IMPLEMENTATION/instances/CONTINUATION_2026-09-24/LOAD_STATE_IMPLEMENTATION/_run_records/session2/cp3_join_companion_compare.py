"""Compare the 0.4.0 unchanged-state joined companions with the 0.3.0
physics-source-1 raw envelopes. Intended differences are excluded explicitly:
case-wide modulus-basis rows and the redundant-G warning (0.4.0 records the
ignored G in member evidence), NOT_JOINED diagnostics, material_basis and
pipe_materials, retained identity digests, pressure temperature_basis, and
receipt hashes/work/ordinary-attempt text. Run from WORKING_ROOT."""
import json,sys,struct
root='fixtures/product_preview/physics_source'
SP=None
def bits(v): return struct.pack('>d',v).hex() if isinstance(v,float) else v
ok=True
for w in ['n05','n06','fields','mixed']:
  for m,mm in [('sparse','sparse_interactive'),('dense','dense_scrutiny')]:
    a=json.load(open(f'{root}/{w}-{mm}.raw.json'))
    b=json.load(open(f'fixtures/product_preview/load_reference_source/{w}-{mm}.raw.json'))
    ra=[(r['id'],r['kind'],bits(float(r['value'])),r['unit'],r['entity_ref'],json.dumps(r.get('metadata'),sort_keys=True),json.dumps(r.get('basis_ref'),sort_keys=True)) for r in a['results']]
    rb=[(r['id'],r['kind'],bits(float(r['value'])),r['unit'],r['entity_ref'],json.dumps(r.get('metadata'),sort_keys=True),json.dumps(r.get('basis_ref'),sort_keys=True)) for r in b['results']]
    drop=lambda rows:[r for r in rows if ':modulus-basis:' not in r[0]]
    res= drop(ra)==rb
    nq= a['numerical_quality']==b['numerical_quality']
    sm= a['summary']==b['summary']
    da=[(d['id'],d['code'],d['severity']) for d in a['diagnostics']]
    da=[x for x in da if x[1]!='EXACT_PRESSURE_REDUNDANT_G_IGNORED']
    db=[(d['id'],d['code'],d['severity']) for d in b['diagnostics'] if d['code']!='LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED']
    dg= da==db
    ea=a['contract_evidence']['exact_cases']; eb=b['contract_evidence']['exact_cases']
    def strip(e):
      e=dict(e); e.pop('material_basis'); e.pop('pipe_materials')
      e['pipe_stress_extrema']=[{k:v for k,v in x.items() if k!='source_identity_sha256'} for x in e['pipe_stress_extrema']]
      return e
    ex= [strip(x) for x in ea]==[strip(x) for x in eb]
    def pstrip(p):
      p=json.loads(json.dumps(p))
      for region in p:
        for m in region.get('materials',[]): m.pop('temperature_basis',None)
      return p
    pr= pstrip(a['contract_evidence']['pressure'])==pstrip(b['contract_evidence']['pressure'])
    ca=a['source_block_recovery']['body']['cases']; cb=b['source_block_recovery']['body']['cases']
    def cstrip(c):
      c=dict(c); c.pop('physical_evidence_sha256'); c.pop('work'); 
      if c.get('source'): 
        s=dict(c['source']); [s.pop(k,None) for k in ('normalized_source_sha256','functional_plan_sha256','retained_identity_sha256')]; c['source']=s
      c.pop('ordinary_attempt')
      c['rows']=[r for r in c['rows'] if ':modulus-basis:' not in r['result_id']]
      return json.dumps(c,sort_keys=True)
    rc= [cstrip(x) for x in ca]==[cstrip(x) for x in cb]
    pol=b['source_block_recovery']['body']['policy']
    print(w,m,'results',res,'nq',nq,'summary',sm,'diag',dg,'exact',ex,'pressure',pr,'receipt_cases',rc,pol, b['source_block_recovery']['body']['status'])
    ok = ok and res and nq and sm and dg and ex and pr and rc
    if not rc:
      for x,y in zip(ca,cb):
        X=json.loads(cstrip(x)); Y=json.loads(cstrip(y))
        for k in X:
          if X[k]!=Y.get(k): print('   diff key',k, str(X[k])[:200], '|', str(Y.get(k))[:200])
print('ALL_MATCH' if ok else 'MISMATCH')
