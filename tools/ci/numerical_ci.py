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

# These tools scan the repository (crates, tests and selection patterns), so a test that imports them reads
# any path: declare the whole tree rather than an incomplete list.
CI_READS = ("**",)


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


# A crate's own test, bench and example targets: no other crate compiles them.
OWN_TARGET_DIRS = frozenset({'tests', 'benches', 'examples'})
# `source_trees` and a scan's `files` are retained, unchecked: callers and scan inventories come from the tree.
DECLARATION_KEYS = frozenset({'sha256', 'source_trees', 'inputs', 'external_reads'})
INPUT_KEYS = frozenset({'anchor', 'base', 'files', 'source', 'source_sha256', 'scan', 'audit'})
SCAN_KEYS = frozenset({'suffix', 'name', 'exclude', 'hidden'})
# An out-of-line or inline module item, with the attributes just before it, or a brace.
MODULE_TOKEN = re.compile(r'((?:#\s*\[[^\]]*\]\s*)*)(?:pub(?:\s*\([^)]*\))?\s+)?\bmod\s+(?:r#)?(\w+)\s*([;{])|[{}]')
COMPILE_READ = re.compile(r'\binclude(?:_str|_bytes)?\s*!\s*\(')


def _valid_scan(scan):
    """An authored scan rule: exactly one of a suffix or a file name, optional excluded names and hidden flag."""
    if not isinstance(scan, dict) or not scan or set(scan) - SCAN_KEYS or ('suffix' in scan) == ('name' in scan):
        return False
    if 'suffix' in scan and not (isinstance(scan['suffix'], str) and len(scan['suffix']) > 1
                                 and scan['suffix'].startswith('.') and '/' not in scan['suffix']):
        return False
    if 'name' in scan and not (isinstance(scan['name'], str) and scan['name'] and '/' not in scan['name']):
        return False
    if 'exclude' in scan and not (isinstance(scan['exclude'], list)
                                  and all(isinstance(x, str) and x and '/' not in x for x in scan['exclude'])):
        return False
    return 'hidden' not in scan or isinstance(scan['hidden'], bool)


def _string_after(code, index):
    """The Rust string literal at `index` after optional whitespace, or None."""
    while index < len(code) and code[index].isspace():
        index += 1
    return RUST_STRING.match(code, index)


CFG_ATTRIBUTE = re.compile(r'#\s*\[\s*cfg\s*\(')


def _top_level(text, separator=','):
    """`text` split at `separator` outside parentheses."""
    parts, depth, start = [], 0, 0
    for index, char in enumerate(text):
        depth += (char == '(') - (char == ')')
        if char == separator and depth == 0:
            parts.append(text[start:index])
            start = index + 1
    return parts + [text[start:]]


def test_only_attributes(attributes):
    """Whether these item attributes confine it to the crate's own unit tests: a `#[cfg(...)]` whose predicate
    is exactly `test`, or an `all(...)` with `test` among its conjuncts. Any other predicate (`test_support`, a
    feature, `any`, `not`) or one that cannot be parsed counts as a library build."""
    for match in CFG_ATTRIBUTE.finditer(attributes):
        depth, index = 1, match.end()
        while index < len(attributes) and depth:
            depth += (attributes[index] == '(') - (attributes[index] == ')')
            index += 1
        if depth:
            continue
        predicate = re.sub(r'\s+', '', attributes[match.end():index - 1])
        if predicate == 'test':
            return True
        if predicate.startswith('all(') and predicate.endswith(')') and 'test' in _top_level(predicate[4:-1]):
            return True
    return False


