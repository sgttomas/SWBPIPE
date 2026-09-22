#!/usr/bin/env python3
"""Freeze or verify the Piping acceptance/finalization delta without Git mutations."""
import argparse,csv,hashlib,json,subprocess
from pathlib import Path
POST=Path(__file__).resolve().parent;ROOT=POST
while not (ROOT/'tools/scope_of_work/validate_scope_of_work.py').exists():ROOT=ROOT.parent
MANIFEST=POST/'FINAL_REVIEW_MANIFEST.csv';BASE='d6cc1482eee78ce860ff18658f11157f7efbd401'
parser=argparse.ArgumentParser();parser.add_argument('--check',action='store_true');a=parser.parse_args()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
if a.check:
 rows=list(csv.DictReader(MANIFEST.open()));errors=[r['Path']for r in rows if not(ROOT/r['Path']).is_file()or sha(ROOT/r['Path'])!=r['SHA256']]
 print(json.dumps({'status':'FAIL'if errors else'PASS','files':len(rows),'mismatches':errors},indent=2));raise SystemExit(bool(errors))
paths=set()
for args in [['git','diff','--name-only','-z',BASE,'--','projects/chirality-piping'],['git','ls-files','--others','--exclude-standard','-z','--','projects/chirality-piping']]:paths.update(x.decode()for x in subprocess.check_output(args,cwd=ROOT).split(b'\0')if x)
paths.update(x['target']for x in json.loads((POST/'FINAL_TARGET_MANIFEST.json').read_text())['files'])
excluded=[str(MANIFEST.relative_to(ROOT)),str((POST/'INDEPENDENT_FINALIZATION_REVIEW.md').relative_to(ROOT)),str((POST/'review').relative_to(ROOT))+'/','__pycache__']
rows=[]
for name in sorted(paths):
 if any(x in name for x in excluded):continue
 p=ROOT/name
 if not p.is_file():raise RuntimeError('Missing final file '+name)
 rows.append({'Path':name,'SHA256':sha(p),'Size':p.stat().st_size})
with MANIFEST.open('w',newline='')as f:
 w=csv.DictWriter(f,fieldnames=['Path','SHA256','Size']);w.writeheader();w.writerows(rows)
print(json.dumps({'status':'FROZEN','files':len(rows),'manifest_sha256':sha(MANIFEST)},indent=2))
