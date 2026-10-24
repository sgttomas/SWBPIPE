"""Bounded adapter tests for the ordinary pressure-2 reader verdict; a synthetic reader stub is not solver evidence.

b7-plan R2: the adapter takes the Rust product reader's verdict (`openpipestress-runner read pressure`, named by
OPENPIPESTRESS_RUNNER_BIN) on the parsed snapshot, after the binding's selected reader source, table and unit
authority are checked. The stub tests exercise process custody and response checking; the real reader is run
on the committed pressure-2 statements. B7b retired physics-1: its binding formats and identities are refused,
and the frozen first-static inputs are submitted as a declared re-authored copy.
"""
from __future__ import annotations
from copy import deepcopy
import hashlib
import json
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

PROJECT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PROJECT))
from tools.validation import qualification_physics as physics

STUB = '''import json, sys
request = json.load(sys.stdin)
source = request["source"]
if source.get("stub_garbage"):
    print("not a verdict"); sys.exit(0)
if source.get("stub_crash"):
    print("openpipestress-runner: input is not usable", file=sys.stderr); sys.exit(2)
ok = not source.get("stub_refuse")
outcome = {"ok": True} if ok else {"ok": False, "error": {"code": "SOURCE_PHYSICS_SYNTHETIC", "detail": None}}
print(json.dumps({"artifact": "openpipestress.result_reader_verdict", "schema_version": "1.0.0", "reader": sys.argv[2], "outcome": outcome}))
sys.exit(0 if ok else 1)
'''


class ReaderVerdictTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name).resolve()
        self.source = self.root / 'candidate'
        self.source.mkdir()
        payloads = {physics.MODULE: b'// synthetic reader source binding; the stub below reads\n',
                    physics.TABLE: (PROJECT / physics.TABLE).read_bytes(), physics.UNITS: b'// synthetic unit authority binding, no execution\n'}
        for path, data in payloads.items():
            target = self.source / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
        self.dependencies = {name: hashlib.sha256(data).hexdigest() for name, data in payloads.items()}
        self.binding = {'format': physics.BINDING_FORMAT, 'status': 'prepared_not_admitted', 'contract_id': physics.CONTRACT,
                        'entrypoint': physics.ENTRYPOINT,
                        'files': [{'path': name, 'sha256': digest} for name, digest in self.dependencies.items()]}
        self.binding_path = self.root / 'binding.json'
        self.binding_path.write_text(json.dumps(self.binding))
        self.stub = self.root / 'stub-reader'
        self.stub.write_text('#!' + sys.executable + '\n' + STUB)
        self.stub.chmod(0o700)
        self.raw = {'schema_version': '0.2.0', 'producer': {'component_name': 'open_pipe_stress_product_physics',
                    'component_version': '0.2.0', 'semantic_contract_id': physics.CONTRACT}}
        self.count = 0

    def loaded(self):
        with patch.object(physics, 'DEPENDENCIES', self.dependencies):
            return physics.load_binding(self.source, self.binding_path, hashlib.sha256(self.binding_path.read_bytes()).hexdigest(),
                                        require_reviewed=False)

    def observe(self, raw=None, binding=None):
        self.count += 1
        with patch.dict(os.environ, {'OPENPIPESTRESS_RUNNER_BIN': str(self.stub)}):
            return physics.consistency_observation(self.raw if raw is None else raw, binding or self.loaded(), self.root / f'reader-{self.count}')

    def test_the_reader_verdict_is_taken_from_the_runner(self):
        observation = self.observe()
        self.assertEqual(observation['response']['verdict'], 'consistent')
        self.assertEqual(observation['process']['command'], [str(self.stub), 'read', 'pressure'])
        self.assertEqual(json.loads((self.root / 'reader-1/stdin.bin').read_bytes()), {'source': self.raw})
        self.assertEqual(observation['reader_executable_sha256'], hashlib.sha256(self.stub.read_bytes()).hexdigest())
        self.assertEqual(observation['response']['dependencies'], self.binding['files'])
        self.assertFalse(observation['binding_admitted'])
        self.assertIn('internal physical consistency only', observation['response']['claim'])

    def test_a_reader_refusal_stays_a_refusal(self):
        with self.assertRaisesRegex(ValueError, 'owned reader refused: SOURCE_PHYSICS_SYNTHETIC'):
            self.observe(raw=self.raw | {'stub_refuse': True})

    def test_an_unknown_response_or_failed_reader_is_an_error(self):
        with self.assertRaisesRegex(ValueError, 'unknown reader response'):
            self.observe(raw=self.raw | {'stub_garbage': True})
        with self.assertRaisesRegex(ValueError, 'unknown reader response'):
            self.observe(raw=self.raw | {'stub_crash': True})

    def test_wrong_candidate_reader_table_or_unit_hash_refuses_before_reading(self):
        binding = self.loaded()
        for name in (physics.MODULE, physics.TABLE, physics.UNITS):
            path = self.source / name
            before = path.read_bytes()
            path.write_bytes(before + b'changed')
            with self.assertRaisesRegex(ValueError, 'bound bytes differ'):
                self.observe(binding=binding)
            path.write_bytes(before)
        self.assertFalse(any((self.root / f'reader-{index}').exists() for index in range(1, self.count + 1)))

    def test_unknown_profile_and_source_receipt_refuse(self):
        retired = dict(self.raw['producer'], semantic_contract_id='openpipestress.result_semantics/0.3.0/physics-1')
        for change in [{'producer': {'semantic_contract_id': 'openpipestress.result_semantics/0.3.0/physics-source-1'}},
                       {'producer': retired}, {'schema_version': '0.1.0'}, {'source_block_recovery': None}]:
            with self.assertRaisesRegex(ValueError, 'ORDINARY-PHYSICS'):
                self.observe(raw=self.raw | change)

    def test_preparation_binding_cannot_be_admitted_for_comparison(self):
        with patch.object(physics, 'DEPENDENCIES', self.dependencies), self.assertRaisesRegex(ValueError, 'not admitted'):
            physics.load_binding(self.source, self.binding_path, hashlib.sha256(self.binding_path.read_bytes()).hexdigest())

    def test_a_former_python_or_physics_one_reader_binding_is_refused(self):
        # Version 1 named the former Python reader, version 2 the retired physics-1 Rust reader (B7b).
        for version in (1, 2):
            self.binding_path.write_text(json.dumps(dict(self.binding, format=f'openpipestress.ordinary_physics_consistency_binding/{version}')))
            with self.assertRaisesRegex(ValueError, 'unsupported binding identity'):
                self.loaded()
        retired = dict(self.binding, contract_id='openpipestress.result_semantics/0.3.0/physics-1')
        self.binding_path.write_text(json.dumps(retired))
        with self.assertRaisesRegex(ValueError, 'unsupported binding identity'):
            self.loaded()
        self.binding_path.write_text(json.dumps(dict(self.binding, entrypoint='validate_physics_evidence')))
        with self.assertRaisesRegex(ValueError, 'unsupported owned entry'):
            self.loaded()