def compile_graph(project, manifest):
    """The Rust files a crate's library and build script compile, and the files they include as data.

    Returns (compiled, test_only, data, certain): absolute paths of every Rust file reached from the library
    root (a `[lib] path` or `src/lib.rs`) and the build script (a `[package] build` path or `build.rs`) through
    ordinary and inline `mod` items, `#[path]` modules and `include!` of Rust, in any order; those reached only
    through a module whose cfg is exactly `test` or an `all(...)` including it (`test_only_attributes`), which
    only the crate's own unit tests compile;
    the `include_str!` and `include_bytes!` targets; and whether every edge resolved. A module that resolves to
    no file, an unreadable file or an unparsable manifest makes `certain` false: callers then treat every source
    of the crate as a library or build input.
    """
    project = project.resolve()
    crate = project / manifest.parent
    try:
        data = tomllib.loads((project / manifest).read_text())
    except (OSError, tomllib.TOMLDecodeError):
        return frozenset(), frozenset(), frozenset(), False
    roots, certain = [], True
    lib = data.get('lib', {}).get('path')
    build = data.get('package', {}).get('build')
    for declared, default in ((lib, 'src/lib.rs'), (build, 'build.rs')):
        if declared is False:
            continue
        if declared is not None and not isinstance(declared, str):
            certain = False
            continue
        target = (crate / (declared or default)).resolve()
        if target.is_file():
            roots.append(target)
        elif declared:
            certain = False
    reached, included = {}, set()  # file -> reached only under cfg(test)
    pending = [(root, True, False) for root in roots]
    while pending:
        source, module_root, test = pending.pop()
        if source in reached and (reached[source] is False or test):
            continue  # already reached at least as widely
        if not source.is_relative_to(project) or not source.is_file():
            certain = False
            continue
        reached[source] = test
        try:
            code = rust_source(source.read_text())
        except (OSError, UnicodeDecodeError, ValueError):
            certain = False
            continue
        syntax = RUST_STRING.sub(lambda m: ' ' * len(m.group()), code)
        # A mod-rs file (a crate root, mod.rs or a #[path] module) holds its children beside it;
        # any other file holds them in a directory named after it.
        children = source.parent if module_root or source.name == 'mod.rs' else source.parent / source.stem
        inline, depth = [], 0  # (name, depth, under cfg(test))
        for token in MODULE_TOKEN.finditer(syntax):
            if token.group(0) == '{':
                depth += 1
                continue
            if token.group(0) == '}':
                if inline and inline[-1][1] == depth:
                    inline.pop()
                depth -= 1
                continue
            attributes, name, end = token.group(1), token.group(2), token.group(3)
            edge_test = test or test_only_attributes(attributes) or any(t for _, _, t in inline)
            if end == '{':
                depth += 1
                inline.append((name, depth, test_only_attributes(attributes)))
                continue
            nested = [n for n, _, _ in inline]
            path = re.search(r'#\s*\[\s*path\s*=', attributes)
            if path:
                literal = _string_after(code, token.start(1) + path.end())
                if literal is None:
                    certain = False
                    continue
                relative = rust_literal(literal.group())
                # Outside inline modules a path is relative to this file's directory; inside them,
                # to the inline modules' directory. Both are taken where they differ.
                bases = {source.parent} | ({children.joinpath(*nested)} if nested else set())
                found = [(base / relative).resolve() for base in bases if (base / relative).is_file()]
                pending.extend((target, True, edge_test) for target in found)
                certain &= bool(found)
                continue
            directory = children.joinpath(*nested)
            found = [c.resolve() for c in (directory / f'{name}.rs', directory / name / 'mod.rs') if c.is_file()]
            pending.extend((target, False, edge_test) for target in found)
            certain &= bool(found)
        for match in COMPILE_READ.finditer(syntax):
            literal = _string_after(code, match.end())
            if literal is None:
                certain = False
                continue
            target = (source.parent / rust_literal(literal.group())).resolve()
            if match.group().startswith(('include_str', 'include_bytes')):
                if target.is_file() and target.is_relative_to(project):
                    included.add(target)
                else:
                    certain = False
            else:
                pending.append((target, module_root, test))
    return (frozenset(reached), frozenset(f for f, t in reached.items() if t), frozenset(included), certain)


def source_read_dependencies(project, manifests, declarations=None):
    """Index exact files read by each crate; these are never Cargo edges.

    The resources of `runtime_read_index`, for which a stale per-file
    attestation widens selection rather than raising."""
    return runtime_read_index(project, manifests, declarations)[0]


