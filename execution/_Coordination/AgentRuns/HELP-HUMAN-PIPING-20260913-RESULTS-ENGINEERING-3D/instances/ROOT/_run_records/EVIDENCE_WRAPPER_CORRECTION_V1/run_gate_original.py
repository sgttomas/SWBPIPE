import datetime, hashlib, json, os, pathlib, subprocess, sys, time

label, cwd, *argv = sys.argv[1:]
root = pathlib.Path('/tmp/chirality-results-engineering-3d-20260913/root-gates')
out = root / label
out.mkdir(exist_ok=False)
env = dict(os.environ)
env.update(CARGO_NET_OFFLINE='true', CARGO_TARGET_DIR='/tmp/chirality-results-engineering-3d-20260913/root-integration-target', PLAYWRIGHT_WORKERS='1', CI='true', PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD='1', PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH='/Users/ryan/Library/Caches/ms-playwright/chromium_headless_shell-1223/chrome-headless-shell-mac-arm64/chrome-headless-shell')
env['PATH']='/Users/ryan/.local/share/mise/installs/node/24/bin:/Users/ryan/.cargo/bin:' + env['PATH']
git = lambda *args: subprocess.check_output(['git', *args], cwd=cwd, text=True)
cmd = {'label':label,'cwd':cwd,'argv':argv,'head':git('rev-parse','HEAD').strip(),'status_before':git('status','--porcelain=v1'),'env':{k:env[k] for k in ['PATH','CARGO_NET_OFFLINE','CARGO_TARGET_DIR','PLAYWRIGHT_WORKERS','CI','PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD','PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH']},'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
(out/'COMMAND.json').write_text(json.dumps(cmd,indent=2)+'\n')
start=time.monotonic()
with (out/'stdout.log').open('wb') as stdout, (out/'stderr.log').open('wb') as stderr:
    result=subprocess.run(argv,cwd=cwd,env=env,stdout=stdout,stderr=stderr)
files=[{'path':p.name,'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in out.iterdir()]
record={'exit_code':result.returncode,'elapsed_seconds':time.monotonic()-start,'finished_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'files':files,'status_after':git('status','--porcelain=v1')}
(out/'RETURN.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({'label':label,'exit_code':result.returncode,'elapsed_seconds':record['elapsed_seconds'],'output':str(out)}),flush=True)
sys.exit(result.returncode)
