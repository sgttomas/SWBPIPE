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
        "schema": "swbpipe-agent-missing-data-observation/1",
        "use_case": "missing-data",
        "runner_mode": "agent with a scripted provider",
        "observer": "desktop_harness",
        "opened_model_text": SCENARIO.start_text,
        "executions": [
            {"job_id": "job:synthetic-1", "result_handle": None, "outcome": "model_incomplete",
             "model_text_sha256": witness_digest(SCENARIO.start_text), "custody_match": None,
             "findings": copy.deepcopy(findings), "results": {}},
            {"job_id": "job:synthetic-2", "result_handle": "handle:synthetic-2", "outcome": "completed",
             "model_text_sha256": witness_digest(text), "custody_match": {"model": True, "result": True},
             "findings": [], "results": dict(RESULTS)},
        ],
        "initial_execution": "job:synthetic-1",
        "final_execution": "job:synthetic-2",
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
        twin.update(job_id="job:synthetic-twin", result_handle="handle:synthetic-twin")
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
    assert codes(mutated(lambda t: t["agent_report"].update(quoted_results={}))) == ["AGENT-REPORT-NO-RESULTS"]


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
        "FINAL-CUSTODY-UNMATCHED"]


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
    assert "EXECUTION-UNKNOWN" in codes(mutated(lambda t: t.update(final_execution="job:forged")))
    def alias(trace):
        trace["final_execution"] = trace["initial_execution"]
    assert "EXECUTION-ALIAS" in codes(mutated(alias))
    assert "EXECUTION-ID-REPEATED" in codes(mutated(lambda t: t["executions"][1].update(job_id="job:synthetic-1")))
    def repeated_handle(trace):
        trace["executions"].insert(1, copy.deepcopy(trace["executions"][1]) | {"job_id": "job:synthetic-1b"})
    assert "RESULT-HANDLE-REPEATED" in codes(mutated(repeated_handle))
    def reversed_order(trace):
        trace["executions"].reverse()
    assert "EXECUTION-ORDER" in codes(mutated(reversed_order))


def test_a_stale_model_binding_is_refused():
    def stale(trace):  # the completed solve received the start model, not the repaired one
        trace["executions"][1]["model_text_sha256"] = witness_digest(SCENARIO.start_text)
    assert codes(mutated(stale)) == ["FINAL-MODEL-BINDING-STALE"]
    assert codes(mutated(lambda t: t["executions"][0].update(model_text_sha256="0" * 64))) == ["INITIAL-MODEL-BINDING"]


def test_a_solved_or_resulted_first_solve_and_a_wrong_start_are_refused():
    assert "INITIAL-SOLVE-NOT-REFUSED" in codes(mutated(lambda t: t["executions"][0].update(outcome="completed")))
    assert codes(mutated(lambda t: t["executions"][0].update(results={"r": 1.0}))) == ["INITIAL-SOLVE-HAS-RESULT"]
    for outcome in ("withheld", "refused"):
        assert codes(mutated(lambda t: t["executions"][0].update(outcome=outcome))) == []
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
