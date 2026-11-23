"""Offline checks of the missing-data scenario and its judge.

Every trace here is SYNTHETIC judge-test input built in memory. None is an
observed run, and nothing here shows that a desktop or an agent carried the
use case out.
"""
from __future__ import annotations

import copy
import json
import os
import re
import subprocess
import sys
from pathlib import Path

import pytest

TOOLS = Path(__file__).resolve().parent
sys.path.insert(0, str(TOOLS))
import agent_missing_data_judge as judge_module  # noqa: E402
from agent_missing_data_judge import judge, load_scenario, witness_digest  # noqa: E402

PROJECT = TOOLS.parent
SCENARIO_DIR = PROJECT / "fixtures" / "embedded_agent" / "use_cases" / "missing-data"
SCENARIO = load_scenario()
INBOARD = ("pipe:md-inboard", "section.material_density")
OUTBOARD = ("pipe:md-outboard", "section.material_density")
RESULTS = {"result:disp:node-md-3:uy": 0.00123, "result:reaction:support-md-anchor:fy": -2210.5}


def final_text(model: dict | None = None) -> str:
    return json.dumps(model if model is not None else SCENARIO.expected_final_model(), indent=2)


def finding(fid: str, key: tuple[str, str]) -> dict:
    return {"finding_id": fid, "entity_ref": key[0], "field_path": key[1], "blocks_solve": True}


def enter(key: tuple[str, str], value: float, unit: str = "kg/m^3", actor: str = "harness") -> dict:
    return {"kind": "enter", "actor": actor, "control": "property_inspector", "entity_ref": key[0],
            "field_path": key[1], "value": value, "unit": unit, "applied": True}


def synthetic_trace() -> dict:
    """SYNTHETIC: a consistent trace, written by hand for the judge's tests."""
    text = final_text()
    findings = [finding("finding:inboard-density", INBOARD), finding("finding:outboard-density", OUTBOARD)]
    return {
        "schema": "swbpipe-agent-missing-data-observation/2",
        "use_case": "missing-data",
        "runner_mode": "agent with a scripted provider",
        "observer": "desktop_harness",
        "opened_model_text": SCENARIO.start_text,
        "executions": [
            # The Solve gate withholds the first solve on the start model: no native job.
            {"request_id": "request:synthetic-1", "job_id": None, "gate_record": {"solve_offered": False},
             "outcome": "withheld", "model_text_sha256": witness_digest(SCENARIO.start_text),
             "custody_match": None, "native_custody": "none", "result_handle": None,
             "findings": copy.deepcopy(findings), "results": {}},
            {"request_id": "request:synthetic-2", "job_id": "job:synthetic-2", "gate_record": None,
             "outcome": "completed", "model_text_sha256": witness_digest(text),
             "custody_match": {"model": True, "result": True}, "native_custody": "held",
             "result_handle": "handle:synthetic-2", "findings": [], "results": dict(RESULTS)},
        ],
        "initial_execution": "request:synthetic-1",
        "final_execution": "request:synthetic-2",
        "findings_shown": findings + [{"finding_id": "finding:rule-inputs", "entity_ref": "project:missing-data-use-case",
                                       "field_path": "rule_pack_refs", "blocks_solve": False}],
        "steps": [
            {"kind": "navigate", "finding_id": "finding:inboard-density", "selected_entity_ref": INBOARD[0]},
            enter(INBOARD, 7850),
            {"kind": "navigate", "finding_id": "finding:outboard-density", "selected_entity_ref": OUTBOARD[0]},
            enter(OUTBOARD, 8000),
        ],
        "final_model_text": text,
        "agent_report": {"execution_ref": "job:synthetic-2", "result_handle_ref": "handle:synthetic-2",
                         "quoted_results": dict(RESULTS),
                         "missing_values_reported": [{"entity_ref": k[0], "field_path": k[1]} for k in (INBOARD, OUTBOARD)]},
    }


def codes(observation) -> list[str]:
    return [code for code, _ in judge(observation).failures]


def test_the_synthetic_consistent_trace_is_route_only_and_claims_nothing_more():
    verdict = judge(synthetic_trace())
    assert verdict.passed, verdict.failures
    reported = verdict.as_json()
    assert reported["outcome"] == "trace consistent offline; route only under a scripted provider"
    assert "not authenticated" in reported["standing"] and "not acceptance" in reported["standing"]
    for text in (reported["outcome"], reported["standing"], judge_module.__doc__):
        assert not re.search(r"\b(accepted|qualified|certif|complian)", text.replace("not acceptance", ""), re.I)


# --- The committed scenario --------------------------------------------------

def test_the_start_model_lacks_exactly_the_missing_values_and_is_pressure_free():
    start = json.loads(SCENARIO.start_text)
    assert SCENARIO.missing == (INBOARD, OUTBOARD)
    for pipe in start["pipe_segments"]:
        assert "material_density" not in pipe["section"]
    assert "pressure_contract" not in start and "model_settings" not in start
    for case in start["load_cases"]:
        assert not [key for key in case if key.startswith("pressure")]  # no pressure member of any kind
        assert all(load["category"] != "pressure" and load["dimension"] != "pressure" for load in case["primitive_loads"])
    expected = SCENARIO.expected_final_model()
    assert [p["section"]["material_density"] for p in expected["pipe_segments"]] == [
        {"value": 7850, "unit": "kg/m^3"}, {"value": 8000, "unit": "kg/m^3"}]
    # Nothing but the two entries differs from the start.
    for pipe in expected["pipe_segments"]:
        del pipe["section"]["material_density"]
    assert expected == start


