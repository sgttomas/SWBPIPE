#!/usr/bin/env python3
"""Release timing runner tests (DEL-04-05, RT-U4): standard library `unittest`.

The pure tests need no child process and no Rust binary. `LiveLaunch` runs two tiny
self-limiting Python children through the runner's own run path: a stand-in driver that prints
one `release-timing-v1` line, and (macOS) a growing child under a 128 MiB cap with a failsafe
exit at 512 MiB. Nothing measures or asserts a time or memory bound.

Run from this directory: python3 -m unittest test_release_runner
"""
import getpass
import hashlib
import io
import json
import os
import platform
import socket
import stat
import subprocess
import sys
import tempfile
import time
import types
import unittest
from contextlib import redirect_stdout
from unittest import mock

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import k6_runner as k6  # noqa: E402
import release_runner as rr  # noqa: E402

BUDGET = rr.BASE_MACHINE_BUDGET_BYTES


def launched(classification='ok', exit_code=0, **extra):
    record = {'classification': classification, 'exit_code': exit_code, 'timeout_s': 60, 'wall_s': 1.5,
              'rss': {'time_max_rss_bytes': 3 * k6.GIB, 'time_peak_footprint_bytes': 2 * k6.GIB,
                      'ps_max_kib': 3 * 1024 * 1024}}
    record.update(extra)
    return record


def driver(status=rr.SOLVED, codes=(), footprint=2 * k6.GIB, **extra):
    line = {'schema': rr.DRIVER_SCHEMA, 'status': status, 'blocking_codes': list(codes), 'route': 'EX',
            'nodes': 2000, 'members': 2100, 'supports': 300, 'cases': 3, 'combinations': 2,
            'rows': 1000, 'output_bytes': 123456, 'output_sha256': 'f' * 64,
            'rows_per_basis': {'load_case:L1': 600, 'combination:C1': 400},
            'row_classes': {'station': {'rows': 800, 'bytes': 100000}},
            'seconds': {'read': 0.1, 'parse': 0.2, 'request': 0.0, 'entry': 5.0, 'serialize': 1.0,
                        'release_envelope': 0.1, 'parse_back': 1.1, 'total': 7.5},
            'peak_memory': {'entry': {'ru_maxrss_bytes': 3 * k6.GIB, 'phys_footprint_bytes': k6.GIB,
                                      'lifetime_max_phys_footprint_bytes': footprint},
                            'parse_back': {'ru_maxrss_bytes': 3 * k6.GIB + 5, 'phys_footprint_bytes': k6.GIB,
                                           'lifetime_max_phys_footprint_bytes': footprint}},
            'worker_peak_footprint_bytes_equivalent': footprint, 'limits': ['a limit']}
    line.update(extra)
    return line


# A synthetic plan: the run-loop tests depend on these rows, not on the default plan's contents.
SYNTHETIC_ROWS = (
    # profile, nodes, timeout_s, estimate_s, expect
    ('linear-ex', 100, 300, 70, 'solved'),
    ('linear-ex', 200, 600, 260, 'solved'),
    ('nonlinear-pf', 100, 60, 4, 'solved'),
    ('nonlinear-pf', 200, 120, 17, 'solved'),
    ('nonlinear-pf', 300, 300, 80, 'solved'),
    ('nonlinear-pf', 400, 60, 2, 'refused:NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED'),
    ('nonlinear-ex', 100, 300, 15, 'refused'),
)


def synthetic_plan(mode='sparse'):
    return rr.build_plan(SYNTHETIC_ROWS, mode)


