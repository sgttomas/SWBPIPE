#!/usr/bin/env python3
"""Execute the release-readiness cargo profile with candidate-bound evidence."""
import argparse
import hashlib
import importlib.util
import json
import os
import re
from pathlib import Path
import subprocess
import sys
import time
import tomllib


def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


HERE = Path(__file__).resolve().parent
readiness = load_module('numerical_release_readiness', HERE.parent / 'release/check_release_readiness.py')
PROJECT = ''


def validate_plan(root, plan):
    if plan.get('schema') != 'chirality-hosted-ci/v1':
        raise ValueError('Unknown selection schema')
    head = subprocess.check_output(['git', '-C', str(root), 'rev-parse', 'HEAD'], text=True).strip()
    if plan.get('head') != head:
        raise ValueError('Selection does not name the checked-out candidate')
    if plan.get('modes', {}).get('piping-numerical') != 'full':
        raise ValueError('Numerical runner requires explicit selected coverage')
    sys.path.insert(0, str(root / 'tools/software_workflow'))
    hosted = load_module('numerical_hosted_ci', root / 'tools/software_workflow/hosted_ci.py')
    hosted.validate_plan(root, plan)


SELECTION_VERSION = 2
RUNTIME_READ = re.compile(r'\b(?:fs\s*::\s*(?:read(?:_to_string|_dir|_link)?|metadata|symlink_metadata|canonicalize|exists|try_exists|copy)|File\s*::\s*(?:open|from_raw_fd|from_raw_handle))\s*\(')
RUST_STRING = re.compile(r'r(?P<hashes>\#*)"[\s\S]*?"(?P=hashes)|"(?:\\[\s\S]|[^"\\])*"')


def rust_source(text):
    """Remove Rust comments without interpreting strings as comments."""
    out, index = [], 0
    while index < len(text):
        literal = RUST_STRING.match(text, index)
        if literal:
            out.append(literal.group())
            index = literal.end()
        elif text.startswith('//', index):
            end = text.find('\n', index)
            end = len(text) if end < 0 else end
            out.append(' ' * (end - index))
            index = end
        elif text.startswith('/*', index):
            start, depth = index, 1
            index += 2
            while depth and index < len(text):
                if text.startswith('/*', index):
                    depth += 1
                    index += 2
                elif text.startswith('*/', index):
                    depth -= 1
                    index += 2
                else:
                    index += 1
            if depth:
                raise ValueError('Unterminated Rust comment')
            out.append(' ' * (index - start))
        elif text[index] == "'":
            # Character literals may contain a quote; lifetimes have no closing
            # quote and are ordinary code. Preserve both without opening strings.
            char = re.match(r"'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'", text[index:])
            length = len(char.group()) if char else 1
            out.append(' ' * length if char else text[index:index + length])
            index += length
        else:
            out.append(text[index])
            index += 1
    return ''.join(out)


def rust_literal(token):
    if token.startswith('r'):
        quote = token.index('"')
        return token[quote + 1:len(token) - quote]
    body = re.sub(r'\\\r?\n\s*', '', token[1:-1])
    escapes = {'n': '\n', 'r': '\r', 't': '\t', '0': '\0', '\\': '\\', '"': '"', "'": "'"}
    def replace(match):
        value = match.group(1)
        if value in escapes:
            return escapes[value]
        if value.startswith('x'):
            return chr(int(value[1:], 16))
        if value.startswith('u{'):
            return chr(int(value[2:-1].replace('_', ''), 16))
        raise ValueError('Unsupported Rust string escape: ' + value)
    return re.sub(r'\\(u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|[\s\S])', replace, body)


def contained(project, path):
    target = Path(path).resolve()
    if not target.is_relative_to(project.resolve()):
        raise ValueError('Input escapes the Piping project: ' + str(path))
    return target


def runtime_declarations(project):
    path = project / 'tools/hosted-ci-routing.json'
    empty = {'source_trees': {}, 'reads': {}}
    return json.loads(path.read_text()).get('piping_runtime_reads', empty) if path.is_file() else empty


