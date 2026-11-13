#!/usr/bin/env python3
"""DEL-04-05 release timing runner (RT-U4, release target exit criterion 9): standard library only.

It runs the release timing driver (``core/solver/release_timing``,
binary ``release_timing``, schema ``release-timing-v2``) on invented rack
models, one process at a time (a solve run, or a model-checks run of the
desktop's on-demand model checks), each under K6's launch machinery
(``k6_runner.launch``: ``/usr/bin/time -l``, the 100 ms ``ps`` RSS watchdog, the
timeout and the process-group kill, and its record files), and gathers a
compact packet. A footprint watchdog emulates the product's memory budget: every
poll it reads the driver's phys_footprint and lifetime-maximum phys_footprint
(macOS ``proc_pid_rusage``, ``RUSAGE_INFO_V4``) and kills the run when either
exceeds the budget, as the desktop kills a worker (``worker_footprint_exceeded``).

Modes (each starts nothing else):

- ``--plan``: the schedule with its estimates and their total;
- ``--generate --models DIR``: the plan's models from the rack generator (a
  committed model, the reference system, is copied after its generator's
  ``--check``), and ``DIR/manifest.json`` (profile, nodes, sha256, bytes, stats);
- ``--run --models DIR --out DIR --binary PATH``: the plan's runs, within a
  window budget (``--budget-s``); ``--write-results`` keeps each result's bytes;
- ``--packet --out DIR``: ``DIR/packet.json`` from the run records.
- ``--read-back BUNDLE --revision SHA``: the direct checks of a baseline bundle's records,
  read only.
- ``--baseline-set OUT --plan-file PLAN --subset NAME``: two results on one supervised
  folder of a named plan, the ten-run timing set and the rack observation, read only.

The footprint budget is the base-machine budget by default: 12 GiB, the
product's per-invocation memory budget on the 16 GiB base machine (docs/PRD.md
section 12.8; ``apps/desktop/src-tauri/src/memory_pool.rs``
``DEFAULT_INVOCATION_BUDGET_BYTES``). The RSS watchdog is an emergency cap only,
16 GiB by default (the base machine's physical memory). Observation only:
nothing here asserts a time or memory bound, and criterion 9's provisional
bounds are not evaluated.
"""
from __future__ import annotations

import argparse
import ctypes
import ctypes.util
import hashlib
import json
import os
import re
import shutil
import stat
import statistics
import subprocess
import sys
import threading
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import k6_runner as k6  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, '..', '..', '..', '..'))
GENERATOR = os.path.join(HERE, '..', 'rack', 'rack_model.py')
# The rack generator's recorded hashes (`rack_model.py --hashes`), held against the generator by
# tests/test_performance_rack_model.py: a named plan's rack models must have these bytes.
RACK_HASHES = os.path.join(HERE, '..', 'observations', 'rack', 'models_sha256.txt')

DRIVER_SCHEMA = 'release-timing-v2'
RECORD_SCHEMA = 'release-runner-record-v1'
PACKET_SCHEMA = 'release-timing-packet-v1'
MANIFEST_SCHEMA = 'release-models-manifest-v1'
GIB = k6.GIB
# memory_pool.rs DEFAULT_INVOCATION_BUDGET_BYTES (12 GiB on the 16 GiB base machine).
BASE_MACHINE_BUDGET_BYTES = 12_884_901_888
# The RSS watchdog's emergency cap: the base machine's physical memory.
EMERGENCY_RSS_CAP_BYTES = 16 * GIB
PROGRESS_PREFIX = 'release-timing-stage '
WINDOW_BUDGET_S = 900
QUIET_WAIT_S = 60
SOLVED = 'MECHANICS_SOLVED'
# The driver's status for a `--model-checks` run whose checks returned a report (whatever it found).
CHECKS_REPORTED = 'MODEL_CHECKS_REPORTED'
# The rack generator's profiles. baseline-one-case-ex is not a criterion 9 profile: it is the headless
# baseline's one-case model (rack_model.py PROFILES).
PROFILES = ('linear-ex', 'nonlinear-ex', 'nonlinear-pf', 'baseline-one-case-ex')
# Committed models that are not racks, by the plan entry's profile: the file the plan entry names, the
# fixture generator whose `--check` must exit 0 (the committed file is what it writes) before
# `--generate` copies the file, and its node count (a label in the plan, as a rack's is).
REFERENCE_SYSTEM_DIR = os.path.join(REPO, 'fixtures', 'reference_system')
COMMITTED_MODELS = {
    'reference-system': {
        'source': os.path.join(REFERENCE_SYSTEM_DIR, 'reference_system.json'),
        'check': os.path.join(REFERENCE_SYSTEM_DIR, 'generate_reference_system.py'),
        'nodes': 27,
    },
}
MODES = ('sparse', 'dense')
# A run's task: the driver's solve invocation, or its `--model-checks` invocation (the desktop's
# on-demand model checks, a process of their own).
TASKS = ('solve', 'model_checks')

# The default plan, in run order. Estimates order the window and bound it, nothing else; none is
# a measurement. The linear-ex estimates follow the records taken after the published-row index
# (RT-A3 increment 1b): 142 s at 1,000 nodes in an exclusive window, and the desktop worker's 58 s
# at 500 nodes. The nonlinear estimates are assumptions: no run of either nonlinear rack exists
# since RT-A3 stage P moved the sparse-mode loop onto the sparse pattern, and the nonlinear-pf
# figures before it (97 s at 500 nodes, 1,466 s at 1,000) no longer describe that loop. A timeout
# is generous because a timed-out run records only the stage it reached.
_DEFAULT = (
    # profile, nodes, timeout_s, estimate_s, expect
    ('linear-ex', 250, 120, 25, 'solved'),
    ('linear-ex', 500, 240, 60, 'solved'),
    ('linear-ex', 1000, 900, 150, 'solved'),
    # Its attribution twin: stack snapshots while it runs (timings perturbed, not measurements).
    ('linear-ex', 1000, 900, 170, 'solved', True),
    # Expected over the 12 GiB base-machine budget or beyond one window.
    ('linear-ex', 2000, 900, 650, 'exceeds_base_machine_budget'),
    ('nonlinear-pf', 250, 120, 30, 'solved'),
    ('nonlinear-pf', 500, 240, 90, 'solved'),
    # Refused before any case by the nonlinear admission cap (nonlinear_integration
    # `DENSE_LOOP_BYTES_PER_GLOBAL_ENTRY`), which sparse mode keeps although its loop no longer
    # holds the dense buffers the estimate counts.
    ('nonlinear-pf', 2000, 60, 2, 'refused:NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED'),
    ('nonlinear-pf', 1000, 900, 300, 'solved'),
    ('nonlinear-pf', 1500, 1800, 700, 'solved'),
    # The exact route carries the rack's nonlinear supports since RT-A3 increment 4
    # (rack_model.py), so nonlinear-ex runs the same active-set loop as nonlinear-pf on the same
    # supports and restrained DOFs, and the cap refuses criterion 9's 2,000 nodes before any case.
    ('nonlinear-ex', 2000, 60, 2, 'refused:NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED'),
    # A stack-sampled run on main c407e3db0 (not a timing) ended in 24 s at 1,000 nodes.
    ('nonlinear-ex', 250, 120, 15, 'solved'),
    ('nonlinear-ex', 500, 240, 30, 'solved'),
    ('nonlinear-ex', 1000, 900, 60, 'solved'),
    # The desktop's on-demand model checks, each a process of its own (the driver's --model-checks
    # invocation) on the same racks: the checks before stiffness formation and the model-data slot
    # checks, no solve. The estimates are assumptions (no run of them exists); none is a measurement.
    ('linear-ex', 1000, 120, 10, 'ok', False, 'model_checks'),
    ('linear-ex', 2000, 120, 15, 'ok', False, 'model_checks'),
    ('nonlinear-ex', 2000, 120, 15, 'ok', False, 'model_checks'),
    ('nonlinear-pf', 2000, 120, 15, 'ok', False, 'model_checks'),
)


def _solve(r):
    return r['task'] == 'solve'


SUBSETS = {
    'all': lambda r: True,
    # One exclusive window of at most 15 minutes: the scaling points that fit.
    'window': lambda r: _solve(r) and not r['attribute'] and (r['profile'], r['nodes']) in {
        ('linear-ex', 250), ('linear-ex', 500), ('linear-ex', 1000), ('nonlinear-pf', 250), ('nonlinear-pf', 500),
        ('nonlinear-pf', 2000), ('nonlinear-ex', 2000)},
    # A second window: linear-ex 1000 timed, then its attribution twin.
    'window2': lambda r: _solve(r) and (r['profile'], r['nodes']) == ('linear-ex', 1000),
    # One window for the growth law of both exact-route profiles below the nonlinear cap.
    'scaling': lambda r: _solve(r) and not r['attribute'] and r['profile'] in ('linear-ex', 'nonlinear-ex')
    and r['nodes'] in (250, 500, 1000),
    # The same, then the linear rack at criterion 9's size, where a longer window allows it.
    'scaling-2000': lambda r: _solve(r) and not r['attribute'] and (
        (r['profile'] in ('linear-ex', 'nonlinear-ex') and r['nodes'] in (250, 500, 1000))
        or (r['profile'], r['nodes']) == ('linear-ex', 2000)),
    'linear': lambda r: r['profile'] == 'linear-ex',
    'nonlinear-pf': lambda r: r['profile'] == 'nonlinear-pf',
    'nonlinear-ex': lambda r: r['profile'] == 'nonlinear-ex',
    'nonlinear': lambda r: r['profile'] in ('nonlinear-pf', 'nonlinear-ex'),
    # Criterion 9's size: its solves, and the model checks on the same racks.
    'criterion': lambda r: r['nodes'] == 2000,
    'model-checks': lambda r: r['task'] == 'model_checks',
}

# Named plans of their own, not subsets of the default plan: `--subset NAME` selects one. Each fixes
# its runs, mode and limits; `--mode`, `--footprint-budget-bytes`, `--rss-cap-bytes`, `--budget-s` and
# `--quiet-wait-s` are refused unless they equal the plan's, and `--attribute` is refused.
#
# The headless baseline (DEL-04-05): a timing baseline that can be reproduced from the repository
# alone. It does NOT measure criterion 9's linear workload (the profile linear-ex: seven cases and five
# combinations). W1 is the one-case model (baseline-one-case-ex: linear-ex's rack at 2,000 nodes with
# its operating case alone); W2 is the committed reference system. Each runs five times, alternating
# W1, W2, W1, W2, ..., so that drift in the host's state reaches both alike. Timeouts are limits, not
# expectations; the estimates order and bound the window and are not measurements (W1: the only
# earlier figure for a one-case rack at 2,000 nodes is about a minute of entry, one dirty-built run
# under foreign load; W2 is a 27-node model).
BASELINE_ROWS = (
    # profile, nodes, timeout_s, estimate_s, expect
    ('baseline-one-case-ex', 2000, 300, 90, 'solved'),
    ('reference-system', COMMITTED_MODELS['reference-system']['nodes'], 60, 5, 'solved'),
)
BASELINE_REPEATS = 5
# The default plan's row for criterion 9's linear rack at 2,000 nodes, taken from _DEFAULT, not
# restated: `baseline-with-rack` runs it once, last, to state criterion 9's own linear workload on the
# measured revision beside the baseline. Its expectation is exceeds_base_machine_budget; it is
# recorded as what it is and is never a timing. Its 900 s timeout lets the entry stage finish (the one
# kept record shows entry ending at about 450 s and the footprint watch killing the run soon after).
LINEAR_RACK_2000_ROW = next(row for row in _DEFAULT if row[:2] == ('linear-ex', 2000) and len(row) == 5)
# A PLANNING allowance for the supervisor's own work between runs (hashing the model, the host check,
# the launch and the records), not an established bound on it: nothing measures or enforces it.
RUN_ALLOWANCE_S = 30


def window_budget_for(plan, quiet_wait_s):
    """The quiet-host wait, every timeout and RUN_ALLOWANCE_S per run. Such a window fits every run of
    `plan` at its full timeout PROVIDED the supervisor's own work between runs stays within the
    allowance; where it does not, the supervisor caps a later run's timeout to what remains or skips
    the run, and that run's record says so (`timeout_capped_by_window` true, or
    `skipped_window_budget`), which `--baseline-set` reads. A scope decision, not a necessity: a
    bounded window could instead decline to start a run whose full timeout no longer fits; both
    supervisors are kept as they are by choice."""
    return quiet_wait_s + sum(r['timeout_s'] for r in plan) + RUN_ALLOWANCE_S * len(plan)


def _baseline_rows():
    return BASELINE_ROWS * BASELINE_REPEATS


NAMED_PLANS = {
    # The ten alternating runs of the headless baseline.
    'baseline': _baseline_rows,
    # The same ten, then criterion 9's linear rack at 2,000 nodes once (never a timing).
    'baseline-with-rack': lambda: _baseline_rows() + (LINEAR_RACK_2000_ROW,),
}
NAMED_PLAN_MODE = 'sparse'
NAMED_PLAN_QUIET_WAIT_S = QUIET_WAIT_S


def named_plan(name):
    """(plan, window_budget_s, quiet_wait_s) of a named plan: sparse mode, the 12 GiB footprint budget
    and the 16 GiB emergency cap, run ids made distinct (`repeat_run_ids`) and a window that fits every
    run at its full timeout provided the supervisor's work between runs stays within RUN_ALLOWANCE_S
    (`window_budget_for`)."""
    plan = repeat_run_ids(build_plan(NAMED_PLANS[name](), NAMED_PLAN_MODE))
    return plan, window_budget_for(plan, NAMED_PLAN_QUIET_WAIT_S), NAMED_PLAN_QUIET_WAIT_S


def repeat_run_ids(plan):
    """Each run whose run id occurs more than once in `plan` gets `-r1`, `-r2`, ... in plan order (a run
    that occurs once keeps its id); `check_run_ids` then holds the result to both supervisors' rules."""
    counts, seen, out = {}, {}, []
    for r in plan:
        counts[r['run_id']] = counts.get(r['run_id'], 0) + 1
    for r in plan:
        if counts[r['run_id']] > 1:
            seen[r['run_id']] = seen.get(r['run_id'], 0) + 1
            r = dict(r, run_id='%s-r%d' % (r['run_id'], seen[r['run_id']]))
        out.append(r)
    check_run_ids(out)
    return out