def runtime_read_index(project, manifests, declarations=None):
    """Exact files read by each crate, and the declarations whose per-file attestations are stale.

    Literal compile reads are resolved from their source directory. Dynamic
    runtime reads require declarations bound to the actual source and literal
    path boundary. A scan's files are computed from the tree; callers come from
    the Cargo graph. A semantic error (malformed shape, a misspelt key or scan
    rule, a scan that finds no files, an unresolved filesystem API, an escaping
    path, a missing input file, an undeclared, unused or unknown declaration, an
    absent anchor) raises, and callers retain the whole Cargo suite.

    Each declaration attests its reader's `sha256` and each anchor source's
    `source_sha256`. A mismatch (the authoring pull request changed that file
    without confirming its declaration) is returned in `stale` with the crates
    that compile the source, and their Cargo consumers when a library or build
    script compiles it: `numerical_selection` adds them, and
    tests/test_ci_numerical.py fails until the attestation is updated.

    Returns (resources, stale, derived): resources maps each read path to its
    reader crates; stale maps a declaration's source to {'manifests',
    'reasons', 'consumers'}; derived['reads'] gives each declaration's current
    hashes, resolved files, the crates whose library or build script compile it
    outside cfg(test) ('compiled_by') and whether one does ('library');
    derived['scans'] the authored scan rules; derived['listed'] every path a
    declaration lists; derived['literal_named'] each file a Rust string
    literal names (`literal_targets`) with the crates whose sources name it;
    derived['naming_sources'] the Rust sources that name one; and
    derived['declared_crates'] the crates with a runtime-read declaration.
    """
    project = project.resolve()
    declarations = runtime_declarations(project) if declarations is None else declarations
    if not isinstance(declarations, dict) or not {'reads'} <= set(declarations) <= {'source_trees', 'reads'}:
        raise ValueError('Invalid runtime read declaration shape')
    declarations = declarations['reads']
    if not isinstance(declarations, dict):
        raise ValueError('Invalid runtime read declaration tables')
    resources, checked, stale, readers_of = {}, set(), {}, {}
    derived = {'reads': {}, 'scans': [], 'listed': set(), 'literal_named': {}, 'naming_sources': {}}
    dependencies = cargo_dependencies(project, manifests)
    graphs = {m: compile_graph(project, m) for m in manifests}
    def add(target, manifest, required=True):
        target = contained(project, target)
        if required and not target.is_file():
            raise ValueError('Unresolved Rust read: ' + target.relative_to(project).as_posix())
        resources.setdefault(target.relative_to(project).as_posix(), set()).add(manifest)
        return target.relative_to(project).as_posix()
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
            for named in literal_targets(project, crate, source, literals):
                derived['literal_named'].setdefault(named, set()).add(manifest)
                derived['naming_sources'].setdefault(local, set()).add(manifest)
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
            if declaration is None:
                raise ValueError('Missing runtime read declaration: ' + local)
            if not isinstance(declaration, dict) or set(declaration) - DECLARATION_KEYS:
                raise ValueError('Malformed runtime read declaration: ' + local)
            readers_of.setdefault(local, set()).add(manifest)
            if local in checked:
                continue  # compiled by more than one crate: its inputs are indexed once, for every reader below
            checked.add(local)
            reasons = []
            current = hashlib.sha256(source.read_bytes()).hexdigest()
            if declaration.get('sha256') != current:
                reasons.append('reader source changed')
            fingerprints = {'sha256': current, 'inputs': []}
            inputs = declaration.get('inputs')
            external = declaration.get('external_reads')
            if 'external_reads' in declaration and (not isinstance(external, dict)
                    or set(external) != {'kind', 'reason'} or external.get('kind') != 'audited-external-inputs'
                    or not isinstance(external.get('reason'), str) or not external['reason'].strip()):
                raise ValueError('Malformed external-input audit: ' + local)
            if not isinstance(inputs, list) or (not inputs and external is None):
                raise ValueError('Unresolved runtime inputs: ' + local)
            for entry in inputs:
                if not isinstance(entry, dict) or set(entry) - INPUT_KEYS:
                    raise ValueError('Malformed runtime input: ' + local)
                entry_fingerprints = {}
                anchor_source = source
                if entry.get('source'):
                    anchor_source = contained(project, crate / entry['source'])
                    if anchor_source not in seen and anchor_source not in pending:
                        raise ValueError('Runtime anchor is not compiled by ' + local)
                    entry_fingerprints['source_sha256'] = hashlib.sha256(anchor_source.read_bytes()).hexdigest()
                    if entry.get('source_sha256') != entry_fingerprints['source_sha256']:
                        reasons.append('anchor source changed: ' + entry['source'])
                    anchor_literals = {rust_literal(m.group()) for m in RUST_STRING.finditer(rust_source(anchor_source.read_text()))}
                else:
                    anchor_literals = literals
                anchor = entry.get('anchor')
                if not isinstance(anchor, str) or anchor not in anchor_literals:
                    raise ValueError('Runtime anchor absent from source: ' + local)
                bases = {'project': project, 'manifest': crate, 'source': anchor_source.parent}
                base = bases.get(entry.get('base'))
                if base is None:
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
                def in_boundary(target):
                    if '{' in anchor:
                        return re.fullmatch(pattern, str(target)) is not None
                    return target.is_relative_to(boundary) if boundary.is_dir() else target == boundary
                if 'scan' in entry:
                    # An authored rule, checked strictly; its inventory is the tree's.
                    scan = entry['scan']
                    if not _valid_scan(scan):
                        raise ValueError('Malformed runtime source scan: ' + local)
                    excludes = set(scan.get('exclude', []))
                    if (not isinstance(entry.get('audit'), str) or not entry['audit'].strip() or not boundary.is_dir()
                            or not excludes <= anchor_literals):
                        raise ValueError('Unverified runtime source scan: ' + local)
                    rule = {'boundary': boundary.relative_to(project).as_posix(), 'exclude': sorted(excludes),
                            'hidden': bool(scan.get('hidden')), 'name': scan.get('name'), 'suffix': scan.get('suffix'),
                            'manifest': manifest.as_posix()}
                    names = {p.relative_to(project).as_posix() for p in boundary.rglob('*')
                             if p.is_file() and scan_covers(rule, p.relative_to(project).as_posix(), project)}
                    if not names:
                        # A scan that finds nothing has lost its boundary or rule; the author restates it.
                        raise ValueError('Runtime source scan finds no files: ' + local + ' -> ' + anchor)
                    derived['scans'].append(rule)
                else:
                    names = entry.get('files')
                    if not isinstance(names, list) or not names or not all(isinstance(f, str) for f in names):
                        raise ValueError('Unresolved runtime boundary: ' + local)
                if isinstance(entry.get('files'), list):
                    derived['listed'].update(f for f in entry['files'] if isinstance(f, str))
                resolved = set()
                for name in sorted(names):
                    target = contained(project, project / name)
                    if not in_boundary(target):
                        raise ValueError('Runtime file outside source boundary: ' + local + ' -> ' + name)
                    resolved.add(add(target, manifest))
                entry_fingerprints['resolved'] = sorted(resolved)
                fingerprints['inputs'].append(entry_fingerprints)
            derived['reads'][local] = fingerprints
            if reasons:
                stale[local] = {'manifests': set(), 'reasons': reasons, 'consumers': True}
    if set(declarations) != checked:
        raise ValueError('Unknown runtime read declarations: ' + ', '.join(sorted(set(declarations) - checked)))
    # Another crate's reads of a source compiled by more than one: each reader reads its inputs.
    for local, readers in readers_of.items():
        for entry in derived['reads'][local]['inputs']:
            for name in entry['resolved']:
                resources[name] |= readers
    for local, fingerprints in derived['reads'].items():
        source = project / local
        # Crates whose library or build script compile it outside cfg(test); a cfg(test) module is
        # compiled only into its crate's own unit tests, like a test, bench or example.
        compiled_by = {m for m, (compiled, test_only, _, _) in graphs.items() if source in compiled and source not in test_only}
        test_compiled = any(source in test_only for _, test_only, _, _ in graphs.values())
        owner = max((m for m in manifests if source.is_relative_to(project / m.parent)), key=lambda m: len(m.parts), default=None)
        if owner is not None and not graphs[owner][3]:
            compiled_by.add(owner)  # an unresolved module edge: every source of the crate counts
        own = not compiled_by and (test_compiled or (
            owner is not None and source.relative_to(project / owner.parent).parts[0] in OWN_TARGET_DIRS))
        if not own and not compiled_by:
            compiled_by = set(readers_of[local])  # outside the compile graph and the own targets: a library source
        fingerprints['compiled_by'] = sorted(m.as_posix() for m in compiled_by)
        fingerprints['library'] = not own
        if local in stale:
            # The crates that compile it, and, for a library or build-script source, their Cargo consumers.
            stale[local]['manifests'] = set(readers_of[local]) | compiled_by
            stale[local]['consumers'] = not own
    derived['declared_crates'] = set().union(*readers_of.values()) if readers_of else set()
    return resources, stale, derived