def rust_tree_digest(project, manifest):
    crate = project / manifest.parent
    rows = [(p.relative_to(crate).as_posix(), hashlib.sha256(p.read_bytes()).hexdigest())
            for p in sorted(crate.rglob('*.rs')) if not {'target', '.git'} & set(p.relative_to(crate).parts)]
    return hashlib.sha256(json.dumps(rows).encode()).hexdigest()


def source_read_dependencies(project, manifests, declarations=None):
    """Index exact files read by each crate; these are never Cargo edges.

    Literal compile reads are resolved from their source directory. Dynamic
    runtime reads require declarations bound to the actual source and literal
    path boundary, enumerating exact files. Any unresolved read rejects reduced
    selection; callers retain the whole Cargo suite.
    """
    project = project.resolve()
    declarations = runtime_declarations(project) if declarations is None else declarations
    if not isinstance(declarations, dict) or set(declarations) != {'source_trees', 'reads'}:
        raise ValueError('Invalid runtime read declaration shape')
    declared_trees, declarations = declarations['source_trees'], declarations['reads']
    if not isinstance(declared_trees, dict) or not isinstance(declarations, dict):
        raise ValueError('Invalid runtime read declaration tables')
    resources, checked = {}, set()
    dependencies = cargo_dependencies(project, manifests)
    tree_digests = {m.as_posix(): rust_tree_digest(project, m) for m in manifests}
    def add(target, manifest):
        target = contained(project, target)
        if not target.is_file():
            raise ValueError('Unresolved Rust read: ' + target.relative_to(project).as_posix())
        resources.setdefault(target.relative_to(project).as_posix(), set()).add(manifest)
    for manifest in manifests:
        crate = project / manifest.parent
        pending = sorted(p for p in crate.rglob('*.rs') if not {'target', '.git'} & set(p.relative_to(crate).parts))
        seen = set()
        while pending:
            source = pending.pop()
            if source in seen:
                continue
            seen.add(source)
            code = rust_source(source.read_text())
            syntax = RUST_STRING.sub(lambda m: ' ' * len(m.group()), code)
            if re.search(r'\buse\s+[^;]*\bfs\s+as\b|\buse\s+[^;]*\bfs\s*::[^;]*\b(?:read(?:_to_string|_dir|_link)?|metadata|symlink_metadata|canonicalize|exists|try_exists|copy|as)\b', syntax):
                raise ValueError('Unresolved filesystem import in ' + source.relative_to(project).as_posix())
            if re.search(r'\buse\s+[^;]*\binclude(?:_str|_bytes)?\b[^;]*\bas\b[^;]*;', syntax):
                raise ValueError('Unresolved compiled Rust macro import in ' + source.relative_to(project).as_posix())
            if re.search(r'\bOpenOptions\b', syntax):
                raise ValueError('Unresolved filesystem OpenOptions reader in ' + source.relative_to(project).as_posix())
            fs_names = set(re.findall(r'\bfs\s*::\s*(\w+)', syntax))
            allowed_fs = {'read', 'read_to_string', 'read_dir', 'read_link', 'metadata', 'symlink_metadata',
                          'canonicalize', 'exists', 'try_exists', 'copy', 'write', 'create_dir', 'create_dir_all',
                          'remove_file', 'remove_dir', 'remove_dir_all', 'rename', 'File'}
            file_methods = set(re.findall(r'\bFile\s*::\s*(\w+)', syntax))
            if (fs_names - allowed_fs or file_methods - {'open', 'create', 'from_raw_fd', 'from_raw_handle'}
                    or ('File' in fs_names and not file_methods)):
                raise ValueError('Unresolved filesystem API in ' + source.relative_to(project).as_posix())
            if re.search(r'\buse\s+[^;]*\bfs\s*::[^;]*\*', syntax):
                raise ValueError('Unresolved filesystem glob import in ' + source.relative_to(project).as_posix())
            local = source.relative_to(project).as_posix()
            if not source.is_relative_to(crate) and re.search(r'\bmod\s+\w+\s*;', syntax):
                raise ValueError('Unresolved external Rust module tree: ' + local)
            literals = {rust_literal(m.group()) for m in RUST_STRING.finditer(code)}
            for match in re.finditer(r'\binclude(?:_str|_bytes)?\s*!\s*\(|#\s*\[\s*path\s*=', syntax):
                start = match.end()
                while start < len(code) and code[start].isspace():
                    start += 1
                literal = RUST_STRING.match(code, start)
                closing = r'\s*,?\s*\)' if match.group().startswith('include') else r'\s*\]'
                if not literal or not re.match(closing, code[literal.end():]):
                    raise ValueError('Unresolved compiled Rust read in ' + local)
                target = contained(project, source.parent / rust_literal(literal.group()))
                add(target, manifest)
                if not match.group().startswith(('include_str', 'include_bytes')):
                    if target.suffix != '.rs':
                        raise ValueError('Unresolved compiled Rust source format: ' + local)
                    pending.append(target)
            declaration = declarations.get(local)
            if not RUNTIME_READ.search(syntax):
                if declaration is not None:
                    raise ValueError('Unused runtime read declaration: ' + local)
                continue
            if not isinstance(declaration, dict) or declaration.get('sha256') != hashlib.sha256(source.read_bytes()).hexdigest():
                raise ValueError('Missing or stale runtime read declaration: ' + local)
            callers = {manifest}
            if source.is_relative_to(crate / 'src'):
                while True:
                    enlarged = callers | {m for m, deps in dependencies.items() if deps & callers}
                    if enlarged == callers:
                        break
                    callers = enlarged
            expected_trees = {m.as_posix(): tree_digests[m.as_posix()] for m in sorted(callers)}
            valid_trees = declaration.get('source_trees') == sorted(expected_trees) and all(
                declared_trees.get(name) == digest for name, digest in expected_trees.items())
            if not valid_trees:
                raise ValueError('Stale or incomplete runtime caller boundaries: ' + local)
            checked.add(local)
            inputs = declaration.get('inputs')
            external = declaration.get('external_reads')
            if 'external_reads' in declaration and (not isinstance(external, dict)
                    or set(external) != {'kind', 'reason'} or external.get('kind') != 'audited-external-inputs'
                    or not isinstance(external.get('reason'), str) or not external['reason'].strip()):
                raise ValueError('Malformed external-input audit: ' + local)
            if not isinstance(inputs, list) or (not inputs and external is None):
                raise ValueError('Unresolved runtime inputs: ' + local)
            for entry in inputs:
                anchor_source = source
                if entry.get('source'):
                    anchor_source = contained(project, crate / entry['source'])
                    if anchor_source not in seen and anchor_source not in pending:
                        raise ValueError('Runtime anchor is not compiled by ' + local)
                    if entry.get('source_sha256') != hashlib.sha256(anchor_source.read_bytes()).hexdigest():
                        raise ValueError('Stale runtime anchor source: ' + local)
                    anchor_literals = {rust_literal(m.group()) for m in RUST_STRING.finditer(rust_source(anchor_source.read_text()))}
                else:
                    anchor_literals = literals
                anchor = entry.get('anchor')
                if not isinstance(anchor, str) or anchor not in anchor_literals:
                    raise ValueError('Runtime anchor absent from source: ' + local)
                bases = {'project': project, 'manifest': crate, 'source': anchor_source.parent}
                base = bases.get(entry.get('base'))
                if base is None or not entry.get('files'):
                    raise ValueError('Unresolved runtime boundary: ' + local)
                # Braced format slots can include subdirectories. The checked
                # exact list below controls coverage; new files are unknown.
                parts = re.split(r'(\{[^{}]*\})', anchor.lstrip('/'))
                static = parts[0]
                boundary = contained(project, base / static)
                # Normalize .. before matching exact files, preserving slots.
                normalized = str((base / anchor.lstrip('/')).resolve())
                pattern = '^' + ''.join('.*' if p.startswith('{') and p.endswith('}') else re.escape(p)
                                       for p in re.split(r'(\{[^{}]*\})', normalized)) + '$'
                scan = entry.get('scan')
                if scan:
                    excludes = set(scan.get('exclude', []))
                    if not entry.get('audit') or not boundary.is_dir() or not excludes <= anchor_literals:
                        raise ValueError('Unverified runtime source scan: ' + local)
                    expected = {p.relative_to(project).as_posix() for p in boundary.rglob('*')
                                if p.is_file() and not excludes & set(p.relative_to(boundary).parts)
                                and (not scan.get('hidden') or not any(part.startswith('.') for part in p.relative_to(boundary).parts))
                                and (p.name == scan.get('name') if scan.get('name') else p.suffix == scan.get('suffix'))}
                    if set(entry['files']) != expected:
                        raise ValueError('Stale runtime source scan: ' + local)
                for name in entry['files']:
                    target = contained(project, project / name)
                    if '{' in anchor:
                        valid = re.fullmatch(pattern, str(target)) is not None
                    elif boundary.is_dir():
                        valid = target.is_relative_to(boundary)
                    else:
                        valid = target == boundary
                    if not valid:
                        raise ValueError('Runtime file outside source boundary: ' + local + ' -> ' + name)
                    add(target, manifest)
    if set(declarations) != checked:
        raise ValueError('Unknown runtime read declarations: ' + ', '.join(sorted(set(declarations) - checked)))
    return resources


