#!/usr/bin/env python3
"""Exact desktop engine output cache. Any unresolved input disables reuse."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

from hosted_ci import wasm_engine_inputs, compiled_crate_inputs, declared_compiled_reads

ROOT = Path(__file__).resolve().parents[2]
SCHEMA = 'desktop-wasm-v1'
ENGINES = {
    'wasm-engine': 'open_pipe_stress_operation_applier',
    'self-weight-engine': 'open_pipe_stress_self_weight_wasm',
}
RECEIPT = 'apps/desktop/public/wasm-cache.json'


def expected_outputs():
    return [f'apps/desktop/public/{directory}/{filename}'
            for directory, crate in ENGINES.items()
            for filename in (f'{crate}.js', f'{crate}.d.ts', f'{crate}_bg.wasm',
                             f'{crate}_bg.wasm.d.ts', 'source-stamp.json')]


def input_paths(root):
    # Each invocation sees the current checkout, including added/deleted sources.
    wasm_engine_inputs.cache_clear()
    compiled_crate_inputs.cache_clear()
    crates, reads = wasm_engine_inputs(root)
    tracked = subprocess.check_output(['git', '-C', str(root), 'ls-files', '-z']).decode().split('\0')
    paths = set(reads)
    # Build scripts also read declared files with fs::read (not include!).
    # Reuse the selector's source-bound declaration verification rather than
    # treating those files as invisible compile inputs.
    declared_compiled_reads.cache_clear()
    profile = root / 'tools/hosted-ci-routing.json'
    declarations = json.loads(profile.read_text())['piping_runtime_reads']
    paths.update(declared_compiled_reads(root, frozenset(crates), json.dumps(declarations, sort_keys=True)))
    paths.add('tools/hosted-ci-routing.json')
    for name in filter(None, tracked):
        if (any(name.startswith(crate + '/') for crate in crates)
                or name.startswith(('apps/desktop/scripts/', 'tools/software_workflow/', 'tools/ci/'))
                or Path(name).name in ('Cargo.lock', 'Cargo.toml', 'rust-toolchain', 'rust-toolchain.toml')
                or name in ('package.json', 'package-lock.json', 'apps/desktop/package.json')):
            paths.add(name)
    # Config can name an untracked compiler, linker or another config. Rather
    # than infer its meaning, use the ordinary build on configured runners.
    # Cargo reads configuration from its working directory and every parent:
    # the build runs Cargo from apps/desktop (the npm workspace) and from each
    # engine crate, so every directory on those chains counts, and so does
    # any tracked configuration anywhere in the tree.
    if any('.cargo/config' in '/' + name for name in tracked if name):
        raise ValueError('Tracked Cargo configuration disables engine output reuse')
    starts = {root / 'apps/desktop', root / 'apps/desktop/scripts', *(root / crate for crate in crates)}
    for start in starts:
        for parent in (start, *start.parents):
            for name in ('config', 'config.toml'):
                if (parent / '.cargo' / name).exists():
                    raise ValueError('Cargo configuration disables engine output reuse')
    for name in ('config', 'config.toml'):
        if (Path(os.environ.get('CARGO_HOME', Path.home() / '.cargo')) / name).exists():
            raise ValueError('Cargo home configuration disables engine output reuse')
    # npm configuration can replace the shell or environment the build script
    # runs under (script-shell, node-options, ...). A candidate can add it
    # anywhere npm looks: refuse any tracked or present project .npmrc.
    if any(Path(name).name == '.npmrc' for name in tracked if name):
        raise ValueError('npm configuration disables engine output reuse')
    for directory in (root, root / 'apps', root / 'apps/desktop'):
        if (directory / '.npmrc').exists():
            raise ValueError('npm configuration disables engine output reuse')
    for name in paths:
        path = root / name
        if not path.is_file() or path.is_symlink() or path.resolve() != root.resolve() / name:
            raise ValueError('Unresolved engine input: ' + name)
    return sorted(paths)


def cache_key(root, environment, rustc, bindgen, node):
    paths = input_paths(root)
    # These cover compiler/flags/profile/target overrides; wrappers and custom
    # compilers cannot be identified by the default rustc probe, so refuse them.
    if any(environment.get(k) for k in ('RUSTC', 'RUSTC_WRAPPER', 'RUSTC_WORKSPACE_WRAPPER',
                                        'CARGO_BUILD_RUSTC', 'CARGO_BUILD_RUSTC_WRAPPER')):
        raise ValueError('Compiler override disables engine output reuse')
    context = {k: v for k, v in environment.items()
               if k.startswith(('CARGO_', 'RUST', 'CC', 'CXX', 'AR_', 'WASM', 'npm_config_', 'NPM_CONFIG_', 'NODE_OPTIONS'))
               and k not in ('CARGO_HOME', 'CARGO_MAKEFLAGS')}
    # Runner-level npm configuration is not candidate-controlled, but it can
    # change how the build runs, so its content is part of the key.
    for name in ('NPM_CONFIG_USERCONFIG', 'npm_config_userconfig', 'NPM_CONFIG_GLOBALCONFIG', 'npm_config_globalconfig'):
        config = environment.get(name)
        if config and Path(config).is_file():
            context['npmrc:' + name] = hashlib.sha256(Path(config).read_bytes()).hexdigest()
    user_npmrc = Path(environment.get('HOME', str(Path.home()))) / '.npmrc'
    if user_npmrc.is_file():
        context['npmrc:home'] = hashlib.sha256(user_npmrc.read_bytes()).hexdigest()
    context.update({k: environment.get(k, '') for k in ('RUNNER_OS', 'RUNNER_ARCH', 'ImageOS', 'ImageVersion')})
    context.update(schema=SCHEMA, rustc=rustc, bindgen=bindgen, node=node)
    # Target, features, flags and dedicated environment assignments are also
    # covered by hashing the build script and all its local helper modules.
    digest = hashlib.sha256(json.dumps(context, sort_keys=True).encode())
    for name in paths:
        digest.update(name.encode() + b'\0' + (root / name).read_bytes() + b'\0')
    return SCHEMA + '-' + digest.hexdigest(), paths


def output_digests(root, inputs):
    result = {}
    for name in expected_outputs():
        path = root / name
        if not path.is_file() or path.is_symlink() or path.stat().st_size == 0:
            raise ValueError('Missing engine output: ' + name)
        if name.endswith('/source-stamp.json'):
            stamp = json.loads(path.read_text())
            if stamp['format'] != 'openpipestress.engine-source-stamp.v2':
                raise ValueError('Unknown engine stamp')
            if not set(stamp['compile_reads']).issubset(inputs):
                raise ValueError('Build discovered compile reads outside the cache key')
        result[name] = hashlib.sha256(path.read_bytes()).hexdigest()
    return result


def restored(root, key, inputs):
    try:
        receipt = json.loads((root / RECEIPT).read_text())
        return receipt == {'key': key, 'outputs': output_digests(root, inputs)}
    except (OSError, ValueError, KeyError, TypeError):
        return False


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('key', 'verify', 'record'))
    parser.add_argument('--key', default='')
    args = parser.parse_args()
    outputs = {}
    try:
        versions = [subprocess.check_output(command, text=True).strip() for command in
                    (['rustc', '-vV'], ['wasm-bindgen', '--version'], ['node', '--version'])]
        key, paths = cache_key(ROOT, os.environ, *versions)
        if args.action == 'key':
            outputs = {'key': key}
            print(json.dumps({'key': key, 'inputs': paths}, indent=2))
        elif args.action == 'verify':
            outputs = {'valid': str(key == args.key and restored(ROOT, key, paths)).lower()}
        elif key == args.key:
            receipt = {'key': key, 'outputs': output_digests(ROOT, paths)}
            (ROOT / RECEIPT).write_text(json.dumps(receipt, sort_keys=True) + '\n')
            outputs = {'valid': 'true'}
        else:
            raise ValueError('Inputs changed during the build; cache disabled')
    except Exception as exc:
        print(f'Engine output cache disabled: {exc}')
        outputs = {'key': '', 'valid': 'false'}
    if os.environ.get('GITHUB_OUTPUT'):
        with open(os.environ['GITHUB_OUTPUT'], 'a') as out:
            for name, value in outputs.items():
                out.write(f'{name}={value}\n')
    else:
        print(json.dumps(outputs))


if __name__ == '__main__':
    main()
