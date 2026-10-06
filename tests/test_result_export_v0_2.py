"""Strict version dispatch and independently captured whole derivative documents."""
from pathlib import Path
from copy import deepcopy
import json,sys,unittest
from jsonschema import Draft202012Validator
ROOT=Path(__file__).resolve().parents[1]
def validator():
 # The dispatcher's 0.3.0 arm is a $ref to results.v0.3.schema.yaml (CQ-7 B). It and that
 # file's own references resolve through the local schema registry, built as
 # tests/schema_validation.py validate_instance builds it; there is no network retrieval.
 from urllib.parse import urljoin
 from referencing import Registry,Resource
 from referencing.jsonschema import DRAFT202012
 schema=json.loads((ROOT/'schemas/results.schema.yaml').read_text());Draft202012Validator.check_schema(schema);resources={}
 for path in (ROOT/'schemas').glob('*.schema.*'):
  try:candidate=json.loads(path.read_text(encoding='utf-8'))
  except (OSError,ValueError):continue
  resource=Resource.from_contents(candidate,default_specification=DRAFT202012);urls={path.resolve().as_uri(),urljoin(schema.get('$id',''),path.name)}
  if '$id' in candidate:urls.add(candidate['$id']);urls.update(urljoin(url,candidate['$id']) for url in list(urls))
  resources.update({url:resource for url in urls})
 return Draft202012Validator(schema,registry=Registry().with_resources(resources.items()))
def error_record(e):return {'path':list(e.absolute_path),'schema_path':list(e.absolute_schema_path),'message':e.message,'context':[error_record(c) for c in e.context]}
def errors(doc):return [error_record(e) for e in validator().iter_errors(doc)]
class ResultVersions(unittest.TestCase):
 def test_exact_legacy_and_strict_unknown_mixed(self):
  doc=json.loads((ROOT/'fixtures/results/invented/tp_phys_015_canonical_solve_result_envelope.json').read_text());self.assertEqual(errors(doc),[])
  for outer,inner in [('0.3.0','0.3.0'),('0.1.0','0.2.0'),('0.2.0','0.1.0'),('0.2.0','0.3.0')]:
   bad=deepcopy(doc);bad['schema_version']=outer;bad['result_envelope']['schema_version']=inner;self.assertTrue(errors(bad),(outer,inner))
 def test_version_sources_and_projected_legacy_definitions(self):
  old=json.loads((ROOT/'schemas/results.v0.1.schema.yaml').read_text());dispatch=json.loads((ROOT/'schemas/results.schema.yaml').read_text());new=json.loads((ROOT/'schemas/results.v0.2.schema.yaml').read_text());self.assertEqual(old['$defs'],dispatch['$defs']);self.assertEqual(new['properties']['schema_version']['const'],'0.2.0')
def inspect_outputs(folder):
 folder=Path(folder);report=[]
 for docpath in sorted(folder.glob('*.document.json')):
  doc=json.loads(docpath.read_text());found=errors(doc);report.append({'path':str(docpath),'error_count':len(found),'errors':found});print(docpath.name,'errors',len(found))
 if not report:raise RuntimeError('NO_FRESH_DOCUMENTS')
 (folder/'FULL_SCHEMA_VALIDATION.json').write_text(json.dumps({'documents':report,'total_errors':sum(x['error_count'] for x in report)},indent=2)+'\n')
 return 1 if any(x['error_count'] for x in report) else 0
if __name__=='__main__':
 if len(sys.argv)>1 and sys.argv[1]=='--outputs':sys.exit(inspect_outputs(sys.argv[2]))
 unittest.main()
