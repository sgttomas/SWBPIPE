#!/usr/bin/env python3
"""Isolated fixed-entry helper for the owned load-reference-1 evidence reader.

Invoked with Python -I -S. Copies the verified reader, its inherited physics-1
validator and both semantic tables to a private snapshot, then loads the two
modules as one private package, so a same-named module in another checkout,
the working directory or PYTHONPATH cannot replace them. Only the fixed entry
``validate_load_reference_evidence`` (raw publication) executes, after the
reader's own ``verify_load_reference_table`` has checked the snapshotted table.
The unit authority is bound read-only. The verdict is internal source
consistency only, never producer authentication, numerical accuracy or model
freshness. No network, runtime build, import fallback or target edit.
"""
from __future__ import annotations
import argparse
import ast
import hashlib
import json
from pathlib import Path
import sys
import tempfile
import types

LIMIT = 8 * 1024 * 1024  # bound files (binding, dependencies)
MAX_SOURCE_LIMIT = 64 * 1024 * 1024  # ceiling for the derived raw snapshot, set per run
FORMAT = 'openpipestress.load_reference_consistency_binding/1'
CONTRACT = 'openpipestress.result_semantics/0.3.0/load-reference-1'
ENTRYPOINT = 'validate_load_reference_evidence'
MODULE = 'core/analysis_runs/load_reference_evidence.py'
PHYSICS_MODULE = 'core/analysis_runs/physics_evidence.py'
TABLE = 'fixtures/results/semantic_contract_v0_3_load_reference_1.json'
PHYSICS_TABLE = 'fixtures/results/semantic_contract_v0_3_physics_1.json'
UNITS = 'core/units/src/lib.rs'
PATHS = {MODULE, PHYSICS_MODULE, TABLE, PHYSICS_TABLE, UNITS}
PRODUCER = {'component_name': 'open_pipe_stress_product_physics', 'component_version': '0.2.0',
            'semantic_contract_id': CONTRACT}
# Top-level imports each executed module may make; deferred relative imports
# resolve only inside the private package (source_blocks is deliberately absent).
ALLOWED_IMPORTS = {
    MODULE: {'__future__', 'copy', 'functools', 'hashlib', 'json', 'math', 'struct', 'pathlib',
             'collections.abc', 'typing'},
    PHYSICS_MODULE: {'__future__', 'math', 'struct', 'json', 'pathlib', 'collections.abc', 'typing'},
}
PACKAGE = 'openpipestress_gate_load_reference'


def require(ok, reason):
    if not ok:
        raise ValueError(reason)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def pairs(items):
    result = {}
    for key, value in items:
        require(key not in result, 'duplicate JSON key')
        result[key] = value
    return result


def parsed(data, limit=LIMIT):
    require(len(data) <= limit, 'helper JSON byte limit')
    def constant(value):
        raise ValueError('nonfinite JSON constant ' + value)
    return json.loads(data.decode('utf-8'), object_pairs_hook=pairs, parse_constant=constant)


def read_bytes(path):
    with path.open('rb') as stream:
        data = stream.read(LIMIT + 1)
    require(len(data) <= LIMIT, 'helper file byte limit')
    return data


def top_level_imports(name, data):
    imports = []
    for node in ast.parse(data, filename=name).body:
        if isinstance(node, ast.Import):
            imports.extend(item.name for item in node.names)
        elif isinstance(node, ast.ImportFrom):
            require(node.level == 0, 'unexpected top-level relative dependency: ' + name)
            imports.append(node.module)
    require(set(imports) <= ALLOWED_IMPORTS[name], 'unbound executed module dependency: ' + name)
    return imports


def load_private_package(snapshot_root, snapshots):
    """Execute both verified modules as members of one private package.

    ``__path__`` is empty, so a relative import can only find a member that is
    already registered here; an absent member fails instead of searching disk.
    """
    package = types.ModuleType(PACKAGE)
    package.__path__ = []
    package.__package__ = PACKAGE
    previous = {name: sys.modules.get(name) for name in (PACKAGE, PACKAGE + '.physics_evidence', PACKAGE + '.load_reference_evidence')}
    sys.modules[PACKAGE] = package
    loaded = {}
    for member, path in (('physics_evidence', PHYSICS_MODULE), ('load_reference_evidence', MODULE)):
        module = types.ModuleType(PACKAGE + '.' + member)
        module.__file__ = str(snapshot_root / path)
        module.__package__ = PACKAGE
        sys.modules[module.__name__] = module
        exec(compile(snapshots[path], module.__file__, 'exec'), module.__dict__)
        setattr(package, member, module)
        loaded[member] = module
    return loaded, previous


