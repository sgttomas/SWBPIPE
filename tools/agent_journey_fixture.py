#!/usr/bin/env python3
"""The embedded agent's W9-A journey in the real app window (RT-E1 D4).

DEL-16-06 `Work/agent-journey-scenario.md`. Launch a native-smoke build of the
app (a private copy whose bundle holds the pinned stock supplier) with
`apps/desktop/native-smoke/agent-journey.js` on stdin. The driver works the
window as a person would and clicks Batch review's own Apply as a
qualification stand-in for the person; this controller judges what it saw.

Process discipline is the live-session controller's
(`tools/agent_live_session_fixture.py`), imported, not copied: the app is an
owned Popen; the product's native-smoke agent gate publishes the original
supplier, which is observed by kqueue and joined by an owned anchor before the
controller opens the gate; rescue is the anchor's own group, never a recorded
PID. A standalone preflight (no supplier, no relay) precedes the launch, and
afterwards no app, supplier or helper process may remain.

Two provider modes:
- `stub`: a loopback Responses-API script that replays the W9-A proposal and
  the solve and results calls as model output, so the scenario's plumbing is
  qualified with no model, no network and no key. It qualifies plumbing, not
  model behaviour.
- `live`: a person-configured endpoint and model. With `--key-stdin` the
  controller reads one key line from its own stdin and passes it to the app on
  the app's stdin (`SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY=stdin`); the key is never
  written, logged or put in the plan, the environment or the report.
"""
from __future__ import annotations

import argparse
import fcntl
import hashlib
import http.server
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import threading
import time

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from tools.agent_live_session_fixture import (  # noqa: E402  (the live-session discipline, reused)
    Records, Refusal, Scope, anchor_start, bounded_wait, cleanup_anchor, pid_live,
)

PROJECT = Path(__file__).resolve().parents[1]
FIXTURE = PROJECT / "fixtures" / "model_operations" / "agent_w9a_journey.json"
REFERENCES = PROJECT / "validation" / "references" / "w9_benchmarks" / "references.json"
PIN = PROJECT / "execution" / "PKG-16" / "DEL-16-06" / "Work" / "agent-witness-pin.json"
DRIVER = PROJECT / "apps" / "desktop" / "native-smoke" / "agent-journey.js"
STEPS = ["new blank", "provider and start", "turn 1", "apply", "turn 2", "observe", "stop"]
TURNS = [
    "Build the W9-A L-frame from the blank model as one batch for my review: material W9 steel "
    "(E 200 GPa, G 76.923 GPa, alpha 1.2e-5 per degC), nodes node:a (0,0,0), node:c (4,0,0) and "
    "node:d (4,3,0) m, pipes pipe:a-c and pipe:c-d (OD 0.2 m, wall 0.01 m), anchors at node:a and "
    "node:d, and one thermal load case raising both pipes by 100 degC. Do not solve yet.",
    "I applied the batch. Confirm it was applied, solve the model and report node:c's displacements "
    "and rotations.",
]
NODAL_KINDS = [f"global_nodal_{q}_{axis}" for q in ("displacement", "rotation") for axis in "xyz"]
LINE_LIMIT, OUTPUT_LIMIT = 16384, 2 * 1024 * 1024


def w9a_reference():
    case = next(c for c in json.loads(REFERENCES.read_text())["cases"] if c["id"] == "W9-A-LFRAME-THERMAL")
    return case


# ---------------------------------------------------------------------------
# The scripted provider (stub mode).

def tool_output(item):
    """A function_call_output's tool outcome, parsed from its text."""
    output = item.get("output")
    if isinstance(output, list):
        output = "".join(part.get("text", "") for part in output if isinstance(part, dict))
    if isinstance(output, dict):
        output = output.get("content") if isinstance(output.get("content"), str) else json.dumps(output)
    try:
        return json.loads(output)
    except (TypeError, ValueError):
        return {"unparsed": str(output)[:400]}


def user_text(item):
    content = item.get("content")
    if isinstance(content, str):
        return content
    return "".join(part.get("text", "") for part in content or [] if isinstance(part, dict))


