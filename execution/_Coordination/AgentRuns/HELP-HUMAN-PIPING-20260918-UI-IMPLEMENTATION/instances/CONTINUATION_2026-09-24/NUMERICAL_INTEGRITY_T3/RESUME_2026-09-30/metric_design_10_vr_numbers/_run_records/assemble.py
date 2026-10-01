# One-off source-bound integer substitution record. Not a product estimator/tool.
# Reads fixed JSON metadata only. Does not run Rust, graphs, models, solver,
# Exact/Nat predicates, serialization, parsing algorithms, or generated cases.
from pathlib import Path
import json,hashlib,os,subprocess,re,datetime
WT=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3')
K=WT/'k6c'; R='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30'
OUT=K/R/'metric_design_10_vr_numbers'; REV='40129a225d73860ac2a53da9a2fa73869df668f3'
SZ=json.loads((OUT/'_run_records/PUBLIC_SIZES.json').read_text())['values']
EXPECT_PATH=OUT/'_run_records/EXPECTED_PARSE_BOUND.json'
EXPECT=json.loads(EXPECT_PATH.read_text()) if EXPECT_PATH.is_file() else None
def sz(k):
    v=SZ[k]; assert isinstance(v,int) and v>0; return v
ENV=dict(os.environ,GIT_OPTIONAL_LOCKS='0'); INPUTS=[]
def gitread(p):
    x=subprocess.run(['git','show',REV+':'+p],cwd=K,env=ENV,capture_output=True,check=True)
    INPUTS.append({'origin':'<K6C> git '+REV+':'+p,'sha256':hashlib.sha256(x.stdout).hexdigest(),'bytes':len(x.stdout),'exit':x.returncode})
    return x.stdout

