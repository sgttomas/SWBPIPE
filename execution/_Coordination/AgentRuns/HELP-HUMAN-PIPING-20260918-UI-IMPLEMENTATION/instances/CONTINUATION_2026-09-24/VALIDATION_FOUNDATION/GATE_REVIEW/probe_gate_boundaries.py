"""Independent bounded synthetic gate probes, not solver/reference execution."""
from pathlib import Path
import importlib.util, json, hashlib, sys
from copy import deepcopy
from unittest.mock import patch
import tempfile

HERE=Path(__file__).resolve().parent
PROJECT=next(p for p in HERE.parents if (p/'tools/validation/qualification_gate.py').is_file())
sys.path.insert(0,str(PROJECT))
spec=importlib.util.spec_from_file_location('gate_fixture',PROJECT/'tests/test_qualification_gate.py')
fixture=importlib.util.module_from_spec(spec);spec.loader.exec_module(fixture)
original_temp=tempfile.TemporaryDirectory
def local_temp(*args,**kwargs):
    return original_temp(*args,dir=HERE,**kwargs)
out=[]
with patch.object(fixture.tempfile,'TemporaryDirectory',local_temp):
    for scenario in ['uppercase_blocking_wrapper','contradictory_row_dimension','reference_lexical_underflow','stdin_file_changed_by_process']:
        t=fixture.GateTests('test_complete_actual_fake_process_and_raw_identity');t.setUp()
        try:
            wrapper=deepcopy(t.wrapper)
            if scenario=='uppercase_blocking_wrapper':
                wrapper['findings']=[{'severity':'BLOCKING','code':'REDACTION_BLOCK','message':'explicit required block'}]
                wrapper['summary']={'blocking_count':1}
                t.write_process(wrapper)
            elif scenario=='contradictory_row_dimension':
                wrapper['payload']['mechanics_envelope']['results'][1]['dimension']='force'
                t.write_process(wrapper)
            elif scenario=='reference_lexical_underflow':
                wrapper['payload']['mechanics_envelope']['results'][1]['value']=0
                t.write_process(wrapper)
                data=(t.root/'reference.json').read_text().replace('"value": 2.0','"value": 1e-999').encode()
                (t.root/'reference.json').write_bytes(data)
                t.case['reference']['sha256']=fixture.sha256_bytes(data)
            else:
                t.write_process(code='from pathlib import Path\nimport sys\nPath("stdin.bin").write_bytes(b"{\\\"different_input\\\":true}")\nactual=sys.stdin.buffer.read()\nPath("actually_read.bin").write_bytes(actual)\nprint('+repr(json.dumps(wrapper))+')')
            ledger=t.run_case()
            row={'scenario':scenario,'outcome':ledger['outcome'],'summary':ledger['summary'],'case_reason':ledger['cases'][0]['reason']}
            if scenario=='stdin_file_changed_by_process':
                process=ledger['cases'][0]['process'];actual=(t.root/'run-1/case-0000/actually_read.bin').read_bytes()
                row.update(reported_stdin_sha256=process['stdin_sha256'],actual_read_sha256=fixture.sha256_bytes(actual),actually_read=actual.decode())
            out.append(row)
        finally:t.doCleanups()
report={'scope':'Only synthetic short Python processes from preserved test setup; no actual Piping solve or reference qualification','gate_sha256':hashlib.sha256((PROJECT/'tools/validation/qualification_gate.py').read_bytes()).hexdigest(),'process_sha256':hashlib.sha256((PROJECT/'tools/validation/qualification_process.py').read_bytes()).hexdigest(),'results':out}
(HERE/'BOUNDARY_PROBES.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
