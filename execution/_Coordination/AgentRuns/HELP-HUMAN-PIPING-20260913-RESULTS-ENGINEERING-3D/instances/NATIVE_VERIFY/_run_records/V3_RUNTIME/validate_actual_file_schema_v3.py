from pathlib import Path
import sys,json,hashlib
from jsonschema import Draft202012Validator
f=Path(sys.argv[1]);doc=json.loads(f.read_bytes());schemas=Path.cwd()/'projects/chirality-piping/schemas';records=[]
for name,expected in [('results.schema.yaml',True),('results.v0.2.schema.yaml',True),('results.v0.1.schema.yaml',False)]:
 p=schemas/name;s=json.loads(p.read_text());Draft202012Validator.check_schema(s);errors=list(Draft202012Validator(s).iter_errors(doc));assert (len(errors)==0)==expected;records.append({'schema':name,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'errors':len(errors),'expected_valid':expected})
print(json.dumps({'status':'PASS_ACTUAL_FILE_STRICT_VERSIONED_SCHEMA','actual_file':str(f),'actual_schema_version':doc['schema_version'],'file_sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'schemas':records},indent=2))
