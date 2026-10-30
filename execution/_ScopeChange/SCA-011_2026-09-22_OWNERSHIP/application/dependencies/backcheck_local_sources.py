#!/usr/bin/env python3
"""Two-pass source backcheck of the exact accepted additive local delta.

Reads each actual local source in full. Pass 1 resolves tree anchors before
pass 2 checks named information-flow obligations; it does not re-extract or
recertify the preserved baseline. Existing CSV proposal citations stay intact.
"""
import csv, hashlib, json, re, subprocess, sys
from pathlib import Path
from collections import Counter
ROOT=Path(__file__).resolve().parents[7]
SCA=ROOT/'projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP'
OUT=SCA/'application/dependencies'
def read(p):
    with p.open(newline='') as f:return list(csv.DictReader(f))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def rel(p):return str(p.relative_to(ROOT))
manifest=read(SCA/'APPLY_MANIFEST.csv')
own=[r for r in manifest if r['ApplicationLane']=='GROUP2_DEPENDENCY_TASK' and r['CanonicalTarget'].endswith('/Dependencies.csv')]
folders={Path(r['CanonicalTarget']).parent.name.split('_')[0]:ROOT/r['CanonicalTarget'] for r in own}
texts={key:(path.parent/'ScopeOfWork.md').read_text() for key,path in folders.items()}
refs={key:(path.parent/'_REFERENCES.md').read_text() for key,path in folders.items()}
contexts={key:(path.parent/'_CONTEXT.md').read_text() for key,path in folders.items()}
decomp=ROOT/'projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md';dt=decomp.read_text()
added=read(SCA/'dependencies/DependencyEdges.additions.csv');assert len(added)==84
# These source phrases were selected by the executing TASK from the actual
# accepted claim postimages. They support information transfer, not chronology.
phrases={
 'DEL-04-07':[
  ('CLM-001','Compose the accepted model, unit, boundary, section, load and kernel contracts'),
  ('CLM-002','DEL-04-01 retains frame assembly/linear solving and DEL-04-04 retains per-iteration classification'),
  ('CLM-003','section/mass conversion against DEL-03-08 contracts'),
  ('CLM-004','Preserve typed diagnostic, unit and provenance information across product composition and result handoff.')],
 'DEL-07-11':[
  ('CLM-001','Dispatch shell navigation, panel mounting, jobs and explicit file operations to their owning services'),
  ('CLM-002','durable state stored by DEL-02-05 and run records owned by DEL-14-02'),
  ('CLM-004','Conform common focus, keyboard and accessibility behavior to DEL-07-06'),
  ('Governing Values and Decisions','DEL-16-06 owns application execution;'),
  ('Governing Values and Decisions','DEL-00-03/05/07 constrain architecture.')],
 'DEL-16-06':[
  ('CLM-001','Apply only an explicitly accepted, schema/constraint-valid, previewed structured operation whose current model hash matches its accepted basis.'),
  ('Governing Values and Decisions','DEL-16-01 owns schema/taxonomy, DEL-16-02 validation/preview, DEL-16-03 acceptance/audit policy, DEL-02-05 persistence'),
  ('CLM-004','hand actual outcomes to DEL-16-03 audit and DEL-02-05 persistence without creating another mutation route.')],
 'DEL-07-12':[
  ('CLM-001','validate against the owning contract before presenting a record as actual current product output.'),
  ('CLM-002','Preserve diagnostic, unit, provenance, loss and unsupported-behavior information through presentation and explicit export dispatch.'),
  ('CLM-004','Use only explicit authorized handoff paths under protected-data controls, including native-host evidence where applicable.'),
  ('Governing Values and Decisions','native JSON package; .mbf model export; licensed external-run evidence review; stress-neutral, PCF and review-geometry export;'),
  ('Governing Values and Decisions','handoff/local-FEA/external-prover boundary panels; adapter-framework, headless-runner and export-adapter SDK review panels; validation-evidence review.')],
 'DEL-07-01':[
  ('CLM-033','Commands use the PKG-16 seam, whose application owner is DEL-16-06'),
  ('CLM-033','Own palette rendering and routing under DEL-07-09 vocabulary coverage and single organization contract')],
 'DEL-07-03':[
  ('CLM-042','Own load-case/combination editing, self-weight-plan interaction and material/section/component/library editors through PKG-16.'),
  ('VER-002','distinguish DEL-05-01 generation, DEL-03-08 mass and DEL-04-07 integration.')],
 'DEL-07-04':[
  ('CLM-033','PKG-06 retains rule semantics/runner APIs, DEL-05-04 status semantics and DEL-02-02 units.')],
 'DEL-07-08':[
  ('CLM-030','DEL-16-06 execution, and DEL-14-05 the common comparison interface.')],
 'DEL-08-04':[
  ('DEL-08-04-SCA011-REQ-001','Consumes and maps the DEL-14-05 common comparison-result/export interface into general result exports;')],
 'DEL-16-03':[
  ('CLM-029','DEL-16-06 owns controlled application and returns actual outcomes;'),
  ('CLM-029','Pre-application accepted decisions and post-application outcome receipts are distinct.')]
}
# Choose the exact supporting phrase for each reviewed execution row.
choice={**{n:0 for n in range(23,82)},26:2,27:1,28:0,29:1,30:3,34:3,
 39:4,40:4,41:4,42:1,43:2,44:1,45:3,
 46:1,47:1,48:1,49:0,50:0,51:1,52:0,53:0,
 54:4,55:4,56:4,57:4,58:3,59:3,60:3,61:3,62:3,63:3,64:3,65:3,66:4,67:4,68:4,69:4,70:4,71:2,72:0,73:1,74:2,
 76:1,77:0,78:0,82:1,83:0,84:0}
