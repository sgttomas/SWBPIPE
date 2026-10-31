"""Diff-driven Python families keep their contracts selected on input changes."""
import bisect
import functools
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


def by_rules(path):
    """The tests the named rules select for `path`: its selection less the tests whose own derived roots reach it.
    The derivation fails toward over-selection (a test whose reads the code does not show reaches every path), so
    a rule's specificity is asserted on this, not on the whole selection."""
    return set(selected(path)) - readers(path)


def readers(path):
    """The derived tests whose own reads include `path`."""
    return {test for test, roots in ci.derived_tests(PROJECT).items()
            if any(ci.python_test_reads.root_matches(path, root) for root in roots)}


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
    adapter = 'core/model_transform/physical_to_analytical/_solver_boundary_adapter.py'
    assert set(selected(adapter)) == base | {
        'tests/test_analytical_solver_boundary_adapter.py',
        'tests/test_physical_to_analytical_transform.py',
    } | readers(adapter)
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
    assert ci.J7_RF_CENSUS_TEST not in by_rules('fixtures/results/pressure_source_reader_corpus.json')


def test_synthetic_base_writer_inputs_select_its_check():
    for path in sorted(ci.SYNTHETIC_BASES_INPUTS):
        assert ci.SYNTHETIC_BASES_TEST in selected(path), path
        assert (PROJECT / path).is_file(), path
    assert ci.SYNTHETIC_BASES_TEST not in by_rules('fixtures/results/pressure_source_reader_corpus.json')


def test_reference_system_inputs_select_its_fixture_check():
    for path in sorted(ci.REFERENCE_SYSTEM_INPUTS) + ['fixtures/reference_system/reference_system.json',
                                                       'core/product_physics/src/self_weight.rs']:
        assert ci.REFERENCE_SYSTEM_TEST in selected(path), path
        assert (PROJECT / path).is_file(), path
    assert ci.REFERENCE_SYSTEM_TEST not in by_rules('fixtures/results/pressure_source_reader_corpus.json')
    # Its frozen references (EQ-1, EQ-2) and their generator.
    for path in ['validation/references/reference_system/references.json',
                 'validation/references/reference_system/generators/rs_reference.py']:
        assert ci.REFERENCE_SYSTEM_TEST in selected(path), path
    assert ci.REFERENCE_SYSTEM_TEST not in by_rules('validation/references/w16_imposed_motion/references.json')
    # The runner's solver kernels and units reach the EX solve.
    for path in ['core/solver/curved_bend/src/lib.rs', 'core/solver/sparse_direct/src/lib.rs', 'core/units/src/lib.rs']:
        assert ci.REFERENCE_SYSTEM_TEST in selected(path), path
    # Full runs (dispatch, push) and a full-fallback numerical plan include it.
    assert ci.REFERENCE_SYSTEM_TEST in ci.select(PROJECT, {'event': 'workflow_dispatch', 'paths': []})
    assert ci.REFERENCE_SYSTEM_TEST in ci.select(PROJECT, {'event': 'push', 'paths': ['README.md']})
    assert ci.REFERENCE_SYSTEM_TEST in ci.select(PROJECT, {
        'event': 'pull_request', 'paths': ['core/solver/frame_kernel/src/lib.rs'],
        'piping_numerical_selection': {'classification': 'full-fallback'}})


def test_pinned_fixture_regenerations_select_the_tests_that_pin_their_bytes():
    import hashlib
    for path, readers in sorted(ci.PINNED_FIXTURE_READERS.items()):
        digest = hashlib.sha256((PROJECT / path).read_bytes()).hexdigest()
        for reader in readers:
            assert reader in selected(path), (path, reader)
            assert digest in (PROJECT / reader).read_text(), (path, reader)
    assert 'tests/test_retained_precision_b3.py' not in by_rules('fixtures/results/pressure_source_reader_corpus.json')


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
    assert 'tests/test_retained_precision_si_first.py' not in by_rules('tests/verifiers/reader_verdict.py')
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
    # A support edit selects its family and the tests that import or read it, not the verifiers' importers as such.
    support = 'tests/qualification_fixture_support.py'
    family = {path.relative_to(PROJECT).as_posix() for pattern in ci.FAMILIES['qualification'] for path in PROJECT.glob(pattern)}
    expected = set(ci.BASE) | family | set(ci.test_tree_importers(PROJECT, [support])) | readers(support)
    assert set(selected(support)) == expected


