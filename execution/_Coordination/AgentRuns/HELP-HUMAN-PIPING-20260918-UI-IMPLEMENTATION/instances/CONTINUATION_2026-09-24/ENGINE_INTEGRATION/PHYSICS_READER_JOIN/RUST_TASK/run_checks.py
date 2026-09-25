#!/usr/bin/env python3
"""Bounded Rust consumer tests. Run only after the parent's heavy-lane release."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[11]
OUT = Path(__file__).resolve().parent
COMMAND = [
    'cargo', 'test', '--offline', '--manifest-path',
    'projects/chirality-piping/core/reporting/result_export/Cargo.toml',
    '--jobs', '2', '--test', 'physics_contract', '--test', 'precision_contract',
    '--test', 'derivative_contract',
]
if not (ROOT / 'projects/chirality-piping').is_dir():
    raise SystemExit(f'Incorrect anchored repository path: {ROOT}')
ENV = os.environ.copy()
ENV['CARGO_TARGET_DIR'] = '/private/tmp/piping-physics-result-export-target'
start = datetime.datetime.now(datetime.timezone.utc).isoformat()
completed = subprocess.run(COMMAND, cwd=ROOT, env=ENV, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
attempt = 1
while (OUT / f'cargo-consumer-tests-{attempt:02}.log').exists():
    attempt += 1
name = f'cargo-consumer-tests-{attempt:02}'
(OUT / f'{name}.log').write_bytes(completed.stdout)
record = {'command': COMMAND, 'cwd': str(ROOT), 'environment': {'CARGO_TARGET_DIR': ENV['CARGO_TARGET_DIR']},
          'start_utc': start, 'end_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
          'exit_code': completed.returncode, 'log': f'{name}.log',
          'log_sha256': hashlib.sha256(completed.stdout).hexdigest()}
(OUT / f'{name}.json').write_text(json.dumps(record, indent=2) + '\n')
print(completed.stdout.decode(errors='replace'))
print(json.dumps(record, indent=2))
raise SystemExit(completed.returncode)
