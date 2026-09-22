#!/usr/bin/env python3
"""Read-only simulation. Does not mutate production or stages1-3 evidence."""
from pathlib import Path
import collections,csv,hashlib,io,json,sys
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent;sys.path.insert(0,str(HERE.parent));import repair
ROOT,sha=repair.ROOT,repair.sha
sys.path.insert(0,str(ROOT/'tools/scope_of_work'));from common import parse_sow_text,validate_document
manifest=json.loads((HERE/'PROPOSED_PHYSICAL_EDITS_STAGE4_V2.json').read_text());sows=0;cells=collections.Counter();states=collections.Counter()
for f in manifest['files']:
 p=ROOT/f['path'];current=p.read_text();states['before' if sha(current)==f['before_sha256'] else 'after' if sha(current)==f['after_sha256'] else 'drift']+=1
 assert sha(current) in [f['before_sha256'],f['after_sha256']],p
 if sha(current)==f['before_sha256']:before=current;after=current
 else:
  after=current;before=current
  for e in reversed(f['edits']):
   a,b=e['new_start_offset'],e['new_end_offset'];assert before[a:b]==e['new'];before=before[:a]+e['old']+before[b:]
  assert sha(before)==f['before_sha256']
 after=before
 for e in reversed(f['edits']):
  a,b=e['old_start_offset'],e['old_end_offset'];assert after[a:b]==e['old'];after=after[:a]+e['new']+after[b:]
 assert sha(after)==f['after_sha256']
 if p.name=='ScopeOfWork.md':
  issues=validate_document(parse_sow_text(p,after));assert not issues,(p,issues);sows+=1
 if p.suffix=='.csv':
  a=list(csv.DictReader(io.StringIO(before)));b=list(csv.DictReader(io.StringIO(after)));assert len(a)==len(b)==27
  allow={'CurrentAcceptance','CurrentEvidenceBasis'} if p.name=='Vocabulary_Coverage.csv' else {'current_coverage_state','current_basis','current_derivative_status'}
  for x,y in zip(a,b):
   assert x.keys()==y.keys()
   for k in x:
    if x[k]!=y[k]:assert k in allow,(p,k);cells[k]+=1
   for k in ['CurrentAcceptance','current_coverage_state']:
    if x.get(k)=='DEFERRED_ROADMAP':assert y[k]=='DEFERRED_ROADMAP'
 assert '/DEL-01-01_' not in str(p) and p.name!='_STATUS.md'
report=dict(source_states=dict(states),files=len(manifest['files']),sow_structural_pass=sows,csv_only_allowed_fields=dict(cells),history_deferred_residual_cells='PRESERVED',inverse_replay='PASS',production_written=False)
(HERE/('DRY_RUN_CHECK.json' if states.get('before') else 'APPLIED_CHECK.json')).write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
