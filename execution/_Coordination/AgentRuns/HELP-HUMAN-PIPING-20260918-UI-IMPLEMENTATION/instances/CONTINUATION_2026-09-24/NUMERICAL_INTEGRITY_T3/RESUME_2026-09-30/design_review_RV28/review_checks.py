"""RV28 independent, stdlib-only design checks; no solver or designer oracle.
Invocation: python -B review_checks.py APP COORD A1 OUT
All outputs are governed review evidence, not production code.
"""
from fractions import Fraction as F
from pathlib import Path
import hashlib, json, os, platform, subprocess, sys

APP, COORD, A1, OUT = map(Path, sys.argv[1:])
T = 'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3'
R = T + '/RESUME_2026-09-30'
FK = 'projects/chirality-piping/core/solver/frame_kernel'
SRC = 'd01ad98a754698631f927709d08284c272de85e8'
REC = 'a6b40d2d036acac556e28f28f5b482a4adb39333'
CTL = '30fd4848b4f61c4fd9405e382b82462048fff8a3'
env = dict(os.environ, GIT_OPTIONAL_LOCKS='0')
commands = []
def git(root, *args):
    p = subprocess.run(['git', *args], cwd=root, env=env, capture_output=True)
    commands.append({'cwd': '<COORD>' if root == COORD else '<A1_WT>', 'argv': ['git', *args], 'exit': p.returncode, 'stdout_sha256': hashlib.sha256(p.stdout).hexdigest(), 'stderr': p.stderr.decode()})
    if p.returncode: raise RuntimeError(p.stderr.decode())
    return p.stdout
def sha(b): return hashlib.sha256(b).hexdigest()
basis=[]
def record(root, alias, path, rev=None):
    b=(root/path).read_bytes()
    match=None if rev is None else b == git(root, 'show', rev+':'+path)
    basis.append({'origin':alias,'path':path,'revision':rev,'sha256':sha(b),'bytes':len(b),'matches_revision':match})
    assert match is not False, path
for path in ['AGENTS.md','agents/AGENT_TASK.md','projects/chirality-piping/AGENTS.md','.agents/skills/software-code-review/SKILL.md']:
    record(APP,'<APP_WORKTREE>',path)
for path in [R+'/BRIEFS/COMMON.md',R+'/BRIEFS/RV28_A1_DESIGN_REVIEW.md',R+'/PLAN.md']:
    record(COORD,'<COORD>',path,CTL)
for suffix in ['src/structural/retained/adaptive.rs','src/structural/retained/verify.rs','src/structural/retained/recover.rs','src/structural/retained/bound.rs','src/structural/retained/wide/multi.rs','src/structural/retained/wide_sum.rs','tests/retained_k4/models.rs']:
    record(A1,'<A1_WT>',FK+'/'+suffix,SRC)
for path in [T+'/DESIGN_NUMERICS/DESIGN.md',T+'/DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md',T+'/ROOT_RULINGS_V1.md',T+'/DESIGN_STANDING/DESIGN.md',R+'/I22/c_01/RETURN.md',R+'/I22/c_01/C17_INPUT.json',R+'/oracle_fresh/addendum_02_C17/CONFIRMATION.json']:
    record(A1,'<A1_WT>',path,REC)
record(A1,'<A1_WT>',R+'/design_a1/DESIGN_PROPOSAL.md')
assert basis[-1]['sha256']=='926dea73178b0ecde07203fdf7fc85e5ce752b1a5f6ead7cf31a30673249b178'
seal=A1/R/'design_a1/SHA256SUMS'
assert sha(seal.read_bytes())=='080de38cd614012a7eba50b28b12e52ac122fb8246747e77f88fa9a82ab6c34c'
seal_count=0
for line in seal.read_text().splitlines():
    digest, path=line.split(maxsplit=1)
    path=path.lstrip('*')
    p=seal.parent/path
    if not p.exists(): p=A1/path
    assert sha(p.read_bytes())==digest, path
    seal_count+=1
