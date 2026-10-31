"""Diff-driven Python families keep their contracts selected on input changes."""
import importlib.util
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('python_ci', PROJECT / 'tools/ci/python_ci.py')
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)


def selected(path):
    return ci.select(PROJECT, {'event': 'pull_request', 'paths': [ci.PREFIX + path]})


def test_sensitive_families_follow_their_inputs():
    assert 'tests/test_qualification_gate.py' in selected('tools/validation/qualification_gate.py')
    assert 'tests/test_qualification_physics.py' in selected('validation/qualification/fixtures/model.json')
    assert 'tests/test_handoff_package_schema.py' in selected('schemas/handoff_package.schema.json')
    assert 'tests/test_checked_canonical_json_adapter.py' in selected('core/serialization/canonical_json/adapter.py')
    assert 'tests/test_binary64_canonical_json_adapter.py' in selected('fixtures/canonical_hash/cases.json')
    assert 'tests/test_qualification_physics.py' in selected('tests/test_qualification_physics.py')


def readers(path):
    """The derived tests whose own reads include `path`."""
    return {test for test, roots in ci.derived_tests(PROJECT).items()
            if any(path == root or path.startswith(root + '/') for root in roots)}


def test_unrelated_security_change_does_not_expand_to_full_suite():
    path = 'core/security/redaction/controls.py'
    assert readers(path)
    assert set(selected(path)) == set(ci.BASE) | readers(path)
    assert not set(ci.FAMILIES['qualification']) & set(selected(path))


def test_dispatch_and_shared_runner_edits_keep_broad_coverage():
    assert 'tests/product_preview' in ci.select(PROJECT, {'event': 'workflow_dispatch'})
    assert 'tests/product_preview' in selected('tools/ci/python_ci.py')


def test_numerical_transform_and_invalid_operation_checks_follow_their_inputs():
    base = set(ci.BASE)
    assert set(selected('core/model_transform/physical_to_analytical/_solver_boundary_adapter.py')) == base | {
        'tests/test_analytical_solver_boundary_adapter.py',
        'tests/test_physical_to_analytical_transform.py',
    }
    assert set(selected('core/model_operations/validation_preview/engine.py')) == base | {
        'tests/test_operation_validation_preview.py',
    } | readers('core/model_operations/validation_preview/engine.py')
    assert set(selected('tools/retained_pricing/lib/linemap_lib.py')) == base | {'tests/test_retained_pricing_tool.py'} | readers(
        'tools/retained_pricing/lib/linemap_lib.py')
    for path in ('core/gui/pkg02_boundary.py', 'core/units/schema_vocabulary.py'):
        assert set(selected(path)) == base | readers(path)



def test_established_helper_edits_keep_direct_importing_tests():
    schema = set(selected('tests/schema_validation.py'))
    assert {'tests/test_schema_validation_helpers.py', 'tests/test_headless_runner_contract.py',
            'tests/test_report_generator_contract.py', 'tests/test_retained_precision_schema.py'} <= schema
    assert {'tests/test_retained_precision_schema.py', 'tests/test_stress_neutral_physics_source.py',
            'tests/test_checked_json_fixture_cache.py'} <= set(selected('tests/checked_json_fixture_cache.py'))
    for helper in ['tests/retained_precision_corpus.py', 'tools/fixtures/retained_precision_corpus.py']:
        assert {'tests/test_retained_precision_b3.py', 'tests/test_retained_precision_contract.py',
                'tests/test_retained_precision_schema.py', 'tests/test_retained_precision_corpus_storage.py'} <= set(selected(helper))


def test_known_helper_import_forms_are_selected_without_unrelated_tests():
    import tempfile
    with tempfile.TemporaryDirectory() as tmp:
        project = Path(tmp)
        tests = project / 'tests'
        tests.mkdir()
        for name, body in [('a', 'from schema_validation import validate_instance'),
                           ('b', 'from tests import schema_validation as validation'),
                           ('c', 'import retained_precision_corpus as corpus'),
                           ('d', 'from . import schema_validation'), ('unrelated', 'import unrelated')]:
            (tests / f'test_{name}.py').write_text(body)
        assert ci.helper_consumers(project, 'tests/schema_validation.py') == [
            'tests/test_a.py', 'tests/test_b.py', 'tests/test_d.py']
        assert ci.helper_consumers(project, 'tests/retained_precision_corpus.py') == ['tests/test_c.py']


