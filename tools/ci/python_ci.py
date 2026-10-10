#!/usr/bin/env python3
"""Select Python regression families from the same complete PR diff as Cargo."""
import argparse
from fnmatch import fnmatchcase
import json
from pathlib import Path
import subprocess
import sys

PREFIX = 'projects/chirality-piping/'
BASE = ['tests/security', 'tests/test_constraint_validation.py', 'tests/test_source_blocks_validation.py']
FAMILIES = {
    'schema': ['tests/test_*schema.py', 'tests/test_*schemas.py'],
    'qualification': ['tests/test_qualification*.py'],
    'canonical': ['tests/test_*canonical_json_adapter.py'],
}
BROAD = BASE + ['tests/product_preview', 'tests/test_operation_validation_preview.py',
                'tests/test_analytical_solver_boundary_adapter.py',
                'tests/test_physical_to_analytical_transform.py'] + sum(FAMILIES.values(), [])


def select(project, plan):
    paths = plan.get('paths', [])
    if plan.get('event') != 'pull_request' or not paths:
        patterns = list(BROAD)
    else:
        patterns = list(BASE)
        for path in paths:
            if not path.startswith(PREFIX):
                patterns.extend(BROAD)
                break
            local = path[len(PREFIX):]
            if local.startswith('core/model_transform/'):
                patterns.extend(['tests/test_analytical_solver_boundary_adapter.py',
                                 'tests/test_physical_to_analytical_transform.py'])
            if local.startswith('core/model_operations/validation_preview/'):
                patterns.append('tests/test_operation_validation_preview.py')
            # Run an edited test itself, in addition to its affected family.
            if local.startswith('tests/') and Path(local).name.startswith('test_') and local.endswith('.py') and (project / local).is_file():
                patterns.append(local)
            if local.startswith(('tools/ci/', 'requirements', 'tests/conftest', 'tests/schema_validation')):
                patterns.extend(BROAD)
                patterns.append('tests/test_ci*.py')
            if local.startswith(('fixtures/', 'schemas/', 'tools/validation/')):
                patterns.extend(sum(FAMILIES.values(), []))
            if any(fnmatchcase(local, p) for p in ['schemas/*', 'tests/test_*schema.py', 'tests/test_*schemas.py']):
                patterns.extend(FAMILIES['schema'])
            if (local.startswith(('validation/qualification/', 'tools/validation/qualification', 'core/solver/', 'core/loads/', 'core/product_physics/', 'core/runner/', 'schemas/'))
                    or fnmatchcase(local, 'tests/*qualification*.py')):
                patterns.extend(FAMILIES['qualification'])
            if (local.startswith(('core/serialization/', 'fixtures/canonical_hash/'))
                    or fnmatchcase(local, 'tests/*canonical*.py')):
                patterns.extend(FAMILIES['canonical'])
    # Resolve globs here so shell expansion cannot silently change selection.
    selected = sorted({p.relative_to(project).as_posix() for pattern in patterns for p in project.glob(pattern)})
    if not selected:
        raise ValueError('Python selection unexpectedly empty')
    return selected


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plan', required=True, type=Path)
    parser.add_argument('--dry-run', action='store_true')
    args = parser.parse_args()
    project = Path(__file__).resolve().parents[2]
    selected = select(project, json.loads(args.plan.read_text()))
    print(json.dumps({'selected': selected}, indent=2), flush=True)
    return 0 if args.dry_run else subprocess.run([sys.executable, '-m', 'pytest', '-q', *selected], cwd=project).returncode


if __name__ == '__main__':
    raise SystemExit(main())
