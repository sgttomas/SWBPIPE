#!/usr/bin/env python3
"""K6 runner tests (K6 brief, Required tests B3, B4, C4 and H): standard library `unittest`.

The pure tests need no child process. `LiveLimit` spawns only self-limiting Python
children under a 128 MiB cap (ROOT's K6 ruling Q9(b)); its group-kill test adds a
sleeping helper that exits by itself within 30 s. No test is skipped: each
platform runs its own live limit path (macOS: the RSS watchdog under
/usr/bin/time; Linux: RLIMIT_AS), and any other platform fails.

Run from this directory: python3 -m unittest test_k6_runner
"""
import getpass
import hashlib
import json
import os
import platform
import signal
import socket
import subprocess
import sys
import tempfile
import time
import types
import unittest
from unittest import mock

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import k6_runner as r  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
OBSERVATIONS = os.path.join(HERE, '..', 'observations', 'k6')

# /usr/bin/time -l on macOS (bytes), recorded on the owner's Mac (macOS 26.6.2, arm64) around
# `k6_observe --noop --heap-cap-bytes 8053063680` (the working-tree release build of A2).
TIME_L_FIXTURE = """        0.00 real         0.00 user         0.00 sys
             1785856  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                 292  page reclaims
                   0  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                   0  voluntary context switches
                   8  involuntary context switches
            15263769  instructions retired
             6382372  cycles elapsed
             1147192  peak memory footprint
"""

# GNU /usr/bin/time -v (KiB). Constructed from GNU time's documented format (no
# Linux host is available to K6); disclosed as constructed.
TIME_V_FIXTURE = """\tCommand being timed: "k6_observe --noop --heap-cap-bytes 8053063680"
\tUser time (seconds): 0.00
\tSystem time (seconds): 0.00
\tPercent of CPU this job got: 0%
\tElapsed (wall clock) time (h:mm:ss or m:ss): 0:00.00
\tAverage shared text size (kbytes): 0
\tAverage unshared data size (kbytes): 0
\tAverage stack size (kbytes): 0
\tAverage total size (kbytes): 0
\tMaximum resident set size (kbytes): 2048
\tAverage resident set size (kbytes): 0
\tMajor (requiring I/O) page faults: 0
\tMinor (reclaiming a frame) page faults: 91
\tVoluntary context switches: 1
\tInvoluntary context switches: 0
\tSwaps: 0
\tFile system inputs: 0
\tFile system outputs: 0
\tSocket messages sent: 0
\tSocket messages received: 0
\tSignals delivered: 0
\tPage size (bytes): 4096
\tExit status: 0
"""

# Lines k6_observe printed (RF-LARGE-CHAIN-n00010-ROT, dense), trimmed to the kinds.
JSONL_FIXTURE = [
    '{"kind":"start","schema":"k6-observe-v1","pid":1,"model":"RF-LARGE-CHAIN-n00010-ROT","model_from_file":false,'
    '"mode":"dense","counts_only":false,"repeats":2,"entry_repeats":1,"heap_cap_bytes":536870912,'
    '"time_budget_s":null,"first_repeat_limit_s":600,"allow_over_estimate":false}',
    '{"kind":"stage_begin","repeat":0,"stage":"assembly","heap_current":5671}',
    '{"kind":"stage","repeat":0,"stage":"assembly","ok":true,"elapsed_ns":300,"heap_current_begin":5607,'
    '"heap_current_end":36287,"heap_peak":48127,"heap_peak_move":48127,"alloc_calls":213,"error":null}',
    '{"kind":"stage","repeat":1,"stage":"assembly","ok":true,"elapsed_ns":100,"heap_current_begin":5607,'
    '"heap_current_end":36287,"heap_peak":48655,"heap_peak_move":48655,"alloc_calls":213,"error":null}',
    '{"kind":"stage","repeat":2,"stage":"assembly","ok":true,"elapsed_ns":1000,"heap_current_begin":5607,'
    '"heap_current_end":36287,"heap_peak":48655,"heap_peak_move":48655,"alloc_calls":213,"error":null}',
    '{"kind":"stage","repeat":0,"stage":"prepare","ok":true,"elapsed_ns":50,"heap_current_begin":1,'
    '"heap_current_end":1,"heap_peak":587838,"heap_peak_move":587838,"alloc_calls":2327,"error":null}',
    '{"kind":"stage","repeat":1,"stage":"prepare","ok":true,"elapsed_ns":70,"heap_current_begin":1,'
    '"heap_current_end":1,"heap_peak":588366,"heap_peak_move":588366,"alloc_calls":2327,"error":null}',
    '{"kind":"outcome","repeat":0,"class":"Passed","failed_stage":null,"factor_error":null,"error":null,'
    '"formation_demoted":false,"load_fidelity_flagged":false,"contribution_rounding_rows":0,"recovery_ok":true,'
    '"recovery_digest":"8d6c91ab5c8add75","solution_debug_len":75460,"solution_debug_fnv64":"a5e27ecd90fb4a9d"}',
    '{"kind":"parity","item":"staged_vs_entry_checked","repeat":0,"equal":true,"entry_class":"Passed",'
    '"entry_debug_len":75460,"entry_debug_fnv64":"a5e27ecd90fb4a9d"}',
    '{"kind":"summary","repeats_completed":2,"stop_reason":null,"repeats_heap_peak":605666,'
    '"repeats_heap_peak_move":605666,"counts_phase_heap_peak":null,"counts_phase_heap_peak_move":null,'
    '"parity_phase_heap_peak":73504,"parity_phase_heap_peak_move":73632,"heap_peak":605666,'
    '"heap_peak_move":605666,"elapsed_ns":93515750}',
]

REQUIRED_KEYS = {
    'start': {'schema', 'pid', 'mode', 'repeats', 'entry_repeats', 'heap_cap_bytes'},
    'stage_begin': {'repeat', 'stage', 'heap_current'},
    'stage': {'repeat', 'stage', 'ok', 'elapsed_ns', 'heap_current_begin', 'heap_current_end', 'heap_peak',
              'heap_peak_move', 'alloc_calls', 'error'},
    'outcome': {'repeat', 'class', 'failed_stage', 'solution_debug_len', 'solution_debug_fnv64'},
    'parity': {'item', 'repeat', 'equal'},
    'summary': {'repeats_completed', 'stop_reason', 'repeats_heap_peak', 'repeats_heap_peak_move', 'heap_peak',
                'heap_peak_move'},
}


def read_hash_list(path):
    out = {}
    with open(path) as fh:
        for line in fh:
            if line.startswith('#') or not line.strip():
                continue
            digest, model_id = line.split()[:2]
            out[model_id] = digest
    return out


