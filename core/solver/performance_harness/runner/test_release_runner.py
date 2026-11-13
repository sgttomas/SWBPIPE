#!/usr/bin/env python3
"""Release timing runner tests (DEL-04-05, RT-U4): standard library `unittest`.

The pure tests need no child process and no Rust binary. `LiveLaunch` runs two tiny
self-limiting Python children through the runner's own run path: a stand-in driver that prints
one driver line (`rr.DRIVER_SCHEMA`), and (macOS) a growing child under a 128 MiB cap with a failsafe
exit at 512 MiB. Nothing measures or asserts a time or memory bound.

Run from this directory: python3 -m unittest test_release_runner
"""
import copy
import getpass
import hashlib
import io
import json
import os
import platform
import re
import shutil
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
            'driver_peak_footprint_bytes': footprint, 'limits': ['a limit']}
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
        self.assertEqual(len({(r['task'], r['profile'], r['nodes']) for r in timed}), len(timed),
                         'one run per (task, profile, nodes)')
        for r in plan:
            if r['attribute']:
                self.assertTrue(r['run_id'].endswith('-attr'))
                self.assertIn((r['task'], r['profile'], r['nodes'], r['mode']),
                              {(t['task'], t['profile'], t['nodes'], t['mode']) for t in timed},
                              'an attribution run has a twin')
        for r in plan:
            self.assertIn(r['task'], rr.TASKS)
            self.assertIn(r['profile'], rr.PROFILES)
            self.assertEqual(r['footprint_budget_bytes'], BUDGET)
            self.assertEqual(r['rss_cap_bytes'], rr.EMERGENCY_RSS_CAP_BYTES)
            self.assertEqual(r['mode'], 'sparse')
            self.assertEqual(r['run_id'], rr.run_id_of(r['profile'], r['nodes'], r['mode'], r['attribute'], r['task']))
            self.assertIsNotNone(rr.as_expected(r['expect'], 'ok', None), 'expectation %r unknown' % r['expect'])
        # The model checks are runs of their own, never inside a solve run or a timed window.
        checks = rr.select(plan, 'model-checks')
        self.assertTrue(checks)
        for r in checks:
            self.assertEqual(r['task'], 'model_checks')
            self.assertTrue(r['run_id'].startswith('model-checks-') and r['run_id'].endswith('-n%04d' % r['nodes']))
            self.assertEqual(r['expect'], 'ok')
            self.assertIn(rr.model_name(r['profile'], r['nodes']),
                          {rr.model_name(s['profile'], s['nodes']) for s in plan if s['task'] == 'solve'},
                          'a model-checks run reads a rack a solve run reads')
        self.assertIn('model-checks-linear-ex-n2000', {r['run_id'] for r in rr.select(plan, 'criterion')})
        for name in ('window', 'window2', 'scaling', 'scaling-2000'):
            window = rr.select(plan, name)
            self.assertTrue(window)
            self.assertTrue(all(r['task'] == 'solve' for r in window), name)
            if name != 'scaling-2000':
                self.assertLessEqual(rr.plan_total_s(window) + rr.QUIET_WAIT_S, rr.WINDOW_BUDGET_S, name)
        self.assertFalse(any(r['attribute'] for r in rr.select(plan, 'window')), 'the timed window samples nothing')
        self.assertEqual(sorted(r['attribute'] for r in rr.select(plan, 'window2')), [False, True])
        self.assertEqual([(r['profile'], r['nodes']) for r in rr.select(plan, 'scaling')],
                         [(p, n) for p in ('linear-ex', 'nonlinear-ex') for n in (250, 500, 1000)])
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

    def test_a_model_checks_run_is_its_own_driver_invocation(self):
        plan = rr.build_plan((('linear-ex', 100, 60, 5, 'solved'), ('linear-ex', 100, 60, 1, 'ok', False, 'model_checks')))
        solve, checks = plan
        self.assertEqual((solve['task'], solve['run_id']), ('solve', 'linear-ex-n0100-sparse'))
        self.assertEqual((checks['task'], checks['run_id']), ('model_checks', 'model-checks-linear-ex-n0100'))
        self.assertEqual(rr.driver_argv('/b/release_timing', '/m/x.json', solve), ['/b/release_timing', '/m/x.json',
                                                                                  '--mode', 'sparse'])
        self.assertEqual(rr.driver_argv('/b/release_timing', '/m/x.json', checks, '/r'),
                         ['/b/release_timing', '/m/x.json', '--model-checks', '--write',
                          os.path.join('/r', 'model-checks-linear-ex-n0100.result.json')])
        with self.assertRaises(ValueError):
            rr.build_plan((('linear-ex', 100, 60, 1, 'ok', False, 'other'),))

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


class WritePlan(unittest.TestCase):
    """`--write-plan`: the plan file of the driver's supervise mode has the runner's plan as its one source."""

    def test_the_plan_file_is_the_selected_plan(self):
        with tempfile.TemporaryDirectory() as folder:
            models = os.path.join(folder, 'models')
            os.makedirs(models)
            listed = rr.model_name('linear-ex', 250)
            with open(os.path.join(models, 'manifest.json'), 'w') as fh:
                json.dump({'models': {listed: {'sha256': 'a' * 64, 'stats': {'nodes': 250}}}}, fh)
            path = os.path.join(folder, 'plan.json')
            out = io.StringIO()
            with mock.patch.object(k6, 'launch', side_effect=AssertionError('launched')), redirect_stdout(out):
                self.assertEqual(rr.main(['--write-plan', path, '--subset', 'window', '--models', models,
                                          '--budget-s', '600', '--quiet-wait-s', '30']), 0)
            with open(path) as fh:
                document = json.load(fh)
        window = rr.select(rr.default_plan(), 'window')
        self.assertEqual(document['schema'], rr.PLAN_SCHEMA)
        self.assertEqual((document['window_budget_s'], document['quiet_wait_s']), (600, 30))
        self.assertEqual([{k: r[k] for k in rr.PLAN_RUN_KEYS} for r in document['runs']],
                         [{k: r[k] for k in rr.PLAN_RUN_KEYS} for r in window])
        self.assertEqual([r['model'] for r in document['runs']], [rr.model_name(r['profile'], r['nodes']) for r in window])
        by_model = {r['model']: r for r in document['runs']}
        self.assertEqual((by_model[listed]['expected_model_sha256'], by_model[listed]['model_stats']),
                         ('a' * 64, {'nodes': 250}))
        others = [r for r in document['runs'] if r['model'] != listed]
        self.assertTrue(others and all(r['expected_model_sha256'] is None for r in others), 'unlisted: no check')
        self.assertNotIn(folder, json.dumps(document))

    def test_attribution_runs_are_refused(self):
        with self.assertRaises(ValueError):
            rr.plan_document(rr.select(rr.default_plan(), 'window2'))
        with tempfile.TemporaryDirectory() as folder, redirect_stdout(io.StringIO()), \
                mock.patch('sys.stderr', io.StringIO()), self.assertRaises(SystemExit):
            rr.main(['--write-plan', os.path.join(folder, 'plan.json'), '--subset', 'window2'])


class BaselinePlans(unittest.TestCase):
    """The headless baseline's named plans: `baseline` (W1, W2 alternating, five each) and
    `baseline-with-rack` (the same ten, then criterion 9's linear rack at 2,000 nodes once)."""

    W1 = ('baseline-one-case-ex', 2000, 300, 'solved')
    W2 = ('reference-system', 27, 60, 'solved')

    def test_baseline_is_ten_runs_alternating_with_the_contract_limits(self):
        plan, budget_s, quiet_wait_s = rr.named_plan('baseline')
        self.assertEqual(len(plan), 10)
        self.assertEqual([(r['profile'], r['nodes'], r['timeout_s'], r['expect']) for r in plan], [self.W1, self.W2] * 5)
        self.assertEqual([r['order'] for r in plan], list(range(10)))
        for r in plan:
            self.assertEqual((r['task'], r['mode'], r['attribute']), ('solve', 'sparse', False))
            self.assertEqual(r['footprint_budget_bytes'], 12_884_901_888)
            self.assertEqual(r['rss_cap_bytes'], 16 * k6.GIB)
        self.assertEqual([r['run_id'] for r in plan],
                         ['%s-r%d' % (rid, k) for k in range(1, 6)
                          for rid in ('baseline-one-case-ex-n2000-sparse', 'reference-system-n0027-sparse')])
        rr.check_run_ids(plan)
        self.assertEqual(len({r['run_id'].lower() for r in plan}), 10)
        # The window lets every run last until its timeout, after the quiet wait, unskipped and uncapped.
        self.assertEqual(quiet_wait_s, rr.QUIET_WAIT_S)
        self.assertGreaterEqual(budget_s, quiet_wait_s + sum(r['timeout_s'] for r in plan) + len(plan))
        self.assertEqual(rr.RUN_ALLOWANCE_S, 30)
        self.assertEqual(budget_s, 60 + 5 * 300 + 5 * 60 + 10 * 30)  # 2,160 s

    def test_baseline_with_rack_is_the_baseline_then_the_default_linear_rack_row_last(self):
        baseline, _, _ = rr.named_plan('baseline')
        plan, budget_s, quiet_wait_s = rr.named_plan('baseline-with-rack')
        self.assertEqual(len(plan), 11)
        self.assertEqual(plan[:10], baseline)
        default = [r for r in rr.default_plan() if r['run_id'] == 'linear-ex-n2000-sparse']
        self.assertEqual(len(default), 1)
        self.assertEqual(dict(plan[10], order=None), dict(default[0], order=None), 'the default row but for its place')
        self.assertEqual((plan[10]['order'], plan[10]['expect'], plan[10]['timeout_s']),
                         (10, 'exceeds_base_machine_budget', 900))
        rr.check_run_ids(plan)
        self.assertGreaterEqual(budget_s, quiet_wait_s + sum(r['timeout_s'] for r in plan) + len(plan))
        self.assertEqual(budget_s, 60 + 5 * 300 + 5 * 60 + 900 + 11 * 30)  # 3,090 s

    def test_a_named_plan_fixes_its_mode_and_limits(self):
        for flags in (['--mode', 'dense'], ['--budget-s', '900'], ['--footprint-budget-bytes', '1'],
                      ['--rss-cap-bytes', '1'], ['--quiet-wait-s', '0'], ['--attribute']):
            with redirect_stdout(io.StringIO()), mock.patch('sys.stderr', io.StringIO()) as err, \
                    self.assertRaises(SystemExit):
                rr.main(['--plan', '--subset', 'baseline'] + flags)
            self.assertIn('fixes its runs, mode and limits', err.getvalue(), flags)
        out = io.StringIO()
        with redirect_stdout(out):
            self.assertEqual(rr.main(['--plan', '--subset', 'baseline', '--mode', 'sparse', '--budget-s', '2160']), 0)
        self.assertIn('window budget 2160 s', out.getvalue())

    def test_a_repeated_run_gets_distinct_ids_and_a_single_one_keeps_its_own(self):
        plan = rr.repeat_run_ids(rr.build_plan((('linear-ex', 100, 60, 1, 'solved'), ('linear-ex', 200, 60, 1, 'solved'),
                                                ('linear-ex', 100, 60, 1, 'solved'))))
        self.assertEqual([r['run_id'] for r in plan],
                         ['linear-ex-n0100-sparse-r1', 'linear-ex-n0200-sparse', 'linear-ex-n0100-sparse-r2'])

    def test_the_baseline_plan_file_names_each_models_sha256_and_needs_the_manifest(self):
        plan, _, _ = rr.named_plan('baseline')
        hashes = rr.recorded_rack_hashes()
        with open(rr.COMMITTED_MODELS['reference-system']['source'], 'rb') as fh:
            reference = hashlib.sha256(fh.read()).hexdigest()
        w1 = hashes[('baseline-one-case-ex', 2000)]
        with tempfile.TemporaryDirectory() as folder:
            path = os.path.join(folder, 'plan.json')
            with redirect_stdout(io.StringIO()), mock.patch('sys.stderr', io.StringIO()) as err, \
                    self.assertRaises(SystemExit):
                rr.main(['--write-plan', path, '--subset', 'baseline', '--models', folder])
            self.assertIn('baseline-one-case-ex-n2000.json: expected sha256 %s' % w1, err.getvalue())
            self.assertIn('found no entry in the manifest', err.getvalue())
            self.assertFalse(os.path.exists(path))
            # The plan document itself, called directly, refuses a rack listed without a sha256 too,
            # unlike in a default subset's plan.
            with open(os.path.join(folder, 'manifest.json'), 'w') as fh:
                json.dump({'models': {'reference_system.json': {'sha256': reference, 'stats': {}}}}, fh)
            with self.assertRaises(ValueError) as refused:
                rr.plan_document(plan, models_dir=folder, require_sha256=True)
            self.assertIn('baseline-one-case-ex-n2000.json is not in the models manifest', str(refused.exception))
            pinned_models(folder)
            self.assertEqual(rr.main(['--write-plan', path, '--subset', 'baseline', '--models', folder]), 0)
            with open(path) as fh:
                document = json.load(fh)
        self.assertEqual((document['schema'], document['window_budget_s'], document['quiet_wait_s']),
                         (rr.PLAN_SCHEMA, 2160, rr.QUIET_WAIT_S))
        self.assertEqual([r['model'] for r in document['runs']],
                         ['baseline-one-case-ex-n2000.json', 'reference_system.json'] * 5)
        self.assertEqual([r['expected_model_sha256'] for r in document['runs']], [w1, reference] * 5)


