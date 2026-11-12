#!/usr/bin/env python3
"""A named embedded-agent use case in the real app window (DEL-16-06; DEL-07-06 REQ-016).

Launch a native-smoke build of the app (a private copy whose bundle holds the
pinned stock supplier) with `apps/desktop/native-smoke/agent-journey.js` on
stdin. The driver reaches the use case's stated start through the app's own
commands, enters the provider in the Agent panel, types the person's requests
in order and, between them, acts as a stand-in for the person only through the
desktop's own controls (today: Batch review's own Validate and Apply). This
controller judges what the window showed against the use case's expectations.

Use cases are data: `fixtures/embedded_agent/use_cases/<id>/use_case.json`
(start, requests, acts, what to observe, expectations; no provider) and, beside
it, `scripted_provider.json` (model output for stub mode only). The README in
that directory gives the format. `--use-case` selects one (default
`w9a-journey`).

Process discipline is the live-session controller's
(`tools/agent_live_session_fixture.py`), imported, not copied: the app is an
owned Popen; the product's native-smoke agent gate publishes the original
supplier, which is observed by kqueue and joined by an owned anchor before the
controller opens the gate; rescue is the anchor's own group, never a recorded
PID. A standalone preflight (no supplier, no relay) precedes the launch, and
afterwards no app, supplier or helper process may remain.

Two provider modes:
- `stub`: a loopback Responses-API scripted provider replays the use case's
  `scripted_provider.json` as model output, so the route is examined under a
  scripted provider with no model, no network and no key. A stub run says
  nothing about what a model does and is no evidence of the embedded agent
  carrying out a use case.
- `live`: a person-configured endpoint and model; the script is never read.
  With `--key-stdin` the controller reads one key line from its own stdin and
  passes it to the app on the app's stdin
  (`SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY=stdin`); the key is never written, logged
  or put in the plan, the environment or the report.

Running a use case in stub mode on macOS (the repository root is the checkout
under test). Host job limits apply throughout: every build, test and launch
runs inside the host's slot wrapper; pytest uses at most 4 parallel workers
(`-n 4` or fewer); cargo, including the cargo that `npm run build:wasm` and
`npm run build` drive, gets CARGO_BUILD_JOBS=4 and RUST_TEST_THREADS=4 through
the environment.
1. Build from the head under test as `apps/desktop/native-smoke/run.mjs` does,
   without running its journeys: `npm ci`; in `apps/desktop`, `npm run
   build:wasm` and `npm run build`; then in `apps/desktop/src-tauri`, with
   CARGO_TARGET_DIR, CARGO_BUILD_TARGET_DIR and CARGO_BUILD_BUILD_DIR set to
   `src-tauri/target` and TAURI_CONFIG set to the smoke's window override
   (`backgroundThrottling: "disabled"`, as run.mjs forms it), `cargo build
   --locked --offline --features native-smoke,tauri/custom-protocol`.
2. Build the two probes: `clang -O2 -o <dir>/agent_host_process_probe
   tools/agent_host_process_probe.c` and `rustc -O --edition 2021 -o
   <dir>/agent_host_sandbox_probe tools/agent_host_sandbox_probe.rs`.
3. The window takes the foreground of a shared screen, so a launch goes in
   this order: slot first (a job inside the slot wrapper that, once it holds a
   slot, opens nothing and waits for a go signal), then the foreground (agree
   the screen with whoever else uses it), then go. One run per go; a job not
   given its go within its wait exits having launched nothing.
4. On go, first check that the screen session is not locked (`/usr/sbin/ioreg
   -n Root -d1 -a`: no `CGSSessionScreenIsLocked` true; a locked session paints
   no window); if it is locked, launch nothing. Then copy a bundle that
   already holds the pinned supplier (never modify the original; never
   download, re-sign or alter the supplier) to a fresh private directory with
   `ditto`, replace `Contents/MacOS/openpipestress-desktop` in the copy with
   the executable from step 1, and confirm the copy's
   `Contents/Resources/agent-supplier/codex*` has the SHA-256 in
   `fixtures/embedded_agent/agent-witness-pin.json` (the controller refuses
   otherwise).
5. Run `python3 tools/agent_journey_fixture.py --mode stub --use-case <id>
   --app <copy>/Contents/MacOS/openpipestress-desktop --process-probe <probe>
   --boundary-probe <probe> --output <fresh directory>`, bounded with
   `--deadline` and `--release-grace` to the time agreed for the screen;
   `--min-free-gib` lowers the free-memory gate when no model runs. The result
   is `<output>/result.json`; the last line printed summarises it.
6. Confirm with `ps` that nothing runs from the copy or the run's HOME, then
   remove the copy and the output's private `home`.
A run that fails by a timeout while the host is heavily loaded is reported as
a possible load effect, not chased as a defect, until it has also failed on a
quiet host.
"""
from __future__ import annotations

import argparse
import fcntl
import hashlib
import http.server
import json
import math
import os
from pathlib import Path
import re
import subprocess
import sys
import threading
import time
from types import SimpleNamespace

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from tools.agent_live_session_fixture import (  # noqa: E402  (the live-session discipline, reused)
    Records, Refusal, Scope, anchor_start, bounded_wait, cleanup_anchor, pid_live,
)
from tools import agent_journey_memory_monitor as memory_monitor  # noqa: E402

PROJECT = Path(__file__).resolve().parents[1]
USE_CASES = PROJECT / "fixtures" / "embedded_agent" / "use_cases"
PIN = PROJECT / "fixtures" / "embedded_agent" / "agent-witness-pin.json"
DRIVER = PROJECT / "apps" / "desktop" / "native-smoke" / "agent-journey.js"
LINE_LIMIT, OUTPUT_LIMIT = 16384, 2 * 1024 * 1024
# A nodal motion result kind and the component name the product and the report use for it.
COMPONENT = {f"global_nodal_{q}_{axis}": f"{q[0] if q == 'rotation' else 'u'}{axis}"
             for q in ("displacement", "rotation") for axis in "xyz"}


# ---------------------------------------------------------------------------
# Use-case definitions (fixtures/embedded_agent/use_cases/<id>/).
#
# A definition (`use_case.json`) states the start, the person's requests in order with the
# stand-in's acts between them, what to observe in the window, and the expectations the judge
# applies to the window, the conversation and their agreement. It names no provider. The
# scripted provider's model output (`scripted_provider.json`, beside it) is read in stub mode
# only. The format is described in fixtures/embedded_agent/use_cases/README.md.

USE_CASE_SCHEMA = "swbpipe-embedded-agent-use-case/1"
SCRIPT_SCHEMA = "swbpipe-embedded-agent-scripted-provider/1"
DEFAULT_USE_CASE = "w9a-journey"
USE_CASE_ID = re.compile(r"[a-z0-9][a-z0-9-]{0,62}")
# What the driver implements (`STARTS` and `ACTS` in agent-journey.js), with the step name
# each reports. A definition naming any other kind is refused before anything is launched.
START_KINDS = {"new_blank": "new blank"}
ACT_KINDS = {"apply_agent_batch": "apply"}
CATEGORIES = ("window", "conversation", "agreement")
# Requested acts (DEL-16-06 Request): an act the agent asks the person to perform, which the
# person performs at the desktop's own control; in a run the driver stands for the person.
# A request is settled as performed only on the owning route's receipt, so a use case with a
# requested act is unfinished for the agent unless the conversation shows the agent was given
# the standing "performed" for each. Print is never performed: its best standing is "sent to
# the print dialog".
REQUESTABLE_ACTS = ("save_model_state", "record_run", "save_project", "export_table", "save_report",
                    "print_report", "user_release")