def test_person_entered_values_never_reach_the_provider_facing_text():
    scenario = json.loads((SCENARIO_DIR / "scenario.json").read_text())
    provider_facing = " ".join(scenario["requests"])
    for entry in SCENARIO.entries.values():
        assert str(entry["value"]) not in provider_facing and entry["unit"] not in provider_facing
    assert "density" not in provider_facing and "kg" not in provider_facing
    # The values live only in the harness's file, which the scenario does not inline.
    assert "7850" not in (SCENARIO_DIR / "scenario.json").read_text()
    assert "8000" not in (SCENARIO_DIR / "scenario.json").read_text()


def test_the_scenario_refuses_undocumented_product_managed_changes(tmp_path):
    for name in ("scenario.json", "person_entered.json", "start_model.json"):
        (tmp_path / name).write_text((SCENARIO_DIR / name).read_text())
    scenario = json.loads((tmp_path / "scenario.json").read_text())
    scenario["expectations"]["product_managed_paths"] = ["$.analysis_status"]
    (tmp_path / "scenario.json").write_text(json.dumps(scenario))
    with pytest.raises(judge_module.ScenarioError):
        load_scenario(tmp_path)


def test_scenario_texts_make_no_completion_or_acceptance_claim():
    for path in SCENARIO_DIR.iterdir():
        text = path.read_text()
        assert not re.search(r"(agent|use case) (has )?(completed|carried out|passed) (the|a|this) use case", text, re.I)
        assert not re.search(r"\b(certif|complian|qualif|approv)", text, re.I), path.name


# --- Negative controls --------------------------------------------------------

def mutated(change) -> dict:
    trace = synthetic_trace()
    change(trace)
    return trace


def test_results_of_another_solve_with_the_same_numbers_are_refused():
    def other_solve(trace):
        twin = copy.deepcopy(trace["executions"][1])
        twin.update(request_id="request:synthetic-twin", job_id="job:synthetic-twin", result_handle="handle:synthetic-twin")
        trace["executions"].insert(1, twin)
        trace["agent_report"].update(execution_ref="job:synthetic-twin", result_handle_ref="handle:synthetic-twin")
    assert set(codes(mutated(other_solve))) == {"AGENT-REPORT-WRONG-EXECUTION", "AGENT-REPORT-WRONG-HANDLE"}

    def run_id_only(trace):  # a reused producer run id binds nothing
        trace["executions"][1]["run_id"] = "run:reused"
        trace["agent_report"].update(execution_ref="run:reused")
    assert "AGENT-REPORT-WRONG-EXECUTION" in codes(mutated(run_id_only))


def test_quoted_numbers_that_are_not_the_completed_solves_are_refused():
    def wrong(trace):
        trace["agent_report"]["quoted_results"]["result:disp:node-md-3:uy"] = 0.002
    assert codes(mutated(wrong)) == ["AGENT-REPORT-RESULT-MISMATCH"]
    assert codes(mutated(lambda t: t["agent_report"].update(quoted_results={}))) == ["AGENT-REPORT-TOO-FEW-RESULTS"]
    for kept in RESULTS:  # deleting either quoted value leaves one, and the request asks for two
        assert codes(mutated(lambda t: t["agent_report"].update(quoted_results={kept: RESULTS[kept]}))) == [
            "AGENT-REPORT-TOO-FEW-RESULTS"]


def test_wrong_repaired_values_are_refused_at_the_entry_and_in_the_final_model():
    def swapped(trace):
        trace["steps"][1]["value"], trace["steps"][3]["value"] = 8000, 7850
        model = SCENARIO.expected_final_model()
        model["pipe_segments"][0]["section"]["material_density"]["value"] = 8000
        model["pipe_segments"][1]["section"]["material_density"]["value"] = 7850
        trace["final_model_text"] = final_text(model)
        trace["executions"][1]["model_text_sha256"] = witness_digest(trace["final_model_text"])
    assert set(codes(mutated(swapped))) == {"ENTRY-VALUE-WRONG", "FINAL-MODEL-MISMATCH"}
    assert codes(mutated(lambda t: t["steps"][3].update(unit="g/cm^3"))) == ["ENTRY-VALUE-WRONG"]

    def extra_change(trace):  # any other change to the model
        model = SCENARIO.expected_final_model()
        model["nodes"][2]["position"]["x"] = 2.5
        trace["final_model_text"] = final_text(model)
        trace["executions"][1]["model_text_sha256"] = witness_digest(trace["final_model_text"])
    assert codes(mutated(extra_change)) == ["FINAL-MODEL-MISMATCH"]


def test_a_number_written_as_an_integer_or_a_float_is_the_same_entry_but_a_boolean_is_not():
    def as_float(trace):
        model = SCENARIO.expected_final_model()
        model["pipe_segments"][1]["section"]["material_density"]["value"] = 8000.0
        trace["final_model_text"] = final_text(model)
        trace["executions"][1]["model_text_sha256"] = witness_digest(trace["final_model_text"])
    assert codes(mutated(as_float)) == []
    assert judge_module.first_difference({"v": 1}, {"v": True}) == "$.v"


