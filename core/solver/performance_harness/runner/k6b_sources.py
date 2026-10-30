#!/usr/bin/env python3
"""K6b: K4SRC bytes of K6b's adapter, written independently in Python (T3 K6b plan Q6(c)).

The models come from the runner's independent Python generator (``k6_runner.model``), not
from the Rust generator. The adapter's rules (``H/src/k6/w1/adapter.rs``, plan section 3.2)
are applied here from their statement, and the bytes follow K4's documented encoding
(K4 RETURN section 13; ``FK/structural/retained/source.rs``, ``PrimitiveSource::encoding``):

    b'K4SRC\\x01', then, little-endian, binary64 values as their bits:
    u32 node count, each node's x, y, z;
    u32 member count, per member (by id): u32 id, u32 i, u32 j, E, G, A, Iy, Iz, J, y_ref x, y, z;
    u32 spring count (0); u32 directional spring count (0);
    u32 constraint count, per constraint (by DOF): u32 node, u8 component, value;
    u32 load count, per load (by DOF, source id, value): u32 node, u8 component,
        u32 source-id length, the source id, value;
    u32 station count, per station (by id): u32 id, u32 member, fraction;
    u32 support group count (0).

``--write <path>`` writes ``observations/k6b/sources.txt``: per sealed model, the K4SRC
length, sha256 and FNV-1a (the Rust tests check the adapter's FNV-1a against it).
``--check <path>`` regenerates and compares. Standard library only; no bytecode is written.
"""
import argparse
import hashlib
import os
import struct
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import k6_runner  # noqa: E402

HEADER = '# k6b-sources v1: K4SRC of K6b\'s adapter per sealed K6 model (runner/k6b_sources.py)\n' \
         '# model length sha256 fnv64\n'
STATION_FRACTION = 0.5


def fnv64(data):
    h = 0xcbf29ce484222325
    for b in data:
        h ^= b
        h = (h * 0x100000001b3) & 0xFFFFFFFFFFFFFFFF
    return h


def k4src(model):
    """The adapter's K4SRC bytes for one ``k6_runner.Model``."""
    out = bytearray(b'K4SRC\x01')

    def u32(v):
        out.extend(struct.pack('<I', v))

    def f64(v):
        out.extend(struct.pack('<d', float(v)))

    def dof(g):
        u32(g // k6_runner.DOF)
        out.append(g % k6_runner.DOF)

    u32(len(model.nodes))
    for _, p in model.nodes:
        for c in p:
            f64(c)
    u32(len(model.members))
    for k, (_, a, b, y) in enumerate(model.members):
        u32(k + 1)
        u32(a)
        u32(b)
        for v in model.section:
            f64(v)
        for c in y:
            f64(c)
    u32(0)  # springs
    u32(0)  # directional springs
    constraints = sorted(node * k6_runner.DOF + d for node, mask in model.restraints.items()
                         for d in range(k6_runner.DOF) if mask[d])
    u32(len(constraints))
    for g in constraints:
        dof(g)
        f64(0.0)
    loads = sorted(model.loads.items())
    u32(len(loads))
    for g, value in loads:
        dof(g)
        sid = ('k6:%d' % g).encode('ascii')
        u32(len(sid))
        out.extend(sid)
        f64(value)
    u32(len(model.members))
    for k in range(len(model.members)):
        u32(k + 1)
        u32(k + 1)
        f64(STATION_FRACTION)
    u32(0)  # support groups
    return bytes(out)


def table():
    lines = [HEADER]
    for model_id in k6_runner.sealed_model_ids():
        data = k4src(k6_runner.model(model_id))
        lines.append('%s %d %s %016x\n' % (model_id, len(data), hashlib.sha256(data).hexdigest(),
                                            fnv64(data)))
    return ''.join(lines)


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    group = ap.add_mutually_exclusive_group(required=True)
    group.add_argument('--write')
    group.add_argument('--check')
    group.add_argument('--emit', help='print one model\'s K4SRC in hex')
    args = ap.parse_args(argv)
    if args.emit:
        print(k4src(k6_runner.model(args.emit)).hex())
        return 0
    text = table()
    if args.write:
        with open(args.write, 'w') as fh:
            fh.write(text)
        return 0
    with open(args.check) as fh:
        ok = fh.read() == text
    print('sources.txt %s' % ('matches' if ok else 'DIFFERS'))
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