def test_full_fallback_and_shared_controls_preserve_edited_test_and_broad_families():
    paths = ['.github/workflows/piping-desktop-e2e.yml', ci.PREFIX + 'tests/test_retained_precision_contract.py']
    for selection in [{}, {'classification': 'full-fallback'}]:
        chosen = ci.select(PROJECT, {'event': 'pull_request', 'paths': paths, 'piping_numerical_selection': selection})
        assert 'tests/test_retained_precision_contract.py' in chosen
        assert 'tests/product_preview' in chosen
    chosen = ci.select(PROJECT, {'event': 'pull_request', 'paths': [ci.PREFIX + 'tools/unknown-new-input.py'],
                               'piping_numerical_selection': {'classification': 'full-fallback'}})
    assert 'tests/product_preview' in chosen
    assert 'tests/test_qualification_gate.py' in chosen


def test_j7_rf_census_generator_inputs_select_the_fixture_currency_check():
    for path in sorted(ci.J7_RF_CENSUS_INPUTS):
        assert ci.J7_RF_CENSUS_TEST in selected(path), path
        assert (PROJECT / path).is_file(), path
    assert ci.J7_RF_CENSUS_TEST not in selected('fixtures/results/pressure_source_reader_corpus.json')


def test_synthetic_base_writer_inputs_select_its_check():
    for path in sorted(ci.SYNTHETIC_BASES_INPUTS):
        assert ci.SYNTHETIC_BASES_TEST in selected(path), path
        assert (PROJECT / path).is_file(), path
    assert ci.SYNTHETIC_BASES_TEST not in selected('fixtures/results/pressure_source_reader_corpus.json')


def test_pinned_fixture_regenerations_select_the_tests_that_pin_their_bytes():
    import hashlib
    for path, readers in sorted(ci.PINNED_FIXTURE_READERS.items()):
        digest = hashlib.sha256((PROJECT / path).read_bytes()).hexdigest()
        for reader in readers:
            assert reader in selected(path), (path, reader)
            assert digest in (PROJECT / reader).read_text(), (path, reader)
    assert 'tests/test_retained_precision_b3.py' not in selected('fixtures/results/pressure_source_reader_corpus.json')


def test_verifier_edits_select_their_importing_tests():
    chosen = set(selected('tests/verifiers/retained_precision.py'))
    assert {'tests/test_retained_precision_b3.py', 'tests/test_retained_precision_contract.py',
            'tests/test_declared_member_load_reader.py', 'tests/test_verifier_independence.py',
            # `from verifiers import retained_precision`, with tests/ on sys.path.
            'tests/test_retained_precision_si_first.py', 'tests/test_retained_precision_failed_ordinary.py',
            # Through another test module: `from tests.test_retained_precision_contract import ...`.
            'tests/test_rust_reader.py'} <= chosen
    # Transitive: this test imports only `reader_verdict` and `source_blocks`; `source_blocks` imports `.canonical_json`.
    assert 'tests/test_base_reader_verifier.py' in selected('tests/verifiers/canonical_json.py')
    assert 'tests/test_retained_precision_si_first.py' not in selected('tests/verifiers/reader_verdict.py')
    assert set(selected('tests/verifiers/__init__.py')) >= chosen


def test_test_tree_imports_are_followed_without_unrelated_tests():
    import tempfile
    with tempfile.TemporaryDirectory() as tmp:
        project = Path(tmp)
        package = project / 'tests/verifiers'
        package.mkdir(parents=True)
        for name, body in [('__init__', ''), ('leaf', 'X = 1'), ('mid', 'def f():\n    from .leaf import X\n'),
                           ('other', 'import json')]:
            (package / f'{name}.py').write_text(body)
        for name, body in [('direct', 'from tests.verifiers import leaf'), ('transitive', 'import tests.verifiers.mid'),
                           ('bare', 'from verifiers import mid'), ('relative', 'from .verifiers.mid import f'),
                           ('via_test', 'from tests.test_transitive import *'),
                           ('unrelated', 'from tests.verifiers import other\nimport leaf_free')]:
            (project / f'tests/test_{name}.py').write_text(body)
        assert ci.test_tree_importers(project, ['tests/verifiers/leaf.py']) == [
            'tests/test_bare.py', 'tests/test_direct.py', 'tests/test_relative.py',
            'tests/test_transitive.py', 'tests/test_via_test.py']
        assert ci.test_tree_importers(project, ['tests/verifiers/other.py']) == ['tests/test_unrelated.py']
        assert ci.test_tree_importers(project, ['tests/verifiers/missing.py']) == []


def test_unrelated_changes_select_no_verifier_importers():
    verifier = set(selected('tests/verifiers/retained_precision.py')) - set(ci.BASE)
    # A product module sharing a verifier's name is not a verifier: it selects only the tests that read it.
    for path in ('core/analysis_runs/compatibility.py', 'core/units/schema_vocabulary.py'):
        assert set(selected(path)) == set(ci.BASE) | readers(path)
    assert not verifier & set(selected('tests/qualification_fixture_support.py'))