def cargo_dependencies(project, manifests):
    """Resolve all local Cargo edges, including inherited dev/build/target deps."""
    project = project.resolve()
    known = {(project / m).resolve(): m for m in manifests}
    dependencies = {m: set() for m in manifests}
    def workspace(crate, data):
        if isinstance(data.get('package', {}).get('workspace'), str):
            candidates = [crate / data['package']['workspace'] / 'Cargo.toml']
        else:
            candidates = [parent / 'Cargo.toml' for parent in (crate, *crate.parents) if parent.is_relative_to(project)]
        for path in candidates:
            path = contained(project, path)
            if path.is_file():
                value = tomllib.loads(path.read_text())
                if 'workspace' in value:
                    return path.parent, value['workspace'].get('dependencies', {})
        raise ValueError('Unresolved Cargo workspace for ' + str(crate.relative_to(project)))
    for manifest in manifests:
        crate = project / manifest.parent
        data = tomllib.loads((project / manifest).read_text())
        if data.get('patch') or data.get('replace'):
            raise ValueError('Unresolved Cargo dependency override: ' + manifest.as_posix())
        def table(entries):
            if not isinstance(entries, dict):
                raise ValueError('Invalid Cargo dependency table: ' + str(manifest))
            for name, spec in entries.items():
                base = crate
                if not isinstance(spec, dict):
                    continue
                if spec.get('workspace') is True:
                    base, inherited = workspace(crate, data)
                    if name not in inherited:
                        raise ValueError('Unresolved inherited Cargo dependency: ' + name)
                    spec = inherited[name]
                    if not isinstance(spec, (str, dict)):
                        raise ValueError('Invalid inherited Cargo dependency: ' + name)
                if isinstance(spec, dict) and 'path' in spec:
                    if not isinstance(spec['path'], str):
                        raise ValueError('Invalid Cargo path dependency: ' + name)
                    target = contained(project, base / spec['path'] / 'Cargo.toml')
                    if target not in known or not target.is_file():
                        raise ValueError('Unresolved local Cargo dependency: ' + str(target))
                    dependencies[manifest].add(known[target])
        for key in ('dependencies', 'dev-dependencies', 'build-dependencies'):
            table(data.get(key, {}))
        for target in data.get('target', {}).values():
            for key in ('dependencies', 'dev-dependencies', 'build-dependencies'):
                table(target.get(key, {}))
    return dependencies


