"""Pure D4 adapter tests. No real subprocess, listener, telemetry, credentials or capture."""
import io
import json
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import Mock

import pytest

from tools import agent_supplier_capture as adapter
from tools import agent_host_witness as witness


class FakeSupplier:
    """In-memory supplier stream which follows Script; every raw body here is a test fixture."""
    def __init__(self, capture):
        self.capture, self.history, self.sent, self.events = capture, [], [], []
        self.turn_index = 0
        self.capture_cwd = Path('/invented/private-cwd')

    def request(self, method, params, **kwargs):
        assert 0 < kwargs['timeout'] <= 30
        if method == 'thread/start':
            assert len(params['dynamicTools']) == 7 and params['environments'] == []
            return {'thread': {'id': 'thread'}}
        if method == 'turn/start':
            self.turn_index += 1
            self.history.append({'role': 'user', 'content': params['input'][0]['text'], 'type': 'message'})
            return {'turn': {'id': f'turn{self.turn_index}'}}
        assert method == 'initialize'
        return {}

    def send(self, frame):
        self.sent.append(frame)
        if 'id' in frame and 'result' in frame:
            self.history.append({'type': 'function_call_output', 'call_id': f'call{len(self.capture.calls)}',
                                 'output': frame['result']['contentItems'][0]['text']})

    def receive(self, deadline):
        if self.events:
            return self.events.pop(0)
        body = adapter.encoded({'model': adapter.MODEL, 'stream': True, 'instructions': adapter.instructions(),
            'tools': [{'type': 'function', 'name': d['name'], 'parameters': d['inputSchema']} for d in adapter.declarations()],
            'input': self.history})
        self.capture.accept('/v1/responses', {'Content-Length': str(len(body))}, io.BytesIO(body))
        turn = f'turn{self.turn_index}'
        if len(self.capture.bodies) in (3, 8):
            text = 'Synthetic test completion, not actual supplier output.'
            self.history.append({'type': 'message', 'role': 'assistant', 'content': text})
            self.events.append({'method': 'turn/completed', 'params': {'threadId': 'thread',
                                'turn': {'id': turn, 'status': 'completed'}}})
            return {'method': 'item/completed', 'params': {'threadId': 'thread', 'turnId': turn,
                    'item': {'type': 'agentMessage', 'text': text}}}
        call = self.capture.calls[-1]
        ident = f'call{len(self.capture.calls)}'
        self.history.append({'type': 'function_call', 'name': call['name'], 'arguments': json.dumps(call['arguments']), 'call_id': ident})
        return {'id': len(self.capture.calls), 'method': 'item/tool/call', 'params': {'threadId': 'thread',
                'turnId': turn, 'callId': ident, 'tool': call['name'], 'arguments': call['arguments']}}


def capture(tmp_path, clock=lambda: 0):
    return adapter.Capture(tmp_path, 285, clock)


def first_request():
    return {'model': adapter.MODEL, 'stream': True,
            'tools': [{'name': d['name']} for d in adapter.declarations()],
            'input': [{'type': 'message', 'role': 'user', 'content': adapter.TURNS[0]}]}


def post(capture, request=None, **headers):
    body = adapter.encoded(request or first_request())
    return capture.accept('/v1/responses', {'Content-Length': str(len(body)), **headers}, io.BytesIO(body))


def test_exact_two_turn_six_call_eight_body_script_is_only_mock_serialization(tmp_path):
    seen = capture(tmp_path)
    result = adapter.drive(FakeSupplier(seen), seen, seen.outcomes, clock=lambda: 0)
    assert len(result['calls']) == 6 and len(seen.bodies) == 8
    assert [x['tool'] for x in result['calls']] == adapter.HOST_TOOLS
    assert [x['method'] for x in result['rpcInputs']] == ['initialize', 'thread/start', 'turn/start', 'turn/start']
    for row in seen.bodies:
        raw = (tmp_path / row['file']).read_bytes()
        assert len(raw) == row['bytes'] and adapter.sha(raw) == row['sha256']
    assert sum(x['bytes'] for x in seen.bodies) == seen.total