# Project roots of the files a Rust literal can name as a runtime resource.
LITERAL_ROOTS = ('fixtures/', 'schemas/', 'examples/', 'validation/')


def literal_targets(project, crate, source, literals):
    """Existing files that Rust string literals name: each literal with a slash, normalized (`./` and `../`
    resolved) against the project, the crate and the source directory, whose project-relative path lies under
    `LITERAL_ROOTS`. A name the filesystem cannot hold is not a file."""
    targets = set()
    for literal in literals:
        if '/' not in literal or '\0' in literal or '\n' in literal or len(literal) > 4096:
            continue
        for base in (project, crate, source.parent):
            target = Path(os.path.normpath(base / literal))
            try:
                if not target.is_relative_to(project):
                    continue
                relative = target.relative_to(project).as_posix()
                if relative.startswith(LITERAL_ROOTS) and target.is_file():
                    targets.add(relative)
            except OSError:
                continue
    return targets


def scan_covers(rule, path, project):
    """Whether an authored scan rule covers a project path."""
    target = Path(os.path.normpath(project / path))
    boundary = project / rule['boundary']
    if not target.is_relative_to(boundary) or target == boundary:
        return False
    parts = target.relative_to(boundary).parts
    if set(rule['exclude']) & set(parts) or (rule['hidden'] and any(part.startswith('.') for part in parts)):
        return False
    return target.name == rule['name'] if rule['name'] else target.suffix == rule['suffix']


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


