from review_io import *
import datetime
results={'component_seals':[],'historical_inputs':[],'raw_hashes':[],'instruction_basis':[]}
for claim in packet('SEALED_BASIS_CHECK.json'):
    base=claim['packet']; manifest=get(base+'/SHA256SUMS'); entries=[]
    for line in manifest.decode().splitlines():
        if not line.strip():continue
        expected,name=line.split(None,1);name=name.lstrip('* ')
        path=name if name.startswith('projects/') else base+'/'+name
        raw=get(path);actual=hashlib.sha256(raw).hexdigest()
        entries.append({'path':path,'sha256':actual,'expected':expected,'ok':actual==expected})
    seal=hashlib.sha256(manifest).hexdigest()
    results['component_seals'].append({'packet':base,'seal':seal,'seal_matches_I25':seal==claim['seal'],'entries':entries,'count_matches_I25':len(entries)==claim['payloads']})
comparison=jget(R+'/I21/k0_assembly_16/HISTORICAL_COMPARISON.json')
for x in comparison['inputs']:
    got=hashlib.sha256(get(x['path'])).hexdigest()
    results['historical_inputs'].append({'path':x['path'],'sha256':got,'expected':x['sha256'],'ok':got==x['sha256']})
# Only the finite already named original raw paths. No searching/reconstruction.
rawinventory=[x for x in packet('INPUT_INVENTORY.json') if x['origin'].startswith('/')]
for x in rawinventory:
    p=pathlib.Path(x['origin']);raw=p.read_bytes();got=hashlib.sha256(raw).hexdigest()
    results['raw_hashes'].append({'path':str(p),'sha256':got,'expected':x['sha256'],'ok':got==x['sha256']})
    if p.name in ['run_slot.sh','build_release.sh','metadata.json']:
        dest=CACHE/'raw_named_sources'/str(p).lstrip('/');dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(raw)
for x in packet('EXTERNAL_INPUT_HASH_CHECK.json'):
    p=pathlib.Path(x['path']);raw=p.read_bytes();got=hashlib.sha256(raw).hexdigest()
    results['raw_hashes'].append({'path':str(p),'sha256':got,'expected':x['sha256'],'bytes':len(raw),'ok':got==x['sha256'] and len(raw)==x['bytes']})
for path in ['AGENTS.md','agents/AGENT_TASK.md','projects/chirality-piping/AGENTS.md',R+'/BRIEFS/COMMON.md',R+'/BRIEFS/RV34_HISTORICAL_REPLAY_01.md']:
    b=get(path,'5853ad20f8cd7a20156882f75fcdfbabd9edcef7')
    results['instruction_basis'].append({'path':path,'revision':'5853ad20f8cd7a20156882f75fcdfbabd9edcef7','sha256':hashlib.sha256(b).hexdigest()})
    if not path.endswith('RV34_HISTORICAL_REPLAY_01.md'):
        results['instruction_basis'][-1]['matches_review_basis']=b==get(path)
results['summary']={'seals':len(results['component_seals']),'payloads':sum(len(x['entries']) for x in results['component_seals']),'historical_inputs':len(results['historical_inputs']),'raw_named_inputs':len(results['raw_hashes']),'failures':[x for s in results['component_seals'] for x in s['entries'] if not x['ok']]+[x for name in ['historical_inputs','raw_hashes'] for x in results[name] if not x['ok']],'seal_failures':[x['packet'] for x in results['component_seals'] if not x['seal_matches_I25'] or not x['count_matches_I25']],'completed_at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
save('PROVENANCE_CHECK.json',results);print(json.dumps(results['summary'],indent=2))
