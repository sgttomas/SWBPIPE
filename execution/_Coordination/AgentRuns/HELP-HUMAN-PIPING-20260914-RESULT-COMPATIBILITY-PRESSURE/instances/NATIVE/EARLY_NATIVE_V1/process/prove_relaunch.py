from pathlib import Path
import datetime,json,subprocess,sys
old=int(sys.argv[1]); exe=sys.argv[2]; output=Path(sys.argv[3])
ps=subprocess.run(["ps","-axo","pid=,ppid=,lstart=,command="],capture_output=True,text=True,check=True).stdout.splitlines()
rows=[]
for line in ps:
    if line.strip().endswith(exe):
        parts=line.strip().split(None,7)
        rows.append({"pid":int(parts[0]),"ppid":int(parts[1]),"lstart":" ".join(parts[2:7]),"command":parts[7]})
result={"captured_at":datetime.datetime.now(datetime.timezone.utc).isoformat(),"exact_executable":exe,"old_pid":old,"old_pid_absent":all(r["pid"]!=old for r in rows),"matching_processes":rows}
assert result["old_pid_absent"]
assert len(rows)==1 and rows[0]["pid"]!=old
result["new_pid"]=rows[0]["pid"]
output.write_text(json.dumps(result,indent=2,sort_keys=True)+"\n")
print(json.dumps(result,sort_keys=True))