trees={rev:git(A1,'rev-parse',rev+':'+FK).decode().strip() for rev in [SRC,'3bddc2b05f6106e969c7cf43373b230845c7cc66',REC]}
assert len(set(trees.values()))==1
checks=[]
def chk(name, result, **values):
    assert result, name
    def val(v):
        if isinstance(v,F):
            if v.denominator & (v.denominator-1)==0: return str(v.numerator)+' * 2^'+str(-(v.denominator.bit_length()-1))
            return str(v)
        return v
    checks.append({'name':name,'pass':True,**{k:val(v) for k,v in values.items()}})
def two(e): return F(2)**e
h,eps=two(-1074),two(-64)
def exponent(x):
    k=x.numerator.bit_length()-x.denominator.bit_length()
    return k-1 if x < two(k) else k
def rnd(x,p=53,minimum=-1074):
    if not x:return F(0)
    sg=-1 if x<0 else 1
    x=abs(x)
    q=exponent(x)-(p-1)
    if minimum is not None:q=max(q,minimum)
    z=x/two(q); n,rem=divmod(z.numerator,z.denominator)
    if 2*rem>z.denominator or (2*rem==z.denominator and n%2):n+=1
    return sg*n*two(q)
def ru(x):
    if not x:return F(0)
    y=rnd(x)
    return y if y>=x else y+two(max(exponent(y)-52,-1074)) if y else h
def couple(a,L,r=lambda x:x):
    if not L:return list(a)
    tr,ro,fo,mo=a
    return [max(tr,r(L*ro)),max(ro,r(tr/L)),max(fo,r(mo/L)),max(mo,r(L*fo))]
def row_b(x,s):
    b=ru(eps*s)
    return b if s==0 or s>=two(-988) else ru(b+ru(two(-53)*abs(x))+h)
chk('rounding ties even at raw subnormals',rnd(h/2)==0 and rnd(3*h/2)==2*h and rnd(5*h/4)==h)
for j,target,L in [(1,0,two(100)),(0,1,two(-100)),(3,2,two(-100)),(2,3,two(100))]:
    c=[F(0)]*4;c[j]=5*h/4
    v=c[:];v[j]+=h/16
    x=list(map(rnd,c))
    D=[abs(a-b) for a,b in zip(v,x)]
    dc=[abs(a-b)+abs(b-z) for a,b,z in zip(v,c,x)]
    sv=couple(v,L,lambda a:rnd(a,256,None)); sx=couple(x,L,rnd)
    lip=couple(D,L)
    chk('four-direction raw loss '+str(j)+'->'+str(target),abs(sv[target]-sx[target])==lip[target] and all(a<=b for a,b in zip(D,dc)),gap=abs(sv[target]-sx[target]),relative_gap=abs(sv[target]-sx[target])/sv[target])
    chk('post-coupling h/2 insufficient '+str(target),abs(sv[target]-sx[target])>h/2)
for source,L,target in [(1,two(-100),0),(0,two(100),1),(3,two(100),2),(2,two(-100),3)]:
    a=[F(0)]*4;a[source]=h
    sv=couple(a,L);sx=couple(a,L,rnd)
    chk('coupling underflow '+str(source)+'->'+str(target),sv[target]>0 and sx[target]==0)
a=[h,2*h,3*h,4*h]
chk('L=0 omits all coupling',couple(a,0,rnd)==a)
# Different max winners; triangle bound uses the maximum per-row discrepancy.
c=[F(1),1+two(-54)];v=[1+two(-51),F(1)];x=list(map(rnd,c))
chk('max switching',v.index(max(v))!=c.index(max(c)) and abs(max(v)-max(x))<=max(abs(a-b) for a,b in zip(v,x)))
# O9 set defined by candidate publication; mathematical maxima are over that same set.
c=[h/4,h,F(0),two(1024)];v=[two(100),h,F(0),two(1024)]
J=[i for i,z in enumerate(c) if z==0 or (rnd(z)!=0 and abs(rnd(z))<two(1024))]
chk('O9 common membership includes zero excludes under/overflow',J==[1,2] and max(v[i] for i in J)==h)
for phi in [F(0),h,2*h,two(-438),F(10)]:
    a,b=3*h/2,h
    chk('same floor nonexpansive '+str(phi),abs(max(a,phi)-max(b,phi))<=abs(a-b))