class Script:
    """Replays W9-A from the actual tool outcomes; every call it makes is derived
    from what the product returned, and anything unexpected ends the turn."""

    def __init__(self, turns):
        fixture = json.loads(FIXTURE.read_text())
        self.operations = fixture["routes"]["PF"]["operations"]
        self.turns = turns
        self.captured = []
        self.failures = []

    @staticmethod
    def tool_name(tools, suffix):
        names = [t.get("name") for t in tools or [] if isinstance(t, dict)]
        matches = [n for n in names if isinstance(n, str) and n.endswith(suffix)]
        return matches[0] if len(matches) == 1 else None

    def next_item(self, request):
        items = request.get("input") or []
        users = [i for i, item in enumerate(items) if item.get("type", "message") == "message" and item.get("role") == "user"]
        if not users:
            return self.fail("no user message")
        last = users[-1]
        text = user_text(items[last])
        turn = next((n for n, t in enumerate(self.turns) if t in text), None)
        if turn is None:
            return self.fail("unrecognized user message")
        outputs = [tool_output(item) for item in items[last + 1:] if item.get("type") == "function_call_output"]
        earlier = [tool_output(item) for item in items[:last] if item.get("type") == "function_call_output"]
        tools = request.get("tools")
        name = lambda suffix: self.tool_name(tools, suffix)  # noqa: E731
        if outputs:
            self.captured.append({"turn": turn + 1, "step": len(outputs), "outcome": outputs[-1]})
        if turn == 0:
            return self.turn_one(outputs, name)
        return self.turn_two(outputs, earlier, name)

    def fail(self, why):
        self.failures.append(why)
        return {"type": "message", "text": f"STUB-UNEXPECTED: {why}"}

    def call(self, name, arguments):
        if name is None:
            return self.fail("tool not declared")
        return {"type": "function_call", "name": name, "arguments": arguments}

    def turn_one(self, outputs, name):
        if not outputs:
            return self.call(name("swbpipe_read_model_summary"), {})
        summary = outputs[0]
        if len(outputs) == 1:
            if summary.get("status") != "ok":
                return self.fail("summary not ok")
            basis = summary["basis"]
            return self.call(name("swbpipe_propose_operations"), {
                "workspaceHandle": summary["data"]["workspaceHandle"], "basis": basis,
                "claimedModelHash": basis["modelHash"], "operations": self.operations,
                "assumptions": [{"assumptionId": "a1", "statement": "Every input is the person's stated W9-A input.",
                                 "affectedOperationIds": [op["operation_id"] for op in self.operations]}]})
        proposed = outputs[1]
        if proposed.get("status") != "pending_review":
            return self.fail(f"proposal {proposed.get('status')}")
        return {"type": "message", "text": "I proposed the W9-A build as one batch. Review it in Batch review and apply it if it is right."}

    def turn_two(self, outputs, earlier, name):
        proposal = next((o for o in earlier if o.get("status") == "pending_review"), None)
        if proposal is None:
            return self.fail("no proposal from turn 1")
        if not outputs:
            return self.call(name("swbpipe_read_model_summary"), {})
        summary = outputs[0]
        workspace = summary.get("data", {}).get("workspaceHandle")
        if len(outputs) == 1:
            return self.call(name("swbpipe_observe_proposal"), {
                "workspaceHandle": workspace, "basis": proposal["basis"],
                "proposalHandle": proposal["data"]["proposalHandle"]})
        if len(outputs) == 2:
            if outputs[1].get("status") != "applied":
                return self.fail(f"proposal standing {outputs[1].get('status')}")
            return self.call(name("swbpipe_solve"), {
                "workspaceHandle": workspace, "basis": summary["basis"], "claimedModelHash": summary["basis"]["modelHash"]})
        if len(outputs) == 3:
            if outputs[2].get("status") != "completed":
                return self.fail(f"solve {outputs[2].get('status')}")
            return self.call(name("swbpipe_read_results"), {
                "workspaceHandle": workspace, "basis": outputs[2]["basis"], "runHandle": outputs[2]["data"]["runHandle"],
                "page": {"cursor": None, "limit": 50}, "filter": {"kinds": NODAL_KINDS, "entityRefs": ["node:c"]}})
        if outputs[3].get("status") != "ok":
            return self.fail(f"results {outputs[3].get('status')}")
        values = "; ".join(f"{v['kind']} {v['value']} {v['unit']}" for v in outputs[3]["data"]["values"])
        return {"type": "message", "text": f"The batch is applied and the model solved. node:c: {values}."}


