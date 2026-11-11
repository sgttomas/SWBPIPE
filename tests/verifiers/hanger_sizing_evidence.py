"""Independent test-only verifier of hanger-sizing result documents (DEL-04-11
design §8.3.7).

It establishes three standings, each passed, failed (with the first difference) or
not_checked, and states nothing about mechanics:

- `closure`: the closed schema, the variant and discriminator rules of §8.3.3 and
  §8.3.5, the cross-field invariants and the byte-equal fixed texts;
- `binding`: the inputs' hashes, for each input supplied;
- `arithmetic_consistent_with_published_positions`: the §8.3.6 arithmetic recomputed
  in binary64, in the stated order, from the published W and positions, the request
  and the library, and compared by numeric equality. A document whose positions were
  invented consistently passes it. `derivation` and `reproduction` are the Rust
  reader's alone.

When closure fails nothing else is checked. A document is admitted by the checked
profile's rules first: duplicate keys are refused by an `object_pairs_hook`, and
numbers outside `openpipestress_jcs_ijson_v1` (non-finite, or integral beyond
2^53 - 1) are refused.

Independent of the producer: it imports only the standard library and its siblings
`canonical_json` (hashing) and `units` (the library's unit conversion), and spawns no
process. Every rule, list and fixed text here is written from the design, not read
from product code.
"""
from __future__ import annotations

import json
import math
from typing import Any

from . import canonical_json, units

MAX_SAFE_INTEGER = 9_007_199_254_740_991
PARSE_DEPTH_LIMIT = 128

DOCUMENT_KIND = "openpipestress.hanger_sizing.result"
SCHEMA_VERSION = "1.0.0"
SEMANTIC_CONTRACT_ID = "openpipestress.result_semantics/hanger-sizing-1"
ROUTE = "pressure_free"
CANONICAL_PROFILE = "openpipestress_jcs_ijson_v1"

LIMITATIONS = (
    "Hanger sizing selects records of the user's own library by the user's own tests on the stated cases; it judges no hanger's suitability for a service and determines no code compliance.",
    "Sized means the sizing algorithm completed. A candidate may have no fitting record, and the verification may be conditional; both are stated in the result.",
    "Loads and positions come from derived solves of the bound solve request and inherit every premise of those solves; nonlinear supports take each case's own converged state.",
    "Hanger sizing is published on the pressure-free route only.",
)
BASIS_STATEMENT = (
    "Every position is measured from the drawn geometry, taken as the installed position at which each spring is set; F_cold is that setting and F_C the cold-condition load.",
    "Quantities are SI: forces in N, lengths in m, rates in N/m; variations are ratios.",
)
# Every refusal code the design names; its two disclosure codes and its verification warning are not refusals.
REFUSAL_CODES = frozenset("HANGER_SIZING_" + code for code in (
    "CANDIDATES_REQUIRED", "CANDIDATE_DOF_SHARED", "CANDIDATE_DOF_UNSUPPORTED", "CANDIDATE_DUPLICATE",
    "CANDIDATE_FAMILY_UNSUPPORTED", "CANDIDATE_MOVED", "CANDIDATE_REFERENCED", "CANDIDATE_SKEW_UNSUPPORTED",
    "CANDIDATE_UNKNOWN", "CASE_ROLES_OVERLAP", "CASE_UNKNOWN", "CHECK_OPERATING_CASES_REQUIRED",
    "DERIVED_DOCUMENT_NOT_HASHABLE", "DERIVED_SOLVE_FAILED", "DESIGN_OPERATING_CASE_REQUIRED", "EXACT_ROUTE_PENDING",
    "FAMILY_POLICY_REQUIRED", "ID_COLLISION", "INPUT_NOT_ADMITTED", "INPUT_SHAPE_INVALID", "INPUT_TOO_LARGE",
    "LIBRARY_NOT_ADMITTED", "LIBRARY_RECORD_DUPLICATE", "LIBRARY_REQUIRED", "LIBRARY_UNAVAILABLE",
    "LOAD_DESIGN_REQUIRED", "LOAD_NOT_POSITIVE", "REQUEST_INVALID", "RESULT_NOT_PUBLISHABLE", "SELECTION_RULE_REQUIRED",
    "SOLVER_MODE_REQUIRED", "TEST_CASE_UNSUPPORTED", "TRAVEL_ALLOWANCE_INVALID", "TRAVEL_ALLOWANCE_REQUIRED",
    "VARIATION_LIMIT_INVALID", "VARIATION_LIMIT_REQUIRED", "VARIATION_REFERENCE_REQUIRED", "WEIGHT_CASE_REQUIRED",
    "WEIGHT_SUPPORT_STATE_REQUIRED", "WEIGHT_SUPPORT_STATE_UNSUPPORTED"))
DISCLOSURE_CODES = ("HANGER_SIZING_SUPPORT_STATE_DIFFERS", "HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS")
LOAD_CATEGORIES = frozenset(("weight", "distributed_force", "pressure", "thermal", "hydrotest", "wind", "seismic",
                             "occasional", "concentrated_force", "concentrated_moment", "imposed_displacement"))
STATE_CODES = frozenset(("inactive", "active", "sticking", "sliding"))
VARIABLE_TESTS = ("E1", "E2", "V0", "L", "T", "V")
CONSTANT_TESTS = ("E1", "L", "T")
POLICIES = ("variable", "constant", "variable_else_constant")

SIZED_KEYS = ("document_kind", "schema_version", "semantic_contract_id", "route", "limitations", "basis_statement",
              "status", "inputs", "result")
REFUSED_KEYS = SIZED_KEYS[:-1] + ("refusals",)
INPUT_KEYS = ("canonical_profile", "sizing_request", "sizing_request_sha256", "solve_request", "library")
REQUEST_KEYS = ("weight_case", "weight_support_state", "design_operating_case", "check_operating_cases", "candidates",
                "method", "solver_mode")
