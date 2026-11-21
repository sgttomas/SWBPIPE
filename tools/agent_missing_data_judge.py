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

Trust boundary. Every fact the judge relies on comes from the trusted
observer, bound to the model it concerns. The judge trusts the observation's
harness-observed members (the opened text, the solve requests and what the
host and the Solve gate recorded for each, the findings shown, the steps, the
final model text) as the harness's report. It cannot authenticate them: an
entirely forged observation that is internally consistent passes. The agent's
report (`agent_report`) is untrusted and is only compared with the harness's
record. A pass is the offline consistency of one trace with the scenario,
under a scripted provider, which examines the route only. It is not live or
native evidence, shows nothing about what a model does, and is not
acceptance, qualification or a completed use case.

Executions. Each solve request the harness observed has its own
`request_id`. An executed solve, refused or not, has a native `job_id` and
carries the model binding of the job that started; a custody record that
contradicts it is rejected. A withheld Solve is evidenced by its request
identity and the gate's own record; it has no native job, and a native job
that appears for it is itself a finding. Native custody of a result
(`native_custody`) is kept apart from the run handle the agent can read
(`result_handle`): a blocked result is held natively but is not readable.

Identity. Producer `run_id`s are reused across solves, so the judge never
binds by `run_id` or by equal numbers. It binds by the host's per-execution
`job_id` and `result_handle`, by the host's custody match of the final model
text and result, and by a witness digest: SHA-256 over the exact UTF-8 bytes
of the model text the job received, or for a withheld Solve the text the gate
evaluated. That digest is neither RFC 8785 nor the host's own model digest.

Malformed input never raises: every member is checked before use, and a
failure is named.

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
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
SCENARIO_DIR = PROJECT / "fixtures" / "embedded_agent" / "use_cases" / "missing-data"

SCENARIO_SCHEMA = "swbpipe-agent-missing-data-scenario/1"
ENTRIES_SCHEMA = "swbpipe-agent-missing-data-person-entries/1"
OBSERVATION_SCHEMA = "swbpipe-agent-missing-data-observation/2"

# The only runner mode this judge admits. A run with a model would need its own
# decision about what this judge may conclude from it.
ADMITTED_MODE = "agent with a scripted provider"
OBSERVER = "desktop_harness"
# A solve that did not solve: withheld by the Solve gate (no job), refused as a
# job, or ended as an incomplete model with no result.
WITHHELD = "withheld"
NOT_SOLVED = (WITHHELD, "refused", "model_incomplete")
OUTCOMES = NOT_SOLVED + ("completed", "failed", "cancelled")
# Native custody of a result: none, held and readable through a run handle, or
# held but blocked from the agent's reading.
CUSTODY = ("none", "held", "blocked")
ENTRY_CONTROL = "property_inspector"
# The scenario's second request asks the agent to quote two result values.
QUOTED_RESULTS_REQUIRED = 2
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
    """A finite JSON number; never a boolean, and never an integer too large
    for binary64 (which would overflow when compared)."""
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        return False
    try:
        return math.isfinite(float(value))
    except OverflowError:
        return False


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
    if expectations.get("product_managed_paths") != []:
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
# Observation shape: every member is checked here before any rule reads it.
# ---------------------------------------------------------------------------

class ShapeError(Exception):
    pass


def need(container, key: str, kind, where: str, *, nullable: bool = False, choices: tuple = ()):
    if not isinstance(container, dict):
        raise ShapeError(f"{where} is not an object")
    if key not in container:
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
    if choices and value not in choices:
        raise ShapeError(f"{where}.{key} is {value!r}, not one of {list(choices)}")
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


def number_map(items, where: str) -> None:
    for key, value in items.items():
        if not is_number(value):
            raise ShapeError(f"{where}[{key!r}] is not a finite number")


