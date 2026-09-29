#!/usr/bin/env python3
"""V-K (T3 slice I17): R1's frozen references as VP-ROBUST kernel-lane case files.

Standard library only.  It reads, read-only and hash-checked, R1's `references.json`
and `references.py` and D1's `floor_kinds.json`, and writes the files next to it:

  rf_<family>.jsonl        one JSON object per R1 case (RF-CANCEL's three UDL cases,
                           which are W1b's, are excluded)
  not_covered.json         the enumerated not-covered list (DESIGN.md §4.10)
  expected_unresolved.json the cases W1a is expected to leave unresolved (ROOT's
                           ruling on I17's A1 stop), with their reason
  absolute_range_rows.json the rows whose expected value lies outside binary64's range
  engine_vectors.jsonl     exact verdicts of the predicate on sampled rows (Fractions)
  large_models.sha256      the RF-LARGE models at 1,000 and 10,000 members (not committed)
  SHA256SUMS               the sha256 of every file above

Usage:
  python3 gen_vk_cases.py                  write the files
  python3 gen_vk_cases.py --check          regenerate in memory; compare bytes; write nothing
  python3 gen_vk_cases.py --large DIR      also write the 12 large models into DIR
  python3 gen_vk_cases.py --compare-k4 OUT the one-off comparison with K4's committed adapter
                                           output (K4T/r1_cases.txt, r1_large.txt), into OUT

The adapter (path A) and the canonical-bytes check (path B) are separate code:
- path A turns each case's R1 model into kernel inputs, each exact input rounded once to
  binary64 (the rules are in `adapt`);
- path B reads R1's model as `references.py --model <id>` prints it (`model_json(defn,
  full=True)`, called in process) and builds K4's canonical source bytes (`K4SRC`,
  `K4R/source.rs` `encoding`) directly, applying the same stated rules independently.
  The Rust test requires sha256(PrimitiveSource::encoding()) of path A's model to equal
  path B's digest.

Every number taken from R1 (expected values, scales, control values) is R1's decimal
string, byte for byte.  Nothing is converted to binary64 for a decision.
"""
import sys

sys.dont_write_bytecode = True

import argparse
import hashlib
import importlib.util
import json
import math
import os
import struct
from decimal import Decimal, localcontext
from fractions import Fraction as Fr
from pathlib import Path

HERE = Path(__file__).resolve().parent
VR = HERE.parent
P_ROOT = VR.parents[2]
T3 = P_ROOT / ('execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/'
               'instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3')
R1_JSON = T3 / 'REFERENCES/references.json'
R1_PY = T3 / 'REFERENCES/references.py'
FLOOR_JSON = T3 / 'DESIGN_NUMERICS/_run_records/floor_kinds.json'
K4T = P_ROOT / 'core/solver/frame_kernel/tests/retained_k4'

PINNED = {
    R1_JSON: '7b176dbbf2296be02d8bca19c698ee56d0d5753751150a9175e0d3ad4cf89cc9',
    R1_PY: '80d473a7351e92a2a903d233b9e633aac794aeff07f3ac41e40d25a0d94fc8a8',
    FLOOR_JSON: '8326561530598e1f6b69d9f174b70946e52800373b5ecab48864ae174087ae78',
}

FAMILIES = ('RF-CHAIN', 'RF-SKEW', 'RF-WEAK', 'RF-LARGE', 'RF-INVARIANCE', 'RF-RANGE', 'RF-ZERO',
            'RF-FINITE', 'RF-MECH', 'RF-CANCEL')
EXCLUDED = ('RF-CANCEL-UDL-W1e5', 'RF-CANCEL-UDL-W1e8', 'RF-CANCEL-UDL-W1e80')
DOF = {'UX': 0, 'UY': 1, 'UZ': 2, 'RX': 3, 'RY': 4, 'RZ': 5}
KINDS = ('translation', 'rotation', 'force', 'moment')
R_FLOOR = Fr(1, 2 ** 34)
# ROOT, "V-K: rulings on I17's A1 stop (THIN-A and THIN-B)": W1a's coverage
# limit, confirmed by K4's generator (GEN's `schedule_em`).
EXPECTED_UNRESOLVED = ('RF-RANGE-THIN-A', 'RF-RANGE-THIN-B')
UNRESOLVED_REASON = ('W1a\'s coverage limit: the axial-to-bending stiffness ratio EA/(12EI/L^3) of the '
                     'PHYS-R4 geometry (OD 4e-77 m, L = 1 m) is about 2^507, beyond the reach of W1a\'s '
                     '512-bit candidate ceiling. 128 and 256 are rejected by the stop rule, 512 by the '
                     'charge test (d); 1024 verifies and nothing lies above it: Unresolved(Ceiling), '
                     'honestly, with no rows. K4\'s generator gives the same schedule.')
