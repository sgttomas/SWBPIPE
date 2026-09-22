#!/usr/bin/env python3
"""Apply reviewed local Group-3 metadata transforms after actual owner acceptance."""
import argparse,hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[7]
POST=Path(__file__).resolve().parents[1];OUT=POST/'dependencies'
SCA=ROOT/'projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP'
TRANSFORM=SCA/'application/dependencies/REPAIRED_GROUP3_PROMOTION.json'
def sha(data):return hashlib.sha256(data).hexdigest()
def rel(p):return str(p.relative_to(ROOT))
def main():
 parser=argparse.ArgumentParser();parser.add_argument('--apply',action='store_true');args=parser.parse_args()
 decision=POST/'OWNER_DECISION.md';assert '> Accept the audited result and adopt DAG-011' in decision.read_text()
 transforms=json.loads(TRANSFORM.read_text())['group3_files'];assert len(transforms)==20
 prepared=[];report=[]
 for row in transforms:
  path=ROOT/row['target'];before=path.read_bytes()
  if args.apply:
   assert sha(before)==row['applied_sha256'],str(path)
   after=before
   for op in row['operations']:
    assert op['operation']=='replace_exact';old=op['old'].encode();assert after.count(old)==op['expected_occurrences'];after=after.replace(old,op['new'].encode())
  else:
   assert sha(before)==row['accepted_sha256'],str(path);after=before
  assert sha(after)==row['accepted_sha256']
  if path.suffix=='.csv':assert row['applied_sha256']==row['accepted_sha256'] and not row['operations']
  prepared.append((path,before,after));report.append({'CanonicalTarget':row['target'],'ReviewedSHA256':row['applied_sha256'],'AcceptedSHA256':row['accepted_sha256'],'OperationCount':len(row['operations']),'BytesChanged':row['applied_sha256']!=row['accepted_sha256']})
 if args.apply:
  for path,before,after in prepared:
   backup=OUT/'preacceptance'/path.relative_to(ROOT);backup.parent.mkdir(parents=True,exist_ok=True);assert not backup.exists();backup.write_bytes(before)
  for path,before,after in prepared:
   if before!=after:path.write_bytes(after)
 for path,before,after in prepared:assert path.read_bytes()==after
 result={'status':'PASS','owner_decision':{'path':rel(decision),'sha256':sha(decision.read_bytes())},'reviewed_commit':'d6cc1482eee78ce860ff18658f11157f7efbd401','transform':{'path':rel(TRANSFORM),'sha256':sha(TRANSFORM.read_bytes())},'targets':report,'summary':{'targets':20,'changed_indexes':10,'unchanged_csvs':10},'limits':'Authority metadata only; no dependency semantic bytes, satisfaction, lifecycle or product changes.'}
 (OUT/'LOCAL_ACCEPTANCE_APPLICATION.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result['summary']))
if __name__=='__main__':main()
