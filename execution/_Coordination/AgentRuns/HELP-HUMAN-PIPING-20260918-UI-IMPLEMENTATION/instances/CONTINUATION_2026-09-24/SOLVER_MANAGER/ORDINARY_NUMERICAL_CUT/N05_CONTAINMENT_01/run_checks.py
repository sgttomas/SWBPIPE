from pathlib import Path
import hashlib
import json
import os
import subprocess
import time

OUT = Path(__file__).resolve().parent
ROOT = next(p for p in OUT.parents if (p / '.git').exists())
PROJECT = ROOT / 'projects/chirality-piping'
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
freeze = json.loads((OUT.parent / 'FINAL_SOURCE_FREEZE_03.json').read_text())
assert all(sha(ROOT / x['path']) == x['sha256'] for x in freeze['source_files'])
stages = []
common = dict(os.environ, CARGO_BUILD_JOBS='2', OPENPIPESTRESS_CHECKED_JSON_BIN='/private/tmp/piping-numerical-corrections-canonical-resume-target/debug/openpipestress_jcs_ijson', PYTHONPATH=str(PROJECT))
commands = [
    ('rust-result-export', ['cargo', 'test', '--offline', '--locked', '-j2', '--manifest-path', 'core/reporting/result_export/Cargo.toml', '--test', 'precision_contract'], dict(CARGO_TARGET_DIR='/private/tmp/piping-numerical-corrections-result-export-containment-target')),
    ('rust-headless-binding', ['cargo', 'test', '--offline', '--locked', '-j2', '--manifest-path', 'core/runner/headless/Cargo.toml', '--lib', 'result_envelope_binding::tests'], dict(CARGO_TARGET_DIR='/private/tmp/piping-numerical-corrections-headless-containment-target', HEADLESS_PRECISION_OUTPUT_DIR=str(OUT/'HEADLESS_OUTPUTS'))),
    ('frontend', ['npm', 'test', '--workspace', 'apps/desktop', '--', 'src/features/results/numericalResultQuality.test.ts', 'src/features/workspace/resultsSessionState.test.ts', 'src/features/result-export/resultExportAdapter.test.ts', 'src/features/stress-neutral/StressNeutralExportPanel.test.tsx', 'src/services/previewService.test.ts', '--maxWorkers=2'], {}),
    ('tsc', ['./node_modules/.bin/tsc', '-b', 'apps/desktop/tsconfig.json'], {}),
    ('python', ['/private/tmp/chirality-piping-dec025-venv/bin/python', '-m', 'pytest', '-q', 'tests/test_analysis_run_compatibility.py', 'tests/test_precision_consumer_contract.py'], dict(HEADLESS_PRECISION_OUTPUT_DIR=str(OUT/'HEADLESS_OUTPUTS'))),
]
for name, command, extra in commands:
    start = time.time()
    with (OUT / f'{name}.log').open('w') as log:
        result = subprocess.run(command, cwd=PROJECT, env=dict(common, **extra), stdout=log, stderr=subprocess.STDOUT)
    stages.append({'stage': name, 'command': command, 'cwd_repository_relative': 'projects/chirality-piping', 'environment_overrides': extra, 'exit_code': result.returncode, 'started_unix': start, 'ended_unix': time.time(), 'log_sha256': sha(OUT / f'{name}.log')})
    (OUT / 'STAGES.json').write_text(json.dumps(stages, indent=2) + '\n')
    print(name, result.returncode, flush=True)
    if result.returncode != 0:
        break
(OUT / 'SOURCE_CHECK.json').write_text(json.dumps({'freeze': 'FINAL_SOURCE_FREEZE_03.json', 'files': len(freeze['source_files']), 'unchanged': all(sha(ROOT / x['path']) == x['sha256'] for x in freeze['source_files'])}, indent=2) + '\n')