TOLERANCE = Fr(1, 10 ** 9)
MIN_NORMAL = 2.0 ** -1022


def sha256_bytes(data):
    return hashlib.sha256(data).hexdigest()


def sha256_file(path):
    return sha256_bytes(Path(path).read_bytes())


def check_pins():
    for path, digest in PINNED.items():
        got = sha256_file(path)
        if got != digest:
            raise SystemExit('pinned input changed: %s (%s)' % (path.name, got))


def load_r1_module():
    spec = importlib.util.spec_from_file_location('references', R1_PY)
    module = importlib.util.module_from_spec(spec)
    sys.modules['references'] = module
    spec.loader.exec_module(module)
    return module


def dumps(obj):
    return json.dumps(obj, sort_keys=True, separators=(',', ':'), ensure_ascii=True)


# ----------------------------------------------------------------------------- numbers

def exact(text):
    """R1's exact input spelling: a decimal (plain or scientific), p/q, or d*2^k."""
    text = str(text)
    if '*2^' in text:
        a, k = text.split('*2^')
        return exact(a) * Fr(2) ** int(k)
    if '/' in text:
        p, q = text.split('/')
        return Fr(int(p), int(q))
    return Fr(Decimal(text))


def fl(q):
    """q rounded once to binary64 (int true division is correctly rounded in CPython)."""
    q = Fr(q)
    return q.numerator / q.denominator


def bits(x):
    return struct.pack('>d', x).hex()


def unbits(h):
    return struct.unpack('>d', bytes.fromhex(h))[0]


def is_normal(x):
    return x != 0.0 and math.isfinite(x) and abs(x) >= MIN_NORMAL


# ----------------------------------------------------------------------------- path A: the adapter

def section_values(sec, basis, pi_q):
    """(E, G, A, Iy, Iz, J) in binary64.  On the intended basis every input is exact; on
    the represented basis each authored input is its binary64 value first (R1's `rep`).
    A = pi(OD^2 - ID^2)/4, I = pi(OD^4 - ID^4)/64, J = 2I, pi = R1's PI_Q, each one exact
    rational rounded once."""
    dec = (lambda q: q) if basis == 'intended' else (lambda q: Fr(fl(q)))
    E, OD, ID = dec(exact(sec['E'])), dec(exact(sec['OD'])), dec(exact(sec['ID']))
    G = dec(exact(sec['G'])) if 'G' in sec else E / (2 * (1 + dec(exact(sec['nu']))))
    A = pi_q * (OD * OD - ID * ID) / 4
    I = pi_q * (OD ** 4 - ID ** 4) / 64
    return fl(E), fl(G), fl(A), fl(I), fl(I), fl(2 * I)


def y_reference(xi, xj):
    """The global unit axis on which the exact chord has the smallest |component|, the
    lowest index on ties (V-K's rule; K4's differs, plan §4.4)."""
    d = [abs(Fr(b) - Fr(a)) for a, b in zip(xi, xj)]
    k = min(range(3), key=lambda c: (d[c], c))
    y = [0.0, 0.0, 0.0]
    y[k] = 1.0
    return y