def numerical_selection(project, manifests, paths, declarations=None, ignored_paths=()):
    manifests = sorted(manifests)
    reasons, selected = [], set()
    def full(reason):
        return {'version': SELECTION_VERSION, 'classification': 'full-fallback',
                'manifests': [m.as_posix() for m in manifests], 'reasons': reasons,
                'fallback_reasons': [reason]}
    if not manifests:
        return full('No numerical Cargo manifests discovered')
    if not paths:
        return full('Unavailable or empty complete diff; full numerical coverage')
    try:
        for manifest in manifests:
            for path in (manifest, manifest.with_name('Cargo.lock')):
                if not (project / path).is_file() or not (project / path).stat().st_size:
                    raise ValueError('Missing or empty cargo input: ' + str(path))
        dependencies = cargo_dependencies(project, manifests)
        resources = source_read_dependencies(project, manifests, declarations)
    except (ValueError, OSError, KeyError, TypeError, AttributeError, tomllib.TOMLDecodeError) as exc:
        return full(str(exc))
    ignored = set(ignored_paths)
    for path in sorted(set(paths)):
        if not path.startswith(PROJECT):
            if path in ignored:
                reasons.append({'path': path, 'kind': 'mapped-other-checks'})
                continue
            return full('Shared or unknown numerical input: ' + path)
        relative = Path(path[len(PROJECT):])
        readers = resources.get(relative.as_posix(), set())
        owners = {m for m in manifests if relative.is_relative_to(m.parent)} if relative.suffix == '.rs' or relative.name in {'Cargo.toml', 'Cargo.lock'} else set()
        for manifest in sorted(owners):
            selected.add(manifest)
            reasons.append({'path': path, 'kind': 'cargo-owner', 'manifest': manifest.as_posix()})
        for manifest in sorted(readers):
            selected.add(manifest)
            reasons.append({'path': path, 'kind': 'exact-file-reader', 'manifest': manifest.as_posix()})
        if owners or readers:
            continue
        if path in ignored:
            reasons.append({'path': path, 'kind': 'mapped-other-checks'})
        elif relative.parts and relative.parts[0] == 'tests' and relative.suffix == '.py' and not relative.name.startswith(('test_ci_', 'test_numerical')):
            reasons.append({'path': path, 'kind': 'python-checks'})
        else:
            return full('Unknown numerical input: ' + path)
    while True:
        consumers = {m for m, deps in dependencies.items() if deps & selected}
        added = consumers - selected
        if not added:
            break
        for manifest in sorted(added):
            reasons.append({'kind': 'cargo-consumer', 'manifest': manifest.as_posix(),
                            'dependencies': [m.as_posix() for m in sorted(dependencies[manifest] & selected)]})
        selected.update(added)
    graph = {'cargo': {m.as_posix(): [d.as_posix() for d in sorted(deps)] for m, deps in dependencies.items()},
             'reads': {p: [m.as_posix() for m in sorted(readers)] for p, readers in sorted(resources.items())}}
    return {'version': SELECTION_VERSION, 'classification': 'reachability',
            'manifests': [m.as_posix() for m in manifests if m in selected], 'reasons': reasons,
            'fallback_reasons': [], 'graph_sha256': hashlib.sha256(json.dumps(graph, sort_keys=True).encode()).hexdigest()}


