"""One-off bounded arithmetic substitution of the sealed I21 H/kernel ledger.
No model, graph, Rust, solver, source algorithm, parser under study or runtime executes.
All inputs are existing immutable count/launch/history metadata. Not a maintained tool.
Run: <VENV>/bin/python -B _run_records/NUMERIC_ARITHMETIC.py <K6C_WT> <R> <OUT>
"""
import sys,json,pathlib
root=pathlib.Path(sys.argv[1]); R=root/sys.argv[2]; out=pathlib.Path(sys.argv[3])
public_fact=json.loads((out/'_run_records/PUBLIC_SIZE_BINDING.json').read_text())
S_COORD=public_fact['consumed_only']['size_bytes'];assert S_COORD==32
public_status=public_fact['status']
data=json.loads((R/'I21/source_09/FIXTURE_INPUTS.json').read_text())['h_rows']
launch=json.loads((R/'I21/k0_assembly_16/PLANNED_LAUNCHES.json').read_text())['plan']
old=json.loads((R/'I21/k0_assembly_16/SUBSTITUTIONS.json').read_text())['rows']
hist=json.loads((R/'I21/k0_assembly_16/HISTORICAL_COMPARISON.json').read_text())['rows']
old={x['model']:x for x in old}
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
records=[]
for c in data:
 model=c['model'];N=c['w1_nodes'];m=c['w1_members'];n=c['w1_dofs'];f=c['w1_free_dofs']
 r=c['w1_constraints'];l=c['w1_loads'];t=c['w1_stations'];B=c['w1_bodies'];b=c['w1_blocks']
 z=c['w1_pattern_entries'];h=c['w1_profile_entries'];q=c['w1_rows'];U=78*m;Pi=144*m
 assert c['w1_source_ok'] and n==6*N and f==n-r and q==7*N+12*m+6*t+r
 src_len=c['w1_source_encoding_len']
 ids=src_len-(38+24*N+84*m+13*r+17*l+16*t)
 assert 4*l<=ids<=l*(3+len(str(max(0,n-1))))
 max_id=3+len(str(max(0,n-1)));v=min(l,n);J=68*v;F=q-7*N
 src0=24*N+88*m+16*r+40*l+16*t+16*n+4*N+2*ids
 clone=24*N+88*m+16*r+40*l+16*t+16*n+4*N+ids
 stf=26+24*N+84*m+5*r;led=10+18*v+8*J
 geom=24*B+8*(4*B+4*N);geomcopy=24*B+16*N
 pattern=8*(n+1)+A(8,z)+8*z+8*(z+1)+8*U
 ordering=G(8,f)+8*n+24*f
 blocks=4*f+G(24,b)+8*(4*b+2*f)+4*b
 prep=432+clone+48*v+v+8*J+48*r+A(1,src_len)+G(20,q)+8*B
 group=368+geom+pattern+ordering+blocks
 call=1464+80+A(1,stf)
 base=src0+prep+group+call+geomcopy+3328+64
 source_sort=max(Sort(88,m),Sort(16,r),Sort(40,l),Sort(16,t))
 source=mx(pair(src0+source_sort),pair(src0+3*2144),pair(src0+12*N),
           pair(src0+G(8,r),O(8,r)),pair(src0,max_id))
 gp=r+6*N
 geom_extra=G(4,N)+TI(N,240)+24*N+G(8,gp)+G(24,N)+G(48,gp)+48*gp+G(48,2*N+5)+G(192,N)+N*(3*G(8,10)+3*G(8,1))+3*G(8,2)+G(8,10)+G(48,N)+80+G(8,11)
 geom_old=max(O(4,N),O(8,gp),O(24,N),O(48,gp),O(48,2*N+5),O(192,N),O(48,N),O(8,2*N),G(8,11)//2)
 pos=G(16,U);raw=24*n+8*(4*n+2*Pi)
 pattern_extra=mx(pair(pos+raw,max(O(16,U),A(8,z)//2,O(8,Pi))),
                  pair(pos+G(16,U)+8*(z+1),O(16,U)))
 degree_sum=min(f*max(0,f-1),Pi)
 adj=24*f+8*(4*f+2*degree_sum)
 neigh=24*f+8*(4*f+4*degree_sum)
 Qmax=0 if not f else max(1,f-1)
 queue=pair(G(8,Qmax),O(8,Qmax))
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
  S[p]=720+m*Mop[L]+z*(wl+wr)+m*coeff[RR]+h*wl+f*(48+pivot[L])+(b*wl if p>=256 else 0)
  UU[p]=104+(n+6*m+q)*wl
  if p>=256:V[p]=va[L]+z*wl+144*m*wr+b*block[L]
 all_schedules={}
 for schedule,ps in [('all',[128,256,512,1024]),('selected128',[128,256])]:
  vs=[p for p in ps if p>=256]
  K=base+sum(S[p]+UU[p] for p in ps)+sum(V[p]+40*B+G(24,2*b)+24*b for p in vs)
  phases={};details={}
  def put(name,val,parts=None):
   phases[name]={'requested_bytes':val[0],'moving_bytes':val[1],'active_old_upper':val[1]-val[0]}
   if parts is not None:details[name]=parts
  put('source_constructor',source)
  put('group_geometry',plus(pair(geom_extra,geom_old),K))
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
   put('own_recovery_'+str(p),pair(K+prefix+(12*m+n)*wl))
   if p<256:continue
   Vbuild=mx(pair(m*coeff[L]+16*b+144*m*wl),
             pair(m*coeff[L]+16*b+(m*Mop[RR] if p<1024 else 0)),
             pair(m*coeff[L]+16*b+max(4*f*wl+4*f,2*f*wl+2*b*wl)))
   put('verify_shared_build_'+str(p),plus(Vbuild,K))
   put('verify_shared_refusal_'+str(p),pair(K+16*b+G(24,b),O(24,b)))
   RES=G(16,B);Pass=3*n*wl+(7*f+C)*wl+(q+6*m)*wl+50*n+RES
   # Zero prescribed operands is proven by H adapter and verify.rs802-803; keep50n headers/flags.
   put('resolution_TOP_'+str(p),pair(K+n*wl+q*wl+2*B*wl+RES,O(16,B)))
   put('resolution_HATCHECK_'+str(p),pair(K+n*wl+q*wl+2*RES,O(16,B)))
   put('first_formation_scale_'+str(p),pair(K+n*wl+q*wl+(18*m+n)*wl))
   early=3*q*wl
   put('pass_delta_solve_'+str(p),plus(solve_helper,K+Pass+early-C*wl))
   put('pass_recovery_'+str(p),pair(K+Pass+early+(12*m+n)*wl))
   put('pass_formation_scale_'+str(p),pair(K+Pass+early+(18*m+n)*wl))
   put('pass_one_contribution_operand_'+str(p),pair(K+Pass+early+wr))
   profile=h*wl+36*f;factor=h*wl+(32+wl)*f+b
   controls=b+16*b+G(start[L],b)+result[L]*b+A(8,b)+16*b+G(start[L],b)+b*wl
   shift_extra=max(f*wl,3*f*wl,f*wl+b*wl+G(start[L],b))
   shift_old=max(O(start[L],b),A(8,b)//2)
   put('shift_factor_Nl_retry_'+str(p),pair(K+Pass+early+profile+factor+controls+shift_extra,shift_old))
   report=344+5*q*wl+(f+C)*wl+bc[L]*b+body[L]*B+RES
   caller_controls=b+16*b+G(start[L],b)+result[L]*b
   passrest=Pass-RES-(f+C)*wl
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
 # Caller model owners from sealed fixed constructor grammar, not model construction.
 node_label=1+len(str(max(0,N-1)));member_label=1+len(str(max(0,m)))
 isdec=c['family']=='DEC053'; source_text_len=len(model) if isdec else len('r1:references.json@7b176dbb')
 caller=(max(G(48,N),N*S_COORD) if isdec else G(48,N))+64*m+G(16,min(N,r))+G(16,l)+G(136,m)+N*A(1,node_label)+m*(A(1,member_label) if isdec else member_label)+len(model)+source_text_len+1700
 saved=3328+120*B+144*b
 variants=[]
 for plan in [x for x in launch if x['model']==model]:
  argc=plan['persistent_arg_string_bytes'];prefix_bytes=387 if plan['w1_prefixes'] else 0
  CP=caller+argc+saved
  sums={}
  for schedule,a in all_schedules.items():
   K=a['Kpad']
   sourceR=CP+max(source[0],150);sourceM=CP+max(source[1],225)
   solveR=CP+max(a['kernel_requested'],K+393616)
   solveM=CP+max(a['kernel_moving'],K+590424)
   prefixR=CP+prefix_bytes+max(a['kernel_requested'],K+393616,K+2756016,a['selected_return_upper']+708) if plan['w1_prefixes'] else 0
   prefixM=CP+prefix_bytes+max(a['kernel_moving'],K+590424,K+3937177,a['selected_return_upper']+1023) if plan['w1_prefixes'] else 0
   sums[schedule]={'requested_bytes':max(sourceR,solveR,prefixR),'moving_bytes':max(sourceM,solveM,prefixM),
     'source_requested':sourceR,'source_moving':sourceM,'solve_requested':solveR,'solve_moving':solveM,'prefix_requested':prefixR,'prefix_moving':prefixM}
  variants.append({'run_id':plan['run_id'],'pass':plan['pass'],'prefix':plan['w1_prefixes'],'persistent_argument_bytes':argc,'Hfixed':base+CP+prefix_bytes,'totals':sums})
 totalR=max(x['totals']['all']['requested_bytes'] for x in variants);totalM=max(x['totals']['all']['moving_bytes'] for x in variants)
 selR=max(x['totals']['selected128']['requested_bytes'] for x in variants);selM=max(x['totals']['selected128']['moving_bytes'] for x in variants)
 delta='max(0,N*S_coord-G_48(N))' if isdec else None
 decision='defer_binary_half_cap' if totalM>4026531840 else 'passes_binary_half_cap_only'
 assert max(0,N*S_COORD-G(48,N))==0
 rec={'model':model,'family':c['family'],'descriptors':c,'input_upper':{'s':0,'d':0,'u':0,'v':v,'J':J,'source_id_bytes':ids,'source_encoding_length':src_len,'raw_insertions':Pi,'U':U,'free_degree_sum_upper':degree_sum,'restrained_nodes_upper':min(N,r)},
      'owners':{'SRC0':src0,'source_clone':clone,'PREP':prep,'GROUP':group,'CALL':call,'GEOM_COPY':geomcopy,'BASE':base,'caller_model_runtime_without_args':caller,'saved_attempts':saved,'S':S,'U':UU,'V':V},
      'schedules':all_schedules,'launch_variants':variants,
      'proposed_mapping':{'E_max_requested_numeric_part':totalR,'E_max_moving_numeric_part':totalM,'selected128_requested_numeric_part':selR,'selected128_moving_numeric_part':selM,'Hfixed_numeric_part':max(x['Hfixed'] for x in variants),'symbolic_addend':delta,'S_coord_threshold_for_zero_addend':str(G(48,N))+'/'+str(N) if isdec else None,'complete_conditional_numeric':True,'S_coord_substitution_bytes':S_COORD if isdec else None,'evaluated_symbolic_addend':0,'public_size_status':public_status if isdec else 'not dependent on this fact','decide_requested_bytes':max(vv['R7_requested_extra'] for vv in all_schedules['all']['details'].values()),'decide_moving_bytes':max(vv['R7_moving_extra'] for vv in all_schedules['all']['details'].values()),'binary_half_cap':4026531840,'decision':decision,'full_runner_admission':'not replayed; recalibration denominators and source/build binding required'},
      'legacy':{'max':old[model]['legacy_H_estimate_max_bytes'],'fixed':old[model]['legacy_H_fixed_bytes']}}
 for a in all_schedules.values():
  for val in a['phases'].values():assert 0<=val['requested_bytes']<=val['moving_bytes']<=LAYOUT
 records.append(rec)
out.mkdir(parents=True,exist_ok=True)
(out/'PHASES.json').write_text(json.dumps({'basis':'40129a225d73860ac2a53da9a2fa73869df668f3','units':'bytes','status':'Conditional source arithmetic; no final qualification/admission','rows':records},indent=2)+'\n')
comparisons=[]
for x in hist:
 if x['binary']!='k6_observe':continue
 rec=next(vv for vv in records if vv['model']==x['model']);p=rec['proposed_mapping']
 variant=next(vv for vv in rec['launch_variants'] if vv['run_id']==x['run_id'])
 tt=variant['totals']['all'];sr=max(tt['source_requested'],tt['solve_requested']);sm=max(tt['source_moving'],tt['solve_moving'])
 comparisons.append({'model':x['model'],'run_id':x['run_id'],'historical':x,
 'candidate_inner_staged_requested_numeric_part':sr,'candidate_inner_staged_moving_numeric_part':sm,
 'candidate_outer_prefix_requested_numeric_part':tt['prefix_requested'] if variant['prefix'] else None,
 'candidate_outer_prefix_moving_numeric_part':tt['prefix_moving'] if variant['prefix'] else None,
 'candidate_legacy_mapping_requested_numeric_part':p['E_max_requested_numeric_part'],
 'candidate_legacy_mapping_moving_numeric_part':p['E_max_moving_numeric_part'],
 'symbolic_addend':p['symbolic_addend'],'inner_requested_difference_numeric_part':sr-x['recorded_heap_bytes'],
 'inner_moving_difference_numeric_part':sm-x['recorded_move_bytes'],
 'status':'Historical inner source/solve staged peaks compare only to candidate inner stages. Prefix upper is separate; no historical prefix peak is manufactured. No calibration/admission replay or final artifact proof'})
(out/'HISTORICAL_COMPARISON.json').write_text(json.dumps(comparisons,indent=2)+'\n')
print(json.dumps({'rows':len(records),'historical_H_rows':len(comparisons),'conditional_numeric_rows':sum(x['proposed_mapping']['complete_conditional_numeric'] for x in records),'DEC_public_fact_rows':sum(x['family']=='DEC053' for x in records),'largest_RF_moving':max(x['proposed_mapping']['E_max_moving_numeric_part'] for x in records if x['proposed_mapping']['complete_conditional_numeric'])},indent=2))