CONDITIONS = (
    'Engine stages only. A solve run: the release_timing driver runs, in one process, the stages the desktop solve '
    'worker runs for an ordinary solve (read, parse, request, entry, serialize, release_envelope, parse_back) and '
    'nothing else; parse_back is a cheaper stand-in for the worker\'s read-back of the result text, so the driver\'s '
    'time and memory from serialize on are lower bounds on the worker\'s, not a measured worker peak. A model-checks '
    'run (task model_checks, run id model-checks-...) is a process of its own: the stages the desktop\'s check worker '
    'runs for the on-demand model checks (read, parse, model_checks, serialize). The desktop app, its IPC and '
    'ingress, the worker process protocol and memory pool, the solve attestation digests and the webview are not '
    'included; the driver lists its own limits in each run.',
    'One driver process at a time, each under /usr/bin/time -l and a per-run timeout. Every 100 ms the runner reads '
    'the driver phys_footprint and lifetime-maximum phys_footprint (proc_pid_rusage RUSAGE_INFO_V4) and its ps RSS; '
    'any limit kills the whole process group.',
    'The footprint budget is the product per-invocation memory budget on the 16 GiB base machine (12 GiB, '
    'memory_pool.rs DEFAULT_INVOCATION_BUDGET_BYTES), enforced as the product enforces it, on phys_footprint: a run '
    'whose sampled footprint or lifetime-maximum footprint exceeds it is killed and classified '
    'exceeds_base_machine_budget (the desktop worker_footprint_exceeded). Like the product, the sampling can miss a '
    'peak shorter than one poll; a completed run whose lifetime-maximum footprint (sampled, the driver reading or '
    '/usr/bin/time peak memory footprint) exceeds the budget is classified the same way, because the desktop would '
    'refuse that result. A killed run has no /usr/bin/time reading, because /usr/bin/time is killed with the '
    'driver\'s process group; each record names the readings its classification used (classification_readings).',
    'The ps RSS watchdog is an emergency cap only (16 GiB by default, the base machine physical memory). RSS can run '
    'well above phys_footprint (about 1.4 times at 500 linear-ex nodes), so it is not the product measure; a kill '
    'there is killed_by_rss_cap unless a footprint reading shows the budget exceeded.',
    'The host is the one described in metadata (memory_bytes, cpu_model); it is not necessarily the 16 GiB base '
    'machine, so memory pressure, paging and compression on a 16 GiB machine are not reproduced.',
    'Models are those the plan names: invented rack models from rack_model.py and, where the plan names it, the '
    'committed invented reference system (fixtures/reference_system/reference_system.json). Each is identified per '
    'run by sha256.',
    'An attribution run (run id ending -attr) takes stack snapshots with macOS sample (1 s at 1 ms every 2 s, and '
    'whenever the footprint has grown by more than 512 MiB since the previous snapshot), each with the footprints at '
    'its start and the top call paths reduced to product frames. Sampling suspends the process at every sample, so '
    'its timings are not measurements; its footprint watchdog and classification are unchanged.',
    'Observation only: no time or memory threshold is asserted, and criterion 9 provisional bounds are not evaluated.',
)


def run_id_of(profile, nodes, mode, attribute=False, task='solve'):
    """`<profile>-n<nodes>-<mode>` for a solve, `model-checks-<profile>-n<nodes>` for the model
    checks (which have no solver mode); `-attr` for an attribution run."""
    if task == 'model_checks':
        return 'model-checks-%s-n%04d%s' % (profile, nodes, '-attr' if attribute else '')
    return '%s-n%04d-%s%s' % (profile, nodes, mode, '-attr' if attribute else '')


def model_name(profile, nodes):
    """A model's file name in the models folder: a rack's `<profile>-n<nodes>.json`, or a committed
    model's own file name."""
    if profile in COMMITTED_MODELS:
        return os.path.basename(COMMITTED_MODELS[profile]['source'])
    return '%s-n%04d.json' % (profile, nodes)


def build_plan(rows, mode='sparse', rss_cap_bytes=EMERGENCY_RSS_CAP_BYTES,
               footprint_budget_bytes=BASE_MACHINE_BUDGET_BYTES):
    """A plan from (profile, nodes, timeout_s, estimate_s, expect[, attribute[, task]]) rows, in row
    order. `task` is `solve` (the default) or `model_checks`; a model-checks run keeps the plan's
    mode in its entry, but the driver is not given it."""
    plan = []
    for i, row in enumerate(rows):
        p, n, t, e, x = row[:5]
        attribute = bool(row[5]) if len(row) > 5 else False
        task = row[6] if len(row) > 6 else 'solve'
        if task not in TASKS:
            raise ValueError('unknown task %r' % task)
        plan.append({'order': i, 'run_id': run_id_of(p, n, mode, attribute, task), 'task': task, 'profile': p,
                     'nodes': n, 'mode': mode, 'timeout_s': t, 'footprint_budget_bytes': footprint_budget_bytes,
                     'rss_cap_bytes': rss_cap_bytes, 'estimate_s': e, 'expect': x, 'attribute': attribute})
    return plan


def with_attribution(plan):
    """`--attribute`: every run of `plan` as an attribution run (one per run id, plan order kept)."""
    out, seen = [], set()
    for r in plan:
        run = dict(r, attribute=True, run_id=run_id_of(r['profile'], r['nodes'], r['mode'], True, r['task']))
        if run['run_id'] not in seen:
            seen.add(run['run_id'])
            out.append(run)
    return out


def default_plan(mode='sparse', rss_cap_bytes=EMERGENCY_RSS_CAP_BYTES, footprint_budget_bytes=BASE_MACHINE_BUDGET_BYTES):
    return build_plan(_DEFAULT, mode, rss_cap_bytes, footprint_budget_bytes)


def select(plan, subset):
    """A named subset, or a comma-separated list of run ids (kept in plan order)."""
    if subset in SUBSETS:
        return [r for r in plan if SUBSETS[subset](r)]
    wanted = [s for s in subset.split(',') if s]
    known = {r['run_id'] for r in plan}
    unknown = [s for s in wanted if s not in known]
    if unknown or not wanted:
        raise ValueError('unknown subset or run ids: %s (subsets: %s)' % (', '.join(unknown) or subset,
                                                                         ', '.join(SUBSETS)))
    return [r for r in plan if r['run_id'] in wanted]


def plan_total_s(plan):
    return sum(r['estimate_s'] for r in plan)


# The plan file the self-supervising driver reads (`release_timing --supervise PLAN --out DIR`).
PLAN_SCHEMA = 'release-timing-plan-v2'
# A run id is a bare name: it names the run's record files inside the output folder, so it is
# refused at plan load unless it matches this closed pattern (no path separator, no leading dot).
RUN_ID_PATTERN = r'[A-Za-z0-9][A-Za-z0-9._-]{0,127}'
_RUN_ID = re.compile(RUN_ID_PATTERN)


# The names `run_plan` and `write_packet` write in the output folder apart from each run's.
METADATA_NAME = 'metadata.json'
PACKET_NAME = 'packet.json'
RESULTS_NAME = 'results'
FOLDER_NAMES = (k6.RECORDS_NAME, METADATA_NAME, PACKET_NAME, RESULTS_NAME)
# An attribution run's raw sample report, `<run_id>.sample-NNN.txt` (`Attributor._take`), and a
# kept result, `results/<run_id>.result.json` (`driver_argv`).
SAMPLE_SUFFIX = '.sample-%03d.txt'
RESULT_SUFFIX = '.result.json'


def names_written(run_ids):
    """Every name the runner writes in the output folder for a plan of `run_ids`: FOLDER_NAMES,
    each run's launch files (`k6.LAUNCH_SUFFIXES`), its first sample report (the later ones differ
    from it only in their digits, which no other suffix ends in) and its kept result."""
    names = list(FOLDER_NAMES)
    for run_id in run_ids:
        names += [run_id + suffix for suffix in k6.LAUNCH_SUFFIXES]
        names += [run_id + SAMPLE_SUFFIX % 0, RESULTS_NAME + '/' + run_id + RESULT_SUFFIX]
    return names


def check_run_ids(plan):
    """Refuses a plan holding a run id that is not a bare name (`RUN_ID_PATTERN`), two run ids equal
    ignoring case (a case-insensitive volume, the macOS default, opens them as one), a run id equal,
    ignoring case, to a name the runner writes, or run ids that make any two names the runner
    writes equal ignoring case (the run id `records` would make its stdout record `records.jsonl`,
    the aggregate record). Run ids are ASCII, so ASCII case folding is the volume's."""
    bad = [r.get('run_id') for r in plan if not isinstance(r.get('run_id'), str) or not _RUN_ID.fullmatch(r['run_id'])]
    if bad:
        raise ValueError('run ids must be bare names (%s): %s' % (RUN_ID_PATTERN, ', '.join(map(repr, bad))))
    ids, folded = [r['run_id'] for r in plan], {}
    for run_id in ids:
        if run_id.lower() in folded:
            raise ValueError('run ids %s and %s are the same ignoring case, so on a case-insensitive volume they '
                             'name the same files' % (folded[run_id.lower()], run_id))
        folded[run_id.lower()] = run_id
        for name in FOLDER_NAMES:
            if name.lower() == run_id.lower():
                raise ValueError('run id %s is, ignoring case, %s, a name the runner writes in the output folder'
                                 % (run_id, name))
    written = [('the runner', name) for name in FOLDER_NAMES]
    for run_id in ids:
        written += [('run ' + run_id, name) for name in names_written([run_id])[len(FOLDER_NAMES):]]
    owners = {}
    for owner, name in written:
        if name.lower() in owners:
            other_owner, other_name = owners[name.lower()]
            raise ValueError("%s's %s and %s's %s are the same file ignoring case; rename the run"
                             % (other_owner, other_name, owner, name))
        owners[name.lower()] = (owner, name)
PLAN_RUN_KEYS = ('order', 'run_id', 'task', 'profile', 'nodes', 'mode', 'timeout_s', 'footprint_budget_bytes',
                 'rss_cap_bytes', 'estimate_s', 'expect', 'attribute')


def plan_document(plan, budget_s=WINDOW_BUDGET_S, quiet_wait_s=QUIET_WAIT_S, models_dir=None, require_sha256=False):
    """`plan` as the plan file the driver's supervise mode reads: each run's plan entry, its model's
    file name, and, where `models_dir`'s manifest lists the model, its sha256 and generator stats.
    That supervisor takes no stack snapshots, so an attribution run is refused here. A committed
    model, and with `require_sha256` every model, must be listed: a run without an expected sha256 is
    not checked by the supervisor."""
    check_run_ids(plan)
    attributed = [r['run_id'] for r in plan if r.get('attribute')]
    if attributed:
        raise ValueError('the self-supervising driver takes no stack snapshots; leave out %s' % ', '.join(attributed))
    manifest = read_manifest(models_dir).get('models', {}) if models_dir else {}
    runs = []
    for r in plan:
        name = model_name(r['profile'], r['nodes'])
        listed = manifest.get(name) or {}
        if not listed.get('sha256') and (require_sha256 or r['profile'] in COMMITTED_MODELS):
            raise ValueError('%s is not in the models manifest with a sha256; run --generate with the same --subset '
                             'and --models first' % name)
        runs.append(dict({k: r[k] for k in PLAN_RUN_KEYS}, model=name, expected_model_sha256=listed.get('sha256'),
                         model_stats=listed.get('stats')))
    return {'schema': PLAN_SCHEMA, 'window_budget_s': budget_s, 'quiet_wait_s': quiet_wait_s, 'runs': runs}


def write_plan(path, plan, budget_s=WINDOW_BUDGET_S, quiet_wait_s=QUIET_WAIT_S, models_dir=None, require_sha256=False):
    document = plan_document(plan, budget_s, quiet_wait_s, models_dir, require_sha256)
    with open(path, 'w') as fh:
        json.dump(document, fh, indent=1, sort_keys=True)
        fh.write('\n')
    return document


def format_plan(plan, budget_s=WINDOW_BUDGET_S, quiet_wait_s=None):
    """The schedule as text; `quiet_wait_s` (given for a named plan) adds the quiet-host wait it carries."""
    out = ['order run_id                        profile       nodes mode    estimate_s timeout_s budget_GiB '
           'rss_cap_GiB expect']
    for r in plan:
        out.append('%5d %-29s %-13s %5d %-7s %10d %9d %10.2f %11.2f %s' % (
            r['order'], r['run_id'], r['profile'], r['nodes'], r['mode'] if r['task'] == 'solve' else '-',
            r['estimate_s'], r['timeout_s'],
            r['footprint_budget_bytes'] / GIB, r['rss_cap_bytes'] / GIB, r['expect']))
    total = plan_total_s(plan)
    out.append('total estimate: %d s (%.1f min); window budget %d s; runs whose estimate exceeds the remaining '
               'budget are skipped, and each timeout is capped at the remaining budget.' % (total, total / 60, budget_s))
    if quiet_wait_s is not None:
        out.append('quiet-host wait: up to %d s before the first run, inside the window budget.' % quiet_wait_s)
        out.append('the window fits every run at its full timeout provided the supervisor\'s own work between runs '
                   '(hashing the model, the host check, the launch, the records) stays within the planning allowance '
                   'of %d s per run; where it does not, the run\'s record says so (timeout_capped_by_window true, or '
                   'skipped_window_budget) and --baseline-set reports that result INCOMPLETE BY THE WINDOW.'
                   % RUN_ALLOWANCE_S)
    out.append('estimates: they order and bound the window; none is a '
               'measurement. budget: the phys_footprint kill (the product budget); rss_cap: the emergency ps RSS kill. '
               'A run id ending -attr takes stack snapshots; its timings are not measurements. A run id starting '
               'model-checks- times the desktop\'s on-demand model checks in a process of its own (no solver mode).')
    return '\n'.join(out) + '\n'


# --------------------------------------------------------------- models


def sha256_file(path):
    digest = hashlib.sha256()
    with open(path, 'rb') as fh:
        for block in iter(lambda: fh.read(1 << 20), b''):
            digest.update(block)
    return digest.hexdigest()


def last_json_line(text):
    objects = k6.jsonl(text.splitlines())
    return objects[-1] if objects else None


def read_manifest(models_dir):
    path = os.path.join(models_dir, 'manifest.json')
    if not os.path.exists(path):
        return {'schema': MANIFEST_SCHEMA, 'models': {}}
    with open(path) as fh:
        return json.load(fh)


def committed_stats(path):
    """A committed model's counts, in the rack generator's stats terms where they apply."""
    with open(path, encoding='utf-8') as fh:
        document = json.load(fh)
    return {'nodes': len(document.get('nodes') or []), 'members': len(document.get('pipe_segments') or []),
            'cases': [c.get('id') for c in document.get('load_cases') or []],
            'combinations': [c.get('id') for c in document.get('combinations') or []],
            'route': 'EX' if 'pressure_contract' in document else 'PF',
            'model_version': document.get('schema_version')}


def copy_committed(profile, path, python=sys.executable, run=subprocess.run, committed=None):
    """A committed model copied to `path` after its generator's `--check` exits 0: (manifest entry,
    None), or (None, why) with nothing copied."""
    spec = (committed or COMMITTED_MODELS)[profile]
    done = run([python, spec['check'], '--check'], capture_output=True, text=True, check=False)
    if done.returncode != 0:
        return None, 'its generator\'s --check exited %s: %s' % (
            done.returncode, k6.sanitize(((done.stdout or '') + (done.stderr or '')).strip()[-500:]))
    source_sha = sha256_file(spec['source'])
    shutil.copyfile(spec['source'], path)
    if sha256_file(path) != source_sha:
        return None, 'the copy differs from the committed file'
    relative = os.path.relpath(spec['source'], REPO) if spec['source'].startswith(REPO + os.sep) else os.path.basename(spec['source'])
    return {'profile': profile, 'nodes': spec['nodes'], 'sha256': source_sha, 'bytes': os.path.getsize(path),
            'source': relative.replace(os.sep, '/'), 'source_check': os.path.basename(spec['check']) + ' --check',
            'stats': committed_stats(path)}, None


def recorded_rack_hashes(path=RACK_HASHES):
    """{(profile, nodes): sha256} of the rack generator's recorded hashes."""
    hashes = {}
    with open(path, encoding='utf-8') as fh:
        for line in fh:
            if line.strip() and not line.startswith('#'):
                digest, profile, nodes = line.split()
                hashes[(profile, int(nodes))] = digest
    return hashes


def pinned_sha256(profile, nodes, hashes, committed=None):
    """(sha256, where it is stated) that a named plan's model must have: a rack's line of the recorded
    hashes, or the committed file's own SHA-256 at this checkout; (None, why) for a rack with no line."""
    committed = committed or COMMITTED_MODELS
    if profile in committed:
        source = committed[profile]['source']
        return sha256_file(source), 'the committed file %s' % os.path.basename(source)
    digest = hashes.get((profile, nodes))
    if digest is None:
        return None, 'no line "%s  %d" in %s' % (profile, nodes, os.path.basename(RACK_HASHES))
    return digest, 'the line "%s  %d" of %s' % (profile, nodes, os.path.basename(RACK_HASHES))