class ReauthoringTests(unittest.TestCase):
    """B7b: the frozen first-static runner inputs are submitted as a declared re-authored copy (the product tests'
    one re-authoring rule); the frozen bytes, and everything but the model version, contract and explicit zero
    corrosion allowance, are unchanged."""

    FIRST_STATIC = PROJECT / 'validation/qualification/fixtures/first_static'

    def test_the_frozen_inputs_are_re_authored_as_a_copy_only(self):
        for slug in ('axial', 'bending_torsion'):
            path = self.FIRST_STATIC / f'{slug}.runner_input.candidate.json'
            data = path.read_bytes()
            original = json.loads(data)
            submitted, record = physics.reauthor_runner_input(data)
            self.assertEqual(path.read_bytes(), data)
            self.assertEqual(record['frozen_input_sha256'], hashlib.sha256(data).hexdigest())
            self.assertEqual(record['submitted_input_sha256'], hashlib.sha256(submitted).hexdigest())
            request = json.loads(submitted)
            model = request['solve']['preview_model']['model']
            self.assertEqual(model['schema_version'], physics.MODEL_VERSION)
            self.assertEqual(model['pressure_contract'], physics.PRESSURE_CONTRACT)
            self.assertEqual([pipe['section']['corrosion_allowance'] for pipe in model['pipe_segments']], [physics.ZERO_CORROSION_ALLOWANCE])
            # Nothing else changes.
            expected = deepcopy(original)
            source = expected['solve']['preview_model']['model']
            self.assertEqual((source['schema_version'], source['pressure_contract']), ('0.3.0', {'version': '2.0.0', 'mode': 'exact_straight_pressure_v2'}))
            source.update(schema_version=physics.MODEL_VERSION, pressure_contract=physics.PRESSURE_CONTRACT)
            for pipe in source['pipe_segments']:
                pipe['section']['corrosion_allowance'] = physics.ZERO_CORROSION_ALLOWANCE
            self.assertEqual(request, expected)
            # The re-authoring is the identity on its own output.
            self.assertEqual(physics.reauthor_runner_input(submitted)[0], submitted)

    def test_other_contracts_and_present_allowances_are_kept(self):
        model = {'schema_version': '0.4.0', 'pressure_contract': {'version': '1.0.0', 'mode': 'legacy_pressure_v1'},
                 'pipe_segments': [{'section': {}}]}
        self.assertEqual(physics.reauthor_model(model), dict(model, schema_version='0.7.0'))
        # A 0.5.0 or 0.6.0 document is re-authored by its version alone: nothing else moves, even an incomplete one.
        for version in ('0.5.0', '0.6.0'):
            previous = {'schema_version': version, 'pressure_contract': {'version': '3.0.0', 'mode': 'exact_pressure_v3'},
                        'pipe_segments': [{'section': {}}]}
            self.assertEqual(physics.reauthor_model(previous), dict(previous, schema_version='0.7.0'))
        model = {'schema_version': '0.4.0', 'pressure_contract': {'version': '3.0.0', 'mode': 'exact_pressure_v3'},
                 'pipe_segments': [{'section': {'corrosion_allowance': {'value': 0.001, 'unit': 'm'}}}, {'section': {}}]}
        reauthored = physics.reauthor_model(model)
        self.assertEqual(reauthored['pressure_contract'], physics.PRESSURE_CONTRACT)
        self.assertEqual([pipe['section']['corrosion_allowance'] for pipe in reauthored['pipe_segments']],
                         [{'value': 0.001, 'unit': 'm'}, physics.ZERO_CORROSION_ALLOWANCE])
        self.assertNotIn('corrosion_allowance', model['pipe_segments'][1]['section'])


class RustReaderTests(unittest.TestCase):
    """The real Rust reader (OPENPIPESTRESS_RUNNER_BIN) on the committed pressure-2 statements, with the pinned binding."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name).resolve()
        self.binding_path = self.root / 'binding.json'
        self.binding_path.write_text(json.dumps({'format': physics.BINDING_FORMAT, 'status': 'prepared_not_admitted', 'contract_id': physics.CONTRACT,
                                                 'entrypoint': physics.ENTRYPOINT,
                                                 'files': [{'path': name, 'sha256': digest} for name, digest in physics.DEPENDENCIES.items()]}))

    def test_the_pins_are_the_working_tree_reader_table_and_units(self):
        for name, digest in physics.DEPENDENCIES.items():
            self.assertEqual(hashlib.sha256((PROJECT / name).read_bytes()).hexdigest(), digest,
                             f'{name} differs from its pin: re-pin deliberately when the reader changes')

    def test_the_rust_reader_admits_and_refuses_committed_statements(self):
        binding = physics.load_binding(PROJECT, self.binding_path, hashlib.sha256(self.binding_path.read_bytes()).hexdigest(), require_reviewed=False)
        for mode in ('sparse', 'dense'):
            raw = json.loads((PROJECT / f'fixtures/results/pressure_connected_mechanics_{mode}.json').read_text())
            observation = physics.consistency_observation(raw, binding, self.root / f'{mode}-reader')
            self.assertEqual(observation['response']['verdict'], 'consistent')
            self.assertEqual(observation['process']['command'][1:], ['read', 'pressure'])
            broken = deepcopy(raw)
            broken['contract_evidence']['exact_cases'].pop()
            with self.assertRaisesRegex(ValueError, 'owned reader refused: SOURCE_PHYSICS_'):
                physics.consistency_observation(broken, binding, self.root / f'{mode}-refused')


if __name__ == '__main__':
    unittest.main()