class Plan(unittest.TestCase):
    """Properties of the default plan, and the plan machinery on a synthetic plan."""

    def test_default_plan_properties(self):
        plan = rr.default_plan()
        self.assertTrue(plan)
        self.assertEqual([r['order'] for r in plan], list(range(len(plan))), 'every run is in plan order')
        self.assertEqual(len({r['run_id'] for r in plan}), len(plan))
        timed = [r for r in plan if not r['attribute']]
        self.assertEqual(len({(r['profile'], r['nodes']) for r in timed}), len(timed),
                         'one model per (profile, nodes)')
        for r in plan:
            if r['attribute']:
                self.assertTrue(r['run_id'].endswith('-attr'))
                self.assertIn((r['profile'], r['nodes'], r['mode']),
                              {(t['profile'], t['nodes'], t['mode']) for t in timed}, 'an attribution run has a twin')
        for r in plan:
            self.assertIn(r['profile'], rr.PROFILES)
            self.assertEqual(r['footprint_budget_bytes'], BUDGET)
            self.assertEqual(r['rss_cap_bytes'], rr.EMERGENCY_RSS_CAP_BYTES)
            self.assertEqual(r['mode'], 'sparse')
            self.assertEqual(r['run_id'], rr.run_id_of(r['profile'], r['nodes'], r['mode'], r['attribute']))
            self.assertIsNotNone(rr.as_expected(r['expect'], 'ok', None), 'expectation %r unknown' % r['expect'])
        for name in ('window', 'window2'):
            window = rr.select(plan, name)
            self.assertTrue(window)
            self.assertLessEqual(rr.plan_total_s(window) + rr.QUIET_WAIT_S, rr.WINDOW_BUDGET_S, name)
        self.assertFalse(any(r['attribute'] for r in rr.select(plan, 'window')), 'the timed window samples nothing')
        self.assertEqual(sorted(r['attribute'] for r in rr.select(plan, 'window2')), [False, True])
        criterion = {r['run_id'] for r in rr.select(plan, 'criterion')}
        self.assertTrue(criterion)
        self.assertEqual({r['run_id'] for r in plan if r['nodes'] == 2000}, criterion)
        for name in rr.SUBSETS:
            subset = rr.select(plan, name)
            self.assertEqual(subset, [r for r in plan if r in subset], '%s keeps plan order' % name)

    def test_budget_is_twelve_gib_and_the_emergency_cap_sixteen(self):
        self.assertEqual(BUDGET, 12 * k6.GIB)
        self.assertEqual(rr.EMERGENCY_RSS_CAP_BYTES, 16 * k6.GIB)

    def test_attribute_marks_every_run_once(self):
        plan = rr.build_plan((('linear-ex', 100, 60, 5, 'solved'), ('linear-ex', 100, 60, 6, 'solved', True),
                              ('nonlinear-pf', 100, 60, 1, 'solved')))
        self.assertEqual([r['run_id'] for r in plan],
                         ['linear-ex-n0100-sparse', 'linear-ex-n0100-sparse-attr', 'nonlinear-pf-n0100-sparse'])
        marked = rr.with_attribution(plan)
        self.assertEqual([r['run_id'] for r in marked], ['linear-ex-n0100-sparse-attr', 'nonlinear-pf-n0100-sparse-attr'])
        self.assertTrue(all(r['attribute'] for r in marked))

    def test_build_plan_totals_and_selection(self):
        plan = synthetic_plan()
        self.assertEqual([r['order'] for r in plan], list(range(len(SYNTHETIC_ROWS))))
        self.assertEqual(plan[1]['run_id'], 'linear-ex-n0200-sparse')
        self.assertEqual(rr.plan_total_s(plan), sum(row[3] for row in SYNTHETIC_ROWS))
        self.assertEqual({r['profile'] for r in rr.select(plan, 'linear')}, {'linear-ex'})
        ids = 'nonlinear-ex-n0100-sparse,linear-ex-n0100-sparse'
        self.assertEqual([r['run_id'] for r in rr.select(plan, ids)],
                         ['linear-ex-n0100-sparse', 'nonlinear-ex-n0100-sparse'], 'plan order kept')
        with self.assertRaises(ValueError):
            rr.select(plan, 'no-such-run')
        self.assertEqual(synthetic_plan('dense')[0]['run_id'], 'linear-ex-n0100-dense')
        text = rr.format_plan(plan, 900)
        self.assertIn('total estimate: %d s' % rr.plan_total_s(plan), text)
        for r in plan:
            self.assertIn(r['run_id'], text)

    def test_plan_prints_the_total_and_starts_nothing(self):
        out = io.StringIO()
        with mock.patch.object(k6, 'launch', side_effect=AssertionError('launched')), \
                mock.patch.object(rr.subprocess, 'run', side_effect=AssertionError('generated')), \
                redirect_stdout(out):
            self.assertEqual(rr.main(['--plan', '--subset', 'window']), 0)
        text = out.getvalue()
        window = rr.select(rr.default_plan(), 'window')
        self.assertIn('total estimate: %d s' % rr.plan_total_s(window), text)
        for r in window:
            self.assertIn(r['run_id'], text)