for ehat in [h,two(-600),F(1),two(100)]:
    phi=ru(ehat*two(-438))
    chk('p512 positive floor/b '+str(exponent(ehat)),phi>0 and row_b(0,phi)>=h,phi=phi,b=row_b(0,phi))
S=two(-988); below=S-two(-1041)
chk('A1 discontinuity actual b bits',row_b(0,below)==eps*S+h and row_b(0,S)==eps*S)
chk('classification threshold tie',rnd(two(-34)*S)==two(-1022))
chk('zero bound iff zero certificate',row_b(0,0)==0 and h/4>row_b(0,0))
chk('C17 direct source equilibrium',((two(-900)+9*h)-two(-900))/two(-33)==9*two(-1041))
b=two(-1038);truth=9*two(-1041)
chk('C17 both radii fail',truth>b*(1+two(-22)),error_over_b=truth/b)
# Concrete pure gate discriminators; these do not claim a reachable R7 state.
for name,term in [('formation',69*two(-256)),('W',F(1,4)),('W_rounding',two(-256)),('charge',F(1,8)),('W_plus',F(1,16)),('magnitude',two(-255))]:
    base=F(1)
    chk('pure certificate omission discriminator '+name,base<=1<base+term,omitted=base,full=base+term)
chk('candidate instead of actual publication discriminator',abs(F(1)-(1+two(-54)))>0 and abs((1+two(-54))-(1+two(-54)))==0)
H=1+two(-100)
chk('rounded H unsafe',rnd(H)==1 and H>1)
chk('qualified radius differs from bare',1+two(-23)<=1+two(-22) and 1+two(-23)>1)
# Public relative decimal check; no approximated 1e-9 constant.
x=two(-34); public=x/F(10**9); sharp=eps*(1+two(-21))+two(-53)*x+h
chk('relative public and protected are separate',sharp<public)
chk('relative decimal exact cross product',public*10**9==x)
# G5a is only a conservative check: an honest E need not pass after rounding.
v=3*h/4;x=h;k=two(100);ehat=ru(k*v);S=x
LB=rnd(k*rnd(x-rnd(two(-60)*S)))
rhs=rnd(ehat*(1+two(-40)))
chk('G5a lower bound honest synthetic failure',abs(x-v)<=row_b(x,S) and LB>rhs,lower=LB,attested=ehat)
# Unit conversion roundoff is material even with a perfect kernel row.
ax=F(1,10);y=rnd(ax)
chk('exact unit factor requires conversion radius',abs(y-ax)>0)
chk('unit transfer triangle',abs(y-F(11,10)*ax)<=abs(y-ax)+ax*F(1,10))
chk('span statics cancellation needs sum errors',abs((F(1)+F(1,8))-(F(1)-F(1,16)))==F(3,16))
chk('outward D2 qualified not single ulp at x=0',1+two(-22)>1+two(-52))
result={'status':'PASS','checks':checks,'count':len(checks),'scope':'Exact independent algebra and synthetic certificate discriminators; no solver execution, no realized mutant kill claim.'}
OUT.mkdir(parents=True,exist_ok=True)
(OUT/'EXACT_CHECKS.json').write_text(json.dumps(result,indent=2)+'\n')
(OUT/'BASIS.json').write_text(json.dumps({'schema':'rv28-design-review-basis-v1','agent':'/root/rv28_a1_design','parent':'/root','mechanism':'collaboration.spawn_agent','selected_skill':'.agents/skills/software-code-review/SKILL.md','workflow':None,'source_records':basis,'source_trees':trees,'sealed_proposal_entries_verified':seal_count,'runtime':{'version':sys.version,'system':platform.system(),'machine':platform.machine(),'executable_sha256':sha(Path(sys.executable).read_bytes())}},indent=2)+'\n')
(OUT/'COMMANDS.json').write_text(json.dumps({'environment':{'GIT_OPTIONAL_LOCKS':'0','python':'<VENV>/bin/python -B'},'commands':commands},indent=2)+'\n')
print(json.dumps({'status':'PASS','exact_checks':len(checks),'basis_entries':len(basis),'sealed_proposal_entries_verified':seal_count,'trees':trees}))
