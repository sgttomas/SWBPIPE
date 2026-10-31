"""SI CAEPIPE MBF text writer for the pilot subset (DEL-17-04).

`render_caepipe_mbf_si` turns the SWBPIPE headless preview model (the
`solve.preview_model.model` object, of the current model-document version) into MBF text in the
SI unit set (mm, N, N*m, MPa, C, and bar for pressure), always Y vertical, following the public CAEPIPE User's Manual
appendix "Import MBF". Only a narrow pilot subset is written; everything else
is refused by name and listed in the loss report, never silently dropped.

SI units scale to the MBF unit by exact powers of ten. US-customary units
(in, lbf, psi, lbf*in, lbf*ft, lbf/in, degF, 1/degF) are normalized to the SI
profile by their exact definitions (Design `caepipe-mbf-import.md` §5.2, O-2),
and the result lists the factors (`MBF-SI-UNITS-NORMALIZED`). Every other unit
is refused. The other non-decimal conversions are stated in the result's
`assumptions` (radians to degrees, weight per length to weight density by
standard gravity, Poisson ratio from E and G).

The writer does not run CAEPIPE and makes no claim about how CAEPIPE reads or
analyses the text.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from fractions import Fraction
import hashlib
import json
import math
from pathlib import Path
from typing import Any, Iterable, Mapping

from . import correctly_rounded


MBF_SI_PROFILE_ID = "caepipe-mbf-si-pilot-1"
STANDARD_GRAVITY = 9.80665  # m/s^2; converts SWBPIPE weight per length to CAEPIPE weight density only.

MBF_VERSION = "15.10"
# The retired model document versions, refused by name (DEL-02-01 Design model-document-0.8.0.md §3.1); the
# current one is the schema's (`current_model_schema_version`).
RETIRED_MODEL_VERSIONS = ("0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0", "0.7.0")
# The target tool's labels for the load cases the writer requests.
LOAD_CASE_LABELS = {"sustained": "Sustained (W+P)", "expansion": "Expansion (T1)"}
DOFS = ("UX", "UY", "UZ", "RX", "RY", "RZ")
_DOF_TOKENS = {
    **{dof: dof for dof in DOFS},
    **{dof[0] + dof[1].lower(): dof for dof in DOFS},
    **{dof.lower(): dof for dof in DOFS},
}
_FORCE_DIRECTIONS = {"global_x": 0, "GLOBAL_X": 0, "global_y": 1, "GLOBAL_Y": 1, "global_z": 2, "GLOBAL_Z": 2}
_MOMENT_DIRECTIONS = {
    **{token: DOFS.index(dof) - 3 for token, dof in _DOF_TOKENS.items() if dof.startswith("R")},
    "rotation_x": 0,
    "rotation_y": 1,
    "rotation_z": 2,
}
_RIGID_FAMILIES = {None, "anchor", "guide", "line_stop", "vertical_support"}
_MECHANICAL_CATEGORIES = {"concentrated_force", "concentrated_moment", "weight"}
_THERMAL_CATEGORIES = {"thermal", "imposed_displacement"}

# Accepted SWBPIPE unit symbols (core/units/src/lib.rs) and the power of ten
# that scales each to the MBF SI unit. "C" is the units crate's legacy alias
# of degC for temperature dimensions (unit_by_symbol). The rotation table's
# "rad" entry is the one non-decimal conversion (radians to degrees).
_LENGTH_TO_MM = {"m": 3, "mm": 0}
_STRESS_TO_MPA = {"Pa": -6, "kPa": -3, "MPa": 0, "GPa": 3}
_FORCE_TO_N = {"N": 0, "kN": 3}
_MOMENT_TO_NM = {"N*m": 0, "kN*m": 3}
_FORCE_PER_LENGTH_TO_N_PER_M = {"N/m": 0, "N/mm": 3, "kN/m": 3, "kN/mm": 6}
_EXPANSION_TO_PER_C = {"1/K": 0, "1/degC": 0}
_TEMPERATURE_INTERVAL_TO_C = {"K": 0, "degC": 0, "C": 0}
_ROTATION_TO_DEG = {"rad": "rad", "deg": 0}
_DIMENSIONLESS = {"1": 0, "none": 0}
# Exact definitional normalization of US-customary units to the SI profile (Design §5.2, O-2): 1 in = 25.4 mm,
# 1 lb = 0.45359237 kg, 1 kgf = 9.80665 N, so 1 lbf = 0.45359237 x 9.80665 N; degF intervals x 5/9.
INCH_MM = Fraction("25.4")
POUND_FORCE_N = Fraction("0.45359237") * Fraction("9.80665")
NORMALIZATION = {
    "in": (INCH_MM, "1 in = 25.4 mm"),
    "lbf": (POUND_FORCE_N, "1 lbf = 0.45359237 kg x 9.80665 m/s^2 = 4.4482216152605 N"),
    "psi": (POUND_FORCE_N / INCH_MM**2, "1 psi = 1 lbf/in^2 = 4.4482216152605 N / 645.16 mm^2"),
    "lbf*in": (POUND_FORCE_N * INCH_MM / 1000, "1 lbf*in = 4.4482216152605 N x 0.0254 m"),
    "lbf*ft": (POUND_FORCE_N * 12 * INCH_MM / 1000, "1 lbf*ft = 4.4482216152605 N x 0.3048 m"),
    "lbf/in": (POUND_FORCE_N * 1000 / INCH_MM, "1 lbf/in = 4.4482216152605 N / 0.0254 m"),
    "1/degF": (Fraction(9, 5), "1/degF = 9/5 per degC"),
    "degF": (Fraction(5, 9), "a degF interval = 5/9 degC"),
}
ABSOLUTE_FAHRENHEIT = "T(degC) = (T(degF) - 32) x 5/9"
_LENGTH_TO_MM["in"] = NORMALIZATION["in"][0]
_STRESS_TO_MPA["psi"] = NORMALIZATION["psi"][0]
_FORCE_TO_N["lbf"] = NORMALIZATION["lbf"][0]
_MOMENT_TO_NM.update({"lbf*in": NORMALIZATION["lbf*in"][0], "lbf*ft": NORMALIZATION["lbf*ft"][0]})
_FORCE_PER_LENGTH_TO_N_PER_M["lbf/in"] = NORMALIZATION["lbf/in"][0]
# A spring's linear stiffness (core/units LinearStiffness, which reads force-per-length symbols) in N/mm, the unit of
# a flexible anchor's TRAK in SI files (Format of .MBF, TRAK).
_STIFFNESS_TO_N_PER_MM: dict[str, int | Fraction] = {"N/m": -3, "N/mm": 0, "kN/m": 0, "kN/mm": 3,
                                                      "lbf/in": NORMALIZATION["lbf/in"][0] / 1000}
_EXPANSION_TO_PER_C["1/degF"] = NORMALIZATION["1/degF"][0]
_TEMPERATURE_INTERVAL_TO_C["degF"] = NORMALIZATION["degF"][0]
# Pressure regions become LOADS Pressure 1 in bar (Format of .MBF, LOADS, SI): 1 bar = 1e5 Pa exactly, and psi by
# its exact definition (Design §9.4).
_PRESSURE_TO_BAR: dict[str, int | Fraction] = {"Pa": -5, "kPa": -2, "MPa": 1, "bar": 0,
                                                 "psi": NORMALIZATION["psi"][0] * 10}
PRESSURE_CONTRACT_VERSION = ("3.0.0", "exact_pressure_v3")
_SHEAR_MODELS = ("euler_bernoulli", "timoshenko_annulus_energy_v1")
_PRESSURE_BASIS = "internal_differential_zero_external_v1"
_CLOSURE_TRANSFERS = ("transfers_to_wall", "separately_supported_or_compensated")
# OPPRES as written with pressure: PD/4t, Bourdon effect off, pressure correction for bends off, peak pressure
# factor 1, Bourdon (off) in expansion cases. SWBPIPE's exact pressure route models neither effect (DEL-04-07 D-3).
OPPRES_LINE = "0,0,0,1,0"
# The OPCODE line written with an ASME B31 piping code (Format of .MBF, OPCODE): axial force in the code stress off,
# liberal allowable off (both code-stress settings only), B31J off. The other fields are not written.
OPCODE_LINE = "0,0,0"
# The piping code whose current edition the public manual names as computing SIFs and flexibility factors by B31J
# whatever OPCODE says (Analysis: "for ASME B31.1 (2020 or later)"); the writer warns rather than refuses.
B31J_LOCKED_CODES = ("B311",)
# The importer's refusals of a restored region by the exact route's value checks (straight runs, bores), which the
# printed digits can change; the writer's preflight reports them as MBF-SI-PRESSURE-REGION-FORMAT.
_REGION_CHECK_CODES = ("MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR", "MBF-IMPORT-PRESSURE-BORE-MISMATCH",
                       "MBF-IMPORT-PRESSURE-MITRE", "MBF-IMPORT-PRESSURE-GEOMETRY-INVALID")
_BEND_SOLVER_CONSUMPTION = "curved_bend_macro_element"
# A realized bend's directional SIF pair, written as the bend comment BSIF(in-plane,out-plane) (Format of .MBF, BSIF);
# the solve takes both or neither (DEL-04-07 Design `u4b-u5b-implementation.md` §2.6).
BEND_SIF_FIELDS = ("sif_in_plane_user_value", "sif_out_of_plane_user_value")
_BEND_RELATIVE_TOLERANCE = 1e-9  # tangent lengths, line intersection, bend angle and bend plane
_PROJECT_UNITS = {"length": ("m", "mm", "in"), "force": ("N", "lbf"), "temperature": ("degC", "degF")}

# The model-document schema whose single `schema_version` enum value is the current version.
MODEL_SCHEMA_PATH = Path(__file__).resolve().parents[3] / "schemas" / "model.schema.yaml"
# Material provenance that affirmatively identifies project-invented data or the user's own source (Design
# `caepipe-mbf-import.md` §7.4). The writer copies the matching origin into `material_map`; otherwise none.
# Both are values the model schema admits: a provenance_record with this certification and a public_permissive
# disposition is project-invented; one with the user-supplied certification is the user's own source.
INVENTED_CERTIFICATION = "invented non-engineering example"
USER_SUPPLIED_CERTIFICATION = "user-supplied own source"
MATERIAL_ORIGINS = ("project_invented", "user_supplied")

# The field sweep (Design §17.1). The fields each member's reader writes (to the MBF or the sidecar), checks against
# what is written, or refuses by name; every other non-empty field is listed once, as one omitted loss entry per field.
_HANDLED_MEMBERS = {"schema_version", "document_kind", "pressure_contract", "project", "materials", "nodes",
                    "pipe_segments", "supports", "components", "load_cases", "combinations",
                    "reference_configurations", "model_settings", "shear_model", "spectra"}
_HANDLED_FIELDS = {
    "materials": {"id", "elastic_modulus", "shear_modulus", "poisson_ratio", "thermal_expansion_coefficient",
                  "temperature_points", "expansion_laws"},
    "nodes": {"id", "position"},
    "pipe_segments": {"id", "from", "to", "section", "material", "y_reference"},
    "pipe_segments.section": {"outside_diameter", "wall_thickness", "corrosion_allowance", "mill_tolerance",
                              "insulation_thickness", "material_density", "contents_density", "insulation_density"},
    "supports": {"id", "node", "restraints", "family", "stiffness", "hanger", "nonlinear", "direction"},
    "components": {"id", "kind", "node", "geometry", "modifiers", "mechanics_interface"},
    "components.geometry": {"bend_pipe_ref", "bend_radius", "bend_angle"},
    "components.modifiers": {"flexibility_factor_user_value", *BEND_SIF_FIELDS},
    "components.mechanics_interface": {"solver_consumption"},
    "load_cases": {"id", "primitive_loads", "pressure_regions", "equivalent_static", "modulus_basis_ref",
                   "modulus_basis_temperature", "analysis_state", "condition_use", "test_condition"},
    "load_cases.primitive_loads": {"id", "category", "target", "direction", "magnitude", "dimension"},
    "load_cases.pressure_regions": {"id", "member_pipe_ids", "member_connector_ids", "pressure_basis", "pressure",
                                    "terminals"},
    "project": {"units"},
}
_NOT_CARRIED = "the MBF has no field for it and the sidecar does not carry it"
_OMITTED_REASONS = {
    "data_boundary": "the import writes its own data boundary for the local document",
    "analysis_status": "the analysis state is not in the MBF; the import starts it afresh",
    "sections": "each pipe segment's own section is written as a PIPE record; the section library is not",
    "diagnostics": "stored diagnostics are not in the MBF; the import's validation forms its own",
    "materials.constitutive_basis": "the MBF carries E and the Poisson ratio; the import forms its own basis",
    "materials.provenance_record": "only its origin classification is carried, in the sidecar's material_map",
    "pipe_segments.section_ref": "the pipe segment's own section is written; the section library is not",
    "supports.properties": "display properties the solve ignores",
    "components.completeness": "a derived finding; the import's validation forms its own",
    "components.modifiers.sif_user_value": "BSIF carries the directional pair; the scalar SIF is a different "
                                           "SWBPIPE field, which BSIF(s,s) would import as the pair",
    "components.modifiers.source_reference": "the import names the MBF line as the source reference",
    "load_cases.label": "the import labels each case by the target program's case (load_case_map)",
    "load_cases.kind": "the import forms each case's kind from the target program's case (load_case_map)",
    "load_cases.status": "the import sets each case's status",
    "load_cases.pressure_regions.provenance": "the import names the MBF line as the region's provenance",
}
# The project units an import of this writer's text sets (importer, SI mode, rotations in degrees); a model unit
# other than these is a display choice the MBF does not keep.
IMPORTED_PROJECT_UNITS = {"length": "mm", "force": "N", "moment": "N*m", "pressure": "MPa", "stress": "Pa",
                          "temperature": "degC", "angle": "deg"}

_TITLE_MAX = 70
_DESCRIPTION_MAX = 30
# The writer sidecar's format (Design §15). Format 2 records the effective value of every field, never null or another
# sentinel for a default to fill; format 3 is format 2 with its self-digest `sidecar_sha256`. A sidecar without
# `sidecar_format` is a legacy one, from main's tranche-1 writer.
SIDECAR_FORMAT = 3
SIDECAR_DIGEST_KEY = "sidecar_sha256"
_MAX_NODES = 9999  # node numbers 10, 20, ... stay below 99999


@dataclass(frozen=True)
class MbfSiResult:
    status: str
    text: str | None
    diagnostics: list[dict[str, Any]] = field(default_factory=list)
    node_map: dict[str, str] = field(default_factory=dict)
    element_map: dict[str, dict[str, Any]] = field(default_factory=dict)
    support_map: dict[str, dict[str, Any]] = field(default_factory=dict)
    load_case_map: dict[str, str] = field(default_factory=dict)
    material_map: dict[str, dict[str, Any]] = field(default_factory=dict)
    section_map: dict[str, dict[str, Any]] = field(default_factory=dict)
    load_map: dict[str, dict[str, Any]] = field(default_factory=dict)
    pressure_map: dict[str, Any] = field(default_factory=dict)
    layout_row_count: int = 0
    loss_report: list[dict[str, Any]] = field(default_factory=list)
    assumptions: list[str] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "profile_id": MBF_SI_PROFILE_ID,
            "status": self.status,
            "text": self.text,
            "diagnostics": [dict(item) for item in self.diagnostics],
            "node_map": dict(self.node_map),
            "element_map": {key: dict(value) for key, value in self.element_map.items()},
            "support_map": {key: dict(value) for key, value in self.support_map.items()},
            "load_case_map": dict(self.load_case_map),
            "material_map": {key: dict(value) for key, value in self.material_map.items()},
            "section_map": {key: dict(value) for key, value in self.section_map.items()},
            "load_map": {key: dict(value) for key, value in self.load_map.items()},
            "pressure_map": json.loads(json.dumps(self.pressure_map)),
            "layout_row_count": self.layout_row_count,
            "loss_report": [dict(item) for item in self.loss_report],
            "assumptions": list(self.assumptions),
        }

    def sidecar(self, mbf_sha256: str) -> dict[str, Any]:
        """The writer sidecar of the written text whose SHA-256 is `mbf_sha256` (Design §15), format 3: what a bound
        import restores and the MBF does not carry, each field with its effective value. What the MBF determines
        (layout order, record names, numbers, bend geometry, sections) is not repeated in it. Its `sidecar_sha256` is
        the self-digest of the rest (sidecar_self_digest)."""
        record = self.to_dict()

        def element(entry: Mapping[str, Any]) -> dict[str, Any]:
            kept = {key: entry[key] for key in ("from", "to", "reversed")}
            if "bend" in entry:
                return {**kept, "bend": {"component": entry["bend"]["component"]}}
            return {**kept, "y_reference": entry["y_reference"], "y_reference_authored": entry["y_reference_authored"]}

        pressure = record["pressure_map"]
        body = {
            "sidecar_format": SIDECAR_FORMAT,
            "node_map": record["node_map"],
            "element_map": {key: element(entry) for key, entry in record["element_map"].items()},
            "support_map": {key: {name: entry[name] for name in ("node", "restrained", "family", "family_authored")}
                            for key, entry in record["support_map"].items()},
            "load_case_map": record["load_case_map"],
            "material_map": {key: {name: entry[name] for name in ("swbpipe_material", "origin")}
                             for key, entry in record["material_map"].items()},
            "load_map": {key: {name: entry[name] for name in ("load_case", "category", "target", "direction")}
                         for key, entry in record["load_map"].items()},
            "pressure_map": {"contract": pressure["contract"], "shear_model": pressure["shear_model"],
                             "regions": [{name: region[name] for name in ("id", "members", "terminals")}
                                         for region in pressure["regions"]]},
            "mbf_sha256": mbf_sha256,
        }
        return {**body, SIDECAR_DIGEST_KEY: sidecar_self_digest(body)}


# Maps whose entries a JSON path names as map[key], as the importer's sidecar parser names them; other fields .field.
_KEYED_MAPS = ("node_map", "element_map", "support_map", "load_case_map", "material_map", "section_map", "load_map")
_ESCAPES = {'"': '\\"', "\\": "\\\\", "\b": "\\b", "\t": "\\t", "\n": "\\n", "\f": "\\f", "\r": "\\r"}


class SidecarNotCanonical(ValueError):
    """A sidecar value that has no canonical serialisation (sidecar_canonical_bytes), at the JSON path `path`."""

    def __init__(self, path: str, reason: str) -> None:
        super().__init__(f"{path}: {reason}")
        self.path, self.reason = path, reason


def sidecar_canonical_bytes(sidecar: Mapping[str, Any]) -> bytes:
    """The byte-exact canonical serialisation of `sidecar` without its `sidecar_sha256`, which the self-digest covers
    (Design §15): RFC 8785 (JCS). Members are sorted by key as UTF-16 code units, with no whitespace (`,` and `:`
    alone separate); a string escapes `"` and `\\` with a backslash, U+0008, U+0009, U+000A, U+000C and U+000D as
    `\\b \\t \\n \\f \\r`, the other controls below U+0020 as `\\u00xx` (lowercase hex), and nothing else; a number is
    the binary64 it denotes (1 and 1.0 are one value, -0 is 0), written as ECMAScript's Number::toString writes it;
    then `true`, `false`, `null`; the whole in UTF-8, no BOM and no final newline. A value with no such form (a
    non-finite number, an integer that is not exactly a binary64, a lone surrogate, a key that is not a string, or
    anything but a JSON value) raises SidecarNotCanonical naming its path."""
    def child(path: str, key: str) -> str:
        return f"{path}[{key}]" if path in _KEYED_MAPS else (f"{path}.{key}" if path else key)

    def text(value: str, path: str) -> str:
        try:
            value.encode("utf-8")
        except UnicodeEncodeError:
            raise SidecarNotCanonical(path, "holds a lone surrogate, which has no UTF-8 form") from None
        return '"' + "".join(_ESCAPES.get(char) or (f"\\u{ord(char):04x}" if char < " " else char)
                             for char in value) + '"'

    def walk(value: Any, path: str) -> str:
        if value is None or type(value) is bool:
            return {None: "null", True: "true", False: "false"}[value]
        if type(value) is int:
            try:
                exact = float(value) == value
            except OverflowError:
                exact = False
            if not exact:
                raise SidecarNotCanonical(path, "is an integer that is not exactly a binary64 number")
            return _canonical_number(float(value))
        if type(value) is float:
            if not math.isfinite(value):
                raise SidecarNotCanonical(path, "is not a finite number")
            return _canonical_number(value)
        if type(value) is str:
            return text(value, path)
        if type(value) is list:
            return "[" + ",".join(walk(item, f"{path}[{index}]") for index, item in enumerate(value)) + "]"
        if type(value) is dict:
            for key in value:
                if type(key) is not str:
                    raise SidecarNotCanonical(path, f"has a key {key!r} that is not a string")
                text(key, child(path, key))
            ordered = sorted(value, key=lambda key: key.encode("utf-16-be"))
            return "{" + ",".join(f"{text(key, child(path, key))}:{walk(value[key], child(path, key))}"
                                  for key in ordered) + "}"
        raise SidecarNotCanonical(path or "sidecar", f"is a {type(value).__name__}, not a JSON value")

    if type(sidecar) is not dict:
        raise SidecarNotCanonical("sidecar", "must be an object")
    return walk({key: value for key, value in sidecar.items() if key != SIDECAR_DIGEST_KEY}, "").encode("utf-8")


def sidecar_self_digest(sidecar: Mapping[str, Any]) -> str:
    """The sidecar's self-digest (Design §15): the lowercase hex SHA-256 of sidecar_canonical_bytes."""
    return hashlib.sha256(sidecar_canonical_bytes(sidecar)).hexdigest()


