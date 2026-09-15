from pathlib import Path
import json,sys,hashlib
r=json.loads(Path(sys.argv[1]).read_text())
print(sorted(r))
for k in ['model_json','mechanics_result_json','analysis_run_json','model_hash_json','project_envelope_hash_json','editor_intents_json','proposal_json','selected_review_target_json','model_migration_ledger_json']:
    s=r[k]
    v=json.loads(s)
    shape=sorted(v)[:20] if isinstance(v,dict) else len(v) if isinstance(v,list) else v
    print(k, len(s), hashlib.sha256(s.encode()).hexdigest(), type(v).__name__, shape)
    if k=='analysis_run_json':
        a=v['analysis_run']
        print('analysis schema',a.get('schema_version'),'run',a.get('run_id'),'refs',len(a.get('result_refs',[])),'hashes',a.get('hashes'))
        print('manifest',a.get('reproducibility',{}).get('input_manifest_hashes'))
    if k=='mechanics_result_json':
        print('mech result keys',sorted(v),'rows',len(v.get('result_rows',[])),'hashes',v.get('hashes'))

