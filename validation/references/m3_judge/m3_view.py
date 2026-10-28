"""The validator of the M3 judge: strictly parsed JSON in, the closed canonical evidence view (m3_types.py) out.

One validator per route view (refutation round 5; Agent 0: "one judge, built once"):
- pf: the pressure-free preview route (PF M3-T, #1663): one section family, `preview_cases` evidence, the
  derived rows judged as information;
- exact: the exact pressure route (K-D5 M3-T, kd5_timoshenko/): both D-5 section bases, `exact_cases` evidence.

A raw runner record becomes a RunView only when every field the judge reads is present, typed, bound to its
model and unique; otherwise it is refused with named problems and nothing of it is judged. The rules are in
RAW_SCHEMA below. Fields the judge does not read are type-checked where the public format fixes their shape and
are then dropped: they do not exist for judging. A reference line becomes a ModelView and an Expected the same way
(reference_views).

Units are fixed per kind and component by the route's public semantic contract,
fixtures/results/semantic_contract_v0_3_{preview_physics_2,pressure_2}.json (`rows`: kind, component, unit and
declared source basis); any other unit, kind or component is refused. Frames (metadata.coordinate_system) per
judged kind, the five metadata members and the entity classes are those of the committed result fixtures of
both routes (fixtures/results/*_mechanics_*.json, preview_physics_*.json) and of the genuine PF probe records.
"""

from __future__ import annotations

import collections
import hashlib
import json
import math
import os
import re
import subprocess
from decimal import Decimal
from fractions import Fraction as Q

import m3_json
from m3_types import (SECTION_BASES, Expected, ExpectedCase, Extremum, Headline, ModelView, PipeSection, Row, RunView, SectionProps,
                      Standing, TauInputs)

HERE = os.path.dirname(os.path.abspath(__file__))
FIXTURES = os.path.normpath(os.path.join(HERE, "..", "..", "..", "fixtures", "results"))
# The semantic contracts each route publishes under, in the order a record is matched against them. A record is admitted
# under the first contract whose table every one of its rows matches; rows are never matched across two contracts.
# The exact route publishes pressure-2 and, for retained-source maxima, pressure-source-1, which alone declares
# `retained_source_endpoint_normal_max_v1` (results.v0.3.schema.yaml: "published only under pressure-source-1").
CONTRACTS = {"pf": (("openpipestress.result_semantics/0.3.0/preview-physics-2", "semantic_contract_v0_3_preview_physics_2.json"),),
             "exact": (("openpipestress.result_semantics/0.3.0/pressure-2", "semantic_contract_v0_3_pressure_2.json"),
                       ("openpipestress.result_semantics/0.3.0/pressure-source-1", "semantic_contract_v0_3_pressure_source_1.json"))}
CONTRACT_FILES = {"pf": CONTRACTS["pf"][0][1], "exact": CONTRACTS["exact"][0][1], "exact:pressure-source-1": CONTRACTS["exact"][1][1]}
CONTRACT_IDS = {"pf": CONTRACTS["pf"][0][0], "exact": CONTRACTS["exact"][0][0]}
SCHEMAS = os.path.normpath(os.path.join(HERE, "..", "..", "..", "schemas"))
# The exact route's contract_evidence: its members (required and admitted) and their shapes are read from this schema
# definition at judge time; only exact_cases' extrema enter the view, the other members are shape-checked or not read.
EXACT_EVIDENCE_SCHEMA = ("results_pressure_2.v0.3.schema.json", "/$defs/PressureContractEvidence")
# Under pressure-source-1 the whole contract_evidence is the recovery evidence of this schema: ordinary, or resolved when
# it carries load_reference_states. It is validated against that definition at judge time; only its exact_cases'
# extrema enter the view (pipe_id, result_id, section_basis, value_lower_pa, value_upper_pa: the members kd5's prior
# judge read), and value_pa, endpoints, the error bounds and the other members are schema-checked and not read.
SOURCE_RECOVERY_SCHEMA = "physics_source_recovery.schema.json"
SOURCE_RECOVERY_EVIDENCE = {False: "/$defs/pressure_source_ordinary_evidence", True: "/$defs/pressure_source_resolved_evidence"}
SCHEMA_FILES = ("results_pressure_2.v0.3.schema.json", "load_reference_state.schema.json", SOURCE_RECOVERY_SCHEMA)

ENTRIES = ("typed", "captured")
MODES = ("SparseInteractive", "DenseScrutiny")
HEX64 = re.compile(r"\A[0-9a-f]{64}\Z")
HEX40 = re.compile(r"\A[0-9a-f]{40}\Z")
SOLVER_ENTITY = "solver:linear_static_preview"
MAXIMUM = "pipe_elastic_normal_stress_maximum_v2"
SECTION_MARKER = "; section_basis="

# The closed sets of outcomes (refutation round 4). Standings: DEL-07-05 numerical-health-indicator.md section 1;
# only CHECKS_PASSED is Passed. Refusals: the codes the route's design names. PF: pf-timoshenko.md sections 1-3.
# Exact: DEL-05-03 section-and-shear-decisions.md and section-bases-and-timoshenko.md, DEL-04-07 pressure-contract.md
# and DEL-04-01 kd5-member-relative-bound.md (the two codes U6 and U8 removed are not in the set).
STANDINGS = frozenset({"NUMERICAL_INTEGRITY_CHECKS_PASSED", "NUMERICAL_INTEGRITY_SENSITIVE", "NUMERICAL_INTEGRITY_UNRESOLVED",
                       "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED", "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED",
                       "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM", "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY", "NUMERICAL_INTEGRITY_FAILED"})
PASSED = "NUMERICAL_INTEGRITY_CHECKS_PASSED"
REFUSALS = {
    "pf": frozenset({"PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE", "PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED", "MODEL-SETTING-SHEAR-MODEL-UNKNOWN",
                     "MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT", "MODEL-SETTING-SHEAR-MODEL-TIMOSHENKO-UNSUPPORTED"}),
    "exact": frozenset({"EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED", "EXACT_PRESSURE_SHEAR_MODEL_REQUIRED", "EXACT_PRESSURE_FAMILY_NOT_ADMITTED",
                        "EXACT_PRESSURE_COMPOSITION_UNSUPPORTED", "EXACT_PRESSURE_COMBINATION_UNSUPPORTED", "EXACT_PRESSURE_REDUCER_IN_REGION"}),
}
PF_REFUSAL_CODE = "PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE"
EULER, TIMOSHENKO = "euler_bernoulli", "timoshenko_annulus_energy_v1"

# Kinds the judge reads, by route. Every other kind of the route's contract is kept as an unjudged row.
NODAL = ("global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z",
         "global_nodal_rotation_x", "global_nodal_rotation_y", "global_nodal_rotation_z")
LOCAL_ACTIONS = ("element_local_axial_force", "element_local_shear_force_y", "element_local_shear_force_z", "element_local_torsional_moment",
                 "element_local_bending_moment_y", "element_local_bending_moment_z")
LOCAL_STRESSES = ("element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z",
                  "element_local_torsional_shear_stress")
SUPPORT = ("support_reaction_component_v2", "support_reaction_force_magnitude_v2", "support_reaction_moment_magnitude_v2")
WALL = ("pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2", "pipe_wall_endpoint_action_v2")
WALL_STRESSES = ("pipe_axial_membrane_stress_v2", "pipe_lame_radial_stress_v2", "pipe_lame_hoop_stress_v2")
DERIVED = ("element_global_end_force_v1", "element_global_end_moment_v1", "element_local_normal_stress_extreme",
           "element_local_transverse_shear_stress_maximum")
SOLVER = "linear_solver_mode_basis"
JUDGED = {"pf": frozenset(NODAL + ("displacement_magnitude",) + LOCAL_ACTIONS + LOCAL_STRESSES + SUPPORT + (MAXIMUM, SOLVER) + DERIVED),
          "exact": frozenset(NODAL + ("displacement_magnitude",) + LOCAL_ACTIONS + LOCAL_STRESSES + SUPPORT + WALL + WALL_STRESSES + (MAXIMUM, SOLVER))}
