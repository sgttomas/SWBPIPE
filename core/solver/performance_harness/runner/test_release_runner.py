#!/usr/bin/env python3
"""Release timing runner tests (DEL-04-05, RT-U4): standard library `unittest`.

The pure tests need no child process and no Rust binary. `LiveLaunch` runs two tiny
self-limiting Python children through the runner's own run path: a stand-in driver that prints
one driver line (`rr.DRIVER_SCHEMA`), and (macOS) a growing child under a 128 MiB cap with a failsafe
exit at 512 MiB. Nothing measures or asserts a time or memory bound.

Run from this directory: python3 -m unittest test_release_runner
"""
import getpass
import hashlib
import io
import json
import os
import platform
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
    for name in os.listdir(_PINNED[0].name):
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


class BaselineSet(unittest.TestCase):
    """`--baseline-set`: the verdict on one supervised folder of a named plan, from the records' fields."""

    def judge(self, mutate=None, name='baseline'):
        plan, _, _ = rr.named_plan(name)
        runs, records = set_records(plan)
        by_id = {r['run_id']: r for r in records}
        if mutate:
            mutate(by_id, records, runs)
        return rr.judge_set(records, runs)

    def test_complete_gives_each_workloads_median_minimum_and_maximum(self):
        report = self.judge()
        self.assertEqual((report['verdict'], report['exit_code']), ('COMPLETE', rr.SET_COMPLETE))
        w1 = report['workloads']['baseline-one-case-ex n2000']
        self.assertEqual((w1['planned'], w1['completed']), (5, 5))
        self.assertEqual((w1['median_entry_s'], w1['min_entry_s'], w1['max_entry_s']), (1.04, 1.0, 1.08))
        self.assertEqual(report['workloads']['reference-system n27']['completed'], 5)
        self.assertEqual((report['findings'], report['window_problems']), ([], []))
        self.assertEqual(report['host_not_quiet'], [])
        # A host check that was not quiet, or read no memory level, is reported and leaves the verdict as it is.
        report = self.judge(lambda by, *_: (by['reference-system-n0027-sparse-r1'].update(host={'quiet': False}),
                                            by['reference-system-n0027-sparse-r2'].update(host={'quiet': True})))
        self.assertEqual((report['verdict'], report['host_not_quiet']),
                         ('COMPLETE', ['reference-system-n0027-sparse-r1', 'reference-system-n0027-sparse-r2']))
        # A run whose timeout the window capped but which completed is unaffected.
        report = self.judge(lambda by, *_: by['reference-system-n0027-sparse-r5'].update(
            timeout_capped_by_window=True, timeout_applied_s=40))
        self.assertEqual(report['verdict'], 'COMPLETE')

    def test_the_rack_run_is_reported_and_never_a_timing(self):
        report = self.judge(name='baseline-with-rack')
        self.assertEqual(report['verdict'], 'COMPLETE')
        self.assertEqual(report['not_timings'], [{'run_id': 'linear-ex-n2000-sparse', 'expect': 'exceeds_base_machine_budget',
                                                  'classification': 'exceeds_base_machine_budget',
                                                  'classification_detail': None, 'last_stage': None,
                                                  'as_expected': True}])
        self.assertEqual(sorted(report['workloads']), ['baseline-one-case-ex n2000', 'reference-system n27'])

    def test_a_skipped_or_capped_and_timed_out_run_is_incomplete_by_the_window(self):
        def window(by, *_):
            by['reference-system-n0027-sparse-r5'].update(classification='skipped_window_budget', driver=None)
            by['baseline-one-case-ex-n2000-sparse-r5'].update(classification='timed_out', timeout_applied_s=250,
                                                               timeout_capped_by_window=True, driver=None)
        report = self.judge(window)
        self.assertEqual((report['verdict'], report['exit_code']), ('INCOMPLETE BY THE WINDOW', 3))
        self.assertEqual(report['findings'], [])
        self.assertEqual(len(report['window_problems']), 2)
        self.assertIn('baseline-one-case-ex-n2000-sparse-r5: timed_out under a timeout the window capped to 250 s',
                      report['window_problems'])
        self.assertEqual(report['workloads']['baseline-one-case-ex n2000']['completed'], 4)

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
            self.assertEqual((report['verdict'], report['exit_code']), ('FINDINGS', 1), label)
            self.assertTrue(report['findings'], label)
            self.assertEqual(report['window_problems'], [], label)
        report = self.judge(cases['full timeout'])
        self.assertIn('timed out at its full timeout', report['findings'][0])
        report = self.judge(cases['missing record'])
        self.assertIn('baseline-one-case-ex-n2000-sparse-r1: no record', report['findings'][0])

    def test_a_finding_with_a_window_problem_is_findings_and_lists_both(self):
        def both(by, *_):
            by['reference-system-n0027-sparse-r5'].update(classification='skipped_window_budget', driver=None)
            by['baseline-one-case-ex-n2000-sparse-r2']['driver'].update(route='PF')
        report = self.judge(both)
        self.assertEqual((report['verdict'], report['exit_code']), ('FINDINGS', 1))
        self.assertEqual(report['window_problems'], ['reference-system-n0027-sparse-r5: skipped_window_budget'])
        self.assertIn('driver.route', report['findings'][0])

    def test_the_cli_reads_one_folder_writes_nothing_and_holds_the_plan_to_the_named_plan(self):
        plan, _, _ = rr.named_plan('baseline')
        hashes = rr.recorded_rack_hashes()
        with open(rr.COMMITTED_MODELS['reference-system']['source'], 'rb') as fh:
            reference = hashlib.sha256(fh.read()).hexdigest()
        runs, records = set_records(plan, {'baseline-one-case-ex-n2000.json': hashes[('baseline-one-case-ex', 2000)],
                                           'reference_system.json': reference})
        with tempfile.TemporaryDirectory() as folder:
            out = os.path.join(folder, 'out')
            os.makedirs(out)
            with open(os.path.join(out, k6.RECORDS_NAME), 'w') as fh:
                fh.writelines(json.dumps(r) + '\n' for r in records)
            plan_path = os.path.join(folder, 'plan.json')
            with open(plan_path, 'w') as fh:
                json.dump({'schema': rr.PLAN_SCHEMA, 'window_budget_s': 2160, 'quiet_wait_s': 60, 'runs': runs}, fh)
            before = {name: os.path.getmtime(os.path.join(out, name)) for name in os.listdir(out)}
            argv = ['--baseline-set', out, '--plan-file', plan_path, '--subset', 'baseline']
            with redirect_stdout(io.StringIO()) as printed:
                self.assertEqual(rr.main(argv), 0)
            self.assertEqual(json.loads(printed.getvalue())['verdict'], 'COMPLETE')
            self.assertEqual({name: os.path.getmtime(os.path.join(out, name)) for name in os.listdir(out)}, before)
            # The same records judged as baseline-with-rack: not that plan, a finding.
            with redirect_stdout(io.StringIO()) as printed:
                self.assertEqual(rr.main(argv[:-1] + ['baseline-with-rack']), 1)
            self.assertIn('is not the named plan baseline-with-rack', json.loads(printed.getvalue())['findings'][0])
            # A plan whose hash is not the repository's own is a finding even where the records agree with it.
            runs[0]['expected_model_sha256'] = records[0]['model_sha256'] = 'e' * 64
            records[0]['driver']['document']['sha256'] = 'e' * 64
            with open(plan_path, 'w') as fh:
                json.dump({'schema': rr.PLAN_SCHEMA, 'window_budget_s': 2160, 'quiet_wait_s': 60, 'runs': runs}, fh)
            with open(os.path.join(out, k6.RECORDS_NAME), 'w') as fh:
                fh.writelines(json.dumps(r) + '\n' for r in records)
            with redirect_stdout(io.StringIO()) as printed:
                self.assertEqual(rr.main(argv), 1)
            self.assertIn("the plan's expected_model_sha256 " + 'e' * 64, printed.getvalue())
            with redirect_stdout(io.StringIO()), mock.patch('sys.stderr', io.StringIO()), self.assertRaises(SystemExit):
                rr.main(['--baseline-set', out, '--plan-file', plan_path, '--subset', 'window'])

    def test_a_run_not_expected_to_solve_must_still_have_run(self):
        rack = 'linear-ex-n2000-sparse'
        not_run = {
            'missing record': lambda by, records, runs: records.remove(by[rack]),
            'model_missing': lambda by, *_: (by[rack].pop('launch'), by[rack].pop('model_sha256'), by[rack].update(
                classification='model_missing', classification_detail='linear-ex-n2000.json', driver=None)),
            'model_mismatch': lambda by, *_: (by[rack].pop('launch'), by[rack].update(
                classification='model_mismatch', model_sha256='d' * 64, driver=None)),
            'error before launch': lambda by, *_: (by[rack].pop('launch'), by[rack].update(classification='error',
                                                                                            driver=None)),
            'other model bytes': lambda by, *_: by[rack].update(model_sha256='d' * 64),
        }
        for label, mutate in not_run.items():
            report = self.judge(mutate, name='baseline-with-rack')
            self.assertEqual((report['verdict'], report['exit_code']), ('FINDINGS', 1), label)
            self.assertIn(rack, report['findings'][0], label)
            self.assertEqual(report['not_timings'], [], label)
        # The same with a window problem among the ten: FINDINGS, and both are listed.
        report = self.judge(lambda by, records, runs: (not_run['model_missing'](by), by[
            'reference-system-n0027-sparse-r5'].update(classification='skipped_window_budget', driver=None)),
            name='baseline-with-rack')
        self.assertEqual((report['verdict'], len(report['findings']), len(report['window_problems'])), ('FINDINGS', 1, 1))
        # Skipped for the window: a window problem.
        report = self.judge(lambda by, *_: by[rack].update(classification='skipped_window_budget', driver=None),
                            name='baseline-with-rack')
        self.assertEqual((report['verdict'], report['window_problems']),
                         ('INCOMPLETE BY THE WINDOW', [rack + ': skipped_window_budget']))
        # Whatever the driver itself did is reported as it is and does not decide the ten's verdict.
        for classification, detail, expected in (
                ('exceeds_base_machine_budget', 'footprint_1_above_budget_2_killed', True),
                ('timed_out', 'after_900_s', False), ('entry_error', 'entry: failed', False),
                ('ok', None, False), ('error', 'exit_code_3', False)):
            report = self.judge(lambda by, *_: by[rack].update(
                classification=classification, classification_detail=detail, as_expected=expected,
                last_stage={'repeat': 0, 'stage': 'request', 'seconds': 0.5}), name='baseline-with-rack')
            self.assertEqual(report['verdict'], 'COMPLETE', classification)
            self.assertEqual(report['not_timings'], [{
                'run_id': rack, 'expect': 'exceeds_base_machine_budget', 'classification': classification,
                'classification_detail': detail, 'last_stage': {'repeat': 0, 'stage': 'request', 'seconds': 0.5},
                'as_expected': expected}])

    def test_absent_output_figures_are_a_finding_even_where_every_run_lacks_them(self):
        def absent(by, records, runs):
            for r in records:
                for key in ('output_sha256', 'rows', 'output_bytes'):
                    r['driver'].pop(key, None)
        report = self.judge(absent)
        self.assertEqual((report['verdict'], report['exit_code']), ('FINDINGS', 1))
        self.assertEqual(len(report['findings']), 10)
        self.assertIn('are not all present', report['findings'][0])

    def test_every_top_level_field_and_run_field_of_the_plan_file_is_the_named_plans(self):
        plan, _, _ = rr.named_plan('baseline')
        hashes = rr.recorded_rack_hashes()
        with open(rr.COMMITTED_MODELS['reference-system']['source'], 'rb') as fh:
            reference = hashlib.sha256(fh.read()).hexdigest()
        runs, records = set_records(plan, {'baseline-one-case-ex-n2000.json': hashes[('baseline-one-case-ex', 2000)],
                                           'reference_system.json': reference})
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
            out = os.path.join(folder, 'out')
            os.makedirs(out)
            with open(os.path.join(out, k6.RECORDS_NAME), 'w') as fh:
                fh.writelines(json.dumps(r) + '\n' for r in records)
            for label, change in cases.items():
                document = json.loads(json.dumps({'schema': rr.PLAN_SCHEMA, 'window_budget_s': 2160,
                                                  'quiet_wait_s': 60, 'runs': runs}))
                change(document)
                path = os.path.join(folder, 'plan.json')
                with open(path, 'w') as fh:
                    json.dump(document, fh)
                report = rr.baseline_set(out, path, 'baseline')
                if label == 'none':
                    self.assertEqual(report['verdict'], 'COMPLETE')
                    continue
                self.assertEqual(report['verdict'], 'FINDINGS', label)
                self.assertIn('is not the named plan baseline', report['findings'][0], label)
                if label in ('window_budget_s', 'quiet_wait_s', 'schema'):
                    self.assertIn('%s differs' % label, report['findings'][0])

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
        self.assertEqual((report['verdict'], report['exit_code']), ('INCOMPLETE BY THE WINDOW', 3), report)
        self.assertTrue(report['window_problems'])


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