# No artifact reader declaration was given (direct callers); an explicit None is an invalid declaration.
UNDECLARED = object()


def _initializers(project, tracked, directory):
    """The package initializer positions in `directory` and each ancestor up to the project root, as (present, absent).
    Python runs a package's `__init__.py` while resolving a dotted import through it, and pytest imports a test or
    conftest as part of a package when its directory holds one."""
    parts = directory.split('/') if directory else []
    positions = ['/'.join(parts[:end] + ['__init__.py']) for end in range(len(parts), -1, -1)]
    present = {c for c in positions if c in tracked and (project / c).is_file()}
    return present, set(positions) - present


def _artifact_reader_files(reads, project, tracked, local):
    """One file of a declared artifact reader's load: (its unresolved import candidates, package initializers it runs).

    Stricter than tools/ci/python_test_reads.py, whose Python test selection keeps a superset root rather than
    fail: here an imported module whose top-level name is project-local but which does not resolve raises, and the
    paths at which every other unresolved name would resolve are returned, so that a change deleting one of them
    can be refused. `from M import name` may import an attribute of module M, so M.name need not resolve when M is
    a module file. Each import's package prefixes that are project directories give their initializers: a present
    one is loaded with its own imports, an absent position is returned with the candidates, so that a change that
    adds one (present at the head) selects through the closure and one that deletes it is refused."""
    reader = reads._reader(project, local)
    dirs = reader.search_dirs()
    def resolved(module):
        return next((found for d in dirs if (found := reads._module_file(project, tracked, d, module))), None)
    missing, initializers = set(), set()
    for name in sorted(reader.imports()):
        parts = name.split('.')
        for d in dirs:
            for end in range(1, len(parts)):
                package = '/'.join(([d] if d else []) + parts[:end])
                if package not in tracked or (project / package).is_file():
                    break
                init = package + '/__init__.py'
                (initializers if init in tracked and (project / init).is_file() else missing).add(init)
        if resolved(name):
            continue
        parent = name.rpartition('.')[0]
        found = resolved(parent) if parent else None
        if found and not found.endswith('/__init__.py'):
            continue  # an attribute of a module file
        top = parts[0]
        if any((f'{d}/{top}' if d else top) in tracked for d in dirs) or resolved(top):
            raise ValueError(f'Unresolved local import {name} in {local}')
        stem = name.replace('.', '/')
        missing.update(f'{d}/{c}' if d else c for d in dirs for c in (f'{stem}.py', f'{stem}/__init__.py'))
    return missing, initializers