# Requested acts the driver can perform at the desktop's own control (`PERFORMS` in
# agent-journey.js). None yet: a definition with a requested act validates, and is refused at
# launch until the driver performs that act.
PERFORMED_BY_DRIVER = {}
PERFORMED = "performed"
REQUEST_STANDINGS = ("pending", "declined", "refused", "stale", "withdrawn", "unknown", PERFORMED,
                     "sent to the print dialog")
# Where a tool outcome observing a request carries the act and its standing. Provisional: the
# catalog defines no request observation yet; follow it when it does.
REQUEST_OBSERVATION = {"act": "data.requestedAct", "standing": "data.requestStanding"}
# The result's one word for the run (`outcome`) and the controller's exit status for each.
# "unfinished": the route was examined and nothing failed, but a requested act was not shown to
# the agent as performed, so the use case is not complete for the agent. It is not a pass.
OUTCOME_EXIT = {"passed": 0, "failed": 1, "unfinished": 3}
# A definition states the person's side only; which provider answers is the run's choice.
PROVIDER_KEYS = {"provider", "endpoint", "model", "apiKey", "key", "contextWindow", "stub", "scripted_provider"}
STANDING = {
    "stub": "the route is examined under a scripted provider; no model ran",
    "live": "observed with a person-configured provider",
}


class Unresolved(Exception):
    """A reference in a definition or script that names nothing."""


def pluck(value, path):
    """Follow `path` (keys, list indices, "*" to map over a list) from `value`."""
    try:
        for index, segment in enumerate(path):
            if segment == "*":
                return [pluck(item, path[index + 1:]) for item in value]
            value = value[segment]
        return value
    except (KeyError, IndexError, TypeError) as error:
        raise Unresolved(f"path {path}") from error


def project_file(name):
    """A repository-relative file that must exist inside the repository."""
    if not isinstance(name, str) or not name or Path(name).is_absolute() or ".." in Path(name).parts:
        raise Unresolved(f"file {name!r} is not a repository path")
    path = PROJECT / name
    if not path.is_file():
        raise Unresolved(f"file {name} does not exist")
    return path


def provider_keys(value, where="$"):
    found = []
    if isinstance(value, dict):
        for key, item in value.items():
            if key in PROVIDER_KEYS:
                found.append(f"{where}.{key}")
            found += provider_keys(item, f"{where}.{key}")
    elif isinstance(value, list):
        for index, item in enumerate(value):
            found += provider_keys(item, f"{where}[{index}]")
    return found


class UseCase:
    """A validated definition and what the controller and driver derive from it."""

    def __init__(self, directory, definition):
        self.directory, self.definition = directory, definition
        self.id = definition["id"]
        self.expectations = definition["expectations"]
        self.requests = [entry["request"] for entry in definition["sequence"] if "request" in entry]
        self.sequence, requests, applies, requested = [], 0, 0, 0
        for entry in definition["sequence"]:
            if "request" in entry:
                requests += 1
                self.sequence.append({"step": f"turn {requests}", "report": f"turn{requests}", "request": entry["request"]})
            elif "requested_act" in entry:
                requested += 1
                self.sequence.append({"step": f"perform {entry['requested_act']}", "requestedAct": entry["requested_act"],
                                      "report": requested_report_key(requested)})
            else:
                applies += entry["act"] == "apply_agent_batch"
                self.sequence.append({"step": ACT_KINDS[entry["act"]], "act": entry["act"],
                                      "report": apply_report_key(applies) if entry["act"] == "apply_agent_batch" else entry["act"]})
        start = definition["start"]["kind"]
        self.steps = [START_KINDS[start], "provider and start", *[e["step"] for e in self.sequence], "observe", "stop"]

    @property
    def requested_acts(self):
        return [entry["requestedAct"] for entry in self.sequence if "requestedAct" in entry]

    @property
    def path(self):
        return self.directory / "use_case.json"

    @property
    def script_path(self):
        return self.directory / "scripted_provider.json"

    def reference(self, name):
        """A named reference: a file path within a file, or a benchmark case of a reference file."""
        ref = self.definition["references"][name]
        data = json.loads(project_file(ref["file"]).read_text())
        if "case" in ref:
            matches = [case for case in data["cases"] if case.get("id") == ref["case"]]
            if len(matches) != 1:
                raise Unresolved(f"reference {name}: case {ref['case']}")
            return matches[0]
        return pluck(data, ref.get("path", []))

    def plan(self, provider, timeouts):
        """What the driver is given: no expectation, reference or script reaches the window."""
        observe = self.definition["observe"]
        return {"useCase": self.id, "provider": provider,
                "start": {"kind": self.definition["start"]["kind"], "step": self.steps[0]},
                "sequence": self.sequence,
                "observe": {"treeTypes": sorted(observe["tree_types"]), "resultProbe": observe["result_filter"]},
                "timeoutsMs": timeouts}


def apply_report_key(occurrence):
    return "batch" if occurrence == 1 else f"batch_{occurrence}"


def requested_report_key(occurrence):
    return f"requested{occurrence}"


def refuse_unperformable(case):
    """At launch: a requested act the driver cannot yet perform at the desktop's control."""
    missing = [act for act in case.requested_acts if act not in PERFORMED_BY_DRIVER]
    if missing:
        raise Refusal(f"requested_act_not_performable {case.id}: {', '.join(missing)}")


def use_case_names():
    return sorted(p.parent.name for p in USE_CASES.glob("*/use_case.json"))


def load_use_case(name):
    """The named definition, validated; any problem refuses before anything is launched."""
    if not isinstance(name, str) or not USE_CASE_ID.fullmatch(name):
        raise Refusal(f"use_case_name {name!r}")
    directory = USE_CASES / name
    try:
        definition = json.loads((directory / "use_case.json").read_text())
    except (OSError, ValueError) as error:
        raise Refusal(f"use_case_unreadable {name}: {type(error).__name__}") from None
    problems = validate_definition(definition, name)
    if problems:
        raise Refusal(f"use_case_invalid {name}: " + "; ".join(problems))
    return UseCase(directory, definition)