# Kinds whose rows carry a section basis on the exact route (each declared basis token of the contract names one).
STRESS_KINDS = frozenset(LOCAL_STRESSES + WALL_STRESSES + (MAXIMUM, "element_local_normal_stress_extreme",
                                                           "element_local_transverse_shear_stress_maximum"))
ENTITY_CLASS = {**{k: "node" for k in NODAL + ("displacement_magnitude",)}, **{k: "pipe" for k in LOCAL_ACTIONS + LOCAL_STRESSES + WALL + WALL_STRESSES + DERIVED + (MAXIMUM,)},
                **{k: "support" for k in SUPPORT}, SOLVER: "solver"}
FRAME = {**{k: "global" for k in NODAL + SUPPORT + ("element_global_end_force_v1", "element_global_end_moment_v1")},
         **{k: "element_local" for k in LOCAL_ACTIONS + LOCAL_STRESSES + WALL},
         **{k: "pipe_section" for k in WALL_STRESSES + (MAXIMUM, "element_local_normal_stress_extreme", "element_local_transverse_shear_stress_maximum")},
         SOLVER: "reduced_system"}
ROW_KEYS = frozenset({"kind", "value", "unit", "entity_ref", "case", "location", "component", "id", "metadata"})
ROW_METADATA = frozenset({"basis", "component", "coordinate_system", "location", "sign_convention"})
DIAG_REQUIRED = frozenset({"code", "id", "message", "severity", "source"})
DIAG_KEYS = DIAG_REQUIRED | {"affected_refs"}
SEVERITIES = frozenset({"info", "warning", "blocking"})
RAW_COMMON = frozenset({"tag", "entry", "mode", "ns", "request_sha256", "runner_sha256", "product_head"})
RAW_SUCCESS = RAW_COMMON | {"diagnostics", "results", "results_count", "numerical_quality", "summary", "contract_evidence", "publication"}
RAW_ERROR = RAW_COMMON | {"error"}
SUMMARY_COUNTS = ("component_stress_modifier_count", "curved_bend_macro_element_count", "load_case_count", "node_count", "segment_count",
                  "spring_hanger_user_input_count", "support_count")
HEADLINE_KEYS = frozenset({"location_ref", "result_ref", "unit", "value"})
SUMMARY_KEYS = {"pf": frozenset(SUMMARY_COUNTS + ("max_displacement", "max_open_formula_stress")),
                "exact": frozenset(SUMMARY_COUNTS + ("max_displacement", "max_open_formula_stress", "max_open_formula_stress_nominal_section",
                                                     "max_open_formula_stress_reduced_section"))}
EXTREMA_REQUIRED = frozenset({"pipe_id", "result_id", "value_lower_pa", "value_upper_pa"})
EXTREMA_TYPES = {"approximation": "str", "certified_gap_pa": "num?", "coefficient_basis": "str", "enclosure_scope": "str",
                 "global_upper_bound_pa": "num?", "local_fraction": "num?", "span_index": "int?", "station_fraction": "num?", "subdivisions": "int?"}
PF_CASE_KEYS = frozenset({"load_case_id", "pipe_stress_extrema", "intensified_measures", "stress_maximum_coverage", "support_attribution"})
EXACT_CASE_KEYS = frozenset({"load_case_id", "pipe_stress_extrema", "material_basis", "pipe_materials", "pipe_sections", "pressure_rhs_assembly",
                             "profile_mode", "stress_maximum_coverage"})

RAW_SCHEMA = """A raw record is exactly one of
  success: {tag, entry, mode, ns, diagnostics, results, results_count, numerical_quality, summary, contract_evidence,
            publication, request_sha256, runner_sha256, product_head}
  runner error: {tag, entry, mode, ns, error, request_sha256, runner_sha256, product_head}
tag a string; entry typed or captured; mode SparseInteractive or DenseScrutiny; ns a nonnegative integer; the two
sha256 64 lowercase hex; product_head 40 lowercase hex; error a nonempty string; results_count = len(results);
publication "ordinary"; numerical_quality null or an object (not read). Every number is a JSON number (a boolean is
not), and JSON is read strictly (m3_json: a duplicate key, NaN or Infinity is refused).
diagnostics: objects {code, id, message, severity, source[, affected_refs]}, strings (affected_refs a list of
  strings), severity info, warning or blocking, ids unique. Exactly one outcome:
  - a standing: per load case of the model exactly one diagnostic whose code is of STANDINGS, not blocking, with
    affected_refs exactly [that case], and no other NUMERICAL_INTEGRITY_* diagnostic and no blocking one; at least
    one row; or
  - a refusal: blocking diagnostics of exactly one code, of the route's REFUSALS, no two with the same affected
    refs, no standing and no rows (summary and contract_evidence then null or an object, not read).
  Other codes are warnings (the PF tau-domain warning is read as a set of codes).
results: rows of exactly {kind, value, unit, entity_ref, case, location, component, id, metadata}; ids unique;
  value a number (an integer only when exactly a binary64) or null; case a load case of the model; (kind,
  component, unit) a row of the route's contract table and the metadata basis one of its declared source bases for
  that row, or any basis without a section-basis token where the table declares none; the section basis is the
  token's (nominal_section or reduced_section; the exact route only, on the stress kinds); metadata null exactly on
  displacement_magnitude (location and component null), otherwise exactly {basis, component, coordinate_system,
  location, sign_convention}, strings, with location and component equal to the row's and the frame fixed for every
  judged kind (FRAME); entity_ref a node, pipe or support of the model by kind (ENTITY_CLASS; the solver-mode row
  names solver:linear_static_preview; other contract kinds any model entity or the solver); row identities (case,
  kind, entity, location, component, section basis) unique.
summary (standing): an object over the route's SUMMARY_KEYS; counts nonnegative integers; max_displacement null or
  {value number or null, unit "mm", location_ref, result_ref}; the headline max_open_formula_stress (and on the
  exact route max_open_formula_stress_<basis> for both bases) present, null or {value number or null, unit "Pa",
  location_ref, result_ref} whose result_ref is the id of a maximum row of the run with entity location_ref (of the
  headline's basis).
contract_evidence (standing):
  pf:    {combination_gates: [], preview_cases: [...]}, exactly one case object per load case, each {load_case_id,
         pipe_stress_extrema, intensified_measures: [], stress_maximum_coverage: {complete: bool,
         outside_domain_pipe_ids: [pipe], unavailable_pipe_ids: [pipe]}, support_attribution:
         {attributed_support_ids: [support], withheld: [...]}};
  exact: {exact_cases: [...], connector: [...], pressure: [...]}, exactly one case object per load case, each
         {load_case_id, pipe_stress_extrema} and optionally material_basis and profile_mode (strings),
         pipe_materials and pipe_sections (lists), pressure_rhs_assembly (an object or null) and
         stress_maximum_coverage (a list of {complete: bool, section_basis, unavailable_pipe_ids: [pipe]});
  every extremum {pipe_id, result_id, value_lower_pa, value_upper_pa[, section_basis on the exact route, required]}
  plus the typed members of EXTREMA_TYPES and nothing else: pipe_id a pipe of the model; bounds numbers or null with
  value_lower_pa <= value_upper_pa when both are numbers; result_id the id of the maximum row of the same case, pipe
  and section basis; (case, pipe, section basis) unique. Only load_case_id and the extrema enter the view: no other
  member is read, so nothing placed elsewhere (a nested or shadow entry) can supply evidence."""


# ----------------------------------------------------------------- the public contract
_CONTRACT = {}