def artifact_producers(project, manifests, readers, inputs=UNDECLARED):
    """Declared Python readers of artifacts that a crate's own tests write: ({path: manifests}, missing paths,
    resource_crates(path)).

    Each declared reader names the manifests whose tests produce what it reads. These select those crates, so their
    shard writes the artifacts and runs the reader on them: the reader; every conftest.py and package `__init__.py`
    that pytest loads for it (in its directory or an ancestor); every project file in their transitive imports,
    derived by tools/ci/python_test_reads.py at selection time, with the initializers of the packages those imports
    pass through and the initializers' own imports; and each declared input, a file the artifact-fed step uses
    besides them (the dependency list it installs). resource_crates gives the crates a changed path selects as a
    resource those files read: a non-Python path at or under one of the read roots that python_test_reads derives
    for them, whether added, edited or deleted. A root with no known leading directory (`**`, `**/*.json`), which
    stands for a read the derivation cannot place, is not used. The missing paths are where an absent conftest.py, package
    initializer or unresolved imported module would be: a change that deletes one is refused by the caller. A
    declaration that is not a mapping, a reader that is not a tracked Python file under tests/, an input that is not
    a tracked file, an undiscovered manifest, an imported project module that does not resolve or a derivation that
    cannot be made rejects reduced selection."""
    inputs = {} if inputs is UNDECLARED else inputs
    if not isinstance(readers, dict) or not isinstance(inputs, dict):
        raise ValueError('Invalid artifact reader declaration')
    if not readers and not inputs:
        return {}, frozenset(), lambda path: set()
    known = {m.as_posix() for m in manifests}
    def producing_manifests(name, producing):
        if not isinstance(producing, list) or not producing or not all(isinstance(m, str) and m in known for m in producing):
            raise ValueError('Unresolved artifact producer for ' + str(name))
        return {Path(m) for m in producing}
    try:
        reads = load_module('numerical_python_test_reads', HERE / 'python_test_reads.py')
        tracked = reads.tracked(project)
    except Exception as exc:
        raise ValueError(f'Artifact reader imports cannot be derived: {type(exc).__name__}: {exc}') from exc
    # The readers share most of their loads; each file is analysed once per call (the tree cannot change within it).
    producers, missing, resources, analysed = {}, set(), [], {}
    for name, producing in inputs.items():
        if not isinstance(name, str) or name not in tracked or not (project / name).is_file():
            raise ValueError('Unresolved artifact reader input: ' + str(name))
        producers.setdefault(name, set()).update(producing_manifests(name, producing))
    for reader, producing in readers.items():
        if (not isinstance(reader, str) or not reader.startswith('tests/') or not reader.endswith('.py')
                or reader not in tracked or not (project / reader).is_file()):
            raise ValueError('Unresolved artifact reader: ' + str(reader))
        crates = producing_manifests(reader, producing)
        directory = reader.rpartition('/')[0]
        parts = directory.split('/')
        conftests = ['/'.join(parts[:end] + ['conftest.py']) for end in range(len(parts) + 1)]
        loaded = [c for c in conftests if c in tracked and (project / c).is_file()]
        missing.update(c for c in conftests if c not in loaded)
        present, absent = _initializers(project, tracked, directory)
        missing |= absent
        try:
            pending, closure = sorted({reader, *loaded, *present}), set()
            while pending:
                local = pending.pop()
                if local in closure:
                    continue
                closure.add(local)
                roots, loaded_files, imported = reads.file_reads(project, local)
                resources.extend((root, crates) for root in sorted(roots) if not root.startswith('*'))
                if local not in analysed:
                    analysed[local] = _artifact_reader_files(reads, project, tracked, local)
                unresolved, initializers = analysed[local]
                missing |= unresolved
                pending.extend(sorted(set(loaded_files) | set(imported) | initializers))
        except ValueError:
            raise
        except Exception as exc:
            raise ValueError(f'Artifact reader imports cannot be derived for {reader}: {type(exc).__name__}: {exc}') from exc
        for local in closure:
            producers.setdefault(local, set()).update(crates)
    def resource_crates(path):
        if path.endswith('.py'):
            return set()  # Python files the readers load are in the closure
        return set().union(*(crates for root, crates in resources if reads.root_matches(path, root)))
    return producers, frozenset(missing), resource_crates


