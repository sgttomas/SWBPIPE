import sys, json
from fractions import Fraction as Fr
sys.path.insert(0, sys.argv[1]); sys.path.insert(0, sys.argv[2])
import probe_skew_precision as d1
import cp0_exact_formation_probe as k4
skew=(3,4,0)
mom=lambda k:(1e-8*k/1e-4,2e-8*k/1e-4,0.0)
for name, case in (("6-member k=1e-12", d1.pin_case(skew,1e-12,mom(1e-12),members=6)), ("k=1e-28", d1.pin_case(skew,1e-28,mom(1e-28)))):
    a,_,_=k4.published(case,128); b,_,_=k4.published(case,256)
    sc=d1.scales(b,case)
    worst={}
    for key,(g,kind,v2) in b.items():
        v1=a[key][2]; den=max(abs(Fr(v2)),sc[key])
        r=abs(Fr(v1)-Fr(v2))/den if den else Fr(0)
        grp=key.split(':')[0]
        if r>worst.get(grp,(Fr(0),None))[0]: worst[grp]=(r,key)
    print(name, {g:(float(r),k) for g,(r,k) in worst.items()}, 'thr', float(Fr(1,2**64)))