def sse(item, sequence):
    """One Responses-API event stream carrying a single output item."""
    response_id = f"resp_stub_{sequence}"
    if item["type"] == "function_call":
        done = {"type": "function_call", "id": f"fc_stub_{sequence}", "call_id": f"call_stub_{sequence}",
                "name": item["name"], "arguments": json.dumps(item["arguments"]), "status": "completed"}
        added = {**done, "arguments": "", "status": "in_progress"}
        middle = []
    else:
        done = {"type": "message", "id": f"msg_stub_{sequence}", "role": "assistant", "status": "completed",
                "content": [{"type": "output_text", "text": item["text"], "annotations": []}]}
        added = {**done, "content": [], "status": "in_progress"}
        middle = [{"type": "response.output_text.delta", "item_id": done["id"], "output_index": 0,
                   "content_index": 0, "delta": item["text"]}]
    usage = {"input_tokens": 1, "input_tokens_details": {"cached_tokens": 0}, "output_tokens": 1,
             "output_tokens_details": {"reasoning_tokens": 0}, "total_tokens": 2}
    events = [{"type": "response.created", "response": {"id": response_id, "status": "in_progress", "output": []}},
              {"type": "response.output_item.added", "output_index": 0, "item": added}, *middle,
              {"type": "response.output_item.done", "output_index": 0, "item": done},
              {"type": "response.completed", "response": {"id": response_id, "status": "completed", "output": [done], "usage": usage}}]
    return "".join(f"event: {e['type']}\ndata: {json.dumps(e)}\n\n" for e in events).encode()


class StubProvider:
    """127.0.0.1 only; answers POST /v1/responses from the Script and 404 otherwise."""

    def __init__(self, turns):
        self.script = Script(turns)
        self.requests = []
        stub = self

        class Handler(http.server.BaseHTTPRequestHandler):
            protocol_version = "HTTP/1.1"

            def log_message(self, *args):
                pass

            def do_POST(self):
                length = int(self.headers.get("Content-Length") or 0)
                body = self.rfile.read(min(length, 8 * 1024 * 1024))
                record = {"path": self.path, "bytes": length, "authorization": "Authorization" in self.headers}
                stub.requests.append(record)
                if self.path.rstrip("/") != "/v1/responses" or length > 8 * 1024 * 1024:
                    return self.reply(404, b"{}", "application/json")
                try:
                    request = json.loads(body)
                except ValueError:
                    return self.reply(400, b"{}", "application/json")
                item = stub.script.next_item(request)
                record["answer"] = item["type"] if item["type"] == "message" else item["name"]
                self.reply(200, sse(item, len(stub.requests)), "text/event-stream")

            def do_GET(self):
                stub.requests.append({"path": self.path, "method": "GET"})
                self.reply(404, b"{}", "application/json")

            def reply(self, status, body, kind):
                self.send_response(status)
                self.send_header("Content-Type", kind)
                self.send_header("Content-Length", str(len(body)))
                self.send_header("Connection", "close")
                self.end_headers()
                self.wfile.write(body)

        self.server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)

    @property
    def endpoint(self):
        return f"http://127.0.0.1:{self.server.server_address[1]}/v1"

    def __enter__(self):
        self.thread.start()
        return self

    def __exit__(self, *exc):
        self.server.shutdown()
        self.server.server_close()
        self.thread.join(timeout=5)


# ---------------------------------------------------------------------------
# Judging what the window showed.

def entered_value(text):
    match = re.fullmatch(r"Entered: (\S+) (\S+)", (text or "").strip())
    return (float(match.group(1)), match.group(2)) if match else None