def adapt(model, basis, pi_q):
    """Path A.  R1's model (a references.json `model` object, or `model_json(full=True)`)
    to V-K's kernel model: node order and member order are R1's; members are numbered 1..
    in R1's order; springs 1.. in R1's support order (one id space); a spring with one
    nonzero direction component is a global-axis spring on that DOF, any other a
    directional spring; a spring with k = 0 is omitted (recorded); rigid DOFs are
    constraints at +0; one load per nonzero component (source l<k>), RF-CANCEL's
    contributions one each (source c<k>, authored order); one station per member at 0.5."""
    names = list(model['nodes_m'])
    index = {n: i for i, n in enumerate(names)}
    coords = [[fl(exact(c)) for c in model['nodes_m'][n]] for n in names]
    sections = {sid: section_values(s, basis, pi_q) for sid, s in model['sections'].items()}
    members = []
    for k, (name, a, b, sid) in enumerate(model['members']):
        i, j = index[a], index[b]
        e, g, area, iy, iz, jt = sections[sid]
        y = y_reference(coords[i], coords[j])
        members.append([name, k + 1, i, j] + [bits(v) for v in (e, g, area, iy, iz, jt)] + [bits(c) for c in y])
    springs, omitted, constraints = [], [], []
    sid = 0
    for node, sup in model.get('supports', {}).items():
        for dof in sup.get('rigid', []):
            constraints.append([index[node], DOF[dof]])
        for pos, s in enumerate(sup.get('springs', [])):
            sid += 1
            key = 'S.%s.%d' % (node, pos)
            kind = 't' if s['kind'] == 'translation' else 'r'
            direction = [exact(c) for c in s['direction']]
            k = exact(s['k'])
            if k == 0:
                omitted.append(key)
                continue
            nonzero = [c for c in range(3) if direction[c] != 0]
            axis = (nonzero[0] + (0 if kind == 't' else 3)) if len(nonzero) == 1 else None
            springs.append([key, sid, index[node], kind, axis, [bits(fl(c)) for c in direction], bits(fl(k))])
    loads = []
    count = 0
    for node, ld in model.get('loads', {}).items():
        for part, off in (('F', 0), ('M', 3)):
            for c, v in enumerate(ld.get(part, ['0', '0', '0'])):
                q = exact(v)
                if q != 0:
                    loads.append([index[node], off + c, bits(fl(q)), 'l%d' % count])
                    count += 1
    for k, (node, dof, v) in enumerate(model.get('load_contributions_in_authored_order', [])):
        loads.append([index[node], DOF[dof], bits(fl(exact(v))), 'c%d' % k])
    stations = [[m[1], m[1], bits(0.5)] for m in members]
    return {'nodes': [[n] + [bits(c) for c in xyz] for n, xyz in zip(names, coords)],
            'members': members, 'springs': springs, 'omitted_springs': omitted,
            'constraints': constraints, 'loads': loads, 'stations': stations}


# ----------------------------------------------------------------------------- path B: K4SRC from --model

def k4src_from_model_json(mj, basis, pi_q):
    """Path B.  K4's canonical source bytes (K4R/source.rs `encoding`), built directly
    from `references.py --model`'s JSON: little-endian, binary64 as bits, lists in K4's
    canonical order (members, springs, directional springs, stations and groups by id;
    constraints by DOF; loads by (DOF, source id, value bits))."""
    u32 = lambda v: struct.pack('<I', v)
    f64 = lambda x: struct.pack('<d', x)
    dofb = lambda node, comp: u32(node) + bytes([comp])
    order = list(mj['nodes_m'])
    where = {name: pos for pos, name in enumerate(order)}
    out = bytearray(b'K4SRC\x01')
    out += u32(len(order))
    xyz = []
    for name in order:
        p = [fl(exact(c)) for c in mj['nodes_m'][name]]
        xyz.append(p)
        for c in p:
            out += f64(c)
    out += u32(len(mj['members']))
    for mid, (_name, a, b, sec_id) in enumerate(mj['members'], start=1):
        sec = mj['sections'][sec_id]
        if basis == 'intended':
            E, OD, ID = exact(sec['E']), exact(sec['OD']), exact(sec['ID'])
            G = exact(sec['G']) if 'G' in sec else E / (2 * (1 + exact(sec['nu'])))
        else:
            r = lambda t: Fr(fl(exact(t)))
            E, OD, ID = r(sec['E']), r(sec['OD']), r(sec['ID'])
            G = r(sec['G']) if 'G' in sec else E / (2 * (1 + r(sec['nu'])))
        i4 = pi_q * (OD ** 4 - ID ** 4) / 64
        out += u32(mid) + u32(where[a]) + u32(where[b])
        for v in (E, G, pi_q * (OD ** 2 - ID ** 2) / 4, i4, i4, 2 * i4):
            out += f64(fl(v))
        chord = [abs(Fr(xyz[where[b]][c]) - Fr(xyz[where[a]][c])) for c in range(3)]
        low = sorted(range(3), key=lambda c: (chord[c], c))[0]
        for c in range(3):
            out += f64(1.0 if c == low else 0.0)
    plain, directional, rigid = [], [], []
    ident = 0
    for node, sup in mj.get('supports', {}).items():
        for d in sup.get('rigid', []):
            rigid.append((where[node], DOF[d]))
        for s in sup.get('springs', []):
            ident += 1
            k = fl(exact(s['k']))
            if k == 0.0:
                continue
            d = [fl(exact(c)) for c in s['direction']]
            base = 0 if s['kind'] == 'translation' else 3
            nz = [c for c in range(3) if d[c] != 0.0]
            if len(nz) == 1:
                plain.append((ident, where[node], base + nz[0], k))
            else:
                directional.append((ident, where[node], 0 if base == 0 else 1, d, k))
    out += u32(len(plain))
    for ident, node, comp, k in sorted(plain):
        out += u32(ident) + dofb(node, comp) + f64(k)
    out += u32(len(directional))
    for ident, node, kind, d, k in sorted(directional, key=lambda t: t[0]):
        out += u32(ident) + u32(node) + bytes([kind])
        for c in d:
            out += f64(c)
        out += f64(k)
    out += u32(len(rigid))
    for node, comp in sorted(rigid):
        out += dofb(node, comp) + f64(0.0)
    loads = []
    n = 0
    for node, ld in mj.get('loads', {}).items():
        for part, off in (('F', 0), ('M', 3)):
            for c, t in enumerate(ld.get(part, ['0', '0', '0'])):
                q = exact(t)
                if q != 0:
                    loads.append(((where[node], off + c), ('l%d' % n).encode(), fl(q)))
                    n += 1
    for k, (node, d, t) in enumerate(mj.get('load_contributions_in_authored_order', [])):
        loads.append(((where[node], DOF[d]), ('c%d' % k).encode(), fl(exact(t))))
    loads.sort(key=lambda l: (l[0], l[1], struct.unpack('<Q', struct.pack('<d', l[2]))[0]))
    out += u32(len(loads))
    for (node, comp), src, v in loads:
        out += dofb(node, comp) + u32(len(src)) + src + f64(v)
    out += u32(len(mj['members']))
    for mid in range(1, len(mj['members']) + 1):
        out += u32(mid) + u32(mid) + f64(0.5)
    out += u32(0)
    return bytes(out)


