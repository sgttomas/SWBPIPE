from pathlib import Path
import subprocess as s, json, hashlib as h, os, datetime, shutil
r=Path(s.check_output(['git','rev-parse','--show-toplevel'],text=True).strip()); out=Path(__file__).resolve().parent; app=r/'projects/chirality-piping/apps/desktop'; prior=out.parent/'NATIVE_ARTIFACT_1C/TASK/_run_records'; target=Path('/private/tmp/piping-native-pan-20260924')
def sha(p): return h.sha256(p.read_bytes()).hexdigest()
def write(n,d): (out/n).write_text(json.dumps(d,indent=2)+'\n')
def manifest(p): return [{'path':str(f.relative_to(p)),'bytes':f.stat().st_size,'sha256':sha(f)} for f in sorted(p.rglob('*')) if f.is_file()]
def run(c,cwd=r):
 x=s.run(c,cwd=cwd,capture_output=True,text=True);return {'command':c,'cwd':str(cwd),'exit':x.returncode,'stdout':x.stdout,'stderr':x.stderr}
freeze=out.parent/'NATIVE_PAN_REPAIR/TASK/FREEZE.json'; fr=json.loads(freeze.read_text()); head=s.check_output(['git','rev-parse','HEAD'],text=True).strip(); assert head==fr['head']
for f in fr['changed']: assert sha(r/f['path'])==f['sha256'],f
paths=[x['path'] for x in json.loads((prior/'product-inputs-preflight.json').read_text())]
paths=sorted(set(paths)|set(s.check_output(['git','ls-files','projects/chirality-piping/apps/desktop','projects/chirality-piping/core','projects/chirality-piping/fixtures'],text=True).splitlines()))
def inputs(): return [{'path':p,'sha256':sha(r/p)} for p in paths if (r/p).is_file()]
pre=inputs();write('inputs-pre.json',pre)
diff=s.check_output(['git','diff',head,'--']+[f['path'] for f in fr['changed']]);(out/'source.diff').write_bytes(diff)
assert h.sha256(diff).hexdigest()=='94ee4d02cc05bcba668f665575310e86af7e912addfa61257adaea81d33f3a9b'
changed=s.check_output(['git','diff','--name-only',head,'--']+paths,text=True).splitlines(); assert set(changed)==set(f['path'] for f in fr['changed']),changed
(out/'git-status-pre.txt').write_bytes(s.check_output(['git','status','--short']))
assert not target.exists()
oldroots=[Path('/private/tmp/piping-native-fit-1c-20260924/debug/bundle/macos/SWBPIPE.app'),app/'src-tauri/target/debug/bundle/macos/SWBPIPE.app'];old={str(p):manifest(p) for p in oldroots};write('old-bundles-pre.json',old)
write('basis.json',{'head':head,'freeze_path':str(freeze.relative_to(r)),'freeze_sha256':sha(freeze),'diff_sha256':h.sha256(diff).hexdigest(),'executor':'/root/native_pan_build','parent':'/root','role':'TASK Type2','mechanism':'delegated-harness-native','instructions':[{'path':p,'sha256':sha(r/p)} for p in ['AGENTS.md','agents/AGENT_TASK.md','projects/chirality-piping/AGENTS.md','projects/chirality-piping/loop/LOOP_INIT.md']],'prior_evidence':str(prior.relative_to(r)),'dirty_checkout':True})
write('tools.json',[run(c,app if c[0]=='npm' else r) for c in [['node','--version'],['npm','--version'],['rustc','-Vv'],['cargo','--version'],['rustup','target','list','--installed'],['wasm-bindgen','--version'],['npm','run','tauri','--','--version'],['sw_vers'],['uname','-m']]])
env=os.environ.copy();env.pop('SWBPIPE_LIVE_CONTROL',None);env.update(CARGO_TARGET_DIR=str(target),CARGO_BUILD_JOBS='2',CARGO_NET_OFFLINE='true',npm_config_offline='true')
cmd=['npm','run','tauri','--','build','--debug','--bundles','app']; rec={'command':cmd,'cwd':str(app),'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'environment':{k:env.get(k) for k in ['CARGO_TARGET_DIR','CARGO_BUILD_JOBS','CARGO_NET_OFFLINE','npm_config_offline','SWBPIPE_LIVE_CONTROL','RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','NODE_ENV','SDKROOT','MACOSX_DEPLOYMENT_TARGET','PATH']},'target_absent_before':True,'host_approval':'default sandbox; no escalation requested'};write('build-command.json',rec)
with (out/'build.log').open('w') as log: result=s.run(cmd,cwd=app,env=env,stdout=log,stderr=s.STDOUT)
rec.update(exit=result.returncode,finished=datetime.datetime.now(datetime.timezone.utc).isoformat());write('build-command.json',rec)
post=inputs();write('inputs-post.json',post);(out/'git-status-post.txt').write_bytes(s.check_output(['git','status','--short']))
check={'maintained_inputs_unchanged':pre==post,'input_count':len(pre),'head_post':s.check_output(['git','rev-parse','HEAD'],text=True).strip(),'old_bundles_unchanged':old=={str(p):manifest(p) for p in oldroots}};write('post-check.json',check)
if result.returncode==0:
 bundle=target/'debug/bundle/macos/SWBPIPE.app'; exe=bundle/'Contents/MacOS/openpipestress-desktop';write('artifact-identity.json',{'app':str(bundle),'executable':str(exe),'executable_sha256':sha(exe),'bundle_files':manifest(bundle),'dist_files':manifest(app/'dist'),'generated_public':{x:manifest(app/'public'/x) for x in ['wasm-engine','self-weight-engine']}})
print(json.dumps({'build':rec,'check':check}))