METHOD_KEYS = ("load_design", "variation_reference", "variation_limit", "travel_allowance_m", "selection_rule")
CANDIDATE_KEYS = ("support", "node", "dof", "family_policy", "outcome", "w_n", "u_cold_m", "operating", "span_m",
                  "required_travel_m", "failure_counts", "selected", "governing", "verification_assumption",
                  "verification")


class Difference(Exception):
    """The first difference a standing found."""


# --- admission ---------------------------------------------------------------------------------

def _no_duplicates(pairs):
    seen = {}
    for key, value in pairs:
        if key in seen:
            raise Difference(f"duplicate_key: {key!r}")
        seen[key] = value
    return seen


def _not_json(name):
    raise Difference(f"invalid_json: {name} is not JSON")


def _integer(token: str) -> int:
    # Refuse a long integer token before conversion: Python's int conversion limit
    # would otherwise raise outside admission. 2^53 - 1 has 16 digits.
    if len(token.lstrip("-")) > 16:
        raise Difference("number_outside_profile: an integer token past the profile")
    return int(token)


def _profile(value: Any, path: str, depth: int) -> None:
    if isinstance(value, bool) or value is None:
        return
    if isinstance(value, int):
        if abs(value) > MAX_SAFE_INTEGER:
            raise Difference(f"{path}: number_outside_profile")
    elif isinstance(value, float):
        if not math.isfinite(value) or (value.is_integer() and abs(value) > MAX_SAFE_INTEGER):
            raise Difference(f"{path}: number_outside_profile")
    elif isinstance(value, str):
        if any(0xD800 <= ord(character) <= 0xDFFF for character in value):
            raise Difference(f"{path}: invalid_json (lone surrogate)")
    elif isinstance(value, (list, dict)):
        if depth >= PARSE_DEPTH_LIMIT:
            raise Difference(f"{path}: invalid_json (nested past the parse depth)")
        items = value.items() if isinstance(value, dict) else enumerate(value)
        for key, item in items:
            if isinstance(key, str) and any(0xD800 <= ord(c) <= 0xDFFF for c in key):
                raise Difference(f"{path}: invalid_json (lone surrogate)")
            _profile(item, f"{path}.{key}" if isinstance(value, dict) else f"{path}[{key}]", depth + 1)


def admit(text: str) -> Any:
    """A document text admitted under the checked profile, or Difference."""
    try:
        value = json.loads(text, object_pairs_hook=_no_duplicates, parse_constant=_not_json,
                           parse_int=_integer)
    except json.JSONDecodeError as error:
        raise Difference(f"invalid_json at character {error.pos}") from error
    except RecursionError as error:
        raise Difference("invalid_json (nested past the parse depth)") from error
    _profile(value, "$", 1)
    return value


# --- closure -----------------------------------------------------------------------------------

def _obj(value, path, keys):
    if type(value) is not dict:
        raise Difference(f"{path}: not an object")
    for key in keys:
        if key not in value:
            raise Difference(f"{path}.{key}: missing")
    for key in value:
        if key not in keys:
            raise Difference(f"{path}.{key}: not a member of the closed object")
    return value


def _string(value, path):
    if type(value) is not str:
        raise Difference(f"{path}: not a string")
    return value


def _number(value, path):
    if type(value) not in (int, float):
        raise Difference(f"{path}: not a number")
    return float(value)


def _boolean(value, path):
    if type(value) is not bool:
        raise Difference(f"{path}: not a boolean")
    return value


def _array(value, path):
    if type(value) is not list:
        raise Difference(f"{path}: not an array")
    return value


def _one_of(value, path, allowed):
    text = _string(value, path)
    if text not in allowed:
        raise Difference(f"{path}: not one of {sorted(allowed)}")
    return text


def _constant(value, path, expected):
    if value != expected or type(value) is not str:
        raise Difference(f"{path}: not {expected!r}")


def _hash(value, path):
    text = _string(value, path)
    if len(text) != 64 or any(c not in "0123456789abcdef" for c in text):
        raise Difference(f"{path}: not a lowercase SHA-256 hex digest")
    return text


def _nullable_hash(value, path):
    return None if value is None else _hash(value, path)


def _count(value, path):
    number = _number(value, path)
    if number < 0 or not number.is_integer():
        raise Difference(f"{path}: not a non-negative integer")


def _equal(a, b, path, what):
    if not a == b:
        raise Difference(f"{path}: {what} ({a!r} against {b!r})")


def _sizing_request(value, path):
    """The request matching $defs.SizingRequest exactly, as (weight, operating cases, candidates, method)."""
    request = _obj(value, path, REQUEST_KEYS)
    _string(request["weight_case"], f"{path}.weight_case")
    _constant(request["weight_support_state"], f"{path}.weight_support_state", "own_solve")
    _string(request["design_operating_case"], f"{path}.design_operating_case")
    for i, case in enumerate(_array(request["check_operating_cases"], f"{path}.check_operating_cases")):
        _string(case, f"{path}.check_operating_cases[{i}]")
    candidates = _array(request["candidates"], f"{path}.candidates")
    if not candidates:
        raise Difference(f"{path}.candidates: empty")
    for i, candidate in enumerate(candidates):
        _obj(candidate, f"{path}.candidates[{i}]", ("support", "family_policy"))
        _string(candidate["support"], f"{path}.candidates[{i}].support")
        _one_of(candidate["family_policy"], f"{path}.candidates[{i}].family_policy", POLICIES)
    method = _obj(request["method"], f"{path}.method", METHOD_KEYS)
    _one_of(method["load_design"], f"{path}.method.load_design", ("hot", "cold"))
    _one_of(method["variation_reference"], f"{path}.method.variation_reference", ("operating_load", "design_load"))
    _number(method["variation_limit"], f"{path}.method.variation_limit")
    _number(method["travel_allowance_m"], f"{path}.method.travel_allowance_m")
    _one_of(method["selection_rule"], f"{path}.method.selection_rule", ("smallest_capacity", "least_variation"))
    _one_of(request["solver_mode"], f"{path}.solver_mode", ("sparse_interactive", "dense_scrutiny"))
    return request