class ReferenceSystemModel(unittest.TestCase):
    """A committed model that is not a rack: `--generate` copies the committed reference system after
    its fixture generator's `--check`, and refuses when the check fails."""

    def test_generate_copies_the_committed_file_and_records_its_sha256(self):
        source = rr.COMMITTED_MODELS['reference-system']['source']
        self.assertTrue(source.endswith(os.path.join('fixtures', 'reference_system', 'reference_system.json')))
        with open(source, 'rb') as fh:
            committed = fh.read()
        with tempfile.TemporaryDirectory() as models:
            plan, _, _ = rr.named_plan('baseline')
            manifest, failures = rr.generate([r for r in plan if r['profile'] == 'reference-system'], models,
                                             log=lambda *a: None)
            self.assertEqual(failures, [])
            entry = manifest['models']['reference_system.json']
            with open(os.path.join(models, 'reference_system.json'), 'rb') as fh:
                self.assertEqual(fh.read(), committed)
        self.assertEqual(entry['sha256'], hashlib.sha256(committed).hexdigest())
        self.assertEqual((entry['source'], entry['source_check']),
                         ('fixtures/reference_system/reference_system.json', 'generate_reference_system.py --check'))
        document = json.loads(committed)
        self.assertEqual(entry['stats']['nodes'], len(document['nodes']))
        self.assertEqual(entry['nodes'], len(document['nodes']), 'the plan label is the fixture\'s node count')
        self.assertEqual(entry['stats']['route'], 'EX')

    def test_a_failing_fixture_check_refuses_and_copies_nothing(self):
        calls = []

        def failing(argv, **kwargs):
            calls.append(argv)
            return types.SimpleNamespace(returncode=1, stdout='differs: reference_system.json\n', stderr='')

        with tempfile.TemporaryDirectory() as models:
            with open(os.path.join(models, 'manifest.json'), 'w') as fh:
                json.dump({'models': {'reference_system.json': {'sha256': 'c' * 64}}}, fh)
            plan = rr.build_plan((('reference-system', 27, 60, 5, 'solved'),))
            logged = []
            manifest, failures = rr.generate(plan, models, run=failing, log=logged.append)
            self.assertEqual(failures, ['reference_system.json'])
            self.assertNotIn('reference_system.json', manifest['models'], 'an earlier entry is dropped')
            self.assertFalse(os.path.exists(os.path.join(models, 'reference_system.json')))
            self.assertEqual(calls, [[sys.executable, rr.COMMITTED_MODELS['reference-system']['check'], '--check']])
            self.assertIn('refused', logged[0])
            with self.assertRaises(ValueError):
                rr.plan_document(plan, models_dir=models)


_PINNED = []


def pinned_models(folder):
    """`folder` filled with the baseline's model files as the repository generates them (the 2,000-node
    W1 and the reference system; generated once per session, then copied) and their manifest."""
    if not _PINNED:
        _PINNED.append(tempfile.TemporaryDirectory())
        plan, _, _ = rr.named_plan('baseline')
        _, failures = rr.generate(plan, _PINNED[0].name, log=lambda *a: None, pinned_hashes=rr.RACK_HASHES)
        assert not failures, failures
    for name in ('baseline-one-case-ex-n2000.json', 'reference_system.json', 'manifest.json'):
        shutil.copyfile(os.path.join(_PINNED[0].name, name), os.path.join(folder, name))
    return rr.read_manifest(folder)['models']


class PinnedModels(unittest.TestCase):
    """A named plan is generated and written only from the repository's own model bytes: its rack
    models hash to their lines of observations/rack/models_sha256.txt and the reference system to the
    committed file; `--generator` is refused for it."""

    # Writes the 46-node baseline model whatever size is asked: valid bytes, not the plan's.
    WRONG_SIZE = ('import subprocess, sys\nargs = sys.argv[1:]\nargs[args.index("--nodes") + 1] = "46"\n'
                  'raise SystemExit(subprocess.run([sys.executable, %r] + args).returncode)\n')

    def _wrong_generator(self, folder):
        path = os.path.join(folder, 'wrong_size.py')
        with open(path, 'w') as fh:
            fh.write(self.WRONG_SIZE % os.path.abspath(rr.GENERATOR))
        return path

    def _main(self, argv):
        with redirect_stdout(io.StringIO()) as out, mock.patch('sys.stderr', io.StringIO()) as err:
            try:
                code = rr.main(argv)
            except SystemExit as stop:
                code = stop.code
        return code, out.getvalue(), err.getvalue()

    def test_a_named_plan_refuses_another_generator_and_a_default_subset_keeps_it(self):
        with tempfile.TemporaryDirectory() as folder:
            wrong = self._wrong_generator(folder)
            for name in rr.NAMED_PLANS:
                code, _, err = self._main(['--generate', '--subset', name, '--models', os.path.join(folder, name),
                                           '--generator', wrong])
                self.assertEqual(code, 2, name)
                self.assertIn('leave out --generator wrong_size.py', err)
                self.assertIn('models_sha256.txt', err)
                self.assertFalse(os.path.exists(os.path.join(folder, name)), 'nothing generated')
            # The repository's own generator, named explicitly, is the default and is accepted.
            code, _, _ = self._main(['--plan', '--subset', 'baseline', '--generator', rr.GENERATOR])
            self.assertEqual(code, 0)
            fake = os.path.join(folder, 'fake_rack.py')
            with open(fake, 'w') as fh:
                fh.write(Generate.FAKE)
            models = os.path.join(folder, 'window')
            code, _, _ = self._main(['--generate', '--subset', 'window', '--models', models, '--generator', fake])
            self.assertEqual(code, 0, 'a default subset still takes --generator')
            self.assertEqual(rr.read_manifest(models)['generator']['name'], 'fake_rack.py')

    def test_a_pinned_rack_model_that_differs_from_its_recorded_line_is_refused(self):
        expected = rr.recorded_rack_hashes()[('baseline-one-case-ex', 250)]
        with tempfile.TemporaryDirectory() as folder:
            models = os.path.join(folder, 'models')
            with open(os.path.join(os.makedirs(models) or models, 'manifest.json'), 'w') as fh:
                json.dump({'models': {'baseline-one-case-ex-n0250.json': {'sha256': expected}}}, fh)
            plan = rr.build_plan((('baseline-one-case-ex', 250, 60, 1, 'solved'),))
            logged = []
            manifest, failures = rr.generate(plan, models, generator=self._wrong_generator(folder), log=logged.append,
                                             pinned_hashes=rr.RACK_HASHES)
            self.assertEqual(failures, ['baseline-one-case-ex-n0250.json'])
            self.assertNotIn('baseline-one-case-ex-n0250.json', manifest['models'], 'an earlier entry is dropped too')
            self.assertFalse(os.path.exists(os.path.join(models, 'baseline-one-case-ex-n0250.json')))
            self.assertIn('refused: expected sha256 %s' % expected, logged[0])
            self.assertIn('baseline-one-case-ex-n0250.json', logged[0])
            self.assertRegex(logged[0], r'found [0-9a-f]{64}$')
            # The repository's generator writes the recorded bytes, and the model is kept.
            manifest, failures = rr.generate(plan, models, log=lambda *a: None, pinned_hashes=rr.RACK_HASHES)
            self.assertEqual((failures, manifest['models']['baseline-one-case-ex-n0250.json']['sha256']), ([], expected))

    def test_a_pinned_profile_and_size_with_no_recorded_line_is_refused_before_the_generator_runs(self):
        with tempfile.TemporaryDirectory() as folder:
            plan = rr.build_plan((('baseline-one-case-ex', 46, 60, 1, 'solved'),))
            logged = []
            run = mock.Mock(side_effect=AssertionError('the generator ran'))
            manifest, failures = rr.generate(plan, folder, run=run, log=logged.append, pinned_hashes=rr.RACK_HASHES)
            self.assertEqual((failures, manifest['models']), (['baseline-one-case-ex-n0046.json'], {}))
            self.assertIn('no line "baseline-one-case-ex  46"', logged[0])
            run.assert_not_called()

    def _folder(self, folder, w1=None, w2=None):
        """The repository's own model files, with the manifest's sha256 replaced where given."""
        listed = pinned_models(folder)
        for name, sha in (('baseline-one-case-ex-n2000.json', w1), ('reference_system.json', w2)):
            if sha:
                listed[name]['sha256'] = sha
        with open(os.path.join(folder, 'manifest.json'), 'w') as fh:
            json.dump({'models': listed}, fh)
        return listed

    def test_every_model_is_compared_again_when_a_named_plan_is_written(self):
        plan, _, _ = rr.named_plan('baseline')
        with tempfile.TemporaryDirectory() as folder:
            self._folder(folder)
            rr.check_pinned_models(plan, folder)
            path = os.path.join(folder, 'plan.json')
            self.assertEqual(self._main(['--write-plan', path, '--subset', 'baseline', '--models', folder])[0], 0)
            os.unlink(path)
            for w1, w2, wrong in (('d' * 64, None, 'baseline-one-case-ex-n2000.json'),
                                  (None, 'e' * 64, 'reference_system.json')):
                self._folder(folder, w1, w2)
                with self.assertRaises(ValueError) as refused:
                    rr.check_pinned_models(plan, folder)
                self.assertIn('%s: expected sha256' % wrong, str(refused.exception))
                self.assertIn('found %s' % (w1 or w2), str(refused.exception))
                for argv in (['--write-plan', path], ['--plan']):
                    code, out, err = self._main(argv + ['--subset', 'baseline', '--models', folder])
                    self.assertEqual(code, 2, argv)
                    self.assertIn('own model bytes; refused: %s' % wrong, err)
                    self.assertFalse(os.path.exists(path), 'no plan file')
                    self.assertEqual(out, '')
        # The model FILES are hashed too: a manifest that agrees with the repository does not cover a file
        # whose bytes differ, or a file that is missing.
        for change, message in (('corrupt', 'the file in the models folder has'), ('remove', 'no such file')):
            with tempfile.TemporaryDirectory() as folder:
                self._folder(folder)
                w1 = os.path.join(folder, 'baseline-one-case-ex-n2000.json')
                if change == 'corrupt':
                    with open(w1, 'w') as fh:
                        fh.write('{"nodes":[]}')
                else:
                    os.unlink(w1)
                path = os.path.join(folder, 'plan.json')
                for argv in (['--write-plan', path], ['--plan'], ['--run', '--out', os.path.join(folder, 'out'),
                                                                  '--binary', '/no-driver']):
                    code, out, err = self._main(argv + ['--subset', 'baseline', '--models', folder])
                    self.assertEqual(code, 2, (change, argv))
                    self.assertIn('refused: baseline-one-case-ex-n2000.json: ', err)
                    self.assertIn(message, err)
                    self.assertFalse(os.path.exists(path) or os.path.exists(os.path.join(folder, 'out')))
        # A default subset's plan is written from whatever the manifest lists, as before.
        with tempfile.TemporaryDirectory() as folder:
            with open(os.path.join(folder, 'manifest.json'), 'w') as fh:
                json.dump({'models': {rr.model_name('linear-ex', 250): {'sha256': 'f' * 64}}}, fh)
            self.assertEqual(self._main(['--write-plan', os.path.join(folder, 'p.json'), '--subset', 'window',
                                         '--models', folder])[0], 0)

    def test_plan_prints_a_named_plans_quiet_wait_and_a_default_subsets_text_is_unchanged(self):
        code, out, _ = self._main(['--plan', '--subset', 'baseline'])
        self.assertEqual(code, 0)
        self.assertIn('quiet-host wait: up to %d s before the first run' % rr.QUIET_WAIT_S, out)
        # The window's condition, stated where the window is printed.
        self.assertIn('provided the supervisor\'s own work between runs', out)
        self.assertIn('planning allowance of 30 s per run', out)
        self.assertIn('INCOMPLETE BY THE WINDOW', out)
        self.assertNotIn('quiet-host wait', self._main(['--plan', '--subset', 'window'])[1])


