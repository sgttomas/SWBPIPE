"""Bounded one-off generic source05/06/09 kernel arithmetic instantiation.
Fixed metadata and exact integer envelope expressions only. No model/graph/count/encoder/solver executes.
RF specialization supplies unchanged operator/phase scaffolding, never the generic source warrant.
"""
import pathlib,json,sys
R=pathlib.Path(sys.argv[1]);out=pathlib.Path(sys.argv[2])
U128=(1<<128)-1; LAYOUT=(1<<63)-1
def checked(x):
 assert isinstance(x,int) and 0<=x<=U128
 return x
def np(k):return 0 if k==0 else 1<<(k-1).bit_length()
def mu(s):return 8 if s==1 else 4 if s<=1024 else 1
def P(s,k):return 0 if not k else max(mu(s),np(k))
def G(s,k):return checked(s*P(s,k))
def A(s,k):return checked(s*max(mu(s),2*k)) if k else 0
def O(s,k):return G(s,k)//2 if k>mu(s) else 0
def Sort(s,k):return s*max(k,48) if k>=2 else 0
def TI(k,node):return 0 if not k else (1+(k-1)//5)*node
def pair(r,old=0):return (checked(r),checked(r+old))
def add(a,b):return (checked(a[0]+b[0]),checked(a[0]+b[0]+max(a[1]-a[0],b[1]-b[0])))
def plus(a,r):return (checked(a[0]+r),checked(a[1]+r))
def mx(*v):return (max(x[0] for x in v),max(x[1] for x in v))
def tracker(Rj):
 lazy=4304*min(512,P(4304,Rj)); tab=A(40,Rj); kept=G(40,Rj)
 lazyold=lazy//2 if Rj>1 else 0
 alts={'lazy_grow':pair(lazy+tab,lazyold),
       'taken_lazy_table_grow':pair(lazy+tab,tab//2),
       'stable_sort_after_lazy_drop':pair(tab+Sort(40,Rj)),
       'old_table_plus_kept':pair(tab+kept,O(40,Rj))}
 return mx(*alts.values()),lazy+tab
def tracker_set(q,F,B):
 offers=q+2*F; max_j=q; keys=8*B
 tab=40*(2*offers+4*keys); lazy=4352*4304 if offers else 0
 trees=TI(keys,1168)+TI(keys,200)
 # Other trackers' lazy buffers deliberately retained during one prune.
 alts={'lazy_grow':pair(lazy+tab+trees,256*4304 if offers else 0),
       'taken_lazy_table_grow':pair(lazy+tab+trees,A(40,max_j)//2),
       'one_table_sort':pair(lazy+tab+trees+Sort(40,max_j)),
       'one_old_table_plus_kept':pair(lazy+tab+trees+G(40,max_j),O(40,max_j))}
 return mx(*alts.values()),alts
widths=[(128,4,4),(256,4,8),(512,8,16),(1024,16,16)]
w=lambda L:8*L+16
Mop={4:7888,8:13136,16:23632};coeff={4:248,8:408,16:728};pivot={4:104,8:168,16:296}
block={4:208,8:336,16:592};body={4:624,8:1040,16:1872};bc={4:976,8:1584,16:2800}
start={4:64,8:96,16:160};result={4:272,8:432,16:752};va={4:560,8:592,16:656}
Dir={4:440,8:728,16:1304}
def evaluate(c,policy):
 N=c['N'];m=c['m'];axis=c['s'];di=c['d'];n=c['n'];f=c['f'];r=c['r'];l=c['l'];t=c['t'];B=c['B'];b=c['b'];z=c['z'];h=c['h'];q=c['q'];ids=c['source_id_bytes'];src_len=c['X']
 assert c['u']==0 and n==6*N and f==n-r
 assert q==7*N+12*m+6*t+axis+3*di+r
 assert src_len==38+24*N+84*m+17*axis+41*di+13*r+17*l+ids+16*t
 assert 0<=B<=N and 0<=b<=f
 v=min(l,n);J=68*v;F=q-7*N;U=78*m+axis+6*di;Pi=144*m+axis+9*di
 src0=24*N+G(88,m)+G(24,axis)+G(48,di)+G(16,r)+G(40,l)+G(16,t)+16*n+4*N+ids
 clone=24*N+88*m+24*axis+48*di+16*r+40*l+16*t+16*n+4*N+ids
 stf=26+24*N+84*m+17*axis+41*di+5*r;led=10+18*v+8*J
 geom=24*B+8*(4*B+4*N);geomcopy=24*B+16*N
 pattern=8*(n+1)+A(8,z)+8*z+8*(z+1)+8*U
 ordering=G(8,f)+8*n+24*f
 blocks=4*f+G(24,b)+8*(4*b+2*f)+4*b
 prep=432+clone+48*v+v+8*J+48*r+A(1,src_len)+G(20,q)+8*B
 group=368+geom+pattern+ordering+blocks
 call=1464+80+A(1,stf)
 base=src0+prep+group+call+geomcopy+3328+64
 sort=max(Sort(88,m),Sort(24,axis),Sort(48,di),Sort(16,r),Sort(40,l),Sort(16,t))
 constructorR=src0+max(sort,3*2144,12*N)
 constructorM=src0+max(sort,3*2144,12*N,O(88,m),O(24,axis),O(48,di),O(16,r),O(40,l),O(16,t))
 # Source05 K03/K04 and source09 nonzero-direction filtered-then-append capacity rule.
 gp=r+axis+6*N
 gcommon=G(4,N)+TI(N,240)+24*N+G(8,gp)
 assessment=G(24,N)+G(48,gp)+48*gp+G(48,2*N+5)+G(192,N)+N*(3*G(8,10)+3*G(8,1))+3*G(8,2)+G(8,10)+G(48,N)+80+G(8,11)
 assessment_old=max(O(4,N),O(8,gp),O(24,N),O(48,gp),O(48,2*N+5),O(192,N),O(48,N),O(8,2*N),G(8,11)//2)
 dircap=24*max(P(24,di),4,2*(di+3)) if di else 0
 geometry_pair=mx(pair(gcommon+assessment,assessment_old),pair(gcommon+dircap,max(dircap//2,O(4,N),O(8,gp),O(8,2*N))))
 pos=G(16,U);raw=24*n+8*(4*n+2*Pi)
 pattern_extra=mx(pair(pos+raw,max(O(16,U),A(8,z)//2,O(8,Pi))),pair(pos+G(16,U)+8*(z+1),O(16,U)))
 degree_sum=min(f*max(0,f-1),Pi)
 adj=24*f+8*(4*f+2*degree_sum);neigh=24*f+8*(4*f+4*degree_sum)
 Qmax=0 if not f else max(1,f-1);queue=pair(G(8,Qmax),O(8,Qmax))
 reach_old=0 if f<=1 else 8 if f<=4 else G(8,f)//2
 reach=pair(f+G(8,f)+queue[0],max(reach_old,queue[1]-queue[0]))
 ecc=pair(f+4*G(8,f),max(O(8,f),reach_old))
 rcm=plus(mx(reach,ecc,queue,pair(Sort(8,max(0,f-1)))),adj+neigh+9*f)
 rcm=mx(rcm,pair(adj+neigh+9*f,max(O(8,max(0,f-1)),O(8,2*max(0,f-1)),O(8,f))))
 block_extra=pair(G(8,f),max(reach_old,O(24,b),O(8,f)))
 prep_extra=mx(pair(G(1104,v),O(1104,v)),pair(G(4,N)+24*N,O(4,N)),pair(0,src_len),pair(0,O(20,q)))
 scaffolding=pair(0,max(stf,O(8,2*N)))
 S={};UU={};V={}
 for p,L,RR in widths:
  wl=w(L);wr=w(RR)
  S[p]=720+m*Mop[L]+di*Dir[L]+z*(wl+wr)+m*coeff[RR]+di*Dir[RR]+h*wl+f*(48+pivot[L])+(b*wl if p>=256 else 0)
  UU[p]=104+(n+6*m+q)*wl
  if p>=256:V[p]=va[L]+z*wl+144*m*wr+9*di*wr+b*block[L]
 all_schedules={}
 for schedule,ps in [('all',[128,256,512,1024]),('selected128',[128,256])]:
  vs=[p for p in ps if p>=256]
  K=base+sum(S[p]+UU[p] for p in ps)+sum(V[p]+40*B+G(24,2*b)+24*b for p in vs)
  phases={};details={}
  def put(name,val,parts=None):
   phases[name]={'requested_bytes':val[0],'moving_bytes':val[1],'active_old_upper':val[1]-val[0]}
   if parts is not None:details[name]=parts
  put('group_geometry',plus(geometry_pair,K))
  put('group_pattern_tagging',plus(pattern_extra,K))
  put('group_order_rcm',plus(rcm,K))
  put('group_free_blocks',plus(block_extra,K))
  put('prep_ledger_extent_encoding_layout',plus(prep_extra,K))
  put('scaffolding_encoding_geometry_copy',plus(scaffolding,K))
  returns=[]
  for p,L,RR in widths:
   if p not in ps:continue
   wl=w(L);wr=w(RR);C=P(wl,f);tr,held=tracker(f)
   sharedbuild=mx(pair(z),pair((m*Mop[RR] if p<1024 else 0)+z),pair(f*wl),pair(5*f*wl+24*b+8*f),tr)
   put('shared_build_'+str(p),plus(sharedbuild,K))
   solve_helper=pair((2*C+f)*wl,O(wl,f))
   prefix=f*wl+C*wl+96+4*f*wl
   residual=f*(wl+16)
   put('own_initial_solve_'+str(p),plus(solve_helper,K+f*wl))
   put('own_residual_'+str(p),plus(tr,K+prefix+residual))
   put('own_correction_'+str(p),plus(add(tr,solve_helper),K+prefix+residual+f*wl))
   active5=plus(tr,4*held)
   fallback_rows=pair(G(4304,f),O(4304,f))
   fallback_state=plus(add(active5,fallback_rows),n*wl+4*(4288+96))
   fallback_blocks=pair(144*m*wr+held)
   put('own_fallback_'+str(p),plus(mx(fallback_state,fallback_blocks),K+prefix+residual+z*wr))
   put('fallback_selected_clone_'+str(p),pair(K+prefix+residual+held+f*wl))
   put('own_recovery_'+str(p),pair(K+prefix+(12*m+axis+3*di+n)*wl))
   if p<256:continue
   Vbuild=mx(pair(m*coeff[L]+16*b+144*m*wl),
             pair(m*coeff[L]+16*b+((m*Mop[RR]+di*Dir[RR]) if p<1024 else 0)),
             pair(m*coeff[L]+16*b+max(4*f*wl+4*f,2*f*wl+2*b*wl)))
   put('verify_shared_build_'+str(p),plus(Vbuild,K))
   put('verify_shared_refusal_'+str(p),pair(K+16*b+G(24,b),O(24,b)))
   RES=G(16,B);Pass=3*n*wl+(7*f+C)*wl+(q+6*m)*wl+50*n+RES
   # Actual VR cases.rs513-517 sets value0; verify.rs802-803 skips it. Keep50n headers/flags.
   put('resolution_TOP_'+str(p),pair(K+n*wl+q*wl+2*B*wl+RES,O(16,B)))
   put('resolution_HATCHECK_'+str(p),pair(K+n*wl+q*wl+2*RES,O(16,B)))
   put('first_formation_scale_'+str(p),pair(K+n*wl+q*wl+(18*m+axis+3*di+n)*wl))
   early=3*q*wl
   put('pass_delta_solve_'+str(p),plus(solve_helper,K+Pass+early-C*wl))
   put('pass_recovery_'+str(p),pair(K+Pass+early+(12*m+axis+3*di+n)*wl))
   put('pass_formation_scale_'+str(p),pair(K+Pass+early+(18*m+axis+3*di+n)*wl))
   put('pass_one_contribution_operand_'+str(p),pair(K+Pass+early+wr))
   profile=h*wl+36*f;factor=h*wl+(32+wl)*f+b
   controls=b+16*b+G(start[L],b)+result[L]*b+A(8,b)+16*b+G(start[L],b)+b*wl
   shift_extra=max(f*wl,3*f*wl,f*wl+b*wl+G(start[L],b))
   shift_old=max(O(start[L],b),A(8,b)//2)
   put('shift_factor_Nl_retry_'+str(p),pair(K+Pass+early+profile+factor+controls+shift_extra,shift_old))
   report=344+5*q*wl+(f+C)*wl+bc[L]*b+body[L]*B+RES
   caller_controls=b+16*b+G(start[L],b)+result[L]*b
   passrest=3*n*wl+6*f*wl+(q+6*m)*wl+50*n # exact moved-identity cancellation, independent of B
   put('report_build_'+str(p),pair(K+passrest+report+caller_controls+G(24,b),max(O(24,b),O(start[L],b))))
   put('attempt_refusal_transfer_'+str(p),pair(K+report+G(24,b),O(24,2*b)))
   decision=G(16,4*B)+2*G(16,2*B)+(16*B if p==1024 else 0)
   ts,tsalts=tracker_set(q,F,B)
   R7=mx(pair(q+8*B*wl),plus(ts,q+4*B*wl+16*B+decision))
   R7=mx(R7,pair(q+4*B*wl+16*B+decision+ts[0],max(O(16,4*B),O(16,2*B))))
   put('R7_tracker_decision_'+str(p),plus(R7,K+report),{'tracker_alternatives':tsalts,'decision':decision,'report_bytes':report,'R7_requested_extra':R7[0],'R7_moving_extra':R7[1]})
   put('canonical_before_R7_'+str(p),pair(K+report+G(20,q),O(20,q)))
   put('canonical_certificate_'+str(p),pair(K+report+decision+G(20,q),O(20,q)))
   pub=G(64,q);scale=A(16,4*B);radius=8*q
   put('publication_draft_'+str(p),pair(K+report+decision+24*q+64*B+pub+scale,max(O(64,q),scale//2)))
   cert=pub+scale+radius
   put('certificate_H_RU_'+str(p),pair(K+report+decision+cert))
   rst=22+(n+6*m)*(9+8*L)
   finish=1976+40*B+(40*B+48*b)+src_len+A(1,led)+A(1,rst)+128*B+64*B+8*r+G(24,q)+G(40,q)+40*B+G(16,B)+24*B
   finish_old=max(led,rst,O(24,q),O(40,q),O(16,B))
   put('selected_finish_'+str(p),pair(K+report+decision+cert+finish,finish_old))
   returns.append(K+cert+finish)
  selreturn=max(returns)
  kernelR=max(x['requested_bytes'] for x in phases.values());kernelM=max(x['moving_bytes'] for x in phases.values())
  all_schedules[schedule]={'Kpad':K,'phases':phases,'details':details,'kernel_requested':kernelR,'kernel_moving':kernelM,'selected_return_upper':selreturn,
    'dominant_requested':[k for k,vv in phases.items() if vv['requested_bytes']==kernelR],
    'dominant_moving':[k for k,vv in phases.items() if vv['moving_bytes']==kernelM]}
 full=all_schedules['all'];short=all_schedules['selected128']
 for a in all_schedules.values():
  a['outcome_union']={'selected':a['selected_return_upper'],'refused':a['Kpad'],'unresolved':a['Kpad'],'stationary_requested':max(a['selected_return_upper'],a['Kpad'])}
  a['outcome_union']['stationary_moving']=a['outcome_union']['stationary_requested']
  a['R7_by_precision']={name.removeprefix('R7_tracker_decision_'):{'requested':v['R7_requested_extra'],'moving':v['R7_moving_extra']} for name,v in a['details'].items()}
  a['R7_max']={metric:max(v[metric] for v in a['R7_by_precision'].values()) for metric in ['requested','moving']}
 positive=S[128]+UU[128]
 premises={'B0_positive':base>0,'S128_U128_positive':positive>0,'O128_contains_B0_S128_U128':short['outcome_union']['stationary_requested']>=base+positive,
 'full_solve_requested_ge_short':full['kernel_requested']>=short['kernel_requested'],'full_solve_moving_ge_short':full['kernel_moving']>=short['kernel_moving'],
 'full_return_ge_short':full['outcome_union']['stationary_requested']>=short['outcome_union']['stationary_requested'],
 'short_phase_roster_contained':set(short['phases']).issubset(full['phases']),
 'each_short_phase_requested_le_full':all(full['phases'][k]['requested_bytes']>=v['requested_bytes'] for k,v in short['phases'].items()),
 'each_short_phase_moving_le_full':all(full['phases'][k]['moving_bytes']>=v['moving_bytes'] for k,v in short['phases'].items())}
 assert all(premises.values()),premises
 return {'id':c['id'],'descriptor':c,'policy':policy,'owners':{'SRC0':src0,'PREP_source_clone':clone,'PREP':prep,'GROUP':group,'CALL':call,'GEOM_COPY':geomcopy,'B0':base,'S':S,'U':UU,'V':V,'minimum_attempt_backing':3328,'minimum_state_backing':64},
 'source_constructor':{'requested':constructorR,'moving':constructorM,'temporal_scope':'before solve; original SRC0 is separately paid once inside solve Base'},
 'generic_details':{'U':U,'Pi':Pi,'F':F,'v_upper':v,'J_upper':J,'STF_length':stf,'LED_length_upper':led,'ground_count_upper':gp,'active_directional_capacity_upper':dircap,'prescribed_operand_children':0,'prescribed_outer_headers_flags':50*n},
 'schedules':all_schedules,'RESULT5_kernel_premises':premises,'unresolved_kernel_cells':[]}
def validate(x):
 if isinstance(x,bool):return
 if isinstance(x,int):assert 0<=x<(1<<63),x
 elif isinstance(x,dict):
  for v in x.values():validate(v)
 elif isinstance(x,list):
  for v in x:validate(v)
descs=json.loads((out/'DESCRIPTORS.json').read_text())['rows']
rows=[]
for r in descs:
 c=dict(r['descriptor'],id=r['id'])
 z=evaluate(c,'Reference193: exact stored z/h; tagged B_upper=N,b_upper=f')
 z['input_provenance']=r
 validate(z);rows.append(z)
(out/'KERNEL_193.json').write_text(json.dumps({'basis':'generic source05/06/09 on40129 with reviewed request profile','rows':rows},indent=2)+'\n')
# Interface projection contains exactly the fields needed by the independent caller/RESULT5 composition.
projection=[]
for z in rows:
 f=z['schedules']['all'];s=z['schedules']['selected128']
 projection.append({'id':z['id'],'B_policy':z['input_provenance']['B_policy'],'b_policy':z['input_provenance']['b_policy'],'descriptor':z['descriptor'],
 'B0':z['owners']['B0'],'Shared':z['owners']['S'],'Solved':z['owners']['U'],
 'K_all':{'requested':f['kernel_requested'],'moving':f['kernel_moving']},'K_128':{'requested':s['kernel_requested'],'moving':s['kernel_moving']},
 'O_all':f['outcome_union']['stationary_requested'],'O_128':s['outcome_union']['stationary_requested'],
 'returned_full':f['outcome_union'],'returned_short':s['outcome_union'],'R7':f['R7_max'],'R7_by_precision':f['R7_by_precision'],
 'subset_premises':z['RESULT5_kernel_premises'],'profile_qualification':'conditional immutable reference facts; not current-host/artifact/global RESULT5/admission'})
(out/'RESULT5_KERNEL_INPUTS.json').write_text(json.dumps(projection,indent=2)+'\n')
# Exact-policy CLI24 comparison and tagged upper-policy dominance for the12 embedded overlap.
prior=json.loads((R/'I21/vr_join_20/KERNEL_PHASES.json').read_text())['rows']
ref={x['id']:x for x in rows};comp=[];exactrows=[]
for old in prior:
 sh=old['VR_shape'];a=old['VR_input_specific']
 c={'id':old['id'],'N':sh['N'],'m':sh['m'],'s':a['s'],'d':a['d'],'u':a['u'],'r':sh['r'],'l':sh['l'],'t':sh['t'],'n':sh['n'],'f':sh['f'],'z':sh['z'],'h':sh['h_W1_unused_for_sparse'],'q':sh['q'],'X':sh['X'],'source_id_bytes':sh['load_id_bytes'],'B':sh['B'],'b':sh['b']}
 z=evaluate(c,'CLI24 exact reviewed B/b/z/h and original input identities');exactrows.append(z);full=z['schedules']['all']
 checks={'SRC0':z['owners']['SRC0']==old['SRC0_VR'],'PREP_clone':z['owners']['PREP_source_clone']==old['PREP_source_clone'],'B0':z['owners']['B0']==old['BASE'],'Kpad':full['Kpad']==old['Kpad'],'solve_R':full['kernel_requested']==old['kernel_solve_requested'],'solve_M':full['kernel_moving']==old['kernel_solve_moving'],'outcome':full['outcome_union']['stationary_requested']==old['kernel_outcome_retained_upper'],'phases':full['phases']==old['phases']}
 assert all(checks.values()),(old['id'],checks)
 upper=None
 if old['id'] in ref:
  rr=ref[old['id']];ss=rr['schedules']['all'];short=rr['schedules']['selected128'];eshort=z['schedules']['selected128']
  upper={'B0':rr['owners']['B0']>=z['owners']['B0'],'full_R':ss['kernel_requested']>=full['kernel_requested'],'full_M':ss['kernel_moving']>=full['kernel_moving'],'full_O':ss['outcome_union']['stationary_requested']>=full['outcome_union']['stationary_requested'],'short_R':short['kernel_requested']>=eshort['kernel_requested'],'short_M':short['kernel_moving']>=eshort['kernel_moving'],'short_O':short['outcome_union']['stationary_requested']>=eshort['outcome_union']['stationary_requested'],
  'all_phase_R':all(ss['phases'][k]['requested_bytes']>=v['requested_bytes'] for k,v in full['phases'].items()),'all_phase_M':all(ss['phases'][k]['moving_bytes']>=v['moving_bytes'] for k,v in full['phases'].items())}
  assert all(upper.values()),(old['id'],upper)
 comp.append({'id':old['id'],'exact_policy_checks':checks,'reference193_upper_dominance':upper,'external12':old['id'] not in ref})
(out/'CLI24_EXACT_KERNEL.json').write_text(json.dumps(exactrows,indent=2)+'\n')
(out/'COMPARISONS.json').write_text(json.dumps(comp,indent=2)+'\n')
print(json.dumps({'reference_rows':len(rows),'phase_records':sum(len(a['phases']) for x in rows for a in x['schedules'].values()),'RESULT5_subset_checks':sum(len(x['RESULT5_kernel_premises']) for x in rows),'CLI24_exact_policy_matches':len(comp),'overlap_upper_dominance_rows':sum(x['reference193_upper_dominance'] is not None for x in comp),'external_exact_rows':sum(x['external12'] for x in comp),'maximum_reference_full_R':max(x['schedules']['all']['kernel_requested'] for x in rows),'maximum_reference_full_M':max(x['schedules']['all']['kernel_moving'] for x in rows)},indent=2))