def judge(report, reference, operations, captured=None):
    """Named failures; empty means the journey was observed as expected."""
    failures = []
    if report.get("error"):
        failures.append(f"driver: {report['error']}")
    if report.get("steps") != STEPS:
        failures.append(f"steps {report.get('steps')}")
    batch = report.get("batch") or {}
    if not str(batch.get("batch_id", "")).startswith("agent-") or batch.get("batches_shown") != 1:
        failures.append("batch review did not show exactly one agent batch")
    if f"{len(operations)} proposed changes" not in str(batch.get("summary")):
        failures.append(f"batch summary {batch.get('summary')!r}")
    if not batch.get("apply_message"):
        failures.append("no batch receipt after Apply")
    if not report.get("tree") or not all(report["tree"].values()):
        failures.append(f"model tree rows {report.get('tree')}")
    rows = (report.get("results") or {}).get("rows") or []
    shown = {}
    scales = reference["zero_scales"]
    for r in reference["rows"]:
        if r["quantity"] != "nodal_motion" or r["entity"] != "node:c":
            continue
        q = "displacement" if r["component"].startswith("u") else "rotation"
        # The Results section's row id for a nodal motion component, as the product forms it.
        row_id = f"result:disp:{r['entity'].replace(':', '-')}:{r['component']}"
        found = [row for row in rows if row["id"] == row_id]
        parsed = entered_value(found[0]["text"]) if len(found) == 1 else None
        if parsed is None:
            failures.append(f"node:c {r['component']}: {len(found)} rows shown")
            continue
        observed = parsed[0] / 1000.0 if q == "displacement" else parsed[0]
        if (q == "displacement" and parsed[1] != "mm") or (q == "rotation" and parsed[1] != "rad"):
            failures.append(f"node:c {r['component']}: unit {parsed[1]}")
        family = "translation" if q == "displacement" else "rotation"
        tolerance = 1e-9 * (abs(r["value"]) if r["value"] != 0 else abs(scales[family]["value"]))
        shown[found[0]["id"]] = parsed[0]
        if abs(observed - r["value"]) > tolerance:
            failures.append(f"node:c {r['component']}: shown {observed!r}, reference {r['value']!r}")
    if captured is not None:
        outcomes = [c["outcome"] for c in captured]
        observe = next((o for o in outcomes if o.get("status") == "applied"), None)
        solve = next((o for o in outcomes if o.get("status") == "completed"), None)
        results = next((o for o in outcomes if o.get("status") == "ok" and "values" in (o.get("data") or {})), None)
        if not observe or not solve or not results:
            failures.append("stub did not capture applied, completed and results outcomes")
        else:
            if observe["data"]["outcomeModelHash"] != solve["data"]["evaluatedModelHash"]:
                failures.append("solve was not on the applied model's hash")
            shown_hash = ((report.get("results") or {}).get("model_sha256") or "").removeprefix("sha256:")
            if shown_hash != (solve["data"]["evaluatedModelHash"] or {}).get("value"):
                failures.append("the window's solve proof names another model hash")
            for value in results["data"]["values"]:
                if value["id"] in shown and value["value"] != shown[value["id"]]:
                    failures.append(f"agent value {value['id']} differs from the window")
            if len(results["data"]["values"]) != 6:
                failures.append(f"agent read {len(results['data']['values'])} node:c motion values")
    return failures


# ---------------------------------------------------------------------------
# The run.

def free_bytes():
    out = subprocess.run(["/usr/bin/vm_stat"], capture_output=True, text=True, check=True).stdout
    page = int(re.search(r"page size of (\d+) bytes", out).group(1))
    pages = lambda name: int(re.search(rf"^{name}:\s+(\d+)\.", out, re.M).group(1))  # noqa: E731
    return page * (pages("Pages free") + pages("Pages inactive") + pages("Pages speculative"))


def sha256(path):
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def residents(markers):
    """Read-only: processes whose executable lies under one of the run's own paths
    (not an argument naming one, as the wrapper that launched this controller does)."""
    out = subprocess.run(["/bin/ps", "-axo", "pid=,ppid=,pgid=,comm="], capture_output=True, text=True, check=True).stdout
    found = []
    for line in out.splitlines():
        fields = line.split(None, 3)
        if len(fields) == 4 and any(marker in fields[3] for marker in markers):
            found.append(line.strip()[:300])
    return found


class Lines:
    """Bounded stderr lines; a scope record is the gate's only input."""

    def __init__(self, pipe):
        self.pipe, self.lines, self.total, self.error = pipe, [], 0, None
        self.thread = threading.Thread(target=self.read, daemon=True)
        self.thread.start()

    def read(self):
        try:
            while line := self.pipe.readline(LINE_LIMIT + 1):
                self.total += len(line)
                if len(line) > LINE_LIMIT or self.total > OUTPUT_LIMIT:
                    self.error = "stderr_limit"
                    return
                self.lines.append(line.decode("utf-8", "replace").rstrip("\n"))
        except Exception as error:  # noqa: BLE001
            self.error = type(error).__name__

    def scope(self):
        for line in self.lines:
            if line.startswith("[native-smoke-agent-scope] "):
                return json.loads(line.split(" ", 1)[1])
        return None


def preflight(args, root):
    """The live-session fixture's standalone preflight: probes only, no supplier or relay."""
    state = root / "preflight"
    state.mkdir(mode=0o700)
    app = subprocess.Popen([str(args.app), "--native-agent-session-fixture"], stdin=subprocess.PIPE,
                           stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, env={"PATH": "/usr/bin:/bin"})
    records = Records(app.stdout)
    try:
        app.stdin.write(json.dumps({"state": str(state), "model": "preflight", "contextWindow": 32768,
                                    "endpoint": "http://127.0.0.1:9/v1", "key": None, "mode": "preflight",
                                    "processProbe": str(args.process_probe), "boundaryProbe": str(args.boundary_probe)}).encode() + b"\n")
        app.stdin.close()
        app.wait(timeout=120)
        records.finish(time.monotonic() + 5)
    finally:
        if app.poll() is None:
            app.kill()
            app.wait(timeout=5)
    passed = app.returncode == 0 and records.has("preflight_complete")
    return {"passed": passed, "status": app.returncode, "records": [r["record"] for r in records.records]}