class Classification(unittest.TestCase):
    def test_ok(self):
        self.assertEqual(rr.classify(launched(), driver(), BUDGET), ('ok', None))

    def test_timed_out(self):
        self.assertEqual(rr.classify(launched('timed_out', -9), None, BUDGET)[0], 'timed_out')

    def test_a_watchdog_kill_exceeds_the_budget_only_on_a_footprint_reading(self):
        # RSS above the cap does not prove the phys_footprint budget exceeded.
        cls, detail = rr.classify(launched('killed_by_rss_watchdog', -9), None, BUDGET)
        self.assertEqual(cls, 'killed_by_rss_cap')
        self.assertIn('footprint_%d_within_budget' % (2 * k6.GIB), detail)
        cls, detail = rr.classify(launched('killed_by_rss_watchdog', -9, rss={}), None, BUDGET)
        self.assertEqual((cls, detail.endswith('footprint_unread')), ('killed_by_rss_cap', True))
        over = launched('killed_by_rss_watchdog', -9, rss={'time_peak_footprint_bytes': BUDGET + 1})
        self.assertEqual(rr.classify(over, None, BUDGET)[0], 'exceeds_base_machine_budget')
        under = launched('killed_by_rss_watchdog', -9, rss={'time_peak_footprint_bytes': BUDGET - 1})
        self.assertEqual(rr.classify(under, None, BUDGET)[0], 'killed_by_rss_cap')
        self.assertEqual(rr.classify(launched('killed_by_rss_watchdog', -9), None, 128 * k6.MIB)[0],
                         'killed_by_rss_cap')

    def test_a_footprint_watchdog_kill_exceeds_the_budget(self):
        record = launched('killed_by_sampler', -9, sampler={'kill_footprint_bytes': BUDGET + 7,
                                                           'peak_phys_footprint_bytes': BUDGET + 7})
        cls, detail = rr.classify(record, None, rr.EMERGENCY_RSS_CAP_BYTES, BUDGET)
        self.assertEqual(cls, 'exceeds_base_machine_budget')
        self.assertIn(str(BUDGET + 7), detail)

    def test_an_emergency_rss_kill_with_and_without_a_footprint_over_budget(self):
        cap = rr.EMERGENCY_RSS_CAP_BYTES
        within = launched('killed_by_rss_watchdog', -9, rss={}, sampler={
            'peak_phys_footprint_bytes': BUDGET - 1, 'peak_lifetime_max_phys_footprint_bytes': BUDGET - 1})
        cls, detail = rr.classify(within, None, cap, BUDGET)
        self.assertEqual(cls, 'killed_by_rss_cap')
        self.assertIn('%d_within_budget' % (BUDGET - 1), detail)
        over = launched('killed_by_rss_watchdog', -9, rss={}, sampler={
            'peak_phys_footprint_bytes': BUDGET - 1, 'peak_lifetime_max_phys_footprint_bytes': BUDGET + 1})
        self.assertEqual(rr.classify(over, None, cap, BUDGET)[0], 'exceeds_base_machine_budget')
        # The driver's last progress line also counts as a footprint reading.
        progress = {'phys_footprint_bytes': BUDGET + 2, 'lifetime_max_phys_footprint_bytes': BUDGET + 2}
        bare = launched('killed_by_rss_watchdog', -9, rss={})
        self.assertEqual(rr.classify(bare, None, cap, BUDGET, progress)[0], 'exceeds_base_machine_budget')
        self.assertEqual(rr.classify(bare, None, cap, BUDGET)[0], 'killed_by_rss_cap')

    def test_a_completed_run_above_the_budget_exceeds_it(self):
        cls, detail = rr.classify(launched(), driver(footprint=BUDGET + 1), BUDGET)
        self.assertEqual(cls, 'exceeds_base_machine_budget')
        self.assertIn('at_result', detail)
        over_time = launched(rss={'time_peak_footprint_bytes': BUDGET + 1})
        self.assertEqual(rr.classify(over_time, driver(), BUDGET)[0], 'exceeds_base_machine_budget')
        sampled = launched(sampler={'peak_lifetime_max_phys_footprint_bytes': BUDGET + 1})
        self.assertEqual(rr.classify(sampled, driver(), rr.EMERGENCY_RSS_CAP_BYTES, BUDGET)[0],
                         'exceeds_base_machine_budget')

    def test_refused_by_name(self):
        codes = ['NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED', 'OTHER']
        cls, detail = rr.classify(launched(), driver('MODEL_INCOMPLETE', codes), BUDGET)
        self.assertEqual((cls, detail), ('refused', 'NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED,OTHER'))
        self.assertTrue(rr.as_expected('refused:NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED', cls, detail))
        self.assertFalse(rr.as_expected('refused:SOMETHING_ELSE', cls, detail))
        self.assertTrue(rr.as_expected('refused', cls, detail))
        self.assertFalse(rr.as_expected('solved', cls, detail))

    def test_plain_classification_expectations_match_exactly(self):
        self.assertTrue(rr.as_expected('exceeds_base_machine_budget', 'exceeds_base_machine_budget', 'x'))
        self.assertFalse(rr.as_expected('exceeds_base_machine_budget', 'ok', None))
        self.assertFalse(rr.as_expected('exceeds_base_machine_budget', 'timed_out', None))
        for name in rr.CLASSES:
            self.assertTrue(rr.as_expected(name, name, None), name)
            self.assertFalse(rr.as_expected(name, 'error' if name != 'error' else 'ok', None), name)
        self.assertTrue(rr.as_expected('solved', 'ok', None))
        self.assertIsNone(rr.as_expected('no-such-expectation', 'ok', None))

    def test_entry_error(self):
        line = driver('entry_error', entry_error={'stage': 'entry', 'message': 'PREVIEW-X: bad'})
        cls, detail = rr.classify(launched('error', 3), line, BUDGET)
        self.assertEqual(cls, 'entry_error')
        self.assertEqual(detail, 'entry: PREVIEW-X: bad')

    def test_errors(self):
        self.assertEqual(rr.classify(launched('error', 101), None, BUDGET), ('error', 'no_driver_line_exit_101'))
        self.assertEqual(rr.classify(launched(), driver('SOMETHING'), BUDGET), ('error', 'status_SOMETHING'))
        self.assertEqual(rr.classify(launched('error', 2), driver('driver_error'), BUDGET)[0], 'error')