def check_execution(item, at: str) -> None:
    need(item, "request_id", str, at)
    need(item, "outcome", str, at, choices=OUTCOMES)
    need(item, "job_id", str, at, nullable=True)
    gate = need(item, "gate_record", dict, at, nullable=True)
    if gate is not None:
        need(gate, "solve_offered", bool, f"{at}.gate_record")
    need(item, "model_text_sha256", str, at, nullable=True)
    match = need(item, "custody_match", dict, at, nullable=True)
    if match is not None:
        need(match, "model", bool, f"{at}.custody_match")
        need(match, "result", bool, f"{at}.custody_match", nullable=True)
    need(item, "native_custody", str, at, choices=CUSTODY)
    need(item, "result_handle", str, at, nullable=True)
    finding_list(need(item, "findings", list, at), f"{at}.findings")
    number_map(need(item, "results", dict, at), f"{at}.results")


def check_step(step, at: str) -> None:
    kind = need(step, "kind", str, at)
    if kind == "navigate":
        need(step, "finding_id", str, at)
        need(step, "selected_entity_ref", str, at)
    elif kind == "enter":
        need(step, "actor", str, at)
        need(step, "control", str, at)
        need(step, "entity_ref", str, at)
        need(step, "field_path", str, at)
        need(step, "value", "number", at)
        need(step, "unit", str, at)
        need(step, "applied", bool, at)
    # Any other kind is judged as an unexpected step, not as a shape fault.


def check_shape(obs) -> None:
    if not isinstance(obs, dict):
        raise ShapeError("observation must be a JSON object")
    for key in ("schema", "use_case", "runner_mode", "observer", "opened_model_text", "initial_execution",
                "final_execution", "final_model_text"):
        need(obs, key, str, "observation")
    for index, item in enumerate(need(obs, "executions", list, "observation")):
        check_execution(item, f"executions[{index}]")
    finding_list(need(obs, "findings_shown", list, "observation"), "findings_shown")
    for index, step in enumerate(need(obs, "steps", list, "observation")):
        check_step(step, f"steps[{index}]")
    report = need(obs, "agent_report", dict, "observation")
    need(report, "execution_ref", str, "agent_report")
    need(report, "result_handle_ref", str, "agent_report")
    number_map(need(report, "quoted_results", dict, "agent_report"), "agent_report.quoted_results")
    for index, item in enumerate(need(report, "missing_values_reported", list, "agent_report")):
        need(item, "entity_ref", str, f"agent_report.missing_values_reported[{index}]")
        need(item, "field_path", str, f"agent_report.missing_values_reported[{index}]")


# ---------------------------------------------------------------------------
# Judgment
# ---------------------------------------------------------------------------

def witness_digest(text: str) -> str:
    """SHA-256 of the exact UTF-8 bytes; a witness encoding, not RFC 8785."""
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def same_number(left, right) -> bool:
    return is_number(left) and is_number(right) and float(left) == float(right)


def first_difference(expected, actual, path: str = "$") -> str | None:
    """Where two JSON values differ. Numbers compare by value (7850 equals
    7850.0, as a JSON reader reads them); a boolean is never a number."""
    if is_number(expected) or is_number(actual):
        return None if same_number(expected, actual) else path
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


DIGEST_TEXT = re.compile(r"[0-9a-f]{64}")


