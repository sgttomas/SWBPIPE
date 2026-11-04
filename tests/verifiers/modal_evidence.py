"""Independent test-only reader of modal result documents (DEL-04-09
`execution/PKG-04/DEL-04-09/Work/modal-analysis.md` §5.3, increment I3a):
`openpipestress.modal_result` 0.1.0 under `openpipestress.result_semantics/modal-1`.

It re-derives the governed relations without an eigen solve, from the document's
own authoritative payload (the formed global mass entries and the published
shapes), and returns the same verdict vocabulary as the Rust reader
(`core/reporting/modal_result/src/lib.rs`), with which it shares no
code:

- `closure` (shape, constants, ids, ordinals, checksums, digests, references) and
  `binding` (hashes, run id, options against the extraction, a supplied request);
- the relations `mass`, `normalization`, `participation`, `counts`, `clusters`,
  `sturm`, `evidence`, `standing` and `units`.

Each is passed, failed (with the first difference), scoped_unsupported (with its
scope) or not_checked. A withheld payload is scoped unsupported and never
reconstructed from a digest. The arithmetic is binary64, in the Design's stated
order and with its stated allowances, with explicit loops (Python's built-in `sum`
compensates, which the rule does not). Imports: the standard library and the
siblings `canonical_json` (hashing) and `units` (the catalog). Every rule, list and
text here is written from the Design, not read from product code.
"""
from __future__ import annotations

import json
import math
from typing import Any

from tests.verifiers import canonical_json, units

FAMILY = "openpipestress.modal_result"
SCHEMA_VERSION = "0.1.0"
SEMANTICS_ID = "openpipestress.result_semantics/modal-1"
SEMANTICS_SHA256 = "22884200060d6bae480ad922d799131486ac439d77ecd8b03516c310305c8e82"
PROFILE_ID = "product_preview_modal_v1"
CANONICAL_PROFILE = "openpipestress_jcs_ijson_v1"
LIMITATIONS = (
    "Small-displacement, linear-elastic Euler-Bernoulli frame modal preview about the unloaded configuration, with any nonlinear supports at the stated state of a named solved case, with consistent mass on straight members and lumped rigid-body mass on realized arcs and component bodies; numerical integrity does not establish physical correctness.",
    "The integrity checks are numerical estimates under stated premises, not mathematical bounds: each mode's residual is evaluated against a double-double re-formation of the stiffness and mass, its eigenvalue error is a first-order estimate, and the Sturm count is relied on only at shifts where the estimated backward error of forming and factoring the shifted matrix is small against both its estimated distance from singularity and the shift's separation from the computed eigenvalues. They do not bound modelling, discretization or input error.",
)
RELATIONS = ("mass", "normalization", "participation", "counts", "clusters", "sturm", "evidence", "standing", "units")

U = 2.0 ** -53
ALLOW = 16.0
TWO_PI = 2.0 * math.pi
COMPONENTS = ("ux", "uy", "uz", "rx", "ry", "rz")
DIRECTIONS = ("x", "y", "z")
CASES = ("unit_max_translation", "unit_max_rotation")
TABLES = ("nodes", "dofs", "mass_matrix", "mass_totals", "modes", "shapes", "participation", "clusters", "computed_modes",
          "sturm_candidates", "identity", "supports", "discretization", "disclosures", "diagnostics")
WITHHOLDABLE = ("mass_matrix", "shapes")
OPTIONS = ("mode_count", "frequency_cutoff", "state_case", "modulus_basis_ref", "modulus_basis_temperature",
           "weight_to_mass_acceleration", "include_dynamic_only_restraints")
PARAMETERS = ("infinite_mu_relative_threshold", "mass_pivot_relative_threshold", "resolution_floor_constant",
              "resolution_floor_multiple", "formed_backward_error_multiple", "screen_error_estimate_threshold",
              "max_ql_iterations_per_eigenvalue", "residual_threshold", "orthogonality_threshold", "cluster_tolerance",
              "no_translation_threshold", "sturm_conditioning_multiple", "sturm_separation_multiple", "sturm_growth_limit",
              "sturm_max_candidates", "sturm_gap_log_positions", "sturm_whole_spectrum_offsets", "identity_tolerance")
DECISIONS = ("accepted", "outside_gap", "out_of_range", "separation_out_of_range", "factorization_rejected", "unseparated",
             "growth_limit", "ill_conditioned")
SOLVED_KEYS = ("document_kind", "schema_version", "semantic_contract", "profile", "canonical_profile", "admission", "run",
               "binding", "status", "basis", "extraction", "census", "standing", "sturm", "orthogonality", "formation", "guard",
               "tables")
REFUSED_KEYS = ("document_kind", "schema_version", "semantic_contract", "profile", "canonical_profile", "admission", "run",
                "binding", "status", "refusals")


# --- modal-1, as this reader states it ----------------------------------------------------

def _catalog(symbol: str, dimension: str, unit_id: str) -> dict:
    return {"unit_status": "catalog", "unit": unit_id, "symbol": symbol, "dimension": dimension}


ONE = _catalog("1", "dimensionless", "unit:one")
METER = _catalog("m", "length", "unit:meter")
KILOGRAM = _catalog("kg", "mass", "unit:kilogram")
HERTZ = _catalog("Hz", "frequency", "unit:hertz")
RAD_PER_S = _catalog("rad/s", "angular_frequency", "unit:radian_per_second")
SECOND = _catalog("s", "time", "unit:second")
ACCELERATION = _catalog("m/s^2", "acceleration", "unit:meter_per_second_squared")
RAD_PER_M = _catalog("rad/m", "rotation_per_length", "unit:radian_per_meter")
M_PER_RAD = _catalog("m/rad", "length_per_rotation", "unit:meter_per_radian")
KG_M2 = _catalog("kg*m^2/rad^2", "rotational_generalized_mass", "unit:kilogram_square_meter_per_square_radian")
EQUILIBRATED = {"unit_status": "equilibrated_basis", "unit": None, "symbol": None, "dimension": None}
COUPLING = {"unit_status": "catalog_family_absent", "unit": None, "symbol": None, "dimension": None,
            "dimension_exponents": {"length": 1, "mass": 1, "time": 0, "temperature": 0, "angle": -1}}


def _by_case(a: dict, b: dict) -> dict:
    return {"unit_max_translation": a, "unit_max_rotation": b}


