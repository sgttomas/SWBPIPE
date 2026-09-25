"""Focused original-token half-subnormal/overflow edge probes, no build."""
from pathlib import Path
from decimal import Decimal, localcontext
import hashlib,json,subprocess,sys
HERE=Path(__file__).resolve().parent
PROJECT=next(p for p in HERE.parents if (p/'core/serialization/canonical_json/adapter.py').is_file())
binary=PROJECT/'core/serialization/canonical_json/target/checked-json/release/openpipestress_jcs_binary64'
with localcontext() as ctx:
    ctx.prec=1300
    zero_half=Decimal(1)/Decimal(2**1075)
    tiny=Decimal('1e-1200')
    overflow=Decimal(2**1024-2**970)
    vectors=[('below_half_subnormal',str(zero_half-tiny),False),('exact_half_subnormal',str(zero_half),False),
             ('above_half_subnormal',str(zero_half+tiny),True),('below_overflow_threshold',str(overflow-1),True),
             ('exact_overflow_threshold',str(overflow),False),('above_overflow_threshold',str(overflow+1),False)]
results=[]
for name,token,admitted in vectors:
    request={'protocol_version':'1.0.0','profile':'openpipestress_jcs_binary64_v1','items':[{'id':name,'json_text':token}]}
    out=subprocess.run([str(binary)],input=json.dumps(request),text=True,capture_output=True,timeout=10)
    assert (out.returncode==0)==admitted,(name,out.stderr)
    canonical=json.loads(out.stdout)['items'][0]['canonical_json'] if admitted else None
    if name=='above_half_subnormal':assert canonical=='5e-324'
    if name=='below_overflow_threshold':assert canonical=='1.7976931348623157e+308'
    results.append({'name':name,'original_token':token,'exit_code':out.returncode,'canonical':canonical,'error':out.stderr.strip()})
report={'scope':'Six exact Decimal boundary tokens through actual standalone Rust CLI; no broad conformance rerun or product adoption','binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'results':results}
(HERE/'NUMERIC_EDGE_PROBES.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps([{k:v for k,v in r.items() if k!='original_token'} for r in results],indent=2))