# ----------------------------------------------------------------------------- the floor check (generator's own)

def s_star(rows, adapted):
    """S* per kind and per member (variant F) from the reference values, D1 §4.1.6.1 items
    4-7 in binary64 (Python floats are IEEE binary64, math.sqrt correctly rounded)."""
    s = {k: 0.0 for k in KINDS}
    for key, value, cls, _scale in rows:
        base = cls.split('@')[0]
        if base in s:
            s[base] = max(s[base], abs(fl(Fr(Decimal(value)))))
    xyz = [[unbits(h) for h in n[1:]] for n in adapted['nodes']]
    d = [max(p[a] for p in xyz) - min(p[a] for p in xyz) for a in range(3)]
    lb = math.sqrt((d[0] * d[0] + d[1] * d[1]) + d[2] * d[2])
    tr, ro, fo, mo = s['translation'], s['rotation'], s['force'], s['moment']
    if lb > 0.0:
        tr, ro, fo, mo = (max(tr, lb * s['rotation']), max(ro, s['translation'] / lb),
                          max(fo, s['moment'] / lb), max(mo, lb * s['force']))
    star = {'translation': tr, 'rotation': ro, 'force': fo, 'moment': mo}
    per_member = {}
    for m in adapted['members']:
        name, i, j = m[0], m[2], m[3]
        e, g, area, _iy, _iz, jt = (unbits(h) for h in m[4:10])
        length = member_length(xyz[i], xyz[j])
        per_member[name] = (mo / coefficient(g, jt, length), fo / coefficient(e, area, length))
    return star, per_member


def member_length(xi, xj):
    """FK's `norm` of FK's `subtract` (FK/lib.rs: dot(v, v).sqrt(), left to right)."""
    v = [xj[k] - xi[k] for k in range(3)]
    return math.sqrt((v[0] * v[0] + v[1] * v[1]) + v[2] * v[2])


def coefficient(a, b, length):
    """k = fl(fl(a*b)/L) (DESIGN.md §4.10), with ROOT's C1 ruling: where fl(a*b) or the
    quotient is not a normal number, the same two operations on operands pre-scaled by an
    exact power of two, the result unscaled exactly.  Identical bits where the design's
    formula is defined (normal intermediates)."""
    p = a * b
    if is_normal(p) and is_normal(p / length):
        return p / length
    # s puts a*b*2^s in [1, 4), as the Rust side (floor.rs) does; any exact
    # power of two with normal intermediates gives the same bits.
    s = -((math.frexp(a)[1] - 1) + (math.frexp(b)[1] - 1))
    q = (a * math.ldexp(b, s)) / length
    return math.ldexp(q, -s)


