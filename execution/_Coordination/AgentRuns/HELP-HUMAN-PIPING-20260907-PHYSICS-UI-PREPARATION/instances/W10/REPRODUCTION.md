# W10 actual-library reproduction — CANDIDATE evidence

Basis: `35249accf139f52478d029458946e50ed25ee5dc`; decomposition 0.12 / SCA-009 / DAG-010; parent freeze V4 SHA256 `5b6dd41664b7a2e01e9138c462e58bd0127386c54b681052c11d36b668b96d68`. Original handoff preserved by parent/CHANGE; this instance never accessed or wrote it. Outputs are derivative diagnostic preparation, not accepted decomposition truth.

Actual worker `/root/canonical_preparation`, WORKING_ITEMS Agent 1, PKG-10 / DEL-10-05. Runtime model identity unexposed; no model inferred. No children. Delegated-harness-native; role boundary instruction+config asserted. Write scopes are exactly W10_V1; production files, decisions, pointers, receipts, staging and publication untouched. Parent owns orchestration and independent review.

## Observations

| Actual library scenario | Quantity rows | Metadata rows | Invalid rows | Metadata violations | Whole-document errors |
|---|---:|---:|---:|---:|---:|
| linear | 777 | 739 | 109 | 158 | 160 |
| nonlinear | 797 | 753 | 123 | 210 | 212 |
| zero_pressure | 774 | 736 | 106 | 150 | 152 |

All three current documents are byte-identical to prior accepted actual-library witnesses under `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT/instances/P5/compatibility_v1/author/`. Each exercises primitive and combined station rows. This is actual `PreviewRunnerOutput.result_envelope_document` from `run_preview_in_memory`, not inline CLI output: serde skips this library-only field. The existing station-category test passes 1/1, but it checks only station metadata and therefore does not imply whole-document conformance. Full Draft202012Validator independently reproduces the failures. All errors and exact paths are in ACTUAL_VALIDATION.json; ROW_INVENTORY.json indexes every quantity by scenario/result ID against the hash-bound accepted documents (which retain family/unit/dimension/reference/magnitude/metadata), and FIELD_INVENTORY.json binds every distinct metadata value to scenario/result IDs.

An in-memory experiment changes only `result_envelope.reproducibility.model_hash.canonicalization` and each `run_hashes[*].canonicalization` from `rfc8785_jcs` to `JCS`. It removes precisely two errors per document, retaining 158/210/150 metadata errors. Every quantity object and referenced checksum algorithm/value/payload_ref is equal before/after. No transformed document or product code was installed. Raw rerun documents were moved to the temporary build root after byte-for-byte comparison, before freeze; durable evidence points to the identical immutable accepted bytes to avoid duplicate payloads. A future checksum over the changed enclosing document must be recalculated over its new bytes; do not relabel an old digest as hashing new bytes. Referenced model/mechanics payloads did not change.

## Reproduction recipe and environment

Resolve REPO_ROOT with `git rev-parse --show-toplevel`, WORKING_ROOT as its `projects/chirality-piping`. The committed loop was verified by regular-blob lookup and read via git show; parent/CHANGE owns receipt/self-check/full harness execution. This instance inspected profile, contracts, register, context, SOW_V1, local dependencies, current DAG pointer and live source.

Use a new temporary root and copy the exact tracked files under `projects/chirality-piping/{core,validation,fixtures,schemas}` retaining relative paths (inventory/hashes in BUILD_CONTEXT.json). This avoids a newly generated Cargo.lock inside production: headless has no tracked lock. Set CARGO_TARGET_DIR to the temporary root's target; set PIPING_P5_SCHEMA_WITNESS_DIR to the temporary `actual` directory. Native cargo/rust toolchain was 1.97.1; `.github/workflows/piping-desktop-e2e.yml` pins 1.97.1. Cargo resolved 48 packages offline using existing cache; the resolved lock bytes are RESOLVED_CARGO_LOCK.txt. To replay identical resolution, copy that text into the temporary headless Cargo.lock then use --offline --locked. No downloads or production build files were needed.

From the temporary root execute:

```sh
cargo test --offline --locked --manifest-path projects/chirality-piping/core/runner/headless/Cargo.toml --lib result_envelope_binding::tests::straight_station_library_document_metadata_uses_canonical_schema_categories -- --exact --nocapture
```

Initial attempt used the unqualified test name with --exact, built successfully but ran zero tests (39 filtered). It is NOT counted as validation. Corrected qualified invocation ran 1 PASS / 38 filtered; WITNESS.log records it. Existing mechanics benchmark unused-import warning retained; no repair needed for this diagnostic.

