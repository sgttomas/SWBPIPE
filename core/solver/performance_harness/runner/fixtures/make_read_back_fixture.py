#!/usr/bin/env python3
"""Regenerates read_back.tar.gz, the fixture of ReadBack and BaselineSet in test_release_runner.py.

The fixture in the repository was written by a debug build of release_timing at commit
e42da5ff19cee9d8276df6b9a3169d9cec80f6f4 (the self-supervising driver's source there is that of every
later commit of this change), with:

    CARGO_TARGET_DIR=BUILD cargo build --locked --manifest-path core/solver/release_timing/Cargo.toml
    python3 -B core/solver/performance_harness/runner/fixtures/make_read_back_fixture.py \
        --binary BUILD/debug/release_timing --work NEW_FOLDER

run from the repository root (on a shared host, the build through its cargo wrapper and this script
through its slot wrapper). It writes two supervised folders under NEW_FOLDER and packs their plan.json,
models/manifest.json and out/ into the fixture (`--fixture` names another path), with the CPU model
blanked in metadata.json and the packet's copy of it. A new run gives new figures (times, memory, host
readings); the tests depend only on the run forms below, which the script checks before it packs.

forms/: every packet row form, from 46-node W1 (baseline-one-case-ex) and the reference system:
  W1 r1 and W2 r1            completed (ok): ordinary runs;
  W1 r2                      stopped, timed_out: a 1 s timeout, shorter than the debug solve;
  W2 r2                      stopped, exceeds_base_machine_budget: a 1,000,000-byte footprint budget;
  baseline-one-case-ex-n0047 never launched, model_missing: its model is deleted after the plan is written;
  baseline-one-case-ex-n0048 never launched, model_mismatch: a byte is appended to its model after the plan;
  W2 r3                      never launched, skipped_window_budget: an estimate (10,000 s) above the window.
window/: two runs of 46-node W1 that the plan expects exceeds_base_machine_budget, in a 4 s window with no
quiet wait: r1 is launched with its timeout capped to what remains of the window and times out
(timeout_capped_by_window true); r2 is then skipped for the window.
"""
import argparse
import gzip
import io
import json
import os
import subprocess
import sys
import tarfile

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))
import release_runner as rr  # noqa: E402

W1, W2 = ('baseline-one-case-ex', 46), ('reference-system', 27)
EXPECTED = {
    'forms': ['ok', 'ok', 'timed_out', 'exceeds_base_machine_budget', 'model_missing', 'model_mismatch',
              'skipped_window_budget'],
    'window': ['timed_out', 'skipped_window_budget'],
}


def bundle(root, which, binary):
    if which == 'forms':
        rows = ((*W1, 120, 1, 'solved'), (*W2, 60, 1, 'solved'), (*W1, 1, 1, 'solved'), (*W2, 60, 1, 'solved'),
                ('baseline-one-case-ex', 47, 60, 1, 'solved'), ('baseline-one-case-ex', 48, 60, 1, 'solved'),
                (*W2, 60, 10000, 'solved'))
        window = 600
    else:
        rows = ((*W1, 120, 1, 'exceeds_base_machine_budget'), (*W1, 120, 1, 'exceeds_base_machine_budget'))
        window = 4
    plan = rr.repeat_run_ids(rr.build_plan(rows, 'sparse'))
    if which == 'forms':
        plan[3]['footprint_budget_bytes'] = 1_000_000
    _, failures = rr.generate(plan, os.path.join(root, 'models'), log=lambda *a: None)
    if failures:
        raise SystemExit('generation failed: %s' % failures)
    rr.write_plan(os.path.join(root, 'plan.json'), plan, window, 0, os.path.join(root, 'models'), require_sha256=True)
    if which == 'forms':
        os.unlink(os.path.join(root, 'models', 'baseline-one-case-ex-n0047.json'))
        with open(os.path.join(root, 'models', 'baseline-one-case-ex-n0048.json'), 'a') as fh:
            fh.write(' ')
    subprocess.run([binary, '--supervise', os.path.join(root, 'plan.json'), '--models', os.path.join(root, 'models'),
                    '--out', os.path.join(root, 'out')], check=False)
    found = [r['classification'] for r in rr.read_set_records(os.path.join(root, 'out'))[0]]
    if found != EXPECTED[which] or (which == 'window' and not rr.read_set_records(
            os.path.join(root, 'out'))[0][0].get('timeout_capped_by_window')):
        raise SystemExit('%s: the runs are %s, not %s; nothing packed' % (which, found, EXPECTED[which]))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('--binary', required=True, help='a debug build of release_timing')
    parser.add_argument('--work', required=True, help='a new folder for the two supervised folders')
    parser.add_argument('--fixture', default=os.path.join(HERE, 'read_back.tar.gz'))
    args = parser.parse_args(argv)
    os.makedirs(args.work)
    members = []
    for which in ('forms', 'window'):
        root = os.path.join(args.work, which)
        os.makedirs(root)
        bundle(root, which, os.path.abspath(args.binary))
        names = ['plan.json', 'models/manifest.json'] + sorted('out/' + n for n in os.listdir(os.path.join(root, 'out')))
        for name in names:
            with open(os.path.join(root, name), 'rb') as fh:
                data = fh.read()
            if name in ('out/metadata.json', 'out/packet.json'):
                document = json.loads(data)
                (document if name == 'out/metadata.json' else document['metadata'])['cpu_model'] = 'test fixture'
                data = (json.dumps(document, indent=1, sort_keys=True) + '\n').encode()
            members.append(('%s/%s' % (which, name), data))
    raw = io.BytesIO()
    with tarfile.open(fileobj=raw, mode='w', format=tarfile.USTAR_FORMAT) as tar:
        for name, data in members:
            info = tarfile.TarInfo(name)
            info.size, info.mtime, info.mode = len(data), 0, 0o644
            tar.addfile(info, io.BytesIO(data))
    with open(args.fixture, 'wb') as fh, gzip.GzipFile(fileobj=fh, mode='wb', mtime=0, filename='') as gz:
        gz.write(raw.getvalue())
    print('%s: %d files' % (args.fixture, len(members)))
    return 0


if __name__ == '__main__':
    sys.exit(main())