def not_covered(rows, adapted, scales):
    """The keys whose comparison scale max(|exp|, scale) is below R*S*, R = 2^-34 (exact)."""
    star, per_member = s_star(rows, adapted)
    out = []
    for key, value, cls, scale in rows:
        base = cls.split('@')[0]
        if base in star:
            sk = star[base]
        else:
            tw, ext = per_member[key.split('.', 1)[1]]
            sk = tw if base == 'twist' else ext
        scale = scale if scale is not None else scales[cls]
        comparison = max(abs(Fr(Decimal(value))), Fr(Decimal(scale)))
        if comparison < R_FLOOR * Fr(sk):
            out.append(key)
    return out


# ----------------------------------------------------------------------------- cases

def case_rows(case):
    rows_src = case.get('expected_represented') if case.get('basis') == 'represented' else case.get('expected', [])
    rows = []
    for r in rows_src or []:
        if case['family'] == 'RF-CANCEL':
            rows.append([r[0], r[1], r[2], r[3]])
        else:
            rows.append([r[0], r[1], r[2], None])
    return rows


def case_controls(case):
    out = []
    for nc in case['negative_controls']:
        if 'values' in nc:
            out.append([nc['id'], bool(nc['discriminates']), 'value', nc['values']])
        else:
            out.append([nc['id'], bool(nc['discriminates']), 'outcome',
                        {'defect': nc.get('defective_outcome', nc['description'])}])
    return out


def is_large(cid):
    return cid.startswith('RF-LARGE-') and ('-n01000-' in cid or '-n10000-' in cid)


def build(r1, ref, floor, want_large):
    """Every generated file's bytes, and the large models."""
    pi_q = r1.PI_Q
    for name in r1.BUILD_ORDER:
        name()
    by_id = {c['id']: c for c in r1.CASES}
    files = {}
    per_family = {f: [] for f in FAMILIES}
    listed, recomputed, large_models, large_lines = [], [], {}, []
    absolute_rows = []
    for cid, case in ref['cases'].items():
        if cid in EXCLUDED:
            continue
        fam = case['family']
        basis = case.get('basis', 'intended')
        rows = case_rows(case)
        mj = json.loads(json.dumps(r1.model_json(by_id[cid]['defn'], full=True)))
        abbreviated = 'generator' in case['model']
        if not abbreviated and case['model'] != mj:
            raise SystemExit('%s: references.json model differs from --model' % cid)
        adapted = adapt(mj if abbreviated else case['model'], basis, pi_q)
        src = k4src_from_model_json(mj, basis, pi_q)
        line = {
            'id': cid, 'family': fam, 'basis': basis,
            'units': (case.get('units') or {}).get('length', 'm'),
            'needs_directional_spring': bool(case['flags'].get('needs_directional_spring')),
            'refuse': 'expected_outcome' in case,
            'k4src_sha256': sha256_bytes(src),
            'scales': {k: v['value'] for k, v in case.get('scales', {}).items()},
            'rows': rows, 'controls': case_controls(case),
        }
        if rows and (basis == 'represented') != bool(case['finite_input'].get('exceeds_1e-9')):
            raise SystemExit('%s: basis and finite_input disagree' % cid)
        fk = floor['cases'].get(cid, {})
        keys = [r[0] for r in fk.get('F_rec_scale_below' if fam == 'RF-CANCEL' else 'F_scale_below', [])]
        line['not_covered'] = keys
        for k in keys:
            listed.append([fam, cid, k])
        if rows:
            mine = not_covered(rows, adapted, line['scales'])
            if mine != keys:
                raise SystemExit('%s: floor check %s differs from floor_kinds.json %s' % (cid, mine, keys))
            recomputed.extend(mine)
        for r in rows:
            v = Fr(Decimal(r[1]))
            if v != 0 and (abs(v) < Fr(1, 2 ** 1075) or abs(v) >= Fr(2 ** 1024)):
                absolute_rows.append({'case': cid, 'key': r[0], 'expected': r[1], 'class': r[2],
                                      'scale': case['scales'][r[2]]['value']})
        if is_large(cid):
            text = dumps(adapted) + '\n'
            line['model'] = None
            line['model_sha256'] = sha256_bytes(text.encode())
            full_rows = [[k, v, c, None] for (k, v, c) in full_rows_of(r1, by_id[cid])]
            line['s_full'] = s_full(full_rows)
            line['not_covered_full_solution'] = not_covered(full_rows, adapted, line['scales'])
            if line['not_covered_full_solution']:
                raise SystemExit('%s: not covered on the full solution: %s' % (cid, line['not_covered_full_solution']))
            large_lines.append('%s  %s  %s\n' % (line['model_sha256'], line['k4src_sha256'], cid))
            if want_large:
                large_models[cid] = text
        else:
            line['model'] = adapted
        per_family[fam].append(dumps(line) + '\n')
    counts = {}
    for k in listed:
        counts[k[0]] = counts.get(k[0], 0) + 1
    if counts != {'RF-WEAK': 46, 'RF-CANCEL': 3, 'RF-SKEW': 2}:
        raise SystemExit('not-covered counts %s differ from DESIGN.md §4.10' % counts)
    for fam in FAMILIES:
        files['%s.jsonl' % fam.lower().replace('-', '_')] = ''.join(per_family[fam]).encode()
    files['not_covered.json'] = (json.dumps({'source': 'floor_kinds.json variant F (F_rec for RF-CANCEL), '
                                             'recomputed by gen_vk_cases.py (R = 2^-34)',
                                             'counts': counts, 'entries': listed}, indent=1, sort_keys=True)
                                 + '\n').encode()
    files['absolute_range_rows.json'] = (json.dumps(absolute_rows, indent=1, sort_keys=True) + '\n').encode()
    files['expected_unresolved.json'] = (json.dumps(expected_unresolved(ref, pi_q), indent=1, sort_keys=True)
                                         + '\n').encode()
    files['engine_vectors.jsonl'] = engine_vectors(ref, absolute_rows).encode()
    files['large_models.sha256'] = ''.join(large_lines).encode()
    files['SHA256SUMS'] = ''.join('%s  %s\n' % (sha256_bytes(files[n]), n) for n in sorted(files)).encode()
    return files, large_models