Python validation used parent's isolated venv, jsonschema Draft202012Validator, and strict JSON loading of results.schema.yaml. Provisioning is C0 evidence; no temporary executable path is a durable prerequisite. Exact diagnostic source follows, also hashed in SOURCE_BINDINGS.json. It is documentation for rerun, not a new committed executable or production test. Run it from REPO_ROOT in a Python environment containing jsonschema, setting W10_DOC_DIR to the temporary witness directory. It writes full diagnostic intermediates; the terminal packet compacts them by retaining paths/field values instead of repeated messages and duplicated quantity objects, and references identical accepted documents. Do not overwrite a frozen evidence packet: replay into a fresh authorized run-local output path by changing `out`.

```python
import json,pathlib,hashlib,collections,copy,csv,os
from jsonschema import Draft202012Validator
root=pathlib.Path.cwd(); w=root/'projects/chirality-piping'; out=w/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260907-PHYSICS-UI-PREPARATION/instances/W10'
schema=json.loads((w/'schemas/results.schema.yaml').read_text()); Draft202012Validator.check_schema(schema); validator=Draft202012Validator(schema)
options={k:v.get('enum',[]) for k,v in schema['$defs']['ResultMetadata']['properties'].items()}; options['canonicalization']=schema['$defs']['Checksum']['properties']['canonicalization']['enum']
(out/'CLOSED_OPTIONS.json').write_text(json.dumps(options,indent=2)+'\n'); summary={}; matrix=collections.defaultdict(list); inventory=[]
doc_dir=pathlib.Path(os.environ['W10_DOC_DIR'])  # newly emitted temporary actual documents
for f in sorted(doc_dir.glob('*_canonical_document.json')):
 d=json.loads(f.read_text()); label=f.name.removesuffix('_canonical_document.json'); env=d['result_envelope']; rows=[r for s in env['result_sets'] for r in s['values']]
 errors=[{'path':'/'+ '/'.join(map(str,e.absolute_path)),'schema_path':'/'+ '/'.join(map(str,e.absolute_schema_path)),'value':e.instance,'validator':e.validator,'message':e.message} for e in validator.iter_errors(d)]
 patched=copy.deepcopy(d)
 repro=patched['result_envelope']['reproducibility']; before=copy.deepcopy(repro)
 for c in [repro['model_hash'],*repro['run_hashes']]:
  if c['canonicalization']=='rfc8785_jcs':c['canonicalization']='JCS'
 remaining=list(validator.iter_errors(patched)); after=copy.deepcopy(repro)
 for a,b in zip([before['model_hash'],*before['run_hashes']],[after['model_hash'],*after['run_hashes']]):
  a.pop('canonicalization');b.pop('canonicalization');assert a==b
 assert rows==[r for s in patched['result_envelope']['result_sets'] for r in s['values']]
 invalid=0
 for r in rows:
  md=r.get('metadata'); fields=[]
  if md:
   for key,val in md.items():
    valid=not options[key] or val in options[key]
    matrix[(key,val,valid)].append({'scenario':label,'result_id':r['result_id']})
    if not valid:fields.append(key)
  invalid+=bool(fields)
  inventory.append({'scenario':label,'result_id':r['result_id'],'family':r['family'],'object_ref':r['object_ref'],'basis_ref':r['basis_ref'],'unit':r['unit'],'dimension':r['dimension'],'magnitude':r['magnitude'],'metadata':md,'invalid_fields':fields,'status':'UNSUPPORTED_MAPPING_REQUIRED' if fields else 'SCHEMA_VALID_IDENTITY_PRESERVE'})
 old=w/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT/instances/P5/compatibility_v1/author'/f.name
 summary[label]={'sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'matches_prior_actual_bytes':f.read_bytes()==old.read_bytes(),'quantity_rows':len(rows),'metadata_rows':sum(bool(r.get('metadata')) for r in rows),'invalid_metadata_rows':invalid,'whole_document_errors':len(errors),'metadata_field_errors':sum('/metadata/' in e['path'] for e in errors),'after_label_only_errors':len(remaining),'label_only_preserves_all_quantities_and_referenced_checksums':True,'errors':errors}
(out/'ACTUAL_VALIDATION.json').write_text(json.dumps(summary,indent=2)+'\n')
(out/'ROW_INVENTORY.json').write_text(json.dumps(inventory,indent=2)+'\n')
(out/'FIELD_INVENTORY.json').write_text(json.dumps([{'field':k,'value':v,'schema_valid':ok,'rows':rs} for (k,v,ok),rs in sorted(matrix.items())],indent=2)+'\n')
print(json.dumps({k:{kk:vv for kk,vv in v.items() if kk!='errors'} for k,v in summary.items()},indent=2))
```