def run(source_root: Path, binding_path: Path, binding_sha256: str, source_bytes: bytes, source_limit: int = LIMIT) -> dict:
    require(sys.flags.isolated == 1 and sys.flags.no_site == 1, 'helper requires isolated Python -I -S')
    require(type(source_limit) is int and 0 < source_limit <= MAX_SOURCE_LIMIT, 'invalid source byte limit')
    binding_bytes = read_bytes(binding_path)
    require(digest(binding_bytes) == binding_sha256, 'binding digest mismatch')
    binding = parsed(binding_bytes)
    require(binding.get('format') == FORMAT and binding.get('contract_id') == CONTRACT, 'unsupported load-reference binding')
    require(binding.get('entrypoint') == ENTRYPOINT, 'unsupported validator entrypoint')
    files = binding.get('files')
    require(type(files) is list and len(files) == len(PATHS), 'closed dependency set required')
    paths = [row.get('path') if type(row) is dict else None for row in files]
    require(set(paths) == PATHS and len(paths) == len(set(paths)), 'dependency paths differ from closed load-reference reader')
    snapshots = {}
    for row in files:
        require(set(row) == {'path', 'sha256'}, 'unknown dependency binding fields')
        path = (source_root / row['path']).resolve()
        require(path.is_relative_to(source_root.resolve()), 'dependency outside selected candidate root')
        data = read_bytes(path)
        require(digest(data) == row['sha256'], 'candidate dependency digest mismatch: ' + row['path'])
        snapshots[row['path']] = data
    source = parsed(source_bytes, source_limit)
    require(type(source) is dict and source.get('schema_version') == '0.2.0', 'load-reference raw0.2 required')
    require(source.get('producer') == PRODUCER, 'load-reference-1 producer identity required')
    require('source_block_recovery' not in source and 'carrier_evidence' not in source, 'source/composite namespaces refused')
    imports = sorted(set(top_level_imports(MODULE, snapshots[MODULE])) | set(top_level_imports(PHYSICS_MODULE, snapshots[PHYSICS_MODULE])))
    with tempfile.TemporaryDirectory(prefix='load-reference-reader-') as temporary:
        snapshot_root = Path(temporary)
        for name in (MODULE, PHYSICS_MODULE, TABLE, PHYSICS_TABLE):
            target = snapshot_root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(snapshots[name])
        modules, previous = load_private_package(snapshot_root, snapshots)
        try:
            reader = modules['load_reference_evidence']
            table = reader.verify_load_reference_table(read_bytes(snapshot_root / TABLE))
            require(table.get('source_schema_version') == '0.2.0', 'semantic table identity mismatch')
            getattr(reader, ENTRYPOINT)(source)
            for name in (TABLE, PHYSICS_TABLE):
                require(read_bytes(snapshot_root / name) == snapshots[name], 'private table snapshot changed')
        finally:
            for name, module in previous.items():
                if module is None:
                    sys.modules.pop(name, None)
                else:
                    sys.modules[name] = module
    origins = {}
    for name in sorted(set(imports) - {'__future__'}):
        imported = sys.modules.get(name)
        origin = getattr(imported, '__file__', None)
        origins[name] = {'origin': origin, 'sha256': digest(read_bytes(Path(origin))) if origin and Path(origin).is_file() else None}
    return {'artifact': 'openpipestress.load_reference_consistency_observation', 'version': '1.0.0',
            'verdict': 'consistent', 'binding_sha256': binding_sha256, 'source_bytes_sha256': digest(source_bytes),
            'contract_id': CONTRACT, 'binding_status': binding.get('status'), 'dependencies': files,
            'entrypoint': ENTRYPOINT, 'python': sys.version, 'stdlib_origins': origins,
            'isolation': {'isolated': bool(sys.flags.isolated), 'no_site': bool(sys.flags.no_site),
                          'source_and_table_snapshot': True, 'unit_authority_execution': False},
            'claim': 'internal source consistency only; no input freshness, producer origin, numerical accuracy or Current qualification'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-root', type=Path, required=True)
    parser.add_argument('--binding', type=Path, required=True)
    parser.add_argument('--binding-sha256', required=True)
    parser.add_argument('--source-limit-bytes', type=int, default=LIMIT)
    args = parser.parse_args()
    try:
        limit = args.source_limit_bytes if 0 < args.source_limit_bytes <= MAX_SOURCE_LIMIT else LIMIT
        result = run(args.source_root.resolve(), args.binding.resolve(), args.binding_sha256,
                     sys.stdin.buffer.read(limit + 1), args.source_limit_bytes)
        print(json.dumps(result, sort_keys=True, allow_nan=False))
    except (OSError, ValueError, TypeError, KeyError, AttributeError, OverflowError, ImportError, RecursionError) as exc:
        print(json.dumps({'artifact': 'openpipestress.load_reference_consistency_observation', 'version': '1.0.0',
                          'verdict': 'refused', 'reason': str(exc), 'claim': 'no qualification'}))
        raise SystemExit(2)


if __name__ == '__main__':
    main()