def _operating(request):
    return [request["design_operating_case"], *request["check_operating_cases"]]


def _identity(doc):
    _constant(doc["document_kind"], "$.document_kind", DOCUMENT_KIND)
    _constant(doc["schema_version"], "$.schema_version", SCHEMA_VERSION)
    _constant(doc["semantic_contract_id"], "$.semantic_contract_id", SEMANTIC_CONTRACT_ID)
    _constant(doc["route"], "$.route", ROUTE)
    for key, texts in (("limitations", LIMITATIONS), ("basis_statement", BASIS_STATEMENT)):
        items = _array(doc[key], f"$.{key}")
        if len(items) != len(texts) or any(item != text or type(item) is not str for item, text in zip(items, texts)):
            raise Difference(f"$.{key}: not the fixed text")


def _closure_refused(doc):
    _obj(doc, "$", REFUSED_KEYS)
    _identity(doc)
    inputs = _obj(doc["inputs"], "$.inputs", INPUT_KEYS)
    _constant(inputs["canonical_profile"], "$.inputs.canonical_profile", CANONICAL_PROFILE)
    if inputs["sizing_request"] is not None:
        _sizing_request(inputs["sizing_request"], "$.inputs.sizing_request")
    sha = _nullable_hash(inputs["sizing_request_sha256"], "$.inputs.sizing_request_sha256")
    if (inputs["sizing_request"] is None) != (sha is None):
        raise Difference("$.inputs.sizing_request_sha256: null exactly when sizing_request is null")
    solve = _obj(inputs["solve_request"], "$.inputs.solve_request", ("sha256",))
    _nullable_hash(solve["sha256"], "$.inputs.solve_request.sha256")
    library = _obj(inputs["library"], "$.inputs.library", ("library_id", "sha256"))
    if library["library_id"] is not None:
        _string(library["library_id"], "$.inputs.library.library_id")
    _nullable_hash(library["sha256"], "$.inputs.library.sha256")
    refusals = _array(doc["refusals"], "$.refusals")
    if not refusals:
        raise Difference("$.refusals: empty")
    for i, refusal in enumerate(refusals):
        path = f"$.refusals[{i}]"
        _obj(refusal, path, ("code", "names", "message"))
        _one_of(refusal["code"], f"{path}.code", REFUSAL_CODES)
        for j, name in enumerate(_array(refusal["names"], f"{path}.names")):
            _string(name, f"{path}.names[{j}]")
        if not _string(refusal["message"], f"{path}.message"):
            raise Difference(f"{path}.message: empty")


def _per_case(value, path, operating, keys):
    items = _array(value, path)
    if len(items) != len(operating):
        raise Difference(f"{path}: {len(items)} entries for {len(operating)} operating cases")
    for i, (item, case) in enumerate(zip(items, operating)):
        _obj(item, f"{path}[{i}]", keys)
        if _string(item["case"], f"{path}[{i}].case") != case:
            raise Difference(f"{path}[{i}].case: not the operating case {case!r}")
    return items


def _span(value, path):
    items = _array(value, path)
    if len(items) != 2:
        raise Difference(f"{path}: not a pair")
    low, high = _number(items[0], f"{path}[0]"), _number(items[1], f"{path}[1]")
    if low > high:
        raise Difference(f"{path}: min above max")


def _failed_tests(value, path, order):
    last = -1
    for i, item in enumerate(_array(value, path)):
        index = order.index(_one_of(item, f"{path}[{i}]", order))
        if index <= last:
            raise Difference(f"{path}: not an ordered subset of {list(order)}")
        last = index


