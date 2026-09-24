"""CI policy and exact collection contracts; no browser execution."""
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

MODULE = Path(__file__).resolve().parents[1] / 'tools/ci/e2e_plan.py'
spec = importlib.util.spec_from_file_location('e2e_plan', MODULE)
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)


class PolicyTests(unittest.TestCase):
    def setUp(self):
        environment = patch.dict(ci.os.environ)
        environment.start()
        self.addCleanup(environment.stop)
        ci.os.environ.pop('GITHUB_EVENT_NAME', None)
        ci.os.environ.pop('GITHUB_EVENT_PATH', None)
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.git('init', '-q')
        self.git('config', 'user.email', 'ci-fixture@example.invalid')
        self.git('config', 'user.name', 'CI fixture')
        for file in {ci.FAST, *ci.LEAN_TITLES, 'e2e/c3-viewport-visibility.spec.ts',
                     'e2e/b3b-project-persistence.spec.ts', 'e2e/b4-table-editing.spec.ts',
                     'e2e/b4-sections.spec.ts'}:
            self.write(ci.DESKTOP + file, '// fixture')
        for file in ['src/styles.css', 'src/features/viewport/viewportDimmingPresentation.ts',
                     'src/features/workspace/projectPersistenceIntegrity.ts', 'src/features/model-tree/ModelTree.tsx']:
            self.write(ci.DESKTOP + file, '// original production fixture')
        self.base = self.commit()

    def git(self, *args):
        return subprocess.check_output(['git', '-C', str(self.root), *args], stderr=subprocess.PIPE).decode().strip()

    def write(self, file, contents='changed'):
        path = self.root / file
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(contents)

    def commit(self):
        self.git('add', '.')
        self.git('commit', '-qm', 'fixture')
        return self.git('rev-parse', 'HEAD')

    def plan(self, **kw):
        return ci.make_plan(self.root, kw.get('event', 'pull_request'), kw.get('base', self.base),
                            'HEAD', kw.get('pr', '827'))

    def test_numerical_policy_is_independent_and_whole_pr(self):
        for path in [ci.PROJECT + p for p in ['core/x.rs', 'validation/hand_calcs/x.md',
                'fixtures/x.json', 'schemas/x.yaml', 'examples/rule_packs/x.json', '.cargo/config.toml',
                'tools/release/check_release_readiness.py', 'tools/ci/numerical_ci.py']] + list(ci.NUMERICAL_EVIDENCE_INPUTS) + ['.github/workflows/piping-desktop-e2e.yml', 'unknown-build-input']:
            with self.subTest(path=path):
                self.assertTrue(ci.numerical_input(path))
        for path in [ci.DESKTOP + 'src/App.tsx', ci.PROJECT + 'docs/design.md',
                ci.PROJECT + 'validation/evidence/other.json', 'projects/other/core/x.rs']:
            self.assertFalse(ci.numerical_input(path), path)
        resource = next(iter(ci.NUMERICAL_EVIDENCE_INPUTS))
        self.write(resource)
        self.commit()
        self.assertEqual(self.plan()['mode'], 'not-applicable')
        self.assertTrue(self.plan()['numerical_required'])
        self.write(ci.DESKTOP + ci.FAST)
        self.commit()
        self.assertTrue(self.plan()['numerical_required'])
        base = self.git('rev-parse', 'HEAD')
        (self.root / resource).rename(self.root / resource.replace('.json', '-renamed.json'))
        self.commit()
        self.assertTrue(self.plan(base=base)['numerical_required'])
        self.assertTrue(self.plan(event='workflow_dispatch')['numerical_required'])

    def test_known_unrelated_projects_and_records_are_explicit_na(self):
        for path in ['projects/chirality-app-dev/src/App.tsx', 'projects/chirality-runtime/core/a.rs',
                     ci.PROJECT + 'execution/run/evidence.json', 'docs/design.md']:
            self.write(path)
        self.commit()
        plan = self.plan()
        self.assertEqual(plan['mode'], 'not-applicable')
        self.assertEqual(plan['selected_specs'], [])
        self.assertFalse(plan['coverage_full'])
        ci.validate(self.root, plan)

    def test_shared_unknown_ci_dependencies_and_model_inputs_are_full(self):
        for path in ['package.json', '.github/workflows/piping-desktop-e2e.yml',
                     '.github/workflows/piping-e2e-cache.yml',
                     '.github/actions/setup-piping-e2e/action.yml', ci.PROJECT + 'tools/ci/e2e_plan.py',
                     ci.PROJECT + 'tools/ci/e2e_duration_hints.json', ci.PROJECT + 'package-lock.json',
                     ci.PROJECT + 'schemas/model.json', ci.PROJECT + 'core/solver/a.rs',
                     ci.DESKTOP + 'playwright.config.ts', ci.DESKTOP + 'src/App.tsx',
                     ci.DESKTOP + 'src/features/workspace/workspaceSession.ts',
                     ci.DESKTOP + 'src/features/workspace/table/EngineeringTable.tsx', ci.PROJECT + 'unknown.ts']:
            previous = self.git('rev-parse', 'HEAD')
            self.write(path)
            self.commit()
            self.assertEqual(self.plan(base=previous)['mode'], 'full', path)

    def test_root_instruction_packages_and_project_agent_guidance_are_not_product_inputs(self):
        for path in ['AGENTS.md', 'docs/SPEC.md', 'workflows/construct-local-work-graph/WORKFLOW.md',
                     'workflows/construct-local-work-graph/resources/work-graph-template.md',
                     '.agents/skills/preparation/SKILL.md', ci.PROJECT + 'AGENTS.md',
                     ci.PROJECT + 'loop/LOOP_INIT.md']:
            self.write(path)
        self.commit()
        plan = self.plan()
        self.assertEqual(plan['mode'], 'not-applicable')
        ci.validate(self.root, plan)

    def test_instruction_edits_cannot_mask_a_product_edit(self):
        self.write('workflows/construct-local-work-graph/WORKFLOW.md')
        self.write(ci.DESKTOP + 'src/App.tsx')
        self.commit()
        self.assertEqual(self.plan()['mode'], 'full')

    def test_complete_pr_diff_prevents_last_commit_underselection(self):
        self.write(ci.DESKTOP + 'src/App.tsx')
        self.commit()
        self.write(ci.DESKTOP + ci.FAST)
        self.commit()
        self.assertEqual(self.plan()['mode'], 'full')

    def test_source_spec_only_and_deleted_renamed_fallback(self):
        path = ci.DESKTOP + 'e2e/ui-foundation.spec.ts'
        self.write(path)
        self.commit()
        self.assertEqual(self.plan()['mode'], 'changed-specs')
        (self.root / path).rename(self.root / ci.DESKTOP / 'e2e/renamed.spec.ts')
        self.commit()
        self.assertEqual(self.plan()['mode'], 'full')

    def test_lean_and_affected_ownership(self):
        cases = [
            ('src/features/workspace/shell/DisabledReason.tsx', 'lean', None),
            ('src/styles.css', 'lean-affected', 'e2e/workspace-layout.spec.ts'),
            ('src/features/viewport/routeDraft.ts', 'lean-affected', 'e2e/ui-foundation.spec.ts'),
            ('src/features/results/ResultsPanel.tsx', 'lean-affected', 'e2e/gui-workflow-validation.spec.ts')]
        for path, mode, file in cases:
            with self.subTest(path=path), patch.object(ci, 'changes', return_value=[{'status': 'M', 'path': ci.DESKTOP + path}]):
                plan = self.plan()
                self.assertEqual(plan['mode'], mode)
                self.assertEqual(plan['selected_titles'], {**ci.LEAN_TITLES, **({'e2e/ui-foundation.spec.ts': ci.LEAN_TITLES['e2e/ui-foundation.spec.ts'] + ci.LAYOUT_TITLES} if path == 'src/styles.css' else {})})
                if file: self.assertIn(file, plan['selected_specs'])

    def assert_dedicated_product_coverage(self, product_path, dedicated_spec):
        # Dedicated specs exist before the diff: this must be production-driven
        # affected selection, not the changed-spec-only route.
        self.assertTrue((MODULE.parents[2] / 'apps/desktop' / dedicated_spec).is_file())
        self.write(ci.DESKTOP + product_path)
        self.commit()
        plan = self.plan()
        self.assertEqual(plan['changed_paths'], [{'status': 'M', 'path': ci.DESKTOP + product_path}])
        self.assertEqual(plan['mode'], 'lean-affected')
        self.assertIn(dedicated_spec, plan['selected_specs'])
        self.assertIn(ci.FAST, plan['selected_specs'])
        for file, titles in ci.LEAN_TITLES.items():
            self.assertTrue(set(titles).issubset(plan['selected_titles'][file]))
        ci.validate(self.root, plan)

    def test_c3_dedicated_spec_follows_product_only_authoring_change(self):
        self.assert_dedicated_product_coverage('src/features/viewport/viewportDimmingPresentation.ts',
                                              'e2e/c3-viewport-visibility.spec.ts')

    def test_c3_dedicated_spec_follows_product_only_layout_change(self):
        self.assert_dedicated_product_coverage('src/styles.css', 'e2e/c3-viewport-visibility.spec.ts')

    def test_b4_dedicated_spec_follows_product_only_table_host_change(self):
        self.assert_dedicated_product_coverage('src/features/model-tree/ModelTree.tsx',
                                              'e2e/b4-table-editing.spec.ts')

    def test_b4_dedicated_spec_follows_product_only_layout_change(self):
        self.assert_dedicated_product_coverage('src/styles.css', 'e2e/b4-table-editing.spec.ts')

    def test_sections_dedicated_spec_follows_product_only_table_host_change(self):
        self.assert_dedicated_product_coverage('src/features/model-tree/ModelTree.tsx',
                                              'e2e/b4-sections.spec.ts')

    def test_sections_dedicated_spec_follows_product_only_layout_change(self):
        self.assert_dedicated_product_coverage('src/styles.css', 'e2e/b4-sections.spec.ts')

    def test_b3b_dedicated_spec_follows_product_only_persistence_change(self):
        self.assert_dedicated_product_coverage('src/features/workspace/projectPersistenceIntegrity.ts',
                                              'e2e/b3b-project-persistence.spec.ts')

    def test_closed_pr_exception_is_retired(self):
        self.write(ci.DESKTOP + 'src/App.tsx')
        self.commit()
        self.assertEqual(self.plan(pr='825')['mode'], 'full')
        self.assertNotIn('baseline', self.plan(pr='825'))

    def test_actual_import_closure_includes_shared_instrument_consumers(self):
        self.write(ci.E2E + 'ui-foundation/benchmark-harness.ts', 'export const x = 1')
        self.write(ci.E2E + 'ui-foundation-workflows.ts', 'export { x } from "./ui-foundation/benchmark-harness"')
        self.write(ci.E2E + 'workspace-layout.spec.ts', 'import { x } from "./ui-foundation-workflows"')
        self.write(ci.E2E + 'ui-foundation.spec.ts', 'import { x } from "./ui-foundation-workflows"')
        with patch.object(ci, 'changes', return_value=[{'status': 'M', 'path': ci.E2E + 'ui-foundation/benchmark-harness.ts'}]):
            plan = self.plan()
            self.assertEqual(plan['mode'], 'lean-affected')
            self.assertIn('e2e/workspace-layout.spec.ts', plan['selected_specs'])
            self.assertIn('e2e/ui-foundation.spec.ts', plan['selected_specs'])

    def test_unknown_instrument_and_fixture_inputs_are_full(self):
        for suffix in ['new-helper.ts', 'fixtures/model.json', 'playwright.custom.config.ts']:
            with patch.object(ci, 'changes', return_value=[{'status': 'M', 'path': ci.E2E + 'ui-foundation/' + suffix}]):
                self.assertEqual(self.plan()['mode'], 'full')

    def test_manual_full_is_explicit_milestone(self):
        self.assertEqual(self.plan(event='workflow_dispatch')['mode'], 'full')
        self.assertEqual(self.plan()['mode'], 'not-applicable')

    def test_manual_retarget_proof_requires_integrated_explicit_target(self):
        plan = self.plan(event='workflow_dispatch')
        self.assertEqual(plan['target_base'], self.base)
        self.assertEqual(plan['base'], self.base)
        ci.validate(self.root, plan)
        for invalid in ['unavailable', 'HEAD']:
            with self.assertRaisesRegex(ValueError, 'Update the PR base'):
                ci.validate(self.root, self.plan(event='workflow_dispatch', base=invalid))
        self.git('checkout', '-qb', 'new-target')
        self.write(ci.DESKTOP + 'src/target-change.ts')
        new_target = self.commit()
        self.git('checkout', '-qb', 'candidate', self.base)
        with self.assertRaisesRegex(ValueError, 'Update the PR base'):
            ci.validate(self.root, self.plan(event='workflow_dispatch', base=new_target))

    def test_manual_event_cannot_drop_requested_target_or_change_head(self):
        plan = self.plan(event='workflow_dispatch')
        event_file = self.root / 'dispatch.json'
        event_file.write_text(json.dumps({'inputs': {'target_base': self.base}}))
        with patch.dict(ci.os.environ, {'GITHUB_EVENT_NAME': 'workflow_dispatch',
                        'GITHUB_EVENT_PATH': str(event_file), 'GITHUB_SHA': plan['head']}):
            ci.validate(self.root, plan)
            for patch_data in [{'target_base': ''}, {'head': 'wrong'}]:
                with self.assertRaises(ValueError): ci.validate(self.root, {**plan, **patch_data})

    def test_unavailable_target_blocks_even_na(self):
        for base in ['', 'unavailable']:
            with self.assertRaisesRegex(ValueError, 'Update the PR base'):
                ci.validate(self.root, self.plan(base=base))

    def test_stale_target_is_distinct_from_merge_base_and_blocks(self):
        self.git('checkout', '-qb', 'target')
        self.write(ci.DESKTOP + 'src/base-only.ts')
        target = self.commit()
        self.git('checkout', '-qb', 'candidate', self.base)
        self.write(ci.DESKTOP + ci.FAST)
        self.commit()
        plan = self.plan(base=target)
        self.assertEqual(plan['target_base'], target)
        self.assertEqual(plan['base'], self.base)
        with self.assertRaisesRegex(ValueError, 'Update the PR base'):
            ci.validate(self.root, plan)

    def test_malformed_and_stale_plan_are_refused(self):
        plan = self.plan()
        for data in [{**plan, 'mode': 'full'}, {**plan, 'selected_specs': [ci.FAST]}, {**plan, 'version': 1}]:
            with self.assertRaises(ValueError): ci.validate(self.root, data)
        self.write(ci.E2E + 'new.spec.ts')
        with self.assertRaises(ValueError): ci.validate(self.root, plan)

    def test_host_event_cannot_be_relabelled(self):
        plan = self.plan()
        event_file = self.root / 'event.json'
        event_file.write_text(json.dumps({'number': 827, 'pull_request': {
            'base': {'sha': self.base}, 'head': {'sha': plan['head']}, 'labels': []}}))
        with patch.dict(ci.os.environ, {'GITHUB_EVENT_NAME': 'pull_request', 'GITHUB_EVENT_PATH': str(event_file)}):
            ci.validate(self.root, plan)
            for patch_data in [{'event': 'workflow_dispatch'}, {'target_base': ''}, {'pr': '825'}]:
                with self.assertRaises(ValueError): ci.validate(self.root, {**plan, **patch_data})


