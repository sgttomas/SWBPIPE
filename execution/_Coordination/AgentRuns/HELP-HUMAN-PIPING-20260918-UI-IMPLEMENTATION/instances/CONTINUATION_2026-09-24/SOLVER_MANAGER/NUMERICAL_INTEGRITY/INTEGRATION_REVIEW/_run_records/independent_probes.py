"""Bounded review probes: exact-ID and exact Fraction formation checks.
Source transcriptions and independent exact arithmetic only; not Rust execution.
"""
from fractions import Fraction as Q
import json, math

def emitted(case): return 'diagnostic:numerical-integrity:' + case.replace(':', '-')
def assigned(cases, diagnostics):
    return [next((d['status'] for d in diagnostics if d['id'] == emitted(c)), 'not_assessed') for c in cases]
cases=['load:a-b','load-a:b']
first={'id':emitted(cases[0]),'status':'checks_passed','affected_refs':[cases[0]]}
second={'id':emitted(cases[1]),'status':'sensitive','affected_refs':[cases[1]]}
identity={'cases':cases,'distinct_nonempty_ids':len(set(cases))==2 and all(cases),'emitted_ids':[emitted(c) for c in cases], 'passed_sensitive_actual':assigned(cases,[first,second]),'passed_sensitive_expected':['checks_passed','sensitive'],'unassessed_actual':assigned(cases,[first]),'unassessed_expected':['checks_passed','not_assessed']}
assert identity['passed_sensitive_actual'] != identity['passed_sensitive_expected']
assert identity['unassessed_actual'] != identity['unassessed_expected']

def zeros(n): return [[0.0]*n for _ in range(n)]
def trans(a): return [list(r) for r in zip(*a)]
def mm(a,b):
    r=[[0.0]*len(b[0]) for _ in a]
    for i in range(len(a)):
        for j in range(len(b[0])):
            for k in range(len(b)): r[i][j] += a[i][k]*b[k][j]
    return r
def exact(a): return [[Q(x) for x in row] for row in a]
def exactmm(a,b): return [[sum((a[i][k]*b[k][j] for k in range(len(b))),Q(0)) for j in range(len(b[0]))] for i in range(len(a))]
def gamma(n): return n*2**-53/(1-n*2**-53)
def formed(h,k):
    c=mm(h,k); a=mm(c,trans(h)); r=zeros(12)
    for i in range(6):
        for j in range(6):
            r[i][j]=a[i][j];r[i][j+6]=-c[i][j];r[i+6][j]=-c[j][i];r[i+6][j+6]=k[i][j]
    return r,c

def curve_probe(scale):
    h=zeros(6)
    for i in range(6): h[i][i]=1.0
    x,y=-1.7,2.3
    h[3][2]=y;h[4][2]=-x;h[5][0]=-y;h[5][1]=x
    k=[[scale*((i+1.1) if i==j else 0.11/(1+abs(i-j))) for j in range(6)] for i in range(6)]
    local,c=formed(h,k); magnitude=[[sum(abs(h[i][z]*k[z][j]) for z in range(6)) for j in range(6)] for i in range(6)]
    g=gamma(12); lb=zeros(12)
    for i in range(6):
        for j in range(6):
            first=g*magnitude[i][j]/(1-g);lb[i][j+6]=first;lb[j+6][i]=first
            inherited=sum(abs(h[j][z])*g*magnitude[i][z]/(1-g) for z in range(6))
            second=sum(abs(c[i][z]*h[j][z]) for z in range(6))
            lb[i][j]=(inherited+g*second)*(1+64*2**-52)
    t=zeros(12)
    for b in range(4):
        for i,row in enumerate([[0.6,0.8,0],[-0.8,0.6,0],[0,0,1]]):
            for j,v in enumerate(row): t[3*b+i][3*b+j]=v
    temp=mm(local,t); computed=mm(trans(t),temp)
    magnitudes=[[sum(abs(local[i][z]*t[z][j]) for z in range(12)) for j in range(12)] for i in range(12)]
    bounds=zeros(12);g=gamma(24)
    for i in range(12):
        for j in range(12):
            inherited=sum(abs(t[z][i])*magnitudes[z][j] for z in range(12))
            second=sum(abs(t[z][i])*abs(temp[z][j]) for z in range(12))
            bounds[i][j]=g*(inherited/(1-g)+second)*(1+64*2**-52)
            local_inherited=sum(abs(t[a][i])*lb[a][b]*abs(t[b][j]) for a in range(12) for b in range(12))
            bounds[i][j] += local_inherited/(1-gamma(432))
    eqh,eqk=exact(h),exact(k)
    cexact=exactmm(eqh,eqk);aexact=exactmm(cexact,trans(eqh)); lexact=[[Q(0)]*12 for _ in range(12)]
    for i in range(6):
        for j in range(6):
            lexact[i][j]=aexact[i][j];lexact[i][j+6]=-cexact[i][j];lexact[i+6][j]=-cexact[j][i];lexact[i+6][j+6]=eqk[i][j]
    oracle=exactmm(trans(exact(t)),exactmm(lexact,exact(t)))
    for i in range(12):
        for j in range(12):
            assert oracle[i][j]==oracle[j][i]
            assert abs(Q(computed[i][j])-oracle[i][j])<=Q(bounds[i][j])
            assert abs(computed[i][j]-computed[j][i])<=bounds[i][j]+bounds[j][i]
    return {'tip_scale':scale,'entries_checked':144,'max_skew':max(abs(computed[i][j]-computed[j][i]) for i in range(12) for j in range(12)),'formation_bound_checks':'passed for this finite represented H/K/T sample'}
print(json.dumps({'scope':'Pure Python source transcriptions plus exact Fraction formation oracle, not production Rust execution; no universal bound proof','identity':identity,'curved_formation_samples':[curve_probe(1.0),curve_probe(1e12),curve_probe(1e-12)]},indent=2))