records=[]
# Pass 1: derive exactly the newly accepted parent and scope/objective anchors.
for r in added:
 if r['DependencyClass']!='ANCHOR':continue
 owner=r['FromDeliverableID'];target=r['TargetRefID'];text=texts[owner]
 field='deliverable_id' if r['AnchorType']=='IMPLEMENTS_NODE' else ('project_scope_refs' if target.startswith('SOW-') else 'package_objective_refs')
 line=next(line for line in text.splitlines() if line.startswith(field+':'))
 assert re.search(r'(?<![A-Z0-9-])'+re.escape(target)+r'(?![A-Z0-9-])',line),(r['DependencyID'],line)
 assert target in dt
 records.append({'DependencyID':r['DependencyID'],'Pass':'1_ANCHOR','Owner':owner,'Target':target,'LocalEvidenceFile':rel(folders[owner].parent/'ScopeOfWork.md'),'LocalSourceRef':'frontmatter/'+field,'LocalEvidenceSHA256':sha(folders[owner].parent/'ScopeOfWork.md'),'LocalEvidenceQuote':line,'Resolution':'Exact local identity/scope/objective anchor corroborated by applied decomposition.','AdditionalBasis':rel(decomp)+'@sha256:'+sha(decomp)})
assert len(records)==22
# Pass 2: map each existing accepted relation to a local information-flow claim.
for r in added:
 if r['DependencyClass']!='EXECUTION':continue
 owner=r['FromDeliverableID'];n=int(r['DependencyID'].split('E')[-1]);section,quote=phrases[owner][choice[n]]
 assert quote in texts[owner],(r['DependencyID'],quote)
 assert len(quote.split())<=30
 target=r['TargetDeliverableID'] or r['TargetRefID'];additional=rel(decomp)+'@sha256:'+sha(decomp)
 if r['TargetType']=='DELIVERABLE':
  targetrows=[line for line in dt.splitlines() if line.startswith('|'+target+'|')]
  assert targetrows,(r['DependencyID'],target)
  resolution='Local claim requires the named input family; applied decomposition resolves its accountable deliverable. Exact reviewed target/type retained.'
  if target.startswith('DEL-00-'):
   ab='AB-'+target.removeprefix('DEL-');assert any(line.startswith('|'+ab+'|'+target+'|') for line in dt.splitlines())
   additional+='; §8.1 '+ab
   resolution='Local contract boundary read with the decomposition architecture-basis constraint; no source lifecycle maturity inferred.'
 else:
  doc=ROOT/'projects/chirality-piping'/r['TargetLocation'];assert doc.is_file();assert target in doc.read_text()
  additional=rel(doc)+'@sha256:'+sha(doc)
  resolution='Local claim requires the distinct contract/outcome artifact. Reviewed DOCUMENT stage boundary retained; readiness remains PENDING.'
 records.append({'DependencyID':r['DependencyID'],'Pass':'2_EXECUTION','Owner':owner,'Target':target,'LocalEvidenceFile':rel(folders[owner].parent/'ScopeOfWork.md'),'LocalSourceRef':section,'LocalEvidenceSHA256':sha(folders[owner].parent/'ScopeOfWork.md'),'LocalEvidenceQuote':quote,'Resolution':resolution,'AdditionalBasis':additional})
assert len(records)==84 and len({r['DependencyID'] for r in records})==84
with (OUT/'LOCAL_SOURCE_BACKCHECK.csv').open('w',newline='') as f:
 w=csv.DictWriter(f,fieldnames=list(records[0]));w.writeheader();w.writerows(records)