def validate_definition(definition, name=None):
    """Problems with a definition; empty means the controller, judge and driver can use it."""
    if not isinstance(definition, dict):
        return ["not an object"]
    problems = []
    extra = set(definition) - {"schema", "id", "summary", "data", "start", "sequence", "observe", "references", "expectations"}
    if extra:
        problems.append(f"unknown members {sorted(extra)}")
    if definition.get("schema") != USE_CASE_SCHEMA:
        problems.append(f"schema {definition.get('schema')!r}")
    if not isinstance(definition.get("id"), str) or not USE_CASE_ID.fullmatch(definition["id"]) or (name and definition["id"] != name):
        problems.append(f"id {definition.get('id')!r} is not the directory name {name!r}")
    problems += [f"{where} names a provider" for where in provider_keys(definition)]
    start = definition.get("start")
    if not isinstance(start, dict) or set(start) != {"kind"} or start.get("kind") not in START_KINDS:
        problems.append(f"start {start!r}: the driver implements {sorted(START_KINDS)}")
    sequence = definition.get("sequence")
    if not isinstance(sequence, list) or not sequence:
        return problems + ["sequence is empty"]
    requests, applies, requested = [], 0, []
    for index, entry in enumerate(sequence):
        if isinstance(entry, dict) and set(entry) == {"request"} and isinstance(entry["request"], str) and entry["request"].strip():
            requests.append(entry["request"])
        elif isinstance(entry, dict) and set(entry) == {"act"} and entry["act"] in ACT_KINDS:
            applies += entry["act"] == "apply_agent_batch"
        elif isinstance(entry, dict) and set(entry) == {"requested_act"} and entry["requested_act"] in REQUESTABLE_ACTS:
            requested.append(entry["requested_act"])
        else:
            problems.append(f"sequence[{index}] {entry!r}: a request text, an act the driver implements ({sorted(ACT_KINDS)}) "
                            f"or a requested act ({', '.join(REQUESTABLE_ACTS)})")
    if not requests:
        problems.append("sequence has no request")
    # The scripted provider and the controller recognise a request by its text.
    for a, first in enumerate(requests):
        for b, second in enumerate(requests):
            if a != b and first in second:
                problems.append(f"request {a + 1} is contained in request {b + 1}")
    observe = definition.get("observe")
    if (not isinstance(observe, dict) or set(observe) != {"tree_types", "result_filter"}
            or not isinstance(observe.get("tree_types"), list) or not all(isinstance(t, str) and t for t in observe["tree_types"])
            or not isinstance(observe.get("result_filter"), str)):
        problems.append(f"observe {observe!r}")
        observe = {"tree_types": [], "result_filter": ""}
    references = definition.get("references")
    if not isinstance(references, dict):
        problems.append("references is not an object")
        references = {}
    for ref_name, ref in references.items():
        try:
            if not isinstance(ref, dict) or set(ref) - {"file", "path", "case"} or ("path" in ref and "case" in ref):
                raise Unresolved("a file with a path or a case")
            UseCase.reference(SimpleNamespace(definition=definition), ref_name)
        except (Unresolved, ValueError, KeyError) as error:
            problems.append(f"reference {ref_name}: {error}")
    expectations = definition.get("expectations")
    if not isinstance(expectations, list) or not expectations:
        return problems + ["no expectations"]
    seen, covered = {}, set()
    context = {"requests": len(requests), "applies": applies, "requested": requested, "references": references, "observe": observe}
    for index, expectation in enumerate(expectations):
        if not isinstance(expectation, dict):
            problems.append(f"expectations[{index}] is not an object")
            continue
        ident, kind = expectation.get("id"), expectation.get("kind")
        check = CHECKS.get(kind)
        if not isinstance(ident, str) or not ident or ident in seen:
            problems.append(f"expectations[{index}] id {ident!r} is missing or repeated")
        if check is None:
            problems.append(f"expectation {ident}: kind {kind!r} is not one the judge implements")
            continue
        if expectation.get("on") != check.category:
            problems.append(f"expectation {ident}: {kind} judges the {check.category}, not {expectation.get('on')!r}")
        params = set(expectation) - {"id", "kind", "on"}
        if params != set(check.params):
            problems.append(f"expectation {ident}: parameters {sorted(params)}, {kind} takes {sorted(check.params)}")
            continue
        for problem in check.validate(expectation, context, seen):
            problems.append(f"expectation {ident}: {problem}")
        seen[ident] = expectation
        if not check.needs_tool_outcomes:
            covered.add(check.category)
    # Each of the window, the conversation and their agreement must be judged in every mode,
    # so that no definition can pass with one of them unexamined.
    for category in CATEGORIES:
        if category not in covered:
            problems.append(f"no expectation judges the {category} in every mode")
    # A requested act whose standing nothing judges could let the use case read as complete.
    for number, act in enumerate(requested, start=1):
        judging = [e for e in seen.values() if e["kind"] == "requested_act_standing" and e["requested"] == number]
        if len(judging) != 1:
            problems.append(f"requested act {number} ({act}) needs exactly one requested_act_standing expectation, has {len(judging)}")
    return problems


def load_scripted_provider(case, mode):
    """The use case's scripted model output. Stub mode only: a live run never reads it."""
    if mode != "stub":
        raise Refusal("scripted_provider_outside_stub_mode")
    try:
        program = json.loads(case.script_path.read_text())
    except (OSError, ValueError) as error:
        raise Refusal(f"scripted_provider_unreadable {case.id}: {type(error).__name__}") from None
    problems = validate_scripted_provider(program, case)
    if problems:
        raise Refusal(f"scripted_provider_invalid {case.id}: " + "; ".join(problems))
    inputs = {name: pluck(json.loads(project_file(ref["file"]).read_text()), ref.get("path", []))
              for name, ref in program.get("inputs", {}).items()}
    return program, inputs


def template_refs(value):
    """Every ($bound or $input, name) a template refers to."""
    if isinstance(value, dict):
        if "$bound" in value:
            return [("$bound", value["$bound"])]
        if "$input" in value:
            return [("$input", value["$input"])]
        return [ref for item in value.values() for ref in template_refs(item)]
    if isinstance(value, list):
        return [ref for item in value for ref in template_refs(item)]
    return []


def validate_scripted_provider(program, case):
    if not isinstance(program, dict):
        return ["not an object"]
    problems = []
    if program.get("schema") != SCRIPT_SCHEMA:
        problems.append(f"schema {program.get('schema')!r}")
    if program.get("use_case") != case.id:
        problems.append(f"use_case {program.get('use_case')!r} is not {case.id}")
    inputs = program.get("inputs", {})
    for name, ref in inputs.items() if isinstance(inputs, dict) else []:
        try:
            pluck(json.loads(project_file(ref["file"]).read_text()), ref.get("path", []))
        except (Unresolved, ValueError, KeyError, TypeError) as error:
            problems.append(f"input {name}: {error}")
    entries = program.get("requests")
    if not isinstance(entries, list) or [e.get("request") if isinstance(e, dict) else None for e in entries] != list(range(1, len(case.requests) + 1)):
        return problems + [f"requests must script each of the definition's {len(case.requests)} requests in order"]
    bound = set()
    for entry in entries:
        where = f"request {entry['request']}"
        for need in entry.get("requires", []):
            if need.get("bound") not in bound or not isinstance(need.get("expect"), dict) or not isinstance(need.get("unexpected"), str):
                problems.append(f"{where}: requires {need!r}")
        steps = entry.get("steps")
        if not isinstance(steps, list):
            problems.append(f"{where}: steps")
            continue
        for index, step in enumerate(steps):
            if (not isinstance(step, dict) or set(step) - {"bind", "call", "arguments", "expect", "unexpected"}
                    or not isinstance(step.get("call"), str) or not step["call"].startswith("swbpipe_")
                    or not isinstance(step.get("arguments"), dict) or not isinstance(step.get("bind"), str) or step["bind"] in bound
                    or ("expect" in step) != ("unexpected" in step) or not isinstance(step.get("expect", {}), dict)):
                problems.append(f"{where} step {index + 1}: {step!r}")
                continue
            for kind, name in template_refs(step["arguments"]):
                if (kind == "$bound" and name not in bound) or (kind == "$input" and name not in inputs):
                    problems.append(f"{where} step {index + 1}: {kind} {name} is not available")
            bound.add(step["bind"])
        reply = entry.get("reply")
        if not isinstance(reply, dict) or not isinstance(reply.get("text"), str) or set(reply) - {"text", "records"}:
            problems.append(f"{where}: reply {reply!r}")
        elif "records" in reply:
            records = reply["records"]
            if (not isinstance(records, dict) or set(records) != {"each", "format", "fields"}
                    or any(k != "$bound" or n not in bound for k, n in template_refs(records.get("each")))):
                problems.append(f"{where}: records {records!r}")
    return problems