def test_an_incomplete_or_failed_final_solve_is_refused():
    for outcome in ("failed", "cancelled", "model_incomplete"):
        assert "FINAL-SOLVE-NOT-COMPLETED" in codes(mutated(lambda t: t["executions"][1].update(outcome=outcome)))
    assert "FINAL-SOLVE-NO-RESULT" in codes(mutated(lambda t: t["executions"][1].update(result_handle=None)))
    assert "FINAL-SOLVE-BLOCKED" in codes(mutated(lambda t: t["executions"][1]["findings"].append(finding("f", INBOARD))))
    assert codes(mutated(lambda t: t["executions"][1].update(custody_match={"model": True, "result": False}))) == [
        "CUSTODY-RESULT-UNMATCHED", "FINAL-CUSTODY-UNMATCHED"]


def test_a_missing_or_an_extra_finding_is_refused():
    assert codes(mutated(lambda t: t["findings_shown"].pop(1))) == ["FINDINGS-MISSING", "NAVIGATION-FROM-UNKNOWN-FINDING"]
    extra = finding("finding:od", ("pipe:md-inboard", "section.outside_diameter"))
    assert codes(mutated(lambda t: t["findings_shown"].append(extra))) == ["FINDINGS-EXTRA"]
    assert codes(mutated(lambda t: t["executions"][0]["findings"].pop())) == ["INITIAL-FINDINGS-MISSING"]
    # A missing value shown, but not as solve-blocking, is not shown as what it is.
    assert "FINDINGS-MISSING" in codes(mutated(lambda t: t["findings_shown"][0].update(blocks_solve=False)))


def test_wrong_or_absent_navigation_is_refused():
    assert codes(mutated(lambda t: t["steps"][0].update(selected_entity_ref="pipe:md-outboard"))) == ["NAVIGATION-WRONG-ENTITY"]
    assert codes(mutated(lambda t: t["steps"].pop(2))) == ["ENTRY-NOT-PRECEDED-BY-NAVIGATION", "ENTRY-MISSING"]
    def crossed(trace):  # navigated to one finding, entered at the other
        trace["steps"][0]["finding_id"] = "finding:outboard-density"
        trace["steps"][0]["selected_entity_ref"] = OUTBOARD[0]
    assert "ENTRY-NOT-AT-NAVIGATED-FINDING" in codes(mutated(crossed))


def test_omitted_agent_authored_or_auto_filled_entries_are_refused():
    assert codes(mutated(lambda t: del_steps(t, 2, 4))) == ["ENTRY-MISSING"]
    assert codes(mutated(lambda t: t["steps"][3].update(actor="agent"))) == ["ENTRY-ACTOR-NOT-PERSON"]
    assert codes(mutated(lambda t: t["steps"][3].update(applied=False))) == ["ENTRY-NOT-THROUGH-CONTROL"]
    def agent_operation(trace):  # the agent proposes the value through the operation route
        del trace["steps"][2:4]
        trace["steps"].append({"kind": "model_operation", "actor": "agent", "entity_ref": OUTBOARD[0]})
    assert codes(mutated(agent_operation)) == ["STEP-UNEXPECTED", "ENTRY-MISSING"]


def del_steps(trace, start, end):
    del trace["steps"][start:end]


def test_missing_forged_or_aliased_identities_are_refused():
    assert codes(mutated(lambda t: t["executions"][1].pop("job_id")))[0] == "OBSERVATION-SHAPE"
    assert codes(mutated(lambda t: t["executions"][1].pop("request_id")))[0] == "OBSERVATION-SHAPE"
    assert "EXECUTION-JOB-MISSING" in codes(mutated(lambda t: t["executions"][1].update(job_id=None)))
    assert "WITHHELD-SOLVE-HAS-JOB" in codes(mutated(lambda t: t["executions"][0].update(job_id="job:synthetic-1")))
    assert "EXECUTION-UNKNOWN" in codes(mutated(lambda t: t.update(final_execution="job:forged")))
    def alias(trace):
        trace["final_execution"] = trace["initial_execution"]
    assert "EXECUTION-ALIAS" in codes(mutated(alias))
    def repeated_job(trace):
        trace.update(three_execution_trace("withheld", "refused"))
        trace["executions"][1]["job_id"] = trace["executions"][2]["job_id"]
    assert codes(mutated(repeated_job)) == ["JOB-ID-REPEATED"]
    assert codes(mutated(lambda t: t["executions"][1].update(request_id="request:synthetic-1"))) == ["REQUEST-ID-REPEATED"]
    def repeated_handle(trace):
        trace["executions"].insert(1, copy.deepcopy(trace["executions"][1]) | {"request_id": "request:1b", "job_id": "job:synthetic-1b"})
    assert "RESULT-HANDLE-REPEATED" in codes(mutated(repeated_handle))
    def reversed_order(trace):
        trace["executions"].reverse()
    assert "EXECUTION-ORDER" in codes(mutated(reversed_order))


def test_a_stale_model_binding_is_refused():
    def stale(trace):  # the completed solve received the start model, not the repaired one
        trace["executions"][1]["model_text_sha256"] = witness_digest(SCENARIO.start_text)
    assert codes(mutated(stale)) == ["FINAL-MODEL-BINDING-STALE"]
    assert codes(mutated(lambda t: t["executions"][0].update(model_text_sha256="0" * 64))) == ["INITIAL-MODEL-BINDING"]