def test_any_python_module_outside_tests_selects_the_verifier_independence_check():
    # It reads every such module, including one added later (its derived `**/*.py` scan); a path naming the
    # verifiers in tools/ci once merged unchecked. No named rule selects it: its derived roots do.
    test = 'tests/test_verifier_independence.py'
    for path in ('tools/ci/python_ci.py', 'core/units/adapter.py', 'tools/agent_journey_fixture.py',
                 'core/a_module_added_later/added.py'):
        assert test in selected(path), path
    assert test in ci.select(PROJECT, {'event': 'workflow_dispatch'})
    assert test not in by_rules('docs/PRD.md')


def test_supplier_acquisition_inputs_select_offline_boundaries_and_notice_hashes():
    for path in ci.AGENT_SUPPLIER_INPUTS:
        assert ci.AGENT_SUPPLIER_TEST in selected(path), path
    assert ci.AGENT_SUPPLIER_TEST in ci.select(PROJECT, {'event': 'workflow_dispatch'})
    assert ci.AGENT_SUPPLIER_TEST not in by_rules('core/security/redaction/controls.py')


def test_supplier_bundle_inputs_keep_preparation_and_canonical_build_checks():
    for path in ci.AGENT_SUPPLIER_BUNDLE_INPUTS:
        assert {ci.AGENT_SUPPLIER_BUNDLE_TEST, 'tests/test_release_signed_build_script.py'} <= set(selected(path)), path
    assert ci.AGENT_SUPPLIER_BUNDLE_TEST in ci.select(PROJECT, {'event': 'workflow_dispatch'})
    assert ci.AGENT_SUPPLIER_BUNDLE_TEST not in by_rules('core/security/redaction/controls.py')


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
    """The guard is selected by its derived roots, which include every root it scans: the selector names
    none of them itself (tests/test_verifier_independence.py forbids naming `tests/verifiers` outside tests)."""
    test = 'tests/test_model_version_literals.py'
    spec = importlib.util.spec_from_file_location('version_literals', PROJECT / test)
    guard = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(guard)
    scanned = {guard.DESKTOP_TS, guard.DESKTOP_RS, guard.APPLIER_RS, *guard.EXTENDED_RS, *guard.PYTHON_VERSION_ROOTS}
    assert {(PROJECT / root) for root in ci.derived_tests(PROJECT)[test]} >= scanned
    for path in ('apps/desktop/src-tauri/src/embedded_agent/session_fixture.rs', 'apps/desktop/src/services/modelSchemaVersion.ts',
                 'core/product_physics/src/model_format.rs', 'core/model_operations/operation_applier/src/lib.rs',
                 'tools/validation/validate_claims_language.py', 'tests/verifiers/u4b.py'):
        assert test in selected(path), path
    assert test in ci.select(PROJECT, {'event': 'workflow_dispatch'})
    assert test not in by_rules('core/gui/pkg02_boundary.py')


@functools.lru_cache(maxsize=None)
def tracked_files():
    reads = ci.python_test_reads
    return tuple(sorted(path for path in reads.tracked(PROJECT) if (PROJECT / path).is_file()))


def probe(roots):
    """A tracked file a change to which reaches one of `roots`: a literal root's own file or first file, else the
    first file a pattern matches."""
    reads, files = ci.python_test_reads, tracked_files()
    for root in sorted(roots, key=lambda root: (reads.is_pattern(root), root)):
        if not reads.is_pattern(root):
            index = bisect.bisect_left(files, root)
            for path in files[index:index + 1] + files[bisect.bisect_left(files, root + '/'):][:1]:
                if reads.root_matches(path, root):
                    return path
            continue
        for path in files:
            if reads.root_matches(path, root):
                return path
    raise AssertionError(f'no tracked file under {sorted(roots)[:3]}')


