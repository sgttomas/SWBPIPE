"""Two-way benchmark comparison: SWBPIPE's own results against CAEPIPE's printed results (DEL-17-05).

`build_benchmark_record` compares, for every load case the MBF writer mapped and for the operating case, every
displacement, rotation, element end force and moment, and support load SWBPIPE solved with the value CAEPIPE
printed, bound through the writer's id maps (nodes, members by `bind_element_rows`, supports by node). No analytic
value is needed. Each scalar is screened against the screening profile of the "Benchmark method" section of
docs/RELEASE_TARGET_v1.0.md and classified as `within_profile`, `finding`, `unmatched` or `status_mismatch`. A
closed-form value the benchmark model carries is kept as the third value with its own checks.

A case whose SWBPIPE side the solver publishes with a numerical quality other than checks passed is not screened
(`SENSITIVE_SIDE_RULE`): its scalars carry `sensitive_side` with SWBPIPE's own values and closed-form checks only, the
target program's results for it are not read, and the case is one finding with the solver's status, codes and cause.
The quality is read from SWBPIPE's result envelope alone; a compared case without it raises NumericalQualityError.

The record is built from CAEPIPE output and is private (docs/IP_AND_DATA_BOUNDARY.md §6.2): it stays in the exchange
folder. It is screening evidence for Agent 0 and the owner's review of the full record, not validation,
compatibility proof, code compliance or professional acceptance. The screening profile is a proposal under that
Benchmark method; the owner's review of the full record decides, not the profile.
"""

from __future__ import annotations

import hashlib
import math
from typing import Any, Mapping

from .correlation import canonical_model_hash
from .print_csv import bind_element_rows


BENCHMARK_RECORD_KIND = "openpipestress.caepipe_benchmark.comparison_record"
BENCHMARK_RECORD_VERSION = "2"
EVIDENCE_CLASSIFICATION = ("private screening evidence for review; not validation, compatibility proof, code "
                           "compliance or professional acceptance")

WITHIN = "within_profile"
FINDING = "finding"
UNMATCHED = "unmatched"
STATUS_MISMATCH = "status_mismatch"
# SWBPIPE itself publishes the case's numerical quality as other than checks passed: the case is not screened.
SENSITIVE_SIDE = "sensitive_side"
CLASSIFICATIONS = (WITHIN, FINDING, UNMATCHED, STATUS_MISMATCH, SENSITIVE_SIDE)
CHECKS_PASSED = "checks_passed"
SENSITIVE_SIDE_RULE = (
    "A case whose SWBPIPE side the solver publishes with a numerical quality other than checks_passed is not "
    "screened: each of its scalars is classed sensitive_side, the case is one finding with the integrity status and "
    "the cause the solver names, and its comparison waits for the deliverable that owns that cause. Which cases these "
    "are is read from SWBPIPE's own result alone. The rule runs both ways: such a case is sensitive_side whatever its "
    "values, and while it is so published no value of the target program and no difference is formed, recorded or "
    "shown for it.")
# The longest verbatim excerpt of the solver's cause sentence a finding carries.
CAUSE_SENTENCE_LIMIT = 600
_INTEGRITY_PREFIX = "NUMERICAL_INTEGRITY_"
_INTEGRITY_PASSED = "NUMERICAL_INTEGRITY_CHECKS_PASSED"


class NumericalQualityError(ValueError):
    """SWBPIPE's result does not state, consistently, the numerical quality of a case it solved and that is compared.
    Raised by name; never read as a pass."""

    def __init__(self, code: str, message: str) -> None:
        super().__init__(f"{code}: {message}")
        self.code = code


QUALITY_MISSING = "BENCHMARK_NUMERICAL_QUALITY_MISSING"
QUALITY_INCONSISTENT = "BENCHMARK_NUMERICAL_QUALITY_INCONSISTENT"