def contract(route, name=None):
    """{(kind, component): [(unit, declared source basis or None), ...]} of a semantic contract of the route (its first
    by default), and the file's sha256."""
    key = (route, name or CONTRACTS[route][0][0])
    if key not in _CONTRACT:
        fname = dict(CONTRACTS[route])[key[1]]
        path = os.path.join(FIXTURES, fname)
        doc, probs = m3_json.read_json(path)
        if probs or not isinstance(doc, dict) or doc.get("semantic_contract_id") != key[1] or not isinstance(doc.get("rows"), list):
            raise ValueError(f"the {key[1]} semantic contract is unreadable at {path}: {probs[:1]}")
        table = collections.defaultdict(list)
        for r in doc["rows"]:
            sb = r.get("source_basis")
            table[(r["kind"], r["component"])].append((r["unit"], sb))
        _CONTRACT[key] = (dict(table), hashlib.sha256(open(path, "rb").read()).hexdigest())
    return _CONTRACT[key]


def contract_unit(route, kind, component):
    """The single unit the route's first contract fixes for a judged kind and component (None when it fixes none or several)."""
    units = {u for u, _ in contract(route)[0].get((kind, component), [])}
    return next(iter(units)) if len(units) == 1 else None


def _section_basis(token):
    if token is None or SECTION_MARKER not in token:
        return None
    return token.split(SECTION_MARKER, 1)[1].split(";")[0].strip()


def match_contract(route, kind, component, unit, source, name=None):
    """(section basis, problem): the row against the contract table: its (kind, component, unit) must be a row of it and
    its source basis a declared basis of that row, or any basis without a section-basis token where none is declared."""
    table = contract(route, name)[0]
    cands = [sb for u, sb in table.get((kind, component), []) if u == unit]
    if not cands:
        known = sorted({u for u, _ in table.get((kind, component), [])})
        return None, (f"unit {unit!r} of {kind}/{component} is not the contract's {known}" if known else f"{kind}/{component} is not in the {route} contract")
    if source is not None and source in cands:
        b = _section_basis(source)
        if b is not None and b not in SECTION_BASES:
            return None, f"section basis {b!r}"
        return b, None
    if None in cands:
        if _section_basis(source) is not None:
            return None, f"{kind}: a section-basis token where the contract declares none ({source!r})"
        return None, None
    return None, f"{kind}/{component}: source basis {source!r} is not a declared basis of the contract"


# ----------------------------------------------------------------- helpers
def is_int(x):
    return type(x) is int


def is_num(x):
    return type(x) in (int, float)


def binary64(x):
    """(float, ok) of a JSON number: a finite float as it is; an integer only when it is exactly a finite binary64."""
    if type(x) is float:
        return (x, True) if math.isfinite(x) else (None, False)
    if type(x) is not int:
        return None, False
    try:
        f = float(x)
    except OverflowError:
        return None, False
    return (f, True) if int(f) == x else (None, False)


def opt_binary64(x):
    if x is None:
        return None, True
    return binary64(x)


def request_line(tag, request):
    """The runner-input line of a model, exactly as the generators' split writes it and the runner hashes it."""
    return json.dumps({"tag": tag, "request": request}, sort_keys=True, separators=(",", ":"))


def request_sha256(tag, request):
    return hashlib.sha256(request_line(tag, request).encode()).hexdigest()


def request_lines(gens):
    return "".join(request_line(g["tag"], g["request"]) + "\n" for g in gens)


class Facts:
    """What the validator binds a record to (not part of the view): the model's entities and load cases."""

    def __init__(self, nodes, pipes, supports, cases):
        self.nodes, self.pipes, self.supports, self.cases = frozenset(nodes), frozenset(pipes), frozenset(supports), tuple(cases)
        self.entities = self.nodes | self.pipes | self.supports

    def entity_ok(self, kind, entity):
        cls = ENTITY_CLASS.get(kind)
        if cls == "node":
            return entity in self.nodes
        if cls == "pipe":
            return entity in self.pipes
        if cls == "support":
            return entity in self.supports
        if cls == "solver":
            return entity == SOLVER_ENTITY
        return entity in self.entities or entity == SOLVER_ENTITY


# ----------------------------------------------------------------- raw record -> RunView
def run_view(raw, route, facts):
    """(RunView or None, problems): a strictly parsed raw record validated against RAW_SCHEMA and bound to its model."""
    try:
        return _run_view(raw, route, facts)
    except Exception as e:  # a shape the checks did not anticipate is itself a refusal, never a crash
        return None, [f"unreadable record ({type(e).__name__}: {e})"]


def _common_problems(r):
    out = []
    g = r.get
    if not isinstance(g("tag"), str) or not g("tag"):
        out.append("tag")
    if g("entry") not in ENTRIES:
        out.append("entry")
    if g("mode") not in MODES:
        out.append("mode")
    if not is_int(g("ns")) or g("ns") < 0:
        out.append("ns")
    for k in ("request_sha256", "runner_sha256"):
        if not isinstance(g(k), str) or not HEX64.match(g(k)):
            out.append(k)
    if not isinstance(g("product_head"), str) or not HEX40.match(g("product_head")):
        out.append("product_head is not 40 lowercase hex")
    return out


def _run_view(r, route, facts):
    if not isinstance(r, dict):
        return None, ["not an object"]
    want = RAW_ERROR if "error" in r else RAW_SUCCESS
    out = [f"missing {k}" for k in sorted(want - set(r))] + [f"unknown key {k}" for k in sorted(set(r) - want)]
    out += _common_problems(r)
    if out:
        return None, out
    base = dict(route=route, tag=r["tag"], entry=r["entry"], mode=r["mode"], request_sha256=r["request_sha256"], runner_sha256=r["runner_sha256"],
                product_head=r["product_head"])
    if "error" in r:
        if not isinstance(r["error"], str) or not r["error"]:
            return None, ["error is not a nonempty string"]
        return RunView(error=r["error"], refusal=None, standings=(), rows=(), extrema=(), headlines=(), warnings=frozenset(), contract=None, **base), []
    if facts is None:
        return None, ["a model outside the reference set"]
    if r["publication"] != "ordinary":
        out.append("publication")
    if not (r["numerical_quality"] is None or isinstance(r["numerical_quality"], dict)):
        out.append("numerical_quality")
    standings, refusal, warnings, probs = _outcome(r["diagnostics"], route, facts)
    out += probs
    names = [n for n, _ in CONTRACTS[route]]
    first, chosen = None, None
    for name in names:
        rows, by_id, probs = _rows(r["results"], route, facts, name)
        first = first or (rows, by_id, probs)
        if not probs:
            chosen = name
            break
    if chosen is None:
        rows, by_id, probs = first
        if len(names) > 1:
            probs = probs + [f"the rows match no single {route} contract ({', '.join(n.rsplit('/', 1)[-1] for n in names)})"]
    out += probs
    if not isinstance(r["results"], list) or not is_int(r["results_count"]) or r["results_count"] != len(r["results"]):
        out.append("results_count")
    if out:
        return None, out
    if refusal is not None:
        if rows:
            out.append("a refusal that publishes rows")
        if not (r["summary"] is None or isinstance(r["summary"], dict)):
            out.append("summary")
        if not (r["contract_evidence"] is None or isinstance(r["contract_evidence"], dict)):
            out.append("contract_evidence")
        if out:
            return None, out
        return RunView(error=None, refusal=refusal, standings=(), rows=(), extrema=(), headlines=(), warnings=warnings, contract=None, **base), []
    if not rows:
        out.append("a standing with no rows")
    headlines, probs = _summary(r["summary"], route, by_id)
    out += probs
    extrema, probs = _evidence(r["contract_evidence"], route, facts, by_id, chosen)
    out += probs
    if out:
        return None, out
    return RunView(error=None, refusal=None, standings=standings, rows=rows, extrema=extrema, headlines=headlines, warnings=warnings, contract=chosen,
                   **base), []