def test_supplier_acquisition_inputs_select_offline_boundaries_and_notice_hashes():
    for path in ci.AGENT_SUPPLIER_INPUTS:
        assert ci.AGENT_SUPPLIER_TEST in selected(path), path
    assert ci.AGENT_SUPPLIER_TEST in ci.select(PROJECT, {'event': 'workflow_dispatch'})
    assert ci.AGENT_SUPPLIER_TEST not in selected('core/security/redaction/controls.py')


def test_supplier_bundle_inputs_keep_preparation_and_canonical_build_checks():
    for path in ci.AGENT_SUPPLIER_BUNDLE_INPUTS:
        assert {ci.AGENT_SUPPLIER_BUNDLE_TEST, 'tests/test_release_signed_build_script.py'} <= set(selected(path)), path
    assert ci.AGENT_SUPPLIER_BUNDLE_TEST in ci.select(PROJECT, {'event': 'workflow_dispatch'})
    assert ci.AGENT_SUPPLIER_BUNDLE_TEST not in selected('core/security/redaction/controls.py')


def test_repository_numerical_routing_checks_are_not_duplicated_in_product_job():
    plans = [
        {'event': 'workflow_dispatch'},
        {'event': 'pull_request', 'paths': []},
        *({'event': 'pull_request', 'paths': [path]} for path in (
            'tools/ci/python_ci.py', 'tests/conftest.py', 'tests/test_ci_numerical.py')),
        {'event': 'pull_request', 'paths': ['tools/unknown-new-input.py'],
         'piping_numerical_selection': {'classification': 'full-fallback'}},
    ]
    for plan in plans:
        assert 'tests/test_ci_numerical.py' not in ci.select(PROJECT, plan, repository_checks_run=True), plan
    # Review r1: without the harness's Repository checks (a direct dispatch of
    # the product workflow), the routing suite stays wherever it was selected.
    for path in ('tests/conftest.py', 'tests/test_ci_numerical.py'):
        assert 'tests/test_ci_numerical.py' in ci.select(PROJECT, {'event': 'pull_request', 'paths': [path]}), path
    assert 'tests/test_ci_python.py' in selected('tests/test_ci_python.py')


def test_model_version_literal_guard_follows_every_root_it_scans():
    spec = importlib.util.spec_from_file_location('version_literals', PROJECT / ci.MODEL_VERSION_LITERAL_TEST)
    guard = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(guard)
    scanned = {guard.DESKTOP_TS, guard.DESKTOP_RS, guard.APPLIER_RS, *guard.EXTENDED_RS, *guard.PYTHON_VERSION_ROOTS}
    routed = {PROJECT / root.rstrip('/') for root in ci.MODEL_VERSION_LITERAL_ROOTS}
    assert routed == scanned
    for path in ('apps/desktop/src-tauri/src/embedded_agent/session_fixture.rs', 'apps/desktop/src/services/modelSchemaVersion.ts',
                 'core/product_physics/src/model_format.rs', 'core/model_operations/operation_applier/src/lib.rs',
                 'tools/validation/validate_claims_language.py'):
        assert ci.MODEL_VERSION_LITERAL_TEST in selected(path), path
    assert ci.MODEL_VERSION_LITERAL_TEST in ci.select(PROJECT, {'event': 'workflow_dispatch'})
    assert ci.MODEL_VERSION_LITERAL_TEST not in selected('core/security/redaction/controls.py')


def test_every_maintained_test_runs_when_a_path_it_reads_changes():
    """No maintained Python test runs only when its own file is edited: a rule names it, or it is derived
    from the paths it reads, or it is left unrouted with a reason."""
    tracked = ci.python_test_reads.tracked(PROJECT)
    named, derived = ci.named_tests(PROJECT), ci.derived_tests(PROJECT)
    assert set(ci.UNROUTED) <= set(ci.python_test_reads.maintained_tests(PROJECT))
    dispatch = set(ci.select(PROJECT, {'event': 'workflow_dispatch'}))
    for test in ci.python_test_reads.maintained_tests(PROJECT):
        if test in ci.UNROUTED or test in named:
            continue
        assert derived[test], test
        assert test in dispatch, test
        root = derived[test][0]
        probe = root if root not in tracked or (PROJECT / root).is_file() else min(
            path for path in tracked if path.startswith(root + '/') and (PROJECT / path).is_file())
        assert test in selected(probe), (test, probe)
        assert test in selected(test), test


