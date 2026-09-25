from fractions import Fraction as Q
from pathlib import Path
import math,itertools,json
E=Q(200000000000);A=Q(1,256);L=Q(1);k=E*A/L;g1=Q(1,2**14);g2=2*g1;F=k*g1
for x in [k,g1,g2,F]:assert Q.from_float(float(x))==x
near=lambda x:[Q.from_float(math.nextafter(float(x),-math.inf)),x,Q.from_float(math.nextafter(float(x),math.inf))]
def response(force,a,b,state):
 if state==(False,False):x=(force/k,2*force/k)
 elif state==(True,False):x=(a,a+force/k)
 elif state==(False,True):x=(b/2,b)
 else:x=(a,b)
 r=(2*k*x[0]-k*x[1],-k*x[0]+k*x[1]-force)
 for i in range(2):assert state[i] or r[i]==0
 return x,r
def advance(force,a,b,state):
 x,r=response(force,a,b,state);return tuple(r[i]<=0 if state[i] else x[i]>=[a,b][i] for i in range(2)),x,r
rows=[];maxit=0
for fi,gi,hi in itertools.product(range(3),repeat=3):
 f,a,b=near(F)[fi],near(g1)[gi],near(g2)[hi]
 stable=[s for s in itertools.product([False,True],repeat=2) if advance(f,a,b,s)[0]==s];assert len(stable)==1
 for sign in [-1,1]:
  for seed in itertools.product([False,True],repeat=2):
   s=seed;trace=[]
   for iteration in range(1,5):
    n,x,r=advance(f,a,b,s);trace.append({'iteration':iteration,'prior':s,'next':n,'u':[str(sign*z) for z in x],'r':[str(sign*z) for z in r]})
    if n==s:break
    s=n
   else:raise AssertionError('cap')
   assert n==stable[0];maxit=max(maxit,iteration)
   rows.append({'neighbors':[fi-1,gi-1,hi-1],'sense':sign,'seed':seed,'force':str(sign*f),'gaps':[str(a),str(b)],'expected':stable[0],'trace':trace})
report={'actor':'/root/solver_manager','method':'Independent-from-production Fraction closed-form axial equations; manager-authored, separate reviewer backcheck pending','E_Pa':str(E),'A_m2':str(A),'L_m':str(L),'k_N_per_m':str(k),'F_N':str(F),'gap1_m':str(g1),'gap2_m':str(g2),'nominal_segment_strain':str(g1/L),'nominal_axial_stress_Pa':str(E*g1/L),'reference_state_traces':len(rows),'two_backend_execution_cases':2*len(rows),'max_iterations':maxit,'cases':rows,'limits':'Defined prismatic elastic bar theory at small strain; not a manufacturer section, support capability, allowable/code or engineering acceptance. No production execution or old-oracle change.'}
Path(__file__).with_name('MANAGER_SMALL_STRAIN.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:v for k,v in report.items() if k!='cases'},indent=2))