def _closure_candidate(c, path, policy, operating):
    """One candidate's closure; returns whether its outcome is no_record_fits."""
    _string(c["node"], f"{path}.node")
    _one_of(c["dof"], f"{path}.dof", ("UX", "UY", "UZ"))
    _constant(c["family_policy"], f"{path}.family_policy", policy)
    outcome = _one_of(c["outcome"], f"{path}.outcome", ("selected", "selected_by_fallback", "no_record_fits"))
    w = _number(c["w_n"], f"{path}.w_n")
    _number(c["u_cold_m"], f"{path}.u_cold_m")
    for i, entry in enumerate(_per_case(c["operating"], f"{path}.operating", operating, ("case", "u_m"))):
        _number(entry["u_m"], f"{path}.operating[{i}].u_m")
    _span(c["span_m"], f"{path}.span_m")
    _number(c["required_travel_m"], f"{path}.required_travel_m")
    counts = _obj(c["failure_counts"], f"{path}.failure_counts", ("variable", "constant"))
    for family, tests in (("variable", VARIABLE_TESTS), ("constant", CONSTANT_TESTS)):
        if counts[family] is not None:
            _obj(counts[family], f"{path}.failure_counts.{family}", tests)
            for test in tests:
                _count(counts[family][test], f"{path}.failure_counts.{family}.{test}")
    apath = f"{path}.verification_assumption"
    assumption = c["verification_assumption"]
    role = assumption.get("role") if type(assumption) is dict else None
    if outcome == "no_record_fits":
        for key in ("selected", "governing", "verification"):
            if c[key] is not None:
                raise Difference(f"{path}.{key}: not null for no fit")
        _obj(assumption, apath, ("role", "force_n"))
        _constant(assumption["role"], f"{apath}.role", "force")
        _equal(_number(assumption["force_n"], f"{apath}.force_n"), w, apath, "force_n is not w_n")
        return True
    spath, gpath, vpath = f"{path}.selected", f"{path}.governing", f"{path}.verification"
    selected = c["selected"]
    family = selected.get("family") if type(selected) is dict else None
    governing = _obj(c["governing"], gpath, ("travel_case", "variation_case"))
    _one_of(governing["travel_case"], f"{gpath}.travel_case", operating)
    v = c["verification"]
    if family == "variable_spring":
        _obj(selected, spath, ("family", "hanger_id", "record_sha256", "k_n_per_m", "f_cold_n", "f_cold_condition_n",
                               "envelope_n", "operating"))
        _string(selected["hanger_id"], f"{spath}.hanger_id")
        _hash(selected["record_sha256"], f"{spath}.record_sha256")
        k = _number(selected["k_n_per_m"], f"{spath}.k_n_per_m")
        f_cold = _number(selected["f_cold_n"], f"{spath}.f_cold_n")
        _number(selected["f_cold_condition_n"], f"{spath}.f_cold_condition_n")
        envelope = _array(selected["envelope_n"], f"{spath}.envelope_n")
        if len(envelope) != 2:
            raise Difference(f"{spath}.envelope_n: not a pair")
        for i, end in enumerate(envelope):
            _number(end, f"{spath}.envelope_n[{i}]")
        for i, entry in enumerate(_per_case(selected["operating"], f"{spath}.operating", operating, ("case", "f_n", "variation"))):
            _number(entry["f_n"], f"{spath}.operating[{i}].f_n")
            _number(entry["variation"], f"{spath}.operating[{i}].variation")
        _one_of(governing["variation_case"], f"{gpath}.variation_case", operating)
        if role != "spring":
            raise Difference(f"{apath}.role: not spring for a variable selection")
        _obj(assumption, apath, ("role", "k_n_per_m", "f_cold_n"))
        _equal(_number(assumption["k_n_per_m"], f"{apath}.k_n_per_m"), k, apath, "k_n_per_m is not the selection's")
        _equal(_number(assumption["f_cold_n"], f"{apath}.f_cold_n"), f_cold, apath, "f_cold_n is not the selection's")
        _obj(v, vpath, ("family", "u_cold_m", "operating", "span_m", "required_travel_m", "t_pass", "f_cold_condition_n",
                        "departure_n", "failed_tests"))
        _constant(v["family"], f"{vpath}.family", "variable_spring")
        _number(v["u_cold_m"], f"{vpath}.u_cold_m")
        for i, entry in enumerate(_per_case(v["operating"], f"{vpath}.operating", operating, ("case", "u_m", "f_n", "variation"))):
            _number(entry["u_m"], f"{vpath}.operating[{i}].u_m")
            _number(entry["f_n"], f"{vpath}.operating[{i}].f_n")
            if entry["variation"] is not None:
                _number(entry["variation"], f"{vpath}.operating[{i}].variation")
        _span(v["span_m"], f"{vpath}.span_m")
        _number(v["required_travel_m"], f"{vpath}.required_travel_m")
        _boolean(v["t_pass"], f"{vpath}.t_pass")
        _number(v["f_cold_condition_n"], f"{vpath}.f_cold_condition_n")
        _number(v["departure_n"], f"{vpath}.departure_n")
        _failed_tests(v["failed_tests"], f"{vpath}.failed_tests", ("V0", "L", "T", "V"))
    elif family == "constant_effort":
        _obj(selected, spath, ("family", "hanger_id", "record_sha256", "load_n"))
        _string(selected["hanger_id"], f"{spath}.hanger_id")
        _hash(selected["record_sha256"], f"{spath}.record_sha256")
        _equal(_number(selected["load_n"], f"{spath}.load_n"), w, spath, "load_n is not w_n")
        if governing["variation_case"] is not None:
            raise Difference(f"{gpath}.variation_case: not null for a constant selection")
        if role != "force":
            raise Difference(f"{apath}.role: not force for a constant selection")
        _obj(assumption, apath, ("role", "force_n"))
        _equal(_number(assumption["force_n"], f"{apath}.force_n"), w, apath, "force_n is not w_n")
        _obj(v, vpath, ("family", "u_cold_m", "operating", "span_m", "required_travel_m", "t_pass", "failed_tests"))
        _constant(v["family"], f"{vpath}.family", "constant_effort")
        _number(v["u_cold_m"], f"{vpath}.u_cold_m")
        for i, entry in enumerate(_per_case(v["operating"], f"{vpath}.operating", operating, ("case", "u_m"))):
            _number(entry["u_m"], f"{vpath}.operating[{i}].u_m")
        _span(v["span_m"], f"{vpath}.span_m")
        _number(v["required_travel_m"], f"{vpath}.required_travel_m")
        _boolean(v["t_pass"], f"{vpath}.t_pass")
        _failed_tests(v["failed_tests"], f"{vpath}.failed_tests", ("T",))
    else:
        raise Difference(f"{spath}: not a variable or constant selection for outcome {outcome}")
    return False


def _solve_plan(request):
    weight, operating = request["weight_case"], _operating(request)
    return ([("S_W", weight), ("S_C", weight)] + [(f"S_O{i + 1}", case) for i, case in enumerate(operating)]
            + [("V_C", weight)] + [(f"V_O{i + 1}", case) for i, case in enumerate(operating)])