def test_formerly_never_run_guards_follow_their_roots():
    cases = {
        'tools/retained_pricing/tests/test_w8_topology_model.py': 'core/product_physics/src/validation.rs',
        'tools/retained_pricing/tests/test_silent_zero_guard.py': 'core/product_physics/src/case_state/resolve.rs',
        'tools/retained_pricing/tests/test_w1_reductions.py': 'tools/retained_pricing/price.py',
        'tools/agent_host_witness_test.py': 'tools/agent_host_witness.py',
        'tools/agent_live_session_fixture_test.py': 'tools/agent_live_session_fixture.py',
        'validation/benchmarks/numerical_integrity/test_reference.py': 'validation/benchmarks/numerical_integrity/generate.py',
        'validation/benchmarks/numerical_robustness/runner/test_vk_scale_runner.py':
            'validation/benchmarks/numerical_robustness/runner/vk_scale_runner.py',
        'tests/test_performance_harness_runner.py': 'core/solver/performance_harness/runner/k6_runner.py',
        'tests/test_caepipe_mbf_importer.py': 'core/handoff/caepipe_mbf/importer.py',
        'tests/test_user_guide_status_wording.py': 'docs/user_guide/index.md',
        'tests/test_release_notarize_script.py': 'tools/release/notarize_app.py',
    }
    for test, path in cases.items():
        assert test in selected(path), (test, path)
        assert test not in selected('core/units/schema_vocabulary.py'), test
    assert 'core/solver/performance_harness/runner/test_k6_runner.py' not in selected(
        'core/solver/performance_harness/runner/k6_runner.py')


def test_reads_follow_imports_and_path_loaded_models_not_imported_modules_reads():
    reads = ci.python_test_reads.test_reads(PROJECT)
    # The W-8 model the test loads by path pins validation.rs; the test itself does not name it.
    assert 'core/product_physics/src/validation.rs' not in (PROJECT / 'tools/retained_pricing/tests/test_w8_topology_model.py').read_text()
    assert 'core/product_physics/src/validation.rs' in reads['tools/retained_pricing/tests/test_w8_topology_model.py']
    # Imports resolve through the package, transitively.
    assert 'core/security/redaction/controls.py' in reads['tests/test_caepipe_benchmark.py']
    # The runner build helper is imported, not path-loaded: the crates it builds are not the test's roots.
    assert 'tools/runner/build_runner.py' in reads['tests/test_units_authority_setup.py']
    assert 'core/product_physics' not in reads['tests/test_caepipe_mbf_importer.py']
    # Roots the Python job cannot read, and whole top-level directories, are never roots.
    assert not [root for roots in reads.values() for root in roots
                if '/' not in root or root.startswith('execution/') and not root.startswith('execution/_DAG')]


def test_repository_check_tests_are_the_governance_step_s():
    workflow = (PROJECT / '.github/workflows/governance-harness.yml').read_text()
    line, = [line for line in workflow.splitlines() if 'python3 -m pytest -q -n auto' in line]
    assert tuple(line.split('"${deselect[@]}"')[1].split()) == ci.python_test_reads.REPOSITORY_CHECK_TESTS


def test_derived_roots_route_a_local_check_that_runs_the_test():
    """software-workflow.json registers each derived test for every path it reads."""
    import json
    import sys
    sys.path.insert(0, str(PROJECT / 'tools/software_workflow'))
    from software_workflow_common import matches
    registry = json.loads((PROJECT / 'software-workflow.json').read_text())

    def runs(check, test):
        return any(test == arg or test.startswith(arg.rstrip('/') + '/') for arg in registry['checks'][check]['command'])
    tracked = ci.python_test_reads.tracked(PROJECT)
    for test, roots in ci.derived_tests(PROJECT).items():
        for root in roots:
            probe = root if root not in tracked or (PROJECT / root).is_file() else root + '/x'
            checks = {check for rule in registry['path_rules'] if matches(probe, rule['paths']) for check in rule['checks']}
            assert any(runs(check, test) for check in checks), (test, root)


def test_a_path_only_a_python_test_reads_starts_the_python_job():
    sys_path = str(PROJECT / 'tools/software_workflow')
    import sys
    if sys_path not in sys.path:
        sys.path.insert(0, sys_path)
    import hosted_ci
    _, profile = hosted_ci.load_profile(hosted_ci.PROFILE)
    for path in ('docs/user_guide/index.md', 'apps/desktop/src/services/modelSchemaVersion.ts'):
        selection = hosted_ci.select_paths([path], profile)
        assert selection['modes']['piping-python'] == 'full', path
        assert path in selection['reasons']['piping-python']
    # The always-run security tests read their policy documents.
    assert hosted_ci.select_paths(['docs/security/telemetry_policy.md'], profile)['modes']['piping-python'] == 'full'
    assert hosted_ci.select_paths(['docs/PRD.md'], profile)['modes']['piping-python'] == 'not-applicable'
