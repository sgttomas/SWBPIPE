"""Python source transcriptions and exact Fraction oracles, not Rust execution."""
import copy,json,math
from fractions import Fraction as Q

def cholesky(k,f):
 n=len(f);exps=[-(math.frexp(k[i][i])[1]-1)//2 for i in range(n)]
 # Rust uses -floor(binary_exponent/2), not floor(-binary_exponent/2).
 exps=[-((math.frexp(k[i][i])[1]-1)//2) for i in range(n)]
 a=[[math.ldexp(k[i][j],exps[i]+exps[j]) for j in range(n)] for i in range(n)]
 b=[math.ldexp(f[i],exps[i]) for i in range(n)]
 l=[[0.0]*n for _ in range(n)]
 for i in range(n):
  for j in range(i+1):
   s=a[i][j]
   for z in range(j):s-=l[i][z]*l[j][z]
   l[i][j]=math.sqrt(s) if i==j else s/l[j][j]
 x=b[:]
 for i in range(n):
  for j in range(i):x[i]-=l[i][j]*x[j]
  x[i]/=l[i][i]
 for i in reversed(range(n)):
  for j in range(i+1,n):x[i]-=l[j][i]*x[j]
  x[i]/=l[i][i]
 return [math.ldexp(x[i],exps[i]) for i in range(n)]

def row(k,f,u):
 exact=sum((Q(a)*Q(b) for a,b in zip(k,u)),Q(0))-Q(f)
 denom=sum((abs(Q(a)*Q(b)) for a,b in zip(k,u)),Q(0))+abs(Q(f))
 scale=max([math.frexp(abs(f))[1]-1 if f else -9999]+[math.frexp(abs(a))[1]-1+math.frexp(abs(b))[1]-1+1 for a,b in zip(k,u) if a and b]);r=-math.ldexp(f,-scale);d=abs(r)
 for a,b in zip(k,u):
  if a==0:continue
  p=math.ldexp(a*b,-scale);r+=p;d+=abs(p)
 m=2*sum(a!=0 for a in k)+2;g=m*2**-53/(1-m*2**-53);guard=(abs(r)/d+g/(1-g))*(1+g)*(1+4*2**-52);target=64*g
 return {'exact_residual':str(exact),'exact_ratio':str(abs(exact)/denom),'guarded_ratio':guard,'target':target,'passed':guard<=target}

k=[[200.0,-100.0],[-100.0,100.0]];f=[0.0,12.5];gap=[.125,.25];u=cholesky(k,f)
assert u[0]<gap[0] and u[1]<gap[1]
exact=[Q(1,8),Q(1,4)]
assert [sum((Q(a)*b for a,b in zip(r,exact)),Q(0)) for r in k]==[Q(v) for v in f]
probes=[]
for sign in [1.0,-1.0]:
 solved=cholesky(k,[sign*v for v in f]);states=['active' if (x>=g if sign>0 else x<=-g) else 'inactive' for x,g in zip(solved,gap)]
 rows=[row(r,sign*b,solved) for r,b in zip(k,f)]
 assert all(r['passed'] for r in rows) and states==['inactive','inactive']
 probes.append({'sign':sign,'computed_u':solved,'gap':gap,'initial_states':['inactive','inactive'],'classified_states':states,'changed_support_count':0,'equilibrium_rows':rows,'exact_closed_contact_u':[str(sign*x) for x in exact],'exact_closed_reactions':[0,0]})

# Relevant conjunction in product_selected_state_is_qualified after actual
# unchanged boundary/reclassification gates. These flags are established by
# a valid solve; preserve them while changing only the claimed report inputs.
def qual(s):
 return (s['converged'] and s['contact'] and not s['changed'] and s['residual_count']==0 and s['product_policy']=='M03-PRODUCT-PREVIEW-EQUILIBRIUM-v1' and s['product_passed'] and s['product_rows']==s['structural_rows'] and s['contribution_audit'] and s['same_states'] and all(r['passed'] for r in s['structural_rows']) and s['same_boundary'] and s['same_values'])
s={'converged':True,'contact':True,'changed':[],'residual_count':0,'product_policy':'M03-PRODUCT-PREVIEW-EQUILIBRIUM-v1','product_passed':True,'product_rows':[{'dof':6,'passed':True}],'structural_rows':[{'dof':6,'passed':True}],'contribution_audit':True,'same_states':True,'same_boundary':True,'same_values':True,'work_rows':[{'dof':6,'passed':True}],'structural_policy':'M03-INTEGRITY-v1','input_force':12.5}
mutations=[]
for name in ['missing_rows','missing_work','unknown_structural_policy','changed_input_force']:
 m=copy.deepcopy(s)
 if name=='missing_rows':m['product_rows']=[];m['structural_rows']=[];m['work_rows']=[]
 if name=='missing_work':m['work_rows']=[]
 if name=='unknown_structural_policy':m['structural_policy']='unknown'
 if name=='changed_input_force':m['input_force']=25.0
 mutations.append({'mutation':name,'qualifier_conjunction_result':qual(m)})
 assert qual(m)
print(json.dumps({'scope':'Source transcriptions, exact Fraction oracle; no production Rust execution or contact runtime observation','contact':probes,'qualification_mutations':mutations},indent=2))