def test_every_maintained_test_runs_when_a_path_it_reads_changes():
    """No maintained Python test runs only when its own file is edited: every test outside UNROUTED is selected
    by the paths it reads, as well as by any rule that names it."""
    named, derived = ci.named_tests(PROJECT), ci.derived_tests(PROJECT)
    assert set(ci.UNROUTED) <= set(ci.python_test_reads.maintained_tests(PROJECT))
    dispatch = set(ci.select(PROJECT, {'event': 'workflow_dispatch'}))
    for test in ci.python_test_reads.maintained_tests(PROJECT):
        if test in ci.UNROUTED:
            continue
        assert derived[test], test
        assert test in named or test in dispatch, test
        path = probe(derived[test])
        assert test in selected(path), (test, path)
        assert test in selected(test), test


def test_named_tests_also_follow_the_paths_they_read():
    """A rule naming a test does not replace its derived roots (review of #28, finding 1)."""
    test = 'tests/test_headless_runner_contract.py'
    assert test in ci.named_tests(PROJECT)
    for path in ('schemas/headless_runner.schema.yaml', 'core/runner/headless/src/bin/openpipestress-runner.rs'):
        assert test in selected(path), path


def test_finite_parameters_resolve_to_the_scripts_they_run_and_their_reads():
    """`VECTORS / writer` over a parametrized writer names each writer, which is followed with its own reads
    (review of #28, finding 2)."""
    test = 'tests/test_member_load_reference_vectors.py'
    reads = ci.python_test_reads.test_reads(PROJECT)[test]
    for writer in ('gen_k4_vectors.py', 'gen_member_recovery_vectors.py', 'gen_member_maximum_vectors.py'):
        assert 'core/solver/frame_kernel/tests/retained_k4/' + writer in reads
    for path in ('validation/references/t3_r1/REFERENCES/references.py', 'validation/references/t3_r1/floor_kinds.json',
                 'core/solver/frame_kernel/tests/retained_k4/gen_member_recovery_vectors.py'):
        assert test in selected(path), path


def test_repository_wide_scans_keep_their_scope():
    """A scan over the project root matches every file it can reach, including one added later
    (review of #28, finding 3)."""
    test = 'tests/test_verifier_independence.py'
    for path in ('tools/agent_host_witness.py', 'core/a_module_added_later/added.py'):
        assert test in selected(path), path
    assert test not in by_rules('docs/user_guide/index.md')
    reads = ci.python_test_reads
    assert reads.root_matches('tools/agent_host_witness.py', '**/*.py')
    assert reads.root_matches('fixtures/a/b.json', 'fixtures/**') and not reads.root_matches('fixtures', 'fixtures/**/*.json')
    assert reads.root_matches('core/x.py', 'core/*.py') and not reads.root_matches('core/a/x.py', 'core/*.py')


def test_declared_reads_stay_current():
    """A test's `CI_READS` stands in for scans the selector cannot read; each entry must stay usable and
    cover every scan the test makes."""
    reads = ci.python_test_reads
    assert reads.declaration_problems(PROJECT) == []
    assert reads.declared_reads(PROJECT)['tests/test_model_document_schema.py'] == ('**/*.json', '**/*.jsonl')
    assert 'tests/test_model_document_schema.py' in selected('package.json')
    assert 'tests/test_model_document_schema.py' not in by_rules('docs/user_guide/index.md')


