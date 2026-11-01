#!/usr/bin/env python3
"""One no-key, no-model supplier-only scripted envelope capture. Launch requires separate authority.

Only --supplier and fresh --output are accepted. No real provider, key, model
loader or desktop App is used. Offline tests inject all process/listener/clock
effects; they do not establish actual supplier serialization or cleanup.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import threading
import time

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from tools import agent_host_witness as witness  # noqa: E402
from tools.agent_journey_fixture import Script, StubProvider, TURNS, sse, residents, FIXTURE  # noqa: E402
from tools.agent_journey_fixture import tool_output, user_text  # noqa: E402

PROJECT = Path(__file__).resolve().parents[1]
CATALOG = PROJECT / 'execution/PKG-16/DEL-16-06/Work/agent-capability-catalog.json'
OUTCOMES = PROJECT / 'fixtures/model_operations/agent_w9a_capture_outcomes.json'
RUNTIME = PROJECT / 'apps/desktop/src-tauri/src/embedded_agent/live_runtime.rs'
MACOS = PROJECT / 'apps/desktop/src-tauri/src/embedded_agent/macos.rs'
MANIFEST = PROJECT / 'apps/desktop/src-tauri/Cargo.toml'
MODEL = 'Qwen3.6-35B-A3B-8bit'
BODY_BYTES, TOTAL_BYTES, FRAME_BYTES = 2 * 1024**2, 16 * 1024**2, 128 * 1024
WORK_SECONDS = 285  # reserve fifteen seconds within the proposed five-minute interval for cleanup
EXPECTED = ['swbpipe_read_model_summary', 'swbpipe_propose_operations', 'message',
            'swbpipe_read_model_summary', 'swbpipe_observe_proposal', 'swbpipe_solve',
            'swbpipe_read_results', 'message']
HOST_TOOLS = [name for name in EXPECTED if name != 'message']
DECLARED_TOOL_IDS = {
    'swbpipe_read_model_summary': 'summary', 'swbpipe_read_nodes': 'nodes',
    'swbpipe_read_records': 'records', 'swbpipe_propose_operations': 'propose',
    'swbpipe_observe_proposal': 'observe', 'swbpipe_solve': 'solve', 'swbpipe_read_results': 'results',
}
TOOLS_REFUSALS = frozenset({'request_tools_vector_type', 'request_tools_count', 'request_tools_member_type',
                           'request_tools_name_type', 'request_tools_name_duplicate', 'request_tools_name_set'})
DIAGNOSTIC_MEMBERS = 8
_MISSING = object()
CATEGORIES = ('missing', 'null', 'boolean', 'number', 'string', 'array', 'object', 'other')


class Refusal(witness.WitnessError):
    pass


def encoded(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode()


def sha(data):
    return hashlib.sha256(data).hexdigest()


def unique_object(pairs):
    value = {}
    for key, child in pairs:
        if key in value:
            raise Refusal('request_duplicate_key')
        value[key] = child
    return value


def identity(value):
    if not isinstance(value, str) or not 0 < len(value) <= 160 or any(ord(c) < 32 for c in value):
        raise Refusal('protocol_identity')
    return value


def invalid_constant(value):
    raise Refusal('request_nonfinite')


def declarations():
    if witness.digest(CATALOG) != witness.CATALOG_SHA256:
        raise Refusal('catalog_pin')
    catalog = json.loads(CATALOG.read_text())
    values = [{'type': 'function', 'name': t['name'], 'description': t['description'],
               'inputSchema': witness.expand(t['inputSchema'], catalog['definitions'])} for t in catalog['tools']]
    return json.loads(encoded(values))  # recursive lexical order, as product serde_json without preserve_order


def checkout_selection():
    revision = subprocess.check_output(['git', '-C', str(PROJECT), 'rev-parse', 'HEAD'], text=True, timeout=2).strip()
    dirty = subprocess.check_output(['git', '-C', str(PROJECT), 'status', '--porcelain', '--untracked-files=all'],
                                    text=True, timeout=2)
    if dirty:
        raise Refusal('checkout_not_clean')
    return revision


def instructions():
    return re.search(r'const INSTRUCTIONS:&str="([^"]+)";', RUNTIME.read_text()).group(1)


def client_version():
    return re.search(r'^version\s*=\s*"([^"]+)"', MANIFEST.read_text(), re.M).group(1)


class CaptureNative(witness.Native):
    """Only the fixed capture recipe/frame cap differ; inherited custody/pump/shutdown remain owned."""
    frame_bytes = FRAME_BYTES
    sort_json_keys = True
    ensure_ascii = False

    def configuration(self, model, provider_url, api_key=None):
        if model != MODEL or api_key is not None:
            raise Refusal('capture_no_key_fixed_model')
        body = re.search(r'const CONFIG_BODY: &str = r#"(.*?)"#;', MACOS.read_text(), re.S).group(1)
        # The capture directly reaches a no-auth script, not the product's authenticated relay.
        body = body.replace('env_key = "SWBPIPE_AGENT_RELAY_CAPABILITY"\n', '')
        body = body.replace('name = "SWBPIPE provider"\n',
                            'name = "SWBPIPE provider"\nbase_url = ' + json.dumps(provider_url) + '\n')
        return 'model = ' + json.dumps(model) + '\nmodel_context_window = 32768\n' + body


def no_secrets(value):
    """Defense in depth for a credential-free, denied-real-HOME run; not a general secret scanner."""
    if isinstance(value, dict):
        for key, child in value.items():
            if str(key).casefold() in {'authorization', 'api_key', 'access_token', 'password', 'cookie', 'secret'}:
                raise Refusal('secret_field')
            no_secrets(child)
    elif isinstance(value, list):
        for child in value:
            no_secrets(child)
    elif isinstance(value, str) and re.search(r'(?i)\bbearer\s+\S+|\bsk-[a-zA-Z0-9_-]{12,}', value):
        raise Refusal('secret_text')
    elif isinstance(value, float) and not math.isfinite(value):
        raise Refusal('request_nonfinite')


def category(value):
    """Fixed JSON categories only; never stringify an untrusted value or type name."""
    if value is _MISSING: return 'missing'
    if value is None: return 'null'
    if isinstance(value, bool): return 'boolean'
    if isinstance(value, (int, float)): return 'number'
    if isinstance(value, str): return 'string'
    if isinstance(value, list): return 'array'
    if isinstance(value, dict): return 'object'
    return 'other'


def tools_summary(request, condition):
    """At most eight members; only fixed labels, capped counts and allowlisted identity booleans."""
    if condition not in TOOLS_REFUSALS:
        raise ValueError('unknown tools diagnostic condition')
    vector = request.get('tools', _MISSING) if isinstance(request, dict) else _MISSING
    sample = vector[:DIAGNOSTIC_MEMBERS] if isinstance(vector, list) else []
    result = {'version': 1, 'condition': condition, 'request_type': category(request), 'vector_type': category(vector),
              'member_count': len(sample), 'count_capped': isinstance(vector, list) and len(vector) > DIAGNOSTIC_MEMBERS,
              'member_types': dict.fromkeys(CATEGORIES, 0),
              'type_tags': dict.fromkeys(('function', 'namespace', 'other_string', *CATEGORIES), 0),
              'recognized': dict.fromkeys(DECLARED_TOOL_IDS.values(), False),
              'unknown_name_count': 0, 'unknown_name_types': dict.fromkeys(CATEGORIES, 0)}
    for member in sample:
        result['member_types'][category(member)] += 1
        tag = member.get('type', _MISSING) if isinstance(member, dict) else _MISSING
        tag_id = tag if isinstance(tag, str) and tag in ('function', 'namespace') else ('other_string' if isinstance(tag, str) else category(tag))
        result['type_tags'][tag_id] += 1
        name = member.get('name', _MISSING) if isinstance(member, dict) else _MISSING
        matched = [tool_id for tool, tool_id in DECLARED_TOOL_IDS.items() if isinstance(name, str) and name.endswith(tool)]
        for tool_id in matched:
            result['recognized'][tool_id] = True
        if not matched:
            result['unknown_name_count'] += 1
            result['unknown_name_types'][category(name)] += 1
    return result


class Capture:
    """Single serialized script, eight bodies maximum. Failed bodies are never retained."""
    def __init__(self, output, deadline, clock=time.monotonic):
        self.output, self.deadline, self.clock = output, deadline, clock
        self.script = Script(TURNS)
        self.bodies, self.calls, self.total, self.discovery = [], [], 0, False
        self.failure = None
        self.tools_diagnostic = None
        self.lock = threading.RLock()
        self.expired = threading.Event()
        self.outcomes = json.loads(OUTCOMES.read_text())['outcomes']

    def check(self):
        with self.lock:
            if self.failure:
                raise Refusal(self.failure)
            if self.expired.is_set() or self.clock() >= self.deadline:
                raise Refusal('deadline')

    def fail(self, reason):
        with self.lock:
            self.failure = self.failure or reason

    def final_failure(self):
        with self.lock:
            return self.failure

    def tools_failure_summary(self):
        with self.lock:
            return self.tools_diagnostic  # assigned once; no untrusted strings or nested input are retained

    def refuse_tools(self, condition, request):
        with self.lock:
            if self.tools_diagnostic is None:
                self.tools_diagnostic = tools_summary(request, condition)
        raise Refusal(condition)

    def accept(self, path, headers, stream):
        with self.lock:
            return self._accept(path, headers, stream)

    def _accept(self, path, headers, stream):
        try:
            self.check()
            pairs = list(headers.items())
            lowered = {key.lower(): value for key, value in pairs}
            if len(lowered) != len(pairs):
                raise Refusal('request_headers')
            if any(key in lowered for key in ('authorization', 'proxy-authorization', 'cookie', 'transfer-encoding')):
                raise Refusal('request_headers')
            length = lowered.get('content-length', '')
            if not re.fullmatch(r'[1-9][0-9]{0,7}', length):
                raise Refusal('request_length')
            size = int(length)
            if path != '/v1/responses' or len(self.bodies) >= 8 or size > BODY_BYTES or self.total + size > TOTAL_BYTES:
                raise Refusal('request_bound_or_route')
            body = stream.read(size)
            if len(body) != size:
                raise Refusal('request_truncated')
            request = json.loads(body, object_pairs_hook=unique_object, parse_constant=invalid_constant)
            if not isinstance(request, dict) or request.get('model') != MODEL or request.get('stream') is not True:
                raise Refusal('request_shape')
            no_secrets(request)
            if request.get('previous_response_id') is not None or not isinstance(request.get('input'), list):
                raise Refusal('request_history')
            tools = request.get('tools')
            if not isinstance(tools, list):
                self.refuse_tools('request_tools_vector_type', request)
            if len(tools) != 7:
                self.refuse_tools('request_tools_count', request)
            if any(not isinstance(t, dict) for t in tools):
                self.refuse_tools('request_tools_member_type', request)
            actual = [t.get('name') for t in tools]
            if any(not isinstance(n, str) for n in actual):
                self.refuse_tools('request_tools_name_type', request)
            if len(set(actual)) != 7:
                self.refuse_tools('request_tools_name_duplicate', request)
            for t in declarations():
                if sum(name.endswith(t['name']) for name in actual) != 1:
                    self.refuse_tools('request_tools_name_set', request)
            index = len(self.bodies)
            inputs = request['input']
            last = max(i for i, item in enumerate(inputs) if item.get('role') == 'user')
            last_user = inputs[last]
            if TURNS[0 if index < 3 else 1] not in user_text(last_user):
                raise Refusal('request_turn')
            current = [tool_output(item) for item in inputs[last + 1:] if item.get('type') == 'function_call_output']
            earlier = [tool_output(item) for item in inputs[:last] if item.get('type') == 'function_call_output']
            wanted = self.outcomes[:index] if index < 3 else self.outcomes[2:2 + index - 3]
            if current != wanted or (index >= 3 and earlier != self.outcomes[:2]):
                raise Refusal('request_outcomes')
            item = self.script.next_item(request)
            answer = 'message' if item.get('type') == 'message' else item.get('name', '')
            if self.script.failures or not answer.endswith(EXPECTED[index]):
                raise Refusal('request_sequence')
            response = sse(item, index + 1)
            if len(response) > FRAME_BYTES:
                raise Refusal('response_bound')
            self.check()
            name = f'request-{index + 1:02d}.json'
            with (self.output / name).open('xb') as file:
                file.write(body)
            self.bodies.append({'file': name, 'bytes': size, 'sha256': sha(body)})
            self.total += size
            if item['type'] == 'function_call':
                self.calls.append(item)
            return response
        except Exception as error:
            self.failure = str(error) if isinstance(error, Refusal) else 'request_invalid'
            raise Refusal(self.failure) from None


class CaptureProvider(StubProvider):
    def __init__(self, capture):
        self.capture = capture
        try:
            super().__init__(TURNS)
            # server_close must join all handlers before the final failure latch is read.
            self.server.daemon_threads = False
            self.server.block_on_close = True
        except Exception:
            if getattr(self, 'server', None) is not None:
                self.server.server_close()
            raise

    def __exit__(self, *exc):
        if self.thread.is_alive():
            self.server.shutdown()
        self.server.server_close()
        if self.thread.ident is not None:
            self.thread.join(timeout=5)

    def accept_post(self, handler):
        try:
            handler.connection.settimeout(1)
            body = self.capture.accept(handler.path, handler.headers, handler.rfile)
            handler.reply(200, body, 'text/event-stream')
        except Exception:
            self.capture.fail('request_io')
            try:
                handler.reply(400, b'{}', 'application/json')
            except Exception:
                pass  # fixed failure is already latched; do not print request-related exceptions

    def accept_get(self, handler):
        # Stock supplier's initial discovery in stub14; bounded and no headers/body retained.
        try:
            handler.connection.settimeout(1)
            with self.capture.lock:
                self.capture.check()
                pairs = list(handler.headers.items())
                headers = {key.lower(): value for key, value in pairs}
                if (handler.path != '/' or self.capture.discovery or self.capture.bodies
                        or len(pairs) != len(headers) or headers.get('content-length', '0') != '0'
                        or any(key in {'authorization', 'proxy-authorization', 'cookie', 'transfer-encoding'} for key in headers)):
                    raise Refusal('unexpected_discovery')
                self.capture.discovery = True
            handler.reply(404, b'{}', 'application/json')
        except Refusal as error:
            self.capture.fail(str(error))
        except Exception:
            self.capture.fail('request_io')

    def prepare_connection(self, handler):
        try:
            handler.connection.settimeout(1)  # includes request-line/header parsing, not just the body
        except Exception:
            self.capture.fail('request_io')
            raise

    def on_http_error(self, handler, code):
        self.capture.fail('request_unexpected')  # 400/431 parse errors and 501 unsupported methods

    def after_request(self, handler):
        # BaseHTTPRequestHandler silently returns on idle timeout/EOF without send_error.
        if not handler._swbpipe_dispatched:
            self.capture.fail('request_incomplete')


def drive(native, capture, outcomes, clock=time.monotonic):
    """Actual adapter protocol with injectable stdio/clock; never applies a model operation."""
    def check():
        capture.check()
    rpc_inputs = []
    def request(method, params):
        check()
        rpc_inputs.append({'method': method, 'params': params})
        return native.request(method, params, timeout=min(30, capture.deadline - clock()))
    check()
    request('initialize', {'clientInfo': {'name': 'swbpipe', 'version': client_version()},
                          'capabilities': {'experimentalApi': True}})
    native.send({'method': 'initialized', 'params': {}})
    start = request('thread/start', {'model': MODEL, 'modelProvider': 'swbpipe_relay',
        'cwd': str(native.capture_cwd), 'baseInstructions': instructions(), 'ephemeral': True,
        'sandbox': 'read-only', 'approvalPolicy': 'never', 'environments': [], 'dynamicTools': declarations()})
    thread = identity(start['thread']['id'])
    used_rpc, used_call, records, calls = set(), set(), [], 0
    for turn_index, text in enumerate(TURNS):
        check()
        turn = identity(request('turn/start', {'threadId': thread, 'input': [{'type': 'text', 'text': text}]})['turn']['id'])
        count, messages, frames = 0, [], 0
        while True:
            check()
            try:
                frame = native.receive(min(capture.deadline, clock() + 0.1))
            except witness.WitnessError as error:
                if str(error) == 'Supplier deadline exceeded':
                    continue
                raise
            frames += 1
            if frames > 512:
                raise Refusal('frame_count')
            params = frame.get('params', {})
            if 'id' in frame and 'method' in frame:
                rpc = frame['id']
                if type(rpc) not in (int, str):
                    raise Refusal('host_call_binding')
                if isinstance(rpc, str):
                    identity(rpc)
                if frame['method'] != 'item/tool/call':
                    native.send({'id': rpc, 'error': {'code': -32601, 'message': 'Unsupported request'}})
                    raise Refusal('host_call_binding')
                call = identity(params.get('callId'))
                if (frame['method'] != 'item/tool/call' or type(rpc) not in (int, str) or not isinstance(call, str)
                        or len(call) > 160 or (isinstance(rpc, str) and len(rpc) > 160)
                        or rpc in used_rpc or call in used_call or params.get('threadId') != thread
                        or params.get('turnId') != turn or params.get('namespace') not in (None, '')
                        or calls >= 6 or calls >= len(capture.calls)):
                    raise Refusal('host_call_binding')
                expected = capture.calls[calls]
                if params.get('tool') != expected['name'] or params.get('arguments') != expected['arguments']:
                    raise Refusal('host_call_content')
                used_rpc.add(rpc); used_call.add(call)
                native.send({'id': rpc, 'result': {'success': True, 'contentItems': [
                    {'type': 'inputText', 'text': encoded(outcomes[calls]).decode()}]}})
                records.append({'turn': turn_index + 1, 'tool': params['tool'], 'callId': call, 'rpcId': rpc})
                calls += 1; count += 1
            else:
                item = params.get('item', {})
                if item.get('type') in witness.FORBIDDEN_ITEMS:
                    raise Refusal('builtin_item')
                if params.get('threadId') is None:
                    continue  # informational global notifications, as in product protocol.rs
                if params.get('threadId') != thread:
                    raise Refusal('event_binding')
                if frame.get('method') == 'item/completed' and item.get('type') == 'agentMessage':
                    if params.get('turnId') != turn:
                        raise Refusal('event_binding')
                    messages.append(item.get('text', ''))
                    if len(encoded(messages)) > 8192:
                        raise Refusal('completion_bound')
                if frame.get('method') == 'turn/completed':
                    if params.get('turn', {}).get('id') != turn or params['turn'].get('status') != 'completed':
                        raise Refusal('turn_failed')
                    if count != (2 if turn_index == 0 else 4) or not messages:
                        raise Refusal('turn_incomplete')
                    break
    check()
    if len(capture.bodies) != 8 or calls != 6:
        raise Refusal('capture_incomplete')
    return {'calls': records, 'rpcInputs': rpc_inputs}


def run(binary, output, *, initialize=CaptureNative.__init__, provider_factory=CaptureProvider,
        timer_factory=threading.Timer, clock=time.monotonic, absence=residents):
    """Effects occur only in an authorized explicit invocation; tests inject all effects."""
    started = clock()
    revision = checkout_selection()
    if any((parent / '.git').exists() for parent in (output.resolve(), *output.resolve().parents)):
        raise Refusal('output_in_checkout')
    if not binary.is_file() or witness.digest(binary) != witness.BINARY_SHA256 or output.exists():
        raise Refusal('input_pin_or_output')
    output.mkdir(mode=0o700)
    capture = Capture(output, started + WORK_SECONDS, clock)
    result = {'captureComplete': False, 'standing': 'scripted_envelope_not_live_model_evidence',
              'inputRevision': revision, 'inputCheckoutClean': True,
              'supplierSha256': witness.BINARY_SHA256, 'inputs': {str(p.relative_to(PROJECT)): witness.digest(p)
                  for p in [CATALOG, OUTCOMES, FIXTURE, RUNTIME, MACOS, MANIFEST, Path(__file__),
                            PROJECT / 'tools/agent_host_witness.py', PROJECT / 'tools/agent_journey_fixture.py']}}
    native = CaptureNative.__new__(CaptureNative)
    provider = watchdog = None
    runtime = Path(tempfile.mkdtemp(prefix='swbpipe-d4-capture-', dir='/private/tmp'))
    expired = capture.expired
    def expire():
        expired.set()
        if getattr(native, 'group_owned', False):
            native.stop()  # existing owned unreaped group, never a stored PID
    try:
        watchdog = timer_factory(max(0, capture.deadline - clock()), expire)
        watchdog.start()
        capture.check()
        provider = provider_factory(capture)
        provider.__enter__()
        capture.check()
        initialize(native, binary, runtime, provider.endpoint, MODEL, None)
        native.capture_cwd = runtime / 'work'
        result['configSha256'] = witness.digest(runtime / 'home/config.toml')
        with (output / 'config.toml').open('xb') as stream:
            stream.write((runtime / 'home/config.toml').read_bytes())
        outcomes = json.loads(OUTCOMES.read_text())['outcomes']
        result.update(drive(native, capture, outcomes, clock))
        result['captureComplete'] = not expired.is_set()
    except Exception as error:
        result['failure'] = str(error) if isinstance(error, Refusal) else 'capture_failed'
    finally:
        failures = []
        if getattr(native, 'process', None) is not None:
            try:
                native.close()
                result['supplierExited'] = native.process.returncode is not None
                result['supplierExitStatus'] = native.process.returncode
                if not result['supplierExited']:
                    failures.append('supplier_exit_unconfirmed')
                reader = getattr(native, 'thread', None)
                if reader is not None and reader.is_alive():
                    failures.append('supplier_reader_unconfirmed')
                if native.group_owned:
                    failures.append('supplier_group_unconfirmed')
            except Exception:
                failures.append('supplier_cleanup_unconfirmed')
        if provider is not None:
            try:
                provider.__exit__()
                if provider.thread.is_alive():
                    failures.append('provider_join_unconfirmed')
            except Exception:
                failures.append('provider_cleanup_unconfirmed')
        if watchdog is not None:
            try:
                watchdog.cancel(); watchdog.join(timeout=1)
                if watchdog.is_alive():
                    failures.append('deadline_guard_unconfirmed')
            except Exception:
                failures.append('deadline_guard_unconfirmed')
        result['elapsedSeconds'] = clock() - started
        if result['elapsedSeconds'] >= 300 or expired.is_set():
            failures.append('deadline')
        try:
            result['residents'] = absence([str(binary), str(runtime)])
            if result['residents']:
                failures.append('resident_executable')
        except Exception:
            failures.append('absence_unconfirmed')
        result.update(bodies=capture.bodies, totalBodyBytes=capture.total, cleanupFailures=failures)
        final_failure = capture.final_failure()  # provider.close has joined every handler
        if final_failure:
            result['failure'] = final_failure
            result['captureComplete'] = False
            summary = capture.tools_failure_summary()
            if summary is not None:
                result['toolsDiagnostic'] = summary
        try:
            if checkout_selection() != revision:
                raise Refusal('checkout_changed')
        except Exception:
            result['captureComplete'] = False
            result['failure'] = 'checkout_changed'
        if failures:
            result['captureComplete'] = False
            result['retainedRuntime'] = str(runtime)
            # Retain the path instead of deleting uncertain process custody's environment.
        else:
            shutil.rmtree(runtime)
        if not result['captureComplete']:
            # Failed captures retain only fixed reasons, never partial/untrusted body evidence.
            for body in output.glob('request-*.json'):
                body.unlink()
            result['bodies'] = []
        (output / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--supplier', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    args = parser.parse_args()
    try:
        result = run(args.supplier.resolve(strict=True), args.output)
        print(json.dumps({'captureComplete': result['captureComplete'], 'cleanupFailures': result['cleanupFailures']}))
        return 0 if result['captureComplete'] else 1
    except Exception:
        print('capture refused; no authorization or live evidence inferred', file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
