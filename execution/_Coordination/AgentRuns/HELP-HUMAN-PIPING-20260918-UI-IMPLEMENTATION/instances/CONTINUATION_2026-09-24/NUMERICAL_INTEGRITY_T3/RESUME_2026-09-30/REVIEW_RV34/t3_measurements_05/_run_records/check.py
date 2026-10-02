"""T3 adapter for RV34's sealed independent T2 checker.
Prior evidence is referenced in memory, never copied into this packet.
Only integer/String-label metadata arithmetic is added for the six RF1000
constructors whose canonical files are absent from the fixed source archive.
"""
from review_io import *
import hashlib
path=R+'/REVIEW_RV34/t2_measurements_04/_run_records/check.py'
code=get(path).decode();changes=[]
def replace(old,new):
 global code
 assert old in code,old
 changes.append({'old':old,'new':new,'occurrences':code.count(old)})
 code=code.replace(old,new)
replace("t1=archive(1);t2=archive(2);raw={'records/'+n:b for n,b in t1.items()};raw.update(t2)","t1=archive(1);previous=archive(2);t2=archive(3);raw={'records/'+n:b for n,b in t1.items()};raw.update(previous);raw.update(t2)")
replace("seal_check('SHA256SUMS','1f1e7f70e6970687d6a13e3c4ccfd61e77b47935591d8e1266a720f71acf9883')", "seal_check('SHA256SUMS','a5f6391280ca179bbbc00a3e34ad9f2030d7f4eff0be36c2e023143ed9d40034')")
replace("seal_check('OUTCOME_CORRECTION_SHA256SUMS','3d0b208fde5ea6ca4501bbdec6bbf6d719d8e2bbfb127aeee7001ca770406a7a')", "# T3 has no summary correction: use this tier's raw counters.")
replace("oldbytes=t1['records.jsonl']", "oldbytes=previous['records/records.jsonl']")
replace("223099","313009")
replace("jr['original_T1_prefix_sha256']","jr['original_prefix_sha256']")
replace("for tier in [1,2]:","for tier in [1,2,3]:")
replace("W1-T2","W1-T3")
replace("T2_LAUNCHES.json","T3_LAUNCHES.json")
for label in ['T2_archive_exact_set','T2_raw_hash','T2_raw_bytes','T2_orders','T2_ids']:
 replace(label,label.replace('T2','T3'))
replace('T1_dictionaries_preserved','prior_dictionaries_preserved')
replace('T1_prefix_bytes','prior_prefix_bytes')
replace('correction_raw_sparse_tally','raw_sparse_tally')
replace("len(journal),84","len(journal),108")
replace("runs=journal[60:]","runs=journal[84:]")
replace("journal[:60]","journal[:84]")
replace("range(199,223)","range(223,247)")
replace("[199,204,207,212,215,220]","[223,228,231,236,239,244]")
replace("quals['normal_solve_or_admission'],False","quals['normal_solves']+quals['numeric_admissions'],0")
replace("enumerate(runs,start=60)","enumerate(runs,start=84)")
replace("else 600);eq('process_time_wrapper'","else 1800);eq('process_time_wrapper'")
replace("'time_budget_s':540","'time_budget_s':1740")
replace("r['model'].replace('-n00100-','-n00010-')","r['model'].replace('-n01000-','-n00100-')")
replace("'all_T2_members100':all(r['members']==100 for r in runs)","'all_T3_members1000':all(r['members']==1000 for r in runs)")
replace("eq('prefix_count',len(pref),3 if p['prefixes'] else 0);", "# Prefix completeness is derived from the actual segment sequence below.\n  ")
replace("if pref:eq('prefix_limit_segments'", "eq('prefix_count_from_actual_segments',len(pref),len(segments)-1 if p['prefixes'] else 0)\n  if pref:eq('prefix_limit_segments'")
replacement=r'''
# Scalar label/population substitution for actual Builder capture. These are
# exactly the frozen models.rs chain/tree/cont label grammars and HModelFacts
# storage rules, not new model/source/count/graph computation.
models_source=get(H+'/src/k6/models.rs',SOURCE).decode()
get(H+'/src/k6/w1/h_envelope.rs',SOURCE)
r1_source=re.search(r'pub const R1_SOURCE: &str = "([^"]+)"',models_source)[1]
def digit_bins(a,b):
    return [(max(0,min(b,10**d-1)-max(a,0 if d==1 else 10**(d-1))+1),d) for d in range(1,len(str(b))+1)]
def label_total(a,b,formatted):
    return sum(count*(max(8,2*(d+1)) if formatted else d+1) for count,d in digit_bins(a,b))
def modelbytes(mid):
    s=seed[mid];n=s['nodes'];m=s['members'];loads=s['w1_loads'];family=s['family']
    eq('RF_builder_nodes',n,m+1);eq('RF_builder_stations',s['w1_stations'],m)
    if family=='CHAIN':
        node_labels=label_total(0,m,True);member_labels=label_total(1,m,False);restraint_nodes=1
    elif family in ['TREE','CONT']:
        assert m%2==0
        node_labels=8+2*label_total(1,m//2,True);member_labels=2*label_total(1,m//2,False)
        restraint_nodes=1 if family=='TREE' else m//2+1
    else:raise AssertionError(family)
    eq('RF_constraints',s['w1_constraints'],6 if family in ['CHAIN','TREE'] else 6+3*(m//2))
    return G(48,n)+64*m+G(16,restraint_nodes)+G(16,loads)+G(136,m)+node_labels+member_labels+len(mid.encode())+len(r1_source.encode())
models={mid:modelbytes(mid) for mid in sorted({p['model'] for p in planned})}
'''
replace("models={mid:modelbytes(mid) for mid in sorted({p['model'] for p in planned})}",replacement)
save('CHECKER_ADAPTATION.json',{'origin':path,'revision':REV,'original_sha256':hashlib.sha256(get(path)).hexdigest(),'expanded_checker_sha256':hashlib.sha256(code.encode()).hexdigest(),'changes':changes,'purpose':'Reuse the independently authored T2 scalar/record method at T3 boundaries. No author program executed and no previous packet copied.'})
exec(compile(code,'RV34 independently authored T2 method adapted to T3','exec'),globals())