def _outcome(ds, route, facts):
    """(standings, refusal code, warning codes, problems) of the diagnostics."""
    if not isinstance(ds, list):
        return (), None, frozenset(), ["diagnostics"]
    for d in ds:
        if (not isinstance(d, dict) or not DIAG_REQUIRED <= set(d) or not set(d) <= DIAG_KEYS or not all(isinstance(d[k], str) for k in DIAG_REQUIRED)
                or d["severity"] not in SEVERITIES
                or ("affected_refs" in d and not (isinstance(d["affected_refs"], list) and all(isinstance(x, str) for x in d["affected_refs"])))):
            return (), None, frozenset(), ["a diagnostic"]
    out = []
    ids = collections.Counter(d["id"] for d in ds)
    if any(n > 1 for n in ids.values()):
        out.append(f"duplicate diagnostic ids {sorted(i for i, n in ids.items() if n > 1)[:3]}")
    integrity = [d for d in ds if d["code"].startswith("NUMERICAL_INTEGRITY")]
    blocking = [d for d in ds if d["severity"] == "blocking"]
    unknown = sorted({d["code"] for d in integrity} - STANDINGS)
    if unknown:
        out.append(f"unrecognised standing {unknown}")
    warnings = frozenset(d["code"] for d in ds if d not in integrity and d["severity"] != "blocking")
    if blocking:
        codes = sorted({d["code"] for d in blocking})
        if len(codes) != 1 or codes[0] not in REFUSALS[route]:
            out.append(f"refusal codes {codes}: exactly one code of the {route} refusals required")
        refs = collections.Counter(tuple(d.get("affected_refs", ())) for d in blocking)
        if any(n > 1 for n in refs.values()):
            out.append("the same refusal given twice")
        if integrity:
            out.append("a refusal with a standing")
        return (), (codes[0] if not out else None), warnings, out
    if not integrity:
        return (), None, warnings, out + ["no outcome: neither a standing nor a refusal"]
    standings = []
    for cid in facts.cases:
        mine = [d for d in integrity if d.get("affected_refs") == [cid]]
        if len(mine) != 1:
            out.append(f"not exactly one standing for {cid} ({len(mine)})")
        else:
            standings.append(Standing(case=cid, code=mine[0]["code"], message=mine[0]["message"]))
    if len(integrity) != len(facts.cases):
        out.append(f"{len(integrity)} standings for {len(facts.cases)} load cases")
    return tuple(standings), None, warnings, out


def _rows(rows, route, facts, name):
    """(rows, {id: Row}, problems) of the published results."""
    if not isinstance(rows, list):
        return (), {}, ["results"]
    out, view, by_id, seen = [], [], {}, set()
    for i, x in enumerate(rows):
        where = f"row {i}"
        if not isinstance(x, dict) or set(x) != ROW_KEYS:
            out.append(f"{where}: keys")
            continue
        if not all(isinstance(x[k], str) for k in ("kind", "unit", "entity_ref", "id")) or not all(x[k] is None or isinstance(x[k], str)
                                                                                                    for k in ("case", "location", "component")):
            out.append(f"{where}: field types")
            continue
        kind, md = x["kind"], x["metadata"]
        value, ok = opt_binary64(x["value"])
        if not ok:
            out.append(f"{where}: value {x['value']!r} is not a binary64 number or null")
            continue
        if x["case"] not in facts.cases:
            out.append(f"{where}: case {x['case']!r} is not a load case of the model")
        if not facts.entity_ok(kind, x["entity_ref"]):
            out.append(f"{where}: entity {x['entity_ref']!r} is not a {ENTITY_CLASS.get(kind, 'model entity')} of the model for {kind}")
        if md is None:
            if kind != "displacement_magnitude" or x["location"] is not None or x["component"] is not None:
                out.append(f"{where}: metadata null on {kind}")
            source = None
        else:
            if (not isinstance(md, dict) or set(md) != ROW_METADATA or not all(isinstance(v, str) for v in md.values())):
                out.append(f"{where}: metadata keys or types")
                continue
            if md["location"] != x["location"] or md["component"] != x["component"]:
                out.append(f"{where}: metadata location/component {md['location']!r}/{md['component']!r} contradict the row's "
                           f"{x['location']!r}/{x['component']!r}")
            if kind in FRAME and md["coordinate_system"] != FRAME[kind]:
                out.append(f"{where}: frame {md['coordinate_system']!r} of {kind} is not {FRAME[kind]!r}")
            if kind == "displacement_magnitude":
                out.append(f"{where}: metadata on displacement_magnitude")
            source = md["basis"]
        basis, problem = match_contract(route, kind, x["component"], x["unit"], source, name)
        if problem:
            out.append(f"{where}: {problem}")
            continue
        if route == "exact" and kind in JUDGED["exact"] and (kind in STRESS_KINDS) != (basis is not None):
            out.append(f"{where}: {kind} {'without' if basis is None else 'with'} a section basis")
            continue
        row = Row(case=x["case"] or "", kind=kind, entity=x["entity_ref"], location=x["location"], component=x["component"], basis=basis,
                  unit=x["unit"], value=value, source=source)
        if x["id"] in by_id:
            out.append(f"{where}: duplicate row id {x['id']!r}")
            continue
        if row.identity in seen:
            out.append(f"{where}: duplicate row identity {row.identity}")
            continue
        seen.add(row.identity)
        by_id[x["id"]] = row
        view.append(row)
    return tuple(view), by_id, out


def _headline(h, basis, by_id, name):
    """(Headline, problems) of one summary headline."""
    if h is None:
        return Headline(basis=basis, is_null=True, value=None), []
    if not isinstance(h, dict) or set(h) != HEADLINE_KEYS or not isinstance(h["location_ref"], str) or not isinstance(h["result_ref"], str):
        return None, [f"summary.{name}"]
    value, ok = opt_binary64(h["value"])
    if not ok or h["unit"] != "Pa":
        return None, [f"summary.{name}: value or unit (must be a number or null, in 'Pa')"]
    row = by_id.get(h["result_ref"])
    if row is None or row.kind != MAXIMUM or row.entity != h["location_ref"] or row.basis != basis:
        return None, [f"summary.{name}: result_ref {h['result_ref']!r} is not a maximum row of {h['location_ref']!r}"
                      + ("" if basis is None else f" on {basis}")]
    return Headline(basis=basis, is_null=False, value=value), []


def _summary(sm, route, by_id):
    if not isinstance(sm, dict) or not set(sm) <= SUMMARY_KEYS[route]:
        return (), ["summary"]
    out = []
    for k in SUMMARY_COUNTS:
        if k in sm and not (is_int(sm[k]) and sm[k] >= 0):
            out.append(f"summary.{k}")
    md = sm.get("max_displacement")
    if md is not None and not (isinstance(md, dict) and set(md) == HEADLINE_KEYS and md["unit"] == "mm" and opt_binary64(md["value"])[1]
                               and isinstance(md["location_ref"], str) and isinstance(md["result_ref"], str)):
        out.append("summary.max_displacement")
    names = [("max_open_formula_stress", None)] + ([("max_open_formula_stress_" + b, b) for b in SECTION_BASES] if route == "exact" else [])
    heads = []
    for name, basis in names:
        if name not in sm:
            out.append(f"summary.{name} absent")
            continue
        h, probs = _headline(sm[name], basis, by_id, name)
        out += probs
        if h is not None:
            heads.append(h)
    return tuple(heads), out


def _typed(v, t):
    if t == "str":
        return isinstance(v, str)
    if t == "num?":
        return v is None or opt_binary64(v)[1]
    if t == "int?":
        return v is None or is_int(v)
    return False


def _pipe_list(xs, facts):
    return isinstance(xs, list) and all(isinstance(p, str) and p in facts.pipes for p in xs)