EXACT = "exact"
NAMED_INEXACT = "named_inexact"
# The screening profile of the "Benchmark method" section of docs/RELEASE_TARGET_v1.0.md: relative tolerance by
# class, plus one printed unit of each result kind as the floor, with scalars below eta of their kind's case maximum
# judged against that scale.
SCREENING_PROFILE = {
    "basis": ("screening profile of the Benchmark method section of docs/RELEASE_TARGET_v1.0.md, proposed by the "
              "validation lane; the owner's review of the full record decides, not the profile; values are only "
              "tightened"),
    "relative": {EXACT: 1.0e-4, NAMED_INEXACT: 5.0e-3},
    "floor": {"m": 1.0e-6, "rad": math.radians(1.0e-4), "N": 1.0, "N*m": 1.0},
    "floor_basis": ("one printed unit of the CSV per kind: 0.001 mm, 0.0001 deg, 1 N, 1 N*m; the printed resolution "
                    "of the result file, observed format behaviour"),
    "near_zero_fraction": 1.0e-3,
    "rule": "|S - C| <= floor_k + r * max(|S|, |C|, near_zero_fraction * Q_k); Q_k the case maximum of kind k",
}
# The closed form against SWBPIPE: both binary64 Euler-Bernoulli values (the pilot's analytic_vs_swbpipe scale).
CLOSED_FORM_TOLERANCE = {"rel": 1.0e-6, "abs": {"m": 1.0e-9, "rad": 1.0e-9, "N": 1.0e-6, "N*m": 1.0e-6}}

# The writer's labels and the operating case the target program forms from them (observed format behaviour).
OPERATING_LABEL = "Operating (W+P1+T1)"
OPERATING = "operating"

_TO_SI = {"m": 1.0, "mm": 1.0e-3, "rad": 1.0, "N": 1.0, "kN": 1.0e3, "N*m": 1.0, "kN*m": 1.0e3}
_AXES = "xyz"
_SWBPIPE_NODAL = {f"global_nodal_displacement_{axis}": ("displacement", axis, "m") for axis in _AXES} | {
    f"global_nodal_rotation_{axis}": ("rotation", axis, "rad") for axis in _AXES}
_SWBPIPE_END = {"element_global_end_force_v1": ("element_force", "N"),
                "element_global_end_moment_v1": ("element_moment", "N*m")}
_SWBPIPE_SUPPORT = "support_reaction_component_v2"
_UNIT = {"displacement": "m", "rotation": "rad", "element_force": "N", "element_moment": "N*m",
         "support_force": "N", "support_moment": "N*m"}


def _si(value: Any, unit: Any) -> float | None:
    if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value):
        return None
    if unit == "deg":
        return math.radians(value)
    factor = _TO_SI.get(unit)
    converted = None if factor is None else value * factor
    return converted if converted is not None and math.isfinite(converted) else None


def screening_class(inexact_inputs: list[str] | None) -> str:
    """`exact` when every input is held exactly, else `named_inexact`."""
    return NAMED_INEXACT if inexact_inputs else EXACT


def screen(swbpipe: float, caepipe: float, unit: str, cls: str, case_maximum: float) -> dict[str, Any]:
    """One scalar against the screening profile."""
    rel = SCREENING_PROFILE["relative"][cls]
    floor = SCREENING_PROFILE["floor"][unit]
    scale = max(abs(swbpipe), abs(caepipe), SCREENING_PROFILE["near_zero_fraction"] * case_maximum)
    allowed = floor + rel * scale
    difference = caepipe - swbpipe
    finite = all(math.isfinite(value) for value in (swbpipe, caepipe, case_maximum, difference, allowed))
    return {"difference": difference, "allowed": allowed, "relative": rel, "floor": floor,
            "within": finite and abs(difference) <= allowed}


def _close(a: float, b: float, unit: str) -> bool:
    return abs(a - b) <= CLOSED_FORM_TOLERANCE["abs"][unit] + CLOSED_FORM_TOLERANCE["rel"] * max(abs(a), abs(b))


