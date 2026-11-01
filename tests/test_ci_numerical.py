"""Numerical CI contracts, without executing Cargo or browser tools."""
import errno
import importlib.util
import io
from functools import cache
import hashlib
import json
import re
from pathlib import Path
import subprocess
import tempfile
import sys
import unittest
from unittest.mock import patch, Mock

import pytest

from tests.public_export import PUBLIC_EXPORT

if PUBLIC_EXPORT:
    # The hosted router lives at the repository root, outside the export.
    pytest.skip("public export: reads the repository's tools/software_workflow, which the export leaves out",
                allow_module_level=True)

MODULE = Path(__file__).resolve().parents[1] / 'tools/ci/numerical_ci.py'
spec = importlib.util.spec_from_file_location('numerical_ci', MODULE)
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)

ROOT = MODULE.parents[2]
sys.path.insert(0, str(ROOT / 'tools/software_workflow'))
import hosted_ci
from software_workflow_common import load_profile


@cache
def real_graph():
    project = MODULE.parents[2]
    manifests = ci.readiness.discover_cargo_manifests(project)
    return ci.source_read_dependencies(project, manifests), ci.cargo_dependencies(project, manifests)


EMPTY_READS = {'source_trees': {}, 'reads': {}}


def wrapped_reads(reads):
    trees = {}
    for entry in reads.values():
        trees.update(entry['source_trees'])
        entry['source_trees'] = sorted(entry['source_trees'])
    return {'source_trees': trees, 'reads': reads}