KINDS = [
    ("node_coordinate", "nodes", "fixed", {"fixed": METER}),
    ("mass_matrix_entry", "mass_matrix", "dof_pair_class",
     {"translation_translation": KILOGRAM, "translation_rotation": COUPLING, "rotation_rotation": KG_M2}),
    ("modal_total_mass", "mass_totals", "fixed", {"fixed": KILOGRAM}),
    ("modal_free_dof_mass", "mass_totals", "fixed", {"fixed": KILOGRAM}),
    ("modal_circular_frequency", "modes", "fixed", {"fixed": RAD_PER_S}),
    ("modal_frequency", "modes", "fixed", {"fixed": HERTZ}),
    ("modal_period", "modes", "fixed", {"fixed": SECOND}),
    ("modal_generalized_mass", "modes", "normalization_case", _by_case(KILOGRAM, KG_M2)),
    ("modal_cluster_error_estimate", "modes", "fixed", {"fixed": ONE}),
    ("mode_shape_translation", "shapes", "normalization_case", _by_case(ONE, M_PER_RAD)),
    ("mode_shape_rotation", "shapes", "normalization_case", _by_case(RAD_PER_M, ONE)),
    ("modal_participation_factor", "participation", "normalization_case", _by_case(ONE, RAD_PER_M)),
    ("modal_effective_mass", "participation", "fixed", {"fixed": KILOGRAM}),
    ("modal_effective_mass_fraction", "participation", "fixed", {"fixed": ONE}),
    ("modal_cumulative_effective_mass_fraction", "participation", "fixed", {"fixed": ONE}),
    ("cluster_error_estimate", "clusters", "fixed", {"fixed": ONE}),
    ("computed_circular_frequency", "computed_modes", "fixed", {"fixed": RAD_PER_S}),
    ("computed_frequency", "computed_modes", "fixed", {"fixed": HERTZ}),
    ("residual_backward_error", "computed_modes", "fixed", {"fixed": ONE}),
    ("eigenvalue_condition_estimate", "computed_modes", "fixed", {"fixed": ONE}),
    ("eigenvalue_error_estimate", "computed_modes", "fixed", {"fixed": ONE}),
    ("sturm_shift_circular_frequency", "sturm_candidates", "fixed", {"fixed": RAD_PER_S}),
    ("sturm_formation_term", "sturm_candidates", "fixed", {"fixed": EQUILIBRATED}),
    ("sturm_formation_separation_ratio", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_factor_term", "sturm_candidates", "fixed", {"fixed": EQUILIBRATED}),
    ("sturm_backward_error", "sturm_candidates", "fixed", {"fixed": EQUILIBRATED}),
    ("sturm_inverse_norm_estimate", "sturm_candidates", "fixed", {"fixed": EQUILIBRATED}),
    ("sturm_conditioning_product", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_separation_ratio", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_growth_factor", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_max_abs_lower_factor", "sturm_candidates", "fixed", {"fixed": EQUILIBRATED}),
    ("sturm_max_abs_pivot", "sturm_candidates", "fixed", {"fixed": EQUILIBRATED}),
    ("sturm_max_abs_scaled_diagonal", "sturm_candidates", "fixed", {"fixed": EQUILIBRATED}),
    ("identity_relative_discrepancy", "identity", "fixed", {"fixed": ONE}),
    ("member_length", "discretization", "fixed", {"fixed": METER}),
    ("member_fixed_fixed_frequency", "discretization", "fixed", {"fixed": HERTZ}),
    ("member_frequency_ratio", "discretization", "fixed", {"fixed": ONE}),
    ("basis_no_translation_threshold", "header", "fixed", {"fixed": ONE}),
    ("basis_bounding_box_diagonal", "header", "fixed", {"fixed": METER}),
    ("basis_mass_center", "header", "fixed", {"fixed": METER}),
    ("request_frequency_cutoff", "header", "fixed", {"fixed": HERTZ}),
    ("request_weight_to_mass_acceleration", "header", "fixed", {"fixed": ACCELERATION}),
    ("identity_acceleration", "header", "fixed", {"fixed": ACCELERATION}),
    ("placement_circular_frequency", "header", "fixed", {"fixed": RAD_PER_S}),
    ("sturm_premise_parameter", "header", "fixed", {"fixed": ONE}),
    ("orthogonality_max_ratio", "header", "fixed", {"fixed": ONE}),
    ("formation_difference", "header", "fixed", {"fixed": EQUILIBRATED}),
    ("reformed_norm", "header", "fixed", {"fixed": EQUILIBRATED}),
    ("section_derivation_difference", "header", "fixed", {"fixed": ONE}),
]
RULES = {kind: rules for kind, _, _, rules in KINDS}


def semantic_table() -> dict:
    """modal-1 as this reader states it (the committed table's value)."""
    return {"id": SEMANTICS_ID, "document_kind": FAMILY, "schema_version": SCHEMA_VERSION,
            "canonical_profile": CANONICAL_PROFILE,
            "kinds": [{"kind": k, "table": t, "selector": s, "units": r} for k, t, s, r in KINDS]}


def catalog_differences() -> list[str]:
    """Each catalog rule's symbol and dimension against the catalog's canonical unit."""
    out = []
    for kind, _, _, rules in KINDS:
        for key, rule in rules.items():
            if rule["unit_status"] != "catalog":
                continue
            canonical = units.CANONICAL.get(rule["dimension"])
            if canonical is None or canonical[0] != rule["symbol"]:
                out.append(f"{kind}/{key}: {rule['symbol']} is not the canonical {rule['dimension']} unit")
    return out


def sha256(value: Any) -> str | None:
    try:
        return canonical_json.canonical_sha256(value)
    except (ValueError, TypeError, RecursionError):
        return None


class Failure(Exception):
    pass


def _fail(text: str):
    raise Failure(text)


def _is_number(v) -> bool:
    return type(v) in (int, float) and math.isfinite(v)


def _obj(value, path, keys):
    if not isinstance(value, dict):
        _fail(f"{path}: not an object")
    for key in keys:
        if key not in value:
            _fail(f"{path}: missing `{key}`")
    for key in value:
        if key not in keys:
            _fail(f"{path}: undeclared `{key}`")
    return value


def _text(value, path):
    if not isinstance(value, str):
        _fail(f"{path}: not a string")
    return value


def _count(value, path):
    if type(value) is not int or value < 0:
        _fail(f"{path}: not a non-negative integer")
    return value


def _flag(value, path):
    if type(value) is not bool:
        _fail(f"{path}: not a boolean")
    return value


def _list(value, path):
    if not isinstance(value, list):
        _fail(f"{path}: not an array")
    return value


def _number(value, path):
    if not _is_number(value):
        _fail(f"{path}: not a finite number")
    return float(value)


def _const(value, path, expected):
    if value != expected or not isinstance(value, str):
        _fail(f"{path}: expected `{expected}`")


def _one_of(value, path, allowed):
    text = _text(value, path)
    if text not in allowed:
        _fail(f"{path}: `{text}` is not allowed")
    return text


def _hex(value, path):
    text = _text(value, path)
    if len(text) != 64 or any(c not in "0123456789abcdef" for c in text):
        _fail(f"{path}: not a lowercase SHA-256")


def _ref_index(text, prefix, path):
    rest = text[len(prefix):] if text.startswith(prefix) else ""
    if not rest.isdigit() or rest != str(int(rest)) or int(rest) < 1 or not rest.isascii():
        _fail(f"{path}: `{text}` is not a `{prefix}<n>` reference")
    return int(rest)


class Vo:
    __slots__ = ("kind", "value", "unit", "status", "below", "exponents", "undefined")


def _vo(value, path, kinds):
    if not isinstance(value, dict):
        _fail(f"{path}: not a value object")
    status = _text(value.get("unit_status"), f"{path}/unit_status")
    keys = ["kind", "value", "unit", "unit_status", "below_normal", "undefined_reason"]
    if status == "catalog_family_absent":
        keys.append("dimension_exponents")
    _obj(value, path, keys)
    vo = Vo()
    vo.kind = _text(value["kind"], f"{path}/kind")
    if vo.kind not in kinds:
        _fail(f"{path}: kind `{vo.kind}` is not {kinds}")
    vo.undefined = None if value["undefined_reason"] is None else _text(value["undefined_reason"], f"{path}/undefined_reason")
    if value["value"] is None:
        if vo.undefined is None:
            _fail(f"{path}: a null value states its undefined_reason")
        vo.value = None
    else:
        if vo.undefined is not None:
            _fail(f"{path}: an undefined value is null")
        vo.value = _number(value["value"], f"{path}/value")
    vo.below = _flag(value["below_normal"], f"{path}/below_normal")
    if vo.below and vo.value != 0.0:
        _fail(f"{path}: a below-normal value is published as 0")
    vo.unit = None if value["unit"] is None else _text(value["unit"], f"{path}/unit")
    vo.status = status
    vo.exponents = None
    if status == "catalog_family_absent":
        e = _obj(value["dimension_exponents"], f"{path}/dimension_exponents", ("length", "mass", "time", "temperature", "angle"))
        for k in ("length", "mass", "time", "temperature", "angle"):
            if type(e[k]) is not int:
                _fail(f"{path}/dimension_exponents/{k}: not an integer")
        vo.exponents = [e["length"], e["mass"], e["time"], e["temperature"], e["angle"]]
    return vo


def _unit_check(vo, key, path, scope):
    rule = RULES.get(vo.kind, {}).get(key)
    if rule is None:
        _fail(f"{path}: modal-1 has no `{key}` rule for `{vo.kind}`")
    if rule["unit_status"] == "catalog":
        if vo.status != "catalog" or vo.unit != rule["unit"]:
            _fail(f"{path}: unit {vo.unit} ({vo.status}) is not modal-1's {rule['unit']}")
    elif rule["unit_status"] == "catalog_family_absent":
        e = rule["dimension_exponents"]
        expected = [e["length"], e["mass"], e["time"], e["temperature"], e["angle"]]
        if vo.status != "catalog_family_absent" or vo.unit is not None or vo.exponents != expected:
            _fail(f"{path}: modal-1 states no catalog family here")
        note = f"catalog_family_absent: `{vo.kind}` {key}"
        if note not in scope:
            scope.append(note)
    else:
        if vo.status != "equilibrated_basis" or vo.unit is not None:
            _fail(f"{path}: modal-1 states the equilibrated basis here")
        note = f"equilibrated_basis: `{vo.kind}`"
        if note not in scope:
            scope.append(note)


# --- Closure ------------------------------------------------------------------------------

def _identity(doc):
    _const(doc["document_kind"], "/document_kind", FAMILY)
    _const(doc["schema_version"], "/schema_version", SCHEMA_VERSION)
    sc = _obj(doc["semantic_contract"], "/semantic_contract", ("id", "sha256"))
    _const(sc["id"], "/semantic_contract/id", SEMANTICS_ID)
    _const(sc["sha256"], "/semantic_contract/sha256", SEMANTICS_SHA256)
    profile = _obj(doc["profile"], "/profile", ("profile_id", "limitations"))
    _const(profile["profile_id"], "/profile/profile_id", PROFILE_ID)
    if _list(profile["limitations"], "/profile/limitations") != list(LIMITATIONS):
        _fail("/profile/limitations: not the profile's two fixed limitations")
    _const(doc["canonical_profile"], "/canonical_profile", CANONICAL_PROFILE)
    admission = _obj(doc["admission"], "/admission", ("status", "public_engineering_result", "parameter_standing", "parameter_label"))
    _const(admission["status"], "/admission/status", "experimental")
    if admission["public_engineering_result"] is not False:
        _fail("/admission/public_engineering_result: no public engineering result")
    _const(admission["parameter_standing"], "/admission/parameter_standing", "test_local")
    if not _text(admission["parameter_label"], "/admission/parameter_label"):
        _fail("/admission/parameter_label: empty")
    run = _obj(doc["run"], "/run", ("run_id", "mode_id_scope", "solver_path"))
    _text(run["run_id"], "/run/run_id")
    _const(run["mode_id_scope"], "/run/mode_id_scope", "run_local")
    _const(run["solver_path"], "/run/solver_path", "dense_scrutiny")
    b = _obj(doc["binding"], "/binding", ("request_sha256", "model_sha256", "materials_sha256", "materials_member", "options",
                                          "undeclared_members", "state_case", "integrity_parameters_sha256", "integrity_parameters"))
    _hex(b["request_sha256"], "/binding/request_sha256")
    if b["model_sha256"] is not None:
        _hex(b["model_sha256"], "/binding/model_sha256")
    _hex(b["materials_sha256"], "/binding/materials_sha256")
    _one_of(b["materials_member"], "/binding/materials_member", ("authored", "absent"))
    if not isinstance(b["options"], dict):
        _fail("/binding/options: not an object")
    for key in b["options"]:
        if key not in OPTIONS:
            _fail(f"/binding/options: `{key}` is not a modal request option")
    for i, member in enumerate(_list(b["undeclared_members"], "/binding/undeclared_members")):
        _text(member, f"/binding/undeclared_members/{i}")
    if b["state_case"] is not None:
        s = _obj(b["state_case"], "/binding/state_case", ("case_id", "case_index", "case_sha256"))
        _text(s["case_id"], "/binding/state_case/case_id")
        _count(s["case_index"], "/binding/state_case/case_index")
        _hex(s["case_sha256"], "/binding/state_case/case_sha256")
    _hex(b["integrity_parameters_sha256"], "/binding/integrity_parameters_sha256")
    p = _obj(b["integrity_parameters"], "/binding/integrity_parameters", PARAMETERS)
    for field in PARAMETERS:
        path = f"/binding/integrity_parameters/{field}"
        if field in ("max_ql_iterations_per_eigenvalue", "sturm_max_candidates"):
            _count(p[field], path)
        elif field in ("sturm_gap_log_positions", "sturm_whole_spectrum_offsets"):
            for i, v in enumerate(_list(p[field], path)):
                _number(v, f"{path}/{i}")
        elif field == "identity_tolerance":
            t = _obj(p[field], path, ("relative", "absolute"))
            _number(t["relative"], f"{path}/relative")
            _number(t["absolute"], f"{path}/absolute")
        else:
            _number(p[field], path)


def _table(tables, name, ids):
    path = f"/tables/{name}"
    if name not in tables:
        _fail(f"{path}: missing")
    table = tables[name]
    status = _text(table.get("status") if isinstance(table, dict) else None, f"{path}/status")
    if status == "withheld":
        if name not in WITHHOLDABLE:
            _fail(f"{path}: only the authoritative payload tables may be withheld")
        _text(_obj(table, path, ("status", "reason"))["reason"], f"{path}/reason")
        return None
    _obj(table, path, ("status", "row_count", "digest", "rows"))
    _const(table["status"], f"{path}/status", "published")
    rows = _list(table["rows"], f"{path}/rows")
    if _count(table["row_count"], f"{path}/row_count") != len(rows):
        _fail(f"{path}/row_count: not the number of rows")
    checksums = []
    for i, row in enumerate(rows):
        rpath = f"{path}/rows/{i}"
        if not isinstance(row, dict):
            _fail(f"{rpath}: not an object")
        row_id = _text(row.get("row_id"), f"{rpath}/row_id")
        if _count(row.get("ordinal"), f"{rpath}/ordinal") != i:
            _fail(f"{rpath}/ordinal: not its position")
        if row_id in ids:
            _fail(f"{rpath}/row_id: `{row_id}` is not unique")
        ids.add(row_id)
        checksum = _text(row.get("checksum"), f"{rpath}/checksum")
        body = {k: v for k, v in row.items() if k != "checksum"}
        if sha256(body) != checksum:
            _fail(f"{rpath}/checksum: not the row's checksum")
        checksums.append(checksum)
    if sha256(checksums) != table["digest"]:
        _fail(f"{path}/digest: not the digest of its rows' checksums")
    return rows


def _fields(row, path, fields):
    return _obj(row, path, tuple(fields) + ("row_id", "ordinal", "checksum"))


def _id_is(row, path, expected):
    if row["row_id"] != expected:
        _fail(f"{path}/row_id: expected `{expected}`")


class Parsed:
    pass


def _closure(doc):
    _obj(doc, "", SOLVED_KEYS)
    _identity(doc)
    _const(doc["status"], "/status", "solved")
    p = Parsed()
    p.unitized = []
    p.params = doc["binding"]["integrity_parameters"]

    def unitize(vo, key, path):
        p.unitized.append((vo, key, path))
        return vo

    tables = _obj(doc["tables"], "/tables", TABLES)
    ids = set()

    def get(name):
        return _table(tables, name, ids)

    node_rows = get("nodes")
    node_index, node_ids, p.nodes = {}, [], []
    for i, row in enumerate(node_rows):
        path = f"/tables/nodes/rows/{i}"
        r = _fields(row, path, ("node_id", "x", "y", "z"))
        node_id = _text(r["node_id"], f"{path}/node_id")
        _id_is(r, path, f"node:{node_id}")
        coordinates = []
        for axis in ("x", "y", "z"):
            vo = unitize(_vo(r[axis], f"{path}/{axis}", ("node_coordinate",)), "fixed", f"{path}/{axis}")
            coordinates.append(vo.value)
        node_index[f"node:{node_id}"] = i
        node_ids.append(node_id)
        p.nodes.append(coordinates)
    ndof = 6 * len(p.nodes)
    dof_rows = get("dofs")
    if len(dof_rows) != ndof:
        _fail("/tables/dofs: not six DOFs per node")
    dof_index, p.free = {}, []
    for i, row in enumerate(dof_rows):
        path = f"/tables/dofs/rows/{i}"
        r = _fields(row, path, ("node_ref", "component", "free_index"))
        node_ref = _text(r["node_ref"], f"{path}/node_ref")
        if node_ref not in node_index:
            _fail(f"{path}/node_ref: no such node")
        node = node_index[node_ref]
        component = _one_of(r["component"], f"{path}/component", COMPONENTS)
        if 6 * node + COMPONENTS.index(component) != i:
            _fail(f"{path}: not DOF {i} in node-major order")
        dof_id = f"dof:{node_ids[node]}:{component}"
        _id_is(r, path, dof_id)
        dof_index[dof_id] = i
        p.free.append(None if r["free_index"] is None else _count(r["free_index"], f"{path}/free_index"))
    expected_free = 0
    for i, f in enumerate(p.free):
        if f is not None:
            if f != expected_free:
                _fail(f"/tables/dofs/rows/{i}/free_index: free indices ascend from 0 in DOF order")
            expected_free += 1

    def dof_of(value, path):
        text = _text(value, path)
        if text not in dof_index:
            _fail(f"{path}: `{text}` is no DOF")
        return dof_index[text]

    rows = get("mass_matrix")
    if rows is None:
        p.mass = None
    else:
        p.mass, seen = [], set()
        for i, row in enumerate(rows):
            path = f"/tables/mass_matrix/rows/{i}"
            r = _fields(row, path, ("row_dof_ref", "col_dof_ref", "pair_class", "value"))
            a = dof_of(r["row_dof_ref"], f"{path}/row_dof_ref")
            b = dof_of(r["col_dof_ref"], f"{path}/col_dof_ref")
            if a > b or (a, b) in seen:
                _fail(f"{path}: entries are distinct upper positions")
            seen.add((a, b))
            ta, tb = a % 6 < 3, b % 6 < 3
            pair = "translation_translation" if ta and tb else ("rotation_rotation" if not ta and not tb else "translation_rotation")
            _const(r["pair_class"], f"{path}/pair_class", pair)
            _id_is(r, path, f"mass:{r['row_dof_ref']}|{r['col_dof_ref']}")
            vo = unitize(_vo(r["value"], f"{path}/value", ("mass_matrix_entry",)), pair, f"{path}/value")
            p.mass.append((a, b, vo.value))
    total_rows = get("mass_totals")
    if len(total_rows) != 3:
        _fail("/tables/mass_totals: one row per direction")
    p.totals, p.free_totals = [None] * 3, [None] * 3
    for k, row in enumerate(total_rows):
        path = f"/tables/mass_totals/rows/{k}"
        r = _fields(row, path, ("direction", "basis_ref", "total_mass", "free_dof_mass"))
        _const(r["direction"], f"{path}/direction", DIRECTIONS[k])
        _id_is(r, path, f"mass-total:{DIRECTIONS[k]}")
        if r["basis_ref"] != {"ref_type": "model", "ref_id": "model"}:
            _fail(f"{path}/basis_ref: the global mass is on the model basis")
        p.totals[k] = unitize(_vo(r["total_mass"], f"{path}/total_mass", ("modal_total_mass",)), "fixed", f"{path}/total_mass").value
        p.free_totals[k] = unitize(_vo(r["free_dof_mass"], f"{path}/free_dof_mass", ("modal_free_dof_mass",)), "fixed",
                                   f"{path}/free_dof_mass").value

    computed_rows = get("computed_modes")
    cluster_rows = get("clusters")
    mode_rows = get("modes")
    n = len(mode_rows)
    p.computed = []
    for i, row in enumerate(computed_rows):
        path = f"/tables/computed_modes/rows/{i}"
        r = _fields(row, path, ("computed_index", "cluster_ref", "reported", "mode_ref", "band_recovered", "circular_frequency",
                                "frequency", "backward_error", "condition", "error_estimate"))
        if _count(r["computed_index"], f"{path}/computed_index") != i + 1:
            _fail(f"{path}/computed_index: not {i + 1}")
        _id_is(r, path, f"computed:{i + 1}")
        cluster = _ref_index(_text(r["cluster_ref"], f"{path}/cluster_ref"), "cluster:", f"{path}/cluster_ref")
        if cluster > len(cluster_rows):
            _fail(f"{path}/cluster_ref: no such cluster")
        reported = _flag(r["reported"], f"{path}/reported")
        mode = None
        if r["mode_ref"] is not None:
            mode = _ref_index(_text(r["mode_ref"], f"{path}/mode_ref"), "mode:", f"{path}/mode_ref")
            if mode > n:
                _fail(f"{path}/mode_ref: no such mode")
        _flag(r["band_recovered"], f"{path}/band_recovered")
        values = []
        for field, kind in (("circular_frequency", "computed_circular_frequency"), ("frequency", "computed_frequency"),
                            ("backward_error", "residual_backward_error"), ("condition", "eigenvalue_condition_estimate"),
                            ("error_estimate", "eigenvalue_error_estimate")):
            values.append(unitize(_vo(r[field], f"{path}/{field}", (kind,)), "fixed", f"{path}/{field}").value)
        p.computed.append({"cluster": cluster, "reported": reported, "mode": mode, "omega": values[0], "frequency": values[1],
                           "eta": values[2], "condition": values[3], "estimate": values[4]})
    p.clusters = []
    for i, row in enumerate(cluster_rows):
        path = f"/tables/clusters/rows/{i}"
        r = _fields(row, path, ("cluster_index", "first_computed_ref", "last_computed_ref", "reported", "error_estimate"))
        if _count(r["cluster_index"], f"{path}/cluster_index") != i + 1:
            _fail(f"{path}/cluster_index: not {i + 1}")
        _id_is(r, path, f"cluster:{i + 1}")
        first = _ref_index(_text(r["first_computed_ref"], f"{path}/first_computed_ref"), "computed:", f"{path}/first_computed_ref")
        last = _ref_index(_text(r["last_computed_ref"], f"{path}/last_computed_ref"), "computed:", f"{path}/last_computed_ref")
        if first > last or last > len(p.computed):
            _fail(f"{path}: not a run of computed modes")
        reported = _flag(r["reported"], f"{path}/reported")
        vo = unitize(_vo(r["error_estimate"], f"{path}/error_estimate", ("cluster_error_estimate",)), "fixed", f"{path}/error_estimate")
        p.clusters.append((first, last, reported, vo.value))
    p.modes = []
    for i, row in enumerate(mode_rows):
        path = f"/tables/modes/rows/{i}"
        r = _fields(row, path, ("mode_index", "computed_ref", "cluster_ref", "normalization_case", "reference_dof_ref", "standing",
                                "circular_frequency", "frequency", "period", "generalized_mass", "cluster_error_estimate"))
        if _count(r["mode_index"], f"{path}/mode_index") != i + 1:
            _fail(f"{path}/mode_index: not {i + 1}")
        _id_is(r, path, f"mode:{i + 1}")
        _const(r["computed_ref"], f"{path}/computed_ref", f"computed:{i + 1}")
        cluster = _ref_index(_text(r["cluster_ref"], f"{path}/cluster_ref"), "cluster:", f"{path}/cluster_ref")
        case = CASES.index(_one_of(r["normalization_case"], f"{path}/normalization_case", CASES))
        reference = dof_of(r["reference_dof_ref"], f"{path}/reference_dof_ref")
        s = _obj(r["standing"], f"{path}/standing", ("token", "reasons"))
        token = _one_of(s["token"], f"{path}/standing/token", ("checks_passed", "sensitive"))
        reasons = _list(s["reasons"], f"{path}/standing/reasons")
        for j, reason in enumerate(reasons):
            _text(reason, f"{path}/standing/reasons/{j}")

        def read(field, kind, key):
            return unitize(_vo(r[field], f"{path}/{field}", (kind,)), key, f"{path}/{field}")

        mode = {"case": case, "reference": reference, "sensitive": token == "sensitive", "reasons": len(reasons),
                "omega": read("circular_frequency", "modal_circular_frequency", "fixed").value,
                "frequency": read("frequency", "modal_frequency", "fixed").value,
                "period": read("period", "modal_period", "fixed").value,
                "generalized_mass": read("generalized_mass", "modal_generalized_mass", CASES[case]),
                "cluster_estimate": read("cluster_error_estimate", "modal_cluster_error_estimate", "fixed").value,
                "cluster": cluster}
        p.modes.append(mode)
    rows = get("shapes")
    if rows is None:
        p.shapes = None
    else:
        if len(rows) != n * ndof:
            _fail("/tables/shapes: one row per mode and DOF")
        p.shapes = [[None] * ndof for _ in range(n)]
        for i, row in enumerate(rows):
            path = f"/tables/shapes/rows/{i}"
            m, dof = i // max(ndof, 1), i % max(ndof, 1)
            r = _fields(row, path, ("mode_ref", "dof_ref", "component", "value"))
            _const(r["mode_ref"], f"{path}/mode_ref", f"mode:{m + 1}")
            if dof_of(r["dof_ref"], f"{path}/dof_ref") != dof:
                _fail(f"{path}/dof_ref: not DOF {dof} of mode {m + 1}")
            _const(r["component"], f"{path}/component", COMPONENTS[dof % 6])
            _id_is(r, path, f"shape:mode:{m + 1}:{r['dof_ref']}")
            kind = "mode_shape_translation" if dof % 6 < 3 else "mode_shape_rotation"
            vo = unitize(_vo(r["value"], f"{path}/value", (kind,)), CASES[p.modes[m]["case"]], f"{path}/value")
            p.shapes[m][dof] = vo.value
    participation_rows = get("participation")
    if len(participation_rows) != 3 * n:
        _fail("/tables/participation: one row per mode and direction")
    p.participation = []
    for m in range(n):
        triple = []
        for k in range(3):
            i = 3 * m + k
            path = f"/tables/participation/rows/{i}"
            r = _fields(participation_rows[i], path, ("mode_ref", "direction", "participation_factor", "effective_mass",
                                                      "effective_mass_fraction", "cumulative_effective_mass_fraction"))
            _const(r["mode_ref"], f"{path}/mode_ref", f"mode:{m + 1}")
            _const(r["direction"], f"{path}/direction", DIRECTIONS[k])
            _id_is(r, path, f"participation:mode:{m + 1}:{DIRECTIONS[k]}")

            def read(field, kind, key):
                return unitize(_vo(r[field], f"{path}/{field}", (kind,)), key, f"{path}/{field}")

            triple.append({"gamma": read("participation_factor", "modal_participation_factor", CASES[p.modes[m]["case"]]),
                           "effective": read("effective_mass", "modal_effective_mass", "fixed"),
                           "fraction": read("effective_mass_fraction", "modal_effective_mass_fraction", "fixed"),
                           "cumulative": read("cumulative_effective_mass_fraction", "modal_cumulative_effective_mass_fraction", "fixed")})
        p.participation.append(triple)
    p.candidates = []
    for i, row in enumerate(get("sturm_candidates")):
        path = f"/tables/sturm_candidates/rows/{i}"
        r = _fields(row, path, ("candidate_index", "decision", "accepted", "negative_pivot_count", "shift", "formation_term",
                                "formation_separation_ratio", "factor_term", "backward_error", "inverse_norm_estimate",
                                "conditioning_product", "separation_ratio", "growth", "max_abs_lower_factor", "max_abs_pivot",
                                "max_abs_scaled_diagonal"))
        if _count(r["candidate_index"], f"{path}/candidate_index") != i + 1:
            _fail(f"{path}/candidate_index: not {i + 1}")
        _id_is(r, path, f"sturm-candidate:{i + 1}")
        decision = _one_of(r["decision"], f"{path}/decision", DECISIONS)
        if _flag(r["accepted"], f"{path}/accepted") != (decision == "accepted"):
            _fail(f"{path}/accepted: not its decision")
        pivots = None if r["negative_pivot_count"] is None else _count(r["negative_pivot_count"], f"{path}/negative_pivot_count")

        def read(field, kind):
            return unitize(_vo(r[field], f"{path}/{field}", (kind,)), "fixed", f"{path}/{field}")

        c = {"decision": decision, "pivots": pivots, "shift": read("shift", "sturm_shift_circular_frequency").value,
             "formation_term": read("formation_term", "sturm_formation_term").value}
        read("formation_separation_ratio", "sturm_formation_separation_ratio")
        c["factor_term"] = read("factor_term", "sturm_factor_term").value
        c["backward_error"] = read("backward_error", "sturm_backward_error").value
        c["inverse_norm"] = read("inverse_norm_estimate", "sturm_inverse_norm_estimate").value
        c["product_vo"] = read("conditioning_product", "sturm_conditioning_product")
        c["product"] = c["product_vo"].value
        c["separation"] = read("separation_ratio", "sturm_separation_ratio")
        c["growth_vo"] = read("growth", "sturm_growth_factor")
        c["growth"] = c["growth_vo"].value
        read("max_abs_lower_factor", "sturm_max_abs_lower_factor")
        read("max_abs_pivot", "sturm_max_abs_pivot")
        read("max_abs_scaled_diagonal", "sturm_max_abs_scaled_diagonal")
        p.candidates.append(c)
    p.identity = []
    for i, row in enumerate(get("identity")):
        path = f"/tables/identity/rows/{i}"
        r = _fields(row, path, ("subject", "axis", "comparison", "relative"))
        _id_is(r, path, f"identity:{i}")
        relative = unitize(_vo(r["relative"], f"{path}/relative", ("identity_relative_discrepancy",)), "fixed", f"{path}/relative")
        p.identity.append({"subject": _text(r["subject"], f"{path}/subject"),
                           "axis": _one_of(r["axis"], f"{path}/axis", DIRECTIONS),
                           "comparison": _one_of(r["comparison"], f"{path}/comparison", ("nodal", "resultant")),
                           "relative": relative.value})
    p.support_states = {}
    for i, row in enumerate(get("supports")):
        path = f"/tables/supports/rows/{i}"
        r = _fields(row, path, ("support_id", "behavior", "dof_ref", "state", "treatment"))
        _id_is(r, path, f"support:{_text(r['support_id'], f'{path}/support_id')}")
        _text(r["behavior"], f"{path}/behavior")
        dof = dof_of(r["dof_ref"], f"{path}/dof_ref")
        _one_of(r["state"], f"{path}/state", ("active", "inactive", "sticking", "sliding"))
        p.support_states[r["support_id"]] = (dof, _one_of(r["treatment"], f"{path}/treatment",
                                                          ("restrained", "already_restrained", "free", "free_frozen_friction")))
    p.straight_members = []
    for i, row in enumerate(get("discretization")):
        path = f"/tables/discretization/rows/{i}"
        r = _fields(row, path, ("member_ref", "length", "fixed_fixed_frequency", "ratio_to_highest_reported"))
        _id_is(r, path, f"member:{_text(r['member_ref'], f'{path}/member_ref')}")
        p.straight_members.append(r["member_ref"])
        for field, kind in (("length", "member_length"), ("fixed_fixed_frequency", "member_fixed_fixed_frequency"),
                            ("ratio_to_highest_reported", "member_frequency_ratio")):
            unitize(_vo(r[field], f"{path}/{field}", (kind,)), "fixed", f"{path}/{field}")
    p.disclosure_texts = {}
    for i, row in enumerate(get("disclosures")):
        path = f"/tables/disclosures/rows/{i}"
        r = _fields(row, path, ("text", "source"))
        _id_is(r, path, f"disclosure:{i}")
        p.disclosure_texts.setdefault(_one_of(r["source"], f"{path}/source", ("modal", "weight_basis")), []).append(
            _text(r["text"], f"{path}/text"))
    for i, row in enumerate(get("diagnostics")):
        _diagnostic_row(row, f"/tables/diagnostics/rows/{i}", f"diagnostic:{i}", ("modal", "state_case_solve"))

    def header(value, path, kind):
        return unitize(_vo(value, path, (kind,)), "fixed", path)

    basis = _obj(doc["basis"], "/basis", ("route", "no_translation_threshold", "bounding_box_diagonal", "weight_to_mass_acceleration",
                                          "identity_acceleration", "mass_center", "state_case"))
    _one_of(basis["route"], "/basis/route", ("pressure_free", "exact_pressure"))
    p.tau_t = header(basis["no_translation_threshold"], "/basis/no_translation_threshold", "basis_no_translation_threshold").value
    p.ell = header(basis["bounding_box_diagonal"], "/basis/bounding_box_diagonal", "basis_bounding_box_diagonal").value
    if basis["weight_to_mass_acceleration"] is not None:
        header(basis["weight_to_mass_acceleration"], "/basis/weight_to_mass_acceleration", "request_weight_to_mass_acceleration")
    header(basis["identity_acceleration"], "/basis/identity_acceleration", "identity_acceleration")
    if basis["mass_center"] is not None:
        center = _obj(basis["mass_center"], "/basis/mass_center", ("x", "y", "z"))
        for axis in ("x", "y", "z"):
            header(center[axis], f"/basis/mass_center/{axis}", "basis_mass_center")
    p.linearized_restrained = None
    if basis["state_case"] is not None:
        s = _obj(basis["state_case"], "/basis/state_case", ("case_id", "case_index", "linearized", "solver_mode", "restrained_dof_refs"))
        _text(s["case_id"], "/basis/state_case/case_id")
        _count(s["case_index"], "/basis/state_case/case_index")
        if _flag(s["linearized"], "/basis/state_case/linearized"):
            _const(s["solver_mode"], "/basis/state_case/solver_mode", "dense_scrutiny")
            p.linearized_restrained = [dof_of(ref, f"/basis/state_case/restrained_dof_refs/{i}")
                                       for i, ref in enumerate(_list(s["restrained_dof_refs"], "/basis/state_case/restrained_dof_refs"))]
        elif s["solver_mode"] is not None or s["restrained_dof_refs"] is not None:
            _fail("/basis/state_case: a case that linearizes no support states no mode or restrained set")
    x = _obj(doc["extraction"], "/extraction", ("mode_count", "frequency_cutoff", "stopped_by", "bounded_count", "extension_count",
                                                "count", "computed_count", "endpoint"))
    p.mode_count = None if x["mode_count"] is None else _count(x["mode_count"], "/extraction/mode_count")
    p.cutoff = None if x["frequency_cutoff"] is None else header(x["frequency_cutoff"], "/extraction/frequency_cutoff",
                                                                  "request_frequency_cutoff").value
    if p.mode_count is None and x["frequency_cutoff"] is None:
        _fail("/extraction: a modal result states its bound")
    p.stopped_by = _one_of(x["stopped_by"], "/extraction/stopped_by", ("mode_count", "frequency_cutoff",
                                                                        "mode_count_and_frequency_cutoff", "spectrum_exhausted"))
    e = _obj(x["endpoint"], "/extraction/endpoint", ("kind", "next_computed_ref", "established_finite_mode_count"))
    p.endpoint = _one_of(e["kind"], "/extraction/endpoint/kind", ("below_computed_cluster", "below_lowest_mode", "whole_finite_spectrum"))
    p.next_computed = None if e["next_computed_ref"] is None else _ref_index(
        _text(e["next_computed_ref"], "/extraction/endpoint/next_computed_ref"), "computed:", "/extraction/endpoint/next_computed_ref")
    p.established_endpoint = None if e["established_finite_mode_count"] is None else _count(
        e["established_finite_mode_count"], "/extraction/endpoint/established_finite_mode_count")
    c = _obj(doc["census"], "/census", ("kind", "finite_mode_count", "structurally_massless", "nonpositive_mu", "rejected_mu",
                                        "mass_block_established"))
    kind = _one_of(c["kind"], "/census/kind", ("established", "unresolved"))
    _count(c["structurally_massless"], "/census/structurally_massless")
    if kind == "established":
        for k in ("nonpositive_mu", "rejected_mu", "mass_block_established"):
            if c[k] is not None:
                _fail(f"/census/{k}: an established census states none")
        p.census_established = _count(c["finite_mode_count"], "/census/finite_mode_count")
    else:
        if c["finite_mode_count"] is not None:
            _fail("/census/finite_mode_count: an unresolved census states none")
        _count(c["nonpositive_mu"], "/census/nonpositive_mu")
        _count(c["rejected_mu"], "/census/rejected_mu")
        _flag(c["mass_block_established"], "/census/mass_block_established")
        p.census_established = None
    st = _obj(doc["standing"], "/standing", ("token", "reasons"))
    p.sensitive = _one_of(st["token"], "/standing/token", ("checks_passed", "sensitive")) == "sensitive"
    set_reasons = _list(st["reasons"], "/standing/reasons")
    for i, reason in enumerate(set_reasons):
        _text(reason, f"/standing/reasons/{i}")
    p.set_reasons = len(set_reasons)
    s = _obj(doc["sturm"], "/sturm", ("count", "reported", "placement", "conditioning_multiple", "conditioning_limit",
                                      "separation_multiple", "growth_limit"))
    pl = _obj(s["placement"], "/sturm/placement", ("kind", "lower", "upper", "established_finite_mode_count"))
    p.placement = _one_of(pl["kind"], "/sturm/placement/kind", ("given", "gap", "above_spectrum"))
    p.placement_lower = None if pl["lower"] is None else header(pl["lower"], "/sturm/placement/lower", "placement_circular_frequency").value
    p.placement_upper = None if pl["upper"] is None else header(pl["upper"], "/sturm/placement/upper", "placement_circular_frequency").value
    p.placement_established = None if pl["established_finite_mode_count"] is None else _count(
        pl["established_finite_mode_count"], "/sturm/placement/established_finite_mode_count")
    p.kappa_c = header(s["conditioning_multiple"], "/sturm/conditioning_multiple", "sturm_premise_parameter").value
    p.limit = header(s["conditioning_limit"], "/sturm/conditioning_limit", "sturm_premise_parameter").value
    p.kappa_s = header(s["separation_multiple"], "/sturm/separation_multiple", "sturm_premise_parameter").value
    p.growth_limit = header(s["growth_limit"], "/sturm/growth_limit", "sturm_premise_parameter").value
    o = _obj(doc["orthogonality"], "/orthogonality", ("max_ratio", "pair"))
    p.ortho = header(o["max_ratio"], "/orthogonality/max_ratio", "orthogonality_max_ratio").value
    p.ortho_pair = o["pair"] is not None
    if p.ortho_pair:
        pair = _list(o["pair"], "/orthogonality/pair")
        if len(pair) != 2:
            _fail("/orthogonality/pair: two modes")
        a = _ref_index(_text(pair[0], "/orthogonality/pair/0"), "mode:", "/orthogonality/pair/0")
        b = _ref_index(_text(pair[1], "/orthogonality/pair/1"), "mode:", "/orthogonality/pair/1")
        if not (a < b <= n):
            _fail("/orthogonality/pair: two reported modes, ascending")
    f = _obj(doc["formation"], "/formation", ("stiffness_difference", "stiffness_row_ref", "stiffness_off_pattern", "mass_difference",
                                              "mass_row_ref", "mass_off_pattern", "reformed_stiffness_norm", "reformed_mass_norm",
                                              "section"))
    p.formation = []
    for field, kind in (("stiffness_difference", "formation_difference"), ("mass_difference", "formation_difference"),
                        ("reformed_stiffness_norm", "reformed_norm"), ("reformed_mass_norm", "reformed_norm")):
        p.formation.append(header(f[field], f"/formation/{field}", kind).value)
    for field in ("stiffness_row_ref", "mass_row_ref"):
        if f[field] is not None:
            dof_of(f[field], f"/formation/{field}")
    _count(f["stiffness_off_pattern"], "/formation/stiffness_off_pattern")
    _count(f["mass_off_pattern"], "/formation/mass_off_pattern")
    if f["section"] is not None:
        sec = _obj(f["section"], "/formation/section", ("area", "second_moment", "torsion_constant"))
        for field in ("area", "second_moment", "torsion_constant"):
            p.formation.append(header(sec[field], f"/formation/section/{field}", "section_derivation_difference").value)
    g = _obj(doc["guard"], "/guard", ("global_dimension", "free_dimension", "static_estimate_bytes", "ceiling_bytes",
                                      "eigen_core_bytes", "operation_estimate", "statement"))
    if _count(g["global_dimension"], "/guard/global_dimension") != ndof or _count(g["free_dimension"], "/guard/free_dimension") != expected_free:
        _fail("/guard: not the document's DOF counts")
    for field in ("static_estimate_bytes", "ceiling_bytes", "eigen_core_bytes", "operation_estimate"):
        text = _text(g[field], f"/guard/{field}")
        if not text or not (text.isascii() and text.isdigit()):
            _fail(f"/guard/{field}: a decimal integer")
    _text(g["statement"], "/guard/statement")
    for i, mode in enumerate(p.modes):
        if mode["cluster"] > len(p.clusters):
            _fail(f"/tables/modes/rows/{i}/cluster_ref: no such cluster")
    p.bounded = _count(x["bounded_count"], "/extraction/bounded_count")
    p.extension = _count(x["extension_count"], "/extraction/extension_count")
    p.count = _count(x["count"], "/extraction/count")
    p.computed_count = _count(x["computed_count"], "/extraction/computed_count")
    p.sturm_count = _count(s["count"], "/sturm/count")
    p.sturm_reported = _count(s["reported"], "/sturm/reported")
    return p


def _diagnostic_row(row, path, row_id, sources):
    r = _fields(row, path, ("diagnostic_id", "code", "severity", "message", "affected_refs", "source"))
    _id_is(r, path, row_id)
    _text(r["diagnostic_id"], f"{path}/diagnostic_id")
    code = _text(r["code"], f"{path}/code")
    if not code or any(not (ch.isascii() and (ch.isupper() or ch.isdigit() or ch == "_")) for ch in code):
        _fail(f"{path}/code: an upper-case code")
    severity = _text(r["severity"], f"{path}/severity")
    _text(r["message"], f"{path}/message")
    for i, ref in enumerate(_list(r["affected_refs"], f"{path}/affected_refs")):
        _text(ref, f"{path}/affected_refs/{i}")
    _one_of(r["source"], f"{path}/source", sources)
    return severity, code


# --- Binding ------------------------------------------------------------------------------

def _binding(doc, request):
    b = doc["binding"]
    if sha256(b["integrity_parameters"]) != b["integrity_parameters_sha256"]:
        _fail("/binding/integrity_parameters_sha256: not the parameters' hash")
    run = sha256({"request_sha256": b["request_sha256"], "integrity_parameters_sha256": b["integrity_parameters_sha256"]}) or ""
    if doc["run"]["run_id"] != f"modal-run:{run[:16]}":
        _fail("/run/run_id: not the binding's run id")
    params, options = b["integrity_parameters"], b["options"]
    if doc["status"] == "solved":
        if doc["basis"]["no_translation_threshold"]["value"] != params["no_translation_threshold"]:
            _fail("/basis/no_translation_threshold: not the bound τ_t")
        for field, param in (("conditioning_multiple", "sturm_conditioning_multiple"), ("separation_multiple", "sturm_separation_multiple"),
                             ("growth_limit", "sturm_growth_limit")):
            if doc["sturm"][field]["value"] != params[param]:
                _fail(f"/sturm/{field}: not the bound {param}")
        if doc["extraction"]["mode_count"] != options.get("mode_count"):
            _fail("/extraction/mode_count: not the request's")
        for option, header, dimension in (("frequency_cutoff", doc["extraction"]["frequency_cutoff"], "frequency"),
                                          ("weight_to_mass_acceleration", doc["basis"]["weight_to_mass_acceleration"], "acceleration")):
            if option not in options and header is None:
                continue
            if option in options and header is not None:
                q = options[option]
                if not isinstance(q, dict) or not _is_number(q.get("value")) or not isinstance(q.get("unit"), str):
                    _fail(f"/binding/options/{option}: not a quantity")
                try:
                    converted = units.to_canonical(q["value"], q["unit"], dimension)
                except ValueError:
                    _fail(f"/binding/options/{option}: not convertible")
                published = header["value"] if _is_number(header.get("value")) else math.nan
                if not abs(published - converted) <= 4.0 * U * abs(converted):
                    _fail(f"{option}: the published value is not the request's")
            else:
                _fail(f"{option}: stated in only one of the request and the document")
        bound, basis_case = b["state_case"], doc["basis"]["state_case"]
        if (bound is None) != (basis_case is None) or (bound is not None and (bound["case_id"] != basis_case["case_id"]
                                                                          or bound["case_index"] != basis_case["case_index"])):
            _fail("/basis/state_case: not the bound state case")
        if (bound is None) != ("state_case" not in options) or (bound is not None and options["state_case"] != bound["case_id"]):
            _fail("/binding/state_case: not the request's state_case")
    not_checked = []
    if request is None:
        not_checked.append("request")
    else:
        if sha256(request) != b["request_sha256"]:
            _fail("/binding/request_sha256: not the supplied request's")
        model = sha256(request["model"]) if "model" in request else None
        if model != b["model_sha256"]:
            _fail("/binding/model_sha256: not the supplied model's")
        if sha256(request.get("materials", [])) != b["materials_sha256"]:
            _fail("/binding/materials_sha256: not the supplied materials'")
        for member in OPTIONS:
            if request.get(member) != options.get(member) or (member in request) != (member in options):
                _fail(f"/binding/options/{member}: not the supplied request's")
        if b["state_case"] is not None:
            cases = request.get("model", {}).get("load_cases", [])
            index = b["state_case"]["case_index"]
            case = cases[index] if type(index) is int and 0 <= index < len(cases) else None
            if case is None or case.get("id") != b["state_case"]["case_id"] or sha256(case) != b["state_case"]["case_sha256"]:
                _fail("/binding/state_case: not the supplied model's case")
    return not_checked


# --- Relations ----------------------------------------------------------------------------

def _close(published, derived, tolerance):
    return abs(published - derived) <= tolerance


def _mass(p):
    scope = []
    for k in range(3):
        total = p.totals[k]
        if total is None:
            _fail(f"T_{DIRECTIONS[k]}: not published")
        if total < 0.0:
            _fail(f"T_{DIRECTIONS[k]}: negative")
        total_sum, count = 0.0, 0
        for m in range(len(p.modes)):
            e = p.participation[m][k]["effective"].value
            if e is not None:
                total_sum += e
                count += 1
        if total_sum > total * (1.0 + ALLOW * float(count + 2) * U):
            _fail(f"Σ m_eff,{DIRECTIONS[k]} exceeds T_{DIRECTIONS[k]} (Bessel's inequality)")
    if p.mass is None:
        scope.append("mass_matrix withheld: T_k and the free-DOF mass are not re-derived")
        return scope
    for i, j, v in p.mass:
        if v is None:
            _fail("a mass entry is not published")
        if i == j and v < 0.0:
            _fail(f"mass diagonal at DOF {i} is negative")
    for k in range(3):
        s, a, n, fs, fa, fn = 0.0, 0.0, 0, 0.0, 0.0, 0
        for i, j, v in p.mass:
            if i % 6 == k and j % 6 == k:
                w = 1.0 if i == j else 2.0
                term = w * v
                s += term
                a += abs(term)
                n += 1
                if p.free[i] is not None and p.free[j] is not None:
                    fs += term
                    fa += abs(term)
                    fn += 1
        if not _close(p.totals[k], s, ALLOW * float(n + 4) * U * a):
            _fail(f"T_{DIRECTIONS[k]} is not R_kᵀ M R_k of the mass entries")
        if p.free_totals[k] is None:
            _fail(f"free-DOF mass {DIRECTIONS[k]}: not published")
        if not _close(p.free_totals[k], fs, ALLOW * float(fn + 4) * U * fa):
            _fail(f"free-DOF mass {DIRECTIONS[k]} is not R_k,fᵀ M_ff R_k,f")
    return scope


def _diagonal(p):
    low, high = [math.inf] * 3, [-math.inf] * 3
    for node in p.nodes:
        for k in range(3):
            c = node[k]
            if c is None:
                _fail("a node coordinate is not published")
            if c < low[k]:
                low[k] = c
            if c > high[k]:
                high[k] = c
    d = [high[0] - low[0], high[1] - low[1], high[2] - low[2]]
    return math.sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2])