def numerical_selection(project, manifests, paths, declarations=None, ignored_paths=(), artifact_readers=UNDECLARED,
                        artifact_inputs=UNDECLARED):
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
        resources, stale, derived = runtime_read_index(project, manifests, declarations)
        producers, missing, resource_crates = artifact_producers(project, manifests, {} if artifact_readers is UNDECLARED else artifact_readers,
                                                {} if artifact_inputs is UNDECLARED else artifact_inputs)
    except (ValueError, OSError, KeyError, TypeError, AttributeError, tomllib.TOMLDecodeError) as exc:
        return full(str(exc))
    ignored = set(ignored_paths)
    declared_crates = derived.get('declared_crates', set())
    def declared_dependencies(crates):
        """Crates with a runtime-read declaration in the Cargo dependency closure of `crates`."""
        closure, pending = set(), list(crates)
        while pending:
            for dependency in dependencies.get(pending.pop(), set()) - closure:
                closure.add(dependency)
                pending.append(dependency)
        return (closure - set(crates)) & declared_crates
    # A crate that names a file by a literal may pass it to a reader it depends on, whatever that
    # reader's declaration lists; the reader runs, without its consumers.
    literal_readers = set()
    for path in sorted(set(paths)):
        if not path.startswith(PROJECT):
            if path in ignored:
                reasons.append({'path': path, 'kind': 'mapped-other-checks'})
                continue
            return full('Shared or unknown numerical input: ' + path)
        relative = Path(path[len(PROJECT):])
        if relative.as_posix() in missing and not (project / relative).exists():
            return full('A file a declared artifact reader loads is missing: ' + path)
        if not (project / relative).exists() and (relative.as_posix() in derived.get('listed', ())
                or any(scan_covers(rule, relative.as_posix(), project) for rule in derived.get('scans', ()))):
            return full('A deleted path a runtime read covers: ' + path)
        naming = derived.get('literal_named', {}).get(relative.as_posix(), set())
        for manifest in sorted(naming):
            selected.add(manifest)
            reasons.append({'path': path, 'kind': 'literal-named', 'manifest': manifest.as_posix()})
        naming_crates = naming | derived.get('naming_sources', {}).get(relative.as_posix(), set())
        for manifest in sorted(declared_dependencies(naming_crates)):
            literal_readers.add(manifest)
            reasons.append({'path': path, 'kind': 'literal-reader-dependency', 'manifest': manifest.as_posix()})
        readers = resources.get(relative.as_posix(), set())
        owners = {m for m in manifests if relative.is_relative_to(m.parent)} if relative.suffix == '.rs' or relative.name in {'Cargo.toml', 'Cargo.lock'} else set()
        for manifest in sorted(owners):
            selected.add(manifest)
            reasons.append({'path': path, 'kind': 'cargo-owner', 'manifest': manifest.as_posix()})
        for manifest in sorted(readers):
            selected.add(manifest)
            reasons.append({'path': path, 'kind': 'exact-file-reader', 'manifest': manifest.as_posix()})
        # The artifact-reader rule only adds: the path keeps whatever else it selects, including a fallback.
        for manifest in sorted(producers.get(relative.as_posix(), set()) | resource_crates(relative.as_posix())):
            selected.add(manifest)
            reasons.append({'path': path, 'kind': 'artifact-reader', 'manifest': manifest.as_posix()})
        if owners or readers or naming:
            continue
        if path in ignored:
            reasons.append({'path': path, 'kind': 'mapped-other-checks'})
        elif relative.parts and relative.parts[0] == 'tests' and relative.suffix == '.py' and not relative.name.startswith(('test_ci_', 'test_numerical')):
            reasons.append({'path': path, 'kind': 'python-checks'})
        else:
            return full('Unknown numerical input: ' + path)
    # A declaration whose per-file attestation is stale (this diff changed its reader or anchor
    # source and did not confirm it; tests/test_ci_numerical.py fails until it does) may now read a
    # file it does not list: the crates that compile the source run, with their Cargo consumers
    # unless it is only a crate's own test, bench or example.
    own_tests = set()
    for local, entry in sorted(stale.items()):
        (selected if entry['consumers'] else own_tests).update(entry['manifests'])
        reasons.append({'kind': 'stale-runtime-read', 'source': local,
                        'manifests': [m.as_posix() for m in sorted(entry['manifests'])], 'stale': entry['reasons']})
    while True:
        consumers = {m for m, deps in dependencies.items() if deps & selected}
        added = consumers - selected
        if not added:
            break
        for manifest in sorted(added):
            reasons.append({'kind': 'cargo-consumer', 'manifest': manifest.as_posix(),
                            'dependencies': [m.as_posix() for m in sorted(dependencies[manifest] & selected)]})
        selected.update(added)
    selected |= own_tests | literal_readers
    graph = {'cargo': {m.as_posix(): [d.as_posix() for d in sorted(deps)] for m, deps in dependencies.items()},
             'reads': {p: [m.as_posix() for m in sorted(readers)] for p, readers in sorted(resources.items())},
             'stale': sorted(stale)}
    return {'version': SELECTION_VERSION, 'classification': 'reachability',
            'manifests': [m.as_posix() for m in manifests if m in selected], 'reasons': reasons,
            'fallback_reasons': [], 'graph_sha256': hashlib.sha256(json.dumps(graph, sort_keys=True).encode()).hexdigest()}


