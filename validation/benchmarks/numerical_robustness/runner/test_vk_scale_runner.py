#!/usr/bin/env python3
"""Tests of `vk_scale_runner.py`'s stop rules and its one named exception.

The exception is ROOT's ruling for V-K's B: at 10,000 members,
`Unresolved(ExactSumSpan)` is the KF3 availability finding. It is recorded and
the tier continues. Standard library only:

    python3 -m unittest runner/test_vk_scale_runner.py   (from VR's directory)
"""
import os
import sys
import unittest

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import vk_scale_runner as v  # noqa: E402

ROWS = 103
SPAN = 'Unresolved ExactSumSpan [Restrained]'


def objects(outcome=SPAN, rows=ROWS, published=False, extra_failures=(), rcm_equal=True,
            counts_match=True, not_covered_equal=True, s_full=None):
    case = 'RF-LARGE-CHAIN-n10000-AX'
    selected = outcome.startswith('Selected')
    failures = [] if selected else ['%s: not selected: %s' % (case, outcome)]
    failures += list(extra_failures)
    fail = 0 if selected else rows
    return [
        {'kind': 'w1', 'outcome': outcome, 'counts_match_storage': counts_match},
        {'kind': 'report', 'rows': rows, 'pass': rows if selected else (1 if published else 0),
         'pass_absolute_range': 0, 'not_covered': 0, 'structural_zero': 0, 'expected_unresolved': 0,
         'fail': fail - (1 if published and not selected else 0), 'accounted': True,
         'failures': len(failures), 'failures_head': failures[:50],
         'not_covered_rows': [], 'not_covered_committed': [], 'not_covered_equal': not_covered_equal,
         'not_covered_rows_s_full': s_full, 'class_mismatches': [],
         'controls_discriminated': 3, 'controls_non_discriminating': 0,
         'controls_undiscriminated': [], 'controls_unexpectedly_failing': []},
        {'kind': 'rcm', 'equal': rcm_equal},
    ]


OK = {'classification': 'ok'}


def run(members):
    return {'members': members, 'model': 'RF-LARGE-CHAIN-n%05d-AX' % members}


class Kf3AvailabilityException(unittest.TestCase):
    def test_exact_sum_span_at_10000_members_is_recorded_and_the_tier_continues(self):
        objs = objects()
        self.assertEqual(v.availability_exception(run(10000), objs), v.KF3_AVAILABILITY)
        self.assertEqual(v.stop_reasons(OK, objs, True, run(10000)), [])

    def test_exact_sum_span_below_10000_members_still_stops(self):
        for members in (100, 1000):
            objs = objects()
            self.assertIsNone(v.availability_exception(run(members), objs))
            reasons = v.stop_reasons(OK, objs, True, run(members))
            self.assertTrue(any(r.startswith('not selected') for r in reasons), reasons)

    def test_any_other_unresolved_reason_still_stops(self):
        for outcome in ('Unresolved Ceiling [Restrained]', 'Unresolved ExponentRange [Restrained]',
                        'Unresolved Budget(Case) [Restrained]', 'Refused MechanismWitnessed { body: 0 } []'):
            objs = objects(outcome=outcome)
            self.assertIsNone(v.availability_exception(run(10000), objs))
            self.assertTrue(v.stop_reasons(OK, objs, True, run(10000)), outcome)

    def test_a_failure_on_a_covered_row_still_stops(self):
        objs = objects(extra_failures=['RF-LARGE-CHAIN-n10000-AX: u.C1.UX (1e-3) predicate'])
        self.assertIsNone(v.availability_exception(run(10000), objs))
        self.assertTrue(v.stop_reasons(OK, objs, True, run(10000)))

    def test_a_published_row_voids_the_exception(self):
        objs = objects(published=True)
        self.assertIsNone(v.availability_exception(run(10000), objs))
        self.assertTrue(v.stop_reasons(OK, objs, True, run(10000)))

    def test_every_other_check_still_applies_under_the_exception(self):
        for kwargs, expect in (({'rcm_equal': False}, "K4's RCM differs"),
                               ({'counts_match': False}, "V-K's counts differ"),
                               ({'not_covered_equal': False}, 'not-covered set'),
                               ({'s_full': ['u.C1.UX']}, 'C9')):
            objs = objects(**kwargs)
            reasons = v.stop_reasons(OK, objs, True, run(10000))
            self.assertTrue(any(expect in r for r in reasons), (kwargs, reasons))

    def test_a_process_that_did_not_complete_still_stops(self):
        for classification in ('killed_by_rss_watchdog', 'heap_cap_abort', 'timed_out', 'error'):
            record = {'classification': classification, 'classification_detail': None}
            self.assertTrue(v.stop_reasons(record, objects(), True, run(10000)), classification)

    def test_a_selected_case_with_no_failure_does_not_stop(self):
        objs = objects(outcome='Selected at 128')
        self.assertIsNone(v.availability_exception(run(10000), objs))
        self.assertEqual(v.stop_reasons(OK, objs, True, run(10000)), [])



