"""Offline checks of the agent journey controller (no app, no supplier, no model)."""
from __future__ import annotations

import json
from pathlib import Path
import re
import sys
import urllib.error
import urllib.request

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from tools import agent_journey_fixture as fixture  # noqa: E402

BASIS = {"workspaceHandle": "w" * 48, "applicationGeneration": 1, "modelRevision": 1,
         "modelHash": {"value": "a" * 64}, "stateId": None, "runId": None}
TOOLS = [{"type": "function", "name": name} for name in (
    "swbpipe_read_model_summary", "swbpipe_read_nodes", "swbpipe_read_records", "swbpipe_propose_operations",
    "swbpipe_observe_proposal", "swbpipe_solve", "swbpipe_read_results")]


def user(text):
    return {"type": "message", "role": "user", "content": [{"type": "input_text", "text": text}]}


def output(outcome, as_list=False):
    text = json.dumps(outcome)
    return {"type": "function_call_output", "call_id": "c", "output": [{"type": "input_text", "text": text}] if as_list else text}


def ask(script, items):
    return script.next_item({"input": items, "tools": TOOLS})


def test_turn_one_reads_the_summary_then_proposes_the_fixture_on_its_basis():
    script = fixture.Script(fixture.TURNS)
    first = ask(script, [user(fixture.TURNS[0])])
    assert (first["type"], first["name"], first["arguments"]) == ("function_call", "swbpipe_read_model_summary", {})
    summary = {"status": "ok", "basis": BASIS, "data": {"workspaceHandle": BASIS["workspaceHandle"]}}
    proposal = ask(script, [user(fixture.TURNS[0]), output(summary, as_list=True)])
    assert proposal["name"] == "swbpipe_propose_operations"
    arguments = proposal["arguments"]
    assert arguments["basis"] == BASIS and arguments["claimedModelHash"] == BASIS["modelHash"]
    assert arguments["operations"] == json.loads(fixture.FIXTURE.read_text())["routes"]["PF"]["operations"]
    done = ask(script, [user(fixture.TURNS[0]), output(summary), output({"status": "pending_review", "basis": BASIS,
                                                                         "data": {"proposalHandle": "p" * 48}})])
    assert done["type"] == "message" and not script.failures


def test_turn_two_observes_on_the_proposal_basis_then_solves_and_reads_node_c():
    script = fixture.Script(fixture.TURNS)
    proposed = {"status": "pending_review", "basis": BASIS, "data": {"proposalHandle": "p" * 48}}
    later = {**BASIS, "modelRevision": 2, "modelHash": {"value": "b" * 64}}
    summary = {"status": "ok", "basis": later, "data": {"workspaceHandle": BASIS["workspaceHandle"]}}
    history = [user(fixture.TURNS[0]), output(proposed), user(fixture.TURNS[1])]
    assert ask(script, history)["name"] == "swbpipe_read_model_summary"
    observe = ask(script, history + [output(summary)])
    assert observe["name"] == "swbpipe_observe_proposal" and observe["arguments"]["basis"] == BASIS
    solve = ask(script, history + [output(summary), output({"status": "applied", "basis": BASIS, "data": {}})])
    assert solve["name"] == "swbpipe_solve" and solve["arguments"]["basis"] == later
    solved = {"status": "completed", "basis": later, "data": {"runHandle": "r" * 48}}
    read = ask(script, history + [output(summary), output({"status": "applied"}), output(solved)])
    assert read["name"] == "swbpipe_read_results"
    assert read["arguments"]["filter"] == {"kinds": fixture.NODAL_KINDS, "entityRefs": ["node:c"]}
    assert read["arguments"]["basis"] == later and read["arguments"]["runHandle"] == "r" * 48


def test_anything_unexpected_ends_the_turn_and_is_recorded():
    script = fixture.Script(fixture.TURNS)
    refused = ask(script, [user(fixture.TURNS[0]), output({"status": "ok", "basis": BASIS, "data": {"workspaceHandle": "w"}}),
                           output({"status": "unavailable", "message": "The operation engine refused the proposal"})])
    assert refused["type"] == "message" and refused["text"].startswith("STUB-UNEXPECTED")
    assert ask(script, [user("something else")])["text"].startswith("STUB-UNEXPECTED")
    assert script.failures == ["proposal unavailable", "unrecognized user message"]