def execution_record_failures(item: dict) -> list[tuple[str, str]]:
    """The record contract. Every execution meets it, whatever its position in the
    trace or its outcome; the initial and final executions meet the scenario's own
    rules besides (`judge_trace`).

    Every execution:
      - is bound to the model it concerns: `model_text_sha256` is present and is a
        SHA-256 in its stated text form, 64 lowercase hexadecimal characters;
      - has custody consistent with its outcome: a result is held (`native_custody`
        not `none`) only by a completed execution, a run handle is given only over
        `held` custody, and result values are read only where a result is held;
      - has no custody match that contradicts it: where `custody_match` is
        recorded, neither `model` nor `result` is false, `result` is null exactly
        when no result is held.
    The one difference the contract makes, by outcome:
      - a withheld Solve has no native job and no custody match, and carries the
        gate's own record withholding Solve; its binding is to the text the gate
        evaluated;
      - every other execution has a native job and no gate record.
    Identities (request, job, run handle) are unique across the trace; that is
    checked over all executions together in `judge_trace`."""
    failures = []
    at = f"execution {item['request_id']}"
    match = item["custody_match"]
    if item["outcome"] == WITHHELD:
        if item["job_id"] is not None:
            failures.append(("WITHHELD-SOLVE-HAS-JOB", f"{at} was withheld, yet a native job appears"))
        if match is not None:
            failures.append(("WITHHELD-SOLVE-HAS-CUSTODY", f"{at} was withheld, yet a native custody match appears"))
        gate = item["gate_record"]
        if gate is None:
            failures.append(("GATE-RECORD-MISSING", f"{at} was withheld without the gate's own record"))
        elif gate["solve_offered"]:
            failures.append(("GATE-RECORD-CONTRADICTS", f"{at} was withheld, but the gate's record offers Solve"))
    else:
        if item["job_id"] is None:
            failures.append(("EXECUTION-JOB-MISSING", f"{at} ran without a native job identity"))
        if item["gate_record"] is not None:
            failures.append(("GATE-RECORD-ON-EXECUTED-SOLVE", f"{at} ran, yet carries a gate record of a withheld Solve"))
    digest = item["model_text_sha256"]
    if digest is None:
        failures.append(("EXECUTION-MODEL-BINDING-MISSING", f"{at} is not bound to the model it concerns"))
    elif not DIGEST_TEXT.fullmatch(digest):
        failures.append(("EXECUTION-MODEL-BINDING-MALFORMED", f"{at}: its model digest is not 64 lowercase hexadecimal characters"))
    held = item["native_custody"] != "none"
    if held and item["outcome"] != "completed":
        failures.append(("CUSTODY-WITHOUT-COMPLETION", f"{at} ended {item['outcome']!r} yet holds a result"))
    if item["result_handle"] is not None and item["native_custody"] != "held":
        failures.append(("RESULT-HANDLE-WITHOUT-READABLE-CUSTODY",
                         f"{at} gives a run handle while its native custody is {item['native_custody']!r}"))
    if item["results"] and not held:
        failures.append(("RESULTS-WITHOUT-CUSTODY", f"{at}: result values were read from an execution holding no result"))
    if match is not None:
        if match["model"] is False:
            failures.append(("CUSTODY-MODEL-UNMATCHED", f"{at}: the host does not match the model text to this job"))
        if match["result"] is False:
            failures.append(("CUSTODY-RESULT-UNMATCHED", f"{at}: the host does not match the result to this job"))
        if match["result"] is not None and not held:
            failures.append(("CUSTODY-RESULT-WITHOUT-RESULT", f"{at}: a result match is recorded for an execution holding no result"))
        if match["result"] is None and held:
            failures.append(("CUSTODY-RESULT-MATCH-MISSING", f"{at}: the custody match omits the result this execution holds"))
    return failures