class Window(unittest.TestCase):
    """The run loop with a fake launch and clock: order, budget skips and timeout caps."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.models = os.path.join(self.tmp.name, 'models')
        self.out = os.path.join(self.tmp.name, 'out')
        os.makedirs(self.models)
        self.plan = synthetic_plan()
        for r in self.plan:
            with open(os.path.join(self.models, rr.model_name(r['profile'], r['nodes'])), 'w') as fh:
                fh.write('{"nodes": []}')
        self.now = [0.0]
        self.calls = []

    def tearDown(self):
        self.tmp.cleanup()

    def fake_launch(self, argv, *, rss_cap_bytes, timeout_s, record_dir, run_id, wrapper, k6_protocol, sampler):
        self.assertFalse(k6_protocol)
        self.assertIsInstance(sampler, rr.FootprintSampler)
        self.calls.append((run_id, timeout_s, argv))
        run = next(r for r in self.plan if r['run_id'] == run_id)
        self.now[0] += run['estimate_s']
        refused = run['expect'].startswith('refused')
        line = driver('MODEL_INCOMPLETE' if refused else rr.SOLVED,
                      ['NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED'] if refused else [])
        with open(os.path.join(record_dir, run_id + '.jsonl'), 'w') as fh:
            fh.write(json.dumps(line) + '\n')
        return launched()

    def run_plan(self, plan, budget_s):
        return rr.run_plan(plan, self.models, self.out, '/bin/release_timing', budget_s=budget_s,
                           launch=self.fake_launch, quiet=lambda log, wait: {'quiet': True, 'waited_s': 0},
                           clock=lambda: self.now[0], log=lambda *a: None, md={'os': 'test'})

    def test_runs_in_order_one_at_a_time(self):
        records = self.run_plan(self.plan, 900)
        self.assertEqual([c[0] for c in self.calls], [r['run_id'] for r in self.plan])
        self.assertEqual([r['classification'] for r in records],
                         ['ok'] * 5 + ['refused', 'refused'])
        self.assertTrue(all(r['as_expected'] for r in records))
        argv = self.calls[0][2]
        self.assertEqual(argv[2:], ['--mode', 'sparse'], 'no --write unless asked')
        self.assertEqual(len(k6.read_records(self.out)), len(self.plan))

    def test_budget_skips_and_caps(self):
        records = self.run_plan(self.plan, 100)
        # 70 s runs; 260 s is skipped; 4 s and 17 s fit; 80 s does not; 2 s fits; 15 s does not.
        ran = [c[0] for c in self.calls]
        self.assertEqual(ran, ['linear-ex-n0100-sparse', 'nonlinear-pf-n0100-sparse', 'nonlinear-pf-n0200-sparse',
                               'nonlinear-pf-n0400-sparse'])
        skipped = [r['run_id'] for r in records if r['classification'] == 'skipped_window_budget']
        self.assertEqual(skipped, ['linear-ex-n0200-sparse', 'nonlinear-pf-n0300-sparse', 'nonlinear-ex-n0100-sparse'])
        self.assertTrue(all(r.get('timeout_applied_s', 0) <= 100 for r in records))
        self.assertEqual(self.calls[0][1], 100, 'the first timeout is capped at the window budget')
        self.assertTrue(records[0]['timeout_capped_by_window'])

    def test_only_the_first_launched_run_waits_for_a_quiet_host(self):
        waits = []

        def quiet(log, wait):
            waits.append(wait)
            return {'quiet': False, 'reason': 'busy'}
        rr.run_plan(rr.select(self.plan, 'nonlinear-pf'), self.models, self.out, '/bin/release_timing',
                    budget_s=900, quiet_wait_s=60, launch=self.fake_launch, quiet=quiet,
                    clock=lambda: self.now[0], log=lambda *a: None, md={})
        self.assertEqual(waits, [60, 0, 0, 0])
        self.assertEqual(len(self.calls), 4, 'a busy host is recorded, not a reason to skip')

    def test_the_host_check_cannot_overrun_the_window(self):
        waits = []

        def slow_quiet(log, wait):
            waits.append(wait)
            self.now[0] += wait + 25  # the check itself takes time beyond its allowance
            return {'quiet': False, 'reason': 'busy'}
        records = rr.run_plan(rr.select(self.plan, 'nonlinear-pf')[:1], self.models, self.out, '/bin/release_timing',
                              budget_s=30, quiet_wait_s=60, launch=self.fake_launch, quiet=slow_quiet,
                              clock=lambda: self.now[0], log=lambda *a: None, md={})
        self.assertEqual(self.calls, [], 'a run whose estimate no longer fits after the host check is not launched')
        self.assertEqual(records[0]['classification'], 'skipped_window_budget')

    def test_quiet_waits_in_whole_steps_within_the_allowance(self):
        seen = []
        original = rr.k6.wait_for_quiet_host
        rr.k6.wait_for_quiet_host = lambda log, max_wait_s: seen.append(max_wait_s) or {'waited_s': 0}
        try:
            for allowance in (19, 30, 59, 60, 61):
                rr.quiet_host(lambda *a: None, allowance)
        finally:
            rr.k6.wait_for_quiet_host = original
        self.assertEqual(seen, [0, 30, 30, 60, 60])

    def test_missing_and_mismatched_models_are_not_run(self):
        os.unlink(os.path.join(self.models, rr.model_name('linear-ex', 100)))
        with open(os.path.join(self.models, 'manifest.json'), 'w') as fh:
            json.dump({'models': {rr.model_name('linear-ex', 200): {'sha256': '0' * 64}}}, fh)
        records = self.run_plan(rr.select(self.plan, 'linear'), 900)
        self.assertEqual([r['classification'] for r in records], ['model_missing', 'model_mismatch'])
        self.assertEqual(self.calls, [])

    def test_refuses_an_output_folder_with_records(self):
        self.run_plan(rr.select(self.plan, 'linear'), 900)
        with self.assertRaises(RuntimeError):
            self.run_plan(rr.select(self.plan, 'linear'), 900)

    def test_write_results_passes_write(self):
        rr.run_plan(rr.select(self.plan, 'linear'), self.models, self.out, '/bin/release_timing',
                    write_results=True, launch=self.fake_launch, quiet=lambda log, wait: {'quiet': True},
                    clock=lambda: self.now[0], log=lambda *a: None, md={})
        argv = self.calls[0][2]
        self.assertEqual(argv[4], '--write')
        self.assertTrue(argv[5].endswith(os.path.join('results', 'linear-ex-n0100-sparse.result.json')))


class PacketAggregation(unittest.TestCase):
    def test_packet_from_synthetic_records(self):
        with tempfile.TemporaryDirectory() as out:
            with open(os.path.join(out, 'metadata.json'), 'w') as fh:
                json.dump({'os': 'Darwin', 'budget_s': 900}, fh)
            base = {'schema': rr.RECORD_SCHEMA, 'profile': 'linear-ex', 'nodes': 2000, 'mode': 'sparse',
                    'rss_cap_bytes': rr.EMERGENCY_RSS_CAP_BYTES, 'footprint_budget_bytes': BUDGET,
                    'expect': 'solved', 'model': 'linear-ex-n2000.json',
                    'footprint_watchdog': {'peak_phys_footprint_bytes': k6.GIB,
                                           'peak_lifetime_max_phys_footprint_bytes': 2 * k6.GIB},
                    'peak_footprint_bytes': 2 * k6.GIB,
                    'last_stage': {'repeat': 0, 'stage': 'parse_back', 'seconds': 1.1},
                    'timeout_applied_s': 600, 'host': {'quiet': True, 'memorystatus_level': 90,
                                                       'load1_at_start': 1.25, 'waited_s': 0}}
            k6.append_record(out, dict(base, order=0, run_id='a', classification='ok', as_expected=True,
                                       launch=launched(), driver=driver(), window_elapsed_after_s=10.0))
            snapshot = {'t_s': 2.0, 'phys_footprint_bytes': k6.GIB, 'samples': 900,
                        'paths': [{'count': 800, 'frames': ['open_pipe_stress_x::f'], 'leaf': 'memmove'}]}
            k6.append_record(out, dict(base, order=1, run_id='b', classification='refused', attribute=True,
                                       timing_use='not a measurement', attribution={'snapshots': [snapshot]},
                                       classification_detail='X', launch=launched(),
                                       driver=driver('MODEL_INCOMPLETE', ['X']), window_elapsed_after_s=20.0))
            k6.append_record(out, dict(base, order=2, run_id='c', classification='skipped_window_budget',
                                       classification_detail='estimate', window_elapsed_after_s=20.0))
            k6.append_record(out, {'schema': 'something-else', 'run_id': 'z', 'classification': 'ok'})
            result = rr.write_packet(out)
            with open(os.path.join(out, 'packet.json')) as fh:
                self.assertEqual(json.load(fh), result)
        self.assertEqual(result['schema'], rr.PACKET_SCHEMA)
        self.assertTrue(result['observation_only'])
        self.assertEqual(result['classification_counts'], {'ok': 1, 'refused': 1, 'skipped_window_budget': 1})
        self.assertEqual([r['run_id'] for r in result['runs']], ['a', 'b'])
        self.assertEqual(result['not_run'][0]['run_id'], 'c')
        self.assertEqual(result['window_elapsed_s'], 20.0)
        a = result['runs'][0]
        self.assertEqual(a['stage_seconds']['entry'], 5.0)
        self.assertEqual(a['peak_memory_bytes'], {
            'time_max_rss': 3 * k6.GIB, 'time_peak_footprint': 2 * k6.GIB, 'ps_max_rss': 3 * k6.GIB,
            'driver_ru_maxrss': 3 * k6.GIB + 5, 'driver_lifetime_max_phys_footprint': 2 * k6.GIB,
            'sampled_phys_footprint': k6.GIB, 'sampled_lifetime_max_phys_footprint': 2 * k6.GIB,
            'peak_footprint': 2 * k6.GIB})
        self.assertEqual(a['last_stage'], {'repeat': 0, 'stage': 'parse_back', 'seconds': 1.1})
        self.assertFalse(a['attribution'])
        self.assertEqual(a['timing_use'], 'measurement')
        attributed = result['runs'][1]
        self.assertTrue(attributed['attribution'])
        self.assertNotEqual(attributed['timing_use'], 'measurement')
        self.assertEqual(attributed['snapshots'][0]['paths'][0]['frames'], ['open_pipe_stress_x::f'])
        self.assertEqual((a['rows'], a['output_bytes'], a['bases']), (1000, 123456, 2))
        self.assertIn('2000 nodes', a['conditions'])
        self.assertIn('footprint budget 12.00 GiB', a['conditions'])
        self.assertIn('emergency RSS cap 16.00 GiB', a['conditions'])
        self.assertEqual(result['runs'][1]['blocking_codes'], ['X'])
        self.assertEqual(result['driver_limits'], ['a limit'])
        for row in result['runs']:
            self.assertFalse({'pass', 'passed', 'threshold', 'within_bound'} & set(row), 'observations only')


class MetadataHasNoHostIdentifiers(unittest.TestCase):
    def test_metadata(self):
        def fake_run(argv, env=None):
            values = {'sw_vers': '26.6.2', 'sysctl': 'Apple M-series', 'rustc': 'rustc 1.97.1\nhost: aarch64-apple-darwin',
                      'cargo': 'cargo 1.97.1'}
            return types.SimpleNamespace(returncode=0, stdout=values.get(argv[0], '0'))
        with tempfile.NamedTemporaryFile(delete=False) as fh:
            fh.write(b'driver bytes')
            binary = fh.name
        try:
            md = rr.metadata(binary, 'a' * 40, 'b' * 40, False, run=fake_run, system='Darwin')
        finally:
            os.unlink(binary)
        text = json.dumps(md)
        for identifier in {socket.gethostname(), platform.node(), getpass.getuser()} - {''}:
            self.assertNotIn(identifier, text)
        for key in md:
            self.assertNotIn(key, ('hostname', 'node', 'user', 'serial', 'path'))
        for value in text.split('"'):
            self.assertFalse(value.startswith('/'), value)
        self.assertEqual(md['binary_sha256'], hashlib.sha256(b'driver bytes').hexdigest())
        self.assertEqual(md['base_machine_budget_bytes'], BUDGET)
        self.assertFalse(md['source_dirty'])


class Generate(unittest.TestCase):
    FAKE = ('import argparse, json, sys\n'
            'p = argparse.ArgumentParser(); p.add_argument("--profile"); p.add_argument("--nodes", type=int)\n'
            'p.add_argument("-o"); a = p.parse_args()\n'
            'open(a.o, "w").write(json.dumps({"nodes": list(range(a.nodes))}))\n'
            'print("progress", file=sys.stderr)\n'
            'print(json.dumps({"profile": a.profile, "nodes": a.nodes}), file=sys.stderr)\n')

    def test_manifest(self):
        with tempfile.TemporaryDirectory() as tmp:
            generator = os.path.join(tmp, 'fake_rack.py')
            with open(generator, 'w') as fh:
                fh.write(self.FAKE)
            models = os.path.join(tmp, 'models')
            plan = synthetic_plan() + synthetic_plan('dense')
            manifest, failures = rr.generate(plan, models, generator=generator, log=lambda *a: None)
            self.assertEqual(failures, [])
            self.assertEqual(len(manifest['models']), len(SYNTHETIC_ROWS), 'one model per (profile, nodes)')
            entry = manifest['models']['nonlinear-pf-n0100.json']
            path = os.path.join(models, 'nonlinear-pf-n0100.json')
            with open(path, 'rb') as fh:
                data = fh.read()
            self.assertEqual(entry['sha256'], hashlib.sha256(data).hexdigest())
            self.assertEqual(entry['bytes'], len(data))
            self.assertEqual(entry['stats'], {'profile': 'nonlinear-pf', 'nodes': 100})
            with open(os.path.join(models, 'manifest.json')) as fh:
                self.assertEqual(json.load(fh)['models'], manifest['models'])
            self.assertNotIn(tmp, json.dumps(manifest))


STAND_IN_DRIVER = ('import json, sys\n'
                   'print(json.dumps({"schema": "release-timing-v1", "status": "MECHANICS_SOLVED", '
                   '"blocking_codes": [], "rows": 3, "argv_tail": sys.argv[2:], '
                   '"worker_peak_footprint_bytes_equivalent": 1}))\n')


def grower(progress=False):
    """K6's self-limiting growing child (1 MiB per 10 ms, failsafe exit 3 at 512 MiB); with
    `progress`, it first writes two driver progress lines to stderr, as the driver does."""
    lines = ''
    if progress:
        lines = ''.join('print(%r, file=sys.stderr, flush=True)\n' % (rr.PROGRESS_PREFIX + json.dumps(
            {'repeat': 0, 'stage': stage, 'seconds': 0.01, 'ru_maxrss_bytes': 1, 'phys_footprint_bytes': 2,
             'lifetime_max_phys_footprint_bytes': 3})) for stage in ('read', 'parse'))
    body = k6.GROWING_CHILD.replace('int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), float(sys.argv[4])',
                                    '%d, 0, %d, 0.01' % (k6.MIB, 512 * k6.MIB))
    return body.replace('import sys, time\n', 'import sys, time\n' + lines, 1)


def executable(folder, name, body):
    path = os.path.join(folder, name)
    with open(path, 'w') as fh:
        fh.write('#!%s\n%s' % (sys.executable, body))
    os.chmod(path, os.stat(path).st_mode | stat.S_IXUSR)
    return path


class LiveLaunch(unittest.TestCase):
    """The runner's run path with real child processes under k6_runner.launch."""

    def plan(self, rss_cap, budget=rr.BASE_MACHINE_BUDGET_BYTES, attribute=False):
        return rr.build_plan((('nonlinear-pf', 100, 60, 1, 'solved', attribute),), rss_cap_bytes=rss_cap,
                             footprint_budget_bytes=budget)

    def models(self, folder):
        models = os.path.join(folder, 'models')
        os.makedirs(models)
        with open(os.path.join(models, rr.model_name('nonlinear-pf', 100)), 'w') as fh:
            fh.write('{}')
        return models

    def test_a_stand_in_driver_is_ok(self):
        with tempfile.TemporaryDirectory() as folder:
            binary = executable(folder, 'release_timing', STAND_IN_DRIVER)
            records = rr.run_plan(self.plan(128 * k6.MIB), self.models(folder), os.path.join(folder, 'out'), binary,
                                  quiet=lambda log, wait: {'quiet': True}, log=lambda *a: None, md={})
        r = records[0]
        self.assertEqual(r['classification'], 'ok', r)
        self.assertNotIn('attribution', r)
        self.assertEqual(r['driver']['argv_tail'], ['--mode', 'sparse'])
        self.assertEqual(r['launch']['argv'][0], 'release_timing', 'record argv holds no path')
        self.assertTrue(r['as_expected'])

    def test_a_growing_child_is_killed_at_the_cap(self):
        # macOS: the ps RSS watchdog kills it. Linux: RLIMIT_AS stops its growth (MemoryError).
        # Any other platform has no live limit path and fails, as in test_k6_runner.
        self.assertIn(platform.system(), ('Darwin', 'Linux'))
        with tempfile.TemporaryDirectory() as folder:
            binary = executable(folder, 'release_timing', grower())
            # The footprint budget sits above the failsafe, so only the emergency RSS cap can act.
            records = rr.run_plan(self.plan(128 * k6.MIB, budget=k6.GIB), self.models(folder),
                                  os.path.join(folder, 'out'), binary,
                                  quiet=lambda log, wait: {'quiet': True}, log=lambda *a: None, md={})
        r = records[0]
        self.assertNotEqual(r['launch']['exit_code'], 3, 'the child reached its failsafe: the watchdog did not kill')
        self.assertEqual(r['classification'], 'killed_by_rss_cap')
        self.assertEqual(r['launch']['survivors'], [])
        if platform.system() == 'Darwin':
            self.assertTrue(r['launch']['killed_by_rss_watchdog'])
            self.assertGreaterEqual(r['launch']['watchdog_last_kib'] * k6.KIB, 128 * k6.MIB)
        else:
            self.assertEqual(r['launch']['classification'], 'rlimit_abort')