def check_pinned_models(plan, models_dir, hashes_path=RACK_HASHES, committed=None):
    """Refuses (ValueError) a named plan's models folder unless the manifest lists every model of the
    plan with the SHA-256 the repository states for it (`pinned_sha256`) and the model file itself
    has those bytes: a folder produced by any other means, or at another revision, cannot be
    written into a named plan."""
    hashes = recorded_rack_hashes(hashes_path)
    listed = read_manifest(models_dir).get('models', {})
    problems, seen = [], set()
    for r in plan:
        name = model_name(r['profile'], r['nodes'])
        if name in seen:
            continue
        seen.add(name)
        expected, where = pinned_sha256(r['profile'], r['nodes'], hashes, committed)
        found = (listed.get(name) or {}).get('sha256')
        path = os.path.join(models_dir, name)
        if expected is None:
            problems.append('%s: %s' % (name, where))
        elif found != expected:
            problems.append('%s: expected sha256 %s (%s), found %s in the manifest' % (
                name, expected, where, found or 'no entry'))
        elif not os.path.isfile(path):
            problems.append('%s: no such file in the models folder' % name)
        elif sha256_file(path) != expected:
            problems.append('%s: expected sha256 %s (%s), the file in the models folder has %s' % (
                name, expected, where, sha256_file(path)))
    if problems:
        raise ValueError('a named plan holds only the repository\'s own model bytes; refused: ' + '; '.join(problems))


def generate(plan, models_dir, generator=GENERATOR, python=sys.executable, run=subprocess.run, log=print,
             committed=None, pinned_hashes=None):
    """Each distinct (profile, nodes) of the plan, from the rack generator or, for a committed model,
    copied after its generator's check (refused, and dropped from the manifest, when the check fails),
    and the manifest. With `pinned_hashes` (a named plan: the recorded hashes' path), a rack model is
    refused, dropped from the manifest and its file removed, unless it hashes to its recorded line,
    and a profile and size with no line is refused before the generator runs."""
    committed = committed or COMMITTED_MODELS
    hashes = recorded_rack_hashes(pinned_hashes) if pinned_hashes else None
    os.makedirs(models_dir, exist_ok=True)
    manifest = read_manifest(models_dir)
    failures = []
    seen = set()
    for r in plan:
        key = (r['profile'], r['nodes'])
        if key in seen:
            continue
        seen.add(key)
        name = model_name(*key)
        path = os.path.join(models_dir, name)
        if r['profile'] in committed:
            entry, why = copy_committed(r['profile'], path, python=python, run=run, committed=committed)
            if entry is None:
                manifest['models'].pop(name, None)
                failures.append(name)
                log('generate %s: refused: %s' % (name, why))
            else:
                manifest['models'][name] = entry
                log('generate %s: copied, %d bytes, sha256 %s' % (name, entry['bytes'], entry['sha256']))
            continue
        expected = None
        if hashes is not None:
            expected, where = pinned_sha256(r['profile'], r['nodes'], hashes, committed)
            if expected is None:
                manifest['models'].pop(name, None)
                failures.append(name)
                log('generate %s: refused: %s' % (name, where))
                continue
        done = run([python, generator, '--profile', r['profile'], '--nodes', str(r['nodes']), '-o', path],
                   capture_output=True, text=True, check=False)
        if done.returncode != 0 or not os.path.exists(path):
            failures.append(name)
            log('generate %s: failed (exit %s) %s' % (name, done.returncode, k6.sanitize(done.stderr.strip()[-500:])))
            continue
        if expected is not None:
            found = sha256_file(path)
            if found != expected:
                manifest['models'].pop(name, None)
                os.unlink(path)  # written by this call; never left for an unchecked use
                failures.append(name)
                log('generate %s: refused: expected sha256 %s (%s), found %s' % (name, expected, where, found))
                continue
        manifest['models'][name] = {'profile': r['profile'], 'nodes': r['nodes'], 'sha256': sha256_file(path),
                                    'bytes': os.path.getsize(path), 'stats': last_json_line(done.stderr)}
        if expected is not None:
            log('generate %s: %d bytes, sha256 %s, %s' % (name, manifest['models'][name]['bytes'], expected, where))
        else:
            log('generate %s: %d bytes' % (name, manifest['models'][name]['bytes']))
    manifest['schema'] = MANIFEST_SCHEMA
    manifest['generator'] = {'name': os.path.basename(generator), 'sha256': sha256_file(generator)}
    with open(os.path.join(models_dir, 'manifest.json'), 'w') as fh:
        json.dump(manifest, fh, indent=1, sort_keys=True)
        fh.write('\n')
    return manifest, failures


# --------------------------------------------------------------- the baseline set
#
# `--baseline-set OUT --plan-file PLAN --subset NAME`: two results on ONE supervised folder of a named
# plan, read from the records' existing fields; it writes nothing and never combines folders.
#
# - THE TEN-RUN TIMING SET: the runs the plan expects `solved`. "TEN-RUN TIMING SET: COMPLETE",
#   "... INCOMPLETE BY THE WINDOW" or "... FINDINGS".
# - THE RACK OBSERVATION: every run the plan expects not to solve (baseline-with-rack's linear rack).
#   It is never a timing, and it is always judged: "RACK OBSERVATION: OBSERVED AS EXPECTED",
#   "... INCOMPLETE BY THE WINDOW", "... FINDINGS", or "... NONE IN THIS PLAN".
#
# THE EXIT CONTRACT, one status for both results: 1 when ANY finding exists (in either result, or in
# the folder or the plan file); otherwise 3 when either result is INCOMPLETE BY THE WINDOW; otherwise
# 0, which is TEN-RUN TIMING SET: COMPLETE with the rack observation OBSERVED AS EXPECTED or NONE IN
# THIS PLAN. A rack finding does not change the timing set's own result.

SET_EXIT_OK = 0
SET_EXIT_FINDINGS = 1
SET_EXIT_WINDOW = 3   # 2 is argparse's usage error
TIMING_SET = 'TEN-RUN TIMING SET'
RACK_OBSERVATION = 'RACK OBSERVATION'
EXIT_CONTRACT = ('exit 1 when any finding exists (in the ten-run timing set, the rack observation, the folder or '
                 'the plan file); otherwise exit 3 when either result is INCOMPLETE BY THE WINDOW; otherwise exit 0: '
                 'TEN-RUN TIMING SET: COMPLETE with RACK OBSERVATION: OBSERVED AS EXPECTED or NONE IN THIS PLAN')


def _number(value):
    return isinstance(value, (int, float)) and not isinstance(value, bool)


def read_set_records(out_dir):
    """(records, findings) of OUT's aggregate record, one JSON object per line, in order."""
    path = os.path.join(out_dir, k6.RECORDS_NAME)
    if not os.path.isfile(path):
        return [], ['%s: no aggregate record in the folder' % k6.RECORDS_NAME]
    with open(path, encoding='utf-8') as fh:
        return records_of(fh.read())


def records_of(text):
    """(records, findings) of an aggregate record's text, one JSON object per line, in order."""
    records, findings = [], []
    for number, line in enumerate(text.splitlines(), 1):
        if not line.strip():
            continue
        try:
            record = json.loads(line)
        except json.JSONDecodeError:
            findings.append('%s line %d does not parse' % (k6.RECORDS_NAME, number))
            continue
        if isinstance(record, dict):
            records.append(record)
        else:
            findings.append('%s line %d is not a record' % (k6.RECORDS_NAME, number))
    return records, findings


def _outcome(rec):
    """A run's own outcome as its record states it."""
    return {'classification': rec.get('classification'), 'classification_detail': rec.get('classification_detail'),
            'last_stage': rec.get('last_stage'), 'as_expected': rec.get('as_expected')}


def _window_problem(rec):
    classification = rec.get('classification')
    if classification == 'skipped_window_budget':
        return 'skipped_window_budget'
    if classification == 'timed_out' and rec.get('timeout_capped_by_window') is True:
        return 'timed_out under a timeout the window capped to %s s' % rec.get('timeout_applied_s')
    return None


def judge_set(records, plan_runs, folder_findings=()):
    """The two results on one folder's `records` against the plan file's `plan_runs`.

    The ten-run timing set: each (profile, nodes) the plan expects `solved` is a workload. A run is a
    completed timing only if: `classification` is `ok` and `as_expected` is true; `driver.route` is
    `EX`; the plan's `expected_model_sha256`, the record's `model_sha256` and `driver.document.sha256`
    are one value; `driver.seconds.entry` and `launch.rss.exit_lifetime_max_phys_footprint_bytes` are
    numbers; and `driver.output_sha256`, `driver.rows` and `driver.output_bytes` are present and equal
    those of the workload's other completed runs. A skip for the window, or a timeout under a capped
    timeout, is a window problem; any other way of not being a completed timing is a finding.

    The rack observation: each run the plan expects otherwise must be OBSERVED: a record, a launch, the
    plan's model bytes. Its outcome (classification, detail, last stage) is reported, and it is a
    success only where the declared expectation is met (`as_expected` true). A missing record, no
    launch (`model_missing`, `model_mismatch`, an error before any driver started), other model bytes,
    or an outcome that does not meet the expectation is a finding; a window problem is a window
    problem. Its exclusion from the timing statistics is never exclusion from the findings.

    `folder_findings` (the folder or the plan file) make both results FINDINGS. `host_not_quiet` lists
    the launched runs whose host check was not quiet or read no memory level; it is reported for a
    measurement contract's rules and changes neither result."""
    folder_findings = list(folder_findings)
    timing_findings, timing_window, rack_findings, rack_window, rack_runs = [], [], [], [], []
    by_id = {}
    for r in records:
        run_id = r.get('run_id')
        if run_id in by_id:
            folder_findings.append('%s: more than one record' % run_id)
        by_id[run_id] = r
    planned = {r.get('run_id') for r in plan_runs}
    for run_id in by_id:
        if run_id not in planned:
            folder_findings.append('%s: a record of a run the plan does not hold' % run_id)
    workloads = {}
    for p in plan_runs:
        run_id = p.get('run_id')
        rec = by_id.get(run_id)
        if p.get('expect') != 'solved':
            row = {'run_id': run_id, 'expect': p.get('expect'), 'observed': False, 'classification': None,
                   'classification_detail': None, 'last_stage': None, 'as_expected': None}
            rack_runs.append(row)
            if rec is None:
                row['result'] = 'FINDING'
                rack_findings.append('%s: no record: the run was not observed' % run_id)
                continue
            row.update(_outcome(rec))
            window = _window_problem(rec)
            if window:
                # A capped timeout was launched and observed (its outcome is the timeout); a skip was not.
                row.update(observed='launch' in rec, result='INCOMPLETE BY THE WINDOW')
                rack_window.append('%s: %s' % (run_id, window))
            elif 'launch' not in rec or rec.get('classification') in ('model_missing', 'model_mismatch'):
                row['result'] = 'FINDING'
                rack_findings.append('%s: not observed: no driver was started (classification %s%s)' % (
                    run_id, rec.get('classification'), ', %s' % rec.get('classification_detail')
                    if rec.get('classification_detail') else ''))
            elif rec.get('model_sha256') is None or rec.get('model_sha256') != p.get('expected_model_sha256'):
                row['result'] = 'FINDING'
                rack_findings.append('%s: observed on model sha256 %s, not the plan\'s %s' % (
                    run_id, rec.get('model_sha256'), p.get('expected_model_sha256')))
            elif rec.get('as_expected') is not True:
                row.update(observed=True, result='FINDING')
                rack_findings.append('%s: observed, but its outcome %s%s does not meet its declared expectation %s' % (
                    run_id, rec.get('classification'), ' (%s)' % rec.get('classification_detail')
                    if rec.get('classification_detail') else '', p.get('expect')))
            else:
                row.update(observed=True, result='OBSERVED AS EXPECTED')
            continue
        entry = workloads.setdefault('%s n%s' % (p.get('profile'), p.get('nodes')), {'planned': [], 'candidates': []})
        entry['planned'].append(run_id)
        if rec is None:
            timing_findings.append('%s: no record (the plan was not run to its end)' % run_id)
            continue
        window = _window_problem(rec)
        if window:
            timing_window.append('%s: %s' % (run_id, window))
            continue
        classification = rec.get('classification')
        driver = rec.get('driver') if isinstance(rec.get('driver'), dict) else {}
        reasons = []
        if classification != 'ok':
            reasons.append('classification %s%s' % (classification, ' (%s)' % rec.get('classification_detail')
                                                     if rec.get('classification_detail') else ''))
            if classification == 'timed_out':
                reasons.append('timed out at its full timeout')
        if rec.get('as_expected') is not True:
            reasons.append('as_expected is %r' % rec.get('as_expected'))
        if driver.get('route') != 'EX':
            reasons.append('driver.route is %r, not EX' % driver.get('route'))
        hashes = (p.get('expected_model_sha256'), rec.get('model_sha256'), (driver.get('document') or {}).get('sha256'))
        if None in hashes or len(set(hashes)) != 1:
            reasons.append('the plan\'s expected_model_sha256, the record\'s model_sha256 and driver.document.sha256 '
                           'are not one value: %s' % ', '.join(map(str, hashes)))
        entry_s = (driver.get('seconds') or {}).get('entry')
        if not _number(entry_s):
            reasons.append('driver.seconds.entry is %r' % entry_s)
        footprint = ((rec.get('launch') or {}).get('rss') or {}).get('exit_lifetime_max_phys_footprint_bytes')
        if not _number(footprint):
            reasons.append('launch.rss.exit_lifetime_max_phys_footprint_bytes is %r' % footprint)
        product = (driver.get('output_sha256'), driver.get('rows'), driver.get('output_bytes'))
        if None in product:
            reasons.append('driver.output_sha256, driver.rows and driver.output_bytes are not all present: %s'
                           % ', '.join(map(str, product)))
        if reasons:
            timing_findings.append('%s: %s' % (run_id, '; '.join(reasons)))
        else:
            entry['candidates'].append((run_id, entry_s, product))
    stats = {}
    for name, entry in workloads.items():
        products = {product for _, _, product in entry['candidates']}
        completed = entry['candidates']
        if len(products) > 1:
            timing_findings.append('%s: output_sha256, rows and output_bytes disagree across its runs: %s' % (
                name, '; '.join('%s %s' % (run_id, list(product)) for run_id, _, product in entry['candidates'])))
            completed = []
        seconds = [s for _, s, _ in completed]
        stats[name] = {'planned': len(entry['planned']), 'completed': len(completed),
                       'completed_run_ids': [run_id for run_id, _, _ in completed],
                       'entry_seconds': seconds,
                       'median_entry_s': statistics.median(seconds) if seconds else None,
                       'min_entry_s': min(seconds) if seconds else None,
                       'max_entry_s': max(seconds) if seconds else None,
                       'output': list(next(iter(products))) if len(products) == 1 else None}
    if not timing_findings and not timing_window and not (
            stats and all(w['completed'] == w['planned'] for w in stats.values())):
        timing_findings.append('the timing set has a workload with fewer completed timings than planned runs, or none')
    if folder_findings or timing_findings:
        timing_result = 'FINDINGS'
    elif timing_window:
        timing_result = 'INCOMPLETE BY THE WINDOW'
    else:
        timing_result = 'COMPLETE'
    if folder_findings or rack_findings:   # a folder finding puts the plan itself in doubt
        rack_result = 'FINDINGS'
    elif not rack_runs:
        rack_result = 'NONE IN THIS PLAN'
    elif rack_window:
        rack_result = 'INCOMPLETE BY THE WINDOW'
    else:
        rack_result = 'OBSERVED AS EXPECTED'
    if folder_findings or timing_findings or rack_findings:
        code = SET_EXIT_FINDINGS
    elif timing_window or rack_window:
        code = SET_EXIT_WINDOW
    else:
        code = SET_EXIT_OK
    host_not_quiet = [r.get('run_id') for r in records if isinstance(r.get('host'), dict) and not (
        r['host'].get('quiet') is True and _number(r['host'].get('memorystatus_level')))]
    timing_line, rack_line = '%s: %s' % (TIMING_SET, timing_result), '%s: %s' % (RACK_OBSERVATION, rack_result)
    return {'results': [timing_line, rack_line], 'exit_code': code, 'exit_contract': EXIT_CONTRACT,
            'folder_findings': folder_findings,
            'timing_set': {'result': timing_line, 'workloads': stats, 'findings': timing_findings,
                           'window_problems': timing_window},
            'rack_observation': {'result': rack_line, 'runs': rack_runs, 'findings': rack_findings,
                                 'window_problems': rack_window},
            'host_not_quiet': host_not_quiet}