class LaunchContextBinding(unittest.TestCase):
    def test_actual_case_and_model_path_are_rebound_before_admission(self):
        from unittest.mock import patch
        run = dict(v.schedule()[12], run_id='binding-test')
        row = {'kind': 'counts', 'case': run['model'], 'estimate_max_bytes': 123456}
        with patch.object(v.k6, 'launch', return_value={'classification': 'ok'}) as launch, \
                patch.object(v, 'objects_of', return_value=[row]):
            result = v.bind_scale_launch_counts('/new/path/vk_scale', run,
                                                 '/changed/models', '/records')
        argv = launch.call_args.args[0]
        expected = v.binary_argv('/new/path/vk_scale', run, '/changed/models',
                                counts_only=True, heap_cap=v.COUNTS_CAP_BYTES)
        self.assertEqual(argv, expected)
        self.assertEqual(result['estimate_max_bytes'], 123456)
        self.assertEqual(result[v.k6.estimate_key(v.MODE)], 123456)
        self.assertEqual(result['model'], run['model'])
        self.assertEqual(launch.call_args.kwargs['run_id'], 'binding-test.counts-binding')

    def test_plan_stays_process_free(self):
        import contextlib
        import io
        from unittest.mock import patch
        with patch.object(v.k6, 'read_counts', return_value={}), \
                patch.object(v.k6, 'launch') as launch, contextlib.redirect_stdout(io.StringIO()):
            rows = v.plan('counts.jsonl', None, False)
        self.assertEqual(len(rows), len(v.schedule()))
        launch.assert_not_called()

    def test_tier_admission_and_record_denominator_use_the_refreshed_context(self):
        import tempfile
        from unittest.mock import patch
        run = dict(v.schedule()[0], run_id='binding-test')
        fresh = {'model': run['model'], 'estimate_max_bytes': 987654,
                 v.k6.estimate_key(v.MODE): 987654}
        with tempfile.TemporaryDirectory() as directory:
            with open(os.path.join(directory, 'metadata.json'), 'w') as fh:
                fh.write('{}')
            with patch.object(v, 'schedule', return_value=[run]), \
                    patch.object(v.k6, 'read_counts', return_value={run['model']: {'estimate_max_bytes': 1}}), \
                    patch.object(v.k6, 'read_records', return_value=[]), \
                    patch.object(v.k6, 'launch', return_value={}), \
                    patch.object(v, 'bind_scale_launch_counts', return_value=fresh) as bind, \
                    patch.object(v, 'admission', return_value={'decision': 'deferred', 'reason': 'test'}) as admission, \
                    patch.object(v.k6, 'append_record') as append:
                v.run_tier(run['tier'], '/binary', '/changed-models', '/counts', directory,
                           'source', 'tree', log=lambda _: None)
            bind.assert_called_once()
            self.assertEqual(admission.call_args.args[1][run['model']], fresh)
            self.assertEqual(append.call_args.args[1]['estimate_adm_bytes'], 987654)

    def test_failed_or_incomplete_binding_never_uses_old_estimate(self):
        from unittest.mock import patch
        run = dict(v.schedule()[0], run_id='binding-test')
        for outcome, rows in [('error', []), ('ok', []),
                              ('ok', [{'kind': 'counts', 'case': run['model'],
                                       'estimate_max_bytes': 0}])]:
            with patch.object(v.k6, 'launch', return_value={'classification': outcome}), \
                    patch.object(v, 'objects_of', return_value=rows):
                with self.assertRaises(RuntimeError):
                    v.bind_scale_launch_counts('/bin/vk_scale', run, '/models', '/records')