@pytest.mark.parametrize('headers', [{'Authorization': 'invented'}, {'Cookie': 'invented'},
                                   {'Proxy-Authorization': 'invented'}, {'Transfer-Encoding': 'chunked'}])
def test_auth_headers_are_rejected_before_read_or_retention(tmp_path, headers):
    class NeverRead:
        def read(self, *a):
            pytest.fail('rejected headers must not read a body')
    with pytest.raises(adapter.Refusal):
        capture(tmp_path).accept('/v1/responses', {'Content-Length': '5', **headers}, NeverRead())
    assert not list(tmp_path.iterdir())


@pytest.mark.parametrize('length', ['', '-1', '0', '1.5', str(adapter.BODY_BYTES + 1)])
def test_bad_and_oversize_lengths_are_refused_without_reading(tmp_path, length):
    with pytest.raises(adapter.Refusal):
        capture(tmp_path).accept('/v1/responses', {'Content-Length': length}, Mock(read=Mock(side_effect=AssertionError())))


@pytest.mark.parametrize('raw', [b'not-json', b'[]', b'{}', b'{"api_key":"invented"}', b'{"temperature":NaN}'])
def test_malformed_or_unexpected_bodies_are_not_retained(tmp_path, raw):
    with pytest.raises(adapter.Refusal):
        capture(tmp_path).accept('/v1/responses', {'Content-Length': str(len(raw))}, io.BytesIO(raw))
    assert not list(tmp_path.iterdir())


def test_aggregate_limit_and_eighth_request_are_independent_of_body_limit(tmp_path):
    seen = capture(tmp_path)
    seen.total = adapter.TOTAL_BYTES
    with pytest.raises(adapter.Refusal):
        post(seen)
    seen = capture(tmp_path)
    seen.bodies = [{}] * 8
    with pytest.raises(adapter.Refusal):
        post(seen)
    assert not list(tmp_path.iterdir())


def test_exact_2mib_and_16mib_boundaries_are_inclusive(tmp_path):
    seen, value = capture(tmp_path), first_request()
    value['instructions'] = ''
    value['instructions'] = 'x' * (adapter.BODY_BYTES - len(adapter.encoded(value)))
    assert len(adapter.encoded(value)) == adapter.BODY_BYTES
    seen.total = adapter.TOTAL_BYTES - adapter.BODY_BYTES
    post(seen, value)
    assert seen.total == adapter.TOTAL_BYTES
    with pytest.raises(adapter.Refusal):
        post(seen)


def test_duplicate_framing_or_json_keys_and_unexpected_outcomes_are_rejected(tmp_path):
    from email.message import Message
    headers = Message()
    headers['Content-Length'] = '5'
    headers['Content-Length'] = '6'
    with pytest.raises(adapter.Refusal):
        capture(tmp_path).accept('/v1/responses', headers, io.BytesIO(b'never read'))
    raw = b'{"model":"a","model":"b"}'
    with pytest.raises(adapter.Refusal, match='duplicate_key'):
        capture(tmp_path).accept('/v1/responses', {'Content-Length': str(len(raw))}, io.BytesIO(raw))
    value = first_request()
    value['input'].append({'type': 'function_call_output', 'output': '{"status":"ok"}'})
    with pytest.raises(adapter.Refusal, match='request_outcomes'):
        post(capture(tmp_path), value)
    assert not list(tmp_path.iterdir())


def test_bounded_response_checked_before_body_is_written(tmp_path, monkeypatch):
    monkeypatch.setattr(adapter, 'sse', lambda *a: b'x' * (adapter.FRAME_BYTES + 1))
    with pytest.raises(adapter.Refusal, match='response_bound'):
        post(capture(tmp_path))
    assert not list(tmp_path.iterdir())