def baseline_set(out_dir, plan_path, name, committed=None):
    """`judge_set` on OUT's records with PLAN, after holding PLAN to the named plan `name` at this
    checkout: every top-level field, every field of each run but expected_model_sha256 and
    model_stats, and each run's expected_model_sha256 the repository's own (`pinned_sha256`). A
    difference is a folder finding, which makes both results FINDINGS. Reads only; writes nothing."""
    records, findings = read_set_records(out_dir)
    try:
        with open(plan_path, encoding='utf-8') as fh:
            document = json.load(fh)
        document.get('runs')
    except (OSError, ValueError, AttributeError) as error:
        return dict(judge_set(records, [], findings + ['the plan file cannot be read: %s' % error]), subset=name)
    return baseline_set_of(records, document, name, findings, committed)


def baseline_set_of(records, document, name, findings=(), committed=None):
    """`baseline_set` on records and a plan document already read (`--read-back` recomputes it from the
    bytes it has verified). The report names its named plan, `subset`."""
    findings = list(findings)
    plan_runs = document.get('runs') or []
    named, budget_s, quiet_wait_s = named_plan(name)
    # Every top-level field of the plan document, and every field of each run but the two the models
    # folder supplies (expected_model_sha256, checked against the repository below, and model_stats,
    # the generator's description of the model).
    expected_top = {'schema': PLAN_SCHEMA, 'window_budget_s': budget_s, 'quiet_wait_s': quiet_wait_s}
    differing = sorted(key for key in set(document) | set(expected_top) | {'runs'}
                       if key != 'runs' and document.get(key) != expected_top.get(key))
    keys = PLAN_RUN_KEYS + ('model',)
    if [{k: r.get(k) for k in keys} for r in plan_runs] != [dict({k: r[k] for k in PLAN_RUN_KEYS}, model=model_name(
            r['profile'], r['nodes'])) for r in named] or any(
            set(r) - set(keys) - {'expected_model_sha256', 'model_stats'} for r in plan_runs):
        differing.append('runs')
    if differing:
        findings.append('the plan file is not the named plan %s at this checkout: %s differ%s' % (
            name, ', '.join(differing), '' if len(differing) > 1 else 's'))
    hashes = recorded_rack_hashes()
    for r in plan_runs:
        expected, where = pinned_sha256(r.get('profile'), r.get('nodes'), hashes, committed)
        if r.get('expected_model_sha256') != expected:
            findings.append('%s: the plan\'s expected_model_sha256 %s is not %s (%s)' % (
                r.get('run_id'), r.get('expected_model_sha256'), expected, where))
    return dict(judge_set(records, plan_runs, findings), subset=name)


# --------------------------------------------------------------- footprint


class RusageInfoV4(ctypes.Structure):
    """struct rusage_info_v4 (<sys/resource.h>), filled by proc_pid_rusage(pid, RUSAGE_INFO_V4, ...)."""
    _fields_ = [('ri_uuid', ctypes.c_uint8 * 16)] + [(name, ctypes.c_uint64) for name in (
        'ri_user_time', 'ri_system_time', 'ri_pkg_idle_wkups', 'ri_interrupt_wkups', 'ri_pageins',
        'ri_wired_size', 'ri_resident_size', 'ri_phys_footprint', 'ri_proc_start_abstime',
        'ri_proc_exit_abstime', 'ri_child_user_time', 'ri_child_system_time', 'ri_child_pkg_idle_wkups',
        'ri_child_interrupt_wkups', 'ri_child_pageins', 'ri_child_elapsed_abstime', 'ri_diskio_bytesread',
        'ri_diskio_byteswritten', 'ri_cpu_time_qos_default', 'ri_cpu_time_qos_maintenance',
        'ri_cpu_time_qos_background', 'ri_cpu_time_qos_utility', 'ri_cpu_time_qos_legacy',
        'ri_cpu_time_qos_user_initiated', 'ri_cpu_time_qos_user_interactive', 'ri_billed_system_time',
        'ri_serviced_system_time', 'ri_logical_writes', 'ri_lifetime_max_phys_footprint', 'ri_instructions',
        'ri_cycles', 'ri_billed_energy', 'ri_serviced_energy', 'ri_interval_max_phys_footprint',
        'ri_runnable_time')]


RUSAGE_INFO_V4 = 4
_LIBPROC = []


def _proc_pid_rusage():
    """libproc's proc_pid_rusage (in libSystem on macOS), or None elsewhere."""
    if not _LIBPROC:
        function = None
        if sys.platform == 'darwin':
            try:
                library = ctypes.CDLL(ctypes.util.find_library('proc') or '/usr/lib/libSystem.B.dylib', use_errno=True)
                function = library.proc_pid_rusage
                function.argtypes = (ctypes.c_int, ctypes.c_int, ctypes.POINTER(RusageInfoV4))
                function.restype = ctypes.c_int
            except (OSError, AttributeError):
                function = None
        _LIBPROC.append(function)
    return _LIBPROC[0]


def read_footprint(pid):
    """(phys_footprint, lifetime_max_phys_footprint) of process `pid` in bytes, as the desktop's
    worker_alloc.rs reads them; None where unavailable or once the process is gone."""
    function = _proc_pid_rusage()
    if function is None:
        return None
    info = RusageInfoV4()
    if function(int(pid), RUSAGE_INFO_V4, ctypes.byref(info)) != 0:
        return None
    return info.ri_phys_footprint, info.ri_lifetime_max_phys_footprint


class FootprintSampler:
    """The product's budget, emulated: a `k6_runner.launch` sampler that reads the driver's
    phys_footprint and lifetime-maximum phys_footprint every poll and asks for a kill when either
    exceeds `budget_bytes` (the desktop kills a worker whose footprint exceeds the budget, and
    refuses a result whose lifetime maximum did: `worker_footprint_exceeded`)."""

    def __init__(self, budget_bytes, read=read_footprint, attributor=None):
        self.budget_bytes = budget_bytes
        self.read = read
        self.attributor = attributor
        self.samples = []
        self.failed_reads = 0
        self.peak_footprint = None
        self.peak_lifetime_max = None
        self.kill_reason = None
        self.kill_footprint = None

    def __call__(self, pid, now):
        reading = self.read(pid)
        if reading is None:
            self.failed_reads += 1
            return False
        footprint, lifetime = reading
        self.samples.append([now, footprint, lifetime])
        self.peak_footprint = max(footprint, self.peak_footprint or 0)
        self.peak_lifetime_max = max(lifetime, self.peak_lifetime_max or 0)
        over = max(footprint, lifetime)
        if over > self.budget_bytes:
            self.kill_footprint = over
            self.kill_reason = 'footprint_%d_above_budget_%d' % (over, self.budget_bytes)
            return True
        if self.attributor is not None:
            self.attributor.poll(pid, now, footprint, lifetime)
        return False

    def summary(self):
        return {'kind': 'phys_footprint', 'budget_bytes': self.budget_bytes, 'samples': self.samples,
                'sample_columns': ['t_s', 'phys_footprint_bytes', 'lifetime_max_phys_footprint_bytes'],
                'failed_reads': self.failed_reads, 'peak_phys_footprint_bytes': self.peak_footprint,
                'peak_lifetime_max_phys_footprint_bytes': self.peak_lifetime_max,
                'kill_reason': self.kill_reason, 'kill_footprint_bytes': self.kill_footprint}


# --------------------------------------------------------------- attribution
#
# An attribution run takes short stack snapshots of the driver with macOS `sample` while it runs, so
# that footprint growth can be read against the code that was running. Sampling suspends the process
# at every sample, so an attribution run's timings are not measurements.

SAMPLE_TOOL = '/usr/bin/sample'
SNAPSHOT_INTERVAL_S = 2.0
SNAPSHOT_GROWTH_BYTES = 512 * k6.MIB
SNAPSHOT_DURATION_S = 1      # `sample` takes whole seconds (a fraction samples once)
SNAPSHOT_SAMPLING_MS = 1
SNAPSHOT_TOP_PATHS = 5
SNAPSHOT_PATH_FRAMES = 12
PRODUCT_FRAME = 'open_pipe_stress_'
_SAMPLE_LINE = re.compile(r'^(?P<lead>[ +!:|]*?)(?P<count>\d+) (?P<rest>\S.*)$')
_RUST_HASH = re.compile(r'::h[0-9a-f]{16}$')


def demangle(symbol):
    """A frame's symbol, readable: legacy Rust mangling (`_ZN...E`) decoded, the `::h<hash>` suffix
    and the `+ offset` dropped. `sample` usually prints Rust symbols demangled already."""
    symbol = symbol.strip()
    name = symbol.lstrip('_')
    if name.startswith('ZN') and name.endswith('E'):
        parts, i, body = [], 0, name[2:-1]
        while i < len(body) and body[i].isdigit():
            j = i
            while j < len(body) and body[j].isdigit():
                j += 1
            length = int(body[i:j])
            parts.append(body[j:j + length])
            i = j + length
        if parts and i == len(body):
            symbol = '::'.join(parts)
            for code, text in (('$LT$', '<'), ('$GT$', '>'), ('$u20$', ' '), ('$C$', ','), ('$RF$', '&'),
                               ('..', '::')):
                symbol = symbol.replace(code, text)
    return _RUST_HASH.sub('', symbol)


def parse_sample(text, frame_pattern=PRODUCT_FRAME, top=SNAPSHOT_TOP_PATHS, frames=SNAPSHOT_PATH_FRAMES):
    """A `sample` report's call graph, reduced: (total samples, the top call paths). Each sample's
    stack is reduced to its product frames (symbols containing `frame_pattern`, outermost first; at
    most `frames`, the innermost kept); paths are merged by those frames and ranked by samples, with
    the leaf frame (where the samples landed) that carried most of them."""
    lines = text.splitlines()
    try:
        start = next(i for i, line in enumerate(lines) if line.strip() == 'Call graph:') + 1
    except StopIteration:
        return 0, []
    nodes = []          # (depth, count, symbol, parent index)
    stack = []          # indices of the open ancestors
    for line in lines[start:]:
        if not line.strip():
            break
        m = _SAMPLE_LINE.match(line)
        if not m:
            continue
        depth = len(m.group('lead'))
        rest = m.group('rest')
        symbol = rest.split('  (in ', 1)[0] if '  (in ' in rest else rest.split('  ', 1)[0]
        while stack and nodes[stack[-1]][0] >= depth:
            stack.pop()
        nodes.append((depth, int(m.group('count')), demangle(symbol), stack[-1] if stack else None))
        stack.append(len(nodes) - 1)
    children = {}
    for index, node in enumerate(nodes):
        if node[3] is not None:
            children[node[3]] = children.get(node[3], 0) + node[1]
    total = sum(node[1] for node in nodes if node[3] is None)
    merged = {}
    for index, node in enumerate(nodes):
        own = node[1] - children.get(index, 0)
        if own <= 0:
            continue
        chain, cursor = [], index
        while cursor is not None:
            chain.append(nodes[cursor][2])
            cursor = nodes[cursor][3]
        chain.reverse()
        product = [frame for frame in chain if frame_pattern in frame]
        key = tuple(product[-frames:])
        entry = merged.setdefault(key, {'count': 0, 'truncated': len(product) > frames, 'leaves': {}})
        entry['count'] += own
        entry['leaves'][node[2]] = entry['leaves'].get(node[2], 0) + own
    ranked = sorted(merged.items(), key=lambda item: (-item[1]['count'], item[0]))[:top]
    paths = []
    for key, entry in ranked:
        leaf, leaf_count = max(entry['leaves'].items(), key=lambda item: (item[1], item[0]))
        paths.append({'count': entry['count'], 'frames': list(key) or ['(no product frame)'],
                      'truncated': entry['truncated'], 'leaf': leaf, 'leaf_count': leaf_count})
    return total, paths


class Attributor:
    """Stack snapshots of a running driver: one every `interval_s`, and one whenever its
    phys_footprint has grown by more than `growth_bytes` since the previous snapshot. Each snapshot
    runs `sample <pid> <duration_s> <sampling_ms>` in a background thread (the footprint watchdog's
    polls continue meanwhile; one snapshot at a time) and keeps the time, the footprints at its start
    and the top product call paths. Raw reports are kept, sanitized, only with `keep_raw`."""

    def __init__(self, record_dir, run_id, keep_raw=False, tool=SAMPLE_TOOL, interval_s=SNAPSHOT_INTERVAL_S,
                 growth_bytes=SNAPSHOT_GROWTH_BYTES, duration_s=SNAPSHOT_DURATION_S,
                 sampling_ms=SNAPSHOT_SAMPLING_MS, frame_pattern=PRODUCT_FRAME, run=subprocess.run):
        self.record_dir, self.run_id, self.keep_raw, self.tool = record_dir, run_id, keep_raw, tool
        self.interval_s, self.growth_bytes = interval_s, growth_bytes
        self.duration_s, self.sampling_ms, self.frame_pattern, self.run = duration_s, sampling_ms, frame_pattern, run
        self.snapshots, self.lock, self.thread = [], threading.Lock(), None
        self.last_start, self.last_footprint, self.baseline = None, None, None

    def poll(self, pid, now, footprint, lifetime):
        if self.baseline is None:
            self.baseline = footprint
        if self.thread is not None and self.thread.is_alive():
            return
        reference = self.last_footprint if self.last_footprint is not None else self.baseline
        if footprint - reference > self.growth_bytes:
            trigger = 'growth'
        elif now - (self.last_start if self.last_start is not None else 0.0) >= self.interval_s:
            trigger = 'interval'
        else:
            return
        self.last_start, self.last_footprint = now, footprint
        snapshot = {'index': len(self.snapshots), 'trigger': trigger, 't_s': now, 'phys_footprint_bytes': footprint,
                    'lifetime_max_phys_footprint_bytes': lifetime, 'sample_seconds': self.duration_s,
                    'sampling_ms': self.sampling_ms}
        with self.lock:
            self.snapshots.append(snapshot)
        self.thread = threading.Thread(target=self._take, args=(pid, snapshot), daemon=True)
        self.thread.start()

    def _take(self, pid, snapshot):
        path = os.path.join(self.record_dir, self.run_id + SAMPLE_SUFFIX % snapshot['index'])
        try:
            done = self.run([self.tool, str(pid), str(self.duration_s), str(self.sampling_ms), '-file', path],
                            capture_output=True, text=True, timeout=self.duration_s + 60, check=False)
            text = ''
            if os.path.exists(path):
                with open(path, errors='replace') as fh:
                    text = fh.read()
            total, paths = parse_sample(text, self.frame_pattern)
            snapshot.update(samples=total, paths=paths)
            if not total:
                snapshot['error'] = k6.sanitize(((done.stderr or '') + (done.stdout or '')).strip()[-300:]) or 'no samples'
        except (OSError, subprocess.SubprocessError) as error:
            snapshot.update(samples=0, paths=[], error=k6.sanitize(str(error))[-300:])
            text = ''
        if os.path.exists(path):
            if self.keep_raw:
                with open(path, 'w') as fh:
                    fh.write(k6.sanitize(text))
                snapshot['raw'] = os.path.basename(path)
            else:
                os.unlink(path)

    def finish(self, timeout_s=None):
        """Waits for a snapshot still in flight (its target has usually exited by then)."""
        if self.thread is not None:
            self.thread.join(timeout=timeout_s if timeout_s is not None else self.duration_s + 65)

    def summary(self):
        with self.lock:
            snapshots = [dict(s) for s in self.snapshots]
        return {'tool': os.path.basename(self.tool), 'interval_s': self.interval_s, 'growth_bytes': self.growth_bytes,
                'sample_seconds': self.duration_s, 'sampling_ms': self.sampling_ms,
                'frame_pattern': self.frame_pattern, 'top_paths': SNAPSHOT_TOP_PATHS,
                'path_frames': SNAPSHOT_PATH_FRAMES, 'raw_kept': self.keep_raw, 'snapshots': snapshots}


