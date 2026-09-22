import csv,hashlib,math,sys,collections,json
RUN='/Users/ryan/dev/chirality/.claude/worktrees/task-management-gen-pass-518da2/projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS'
ck={}
for r in csv.DictReader(open(RUN+'/CLAIM_KEYS_V2.csv',encoding='utf-8')):
    ck[r['ClaimKey']]=r
def shared(k):
    if k in ck: return int(ck[k]['SharedTextCount'] or 1)
    base=k.rsplit('.s',1)[0]
    return int(ck.get(base,{}).get('SharedTextCount') or 1)
NORM={'REQUIREMENT','ACCEPTANCE','EXCLUSION'}
C100D={'UNKNOWN','ACCEPTED_DIVERGENCE','AUTHORITY_CONFLICT','LIFECYCLE_REASSESSMENT_REQUIRED'}
QUIET={'ALIGNED','COVERED_BY_CHILDREN','NOT_ASSESSED'}
rates={'F7':.25,'NA':.25,'AN':.20,'Q':.10}
out={}
for d in ['01','02','03','04']:
    rows=[r for r in csv.DictReader(open(f'{RUN}/WAVES/W3/PKG-15/DEL-15-{d}/DEL-15-{d}_forward.csv',encoding='utf-8')) if r['ClaimKey']!='#END']
    cls=collections.defaultdict(list)
    for r in rows:
        k=r['ClaimKey'];n=r['Notes'];disp=r['Disposition']
        if r['AuthorityTier']=='INVARIANT' or disp in C100D or r['BaselineClass'] in ('PROTECTED_CHECK','FROZEN_CONTRACT'): c='C100'
        elif shared(k)>1: c='SH'
        elif disp=='ALIGNED' and ('GAP_WORDING_CHECKED:' in n or 'OPEN_ACTION:' in n): c='F24'
        elif disp=='ALIGNED' and 'PRODUCT_CALLER: NONE' in n: c='F7'
        elif disp not in QUIET: c='NA'
        elif disp=='ALIGNED' and r['ClaimType'] in NORM: c='AN'
        else: c='Q'
        cls[c].append(k)
    sel={}
    for c,ks in cls.items():
        if c in rates:
            ks2=sorted(ks,key=lambda x:hashlib.sha256(x.encode()).hexdigest())
            sel[c]=ks2[:math.ceil(len(ks)*rates[c])]
        else: sel[c]=list(ks)
    out[d]={'counts':{c:len(v) for c,v in cls.items()},'sel':sel}
    print('DEL-15-'+d,len(rows),{c:f"{len(cls[c])}/{len(sel.get(c,[]))}" for c in ['C100','SH','F24','F7','NA','AN','Q']}, 'sampled',sum(len(v) for v in sel.values()))
json.dump(out,open(RUN+'/WAVES/W3/PKG-15/_verify_scratch_1/sel.json','w'),indent=1)
