from pathlib import Path
import datetime,hashlib,json,sys
repo=Path(sys.argv[1]).resolve(); checkpoint_path=Path(sys.argv[2]).resolve(); output=Path(sys.argv[3]).resolve()
checkpoint_bytes=checkpoint_path.read_bytes(); checkpoint=json.loads(checkpoint_bytes)
files=[]
for expected in checkpoint["candidate_source_manifest"]:
    path=repo/expected["path"]; data=path.read_bytes()
    actual={"path":expected["path"],"expected_sha256":expected["candidate_sha256"],"expected_bytes":expected["bytes"],"actual_sha256":hashlib.sha256(data).hexdigest(),"actual_bytes":len(data)}
    actual["match"]=actual["expected_sha256"]==actual["actual_sha256"] and actual["expected_bytes"]==actual["actual_bytes"]
    files.append(actual)
result={"captured_at":datetime.datetime.now(datetime.timezone.utc).isoformat(),"candidate_commit_declared":"6bb26b118fc038c451b97d3b86e3ae27d8ba8e91","checkpoint":str(checkpoint_path),"checkpoint_sha256":hashlib.sha256(checkpoint_bytes).hexdigest(),"checkpoint_entries":len(files),"all_match":all(x["match"] for x in files),"files":files}
assert result["all_match"] and len(files)==64 and result["checkpoint_sha256"]=="8c2c7154aaf150f8d0db5eed481a9c1d1a93d9a2501511113fc45d5539a94d6b"
output.write_text(json.dumps(result,indent=2,sort_keys=True)+"\n")
print(json.dumps({"all_match":True,"files":len(files),"checkpoint_sha256":result["checkpoint_sha256"]},sort_keys=True))