def test_truncation_secret_content_wrong_tool_route_turn_and_outcome_refuse(tmp_path):
    cases = []
    for change in [lambda x:x.update(api_key='invented'), lambda x:x.update(instructions='Bearer invented'),
                   lambda x:x['tools'].pop(), lambda x:x.update(model='other'),
                   lambda x:x['input'][0].update(content=adapter.TURNS[1])]:
        value = first_request(); change(value); cases.append(value)
    for value in cases:
        with pytest.raises(adapter.Refusal):
            post(capture(tmp_path), value)
    with pytest.raises(adapter.Refusal):
        capture(tmp_path).accept('/other', {'Content-Length': '10'}, io.BytesIO(b'not read'))
    with pytest.raises(adapter.Refusal):
        capture(tmp_path).accept('/v1/responses', {'Content-Length': '10'}, io.BytesIO(b'short'))
    assert not list(tmp_path.iterdir())


def test_deadline_and_unexpected_rpc_always_refuse(tmp_path):
    with pytest.raises(adapter.Refusal, match='deadline'):
        post(capture(tmp_path, clock=lambda: 285))
    seen = capture(tmp_path)
    fake = FakeSupplier(seen)
    fake.receive = lambda deadline: {'id': 'bad', 'method': 'command/exec', 'params': {}}
    with pytest.raises(adapter.Refusal):
        adapter.drive(fake, seen, seen.outcomes, clock=lambda: 0)


def test_capture_configuration_selects_production_recipe_except_no_auth_direct_stub():
    native = adapter.CaptureNative.__new__(adapter.CaptureNative)
    text = native.configuration(adapter.MODEL, 'http://127.0.0.1:4321/v1')
    assert 'model_context_window = 32768' in text and 'model_provider = "swbpipe_relay"' in text
    assert 'model_reasoning_effort = "low"' in text and 'env_key' not in text
    assert 'shell_tool = false' in text and 'memories = false' in text
    with pytest.raises(adapter.Refusal):
        native.configuration(adapter.MODEL, 'unused', 'invented-key')


class Timer:
    def __init__(self, seconds, callback):
        assert 0 <= seconds <= 285
        self.callback = callback
    def start(self): pass
    def cancel(self): pass
    def join(self, **kwargs): pass
    def is_alive(self): return False


def runner(tmp_path, monkeypatch, *, broken=False, cleanup_failure=False, timeout=False, resident=False,
           late_handler=False, checkout_changed=False):
    events = []
    binary = tmp_path / 'invented-codex'
    binary.touch()
    real_digest = adapter.witness.digest
    monkeypatch.setattr(adapter.witness, 'digest', lambda p: witness.BINARY_SHA256 if p == binary else real_digest(p))
    def private_runtime(**kwargs):
        runtime = tmp_path / 'mock-runtime'
        runtime.mkdir(mode=0o700)
        return str(runtime)
    monkeypatch.setattr(adapter.tempfile, 'mkdtemp', private_runtime)
    monkeypatch.setattr(adapter, 'checkout_selection', Mock(side_effect=['a' * 40, adapter.Refusal('dirty')])
                        if checkout_changed else lambda: 'a' * 40)
    def initialize(native, binary, root, endpoint, model, key):
        assert key is None
        (root / 'home').mkdir()
        (root / 'home/config.toml').write_text(native.configuration(model, endpoint))
        native.process = SimpleNamespace(returncode=None)
        native.group_owned = True
        def close():
            events.append('close')
            if cleanup_failure: raise RuntimeError('invented cleanup failure')
            native.process.returncode = 0
            native.group_owned = False
        native.close, native.stop = close, lambda: events.append('stop')
        if broken: raise RuntimeError('invented partial startup failure')
        fake = FakeSupplier(holder['capture'])
        native.request, native.send, native.receive = fake.request, fake.send, fake.receive
    holder = {}
    def provider_factory(seen):
        holder['capture'] = seen
        def close():
            events.append('provider-stop')
            if late_handler:
                seen.fail('request_unexpected')  # simulated joined handler's final failure
        return SimpleNamespace(endpoint='http://127.0.0.1:4321/v1', thread=SimpleNamespace(is_alive=lambda: False),
                               __enter__=lambda: events.append('provider-start'), __exit__=close)
    def timer_factory(seconds, callback):
        timer = Timer(seconds, callback)
        if timeout:
            timer.start = callback
        return timer
    result = adapter.run(binary, tmp_path / 'evidence', initialize=initialize, provider_factory=provider_factory,
        timer_factory=timer_factory, clock=lambda: 0,
        absence=lambda markers: ['invented resident'] if resident else [])
    return result, events


