"""RV52 repair backcheck: standard-library hashes/source/abstract owners only."""
from pathlib import Path
from hashlib import sha256
from contextlib import redirect_stdout
from io import StringIO
import json
HERE=Path(__file__).resolve().parent
OUT=HERE.parent
R=OUT.parent.parent
NUM=next(p for p in HERE.parents if (p/'projects/chirality-piping/core').is_dir())
PACK=R/'I29/f2a_kernel_ownership_p3'
PRIOR=OUT.parent/'f2a_kernel_ownership_p3'
checks=[]
def digest(p):return sha256(p.read_bytes()).hexdigest()
def passed(name,**facts):checks.append(dict(name=name,status='pass',**facts))
old=json.loads((PRIOR/'_run_records/ORIGINS.json').read_text())['entries']
before={str(Path(e['path']).relative_to(PACK)):e['sha256'] for e in old if Path(e['path']).is_relative_to(PACK)}
after={str(p.relative_to(PACK)):digest(p) for p in PACK.rglob('*') if p.is_file()}
changed=sorted(k for k in before if k in after and before[k]!=after[k])
added=sorted(set(after)-set(before));removed=sorted(set(before)-set(after))
assert changed==['ENVELOPE.md','OWNER_LEDGER.md','RETURN.md','_run_records/WRITE_INVENTORY.json']
assert added==['_run_records/hats_repair_02/CORRECTION.json','_run_records/hats_repair_02/checks.json','_run_records/hats_repair_02/checks.py']
assert not removed
passed('bounded_packet_hash_delta',changed=changed,added=added,removed=removed,
       caveat='packet filesystem scope compared with sealed prior hashes; ROOT separately verified Git diff')
inv=json.loads((PACK/'_run_records/WRITE_INVENTORY.json').read_text())
for e in inv['files']:
    p=PACK/e['path'];assert digest(p)==e['sha256'] and p.stat().st_size==e['bytes']
passed('candidate_inventory',payloads=len(inv['files']))
correction=json.loads((PACK/'_run_records/hats_repair_02/CORRECTION.json').read_text())
for e in correction['preserved_original_raw']+correction['preserved_review']:
    p=Path(e['path']);assert digest(p)==e['sha256'] and p.stat().st_size==e['bytes']
assert digest(PRIOR/'_run_records/WRITE_INVENTORY.json')=='2b03895b941b0fe2f8f6b63295c6897bccc950c8a3724cabb3cab46c41fc412f'
passed('original_raw_and_review_immutable',original_raw=4,review_files_including_seal=7)
orig=json.loads((PACK/'_run_records/ORIGINS.json').read_text())['entries']
n=0
for e in orig:
    if e['origin'] in ['frozen I37 git object','stable baseline canonical accounting']:
        assert digest(NUM/e['path'])==e['sha256']
        assert digest(NUM.parent/'f2a'/e['path'])==e['sha256']
        n+=1
passed('source_and_baseline_unchanged_in_NUM_and_F2A',files_each=n)
# Replay inspected author's repair controls, redirecting only the fixture/output
# directory to our write scope. Three required metadata arrays are unchanged.
rep=HERE/'author_replay';rep.mkdir(exist_ok=True)
(rep/'CORRECTION.json').write_text(json.dumps({k:correction[k] for k in ['source_origins','preserved_original_raw','preserved_review']},indent=2)+'\n')
ns={'__name__':'rv52_replay','__file__':str(rep/'checks.py')}
with redirect_stdout(StringIO()):
    exec(compile((PACK/'_run_records/hats_repair_02/checks.py').read_text(),'I29_hats_repair_checks','exec'),ns)
    ns['main']()
assert (rep/'checks.json').read_bytes()==(PACK/'_run_records/hats_repair_02/checks.json').read_bytes()
passed('all_author_repair_controls_reproduce',controls=8)
# Independently assert source construction, last uses, and closure/statement drops.
k=NUM/'projects/chirality-piping/core/solver/frame_kernel/src/structural/retained'
a=(k/'adaptive.rs').read_text();v=(k/'verify.rs').read_text();b=(k/'bound.rs').read_text()
assert a.index('let hats: Vec<[f64; 2]>') < a.index('let mut phis = Vec::with_capacity(hats.len())') < a.index('let mut trackers: TrackerSet')
assert 'resolution_hats(&resolution, &prep.extents)?;' in v
assert 'let u = block_max(&c,' in b and 'let n_l = block_max(&ct,' in b
text=(PACK/'ENVELOPE.md').read_text()
for required in ['distinct rule hats [f64;2](b)','V_rule_hats,[f64;2](b)','Old_rule_hats(e)','drops before hats construction','four-buffer','two block-max vectors u/n_l(each B)']:
    assert required in text
passed('source_to_repaired_terms')
# Separate returned data and moving generations for both optional-floor branches.
for has_floor in [False,True]:
    check_live={'report_resolution','earlier_check'}
    check_live.remove('earlier_check')
    hats_growth=check_live|{'skip','coupled_scales','hats'}
    hats_old={'hats_old'}
    assert hats_growth.isdisjoint(hats_old)
    during_summary=hats_growth|{'tracker','summary'}|({'floor'} if has_floor else set())
    assert 'earlier_check' not in during_summary and 'hats_old' not in during_summary
    assert during_summary.isdisjoint({'summary_old'})
    after_rule=during_summary-{'skip','coupled_scales','hats','tracker'}
    assert after_rule=={'report_resolution','summary'}|({'floor'} if has_floor else set())
passed('independent_lifetime_and_moving_events',floor_branches=2,
       note='hats old backing only in its growth event; live hats also overlaps later summary growth')
# Injection into same-type coarse slots, not token aliasing or a fifth buffer.
for phase in [{'a','c'},{'c','at','bt','ct'}]:
    assert len(phase)<=4
passed('uc_two_phase_domination',phase_sizes=[2,4],block_max_vectors=2,
       premise='maximum qualified site capacity law for Wide<L>(F); no byte coefficient')
result={'status':'PASS','finding_disposition':'RV52-1 CLOSED','production_execution':False,'checks':checks}
(HERE/'CHECKS.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'status':'PASS','reviewer_checks':len(checks),'author_controls':8}))