# A `sample` report in the macOS format (constructed; addresses and counts invented): one thread,
# a product path that copies (memmove under the envelope build), one that serializes, and the
# watchdog-free remainder outside product code.
CANNED_SAMPLE = """Analysis of sampling release_timing (pid 4242) every 1 millisecond
Process:         release_timing [4242]
Path:            /Users/someone/targets/release/release_timing

Call graph:
    1000 Thread_1   DispatchQueue_1: com.apple.main-thread  (serial)
      1000 start  (in dyld) + 6992  [0x18cea84e4]
        1000 main  (in release_timing) + 36  [0x100bd16b4]
          1000 release_timing::main::h0123456789abcdef  (in release_timing) + 468  [0x100bd1894]
            700 open_pipe_stress_product_physics::run_linear_static_preview_value_with_mode::h1111111111111111  (in release_timing) + 96  [0x1]
            + 600 open_pipe_stress_product_physics::aggregate::combine_rows::h2222222222222222  (in release_timing) + 12  [0x2]
            + ! 450 <alloc::vec::Vec<T> as core::clone::Clone>::clone::h3333333333333333  (in release_timing) + 40  [0x3]
            + ! : 450 _platform_memmove  (in libsystem_platform.dylib) + 52  [0x4]
            + ! 150 _ZN32open_pipe_stress_product_physics9aggregate8row_copy17h4444444444444444E  (in release_timing) + 8  [0x5]
            + 100 open_pipe_stress_product_physics::envelope::build::h5555555555555555  (in release_timing) + 4  [0x6]
            250 serde_json::ser::to_string::h6666666666666666  (in release_timing) + 20  [0x7]
            + 250 <open_pipe_stress_product_physics::ResultItem as serde::ser::Serialize>::serialize::h7777777777777777  (in release_timing) + 8  [0x8]
            50 __rust_dealloc  (in release_timing) + 4  [0x9]

Total number in stack (recursive counted multiple, when >=5):
"""