class CollectionTests(unittest.TestCase):
    def source(self):
        rows = []
        def add(file, title, projects=ci.PROJECTS, tags=()):
            for project in projects:
                rows.append(dict(id=file + title + project, file=file, title=title,
                                 title_path=[title], tags=list(tags), project=project, line=1))
        add(ci.FAST, 'accessibility')
        for file, titles in ci.LEAN_TITLES.items():
            for title in titles: add(file, title)
        for title in ci.LAYOUT_TITLES: add('e2e/ui-foundation.spec.ts', title)
        for theme in ('light', 'dark'):
            for density in ('comfortable', 'compact'):
                for width, height in ((1024, 768), (1280, 800), (1440, 920)):
                    add('e2e/ui-foundation.spec.ts', f'task and analysis dock preserve usable canvas {theme} {density} {width}x{height}', ['chromium-desktop'], ['@explicit-viewport'])
        return rows

    def plan(self, mode='full'):
        source = self.source()
        return dict(mode=mode, selected_specs=sorted({t['file'] for t in source}) if mode == 'full' else [ci.FAST],
                    selected_titles={} if mode == 'full' else ci.LEAN_TITLES, appearance=False)

    def test_full_and_lean_preserve_explicit_selection(self):
        source = self.source()
        self.assertEqual(ci.select_tests(self.plan(), source), source)
        lean = ci.select_tests(self.plan('lean'), source)
        self.assertEqual(len(lean), 18)
        self.assertLess(len(lean), len(source))

    def test_missing_duplicate_title_or_profile_fails(self):
        for kind in ['missing', 'duplicate', 'profile']:
            source = self.source()
            if kind == 'missing': source.pop(3)
            elif kind == 'duplicate': source.append({**source[3], 'id': 'another-id'})
            else: source = [t for t in source if t['project'] != 'chromium-compact']
            with self.subTest(kind=kind), self.assertRaises(ValueError):
                ci.select_tests(self.plan('lean'), source)

    def test_dedup_matrix_loss_and_override_are_refused(self):
        source = self.source()
        with self.assertRaises(ValueError): ci.validate_source(source[:-1])
        duplicate = {**source[-1], 'project': 'chromium-compact', 'id': 'reintroduced'}
        with self.assertRaisesRegex(ValueError, 'reintroduced'): ci.validate_source(source + [duplicate])

    def test_empty_missing_extra_or_duplicate_partition_fails(self):
        rows = self.source()
        for actual in [[], rows[:-1], rows + [rows[0]], rows + [{**rows[0], 'id': 'extra'}]]:
            with self.assertRaises(ValueError): ci.assert_partition(rows, actual)
        ci.assert_partition(rows, rows[::-1])

    def test_exact_list_preserves_project_file_and_nested_titles(self):
        row = self.source()[0]
        row = {**row, 'title_path': ['group [a] $(touch nope)', 'literal > child']}
        line = ci.exact_list([row])
        self.assertIn('[chromium-desktop] › b3-accessibility.spec.ts › group [a] $(touch nope) › literal > child', line)
        args = ci.command('/tmp/list with spaces.txt', list_only=True)
        self.assertIn(str(Path('/tmp/list with spaces.txt').resolve()), args)
        self.assertNotIn('--grep', args)
        with self.assertRaises(ValueError): ci.exact_list([{**row, 'title_path': ['bad\nline']}])
        with self.assertRaises(ValueError): ci.exact_list([])

    def test_balancing_is_deterministic_exact_and_keeps_serial_file(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            for file in {t['file'] for t in self.source()}:
                path = root / ci.DESKTOP / file
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text('// independent')
            serial_file = 'e2e/workspace-layout.spec.ts'
            (root / ci.DESKTOP / serial_file).write_text('test.describe.configure({mode: "serial"})')
            first = ci.assign_partitions(self.plan(), self.source(), root)
            second = ci.assign_partitions(self.plan(), self.source()[::-1], root)
            self.assertEqual({k: sorted(map(ci.test_key,v)) for k,v in first.items()},
                             {k: sorted(map(ci.test_key,v)) for k,v in second.items()})
            ci.assert_partition(self.source(), [t for rows in first.values() for t in rows])
            for project in ci.PROJECTS:
                self.assertEqual(sum(any(t['file'] == serial_file and t['project'] == project for t in rows) for rows in first.values()), 1)
            unknown = {**self.source()[0], 'title': 'new', 'title_path': ['new']}
            self.assertEqual(ci.duration_weight(unknown), 30)

    def test_new_unobserved_tests_remain_in_full_union(self):
        source = self.source()
        for project in ci.PROJECTS:
            source.append(dict(id='new-' + project, file='e2e/new.spec.ts', title='unknown',
                               title_path=['unknown'], tags=[], project=project, line=1))
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            for file in {t['file'] for t in source}:
                path = root / ci.DESKTOP / file
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text('// new file remains atomic')
            partitions = ci.assign_partitions(self.plan(), source, root)
            ci.assert_partition(source, [t for rows in partitions.values() for t in rows])
            self.assertEqual(sum(t['file'] == 'e2e/new.spec.ts' for rows in partitions.values() for t in rows), 2)

    def test_json_reporter_tag_spelling_is_normalized(self):
        report = {'specs': [{'id': 'id', 'file': 'a.spec.ts', 'title': 'case', 'line': 1,
                            'tags': ['explicit-viewport'], 'tests': [{'projectName': 'chromium-desktop'}]}]}
        self.assertEqual(ci.collected_tests(report)[0]['tags'], ['@explicit-viewport'])

    def test_empty_error_collection_refused(self):
        for report in [{}, {'errors': [{'message': 'failed'}]}, {'suites': []}]:
            with self.assertRaises(ValueError): ci.collected_tests(report)


class WorkflowTriggerTests(unittest.TestCase):
    def test_metadata_events_cannot_create_or_replace_validation_checks(self):
        workflow = (MODULE.parents[4] / '.github/workflows/piping-desktop-e2e.yml').read_text()
        match = ci.re.search(r'types:\s*\[([^]]+)\]', workflow)
        self.assertIsNotNone(match)
        subscribed = {event.strip() for event in match.group(1).split(',')}
        self.assertEqual(subscribed, {'opened', 'synchronize', 'reopened'})
        for metadata in ['edited', 'ready_for_review', 'labeled', 'unlabeled']:
            # No workflow event means no job, concurrency participant or stable
            # success/skipped check can supersede the source validation.
            self.assertNotIn(metadata, subscribed)
        self.assertNotIn('pull_request_target:', workflow)
        self.assertNotIn('labels.*.name', workflow)
        self.assertIn('workflow_dispatch:', workflow)
        self.assertIn('inputs.target_base', workflow)
        self.assertIn('cancel-in-progress: ${{ github.event_name == \'pull_request\' }}', workflow)
        self.assertIn('github.event.pull_request.number || github.ref', workflow)


class GateTests(unittest.TestCase):
    def gate(self, *args):
        return ci.aggregate(*args, 'false', 'skipped')

    def test_every_mode_fails_closed(self):
        valid = [('not-applicable', 'skipped', 'skipped'), ('full', 'success', 'success'),
                 ('lean', 'success', 'skipped'), ('lean-affected', 'success', 'skipped'), ('changed-specs', 'success', 'skipped')]
        for mode, barrier, remainder in valid:
            self.assertTrue(self.gate(mode, 'success', barrier, remainder))
            for state in ['failure', 'cancelled', '', 'unknown']:
                self.assertFalse(self.gate(mode, state, barrier, remainder))
                self.assertFalse(self.gate(mode, 'success', state, remainder))
                self.assertFalse(self.gate(mode, 'success', barrier, state))
        for mode in ['', 'pr825-repair', 'instruments']:
            self.assertFalse(self.gate(mode, 'success', 'success', 'skipped'))
        self.assertFalse(self.gate('not-applicable', 'success', 'success', 'skipped'))
        self.assertFalse(self.gate('full', 'success', 'success', 'skipped'))


if __name__ == '__main__':
    unittest.main()
