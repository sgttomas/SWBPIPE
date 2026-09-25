"""Minimal portable package metadata for synthetic adapter tests only.

No dated run directory is opened. Numeric/input/selector/criterion bytes stay
identical to the maintained fixtures. The package inventory and review labels
are explicitly synthetic; their patched hashes cannot qualify a real solver.
"""
from contextlib import ExitStack
import hashlib
import json
from pathlib import Path
from unittest.mock import patch

from tools.validation import build_first_static_selection as derivation
from tools.validation import qualification_physics as physics

FIXTURES = Path(__file__).resolve().parents[1] / 'validation/qualification/fixtures/first_static'


def fixture_package(test_case, destination):
    """Create independent temporary files; install test-only admission metadata."""
    destination.mkdir()
    provenance = json.loads((FIXTURES / 'PROVENANCE.json').read_text())
    entries = []
    for row in provenance['files']:
        data = (FIXTURES / row['path']).read_bytes()
        assert hashlib.sha256(data).hexdigest() == row['sha256']
        path = destination / row['path']
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        entries.append({'path': row['path'], 'sha256': row['sha256']})
    packet = (json.dumps({'files': entries}, indent=2) + '\n').encode()
    (destination / 'PACKET_MANIFEST.json').write_bytes(packet)
    admission = {}
    for name in physics.SELECTED_ADMISSION_FILES:
        data = ('Synthetic test-only review/admission stub for ' + name + '\n'
                'No independent review, owner approval or production admission is claimed.\n').encode()
        path = destination / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        admission[name] = hashlib.sha256(data).hexdigest()
    context = ExitStack()
    test_case.addCleanup(context.close)
    context.enter_context(patch.object(derivation, 'PACKET_HASH', hashlib.sha256(packet).hexdigest()))
    context.enter_context(patch.object(derivation, 'SELECTED_ADMISSION_FILES', admission))
    context.enter_context(patch.object(physics, 'SELECTED_ADMISSION_FILES', admission))
    return destination