class Attribution(unittest.TestCase):
    def test_demangle(self):
        self.assertEqual(rr.demangle('open_pipe_stress_a::b::h0123456789abcdef'), 'open_pipe_stress_a::b')
        self.assertEqual(rr.demangle('_ZN32open_pipe_stress_product_physics9aggregate8row_copy17h4444444444444444E'),
                         'open_pipe_stress_product_physics::aggregate::row_copy')
        self.assertEqual(rr.demangle('_platform_memmove'), '_platform_memmove')

    def test_parse_a_canned_sample(self):
        total, paths = rr.parse_sample(CANNED_SAMPLE)
        self.assertEqual(total, 1000)
        self.assertEqual(sum(p['count'] for p in paths), 1000, 'every sample is in one reduced path')
        entry = 'open_pipe_stress_product_physics::run_linear_static_preview_value_with_mode'
        combine = 'open_pipe_stress_product_physics::aggregate::combine_rows'
        self.assertEqual(paths[0], {'count': 450, 'frames': [entry, combine], 'truncated': False,
                                    'leaf': '_platform_memmove', 'leaf_count': 450})
        self.assertEqual(paths[1]['count'], 250)
        self.assertEqual(paths[1]['frames'], ['<open_pipe_stress_product_physics::ResultItem as serde::ser::Serialize>'
                                              '::serialize'])
        self.assertEqual([p['count'] for p in paths[2:]], [150, 100, 50])
        self.assertEqual(paths[2]['frames'][-1], 'open_pipe_stress_product_physics::aggregate::row_copy')
        self.assertEqual(paths[4]['frames'], ['(no product frame)'])
        self.assertEqual(paths[4]['leaf'], '__rust_dealloc')
        # Compactness: the top paths only, and at most `frames` product frames, the innermost kept.
        total, top = rr.parse_sample(CANNED_SAMPLE, top=2, frames=1)
        self.assertEqual([p['count'] for p in top], [450, 250])
        self.assertEqual(top[0]['frames'], [combine])
        self.assertTrue(top[0]['truncated'])

    def test_no_call_graph(self):
        self.assertEqual(rr.parse_sample('sample cannot examine process 1'), (0, []))

    def test_snapshot_triggers(self):
        taken = []
        attributor = rr.Attributor('/nonexistent', 'r', interval_s=2.0, growth_bytes=512 * k6.MIB)
        attributor._take = lambda pid, snapshot: taken.append(snapshot)

        class Done:
            def start(self):
                pass

            def is_alive(self):
                return False
        started = []
        with mock.patch.object(rr.threading, 'Thread', lambda target, args, daemon: started.append(args) or Done()):
            for now, footprint in ((0.1, 100), (1.0, 100), (2.0, 200), (2.5, 200 + 513 * k6.MIB),
                                   (3.0, 200 + 600 * k6.MIB), (4.6, 200 + 600 * k6.MIB)):
                attributor.poll(1, now, footprint, footprint)
        self.assertEqual([(s['t_s'], s['trigger']) for s in attributor.snapshots],
                         [(2.0, 'interval'), (2.5, 'growth'), (4.6, 'interval')])
        self.assertEqual(len(started), 3)


