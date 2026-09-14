from pathlib import Path
import json,hashlib
out=Path(__file__).resolve().parent
f=out/'_run_records/CANDIDATE_FREEZE_V2.json';basis=json.loads(f.read_text())
items=[]
for row in basis['files']:
 p=out/row['path'];b=p.read_bytes();digest=hashlib.sha256(b).hexdigest()
 items.append({'path':row['path'],'preSha256':row['sha256'],'postSha256':digest,'bytes':len(b),'unchanged':digest==row['sha256']})
(out/'_run_records/LEASE_01_POST_HASHES.json').write_text(json.dumps({'lease':'ROOT_LEASE_UI_PROTOTYPE_01','candidateFreezeSHA256':hashlib.sha256(f.read_bytes()).hexdigest(),'files':items,'status':'syntax_checks_pass_browser_launch_blocked_no_repairs'},indent=2)+'\n')
node=Path('/Users/ryan/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin/node')
package=Path('/Users/ryan/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright/package.json')
identity={'node':{'path':str(node),'version':'v24.19.0','versionEvidence':'04-witness-launch-failure.json actual stderr','sha256':hashlib.sha256(node.read_bytes()).hexdigest()},'playwright':{'path':str(package),'version':json.loads(package.read_text())['version'],'sha256':hashlib.sha256(package.read_bytes()).hexdigest()},'browser':{'status':'not launched','expectedExecutable':'/Users/ryan/Library/Caches/ms-playwright/chromium_headless_shell-1234/chrome-headless-shell-mac-arm64/chrome-headless-shell','actualVersion':None}}
(out/'_run_records/RUNTIME_IDENTITY_LEASE_01.json').write_text(json.dumps(identity,indent=2)+'\n')