def set_records(plan, models=None):
    """A folder's records for `plan` in which every run is a completed timing: the plan document's
    runs (with their expected sha256) and one record each, as the supervisor writes them."""
    models = models or {'baseline-one-case-ex-n2000.json': 'a' * 64, 'reference_system.json': 'b' * 64,
                        'linear-ex-n2000.json': 'c' * 64}
    runs, records = [], []
    for r in plan:
        name = rr.model_name(r['profile'], r['nodes'])
        runs.append(dict({k: r[k] for k in rr.PLAN_RUN_KEYS}, model=name, expected_model_sha256=models[name]))
        solved = r['expect'] == 'solved'
        line = driver(document={'sha256': models[name], 'bytes': 1}, output_sha256='%s-out' % name,
                      rows=7 if 'reference' in name else 9, output_bytes=100)
        line['seconds'] = dict(line['seconds'], entry=1.0 + r['order'] / 100)
        records.append(dict(r, model=name, model_sha256=models[name], timeout_applied_s=r['timeout_s'],
                            timeout_capped_by_window=False, driver=line,
                            classification='ok' if solved else 'exceeds_base_machine_budget', as_expected=True,
                            launch={'rss': {'exit_lifetime_max_phys_footprint_bytes': 2 * k6.GIB}}))
    return runs, records


TIMING_COMPLETE = 'TEN-RUN TIMING SET: COMPLETE'
TIMING_WINDOW = 'TEN-RUN TIMING SET: INCOMPLETE BY THE WINDOW'
TIMING_FINDINGS = 'TEN-RUN TIMING SET: FINDINGS'
RACK_OK = 'RACK OBSERVATION: OBSERVED AS EXPECTED'
RACK_WINDOW = 'RACK OBSERVATION: INCOMPLETE BY THE WINDOW'
RACK_FINDINGS = 'RACK OBSERVATION: FINDINGS'
RACK_NONE = 'RACK OBSERVATION: NONE IN THIS PLAN'


def results(report):
    """(timing set result, rack observation result, exit status)."""
    return report['timing_set']['result'], report['rack_observation']['result'], report['exit_code']


def unqualified_complete(text):
    """Every place `COMPLETE` stands as a word in `text` without the timing set's name before it."""
    return [m.start() for m in re.finditer(r'(?<![A-Z])COMPLETE', text)
            if text[max(0, m.start() - len('TEN-RUN TIMING SET: ')):m.start()] != 'TEN-RUN TIMING SET: ']