def executed_initial(outcome: str):
    """SYNTHETIC: the first solve ran as a native job on the start model and ended `outcome`,
    an otherwise consistent executed record."""
    def change(trace):
        trace["executions"][0].update(outcome=outcome, job_id="job:synthetic-1", gate_record=None,
                                      custody_match={"model": True, "result": None})
    return change


def test_a_solved_or_resulted_first_solve_and_a_wrong_start_are_refused():
    assert "INITIAL-SOLVE-NOT-WITHHELD" in codes(mutated(lambda t: t["executions"][0].update(outcome="completed")))
    # Values read from a solve that holds no result also contradict its custody record.
    assert codes(mutated(lambda t: t["executions"][0].update(results={"r": 1.0}))) == [
        "RESULTS-WITHOUT-CUSTODY", "INITIAL-SOLVE-HAS-RESULT"]
    # The first solve is withheld before any solve: an executed one, refused or incomplete, is not.
    for outcome in ("refused", "model_incomplete"):
        assert codes(mutated(executed_initial(outcome))) == ["INITIAL-SOLVE-NOT-WITHHELD"], outcome
    assert codes(mutated(withheld)) == []
    assert codes(mutated(lambda t: t.update(opened_model_text=final_text()))) == ["START-MODEL-MISMATCH"]
    assert codes(mutated(lambda t: t.update(opened_model_text=SCENARIO.start_text.rstrip()))) == ["START-MODEL-MISMATCH"]


def test_an_unadmitted_provider_mode_or_observer_is_refused():
    for mode in ("agent with a model", "harness alone", ""):
        assert codes(mutated(lambda t: t.update(runner_mode=mode)))[0] in {"PROVIDER-MODE-NOT-ADMITTED", "OBSERVATION-SHAPE"}
    assert codes(mutated(lambda t: t.update(runner_mode="agent with a model"))) == ["PROVIDER-MODE-NOT-ADMITTED"]
    assert codes(mutated(lambda t: t.update(observer="agent"))) == ["OBSERVER-NOT-HARNESS"]


def test_the_agents_missing_value_report_is_compared_with_the_scenario():
    assert codes(mutated(lambda t: t["agent_report"]["missing_values_reported"].pop())) == [
        "AGENT-REPORT-MISSING-VALUES-MISSING"]


@pytest.mark.parametrize("path", [
    ("schema",), ("executions", 0, "results"), ("findings_shown", 0, "blocks_solve"), ("steps", 0, "kind"),
    ("agent_report", "quoted_results"), ("final_model_text",),
])
def test_invalid_shapes_fail_by_name(path):
    trace = synthetic_trace()
    target = trace
    for part in path[:-1]:
        target = target[part]
    del target[path[-1]]
    verdict = judge(trace)
    assert [code for code, _ in verdict.failures] == ["OBSERVATION-SHAPE"]
    assert str(path[-1]) in verdict.failures[0][1]
    assert codes([]) == ["OBSERVATION-SHAPE"]
    assert codes(mutated(lambda t: t["executions"][1]["results"].update(bad=True))) == ["OBSERVATION-SHAPE"]


def test_the_cli_reads_only_the_observation_and_never_a_scenario_path(tmp_path):
    observation = tmp_path / "observation.json"
    observation.write_text(json.dumps(synthetic_trace()))
    script = TOOLS / "agent_missing_data_judge.py"
    done = subprocess.run([sys.executable, str(script), "--observation", str(observation)], capture_output=True, text=True)
    assert done.returncode == 0, done.stderr
    assert json.loads(done.stdout)["passed"] is True
    refused = subprocess.run([sys.executable, str(script), "--observation", str(observation), "--scenario", str(tmp_path)],
                             capture_output=True, text=True)
    assert refused.returncode == 2
    observation.write_text("{")
    assert subprocess.run([sys.executable, str(script), "--observation", str(observation)], capture_output=True).returncode == 2


def withheld(trace):
    """SYNTHETIC: the Solve gate withholds the first solve. There is no native job; the
    request identity and the gate's own record evidence it, bound to the start model."""
    trace["executions"][0].update(outcome="withheld", job_id=None, custody_match=None,
                                  gate_record={"solve_offered": False})


# --- Review round 1 (head 38a32772a): repaired findings, each with its control ---

def test_a_final_model_that_is_not_an_object_is_refused():
    for text in ("null", "[]", "7850", '"model"', "true"):
        def not_object(trace, text=text):
            trace["final_model_text"] = text
            trace["executions"][1]["model_text_sha256"] = witness_digest(text)  # bound consistently, still refused
        assert codes(mutated(not_object)) == ["FINAL-MODEL-NOT-OBJECT"], text


def test_every_executed_solve_carries_its_jobs_model_binding():
    assert codes(mutated(lambda t: t["executions"][0].update(model_text_sha256=None))) == ["EXECUTION-MODEL-BINDING-MISSING"]
    for outcome in ("refused", "model_incomplete"):
        def unbound(trace, outcome=outcome):
            trace.update(three_execution_trace("withheld", "refused"))
            trace["executions"][1].update(outcome=outcome, model_text_sha256=None)
        assert codes(mutated(unbound)) == ["EXECUTION-MODEL-BINDING-MISSING"], outcome
    assert "EXECUTION-MODEL-BINDING-MISSING" in codes(mutated(lambda t: t["executions"][1].update(model_text_sha256=None)))
    # A withheld Solve is bound too: to the text the gate evaluated.
    def withheld_unbound(trace):
        withheld(trace)
        trace["executions"][0]["model_text_sha256"] = None
    assert codes(mutated(withheld_unbound)) == ["EXECUTION-MODEL-BINDING-MISSING"]


