#!/usr/bin/env python3
"""Freeze or check Piping applied review files, excluding review outputs/self manifests."""
import argparse,csv,hashlib,json,subprocess
from pathlib import Path
RUN=Path(__file__).resolve().parents[1]
ROOT=RUN
while not (ROOT/'tools/scope_of_work/validate_scope_of_work.py').exists(): ROOT=ROOT.parent
BASE='3e18334eca72509475684cc86786b3eeade83572'
MANIFEST=RUN/'application/APPLIED_REVIEW_MANIFEST.csv'
parser=argparse.ArgumentParser();parser.add_argument('--check',action='store_true');a=parser.parse_args()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
if a.check:
 rows=list(csv.DictReader(MANIFEST.open()));errors=[r['Path'] for r in rows if not(ROOT/r['Path']).exists()or sha(ROOT/r['Path'])!=r['SHA256']]
 print(json.dumps({'status':'FAIL'if errors else'PASS','files':len(rows),'mismatches':errors},indent=2));raise SystemExit(bool(errors))
paths=set()
for args in [['git','diff','--name-only','-z',BASE,'--','projects/chirality-piping'],['git','ls-files','--others','--exclude-standard','-z','--','projects/chirality-piping']]:
 paths.update(x.decode()for x in subprocess.check_output(args,cwd=ROOT).split(b'\0')if x)
paths.update(r['CanonicalTarget']for r in csv.DictReader((RUN/'APPLY_MANIFEST.csv').open()))
exclude=['/application/APPLIED_REVIEW_MANIFEST.csv','/application/INDEPENDENT_POSTSTATE_REVIEW.md','/application/review/','__pycache__']
rows=[]
for name in sorted(paths):
 if any(x in name for x in exclude):continue
 p=ROOT/name
 if not p.is_file():raise RuntimeError('Missing frozen file '+name)
 rows.append({'Path':name,'SHA256':sha(p),'Size':p.stat().st_size})
with MANIFEST.open('w',newline='')as f:
 w=csv.DictWriter(f,fieldnames=['Path','SHA256','Size']);w.writeheader();w.writerows(rows)
print(json.dumps({'status':'FROZEN','files':len(rows),'manifest_sha256':sha(MANIFEST)},indent=2))
