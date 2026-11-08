"""Build (no run) the two AC-004 witness test binaries of the RT-A3 S6 final composition, and record their binding.

Invoked as: CARGO_TARGET_DIR=... CARGO_BUILD_JOBS=2 t3_slot.sh python3 -I build_witnesses.py <expected head> <expected core>
Adapted from the modal composition's build_freshness.py; no test is executed.
"""
from pathlib import Path
import hashlib, json, os, subprocess, sys

REPO = Path('/Users/ryan/dev/chirality-t3/swbpipe-rt-a3-s6-final')
OUT = Path('/Users/ryan/dev/chirality-t3/scratch/rt-a3-s6-final/ac004')
HEAD, CORE, LABEL = sys.argv[1], sys.argv[2], sys.argv[3]
FLAGS = ('RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'CARGO_BUILD_RUSTFLAGS', 'RUSTDOCFLAGS')


def query(*args):
    return subprocess.check_output(args, text=True).strip()


def digest(p):
    h = hashlib.sha256()
    with open(p, 'rb') as f:
        for b in iter(lambda: f.read(1 << 20), b''):
            h.update(b)
    return h.hexdigest()


def state():
    return {'head': query('git', '-C', str(REPO), 'rev-parse', 'HEAD'),
            'core': query('git', '-C', str(REPO), 'rev-parse', 'HEAD:core'),
            'status': query('git', '-C', str(REPO), 'status', '--porcelain', '--untracked-files=all'),
            'flags_present': {k: k in os.environ for k in FLAGS},
            'profile_sha256': digest(REPO / 'core/product_physics/src/retained_memory.rs'),
            'lock_sha256': digest(REPO / 'core/product_physics/Cargo.lock')}


before = state()
assert before['head'] == HEAD and before['core'] == CORE and before['status'] == '', before
assert not any(before['flags_present'].values()), before['flags_present']
rustc = query('rustc', '-vV')
fields = dict(line.split(': ', 1) for line in rustc.splitlines() if ': ' in line)
assert (fields['release'], fields['commit-hash'], fields['host']) == (
    '1.97.1', '8bab26f4f68e0e26f0bb7960be334d5b520ea452', 'aarch64-apple-darwin'), fields
cmd = ['cargo', 'test', '--offline', '--locked', '--no-run', '--lib', '--test', 'retained_memory_challenge',
       '--message-format=json-render-diagnostics']
record = {'label': LABEL, 'before': before, 'compiler': fields, 'compiler_verbose': rustc, 'command': cmd,
          'cwd': str(REPO / 'core/product_physics'), 'outer_wrapper': '/Users/ryan/dev/chirality-t3/tools/t3_slot.sh',
          'target_dir': os.environ['CARGO_TARGET_DIR'], 'jobs': os.environ['CARGO_BUILD_JOBS'],
          'flag_presence': before['flags_present'], 'test_execution_authorized_or_performed': False}
with (OUT / f'{LABEL}.jsonl').open('w') as stdout, (OUT / f'{LABEL}.err').open('w') as stderr:
    rc = subprocess.run(cmd, cwd=record['cwd'], stdout=stdout, stderr=stderr).returncode
record['returncode'] = rc
(OUT / f'{LABEL}-receipt.json').write_text(json.dumps(record, indent=2) + '\n')
assert rc == 0, rc
artifacts = []
for line in (OUT / f'{LABEL}.jsonl').read_text().splitlines():
    try:
        d = json.loads(line)
    except json.JSONDecodeError:
        continue
    if (d.get('reason') == 'compiler-artifact' and d.get('manifest_path') == str(REPO / 'core/product_physics/Cargo.toml')
            and d.get('executable')):
        assert d['profile']['test'] and d['profile']['opt_level'] == '0' and d['profile']['debug_assertions'], d['profile']
        exe = d['executable']
        dep_info = exe + '.d'
        local = sorted({p for p in Path(dep_info).read_text().replace('\\\n', ' ').split()
                        if p.startswith(str(REPO))} - {exe + ':'}) if Path(dep_info).exists() else None
        artifacts.append({'target': d['target']['name'], 'kind': d['target']['kind'], 'executable': exe,
                          'sha256': digest(exe), 'profile': d['profile'], 'features': d['features'], 'fresh': d['fresh'],
                          'dep_info': dep_info, 'dep_info_local_files': None if local is None else len(local),
                          'dep_info_outside_core': None if local is None else
                          [p for p in local if not p.startswith(str(REPO / 'core'))]})
assert {a['target'] for a in artifacts} == {'open_pipe_stress_product_physics', 'retained_memory_challenge'} \
    and len(artifacts) == 2, artifacts
record['artifacts'] = artifacts
record['after'] = state()
assert record['after'] == before, (before, record['after'])
assert query('rustc', '-vV') == rustc
(OUT / f'{LABEL}-receipt.json').write_text(json.dumps(record, indent=2) + '\n')
print(json.dumps({'returncode': rc, 'head': HEAD, 'core': CORE, 'artifacts': artifacts,
                  'flags_present': before['flags_present']}, indent=1))
