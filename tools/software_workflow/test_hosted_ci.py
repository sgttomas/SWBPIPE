"""Real-diff routing and aggregate failures for hosted Piping coverage and detached App paths."""
from fnmatch import fnmatchcase
import json
import os
from pathlib import Path
import subprocess
import tempfile
import sys
import unittest
from unittest.mock import patch

import hosted_ci
from hosted_ci import (aggregate, browser_plan, desktop_plan, desktop_rust_plan, make_plan, route_paths, select_paths, validate_plan,
                       numerical_plan, ROOT as SOURCE_ROOT, SUITES, WASM_ENGINE_CRATES)

PROFILE = json.loads((Path(__file__).resolve().parents[1] / "hosted-ci-routing.json").read_text())

# Every plan on this checkout re-scans each crate's Rust source for its reads
# (about 3.3 s). No test writes this checkout, so a scan's result is a pure
# function of the project, the manifests and the declarations it is given.
SOURCE_READS = {}
load_numerical_module = hosted_ci.numerical_module


def shared_numerical_module(root):
    """The real numerical selector, loaded as each plan loads it, whose source
    read scan of this checkout is computed once per distinct input. A changed
    declaration is a new key; a scan that fails is never kept; a fixture
    checkout is scanned every time."""
    module = load_numerical_module(root)
    scan = module.source_read_dependencies

    def source_read_dependencies(project, manifests, declarations=None):
        resolved = Path(project).resolve()
        if not resolved.is_relative_to(SOURCE_ROOT.resolve()):
            return scan(project, manifests, declarations)
        key = (resolved, tuple(Path(m).as_posix() for m in manifests), json.dumps(declarations, sort_keys=True))
        if key not in SOURCE_READS:
            SOURCE_READS[key] = scan(project, manifests, declarations)
        return {path: set(readers) for path, readers in SOURCE_READS[key].items()}

    module.source_read_dependencies = source_read_dependencies
    return module


def modes(**overrides):
    return {**dict.fromkeys(SUITES, "not-applicable"), **overrides}


def browser_selection(paths, profile=PROFILE):
    """`piping_browser_selection` as `select_paths` computes it, from the real
    path rules. It reads only their piping mode, piping reasons and unmatched
    paths; the reachability stages after them (about 4 s of source scanning per
    plan) do not change those for these inputs."""
    return browser_plan(paths, route_paths(paths, profile), profile)


