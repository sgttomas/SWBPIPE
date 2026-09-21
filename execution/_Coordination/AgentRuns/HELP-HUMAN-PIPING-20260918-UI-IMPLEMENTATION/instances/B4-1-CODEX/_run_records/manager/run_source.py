"""Bounded manager source-browser runner; execute from the assigned repository root."""
from pathlib import Path
import subprocess, os, json, time, hashlib, sys
root = Path.cwd()
run = root / "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION"
desktop = root / "projects/chirality-piping/apps/desktop"
out = run / "instances/B4-1-CODEX/_run_records/browser" / sys.argv[1]
out.mkdir(parents=True, exist_ok=False)
exe = "/Users/ryan/Library/Caches/ms-playwright/chromium-1223/chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing"
assert Path(exe).is_file()
assert not subprocess.check_output(["git", "diff", "--name-only"], text=True).strip()
ports = subprocess.run(["lsof", "-nP", "-iTCP:5174", "-iTCP:5175", "-sTCP:LISTEN"], capture_output=True, text=True)
assert ports.returncode == 1, ports.stdout
common = Path(subprocess.check_output(["git", "rev-parse", "--path-format=absolute", "--git-common-dir"], text=True).strip())
assert not (common / "swbpipe-e2e.lock").exists()
args = ["sh", str(run / "tools/with_e2e_lock.sh"), "npx", "playwright", "test", *sys.argv[2:], "--workers=1", "--output", str(out / "artifacts")]
env = os.environ.copy()
env.update(PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=exe, PLAYWRIGHT_WORKERS="1")
meta = {"source": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip(), "cwd": str(desktop), "argv": args, "executable": exe, "version": subprocess.check_output([exe, "--version"], text=True).strip(), "started": time.time()}
(out / "RUN.json").write_text(json.dumps(meta, indent=2) + "\n")
with (out / "stdout.txt").open("w") as stream:
    result = subprocess.run(args, cwd=desktop, env=env, stdout=stream, stderr=subprocess.STDOUT)
meta.update(exitCode=result.returncode, finished=time.time(), stdoutSha256=hashlib.sha256((out / "stdout.txt").read_bytes()).hexdigest())
(out / "RESULT.json").write_text(json.dumps(meta, indent=2) + "\n")
ports = subprocess.run(["lsof", "-nP", "-iTCP:5174", "-iTCP:5175", "-sTCP:LISTEN"], capture_output=True, text=True)
(out / "CLEANUP.json").write_text(json.dumps({"listenersExit": ports.returncode, "listeners": ports.stdout, "lockPresent": (common / "swbpipe-e2e.lock").exists()}, indent=2) + "\n")
print((out / "stdout.txt").read_text()[-9000:])
print("RESULT", result.returncode)
sys.exit(result.returncode)
