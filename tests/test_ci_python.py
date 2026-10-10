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