def _combination_member_0_9_0(combination: Mapping[str, Any], case_ids: set[Any],
                              combination_ids: set[Any]) -> tuple[str, str] | None:
    """Model document 0.9.0's combination members (§2.2, §2.4, §3.5): the code and what is refused, or None. An
    operand id names a load case first, and a combination only when no load case has it."""
    if "difference_basis" in combination:
        return "MBF-SI-DIFFERENCE-BASIS-UNSUPPORTED", "a difference basis"
    if "subtrahend_support_state" in combination:
        return "MBF-SI-SUBTRAHEND-SUPPORT-STATE-UNSUPPORTED", "a subtrahend support state"
    if combination.get("basis") in ("absolute_sum", "srss"):
        return "MBF-SI-COMBINATION-BASIS-UNSUPPORTED", f"the {combination['basis']} basis"
    terms = combination.get("terms") if isinstance(combination.get("terms"), list) else []
    ids = [combination.get(key) for key in ("minuend_id", "subtrahend_id")]
    ids += combination.get("operand_ids") if isinstance(combination.get("operand_ids"), list) else []
    nested = any(isinstance(term, Mapping) and "combination" in term for term in terms) or any(
        isinstance(item, str) and item not in case_ids and item in combination_ids for item in ids)
    if nested:
        return "MBF-SI-COMBINATION-NESTED-UNSUPPORTED", "a combination operand"
    return None


def _canonical_number(value: float) -> str:
    """ECMAScript's Number::toString of the finite binary64 `value` (RFC 8785 §3.2.2.3): the fewest decimal digits
    that read back as `value`, the nearest where several do (Python's repr gives these digits), placed as ECMAScript
    places them for the decimal exponent n of value = 0.digits x 10^n."""
    if value == 0:
        return "0"
    mantissa, _, exponent = repr(abs(value)).partition("e")
    whole, _, fraction = mantissa.partition(".")
    significant = (whole + fraction).lstrip("0")
    n = len(whole) + int(exponent or 0) - (len(whole + fraction) - len(significant))
    digits = significant.rstrip("0")
    k = len(digits)
    sign = "-" if value < 0 else ""
    if k <= n <= 21:
        return sign + digits + "0" * (n - k)
    if 0 < n <= 21:
        return sign + digits[:n] + "." + digits[n:]
    if -6 < n <= 0:
        return sign + "0." + "0" * -n + digits
    e = n - 1
    return sign + digits[0] + ("." + digits[1:] if k > 1 else "") + "e" + ("+" if e >= 0 else "-") + str(abs(e))


def _known(value: Any, table: Mapping[str, Any]) -> bool:
    """`value` is a string key of `table`: a model value of another type (a list, say) is never looked up."""
    return isinstance(value, str) and value in table


def documented_orientations(data: bytes) -> dict[tuple[str, str], tuple[float, float, float]] | None:
    """The importer's documented local y of each straight of `data` (Design §10.5); deferred: the importer imports
    this module."""
    from .importer import documented_orientations as derive
    return derive(data)


def current_model_schema_version() -> str:
    """The one `schema_version` the model-document schema admits; fails unless there is exactly one."""
    schema = json.loads(MODEL_SCHEMA_PATH.read_text(encoding="utf-8"))
    versions = schema["properties"]["schema_version"].get("enum") or []
    if len(versions) != 1 or not isinstance(versions[0], str):
        raise ValueError(f"{MODEL_SCHEMA_PATH.name}: schema_version must admit exactly one version (got {versions!r})")
    return versions[0]


def material_origin(material: Mapping[str, Any]) -> str | None:
    """`project_invented` or `user_supplied` only from affirmative provenance without a library_ref, else None."""
    record = material.get("provenance_record")
    if not isinstance(record, Mapping) or record.get("library_ref") is not None:
        return None
    if (record.get("contributor_certification") == INVENTED_CERTIFICATION
            and record.get("redistribution_status") == "public_permissive"):
        return "project_invented"
    if record.get("contributor_certification") == USER_SUPPLIED_CERTIFICATION:
        return "user_supplied"
    return None


def render_caepipe_mbf_si(
    model: Mapping[str, Any],
    *,
    title: str,
    reference_temperature_c: float = 20.0,
    max_layout_rows: int | None = 20,
    reference_temperature_f: float | None = None,
    piping_code: str | None = None,
) -> MbfSiResult:
    """Render the pilot subset of a SWBPIPE preview model as SI MBF text, or refuse by name.

    `reference_temperature_f`, when given, replaces `reference_temperature_c` by the exact definition
    (T - 32) x 5/9, and the normalization is listed.

    `piping_code` is the caller's choice of one documented OPTIONS piping-code token (Format of .MBF, OPTIONS). It
    is written as `OPTIONS HGRA,<code>,SI`, and an ASME B31 code also gets `OPCODE 0,0,0` (B31J off). Without one
    the text is unchanged (no code token) and the info diagnostic MBF-SI-PIPING-CODE-NOT-WRITTEN states that the
    target program's default code applies."""

    writer = _Writer(model, title, reference_temperature_c, max_layout_rows)
    if reference_temperature_f is not None:
        writer.t_ref_f = reference_temperature_f
    writer.piping_code = piping_code
    return writer.run()


def regenerate_caepipe_sidecar(
    model: Mapping[str, Any],
    *,
    mbf_sha256: str,
    derived_y_reference: Iterable[str],
    reference_temperature_c: float = 20.0,
    reference_temperature_f: float | None = None,
) -> tuple[dict[str, Any] | None, list[dict[str, Any]]]:
    """The sidecar this writer records for `model`, as `render_caepipe_mbf_si` records it, for the importer's
    regeneration identity (Design §14). Two inputs are not in the model: each straight named in `derived_y_reference`
    records its y_reference as derived (`y_reference_authored` false, the model's vector, which the import derived),
    and the export preflight is not run (the import is the caller). Returns (sidecar, []) or (None, the writer's
    error diagnostics)."""
    writer = _Writer(model, "REGENERATED", reference_temperature_c, None)
    if reference_temperature_f is not None:
        writer.t_ref_f = reference_temperature_f
    writer.regenerating = frozenset(derived_y_reference)
    result = writer.run()
    if result.status != "written":
        return None, [item for item in result.diagnostics if item["severity"] == "error"]
    return result.sidecar(mbf_sha256), []


