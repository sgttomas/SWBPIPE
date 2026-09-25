"""Bounded allocation-order probe; no subprocess, build or large stress run."""
from pathlib import Path
import hashlib, json, sys
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
PROJECT = next(p for p in HERE.parents if (p/'core/serialization/canonical_json/adapter.py').is_file())
sys.path.insert(0, str(PROJECT))
from core.serialization.canonical_json import adapter

# One shared 1MiB string plus a tiny list exceeds the 8MiB serialized document
# limit. Nine entries keep this concrete probe bounded and do not approach OOM.
shared = 'x' * (1024 * 1024)
snapshot = [shared] * 9
calls = []
original_dumps = json.dumps
def observe_dumps(value, *args, **kwargs):
    output = original_dumps(value, *args, **kwargs)
    calls.append({'output_characters':len(output),'over_document_limit':len(output)>adapter._BINARY64_MAX_BYTES})
    return output

with patch.object(adapter.json, 'dumps', observe_dumps), patch.object(adapter.subprocess, 'run', side_effect=AssertionError('No process should be reached')):
    try:
        adapter.canonical_json_binary64_v1(snapshot)
        raise AssertionError('Expected byte refusal')
    except ValueError as error:
        outcome = str(error)

assert 'BYTE-LIMIT' in outcome
assert calls[0]['over_document_limit']
report = {'adapter_sha256':hashlib.sha256((PROJECT/'core/serialization/canonical_json/adapter.py').read_bytes()).hexdigest(),
          'scope':'Small allocation-order probe, not an OOM experiment or process execution',
          'unique_string_payload_bytes':len(shared),'list_items':len(snapshot),'serialization_calls_before_refusal':calls,'final_refusal':outcome,
          'implication':'The document byte guard rejects only after a 9MiB-plus JSON string has been fully allocated. Increasing references below the node cap proportionally amplifies allocation before rejection.'}
(HERE/'RESOURCE_ORDER.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