def _closure_sized(doc):
    _obj(doc, "$", SIZED_KEYS)
    _identity(doc)
    inputs = _obj(doc["inputs"], "$.inputs", INPUT_KEYS)
    _constant(inputs["canonical_profile"], "$.inputs.canonical_profile", CANONICAL_PROFILE)
    request = _sizing_request(inputs["sizing_request"], "$.inputs.sizing_request")
    _hash(inputs["sizing_request_sha256"], "$.inputs.sizing_request_sha256")
    _hash(_obj(inputs["solve_request"], "$.inputs.solve_request", ("sha256",))["sha256"], "$.inputs.solve_request.sha256")
    library = _obj(inputs["library"], "$.inputs.library", ("library_id", "sha256"))
    _string(library["library_id"], "$.inputs.library.library_id")
    _hash(library["sha256"], "$.inputs.library.sha256")

    result = _obj(doc["result"], "$.result", ("verification_standing", "cases", "candidates", "solves", "disclosures"))
    standing = _one_of(result["verification_standing"], "$.result.verification_standing", ("unconditional", "conditional"))
    operating = _operating(request)
    named = [request["weight_case"], *operating]
    cases = _array(result["cases"], "$.result.cases")
    if len(cases) != len(named):
        raise Difference(f"$.result.cases: {len(cases)} entries for {len(named)} named cases")
    for i, (case, name) in enumerate(zip(cases, named)):
        path = f"$.result.cases[{i}]"
        _obj(case, path, ("case", "role", "load_categories", "uses_condition_set"))
        if _string(case["case"], f"{path}.case") != name:
            raise Difference(f"{path}.case: not the request's case {name!r}")
        if _one_of(case["role"], f"{path}.role", ("weight", "operating")) != ("weight" if i == 0 else "operating"):
            raise Difference(f"{path}.role: not the case's role in the request")
        previous = None
        for j, category in enumerate(_array(case["load_categories"], f"{path}.load_categories")):
            _one_of(category, f"{path}.load_categories[{j}]", LOAD_CATEGORIES)
            if previous is not None and previous.encode() >= category.encode():
                raise Difference(f"{path}.load_categories: not sorted and unique")
            previous = category
        _boolean(case["uses_condition_set"], f"{path}.uses_condition_set")
    if len(set(named)) != len(named):
        raise Difference("$.result.cases: case ids not unique")

    candidates = _array(result["candidates"], "$.result.candidates")
    if len(candidates) != len(request["candidates"]):
        raise Difference(f"$.result.candidates: {len(candidates)} entries for {len(request['candidates'])} requested")
    supports, no_fit = [], False
    for i, (candidate, asked) in enumerate(zip(candidates, request["candidates"])):
        path = f"$.result.candidates[{i}]"
        _obj(candidate, path, CANDIDATE_KEYS)
        if _string(candidate["support"], f"{path}.support") != asked["support"]:
            raise Difference(f"{path}.support: not the request's candidate {asked['support']!r}")
        supports.append(candidate["support"])
        no_fit |= _closure_candidate(candidate, path, asked["family_policy"], operating)
    if len(set(supports)) != len(supports):
        raise Difference("$.result.candidates: support not unique")
    if (standing == "conditional") != no_fit:
        raise Difference("$.result.verification_standing: not conditional exactly when some candidate has no fit")

    plan = _solve_plan(request)
    solves = _array(result["solves"], "$.result.solves")
    if len(solves) != len(plan):
        raise Difference(f"$.result.solves: {len(solves)} entries, expected {len(plan)}")
    for i, (solve, (label, case)) in enumerate(zip(solves, plan)):
        path = f"$.result.solves[{i}]"
        _obj(solve, path, ("label", "case", "derived_document_sha256", "states"))
        if _string(solve["label"], f"{path}.label") != label:
            raise Difference(f"{path}.label: not {label!r}")
        if _string(solve["case"], f"{path}.case") != case:
            raise Difference(f"{path}.case: not the case of {label}")
        _hash(solve["derived_document_sha256"], f"{path}.derived_document_sha256")
        if type(solve["states"]) is not dict:
            raise Difference(f"{path}.states: not an object")
        for support, state in solve["states"].items():
            if not support:
                raise Difference(f"{path}.states: an empty support id")
            _one_of(state, f"{path}.states.{support}", STATE_CODES)

    disclosures = _array(result["disclosures"], "$.result.disclosures")
    if len(disclosures) > 2:
        raise Difference("$.result.disclosures: more than two entries")
    codes = []
    for i, disclosure in enumerate(disclosures):
        path = f"$.result.disclosures[{i}]"
        _obj(disclosure, path, ("code", "names", "message"))
        code = _one_of(disclosure["code"], f"{path}.code", DISCLOSURE_CODES)
        if code in codes:
            raise Difference(f"{path}.code: repeated")
        codes.append(code)
        names = _array(disclosure["names"], f"{path}.names")
        if not names:
            raise Difference(f"{path}.names: empty")
        previous = None
        for j, name in enumerate(names):
            _string(name, f"{path}.names[{j}]")
            if previous is not None and previous.encode() >= name.encode():
                raise Difference(f"{path}.names: not unique and sorted bytewise")
            previous = name
        _string(disclosure["message"], f"{path}.message")


def closure(doc) -> None:
    status = doc.get("status") if type(doc) is dict else None
    if status == "sized":
        _closure_sized(doc)
    elif status == "refused":
        _closure_refused(doc)
    else:
        raise Difference('$.status: neither "sized" nor "refused"')


# --- binding -----------------------------------------------------------------------------------

def _sha(value):
    try:
        return canonical_json.canonical_sha256(value)
    except ValueError:
        return None


def _hash_matches(value, published, path):
    digest = _sha(value)
    if digest is None:
        raise Difference(f"{path}: the supplied input cannot be hashed under the profile")
    if published != digest:
        raise Difference(f"{path}: does not match the supplied input's hash")


def _records_with_id(library, hanger_id):
    records = library.get("hanger_records") if type(library) is dict else None
    return [r for r in records if type(r) is dict and r.get("hanger_id") == hanger_id] if type(records) is list else []