def expected_unresolved(ref, pi_q):
    """The expected-unresolved list, with each case's exact stiffness ratio."""
    out = []
    for cid in EXPECTED_UNRESOLVED:
        case = ref['cases'][cid]
        m = adapt(case['model'], case.get('basis', 'intended'), pi_q)
        ratios = []
        for mem in m['members']:
            xi = [Fr(unbits(h)) for h in m['nodes'][mem[2]][1:]]
            xj = [Fr(unbits(h)) for h in m['nodes'][mem[3]][1:]]
            l2 = sum((b - a) ** 2 for a, b in zip(xi, xj))
            area, iy = Fr(unbits(mem[6])), Fr(unbits(mem[7]))
            ratios.append(area * l2 / (12 * iy))
        worst = max(ratios)
        out.append({'case': cid, 'family': case['family'], 'reason': UNRESOLVED_REASON,
                    'ea_over_12ei_l3': '%.6e' % float(worst),
                    'log2_ea_over_12ei_l3': '%.3f' % math.log2(float(worst))})
    return {'source': 'ROOT_RULINGS_V1.md, "V-K: rulings on I17\'s A1 stop (THIN-A and THIN-B)"',
            'rule': 'a listed case must end Unresolved with no rows; its comparisons are counted '
                    'separately, never as passes; a case leaving or joining the list fails the gate',
            'entries': out}


def full_rows_of(r1, case):
    sol = case['sol'] if 'sol' in case else r1.solve(case['defn'], case['method'])
    return [(k, r1.fmt(v), c) for (k, v, c) in r1.flatten(sol, case['defn'], regions=case.get('regions'))]


def s_full(full_rows):
    """S(kind) of the complete reference solution (R1's 40-digit strings of the largest
    magnitude of each class), for the floor check at n >= 1,000 (plan §7.2)."""
    best = {}
    for key, value, cls, _ in full_rows:
        v = abs(Fr(Decimal(value)))
        if cls not in best or v > best[cls][0]:
            best[cls] = (v, value.lstrip('-'))
    return {cls: text for cls, (_, text) in best.items()}


# ----------------------------------------------------------------------------- engine vectors

def verdict(obs, exp, scale):
    return abs(obs - exp) <= TOLERANCE * max(abs(exp), scale)


