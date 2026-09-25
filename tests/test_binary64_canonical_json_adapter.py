"""Actual Rust authority/Python/stock Node conformance; no product activation."""
from __future__ import annotations

from decimal import Decimal, localcontext
from fractions import Fraction
import hashlib
import json
import math
import os
from pathlib import Path
import random
import struct
import subprocess
import sys
from types import SimpleNamespace

import pytest
from core.serialization.canonical_json import adapter as api


@pytest.fixture(scope='session', autouse=True)
def explicitly_build_test_authority():
    # Test setup owns explicit builds, just as the existing project conftest
    # does for v1. Runtime adapter calls never build or search another profile.
    if os.environ.get(api.ENV_BINARY64_EXECUTABLE):
        yield
        return
    script = Path(__file__).parents[1] / 'tools/serialization/build_checked_json.py'
    built = subprocess.run([sys.executable, str(script), '--profile', api.BINARY64_PROFILE], text=True, capture_output=True, check=True)
    os.environ[api.ENV_BINARY64_EXECUTABLE] = built.stdout.strip()
    try:
        yield
    finally:
        os.environ.pop(api.ENV_BINARY64_EXECUTABLE, None)


def _bits(value: float) -> str:
    return struct.pack('>d', value).hex()


def test_raw_real_tokens_converge_and_preserve_source_distinction() -> None:
    texts = ['9007199254740993', '9007199254740993.0', '9.007199254740993e15']
    results = api.canonicalize_binary64_text_batch([(str(i), text) for i, text in enumerate(texts)])
    assert set(results.values()) == {'9007199254740992'}
    assert api.canonical_json_binary64_v1_text('1e160') == '1e+160'
    assert api.canonical_json_binary64_v1_text('18446744073709551615') == '18446744073709552000'
    raw_a, raw_b = '{"x":9007199254740993}', '{"x":9007199254740992}'
    assert hashlib.sha256(raw_a.encode()).digest() != hashlib.sha256(raw_b.encode()).digest()
    assert api.canonical_sha256_binary64_v1_text(raw_a) == api.canonical_sha256_binary64_v1_text(raw_b)


@pytest.mark.parametrize('text', [
    '-0', '-0.0', '-0e3', '1e-324', '-1e-324', '1e309', 'NaN', 'Infinity',
    '{"a":1,"\\u0061":2}', '{"a":{"x":1,"x":2}}', '"\\ud800"', '"\\udfff"',
    '"\\uffff"', '{"\\ufdd0":0}', '["\\udbff\\udfff"]', '1 trailing', '[1,]', '+1',
])
def test_raw_invalids_reach_rust_and_are_rejected(text: str) -> None:
    with pytest.raises(RuntimeError, match='AUTHORITY-REJECTED'):
        api.canonical_json_binary64_v1_text(text)


def test_programmatic_real_declaration_and_exact_counters() -> None:
    assert api.canonical_json_binary64_v1({'real': api.scientific_real(9007199254740993), 'count': api.exact_integer(4, 0, 10)}) == '{"count":4,"real":9007199254740992}'
    assert api.canonical_json_binary64_v1({'n': 1e160}) == '{"n":1e+160}'
    for value in [True, 1.0, '1', -1, 11, 9007199254740992]:
        with pytest.raises(ValueError):
            api.exact_integer(value, 0, 10)
    for value in [Decimal('1'), Fraction(1), True, 10 ** 1000]:
        with pytest.raises(ValueError):
            api.scientific_real(value)
    with pytest.raises(ValueError):
        api.exact_integer(1, minimum=True)