def _binding(doc, solve_request, library, sized):
    """(standing, not_checked, library hash-matched)."""
    inputs = doc["inputs"]
    differences, checked, not_checked = [], False, []

    def note(check, *args):
        try:
            check(*args)
        except Difference as difference:
            differences.append(str(difference))

    if inputs["sizing_request"] is not None:
        checked = True
        note(_hash_matches, inputs["sizing_request"], inputs["sizing_request_sha256"], "$.inputs.sizing_request_sha256")
    library_bound = False
    if library is None:
        not_checked.append("library")
    elif sized:
        checked = True
        before = len(differences)
        note(_hash_matches, library, inputs["library"]["sha256"], "$.inputs.library.sha256")
        hanger_library = library.get("hanger_library") if type(library) is dict else None
        library_id = hanger_library.get("library_id") if type(hanger_library) is dict else None
        if library_id != inputs["library"]["library_id"]:
            differences.append("$.inputs.library.library_id: not the supplied library's")
        library_bound = len(differences) == before
        for i, candidate in enumerate(doc["result"]["candidates"]):
            selected = candidate["selected"]
            if selected is None:
                continue
            path = f"$.result.candidates[{i}].selected"
            found = _records_with_id(library, selected["hanger_id"])
            if len(found) == 1:
                note(_hash_matches, found[0], selected["record_sha256"], f"{path}.record_sha256")
            elif not found:
                differences.append(f"{path}.hanger_id: no record of the supplied library")
            else:
                differences.append(f"{path}.hanger_id: not unique in the supplied library")
    elif inputs["library"]["sha256"] is not None:
        checked = True
        note(_hash_matches, library, inputs["library"]["sha256"], "$.inputs.library.sha256")
    if solve_request is None:
        not_checked.append("solve_request")
    elif inputs["solve_request"]["sha256"] is not None:
        checked = True
        note(_hash_matches, solve_request, inputs["solve_request"]["sha256"], "$.inputs.solve_request.sha256")
    if differences:
        standing = {"status": "failed", "difference": differences[0]}
    else:
        standing = {"status": "passed" if checked else "not_checked", "difference": None}
    return standing, not_checked, library_bound


# --- arithmetic, binary64 in the stated order --------------------------------

def _quantity(value, dimension):
    """A library quantity in SI, or None when absent or unreadable (the record then fails E1)."""
    if type(value) is not dict:
        return None
    magnitude, unit = value.get("magnitude"), value.get("unit")
    if type(magnitude) not in (int, float) or type(unit) is not str:
        return None
    try:
        converted = units.to_canonical(float(magnitude), unit, dimension)
    except ValueError:
        return None
    return converted if math.isfinite(converted) else None


def library_records(library):
    """The library's records in document order: (hanger_id, family, k, k_dof, (f_min, f_max) | None, travel)."""
    out = []
    records = library.get("hanger_records") if type(library) is dict else None
    for record in records if type(records) is list else []:
        hanger = record.get("hanger") if type(record) is dict else None
        if type(hanger) is not dict:
            continue
        family = {"variable_spring_hanger": "variable", "constant_effort_support": "constant"}.get(hanger.get("hanger_type"))
        hanger_id = record.get("hanger_id")
        if family is None or type(hanger_id) is not str:
            continue
        stiffness = hanger.get("stiffness") if type(hanger.get("stiffness")) is dict else {}
        span = hanger.get("working_load_range") if type(hanger.get("working_load_range")) is dict else {}
        low, high = _quantity(span.get("minimum"), "force"), _quantity(span.get("maximum"), "force")
        dof = stiffness.get("dof")
        out.append({
            "hanger_id": hanger_id,
            "family": family,
            "k": _quantity(stiffness.get("value"), "force_per_length"),
            "dof": dof if dof in ("UX", "UY", "UZ") else None,
            "range": (low, high) if low is not None and high is not None else None,
            "travel": _quantity(hanger.get("travel_range"), "length"),
        })
    return out


def _finite_positive(value):
    return value is not None and math.isfinite(value) and value > 0.0


def _sizable(record):
    if record["range"] is None or record["travel"] is None:
        return None
    low, high = record["range"]
    if _finite_positive(low) and _finite_positive(high) and low < high and _finite_positive(record["travel"]):
        return low, high, record["travel"]
    return None


def _span_of(u_cold, u_operating):
    low, high = min(0.0, u_cold), max(0.0, u_cold)
    for u in u_operating:
        low, high = min(low, u), max(high, u)
    return low, high


def _required(span, a):
    return (span[1] - span[0]) + 2.0 * a


def _envelope(span, f_cold, k, a):
    return f_cold - k * (span[1] + a), f_cold - k * (span[0] - a)


def _variations(k, u_cold, u_operating, f_operating, w, reference):
    out = []
    for u, f in zip(u_operating, f_operating):
        r = f if reference == "operating_load" else w
        out.append(k * abs(u - u_cold) / r if r > 0.0 else None)
    return out


def _evaluate(c, record, method):
    """(first failed test, None) or (None, loads) for a variable record; (test | None, None) for a constant one."""
    span = _span_of(c["u_cold"], c["u_operating"])
    a = method["travel_allowance_m"]
    if record["family"] == "variable":
        k = record["k"]
        if not _finite_positive(k):
            return "E1", None
        sizable = _sizable(record)
        if sizable is None:
            return "E1", None
        low, high, travel = sizable
        if record["dof"] != c["dof"]:
            return "E2", None
        u_design = c["u_operating"][0]
        f_cold = c["w"] + k * u_design if method["load_design"] == "hot" else c["w"] + k * c["u_cold"]
        f_cold_condition = f_cold - k * c["u_cold"]
        f_operating = [f_cold - k * u for u in c["u_operating"]]
        if not f_cold_condition > 0.0 or any(not f > 0.0 for f in f_operating):
            return "V0", None
        envelope = _envelope(span, f_cold, k, a)
        if envelope[0] < low or envelope[1] > high:
            return "L", None
        if _required(span, a) > travel:
            return "T", None
        variation = _variations(k, c["u_cold"], c["u_operating"], f_operating, c["w"], method["variation_reference"])
        if any(v > method["variation_limit"] for v in variation):
            return "V", None
        return None, {"k": k, "f_cold": f_cold, "f_cold_condition": f_cold_condition, "f_operating": f_operating,
                      "variation": variation, "envelope": envelope}
    sizable = _sizable(record)
    if sizable is None:
        return "E1", None
    low, high, travel = sizable
    if c["w"] < low or c["w"] > high:
        return "L", None
    if _required(span, a) > travel:
        return "T", None
    return None, None