class ModelEligibilityBeforeLaunch(unittest.TestCase):
    def test_legacy_admission_priority_and_estimate_diagnostics_remain(self):
        run = next(x for x in v.schedule() if x['members'] == 10000)
        self.assertEqual(v.admission(run, {}, [], None, None, False)['reason'], 'deferred:no_counts_line')
        row = {'estimate_max_bytes': run['heap_cap_bytes'],
               v.k6.estimate_key(v.MODE): run['heap_cap_bytes']}
        result = v.admission(run, {run['model']: row}, [], None, None, False)
        self.assertTrue(result['reason'].startswith("deferred:the binary's backstop would refuse"))
        row = {'estimate_max_bytes': 321, v.k6.estimate_key(v.MODE): 321}
        self.assertEqual(v.admission(run, {run['model']: row}, [], None, None, False)['reason'],
                         'deferred:10000_members_needs_root_approval')
        result = v.admission(run, {run['model']: row}, [], None, None, True)
        self.assertIn('ascent_previous_size_not_recorded', result['reason'])
        self.assertEqual(result['estimate_adm_bytes'], 321)

    def test_approval_and_ascent_holds_have_only_the_tier_noop(self):
        import tempfile
        from unittest.mock import patch
        run = next(x for x in v.schedule() if x['members'] == 10000)
        for approved, reason in [(False, 'deferred:10000_members_needs_root_approval'),
                                 (True, 'deferred:ascent_previous_size_not_recorded')]:
            with self.subTest(approved=approved), tempfile.TemporaryDirectory() as folder:
                with open(os.path.join(folder, 'metadata.json'), 'w') as f:
                    f.write('{}')
                with patch.object(v, 'schedule', return_value=[run]), \
                        patch.object(v.k6, 'read_counts', return_value={}), \
                        patch.object(v.k6, 'launch', return_value={'rss': {}}) as launch, \
                        patch.object(v, 'bind_scale_launch_counts', side_effect=AssertionError('held model bound counts')) as bind, \
                        patch.object(v, 'admission', side_effect=AssertionError('held model reached numeric admission')) as numeric, \
                        patch.object(v.k6, 'wait_for_quiet_host') as quiet:
                    v.run_tier(run['tier'], 'binary', '/models', None, folder, 'c', 't',
                               approve_10000=approved, log=lambda *a: None)
                self.assertEqual(launch.call_count, 1)
                self.assertIn('--noop', launch.call_args.args[0])
                bind.assert_not_called()
                numeric.assert_not_called()
                quiet.assert_not_called()
                record = v.k6.run_entries(v.k6.read_records(folder))[0]
                self.assertEqual(record['classification'], 'not_run')
                self.assertTrue(record['admission']['reason'].startswith(reason))
                self.assertIsNone(record['estimate_adm_bytes'])

    def test_approved_recorded_model_rebinds_before_numeric_use_with_original_caps(self):
        import json
        import tempfile
        from unittest.mock import patch
        run = next(x for x in v.schedule() if x['members'] == 10000)
        events = []
        with tempfile.TemporaryDirectory() as folder:
            with open(os.path.join(folder, 'metadata.json'), 'w') as f:
                f.write('{}')
            v.k6.append_record(folder, dict(run, run_id='previous', model=v.k6.previous_size(run['model']),
                                           members=1000, classification='timed_out'))
            def fake_launch(argv, *, record_dir, run_id, **kw):
                kind = 'baseline' if '--noop' in argv else 'prepass' if '--counts-only' in argv else 'normal'
                events.append((kind, argv))
                with open(os.path.join(record_dir, run_id + '.jsonl'), 'w') as f:
                    f.write(json.dumps({'kind': 'counts', 'case': run['model'],
                                        'estimate_max_bytes': 321}) + '\n')
                return {'classification': 'ok', 'wall_s': 0.0, 'rss': {}, 'run_id': run_id}
            original = v.admission
            def numeric(run, counts, *a, **kw):
                events.append(('numeric', counts[run['model']]['estimate_max_bytes']))
                return original(run, counts, *a, **kw)
            with patch.object(v, 'schedule', return_value=[run]), \
                    patch.object(v.k6, 'read_counts', return_value={run['model']: {'estimate_max_bytes': 99999999999}}), \
                    patch.object(v.k6, 'launch', side_effect=fake_launch), \
                    patch.object(v.k6, 'wait_for_quiet_host', return_value={}), \
                    patch.object(v, 'admission', side_effect=numeric), \
                    patch.object(v, 'stop_reasons', return_value=[]):
                v.run_tier(run['tier'], 'binary', '/models', None, folder, 'c', 't',
                           approve_10000=True, log=lambda *a: None)
            self.assertEqual([x[0] for x in events], ['baseline', 'prepass', 'numeric', 'normal'])
            self.assertEqual(events[2][1], 321)
            pre, normal = events[1][1], events[3][1]
            self.assertEqual(pre[pre.index('--heap-cap-bytes') + 1], str(v.COUNTS_CAP_BYTES))
            self.assertEqual(normal[normal.index('--heap-cap-bytes') + 1], str(run['heap_cap_bytes']))
            self.assertEqual(pre[pre.index('--model-file') + 1], normal[normal.index('--model-file') + 1])
            self.assertEqual(v.k6.run_entries(v.k6.read_records(folder))[-1]['estimate_adm_bytes'], 321)

if __name__ == '__main__':
    unittest.main()
