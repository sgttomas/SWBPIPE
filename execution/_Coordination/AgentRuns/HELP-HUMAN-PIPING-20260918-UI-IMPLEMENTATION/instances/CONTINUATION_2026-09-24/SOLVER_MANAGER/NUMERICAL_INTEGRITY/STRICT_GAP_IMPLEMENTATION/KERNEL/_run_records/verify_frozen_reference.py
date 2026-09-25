from pathlib import Path
from fractions import Fraction as Q
import json
num=Path(__file__).resolve().parents[3]
p=json.loads((num/'STRICT_GAP_DESIGN/_run_records/REFERENCE_PROOF.json').read_text())
count=0
for case in p['cases']:
 F=Q(case['F']);g=list(map(Q,case['g']))
 for t in case['traces']:
  for step in t['iterations']:
   u=list(map(Q,step['u']));r=list(map(Q,step['r']));st=step['prior']
   assert r==[200*u[0]-100*u[1],-100*u[0]+100*u[1]-F]
   assert all(u[i]==g[i] if st[i] else r[i]==0 for i in range(2))
   assert step['classified']==[int(r[i]<=0) if st[i] else int(u[i]>=g[i]) for i in range(2)]
   count+=1
print(json.dumps({'cases':len(p['cases']),'verified_trace_rows':count,'result':'pass'}))
