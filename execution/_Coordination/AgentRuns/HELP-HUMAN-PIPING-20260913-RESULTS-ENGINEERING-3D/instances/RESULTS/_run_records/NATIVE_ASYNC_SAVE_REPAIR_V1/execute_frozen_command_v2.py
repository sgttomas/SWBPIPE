import datetime, hashlib, json, os, pathlib, subprocess, sys
folder = pathlib.Path(__file__).resolve().parent
frozen = folder / "SOURCE_AND_COMMAND_FREEZE_V2.json"
packet = json.loads(frozen.read_text())
command = next(c for c in packet["commands_proposed_not_run"] if c["id"] == sys.argv[1])
for row in packet["source"]:
    actual = hashlib.sha256((pathlib.Path(packet["commands_proposed_not_run"][0]["cwd"]) / row["path"]).read_bytes()).hexdigest()
    if actual != row["sha256"]:
        raise SystemExit("FROZEN_SOURCE_MISMATCH: " + row["path"])
out = folder / ("attempt" + command["id"])
out.mkdir(exist_ok=False)
env = os.environ.copy()
env.update(command["env"])
record = dict(command, started_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(), source_freeze_sha256=hashlib.sha256(frozen.read_bytes()).hexdigest(), runner_sha256=hashlib.sha256(pathlib.Path(__file__).read_bytes()).hexdigest())
(out / "command.json").write_text(json.dumps(record, indent=2) + "\n")
with (out / "stdout.txt").open("wb") as stdout, (out / "stderr.txt").open("wb") as stderr:
    result = subprocess.run(command["argv"], cwd=command["cwd"], env=env, stdout=stdout, stderr=stderr)
record.update(exit_code=result.returncode, finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat())
record["outputs"] = [{"path":name,"sha256":hashlib.sha256((out/name).read_bytes()).hexdigest(),"bytes":(out/name).stat().st_size} for name in ["stdout.txt", "stderr.txt"]]
(out / "result.json").write_text(json.dumps(record, indent=2) + "\n")
print(json.dumps(record, indent=2), flush=True)
raise SystemExit(result.returncode)