class _Writer:
    def __init__(self, model: Any, title: str, t_ref: Any, max_layout_rows: int | None) -> None:
        self.model = model
        self.title = title
        self.t_ref = t_ref
        self.max_layout_rows = max_layout_rows
        self.diagnostics: list[dict[str, Any]] = []
        self.loss: list[dict[str, Any]] = []
        self.assumptions: list[str] = []
        self.length_exp: int | Fraction = 0
        self.normalized: dict[str, str] = {}
        self.t_ref_f: Any = None
        self.nodes: dict[str, tuple[float, float, float]] = {}
        self.materials: dict[str, dict[str, Any]] = {}
        self.pipes: dict[str, dict[str, Any]] = {}
        self.node_restraints: dict[str, set[str]] = {}
        self.node_supports: dict[str, list[str]] = {}
        self.supports: dict[str, dict[str, Any]] = {}
        self.node_springs: dict[str, dict[str, float]] = {}  # node -> translational DOF -> stiffness in N/mm
        self.case_labels: dict[str, str] = {}
        self.forces: dict[str, list[float]] = {}
        self.moments: dict[str, list[float]] = {}
        self.weight: dict[str, float] = {}
        self.delta_t: dict[str, float] = {}
        self.anchor_disp: dict[str, list[float]] = {}
        self.anchor_rot: dict[str, list[float]] = {}
        self.load_records: dict[str, dict[str, Any]] = {}
        self.bends: dict[str, dict[str, Any]] = {}
        self.tangent_points: dict[str, str] = {}
        self.rotation_converted = False
        self.contract: dict[str, Any] | None = None
        self.pipe_pressure: dict[str, float] = {}
        self.regions: list[dict[str, Any]] = []
        self.pressure_case: str | None = None
        self.regenerating: frozenset[str] | None = None  # regenerate_caepipe_sidecar: the derived straights
        self.piping_code: Any = None

    # -- diagnostics -----------------------------------------------------
    def diag(self, code: str, severity: str, entity: str, message: str) -> None:
        self.diagnostics.append({"code": code, "severity": severity, "entity": entity, "message": message})

    def refuse(self, code: str, entity: str, message: str) -> None:
        self.diag(code, "error", entity, message)
        self.loss.append({"entity": entity, "category": "unsupported", "code": code, "reason": message})

    def omit(self, code: str, entity: str, message: str) -> None:
        self.diag(code, "warning", entity, message)
        self.loss.append({"entity": entity, "category": "omitted", "code": code, "reason": message})

    def invalid(self, entity: str, message: str) -> None:
        self.diag("MBF-SI-INPUT-INVALID", "error", entity, message)

    def has_errors(self) -> bool:
        return any(item["severity"] == "error" for item in self.diagnostics)

    # -- value helpers ---------------------------------------------------
    def number(self, value: Any, entity: str, name: str) -> float | None:
        if isinstance(value, bool) or not isinstance(value, (int, float)):
            self.invalid(entity, f"{entity} {name} must be a number (got {value!r})")
            return None
        if not math.isfinite(value):
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity, f"{entity} {name} is not finite ({value!r})")
            return None
        return float(value)

    def quantity(self, record: Any, table: Mapping[str, Any], target: str, entity: str, name: str,
                 *, exact: bool = False) -> Any:
        """The quantity in the MBF unit, rounded once to binary64; with `exact`, the unrounded rational (the
        radians-to-degrees case is rounded once first), so sums round once at the end (Design §18.2)."""
        if not isinstance(record, Mapping) or "value" not in record or "unit" not in record:
            self.invalid(entity, f"{entity} {name} must be a {{value, unit}} quantity")
            return None
        unit = record["unit"]
        value = self.number(record["value"], entity, name)
        if value is None:
            return None
        if not isinstance(unit, str) or unit not in table:
            self.refuse(
                "MBF-SI-UNIT-UNSUPPORTED",
                entity,
                f"{entity} {name} unit {unit!r} is refused: only units with an exact power-of-ten scaling or an "
                f"exact definitional normalization to the MBF unit {target} are written ({', '.join(table)})",
            )
            return None
        scaling = table[unit]
        if scaling == "rad":
            self.rotation_converted = True
            degrees = math.degrees(value)  # the one non-rational factor: rounded once, before any sum
            if not math.isfinite(degrees) or (value != 0 and degrees == 0):
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity,
                          f"{entity} {name} is not representable in binary64 after conversion to degrees")
                return None
            return Fraction(degrees) if exact else degrees
        if isinstance(scaling, Fraction):
            self.normalized[unit] = NORMALIZATION[unit][1]
        value_exact = Fraction(value) * (scaling if isinstance(scaling, Fraction) else Fraction(10) ** scaling)
        return value_exact if exact else self.rounded(value_exact, entity, name)

    def rounded(self, value: Fraction, entity: str, name: str) -> float | None:
        """`value` rounded once to binary64, or a named refusal when it overflows or a non-zero value underflows to 0."""
        try:
            result = float(value)
        except OverflowError:
            result = math.inf
        if not math.isfinite(result) or (value != 0 and result == 0):
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity,
                      f"{entity} {name} is not representable in binary64 after conversion to the MBF unit")
            return None
        return result

    def finite(self, values: Any, entity: str, what: str) -> bool:
        """True when every derived value is finite; otherwise MBF-SI-VALUE-OUT-OF-RANGE, never an `inf` in the MBF."""
        values = values if isinstance(values, (list, tuple)) else [values]
        if all(math.isfinite(value) for value in values):
            return True
        self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity, f"{entity} {what} is not representable in binary64")
        return False

    def convert(self, value: float, scaling: int | Fraction, unit: str) -> float | None:
        """A power-of-ten scaling, or an exact definitional normalization rounded once to binary64."""
        if isinstance(scaling, Fraction):
            self.normalized[unit] = NORMALIZATION[unit][1]
        return self.rounded(Fraction(value) * (scaling if isinstance(scaling, Fraction) else Fraction(10) ** scaling),
                            "model.nodes", "position")

    # -- run -------------------------------------------------------------
    def run(self) -> MbfSiResult:
        if not isinstance(self.model, Mapping):
            self.invalid("model", "model must be an object")
            return self.refused()
        if self.t_ref_f is not None:
            if isinstance(self.t_ref_f, bool) or not isinstance(self.t_ref_f, (int, float)) or not math.isfinite(self.t_ref_f):
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", "reference_temperature_f",
                          f"reference temperature must be a finite number (got {self.t_ref_f!r})")
                return self.refused()
            self.t_ref = float((Fraction(self.t_ref_f) - 32) * Fraction(5, 9))
            self.normalized["degF absolute"] = ABSOLUTE_FAHRENHEIT
        if isinstance(self.t_ref, bool) or not isinstance(self.t_ref, (int, float)) or not math.isfinite(self.t_ref):
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", "reference_temperature_c",
                      f"reference temperature must be a finite number (got {self.t_ref!r})")
            return self.refused()
        self.t_ref = float(self.t_ref)
        self.check_piping_code()
        self.check_document()
        self.check_project_units()
        if self.has_errors():
            return self.refused()
        self.read_nodes()
        self.read_materials()
        self.read_pipes()
        self.read_components()
        self.read_supports()
        self.read_load_cases()
        self.read_other_members()
        if self.has_errors():
            return self.refused()
        self.resolve_bends()
        if self.has_errors():
            return self.refused()
        rows = self.layout()
        if self.has_errors():
            return self.refused()
        return self.render(rows)

    def refused(self) -> MbfSiResult:
        return MbfSiResult(
            status="refused",
            text=None,
            diagnostics=self.diagnostics,
            loss_report=self.loss,
            assumptions=self.assumptions,
        )

    def check_piping_code(self) -> None:
        """The caller's piping code: a documented OPTIONS token, or none, which is stated, never defaulted."""
        from .importer import PIPING_CODE_OPTIONS  # deferred: the importer imports this module

        code = self.piping_code
        if code is None:
            self.diag("MBF-SI-PIPING-CODE-NOT-WRITTEN", "info", "OPTIONS",
                      "no piping code was chosen, so OPTIONS carries none and the target program's default piping code "
                      "applies; the code decides the load cases offered and which code-dependent settings are "
                      "honoured (Analysis)")
        elif not isinstance(code, str) or code not in PIPING_CODE_OPTIONS:
            self.refuse("MBF-SI-PIPING-CODE-UNSUPPORTED", "OPTIONS",
                        f"piping code {code!r} is not one of the documented OPTIONS piping-code tokens (Format of .MBF)")
        elif code in B31J_LOCKED_CODES:
            self.diag("MBF-SI-PIPING-CODE-B31J-LOCKED", "warning", "OPTIONS",
                      f"piping code {code} is the current edition, for which the target program computes SIFs and "
                      "flexibility factors by B31J whatever OPCODE says (Analysis); OPCODE B31J off is written but "
                      "not honoured, and a tee's flexibility then differs from SWBPIPE's, which assembles none")

    def check_document(self) -> None:
        version = self.model.get("schema_version")
        # Model document 0.8.0 (DEL-02-01 Design model-document-0.8.0.md §3.1): only the current document is read;
        # a retired one is refused by name with the solve's code and text, any other version as not readable.
        current = current_model_schema_version()
        if version in RETIRED_MODEL_VERSIONS:
            self.diag("MODEL_SCHEMA_VERSION_RETIRED", "error", "model.schema_version",
                      f"model document {version} is retired; re-author it as model document {current} "
                      "(a pressure-free model omits pressure_contract). Nothing is migrated.")
        elif version != current:
            self.diag("MBF-SI-SCHEMA-VERSION", "error", "model.schema_version",
                      f"model schema_version {version!r} is not {current}, the version this writer reads")
        contract = self.model.get("pressure_contract")
        if contract is not None:
            if not (isinstance(contract, Mapping)
                    and (contract.get("version"), contract.get("mode")) == PRESSURE_CONTRACT_VERSION
                    and contract.get("shear_model") in _SHEAR_MODELS and set(contract) == {"version", "mode",
                                                                                             "shear_model"}):
                self.refuse("MBF-SI-PRESSURE-UNSUPPORTED", "model.pressure_contract",
                            "only the exact pressure contract 3.0.0/exact_pressure_v3 with an admitted shear_model "
                            "is exported")
            else:
                self.contract = dict(contract)
                self.omit("MBF-SI-SHEAR-MODEL-NOT-EXPORTED", "model.pressure_contract.shear_model",
                          f"shear_model {contract['shear_model']} is not in the MBF; the target program applies its "
                          "own beam theory, and the sidecar's pressure_map keeps the contract for a bound import")
        if self.model.get("shear_model") is not None:
            self.refuse("MBF-SI-SHEAR-MODEL-UNSUPPORTED", "model.shear_model", "shear_model is not exported")
        if self.model.get("reference_configurations"):
            self.refuse("MBF-SI-REFERENCE-CONFIGURATION-UNSUPPORTED", "model.reference_configurations",
                        "reference configurations are not exported by the pilot writer")
        # Model document 0.9.0 (DEL-02-01 Design model-document-0.9.0.md §2.4, §3.5): spectra are refused by name.
        if _present(self.model.get("spectra")):
            self.refuse("MBF-SI-SPECTRUM-UNSUPPORTED", "model.spectra",
                        "response spectra (model document 0.9.0) are not exported by the pilot writer")

    def check_project_units(self) -> None:
        project = self.model.get("project")
        units = project.get("units") if isinstance(project, Mapping) else None
        if not isinstance(units, Mapping):
            units = {}
        for key, accepted in _PROJECT_UNITS.items():
            unit = units.get(key)
            if unit not in accepted:
                self.refuse(
                    "MBF-SI-PROJECT-UNIT-UNSUPPORTED",
                    f"project.units.{key}",
                    f"project.units.{key} is {unit!r}; the SI MBF writer accepts only {' or '.join(accepted)} "
                    "and converts no other unit system",
                )
        if _known(units.get("length"), _LENGTH_TO_MM):
            self.length_exp = _LENGTH_TO_MM[units["length"]]

    # -- model members ----------------------------------------------------
    def read_nodes(self) -> None:
        for node in _list(self.model.get("nodes"), self, "model.nodes"):
            if not isinstance(node, Mapping) or not isinstance(node.get("id"), str):
                self.invalid("model.nodes", "each node needs a string id")
                continue
            entity = f"node:{node['id']}"
            if node["id"] in self.nodes:
                self.invalid(entity, f"node id {node['id']!r} is repeated")
                continue
            position = node.get("position")
            if not isinstance(position, Mapping):
                self.invalid(entity, f"{entity} position must be an object with x, y, z")
                continue
            coords = [self.number(position.get(axis), entity, f"position.{axis}") for axis in "xyz"]
            if any(value is None for value in coords):
                continue
            converted = tuple(self.convert(value, self.length_exp, "in") for value in coords)
            if any(value is None for value in converted):
                continue
            self.nodes[node["id"]] = converted  # type: ignore[assignment]

    def read_materials(self) -> None:
        for material in _list(self.model.get("materials"), self, "model.materials"):
            if not isinstance(material, Mapping) or not isinstance(material.get("id"), str):
                self.invalid("model.materials", "each material needs a string id")
                continue
            entity = f"material:{material['id']}"
            if material["id"] in self.materials:
                self.invalid(entity, f"material id {material['id']!r} is repeated")
                continue
            ok = True
            if material.get("temperature_points"):
                self.refuse("MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED", entity,
                            f"{entity} has temperature_points; temperature-dependent properties are not exported")
                ok = False
            if "expansion_laws" in material:
                self.refuse("MBF-SI-TEMPERATURE-DEPENDENT-MATERIAL-UNSUPPORTED", entity,
                            f"{entity} has expansion_laws; material expansion laws are not exported")
                ok = False
            e_mpa = self.quantity(material.get("elastic_modulus"), _STRESS_TO_MPA, "N/mm2", entity, "elastic_modulus")
            if e_mpa is not None and e_mpa <= 0:
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity, f"{entity} elastic_modulus must be positive")
                e_mpa = None
            g_mpa = None
            if material.get("shear_modulus") is not None:
                g_mpa = self.quantity(material["shear_modulus"], _STRESS_TO_MPA, "N/mm2", entity, "shear_modulus")
            nu = None
            if material.get("poisson_ratio") is not None:
                nu = self.quantity(material["poisson_ratio"], _DIMENSIONLESS, "1", entity, "poisson_ratio")
                if nu is not None and e_mpa is not None and g_mpa is not None and g_mpa > 0:
                    implied = e_mpa / (2.0 * (1.0 + nu))
                    if abs(implied - g_mpa) > 1e-9 * g_mpa:
                        self.omit("MBF-SI-SHEAR-MODULUS-NOT-EXPORTED", entity,
                                  f"{entity} gives both poisson_ratio and shear_modulus; the MBF carries the Poisson "
                                  f"ratio {_num(nu)}, whose implied G = {_num(implied)} MPa differs from the authored "
                                  f"G = {_num(g_mpa)} MPa")
            elif e_mpa is not None and g_mpa is not None:
                if g_mpa <= 0:
                    self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity, f"{entity} shear_modulus must be positive")
                else:
                    nu = e_mpa / (2.0 * g_mpa) - 1.0
                    self.assumptions.append(
                        f"{entity}: Poisson ratio derived as E/(2G) - 1 = {_num(nu)} "
                        f"(E = {_num(e_mpa)} MPa, G = {_num(g_mpa)} MPa)"
                    )
            elif e_mpa is not None:
                self.refuse("MBF-SI-MATERIAL-POISSON-MISSING", entity,
                            f"{entity} has neither poisson_ratio nor shear_modulus; the MBF needs a Poisson ratio")
            if nu is not None and not 0.0 < nu < 0.5:
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity,
                          f"{entity} Poisson ratio {_num(nu)} is outside (0, 0.5)")
                nu = None
            alpha = 0.0
            has_alpha = material.get("thermal_expansion_coefficient") is not None
            if has_alpha:
                alpha = self.quantity(material["thermal_expansion_coefficient"], _EXPANSION_TO_PER_C, "mm/mm/C",
                                      entity, "thermal_expansion_coefficient")
            if not ok or e_mpa is None or nu is None or alpha is None:
                continue
            self.materials[material["id"]] = {"E": e_mpa, "nu": nu, "alpha": alpha, "has_alpha": has_alpha,
                                              "origin": material_origin(material)}

    def read_pipes(self) -> None:
        for pipe in _list(self.model.get("pipe_segments"), self, "model.pipe_segments"):
            if not isinstance(pipe, Mapping) or not isinstance(pipe.get("id"), str):
                self.invalid("model.pipe_segments", "each pipe segment needs a string id")
                continue
            entity = f"pipe:{pipe['id']}"
            if pipe["id"] in self.pipes:
                self.invalid(entity, f"pipe segment id {pipe['id']!r} is repeated")
                continue
            start, end = pipe.get("from"), pipe.get("to")
            if not _known(start, self.nodes) or not _known(end, self.nodes):
                self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity,
                          f"{entity} connects {start!r} to {end!r}; both must be model nodes")
                continue
            if start == end:
                self.refuse("MBF-SI-GEOMETRY-UNSUPPORTED", entity, f"{entity} starts and ends at the same node")
                continue
            material_id = pipe.get("material")
            if not _known(material_id, self.materials):
                if not any(item["entity"] == f"material:{material_id}" for item in self.diagnostics):
                    self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity,
                              f"{entity} names material {material_id!r}, which is not in model.materials")
                continue
            y_reference = pipe.get("y_reference")
            section = self.read_section(pipe.get("section"), entity)
            if section is None:
                continue
            self.pipes[pipe["id"]] = {"from": start, "to": end, "material": material_id, "y_reference": y_reference,
                                      **section}

    def omit_orientation(self) -> None:
        orientation = [pipe_id for pipe_id, pipe in self.pipes.items()
                       if pipe["y_reference"] is not None and pipe_id not in self.bends]
        if orientation:
            self.omit("MBF-SI-ORIENTATION-NOT-EXPORTED", "model.pipe_segments.y_reference",
                      f"y_reference of {', '.join(orientation)} is not exported; element local axes follow the "
                      "target program's own convention")

    def read_section(self, section: Any, entity: str) -> dict[str, float] | None:
        if not isinstance(section, Mapping):
            self.invalid(entity, f"{entity} section must be an object")
            return None
        od = self.quantity(section.get("outside_diameter"), _LENGTH_TO_MM, "mm", entity, "section.outside_diameter")
        thk = self.quantity(section.get("wall_thickness"), _LENGTH_TO_MM, "mm", entity, "section.wall_thickness")
        ok = od is not None and thk is not None
        if ok and not (od > 0 and thk > 0 and 2 * thk < od):
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity,
                      f"{entity} needs 0 < wall_thickness < outside_diameter / 2 (got {_num(od)} mm, {_num(thk)} mm)")
            ok = False
        if self.contract is not None and section.get("corrosion_allowance") is None:
            # The exact pressure contract needs an explicit allowance on every section (the solve refuses the model
            # with EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED); the MBF's written 0 would author one.
            self.refuse("MBF-SI-CORROSION-ALLOWANCE-MISSING", entity,
                        f"{entity} has no corrosion_allowance under the exact pressure contract; the MBF would write "
                        "0, an allowance the model does not give")  # the pipe stays read: no cascade
        for name in ("corrosion_allowance", "mill_tolerance", "insulation_thickness"):
            if section.get(name) is None:
                continue
            value = self.quantity(section[name], _LENGTH_TO_MM, "mm", entity, f"section.{name}")
            if value is None:
                ok = False
            elif value != 0:
                self.refuse("MBF-SI-SECTION-UNSUPPORTED", entity,
                            f"{entity} section.{name} is nonzero; the pilot writer exports bare pipe only")
                ok = False
        for name in ("material_density", "contents_density", "insulation_density"):
            if section.get(name) is not None:
                self.omit("MBF-SI-SECTION-FIELD-NOT-EXPORTED", entity,
                          f"{entity} section.{name} is not exported; weight is exported only from weight primitives")
        if not ok:
            return None
        return {"od": od, "thk": thk}  # type: ignore[dict-item]

    def read_components(self) -> None:
        for component in _list(self.model.get("components"), self, "model.components"):
            identity = component.get("id") if isinstance(component, Mapping) else None
            kind = component.get("kind") if isinstance(component, Mapping) else None
            interface = component.get("mechanics_interface") if isinstance(component, Mapping) else None
            consumption = interface.get("solver_consumption") if isinstance(interface, Mapping) else None
            if kind == "bend" and consumption == _BEND_SOLVER_CONSUMPTION:
                self.read_bend(component)
                continue
            self.refuse("MBF-SI-COMPONENT-UNSUPPORTED", f"component:{identity}",
                        f"component {identity!r} of kind {kind!r} (solver_consumption {consumption!r}) is not "
                        "exported; the pilot writer exports straight pipe segments and bends realized as "
                        f"{_BEND_SOLVER_CONSUMPTION!r} only")
        self.omit_orientation()

    def read_bend(self, component: Mapping[str, Any]) -> None:
        entity = f"component:{component.get('id')}"
        geometry = component.get("geometry")
        modifiers = component.get("modifiers")
        if not isinstance(geometry, Mapping):
            self.invalid(entity, f"{entity} bend needs a geometry block with bend_pipe_ref and bend_radius")
            return
        pipe_id = geometry.get("bend_pipe_ref")
        if not _known(pipe_id, self.pipes):
            self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity,
                      f"{entity} bend_pipe_ref {pipe_id!r} is not an exported pipe segment")
            return
        if pipe_id in self.bends:
            self.refuse("MBF-SI-BEND-UNSUPPORTED", entity, f"{entity} realizes pipe:{pipe_id}, which another bend already realizes")
            return
        pipe = self.pipes[pipe_id]
        if component.get("node") not in (pipe["from"], pipe["to"]):
            self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity,
                      f"{entity} node {component.get('node')!r} is not an endpoint of pipe:{pipe_id}")
            return
        factor_record = modifiers.get("flexibility_factor_user_value") if isinstance(modifiers, Mapping) else None
        if factor_record is None:
            self.refuse("MBF-SI-BEND-FLEXIBILITY-MISSING", entity,
                        f"{entity} has no modifiers.flexibility_factor_user_value; the writer never supplies an "
                        "implicit or code-based bend flexibility factor")
            return
        factor = self.quantity(factor_record, _DIMENSIONLESS, "1", entity, "flexibility_factor_user_value")
        radius = self.quantity(geometry.get("bend_radius"), _LENGTH_TO_MM, "mm", entity, "bend_radius")
        angle = None
        if geometry.get("bend_angle") is not None:
            record = geometry["bend_angle"]
            unit = record.get("unit") if isinstance(record, Mapping) else None
            if unit not in ("rad", "deg"):
                self.refuse("MBF-SI-UNIT-UNSUPPORTED", entity, f"{entity} bend_angle unit {unit!r} is refused (rad or deg)")
                return
            angle = self.number(record.get("value"), entity, "bend_angle")
            if angle is not None and unit == "deg":
                angle = math.radians(angle)
        sif_records = [modifiers.get(name) for name in BEND_SIF_FIELDS]
        sifs: list[Any] | None = None
        if any(record is not None for record in sif_records):
            if any(record is None for record in sif_records):
                self.refuse("MBF-SI-BEND-SIF-INCOMPLETE", entity,
                            f"{entity} gives only one of modifiers.{BEND_SIF_FIELDS[0]} and modifiers."
                            f"{BEND_SIF_FIELDS[1]}; the pair is written together as BSIF or not at all")
                return
            sifs = [self.quantity(record, _DIMENSIONLESS, "1", entity, name)
                    for record, name in zip(sif_records, BEND_SIF_FIELDS)]
        if factor is None or radius is None or (sifs is not None and None in sifs):
            return
        if not factor > 0 or not radius > 0:
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity,
                      f"{entity} flexibility factor and bend radius must be positive")
            return
        if sifs is not None and not all(value > 0 for value in sifs):
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity,
                      f"{entity} directional SIFs must be positive (got {_num(sifs[0])}, {_num(sifs[1])})")
            return
        if not _is_vector(pipe["y_reference"]):
            self.invalid(entity, f"pipe:{pipe_id} needs a numeric y_reference to fix the bend plane")
            return
        self.bends[pipe_id] = {"component": component.get("id"), "entity": entity, "radius": radius, "k": factor,
                               "angle": angle, "sifs": sifs}

    def resolve_bends(self) -> None:
        """Place each bend at the tangent intersection (TI) of its adjacent straights, or refuse it by name."""
        incident: dict[str, list[str]] = {}
        for pipe_id, pipe in self.pipes.items():
            incident.setdefault(pipe["from"], []).append(pipe_id)
            incident.setdefault(pipe["to"], []).append(pipe_id)
        loaded = {record["target"].get("node") for record in self.load_records.values()}
        for pipe_id, bend in sorted(self.bends.items()):
            entity = bend["entity"]
            pipe = self.pipes[pipe_id]
            straights: dict[str, str] = {}
            for tangent in (pipe["from"], pipe["to"]):
                others = [other for other in incident.get(tangent, []) if other != pipe_id]
                if tangent in self.node_restraints or tangent in loaded:
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", entity,
                                f"{entity} tangent point {tangent!r} carries a support or load; the target program "
                                "reports bend tangent points as generated nodes, so the writer does not place one there")
                elif not others:
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", entity,
                                f"{entity} ends a run at tangent point {tangent!r}; a bend needs a straight pipe on both sides")
                elif len(others) > 1:
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", entity,
                                f"{entity} tangent point {tangent!r} joins {len(others) + 1} pipes; a branch at a bend "
                                "tangent point is not exported")
                elif others[0] in self.bends:
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", entity,
                                f"{entity} meets another bend at {tangent!r}; consecutive bends without a straight pipe "
                                "between them are not exported")
                else:
                    straights[tangent] = others[0]
            if len(straights) != 2:
                continue
            ti = self.tangent_intersection(pipe_id, bend, straights)
            if ti is None:
                continue
            bend.update(ti=ti, straights=straights)
            for tangent in straights:
                self.tangent_points[tangent] = pipe_id

    def tangent_intersection(self, pipe_id: str, bend: dict[str, Any],
                             straights: dict[str, str]) -> tuple[float, float, float] | None:
        entity = bend["entity"]
        pipe = self.pipes[pipe_id]
        near, far = pipe["from"], pipe["to"]

        def far_end(tangent: str) -> str:
            straight = self.pipes[straights[tangent]]
            return straight["to"] if straight["from"] == tangent else straight["from"]

        p, q = self.nodes[near], self.nodes[far]
        incoming, outgoing, chord = _sub(p, self.nodes[far_end(near)]), _sub(self.nodes[far_end(far)], q), _sub(q, p)
        if min(_norm(incoming), _norm(outgoing), _norm(chord)) == 0:
            self.refuse("MBF-SI-GEOMETRY-UNSUPPORTED", entity, f"{entity}: a bend or adjacent straight has zero length")
            return None
        d1, d2 = _unit(incoming), _unit(outgoing)
        cosine = max(-1.0, min(1.0, _dot(d1, d2)))
        det = 1.0 - cosine * cosine
        if det <= _BEND_RELATIVE_TOLERANCE:
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", entity,
                        f"{entity}: the straights at {near!r} and {far!r} are parallel; there is no tangent intersection")
            return None
        b1, b2 = _dot(d1, chord), _dot(d2, chord)
        t1 = (b1 - cosine * b2) / det
        t2 = (b2 - cosine * b1) / det
        ti = tuple(p[axis] + t1 * d1[axis] for axis in range(3))
        residual = _norm(_sub(_sub(q, tuple(t2 * value for value in d2)), ti))
        chord_length = _norm(chord)
        theta = correctly_rounded.acos(cosine)
        tangent_length = bend["radius"] * correctly_rounded.tan(theta / 2.0)
        if not self.finite([*ti, t1, t2, residual, tangent_length], entity, "tangent intersection"):
            return None
        tol = _BEND_RELATIVE_TOLERANCE
        if residual > tol * chord_length:
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", entity,
                        f"{entity}: the straights at {near!r} and {far!r} do not intersect (gap {_num(residual)} mm)")
            return None
        if not (abs(t1 - tangent_length) <= tol * tangent_length and abs(t2 - tangent_length) <= tol * tangent_length):
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", entity,
                        f"{entity}: tangent lengths {_num(t1)} mm and {_num(t2)} mm disagree with R tan(theta/2) = "
                        f"{_num(tangent_length)} mm (R = {_num(bend['radius'])} mm, theta = {_num(math.degrees(theta))} deg)")
            return None
        if bend["angle"] is not None and abs(bend["angle"] - theta) > tol * max(theta, 1.0):
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", entity,
                        f"{entity}: bend_angle {_num(bend['angle'])} rad disagrees with the {_num(theta)} rad between "
                        "the adjacent straights")
            return None
        y_ref = self.pipes[pipe_id]["y_reference"]
        y_raw = tuple(float(y_ref[axis]) for axis in "xyz")
        chord_unit = _unit(chord)
        side = _sub(y_raw, tuple(_dot(y_raw, chord_unit) * value for value in chord_unit))
        middle = tuple((p[axis] + q[axis]) / 2.0 for axis in range(3))
        toward_ti = _sub(ti, middle)
        if _norm(side) == 0 or _norm(toward_ti) == 0:
            aligned = False
        else:
            side_unit, ti_unit = _unit(side), _unit(toward_ti)
            aligned = _dot(side_unit, ti_unit) > 0 and _norm(_cross(side_unit, ti_unit)) <= tol
        if not aligned:
            self.refuse("MBF-SI-BEND-GEOMETRY-INCONSISTENT", entity,
                        f"{entity}: pipe:{pipe_id} y_reference does not point from the chord toward the tangent "
                        "intersection in the plane of the adjacent straights, so SWBPIPE's arc is not the corner bend "
                        "the target program would form")
            return None
        bend["theta"] = theta
        return ti  # type: ignore[return-value]

    def read_supports(self) -> None:
        for support in _list(self.model.get("supports"), self, "model.supports"):
            if not isinstance(support, Mapping) or not isinstance(support.get("id"), str):
                self.invalid("model.supports", "each support needs a string id")
                continue
            entity = f"support:{support['id']}"
            if support["id"] in self.supports:
                self.invalid(entity, f"support id {support['id']!r} is repeated")
                continue
            node = support.get("node")
            if not _known(node, self.nodes):
                self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity, f"{entity} is at unknown node {node!r}")
                continue
            family = support.get("family")
            # Model document 0.9.0 (§2.4, §3.5): a snubber is refused by name.
            if family == "snubber":
                self.refuse("MBF-SI-SNUBBER-UNSUPPORTED", entity,
                            f"{entity} is a snubber (model document 0.9.0), a dynamic-only restraint the pilot writer "
                            "does not export")
                continue
            support = self.axis_aligned(support, entity)
            if support is None:
                continue
            if family == "spring" and support.get("hanger") is None and support.get("nonlinear") is None:
                self.read_spring(support, entity, node)
                continue
            special = [name for name in ("stiffness", "hanger", "nonlinear") if support.get(name) is not None]
            if family not in _RIGID_FAMILIES or special:
                what = f"family {family!r}" if family not in _RIGID_FAMILIES else ", ".join(special)
                self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", entity,
                            f"{entity} ({what}) is not a rigid restraint or a spring; hangers and nonlinear or "
                            "gapped supports are not exported by the pilot writer")
                continue
            restraints = support.get("restraints")
            if not isinstance(restraints, list):
                self.invalid(entity, f"{entity} restraints must be a list")
                continue
            dofs = set()
            for token in restraints:
                if token not in _DOF_TOKENS:
                    self.invalid(entity, f"{entity} restraint {token!r} is not a frame DOF")
                    break
                dofs.add(_DOF_TOKENS[token])
            else:
                if not dofs:
                    self.omit("MBF-SI-SUPPORT-EMPTY", entity, f"{entity} restrains no DOF and is not exported")
                    continue
                if family == "anchor" and len(dofs) != 6:
                    self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", entity,
                                f"{entity} has family anchor but restrains only {', '.join(sorted(dofs))}")
                    continue
                self.supports[support["id"]] = {"node": node, "dofs": dofs, "family": family}
                self.node_restraints.setdefault(node, set()).update(dofs)
                self.node_supports.setdefault(node, []).append(support["id"])
        for node, dofs in sorted(self.node_restraints.items()):
            rotations = sorted(dof for dof in dofs if dof.startswith("R"))
            if not rotations or len(dofs) == 6:
                continue
            for support_id in self.node_supports[node]:
                if any(dof.startswith("R") for dof in self.supports[support_id]["dofs"]):
                    self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", f"support:{support_id}",
                                f"support:{support_id} at node {node!r} restrains rotation ({', '.join(rotations)}) "
                                "without being a full anchor; the target tool does not accept "
                                "rotational restraints (FIXR) in an imported MBF, so partial rotational restraints are "
                                "not exported")

        for node in sorted(set(self.node_springs) & set(self.node_restraints)):
            self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", f"node:{node}",
                        f"node {node!r} carries springs and rigid restraints; a spring is written as a flexible "
                        "anchor (KA with TRAK), which the importer reads alone at its node")

    def axis_aligned(self, support: Mapping[str, Any], entity: str) -> Mapping[str, Any] | None:
        """Model document 0.9.0 (§2.1, §3.5): an exactly axis-aligned `direction` (one nonzero component) is the
        record on that axis, as the solve's lowering reads it: the direction is removed and the record's single DOF
        token (restraint and stiffness) becomes UX, UY or UZ for it. Any other direction is refused by name."""
        if "direction" not in support:
            return support
        direction = support["direction"]
        components = [direction.get(axis) for axis in ("x", "y", "z")] if isinstance(direction, Mapping) else []
        numbers = len(components) == 3 and all(isinstance(c, (int, float)) and not isinstance(c, bool)
                                                and math.isfinite(c) for c in components)
        nonzero = [index for index, c in enumerate(components) if numbers and c != 0]
        restraints = support.get("restraints")
        stiffness = support.get("stiffness")
        tokens = {_DOF_TOKENS.get(token) for token in restraints} if isinstance(restraints, list) else {None}
        if isinstance(stiffness, Mapping):
            tokens.add(_DOF_TOKENS.get(stiffness.get("dof")))
        if not (numbers and len(nonzero) == 1 and len(tokens) == 1 and next(iter(tokens)) in DOFS[:3]):
            self.refuse("MBF-SI-SUPPORT-DIRECTION-UNSUPPORTED", entity,
                        f"{entity} has a direction (model document 0.9.0) that is not exactly along one global axis "
                        "on a single translational DOF; a skewed restraint is not exported")
            return None
        token = DOFS[nonzero[0]]
        lowered = {key: value for key, value in support.items() if key != "direction"}
        lowered["restraints"] = [token for _ in restraints]
        if isinstance(stiffness, Mapping):
            lowered["stiffness"] = {**stiffness, "dof": token}
        return lowered

    def read_spring(self, support: Mapping[str, Any], entity: str, node: str) -> None:
        """A spring support (Design §12, tranche 3): one translational DOF, named by its stiffness and its restraint
        list alike, with a positive stiffness. It is written as a flexible anchor KA,TRAK(kx,ky,kz),ROTK(0,0,0) in
        N/mm; a zero TRAK component leaves its DOF free (technical reference, Anchor). Rotational springs are not
        written: ROTK's unit is open (TBD-17-04-010)."""
        stiffness = support.get("stiffness")
        restraints = support.get("restraints")
        if not isinstance(stiffness, Mapping) or not isinstance(restraints, list):
            self.invalid(entity, f"{entity} is a spring and needs a stiffness and a restraints list")
            return
        dof = _DOF_TOKENS.get(stiffness.get("dof")) if isinstance(stiffness.get("dof"), str) else None
        tokens = [_DOF_TOKENS.get(token) if isinstance(token, str) else None for token in restraints]
        if dof is None or tokens != [dof]:
            self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", entity,
                        f"{entity} is a spring whose restraints and stiffness must name the same single DOF")
            return
        if dof not in DOFS[:3]:
            self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", entity,
                        f"{entity} is a rotational spring ({dof}); ROTK's unit is open (TBD-17-04-010), so only "
                        "translational springs are written")
            return
        value = self.quantity(stiffness.get("value"), _STIFFNESS_TO_N_PER_MM, "N/mm", entity, "stiffness")
        if value is None:
            return
        if not value > 0:
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity,
                      f"{entity} stiffness must be positive (the solve refuses a spring without one)")
            return
        springs = self.node_springs.setdefault(node, {})
        if dof in springs:
            self.refuse("MBF-SI-SUPPORT-UNSUPPORTED", entity,
                        f"{entity} is a second spring on {dof} at node {node!r}; TRAK carries one stiffness per DOF")
            return
        springs[dof] = value
        self.supports[support["id"]] = {"node": node, "dofs": {dof}, "family": "spring"}

    def is_anchor(self, node: Any) -> bool:
        return isinstance(node, str) and len(self.node_restraints.get(node, ())) == 6

    def read_load_cases(self) -> None:
        kinds: dict[str, str] = {}
        seen: set[str] = set()
        for case in _list(self.model.get("load_cases"), self, "model.load_cases"):
            if not isinstance(case, Mapping) or not isinstance(case.get("id"), str):
                self.invalid("model.load_cases", "each load case needs a string id")
                continue
            entity = f"load_case:{case['id']}"
            if case["id"] in seen:
                self.invalid(entity, f"load case id {case['id']!r} is repeated")
                continue
            seen.add(case["id"])
            regions = self.read_regions(case.get("pressure_regions"), entity)
            for name in ("equivalent_static", "modulus_basis_ref", "modulus_basis_temperature", "analysis_state"):
                if case.get(name) is not None:
                    self.refuse("MBF-SI-LOAD-CASE-FEATURE-UNSUPPORTED", entity, f"{entity} {name} is not exported")
            # Model document 0.9.0 (§1.2, §2.4, §3.5): a case's condition use and test condition are refused by name.
            if "condition_use" in case:
                self.refuse("MBF-SI-CONDITION-USE-UNSUPPORTED", entity,
                            f"{entity} uses operating conditions (condition_use, model document 0.9.0), which the "
                            "pilot writer does not export")
            if "test_condition" in case:
                self.refuse("MBF-SI-TEST-CONDITION-UNSUPPORTED", entity,
                            f"{entity} is a hydrostatic test condition (test_condition, model document 0.9.0), which "
                            "the pilot writer does not export")
            loads = case.get("primitive_loads") or []
            if not isinstance(loads, list):
                self.invalid(entity, f"{entity} primitive_loads must be a list")
                continue
            categories = {load.get("category") for load in loads if isinstance(load, Mapping)}
            unsupported = categories - _MECHANICAL_CATEGORIES - _THERMAL_CATEGORIES
            for load in loads:
                if isinstance(load, Mapping) and load.get("category") in unsupported:
                    code = "MBF-SI-PRESSURE-UNSUPPORTED" if load["category"] in {"pressure", "hydrotest"} else "MBF-SI-LOAD-UNSUPPORTED"
                    self.refuse(code, f"load:{case['id']}/{load.get('id')}",
                                f"load {load.get('id')!r} in {entity} has category {load['category']!r}, which the pilot "
                                "writer does not export")
            if not loads and not regions:
                self.omit("MBF-SI-LOAD-CASE-EMPTY", entity, f"{entity} has no primitive loads and is not exported")
                continue
            mechanical = bool(categories & _MECHANICAL_CATEGORIES) or bool(regions)
            thermal = bool(categories & _THERMAL_CATEGORIES)
            if regions and thermal:
                self.refuse("MBF-SI-PRESSURE-CASE-UNSUPPORTED", entity,
                            f"{entity} has pressure regions and thermal or displacement primitives; the target program "
                            f"applies Pressure 1 with {LOAD_CASE_LABELS['sustained']!r} and its operating cases, not "
                            f"with {LOAD_CASE_LABELS['expansion']!r}")
                continue
            if mechanical and thermal:
                self.refuse("MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED", entity,
                            f"{entity} mixes mechanical (force, moment, weight) and thermal/displacement primitives; "
                            f"each case must map to exactly one of {LOAD_CASE_LABELS['sustained']!r} or {LOAD_CASE_LABELS['expansion']!r}")
                continue
            if unsupported and not (mechanical or thermal):
                continue
            label = LOAD_CASE_LABELS["sustained" if mechanical else "expansion"]
            other = next((key for key, value in kinds.items() if value == label), None)
            if other is not None:
                self.refuse("MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED", entity,
                            f"{entity} and load_case:{other} both map to {label!r}; at most one case of each kind is exported")
                continue
            kinds[case["id"]] = label
            if regions:
                self.pressure_case = case["id"]
                for region, pressure in regions:
                    self.regions.append(region)
                    for pipe_id in region["member_pipe_ids"]:
                        self.pipe_pressure[pipe_id] = pressure
            for load in loads:
                if isinstance(load, Mapping) and load.get("category") not in unsupported:
                    self.read_load(case["id"], label, load)
        self.case_labels = kinds
        self.round_sums()
        combinations = _list(self.model.get("combinations"), self, "model.combinations")
        cases = self.model.get("load_cases")
        case_ids = {case.get("id") for case in (cases if isinstance(cases, list) else []) if isinstance(case, Mapping)}
        combination_ids = {item.get("id") for item in combinations if isinstance(item, Mapping)}
        for combination in combinations:
            identity = combination.get("id") if isinstance(combination, Mapping) else None
            refusal = _combination_member_0_9_0(combination, case_ids, combination_ids) \
                if isinstance(combination, Mapping) else None
            if refusal is not None:
                code, what = refusal
                self.refuse(code, f"combination:{identity}",
                            f"combination {identity!r} carries {what} (model document 0.9.0), which the pilot writer "
                            "does not export")
                continue
            self.omit("MBF-SI-COMBINATION-NOT-EXPORTED", f"combination:{identity}",
                      f"combination {identity!r} is not exported; the target program forms its own combinations")

    def read_regions(self, regions: Any, entity: str) -> list[tuple[dict[str, Any], float]]:
        """The case's non-zero pressure regions with their pressure in bar, rounded once (Design §9.4)."""
        if regions is None:
            return []
        if not isinstance(regions, list):
            self.invalid(entity, f"{entity} pressure_regions must be a list")
            return []
        if regions and self.contract is None:
            self.refuse("MBF-SI-PRESSURE-UNSUPPORTED", entity,
                        f"{entity} has pressure_regions without the exact pressure contract, which are not solved "
                        "and not exported")
            return []
        out, owned = [], set()
        for region in regions:
            identity = region.get("id") if isinstance(region, Mapping) else None
            where = f"{entity}/region:{identity}"
            members = region.get("member_pipe_ids") if isinstance(region, Mapping) else None
            terminals = region.get("terminals") if isinstance(region, Mapping) else None
            if not (isinstance(identity, str) and isinstance(members, list) and members
                    and all(isinstance(item, str) and item in self.pipes and item not in owned for item in members)
                    and len(set(members)) == len(members)
                    and region.get("pressure_basis") == _PRESSURE_BASIS
                    and isinstance(region.get("provenance"), str)
                    and isinstance(terminals, list) and len(terminals) == 2
                    and all(isinstance(item, Mapping) and _known(item.get("node_ref"), self.nodes)
                            and item.get("closure_transfer") in _CLOSURE_TRANSFERS
                            and isinstance(item.get("provenance"), str) and item["provenance"].strip()
                            for item in terminals)):
                self.invalid(where, f"{where} needs an id, unique exported member pipes, the basis {_PRESSURE_BASIS}, "
                             "a provenance and two terminals at model nodes with a closure transfer and provenance")
                continue
            if region.get("member_connector_ids"):
                self.refuse("MBF-SI-PRESSURE-UNSUPPORTED", where, f"{where} has member connectors, which are not "
                            "exported")
                continue
            owned.update(members)
            value = self.quantity(region.get("pressure"), _PRESSURE_TO_BAR, "bar", where, "pressure", exact=True)
            if value is None:
                continue
            if value < 0:
                self.refuse("MBF-SI-PRESSURE-UNSUPPORTED", where,
                            f"{where} has a negative (external) pressure differential, which is not exported")
                continue
            if value == 0:
                self.omit("MBF-SI-PRESSURE-REGION-ZERO", where,
                          f"{where} has zero pressure; it writes Pressure 1 = 0, which imports as no region")
                continue
            pressure = self.rounded(value, where, "pressure")
            if pressure is not None:
                out.append((region, pressure))
        return out

    def round_sums(self) -> None:
        """Round each summed load once: the primitives on one target were added as exact rationals in the MBF
        unit, so cancellation loses nothing (Design §18.2)."""
        for name, store in (("forces", self.forces), ("moments", self.moments), ("displacements", self.anchor_disp),
                            ("rotations", self.anchor_rot)):
            for node, values in store.items():
                rounded = [self.rounded(value, f"node:{node}", f"summed {name}") for value in values]
                store[node] = [0.0 if value is None else value for value in rounded]
        for name, store in (("weight per length", self.weight), ("thermal change", self.delta_t)):
            for pipe, value in store.items():
                rounded = self.rounded(value, f"pipe:{pipe}", f"summed {name}")
                store[pipe] = 0.0 if rounded is None else rounded

    def read_load(self, case_id: str, label: str, load: Mapping[str, Any]) -> None:
        load_id = load.get("id")
        entity = f"load:{case_id}/{load_id}"
        category = load["category"]
        target = load.get("target")
        if not isinstance(target, Mapping):
            self.invalid(entity, f"{entity} target must be an object")
            return
        direction = load.get("direction")
        dimension = load.get("dimension")
        # The effective direction: the authored one, or the token the import forms (it reads no thermal or weight
        # direction from the MBF).
        record: dict[str, Any] = {"load_case": case_id, "caepipe_load_case": label, "category": category,
                                  "direction": direction if direction is not None else
                                  {"thermal": "global_x", "weight": "global_y"}.get(category)}
        if category in {"concentrated_force", "concentrated_moment"}:
            force = category == "concentrated_force"
            node = target.get("node")
            if target.get("type") != "node" or not _known(node, self.nodes):
                self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity, f"{entity} must target a model node")
                return
            directions = _FORCE_DIRECTIONS if force else _MOMENT_DIRECTIONS
            if direction not in directions or dimension != ("force" if force else "moment"):
                self.refuse("MBF-SI-LOAD-UNSUPPORTED", entity,
                            f"{entity} direction {direction!r} / dimension {dimension!r} is not exported; "
                            + ("global_x/y/z force" if force else "global RX/RY/RZ moment") + " only")
                return
            value = self.quantity(load.get("magnitude"), _FORCE_TO_N if force else _MOMENT_TO_NM,
                                  "N" if force else "N*m", entity, "magnitude", exact=True)
            if value is None:
                return
            store = self.forces if force else self.moments
            store.setdefault(node, [Fraction(0)] * 3)[directions[direction]] += value
            record.update(target={"node": node}, mbf="FOR" if force else "MOM")
        elif category == "weight":
            pipe = target.get("pipe")
            if target.get("type") != "element" or not _known(pipe, self.pipes):
                self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity, f"{entity} must target a model pipe segment")
                return
            if dimension != "force_per_length":
                self.refuse("MBF-SI-LOAD-UNSUPPORTED", entity, f"{entity} weight needs dimension force_per_length")
                return
            value = self.quantity(load.get("magnitude"), _FORCE_PER_LENGTH_TO_N_PER_M, "N/m", entity, "magnitude",
                                  exact=True)
            if value is None:
                return
            if direction not in {"global_y", "GLOBAL_Y"} or value > 0:
                self.refuse("MBF-SI-LOAD-UNSUPPORTED", entity,
                            f"{entity} weight acts along {direction!r} with magnitude {_num(value)} N/m; only global -Y "
                            "weight (the target program's default gravity) is exported")
                return
            self.weight[pipe] = self.weight.get(pipe, Fraction(0)) + value
            record.update(target={"pipe": pipe}, mbf="MATERIAL weight density")
        elif category == "thermal":
            pipe = target.get("pipe")
            if target.get("type") != "element" or not _known(pipe, self.pipes):
                self.diag("MBF-SI-REFERENCE-UNRESOLVED", "error", entity, f"{entity} must target a model pipe segment")
                return
            if dimension not in {"temperature_change", "temperature_interval"}:
                self.refuse("MBF-SI-LOAD-UNSUPPORTED", entity, f"{entity} thermal needs a temperature-interval dimension")
                return
            value = self.quantity(load.get("magnitude"), _TEMPERATURE_INTERVAL_TO_C, "C", entity, "magnitude",
                                  exact=True)
            if value is None:
                return
            material_id = self.pipes[pipe]["material"]
            if value != 0 and not self.materials[material_id]["has_alpha"]:
                self.refuse("MBF-SI-MATERIAL-EXPANSION-MISSING", entity,
                            f"{entity} changes the temperature of pipe:{pipe}, whose material {material_id!r} has no "
                            "thermal_expansion_coefficient")
                return
            self.delta_t[pipe] = self.delta_t.get(pipe, Fraction(0)) + value
            record.update(target={"pipe": pipe}, mbf="LOADS Temperature 1")
        else:  # imposed_displacement
            support_id = target.get("support")
            dof = _DOF_TOKENS.get(target.get("dof")) if _known(target.get("dof"), _DOF_TOKENS) else None
            support = self.supports.get(support_id) if _known(support_id, self.supports) else None
            if target.get("type") != "support" or support is None:
                self.refuse("MBF-SI-IMPOSED-DISPLACEMENT-UNSUPPORTED", entity,
                            f"{entity} must target an exported rigid support (got {dict(target)!r})")
                return
            if dof is None or (_DOF_TOKENS.get(direction) if _known(direction, _DOF_TOKENS) else None) != dof:
                self.invalid(entity, f"{entity} target dof and direction must name the same frame DOF")
                return
            if not self.is_anchor(support["node"]) or dof not in support["dofs"]:
                self.refuse("MBF-SI-IMPOSED-DISPLACEMENT-UNSUPPORTED", entity,
                            f"{entity} moves support {support_id!r}, which does not map to an anchor restraining {dof}; "
                            "only anchor displacements are exported")
                return
            translation = dof.startswith("U")
            if dimension != ("displacement" if translation else "rotation"):
                self.refuse("MBF-SI-LOAD-UNSUPPORTED", entity,
                            f"{entity} {dof} needs dimension {'displacement' if translation else 'rotation'}")
                return
            value = self.quantity(load.get("magnitude"), _LENGTH_TO_MM if translation else _ROTATION_TO_DEG,
                                  "mm" if translation else "deg", entity, "magnitude", exact=True)
            if value is None:
                return
            store = self.anchor_disp if translation else self.anchor_rot
            store.setdefault(support["node"], [Fraction(0)] * 3)[DOFS.index(dof) % 3] += value
            record.update(target={"support": support_id, "dof": dof}, mbf="D" if translation else "ROT")
        self.load_records[f"{case_id}/{load_id}"] = record

    def read_other_members(self) -> None:
        if self.model.get("model_settings") is not None:
            self.omit("MBF-SI-MODEL-SETTINGS-NOT-EXPORTED", "model.model_settings",
                      "model_settings (input validation settings) are not exported")
        self.sweep_fields()

    def sweep_fields(self) -> None:
        """Design §17.1: every non-empty model member and field no reader writes, checks or refuses is listed, one
        omitted loss entry per member or field naming its entities, never dropped silently."""
        for name, value in self.model.items():
            if name not in _HANDLED_MEMBERS and _present(value):
                self.omit("MBF-SI-MEMBER-NOT-EXPORTED", f"model.{name}",
                          f"model.{name} is not exported; {_OMITTED_REASONS.get(name, _NOT_CARRIED)}")
        unused = [material_id for material_id in self.materials
                  if material_id not in {pipe["material"] for pipe in self.pipes.values()}]
        if unused:
            self.omit("MBF-SI-FIELD-NOT-EXPORTED", "model.materials",
                      f"materials {', '.join(unused)} are not exported; the MBF writes only the materials its pipe "
                      "segments use")
        found: dict[str, list[str]] = {}

        def sweep(record: Any, path: str, entity: str) -> None:
            if isinstance(record, Mapping):
                for key, value in record.items():
                    if key not in _HANDLED_FIELDS[path] and _present(value):
                        found.setdefault(f"{path}.{key}", []).append(entity)

        project = self.model.get("project")
        sweep(project, "project", "project")
        units = project.get("units") if isinstance(project, Mapping) else None
        if isinstance(units, Mapping):
            kept = {key: value for key, value in units.items() if IMPORTED_PROJECT_UNITS.get(key) != value}
            if kept:
                found["project.units"] = [f"{key} {value}" for key, value in kept.items()]
        prefixes = {"materials": "material", "nodes": "node", "pipe_segments": "pipe", "supports": "support",
                    "components": "component", "load_cases": "load_case"}
        for member, prefix in prefixes.items():
            records = self.model.get(member)
            for record in records if isinstance(records, list) else []:
                if not isinstance(record, Mapping):
                    continue
                entity = f"{prefix}:{record.get('id')}"
                sweep(record, member, entity)
                if member == "pipe_segments":
                    sweep(record.get("section"), "pipe_segments.section", entity)
                elif member == "components" and record.get("kind") == "bend":
                    for part in ("geometry", "modifiers", "mechanics_interface"):
                        sweep(record.get(part), f"components.{part}", entity)
                elif member == "load_cases":
                    for part, kind in (("primitive_loads", "load"), ("pressure_regions", "region")):
                        items = record.get(part)
                        for item in items if isinstance(items, list) else []:
                            label = (f"load:{record.get('id')}/" if kind == "load" else f"{entity}/region:")
                            sweep(item, f"load_cases.{part}", f"{label}{item.get('id') if isinstance(item, Mapping) else None}")
        for path, entities in found.items():
            if path == "project.units":
                self.omit("MBF-SI-FIELD-NOT-EXPORTED", "model.project.units",
                          f"project.units ({', '.join(entities)}) are not kept; values are written in the MBF's SI "
                          "units, and the import sets the project units "
                          + ", ".join(f"{key} {value}" for key, value in IMPORTED_PROJECT_UNITS.items()))
                continue
            self.omit("MBF-SI-FIELD-NOT-EXPORTED", f"model.{path}",
                      f"{path.split('.', 1)[1]} of {', '.join(entities)} is not exported; "
                      f"{_OMITTED_REASONS.get(path, _NOT_CARRIED)}")

    # -- layout ---------------------------------------------------------
    def layout(self) -> list[dict[str, Any]]:
        incident: dict[str, list[tuple[str, str]]] = {}
        for pipe_id in sorted(self.pipes):
            pipe = self.pipes[pipe_id]
            incident.setdefault(pipe["from"], []).append((pipe_id, pipe["to"]))
            incident.setdefault(pipe["to"], []).append((pipe_id, pipe["from"]))
        for node in sorted(set(self.nodes) - set(incident)):
            entity = f"node:{node}"
            referenced = node in self.node_restraints or any(
                record.get("target", {}).get("node") == node for record in self.load_records.values()
            )
            if referenced:
                self.refuse("MBF-SI-TOPOLOGY-UNSUPPORTED", entity,
                            f"{entity} carries a support or load but no pipe segment")
            else:
                self.omit("MBF-SI-NODE-NOT-EXPORTED", entity, f"{entity} is not connected to any pipe segment")
        if len(incident) > _MAX_NODES:
            self.refuse("MBF-SI-NODE-NUMBER-LIMIT", "model.nodes",
                        f"{len(incident)} connected nodes exceed the {_MAX_NODES} node numbers 10..{_MAX_NODES * 10}")
        if not self.pipes:
            self.refuse("MBF-SI-TOPOLOGY-UNSUPPORTED", "model.pipe_segments", "the model has no exportable pipe segment")
        # Row keys: a model node id, or ("TI", bend pipe id) for a bend row at its tangent intersection.
        # A bend row stands for the incoming straight plus the bend and ends (for continuation) at the far
        # tangent point; its offsets end at the tangent intersection.
        rows: list[dict[str, Any]] = []
        written: set[str] = set()
        used: set[str] = set()
        while True:
            remaining = sorted(set(incident) - written)
            if not remaining:
                break
            anchors = [node for node in remaining if self.is_anchor(node)]
            plain = [node for node in remaining if node not in self.tangent_points]
            root = anchors[0] if anchors else (plain or remaining)[0]
            rows.append({"kind": "F", "node": root, "end": root, "element": None, "new": True})
            written.add(root)
            stack = [root]
            while stack:
                current = stack[-1]
                step = next(((pipe_id, other) for pipe_id, other in incident[current] if pipe_id not in used), None)
                if step is None:
                    stack.pop()
                    continue
                pipe_id, other = step
                used.add(pipe_id)
                if other in written:
                    self.refuse("MBF-SI-TOPOLOGY-UNSUPPORTED", f"pipe:{pipe_id}",
                                f"pipe:{pipe_id} closes a loop at node {other!r}; closed loops are not exported")
                    continue
                if pipe_id in self.bends:
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", self.bends[pipe_id]["entity"],
                                f"{self.bends[pipe_id]['entity']} is reached from its tangent point {current!r} without "
                                "its incoming straight; the layout cannot place it")
                    continue
                if rows[-1]["end"] != current:
                    if current in self.tangent_points:
                        self.refuse("MBF-SI-BEND-UNSUPPORTED", f"node:{current}",
                                    f"the layout would restart at bend tangent point {current!r}")
                    rows.append({"kind": "F", "node": current, "end": current, "element": None, "new": False})
                bend_id = self.tangent_points.get(other)
                if bend_id is None:
                    rows.append({"kind": "T", "node": other, "end": other, "element": pipe_id, "new": True})
                    written.add(other)
                    stack.append(other)
                    continue
                bend_pipe = self.pipes[bend_id]
                far = bend_pipe["to"] if bend_pipe["from"] == other else bend_pipe["from"]
                if far in written:
                    self.refuse("MBF-SI-TOPOLOGY-UNSUPPORTED", f"pipe:{bend_id}",
                                f"pipe:{bend_id} closes a loop at node {far!r}; closed loops are not exported")
                    continue
                used.add(bend_id)
                rows.append({"kind": "T", "node": ("TI", bend_id), "end": far, "element": pipe_id, "bend": bend_id,
                             "near": other, "far": far, "new": True})
                written.update((other, far))
                stack.append(far)
        return rows

    def position(self, key: Any) -> tuple[float, float, float]:
        if isinstance(key, tuple):
            return self.bends[key[1]]["ti"]
        return self.nodes[key]

    def pipe_attributes(self, pipe_id: str, materials: dict, sections: dict, loads: dict,
                        material_map: dict, section_map: dict, load_entries: dict,
                        temperatures: list[float]) -> tuple[str, str, str]:
        pipe = self.pipes[pipe_id]
        material = self.materials[pipe["material"]]
        density = 0.0
        if pipe_id in self.weight:
            od_m, thk_m = pipe["od"] / 1000.0, pipe["thk"] / 1000.0
            area = math.pi / 4.0 * (od_m * od_m - (od_m - 2.0 * thk_m) * (od_m - 2.0 * thk_m))  # inf, not OverflowError
            density = -self.weight[pipe_id] / (area * STANDARD_GRAVITY) if area > 0 else math.inf
            if not math.isfinite(density) or (self.weight[pipe_id] != 0 and density == 0):
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", f"pipe:{pipe_id}",
                          f"pipe:{pipe_id} weight density is not representable in binary64")
        material_key = (pipe["material"], _num(density))
        if material_key not in materials:
            name = f"M{len(materials) + 1}"
            materials[material_key] = name
            material_map[name] = {
                "swbpipe_material": pipe["material"],
                "weight_density_kgf_per_m3": density,
                "poisson_ratio": material["nu"],
                "elastic_modulus_n_per_mm2": material["E"],
                "thermal_expansion_mm_per_mm_per_c": material["alpha"],
                "temperatures_c": temperatures,
                "elements": [],
            }
            material_map[name]["origin"] = material["origin"] or "undeclared"  # the import then needs a declaration
        section_key = (_num(pipe["od"]), _num(pipe["thk"]))
        if section_key not in sections:
            name = f"P{len(sections) + 1}"
            sections[section_key] = name
            section_map[name] = {"outside_diameter_mm": pipe["od"], "wall_thickness_mm": pipe["thk"], "elements": []}
        delta = self.delta_t.get(pipe_id, 0.0)
        pressure = self.pipe_pressure.get(pipe_id, 0.0)
        load_key = (_num(delta), _num(pressure))
        if load_key not in loads:
            name = f"L{len(loads) + 1}"
            loads[load_key] = name
            load_entries[name] = (self.t_ref + delta, pressure)
            self.finite(load_entries[name][0], f"pipe:{pipe_id}", "operating temperature (reference + thermal change)")
        names = (materials[material_key], sections[section_key], loads[load_key])
        material_map[names[0]]["elements"].append(pipe_id)
        section_map[names[1]]["elements"].append(pipe_id)
        return names

    # -- rendering ------------------------------------------------------
    def render(self, rows: list[dict[str, Any]]) -> MbfSiResult:
        numbers: dict[Any, int] = {}
        node_map: dict[str, str] = {}
        for row in rows:
            if row["node"] not in numbers:
                numbers[row["node"]] = 10 * (len(numbers) + 1)
            number = numbers[row["node"]]
            if "bend" in row:
                node_map[row["near"]] = f"{number}A"
                node_map[row["far"]] = f"{number}B"
            else:
                node_map.setdefault(row["node"], str(number))
        element_rows = [row for row in rows if row["kind"] == "T"]
        deltas = [self.delta_t.get(pipe_id, 0.0) for pipe_id in self.pipes]
        t_hi = self.t_ref + max(max(abs(value) for value in deltas), 100.0)
        t_lo = self.t_ref + min(0.0, min(deltas))
        if not self.finite([t_lo, t_hi], "MATERIAL", "temperature table (reference temperature and thermal changes)"):
            return self.refused()
        temperatures = sorted({t_lo, self.t_ref, t_hi})

        materials: dict[tuple[str, str], str] = {}
        sections: dict[tuple[str, str], str] = {}
        loads: dict[tuple[str, str], str] = {}
        material_map: dict[str, dict[str, Any]] = {}
        section_map: dict[str, dict[str, Any]] = {}
        element_map: dict[str, dict[str, Any]] = {}
        load_entries: dict[str, tuple[float, float]] = {}
        attributes: dict[str, tuple[str, str, str]] = {}
        order = 0
        start = rows[0]["end"]
        for layout_row, row in enumerate(rows):
            if row["kind"] == "F":
                start = row["end"]
                continue
            pipe_id = row["element"]
            names = self.pipe_attributes(pipe_id, materials, sections, loads, material_map, section_map,
                                         load_entries, temperatures)
            attributes[pipe_id] = names
            members = [(pipe_id, start, row["near"] if "bend" in row else row["node"])]
            if "bend" in row:
                bend_id = row["bend"]
                bend_names = self.pipe_attributes(bend_id, materials, sections, loads, material_map, section_map,
                                                  load_entries, temperatures)
                if bend_names != names:
                    self.refuse("MBF-SI-BEND-UNSUPPORTED", self.bends[bend_id]["entity"],
                                f"{self.bends[bend_id]['entity']}: pipe:{bend_id} and its incoming straight pipe:{pipe_id} "
                                "differ in material, weight, section, temperature or pressure; the target program writes them as "
                                "one layout row with one set of properties")
                attributes[bend_id] = bend_names
                members.append((bend_id, row["near"], row["far"]))
            for member_id, member_start, member_end in members:
                member = self.pipes[member_id]
                element_map[member_id] = {
                    "from": node_map[member_start],
                    "to": node_map[member_end],
                    "layout_order": order,
                    "layout_row": layout_row,
                    "reversed": member_start != member["from"],
                    "material": attributes[member_id][0],
                    "section": attributes[member_id][1],
                    "load": attributes[member_id][2],
                }
                if member_id not in self.bends:
                    # Straight-pipe orientation is not in the MBF; the sidecar carries the effective one for bound
                    # imports: the authored vector, or else CAEPIPE's documented local y the import would assign.
                    # On regeneration a straight the import derived records the model's vector, that derivation.
                    derived = self.regenerating is not None and member_id in self.regenerating
                    given = _is_vector(member["y_reference"])
                    element_map[member_id]["y_reference"] = (  # an unauthored one is derived from the text below
                        {axis: float(member["y_reference"][axis]) for axis in "xyz"} if given else None)
                    element_map[member_id]["y_reference_authored"] = given and not derived
                order += 1
            if "bend" in row:
                bend = self.bends[row["bend"]]
                element_map[row["bend"]]["bend"] = {
                    "component": bend["component"],
                    "radius_mm": bend["radius"],
                    "angle_deg": math.degrees(bend["theta"]),
                    "flexibility_factor": bend["k"],
                    "tangent_intersection_mm": list(bend["ti"]),
                    "caepipe_row_node": str(numbers[row["node"]]),
                }
                if bend["sifs"] is not None:
                    element_map[row["bend"]]["bend"].update(sif_in_plane=bend["sifs"][0], sif_out_of_plane=bend["sifs"][1])
            start = row["end"]
        if self.has_errors():
            return self.refused()

        lines = [_title(self.title), "VERSION", MBF_VERSION, "OPTIONS",
                 f"HGRA,{self.piping_code},SI" if self.piping_code is not None else "HGRA,SI"]
        if self.piping_code is not None and self.piping_code.startswith("B31"):
            lines += ["OPCODE", OPCODE_LINE]
        lines += ["OPTEMP", _num(self.t_ref)]
        if self.pipe_pressure:
            lines += ["OPPRES", OPPRES_LINE]
        lines.append("MATERIAL")
        for name, record in material_map.items():
            description = _sanitize(record["swbpipe_material"], _DESCRIPTION_MAX) or name
            lines.append(f"{name},{_num(record['weight_density_kgf_per_m3'])},{_num(record['poisson_ratio'])},1,1,CS,{description}")
            for temperature in temperatures:
                lines.append(f"{name},{_num(temperature)},{_num(record['elastic_modulus_n_per_mm2'])},"
                             f"{_num(record['thermal_expansion_mm_per_mm_per_c'])}")
        lines.append("PIPE")
        for name, record in section_map.items():
            lines.append(f"{name},{_num(record['outside_diameter_mm'])},{_num(record['wall_thickness_mm'])},0,0,0,0,0,0")
        lines.append("LOADS")
        for name, (temperature, pressure) in load_entries.items():
            lines.append(f"{name},{_num(temperature)},{_num(pressure)},0")
        lines.append("LAYOUT")

        carried: tuple[str | None, str | None, str | None] = (None, None, None)
        first_element = element_rows[0]["element"]
        for index, row in enumerate(rows):
            node = row["node"]
            bend = self.bends[row["bend"]] if "bend" in row else None
            fields = [f"{row['kind']}{numbers[node]}"]
            comments: list[str] = []
            if bend is not None:
                fields.append("KI")
                comments.append(f"FF={_num(bend['k'])}")
                if bend["sifs"] is not None:
                    comments.append("BSIF({},{})".format(*(_num(value) for value in bend["sifs"])))
            if row["new"] and (self.is_anchor(node) or node in self.node_springs):
                fields.append("KA")
            pipe_id = row["element"] if row["kind"] == "T" else (first_element if index == 0 else None)
            if pipe_id is not None:
                wanted = attributes[pipe_id]
                if wanted[0] != carried[0]:
                    fields.append(f"M{wanted[0]}")
                if wanted[1] != carried[1]:
                    fields.append(f"P{wanted[1]}")
                if wanted[2] != carried[2]:
                    comments.append(f"L={wanted[2]}")
                carried = wanted
            if bend is not None:
                fields.append(f"B{_num(bend['radius'])}")
            x, y, z = self.position(node)
            if row["kind"] == "F":
                if index > 0 or any(value != 0 for value in (x, y, z)):
                    fields.extend([f"X{_num(x)}", f"Y{_num(y)}", f"Z{_num(z)}"])
            else:
                px, py, pz = self.position(rows[index - 1]["node"])
                offsets = [(axis, value) for axis, value in zip("XYZ", (x - px, y - py, z - pz)) if value != 0]
                self.finite([value for _, value in offsets], f"pipe:{pipe_id}", "layout offset")
                if not offsets:
                    self.refuse("MBF-SI-GEOMETRY-UNSUPPORTED", f"pipe:{pipe_id}", f"pipe:{pipe_id} has zero length")
                fields.extend(f"{axis}{_num(value)}" for axis, value in offsets)
            if row["new"] and bend is None:
                comments.extend(self.node_comments(node))
            if comments:
                comments[0] = "C" + comments[0]
            lines.append(",".join(fields + comments))
        if self.has_errors():  # degenerate geometry (a zero-length pipe) is refused by name before any derivation
            return self.refused()
        text = "".join(line + "\r\n" for line in lines)
        # An unauthored straight records the documented local y that the import derives from this text's own
        # positions, so the record and the derivation agree exactly (Design §15).
        derived = documented_orientations(text.encode("ascii"))
        for member_id, entry in element_map.items():
            if entry.get("y_reference_authored") is False and entry["y_reference"] is None:
                vector = None if derived is None else derived.get((entry["from"], entry["to"]))
                if vector is None:
                    self.refuse("MBF-SI-PREFLIGHT-REFUSED", f"pipe:{member_id}",
                                f"pipe:{member_id} has no authored y_reference, and the import does not read this MBF "
                                "far enough to derive its documented local y")
                    continue
                entry["y_reference"] = dict(zip("xyz", vector))
        if self.has_errors():
            return self.refused()

        layout_row_count = 1 + len(rows)
        if self.max_layout_rows is not None and layout_row_count > self.max_layout_rows:
            self.diag("MBF-SI-LAYOUT-ROW-LIMIT", "warning", "LAYOUT",
                      f"{layout_row_count} layout rows (title row plus {len(rows)} F/T rows) exceed {self.max_layout_rows}; "
                      "a row-limited (evaluation) build of the target program may refuse the file")

        support_map = {}
        for support_id, support in self.supports.items():
            support_map[support_id] = {
                "node": node_map[support["node"]],
                "caepipe_kind": ("anchor" if self.is_anchor(support["node"]) or support["family"] == "spring"
                                 else "restraint"),
                "restrained": [dof for dof in DOFS if dof in support["dofs"]],
                # the effective family: the authored one, or the solve's inference (anchor with six DOFs, else guide)
                "family": support["family"] or ("anchor" if len(support["dofs"]) == len(DOFS) else "guide"),
                "family_authored": support["family"] is not None,
            }
        if self.normalized:
            self.diag("MBF-SI-UNITS-NORMALIZED", "info", "project.units",
                      "US-customary units are normalized to the SI profile by their exact definitions: "
                      + "; ".join(self.normalized[unit] for unit in sorted(self.normalized)))
        load_map = {}
        for key, record in self.load_records.items():
            entry = dict(record)
            target = entry["target"]
            if "node" in target:
                entry["caepipe_node"] = node_map[target["node"]]
            elif "pipe" in target:
                entry["caepipe_element"] = dict(element_map[target["pipe"]])
                if entry["category"] == "thermal":
                    entry["mbf"] = f"LOADS {element_map[target['pipe']]['load']} Temperature 1"
                else:
                    entry["mbf"] = f"MATERIAL {element_map[target['pipe']]['material']} weight density"
            else:
                entry["caepipe_node"] = node_map[self.supports[target["support"]]["node"]]
            load_map[key] = entry
        # The effective pressure state of every model: the contract or "none", the shear model (the route's
        # Euler-Bernoulli without a contract) and the regions.
        pressure_map: dict[str, Any] = {"contract": "none", "shear_model": "euler_bernoulli", "regions": []}
        if self.contract is not None:
            pressure_map = {"contract": {"version": self.contract["version"], "mode": self.contract["mode"]},
                            "shear_model": self.contract["shear_model"], "regions": [
                {"id": region["id"], "members": list(region["member_pipe_ids"]),
                 "terminals": [{key: terminal[key] for key in ("node_ref", "closure_transfer", "provenance")}
                               for terminal in region["terminals"]],
                 "provenance": region["provenance"], "pressure_bar": self.pipe_pressure[region["member_pipe_ids"][0]]}
                for region in self.regions]}
        self.add_assumptions(temperatures)
        result = MbfSiResult(
            status="written",
            text=text,
            diagnostics=self.diagnostics,
            node_map=node_map,
            element_map=element_map,
            support_map=support_map,
            load_case_map=dict(self.case_labels),
            material_map=material_map,
            section_map=section_map,
            load_map=load_map,
            pressure_map=pressure_map,
            layout_row_count=layout_row_count,
            loss_report=self.loss,
            assumptions=self.assumptions,
        )
        try:  # the self-digest covers canonical JSON content only (Design §15)
            result.sidecar("0" * 64)
        except SidecarNotCanonical as error:
            self.invalid(error.path, f"the sidecar's {error.path} {error.reason}; the sidecar has no self-digest")
            return self.refused()
        # The preflight is total (Design §14): every export but the regeneration route's, whose caller is the import.
        if self.regenerating is None and not self.survives_reimport(result):
            return self.refused()
        return result

    def survives_reimport(self, result: MbfSiResult) -> bool:
        """Design §9.4, §12 and §14: every export as the printed MBF gives it back. The writer's own bound re-import
        reads the printed values exactly as an import will (positions summed from the printed offsets, bend tangent
        nodes rebuilt, OD and wall as printed) and applies the exact route's whole-region checks to the restored
        regions in their declared order. The export stands only if that re-import succeeds completely and checks
        every region. A failed region check is refused as MBF-SI-PRESSURE-REGION-FORMAT; any other refusal, at any
        stage, as MBF-SI-PREFLIGHT-REFUSED carrying the importer's code. Nothing is cut or reordered."""
        from hashlib import sha256
        from .importer import import_caepipe_mbf  # deferred: the importer imports this module

        data = result.text.encode("ascii")  # type: ignore[union-attr]
        sidecar = result.sidecar(sha256(data).hexdigest())
        # Only an undeclared material takes a declaration; a recorded origin is the import's (Design §7.4).
        materials = sidecar.get("material_map")
        undeclared = {name: {"origin": "project_invented", "source_name": "writer preflight; never output"}
                      for name, entry in (materials.items() if isinstance(materials, Mapping) else ())
                      if isinstance(entry, Mapping) and entry.get("origin") == "undeclared"}
        back = import_caepipe_mbf(data, sidecar=sidecar, source_label="writer preflight (geometry only)",
                                  material_provenance=undeclared)
        prefix = "pressure_map region "
        for item in back.diagnostics:
            if item["severity"] != "error":
                continue
            if item["code"] in _REGION_CHECK_CODES and item["location"].startswith(prefix):
                region = item["location"][len(prefix):]
                self.refuse("MBF-SI-PRESSURE-REGION-FORMAT", f"region:{region}",
                            f"region {region} passes the exact route's checks on the model's values but not on the "
                            f"MBF's printed values (10 significant digits): {item['code']}: {item['message']}")
            else:
                self.refuse("MBF-SI-PREFLIGHT-REFUSED", item["location"] or "MBF",
                            f"the bound re-import of this MBF refuses it ({item['code']} at {item['location']}, line "
                            f"{item['line']}): {item['message']}")
        if (back.status == "imported" and self.contract is not None
                and back.summary.get("pressure_regions") != len(self.regions)):
            self.refuse("MBF-SI-PREFLIGHT-REFUSED", "pressure_map",
                        f"the bound re-import restored {back.summary.get('pressure_regions')} pressure regions, not "
                        f"the {len(self.regions)} written; the region checks did not cover every region")
        if back.status != "imported" and not self.has_errors():
            self.refuse("MBF-SI-PREFLIGHT-REFUSED", "MBF", "the bound re-import of this MBF did not complete")
        return not self.has_errors()

    def node_comments(self, node: str) -> list[str]:
        comments = []
        springs = self.node_springs.get(node)
        if springs:  # a flexible anchor: one stiffness per translation, 0 free; rotations free
            comments.append("TRAK({})".format(",".join(_num(springs.get(dof, 0.0)) for dof in DOFS[:3])))
            comments.append("ROTK(0,0,0)")
        restrained = self.node_restraints.get(node, set())
        if restrained and not self.is_anchor(node):
            translations = [1 if dof in restrained else 0 for dof in DOFS[:3]]
            rotations = [1 if dof in restrained else 0 for dof in DOFS[3:]]
            if any(translations):
                comments.append("FIXD({},{},{})".format(*translations))
            if any(rotations):  # unreachable: read_supports refuses partial rotational restraints
                raise AssertionError("partial rotational restraint reached rendering")
        for code, store in (("FOR", self.forces), ("MOM", self.moments), ("D", self.anchor_disp), ("ROT", self.anchor_rot)):
            values = store.get(node)
            if values and any(value != 0 for value in values):
                comments.append(f"{code}({','.join(_num(value) for value in values)})")
        return comments

    def add_assumptions(self, temperatures: list[float]) -> None:
        notes = [
            f"Profile {MBF_SI_PROFILE_ID}: MBF version {MBF_VERSION}, OPTIONS SI with the default vertical axis Y; "
            "SWBPIPE global X, Y, Z are written unchanged as the target program's X, Y, Z, with gravity along -Y. "
            "The writer always writes Y vertical: SWBPIPE has one vertical convention (weight along -Y) and the model "
            "document records no vertical axis, so a Z-vertical import, which arrives rotated, is written Y-vertical.",
            "Lengths are written in mm, forces in N, moments in N*m, modulus in N/mm2 (= MPa), temperatures in C; "
            "SI source units scale to these by an exact power of ten, and US-customary units by the exact definitions "
            "listed in MBF-SI-UNITS-NORMALIZED (Design caepipe-mbf-import.md section 5.2).",
            f"Load case mapping: mechanical cases (concentrated force, concentrated moment, weight) -> {LOAD_CASE_LABELS['sustained']!r}; "
            f"thermal and imposed-displacement cases -> {LOAD_CASE_LABELS['expansion']!r}.",
            "Materials are written temperature-independent: identical E and expansion coefficient at "
            + ", ".join(f"{_num(value)} C" for value in temperatures)
            + "; the optional allowable, yield and rupture columns are omitted.",
            ("Every LOADS entry has Pressure 1 = the element's pressure-region pressure in the sustained case, in bar "
             "(1 bar = 1e5 Pa exactly; psi by its exact definition), or 0 outside every region"
             if self.pipe_pressure else "Every LOADS entry has Pressure 1 = 0")
            + " and specific gravity 0 (empty pipe); Temperature 1 = reference "
            "temperature + the element's SWBPIPE thermal change (summed when several primitives act on one element).",
        ]
        if self.piping_code is not None:
            notes.append(f"Piping code {self.piping_code}, chosen by the caller, is written in OPTIONS"
                         + (f"; OPCODE is written {OPCODE_LINE}: axial force in the code stress off and liberal "
                            "allowable off (code-stress settings only), and B31J off, so SIFs and flexibility "
                            "factors come from the code's own appendix and a tee assembles no branch flexibility "
                            "(Analysis; Format of .MBF, OPCODE)" if self.piping_code.startswith("B31") else
                            "; OPCODE is not written (its B31J field applies to ASME B31 codes only)")
                         + ". Every bend carries the user flexibility factor, which replaces the code's value (Bend).")
        if self.node_springs:
            notes.append("Spring supports are written as flexible anchors KA,TRAK(kx,ky,kz),ROTK(0,0,0), stiffness "
                         "in N/mm; a zero component leaves its DOF free and ROTK(0,0,0) leaves rotations free "
                         "(technical reference, Anchor).")
        if self.pipe_pressure:
            notes.append(f"OPPRES is written {OPPRES_LINE}: PD/4t, Bourdon effect off, pressure correction for bends "
                         "off, peak pressure factor 1. SWBPIPE's exact pressure route models neither effect (DEL-04-07 "
                         "D-3).")
        if self.contract is not None:
            notes.append("The pressure contract, region ids, members, terminals with their closure transfer and the "
                         "provenance are not in the MBF; the sidecar's pressure_map carries them for a bound import.")
        if self.weight:
            notes.append(
                f"Weight per length w (global -Y) is written as material weight density rho = |w| / (A * {STANDARD_GRAVITY} m/s^2) "
                "with A the bare metal area pi/4 (OD^2 - ID^2); kgf/m3 equals kg/m3 numerically. Elements without a "
                "weight primitive have density 0."
            )
        if any(not material["has_alpha"] for material in self.materials.values()):
            notes.append("A material without thermal_expansion_coefficient is written with expansion coefficient 0.")
        if self.rotation_converted:
            notes.append("Rotations authored in rad are written in degrees (x 180/pi).")
        if self.bends:
            notes.append(
                "Each curved bend (curved_bend_macro_element) is written as one layout row at the tangent intersection "
                "(TI) of its adjacent straights: 'T<n>,KI,B<radius mm>,<offsets to the TI>' with the user flexibility "
                "factor as the comment 'FF=<k>'; the incoming straight pipe and the bend share that row and its "
                "properties. The target program reports the near and far tangent points as nodes '<n>A' and '<n>B', "
                "which node_map assigns to the SWBPIPE tangent-point nodes; the next row's offsets start at the TI. "
                "The single user factor stands for SWBPIPE's in-plane and out-of-plane bending factor alike."
            )
        if any(bend["sifs"] is not None for bend in self.bends.values()):
            notes.append("A bend's directional SIF pair is written as the comment 'BSIF(<in-plane>,<out-of-plane>)' "
                         "(Format of .MBF, BSIF), from SWBPIPE's in-plane (M_z of the arc's tangent frame) and out-of-plane "
                         "(M_y) user values.")
        notes.append(
            "Model members the SWBPIPE solve does not read (analysis_status, data_boundary, diagnostics, lines, "
            "unresolved_assumptions, constraints, equipment_interfaces, rule_pack_refs) are not exported."
        )
        self.assumptions.extend(notes)