def test_contradictory_custody_is_refused():
    def refused_job(change):
        """A refused job between the withheld and the completed solve, changed by `change`."""
        def apply(trace):
            trace.update(three_execution_trace("withheld", "refused"))
            change(trace["executions"][1])
        return apply
    # The host refuses to match the refused job's model text: the binding is contradicted.
    assert codes(mutated(refused_job(lambda r: r.update(custody_match={"model": False, "result": None})))) == [
        "CUSTODY-MODEL-UNMATCHED"]
    # A result match on a job that holds no result.
    assert codes(mutated(refused_job(lambda r: r.update(custody_match={"model": True, "result": True})))) == [
        "CUSTODY-RESULT-WITHOUT-RESULT"]
    # Results or custody on a solve that did not complete.
    # Its custody match (model only) now also omits the result the record claims to hold.
    assert set(codes(mutated(refused_job(lambda r: r.update(native_custody="held"))))) == {
        "CUSTODY-WITHOUT-COMPLETION", "CUSTODY-RESULT-MATCH-MISSING"}
    # Held custody on the withheld first solve is a result it cannot carry.
    assert set(codes(mutated(lambda t: t["executions"][0].update(native_custody="held")))) == {
        "CUSTODY-WITHOUT-COMPLETION", "INITIAL-SOLVE-HAS-RESULT"}
    # No custody record at all on the refused job is admitted: the binding is its digest.
    assert codes(mutated(refused_job(lambda r: r.update(custody_match=None)))) == []


def test_a_withheld_solve_needs_no_job_and_a_job_for_it_is_a_finding():
    assert codes(mutated(withheld)) == []
    def with_job(trace):
        withheld(trace)
        trace["executions"][0]["job_id"] = "job:appeared"
    assert codes(mutated(with_job)) == ["WITHHELD-SOLVE-HAS-JOB"]
    def with_custody(trace):
        withheld(trace)
        trace["executions"][0]["custody_match"] = {"model": True, "result": None}
    assert codes(mutated(with_custody)) == ["WITHHELD-SOLVE-HAS-CUSTODY"]
    def without_gate(trace):
        withheld(trace)
        trace["executions"][0]["gate_record"] = None
    assert codes(mutated(without_gate)) == ["GATE-RECORD-MISSING"]
    def gate_offers(trace):
        withheld(trace)
        trace["executions"][0]["gate_record"] = {"solve_offered": True}
    assert codes(mutated(gate_offers)) == ["GATE-RECORD-CONTRADICTS"]
    def wrong_model(trace):
        withheld(trace)
        trace["executions"][0]["model_text_sha256"] = witness_digest(final_text())
    assert codes(mutated(wrong_model)) == ["INITIAL-MODEL-BINDING"]
    def executed_with_gate(trace):
        executed_initial("refused")(trace)
        trace["executions"][0]["gate_record"] = {"solve_offered": False}
    assert codes(mutated(executed_with_gate)) == ["GATE-RECORD-ON-EXECUTED-SOLVE", "INITIAL-SOLVE-NOT-WITHHELD"]


def test_blocked_native_custody_is_not_a_readable_run_handle():
    # Held natively but blocked from the agent: no run handle, and the use case does not complete.
    def blocked(trace):
        trace["executions"][1].update(native_custody="blocked", result_handle=None)
    assert set(codes(mutated(blocked))) == {"FINAL-RESULT-BLOCKED", "FINAL-SOLVE-NO-RESULT", "AGENT-REPORT-WRONG-HANDLE"}
    # A run handle offered over blocked or absent custody contradicts the custody record.
    assert "RESULT-HANDLE-WITHOUT-READABLE-CUSTODY" in codes(mutated(lambda t: t["executions"][1].update(native_custody="blocked")))
    assert "RESULT-HANDLE-WITHOUT-READABLE-CUSTODY" in codes(mutated(lambda t: t["executions"][1].update(native_custody="none")))


@pytest.mark.parametrize("change, where", [
    (lambda t: t["agent_report"]["quoted_results"].update({"result:huge": 10 ** 400}), "quoted_results"),
    (lambda t: t["executions"][1]["results"].update({"result:huge": 10 ** 400}), "results"),
    (lambda t: t["steps"][1].update(value=10 ** 400), "value"),
    (lambda t: t["steps"][1].update(value=float("nan")), "value"),
    (lambda t: t["steps"][1].update(value="7850"), "value"),
    (lambda t: t["steps"][0].update(finding_id=[]), "finding_id"),
    (lambda t: t["steps"][0].update(selected_entity_ref={"id": "pipe:md-inboard"}), "selected_entity_ref"),
    (lambda t: t["steps"][0].pop("finding_id"), "finding_id"),
    (lambda t: t["steps"][1].update(entity_ref=["pipe:md-inboard"]), "entity_ref"),
    (lambda t: t["steps"][1].update(field_path=None), "field_path"),
    (lambda t: t["steps"][1].update(actor=1), "actor"),
    (lambda t: t["steps"][1].update(control=None), "control"),
    (lambda t: t["steps"][1].update(unit=[]), "unit"),
    (lambda t: t["steps"][1].update(applied="yes"), "applied"),
    (lambda t: t["steps"].__setitem__(0, "navigate"), "steps[0]"),
    (lambda t: t["executions"][0].update(outcome="paused"), "outcome"),
    (lambda t: t["executions"][1].update(native_custody="kept"), "native_custody"),
    (lambda t: t["executions"][0].update(gate_record={"solve_offered": "no"}), "solve_offered"),
    (lambda t: t["executions"][1].update(custody_match={"model": True, "result": "yes"}), "result"),
])
def test_malformed_members_fail_by_name_and_never_raise(change, where):
    verdict = judge(mutated(change))
    assert [code for code, _ in verdict.failures] == ["OBSERVATION-SHAPE"], verdict.failures
    assert where in verdict.failures[0][1]