# ---------------------------------------------------------------------------
# The scripted provider (stub mode): model output replayed from a use case's script.

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


def matches(outcome, expect):
    """A tool outcome meets a step's expectation field by field (dotted paths), not as a whole
    object, so members the product adds later do not disturb a script."""
    for path, wanted in expect.items():
        try:
            if pluck(outcome, path.split(".")) != wanted:
                return False
        except Unresolved:
            return False
    return True


class Fields(dict):
    def __missing__(self, key):
        return None


class Script:
    """Replays a use case's scripted provider from the actual tool outcomes; every call it
    makes is derived from what the product returned, and anything unexpected ends the turn."""

    def __init__(self, case):
        self.case = case
        self.program, self.inputs = load_scripted_provider(case, "stub")
        self.turns = case.requests
        self.captured = []
        self.failures = []

    @staticmethod
    def tool_name(tools, suffix):
        names = [t.get("name") for t in tools or [] if isinstance(t, dict)]
        matches_ = [n for n in names if isinstance(n, str) and n.endswith(suffix)]
        return matches_[0] if len(matches_) == 1 else None

    def turn_of(self, text):
        return next((n for n, t in enumerate(self.turns) if t in text), None)

    def next_item(self, request):
        items = request.get("input") or []
        users = [i for i, item in enumerate(items) if item.get("type", "message") == "message" and item.get("role") == "user"]
        if not users:
            return self.fail("no user message")
        last = users[-1]
        turn = self.turn_of(user_text(items[last]))
        if turn is None:
            return self.fail("unrecognized user message")
        outputs = [tool_output(item) for item in items[last + 1:] if item.get("type") == "function_call_output"]
        if outputs:
            self.captured.append({"turn": turn + 1, "step": len(outputs), "outcome": outputs[-1]})
        # Earlier requests' outcomes, bound by position to the steps their script called.
        bindings, group = {}, None
        for item in items[:last]:
            if item.get("type", "message") == "message" and item.get("role") == "user":
                index = self.turn_of(user_text(item))
                if index is not None:
                    group = [self.program["requests"][index]["steps"], 0]
            elif item.get("type") == "function_call_output" and group is not None:
                steps, position = group
                if position < len(steps):
                    bindings[steps[position]["bind"]] = tool_output(item)
                group[1] += 1
        entry = self.program["requests"][turn]
        steps = entry["steps"]
        for position, outcome in enumerate(outputs[:len(steps)]):
            bindings[steps[position]["bind"]] = outcome
        for need in entry.get("requires", []):
            if need["bound"] not in bindings or not matches(bindings[need["bound"]], need["expect"]):
                return self.fail(need["unexpected"])
        tools = request.get("tools")
        count = len(outputs)
        if count > len(steps):
            return self.fail("more tool outcomes than the script's calls")
        if count:
            step = steps[count - 1]
            if "expect" in step and not matches(outputs[-1], step["expect"]):
                return self.fail(step["unexpected"].format_map(Fields(outputs[-1] if isinstance(outputs[-1], dict) else {})))
        try:
            if count < len(steps):
                return self.call(self.tool_name(tools, steps[count]["call"]), self.resolve(steps[count]["arguments"], bindings))
            return {"type": "message", "text": self.reply(entry["reply"], bindings)}
        except Unresolved as error:
            return self.fail(f"unresolved {error}")

    def resolve(self, template, bindings):
        if isinstance(template, dict):
            if "$bound" in template:
                if template["$bound"] not in bindings:
                    raise Unresolved(template["$bound"])
                return pluck(bindings[template["$bound"]], template.get("path", []))
            if "$input" in template:
                return pluck(self.inputs[template["$input"]], template.get("path", []))
            return {key: self.resolve(value, bindings) for key, value in template.items()}
        if isinstance(template, list):
            return [self.resolve(value, bindings) for value in template]
        return template

    def reply(self, reply, bindings):
        if "records" not in reply:
            return reply["text"]
        records = reply["records"]
        lines = []
        for item in self.resolve(records["each"], bindings):
            fields = {}
            for name, spec in records["fields"].items():
                value = pluck(item, spec["path"])
                value = spec.get("map", {}).get(value, value) if isinstance(value, str) else value
                fields[name] = repr(value) if spec.get("repr") else str(value)
            lines.append(records["format"].format(**fields))
        return reply["text"] + "\n" + "\n".join(lines)

    def fail(self, why):
        self.failures.append(why)
        return {"type": "message", "text": f"STUB-UNEXPECTED: {why}"}

    def call(self, name, arguments):
        if name is None:
            return self.fail("tool not declared")
        return {"type": "function_call", "name": name, "arguments": arguments}


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
    """127.0.0.1 only; answers POST /v1/responses from the use case's Script and 404 otherwise."""

    def __init__(self, case):
        self.script = Script(case)
        self.requests = []
        stub = self

        class Handler(http.server.BaseHTTPRequestHandler):
            protocol_version = "HTTP/1.1"

            def setup(self):
                super().setup()
                stub.prepare_connection(self)

            def send_error(self, code, message=None, explain=None):
                stub.on_http_error(self, code)
                super().send_error(code, message, explain)

            def handle_one_request(self):
                self._swbpipe_dispatched = False
                try:
                    super().handle_one_request()
                finally:
                    stub.after_request(self)

            def log_message(self, *args):
                pass

            def do_POST(self):
                self._swbpipe_dispatched = True
                stub.accept_post(self)

            def do_GET(self):
                self._swbpipe_dispatched = True
                stub.accept_get(self)

            def reply(self, status, body, kind):
                self.send_response(status)
                self.send_header("Content-Type", kind)
                self.send_header("Content-Length", str(len(body)))
                self.send_header("Connection", "close")
                self.end_headers()
                self.wfile.write(body)

        self.server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)

    def prepare_connection(self, handler):
        pass

    def on_http_error(self, handler, code):
        pass

    def after_request(self, handler):
        pass

    def accept_post(self, handler):
        length = int(handler.headers.get("Content-Length") or 0)
        body = handler.rfile.read(min(length, 8 * 1024 * 1024))
        record = {"path": handler.path, "bytes": length, "authorization": "Authorization" in handler.headers}
        self.requests.append(record)
        if handler.path.rstrip("/") != "/v1/responses" or length > 8 * 1024 * 1024:
            return handler.reply(404, b"{}", "application/json")
        try:
            request = json.loads(body)
        except ValueError:
            return handler.reply(400, b"{}", "application/json")
        item = self.script.next_item(request)
        record["answer"] = item["type"] if item["type"] == "message" else item["name"]
        handler.reply(200, sse(item, len(self.requests)), "text/event-stream")


    def accept_get(self, handler):
        self.requests.append({"path": handler.path, "method": "GET"})
        handler.reply(404, b"{}", "application/json")

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