def events(body):
    return [json.loads(block.split("data: ", 1)[1]) for block in body.decode().strip().split("\n\n")]


def test_the_event_stream_carries_one_completed_item():
    call = events(fixture.sse({"type": "function_call", "name": "swbpipe_solve", "arguments": {"a": 1}}, 3))
    assert [e["type"] for e in call] == ["response.created", "response.output_item.added", "response.output_item.done", "response.completed"]
    assert call[2]["item"]["arguments"] == '{"a": 1}' and call[2]["item"]["call_id"] == "call_stub_3"
    assert call[3]["response"]["usage"]["total_tokens"] == 2
    message = events(fixture.sse({"type": "message", "text": "done"}, 4))
    assert message[2]["type"] == "response.output_text.delta" and message[3]["item"]["content"][0]["text"] == "done"


def test_the_stub_listens_on_loopback_and_answers_only_responses():
    with fixture.StubProvider(fixture.TURNS) as stub:
        assert stub.endpoint.startswith("http://127.0.0.1:")
        request = urllib.request.Request(stub.endpoint + "/responses", method="POST",
                                         data=json.dumps({"input": [user(fixture.TURNS[0])], "tools": TOOLS}).encode())
        with urllib.request.urlopen(request, timeout=5) as reply:
            assert reply.headers["Content-Type"] == "text/event-stream"
            assert events(reply.read())[2]["item"]["name"] == "swbpipe_read_model_summary"
        try:
            urllib.request.urlopen(stub.endpoint + "/models", timeout=5)
            raise AssertionError("GET answered")
        except urllib.error.HTTPError as error:
            assert error.code == 404
        assert [r["path"] for r in stub.requests] == ["/v1/responses", "/v1/models"]


def observed_report(reference, operations, nudge=0.0):
    rows = []
    for r in reference["rows"]:
        if r["quantity"] == "nodal_motion" and r["entity"] == "node:c":
            displacement = r["component"].startswith("u")
            kind = f"global_nodal_{'displacement' if displacement else 'rotation'}_{r['component'][1]}"
            value = r["value"] * 1000.0 if displacement else r["value"]
            rows.append({"id": f"result:case:{kind}:node:c:", "text": f"Entered: {value + nudge!r} {'mm' if displacement else 'rad'}"})
    return {"steps": fixture.STEPS, "batch": {"batch_id": "agent-x", "batches_shown": 1,
            "summary": f"{len(operations)} proposed changes. Bound to the model used to prepare this batch.",
            "apply_message": "Applied"}, "tree": {"node:node:a": True}, "results": {"rows": rows, "model_sha256": None}}


def test_the_judge_accepts_the_reference_and_names_each_departure():
    reference = fixture.w9a_reference()
    operations = json.loads(fixture.FIXTURE.read_text())["routes"]["PF"]["operations"]
    assert fixture.judge(observed_report(reference, operations), reference, operations) == []
    wrong = fixture.judge(observed_report(reference, operations, nudge=1e-3), reference, operations)
    assert wrong and all(re.match(r"node:c [ur][xyz]: shown", why) for why in wrong)
    missing = observed_report(reference, operations)
    missing["steps"] = fixture.STEPS[:-1]
    missing["batch"]["batches_shown"] = 2
    assert fixture.judge(missing, reference, operations)[:2] == [f"steps {fixture.STEPS[:-1]}", "batch review did not show exactly one agent batch"]


def test_the_driver_calls_no_command_that_could_apply():
    driver = fixture.DRIVER.read_text()
    assert set(re.findall(r'call\("([a-z_]+)"', driver)) == {"native_smoke_log", "native_smoke_menu", "native_smoke_plan", "native_smoke_finish"}
    assert "invoke(" not in driver.replace("ipc.invoke(command", "")
