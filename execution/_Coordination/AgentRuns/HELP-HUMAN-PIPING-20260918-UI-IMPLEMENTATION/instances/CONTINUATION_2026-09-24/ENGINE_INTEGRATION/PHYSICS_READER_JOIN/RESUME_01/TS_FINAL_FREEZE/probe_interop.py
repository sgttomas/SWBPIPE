from pathlib import Path
import sys,json,hashlib,struct
HERE=Path(__file__).resolve().parent
PROJECT=next(p for p in HERE.parents if (p/'core/analysis_runs/compatibility.py').is_file())
sys.path[:0]=[str(PROJECT),str(PROJECT/'tests')]
from core.handoff.stress_neutral import package_v0_3 as sn
from schema_validation import validate_instance
ARTIFACTS=Path('/private/tmp/piping-composite-ts-artifacts-resume1')
DEST=Path('/private/tmp/piping-composite-ts-python-members-final')
def read(path):return json.loads(path.read_text(),parse_int=lambda s:-0.0 if s=='-0' else int(s))
def schema(name,value):validate_instance(read(PROJECT/'schemas'/name),value,instance_label='actual producer through TS consumer')
records=[]
for path in sorted(ARTIFACTS.glob('*.packet.json')):
 stem=path.name.removesuffix('.packet.json'); packet=read(path);source=read(ARTIFACTS/f'{stem}.raw.json');analysis=read(ARTIFACTS/f'{stem}.analysis.json');document=read(ARTIFACTS/f'{stem}.document.json')
 schema('stress_neutral_export.v0.3.schema.json',packet)
 schema('analysis_run.v0.3.schema.json',analysis)
 schema('results.v0.3.schema.yaml',document)
 sn.validate_stress_neutral_export_package_v0_3(packet,source_envelope=source,analysis_record=analysis)
 sn.write_materialized_members_v0_3(packet,DEST/stem)
 reopened=sn.read_materialized_members_v0_3(DEST/stem,source_envelope=source,analysis_record=analysis)
 assert reopened==packet
 assert len(list((DEST/stem).iterdir()))==9
 assert (DEST/stem/'stress_neutral_results.csv').read_bytes()==packet['csv_text'].encode('utf-8')
 for annotation,row in zip(reopened['source_annotations'],source['results']):assert annotation['source_value_bits']==struct.pack('>d',float(row['value'])).hex()
 records.append({'stem':stem,'rows':len(source['results']),'source_sha256':hashlib.sha256((ARTIFACTS/f'{stem}.raw.json').read_bytes()).hexdigest(),'packet_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'member_hashes':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((DEST/stem).iterdir())}})
assert len(records)==14,len(records)
headless=[]
for path in sorted(Path('/private/tmp/piping-headless-composite-artifacts-resume1').glob('*.document.json')):
 schema('results.v0.3.schema.yaml',read(path));headless.append({'path':str(path),'sha256':hashlib.sha256(path.read_bytes()).hexdigest()})
assert len(headless)==8,len(headless)
(HERE/'INTEROP_RESULT.json').write_text(json.dumps({'scope':'Actual producer captures through TS consumers then Python schema and nine-member transport; no native UI witness','ts_pairs':records,'actual_headless_canonical':headless},indent=2)+'\n')
print(f'PASS {len(records)} TS pairs, {len(records)*9} materialized members, 42 TS artifact schema checks, {len(headless)} actual headless canonical schemas; all source row bit sidecars retained')
