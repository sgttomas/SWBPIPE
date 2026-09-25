"""Independent one-packet transport probe; no solve/build/native execution."""
from pathlib import Path
import sys,json,hashlib,struct
HERE=Path(__file__).resolve().parent
PROJECT=next(p for p in HERE.parents if (p/'core/analysis_runs/physics_source.py').is_file())
sys.path[:0]=[str(PROJECT),str(PROJECT/'tests')]
from test_stress_neutral_physics_source import captured
from core.handoff.stress_neutral import package_v0_3 as sn
bits=lambda x:struct.pack('>d',float(x)).hex()
path=PROJECT/'fixtures/product_preview/physics_source/mixed-sparse_interactive.raw.json'
raw,analysis,package=captured(path)
members=sn.materialized_members_v0_3(package)
reopened=sn.reconstruct_materialized_members_v0_3(members,source_envelope=raw,analysis_record=analysis)
changed=[]
for before,after in zip(package['source_annotations'],reopened['source_annotations']):
    if bits(before['source_row']['value'])!=bits(after['source_row']['value']):
        changed.append({'id':before['source_result_id'],'before_bits':bits(before['source_row']['value']),'after_bits':bits(after['source_row']['value'])})
report={'scope':'Actual retained mixed producer source; pure nine-member projection/readback only, no current/native proof.',
        'source_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
        'package_code_sha256':hashlib.sha256((PROJECT/'core/handoff/stress_neutral/package_v0_3.py').read_bytes()).hexdigest(),
        'original_source_validation_and_package_readback':'passed','changed_source_value_bits':changed,
        'members':{name:hashlib.sha256(data).hexdigest() for name,data in members.items()}}
(HERE/'STRESS_TRANSPORT_PROBE.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'changed_source_value_count':len(changed),'samples':changed[:3],'readback_validation':'passed'},indent=2))