def test_a_final_model_that_breaks_a_rule_fails_closed_and_never_raises():
    def deep(trace):
        trace["final_model_text"] = "[" * 100000 + "]" * 100000
        trace["executions"][1]["model_text_sha256"] = witness_digest(trace["final_model_text"])
    failures = codes(mutated(deep))
    assert failures and set(failures) <= {"FINAL-MODEL-UNREADABLE", "FINAL-MODEL-NOT-OBJECT", "JUDGE-RULE-NOT-APPLIED"}
    def huge_number(trace):
        model = json.loads(final_text())
        model["pipe_segments"][0]["section"]["material_density"]["value"] = 10 ** 400
        trace["final_model_text"] = json.dumps(model)
        trace["executions"][1]["model_text_sha256"] = witness_digest(trace["final_model_text"])
    assert codes(mutated(huge_number)) == ["FINAL-MODEL-MISMATCH"]


# --- Record-contract sweep (review round 2, head bb0e733cf) ---------------------
# The record contract binds every execution, whatever its position. For each
# property and each slot (position and baseline record kind), a SYNTHETIC record
# violating exactly that property must be refused for its named reason. Where the
# scenario's own initial or final rules necessarily fire as well, they are listed
# as consequences; combinations the contract makes impossible are named in
# INAPPLICABLE instead of being counted.

SLOTS = {
    # slot: (index in a three-execution trace, initial kind, intermediate kind)
    # The first solve is withheld (an executed one is INITIAL-SOLVE-NOT-WITHHELD,
    # `test_a_solved_or_resulted_first_solve_and_a_wrong_start_are_refused`).
    "initial/withheld": (0, "withheld", "completed"),
    "intermediate/completed": (1, "withheld", "completed"),
    "intermediate/refused": (1, "withheld", "refused"),
    "intermediate/withheld": (1, "withheld", "withheld"),
    "final/completed": (2, "withheld", "completed"),
}


def three_execution_trace(initial: str, intermediate: str) -> dict:
    """SYNTHETIC: a valid trace with one intermediate solve between the refused and the completed one."""
    assert initial == "withheld", "the first solve of a valid trace is withheld"
    trace = synthetic_trace()
    if intermediate == "completed":
        record = copy.deepcopy(trace["executions"][1])
        record.update(request_id="request:mid", job_id="job:mid", result_handle="handle:mid")
    else:
        record = copy.deepcopy(synthetic_trace()["executions"][0])
        record["request_id"] = "request:mid"
        if intermediate == "refused":
            record.update(outcome="refused", job_id="job:mid", gate_record=None, custody_match={"model": True, "result": None})
    trace["executions"].insert(1, record)
    return trace


def set_match(**values):
    def change(record):
        record["custody_match"] = {**(record["custody_match"] or {"model": True, "result": None}), **values}
    return change


def upper_digest(record):
    record["model_text_sha256"] = record["model_text_sha256"].upper()


R = "intermediate/refused"
W = "intermediate/withheld"
C = "intermediate/completed"
EXECUTED_SLOTS = (C, R, "final/completed")
WITHHELD_SLOTS = ("initial/withheld", W)
ALL_SLOTS = tuple(SLOTS)

