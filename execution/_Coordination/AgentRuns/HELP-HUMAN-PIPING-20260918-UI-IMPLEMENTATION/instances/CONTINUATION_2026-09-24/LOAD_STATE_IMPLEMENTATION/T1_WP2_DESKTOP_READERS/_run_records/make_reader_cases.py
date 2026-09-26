"""Targeted load-reference-1 reader cases for the T1 WP2 desktop mutation run.

Each case is a list of JSON-pointer ops on a committed load-reference-1 raw.
The script applies the ops, runs Python
core.analysis_runs.load_reference_evidence.validate_load_reference_evidence on
the result, and records the code Python raised. The desktop test
(apps/desktop/src/features/results/loadReferenceReaderCases.test.ts) applies
the same ops and must raise the same code. Two further probes exercise the
reader's guards on non-JSON values (no JSON input reaches them); they are
logged here and mirrored by hand in loadReferenceReaders.test.ts.

Run from projects/chirality-piping with PYTHONPATH=. :
    python <this file> <out cases.json>
All values are invented.
"""
from __future__ import annotations
import copy, json, sys
from pathlib import Path

from core.analysis_runs.load_reference_evidence import LoadReferenceError, validate_load_reference_evidence

C = "fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json"
P = "fixtures/product_preview/load_reference/pressure-sparse_interactive.raw.json"
F = "core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-sparse_interactive.raw.json"
REC = "/contract_evidence/load_reference_states/0"
EXACT = "/contract_evidence/exact_cases/0"


def load(path):
    return json.loads(Path(path).read_text())


def at(doc, pointer):
    for token in pointer[1:].split("/"):
        doc = doc[int(token)] if isinstance(doc, list) else doc[token]
    return doc


def with_(value, **changes):
    out = copy.deepcopy(value)
    out.update(changes)
    return out


def cases():
    c, p, f = load(C), load(P), load(F)
    member_contribution = next(x for x in at(c, REC + "/contributions") if x["owner_kind"] == "resolved_member_state")
    support_contribution = next(x for x in at(c, REC + "/contributions") if x["owner_kind"] == "support_state")
    yield "NUMERICAL_CASE_DUPLICATE", C, [{"op": "append", "path": "/numerical_quality/cases", "value": at(c, "/numerical_quality/cases/0")}]
    yield "MATERIAL_DUPLICATE", C, [{"op": "append", "path": EXACT + "/pipe_materials", "value": at(c, EXACT + "/pipe_materials/0")}]
    yield "SECTION_DUPLICATE", C, [{"op": "append", "path": EXACT + "/pipe_sections", "value": at(c, EXACT + "/pipe_sections/0")}]
    yield "MEMBER_MATERIAL_COVERAGE", C, [{"op": "append", "path": EXACT + "/pipe_materials", "value": with_(at(c, EXACT + "/pipe_materials/0"), pipe_id="pipe:invented")}]
    yield "MEMBER_CONTRIBUTION_COVERAGE", C, [{"op": "append", "path": REC + "/contributions", "value": with_(member_contribution, source_id="member_state:pipe:invented")}]
    yield "SUPPORT_CONTRIBUTION_COVERAGE", C, [{"op": "append", "path": REC + "/contributions", "value": with_(support_contribution, source_id="support_state:support:invented:UX")}]
    yield "POINT_TEMPERATURE_RANGE", C, [{"op": "set", "path": REC + "/members/0/consumed_material_points/0/temperature_k", "value": -1.0}]
    yield "MATERIAL_POINT_RANGE", C, [{"op": "set", "path": REC + "/members/0/consumed_material_points/0/E_pa", "value": -1.0}]
    yield "DIRECT_THERMAL_LAW_BINDING", F, [{"op": "set", "path": REC + "/members/0/expansion_law_id", "value": "law:invented"}]
    yield "REFERENCE_LENGTH", P, [{"op": "set", "path": REC + "/members/0/reference_length_m", "value": -1.0}]
    yield "CONNECTOR_NOT_ARRAY", C, [{"op": "set", "path": "/contract_evidence/connector", "value": {}}]
    yield "REGION_CASE_UNRESOLVED", P, [{"op": "append", "path": "/contract_evidence/pressure", "value": with_(at(p, "/contract_evidence/pressure/0"), load_case_id="case:invented", region_id="region:invented")}]
    # Reaches the member-not-found refusal; the desktop mutant that removes it is
    # behaviourally equivalent (the following binding check raises the same code).
    yield "REGION_MATERIAL_UNBOUND_PIPE", P, [{"op": "append", "path": "/contract_evidence/pressure/0/materials", "value": with_(at(p, "/contract_evidence/pressure/0/materials/0"), pipe_id="pipe:invented")}]
    yield "SUPPORT_COMPONENT_DOF_NAME", C, [{"op": "set", "path": REC + "/support_components/0/dof", "value": "invented"}, {"op": "set", "path": REC + "/support_components/0/global_dof", "value": -1}]
    del f


def apply(doc, ops):
    for op in ops:
        tokens = op["path"][1:].split("/")
        parent = doc
        for token in tokens[:-1]:
            parent = parent[int(token)] if isinstance(parent, list) else parent[token]
        last = tokens[-1]
        key = int(last) if isinstance(parent, list) else last
        if op["op"] == "set":
            parent[key] = copy.deepcopy(op["value"])
        elif op["op"] == "append":
            parent[key].append(copy.deepcopy(op["value"]))
        else:
            raise ValueError(op["op"])
    return doc


def outcome(doc):
    try:
        validate_load_reference_evidence(doc)
        return "accept"
    except LoadReferenceError as error:
        return str(error)


class Hostile(dict):
    """A mapping whose values() raises TypeError: exercises the reader's _guarded conversion."""
    def values(self):
        raise TypeError("invented hostile mapping")


def main():
    out = []
    for name, source, ops in cases():
        doc = apply(load(source), ops)
        out.append({"id": name, "source": source, "ops": ops, "python": outcome(doc)})
        print(f"{name:32s} {out[-1]['python']}")
    # Guards reached only by non-JSON values (mirrored in loadReferenceReaders.test.ts).
    doc = load(C)
    doc["diagnostics"][0]["invented"] = object()
    print(f"{'NON_JSON_VALUE (object())':32s} {outcome(doc)}")
    doc = load(C)
    doc["contract_evidence"] = Hostile(doc["contract_evidence"])
    print(f"{'HOSTILE_MAPPING (TypeError)':32s} {outcome(doc)}")
    Path(sys.argv[1]).write_text(json.dumps(out, indent=1) + "\n")


if __name__ == "__main__":
    main()