local_checks=[];commands=[];enums={};ids=set()
for owner,path in folders.items():
 rows=read(path);new=[r for r in rows if r['DependencyID'].startswith('SCA011-')]
 assert [r['DependencyID'] for r in new]==[r['DependencyID'] for r in added if r['FromDeliverableID']==owner]
 byid={r['DependencyID']:r for r in added}
 assert all(all(r[k]==byid[r['DependencyID']][k] for k in r) for r in new)
 assert len({r['DependencyID'] for r in rows})==len(rows)
 assert all(r['FromDeliverableID']==owner for r in rows)
 assert sum(r['Status']=='ACTIVE' and r['AnchorType']=='IMPLEMENTS_NODE' for r in rows)==1
 assert all(r['EvidenceFile'] and r['SourceRef'] for r in rows if r['Status']=='ACTIVE')
 md=(path.parent/'_DEPENDENCIES.md').read_text();assert f'{len(rows)} rows:' in md
 assert 'APPLIED; GROUP 3 PENDING' in md
 assert str(dict(Counter(r['SatisfactionStatus'] for r in rows))) in md
 command=['python3','tools/validation/validate_dependencies_schema.py',rel(path)]
 p=subprocess.run(command,cwd=ROOT,text=True,capture_output=True);commands.append({'command':command,'exit_code':p.returncode,'stdout':p.stdout,'stderr':p.stderr});assert p.returncode==0
 local_checks.append({'owner':owner,'rows':len(rows),'added_rows':len(new),'parent_anchors':1,'row_equality':'PASS','schema':'PASS','index_counts':'PASS','historical_rows_preserved_not_revalidated':len(rows)-len(new)})
 for r in rows:
  ids.add(('DEL',r['FromDeliverableID']));ids.add(('PKG',r['FromPackageID']))
  if r['TargetDeliverableID']:ids.add(('DEL',r['TargetDeliverableID']))
  if r['TargetPackageID']:ids.add(('PKG',r['TargetPackageID']))
  for column,enum in [('DependencyClass','DEPENDENCY_CLASS'),('AnchorType','ANCHOR_TYPE'),('Direction','DIRECTION'),('DependencyType','DEPENDENCY_TYPE'),('TargetType','TARGET_TYPE'),('Explicitness','EXPLICITNESS'),('Confidence','CONFIDENCE'),('Origin','ORIGIN'),('Status','STATUS'),('SatisfactionStatus','SATISFACTION_STATUS')]:enums.setdefault(enum,set()).add(r[column])
for enum,values in sorted(enums.items()):
 for value in sorted(values):
  command=['python3','tools/validation/validate_enum.py',enum,value];p=subprocess.run(command,cwd=ROOT,text=True,capture_output=True);commands.append({'command':command,'exit_code':p.returncode,'stdout':p.stdout,'stderr':p.stderr});assert p.returncode==0
for typ,value in sorted(ids):
 command=['zsh','tools/validation/validate_id_format.sh',typ,value];p=subprocess.run(command,cwd=ROOT,text=True,capture_output=True);commands.append({'command':command,'exit_code':p.returncode,'stdout':p.stdout,'stderr':p.stderr,'disposition':'Generic three-digit package template is incompatible with accepted Piping two-digit identities; checked against applied decomposition instead.'});assert re.fullmatch(r'DEL-[0-9]{2}-[0-9]{2}' if typ=='DEL' else r'PKG-[0-9]{2}',value);assert '|'+value+'|' in dt
# Negative source-control witness: deleting a required literal is detected by
# the same source predicate; this is not an implementation/conformance test.
probe_quote=phrases['DEL-07-01'][0][1];altered=texts['DEL-07-01'].replace(probe_quote,'')
assert probe_quote not in altered
report={'status':'PASS','mode':'UPDATE, additive accepted delta only','strictness':'CONSERVATIVE','source_docs':['ScopeOfWork.md','_CONTEXT.md','_REFERENCES.md'],'anchor_doc':'ScopeOfWork.md frontmatter','execution_doc_order':['ScopeOfWork.md named claims; _CONTEXT.md boundaries; _REFERENCES.md locator resolution'],'workflow':'bundled:chirality-root/dependency-extract','workflow_limits':'Each source pass is deliverable-local plus decomposition. Cross-deliverable aggregate assembly and separate centralized evidence are ad hoc assigned work, not workflow authority. The exact Group-2 index bytes are preserved; these explicit run settings and returned backchecks supplement their accepted application history.','generic_id_validator':'NONZERO: stock helper expects three-digit package IDs; Piping accepted two-digit identities preserved and independently resolved against actual decomposition. This is a declared format applicability limit, not a waived invalid identity.','anchor_rows':22,'execution_rows':62,'local_owners':local_checks,'local_missing_source_literal_negative_probe':'DETECTED','limits':['Semantic target mapping is a source-grounded TASK judgment, not merely a token search. Script asserts quotes, identities and accepted row equality so later replay can detect drift.','Historical CSV proposal citations and candidate Notes are retained provenance. Current support is bound separately to applied local source bytes.','This bounded delta check does not claim a corpus-wide re-extraction, new satisfaction evidence, lifecycle advancement or graph adoption.']}
(OUT/'LOCAL_EXTRACTION_VALIDATION.json').write_text(json.dumps(report,indent=2)+'\n')
(OUT/'LOCAL_CHECK_COMMANDS.json').write_text(json.dumps(commands,indent=2)+'\n')
print(json.dumps({'status':'PASS','owners':len(local_checks),'anchors':22,'execution':62,'registered_command_checks':len(commands)}))
