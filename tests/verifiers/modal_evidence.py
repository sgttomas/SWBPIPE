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

from fractions import Fraction

import json
import math
from typing import Any

from tests.verifiers import canonical_json, units

FAMILY = "openpipestress.modal_result"
SCHEMA_VERSION = "0.1.0"
SEMANTICS_ID = "openpipestress.result_semantics/modal-1"
SEMANTICS_SHA256 = "21d0012729d8466436d0c064d7edecbdc0fc481bf101b6514a65eecbee6fb23b"
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
# The mass matrix's translation-rotation coupling: kg*m/rad (Design §5.3.5).
COUPLING = _catalog("kg*m/rad", "mass_length_per_rotation", "unit:kilogram_meter_per_radian")
# D*M*D with D = diag(|k_ii|^-1/2) of the formed K_ff: seconds squared whatever the DOF
# classes (Design §5.3.5).
TIME_SQUARED = _catalog("s^2", "time_squared", "unit:square_second")
# The statuses a value may carry: catalog, and equilibrated_basis for a listed kind only. The
# former catalog_family_absent is not one: every kind now has its catalog family.
UNIT_STATUSES = ("catalog", "equilibrated_basis")
# modal-1's closed list of the kinds that may be published `equilibrated_basis` (a
# coordinate-basis magnitude with mixed exponents: no dimension, no physical unit, not
# dimensionless). The Design's dimensional trace lists none.
EQUILIBRATED_BASIS_KINDS: list[str] = []


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
    ("sturm_formation_term", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_formation_separation_ratio", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_factor_term", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_backward_error", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_inverse_norm_estimate", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_conditioning_product", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_separation_ratio", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_growth_factor", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_max_abs_lower_factor", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_max_abs_pivot", "sturm_candidates", "fixed", {"fixed": ONE}),
    ("sturm_max_abs_scaled_diagonal", "sturm_candidates", "fixed", {"fixed": ONE}),
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
    ("stiffness_formation_difference", "header", "fixed", {"fixed": ONE}),
    ("mass_formation_difference", "header", "fixed", {"fixed": TIME_SQUARED}),
    ("reformed_stiffness_norm", "header", "fixed", {"fixed": ONE}),
    ("reformed_mass_norm", "header", "fixed", {"fixed": TIME_SQUARED}),
    ("section_derivation_difference", "header", "fixed", {"fixed": ONE}),
]
RULES = {kind: rules for kind, _, _, rules in KINDS}