def magnitude_verdict(my, mz, exp, scale):
    """|sqrt(my^2 + mz^2) - exp| <= 1e-9 max(|exp|, scale), decided by 400-digit Decimal
    arithmetic (a method independent of the Rust engine's exact squares); a vector whose
    margin is below 1e-300 relative is not emitted."""
    with localcontext() as ctx:
        ctx.prec = 400
        ctx.Emin = -999999
        ctx.Emax = 999999
        s = Decimal(my) ** 2 + Decimal(mz) ** 2
        mag = s.sqrt()
        t = Decimal(TOLERANCE.numerator) / Decimal(TOLERANCE.denominator) * max(abs(Decimal(exp)), Decimal(scale))
        margin = t - abs(mag - Decimal(exp))
        if t != 0 and abs(margin) < t * Decimal('1e-300'):
            return None
        return margin >= 0


def engine_vectors(ref, absolute_rows):
    """Sampled rows (every 101st row of the lane, in order) with observations at and
    around the threshold, each verdict decided in Fractions (plain rows) or 400-digit
    Decimal (magnitude rows).  Keys: k kind (p plain, m magnitude), o obs bits (y, z the
    magnitude's two components), e expected, s scale, v verdict, r the source row."""
    out = []
    n = 0
    for cid, case in ref['cases'].items():
        if cid in EXCLUDED or case.get('expected_outcome'):
            continue
        for key, value, cls, row_scale in case_rows(case):
            n += 1
            if n % 101:
                continue
            exp = Fr(Decimal(value))
            scale = Fr(Decimal(row_scale if row_scale is not None else case['scales'][cls]['value']))
            if exp != 0 and abs(exp) < Fr(1, 2 ** 1075):
                continue
            t = TOLERANCE * max(abs(exp), scale)
            obs = [fl(exp), 0.0, -fl(exp)]
            for b in (fl(exp + t), fl(exp - t)):
                obs += [b, math.nextafter(b, math.inf), math.nextafter(b, -math.inf)]
            src = '%s:%s' % (cid, key)
            if key.startswith('Mb.'):
                for b in obs:
                    if b < 0 or not math.isfinite(b):
                        continue
                    my, mz = fl(Fr(b) * Fr(3, 5)), fl(Fr(b) * Fr(4, 5))
                    v = magnitude_verdict(my, mz, value, row_scale if row_scale is not None
                                          else case['scales'][cls]['value'])
                    if v is None:
                        continue
                    out.append({'k': 'm', 'y': bits(my), 'z': bits(mz), 'e': value,
                                's': row_scale if row_scale is not None else case['scales'][cls]['value'],
                                'v': v, 'r': src})
            else:
                for b in obs:
                    if not math.isfinite(b):
                        continue
                    out.append({'k': 'p', 'o': bits(b), 'e': value,
                                's': row_scale if row_scale is not None else case['scales'][cls]['value'],
                                'v': verdict(Fr(b), exp, scale), 'r': src})
    for r in absolute_rows:
        exp, scale = Fr(Decimal(r['expected'])), Fr(Decimal(r['scale']))
        t = TOLERANCE * scale
        for b in (0.0, -0.0, 5e-324, -5e-324, fl(t), math.nextafter(fl(t), math.inf), -fl(t), 1.0):
            out.append({'k': 'p', 'o': bits(b), 'e': r['expected'], 's': r['scale'],
                        'v': verdict(Fr(b), exp, scale), 'r': '%s:%s' % (r['case'], r['key'])})
    return ''.join(dumps(v) + '\n' for v in out)


# ----------------------------------------------------------------------------- the K4 comparison

def parse_k4(text):
    """K4T's `model ... end` blocks: numeric inputs only."""
    models, cur = {}, None
    for line in text.splitlines():
        f = line.split()
        if not f:
            continue
        if f[0] == 'model':
            cur = {'nodes': [], 'members': [], 'springs': [], 'dsprings': [], 'constraints': [], 'loads': [],
                   'stations': []}
            models[f[1]] = cur
        elif f[0] == 'end':
            cur = None
        elif cur is not None and f[0] in ('node', 'member', 'spring', 'dspring', 'constraint', 'load', 'station'):
            cur[{'node': 'nodes', 'member': 'members', 'spring': 'springs', 'dspring': 'dsprings',
                 'constraint': 'constraints', 'load': 'loads', 'station': 'stations'}[f[0]]].append(f[1:])
    return models