def run(args, key, root):
    result = {"passed": False, "mode": args.mode, "startedAt": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
              "appSha256": sha256(args.app), "cleanup": "unknown"}
    supplier = args.app.parents[1] / "Resources" / "agent-supplier"
    pin = json.loads(PIN.read_text())["codex"]["binarySha256"]
    found = [p for p in supplier.glob("codex*") if p.is_file()]
    result["supplierPin"] = {"expected": pin, "observed": [sha256(p) for p in found]}
    if len(found) != 1 or sha256(found[0]) != pin:
        raise Refusal("supplier_not_pinned")
    result["preflight"] = preflight(args, root)
    if not result["preflight"]["passed"]:
        raise Refusal("preflight")
    home, gate, report_path = root / "home", root / "gate", root / "report.json"
    home.mkdir(mode=0o700)
    reference = w9a_reference()
    operations = json.loads(FIXTURE.read_text())["routes"]["PF"]["operations"]
    stub = StubProvider(TURNS) if args.mode == "stub" else None
    app = anchor = observer = lines = None
    sdk = None
    try:
        if stub:
            stub.__enter__()
        plan = {"provider": {"endpoint": stub.endpoint if stub else args.endpoint, "model": args.model,
                             "contextWindow": args.context_window},
                "turns": TURNS, "resultProbe": "node:c",
                "expectTreeRows": [["node", "node:a"], ["node", "node:c"], ["node", "node:d"]],
                "timeoutsMs": {"ui": 60000, "start": 60000, "turn": args.turn_timeout * 1000}}
        env = {"PATH": "/usr/bin:/bin", "HOME": str(home), "SWBPIPE_NATIVE_SMOKE_DRIVER": "-",
               "SWBPIPE_NATIVE_SMOKE_PLAN": json.dumps(plan), "SWBPIPE_NATIVE_SMOKE_REPORT": str(report_path),
               "SWBPIPE_NATIVE_SMOKE_AGENT_GATE": str(gate)}
        if key is not None:
            env["SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY"] = "stdin"
        app = subprocess.Popen([str(args.app)], stdin=subprocess.PIPE, stdout=subprocess.DEVNULL,
                               stderr=subprocess.PIPE, env=env)
        result["app"] = {"pid": app.pid}
        lines = Lines(app.stderr)
        app.stdin.write((key.encode() + b"\n" if key is not None else b"") + DRIVER.read_bytes())
        app.stdin.close()
        key = None
        deadline = time.monotonic() + args.deadline

        def safety():
            if args.cancel_file and args.cancel_file.exists():
                raise Refusal("cancelled")
            if lines.error:
                raise Refusal(lines.error)
            if observer:
                observer.sample()

        while app.poll() is None:
            safety()
            if observer is None and (scope := lines.scope()):
                if scope.get("appPid") != app.pid:
                    raise Refusal("app_identity")
                sdk = scope["sdkPid"]
                result["supplier"] = {"pid": sdk, "scope": scope["scope"]}
                observer = Scope(sdk)
                if observer.exited:
                    raise Refusal("supplier_exited_before_gate")
                anchor = anchor_start(args.process_probe, sdk, app)
                result["anchor"] = {"pid": anchor.pid, "group": sdk}
                descriptor = os.open(gate, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
                with os.fdopen(descriptor, "wb") as handle:
                    handle.write(b"READY\n")
            if time.monotonic() >= deadline:
                raise Refusal("deadline")
            time.sleep(.01)
        result["app"]["status"] = app.returncode
        if app.returncode != 0:
            raise Refusal("app_nonzero")
        if observer is None:
            raise Refusal("supplier_never_published")
        bounded_wait(lambda: observer.sample() and not pid_live(sdk), time.monotonic() + 5, check=safety)
        if observer.status is None or not os.WIFEXITED(observer.status) or os.WEXITSTATUS(observer.status) != 0:
            raise Refusal("supplier_nonzero_or_signal")
        # The anchor has no supplier writer. Retire it via its own Child, not its group.
        anchor.kill()
        anchor.wait(timeout=5)
        anchor.stdin.close()
        anchor.stdout.close()
        anchor = None
        result["cleanup"] = "qualified_native_owned_group"
        report = json.loads(report_path.read_text())
        result["report"] = report
        failures = judge(report, reference, operations, stub.script.captured if stub else None)
        if stub:
            result["stub"] = {"requests": stub.requests, "captured": stub.script.captured, "failures": stub.script.failures}
            failures += [f"stub: {why}" for why in stub.script.failures]
        result["failures"] = failures
        result["passed"] = not failures
    except Exception as error:  # noqa: BLE001
        result["failure"] = str(error) if isinstance(error, Refusal) else type(error).__name__
    finally:
        if app and app.poll() is None:
            if observer is not None and anchor is None:
                result["rescue"] = "supplier_observed_without_anchor"
            app.kill()  # the original owned app only; the anchor below rescues its supplier's group.
            app.wait(timeout=5)
            result.setdefault("app", {})["status"] = app.returncode
        if anchor is not None:
            try:
                cleanup_anchor(anchor)  # the anchor's own group: the original supplier's.
                result["cleanup"] = "rescued_owned_group"
            except Exception:  # noqa: BLE001
                result["rescue"] = "unconfirmed"
        if observer:
            result.setdefault("supplier", {}).update(exitObserved=observer.exited, statusRaw=observer.status, invalid=observer.invalid)
            observer.close()
        if stub:
            stub.__exit__()
        if lines:
            lines.thread.join(timeout=5)
            # The stub run has no secret anywhere, so its full stderr is kept for diagnosis;
            # a live run keeps only the harness's own records.
            kept = lines.lines if args.mode == "stub" else [line for line in lines.lines if line.startswith(("[native-smoke]", "[native-smoke-agent-scope]"))]
            result["stderr"] = [line[:2000] for line in kept[-200:]]
        time.sleep(1)
        remaining = residents([str(args.app.parents[2]), str(home), f"swbpipe-agent-{(result.get('supplier') or {}).get('scope', '-')}"])
        result["residents"] = remaining
        if remaining:
            result["passed"] = False
            result["failure"] = result.get("failure") or "resident_processes"
        result["endedAt"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--app", type=Path, required=True, help="the native-smoke executable inside a private bundle copy")
    parser.add_argument("--process-probe", type=Path, required=True)
    parser.add_argument("--boundary-probe", type=Path, required=True)
    parser.add_argument("--mode", choices=["stub", "live"], required=True)
    parser.add_argument("--endpoint", help="live mode: the provider endpoint")
    parser.add_argument("--model", default="stub-model")
    parser.add_argument("--context-window", type=int, default=32768)
    parser.add_argument("--key-stdin", action="store_true", help="live mode: read one key line from stdin")
    parser.add_argument("--min-free-gib", type=float, default=50.0)
    parser.add_argument("--turn-timeout", type=int, default=600, help="seconds per agent turn")
    parser.add_argument("--deadline", type=int, default=1800, help="seconds for the whole launch")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cancel-file", type=Path)
    args = parser.parse_args()
    if args.mode == "live" and not args.endpoint:
        parser.error("live mode needs --endpoint")
    if args.mode == "stub" and args.key_stdin:
        parser.error("stub mode takes no key")
    key = sys.stdin.readline().rstrip("\n") if args.key_stdin else None
    args.output.mkdir(mode=0o700, parents=False, exist_ok=False)
    root = args.output.resolve()
    for name in ("app", "process_probe", "boundary_probe"):
        setattr(args, name, getattr(args, name).resolve(strict=True))
    # One attempt at a time on this host, whatever the output directory.
    lock = open(Path("/private/tmp") / f"swbpipe-agent-journey-{os.getuid()}.lock", "w")
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        print("another agent journey attempt is running", file=sys.stderr)
        return 2
    free = free_bytes()
    result = {"passed": False, "freeBytesAtStart": free, "minFreeBytes": int(args.min_free_gib * 1024 ** 3)}
    if free < result["minFreeBytes"]:
        result["failure"] = "memory_gate"
    else:
        try:
            result.update(run(args, key, root))
        except Exception as error:  # noqa: BLE001
            result["failure"] = str(error) if isinstance(error, Refusal) else type(error).__name__
    key = None
    (root / "result.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({k: result.get(k) for k in ("passed", "failure", "failures", "residents", "startedAt", "endedAt")}))
    return 0 if result["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
