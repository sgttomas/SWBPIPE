"""Offline checks of the agent journey controller (no app, no supplier, no model)."""
from __future__ import annotations

# The use cases are found by scanning their directory; each names repository files it reads.
CI_READS = ("fixtures/embedded_agent/use_cases/**", "fixtures/model_operations/**", "validation/references/w9_benchmarks/**")

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
# The W9-A journey as it was hard-coded before it became a definition: the same seven driver
# steps, the tree types and the nodal kinds the scripted provider reads.
SEVEN_STEPS = ["new blank", "provider and start", "turn 1", "apply", "turn 2", "observe", "stop"]
TREE_TYPES = {"Node": "node", "Element": "pipe", "Support": "support"}
NODAL_KINDS = [f"global_nodal_{q}_{axis}" for q in ("displacement", "rotation") for axis in "xyz"]
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
    script = fixture.Script(fixture.W9A)
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
    script = fixture.Script(fixture.W9A)
    proposed = {"status": "pending_review", "basis": BASIS, "data": {"proposalHandle": "p" * 48}}
    summary = {"status": "ok", "basis": BASIS, "data": {"workspaceHandle": BASIS["workspaceHandle"]}}
    values = [{"id": f"r{i}", "kind": kind, "entityRef": "node:c", "value": i / 4, "unit": "mm" if "displacement" in kind else "rad"}
              for i, kind in enumerate(NODAL_KINDS)]
    done = ask(script, [user(fixture.TURNS[0]), output(summary), output(proposed), user(fixture.TURNS[1]), output(summary),
                        output({"status": "applied"}), output({"status": "completed", "basis": BASIS, "data": {"runHandle": "r"}}),
                        output({"status": "ok", "data": {"values": values}})])
    assert fixture.report_record("node:c").findall(done["text"]) == [(fixture.COMPONENT[v["kind"]], repr(v["value"]), v["unit"]) for v in values]


def test_turn_two_observes_on_the_proposal_basis_then_solves_and_reads_node_c():
    script = fixture.Script(fixture.W9A)
    proposed = {"status": "pending_review", "basis": BASIS, "data": {"proposalHandle": "p" * 48}}
    later = {**BASIS, "modelRevision": 2, "modelHash": {"value": "b" * 64}}
    summary = {"status": "ok", "basis": later, "data": {"workspaceHandle": BASIS["workspaceHandle"]}}
    history = [user(fixture.TURNS[0]), output(summary), output(proposed), user(fixture.TURNS[1])]
    assert ask(script, history)["name"] == "swbpipe_read_model_summary"
    observe = ask(script, history + [output(summary)])
    assert observe["name"] == "swbpipe_observe_proposal" and observe["arguments"]["basis"] == BASIS
    solve = ask(script, history + [output(summary), output({"status": "applied", "basis": BASIS, "data": {}})])
    assert solve["name"] == "swbpipe_solve" and solve["arguments"]["basis"] == later
    solved = {"status": "completed", "basis": later, "data": {"runHandle": "r" * 48}}
    read = ask(script, history + [output(summary), output({"status": "applied"}), output(solved)])
    assert read["name"] == "swbpipe_read_results"
    assert read["arguments"]["filter"] == {"kinds": NODAL_KINDS, "entityRefs": ["node:c"]}
    assert read["arguments"]["basis"] == later and read["arguments"]["runHandle"] == "r" * 48