def _list(value: Any, writer: _Writer, entity: str) -> list[Any]:
    if value is None:
        return []
    if not isinstance(value, list):
        writer.invalid(entity, f"{entity} must be a list")
        return []
    return value


def _present(value: Any) -> bool:
    """A value that carries something: not None, an empty string, list or object."""
    return value is not None and value != "" and value != [] and value != {}


def _is_vector(value: Any) -> bool:
    return isinstance(value, Mapping) and all(
        not isinstance(value.get(axis), bool) and isinstance(value.get(axis), (int, float))
        and math.isfinite(value.get(axis)) for axis in "xyz"
    )


def _sub(a: tuple[float, ...], b: tuple[float, ...]) -> tuple[float, float, float]:
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def _dot(a: tuple[float, ...], b: tuple[float, ...]) -> float:
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def _cross(a: tuple[float, ...], b: tuple[float, ...]) -> tuple[float, float, float]:
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def _norm(a: tuple[float, ...]) -> float:
    return math.hypot(a[0], a[1], a[2])  # scaled: no overflow or underflow in the squares


def _unit(a: tuple[float, ...]) -> tuple[float, float, float]:
    length = _norm(a)
    return (a[0] / length, a[1] / length, a[2] / length)


def _scale(value: float, exponent: int) -> float:
    """Scale by an exact power of ten; division keeps down-scaling correctly rounded."""
    if exponent >= 0:
        return value * 10**exponent
    return value / 10 ** (-exponent)


def _num(value: float) -> str:
    if value == 0:
        return "0"
    return format(value, ".10g")


def _sanitize(text: Any, limit: int) -> str:
    cleaned = "".join(ch for ch in str(text) if 32 <= ord(ch) < 127 and ch != ",")
    return cleaned.strip()[:limit].strip()


def _title(text: Any) -> str:
    return _sanitize(text, _TITLE_MAX) or "SWBPIPE model"
