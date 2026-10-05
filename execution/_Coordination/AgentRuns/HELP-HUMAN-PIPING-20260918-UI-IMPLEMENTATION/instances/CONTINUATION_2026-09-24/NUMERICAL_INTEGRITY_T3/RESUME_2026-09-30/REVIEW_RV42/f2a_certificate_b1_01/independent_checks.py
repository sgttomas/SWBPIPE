#!/usr/bin/env python3
"""RV42 finite abstract exact checks; no product code or solver/runtime imports."""
from fractions import Fraction as F
from pathlib import Path
import hashlib, json, struct, subprocess, os, contextlib, io

def interval_divide(qlo,qhi,dlo,dhi):
    if dlo<=0 or dhi<dlo: raise ValueError('denominator_not_separated')
    v=[n/d for n in (qlo,qhi) for d in (dlo,dhi)]
    return min(v),max(v)
def interval_torsion(qlo,qhi,rlo,rhi,jlo,jhi):
    if min(rlo,jlo)<=0 or rhi<rlo or jhi<jlo:raise ValueError('operand_not_positive')
    vals=[q*r/j for q in (qlo,qhi) for r in (rlo,rhi) for j in (jlo,jhi)]
    return min(vals),max(vals)
def mid(a,b):return (a+b)/2

def main():
    checks=[]; count=0
    for qlo,qhi in [(F(-3),F(-1)),(F(-1),F(2)),(F(0),F(0)),(F(2),F(5))]:
      for dlo,dhi in [(F(1,7),F(1,7)),(F(1,16),F(2)),(F(3),F(7))]:
        l,u=interval_divide(qlo,qhi,dlo,dhi)
        for q in (qlo,mid(qlo,qhi),qhi):
          for d in (dlo,mid(dlo,dhi),dhi):
            assert l<=q/d<=u; count+=1
        for raw_a in (F(1),F(10**6)):
          y=F.from_float(float(mid(l,u)/raw_a)); n=F.from_float(float(y*raw_a))
          hn=max(abs(n-l),abs(n-u)); hu=max(abs(y-l/raw_a),abs(y-u/raw_a))
          for t in (l,mid(l,u),u):
            assert abs(n-t)<=hn and abs(y-t/raw_a)<=hu; count+=1
    checks.append({'name':'independent_signed_division_and_actual_coordinates','asserted_points':count})
    ntests=0
    for qlo,qhi in [(F(-3),F(-1)),(F(-1),F(2)),(F(0),F(0)),(F(2),F(5))]:
      for rlo,rhi in [(F(1,16),F(3,2)),(F(2),F(2))]:
       for jlo,jhi in [(F(1,3),F(3)),(F(5),F(5))]:
        l,u=interval_torsion(qlo,qhi,rlo,rhi,jlo,jhi)
        for q in (qlo,mid(qlo,qhi),qhi):
         for r in (rlo,mid(rlo,rhi),rhi):
          for j in (jlo,mid(jlo,jhi),jhi):
           assert l<=q*r/j<=u; ntests+=1
    checks.append({'name':'independent_torsion_endpoint_enclosure','asserted_points':ntests})
    # Independent standard-library binary64 comparator for the frozen author's integer RN.
    author='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I31/f2a_certificate_b1/_run_records/exact_checks.py'
    src=subprocess.check_output(['git','show','751ac56ce8:'+author],env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')).decode()
    ns={'__name__':'__main__'}
    with contextlib.redirect_stdout(io.StringIO()):exec(compile(src,'frozen_author_exact_checks','exec'),ns)
    def p2(e):return F(2**e) if e>=0 else F(1,2**(-e))
    rounding=0
    for e in (-1074,-1073,-1023,-1022,-1021,-988,-53,-1,0,1,53,971,1023):
      for mant in (F(0),F(1,2),F(1),F(3,2),F(2)-p2(-52)):
       for sign in (-1,1):
        x=sign*p2(e)*mant
        # Decode comparisons preserve nonzero negative-underflow sign, treating exact zero canonically.
        try:
          native=struct.unpack('>Q',struct.pack('>d',float(x)))[0]
        except OverflowError:continue
        if native & 0x7ff0000000000000 == 0x7ff0000000000000:continue
        observed=ns['rn_bits'](x)
        assert observed==native,(e,mant,sign,hex(observed),hex(native)); rounding+=1
    checks.append({'name':'author_RN_reference_against_stdlib_Fraction_to_binary64','finite_points':rounding,'limit':'finite comparator samples, not a complete rounding proof'})
    for dlo,dhi in [(F(0),F(1)),(F(-1),F(1)),(F(1),F(0))]:
      try:interval_divide(F(0),F(1),dlo,dhi)
      except ValueError:pass
      else:raise AssertionError('denominator admitted')
    checks.append({'name':'denominator_separation','refusals':3})
    # Source bit difference is independently calculated; only an abstract witness.
    T,I,r,J=F(1),F(3),F(11),F(6)
    z=F.from_float(float(I/r))
    actual=F.from_float(float(F.from_float(float(T*r))/J))
    shortcut=F.from_float(float(T/F.from_float(float(2*z))))
    assert actual!=shortcut
    checks.append({'name':'actual_torsion_not_shortcut','actual':float(actual).hex(),'shortcut':float(shortcut).hex()})
    return {'nature':'finite independent standard-library exact abstract checks; no solver/model/product/native reach or qualification','all_passed':True,'checks':checks}
if __name__=='__main__':print(json.dumps(main(),indent=2))