def test_the_runner_wrappers_collect_every_runner_test_class():
    """Each runner suite left unrouted is collected whole by its tests/ wrapper (review of #28, finding 4: the K6
    wrapper once omitted two classes)."""
    import ast
    import sys
    import unittest
    wrappers = {'core/solver/performance_harness/runner/test_k6_runner.py': 'tests/test_performance_harness_runner.py',
                'core/solver/performance_harness/runner/test_release_runner.py': 'tests/test_release_timing_runner.py'}
    assert set(wrappers) == set(ci.UNROUTED)
    for source, wrapper_path in wrappers.items():
        classes = {node.name for node in ast.parse((PROJECT / source).read_text()).body if isinstance(node, ast.ClassDef)
                   and any(isinstance(base, ast.Attribute) and base.attr == 'TestCase' for base in node.bases)}
        if source.endswith('test_k6_runner.py'):
            assert {'W1ContextBinding', 'ModelEligibilityBeforeLaunch'} <= classes
        spec = importlib.util.spec_from_file_location('wrapper_' + Path(wrapper_path).stem, PROJECT / wrapper_path)
        wrapper = importlib.util.module_from_spec(spec)
        path = list(sys.path)
        try:
            spec.loader.exec_module(wrapper)
        finally:
            sys.path[:] = path
        assert {name for name, value in vars(wrapper).items()
                if isinstance(value, type) and issubclass(value, unittest.TestCase)} == classes, wrapper_path
        assert wrapper_path in selected(source), source


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
        assert test not in by_rules('core/units/schema_vocabulary.py'), test
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
    # Roots the Python job cannot read are never roots.
    assert not [root for roots in reads.values() for root in roots
                if root.startswith('execution/') and not root.startswith('execution/_DAG')]


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
    reads = ci.python_test_reads
    files = sorted(path for path in reads.tracked(PROJECT) if (PROJECT / path).is_file() and not reads._unavailable(path))
    for test, roots in ci.derived_tests(PROJECT).items():
        # A root under no known directory (a base the code does not show) spans the project and would route every
        # local change to the full suite; the advisory local registry leaves those to hosted selection.
        for root in [root for root in roots if reads._prefix(root)]:
            for path in [path for path in files if reads.root_matches(path, root)][:1]:
                checks = {check for rule in registry['path_rules'] if matches(path, rule['paths']) for check in rule['checks']}
                assert any(runs(check, test) for check in checks), (test, root, path)


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
    # The always-run security tests read their policy documents; the claims-language test reads every doc.
    assert hosted_ci.select_paths(['docs/security/telemetry_policy.md'], profile)['modes']['piping-python'] == 'full'
    assert hosted_ci.select_paths(['docs/PRD.md'], profile)['modes']['piping-python'] == 'full'
    # A test whose reads the code does not show reads every path, so a governance-only change starts the job too.
    assert hosted_ci.select_paths(['loop/LOOP_INIT.md'], profile)['modes']['piping-python'] == 'full'


def test_claims_language_surface_test_follows_the_validator_s_inputs():
    """The declared verification (DEL-04-08 VER-008) runs when any input of its validator changes: its
    CI_READS are the validator's own `input_patterns()`, less the execution/ surface the Python job lacks."""
    test = 'tests/test_claims_language_surface.py'
    assert test not in ci.UNROUTED
    import sys
    spec = importlib.util.spec_from_file_location('claims_inputs', PROJECT / 'tools/validation/validate_claims_language.py')
    validator = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = validator  # its dataclasses resolve their module
    try:
        spec.loader.exec_module(validator)
    finally:
        del sys.modules[spec.name]
    # fnmatch's `*` spans `/`: a trailing `/*` is the whole tree.
    expected = tuple(pattern[:-1] + '**' if pattern.endswith('/*') else pattern
                     for pattern in validator.input_patterns() if not pattern.startswith('execution/'))
    assert ci.python_test_reads.declared_reads(PROJECT)[test] == expected
    for path in ('docs/PRD.md', 'docs/user_guide/index.md', 'apps/desktop/src/App.tsx',
                 'core/reporting/report_renderer/src/lib.rs', 'tools/validation/validate_claims_language.py'):
        assert test in selected(path), path
    assert test in ci.select(PROJECT, {'event': 'workflow_dispatch'})
    assert test not in by_rules('loop/LOOP_INIT.md')


# Review of #28, round 2 (head 313a0f1): four false negatives. The deriver now fails toward over-selection.

def test_a_parameter_resolves_from_its_own_bindings_not_outer_ones():
    """Finding 1: `SCHEMAS / name` in `schema(name)` took unrelated outer bindings of `name`, so the file's roots
    named only those; a parameter keeps its own scope, and an unresolved one keeps its known prefix."""
    reads = ci.python_test_reads
    for test in ('tests/test_results_dispatcher_v0_3.py', 'tests/test_load_reference_schema.py'):
        roots = reads.file_reads(PROJECT, test)[0]
        assert any(reads.root_matches('schemas/a_schema_added_later.schema.json', root) for root in roots), test