class HostedCITests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        patcher = patch('hosted_ci.numerical_module', shared_numerical_module)
        patcher.start()
        cls.addClassCleanup(patcher.stop)

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.git("init", "-q")
        self.git("config", "user.email", "ci-test@example.invalid")
        self.git("config", "user.name", "CI routing fixture")
        self.write("README.md")
        self.base = self.commit()

    def git(self, *args):
        return subprocess.check_output(["git", "-C", str(self.root), *args], stderr=subprocess.PIPE).decode().strip()

    def write(self, path, content="fixture"):
        target = self.root / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(content)

    def commit(self):
        self.git("add", ".")
        self.git("commit", "-qm", "fixture")
        return self.git("rev-parse", "HEAD")

    def plan(self, base=None, event="pull_request"):
        return make_plan(self.root, PROFILE, event, base or self.base, "HEAD")

    def test_evergreen_instruction_pr_uses_only_bundle_and_governance(self):
        paths = ["AGENTS.md", "docs/PRD.md", "loop/LOOP_INIT.md", "execution/_Coordination/NOTICE.md"]
        for path in paths:
            self.write(path)
        self.commit()
        # This temporary Git fixture models a diff, not a product checkout.
        # Supply the actual repository source graph for its records-only paths.
        with patch('hosted_ci.numerical_plan', side_effect=lambda root, paths, profile: numerical_plan(SOURCE_ROOT, paths, profile)), \
             patch('hosted_ci.desktop_plan', side_effect=lambda root, *args: desktop_plan(SOURCE_ROOT, *args)), \
             patch('hosted_ci.desktop_rust_plan', side_effect=lambda root, *args: desktop_rust_plan(SOURCE_ROOT, *args)):
            self.assertEqual(self.plan()["modes"], modes())





    def test_earlier_source_commit_is_not_hidden_by_later_docs(self):
        self.write("unmapped/a.tsx")
        self.commit()
        self.write("docs/notes.md")
        self.commit()
        plan = self.plan()
        self.assertEqual(plan["modes"]["piping"], "full")
        self.assertEqual(len(plan["paths"]), 2)

    def test_unknown_deleted_path_requires_conservative_coverage(self):
        path = "projects/chirality-app-v4/execution/PKG-02/DEL-02-04/ScopeOfWork.md"
        self.write(path)
        base = self.commit()
        (self.root / path).unlink()
        self.commit()
        self.assertEqual(self.plan(base)["modes"], dict.fromkeys(SUITES, "full"))

    def test_move_from_product_to_records_still_checks_removed_product(self):
        original = "unmapped/a.ts"
        self.write(original)
        base = self.commit()
        self.write("docs/a.ts")
        (self.root / original).unlink()
        self.commit()
        plan = self.plan(base)
        self.assertIn(original, plan["paths"])
        self.assertEqual(plan["modes"]["piping"], "full")

    def test_newline_in_filename_cannot_hide_product_path(self):
        path = "unmapped/odd\nname.ts"
        self.write(path)
        self.commit()
        self.assertIn(path, self.plan()["paths"])
        self.assertEqual(self.plan()["modes"]["piping"], "full")

    def test_unknown_inputs_including_legacy_project_names_select_all(self):
        for path in ['unknown-root-input.bin', 'projects/pec/new-input.bin',
                     'projects/chirality-app-v4/unmapped/new.rs', 'new-code.bin']:
            selection = select_paths([path], PROFILE)
            self.assertEqual(selection['unmatched_paths'], [path])
            self.assertEqual(selection['modes'], dict.fromkeys(SUITES, 'full'))


    def test_policy_changes_require_full_consumer_checks(self):
        selection = select_paths(["tools/hosted-ci-routing.json"], PROFILE)
        self.assertEqual(selection["modes"], dict.fromkeys(SUITES, "full"))

    def test_missing_base_and_manual_dispatch_request_full_coverage(self):
        for plan in [self.plan(base="missing-ref"), self.plan(event="workflow_dispatch")]:
            self.assertEqual(plan["modes"], dict.fromkeys(SUITES, "full"))

    def test_product_paths_and_consumed_design_select_without_deliverable_mappings(self):
        selection = select_paths(['core/solver/src/lib.rs'], PROFILE)
        self.assertEqual(selection['modes'], modes(piping='full', **{'piping-numerical':'full','piping-python':'full'}))
        self.assertEqual(select_paths(['docs/note.md'], PROFILE)['modes'], modes())

    def test_piping_python_implementation_and_tests_select_python_lane(self):
        for path in ('tools/product_preview/validate.py',
                     'tests/product_preview/test_contract.py',
                     'tests/security/test_permissions.py'):
            selection = select_paths([path], PROFILE)
            self.assertEqual(selection['modes']['piping-python'], 'full')
            self.assertEqual(selection['modes']['piping-numerical'], 'full' if path.startswith('tools/') else 'not-applicable')
            self.assertEqual(selection['modes']['piping'], 'full')

    def test_new_required_results_never_launder_missing_failed_or_cancelled_checks(self):
        for suite in SUITES:
            for state in ('failure','cancelled','skipped',''):
                self.assertFalse(aggregate(suite,'full','success',state))
            self.assertFalse(aggregate(suite,'not-applicable','failure','skipped'))
            self.assertTrue(aggregate(suite,'not-applicable','success','skipped'))
            self.assertTrue(aggregate(suite,'full','success','success'))

    def test_unselected_desktop_suite_requires_its_own_job_skipped(self):
        # The desktop suite runs in its own sharded job, skipped when unselected.
        self.assertTrue(aggregate('piping-desktop', 'not-applicable', 'success', 'skipped'))
        for state in ('success', 'failure', 'cancelled'):
            self.assertFalse(aggregate('piping-desktop', 'not-applicable', 'success', state))

    def test_aggregate_cli_failure_returns_failure_without_traceback(self):
        completed = subprocess.run([sys.executable, str(Path(__file__).with_name('hosted_ci.py')),
            'aggregate', '--suite', 'piping-python', '--mode', 'full', '--selection', 'success',
            '--product', 'skipped'], capture_output=True, text=True)
        self.assertEqual(completed.returncode, 1)
        self.assertIn('did not complete', completed.stdout)
        self.assertNotIn('Traceback', completed.stderr)

    def test_merge_base_excludes_changes_unique_to_advanced_base(self):
        self.git('checkout', '-qb', 'candidate')
        self.write('projects/chirality-app-v4/app/src/candidate.ts')
        head = self.commit()
        self.git('checkout', '-qb', 'advanced-base', self.base)
        self.write('unknown-root-input.bin')
        advanced = self.commit()
        self.git('checkout', 'candidate')
        plan = make_plan(self.root, PROFILE, 'pull_request', advanced, head)
        self.assertEqual(plan['base'], self.base)
        self.assertEqual(plan['paths'], ['projects/chirality-app-v4/app/src/candidate.ts'])
        self.assertEqual(plan['modes'], dict.fromkeys(SUITES, 'full'))

    def test_renamed_fixture_keeps_old_and_new_paths(self):
        original = 'fixtures/old.json'
        self.write(original)
        base = self.commit()
        self.write('fixtures/new.json')
        (self.root / original).unlink()
        self.commit()
        plan = self.plan(base)
        self.assertEqual(plan['paths'], ['fixtures/new.json', original])
        self.assertEqual(plan['modes']['piping-numerical'], 'full')
        self.assertEqual(plan['piping_numerical_selection']['classification'], 'full-fallback')

    def test_execution_recomputes_exact_selection_identity(self):
        self.write('projects/chirality-app-v4/app/src/candidate.ts')
        self.commit()
        target = self.root / 'tools/hosted-ci-routing.json'
        target.parent.mkdir()
        target.write_text(json.dumps(PROFILE))
        plan = self.plan()
        validate_plan(self.root, plan)
        for key, value in [('classification', 'reachability'), ('manifests', ['core/unregistered/Cargo.toml']),
                           ('reasons', [{'kind': 'forged'}])]:
            modified = json.loads(json.dumps(plan))
            modified['piping_numerical_selection'][key] = value
            with self.assertRaisesRegex(ValueError, 'Selection differs'):
                validate_plan(self.root, modified)
        changed = dict(plan, paths=['projects/chirality-app-v4/docs/candidate.ts'])
        with self.assertRaisesRegex(ValueError, 'Selection differs'):
            validate_plan(self.root, changed)

    def test_piping_cache_control_does_not_expand_unrelated_app_coverage(self):
        # The browser result it keys now covers the desktop suite too; as a
        # shared Piping control it also selects the desktop Rust tests.
        self.assertEqual(select_paths(['tools/software_workflow/result_cache.py'], PROFILE)['modes'],
                         modes(piping='full', **{'piping-desktop': 'full', 'piping-desktop-rust': 'full', 'piping-numerical': 'full', 'piping-python': 'full'}))

    def test_desktop_suite_follows_the_inputs_it_reads_or_compiles(self):
        project = ''
        for path, kind in [
            ('apps/desktop/src/App.tsx', 'desktop-source-reference'),
            ('apps/desktop/package.json', 'desktop-source-reference'),
            ('package-lock.json', 'desktop-input'),
            ('fixtures/results/new.json', 'desktop-source-reference'),
            ('validation/references/t3_r1/new.json', 'desktop-input'),
            # Docs that desktop tests read are selected although docs select no product job.
            ('docs/BUILD_AND_RELEASE.md', 'desktop-source-reference'),
            # Python a desktop test reads as text.
            ('core/analysis_runs/compatibility.py', 'desktop-source-reference'),
            # Rust compiled into the wasm engines, and a file one of them includes.
            ('core/loads/primitive_loads/src/lib.rs', 'wasm-engine-source'),
            ('core/loads/self_weight_wasm/tests/fixtures/applied_self_weight_v1.json', 'desktop-input'),
            ('validation/benchmarks/numerical_robustness/src/exact.rs', 'wasm-engine-compile-read'),
        ]:
            with self.subTest(path=path):
                selection = select_paths([project + path], PROFILE)
                self.assertEqual(selection['modes']['piping-desktop'], 'full')
                self.assertEqual(selection['modes']['piping'], 'full')
                self.assertEqual(selection['piping_desktop_selection']['reasons'], [{'path': project + path, 'kind': kind}])
        for paths in [
            # Python-only: tests and tools the desktop suite neither names nor runs.
            ['tests/test_release_candidate_scan.py', 'tools/release/run_evidence_sweep.py'],
            # Rust-only: crates outside the wasm engines' normal/build dependencies.
            ['core/solver/performance_harness/src/lib.rs', 'core/reporting/report_package/src/lib.rs'],
            ['docs/note.md'],
        ]:
            with self.subTest(paths=paths):
                selection = select_paths([project + path for path in paths], PROFILE)
                self.assertEqual(selection['modes']['piping-desktop'], 'not-applicable')
                self.assertEqual(selection['piping_desktop_selection']['classification'], 'not-applicable')

    def test_desktop_selection_fails_toward_the_full_suite(self):
        selection = select_paths(['tests/test_release_candidate_scan.py'], PROFILE, self.root)
        self.assertEqual(selection['modes']['piping-desktop'], 'full')
        self.assertEqual(selection['piping_desktop_selection']['classification'], 'full-fallback')

    def test_wasm_engine_crates_are_the_ones_the_desktop_build_compiles(self):
        script = (SOURCE_ROOT / 'apps/desktop/scripts/build-wasm-engine.mjs').read_text()
        for manifest in WASM_ENGINE_CRATES:
            segments = Path(manifest).parent.parts
            self.assertIn(', '.join(f'"{part}"' for part in segments), script)

    def test_desktop_resource_table_cannot_hide_new_unknown_fixture(self):
        profile = json.loads(json.dumps(PROFILE))
        profile['piping_desktop_resources']['files'].append('fixtures/new-unreviewed.json')
        selection = select_paths(['fixtures/new-unreviewed.json'], profile)
        numerical = selection['piping_numerical_selection']
        self.assertEqual(numerical['classification'], 'full-fallback')
        # 42 discovered crates since the MBF engine (core/handoff/caepipe_mbf_engine) joined the 41 of the
        # pressure-source join, which added core/numerics/correct_norm.
        self.assertEqual(len(numerical['manifests']), 42)
        self.assertIn('Stale desktop resource declaration', numerical['fallback_reasons'][0])


    def test_desktop_rust_tests_follow_the_crate_its_path_dependencies_and_their_reads(self):
        project = ''
        for path, kind in [
            # The desktop shell crate itself: Rust, manifest, lock, build script and Tauri config.
            ('apps/desktop/src-tauri/src/solve_jobs.rs', 'desktop-rust-source'),
            ('apps/desktop/src-tauri/Cargo.lock', 'desktop-rust-source'),
            ('apps/desktop/src-tauri/build.rs', 'desktop-rust-source'),
            ('apps/desktop/src-tauri/tauri.conf.json', 'desktop-rust-source'),
            # Cargo controls the job's `cargo test` reads from its working directory (the project)
            # and the directories above the crate (Codex review of #1590).
            ('.cargo/config.toml', 'cargo-control'),
            ('.cargo/config', 'cargo-control'),
            ('apps/desktop/.cargo/config.toml', 'cargo-control'),
            ('rust-toolchain.toml', 'cargo-control'),
            ('apps/rust-toolchain', 'cargo-control'),
            # Direct and transitive Cargo path dependencies.
            ('core/units/src/lib.rs', 'path-dependency-source'),
            ('core/model_operations/operation_applier/Cargo.toml', 'path-dependency-source'),
            ('core/solver/frame_kernel/src/lib.rs', 'path-dependency-source'),
            # A fixture the crate compiles in, and fixtures its tests read by a literal or a format template.
            ('fixtures/model_document/invented_every_slot_model.json', 'desktop-rust-compile-read'),
            ('fixtures/product_preview/r2_from_blank_rehearsal.json', 'desktop-rust-source-reference'),
            ('fixtures/results/new_exact_source.json', 'desktop-rust-source-reference'),
            # The user-release reach scan reads the headless runner's sources, an allowed
            # dependent of report_package (code review r9 at #1682), so a runner change re-runs it.
            ('core/runner/headless/src/lib.rs', 'desktop-rust-source-reference'),
            ('core/runner/headless/src/bin/openpipestress-runner.rs', 'desktop-rust-source-reference'),
            # Files a path dependency's compiled source or build script reads (declared): the
            # product_physics build script reads its own sources.
            ('fixtures/rule_expressions/checksum_binding/payload_grammar_v1_0_0.json', 'path-dependency-runtime-read'),
            ('core/product_physics/src/bend_pressure.rs', 'path-dependency-runtime-read'),
            # Any other project manifest or lock: the shell's reach scan reads every project
            # package's dependencies from `cargo metadata` (W-18b r2c), so a new package that
            # depends on a reporting crate must run it.
            ('core/reporting/new_dependent/Cargo.toml', 'cargo-manifest'),
            ('validation/benchmarks/mechanics/Cargo.toml', 'cargo-manifest'),
            # (The headless runner's manifest and lock, main's examples, are under the reach scan's
            # source reference since code review r9 at #1682; performance_harness's stand in.)
            ('core/solver/performance_harness/Cargo.lock', 'cargo-manifest'),
            ('core/solver/performance_harness/Cargo.toml', 'cargo-manifest'),
            ('core/runner/headless/Cargo.toml', 'desktop-rust-source-reference'),
            # The scan finds manifests case-insensitively (code review r1 of #1706).
            ('core/runner/new_dependent/cargo.toml', 'cargo-manifest'),
            ('core/runner/new_dependent/CARGO.TOML', 'cargo-manifest'),
            ('core/runner/new_dependent/cargo.lock', 'cargo-manifest'),
        ]:
            with self.subTest(path=path):
                selection = select_paths([project + path], PROFILE)
                self.assertEqual(selection['modes']['piping-desktop-rust'], 'full')
                self.assertIn('piping-desktop-rust', selection['checks'])
                self.assertEqual(selection['piping_desktop_rust_selection']['reasons'], [{'path': project + path, 'kind': kind}])
        for paths in [
            # Desktop TypeScript, a crate outside the path dependencies and the scanned crates, Python,
            # docs and another product. (The headless runner left this list in code review r9 at
            # #1682: the reach scan reads its sources.)
            [project + 'apps/desktop/src/App.tsx', project + 'apps/desktop/package.json'],
            [project + 'core/solver/performance_harness/src/lib.rs'],
            # Non-Cargo files beside a manifest stay unselected. (The headless runner's README,
            # main's example, is under the reach scan's source reference since code review r9 at
            # #1682, as the reporting crates' directories are; performance_harness's stands in.)
            [project + 'validation/benchmarks/mechanics/src/lib.rs', project + 'core/solver/performance_harness/README.md'],
            # ASCII case only, as Rust's eq_ignore_ascii_case: a Kelvin sign is not a `k`.
            [project + 'core/runner/new_dependent/Cargo.loc\u212a'],
            [project + 'tests/test_release_candidate_scan.py', project + 'tools/release/run_evidence_sweep.py'],
            [project + 'docs/note.md'],
            # A dependency's own `.cargo/` is never read by Cargo.
            [project + 'core/units/.cargo/config.toml'],
        ]:
            with self.subTest(paths=paths):
                selection = select_paths(paths, PROFILE)
                self.assertEqual(selection['modes']['piping-desktop-rust'], 'not-applicable')
                self.assertNotIn('piping-desktop-rust', selection['checks'])
                self.assertEqual(selection['piping_desktop_rust_selection']['classification'], 'not-applicable')

    def test_desktop_rust_tests_run_for_shared_controls_unmatched_inputs_and_full_coverage(self):
        for path, kind in [('tools/hosted-ci-routing.json', 'shared-input'),
                           ('.github/workflows/piping-desktop-e2e.yml', 'shared-input'),
                           # Its dependency cache is keyed and trimmed here.
                           ('tools/software_workflow/build_cache.py', 'shared-input'),
                           ('.github/workflows/shared-build-cache.yml', 'shared-input'),
                           ('new-code.bin', 'unmatched-input'),
                           ('unknown-root-input.bin', 'unmatched-input')]:
            with self.subTest(path=path):
                selection = select_paths([path], PROFILE)
                self.assertEqual(selection['modes']['piping-desktop-rust'], 'full')
                self.assertEqual(selection['piping_desktop_rust_selection']['reasons'], [{'path': path, 'kind': kind}])
        for plan in [self.plan(base='missing-ref'), self.plan(event='workflow_dispatch')]:
            self.assertEqual(plan['modes']['piping-desktop-rust'], 'full')
            self.assertEqual(plan['piping_desktop_rust_selection']['classification'], 'full-fallback')

    def test_desktop_rust_selection_fails_toward_running_the_tests(self):
        # No readable crate in this fixture checkout.
        selection = select_paths(['tests/test_release_candidate_scan.py'], PROFILE, self.root)
        self.assertEqual(selection['modes']['piping-desktop-rust'], 'full')
        self.assertEqual(selection['piping_desktop_rust_selection']['classification'], 'full-fallback')
        # A stale runtime-read declaration in a path dependency also fails toward the tests.
        profile = json.loads(json.dumps(PROFILE))
        profile['piping_runtime_reads']['reads']['core/rules/rule_pack_lifecycle/src/lib.rs']['sha256'] = '0' * 64
        selection = select_paths(['tests/test_release_candidate_scan.py'], profile)
        self.assertEqual(selection['piping_desktop_rust_selection']['classification'], 'full-fallback')
        self.assertIn('stale runtime read declaration', selection['piping_desktop_rust_selection']['fallback_reasons'][0])

    def test_desktop_rust_closure_is_every_path_dependency_the_crate_compiles(self):
        import tomllib
        from hosted_ci import DESKTOP_RUST_CRATE, compiled_crate_inputs
        project = SOURCE_ROOT / '.'
        manifest = tomllib.loads((project / DESKTOP_RUST_CRATE / 'Cargo.toml').read_text())
        direct = {Path(os.path.normpath(Path(DESKTOP_RUST_CRATE) / spec['path'])).as_posix()
                  for key in ('dependencies', 'dev-dependencies', 'build-dependencies')
                  for spec in manifest.get(key, {}).values() if isinstance(spec, dict) and 'path' in spec}
        crates, _ = compiled_crate_inputs(SOURCE_ROOT, (DESKTOP_RUST_CRATE + '/Cargo.toml',), True, 'desktop Rust')
        self.assertTrue(direct)
        self.assertLessEqual(direct | {DESKTOP_RUST_CRATE}, crates)
        # Transitive: the applier's solver dependencies are compiled in too.
        self.assertIn('core/solver/frame_kernel', crates)

    # W-12: #1589 extended the unsaved-changes guard to Open and New and broke
    # two browser specs no hosted check ran (#1609).
    CORE_SPECS = ['r2-smoke.spec.ts', 'b3b-project-persistence.spec.ts', 'rotation-glyph.spec.ts', 'result-overlays.spec.ts']
    REPLACE_SPECS = ['ui-foundation.spec.ts', 'c3-viewport-visibility.spec.ts', 'gui-workflow-validation.spec.ts', 'linear-authoring.spec.ts']

    def test_open_new_replace_changes_add_their_browser_specs_with_reasons(self):
        project = ''
        for path in ['apps/desktop/src/features/project-storage/unsavedChangesGuard.ts',
                     'apps/desktop/src/features/project-storage/NewPrompt.tsx',
                     'apps/desktop/src/features/workspace/workspaceSession.ts',
                     'apps/desktop/src/services/projectService.ts',
                     'apps/desktop/e2e/ui-foundation.spec.ts']:
            with self.subTest(path=path):
                # An unrelated desktop path in the same diff adds no reason of its own.
                paths = [project + path, project + 'apps/desktop/src/App.tsx']
                self.assertEqual(route_paths(paths, PROFILE)['modes']['piping'], 'full')
                browser = browser_selection(paths)
                self.assertEqual(browser['classification'], 'reachability')
                self.assertEqual(browser['project'], 'chromium-desktop')
                self.assertEqual(browser['specs'], self.CORE_SPECS + self.REPLACE_SPECS)
                self.assertEqual([(r['path'], r['kind'], r['group'], r['specs']) for r in browser['reasons']],
                                 [(project + path, 'group-path', 'project-open-new-replace', self.REPLACE_SPECS)])
                self.assertIn('unsaved-changes guard', browser['reasons'][0]['why'])
        # The plan carries exactly this selection (an empty checkout keeps the
        # reachability stages to their quick fallbacks).
        paths = [project + 'apps/desktop/src/features/project-storage/unsavedChangesGuard.ts', project + 'apps/desktop/src/App.tsx']
        self.assertEqual(select_paths(paths, PROFILE, self.root)['piping_browser_selection'], browser_selection(paths))

    def test_unrelated_desktop_changes_run_only_the_core_browser_specs(self):
        project = ''
        for paths in [['apps/desktop/src/App.tsx'], ['apps/desktop/src/features/workspace/shellLayout.ts'],
                      ['apps/desktop/src/features/project-storageX/a.ts', 'apps/desktop/e2e/r2-smoke.spec.ts'],
                      ['core/solver/frame_kernel/src/lib.rs'], ['package-lock.json']]:
            with self.subTest(paths=paths):
                paths = [project + path for path in paths]
                self.assertEqual(route_paths(paths, PROFILE)['modes']['piping'], 'full')
                browser = browser_selection(paths)
                self.assertEqual(browser['classification'], 'core')
                self.assertEqual(browser['specs'], self.CORE_SPECS)
                self.assertEqual(browser['reasons'], [])
        # No product job, no specs.
        browser = browser_selection([project + 'docs/note.md'])
        self.assertEqual((browser['classification'], browser['specs']), ('not-applicable', []))

    def test_full_shared_and_unmatched_coverage_includes_every_browser_spec(self):
        for plan in [self.plan(base='missing-ref'), self.plan(event='workflow_dispatch')]:
            browser = plan['piping_browser_selection']
            self.assertEqual(browser['classification'], 'full-fallback')
            self.assertEqual(browser['specs'], self.CORE_SPECS + self.REPLACE_SPECS)
        for path, kind in [('tools/hosted-ci-routing.json', 'shared-input'),
                           ('tools/software_workflow/hosted_ci.py', 'shared-input'),
                           ('tools/software_workflow/result_cache.py', 'shared-input'),
                           ('.github/workflows/piping-desktop-e2e.yml', 'shared-input'),
                           ('.github/actions/setup-piping-e2e/action.yml', 'shared-input'),
                           ('unknown-root-input.bin', 'unmatched-input'),
                           ('new-code.bin', 'unmatched-input')]:
            with self.subTest(path=path):
                browser = browser_selection([path])
                self.assertEqual(browser['specs'], self.CORE_SPECS + self.REPLACE_SPECS)
                self.assertEqual([(r['path'], r['kind']) for r in browser['reasons']], [(path, kind)])

    def test_declared_browser_specs_and_trigger_paths_exist(self):
        declaration = PROFILE['piping_browser_journeys']
        # Only adds: the four specs the PR lane ran before W-12 remain core.
        self.assertEqual(declaration['core'], self.CORE_SPECS)
        e2e = SOURCE_ROOT / 'apps/desktop/e2e'
        tracked = subprocess.check_output(['git', '-C', str(SOURCE_ROOT), 'ls-files', 'apps/desktop'],
                                          text=True).splitlines()
        for group in declaration['groups']:
            for spec in group['specs']:
                # A filter naming a missing or dist-only spec would silently run nothing.
                self.assertTrue((e2e / spec).is_file(), spec)
                self.assertFalse(spec.endswith('-dist.spec.ts'), spec)
            for pattern in group['paths']:
                self.assertTrue(any(fnmatchcase(path[len(''):], pattern) for path in tracked), pattern)
        for spec in declaration['core']:
            self.assertTrue((e2e / spec).is_file(), spec)

    def test_malformed_browser_declaration_fails_selection(self):
        for field, value in [('project', 'chromium desktop'), ('core', []), ('core', ['r2-smoke.spec.ts; true']),
                             ('groups', [{'name': 'g', 'reason': 'r', 'paths': [], 'specs': ['a.spec.ts']}])]:
            with self.subTest(field=field, value=value):
                profile = json.loads(json.dumps(PROFILE))
                profile['piping_browser_journeys'][field] = value
                with self.assertRaises(ValueError):
                    browser_selection(['apps/desktop/src/App.tsx'], profile)
                # The plan itself fails rather than run an underived spec set.
                with self.assertRaises(ValueError):
                    select_paths(['apps/desktop/src/App.tsx'], profile, self.root)

    # Repository regressions scale with the class of change.

    def repository(self, base=None, event="pull_request"):
        return hosted_ci.repository_plan(self.root, event, base or self.base, "HEAD")

    def test_markdown_only_pr_omits_only_the_source_reachability_regressions(self):
        for path in ["docs/BUILD_AND_RELEASE.md", "execution/PKG-01/DEL-01-01/Design/note.md"]:
            self.write(path, "Prose.\n")
        base = self.commit()
        for path in ["docs/BUILD_AND_RELEASE.md", "execution/PKG-01/DEL-01-01/Design/note.md", "README.md"]:
            self.write(path, "Edited prose.\n")
        self.commit()
        plan = self.repository(base)
        self.assertEqual(plan["mode"], "markdown-only")
        self.assertEqual(plan["deselect"], list(hosted_ci.SOURCE_REACHABILITY_TESTS))

    def test_any_other_change_keeps_every_repository_regression(self):
        for path, content in [("core/x/src/lib.rs", "fn main() {}\n"), ("docs/table.csv", "a,b\n"),
                              ("tools/release/README.md", "Tool notes.\n"), ("tests/fixtures/x.md", "Fixture.\n"),
                              (".github/ISSUE_TEMPLATE/bug.md", "Template.\n"), ("docs/binary.md", "a\0b")]:
            with self.subTest(path=path):
                base = self.git("rev-parse", "HEAD")
                self.write("docs/also-changed.md", path)
                self.write(path, content)
                self.commit()
                plan = self.repository(base)
                self.assertEqual(plan["mode"], "full")
                self.assertEqual(plan["deselect"], [])

    def test_repository_regressions_fail_toward_running_everything(self):
        self.write("docs/note.md", "Prose.\n")
        self.commit()
        for event, base in [("workflow_dispatch", self.base), ("pull_request", "0" * 40)]:
            with self.subTest(event=event, base=base):
                plan = hosted_ci.repository_plan(self.root, event, base, "HEAD")
                self.assertEqual((plan["mode"], plan["deselect"]), ("full", []))
        plan = hosted_ci.repository_plan(self.root, "pull_request", "", "HEAD")
        self.assertEqual((plan["mode"], plan["deselect"]), ("full", []))
        empty = self.git("rev-parse", "HEAD")
        self.assertEqual(hosted_ci.repository_plan(self.root, "pull_request", empty, "HEAD")["mode"], "full")

    def test_adding_deleting_or_renaming_markdown_keeps_every_regression(self):
        # Review r1: deleting a declared runtime input such as
        # fixtures/model_operations/contract_corpus/README.md changes a
        # source-reachability result; existence is an input that a trace of
        # opened files cannot see.
        self.write("docs/kept.md", "Prose.\n")
        base = self.commit()
        for change in (lambda: self.git("rm", "-q", "docs/kept.md"),
                       lambda: self.write("docs/new.md", "Prose.\n"),
                       lambda: self.git("mv", "docs/kept.md", "docs/moved.md")):
            with self.subTest(change=change):
                self.git("reset", "-q", "--hard", base)
                change()
                self.git("add", "-A")
                self.git("commit", "-qm", "change")
                plan = self.repository(base)
                self.assertEqual((plan["mode"], plan["deselect"]), ("full", []))

    def test_markdown_edited_to_binary_keeps_every_regression(self):
        self.write("docs/note.md", "Prose.\n")
        base = self.commit()
        for content in ("a\0b", b"\xff\xfe".decode("latin-1")):
            with self.subTest(content=content):
                (self.root / "docs/note.md").write_bytes(content.encode("latin-1"))
                self.commit()
                self.assertEqual(self.repository(base)["mode"], "full")

    def test_symlinked_or_typechanged_markdown_keeps_every_regression(self):
        self.write("docs/target.md", "Prose.\n")
        self.write("docs/plain.md", "Prose.\n")
        os.symlink("target.md", self.root / "docs/link.md")
        base = self.commit()
        os.remove(self.root / "docs/link.md")
        os.symlink("plain.md", self.root / "docs/link.md")
        self.commit()
        self.assertEqual(self.repository(base)["mode"], "full")
        self.git("reset", "-q", "--hard", base)
        os.remove(self.root / "docs/link.md")
        self.write("docs/link.md", "Now a file.\n")
        self.commit()
        self.assertEqual(self.repository(base)["mode"], "full")

    def test_every_omitted_regression_exists(self):
        for node in hosted_ci.SOURCE_REACHABILITY_TESTS:
            with self.subTest(node=node):
                path, *names = node.split("::")
                source = (SOURCE_ROOT / path).read_text()
                self.assertIn(f"class {names[0]}(", source)
                self.assertIn(f"def {names[1]}(", source)

    # A document desktop source only names: in-place edits do not select the
    # desktop suite; additions, deletions and renames still do.

    def test_named_only_document_content_edit_selects_no_desktop_suite(self):
        self.assertTrue(hosted_ci.named_only_document(SOURCE_ROOT, "docs/BUILD_AND_RELEASE.md"))
        selection = select_paths(["docs/BUILD_AND_RELEASE.md"], PROFILE, SOURCE_ROOT,
                                 frozenset({"docs/BUILD_AND_RELEASE.md"}))
        self.assertEqual(selection["modes"]["piping-desktop"], "not-applicable")
        self.assertEqual(selection["modes"]["piping"], "not-applicable")
        self.assertEqual(selection["piping_desktop_selection"]["named_document_edits"], ["docs/BUILD_AND_RELEASE.md"])
        # Added, deleted or renamed: the named path's existence changed.
        selection = select_paths(["docs/BUILD_AND_RELEASE.md"], PROFILE, SOURCE_ROOT)
        self.assertEqual(selection["modes"]["piping-desktop"], "full")

    def test_documents_a_desktop_test_or_loading_line_names_stay_selected(self):
        for path in ["docs/claims_registry.md", "docs/IP_AND_DATA_BOUNDARY.md"]:
            with self.subTest(path=path):
                self.assertFalse(hosted_ci.named_only_document(SOURCE_ROOT, path))
                selection = select_paths([path], PROFILE, SOURCE_ROOT, frozenset({path}))
                self.assertEqual(selection["modes"]["piping-desktop"], "full")

    def test_named_reference_sites_distinguish_display_from_reads(self):
        sources = self.root / "apps/desktop/src"
        (sources / "features").mkdir(parents=True)
        (sources / "features/Panel.tsx").write_text('const paths = ["docs/shown.md", "docs/both.md"];\n'
                                                     'import notes from "docs/imported.md?raw";\n'
                                                     'const dir = "docs/guide";\n')
        (sources / "features/Panel.test.tsx").write_text('readFileSync(path.join(root, "docs/both.md"));\n')
        (sources / "test-support").mkdir()
        (sources / "test-support/load.ts").write_text('const p = "docs/support.md";\n')
        hosted_ci.desktop_reference_sites.cache_clear()
        self.addCleanup(hosted_ci.desktop_reference_sites.cache_clear)
        self.assertTrue(hosted_ci.named_only_document(self.root, "docs/shown.md"))
        for path in ["docs/both.md", "docs/imported.md", "docs/support.md", "docs/guide/page.md", "docs/unnamed.md"]:
            with self.subTest(path=path):
                self.assertFalse(hosted_ci.named_only_document(self.root, path))

    def test_plan_reads_in_place_edits_from_the_complete_diff(self):
        self.write("docs/a.md", "one\n")
        self.write("docs/b.md", "one\n")
        base = self.commit()
        self.write("docs/a.md", "two\n")
        self.git("rm", "-q", "docs/b.md")
        self.write("docs/c.md", "new\n")
        head = self.commit()
        merge_base = self.git("merge-base", base, head)
        paths, edits = hosted_ci.changed_paths(self.root, merge_base, head)
        self.assertEqual(paths, ["docs/a.md", "docs/b.md", "docs/c.md"])
        self.assertEqual(edits, frozenset({"docs/a.md"}))
        # A mode change is not an in-place content edit.
        os.chmod(self.root / "docs/a.md", 0o755)
        head = self.commit()
        self.assertEqual(hosted_ci.changed_paths(self.root, merge_base, head)[1], frozenset())


if __name__ == "__main__":
    unittest.main()