# -- SWBPIPE's own numerical quality ---------------------------------------------------------------------------------
def numerical_quality(envelope: Mapping[str, Any], case_ids: list[str]) -> dict[str, str]:
    """{case: solve_quality} for every named case, read from SWBPIPE's result envelope alone (its
    numerical_quality.cases entry for the case), cross-checked against the envelope's own status. A missing entry, a
    non-string quality, or a case that is not checks_passed under an envelope status of checks_passed raises
    NumericalQualityError by name; nothing else sets or overrides a case's quality."""
    if not case_ids:
        return {}  # SWBPIPE solved no compared case: nothing of its values is compared
    quality = envelope.get("numerical_quality")
    if not isinstance(quality, Mapping):
        raise NumericalQualityError(QUALITY_MISSING, "the SWBPIPE result has no numerical_quality")
    entries: dict[str, list[Any]] = {}
    for entry in quality.get("cases") or []:
        if isinstance(entry, Mapping):
            entries.setdefault(str((entry.get("basis_ref") or {}).get("ref_id")), []).append(entry.get("solve_quality"))
    out = {}
    for case in case_ids:
        found = entries.get(case, [])
        if len(found) != 1 or not isinstance(found[0], str) or not found[0]:
            raise NumericalQualityError(
                QUALITY_MISSING, f"the SWBPIPE result states no single numerical quality for load case {case}")
        out[case] = found[0]
    if quality.get("status") == CHECKS_PASSED and any(value != CHECKS_PASSED for value in out.values()):
        raise NumericalQualityError(QUALITY_INCONSISTENT, "the SWBPIPE result's numerical quality status is "
                                    "checks_passed but a compared case is not")
    return out


def _cause_sentence(message: str, case: str) -> str | None:
    """The solver's own sentence that names why `case` is not checks passed, verbatim and bounded in length."""
    for marker in (f"load case {case} is Sensitive", "is Sensitive because", "is sensitive because"):
        at = message.find(marker)
        if at >= 0:
            guard = message.rfind("S11-G", 0, at)  # the guard sentence opens with the guard's name
            start = guard if guard >= 0 else message.rfind(". ", 0, at) + 2 if ". " in message[:at] else 0
            return message[start:start + CAUSE_SENTENCE_LIMIT]
    return None


def sensitive_finding(envelope: Mapping[str, Any], case: str, solve_quality: str) -> dict[str, Any]:
    """The one finding of a case SWBPIPE publishes as other than checks passed: the status, the integrity diagnostic
    codes of the case, and the solver's sentence naming the cause. SWBPIPE's own output only."""
    codes, cause = [], None
    for item in envelope.get("diagnostics") or []:
        if not isinstance(item, Mapping):
            continue
        code = str(item.get("code") or "")
        message = str(item.get("message") or "")
        refs = [str(ref) for key in ("affected_refs", "refs", "references", "affected_objects")
                for ref in (item.get(key) or []) if isinstance(item.get(key), list)]
        mine = (case in refs or str(item.get("id") or "").endswith(f":{case}")
                or f"load case {case}:" in message or f"load case {case} " in message)
        if code.startswith(_INTEGRITY_PREFIX) and code != _INTEGRITY_PASSED and mine:
            codes.append(code)
            cause = cause or _cause_sentence(message, case)
    return {"case": case, "classification": SENSITIVE_SIDE, "solve_quality": solve_quality,
            "numerical_quality_status": (envelope.get("numerical_quality") or {}).get("status"),
            "diagnostic_codes": sorted(set(codes)),
            "cause": cause if cause is not None else "the solver names no cause sentence for this case",
            "comparison": "not formed; waits for the answer of the deliverable that owns the cause"}


# -- extraction ------------------------------------------------------------------------------------------------------
Key = tuple[str, str, str, str]  # (kind, entity, end or "", component)


