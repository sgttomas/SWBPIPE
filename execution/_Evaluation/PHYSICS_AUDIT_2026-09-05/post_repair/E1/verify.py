from pathlib import Path
import subprocess,json,hashlib,re,tempfile,difflib
root=Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],text=True).strip())
audit=root/'projects/chirality-piping/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05'
out=audit/'post_repair/E1'
def run(args,cwd=root,data=None):
 p=subprocess.run(args,cwd=cwd,input=data,capture_output=True);return {'command':args,'exit':p.returncode,'stdout':p.stdout.decode(),'stderr':p.stderr.decode()}
def sha(b):return hashlib.sha256(b).hexdigest()
def save(name,value):(out/name).write_text(json.dumps(value,indent=2)+'\n')
inv=json.loads(Path('/tmp/piping-audit-crlf-hazard.json').read_text()); paths=inv['paths']; assert len(paths)==len(set(paths))==67
log=Path('/tmp/piping-audit-staged-whitespace.txt').read_bytes()
reported=set(m.decode() for m in re.findall(rb'^(projects/[^\r\n]+):\d+: trailing whitespace\.$',log,re.M))
assert reported==set(paths),(len(reported),set(paths)^reported)
actual={str(p.relative_to(root)) for p in audit.rglob('*.csv') if b'\r\n' in p.read_bytes()};assert actual==set(paths),(len(actual),actual^set(paths))
config=run(['git','config','--show-origin','--get-all','core.whitespace']);assert config['exit']==1
attrs=run(['git','check-attr','whitespace','--stdin'],data=('\n'.join(paths)+'\n').encode());assert all(x.endswith((': unspecified', ': blank-at-eol,blank-at-eof,space-before-tab,cr-at-eol')) for x in attrs['stdout'].splitlines())
records=[]
for name in paths:
 p=root/name; b=p.read_bytes(); assert p.suffix=='.csv' and b'\r\n' in b
 assert b.count(b'\r')==b.count(b'\r\n')==b.count(b'\n')
 lines=b.split(b'\r\n'); assert lines[-1]==b'' and lines[-2]!=b''
 assert not any(line.endswith((b' ',b'\t')) or re.match(rb'^[ \t]* \t',line) for line in lines)
 idx=subprocess.check_output(['git','show',':'+name],cwd=root);assert idx==b
 records.append({'path':name,'bytes':len(b),'crlf_lines':len(lines)-1,'sha256_before':sha(b),'index_sha256':sha(idx)})
if not (out/'BEFORE.json').exists(): save('BEFORE.json',{'inventory':inv,'config':config,'attributes':attrs,'records':records,'reported_paths_equal_inventory':True,'all_actual_crlf_csv_equal_inventory':True})
attribute=audit/'.gitattributes'
rule='whitespace=blank-at-eol,blank-at-eof,space-before-tab,cr-at-eol'
content='# Exact frozen audit CSV paths: recognize CRLF while retaining every enabled whitespace check.\n# No line-ending conversion or whitespace exemption; future CSV paths require separate review.\n'+''.join(str((root/name).relative_to(audit))+' '+rule+'\n' for name in paths)
attribute.write_text(content)
(out/'ATTRIBUTES_DIFF.txt').write_text(''.join(difflib.unified_diff([],content.splitlines(True),fromfile='/dev/null',tofile=str(attribute.relative_to(root)))))
checks={'real_cached_check':run(['git','diff','--cached','--check']),'attributes_after':run(['git','check-attr','whitespace','--stdin'],data=('\n'.join(paths)+'\n').encode())}
assert checks['real_cached_check']['stdout']=='projects/chirality-piping/execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/V1/baseline/cargo.log:1757: new blank line at EOF.\n',checks['real_cached_check']
toy=Path(tempfile.mkdtemp(prefix='piping-physics-crlf-'))
assert run(['git','init','-q'],toy)['exit']==0
(toy/'.gitattributes').write_text('sample.csv '+rule+'\n')
assert run(['git','add','.gitattributes'],toy)['exit']==0
checks['toy_path']=str(toy);checks['toy_cases']={}
for label,b,expected in [('valid_crlf',b'a,b\r\n1,2\r\n',0),('space_before_crlf',b'a,b \r\n',2),('tab_before_crlf',b'a,b\t\r\n',2),('blank_at_eof',b'a,b\r\n\r\n',2),('space_before_tab',b' \ta,b\r\n',2)]:
 (toy/'sample.csv').write_bytes(b); assert run(['git','add','sample.csv'],toy)['exit']==0
 result=run(['git','diff','--cached','--check'],toy);assert result['exit']==expected,(label,result)
 checks['toy_cases'][label]={'input_hex':b.hex(),**result}
(toy/'sample.csv').write_bytes(b'a,b\r\n');run(['git','add','sample.csv'],toy)
(toy/'unlisted.csv').write_bytes(b'a,b\r\n');run(['git','add','unlisted.csv'],toy)
checks['toy_cases']['unlisted_crlf']=run(['git','diff','--cached','--check'],toy);assert checks['toy_cases']['unlisted_crlf']['exit']==2
for record in records:
 record['sha256_after']=sha((root/record['path']).read_bytes());assert record['sha256_after']==record['sha256_before']
save('CHECKS.json',checks);save('BYTE_PRESERVATION.json',records)
print(json.dumps({'paths':len(records),'real_cached_check':checks['real_cached_check']['exit'],'toy_cases':{k:v['exit'] for k,v in checks['toy_cases'].items()},'attribute_sha256':sha(attribute.read_bytes())}))
