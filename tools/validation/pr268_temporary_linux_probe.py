import json
import math
import os
from pathlib import Path
import platform
import sys
from decimal import Decimal
from fractions import Fraction as F

root=Path.cwd()
sys.path.insert(0,str(root/'validation/references/f2_spectrum/closed_form/generators'))
import beams_build
import beams
rows=[]
for name,r in beams_build.compute()['real'].items():
    for n in beams.TRUNCATIONS:
        f=r['cont']['rayleigh'].freq[n-1]
        before=f*Decimal('1.01')
        x=float(before)
        formatted=format(x,'.6g')
        target=Decimal(formatted)
        hi_step=Decimal(10)**(target.adjusted()-5)
        lo_step=hi_step/10 if target==Decimal(10)**target.adjusted() else hi_step
        low=F(target-lo_step/2); high=F(target+hi_step/2)
        xf=F.from_float(x)
        assert low<=xf<=high
        distance=min(xf-low,high-xf)
        assert float(formatted)==r['fr'][n]
        rows.append(dict(case=name,mode=n,decimal_frequency=str(f),decimal_before_float=str(before),
            preformat_float_hex=x.hex(),formatted=formatted,final_hex=r['fr'][n].hex(),
            lower_format_boundary=str(low),upper_format_boundary=str(high),
            distance_exact=str(distance),distance_hz=float(distance),
            distance_ulps=float(distance/F.from_float(math.ulp(x)))))
out=Path(os.environ.get('DIAGNOSTIC_OUTPUT','/tmp/pr268-linux-evidence'))
out.mkdir(parents=True,exist_ok=True)
result=dict(platform=platform.platform(),python=sys.version,rows=rows,
    closest=min(rows,key=lambda r:r['distance_ulps']))
(out/'beam-inputs.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result['closest'],indent=2))