def _evidence(ev, route, facts, by_id, contract=None):
    """(extrema, problems): the case-bound, validated maximum-stress enclosures; nothing else of the evidence is read.
    The evidence shape follows the record's contract: pressure-2's PressureContractEvidence, or under pressure-source-1
    the recovery evidence of SOURCE_RECOVERY_SCHEMA (each refused under the other)."""
    out = []
    recovery = route == "exact" and contract == CONTRACTS["exact"][1][0]
    if recovery:
        if not isinstance(ev, dict):
            return (), [f"contract_evidence {type(ev).__name__} (pressure-source-1 requires its recovery evidence object)"]
        ptr = SOURCE_RECOVERY_EVIDENCE["load_reference_states" in ev]
        probs = schema_problems(ev, {"$ref": f"#{ptr}"}, SOURCE_RECOVERY_SCHEMA)
        if probs:
            return (), [f"contract_evidence (pressure-source-1, {ptr.rsplit('/', 1)[-1]}){p}" for p in probs]
        out += _state_cases(ev, facts)
        cases, case_keys = ev["exact_cases"], recovery_case_keys()
    elif route == "pf":
        if not isinstance(ev, dict) or set(ev) != {"combination_gates", "preview_cases"} or ev["combination_gates"] != [] or not isinstance(ev["preview_cases"], list):
            return (), ["contract_evidence"]
        cases, case_keys = ev["preview_cases"], PF_CASE_KEYS
    else:
        members, required = evidence_members()
        if not isinstance(ev, dict) or not required <= set(ev) <= members or not all(isinstance(v, list) for v in ev.values()):
            return (), [f"contract_evidence members {sorted(ev) if isinstance(ev, dict) else type(ev).__name__} (the schema requires "
                        f"{sorted(required)}, admits {sorted(members)})"]
        out += _schema_checked_members(ev, facts)
        cases, case_keys = ev["exact_cases"], EXACT_CASE_KEYS
    seen_cases = collections.Counter()
    extrema, seen = [], set()
    for c in cases:
        if not isinstance(c, dict) or not {"load_case_id", "pipe_stress_extrema"} <= set(c) <= case_keys or not isinstance(c["load_case_id"], str) \
                or not isinstance(c["pipe_stress_extrema"], list):
            out.append("a case of the contract evidence")
            continue
        cid = c["load_case_id"]
        seen_cases[cid] += 1
        if cid not in facts.cases:
            out.append(f"evidence of {cid!r}, not a load case of the model")
            continue
        if route == "pf":
            if set(c) != PF_CASE_KEYS or c["intensified_measures"] != []:
                out.append(f"evidence {cid}: members (intensified_measures must be [] on PF's straight members)")
            cov, att = c.get("stress_maximum_coverage"), c.get("support_attribution")
            if not (isinstance(cov, dict) and set(cov) == {"complete", "outside_domain_pipe_ids", "unavailable_pipe_ids"} and isinstance(cov["complete"], bool)
                    and _pipe_list(cov["outside_domain_pipe_ids"], facts) and _pipe_list(cov["unavailable_pipe_ids"], facts)):
                out.append(f"evidence {cid}: stress_maximum_coverage")
            if not (isinstance(att, dict) and set(att) == {"attributed_support_ids", "withheld"} and isinstance(att["withheld"], list)
                    and isinstance(att["attributed_support_ids"], list) and all(isinstance(s, str) and s in facts.supports for s in att["attributed_support_ids"])):
                out.append(f"evidence {cid}: support_attribution")
        else:
            for k, ok in (("material_basis", lambda v: isinstance(v, str)), ("profile_mode", lambda v: isinstance(v, str)),
                          ("pipe_materials", lambda v: isinstance(v, list)), ("pipe_sections", lambda v: isinstance(v, list)),
                          ("pressure_rhs_assembly", lambda v: v is None or isinstance(v, dict)),
                          ("stress_maximum_coverage", lambda v: isinstance(v, list) and all(
                              isinstance(x, dict) and set(x) == {"complete", "section_basis", "unavailable_pipe_ids"} and isinstance(x["complete"], bool)
                              and x["section_basis"] in SECTION_BASES and _pipe_list(x["unavailable_pipe_ids"], facts) for x in v))):
                if k in c and not ok(c[k]):
                    out.append(f"evidence {cid}: {k}")
        for e in c["pipe_stress_extrema"]:
            if recovery:   # members and types already validated against the recovery schema; its own case binding:
                if not isinstance(e, dict) or not EXTREMA_REQUIRED | {"section_basis"} <= set(e) or e.get("load_case_id", cid) != cid:
                    out.append(f"evidence {cid}: an extremum's members or its load_case_id")
                    continue
            else:
                allowed = EXTREMA_REQUIRED | set(EXTREMA_TYPES) | ({"section_basis"} if route == "exact" else set())
                if not isinstance(e, dict) or not EXTREMA_REQUIRED <= set(e) <= allowed or (route == "exact" and "section_basis" not in e):
                    out.append(f"evidence {cid}: an extremum's members")
                    continue
                if not all(_typed(e[k], t) for k, t in EXTREMA_TYPES.items() if k in e):
                    out.append(f"evidence {cid}: an extremum member's type")
                    continue
            basis = e.get("section_basis")
            if route == "exact" and basis not in SECTION_BASES:
                out.append(f"evidence {cid}: section_basis {basis!r}")
                continue
            lo, ok1 = opt_binary64(e["value_lower_pa"])
            hi, ok2 = opt_binary64(e["value_upper_pa"])
            if not (ok1 and ok2):
                out.append(f"evidence {cid}: extremum bounds must be numbers or null")
                continue
            if lo is not None and hi is not None and not lo <= hi:
                out.append(f"evidence {cid}: inverted extremum interval {lo!r} > {hi!r} for {e['pipe_id']!r}")
                continue
            if not isinstance(e["pipe_id"], str) or e["pipe_id"] not in facts.pipes:
                out.append(f"evidence {cid}: extremum of {e['pipe_id']!r}, not a pipe of the model")
                continue
            row = by_id.get(e["result_id"]) if isinstance(e["result_id"], str) else None
            if row is None or row.kind != MAXIMUM or row.case != cid or row.entity != e["pipe_id"] or row.basis != basis:
                out.append(f"evidence {cid}: extremum result_id {e['result_id']!r} is not the maximum row of {e['pipe_id']!r}"
                           + ("" if basis is None else f" on {basis}"))
                continue
            ident = (cid, e["pipe_id"], basis)
            if ident in seen:
                out.append(f"evidence {cid}: duplicate extremum identity {ident}")
                continue
            seen.add(ident)
            extrema.append(Extremum(case=cid, pipe=e["pipe_id"], basis=basis, lower=lo, upper=hi))
    for cid in facts.cases:
        if seen_cases[cid] != 1:
            out.append(f"evidence: {seen_cases[cid]} case objects for {cid} (exactly one required)")
    return tuple(extrema), out


# ----------------------------------------------------------------- public schemas (exact evidence members)
_SCHEMA_DOCS = {}
SCHEMA_KEYWORDS = frozenset({"$ref", "type", "additionalProperties", "required", "properties", "items", "const", "enum", "minLength", "minimum",
                             "pattern", "oneOf", "minItems", "if", "then", "else", "exclusiveMinimum", "maximum", "maxItems", "uniqueItems",
                             "description", "title", "$comment"})
SHAPE_CHECKED = ("load_reference_states", "exact_combinations")     # read only for their shape; never judged


def _schema_doc(name):
    if name not in _SCHEMA_DOCS:
        doc, probs = m3_json.read_json(os.path.join(SCHEMAS, name))
        if probs or not isinstance(doc, dict):
            raise ValueError(f"schema {name} unreadable: {probs[:1]}")
        _SCHEMA_DOCS[name] = doc
    return _SCHEMA_DOCS[name]


def _resolve(doc_name, pointer):
    node = _schema_doc(doc_name)
    for part in pointer.strip("/").split("/"):
        node = node[part]
    return node


def evidence_members():
    """(admitted members, required members) of the exact route's contract_evidence, read from the schema."""
    node = _resolve(*EXACT_EVIDENCE_SCHEMA)
    if node.get("additionalProperties") is not False:
        raise ValueError("the exact evidence schema is not closed")
    return frozenset(node["properties"]), frozenset(node.get("required", ()))


# JSON Schema's types: an integer is any number with a zero fractional part (0.0 and 12.0 included); a boolean is never a
# number (type(True) is bool, not int).
_TYPES = {"object": lambda v: type(v) is dict, "array": lambda v: type(v) is list, "string": lambda v: type(v) is str,
          "number": lambda v: type(v) in (int, float), "integer": lambda v: type(v) is int or (type(v) is float and v.is_integer()),
          "boolean": lambda v: type(v) is bool, "null": lambda v: v is None}
