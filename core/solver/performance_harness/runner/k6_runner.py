#!/usr/bin/env python3
"""K6 runner (T3 D1 revision 5a.2 section 4.8): standard library only.

This file carries, in order:

1. An independent Python generator of K6's kernel models and their canonical
   bytes (``k6-model v1``). It is written from R1's family definitions
   (``T3/REFERENCES/references.py``, sha256 ``80d473a7...``) and the harness's
   invented fixtures, not from the Rust generator, and it is what checks
   ``H/observations/k6/models_sha256.txt`` without a Rust build (ROOT's K6
   ruling N13).
2. The closed-form counts (K6 plan section 4.2).
3. The runner itself: the process tree, the RSS watchdog, the limits, the
   parsers, the classification, the aggregation and the packet (checkpoint
   A2).

Observation only: nothing here asserts a time or memory bound.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
import platform
import re
import signal
import struct
import subprocess
import sys
import threading
import time

# --------------------------------------------------------------- models

R1_SOURCE = "r1:references.json@7b176dbb"
RF_LARGE_SIZES = (10, 100, 1000, 10000)
CEILING_CHAIN_MEMBERS = 1364
Q5_GRID_SIDES = (16, 32, 64, 96, 128)
DOF = 6
# R1's Q3 (references.py:1298), rows.
Q3 = ((1, 2, 2), (2, 1, -2), (-2, 2, -1))
# CHAIN's (s, s2) and TREE's sc per n: R1's exponents (see the Rust
# generator's `chain_exponents` and `tree_exponent` for their derivation).
CHAIN_EXPONENTS = {10: (-6, 1), 100: (-13, -2), 1000: (-20, -5), 10000: (-26, -9),
                   CEILING_CHAIN_MEMBERS: (-20, -6)}
TREE_EXPONENTS = {10: 0, 100: -6, 1000: -14, 10000: -20}
# The DEC-053 nine, in the harness's order (H/src/lib.rs:704-752).
DEC053 = (('chain', 8), ('chain', 24), ('chain', 48), ('grid', 4, 3), ('grid', 6, 8),
          ('grid', 7, 8), ('grid', 5, 5), ('chain', 32), ('grid', 5, 6))


def bits(value):
    """The 16 hex digits of a binary64."""
    return struct.pack('>d', float(value)).hex()


def rf_large_section():
    """E, G, A, Iy, Iz, J: od and id rounded once, then explicit products and pi."""
    od, id_ = 0.2, 0.18
    od2, id2 = od * od, id_ * id_
    area = math.pi * (od2 - id2) / 4.0
    second = math.pi * (od2 * od2 - id2 * id2) / 64.0
    return (200000000000.0, 80000000000.0, area, second, second, 2.0 * second)


INVENTED_SECTION = (2.0e11, 7.7e10, 0.01, 8.0e-6, 8.0e-6, 1.6e-5)


def q3(v):
    out = []
    for row in Q3:
        total = sum(r * c for r, c in zip(row, v))
        assert total % 3 == 0, v
        out.append(total // 3)
    return tuple(out)


def y_reference(pi, pj):
    """P1's rule (DETECTION/scripts/gen.py.txt): least-aligned axis, Y, Z, X on ties."""
    mags = [abs(pj[k] - pi[k]) for k in range(3)]
    best = min((1, 2, 0), key=lambda k: mags[k])
    return tuple(1.0 if k == best else 0.0 for k in range(3))


class Model:
    def __init__(self, id_, source, section):
        self.id, self.source, self.section = id_, source, section
        self.nodes, self.members, self.restraints, self.loads = [], [], {}, {}

    def node(self, label, p):
        self.nodes.append((label, tuple(float(c) for c in p)))
        return len(self.nodes) - 1

    def load(self, node, offset, v, exponent=0):
        for axis, c in enumerate(v):
            if c != 0:
                self.loads[node * DOF + offset + axis] = math.ldexp(float(c), exponent)

    def canonical(self):
        lines = ['k6-model v1', 'id ' + self.id, 'source ' + self.source,
                 'counts nodes %d members %d loads %d' % (len(self.nodes), len(self.members), len(self.loads)),
                 'section E %s G %s A %s Iy %s Iz %s J %s' % tuple(bits(v) for v in self.section)]
        for i, (label, p) in enumerate(self.nodes):
            lines.append('node %d %s %s %s %s' % (i, label, bits(p[0]), bits(p[1]), bits(p[2])))
        for k, (label, a, b, y) in enumerate(self.members):
            lines.append('member %d %s %d %d %s %s %s' % (k, label, a, b, bits(y[0]), bits(y[1]), bits(y[2])))
        for node in sorted(self.restraints):
            lines.append('restraint %d %s' % (node, ''.join('1' if r else '0' for r in self.restraints[node])))
        for dof in sorted(self.loads):
            lines.append('load %d %s' % (dof, bits(self.loads[dof])))
        return ('\n'.join(lines) + '\n').encode('ascii')


def _integer_frames(model, points, members):
    for label, a, b in members:
        model.members.append((label, a, b, y_reference(points[a], points[b])))


def chain_model(id_, source, n, rotated):
    s, s2 = CHAIN_EXPONENTS[n]
    rot = q3 if rotated else (lambda v: v)
    m = Model(id_, source, rf_large_section())
    pts = [rot((3 * i, 0, 0)) for i in range(n + 1)]
    for i, p in enumerate(pts):
        m.node('N%d' % i, p)
    _integer_frames(m, pts, [('M%d' % i, i - 1, i) for i in range(1, n + 1)])
    m.restraints[0] = (True,) * 6
    m.load(n, 0, rot((6, -3, 9)), s)
    m.load(n, 3, rot((3, 6, -3)), s2)
    return m


