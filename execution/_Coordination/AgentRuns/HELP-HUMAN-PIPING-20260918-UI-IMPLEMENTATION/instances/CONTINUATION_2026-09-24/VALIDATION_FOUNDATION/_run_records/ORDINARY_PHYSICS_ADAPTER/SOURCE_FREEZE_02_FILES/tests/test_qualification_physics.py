"""Bounded adapter/isolated-helper tests; synthetic API stub is not solver evidence."""
from __future__ import annotations
from copy import deepcopy
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

PROJECT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PROJECT))
from tools.validation import qualification_physics as physics


class IsolatedReaderTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.source = self.root / 'candidate'
        self.source.mkdir()
        # A deliberately tiny API stub tests isolation and byte binding only.
        # Real owned-reader execution is separately recorded as received-packet
        # preparation evidence, never substituted by this stub's assertions.
        code = '''from pathlib import Path
import json

def validate_physics_evidence(source):
    table=json.loads((Path(__file__).resolve().parents[2]/"fixtures/results/semantic_contract_v0_3_physics_1.json").read_text())
    if source.get("stub_expected") != table.get("stub_expected"):
        raise ValueError("synthetic API refusal")
'''
        self.table = {'semantic_contract_id': physics.CONTRACT, 'source_schema_version': '0.2.0', 'stub_expected': 'same-selected-origin'}
        payloads = {physics.MODULE: code.encode(), physics.TABLE: json.dumps(self.table).encode(), physics.UNITS: b'// synthetic unit authority binding, no execution\n'}
        for path, data in payloads.items():
            target = self.source/path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
        self.binding = {'format': 'openpipestress.ordinary_physics_consistency_binding/1', 'status': 'prepared_not_admitted',
                        'contract_id': physics.CONTRACT, 'entrypoint': 'validate_physics_evidence',
                        'files': [{'path': name, 'sha256': hashlib.sha256(data).hexdigest()} for name, data in payloads.items()]}
        self.binding_path = self.root/'binding.json'
        self.binding_path.write_text(json.dumps(self.binding))
        self.raw = {'schema_version': '0.2.0', 'producer': {'component_name': 'open_pipe_stress_product_physics',
                    'component_version': '0.2.0', 'semantic_contract_id': physics.CONTRACT}, 'stub_expected': 'same-selected-origin'}

    def run_helper(self, source=None, raw=None, isolated=True, extra_env=None):
        command = [sys.executable, *(['-I', '-S'] if isolated else []), str(physics.HELPER),
                   '--source-root', str(source or self.source), '--binding', str(self.binding_path),
                   '--binding-sha256', hashlib.sha256(self.binding_path.read_bytes()).hexdigest()]
        env = os.environ.copy()
        if extra_env: env.update(extra_env)
        return subprocess.run(command, input=json.dumps(self.raw if raw is None else raw).encode(),
                              cwd=self.root, env=env, capture_output=True, timeout=5)

    def test_isolated_selected_module_and_table(self):
        result = self.run_helper()
        self.assertEqual(result.returncode, 0, result.stderr)
        reply = json.loads(result.stdout)
        self.assertEqual(reply['verdict'], 'consistent')
        self.assertEqual(reply['binding_status'], 'prepared_not_admitted')
        self.assertEqual(reply['dependencies'], self.binding['files'])
        self.assertTrue(reply['isolation']['isolated'])
        self.assertFalse(reply['isolation']['unit_authority_execution'])
        self.assertIn('internal physical consistency only', reply['claim'])

    def test_same_named_modules_in_cwd_or_pythonpath_cannot_shadow(self):
        (self.root/'json.py').write_text('raise RuntimeError("shadow json executed")')
        (self.root/'physics_evidence.py').write_text('raise RuntimeError("shadow reader executed")')
        result = self.run_helper(extra_env={'PYTHONPATH': str(self.root)})
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(json.loads(result.stdout)['verdict'], 'consistent')

    def test_wrong_candidate_module_table_or_unit_hash_refuses(self):
        for name in (physics.MODULE, physics.TABLE, physics.UNITS):
            path = self.source/name
            before = path.read_bytes()
            path.write_bytes(before + b'changed')
            result = self.run_helper()
            self.assertEqual(result.returncode, 2)
            self.assertIn('dependency digest mismatch', json.loads(result.stdout)['reason'])
            path.write_bytes(before)

    def test_unknown_profile_and_source_receipt_refuse(self):
        for change in [{'producer': {'semantic_contract_id': 'openpipestress.result_semantics/0.3.0/physics-source-1'}},
                       {'schema_version': '0.1.0'}, {'source_block_recovery': None}]:
            result = self.run_helper(raw=self.raw | change)
            self.assertEqual(result.returncode, 2)
            self.assertEqual(json.loads(result.stdout)['verdict'], 'refused')

    def test_public_consistency_refusal_stays_refusal(self):
        result = self.run_helper(raw=self.raw | {'stub_expected': 'wrong'})
        self.assertEqual(result.returncode, 2)
        self.assertIn('synthetic API refusal', json.loads(result.stdout)['reason'])

    def test_nonisolated_helper_refuses(self):
        result = self.run_helper(isolated=False)
        self.assertEqual(result.returncode, 2)
        self.assertIn('isolated Python', json.loads(result.stdout)['reason'])

    def test_preparation_binding_cannot_be_admitted_for_comparison(self):
        with self.assertRaisesRegex(ValueError, 'not admitted'):
            physics.load_binding(self.source, self.binding_path, hashlib.sha256(self.binding_path.read_bytes()).hexdigest())

    def test_wrong_binding_digest_refuses_before_source_execution(self):
        command = [sys.executable, '-I', '-S', str(physics.HELPER), '--source-root', str(self.source),
                   '--binding', str(self.binding_path), '--binding-sha256', '0'*64]
        result = subprocess.run(command, input=json.dumps(self.raw).encode(), capture_output=True, timeout=5)
        self.assertEqual(result.returncode, 2)
        self.assertIn('binding digest mismatch', json.loads(result.stdout)['reason'])


if __name__ == '__main__':
    unittest.main()