def _normalization(p):
    ell = p.ell
    if ell is None:
        _fail("ℓ: not published")
    derived = _diagonal(p)
    if not (ell > 0.0 and _close(ell, derived, 8.0 * U * derived)):
        _fail("ℓ is not the nodes' bounding-box diagonal")
    tau = p.tau_t
    if tau is None:
        _fail("τ_t: not published")
    if p.shapes is None:
        return ["shapes withheld: the normalization cases and signs are not re-derived"]
    for m, mode in enumerate(p.modes):
        shape, label = p.shapes[m], m + 1
        if p.free[mode["reference"]] is None:
            _fail(f"mode {label}: its reference DOF is restrained")
        if (mode["reference"] % 6 < 3) != (mode["case"] == 0):
            _fail(f"mode {label}: its reference DOF is not of its case's kind")
        if shape[mode["reference"]] != 1.0:
            _fail(f"mode {label}: its reference component is not exactly +1")
        t, theta = 0.0, 0.0
        for dof, v in enumerate(shape):
            if v is None:
                _fail(f"mode {label}: a shape component is not published")
            if p.free[dof] is None:
                if v != 0.0:
                    _fail(f"mode {label}: restrained DOF {dof} is not exactly 0")
                continue
            if dof % 6 < 3:
                t = max(t, abs(v))
            else:
                theta = max(theta, abs(v))
        if mode["case"] == 0:
            if t > 1.0:
                _fail(f"mode {label}: a translation exceeds its unit maximum")
            if tau * ell * theta * (1.0 - 8.0 * U) >= 1.0:
                _fail(f"mode {label}: published in case (a), but its translation is at most τ_t·ℓ times its rotation")
        else:
            if theta > 1.0:
                _fail(f"mode {label}: a rotation exceeds its unit maximum")
            if t > tau * ell * (1.0 + 8.0 * U):
                _fail(f"mode {label}: published in case (b), but a translation exceeds τ_t·ℓ")
    return []