class BaselineSet(unittest.TestCase):
    """`--baseline-set`: two results on one supervised folder of a named plan, from the records' fields:
    the ten-run timing set and the rack observation, under one exit contract."""

    def judge(self, mutate=None, name='baseline'):
        plan, _, _ = rr.named_plan(name)
        runs, records = set_records(plan)
        by_id = {r['run_id']: r for r in records}
        if mutate:
            mutate(by_id, records, runs)
        return rr.judge_set(records, runs)

    def test_complete_gives_each_workloads_median_minimum_and_maximum(self):
        report = self.judge()
        self.assertEqual(results(report), (TIMING_COMPLETE, RACK_NONE, rr.SET_EXIT_OK))
        self.assertEqual(report['results'], [TIMING_COMPLETE, RACK_NONE])
        w1 = report['timing_set']['workloads']['baseline-one-case-ex n2000']
        self.assertEqual((w1['planned'], w1['completed']), (5, 5))
        self.assertEqual((w1['median_entry_s'], w1['min_entry_s'], w1['max_entry_s']), (1.04, 1.0, 1.08))
        self.assertEqual(report['timing_set']['workloads']['reference-system n27']['completed'], 5)
        self.assertEqual((report['timing_set']['findings'], report['timing_set']['window_problems']), ([], []))
        self.assertEqual(report['host_not_quiet'], [])
        self.assertEqual(unqualified_complete(json.dumps(report)), [])
        # A host check that was not quiet, or read no memory level, is reported and leaves the results as they are.
        report = self.judge(lambda by, *_: (by['reference-system-n0027-sparse-r1'].update(host={'quiet': False}),
                                            by['reference-system-n0027-sparse-r2'].update(host={'quiet': True})))
        self.assertEqual((results(report), report['host_not_quiet']),
                         ((TIMING_COMPLETE, RACK_NONE, 0), ['reference-system-n0027-sparse-r1', 'reference-system-n0027-sparse-r2']))
        # A run whose timeout the window capped but which completed is unaffected.
        report = self.judge(lambda by, *_: by['reference-system-n0027-sparse-r5'].update(
            timeout_capped_by_window=True, timeout_applied_s=40))
        self.assertEqual(results(report), (TIMING_COMPLETE, RACK_NONE, 0))

    def test_a_skipped_or_capped_and_timed_out_run_is_incomplete_by_the_window(self):
        def window(by, *_):
            by['reference-system-n0027-sparse-r5'].update(classification='skipped_window_budget', driver=None)
            by['baseline-one-case-ex-n2000-sparse-r5'].update(classification='timed_out', timeout_applied_s=250,
                                                               timeout_capped_by_window=True, driver=None)
        report = self.judge(window)
        self.assertEqual(results(report), (TIMING_WINDOW, RACK_NONE, rr.SET_EXIT_WINDOW))
        self.assertEqual(report['timing_set']['findings'], [])
        self.assertEqual(len(report['timing_set']['window_problems']), 2)
        self.assertIn('baseline-one-case-ex-n2000-sparse-r5: timed_out under a timeout the window capped to 250 s',
                      report['timing_set']['window_problems'])
        self.assertEqual(report['timing_set']['workloads']['baseline-one-case-ex n2000']['completed'], 4)

    def test_each_finding_alone_is_findings(self):
        r1 = 'baseline-one-case-ex-n2000-sparse-r1'
        cases = {
            'disagreeing output': lambda by, *_: by[r1]['driver'].update(output_sha256='other'),
            'memory breach': lambda by, *_: by[r1].update(classification='exceeds_base_machine_budget', as_expected=False),
            'full timeout': lambda by, *_: by[r1].update(classification='timed_out', as_expected=False),
            'refusal': lambda by, *_: by[r1].update(classification='refused', as_expected=False),
            'route': lambda by, *_: by[r1]['driver'].update(route='PF'),
            'model mismatch': lambda by, *_: by[r1].update(model_sha256='d' * 64),
            'document hash': lambda by, *_: by[r1]['driver'].update(document={'sha256': 'd' * 64}),
            'plan hash': lambda by, records, runs: runs[0].update(expected_model_sha256='d' * 64),
            'null footprint': lambda by, *_: by[r1]['launch']['rss'].update(exit_lifetime_max_phys_footprint_bytes=None),
            'no entry figure': lambda by, *_: by[r1]['driver']['seconds'].pop('entry'),
            'missing record': lambda by, records, runs: records.remove(by[r1]),
        }
        for label, mutate in cases.items():
            report = self.judge(mutate)
            self.assertEqual(results(report), (TIMING_FINDINGS, RACK_NONE, rr.SET_EXIT_FINDINGS), label)
            self.assertTrue(report['timing_set']['findings'], label)
            self.assertEqual(report['timing_set']['window_problems'], [], label)
        report = self.judge(cases['full timeout'])
        self.assertIn('timed out at its full timeout', report['timing_set']['findings'][0])
        report = self.judge(cases['missing record'])
        self.assertIn('baseline-one-case-ex-n2000-sparse-r1: no record', report['timing_set']['findings'][0])

    def test_a_finding_with_a_window_problem_is_findings_and_lists_both(self):
        def both(by, *_):
            by['reference-system-n0027-sparse-r5'].update(classification='skipped_window_budget', driver=None)
            by['baseline-one-case-ex-n2000-sparse-r2']['driver'].update(route='PF')
        report = self.judge(both)
        self.assertEqual(results(report), (TIMING_FINDINGS, RACK_NONE, 1))
        self.assertEqual(report['timing_set']['window_problems'], ['reference-system-n0027-sparse-r5: skipped_window_budget'])
        self.assertIn('driver.route', report['timing_set']['findings'][0])

    def test_the_rack_observation_is_judged_apart_from_the_ten_timings(self):
        """The round 2 reader's rack outcomes, each against the declared expectation
        exceeds_base_machine_budget; the ten timings stay COMPLETE whatever the rack did."""
        rack = 'linear-ex-n2000-sparse'
        stage = {'repeat': 0, 'stage': 'request', 'seconds': 0.5}

        def outcome(classification, detail=None, expected=False):
            return lambda by, *_: by[rack].update(classification=classification, classification_detail=detail,
                                                  as_expected=expected, last_stage=stage)

        # The expected budget-limited outcome is a successful observation, reported as it is.
        report = self.judge(outcome('exceeds_base_machine_budget', 'footprint_1_above_budget_2_killed', True),
                            name='baseline-with-rack')
        self.assertEqual(results(report), (TIMING_COMPLETE, RACK_OK, rr.SET_EXIT_OK))
        self.assertEqual(report['rack_observation']['runs'], [{
            'run_id': rack, 'expect': 'exceeds_base_machine_budget', 'observed': True,
            'classification': 'exceeds_base_machine_budget', 'classification_detail': 'footprint_1_above_budget_2_killed',
            'last_stage': stage, 'as_expected': True, 'result': 'OBSERVED AS EXPECTED'}])
        self.assertEqual(sorted(report['timing_set']['workloads']), ['baseline-one-case-ex n2000', 'reference-system n27'])
        # Not observed: no record, no driver started, or other model bytes.
        not_observed = {
            'missing record': lambda by, records, runs: records.remove(by[rack]),
            'model_missing': lambda by, *_: (by[rack].pop('launch'), by[rack].pop('model_sha256'), by[rack].update(
                classification='model_missing', classification_detail='linear-ex-n2000.json', driver=None)),
            'model_mismatch': lambda by, *_: (by[rack].pop('launch'), by[rack].update(
                classification='model_mismatch', model_sha256='d' * 64, driver=None)),
            'error before launch': lambda by, *_: (by[rack].pop('launch'), by[rack].update(classification='error',
                                                                                            driver=None)),
            'other model bytes': lambda by, *_: by[rack].update(model_sha256='d' * 64),
        }
        # Observed, but the outcome does not meet the declared expectation.
        unmet = {
            'entry error': outcome('entry_error', 'entry: failed'),
            'refusal': outcome('refused', 'NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED'),
            'full timeout': outcome('timed_out', 'after_900_s'),
            'resident-cap kill': outcome('killed_by_rss_cap', 'killed_above_rss_cap'),
            'ok where a budget limit is expected': outcome('ok'),
        }
        for label, mutate in list(not_observed.items()) + list(unmet.items()):
            report = self.judge(mutate, name='baseline-with-rack')
            self.assertEqual(results(report), (TIMING_COMPLETE, RACK_FINDINGS, rr.SET_EXIT_FINDINGS), label)
            self.assertIn(rack, report['rack_observation']['findings'][0], label)
            self.assertEqual(report['rack_observation']['runs'][0]['result'], 'FINDING', label)
            self.assertEqual(report['rack_observation']['runs'][0]['observed'], label in unmet, label)
            self.assertEqual(report['timing_set']['findings'], [], 'a rack finding does not invalidate the ten')
        report = self.judge(unmet['entry error'], name='baseline-with-rack')
        self.assertIn('does not meet its declared expectation exceeds_base_machine_budget',
                      report['rack_observation']['findings'][0])
        self.assertEqual(report['rack_observation']['runs'][0]['classification_detail'], 'entry: failed')
        # A window skip is INCOMPLETE BY THE WINDOW for the observation.
        skipped = lambda by, *_: (by[rack].pop('launch'), by[rack].pop('model_sha256'),  # noqa: E731
                                  by[rack].update(classification='skipped_window_budget', driver=None))
        report = self.judge(skipped, name='baseline-with-rack')
        self.assertEqual(results(report), (TIMING_COMPLETE, RACK_WINDOW, rr.SET_EXIT_WINDOW))
        self.assertEqual(report['rack_observation']['window_problems'], [rack + ': skipped_window_budget'])
        self.assertEqual((report['rack_observation']['runs'][0]['observed'],
                          report['rack_observation']['runs'][0]['classification']), (False, 'skipped_window_budget'))
        # A launched run that timed out under a timeout the window capped: observed, its outcome given.
        capped = lambda by, *_: by[rack].update(classification='timed_out', classification_detail='after_100_s',  # noqa: E731
                                                timeout_capped_by_window=True, timeout_applied_s=100, as_expected=False,
                                                last_stage=stage)
        report = self.judge(capped, name='baseline-with-rack')
        self.assertEqual(results(report), (TIMING_COMPLETE, RACK_WINDOW, rr.SET_EXIT_WINDOW))
        self.assertEqual(report['rack_observation']['runs'], [{
            'run_id': rack, 'expect': 'exceeds_base_machine_budget', 'observed': True, 'classification': 'timed_out',
            'classification_detail': 'after_100_s', 'last_stage': stage, 'as_expected': False,
            'result': 'INCOMPLETE BY THE WINDOW'}])
        self.assertIn('capped to 100 s', report['rack_observation']['window_problems'][0])
        # A rack finding with a window problem among the ten: exit 1, both listed, each in its own result.
        report = self.judge(lambda by, records, runs: (not_observed['model_missing'](by), by[
            'reference-system-n0027-sparse-r5'].update(classification='skipped_window_budget', driver=None)),
            name='baseline-with-rack')
        self.assertEqual(results(report), (TIMING_WINDOW, RACK_FINDINGS, rr.SET_EXIT_FINDINGS))

    def test_the_supervisors_own_capped_timeout_is_observed_and_its_skip_is_not(self):
        """The fixture's window/ folder, written by the self-supervising driver: two runs a plan expects
        exceeds_base_machine_budget, in a 4 s window.
        - Run r1 is a LAUNCHED, window-capped timeout. supervise.rs `run_plan` caps the timeout to what
          remains (`let timeout = run.timeout_s.min(remaining.trunc() as i64)`, recording
          `timeout_capped_by_window`), launches the driver, and classify.rs classifies the kill
          `timed_out` (`after_<timeout>_s`); the record keeps its `launch`.
        - Run r2 is a SKIP for the window. supervise.rs `run_plan` writes `skipped_window_budget` when the
          run's estimate exceeds the remaining window, before any host check or launch ("estimate N s
          exceeds the remaining M s"); the record has no `launch`.
        The plan holds no run expected solved, so the timing set has no workload; only the rack observation
        is read here."""
        with tempfile.TemporaryDirectory() as folder:
            extract_fixture(folder, 'window')
            with open(os.path.join(folder, 'plan.json')) as fh:
                runs = json.load(fh)['runs']
            records = rr.read_set_records(os.path.join(folder, 'out'))[0]
        capped, skipped = records
        self.assertEqual((capped['classification'], capped['timeout_capped_by_window'], 'launch' in capped),
                         ('timed_out', True, True))
        self.assertEqual((skipped['classification'], 'launch' in skipped), ('skipped_window_budget', False))
        report = rr.judge_set(records, runs)
        rack = report['rack_observation']
        self.assertEqual((rack['result'], report['exit_code']), (RACK_WINDOW, rr.SET_EXIT_FINDINGS))
        self.assertEqual([(r['run_id'], r['observed'], r['classification'], r['result']) for r in rack['runs']], [
            ('baseline-one-case-ex-n0046-sparse-r1', True, 'timed_out', 'INCOMPLETE BY THE WINDOW'),
            ('baseline-one-case-ex-n0046-sparse-r2', False, 'skipped_window_budget', 'INCOMPLETE BY THE WINDOW')])
        self.assertEqual(rack['runs'][0]['classification_detail'], capped['classification_detail'])
        self.assertEqual(rack['runs'][0]['last_stage'], capped['last_stage'])
        self.assertEqual(rack['findings'], [])
        self.assertIn('capped to %s s' % capped['timeout_applied_s'], rack['window_problems'][0])

    def test_absent_output_figures_are_a_finding_even_where_every_run_lacks_them(self):
        def absent(by, records, runs):
            for r in records:
                for key in ('output_sha256', 'rows', 'output_bytes'):
                    r['driver'].pop(key, None)
        report = self.judge(absent)
        self.assertEqual(results(report), (TIMING_FINDINGS, RACK_NONE, 1))
        self.assertEqual(len(report['timing_set']['findings']), 10)
        self.assertIn('are not all present', report['timing_set']['findings'][0])

    def _cli_folder(self, folder, name='baseline'):
        plan, budget, wait = rr.named_plan(name)
        hashes = rr.recorded_rack_hashes()
        with open(rr.COMMITTED_MODELS['reference-system']['source'], 'rb') as fh:
            reference = hashlib.sha256(fh.read()).hexdigest()
        runs, records = set_records(plan, {'baseline-one-case-ex-n2000.json': hashes[('baseline-one-case-ex', 2000)],
                                           'reference_system.json': reference,
                                           'linear-ex-n2000.json': hashes[('linear-ex', 2000)]})
        out = os.path.join(folder, 'out')
        os.makedirs(out, exist_ok=True)
        with open(os.path.join(out, k6.RECORDS_NAME), 'w') as fh:
            fh.writelines(json.dumps(r) + '\n' for r in records)
        plan_path = os.path.join(folder, 'plan.json')
        with open(plan_path, 'w') as fh:
            json.dump({'schema': rr.PLAN_SCHEMA, 'window_budget_s': budget, 'quiet_wait_s': wait, 'runs': runs}, fh)
        return out, plan_path, runs, records

    def _cli(self, argv):
        with redirect_stdout(io.StringIO()) as printed, mock.patch('sys.stderr', io.StringIO()) as err:
            code = rr.main(argv)
        return code, json.loads(printed.getvalue()), err.getvalue()

    def test_the_cli_prints_both_results_and_the_exit_contract(self):
        with tempfile.TemporaryDirectory() as folder:
            out, plan_path, _, records = self._cli_folder(folder, 'baseline-with-rack')
            aggregate = os.path.join(out, k6.RECORDS_NAME)
            before = (os.path.getmtime(aggregate), rr.sha256_file(aggregate))
            argv = ['--baseline-set', out, '--plan-file', plan_path, '--subset', 'baseline-with-rack']
            code, report, err = self._cli(argv)
            self.assertEqual((code, report['results']), (0, [TIMING_COMPLETE, RACK_OK]))
            self.assertEqual(err.splitlines()[:2], [TIMING_COMPLETE, RACK_OK])
            self.assertIn('exit 0 (exit 1 when any finding exists', err)
            self.assertEqual(report['exit_contract'], rr.EXIT_CONTRACT)
            self.assertEqual(unqualified_complete(json.dumps(report) + err), [])
            self.assertEqual((os.path.getmtime(aggregate), rr.sha256_file(aggregate)), before)
            for name in rr.FOLDER_NAMES + ('baseline-set.json',):
                if name != k6.RECORDS_NAME:
                    self.assertFalse(os.path.exists(os.path.join(out, name)), 'it writes nothing: %s' % name)
            # A rack finding: the ten stay COMPLETE, the rack is FINDINGS and the status is 1.
            records[-1].update(classification='model_missing', driver=None)
            records[-1].pop('launch')
            with open(os.path.join(out, k6.RECORDS_NAME), 'w') as fh:
                fh.writelines(json.dumps(r) + '\n' for r in records)
            code, report, err = self._cli(argv)
            self.assertEqual((code, report['results']), (1, [TIMING_COMPLETE, RACK_FINDINGS]))
            self.assertEqual(err.splitlines()[:2], [TIMING_COMPLETE, RACK_FINDINGS])
        with redirect_stdout(io.StringIO()) as printed:
            with self.assertRaises(SystemExit):
                rr.main(['--help'])
        self.assertIn('exit 1 when any finding exists', ' '.join(printed.getvalue().split()))

    def test_the_cli_holds_the_plan_to_the_named_plan(self):
        with tempfile.TemporaryDirectory() as folder:
            out, plan_path, runs, records = self._cli_folder(folder)
            argv = ['--baseline-set', out, '--plan-file', plan_path, '--subset', 'baseline']
            self.assertEqual(self._cli(argv)[0], 0)
            # The same records judged as baseline-with-rack: not that plan, a finding in both results.
            code, report, _ = self._cli(argv[:-1] + ['baseline-with-rack'])
            self.assertEqual((code, report['results']), (1, [TIMING_FINDINGS, RACK_FINDINGS]))
            self.assertIn('is not the named plan baseline-with-rack', report['folder_findings'][0])
            # A plan whose hash is not the repository's own is a finding even where the records agree with it.
            runs[0]['expected_model_sha256'] = records[0]['model_sha256'] = 'e' * 64
            records[0]['driver']['document']['sha256'] = 'e' * 64
            with open(plan_path, 'w') as fh:
                json.dump({'schema': rr.PLAN_SCHEMA, 'window_budget_s': 2160, 'quiet_wait_s': 60, 'runs': runs}, fh)
            with open(os.path.join(out, k6.RECORDS_NAME), 'w') as fh:
                fh.writelines(json.dumps(r) + '\n' for r in records)
            code, report, _ = self._cli(argv)
            self.assertEqual(code, 1)
            self.assertIn("the plan's expected_model_sha256 " + 'e' * 64, report['folder_findings'][0])
            with redirect_stdout(io.StringIO()), mock.patch('sys.stderr', io.StringIO()), self.assertRaises(SystemExit):
                rr.main(['--baseline-set', out, '--plan-file', plan_path, '--subset', 'window'])

    def test_every_top_level_field_and_run_field_of_the_plan_file_is_the_named_plans(self):
        cases = {
            'none': lambda d: None,
            'window_budget_s': lambda d: d.update(window_budget_s=1960),
            'quiet_wait_s': lambda d: d.update(quiet_wait_s=0),
            'schema': lambda d: d.update(schema='release-timing-plan-v1'),
            'an added top-level field': lambda d: d.update(extra=1),
            'a missing top-level field': lambda d: d.pop('quiet_wait_s'),
            'estimate_s': lambda d: d['runs'][0].update(estimate_s=1),
            'attribute': lambda d: d['runs'][0].update(attribute=True),
            'model': lambda d: d['runs'][1].update(model='other.json'),
            'an added run field': lambda d: d['runs'][2].update(extra=1),
        }
        with tempfile.TemporaryDirectory() as folder:
            out, plan_path, _, _ = self._cli_folder(folder)
            with open(plan_path) as fh:
                original = json.load(fh)
            for label, change in cases.items():
                document = json.loads(json.dumps(original))
                change(document)
                with open(plan_path, 'w') as fh:
                    json.dump(document, fh)
                report = rr.baseline_set(out, plan_path, 'baseline')
                if label == 'none':
                    self.assertEqual(results(report), (TIMING_COMPLETE, RACK_NONE, 0))
                    continue
                self.assertEqual(results(report), (TIMING_FINDINGS, RACK_FINDINGS, 1), label)
                self.assertIn('is not the named plan baseline', report['folder_findings'][0], label)
                if label in ('window_budget_s', 'quiet_wait_s', 'schema'):
                    self.assertIn('%s differs' % label, report['folder_findings'][0])

    def test_overhead_above_the_allowance_through_the_run_path_is_incomplete_by_the_window(self):
        """The runner's own run loop with a fake clock and launcher: every run lasts its full timeout and
        each host check costs 40 s, above the 30 s planning allowance, so the window runs short."""
        plan, budget, wait = rr.named_plan('baseline')
        runs, _ = set_records(plan)
        with tempfile.TemporaryDirectory() as folder:
            models = os.path.join(folder, 'models')
            os.makedirs(models)
            shas = {}
            for name in ('baseline-one-case-ex-n2000.json', 'reference_system.json'):
                with open(os.path.join(models, name), 'w') as fh:
                    fh.write(name)
                shas[name] = rr.sha256_file(os.path.join(models, name))
            for r in runs:
                r['expected_model_sha256'] = shas[r['model']]
            now, first = [0.0], [True]

            def quiet(log, allowance):
                if first[0]:
                    now[0] += wait
                    first[0] = False
                now[0] += 40
                return {'quiet': True, 'waited_s': 0}

            def launch(argv, *, timeout_s, record_dir, run_id, **kwargs):
                run = next(r for r in plan if r['run_id'] == run_id)
                now[0] += timeout_s
                name = rr.model_name(run['profile'], run['nodes'])
                with open(os.path.join(record_dir, run_id + k6.STDOUT_SUFFIX), 'w') as fh:
                    fh.write(json.dumps(driver(document={'sha256': shas[name]}, output_sha256=name)) + '\n')
                if timeout_s < run['timeout_s']:
                    return launched('timed_out', exit_code=None, timeout_s=timeout_s)
                return launched(rss={'exit_lifetime_max_phys_footprint_bytes': k6.GIB, 'time_peak_footprint_bytes': k6.GIB})

            out = os.path.join(folder, 'out')
            records = rr.run_plan(plan, models, out, '/no-driver', budget_s=budget, quiet_wait_s=wait, launch=launch,
                                  quiet=quiet, clock=lambda: now[0], log=lambda *a: None, md={})
            self.assertTrue(any(r.get('timeout_capped_by_window') or r['classification'] == 'skipped_window_budget'
                                for r in records))
            report = rr.judge_set(rr.read_set_records(out)[0], runs)
        self.assertEqual(results(report), (TIMING_WINDOW, RACK_NONE, rr.SET_EXIT_WINDOW), report)
        self.assertTrue(report['timing_set']['window_problems'])


