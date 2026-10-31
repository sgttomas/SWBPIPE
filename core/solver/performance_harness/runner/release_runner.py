#!/usr/bin/env python3
"""DEL-04-05 release timing runner (RT-U4, release target exit criterion 9): standard library only.

It runs the release timing driver (``core/solver/release_timing``,
binary ``release_timing``, schema ``release-timing-v1``) on invented rack
models, one process at a time, each under K6's launch machinery
(``k6_runner.launch``: ``/usr/bin/time -l``, the 100 ms ``ps`` RSS watchdog, the
timeout and the process-group kill, and its record files), and gathers a
compact packet. A footprint watchdog emulates the product's memory budget: every
poll it reads the driver's phys_footprint and lifetime-maximum phys_footprint
(macOS ``proc_pid_rusage``, ``RUSAGE_INFO_V4``) and kills the run when either
exceeds the budget, as the desktop kills a worker (``worker_footprint_exceeded``).

Modes (each starts nothing else):

- ``--plan``: the schedule with its estimates and their total;
- ``--generate --models DIR``: the plan's models from the rack generator, and
  ``DIR/manifest.json`` (profile, nodes, sha256, bytes, the generator's stats);
- ``--run --models DIR --out DIR --binary PATH``: the plan's runs, within a
  window budget (``--budget-s``); ``--write-results`` keeps each result's bytes;
- ``--packet --out DIR``: ``DIR/packet.json`` from the run records.

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
import subprocess
import sys
import threading
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import k6_runner as k6  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, '..', '..', '..', '..'))
GENERATOR = os.path.join(HERE, '..', 'rack', 'rack_model.py')

DRIVER_SCHEMA = 'release-timing-v1'
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
PROFILES = ('linear-ex', 'nonlinear-ex', 'nonlinear-pf')
MODES = ('sparse', 'dense')

# The default plan, in run order. Estimates are dry runs of these rack models with this driver
# on one shared slot of the development host (main 7e061cf2e; not measurements); they order the
# window and bound it, nothing else. Dry runs: linear-ex 250 nodes 27 s and 2.7 GB peak footprint,
# 500 nodes 76 s and 5.6 GB; nonlinear-pf 500 nodes 97 s and 1,000 nodes 1,466 s (the dense
# nonlinear loop's Cholesky per friction iteration; RT-A3 stage P).
_DEFAULT = (
    # profile, nodes, timeout_s, estimate_s, expect
    ('linear-ex', 250, 120, 30, 'solved'),
    ('linear-ex', 500, 240, 80, 'solved'),
    ('linear-ex', 1000, 480, 220, 'solved'),
    # Its attribution twin: stack snapshots while it runs (timings perturbed, not measurements).
    ('linear-ex', 1000, 480, 240, 'solved', True),
    # Projected above the 12 GiB base-machine budget (about 22 GB) and beyond one window.
    ('linear-ex', 2000, 900, 650, 'exceeds_base_machine_budget'),
    ('nonlinear-pf', 250, 60, 15, 'solved'),
    ('nonlinear-pf', 500, 240, 100, 'solved'),
    # Refused before any n^2 allocation by the nonlinear loop's dense-memory guard.
    ('nonlinear-pf', 2000, 60, 2, 'refused:NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED'),
    # The dense loop's cubic cost puts these beyond one window until RT-A3 stage P.
    ('nonlinear-pf', 1000, 2400, 1500, 'solved'),
    ('nonlinear-pf', 1500, 7200, 5000, 'solved'),
    # Until RT-A3 increment 4 the exact route's nonlinear supports are emitted in their linear
    # interim forms, so these solve OPE linearly (rack_model.py LEDGER).
    ('nonlinear-ex', 1000, 480, 40, 'solved'),
    ('nonlinear-ex', 2000, 900, 120, 'solved'),
)

SUBSETS = {
    'all': lambda r: True,
    # One exclusive window of at most 15 minutes: the scaling points that fit.
    'window': lambda r: not r['attribute'] and (r['profile'], r['nodes']) in {
        ('linear-ex', 250), ('linear-ex', 500), ('linear-ex', 1000), ('nonlinear-pf', 250), ('nonlinear-pf', 500),
        ('nonlinear-pf', 2000)},
    # A second window: linear-ex 1000 timed, then its attribution twin.
    'window2': lambda r: (r['profile'], r['nodes']) == ('linear-ex', 1000),
    'linear': lambda r: r['profile'] == 'linear-ex',
    'nonlinear-pf': lambda r: r['profile'] == 'nonlinear-pf',
    'nonlinear-ex': lambda r: r['profile'] == 'nonlinear-ex',
    'nonlinear': lambda r: r['profile'] in ('nonlinear-pf', 'nonlinear-ex'),
    'criterion': lambda r: r['nodes'] == 2000,
}

CONDITIONS = (
    'Engine stages only: the release_timing driver runs, in one process, the stages the desktop solve worker runs for '
    'an ordinary solve (read, parse, request, entry, serialize, release_envelope, parse_back). The desktop app, its IPC '
    'and ingress, the worker process protocol and memory pool, the solve attestation digests and the webview are not '
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
    'refuse that result.',
    'The ps RSS watchdog is an emergency cap only (16 GiB by default, the base machine physical memory). RSS can run '
    'well above phys_footprint (about 1.4 times at 500 linear-ex nodes), so it is not the product measure; a kill '
    'there is killed_by_rss_cap unless a footprint reading shows the budget exceeded.',
    'The host is the one described in metadata (memory_bytes, cpu_model); it is not necessarily the 16 GiB base '
    'machine, so memory pressure, paging and compression on a 16 GiB machine are not reproduced.',
    'Models are invented rack models from rack_model.py, identified per run by sha256.',
    'An attribution run (run id ending -attr) takes stack snapshots with macOS sample (1 s at 1 ms every 2 s, and '
    'whenever the footprint has grown by more than 512 MiB since the previous snapshot), each with the footprints at '
    'its start and the top call paths reduced to product frames. Sampling suspends the process at every sample, so '
    'its timings are not measurements; its footprint watchdog and classification are unchanged.',
    'Observation only: no time or memory threshold is asserted, and criterion 9 provisional bounds are not evaluated.',
)


def run_id_of(profile, nodes, mode, attribute=False):
    return '%s-n%04d-%s%s' % (profile, nodes, mode, '-attr' if attribute else '')


def model_name(profile, nodes):
    return '%s-n%04d.json' % (profile, nodes)


def build_plan(rows, mode='sparse', rss_cap_bytes=EMERGENCY_RSS_CAP_BYTES,
               footprint_budget_bytes=BASE_MACHINE_BUDGET_BYTES):
    """A plan from (profile, nodes, timeout_s, estimate_s, expect[, attribute]) rows, in row order."""
    plan = []
    for i, row in enumerate(rows):
        p, n, t, e, x = row[:5]
        attribute = bool(row[5]) if len(row) > 5 else False
        plan.append({'order': i, 'run_id': run_id_of(p, n, mode, attribute), 'profile': p, 'nodes': n, 'mode': mode,
                     'timeout_s': t, 'footprint_budget_bytes': footprint_budget_bytes, 'rss_cap_bytes': rss_cap_bytes,
                     'estimate_s': e, 'expect': x, 'attribute': attribute})
    return plan


def with_attribution(plan):
    """`--attribute`: every run of `plan` as an attribution run (one per run id, plan order kept)."""
    out, seen = [], set()
    for r in plan:
        run = dict(r, attribute=True, run_id=run_id_of(r['profile'], r['nodes'], r['mode'], True))
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


def format_plan(plan, budget_s=WINDOW_BUDGET_S):
    out = ['order run_id                        profile       nodes mode    estimate_s timeout_s budget_GiB '
           'rss_cap_GiB expect']
    for r in plan:
        out.append('%5d %-29s %-13s %5d %-7s %10d %9d %10.2f %11.2f %s' % (
            r['order'], r['run_id'], r['profile'], r['nodes'], r['mode'], r['estimate_s'], r['timeout_s'],
            r['footprint_budget_bytes'] / GIB, r['rss_cap_bytes'] / GIB, r['expect']))
    total = plan_total_s(plan)
    out.append('total estimate: %d s (%.1f min); window budget %d s; runs whose estimate exceeds the remaining '
               'budget are skipped, and each timeout is capped at the remaining budget.' % (total, total / 60, budget_s))
    out.append('estimates: dry runs of these rack models with this driver on a shared development host, not '
               'measurements. budget: the phys_footprint kill (the product budget); rss_cap: the emergency ps RSS kill. '
               'A run id ending -attr takes stack snapshots; its timings are not measurements.')
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


def generate(plan, models_dir, generator=GENERATOR, python=sys.executable, run=subprocess.run, log=print):
    """Each distinct (profile, nodes) of the plan, from the rack generator, and the manifest."""
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
        done = run([python, generator, '--profile', r['profile'], '--nodes', str(r['nodes']), '-o', path],
                   capture_output=True, text=True, check=False)
        if done.returncode != 0 or not os.path.exists(path):
            failures.append(name)
            log('generate %s: failed (exit %s) %s' % (name, done.returncode, k6.sanitize(done.stderr.strip()[-500:])))
            continue
        manifest['models'][name] = {'profile': r['profile'], 'nodes': r['nodes'], 'sha256': sha256_file(path),
                                    'bytes': os.path.getsize(path), 'stats': last_json_line(done.stderr)}
        log('generate %s: %d bytes' % (name, manifest['models'][name]['bytes']))
    manifest['schema'] = MANIFEST_SCHEMA
    manifest['generator'] = {'name': os.path.basename(generator), 'sha256': sha256_file(generator)}
    with open(os.path.join(models_dir, 'manifest.json'), 'w') as fh:
        json.dump(manifest, fh, indent=1, sort_keys=True)
        fh.write('\n')
    return manifest, failures


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
        path = os.path.join(self.record_dir, '%s.sample-%03d.txt' % (self.run_id, snapshot['index']))
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
    path = os.path.join(record_dir, run_id + '.stderr.txt')
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
        values.append(driver.get('worker_peak_footprint_bytes_equivalent'))
    if progress_memory:
        values += [progress_memory.get('phys_footprint_bytes'), progress_memory.get('lifetime_max_phys_footprint_bytes')]
    values = [v for v in values if isinstance(v, int) and not isinstance(v, bool)]
    return max(values) if values else None


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
    if status == SOLVED:
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
    path = os.path.join(record_dir, run_id + '.jsonl')
    if not os.path.exists(path):
        return None
    with open(path) as fh:
        for obj in k6.jsonl(fh):
            if obj.get('schema') == DRIVER_SCHEMA:
                return obj
    return None


def driver_argv(binary, model_path, run, results_dir=None):
    argv = [binary, model_path, '--mode', run['mode']]
    if results_dir:
        argv += ['--write', os.path.join(results_dir, run['run_id'] + '.result.json')]
    return argv


def git(args, cwd=REPO):
    done = k6.run_quiet(['git', '-C', cwd] + args)
    return done.stdout.strip() if done is not None and done.returncode == 0 else None


def source_identity():
    """The runner checkout's commit and tree (hashes only) and whether its tree is dirty."""
    status = git(['status', '--porcelain'])
    return git(['rev-parse', 'HEAD']), git(['rev-parse', 'HEAD^{tree}']), (bool(status) if status is not None else None)


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
    start = clock()
    if wrapper is None:
        wrapper = os.path.exists(k6.TIME_WRAPPER)
    os.makedirs(out_dir, exist_ok=True)
    if os.path.exists(os.path.join(out_dir, 'records.jsonl')):
        raise RuntimeError('records already exist in the output folder; use a new one')
    if md is None:
        md = metadata(binary, *source_identity())
    md = dict(md, budget_s=budget_s, write_results=write_results, quiet_wait_s=quiet_wait_s,
              plan=[r['run_id'] for r in plan])
    with open(os.path.join(out_dir, 'metadata.json'), 'w') as fh:
        json.dump(md, fh, indent=1, sort_keys=True)
        fh.write('\n')
    manifest = read_manifest(models_dir).get('models', {})
    results_dir = os.path.join(out_dir, 'results') if write_results else None
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
        'order': r.get('order'), 'run_id': r['run_id'], 'profile': r['profile'], 'nodes': r['nodes'],
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
            'driver_lifetime_max_phys_footprint': driver.get('worker_peak_footprint_bytes_equivalent'),
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


