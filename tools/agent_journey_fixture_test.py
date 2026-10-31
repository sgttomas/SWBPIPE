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


def test_the_stub_reports_one_record_per_component():
    script = fixture.Script(fixture.TURNS)
    proposed = {"status": "pending_review", "basis": BASIS, "data": {"proposalHandle": "p" * 48}}
    summary = {"status": "ok", "basis": BASIS, "data": {"workspaceHandle": BASIS["workspaceHandle"]}}
    values = [{"id": f"r{i}", "kind": kind, "entityRef": "node:c", "value": i / 4, "unit": "mm" if "displacement" in kind else "rad"}
              for i, kind in enumerate(fixture.NODAL_KINDS)]
    done = ask(script, [user(fixture.TURNS[0]), output(proposed), user(fixture.TURNS[1]), output(summary),
                        output({"status": "applied"}), output({"status": "completed", "basis": BASIS, "data": {"runHandle": "r"}}),
                        output({"status": "ok", "data": {"values": values}})])
    assert fixture.REPORT_RECORD.findall(done["text"]) == [(fixture.COMPONENT[v["kind"]], repr(v["value"]), v["unit"]) for v in values]


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
    rows, values = [], []
    for r in reference["rows"]:
        if r["quantity"] == "nodal_motion" and r["entity"] == "node:c":
            displacement = r["component"].startswith("u")
            value = r["value"] * 1000.0 + nudge if displacement else r["value"] + nudge
            unit = "mm" if displacement else "rad"
            row_id = f"result:disp:node-c:{r['component']}"
            rows.append({"id": row_id, "text": f"Entered: {value!r} {unit}"})
            values.append({"id": row_id, "kind": f"global_nodal_{'displacement' if displacement else 'rotation'}_{r['component'][1]}",
                           "entityRef": "node:c", "value": value, "unit": unit})
    agent = "Agent: The batch is applied and the model solved.\n" + "\n".join(
        f"node:c {fixture.COMPONENT[v['kind']]} = {v['value']!r} {v['unit']}" for v in values)
    rows_expected = [[fixture.TREE_TYPES[op["target"]["object_type"]], op["target"]["ref"]]
                     for op in operations if op["target"]["object_type"] in fixture.TREE_TYPES]
    report = {"steps": fixture.STEPS,
              "turn1": {"standing": "Ready.", "messages": ["You: build", "Agent: proposed"]},
              "turn2": {"standing": "Ready.", "messages": ["You: solve", agent]},
              "batch": {"batch_id": "agent-x", "batches_shown": 1,
                        "summary": f"{len(operations)} proposed changes. Bound to the model used to prepare this batch.",
                        "submitted": [fixture.operation_key(op) for op in operations], "apply_message": "Applied"},
              "tree_rows": [list(row) for row in rows_expected],
              "results": {"rows": rows, "model_sha256": "sha256:" + "c" * 64}}
    hash_ = {"value": "c" * 64}
    captured = [{"outcome": {"status": "applied", "data": {"outcomeModelHash": hash_}}},
                {"outcome": {"status": "completed", "data": {"evaluatedModelHash": hash_}}},
                {"outcome": {"status": "ok", "data": {"values": values}}}]
    return report, captured


def judged(mutate=None, captured_mutate=None, nudge=0.0, live=False):
    reference = fixture.w9a_reference()
    operations = json.loads(fixture.FIXTURE.read_text())["routes"]["PF"]["operations"]
    report, captured = observed_report(reference, operations, nudge)
    if mutate:
        mutate(report)
    if captured_mutate:
        captured_mutate(captured)
    return fixture.judge(report, reference, operations, None if live else captured)


def test_the_judge_accepts_the_reference_and_names_each_departure():
    assert judged() == []
    wrong = judged(nudge=1e-3)
    assert wrong and all(re.match(r"node:c [ur][xyz]: shown", why) for why in wrong if why.startswith("node:c"))
    def missing(report):
        report["steps"] = fixture.STEPS[:-1]
        report["batch"]["batches_shown"] = 2
    assert judged(missing)[:2] == [f"steps {fixture.STEPS[:-1]}", "batch review did not show exactly one agent batch"]


def test_the_judge_refuses_a_failed_or_silent_turn():
    def failed(report):
        report["turn2"]["standing"] = "Ready. The last turn failed."
    assert "turn2 standing 'Ready. The last turn failed.'" in judged(failed)
    def silent(report):
        report["turn2"]["messages"] = ["You: solve"]
    assert "turn2: no agent message in the window" in judged(silent)
    def unreported(report):
        report["turn2"]["messages"] = ["You: solve", "Agent: done"]
    assert sum("times" in why for why in judged(unreported)) == 6


def test_the_report_must_give_each_component_of_node_c_in_its_unit():
    assert judged(live=True) == []
    def wrong(report):
        report["turn2"]["messages"] = ["You: solve", "Agent: done. node:c ux = 999 mm; no rotations reported."]
    assert sum("the agent's report states" in why for why in judged(wrong, live=True)) == 6
    rounded = ("Agent: node:c ux = 4.77111 mm\nnode:c uy = -3.58687 mm\nnode:c uz = 0 mm\n"
               "node:c rx = 0 rad\nnode:c ry = 0 rad\nnode:c rz = 0.000786714 rad")
    def round_off(report):
        report["turn2"]["messages"] = ["You: solve", rounded]
    assert judged(round_off, live=True) == []
    assert any("the agent's report states" in why for why in judged(round_off))  # the stub states values exactly
    # The second review's counterexample: another node, ux/uy swapped, wrong units, rz's value
    # given as uz, two rotations missing, one zero standing for every zero component.
    def misattributed(report):
        report["turn2"]["messages"] = ["You: solve", "Agent: node:z ux = 4.77111 mm\nnode:c ux = -3.58687 mm\n"
                                       "node:c uy = 4.77111 m\nnode:c uz = 0.000786714 rad\nnode:c rz = 0 rad"]
    found = [why for why in judged(misattributed, live=True) if "the agent's report states" in why]
    assert len(found) == 6, found
    def duplicated(report):
        report["turn2"]["messages"] = ["You: solve", rounded + "\nnode:c ux = 4.77111 mm"]
    assert "the agent's report states node:c ux 2 times" in judged(duplicated, live=True)