def tree_model(id_, n, rotated):
    sc = TREE_EXPONENTS[n]
    rot = q3 if rotated else (lambda v: v)
    m = Model(id_, R1_SOURCE, rf_large_section())
    order = [('P0', (0, 0, 0))]
    for k in range(1, n // 2 + 1):
        order.append(('P%d' % k, rot((3 * k, 0, 0))))
        order.append(('B%d' % k, rot((3 * k, 3, 0) if k % 2 else (3 * k, 0, 3))))
    index = {label: i for i, (label, _) in enumerate(order)}
    pts = [p for _, p in order]
    for label, p in order:
        m.node(label, p)
    members = []
    for k in range(1, n // 2 + 1):
        members.append(('S%d' % k, index['P%d' % (k - 1)], index['P%d' % k]))
        members.append(('Q%d' % k, index['P%d' % k], index['B%d' % k]))
        c = (k % 7) - 3
        if c:
            m.load(index['B%d' % k], 0, rot((0, 0, 3 * c) if k % 2 else (0, 3 * c, 0)), sc)
    _integer_frames(m, pts, members)
    m.restraints[0] = (True,) * 6
    return m


def cont_model(id_, n, rotated):
    s = n // 2
    rot = q3 if rotated else (lambda v: v)
    m = Model(id_, R1_SOURCE, rf_large_section())
    order = [('S%d' % i, rot((6 * i, 0, 0))) for i in range(s + 1)]
    order += [('C%d' % j, rot((3 * (2 * j - 1), 0, 0))) for j in range(1, s + 1)]
    index = {label: i for i, (label, _) in enumerate(order)}
    pts = [p for _, p in order]
    for label, p in order:
        m.node(label, p)
    members = []
    for j in range(1, s + 1):
        members.append(('A%d' % j, index['S%d' % (j - 1)], index['C%d' % j]))
        members.append(('B%d' % j, index['C%d' % j], index['S%d' % j]))
    _integer_frames(m, pts, members)
    m.restraints[0] = (True,) * 6
    for i in range(1, s + 1):
        m.restraints[index['S%d' % i]] = (True, True, True, False, False, False)
    for j in range(1, s + 1):
        py, pz = 96 * ((j % 5) - 2), 96 * ((j % 3) - 1)
        if py or pz:
            m.load(index['C%d' % j], 0, rot((0, py, pz)))
    return m


def invented_chain(id_, source, k):
    """invented_cantilever_chain_fixture (H/src/lib.rs:347-395)."""
    m = Model(id_, source, INVENTED_SECTION)
    for i in range(k + 1):
        m.node('n%d' % i, (i, 0, 0))
    for i in range(k):
        m.members.append(('e%d' % i, i, i + 1, (0.0, 1.0, 0.0)))
    m.restraints[0] = (True,) * 6
    m.loads[k * DOF + 1] = -1.0e3
    return m


def invented_grid(id_, source, x, y):
    """invented_grid_frame_fixture (H/src/lib.rs:403-472)."""
    m = Model(id_, source, INVENTED_SECTION)
    for j in range(y):
        for i in range(x):
            m.node('n%d' % (j * x + i), (i, j, 0))
    for j in range(y):
        for i in range(x):
            k = j * x + i
            if i + 1 < x:
                m.members.append(('e%d' % len(m.members), k, k + 1, (0.0, 1.0, 0.0)))
            if j + 1 < y:
                m.members.append(('e%d' % len(m.members), k, k + x, (0.0, 0.0, 1.0)))
    for i in range(x):
        m.restraints[i] = (True,) * 6
    m.loads[(x * y - 1) * DOF] = 1.0e3
    return m


def dec053_fixture_id(spec):
    if spec[0] == 'chain':
        return 'invented-cantilever-chain-%d' % spec[1]
    return 'invented-grid-frame-%dx%d' % (spec[1], spec[2])


def sealed_model_ids():
    ids = ['RF-LARGE-%s-n%05d-%s' % (f, n, o) for f in ('CHAIN', 'TREE', 'CONT')
           for n in RF_LARGE_SIZES for o in ('AX', 'ROT')]
    return ids + ['DEC053:' + dec053_fixture_id(s) for s in DEC053]


def extra_model_ids():
    return ['K6-CEIL-CHAIN-n%05d-AX' % CEILING_CHAIN_MEMBERS] + \
        ['K6-GRID-%dx%d' % (s, s) for s in Q5_GRID_SIDES]


def model(id_):
    if id_.startswith('RF-LARGE-'):
        family, size, orientation = id_[len('RF-LARGE-'):].split('-')
        n, rotated = int(size[1:]), orientation == 'ROT'
        if family == 'CHAIN':
            return chain_model(id_, R1_SOURCE, n, rotated)
        if family == 'TREE':
            return tree_model(id_, n, rotated)
        return cont_model(id_, n, rotated)
    if id_.startswith('DEC053:'):
        spec = next(s for s in DEC053 if dec053_fixture_id(s) == id_[len('DEC053:'):])
        source = 'dec053:' + dec053_fixture_id(spec)
        if spec[0] == 'chain':
            return invented_chain(id_, source, spec[1])
        return invented_grid(id_, source, spec[1], spec[2])
    if id_ == 'K6-CEIL-CHAIN-n%05d-AX' % CEILING_CHAIN_MEMBERS:
        return chain_model(id_, 'k6-invented:rf-large-chain-rule', CEILING_CHAIN_MEMBERS, False)
    if id_.startswith('K6-GRID-'):
        side = int(id_[len('K6-GRID-'):].split('x')[0])
        return invented_grid(id_, 'k6-invented:invented_grid_frame_fixture', side, side)
    raise KeyError(id_)


def model_sha256(id_):
    return hashlib.sha256(model(id_).canonical()).hexdigest()


# --------------------------------------------------------------- closed forms


def closed_forms(m):
    """The counts that depend only on nodes, members and restraints (plan 4.2).

    Premises, true for every K6 model: each node carries at least one frame, no
    two frames join the same node pair, and there are no springs or blocks.
    """
    nodes, members = len(m.nodes), len(m.members)
    free = [DOF - sum(m.restraints.get(k, (False,) * 6)) for k in range(nodes)]
    free_entries = sum(f * f for f in free) + 2 * sum(free[a] * free[b] for _, a, b, _ in m.members)
    free_dofs = sum(free)
    return {
        'nodes': nodes, 'members': members, 'dofs': DOF * nodes, 'free_dofs': free_dofs,
        'restrained_dofs': DOF * nodes - free_dofs,
        'pattern_entries': 36 * (nodes + 2 * members),
        'lower_entries': 18 * (nodes + 2 * members) + 3 * nodes,
        'free_entries': free_entries, 'free_lower_entries': (free_entries + free_dofs) // 2,
        'contributions': 144 * members, 'dense_entries': (DOF * nodes) ** 2,
    }


# --------------------------------------------------------------- constants

KIB, MIB, GIB = 1024, 1024 ** 2, 1024 ** 3
C_DEFAULT = 8 * GIB          # RSS watchdog cap (ROOT's K6 ruling Q3)
C_CEILING = 16 * GIB         # the Q4 ceiling run (ROOT's K6 ruling Q4)
HEAP_MARGIN = 512 * MIB      # heap cap = C - 512 MiB
TIMEOUT_SMALL, TIMEOUT_LARGE = 600, 1800   # P1's (DETECTION/scripts/run.py.txt:21-22)
BUDGET_MARGIN = 60           # --time-budget-s = timeout - 60
FIRST_REPEAT_LIMIT = 600
POLL_S = 0.1                 # the watchdog's 100 ms (DESIGN.md:826)
MEMORYSTATUS_FLOOR = 80      # ROOT's K6 ruling N18
MODES = ('sparse', 'dense', 'lane-id', 'lane-lu')
N2_MODES = ('dense', 'lane-lu')
N2_REFUSAL_MEMBERS = 10000
RHO_DEFAULT = 2.0
RHO_MIN_MEMBERS_FOR_LARGE = 100   # ROOT's A1-stop ruling Q2
LARGE_MEMBERS = 1000
KNOWN_DENSE_TIMEOUTS = ('RF-LARGE-CHAIN-n01000-ROT', 'RF-LARGE-TREE-n01000-AX')
CONDITIONAL_MODELS = ('K6-GRID-128x128',)   # ROOT's K6 ruling N14
TIME_WRAPPER = '/usr/bin/time'
HEAP_CAP_MARKER = 'k6_observe: heap cap refused '
RUST_ALLOC_FAILURE = 'memory allocation of '
PYTHON_MEMORY_ERROR = 'MemoryError'

# P1's product-level Linux peaks (MiB), c61a540ea, RLIMIT_AS 6 GiB
# (T3/DETECTION/RETURN.md:230-265). The 10,000-member runs all aborted at the
# 6 GiB limit, so no peak is known there. Two dense 1,000-member runs timed out
# at 1,800 s; their peak up to the kill is recorded as P1 recorded it.
P1_LINUX_PEAK_MIB = {
    ('RF-LARGE-CHAIN-n00010-AX', 'sparse'): 34.4, ('RF-LARGE-CHAIN-n00010-AX', 'dense'): 34.4,
    ('RF-LARGE-CHAIN-n00010-ROT', 'sparse'): 34.4, ('RF-LARGE-CHAIN-n00010-ROT', 'dense'): 34.4,
    ('RF-LARGE-TREE-n00010-AX', 'sparse'): 34.4, ('RF-LARGE-TREE-n00010-AX', 'dense'): 34.4,
    ('RF-LARGE-TREE-n00010-ROT', 'sparse'): 34.6, ('RF-LARGE-TREE-n00010-ROT', 'dense'): 34.4,
    ('RF-LARGE-CONT-n00010-AX', 'sparse'): 34.4, ('RF-LARGE-CONT-n00010-AX', 'dense'): 34.4,
    ('RF-LARGE-CONT-n00010-ROT', 'sparse'): 34.4, ('RF-LARGE-CONT-n00010-ROT', 'dense'): 34.4,
    ('RF-LARGE-CHAIN-n00100-AX', 'sparse'): 49.6, ('RF-LARGE-CHAIN-n00100-AX', 'dense'): 49.9,
    ('RF-LARGE-CHAIN-n00100-ROT', 'sparse'): 49.8, ('RF-LARGE-CHAIN-n00100-ROT', 'dense'): 50.1,
    ('RF-LARGE-TREE-n00100-AX', 'sparse'): 50.5, ('RF-LARGE-TREE-n00100-AX', 'dense'): 50.5,
    ('RF-LARGE-TREE-n00100-ROT', 'sparse'): 52.6, ('RF-LARGE-TREE-n00100-ROT', 'dense'): 52.6,
    ('RF-LARGE-CONT-n00100-AX', 'sparse'): 52.3, ('RF-LARGE-CONT-n00100-AX', 'dense'): 51.6,
    ('RF-LARGE-CONT-n00100-ROT', 'sparse'): 52.7, ('RF-LARGE-CONT-n00100-ROT', 'dense'): 51.7,
    ('RF-LARGE-CHAIN-n01000-AX', 'sparse'): 3605.2, ('RF-LARGE-CHAIN-n01000-AX', 'dense'): 3605.2,
    ('RF-LARGE-CHAIN-n01000-ROT', 'sparse'): 3606.5, ('RF-LARGE-CHAIN-n01000-ROT', 'dense'): 3606.7,
    ('RF-LARGE-TREE-n01000-AX', 'sparse'): 3608.7, ('RF-LARGE-TREE-n01000-AX', 'dense'): 3609.1,
    ('RF-LARGE-TREE-n01000-ROT', 'sparse'): 3617.2, ('RF-LARGE-TREE-n01000-ROT', 'dense'): 3617.2,
    ('RF-LARGE-CONT-n01000-AX', 'sparse'): 3372.8, ('RF-LARGE-CONT-n01000-AX', 'dense'): 3373.0,
    ('RF-LARGE-CONT-n01000-ROT', 'sparse'): 3377.2, ('RF-LARGE-CONT-n01000-ROT', 'dense'): 3377.2,
}


# --------------------------------------------------------------- schedule


def rf_ids(n):
    return ['RF-LARGE-%s-n%05d-%s' % (f, n, o) for f in ('CHAIN', 'TREE', 'CONT') for o in ('AX', 'ROT')]


TIERS = (
    ('T1', 'B1', rf_ids(10) + ['DEC053:' + dec053_fixture_id(s) for s in DEC053], MODES),
    ('T2', 'B1', rf_ids(100), MODES),
    ('T3a', 'B1', rf_ids(1000), ('sparse', 'lane-id')),
    ('T3b', 'B2', rf_ids(1000), ('dense', 'lane-lu')),
    ('T4', 'B1', rf_ids(10000), MODES),
    ('T5', 'B1', ['K6-GRID-%dx%d' % (s, s) for s in Q5_GRID_SIDES], ('sparse',)),
    ('T6', 'B3', ['K6-CEIL-CHAIN-n%05d-AX' % CEILING_CHAIN_MEMBERS], ('dense',)),
)


def rotate(modes, index):
    """Q7(a): which mode runs first alternates by the model's index in its tier."""
    k = index % len(modes)
    return tuple(modes[k:]) + tuple(modes[:k])


def members_of(model_id):
    m = re.search(r'-n(\d{5})-', model_id)
    if m:
        return int(m.group(1))
    return len(model(model_id).members)


def family_of(model_id):
    if model_id.startswith('RF-LARGE-') or model_id.startswith('K6-CEIL-'):
        return model_id.split('-')[2]
    if model_id.startswith('K6-GRID-'):
        return 'GRID'
    return 'DEC053'


def run_parameters(model_id, mode, members):
    """Repeats, entry repeats, caps and timeouts (Q3, Q4, Q7, N3, N11)."""
    ceiling = model_id.startswith('K6-CEIL-')
    c = C_CEILING if ceiling else C_DEFAULT
    repeats = 5
    if mode == 'dense' and model_id in KNOWN_DENSE_TIMEOUTS:
        repeats = 1                      # N11: the two known dense timeouts
    if mode == 'lane-lu' and members >= LARGE_MEMBERS:
        repeats = 1                      # N11: lane-lu at 1,000 members
    entry_repeats = repeats
    if mode == 'dense' and (members >= LARGE_MEMBERS or ceiling):
        entry_repeats = 1                # N3
    timeout = TIMEOUT_LARGE if members >= LARGE_MEMBERS else TIMEOUT_SMALL
    return {'repeats': repeats, 'entry_repeats': entry_repeats, 'rss_cap_bytes': c,
            'heap_cap_bytes': c - HEAP_MARGIN, 'timeout_s': timeout,
            'time_budget_s': timeout - BUDGET_MARGIN, 'first_repeat_limit_s': FIRST_REPEAT_LIMIT}


def schedule():
    runs = []
    for tier, slot, ids, modes in TIERS:
        for index, model_id in enumerate(ids):
            members = members_of(model_id)
            for mode in rotate(modes, index):
                run = {'order': len(runs) + 1, 'tier': tier, 'slot': slot, 'model': model_id, 'mode': mode,
                       'family': family_of(model_id), 'members': members,
                       'conditional': model_id in CONDITIONAL_MODELS}
                run.update(run_parameters(model_id, mode, members))
                run['run_id'] = '%03d_%s_%s' % (run['order'], model_id.replace(':', '_'), mode)
                runs.append(run)
    return runs


# --------------------------------------------------------------- admission


def refusal_by_name(model_id, mode, members):
    """The never-run refusals, by name (host rule; ROOT's rulings Q12 and on RV16-N4)."""
    if mode in N2_MODES and members >= N2_REFUSAL_MEMBERS:
        return 'never:n2_mode_at_or_above_10000_members'
    if mode == 'lane-id' and model_id.startswith('RF-LARGE-CONT-n10000-'):
        return 'never:cont_n10000_identity_lane'
    return None


def estimate_key(mode):
    return 'estimate_adm_bytes_' + mode.replace('-', '_')


def measured_ratio(record, baseline_rss_bytes):
    """ρ for one measured run: max(RSS net of the no-op baseline, move-model heap peak) / E_adm."""
    estimate = record.get('estimate_adm_bytes')
    if not estimate:
        return None
    rss = record.get('peak_rss_bytes')
    rss_net = max(0, rss - (baseline_rss_bytes or 0)) if rss is not None else 0
    heap = record.get('repeats_heap_peak_move') or 0
    return max(rss_net, heap) / estimate


def previous_size(model_id):
    """The ascent's previous size of the same family, orientation and mode (None at the bottom)."""
    if model_id.startswith('RF-LARGE-'):
        n = members_of(model_id)
        k = RF_LARGE_SIZES.index(n)
        return None if k == 0 else model_id.replace('-n%05d-' % n, '-n%05d-' % RF_LARGE_SIZES[k - 1])
    if model_id.startswith('K6-GRID-'):
        side = int(model_id[len('K6-GRID-'):].split('x')[0])
        k = Q5_GRID_SIDES.index(side)
        return None if k == 0 else 'K6-GRID-%dx%d' % (Q5_GRID_SIDES[k - 1], Q5_GRID_SIDES[k - 1])
    if model_id.startswith('K6-CEIL-'):
        return 'RF-LARGE-CHAIN-n01000-AX'
    return None


def admission(run, counts, measured, baseline_rss_bytes, *, require_ascent=True):
    """ROOT's K6 ruling Q3, with N9 and the A1-stop ruling Q2.

    counts: {model: counts line}; measured: runner records so far.
    Returns a dict with decision (never / admitted / deferred), reason, the
    estimate, ρ and the C/2 bound.
    """
    model_id, mode, members = run['model'], run['mode'], run['members']
    half = run['rss_cap_bytes'] // 2
    out = {'decision': None, 'reason': None, 'estimate_adm_bytes': None, 'rho': None,
           'half_cap_bytes': half}
    never = refusal_by_name(model_id, mode, members)
    if never:
        out.update(decision='never', reason=never)
        return out
    line = counts.get(model_id)
    if line is None:
        out.update(decision='deferred', reason='deferred:no_counts_line')
        return out
    estimate = line[estimate_key(mode)]
    out['estimate_adm_bytes'] = estimate
    previous = previous_size(model_id)
    if require_ascent and previous is not None:
        done = [r for r in measured if r['model'] == previous and r['mode'] == mode
                and r.get('classification') not in (None, 'not_run')]
        if not done:
            out.update(decision='deferred', reason='deferred:ascent_previous_size_not_recorded (%s)' % previous)
            return out
    p1 = P1_LINUX_PEAK_MIB.get((model_id, mode))
    if p1 is not None and p1 * MIB <= half:
        out.update(decision='admitted', reason='p1_linux_peak %.1f MiB <= C/2' % p1)
        return out
    floor = RHO_MIN_MEMBERS_FOR_LARGE if members >= LARGE_MEMBERS else 0
    ratios = [measured_ratio(r, baseline_rss_bytes) for r in measured
              if r['family'] == run['family'] and r['mode'] == mode
              and floor <= r['members'] < members and r.get('classification') == 'ok']
    ratios = [x for x in ratios if x is not None]
    rho = max(ratios) if ratios else RHO_DEFAULT
    out['rho'] = rho
    if estimate * rho <= half:
        out.update(decision='admitted', reason='estimate %d B x rho %.3f <= C/2' % (estimate, rho))
    else:
        out.update(decision='deferred', reason='deferred:estimate_fails_admission (%d B x rho %.3f > C/2)'
                   % (estimate, rho))
    return out


# --------------------------------------------------------------- parsers


def parse_time_macos(text):
    """/usr/bin/time -l: maximum resident set size and peak memory footprint, in bytes."""
    out = {}
    m = re.search(r'^\s*(\d+)\s+maximum resident set size', text, re.M)
    if m:
        out['time_max_rss_bytes'] = int(m.group(1))
    m = re.search(r'^\s*(\d+)\s+peak memory footprint', text, re.M)
    if m:
        out['time_peak_footprint_bytes'] = int(m.group(1))
    return out


def parse_time_gnu(text):
    """GNU /usr/bin/time -v: Maximum resident set size (kbytes), normalized to bytes."""
    out = {}
    m = re.search(r'Maximum resident set size \(kbytes\):\s*(\d+)', text)
    if m:
        out['time_max_rss_bytes'] = int(m.group(1)) * KIB
    m = re.search(r'Command terminated by signal (\d+)', text)
    if m:
        out['time_child_signal'] = int(m.group(1))
    return out


def parse_time(text, system):
    return parse_time_macos(text) if system == 'Darwin' else parse_time_gnu(text)


def ru_maxrss_bytes(value, system):
    """wait4's ru_maxrss: bytes on macOS, KiB on Linux."""
    return value if system == 'Darwin' else value * KIB


def ps_kib_to_bytes(kib):
    return kib * KIB


def watchdog_exceeds(rss_kib, cap_bytes):
    """ps -o rss= reports KiB; the cap is in bytes."""
    return rss_kib * KIB > cap_bytes


ABSOLUTE_PATH = re.compile(r'/(?:Users|home|private|tmp|var|opt|Volumes)/[^\s:\'"]*')


def sanitize(text):
    """No machine path in a record: absolute paths become <path>."""
    return ABSOLUTE_PATH.sub('<path>', text)


def jsonl(lines):
    out = []
    for line in lines:
        line = line.strip()
        if line.startswith('{'):
            try:
                out.append(json.loads(line))
            except json.JSONDecodeError:
                pass
    return out


# --------------------------------------------------------------- classification

CLASSES = ('ok', 'timed_out', 'killed_by_rss_watchdog', 'heap_cap_abort', 'rlimit_abort',
           'refused_by_binary', 'error')


def classify(*, killed_by_rss_watchdog, timed_out, exit_code, objects, stderr_text, system,
             rlimit_applied, k6_protocol=True, external_signal=None):
    if killed_by_rss_watchdog:
        return 'killed_by_rss_watchdog', None
    if timed_out:
        return 'timed_out', None
    kinds = {o.get('kind') for o in objects}
    if k6_protocol and 'refusal' in kinds and exit_code == 3:
        return 'refused_by_binary', None
    if HEAP_CAP_MARKER in stderr_text and 'summary' not in kinds:
        return 'heap_cap_abort', None
    if (system == 'Linux' and rlimit_applied and exit_code != 0 and 'summary' not in kinds
            and (RUST_ALLOC_FAILURE in stderr_text or PYTHON_MEMORY_ERROR in stderr_text)):
        return 'rlimit_abort', None
    if exit_code == 0 and (not k6_protocol or 'summary' in kinds):
        return 'ok', None
    if external_signal is not None:
        return 'error', 'external_signal_%d' % external_signal
    return 'error', 'exit_code_%s' % exit_code


# --------------------------------------------------------------- aggregation


def median(values):
    ordered = sorted(values)
    n = len(ordered)
    if n == 0:
        return None
    if n % 2:
        return ordered[n // 2]
    return (ordered[n // 2 - 1] + ordered[n // 2]) / 2


def aggregate_stages(objects):
    """Per stage, over the completed repeats: median and minimum elapsed, the peaks."""
    stages = {}
    for o in objects:
        if o.get('kind') != 'stage':
            continue
        entry = stages.setdefault(o['stage'], {'elapsed_ns': [], 'heap_peak': 0, 'heap_peak_move': 0,
                                              'ok': True})
        entry['elapsed_ns'].append(o['elapsed_ns'])
        entry['heap_peak'] = max(entry['heap_peak'], o['heap_peak'])
        entry['heap_peak_move'] = max(entry['heap_peak_move'], o['heap_peak_move'])
        entry['ok'] = entry['ok'] and o['ok']
    out = {}
    for name, entry in stages.items():
        values = entry.pop('elapsed_ns')
        out[name] = dict(entry, samples=len(values), median_ns=median(values), min_ns=min(values))
    return out


def summary_of(objects):
    for o in objects:
        if o.get('kind') == 'summary':
            return o
    return None


def first_of(objects, kind):
    for o in objects:
        if o.get('kind') == kind:
            return o
    return None


def parity_failures(objects):
    """Parity lines that are a stop (K6 brief, Checkpoints)."""
    return [o for o in objects if o.get('kind') == 'parity' and o.get('equal') is False
            and o.get('item') != 'entry_plain_vs_entry_checked']


def read_solution(path):
    with open(path) as fh:
        return [struct.unpack('>d', bytes.fromhex(line.strip()))[0] for line in fh if line.strip()]


def dec053_delta(sparse, dense):
    delta = max((abs(a - b) for a, b in zip(sparse, dense)), default=0.0)
    scale = max((abs(b) for b in dense), default=0.0)
    return delta, scale, (delta / scale if scale > 0 else 0.0)


def fit_loglog(points):
    """Least squares of log(y) on log(x): slope, intercept and residuals (observed fit, not a threshold)."""
    pts = [(math.log(x), math.log(y)) for x, y in points if x > 0 and y > 0]
    if len(pts) < 2:
        return None
    n = len(pts)
    mx = sum(p[0] for p in pts) / n
    my = sum(p[1] for p in pts) / n
    sxx = sum((p[0] - mx) ** 2 for p in pts)
    if sxx == 0:
        return None
    slope = sum((p[0] - mx) * (p[1] - my) for p in pts) / sxx
    intercept = my - slope * mx
    return {'slope': slope, 'intercept': intercept, 'points': points,
            'residuals': [p[1] - (intercept + slope * p[0]) for p in pts]}


# --------------------------------------------------------------- host


def run_quiet(argv, env=None):
    try:
        return subprocess.run(argv, capture_output=True, text=True, env=env, check=False)
    except OSError:
        return None


def memorystatus_level(system=None):
    if (system or platform.system()) != 'Darwin':
        return None
    done = run_quiet(['sysctl', '-n', 'kern.memorystatus_level'])
    try:
        return int(done.stdout.strip()) if done else None
    except ValueError:
        return None


def host_busy(run=run_quiet):
    """Another cargo job or a DEC-025 sweep (anchored pattern, _COMMON.md:31, :33)."""
    busy = []
    for name, argv in (('cargo', ['pgrep', '-x', 'cargo']),
                       ('sweep', ['pgrep', '-f', 'python[0-9.]* .*run_evidence_sweep'])):
        done = run(argv)
        if done is not None and done.returncode == 0:
            busy.append(name)
    return busy


def wait_for_quiet_host(log, max_wait_s=1800, run=run_quiet, level=memorystatus_level, sleep=time.sleep):
    waited = 0
    while True:
        busy = host_busy(run)
        mem = level()
        if not busy and (mem is None or mem >= MEMORYSTATUS_FLOOR):
            return {'waited_s': waited, 'memorystatus_level': mem}
        if waited >= max_wait_s:
            raise RuntimeError('host not quiet after %d s: %s, memorystatus %s' % (waited, busy, mem))
        log('host busy (%s; memorystatus %s); waiting 30 s' % (busy, mem))
        sleep(30)
        waited += 30


def metadata(binary, source_commit, source_tree, run=run_quiet, system=None):
    """Hardware, toolchain and build metadata. No host name, user name, serial or path."""
    system = system or platform.system()
    md = {'os': system, 'os_release': platform.release(), 'machine': platform.machine(),
          'python': platform.python_version(), 'build_profile': 'release',
          'source_commit': source_commit, 'source_tree': source_tree}
    if system == 'Darwin':
        for key, argv in (('os_product_version', ['sw_vers', '-productVersion']),
                          ('os_build_version', ['sw_vers', '-buildVersion']),
                          ('cpu_model', ['sysctl', '-n', 'machdep.cpu.brand_string']),
                          ('cpu_physical', ['sysctl', '-n', 'hw.physicalcpu']),
                          ('cpu_logical', ['sysctl', '-n', 'hw.logicalcpu']),
                          ('memory_bytes', ['sysctl', '-n', 'hw.memsize'])):
            done = run(argv)
            md[key] = done.stdout.strip() if done and done.returncode == 0 else None
    else:
        md['cpu_logical'] = str(os.cpu_count())
    env = dict(os.environ, RUSTUP_TOOLCHAIN='1.97.1', RUSTUP_AUTO_INSTALL='0')
    for key, argv in (('rustc', ['rustc', '-Vv']), ('cargo', ['cargo', '-Vv'])):
        done = run(argv, env=env)
        md[key] = done.stdout.strip().splitlines() if done and done.returncode == 0 else None
    if binary:
        with open(binary, 'rb') as fh:
            md['binary_sha256'] = hashlib.sha256(fh.read()).hexdigest()
        md['binary_name'] = os.path.basename(binary)
    return md


# --------------------------------------------------------------- launch


def rlimit_preexec(system, cap_bytes, resource_module=None):
    """Linux: RLIMIT_AS in the child before exec (P1's method). macOS cannot set it."""
    if system != 'Linux':
        return None
    if resource_module is None:
        import resource as resource_module

    def limit():
        resource_module.setrlimit(resource_module.RLIMIT_AS, (cap_bytes, cap_bytes))
    return limit


def find_child(parent_pid, deadline_s=2.0, run=run_quiet):
    """The binary's PID under the /usr/bin/time wrapper (pgrep -P)."""
    end = time.monotonic() + deadline_s
    while time.monotonic() < end:
        done = run(['pgrep', '-P', str(parent_pid)])
        if done is not None and done.returncode == 0 and done.stdout.strip():
            return int(done.stdout.split()[0])
        time.sleep(0.01)
    return None


def ps_rss_kib(pid):
    done = run_quiet(['ps', '-o', 'rss=', '-p', str(pid)])
    if done is None or done.returncode != 0 or not done.stdout.strip():
        return None
    try:
        return int(done.stdout.split()[0])
    except ValueError:
        return None


def survivors_of(pgid, attempts=20):
    """Processes left in the group after a kill (pgrep -g), retried while zombies are reaped."""
    for _ in range(attempts):
        done = run_quiet(['pgrep', '-g', str(pgid)])
        if done is None or done.returncode != 0 or not done.stdout.strip():
            return []
        time.sleep(0.05)
    return [int(x) for x in done.stdout.split()]


def launch(argv, *, rss_cap_bytes, timeout_s, record_dir, run_id, wrapper=True, system=None,
           resource_module=None, poll_s=POLL_S, k6_protocol=True):
    """One observation process: wrapper, watchdog, limits, record. Returns the record."""
    system = system or platform.system()
    os.makedirs(record_dir, exist_ok=True)
    base = os.path.join(record_dir, run_id)
    time_path = base + '.time.txt'
    cmd = list(argv)
    if wrapper:
        cmd = [TIME_WRAPPER, '-l' if system == 'Darwin' else '-v', '-o', time_path] + cmd
    preexec = rlimit_preexec(system, rss_cap_bytes, resource_module)
    record = {'schema': 'k6-runner-record-v1', 'run_id': run_id, 'system': system,
              'argv': [os.path.basename(argv[0])] + [a if not os.path.isabs(a) else os.path.basename(a)
                                                    for a in argv[1:]],
              'wrapper': ('%s %s' % (TIME_WRAPPER, '-l' if system == 'Darwin' else '-v')) if wrapper else None,
              'rss_cap_bytes': rss_cap_bytes, 'timeout_s': timeout_s, 'poll_s': poll_s,
              'rlimit_as_applied': preexec is not None,
              'load_before': list(os.getloadavg()), 'memorystatus_before': memorystatus_level(system)}
    out_path = base + '.jsonl'
    stdout_lines, stderr_chunks = [], []
    t0 = time.monotonic()
    proc = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                            start_new_session=True, preexec_fn=preexec)
    pgid = proc.pid

    def pump_out():
        with open(out_path, 'wb') as fh:
            for line in iter(proc.stdout.readline, b''):
                fh.write(line)
                fh.flush()
                stdout_lines.append(line.decode('utf-8', 'replace'))

    def pump_err():
        for chunk in iter(lambda: proc.stderr.read(1 << 16), b''):
            stderr_chunks.append(chunk)
    threads = [threading.Thread(target=pump_out, daemon=True), threading.Thread(target=pump_err, daemon=True)]
    for t in threads:
        t.start()
    child = find_child(proc.pid) if wrapper else proc.pid
    samples, killed, timed_out, kill_at, polls = [], False, False, None, 0
    while True:
        pid, status, usage = os.wait4(proc.pid, os.WNOHANG)
        if pid:
            break
        now = time.monotonic() - t0
        if child is None and wrapper:
            child = find_child(proc.pid, deadline_s=0.05)
        if child is not None:
            kib = ps_rss_kib(child)
            polls += 1
            if kib is not None:
                samples.append([round(now, 3), kib])
                if system == 'Darwin' and watchdog_exceeds(kib, rss_cap_bytes):
                    os.killpg(pgid, signal.SIGKILL)
                    killed, kill_at = True, round(now, 3)
                    pid, status, usage = os.wait4(proc.pid, 0)
                    break
        if now > timeout_s:
            os.killpg(pgid, signal.SIGKILL)
            timed_out, kill_at = True, round(now, 3)
            pid, status, usage = os.wait4(proc.pid, 0)
            break
        time.sleep(poll_s)
    wall = time.monotonic() - t0
    for t in threads:
        t.join(timeout=5)
    for pipe in (proc.stdout, proc.stderr):
        pipe.close()
    exit_code = os.waitstatus_to_exitcode(status)
    proc.returncode = exit_code
    stderr_text = b''.join(stderr_chunks).decode('utf-8', 'replace')
    with open(base + '.stderr.txt', 'w') as fh:
        fh.write(sanitize(stderr_text))
    time_info = {}
    if os.path.exists(time_path):
        with open(time_path) as fh:
            time_info = parse_time(fh.read(), system)
    objects = jsonl(stdout_lines)
    external = -exit_code if exit_code < 0 and not (killed or timed_out) else None
    classification, detail = classify(killed_by_rss_watchdog=killed, timed_out=timed_out, exit_code=exit_code,
                                      objects=objects, stderr_text=stderr_text, system=system,
                                      rlimit_applied=preexec is not None, k6_protocol=k6_protocol,
                                      external_signal=external)
    survivors = survivors_of(pgid) if (killed or timed_out) else []
    summary = summary_of(objects)
    rss = {'wait4_maxrss_bytes': ru_maxrss_bytes(usage.ru_maxrss, system),
           'ps_max_kib': max((s[1] for s in samples), default=None), 'ps_samples': len(samples)}
    rss.update(time_info)
    peak_rss = rss.get('time_max_rss_bytes')
    if peak_rss is None and rss['ps_max_kib'] is not None:
        peak_rss = ps_kib_to_bytes(rss['ps_max_kib'])
    record.update({
        'classification': classification, 'classification_detail': detail, 'exit_code': exit_code,
        'wall_s': round(wall, 4), 'binary_pid_found': child is not None,
        'killed_by_rss_watchdog': killed, 'timed_out': timed_out, 'kill_after_s': kill_at,
        'watchdog_last_kib': samples[-1][1] if samples else None, 'watchdog_polls': polls,
        'watchdog_samples': samples, 'survivors': survivors, 'rss': rss, 'peak_rss_bytes': peak_rss,
        'peak_rss_source': ('time' if 'time_max_rss_bytes' in rss else ('ps' if peak_rss else None)),
        'load_after': list(os.getloadavg()), 'memorystatus_after': memorystatus_level(system),
        'summary': summary, 'stdout_lines': len(stdout_lines),
        'stderr_tail': sanitize(stderr_text[-1500:]),
    })
    if summary:
        record['repeats_heap_peak'] = summary.get('repeats_heap_peak')
        record['repeats_heap_peak_move'] = summary.get('repeats_heap_peak_move')
    with open(out_path, 'rb') as fh:
        record['stdout_sha256'] = hashlib.sha256(fh.read()).hexdigest()
    with open(base + '.record.json', 'w') as fh:
        json.dump(record, fh, indent=1, sort_keys=True)
        fh.write('\n')
    return record


def binary_argv(binary, run, counts_path, record_dir):
    argv = [binary, '--model', run['model'], '--mode', run['mode'],
            '--heap-cap-bytes', str(run['heap_cap_bytes']), '--repeats', str(run['repeats']),
            '--entry-repeats', str(run['entry_repeats']), '--time-budget-s', str(run['time_budget_s']),
            '--first-repeat-limit-s', str(run['first_repeat_limit_s'])]
    if counts_path:
        argv += ['--counts-file', counts_path]
    if run['mode'] in ('sparse', 'dense') and run['members'] <= LARGE_MEMBERS:
        argv += ['--dump-solution', os.path.join(record_dir, run['run_id'] + '.u')]
    return argv


def read_counts(path):
    counts = {}
    if path and os.path.exists(path):
        with open(path) as fh:
            for o in jsonl(fh):
                if o.get('kind') == 'counts':
                    counts[o['model']] = o
    return counts


def read_records(record_dir):
    path = os.path.join(record_dir, 'records.jsonl')
    if not os.path.exists(path):
        return []
    with open(path) as fh:
        return [json.loads(line) for line in fh if line.strip()]


def append_record(record_dir, record):
    with open(os.path.join(record_dir, 'records.jsonl'), 'a') as fh:
        fh.write(json.dumps(record, sort_keys=True) + '\n')


def baseline_run(binary, record_dir, tag):
    """The slot's process baseline: a no-op run of the same binary (A1-stop ruling Q2)."""
    rec = launch([binary, '--noop', '--heap-cap-bytes', str(C_DEFAULT - HEAP_MARGIN)],
                 rss_cap_bytes=C_DEFAULT, timeout_s=60, record_dir=record_dir, run_id='baseline_' + tag)
    return rec


def observed_class(objects):
    """Repeat 0's outcome class; for a run killed before its first outcome (a timeout),
    'FactorRefused' when its factor stage returned a refusal (the witness then ran),
    otherwise 'Unknown'."""
    outcome = first_of(objects, 'outcome')
    if outcome:
        return outcome.get('class')
    for o in objects:
        if o.get('kind') == 'stage' and o.get('stage') == 'factor' and o.get('repeat') == 0 and not o.get('ok'):
            return 'FactorRefused'
    return 'Unknown'


def cross_mode(run, records, record_dir):
    """Outcome-class parity and the DEC-053 basis for a model whose sparse and dense runs exist."""
    by_mode = {r['mode']: r for r in records if r['model'] == run['model'] and r['mode'] in ('sparse', 'dense')
               and r.get('classification') in ('ok', 'timed_out')}
    if set(by_mode) != {'sparse', 'dense'}:
        return None
    classes = {}
    for mode, r in by_mode.items():
        with open(os.path.join(record_dir, r['run_id'] + '.jsonl')) as fh:
            classes[mode] = observed_class(jsonl(fh))
    known = [c for c in classes.values() if c not in (None, 'Unknown')]
    out = {'model': run['model'], 'sparse_class': classes['sparse'], 'dense_class': classes['dense'],
           'class_equal': (classes['sparse'] == classes['dense']) if len(known) == 2 else None}
    paths = [os.path.join(record_dir, by_mode[m]['run_id'] + '.u') for m in ('sparse', 'dense')]
    if all(os.path.exists(p) for p in paths):
        delta, scale, rel = dec053_delta(read_solution(paths[0]), read_solution(paths[1]))
        both_passed = classes['sparse'] == 'Passed' and classes['dense'] == 'Passed'
        out.update(max_abs_delta=delta, dense_scale=scale, relative=rel, dec053_asserted=both_passed,
                   dec053_within=(rel <= 1e-9) if both_passed else None)
    return out


def stop_reasons(record, admitted):
    """The brief's stops that a single record shows."""
    reasons = []
    if admitted and record['classification'] in ('killed_by_rss_watchdog', 'heap_cap_abort'):
        reasons.append('%s on an admitted run' % record['classification'])
    if record['classification'] == 'error':
        reasons.append('unexpected failure (%s); check the memory guard log' % record.get('classification_detail'))
    if record['classification'] == 'refused_by_binary' and admitted:
        reasons.append('the binary refused a run the runner admitted')
    return reasons


def run_tier(tier, binary, counts_path, record_dir, source_commit, source_tree, log=print):
    counts = read_counts(counts_path)
    os.makedirs(record_dir, exist_ok=True)
    md_path = os.path.join(record_dir, 'metadata.json')
    if not os.path.exists(md_path):
        with open(md_path, 'w') as fh:
            json.dump(metadata(binary, source_commit, source_tree), fh, indent=1, sort_keys=True)
            fh.write('\n')
    base = baseline_run(binary, record_dir, tier)
    baseline_rss = base.get('peak_rss_bytes')
    log('baseline (no-op) peak RSS %s bytes' % baseline_rss)
    stop, stop_after_tier = [], []
    for run in [r for r in schedule() if r['tier'] == tier]:
        records = read_records(record_dir)
        decision = admission(run, counts, records, baseline_rss)
        entry = dict(run, admission=decision, baseline_rss_bytes=baseline_rss)
        if decision['decision'] != 'admitted' or run['conditional']:
            if run['conditional'] and decision['decision'] == 'admitted':
                decision = dict(decision, decision='deferred',
                                reason='deferred:conditional_run_needs_rulings_on_projection (N14)')
                entry['admission'] = decision
            entry.update(classification='not_run', run_id=run['run_id'])
            append_record(record_dir, entry)
            log('%s %s %s: %s' % (run['run_id'], run['model'], run['mode'], decision['reason']))
            continue
        quiet = wait_for_quiet_host(log)
        record = launch(binary_argv(binary, run, counts_path, record_dir), rss_cap_bytes=run['rss_cap_bytes'],
                        timeout_s=run['timeout_s'], record_dir=record_dir, run_id=run['run_id'])
        with open(os.path.join(record_dir, run['run_id'] + '.jsonl')) as fh:
            objects = jsonl(fh)
        entry.update({k: v for k, v in record.items() if k not in ('watchdog_samples',)})
        entry['quiet_host'] = quiet
        entry['estimate_adm_bytes'] = decision['estimate_adm_bytes']
        entry['estimate_f1b_bytes'] = counts.get(run['model'], {}).get(
            'estimate_f1b_bytes_' + run['mode'].replace('-', '_'))
        entry['stages'] = aggregate_stages(objects)
        entry['parity_failures'] = parity_failures(objects)
        append_record(record_dir, entry)
        reasons = stop_reasons(record, True) + ['parity: %s' % p.get('item') for p in entry['parity_failures']]
        cm = cross_mode(run, read_records(record_dir), record_dir)
        if cm:
            append_record(record_dir, dict(cm, kind='cross_mode', run_id='cross_' + run['run_id']))
            if cm['class_equal'] is False:
                stop_after_tier.append('outcome-class divergence on %s (N10: finish the tier, then stop)' % run['model'])
            if cm.get('dec053_asserted') and not cm.get('dec053_within'):
                reasons.append('DEC-053 basis breached on a Passed result: %s' % run['model'])
        log('%s %s %s: %s %.1fs rss=%s' % (run['run_id'], run['model'], run['mode'], record['classification'],
                                          record['wall_s'], record.get('peak_rss_bytes')))
        if reasons:
            stop = reasons
            log('STOP: ' + '; '.join(reasons))
            break
    return {'stop': stop, 'stop_after_tier': stop_after_tier}


# --------------------------------------------------------------- plan, smoke, packet


def plan(counts_path):
    """--plan: the full schedule with each run's estimates, caps and admission. No child process."""
    counts = read_counts(counts_path)
    rows = []
    for run in schedule():
        decision = admission(run, counts, [], None, require_ascent=False)
        line = counts.get(run['model'], {})
        rows.append(dict(run, admission=decision,
                         estimate_f1b_bytes=line.get('estimate_f1b_bytes_' + run['mode'].replace('-', '_')),
                         ascent_previous=previous_size(run['model'])))
    return rows


def format_plan(rows):
    out = ['order tier slot model mode repeats entry_repeats C_GiB heap_cap_bytes timeout_s estimate_adm_MiB '
           'estimate_f1b_MiB decision reason ascent_after']
    for r in rows:
        a = r['admission']
        est = a['estimate_adm_bytes']
        f1b = r['estimate_f1b_bytes']
        out.append('%d %s %s %s %s %d %d %d %d %d %s %s %s %s %s' % (
            r['order'], r['tier'], r['slot'], r['model'], r['mode'], r['repeats'], r['entry_repeats'],
            r['rss_cap_bytes'] // GIB, r['heap_cap_bytes'], r['timeout_s'],
            '%.1f' % (est / MIB) if est else '-', '%.1f' % (f1b / MIB) if f1b else '-',
            a['decision'] + ('(conditional)' if r['conditional'] else ''), '"%s"' % a['reason'],
            r['ascent_previous'] or '-'))
    return '\n'.join(out) + '\n'


GROWING_CHILD = r'''
import sys, time
step, limit, failsafe, delay = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), float(sys.argv[4])
chunks, total = [], 0
while True:
    if total >= failsafe:
        sys.exit(3)
    if limit and total >= limit:
        sys.exit(0)
    block = bytearray(step)
    for i in range(0, step, 4096):
        block[i] = 1
    chunks.append(block)
    total += step
    time.sleep(delay)
'''


def growing_child_argv(step, limit, failsafe, delay):
    """A self-limiting child: grows `step` bytes per `delay` seconds (touching each page),
    exits 0 at `limit` (0: no limit) and exits 3 at `failsafe`."""
    return [sys.executable, '-c', GROWING_CHILD, str(step), str(limit), str(failsafe), str(delay)]


def smoke(binary, record_dir, counts_path, log=print):
    """--smoke (A2): the 10- and 100-member RF-LARGE models and the nine in sparse and dense, the
    watchdog path and the heap-cap abort path, every binary run under a 512 MiB heap cap."""
    os.makedirs(record_dir, exist_ok=True)
    results = []
    base = baseline_run(binary, record_dir, 'smoke')
    results.append(('baseline', base['classification'], base.get('peak_rss_bytes')))
    ids = rf_ids(10) + rf_ids(100) + ['DEC053:' + dec053_fixture_id(s) for s in DEC053]
    for index, model_id in enumerate(ids):
        for mode in rotate(('sparse', 'dense'), index):
            run_id = 'smoke_%s_%s' % (model_id.replace(':', '_'), mode)
            argv = [binary, '--model', model_id, '--mode', mode, '--heap-cap-bytes', str(512 * MIB),
                    '--repeats', '2', '--counts-file', counts_path]
            rec = launch(argv, rss_cap_bytes=GIB, timeout_s=TIMEOUT_SMALL, record_dir=record_dir, run_id=run_id)
            with open(os.path.join(record_dir, run_id + '.jsonl')) as fh:
                failures = parity_failures(jsonl(fh))
            results.append((run_id, rec['classification'], rec.get('peak_rss_bytes'), len(failures)))
            log(results[-1])
    rec = launch(growing_child_argv(MIB, 0, 512 * MIB, 0.01), rss_cap_bytes=128 * MIB, timeout_s=60,
                 record_dir=record_dir, run_id='smoke_watchdog', k6_protocol=False)
    results.append(('smoke_watchdog', rec['classification'], rec['watchdog_last_kib'], rec['survivors']))
    log(results[-1])
    rec = launch([binary, '--model', 'RF-LARGE-CHAIN-n00100-AX', '--mode', 'dense', '--heap-cap-bytes',
                  str(8 * MIB), '--allow-over-estimate', '--repeats', '1'], rss_cap_bytes=GIB,
                 timeout_s=TIMEOUT_SMALL, record_dir=record_dir, run_id='smoke_heap_cap_abort')
    results.append(('smoke_heap_cap_abort', rec['classification'], rec.get('peak_rss_bytes')))
    log(results[-1])
    return results


def packet(record_dir):
    """The compact packet: per (model, mode) results, fits, F1b ratios, parity and metadata."""
    records = read_records(record_dir)
    runs = [r for r in records if r.get('schema') == 'k6-runner-record-v1' and 'order' in r]
    not_run = [{k: r.get(k) for k in ('order', 'tier', 'model', 'mode', 'admission')}
               for r in records if r.get('classification') == 'not_run']
    cross = [r for r in records if r.get('kind') == 'cross_mode']
    md_path = os.path.join(record_dir, 'metadata.json')
    md = None
    if os.path.exists(md_path):
        with open(md_path) as fh:
            md = json.load(fh)
    table, series, ratios = [], {}, []
    for r in runs:
        row = {k: r.get(k) for k in ('order', 'run_id', 'tier', 'slot', 'model', 'mode', 'family', 'members',
                                     'repeats', 'entry_repeats', 'classification', 'wall_s', 'peak_rss_bytes',
                                     'peak_rss_source', 'rss', 'repeats_heap_peak', 'repeats_heap_peak_move',
                                     'estimate_adm_bytes', 'estimate_f1b_bytes', 'baseline_rss_bytes',
                                     'load_before', 'load_after', 'memorystatus_before', 'memorystatus_after',
                                     'stdout_sha256', 'stages', 'parity_failures', 'admission')}
        table.append(row)
        if r.get('classification') != 'ok' or not r.get('repeats_heap_peak_move'):
            continue
        key = '%s/%s' % (r['family'], r['mode'])
        series.setdefault(key, {'heap_move': [], 'rss_net': []})
        series[key]['heap_move'].append((r['members'], r['repeats_heap_peak_move']))
        rss_net = None
        if r.get('peak_rss_bytes') is not None and r.get('baseline_rss_bytes') is not None:
            rss_net = max(1, r['peak_rss_bytes'] - r['baseline_rss_bytes'])
            series[key]['rss_net'].append((r['members'], rss_net))
        f1b = r.get('estimate_f1b_bytes')
        if f1b:
            ratios.append({'model': r['model'], 'mode': r['mode'], 'members': r['members'],
                           'estimate_f1b_bytes': f1b,
                           'heap_peak_over_f1b': r['repeats_heap_peak'] / f1b,
                           'heap_peak_move_over_f1b': r['repeats_heap_peak_move'] / f1b,
                           'rss_net_over_f1b': (rss_net / f1b) if rss_net else None,
                           'rss_over_f1b': (r['peak_rss_bytes'] / f1b) if r.get('peak_rss_bytes') else None})
    fits = {key: {'heap_move': fit_loglog(v['heap_move']), 'rss_net': fit_loglog(v['rss_net'])}
            for key, v in sorted(series.items())}
    return {'schema': 'k6-packet-v1', 'observation_only': True,
            'claims': ['observed log-log growth fits of peak requested heap and of peak RSS against members',
                       "actual-to-estimate ratios for F1b's two estimates at the measured sizes"],
            'no_thresholds': 'no time or memory bound is asserted anywhere',
            'metadata': md, 'runs': table, 'not_run': not_run, 'cross_mode': cross, 'fits': fits,
            'f1b_ratios': ratios}


# The order of each stage's cost in the free DOF count n_f (K6 plan 8.3): the
# projection to 1,000 members scales a 100-member first-repeat stage time by
# (n_f,1000 / n_f,100) to this power. It is a projection, not a claim.
STAGE_ORDER = {'assembly': 1, 'evidence': 1, 'ledger': 1, 'reduce': 1, 'recovery': 1, 'geometry': 2,
               'densify': 2, 'prepare': 2, 'finish': 2, 'factor': 3, 'witness': 4, 'entry_checked': 3,
               'entry_plain': 3, 'densify_lu': 2, 'lane_reduce': 2, 'lane_solve': 3, 'lane_entries': 1}
ENTRY_STAGES = ('entry_checked', 'entry_plain')


def project(record_dir, counts_path):
    """Q7: the projected quiet-host time of T3b (dense and lane-lu at 1,000 members) from the
    100-member runs' first repeats. Known dense timeouts are projected at their timeout."""
    counts = read_counts(counts_path)
    records = read_records(record_dir)
    rows, total = [], 0.0
    for run in [x for x in schedule() if x['tier'] == 'T3b']:
        small = run['model'].replace('-n01000-', '-n00100-')
        done = [x for x in records if x.get('model') == small and x.get('mode') == run['mode']
                and x.get('classification') == 'ok']
        if run['mode'] == 'dense' and run['model'] in KNOWN_DENSE_TIMEOUTS:
            seconds, basis = float(run['timeout_s']), 'known timeout (N10)'
        elif not done or small not in counts or run['model'] not in counts:
            rows.append({'model': run['model'], 'mode': run['mode'], 'projected_s': None,
                         'basis': 'no 100-member record'})
            continue
        else:
            ratio = counts[run['model']]['free_dofs'] / counts[small]['free_dofs']
            with open(os.path.join(record_dir, done[0]['run_id'] + '.jsonl')) as fh:
                stages = [o for o in jsonl(fh) if o.get('kind') == 'stage' and o.get('repeat') == 0]
            first = sum(o['elapsed_ns'] * ratio ** STAGE_ORDER.get(o['stage'], 3) for o in stages) / 1e9
            entries = sum(o['elapsed_ns'] * ratio ** STAGE_ORDER.get(o['stage'], 3) for o in stages
                          if o['stage'] in ENTRY_STAGES) / 1e9
            seconds = first + (run['repeats'] - 1) * (first - entries)
            basis = '100-member first repeat x (n_f ratio %.2f)^order' % ratio
            if first > FIRST_REPEAT_LIMIT:
                seconds, basis = first, basis + '; first repeat over 600 s, so one repeat'
            seconds = min(seconds, float(run['timeout_s']))
        rows.append({'model': run['model'], 'mode': run['mode'], 'projected_s': round(seconds, 1),
                     'basis': basis})
        total += seconds
    return {'rows': rows, 'total_s': round(total, 1), 'two_hour_stop': total > 7200}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('--emit-model')
    parser.add_argument('--model-hashes', action='store_true')
    parser.add_argument('--plan', action='store_true')
    parser.add_argument('--smoke', action='store_true')
    parser.add_argument('--run', action='store_true')
    parser.add_argument('--packet', action='store_true')
    parser.add_argument('--project', action='store_true')
    parser.add_argument('--tier')
    parser.add_argument('--binary')
    parser.add_argument('--counts')
    parser.add_argument('--records')
    parser.add_argument('--source-commit')
    parser.add_argument('--source-tree')
    args = parser.parse_args(argv)
    if args.emit_model:
        sys.stdout.buffer.write(model(args.emit_model).canonical())
        return 0
    if args.model_hashes:
        for id_ in sealed_model_ids():
            print('%s  %s' % (model_sha256(id_), id_))
        for id_ in extra_model_ids():
            print('%s  %s  (extra, outside the sealed set)' % (model_sha256(id_), id_))
        return 0
    if args.plan:
        sys.stdout.write(format_plan(plan(args.counts)))
        return 0
    if args.smoke:
        results = smoke(args.binary, args.records, args.counts)
        print(json.dumps(results))
        return 0
    if args.run:
        outcome = run_tier(args.tier, args.binary, args.counts, args.records, args.source_commit, args.source_tree)
        print(json.dumps(outcome))
        return 1 if outcome['stop'] else 0
    if args.packet:
        print(json.dumps(packet(args.records), indent=1, sort_keys=True))
        return 0
    if args.project:
        print(json.dumps(project(args.records, args.counts), indent=1))
        return 0
    parser.print_help()
    return 2


if __name__ == '__main__':
    sys.exit(main())