_SUBSCHEMA = ("items", "if", "then", "else")       # keywords whose value is one schema; properties and oneOf hold several


class SchemaUnsupported(ValueError):
    """A schema this checker cannot evaluate in full (an unsupported keyword or type, a node that is not an object, an
    unresolvable $ref). It is raised, never reported as a failed match, so the record is refused outright."""


def schema_supported(node, doc_name, seen=None):
    """Every node reachable from `node` (through $ref, properties, additionalProperties, items, if/then/else and oneOf)
    uses only SCHEMA_KEYWORDS and _TYPES, wherever it sits; raises SchemaUnsupported otherwise."""
    seen = set() if seen is None else seen
    if (doc_name, id(node)) in seen:
        return
    seen.add((doc_name, id(node)))
    if not isinstance(node, dict):
        raise SchemaUnsupported(f"{doc_name}: a schema node that is not an object")
    unknown = set(node) - SCHEMA_KEYWORDS
    if unknown:
        raise SchemaUnsupported(f"{doc_name}: unsupported schema keywords {sorted(unknown)}")
    if "type" in node and any(t not in _TYPES for t in (node["type"] if isinstance(node["type"], list) else [node["type"]])):
        raise SchemaUnsupported(f"{doc_name}: unsupported type {node['type']}")
    if "$ref" in node:
        f, _, ptr = node["$ref"].partition("#")
        f = f or doc_name
        try:
            target = _resolve(f, ptr)
        except Exception:
            raise SchemaUnsupported(f"{doc_name}: unresolvable $ref {node['$ref']}") from None
        schema_supported(target, f, seen)
    for k in _SUBSCHEMA:
        if k in node:
            schema_supported(node[k], doc_name, seen)
    if isinstance(node.get("additionalProperties"), dict):
        schema_supported(node["additionalProperties"], doc_name, seen)
    for sub in (node.get("properties") or {}).values():
        schema_supported(sub, doc_name, seen)
    for sub in node.get("oneOf", ()):
        schema_supported(sub, doc_name, seen)


def _json_equal(a, b):
    """JSON Schema equality: numbers by value (0 equals 0.0), a boolean never a number, containers member by member."""
    num = lambda x: type(x) in (int, float)
    if num(a) and num(b):
        return a == b
    if type(a) is not type(b):
        return False
    if type(a) is list:
        return len(a) == len(b) and all(_json_equal(x, y) for x, y in zip(a, b))
    if type(a) is dict:
        return set(a) == set(b) and all(_json_equal(a[k], b[k]) for k in a)
    return a == b


def schema_problems(value, node, doc_name, where="", budget=None):
    """Problems of a JSON value against a schema node (a closed subset of JSON Schema). The whole schema reachable from
    the node is checked first: an unsupported keyword anywhere, in a branch of if/then/else or oneOf included, raises
    SchemaUnsupported (a refusal of the record), never a failed match."""
    if budget is None:
        schema_supported(node, doc_name)
        budget = [20]
    out = []

    def bad(msg):
        if budget[0] > 0:
            out.append(f"{where or '/'}: {msg}")
        budget[0] -= 1
    if not isinstance(node, dict) or set(node) - SCHEMA_KEYWORDS:   # unreachable after schema_supported; never a failed match
        raise SchemaUnsupported(f"{doc_name}{where}: an unsupported schema node")
    if "$ref" in node:
        f, _, ptr = node["$ref"].partition("#")
        f = f or doc_name
        out += schema_problems(value, _resolve(f, ptr), f, where, budget)
    if "type" in node:
        ts = node["type"] if isinstance(node["type"], list) else [node["type"]]
        if not any(_TYPES[t](value) for t in ts):
            bad(f"type {type(value).__name__}, not {node['type']}")
            return out
    if "const" in node and not _json_equal(value, node["const"]):
        bad(f"not the constant {node['const']!r}")
    if "enum" in node and not any(_json_equal(value, e) for e in node["enum"]):
        bad(f"{value!r} not in {node['enum']}")
    if type(value) is str:
        if "minLength" in node and len(value) < node["minLength"]:
            bad("too short")
        if "pattern" in node and not re.search(node["pattern"], value):
            bad(f"does not match {node['pattern']}")
    if type(value) in (int, float):
        if "minimum" in node and value < node["minimum"]:
            bad(f"below {node['minimum']}")
        if "exclusiveMinimum" in node and not value > node["exclusiveMinimum"]:
            bad(f"not above {node['exclusiveMinimum']}")
        if "maximum" in node and value > node["maximum"]:
            bad(f"above {node['maximum']}")
    if type(value) is list:
        if "minItems" in node and len(value) < node["minItems"]:
            bad("too few items")
        if "maxItems" in node and len(value) > node["maxItems"]:
            bad("too many items")
        if node.get("uniqueItems") is True and any(_json_equal(value[i], value[k]) for i in range(len(value)) for k in range(i)):
            bad("repeated items")
        if "items" in node:
            for i, v in enumerate(value):
                out += schema_problems(v, node["items"], doc_name, f"{where}[{i}]", budget)
    if type(value) is dict:
        props = node.get("properties", {})
        for k in node.get("required", ()):
            if k not in value:
                bad(f"missing {k}")
        extra = node.get("additionalProperties", True)
        for k, v in value.items():
            if k in props:
                out += schema_problems(v, props[k], doc_name, f"{where}.{k}", budget)
            elif extra is False:
                bad(f"unknown key {k}")
            elif isinstance(extra, dict):
                out += schema_problems(v, extra, doc_name, f"{where}.{k}", budget)
    if "if" in node:
        branch = "then" if not schema_problems(value, node["if"], doc_name, where, [20]) else "else"
        if branch in node:
            out += schema_problems(value, node[branch], doc_name, where, budget)
    if "oneOf" in node:
        n = sum(1 for sub in node["oneOf"] if not schema_problems(value, sub, doc_name, where, [20]))
        if n != 1:
            bad(f"matches {n} of oneOf's {len(node['oneOf'])} alternatives")
    return out


def _schema_checked_members(ev, facts):
    """load_reference_states and exact_combinations: shape-checked against the schema and (load_reference_states) one
    record per load case of the model; never read for judging and never part of the view."""
    out = []
    doc, ptr = EXACT_EVIDENCE_SCHEMA
    props = _resolve(doc, ptr)["properties"]
    for member in SHAPE_CHECKED:
        if member in ev:
            out += [f"contract_evidence.{member}{p}" for p in schema_problems(ev[member], props[member], doc)]
    if not out:
        out += _state_cases(ev, facts)
    return out


def _state_cases(ev, facts):
    """load_reference_states (when present): one record per load case, each a load case of the model."""
    out = []
    if "load_reference_states" in ev:
        ids = [x["load_case_id"] for x in ev["load_reference_states"]]
        if len(set(ids)) != len(ids):
            out.append("contract_evidence.load_reference_states: a load case given twice")
        if not set(ids) <= set(facts.cases):
            out.append(f"contract_evidence.load_reference_states: {sorted(set(ids) - set(facts.cases))[:3]} not a load case of the model")
    return out


def recovery_case_keys():
    """The members a pressure-source-1 exact case may carry: the union of the ordinary and resolved case definitions."""
    keys = set()
    for ptr in SOURCE_RECOVERY_EVIDENCE.values():
        items = _resolve(SOURCE_RECOVERY_SCHEMA, ptr)["properties"]["exact_cases"]["items"]
        if "$ref" in items:
            items = _resolve(SOURCE_RECOVERY_SCHEMA, items["$ref"].partition("#")[2])
        if items.get("additionalProperties") is not False:
            raise ValueError("a pressure-source-1 exact case definition is not closed")
        keys |= set(items["properties"])
    return frozenset(keys)