def compare_k4(files, out_dir):
    """One-off: V-K's adapted inputs against K4's committed adapter output, every numeric
    input bit for bit.  Differences are classified against plan §4.4; any other is listed
    as UNEXPECTED."""
    k4 = {}
    for name in ('r1_cases.txt', 'r1_large.txt'):
        k4.update(parse_k4((K4T / name).read_text()))
    report, unexpected = [], 0
    for fname, data in sorted(files.items()):
        if not fname.endswith('.jsonl') or fname.startswith('engine'):
            continue
        for text in data.decode().splitlines():
            case = json.loads(text)
            cid, m = case['id'], case['model']
            if cid not in k4 or m is None:
                continue
            km = k4[cid]
            diffs = []
            if [n[1:] for n in m['nodes']] != km['nodes']:
                diffs.append('UNEXPECTED nodes')
            mine = [[str(x[2]), str(x[3])] + x[4:10] for x in m['members']]
            theirs = [x[1:3] + x[3:9] for x in km['members']]
            if mine != theirs:
                diffs.append('UNEXPECTED member properties or ends')
            if [x[10:13] for x in m['members']] != [x[9:12] for x in km['members']]:
                diffs.append('y_reference (plan §4.4)')
            vk_loads = sorted((str(l[0]), str(l[1]), l[2]) for l in m['loads'])
            k4_loads = sorted((l[0], l[1], l[2]) for l in km['loads'])
            if vk_loads != k4_loads:
                diffs.append('UNEXPECTED loads')
            if sorted((str(c[0]), str(c[1])) for c in m['constraints']) != sorted((c[0], c[1]) for c in km['constraints']):
                diffs.append('UNEXPECTED constraints')
            if any(c[2] != '0000000000000000' for c in km['constraints']):
                diffs.append('UNEXPECTED nonzero constraint value in K4')
            zero = '0000000000000000'
            k4_plain = [s for s in km['springs'] if not (m['omitted_springs'] and s[3] == zero)]
            vk_springs = sorted(s[6] for s in m['springs'])
            k4_springs = sorted([s[3] for s in k4_plain] + [s[6] for s in km['dsprings']])
            if vk_springs != k4_springs:
                diffs.append('UNEXPECTED spring stiffnesses')
            vk_axis = sum(1 for s in m['springs'] if s[4] is not None)
            if vk_axis != len(k4_plain):
                diffs.append('spring representation at mixed nodes (plan §4.4): V-K %d global-axis, K4 %d'
                             % (vk_axis, len(km['springs'])))
            if sorted(s[2] for s in m['stations']) != sorted(s[2] for s in km['stations']):
                diffs.append('UNEXPECTED station fractions')
            if m['omitted_springs']:
                diffs.append('k = 0 spring omitted by V-K, kept by K4 (plan §4.1, C10)')
            unexpected += sum(1 for d in diffs if d.startswith('UNEXPECTED'))
            report.append({'case': cid, 'differences': diffs})
    os.makedirs(out_dir, exist_ok=True)
    with open(os.path.join(out_dir, 'compare_k4.json'), 'w') as fh:
        json.dump({'cases_compared': len(report), 'unexpected': unexpected, 'report': report}, fh, indent=1)
        fh.write('\n')
    print('compared %d cases with K4T; unexpected differences: %d' % (len(report), unexpected))
    return unexpected


# ----------------------------------------------------------------------------- main

def main(argv):
    ap = argparse.ArgumentParser()
    ap.add_argument('--check', action='store_true')
    ap.add_argument('--large')
    ap.add_argument('--compare-k4')
    args = ap.parse_args(argv)
    check_pins()
    ref = json.loads(R1_JSON.read_text())
    floor = json.loads(FLOOR_JSON.read_text())
    r1 = load_r1_module()
    files, large = build(r1, ref, floor, want_large=bool(args.large))
    if args.check:
        bad = [n for n, data in sorted(files.items()) if not (HERE / n).exists() or (HERE / n).read_bytes() != data]
        extra = sorted(p.name for p in HERE.iterdir()
                       if p.is_file() and p.name not in files and p.name != Path(__file__).name)
        for n in sorted(files):
            print('%s %s' % ('DIFFERS' if n in bad else 'OK', n))
        for n in extra:
            print('UNEXPECTED FILE %s' % n)
        return 1 if bad or extra else 0
    for n, data in files.items():
        (HERE / n).write_bytes(data)
    if args.large:
        os.makedirs(args.large, exist_ok=True)
        for cid, text in large.items():
            with open(os.path.join(args.large, cid + '.json'), 'w') as fh:
                fh.write(text)
    if args.compare_k4:
        return 1 if compare_k4(files, args.compare_k4) else 0
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
