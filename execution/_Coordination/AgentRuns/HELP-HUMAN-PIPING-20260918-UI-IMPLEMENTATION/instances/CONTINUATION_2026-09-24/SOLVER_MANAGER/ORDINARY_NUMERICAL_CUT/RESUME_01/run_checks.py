from pathlib import Path
import hashlib
import json
import os
import subprocess
import time

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / '.git').exists())
PROJECT = ROOT / 'projects/chirality-piping'
TARGET = Path('/private/tmp/piping-numerical-corrections-canonical-resume-target')
PYTHON = '/private/tmp/chirality-piping-dec025-venv/bin/python'
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
freeze = json.loads((HERE.parent / 'FINAL_SOURCE_FREEZE_02.json').read_text())
assert all(sha(ROOT / f['path']) == f['sha256'] for f in freeze['source_files'])
env = dict(os.environ, CARGO_TARGET_DIR=str(TARGET), CARGO_BUILD_JOBS='2')
stages = []
commands = [
    ('canonical-build', ['cargo', 'build', '--offline', '--locked', '-j2', '--manifest-path', 'core/serialization/canonical_json/Cargo.toml', '--features', 'checked-cli', '--bin', 'openpipestress_jcs_ijson'], PROJECT),
    ('python', [PYTHON, '-m', 'pytest', '-q', 'tests/test_analysis_run_compatibility.py', 'tests/test_precision_consumer_contract.py'], PROJECT),
    ('vitest', ['./node_modules/.bin/vitest', 'run', 'src/features/result-export/resultExportAdapter.test.ts', '--maxWorkers=2', '--minWorkers=1'], PROJECT / 'apps/desktop'),
    ('tsc', ['./node_modules/.bin/tsc', '--noEmit'], PROJECT / 'apps/desktop'),
]
for name, command, cwd in commands:
    if name == 'python':
        env['OPENPIPESTRESS_CHECKED_JSON_BIN'] = str(TARGET / 'debug/openpipestress_jcs_ijson')
        env['PYTHONPATH'] = str(PROJECT)
    start = time.time()
    with (HERE / f'{name}.log').open('w') as log:
        result = subprocess.run(command, cwd=cwd, env=env, stdout=log, stderr=subprocess.STDOUT)
    record = {'stage': name, 'command': command, 'cwd_repository_relative': str(cwd.relative_to(ROOT)), 'exit_code': result.returncode, 'started_unix': start, 'ended_unix': time.time(), 'log': f'{name}.log', 'log_sha256': sha(HERE / f'{name}.log')}
    if name == 'canonical-build' and result.returncode == 0:
        record['binary_sha256'] = sha(TARGET / 'debug/openpipestress_jcs_ijson')
    stages.append(record)
    (HERE / 'STAGES.json').write_text(json.dumps(stages, indent=2) + '\n')
    print(name, result.returncode, flush=True)
    if name == 'canonical-build' and result.returncode:
        break
(HERE / 'FINAL_SOURCE_CHECK.json').write_text(json.dumps({'freeze': 'FINAL_SOURCE_FREEZE_02.json', 'source_files': len(freeze['source_files']), 'unchanged': all(sha(ROOT / f['path']) == f['sha256'] for f in freeze['source_files'])}, indent=2) + '\n')
