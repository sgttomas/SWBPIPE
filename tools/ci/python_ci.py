#!/usr/bin/env python3
"""Select Python regression families from the same complete PR diff as Cargo."""
import argparse
import ast
from fnmatch import fnmatchcase
import json
from pathlib import Path
import subprocess
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'software_workflow'))
import hosted_ci

PREFIX = ''
BASE = ['tests/security', 'tests/test_constraint_validation.py', 'tests/test_source_block_schema_contract.py']
FAMILIES = {
    # The report package's schema evaluator against the Draft 2020-12 oracle over the results schemas.
    'schema': ['tests/test_*schema.py', 'tests/test_*schemas.py', 'tests/test_schema_eval_parity.py'],
    'qualification': ['tests/test_qualification*.py'],
    'canonical': ['tests/test_*canonical_json_adapter.py'],
}
AGENT_SUPPLIER_TEST = 'tools/agent_supplier_resource_test.py'
AGENT_SUPPLIER_INPUTS = {
    'tools/agent_supplier_resource.py',
    'tools/agent_supplier_resource_test.py',
    'apps/desktop/src-tauri/resources/agent-supplier-pins.json',
    'docs/THIRD_PARTY_NOTICES.md',
    'docs/third-party/codex-0.160.0-LICENSE.txt',
    'docs/third-party/codex-0.160.0-NOTICE.txt',
}
AGENT_SUPPLIER_BUNDLE_TEST = 'tools/agent_supplier_bundle_test.py'
AGENT_SUPPLIER_BUNDLE_INPUTS = AGENT_SUPPLIER_INPUTS | {
    'tools/agent_supplier_bundle.py',
    'tools/agent_supplier_bundle_test.py',
    'tools/release/build_signed_app.py',
    'tests/test_release_signed_build_script.py',
}
# The model-version literal guard scans these production roots (its own constants;
# `tests/test_ci_python.py` keeps the two lists equal). A change under any of them
# selects it, so a stray version literal fails on the PR that adds it.
MODEL_VERSION_LITERAL_TEST = 'tests/test_model_version_literals.py'
MODEL_VERSION_LITERAL_ROOTS = (
    'apps/desktop/src/', 'apps/desktop/src-tauri/src/', 'core/model_operations/operation_applier/src/',
    'core/product_physics/src/', 'core/reporting/result_export/src/', 'tests/verifiers/', 'tools/validation/',
    'tools/fixtures/', 'core/handoff/', 'core/project_persistence/', 'core/product_preview/',
)
BROAD = BASE + [AGENT_SUPPLIER_TEST, AGENT_SUPPLIER_BUNDLE_TEST, MODEL_VERSION_LITERAL_TEST, 'tests/product_preview', 'tests/test_operation_validation_preview.py',
                'tests/test_analytical_solver_boundary_adapter.py',
                'tests/test_physical_to_analytical_transform.py',
                'tests/test_reference_system_fixture.py'] + sum(FAMILIES.values(), [])


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

# The synthetic reader-control bases' writer and the files it reads or writes (exact files): its --check
# test reproduces the committed corpus from them.
SYNTHETIC_BASES_TEST = 'tests/test_retained_precision_synthetic_bases.py'
SYNTHETIC_BASES_INPUTS = {
    'tools/fixtures/retained_precision_synthetic_bases.py',
    'tools/fixtures/retained_precision_corpus.py',
    'fixtures/results/retained_precision_cases.json',
    'fixtures/results/retained_precision_b2_parity_probes.json',
    'fixtures/results/retained_precision_milestone_successor_sparse_interactive.json',
    'fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json',
    'fixtures/results/semantic_contract_v0_3_physics_retained_1.json',
    'fixtures/canonical_hash/cases.json',
}

# The reference-system fixture check (DEL-09-01 Work/reference-system.md): its generator, files and specification,
# and the product inputs its EX solve and self-weight capture read: the runner and its path dependencies
# (prefixes and exact files). Full runs include it through BROAD.
REFERENCE_SYSTEM_TEST = 'tests/test_reference_system_fixture.py'
REFERENCE_SYSTEM_PREFIXES = ('fixtures/reference_system/', 'core/product_physics/', 'core/runner/', 'core/loads/',
                             'core/solver/', 'core/units/src/', 'core/units/Cargo', 'core/numerics/', 'core/serialization/',
                             'core/reporting/', 'validation/benchmarks/mechanics/',
                             'validation/benchmarks/nonlinear/', 'validation/benchmarks/stress/')