# ----------------------------------------------------------------- reference lines -> ModelView, Expected
# Agent 0 (the judge's final round): every datum the judge uses is either (a) derived from the hash-bound request,
# parsed into the view and unique, or (b) read from a reference file whose sha256 the judging pass verifies against
# the committed REFERENCE_SHA256SUMS as part of provenance. Nothing else in a reference is read.
#   (a) the request: node ids and positions (L_b, bodies), pipe ids and ends, support ids and nodes, load case ids,
#       components; every run binds it through request_sha256 = sha256(request_line(tag, request)).
#   (b) the reference line, besides its request: meta family, theory, twin and the reporting keys REPORT_META; and
#       ref: on PF expect, refusal.code, expected_warnings, sections[pipe] {ro, A, Z, J, I, tau_c, E, G, od, wall,
#       mill}; on the exact route sections[pipe] {ro, nominal_section and reduced_section {A, Z, J}}; and on both
#       cases[case] {rows, maxima}. The controls file (exact route): one {tag, pertE} per model with a control,
#       pertE an exact-route ref read as above.
# ref.nodes, phi, notes and every other member are not read.
REPORT_META = ("design", "family", "theory", "placement", "section_variant", "twin")
PF_TAU_FLOATS = ("od", "wall", "mill", "E", "G")
PF_EXACT = ("ro", "A", "I", "J", "Z", "tau_c")


def _decimal_text(x):
    try:
        return type(x) is str and Decimal(x).is_finite()
    except Exception:
        return False


def _fq(text):
    return Q(Decimal(text))


def _float_text(x):
    """A decimal text of a finite binary64 (the generators write repr(float); E, G, OD, wall and mill are read as floats)."""
    return _decimal_text(x) and math.isfinite(float(x))


def _id_list(xs, what):
    """(ids, problems) of a request list of objects with string ids: unique."""
    if type(xs) is not list or not all(type(e) is dict and type(e.get("id")) is str and e["id"] for e in xs):
        return None, [f"request {what}"]
    ids = [e["id"] for e in xs]
    return ids, ([f"request: duplicate {what} ids {sorted({i for i in ids if ids.count(i) > 1})[:3]}"] if len(set(ids)) != len(ids) else [])


def request_model(q):
    """(Facts, L_b, bodies, components, problems) parsed from a request: unique ids across nodes, pipes, supports and
    load cases; every pipe end and support node a defined node; positions finite binary64 (m)."""
    if type(q) is not dict or type(q.get("model")) is not dict:
        return None, 0.0, 0, 0, ["request"]
    mdl = q["model"]
    out = []
    nodes, p1 = _id_list(mdl.get("nodes"), "nodes")
    pipes, p2 = _id_list(mdl.get("pipe_segments"), "pipe_segments")
    sups, p3 = _id_list(mdl.get("supports"), "supports")
    cases, p4 = _id_list(mdl.get("load_cases"), "load_cases")
    out += p1 + p2 + p3 + p4
    if None in (nodes, pipes, sups, cases) or type(mdl.get("components")) is not list:
        return None, 0.0, 0, 0, out + ["request model"]
    every = nodes + pipes + sups + cases + [SOLVER_ENTITY]
    if len(set(every)) != len(every):
        out.append("request: an id names two entities")
    if not cases:
        out.append("request: no load cases")
    points = []
    for n in mdl["nodes"]:
        pos = n.get("position")
        if type(pos) is not dict or not all(type(pos.get(a)) is float and math.isfinite(pos[a]) for a in "xyz"):
            out.append(f"request: position of {n['id']}")
        else:
            points.append([pos["x"], pos["y"], pos["z"]])
    known = set(nodes)
    for p in mdl["pipe_segments"]:
        if type(p.get("from")) is not str or type(p.get("to")) is not str or p["from"] not in known or p["to"] not in known or p["from"] == p["to"]:
            out.append(f"request: ends of {p['id']}")
    for s in mdl["supports"]:
        if type(s.get("node")) is not str or s["node"] not in known:
            out.append(f"request: node of {s['id']}")
    if out or not points:
        return None, 0.0, 0, 0, out or ["request: no nodes"]
    # the node bounding-box diagonal, in kd5t_compare's binary64 arithmetic (formerly read from ref.nodes)
    delta = [max(p[a] for p in points) - min(p[a] for p in points) for a in range(3)]
    Lb = ((delta[0] ** 2 + delta[1] ** 2) + delta[2] ** 2) ** 0.5
    parent = {n: n for n in nodes}

    def find(a):
        while parent[a] != a:
            parent[a] = parent[parent[a]]
            a = parent[a]
        return a

    for p in mdl["pipe_segments"]:
        parent[find(p["from"])] = find(p["to"])
    bodies = len({find(a) for a in parent})
    return Facts(nodes, pipes, sups, cases), Lb, bodies, len(mdl["components"]), []


def _ref_rows(c, facts, route, cid):
    """(rows, derived, maxima, problems) of one reference case, bound to the request's entities."""
    if type(c) is not dict or type(c.get("rows")) is not list or type(c.get("maxima")) is not list:
        return None, None, None, [f"case {cid}"]
    out, rows, derived, maxima = [], [], [], []
    for w in c["rows"]:
        ok = (type(w) is list and len(w) == 6 and type(w[0]) is str and type(w[1]) is str and (w[2] is None or type(w[2]) is str)
              and (w[3] is None or type(w[3]) is str) and _decimal_text(w[5]))
        if ok:
            ok = (w[4] in SECTION_BASES if (route == "exact" and w[0] in STRESS_KINDS) else w[4] is None) and facts.entity_ok(w[0], w[1]) \
                and w[0] in ENTITY_CLASS and w[0] != SOLVER
        if not ok:
            out.append(f"case {cid}: a reference row {str(w)[:120]}")
            continue
        ((derived if (route == "pf" and w[0] in DERIVED) else rows)).append(((w[0], w[1], w[2], w[3], w[4]), _fq(w[5])))
    for w in c["maxima"]:
        ok = (type(w) is list and len(w) == 3 and type(w[0]) is str and w[0] in facts.pipes and _decimal_text(w[2])
              and (w[1] in SECTION_BASES if route == "exact" else w[1] is None))
        if not ok:
            out.append(f"case {cid}: a reference maximum {str(w)[:120]}")
            continue
        maxima.append(((w[0], w[1]), _fq(w[2])))
    if any(len({k for k, _ in xs}) != len(xs) for xs in (rows, derived, maxima)):
        out.append(f"case {cid}: duplicate reference keys")
    return rows, derived, maxima, out


def _expected(ref, facts, route, variant):
    """(Expected or None, problems) of a ref: only the members the judge reads, each of an exact type."""
    if type(ref) is not dict or type(ref.get("sections")) is not dict or type(ref.get("cases")) is not dict:
        return None, ["ref"]
    out = []
    secs = ref["sections"]
    if sorted(secs) != sorted(facts.pipes):
        return None, ["ref sections differ from the request's pipes"]
    if sorted(ref["cases"]) != sorted(facts.cases):
        return None, ["ref cases differ from the request's load cases"]
    sections = []
    for pid in sorted(secs):
        s = secs[pid]
        if type(s) is not dict or not _decimal_text(s.get("ro")):
            out.append(f"ref section {pid}")
            continue
        if route == "pf":
            if not (all(_decimal_text(s.get(k)) for k in PF_EXACT) and all(_float_text(s.get(k)) for k in PF_TAU_FLOATS)):
                out.append(f"ref section {pid}")
                continue
            sections.append(PipeSection(pipe=pid, ro=_fq(s["ro"]), bases=((None, SectionProps(A=_fq(s["A"]), Z=_fq(s["Z"]), J=_fq(s["J"]))),),
                                        tau=TauInputs(E=float(s["E"]), G=float(s["G"]), od=float(s["od"]), wall=float(s["wall"]),
                                                      mill=float(s["mill"]), I=_fq(s["I"]), Z=_fq(s["Z"]), tau_c=_fq(s["tau_c"]))))
        else:
            if not all(type(s.get(b)) is dict and all(_decimal_text(s[b].get(k)) for k in ("A", "Z", "J")) for b in SECTION_BASES):
                out.append(f"ref section {pid}")
                continue
            sections.append(PipeSection(pipe=pid, ro=_fq(s["ro"]), tau=None,
                                        bases=tuple((b, SectionProps(A=_fq(s[b]["A"]), Z=_fq(s[b]["Z"]), J=_fq(s[b]["J"]))) for b in SECTION_BASES)))
    cases = []
    for cid in facts.cases:
        rows, derived, maxima, probs = _ref_rows(ref["cases"][cid], facts, route, cid)
        out += probs
        if not probs:
            cases.append(ExpectedCase(case=cid, rows=tuple(rows), maxima=tuple(maxima), derived=tuple(derived)))
    expect = refusal = None
    warnings = frozenset()
    if route == "pf":
        expect = ref.get("expect")
        if expect not in ("rows", "refused") or type(expect) is not str:
            out.append("ref expect")
        elif expect == "refused":
            r = ref.get("refusal")
            if type(r) is not dict or type(r.get("code")) is not str or r["code"] not in REFUSALS["pf"]:
                out.append("ref refusal")
            else:
                refusal = r["code"]
        w = ref.get("expected_warnings")
        if type(w) is not list or not all(type(x) is str for x in w):
            out.append("ref expected_warnings")
        else:
            warnings = frozenset(w)
    if out:
        return None, out
    return Expected(variant=variant, sections=tuple(sections), cases=tuple(cases), bases=(None,) if route == "pf" else SECTION_BASES,
                    single_headline=route == "pf", expect=expect, refusal=refusal, warnings=warnings, derived_judged=route == "pf"), []


