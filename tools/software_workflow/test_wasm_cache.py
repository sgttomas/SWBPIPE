"""Output reuse is exact and incomplete restores always build."""
import json
from unittest.mock import patch

import pytest
import wasm_cache


def fixture_tree(root):
    files = ['crate/src/lib.rs', 'crate/Cargo.toml', 'crate/Cargo.lock',
             'schemas/compile.json', 'apps/desktop/scripts/build-wasm-engine.mjs',
             'apps/desktop/scripts/engineSourceStamp.mjs', 'package.json', 'package-lock.json',
             'apps/desktop/package.json', 'tools/hosted-ci-routing.json']
    for name in files:
        path = root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text('{"piping_runtime_reads": {}}' if name == 'tools/hosted-ci-routing.json' else '{}')
    return files


def test_key_covers_input_classes_and_is_stable(tmp_path):
    files = fixture_tree(tmp_path)
    with patch.object(wasm_cache, 'input_paths', return_value=files):
        key = lambda **kw: wasm_cache.cache_key(tmp_path, kw.get('env', {}), kw.get('rustc', 'rustc 1'),
                                               kw.get('bindgen', 'bindgen 1'), kw.get('node', 'node 1'))[0]
        original = key()
        assert key(env={'GITHUB_REF': 'refs/pull/2/merge'}) == original
        for name in files:
            path = tmp_path / name
            before = path.read_bytes()
            path.write_text('changed')
            assert key() != original, name
            path.write_bytes(before)
        assert key(rustc='rustc 2') != original
        assert key(bindgen='bindgen 2') != original
        assert key(node='node 2') != original
        assert key(env={'RUSTFLAGS': '-C opt-level=1'}) != original
        with patch.object(wasm_cache, 'SCHEMA', 'next'):
            assert key() != original
        with pytest.raises(ValueError):
            key(env={'RUSTC_WRAPPER': 'unknown'})


def test_input_enumeration_reuses_selection_and_tracks_whole_crates(tmp_path):
    files = fixture_tree(tmp_path)
    with patch.object(wasm_cache, 'wasm_engine_inputs', return_value=({'crate'}, {'schemas/compile.json'})), \
         patch.object(wasm_cache, 'declared_compiled_reads', return_value={'schemas/compile.json'}), \
         patch.object(wasm_cache.subprocess, 'check_output', return_value=('\0'.join(files) + '\0').encode()):
        paths = wasm_cache.input_paths(tmp_path)
        assert set(files) <= set(paths)
        (tmp_path / 'crate/src/lib.rs').unlink()
        with pytest.raises(ValueError):
            wasm_cache.input_paths(tmp_path)


def test_cargo_configuration_on_any_working_directory_chain_disables_reuse(tmp_path):
    # Review r1: the npm workspace build runs Cargo from apps/desktop, whose
    # ancestors (not only the repository root's) carry Cargo configuration.
    for config in ['apps/desktop/.cargo/config.toml', 'apps/.cargo/config', 'crate/.cargo/config.toml',
                   'crate/src/.cargo/config.toml', '.cargo/config.toml']:
        files = fixture_tree(tmp_path)
        tracked = files + ([config] if config.startswith('crate/src/') else [])
        target = tmp_path / config
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text('[build]\nrustflags = ["-C", "opt-level=0"]\n')
        with patch.object(wasm_cache, 'wasm_engine_inputs', return_value=({'crate'}, {'schemas/compile.json'})), \
             patch.object(wasm_cache, 'declared_compiled_reads', return_value={'schemas/compile.json'}), \
             patch.object(wasm_cache.subprocess, 'check_output', return_value=('\0'.join(tracked) + '\0').encode()):
            with pytest.raises(ValueError, match='Cargo configuration'):
                wasm_cache.input_paths(tmp_path)
        target.unlink()