class ProgressLines(unittest.TestCase):
    def test_last_stage_from_a_killed_runs_stderr(self):
        line = lambda stage, f: rr.PROGRESS_PREFIX + json.dumps(  # noqa: E731
            {'repeat': 0, 'stage': stage, 'seconds': 0.5, 'ru_maxrss_bytes': f + 9, 'phys_footprint_bytes': f,
             'lifetime_max_phys_footprint_bytes': f + 1})
        # A kill can cut the last line short; other stderr text is ignored.
        text = '\n'.join([line('read', 10), 'warning: something', line('parse', 20), line('request', 30),
                          line('entry', 40)[:25]])
        with tempfile.TemporaryDirectory() as folder:
            with open(os.path.join(folder, 'r.stderr.txt'), 'w') as fh:
                fh.write(text)
            stage, memory, finished = rr.last_progress(folder, 'r')
            self.assertEqual(rr.last_progress(folder, 'absent'), (None, None, 0))
        self.assertEqual(stage, {'repeat': 0, 'stage': 'request', 'seconds': 0.5})
        self.assertEqual(memory, {'ru_maxrss_bytes': 39, 'phys_footprint_bytes': 30,
                                  'lifetime_max_phys_footprint_bytes': 31})
        self.assertEqual(finished, 3)


