"""Independent RV28 additive backcheck. stdlib only; no designer test imports.
Usage: <VENV>/bin/python -B backcheck.py A1 OUT
"""
from fractions import Fraction as Q
from pathlib import Path
import hashlib,json,os,platform,subprocess,sys
A1,OUT=map(Path,sys.argv[1:])
T='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3'
R=T+'/RESUME_2026-09-30'
FK='projects/chirality-piping/core/solver/frame_kernel'
SRC='d01ad98a754698631f927709d08284c272de85e8'
def digest(b):return hashlib.sha256(b).hexdigest()
seals=[]
for rel,expected in [(R+'/design_a1','080de38cd614012a7eba50b28b12e52ac122fb8246747e77f88fa9a82ab6c34c'),(R+'/design_a1/addendum_01','d1593a77f61ff7bd76e91248cfa224be01f0aaaaa75e2e677ff311ec5f370151'),(R+'/design_review_RV28','fb371fe7ff060940ea7314a4f64ac68763eb0fd67c336eed49cc4d4517f49b12')]:
    p=A1/rel/'SHA256SUMS';assert digest(p.read_bytes())==expected
    entries=[]
    for line in p.read_text().splitlines():
        h,f=line.split(maxsplit=1);f=f.lstrip('*');p1=p.parent/f
        if not p1.exists():p1=A1/f
        assert digest(p1.read_bytes())==h,f
        entries.append(f)
    seals.append({'path':str(Path(rel)/'SHA256SUMS'),'sha256':expected,'entries_checked':len(entries)})
correction=A1/R/'design_a1/addendum_01/CORRECTION.md'
assert digest(correction.read_bytes())=='30c907589e80a60407acc139a5805d9941b5df1a5b9dd5a2f1dc845159bae80a'
basis=[];commands=[]
for rel in [FK+'/src/structural/retained/adaptive.rs',FK+'/src/structural/retained/wide_sum.rs',FK+'/tests/retained_k4/models.rs','projects/chirality-piping/core/rules/rule_check_runner/src/lib.rs',T+'/DESIGN_NUMERICS/DESIGN.md',T+'/DESIGN_STANDING/DESIGN.md']:
    p=subprocess.run(['git','show',SRC+':'+rel],cwd=A1,env=dict(os.environ,GIT_OPTIONAL_LOCKS='0'),capture_output=True)
    assert p.returncode==0,p.stderr
    assert p.stdout==(A1/rel).read_bytes(),rel
    basis.append({'path':rel,'origin':'<A1_WT>','revision':SRC,'sha256':digest(p.stdout),'bytes':len(p.stdout)})
    commands.append({'argv':['git','show',SRC+':'+rel],'cwd':'<A1_WT>','GIT_OPTIONAL_LOCKS':'0','exit':p.returncode,'stdout_sha256':digest(p.stdout),'stderr':p.stderr.decode()})
def two(e):return Q(2)**e
h=two(-1074);MAX=(2-two(-52))*two(1023);EPS=two(-64);U=two(-53)
def exp(x):
    e=x.numerator.bit_length()-x.denominator.bit_length()
    return e-1 if x<two(e) else e
def rn(x):
    if not x:return Q(0)
    if x<0:return -rn(-x)
    quantum=two(max(-1074,exp(x)-52));a=x/quantum
    n,r=divmod(a.numerator,a.denominator)
    n+=int(2*r>a.denominator or (2*r==a.denominator and n%2))
    out=n*quantum
    return out # values above MAX represent overflow and are explicitly refused
def up(x):
    assert x>=0
    if x>MAX:return None
    if not x:return Q(0)
    quantum=two(max(-1074,exp(x)-52));a=x/quantum
    n,r=divmod(a.numerator,a.denominator)
    return (n+bool(r))*quantum
def nextup(y):
    if y<0:return -nextdown(-y)
    if y==MAX:return None
    if not y:return h
    return y+two(max(-1074,exp(y)-52))
def nextdown(y):
    if y<0:
        r=nextup(-y);return None if r is None else -r
    if not y:return -h
    e=exp(y)
    return y-two(max(-1074,e-53 if y==two(e) else e-52))
def outward(lo,hi,scale):
    return nextdown(rn(lo*scale)),nextup(rn(hi*scale))
checks=[]
def show(v):
    if isinstance(v,Q):
        if v.denominator&(v.denominator-1)==0:return str(v.numerator)+' * 2^'+str(1-v.denominator.bit_length())
        return str(v)
    return v
def check(name,ok,**details):
    assert ok,name
    checks.append({'name':name,'pass':True,**{k:show(v) for k,v in details.items()}})
for H in [Q(0),h/7,h,h*Q(7,5),1-two(-80),Q(1),1+two(-80),two(1023),MAX]:
    r=up(H)
    check('RU64 bound '+str(H),r is not None and H<=r and (H==0 or nextdown(r)<H),H=H,r=r)
check('finite overflow refusal',up(MAX+1) is None)
check('strictly positive underflow becomes h',up(two(-1200))==h)
for b in [h,2*h,two(-100),Q(1),Q(3,2),two(100),MAX]:
    for H in [Q(0),b/3,b-two(-1600),b]:
        check('monotonic finite upper '+str(b)+' '+str(H),0<=H<=up(H)<=b)
