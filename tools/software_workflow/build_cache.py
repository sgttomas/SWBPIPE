#!/usr/bin/env python3
"""Key and trim shared Cargo dependency caches; never cache workspace outputs."""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[2]
SUITES = ('piping-numerical', 'piping-desktop-rust')


def inputs(root, suite):
    if suite == 'piping-desktop-rust':
        project = root / 'apps/desktop/src-tauri'
        return project, [project / 'Cargo.toml'], project / 'target'
    path = root / 'tools/ci/numerical_ci.py'
    spec = importlib.util.spec_from_file_location('cache_numerical', path)
    numerical = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(numerical)
    project = root / numerical.PROJECT
    manifests, _ = numerical.cargo_plan(project)
    return project, [project / manifest for manifest in manifests], root / 'piping-numerical-evidence/target'


def cache_key(root, suite, environment, rustc):
    project, manifests, target = inputs(root, suite)
    compiler = {key: environment.get(key, '') for key in
                ('RUNNER_OS', 'RUNNER_ARCH', 'ImageOS', 'CARGO_INCREMENTAL',
                 'CARGO_BUILD_JOBS', 'RUSTFLAGS', 'RUSTDOCFLAGS', 'CARGO_ENCODED_RUSTFLAGS')}
    compiler['rustc'] = rustc
    # A numerical command/profile change must not restore incompatible profiles.
    if suite == 'piping-numerical':
        compiler['profile'] = (project / 'tools/ci/numerical_ci.py').read_text()
    compiler_hash = hashlib.sha256(json.dumps(compiler, sort_keys=True).encode()).hexdigest()[:16]
    prefix = f'shared-build-v1-{suite}-{compiler_hash}-'
    digest = hashlib.sha256()
    paths = set(manifests + [manifest.with_name('Cargo.lock') for manifest in manifests])
    # Include Cargo configuration along each manifest's ancestor chain.
    for manifest in manifests:
        for parent in manifest.parents:
            if not parent.is_relative_to(root):
                break
            paths.update(path for path in (parent / '.cargo/config', parent / '.cargo/config.toml') if path.is_file())
    for path in sorted(paths):
        digest.update(path.relative_to(root).as_posix().encode() + b'\0' + path.read_bytes())
    return prefix + digest.hexdigest(), prefix, target


def external_names(manifests):
    names = set()
    for manifest in manifests:
        metadata = json.loads(subprocess.check_output([
            'cargo', 'metadata', '--locked', '--format-version', '1',
            '--manifest-path', str(manifest)], text=True))
        for package in metadata['packages']:
            # Local path crates are workspace outputs even when Cargo regards
            # them as dependencies of another standalone manifest.
            if package.get('source') is None:
                continue
            names.add(package['name'])
            names.update(target['name'] for target in package['targets'])
    return names | {name.replace('-', '_') for name in names}


def remove(path):
    if path.is_dir() and not path.is_symlink():
        shutil.rmtree(path)
    else:
        path.unlink()


def trim_target(target, names):
    """Retain dependency fingerprints, libraries and build-script outputs only."""
    if not target.is_dir():
        return
    for path in target.iterdir():
        if path.name == 'CACHEDIR.TAG':
            continue
        if path.is_dir() and not path.is_symlink():
            if any((path / name).is_dir() for name in ('deps', '.fingerprint', 'build')):
                for child in path.iterdir():
                    if child.name not in ('deps', '.fingerprint', 'build') or not child.is_dir() or child.is_symlink():
                        remove(child)
                        continue
                    for artifact in child.iterdir():
                        match = re.fullmatch(r'(?:lib)?(.+)-[0-9a-f]+(?:\..*)?', artifact.name)
                        # Try both spellings: optional lib can also be part of
                        # the real crate name, e.g. libc.
                        stem = re.sub(r'-[0-9a-f]+(?:\..*)?$', '', artifact.name)
                        if not match or not ({stem, stem.removeprefix('lib')} & names):
                            remove(artifact)
            else:
                trim_target(path, names)
                if not any(path.iterdir()):
                    path.rmdir()
        else:
            remove(path)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('key', 'trim'))
    parser.add_argument('--suite', choices=SUITES, required=True)
    args = parser.parse_args()
    if args.action == 'key':
        rustc = subprocess.check_output(['rustc', '--version', '--verbose'], text=True)
        key, prefix, target = cache_key(ROOT, args.suite, os.environ, rustc)
        cargo_home = Path(os.environ.get('CARGO_HOME', Path.home() / '.cargo'))
        paths = [cargo_home / 'registry', cargo_home / 'git', target]
        with open(os.environ['GITHUB_OUTPUT'], 'a') as out:
            out.write(f'key={key}\nprefix={prefix}\npaths<<CACHE_PATHS\n')
            out.write('\n'.join(str(path) for path in paths) + '\nCACHE_PATHS\n')
        print(key)
    else:
        _, manifests, target = inputs(ROOT, args.suite)
        trim_target(target, external_names(manifests))
        # Cargo recreates unpacked crate sources from the registry archives.
        # Keep -sys sources whose timestamp-sensitive build scripts need them.
        source = Path(os.environ.get('CARGO_HOME', Path.home() / '.cargo')) / 'registry/src'
        if source.is_dir():
            for registry in source.iterdir():
                if registry.is_dir():
                    for crate in registry.iterdir():
                        if not re.search(r'-sys-\d', crate.name):
                            remove(crate)
        print('Trimmed cache to registry/Git dependencies and compiled dependency artifacts; no local crate outputs.')


if __name__ == '__main__':
    main()