TINY = 2.0 ** -1022  # the smallest normal binary64


def _subnormal_possible(lowest_magnitude, scale=1.0):
    """Whether a quantity whose magnitude is at least `lowest_magnitude` (times `scale`)
    can still lie below the normal range: the claim a below-normal flag makes."""
    return lowest_magnitude <= TINY * scale * (1.0 + ALLOW * U)


def _payload_quantities(p, m):
    """φᵀM_ffφ and φᵀp_k of mode m from the payload, each with the absolute sum of its
    terms and its term count (the rounding allowance's inputs)."""
    phi = [math.nan if v is None else v for v in p.shapes[m]]
    gm = {"value": 0.0, "abs": 0.0, "terms": 0}
    excitation = [{"value": 0.0, "abs": 0.0, "terms": 0} for _ in range(3)]

    def add(acc, term):
        acc["value"] += term
        acc["abs"] += abs(term)
        acc["terms"] += 1

    for i, j, v in p.mass:
        free_i, free_j = p.free[i] is not None, p.free[j] is not None
        if free_i and free_j:
            add(gm, (1.0 if i == j else 2.0) * (phi[i] * v * phi[j]))
        if free_i and j % 6 < 3:
            add(excitation[j % 6], phi[i] * v)
        if i != j and free_j and i % 6 < 3:
            add(excitation[i % 6], phi[j] * v)
    return gm, excitation