def test_run_mock_completion_uses_owned_cleanup_and_retains_only_bounded_fixture_bodies(tmp_path, monkeypatch):
    result, events = runner(tmp_path, monkeypatch)
    assert result['captureComplete'] and result['standing'] == 'scripted_envelope_not_live_model_evidence'
    assert len(result['bodies']) == 8 and result['supplierExited']
    assert events == ['provider-start', 'close', 'provider-stop']
    assert result['inputs'][str(adapter.FIXTURE.relative_to(adapter.PROJECT))] == witness.digest(adapter.FIXTURE)


@pytest.mark.parametrize('case', ['broken', 'cleanup_failure', 'timeout', 'resident', 'late_handler', 'checkout_changed'])
def test_run_failures_cleanup_or_retain_custody_and_never_retain_partial_bodies(tmp_path, monkeypatch, case):
    result, events = runner(tmp_path, monkeypatch, **{case: True})
    assert not result['captureComplete'] and not result['bodies']
    if case == 'timeout':
        assert not events  # the expired guard prevents even provider/supplier startup
    else:
        assert 'close' in events and 'provider-stop' in events
    assert not list((tmp_path / 'evidence').glob('request-*.json'))
    if 'retainedRuntime' in result:
        # This test's only retained path is private mock data, with no running process.
        import shutil
        shutil.rmtree(result['retainedRuntime'])


def test_original_custody_close_handles_partial_reader_startup_without_a_process_probe():
    native = witness.Native.__new__(witness.Native)
    native.stop, native.signal_group = Mock(), Mock()
    native.signal_lock = __import__('threading').Lock()
    native.group_owned, native.thread = True, None
    native.process = SimpleNamespace(wait=Mock(), stdin=io.BytesIO(), stdout=io.BytesIO())
    native.close()
    native.stop.assert_called_once()
    native.signal_group.assert_called_once_with(witness.signal.SIGKILL)
    native.process.wait.assert_called_once_with(timeout=5)
    assert native.group_owned is False


def test_capture_stdIO_has_128k_incoming_and_outgoing_bounds_without_starting_reader():
    native = adapter.CaptureNative.__new__(adapter.CaptureNative)
    native.process = SimpleNamespace(stdout=io.BytesIO(b'x' * (adapter.FRAME_BYTES + 1)))
    native.stop = Mock()
    native.incoming = __import__('queue').Queue()
    native._read()  # direct synchronous call on BytesIO, no native/reader thread
    error = native.incoming.get_nowait()
    assert isinstance(error, witness.WitnessError)
    native.stop.assert_called_once()
    with pytest.raises(witness.WitnessError, match='Outgoing frame'):
        native.send({'text': 'x' * adapter.FRAME_BYTES})


def test_inherited_configuration_still_has_the_historical_8192_and_optional_key_selection():
    native = witness.Native.__new__(witness.Native)
    plain = native.configuration('invented', 'http://127.0.0.1:9/v1')
    keyed = native.configuration('invented', 'http://127.0.0.1:9/v1', 'invented')
    assert 'model_context_window = 8192' in plain and 'env_key' not in plain
    assert keyed.replace('env_key = "SWBPIPE_OMLX_API_KEY"\n', '') == plain