def _family_pass(c, records, method, family):
    tests = VARIABLE_TESTS if family == "variable" else CONSTANT_TESTS
    failures = {test: 0 for test in tests}
    best = None
    for index, record in enumerate(records):
        if record["family"] != family:
            continue
        failed, loads = _evaluate(c, record, method)
        if failed is not None:
            failures[failed] += 1
            continue
        capacity = record["range"][1]
        variation = max([0.0, *loads["variation"]]) if loads else 0.0
        key = ((capacity, variation) if method["selection_rule"] == "smallest_capacity" else (variation, capacity)) + (
            record["hanger_id"].encode("utf-8"),)
        if best is None or key < best[0]:
            best = (key, index, loads)
    return best, failures


def _size_candidate(c, records, method):
    span = _span_of(c["u_cold"], c["u_operating"])
    result = {"outcome": "no_record_fits", "selected": None, "span": span,
              "required": _required(span, method["travel_allowance_m"]), "counts": {"variable": None, "constant": None}}
    if c["policy"] in ("variable", "variable_else_constant"):
        best, failures = _family_pass(c, records, method, "variable")
        result["counts"]["variable"] = failures
        if best is not None:
            result["outcome"], result["selected"] = "selected", ("variable", best[1], best[2])
            return result
    if c["policy"] in ("constant", "variable_else_constant"):
        best, failures = _family_pass(c, records, method, "constant")
        result["counts"]["constant"] = failures
        if best is not None:
            result["outcome"] = "selected_by_fallback" if c["policy"] == "variable_else_constant" else "selected"
            result["selected"] = ("constant", best[1], None)
    return result


def _request_refused(candidates, records, method):
    """Whether the engine's request checks refuse these published inputs."""
    limit, allowance = method["variation_limit"], method["travel_allowance_m"]
    ids = [r["hanger_id"] for r in records]
    supports = [c["id"] for c in candidates]
    return (not _finite_positive(limit) or not (math.isfinite(allowance) and allowance >= 0.0) or not candidates
            or len(set(supports)) != len(supports) or len(set(ids)) != len(ids)
            or any(not c["u_operating"] for c in candidates)
            or any(not c["w"] > 0.0 for c in candidates))


def _verify(c, selection, records, method, u_cold, u_operating):
    family, index, loads = selection
    a = method["travel_allowance_m"]
    span = _span_of(u_cold, u_operating)
    required = _required(span, a)
    record = records[index]
    t_pass = required <= record["travel"]
    out = {"span": span, "required": required, "t_pass": t_pass, "failed": []}
    if family == "constant":
        if not t_pass:
            out["failed"].append("T")
        return out
    k, f_cold = loads["k"], loads["f_cold"]
    f_cold_condition = f_cold - k * u_cold
    f_operating = [f_cold - k * u for u in u_operating]
    variation = _variations(k, u_cold, u_operating, f_operating, c["w"], method["variation_reference"])
    departure = f_operating[0] - c["w"] if method["load_design"] == "hot" else f_cold_condition - c["w"]
    low, high = record["range"]
    if not f_cold_condition > 0.0 or any(not f > 0.0 for f in f_operating):
        out["failed"].append("V0")
    envelope = _envelope(span, f_cold, k, a)
    if envelope[0] < low or envelope[1] > high:
        out["failed"].append("L")
    if not t_pass:
        out["failed"].append("T")
    if any(v is not None and v > method["variation_limit"] for v in variation):
        out["failed"].append("V")
    out.update(f_cold_condition=f_cold_condition, f_operating=f_operating, variation=variation, departure=departure)
    return out


def _argmax(values):
    best = None
    for i, value in enumerate(values):
        if best is None or value > best[1]:
            best = (i, value)
    return best[0] if best else 0


