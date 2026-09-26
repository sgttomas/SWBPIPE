#!/usr/bin/env python3
"""Isolated fixed-entry helper for the owned ordinary physics evidence reader.

Invoked with Python -I -S. Copies verified source/table bytes to a private
snapshot so relative table reads cannot select a same-named candidate elsewhere.
The verdict is internal physical consistency only, never producer authentication
or numerical accuracy. No network, runtime build, import fallback or target edit.
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

LIMIT = 8 * 1024 * 1024
FORMAT = 'openpipestress.ordinary_physics_consistency_binding/1'
CONTRACT = 'openpipestress.result_semantics/0.3.0/physics-1'
MODULE = 'core/analysis_runs/physics_evidence.py'
TABLE = 'fixtures/results/semantic_contract_v0_3_physics_1.json'
UNITS = 'core/units/src/lib.rs'
PATHS = {MODULE, TABLE, UNITS}


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


def parsed(data):
    require(len(data) <= LIMIT, 'helper JSON byte limit')
    def constant(value):
        raise ValueError('nonfinite JSON constant ' + value)
    return json.loads(data.decode('utf-8'), object_pairs_hook=pairs, parse_constant=constant)


def read_bytes(path):
    with path.open('rb') as stream:
        data = stream.read(LIMIT + 1)
    require(len(data) <= LIMIT, 'helper file byte limit')
    return data


def run(source_root: Path, binding_path: Path, binding_sha256: str, source_bytes: bytes) -> dict:
    require(sys.flags.isolated == 1 and sys.flags.no_site == 1, 'helper requires isolated Python -I -S')
    binding_bytes = read_bytes(binding_path)
    require(digest(binding_bytes) == binding_sha256, 'binding digest mismatch')
    binding = parsed(binding_bytes)
    require(binding.get('format') == FORMAT and binding.get('contract_id') == CONTRACT, 'unsupported physics binding')
    require(binding.get('entrypoint') == 'validate_physics_evidence', 'unsupported validator entrypoint')
    files = binding.get('files')
    require(type(files) is list and len(files) == len(PATHS), 'closed dependency set required')
    paths = [row.get('path') for row in files]
    require(set(paths) == PATHS and len(paths) == len(set(paths)), 'dependency paths differ from closed ordinary reader')
    snapshots = {}
    for row in files:
        require(set(row) == {'path', 'sha256'}, 'unknown dependency binding fields')
        path = (source_root / row['path']).resolve()
        require(path.is_relative_to(source_root.resolve()), 'dependency outside selected candidate root')
        data = read_bytes(path)
        require(digest(data) == row['sha256'], 'candidate dependency digest mismatch: ' + row['path'])
        snapshots[row['path']] = data
    table = parsed(snapshots[TABLE])
    require(table.get('semantic_contract_id') == CONTRACT and table.get('source_schema_version') == '0.2.0', 'semantic table identity mismatch')
    source = parsed(source_bytes)
    require(type(source) is dict and source.get('schema_version') == '0.2.0', 'ordinary raw0.2 required')
    require(source.get('producer') == {'component_name': 'open_pipe_stress_product_physics',
                                     'component_version': '0.2.0', 'semantic_contract_id': CONTRACT}, 'ordinary producer identity required')
    require('source_block_recovery' not in source and 'carrier_evidence' not in source, 'source/composite namespaces refused')
    # The current public entry has only standard-library imports. Its unused
    # transport-metadata helper contains a deferred relative source_blocks import;
    # that route is neither invoked nor made importable in this private snapshot.
    tree = ast.parse(snapshots[MODULE], filename=MODULE)
    imports = []
    for node in tree.body:
        if isinstance(node, ast.Import):
            imports.extend(item.name for item in node.names)
        elif isinstance(node, ast.ImportFrom):
            require(node.level == 0, 'unexpected top-level relative dependency')
            imports.append(node.module)
    allowed = {'__future__', 'math', 'struct', 'json', 'pathlib', 'collections.abc', 'typing'}
    require(set(imports) <= allowed, 'unbound executed module dependency')
    with tempfile.TemporaryDirectory(prefix='ordinary-physics-reader-') as temporary:
        snapshot_root = Path(temporary)
        for name in (MODULE, TABLE):
            target = snapshot_root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(snapshots[name])
        module = types.ModuleType('openpipestress_gate_ordinary_physics')
        module.__file__ = str(snapshot_root / MODULE)
        exec(compile(snapshots[MODULE], module.__file__, 'exec'), module.__dict__)
        module.validate_physics_evidence(source)
        require(read_bytes(snapshot_root / TABLE) == snapshots[TABLE], 'private table snapshot changed')
    origins = {}
    for name in sorted(set(imports) - {'__future__'}):
        imported = sys.modules.get(name)
        origin = getattr(imported, '__file__', None)
        origins[name] = {'origin': origin, 'sha256': digest(read_bytes(Path(origin))) if origin and Path(origin).is_file() else None}
    return {'artifact': 'openpipestress.ordinary_physics_consistency_observation', 'version': '1.0.0',
            'verdict': 'consistent', 'binding_sha256': binding_sha256, 'source_bytes_sha256': digest(source_bytes),
            'contract_id': CONTRACT, 'binding_status': binding.get('status'), 'dependencies': files,
            'entrypoint': 'validate_physics_evidence', 'python': sys.version, 'stdlib_origins': origins,
            'isolation': {'isolated': bool(sys.flags.isolated), 'no_site': bool(sys.flags.no_site),
                          'source_and_table_snapshot': True, 'unit_authority_execution': False},
            'claim': 'internal physical consistency only; no input freshness, producer origin, numerical accuracy or Current qualification'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-root', type=Path, required=True)
    parser.add_argument('--binding', type=Path, required=True)
    parser.add_argument('--binding-sha256', required=True)
    args = parser.parse_args()
    try:
        result = run(args.source_root.resolve(), args.binding.resolve(), args.binding_sha256,
                     sys.stdin.buffer.read(LIMIT + 1))
        print(json.dumps(result, sort_keys=True, allow_nan=False))
    except (OSError, ValueError, TypeError, KeyError, AttributeError, OverflowError, ImportError, RecursionError) as exc:
        print(json.dumps({'artifact': 'openpipestress.ordinary_physics_consistency_observation', 'version': '1.0.0',
                          'verdict': 'refused', 'reason': str(exc), 'claim': 'no qualification'}))
        raise SystemExit(2)


if __name__ == '__main__':
    main()
