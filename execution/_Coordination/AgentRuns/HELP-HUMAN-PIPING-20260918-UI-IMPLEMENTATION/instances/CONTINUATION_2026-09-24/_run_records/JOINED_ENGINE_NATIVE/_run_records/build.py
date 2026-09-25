from pathlib import Path
import subprocess as s,json,hashlib as h,os,datetime,sys
r=Path(sys.argv[1]).resolve();expected_head=sys.argv[2];out=Path(__file__).parent;app=r/'projects/chirality-piping/apps/desktop';target=app/'src-tauri/target'
def sha(p):return h.sha256(p.read_bytes()).hexdigest()
def write(n,d):(out/n).write_text(json.dumps(d,indent=2)+'\n')
def manifest(p):return [{'path':str(f.relative_to(p)),'bytes':f.stat().st_size,'sha256':sha(f)} for f in sorted(p.rglob('*')) if f.is_file()]
def git(*c):return s.check_output(['git',*c],cwd=r,text=True).strip()
head=git('rev-parse','HEAD');assert head==expected_head;assert not git('diff','--name-only','HEAD')
paths=git('ls-files','projects/chirality-piping/apps/desktop','projects/chirality-piping/core','projects/chirality-piping/schemas','projects/chirality-piping/fixtures','projects/chirality-piping/package.json','projects/chirality-piping/package-lock.json').splitlines()
def inputs():return [{'path':p,'sha256':sha(r/p)} for p in paths]
pre=inputs();write('inputs-pre.json',pre);
for command in [['node','--version'],['npm','--version'],['rustc','--version'],['cargo','--version'],['wasm-bindgen','--version']]:
 write('tool-'+command[0]+'.json',{'argv':command,'output':s.check_output(command,cwd=r,text=True).strip()})
write('basis.json',{'head':head,'status':git('status','--short'),'executor':'/root','role':'HELP_HUMAN','purpose':'Actual candidate native author/edit/review/apply/solve/persist witness preparation; no GUI execution in this command','workspace_desktop_alias':str((r/'projects/chirality-piping/node_modules/@openpipestress/desktop').resolve())})
assert (r/'projects/chirality-piping/node_modules/@openpipestress/desktop').resolve()==app
assert not (target/'debug/bundle/macos/SWBPIPE.app').exists()
oldbundle=Path('/private/tmp/piping-generated-loads-20260924/projects/chirality-piping/apps/desktop/src-tauri/target/debug/bundle/macos/SWBPIPE.app');old=manifest(oldbundle);write('old-bundle-pre.json',old)
env=os.environ.copy();env.pop('SWBPIPE_LIVE_CONTROL',None);env.pop('CARGO_TARGET_DIR',None);env.update(CARGO_BUILD_JOBS='2',CARGO_NET_OFFLINE='true',npm_config_offline='true')
cmd=['npm','run','tauri','--','build','--debug','--bundles','app'];rec={'command':cmd,'cwd':str(app),'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'environment':{k:env.get(k) for k in ['PATH','CARGO_TARGET_DIR','CARGO_BUILD_JOBS','CARGO_NET_OFFLINE','npm_config_offline','SWBPIPE_LIVE_CONTROL','RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','NODE_ENV','SDKROOT','MACOSX_DEPLOYMENT_TARGET']}};write('build-command.json',rec)
with (out/'build.log').open('w') as log:res=s.run(cmd,cwd=app,env=env,stdout=log,stderr=s.STDOUT)
rec.update(exit_code=res.returncode,finished=datetime.datetime.now(datetime.timezone.utc).isoformat());write('build-command.json',rec)
post=inputs();write('inputs-post.json',post);check={'maintained_inputs_unchanged':pre==post,'input_count':len(pre),'head_post':git('rev-parse','HEAD'),'status_post':git('status','--short'),'old_bundle_unchanged':old==manifest(oldbundle)};write('post-check.json',check)
if res.returncode==0:
 bundle=target/'debug/bundle/macos/SWBPIPE.app';exe=bundle/'Contents/MacOS/openpipestress-desktop';write('artifact-identity.json',{'app':str(bundle),'executable':str(exe),'executable_sha256':sha(exe),'bundle_files':manifest(bundle),'dist_files':manifest(app/'dist'),'generated_public':{x:manifest(app/'public'/x) for x in ['wasm-engine','self-weight-engine']}})
print(json.dumps({'result':rec,'check':check}));assert pre==post and old==manifest(oldbundle);raise SystemExit(res.returncode)
