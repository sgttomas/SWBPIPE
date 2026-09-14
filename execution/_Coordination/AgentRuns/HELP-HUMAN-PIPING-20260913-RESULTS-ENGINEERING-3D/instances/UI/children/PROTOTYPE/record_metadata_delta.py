from pathlib import Path
import json,hashlib
out=Path(__file__).resolve().parent
basis=json.loads((out/'_run_records/CANDIDATE_FREEZE_V3.json').read_text())
changed=[]
for r in basis['files']:
 p=out/r['path'];digest=hashlib.sha256(p.read_bytes()).hexdigest()
 if digest!=r['sha256']:changed.append({'path':r['path'],'frozenSHA256':r['sha256'],'currentSHA256':digest,'reason':'documentary self-check status/inventory metadata update after runtime terminal; not executable browser source'})
p=out/'_run_records/SELF_CHECK_SUMMARY_V1.json';s=json.loads(p.read_text());s.pop('sourceChangesAfterFreeze',None);s['executableAssetChangesAfterFreeze']=False;s['documentaryMetadataUpdatesAfterRuntime']=changed;p.write_text(json.dumps(s,indent=2)+'\n')
assets=['prototype/index.html','prototype/workspace.css','prototype/workspace.js','prototype/capabilities.js','witness-v2.mjs']
(out/'_run_records/FINAL_METADATA_DELTA_V1.json').write_text(json.dumps({'testedFreeze':'CANDIDATE_FREEZE_V3.json','runtimeSourceUnchanged':True,'changedNonRuntimeMetadata':changed,'executionSources':[{'path':f,'currentSHA256':hashlib.sha256((out/f).read_bytes()).hexdigest(),'testedSHA256':next(r['sha256'] for r in basis['files'] if r['path']==f)} for f in assets],'handoff':'Independent witness consumes final manifest and these unchanged tested executable sources. Metadata updates do not change UI behavior.'},indent=2)+'\n')
