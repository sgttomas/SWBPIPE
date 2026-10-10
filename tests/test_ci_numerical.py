"""Numerical CI contracts, without executing Cargo or browser tools."""
import importlib.util
import json
import re
from pathlib import Path
import subprocess
import tempfile
import sys
import unittest
from unittest.mock import patch

MODULE = Path(__file__).resolve().parents[1] / 'tools/ci/numerical_ci.py'
spec = importlib.util.spec_from_file_location('numerical_ci', MODULE)
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)

ROOT = MODULE.parents[4]
sys.path.insert(0, str(ROOT / 'tools/software_workflow'))
import hosted_ci
from software_workflow_common import load_profile


def numerical_input(path):
    return hosted_ci.select_paths([path], load_profile(ROOT / 'tools/hosted-ci-routing.json')[1])['modes']['piping-numerical'] == 'full'


class NumericalTests(unittest.TestCase):
    def fixture(self, root):
        crate = root / 'core/new_crate'
        crate.mkdir(parents=True)
        (crate / 'Cargo.toml').write_text('[package]\nname="failure_control"\nversion="0.1.0"\n')
        (crate / 'Cargo.lock').write_text('version = 4\n')
        return crate

    def test_discovery_requires_nonempty_manifests_and_locks(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            with self.assertRaisesRegex(ValueError, 'No cargo'): ci.cargo_plan(root)
            crate = self.fixture(root)
            manifests, commands = ci.cargo_plan(root)
            self.assertEqual(manifests, [Path('core/new_crate/Cargo.toml')])
            self.assertEqual(commands[0], ['cargo', 'fetch', '--locked', '--manifest-path', manifests[0].as_posix()])
            self.assertEqual(commands[1], ['cargo', 'test', '--offline', '--manifest-path', manifests[0].as_posix(), '--locked'])
            for file in ['Cargo.toml', 'Cargo.lock']:
                path = crate / file
                original = path.read_text()
                path.write_text('')
                with self.assertRaisesRegex(ValueError, 'empty cargo input'): ci.cargo_plan(root)
                path.write_text(original)
            (crate / 'Cargo.lock').unlink()
            with self.assertRaises(ValueError): ci.cargo_plan(root)

    def test_all_fetches_precede_tests(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            self.fixture(project)
            second = project / 'validation/benchmarks/new_benchmark'
            second.mkdir(parents=True)
            (second / 'Cargo.toml').write_text('[package]\nname="benchmark"\nversion="0.1.0"\n')
            (second / 'Cargo.lock').write_text('version = 4\n')
            manifests, commands = ci.cargo_plan(project)
            self.assertEqual(len(manifests), 2)
            self.assertTrue(all(c[:2] == ['cargo', 'fetch'] and '--locked' in c for c in commands[:len(manifests)]))
            self.assertTrue(all(c[:2] == ['cargo', 'test'] and '--locked' in c and '--offline' in c for c in commands[len(manifests):]))

    def test_frame_kernel_preserves_all_test_classes_and_integration_profile(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            source = self.fixture(project)
            destination = project / 'core/solver/frame_kernel'
            destination.parent.mkdir(parents=True)
            source.rename(destination)
            manifests, commands = ci.cargo_plan(project)
            tests = commands[len(manifests):]
            self.assertEqual(len(tests), 3)
            self.assertIn('--lib', tests[0])
            self.assertIn('profile.test.opt-level=2', tests[0])
            self.assertIn('profile.test.debug-assertions=true', tests[0])
            self.assertIn('profile.test.overflow-checks=true', tests[0])
            self.assertIn('--test', tests[1])
            self.assertEqual(tests[1][tests[1].index('--test')+1], '*')
            self.assertNotIn('--config', tests[1])
            self.assertIn('--bins', tests[1])
            self.assertIn('--examples', tests[1])
            self.assertIn('--doc', tests[2])
            self.assertNotIn('--config', tests[2])

    def test_subprocess_failure_is_recorded_and_stops(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = {'commands': []}
            with patch.object(ci.subprocess, 'run', return_value=subprocess.CompletedProcess([], 101)) as run:
                code = ci.execute_commands(Path(tmp), [['cargo', 'test'], ['must-not-run']], evidence, Path(tmp))
            self.assertEqual(code, 101)
            self.assertEqual(run.call_count, 1)
            self.assertEqual(evidence['commands'][0]['exit_code'], 101)
            self.assertTrue((Path(tmp) / evidence['commands'][0]['output']).exists())

    def test_invalid_candidate_still_writes_failure_evidence(self):
        with tempfile.TemporaryDirectory() as tmp, patch.object(ci, 'validate_plan', side_effect=ValueError('wrong candidate')):
            self.assertEqual(ci.run(tmp, {'head': 'wrong'}, tmp), 1)
            evidence = json.loads((Path(tmp) / 'numerical.json').read_text())
            self.assertFalse(evidence['plan_validated'])
            self.assertEqual(evidence['status'], 'failed')
            self.assertEqual(evidence['commands'], [])

    def test_cargo_launch_exception_after_version_success_is_nonzero(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self.fixture(root / ci.PROJECT)
            with patch.object(ci, 'validate_plan'), patch.object(ci, 'execute_commands', return_value=0), patch.object(ci, 'execute_cargo', side_effect=OSError('launch failed')):
                self.assertEqual(ci.run(root, {'numerical_required': True}, root / 'evidence'), 1)
            evidence = json.loads((root / 'evidence/numerical.json').read_text())
            self.assertEqual(evidence['exit_code'], 1)
            self.assertEqual(evidence['status'], 'failed')

    def rust_sources(self):
        project = MODULE.parents[2]
        sources = sorted(path for search in ci.readiness.CARGO_SEARCH_ROOTS
                         for path in (project / search).rglob('*.rs')
                         if 'target' not in path.relative_to(project).parts)
        self.assertTrue(sources, 'Real Rust source discovery must be nonempty')
        return project, sources

    def rust_path_literal(self, text):
        # Rust removes escaped newline and following whitespace. JSON handles
        # the remaining ordinary path escapes and rejects unsupported ones.
        text = re.sub(r'\\\r?\n\s*', '', text)
        try:
            return json.loads('"' + text + '"')
        except ValueError as exc:
            self.fail(f'Unsupported Rust path literal {text!r}: {exc}')

    def test_real_rust_literal_includes_require_numerical(self):
        project, sources = self.rust_sources()
        repo = project.parents[1]
        references = set()
        literal = re.compile(r'\s*"((?:\\[\s\S]|[^"\\])*)"\s*,?\s*\)')
        for source in sources:
            body = source.read_text()
            for include in re.finditer(r'\binclude_(?:str|bytes)!\s*\(', body):
                match = literal.match(body, include.end())
                self.assertIsNotNone(match, f'Unsupported include syntax in {source}: {body[include.start():include.start()+160]}')
                path = (source.parent / self.rust_path_literal(match.group(1))).resolve()
                self.assertTrue(path.is_file(), f'Unresolved Rust include: {source} -> {path}')
                relative = path.relative_to(repo).as_posix()
                self.assertTrue(numerical_input(relative), f'Rust include omitted by numerical policy: {source} -> {relative}')
                references.add(relative)
        self.assertTrue(references, 'Real include coverage must be nonempty')
        self.assertIn(ci.PROJECT + 'validation/evidence/comparison_measurement/DEL0904_VD_20260811/CURRENT_25_FIXTURE_RUNNER_OUTPUT.json', references)

    def test_real_rust_runtime_resource_literals_require_numerical(self):
        project, sources = self.rust_sources()
        repo = project.parents[1]
        references = set()
        for source in sources:
            for match in re.finditer(r'"((?:\\[\s\S]|[^"\\])*)"', source.read_text()):
                raw = match.group(1)
                if not re.match(r'^(?:\.\./)*(?:fixtures|schemas|examples|validation)/', raw):
                    continue
                value = self.rust_path_literal(raw)
                # Runtime paths use either the project root, source directory,
                # or CARGO_MANIFEST_DIR; inspect only real contained targets.
                crate = next(parent for parent in source.parents if (parent / 'Cargo.toml').is_file())
                candidates = {base.joinpath(value).resolve() for base in (project, source.parent, crate)}
                for path in candidates:
                    if not path.is_relative_to(project) or not path.exists():
                        continue
                    relative = path.relative_to(repo).as_posix()
                    self.assertTrue(numerical_input(relative), f'Runtime resource omitted by numerical policy: {source} -> {relative}')
                    references.add(relative)
        self.assertTrue(references, 'Real runtime resource coverage must be nonempty')
        for prefix in ('fixtures/', 'schemas/', 'examples/', 'validation/'):
            self.assertTrue(any(path.startswith(ci.PROJECT + prefix) for path in references), prefix)

    def test_gate_requires_exact_numerical_state(self):
        for mode, expected in [('full', 'success'), ('not-applicable', 'skipped')]:
            self.assertTrue(hosted_ci.aggregate('piping-numerical', mode, 'success', expected))
            for state in {'failure', 'cancelled', '', 'unknown', 'skipped', 'success'} - {expected}:
                self.assertFalse(hosted_ci.aggregate('piping-numerical', mode, 'success', state))
        for mode in ['', 'unknown', None]:
            self.assertFalse(hosted_ci.aggregate('piping-numerical', mode, 'success', 'skipped'))

    def test_plan_must_name_exact_selected_candidate(self):
        plan = {'schema': 'chirality-hosted-ci/v1', 'head': 'abc', 'modes': {'piping-numerical': 'full'}}
        with patch.object(ci.subprocess, 'check_output', return_value='abc\n'):
            ci.validate_plan(Path('.'), plan)
            with self.assertRaises(ValueError):
                ci.validate_plan(Path('.'), dict(plan, head='other'))
            with self.assertRaises(ValueError):
                ci.validate_plan(Path('.'), dict(plan, modes={'piping-numerical': 'not-applicable'}))


if __name__ == '__main__':
    unittest.main()