def affected_manifests(project, manifests, paths, declarations=None, ignored_paths=()):
    selection = numerical_selection(project, manifests, paths, declarations, ignored_paths)
    return [Path(m) for m in selection['manifests']]


def cargo_plan(project, paths=None, selection=None):
    """Use the real readiness discovery and commands, adding locked resolution."""
    manifests = readiness.discover_cargo_manifests(project)
    if not manifests:
        raise ValueError('No cargo manifests discovered')
    for manifest in manifests:
        for path in (manifest, manifest.with_name('Cargo.lock')):
            if not (project / path).is_file() or not (project / path).stat().st_size:
                raise ValueError('Missing or empty cargo input: ' + str(path))
    steps = readiness.build_plan('cargo', project)
    if len(steps) != len(manifests):
        raise ValueError('Cargo profile and discovery differ')
    chosen = [Path(m) for m in selection['manifests']] if selection is not None else (affected_manifests(project, manifests, paths) if paths is not None else manifests)
    if len(set(chosen)) != len(chosen) or not set(chosen) <= set(manifests):
        raise ValueError('Selection contains duplicate or unknown Cargo manifests')
    tests = []
    for manifest, step in zip(manifests, steps):
        if manifest not in chosen:
            continue
        argv = list(step.command)
        if argv[:2] != ['cargo', 'test'] or '--offline' not in argv or argv[argv.index('--manifest-path') + 1] != manifest.as_posix():
            raise ValueError('Unsupported release cargo command: ' + repr(argv))
        if '--locked' not in argv:
            argv.append('--locked')
        if manifest.as_posix() == 'core/solver/frame_kernel/Cargo.toml':
            # Exact-arithmetic unit vectors dominate debug runtime. Their full
            # unchanged oracle set passes optimized with safety checks retained.
            # Historical floating-point integration pins remain on the original
            # profile: powi-derived inputs are profile-sensitive.
            tests.append(argv + ['--lib', '--config', 'profile.test.opt-level=2',
                '--config', 'profile.test.debug-assertions=true',
                '--config', 'profile.test.overflow-checks=true'])
            tests.append(argv + ['--test', '*', '--bins', '--examples'])
            tests.append(argv + ['--doc'])
        elif manifest.as_posix() in ('core/reporting/result_export/Cargo.toml', 'core/product_physics/Cargo.toml'):
            # Full reader/transport and reserialization oracles, and product_physics's
            # retained-source and judging oracles, are computationally heavy. Keep
            # every target and assertion, with both safety checks on. Hosted builds
            # are unregistered either way; the registered debug profile's evidence
            # is taken on the qualification host.
            tests.append(argv + ['--config', 'profile.test.opt-level=2',
                '--config', 'profile.test.debug-assertions=true',
                '--config', 'profile.test.overflow-checks=true'])
            if manifest == readiness.STACK_WITNESS_MANIFEST:
                # b7-plan 5a: the stack witnesses need the debug profile's frames,
                # one process each. Hosted builds are unregistered, so this is an
                # early warning, not VER-004's registered record.
                for witness in readiness.stack_witness_commands():
                    split = witness.index('--')
                    tests.append(list(witness[:split]) + ['--locked'] + list(witness[split:]))
        else:
            tests.append(argv)
    fetches = [['cargo', 'fetch', '--locked', '--manifest-path', p.as_posix()] for p in chosen]
    return chosen, fetches + tests


