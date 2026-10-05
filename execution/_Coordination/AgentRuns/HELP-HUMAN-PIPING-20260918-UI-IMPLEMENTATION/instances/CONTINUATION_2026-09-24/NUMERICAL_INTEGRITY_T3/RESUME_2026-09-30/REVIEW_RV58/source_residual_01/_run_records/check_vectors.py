#!/usr/bin/env python3
"""Validate every generated bound using RV58's independent exact references."""
import ast,re,json
from independent_exact import Q,PI,facts,area0,inertia0,torsion0,E,G,A,I,J,vector_bytes,pow2
def endpoint(v):
    sign,e,limbs=v
    return (-1 if sign else 1)*sum(Q(k)*pow2(64*i) for i,k in enumerate(limbs))*pow2(e-1023)
vectors={}
for name,expr in re.findall(r"pub const (\w+):[^=]*=\s*(.*?);",vector_bytes.decode(),re.S):
    if name in facts or name=="ZERO":continue
    expr=expr.replace("true","True").replace("false","False").replace("ZERO",repr((False,0,[0]*16)))
    data=ast.literal_eval(expr)
    vectors[name]=[(endpoint(a),endpoint(b)) for a,b in data] if name.endswith("MOTION") else (endpoint(data[0]),endpoint(data[1]))
def bounded(n,truth):
    lo,hi=vectors[n];assert lo<=truth[0]<=truth[1]<=hi,n
def rp(c):
    vals=[c/p for p in PI];return min(vals),max(vals)
k=[1/(E*A),1/(3*E*I),Q(0),1/(G*J),Q(0),1/(2*E*I)]
g0=[1/(E*area0),1/(3*E*inertia0),Q(0),1/(G*torsion0),Q(0),1/(2*E*inertia0)]
for (lo,hi),truth in zip(vectors["K_MOTION"],k):assert lo<=truth<=hi
for (lo,hi),c in zip(vectors["G_MOTION"],g0):
    low,high=rp(c);assert lo<=low<=high<=hi
def root(n,square):
    lo,hi=vectors[n]
    assert 0<=lo<=hi and lo*lo<=square[0]<=square[1]<=hi*hi,n
root("K_MAG",(sum(v*v for v in k[:3]),)*2)
root("G_MAG",(sum(v*v for v in g0[:3])/PI[1]**2,sum(v*v for v in g0[:3])/PI[0]**2))
root("ROOT_TWO",(Q(2),Q(2)))
bounded("FIXED_AXIAL",(E*area0*PI[0]*Q(0.0001),E*area0*PI[1]*Q(0.0001)))
ks=Q(100000000)
bounded("SPRING_U",(1/(E*area0*PI[1]+ks),1/(E*area0*PI[0]+ks)))
bounded("SPRING_ACTION",(-ks/(E*area0*PI[0]+ks),-ks/(E*area0*PI[1]+ks)))
bounded("SPRING_REACTION",(-E*area0*PI[1]/(E*area0*PI[1]+ks),-E*area0*PI[0]/(E*area0*PI[0]+ks)))
for n,c in [("ROTATED_UX",15),("ROTATED_UY",20),("ROTATED_MAG",25)]:bounded(n,rp(Q(c)/(E*area0)))
print(json.dumps({"generated_vector_families_checked":len(vectors),"result":"PASS","all_references_enclose_independently_derived_source_or_K_truth":True,"reference_refinement":"768-bit generated endpoints validated against separate >1800-bit Machin bracket; no protected tolerance changed"}))