def numerical_input(path):
    # These exhaustive existing assertions inspect one immutable repository
    # snapshot. Reuse its verified graph, while exercising routing for each path.
    resources, dependencies = real_graph()
    with patch.object(hosted_ci, 'numerical_module', return_value=ci), \
         patch.object(ci, 'source_read_dependencies', return_value=resources), \
         patch.object(ci, 'cargo_dependencies', return_value=dependencies):
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

    def assert_partition(self, project, selection):
        from collections import Counter
        manifests, commands = ci.cargo_plan(project, selection=selection)
        partitions = [ci.cargo_plan(project, selection=selection, shard=shard) for shard in ci.SHARDS]
        self.assertEqual(Counter(m for ms, _ in partitions for m in ms), Counter(manifests))
        self.assertEqual(Counter(tuple(c) for _, cs in partitions for c in cs), Counter(tuple(c) for c in commands))
        for shard, (members, actual) in zip(ci.SHARDS, partitions):
            expected = [c for c in commands if ci.manifest_shard(c[c.index('--manifest-path') + 1]) == shard]
            self.assertEqual(actual, expected)
            self.assertTrue(all(ci.manifest_shard(m) == shard for m in members))

    def test_real_plan_partitions_preserve_every_command_and_witness(self):
        manifests = ci.readiness.discover_cargo_manifests(ROOT)
        # Full coverage and each possible singleton reduced selection, with
        # actual discovery, readiness commands and all profile special cases.
        for members in [manifests, [], *[[m] for m in manifests]]:
            self.assert_partition(ROOT, {'manifests': [str(m) for m in members]})

    def test_synthetic_partition_is_total_disjoint_and_unknown_crates_run(self):
        from itertools import combinations
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            names = ['core/product_physics', 'core/reporting/result_export',
                     'core/runner/headless', 'core/new_crate']
            for name in names:
                crate = root / name
                crate.mkdir(parents=True)
                (crate / 'Cargo.toml').write_text('[package]\nname="fixture"\nversion="0.1.0"\n')
                (crate / 'Cargo.lock').write_text('version = 4\n')
            for count in range(len(names) + 1):
                for subset in combinations(names, count):
                    self.assert_partition(root, {'manifests': [n + '/Cargo.toml' for n in subset]})
            self.assertEqual(ci.manifest_shard('core/new_crate/Cargo.toml'), 'other')
            with self.assertRaisesRegex(ValueError, 'Unknown numerical shard'):
                ci.cargo_plan(root, shard='typo')
            # Filtering must not hide an invalid selection in another shard.
            with self.assertRaisesRegex(ValueError, 'duplicate or unknown'):
                ci.cargo_plan(root, selection={'manifests': ['missing/Cargo.toml']}, shard='physics')

    def test_empty_shard_validates_and_records_without_invoking_cargo(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self.fixture(root)
            plan = {'piping_numerical_selection': {'manifests': ['core/new_crate/Cargo.toml']}}
            with patch.object(ci, 'validate_plan') as validate, patch.object(ci, 'execute_commands') as execute, patch.object(ci, 'execute_cargo') as cargo:
                self.assertEqual(ci.run(root, plan, root / 'evidence', shard='physics'), 0)
                validate.assert_called_once_with(root.resolve(), plan)
                execute.assert_not_called()
                cargo.assert_not_called()
            evidence = json.loads((root / 'evidence/numerical.json').read_text())
            self.assertEqual((evidence['shard'], evidence['status'], evidence['empty']), ('physics', 'success', True))
            self.assertTrue(evidence['plan_validated'])
            self.assertEqual(evidence['planned_commands'], [])
            with patch.object(ci, 'validate_plan', side_effect=ValueError('wrong candidate')):
                self.assertEqual(ci.run(root, plan, root / 'evidence', shard='physics'), 1)

    def test_dry_run_records_commands_without_claiming_verification(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self.fixture(root)
            plan = {'piping_numerical_selection': {'manifests': ['core/new_crate/Cargo.toml']}}
            with patch.object(ci, 'validate_plan'), patch.object(ci, 'execute_commands') as execute:
                self.assertEqual(ci.run(root, plan, root / 'evidence', shard='other', dry_run=True), 0)
                execute.assert_not_called()
            evidence = json.loads((root / 'evidence/numerical.json').read_text())
            self.assertEqual(evidence['status'], 'planned')
            self.assertEqual(len(evidence['planned_commands']), 2)
            self.assertEqual(evidence['commands'], [])

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

    def test_result_export_keeps_every_target_with_optimized_safety_checks(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            source = self.fixture(project)
            destination = project / 'core/reporting/result_export'
            destination.parent.mkdir(parents=True)
            source.rename(destination)
            manifests, commands = ci.cargo_plan(project)
            tests = commands[len(manifests):]
            self.assertEqual(len(tests), 1)
            self.assertIn('profile.test.opt-level=2', tests[0])
            self.assertIn('profile.test.debug-assertions=true', tests[0])
            self.assertIn('profile.test.overflow-checks=true', tests[0])
            self.assertFalse(any(flag in tests[0] for flag in ('--lib', '--test', '--skip')))

    def test_product_physics_keeps_every_target_with_optimized_safety_checks(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            source = self.fixture(project)
            destination = project / 'core/product_physics'
            destination.parent.mkdir(parents=True, exist_ok=True)
            source.rename(destination)
            manifests, commands = ci.cargo_plan(project)
            tests = commands[len(manifests):]
            self.assertEqual(len(tests), 7)
            self.assertIn('profile.test.opt-level=2', tests[0])
            self.assertIn('profile.test.debug-assertions=true', tests[0])
            self.assertIn('profile.test.overflow-checks=true', tests[0])
            self.assertFalse(any(flag in tests[0] for flag in ('--lib', '--test', '--skip', '--ignored', '--include-ignored')))
            # b7-plan 5a: each stack witness alone, in the default debug profile.
            witnesses = tests[1:]
            self.assertEqual([w[w.index('--') + 1] for w in witnesses],
                             ['retained_memory::witness_tests::%s::%s' % (w, m)
                              for w in ('witness_headroom_w1_at_one_mebibyte', 'witness_w2_deep_milestone_publishes', 'witness_w_c2_publishes')
                              for m in ('sparse', 'dense')])
            for witness in witnesses:
                harness = witness.index('--')
                self.assertEqual(witness[:2], ['cargo', 'test'])
                self.assertNotIn('--config', witness)
                self.assertIn('--locked', witness[:harness])
                self.assertIn('--lib', witness[:harness])
                self.assertEqual(witness[harness + 2:], ['--exact', '--ignored', '--test-threads=1'])
            build = [b for b in ci.build_commands(commands) if b[:2] == ['cargo', 'test']]
            self.assertEqual(build, [tests[0] + ['--no-run'], witnesses[0][:witnesses[0].index('--')] + ['--no-run']])

    def test_build_only_retains_profiles_without_executing_tests_or_doctests(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            source = self.fixture(project)
            destination = project / 'core/solver/frame_kernel'
            destination.parent.mkdir(parents=True)
            source.rename(destination)
            _, commands = ci.cargo_plan(project)
            build = ci.build_commands(commands)
            self.assertEqual(build[0], commands[0])
            self.assertEqual(len(build), 3)
            self.assertIn('profile.test.opt-level=2', build[1])
            self.assertNotIn('--config', build[2])
            self.assertTrue(all('--no-run' in argv for argv in build[1:]))
            self.assertTrue(all('--doc' not in argv for argv in build))

    def test_cargo_timing_separates_compile_test_and_ignores_ansi(self):
        timings = ci.cargo_timings('\x1b[32mFinished `test` profile [unoptimized] target(s) in 1m 02.5s\x1b[0m\n'
            'test result: ok. 3 passed; finished in 9.75s\n'
            'test result: ok. 2 passed; finished in 1.25s\n')
        self.assertEqual(timings, {'compile_seconds': 62.5, 'test_seconds': 11.0})

    def test_build_only_uses_bound_selection_and_preserves_result_export_profile(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            project = root / ci.PROJECT
            source = self.fixture(project)
            destination = project / 'core/reporting/result_export'
            destination.parent.mkdir(parents=True)
            source.rename(destination)
            self.fixture(project)
            selected = {'manifests': ['core/reporting/result_export/Cargo.toml']}
            plan = {'piping_numerical_selection': selected, 'event': 'pull_request',
                    'paths': [ci.PROJECT + 'core/new_crate/src/lib.rs']}
            def compiled(project, commands, evidence, evidence_dir, env):
                evidence['commands'].append({'compile_seconds': 12.0, 'test_seconds': 0.0, 'elapsed_seconds': 14.0})
                return 0
            with patch.object(ci, 'validate_plan'), patch.object(ci, 'execute_commands', return_value=0), \
                 patch.object(ci, 'execute_cargo', side_effect=compiled) as cargo:
                self.assertEqual(ci.run(root, plan, root / 'evidence', build_only=True), 0)
            commands = cargo.call_args.args[1]
            self.assertEqual(len(commands), 2)
            self.assertEqual(commands[0][:2], ['cargo', 'fetch'])
            self.assertIn('--no-run', commands[1])
            for setting in ('profile.test.opt-level=2', 'profile.test.debug-assertions=true', 'profile.test.overflow-checks=true'):
                self.assertIn(setting, commands[1])
            evidence = json.loads((root / 'evidence/numerical.json').read_text())
            self.assertEqual(evidence['manifests'], selected['manifests'])
            self.assertEqual(evidence['execution'], 'build-only')
            self.assertEqual(evidence['timing'], {'compile_seconds': 12.0, 'test_seconds': 0.0, 'elapsed_seconds': 14.0})

    def test_command_keeps_completed_wall_timing_and_cargo_compile_test_fields(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = {'commands': []}
            def completed(argv, **kwargs):
                kwargs['stdout'].write('Finished `test` profile target(s) in 2.0s\n'
                    'test result: ok. 1 passed; finished in 1.25s\n')
                return subprocess.CompletedProcess(argv, 0)
            output = io.StringIO()
            with patch.object(ci.subprocess, 'run', side_effect=completed), \
                 patch.object(ci.time, 'perf_counter', side_effect=[10.0, 15.0]), \
                 patch('sys.stdout', output):
                self.assertEqual(ci.execute_commands(Path(tmp), [['cargo', 'test']], evidence, Path(tmp)), 0)
            row = evidence['commands'][0]
            self.assertEqual(row['elapsed_seconds'], 5.0)
            self.assertEqual(row['compile_seconds'], 2.0)
            self.assertEqual(row['test_seconds'], 1.25)
            self.assertIn('Completed in 5.000s (exit 0): cargo test', output.getvalue())


    def test_subprocess_failure_is_recorded_and_stops(self):
        with tempfile.TemporaryDirectory() as tmp:
            evidence = {'commands': []}
            with patch.object(ci.subprocess, 'run', return_value=subprocess.CompletedProcess([], 101)) as run:
                code = ci.execute_commands(Path(tmp), [['cargo', 'test'], ['must-not-run']], evidence, Path(tmp))
            self.assertEqual(code, 101)
            self.assertEqual(run.call_count, 1)
            self.assertEqual(evidence['commands'][0]['exit_code'], 101)
            self.assertGreaterEqual(evidence['commands'][0]['elapsed_seconds'], 0)
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
                self.assertEqual(ci.run(root, {'piping_numerical_selection': {'manifests': ['core/new_crate/Cargo.toml']}}, root / 'evidence'), 1)
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
        repo = project
        references = set()
        literal = re.compile(r'\s*"((?:\\[\s\S]|[^"\\])*)"\s*,?\s*\)')
        for source in sources:
            # As the selector does: comments removed and string literals blanked,
            # so include text quoted inside a test string is not a real include.
            body = ci.rust_source(source.read_text())
            syntax = ci.RUST_STRING.sub(lambda m: ' ' * len(m.group()), body)
            for include in re.finditer(r'\binclude_(?:str|bytes)\s*!\s*\(', syntax):
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
        repo = project
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
                    if not path.is_relative_to(project):
                        continue
                    # Every literal with a resource prefix is probed, prose included: a
                    # provenance sentence such as connector_readers.rs's corpus `inputs`
                    # (written into a committed corpus) has a path component longer than
                    # the filesystem allows, and `exists()` raises ENAMETOOLONG instead of
                    # answering. A name the filesystem cannot hold is no runtime resource;
                    # any other error is raised, never read as an absent resource.
                    try:
                        present = path.exists()
                    except OSError as error:
                        if error.errno != errno.ENAMETOOLONG:
                            raise
                        present = False
                    if not present:
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
        with patch.object(ci.subprocess, 'check_output', return_value='abc\n'), patch.object(ci, 'load_module', return_value=Mock()):
            ci.validate_plan(Path('.'), plan)
            with self.assertRaises(ValueError):
                ci.validate_plan(Path('.'), dict(plan, head='other'))
            with self.assertRaises(ValueError):
                ci.validate_plan(Path('.'), dict(plan, modes={'piping-numerical': 'not-applicable'}))


if __name__ == '__main__':
    unittest.main()


class AffectedCratesTests(unittest.TestCase):
    def test_changed_dependency_selects_transitive_consumers_not_unrelated_crates(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifests = []
            for name, supplier in [('a', None), ('b', 'a'), ('c', 'b'), ('unrelated', None)]:
                manifest = Path('core') / name / 'Cargo.toml'
                (root / manifest).parent.mkdir(parents=True)
                text = f'[package]\nname="{name}"\nversion="0.1.0"\n'
                if supplier:
                    text += f'[dependencies]\n{supplier}={{path="../{supplier}"}}\n'
                (root / manifest).write_text(text)
                (root / manifest.with_name('Cargo.lock')).write_text('version = 4\n')
                manifests.append(manifest)
            chosen = ci.affected_manifests(root, manifests, [ci.PROJECT + 'core/a/src/lib.rs'])
            self.assertEqual(chosen, manifests[:3])
            self.assertEqual(ci.affected_manifests(root, manifests, [ci.PROJECT + 'core/c/src/lib.rs']), [manifests[2]])
            self.assertEqual(ci.affected_manifests(root, manifests, [ci.PROJECT + 'tests/test_schema.py']), [])
            self.assertEqual(ci.affected_manifests(root, manifests, ['tools/shared.py']), manifests)
            self.assertEqual(ci.affected_manifests(root, manifests, [ci.PROJECT + 'core/deleted/lib.rs']), manifests)


class SourceReadTests(unittest.TestCase):
    def test_cross_crate_literals_add_edges_without_cargo_dependencies(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            manifests = []
            for name in ['supplier', 'reader', 'consumer', 'unrelated']:
                manifest = Path('core') / name / 'Cargo.toml'
                (project / manifest).parent.mkdir(parents=True)
                text = f'[package]\nname="{name}"\nversion="0.1.0"\n'
                if name == 'consumer':
                    text += '[dependencies]\nreader={path="../reader"}\n'
                (project / manifest).write_text(text)
                (project / manifest.with_name('Cargo.lock')).write_text('version = 4\n')
                manifests.append(manifest)
            source = project / 'core/reader/tests/read.rs'
            source.parent.mkdir()
            supplier_source = project / 'core/supplier/src/lib.rs'
            supplier_source.parent.mkdir()
            supplier_source.write_text('pub fn value() {}')
            fixture = project / 'core/supplier/fixture.json'
            fixture.write_text('{}')
            source.write_text('include_str!("../../supplier/src/lib.rs")')
            self.assertEqual(ci.affected_manifests(project, manifests,
                [ci.PROJECT + 'core/supplier/src/lib.rs']), sorted(manifests[:3]))
            source.write_text('let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../supplier/fixture.json"); fs::read_to_string(path);')
            declaration = {source.relative_to(project).as_posix(): {
                'sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
                'source_trees': {manifests[1].as_posix(): ci.rust_tree_digest(project, manifests[1])},
                'inputs': [{'anchor': '../supplier/fixture.json', 'base': 'manifest',
                            'files': ['core/supplier/fixture.json']}]}}
            declaration = wrapped_reads(declaration)
            self.assertEqual(ci.affected_manifests(project, manifests,
                [ci.PROJECT + 'core/supplier/fixture.json'], declaration), sorted(manifests[1:3]))
            # A fixture read does not make unrelated supplier Rust source an API.
            self.assertEqual(ci.affected_manifests(project, manifests,
                [ci.PROJECT + 'core/supplier/src/lib.rs'], declaration), [manifests[0]])

    def test_repository_cross_crate_examples_select_readers(self):
        project = MODULE.parents[2]
        manifests = ci.readiness.discover_cargo_manifests(project)
        cases = [
            ('core/solver/nonlinear_integration/src/lib.rs', 'core/solver/frame_kernel/Cargo.toml'),
            ('core/solver/straight_pipe/src/lib.rs', 'core/solver/frame_kernel/Cargo.toml'),
            ('core/solver/curved_bend/src/lib.rs', 'core/solver/frame_kernel/Cargo.toml'),
            ('core/loads/load_case_algebra/src/lib.rs', 'core/solver/frame_kernel/Cargo.toml'),
            ('core/reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json', 'core/product_physics/Cargo.toml'),
            ('core/product_physics/tests/fixtures/exact_pressure_connected_request.json', 'core/reporting/result_export/Cargo.toml'),
            ('core/loads/self_weight_wasm/tests/fixtures/applied_self_weight_v1.json', 'core/model_operations/operation_applier/Cargo.toml'),
        ]
        for changed, reader in cases:
            with self.subTest(changed=changed):
                self.assertIn(Path(reader), ci.affected_manifests(project, manifests, [ci.PROJECT + changed]))



class ReachabilitySafetyTests(unittest.TestCase):
    def crate(self, project, name, dependencies=''):
        manifest = Path('core') / name / 'Cargo.toml'
        (project / manifest).parent.mkdir(parents=True)
        (project / manifest).write_text(f'[package]\nname="{name}"\nversion="0.1.0"\n' + dependencies)
        (project / manifest.with_name('Cargo.lock')).write_text('version = 4\n')
        (project / manifest.parent / 'src').mkdir()
        (project / manifest.parent / 'src/lib.rs').write_text('pub fn value() {}')
        return manifest

    def test_dev_build_target_and_workspace_dependencies_are_transitive(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            a = self.crate(project, 'a')
            b = self.crate(project, 'b', '[dev-dependencies]\na={path="../a"}\n')
            c = self.crate(project, 'c', '[build-dependencies]\nb={path="../b"}\n')
            d = self.crate(project, 'd', '[target.\'cfg(windows)\'.dependencies]\nc={path="../c"}\n')
            e = self.crate(project, 'e', '[dependencies]\nd={workspace=true}\nserde={workspace=true}\n')
            unrelated = self.crate(project, 'unrelated')
            (project / 'Cargo.toml').write_text('[workspace]\nmembers=["core/e"]\n[workspace.dependencies]\nd={path="core/d"}\nserde="1"\n')
            manifests = [a, b, c, d, e, unrelated]
            self.assertEqual(ci.affected_manifests(project, manifests, [ci.PROJECT + 'core/a/src/lib.rs'], EMPTY_READS), manifests[:5])

    def test_missing_or_uncontained_local_dependencies_and_inputs_select_full(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            a = self.crate(project, 'a')
            b = self.crate(project, 'b')
            for spec in ['missing={path="../missing"}', 'external={path="../../../outside"}', 'missing={workspace=true}']:
                (project / b).write_text('[package]\nname="b"\nversion="0.1.0"\n[dependencies]\n' + spec)
                result = ci.numerical_selection(project, [a, b], [ci.PROJECT + 'core/a/src/lib.rs'], EMPTY_READS)
                self.assertEqual(result['classification'], 'full-fallback')
                self.assertEqual(result['manifests'], [a.as_posix(), b.as_posix()])
            (project / b).write_text('[package]\nname="b"\nversion="0.1.0"\n')
            (project / b.with_name('Cargo.lock')).unlink()
            result = ci.numerical_selection(project, [a, b], [ci.PROJECT + 'core/a/src/lib.rs'], EMPTY_READS)
            self.assertIn('Missing or empty cargo input', result['fallback_reasons'][0])

    def test_literal_compile_reads_are_exact_and_ignore_comments(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            supplier = self.crate(project, 'supplier')
            reader = self.crate(project, 'reader')
            unrelated = self.crate(project, 'unrelated')
            fixture = project / 'fixtures/case.json'
            fixture.parent.mkdir()
            fixture.write_text('{}')
            helper = project / 'fixtures/shared.rs'
            helper.write_text('pub fn shared() {}')
            source = project / reader.parent / 'src/lib.rs'
            source.write_text('// include_str!("missing.json")\n/* nested /* include!("missing.rs") */ comment */\nconst X: &[u8] = include_bytes!(r##"../../../fixtures/case.json"##);\n#[path = "../../../fixtures/shared.rs"] mod shared;\n')
            resources = ci.source_read_dependencies(project, [supplier, reader, unrelated], EMPTY_READS)
            self.assertEqual(resources['fixtures/case.json'], {reader})
            self.assertEqual(resources['fixtures/shared.rs'], {reader})
            self.assertEqual(ci.affected_manifests(project, [supplier, reader, unrelated], [ci.PROJECT + 'fixtures/case.json'], EMPTY_READS), [reader])
            self.assertEqual(ci.rust_literal('"a\\\n  b\\u{2f}c"'), 'ab/c')
            helper.write_text('mod indirect;')
            result = ci.numerical_selection(project, [supplier, reader, unrelated], [ci.PROJECT + 'fixtures/case.json'], EMPTY_READS)
            self.assertIn('external Rust module tree', result['fallback_reasons'][0])
            helper.write_text('pub fn shared() {}')
            fixture.unlink()
            result = ci.numerical_selection(project, [supplier, reader, unrelated], [ci.PROJECT + 'fixtures/case.json'], EMPTY_READS)
            self.assertEqual(result['classification'], 'full-fallback')
            self.assertIn('Unresolved Rust read', result['fallback_reasons'][0])

    def test_dynamic_read_requires_source_grounded_declaration_and_caller_boundaries(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            reader = self.crate(project, 'reader')
            consumer = self.crate(project, 'consumer', '[dependencies]\nreader={path="../reader"}\n')
            unrelated = self.crate(project, 'unrelated')
            source = project / reader.parent / 'src/lib.rs'
            source.write_text('pub fn read(path: &str) { std::fs::read_to_string(path); }\nconst FIXTURE: &str = "../../../fixtures/shared.json";\n')
            fixture = project / 'fixtures/shared.json'
            fixture.parent.mkdir()
            fixture.write_text('{}')
            manifests = [reader, consumer, unrelated]
            path = ci.PROJECT + 'fixtures/shared.json'
            self.assertEqual(ci.numerical_selection(project, manifests, [path], EMPTY_READS)['classification'], 'full-fallback')
            declaration = {source.relative_to(project).as_posix(): {
                'sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
                'source_trees': {m.as_posix(): ci.rust_tree_digest(project, m) for m in [reader, consumer]},
                'inputs': [{'anchor': '../../../fixtures/shared.json', 'base': 'source', 'files': ['fixtures/shared.json']}]}}
            declaration = wrapped_reads(declaration)
            result = ci.numerical_selection(project, manifests, [path], declaration)
            self.assertEqual(result['classification'], 'reachability')
            self.assertEqual(result['manifests'], sorted([reader.as_posix(), consumer.as_posix()]))
            original = (project / consumer.parent / 'src/lib.rs').read_text()
            (project / consumer.parent / 'src/lib.rs').write_text(original + '\nfn new_caller() { reader::read("../../../fixtures/new.json"); }')
            result = ci.numerical_selection(project, manifests, [path], declaration)
            self.assertEqual(result['classification'], 'full-fallback')
            self.assertIn('caller boundaries', result['fallback_reasons'][0])
            (project / consumer.parent / 'src/lib.rs').write_text(original)
            declaration['reads'][source.relative_to(project).as_posix()]['inputs'][0]['anchor'] = 'absent-from-source.json'
            result = ci.numerical_selection(project, manifests, [path], declaration)
            self.assertIn('anchor absent', result['fallback_reasons'][0])

    def test_unsupported_compiled_and_aliased_runtime_reads_select_full(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            a = self.crate(project, 'a')
            b = self.crate(project, 'b')
            for code in ['include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/fixture.json"));',
                         'use std::fs::{read_to_string}; fn read() { read_to_string("fixture.json"); }',
                         'use std::fs as files; fn read() { files::read("fixture.json"); }']:
                (project / a.parent / 'src/lib.rs').write_text(code)
                self.assertEqual(ci.numerical_selection(project, [a, b], [ci.PROJECT + 'core/a/src/lib.rs'], EMPTY_READS)['classification'], 'full-fallback')

    def test_empty_diff_unknown_resource_tool_and_removed_crate_select_full(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            a = self.crate(project, 'a')
            b = self.crate(project, 'b')
            for paths in [None, [], [ci.PROJECT + 'fixtures/new.json'], [ci.PROJECT + 'tools/new.py'], [ci.PROJECT + 'core/removed/src/lib.rs']]:
                result = ci.numerical_selection(project, [a, b], paths, EMPTY_READS)
                self.assertEqual(result['classification'], 'full-fallback')
                self.assertTrue(result['fallback_reasons'])

    def test_actual_complete_desktop_pr_diffs_retain_every_reader_and_controls_select_all(self):
        profile = load_profile(ROOT / 'tools/hosted-ci-routing.json')[1]
        for base, head in [('a467a1f03799f12ebd02df82a0145cfc958f9dcb','c986759b21605293e29383dd2b660ab5acb7c3fb'),
                           ('2844f2b32d4b9f30effc41b4b387a302417697dc','b137b9c46a0a848a9284e5147391641845c83815'),
                           ('e2055cb35fe71af03caae2c386bd8055fb0afd81','a9524745a3227ff229e22bf6aba9b307e0382701')]:
            # Historical source identities survive in the extraction correspondence.
            mapping = dict(line.split(':') for line in (ROOT / 'reference/migration/commit-map.txt').read_text().splitlines() if line)
            plan = hosted_ci.make_plan(ROOT, profile, 'pull_request', mapping[base], mapping[head])
            selected = plan['piping_numerical_selection']
            self.assertEqual(selected['classification'], 'reachability', selected['fallback_reasons'])
            # #1246 also edits the desktop manifest read by VR's feature guard. Each count includes
            # DEL-04-05's release timing driver (core/solver/release_timing), which depends on
            # product_physics.
            expected = 12 if head == 'c986759b21605293e29383dd2b660ab5acb7c3fb' else 11
            self.assertEqual(len(selected['manifests']), expected)
            self.assertIn('core/reporting/result_export/Cargo.toml', selected['manifests'])
        for path in ['fixtures/unknown-new-input.json','tools/unknown-new-input.py']:
            selected = hosted_ci.select_paths([ci.PROJECT + path], profile)['piping_numerical_selection']
            self.assertEqual(selected['classification'], 'full-fallback')
            # 43 discovered crates since DEL-04-05's release timing driver (core/solver/release_timing)
            # joined the 42 of the MBF engine (core/handoff/caepipe_mbf_engine), which joined the 41 of
            # the pressure-source join, which added core/numerics/correct_norm.
            self.assertEqual(len(selected['manifests']), 43)
        # D8's one current corpus retains its exact reader reachability.
        selected = hosted_ci.select_paths([ci.PROJECT + 'fixtures/results/retained_precision_cases.json'], profile)['piping_numerical_selection']
        self.assertEqual(selected['classification'], 'reachability')
        # Ten readers, and DEL-04-05's release timing driver through product_physics.
        self.assertEqual(len(selected['manifests']), 11)
        self.assertIn('core/reporting/result_export/Cargo.toml', selected['manifests'])
        # The retired snapshot has no current reader edge; unknown reads stay full.
        retired = hosted_ci.select_paths([ci.PROJECT + 'fixtures/results/retained_precision_cases_07o.json'], profile)['piping_numerical_selection']
        self.assertEqual(retired['classification'], 'full-fallback')
        self.assertEqual(len(retired['manifests']), 43)  # every discovered crate

    def test_python_and_browser_checks_are_independent_of_cargo(self):
        profile = load_profile(ROOT / 'tools/hosted-ci-routing.json')[1]
        # A path starts the Python job when a Python test reads it (tools/ci/python_test_reads.py). The derivation
        # fails toward over-selection, and a test whose reads the code does not show reads every path, so every path
        # starts it; none of them selects Cargo.
        for path, piping, python in [('tests/test_persistence_schema.py', 'full', 'full'),
                                     ('apps/desktop/src/App.tsx', 'full', 'full'),
                                     ('docs/PRD.md', 'not-applicable', 'full'),
                                     ('loop/LOOP_INIT.md', 'not-applicable', 'full')]:
            result = hosted_ci.select_paths([ci.PROJECT + path], profile)
            self.assertEqual(result['modes']['piping'], piping)
            self.assertEqual(result['modes']['piping-python'], python)
            self.assertEqual(result['modes']['piping-numerical'], 'not-applicable')
            self.assertEqual(result['piping_numerical_selection']['manifests'], [])


    def test_known_fixture_keeps_second_reader_with_unmodeled_filesystem_imports(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            known = self.crate(project, 'known_reader')
            second = self.crate(project, 'second_reader')
            fixture = project / 'fixtures/known.json'
            fixture.parent.mkdir()
            fixture.write_text('{}')
            (project / known.parent / 'src/lib.rs').write_text('include_str!("../../../fixtures/known.json");')
            for body in [
                'use std::fs::{OpenOptions}; fn read() { OpenOptions::new().read(true).open("../../../fixtures/known.json"); }',
                'use std::fs::{read_to_string as text}; fn read() { text("../../../fixtures/known.json"); }',
                'use std::fs::{File as LocalFile}; fn read() { LocalFile::open("../../../fixtures/known.json"); }',
                'use std::fs::*; fn read() { read_to_string("../../../fixtures/known.json"); }',
                'use std::{fs as files}; fn read() { files::read("../../../fixtures/known.json"); }',
            ]:
                (project / second.parent / 'src/lib.rs').write_text(body)
                result = ci.numerical_selection(project, [known, second], [ci.PROJECT + 'fixtures/known.json'], EMPTY_READS)
                self.assertEqual(result['classification'], 'full-fallback', body)
                self.assertEqual(result['manifests'], [known.as_posix(), second.as_posix()])
                self.assertIn('filesystem', result['fallback_reasons'][0])


    def test_external_input_audits_require_explicit_shape_and_reason(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            reader = self.crate(project, 'reader')
            unrelated = self.crate(project, 'unrelated')
            source = project / reader.parent / 'src/lib.rs'
            source.write_text('fn main() { std::fs::read_to_string(std::env::args().nth(1).unwrap()); }')
            local = source.relative_to(project).as_posix()
            declaration = {'source_trees': {reader.as_posix(): ci.rust_tree_digest(project, reader)},
                           'reads': {local: {'sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
                               'source_trees': [reader.as_posix()], 'inputs': []}}}
            for audit in [None, True, 'arbitrary truthy string', {'kind': 'unknown', 'reason': 'claim'},
                          {'kind': 'audited-external-inputs', 'reason': True},
                          {'kind': 'audited-external-inputs', 'reason': ' '}]:
                declaration['reads'][local]['external_reads'] = audit
                result = ci.numerical_selection(project, [reader, unrelated], [ci.PROJECT + 'core/reader/src/lib.rs'], declaration)
                self.assertEqual(result['classification'], 'full-fallback')
                self.assertIn('Malformed external-input audit', result['fallback_reasons'][0])
            declaration['reads'][local]['external_reads'] = {'kind': 'audited-external-inputs',
                'reason': 'The executable reads only its user-supplied CLI argument; no repository fixture callers.'}
            self.assertEqual(ci.numerical_selection(project, [reader, unrelated],
                [ci.PROJECT + 'core/reader/src/lib.rs'], declaration)['classification'], 'reachability')

    def test_spaced_compile_macros_keep_second_reader_of_known_file(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            known = self.crate(project, 'known_reader')
            second = self.crate(project, 'second_reader')
            unrelated = self.crate(project, 'unrelated')
            fixture = project / 'fixtures/known.json'
            fixture.parent.mkdir()
            fixture.write_text('{}')
            shared = project / 'fixtures/shared.rs'
            shared.write_text('pub fn shared() {}')
            for expression, resource in [('include_str ! ("../../../fixtures/known.json");', 'fixtures/known.json'),
                                         ('include_bytes \n! ("../../../fixtures/known.json");', 'fixtures/known.json'),
                                         ('include ! ("../../../fixtures/shared.rs");', 'fixtures/shared.rs')]:
                (project / known.parent / 'src/lib.rs').write_text(f'include_str!("../../../{resource}");')
                (project / second.parent / 'src/lib.rs').write_text(expression)
                result = ci.numerical_selection(project, [known, second, unrelated], [ci.PROJECT + resource], EMPTY_READS)
                self.assertEqual(result['classification'], 'reachability', expression)
                self.assertEqual(result['manifests'], [known.as_posix(), second.as_posix()])
            (project / second.parent / 'src/lib.rs').write_text('include_str ! (concat!("../../../fixtures/known.json"));')
            result = ci.numerical_selection(project, [known, second, unrelated], [ci.PROJECT + 'fixtures/known.json'], EMPTY_READS)
            self.assertEqual(result['classification'], 'full-fallback')
            self.assertIn('Unresolved compiled Rust read', result['fallback_reasons'][0])

    def test_imported_compile_macro_alias_keeps_known_fixture_reader_by_full_fallback(self):
        with tempfile.TemporaryDirectory() as tmp:
            project = Path(tmp)
            known = self.crate(project, 'known_reader')
            second = self.crate(project, 'second_reader')
            unrelated = self.crate(project, 'unrelated')
            fixture = project / 'fixtures/known.json'
            fixture.parent.mkdir()
            fixture.write_text('{}')
            (project / known.parent / 'src/lib.rs').write_text('include_str!("../../../fixtures/known.json");')
            for name in ['include', 'include_str', 'include_bytes']:
                (project / second.parent / 'src/lib.rs').write_text(
                    f'use std::{name} as read_fixture; read_fixture!("../../../fixtures/known.json");')
                result = ci.numerical_selection(project, [known, second, unrelated], [ci.PROJECT + 'fixtures/known.json'], EMPTY_READS)
                self.assertEqual(result['classification'], 'full-fallback')
                self.assertEqual(result['manifests'], [known.as_posix(), second.as_posix(), unrelated.as_posix()])
                self.assertIn('Unresolved compiled Rust macro import', result['fallback_reasons'][0])
