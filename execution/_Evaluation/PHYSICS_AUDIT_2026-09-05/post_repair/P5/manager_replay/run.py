from pathlib import Path
import subprocess,json,hashlib,base64
p=Path(__file__).resolve().parent
w=next(x for x in p.parents if x.name=="chirality-piping")
source=w/"core/product_physics/src/lib.rs"
before=hashlib.sha256(source.read_bytes()).hexdigest()
manifest=json.loads((p/"PLANNED_INPUTS_V2.json").read_text())
binary=p/"target/debug/p5-manager-replay"
(p/"results").mkdir(exist_ok=True)
records=[]
for item in manifest["inputs"]:
    x=w/item["path"];assert hashlib.sha256(x.read_bytes()).hexdigest()==item["sha256"]
    for mode in item["modes"]:
        cmd=[str(binary),str(x),mode]
        result=subprocess.run(cmd,capture_output=True)
        name=x.stem+"."+mode
        if result.returncode==0:
            payload=json.loads(result.stdout);(p/"results"/(name+".json")).write_text(json.dumps(payload,indent=2)+"\n")
        record=dict(input=item["path"],input_sha256=item["sha256"],mode=mode,returncode=result.returncode,stderr_base64=base64.b64encode(result.stderr).decode(),stdout_sha256=hashlib.sha256(result.stdout).hexdigest(),source_sha256=before,binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest())
        if result.returncode:record["stdout_base64"]=base64.b64encode(result.stdout).decode()
        records.append(record)
assert hashlib.sha256(source.read_bytes()).hexdigest()==before,"source changed during replay"
(p/"EXECUTIONS.json").write_text(json.dumps(dict(source_sha256=before,executions=records),indent=2)+"\n")
print(json.dumps(dict(executions=len(records),failures=sum(r["returncode"]!=0 for r in records),source_sha256=before)))
raise SystemExit(any(r["returncode"]!=0 for r in records))
