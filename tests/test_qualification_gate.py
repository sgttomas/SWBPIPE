"""Synthetic-process and received-packet gate checks, not solver qualification.

Run as unittest to avoid the repository pytest setup's unrelated Rust build.
All explicit numbers below are harness-test data, not engineering criteria.
"""
from __future__ import annotations
from copy import deepcopy
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

PROJECT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PROJECT))
from tools.validation import qualification_gate as gate
from tools.validation.qualification_process import capture, sha256_bytes


class GateTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.input = {'request': {'request_id': 'invented-request', 'operation': 'solve'},
                      'solve': {'preview_model': {'model': {'project': {'id': 'invented-model'}}}}}
        self.rows = []
        for axis, value in [('x', 2.0), ('y', -3.0)]:
            self.rows.append({'id': 'result:' + axis, 'kind': 'global_nodal_displacement_' + axis, 'value': value,
                              'unit': 'mm', 'entity_ref': 'node:tip', 'basis_ref': {'ref_type': 'load_case', 'ref_id': 'LC1'},
                              'metadata': {'component': 'nodal_displacement_' + axis, 'coordinate_system': 'global',
                                           'location': 'node', 'basis': 'solved_from_global_linear_system', 'sign_convention': 'signed_global_component'}})
        self.mode = {'id': 'result:mode', 'kind': 'linear_solver_mode_basis', 'value': 1, 'unit': 'mode_code', 'entity_ref': 'solver'}
        self.payload = {'artifact': 'openpipestress.headless_runner_cli_output', 'schema_version': '1.0.0', 'command': 'solve', 'operation': 'solve',
                        'request_validation': {'diagnostics': []}, 'result_validation': {'diagnostics': []}, 'diagnostics': [],
                        'runner_result': {'run_id': 'run:headless-preview:invented-request', 'job': {'state': 'COMPLETED'}, 'analysis_status': ['MECHANICS_SOLVED'], 'diagnostics': []},
                        'mechanics_envelope': {'schema_version': '0.1.0', 'document_kind': 'openpipestress.product_preview.mechanics_result',
                                              'run_id': 'run:preview-linear-static-001', 'model_ref': 'invented-model',
                                              'status': {'mechanics': 'MECHANICS_SOLVED'}, 'accepted_model_state_mutated': False,
                                              'diagnostics': [], 'results': [self.mode, *self.rows]}}
        self.wrapper = {'payload': self.payload, 'decisions': [], 'findings': [], 'blocked': False, 'summary': {'blocking_count': 0, 'decision_count': 0, 'finding_count': 0, 'warning_count': 0}}
        self.reference = {'format': gate.REFERENCE_FORMAT, 'case_id': 'case:one', 'readiness': 'ready',
                          'reference_kind': 'harness_synthetic', 'basis': 'explicit invented gate fixture, not an analytical oracle',
                          'values': [{'assertion_id': axis, 'unit': 'mm', 'value': value} for axis, value in [('x', 2.0), ('y', -3.0)]]}
        self.criterion = {'schema_version': '0.1.0', 'tolerance_profile': {'profile_id': 'harness-only', 'profile_status': 'reviewed',
                          'rules': [{'rule_id': 'test-rule', 'result_family': 'displacement', 'dimension_id': 'length', 'unit_ref': {'object_type': 'unit', 'ref': 'mm'},
                                     'normalization_basis': 'same_unit_required', 'tolerance_value': 0,
                                     'tolerance_value_status': 'project_specific_review_required',
                                     'review': {'scope': 'synthetic gate fixture only'}, 'provenance': {'source': 'invented test values'}}]}}
        self.bin = self.root / 'fake-runner'
        self.write_process(self.wrapper)
        self.case = {'id': 'case:one', 'input': self.write_bound('input.json', self.input),
                     'reference': self.write_bound('reference.json', self.reference), 'criterion': self.write_bound('criterion.json', self.criterion),
                     'assertions': [{'id': axis, 'selector': {**{k: v for k, v in row.items() if k != 'value'}, 'dimension': 'length'},
                                     'criterion_rule_id': 'test-rule'} for axis, row in zip(('x', 'y'), self.rows)]}
        self.commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=PROJECT, text=True).strip()
        self.manifest = {'format': gate.MANIFEST_FORMAT, 'profile_id': 'synthetic-gate-only', 'purpose': 'harness_development',
                         'transport': gate.TRANSPORT, 'runner': {'candidate_commit': self.commit, 'executable_sha256': sha256_bytes(self.bin.read_bytes()),
                                                              'solver_mode': 'sparse_interactive'}, 'cases': [self.case]}
        self.counter = 0

    def write_bound(self, filename, value):
        data = json.dumps(value, allow_nan=False).encode()
        (self.root / filename).write_bytes(data)
        return {'path': filename, 'sha256': sha256_bytes(data)}

    def write_process(self, wrapper=None, code=None):
        body = code if code is not None else 'import sys\nsys.stdin.buffer.read()\nprint(' + repr(json.dumps(wrapper)) + ')\n'
        self.bin.write_text('#!' + sys.executable + '\n' + body)
        self.bin.chmod(0o700)

    def run_case(self, manifest=None, **kwargs):
        self.counter += 1
        selected = deepcopy(self.manifest if manifest is None else manifest)
        selected['runner']['executable_sha256'] = sha256_bytes(self.bin.read_bytes())
        path = self.root / f'selection-{self.counter}.json'
        path.write_text(json.dumps(selected))
        return gate.run_selection(path, self.bin, PROJECT, self.root / f'run-{self.counter}', **kwargs)

    def assert_complete_denominator(self, result, state):
        self.assertEqual(result['summary']['required_cases'], 1)
        self.assertEqual(result['summary']['required_assertions'], 2)
        self.assertEqual(result['summary']['assertions'][state], 2)
        self.assertNotEqual(result['outcome'], 'all_required_assertions_matched')

    def test_complete_actual_fake_process_and_raw_identity(self):
        result = self.run_case()
        self.assertEqual(result['outcome'], 'all_required_assertions_matched')
        self.assertEqual(result['summary']['assertions']['matched'], 2)
        self.assertEqual(result['qualification'], 'not_established_by_this_harness')
        process = result['cases'][0]['process']
        self.assertEqual(process['command'], [str(self.bin), 'solve'])
        self.assertEqual(process['stdin_sha256'], self.case['input']['sha256'])
        self.assertEqual((self.root / 'run-1/case-0000.reference.json').read_bytes(), (self.root / 'reference.json').read_bytes())
        self.assertEqual((self.root / 'run-1/case-0000.criterion.json').read_bytes(), (self.root / 'criterion.json').read_bytes())
        self.assertEqual(process['stdout_sha256'], sha256_bytes((self.root / 'run-1/case-0000/stdout.bin').read_bytes()))
        self.assertIn('legacy raw0.1', result['execution_basis']['producer_precision'])
        self.assertIn('Required: 1 cases / 2 assertions', (self.root / 'run-1/summary.md').read_text())
        self.assertEqual(json.loads((self.root / 'run-1/ledger.json').read_text()), result)

    def test_ledger_exists_before_process_starts(self):
        original = capture
        def observing(*args, **kwargs):
            ledger = json.loads((args[3].parent / 'ledger.json').read_text())
            self.assertEqual([x['state'] for x in ledger['cases'][0]['assertions']], ['not_run', 'not_run'])
            return original(*args, **kwargs)
        with patch.object(gate, 'capture', observing):
            self.assertEqual(self.run_case()['outcome'], 'all_required_assertions_matched')

    def test_wrong_value_sign_and_partial_missing_row_keep_denominator(self):
        for mutation in ('sign', 'missing'):
            wrapper = deepcopy(self.wrapper)
            rows = wrapper['payload']['mechanics_envelope']['results']
            if mutation == 'sign': rows[1]['value'] *= -1
            else: rows.pop(1)
            self.write_process(wrapper)
            result = self.run_case()
            self.assertEqual(result['summary']['required_assertions'], 2)
            self.assertEqual(result['summary']['assertions']['matched'], 1)
            self.assertNotEqual(result['outcome'], 'all_required_assertions_matched')

    def test_duplicate_rows_and_wrong_mode_cannot_pass(self):
        for mutation in ('duplicate', 'dense', 'fallback', 'missing_mode'):
            wrapper = deepcopy(self.wrapper); rows = wrapper['payload']['mechanics_envelope']['results']
            if mutation == 'duplicate': rows.append(deepcopy(rows[1]))
            elif mutation == 'missing_mode': rows.pop(0)
            else: rows[0]['value'] = 2 if mutation == 'dense' else 3
            self.write_process(wrapper)
            self.assert_complete_denominator(self.run_case(), 'error')

    def test_frame_unit_case_entity_and_kind_mismatch(self):
        for field, value in [('unit', 'm'), ('basis_ref', {'ref_type': 'load_case', 'ref_id': 'WRONG'}), ('entity_ref', 'wrong'), ('kind', 'unknown')]:
            wrapper = deepcopy(self.wrapper); wrapper['payload']['mechanics_envelope']['results'][1][field] = value
            self.write_process(wrapper); result = self.run_case()
            self.assertEqual(result['summary']['assertions']['error'], 1)
        wrapper = deepcopy(self.wrapper); wrapper['payload']['mechanics_envelope']['results'][1]['metadata']['coordinate_system'] = 'element_local'
        self.write_process(wrapper); self.assertEqual(self.run_case()['summary']['assertions']['error'], 1)

    def test_blocked_failed_unconverged_or_future_wrapper(self):
        for mutation in ('blocked', 'null', 'future', 'failed', 'nonconverged', 'request_identity', 'model_identity', 'mutated', 'validation'):
            wrapper = deepcopy(self.wrapper); payload = wrapper['payload']
            if mutation == 'blocked': wrapper['blocked'] = True
            elif mutation == 'null': wrapper['payload'] = None
            elif mutation == 'future': payload['mechanics_envelope']['schema_version'] = '0.2.0'
            elif mutation == 'failed': payload['runner_result']['job']['state'] = 'FAILED'
            elif mutation == 'nonconverged': payload['mechanics_envelope']['status']['mechanics'] = 'MECHANICS_UNRESOLVED'
            elif mutation == 'request_identity': payload['runner_result']['run_id'] = 'different'
            elif mutation == 'model_identity': payload['mechanics_envelope']['model_ref'] = 'different'
            elif mutation == 'mutated': payload['mechanics_envelope']['accepted_model_state_mutated'] = True
            else: payload['request_validation']['diagnostics'] = [{'severity': 'blocking'}]
            self.write_process(wrapper); self.assert_complete_denominator(self.run_case(), 'error')

    def test_nonfinite_nonnumeric_and_finite_overflow(self):
        for value in (None, True, '2', float('inf')):
            wrapper = deepcopy(self.wrapper); wrapper['payload']['mechanics_envelope']['results'][1]['value'] = value
            self.write_process(wrapper); result = self.run_case()
            self.assertNotEqual(result['outcome'], 'all_required_assertions_matched')
        self.reference['values'][0]['value'] = -1e308
        self.case['reference'] = self.write_bound('reference.json', self.reference)
        wrapper = deepcopy(self.wrapper); wrapper['payload']['mechanics_envelope']['results'][1]['value'] = 1e308
        self.write_process(wrapper); self.assertEqual(self.run_case()['summary']['assertions']['error'], 1)

    def test_existing_predicate_exact_boundary_pair_and_overflow(self):
        rule = self.criterion['tolerance_profile']['rules'][0]
        rule.update(relative_tolerance_value=0.0, absolute_tolerance_value=0.25)
        self.case['criterion'] = self.write_bound('criterion.json', self.criterion)
        for value, state in [(2.25, 'matched'), (2.2500000001, 'failed')]:
            wrapper = deepcopy(self.wrapper); wrapper['payload']['mechanics_envelope']['results'][1]['value'] = value
            self.write_process(wrapper); result = self.run_case()
            self.assertEqual(result['cases'][0]['assertions'][0]['state'], state)
        rule['relative_tolerance_value'] = 1e308
        self.case['criterion'] = self.write_bound('criterion.json', self.criterion)
        self.write_process(self.wrapper); self.assertEqual(self.run_case()['summary']['assertions']['error'], 2)

    def test_reference_selector_and_criterion_admission(self):
        for mutate in [lambda x: x['reference'].update(sha256='0'*64), lambda x: x['assertions'].append(deepcopy(x['assertions'][0])),
                       lambda x: x['assertions'][0]['selector'].update(dimension='force')]:
            selected = deepcopy(self.manifest); mutate(selected['cases'][0])
            if len(selected['cases'][0]['assertions']) > 2:
                with self.assertRaises(gate.AdmissionError): self.run_case(selected)
            else: self.assert_complete_denominator(self.run_case(selected), 'blocked')
        for change in [{'readiness': 'pending'}, {'values': self.reference['values'][:1]}, {'reference_kind': 'future'}]:
            ref = self.reference | change; self.case['reference'] = self.write_bound('reference.json', ref)
            self.assert_complete_denominator(self.run_case(), 'blocked')

    def test_unreviewed_negative_incomplete_or_wrong_unit_criterion(self):
        for change in [{'result_family': 'stress'}, {'tolerance_value': -1}, {'tolerance_value_status': 'TBD'}, {'relative_tolerance_value': 0.1}, {'unit_ref': {'ref': 'm'}}, {'review': {}}]:
            criterion = deepcopy(self.criterion); criterion['tolerance_profile']['rules'][0].update(change)
            self.case['criterion'] = self.write_bound('criterion.json', criterion)
            self.assert_complete_denominator(self.run_case(), 'blocked')

    def test_empty_duplicate_or_unsupported_selection(self):
        for change in [{'cases': []}, {'cases': [self.case, deepcopy(self.case)]}]:
            with self.assertRaises(gate.AdmissionError): self.run_case(self.manifest | change)
        for change in [{'transport': 'future'}, {'purpose': 'release_qualification'}]:
            self.assert_complete_denominator(self.run_case(self.manifest | change), 'blocked')
        selected = deepcopy(self.manifest); selected['runner']['solver_mode'] = 'dense_scrutiny'
        self.assert_complete_denominator(self.run_case(selected), 'blocked')
        selected = deepcopy(self.manifest); selected['runner']['candidate_commit'] = '0'*40
        self.assert_complete_denominator(self.run_case(selected), 'blocked')

    def test_nonzero_timeout_output_limit_and_malformed_output(self):
        cases = [('import sys\nprint("partial")\nsys.exit(7)', 'nonzero_exit', {}),
                 ('import time\nprint("partial",flush=True)\ntime.sleep(2)', 'timeout', {'timeout_seconds': .05}),
                 ('print("x"*1024)', 'output_limit', {'output_limit_bytes': 32}),
                 ('print("{")', 'completed', {})]
        for code, outcome, options in cases:
            self.write_process(code=code)
            result = self.run_case(**options)
            self.assert_complete_denominator(result, 'error')
            self.assertEqual(result['cases'][0]['process']['outcome'], outcome)
            self.assertTrue((self.root / f'run-{self.counter}/case-0000/stdout.bin').exists())

    def test_actual_received_legacy_packet_is_not_silently_new_transport(self):
        packet = (PROJECT / 'validation/witness/generated/tp_runner_015_final_cli_solve.json').read_bytes()
        self.assertEqual(json.loads(packet)['artifact'], 'openpipestress.headless_runner_cli_output')
        with self.assertRaisesRegex(gate.AdmissionError, 'unsupported controlled wrapper'):
            gate.unwrap(packet, self.input, 'sparse_interactive')

    def test_uncompared_nonfinite_row_is_not_silently_discarded(self):
        wrapper = deepcopy(self.wrapper)
        wrapper['payload']['mechanics_envelope']['results'].append({'id': 'extra', 'kind': 'extra', 'value': None})
        self.write_process(wrapper)
        self.assert_complete_denominator(self.run_case(), 'error')

    def test_supervisor_refuses_changed_executable_before_launch(self):
        with self.assertRaisesRegex(ValueError, 'before process launch'):
            capture(self.bin, ['solve'], b'{}', self.root/'refused-process', 1, 1024, expected_executable_sha256='0'*64)
        self.assertFalse((self.root/'refused-process/stdout.bin').exists())

    def test_actual_uppercase_blocking_and_inconsistent_wrapper_counts(self):
        for mutation in ('blocking', 'count', 'unknown_severity'):
            wrapper = deepcopy(self.wrapper)
            if mutation == 'count': wrapper['summary']['blocking_count'] = 1
            else:
                wrapper['findings'] = [{'severity': 'BLOCKING' if mutation == 'blocking' else 'FUTURE'}]
                wrapper['summary']['finding_count'] = 1
            self.write_process(wrapper)
            self.assert_complete_denominator(self.run_case(), 'error')

    def test_stdin_archive_mutation_cannot_change_delivered_bytes_or_pass(self):
        expected = sha256_bytes((self.root/'input.json').read_bytes())
        code = 'import sys,hashlib\nfrom pathlib import Path\nPath("stdin.bin").write_bytes(b"changed")\nactual=sys.stdin.buffer.read()\nprint(hashlib.sha256(actual).hexdigest(),file=sys.stderr)\nprint(' + repr(json.dumps(self.wrapper)) + ')'
        self.write_process(code=code)
        result = self.run_case()
        self.assert_complete_denominator(result, 'error')
        process = result['cases'][0]['process']
        self.assertEqual(process['outcome'], 'input_archive_changed')
        self.assertEqual((self.root/'run-1/case-0000/stderr.bin').read_text().strip(), expected)
        self.assertTrue(process['stdin_delivery_complete'])
        self.assertEqual(process['stdin_sha256'], expected)
        self.assertNotEqual(process['retained_stdin_sha256'], expected)

    def test_pipe_multiplexing_delivers_large_input_without_deadlock(self):
        data = b'x' * (1024 * 1024)
        self.write_process(code='import sys,hashlib\nsys.stdout.write("y"*100000);sys.stdout.flush()\ndata=sys.stdin.buffer.read()\nprint(hashlib.sha256(data).hexdigest(),file=sys.stderr)')
        result = capture(self.bin, ['solve'], data, self.root/'multiplexed', 5, 200000)
        self.assertEqual(result['outcome'], 'completed')
        self.assertEqual(result['stdin_bytes_delivered'], len(data))
        self.assertEqual((self.root/'multiplexed/stderr.bin').read_text().strip(), sha256_bytes(data))

    def test_closed_raw_rows_reject_contradictory_dimension(self):
        for field, value in [('dimension', 'force'), ('future_metadata', {})]:
            wrapper = deepcopy(self.wrapper)
            wrapper['payload']['mechanics_envelope']['results'][1][field] = value
            self.write_process(wrapper)
            self.assert_complete_denominator(self.run_case(), 'error')

    def test_reference_nonzero_underflow_cannot_match_zero(self):
        raw = (self.root/'reference.json').read_text().replace('2.0', '1e-999')
        (self.root/'reference.json').write_text(raw)
        self.case['reference']['sha256'] = sha256_bytes(raw.encode())
        wrapper = deepcopy(self.wrapper); wrapper['payload']['mechanics_envelope']['results'][1]['value'] = 0
        self.write_process(wrapper)
        self.assert_complete_denominator(self.run_case(), 'blocked')

    def test_actual_combination_token_supported_and_invented_token_refused(self):
        for assertion in self.case['assertions']:
            assertion['selector']['basis_ref'] = {'ref_type': 'combination', 'ref_id': 'COMB1'}
        wrapper = deepcopy(self.wrapper)
        for row in wrapper['payload']['mechanics_envelope']['results'][1:]:
            row['basis_ref'] = {'ref_type': 'combination', 'ref_id': 'COMB1'}
        self.write_process(wrapper)
        self.assertEqual(self.run_case()['outcome'], 'all_required_assertions_matched')
        self.case['assertions'][0]['selector']['basis_ref']['ref_type'] = 'load_combination'
        self.assert_complete_denominator(self.run_case(), 'blocked')

    def test_stdout_boundary_replacement_cannot_change_compared_value(self):
        emitted = deepcopy(self.wrapper)
        emitted['payload']['mechanics_envelope']['results'][1]['value'] = 99
        self.write_process(emitted)
        real_capture = gate.capture
        def replace_after_capture(*args, **kwargs):
            process = real_capture(*args, **kwargs)
            (args[3]/'stdout.bin').write_bytes(json.dumps(self.wrapper).encode())
            return process
        with patch.object(gate, 'capture', replace_after_capture), patch.object(gate, 'unwrap', wraps=gate.unwrap) as decoder:
            result = self.run_case()
        self.assert_complete_denominator(result, 'error')
        decoder.assert_not_called()
        self.assertEqual(result['cases'][0]['custody_at_publication'], 'failed')

    def test_stdout_growth_is_bounded_before_decoding(self):
        real_capture = gate.capture
        def grow_after_capture(*args, **kwargs):
            process = real_capture(*args, **kwargs)
            with (args[3]/'stdout.bin').open('wb') as stream:
                stream.truncate(gate.MAX_INPUT_BYTES + 1)
            return process
        with patch.object(gate, 'capture', grow_after_capture), patch.object(gate, 'unwrap', wraps=gate.unwrap) as decoder:
            result = self.run_case()
        self.assert_complete_denominator(result, 'error')
        decoder.assert_not_called()
        # The bounded reader reads only the selected length plus one; it never
        # allocates the grown archive just to discover a size/hash mismatch.
        import io
        calls = []
        class Reader(io.BytesIO):
            def read(self, size=-1):
                calls.append(size)
                return super().read(size)
        class Source:
            def open(self, mode):
                return Reader(b'x' * 100)
        with self.assertRaises(gate.AdmissionError):
            gate.read_captured_bytes(Source(), sha256_bytes(b'x' * 10), 10, 1000)
        self.assertEqual(calls, [11])

    def test_each_retained_snapshot_and_stream_is_rechecked(self):
        real_capture = gate.capture
        for name in ('case-0000.reference.json', 'case-0000.criterion.json', 'selection.json',
                     'case-0000/stdin.bin', 'case-0000/stderr.bin'):
            def change_after_capture(*args, **kwargs):
                process = real_capture(*args, **kwargs)
                (args[3].parent/name).write_bytes(b'{}')
                return process
            with patch.object(gate, 'capture', change_after_capture):
                self.assert_complete_denominator(self.run_case(), 'error')
        # The original reported reference-copy consequence used an actual child,
        # not a hook. Preserve that independent execution route as a regression.
        code = 'from pathlib import Path\nimport sys\nsys.stdin.buffer.read()\nPath("../case-0000.reference.json").write_bytes(b"{}")\nprint(' + repr(json.dumps(self.wrapper)) + ')'
        self.write_process(code=code)
        self.assert_complete_denominator(self.run_case(), 'error')

    def test_later_case_cannot_change_an_earlier_case_snapshot(self):
        selected = deepcopy(self.manifest)
        second = deepcopy(self.case)
        second['id'] = 'case:two'
        second_ref = deepcopy(self.reference)
        second_ref['case_id'] = second['id']
        second['reference'] = self.write_bound('reference-two.json', second_ref)
        selected['cases'].append(second)
        code = 'from pathlib import Path\nimport sys\nsys.stdin.buffer.read()\nif Path.cwd().name=="case-0001": Path("../case-0000.reference.json").write_bytes(b"{}")\nprint(' + repr(json.dumps(self.wrapper)) + ')'
        self.write_process(code=code)
        result = self.run_case(selected)
        self.assertEqual(result['summary']['required_assertions'], 4)
        self.assertEqual(result['summary']['assertions']['error'], 2)
        self.assertEqual(result['summary']['assertions']['matched'], 2)
        self.assertEqual(result['cases'][0]['custody_at_publication'], 'failed')
        self.assertNotEqual(result['outcome'], 'all_required_assertions_matched')

    def test_only_final_custody_checked_publication_can_claim_matches(self):
        publications = []
        real_record, real_unwrap = gate.atomic_record, gate.unwrap
        parsed_hashes = []
        def observe_record(path, value):
            publications.append((value['publication_phase'], value['outcome']))
            return real_record(path, value)
        def observe_bytes(raw, request, mode):
            parsed_hashes.append(sha256_bytes(raw))
            return real_unwrap(raw, request, mode)
        with patch.object(gate, 'atomic_record', observe_record), patch.object(gate, 'unwrap', observe_bytes):
            result = self.run_case()
        self.assertEqual(result['outcome'], 'all_required_assertions_matched')
        self.assertEqual(parsed_hashes, [result['cases'][0]['process']['stdout_sha256']])
        self.assertEqual(result['cases'][0]['custody_at_publication'], 'checked')
        self.assertTrue(all(phase == 'complete' for phase, outcome in publications if outcome == 'all_required_assertions_matched'))
        self.assertEqual(publications[-1], ('complete', 'all_required_assertions_matched'))

    def test_immutable_output_directory_and_reference_files(self):
        before = {name: (self.root / name).read_bytes() for name in ('input.json', 'reference.json', 'criterion.json')}
        self.run_case()
        for name, data in before.items(): self.assertEqual((self.root / name).read_bytes(), data)
        with self.assertRaises(FileExistsError):
            gate.run_selection(self.root/'selection-1.json', self.bin, PROJECT, self.root/'run-1')

    def test_reference_changed_during_run_is_error(self):
        self.write_process(code='from pathlib import Path\nPath(' + repr(str(self.root/'reference.json')) + ').write_text("changed")\nprint(' + repr(json.dumps(self.wrapper)) + ')')
        self.assert_complete_denominator(self.run_case(), 'error')

    def test_interrupted_case_preserves_subsequent_not_run_rows(self):
        selected = deepcopy(self.manifest)
        second = deepcopy(self.case); second['id'] = 'case:two'; selected['cases'].append(second)
        def interrupted(*args, **kwargs):
            raise KeyboardInterrupt()
        with patch.object(gate, 'capture', interrupted):
            result = self.run_case(selected)
        self.assertEqual(result['summary']['required_cases'], 2)
        self.assertEqual(result['summary']['required_assertions'], 4)
        self.assertEqual(result['summary']['assertions']['error'], 2)
        self.assertEqual(result['summary']['assertions']['not_run'], 2)

    def test_duplicate_json_rejected_without_parser_overwrite(self):
        for raw in [b'{"x":1,"x":2}', b'{"a":1,"\\u0061":2}', b'{"x":NaN}', b'{"x":Infinity}', b'{"x":1e309}', b'{"x":1e-999}', b'\xff']:
            with self.assertRaises(gate.AdmissionError): gate.strict_json(raw)

    def test_executable_and_semantic_digest_mismatch_blocks_before_launch(self):
        selected = deepcopy(self.manifest); selected['runner']['executable_sha256'] = '0'*64
        path = self.root / 'bad-hash-selection.json'; path.write_text(json.dumps(selected))
        result = gate.run_selection(path, self.bin, PROJECT, self.root/'bad-hash-run')
        self.assert_complete_denominator(result, 'blocked')
        with patch.object(gate, 'SEMANTICS_SHA256', '0'*64):
            self.assert_complete_denominator(self.run_case(), 'blocked')


if __name__ == '__main__':
    unittest.main()