def swbpipe_values(results: list[Any], case_ids: list[str], support_nodes: Mapping[str, str],
                   anchor_nodes: set[str] | frozenset[str] = frozenset()) -> tuple[dict[Key, float], set[Key]]:
    """SWBPIPE's values of the named cases, summed over the cases (one case, or the operating sum), in SI, and the
    keys that cannot be screened: a contribution that is missing from one of the cases, not a finite number in a
    known unit, or a sum that is not finite. Those keys carry no value and are classed unmatched, never screened on
    a partial sum.

    Support reactions are summed per CAEPIPE node (`support_nodes`: support id -> node label) because the target
    program prints one row per node; displacements are keyed by SWBPIPE node, element ends by member and end. The
    moment components of supports at a node that is not written as an anchor (`anchor_nodes`) are left out: the
    printed restraint rows give forces only."""
    # Each contribution keeps its own identity (a support's reaction its support id) until it is complete in every
    # case; only then are contributions summed into the compared key.
    Source = tuple[Key, str]
    per_case: dict[str, dict[Source, list[float]]] = {case: {} for case in case_ids}
    invalid: set[Key] = set()

    def add(case: str, key: Key, value: float | None, source: str = "") -> None:
        if value is None:
            invalid.add(key)
        else:
            per_case[case].setdefault((key, source), []).append(value)

    for row in results:
        if not isinstance(row, Mapping) or (row.get("basis_ref") or {}).get("ref_id") not in per_case:
            continue
        case = (row.get("basis_ref") or {}).get("ref_id")
        kind = row.get("kind")
        value = _si(row.get("value"), row.get("unit"))
        if kind in _SWBPIPE_NODAL:
            name, axis, _ = _SWBPIPE_NODAL[kind]
            add(case, (name, str(row.get("entity_ref")), "", axis), value)
        elif kind in _SWBPIPE_END:
            location = (row.get("metadata") or {}).get("location")
            component = str((row.get("metadata") or {}).get("component", "")).lower()
            if location in ("end_i", "end_j") and len(component) == 2 and component[1] in _AXES:
                add(case, (_SWBPIPE_END[kind][0], str(row.get("entity_ref")), location, component[1]), value)
        elif kind == _SWBPIPE_SUPPORT:
            component = str((row.get("metadata") or {}).get("component", ""))
            node = support_nodes.get(str(row.get("entity_ref")))
            if node is not None and len(component) == 2 and component[0] in "FM" and component[1].lower() in _AXES:
                name = "support_force" if component[0] == "F" else "support_moment"
                if name == "support_moment" and node not in anchor_nodes:
                    continue
                add(case, (name, f"node:{node}", "", component[1].lower()), value, str(row.get("entity_ref")))
    sources = set().union(*per_case.values())
    for key, source in sources:
        if not all((key, source) in case_values for case_values in per_case.values()):
            invalid.add(key)  # a contribution missing from one of the summed cases
    grouped: dict[Key, list[float]] = {}
    for case_values in per_case.values():
        for (key, _), items in case_values.items():
            if key not in invalid:
                grouped.setdefault(key, []).extend(items)
    values: dict[Key, float] = {}
    for key, parts in grouped.items():
        try:
            total = math.fsum(parts) if all(math.isfinite(part) for part in parts) else math.nan
        except (OverflowError, ValueError):
            total = math.nan
        if math.isfinite(total):
            values[key] = total
        else:
            invalid.add(key)
    return values, invalid


