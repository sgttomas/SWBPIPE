from pathlib import Path
import subprocess,json,os,hashlib,sys
r=Path(subprocess.check_output(["git","rev-parse","--show-toplevel"],text=True).strip())
entry=Path(__file__).resolve().parent
e=entry.parent
run_name=sys.argv[2]
if not run_name or any(c not in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_" for c in run_name): raise SystemExit("Use a simple unused run name")
out=e/"_run_records/BROWSER/replays"/run_name
out.mkdir(parents=True,exist_ok=False)
d=r/"projects/chirality-piping/apps/desktop"
lane=sys.argv[1]
executables={"chrome153": "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome", "chromium148": str(Path.home()/"Library/Caches/ms-playwright/chromium-1223/chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing")}
executable=executables[lane]
check=subprocess.run(["lsof","-nP","-iTCP:5174","-iTCP:5175","-sTCP:LISTEN"],capture_output=True,text=True)
(out/(lane+"-ports-before.txt")).write_text(check.stdout+check.stderr)
if check.stdout.strip(): raise SystemExit("Ports occupied")
freeze=json.loads((e/"TASK/FREEZE.json").read_text())
for item in freeze["changed"]:
    if hashlib.sha256((r/item["path"]).read_bytes()).hexdigest()!=item["sha256"]: raise SystemExit("Source changed: "+item["path"])
cmd=["npx","playwright","test","e2e/b4-table-editing.spec.ts","--project=chromium-desktop","--grep","classic scrollbar compact","--workers=1","--output="+str(out/(lane+"-artifacts"))]
env=os.environ.copy(); env["PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH"]=executable; env["PLAYWRIGHT_WORKERS"]="1"; env["CI"]="1"
(out/(lane+"-command.json")).write_text(json.dumps({"cwd":str(d.relative_to(r)),"argv":cmd,"env":{k:env[k] for k in ["PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH","PLAYWRIGHT_WORKERS","CI"]},"source_freeze":freeze["changed"]},indent=2)+"\n")
with (out/(lane+"-version.txt")).open("w") as log: subprocess.run([executable,"--version"],stdout=log,stderr=subprocess.STDOUT)
with (out/(lane+".log")).open("w") as log: result=subprocess.run(cmd,cwd=d,env=env,stdout=log,stderr=subprocess.STDOUT)
(out/(lane+"-result.json")).write_text(json.dumps({"exit_code":result.returncode})+"\n")
print(lane+" exit "+str(result.returncode),flush=True)
raise SystemExit(result.returncode)