def packet(out_dir):
    records = [r for r in k6.read_records(out_dir) if r.get('schema') == RECORD_SCHEMA]
    md_path = os.path.join(out_dir, 'metadata.json')
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
    with open(os.path.join(out_dir, 'packet.json'), 'w') as fh:
        json.dump(result, fh, indent=1, sort_keys=True)
        fh.write('\n')
    return result


# --------------------------------------------------------------- main


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('--plan', action='store_true', help='print the schedule and its estimated total')
    parser.add_argument('--generate', action='store_true', help="generate the plan's models and the manifest")
    parser.add_argument('--run', action='store_true', help="run the plan (one process at a time)")
    parser.add_argument('--packet', action='store_true', help='write OUT/packet.json from the records')
    parser.add_argument('--subset', default='all', help='%s, or comma-separated run ids' % ', '.join(SUBSETS))
    parser.add_argument('--mode', choices=MODES, default='sparse')
    parser.add_argument('--footprint-budget-bytes', type=int, default=BASE_MACHINE_BUDGET_BYTES,
                        help='the phys_footprint kill (default: the 12 GiB base-machine budget)')
    parser.add_argument('--rss-cap-bytes', type=int, default=EMERGENCY_RSS_CAP_BYTES,
                        help='the emergency ps RSS kill (default: 16 GiB, the base machine memory)')
    parser.add_argument('--budget-s', type=int, default=WINDOW_BUDGET_S)
    parser.add_argument('--quiet-wait-s', type=int, default=QUIET_WAIT_S)
    parser.add_argument('--models')
    parser.add_argument('--out')
    parser.add_argument('--binary')
    parser.add_argument('--generator', default=GENERATOR)
    parser.add_argument('--attribute', action='store_true',
                        help='run every selected run as an attribution run (stack snapshots; timings not measurements)')
    parser.add_argument('--keep-samples', action='store_true', help="keep attribution runs' raw sample reports")
    parser.add_argument('--write-results', action='store_true', help='keep each result (large: about 1.5 GB at 2,000 nodes)')
    args = parser.parse_args(argv)
    try:
        plan = select(default_plan(args.mode, args.rss_cap_bytes, args.footprint_budget_bytes), args.subset)
        if args.attribute:
            plan = with_attribution(plan)
    except ValueError as error:
        parser.error(str(error))
    if args.plan:
        sys.stdout.write(format_plan(plan, args.budget_s))
        return 0
    if args.generate:
        if not args.models:
            parser.error('--generate needs --models')
        _, failures = generate(plan, args.models, generator=args.generator)
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
        result = write_packet(args.out)
        print(json.dumps({'packet': 'packet.json', 'classification_counts': result['classification_counts']}))
        return 0
    parser.print_help()
    return 2


if __name__ == '__main__':
    sys.exit(main())