class Model:
    """A validated reference line: the view of its model, the validator's facts and its expected reference."""

    def __init__(self, view, facts, expected):
        self.view, self.facts, self.expected = view, facts, expected


def reference_views(lines, route, variant="own", controls=None):
    """({tag: Model}, problems): every reference line admitted (request and read members), with the expected
    reference of the variant (own, twin or, on the exact route, control: the controls file's pertE). Every control
    entry is admitted too: one per tag, a tag of the set, a well-formed pertE. With any problem nothing is judged."""
    if not lines:
        return {}, ["an empty reference set"]
    out = []
    tags = [x.get("tag") if type(x) is dict else None for x in lines]
    out += [f"duplicate reference tag {t}" for t in sorted({t for t in tags if tags.count(t) > 1}, key=str)]
    by_tag = {x["tag"]: x for x in lines if type(x) is dict and type(x.get("tag")) is str}
    ctl = {}
    if variant == "control":
        if route != "exact" or controls is None:
            return {}, ["the control variant needs the exact route and a controls file"]
        if not controls:
            return {}, ["controls: empty"]
        for x in controls:
            if type(x) is not dict or set(x) != {"tag", "pertE"} or type(x["tag"]) is not str:
                out.append("controls: an entry's keys")
            elif x["tag"] in ctl:
                out.append(f"controls: duplicate tag {x['tag']}")
            elif x["tag"] not in by_tag:
                out.append(f"controls: {x['tag']} is not a model of the set")
            else:
                ctl[x["tag"]] = x["pertE"]
    models = {}
    for x in lines:
        t = x.get("tag") if type(x) is dict else None
        try:
            m, probs = _reference_model(x, route, variant, by_tag, ctl)
        except Exception as e:  # never a crash
            m, probs = None, [f"unreadable ({type(e).__name__})"]
        out += [f"{t}: {p}" for p in probs]
        if m is not None and m.expected is not None:
            models[t] = m
    return ({} if out else models), out


def _reference_model(x, route, variant, by_tag, ctl):
    if type(x) is not dict or type(x.get("tag")) is not str or type(x.get("meta")) is not dict or "ref" not in x or "request" not in x:
        return None, ["line keys"]
    m = x["meta"]
    if not all(type(m.get(k)) is str for k in REPORT_META):
        return None, ["meta (" + ", ".join(REPORT_META) + " must be strings)"]
    facts, Lb, bodies, components, out = request_model(x["request"])
    if facts is None:
        return None, out
    own, probs = _expected(x["ref"], facts, route, "own")       # the own reference is admitted whatever the variant
    out += probs
    expected = own
    if variant == "twin":
        if m["twin"] not in by_tag:
            return None, out + [f"twin {m['twin']} absent"]
        expected, probs = _expected(by_tag[m["twin"]].get("ref"), facts, route, "twin")
        out += [f"twin reference: {p}" for p in probs]
    elif variant == "control":
        expected = None
        if x["tag"] in ctl:
            expected, probs = _expected(ctl[x["tag"]], facts, route, "control")
            out += [f"control reference: {p}" for p in probs]
    harness = None
    if components:
        harness = "components are not modelled by this judge"
    elif bodies != 1:
        harness = f"a model with {bodies} bodies: per-body scales are not implemented"
    view = ModelView(route=route, tag=x["tag"], family=m["family"], theory=m["theory"], twin=m["twin"], cases=facts.cases, Lb=float(Lb),
                     request_sha256=request_sha256(x["tag"], x["request"]), harness_error=harness, meta=tuple((k, m[k]) for k in REPORT_META))
    return Model(view, facts, expected), out


# ----------------------------------------------------------------- reference files (provenance)
def file_sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


REFERENCE_SUMS = os.path.join(HERE, "REFERENCE_SHA256SUMS")


def pinned_references(path=REFERENCE_SUMS):
    """({(route, kind): {sha256: name}}, problems) of the committed reference digests (Agent 0, round 7): the manifest
    is read from HEAD's blob (`git cat-file blob HEAD:<path>`, git show HEAD:<path> without text conversion) of the
    checkout it sits in, and the working file must exist and be byte-identical to that blob. No index, status or diff
    is consulted, so no index flag (assume-unchanged, skip-worktree) and no staged or unstaged edit can stand in for
    the commit. Lines are `<sha256> <route> <set|controls> <name...>`."""
    out = []
    try:
        real = os.path.realpath(path)
        d = os.path.dirname(real)
        top = subprocess.run(["git", "-C", d, "rev-parse", "--show-toplevel"], capture_output=True, text=True, timeout=60)
        if top.returncode != 0:
            return {}, [f"{path} is not in a git checkout"]
        rel = os.path.relpath(real, os.path.realpath(top.stdout.strip())).replace(os.sep, "/")
        blob = subprocess.run(["git", "-C", top.stdout.strip(), "cat-file", "blob", "HEAD:" + rel], capture_output=True, timeout=60)
    except Exception as e:
        return {}, [f"git failed on {path} ({type(e).__name__})"]
    if blob.returncode != 0:
        return {}, [f"{path} is not in HEAD (no blob HEAD:{rel})"]
    try:
        with open(real, "rb") as f:
            work = f.read()
    except OSError as e:
        return {}, [f"{path} is missing from the working tree ({type(e).__name__})"]
    if work != blob.stdout:
        return {}, [f"{path} differs from HEAD's blob HEAD:{rel}"]
    try:
        text = blob.stdout.decode("utf-8")
    except UnicodeDecodeError:
        return {}, [f"{path}: HEAD's blob is not UTF-8"]
    pins = collections.defaultdict(dict)
    for i, line in enumerate(text.splitlines()):
        if not line.strip() or line.startswith("#"):
            continue
        parts = line.split(None, 3)
        if len(parts) != 4 or not HEX64.match(parts[0]) or parts[1] not in JUDGED or parts[2] not in ("set", "controls"):
            out.append(f"{path}:{i + 1}: malformed")
            continue
        pins[(parts[1], parts[2])][parts[0]] = parts[3]
    return dict(pins), out


def reference_file_problems(path, route, kind, pins):
    """The sha256 of a reference file must be pinned for the route and kind."""
    try:
        h = file_sha256(path)
    except Exception as e:
        return [f"{kind} file {path} unreadable ({type(e).__name__})"]
    if h not in pins.get((route, kind), {}):
        return [f"{kind} file {path} (sha256 {h[:16]}...) is not a committed {route} reference (REFERENCE_SHA256SUMS)"]
    return []