def build_commands(commands):
    """Compile the selected test profiles without executing tests.

    rustdoc has no stable --no-run mode. The ordinary test-profile library
    compilation supplies its dependencies; omit the separate doctest execution.
    """
    built = []
    for command in commands:
        if '--doc' in command:
            continue
        if command[:2] == ['cargo', 'test']:
            # Harness arguments after `--` select tests; compiling ignores them.
            command = command[:command.index('--')] if '--' in command else command
            command = command + ['--no-run']
            if command in built:
                continue
        built.append(list(command))
    return built


def cargo_timings(output):
    """Cargo compilation and harness test time, distinct from command wall time."""
    text = re.sub(r'\x1b\[[0-9;]*m', '', output)
    compile_seconds = 0.0
    for duration in re.findall(r'Finished .* target\(s\) in ([^\n]+)', text):
        compile_seconds += sum(float(number) * {'m': 60, 's': 1}[unit]
                               for number, unit in re.findall(r'([0-9.]+)([ms])', duration))
    test_seconds = sum(float(seconds) for seconds in
                       re.findall(r'test result: .* finished in ([0-9.]+)s', text))
    return dict(compile_seconds=round(compile_seconds, 3), test_seconds=round(test_seconds, 3))


def execute_commands(project, commands, evidence, evidence_dir, env=None):
    """Public bounded executor also supports a real temporary failing-crate control."""
    for index, argv in enumerate(commands):
        output = evidence_dir / f'command-{index:03d}.log'
        row = dict(argv=list(argv), cwd=str(project), output=output.name, exit_code=None)
        evidence['commands'].append(row)
        print('Running: ' + ' '.join(argv), flush=True)
        started = time.perf_counter()
        try:
            with output.open('w') as log:
                result = subprocess.run(argv, cwd=project, env=env, stdout=log, stderr=subprocess.STDOUT, check=False)
        finally:
            row['elapsed_seconds'] = time.perf_counter() - started
        row['exit_code'] = result.returncode
        row.update(cargo_timings(output.read_text()))
        print(f"Completed in {row['elapsed_seconds']:.3f}s (exit {result.returncode}): " + ' '.join(argv), flush=True)
        row['output_sha256'] = hashlib.sha256(output.read_bytes()).hexdigest()
        if result.returncode:
            return result.returncode if result.returncode > 0 else 128 - result.returncode
    return 0