def _participation(p):
    scope, cumulative = [], [0.0, 0.0, 0.0]
    for m, mode in enumerate(p.modes):
        label, gm_vo = m + 1, mode["generalized_mass"]
        mass_value = gm_vo.value
        if mass_value is None:
            _fail(f"mode {label}: m not published")
        if not (mass_value > 0.0 or gm_vo.below):
            _fail(f"mode {label}: m is not positive")
        for k in range(3):
            row = p.participation[m][k]
            gamma, effective = row["gamma"].value, row["effective"].value
            if gamma is None:
                _fail(f"mode {label}: Γ not published")
            if effective is None:
                _fail(f"mode {label}: m_eff not published")
            if effective < 0.0:
                _fail(f"mode {label}: m_eff is negative")
            flagged = row["gamma"].below or row["effective"].below or gm_vo.below
            g2m = gamma * gamma * mass_value
            if not flagged and not _close(g2m, effective, ALLOW * U * max(effective, g2m)):
                _fail(f"mode {label}: Γ²·m is not m_eff")
            total = p.totals[k] if p.totals[k] is not None else math.nan
            if total == 0.0:
                if row["fraction"].undefined != "total_mass_zero" or effective != 0.0:
                    _fail(f"mode {label}: with T_k = 0 the fraction is undefined and m_eff is 0")
                continue
            fraction = row["fraction"].value
            if fraction is None:
                _fail(f"mode {label}: fraction undefined with T_k > 0")
            # Two published sides of m_eff = fraction·T_k: a below-normal claim on one
            # side must survive the other side's value.
            if row["fraction"].below and not row["effective"].below and not _subnormal_possible(effective, total):
                _fail(f"mode {label}: the fraction along {DIRECTIONS[k]} is flagged below-normal, but m_eff/T_k is normal")
            if row["effective"].below and not row["fraction"].below and not _subnormal_possible(fraction * total):
                _fail(f"mode {label}: m_eff along {DIRECTIONS[k]} is flagged below-normal, but fraction·T_k is normal")
            ft = fraction * total
            if not row["fraction"].below and not row["effective"].below and not _close(ft, effective, ALLOW * U * max(effective, ft)):
                _fail(f"mode {label}: fraction·T is not m_eff")
            cumulative[k] += fraction
            published = row["cumulative"].value
            if published is None:
                _fail(f"mode {label}: cumulative undefined")
            if not _close(published, cumulative[k], ALLOW * float(m + 2) * U * cumulative[k]):
                _fail(f"mode {label}: cumulative fraction is not the running sum")
    if p.mass is None or p.shapes is None:
        scope.append("mass_matrix or shapes withheld: m_i and φᵀp_k are not re-derived")
        if any(mode["generalized_mass"].below or any(r["gamma"].below or r["effective"].below for r in p.participation[m])
               for m, mode in enumerate(p.modes)):
            scope.append("below-normal flags on m, Γ or m_eff cannot be verified without the payload")
        return scope
    for m, mode in enumerate(p.modes):
        label, gm_vo = m + 1, mode["generalized_mass"]
        gm, excitation = _payload_quantities(p, m)
        gm_slack = ALLOW * float(gm["terms"] + 4) * U * gm["abs"]
        if gm_vo.below:
            # The flag asserts 0 < m < 2⁻¹⁰²²; the payload must allow it.
            if not _subnormal_possible(abs(gm["value"]) - gm_slack):
                _fail(f"mode {label}: m is flagged below-normal, but the payload's φᵀM_ffφ = {gm['value']:e} is normal")
            scope.append(f"mode {label}: m lies below the normal range, so Γ and m_eff are not re-derived")
            continue
        published = gm_vo.value
        if not _close(published, gm["value"], gm_slack):
            _fail(f"mode {label}: m is not φᵀM_ffφ")
        for k in range(3):
            row, acc = p.participation[m][k], excitation[k]
            s, a = acc["value"], acc["abs"]
            e = ALLOW * float(acc["terms"] + 6) * U
            smallest = max(abs(s) - e * a, 0.0)
            claims = (
                ("Γ", row["gamma"], smallest, lambda: _close(row["gamma"].value * published, s, e * (a + abs(row["gamma"].value * published)))),
                ("m_eff", row["effective"], smallest * smallest, lambda: _close(
                    row["effective"].value * published, s * s,
                    e * (2.0 * abs(s) * a + abs(row["effective"].value * published)) + (e * a) * (e * a))),
            )
            for name, vo, lowest_numerator, agrees in claims:
                if vo.below:
                    if not _subnormal_possible(lowest_numerator, published):
                        _fail(f"mode {label}: {name} along {DIRECTIONS[k]} is flagged below-normal, but the payload gives a normal value")
                elif not agrees():
                    _fail(f"mode {label}: {name} along {DIRECTIONS[k]} disagrees with φᵀp_k from the payload")
    return scope