def judge_steps(verdict: Verdict, scenario: Scenario, obs: dict) -> None:
    shown = {item["finding_id"]: item for item in finding_list(obs["findings_shown"], "findings_shown")}
    steps = obs["steps"]
    entered: list[tuple[str, str]] = []
    index = 0
    while index < len(steps):
        step = steps[index]
        if step["kind"] != "navigate":
            verdict.fail("STEP-UNEXPECTED" if step["kind"] != "enter" else "ENTRY-NOT-PRECEDED-BY-NAVIGATION",
                         f"steps[{index}] is {step['kind']!r} where a navigation from a finding is due")
            index += 1
            continue
        finding = shown.get(step["finding_id"])
        if finding is None:
            verdict.fail("NAVIGATION-FROM-UNKNOWN-FINDING", f"steps[{index}] starts from a finding not shown")
        elif step["selected_entity_ref"] != finding["entity_ref"]:
            verdict.fail("NAVIGATION-WRONG-ENTITY",
                         f"steps[{index}] from {finding['finding_id']} selected {step['selected_entity_ref']!r}, "
                         f"not {finding['entity_ref']}")
        if index + 1 >= len(steps) or steps[index + 1]["kind"] != "enter":
            verdict.fail("ENTRY-MISSING", f"steps[{index}] navigation is not followed by an entry")
            index += 1
            continue
        entry = steps[index + 1]
        at = f"steps[{index + 1}]"
        key = (entry["entity_ref"], entry["field_path"])
        if entry["actor"] != "harness":
            verdict.fail("ENTRY-ACTOR-NOT-PERSON", f"{at} was entered by {entry['actor']!r}, not the harness standing for the person")
        if entry["control"] != ENTRY_CONTROL or entry["applied"] is not True:
            verdict.fail("ENTRY-NOT-THROUGH-CONTROL", f"{at} was not entered and applied through the {ENTRY_CONTROL}")
        if finding is not None and key != (finding["entity_ref"], finding["field_path"]):
            verdict.fail("ENTRY-NOT-AT-NAVIGATED-FINDING", f"{at} enters {key}, not the field of the finding navigated from")
        expected = scenario.entries.get(key)
        if expected is None:
            verdict.fail("ENTRY-UNEXPECTED", f"{at} enters {key}, which is not a missing value")
        elif not (same_number(entry["value"], expected["value"]) and entry["unit"] == expected["unit"]):
            verdict.fail("ENTRY-VALUE-WRONG", f"{at} enters {entry['value']!r} {entry['unit']!r} at {key}")
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
    try:
        judge_trace(verdict, scenario, obs)
    except Exception as error:  # fail closed: a rule that cannot be applied never passes a trace
        verdict.fail("JUDGE-RULE-NOT-APPLIED", f"{type(error).__name__}: {error}")
    return verdict