@unittest.skipUnless(platform.system() == 'Darwin',
                     'phys_footprint is the macOS ledger the product budgets; elsewhere there is nothing to read')
class LiveFootprint(unittest.TestCase):
    """The footprint watchdog against live children (macOS)."""

    def test_reads_a_live_child(self):
        child = subprocess.Popen([sys.executable, '-c', 'b = bytearray(32 << 20)\nfor i in range(0, len(b), 4096): '
                                  'b[i] = 1\nimport time; time.sleep(30)'], stdout=subprocess.DEVNULL)
        try:
            reading = None
            for _ in range(100):
                reading = rr.read_footprint(child.pid)
                if reading and reading[0] > 32 << 20:
                    break
                time.sleep(0.05)
        finally:
            child.kill()
            child.wait()
        footprint, lifetime = reading
        self.assertGreater(footprint, 0)
        self.assertGreaterEqual(lifetime, footprint)
        self.assertGreater(footprint, 32 << 20, 'the touched 32 MiB is in the footprint')
        self.assertIsNone(rr.read_footprint(child.pid), 'a reaped process has no reading')

    def test_a_growing_child_is_killed_by_the_footprint_watchdog(self):
        with tempfile.TemporaryDirectory() as folder:
            binary = executable(folder, 'release_timing', grower(progress=True))
            plan = LiveLaunch.plan(None, rss_cap=k6.GIB, budget=128 * k6.MIB)
            records = rr.run_plan(plan, LiveLaunch.models(None, folder), os.path.join(folder, 'out'), binary,
                                  quiet=lambda log, wait: {'quiet': True}, log=lambda *a: None, md={})
        r = records[0]
        self.assertNotEqual(r['launch']['exit_code'], 3, 'the child reached its failsafe: the watchdog did not kill')
        self.assertTrue(r['launch']['killed_by_sampler'])
        self.assertFalse(r['launch']['killed_by_rss_watchdog'])
        self.assertEqual(r['classification'], 'exceeds_base_machine_budget')
        self.assertEqual(r['launch']['survivors'], [])
        watchdog = r['footprint_watchdog']
        self.assertGreater(watchdog['kill_footprint_bytes'], 128 * k6.MIB)
        self.assertLess(watchdog['kill_footprint_bytes'], 512 * k6.MIB)
        self.assertGreaterEqual(watchdog['peak_lifetime_max_phys_footprint_bytes'], watchdog['peak_phys_footprint_bytes'])
        self.assertEqual(r['last_stage']['stage'], 'parse')
        self.assertEqual(r['stages_finished'], 2)
        self.assertEqual(r['last_stage_memory']['phys_footprint_bytes'], 2)

    def test_an_attribution_run_takes_snapshots(self):
        # A busy child for about 3.5 s; the snapshots reduce its stacks to frames matching the pattern.
        busy = ('import json, time\n'
                'def spin(until):\n'
                '    x = 0\n'
                '    while time.monotonic() < until:\n'
                '        x += 1\n'
                'spin(time.monotonic() + 3.5)\n'
                'print(json.dumps({"schema": "release-timing-v1", "status": "MECHANICS_SOLVED", '
                '"blocking_codes": [], "worker_peak_footprint_bytes_equivalent": 1}))\n')
        with tempfile.TemporaryDirectory() as folder:
            binary = executable(folder, 'release_timing', busy)
            factory = lambda out, run_id, keep: rr.Attributor(out, run_id, keep, frame_pattern='PyEval')  # noqa: E731
            records = rr.run_plan(LiveLaunch.plan(None, rss_cap=k6.GIB, attribute=True),
                                  LiveLaunch.models(None, folder), os.path.join(folder, 'out'), binary,
                                  quiet=lambda log, wait: {'quiet': True}, log=lambda *a: None, md={},
                                  attributor_factory=factory, keep_samples=True)
            r = records[0]
            raws = [s.get('raw') for s in r['attribution']['snapshots'] if s.get('raw')]
            texts = []
            for name in raws:
                with open(os.path.join(folder, 'out', name)) as fh:
                    texts.append(fh.read())
        self.assertEqual(r['classification'], 'ok', r)
        self.assertTrue(r['run_id'].endswith('-attr'))
        self.assertIn('not a measurement', r['timing_use'])
        snapshots = r['attribution']['snapshots']
        self.assertGreaterEqual(len(snapshots), 1)
        first = snapshots[0]
        self.assertEqual(first['trigger'], 'interval')
        self.assertGreater(first['phys_footprint_bytes'], 0)
        self.assertGreater(first['samples'], 0, first)
        self.assertLessEqual(len(first['paths']), rr.SNAPSHOT_TOP_PATHS)
        self.assertTrue(any('PyEval' in frame for path in first['paths'] for frame in path['frames']), first)
        self.assertTrue(texts, 'raw reports kept on request')
        for text in texts:
            self.assertNotIn(folder, text)
            self.assertNotIn('/Users/', text)


if __name__ == '__main__':
    unittest.main()