def _reported_set(frequencies, clusters, mode_count, cutoff):
    total = len(frequencies)

    def by_cutoff(f):
        count = 0
        for x in frequencies:
            if x <= f:
                count += 1
            else:
                break
        return count, count < total

    if mode_count is not None and cutoff is None:
        bounded, stop = min(mode_count, total), ("mode_count" if mode_count <= total else "spectrum_exhausted")
    elif mode_count is None and cutoff is not None:
        bounded, reached = by_cutoff(cutoff)
        stop = "frequency_cutoff" if reached else "spectrum_exhausted"
    else:
        nc, nr = min(mode_count, total), mode_count <= total
        fc, fr = by_cutoff(cutoff)
        bounded = min(nc, fc)
        n_stops, f_stops = nr and nc == bounded, fr and fc == bounded
        stop = {(True, True): "mode_count_and_frequency_cutoff", (True, False): "mode_count",
                (False, True): "frequency_cutoff", (False, False): "spectrum_exhausted"}[(n_stops, f_stops)]
    count = bounded
    if count > 0:
        last = clusters[count - 1]
        while count < total and clusters[count] == last:
            count += 1
    return bounded, stop, count


def _counts(p):
    n, total = len(p.modes), len(p.computed)
    if total == 0:
        _fail("a solved document carries at least one computed mode")
    if n != p.count or p.computed_count != total or n > total:
        _fail("the mode counts are not the stated counts")
    if p.extension != p.count - min(p.bounded, p.count) or p.bounded > p.count:
        _fail("the cluster extension is not the count beyond the bound")
    frequencies, previous = [], None
    for i, c in enumerate(p.computed):
        label = i + 1
        if c["reported"] != (i < n) or c["mode"] != (label if i < n else None):
            _fail(f"computed mode {label}: its reported flag and mode reference are not the reported set's")
        omega, f = c["omega"], c["frequency"]
        if omega is None or f is None:
            _fail(f"computed mode {label}: ω or f not published")
        if not (0.0 < omega < math.inf and 0.0 < f < math.inf):
            _fail(f"computed mode {label}: ω and f must be positive and finite")
        if f != omega / TWO_PI:
            _fail(f"computed mode {label}: f is not ω/2π")
        if previous is not None and (omega < previous[0] or f < previous[1]):
            _fail(f"computed mode {label}: not in ascending order")
        previous = (omega, f)
        frequencies.append(f)
    for m, mode in enumerate(p.modes):
        c = p.computed[m]
        if mode["omega"] != c["omega"] or mode["frequency"] != c["frequency"] or mode["cluster"] != c["cluster"]:
            _fail(f"mode {m + 1}: not computed mode {m + 1}")
        f, period = mode["frequency"], mode["period"]
        if f is None or period is None or not (0.0 < f < math.inf and 0.0 < period < math.inf):
            _fail(f"mode {m + 1}: f and T must be positive and finite")
        if period != 1.0 / f:
            _fail(f"mode {m + 1}: T is not 1/f")
    ids = [c["cluster"] for c in p.computed]
    bounded, stop, count = _reported_set(frequencies, ids, p.mode_count, p.cutoff)
    if bounded != p.bounded or stop != p.stopped_by or count != p.count:
        _fail("the bound does not select the stated set")
    expected = "below_lowest_mode" if n == 0 else ("below_computed_cluster" if n < total else "whole_finite_spectrum")
    if p.endpoint != expected:
        _fail(f"endpoint {p.endpoint} is not {expected}")
    if expected == "whole_finite_spectrum":
        if p.next_computed is not None or p.established_endpoint != total or p.census_established != total:
            _fail("the whole finite spectrum is claimed only on an established census of every computed mode")
    elif p.next_computed != n + 1 or p.established_endpoint is not None:
        _fail("the set ends below the next computed mode")
    if p.census_established is not None and p.census_established != total:
        _fail("an established census counts every computed mode")
    if p.sturm_count != n or p.sturm_reported != n:
        _fail("the Sturm count and its reported count are not the modes reported")
    return []