def test_partial_reader_construction_failure_still_owns_cleanup_with_minimal_environment(tmp_path, monkeypatch):
    process = SimpleNamespace(stdin=io.BytesIO(), stdout=io.BytesIO(), wait=Mock())
    popen = Mock(return_value=process)
    reader = Mock(ident=None)
    reader.start.side_effect = RuntimeError('injected reader startup failure')
    monkeypatch.setattr(witness.subprocess, 'Popen', popen)
    monkeypatch.setattr(witness.threading, 'Thread', Mock(return_value=reader))
    native = adapter.CaptureNative.__new__(adapter.CaptureNative)
    native.signal_group = Mock()
    with pytest.raises(RuntimeError):
        native.__init__(Path('/invented/pinned-codex'), tmp_path, 'http://127.0.0.1:4321/v1', adapter.MODEL)
    env = popen.call_args.kwargs['env']
    assert set(env) == {'PATH', 'LANG', 'HOME', 'CODEX_HOME', 'TMPDIR'}
    assert popen.call_args.kwargs['start_new_session'] is True
    native.close()
    assert [call.args[0] for call in native.signal_group.call_args_list] == [witness.signal.SIGTERM, witness.signal.SIGKILL]
    reader.join.assert_not_called()
    process.wait.assert_called_once()


def test_drive_deadline_when_a_supplier_stream_stalls_uses_injected_clock(tmp_path):
    now = [0]
    seen = capture(tmp_path, clock=lambda: now[0])
    fake = FakeSupplier(seen)
    def stalled(deadline):
        now[0] = 285
        raise witness.WitnessError('Supplier deadline exceeded')
    fake.receive = stalled
    with pytest.raises(adapter.Refusal, match='deadline'):
        adapter.drive(fake, seen, seen.outcomes, clock=lambda: now[0])
    assert not seen.bodies


def test_capture_provider_failed_construction_closes_the_mock_server(monkeypatch, tmp_path):
    from tools import agent_journey_fixture as journey
    server = Mock()
    monkeypatch.setattr(journey.http.server, 'ThreadingHTTPServer', Mock(return_value=server))
    monkeypatch.setattr(journey.threading, 'Thread', Mock(side_effect=RuntimeError('injected thread allocation failure')))
    with pytest.raises(RuntimeError):
        adapter.CaptureProvider(capture(tmp_path))
    server.server_close.assert_called_once()


def test_original_stub_provider_post_seam_preserves_its_script_without_a_listener():
    from tools import agent_journey_fixture as journey
    provider = journey.StubProvider.__new__(journey.StubProvider)
    provider.script, provider.requests = journey.Script(journey.TURNS), []
    body = adapter.encoded(first_request())
    handler = SimpleNamespace(path='/v1/responses', headers={'Content-Length': str(len(body))},
                              rfile=io.BytesIO(body), reply=Mock())
    provider.accept_post(handler)
    assert handler.reply.call_args.args[0] == 200
    assert provider.requests[0]['answer'] == 'swbpipe_read_model_summary'


def test_selected_outcome_values_keep_the_frozen_reference_meaning():
    from tools.agent_journey_fixture import w9a_reference, COMPONENT
    outcomes = json.loads(adapter.OUTCOMES.read_text())['outcomes']
    rows = {r['component']: r for r in w9a_reference()['rows'] if r['quantity'] == 'nodal_motion' and r['entity'] == 'node:c'}
    values = outcomes[-1]['data']['values']
    assert len(values) == 6
    for value in values:
        row = rows[COMPONENT[value['kind']]]
        expected = row['value'] * (1000 if row['unit'] == 'm' else 1)
        assert value['value'] == pytest.approx(expected, rel=1e-9, abs=0)


def test_capture_handler_failure_latches_without_logging_or_retaining_a_failed_body(tmp_path):
    provider = adapter.CaptureProvider.__new__(adapter.CaptureProvider)
    provider.capture = capture(tmp_path)
    handler = SimpleNamespace(path='/unexpected', headers={}, rfile=io.BytesIO(),
                              connection=SimpleNamespace(settimeout=Mock()), reply=Mock(side_effect=OSError('invented')))
    provider.accept_post(handler)
    assert provider.capture.failure and not provider.capture.bodies
    assert not list(tmp_path.iterdir())