# (property, reason, mutation, {slot: scenario consequences})
SWEEP = [
    ("binding present", "EXECUTION-MODEL-BINDING-MISSING", lambda r: r.update(model_text_sha256=None),
     {slot: () for slot in ALL_SLOTS} | {"final/completed": ("FINAL-MODEL-BINDING-STALE",)}),
    ("binding well-formed", "EXECUTION-MODEL-BINDING-MALFORMED", lambda r: r.update(model_text_sha256="not-a-digest"),
     {slot: () for slot in ALL_SLOTS} | {"initial/withheld": ("INITIAL-MODEL-BINDING",),
                                         "final/completed": ("FINAL-MODEL-BINDING-STALE",)}),
    ("binding in lowercase text form", "EXECUTION-MODEL-BINDING-MALFORMED", upper_digest,
     {slot: () for slot in ALL_SLOTS} | {"initial/withheld": ("INITIAL-MODEL-BINDING",),
                                         "final/completed": ("FINAL-MODEL-BINDING-STALE",)}),
    ("custody model match not false", "CUSTODY-MODEL-UNMATCHED", set_match(model=False),
     {C: (), R: (), "final/completed": ("FINAL-CUSTODY-UNMATCHED",)}),
    ("custody result match not false", "CUSTODY-RESULT-UNMATCHED", set_match(result=False),
     {C: (), "final/completed": ("FINAL-CUSTODY-UNMATCHED",),
      # Not isolable: a result verdict on a record holding no result also contradicts its custody.
      R: ("CUSTODY-RESULT-WITHOUT-RESULT",)}),
    ("custody result match only where a result is held", "CUSTODY-RESULT-WITHOUT-RESULT", set_match(result=True),
     {R: ()}),
    ("custody result match present where a result is held", "CUSTODY-RESULT-MATCH-MISSING", set_match(result=None),
     {C: (), "final/completed": ("FINAL-CUSTODY-UNMATCHED",)}),
    ("custody held only on completion", "CUSTODY-WITHOUT-COMPLETION",
     lambda r: r.update(native_custody="blocked", custody_match=None) if r["outcome"] != "completed" else r.update(outcome="failed"),
     {"initial/withheld": ("INITIAL-SOLVE-HAS-RESULT",),
      R: (), W: (), C: (), "final/completed": ("FINAL-SOLVE-NOT-COMPLETED",)}),
    ("run handle only over held custody", "RESULT-HANDLE-WITHOUT-READABLE-CUSTODY",
     lambda r: r.update(native_custody="blocked") if r["result_handle"] else r.update(result_handle="handle:stray"),
     {"initial/withheld": ("INITIAL-SOLVE-HAS-RESULT",),
      R: (), W: (), C: (), "final/completed": ("FINAL-RESULT-BLOCKED",)}),
    ("results read only where a result is held", "RESULTS-WITHOUT-CUSTODY", lambda r: r.update(results={"result:stray": 1.0}),
     {"initial/withheld": ("INITIAL-SOLVE-HAS-RESULT",), R: (), W: ()}),
    ("executed solve has a native job", "EXECUTION-JOB-MISSING", lambda r: r.update(job_id=None),
     {C: (), R: (), "final/completed": ("AGENT-REPORT-WRONG-EXECUTION",)}),
    ("executed solve has no gate record", "GATE-RECORD-ON-EXECUTED-SOLVE", lambda r: r.update(gate_record={"solve_offered": False}),
     {slot: () for slot in EXECUTED_SLOTS}),
    ("withheld Solve has no native job", "WITHHELD-SOLVE-HAS-JOB", lambda r: r.update(job_id="job:appeared"),
     {slot: () for slot in WITHHELD_SLOTS}),
    ("withheld Solve has no custody match", "WITHHELD-SOLVE-HAS-CUSTODY", set_match(model=True),
     {slot: () for slot in WITHHELD_SLOTS}),
    ("withheld Solve has the gate's record", "GATE-RECORD-MISSING", lambda r: r.update(gate_record=None),
     {slot: () for slot in WITHHELD_SLOTS}),
    ("the gate's record withholds Solve", "GATE-RECORD-CONTRADICTS", lambda r: r.update(gate_record={"solve_offered": True}),
     {slot: () for slot in WITHHELD_SLOTS}),
]

INAPPLICABLE = {
    # (property, slot): why no isolating record exists there
    **{(prop, slot): "a withheld Solve has no custody match; one appearing is WITHHELD-SOLVE-HAS-CUSTODY"
       for prop in ("custody model match not false", "custody result match not false",
                    "custody result match only where a result is held", "custody result match present where a result is held")
       for slot in WITHHELD_SLOTS},
    **{(prop, slot): "the baseline holds a result; removing it breaks other properties first"
       for prop in ("custody result match only where a result is held", "results read only where a result is held")
       for slot in (C, "final/completed")},
    **{("custody result match present where a result is held", slot): "the baseline holds no result"
       for slot in (R,)},
    **{(prop, slot): "the contract difference by outcome: this property belongs to the other kind of record"
       for prop in ("executed solve has a native job", "executed solve has no gate record") for slot in WITHHELD_SLOTS},
    **{(prop, slot): "the contract difference by outcome: this property belongs to the other kind of record"
       for prop in ("withheld Solve has no native job", "withheld Solve has no custody match",
                    "withheld Solve has the gate's record", "the gate's record withholds Solve") for slot in EXECUTED_SLOTS},
}

READER_WITNESSES = {("custody result match not false", C): "reader-witness-1",
                    ("binding well-formed", C): "reader-witness-2"}

SWEEP_CASES = [pytest.param(prop, reason, change, slot, consequences,
                            id=f"{READER_WITNESSES.get((prop, slot), prop)}@{slot}")
               for prop, reason, change, slots in SWEEP for slot, consequences in slots.items()]


@pytest.mark.parametrize("slot", ALL_SLOTS)
def test_every_sweep_baseline_is_consistent(slot):
    _, initial, intermediate = SLOTS[slot]
    assert codes(three_execution_trace(initial, intermediate)) == []


@pytest.mark.parametrize("prop, reason, change, slot, consequences", SWEEP_CASES)
def test_the_record_contract_binds_every_position(prop, reason, change, slot, consequences):
    index, initial, intermediate = SLOTS[slot]
    trace = three_execution_trace(initial, intermediate)
    change(trace["executions"][index])
    assert sorted(codes(trace)) == sorted((reason, *consequences)), (prop, slot)