def _clusters(p):
    scope = []
    delta = p.params.get("cluster_tolerance")
    if not _is_number(delta):
        _fail("δ_c: not bound")
    total = len(p.computed)
    for i, c in enumerate(p.computed):
        ok = c["cluster"] == 1 if i == 0 else c["cluster"] in (p.computed[i - 1]["cluster"], p.computed[i - 1]["cluster"] + 1)
        if not ok:
            _fail(f"computed mode {i + 1}: cluster ids are not chained from 1")
    highest = p.computed[-1]["cluster"] if p.computed else 0
    if len(p.clusters) != highest:
        _fail("one cluster row per cluster")
    for c, (first, last, reported, estimate) in enumerate(p.clusters):
        cid = c + 1
        members = [i for i in range(total) if p.computed[i]["cluster"] == cid]
        if not members or members[0] + 1 != first or members[-1] + 1 != last:
            _fail(f"cluster {cid}: its first and last computed modes are not its members'")
        if reported != (first <= len(p.modes)):
            _fail(f"cluster {cid}: its reported flag is not the set's")
        largest = 0.0
        for i in members:
            e = p.computed[i]["estimate"]
            if e is None:
                _fail(f"computed mode {i + 1}: e_λ not published")
            if e > largest:
                largest = e
        if estimate != largest:
            _fail(f"cluster {cid}: its e_λ is not its members' largest")
    for m, mode in enumerate(p.modes):
        index = mode["cluster"] - 1
        own = p.clusters[index][3] if 0 <= index < len(p.clusters) else None
        if mode["cluster_estimate"] != own:
            _fail(f"mode {m + 1}: its cluster e_λ is not its cluster's")
    n = len(p.modes)
    if 0 < n < total and p.computed[n - 1]["cluster"] == p.computed[n]["cluster"]:
        _fail("the reported set ends inside a cluster")
    undetermined = 0
    for i in range(1, total):
        a = p.computed[i - 1]["omega"] if p.computed[i - 1]["omega"] is not None else math.nan
        b = p.computed[i]["omega"] if p.computed[i]["omega"] is not None else math.nan
        gap = b * b - a * a
        threshold = delta * (b * b)
        margin = 8.0 * U * (b * b)
        same = p.computed[i]["cluster"] == p.computed[i - 1]["cluster"]
        if gap > threshold + margin and same:
            _fail(f"computed modes {i} and {i + 1}: share a cluster across a relative gap above δ_c")
        if gap < threshold - margin and not same:
            _fail(f"computed modes {i} and {i + 1}: split within δ_c")
        if not (gap > threshold + margin or gap < threshold - margin):
            undetermined += 1
    if undetermined:
        scope.append(f"{undetermined} adjacent pairs lie within ω's rounding of the δ_c boundary; λ is not published")
    return scope


PROFILE_FLOOR = 2.0 ** 53  # a null `outside_canonical_profile` value is at least this large


def _bound_of(vo):
    """(lowest possible value, known) of a nonnegative published value object."""
    if vo.value is not None:
        return vo.value, True
    if vo.undefined == "outside_canonical_profile":
        return PROFILE_FLOOR, False
    return None, False


def _candidate_arithmetic(c, label):
    """The two published sums and products of one factored candidate, each re-derived
    only when every operand it needs is published."""
    scope = []
    relations = (
        ("e_b = factor term + formation term", (c["backward_error"], c["factor_term"], c["formation_term"]),
         lambda be, ft, fo: be == ft + fo),
        ("conditioning product = e_b·ν", (c["product"], c["backward_error"], c["inverse_norm"]),
         lambda product, be, nu: product == be * nu),
    )
    for name, operands, holds in relations:
        if any(x is None for x in operands):
            scope.append(f"candidate {label}: {name} is not re-derivable (an operand is not published)")
        elif not holds(*operands):
            _fail(f"candidate {label}: {name} does not hold")
    return scope


def _accepted_premises(c, sturm_count, limit, g_max, separated):
    """Every premise of the accepted shift, each judged on its own evidence."""
    scope = []
    if c["pivots"] != sturm_count:
        _fail(f"the accepted negative-pivot count {c['pivots']} is not the Sturm count {sturm_count}")
    for name, vo, ceiling in (("conditioning product e_b·ν", c["product_vo"], limit), ("growth", c["growth_vo"], g_max)):
        lowest, exact = _bound_of(vo)
        if lowest is None:
            scope.append(f"the accepted {name} is not published, so its premise is not re-derived")
        elif lowest > ceiling:
            _fail(f"the accepted shift's {name} {'is' if exact else 'is at least'} {lowest:e}, above its limit {ceiling:e}")
    if separated is False:
        _fail("the accepted shift fails the separation premise")
    if separated is None:
        scope.append("the accepted separation ratio is not published, so its premise is not re-derived")
    return scope


def _sturm(p):
    scope = []
    s_max = p.params.get("sturm_max_candidates")
    if type(s_max) is not int:
        _fail("s_max: not bound")
    kc, limit, ks, g_max = p.kappa_c, p.limit, p.kappa_s, p.growth_limit
    if kc is None or limit is None or ks is None or g_max is None:
        _fail("a Sturm premise parameter is not published")
    for name, value in (("κ_c", kc), ("κ_s", ks), ("g_max", g_max)):
        if not value >= 1.0:
            _fail(f"{name} = {value} is below 1")
    if limit != 1.0 / kc:
        _fail("the conditioning limit is not 1/κ_c")
    count = len(p.candidates)
    if count == 0 or count > s_max:
        _fail(f"{count} candidates against s_max = {s_max}")
    for j, c in enumerate(p.candidates):
        label, last = j + 1, j + 1 == count
        if (c["decision"] == "accepted") != last:
            _fail(f"candidate {label}: only the last candidate is accepted")
        factored = c["pivots"] is not None
        sep = c["separation"]
        if sep.value is None:
            separated = True if sep.undefined == "no_neighbour" else None
        else:
            separated = sep.value >= ks * (1.0 - 8.0 * U)
        if c["decision"] in ("accepted", "growth_limit", "ill_conditioned") and not factored:
            _fail(f"candidate {label}: decision `{c['decision']}` needs a factorization")
        if factored:
            scope.extend(_candidate_arithmetic(c, label))
        if c["decision"] == "accepted":
            scope.extend(_accepted_premises(c, p.sturm_count, limit, g_max, separated))
        if factored:
            growth, product = c["growth"], c["product"]
            if c["decision"] == "growth_limit" and not ((growth is None or growth > g_max) and separated is not False):
                _fail(f"candidate {label}: refused for growth within the limit")
            if c["decision"] == "ill_conditioned" and not ((product is None or product > limit) and growth is not None and growth <= g_max):
                _fail(f"candidate {label}: refused as ill-conditioned within the premise")
            if c["decision"] == "unseparated" and separated is True and sep.value is not None and sep.value >= ks * (1.0 + 8.0 * U):
                _fail(f"candidate {label}: refused as unseparated with its separation above κ_s")
    n = len(p.modes)

    def omega(i):
        return p.computed[i]["omega"] if 0 <= i < len(p.computed) else None

    accepted = p.candidates[-1]["shift"] if p.candidates else None
    if p.endpoint in ("below_computed_cluster", "below_lowest_mode"):
        if p.placement != "gap" or p.placement_established is not None:
            _fail("a set below a computed cluster is counted in a gap")
        upper, lower = p.placement_upper, p.placement_lower
        if upper is None:
            _fail("the gap's upper end is not published")
        if upper != omega(n):
            _fail("the gap's upper end is not the next computed mode's ω")
        if lower is None:
            _fail("the gap's lower end is not published")
        if n > 0:
            if lower != omega(n - 1):
                _fail("the gap's lower end is not the last reported mode's ω")
        else:
            if p.cutoff is None:
                _fail("an empty set has a cut-off")
            expected = TWO_PI * p.cutoff
            if not _close(lower, expected, 8.0 * U * expected):
                _fail("the gap's lower end is not 2π·f_c")
        if accepted is not None and not (lower <= accepted <= upper):
            _fail("the accepted shift is not in its gap")
    else:
        if p.placement != "above_spectrum" or p.placement_upper is not None or p.placement_established != n:
            _fail("the whole spectrum is counted above it, on its established count")
        lower = p.placement_lower
        if lower is None:
            _fail("the top eigenvalue is not published")
        if n == 0 or lower != omega(n - 1):
            _fail("the placement's top is not the top mode's ω")
        if accepted is not None and accepted < lower:
            _fail("the accepted whole-spectrum shift is below the top mode")
    if accepted is None:
        scope.append("the accepted shift is outside the profile")
    return scope