class ModelHashes(unittest.TestCase):
    """B3: the independent Python generator reproduces models_sha256.txt; the committed small
    canonical models hash to the same lines."""

    def test_python_generator_matches_committed_hashes(self):
        listed = read_hash_list(os.path.join(OBSERVATIONS, 'models_sha256.txt'))
        ids = r.sealed_model_ids() + r.extra_model_ids()
        self.assertEqual(len(r.sealed_model_ids()), 33)
        self.assertEqual(sorted(listed), sorted(ids))
        for model_id in ids:
            self.assertEqual(r.model_sha256(model_id), listed[model_id], model_id)

    def test_committed_models_hash_to_the_list(self):
        listed = read_hash_list(os.path.join(OBSERVATIONS, 'models_sha256.txt'))
        folder = os.path.join(OBSERVATIONS, 'models')
        names = sorted(os.listdir(folder))
        self.assertEqual(len(names), 21)
        for name in names:
            model_id = name[:-len('.k6model')].replace('DEC053_', 'DEC053:', 1)
            with open(os.path.join(folder, name), 'rb') as fh:
                self.assertEqual(hashlib.sha256(fh.read()).hexdigest(), listed[model_id], name)

    def test_k6b_sources_match_the_independent_python_bytes(self):
        """K6b (plan Q6(c)): the Python K4SRC of the independent generator's models equals
        observations/k6b/sources.txt, which the Rust adapter test checks by FNV-1a."""
        import k6b_sources
        path = os.path.join(os.path.dirname(OBSERVATIONS), 'k6b', 'sources.txt')
        with open(path) as fh:
            self.assertEqual(fh.read(), k6b_sources.table())


class SectionBits(unittest.TestCase):
    """B4, independently of the Rust test: the stated formula's bits."""

    def test_section_bits(self):
        section = r.rf_large_section()
        self.assertEqual([r.bits(v) for v in section],
                         ['42474876e8000000', '4232a05f20000000', '3f7872fa3a37ac1b', '3efc526644422115',
                          '3efc526644422115', '3f0c526644422115'])


class CountsClosedForms(unittest.TestCase):
    """C4: every counts line in observations/k6/counts.jsonl against the Python closed forms."""

    def test_counts_match_closed_forms(self):
        counts = r.read_counts(os.path.join(OBSERVATIONS, 'counts.jsonl'))
        ids = r.sealed_model_ids() + r.extra_model_ids()
        self.assertEqual(sorted(counts), sorted(ids))
        for model_id in ids:
            forms = r.closed_forms(r.model(model_id))
            line = counts[model_id]
            for key, value in forms.items():
                self.assertEqual(line[key], value, '%s %s' % (model_id, key))
            self.assertEqual(line['estimate_f1b_bytes_dense'], 96 * forms['dense_entries'], model_id)
            self.assertEqual(line['estimate_f1b_bytes_lane_id'], 24 * line['identity_profile_entries'], model_id)


class Parsers(unittest.TestCase):
    """H5: /usr/bin/time -l (bytes) and -v (KiB), ru_maxrss and ps units."""

    def test_time_l_bytes(self):
        parsed = r.parse_time(TIME_L_FIXTURE, 'Darwin')
        self.assertEqual(parsed['time_max_rss_bytes'], 1785856)
        self.assertEqual(parsed['time_peak_footprint_bytes'], 1147192)

    def test_time_v_kib(self):
        parsed = r.parse_time(TIME_V_FIXTURE, 'Linux')
        self.assertEqual(parsed['time_max_rss_bytes'], 2048 * 1024)
        signalled = r.parse_time(TIME_V_FIXTURE.replace('\tExit status: 0\n', '\tCommand terminated by signal 6\n'),
                                 'Linux')
        self.assertEqual(signalled['time_child_signal'], 6)

    def test_time_v_output_file_is_scrubbed(self):
        # RV18-N2: GNU time -v writes the binary's absolute path ("Command being timed") into
        # the output file, which becomes a record. The path is built at run time.
        machine_path = '/' + '/'.join(['home', 'someone', 'build', 'k6_observe'])
        text = TIME_V_FIXTURE.replace('"k6_observe --noop', '"' + machine_path + ' --noop')
        self.assertIn(machine_path, text)
        with tempfile.TemporaryDirectory() as folder:
            path = os.path.join(folder, 'run.time.txt')
            with open(path, 'w') as fh:
                fh.write(text)
            parsed = r.read_time_file(path, 'Linux')
            with open(path) as fh:
                written = fh.read()
        self.assertEqual(parsed['time_max_rss_bytes'], 2048 * 1024)
        self.assertNotIn(machine_path, written)
        self.assertIn('Command being timed: <omitted>', written)
        self.assertEqual(r.parse_time(written, 'Linux'), parsed)

    def test_ru_maxrss_units(self):
        self.assertEqual(r.ru_maxrss_bytes(4096, 'Darwin'), 4096)
        self.assertEqual(r.ru_maxrss_bytes(4096, 'Linux'), 4096 * 1024)

    def test_ps_is_kib(self):
        self.assertEqual(r.ps_kib_to_bytes(128), 128 * 1024)


class Watchdog(unittest.TestCase):
    """The watchdog's comparison: ps KiB against a cap in bytes (K6-M2's pure kill)."""

    def test_kib_against_bytes(self):
        cap = 128 * r.MIB
        self.assertFalse(r.watchdog_exceeds(128 * 1024, cap))
        self.assertTrue(r.watchdog_exceeds(128 * 1024 + 1, cap))
        self.assertFalse(r.watchdog_exceeds(1000, cap))

    def test_poll_interval_is_the_designs_100_ms(self):
        # RV18-N1: the watchdog polls every 100 ms (DESIGN.md:826). The live test's tolerance,
        # one poll's growth, scales with the interval, so the interval is pinned here.
        self.assertEqual(r.POLL_S, 0.1)
        self.assertEqual(r.launch.__kwdefaults__['poll_s'], 0.1)


class RlimitOnlyOnLinux(unittest.TestCase):
    """H4: the preexec applies RLIMIT_AS on Linux, and never on macOS (a mocked resource)."""

    def mock(self):
        calls = []
        resource = types.SimpleNamespace(RLIMIT_AS=9, setrlimit=lambda which, limits: calls.append((which, limits)))
        return resource, calls

    def test_linux_applies_rlimit_as(self):
        resource, calls = self.mock()
        preexec = r.rlimit_preexec('Linux', 1234, resource)
        self.assertIsNotNone(preexec)
        preexec()
        self.assertEqual(calls, [(9, (1234, 1234))])

    def test_darwin_does_not(self):
        resource, calls = self.mock()
        self.assertIsNone(r.rlimit_preexec('Darwin', 1234, resource))
        self.assertEqual(calls, [])


