"""Addendum-only exact arithmetic/provenance. No solver or external oracle."""
from fractions import Fraction as F
import hashlib,json,os,pathlib,struct,subprocess,sys
p2=lambda n:F(2**n) if n>=0 else F(1,2**-n)
def rn(x): return F.from_float(float(x))
def bits(x): return struct.unpack('>Q',struct.pack('>d',float(x)))[0]
def of_bits(b): return F.from_float(struct.unpack('>d',struct.pack('>Q',b))[0])
def ru(x):
    q=rn(x)
    return of_bits(bits(q)+1) if q<x else q
def rd(x):
    q=rn(x)
    return of_bits(bits(q)-1) if q>x else q
checks=[]
def ck(name,condition,**data):
    assert condition,name
    checks.append({'name':name,'pass':True,**{k:str(v) for k,v in data.items()}})
h=p2(-1074)
for name,H in [('zero',F(0)),('tiny',h/4),('subnormal',5*h/4),('normal',F(1)+p2(-60))]:
    R=ru(H)
    ck('radius_'+name,R>=H and (H!=0 or R==0),H=H,R=R)
for H,b in [(F(1)-p2(-70),F(1)),(h/4,h),(F(3)*p2(-40),F(1))]:
    ck('representable_bound_'+str(len(checks)),H<=b and ru(H)<=b,H=H,b=b,radius=ru(H))
# mm: exact SI intended quantity, independently given, and source value/radius.
s=F(1,1000);x=rn(s);R=ru(abs(x-s));y=rn(1000*x);a=F(1,1000);n=rn(y/1000)
e_pub=abs(a*y-x);e_norm=abs(n-a*y)
H_y=e_pub+R;H_U=H_y/a;H_n=e_norm+H_y
ck('mm_exact_normalization',y==1 and n==rn(s) and e_norm>0,y=y,n=n,e_norm=e_norm)
ck('mm_raw_and_SI_certificates',abs(y-s/a)<=H_U and abs(n-s)<=H_n,H_U=H_U,H_n=H_n)
ck('mm_roundtrip_not_zero_error',n==x and e_norm>0,e_norm=e_norm)
# kN: raw 0.1 is inexact although the reader normalizes to exactly 100 N.
s=F(100);x=s;R=F(0);y=F.from_float(0.1);a=F(1000);n=rn(y*a)
e_pub=abs(a*y-x);e_norm=abs(n-a*y);H_U=(e_pub+R)/a;H_n=e_norm+e_pub+R
ck('kN_exact_normalization',n==100 and e_norm>0,y=y,n=n,e_norm=e_norm)
ck('kN_raw_and_SI_certificates',abs(y-s/a)<=H_U and abs(n-s)<=H_n,H_U=H_U,H_n=H_n)
ck('kN_roundtrip_not_zero_error',rn(n/1000)==y and e_norm>0,e_norm=e_norm)
# Raw-mm misuse changes class and can exclude a truth inside the SI interval.
y=F(1);a=F(1,1000);n=rn(y*a);S=p2(27);threshold=rn(p2(-34)*S);b=p2(-64)*S
s=n+b/2;t=s/a
ck('raw_mm_wrong_class',n<threshold and y>=threshold,normalized=n,raw=y,threshold=threshold)
ck('wrong_unit_interval_excludes_allowed_truth',n-b<=s<=n+b and not y-b<=t<=y+b,b_SI=b,truth_SI=s,truth_mm=t)
B_U=ru((b+abs(n-a*y))/a)
ck('raw_symmetric_display_radius_valid',abs(y-t)<=B_U,b_display_mm=B_U)
# Protected comparator operation order, explicitly not fused/reassociated.
z=F(1);S=F(1);eps=p2(-64);u=p2(-53);factor=1+p2(-21)
A_exact=eps*max(abs(z),S)*factor+u*abs(z)+h
a0=rn(eps*max(abs(z),S));a1=rn(a0*factor);u0=rn(u*abs(z));u1=rn(u0+h);A_f64=rn(a1+u1)
H=(A_exact+A_f64)/2
ck('relative_allowances_differ',A_exact>A_f64,A_exact=A_exact,A_f64=A_f64)
ck('relative_min_is_mandatory',H<=A_exact and H>A_f64 and 10**9*H<=abs(z),H=H)
# A zero SI radius does not guarantee an exactly represented target-unit point.
n=F(1);a_V=F(1000);truth_V=n/a_V;lo=rd(truth_V);hi=ru(truth_V)
ck('zero_SI_bound_nonexact_target_point',lo<truth_V<hi,SI=n,truth_kN=truth_V,lo=lo,hi=hi)
out=pathlib.Path.cwd()
(out/'EXACT_CHECKS.json').write_text(json.dumps({'status':'PASS','scope':'design arithmetic only; no source-unit re-entry or solver execution','checks':checks},indent=2)+'\n')
a1=pathlib.Path(sys.argv[1]);r=sys.argv[2];t=r.rsplit('/',1)[0];rev='d01ad98a754698631f927709d08284c272de85e8'
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
reads=[]
for rel in [t+'/DESIGN_NUMERICS/DESIGN.md',t+'/DESIGN_STANDING/DESIGN.md',
'projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/adaptive.rs',
'projects/chirality-piping/core/solver/frame_kernel/tests/retained_k4/models.rs',
'projects/chirality-piping/core/rules/rule_check_runner/src/lib.rs']:
    data=(a1/rel).read_bytes()
    pinned=subprocess.run(['git','show',rev+':'+rel],cwd=a1,env=env,check=True,stdout=subprocess.PIPE).stdout
    assert data==pinned,rel
    reads.append({'path':rel,'revision':rev,'sha256':hashlib.sha256(data).hexdigest(),'read':'selected cited sections','matches_revision':True})
review=a1/r/'design_review_RV28/REVIEW.md'
digest=hashlib.sha256(review.read_bytes()).hexdigest()
assert digest=='75e8a893403fdf51ad914357ed7d32652d6a2084094ef553d9000f4ad826f3bd'
reads.append({'path':r+'/design_review_RV28/REVIEW.md','sha256':digest,'read':'full review'})
old=out.parent
manifest=old/'SHA256SUMS'
old_results=[]
for line in manifest.read_text().splitlines():
    digest,name=line.split('  ',1)
    actual=hashlib.sha256((old/name).read_bytes()).hexdigest()
    assert digest==actual,name
    old_results.append({'path':name,'sha256':digest,'unchanged':True})
basis={'source_revision':rev,'instructions':'same sealed design BASIS; no new role/workflow/skill loaded',
'agent':'/root/a1_design','parent':'/root','mechanism':'collaboration.followup_task',
'additional_reads':reads,'original_seal_sha256':hashlib.sha256(manifest.read_bytes()).hexdigest(),
'original_entries':old_results,'runtime':{'executable':'<VENV>/bin/python','version':sys.version},
'command':'<VENV>/bin/python -B checks.py <A1_WT> <R> > RUN.stdout.json',
'no_Git_or_source_mutation':True,'arithmetic_checks':len(checks)}
(out/'BASIS.json').write_text(json.dumps(basis,indent=2)+'\n')
print(json.dumps({'status':'PASS','arithmetic_checks':len(checks),'additional_origins':len(reads),'original_sealed_entries_unchanged':len(old_results)}))

