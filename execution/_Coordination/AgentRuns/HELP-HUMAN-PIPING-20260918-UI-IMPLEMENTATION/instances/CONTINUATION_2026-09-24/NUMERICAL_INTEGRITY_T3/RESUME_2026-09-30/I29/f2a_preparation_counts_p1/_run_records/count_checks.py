"""P1 exact count/encoding guard examples; no model construction or runtime.
No byte coefficient, memory allowance or source-domain decision is selected.
"""
import json
from pathlib import Path
U32=(1<<32)-1
SAFE=(1<<53)-1
def natural(x):
    if isinstance(x,bool) or not isinstance(x,int) or x<0: raise ValueError('nonnegative integer required')
    return x
def checked(x,bits):
    natural(x)
    if x>(1<<bits)-1: raise OverflowError('target width')
    return x
def raw(n,m,s,r,l,g,id_lengths,child_lengths,bits=64):
    for x in [n,m,s,r,l,g,*id_lengths,*child_lengths]: checked(x,32)
    if len(id_lengths)!=l or len(child_lengths)!=g: raise ValueError('shape mismatch')
    t=checked(3*m,32)
    N=checked(6*n,bits)
    I=checked(sum(id_lengths),bits)
    Cs=checked(sum(child_lengths),bits)
    U=checked(78*m+s,bits); P=checked(144*m+s,bits)
    E=checked(38+24*n+84*m+17*s+13*r+17*l+I+16*t+22*g+4*Cs,bits)
    return dict(N=N,t=t,I=I,Cs=Cs,U=U,P=P,E_src_raw_upper=E)
def validated(n,m,s,k,l,g,I,Cs,bits=64):
    N=checked(6*n,bits)
    if k>N: raise ValueError('invalid validated constraints')
    t=checked(3*m,32)
    Q=checked(7*n+12*m+6*t+s+k+2*g,bits)
    E=checked(38+24*n+84*m+17*s+13*k+17*l+I+16*t+22*g+4*Cs,bits)
    stf=checked(26+24*n+84*m+17*s+5*k,bits)
    return dict(N=N,F=N-k,Q=Q,E_src=E,E_stf=stf)
def combo(encs,loads,k,bits=64):
    h=checked(len(encs),32)
    if len(loads)!=h: raise ValueError('shape mismatch')
    for e in encs: checked(e,32)
    L=checked(sum(loads),bits)
    pairs=checked(h*k,bits)
    E=checked(10+sum(12+e for e in encs),bits)
    return dict(h=h,load_visits=L,prescribed_pairs=pairs,E_cmb=E)
def layout_count(size,align,count,bits):
    for x in (size,align,count): natural(x)
    if not align or align&(align-1) or size%align: raise ValueError('layout premise')
    b=checked(size*count,bits)
    rounded=checked(b+align-1,bits)//align*align
    if rounded>(1<<(bits-1))-1: raise OverflowError('isize layout')
    return b
rows=[]
def record(name,fn,expected_exception=None):
    try:
        value=fn()
        assert expected_exception is None,name
        rows.append({'check':name,'passed':True,'result':value})
    except (OverflowError,ValueError) as e:
        assert expected_exception and isinstance(e,expected_exception),(name,str(e))
        rows.append({'check':name,'passed':True,'expected_rejection':type(e).__name__})
record('raw duplicates not silently treated as validated F',lambda:raw(2,1,1,13,3,1,[4,4,4],[1]))
record('validated counterpart',lambda:validated(2,1,1,6,3,1,12,1))
record('invalid validated r',lambda:validated(2,1,1,13,3,1,12,1),ValueError)
record('3m final valid count',lambda:checked(3*(U32//3),32))
record('3m next member rejected before cast',lambda:checked(3*(U32//3+1),32),OverflowError)
record('6n target usize32 boundary',lambda:checked(6*(U32//6),32))
record('6n target usize32 overflow',lambda:checked(6*(U32//6+1),32),OverflowError)
record('case encoding fits usize64 but not operand u32',lambda:combo([U32+1],[1],1),OverflowError)
record('repeated operands preserve visits and prescription pairs',lambda:combo([123,123,77],[3,3,0],6))
record('empty operand arithmetic is not a valid combined source',lambda:combo([],[],6))
record('bool count rejected',lambda:natural(True),ValueError)
record('u8 layout target32 final byte',lambda:layout_count(1,1,(1<<31)-1,32))
record('u8 layout target32 first forbidden byte',lambda:layout_count(1,1,1<<31,32),OverflowError)
# Abstract capacity algebra only; these coefficients are not build profiles.
for alpha,beta,parts in [(0,1,[0,1,4]),(4,2,[0,1,4]),(8,3,[2,2,2,0])]:
    actual=sum(alpha*(x>0)+beta*x for x in parts)
    upper=alpha*len(parts)+beta*sum(parts)
    assert actual<=upper
    rows.append({'check':'abstract child-vector partition majorant','parameters':[alpha,beta,parts],
                 'passed':True,'sum':actual,'upper':upper,'not_build_coefficients':True})
out=Path(__file__).parent
(out/'COUNT_CHECKS.json').write_text(json.dumps({'scope':'Scalar/encoding/layout arithmetic only; no valid models, sources, heap profiles or admission decisions constructed','checks':rows},indent=2)+'\n')
print(json.dumps({'checks':len(rows),'all_passed':all(x['passed'] for x in rows)}))
