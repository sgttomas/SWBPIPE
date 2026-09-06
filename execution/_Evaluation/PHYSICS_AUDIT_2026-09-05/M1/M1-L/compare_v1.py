import json, ast, re, math, hashlib
from pathlib import Path
from decimal import Decimal, localcontext
import numpy as np
p=Path(__file__).parent
lines=json.loads((p/'raw_v1.json').read_text())['utf8'].splitlines()
inputs={}; expected={}; solutions=[]; conditions={}; matrices={}
for l in lines:
 t=l.split('|')
 if t[0]=='INPUT': inputs[t[1]]=(ast.literal_eval(t[2]),ast.literal_eval(t[3]))
 if t[0]=='EXPECTED': expected[t[1]]=ast.literal_eval(t[2])
 if t[0] in ['DENSE','SPARSE','ENTRY'] and t[1].startswith('spd'):
  match=re.search(r'(?:Ok\(|solution: )(\[[^]]*\])',t[2]); solutions.append((t[0],t[1],ast.literal_eval(match[1])))
 if t[0]=='COND_MATRIX': matrices[t[1]]=np.array(ast.literal_eval(t[2]))
 if t[0]=='CONDITION': conditions[t[1]]=float(t[2][3:-1])
results=[]
for solver,name,x in solutions:
 a,b=inputs[name]
 with localcontext() as ctx:
  ctx.prec=80
  D=Decimal.from_float
  r=max(abs(sum(D(v)*D(u) for v,u in zip(row,x))-D(rhs)) for row,rhs in zip(a,b))
  an=max(sum(abs(D(v)) for v in row) for row in a);xn=max(abs(D(u)) for u in x);bn=max(abs(D(v)) for v in b)
  eta=r/(an*xn+bn) if an*xn+bn else Decimal(0)
 results.append(dict(solver=solver,case=name,max_forward_error=max(abs(u-v) for u,v in zip(x,expected[name])),residual_inf_decimal=str(r),backward_error_decimal=str(eta)))
refs=[]
for name,a in matrices.items():
 eig,q=np.linalg.eigh(a)
 residual=np.linalg.norm(a@q-q*eig,ord=2)
 orth=np.linalg.norm(q.T@q-np.eye(len(a)),ord=2)
 absmin=float(np.min(np.abs(eig)));absmax=float(np.max(np.abs(eig)))
 cond=absmax/absmin if absmin else None
 closed=(2+2*math.cos(math.pi/33))/(2-2*math.cos(math.pi/33)) if name=='tridiag32' else (1e10 if name=='eigen_floor' else None)
 refs.append(dict(case=name,production_condition=conditions[name],reference_condition=cond,analytical_condition=closed,eigen_min=float(min(eig)),eigen_max=float(max(eig)),eigenpair_residual_2=residual,orthogonality_error_2=orth,ratio_production_to_reference=conditions[name]/cond if cond else None))
out=dict(raw_sha256=hashlib.sha256(json.loads((p/'raw_v1.json').read_text())['utf8'].encode()).hexdigest(),numpy_version=np.__version__,linear_cases=results,condition_checks=refs)
(p/'COMPARISON_V1.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps(dict(condition_checks=refs,max_forward_error=max(r['max_forward_error'] for r in results),max_backward_error=max(float(r['backward_error_decimal']) for r in results)),indent=2))
