#!/usr/bin/env python3
"""Offline judge of one trace of the missing-data use case (DEL-07-06 AC-007).

The use case: the person opens a model that lacks values a solve requires;
every missing value is shown as a finding on its entity; from each finding
the person reaches the entity and enters the value; and the solve, refused
before, then completes.

This module judges a trace of that use case offline. The trace is an
*observation*: what a trusted desktop harness saw while it ran the use case.
Everything the judge expects comes from the committed scenario in
`fixtures/embedded_agent/use_cases/missing-data/`, never from the
observation. No option takes the scenario from another path.

Trust boundary. The judge trusts the observation's harness-observed members
(the opened text, the host's execution records, the findings shown, the
steps, the final model text) as the harness's report. It cannot authenticate
them: an entirely forged observation that is internally consistent passes.
The agent's report (`agent_report`) is untrusted and is only compared with
the harness's record. A pass is the offline consistency of one trace with the
scenario, under a scripted provider, which examines the route only. It is not
live or native evidence, shows nothing about what a model does, and is not
acceptance, qualification or a completed use case.

Identity. Producer `run_id`s are reused across solves, so the judge never
binds by `run_id` or by equal numbers. It binds by the host's per-execution
`job_id` and `result_handle`, by the host's custody match of the final model
text and result, and by a witness digest: SHA-256 over the exact UTF-8 bytes
of the model text the job received. That digest is neither RFC 8785 nor the
host's own model digest.

Usage: python3 tools/agent_missing_data_judge.py --observation <file>
Prints the verdict as JSON. Exit 0 when the trace is consistent, 1 when it is
not, 2 when the observation cannot be read.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
import math
import sys
from dataclasses import dataclass, field
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
SCENARIO_DIR = PROJECT / "fixtures" / "embedded_agent" / "use_cases" / "missing-data"

SCENARIO_SCHEMA = "swbpipe-agent-missing-data-scenario/1"
ENTRIES_SCHEMA = "swbpipe-agent-missing-data-person-entries/1"
OBSERVATION_SCHEMA = "swbpipe-agent-missing-data-observation/1"

# The only runner mode this judge admits. A run with a model would need its own
# decision about what this judge may conclude from it.
ADMITTED_MODE = "agent with a scripted provider"
OBSERVER = "desktop_harness"
# A solve that did not solve: withheld by the Solve gate, refused as a job, or
# ended as an incomplete model with no result.
NOT_SOLVED = ("withheld", "refused", "model_incomplete")
OUTCOMES = NOT_SOLVED + ("completed", "failed", "cancelled")
ENTRY_CONTROL = "property_inspector"
PASS_OUTCOME = "trace consistent offline; route only under a scripted provider"
STANDING = ("offline consistency of one harness-reported trace with the committed scenario; "
            "the observer is not authenticated; not live or native evidence, not acceptance")


class ScenarioError(Exception):
    """The committed scenario itself is malformed."""


@dataclass
class Verdict:
    failures: list[tuple[str, str]] = field(default_factory=list)

    def fail(self, code: str, message: str) -> None:
        self.failures.append((code, message))

    @property
    def passed(self) -> bool:
        return not self.failures

    def as_json(self) -> dict:
        return {
            "use_case": "missing-data",
            "passed": self.passed,
            "outcome": PASS_OUTCOME if self.passed else "failed",
            "standing": STANDING,
            "failures": [{"code": code, "message": message} for code, message in self.failures],
        }


# ---------------------------------------------------------------------------
# The committed scenario
# ---------------------------------------------------------------------------

@dataclass(frozen=True)
class Scenario:
    start_text: str
    start_model: dict
    requests: tuple[str, ...]
    missing: tuple[tuple[str, str], ...]
    entries: dict  # (entity_ref, field_path) -> {"value": number, "unit": str}

    def expected_final_model(self) -> dict:
        """The start model with each person-entered value written at its field."""
        model = copy.deepcopy(self.start_model)
        for (entity_ref, field_path), entry in self.entries.items():
            record = entity(model, entity_ref)
            if record is None:
                raise ScenarioError(f"entry names {entity_ref}, which the start model lacks")
            *parents, leaf = field_path.split(".")
            for name in parents:
                record = record.setdefault(name, {})
            record[leaf] = {"value": entry["value"], "unit": entry["unit"]}
        return model


def entity(model: dict, ref: str) -> dict | None:
    for collection in ("pipe_segments", "materials", "nodes", "supports", "components", "load_cases"):
        for record in model.get(collection) or []:
            if isinstance(record, dict) and record.get("id") == ref:
                return record
    return None


def field_value(record: dict, field_path: str):
    value = record
    for name in field_path.split("."):
        if not isinstance(value, dict) or name not in value:
            return None
        value = value[name]
    return value


def is_number(value) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value)


def load_scenario(directory: Path = SCENARIO_DIR) -> Scenario:
    def read(name: str) -> dict:
        try:
            return json.loads((directory / name).read_text(encoding="utf-8"))
        except (OSError, ValueError) as error:
            raise ScenarioError(f"{name}: {error}") from error

    scenario = read("scenario.json")
    if scenario.get("schema") != SCENARIO_SCHEMA or scenario.get("id") != "missing-data":
        raise ScenarioError("scenario.json: wrong schema or id")
    start = scenario.get("start") or {}
    if start.get("kind") != "open_model_file" or start.get("model") != "start_model.json":
        raise ScenarioError("scenario.json: start must open start_model.json")
    start_text = (directory / "start_model.json").read_text(encoding="utf-8")
    start_model = json.loads(start_text)
    requests = scenario.get("requests")
    if not isinstance(requests, list) or not requests or not all(isinstance(r, str) and r.strip() for r in requests):
        raise ScenarioError("scenario.json: requests must be nonblank texts")
    expectations = scenario.get("expectations") or {}
    missing = []
    for item in expectations.get("missing_values") or []:
        key = (item.get("entity_ref"), item.get("field_path"))
        if not all(isinstance(part, str) and part for part in key) or key in missing:
            raise ScenarioError(f"scenario.json: bad or repeated missing value {item!r}")
        record = entity(start_model, key[0])
        if record is None or field_value(record, key[1]) is not None:
            raise ScenarioError(f"scenario.json: {key} is not a value the start model lacks")
        missing.append(key)
    if not missing:
        raise ScenarioError("scenario.json: no missing values")
    if expectations.get("entries_from") != "person_entered.json":
        raise ScenarioError("scenario.json: entries come from person_entered.json")
    managed = expectations.get("product_managed_paths")
    if managed != []:
        # A change the product makes on its own would be documented here with its
        # reason and given its own comparison rule; none is known, so none is admitted.
        raise ScenarioError("scenario.json: product_managed_paths must be empty until a documented change has a rule")
    person = read("person_entered.json")
    if person.get("schema") != ENTRIES_SCHEMA or person.get("use_case") != "missing-data":
        raise ScenarioError("person_entered.json: wrong schema or use case")
    entries = {}
    for item in person.get("entries") or []:
        key = (item.get("entity_ref"), item.get("field_path"))
        if key in entries or not is_number(item.get("value")) or not isinstance(item.get("unit"), str) or not item["unit"]:
            raise ScenarioError(f"person_entered.json: bad or repeated entry {item!r}")
        entries[key] = {"value": item["value"], "unit": item["unit"]}
    if set(entries) != set(missing):
        raise ScenarioError("person_entered.json: entries must be exactly the missing values")
    return Scenario(start_text, start_model, tuple(requests), tuple(missing), entries)


# ---------------------------------------------------------------------------
# Observation shape
# ---------------------------------------------------------------------------

class ShapeError(Exception):
    pass


def need(container: dict, key: str, kind, where: str, *, nullable: bool = False):
    if not isinstance(container, dict) or key not in container:
        raise ShapeError(f"{where}.{key} is missing")
    value = container[key]
    if value is None and nullable:
        return None
    if kind is str:
        ok = isinstance(value, str) and value.strip() != ""
    elif kind is bool:
        ok = isinstance(value, bool)
    elif kind == "number":
        ok = is_number(value)
    else:
        ok = isinstance(value, kind) and not isinstance(value, bool)
    if not ok:
        raise ShapeError(f"{where}.{key} has the wrong type")
    return value


def finding_list(items, where: str) -> list[dict]:
    if not isinstance(items, list):
        raise ShapeError(f"{where} must be a list")
    out = []
    for index, item in enumerate(items):
        at = f"{where}[{index}]"
        out.append({"finding_id": need(item, "finding_id", str, at), "entity_ref": need(item, "entity_ref", str, at),
                    "field_path": need(item, "field_path", str, at), "blocks_solve": need(item, "blocks_solve", bool, at)})
    return out


def check_shape(obs) -> None:
    if not isinstance(obs, dict):
        raise ShapeError("observation must be a JSON object")
    for key in ("schema", "use_case", "runner_mode", "observer", "opened_model_text", "initial_execution",
                "final_execution", "final_model_text"):
        need(obs, key, str, "observation")
    executions = need(obs, "executions", list, "observation")
    for index, item in enumerate(executions):
        at = f"executions[{index}]"
        need(item, "job_id", str, at)
        need(item, "result_handle", str, at, nullable=True)
        need(item, "outcome", str, at)
        need(item, "model_text_sha256", str, at, nullable=True)
        match = need(item, "custody_match", dict, at, nullable=True)
        if match is not None:
            need(match, "model", bool, f"{at}.custody_match")
            need(match, "result", bool, f"{at}.custody_match")
        finding_list(need(item, "findings", list, at), f"{at}.findings")
        results = need(item, "results", dict, at)
        for key, value in results.items():
            if not is_number(value):
                raise ShapeError(f"{at}.results[{key!r}] is not a finite number")
    finding_list(need(obs, "findings_shown", list, "observation"), "findings_shown")
    for index, step in enumerate(need(obs, "steps", list, "observation")):
        need(step, "kind", str, f"steps[{index}]")
    report = need(obs, "agent_report", dict, "observation")
    need(report, "execution_ref", str, "agent_report")
    need(report, "result_handle_ref", str, "agent_report")
    quoted = need(report, "quoted_results", dict, "agent_report")
    for key, value in quoted.items():
        if not is_number(value):
            raise ShapeError(f"agent_report.quoted_results[{key!r}] is not a finite number")
    for index, item in enumerate(need(report, "missing_values_reported", list, "agent_report")):
        need(item, "entity_ref", str, f"agent_report.missing_values_reported[{index}]")
        need(item, "field_path", str, f"agent_report.missing_values_reported[{index}]")


# ---------------------------------------------------------------------------
# Judgment
# ---------------------------------------------------------------------------

def witness_digest(text: str) -> str:
    """SHA-256 of the exact UTF-8 bytes; a witness encoding, not RFC 8785."""
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def first_difference(expected, actual, path: str = "$") -> str | None:
    """Where two JSON values differ. Numbers compare by value (7850 equals
    7850.0, as a JSON reader reads them); a boolean is never a number."""
    if is_number(expected) and is_number(actual):
        return None if float(expected) == float(actual) else path
    if type(expected) is not type(actual):
        return path
    if isinstance(expected, dict):
        for key in sorted(set(expected) | set(actual)):
            if key not in expected or key not in actual:
                return f"{path}.{key}"
            found = first_difference(expected[key], actual[key], f"{path}.{key}")
            if found:
                return found
        return None
    if isinstance(expected, list):
        if len(expected) != len(actual):
            return path
        for index, (left, right) in enumerate(zip(expected, actual)):
            found = first_difference(left, right, f"{path}[{index}]")
            if found:
                return found
        return None
    return None if expected == actual else path


def blocking_set(findings: list[dict]) -> list[tuple[str, str]]:
    return [(item["entity_ref"], item["field_path"]) for item in findings if item["blocks_solve"]]


def compare_set(verdict: Verdict, code: str, what: str, expected, observed) -> None:
    seen = list(observed)
    repeated = sorted({item for item in seen if seen.count(item) > 1})
    missing = sorted(set(expected) - set(seen))
    extra = sorted(set(seen) - set(expected))
    if missing:
        verdict.fail(f"{code}-MISSING", f"{what} lacks {missing}")
    if extra:
        verdict.fail(f"{code}-EXTRA", f"{what} has unexpected {extra}")
    if repeated:
        verdict.fail(f"{code}-REPEATED", f"{what} repeats {repeated}")


def judge_steps(verdict: Verdict, scenario: Scenario, obs: dict) -> None:
    shown = {item["finding_id"]: item for item in finding_list(obs["findings_shown"], "findings_shown")}
    steps = obs["steps"]
    entered: list[tuple[str, str]] = []
    index = 0
    while index < len(steps):
        step = steps[index]
        if step.get("kind") != "navigate":
            verdict.fail("STEP-UNEXPECTED" if step.get("kind") != "enter" else "ENTRY-NOT-PRECEDED-BY-NAVIGATION",
                         f"steps[{index}] is {step.get('kind')!r} where a navigation from a finding is due")
            index += 1
            continue
        finding = shown.get(step.get("finding_id"))
        if finding is None:
            verdict.fail("NAVIGATION-FROM-UNKNOWN-FINDING", f"steps[{index}] starts from a finding not shown")
        elif step.get("selected_entity_ref") != finding["entity_ref"]:
            verdict.fail("NAVIGATION-WRONG-ENTITY",
                         f"steps[{index}] from {finding['finding_id']} selected {step.get('selected_entity_ref')!r}, "
                         f"not {finding['entity_ref']}")
        if index + 1 >= len(steps) or steps[index + 1].get("kind") != "enter":
            verdict.fail("ENTRY-MISSING", f"steps[{index}] navigation is not followed by an entry")
            index += 1
            continue
        entry = steps[index + 1]
        at = f"steps[{index + 1}]"
        key = (entry.get("entity_ref"), entry.get("field_path"))
        if entry.get("actor") != "harness":
            verdict.fail("ENTRY-ACTOR-NOT-PERSON", f"{at} was entered by {entry.get('actor')!r}, not the harness standing for the person")
        if entry.get("control") != ENTRY_CONTROL or entry.get("applied") is not True:
            verdict.fail("ENTRY-NOT-THROUGH-CONTROL", f"{at} was not entered and applied through the {ENTRY_CONTROL}")
        if finding is not None and key != (finding["entity_ref"], finding["field_path"]):
            verdict.fail("ENTRY-NOT-AT-NAVIGATED-FINDING", f"{at} enters {key}, not the field of the finding navigated from")
        expected = scenario.entries.get(key)
        if expected is None:
            verdict.fail("ENTRY-UNEXPECTED", f"{at} enters {key}, which is not a missing value")
        elif not (is_number(entry.get("value")) and float(entry["value"]) == float(expected["value"])
                  and entry.get("unit") == expected["unit"]):
            verdict.fail("ENTRY-VALUE-WRONG", f"{at} enters {entry.get('value')!r} {entry.get('unit')!r} at {key}")
        entered.append(key)
        index += 2
    compare_set(verdict, "ENTRY", "the person's entries", scenario.missing, entered)


def judge(obs, scenario: Scenario | None = None) -> Verdict:
    scenario = scenario or load_scenario()
    verdict = Verdict()
    try:
        check_shape(obs)
    except ShapeError as error:
        verdict.fail("OBSERVATION-SHAPE", str(error))
        return verdict
    if obs["schema"] != OBSERVATION_SCHEMA or obs["use_case"] != "missing-data":
        verdict.fail("OBSERVATION-SCHEMA", f"schema {obs['schema']!r} for use case {obs['use_case']!r}")
    if obs["runner_mode"] != ADMITTED_MODE:
        verdict.fail("PROVIDER-MODE-NOT-ADMITTED", f"runner mode {obs['runner_mode']!r}; this judge admits only {ADMITTED_MODE!r}")
    if obs["observer"] != OBSERVER:
        verdict.fail("OBSERVER-NOT-HARNESS", f"observer {obs['observer']!r}")

    # The stated start: the committed model, byte for byte.
    if obs["opened_model_text"] != scenario.start_text:
        verdict.fail("START-MODEL-MISMATCH", "the opened model text is not the committed start_model.json")

    # Executions, by the host's per-execution identity.
    executions = obs["executions"]
    ids = [item["job_id"] for item in executions]
    if len(set(ids)) != len(ids):
        verdict.fail("EXECUTION-ID-REPEATED", "two executions share a job_id")
    handles = [item["result_handle"] for item in executions if item["result_handle"] is not None]
    if len(set(handles)) != len(handles):
        verdict.fail("RESULT-HANDLE-REPEATED", "two executions share a result handle")
    by_id = {item["job_id"]: item for item in executions}
    initial = by_id.get(obs["initial_execution"])
    final = by_id.get(obs["final_execution"])
    if obs["initial_execution"] == obs["final_execution"]:
        verdict.fail("EXECUTION-ALIAS", "the refused and the completed solve are the same execution")
    if initial is None or final is None:
        verdict.fail("EXECUTION-UNKNOWN", "the initial or final execution is not among the host's executions")
        return verdict
    if executions[0] is not initial or executions[-1] is not final:
        verdict.fail("EXECUTION-ORDER", "the refused solve must be the first execution and the completed one the last")

    # The refused solve.
    if initial["outcome"] not in NOT_SOLVED:
        verdict.fail("INITIAL-SOLVE-NOT-REFUSED", f"the first solve ended {initial['outcome']!r}")
    if initial["result_handle"] is not None or initial["results"]:
        verdict.fail("INITIAL-SOLVE-HAS-RESULT", "the refused solve carries a result")
    if initial["model_text_sha256"] is not None and initial["model_text_sha256"] != witness_digest(scenario.start_text):
        verdict.fail("INITIAL-MODEL-BINDING", "the refused solve did not receive the start model")
    compare_set(verdict, "INITIAL-FINDINGS", "the refused solve's solve-blocking findings", scenario.missing,
                blocking_set(finding_list(initial["findings"], "initial.findings")))

    # Every missing value shown as a finding on its entity.
    compare_set(verdict, "FINDINGS", "the findings shown", scenario.missing,
                blocking_set(finding_list(obs["findings_shown"], "findings_shown")))

    # From each finding to its entity, then the person's entry.
    judge_steps(verdict, scenario, obs)

    # The model the completed solve received.
    try:
        final_model = json.loads(obs["final_model_text"])
    except ValueError:
        verdict.fail("FINAL-MODEL-UNREADABLE", "the final model text is not JSON")
        final_model = None
    if final_model is not None:
        expected = scenario.expected_final_model()
        difference = first_difference(expected, final_model)
        if difference:
            verdict.fail("FINAL-MODEL-MISMATCH", f"the final model differs from the start with the person's entries at {difference}")

    # The completed solve, bound to that model and its own result.
    if final["outcome"] != "completed":
        verdict.fail("FINAL-SOLVE-NOT-COMPLETED", f"the last solve ended {final['outcome']!r}")
    if final["result_handle"] is None or not final["results"]:
        verdict.fail("FINAL-SOLVE-NO-RESULT", "the completed solve has no held result or no result values read")
    if blocking_set(finding_list(final["findings"], "final.findings")):
        verdict.fail("FINAL-SOLVE-BLOCKED", "the completed solve still carries solve-blocking findings")
    match = final["custody_match"]
    if not (match and match["model"] and match["result"]):
        verdict.fail("FINAL-CUSTODY-UNMATCHED", "the host did not match the final model text and result to that execution")
    if final["model_text_sha256"] != witness_digest(obs["final_model_text"]):
        verdict.fail("FINAL-MODEL-BINDING-STALE", "the completed solve's model digest is not that of the final model text")

    # The agent's report: untrusted, compared with the harness's record.
    report = obs["agent_report"]
    if report["execution_ref"] != final["job_id"]:
        verdict.fail("AGENT-REPORT-WRONG-EXECUTION", f"the agent cites {report['execution_ref']!r}, not the completed solve")
    if report["result_handle_ref"] != final["result_handle"]:
        verdict.fail("AGENT-REPORT-WRONG-HANDLE", "the agent cites another result handle")
    quoted = report["quoted_results"]
    if not quoted:
        verdict.fail("AGENT-REPORT-NO-RESULTS", "the agent quotes no result value")
    for key, value in sorted(quoted.items()):
        if key not in final["results"] or float(final["results"][key]) != float(value):
            verdict.fail("AGENT-REPORT-RESULT-MISMATCH", f"the agent's {key!r} is not the completed solve's value read by the harness")
    compare_set(verdict, "AGENT-REPORT-MISSING-VALUES", "the agent's list of missing values", scenario.missing,
                [(item["entity_ref"], item["field_path"]) for item in report["missing_values_reported"]])
    return verdict


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--observation", type=Path, required=True,
                        help="the harness's observation JSON; the scenario is always the committed one")
    args = parser.parse_args(argv)
    try:
        observation = json.loads(args.observation.read_text(encoding="utf-8"))
    except (OSError, ValueError) as error:
        print(json.dumps({"passed": False, "outcome": "unreadable", "error": str(error)}))
        return 2
    verdict = judge(observation)
    print(json.dumps(verdict.as_json(), indent=2))
    return 0 if verdict.passed else 1


if __name__ == "__main__":
    sys.exit(main())
