"""Future lease-only raw source numeric check, separate from derivative adapter."""
from pathlib import Path
import argparse,json
p=argparse.ArgumentParser();p.add_argument("--snapshot",required=True);p.add_argument("--force",type=int,choices=[350,500],required=True);a=p.parse_args()
basis=json.loads((Path(__file__).parent/"FROZEN_INDEPENDENT_EXPECTATIONS_V1.json").read_text());row=json.loads(Path(a.snapshot).read_text());r=json.loads(row["mechanics_result_json"]);assert r["status"]["mechanics"]=="MECHANICS_SOLVED"
rows={x["id"]:x for x in r["results"]};assert len(rows)==len(r["results"])
checks=[]
for e in basis["values"][str(a.force)+"N"]:
 x=rows[e["source_row_id"]];assert x["unit"]==e["unit"];error=abs(x["value"]-e["expected"]);assert error<=basis["publication_absolute_allowance_declared_units"]
 checks.append({"row_id":x["id"],"value":x["value"],"expected":e["expected"],"unit":x["unit"],"error":error})
print(json.dumps({"status":"PASS_RAW_SOURCE_NUMERICS_ONLY","checks":checks},indent=2))