@pytest.mark.parametrize('headers', [{'Proxy-Authorization': 'invented'}, {'Content-Length': '1'}, {'Transfer-Encoding': 'chunked'}])
def test_discovery_auth_or_unrecorded_body_is_not_admitted(tmp_path, headers):
    provider = adapter.CaptureProvider.__new__(adapter.CaptureProvider)
    provider.capture = capture(tmp_path)
    provider.accept_get(SimpleNamespace(path='/', headers=headers, connection=SimpleNamespace(settimeout=Mock()), reply=Mock()))
    assert provider.capture.final_failure() == 'unexpected_discovery'


@pytest.mark.parametrize('raw_request', [
    b'HEAD / HTTP/1.1\r\nHost: local\r\n\r\n',
    b'OPTIONS / HTTP/1.1\r\nHost: local\r\n\r\n',
    b'PUT / HTTP/1.1\r\nHost: local\r\n\r\n',
    b'DELETE / HTTP/1.1\r\nHost: local\r\n\r\n',
    b'garbage request line\r\n',
    b'GET / HTTP/1.1\r\nX: ' + b'x' * 65537 + b'\r\n\r\n',
    b'', None,
])
def test_http_parser_and_unsupported_methods_latch_through_actual_handler_without_socket(tmp_path, monkeypatch, raw_request):
    from tools import agent_journey_fixture as journey
    selected = {}
    server = SimpleNamespace(serve_forever=Mock(), server_close=Mock(), shutdown=Mock())
    def no_listener(address, handler):
        selected['handler'] = handler
        return server
    monkeypatch.setattr(journey.http.server, 'ThreadingHTTPServer', no_listener)
    provider = adapter.CaptureProvider(capture(tmp_path))
    class MemorySocket:
        def __init__(self):
            self.reader = Mock(readline=Mock(side_effect=TimeoutError())) if raw_request is None else io.BytesIO(raw_request)
            self.output, self.timeouts = bytearray(), []
        def makefile(self, *a, **kw):
            return self.reader
        def sendall(self, data):
            self.output.extend(data)
        def settimeout(self, value):
            self.timeouts.append(value)
    connection = MemorySocket()
    selected['handler'](connection, ('127.0.0.1', 1), server)
    assert provider.capture.final_failure() == ('request_incomplete' if raw_request in (None, b'') else 'request_unexpected')
    assert connection.timeouts == [1]  # installed BEFORE request/header parsing
    assert not list(tmp_path.iterdir())
    assert server.daemon_threads is False and server.block_on_close is True
    provider.__exit__()  # only mocked server, and an unstarted Thread object
    server.server_close.assert_called_once()


def test_discovery_check_and_failure_latch_are_inside_the_same_lock():
    class Guard:
        depth = 0
        def __enter__(self): self.depth += 1
        def __exit__(self, *a): self.depth -= 1
    class GuardedCapture:
        def __init__(self): self.lock, self._discovery, self.failure = Guard(), False, None
        def check(self): assert self.lock.depth > 0
        @property
        def discovery(self):
            assert self.lock.depth > 0
            return self._discovery
        @discovery.setter
        def discovery(self, value):
            assert self.lock.depth > 0
            self._discovery = value
        @property
        def bodies(self):
            assert self.lock.depth > 0
            return []
        def fail(self, reason):
            with self.lock: self.failure = reason
    provider = adapter.CaptureProvider.__new__(adapter.CaptureProvider)
    provider.capture = GuardedCapture()
    handler = SimpleNamespace(connection=SimpleNamespace(settimeout=Mock()), path='/', headers={}, reply=Mock())
    provider.accept_get(handler)
    assert provider.capture.failure is None and provider.capture._discovery is True
    provider.accept_get(handler)
    assert provider.capture.failure == 'unexpected_discovery'