def mu(s):return 8 if s==1 else 4 if s<=1024 else 1
def P(s,k):return 0 if k==0 else max(mu(s),1<<(k-1).bit_length())
def G(s,k):return s*P(s,k)
def O(s,k):return 0 if k<=mu(s) else G(s,k)//2
def A(s,k):return 0 if k==0 else s*max(mu(s),2*k)
def Sort(s,k):return 0 if k<2 else s*max(k,48)
def F(d):return max(8,2*d) if d else 0
def TI(k,leaf,internal):return 0 if not k else leaf if k<=11 else (1+(k-1)//5)*max(leaf,internal)
NODES={'json':(632,728),'control':(280,376),'published':(848,944),'floor':(456,552),'allow':(368,464),'case':(544,640),'set':(104,200),'pairset':(104,200)}
def T(kind,k):return TI(k,*NODES[kind])
def TB(kind,k):
    if not k:return 0
    leaf,inter=NODES[kind];ret=(1+k//12)*leaf;v=12
    while k>=v:ret+=(1+k//(v*12))*inter;v*=12
    return ret

def Jin(v):
    if isinstance(v,str):return len(v.encode()),0
    if isinstance(v,list):
        a=[Jin(x) for x in v];return G(32,len(v))+sum(x[0] for x in a),max([O(32,len(v))]+[x[1] for x in a])
    if isinstance(v,dict):
        a=[Jin(x) for x in v.values()];return T('json',len(v))+sum(len(x.encode()) for x in v)+sum(x[0] for x in a),max([0]+[x[1] for x in a])
    return 0,0

def model_bytes(m,clone=False):
    N=len(m['nodes']);nm=len(m['members']);ns=len(m['springs']);nr=len(m['constraints']);nl=len(m['loads']);nt=len(m['stations'])
    chars=sum(len(str(x[0]).encode()) for key in ['nodes','members','springs'] for x in m[key])+sum(len(x.encode()) for x in m['omitted_springs'])+sum(len(x[3].encode()) for x in m['loads'])
    r=(48*N if clone else 2*G(24,N))+112*nm+88*ns+24*len(m['omitted_springs'])+sz('constraint')*nr+48*nl+16*nt+chars
    return r,0 if clone else O(24,N),chars

def case_bytes(c):
    r=sum(len(c.get(x,'').encode()) for x in ['id','family','basis','units','k4src_sha256'])+len((c.get('model_sha256') or '').encode())
    rows=c['rows'];ctrl=c['controls'];r+=96*len(rows)+sum(sum(len(x.encode()) for x in row if isinstance(x,str)) for row in rows)
    r+=64*len(ctrl)
    for cc in ctrl:
        r+=len(cc[0].encode())
        if cc[2]=='value':r+=48*len(cc[3])+sum(len(k.encode())+len(v.encode()) for k,v in cc[3].items())
        else:r+=len(cc[3]['defect'].encode())
    r+=24*len(c['not_covered'])+sum(len(x.encode()) for x in c['not_covered'])
    build_extra=0;grow=0
    for field in ['scales','s_full']:
        mp=c.get(field) or {};m=len(mp);nodes=TB('case',m)
        r+=nodes+sum(len(k.encode())+len(v.encode()) for k,v in mp.items())
        build_extra=max(build_extra,48*m+max(Sort(48,m),nodes)-nodes)
    if c['model'] is not None:
        rm,om,_=model_bytes(c['model']);r+=rm;grow=max(grow,om)
    return r,r+build_extra,r+max(build_extra,grow)

def enc(X,e):return A(1,X)+e*X
def hashh(L,e):
    D=64*((L+9+63)//64);M=max(8,2*D)
    return M+144 if not e else max(M+D,M+208)
def readh(L,e):return (max(8,2*(L+32)) if L else 0)+e*L

def qret(f):return G(8,max(1,f-1)) if f else 0
def qold(f):return O(8,max(1,f-1)) if f else 0
def rold(f):return 0 if f<=1 else 8 if f<=4 else O(8,f)
def rcm_helper(f,edge_sum,e,six):
    # Aggregate neighbor padding: sum G(8,2d_i)<=32*edge_sum+32*f.
    neighbors=24*f+32*edge_sum+32*f
    base=neighbors+8*f+f+8*f
    reach=f+G(8,f)+qret(f)+e*max(rold(f),qold(f))
    ecc=f+(6 if six else 4)*G(8,f)+e*O(8,f)
    return base+max(e*O(8,2*max(0,f-1)),Sort(8,max(0,f-1)),reach,ecc,qret(f)+e*qold(f))

def adj_terms(n,f,edge_sum,e):
    # per-vertex node population bound uses d_i<=f-1 and aggregate ceiling.
    # Sum node counts <= f+edge_sum//5 (safe ceiling using floor inequality).
    nodes=(f+edge_sum//5)*200 if edge_sum else 0
    r=24*f+f*sz('set_header')+16*edge_sum+32*f # safe possible inherited outer bytes
    h=9*n+f*sz('set_header')+nodes+r+A(8,12)+e*max(96,8*max(0,f-1))
    return r,h,nodes

# Arithmetic state envelopes, not execution of Exact values.
def state(B,p2lo,p2hi,p10lo,p10hi,c=None):
    l=(B+31)//32;return {'B':B,'l':l,'c':l if c is None else c,'a':p2lo,'A':p2hi,'b':p10lo,'B10':p10hi}
def rr(x):return 4*x['c']
def clone(x,da=0,db=0):return state(x['B'],x['a']+da,x['A']+da,x['b']+db,x['B10']+db,x['l'])
def dec(s):
    m=re.fullmatch(r'([+-]?)([0-9]*)(?:\.([0-9]*))?(?:[eE]([+-]?[0-9]+))?',s);assert m
    ds=m[2]+(m[3] or '');j=int(m[4] or '0')-len(m[3] or '');B=0 if all(x=='0' for x in ds) else 4*len(ds)
    return state(B,0,0,j,j,P(4,(B+31)//32))
def lift(x,ka,kb,e):
    if x['B']==0:return state(0,0,0,0,0),0
    B=x['B']+ka;ll=(B+31)//32;w=ka//32;K=w+x['l']+(1 if ka else 0);cs=max(w,4,2*K)
    Hsh=4*cs+e*4*K
    B2=B+4*kb;L2=(B2+31)//32;cp=ll if not kb else max(ll,4,2*L2)
    Hp=4*cp+(e*4*L2 if kb else 0)
    return state(B2,0,0,0,0,cp),max(Hsh,4*cs+Hp)
def align(x,y,e):
    amin=min(x['a'],y['a']);bmin=min(x['b'],y['b'])
    xx,hx=lift(x,max(0,x['A']-amin),max(0,x['B10']-bmin),e)
    yy,hy=lift(y,max(0,y['A']-amin),max(0,y['B10']-bmin),e)
    return xx,yy,max(hx,rr(xx)+hy),amin,max(x['A'],y['A']),bmin,max(x['B10'],y['B10'])
def cmpcost(x,y,e):return align(x,y,e)[2]
def add(x,y,e):
    a,b,h,lo,hi,jlo,jhi=align(x,y,e);B=max(a['B'],b['B'])+1;c=max(a['l'],b['l'])+1
    return state(B,lo,hi,jlo,jhi,c),max(h,rr(a)+rr(b)+4*c)
def sub(x,y,e):
    ny=clone(y);ret,h=add(x,ny,e);return ret,rr(ny)+h
def mul(x,y,e):
    if not x['B'] or not y['B']:return state(0,x['a']+y['a'],x['A']+y['A'],x['b']+y['b'],x['B10']+y['B10'],0),0
    q=x['l']+y['l']+1;cap=max(2*q,4)
    return state(x['B']+y['B'],x['a']+y['a'],x['A']+y['A'],x['b']+y['b'],x['B10']+y['B10'],cap),8*q+4*cap+e*4*q
def xmax(x,y,e):
    ret=state(max(x['B'],y['B']),min(x['a'],y['a']),max(x['A'],y['A']),min(x['b'],y['b']),max(x['B10'],y['B10']),max(x['l'],y['l']))
    return ret,max(cmpcost(x,y,e),rr(ret))
def tol(x,y,e):
    ax=clone(x);mx,h=xmax(ax,y,e);t=clone(mx,db=-9)
    return t,max(rr(ax)+h,rr(ax)+rr(mx)+rr(t))
def pred(o,x,y,e):
    d,hd=sub(o,x,e);ad=clone(d);t,ht=tol(x,y,e)
    return max(hd,rr(d)+rr(ad),rr(d)+rr(ad)+ht,rr(d)+rr(ad)+rr(t)+cmpcost(ad,t,e))
F64=state(53,-1074,971,0,0,2)
def mag(x,y,e):
    q1,h1=mul(F64,F64,e);q2,h2=mul(F64,F64,e);q,hq=add(q1,q2,e);t,ht=tol(x,y,e);u,hu=add(x,t,e);u2,hu2=mul(u,u,e);l,hl=sub(x,t,e);l2,hl2=mul(l,l,e)
    return 16+max(h1,rr(q1)+h2,rr(q1)+rr(q2)+hq,rr(q)+ht,rr(q)+rr(t)+hu,rr(q)+rr(t)+rr(u)+max(hu2,rr(u2)+cmpcost(q,u2,e)),rr(q)+rr(t)+rr(u)+hl,rr(q)+rr(t)+rr(u)+rr(l)+max(hl2,rr(l2)+cmpcost(q,l2,e)))
def floorcost(x,y,e):
    a=clone(x);c,h=xmax(a,y,e);f=clone(F64,da=-34)
    return max(rr(a)+h,rr(c)+8+rr(f),rr(c)+rr(f)+cmpcost(c,f,e))
def rangecost(x,e):
    a=clone(x);tiny=state(1,-1075,-1075,0,0,2);p=state(1,1024,1024,0,0,2);v=state(1,970,970,0,0,2);huge,h=sub(p,v,e)
    return max(rr(a),rr(a)+24+h,rr(a)+8+rr(huge)+max(cmpcost(a,tiny,e),cmpcost(a,huge,e)))
def parsecost(s,e):
    m=re.fullmatch(r'([+-]?)([0-9]*)(?:\.([0-9]*))?(?:[eE]([+-]?[0-9]+))?',s);ds=m[2]+(m[3] or '');d=len(ds);j=(d+8)//9;l=(4*d+31)//32;U=sz('chunk_ref')
    return max(F(d)+e*d,F(d)+A(U,j)+e*U*j,F(d)+A(U,j)+G(4,l)+e*4*l)
def exact_case(c,e):
    mxrow=0;mxctrl=0;ES=0
    bykey={r[0]:r for r in c['rows']}
    for row in c['rows']:
        ex=row[1];sc=row[3] if len(row)>3 and isinstance(row[3],str) else c['scales'][row[2]];x=dec(ex);y=dec(sc);held=rr(x)+rr(y);ES=max(ES,held)
        h=max(parsecost(ex,e),rr(x)+parsecost(sc,e),held+floorcost(x,y,e),held+8+pred(F64,x,y,e),held+mag(x,y,e),held+rangecost(x,e));mxrow=max(mxrow,h)
    for co in c['controls']:
        if co[2]!='value':continue
        for key,obs in co[3].items():
            row=bykey[key];ex=row[1];sc=row[3] if len(row)>3 and isinstance(row[3],str) else c['scales'][row[2]];x=dec(ex);y=dec(sc);z=dec(obs)
            h=max(parsecost(ex,e),rr(x)+parsecost(sc,e),rr(x)+rr(y)+parsecost(obs,e),rr(x)+rr(y)+rr(z)+pred(z,x,y,e));mxctrl=max(mxctrl,h)
    return mxrow,mxctrl,ES

# JSON finite output shape algebra; 'cost expressions' are source envelope numbers.
SC=json.loads((K/R/'I21/source_15/SCHEMAS.json').read_text())['schemas']
def obj(name,children=0,extra_keys=0,extra_keybytes=0):
    z=SC[name];return T('json',z['count']+extra_keys)+z['key_bytes']+extra_keybytes+children

def outputphase(name,children=0,phase=True,extra=0):
    z=SC[name];k=z['count'];keys=z['key_bytes'];shift=4 if phase else 0;pk=45 if phase else 0
    # complete augmented object prepayment plus consumed Phase nodes and handled error.
    return T('json',k+shift+1)+keys+pk+17+children+(T('json',4) if phase else 0)+max(extra,97)

def record_terms(c,B,e):
    I=len(c['id']);fm=len(c['family']);geom=2+23*B;S=obj('stage');U=obj('storage');W=obj('verification');tally=obj('tally')
    # Complete selected root/4 attempts/3 verification slots; upper branch text all variants.
    attemptbase=obj('attempt',25+171+38+2*S+U)
    rsel=obj('selected_record_root',17+I+fm+64+16+geom+128+4*attemptbase+3*W+tally)
    rother=obj('other_record_root',17+I+fm+64+100+geom+128+4*attemptbase+3*W+tally)
    rsource=obj('source_refused',17+I+fm+64+89+tally)
    ret=max(rsel,rother,rsource)
    # Explicit one temporary stage/verification subtree or one finite format source;
    # complete output ret padding means no later subtree is paid twice as helper.
    d_out=100+geom
    hformat=max(F(89)+e*89,F(89)+F(d_out)+e*d_out,F(d_out)+d_out,F(geom)+max(e*geom,geom),F(171)+max(171,e*171))
    hbuild=ret+max(S,W,tally,hformat)
    return ret,hbuild,geom,d_out

def sparse(c,d,e):
    N,m,n,f,r,z=d['N'],d['m'],d['n'],d['f'],d['r'],d['z'];C=144*m;zf=min(f*f,z);L=zf;hs=f*(f+1)//2;tau=m;Tr=12*m;k=min(r,n);j=min(f,n)
    B=G(136,m)+8*n+G(8,f)+16*r
    HB=B+G(32,N)+8*r+max(Sort(16,r),e*O(32,N),e*O(136,m),e*O(8,f))
    Con=G(24,C);I=T('allow',z)
    Neighbor=24*N+64*m+32*N;Rows=24*n+16*z+32*n;Pat=8*(n+1)+A(8,z)+8*z
    HPat=Pat+Neighbor+n+Rows+e*max(8*4*m,8*n,8*z)
    Kg=Pat+8*z;Form=sz('formed')*m+16*m
    HA=Form+max(HPat,Kg)
    W=B+Con+I+Kg+2*G(8,z)+8*n
    Hfront=max(HB,B+Con+e*O(24,C),B+Con+2*I,B+Con+I+HA,W+e*O(8,z))
    sums=32*z+16*C+32*z
    hsums=sums+G(8,tau)+e*O(8,tau)
    diff=32*z+16*C+48*z
    hdiff=diff+G(8,tau+1)+e*O(8,tau+1)
    rnd=G(96,z)+8*(2*C+z);rndclone=96*z+8*(2*C+z)
    haudit=max(hsums,sums+hdiff,sums+diff+rnd+e*O(96,z),sums+diff+rnd+G(16,k)+e*O(16,k))
    basis=len("V-K parity: the structural adapter's formation allowances")
    pcore=4*f+8*(f+1)+3*G(8,zf)+8*zf+16*f+basis;prep=pcore+rnd
    rowprep=max(G(16,j)+e*max(O(16,j),O(8,zf)),G(sz('coupling'),k)+e*O(sz('coupling'),k))
    hprep=max(n,4*f+8*n,pcore+8*n+max(rowprep,haudit))
    entries=A(24,L);adj=24*f+16*zf+32*f
    hrcm=rcm_helper(f,zf,e,True)
    hprof=max(f,24*f+8*(f+1)+A(8,hs)+e*8*hs)
    horder=max(entries+e*24*L,entries+adj+e*O(8,max(0,f-1)),entries+adj+hrcm,entries+adj+8*f+hprof)
    fct=sz('vec_f64')*f+8*hs+16*f+G(48,f);pivotcopy=48*f
    hfactor=max(n+16*f,fct+16*f+f+24*f+e*O(48,f))
    rr_=G(104,f);res=max(n,rr_+e*O(104,f));Krow=1+2*Tr
    intended=max(hsums,sums+rr_+max(2*G(8,Krow)+e*O(8,Krow),G(8,Krow)+e*O(104,f)))
    finish=max(n,max(n,40*f),16*f,8*f+8*n+res,8*f+8*n+rr_+G(8,f)+e*O(8,f),8*f+8*n+rr_+G(8,f)+16*f,8*f+8*n+rr_+intended,8*f+8*n+2*rr_+4*f+rndclone+basis)
    hsolver=max(hprep,prep+horder,prep+hfactor,prep+fct+pivotcopy+finish,prep+max(n,8*f+8*n))
    sol=8*n+4*f+pivotcopy+2*rr_+rndclone+basis
    # Returned typed class may retain solution/direction; handled errors included.
    hclass=200+e*100
    h=max(Hfront,W+hsolver,W+max(sol,8*n)+hclass)
    # Early returned frame error may coexist with any constructor prefix; padding
    # this explicit text destination to Hfront is conservative, not a generic factor.
    h=max(h,Hfront+66144+e*33072)
    return {'front':Hfront,'W':W,'prep':prep,'prepare':hprep,'order':horder,'factor':hfactor,'finish':finish,'solver':hsolver,'solution':sol,'max':h,'hs_upper':hs,'factor_values':8*hs}

familyraw=gitread('projects/chirality-piping/validation/benchmarks/numerical_robustness/cases/rf_large.jsonl');assert hashlib.sha256(familyraw).hexdigest()=='16357afa5efeaaac3ab5798bb6f632104bec1e2dfada288f5b0936bfe6188759'
CASES=[json.loads(x) for x in familyraw.splitlines() if x];assert len(CASES)==24
EXT={x['id']:x for x in json.loads((K/R/'I23/external_inputs_06/INPUTS.json').read_text())['entries']}
MODELS={};RAWJSON={};METADATA=[]
for c in CASES:
    if c['model'] is not None:MODELS[c['id']]=c['model']
    else:
        x=EXT[c['id']];path=WT/x['raw_path'].removeprefix('<wt>/');b=path.read_bytes();assert hashlib.sha256(b).hexdigest()==x['expected']['model_sha256'];MODELS[c['id']]=json.loads(b);RAWJSON[c['id']]=(len(b),Jin(MODELS[c['id']]));INPUTS.append({'origin':x['raw_path'],'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b)})
COUNT={x['model']:x for x in json.loads((K/R/'I21/k0_assembly_16/SUBSTITUTIONS.json').read_text())['rows']}
for c in CASES:
    mid=c['id'];m=MODELS[mid];h=COUNT[mid];assert not m['springs'];assert len(m['nodes'])==h['N'] and len(m['members'])==h['m']
    dd={'N':len(m['nodes']),'m':len(m['members']),'r':len(m['constraints']),'l':len(m['loads']),'t':len(m['stations']),**{v:h[v] for v in ['n','f','z','q','B','b']},'h_W1_unused_for_sparse':h['h_W1']}
    dd['load_id_bytes']=sum(len(a[3].encode()) for a in m['loads']);dd['X']=38+24*dd['N']+84*dd['m']+13*dd['r']+17*dd['l']+dd['load_id_bytes']+16*dd['t'];METADATA.append(dict(id=mid,**dd))
# Reuse exact-size/fixed-byte typed expected-list interface; inspect only metadata.
expr=gitread('projects/chirality-piping/validation/benchmarks/numerical_robustness/cases/expected_unresolved.json');EXPV=json.loads(expr);expectedJ,expectedO=Jin(EXPV)
CR=[case_bytes(c) for c in CASES];JRAW=[Jin(c) for c in CASES]
BIN=str(WT/'k6c-final-target/release/examples/vk_scale');ARCH=str(WT/'scratch/k6c_final');MAN=ARCH+'/projects/chirality-piping/validation/benchmarks/numerical_robustness';LF=MAN+'/cases/rf_large.jsonl';LE=MAN+'/cases/expected_unresolved.json'
def pathphase(e):
    l0=len(MAN.encode());l1=len((MAN+'/cases').encode());l2=len(LF.encode());return l0+A(1,l1)+A(1,l2)+e*max(l1,l2)
def cpath(L):return 0 if L<384 else L+1
Family={}
for e in [0,1]:
    prev=0;best=0
    for (ret,hr,hm),(jr,jo) in zip(CR,JRAW):
        best=max(best,prev+max(jr+e*jo,jr+(hm if e else hr),ret+e*O(472,24)));prev+=ret
    Family[e]=pathphase(e)+max(readh(len(familyraw),e),max(8,2*(len(familyraw)+32))+G(472,24)+best)
RESULT=[];LAUNCHES=[]
for idx,(c,d) in enumerate(zip(CASES,METADATA)):
    mid=c['id'];m=MODELS[mid];I=len(mid.encode());fm=len(c['family']);Rr=len(c['rows']);Cc=len(c['controls']);Nk=len(c['not_covered']);Kmax=max([0]+[len(x[0].encode()) for x in c['rows']]);Emax=max([0]+[len(x[1].encode()) for x in c['rows']]);keysum=sum(len(x[0].encode()) for x in c['rows']);names=sum(len(x[0].encode()) for x in m['members'])
    ext=c['model'] is None;modelpath=str(WT/EXT[mid]['raw_path'].removeprefix('<wt>/')) if ext else None
    argv=[BIN,'--case',mid,'--heap-cap-bytes','8053063680']+(['--model-file',modelpath] if ext else [])
    argvC=[BIN,'--case',mid,'--heap-cap-bytes','536870912']+(['--model-file',modelpath] if ext else [])+['--counts-only']
    avgr=max(24*len(v)+sum(len(a.encode()) for a in v) for v in [argv,argvC]);argsRet=I+(len(modelpath.encode()) if ext else 0)
    rm,om,_=model_bytes(m,clone=not ext);rc=CR[idx][0];base=1700+argsRet+I+rc+rm+(128 if ext else 0);cut=base+128
    N,mm,n,f,r,z,q,B=d['N'],d['m'],d['n'],d['f'],d['r'],d['z'],d['q'],d['B'];X=d['X'];Pi=144*mm;edge=min(f*max(0,f-1),Pi)
    src=24*N+G(88,mm)+G(16,r)+G(40,d['l'])+G(16,d['t'])+16*n+4*N+d['load_id_bytes']
    parts=src-16*n-4*N
    recs={};epsrows={}
    for e in [0,1]:
        exactrow,exactctrl,heldES=exact_case(c,e)
        rec,hrec,geom,dout=record_terms(c,B,e)
        why=max(9,40,75,Kmax+18)
        Dfail=max(I+Kmax+Emax+6+why,I+Kmax+94,I+93,I+56,I+36,I+39+dout,I+75,I+20,I+16+dout)
        df=Rr+4
        ctrlchildren=sum(F(I+1+len(x[0])) for x in c['controls'])
        ctrldiag=2*G(24,Cc)+ctrlchildren
        rowdiag=G(24,df)+df*F(Dfail)+G(24,Rr)+keysum+2*G(24,2*Rr)+2*Rr*(F(I+Kmax+92)+F(I+Kmax+1))
        diag=ctrldiag+rowdiag
        floorRet=T('floor',mm)+names
        floorH=names+max(sz('floor_pair')*mm+Sort(sz('floor_pair'),mm),sz('floor_pair')*mm+TB('floor',mm))
        controlH=max(sz('control_pair')*Rr+Sort(sz('control_pair'),Rr),sz('control_pair')*Rr+TB('control',Rr))
        lookupRet=T('control',Rr)
        Dcontrol=I+1+max([0]+[len(x[0]) for x in c['controls']])
        control=max(controlH,lookupRet+ctrldiag+max(exactctrl,e*O(24,Cc),F(Dcontrol)+e*Dcontrol))
        pub=T('published',q)
        ncfull=G(24,Rr)+keysum if c.get('s_full') is not None else 0
        maxdiagGrow=max(O(24,df),O(24,2*Rr),O(24,Rr),O(24,Cc),Dfail,I+Kmax+92)
        diagphase=max(exactrow,375 if e else 300,heldES+F(Dfail)+e*Dfail+max(F(why),75),heldES+e*maxdiagGrow)
        comparison=diag+pub+4*N+floorRet+max(diagphase,0)
        compbuild=max(diag+pub+16*N,diag+pub+4*N+floorH,comparison)
        # Expected-list read/parse/typed transition per reviewed prefix machinery.
        expectedPaths=len(MAN.encode())+A(1,len((MAN+'/cases').encode()))+A(1,len(LE.encode()))+e*len(LE.encode())
        expectedScratch=EXPECT['moving' if e else 'requested'] if EXPECT is not None else None
        expectedAlternatives=[e*expectedO,78]+([] if expectedScratch is None else [expectedScratch])
        expectedH=expectedPaths+max(cpath(len(LE)),readh(len(expr),e),max(8,2*(len(expr)+32))+expectedJ+max(expectedAlternatives))
        radj,hadj,_=adj_terms(n,f,edge,e)
        countprof=max(rcm_helper(f,edge,e,False),24*f+G(8,f)+e*O(8,f))
        hcounts=max(hadj,radj+countprof,G(8,f)+e*O(8,f),G(8,f)+N+T('pairset',2*mm),G(8,f)+G(20,q)+e*O(20,q),G(8,f)+G(20,q)+enc(X,e))
        sourceH=src+max(Sort(88,mm),Sort(16,r),Sort(40,d['l']),Sort(16,d['t']),3*2144,12*N,e*max(O(88,mm),O(16,r),O(40,d['l']),O(16,d['t'])))
        prefix_load=1700+argsRet+I+Family[e]
        if ext:
            flen,(jr,jo)=RAWJSON[mid];rmp,omp,_=model_bytes(m)
            external=max(cpath(len(modelpath)),readh(flen,e),max(8,2*(flen+32))+jr+e*jo,max(8,2*(flen+32))+jr+rmp+e*omp,max(8,2*(flen+32))+jr+rmp+hashh(flen,e))
            prefix_load=max(prefix_load,1700+argsRet+I+rc+external)
        else:prefix_load=max(prefix_load,1700+argsRet+I+rc+rm)
        startOut=outputphase('start',5+I+(128 if ext else 0))
        countsOut=outputphase('counts',6+I+64)
        Dsource=I+len(': source refused: ')+75
        Dhash=max(I+len(': K4SRC sha256 ')+64+len(", the generator's ")+64,I+len(': model file sha256 ')+64+len(', committed ')+64)
        prefix=max(1700+avgr,prefix_load,base+startOut,base+sourceH,base+src+max(enc(X,e),A(1,X)+hashh(X,e)),cut+src+hcounts,base+sourceH+max(F(Dsource)+e*Dsource,F(Dsource)+outputphase('fail',5+Dsource,phase=False)),cut+src+max(F(Dhash)+e*Dhash,F(Dhash)+outputphase('fail',5+Dhash,phase=False)))
        # All other numeric post-cut phase addends; kernel-return heap excluded.
        laneEncoding=A(1,X)
        kcommon=cut+ctrldiag+laneEncoding+G(24,4)+4*F(Dfail) # explicit four nonrow failure slots padded early
        kout=cut+laneEncoding+78+compbuild
        recordCaller=cut+laneEncoding+78+diag+pub+hrec+I+fm+16
        returnedRun=diag+pub+rec+I+fm+16
        phaseMap=obj('phase')
        w1=cut+78+returnedRun+rec+outputphase('w1',2+I+dout)
        reportchildren=6+I+32*min(50,df)+min(50,df)*Dfail+32*Rr+keysum+32*Nk+sum(len(x) for x in c['not_covered'])+(32*Rr+keysum if c.get('s_full') is not None else 0)+32*(2*Rr)+2*Rr*(I+Kmax+92)+32*Cc+sum(I+1+len(x[0]) for x in c['controls'])
        report=cut+78+returnedRun+rec+ncfull+sz('string_ref')*min(50,df)+outputphase('report',reportchildren)
        fullfloor=cut+78+returnedRun+rec+max(floorH,floorRet+ncfull+max(exactrow,e*O(24,Rr)))
        recordout=cut+78+returnedRun+rec+ncfull+outputphase('record_wrapper',6+I+rec,phase=False)
        late=cut+78+rec+ncfull
        hrLate=max(hadj,radj+rcm_helper(f,edge,e,False),radj+8*f+rcm_helper(f,edge,e,True),radj+16*f)
        lateRCM=max(late+hrLate,late+outputphase('rcm',3+I))
        sp=sparse(c,d,e)
        sparseTotal=max(late+sp['max'],late+66144+outputphase('binary64_error',8+I+33072),late+200+outputphase('binary64_success',8+I+100))
        summary=late+632+(54 if e else 32)+I
        # Reached clean source refusal/error emit; generic panic/allocator failure stays qualified.
        sourceRefused=cut+parts+ctrldiag+max(F(91)+e*91,F(I+Kmax+94)+e*(I+Kmax+94),diag+hrec+225)
        phases={'prefix_before_cut':prefix,'postcut_counts_output':cut+countsOut,'counts_only_summary_sample':cut+632+(54 if e else 32)+I+11,'half_cap_refusal_output':cut+outputphase('half_cap_refusal',7+I+len('estimate_exceeds_half_cap'),phase=False),'sourceparts_controls':cut+parts+control,'lane_source_constructor':cut+ctrldiag+sourceH,'lane_encoding_hash':cut+ctrldiag+src+max(enc(X,e),laneEncoding+hashh(X,e)),
                'source_refused_caller_only':sourceRefused,'record2_w1_output':w1,'full_floor':fullfloor,'report':report,'record3_output':recordout,'late_RCM':lateRCM,'late_sparse_parity':sparseTotal,'summary_sample':summary}
        kernel_slots={'solve_max':kcommon,'selected_or_other_outcome_comparison':kout,'record1_build_while_outcome_alive':recordCaller,'expected_list_initialization_while_outcome_alive':cut+ctrldiag+laneEncoding+expectedH,'nonselected_diagnostic_while_outcome_alive':cut+laneEncoding+diag+F(dout)+F(I+39+dout)+e*(I+39+dout)}
        epsrows[str(e)]={'caller_only_phases':phases,'kernel_phase_addends':kernel_slots,'unbound_source_addends':({} if EXPECT is not None else {'expected_list_initialization_while_outcome_alive':'ExpectedListParseScratch_'+('moving' if e else 'requested')}),'caller_only_max':max(phases.values()),'dominant_caller_phase':max(phases,key=phases.get),'details':{'cut_survivors':cut,'source_retained':src,'source_constructor':sourceH,'count_helpers':hcounts,'family_load_common':Family[e],'model_standalone':rm,'case_retained':rc,'Exact_row_max':exactrow,'Exact_control_max':exactctrl,'record_retained':rec,'record_build':hrec,'diagnostics':diag,'published_map':pub,'floor_map':floorRet,'sparse':sp,'launch_bytes':avgr}}
    RESULT.append({'id':mid,'members':mm,'shape':d,'qualifier':'caller numeric bound candidate plus explicit Kernel_phi slots; no full Emax or replay','requested':epsrows['0'],'moving':epsrows['1'],'coarse_binary_half_cap_bytes':4026531840,'coarse_factor_value_subterm_exceeds_backstop':epsrows['0']['details']['sparse']['factor_values']>4026531840})
    def portable(s):return s.replace(str(WT),'<WT>')
    LAUNCHES.append({'id':mid,'cwd':portable(ARCH),'CARGO_MANIFEST_DIR':portable(MAN),'normal_argv':[portable(x) for x in argv],'counts_argv':[portable(x) for x in argvC],'argument_utf8_lengths':list(map(lambda x:len(x.encode()),argv)),'counts_argument_utf8_lengths':list(map(lambda x:len(x.encode()),argvC)),'archive_path_utf8_bytes':len(ARCH.encode()),'manifest_path_utf8_bytes':len(MAN.encode()),'family_path_utf8_bytes':len(LF.encode()),'external_model_path_utf8_bytes':len(modelpath.encode()) if ext else 0,'run_status':'planned only; RF-LARGE10 comparator/count point, no invented scale tier'})
(OUT/'VR_CALLER_TABLE.json').write_text(json.dumps({'source_basis':REV,'status':'conditional caller-only numeric candidate; Kernel_phi not substituted; no full Emax','rows':RESULT,'profile':'supplied reviewed artifact/public sizes plus final ordinary-production transfer gate'},indent=2)+'\n')
noopargv=[BIN,'--noop','--heap-cap-bytes','8053063680']
noopheap=max(1700+24*len(noopargv)+sum(len(x.encode()) for x in noopargv),1700+outputphase('noop_start',5,phase=False),1700+632+50)
(OUT/'NOOP_BRANCH.json').write_text(json.dumps({'requested_and_move_source_upper':noopheap,'scoped_route':'ordinary direct entry with exact noop argv; normal/handled-I/O qualification','argv_bytes':[len(x.encode()) for x in noopargv],'kernel_allocations':False,'not_an_observed_baseline':True},indent=2)+'\n')
(OUT/'LAUNCHES.json').write_text(json.dumps({'root_freeze':'in-turn current assignment; no build/run authorization','rows':LAUNCHES,'noop_argv':['<WT>/k6c-final-target/release/examples/vk_scale','--noop','--heap-cap-bytes','8053063680']},indent=2)+'\n')
(OUT/'_run_records/INPUT_HASHES.json').write_text(json.dumps(INPUTS,indent=2)+'\n')
(OUT/'_run_records/INPUT_SHAPES.json').write_text(json.dumps(METADATA,indent=2)+'\n')
print(json.dumps({'rows':len(RESULT),'caller_requested_max':max(x['requested']['caller_only_max'] for x in RESULT),'caller_moving_max':max(x['moving']['caller_only_max'] for x in RESULT),'dominant':sorted(set(x['moving']['dominant_caller_phase'] for x in RESULT)),'finished':datetime.datetime.now(datetime.timezone.utc).isoformat()},indent=2))