@pytest.mark.parametrize('value', [
    9007199254740992, -9007199254740992, float('nan'), float('inf'), -0.0,
    {1: 'bad'}, '\ud800', '\uffff', {'\ufdd0': 1}, Decimal('0.1'), Fraction(1, 3),
    (1, 2), {1, 2}, SimpleNamespace(x=1),
])
def test_snapshot_invalids_fail_before_process(value: object, monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setattr(api.subprocess, 'run', lambda *a, **kw: pytest.fail('process should not run'))
    with pytest.raises(ValueError):
        api.canonical_json_binary64_v1(value)


def test_snapshot_cycles_subclasses_and_aliases() -> None:
    class BadDict(dict):
        def items(self):
            pytest.fail('custom method must not be called')
    class BadString(str):
        pass
    for value in [BadDict(x=1), BadString('x'), {BadString('x'): 1}]:
        with pytest.raises(ValueError):
            api.canonical_json_binary64_v1(value)
    cycle: list[object] = []
    cycle.append(cycle)
    with pytest.raises(ValueError, match='CYCLIC'):
        api.canonical_json_binary64_v1(cycle)
    shared = {'x': 1}
    assert api.canonical_json_binary64_v1([shared, shared]) == '[{"x":1},{"x":1}]'


def test_response_mismatch_never_falls_back(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setattr(api, 'binary64_executable', lambda: Path('/explicit-test-authority'))
    for stdout in [
        '{"protocol_version":"1.0.0","profile":"openpipestress_jcs_ijson_v1","items":[]}',
        '{"protocol_version":"1.0.0","profile":"openpipestress_jcs_binary64_v1","items":[{"id":"wrong","canonical_json":"0"}]}',
        '{"protocol_version":"1.0.0","profile":"openpipestress_jcs_binary64_v1","items":[{"id":"value","canonical_json":0}]}',
        '{"profile":"a","profile":"b"}', 'null', '{',
    ]:
        monkeypatch.setattr(api.subprocess, 'run', lambda *a, **kw: SimpleNamespace(returncode=0, stdout=stdout))
        with pytest.raises(RuntimeError, match='RESPONSE-INVALID'):
            api.canonical_json_binary64_v1(0)
    monkeypatch.setattr(api.subprocess, 'run', lambda *a, **kw: SimpleNamespace(returncode=2, stderr='profile rejected'))
    with pytest.raises(RuntimeError, match='AUTHORITY-REJECTED'):
        api.canonical_json_binary64_v1(0)


def test_no_runtime_build_or_old_executable_fallback(monkeypatch: pytest.MonkeyPatch, tmp_path: Path) -> None:
    monkeypatch.setenv(api.ENV_BINARY64_EXECUTABLE, str(tmp_path / 'missing'))
    monkeypatch.setattr(api.subprocess, 'run', lambda *a, **kw: pytest.fail('no build or fallback'))
    with pytest.raises(RuntimeError, match='AUTHORITY-MISSING'):
        api.canonical_json_binary64_v1(0)


def test_text_route_syntax_limits_and_snapshot_budget() -> None:
    assert api.canonical_json_binary64_v1_text(' ' * (api._BINARY64_MAX_BYTES - 1) + '0') == '0'
    with pytest.raises(ValueError, match='BYTE-LIMIT'):
        api.canonical_json_binary64_v1_text(' ' * api._BINARY64_MAX_BYTES + '0')
    for items in [[('', '0')], [('same', '0'), ('same', '1')], [(1, '0')], [('x', 1)], [('\uffff', '0')]]:
        with pytest.raises(ValueError):
            api.canonicalize_binary64_text_batch(items)
    value: object = 0
    for _ in range(api._BINARY64_MAX_DEPTH + 1):
        value = [value]
    with pytest.raises(ValueError, match='DEPTH-LIMIT'):
        api.canonical_json_binary64_v1(value)


def test_all_old_fixture_hashes_remain_on_old_profile() -> None:
    corpus = json.loads((Path(__file__).parents[1] / 'fixtures/canonical_hash/cases.json').read_text())
    old_supported = 0
    for case in corpus['cases']:
        value = json.loads(case['input_json'])
        try:
            actual = api.canonical_json_checked_v1(value)
        except ValueError:
            continue
        assert actual == case['expected_canonical'], case['case_id']
        assert hashlib.sha256(actual.encode()).hexdigest() == case['expected_sha256']
        old_supported += 1
    assert old_supported >= 15
    with pytest.raises(ValueError):
        api.canonical_json_checked_v1(1e160)
    assert api.canonical_json_checked_v1(-0.0) == '0'


def test_deterministic_cross_language_binary64_conformance() -> None:
    # Includes ties, adjacent values, exponent boundaries and extreme finite
    # numbers. Source and seed reproduce the exact tested tokens without a
    # checked-in self-blessed output table.
    tokens = ['0', '0.1', '1e160', '1e20', '1e21', '1e-6', '1e-7', '9007199254740993',
              '18446744073709551615', '2.2250738585072014e-308', '5e-324', '-5e-324',
              '1.7976931348623157e308', '-1.7976931348623157e308']
    randomizer = random.Random(0xB1642026)
    for _ in range(512):
        bits = randomizer.getrandbits(64)
        value = struct.unpack('>d', bits.to_bytes(8, 'big'))[0]
        if math.isfinite(value) and value != 0:
            tokens.append(repr(value))
    with localcontext() as context:
        context.prec = 1200
        for value in [1.0, math.nextafter(1.0, math.inf), 2.0 ** 53, 1e20, 1e-6, 1e-300, 5e-324]:
            next_value = math.nextafter(value, math.inf)
            midpoint = (Decimal.from_float(value) + Decimal.from_float(next_value)) / 2
            tokens.append(str(midpoint))
            step = Decimal(1).scaleb(midpoint.adjusted() - 1100)
            tokens.extend([str(midpoint - step), str(midpoint + step)])
    rust = api.canonicalize_binary64_text_batch([(str(i), token) for i, token in enumerate(tokens)])
    node_script = """
const fs=require('node:fs');
const tokens=JSON.parse(fs.readFileSync(0,'utf8'));
const rows=tokens.map(t=>{const n=JSON.parse(t);const b=Buffer.alloc(8);b.writeDoubleBE(n);return {bits:b.toString('hex'),canonical:JSON.stringify(n)};});
process.stdout.write(JSON.stringify({node:process.version,v8:process.versions.v8,rows}));
"""
    node = subprocess.run(['node', '-e', node_script], input=json.dumps(tokens), text=True, capture_output=True, check=True)
    observed = json.loads(node.stdout)
    for i, token in enumerate(tokens):
        row = observed['rows'][i]
        assert rust[str(i)] == row['canonical'], token
        assert _bits(float(token)) == row['bits'], token
        assert _bits(float(rust[str(i)])) == row['bits'], token
    print(json.dumps({'conformance_tokens': len(tokens), 'random_seed': '0xB1642026', 'node': observed['node'], 'v8': observed['v8'], 'scope': 'Rust native CLI/Python/stock Node; no WASM or solver qualification'}))


def test_actual_cli_rejects_invalid_utf8_and_wrong_profile() -> None:
    invalid = subprocess.run([str(api.binary64_executable())], input=b'\xff', capture_output=True)
    assert invalid.returncode == 2 and invalid.stdout == b'' and b'UTF8' in invalid.stderr
    for change in [{'profile': api.PROFILE}, {'protocol_version': '2.0.0'}]:
        request = {'protocol_version': api.PROTOCOL_VERSION, 'profile': api.BINARY64_PROFILE, 'items': []} | change
        result = subprocess.run([str(api.binary64_executable())], input=json.dumps(request), text=True, capture_output=True)
        assert result.returncode == 2 and not result.stdout and 'MISMATCH' in result.stderr


def test_alias_expansion_rejected_before_snapshot_materialization(monkeypatch: pytest.MonkeyPatch) -> None:
    shared = 'x' * (1024 * 1024)
    original_dumps = api.json.dumps
    def refuse_snapshot_dump(value, *args, **kwargs):
        if type(value) is list:
            pytest.fail('oversized snapshot was serialized before admission')
        return original_dumps(value, *args, **kwargs)
    monkeypatch.setattr(api.json, 'dumps', refuse_snapshot_dump)
    monkeypatch.setattr(api.subprocess, 'run', lambda *args, **kwargs: pytest.fail('process started'))
    with pytest.raises(ValueError, match='BYTE-LIMIT'):
        api.canonical_json_binary64_v1([shared] * 9)


def test_request_expansion_rejected_before_materialization(monkeypatch: pytest.MonkeyPatch) -> None:
    # A small test budget exercises exactly the same cumulative64MiB production
    # path without constructing a64MiB test request. Default limit is pinned.
    assert api._BINARY64_MAX_REQUEST_BYTES == 64 * 1024 * 1024
    monkeypatch.setattr(api, '_BINARY64_MAX_REQUEST_BYTES', 512)
    original_dumps = api.json.dumps
    def refuse_populated_request(value, *args, **kwargs):
        if type(value) is dict and value.get('items'):
            pytest.fail('oversized request was materialized before admission')
        return original_dumps(value, *args, **kwargs)
    monkeypatch.setattr(api.json, 'dumps', refuse_populated_request)
    monkeypatch.setattr(api.subprocess, 'run', lambda *args, **kwargs: pytest.fail('process started'))
    with pytest.raises(ValueError, match='BYTE-LIMIT'):
        api.canonicalize_binary64_text_batch([(str(i), '"' + 'a' * 150 + '"') for i in range(4)])
    with pytest.raises(ValueError, match='BYTE-LIMIT'):
        api.canonicalize_binary64_batch([(str(i), 'a' * 150) for i in range(4)])


def test_preflight_byte_accounting_matches_transport() -> None:
    values = [None, True, False, 0, -1, 1e160, 0.1, 'é😀\x00\n\"\\', [], {}, {'a': [1, 'b'], '😀': False}]
    for value in values:
        state = [0, 0]
        frozen = api._freeze_binary64(value, set(), 0, state)
        assert state[1] == len(json.dumps(frozen, ensure_ascii=False, separators=(',', ':'), allow_nan=False).encode('utf-8'))
    for text in ['', 'é😀\x00\n\"\\', 'x' * 100]:
        expected = len(json.dumps(text, ensure_ascii=False).encode('utf-8'))
        assert api._bounded_string_size(text, expected, escaped=True) == expected
        with pytest.raises(ValueError, match='BYTE-LIMIT'):
            api._bounded_string_size(text, expected - 1, escaped=True)


def test_snapshot_text_and_batch_accepted_outputs_reenter_authority() -> None:
    values = [None, True, False, 0, 1e160, 5e-324,
              {'a/~😀': [api.scientific_real(9007199254740993), 'é', 'e\u0301', '\x00\n"\\'], 'count': api.exact_integer(7, 0, 9)}]
    first = api.canonicalize_binary64_batch([(str(i), value) for i, value in enumerate(values)])
    assert api.canonicalize_binary64_text_batch(list(first.items())) == first
    for key, text in first.items():
        assert api.canonical_json_binary64_v1_text(text) == first[key]
    # Python's bounded snapshot preflight does not qualify structural metadata:
    # final Rust authority still rejects paths exceeding its shared resource cap.
    with pytest.raises(RuntimeError, match='PATH-LIMIT'):
        api.canonical_json_binary64_v1({'k' * (4 * 1024 * 1024): [0, 0, 0, 0]})