def affected_manifests(project, manifests, paths, declarations=None, ignored_paths=()):
    selection = numerical_selection(project, manifests, paths, declarations, ignored_paths)
    return [Path(m) for m in selection['manifests']]


# Named heavy crates keep their complete command groups, including witnesses.
# Every other current or future manifest belongs to the catch-all shard.
SHARDS = ('physics', 'reporting-runner', 'other')
SHARD_MANIFESTS = {
    'physics': frozenset({'core/product_physics/Cargo.toml'}),
    'reporting-runner': frozenset({'core/reporting/result_export/Cargo.toml',
                                   'core/runner/headless/Cargo.toml'}),
}


def manifest_shard(manifest):
    owners = [name for name, manifests in SHARD_MANIFESTS.items() if str(manifest) in manifests]
    if len(owners) > 1:
        raise ValueError('Overlapping numerical shard assignments: ' + str(manifest))
    return owners[0] if owners else 'other'


def cargo_plan(project, paths=None, selection=None, shard=None):
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
    if shard is not None:
        if shard not in SHARDS:
            raise ValueError('Unknown numerical shard: ' + shard)
        chosen = [m for m in chosen if manifest_shard(m) == shard]
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


def run(root, plan, evidence_dir, build_only=False, shard=None, dry_run=False):
    root, evidence_dir = Path(root).resolve(), Path(evidence_dir).resolve()
    evidence_dir.mkdir(parents=True, exist_ok=True)
    evidence = dict(status='failed', head=plan.get('head'), target_base=plan.get('target_base'),
                    plan=plan, commands=[], manifests=[], plan_validated=False,
                    execution='dry-run' if dry_run else ('build-only' if build_only else 'tests'),
                    shard=shard, empty=False)
    code = 1
    try:
        validate_plan(root, plan)
        evidence['plan_validated'] = True
        project = root / PROJECT
        manifests, commands = cargo_plan(project, selection=plan['piping_numerical_selection'], shard=shard)
        if build_only:
            commands = build_commands(commands)
            print('Build-only: compiling selected profiles; no tests execute.', flush=True)
        evidence['manifests'] = [p.as_posix() for p in manifests]
        evidence['input_sha256'] = {p.as_posix(): hashlib.sha256((project / p).read_bytes()).hexdigest()
            for m in manifests for p in (m, m.with_name('Cargo.lock'))}
        evidence['planned_commands'] = commands
        evidence['empty'] = not manifests
        if not manifests or dry_run:
            code = 0
            evidence['status'] = 'planned' if dry_run else 'success'
            print(f"Numerical shard {shard or 'all'}: " + ('empty; no Cargo commands selected.' if not manifests else 'dry run; no commands executed.'), flush=True)
            return code
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
    parser.add_argument('--shard', choices=SHARDS, help='Run only this partition of the validated selection')
    parser.add_argument('--dry-run', action='store_true', help='Validate and record commands without executing them')
    args = parser.parse_args()
    return run(HERE.parents[1], json.loads(Path(args.plan).read_text()), args.evidence_dir, build_only=args.build_only, shard=args.shard, dry_run=args.dry_run)


if __name__ == '__main__':
    raise SystemExit(main())