def run(root, plan, evidence_dir, build_only=False):
    root, evidence_dir = Path(root).resolve(), Path(evidence_dir).resolve()
    evidence_dir.mkdir(parents=True, exist_ok=True)
    evidence = dict(status='failed', head=plan.get('head'), target_base=plan.get('target_base'),
                    plan=plan, commands=[], manifests=[], plan_validated=False,
                    execution='build-only' if build_only else 'tests')
    code = 1
    try:
        validate_plan(root, plan)
        evidence['plan_validated'] = True
        project = root / PROJECT
        manifests, commands = cargo_plan(project, selection=plan['piping_numerical_selection'])
        if build_only:
            commands = build_commands(commands)
            print('Build-only: compiling selected profiles; no tests execute.', flush=True)
        evidence['manifests'] = [p.as_posix() for p in manifests]
        evidence['input_sha256'] = {p.as_posix(): hashlib.sha256((project / p).read_bytes()).hexdigest()
            for m in manifests for p in (m, m.with_name('Cargo.lock'))}
        evidence['planned_commands'] = commands
        env = os.environ.copy()
        env.pop('CARGO_NET_OFFLINE', None)  # Fetch first; every test explicitly uses --offline.
        env['CARGO_TARGET_DIR'] = str(evidence_dir / 'target')
        evidence['environment'] = {'CARGO_TARGET_DIR': env['CARGO_TARGET_DIR'], 'python': sys.version}
        code = execute_commands(project, [['rustc', '--version'], ['cargo', '--version']], evidence, evidence_dir, env)
        if code == 0:
            code = execute_cargo(project, commands, evidence, evidence_dir, env)
        evidence['status'] = 'success' if code == 0 else 'failed'
    except Exception as exc:
        code = 1
        evidence['error'] = f'{type(exc).__name__}: {exc}'
        print(evidence['error'], file=sys.stderr)
    finally:
        evidence['exit_code'] = code
        totals = {key: round(sum(row.get(key, 0) for row in evidence['commands']), 3)
                  for key in ('compile_seconds', 'test_seconds', 'elapsed_seconds')}
        evidence['timing'] = totals
        message = f"Numerical {evidence['execution']} timing (seconds): {json.dumps(totals)}; status={evidence['status']}"
        print(message, flush=True)
        if os.environ.get('GITHUB_STEP_SUMMARY'):
            with open(os.environ['GITHUB_STEP_SUMMARY'], 'a') as summary:
                summary.write(message + '\n')
        (evidence_dir / 'numerical.json').write_text(json.dumps(evidence, indent=2) + '\n')
    return code


def execute_cargo(project, commands, evidence, evidence_dir, env):
    # Separate filenames retain tool-version output without overwriting it.
    logs = evidence_dir / 'cargo-logs'
    logs.mkdir(exist_ok=True)
    start = len(evidence['commands'])
    code = execute_commands(project, commands, evidence, logs, env)
    for row in evidence['commands'][start:]:
        row['output'] = 'cargo-logs/' + row['output']
    return code


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plan', required=True)
    parser.add_argument('--evidence-dir', required=True)
    parser.add_argument('--build-only', action='store_true', help='Compile the selected test profiles without running tests')
    args = parser.parse_args()
    return run(HERE.parents[1], json.loads(Path(args.plan).read_text()), args.evidence_dir, build_only=args.build_only)


if __name__ == '__main__':
    raise SystemExit(main())
