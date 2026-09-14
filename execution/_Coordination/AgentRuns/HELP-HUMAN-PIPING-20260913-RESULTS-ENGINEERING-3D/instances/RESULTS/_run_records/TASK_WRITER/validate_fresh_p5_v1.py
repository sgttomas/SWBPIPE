from pathlib import Path
import json,hashlib,sys
from jsonschema import Draft202012Validator
w=Path(sys.argv[1]); out=Path(sys.argv[2]); schema=json.loads((w/"schemas/results.schema.yaml").read_text()); Draft202012Validator.check_schema(schema)
records=[]
for p in sorted(out.glob("*.document.json")):
 doc=json.loads(p.read_text()); errors=list(Draft202012Validator(schema).iter_errors(doc)); records.append({"case":p.stem,"document_sha256":hashlib.sha256(p.read_bytes()).hexdigest(),"schema_errors":[{"path":list(x.path),"message":x.message} for x in errors],"count":len(errors)})
status=[]
for p in sorted(out.glob("*.mechanics.json")):
 mechanics=json.loads(p.read_text()); case=p.name.removesuffix(".mechanics.json"); raw=mechanics["results"]; rows=[{"index":n,"id":x["id"],"kind":x["kind"],"value":x["value"],"unit":x["unit"],"source_dimension":x.get("dimension"),"entity_ref":x["entity_ref"],"metadata":x.get("metadata"),"basis_ref":x.get("basis_ref")} for n,x in enumerate(raw)]; (out/(case+".original_rows.json")).write_text(json.dumps(rows,indent=2)+"\n"); status.append({"case":case,"mechanics_status":mechanics["status"]["mechanics"],"diagnostic_codes":[x["code"] for x in mechanics["diagnostics"]],"source_rows":len(rows),"canonical_document_exists":(out/(case+".document.json")).exists(),"nonconvergence_observed":any(x["code"] in ["NONLINEAR_SUPPORT_NONCONVERGENCE","NONLINEAR_SUPPORT_LOOP_NOT_CONVERGED"] for x in mechanics["diagnostics"])})
result={"schema_sha256":hashlib.sha256((w/"schemas/results.schema.yaml").read_bytes()).hexdigest(),"full_documents":records,"statuses":status,"total_schema_errors":sum(x["count"] for x in records),"nonconvergence_qualified":any(x["nonconvergence_observed"] for x in status)}; (out/"VALIDATION.json").write_text(json.dumps(result,indent=2)+"\n"); print(json.dumps(result)); sys.exit(1 if result["total_schema_errors"] else 0)