REFERENCE_SYSTEM_INPUTS = {
    'execution/PKG-09/DEL-09-01/Work/reference-system.md',
    'schemas/model.schema.yaml',
    'schemas/load_reference_state.schema.json',
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


def _module_name(local):
    """Dotted name of a project-relative Python file (a package's `__init__.py` names the package)."""
    parts = list(Path(local).with_suffix('').parts)
    return '.'.join(parts[:-1] if parts[-1] == '__init__' else parts)


def _imported_names(source, name, is_package):
    """Every module name one test-tree file may import, as written and as resolved from `tests/` on sys.path."""
    package = name.split('.') if is_package else name.split('.')[:-1]
    found = set()
    for node in ast.walk(ast.parse(source.read_text(), filename=str(source))):
        if isinstance(node, ast.Import):
            modules = [alias.name for alias in node.names]
        elif isinstance(node, ast.ImportFrom):
            if node.level:
                anchor = package[:len(package) - node.level + 1]
                base = '.'.join(anchor + ([node.module] if node.module else []))
            else:
                base = node.module or ''
            modules = [base] + [f'{base}.{alias.name}' for alias in node.names]
        else:
            continue
        for module in filter(None, modules):
            for written in (module, 'tests.' + module):
                parts = written.split('.')
                found.update('.'.join(parts[:end]) for end in range(1, len(parts) + 1))
    return found


def test_tree_importers(project, changed):
    """Test files that import a changed test-tree module (test-only support code, such as a package of
    test-only verifiers), directly or through other test-tree modules, read from the tests' own import
    statements: no module here names, imports or loads that support code."""
    tests = project / 'tests'
    files = {_module_name(path.relative_to(project).as_posix()): path
             for path in tests.rglob('*.py') if '__pycache__' not in path.parts}
    imports = {name: _imported_names(path, name, path.name == '__init__.py') for name, path in files.items()}
    targets = {_module_name(local) for local in changed}
    importers = []
    for name, path in sorted(files.items()):
        if not path.name.startswith('test_'):
            continue
        seen, stack = {name}, [name]
        while stack:
            for imported in imports.get(stack.pop(), ()):
                if imported not in seen:
                    seen.add(imported)
                    stack.append(imported)
        if seen & targets - {name}:
            importers.append(path.relative_to(project).as_posix())
    return importers


def select(project, plan, repository_checks_run=False):
    paths = plan.get('paths', [])
    if plan.get('event') != 'pull_request' or not paths:
        patterns = list(BROAD)
    else:
        patterns = list(BASE)
        for path in paths:
            if hosted_ci.shared_input(path) or not path.startswith(PREFIX):
                patterns.extend(BROAD)
                continue
            local = path[len(PREFIX):]
            patterns.extend(helper_consumers(project, local))
            if local in AGENT_SUPPLIER_INPUTS:
                patterns.append(AGENT_SUPPLIER_TEST)
            if local.startswith(MODEL_VERSION_LITERAL_ROOTS):
                patterns.append(MODEL_VERSION_LITERAL_TEST)
            if local in AGENT_SUPPLIER_BUNDLE_INPUTS:
                patterns.extend([AGENT_SUPPLIER_BUNDLE_TEST, 'tests/test_release_signed_build_script.py'])
            if local in J7_RF_CENSUS_INPUTS:
                patterns.append(J7_RF_CENSUS_TEST)
            if local in SYNTHETIC_BASES_INPUTS:
                patterns.append(SYNTHETIC_BASES_TEST)
            if local in REFERENCE_SYSTEM_INPUTS or local.startswith(REFERENCE_SYSTEM_PREFIXES):
                patterns.append(REFERENCE_SYSTEM_TEST)
            patterns.extend(PINNED_FIXTURE_READERS.get(local, []))
            if local.startswith('core/model_transform/'):
                patterns.extend(['tests/test_analytical_solver_boundary_adapter.py',
                                 'tests/test_physical_to_analytical_transform.py'])
            if local.startswith('core/model_operations/validation_preview/'):
                patterns.append('tests/test_operation_validation_preview.py')
            if local.startswith('core/reporting/report_package/'):
                patterns.append('tests/test_schema_eval_parity.py')
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
        support = [path[len(PREFIX):] for path in paths if path.startswith(PREFIX + 'tests/') and path.endswith('.py')
                   and not Path(path).name.startswith('test_')]
        if support:
            patterns.extend(test_tree_importers(project, support))
    if plan.get('piping_numerical_selection', {}).get('classification') == 'full-fallback':
        patterns.extend(BROAD)
    # Resolve globs here so shell expansion cannot silently change selection.
    selected = {p.relative_to(project).as_posix() for pattern in patterns for p in project.glob(pattern)}
    if repository_checks_run:
        # The calling harness's Repository checks job runs this routing suite;
        # a direct dispatch of the product workflow keeps it here.
        selected.discard('tests/test_ci_numerical.py')
    selected = sorted(selected)
    if not selected:
        raise ValueError('Python selection unexpectedly empty')
    return selected


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plan', required=True, type=Path)
    parser.add_argument('--dry-run', action='store_true')
    parser.add_argument('--repository-checks-run', action='store_true',
                        help='The caller runs tests/test_ci_numerical.py in Repository checks')
    args = parser.parse_args()
    project = Path(__file__).resolve().parents[2]
    plan = json.loads(args.plan.read_text())
    root = project
    hosted_ci.validate_plan(root, plan)
    if plan.get('modes', {}).get('piping-python') != 'full':
        raise ValueError('Python runner requires explicit selected coverage')
    selected = select(project, plan, args.repository_checks_run)
    print(json.dumps({'selected': selected}, indent=2), flush=True)
    # Loadscope groups module functions or class methods on each parallel worker.
    # The controller builds the test binaries once before workers start (tests/conftest.py).
    # Name each result as it arrives, so a timeout cannot hide failing test identities.
    command = [sys.executable, '-m', 'pytest', '-v', '--tb=short', '--durations=20',
               '-n', 'auto', '--dist', 'loadscope', *selected]
    return 0 if args.dry_run else subprocess.run(command, cwd=project).returncode


if __name__ == '__main__':
    raise SystemExit(main())