def test_sorted_product_declarations_and_outbound_thread_bytes():
    declarations = adapter.declarations()
    def ordered(value):
        if isinstance(value, dict):
            assert list(value) == sorted(value)
            for child in value.values(): ordered(child)
        elif isinstance(value, list):
            for child in value: ordered(child)
    ordered(declarations)
    native = adapter.CaptureNative.__new__(adapter.CaptureNative)
    native.process = SimpleNamespace(stdin=io.BytesIO())
    native.write_lock = __import__('threading').Lock()
    frame = {'params': {'z': '\u0394', 'dynamicTools': declarations, 'a': 1}, 'method': 'thread/start', 'id': 2}
    native.send(frame)
    raw = native.process.stdin.getvalue()
    assert raw == adapter.encoded(frame) + b'\n'
    assert '\u0394'.encode() in raw


def test_dirty_or_untracked_checkout_is_refused_before_capture(monkeypatch):
    commands = []
    def git(args, **kwargs):
        commands.append(args)
        assert kwargs == {'text': True, 'timeout': 2}
        return 'a' * 40 + '\n' if args[-1] == 'HEAD' else '?? invented-untracked-file\n'
    monkeypatch.setattr(adapter.subprocess, 'check_output', git)
    with pytest.raises(adapter.Refusal, match='checkout_not_clean'):
        adapter.checkout_selection()
    assert '--untracked-files=all' in commands[-1]


def test_dirty_selection_or_output_inside_git_prevents_any_supplier_or_listener_preparation(tmp_path, monkeypatch):
    initialize, provider = Mock(), Mock()
    monkeypatch.setattr(adapter, 'checkout_selection', Mock(side_effect=adapter.Refusal('checkout_not_clean')))
    with pytest.raises(adapter.Refusal, match='checkout_not_clean'):
        adapter.run(tmp_path / 'unused', tmp_path / 'output', initialize=initialize, provider_factory=provider)
    monkeypatch.setattr(adapter, 'checkout_selection', lambda: 'a' * 40)
    with pytest.raises(adapter.Refusal, match='output_in_checkout'):
        adapter.run(tmp_path / 'unused', adapter.PROJECT / 'not-created-capture-output',
                    initialize=initialize, provider_factory=provider)
    initialize.assert_not_called(); provider.assert_not_called()
    assert not (tmp_path / 'output').exists()


def test_global_information_is_ignored_but_foreign_thread_or_unsupported_request_refuses(tmp_path):
    seen = capture(tmp_path)
    fake = FakeSupplier(seen)
    original = fake.receive
    info = [{'method': 'account/rateLimits/updated', 'params': {}}]
    fake.receive = lambda deadline: info.pop() if info else original(deadline)
    assert len(adapter.drive(fake, seen, seen.outcomes, clock=lambda: 0)['calls']) == 6
    other = tmp_path / 'other'; other.mkdir()
    seen = capture(other); fake = FakeSupplier(seen)
    fake.receive = lambda deadline: {'method': 'item/started', 'params': {'threadId': 'foreign'}}
    with pytest.raises(adapter.Refusal, match='event_binding'):
        adapter.drive(fake, seen, seen.outcomes, clock=lambda: 0)
    seen = capture(other); fake = FakeSupplier(seen)
    fake.receive = lambda deadline: {'id': 'unsupported', 'method': 'server/unsupported', 'params': {}}
    # Error response is product-compatible, but the capture is intentionally stricter: fail the extra request.
    fake.send = Mock()
    with pytest.raises(adapter.Refusal, match='host_call_binding'):
        adapter.drive(fake, seen, seen.outcomes, clock=lambda: 0)
    assert fake.send.call_args.args[0]['error']['code'] == -32601
    seen = capture(other); fake = FakeSupplier(seen)
    fake.receive = lambda deadline: {'method': 'item/started', 'params': {'item': {'type': 'commandExecution'}}}
    with pytest.raises(adapter.Refusal, match='builtin_item'):
        adapter.drive(fake, seen, seen.outcomes, clock=lambda: 0)