def progress_lines(stderr_text):
    """The driver's `release-timing-stage {json}` progress lines, in order."""
    out = []
    for line in stderr_text.splitlines():
        if line.startswith(PROGRESS_PREFIX):
            try:
                out.append(json.loads(line[len(PROGRESS_PREFIX):]))
            except json.JSONDecodeError:
                pass
    return out


def last_progress(record_dir, run_id):
    """(last_stage, last_stage_memory, stages_finished) from the run's stderr record."""
    path = os.path.join(record_dir, run_id + k6.STDERR_SUFFIX)
    if not os.path.exists(path):
        return None, None, 0
    with open(path) as fh:
        lines = progress_lines(fh.read())
    if not lines:
        return None, None, 0
    last = lines[-1]
    stage = {'repeat': last.get('repeat'), 'stage': last.get('stage'), 'seconds': last.get('seconds')}
    memory = {k: last.get(k) for k in ('ru_maxrss_bytes', 'phys_footprint_bytes', 'lifetime_max_phys_footprint_bytes')}
    return stage, memory, len(lines)


# --------------------------------------------------------------- classification

CLASSES = ('ok', 'refused', 'timed_out', 'exceeds_base_machine_budget', 'killed_by_rss_cap', 'entry_error',
           'error', 'skipped_window_budget', 'model_missing', 'model_mismatch')


def peak_footprint(driver, launch_record, progress_memory=None):
    """The largest phys_footprint reading seen: the footprint watchdog's samples (current and
    lifetime maximum), the driver's own reading (the desktop's worker_peak_footprint_bytes
    measure) or its last progress line, and /usr/bin/time's peak memory footprint."""
    launch_record = launch_record or {}
    sampler = launch_record.get('sampler') or {}
    values = [sampler.get('peak_phys_footprint_bytes'), sampler.get('peak_lifetime_max_phys_footprint_bytes'),
              (launch_record.get('rss') or {}).get('time_peak_footprint_bytes')]
    if driver:
        values.append(driver.get('driver_peak_footprint_bytes'))
    if progress_memory:
        values += [progress_memory.get('phys_footprint_bytes'), progress_memory.get('lifetime_max_phys_footprint_bytes')]
    values = [v for v in values if isinstance(v, int) and not isinstance(v, bool)]
    return max(values) if values else None


def _is_int(value):
    return isinstance(value, int) and not isinstance(value, bool)


def footprint_readings(driver, launch_record, progress_memory=None):
    """The readings `peak_footprint` found, each named `source:key` (`sampler`, `rss`, `driver`,
    `progress`), in its order. The self-supervising driver ports this (`classify.rs`) and has one
    more source, `rss:exit_lifetime_max_phys_footprint_bytes`, its reading of the driver at exit."""
    launch_record = launch_record or {}
    sampler = launch_record.get('sampler') or {}
    rss = launch_record.get('rss') or {}
    candidates = [('sampler', sampler, 'peak_phys_footprint_bytes'),
                  ('sampler', sampler, 'peak_lifetime_max_phys_footprint_bytes'),
                  ('rss', rss, 'time_peak_footprint_bytes'),
                  ('rss', rss, 'exit_lifetime_max_phys_footprint_bytes')]
    if driver:
        candidates.append(('driver', driver, 'driver_peak_footprint_bytes'))
    if progress_memory:
        candidates += [('progress', progress_memory, 'phys_footprint_bytes'),
                       ('progress', progress_memory, 'lifetime_max_phys_footprint_bytes')]
    return ['%s:%s' % (source, key) for source, values, key in candidates if _is_int(values.get(key))]


def classification_readings(launch_record, driver, progress_memory=None):
    """The footprint readings `classify` compared with the budget to reach its classification, in
    its branch order (none where no footprint decided it: a timeout, no driver line, an entry error
    or a failed exit)."""
    k6_class = launch_record.get('classification')
    if k6_class == 'killed_by_sampler':
        kill = (launch_record.get('sampler') or {}).get('kill_footprint_bytes')
        return ['sampler:kill_footprint_bytes'] if _is_int(kill) else []
    if k6_class in ('killed_by_rss_watchdog', 'rlimit_abort', 'heap_cap_abort'):
        return footprint_readings(driver, launch_record, progress_memory)
    if k6_class == 'timed_out' or driver is None:
        return []
    if driver.get('status') == 'entry_error' or launch_record.get('exit_code') != 0:
        return []
    return footprint_readings(driver, launch_record, progress_memory)


KILLED_CLASSES = ('killed_by_sampler', 'killed_by_rss_watchdog', 'timed_out', 'heap_cap_abort', 'rlimit_abort')


def readings_record(launch_record, driver, progress_memory=None):
    """The record's `classification_readings`: `used`, the readings the classification compared
    with the budget; `at_exit`, this runner's reading of the driver at its exit (/usr/bin/time's peak
    memory footprint, `rss:time_peak_footprint_bytes`), or None with `at_exit_absent` saying why. A
    killed run has none: /usr/bin/time is killed with the driver's process group."""
    at_exit = 'rss:time_peak_footprint_bytes' if _is_int(
        (launch_record.get('rss') or {}).get('time_peak_footprint_bytes')) else None
    absent = None
    if at_exit is None:
        if not launch_record.get('wrapper'):
            absent = 'the driver did not run under /usr/bin/time'
        elif launch_record.get('classification') in KILLED_CLASSES:
            absent = 'the run was killed with its process group, /usr/bin/time with it, so there is no reading at exit'
        else:
            absent = '/usr/bin/time reported no peak memory footprint'
    return {'used': classification_readings(launch_record, driver, progress_memory), 'at_exit': at_exit,
            'at_exit_absent': absent}


def classify(launch_record, driver, rss_cap_bytes, budget_bytes=BASE_MACHINE_BUDGET_BYTES, progress_memory=None):
    """(classification, detail) of one launched run."""
    k6_class = launch_record.get('classification')
    if k6_class == 'killed_by_sampler':
        # The footprint watchdog: the product's own measure above the budget (worker_footprint_exceeded).
        footprint = (launch_record.get('sampler') or {}).get('kill_footprint_bytes')
        return 'exceeds_base_machine_budget', 'footprint_%s_above_budget_%d_killed' % (footprint, budget_bytes)
    if k6_class in ('killed_by_rss_watchdog', 'rlimit_abort', 'heap_cap_abort'):
        # The emergency RSS cap; RSS runs above phys_footprint, so only a footprint reading above the
        # budget makes the kill `exceeds_base_machine_budget`.
        footprint = peak_footprint(driver, launch_record, progress_memory)
        if footprint is not None and footprint > budget_bytes:
            return 'exceeds_base_machine_budget', 'killed_above_rss_cap_%d_footprint_%d' % (rss_cap_bytes, footprint)
        return 'killed_by_rss_cap', 'killed_above_rss_cap_%d_footprint_%s' % (
            rss_cap_bytes, 'unread' if footprint is None else '%d_within_budget' % footprint)
    if k6_class == 'timed_out':
        return 'timed_out', 'after_%s_s' % launch_record.get('timeout_s')
    if driver is None:
        return 'error', 'no_driver_line_exit_%s' % launch_record.get('exit_code')
    status = driver.get('status')
    if status == 'entry_error':
        error = driver.get('entry_error') or {}
        return 'entry_error', '%s: %s' % (error.get('stage'), str(error.get('message'))[:300])
    if launch_record.get('exit_code') != 0:
        return 'error', 'exit_code_%s' % launch_record.get('exit_code')
    footprint = peak_footprint(driver, launch_record, progress_memory)
    if footprint is not None and footprint > budget_bytes:
        return 'exceeds_base_machine_budget', 'footprint_%d_above_budget_at_result' % footprint
    if status in (SOLVED, CHECKS_REPORTED):
        return 'ok', None
    codes = driver.get('blocking_codes') or []
    if codes:
        return 'refused', ','.join(codes)
    return 'error', 'status_%s' % status


def as_expected(expect, classification, detail):
    """Whether the outcome is the plan's expectation (an observation, not a pass/fail)."""
    if expect == 'solved':
        return classification == 'ok'
    if expect.startswith('refused:'):
        return classification == 'refused' and expect.split(':', 1)[1] in (detail or '').split(',')
    if expect in CLASSES:
        # Any plain classification name (refused, exceeds_base_machine_budget, timed_out, ...): exact.
        return classification == expect
    return None


# --------------------------------------------------------------- run


def driver_line(record_dir, run_id):
    path = os.path.join(record_dir, run_id + k6.STDOUT_SUFFIX)
    if not os.path.exists(path):
        return None
    with open(path) as fh:
        for obj in k6.jsonl(fh):
            if obj.get('schema') == DRIVER_SCHEMA:
                return obj
    return None


def driver_argv(binary, model_path, run, results_dir=None):
    """A solve run's model and `--mode`, or a model-checks run's model and `--model-checks`, then
    `--write` when results are kept."""
    if run.get('task', 'solve') == 'model_checks':
        argv = [binary, model_path, '--model-checks']
    else:
        argv = [binary, model_path, '--mode', run['mode']]
    if results_dir:
        argv += ['--write', os.path.join(results_dir, run['run_id'] + RESULT_SUFFIX)]
    return argv


def git(args, cwd=REPO):
    done = k6.run_quiet(['git', '-C', cwd] + args)
    return done.stdout.strip() if done is not None and done.returncode == 0 else None


def source_identity(git=git):
    """The runner checkout's commit and tree (hashes only) and whether its tree is dirty: True,
    False, or 'unknown' where git cannot establish it (no git, no repository, no commit or a failed
    status), never False then. The driver's build identity has the same three states."""
    commit, tree = git(['rev-parse', '--verify', 'HEAD']), git(['rev-parse', '--verify', 'HEAD^{tree}'])
    status = git(['status', '--porcelain'])
    return commit, tree, (bool(status) if status is not None and commit is not None else 'unknown')


def metadata(binary, source_commit, source_tree, source_dirty, run=k6.run_quiet, system=None):
    """K6's metadata (no host name, user name, serial or path) and the runner's fixed terms."""
    md = k6.metadata(binary, source_commit, source_tree, run=run, system=system)
    md.update({'runner': os.path.basename(__file__), 'source_dirty': source_dirty,
               'base_machine_budget_bytes': BASE_MACHINE_BUDGET_BYTES,
               'emergency_rss_cap_bytes': EMERGENCY_RSS_CAP_BYTES,
               'footprint_watchdog': 'proc_pid_rusage RUSAGE_INFO_V4' if _proc_pid_rusage() else None,
               'driver_schema': DRIVER_SCHEMA, 'watchdog_poll_s': k6.POLL_S})
    return md