def caepipe_values(tables: Mapping[str, Any], units: Mapping[str, Any], mbf: Mapping[str, Any]
                   ) -> tuple[dict[Key, float], list[dict[str, Any]], dict[str, Any]]:
    """CAEPIPE's values of one load case bound to SWBPIPE ids, the rows that did not bind, and the element binding."""
    values: dict[Key, float] = {}
    unbound: list[dict[str, Any]] = []
    by_label: dict[str, list[str]] = {}
    for node, label in (mbf.get("node_map") or {}).items():
        by_label.setdefault(str(label), []).append(node)
    length, rotation = units.get("length"), units.get("rotation")
    force, moment = units.get("force"), units.get("moment")
    for label, row in (tables.get("displacements") or {}).items():
        nodes = by_label.get(str(label), [])
        if len(nodes) != 1:
            unbound.append({"section": "displacements", "caepipe_node": label,
                            "reason": "no SWBPIPE node maps to this label" if not nodes else "several nodes map to it"})
            continue
        for axis in _AXES:
            add_value(values, ("displacement", nodes[0], "", axis), _si(row.get(f"u{axis}"), length), 1.0)
            add_value(values, ("rotation", nodes[0], "", axis), _si(row.get(f"r{axis}"), rotation), 1.0)
    binding = bind_element_rows(list(tables.get("element_forces_global") or []), dict(mbf.get("element_map") or {}))
    for member, ends in binding["bound"].items():
        for end in ("end_i", "end_j"):
            row = ends[end]["values"] or {}
            for axis in _AXES:
                add_value(values, ("element_force", member, end, axis), _si(row.get(f"f{axis}"), force), 1.0)
                add_value(values, ("element_moment", member, end, axis), _si(row.get(f"m{axis}"), moment), 1.0)
    for item in binding["unmatched_rows"]:
        unbound.append({"section": "element_forces_global", **item})
    support_nodes = {str(entry.get("node")): entry.get("caepipe_kind")
                     for entry in (mbf.get("support_map") or {}).values() if isinstance(entry, Mapping)}
    for section in ("anchor_loads_global", "restraint_loads_global"):
        for label, row in (tables.get(section) or {}).items():
            if str(label) not in support_nodes:
                unbound.append({"section": section, "caepipe_node": label,
                                "reason": "no SWBPIPE support is written at this node"})
                continue
            # The printed load is the pipe's load on the support: sign-flipped to the support's action on the pipe.
            for axis in _AXES:
                add_value(values, ("support_force", f"node:{label}", "", axis), _si(row.get(f"f{axis}"), force), -1.0)
                if section == "anchor_loads_global":
                    add_value(values, ("support_moment", f"node:{label}", "", axis), _si(row.get(f"m{axis}"), moment),
                              -1.0)
    return values, unbound, binding


def add_value(values: dict[Key, float], key: Key, value: float | None, sign: float) -> None:
    if value is not None:
        total = values.get(key, 0.0) + sign * value
        if math.isfinite(total):
            values[key] = total
        else:
            values.pop(key, None)  # not screenable: the key is then classed unmatched


def _expected_keys(mbf: Mapping[str, Any]) -> set[Key]:
    """Every key the comparison expects on both sides: written nodes, members' ends, and support nodes (restraint
    nodes carry forces only, since the printed restraint rows give no moments)."""
    keys: set[Key] = set()
    for node in mbf.get("node_map") or {}:
        for axis in _AXES:
            keys |= {("displacement", node, "", axis), ("rotation", node, "", axis)}
    for member in mbf.get("element_map") or {}:
        for end in ("end_i", "end_j"):
            for axis in _AXES:
                keys |= {("element_force", member, end, axis), ("element_moment", member, end, axis)}
    for entry in (mbf.get("support_map") or {}).values():
        node = f"node:{entry.get('node')}"
        for axis in _AXES:
            keys.add(("support_force", node, "", axis))
            if entry.get("caepipe_kind") == "anchor":
                keys.add(("support_moment", node, "", axis))
    return keys


# -- closed forms ----------------------------------------------------------------------------------------------------
def _closed_form_key(entry: Mapping[str, Any], mbf: Mapping[str, Any]) -> Key | None:
    quantity, axis = entry.get("quantity"), entry.get("component")
    if quantity in ("displacement", "rotation"):
        return (quantity, str(entry.get("node")), "", str(axis))
    support = (mbf.get("support_map") or {}).get(entry.get("support"))
    if not isinstance(support, Mapping):
        return None
    sharing = [other for other in (mbf.get("support_map") or {}).values()
               if isinstance(other, Mapping) and other.get("node") == support.get("node")]
    if len(sharing) != 1:
        return None  # a node total is not one support's closed form
    name = {"reaction_force": "support_force", "reaction_moment": "support_moment"}.get(quantity)
    return None if name is None else (name, f"node:{support.get('node')}", "", str(axis))


