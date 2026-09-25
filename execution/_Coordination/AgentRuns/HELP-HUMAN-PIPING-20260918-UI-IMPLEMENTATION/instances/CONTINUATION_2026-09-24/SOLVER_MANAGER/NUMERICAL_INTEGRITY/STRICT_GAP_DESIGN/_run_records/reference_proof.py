"""Independent exact reference. No production package/import/output is used as oracle."""
from fractions import Fraction as Q
from itertools import product
from math import nextafter, inf
from pathlib import Path
import json

K=((Q(200),Q(-100)),(Q(-100),Q(100)))
F0=Q(25,2)
g0=(Q(1,8),Q(1,4))

def action(u,F):
    return tuple(sum(K[i][j]*u[j] for j in range(2))-(Q(0),F)[i] for i in range(2))

def constrained(F,g,state):
    # 0/1 denotes released/engaged. Solve exactly by equilibrium, independent of product code.
    if state==(0,0): u=(F/100,F/50)
    elif state==(1,0): u=(g[0],g[0]+F/100)
    elif state==(0,1): u=(g[1]/2,g[1])
    else: u=g
    return u,action(u,F)

def admissible(u,r,g,state):
    return all((u[i]==g[i] and r[i]<=0) if state[i] else (u[i]<g[i] and r[i]==0) for i in range(2))

def classify(u,r,g,state):
    return tuple(int(r[i]<=0) if state[i] else int(u[i]>=g[i]) for i in range(2))

def trace(F,g,seed):
    state=seed; rows=[]
    for n in range(1,9):
        u,r=constrained(F,g,state); nxt=classify(u,r,g,state)
        rows.append(dict(iteration=n,prior=state,u=u,r=r,classified=nxt,changed=sum(a!=b for a,b in zip(state,nxt))))
        if nxt==state:
            assert admissible(u,r,g,state)
            return rows
        state=nxt
    raise AssertionError('Exact active-set reference failed cap eight')

def sign(v): return (v>0)-(v<0)

loads={'below':Q(nextafter(12.5,-inf)),'equal':F0,'above':Q(nextafter(12.5,inf))}
gaps=[{'below':Q(nextafter(float(g),-inf)),'equal':g,'above':Q(nextafter(float(g),inf))} for g in g0]
rows=[]
for lk,g1k,g2k in product(loads,*[x.keys() for x in gaps]):
    F=loads[lk]; g=(gaps[0][g1k],gaps[1][g2k])
    solutions=[]
    for st in product((0,1),repeat=2):
        u,r=constrained(F,g,st)
        if admissible(u,r,g,st): solutions.append((st,u,r))
    assert len(solutions)==1
    expected,u,r=solutions[0]
    # Proof of 2x2 solution and authored-boundary residual without rounded displacement oracle.
    a,b,c=K[0][0],K[0][1],K[1][1]
    det=a*c-b*b
    assert a>0 and det==10000
    nums=(-b*F,a*F)
    assert tuple(n/det for n in nums)==constrained(F,g,(0,0))[0]
    gap_nums=tuple(nums[i]-g[i]*det for i in range(2))
    trial_rows=[]
    for seed in product((0,1),repeat=2):
        history=trace(F,g,seed)
        assert tuple(history[-1]['classified'])==expected
        trial_rows.append(dict(seed=seed,iterations=history))
    signed=[]
    for s in (1,-1):
        su=tuple(s*x for x in u); sr=tuple(s*x for x in r)
        # Reflect all physical loads/displacements/reactions; normalized strict predicates identical.
        assert action(su,s*F)==sr
        assert all((s*sr[i]<=0 and s*su[i]==g[i]) if expected[i] else (s*su[i]<g[i] and sr[i]==0) for i in range(2))
        signed.append(dict(sign=s,u=su,r=sr))
    rows.append(dict(load=lk,g1=g1k,g2=g2k,F=F,g=g,expected=expected,u=u,r=r,free_gap_signs=[sign(v) for v in gap_nums],authored_all_boundary_residual=action(g,F),signed=signed,traces=trial_rows))

# A nonzero original prescribed value: root u0=c0, original axial root coupling [-100,0].
c0=Q(1,16); target=(c0+g0[0],c0+g0[1]); fullK=((100,-100,0),(-100,200,-100),(0,-100,100)); fullu=(c0,*target)
f=(Q(0),Q(0),F0)
fullr=tuple(sum(Q(fullK[i][j])*fullu[j] for j in range(3))-f[i] for i in range(3))
assert fullr[1:]==(0,0)
wrong_omitted_Kfc=action(target,F0)
assert wrong_omitted_Kfc==(100*c0,0)
# Tiny lost force-source control: direct represented source sum differs though round(sum) absorbs it.
tiny=Q(1,2**60)
assert float(F0+tiny)==float(F0)
assert action(g0,F0+tiny)==(0,-tiny)
# Positive computed factors alone do not prove original-system nonsingularity: A=0 admits every u.
assert Q(0)*Q(1)==0 and Q(0)*Q(2)==0
# Individual replacement is inadequate: dense source transcription's remaining second coordinate stays wrong.
hat=(Q(0.12499999999999997),Q(0.24999999999999997))
individual=[action((g0[0],hat[1]),F0),action((hat[0],g0[1]),F0)]
assert all(any(r!=0 for r in rr) for rr in individual)
assert action(g0,F0)==(0,0)

def enc(v):
    if isinstance(v,Q): return str(v)
    if isinstance(v,dict): return {str(k):enc(x) for k,x in v.items()}
    if isinstance(v,(list,tuple)): return [enc(x) for x in v]
    return v
out={'basis':'Independently derived exact rational K, F and represented neighbor inputs; no production output oracle','physical_signs':2,'input_combinations':len(rows),'initial_seeds':4,'backend_independent_expected_cases_for_two_modes':len(rows)*2*4*2,'max_reference_iterations':max(len(t['iterations']) for r in rows for t in r['traces']),'cases':rows,'controls':{'root_prescribed':c0,'root_shifted_target':target,'full_original_reactions':fullr,'omitted_Kfc_free_error':wrong_omitted_Kfc,'lost_load':tiny,'lost_load_candidate_reaction':action(g0,F0+tiny),'individual_candidate_nonzero_residuals':individual}}
Path(__file__).with_name('REFERENCE_PROOF.json').write_text(json.dumps(enc(out),indent=2)+'\n')
print(json.dumps({k:v for k,v in out.items() if k not in ('cases','controls')},indent=2))
for r in rows:
    if r['load']=='equal' or (r['g1']=='equal' and r['g2']=='equal'):
        print(r['load'],r['g1'],r['g2'],r['expected'],'r=',tuple(str(x) for x in r['r']))
