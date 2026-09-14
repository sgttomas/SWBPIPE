import hashlib, importlib.metadata, json, pathlib, sys
from jsonschema import Draft202012Validator
r=pathlib.Path.cwd();w=r/"projects/chirality-piping"
sys.path.insert(0,str(w/"tests"))
from schema_validation import load_schema, validate_schema_document
source=pathlib.Path(sys.argv[1]);target=pathlib.Path(sys.argv[2]);target.mkdir(parents=True,exist_ok=False)
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
schema_path=w/"schemas/results.schema.yaml";schema=load_schema(schema_path);validate_schema_document(schema)
report={"environment":{"executable":sys.executable,"version":sys.version,"jsonschema":importlib.metadata.version("jsonschema")},"schema_origin":str(schema_path),"schema_sha256":h(schema_path),"fresh_output_root":str(source),"cases":[]}
for p in sorted(source.glob("*.mechanics.json")):
 case=p.name.removesuffix(".mechanics.json");m=json.loads(p.read_text());d=source/(case+".document.json");absence=source/(case+".canonical_absence.json");error=source/(case+".adapter_error.json")
 item={"case":case,"mechanics_sha256":h(p),"source_status":m["status"],"source_row_count":len(m["results"]),"source_dimensions_present":sum("dimension" in row for row in m["results"]),"source_rows":m["results"]}
 if d.exists():
  document=json.loads(d.read_text());errs=[{"instance_path":"/"+"/".join(map(str,e.absolute_path)),"schema_path":"/"+"/".join(map(str,e.absolute_schema_path)),"message":e.message} for e in sorted(Draft202012Validator(schema).iter_errors(document),key=lambda e:str(list(e.absolute_path)))]
  item.update({"document_sha256":h(d),"schema_error_count":len(errs),"schema_errors":errs,"status":"FAIL_SCHEMA" if errs else "PASS_SCHEMA"})
 elif absence.exists():item.update({"canonical_absence_sha256":h(absence),"canonical_absence":json.loads(absence.read_text()),"status":"EXPECTED_PRODUCER_ABSENCE"})
 elif error.exists():item.update({"adapter_error_sha256":h(error),"adapter_error":json.loads(error.read_text()),"status":"ADAPTER_ERROR"})
 else:raise AssertionError("No explicit document/absence/error for "+case)
 report["cases"].append(item)
(target/"FULL_CURRENT_VALIDATION_V1.json").write_text(json.dumps(report,indent=2,ensure_ascii=False)+"\n")
for item in report["cases"]:print(item["case"],item["status"],"rows",item["source_row_count"],"schema_errors",item.get("schema_error_count","n/a"),"mechanics",item["source_status"]["mechanics"])
sys.exit(1 if any(x["status"] in ["FAIL_SCHEMA","ADAPTER_ERROR"] for x in report["cases"]) else 0)
