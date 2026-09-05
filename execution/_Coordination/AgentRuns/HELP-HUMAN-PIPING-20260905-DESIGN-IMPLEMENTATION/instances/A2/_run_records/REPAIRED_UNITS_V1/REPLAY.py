from pathlib import Path
import json,subprocess,hashlib,sys
r=Path(__file__).resolve().parent
p=r.parents[1]
outputs=[]
for unit in ["m","mm","in"]:
 fixture=p/"snapshots/UNITS_V1/fixtures"/f"invented_linear_{unit}.json"
 output=subprocess.check_output([sys.argv[1],str(fixture)],text=True)
 outputs.append(json.loads(output))
 print(unit,hashlib.sha256(fixture.read_bytes()).hexdigest(),outputs[-1]["status"]["mechanics"],outputs[-1]["summary"]["max_displacement"])
assert all(o["status"]["mechanics"]=="MECHANICS_SOLVED" for o in outputs)
assert outputs[0]==outputs[1]==outputs[2]
print("PASS: entire envelopes equal for three frozen fixtures")