READ_BACK_FIXTURE = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'fixtures', 'read_back')
# The fixture holds, as the text files the supervisor wrote, two folders the self-supervising driver wrote (debug build, 46-node W1 and the
# reference system; CPU model blanked), each with its plan.json and models/manifest.json:
# - forms/: every packet row form. Completed: W1 r1 and W2 r1 (ok). Stopped: W1 r2 (timed_out, a 1 s
#   timeout) and W2 r2 (exceeds_base_machine_budget, a 1 MB footprint budget). Never launched: n0047
#   (model_missing), n0048 (model_mismatch) and W2 r3 (skipped_window_budget, an estimate above the window).
# - window/: two runs a plan expects exceeds_base_machine_budget, in a 4 s window with no quiet wait.
COMPLETED, COMPLETED_W2 = 'baseline-one-case-ex-n0046-sparse-r1', 'reference-system-n0027-sparse-r1'
STOPPED, STOPPED_W2 = 'baseline-one-case-ex-n0046-sparse-r2', 'reference-system-n0027-sparse-r2'
NEVER_LAUNCHED = ('baseline-one-case-ex-n0047-sparse', 'baseline-one-case-ex-n0048-sparse',
                  'reference-system-n0027-sparse-r3')


def extract_fixture(folder, bundle):
    """`bundle`'s files of the fixture, copied byte for byte under `folder`; their names."""
    names = []
    source = os.path.join(READ_BACK_FIXTURE, bundle)
    for parent, _, files in sorted(os.walk(source)):
        for name in sorted(files):
            names.append(os.path.relpath(os.path.join(parent, name), source).replace(os.sep, '/'))
            path = os.path.join(folder, names[-1])
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(os.path.join(parent, name), 'rb') as src, open(path, 'wb') as fh:
                fh.write(src.read())
    return names


def write_inventory(folder, names):
    """SHA256SUMS of `names` that are regular files reached without a link, as README step 8's
    `find . -type f` inventory writes it (a link is not a regular file there)."""
    lines = []
    for name in sorted(names):
        path = os.path.join(folder, name)
        if os.path.isfile(path) and not os.path.islink(path) and not os.path.islink(os.path.dirname(path)):
            with open(path, 'rb') as fh:
                lines.append('%s  ./%s\n' % (hashlib.sha256(fh.read()).hexdigest(), name))
    with open(os.path.join(folder, 'SHA256SUMS'), 'w') as fh:
        fh.writelines(lines)
    return root_digest(folder)


def root_digest(folder):
    """The SHA-256 of a bundle's SHA256SUMS, the root digest a worker states outside the bundle."""
    path = os.path.join(folder, 'SHA256SUMS')
    if not os.path.isfile(path):
        return '0' * 64
    with open(path, 'rb') as fh:
        return hashlib.sha256(fh.read()).hexdigest()


class BundleCase:
    """A bundle made from the fixture's forms/ folder, inventoried, with the helpers that change it."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.bundle = self.tmp.name
        self.names = extract_fixture(self.bundle, 'forms')
        for name in ('build-facts.txt', 'host-before.txt', 'host-processes.log', 'contract.md'):
            with open(os.path.join(self.bundle, name), 'w') as fh:
                fh.write('stand-in\n')
            self.names.append(name)
        # baseline-set.json as README step 7 writes it: the read-back recomputes it.
        with redirect_stdout(io.StringIO()) as printed, mock.patch('sys.stderr', io.StringIO()):
            rr.main(['--baseline-set', self.path('out'), '--plan-file', self.path('plan.json'), '--subset', 'baseline'])
        with open(self.path('baseline-set.json'), 'w') as fh:
            fh.write(printed.getvalue())
        self.names.append('baseline-set.json')
        self.revision = self.load('out/metadata.json')['source_commit']
        self.inventory()

    def inventory(self):
        self.root = write_inventory(self.bundle, self.names)

    def tearDown(self):
        self.tmp.cleanup()

    def path(self, name):
        return os.path.join(self.bundle, name)

    def load(self, name):
        with open(self.path(name)) as fh:
            return json.load(fh)

    def edit(self, name, change):
        value = self.load(name)
        change(value)
        with open(self.path(name), 'w') as fh:
            json.dump(value, fh)

    def records(self, change):
        with open(self.path('out/records.jsonl')) as fh:
            rows = [json.loads(line) for line in fh if line.strip()]
        change({r['run_id']: r for r in rows}, rows)
        with open(self.path('out/records.jsonl'), 'w') as fh:
            fh.writelines(json.dumps(r) + '\n' for r in rows)

    def row(self, packet, run_id):
        return next(r for r in packet['runs'] + packet['not_run'] if r['run_id'] == run_id)

    def problems(self):
        """The read-back after the bundle is inventoried again, so that a change reaches the semantic checks."""
        self.inventory()
        return rr.read_back(self.bundle, self.revision, self.root, allow_unreleased_build=True)



class ReadBack(BundleCase, unittest.TestCase):
    """`--read-back`: the direct checks of a bundle, on the fixture's forms/ folder."""

    def test_the_intact_bundle_has_no_difference_and_holds_every_row_form(self):
        self.assertEqual(self.problems(), [])
        with redirect_stdout(io.StringIO()) as printed:
            self.assertEqual(rr.main(['--read-back', self.bundle, '--revision', self.revision, '--root-digest', self.root,
                                      '--allow-unreleased-build']), 0)
        self.assertEqual(printed.getvalue(), 'read-back: 0 difference(s)\n%s\n%s\n' % (
            rr.UNRELEASED_BUILD_ALLOWED, rr.CLEAN_READ_BACK_LIMIT))
        forms = {r['run_id']: (rr.row_form(r), r['classification']) for r in rr.read_set_records(self.path('out'))[0]}
        self.assertEqual(forms, {
            COMPLETED: ('completed', 'ok'), COMPLETED_W2: ('completed', 'ok'), STOPPED: ('stopped', 'timed_out'),
            STOPPED_W2: ('stopped', 'exceeds_base_machine_budget'), NEVER_LAUNCHED[0]: ('not_run', 'model_missing'),
            NEVER_LAUNCHED[1]: ('not_run', 'model_mismatch'), NEVER_LAUNCHED[2]: ('not_run', 'skipped_window_budget')})
        # A stopped run's legitimate nulls (no driver line, so no route, rows or output hash) are kept, and
        # are not differences.
        packet = self.load('out/packet.json')
        self.assertEqual([self.row(packet, STOPPED)[k] for k in ('route', 'rows', 'output_sha256')], [None, None, None])
        self.assertIn('exclusive.log is missing', rr.read_back(self.bundle, self.revision, self.root, shared_host=True, allow_unreleased_build=True))

    def test_the_mapping_names_every_field_of_the_supervisors_rows(self):
        """PACKET_RUN_FIELDS is the one statement of the mapping: its fields are exactly the fields of a
        launched row the supervisor wrote, and NOT_RUN_KEYS those of a never-launched one."""
        def leaves(obj, prefix=''):
            for key, value in obj.items():
                if isinstance(value, dict) and key in ('counts', 'peak_memory_bytes'):
                    yield from leaves(value, prefix + key + '.')
                else:
                    yield prefix + key
        packet = self.load('out/packet.json')
        for run_id in (COMPLETED, STOPPED, STOPPED_W2):
            self.assertEqual(sorted(leaves(self.row(packet, run_id))), sorted(f for f, _, _ in rr.PACKET_RUN_FIELDS))
        for run_id in NEVER_LAUNCHED:
            self.assertEqual(sorted(self.row(packet, run_id)), sorted(rr.NOT_RUN_KEYS))
        for form, fields in rr.PACKET_REQUIRED_VALUES.items():
            known = rr.NOT_RUN_KEYS if form == 'not_run' else [f for f, _, _ in rr.PACKET_RUN_FIELDS]
            self.assertTrue(set(fields) <= set(known), form)

    def test_each_row_form_with_a_missing_key_and_a_wrong_value(self):
        cases = [
            # completed
            ('completed, a key missing', lambda d: self.row(d, COMPLETED).pop('output_sha256'),
             'packet[%s].output_sha256 is missing' % COMPLETED),
            ('completed, a wrong value', lambda d: self.row(d, COMPLETED)['peak_memory_bytes'].update(
                exit_lifetime_max_phys_footprint=1), 'peak_memory_bytes.exit_lifetime_max_phys_footprint is 1'),
            ('completed, a required value null', lambda d: self.row(d, COMPLETED).update(route=None),
             'packet[%s].route: a completed row requires a value' % COMPLETED),
            # stopped
            ('stopped, a key missing', lambda d: self.row(d, STOPPED).pop('last_stage'),
             'packet[%s].last_stage is missing' % STOPPED),
            ('stopped, a wrong value', lambda d: self.row(d, STOPPED).update(classification_detail='after_2_s'),
             "packet[%s].classification_detail is 'after_2_s'" % STOPPED),
            ('stopped, a value where the aggregate has none', lambda d: self.row(d, STOPPED).update(route='EX'),
             "packet[%s].route is 'EX', the aggregate gives None" % STOPPED),
            # never launched
            ('never launched, a key missing', lambda d: self.row(d, NEVER_LAUNCHED[2]).pop('classification_detail'),
             'packet[%s].classification_detail is missing' % NEVER_LAUNCHED[2]),
            ('never launched, a wrong value', lambda d: self.row(d, NEVER_LAUNCHED[0]).update(classification='ok'),
             "packet[%s].classification is 'ok'" % NEVER_LAUNCHED[0]),
            ('never launched, an added key', lambda d: self.row(d, NEVER_LAUNCHED[1]).update(model_sha256='x'),
             'packet[%s].model_sha256 is a field this check does not know' % NEVER_LAUNCHED[1]),
        ]
        packet = self.load('out/packet.json')
        for label, change, expected in cases:
            self.edit('out/packet.json', lambda d: d.update(packet))
            self.edit('out/packet.json', change)
            problems = self.problems()
            self.assertTrue(any(expected in p for p in problems), (label, problems[:3]))
        # A required value the aggregate itself lacks, though the packet agrees with it.
        self.edit('out/packet.json', lambda d: d.update(packet))
        self.records(lambda by, rows: by[COMPLETED]['launch']['rss'].update(exit_lifetime_max_phys_footprint_bytes=None))
        self.edit('out/%s.record.json' % COMPLETED, lambda d: d['rss'].update(exit_lifetime_max_phys_footprint_bytes=None))
        self.edit('out/packet.json', lambda d: self.row(d, COMPLETED)['peak_memory_bytes'].update(
            exit_lifetime_max_phys_footprint=None))
        self.assertIn('packet[%s].peak_memory_bytes.exit_lifetime_max_phys_footprint: a completed row requires a value, '
                      'and the row has none' % COMPLETED, self.problems())

    def test_every_field_of_every_row_form_is_compared(self):
        """The third reader's three controls, then each field of a completed, a stopped and a never-launched
        row, set to null where it has a value and to a value where it is null."""
        cases = [
            ('exit footprint changed', lambda d: self.row(d, COMPLETED)['peak_memory_bytes'].update(
                exit_lifetime_max_phys_footprint=1), 'exit_lifetime_max_phys_footprint is 1'),
            ('last stage changed', lambda d: self.row(d, COMPLETED)['last_stage'].update(stage='request'),
             "last_stage.stage is 'request'"),
            ('output hash deleted', lambda d: self.row(d, COMPLETED).pop('output_sha256'), 'output_sha256 is missing'),
        ]
        packet = self.load('out/packet.json')

        def leaves(obj, prefix=()):
            for key, value in obj.items():
                if isinstance(value, dict) and value:
                    yield from leaves(value, prefix + (key,))
                else:
                    yield prefix + (key,)
        for run_id in (COMPLETED, STOPPED, NEVER_LAUNCHED[2]):
            for keys in leaves(self.row(packet, run_id)):
                def change(d, run_id=run_id, keys=keys):
                    target = self.row(d, run_id)
                    for key in keys[:-1]:
                        target = target[key]
                    target[keys[-1]] = None if target[keys[-1]] is not None else 'x'
                # A row whose run id is changed is a row of no run, and its run's row is missing.
                expected = 'is not listed' if keys == ('run_id',) else '.'.join(keys)
                cases.append(('%s %s' % (run_id, '.'.join(keys)), change, expected))
        cases += [
            ('an added field in a launched row', lambda d: self.row(d, COMPLETED).update(extra=1),
             'extra is a field this check does not know'),
            ('a launched row under not_run', lambda d: d['not_run'].append(d['runs'].pop(0)), 'is listed under not_run'),
            ('a row listed twice', lambda d: d['runs'].append(dict(d['runs'][0])), 'listed more than once'),
            ('a row missing', lambda d: d['runs'].pop(0), 'is not listed'),
            ('classification counts', lambda d: d['classification_counts'].update(ok=9), 'classification_counts'),
            ('metadata', lambda d: d['metadata'].update(budget_s=1), 'packet.metadata'),
        ]
        self.assertGreater(len(cases), 100)
        for label, change, expected in cases:
            self.edit('out/packet.json', lambda d: d.update(packet))
            self.edit('out/packet.json', change)
            problems = self.problems()
            self.assertTrue(problems and any(expected in p for p in problems), (label, problems[:3]))
        self.edit('out/packet.json', lambda d: d.update(packet))
        self.assertEqual(self.problems(), [])

    def test_each_record_check_finds_its_difference(self):
        def stdout_edited():
            path = self.path('out/%s.jsonl' % COMPLETED)
            with open(path) as fh:
                line = json.loads(fh.read())
            line['rows'] = 1
            with open(path, 'w') as fh:
                fh.write(json.dumps(line) + '\n')
        cases = {
            'a required file missing': (lambda: os.unlink(self.path('baseline-set.json')), 'baseline-set.json is missing'),
            'a launched run without its stderr': (lambda: os.unlink(self.path('out/%s.stderr.txt' % STOPPED)),
                                                  'out/%s.stderr.txt is missing' % STOPPED),
            'a file planted for a never-launched run': (
                lambda: open(self.path('out/%s.jsonl' % NEVER_LAUNCHED[2]), 'w').close(),
                'exists for a run that was never launched'),
            'the launch record differs': (lambda: self.edit('out/%s.record.json' % STOPPED,
                                                            lambda d: d.update(wall_s=999.0)), 'launch.wall_s differs'),
            'the stdout line edited': (stdout_edited, 'is not launch.stdout_sha256'),
            'the last stage differs': (lambda: self.records(lambda by, rows: by[STOPPED]['last_stage'].update(
                stage='read')), 'last_stage or last_stage_memory differs'),
            'the model hash differs': (lambda: self.edit('plan.json', lambda d: d['runs'][0].update(
                expected_model_sha256='0' * 64)), 'model_sha256'),
            'a stage second differs': (lambda: self.records(lambda by, rows: by[COMPLETED]['driver']['seconds'].update(
                parse=9.0)), "is not one line equal to the aggregate's driver"),
            'a driver for a run with no line': (lambda: self.records(lambda by, rows: by[STOPPED].update(driver={'x': 1})),
                                                'no driver line, but the aggregate has a driver'),
            'a record missing': (lambda: self.records(lambda by, rows: rows.remove(by[NEVER_LAUNCHED[0]])),
                                 'one record for each run of plan.json'),
        }
        for label, (change, expected) in cases.items():
            self.tearDown()
            self.setUp()
            change()
            problems = self.problems()
            self.assertTrue(any(expected in p for p in problems), (label, problems[:3]))
        # The total of an ok run's stages, and the revision.
        self.tearDown()
        self.setUp()
        self.records(lambda by, rows: by[COMPLETED]['driver']['seconds'].update(
            total=by[COMPLETED]['driver']['seconds']['total'] + 1))
        with open(self.path('out/%s.jsonl' % COMPLETED), 'w') as fh:
            fh.write(json.dumps(next(r for r in rr.read_set_records(self.path('out'))[0]
                                     if r['run_id'] == COMPLETED)['driver']) + '\n')
        self.records(lambda by, rows: by[COMPLETED]['launch'].update(stdout_sha256=None))
        self.edit('out/%s.record.json' % COMPLETED, lambda d: d.update(stdout_sha256=None))
        self.assertTrue(any('total is not the sum' in p for p in self.problems()), self.problems()[:3])
        self.tearDown()
        self.setUp()
        problems = rr.read_back(self.bundle, '0' * 40, self.root, allow_unreleased_build=True)
        self.assertEqual(problems, ['metadata.json names %s, not the revision %s' % (self.revision, '0' * 40)])
        with redirect_stdout(io.StringIO()) as printed:
            self.assertEqual(rr.main(['--read-back', self.bundle, '--revision', '0' * 40, '--root-digest', self.root,
                                      '--allow-unreleased-build']), 1)
        self.assertIn('DIFFERS: metadata.json names', printed.getvalue())


