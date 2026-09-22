#!/usr/bin/env python3
"""Refresh only dependency evidence against exact accepted current sources.

Reuses reviewed validation checks with explicit state/output rebinding. Never
writes the original SCA-011 candidate, application or review reports.
"""
import hashlib,json,re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[7];POST=Path(__file__).resolve().parents[1];POST_OUT=POST/'review/dependencies'
SCA=ROOT/'projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP';HIST=SCA/'application/dependencies';DAG=ROOT/'projects/chirality-piping/execution/_DAG/DAG-011'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def rel(p):return str(p.relative_to(ROOT))
carrier_file=POST/'carriers/FINAL_TARGET_MANIFEST.json';carriers=json.loads(carrier_file.read_text());assert carriers['status']=='PASS'
carrier_map={r['target']:r for r in carriers['targets']}
for target,row in carrier_map.items():assert sha(ROOT/target)==row['accepted_sha256']==row['actual_sha256'],target
source=HIST/'validate_repaired.py';code=source.read_text()
# The old OUT remains the frozen read basis; only report writes move to the
# postacceptance home. The reviewed canonical/quote/parity/stage predicates stay.
replacements=[
 ("assert 'APPLIED; GROUP 3 PENDING' in md", "assert 'ACCEPTED DEPENDENCY MIRROR' in md and 'Current graph authority is DAG-011' in md"),
 ("p=ROOT/f['target'];data=p.read_bytes();assert sha(p)==f['applied_sha256']\n for op in f['operations']:\n  assert data.count(op['old'].encode())==op['expected_occurrences'];data=data.replace(op['old'].encode(),op['new'].encode())\n assert hashlib.sha256(data).hexdigest()==f['accepted_sha256']", "p=ROOT/f['target'];data=p.read_bytes();assert sha(p)==f['accepted_sha256']"),
 ("'REPAIRED_GROUP2_POSTSTATE_GROUP3_PENDING'", "'OWNER_ACCEPTED_GROUP3_CURRENT_SOURCE_CHECK'"),
 ("'STAGED_NOT_APPROVED_GROUP3_PENDING'", "'ACCEPTED_DAG011_PENDING_EXECUTION_SATISFACTION'"),
 ("(OUT/name).write_text", "(POST_OUT/name.replace('REPAIRED_','')).write_text"),
 ("'No graph adoption or lifecycle, product, native, engineering or release acceptance.'", "'Graph adoption is authorized by the actual Group-3 owner decision; lifecycle, product, native, engineering and release acceptance remain outside this check.'"),
]
for old,new in replacements:
 assert old in code,old
 code=code.replace(old,new)
ns={'__file__':str(source),'__name__':'__main__','POST_OUT':POST_OUT};exec(compile(code,str(source),'exec'),ns)
quotes=json.loads((POST_OUT/'QUOTE_CHECKS.json').read_text());oldquotes={r['DependencyID']:r for r in json.loads((HIST/'REPAIRED_QUOTE_CHECKS.json').read_text())}
rows={r['DependencyID']:r for r in ns['added']};currency=[]
# Verify the quote within the named frontmatter/claim/verification locus, not
# merely somewhere in the file. No synthesized quote or broadened locus.
for q in quotes:
 row=rows[q['DependencyID']];path=ROOT/'projects/chirality-piping'/row['EvidenceFile'];text=path.read_text();locus=row['SourceRef'].split(' # ',1)[1];quote=row['EvidenceQuote']
 if locus.startswith('frontmatter/'):
  field=locus.split('/',1)[1];section=next(x for x in text.splitlines() if x.startswith(field+':'))
 elif locus.startswith('CLM-') or locus=='Governing Values and Decisions':
  match=re.search(r'^#{2,3} '+re.escape(locus)+r'[^\n]*$',text,re.M);assert match,(row['DependencyID'],locus)
  tail=text[match.end():];next_heading=re.search(r'^#{1,3} ',tail,re.M);section=tail[:next_heading.start()] if next_heading else tail
 else:
  matched=[line for line in text.splitlines() if locus in line]
  section='\n'.join(matched)
 assert quote in section,(row['DependencyID'],locus)
 prior=oldquotes[row['DependencyID']]['source_sha256'];current=sha(path);target=rel(path)
 if prior!=current:
  c=carrier_map[target];assert c['before_sha256']==prior and c['accepted_sha256']==current
 currency.append({'DependencyID':row['DependencyID'],'EvidenceFile':row['EvidenceFile'],'SourceRef':row['SourceRef'],'EvidenceQuote':quote,'reviewed_source_sha256':prior,'accepted_source_sha256':current,'source_changed_only_by_reviewed_carrier_transform':prior!=current,'quote_verbatim_in_named_locus':True})
assert len(currency)==84
# No graph row/node bytes change during authority transition.
reviewed=json.loads((HIST/'REPAIRED_VALIDATION.json').read_text())
assert sha(DAG/'DependencyEdges.csv')==reviewed['actual_edges_sha256']
assert sha(DAG/'DeliverableNodes.csv')==reviewed['actual_nodes_sha256']
source_records=[]
initial=json.loads((HIST/'APPLICATION_FILE_CHECKS.json').read_text())['source_files']
for r in initial:
 path=ROOT/r['path'];current=sha(path)
 if current!=r['sha256']:
  c=carrier_map[r['path']];assert c['before_sha256']==r['sha256'] and c['accepted_sha256']==current
 source_records.append({'path':r['path'],'reviewed_sha256':r['sha256'],'accepted_sha256':current,'read_scope':r['read_scope'],'currency':'CURRENT_ACCEPTED_BYTES'})
report=json.loads((POST_OUT/'VALIDATION.json').read_text());report.update({'reviewed_state_commit':'d6cc1482eee78ce860ff18658f11157f7efbd401','reused_check_source':{'path':rel(source),'sha256':sha(source)},'explicit_state_output_rebindings':[old for old,new in replacements],'source_currency':'CURRENT_ACCEPTED_BYTES','named_locus_quote_checks':84,'graph_bytes_unchanged_since_review':True,'carrier_manifest':{'path':rel(carrier_file),'sha256':sha(carrier_file)},'authority_decision':{'path':rel(POST/'OWNER_DECISION.md'),'sha256':sha(POST/'OWNER_DECISION.md')}})
(POST_OUT/'VALIDATION.json').write_text(json.dumps(report,indent=2)+'\n')
(POST_OUT/'CURRENT_SOURCE_BINDINGS.json').write_text(json.dumps({'status':'PASS_CURRENT_ACCEPTED_BYTES','row_bindings':currency,'source_files':source_records,'limits':'Actual quotes/loci and exact reviewed carrier transforms checked; no new claim, satisfaction or lifecycle conclusion.'},indent=2)+'\n')
print(json.dumps({'source_currency':'CURRENT_ACCEPTED_BYTES','named_locus_quotes':84,'unchanged_graph_bytes':True}))