def _identity_coverage(p):
    """The identity rows a run carries, from the document itself: the three axes in
    order, each ending with the model's resultant, each straight member of the
    discretization table compared nodally (and nothing else nodally), and one subject
    set shared by the axes. Arcs and bodies are not listed anywhere in the document,
    so a subject missing on every axis cannot be seen: that coverage is scoped."""
    axes = [row["axis"] for row in p.identity]
    if axes != sorted(axes, key=DIRECTIONS.index):
        _fail("identity: rows are not grouped in axis order")
    by_axis = {axis: [row for row in p.identity if row["axis"] == axis] for axis in DIRECTIONS}
    straight = set(p.straight_members)
    reference = None
    for axis, rows in by_axis.items():
        if not rows or (rows[-1]["subject"], rows[-1]["comparison"]) != ("model", "resultant"):
            _fail(f"identity: no closing model resultant row along {axis}")
        subjects = [row["subject"] for row in rows]
        if len(set(subjects)) != len(subjects):
            _fail(f"identity: a subject repeats along {axis}")
        nodal = {row["subject"] for row in rows if row["comparison"] == "nodal"}
        if nodal != straight:
            _fail(f"identity: the nodal rows along {axis} are not the discretization table's straight members")
        if reference is None:
            reference = subjects
        elif subjects != reference:
            _fail(f"identity: the subjects along {axis} differ from those along x")
    return ["identity: arc and component-body resultant coverage is not derivable (the document lists no arc or body)"]


def _state_coverage(p):
    """A linearized state case's supports and restrained set against the partition."""
    restrained_here = {dof for dof, index in enumerate(p.free) if index is None}
    if p.linearized_restrained is None:
        if p.support_states:
            _fail("supports: support states are listed but no state case is linearized")
        return
    if not p.support_states:
        _fail("supports: a linearized state case lists no support")
    listed = p.linearized_restrained
    if len(set(listed)) != len(listed) or set(listed) != restrained_here:
        _fail("supports: the restrained set of the state case is not the document's restrained DOFs")
    for support, (dof, treatment) in p.support_states.items():
        if treatment in ("restrained", "already_restrained") and dof not in restrained_here:
            _fail(f"supports: {support} is {treatment} but its DOF is free")


def _evidence(p):
    scope = []
    for i, c in enumerate(p.computed):
        eta, kappa, e = c["eta"], c["condition"], c["estimate"]
        if eta is None or kappa is None or e is None:
            scope.append(f"computed mode {i + 1}: a check-2 value is outside the profile")
            continue
        if eta < 0.0 or kappa < 0.0 or e < 0.0:
            _fail(f"computed mode {i + 1}: a negative check-2 value")
        product = eta * kappa
        if not _close(e, product, 8.0 * U * max(e, product)):
            _fail(f"computed mode {i + 1}: e_λ is not η·κ_λ")
    tolerance = p.params.get("identity_tolerance")
    if not isinstance(tolerance, dict) or not _is_number(tolerance.get("relative")) or not _is_number(tolerance.get("absolute")):
        _fail("the identity tolerance is not bound")
    scope.extend(_identity_coverage(p))
    _state_coverage(p)
    if not any(text.startswith("numerical integrity does not establish physical correctness")
               for text in p.disclosure_texts.get("modal", [])):
        _fail("disclosures: the fixed numerical-integrity disclosure is missing")
    if tolerance["absolute"] == 0.0:
        for i, d in enumerate(row["relative"] for row in p.identity):
            if d is None:
                _fail(f"identity row {i}: not published")
            if not (0.0 <= d <= tolerance["relative"]):
                _fail(f"identity row {i}: relative discrepancy exceeds the bound tolerance")
    else:
        scope.append("an absolute identity tolerance: its scale is not published")
    for v in p.formation:
        if v is not None and v < 0.0:
            _fail("a negative formation-difference or norm")
    if p.ortho is not None and p.ortho < 0.0:
        _fail("a negative orthogonality deviation")
    if p.ortho_pair != (len(p.modes) >= 2):
        _fail("the orthogonality pair is stated exactly when two modes are reported")
    return scope


def _standing(p):
    tau_r, tau_o = p.params.get("residual_threshold"), p.params.get("orthogonality_threshold")
    if not _is_number(tau_r) or not _is_number(tau_o):
        _fail("τ_r or τ_o: not bound")
    any_sensitive = False
    for m, mode in enumerate(p.modes):
        eta = p.computed[m]["eta"]
        if eta is None:
            _fail(f"mode {m + 1}: η not published")
        sensitive = eta > tau_r
        if mode["sensitive"] != sensitive or (mode["reasons"] > 0) != sensitive:
            _fail(f"mode {m + 1}: Sensitive exactly when η > τ_r, with its reason")
        any_sensitive = any_sensitive or sensitive
    if p.ortho is None:
        _fail("the orthogonality deviation is not published")
    sensitive = any_sensitive or p.ortho > tau_o
    if p.sensitive != sensitive or (p.set_reasons > 0) != sensitive:
        _fail("the set is Sensitive exactly when a mode is or the orthogonality deviation exceeds τ_o")
    return []


def _units(p):
    scope = []
    for vo, key, path in p.unitized:
        _unit_check(vo, key, path, scope)
    return scope


# --- The verdict --------------------------------------------------------------------------

def _standing_of(function, *args):
    try:
        scope = function(*args)
    except Failure as failure:
        return {"status": "failed", "difference": str(failure), "scope": []}
    except (KeyError, IndexError, TypeError, AttributeError, ValueError, ArithmeticError) as error:
        return {"status": "failed", "difference": f"malformed: {error!r}", "scope": []}
    return {"status": "scoped_unsupported" if scope else "passed", "difference": None, "scope": scope}


def _unchecked():
    return {"status": "not_checked", "difference": None, "scope": []}


def _closure_failed(difference):
    return {"family": FAMILY, "closure": {"status": "failed", "difference": difference, "scope": []},
            "binding": dict(_unchecked(), not_checked=[]), "relations": {name: _unchecked() for name in RELATIONS},
            "health": "not_assessed", "sensitive": False, "refusal_codes": [], "decision": "rejected"}


def read(doc: Any, request: Any = None) -> dict:
    """The verdict of one document, with the request it binds when supplied."""
    if not isinstance(doc, dict):
        return _closure_failed("document: not an object")
    if doc.get("status") == "refused":
        return _read_refused(doc, request)
    try:
        parsed = _closure(doc)
    except Failure as failure:
        return _closure_failed(str(failure))
    except (KeyError, IndexError, TypeError, AttributeError, ValueError) as error:
        return _closure_failed(f"malformed: {error!r}")
    not_checked = []

    def binding():
        not_checked.extend(_binding(doc, request))
        return []

    bound = _standing_of(binding)
    bound["not_checked"] = not_checked
    functions = {"mass": _mass, "normalization": _normalization, "participation": _participation, "counts": _counts,
                 "clusters": _clusters, "sturm": _sturm, "evidence": _evidence, "standing": _standing, "units": _units}
    relations = {name: _standing_of(functions[name], parsed) for name in RELATIONS}
    failed = bound["status"] == "failed" or any(r["status"] == "failed" for r in relations.values())
    scoped = any(r["status"] == "scoped_unsupported" for r in relations.values())
    decision = "rejected" if failed else ("consistent_with_scoped_unsupported" if scoped else "consistent")
    health = "not_assessed" if failed else ("sensitive" if parsed.sensitive else "checks_passed")
    return {"family": FAMILY, "closure": {"status": "passed", "difference": None, "scope": []}, "binding": bound,
            "relations": relations, "health": health, "sensitive": health == "sensitive", "refusal_codes": [], "decision": decision}


def _read_refused(doc, request):
    try:
        _obj(doc, "", REFUSED_KEYS)
        _identity(doc)
        rows = _table({"refusals": doc["refusals"]}, "refusals", set())
        if rows is None:
            _fail("/refusals: withheld")
        codes = []
        for i, row in enumerate(rows):
            severity, code = _diagnostic_row(row, f"/refusals/rows/{i}", f"refusal:{i}", ("modal",))
            if severity == "blocking":
                codes.append(code)
        if not codes:
            _fail("/refusals: a refused document carries at least one blocking diagnostic")
    except Failure as failure:
        return _closure_failed(str(failure))
    except (KeyError, IndexError, TypeError, AttributeError, ValueError) as error:
        return _closure_failed(f"malformed: {error!r}")
    not_checked = []

    def binding():
        not_checked.extend(_binding(doc, request))
        return []

    bound = _standing_of(binding)
    bound["not_checked"] = not_checked
    return {"family": FAMILY, "closure": {"status": "passed", "difference": None, "scope": []}, "binding": bound,
            "relations": {name: _unchecked() for name in RELATIONS}, "health": "not_assessed", "sensitive": False,
            "refusal_codes": codes, "decision": "rejected" if bound["status"] == "failed" else "refusal_consistent"}


MAX_SAFE_INTEGER = 2 ** 53 - 1


def _no_duplicates(pairs):
    seen = {}
    for key, value in pairs:
        if key in seen:
            raise ValueError(f"duplicate object member `{key}`")
        seen[key] = value
    return seen


def _refuse_constant(name):
    raise ValueError(f"non-finite number `{name}`")


def _profile_problems(value, path=""):
    """The first value the checked profile (openpipestress_jcs_ijson_v1) refuses."""
    stack = [(value, path)]
    while stack:
        item, where = stack.pop()
        if type(item) is int and abs(item) > MAX_SAFE_INTEGER:
            return f"{where or '/'}: an integer beyond 2^53 - 1"
        if type(item) is float and (not math.isfinite(item) or (item.is_integer() and abs(item) > MAX_SAFE_INTEGER)):
            return f"{where or '/'}: a number outside I-JSON's binary64 range"
        if isinstance(item, str):
            try:
                item.encode("utf-8")
            except UnicodeEncodeError:
                return f"{where or '/'}: a lone surrogate"
        if isinstance(item, dict):
            for key, child in item.items():
                try:
                    key.encode("utf-8")
                except UnicodeEncodeError:
                    return f"{where}: a member name with a lone surrogate"
                stack.append((child, f"{where}/{key}"))
        elif isinstance(item, list):
            stack.extend((child, f"{where}/{i}") for i, child in enumerate(item))
    return None


def checked_load(text: str) -> Any:
    """The document text admitted by the governed checked profile before any value is
    read: duplicate members, NaN/Infinity, non-finite or unsafe integral numbers and
    lone surrogates are refused (ValueError)."""
    value = json.loads(text, object_pairs_hook=_no_duplicates, parse_constant=_refuse_constant)
    problem = _profile_problems(value)
    if problem:
        raise ValueError(problem)
    return value


def read_text(text: str, request: Any = None) -> dict:
    """The governed entry: the original text is checked before it is read."""
    try:
        doc = checked_load(text)
    except (ValueError, RecursionError) as error:
        return _closure_failed(f"document: outside the checked profile {CANONICAL_PROFILE} ({error})")
    return read(doc, request)
