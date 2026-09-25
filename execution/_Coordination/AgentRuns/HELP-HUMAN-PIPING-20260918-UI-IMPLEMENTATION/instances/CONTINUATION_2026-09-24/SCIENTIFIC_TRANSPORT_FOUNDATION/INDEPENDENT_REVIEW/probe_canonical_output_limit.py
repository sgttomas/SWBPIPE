"""Single bounded real-CLI output/round-trip resource probe; no build."""
from pathlib import Path
import hashlib, json, subprocess
HERE=Path(__file__).resolve().parent
PROJECT=next(p for p in HERE.parents if (p/'core/serialization/canonical_json/adapter.py').is_file())
binary=PROJECT/'core/serialization/canonical_json/target/checked-json/release/openpipestress_jcs_binary64'
profile='openpipestress_jcs_binary64_v1'
source='["'+'x'*(4*1024*1024)+'",'+','.join(['1e20']*200000)+']'
def run(text):
    request={'protocol_version':'1.0.0','profile':profile,'items':[{'id':'probe','json_text':text}]}
    return subprocess.run([str(binary)],input=json.dumps(request,separators=(',',':')),text=True,capture_output=True,timeout=30)
first=run(source)
assert first.returncode==0,first.stderr
canonical=json.loads(first.stdout)['items'][0]['canonical_json']
second=run(canonical)
report={'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'source_bytes':len(source.encode()),'source_nodes':200002,'declared_document_byte_limit':8*1024*1024,'first_exit':first.returncode,'canonical_bytes':len(canonical.encode()),'canonical_sha256':hashlib.sha256(canonical.encode()).hexdigest(),'second_exit':second.returncode,'second_error':second.stderr.strip(),'scope':'One actual standalone CLI probe; no product/native UI or broad suite.'}
(HERE/'CANONICAL_OUTPUT_LIMIT.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