class ReadBackBinding(BundleCase, unittest.TestCase):
    """`--read-back`: the plan of this run (F1), the inventory's binding of every file consumed (F4), a
    never-launched run's absent stage and each stage's memory (F5), the launch keys, the required values,
    the recomputed results, on the fixture's forms/ folder."""

    def test_the_intact_bundle_has_no_difference(self):
        self.assertEqual(self.problems(), [])

    def expect(self, change, expected, inventory=True):
        self.tearDown()
        self.setUp()
        change()
        problems = self.problems() if inventory else rr.read_back(self.bundle, self.revision, root_digest(self.bundle), allow_unreleased_build=True)
        self.assertTrue(any(expected in p for p in problems), (expected, problems[:4]))

    def test_the_plan_is_the_plan_the_records_and_metadata_were_written_from(self):
        """F1. Each plan-entry field the supervisor copies into a record (supervise.rs 181-211, 1135-1145),
        the run ids in order, and the plan's schema, window and quiet wait (1117-1123)."""
        def plan_field(key):
            def change():
                def edit(d):
                    run = d['runs'][0]
                    run[key] = (not run[key]) if isinstance(run[key], bool) else (
                        run[key] + 1 if isinstance(run[key], int) else run[key] + '-other')
                self.edit('plan.json', edit)
            return change
        for key in rr.PLAN_RUN_KEYS + ('model',):
            if key == 'run_id':
                continue
            self.expect(plan_field(key), "%s: %s in the aggregate is not plan.json's" % (COMPLETED, key))
        self.expect(lambda: self.edit('plan.json', lambda d: d['runs'].reverse()),
                    'metadata.json plan is not the run ids of plan.json in their order')
        self.expect(lambda: self.edit('plan.json', lambda d: d['runs'].reverse()),
                    'the aggregate does not hold one record for each run of plan.json, in its order')
        self.expect(lambda: self.edit('plan.json', lambda d: d.update(schema='changed')), 'plan.json schema is')
        self.expect(lambda: self.edit('plan.json', lambda d: d.update(window_budget_s=d['window_budget_s'] + 1)),
                    'metadata.json budget_s')
        self.expect(lambda: self.edit('plan.json', lambda d: d.update(quiet_wait_s=d['quiet_wait_s'] + 1)),
                    'metadata.json quiet_wait_s')

        def metadata_plan_in_both_copies():
            self.edit('out/metadata.json', lambda d: d.update(plan=['some-other-run']))
            self.edit('out/packet.json', lambda d: d['metadata'].update(plan=['some-other-run']))
        self.expect(metadata_plan_in_both_copies, 'metadata.json plan is not the run ids of plan.json in their order')

    def test_every_file_consumed_is_a_regular_file_bound_by_the_inventory(self):
        """F4. The command reads SHA256SUMS itself."""
        outside = tempfile.TemporaryDirectory()
        self.addCleanup(outside.cleanup)

        def plan_linked_outside(inventoried):
            def change():
                target = os.path.join(outside.name, 'plan.json')
                shutil.copyfile(self.path('plan.json'), target)
                os.unlink(self.path('plan.json'))
                os.symlink(target, self.path('plan.json'))
                self.inventory()
                if inventoried:   # the link's bytes are listed, as if the inventory had followed it
                    with open(target, 'rb') as fh, open(self.path('SHA256SUMS'), 'a') as sums:
                        sums.write('%s  ./plan.json\n' % hashlib.sha256(fh.read()).hexdigest())
            return change
        self.expect(plan_linked_outside(False), 'plan.json is not in SHA256SUMS, so its bytes are not bound', False)
        self.expect(plan_linked_outside(True), 'plan.json is reached through a symbolic link (plan.json)', False)

        def folder_linked():
            moved = os.path.join(outside.name, 'out')
            shutil.move(self.path('out'), moved)
            os.symlink(moved, self.path('out'))
            lines = []   # every file listed through the link, as if the inventory had followed it
            for name in sorted(self.names):
                with open(self.path(name), 'rb') as fh:
                    lines.append('%s  ./%s\n' % (hashlib.sha256(fh.read()).hexdigest(), name))
            with open(self.path('SHA256SUMS'), 'w') as fh:
                fh.writelines(lines)
        self.expect(folder_linked, 'out/metadata.json is reached through a symbolic link (out)', False)

        def changed_after_inventory():
            self.inventory()
            self.edit('out/packet.json', lambda d: d.update(window_elapsed_s=-1))
        self.expect(changed_after_inventory, 'out/packet.json does not have the SHA-256 SHA256SUMS states for it', False)

        def left_out_of_inventory():
            self.inventory()
            with open(self.path('SHA256SUMS')) as fh:
                kept = [line for line in fh if not line.endswith('./out/records.jsonl\n')]
            with open(self.path('SHA256SUMS'), 'w') as fh:
                fh.writelines(kept)
        self.expect(left_out_of_inventory, 'out/records.jsonl is not in SHA256SUMS, so its bytes are not bound', False)
        self.expect(lambda: os.unlink(self.path('SHA256SUMS')), 'SHA256SUMS is missing', False)

        def malformed():
            with open(self.path('SHA256SUMS'), 'a') as fh:
                fh.write('not a line\n')
        self.expect(malformed, 'SHA256SUMS line', False)

    def test_the_root_digest_binds_the_inventory_and_the_inventory_names_only_the_bundles_files(self):
        """The command verifies SHA256SUMS against the root digest stated outside the bundle; a file listed
        there that is no file of the bundle (README step 8) or of a launched run is a difference; a file the
        worker examines by hand (exclusive.log) is bound too, where present."""
        self.assertEqual(rr.read_back(self.bundle, self.revision, '0' * 64, allow_unreleased_build=True),
                         ['SHA256SUMS does not have the root digest %s; nothing in the bundle is bound' % ('0' * 64)])

        def orphan():
            with open(self.path('out/unrelated-old-run.record.json'), 'w') as fh:
                fh.write('{"wall_s": 999}\n')
            self.names.append('out/unrelated-old-run.record.json')
        self.expect(orphan, 'out/unrelated-old-run.record.json is in SHA256SUMS but is no file of the bundle')

        def exclusive_changed_after_inventory():
            with open(self.path('exclusive.log'), 'w') as fh:
                fh.write('wrapper lines\n')
            self.names.append('exclusive.log')
            self.inventory()
            with open(self.path('exclusive.log'), 'w') as fh:
                fh.write('other lines\n')
        self.expect(exclusive_changed_after_inventory, 'exclusive.log does not have the SHA-256 SHA256SUMS states', False)
        with redirect_stdout(io.StringIO()), mock.patch('sys.stderr', io.StringIO()) as err, \
                self.assertRaises(SystemExit):
            rr.main(['--read-back', self.bundle, '--revision', self.revision])
        self.assertIn('--root-digest', err.getvalue())

    def test_every_launch_key_the_supervisor_writes_is_present_even_as_null(self):
        """F5 in full: a key absent where the producer writes it, even as null, is a difference."""
        self.assertIsNone(self.load('out/%s.record.json' % COMPLETED)['classification_detail'])
        self.expect(lambda: self.edit('out/%s.record.json' % COMPLETED, lambda d: d.pop('classification_detail')),
                    '%s: launch.classification_detail is missing from its .record.json' % COMPLETED)
        self.expect(lambda: self.records(lambda by, rows: by[COMPLETED]['launch'].pop('kill_after_s')),
                    "%s: the aggregate's launch does not carry exactly the supervisor's launch keys" % COMPLETED)

    def test_an_unreleased_build_is_a_difference_unless_explicitly_allowed(self):
        """Contract section 4 item 4, section 11 item 2, section 14.6 step 1: by default a build identity
        that is not clean and release is a difference; --allow-unreleased-build allows it and says so."""
        self.assertEqual(self.load('out/metadata.json')['build_profile'], 'debug')
        self.inventory()
        self.assertEqual(rr.read_back(self.bundle, self.revision, self.root),
                         ['metadata.json build_profile is \'debug\', not "release"'])

        def identity(dirty, profile):
            self.edit('out/metadata.json', lambda d: d.update(source_dirty=dirty, build_profile=profile))
            self.edit('out/packet.json', lambda d: d['metadata'].update(source_dirty=dirty, build_profile=profile))
            self.inventory()
        identity(True, 'release')
        self.assertEqual(rr.read_back(self.bundle, self.revision, self.root),
                         ['metadata.json source_dirty is True, not false: the build is not of a clean checkout'])
        identity('unknown', 'release')
        self.assertIn("metadata.json source_dirty is 'unknown'", rr.read_back(self.bundle, self.revision, self.root)[0])
        for dirty, profile in ((True, 'release'), (False, 'debug'), (True, 'debug')):
            identity(dirty, profile)
            self.assertEqual(rr.read_back(self.bundle, self.revision, self.root, allow_unreleased_build=True), [])
            with redirect_stdout(io.StringIO()) as printed:
                self.assertEqual(rr.main(['--read-back', self.bundle, '--revision', self.revision, '--root-digest',
                                          self.root, '--allow-unreleased-build']), 0)
            self.assertIn(rr.UNRELEASED_BUILD_ALLOWED, printed.getvalue())
        # A clean, release build reads clean without the allowance, and then the line is absent.
        identity(False, 'release')
        with redirect_stdout(io.StringIO()) as printed:
            self.assertEqual(rr.main(['--read-back', self.bundle, '--revision', self.revision, '--root-digest', self.root]), 0)
        self.assertEqual(printed.getvalue(), 'read-back: 0 difference(s)\n%s\n' % rr.CLEAN_READ_BACK_LIMIT)
        self.assertNotIn(rr.UNRELEASED_BUILD_ALLOWED, printed.getvalue())

    def test_a_required_value_nulled_in_every_copy_is_a_difference(self):
        """REQUIRED_BY_BRANCH applied by the read-back itself: a value a rule reads, nulled consistently in the
        aggregate and the launch record (the packet does not carry it), is still a difference."""
        def nulled():
            self.records(lambda by, rows: by[COMPLETED]['launch'].update(killed_by_rss_watchdog=None))
            self.edit('out/%s.record.json' % COMPLETED, lambda d: d.update(killed_by_rss_watchdog=None))
        self.expect(nulled, '%s: a completed record has no value at launch.killed_by_rss_watchdog' % COMPLETED)

    def test_a_null_entry_peak_on_a_completed_record_reads_clean_outside_the_s5_plan(self):
        """E nulled in every copy that carries it (the aggregate's driver and the stdout line, with its hash):
        no difference, since no baseline rule reads it and the stage-line agreement needs both values."""
        def entry_peak_null():
            path = self.path('out/%s.jsonl' % COMPLETED)

            def null(by, rows):
                by[COMPLETED]['driver']['peak_memory']['entry']['lifetime_max_phys_footprint_bytes'] = None
                with open(path, 'w') as fh:
                    fh.write(json.dumps(by[COMPLETED]['driver']) + '\n')
                with open(path, 'rb') as fh:
                    digest = hashlib.sha256(fh.read()).hexdigest()
                by[COMPLETED]['launch']['stdout_sha256'] = digest
                self.edit('out/%s.record.json' % COMPLETED, lambda d: d.update(stdout_sha256=digest))
            self.records(null)
        self.tearDown()
        self.setUp()
        entry_peak_null()
        self.assertEqual(self.problems(), [])

    def test_baseline_set_json_is_recomputed_from_the_records(self):
        """The two results recomputed from the records the command has just checked, against the stored file."""
        def results_changed():
            stored = self.load('baseline-set.json')
            stored['results'][0] = 'TEN-RUN TIMING SET: COMPLETE'
            with open(self.path('baseline-set.json'), 'w') as fh:
                json.dump(stored, fh)
        self.expect(results_changed, 'baseline-set.json is not the two results --baseline-set gives')
        self.expect(lambda: self.records(lambda by, rows: by[STOPPED].update(classification_detail='after_2_s')),
                    'baseline-set.json is not the two results --baseline-set gives')
        self.expect(lambda: self.edit('baseline-set.json', lambda d: d.update(subset='window')),
                    "baseline-set.json names no named plan (subset 'window')")

    def test_a_never_launched_run_carries_no_last_stage(self):
        """F5 (a): a skip, a missing or a mismatched model has no stage (supervise.rs 1146-1162, 1173-1190)."""
        stage = {'repeat': 0, 'stage': 'parse_back', 'seconds': 0.1}
        self.expect(lambda: self.records(lambda by, rows: by[NEVER_LAUNCHED[2]].update(last_stage=stage)),
                    '%s: a run never launched carries a last stage' % NEVER_LAUNCHED[2])

    def test_each_stage_line_carries_the_drivers_memory_for_that_stage(self):
        """F5 (b): each stderr stage line's readings against the driver's `peak_memory.<stage>`."""
        def first_stage_memory():
            path = self.path('out/%s.stderr.txt' % COMPLETED)
            with open(path) as fh:
                lines = fh.read().splitlines(True)
            index = next(i for i, line in enumerate(lines) if line.startswith(rr.PROGRESS_PREFIX))
            reading = json.loads(lines[index][len(rr.PROGRESS_PREFIX):])
            reading['lifetime_max_phys_footprint_bytes'] = 1
            lines[index] = rr.PROGRESS_PREFIX + json.dumps(reading) + '\n'
            with open(path, 'w') as fh:
                fh.writelines(lines)
        self.expect(first_stage_memory, '%s: stage read lifetime_max_phys_footprint_bytes differs from driver.peak_memory'
                    % COMPLETED)

    def test_each_stage_lines_seconds_are_the_drivers(self):
        """The stderr stage seconds alone, the aggregate and stdout left as they are."""
        def first_stage_seconds():
            path = self.path('out/%s.stderr.txt' % COMPLETED)
            with open(path) as fh:
                lines = fh.read().splitlines(True)
            index = next(i for i, line in enumerate(lines) if line.startswith(rr.PROGRESS_PREFIX))
            reading = json.loads(lines[index][len(rr.PROGRESS_PREFIX):])
            reading['seconds'] += 1.0
            lines[index] = rr.PROGRESS_PREFIX + json.dumps(reading) + '\n'
            with open(path, 'w') as fh:
                fh.writelines(lines)
        self.expect(first_stage_seconds, '%s: stage read seconds differ from driver.seconds' % COMPLETED)

    def test_an_ok_run_has_a_number_for_every_stage_and_the_total(self):
        """A stage missing from driver.seconds in every copy (aggregate, stdout, its stdout hash) and from the
        stage lines: only the numeric-presence check sees it."""
        def stage_missing():
            path = self.path('out/%s.jsonl' % COMPLETED)

            def drop(by, rows):
                by[COMPLETED]['driver']['seconds'].pop('serialize')
                with open(path, 'w') as fh:
                    fh.write(json.dumps(by[COMPLETED]['driver']) + '\n')
                with open(path, 'rb') as fh:
                    digest = hashlib.sha256(fh.read()).hexdigest()
                by[COMPLETED]['launch']['stdout_sha256'] = digest
                self.edit('out/%s.record.json' % COMPLETED, lambda d: d.update(stdout_sha256=digest))
            self.records(drop)
            stderr = self.path('out/%s.stderr.txt' % COMPLETED)
            with open(stderr) as fh:
                kept = [line for line in fh if '"stage":"serialize"' not in line.replace(' ', '')]
            with open(stderr, 'w') as fh:
                fh.writelines(kept)
        self.expect(stage_missing, '%s: driver.seconds lacks a number for a stage or the total' % COMPLETED)