class Classification(unittest.TestCase):
    """H6: every class, from recorded-shape inputs."""

    def classify(self, **kw):
        args = dict(killed_by_rss_watchdog=False, timed_out=False, exit_code=0, objects=[], stderr_text='',
                    system='Darwin', rlimit_applied=False, k6_protocol=True)
        args.update(kw)
        return r.classify(**args)[0]

    def test_classes(self):
        summary = [{'kind': 'summary'}]
        self.assertEqual(self.classify(objects=summary), 'ok')
        self.assertEqual(self.classify(exit_code=0, objects=[]), 'error')
        self.assertEqual(self.classify(exit_code=0, objects=[], k6_protocol=False), 'ok')
        self.assertEqual(self.classify(killed_by_rss_watchdog=True, exit_code=-9), 'killed_by_rss_watchdog')
        self.assertEqual(self.classify(timed_out=True, exit_code=-9), 'timed_out')
        self.assertEqual(self.classify(exit_code=3, objects=[{'kind': 'refusal'}]), 'refused_by_binary')
        self.assertEqual(self.classify(exit_code=1, stderr_text=r.HEAP_CAP_MARKER + '8 bytes (current 1, cap 1)\n'
                                       'memory allocation of 8 bytes failed\n'), 'heap_cap_abort')
        self.assertEqual(self.classify(exit_code=-6, system='Linux', rlimit_applied=True,
                                       stderr_text='memory allocation of 8 bytes failed\n'), 'rlimit_abort')
        self.assertEqual(self.classify(exit_code=1, system='Linux', rlimit_applied=True, k6_protocol=False,
                                       stderr_text='MemoryError\n'), 'rlimit_abort')
        self.assertEqual(self.classify(exit_code=1, system='Darwin', rlimit_applied=False,
                                       stderr_text='memory allocation of 8 bytes failed\n'), 'error')
        self.assertEqual(set(r.CLASSES), {'ok', 'timed_out', 'killed_by_rss_watchdog', 'heap_cap_abort',
                                          'rlimit_abort', 'refused_by_binary', 'error'})


class Aggregation(unittest.TestCase):
    """H6: median and minimum per stage over the repeats (K6-M7)."""

    def test_median_and_minimum(self):
        self.assertEqual(r.median([5, 1, 3]), 3)
        self.assertEqual(r.median([4, 1, 3, 100]), 3.5)
        stages = r.aggregate_stages(r.jsonl(JSONL_FIXTURE))
        self.assertEqual(stages['assembly']['samples'], 3)
        self.assertEqual(stages['assembly']['median_ns'], 300)
        self.assertEqual(stages['assembly']['min_ns'], 100)
        self.assertEqual(stages['assembly']['heap_peak'], 48655)
        self.assertEqual(stages['prepare']['median_ns'], 60)
        self.assertEqual(stages['prepare']['min_ns'], 50)

    def test_w1_figures(self):
        """K6b: seconds per LME divides the median w1_solve time by the meter's charged work; the
        prefix increments and the stop rule's increment are differences of the recorded peaks."""
        import k6b_analysis as a
        objects = [
            {'kind': 'counts', 'model': 'M', 'estimate_adm_bytes_w1a': 1000, 'estimate_w1_sel128_bytes': 500,
             'estimate_w1_decide_bytes': 300},
            {'kind': 'stage', 'stage': 'w1_solve', 'repeat': 0, 'elapsed_ns': 3_000_000_000,
             'heap_current_begin': 100, 'heap_peak': 400, 'heap_peak_move': 400, 'ok': True},
            {'kind': 'outcome', 'repeat': 0, 'class': 'Selected', 'selected_precision': 128, 'rows': 7,
             'attempts': 2, 'meter_charged': 1_000_000},
            {'kind': 'stage', 'stage': 'w1_solve', 'repeat': 1, 'elapsed_ns': 1_000_000_000,
             'heap_current_begin': 100, 'heap_peak': 400, 'heap_peak_move': 400, 'ok': True},
            {'kind': 'stage', 'stage': 'w1_solve', 'repeat': 2, 'elapsed_ns': 2_000_000_000,
             'heap_current_begin': 100, 'heap_peak': 400, 'heap_peak_move': 400, 'ok': True},
            {'kind': 'stage', 'stage': 'w1_prefix_1', 'repeat': 3, 'elapsed_ns': 5, 'heap_current_begin': 100,
             'heap_peak': 200, 'heap_peak_move': 200, 'ok': True},
            {'kind': 'prefix', 'prefix': 1, 'through_segment': 'solve_128', 'case_limit': 10},
            {'kind': 'stage', 'stage': 'w1_prefix_2', 'repeat': 3, 'elapsed_ns': 7, 'heap_current_begin': 100,
             'heap_peak': 350, 'heap_peak_move': 350, 'ok': True},
            {'kind': 'prefix', 'prefix': 2, 'through_segment': 'verify_256', 'case_limit': 20},
        ]
        f = a.w1_figures(objects)
        self.assertEqual(f['solve_median_ns'], 2_000_000_000)
        self.assertEqual(f['s_per_lme_median'], 2.0 / 1_000_000)
        self.assertEqual(f['s_per_lme_min'], 1.0 / 1_000_000)
        self.assertEqual([p['heap_peak'] for p in f['prefixes']], [100, 250])
        self.assertEqual([p['increment'] for p in f['prefixes']], [100, 150])
        self.assertEqual(f['call_heap_peak'], 300)
        self.assertEqual(f['decide_increment'], 50)
        self.assertEqual(f['tracker_entries_equivalent'], 50 / a.TRACKER_ENTRY_BYTES)
        self.assertEqual(f['heap_over_e_max'], 0.3)
        self.assertEqual(a.TRACKER_ENTRY_BYTES, 4304)

    def test_fit(self):
        fit = r.fit_loglog([(10, 1000.0), (100, 10000.0), (1000, 100000.0)])
        self.assertAlmostEqual(fit['slope'], 1.0, places=12)
        self.assertEqual(r.fit_loglog([(10, 1.0)]), None)


class Schema(unittest.TestCase):
    """H6: the JSONL schema's required keys per kind."""

    def test_required_keys(self):
        for obj in r.jsonl(JSONL_FIXTURE):
            self.assertTrue(REQUIRED_KEYS.get(obj['kind'], set()) <= set(obj), obj['kind'])
        self.assertEqual(r.jsonl(JSONL_FIXTURE)[0]['schema'], 'k6-observe-v1')