def quiet_host(log, max_wait_s):
    """K6's quiet-host check (no cargo job or sweep, memorystatus_level >= 80, 1-minute load <= 8),
    waiting at most `max_wait_s` (in 30 s steps); a host still busy is recorded, and the run goes ahead."""
    # K6 waits in 30 s steps; round the allowance down so no step can overrun it.
    try:
        return dict(k6.wait_for_quiet_host(log, max_wait_s=(int(max_wait_s) // 30) * 30), quiet=True)
    except RuntimeError as error:
        return {'quiet': False, 'reason': str(error)}


def run_plan(plan, models_dir, out_dir, binary, *, budget_s=WINDOW_BUDGET_S, write_results=False,
             quiet_wait_s=QUIET_WAIT_S, launch=k6.launch, quiet=quiet_host, clock=time.monotonic, log=print,
             md=None, wrapper=None, sampler_factory=FootprintSampler, attributor_factory=Attributor,
             keep_samples=False):
    """The plan's runs in order, one at a time, within the window budget. Returns the records.
    `wrapper` (default: whether /usr/bin/time exists) runs each driver under /usr/bin/time;
    `sampler_factory(budget_bytes)` makes each run's footprint watchdog, and for an attribution run
    `attributor_factory(out_dir, run_id, keep_raw)` its stack snapshots (raw reports kept only with
    `keep_samples`)."""
    check_run_ids(plan)  # before anything is created or written
    start = clock()
    if wrapper is None:
        wrapper = os.path.exists(k6.TIME_WRAPPER)
    os.makedirs(out_dir, exist_ok=True)
    if os.path.exists(os.path.join(out_dir, k6.RECORDS_NAME)):
        raise RuntimeError('records already exist in the output folder; use a new one')
    if md is None:
        md = metadata(binary, *source_identity())
    md = dict(md, budget_s=budget_s, write_results=write_results, quiet_wait_s=quiet_wait_s,
              plan=[r['run_id'] for r in plan])
    with open(os.path.join(out_dir, METADATA_NAME), 'w') as fh:
        json.dump(md, fh, indent=1, sort_keys=True)
        fh.write('\n')
    manifest = read_manifest(models_dir).get('models', {})
    results_dir = os.path.join(out_dir, RESULTS_NAME) if write_results else None
    if results_dir:
        os.makedirs(results_dir, exist_ok=True)
    records = []
    launched_any = False
    for run in plan:
        elapsed = clock() - start
        remaining = budget_s - elapsed
        record = dict(run, schema=RECORD_SCHEMA, window_elapsed_before_s=round(elapsed, 3))
        name = model_name(run['profile'], run['nodes'])
        path = os.path.join(models_dir, name)
        record['model'] = name
        if run['estimate_s'] > remaining:
            record.update(classification='skipped_window_budget',
                          classification_detail='estimate %d s exceeds the remaining %d s' % (run['estimate_s'], remaining))
        elif not os.path.exists(path):
            record.update(classification='model_missing', classification_detail=name)
        else:
            sha = sha256_file(path)
            record.update(model_sha256=sha, model_bytes=os.path.getsize(path))
            listed = manifest.get(name)
            if listed and listed.get('sha256') != sha:
                record.update(classification='model_mismatch', classification_detail='manifest sha256 differs')
            else:
                if listed:
                    record['model_stats'] = listed.get('stats')
                # Before the first launched run the host may settle for up to quiet_wait_s; before each
                # later run its state is checked once and recorded, without waiting.
                wait = quiet_wait_s if not launched_any else 0
                record['host'] = quiet(log, max(0, min(wait, remaining - run['estimate_s'])))
                launched_any = True
                remaining = budget_s - (clock() - start)
                if run['estimate_s'] > remaining or remaining < 1:
                    record.update(classification='skipped_window_budget',
                                  classification_detail='estimate %d s exceeds the %d s remaining after the host '
                                                        'check' % (run['estimate_s'], remaining))
                    record['window_elapsed_after_s'] = round(clock() - start, 3)
                    k6.append_record(out_dir, record)
                    records.append(record)
                    log('%s: skipped_window_budget after the host check' % run['run_id'])
                    continue
                timeout = min(run['timeout_s'], int(remaining))
                record.update(timeout_applied_s=timeout, timeout_capped_by_window=timeout < run['timeout_s'])
                sampler = sampler_factory(run['footprint_budget_bytes'])
                attributor = None
                if run.get('attribute'):
                    attributor = attributor_factory(out_dir, run['run_id'], keep_samples)
                    sampler.attributor = attributor
                launched = launch(driver_argv(binary, path, run, results_dir), rss_cap_bytes=run['rss_cap_bytes'],
                                  timeout_s=timeout, record_dir=out_dir, run_id=run['run_id'], wrapper=wrapper,
                                  k6_protocol=False, sampler=sampler)
                if attributor is not None:
                    attributor.finish()
                    record.update(attribution=attributor.summary(),
                                  timing_use='not a measurement: stack sampling suspends the process at every sample')
                driver = driver_line(out_dir, run['run_id'])
                last_stage, last_memory, stages_finished = last_progress(out_dir, run['run_id'])
                classification, detail = classify(launched, driver, run['rss_cap_bytes'],
                                                  run['footprint_budget_bytes'], last_memory)
                watchdog = launched.get('sampler') or {}
                record['classification_readings'] = readings_record(launched, driver, last_memory)
                record.update(last_stage=last_stage, last_stage_memory=last_memory, stages_finished=stages_finished,
                              footprint_watchdog={k: v for k, v in watchdog.items() if k != 'samples'},
                              peak_footprint_bytes=peak_footprint(driver, launched, last_memory))
                record.update(classification=classification, classification_detail=detail,
                              as_expected=as_expected(run['expect'], classification, detail),
                              launch={k: launched.get(k) for k in (
                                  'classification', 'classification_detail', 'exit_code', 'wall_s', 'rss',
                                  'peak_rss_bytes', 'peak_rss_source', 'killed_by_rss_watchdog', 'killed_by_sampler',
                                  'timed_out',
                                  'kill_after_s', 'watchdog_last_kib', 'watchdog_polls', 'survivors',
                                  'binary_pid_found', 'load_before', 'load_after', 'memorystatus_before',
                                  'memorystatus_after', 'stdout_sha256', 'stderr_tail', 'argv')},
                              driver=driver)
        record['window_elapsed_after_s'] = round(clock() - start, 3)
        k6.append_record(out_dir, record)
        records.append(record)
        wall = (record.get('launch') or {}).get('wall_s')
        log('%s: %s%s (%s; window %.0f s)' % (
            run['run_id'], record['classification'],
            ' [%s]' % record['classification_detail'] if record.get('classification_detail') else '',
            'wall %.1f s' % wall if wall is not None else 'not run', record['window_elapsed_after_s']))
    return records


# --------------------------------------------------------------- packet


def _stage_memory(driver, key):
    peaks = (driver or {}).get('peak_memory') or {}
    values = [m.get(key) for m in peaks.values() if isinstance(m, dict) and isinstance(m.get(key), int)]
    return max(values) if values else None


def run_conditions(r):
    host = r.get('host') or {}
    if 'quiet' not in host:
        state = 'not checked'
    elif host['quiet']:
        state = 'quiet (memorystatus %s, 1-minute load %.2f, waited %s s)' % (
            host.get('memorystatus_level'), host.get('load1_at_start') or 0.0, host.get('waited_s'))
    else:
        state = 'not quiet: %s' % host.get('reason')
    budget = r.get('footprint_budget_bytes')
    return ('%s, %d nodes, %s mode, %s; timeout %s s%s, footprint budget %s, emergency RSS cap %.2f GiB; host %s.' % (
        r['profile'], r['nodes'], r['mode'], r.get('model') or 'no model', r.get('timeout_applied_s'),
        ' (capped by the window)' if r.get('timeout_capped_by_window') else '',
        '%.2f GiB' % (budget / GIB) if budget else 'none', r['rss_cap_bytes'] / GIB, state))


def packet_row(r):
    driver = r.get('driver') or {}
    launch = r.get('launch') or {}
    rss = launch.get('rss') or {}
    ps_kib = rss.get('ps_max_kib')
    return {
        'order': r.get('order'), 'run_id': r['run_id'], 'task': r.get('task'), 'profile': r['profile'],
        'nodes': r['nodes'],
        'mode': r['mode'], 'route': driver.get('route'), 'expect': r.get('expect'),
        'classification': r['classification'], 'classification_detail': r.get('classification_detail'),
        'as_expected': r.get('as_expected'), 'conditions': run_conditions(r),
        'model': r.get('model'), 'model_sha256': r.get('model_sha256'), 'model_bytes': r.get('model_bytes'),
        'counts': {k: driver.get(k) for k in ('nodes', 'members', 'supports', 'cases', 'combinations')},
        'status': driver.get('status'), 'blocking_codes': driver.get('blocking_codes'),
        'wall_s': launch.get('wall_s'), 'stage_seconds': driver.get('seconds'),
        'peak_memory_bytes': {
            'time_max_rss': rss.get('time_max_rss_bytes'),
            'time_peak_footprint': rss.get('time_peak_footprint_bytes'),
            'ps_max_rss': k6.ps_kib_to_bytes(ps_kib) if ps_kib is not None else None,
            'driver_ru_maxrss': _stage_memory(driver, 'ru_maxrss_bytes'),
            'driver_lifetime_max_phys_footprint': driver.get('driver_peak_footprint_bytes'),
            'sampled_phys_footprint': (r.get('footprint_watchdog') or {}).get('peak_phys_footprint_bytes'),
            'sampled_lifetime_max_phys_footprint':
                (r.get('footprint_watchdog') or {}).get('peak_lifetime_max_phys_footprint_bytes'),
            'peak_footprint': r.get('peak_footprint_bytes'),
        },
        'last_stage': r.get('last_stage'), 'last_stage_memory': r.get('last_stage_memory'),
        'attribution': bool(r.get('attribute')), 'timing_use': r.get('timing_use', 'measurement'),
        'snapshots': (r.get('attribution') or {}).get('snapshots'),
        'rows': driver.get('rows'), 'output_bytes': driver.get('output_bytes'),
        'output_sha256': driver.get('output_sha256'), 'row_classes': driver.get('row_classes'),
        'bases': len(driver.get('rows_per_basis') or {}), 'diagnostic_counts': driver.get('diagnostic_counts'),
        'nonlinear_iterations': driver.get('nonlinear_iterations'),
    }


def stale_driver_message(record):
    """The refusal of a record whose driver line is not the current form (DRIVER_SCHEMA), or None."""
    driver = record.get('driver')
    if driver is None or (isinstance(driver, dict) and driver.get('schema') == DRIVER_SCHEMA):
        return None
    schema = driver.get('schema') if isinstance(driver, dict) else None
    return ('record %s: its driver line has schema %s, not %s; an earlier driver\'s record is not repackaged'
            % (record.get('run_id'), schema, DRIVER_SCHEMA))


def packet(out_dir):
    """The packet of OUT's records. A record whose driver line is not the current form is refused
    (ValueError), never repackaged under the current conditions."""
    records = [r for r in k6.read_records(out_dir) if r.get('schema') == RECORD_SCHEMA]
    for r in records:
        message = stale_driver_message(r)
        if message:
            raise ValueError(message)
    md_path = os.path.join(out_dir, METADATA_NAME)
    md = None
    if os.path.exists(md_path):
        with open(md_path) as fh:
            md = json.load(fh)
    runs = [packet_row(r) for r in records if 'launch' in r]
    not_run = [{k: r.get(k) for k in ('order', 'run_id', 'classification', 'classification_detail')}
               for r in records if 'launch' not in r]
    counts = {}
    for r in records:
        counts[r['classification']] = counts.get(r['classification'], 0) + 1
    driver_limits = next((r['driver']['limits'] for r in records if (r.get('driver') or {}).get('limits')), None)
    return {
        'schema': PACKET_SCHEMA, 'observation_only': True,
        'no_thresholds': 'no time or memory bound is asserted anywhere; criterion 9 provisional bounds are not evaluated',
        'conditions': list(CONDITIONS)
        + ['Window budget %s s; %d run(s) not run (skipped, missing or mismatched model).'
           % ((md or {}).get('budget_s'), len(not_run))],
        'metadata': md, 'classification_counts': counts, 'runs': runs, 'not_run': not_run,
        'window_elapsed_s': max((r.get('window_elapsed_after_s') or 0 for r in records), default=0),
        'driver_limits': driver_limits,
    }


def write_packet(out_dir):
    result = packet(out_dir)
    with open(os.path.join(out_dir, PACKET_NAME), 'w') as fh:
        json.dump(result, fh, indent=1, sort_keys=True)
        fh.write('\n')
    return result


# --------------------------------------------------------------- read-back
#
# `--read-back BUNDLE --revision SHA`: the direct checks of a baseline bundle's records (README, "The
# headless baseline", step 9, items 2 to 6), after the shell's integrity and membership checks. It
# reads only, and prints each difference; it does not show that a record is true.

READ_BACK_FILES = ('out/metadata.json', 'out/' + k6.RECORDS_NAME, 'out/packet.json', 'plan.json',
                   'models/manifest.json', 'build-facts.txt', 'host-before.txt', 'host-processes.log',
                   'baseline-set.json', 'contract.md')
READ_BACK_SHARED_HOST_FILES = ('exclusive.log',)
SOLVE_STAGES = ('read', 'parse', 'request', 'entry', 'serialize', 'release_envelope', 'parse_back')
NOT_RUN_KEYS = ('order', 'run_id', 'classification', 'classification_detail')


def _get(record, path):
    """The value at a dotted path of a record, or None."""
    value = record
    for key in path.split('.'):
        value = value.get(key) if isinstance(value, dict) else None
    return value


def _stage_max(record, key):
    peaks = _get(record, 'driver.peak_memory') or {}
    values = [m.get(key) for m in peaks.values() if isinstance(m, dict) and _is_int(m.get(key))]
    return max(values) if values else None


# THE PACKET ROW MAPPING, the one statement of it (README, "The headless baseline", step 9 refers
# here). A launched run's row in `packet.json` `runs` (the self-supervising driver's `packet_row`,
# supervise.rs) carries every field below, each the value the aggregate record gives by the stated
# source. A never-launched run's row in `not_run` carries exactly NOT_RUN_KEYS, each the record's own.
# (packet field, its source in the aggregate record, the value)
PACKET_RUN_FIELDS = (
    ('order', 'order', lambda r: r.get('order')),
    ('run_id', 'run_id', lambda r: r.get('run_id')),
    ('task', 'task', lambda r: r.get('task')),
    ('profile', 'profile', lambda r: r.get('profile')),
    ('nodes', 'nodes', lambda r: r.get('nodes')),
    ('mode', 'mode', lambda r: r.get('mode')),
    ('route', 'driver.route', lambda r: _get(r, 'driver.route')),
    ('expect', 'expect', lambda r: r.get('expect')),
    ('classification', 'classification', lambda r: r.get('classification')),
    ('classification_detail', 'classification_detail', lambda r: r.get('classification_detail')),
    ('as_expected', 'as_expected', lambda r: r.get('as_expected')),
    ('conditions', 'run_conditions(record): profile, nodes, mode, model, timeout_applied_s, '
                   'timeout_capped_by_window, footprint_budget_bytes, rss_cap_bytes, host', lambda r: run_conditions(r)),
    ('model', 'model', lambda r: r.get('model')),
    ('model_sha256', 'model_sha256', lambda r: r.get('model_sha256')),
    ('model_bytes', 'model_bytes', lambda r: r.get('model_bytes')),
    ('counts.nodes', 'driver.nodes', lambda r: _get(r, 'driver.nodes')),
    ('counts.members', 'driver.members', lambda r: _get(r, 'driver.members')),
    ('counts.supports', 'driver.supports', lambda r: _get(r, 'driver.supports')),
    ('counts.cases', 'driver.cases', lambda r: _get(r, 'driver.cases')),
    ('counts.combinations', 'driver.combinations', lambda r: _get(r, 'driver.combinations')),
    ('status', 'driver.status', lambda r: _get(r, 'driver.status')),
    ('blocking_codes', 'driver.blocking_codes', lambda r: _get(r, 'driver.blocking_codes')),
    ('wall_s', 'launch.wall_s', lambda r: _get(r, 'launch.wall_s')),
    ('stage_seconds', 'driver.seconds', lambda r: _get(r, 'driver.seconds')),
    ('peak_memory_bytes.time_max_rss', 'launch.rss.time_max_rss_bytes',
     lambda r: _get(r, 'launch.rss.time_max_rss_bytes')),
    ('peak_memory_bytes.time_peak_footprint', 'launch.rss.time_peak_footprint_bytes',
     lambda r: _get(r, 'launch.rss.time_peak_footprint_bytes')),
    ('peak_memory_bytes.ps_max_rss', 'launch.rss.ps_max_kib x 1024',
     lambda r: k6.ps_kib_to_bytes(_get(r, 'launch.rss.ps_max_kib')) if _is_int(_get(r, 'launch.rss.ps_max_kib')) else None),
    ('peak_memory_bytes.driver_ru_maxrss', 'the largest driver.peak_memory.<stage>.ru_maxrss_bytes',
     lambda r: _stage_max(r, 'ru_maxrss_bytes')),
    ('peak_memory_bytes.driver_lifetime_max_phys_footprint', 'driver.driver_peak_footprint_bytes',
     lambda r: _get(r, 'driver.driver_peak_footprint_bytes')),
    ('peak_memory_bytes.sampled_phys_footprint', 'footprint_watchdog.peak_phys_footprint_bytes',
     lambda r: _get(r, 'footprint_watchdog.peak_phys_footprint_bytes')),
    ('peak_memory_bytes.sampled_lifetime_max_phys_footprint', 'footprint_watchdog.peak_lifetime_max_phys_footprint_bytes',
     lambda r: _get(r, 'footprint_watchdog.peak_lifetime_max_phys_footprint_bytes')),
    ('peak_memory_bytes.peak_footprint', 'peak_footprint_bytes', lambda r: r.get('peak_footprint_bytes')),
    ('peak_memory_bytes.wait4_maxrss', 'launch.rss.wait4_maxrss_bytes', lambda r: _get(r, 'launch.rss.wait4_maxrss_bytes')),
    ('peak_memory_bytes.sampled_resident_size', 'launch.rss.resident_max_bytes',
     lambda r: _get(r, 'launch.rss.resident_max_bytes')),
    ('peak_memory_bytes.exit_lifetime_max_phys_footprint', 'launch.rss.exit_lifetime_max_phys_footprint_bytes',
     lambda r: _get(r, 'launch.rss.exit_lifetime_max_phys_footprint_bytes')),
    ('last_stage', 'last_stage', lambda r: r.get('last_stage')),
    ('last_stage_memory', 'last_stage_memory', lambda r: r.get('last_stage_memory')),
    ('attribution', 'attribute (true or false)', lambda r: bool(r.get('attribute'))),
    ('timing_use', 'timing_use, else "measurement"', lambda r: r.get('timing_use', 'measurement')),
    ('snapshots', 'attribution.snapshots', lambda r: _get(r, 'attribution.snapshots')),
    ('rows', 'driver.rows', lambda r: _get(r, 'driver.rows')),
    ('output_bytes', 'driver.output_bytes', lambda r: _get(r, 'driver.output_bytes')),
    ('output_sha256', 'driver.output_sha256', lambda r: _get(r, 'driver.output_sha256')),
    ('row_classes', 'driver.row_classes', lambda r: _get(r, 'driver.row_classes')),
    ('bases', 'the number of driver.rows_per_basis', lambda r: len(_get(r, 'driver.rows_per_basis') or {})),
    ('diagnostic_counts', 'driver.diagnostic_counts', lambda r: _get(r, 'driver.diagnostic_counts')),
    ('nonlinear_iterations', 'driver.nonlinear_iterations', lambda r: _get(r, 'driver.nonlinear_iterations')),
)
# The fields whose VALUE each row form requires: a null there, in the row or in the aggregate's own
# value, is a difference. Every other field may be null where the aggregate's value is null (a
# stopped run has no driver line; this supervisor has no /usr/bin/time or ps reading).
#   completed: launched, classification ok;
#   stopped: launched, any other classification (killed, timed out, refused, an entry error);
#   not_run: never launched (skipped for the window, model_missing, model_mismatch).
PACKET_REQUIRED_VALUES = {
    'completed': ('order', 'run_id', 'task', 'profile', 'nodes', 'mode', 'route', 'expect', 'classification',
                  'as_expected', 'conditions', 'model', 'model_sha256', 'model_bytes', 'counts.nodes', 'counts.members',
                  'counts.supports', 'counts.cases', 'counts.combinations', 'status', 'blocking_codes', 'wall_s',
                  'stage_seconds', 'peak_memory_bytes.driver_ru_maxrss',
                  'peak_memory_bytes.driver_lifetime_max_phys_footprint', 'peak_memory_bytes.peak_footprint',
                  'peak_memory_bytes.exit_lifetime_max_phys_footprint', 'last_stage', 'last_stage_memory',
                  'attribution', 'timing_use', 'rows', 'output_bytes', 'output_sha256', 'row_classes', 'bases',
                  'diagnostic_counts'),
    'stopped': ('order', 'run_id', 'task', 'profile', 'nodes', 'mode', 'expect', 'classification',
                'classification_detail', 'as_expected', 'conditions', 'model', 'model_sha256', 'model_bytes', 'wall_s',
                'attribution', 'timing_use', 'bases'),
    'not_run': ('order', 'run_id', 'classification', 'classification_detail'),
}


def row_form(record):
    """`completed`, `stopped` or `not_run`: which packet row form a record has."""
    if 'launch' not in record:
        return 'not_run'
    return 'completed' if record.get('classification') == 'ok' else 'stopped'


def expected_packet_row(record):
    """The packet row PACKET_RUN_FIELDS (a launched record) or NOT_RUN_KEYS (a never-launched one)
    gives for `record`, as nested dicts."""
    if row_form(record) == 'not_run':
        return {k: record.get(k) for k in NOT_RUN_KEYS}
    row = {}
    for field, _source, value in PACKET_RUN_FIELDS:
        *parents, last = field.split('.')
        target = row
        for key in parents:
            target = target.setdefault(key, {})
        target[last] = value(record)
    return row


def _fields_differ(prefix, found, expected):
    """Each field of `expected` that `found` lacks or holds otherwise, and each field `found` holds that
    `expected` does not, by dotted name."""
    out = []
    for key in sorted(set(found) | set(expected)):
        name = '%s.%s' % (prefix, key)
        if key not in expected:
            out.append('%s is a field this check does not know' % name)
        elif key not in found:
            out.append('%s is missing' % name)
        elif isinstance(expected[key], dict) and isinstance(found[key], dict):
            out += _fields_differ(name, found[key], expected[key])
        elif found[key] != expected[key]:
            out.append('%s is %r, the aggregate gives %r' % (name, found[key], expected[key]))
    return out


INVENTORY_NAME = 'SHA256SUMS'
# The aggregate record's `launch` member: the supervisor copies exactly these keys from the launch record,
# a null where it has none (supervise.rs LAUNCH_KEYS), and its launch route writes each into the launch
# record (`launch`, supervise.rs 987-1018), null included.
LAUNCH_KEYS = ('classification', 'classification_detail', 'exit_code', 'wall_s', 'rss', 'peak_rss_bytes',
               'peak_rss_source', 'killed_by_rss_watchdog', 'killed_by_sampler', 'timed_out', 'kill_after_s',
               'watchdog_last_kib', 'watchdog_polls', 'survivors', 'binary_pid_found', 'load_before', 'load_after',
               'memorystatus_before', 'memorystatus_after', 'stdout_sha256', 'stderr_tail', 'argv',
               'watchdog_last_resident_bytes', 'schema', 'stdout_record_error')
# REQUIRED VALUES BY DECISION BRANCH, mirroring the decision-input table of the measurement contract: for
# each record form, the aggregate paths a decision rule reads (S3, S4: sections 7, 8, 11; S5: section 14).
# A null or a missing key there is a difference. Every other path may hold the null the producer leaves.
_EVERY_RECORD = ('classification',)
_LAUNCHED = _EVERY_RECORD + ('as_expected', 'model_sha256', 'timeout_applied_s', 'timeout_capped_by_window',
                             'host.quiet', 'launch.killed_by_sampler', 'launch.killed_by_rss_watchdog')
REQUIRED_BY_BRANCH = {
    'completed': _LAUNCHED + (
        'driver.route', 'driver.document.sha256', 'driver.output_sha256', 'driver.rows', 'driver.output_bytes',
        'driver.seconds.read', 'driver.seconds.parse', 'driver.seconds.request', 'driver.seconds.entry',
        'driver.seconds.serialize', 'driver.seconds.release_envelope', 'driver.seconds.parse_back',
        'driver.seconds.total', 'launch.rss.exit_lifetime_max_phys_footprint_bytes', 'last_stage'),
    'refused': _LAUNCHED + ('classification_detail', 'last_stage'),
    'entry_error': _LAUNCHED + ('classification_detail', 'last_stage'),   # last_stage: not after a read error
    'killed_by_sampler': _LAUNCHED + ('classification_detail',),
    'over_budget_at_result': _LAUNCHED + ('classification_detail', 'last_stage'),
    'killed_by_rss_cap': _LAUNCHED + ('classification_detail',),
    'timed_out': _LAUNCHED + ('classification_detail',),
    'error': _LAUNCHED + ('classification_detail',),
    'never_launched': _EVERY_RECORD + ('classification_detail',),
}


# Values only one plan's rules read, by named plan and branch, added to REQUIRED_BY_BRANCH for that plan.
# The baseline plans (`baseline`, `baseline-with-rack`) add none: no rule of S3 or S4 reads E or S.
# The S5 plan, when it exists, adds S5_COMPLETED_MEMORY to its completed branch: E and S of the measurement
# contract's section 14.5, read by section 14.6's K2.
S5_COMPLETED_MEMORY = ('driver.peak_memory.entry.lifetime_max_phys_footprint_bytes',
                       'driver.peak_memory.serialize.lifetime_max_phys_footprint_bytes')
PLAN_REQUIRED_EXTRA = {}


def named_plan_of(plan_runs):
    """The named plan whose run ids, in order, are `plan_runs`', or None."""
    ids = [r.get('run_id') for r in plan_runs]
    return next((name for name in NAMED_PLANS if [r['run_id'] for r in named_plan(name)[0]] == ids), None)


def record_branch(record):
    """The decision branch a record's form takes (REQUIRED_BY_BRANCH)."""
    if 'launch' not in record:
        return 'never_launched'
    classification, detail = record.get('classification'), str(record.get('classification_detail') or '')
    if classification == 'ok':
        return 'completed'
    if classification in ('refused', 'entry_error', 'timed_out', 'killed_by_rss_cap'):
        return classification
    if classification == 'exceeds_base_machine_budget':
        if _get(record, 'launch.killed_by_sampler') is True:
            return 'killed_by_sampler'
        if detail.endswith('_above_budget_at_result'):
            return 'over_budget_at_result'
        return 'killed_by_rss_cap'
    return 'error'


def required_value_problems(record, plan_name=None, extra=None):
    """Each path a decision reads for this record's branch that holds no value: REQUIRED_BY_BRANCH, and what
    the plan's own rules add (PLAN_REQUIRED_EXTRA[plan_name], or `extra`)."""
    branch = record_branch(record)
    extra = PLAN_REQUIRED_EXTRA.get(plan_name, {}) if extra is None else extra
    problems = []
    for path in REQUIRED_BY_BRANCH[branch] + tuple(extra.get(branch, ())):
        if path == 'last_stage' and branch == 'entry_error' and str(record.get('classification_detail') or '').startswith('read:'):
            continue   # an error in the read stage ends before any stage line
        *parents, last = path.split('.')
        holder = _get(record, '.'.join(parents)) if parents else record
        if not isinstance(holder, dict) or holder.get(last) is None:
            problems.append('%s: a %s record has no value at %s, which a decision rule reads' % (
                record.get('run_id'), branch, path))
    return problems
_INVENTORY_LINE = re.compile(r'([0-9a-f]{64})  \./(.+)')
UNRELEASED_BUILD_ALLOWED = ('The build identity was NOT required: --allow-unreleased-build was given, so source_dirty '
                            'and build_profile were not checked.')
CLEAN_READ_BACK_LIMIT = ('A clean read-back does not by itself make a measurement usable: the worker\'s checks of the '
                         'build (contract section 4) and of the host and window (section 8) do that.')
MEMORY_KEYS = ('ru_maxrss_bytes', 'phys_footprint_bytes', 'lifetime_max_phys_footprint_bytes')


class BoundBundle:
    """A bundle's files, read only as its inventory binds them. Every file consumed must be a regular file
    reached without a symbolic link, named in SHA256SUMS (the inventory of README step 8), and have the
    SHA-256 SHA256SUMS states; its bytes are read once, hashed, and only those bytes are used."""

    def __init__(self, bundle, problems, root_digest):
        self.bundle, self.problems, self.sums = bundle, problems, None
        raw = self._regular(INVENTORY_NAME)
        if raw is None:
            return
        if hashlib.sha256(raw).hexdigest() != root_digest:
            problems.append('%s does not have the root digest %s; nothing in the bundle is bound' % (INVENTORY_NAME, root_digest))
            return
        self.sums = {}
        for number, line in enumerate(raw.decode('utf-8', 'replace').splitlines(), 1):
            match = _INVENTORY_LINE.fullmatch(line)
            if match is None:
                problems.append('%s line %d is not "<sha256>  ./<path>"' % (INVENTORY_NAME, number))
            else:
                self.sums[match.group(2)] = match.group(1)

    def _regular(self, name):
        """The bytes of `name` if it and every folder above it within the bundle are not links and it is
        a regular file; else None, with the difference recorded."""
        path = self.bundle
        parts = name.split('/')
        for index, part in enumerate(parts):
            path = os.path.join(path, part)
            try:
                mode = os.lstat(path).st_mode
            except OSError:
                self.problems.append('%s is missing' % name)
                return None
            if stat.S_ISLNK(mode):
                self.problems.append('%s is reached through a symbolic link (%s)' % (name, '/'.join(parts[:index + 1])))
                return None
        if not stat.S_ISREG(mode):
            self.problems.append('%s is not a regular file' % name)
            return None
        with open(path, 'rb') as fh:
            return fh.read()

    def read(self, name):
        """The bytes of a consumed file, or None with the difference recorded."""
        if self.sums is None:
            return None
        if name not in self.sums:
            if os.path.lexists(os.path.join(self.bundle, name)):
                self.problems.append('%s is not in %s, so its bytes are not bound' % (name, INVENTORY_NAME))
            else:
                self.problems.append('%s is missing' % name)
            return None
        data = self._regular(name)
        if data is None:
            return None
        if hashlib.sha256(data).hexdigest() != self.sums[name]:
            self.problems.append('%s does not have the SHA-256 %s states for it' % (name, INVENTORY_NAME))
            return None
        return data

    def json(self, name):
        data = self.read(name)
        if data is None:
            return None
        try:
            return json.loads(data)
        except ValueError as error:
            self.problems.append('%s does not parse: %s' % (name, error))
            return None

    def exists(self, name):
        return os.path.lexists(os.path.join(self.bundle, name))


def _stage_lines(data, run_id, problems):
    lines = []
    for line in data.decode('utf-8', 'replace').splitlines():
        if line.startswith(PROGRESS_PREFIX):
            try:
                lines.append(json.loads(line[len(PROGRESS_PREFIX):]))
            except ValueError:
                problems.append('%s: a stage line of its .stderr.txt does not parse' % run_id)
    return lines


def read_back(bundle, revision, root_digest, shared_host=False, allow_unreleased_build=False):
    """The differences between a bundle's records (items 2 to 6 of the read-back), as text. Unless
    `allow_unreleased_build`, a build identity that is not clean and release is a difference (contract
    section 4 item 4, section 11 item 2, section 14.6 step 1)."""
    problems = []
    files = BoundBundle(bundle, problems, root_digest)
    if files.sums is None:
        return problems
    # 2. What is consumed is present, bound by the inventory and not a link. The files the worker
    # examines by hand (build facts, host files, the exclusive log, the contract) are inside the same
    # boundary: their bytes are verified here, their content is not read.
    for name in READ_BACK_FILES + (READ_BACK_SHARED_HOST_FILES if shared_host else ()):
        if name not in ('plan.json', 'models/manifest.json', 'out/packet.json', 'out/metadata.json',
                        'out/' + k6.RECORDS_NAME, 'baseline-set.json'):
            files.read(name)
    if not shared_host and 'exclusive.log' in files.sums:
        files.read('exclusive.log')
    plan, manifest, packet_doc, metadata = (files.json(n) for n in (
        'plan.json', 'models/manifest.json', 'out/packet.json', 'out/metadata.json'))
    aggregate_bytes = files.read('out/' + k6.RECORDS_NAME)
    if None in (plan, manifest, packet_doc, metadata, aggregate_bytes):
        return problems
    records, unread = records_of(aggregate_bytes.decode('utf-8', 'replace'))
    problems += unread
    expected_names = set(READ_BACK_FILES) | set(READ_BACK_SHARED_HOST_FILES) | {
        'models/%s' % r.get('model') for r in plan.get('runs') or []} | {
        'out/%s%s' % (r.get('run_id'), suffix) for r in records if 'launch' in r
        for suffix in (k6.STDOUT_SUFFIX, k6.STDERR_SUFFIX, '.record.json')}
    for name in sorted(set(files.sums) - expected_names):
        problems.append('%s is in %s but is no file of the bundle (README step 8) or of a launched run' % (
            name, INVENTORY_NAME))
    for name in sorted(n for n in files.sums if n.startswith('models/') and n != 'models/manifest.json'
                       and n in expected_names):
        files.read(name)   # a model kept in the bundle is bound too
    manifest = manifest.get('models') or {}
    aggregate = {r.get('run_id'): r for r in records}
    plan_runs = plan.get('runs') or []
    plan_ids = [r.get('run_id') for r in plan_runs]
    # The plan of this run: the one the supervisor read (its schema), whose window and quiet wait it
    # wrote into the metadata with the run ids in order, and whose entries it copied into each record.
    if plan.get('schema') != PLAN_SCHEMA:
        problems.append('plan.json schema is %r, not %s, the only plan the supervisor reads' % (plan.get('schema'), PLAN_SCHEMA))
    if metadata.get('budget_s') != plan.get('window_budget_s'):
        problems.append('metadata.json budget_s %r is not plan.json window_budget_s %r' % (
            metadata.get('budget_s'), plan.get('window_budget_s')))
    if metadata.get('quiet_wait_s') != plan.get('quiet_wait_s'):
        problems.append('metadata.json quiet_wait_s %r is not plan.json quiet_wait_s %r' % (
            metadata.get('quiet_wait_s'), plan.get('quiet_wait_s')))
    if metadata.get('plan') != plan_ids:
        problems.append('metadata.json plan is not the run ids of plan.json in their order')
    if [r.get('run_id') for r in records] != plan_ids:
        problems.append('the aggregate does not hold one record for each run of plan.json, in its order')
    plan_name = named_plan_of(plan_runs)
    for p in plan_runs:
        run_id = p.get('run_id')
        r = aggregate.get(run_id)
        if r is None:
            continue
        problems += required_value_problems(r, plan_name)
        for key in PLAN_RUN_KEYS + ('model',):
            if key not in p or key not in r or p[key] != r[key]:
                problems.append('%s: %s in the aggregate is not plan.json\'s' % (run_id, key))
        names = ['out/%s%s' % (run_id, suffix) for suffix in (k6.STDOUT_SUFFIX, k6.STDERR_SUFFIX, '.record.json')]
        launched = 'launch' in r
        if not launched:
            for name in names:
                if files.exists(name):
                    problems.append('%s: %s exists for a run that was never launched' % (run_id, os.path.basename(name)))
            if r.get('last_stage') is not None or r.get('last_stage_memory') is not None:
                problems.append('%s: a run never launched carries a last stage' % run_id)
        stdout_bytes, stderr_bytes, record_bytes = (files.read(n) for n in names) if launched else (None, None, None)
        stages = _stage_lines(stderr_bytes, run_id, problems) if stderr_bytes is not None else []
        # 3. Every launched run.
        if launched:
            if record_bytes is not None:
                try:
                    launch_record = json.loads(record_bytes)
                except ValueError:
                    launch_record = None
                    problems.append('%s: its .record.json does not parse' % run_id)
                if sorted(r['launch']) != sorted(LAUNCH_KEYS):
                    problems.append('%s: the aggregate\'s launch does not carry exactly the supervisor\'s launch keys' % run_id)
                for key in LAUNCH_KEYS if isinstance(launch_record, dict) else ():
                    if key not in launch_record or key not in r['launch']:
                        problems.append('%s: launch.%s is missing from %s (the supervisor writes it, null included)' % (
                            run_id, key, 'its .record.json' if key not in launch_record else 'the aggregate'))
                    elif launch_record[key] != r['launch'][key]:
                        problems.append('%s: launch.%s differs from its .record.json' % (run_id, key))
            stdout_sha = r['launch'].get('stdout_sha256')
            if stdout_sha is not None and (stdout_bytes is None or hashlib.sha256(stdout_bytes).hexdigest() != stdout_sha):
                problems.append('%s: its .jsonl is not launch.stdout_sha256' % run_id)
            last = stages[-1] if stages else None
            expected_stage = {k: last.get(k) for k in ('repeat', 'stage', 'seconds')} if last else None
            expected_memory = {k: last.get(k) for k in MEMORY_KEYS} if last else None
            if r.get('last_stage') != expected_stage or r.get('last_stage_memory') != expected_memory:
                problems.append('%s: last_stage or last_stage_memory differs from the last stage line of its '
                                '.stderr.txt' % run_id)
            listed = (manifest.get(p.get('model')) or {}).get('sha256')
            if r.get('model_sha256') is None or not (r.get('model_sha256') == p.get('expected_model_sha256') == listed):
                problems.append('%s: model_sha256 %s is not the plan\'s %s and the manifest\'s %s' % (
                    run_id, r.get('model_sha256'), p.get('expected_model_sha256'), listed))
        # 4. A run with a driver line; 5. a run without one.
        stdout = stdout_bytes.decode('utf-8', 'replace') if stdout_bytes is not None else ''
        lines = [line for line in stdout.splitlines() if line.strip()]
        if lines:
            try:
                parsed = json.loads(lines[0]) if len(lines) == 1 else None
            except ValueError:
                parsed = None
            if parsed is None or parsed != r.get('driver'):
                problems.append('%s: its .jsonl is not one line equal to the aggregate\'s driver' % run_id)
            else:
                # Each stage line's memory readings are the driver's own for that stage.
                peaks = r['driver'].get('peak_memory') or {}
                for line in stages:
                    held = peaks.get(line.get('stage'))
                    for key in MEMORY_KEYS if isinstance(held, dict) else ():
                        if line.get(key) is not None and held.get(key) is not None and line[key] != held[key]:
                            problems.append('%s: stage %s %s differs from driver.peak_memory' % (
                                run_id, line.get('stage'), key))
                if r.get('classification') == 'ok':
                    seconds = r['driver'].get('seconds') or {}
                    for line in stages:
                        if seconds.get(line.get('stage')) != line.get('seconds'):
                            problems.append('%s: stage %s seconds differ from driver.seconds' % (run_id, line.get('stage')))
                    if not all(_number(seconds.get(s)) for s in SOLVE_STAGES + ('total',)):
                        problems.append('%s: driver.seconds lacks a number for a stage or the total' % run_id)
                    elif abs(seconds['total'] - sum(seconds[s] for s in SOLVE_STAGES)) > 1e-6:
                        problems.append('%s: driver.seconds.total is not the sum of the seven stages to 1e-6 s' % run_id)
        elif r.get('driver') is not None or r.get('classification') == 'ok':
            problems.append('%s: no driver line, but the aggregate has a driver or classification ok' % run_id)
    # 6. The packet and the metadata.
    rows = {}
    for kind, entries in (('runs', packet_doc.get('runs') or []), ('not_run', packet_doc.get('not_run') or [])):
        for row in entries:
            if row.get('run_id') in rows:
                problems.append('packet: %s is listed more than once' % row.get('run_id'))
            rows[row.get('run_id')] = (kind, row)
    for run_id, r in aggregate.items():
        kind, row = rows.get(run_id, (None, None))
        want = 'runs' if 'launch' in r else 'not_run'
        if row is None:
            problems.append('packet: %s is not listed' % run_id)
            continue
        if kind != want:
            problems.append('packet: %s is listed under %s, not %s' % (run_id, kind, want))
            continue
        expected = expected_packet_row(r)
        problems += _fields_differ('packet[%s]' % run_id, row, expected)
        for field in PACKET_REQUIRED_VALUES[row_form(r)]:
            if _get(row, field) is None or _get(expected, field) is None:
                problems.append('packet[%s].%s: a %s row requires a value, and the %s has none' % (
                    run_id, field, row_form(r), 'row' if _get(row, field) is None else 'aggregate'))
    for run_id in rows:
        if run_id not in aggregate:
            problems.append('packet: %s has no aggregate record' % run_id)
    counts = {}
    for r in records:
        counts[str(r.get('classification'))] = counts.get(str(r.get('classification')), 0) + 1
    if packet_doc.get('classification_counts') != counts:
        problems.append('packet.classification_counts is not the aggregate\'s')
    if packet_doc.get('metadata') != metadata:
        problems.append('packet.metadata is not metadata.json')
    if metadata.get('source_commit') != revision:
        problems.append('metadata.json names %s, not the revision %s' % (metadata.get('source_commit'), revision))
    if not allow_unreleased_build:
        if metadata.get('source_dirty') is not False:
            problems.append('metadata.json source_dirty is %r, not false: the build is not of a clean checkout'
                            % metadata.get('source_dirty'))
        if metadata.get('build_profile') != 'release':
            problems.append('metadata.json build_profile is %r, not "release"' % metadata.get('build_profile'))
    # baseline-set.json: the two results recomputed from the records just checked, then compared.
    stored = files.json('baseline-set.json')
    if stored is not None:
        name = stored.get('subset') if isinstance(stored, dict) else None
        if name not in NAMED_PLANS:
            problems.append('baseline-set.json names no named plan (subset %r), so its results cannot be recomputed' % name)
        elif baseline_set_of(records, plan, name, unread) != stored:
            problems.append('baseline-set.json is not the two results --baseline-set gives for these records and plan')
    return problems


# --------------------------------------------------------------- main


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('--plan', action='store_true', help='print the schedule and its estimated total')
    parser.add_argument('--generate', action='store_true', help="generate the plan's models and the manifest")
    parser.add_argument('--run', action='store_true', help="run the plan (one process at a time)")
    parser.add_argument('--packet', action='store_true', help='write OUT/packet.json from the records')
    parser.add_argument('--baseline-set', metavar='OUT',
                        help='two results on ONE supervised folder of a named plan (with --plan-file and --subset), '
                             'the ten-run timing set and the rack observation; writes nothing; prints the JSON '
                             'report on stdout and the two results on stderr; ' + EXIT_CONTRACT.replace('%', '%%'))
    parser.add_argument('--plan-file', help='the plan file the supervised folder was run from (--baseline-set)')
    parser.add_argument('--read-back', metavar='BUNDLE',
                        help='the direct checks of a baseline bundle (with --revision): each difference is printed; '
                             'exit 0 when there is none, 1 otherwise; reads only')
    parser.add_argument('--revision', help='the full commit the bundle measured (--read-back)')
    parser.add_argument('--root-digest', help='--read-back: the SHA-256 of the bundle\'s SHA256SUMS, stated outside it')
    parser.add_argument('--allow-unreleased-build', action='store_true',
                        help='--read-back: do not require metadata.json source_dirty false and build_profile release '
                             '(for test fixtures made by a debug build); the output says so')
    parser.add_argument('--shared-host', action='store_true',
                        help='--read-back: the bundle must also hold exclusive.log, the exclusive wrapper\'s lines')
    parser.add_argument('--write-plan', metavar='FILE',
                        help="write the selected runs as the plan file of the driver's supervise mode (with --models, "
                             "each model's sha256 from the manifest)")
    parser.add_argument('--subset', default='all', help='%s, or comma-separated run ids; or a named plan: %s' % (
        ', '.join(SUBSETS), ', '.join(NAMED_PLANS)))
    parser.add_argument('--mode', choices=MODES, help='default sparse')
    parser.add_argument('--footprint-budget-bytes', type=int,
                        help='the phys_footprint kill (default: the 12 GiB base-machine budget)')
    parser.add_argument('--rss-cap-bytes', type=int,
                        help='the emergency ps RSS kill (default: 16 GiB, the base machine memory)')
    parser.add_argument('--budget-s', type=int, help='default %d; a named plan carries its own' % WINDOW_BUDGET_S)
    parser.add_argument('--quiet-wait-s', type=int, help='default %d' % QUIET_WAIT_S)
    parser.add_argument('--models')
    parser.add_argument('--out')
    parser.add_argument('--binary')
    parser.add_argument('--generator', help='the rack generator (default: rack/rack_model.py); refused for a named plan')
    parser.add_argument('--attribute', action='store_true',
                        help='run every selected run as an attribution run (stack snapshots; timings not measurements)')
    parser.add_argument('--keep-samples', action='store_true', help="keep attribution runs' raw sample reports")
    parser.add_argument('--write-results', action='store_true', help='keep each result (large: about 1.5 GB at 2,000 nodes)')
    args = parser.parse_args(argv)
    named = args.subset in NAMED_PLANS
    if named:
        plan, budget_s, quiet_wait_s = named_plan(args.subset)
        fixed = {'--mode': (args.mode, NAMED_PLAN_MODE),
                 '--footprint-budget-bytes': (args.footprint_budget_bytes, BASE_MACHINE_BUDGET_BYTES),
                 '--rss-cap-bytes': (args.rss_cap_bytes, EMERGENCY_RSS_CAP_BYTES),
                 '--budget-s': (args.budget_s, budget_s), '--quiet-wait-s': (args.quiet_wait_s, quiet_wait_s)}
        differing = ['%s %s (the plan carries %s)' % (flag, given, own) for flag, (given, own) in fixed.items()
                     if given is not None and given != own]
        if args.attribute:
            differing.append('--attribute')
        if args.generator is not None and os.path.realpath(args.generator) != os.path.realpath(GENERATOR):
            differing.append('--generator %s (its rack models are the repository generator\'s bytes, held to '
                             'observations/rack/models_sha256.txt)' % os.path.basename(args.generator))
        if differing:
            parser.error('the named plan %s fixes its runs, mode and limits; leave out %s'
                         % (args.subset, ', '.join(differing)))
        args.budget_s, args.quiet_wait_s = budget_s, quiet_wait_s
        args.generator = GENERATOR
        # Whatever reads a models folder for a named plan first holds it to the repository's bytes.
        if args.models and (args.plan or args.write_plan or args.run):
            try:
                check_pinned_models(plan, args.models)
            except ValueError as error:
                parser.error(str(error))
    else:
        args.generator = args.generator or GENERATOR
        args.mode = args.mode or 'sparse'
        args.rss_cap_bytes = EMERGENCY_RSS_CAP_BYTES if args.rss_cap_bytes is None else args.rss_cap_bytes
        args.footprint_budget_bytes = (BASE_MACHINE_BUDGET_BYTES if args.footprint_budget_bytes is None
                                       else args.footprint_budget_bytes)
        args.budget_s = WINDOW_BUDGET_S if args.budget_s is None else args.budget_s
        args.quiet_wait_s = QUIET_WAIT_S if args.quiet_wait_s is None else args.quiet_wait_s
        try:
            plan = select(default_plan(args.mode, args.rss_cap_bytes, args.footprint_budget_bytes), args.subset)
            if args.attribute:
                plan = with_attribution(plan)
        except ValueError as error:
            parser.error(str(error))
    if args.read_back:
        if not (args.revision and args.root_digest):
            parser.error('--read-back needs --revision, the full commit the bundle measured, and --root-digest, '
                         'the SHA-256 of its SHA256SUMS stated outside the bundle')
        problems = read_back(args.read_back, args.revision, args.root_digest, args.shared_host,
                             args.allow_unreleased_build)
        for problem in problems:
            print('DIFFERS: %s' % problem)
        print('read-back: %d difference(s)' % len(problems))
        if args.allow_unreleased_build:
            print(UNRELEASED_BUILD_ALLOWED)
        if not problems:
            print(CLEAN_READ_BACK_LIMIT)
        return 1 if problems else 0
    if args.baseline_set:
        if not named or not args.plan_file:
            parser.error('--baseline-set needs --plan-file and a named --subset (%s)' % ', '.join(NAMED_PLANS))
        report = baseline_set(args.baseline_set, args.plan_file, args.subset)
        sys.stdout.write(json.dumps(report, indent=1, sort_keys=True) + '\n')
        sys.stderr.write(''.join(line + '\n' for line in report['results'])
                         + 'exit %d (%s)\n' % (report['exit_code'], EXIT_CONTRACT))
        return report['exit_code']
    if args.plan:
        sys.stdout.write(format_plan(plan, args.budget_s, args.quiet_wait_s if named else None))
        return 0
    if args.write_plan:
        try:
            write_plan(args.write_plan, plan, args.budget_s, args.quiet_wait_s, args.models, require_sha256=named)
        except ValueError as error:
            parser.error(str(error))
        return 0
    if args.generate:
        if not args.models:
            parser.error('--generate needs --models')
        _, failures = generate(plan, args.models, generator=args.generator, pinned_hashes=RACK_HASHES if named else None)
        return 1 if failures else 0
    if args.run:
        if not (args.models and args.out and args.binary):
            parser.error('--run needs --models, --out and --binary')
        records = run_plan(plan, args.models, args.out, os.path.abspath(args.binary), budget_s=args.budget_s,
                           write_results=args.write_results, quiet_wait_s=args.quiet_wait_s,
                           keep_samples=args.keep_samples)
        bad = [r for r in records if r['classification'] in ('error', 'model_missing', 'model_mismatch')]
        return 1 if bad else 0
    if args.packet:
        if not args.out:
            parser.error('--packet needs --out')
        try:
            result = write_packet(args.out)
        except ValueError as error:
            print('release_runner --packet: %s' % error, file=sys.stderr)
            return 2
        print(json.dumps({'packet': 'packet.json', 'classification_counts': result['classification_counts']}))
        return 0
    parser.print_help()
    return 2


if __name__ == '__main__':
    sys.exit(main())