class RequiredValuesByBranch(unittest.TestCase):
    """REQUIRED_BY_BRANCH: for each record form, a value a decision rule reads is required (nulled, a
    difference), and a null the producer leaves where no rule reads it is legitimate (not a difference)."""

    @classmethod
    def setUpClass(cls):
        tmp = tempfile.TemporaryDirectory()
        extract_fixture(tmp.name, 'forms')
        cls.real = {r['run_id']: r for r in rr.read_set_records(os.path.join(tmp.name, 'out'))[0]}
        tmp.cleanup()

    def forms(self):
        """A record of each branch: the fixture's own where it holds one, else the producer's shape for it
        (main.rs, classify.rs) built from the fixture's completed or stopped record."""
        completed, timed_out, sampler = (copy.deepcopy(self.real[k]) for k in (COMPLETED, STOPPED, STOPPED_W2))
        refused = dict(copy.deepcopy(completed), classification='refused', classification_detail='SUPPORT_INPUT_MISSING',
                       as_expected=False)
        entry_error = dict(copy.deepcopy(completed), classification='entry_error', classification_detail='entry: synthetic',
                           as_expected=False)
        read_error = dict(copy.deepcopy(timed_out), classification='entry_error', classification_detail='read: synthetic',
                          as_expected=False, last_stage=None, last_stage_memory=None)
        at_result = dict(copy.deepcopy(completed), classification='exceeds_base_machine_budget', as_expected=False,
                         classification_detail='footprint_1_above_budget_at_result')
        rss_cap = dict(copy.deepcopy(timed_out), classification='killed_by_rss_cap', as_expected=False,
                       classification_detail='killed_above_rss_cap_1_footprint_unread')
        rss_cap['launch'] = dict(rss_cap['launch'], killed_by_rss_watchdog=True)
        error = dict(copy.deepcopy(timed_out), classification='error', classification_detail='exit_code_2', as_expected=False)
        return {'completed': completed, 'timed_out': timed_out, 'killed_by_sampler': sampler, 'refused': refused,
                'entry_error': entry_error, 'entry_error at read': read_error, 'over_budget_at_result': at_result,
                'killed_by_rss_cap': rss_cap, 'error': error,
                'never_launched': copy.deepcopy(self.real[NEVER_LAUNCHED[2]])}

    def test_the_required_values_are_the_decision_input_tables(self):
        """REQUIRED_BY_BRANCH mirrors the decision-input table's "Required" column, branch by branch."""
        launched = {'classification', 'as_expected', 'model_sha256', 'timeout_applied_s', 'timeout_capped_by_window',
                    'host.quiet', 'launch.killed_by_sampler', 'launch.killed_by_rss_watchdog'}
        stages = {'driver.seconds.' + s for s in rr.SOLVE_STAGES + ('total',)}
        self.assertEqual({k: set(v) for k, v in rr.REQUIRED_BY_BRANCH.items()}, {
            'completed': launched | stages | {
                'driver.route', 'driver.document.sha256', 'driver.output_sha256', 'driver.rows', 'driver.output_bytes',
                'launch.rss.exit_lifetime_max_phys_footprint_bytes', 'last_stage'},
            'refused': launched | {'classification_detail', 'last_stage'},
            'entry_error': launched | {'classification_detail', 'last_stage'},
            'killed_by_sampler': launched | {'classification_detail'},
            'over_budget_at_result': launched | {'classification_detail', 'last_stage'},
            'killed_by_rss_cap': launched | {'classification_detail'},
            'timed_out': launched | {'classification_detail'},
            'error': launched | {'classification_detail'},
            'never_launched': {'classification', 'classification_detail'},
        })

    def test_e_and_s_are_required_on_a_completed_record_in_the_s5_plan_only(self):
        """Section 14.5's E and S are read by K2 in the S5 plan alone: on a baseline plan a null E or S is
        legitimate; through PLAN_REQUIRED_EXTRA (here with a synthetic S5 plan name) it is a difference."""
        self.assertEqual(rr.PLAN_REQUIRED_EXTRA.get('baseline', {}), {})
        self.assertEqual(rr.PLAN_REQUIRED_EXTRA.get('baseline-with-rack', {}), {})
        completed = self.forms()['completed']
        for path in rr.S5_COMPLETED_MEMORY:
            record = copy.deepcopy(completed)
            stage = path.split('.')[2]
            record['driver']['peak_memory'][stage]['lifetime_max_phys_footprint_bytes'] = None
            for plan in (None, 'baseline', 'baseline-with-rack'):
                self.assertEqual(rr.required_value_problems(record, plan), [], (path, plan))
            with mock.patch.dict(rr.PLAN_REQUIRED_EXTRA, {'s5-synthetic': {'completed': rr.S5_COMPLETED_MEMORY}}):
                self.assertEqual(rr.required_value_problems(record, 's5-synthetic'),
                                 ['%s: a completed record has no value at %s, which a decision rule reads'
                                  % (record['run_id'], path)])

    def test_each_form_takes_its_branch_and_is_complete_as_the_producer_writes_it(self):
        for label, record in self.forms().items():
            self.assertEqual(rr.record_branch(record), label.split(' ')[0], label)
            self.assertEqual(rr.required_value_problems(record), [], label)

    def test_a_required_value_nulled_is_a_difference(self):
        for label, record in self.forms().items():
            branch = rr.record_branch(record)
            for path in rr.REQUIRED_BY_BRANCH[branch]:
                if label == 'entry_error at read' and path == 'last_stage':
                    continue
                broken = copy.deepcopy(record)
                *parents, last = path.split('.')
                holder = broken
                for key in parents:
                    holder = holder[key]
                holder[last] = None
                # (Nulling a path that selects the branch moves the record to another branch, which still
                # requires it.)
                selects = ('classification', 'classification_detail', 'launch.killed_by_sampler')
                expected = 'has no value at %s,' % path if path in selects else \
                    'a %s record has no value at %s,' % (branch, path)
                self.assertIn(expected, ' '.join(rr.required_value_problems(broken)), (label, path))
                del holder[last]
                self.assertTrue(rr.required_value_problems(broken), (label, path, 'missing key'))

    def test_a_null_the_producer_leaves_where_no_rule_reads_it_is_not_a_difference(self):
        """The reader's co-null cases among them: a refused run's route, model counts, stage seconds, output
        hash, blocking codes and nonlinear iterations; an entry error's route, counts, seconds and status; a
        completed run's nonlinear iterations; a stopped run's driver and last stage."""
        legitimate = {
            'refused': ('driver.route', 'driver.nodes', 'driver.seconds', 'driver.output_sha256', 'driver.blocking_codes',
                        'driver.nonlinear_iterations'),
            'entry_error': ('driver.route', 'driver.nodes', 'driver.seconds', 'driver.status'),
            'completed': ('driver.nonlinear_iterations', 'classification_detail', 'host.memorystatus_level'),
            'timed_out': ('driver', 'last_stage', 'last_stage_memory'),
            'killed_by_sampler': ('driver', 'last_stage'),
            'killed_by_rss_cap': ('driver', 'last_stage'),
            'never_launched': ('last_stage', 'timeout_capped_by_window', 'model_sha256'),
        }
        forms = self.forms()
        for label, paths in legitimate.items():
            for path in paths:
                record = copy.deepcopy(forms[label])
                *parents, last = path.split('.')
                holder = record
                for key in parents:
                    holder = holder[key]
                holder[last] = None
                self.assertEqual(rr.required_value_problems(record), [], (label, path))


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

    def test_a_model_checks_report_is_ok(self):
        line = driver(rr.CHECKS_REPORTED, footprint=k6.GIB)
        self.assertEqual(rr.classify(launched(), line, BUDGET), ('ok', None))
        self.assertTrue(rr.as_expected('ok', 'ok', None))
        self.assertEqual(rr.classify(launched(), driver(rr.CHECKS_REPORTED, footprint=BUDGET + 1), BUDGET)[0],
                         'exceeds_base_machine_budget')


