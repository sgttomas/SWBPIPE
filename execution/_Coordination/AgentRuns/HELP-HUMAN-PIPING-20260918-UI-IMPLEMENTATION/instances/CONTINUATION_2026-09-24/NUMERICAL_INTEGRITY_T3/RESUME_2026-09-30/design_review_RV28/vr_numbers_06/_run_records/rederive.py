"""Independent integer substitution from reviewed source equations.
Does not import/execute the author's calculator or product/source algorithms.
K6C OUT arguments; fixed JSON is read only as metadata.
"""
from pathlib import Path
import json,hashlib,os,subprocess,re,sys
K,OUT=map(Path,sys.argv[1:]); WT=K.parent;raw=OUT/'_run_records'
T='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3';R=T+'/RESUME_2026-09-30';P=R+'/metric_design_10_vr_numbers';REV='40129a225d73860ac2a53da9a2fa73869df668f3'
checks=[];mismatch=[];basis=[];cmds=[]
def digest(b):return hashlib.sha256(b).hexdigest()
def rd(p):
 b=(K/p).read_bytes();basis.append({'origin':'<K6C>','path':p,'sha256':digest(b),'bytes':len(b)});return b
def jo(p):return json.loads(rd(p))
def ck(n,a,b=True):
 checks.append(n)
 if a!=b:mismatch.append({'check':n,'actual':a,'expected':b})
def git(p):
 x=subprocess.run(['git','show',REV+':'+p],cwd=K,env=dict(os.environ,GIT_OPTIONAL_LOCKS='0'),capture_output=True);assert x.returncode==0
 cmds.append({'cwd':str(K),'argv':['git','show',REV+':'+p],'exit':x.returncode,'GIT_OPTIONAL_LOCKS':'0','stdout_sha256':digest(x.stdout),'stderr':x.stderr.decode()});basis.append({'origin':'Git object','revision':REV,'path':p,'sha256':digest(x.stdout),'bytes':len(x.stdout)});return x.stdout
seal=rd(P+'/SHA256SUMS');ck('candidate seal',digest(seal),'a5390504ba0198ebe280877b75d8b374a38c266e780a7e3dc0d7a280f1fb9a9c')
for line in seal.decode().splitlines():
 h,f=line.split(maxsplit=1);ck('candidate payload '+f,digest((K/P/f).read_bytes()),h)
candidate=jo(P+'/VR_CALLER_TABLE.json'); proposed={x['id']:x for x in candidate['rows']}
schema=jo(R+'/I21/source_15/SCHEMAS.json')['schemas'];counts={x['model']:x for x in jo(R+'/I21/k0_assembly_16/SUBSTITUTIONS.json')['rows']}
launches=jo(P+'/LAUNCHES.json');launch_choice=jo(P+'/_run_records/LAUNCH_CHOICE.json')
public=jo(P+'/_run_records/PUBLIC_SIZES.json')['values'];ck('public substitutions',public,{'constraint':16,'set_header':24,'formed':1168,'control_pair':24,'floor_pair':40,'coupling':24,'chunk_ref':16,'string_ref':8,'vec_f64':24})
for p in ['source_review_RV30/vr_nodes_17/RETURN.md','source_review_RV30/public_layout_result_22/RETURN.md','source_review_RV30/format_stream_18/RETURN.md','source_review_RV30/static_pools_20/RETURN.md','source_review_RV30/wrapped_errors_19/RETURN.md','metric_design_03_exact/DERIVATION.md','metric_design_04_sparse/DERIVATION.md','metric_design_04_sparse/DESCRIPTORS.md','metric_design_07_sparse_correction/CORRECTION.md','metric_design_09_prefix_correction/CORRECTION.md','I21/source_11/FORMULAS_AND_GAPS.md','I21/source_15/SERIALIZER_ENVELOPE.md','I21/k0_assembly_16/OWNER_LEDGER.md']:
 rd(R+'/'+p)
for p in ['src/lane.rs','src/cases.rs','src/records.rs','src/compare.rs','examples/vk_scale.rs']:
 git('projects/chirality-piping/validation/benchmarks/numerical_robustness/'+p)