def test_an_unknown_f_string_slot_may_span_directories():
    """Finding 2: a slot may hold `/`, so `pressure_load_state_{...}.analysis_run.json` matches across directories."""
    path = 'fixtures/results/pressure_load_state_case/one.analysis_run.json'
    assert 'tests/test_load_reference_headless_artifacts.py' in selected(path)
    reads = ci.python_test_reads
    assert reads.to_roots('fixtures/results/a_' + reads.SLOT + '.json') == {
        'fixtures/results/a_*.json', 'fixtures/results/a_*/**/*.json'}


def test_a_path_under_an_unresolved_base_keeps_its_known_suffix():
    """Finding 3: `snap / "core/.../section_basis.rs"` (a copy of the project) and the b7a census's owner sources."""
    assert 'tools/retained_pricing/tests/test_w15_rigid_scaling.py' in selected('core/reporting/result_export/src/section_basis.rs')
    assert 'tools/retained_pricing/tests/test_b7a_scoped_text_model.py' in selected('core/solver/frame_kernel/src/load_ledger.rs')
    reads = ci.python_test_reads
    assert reads.to_roots(reads.OUTSIDE + '/core/x.rs') == {'**/core/x.rs'}
    assert reads.to_roots(reads.OUTSIDE + '/' + reads.SLOT) == set()


def test_git_pathspecs_and_unshown_scans_are_derived_or_declared():
    """Finding 4: `git ls-files -- *.rs ...` is a scan of every Rust file; a scan or pathspec the code does not
    show needs CI_READS in the file that makes it."""
    assert 'tests/test_preview_identity_inventory.py' in selected('core/units/src/lib.rs')
    assert 'tests/test_preview_identity_inventory.py' in selected('core/a_crate_added_later/src/lib.rs')
    assert ci.python_test_reads.declaration_problems(PROJECT) == []
    declared = {local for test in ci.python_test_reads.maintained_tests(PROJECT)
                for local in ci.python_test_reads._closure(PROJECT, test)
                if ci.python_test_reads._reader(PROJECT, local).declared() is not None}
    assert {'tests/test_caepipe_correlation.py', 'tools/retained_pricing/price.py'} <= declared


# Review of #28, round 3 (head 870bbff).

def test_an_imported_helpers_input_reads_select_the_test():
    """Finding 1: the preview tables both tests open through imported helpers (traced: `rp._preview_statics()` and
    the u4b corpus's own table reads) select them."""
    tables = ('fixtures/results/semantic_contract_v0_3_preview_physics_2.json',
              'fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json',
              'fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json')
    for test in ('tests/test_retained_precision_b2.py', 'tests/test_u4b_retained_tau.py'):
        for path in tables:
            assert test in selected(path), (test, path)


def test_too_many_pattern_forms_collapse_to_a_covering_prefix():
    """Finding 2: `a{a}z/a{b}z/...` past the alternatives cap is every path under its known directories, never only
    the first forms."""
    reads = ci.python_test_reads
    text = 'fixtures/' + '/'.join('a' + reads.SLOT + 'z' for _ in range(5)) + '/x'
    roots = reads.to_roots(text)
    for path in ('fixtures/aqz/arz/asz/atz/auz/x', 'fixtures/aq/r/z/a1z/a2z/a3z/a4z/x', 'fixtures/a1z/a2z/a3z/a4z/aq/r/z/x'):
        assert any(reads.root_matches(path, root) for root in roots), path


def test_a_printf_specification_is_consumed_whole(tmp_path):
    """Finding 3: `%03d` is one unknown slot, not `%0` followed by a literal `3d`."""
    reads = ci.python_test_reads
    (tmp_path / 'm.py').write_text('from pathlib import Path\nPath("fixtures/case_%03d.json" % 7).read_text()\n')
    roots = {root for text in reads._Reader(tmp_path, 'm.py').chain_paths() for root in reads.to_roots(text)}
    assert any(reads.root_matches('fixtures/case_007.json', root) for root in roots), roots
