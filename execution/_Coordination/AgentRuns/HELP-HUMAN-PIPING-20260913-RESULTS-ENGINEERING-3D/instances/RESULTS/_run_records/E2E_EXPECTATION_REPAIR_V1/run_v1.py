import pathlib,json,hashlib,subprocess,sys,os,datetime,shutil,plistlib
E=pathlib.Path(__file__).parent;P=pathlib.Path('/Users/ryan/.codex/worktrees/8728/chirality-results-integrity-20260913/projects/chirality-piping');D=P/'apps/desktop';B=json.loads((E/'MANAGER_PRE_DISPATCH_BASELINE_V1.json').read_text());PRE=json.loads((E/'PRE_EFFECT_V1.json').read_text())
CHROME=pathlib.Path('/Users/ryan/Library/Caches/ms-playwright/chromium-1223/chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing')
def sha(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
env=os.environ.copy();env.update({'CARGO_NET_OFFLINE':'true','CARGO_TARGET_DIR':'/tmp/chirality-results-engineering-3d-20260913/results-e2e-expectation-repair-target','PLAYWRIGHT_WORKERS':'1','PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD':'1','PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH':str(CHROME),'CI':'1'})
assert CHROME.is_file()
paths=[x['path'] for x in B['accepted42']+B['all_e2e_before']+PRE['input_files']]
for x in B['accepted42']:assert sha(P/x['path'])==x['sha256']
argv=sys.argv[2:];d=E/sys.argv[1];d.mkdir()
cmd={'argv':argv,'cwd':str(D),'env_overrides':{k:env[k] for k in ['CARGO_NET_OFFLINE','CARGO_TARGET_DIR','PLAYWRIGHT_WORKERS','PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD','PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH','CI']},'tool_paths':{k:shutil.which(k) for k in ['node','npm','cargo','rustc','wasm-bindgen']},'cached_chromium':{'executable':str(CHROME),'sha256':sha(CHROME),'info':plistlib.loads((CHROME.parents[1]/'Info.plist').read_bytes()).get('CFBundleShortVersionString')},'source_input_hashes':{x:sha(P/x) for x in sorted(set(paths))},'runner_sha256':sha(pathlib.Path(__file__)),'prepared_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
(d/'COMMAND.json').write_text(json.dumps(cmd,indent=2)+'\n')
with (d/'stdout.log').open('wb') as out,(d/'stderr.log').open('wb') as err:r=subprocess.run(argv,cwd=D,env=env,stdout=out,stderr=err)
result={'exit_code':r.returncode,'stdout_sha256':sha(d/'stdout.log'),'stderr_sha256':sha(d/'stderr.log'),'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()};(d/'RESULT.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result));print((d/'stdout.log').read_text()[-6000:]);print((d/'stderr.log').read_text()[-3000:]);sys.exit(r.returncode)
