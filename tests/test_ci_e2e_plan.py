"""CI routing contracts. Temporary Git history is isolated from the worktree."""
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import tempfile
import unittest
from unittest.mock import patch

MODULE = Path(__file__).resolve().parents[1] / 'tools/ci/e2e_plan.py'
spec = importlib.util.spec_from_file_location('e2e_plan', MODULE)
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)


class SelectionTests(unittest.TestCase):
    def setUp(self):
        # Fixture repositories are independent of the outer GitHub event.
        environment = patch.dict(ci.os.environ)
        environment.start()
        self.addCleanup(environment.stop)
        ci.os.environ.pop('GITHUB_EVENT_NAME', None)
        ci.os.environ.pop('GITHUB_EVENT_PATH', None)
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.git('init', '-q')
        self.git('config', 'user.email', 'ci-test@example.invalid')
        self.git('config', 'user.name', 'CI fixture')
        for path in ci.FOCUSED + ['e2e/ui-foundation.spec.ts', 'e2e/new.test.mts',
                                 'e2e/ui-foundation/a.spec.ts', 'e2e/ui-foundation/b.spec.ts',
                                 'e2e/example-dist.spec.ts', 'e2e/ui-foundation/a.benchmark.ts']:
            self.write(ci.DESKTOP + path, '// fixture\n')
        self.write(ci.DESKTOP + 'src/styles.css', 'old')
        self.write(ci.DESKTOP + 'src/features/workspace/shell/DisabledReason.tsx', 'old')
        self.base = self.commit()
        self.baseline = patch.object(ci, 'BASELINE', self.base)
        self.baseline.start()
        self.addCleanup(self.baseline.stop)

    def git(self, *args):
        return subprocess.check_output(['git', '-C', str(self.root), *args], stderr=subprocess.PIPE).decode().strip()

    def write(self, path, text='changed'):
        target = self.root / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text)

    def commit(self):
        self.git('add', '.')
        self.git('commit', '-qm', 'fixture')
        return self.git('rev-parse', 'HEAD')

    def plan(self, **kwargs):
        return ci.make_plan(self.root, kwargs.pop('event', 'pull_request'),
                            kwargs.pop('base', self.base), 'HEAD', kwargs.pop('pr', '826'))

    def test_manual_and_unavailable_diff_are_full(self):
        self.assertEqual(self.plan(event='workflow_dispatch')['mode'], 'full')
        self.assertEqual(self.plan(base='unavailable')['mode'], 'full')
        self.assertEqual(self.plan()['mode'], 'full')

    def test_complete_pr_diff_not_last_commit(self):
        self.write(ci.DESKTOP + 'src/App.tsx')
        self.commit()
        self.write(ci.E2E + 'b3-accessibility.spec.ts')
        self.commit()
        plan = self.plan()
        self.assertEqual(plan['mode'], 'full')
        self.assertEqual(len(plan['changed_paths']), 2)

    def test_merge_base_excludes_base_branch_only_changes(self):
        self.git('checkout', '-qb', 'target')
        self.write(ci.DESKTOP + 'src/base-only.ts')
        target = self.commit()
        self.git('checkout', '-qb', 'candidate', self.base)
        self.write(ci.E2E + 'workspace-layout.spec.ts')
        self.commit()
        plan = self.plan(base=target)
        self.assertEqual(plan['base'], self.base)
        self.assertEqual(plan['mode'], 'changed-specs')
        self.assertEqual(plan['target_base'], target)
        with self.assertRaisesRegex(ValueError, 'Update the PR base'):
            ci.validate(self.root, plan)

    def test_spec_and_record_reduction(self):
        self.write(ci.E2E + 'workspace-layout.spec.ts')
        self.write(ci.PROJECT + 'execution/run/record.json', '{}')
        self.commit()
        plan = self.plan()
        self.assertEqual(plan['mode'], 'changed-specs')
        self.assertEqual(set(plan['selected_specs']), {ci.FAST, 'e2e/workspace-layout.spec.ts'})
        self.assertFalse(plan['coverage_full'])
        ci.validate(self.root, plan)

    def test_instrument_inputs_use_full_fallback(self):
        self.write(ci.E2E + 'ui-foundation/helper.ts')
        self.commit()
        plan = self.plan()
        self.assertEqual(plan['mode'], 'full')
        self.assertIn('e2e/workspace-layout.spec.ts', plan['selected_specs'])

    def test_instrument_spec_only_changes_use_changed_specs(self):
        self.write(ci.E2E + 'ui-foundation/a.spec.ts')
        self.commit()
        plan = self.plan()
        self.assertEqual(plan['mode'], 'changed-specs')
        self.assertNotIn('e2e/ui-foundation/b.spec.ts', plan['selected_specs'])

    def test_unknown_input_and_executable_record_fall_back(self):
        for path in ['unclassified.config', ci.PROJECT + 'execution/run/code.py',
                     ci.PROJECT + 'schemas/a.json', ci.PROJECT + 'core/a.rs',
                     ci.PROJECT + 'fixtures/a.json', ci.DESKTOP + 'playwright.config.ts']:
            with self.subTest(path=path):
                self.write(path)
                self.commit()
                self.assertEqual(self.plan()['mode'], 'full')

    def test_shared_instrument_helper_and_fixture_force_full(self):
        for path in ['benchmark-harness.ts', 'fixtures/input.json']:
            self.write(ci.E2E + 'ui-foundation/' + path)
            self.commit()
            self.assertEqual(self.plan()['mode'], 'full')

    def test_unavailable_target_base_blocks_execution(self):
        for base in ['', 'unavailable']:
            plan = self.plan(base=base)
            with self.assertRaisesRegex(ValueError, 'Update the PR base'):
                ci.validate(self.root, plan)

    def test_hosted_event_cannot_be_relabelled_or_drop_target_base(self):
        plan = self.plan()
        event_file = self.root / 'event.json'
        event_file.write_text(json.dumps({'number': 826, 'pull_request': {
            'base': {'sha': self.base}, 'head': {'sha': plan['head']}}}))
        with patch.dict(ci.os.environ, {'GITHUB_EVENT_NAME': 'pull_request', 'GITHUB_EVENT_PATH': str(event_file)}):
            ci.validate(self.root, plan)
            for patch_data in [{'event': 'workflow_dispatch'}, {'target_base': ''}, {'pr': '825'}]:
                with self.assertRaises(ValueError): ci.validate(self.root, {**plan, **patch_data})

    def test_deleted_and_renamed_specs_fall_back(self):
        old = self.root / (ci.E2E + 'workspace-layout.spec.ts')
        old.rename(old.with_name('renamed.spec.ts'))
        self.commit()
        self.assertEqual(self.plan()['mode'], 'full')
        old.with_name('renamed.spec.ts').unlink()
        self.commit()
        self.assertEqual(self.plan()['mode'], 'full')

    def test_pr825_exception_is_narrow_and_partial(self):
        self.write(ci.DESKTOP + 'src/styles.css')
        self.write(ci.DESKTOP + 'src/features/workspace/shell/DisabledReason.tsx')
        self.write(ci.PROJECT + 'tools/ci/e2e_plan.py')
        self.write(ci.PROJECT + 'docs/CI_STRATEGY.md')
        self.commit()
        plan = self.plan(pr='825')
        self.assertEqual(plan['mode'], 'pr825-repair')
        self.assertEqual(plan['baseline'], self.base)
        self.assertEqual(plan['selected_specs'], ci.FOCUSED)
        self.assertEqual(plan['focused_titles'], ci.FOCUSED_TITLES)
        self.assertFalse(plan['coverage_full'])
        ci.validate(self.root, plan)
        self.assertEqual(self.plan(pr='826')['mode'], 'full')
        self.write(ci.DESKTOP + 'src/App.tsx')
        self.commit()
        self.assertEqual(self.plan(pr='825')['mode'], 'full')

    def test_missing_or_nonancestor_baseline_cannot_enable_exception(self):
        self.write(ci.DESKTOP + 'src/styles.css')
        candidate = self.commit()
        with patch.object(ci, 'BASELINE', 'a' * 40):
            self.assertEqual(self.plan(pr='825')['mode'], 'full')
        self.git('checkout', '--orphan', 'unrelated')
        other = self.commit()
        self.git('checkout', candidate)
        with patch.object(ci, 'BASELINE', other):
            self.assertEqual(self.plan(pr='825')['mode'], 'full')

    def test_manifest_and_named_evidence_can_accompany_repair(self):
        self.write(ci.DESKTOP + 'src/styles.css')
        self.write('docs/governance_harness/tranche_manifests/PIPING-CI-STRATEGY-20260920.yaml')
        self.write(ci.REPAIR_EVIDENCE[0] + 'trace.zip')
        self.write(ci.PROJECT + 'validation/evidence/sweeps/new.json', '{}')
        self.commit()
        self.assertEqual(self.plan(pr='825')['mode'], 'pr825-repair')
        self.write(ci.PROJECT + 'execution/other/trace.zip')
        self.commit()
        self.assertEqual(self.plan(pr='825')['mode'], 'full')

    def test_only_exact_reproduction_scripts_are_exception_records(self):
        self.write(ci.DESKTOP + 'src/styles.css')
        prefix = ci.RUN + 'instances/B3-CODEX/ci-tooltip-repair/'
        self.write(prefix + '_run_records/implementer/run-focused.sh')
        self.commit()
        self.assertEqual(self.plan(pr='825')['mode'], 'pr825-repair')
        self.write(prefix + '_run_records/other.sh')
        self.commit()
        self.assertEqual(self.plan(pr='825')['mode'], 'full')

    def test_deleted_allowed_component_falls_back(self):
        (self.root / (ci.DESKTOP + 'src/features/workspace/shell/DisabledReason.tsx')).unlink()
        self.commit()
        self.assertEqual(self.plan(pr='825')['mode'], 'full')

    def test_malformed_or_stale_plan_is_rejected(self):
        plan = self.plan()
        ci.validate(self.root, plan)
        for key, value in [('mode', 'changed-specs'), ('selected_specs', []),
                           ('projects', ['other']), ('inventory', []), ('coverage_full', False)]:
            with self.subTest(key=key):
                with self.assertRaises(ValueError):
                    ci.validate(self.root, {**plan, key: value})
        self.write(ci.E2E + 'new.spec.ts')
        with self.assertRaises(ValueError):
            ci.validate(self.root, plan)
        (self.root / ci.DESKTOP / ci.FAST).unlink()
        with self.assertRaises(ValueError):
            self.plan()

    def test_argument_arrays_escape_regex_and_preserve_spaces(self):
        unusual = 'e2e/odd [a] $(touch nope);.spec.ts'
        self.write(ci.DESKTOP + unusual)
        self.commit()
        plan = self.plan()
        command = ci.commands(plan, 'barrier')[-1]
        self.assertIn(re.escape(unusual) + '$', command)
        self.assertEqual(command[0], '../../node_modules/.bin/playwright')
        self.assertIn('--workers=1', command)
        self.assertNotIn('--fully-parallel', command)

    def test_full_barrier_remainder_partition_and_new_spec_discovery(self):
        plan = self.plan(event='workflow_dispatch')
        barrier = ci.commands(plan, 'barrier')[0]
        remainder = ci.commands(plan, 'remainder', 1)[0]
        patterns = lambda command: [v for v in command if v.startswith('e2e/')]
        seen = patterns(barrier) + patterns(remainder)
        self.assertEqual(len(seen), len(set(seen)))
        self.assertEqual(set(seen), {re.escape(s) + '$' for s in plan['inventory']})
        self.assertIn('e2e/new.test.mts', plan['inventory'])
        self.assertNotIn('e2e/example-dist.spec.ts', plan['inventory'])
        for shard in range(1, 5):
            self.assertIn(f'--shard={shard}/4', ci.commands(plan, 'remainder', shard)[0])
        for shard in [None, 0, 5]:
            with self.assertRaises(ValueError):
                ci.commands(plan, 'remainder', shard)

    def test_focused_cases_use_separate_invocation_and_exact_suffix(self):
        self.write(ci.DESKTOP + 'src/styles.css')
        self.commit()
        plan = self.plan(pr='825')
        commands = ci.commands(plan, 'barrier')
        self.assertEqual(len(commands), 3)
        self.assertEqual(commands[0][-1], re.escape(ci.FAST) + '$')
        grep = commands[-1][commands[-1].index('--grep') + 1]
        self.assertEqual(commands[-1][-1], re.escape('e2e/ui-foundation.spec.ts') + '$')
        for title in ci.FOCUSED_TITLES:
            self.assertIsNotNone(re.search(grep, '[chromium-desktop] file ' + title))
            self.assertIsNone(re.search(grep, title + ' changed'))
        with self.assertRaises(ValueError):
            ci.commands(plan, 'remainder', 1)