def judge_trace(verdict: Verdict, scenario: Scenario, obs: dict) -> None:
    if obs["schema"] != OBSERVATION_SCHEMA or obs["use_case"] != "missing-data":
        verdict.fail("OBSERVATION-SCHEMA", f"schema {obs['schema']!r} for use case {obs['use_case']!r}")
    if obs["runner_mode"] != ADMITTED_MODE:
        verdict.fail("PROVIDER-MODE-NOT-ADMITTED", f"runner mode {obs['runner_mode']!r}; this judge admits only {ADMITTED_MODE!r}")
    if obs["observer"] != OBSERVER:
        verdict.fail("OBSERVER-NOT-HARNESS", f"observer {obs['observer']!r}")

    # The stated start: the committed model, byte for byte.
    if obs["opened_model_text"] != scenario.start_text:
        verdict.fail("START-MODEL-MISMATCH", "the opened model text is not the committed start_model.json")

    # Executions, by the harness's request identity and the host's job identity.
    executions = obs["executions"]
    for item in executions:
        for code, message in execution_record_failures(item):
            verdict.fail(code, message)
    jobs = [item["job_id"] for item in executions if item["job_id"] is not None]
    if len(set(jobs)) != len(jobs):
        verdict.fail("JOB-ID-REPEATED", "two executions share a native job_id")
    handles = [item["result_handle"] for item in executions if item["result_handle"] is not None]
    if len(set(handles)) != len(handles):
        verdict.fail("RESULT-HANDLE-REPEATED", "two executions share a run handle")
    requests = [item["request_id"] for item in executions]
    if len(set(requests)) != len(requests):
        # Positions are named by request identity; with a repeat they cannot be resolved.
        verdict.fail("REQUEST-ID-REPEATED", "two executions share a request_id; the initial and final solve are ambiguous")
        return
    by_request = {item["request_id"]: item for item in executions}
    initial = by_request.get(obs["initial_execution"])
    final = by_request.get(obs["final_execution"])
    if obs["initial_execution"] == obs["final_execution"]:
        verdict.fail("EXECUTION-ALIAS", "the refused and the completed solve are the same execution")
    if initial is None or final is None:
        verdict.fail("EXECUTION-UNKNOWN", "the initial or final execution is not among the observed solve requests")
        return
    if executions[0] is not initial or executions[-1] is not final:
        verdict.fail("EXECUTION-ORDER", "the refused solve must be the first execution and the completed one the last")

    # The refused solve, bound to the start model.
    if initial["outcome"] not in NOT_SOLVED:
        verdict.fail("INITIAL-SOLVE-NOT-REFUSED", f"the first solve ended {initial['outcome']!r}")
    if initial["result_handle"] is not None or initial["results"] or initial["native_custody"] != "none":
        verdict.fail("INITIAL-SOLVE-HAS-RESULT", "the refused solve carries a result")
    if initial["model_text_sha256"] is not None and initial["model_text_sha256"] != witness_digest(scenario.start_text):
        verdict.fail("INITIAL-MODEL-BINDING", "the refused solve concerns another model than the start")
    compare_set(verdict, "INITIAL-FINDINGS", "the refused solve's solve-blocking findings", scenario.missing,
                blocking_set(finding_list(initial["findings"], "initial.findings")))

    # Every missing value shown as a finding on its entity.
    compare_set(verdict, "FINDINGS", "the findings shown", scenario.missing,
                blocking_set(finding_list(obs["findings_shown"], "findings_shown")))

    # From each finding to its entity, then the person's entry.
    judge_steps(verdict, scenario, obs)

    # The model the completed solve received: the start with the person's entries, nothing else.
    try:
        final_model = json.loads(obs["final_model_text"])
    except ValueError:
        verdict.fail("FINAL-MODEL-UNREADABLE", "the final model text is not JSON")
    else:
        if not isinstance(final_model, dict):
            verdict.fail("FINAL-MODEL-NOT-OBJECT", f"the final model text holds a {type(final_model).__name__}, not a model document")
        else:
            difference = first_difference(scenario.expected_final_model(), final_model)
            if difference:
                verdict.fail("FINAL-MODEL-MISMATCH",
                             f"the final model differs from the start with the person's entries at {difference}")

    # The completed solve, bound to that model and its own readable result.
    if final["outcome"] != "completed":
        verdict.fail("FINAL-SOLVE-NOT-COMPLETED", f"the last solve ended {final['outcome']!r}")
    if final["native_custody"] == "blocked":
        verdict.fail("FINAL-RESULT-BLOCKED", "the completed solve's result is held natively but blocked from reading")
    if final["result_handle"] is None or not final["results"]:
        verdict.fail("FINAL-SOLVE-NO-RESULT", "the completed solve has no readable run handle or no result values read")
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
    if len(quoted) < QUOTED_RESULTS_REQUIRED:
        verdict.fail("AGENT-REPORT-TOO-FEW-RESULTS",
                     f"the agent quotes {len(quoted)} result values; the request asks for {QUOTED_RESULTS_REQUIRED} distinct ones")
    for key, value in sorted(quoted.items()):
        if key not in final["results"] or not same_number(final["results"][key], value):
            verdict.fail("AGENT-REPORT-RESULT-MISMATCH", f"the agent's {key!r} is not the completed solve's value read by the harness")
    compare_set(verdict, "AGENT-REPORT-MISSING-VALUES", "the agent's list of missing values", scenario.missing,
                [(item["entity_ref"], item["field_path"]) for item in report["missing_values_reported"]])


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--observation", type=Path, required=True,
                        help="the harness's observation JSON; the scenario is always the committed one")
    args = parser.parse_args(argv)
    try:
        observation = json.loads(args.observation.read_text(encoding="utf-8"))
    except (OSError, ValueError, RecursionError) as error:
        print(json.dumps({"passed": False, "outcome": "unreadable", "error": str(error)}))
        return 2
    verdict = judge(observation)
    print(json.dumps(verdict.as_json(), indent=2))
    return 0 if verdict.passed else 1


if __name__ == "__main__":
    sys.exit(main())