def semantic_table() -> dict:
    """modal-1 as this reader states it (the committed table's value)."""
    return {"id": SEMANTICS_ID, "document_kind": FAMILY, "schema_version": SCHEMA_VERSION,
            "canonical_profile": CANONICAL_PROFILE, "equilibrated_basis_kinds": list(EQUILIBRATED_BASIS_KINDS),
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


def _unit_check(vo, key, path):
    if vo.status not in UNIT_STATUSES:
        _fail(f"{path}: `{vo.status}` is not a modal-1 unit status")
    if vo.status == "equilibrated_basis" and vo.kind not in EQUILIBRATED_BASIS_KINDS:
        _fail(f"{path}: `{vo.kind}` is not in modal-1's closed equilibrated_basis list")
    rule = RULES.get(vo.kind, {}).get(key)
    if rule is None:
        _fail(f"{path}: modal-1 has no `{key}` rule for `{vo.kind}`")
    if rule["unit_status"] == "catalog":
        if vo.status != "catalog" or vo.unit != rule["unit"]:
            _fail(f"{path}: unit {vo.unit} ({vo.status}) is not modal-1's {rule['unit']}")
    else:
        # A listed kind (none today) claims no dimension and no unit.
        if vo.status != "equilibrated_basis" or vo.unit is not None:
            _fail(f"{path}: modal-1 states the equilibrated basis here")


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
            p.mass.append((a, b, vo))
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
        p.totals[k] = unitize(_vo(r["total_mass"], f"{path}/total_mass", ("modal_total_mass",)), "fixed", f"{path}/total_mass")
        p.free_totals[k] = unitize(_vo(r["free_dof_mass"], f"{path}/free_dof_mass", ("modal_free_dof_mass",)), "fixed",
                                   f"{path}/free_dof_mass")

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
            values.append(unitize(_vo(r[field], f"{path}/{field}", (kind,)), "fixed", f"{path}/{field}"))
        # ω and f by value; η, κ_λ and e_λ as value objects (a null's reason bounds them).
        p.computed.append({"cluster": cluster, "reported": reported, "mode": mode, "omega": values[0].value,
                           "frequency": values[1].value, "eta": values[2], "condition": values[3], "estimate": values[4]})
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
        p.clusters.append((first, last, reported, vo))
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
                "cluster_estimate": read("cluster_error_estimate", "modal_cluster_error_estimate", "fixed"),
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
            p.shapes[m][dof] = vo
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

        # Every candidate value is kept as its value object: a null's reason is a bound.
        c = {"decision": decision, "pivots": pivots}
        for key, field, kind in (("shift", "shift", "sturm_shift_circular_frequency"),
                                 ("formation_term", "formation_term", "sturm_formation_term"),
                                 ("formation_separation", "formation_separation_ratio", "sturm_formation_separation_ratio"),
                                 ("factor_term", "factor_term", "sturm_factor_term"),
                                 ("backward_error", "backward_error", "sturm_backward_error"),
                                 ("inverse_norm", "inverse_norm_estimate", "sturm_inverse_norm_estimate"),
                                 ("product", "conditioning_product", "sturm_conditioning_product"),
                                 ("separation", "separation_ratio", "sturm_separation_ratio"),
                                 ("growth", "growth", "sturm_growth_factor"),
                                 ("max_lower", "max_abs_lower_factor", "sturm_max_abs_lower_factor"),
                                 ("max_pivot", "max_abs_pivot", "sturm_max_abs_pivot"),
                                 ("max_diagonal", "max_abs_scaled_diagonal", "sturm_max_abs_scaled_diagonal")):
            c[key] = read(field, kind)
        p.candidates.append(c)
    p.identity = []
    for i, row in enumerate(get("identity")):
        path = f"/tables/identity/rows/{i}"
        r = _fields(row, path, ("subject", "subject_kind", "axis", "comparison", "relative"))
        _id_is(r, path, f"identity:{i}")
        subject = _text(r["subject"], f"{path}/subject")
        # What the subject names: a member, a component body, or the model's
        # aggregate, whose subject is `model` (a member may carry the same id).
        kind = _one_of(r["subject_kind"], f"{path}/subject_kind", ("member", "component_body", "model"))
        if kind == "model" and subject != "model":
            _fail(f"{path}: an aggregate row's subject is `model`, not `{subject}`")
        relative = unitize(_vo(r["relative"], f"{path}/relative", ("identity_relative_discrepancy",)), "fixed", f"{path}/relative")
        p.identity.append({"subject": subject, "kind": kind,
                           "axis": _one_of(r["axis"], f"{path}/axis", DIRECTIONS),
                           "comparison": _one_of(r["comparison"], f"{path}/comparison", ("nodal", "resultant")),
                           "relative": relative})
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
    p.ortho = header(o["max_ratio"], "/orthogonality/max_ratio", "orthogonality_max_ratio")
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
    for field, kind in (("stiffness_difference", "stiffness_formation_difference"), ("mass_difference", "mass_formation_difference"),
                        ("reformed_stiffness_norm", "reformed_stiffness_norm"), ("reformed_mass_norm", "reformed_mass_norm")):
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


# --- What a published value can be ------------------------------------------------------
#
# Design §5.3.4's relation inventory, from the producer's own formation. A value object
# stands for the value the producer published:
# - an unflagged number is that binary64 number; 0 is an exact zero (`WideDd::to_published`
#   and `to_normal_or_zero`, modal_eigen dd.rs:476 and :507, publish a nonzero value
#   below the normal range only as a flagged 0);
# - `below_normal` is the producer's flagged 0: a NONZERO value of magnitude below 2⁻¹⁰²²,
#   decided on the extended-exponent value, so unbounded below (2⁻²⁰⁰³ is one);
# - `outside_canonical_profile` is finite with magnitude in [2⁵³, largest binary64];
# - `not_finite` is ±∞ or NaN.
# A signed kind (Γ, a shape component, an off-diagonal mass entry) takes either sign; every
# other kind is nonnegative. Every other reason is a claim that the relation consuming it
# checks. Where the producer forms a quantity from PUBLISHED values (the shape in m and
# φᵀp_k, the fractions in their cumulative sums), the readers use the published value: a
# flagged 0 is then 0.

SUBNORMAL_MIN = 2.0 ** -1074
NORMAL_MIN = 2.0 ** -1022
LARGEST = 1.7976931348623157e308
PROFILE_FLOOR = 2.0 ** 53
UNRESOLVED = "unresolved: "


def _times(x, y):
    # A real product: exactly 0 when a factor is 0, even against an unbounded endpoint.
    return 0.0 if x == 0.0 or y == 0.0 else x * y


def _product_bounds(x, y):
    """An interval holding the real product of two binary64 endpoints. A nonzero product
    that underflows (below 2⁻¹⁰²², down to an exact-looking 0) is widened by the smallest
    subnormal, never taken as an exact 0."""
    v = _times(x, y)
    if math.isinf(v) or (v == 0.0 and (x == 0.0 or y == 0.0)):
        return v, v
    slack = 4.0 * U * abs(v)
    if abs(v) < NORMAL_MIN:
        slack += SUBNORMAL_MIN
    return v - slack, v + slack


class Piece:
    """A closed interval of reals [lo, hi] (an infinite end means unbounded); `nonzero`
    excludes 0 from it (a flagged value, or a product of such)."""

    __slots__ = ("lo", "hi", "nonzero")

    def __init__(self, lo, hi, nonzero=False):
        self.lo, self.hi = lo, hi
        self.nonzero = nonzero and lo <= 0.0 <= hi

    def excludes_zero(self):
        return self.lo > 0.0 or self.hi < 0.0 or self.nonzero

    def sign(self):
        """+1 or −1 when every value has that sign, else 0."""
        if self.lo > 0.0 or (self.lo == 0.0 and self.nonzero):
            return 1
        if self.hi < 0.0 or (self.hi == 0.0 and self.nonzero):
            return -1
        return 0


class Values:
    """A set of values: pieces of reals, and whether the binary64 value may be ±∞ or NaN.
    Arithmetic is outward-rounded, exact at 0, and never turns a nonzero underflow into
    0. A real result beyond the largest binary64 is kept, and marks a possible overflow."""

    __slots__ = ("pieces", "nonfinite")
    MAX_PIECES = 8

    def __init__(self, pieces=(), nonfinite=False):
        out = []
        for p in sorted(pieces, key=lambda q: (q.lo, q.hi)):
            if p.hi > LARGEST or p.lo < -LARGEST:
                nonfinite = True
            out.append(p)
        if len(out) > self.MAX_PIECES:
            out = [Piece(min(p.lo for p in out), max(p.hi for p in out))]
        self.pieces = tuple(out)
        self.nonfinite = nonfinite

    @classmethod
    def point(cls, x):
        return cls((Piece(x, x),))

    @classmethod
    def of(cls, vo, signed=False):
        """The set a value object states, or None for a null whose reason is a claim."""
        if vo.value is not None:
            if vo.below:
                flagged = [Piece(0.0, NORMAL_MIN, True)] + ([Piece(-NORMAL_MIN, 0.0, True)] if signed else [])
                return cls(flagged)
            return cls.point(vo.value)
        if vo.undefined == "outside_canonical_profile":
            return cls([Piece(PROFILE_FLOOR, LARGEST)] + ([Piece(-LARGEST, -PROFILE_FLOOR)] if signed else []))
        if vo.undefined == "not_finite":
            return cls((), True)
        return None

    @classmethod
    def published(cls, vo, signed=False):
        """The value the producer itself carries forward: the published number (a flagged 0
        is 0); a null outside the profile or not finite as its set; None for a claim."""
        if vo.value is not None:
            return cls.point(vo.value)
        return cls.of(vo, signed)

    def as_point(self):
        if not self.nonfinite and len(self.pieces) == 1 and self.pieces[0].lo == self.pieces[0].hi:
            return self.pieces[0].lo
        return None

    def only_nonfinite(self):
        return self.nonfinite and not self.pieces

    def __add__(self, other):
        a, b = self.as_point(), other.as_point()
        if a is not None and b is not None:
            v = a + b
            if math.isinf(v):
                return Values([Piece(LARGEST, math.inf) if v > 0 else Piece(-math.inf, -LARGEST)], True)
            return Values.point(v)
        out = []
        for p in self.pieces:
            for q in other.pieces:
                lo, hi = p.lo + q.lo, p.hi + q.hi
                lo -= 4.0 * U * abs(lo) if math.isfinite(lo) else 0.0
                hi += 4.0 * U * abs(hi) if math.isfinite(hi) else 0.0
                # Two same-signed parts, one of them nonzero, have a nonzero sum.
                signs = (p.sign() or (1 if p.lo >= 0.0 else -1 if p.hi <= 0.0 else 0),
                         q.sign() or (1 if q.lo >= 0.0 else -1 if q.hi <= 0.0 else 0))
                nonzero = signs[0] == signs[1] != 0 and (p.excludes_zero() or q.excludes_zero())
                out.append(Piece(lo, hi, nonzero))
        return Values(out, self.nonfinite or other.nonfinite)

    def __mul__(self, other):
        a, b = self.as_point(), other.as_point()
        if a is not None and b is not None:
            v = a * b
            if math.isinf(v):
                return Values([Piece(LARGEST, math.inf) if v > 0 else Piece(-math.inf, -LARGEST)], True)
            if v != 0.0 and abs(v) >= NORMAL_MIN or a == 0.0 or b == 0.0:
                return Values.point(v)
            # A nonzero product below the normal range: nonzero, signed, never 0.
            # Rounded within one subnormal step, and nonzero.
            positive = (a > 0.0) == (b > 0.0)
            bottom, top = max(abs(v) - SUBNORMAL_MIN, 0.0), abs(v) + SUBNORMAL_MIN
            return Values([Piece(bottom, top, True) if positive else Piece(-top, -bottom, True)])
        out = []
        for p in self.pieces:
            for q in other.pieces:
                ends = [e for x in (p.lo, p.hi) for y in (q.lo, q.hi) for e in _product_bounds(x, y)]
                lo, hi = min(ends), max(ends)
                sign = p.sign() * q.sign()
                nonzero = p.excludes_zero() and q.excludes_zero()
                if sign > 0:
                    lo = max(lo, 0.0)
                elif sign < 0:
                    hi = min(hi, 0.0)
                out.append(Piece(lo, hi, nonzero))
        # 0·(±∞ or NaN) is NaN: a non-finite factor gives a non-finite product.
        return Values(out, self.nonfinite or other.nonfinite)

    def square(self):
        a = self.as_point()
        if a is not None:
            return self * self
        out = []
        for p in self.pieces:
            ends = [e for x in (p.lo, p.hi) for e in _product_bounds(x, x)]
            # Across 0 the square reaches 0; on one side it is monotone.
            bottom = 0.0 if p.lo < 0.0 < p.hi else max(0.0, min(ends))
            out.append(Piece(bottom, max(ends), p.excludes_zero()))
        return Values(out, self.nonfinite)

    def widen(self, relative):
        """Each piece grown by `relative` times its ends' magnitudes (0 stays 0)."""
        return Values([Piece(p.lo - _times(relative, abs(p.lo)), p.hi + _times(relative, abs(p.hi)), p.nonzero)
                       for p in self.pieces], self.nonfinite)

    def meets(self, other):
        if self.nonfinite and other.nonfinite:
            return True
        for p in self.pieces:
            for q in other.pieces:
                lo, hi = max(p.lo, q.lo), min(p.hi, q.hi)
                if lo < hi:
                    return True
                if lo == hi and (lo != 0.0 or not (p.excludes_zero() or q.excludes_zero())):
                    return True
        return False

    def outcomes(self, predicate_low, predicate_high, nonfinite_outcomes):
        """The truth values a monotone threshold predicate can take over the set: each
        piece contributes its ends' values; a non-finite value contributes
        `nonfinite_outcomes` (the producer's comparison on ±∞ and NaN)."""
        found = set()
        for p in self.pieces:
            found.add(predicate_low(p.lo))
            found.add(predicate_high(p.hi))
        if self.nonfinite:
            found |= set(nonfinite_outcomes)
        return found


def _formed(*pairs):
    """Whether each binary64 product (product, *factors) is what the real product is up to
    rounding: not a nonzero product that underflowed below the normal range, which the
    point path would mistake for (near) 0. Otherwise the set path decides."""
    for product, *factors in pairs:
        if all(f != 0.0 for f in factors) and abs(product) < NORMAL_MIN:
            return False
    return True


def _generalized_mass_term(i, j, shape, entry):
    """One entry's term of m = Σ_ij φ_i·M_ij·φ_j. The producer sums each full triple
    product exactly (`mass_bilinear`, modal_quantities.rs:294, `ExactSum::add_product`),
    so the coefficient w·φ_i·φ_j is never a binary64 product that can underflow: it is
    formed on sets, which keep a nonzero underflow nonzero. None only for an exactly zero
    coefficient against a finite entry."""
    coefficient = Values.point(1.0 if i == j else 2.0) * Values.point(shape[i]) * Values.point(shape[j])
    if coefficient.as_point() == 0.0 and not entry.nonfinite:
        return None
    return coefficient * entry


def _generalized_mass_check(scope, label, gm_vo, gm, mass_terms, count):
    """m against φᵀM_ffφ from its terms (`_generalized_mass_term`), within A(N+4)u·Σ|terms|
    over the N free pairs: decided on points only when every term is a point."""
    allowance = ALLOW * float(count + 4) * U
    points = [t.as_point() for t in mass_terms]
    if all(x is not None for x in points) and gm.as_point() is not None:
        total = sum(points)
        if not _close(gm.as_point(), total, allowance * sum(abs(x) for x in points)):
            _fail(f"mode {label}: m is not φᵀM_ffφ")
        return
    derived = _total(t.widen(allowance) for t in mass_terms)
    if not gm.meets(derived):
        if gm_vo.below:
            _fail(f"mode {label}: m is flagged below-normal, but the payload's φᵀM_ffφ is normal")
        _fail(f"mode {label}: m is not φᵀM_ffφ for any value the bounds allow")
    scope.append(UNRESOLVED + f"mode {label}: m = φᵀM_ffφ is undecidable on the published bounds")


def _excitation_terms(free, entries, phi):
    """φᵀp_k's terms, one per mass entry and direction: {k: {entry index: [c, w]}}, with c
    the entry's combined coefficient and w the sum of its coefficients' magnitudes, and
    the incidence count per direction. An entry M_ij meets φᵀp_k in row i (i free, j along
    k) and in row j (j free, i along k); it is ONE variable (reviewer's repro: +1 and −1 on
    the same entry sum to exactly 0)."""
    terms, counts = [dict(), dict(), dict()], [0, 0, 0]
    for index, (i, j) in enumerate(entries):
        hits = []
        if free[i] and j % 6 < 3:
            hits.append((j % 6, phi[i]))
        if i != j and free[j] and i % 6 < 3:
            hits.append((i % 6, phi[j]))
        for k, c in hits:
            counts[k] += 1
            held = terms[k].setdefault(index, [0.0, 0.0])
            held[0] += c
            held[1] += abs(c)
    return terms, counts


def _total(sets):
    out = Values.point(0.0)
    for s in sets:
        out = out + s
    return out


def _claim_text(vo):
    return f"null, and its reason `{vo.undefined}` states no value where one is needed"


TO_PUBLISHED, NORMAL_OR_ZERO, BINARY64 = "to_published", "to_normal_or_zero", "binary64"


def _producer_domain(vo, rule, what):
    """The field's own domain from the producer's rounding rule (Design §5.3.4):
    `to_published` (dd.rs:476) is 0, normal, or a flagged nonzero below 2⁻¹⁰²²;
    `to_normal_or_zero` (dd.rs:507) is 0 or normal and never flagged (a subnormal refuses
    the run); a plain binary64 value is never flagged. A value outside it is no producer
    output."""
    if vo.value is None:
        return
    subnormal = vo.value != 0.0 and abs(vo.value) < NORMAL_MIN
    if rule == NORMAL_OR_ZERO and (vo.below or subnormal):
        _fail(f"{what}: {'flagged below-normal' if vo.below else 'subnormal'}, but the producer publishes it only as 0 or normal (to_normal_or_zero)")
    if rule == TO_PUBLISHED and not vo.below and subnormal:
        _fail(f"{what}: an unflagged subnormal, but the producer flags every nonzero value below 2⁻¹⁰²² (to_published)")
    if rule == BINARY64 and vo.below:
        _fail(f"{what}: flagged below-normal, but the producer never flags this field")


def _needed(vo, what, signed=False):
    """The set of a value a relation needs; a claim-null fails that relation."""
    s = Values.of(vo, signed)
    if s is None:
        _fail(f"{what}: {_claim_text(vo)}")
    return s


def _mass_entry_set(i, j, vo):
    # A diagonal entry is a mass (nonnegative); an off-diagonal one has either sign.
    return _needed(vo, f"mass entry ({i}, {j})", signed=i != j)


def bessel_established(effective_highs, t_low, n):
    """Agent 0's Bessel interim contract: True exactly when Σ m_eff ≤ T_k·(1 + 16(n+2)u)
    holds for the given upper ends of the effective masses and lower end of T_k. Every
    operand is a finite binary64 value, so the comparison is done in exact rationals."""
    highest = sum((Fraction(h) for h in effective_highs), Fraction(0))
    return highest <= Fraction(t_low) * (1 + Fraction(ALLOW) * (n + 2) * Fraction(U))


def _mass(p):
    scope = []
    n = len(p.modes)
    for k in range(3):
        direction = DIRECTIONS[k]
        for name, vo in ((f"T_{direction}", p.totals[k]), (f"free-DOF mass {direction}", p.free_totals[k])):
            _producer_domain(vo, BINARY64, name)
            _needed(vo, name)
            if vo.value is not None and vo.value < 0.0:
                _fail(f"{name}: negative")
    # Bessel's inequality, per direction, on its own operands: Σ_i m_eff,ik ≤ T_k.
    for k in range(3):
        direction = DIRECTIONS[k]
        t = _needed(p.totals[k], f"T_{direction}")
        effective = [Values.of(p.participation[m][k]["effective"]) for m in range(n)]
        # A claim-null m_eff fails participation; m_eff ≥ 0, so leaving it out keeps a lower bound.
        effective = [e for e in effective if e is not None]
        if t.only_nonfinite() or any(e.nonfinite for e in effective) or t.nonfinite:
            scope.append(UNRESOLVED + f"Σ m_eff,{direction} ≤ T_{direction}: an operand is not finite (±∞ or NaN)")
            continue
        # Agent 0's interim contract (Design §5.3.4): Bessel passes only where sound
        # arithmetic establishes Σ m_eff ≤ T_k from the published operands; otherwise it
        # is unresolved, never refused. Σ m_eff ≤ T_k holds for modes orthogonal in the
        # assembled M, which the document does not evidence (the published deviation is
        # on the re-formed M̂). The comparison is exact: every operand here is a finite
        # binary64 value, so the sum of upper ends and the allowed total are rationals,
        # with no rounding in the decision (review of f5c66b167).
        if not bessel_established([max(p.hi for p in e.pieces) for e in effective], min(p.lo for p in t.pieces), n):
            scope.append(UNRESOLVED + f"Bessel: Σ m_eff,{direction} ≤ T_{direction} is not established within the strict "
                         "rounding allowance, and no common-M orthogonality bound is published (a temporary capability limit)")
    if p.mass is None:
        scope.append("mass_matrix withheld: T_k and the free-DOF mass are not re-derived")
        return scope
    sets = []
    for i, j, vo in p.mass:
        _producer_domain(vo, BINARY64, f"mass entry ({i}, {j})")
        s = _mass_entry_set(i, j, vo)
        if i == j and vo.value is not None and vo.value < 0.0:
            _fail(f"mass diagonal at DOF {i} is negative")
        sets.append((i, j, vo, s))
    for k in range(3):
        direction = DIRECTIONS[k]
        for name, published, free_only in ((f"T_{direction}", p.totals[k], False),
                                           (f"free-DOF mass {direction}", p.free_totals[k], True)):
            # T_k = R_kᵀ M R_k over every DOF (restrained included); the free-DOF mass over free pairs.
            terms = [(1.0 if i == j else 2.0, vo, s) for i, j, vo, s in sets
                     if i % 6 == k and j % 6 == k and not (free_only and (p.free[i] is None or p.free[j] is None))]
            allowance = ALLOW * float(len(terms) + 4) * U
            stated = _needed(published, name)
            if all(s.as_point() is not None for _, _, s in terms) and stated.as_point() is not None:
                total = sum(w * s.as_point() for w, _, s in terms)
                magnitude = sum(abs(w * s.as_point()) for w, _, s in terms)
                if not _close(stated.as_point(), total, allowance * magnitude):
                    _fail(f"{name} is not the sum of its mass entries")
                continue
            derived = _total((Values.point(w) * s).widen(allowance) for w, _, s in terms)
            if not stated.meets(derived):
                _fail(f"{name} is not the sum of its mass entries for any value its bounds allow")
            scope.append(UNRESOLVED + f"{name} = Σ of its mass entries is undecidable on the published bounds")
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
        if shape[mode["reference"]].value != 1.0:
            _fail(f"mode {label}: its reference component is not exactly +1")
        t, theta = 0.0, 0.0
        for dof, vo in enumerate(shape):
            _producer_domain(vo, TO_PUBLISHED, f"mode {label}: shape component {dof}")
            v = vo.value
            if v is None:
                _fail(f"mode {label}: a shape component is not published")
            if p.free[dof] is None:
                if v != 0.0 or vo.below:
                    _fail(f"mode {label}: restrained DOF {dof} is not exactly 0")
                continue
            # A below-normal component is below 2⁻¹⁰²² in magnitude, inside every bound here.
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


def _within_relative(lhs, rhs, relative):
    """Whether some x in lhs and y in rhs have |x − y| ≤ relative·max(|x|, |y|)."""
    return lhs.widen(relative).meets(rhs) or lhs.meets(rhs.widen(relative))


def _quotient_relation(scope, label, operands, lhs, rhs, relative, failure, one_sided=False):
    """One relation the producer forms through a quotient (Γ²·m = m_eff, fraction·T_k =
    m_eff, Γ·m = φᵀp_k, m_eff·m = (φᵀp_k)²), decided on sets of real values. `one_sided`:
    only lhs takes the allowance (rhs already carries its terms'). A non-finite OPERAND
    leaves it undecidable (the producer's quotient can then be finite or not); a real
    product beyond the largest binary64 is no such operand, and it cannot equal a finite
    published side, so only the finite pieces count."""
    if any(x.nonfinite for x in operands):
        scope.append(UNRESOLVED + f"{label}: an operand is not finite (±∞ or NaN)")
        return
    lhs, rhs = Values(lhs.pieces), Values(rhs.pieces)  # the real parts only
    holds = lhs.widen(relative).meets(rhs) if one_sided else _within_relative(lhs, rhs, relative)
    if not holds:
        _fail(failure)
    scope.append(UNRESOLVED + f"{label} is undecidable on the published bounds")


def _participation(p):
    scope = []
    n = len(p.modes)
    for m, mode in enumerate(p.modes):
        label, gm_vo = m + 1, mode["generalized_mass"]
        _producer_domain(gm_vo, TO_PUBLISHED, f"mode {label}: m")
        for k in range(3):
            for field in ("gamma", "effective", "fraction"):
                _producer_domain(p.participation[m][k][field], TO_PUBLISHED, f"mode {label}: {field} along {DIRECTIONS[k]}")
            _producer_domain(p.participation[m][k]["cumulative"], BINARY64, f"mode {label}: cumulative fraction along {DIRECTIONS[k]}")
        gm = _needed(gm_vo, f"mode {label}: m")
        if gm_vo.value is not None and not gm_vo.below and gm_vo.value <= 0.0:
            _fail(f"mode {label}: m is not positive")
        for k in range(3):
            row, direction = p.participation[m][k], DIRECTIONS[k]
            gamma_vo, effective_vo = row["gamma"], row["effective"]
            gamma = _needed(gamma_vo, f"mode {label}: Γ along {direction}", signed=True)
            effective = _needed(effective_vo, f"mode {label}: m_eff along {direction}")
            if effective_vo.value is not None and effective_vo.value < 0.0:
                _fail(f"mode {label}: m_eff is negative")
            # Γ²·m = m_eff.
            points = [v.as_point() for v in (gamma, gm, effective)]
            if all(x is not None for x in points) and _formed((points[0] * points[0], points[0], points[0]),
                                                               (points[0] * points[0] * points[1], points[0], points[1])):
                g, mass, e = points
                g2m = g * g * mass
                if not _close(g2m, e, ALLOW * U * max(e, g2m)):
                    _fail(f"mode {label}: Γ²·m is not m_eff")
            else:
                _quotient_relation(scope, f"mode {label}: Γ²·m = m_eff along {direction}", (gamma, gm, effective),
                                   gamma.square() * gm, effective,
                                  ALLOW * U, f"mode {label}: Γ²·m = m_eff along {direction} is contradicted by the bounds"
                                   + (" (a side flagged below-normal)" if gamma_vo.below or gm_vo.below or effective_vo.below else ""))
            # The fraction: undefined exactly when T_k = 0, which the null claims and is checked.
            total_vo, fraction_vo, cumulative_vo = p.totals[k], row["fraction"], row["cumulative"]
            total = _needed(total_vo, f"T_{direction}")
            claims_zero = [vo.undefined == "total_mass_zero" for vo in (fraction_vo, cumulative_vo)]
            if any(claims_zero):
                if not all(claims_zero):
                    _fail(f"mode {label}: the fraction and its cumulative sum along {direction} disagree on T_k = 0")
                if total.as_point() != 0.0:
                    _fail(f"mode {label}: the fraction along {direction} claims T_k = 0, but T_k cannot be 0")
                if effective.as_point() != 0.0:
                    _fail(f"mode {label}: with T_k = 0 m_eff is 0")
                continue
            if total.as_point() == 0.0:
                _fail(f"mode {label}: with T_k = 0 the fraction is undefined")
            fraction = _needed(fraction_vo, f"mode {label}: fraction along {direction}")
            if fraction_vo.value is not None and fraction_vo.value < 0.0:
                _fail(f"mode {label}: the fraction along {direction} is negative")
            points = [v.as_point() for v in (fraction, total, effective)]
            if all(x is not None for x in points) and _formed((points[0] * points[1], points[0], points[1])):
                ft = fraction.as_point() * total.as_point()
                e = effective.as_point()
                if not _close(ft, e, ALLOW * U * max(e, ft)):
                    _fail(f"mode {label}: fraction·T is not m_eff")
            else:
                flagged = " (a side flagged below-normal)" if fraction_vo.below or effective_vo.below else ""
                _quotient_relation(scope, f"mode {label}: fraction·T_k = m_eff along {direction}", (fraction, total, effective),
                                   fraction * total, effective,
                                   ALLOW * U, f"mode {label}: fraction·T_k = m_eff along {direction} is contradicted by the bounds{flagged}")
    # The cumulative sums, each on its own operands: the fractions up to it and itself.
    for k in range(3):
        direction = DIRECTIONS[k]
        running = []
        for m in range(n):
            row, label = p.participation[m][k], m + 1
            if row["fraction"].undefined == "total_mass_zero":
                break
            # The producer sums the PUBLISHED fractions (a flagged fraction is its published 0),
            # modal_quantities.rs:460.
            published = Values.published(row["fraction"])
            if published is None:
                _fail(f"mode {label}: fraction along {direction}: {_claim_text(row['fraction'])}")
            running.append(published)
            cumulative_vo = row["cumulative"]
            cumulative = _needed(cumulative_vo, f"mode {label}: cumulative fraction along {direction}")
            if cumulative_vo.value is not None and cumulative_vo.value < 0.0:
                _fail(f"mode {label}: the cumulative fraction along {direction} is negative")
            allowance = ALLOW * float(m + 2) * U
            if all(f.as_point() is not None for f in running) and cumulative.as_point() is not None:
                total = sum(f.as_point() for f in running)
                if not _close(cumulative.as_point(), total, allowance * total):
                    _fail(f"mode {label}: cumulative fraction is not the running sum")
                continue
            derived = _total(running).widen(allowance)
            if not cumulative.meets(derived):
                _fail(f"mode {label}: cumulative fraction along {direction} is not the running sum for any value its bounds allow")
            scope.append(UNRESOLVED + f"mode {label}: the cumulative fraction along {direction} is undecidable on the published bounds")
    if p.mass is None or p.shapes is None:
        scope.append("mass_matrix or shapes withheld: m_i and φᵀp_k are not re-derived")
        return scope
    # An entry is needed only by the payload relations whose sums include it.
    entries = [(i, j, vo) for i, j, vo in p.mass]
    for m, mode in enumerate(p.modes):
        label = m + 1
        # m and φᵀp_k are formed from the PUBLISHED shape (a flagged component is its
        # published 0), modal_quantities.rs:377 and :398, :426.
        phi = [Values.published(vo, signed=True) for vo in p.shapes[m]]
        if any(v is None or v.as_point() is None for v in phi):
            _fail(f"mode {label}: a shape component is not published")
        # Each term with the entry it depends on; a term with an exactly-zero coefficient
        # is exactly 0 for a finite entry, and so depends on nothing.
        # Each allowance counts every entry of its sum, zero-coefficient ones included.
        # One variable per entry. m = Σ_ij φ_i M_ij φ_j over the full symmetric M_ff gives an
        # off-diagonal entry the single coefficient 2·φ_i·φ_j; in φᵀp_k (p_k = (M R_k)_f,
        # stiffness.rs:255, mass.rs:147) each entry's coefficients are combined
        # (`_excitation_terms`), and the allowance is held to their magnitudes (each row of
        # p_k is rounded on its own). Every allowance counts all its incidences.
        free = [f is not None for f in p.free]
        shape = [v.as_point() for v in phi]
        combined, incidence_counts = _excitation_terms(free, [(i, j) for i, j, _ in entries], shape)
        mass_terms, counts, used = [], [0, 0, 0, 0], [[], [], [], []]
        excitation = [dict(), dict(), dict()]
        for index, (i, j, vo) in enumerate(entries):
            in_mass = free[i] and free[j]
            if not in_mass and not any(index in combined[k] for k in range(3)):
                continue
            entry = _mass_entry_set(i, j, vo)
            if in_mass:
                counts[3] += 1
                used[3].append(entry)
                term = _generalized_mass_term(i, j, shape, entry)
                if term is not None:
                    mass_terms.append(term)
            for k in range(3):
                if index in combined[k]:
                    c, w = combined[k][index]
                    excitation[k][index] = [c, w, entry]
                    used[k].append(entry)
        counts[:3] = incidence_counts
        gm_vo = mode["generalized_mass"]
        gm = _needed(gm_vo, f"mode {label}: m")
        _generalized_mass_check(scope, label, gm_vo, gm, mass_terms, counts[3])
        for k in range(3):
            row, direction = p.participation[m][k], DIRECTIONS[k]
            terms = [held for held in excitation[k].values() if held[1] != 0.0 or held[2].nonfinite]
            e = ALLOW * float(counts[k] + 6) * U
            gamma = _needed(row["gamma"], f"mode {label}: Γ along {direction}", signed=True)
            effective = _needed(row["effective"], f"mode {label}: m_eff along {direction}")
            points_ok = (all(t[2].as_point() is not None for t in terms) and all(v.as_point() is not None for v in (gamma, effective, gm))
                         and _formed(*((c * x.as_point(), c, x.as_point()) for c, _, x in terms)))
            if points_ok:
                s = sum(c * x.as_point() for c, _, x in terms)
                a = sum(w * abs(x.as_point()) for _, w, x in terms)
                published, g, eff = gm.as_point(), gamma.as_point(), effective.as_point()
                points_ok = _formed((g * published, g, published), (eff * published, eff, published), (s * s, s, s))
            if points_ok:
                if not _close(g * published, s, e * (a + abs(g * published))):
                    _fail(f"mode {label}: Γ along {direction} disagrees with φᵀp_k from the payload")
                if not _close(eff * published, s * s, e * (2.0 * abs(s) * a + abs(eff * published)) + (e * a) * (e * a)):
                    _fail(f"mode {label}: m_eff along {direction} disagrees with φᵀp_k from the payload")
                continue
            # Each entry once: c·x ± e·w·|x| is exactly x·[c − e·w, c + e·w] (the allowance is
            # proportional to the same x).
            s = _total(Values([Piece(c - e * w, c + e * w)]) * x for c, w, x in terms)
            flag = " (flagged below-normal)"
            _quotient_relation(scope, f"mode {label}: Γ·m = φᵀp_k along {direction}", [gamma, gm] + used[k], gamma * gm, s, e,
                               f"mode {label}: Γ along {direction}{flag if row['gamma'].below else ''} disagrees with "
                               "φᵀp_k from the payload for any value the bounds allow", one_sided=True)
            _quotient_relation(scope, f"mode {label}: m_eff·m = (φᵀp_k)² along {direction}", [effective, gm] + used[k],
                               effective * gm, s.square(), e,
                               f"mode {label}: m_eff along {direction}{flag if row['effective'].below else ''} disagrees "
                               "with φᵀp_k from the payload for any value the bounds allow", one_sided=True)
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
        # The cluster's e_λ is the members' maximum, NaN-propagating (the producer's
        # `max_propagating_nan`): exactly when all are published; else on sets.
        _producer_domain(estimate, NORMAL_OR_ZERO, f"cluster {cid}: e_λ")
        sets = [_needed(p.computed[i]["estimate"], f"computed mode {i + 1}: e_λ") for i in members]
        stated = _needed(estimate, f"cluster {cid}: e_λ")
        if all(x.as_point() is not None for x in sets) and stated.as_point() is not None:
            if stated.as_point() != max(x.as_point() for x in sets):
                _fail(f"cluster {cid}: its e_λ is not its members' largest")
        else:
            finite = [x for x in sets if x.pieces]
            maximum = Values()
            if len(finite) == len(sets):
                # Every member finite: the maximum ranges over [max of lows, max of highs].
                low = max(min(p.lo for p in x.pieces) for x in sets)
                maximum = Values([Piece(low, max(max(p.hi for p in x.pieces) for x in sets),
                                        all(all(p.excludes_zero() for p in x.pieces) for x in sets))])
            maximum.nonfinite = any(x.nonfinite for x in sets)
            if not stated.meets(maximum):
                _fail(f"cluster {cid}: its e_λ is not its members' largest for any value the bounds allow")
            scope.append(UNRESOLVED + f"cluster {cid}: its e_λ against its members' is undecidable on the published bounds")
    for m, mode in enumerate(p.modes):
        index = mode["cluster"] - 1
        own = p.clusters[index][3] if 0 <= index < len(p.clusters) else None
        mine = mode["cluster_estimate"]
        _producer_domain(mine, NORMAL_OR_ZERO, f"mode {m + 1}: its cluster e_λ")
        # The copy is the same value object: value, reason and below-normal flag.
        if own is None or (mine.value, mine.undefined, mine.below) != (own.value, own.undefined, own.below):
            _fail(f"mode {m + 1}: its cluster e_λ is not its cluster's")
    n = len(p.modes)
    if 0 < n < total and p.computed[n - 1]["cluster"] == p.computed[n]["cluster"]:
        _fail("the reported set ends inside a cluster")
    undetermined = 0
    for i in range(1, total):
        a = p.computed[i - 1]["omega"] if p.computed[i - 1]["omega"] is not None else math.nan
        b = p.computed[i]["omega"] if p.computed[i]["omega"] is not None else math.nan
        # The kernel breaks where fl(lambda_b - lambda_a) > fl(delta_c * lambda_b); each
        # published omega is fl(sqrt(lambda)), so fl(omega^2) is within 3u*lambda (+O(u^2)).
        # The reader and the kernel then differ by at most (8u + 5u*delta_c)*lambda_b +
        # O(u^2), which 16u*(1 + delta_c)*omega_b^2 covers for every delta_c in (0, 1).
        gap = b * b - a * a
        threshold = delta * (b * b)
        margin = 16.0 * U * (1.0 + delta) * (b * b)
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


FACTORED = ("factor_term", "backward_error", "inverse_norm", "product", "separation", "growth", "max_lower", "max_pivot",
            "max_diagonal")


def _exact_formula(scope, label, lhs, operands, combine, relation, contradiction):
    """A value the producer forms by one + or ·: bit for bit when every operand is
    published, else within 8u on sets (a non-finite operand gives a non-finite result)."""
    points = [x.as_point() for x in operands]
    if lhs.as_point() is not None and all(x is not None for x in points):
        if lhs.as_point() != combine(*points):
            _fail(f"{label}: {relation} does not hold")
        return
    if not lhs.meets(combine(*operands).widen(8.0 * U)):
        _fail(f"{label}: {contradiction}")
    scope.append(UNRESOLVED + f"{label}: {relation} is undecidable on the published bounds")


def _candidate(scope, c, label, sturm_count, limit, g_max, ks):
    """One candidate: its claims, its two formulas and its decision against its premises,
    each on its own operands."""
    # formation_separation is plain binary64 at the producer and never flagged; the
    # readers do not re-derive it (a stated limit), but its domain is still enforced.
    for key in ("shift", "formation_term", "formation_separation") + FACTORED:
        _producer_domain(c[key], BINARY64, f"candidate {label}: {key.replace('_', ' ')}")
    factored = c["pivots"] is not None
    if not factored:
        for key in FACTORED:
            if c[key].value is not None or c[key].undefined != "not_factored":
                _fail(f"candidate {label}: not factored, but its {key.replace('_', ' ')} is not null `not_factored`")
        if c["decision"] in ("accepted", "growth_limit", "ill_conditioned"):
            _fail(f"candidate {label}: decision `{c['decision']}` needs a factorization")
        return
    sets = {}
    for key in ("formation_term",) + FACTORED:
        vo = c[key]
        sets[key] = Values.of(vo)
        if sets[key] is None:
            if key == "separation":
                # `no_neighbour` claims no neighbouring computed eigenvalue; every placement
                # has one (the gap's upper mode, or the top mode above the spectrum).
                _fail(f"candidate {label}: its separation ratio is null `{vo.undefined}`, which a factored candidate in a placement with a neighbour cannot be")
            _fail(f"candidate {label}: its {key.replace('_', ' ')} is null without a profile bound")
    _exact_formula(scope, f"candidate {label}", sets["backward_error"], (sets["factor_term"], sets["formation_term"]),
                   lambda a, b: a + b, "e_b = factor term + formation term",
                   "e_b = factor term + formation term is contradicted by the bounds")
    _exact_formula(scope, f"candidate {label}", sets["product"], (sets["backward_error"], sets["inverse_norm"]),
                   lambda a, b: a * b, "conditioning product = e_b·ν",
                   "the conditioning product is contradicted by the bounds of e_b·ν")
    # The producer's three tests, as the truth values each can take: separated is
    # s ≥ κ_s·e_b (a non-finite ratio means e_b = 0, so separated); the growth and the
    # conditioning are its ≤ comparisons, false for ±∞ and NaN.
    separated = sets["separation"].outcomes(lambda lo: lo >= ks * (1.0 - 8.0 * U), lambda hi: hi >= ks * (1.0 - 8.0 * U), (True,))
    if any(p.lo < ks * (1.0 + 8.0 * U) for p in sets["separation"].pieces):
        separated.add(False)
    within = sets["growth"].outcomes(lambda lo: lo <= g_max, lambda hi: hi <= g_max, (False,))
    conditioned = sets["product"].outcomes(lambda lo: lo <= limit, lambda hi: hi <= limit, (False,))
    decision = c["decision"]
    if decision == "accepted":
        if c["pivots"] != sturm_count:
            _fail(f"the accepted negative-pivot count {c['pivots']} is not the Sturm count {sturm_count}")
        for name, outcomes, failure in (("conditioning product", conditioned, "fails the conditioning premise"),
                                        ("growth", within, "exceeds the growth limit"),
                                        ("separation ratio", separated, "fails the separation premise")):
            if True not in outcomes:
                _fail(f"the accepted shift {failure}: its {name} cannot meet it")
            if False in outcomes:
                scope.append(UNRESOLVED + f"the accepted {name} is undecidable against its premise on the published bounds")
    elif decision == "growth_limit":
        if True not in separated or False not in within:
            _fail(f"candidate {label}: refused for growth within the limit")
    elif decision == "ill_conditioned":
        if True not in separated or True not in within or False not in conditioned:
            _fail(f"candidate {label}: refused as ill-conditioned within the premise")
    elif decision == "unseparated":
        if False not in separated:
            _fail(f"candidate {label}: refused as unseparated with its separation above κ_s")


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
        _candidate(scope, c, label, p.sturm_count, limit, g_max, ks)
    n = len(p.modes)

    def omega(i):
        return p.computed[i]["omega"] if 0 <= i < len(p.computed) else None

    shift = p.candidates[-1]["shift"] if p.candidates else None
    # The accepted shift's set: finite, so a non-finite or claim-null one cannot be in a gap.
    shift_set = _needed(shift, "the accepted shift") if shift is not None else None
    if shift_set is not None and not shift_set.pieces:
        _fail("the accepted shift is not finite")
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
            # The producer squares omega_c = fl(2*pi*f_c) and publishes the root of that square.
            # A square below the normal range (f_c under about 2.4e-155 Hz) has fewer significant
            # bits, so a relative allowance about 2*pi*f_c does not hold; repeat the operations.
            omega_c = TWO_PI * p.cutoff
            if lower != math.sqrt(omega_c * omega_c):
                _fail("the gap's lower end is not √fl((2π·f_c)²)")
        if shift_set is not None:
            if not shift_set.meets(Values([Piece(lower, upper)])):
                if shift.value is None:
                    _fail("the accepted shift is outside the profile, so above its gap")
                _fail("the accepted shift is not in its gap")
            if shift_set.as_point() is None:
                scope.append(UNRESOLVED + "the accepted shift's place in its gap is undecidable on the published bounds")
    else:
        if p.placement != "above_spectrum" or p.placement_upper is not None or p.placement_established != n:
            _fail("the whole spectrum is counted above it, on its established count")
        lower = p.placement_lower
        if lower is None:
            _fail("the top eigenvalue is not published")
        if n == 0 or lower != omega(n - 1):
            _fail("the placement's top is not the top mode's ω")
        if shift_set is not None:
            if not shift_set.meets(Values([Piece(lower, LARGEST)])):
                _fail("the accepted whole-spectrum shift is below the top mode")
            if shift_set.as_point() is None:
                scope.append(UNRESOLVED + "the accepted shift's place above the spectrum is undecidable on the published bounds")
    return scope


def _identity_coverage(p):
    """The identity rows a run carries, from the document itself. Along each axis, in
    axis order: one nodal row for each straight member of the discretization table and
    nodal rows for nothing else, resultant rows for the other subjects, and the model's
    aggregate (subject kind `model`) last and once; the same (kind, subject) list on
    every axis. Subjects are told apart by their kind, so a member may be named
    `model`. The document carries no census of what bears mass, so only what is listed
    is covered."""
    axes = [row["axis"] for row in p.identity]
    if axes != sorted(axes, key=DIRECTIONS.index):
        _fail("identity: rows are not grouped in axis order")
    straight = set(p.straight_members)
    reference = None
    for axis in DIRECTIONS:
        rows = [row for row in p.identity if row["axis"] == axis]
        aggregate = [row for row in rows if row["kind"] == "model"]
        if not rows or len(aggregate) != 1 or rows[-1] is not aggregate[0] or rows[-1]["comparison"] != "resultant":
            _fail(f"identity: along {axis}, the model's aggregate resultant is not the single last row")
        keys = [(row["kind"], row["subject"]) for row in rows]
        if len(set(keys)) != len(keys):
            _fail(f"identity: a subject repeats along {axis}")
        nodal = {key for key, row in zip(keys, rows) if row["comparison"] == "nodal"}
        if nodal != {("member", member) for member in straight}:
            _fail(f"identity: the nodal rows along {axis} are not the discretization table's straight members")
        if reference is None:
            reference = keys
        elif keys != reference:
            _fail(f"identity: the subjects along {axis} differ from those along x")
    return ["identity: covered are the straight members the discretization table lists and the model's aggregate; "
            "the document has no census of mass-bearing members, arcs or component bodies, so a subject absent "
            "from every axis (and, for a straight member, from the discretization table as well) cannot be seen"]


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
        vos = (c["eta"], c["condition"], c["estimate"])
        for name, vo in zip(("η", "κ_λ", "e_λ"), vos):
            _producer_domain(vo, NORMAL_OR_ZERO, f"computed mode {i + 1}: {name}")
        if any(vo.value is None and Values.of(vo) is None for vo in vos):
            _fail(f"computed mode {i + 1}: a check-2 value is null, and its null states no bound")
        if any(vo.value is not None and vo.value < 0.0 for vo in vos):
            _fail(f"computed mode {i + 1}: a negative check-2 value")
        eta, kappa, e = (Values.of(vo) for vo in vos)
        # The producer multiplies in extended range and then requires 0 or normal
        # (reformation/mod.rs:964): a nonzero product below the normal range is no
        # published 0, so the point path decides only a product it kept.
        if all(x.as_point() is not None for x in (eta, kappa, e)) and _formed(
                (eta.as_point() * kappa.as_point(), eta.as_point(), kappa.as_point())):
            product = eta.as_point() * kappa.as_point()
            if not _close(e.as_point(), product, 8.0 * U * max(e.as_point(), product)):
                _fail(f"computed mode {i + 1}: e_λ is not η·κ_λ")
            continue
        # e_λ is the producer's η·κ_λ: a non-finite factor gives a non-finite e_λ.
        product = eta * kappa
        if not (e.nonfinite and product.nonfinite) and not _within_relative(e, Values(product.pieces), 8.0 * U):
            _fail(f"computed mode {i + 1}: e_λ = η·κ_λ is contradicted by the bounds")
        scope.append(UNRESOLVED + f"computed mode {i + 1}: e_λ = η·κ_λ is undecidable on the published bounds")
    tolerance = p.params.get("identity_tolerance")
    if not isinstance(tolerance, dict) or not _is_number(tolerance.get("relative")) or not _is_number(tolerance.get("absolute")):
        _fail("the identity tolerance is not bound")
    scope.extend(_identity_coverage(p))
    _state_coverage(p)
    if not any(text.startswith("numerical integrity does not establish physical correctness")
               for text in p.disclosure_texts.get("modal", [])):
        _fail("disclosures: the fixed numerical-integrity disclosure is missing")
    # The producer's domain for the relative discrepancy (plain binary64, never flagged)
    # holds whatever the tolerance: it is enforced unconditionally. Only the tolerance
    # comparison needs a comparison scale, which an absolute tolerance does not publish.
    for i, vo in enumerate(row["relative"] for row in p.identity):
        _producer_domain(vo, BINARY64, f"identity row {i}")
    if tolerance["absolute"] == 0.0:
        for i, vo in enumerate(row["relative"] for row in p.identity):
            d = _needed(vo, f"identity row {i}")
            if vo.value is not None and vo.value < 0.0:
                _fail(f"identity row {i}: a negative relative discrepancy")
            within = d.outcomes(lambda lo: lo <= tolerance["relative"], lambda hi: hi <= tolerance["relative"], (True, False))
            if True not in within:
                if vo.value is None:
                    _fail(f"identity row {i}: its relative discrepancy is at least 2⁵³, above the bound tolerance")
                _fail(f"identity row {i}: relative discrepancy exceeds the bound tolerance")
            if False in within:
                scope.append(UNRESOLVED + f"identity row {i}: its discrepancy against the tolerance is undecidable (not finite)")
    else:
        scope.append("an absolute identity tolerance: its scale is not published")
    for v in p.formation:
        if v is not None and v < 0.0:
            _fail("a negative formation-difference or norm")
    _producer_domain(p.ortho, TO_PUBLISHED, "the orthogonality deviation")
    _needed(p.ortho, "the orthogonality deviation")
    if p.ortho.value is not None and p.ortho.value < 0.0:
        _fail("a negative orthogonality deviation")
    if p.ortho_pair != (len(p.modes) >= 2):
        _fail("the orthogonality pair is stated exactly when two modes are reported")
    return scope


def _standing(p):
    tau_r, tau_o = p.params.get("residual_threshold"), p.params.get("orthogonality_threshold")
    if not _is_number(tau_r) or not _is_number(tau_o):
        _fail("τ_r or τ_o: not bound")
    # The producer's tests are η > τ_r and the deviation > τ_o: true for +∞, false for NaN.
    scope = []

    def above(vo, threshold, what):
        return _needed(vo, what).outcomes(lambda lo: lo > threshold, lambda hi: hi > threshold, (True, False))

    for m, mode in enumerate(p.modes):
        possible = above(p.computed[m]["eta"], tau_r, f"mode {m + 1}: η")
        stated = mode["sensitive"]
        if stated not in possible or (mode["reasons"] > 0) != stated:
            _fail(f"mode {m + 1}: Sensitive exactly when η > τ_r, with its reason")
        if len(possible) > 1:
            scope.append(UNRESOLVED + f"mode {m + 1}: η > τ_r is undecidable (η not finite)")
    # The set: Sensitive when a mode's token is, or the deviation is above τ_o.
    ortho = above(p.ortho, tau_o, "the orthogonality deviation")
    possible = {True} if any(mode["sensitive"] for mode in p.modes) else ortho
    if p.sensitive not in possible or (p.set_reasons > 0) != p.sensitive:
        _fail("the set is Sensitive exactly when a mode is or the orthogonality deviation exceeds τ_o")
    if len(possible) > 1:
        scope.append(UNRESOLVED + "the set's standing is undecidable (the deviation is not finite)")
    return scope


def _units(p):
    # Every kind has its catalog unit, so the standing is passed or failed, never scoped.
    for vo, key, path in p.unitized:
        _unit_check(vo, key, path)
    return []


# --- The verdict --------------------------------------------------------------------------

def _standing_of(function, *args):
    """A relation's standing: failed (its first difference); unresolved, when an instance
    is consistent with its operands' stated sets but not decided by them (never passed);
    scoped unsupported, when evidence is withheld or outside the catalog; else passed."""
    try:
        items = function(*args)
    except Failure as failure:
        return {"status": "failed", "difference": str(failure), "scope": [], "unresolved": []}
    except (KeyError, IndexError, TypeError, AttributeError, ValueError, ArithmeticError) as error:
        return {"status": "failed", "difference": f"malformed: {error!r}", "scope": [], "unresolved": []}
    unresolved = [x[len(UNRESOLVED):] for x in items if x.startswith(UNRESOLVED)]
    scope = [x for x in items if not x.startswith(UNRESOLVED)]
    status = "unresolved" if unresolved else ("scoped_unsupported" if scope else "passed")
    return {"status": status, "difference": None, "scope": scope, "unresolved": unresolved}


# What the verdict establishes, in every verdict (Design §5.3.4).
VERDICT_LIMIT = ("per-relation consistency, not joint realizability: each relation is decided on its own operands, "
                 "so a document every relation admits may still have no single coherent set of underlying values; "
                 "feasibility-oracle agreement does not establish payload coherence either")


def _validation(decision, relations):
    """`fully_validated` only when every mandatory governing relation has passed: an
    unresolved or scoped relation confers no validated or current-authority status."""
    unmet = sorted(name for name, r in relations.items() if r["status"] != "passed")
    return {"status": "fully_validated" if decision == "consistent" and not unmet else "not_fully_validated",
            "unmet": unmet}


def _unchecked():
    return {"status": "not_checked", "difference": None, "scope": [], "unresolved": []}


def _closure_failed(difference):
    return {"family": FAMILY, "closure": {"status": "failed", "difference": difference, "scope": []},
            "binding": dict(_unchecked(), not_checked=[]), "relations": {name: _unchecked() for name in RELATIONS},
            "health": "not_assessed", "sensitive": False, "refusal_codes": [], "decision": "rejected",
            "validation": {"status": "not_fully_validated", "unmet": list(RELATIONS)}, "limit": VERDICT_LIMIT}


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
    unresolved = any(r["status"] == "unresolved" for r in relations.values())
    scoped = any(r["status"] == "scoped_unsupported" for r in relations.values())
    decision = ("rejected" if failed else "consistent_with_unresolved" if unresolved
                else "consistent_with_scoped_unsupported" if scoped else "consistent")
    health = "not_assessed" if failed else ("sensitive" if parsed.sensitive else "checks_passed")
    return {"family": FAMILY, "closure": {"status": "passed", "difference": None, "scope": []}, "binding": bound,
            "relations": relations, "health": health, "sensitive": health == "sensitive", "refusal_codes": [], "decision": decision,
            "validation": _validation(decision, relations), "limit": VERDICT_LIMIT}


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
            "refusal_codes": codes, "decision": "rejected" if bound["status"] == "failed" else "refusal_consistent",
            "validation": {"status": "not_fully_validated", "unmet": list(RELATIONS)}, "limit": VERDICT_LIMIT}


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