def test_npm_configuration_disables_reuse_or_enters_the_key(tmp_path, monkeypatch):
    # Review r2: a candidate .npmrc can set script-shell to a wrapper that
    # exports different compiler flags before the build script runs.
    for npmrc, tracked_extra in [('.npmrc', []), ('apps/desktop/.npmrc', []), ('docs/.npmrc', ['docs/.npmrc'])]:
        files = fixture_tree(tmp_path)
        target = tmp_path / npmrc
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text('script-shell=${PWD}/tools/engine-shell.sh\n')
        with patch.object(wasm_cache, 'wasm_engine_inputs', return_value=({'crate'}, {'schemas/compile.json'})), \
             patch.object(wasm_cache, 'declared_compiled_reads', return_value={'schemas/compile.json'}), \
             patch.object(wasm_cache.subprocess, 'check_output', return_value=('\0'.join(files + tracked_extra) + '\0').encode()):
            with pytest.raises(ValueError, match='npm configuration'):
                wasm_cache.input_paths(tmp_path)
        target.unlink()
    files = fixture_tree(tmp_path)
    home = tmp_path / 'home'
    home.mkdir()
    with patch.object(wasm_cache, 'input_paths', return_value=files):
        key = lambda env: wasm_cache.cache_key(tmp_path, env, 'rustc 1', 'bindgen 1', 'node 1')[0]
        original = key({'HOME': str(home)})
        assert key({'HOME': str(home), 'npm_config_script_shell': '/bin/other'}) != original
        assert key({'HOME': str(home), 'NODE_OPTIONS': '--require x'}) != original
        (home / '.npmrc').write_text('node-options=--require x\n')
        assert key({'HOME': str(home)}) != original


def test_tracked_node_modules_tooling_disables_reuse(tmp_path):
    # Review r3: a tracked apps/node_modules/.bin/cargo survives npm ci and
    # shadows the trusted cargo for the workspace build.
    files = fixture_tree(tmp_path)
    for shadow in ['apps/node_modules/.bin/cargo', 'node_modules/.bin/wasm-bindgen', 'apps/desktop/node_modules/x/index.js']:
        with patch.object(wasm_cache, 'wasm_engine_inputs', return_value=({'crate'}, {'schemas/compile.json'})), \
             patch.object(wasm_cache, 'declared_compiled_reads', return_value={'schemas/compile.json'}), \
             patch.object(wasm_cache.subprocess, 'check_output', return_value=('\0'.join(files + [shadow]) + '\0').encode()):
            with pytest.raises(ValueError, match='node_modules'):
                wasm_cache.input_paths(tmp_path)


def test_every_missing_or_modified_output_falls_back(tmp_path):
    for name in wasm_cache.expected_outputs():
        path = tmp_path / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps({'format': 'openpipestress.engine-source-stamp.v2', 'compile_reads': []})
                        if name.endswith('source-stamp.json') else 'artifact')
    receipt = tmp_path / wasm_cache.RECEIPT
    receipt.write_text(json.dumps({'key': 'exact', 'outputs': wasm_cache.output_digests(tmp_path, [])}))
    assert wasm_cache.restored(tmp_path, 'exact', [])
    assert not wasm_cache.restored(tmp_path, 'other-inputs', [])
    for name in wasm_cache.expected_outputs():
        path = tmp_path / name
        content = path.read_bytes()
        path.unlink()
        assert not wasm_cache.restored(tmp_path, 'exact', []), name
        path.write_bytes(content + b' ')
        assert not wasm_cache.restored(tmp_path, 'exact', []), name
        path.write_bytes(content)
    stamp = tmp_path / wasm_cache.expected_outputs()[4]
    stamp.write_text(json.dumps({'format': 'openpipestress.engine-source-stamp.v2', 'compile_reads': ['unknown']}))
    with pytest.raises(ValueError, match='outside the cache key'):
        wasm_cache.output_digests(tmp_path, [])


def test_key_computation_error_disables_cache_without_failing_build(tmp_path, monkeypatch):
    output = tmp_path / 'github-output'
    monkeypatch.setenv('GITHUB_OUTPUT', str(output))
    monkeypatch.setattr('sys.argv', ['wasm_cache.py', 'key'])
    with patch.object(wasm_cache.subprocess, 'check_output', side_effect=OSError('tool unavailable')):
        wasm_cache.main()
    assert output.read_text() == 'key=\nvalid=false\n'
