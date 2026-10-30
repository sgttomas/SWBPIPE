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


def test_unrelated_security_change_does_not_expand_to_full_suite():
    assert selected('core/security/redaction/controls.py') == sorted(ci.BASE)


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
    }
    assert set(selected('tools/retained_pricing/lib/linemap_lib.py')) == base | {'tests/test_retained_pricing_tool.py'}
    assert selected('core/gui/pkg02_boundary.py') == sorted(base)
    assert selected('core/units/schema_vocabulary.py') == sorted(base)



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
    # A product module sharing a verifier's name is not a verifier.
    assert selected('core/analysis_runs/compatibility.py') == sorted(ci.BASE)
    assert selected('core/units/schema_vocabulary.py') == sorted(ci.BASE)
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
        chosen = ci.select(PROJECT, plan)
        assert 'tests/test_ci_numerical.py' not in chosen, plan
    assert 'tests/test_ci_python.py' in selected('tests/test_ci_python.py')