class MetadataHasNoHostIdentifiers(unittest.TestCase):
    """H6: the metadata carries no host name, user name, serial or path."""

    def test_metadata(self):
        def fake_run(argv, env=None):
            values = {'sw_vers': '26.6.2', 'sysctl': 'Apple M-series', 'rustc': 'rustc 1.97.1\nhost: aarch64-apple-darwin',
                      'cargo': 'cargo 1.97.1\nos: Mac OS 26.6.2 [64-bit]'}
            return types.SimpleNamespace(returncode=0, stdout=values.get(argv[0], '0'))
        with tempfile.NamedTemporaryFile(delete=False) as fh:
            fh.write(b'binary bytes')
            binary = fh.name
        try:
            md = r.metadata(binary, 'a' * 40, 'b' * 40, run=fake_run, system='Darwin')
        finally:
            os.unlink(binary)
        text = json.dumps(md)
        for identifier in {socket.gethostname(), platform.node(), getpass.getuser()} - {''}:
            self.assertNotIn(identifier, text)
        for key in md:
            self.assertNotIn(key, ('hostname', 'node', 'user', 'serial', 'path'))
        for value in json.dumps(md).split('"'):
            self.assertFalse(value.startswith('/'), value)
        self.assertEqual(md['binary_name'], os.path.basename(binary))
        self.assertEqual(md['binary_sha256'], hashlib.sha256(b'binary bytes').hexdigest())

    def test_sanitize(self):
        # Paths are built at run time, so the source holds no machine-path literal (RV18-N2).
        def root(*parts):
            return '/' + '/'.join(parts)
        self.assertEqual(r.sanitize('at %s:3:4 and %s' % (root('Volumes', 'scratch', 'src', 'a.rs'),
                                                        root('var', 'folders', 'q'))),
                         'at <path>:3:4 and <path>')
        for path in (root('Us' + 'ers', 'someone', 'dev', 'b.rs'), root('private', 'tmp', 'c.txt'),
                     root('home', 'someone', 'd.txt')):
            self.assertEqual(r.sanitize('in %s:7' % path), 'in <path>:7', path)


def fake_counts(estimates):
    """Synthetic counts lines: {model: {mode: estimate}}."""
    out = {}
    for model_id, per_mode in estimates.items():
        line = {'kind': 'counts', 'model': model_id}
        for mode, value in per_mode.items():
            line[r.estimate_key(mode)] = value
        out[model_id] = line
    return out


def run_of(model_id, mode):
    return next(x for x in r.schedule() if x['model'] == model_id and x['mode'] == mode)