class ClassificationReadings(unittest.TestCase):
    """Each record names the footprint readings its classification used, and this runner's reading
    at exit (/usr/bin/time's peak memory footprint) or why it has none."""

    def test_a_completed_run_names_every_reading_it_compared(self):
        progress = {'phys_footprint_bytes': 5, 'lifetime_max_phys_footprint_bytes': 6}
        record = launched(wrapper='/usr/bin/time -l', sampler={'peak_phys_footprint_bytes': 1,
                                                               'peak_lifetime_max_phys_footprint_bytes': 2})
        readings = rr.readings_record(record, driver(), progress)
        self.assertEqual(readings['used'], [
            'sampler:peak_phys_footprint_bytes', 'sampler:peak_lifetime_max_phys_footprint_bytes',
            'rss:time_peak_footprint_bytes', 'driver:driver_peak_footprint_bytes',
            'progress:phys_footprint_bytes', 'progress:lifetime_max_phys_footprint_bytes'])
        self.assertEqual((readings['at_exit'], readings['at_exit_absent']), ('rss:time_peak_footprint_bytes', None))

    def test_a_killed_run_says_it_had_no_reading_at_exit(self):
        # /usr/bin/time is in the killed group: no time_* readings.
        killed = launched('killed_by_rss_watchdog', -9, wrapper='/usr/bin/time -l', rss={'ps_max_kib': 1},
                          sampler={'peak_phys_footprint_bytes': BUDGET - 1,
                                   'peak_lifetime_max_phys_footprint_bytes': BUDGET - 1})
        readings = rr.readings_record(killed, None)
        self.assertEqual(readings['used'], ['sampler:peak_phys_footprint_bytes',
                                            'sampler:peak_lifetime_max_phys_footprint_bytes'])
        self.assertIsNone(readings['at_exit'])
        self.assertIn('killed with its process group', readings['at_exit_absent'])
        self.assertEqual(rr.classify(killed, None, rr.EMERGENCY_RSS_CAP_BYTES, BUDGET)[0], 'killed_by_rss_cap')
        sampled = launched('killed_by_sampler', -9, wrapper='/usr/bin/time -l', rss={},
                           sampler={'kill_footprint_bytes': BUDGET + 1})
        self.assertEqual(rr.readings_record(sampled, None)['used'], ['sampler:kill_footprint_bytes'])
        self.assertIn('killed with its process group', rr.readings_record(sampled, None)['at_exit_absent'])
        self.assertIn('did not run under /usr/bin/time', rr.readings_record(launched(rss={}), driver())['at_exit_absent'])

    def test_no_footprint_decides_a_timeout_or_an_entry_error(self):
        self.assertEqual(rr.classification_readings(launched('timed_out', -9), None), [])
        line = driver('entry_error', entry_error={'stage': 'entry', 'message': 'x'})
        self.assertEqual(rr.classification_readings(launched('error', 3), line), [])
        self.assertEqual(rr.classification_readings(launched('error', 101), None), [])


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


class RunIds(unittest.TestCase):
    """A run id names record files inside the output folder: only bare names are accepted."""

    def test_a_run_id_that_is_not_a_bare_name_is_refused_before_anything_is_written(self):
        for bad in ('../escaped', 'a/b', '.hidden', '', 'x' * 129, 'a b'):
            with tempfile.TemporaryDirectory() as folder:
                plan = rr.build_plan((('linear-ex', 100, 60, 1, 'solved'),))
                plan[0]['run_id'] = bad
                out = os.path.join(folder, 'out')
                with self.assertRaises(ValueError) as refused:
                    rr.run_plan(plan, folder, out, '/bin/false', launch=None, quiet=None, log=lambda *a: None, md={})
                self.assertIn('bare names', str(refused.exception))
                self.assertEqual(os.listdir(folder), [], 'nothing written, inside or outside the output folder')
                with self.assertRaises(ValueError):
                    rr.plan_document(plan)
        for good in ('linear-ex-n2000-sparse', 'model-checks-linear-ex-n1000', 'a.b_c-1'):
            rr.check_run_ids([{'run_id': good}])

    def test_run_ids_that_would_name_one_file_twice_are_refused_before_anything_is_written(self):
        # The third review's cases: the run id `records` made its stdout record the aggregate
        # record, and `Case` and `case` were one set of files on a case-insensitive volume.
        cases = {('records',): 'records.jsonl', ('Records',): 'records.jsonl', ('Case', 'case'): 'same ignoring case'}
        for name in rr.FOLDER_NAMES:
            cases[(name,)] = 'a name the runner writes'
            cases[(name.upper(),)] = 'a name the runner writes'
        for ids, message in cases.items():
            with tempfile.TemporaryDirectory() as folder:
                plan = rr.build_plan(tuple(('linear-ex', 100 + i, 60, 1, 'solved') for i in range(len(ids))))
                for r, run_id in zip(plan, ids):
                    r['run_id'] = run_id
                with self.assertRaises(ValueError) as refused:
                    rr.run_plan(plan, folder, os.path.join(folder, 'out'), '/bin/false', launch=None, quiet=None,
                                log=lambda *a: None, md={})
                self.assertIn(message, str(refused.exception), ids)
                self.assertEqual(os.listdir(folder), [], 'nothing written')
                with self.assertRaises(ValueError):
                    rr.plan_document(plan)
        # The default plan's ids, and every pair of suffix-shaped ids, name distinct files.
        rr.check_run_ids(rr.default_plan())
        rr.check_run_ids([{'run_id': i} for i in ('x', 'x.record', 'x.stderr', 'x.time', 'x.jsonl', 'x.json', 'x.txt',
                                                   'x.sample-000', 'x.result')])


class PacketAggregation(unittest.TestCase):
    def test_a_record_of_an_earlier_driver_is_refused_by_name(self):
        # A record written with a release-timing-v1 driver line (the side stage form) is not
        # repackaged under the current conditions.
        with tempfile.TemporaryDirectory() as out:
            old = driver(schema='release-timing-v1', side_stages=['model_checks'])
            k6.append_record(out, {'schema': rr.RECORD_SCHEMA, 'run_id': 'old-run', 'classification': 'ok',
                                   'launch': launched(), 'driver': old})
            with self.assertRaises(ValueError) as refused:
                rr.packet(out)
            self.assertEqual(str(refused.exception), "record old-run: its driver line has schema release-timing-v1, "
                             "not release-timing-v2; an earlier driver's record is not repackaged")
            err = io.StringIO()
            with mock.patch('sys.stderr', err):
                self.assertEqual(rr.main(['--packet', '--out', out]), 2)
            self.assertIn('not repackaged', err.getvalue())
            self.assertFalse(os.path.exists(os.path.join(out, 'packet.json')))

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
        # The runner's own fixed labels are not host data; a host or user name such as a CI
        # runner's "runner" would otherwise match them.
        host_text = json.dumps({k: v for k, v in md.items() if k not in ('runner', 'driver_schema')})
        for identifier in {socket.gethostname(), platform.node(), getpass.getuser()} - {''}:
            self.assertNotIn(identifier, host_text)
        for key in md:
            self.assertNotIn(key, ('hostname', 'node', 'user', 'serial', 'path'))
        for value in text.split('"'):
            self.assertFalse(value.startswith('/'), value)
        self.assertEqual(md['binary_sha256'], hashlib.sha256(b'driver bytes').hexdigest())
        self.assertEqual(md['base_machine_budget_bytes'], BUDGET)
        self.assertFalse(md['source_dirty'])

    def test_a_dirty_flag_git_cannot_establish_is_unknown_never_false(self):
        def fake(answers):
            return lambda args: answers.get(args[-1] if args[0] == 'rev-parse' else args[0])
        clean = {'HEAD': 'a' * 40, 'HEAD^{tree}': 'b' * 40, 'status': ''}
        self.assertEqual(rr.source_identity(fake(clean)), ('a' * 40, 'b' * 40, False))
        self.assertIs(rr.source_identity(fake(dict(clean, status=' M core/x.rs')))[2], True)
        self.assertEqual(rr.source_identity(fake(dict(clean, status=None)))[2], 'unknown')
        self.assertEqual(rr.source_identity(fake({'status': ''}))[2], 'unknown', 'no commit: not established')
        self.assertEqual(rr.source_identity(lambda args: None), (None, None, 'unknown'))


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
                   'print(json.dumps({"schema": "%s", "status": "MECHANICS_SOLVED", '
                   '"blocking_codes": [], "rows": 3, "argv_tail": sys.argv[2:], '
                   '"driver_peak_footprint_bytes": 1}))\n' % rr.DRIVER_SCHEMA)


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

    def test_each_recorded_hash_is_its_files_at_the_end_of_a_multi_run_plan(self):
        with tempfile.TemporaryDirectory() as folder:
            binary = executable(folder, 'release_timing', STAND_IN_DRIVER)
            models = self.models(folder)
            with open(os.path.join(models, rr.model_name('nonlinear-pf', 200)), 'w') as fh:
                fh.write('{}')
            plan = rr.build_plan((('nonlinear-pf', 100, 60, 1, 'solved'), ('nonlinear-pf', 200, 60, 1, 'solved')),
                                 rss_cap_bytes=128 * k6.MIB)
            out = os.path.join(folder, 'out')
            records = rr.run_plan(plan, models, out, binary, quiet=lambda log, wait: {'quiet': True},
                                  log=lambda *a: None, md={})
            self.assertEqual(len(records), 2)
            for r in records:
                with open(os.path.join(out, r['run_id'] + k6.STDOUT_SUFFIX), 'rb') as fh:
                    self.assertEqual(r['launch']['stdout_sha256'], hashlib.sha256(fh.read()).hexdigest(), r['run_id'])

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
                'print(json.dumps({"schema": "%s", "status": "MECHANICS_SOLVED", '
                '"blocking_codes": [], "driver_peak_footprint_bytes": 1}))\n' % rr.DRIVER_SCHEMA)
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