def scripted_provider_for(mode, case):
    """The loopback scripted provider in stub mode; nothing at all in live mode, where the
    use case's script is never read and a person-configured endpoint answers."""
    if mode == "stub":
        return StubProvider(case)
    if mode == "live":
        return None
    raise Refusal(f"mode {mode!r}")


# ---------------------------------------------------------------------------
# Judging what the window showed, against the use case's expectations.

def entered_value(text):
    match = re.fullmatch(r"Entered: (\S+) (\S+)", (text or "").strip())
    return (float(match.group(1)), match.group(2)) if match else None


def report_record(entity):
    """`<entity> <component> = <value> <unit>` records in the agent's report."""
    return re.compile(re.escape(entity) + r"\s+(ux|uy|uz|rx|ry|rz)\s*=\s*([-+]?(?:\d+\.\d*|\.\d+|\d+)(?:[eE][-+]?\d+)?)\s*(mm|rad)\b")


class Check:
    """One kind of expectation: what it judges, its parameters, whether it needs the tool
    outcomes only a scripted provider exposes, and the judging function, which returns its
    named failures and how many comparisons it made (none is a failure)."""

    def __init__(self, category, params, judge_fn, validate=None, needs_tool_outcomes=False):
        self.category, self.params, self.judge = category, params, judge_fn
        self.needs_tool_outcomes = needs_tool_outcomes
        self.validate = validate or (lambda expectation, context, seen: [])


def request_report_key(number):
    return f"turn{number}"


def judge_requests_answered(expectation, report, case, context, captured):
    # Each turn completed (the bare standing "Ready."; a failed or interrupted turn is not)
    # and the agent answered in the window.
    failures = []
    for number in range(1, len(case.requests) + 1):
        name = request_report_key(number)
        turn = report.get(name) or {}
        if turn.get("standing") != "Ready.":
            failures.append(f"{name} standing {turn.get('standing')!r}")
        if not any(str(line).startswith("Agent:") for line in turn.get("messages") or []):
            failures.append(f"{name}: no agent message in the window")
    return failures, len(case.requests)


def judge_agent_batch_shown(expectation, report, case, context, captured):
    failures = []
    operations = case.reference(expectation["operations"])
    batch = report.get(apply_report_key(expectation["apply"])) or {}
    if not str(batch.get("batch_id", "")).startswith("agent-") or batch.get("batches_shown") != 1:
        failures.append("batch review did not show exactly one agent batch")
    count = re.match(r"(\d+) proposed changes\b", str(batch.get("summary") or ""))
    if not count or int(count.group(1)) != len(operations):
        failures.append(f"batch summary {batch.get('summary')!r}")
    # The batch as Batch review shows it submitted: the referenced operations, in order.
    if batch.get("submitted") != [operation_key(op) for op in operations]:
        failures.append("the submitted batch is not the fixture's operations in order")
    if not batch.get("apply_message"):
        failures.append("no batch receipt after Apply")
    return failures, len(operations)


def judge_tree_rows(expectation, report, case, context, captured):
    # Every row of the observed types the tree shows, against the entities the batch creates.
    types = expectation["object_types"]
    expected_rows = sorted([types[op["target"]["object_type"]], op["target"]["ref"]]
                           for op in case.reference(expectation["operations"]) if op["target"]["object_type"] in types)
    if sorted(list(row) for row in report.get("tree_rows") or []) != expected_rows:
        return [f"model tree rows {report.get('tree_rows')}, expected {expected_rows}"], len(expected_rows)
    return [], len(expected_rows)


def judge_nodal_motions(expectation, report, case, context, captured):
    """The entity's nodal motions as the Results section shows them, each finite and in its
    unit, against the reference under its criterion (relative, zero rows on the family scale)."""
    failures, entity = [], expectation["entity"]
    reference = case.reference(expectation["reference"])
    reference_rows = [r for r in reference["rows"] if r["quantity"] == "nodal_motion" and r["entity"] == entity]
    if len(reference_rows) != expectation["rows"]:
        failures.append(f"the reference has {len(reference_rows)} nodal motions of {entity}, the use case expects {expectation['rows']}")
    rows = (report.get("results") or {}).get("rows") or []
    shown = {}
    scales = reference["zero_scales"]
    for r in reference_rows:
        q = "displacement" if r["component"].startswith("u") else "rotation"
        unit = "mm" if q == "displacement" else "rad"
        # The Results section's row id for a nodal motion component, as the product forms it.
        row_id = f"result:disp:{r['entity'].replace(':', '-')}:{r['component']}"
        found = [row for row in rows if row["id"] == row_id]
        parsed = entered_value(found[0]["text"]) if len(found) == 1 else None
        if parsed is None:
            failures.append(f"{entity} {r['component']}: {len(found)} rows shown")
            continue
        if not math.isfinite(parsed[0]):
            failures.append(f"{entity} {r['component']}: shown {parsed[0]!r} is not finite")
            continue
        if parsed[1] != unit:
            failures.append(f"{entity} {r['component']}: unit {parsed[1]}")
        observed = parsed[0] / 1000.0 if q == "displacement" else parsed[0]
        family = "translation" if q == "displacement" else "rotation"
        tolerance = expectation["relative"] * (abs(r["value"]) if r["value"] != 0 else abs(scales[family]["value"]))
        shown[row_id] = (f"global_nodal_{q}_{r['component'][1]}", parsed[0], unit)
        if abs(observed - r["value"]) > tolerance:
            failures.append(f"{entity} {r['component']}: shown {observed!r}, reference {r['value']!r}")
    context.setdefault("shown", {})[entity] = (shown, expectation["rows"])
    return failures, len(reference_rows)


def judge_tool_outcomes(expectation, report, case, context, captured):
    """Stub mode: the solve was on the applied model, the window's solve proof names that
    model, and the agent read exactly the rows the window shows (ids, kinds, entity, units,
    values)."""
    failures, entity = [], expectation["entity"]
    shown, rows = context["shown"][entity]
    outcomes = [c["outcome"] for c in captured]
    observe = next((o for o in outcomes if o.get("status") == "applied"), None)
    solve = next((o for o in outcomes if o.get("status") == "completed"), None)
    results = next((o for o in outcomes if o.get("status") == "ok" and "values" in (o.get("data") or {})), None)
    if not observe or not solve or not results:
        return ["stub did not capture applied, completed and results outcomes"], 1
    if observe["data"]["outcomeModelHash"] != solve["data"]["evaluatedModelHash"]:
        failures.append("solve was not on the applied model's hash")
    shown_hash = ((report.get("results") or {}).get("model_sha256") or "").removeprefix("sha256:")
    if shown_hash != (solve["data"]["evaluatedModelHash"] or {}).get("value"):
        failures.append("the window's solve proof names another model hash")
    read = {value.get("id"): value for value in results["data"]["values"]}
    if len(read) != len(results["data"]["values"]) or set(read) != set(shown) or len(shown) != rows:
        failures.append(f"agent read {sorted(read)}, the window shows {sorted(shown)}")
    for row_id, (kind, value, unit) in shown.items():
        item = read.get(row_id)
        if item and (item.get("kind"), item.get("entityRef"), item.get("unit"), item.get("value")) != (kind, entity, unit, value):
            failures.append(f"agent value {row_id} differs from the window")
    return failures, 2 + len(shown)


