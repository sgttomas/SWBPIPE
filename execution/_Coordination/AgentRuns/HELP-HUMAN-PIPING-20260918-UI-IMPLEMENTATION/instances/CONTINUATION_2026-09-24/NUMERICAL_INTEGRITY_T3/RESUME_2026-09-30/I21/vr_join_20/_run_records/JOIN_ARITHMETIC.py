"""One-off VR kernel/caller join arithmetic; fixed metadata + integer bounds only.
No H caller/profile transplanted. No production parser/model/graph/source encoder/solver executes.
Arithmetic operators/phases reuse reviewed h_numeric19 kernel equations; VR input owners below replace H ones.
"""
import pathlib,json,sys
R=pathlib.Path(sys.argv[1]);out=pathlib.Path(sys.argv[2])
caller=json.loads((out/'CORRECTED_CALLER_TABLE.json').read_text())
input_rows={x['id']:x for x in json.loads((out/'_run_records/INPUT_BINDING.json').read_text())['rows']}
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
records=[];joined=[]
for callrow in caller['rows']:
 c=callrow['shape'];model=c['id'];a=input_rows[model]['actual_VR_metadata']
 N=c['N'];m=c['m'];n=c['n'];f=c['f'];r=c['r'];l=c['l'];t=c['t'];B=c['B'];b=c['b'];z=c['z'];h=c['h_W1_unused_for_sparse'];q=c['q']
 assert a['s']==a['d']==a['u']==0 and a['prescribed_operand_child_terms']==0
 assert n==6*N and f==n-r and q==7*N+12*m+6*t+r
 ids=c['load_id_bytes'];src_len=c['X'];v=min(l,n);J=68*v;F=q-7*N;U=78*m;Pi=144*m
 assert ids==a['load_id_bytes'] and src_len==38+24*N+84*m+13*r+17*l+ids+16*t
 # Actual VR source_parts: exact nodes, pushed member/constraint/load/station arrays; exact cloned IDs.
 src0=24*N+G(88,m)+G(16,r)+G(40,l)+G(16,t)+16*n+4*N+ids
 clone=24*N+88*m+16*r+40*l+16*t+16*n+4*N+ids
 assert src0==callrow['requested']['details']['source_retained']==callrow['moving']['details']['source_retained']
 stf=26+24*N+84*m+5*r;led=10+18*v+8*J
 geom=24*B+8*(4*B+4*N);geomcopy=24*B+16*N
 pattern=8*(n+1)+A(8,z)+8*z+8*(z+1)+8*U
 ordering=G(8,f)+8*n+24*f
 blocks=4*f+G(24,b)+8*(4*b+2*f)+4*b
 prep=432+clone+48*v+v+8*J+48*r+A(1,src_len)+G(20,q)+8*B
 group=368+geom+pattern+ordering+blocks
 call=1464+80+A(1,stf)
 base=src0+prep+group+call+geomcopy+3328+64
 sort=max(Sort(88,m),Sort(16,r),Sort(40,l),Sort(16,t))
 constructorR=src0+max(sort,3*2144,12*N)
 constructorM=src0+max(sort,3*2144,12*N,O(88,m),O(16,r),O(40,l),O(16,t))
 assert constructorR==callrow['requested']['details']['source_constructor']
 assert constructorM==callrow['moving']['details']['source_constructor']
 # The pre-call constructor is an independent temporal caller phase, excluded from pure solve phase max.
 gp=r+6*N
 geom_extra=G(4,N)+TI(N,240)+24*N+G(8,gp)+G(24,N)+G(48,gp)+48*gp+G(48,2*N+5)+G(192,N)+N*(3*G(8,10)+3*G(8,1))+3*G(8,2)+G(8,10)+G(48,N)+80+G(8,11)
 geom_old=max(O(4,N),O(8,gp),O(24,N),O(48,gp),O(48,2*N+5),O(192,N),O(48,N),O(8,2*N),G(8,11)//2)
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
  S[p]=720+m*Mop[L]+z*(wl+wr)+m*coeff[RR]+h*wl+f*(48+pivot[L])+(b*wl if p>=256 else 0)
  UU[p]=104+(n+6*m+q)*wl
  if p>=256:V[p]=va[L]+z*wl+144*m*wr+b*block[L]
 all_schedules={}
 for schedule,ps in [('all',[128,256,512,1024])]:
  vs=[p for p in ps if p>=256]
  K=base+sum(S[p]+UU[p] for p in ps)+sum(V[p]+40*B+G(24,2*b)+24*b for p in vs)
  phases={};details={}
  def put(name,val,parts=None):
   phases[name]={'requested_bytes':val[0],'moving_bytes':val[1],'active_old_upper':val[1]-val[0]}
   if parts is not None:details[name]=parts
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
   # Actual VR cases.rs513-517 sets value0; verify.rs802-803 skips it. Keep50n headers/flags.
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
 full=all_schedules['all']
 # Common stationary returned-union upper; finite Kpad/finish padding, no Report/Decision duplicated as retained.
 kout=max(full['Kpad'],full['selected_return_upper'])
 assert kout<=full['kernel_requested']
 k={'id':model,'VR_shape':c,'VR_input_specific':a,'SRC0_VR':src0,'PREP_source_clone':clone,'PREP':prep,'GROUP':group,'CALL':call,'BASE':base,'Kpad':full['Kpad'],'S':S,'U':UU,'V':V,
 'precall_constructor_separate':{'requested':constructorR,'moving':constructorM},'kernel_solve_requested':full['kernel_requested'],'kernel_solve_moving':full['kernel_moving'],
 'kernel_outcome_retained_upper':kout,'return_selected_upper':full['selected_return_upper'],'return_refused_upper':full['Kpad'],'return_unresolved_upper':full['Kpad'],
 'outcome_padding':'Finite source06 Kpad and source05 finish-child padding retained as upper, not exact liveness. No complete solve maximum substituted in the outcome joins; no active kernel realloc after return.',
 'full_solve_padding_alternative_not_used':full['kernel_requested'],'phases':full['phases'],'details':full['details'],'dominant_kernel_requested':full['dominant_requested'],'dominant_kernel_moving':full['dominant_moving']}
 records.append(k)
 jr={'id':model,'members':m,'input_identity':input_rows[model],'kernel_interface':{key:k[key] for key in ['SRC0_VR','kernel_solve_requested','kernel_solve_moving','kernel_outcome_retained_upper','return_selected_upper','return_refused_upper','return_unresolved_upper','outcome_padding']}}
 # solve caller addends carry no active grow while kernel executes.
 assert callrow['requested']['kernel_phase_addends']['solve_max']==callrow['moving']['kernel_phase_addends']['solve_max']
 for metric in ['requested','moving']:
  pieces={}
  for name,value in callrow[metric]['caller_only_phases'].items():
   pieces['caller_only:'+name]={'caller_addend':value,'kernel_addend':0,'total':value,'kernel_absence':'Kernel outcome already dropped/not yet called at this source phase; not an unresolved symbol set to zero'}
  for name,value in callrow[metric]['kernel_phase_addends'].items():
   kval=k['kernel_solve_'+metric] if name=='solve_max' else kout
   pieces['kernel_join:'+name]={'caller_addend':value,'kernel_addend':kval,'total':checked(value+kval),'kernel_slot':'solve_'+metric if name=='solve_max' else 'stationary returned union'}
  maximum=max(vv['total'] for vv in pieces.values())
  jr[metric]={'phases':pieces,'max_bytes':maximum,'dominant_phases':[name for name,vv in pieces.items() if vv['total']==maximum]}
 assert jr['moving']['max_bytes']>=jr['requested']['max_bytes']
 for name,vv in jr['requested']['phases'].items():assert jr['moving']['phases'][name]['total']>=vv['total']
 jr['binary_half_heap_backstop_bytes']=4026531840
 jr['binary_backstop_consequence']='deferred_by_this_conditional_upper' if jr['moving']['max_bytes']>4026531840 else 'inequality_pass_only_not_runner_admission'
 jr['unresolved_mathematical_inputs']={}
 jr['scope']='Complete conditional VR global source-upper candidate, not Emax/admission/final artifact acceptance. Final source/type/library/features/request/launch/normal-error premises retained.'
 joined.append(jr)
def ranges(x):
 if isinstance(x,bool):return
 if isinstance(x,int):assert 0<=x<(1<<63),x
 elif isinstance(x,dict):
  for v in x.values():ranges(v)
 elif isinstance(x,list):
  for v in x:ranges(v)
ranges(records);ranges(joined)
(out/'KERNEL_PHASES.json').write_text(json.dumps({'basis':'40129a225d73860ac2a53da9a2fa73869df668f3','units':'requested bytes','H_caller_terms_included':False,'rows':records},indent=2)+'\n')
(out/'JOINED_PHASES.json').write_text(json.dumps({'basis':'40129a225d73860ac2a53da9a2fa73869df668f3','overlay_replacements_applied':96,'global_window':'vk_scale original global summary through actual final reads','rows':joined},indent=2)+'\n')
print(json.dumps({'cases':len(records),'kernel_phase_rows':sum(len(x['phases']) for x in records),'five_interfaces':len(joined)*2*5,'joined_phase_metric_rows':sum(len(x[m]['phases']) for x in joined for m in ['requested','moving']),'largest_requested':max(x['requested']['max_bytes'] for x in joined),'largest_moving':max(x['moving']['max_bytes'] for x in joined),'backstop_deferral_ids':[x['id'] for x in joined if x['moving']['max_bytes']>4026531840]},indent=2))