def test_the_sweep_covers_every_property_at_every_slot_or_names_why_not():
    for prop, _, _, slots in SWEEP:
        for slot in ALL_SLOTS:
            assert (slot in slots) != ((prop, slot) in INAPPLICABLE), (prop, slot)
    assert all(slot in SLOTS for _, slot in INAPPLICABLE)
    assert {name for name in READER_WITNESSES.values()} == {"reader-witness-1", "reader-witness-2"}


@pytest.mark.parametrize("pair", [(0, 1), (1, 2), (0, 2)])
def test_identities_are_unique_across_every_pair_of_positions(pair):
    earlier, later = pair
    trace = three_execution_trace("withheld", "completed")
    trace["executions"][earlier]["request_id"] = trace["executions"][later]["request_id"]
    assert codes(trace) == ["REQUEST-ID-REPEATED"], pair
    # Only executed solves hold a native job; the withheld first solve holds none.
    if earlier == 0:
        return
    trace = three_execution_trace("withheld", "completed")
    trace["executions"][earlier]["job_id"] = trace["executions"][later]["job_id"]
    assert codes(trace) == ["JOB-ID-REPEATED"], pair


def test_run_handles_are_unique_where_two_positions_hold_one():
    # Only the intermediate and final records hold a result in a valid baseline; an initial
    # handle is RESULT-HANDLE-WITHOUT-READABLE-CUSTODY first (swept above).
    trace = three_execution_trace("withheld", "completed")
    trace["executions"][1]["result_handle"] = trace["executions"][2]["result_handle"]
    assert codes(trace) == ["RESULT-HANDLE-REPEATED"]


# --- Feasibility through the product's own solve entry ------------------------

def runner_binary() -> Path:
    supplied = os.environ.get("OPENPIPESTRESS_RUNNER_BIN")
    if supplied and Path(supplied).is_file():
        return Path(supplied)
    if os.environ.get("OPENPIPESTRESS_NO_CARGO_BUILD") == "1":
        pytest.skip("OPENPIPESTRESS_NO_CARGO_BUILD=1 and OPENPIPESTRESS_RUNNER_BIN is not supplied: "
                    "the scenario's solve feasibility was not examined")
    sys.path.insert(0, str(PROJECT / "tools" / "runner"))
    from build_runner import build
    return build()


def runner_solve(binary: Path, model: dict) -> dict:
    request = {
        "request_id": "missing-data-feasibility", "operation": "solve",
        "operation_ref": {"ref_type": "api_operation", "ref_id": "solve"},
        "project_ref": {"ref_type": "project", "ref_id": "missing-data-use-case"},
        "model_ref": {"ref_type": "model", "ref_id": "missing-data-start"},
        "unit_system_ref": {"ref_type": "unit_system", "ref_id": "invented-si"},
        "load_basis_refs": [{"ref_type": "load_case", "ref_id": "case:md-seismic"}],
        "input_manifest_ref": {"ref_type": "audit_manifest", "ref_id": "manifest-1"},
        "requested_outputs": ["result_envelope", "audit_manifest", "diagnostics"],
        "privacy": {"local_only": True, "telemetry_allowed": False, "private_payload_redacted": True,
                    "classification": "public_metadata"},
        "provenance": {"source_name": "invented missing-data scenario", "source_location": "fixtures/embedded_agent/use_cases/missing-data",
                       "source_license": "project invented", "contributor": "SWBPIPE",
                       "contributor_certification": "invented non-engineering example",
                       "redistribution_status": "invented_non_engineering_example", "review_status": "accepted"},
        "professional_boundary": {"human_review_required": True, "software_makes_compliance_claim": False,
                                  "software_makes_certification_claim": False, "software_makes_sealing_claim": False,
                                  "software_makes_approval_claim": False, "software_makes_authentication_claim": False},
        "tbd_decisions": {key: "SETTLED_DEC_065" for key in ("final_cli_command_syntax", "package_scripts", "process_invocation",
                                                              "network_access", "filesystem_mutation_policy")}
        | {key: "TBD" for key in ("ci_provider", "release_matrix", "public_transport_protocol", "external_adapter_formats",
                                  "physical_project_container")},
    }
    body = {"request": request, "solve": {"preview_model": {"model": model, "materials": []}}}
    done = subprocess.run([str(binary), "solve", "--explicit-local-private-intent"], input=json.dumps(body),
                          capture_output=True, text=True, check=False)
    return json.loads(done.stdout)["payload"]["mechanics_envelope"]


def test_the_start_is_refused_naming_each_missing_value_and_the_entered_model_solves():
    binary = runner_binary()
    start = runner_solve(binary, json.loads(SCENARIO.start_text))
    assert start["status"]["mechanics"] == "MODEL_INCOMPLETE" and not start["results"]
    named = set()
    for item in start["diagnostics"]:
        if item["severity"] == "blocking":
            refs = item.get("affected_refs") or []
            fields = [key for key in SCENARIO.missing if key[0] in refs and key[1].rsplit(".", 1)[-1] in refs]
            assert fields, item  # every blocking finding names a missing value on its entity
            named.update(fields)
    assert named == set(SCENARIO.missing)
    solved = runner_solve(binary, SCENARIO.expected_final_model())
    assert solved["status"]["mechanics"] == "MECHANICS_SOLVED" and solved["results"]
    assert not [item for item in solved["diagnostics"] if item["severity"] == "blocking"]
