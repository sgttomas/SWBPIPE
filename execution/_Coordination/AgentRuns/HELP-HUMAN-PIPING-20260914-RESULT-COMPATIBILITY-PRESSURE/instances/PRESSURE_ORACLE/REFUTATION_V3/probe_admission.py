#!/usr/bin/env python3
import sys
sys.dont_write_bytecode=True
from decimal import Decimal as D, localcontext
from pathlib import Path
import hashlib,importlib.util,json,math
HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('v3_refutation',HERE/'run_refutation.py')
runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
maximum=D.from_float(sys.float_info.max)
cases=[]
for numerator in range(48,513):
    ratio=numerator/32.
    center=math.sqrt(sys.float_info.max/(math.pi*(ratio*ratio-1)))
    for offset in (-2,-1,0,1,2):
        ri=center
        for _ in range(abs(offset)):ri=math.nextafter(ri,math.inf if offset>0 else 0.)
        inp=dict(ri_m=ri,ro_m=ri*ratio,E_pa=2.**-900,nu=.25,p_pa=1.,thermal_strain=0.,epsilon_z=0.)
        checks=runner.oracle.expected_values(inp)
        # Restrict to ideal areas strictly no greater than MAX, avoiding any
        # interpretation of real values above MAX that round back to MAX.
        if D(checks['As_m2']['expected_decimal'])>maximum or D(checks['Ai_m2']['expected_decimal'])>maximum:continue
        assert all(math.isfinite(v['expected_f64']) for v in checks.values())
        cases.append({'id':f'STRICT_BELOW_MAX_ratio{numerator}_offset{offset}','inputs':inp,'checks':checks})
(HERE/'STRICT_ADMISSION_EXPECTATIONS.json').write_text(json.dumps(cases,indent=2,allow_nan=False)+'\n')
(HERE/'STRICT_ADMISSION_FREEZE.json').write_text(json.dumps({'status':'FROZEN_BEFORE_INVOKING_CANDIDATE_ON_THESE_INPUTS','cases':len(cases),'input_domain':'Exact high-precision Ai and As <= f64::MAX; excludes rounding-to-MAX ambiguity','expectations_sha256':hashlib.sha256((HERE/'STRICT_ADMISSION_EXPECTATIONS.json').read_bytes()).hexdigest(),'source_sha256':runner.SOURCE_HASH},indent=2)+'\n')
r=runner.run_adapter('STRICT_ADMISSION',runner.SOURCE,cases,[])
print(json.dumps({'cases':len(cases),'api_errors':r['api_errors'],'failure_count':len(r['comparison_failures'])},indent=2))
