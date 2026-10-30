"""Build-cache scope, compiler keys and dependency-only retention."""
from pathlib import Path
from unittest.mock import patch
import build_cache
import yaml

ROOT = Path(__file__).resolve().parents[2]


def load(name):
    return yaml.load((ROOT / '.github/workflows' / name).read_text(), Loader=yaml.BaseLoader)


def test_keys_are_independent_of_pr_and_workspace_source(tmp_path):
    project = tmp_path / 'apps/desktop/src-tauri'
    project.mkdir(parents=True)
    (project / 'Cargo.toml').write_text('[package]\nname="app"\nversion="1"\n')
    (project / 'Cargo.lock').write_text('version = 4\n')
    env = {'RUNNER_OS': 'macOS', 'RUNNER_ARCH': 'ARM64', 'ImageOS': 'macos26', 'CARGO_INCREMENTAL': '0'}
    first = build_cache.cache_key(tmp_path, 'piping-desktop-rust', env, 'rustc 1.97.1')
    (project / 'source.rs').write_text('changed own source')
    assert build_cache.cache_key(tmp_path, 'piping-desktop-rust', dict(env, GITHUB_REF='refs/pull/999/merge'), 'rustc 1.97.1') == first
    assert build_cache.cache_key(tmp_path, 'piping-desktop-rust', env, 'rustc 1.98')[:2] != first[:2]
    (project / 'Cargo.lock').write_text('version = 4\n# new dependencies\n')
    changed = build_cache.cache_key(tmp_path, 'piping-desktop-rust', env, 'rustc 1.97.1')
    assert changed[0] != first[0] and changed[1] == first[1]


def test_desktop_rust_cache_is_keyed_by_the_crate_manifest_and_lock_only(tmp_path):
    project = tmp_path / 'apps/desktop/src-tauri'
    project.mkdir(parents=True)
    (project / 'Cargo.toml').write_text('[package]\nname="desktop"\nversion="1"\n')
    (project / 'Cargo.lock').write_text('version = 4\n')
    env = {'RUNNER_OS': 'macOS', 'RUNNER_ARCH': 'ARM64', 'ImageOS': 'macos26', 'CARGO_INCREMENTAL': '0'}
    key, prefix, target = build_cache.cache_key(tmp_path, 'piping-desktop-rust', env, 'rustc 1.97.1')
    assert prefix.startswith('shared-build-v1-piping-desktop-rust-') and key.startswith(prefix)
    assert target == project / 'target'
    (project / 'src').mkdir()
    (project / 'src/lib.rs').write_text('changed own source')
    assert build_cache.cache_key(tmp_path, 'piping-desktop-rust', env, 'rustc 1.97.1')[0] == key
    (project / 'Cargo.lock').write_text('version = 4\n# new dependencies\n')
    assert build_cache.cache_key(tmp_path, 'piping-desktop-rust', env, 'rustc 1.97.1')[0] != key


def test_cleanup_retains_union_of_external_dependencies_and_no_local_outputs(tmp_path):
    paths = ['debug/deps/libserde-abcdef.rlib', 'debug/deps/liblibc-abcdef.rmeta',
             'debug/.fingerprint/serde-abcdef/lib-serde.json', 'debug/build/foreign-sys-abcdef/out/lib.a',
             'debug/deps/liblocal-abcdef.rlib', 'debug/deps/local-abcdef',
             'debug/.fingerprint/local-abcdef/lib-local.json', 'debug/build/local-abcdef/out/local',
             'debug/incremental/local/a', 'debug/examples/local', 'debug/local', '.rustc_info.json']
    for name in paths:
        path = tmp_path / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text('artifact')
    build_cache.trim_target(tmp_path, {'serde', 'libc', 'foreign-sys', 'foreign_sys'})
    for name in paths[:4]:
        assert (tmp_path / name).is_file(), name
    for name in paths[4:]:
        assert not (tmp_path / name).exists(), name


def test_path_dependencies_are_not_cacheable_workspace_outputs():
    import json
    metadata = {'packages': [
        {'name': 'local', 'source': None, 'targets': [{'name': 'local'}]},
        {'name': 'foreign-crate', 'source': 'registry+url', 'targets': [{'name': 'foreign_crate'}]}]}
    with patch.object(build_cache.subprocess, 'check_output', return_value=json.dumps(metadata)):
        assert build_cache.external_names([Path('Cargo.toml')]) == {'foreign-crate', 'foreign_crate'}


def test_only_main_warmer_saves_build_caches_and_runs_no_tests():
    warmer = load('shared-build-cache.yml')
    assert warmer['on']['push']['branches'] == ['main']
    assert 'schedule' not in warmer['on']
    for name, job in warmer['jobs'].items():
        if name == 'piping-desktop-wasm':
            continue  # Build-only composite and exact outputs covered by workflow tests.
        assert "github.ref == 'refs/heads/main'" in job['if']
        steps = job['steps']
        assert any(step.get('uses') == 'actions/cache/save@v4' for step in steps)
        runs = '\n'.join(step.get('run', '') for step in steps)
        assert 'build_cache.py trim' in runs
        for line in runs.splitlines():
            if 'cargo test' in line:
                assert '--no-run' in line
            if 'numerical_ci.py' in line:
                assert '--build-only' in line
    for workflow, job in [('piping-desktop-e2e.yml', 'numerical'), ('piping-desktop-e2e.yml', 'desktop-rust')]:
        steps = load(workflow)['jobs'][job]['steps']
        assert any(step.get('uses') == './.github/actions/restore-build-cache' for step in steps)
        # Exact-result records may still save; full Cargo target caches may not.
        for step in steps:
            assert step.get('uses') not in ('Swatinem/rust-cache@v2', 'actions/cache@v4')
            if step.get('uses') == 'actions/cache/save@v4':
                assert 'verified-' in step['with']['path']
    action = yaml.load((ROOT / '.github/actions/restore-build-cache/action.yml').read_text(), Loader=yaml.BaseLoader)
    assert all(step.get('uses') != 'actions/cache/save@v4' for step in action['runs']['steps'])


def test_warmer_and_consumer_compiler_environment_match():
    warmer = load('shared-build-cache.yml')
    for suite, filename, job in [('piping-numerical', 'piping-desktop-e2e.yml', 'numerical'),
                                 ('piping-desktop-rust', 'piping-desktop-e2e.yml', 'desktop-rust')]:
        consumer = load(filename)['jobs'][job]
        assert warmer['jobs'][suite]['runs-on'] == consumer['runs-on']
        compiler_env = lambda env: {key: value for key, value in env.items() if key.startswith(('CARGO', 'RUST'))}
        assert compiler_env(consumer['env']) == compiler_env(warmer['jobs'][suite]['env'])