def test_anything_unexpected_ends_the_turn_and_is_recorded():
    script = fixture.Script(fixture.W9A)
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
    with fixture.StubProvider(fixture.W9A) as stub:
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
    rows_expected = [[TREE_TYPES[op["target"]["object_type"]], op["target"]["ref"]]
                     for op in operations if op["target"]["object_type"] in TREE_TYPES]
    report = {"steps": SEVEN_STEPS,
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


def judged(mutate=None, captured_mutate=None, nudge=0.0, live=False, case=None, judgement=None):
    reference = fixture.w9a_reference()
    operations = json.loads(fixture.FIXTURE.read_text())["routes"]["PF"]["operations"]
    report, captured = observed_report(reference, operations, nudge)
    if mutate:
        mutate(report)
    if captured_mutate:
        captured_mutate(captured)
    return fixture.judge(report, case or fixture.W9A, None if live else captured, judgement)


def test_the_judge_accepts_the_reference_and_names_each_departure():
    assert judged() == []
    wrong = judged(nudge=1e-3)
    assert wrong and all(re.match(r"node:c [ur][xyz]: shown", why) for why in wrong if why.startswith("node:c"))
    def missing(report):
        report["steps"] = SEVEN_STEPS[:-1]
        report["batch"]["batches_shown"] = 2
    assert judged(missing)[:2] == [f"steps {SEVEN_STEPS[:-1]}", "batch review did not show exactly one agent batch"]


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
    @property
    def returncode(self):
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


def test_gate_io_failure_still_reaches_the_retained_custody_latch(tmp_path, monkeypatch):
    # A missing gate directory makes os.open fail inside final cleanup, without a launch.
    latch = tmp_path / "retained"
    monkeypatch.setattr(fixture, "RETAINED", latch)
    app, result = FakeApp(), {"passed": False}
    lines = fixture.Lines.__new__(fixture.Lines)
    lines.lines = ['[native-smoke-agent-scope] {"sdkPid": 123}']
    fixture.release_launch(app, None, None, lines, tmp_path / "absent" / "gate", 123,
                           False, tmp_path / "run", 0, result)
    assert result["gateAnswerFailed"] and result["failure"] == "gate_answer_failed"
    assert result["release"] == "retained_original_app_custody" and not app.killed
    assert json.loads(latch.read_text())["appPid"] == app.pid


def test_malformed_scope_evidence_keeps_and_latches_the_app_even_with_an_anchor(tmp_path, monkeypatch):
    latch = tmp_path / "retained"
    monkeypatch.setattr(fixture, "RETAINED", latch)
    app, result = FakeApp(), {"passed": False}
    lines = fixture.Lines.__new__(fixture.Lines)
    lines.lines = ["[native-smoke-agent-scope] {broken"]
    fixture.release_launch(app, object(), None, lines, tmp_path / "gate", 123,
                           True, tmp_path / "run", 0, result)
    assert result["release"] == "retained_original_app_custody" and not app.killed
    assert latch.exists()


def test_gate_write_failure_is_refused_without_echo(tmp_path, monkeypatch):
    class BrokenWrite:
        def __enter__(self):
            return self
        def __exit__(self, *args):
            pass
        def write(self, value):
            raise OSError("invented write refusal")
    monkeypatch.setattr(fixture.os, "open", lambda *args: 123)
    monkeypatch.setattr(fixture.os, "fdopen", lambda *args: BrokenWrite())
    assert fixture.answer_gate(tmp_path / "gate", b"REFUSE\n") is False


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


# ---------------------------------------------------------------------------
# Use cases as data: definitions, scripted providers and what the judge must examine.

import copy  # noqa: E402

import pytest  # noqa: E402


def w9a_definition():
    return copy.deepcopy(fixture.W9A.definition)


def case_from(definition):
    problems = fixture.validate_definition(definition, definition.get("id"))
    assert not problems, problems
    return fixture.UseCase(fixture.W9A.directory, definition)


def test_every_use_case_definition_and_its_scripted_provider_validate():
    names = fixture.use_case_names()
    assert fixture.DEFAULT_USE_CASE in names
    for name in names:
        case = fixture.load_use_case(name)
        assert case.script_path.is_file(), name
        program, inputs = fixture.load_scripted_provider(case, "stub")
        assert program["use_case"] == name


def test_the_w9a_definition_keeps_the_seven_steps_and_hands_the_driver_only_the_persons_side():
    case = fixture.W9A
    assert case.steps == SEVEN_STEPS
    assert fixture.TURNS == case.requests and len(case.requests) == 2
    plan = case.plan({"endpoint": "http://127.0.0.1:1/v1", "model": "m", "contextWindow": 1}, {"ui": 1})
    assert set(plan) == {"useCase", "provider", "start", "sequence", "observe", "timeoutsMs"}
    assert plan["start"] == {"kind": "new_blank", "step": "new blank"}
    assert [e["step"] for e in plan["sequence"]] == ["turn 1", "apply", "turn 2"]
    assert [e.get("report") for e in plan["sequence"]] == ["turn1", "batch", "turn2"]
    assert plan["observe"] == {"treeTypes": sorted(TREE_TYPES.values()), "resultProbe": "node:c"}
    # No expectation, reference or scripted output reaches the window.
    text = json.dumps(plan)
    assert "expectations" not in text and "references.json" not in text and "STUB" not in text


def driver_kinds(table):
    found = re.search(r"const " + table + r" = \{([^}]*)\};", fixture.DRIVER.read_text())
    return set(re.findall(r"(\w+):", found.group(1)))


def test_the_driver_implements_exactly_the_starts_and_acts_the_controller_admits():
    assert driver_kinds("STARTS") == set(fixture.START_KINDS)
    assert driver_kinds("ACTS") == set(fixture.ACT_KINDS)


def test_a_definition_naming_a_start_or_act_the_driver_lacks_is_refused(tmp_path, monkeypatch):
    unknown_act = w9a_definition()
    unknown_act["sequence"][1] = {"act": "confirm_requested_act"}
    assert any("sequence[1]" in p for p in fixture.validate_definition(unknown_act, "w9a-journey"))
    unknown_start = w9a_definition()
    unknown_start["start"] = {"kind": "open_model_document"}
    assert any(p.startswith("start ") for p in fixture.validate_definition(unknown_start, "w9a-journey"))
    # Through the loader, before anything is launched.
    (tmp_path / "w9a-journey").mkdir()
    (tmp_path / "w9a-journey" / "use_case.json").write_text(json.dumps(unknown_act))
    monkeypatch.setattr(fixture, "USE_CASES", tmp_path)
    with pytest.raises(fixture.Refusal, match="use_case_invalid"):
        fixture.load_use_case("w9a-journey")
    with pytest.raises(fixture.Refusal, match="use_case_name"):
        fixture.load_use_case("../w9a-journey")


def without(kind):
    definition = w9a_definition()
    definition["expectations"] = [e for e in definition["expectations"] if e["kind"] != kind]
    return definition


def test_a_definition_must_judge_the_window_the_conversation_and_their_agreement_in_every_mode():
    assert "no expectation judges the conversation in every mode" in fixture.validate_definition(without("requests_answered"), "w9a-journey")
    # The tool-outcome agreement is examined only under the scripted provider, so it alone does
    # not cover the agreement.
    assert "no expectation judges the agreement in every mode" in fixture.validate_definition(without("report_states_window_motions"), "w9a-journey")
    window_only_by_stub = w9a_definition()
    window_only_by_stub["expectations"] = [e for e in window_only_by_stub["expectations"] if e["on"] != "window"]
    assert "no expectation judges the window in every mode" in fixture.validate_definition(window_only_by_stub, "w9a-journey")
    empty = w9a_definition()
    empty["expectations"] = []
    assert "no expectations" in fixture.validate_definition(empty, "w9a-journey")


def test_each_expectation_is_one_the_judge_implements_with_its_own_parameters_and_order():
    def changed(change):
        definition = w9a_definition()
        change(definition["expectations"])
        return fixture.validate_definition(definition, "w9a-journey")
    assert any("is not one the judge implements" in p for p in changed(lambda e: e[0].update(kind="looks_fine")))
    assert any("judges the conversation" in p for p in changed(lambda e: e[0].update(on="window")))
    assert any("parameters" in p for p in changed(lambda e: e[3].pop("rows")))
    assert any("parameters" in p for p in changed(lambda e: e[1].update(extra=1)))
    assert any("needs an earlier nodal_motions_match_reference" in p for p in changed(lambda e: e.insert(0, e.pop(5))))
    assert any("apply 2" in p for p in changed(lambda e: e[1].update(apply=2)))
    assert any("request 3" in p for p in changed(lambda e: e[5].update(request=3)))
    assert any("repeated" in p for p in changed(lambda e: e[1].update(id=e[0]["id"])))


def test_a_definition_names_no_provider_and_no_request_hides_inside_another():
    definition = w9a_definition()
    definition["start"]["endpoint"] = "http://127.0.0.1:8000/v1"
    definition["observe"]["model"] = "any"
    problems = fixture.validate_definition(definition, "w9a-journey")
    assert "$.start.endpoint names a provider" in problems and "$.observe.model names a provider" in problems
    nested = w9a_definition()
    nested["sequence"].append({"request": nested["sequence"][0]["request"][:40]})
    assert any("is contained in" in p for p in fixture.validate_definition(nested, "w9a-journey"))


def test_every_expectation_is_judged_under_the_scripted_provider_and_named_when_live_cannot():
    stub, live = {}, {}
    assert judged(judgement=stub) == [] and judged(live=True, judgement=live) == []
    ids = [e["id"] for e in fixture.W9A.expectations]
    assert stub == {"judged": ids, "notJudged": [], "observedByPerson": [], "requestedActs": {"standing": "none", "acts": []}}
    assert live["notJudged"] == ["tool-outcomes-agree"] and live["judged"] == [i for i in ids if i != "tool-outcomes-agree"]


def test_an_expectation_that_compares_nothing_fails():
    definition = w9a_definition()
    for expectation in definition["expectations"]:
        if "entity" in expectation:
            expectation["entity"] = "node:z"
    found = judged(case=case_from(definition))
    assert "the reference has 0 nodal motions of node:z, the use case expects 6" in found
    assert "expectation node-c-motions compared nothing" in found
    assert "expectation report-states-window compared nothing" in found


def test_a_check_that_reports_nothing_cannot_pass(monkeypatch):
    check = fixture.CHECKS["agent_batch_shown"]
    monkeypatch.setitem(fixture.CHECKS, "agent_batch_shown",
                        fixture.Check(check.category, check.params, lambda *a: ([], 0), check.validate))
    assert judged() == ["expectation one-agent-batch compared nothing"]
    def broken(*a):
        raise KeyError("x")
    monkeypatch.setitem(fixture.CHECKS, "agent_batch_shown", fixture.Check(check.category, check.params, broken, check.validate))
    assert judged() == ["expectation one-agent-batch could not be judged: KeyError"]


def test_the_scripted_provider_is_never_read_or_served_in_live_mode(monkeypatch, tmp_path):
    with pytest.raises(fixture.Refusal, match="scripted_provider_outside_stub_mode"):
        fixture.load_scripted_provider(fixture.W9A, "live")
    monkeypatch.setattr(fixture, "load_scripted_provider", lambda *a: pytest.fail("the script was read"))
    assert fixture.scripted_provider_for("live", fixture.W9A) is None
    with pytest.raises(fixture.Refusal):
        fixture.scripted_provider_for("replay", fixture.W9A)
    # And a stub run takes no endpoint of its own, so no other server can stand in for the script.
    monkeypatch.setattr(fixture, "RETAINED", tmp_path / "retained")
    monkeypatch.setattr(sys, "argv", ["agent_journey_fixture.py", "--app", "x", "--process-probe", "x", "--boundary-probe", "x",
                                      "--mode", "stub", "--endpoint", "http://127.0.0.1:8000/v1", "--output", str(tmp_path / "out")])
    with pytest.raises(SystemExit):
        fixture.main()
    assert not (tmp_path / "out").exists()


def test_a_stub_run_states_its_standing_as_a_route_examined_not_a_use_case_carried_out():
    assert fixture.STANDING["stub"] == "the route is examined under a scripted provider; no model ran"
    texts = [fixture.STANDING["stub"], fixture.__doc__, fixture.DRIVER.read_text(),
             *(p.read_text() for p in fixture.USE_CASES.rglob("*") if p.is_file())]
    for text in texts:
        assert not re.search(r"(agent|use case) (has )?(completed|carried out|passed) (the|a|this) use case", text, re.I)


def test_scripted_outcomes_are_expected_by_field_so_added_members_do_not_disturb_them():
    script = fixture.Script(fixture.W9A)
    summary = {"status": "ok", "basis": BASIS, "data": {"workspaceHandle": BASIS["workspaceHandle"], "addedLater": [1]}, "extra": True}
    proposed = {"status": "pending_review", "basis": BASIS, "data": {"proposalHandle": "p" * 48}, "notes": "added"}
    history = [user(fixture.TURNS[0]), output(summary), output(proposed), user(fixture.TURNS[1]), output(summary),
               output({"status": "applied", "reason": None})]
    solve = ask(script, history)
    assert solve["name"] == "swbpipe_solve" and not script.failures
    # A refused solve, in whatever shape, ends the turn with its status named.
    refused = ask(script, history + [output({"status": "refused", "data": {"findings": [{"code": "X"}]}})])
    assert refused["text"] == "STUB-UNEXPECTED: solve refused"
    assert fixture.matches({"a": {"b": 1}, "c": 2}, {"a.b": 1}) and not fixture.matches({"a": {}}, {"a.b": 1})


def test_a_script_is_refused_unless_it_scripts_its_own_use_case_in_order():
    program, _ = fixture.load_scripted_provider(fixture.W9A, "stub")
    def problems(change):
        changed = copy.deepcopy(program)
        change(changed)
        return fixture.validate_scripted_provider(changed, fixture.W9A)
    assert problems(lambda p: None) == []
    assert any("use_case" in x for x in problems(lambda p: p.update(use_case="other")))
    assert any("in order" in x for x in problems(lambda p: p["requests"].reverse()))
    assert any("in order" in x for x in problems(lambda p: p["requests"].pop()))
    assert any("not available" in x for x in problems(lambda p: p["requests"][0]["steps"][0]["arguments"].update(
        later={"$bound": "results"})))
    assert any("requires" in x for x in problems(lambda p: p["requests"][0].update(
        requires=[{"bound": "results", "expect": {}, "unexpected": "x"}])))


def test_the_scripted_provider_replays_the_captured_w9a_outcomes_in_order():
    outcomes = json.loads((fixture.PROJECT / "fixtures/model_operations/agent_w9a_capture_outcomes.json").read_text())["outcomes"]
    script = fixture.Script(fixture.W9A)
    history = [{"type": "message", "role": "user", "content": "<environment_context/>"}, user(fixture.TURNS[0])]
    answers = [ask(script, history)]
    for index, outcome in enumerate(outcomes):
        if index == 2:
            history.append(user(fixture.TURNS[1]))
            answers.append(ask(script, history))
        history.append(output(outcome))
        answers.append(ask(script, history))
    assert [a.get("name", "message") for a in answers] == [
        "swbpipe_read_model_summary", "swbpipe_propose_operations", "message", "swbpipe_read_model_summary",
        "swbpipe_observe_proposal", "swbpipe_solve", "swbpipe_read_results", "message"]
    assert not script.failures and len(script.captured) == 6
    assert len(fixture.report_record("node:c").findall(answers[-1]["text"])) == 6


# ---------------------------------------------------------------------------
# Requested acts: settled only as performed; anything else leaves the use case unfinished.

def with_requested_act(act="save_model_state", expectation=True):
    """An invented definition: the W9-A journey with one requested act after the last request."""
    definition = w9a_definition()
    definition["sequence"].append({"requested_act": act})
    if expectation:
        definition["expectations"].append({"id": "requested-standing", "kind": "requested_act_standing",
                                           "on": "conversation", "requested": 1})
    return definition


def judged_request(act, standings, live=False):
    """The invented use case judged on an invented conversation whose request observations
    give `standings` in order (the last is the one that stands)."""
    case = case_from(with_requested_act(act))
    reference = fixture.w9a_reference()
    operations = json.loads(fixture.FIXTURE.read_text())["routes"]["PF"]["operations"]
    report, captured = observed_report(reference, operations)
    report["steps"] = case.steps
    captured += [{"outcome": {"status": "ok", "data": {"requestedAct": act, "requestStanding": s}}} for s in standings]
    judgement = {}
    failures = fixture.judge(report, case, None if live else captured, judgement)
    result = {"passed": not failures, "mode": "live" if live else "stub", "expectations": judgement}
    return failures, judgement["requestedActs"], fixture.settle(result), result


def test_a_requested_act_settled_as_performed_lets_the_use_case_pass():
    failures, acts, outcome, result = judged_request("save_model_state", ["pending", "performed"])
    assert failures == [] and acts["standing"] == "all performed" and result["passed"] is True
    # Under a scripted provider a pass is the route examined, never the use case done by the agent.
    assert outcome == "route examined under a scripted provider" and result["runnerMode"] == "agent with a scripted provider"
    assert acts["acts"] == [{"requested": 1, "act": "save_model_state", "standing": "performed"}]


@pytest.mark.parametrize("standing", ["pending", "declined", "refused", "stale", "withdrawn", "unknown"])
def test_any_other_standing_leaves_the_use_case_unfinished_and_never_passed(standing):
    failures, acts, outcome, result = judged_request("export_table", ["performed", standing] if standing != "pending" else [standing])
    assert failures == [], "an unfinished use case is not a failure of the route"
    assert acts["standing"] == "unfinished" and acts["acts"][0]["standing"] == standing
    assert outcome == "unfinished" and result["passed"] is False
    assert fixture.OUTCOME_EXIT[outcome] not in (0, fixture.OUTCOME_EXIT["failed"])


def test_a_print_request_sent_to_the_print_dialog_is_unfinished_and_never_performed():
    failures, acts, outcome, _ = judged_request("print_report", ["sent to the print dialog"])
    assert failures == [] and acts["standing"] == "unfinished" and outcome == "unfinished"
    failures, _, outcome, _ = judged_request("print_report", ["performed"])
    assert failures == ["print_report was reported performed; a print request is settled at most as sent to the print dialog"]
    assert outcome == "failed"


def test_a_standing_not_observed_or_not_recognised_is_unfinished():
    for standings, word in (([], "not observed"), (["done"], "unrecognized"), ([None], "unrecognized")):
        failures, acts, outcome, _ = judged_request("save_project", standings)
        assert failures == [] and acts["acts"][0]["standing"] == word and outcome == "unfinished"
    # A live run does not see the agent's tool outcomes, so it cannot show a performed standing.
    failures, acts, outcome, _ = judged_request("record_run", ["performed"], live=True)
    assert acts["acts"][0]["standing"] == "not observed" and outcome == "unfinished"


def test_another_acts_standing_settles_nothing():
    case = case_from(with_requested_act("save_report"))
    reference = fixture.w9a_reference()
    operations = json.loads(fixture.FIXTURE.read_text())["routes"]["PF"]["operations"]
    report, captured = observed_report(reference, operations)
    report["steps"] = case.steps
    captured.append({"outcome": {"status": "ok", "data": {"requestedAct": "save_project", "requestStanding": "performed"}}})
    judgement = {}
    assert fixture.judge(report, case, captured, judgement) == []
    assert judgement["requestedActs"]["standing"] == "unfinished"


def test_a_use_case_without_requested_acts_is_settled_by_the_route_alone():
    judgement = {}
    assert judged(judgement=judgement) == [] and judgement["requestedActs"] == {"standing": "none", "acts": []}
    assert fixture.settle({"passed": True, "mode": "live", "expectations": judgement}) == "passed"
    assert fixture.settle({"passed": True, "mode": "stub", "expectations": judgement}) == "route examined under a scripted provider"
    assert fixture.settle({"passed": False, "mode": "stub", "expectations": judgement}) == "failed"
    assert fixture.W9A.requested_acts == []


def test_a_requested_act_needs_its_standing_judged_and_must_be_on_the_closed_list():
    unjudged = with_requested_act(expectation=False)
    assert any("needs exactly one requested_act_standing expectation, has 0" in p
               for p in fixture.validate_definition(unjudged, "w9a-journey"))
    twice = with_requested_act()
    twice["expectations"].append(dict(twice["expectations"][-1], id="again"))
    assert any("has 2" in p for p in fixture.validate_definition(twice, "w9a-journey"))
    off_list = with_requested_act("send_email")
    assert any("sequence[3]" in p for p in fixture.validate_definition(off_list, "w9a-journey"))
    out_of_range = with_requested_act()
    out_of_range["expectations"][-1]["requested"] = 2
    assert any("requested 2" in p for p in fixture.validate_definition(out_of_range, "w9a-journey"))
    for act in fixture.REQUESTABLE_ACTS:
        assert fixture.validate_definition(with_requested_act(act), "w9a-journey") == []


def test_a_requested_act_the_driver_cannot_perform_refuses_the_launch():
    case = case_from(with_requested_act("save_project"))
    assert case.steps[-3:] == ["perform save_project", "observe", "stop"]
    assert case.plan({}, {})["sequence"][-1] == {"step": "perform save_project", "requestedAct": "save_project", "report": "requested1"}
    with pytest.raises(fixture.Refusal, match="requested_act_not_performable"):
        fixture.refuse_unperformable(case)
    fixture.refuse_unperformable(fixture.W9A)
    assert driver_kinds("PERFORMS") == set(fixture.PERFORMED_BY_DRIVER)


def test_the_summary_line_and_exit_status_carry_the_outcome(tmp_path, monkeypatch, capsys):
    monkeypatch.setattr(fixture, "RETAINED", tmp_path / "retained")
    monkeypatch.setattr(fixture.fcntl, "flock", lambda *a: None)
    real_open = open
    monkeypatch.setattr(fixture, "open", lambda path, *a, **kw: real_open(tmp_path / "lock", *a, **kw), raising=False)
    for name in ("app", "probe"):
        (tmp_path / name).touch()
    monkeypatch.setattr(fixture, "free_bytes", lambda: 10 ** 15)
    unfinished = {"passed": True, "expectations": {"requestedActs": {"standing": "unfinished", "acts": []}}}
    monkeypatch.setattr(fixture, "run", lambda *a: dict(unfinished))
    monkeypatch.setattr(sys, "argv", ["agent_journey_fixture.py", "--app", str(tmp_path / "app"), "--process-probe",
                                      str(tmp_path / "probe"), "--boundary-probe", str(tmp_path / "probe"),
                                      "--mode", "stub", "--output", str(tmp_path / "out")])
    assert fixture.main() == fixture.OUTCOME_EXIT["unfinished"] == 3
    summary = json.loads(capsys.readouterr().out.strip().splitlines()[-1])
    assert summary["outcome"] == "unfinished" and summary["passed"] is False and not summary["failures"]
    assert summary["runnerMode"] == "agent with a scripted provider"
    written = json.loads((tmp_path / "out" / "result.json").read_text())
    assert written["outcome"] == "unfinished" and written["passed"] is False


# ---------------------------------------------------------------------------
# Things a person observes: never passed or failed by the judge, listed, and never a pass.

def with_person(thing="copy_pressed_on_selected_table", **override):
    definition = w9a_definition()
    expectation = {"id": "copy-by-person", "kind": "observed_by_person", "on": "person", "thing": thing,
                   "machine_observed": "The node:c rows were shown and selectable, and are what Copy would take.",
                   "machine_check": "node-c-motions"}
    expectation.update(override)
    definition["expectations"].append(expectation)
    return definition


def test_a_thing_a_person_observes_is_listed_never_judged_and_the_run_awaits_a_person():
    case = case_from(with_person())
    reference = fixture.w9a_reference()
    operations = json.loads(fixture.FIXTURE.read_text())["routes"]["PF"]["operations"]
    report, captured = observed_report(reference, operations)
    judgement = {}
    failures = fixture.judge(report, case, captured, judgement)
    assert failures == [] and "copy-by-person" not in judgement["judged"] + judgement["notJudged"]
    assert judgement["observedByPerson"] == [{"id": "copy-by-person", "thing": "copy_pressed_on_selected_table",
                                               "machineObserved": case.expectations[-1]["machine_observed"],
                                               "machineCheck": "node-c-motions", "machineCheckJudged": True}]
    result = {"passed": True, "mode": "stub", "expectations": judgement}
    assert fixture.settle(result) == "awaiting a person" and result["passed"] is False
    assert fixture.OUTCOME_EXIT["awaiting a person"] not in (0, 1, fixture.OUTCOME_EXIT["unfinished"])
    # A failure of the route still fails; a missing receipt still reads as unfinished first.
    assert fixture.settle({"passed": False, "mode": "stub", "expectations": judgement}) == "failed"
    both = dict(judgement, requestedActs={"standing": "unfinished", "acts": []})
    assert fixture.settle({"passed": True, "mode": "live", "expectations": both}) == "unfinished"


def test_a_person_observed_thing_must_name_what_the_machine_observed_and_its_check():
    assert fixture.validate_definition(with_person("print_dialog_outcome"), "w9a-journey") == []
    def problems(**override):
        return fixture.validate_definition(with_person(**override), "w9a-journey")
    assert any("is not one a person observes" in p for p in problems(thing="window_looked_right"))
    assert any("machine_observed must say" in p for p in problems(machine_observed=""))
    assert any("machine_observed must say" in p for p in problems(machine_observed="nothing"))
    assert any("names no earlier expectation" in p for p in problems(machine_check="no-such-check"))
    assert any("names no earlier expectation" in p for p in problems(machine_check=None))
    missing = with_person()
    del missing["expectations"][-1]["machine_observed"]
    assert any("parameters" in p for p in fixture.validate_definition(missing, "w9a-journey"))
    selfref = with_person()
    selfref["expectations"].append(dict(selfref["expectations"][-1], id="second", machine_check="copy-by-person"))
    assert any("names no earlier expectation" in p for p in fixture.validate_definition(selfref, "w9a-journey"))
    assert any("judges the person" in p for p in problems(on="window"))


# ---------------------------------------------------------------------------
# Judge and validator rules that had no test of their own.

def test_the_judge_names_a_driver_error_and_a_missing_receipt():
    def errored(report):
        report["error"] = "timed out after 60000 ms waiting for agent ready"
    assert judged(errored)[0] == "driver: timed out after 60000 ms waiting for agent ready"
    def no_receipt(report):
        report["batch"]["apply_message"] = None
    assert "no batch receipt after Apply" in judged(no_receipt)


def test_the_judge_refuses_a_window_value_in_the_wrong_unit():
    def metres(report):
        row = next(r for r in report["results"]["rows"] if r["id"].endswith(":ux"))
        row["text"] = row["text"].replace(" mm", " m")
    assert "node:c ux: unit m" in judged(metres)


def test_the_judge_requires_the_solve_on_the_applied_model_and_the_window_naming_it():
    def other_solve(captured):
        captured[1]["outcome"]["data"]["evaluatedModelHash"] = {"value": "d" * 64}
    found = judged(captured_mutate=other_solve)
    assert "solve was not on the applied model's hash" in found
    assert "the window's solve proof names another model hash" in found
    def other_proof(report):
        report["results"]["model_sha256"] = "sha256:" + "e" * 64
    assert judged(other_proof) == ["the window's solve proof names another model hash"]


def test_the_validator_refuses_an_id_other_than_the_directory_and_tree_types_off_the_observed_ones():
    assert any("is not the directory name" in p for p in fixture.validate_definition(w9a_definition(), "another-case"))
    off = w9a_definition()
    off["observe"]["tree_types"] = ["node", "pipe"]
    assert any("object_types" in p for p in fixture.validate_definition(off, "w9a-journey"))


def test_each_runner_mode_is_recorded_and_a_scripted_pass_reads_as_the_route_only():
    assert fixture.RUNNER_MODE == {"stub": "agent with a scripted provider", "live": "agent with a model"}
    assert set(fixture.RUNNER_MODE) == set(fixture.PASS_WORDS) == set(fixture.STANDING)
    stub = {"passed": True, "mode": "stub", "expectations": {}}
    assert fixture.settle(stub) == "route examined under a scripted provider" and stub["runnerMode"] == "agent with a scripted provider"
    live = {"passed": True, "mode": "live", "expectations": {}}
    assert fixture.settle(live) == "passed" and live["runnerMode"] == "agent with a model"
    # Nothing a scripted run writes says the use case was passed or completed.
    for word in (fixture.PASS_WORDS["stub"], fixture.STANDING["stub"], fixture.RUNNER_MODE["stub"]):
        assert not re.search(r"\b(passed|complete|completed)\b", word)
