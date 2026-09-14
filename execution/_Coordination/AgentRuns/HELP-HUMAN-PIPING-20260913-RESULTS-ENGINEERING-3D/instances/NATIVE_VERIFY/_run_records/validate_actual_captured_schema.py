from pathlib import Path
import json,hashlib
from jsonschema import Draft202012Validator
raw=Path(__file__).parent;schemas=Path.cwd()/'projects/chirality-piping/schemas';payload=json.loads((raw/'actual-native-canonical500.json').read_bytes());records=[]
for name,expected in [('results.schema.yaml',True),('results.v0.2.schema.yaml',True),('results.v0.1.schema.yaml',False)]:
 p=schemas/name;s=json.loads(p.read_text());Draft202012Validator.check_schema(s);errors=list(Draft202012Validator(s).iter_errors(payload));assert (len(errors)==0)==expected
 records.append({'schema':str(p),'schema_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'errors':len(errors),'expected_valid':expected})
print(json.dumps({'status':'PASS_ACTUAL_CAPTURE_STRICT_VERSIONED_SCHEMA','actual_schema_version':payload['schema_version'],'dispatcher_and_current_validate':True,'legacy_branch_rejects_current':True,'schemas':records,'source_row_hash_verification':'separate captured-payload-verifier-v1.stdout'},indent=2))
