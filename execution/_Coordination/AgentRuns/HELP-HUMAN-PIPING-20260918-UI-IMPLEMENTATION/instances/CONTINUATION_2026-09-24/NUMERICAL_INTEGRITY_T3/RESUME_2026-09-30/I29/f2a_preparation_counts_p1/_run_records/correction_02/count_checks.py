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
def width(bits):
    natural(bits)
    if bits == 0: raise ValueError("positive bit width required")
    return bits
def checked(x,bits):
    width(bits)
    natural(x)
    if x>(1<<bits)-1: raise OverflowError('target width')
    return x
def raw(n,m,s,r,l,g,id_lengths,child_lengths,bits=64):
    width(bits)
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
    width(bits)
    for x in (n,m,s,k,l,g): checked(x,32)
    for x in (I,Cs): checked(x,bits)
    N=checked(6*n,bits)
    if k>N: raise ValueError('invalid validated constraints')
    t=checked(3*m,32)
    Q=checked(7*n+12*m+6*t+s+k+2*g,bits)
    E=checked(38+24*n+84*m+17*s+13*k+17*l+I+16*t+22*g+4*Cs,bits)
    stf=checked(26+24*n+84*m+17*s+5*k,bits)
    return dict(N=N,F=N-k,Q=Q,E_src=E,E_stf=stf)
def combo(encs,loads,k,bits=64):
    width(bits)
    checked(k,32)
    for e in encs: checked(e,32)
    for count in loads: checked(count,32)
    h=checked(len(encs),32)
    if len(loads)!=h: raise ValueError('shape mismatch')
    for e in encs: checked(e,32)
    L=checked(sum(loads),bits)
    pairs=checked(h*k,bits)
    E=checked(10+sum(12+e for e in encs),bits)
    return dict(h=h,load_visits=L,prescribed_pairs=pairs,E_cmb=E)
def layout_count(size,align,count,bits):
    width(bits)
    for x in (size,align,count): checked(x,bits)
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
# Preserve every original selected control before adding repaired-entry negatives.
original_path=Path(__file__).parent.parent/'COUNT_CHECKS.json'
original=json.loads(original_path.read_text())
legacy_controls_equal=(rows==original['checks'])
assert legacy_controls_equal
record('RV48 validated Boolean node rejected',lambda:validated(True,0,0,0,0,0,0,0),ValueError)
record('RV48 validated negative k rejected',lambda:validated(1,0,0,-1,0,0,0,0),ValueError)
record('RV48 combo masked negative load rejected',lambda:combo([38,38],[-1,1],0),ValueError)
base=[2,1,1,6,3,1,12,1]
for at,name in enumerate(['n','m','s','k','l','g','I','Cs']):
    for bad,label in [(True,'Boolean'),(-1,'negative')]:
        values=base.copy();values[at]=bad
        record('validated individual '+name+' '+label,lambda values=values:validated(*values),ValueError)
for bad,label in [(True,'Boolean'),(-1,'negative')]:
    record('combo k '+label,lambda bad=bad:combo([38],[1],bad),ValueError)
    record('combo encoding element '+label,lambda bad=bad:combo([bad,38],[1,1],0),ValueError)
    record('combo load element '+label,lambda bad=bad:combo([38,38],[bad,2],0),ValueError)
    record('raw id-length element '+label,lambda bad=bad:raw(1,0,0,0,1,0,[bad],[]),ValueError)
    record('raw child-length element '+label,lambda bad=bad:raw(1,0,0,0,0,1,[],[bad]),ValueError)
    record('bit width '+label,lambda bad=bad:validated(*base,bits=bad),ValueError)
record('combo negative k with zero operands cannot hide in zero product',lambda:combo([],[],-1),ValueError)
record('masked encoding element sum rejected',lambda:combo([-1,39],[0,0],0),ValueError)
record('zero bit width rejected',lambda:checked(0,0),ValueError)
record('layout count width cannot hide in zero-size product',lambda:layout_count(0,1,1<<32,32),OverflowError)
# Sentinel mathematics only: no model/counterexample to the stronger guard.
f=U32+1
sentinel_note={
    'F':f,
    'reduced_triangle_fits_usize64':f*(f+1)//2 <= (1<<64)-1,
    'unreduced_product_fits_usize64':f*(f+1) <= (1<<64)-1,
    'not_a_counterexample_to_unreduced_premise':True,
    'valid_block_ids_when_count_is_U32':'0 through U32-1; U32 remains unvisited sentinel'
}
assert sentinel_note['reduced_triangle_fits_usize64']
assert not sentinel_note['unreduced_product_fits_usize64']
out=Path(__file__).parent
(out/'COUNT_CHECKS.json').write_text(json.dumps({
    'scope':'Corrected scalar/encoding/layout helper examples only; no model/source/profile/admission constructed',
    'legacy_control_comparison':{'original_relative_path':'../COUNT_CHECKS.json','original_controls':len(original['checks']),'equal':legacy_controls_equal},
    'checks':rows,'sentinel_clarification':sentinel_note},indent=2)+'\n')
print(json.dumps({'checks':len(rows),'all_passed':all(x['passed'] for x in rows),'original16_equal':legacy_controls_equal}))
