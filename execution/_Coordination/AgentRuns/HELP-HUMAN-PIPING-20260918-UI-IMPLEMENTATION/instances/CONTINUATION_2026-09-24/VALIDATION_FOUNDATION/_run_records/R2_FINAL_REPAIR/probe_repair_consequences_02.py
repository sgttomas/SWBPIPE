"""Independent repair-consequence probes; synthetic processes, no solver proof."""
from pathlib import Path
from copy import deepcopy
import importlib.util, json, hashlib, sys, tempfile
from unittest.mock import patch
HERE=Path(__file__).resolve().parent
PROJECT=next(p for p in HERE.parents if (p/'tools/validation/qualification_gate.py').is_file())
sys.path.insert(0,str(PROJECT))
spec=importlib.util.spec_from_file_location('gate_fixture_backcheck',PROJECT/'tests/test_qualification_gate.py')
fixture=importlib.util.module_from_spec(spec);spec.loader.exec_module(fixture)
original_temp=tempfile.TemporaryDirectory
def local_temp(*args,**kwargs):return original_temp(*args,dir=HERE,**kwargs)
results=[]
with patch.object(fixture.tempfile,'TemporaryDirectory',local_temp):
    for scenario in ['stdout_changed_between_capture_and_consumer','retained_reference_copy_changed_during_process']:
        t=fixture.GateTests('test_complete_actual_fake_process_and_raw_identity');t.setUp()
        try:
            emitted=deepcopy(t.wrapper)
            if scenario.startswith('stdout'):
                emitted['payload']['mechanics_envelope']['results'][1]['value']=99
                t.write_process(emitted)
                actual_capture=fixture.gate.capture
                def boundary_hook(*args,**kwargs):
                    result=actual_capture(*args,**kwargs)
                    # Deterministic mutation at the exact boundary being checked.
                    (args[3]/'stdout.bin').write_bytes(json.dumps(t.wrapper).encode())
                    return result
                with patch.object(fixture.gate,'capture',boundary_hook):
                    ledger=t.run_case()
                result={'scenario':scenario,'probe_mechanism':'real synthetic process then deterministic post-capture file mutation hook, not a claimed solver attack','outcome':ledger['outcome'],'case_reason':ledger['cases'][0]['reason'],'process_captured_sha256':ledger['cases'][0]['process']['stdout_sha256'],'consumed_archive_sha256':fixture.sha256_bytes((t.root/'run-1/case-0000/stdout.bin').read_bytes()),'emitted_required_x':99,'replacement_required_x':2}
            else:
                code='from pathlib import Path\nimport sys\nsys.stdin.buffer.read()\nPath("../case-0000.reference.json").write_bytes(b"{}")\nprint('+repr(json.dumps(t.wrapper))+')'
                t.write_process(code=code)
                ledger=t.run_case()
                result={'scenario':scenario,'probe_mechanism':'actual synthetic child modifies retained reference copy; original bound reference untouched','outcome':ledger['outcome'],'case_reason':ledger['cases'][0]['reason'],'original_reference_sha256':fixture.sha256_bytes((t.root/'reference.json').read_bytes()),'retained_reference_sha256':fixture.sha256_bytes((t.root/'run-1/case-0000.reference.json').read_bytes())}
            results.append(result)
        finally:t.doCleanups()
report={'scope':'Two narrow consequences of repaired archive custody, not solver qualification','source_hashes':{n:hashlib.sha256((PROJECT/'tools/validation'/n).read_bytes()).hexdigest() for n in ['qualification_gate.py','qualification_process.py']},'results':results}
(HERE/'REPAIR_CONSEQUENCE_PROBES_02.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))

