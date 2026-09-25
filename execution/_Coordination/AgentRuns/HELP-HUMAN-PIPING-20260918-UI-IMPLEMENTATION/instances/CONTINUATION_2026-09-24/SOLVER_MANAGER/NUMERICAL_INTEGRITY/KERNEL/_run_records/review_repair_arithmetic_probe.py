"""Repair arithmetic probes only; no Rust/production code execution."""
from fractions import Fraction as F
import json

def grow(expansion,value):
    q=value; result=[]
    for e in expansion:
        total=q+e; z=total-q; low=(q-(total-z))+(e-z)
        if low:result.append(low)
        q=total
    if q:result.append(q)
    return result
terms=[1e16,1.,1e-16,-1.,-1e16]; expansion=[]
for value in terms:expansion=grow(expansion,value)
assert sum(map(F,expansion))==sum(map(F,terms))==F(1e-16)

def actions(coords,omega):
    result=[]
    origin=list(map(F,coords[0])); w=list(map(F,omega))
    for point in coords:
        x,y,z=[F(p)-o for p,o in zip(point,origin)]
        result.append([w[1]*z-w[2]*y,w[2]*x-w[0]*z,w[0]*y-w[1]*x])
    return result
under=[[0.,0.,0.],[1e200,0.,0.],[0.,1e-200,0.]]
center=[[1e16,1e16,0.],[-1e16,-1e16,0.],[-1e16,-9999999999999998.,0.]]
assert any(any(row) for row in actions(under,[1.,0.,0.]))
assert any(any(row) for row in actions(center,[-1.,-1.,0.]))
assert not any(any(row) for row in actions([[8.,8.,0.],[11.,12.,0.]],[3.,4.,0.]))
print(json.dumps({"kind":"exact Fraction oracles plus expansion transcription, not Rust execution","low_tail_expansion":expansion,"original_geometry_refutes_both_counterexamples":True,"original_geometry_valid_3_4_axis":True,"prescribed_coupling":{"stored_u":0.5,"intended_u":1.,"intended_residual":-1.,"relative_rhs_difference":1.}},indent=2))