def judge_report_states_window(expectation, report, case, context, captured):
    """The agent's report in the window gives one `<entity> <component> = <value> <unit>`
    record per motion the window shows, in its unit: exactly under the scripted provider, to
    the stated relative tolerance live (a model may round); a zero is reported as zero."""
    failures, entity = [], expectation["entity"]
    shown, _ = context["shown"][entity]
    turn = report.get(request_report_key(expectation["request"])) or {}
    agent_text = "\n".join(str(line) for line in turn.get("messages") or [] if str(line).startswith("Agent:"))
    records = {}
    for component, number, unit in report_record(entity).findall(agent_text):
        records.setdefault(component, []).append((float(number), unit))
    relative = expectation["relative"]
    for row_id, (kind, value, unit) in sorted(shown.items()):
        component = COMPONENT[kind]
        stated = records.get(component, [])
        if len(stated) != 1:
            failures.append(f"the agent's report states {entity} {component} {len(stated)} times")
            continue
        number, stated_unit = stated[0]
        close = number == value if captured is not None else (abs(number - value) <= relative * abs(value) if value else number == 0)
        if stated_unit != unit or not math.isfinite(number) or not close:
            failures.append(f"the agent's report states {entity} {component} = {number!r} {stated_unit}, the window {value!r} {unit}")
    return failures, len(shown)


def validate_operations_ref(expectation, context, seen):
    ref = context["references"].get(expectation["operations"])
    return [] if isinstance(ref, dict) and "case" not in ref else [f"operations names no file reference {expectation['operations']!r}"]


def validate_apply(expectation, context, seen):
    problems = validate_operations_ref(expectation, context, seen)
    if not isinstance(expectation["apply"], int) or not 1 <= expectation["apply"] <= context["applies"]:
        problems.append(f"apply {expectation['apply']!r}: the sequence has {context['applies']} apply acts")
    return problems


def validate_tree(expectation, context, seen):
    problems = validate_operations_ref(expectation, context, seen)
    types = expectation["object_types"]
    if not isinstance(types, dict) or not types or set(types.values()) != set(context["observe"]["tree_types"]):
        problems.append(f"object_types {types!r} must map onto exactly the observed tree types")
    return problems


def validate_motions(expectation, context, seen):
    problems = []
    ref = context["references"].get(expectation["reference"])
    if not isinstance(ref, dict) or "case" not in ref:
        problems.append(f"reference {expectation['reference']!r} names no benchmark case")
    if not isinstance(expectation["entity"], str) or not expectation["entity"]:
        problems.append("entity")
    if not isinstance(expectation["rows"], int) or expectation["rows"] < 1:
        problems.append(f"rows {expectation['rows']!r}")
    if not isinstance(expectation["relative"], (int, float)) or not 0 < expectation["relative"] < 1:
        problems.append(f"relative {expectation['relative']!r}")
    return problems


def needs_motions(expectation, seen):
    if not any(e["kind"] == "nodal_motions_match_reference" and e["entity"] == expectation["entity"] for e in seen.values()):
        return [f"needs an earlier nodal_motions_match_reference expectation for {expectation['entity']}"]
    return []


def validate_report(expectation, context, seen):
    problems = needs_motions(expectation, seen)
    if not isinstance(expectation["request"], int) or not 1 <= expectation["request"] <= context["requests"]:
        problems.append(f"request {expectation['request']!r}: the sequence has {context['requests']} requests")
    if not isinstance(expectation["relative"], (int, float)) or not 0 < expectation["relative"] < 1:
        problems.append(f"relative {expectation['relative']!r}")
    return problems


def judge_requested_act_standing(expectation, report, case, context, captured):
    """The standing the agent was given for a requested act, as the tool outcomes it received
    show it. Only "performed" settles the act; any other standing, or none seen (always so in a
    live run, whose tool outcomes the controller does not see), leaves the use case unfinished
    for the agent. That is not a failure of the route. A print request reported as performed is
    a failure: print is settled at best as sent to the print dialog."""
    number = expectation["requested"]
    act = case.requested_acts[number - 1]
    seen = []
    for item in captured or []:
        outcome = item.get("outcome")
        try:
            if isinstance(outcome, dict) and pluck(outcome, REQUEST_OBSERVATION["act"].split(".")) == act:
                seen.append(pluck(outcome, REQUEST_OBSERVATION["standing"].split(".")))
        except Unresolved:
            continue
    standing = seen[-1] if seen else "not observed"
    standing = standing if isinstance(standing, str) and standing in REQUEST_STANDINGS + ("not observed",) else "unrecognized"
    context.setdefault("requested", []).append({"requested": number, "act": act, "standing": standing})
    if act == "print_report" and standing == PERFORMED:
        return ["print_report was reported performed; a print request is settled at most as sent to the print dialog"], 1
    return [], 1


def validate_requested(expectation, context, seen):
    number = expectation["requested"]
    if not isinstance(number, int) or not 1 <= number <= len(context["requested"]):
        return [f"requested {number!r}: the sequence has {len(context['requested'])} requested acts"]
    return []


CHECKS = {
    "requested_act_standing": Check("conversation", ("requested",), judge_requested_act_standing, validate_requested),
    "requests_answered": Check("conversation", (), judge_requests_answered),
    "agent_batch_shown": Check("window", ("apply", "operations"), judge_agent_batch_shown, validate_apply),
    "tree_rows_are_batch_targets": Check("window", ("operations", "object_types"), judge_tree_rows, validate_tree),
    "nodal_motions_match_reference": Check("window", ("reference", "entity", "rows", "relative"), judge_nodal_motions, validate_motions),
    "tool_outcomes_agree_with_window": Check("agreement", ("entity",), judge_tool_outcomes,
                                             lambda e, c, s: needs_motions(e, s), needs_tool_outcomes=True),
    "report_states_window_motions": Check("agreement", ("request", "entity", "relative"), judge_report_states_window, validate_report),
}