def _arithmetic(doc, library):
    request = doc["inputs"]["sizing_request"]
    method = request["method"]
    method = {**method, "variation_limit": float(method["variation_limit"]), "travel_allowance_m": float(method["travel_allowance_m"])}
    operating = _operating(request)
    records = library_records(library)
    published = doc["result"]["candidates"]
    candidates = [{"id": c["support"], "dof": c["dof"], "policy": asked["family_policy"], "w": float(c["w_n"]),
                   "u_cold": float(c["u_cold_m"]), "u_operating": [float(e["u_m"]) for e in c["operating"]]}
                  for c, asked in zip(published, request["candidates"])]
    if _request_refused(candidates, records, method):
        raise Difference("$.result: the engine's request checks refuse these published inputs")
    for i, (c, p) in enumerate(zip(candidates, published)):
        path = f"$.result.candidates[{i}]"
        expected = _size_candidate(c, records, method)
        if p["outcome"] != expected["outcome"]:
            raise Difference(f"{path}.outcome: recomputed {expected['outcome']}")
        _equal(float(p["span_m"][0]), expected["span"][0], f"{path}.span_m[0]", "differs")
        _equal(float(p["span_m"][1]), expected["span"][1], f"{path}.span_m[1]", "differs")
        _equal(float(p["required_travel_m"]), expected["required"], f"{path}.required_travel_m", "differs")
        for family in ("variable", "constant"):
            counts, published_counts = expected["counts"][family], p["failure_counts"][family]
            if (counts is None) != (published_counts is None):
                raise Difference(f"{path}.failure_counts.{family}: tested-ness differs")
            for test, n in (counts or {}).items():
                _equal(float(published_counts[test]), float(n), f"{path}.failure_counts.{family}.{test}", "count differs")
        if expected["selected"] is None:
            continue
        family, index, loads = expected["selected"]
        s, spath = p["selected"], f"{path}.selected"
        if s["hanger_id"] != records[index]["hanger_id"]:
            raise Difference(f"{spath}.hanger_id: recomputed {records[index]['hanger_id']!r}")
        travel_case = operating[_argmax([abs(u - c["u_cold"]) for u in c["u_operating"]])]
        if p["governing"]["travel_case"] != travel_case:
            raise Difference(f"{path}.governing.travel_case: recomputed {travel_case!r}")
        if family == "variable":
            if s["family"] != "variable_spring":
                raise Difference(f"{spath}.family: recomputed variable_spring")
            _equal(float(s["k_n_per_m"]), loads["k"], f"{spath}.k_n_per_m", "differs")
            _equal(float(s["f_cold_n"]), loads["f_cold"], f"{spath}.f_cold_n", "differs")
            _equal(float(s["f_cold_condition_n"]), loads["f_cold_condition"], f"{spath}.f_cold_condition_n", "differs")
            _equal(float(s["envelope_n"][0]), loads["envelope"][0], f"{spath}.envelope_n[0]", "differs")
            _equal(float(s["envelope_n"][1]), loads["envelope"][1], f"{spath}.envelope_n[1]", "differs")
            for j, entry in enumerate(s["operating"]):
                _equal(float(entry["f_n"]), loads["f_operating"][j], f"{spath}.operating[{j}].f_n", "differs")
                _equal(float(entry["variation"]), loads["variation"][j], f"{spath}.operating[{j}].variation", "differs")
            variation_case = operating[_argmax(loads["variation"])]
            if p["governing"]["variation_case"] != variation_case:
                raise Difference(f"{path}.governing.variation_case: recomputed {variation_case!r}")
        else:
            if s["family"] != "constant_effort":
                raise Difference(f"{spath}.family: recomputed constant_effort")
            _equal(float(s["load_n"]), c["w"], f"{spath}.load_n", "differs")
        v, vpath = p["verification"], f"{path}.verification"
        recomputed = _verify(c, expected["selected"], records, method, float(v["u_cold_m"]),
                             [float(e["u_m"]) for e in v["operating"]])
        _equal(float(v["span_m"][0]), recomputed["span"][0], f"{vpath}.span_m[0]", "differs")
        _equal(float(v["span_m"][1]), recomputed["span"][1], f"{vpath}.span_m[1]", "differs")
        _equal(float(v["required_travel_m"]), recomputed["required"], f"{vpath}.required_travel_m", "differs")
        if v["t_pass"] is not recomputed["t_pass"]:
            raise Difference(f"{vpath}.t_pass: recomputed {recomputed['t_pass']}")
        if v["failed_tests"] != recomputed["failed"]:
            raise Difference(f"{vpath}.failed_tests: recomputed {recomputed['failed']}")
        if family == "variable":
            _equal(float(v["f_cold_condition_n"]), recomputed["f_cold_condition"], f"{vpath}.f_cold_condition_n", "differs")
            _equal(float(v["departure_n"]), recomputed["departure"], f"{vpath}.departure_n", "differs")
            for j, entry in enumerate(v["operating"]):
                epath = f"{vpath}.operating[{j}]"
                _equal(float(entry["f_n"]), recomputed["f_operating"][j], f"{epath}.f_n", "differs")
                expected_variation = recomputed["variation"][j]
                if (entry["variation"] is None) != (expected_variation is None):
                    raise Difference(f"{epath}.variation: formed-ness differs")
                if expected_variation is not None:
                    _equal(float(entry["variation"]), expected_variation, f"{epath}.variation", "differs")


# --- the verdict -------------------------------------------------------------------------------

def _standing(check, *args):
    try:
        check(*args)
    except Difference as difference:
        return {"status": "failed", "difference": str(difference)}
    return {"status": "passed", "difference": None}


NOT_CHECKED = {"status": "not_checked", "difference": None}


def verify(document: str | Any, *, solve_request: Any = None, library: Any = None) -> dict:
    """The three standings of one document (its text, admitted here, or an already-parsed value)."""
    if isinstance(document, str):
        try:
            document = admit(document)
        except Difference as difference:
            return {"closure": {"status": "failed", "difference": f"not admitted: {difference}"},
                    "binding": {**NOT_CHECKED, "not_checked": []},
                    "arithmetic_consistent_with_published_positions": dict(NOT_CHECKED)}
    else:
        try:
            _profile(document, "$", 1)
        except Difference as difference:
            return {"closure": {"status": "failed", "difference": f"not admitted: {difference}"},
                    "binding": {**NOT_CHECKED, "not_checked": []},
                    "arithmetic_consistent_with_published_positions": dict(NOT_CHECKED)}
    closure_standing = _standing(closure, document)
    if closure_standing["status"] != "passed":
        return {"closure": closure_standing, "binding": {**NOT_CHECKED, "not_checked": []},
                "arithmetic_consistent_with_published_positions": dict(NOT_CHECKED)}
    sized = document["status"] == "sized"
    binding, not_checked, library_bound = _binding(document, solve_request, library, sized)
    arithmetic = _standing(_arithmetic, document, library) if sized and library is not None and library_bound else dict(NOT_CHECKED)
    return {"closure": closure_standing, "binding": {**binding, "not_checked": not_checked},
            "arithmetic_consistent_with_published_positions": arithmetic}