def test_the_judge_refuses_nonfinite_values():
    def nan(report):
        for row in report["results"]["rows"]:
            row["text"] = "Entered: NaN " + row["text"].rsplit(" ", 1)[1]
    found = judged(nan)
    assert sum("is not finite" in why for why in found) == 6


def test_the_judge_requires_the_agent_to_read_exactly_the_window_rows():
    def duplicates(captured):
        captured[2]["outcome"]["data"]["values"] = [{"id": "result:disp:node-a:ux", "kind": "global_nodal_displacement_x",
                                                      "entityRef": "node:a", "value": 999, "unit": "N"}] * 6
    assert any(why.startswith("agent read ") for why in judged(captured_mutate=duplicates))
    def wrong_unit(captured):
        captured[2]["outcome"]["data"]["values"][0]["unit"] = "m"
    assert any("differs from the window" in why for why in judged(captured_mutate=wrong_unit))


def test_the_judge_checks_the_batch_and_tree_exactly():
    def more(report):
        report["batch"]["summary"] = "111 proposed changes. Bound to the model used to prepare this batch."
    assert any(why.startswith("batch summary") for why in judged(more))
    def renamed(report):
        report["batch"]["submitted"][-1][0] = report["batch"]["submitted"][-1][0] + "-wrong"
    assert "the submitted batch is not the fixture's operations in order" in judged(renamed)
    def reordered(report):
        report["batch"]["submitted"].reverse()
    assert "the submitted batch is not the fixture's operations in order" in judged(reordered)
    def partial(report):
        report["tree_rows"] = report["tree_rows"][:1]
    assert any(why.startswith("model tree rows") for why in judged(partial))
    def extra(report):
        report["tree_rows"].append(["node", "node:e"])
    assert any(why.startswith("model tree rows") for why in judged(extra))


class FakeApp:
    pid = 4321

    def __init__(self, alive=True):
        self.alive, self.killed = alive, False
    def poll(self):
        return None if self.alive else 0
    def kill(self):
        self.killed, self.alive = True, False
    def wait(self, timeout=None):
        return 0


def test_release_kills_the_owned_app_only_on_evidence():
    # Unknown custody (no anchor and no observed exit), or a second supplier: the app is kept.
    for anchor, exited, single in ((None, False, True), (object(), False, False), (None, True, False)):
        app = FakeApp()
        assert fixture.release_app(app, anchor, exited, single) == "retained_original_app_custody" and not app.killed
    for anchor, exited in ((object(), False), (None, True)):
        app = FakeApp()
        assert fixture.release_app(app, anchor, exited, True) == "killed_owned_app" and app.killed
    assert fixture.release_app(FakeApp(alive=False), None, False, True) is None


def test_the_gate_is_answered_once(tmp_path):
    gate = tmp_path / "gate"
    assert fixture.answer_gate(gate, b"READY\n") and gate.read_bytes() == b"READY\n"
    assert fixture.answer_gate(gate, b"REFUSE\n") and gate.read_bytes() == b"READY\n"
    assert oct(gate.stat().st_mode & 0o777) == "0o600"


def test_an_invalid_observation_is_not_evidence_of_exit():
    class Broken:
        def sample(self):
            raise fixture.Refusal("scope_observation")
    class Exited:
        def sample(self):
            return True
    assert fixture.supplier_gone(None) is False
    assert fixture.supplier_gone(Broken()) is False
    assert fixture.supplier_gone(Exited()) is True


def test_retained_custody_latches_the_host(tmp_path, monkeypatch):
    latch = tmp_path / "retained"
    monkeypatch.setattr(fixture, "RETAINED", latch)
    fixture.retain(FakeApp(), tmp_path / "run")
    record = json.loads(latch.read_text())
    assert set(record) == {"appPid", "output", "at"}
    monkeypatch.setattr(sys, "argv", ["agent_journey_fixture.py", "--app", "x", "--process-probe", "x", "--boundary-probe", "x",
                                      "--mode", "stub", "--output", str(tmp_path / "next")])
    # Checked under the host lock, before any key is read or run state created.
    assert fixture.main() == 2 and not (tmp_path / "next").exists()


def test_the_key_is_one_bounded_newline_terminated_printable_line():
    import io
    assert fixture.read_key(io.BytesIO(b"invented-key\nrest")) == "invented-key"
    for bad in (b"no-newline", b"\n", b"two words\n", b"tab\tkey\n", b"k" * (fixture.KEY_BYTES + 1) + b"\n", "\u00e9\n".encode()):
        try:
            fixture.read_key(io.BytesIO(bad))
            raise AssertionError(bad[:20])
        except fixture.Refusal as refusal:
            assert str(refusal) == "key_input_refused"


def test_the_driver_calls_no_command_that_could_apply():
    driver = fixture.DRIVER.read_text()
    assert set(re.findall(r'call\("([a-z_]+)"', driver)) == {"native_smoke_log", "native_smoke_menu", "native_smoke_plan", "native_smoke_finish"}
    assert "invoke(" not in driver.replace("ipc.invoke(command", "")