def judge(report, case, captured=None, judgement=None):
    """Named failures; empty means the window, the conversation and their agreement were as the
    use case expects. `captured` is the scripted provider's view of the tool outcomes (stub
    mode only). `judgement`, when given, receives which expectations were judged and which
    could not be in this mode."""
    failures, judged, unjudged, context = [], [], [], {}
    if report.get("error"):
        failures.append(f"driver: {report['error']}")
    if report.get("steps") != case.steps:
        failures.append(f"steps {report.get('steps')}")
    for expectation in case.expectations:
        check = CHECKS[expectation["kind"]]
        if check.needs_tool_outcomes and captured is None:
            unjudged.append(expectation["id"])
            continue
        try:
            found, compared = check.judge(expectation, report, case, context, captured)
        except (Unresolved, KeyError, TypeError, ValueError) as error:
            found, compared = [f"expectation {expectation['id']} could not be judged: {type(error).__name__}"], 1
        failures += found
        if compared < 1:
            failures.append(f"expectation {expectation['id']} compared nothing")
        judged.append(expectation["id"])
    if judgement is not None:
        acts = context.get("requested", [])
        if len(acts) != len(case.requested_acts):
            failures.append(f"{len(case.requested_acts)} requested acts, {len(acts)} judged")
        judgement.update(judged=judged, notJudged=unjudged, requestedActs={
            "standing": "none" if not case.requested_acts
            else "all performed" if len(acts) == len(case.requested_acts) and all(a["standing"] == PERFORMED for a in acts)
            else "unfinished",
            "acts": acts})
    return failures


def settle(result):
    """The run's one word. "passed" needs the route examined with no failure and, for a use case
    with requested acts, every one shown to the agent as performed; "unfinished" is a route that
    passed with a requested act not so shown, and is no pass: `passed` is then false, with no
    failure named."""
    route_passed = bool(result.get("passed"))
    unfinished = ((result.get("expectations") or {}).get("requestedActs") or {}).get("standing") == "unfinished"
    result["outcome"] = "failed" if not route_passed else "unfinished" if unfinished else "passed"
    result["passed"] = result["outcome"] == "passed"
    return result["outcome"]


def operation_key(operation):
    """An operation as the judge compares it: id, kind and target."""
    change, target = operation.get("change") or {}, operation.get("target") or {}
    return [operation.get("operation_id"), f"{operation.get('operation_kind')}/{target.get('object_type')}/"
            f"{change.get('change_kind')}/{change.get('field_path')}", target.get("ref")]


# The W9-A journey, the first use case. These names keep the supplier capture working.
W9A = load_use_case(DEFAULT_USE_CASE)
TURNS = W9A.requests
FIXTURE = PROJECT / W9A.definition["references"]["batch"]["file"]


def w9a_reference():
    return W9A.reference("motions")


# ---------------------------------------------------------------------------
# The run.

def free_bytes():
    return memory_monitor.read_free_bytes()


def check_memory_monitor(args):
    """Health loss enters the existing Refusal/finally custody route; never signals a PID."""
    heartbeat = getattr(args, "memory_heartbeat", None)
    if heartbeat is None:
        if args.mode == "live":
            raise Refusal("memory_monitor_required")
        return
    if args.cancel_file is None:
        raise Refusal("memory_monitor_cancel_required")
    try:
        memory_monitor.check_health(heartbeat, args.cancel_file)
    except memory_monitor.MonitorRefusal as error:
        raise Refusal("memory_monitor_" + str(error)) from None


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

    def scopes(self):
        return [json.loads(line.split(" ", 1)[1]) for line in self.lines if line.startswith("[native-smoke-agent-scope] ")]

    def scope(self):
        found = self.scopes()
        return found[0] if found else None


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
    case = getattr(args, "case", None) or W9A
    result = {"passed": False, "mode": args.mode, "startedAt": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
              "appSha256": sha256(args.app), "cleanup": "unknown", "standing": STANDING[args.mode],
              "useCase": {"id": case.id, "definitionSha256": sha256(case.path)}}
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
    stub = scripted_provider_for(args.mode, case)
    if stub:
        result["useCase"]["scriptedProviderSha256"] = sha256(case.script_path)
    app = anchor = observer = lines = None
    sdk = None
    try:
        check_memory_monitor(args)
        if stub:
            stub.__enter__()
        plan = case.plan({"endpoint": stub.endpoint if stub else args.endpoint, "model": args.model,
                          "contextWindow": args.context_window},
                         {"ui": 60000, "start": 60000, "turn": args.turn_timeout * 1000})
        env = {"PATH": "/usr/bin:/bin", "HOME": str(home), "SWBPIPE_NATIVE_SMOKE_DRIVER": "-",
               "SWBPIPE_NATIVE_SMOKE_PLAN": json.dumps(plan), "SWBPIPE_NATIVE_SMOKE_REPORT": str(report_path),
               "SWBPIPE_NATIVE_SMOKE_AGENT_GATE": str(gate), "SWBPIPE_NATIVE_SMOKE_AGENT_GATE_MODE": "handshake"}
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
            check_memory_monitor(args)
            if args.cancel_file and args.cancel_file.exists():
                raise Refusal("cancelled")
            if lines.error:
                raise Refusal(lines.error)
            if observer:
                observer.sample()

        refused, answered = None, False
        while app.poll() is None:
            safety()
            if sdk is None and (scope := lines.scope()):
                if scope.get("appPid") != app.pid:
                    raise Refusal("app_identity")
                sdk = scope["sdkPid"]
                result["supplier"] = {"pid": sdk, "scope": scope["scope"]}
                # One admission. The gate (handshake mode) holds the original supplier
                # unreaped, with no deadline, until it is answered, so its id cannot be
                # reused while admission is possible. It is never retried: on refusal the
                # gate is answered REFUSE and native cleanup ends the supplier, as the
                # product does for a failed start.
                try:
                    observer = Scope(sdk)
                    if observer.exited:
                        raise Refusal("supplier_exited_before_gate")
                    anchor = anchor_start(args.process_probe, sdk, app)
                except Exception as error:  # noqa: BLE001
                    refused = str(error) if isinstance(error, Refusal) else type(error).__name__
                    result["anchorRefused"] = refused
                    answered = answer_gate(gate, b"REFUSE\n")
                    continue
                result["anchor"] = {"pid": anchor.pid, "group": sdk}
                safety()  # Admission can take time: recheck health before allowing any inference.
                answered = answer_gate(gate, b"READY\n")
            if time.monotonic() >= deadline:
                raise Refusal("deadline")
            time.sleep(.01)
        safety()
        result["app"]["status"] = app.returncode
        if app.returncode != 0:
            raise Refusal("app_nonzero")
        if len(lines.scopes()) > 1:
            raise Refusal("second_supplier_published")
        if observer is None:
            raise Refusal(f"anchor_refused: {refused}" if refused else "supplier_never_published")
        bounded_wait(lambda: observer.sample() and not pid_live(sdk), time.monotonic() + 5, check=safety)
        if refused:
            raise Refusal(f"anchor_refused: {refused}")
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
        judgement = {}
        failures = judge(report, case, stub.script.captured if stub else None, judgement)
        result["expectations"] = judgement
        if stub:
            result["stub"] = {"requests": stub.requests, "captured": stub.script.captured, "failures": stub.script.failures}
            failures += [f"stub: {why}" for why in stub.script.failures]
        result["failures"] = failures
        safety()
        result["passed"] = not failures
    except Exception as error:  # noqa: BLE001
        result["failure"] = str(error) if isinstance(error, Refusal) else type(error).__name__
    finally:
        if app and app.poll() is None:
            release_launch(app, anchor, observer, lines, gate, sdk, answered, root, args.release_grace, result)
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


