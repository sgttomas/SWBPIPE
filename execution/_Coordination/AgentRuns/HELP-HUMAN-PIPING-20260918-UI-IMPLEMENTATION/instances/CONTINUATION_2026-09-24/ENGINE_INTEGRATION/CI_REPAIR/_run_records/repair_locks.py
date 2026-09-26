from pathlib import Path
import subprocess,json,os,tomllib,hashlib
out=Path('/private/tmp/piping-joined-qualification-20260925/lock-check');results=json.loads((out/'RESULTS.json').read_text());env=os.environ.copy();env['CARGO_NET_OFFLINE']='true';env['CARGO_BUILD_JOBS']='2';changes=[]
for x in results:
 if not x['exit_code']:continue
 cmd=[v for v in x['command'] if v!='--locked'];manifest=cmd[cmd.index('--manifest-path')+1];lock=Path('projects/chirality-piping')/Path(manifest).with_name('Cargo.lock');old=lock.read_bytes();before=tomllib.loads(old.decode())['package'];r=subprocess.run(cmd,cwd='projects/chirality-piping',env=env,capture_output=True,text=True);(out/(lock.parent.name+'-repair.log')).write_text(r.stdout+r.stderr);assert not r.returncode,(manifest,r.stderr)
 new=lock.read_bytes();after=tomllib.loads(new.decode())['package'];a={(p['name'],p['version'],p.get('source')):p for p in before};b={(p['name'],p['version'],p.get('source')):p for p in after};assert not a.keys()-b.keys(),('removed or changed versions',a.keys()-b.keys());checksum_changes=[k for k in a if a[k].get('checksum')!=b[k].get('checksum')];assert not checksum_changes
 changes.append({'path':str(lock),'before_sha256':hashlib.sha256(old).hexdigest(),'after_sha256':hashlib.sha256(new).hexdigest(),'packages_before':len(a),'packages_after':len(b),'added_packages':[list(k) for k in b.keys()-a.keys()],'changed_dependency_edges':[k[0] for k in a if a[k].get('dependencies')!=b[k].get('dependencies')],'removed_or_changed_package_versions':[],'checksum_changes':[]})
(out/'REPAIR.json').write_text(json.dumps(changes,indent=2)+'\n');print(json.dumps(changes,indent=2))
