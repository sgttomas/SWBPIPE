#!/usr/bin/env python3
"""Select Python regression families from the same complete PR diff as Cargo."""
import argparse
import ast
from fnmatch import fnmatchcase
import json
from pathlib import Path
import subprocess
import sys

PREFIX = 'projects/chirality-piping/'
BASE = ['tests/security', 'tests/test_constraint_validation.py', 'tests/test_source_block_schema_contract.py']
FAMILIES = {
    'schema': ['tests/test_*schema.py', 'tests/test_*schemas.py'],
    'qualification': ['tests/test_qualification*.py'],
    'canonical': ['tests/test_*canonical_json_adapter.py'],
}
BROAD = BASE + ['tests/product_preview', 'tests/test_operation_validation_preview.py',
                'tests/test_analytical_solver_boundary_adapter.py',
                'tests/test_physical_to_analytical_transform.py'] + sum(FAMILIES.values(), [])


HELPER_IMPORTS = {
    'tests/checked_json_fixture_cache.py': {'checked_json_fixture_cache', 'tests.checked_json_fixture_cache'},
    'tests/schema_validation.py': {'schema_validation', 'tests.schema_validation'},
    'tests/retained_precision_corpus.py': {'retained_precision_corpus', 'tests.retained_precision_corpus'},
    'tools/fixtures/retained_precision_corpus.py': {'retained_precision_corpus', 'tests.retained_precision_corpus'},
}
# Python tests that sha256-pin these fixtures' bytes (exact files): a regeneration re-runs them even
# when their pins are not edited.
PINNED_FIXTURE_READERS = {
    **{f'fixtures/results/retained_precision_milestone_successor_{mode}.json': [
        'tests/test_retained_precision_b3.py', 'tests/test_retained_precision_carriers.py',
        'tests/test_retained_precision_contract.py', 'tests/test_retained_precision_schema.py']
       for mode in ('sparse_interactive', 'dense_scrutiny')},
    **{f'fixtures/results/retained_precision_exact_successor_{mode}.json': ['tests/test_retained_precision_b3.py']
       for mode in ('sparse_interactive', 'dense_scrutiny')},
}
# Path-loaded generator inputs of the J7 RF census fixture check (exact files).
J7_RF_CENSUS_TEST = 'tests/test_j7_rf_census_inputs.py'
J7_RF_CENSUS_INPUTS = {
    'tools/fixtures/j7_rf_census_inputs.py',
    'tools/fixtures/retained_precision_corpus.py',
    'fixtures/results/j7_rf_census_inputs.json',
    'fixtures/results/j7_rf_census_expected.json',
    'fixtures/product_preview/pressure_source/physics_source/n05.request.json',
    'validation/benchmarks/numerical_robustness/cases/gen_vk_cases.py',
    'validation/references/t3_r1/REFERENCES/references.json',
    'validation/references/t3_r1/REFERENCES/references.py',
}


def helper_consumers(project, helper):
    """Direct test imports of the established shared test helpers only."""
    names = HELPER_IMPORTS.get(helper)
    if names is None:
        return []
    consumers = []
    for source in sorted((project / 'tests').rglob('test_*.py')):
        imported = set()
        for node in ast.walk(ast.parse(source.read_text(), filename=str(source))):
            if isinstance(node, ast.Import):
                imported.update(alias.name for alias in node.names)
            elif isinstance(node, ast.ImportFrom):
                module = node.module or ''
                imported.add(module)
                imported.update(module + '.' + alias.name for alias in node.names)
                if not module:
                    imported.update(alias.name for alias in node.names)
        if imported & names:
            consumers.append(source.relative_to(project).as_posix())
    return consumers


def select(project, plan):
    paths = plan.get('paths', [])
    if plan.get('event') != 'pull_request' or not paths:
        patterns = list(BROAD)
    else:
        patterns = list(BASE)
        for path in paths:
            if not path.startswith(PREFIX):
                patterns.extend(BROAD)
                continue
            local = path[len(PREFIX):]
            patterns.extend(helper_consumers(project, local))
            if local in J7_RF_CENSUS_INPUTS:
                patterns.append(J7_RF_CENSUS_TEST)
            patterns.extend(PINNED_FIXTURE_READERS.get(local, []))
            if local.startswith('core/model_transform/'):
                patterns.extend(['tests/test_analytical_solver_boundary_adapter.py',
                                 'tests/test_physical_to_analytical_transform.py'])
            if local.startswith('core/model_operations/validation_preview/'):
                patterns.append('tests/test_operation_validation_preview.py')
            if local.startswith('tools/retained_pricing/'):
                patterns.append('tests/test_retained_pricing_tool.py')
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
    if plan.get('piping_numerical_selection', {}).get('classification') == 'full-fallback':
        patterns.extend(BROAD)
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
    plan = json.loads(args.plan.read_text())
    root = project.parents[1]
    sys.path.insert(0, str(root / 'tools/software_workflow'))
    import hosted_ci
    hosted_ci.validate_plan(root, plan)
    if plan.get('modes', {}).get('piping-python') != 'full':
        raise ValueError('Python runner requires explicit selected coverage')
    selected = select(project, plan)
    print(json.dumps({'selected': selected}, indent=2), flush=True)
    return 0 if args.dry_run else subprocess.run([sys.executable, '-m', 'pytest', '-q', *selected], cwd=project).returncode


if __name__ == '__main__':
    raise SystemExit(main())