RETAINED = Path("/private/tmp") / f"swbpipe-agent-journey-{os.getuid()}.retained"


def answer_gate(gate, answer):
    """The controller's one answer at the native gate (READY or REFUSE), written once."""
    try:
        try:
            descriptor = os.open(gate, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        except FileExistsError:
            return True
        with os.fdopen(descriptor, "wb") as handle:
            handle.write(answer)
    except OSError:
        # A failed refusal must not bypass the custody decision and retained-app latch.
        return False
    return True


def release_launch(app, anchor, observer, lines, gate, sdk, answered, root, release_grace, result):
    """Refuse an unanswered gate, then decide custody even if gate I/O or scope parsing fails."""
    if sdk is not None and not answered and not answer_gate(gate, b"REFUSE\n"):
        result["gateAnswerFailed"] = True
        result["passed"] = False
        result["failure"] = result.get("failure") or "gate_answer_failed"
    grace = time.monotonic() + release_grace
    while app.poll() is None and time.monotonic() < grace and not (anchor is None and supplier_gone(observer)):
        time.sleep(.1)
    try:
        single = lines is not None and len(lines.scopes()) == 1
    except (ValueError, TypeError):
        # Malformed identity evidence proves no single-supplier custody; keep the app.
        single = False
    outcome = release_app(app, anchor, supplier_gone(observer), single)
    result["release"] = outcome
    if outcome == "retained_original_app_custody":
        result["rescue"] = outcome
        result["passed"] = False
        result["failure"] = result.get("failure") or outcome
        retain(app, root)
    result.setdefault("app", {})["status"] = app.returncode


def supplier_gone(observer):
    """True only when the kqueue observer of the original supplier has seen it exit."""
    if observer is None:
        return False
    try:
        return bool(observer.sample())
    except Exception:  # noqa: BLE001  (an invalid observation proves nothing)
        return False


def release_app(app, anchor, supplier_exited, single_supplier):
    """End a launch that did not end itself, from evidence only. The owned app is killed by
    its own handle only when exactly one supplier was published and either an anchor holds
    its group (and rescues it after) or the original supplier was observed to exit.
    Otherwise the app may be some supplier's only custodian and is left running (custody
    retained, the run fails); nothing is retried and no saved id is reused."""
    if app.poll() is not None:
        return None
    if not single_supplier or (anchor is None and not supplier_exited):
        return "retained_original_app_custody"
    app.kill()
    app.wait(timeout=5)
    return "killed_owned_app"


def retain(app, root):
    """A host latch that outlives this controller: no further attempt starts until a person
    has quit the retained app normally (its quit performs native cleanup) and removed it."""
    RETAINED.write_text(json.dumps({"appPid": app.pid, "output": str(root),
                                    "at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}) + "\n")


KEY_BYTES = 8192


def read_key(stream):
    """One provider key line from the controller's stdin: at most 8192 printable ASCII
    bytes without spaces, then a newline. Anything else is refused without echo."""
    line = stream.readline(KEY_BYTES + 2)
    if len(line) > KEY_BYTES + 1 or not line.endswith(b"\n"):
        raise Refusal("key_input_refused")
    key = line[:-1]
    if not key or any(byte < 33 or byte > 126 for byte in key):
        raise Refusal("key_input_refused")
    return key.decode("ascii")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--app", type=Path, required=True, help="the native-smoke executable inside a private bundle copy")
    parser.add_argument("--process-probe", type=Path, required=True)
    parser.add_argument("--boundary-probe", type=Path, required=True)
    parser.add_argument("--mode", choices=["stub", "live"], required=True)
    parser.add_argument("--use-case", default=DEFAULT_USE_CASE,
                        help=f"a use case under fixtures/embedded_agent/use_cases ({', '.join(use_case_names())})")
    parser.add_argument("--endpoint", help="live mode: the provider endpoint")
    parser.add_argument("--model", default="stub-model")
    parser.add_argument("--context-window", type=int, default=32768)
    parser.add_argument("--key-stdin", action="store_true", help="live mode: read one key line from stdin")
    parser.add_argument("--min-free-gib", type=float, default=50.0)
    parser.add_argument("--turn-timeout", type=int, default=600, help="seconds per agent turn")
    parser.add_argument("--deadline", type=int, default=1800, help="seconds for the whole launch")
    parser.add_argument("--release-grace", type=int, default=120,
                        help="seconds a launch that did not end itself is given to end before custody is decided")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--cancel-file", type=Path)
    parser.add_argument("--memory-heartbeat", type=Path,
                        help="required live: the external two-second monitor's heartbeat.json")
    args = parser.parse_args()
    # A retained run's latch refuses before anything is read, created or launched.
    if RETAINED.exists():
        print(f"refused: a previous attempt retained its app ({RETAINED.read_text().strip()}). Quit that app "
              f"normally, confirm no process remains under its output directory, then remove {RETAINED}.", file=sys.stderr)
        return 2
    if args.mode == "live" and not args.endpoint:
        parser.error("live mode needs --endpoint")
    if args.mode == "stub" and args.key_stdin:
        parser.error("stub mode takes no key")
    if args.mode == "stub" and args.endpoint:
        parser.error("stub mode takes no endpoint: the scripted provider answers on loopback")
    if args.mode == "live" and not args.memory_heartbeat:
        parser.error("live mode needs --memory-heartbeat and --cancel-file")
    if args.memory_heartbeat and not args.cancel_file:
        parser.error("--memory-heartbeat needs --cancel-file")
    if args.mode == "live" and args.release_grace != 0:
        parser.error("live monitoring requires --release-grace 0 for prompt owned cancellation")
    # One attempt at a time on this host, whatever the output directory; the retained-run
    # latch is checked again under that lock, before the key is read or run state created.
    lock = open(Path("/private/tmp") / f"swbpipe-agent-journey-{os.getuid()}.lock", "w")
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        print("another agent journey attempt is running", file=sys.stderr)
        return 2
    if RETAINED.exists():
        print(f"refused: a previous attempt retained its app ({RETAINED.read_text().strip()}). Quit that app "
              f"normally, confirm no process remains under its output directory, then remove {RETAINED}.", file=sys.stderr)
        return 2
    try:
        args.case = load_use_case(args.use_case)  # An invalid definition refuses before anything is created.
        refuse_unperformable(args.case)
        check_memory_monitor(args)  # Refuse unhealthy admission before consuming the single key read.
        key = read_key(sys.stdin.buffer) if args.key_stdin else None
    except Refusal as refusal:
        print(f"refused: {refusal}", file=sys.stderr)
        return 2
    args.output.mkdir(mode=0o700, parents=False, exist_ok=False)
    root = args.output.resolve()
    for name in ("app", "process_probe", "boundary_probe"):
        setattr(args, name, getattr(args, name).resolve(strict=True))
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
    settle(result)
    (root / "result.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({k: result.get(k) for k in ("useCase", "standing", "outcome", "passed", "failure", "failures", "expectations",
                                                 "rescue", "release", "residents", "startedAt", "endedAt")}))
    return OUTCOME_EXIT[result["outcome"]]


if __name__ == "__main__":
    raise SystemExit(main())