# -- record ----------------------------------------------------------------------------------------------------------
def build_benchmark_record(benchmark_model: Mapping[str, Any], *, swbpipe_result_payload: Mapping[str, Any],
                           mbf_result: Any, mbf_text: str, parsed_caepipe: Mapping[str, Any] | None) -> dict[str, Any]:
    """The two-way comparison record of one benchmark model (private)."""
    mbf = mbf_result.to_dict() if hasattr(mbf_result, "to_dict") else dict(mbf_result)
    envelope = swbpipe_result_payload.get("mechanics_envelope", swbpipe_result_payload)
    results = list(envelope.get("results") or [])
    parsed_cases = (parsed_caepipe or {}).get("load_cases") or {}
    units = (parsed_caepipe or {}).get("units") or {}
    header = (parsed_caepipe or {}).get("header") or {}
    inexact = benchmark_model.get("inexact_inputs") or {}
    support_nodes = {support: str(entry.get("node")) for support, entry in (mbf.get("support_map") or {}).items()
                     if isinstance(entry, Mapping)}
    anchor_nodes = {str(entry.get("node")) for entry in (mbf.get("support_map") or {}).values()
                    if isinstance(entry, Mapping) and entry.get("caepipe_kind") == "anchor"}
    case_map = dict(mbf.get("load_case_map") or {})
    compared: list[tuple[str, list[str], str]] = [(case, [case], label) for case, label in sorted(case_map.items())]
    if case_map:
        compared.append((OPERATING, sorted(case_map), OPERATING_LABEL))
    solved = {(row.get("basis_ref") or {}).get("ref_id") for row in results if isinstance(row, Mapping)}
    closed_forms = [entry for entry in benchmark_model.get("closed_form") or [] if isinstance(entry, Mapping)]
    expected = _expected_keys(mbf)
    # SWBPIPE's own numerical quality of every mapped case it solved: read from its result alone, before anything of
    # the target program is read.
    quality = numerical_quality(envelope, sorted(case for case in case_map if case in solved))
    sensitive = sorted(case for case, value in quality.items() if value != CHECKS_PASSED)
    findings = [sensitive_finding(envelope, case, quality[case]) for case in sensitive]

    cases_out, rows, unbound_rows, bindings = [], [], [], {}
    for case_name, swbpipe_cases, label in compared:
        inputs = list(inexact.get(case_name) or [])
        cls = screening_class(inputs)
        withheld = sorted(set(swbpipe_cases) & set(sensitive))
        if withheld:
            # Not screened: SWBPIPE's own values and closed-form checks only; the target program's results for this
            # case are not read, and no value, difference or printed status of it is recorded.
            cases_out.append({"case": case_name, "swbpipe_cases": swbpipe_cases, "caepipe_load_case": label,
                              "screening_class": cls, "inexact_inputs": inputs, "swbpipe_solved": True,
                              "sensitive_side": {case: quality[case] for case in withheld}})
            swb, swb_invalid = swbpipe_values(results, swbpipe_cases, support_nodes, anchor_nodes)
            for key in sorted(expected | set(swb) | swb_invalid):
                kind, entity, end, axis = key
                unit = _UNIT[kind]
                row = {"case": case_name, "kind": kind, "entity": entity, "component": axis, "unit": unit,
                       **({"end": end} if end else {}), "values": {"swbpipe": swb.get(key)},
                       "classification": SENSITIVE_SIDE,
                       "reason": (f"SWBPIPE publishes the numerical quality of {', '.join(withheld)} as "
                                  f"{', '.join(sorted({quality[case] for case in withheld}))}; not screened")}
                for entry in closed_forms:
                    if entry.get("load_case") == case_name and _closed_form_key(entry, mbf) == key:
                        value = _si(entry.get("value"), entry.get("unit"))
                        if value is not None:
                            row["values"]["closed_form"] = value
                            row["closed_form"] = {"formula": entry.get("formula"), "basis": entry.get("basis")}
                            if key in swb:
                                row["closed_form"]["closed_form_vs_swbpipe"] = (
                                    "agree" if _close(value, swb[key], unit) else "disagree")
                rows.append(row)
            continue
        status = {"case": case_name, "swbpipe_cases": swbpipe_cases, "caepipe_load_case": label,
                  "screening_class": cls, "inexact_inputs": inputs,
                  "swbpipe_solved": all(case in solved for case in swbpipe_cases),
                  "caepipe_printed": isinstance(parsed_cases.get(label), Mapping)}
        if case_name == OPERATING:
            status["basis"] = ("SWBPIPE: the sum of its mapped primitive cases (linear models, superposition); "
                               f"CAEPIPE: its {OPERATING_LABEL!r} case")
        cases_out.append(status)
        if not (status["swbpipe_solved"] and status["caepipe_printed"]):
            reason = ("SWBPIPE solved the case but CAEPIPE printed no such case" if status["swbpipe_solved"] else
                      "CAEPIPE printed the case but SWBPIPE did not solve all of it" if status["caepipe_printed"] else
                      "neither side has the case")
            rows.append({"case": case_name, "kind": "load_case", "entity": label, "classification": STATUS_MISMATCH,
                         "reason": reason})
            continue
        swb, swb_invalid = swbpipe_values(results, swbpipe_cases, support_nodes, anchor_nodes)
        cae, unbound, binding = caepipe_values(parsed_cases[label], units, mbf)
        bindings[case_name] = {key: binding[key] for key in ("basis", "unmatched_rows", "unmatched_members", "notes")}
        unbound_rows.extend({"case": case_name, **item} for item in unbound)
        maxima: dict[str, float] = {}
        for key, value in [*swb.items(), *cae.items()]:
            unit = _UNIT[key[0]]
            maxima[unit] = max(maxima.get(unit, 0.0), abs(value))
        case_closed = {}
        for entry in closed_forms:
            if entry.get("load_case") == case_name:
                key = _closed_form_key(entry, mbf)
                value = _si(entry.get("value"), entry.get("unit"))
                if key is not None and value is not None:
                    case_closed[key] = (value, entry)
        for key in sorted(expected | set(swb) | set(cae) | swb_invalid):
            kind, entity, end, axis = key
            unit = _UNIT[kind]
            row: dict[str, Any] = {"case": case_name, "kind": kind, "entity": entity, "component": axis, "unit": unit,
                                   **({"end": end} if end else {}),
                                   "values": {"swbpipe": swb.get(key), "caepipe": cae.get(key)}}
            if key in swb and key in cae:
                check = screen(swb[key], cae[key], unit, cls, maxima.get(unit, 0.0))
                row["screen"] = {name: check[name] for name in ("difference", "allowed", "relative", "floor")}
                row["classification"] = WITHIN if check["within"] else FINDING
            else:
                row["classification"] = UNMATCHED
                row["reason"] = ("a SWBPIPE contribution is missing from a case or not a finite number"
                                 if key in swb_invalid else "only SWBPIPE has this value" if key in swb else
                                 "only CAEPIPE has this value" if key in cae else "neither side has this value")
            if key in case_closed:
                value, entry = case_closed[key]
                row["values"]["closed_form"] = value
                closed: dict[str, Any] = {"formula": entry.get("formula"), "basis": entry.get("basis")}
                if key in swb:
                    closed["closed_form_vs_swbpipe"] = "agree" if _close(value, swb[key], unit) else "disagree"
                if key in cae:
                    within = screen(value, cae[key], unit, cls, maxima.get(unit, 0.0))["within"]
                    closed["closed_form_vs_caepipe"] = "agree" if within else "disagree"
                row["closed_form"] = closed
                if closed.get("closed_form_vs_swbpipe") == "disagree" and row["classification"] == WITHIN:
                    row["classification"] = FINDING
                    row["reason"] = "SWBPIPE differs from the model's closed form"
            rows.append(row)
    # A parser error (a refused section or row) is never silent: one unmatched row each, so the counts show it.
    withheld_labels = {label: case_name for case_name, swbpipe_cases, label in compared
                       if set(swbpipe_cases) & set(sensitive)}
    for item in (parsed_caepipe or {}).get("diagnostics") or []:
        if isinstance(item, Mapping) and item.get("severity") == "error":
            owner = next((case for label, case in withheld_labels.items() if label in str(item.get("section") or "")),
                         None)
            if owner is not None:
                # A parser error in a section of a case that is not screened: counted, its text not carried.
                rows.append({"case": owner, "kind": "parser_error", "entity": "withheld",
                             "classification": SENSITIVE_SIDE, "code": item.get("code"),
                             "reason": "a parser error in a section of a case that is not screened"})
                continue
            rows.append({"case": "parser", "kind": "parser_error", "entity": str(item.get("section")),
                         "classification": UNMATCHED, "code": item.get("code"), "line": item.get("line"),
                         "reason": item.get("message")})
    for item in unbound_rows:
        rows.append({"case": item["case"], "kind": "unbound_row", "entity": str(item.get("caepipe_node") or
                     f"{item.get('node_from')}-{item.get('node_to')}"), "classification": UNMATCHED,
                     "section": item.get("section"), "reason": item.get("reason")})

    summary: dict[str, Any] = {"results": len(rows), "by_classification": {}, "by_case": {}}
    for row in rows:
        summary["by_classification"][row["classification"]] = summary["by_classification"].get(
            row["classification"], 0) + 1
        per_case = summary["by_case"].setdefault(row["case"], {})
        per_case[row["classification"]] = per_case.get(row["classification"], 0) + 1
    return {
        "document_kind": BENCHMARK_RECORD_KIND,
        "record_version": BENCHMARK_RECORD_VERSION,
        "model_id": benchmark_model.get("model_id"),
        "family": benchmark_model.get("family"),
        "internal_model_hash": canonical_model_hash(benchmark_model.get("model")),
        "exported_model_hash": hashlib.sha256(mbf_text.encode("utf-8")).hexdigest(),
        "piping_code": benchmark_model.get("piping_code"),
        "target_tool": "CAEPIPE",
        "target_version": header.get("version"),
        "product_label": header.get("product_label"),
        "screening_profile": SCREENING_PROFILE,
        "closed_form_tolerance": CLOSED_FORM_TOLERANCE,
        "comparison_basis": {
            "units": "SI base units: m, rad, N, N*m",
            "sign_convention": ("SWBPIPE global axes and right-hand rotations. Displacements as printed (mm and deg "
                                "converted). Element end forces and moments in global axes are the end action on "
                                "the element at its node on both sides. Support loads: the printed load of the pipe "
                                "on the support is sign-flipped to the support's action on the pipe; SWBPIPE "
                                "reactions of supports sharing a node are summed, as the node prints one row; a "
                                "restraint row prints forces only, so moments are compared at anchors alone."),
            "binding": ("nodes through node_map, element rows through element_map by their node labels "
                        "(bind_element_rows), supports through support_map by node; nothing is matched by "
                        "position"),
            "parser_version": (parsed_caepipe or {}).get("parser_version"),
            "cases_not_compared": sorted(set(parsed_cases) - {label for _, _, label in compared}),
        },
        "load_cases": cases_out,
        "element_binding": bindings,
        "results": rows,
        "summary": summary,
        "evidence_classification": EVIDENCE_CLASSIFICATION,
        # SWBPIPE's own numerical quality and the cases this record screens: present in every record.
        "numerical_quality": {"status": (envelope.get("numerical_quality") or {}).get("status"), "cases": quality,
                              "rule": SENSITIVE_SIDE_RULE},
        "screening": {
            "screened_cases": [case_name for case_name, swbpipe_cases, _ in compared
                               if not set(swbpipe_cases) & set(sensitive)
                               and not any(row["classification"] == STATUS_MISMATCH and row["case"] == case_name
                                           and row["kind"] == "load_case" for row in rows)],
            "sensitive_side_cases": [case_name for case_name, swbpipe_cases, _ in compared
                                     if set(swbpipe_cases) & set(sensitive)],
            "findings": findings,
        },
    }
