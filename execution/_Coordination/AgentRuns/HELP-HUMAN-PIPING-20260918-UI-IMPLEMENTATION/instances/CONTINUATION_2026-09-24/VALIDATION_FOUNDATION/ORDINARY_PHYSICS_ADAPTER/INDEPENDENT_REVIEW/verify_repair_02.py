from pathlib import Path
import json,hashlib,re,subprocess
O=Path(__file__).parent;A=O.parent;ROOT=next(p for p in A.parents if (p/'agents/AGENT_TASK.md').is_file());P=ROOT/'projects/chirality-piping'
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def apply(text, bases, prefix='b/'):
 result={}
 for ch in [x for x in re.split(r'(?m)(?=^--- (?:a/|/dev/null|freeze01/))',text) if x.strip()]:
  lines=ch.splitlines(keepends=True);path=lines[1].strip().removeprefix('+++ '+prefix)
  if prefix=='freeze02/':path='projects/chirality-piping/'+path
  old=bases[path].decode().splitlines(keepends=True);out=[];pos=0;i=0
  while i<len(lines):
   m=re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@',lines[i])
   if not m:i+=1;continue
   at=max(0,int(m[1])-1);out.extend(old[pos:at]);pos=at;i+=1
   while i<len(lines) and not lines[i].startswith('@@ '):
    l=lines[i]
    if l.startswith(' '):assert old[pos]==l[1:];out.append(l[1:]);pos+=1
    elif l.startswith('-'):assert old[pos]==l[1:];pos+=1
    elif l.startswith('+'):out.append(l[1:])
    elif l.strip():raise AssertionError((path,l))
    i+=1
  out.extend(old[pos:]);result[path]=''.join(out).encode()
 return result
f1=read(A/'SOURCE_FREEZE_01.json');f2=read(A/'SOURCE_FREEZE_02.json');assert sha(A/'SOURCE_FREEZE_02.json')=='609ed22c4204465712d773d2f881a0971612c8c4c03d07a094b09c16a0e5d951'
assert sha(ROOT/f2['repair_patch']['path'])=='664a27851dd4bb0eed3b978a6f845a79423104e70efbf95b777987aa689d3c16';assert sha(A/'REPAIR_RETURN_02.md')=='830f77cbf1085fe84282348ae681c8b93fd0689de7b2176d9aeee5345970e38c'
base={r['path']:subprocess.check_output(['git','show',f1['base_commit']+':'+r['path']],cwd=ROOT) if r['base_sha256'] else b'' for r in f1['files']}
old=apply((ROOT/f1['patch']['path']).read_text(),base);new=apply((ROOT/f2['patch']['path']).read_text(),base);delta=apply((ROOT/f2['repair_patch']['path']).read_text(),old,'freeze02/')
assert len(old)==len(new)==11 and len(delta)==4 and set(delta)==set(f2['changed_paths'])
for r in f1['files']:assert hashlib.sha256(old[r['path']]).hexdigest()==r['sha256']
for r in f2['files']:
 path=r['path'];assert hashlib.sha256(new[path]).hexdigest()==r['sha256']==sha(ROOT/path)
 assert new[path]==(delta[path] if path in delta else old[path])
assert len([p for p in old if old[p]==new[p]])==7
for r in f2['evidence']:assert sha(ROOT/r['path'])==r['sha256']
focused=read(A/'FOCUSED_CHECKS_02.json');assert focused['source_unchanged'] and focused['before']==focused['after']
for r in focused['before']:assert sha(P/r['path'])==r['sha256']
counts=[]
for check in focused['checks']:
 assert sha(A/check['log'])==check['log_sha256'] and check['exit_code']==0
 counts.append(int(re.search(r'Ran (\d+) tests?',(A/check['log']).read_text())[1]))
assert counts==[73,3]
for name in ['FOCUSED_BOUNDARIES.json','RETURN.md','FINDINGS.md','FINAL_CHECKED_BASIS.json']:assert (O/name).is_file()
original_review=read(O/'FINAL_CHECKED_BASIS.json')
for r in original_review['outputs']:assert sha(Path(r['origin']))==r['sha256']
package=A.parent/'FIRST_STATIC_BINDINGS';pm=read(package/'PACKET_MANIFEST.json')
assert len(pm['files'])==25 and all(sha(package/r['path'])==r['sha256'] for r in pm['files'])
records={'status':'PASS frozen four-file repair and seven unchanged source paths','freeze02_sha256':sha(A/'SOURCE_FREEZE_02.json'),'repair_patch_sha256':sha(ROOT/f2['repair_patch']['path']),'full_patch_sha256':sha(ROOT/f2['patch']['path']),'files':f2['files'],'changed_paths':f2['changed_paths'],'old_and_new_full_patch_reconstruction':True,'repair_postimages_equal_full_freeze02':True,'unchanged_paths':7,'owner_focused_methods':counts,'original_negative_and_full_review_hashes_unchanged':True,'original25_packet_unchanged':True,'no_actual_case_execution':True}
(O/'REPAIR_SOURCE_CHECKS_02.json').write_text(json.dumps(records,indent=2)+'\n');print(json.dumps({k:v for k,v in records.items() if k!='files'},indent=2))