def allowance(z,S):
    z=abs(z);m=max(z,S)
    exact=EPS*m*(1+two(-21))+U*z+h
    f64=rn(rn(rn(EPS*m)*(1+two(-21)))+rn(rn(U*z)+h))
    return exact,f64
found={}
for z in [Q(1),1+two(-52),1+two(-51),Q(3,2),two(-1022),3*two(-1022)]:
    ae,af=allowance(z,z)
    if ae!=af:
        key='exact_smaller' if ae<af else 'binary64_smaller'
        found.setdefault(key,(z,ae,af))
for key,(z,ae,af) in found.items():
    middle=(ae+af)/2
    check('mandatory min discriminates '+key,middle<=max(ae,af) and middle>min(ae,af),z=z,A_exact=ae,A_f64=af)
assert found
# A certified non-dyadic relative limit can lie below the stored dyadic RU.
z=Q(1);H=z/Q(10**9);r=up(H)
check('private RU can exceed exact decimal bound',r>H)
check('minimum of simultaneous certified limits remains upper',H<=min(r,H)<=r)
maps={'m':Q(1),'rad':Q(1),'N':Q(1),'N*m':Q(1),'Pa':Q(1),'mm':Q(1,1000),'kN':Q(1000),'kN*m':Q(1000),'MPa':Q(1000000)}
for unit,a in maps.items():
    x=Q(1)+two(-52);radius=two(-62)
    y=rn(x/a);n=rn(a*y);truth=x+radius
    hp=abs(a*y-x)+radius;hn=abs(n-a*y)+hp;hu=hp/a;tight=abs(n-x)+radius
    check('coordinate chain '+unit,abs(a*y-truth)<=hp and abs(y-truth/a)<=hu and abs(n-truth)<=hn and abs(n-truth)<=tight,raw=y,normalized=n,e_pub=abs(a*y-x),e_norm=abs(n-a*y))
    lo,hi=outward(n-hn,n+hn,Q(1)/a)
    check('interval rule-unit conversion '+unit,lo<=truth/a<=hi)
# Raw/public predicate has distinct coordinates; this intentionally isolates
# that predicate and does not claim an all-gates source witness.
y=Q(1);a=Q(1,1000);n=rn(a*y);truth=n*(1+Q(1,10**9))
check('normalized public predicate alone does not establish raw claim',abs(n-truth)*10**9<=abs(n) and abs(y-truth/a)*10**9>abs(y))
check('mm division differs from rounded reciprocal multiplication',any(rn(y/Q(1000))!=rn(y*rn(Q(1,1000))) for y in [Q(1)+i*two(-52) for i in range(1,100)]))
y=Q(1000);n=Q(1);b=Q(1,1024);truth=Q(2001,2000)
check('wrong-unit raw interval is too narrow',abs(n-truth)<=b and abs(y-truth*1000)>b)
lo,hi=outward(n-b,n+b,Q(1000))
check('SI binding then raw-unit outward interval encloses',lo<=truth*1000<=hi)
n=Q(1);lo,hi=outward(n,n,Q(1,1000))
check('zero-SI-bound conversion needs interval',lo<Q(1,1000)<hi and rn(Q(1,1000))!=Q(1,1000))
check('zero initial point retains exact source identity',n==Q(1))
# Symmetric display bound includes center displacement and exact unit map.
for y in [Q(1),1+two(-52),h]:
    a=Q(1,1000);n=rn(a*y);b=h if n==0 else two(-60)*abs(n)
    r=up((b+abs(n-a*y))/a)
    check('raw-centered display radius compensation '+str(y),r is not None and y-r<=(n-b)/a and y+r>=(n+b)/a)
# Sentinel is a type-level absence, not an admissible floating error bound.
sentinel=0x7ff0000000000000
def valid(bits,eligible):
    if not eligible:return bits==sentinel
    return (bits>>63)==0 and ((bits>>52)&2047)!=2047
check('sentinel class discrimination',valid(sentinel,False) and not valid(sentinel,True) and valid(0,True) and not valid(0,False))
check('negative/nonfinite patterns rejected',all(not valid(i,True) for i in [1<<63,0xbff0000000000000,0x7ff8000000000000,0xfff0000000000000]))
OUT.mkdir(parents=True,exist_ok=True)
(OUT/'EXACT_CHECKS.json').write_text(json.dumps({'status':'PASS','count':len(checks),'checks':checks,'boundary':'Algebraic backcheck and modeled predicate discriminators only; no implementation/mutant execution.'},indent=2)+'\n')
(OUT/'BASIS.json').write_text(json.dumps({'agent':'/root/rv28_a1_design','parent':'/root','role':'TASK','mechanism':'collaboration.followup_task','correction_sha256':digest(correction.read_bytes()),'source_records':basis,'preserved_seals':seals,'instruction_basis':'Original sealed RV28 BASIS.json; same active session and bounded role/skill','runtime':{'version':sys.version,'system':platform.system(),'machine':platform.machine(),'executable_sha256':digest(Path(sys.executable).read_bytes())}},indent=2)+'\n')
(OUT/'COMMANDS.json').write_text(json.dumps(commands,indent=2)+'\n')
print(json.dumps({'status':'PASS','checks':len(checks),'sealed_packets_preserved':len(seals),'source_files':len(basis),'relative_rounding_directions_found':list(found)}))