class CollectionTests(unittest.TestCase):
    def row(self, file, title, project, suffix=''):
        return dict(id=file + title + project + suffix, file=file, title=title, project=project, line=1)

    def basis(self, mode='full'):
        files = ci.FOCUSED
        source = [self.row(file, 'ordinary', project) for file in files for project in ci.PROJECTS]
        source += [self.row('e2e/ui-foundation.spec.ts', title, project)
                   for title in ci.FOCUSED_TITLES for project in ci.PROJECTS]
        plan = dict(mode=mode, inventory=files + ['e2e/ui-foundation.spec.ts'],
                    selected_specs=files + (['e2e/ui-foundation.spec.ts'] if mode == 'full' else []),
                    focused_spec='e2e/ui-foundation.spec.ts' if mode == 'pr825-repair' else None,
                    focused_titles=ci.FOCUSED_TITLES if mode == 'pr825-repair' else [])
        collections = dict(source=source, **{'barrier-0': source[:2]})
        if mode == 'full':
            for n in range(4): collections[f'shard-{n+1}'] = source[2+n::4]
        else:
            collections['barrier-1'] = source[2:6]
            if mode == 'pr825-repair': collections['barrier-2'] = source[6:]
        return plan, collections

    def test_valid_full_and_reduced_collections(self):
        for mode in ['full', 'changed-specs', 'pr825-repair']:
            plan, collections = self.basis(mode)
            selected, omitted = ci.validate_collections(plan, collections)
            self.assertEqual(len(selected) + len(omitted), len(collections['source']))
            self.assertEqual(bool(omitted), mode == 'changed-specs')

    def test_missing_duplicate_focused_title_and_profile_are_refused(self):
        for mutation in ['missing', 'duplicate', 'profile']:
            plan, collections = self.basis('pr825-repair')
            target = collections['source'][-1]
            if mutation == 'missing': collections['source'].remove(target)
            elif mutation == 'duplicate': collections['source'].append({**target, 'id': 'distinct-duplicate-title'})
            else: collections['source'] = [t for t in collections['source'] if t['project'] != ci.PROJECTS[1]]
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                ci.validate_collections(plan, collections)

    def test_empty_selected_file_or_profile_is_refused(self):
        plan, collections = self.basis()
        collections['source'] = [t for t in collections['source'] if t['file'] != ci.FOCUSED[1]]
        with self.assertRaisesRegex(ValueError, 'Empty selected file/profile'):
            ci.validate_collections(plan, collections)

    def test_missing_duplicate_empty_partition_refused(self):
        for mutation in ['missing', 'duplicate', 'empty', 'lost-test', 'wrong-barrier']:
            plan, collections = self.basis()
            if mutation == 'missing': del collections['shard-4']
            elif mutation == 'duplicate': collections['shard-4'].append(collections['shard-1'][0])
            elif mutation == 'empty': collections['shard-4'] = []
            elif mutation == 'lost-test': collections['shard-4'].pop()
            else: collections['barrier-0'][0], collections['shard-1'][0] = collections['shard-1'][0], collections['barrier-0'][0]
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                ci.validate_collections(plan, collections)

    def test_missing_filtered_case_cannot_hide_behind_other_matches(self):
        plan, collections = self.basis('pr825-repair')
        collections['barrier-2'].pop()
        with self.assertRaisesRegex(ValueError, 'partition'):
            ci.validate_collections(plan, collections)

    def test_collection_json_must_be_nonempty_and_error_free(self):
        for report in [{}, {'errors': [{'message': 'collection failed'}]}, {'suites': []}]:
            with self.assertRaises(ValueError): ci.collected_tests(report)


class GateTests(unittest.TestCase):
    def test_full_requires_every_dependency_success(self):
        self.assertTrue(ci.aggregate('full', 'success', 'success', 'success'))
        for state in ['failure', 'cancelled', 'skipped', '', 'unknown']:
            for i in range(3):
                values = ['success'] * 3
                values[i] = state
                self.assertFalse(ci.aggregate('full', *values))

    def test_only_explicit_reduced_remainder_can_skip(self):
        for mode in ['changed-specs', 'pr825-repair']:
            self.assertTrue(ci.aggregate(mode, 'success', 'success', 'skipped'))
            self.assertFalse(ci.aggregate(mode, 'success', 'success', 'failure'))
            self.assertFalse(ci.aggregate(mode, 'failure', 'success', 'skipped'))
            self.assertFalse(ci.aggregate(mode, 'success', 'cancelled', 'skipped'))
        self.assertFalse(ci.aggregate('', 'success', 'success', 'skipped'))
        self.assertFalse(ci.aggregate('instruments', 'success', 'success', 'skipped'))


if __name__ == '__main__':
    unittest.main()