class PlanAdmission(unittest.TestCase):
    """H7: the schedule and the admission rule (Q3, Q4, Q12, N9, N11, N14; RV16-N4; the A1-stop Q2)."""

    def test_schedule_shape(self):
        # K6's tiers keep K6's 138 rows (K6b freezes K6_MODES); K6b's W1 tiers follow them.
        runs = [x for x in r.schedule() if not x['tier'].startswith('W1-')]
        self.assertEqual(len(runs), 138)
        self.assertEqual([x['order'] for x in runs], list(range(1, 139)))
        self.assertEqual(r.K6_MODES, ('sparse', 'dense', 'lane-id', 'lane-lu'))
        self.assertEqual({x['slot'] for x in runs if x['tier'] == 'T3b'}, {'B2'})
        self.assertEqual([x['slot'] for x in runs if x['tier'] == 'T6'], ['B3'])
        ceiling = run_of('K6-CEIL-CHAIN-n01364-AX', 'dense')
        self.assertEqual((ceiling['rss_cap_bytes'], ceiling['heap_cap_bytes']), (16 * r.GIB, 16 * r.GIB - 512 * r.MIB))
        self.assertEqual(ceiling['entry_repeats'], 1)
        self.assertEqual(run_of('RF-LARGE-CHAIN-n01000-ROT', 'dense')['repeats'], 1)
        self.assertEqual(run_of('RF-LARGE-TREE-n01000-AX', 'dense')['repeats'], 1)
        self.assertEqual(run_of('RF-LARGE-CONT-n01000-AX', 'dense')['repeats'], 5)
        self.assertEqual(run_of('RF-LARGE-CONT-n01000-AX', 'dense')['entry_repeats'], 1)
        self.assertEqual(run_of('RF-LARGE-CONT-n01000-AX', 'lane-lu')['repeats'], 1)
        self.assertEqual(run_of('RF-LARGE-CONT-n00100-AX', 'dense')['timeout_s'], 600)
        self.assertEqual(run_of('RF-LARGE-CONT-n01000-AX', 'dense')['time_budget_s'], 1740)
        first = [x['mode'] for x in runs if x['tier'] == 'T1' and x['model'] == 'RF-LARGE-CHAIN-n00010-AX']
        second = [x['mode'] for x in runs if x['tier'] == 'T1' and x['model'] == 'RF-LARGE-CHAIN-n00010-ROT']
        self.assertEqual(first, ['sparse', 'dense', 'lane-id', 'lane-lu'])
        self.assertEqual(second, ['dense', 'lane-id', 'lane-lu', 'sparse'])

    def test_refusals_by_name(self):
        counts = fake_counts({x['model']: {m: 1 for m in r.MODES} for x in r.schedule()})
        rows = [dict(x, admission=r.admission(x, counts, [], None, require_ascent=False)) for x in r.schedule()]
        never = [x for x in rows if x['admission']['decision'] == 'never']
        self.assertEqual(len(never), 14)
        for x in never:
            if x['mode'] in ('dense', 'lane-lu'):
                self.assertEqual(x['members'], 10000)
                self.assertEqual(x['admission']['reason'], 'never:n2_mode_at_or_above_10000_members')
            else:
                self.assertTrue(x['model'].startswith('RF-LARGE-CONT-n10000-'))
                self.assertEqual(x['admission']['reason'], 'never:cont_n10000_identity_lane')
        self.assertEqual({x['mode'] for x in never}, {'dense', 'lane-lu', 'lane-id'})

    def test_lane_id_deferred_by_name_when_the_estimate_fails(self):
        run = run_of('RF-LARGE-CHAIN-n10000-AX', 'lane-id')
        counts = fake_counts({run['model']: {'lane-id': 3 * r.GIB}})
        decision = r.admission(run, counts, [], None, require_ascent=False)
        self.assertEqual(decision['decision'], 'deferred')
        self.assertTrue(decision['reason'].startswith('deferred:estimate_fails_admission'))
        self.assertEqual(decision['rho'], 2.0)

    def test_p1_branch_admits_dense_1000(self):
        run = run_of('RF-LARGE-CONT-n01000-AX', 'dense')
        counts = fake_counts({run['model']: {'dense': 3312 * r.MIB}})
        decision = r.admission(run, counts, [], None, require_ascent=False)
        self.assertEqual(decision['decision'], 'admitted')
        self.assertTrue(decision['reason'].startswith('p1_linux_peak'))

    def test_rho_is_the_largest_footprint_ratio_at_smaller_sizes_net_of_baseline(self):
        run = run_of('RF-LARGE-CHAIN-n10000-AX', 'sparse')
        counts = fake_counts({run['model']: {'sparse': 1 * r.GIB}})

        def rec(model_id, members, footprint, rss, heap, estimate):
            return {'order': 1, 'model': model_id, 'mode': 'sparse', 'family': 'CHAIN', 'members': members,
                    'classification': 'ok', 'peak_rss_bytes': rss, 'rss': {'time_peak_footprint_bytes': footprint},
                    'repeats_heap_peak_move': heap, 'estimate_adm_bytes': estimate}
        measured = [rec('RF-LARGE-CHAIN-n00010-AX', 10, 50 * r.MIB, 60 * r.MIB, 1, 1 * r.MIB),   # excluded
                    rec('RF-LARGE-CHAIN-n00100-AX', 100, 11 * r.MIB, 12 * r.MIB, 3 * r.MIB, 4 * r.MIB),
                    rec('RF-LARGE-CHAIN-n01000-AX', 1000, 21 * r.MIB, 25 * r.MIB, 10 * r.MIB, 40 * r.MIB)]
        decision = r.admission(run, counts, measured, 2 * r.MIB, baseline_footprint_bytes=1 * r.MIB)
        # rho (footprint) = max(max(11-1, 3)/4, max(21-1, 10)/40) = 2.5; rho_rss = max((12-2)/4, (25-2)/40) = 2.5
        self.assertAlmostEqual(decision['rho'], 2.5)
        self.assertAlmostEqual(decision['rho_rss'], 2.5)
        self.assertEqual(decision['decision'], 'admitted')
        measured[1]['rss']['time_peak_footprint_bytes'] = 18 * r.MIB     # rho 4.25: 4.25 GiB > 4 GiB
        decision = r.admission(run, counts, measured, 2 * r.MIB, baseline_footprint_bytes=1 * r.MIB)
        self.assertEqual(decision['decision'], 'deferred')
        self.assertTrue(decision['reason'].startswith('deferred:estimate_fails_admission'))

    def test_footprint_not_rss_decides_rho(self):
        run = run_of('RF-LARGE-CHAIN-n10000-AX', 'sparse')
        counts = fake_counts({run['model']: {'sparse': 1 * r.GIB}})
        measured = [{'order': 1, 'model': 'RF-LARGE-CHAIN-n01000-AX', 'mode': 'sparse', 'family': 'CHAIN',
                     'members': 1000, 'classification': 'ok', 'peak_rss_bytes': 180 * r.MIB,
                     'rss': {'time_peak_footprint_bytes': 101 * r.MIB}, 'repeats_heap_peak_move': 90 * r.MIB,
                     'estimate_adm_bytes': 100 * r.MIB}]
        decision = r.admission(run, counts, measured, 0, baseline_footprint_bytes=1 * r.MIB)
        self.assertAlmostEqual(decision['rho'], 1.0)          # footprint-based: admitted at 1 GiB
        self.assertAlmostEqual(decision['rho_rss'], 1.8)      # RSS-based, recorded for comparison
        self.assertEqual(decision['decision'], 'admitted')
        self.assertAlmostEqual(decision['rss_to_footprint'], 180 / 101)

    def test_projected_rss_above_0_8_c_is_deferred_by_name(self):
        run = run_of('RF-LARGE-CHAIN-n10000-AX', 'sparse')
        counts = fake_counts({run['model']: {'sparse': 3 * r.GIB}})
        measured = [{'order': 1, 'model': 'RF-LARGE-CHAIN-n01000-AX', 'mode': 'sparse', 'family': 'CHAIN',
                     'members': 1000, 'classification': 'ok', 'peak_rss_bytes': 250 * r.MIB,
                     'rss': {'time_peak_footprint_bytes': 100 * r.MIB}, 'repeats_heap_peak_move': 90 * r.MIB,
                     'estimate_adm_bytes': 100 * r.MIB}]
        decision = r.admission(run, counts, measured, 0, baseline_footprint_bytes=0)
        # footprint estimate 3 GiB x rho 1.0 <= 4 GiB, but projected RSS 3 GiB x 2.5 = 7.5 GiB > 6.4 GiB.
        self.assertEqual(decision['decision'], 'deferred')
        self.assertTrue(decision['reason'].startswith('deferred:projected_rss_exceeds_0.8C'))

    def test_dense_uses_the_ruled_rss_ratio_floor(self):
        run = run_of('RF-LARGE-TREE-n01000-ROT', 'dense')
        counts = fake_counts({run['model']: {'dense': 3312 * r.MIB}})
        measured = [{'order': 1, 'model': 'RF-LARGE-TREE-n00100-ROT', 'mode': 'dense', 'family': 'TREE',
                     'members': 100, 'classification': 'ok', 'peak_rss_bytes': 46 * r.MIB,
                     'rss': {'time_peak_footprint_bytes': 45 * r.MIB}, 'repeats_heap_peak_move': 35 * r.MIB,
                     'estimate_adm_bytes': 35 * r.MIB}]
        decision = r.admission(run, counts, measured, 0, baseline_footprint_bytes=0)
        self.assertEqual(decision['rss_to_footprint'], r.RSS_TO_FOOTPRINT_DEFAULT)
        self.assertEqual(decision['decision'], 'admitted')     # P1 3,617.2 MiB x 1.45 = 5.1 GiB <= 6.4 GiB
        self.assertEqual(decision['projected_rss_bytes'], int(int(3617.2 * r.MIB) * 1.45))

    def test_record_consumers_ignore_cross_mode_rows(self):
        rows = [{'kind': 'cross_mode', 'model': 'RF-LARGE-CHAIN-n00010-AX', 'class_equal': True},
                {'order': 1, 'model': 'RF-LARGE-CHAIN-n00010-AX', 'mode': 'sparse'}]
        self.assertEqual(r.run_entries(rows), rows[1:])
        with tempfile.TemporaryDirectory() as folder:
            self.assertIsNone(r.cross_mode(run_of('RF-LARGE-CHAIN-n00010-AX', 'dense'), rows, folder))

    def test_ascent_waits_for_the_previous_size(self):
        run = run_of('RF-LARGE-CHAIN-n10000-AX', 'sparse')
        counts = fake_counts({run['model']: {'sparse': r.MIB}})
        decision = r.admission(run, counts, [], None)
        self.assertEqual(decision['decision'], 'deferred')
        self.assertIn('ascent_previous_size_not_recorded', decision['reason'])

    def test_ceiling_uses_its_own_cap(self):
        run = run_of('K6-CEIL-CHAIN-n01364-AX', 'dense')
        counts = fake_counts({run['model']: {'dense': 6154 * r.MIB}})
        measured = [{'order': 1, 'model': 'RF-LARGE-CHAIN-n01000-AX', 'mode': 'dense', 'family': 'CHAIN',
                     'members': 1000,
                     'classification': 'ok', 'peak_rss_bytes': 3400 * r.MIB, 'repeats_heap_peak_move': 3350 * r.MIB,
                     'estimate_adm_bytes': 3312 * r.MIB}]
        decision = r.admission(run, counts, measured, 2 * r.MIB)
        self.assertEqual(decision['half_cap_bytes'], 8 * r.GIB)
        self.assertEqual(decision['decision'], 'admitted')

    def test_plan_starts_no_child(self):
        original = r.subprocess.Popen
        r.subprocess.Popen = None
        try:
            rows = r.plan(None)
        finally:
            r.subprocess.Popen = original
        self.assertEqual(len(rows), 138 + 132)
        self.assertIn('never:n2_mode_at_or_above_10000_members', r.format_plan(rows))

    def test_w1_tiers(self):
        """K6b (plan section 4): four W1 tiers of ABAB/BABA pairs, 132 rows after K6's 138."""
        runs = [x for x in r.schedule() if x['tier'].startswith('W1-')]
        self.assertEqual(len(runs), 132)
        self.assertEqual(runs[0]['order'], 139)
        self.assertEqual({x['tier']: x['slot'] for x in runs},
                         {'W1-T1': 'K6B-S1', 'W1-T2': 'K6B-S1', 'W1-T3': 'K6B-S2', 'W1-T4': 'K6B-S3'})
        by_model = {}
        for x in runs:
            by_model.setdefault((x['tier'], x['model']), []).append(x)
        self.assertEqual(len(by_model), 15 + 6 + 6 + 6)
        for (tier, model_id), rows in by_model.items():
            modes = [x['mode'] for x in rows]
            self.assertIn(modes, (['w1a', 'sparse', 'w1a', 'sparse'], ['sparse', 'w1a', 'sparse', 'w1a']))
            self.assertEqual([x['pass'] for x in rows if x['mode'] == 'w1a'], [1, 2])
            self.assertEqual(len({x['run_id'] for x in rows}), 4)
        first = [x['mode'] for x in runs if x['tier'] == 'W1-T1' and x['model'] == 'RF-LARGE-CHAIN-n00010-AX']
        second = [x['mode'] for x in runs if x['tier'] == 'W1-T1' and x['model'] == 'RF-LARGE-CHAIN-n00010-ROT']
        self.assertEqual(first, ['w1a', 'sparse', 'w1a', 'sparse'])
        self.assertEqual(second, ['sparse', 'w1a', 'sparse', 'w1a'])
        self.assertTrue(all(x['conditional'] for x in runs if x['tier'] == 'W1-T4'))
        self.assertFalse(any(x['conditional'] for x in runs if x['tier'] != 'W1-T4'))

    def test_w1_admission_and_argv(self):
        """w1a is never refused by name; it is admitted on E_max x rho; pass 1 dumps its rows and
        runs the prefixes, pass 2 does not."""
        self.assertIn('w1a', r.MODES)
        self.assertEqual(r.estimate_key('w1a'), 'estimate_adm_bytes_w1a')
        self.assertIsNone(r.refusal_by_name('RF-LARGE-CHAIN-n10000-AX', 'w1a', 10000))
        run = next(x for x in r.schedule() if x['tier'] == 'W1-T3' and x['mode'] == 'w1a')
        counts = fake_counts({run['model']: {'w1a': 3 * r.GIB}})
        decision = r.admission(run, counts, [], None, require_ascent=False)
        self.assertEqual(decision['decision'], 'deferred')
        self.assertTrue(decision['reason'].startswith('deferred:estimate_fails_admission'))
        counts = fake_counts({run['model']: {'w1a': r.GIB}})
        self.assertEqual(r.admission(run, counts, [], None, require_ascent=False)['decision'], 'admitted')
        argv = r.binary_argv('k6_observe', run, 'counts.jsonl', 'records')
        self.assertIn('--w1-prefixes', argv)
        self.assertIn('--dump-published', argv)
        second = next(x for x in r.schedule() if x['tier'] == 'W1-T3' and x['mode'] == 'w1a'
                      and x['model'] == run['model'] and x['pass'] == 2)
        argv2 = r.binary_argv('k6_observe', second, 'counts.jsonl', 'records')
        self.assertNotIn('--w1-prefixes', argv2)
        self.assertNotIn('--dump-published', argv2)

    def test_w1_t4_rows_are_deferred_by_name_until_root_approves(self):
        run = next(x for x in r.schedule() if x['tier'] == 'W1-T4' and x['mode'] == 'sparse')
        self.assertEqual(run['conditional_reason'], 'deferred:w1_t4_needs_root_approval_after_w1_t3_prefixes')
        with tempfile.TemporaryDirectory() as d:
            counts_path = os.path.join(d, 'counts.jsonl')
            with open(counts_path, 'w') as fh:
                for x in r.schedule():
                    if x['tier'] == 'W1-T4':
                        fh.write(json.dumps({'kind': 'counts', 'model': x['model'],
                                             'estimate_adm_bytes_sparse': 1, 'estimate_adm_bytes_w1a': 1}) + '\n')
            calls = []
            original = (r.baseline_run, r.wait_for_quiet_host, r.launch, r.metadata)
            r.baseline_run = lambda *a, **k: {'rss': {}}
            r.wait_for_quiet_host = lambda *a, **k: calls.append('quiet')
            r.launch = lambda *a, **k: calls.append('launch')
            r.metadata = lambda *a, **k: {}
            try:
                r.run_tier('W1-T4', 'k6_observe', counts_path, d, 'c', 't', log=lambda *a: None)
            finally:
                r.baseline_run, r.wait_for_quiet_host, r.launch, r.metadata = original
            self.assertNotIn('launch', calls)
            records = r.read_records(d)
            self.assertEqual(len(records), 24)
            self.assertTrue(all(x['classification'] == 'not_run' for x in records))
            self.assertTrue(all(x['admission']['reason'].startswith('deferred:') for x in records))

    def test_every_admitted_row_passes_the_binarys_backstop(self):
        """ROOT's ruling after W1-T4's stop: for every row the runner admits, the binary's own
        backstop (estimate <= heap cap / 2) admits it too, or the row is deferred by name. Checked
        on the committed K6b counts at plan time, and on a w1a row whose measured rho would
        otherwise admit it (W1-T4's row 247)."""
        counts = r.read_counts(os.path.join(os.path.dirname(OBSERVATIONS), 'k6b', 'counts.jsonl'))
        admitted = 0
        for run in r.schedule():
            decision = r.admission(run, counts, [], None, require_ascent=False)
            if decision['decision'] == 'admitted':
                admitted += 1
                self.assertLessEqual(decision['estimate_adm_bytes'], run['heap_cap_bytes'] // 2, run['run_id'])
        self.assertGreater(admitted, 200)
        run = next(x for x in r.schedule() if x['order'] == 247)
        self.assertEqual((run['model'], run['mode']), ('RF-LARGE-CHAIN-n10000-AX', 'w1a'))
        smaller = next(x for x in r.schedule() if x['tier'] == 'W1-T3' and x['mode'] == 'w1a'
                       and x['family'] == 'CHAIN')
        measured = [dict(smaller, classification='ok', estimate_adm_bytes=10 ** 9,
                         repeats_heap_peak_move=184 * 10 ** 6, rss={'time_peak_footprint_bytes': 1})]
        counts = fake_counts({run['model']: {'w1a': 9230781374}})
        decision = r.admission(run, counts, measured, None, require_ascent=False)
        self.assertAlmostEqual(decision['rho'], 0.184)
        self.assertEqual(decision['decision'], 'deferred')
        self.assertTrue(decision['reason'].startswith('deferred:binary_backstop_refuses'), decision['reason'])
        # Just under the backstop, the same row is admitted.
        counts = fake_counts({run['model']: {'w1a': run['heap_cap_bytes'] // 2}})
        self.assertEqual(r.admission(run, counts, measured, None, require_ascent=False)['decision'], 'admitted')

    def test_measured_runs_are_kept_and_not_run_records_are_run(self):
        # The resume skip (I15's B1 fix; C mutant K6-M26): run_tier keeps a measured run and runs
        # a not_run record again. Launch, the baseline, the quiet-host wait and admission are stubbed.
        tier = [x for x in r.schedule() if x['tier'] == 'T5']
        first, last = tier[0], tier[-1]
        launched = []

        def fake_launch(argv, *, record_dir, run_id, **kw):
            launched.append(run_id)
            open(os.path.join(record_dir, run_id + '.jsonl'), 'w').close()
            return {'classification': 'ok', 'wall_s': 0.0, 'peak_rss_bytes': 1, 'exit_code': 0, 'rss': {}}

        admitted = {'decision': 'admitted', 'reason': 'stub', 'estimate_adm_bytes': 1}
        with tempfile.TemporaryDirectory() as folder:
            with open(os.path.join(folder, 'metadata.json'), 'w') as fh:
                fh.write('{}\n')
            r.append_record(folder, dict(first, classification='ok'))
            r.append_record(folder, dict(last, classification='not_run',
                                         admission={'decision': 'deferred', 'reason': 'deferred:stub'}))
            with mock.patch.object(r, 'launch', fake_launch), \
                    mock.patch.object(r, 'baseline_run', lambda *a, **k: {'peak_rss_bytes': 1, 'rss': {}}), \
                    mock.patch.object(r, 'wait_for_quiet_host', lambda log: {'waited_s': 0}), \
                    mock.patch.object(r, 'admission', lambda *a, **k: dict(admitted)), \
                    mock.patch.object(r, 'CONDITIONAL_MODELS', ()):
                r.run_tier('T5', 'k6_observe', None, folder, 'commit', 'tree', log=lambda m: None)
        self.assertNotIn(first['run_id'], launched, 'a measured run was repeated')
        self.assertIn(last['run_id'], launched, 'a not_run record was taken as measured')
        self.assertEqual(launched, [x['run_id'] for x in tier[1:]])


class QuietHost(unittest.TestCase):
    """ROOT's B1 grant: wait while the 1-minute load is above 8, until it is below 6; no cargo or
    sweep; memorystatus at least 80."""

    def test_load_wait_hysteresis(self):
        loads = iter([9.0, 7.0, 7.5, 5.9])
        slept = []
        idle = lambda argv, env=None: types.SimpleNamespace(returncode=1, stdout='')
        result = r.wait_for_quiet_host(lambda msg: None, run=idle, level=lambda: 95, sleep=slept.append,
                                       load=lambda: (next(loads), 0, 0))
        self.assertEqual(len(slept), 3)        # 9.0 starts the wait; 7.0 and 7.5 keep it; 5.9 ends it
        self.assertEqual(result['load1_at_start'], 5.9)

    def test_busy_cargo_and_memory_wait(self):
        # Per check: (cargo running, memorystatus_level). Busy, then low memory, then quiet.
        states = [(True, 95), (False, 70), (False, 95)]
        calls = {'level': 0}

        def run(argv, env=None):
            busy = argv[:2] == ['pgrep', '-x'] and states[calls['level']][0]
            return types.SimpleNamespace(returncode=0 if busy else 1, stdout='1' if busy else '')

        def level():
            mem = states[calls['level']][1]
            calls['level'] += 1
            return mem
        slept = []
        r.wait_for_quiet_host(lambda msg: None, run=run, level=level, sleep=slept.append, load=lambda: (1.0, 0, 0))
        self.assertEqual(len(slept), 2)

    def test_a_running_sweep_alone_makes_the_host_busy(self):
        # RV18-N3: the DEC-025 sweep check. Only the sweep's pattern is busy, once.
        def sweep_only(argv, env=None):
            busy = argv[:2] == ['pgrep', '-f']
            return types.SimpleNamespace(returncode=0 if busy else 1, stdout='1' if busy else '')
        self.assertEqual(r.host_busy(sweep_only), ['sweep'])
        calls = {'sweep': 0}

        def once(argv, env=None):
            if argv[:2] == ['pgrep', '-f']:
                calls['sweep'] += 1
                if calls['sweep'] == 1:
                    return types.SimpleNamespace(returncode=0, stdout='1')
            return types.SimpleNamespace(returncode=1, stdout='')
        slept = []
        r.wait_for_quiet_host(lambda msg: None, run=once, level=lambda: 95, sleep=slept.append,
                              load=lambda: (1.0, 0, 0))
        self.assertEqual(len(slept), 1)


# A sleeping helper started first by a live child, in the child's process group, with its stdio
# on /dev/null; it exits by itself after 30 s.
HELPER_FIRST = (
    "import subprocess, sys\n"
    "subprocess.Popen([sys.executable, '-c', 'import time; time.sleep(30)'],\n"
    "                 stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)\n")


def with_helper(child_argv):
    """The same `python -c` child, with the sleeping helper started first."""
    assert child_argv[1] == '-c'
    return [child_argv[0], '-c', HELPER_FIRST + child_argv[2]] + child_argv[3:]


class LiveLimit(unittest.TestCase):
    """H1–H3, live: a self-limiting Python child (failsafe exit 3 at 512 MiB) under a 128 MiB cap."""

    CAP = 128 * r.MIB

    def test_live_memory_limit(self):
        system = platform.system()
        child = r.growing_child_argv(r.MIB, 0, 512 * r.MIB, 0.01)
        with tempfile.TemporaryDirectory() as folder:
            if system == 'Darwin':
                rec = r.launch(child, rss_cap_bytes=self.CAP, timeout_s=60, record_dir=folder, run_id='watchdog',
                               k6_protocol=False)
                self.assertNotEqual(rec['exit_code'], 3, 'the child reached its failsafe: the watchdog did not kill')
                self.assertEqual(rec['classification'], 'killed_by_rss_watchdog')
                self.assertTrue(rec['binary_pid_found'])
                last = rec['watchdog_last_kib'] * r.KIB
                self.assertGreaterEqual(last, self.CAP)
                samples = rec['watchdog_samples']
                growth = max(b[1] - a[1] for a, b in zip(samples, samples[1:])) * r.KIB
                self.assertLessEqual(last - self.CAP, growth)
                self.assertEqual(rec['survivors'], [])
            elif system == 'Linux':
                rec = r.launch(child, rss_cap_bytes=self.CAP, timeout_s=60, record_dir=folder, run_id='rlimit',
                               wrapper=False, k6_protocol=False)
                self.assertNotEqual(rec['exit_code'], 3)
                self.assertTrue(rec['rlimit_as_applied'])
                self.assertEqual(rec['classification'], 'rlimit_abort')
            else:
                self.fail('no live memory-limit path on %s' % system)

    def test_negative_control(self):
        system = platform.system()
        child = r.growing_child_argv(r.MIB, 64 * r.MIB, 512 * r.MIB, 0.01)
        with tempfile.TemporaryDirectory() as folder:
            rec = r.launch(child, rss_cap_bytes=self.CAP, timeout_s=60, record_dir=folder, run_id='control',
                           wrapper=(system == 'Darwin'), k6_protocol=False)
        self.assertFalse(rec['killed_by_rss_watchdog'])
        self.assertEqual(rec['classification'], 'ok')

    def test_kills_take_the_whole_group(self):
        # C mutant K6-M4: under /usr/bin/time the killed binary is the wrapper's only child, so the
        # wrapper exits by itself and a kill of the child alone looks like a group kill. Here the
        # child first starts a sleeping helper in its process group; a group kill leaves no survivor.
        # The helper's stdio is /dev/null: a helper holding the runner's pipes would make the
        # runner's pipe close wait for it, and the survivor check would run only after it exited.
        system = platform.system()
        with tempfile.TemporaryDirectory() as folder:
            if system == 'Darwin':
                child = with_helper(r.growing_child_argv(r.MIB, 0, 512 * r.MIB, 0.01))
                started = time.monotonic()
                rec = r.launch(child, rss_cap_bytes=self.CAP, timeout_s=60, record_dir=folder, run_id='group',
                               k6_protocol=False)
                self.assertLess(time.monotonic() - started, 20)
                self.assertEqual(rec['classification'], 'killed_by_rss_watchdog')
                self.assertEqual(rec['survivors'], [], 'the watchdog kill left a process of the group')
            # The timeout path, on every platform: a child that neither grows nor exits.
            sleeper = [sys.executable, '-c', HELPER_FIRST + 'import time; time.sleep(30)\n']
            started = time.monotonic()
            rec = r.launch(sleeper, rss_cap_bytes=self.CAP, timeout_s=1, record_dir=folder, run_id='timeout',
                           wrapper=(system == 'Darwin'), k6_protocol=False)
            self.assertLess(time.monotonic() - started, 20)
            self.assertEqual(rec['classification'], 'timed_out')
            self.assertEqual(rec['survivors'], [], 'the timeout kill left a process of the group')

    def test_a_terminated_runner_leaves_no_survivor(self):
        # RV18-3: SIGTERM to the runner mid-run. A driver process runs `launch` on a sleeping
        # child, which starts the sleeping helper and writes its process group to a file.
        # The test then sends SIGTERM to the driver. The driver's `finally` must kill the group
        # before it exits with 143. The cap is 1 GiB, so Linux's RLIMIT_AS does not bind on the
        # interpreters.
        system = platform.system()
        child_code = (HELPER_FIRST + 'import os, time\n'
                      'open(sys.argv[1] + ".tmp", "w").write(str(os.getpgrp()))\n'
                      'os.replace(sys.argv[1] + ".tmp", sys.argv[1])\n'
                      'time.sleep(30)\n')
        driver_code = ('import sys\n'
                       'sys.path.insert(0, sys.argv[1])\n'
                       'import k6_runner as r\n'
                       "r.launch([sys.executable, '-c', sys.argv[3], sys.argv[2]], rss_cap_bytes=r.GIB,"
                       " timeout_s=60, record_dir=sys.argv[4], run_id='term', wrapper=sys.argv[5] == 'Darwin',"
                       ' k6_protocol=False)\n')
        with tempfile.TemporaryDirectory() as folder:
            marker = os.path.join(folder, 'pgid')
            driver = subprocess.Popen([sys.executable, '-c', driver_code, HERE, marker, child_code, folder, system])
            pgid = None
            try:
                deadline = time.monotonic() + 20
                while not os.path.exists(marker) and time.monotonic() < deadline and driver.poll() is None:
                    time.sleep(0.02)
                self.assertTrue(os.path.exists(marker), 'the observed child did not start')
                with open(marker) as fh:
                    pgid = int(fh.read())
                started = time.monotonic()
                driver.send_signal(signal.SIGTERM)
                code = driver.wait(timeout=20)
                survivors = r.survivors_of(pgid)
            finally:
                if driver.poll() is None:
                    driver.kill()
                    driver.wait()
                if pgid is not None and r.survivors_of(pgid, attempts=1):
                    os.killpg(pgid, signal.SIGKILL)
        self.assertEqual(code, 128 + signal.SIGTERM)
        self.assertLess(time.monotonic() - started, 20)
        self.assertEqual(survivors, [], 'a terminated runner left a process of the observation group')


if __name__ == '__main__':
    unittest.main()