def minimum(s):return 8 if s==1 else (4 if s<=1024 else 1)
def pushed(s,k):return 0 if k==0 else s*max(minimum(s),1<<(k-1).bit_length())
def old(s,k):return 0 if k<=minimum(s) else pushed(s,k)//2
def arbitrary(s,k):return 0 if not k else s*max(minimum(s),2*k)
def sortmem(s,k):return s*max(48,k) if k>=2 else 0
def fmt(k):return max(8,2*k) if k else 0
node={'json':(632,728),'case':(544,640),'borrowed':(280,376),'floor':(456,552),'published':(848,944),'allow':(368,464),'adj':(104,200),'pair':(104,200)}
def insert(kind,k):
 l,i=node[kind];return 0 if not k else l if k<12 else (1+(k-1)//5)*max(l,i)
def bulk(kind,k):
 if not k:return 0
 l,i=node[kind];z=l*(1+k//12);power=12
 while power<=k:z+=i*(1+k//(12*power));power*=12
 return z
def jtree(x):
 if isinstance(x,str):return len(x.encode()),0
 if isinstance(x,list):
  sub=[jtree(v) for v in x];return pushed(32,len(x))+sum(v[0] for v in sub),max([old(32,len(x))]+[v[1] for v in sub])
 if isinstance(x,dict):
  sub=[jtree(v) for v in x.values()];return insert('json',len(x))+sum(len(k.encode()) for k in x)+sum(v[0] for v in sub),max([0]+[v[1] for v in sub])
 return 0,0
def model(m,clone=False):
 n=len(m['nodes']);children=sum(len(x[0].encode()) for k in ['nodes','members','springs'] for x in m[k])+sum(len(x.encode()) for x in m['omitted_springs'])+sum(len(x[3].encode()) for x in m['loads'])
 mem=(48*n if clone else 2*pushed(24,n))+112*len(m['members'])+88*len(m['springs'])+24*len(m['omitted_springs'])+16*len(m['constraints'])+48*len(m['loads'])+16*len(m['stations'])+children
 return mem,0 if clone else old(24,n)
def caseheap(c):
 r=sum(len(c.get(k,'').encode()) for k in ['id','family','basis','units','k4src_sha256'])+len(c.get('model_sha256') or '')
 r+=96*len(c['rows'])+sum(len(v.encode()) for row in c['rows'] for v in row if isinstance(v,str))
 r+=64*len(c['controls'])
 for ident,_,kind,payload in c['controls']:
  r+=len(ident.encode())
  r+=48*len(payload)+sum(len(k.encode())+len(v.encode()) for k,v in payload.items()) if kind=='value' else len(payload['defect'].encode())
 r+=24*len(c['not_covered'])+sum(len(v.encode()) for v in c['not_covered']);extra=0;move=0
 for name in ['scales','s_full']:
  obj=c.get(name) or {};k=len(obj);tree=bulk('case',k);r+=tree+sum(len(a.encode())+len(b.encode()) for a,b in obj.items());extra=max(extra,48*k+max(sortmem(48,k),tree)-tree)
 if c['model'] is not None:
  h,o=model(c['model']);r+=h;move=o
 return r,r+extra,r+max(extra,move)
def encode(x,e):return arbitrary(1,x)+e*x
def hashing(x,e):
 d=((x+72)//64)*64;msg=max(8,2*d);return msg+144 if not e else msg+max(d,208)
def readfile(x,e):return (max(8,2*(x+32)) if x else 0)+e*x
def obj(name,child=0):
 s=schema[name];return insert('json',s['count'])+s['key_bytes']+child
def output(name,child=0,with_phase=True,extra=0):
 s=schema[name];n=4 if with_phase else 0
 return insert('json',s['count']+n+1)+s['key_bytes']+(45 if with_phase else 0)+17+child+(insert('json',4) if with_phase else 0)+max(extra,97)
def recbound(c,b,e):
 I=len(c['id']);family=len(c['family']);geometry=2+23*b;S=obj('stage');W=obj('verification');Tally=obj('tally');attempt=obj('attempt',25+171+38+2*S+obj('storage'))
 shared=17+I+family+64+128+4*attempt+3*W+Tally
 ret=max(obj('selected_record_root',shared+16+geometry),obj('other_record_root',shared+100+geometry),obj('source_refused',17+I+family+64+89+Tally))
 dout=100+geometry;formatpeak=max(fmt(89)+e*89,fmt(89)+fmt(dout)+e*dout,fmt(dout)+dout,fmt(geometry)+geometry,fmt(171)+171)
 return ret,ret+max(S,W,Tally,formatpeak),dout
def rcm(f,edges,e,levels):
 neighbors=56*f+32*edges;base=neighbors+17*f
 qr=pushed(8,max(1,f-1)) if f else 0;qo=old(8,max(1,f-1)) if f else 0;reachold=0 if f<=1 else 8 if f<=4 else old(8,f)
 return base+max(e*old(8,2*max(0,f-1)),sortmem(8,max(0,f-1)),f+pushed(8,f)+qr+e*max(qo,reachold),f+levels*pushed(8,f)+e*old(8,f),qr+e*qo)
def adjacency(n,f,edges,e):
 tree=(f+edges//5)*200 if edges else 0;ret=80*f+16*edges
 return ret,9*n+24*f+tree+ret+arbitrary(8,12)+e*max(96,8*max(0,f-1))
def sparse(d,e):
 N,m,n,f,r,z=[d[k] for k in ['N','m','n','f','r','z']];C=144*m;zf=min(f*f,z);hs=f*(f+1)//2;k=min(r,n)
 B=pushed(136,m)+8*n+pushed(8,f)+16*r
 HB=B+pushed(32,N)+8*r+max(sortmem(16,r),e*old(32,N),e*old(136,m),e*old(8,f))
 Con=pushed(24,C);I=insert('allow',z);Neighbors=56*N+64*m;Rows=56*n+16*z;Pat=8*(n+1)+arbitrary(8,z)+8*z
 HP=Pat+Neighbors+n+Rows+e*max(32*m,8*n,8*z);Kg=Pat+8*z;HA=1184*m+max(HP,Kg)
 W=B+Con+I+Kg+2*pushed(8,z)+8*n
 front=max(HB,B+Con+e*old(24,C),B+Con+2*I,B+Con+I+HA,W+e*old(8,z))
 Sums=64*z+16*C;Diff=80*z+16*C;HSums=Sums+pushed(8,m)+e*old(8,m);HDiff=Diff+pushed(8,m+1)+e*old(8,m+1)
 Round=pushed(96,z)+8*(2*C+z);RoundCopy=96*z+8*(2*C+z)
 Audit=max(HSums,Sums+HDiff,Sums+Diff+Round+e*old(96,z),Sums+Diff+Round+pushed(16,k)+e*old(16,k))
 textlen=len("V-K parity: the structural adapter's formation allowances")
 Core=4*f+8*(f+1)+3*pushed(8,zf)+8*zf+16*f+textlen;Prep=Core+Round
 Row=max(pushed(16,min(f,n))+e*max(old(16,min(f,n)),old(8,zf)),pushed(24,k)+e*old(24,k))
 Prepare=max(n,4*f+8*n,Core+8*n+max(Row,Audit))
 Entries=arbitrary(24,zf);Adj=56*f+16*zf;Profile=max(f,24*f+8*(f+1)+arbitrary(8,hs)+e*8*hs)
 Order=max(Entries+e*24*zf,Entries+Adj+e*old(8,max(0,f-1)),Entries+Adj+rcm(f,zf,e,6),Entries+Adj+8*f+Profile)
 Fct=40*f+8*hs+pushed(48,f);Pivot=48*f;Factor=max(n+16*f,Fct+41*f+e*old(48,f))
 RR=pushed(104,f);Residual=max(n,RR+e*old(104,f));kr=1+24*m
 Intended=max(HSums,Sums+RR+max(2*pushed(8,kr)+e*old(8,kr),pushed(8,kr)+e*old(104,f)))
 Finish=max(n,40*f,16*f,8*f+8*n+Residual,8*f+8*n+RR+pushed(8,f)+e*old(8,f),24*f+8*n+RR+pushed(8,f),8*f+8*n+RR+Intended,8*f+8*n+2*RR+4*f+RoundCopy+textlen)
 Solver=max(Prepare,Prep+Order,Prep+Factor,Prep+Fct+Pivot+Finish,Prep+max(n,8*f+8*n))
 Solution=8*n+4*f+Pivot+2*RR+RoundCopy+textlen
 total=max(front,W+Solver,W+max(Solution,8*n)+200+e*100,front+66144+e*33072)
 return dict(front=front,W=W,prep=Prep,prepare=Prepare,order=Order,factor=Factor,finish=Finish,solver=Solver,solution=Solution,max=total,hs_upper=hs,factor_values=8*hs)

# Exact envelope tuples: bits, length, capacity, exponent2 interval, exponent10 interval.
def S(bits,a,A,b,B,cap=None):
 l=(bits+31)//32;return (bits,l,l if cap is None else cap,a,A,b,B)
def mem(x):return 4*x[2]
def copy(x,da=0,db=0):return S(x[0],x[3]+da,x[4]+da,x[5]+db,x[6]+db)
def decimal(text):
 m=re.fullmatch(r'([+-]?)([0-9]*)(?:\.([0-9]*))?(?:[eE]([+-]?[0-9]+))?',text);assert m
 ds=m[2]+(m[3] or '');j=int(m[4] or 0)-len(m[3] or '');bits=0 if set(ds)<=set('0') else 4*len(ds)
 return S(bits,0,0,j,j,pushed(4,(bits+31)//32)//4)
def parseheap(text,e):
 m=re.fullmatch(r'([+-]?)([0-9]*)(?:\.([0-9]*))?(?:[eE]([+-]?[0-9]+))?',text);d=len(m[2]+(m[3] or ''));chunks=(d+8)//9;l=(4*d+31)//32
 return max(fmt(d)+e*d,fmt(d)+arbitrary(16,chunks)+e*16*chunks,fmt(d)+arbitrary(16,chunks)+pushed(4,l)+e*4*l)
def lift(x,k,j,e):
 if not x[0]:return S(0,0,0,0,0),0
 bits=x[0]+k;l=(bits+31)//32;w=k//32;needed=w+x[1]+bool(k);c=max(w,4,2*needed)
 cb=l if j==0 else max(l,4,2*((bits+4*j+31)//32));hb=4*cb+e*(4*((bits+4*j+31)//32) if j else 0)
 return S(bits+4*j,0,0,0,0,cb),max(4*c+e*4*needed,4*c+hb)
def align(x,y,e):
 a=min(x[3],y[3]);b=min(x[5],y[5]);xx,hx=lift(x,max(0,x[4]-a),max(0,x[6]-b),e);yy,hy=lift(y,max(0,y[4]-a),max(0,y[6]-b),e)
 return xx,yy,max(hx,mem(xx)+hy),(a,max(x[4],y[4]),b,max(x[6],y[6]))
def compare(x,y,e):return align(x,y,e)[2]
def plus(x,y,e):
 a,b,h,exps=align(x,y,e);c=max(a[1],b[1])+1;return S(max(a[0],b[0])+1,*exps,c),max(h,mem(a)+mem(b)+4*c)
def minus(x,y,e):
 ny=copy(y);s,h=plus(x,ny,e);return s,mem(ny)+h
def times(x,y,e):
 exps=(x[3]+y[3],x[4]+y[4],x[5]+y[5],x[6]+y[6])
 if x[0]==0 or y[0]==0:return S(0,*exps,0),0
 q=x[1]+y[1]+1;c=max(2*q,4);return S(x[0]+y[0],*exps,c),8*q+4*c+e*4*q
def mx(x,y,e):
 z=S(max(x[0],y[0]),min(x[3],y[3]),max(x[4],y[4]),min(x[5],y[5]),max(x[6],y[6]),max(x[1],y[1]));return z,max(compare(x,y,e),mem(z))
def tolerance(x,y,e):
 a=copy(x);m,h=mx(a,y,e);t=copy(m,db=-9);return t,max(mem(a)+h,mem(a)+mem(m)+mem(t))
F64=S(53,-1074,971,0,0,2)
def predicate(o,x,y,e):
 d,hd=minus(o,x,e);da=copy(d);t,ht=tolerance(x,y,e)
 return max(hd,mem(d)+mem(da),mem(d)+mem(da)+ht,mem(d)+mem(da)+mem(t)+compare(da,t,e))
def magnitude(x,y,e):
 p,hp=times(F64,F64,e);q,hq=times(F64,F64,e);v,hv=plus(p,q,e);t,ht=tolerance(x,y,e);u,hu=plus(x,t,e);u2,hu2=times(u,u,e);l,hl=minus(x,t,e);l2,hl2=times(l,l,e)
 return 16+max(hp,mem(p)+hq,mem(p)+mem(q)+hv,mem(v)+ht,mem(v)+mem(t)+hu,mem(v)+mem(t)+mem(u)+max(hu2,mem(u2)+compare(v,u2,e)),mem(v)+mem(t)+mem(u)+hl,mem(v)+mem(t)+mem(u)+mem(l)+max(hl2,mem(l2)+compare(v,l2,e)))
def floorhelp(x,y,e):
 a=copy(x);c,h=mx(a,y,e);f=copy(F64,da=-34);return max(mem(a)+h,mem(c)+8+mem(f),mem(c)+mem(f)+compare(c,f,e))
def rangehelp(x,e):
 a=copy(x);tiny=S(1,-1075,-1075,0,0,2);big=S(1,1024,1024,0,0,2);v=S(1,970,970,0,0,2);huge,h=minus(big,v,e)
 return max(mem(a),mem(a)+24+h,mem(a)+8+mem(huge)+max(compare(a,tiny,e),compare(a,huge,e)))
def exact_helpers(c,e):
 rowmax=ctrlmax=held=0;rows={r[0]:r for r in c['rows']}
 def operands(r):
  a=r[1];b=r[3] if isinstance(r[3],str) else c['scales'][r[2]];return a,b,decimal(a),decimal(b)
 for r in c['rows']:
  a,b,x,y=operands(r);live=mem(x)+mem(y);held=max(held,live)
  rowmax=max(rowmax,parseheap(a,e),mem(x)+parseheap(b,e),live+floorhelp(x,y,e),live+8+predicate(F64,x,y,e),live+magnitude(x,y,e),live+rangehelp(x,e))
 for ctl in c['controls']:
  if ctl[2]!='value':continue
  for key,o in ctl[3].items():
   a,b,x,y=operands(rows[key]);z=decimal(o);ctrlmax=max(ctrlmax,parseheap(a,e),mem(x)+parseheap(b,e),mem(x)+mem(y)+parseheap(o,e),mem(x)+mem(y)+mem(z)+predicate(z,x,y,e))
 return rowmax,ctrlmax,held

family=git('projects/chirality-piping/validation/benchmarks/numerical_robustness/cases/rf_large.jsonl');cases=[json.loads(x) for x in family.splitlines()];assert len(cases)==24
external={x['id']:x for x in jo(R+'/I23/external_inputs_06/INPUTS.json')['entries']};models={};external_data={}
for c in cases:
 if c['model'] is not None:models[c['id']]=c['model']
 else:
  src=external[c['id']];p=WT/src['raw_path'].removeprefix('<wt>/');data=p.read_bytes();ck('external '+c['id'],digest(data),src['expected']['model_sha256']);models[c['id']]=json.loads(data);external_data[c['id']]=(len(data),jtree(models[c['id']]))
casebounds=[caseheap(c) for c in cases];rawbounds=[jtree(c) for c in cases]
manifest=launch_choice['VR_CARGO_MANIFEST_DIR'];fam_path=manifest+'/cases/rf_large.jsonl';expected_path=manifest+'/cases/expected_unresolved.json';binary=launch_choice['VR_binary']
Paths=[len(manifest)+arbitrary(1,len(manifest+'/cases'))+arbitrary(1,len(fam_path))+e*len(fam_path) for e in [0,1]]
Family=[]
for e in [0,1]:
 done=0;h=0
 for typed,rawj in zip(casebounds,rawbounds):
  r,tr,tm=typed;j,o=rawj;h=max(h,done+max(j+e*o,j+(tm if e else tr),r+e*old(472,24)));done+=r
 Family.append(Paths[e]+max(readfile(len(family),e),max(8,2*(len(family)+32))+pushed(472,24)+h))
expected=git('projects/chirality-piping/validation/benchmarks/numerical_robustness/cases/expected_unresolved.json');jexpect,oexpect=jtree(json.loads(expected))
results=[]
for ci,c in enumerate(cases):
 ident=c['id'];expectedrow=proposed[ident];d=expectedrow['shape'];m=models[ident];Kc=counts[ident]
 for field,actual in [('N',len(m['nodes'])),('m',len(m['members'])),('r',len(m['constraints'])),('l',len(m['loads'])),('t',len(m['stations']))]:ck(ident+' descriptor '+field,d[field],actual)
 for field in ['n','f','z','q','B','b']:ck(ident+' count '+field,d[field],Kc[field])
 N,n,f,z,q,B,mm,r=[d[k] for k in ['N','n','f','z','q','B','m','r']];ids=sum(len(t[3].encode()) for t in m['loads']);X=38+24*N+84*mm+13*r+17*d['l']+ids+16*d['t'];ck(ident+' encoding length',X,d['X'])
 I=len(ident);FM=len(c['family']);ext=c['model'] is None;mp=str(WT/external[ident]['raw_path'].removeprefix('<wt>/')) if ext else None
 av=[binary,'--case',ident,'--heap-cap-bytes','8053063680']+(['--model-file',mp] if ext else []);cv=[binary,'--case',ident,'--heap-cap-bytes','536870912']+(['--model-file',mp] if ext else [])+['--counts-only'];argv=max(24*len(v)+sum(len(x.encode()) for x in v) for v in [av,cv])
 lc=launches['rows'][ci];ck(ident+' argv normal lengths',list(map(len,av)),lc['argument_utf8_lengths']);ck(ident+' argv counts lengths',list(map(len,cv)),lc['counts_argument_utf8_lengths'])
 args=I+(len(mp) if ext else 0);M,Mo=model(m,clone=not ext);C=casebounds[ci][0];base=1700+args+I+C+M+(128 if ext else 0);cut=base+128
 source=24*N+pushed(88,mm)+pushed(16,r)+pushed(40,d['l'])+pushed(16,d['t'])+16*n+4*N+ids;parts=source-16*n-4*N
 rows=len(c['rows']);controls=len(c['controls']);maxkey=max(len(x[0]) for x in c['rows']);maxexpected=max(len(x[1]) for x in c['rows']);sumkey=sum(len(x[0]) for x in c['rows']);names=sum(len(x[0]) for x in m['members']);edges=min(f*(f-1),144*mm)
 for e,metric in [(0,'requested'),(1,'moving')]:
  cand=expectedrow[metric];detail=cand['details'];exactrow,exactcontrol,held=exact_helpers(c,e);rec,hr,dout=recbound(c,B,e);sp=sparse(d,e)
  radj,hadj=adjacency(n,f,edges,e);hprofile=max(rcm(f,edges,e,4),24*f+pushed(8,f)+e*old(8,f));countsH=max(hadj,radj+hprofile,pushed(8,f)+e*old(8,f),pushed(8,f)+N+insert('pair',2*mm),pushed(8,f)+pushed(20,q)+e*old(20,q),pushed(8,f)+pushed(20,q)+encode(X,e))
  sourceH=source+max(sortmem(88,mm),sortmem(16,r),sortmem(40,d['l']),sortmem(16,d['t']),3*2144,12*N,e*max(old(88,mm),old(16,r),old(40,d['l']),old(16,d['t'])))
  why=max(9,40,75,maxkey+18);Df=max(I+maxkey+maxexpected+6+why,I+maxkey+94,I+93,I+56,I+36,I+39+dout,I+75,I+20,I+16+dout);nf=rows+4
  controlstrings=sum(fmt(I+1+len(x[0])) for x in c['controls']);ctdiag=2*pushed(24,controls)+controlstrings
  diag=ctdiag+pushed(24,nf)+nf*fmt(Df)+pushed(24,rows)+sumkey+2*pushed(24,2*rows)+2*rows*(fmt(I+maxkey+92)+fmt(I+maxkey+1))
  floorR=insert('floor',mm)+names;floorH=names+max(40*mm+sortmem(40,mm),40*mm+bulk('floor',mm));pub=insert('published',q)
  details={'cut_survivors':cut,'source_retained':source,'source_constructor':sourceH,'count_helpers':countsH,'family_load_common':Family[e],'model_standalone':M,'case_retained':C,'Exact_row_max':exactrow,'Exact_control_max':exactcontrol,'record_retained':rec,'record_build':hr,'diagnostics':diag,'published_map':pub,'floor_map':floorR,'sparse':sp,'launch_bytes':argv}
  for key,v in details.items():ck(ident+' '+metric+' '+key,v,detail[key])
  # Phase arithmetic below is recomposed from the checked identities, not read
  # from proposed phase values. Owner completeness is assessed in REVIEW.md.
  load=1700+args+I+Family[e]
  if ext:
   fl,(jr,jo)=external_data[ident];mr,mo=model(m);eh=max((len(mp)+1 if len(mp)>=384 else 0),readfile(fl,e),max(8,2*(fl+32))+jr+e*jo,max(8,2*(fl+32))+jr+mr+e*mo,max(8,2*(fl+32))+jr+mr+hashing(fl,e));load=max(load,1700+args+I+C+eh)
  else:load=max(load,1700+args+I+C+M)
  ds=I+len(': source refused: ')+75;dh=max(I+len(': K4SRC sha256 ')+64+len(", the generator's ")+64,I+len(': model file sha256 ')+64+len(', committed ')+64)
  prefix=max(1700+argv,load,base+output('start',5+I+(128 if ext else 0)),base+sourceH,base+source+max(encode(X,e),arbitrary(1,X)+hashing(X,e)),cut+source+countsH,base+sourceH+max(fmt(ds)+e*ds,fmt(ds)+output('fail',5+ds,False)),cut+source+max(fmt(dh)+e*dh,fmt(dh)+output('fail',5+dh,False)))
  lookup=insert('borrowed',rows);cb=max(24*rows+sortmem(24,rows),24*rows+bulk('borrowed',rows));dn=I+1+max([0]+[len(x[0]) for x in c['controls']]);control=max(cb,lookup+ctdiag+max(exactcontrol,e*old(24,controls),fmt(dn)+e*dn))
  nc=pushed(24,rows)+sumkey if c.get('s_full') is not None else 0;run=diag+pub+rec+I+FM+16;late=cut+78+rec+nc
  reportchild=6+I+32*min(50,nf)+min(50,nf)*Df+32*rows+sumkey+32*len(c['not_covered'])+sum(len(x) for x in c['not_covered'])+(32*rows+sumkey if c.get('s_full') is not None else 0)+64*rows+2*rows*(I+maxkey+92)+32*controls+sum(I+1+len(x[0]) for x in c['controls'])
  phases={'prefix_before_cut':prefix,'postcut_counts_output':cut+output('counts',6+I+64),'counts_only_summary_sample':cut+632+(54 if e else 32)+I+11,'half_cap_refusal_output':cut+output('half_cap_refusal',7+I+len('estimate_exceeds_half_cap'),False),'sourceparts_controls':cut+parts+control,'lane_source_constructor':cut+ctdiag+sourceH,'lane_encoding_hash':cut+ctdiag+source+max(encode(X,e),arbitrary(1,X)+hashing(X,e)),'source_refused_caller_only':cut+parts+ctdiag+max(fmt(91)+e*91,fmt(I+maxkey+94)+e*(I+maxkey+94),diag+hr+225),'record2_w1_output':cut+78+run+rec+output('w1',2+I+dout),'full_floor':cut+78+run+rec+max(floorH,floorR+nc+max(exactrow,e*old(24,rows))),'report':cut+78+run+rec+nc+8*min(50,nf)+output('report',reportchild),'record3_output':cut+78+run+rec+nc+output('record_wrapper',6+I+rec,False),'late_RCM':max(late+max(hadj,radj+rcm(f,edges,e,4),radj+8*f+rcm(f,edges,e,6),radj+16*f),late+output('rcm',3+I)),'late_sparse_parity':max(late+sp['max'],late+66144+output('binary64_error',8+I+33072),late+200+output('binary64_success',8+I+100)),'summary_sample':late+632+(54 if e else 32)+I}
  for key,v in phases.items():ck(ident+' '+metric+' phase '+key,v,cand['caller_only_phases'][key])
  expectedPaths=len(manifest)+arbitrary(1,len(manifest+'/cases'))+arbitrary(1,len(expected_path))+e*len(expected_path);expH=expectedPaths+max((len(expected_path)+1 if len(expected_path)>=384 else 0),readfile(len(expected),e),max(8,2*(len(expected)+32))+jexpect+max(e*oexpect,78,120 if e else 80))
  dg=max(old(24,nf),old(24,2*rows),old(24,rows),old(24,controls),Df,I+maxkey+92);dp=max(exactrow,375 if e else 300,held+fmt(Df)+e*Df+max(fmt(why),75),held+e*dg);comparison=max(diag+pub+16*N,diag+pub+4*N+floorH,diag+pub+4*N+floorR+dp)
  slots={'solve_max':cut+ctdiag+arbitrary(1,X)+pushed(24,4)+4*fmt(Df),'selected_or_other_outcome_comparison':cut+arbitrary(1,X)+78+comparison,'record1_build_while_outcome_alive':cut+arbitrary(1,X)+78+diag+pub+hr+I+FM+16,'expected_list_initialization_while_outcome_alive':cut+ctdiag+arbitrary(1,X)+expH,'nonselected_diagnostic_while_outcome_alive':cut+arbitrary(1,X)+diag+fmt(dout)+fmt(I+39+dout)+e*(I+39+dout)}
  for key,v in slots.items():ck(ident+' '+metric+' kernel addend '+key,v,cand['kernel_phase_addends'][key])
  ck(ident+' '+metric+' maximum',max(phases.values()),cand['caller_only_max']);ck(ident+' '+metric+' dominant',max(phases,key=phases.get),cand['dominant_caller_phase'])
  # Explicit formerly omitted pre-list failures are bounded by existing
  # comparison slot with the SAME Kernel_outcome_retained added to either.
  priorfails=pushed(24,4)+4*fmt(Df);ck(ident+' '+metric+' prelist dominance',slots['expected_list_initialization_while_outcome_alive']+priorfails<=slots['selected_or_other_outcome_comparison'])
  results.append({'id':ident,'metric':metric,'details':details,'phases':phases,'kernel_addends':slots,'prelist_prior_failure_upper':priorfails,'prelist_dominance_margin':slots['selected_or_other_outcome_comparison']-slots['expected_list_initialization_while_outcome_alive']-priorfails})
raw.mkdir(parents=True,exist_ok=True)
(raw/'BASIS.json').write_text(json.dumps({'source_records':basis,'candidate_seal':digest(seal),'scope':'Independent source-equation substitution; no author calculator import/execution'},indent=2)+'\n');(raw/'COMMANDS.json').write_text(json.dumps(cmds,indent=2)+'\n')
(raw/'CHECKS.json').write_text(json.dumps({'checks':len(checks),'mismatches':mismatch,'status':'PASS' if not mismatch else 'MISMATCH'},indent=2)+'\n')
(raw/'RECOMPUTED.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps({'checks':len(checks),'mismatches':mismatch[:8],'mismatch_count':len(mismatch),'cases':len(cases),'metric_rows':len(results),'caller_max':max(x['phases'][max(x['phases'],key=x['phases'].get)] for x in results)}))
