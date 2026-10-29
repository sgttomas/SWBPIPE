"""CAEPIPE MBF model importer (DEL-17-04; Design `caepipe-mbf-import.md`).

`import_caepipe_mbf` reads the declared first-tranche subset of a CAEPIPE MBF
(model batch file) into a SWBPIPE model document of the current model-document
version, with an import report and an id map. It is the inverse of the SI
writer's profile (`si_writer.py`), extended only where SWBPIPE represents an
MBF construct exactly. It reads model input only, never analysis results.

Every section, record, key letter, comment, field or value outside the subset
is refused by name (the import produces no model) or listed in the import loss
report; nothing is silently dropped, defaulted or inferred. Quantities keep
their MBF unit (pressure too: bar or psi). The only conversions are the two exact definitions of Design
§5.3: feet and inches to inches, and kilogram-force to newton. A Z-vertical
file is turned into SWBPIPE's Y-vertical axes by an exact rotation (§5.1).

An import is bound when the writer sidecar's `mbf_sha256` equals the SHA-256 of
the input bytes; only then are the writer's own conventions applied (§1).

From the project root:

    python3 -m core.handoff.caepipe_mbf.importer <in.mbf> --out <dir> [--sidecar <id_map.json>]
        [--material-provenance <declaration.json>] [--pressure-closure <declaration.json>]
        [--shear-model euler_bernoulli|timoshenko_annulus_energy_v1] [--rebind-by-node-number]
        [--source-label <text>]
"""

from __future__ import annotations

import argparse
from dataclasses import dataclass, field
import decimal
from fractions import Fraction
import hashlib
import json
import math
import os
from pathlib import Path
import re
import subprocess
import sys
from typing import Any, Iterable, Mapping

from .si_writer import (
    BEND_SIF_FIELDS,
    INVENTED_CERTIFICATION,
    LOAD_CASE_LABELS,
    MATERIAL_ORIGINS,
    MBF_VERSION,
    SIDECAR_DIGEST_KEY,
    SIDECAR_FORMAT,
    STANDARD_GRAVITY,
    USER_SUPPLIED_CERTIFICATION,
    _BEND_RELATIVE_TOLERANCE,
    _KEYED_MAPS,
    SidecarNotCanonical,
    current_model_schema_version,
    regenerate_caepipe_sidecar,
    sidecar_self_digest,
)


IMPORT_PROFILE_ID = "caepipe-mbf-import-1"
REPORT_KIND = "swbpipe.caepipe_mbf_import_report"
MODEL_KIND = "openpipestress.product_preview.model"
# MBF versions whose field layouts this importer reads (§4.2); adding one is a reviewed edit with its own tests.
ACCEPTED_VERSIONS = (MBF_VERSION,)
# Appendix PD-1 (a retained, bounded profile decision): unbound imports of these versions admit the load
# reference `L=` and FOR/MOM without FLC. An empty tuple restores the base rule (TBD-17-04-007).
PD1_VERSIONS = (MBF_VERSION,)

SUSTAINED_CASE = "case:mbf-sustained"
EXPANSION_CASE = "case:mbf-expansion-t1"
# Desktop blank-document data-boundary strings (apps/desktop/src/services/projectService.ts); §16.1.
_BLANK_BOUNDARY = {
    "public_examples_policy": "blank_user_created_local_document_no_bundled_engineering_values",
    "protected_source_policy": "no_protected_standards_content_inserted",
    "professional_boundary": "human_review_required_no_software_approval_claim",
}
# The analysis_status labels the invented correlation cases carry.
_ANALYSIS_STATUS = {"mechanics": "ready_for_preview_diagnostics",
                    "rule_check": "not_performed_user_rule_inputs_missing",
                    "professional_acceptance": "not_provided"}
DOFS = ("UX", "UY", "UZ", "RX", "RY", "RZ")
_AXES = "XYZ"
# OPTIONS Z (Design §5.1): the file's global (X, Y, Z) becomes SWBPIPE's Y-vertical (x, y, z) = (X, Z, -Y), the
# proper rotation by -90 degrees about X that takes the file's vertical +Z to +y. It permutes axes and changes one
# sign, so it is exact. File axis j goes to SWBPIPE axis _Z_TARGET[j][0] with sign _Z_TARGET[j][1].
_Z_TARGET = {0: (0, 1), 1: (2, -1), 2: (1, 1)}
_FORCE_DIRECTIONS = ("global_x", "global_y", "global_z")
_THERMAL_DIRECTION = "global_x"  # the token the invented correlation cases use; the solve does not read it
# Exact catalog factors to Pa (core/units: MPa = 1e6 Pa; psi = lbf / in^2), used only to derive G in Pa.
_PA_PER_UNIT = {"MPa": 1_000_000.0, "psi": 4.448_221_615_260_5 / 0.000_645_16}
# Pressure (Design §9.4). LOADS Pressure 1 is in bar in SI files and psi in English files (Format of .MBF, LOADS);
# it keeps that unit, which the units catalog reads to Pa by its binary64 factors (bar 1e5; psi PSI_IN_PASCALS).
PRESSURE_UNIT = {True: "bar", False: "psi"}
# A document with pressure takes the exact pressure contract (schemas/model.schema.yaml, PressureContract). An
# unbound import takes the contract's default shear model, Timoshenko with the energy-matched annular shear factor
# (owner D-6), or the caller's `shear_model`; a bound import keeps the sidecar's (Design §9.4).
PRESSURE_CONTRACT = {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "timoshenko_annulus_energy_v1"}
SHEAR_MODELS = ("euler_bernoulli", "timoshenko_annulus_energy_v1")
PRESSURE_BASIS = "internal_differential_zero_external_v1"
CLOSURE_TRANSFERS = ("transfers_to_wall", "separately_supported_or_compensated")
# What a bound or rebound import admits from a sidecar, field by field (Design §15): the rigid support families the
# writer exports and the exact route admits (absent for a restraint), and the direction tokens the solve parses
# (product_physics `parse_direction`), the only restored direction being a thermal primitive's, which it does not read.
RESTORABLE_SUPPORT_FAMILIES = (None, "anchor", "guide", "line_stop", "vertical_support")
# Each direction token's (translation or rotation, axis), so that equivalent tokens compare equal.
_DIRECTION_AXIS = {**{token: ("t", index) for index, axis in enumerate("xyz")
                      for token in (f"global_{axis}", f"GLOBAL_{axis.upper()}", f"U{axis.upper()}", f"U{axis}",
                                    f"u{axis}")},
                   **{token: ("r", index) for index, axis in enumerate("xyz")
                      for token in (f"rotation_{axis}", f"R{axis.upper()}", f"R{axis}", f"r{axis}")}}
RESTORABLE_DIRECTIONS = ("global_x", "GLOBAL_X", "global_y", "GLOBAL_Y", "global_z", "GLOBAL_Z", "rotation_x",
                         "rotation_y", "rotation_z", "UX", "Ux", "ux", "UY", "Uy", "uy", "UZ", "Uz", "uz", "RX", "Rx",
                         "rx", "RY", "Ry", "ry", "RZ", "Rz", "rz")
# OPPRES fields 2 and 3 switch CAEPIPE's Bourdon effect and pressure correction for bends; a blank OPPRES line
# gives "0, 1, 1, 1.00, 0", both on (Format of .MBF, OPPRES). SWBPIPE's exact route models neither (DEL-04-07 D-3).
OPPRES_EFFECTS = {1: "Bourdon effect", 2: "pressure correction for bends"}
OPPRES_DEFAULTS = ("0", "1", "1", "1.00", "0")
# The exact route's straight-run guard (pressure_runtime.rs REPRESENTATION_GUARD = 64 * f64::EPSILON, used by
# collinear_run), and the catalog factors (core/units: value * factor + 0.0) by which it reads node positions in m.
REPRESENTATION_GUARD = 64 * 2.0 ** -52
# The exact route's mitre threshold alpha_tan (curved_bend TANGENCY_TOLERANCE_RAD): a direction change above it,
# measured as its `kink`, 2 atan2(|a - t|, |a + t|) of the unit directions, is a corner (Design §9.4).
TANGENCY_TOLERANCE_RAD = 1.0e-3
# frame_kernel AXIS_TOLERANCE: a vector whose norm (sqrt of the dot product) is at most this cannot be normalized.
AXIS_TOLERANCE = 1.0e-12
_METRES_PER_UNIT = {"mm": 0.001, "in": 0.0254}

# --- MBF format vocabulary (CAEPIPE User's Manual, Import MBF, Format of .MBF; DEL-17-01 CAEPIPE-IMPORT-MBF) ---
SECTION_KEYWORDS = {
    "VERSION": "version", "OPTIONS": "options", "OPCODE": "opcode", "OPTEMP": "optemp", "OPPRES": "oppres",
    "OPMISC": "opmisc", "MATERIAL": "material", "PIPE": "pipe", "LOADS": "loads", "LAYOUT": "layout",
    "SOILS": "refused", "BMATERIALS": "refused", "BSECTIONS": "refused", "BLOADS": "refused", "PUMPS": "refused",
    "COMPRESSORS": "refused", "TURBINES": "refused", "ACHXS": "refused", "FIHTR": "refused",
    "SEISMIC": "zero", "WIND": "zero", "ASCES": "zero", "ASCEW": "zero", "1991W": "zero", "WIND1": "zero",
    "WIND2": "zero", "WIND3": "zero", "WIND4": "zero", "SNOWICE": "zero", "WAVE1": "zero", "WAVE2": "zero",
    "WAVE3": "zero", "WAVE4": "zero",
    "FCURVE": "delegated", "FCYCLE": "delegated", "ALLOWABLES": "delegated", "SPCMS": "delegated",
    "SPLVL": "delegated", "PSDDT": "delegated", "PSDLS": "delegated", "PSDLD": "delegated",
}
HANGER_OPTIONS = tuple(f"HGR{letter}" for letter in "ABCDEFGHIJK")
PIPING_CODE_OPTIONS = (
    "B311", "B311-67", "B311-73", "B311-77", "B311-80", "B313", "B314", "B315", "B318", "B319", "B3112",
    "B3112-PL", "ASME-NM1", "ASME-NM2", "ASME", "ASME-86", "ASME-92", "ASME-2015", "ASME-2017", "ASME-2021",
    "ASME-2023", "ASME-ND-2017", "ASME-ND-2021", "ASME-ND-2023", "BS806", "DNV-ST", "NORWEIGIAN-83",
    "NORWEIGIAN-90", "IGEM", "RCC-M", "RCC-M-2018", "RCC-M-2020", "RCC-M-2022", "SNCT", "SWEDISH",
    "STOOMWEZEN", "Z183", "Z184", "Z662", "EUROPEAN", "EN13941", "ISO14692",
)
# Documented field counts per record for VERSION 15.10.
# OPMISC's documented layout has 10 fields; the layout CAEPIPE's MBF export writes has 13 (observed format
# behaviour, Design §6): fields 1-8 as documented, field 9 not publicly described, field 10 the gravitational
# acceleration and fields 11-13 the gravitational direction as three components.
OPMISC_EXPORT_FIELDS = 13
# The gravitational setting every observed export writes for the default (earth) environment, Y vertical: 386.4
# (in/s^2, in SI files too) and direction (0, -1, 0). Only this setting is read (Design §6, TBD-17-04-005).
EARTH_GRAVITY = (Fraction("386.4"), (Fraction(0), Fraction(-1), Fraction(0)))
# In a Z-vertical file the same setting is read only with its direction along the file's -Z, the vertical the
# option declares (selected interpretation, Design §6; no Z-vertical export has been observed).
EARTH_GRAVITY_DIRECTION_Z = (Fraction(0), Fraction(0), Fraction(-1))
# The documented 10-field layout gives the acceleration in the file's unit (Format of .MBF, OPMISC: in/sec2 or
# m/sec2). It is read only as standard gravity, 9.80665 m/s^2 or 9.80665 / 0.0254 in/s^2, printed correctly rounded
# to its own digits with at least three significant digits (Design §6).
STANDARD_GRAVITY_BY_MODE = {True: Fraction("9.80665"), False: Fraction("9.80665") / Fraction("0.0254")}
GRAVITY_MIN_DIGITS = 3
FIELD_COUNTS = {"OPCODE": 9, "OPTEMP": 5, "OPPRES": 5, "OPMISC": 10, "MATERIAL_FIRST": 13,
                "MATERIAL_TEMPERATURE": 7, "PIPE": 10, "LOADS": 35}
_MATERIAL_FRP_TYPES = ("FR", "F1", "F2")
_MATERIAL_FIRST_PROTECTED = {8: "yield strength", 9: "tensile strength", 10: "slope M", 11: "constant",
                             12: "endurance", 13: "fatigue curve name"}
_MATERIAL_TEMPERATURE_PROTECTED = {5: "allowable stress", 6: "yield stress", 7: "rupture stress"}
_PIPE_ZERO_FIELDS = {4: "corrosion allowance", 5: "mill tolerance", 6: "insulation density",
                     7: "insulation thickness", 8: "lining density", 9: "lining thickness"}
# LOADS fields after Name, T1, P1, SG (positions 5..35).
_LOADS_TAIL = ["T2", "P2", "T3", "P3", "additional weight", "peak temperature", "peak pressure",
               *[f"{kind}{n}" for n in range(4, 11) for kind in ("T", "P")],
               "Wind 1", "Wind 2", "Wind 3", "Wind 4", "Snow", "Ice", "Wave", "Wave 2", "Wave 3", "Wave 4"]
_ROW_KEY_LETTERS = "KMPJXYZBH"
_XYZ = lambda base: {base + axis for axis in _AXES}  # noqa: E731 - axis-suffixed comment names
_DOCUMENTED_REFUSED_COMMENTS = {
    "MBF-IMPORT-SUPPORT-UNSUPPORTED": {
        "CLD", "CNOD", "CS", "E", "FREE", *_XYZ("FREE"), "GRA1", "GRA2", "GRB1", "GRB2", "GRB3", "GRC1", "GRC2",
        "HTYP", "MLV", "NOD", "NOZZLE", "NTHK", "SPIDER", "SR", "US", "VOD", "VTHK", "VS"},
    "MBF-IMPORT-DISPLACEMENT-UNSUPPORTED": {
        "LD", *{f"DIS{n}" for n in range(2, 11)}, *{f"ROT{n}" for n in range(2, 11)}, "DISD", "ROTD", "SETT",
        "STRO", "SEIS", "SEIS2", "SEIS3", "SERO", "SERO2", "SERO3", *{f"WIND{n}" for n in range(1, 5)},
        *{f"WINR{n}" for n in range(1, 5)}},
    "MBF-IMPORT-LOAD-UNSUPPORTED": {"CWGT", "U", "UNIF", "T", "TEMP", "P", "PRES", "SG", "AMB", "HSG", "HPRES",
                                    "FRE", "PH"},
    "MBF-IMPORT-BEND-UNSUPPORTED": {"IN1", "IN2", "BTHK", "BMAT", "WS"},
    "MBF-IMPORT-ROW-KEY-UNSUPPORTED": {
        "WGT", "VWGT", "AWGT", "ES", "TA", "OD1", "OD2", "THK1", "THK2", "CONE", "FLANGE", "BDIA", "GDIA", "BK",
        "BETA", "FFOR", "FTOR", "FRCT", "GAP", "KTIE", "INSF", "LONG", "SHORT", "MAT", "SEC", "LOAD", "LXAX",
        "LYAX", "RLIM", "ROTL", "RK", "L1", "L2", "OFFSET", "RPAD", "THKF", "JCAP", "JMAT", "JSEC", "JLOAD",
        "JTHK", "JR"},
    "MBF-IMPORT-COMMENT-UNSUPPORTED": {
        "SIF", "SIFA", "SIFT", "SIA", "SII", "SIO", "SIT", "SIFB", "IPS", "IBS", "ITS", "IQS", "ISS", "IPC",
        "IBC", "ITC", "IQC", "ISC", "R", "PAD", "CRTCH", "THK", "LEN", "MM", "WTYPE", "TJOINT"},
}
_FORCE_NAMES = {"FOR": None, **{f"FOR{axis}": index for index, axis in enumerate(_AXES)}}
_MOMENT_NAMES = {"MOM": None, **{f"MOM{axis}": index for index, axis in enumerate(_AXES)}}
_DISPLACEMENT_NAMES = {"D": None, "DIS": None, **{f"D{axis}": index for index, axis in enumerate(_AXES)},
                       **{f"DIS{axis}": index for index, axis in enumerate(_AXES)}}
_ROTATION_NAMES = {"ROT": None, **{f"ROT{axis}": index for index, axis in enumerate(_AXES)}}
_FIXD_NAMES = {"FIXD": None, "F": None, **{f"FIXD{axis}": index for index, axis in enumerate(_AXES)},
               **{f"F{axis}": index for index, axis in enumerate(_AXES)}}
_FIXR_NAMES = {"FIXR": None, **{f"FIXR{axis}": index for index, axis in enumerate(_AXES)}}
# Guides, limit stops, skewed restraints and flexible anchors (Design §12, tranche 3; Format of .MBF, comment list).
# A row carries at most one support type, and its data comments belong to it: GGAP to a guide, MU and STIFF to a guide
# or limit stop, DV to a limit stop or skewed restraint. TRAK and ROTK are a flexible anchor's (KA) stiffnesses.
_SUPPORT_TYPES = {"G": "guide", "LS": "stop", "K": "skew"}
# Two spellings MBF files carry (observed format behaviour; Design §12): `STIFF=Rigid` on a guide or limit stop, the
# rigid stiffness the technical reference names as both supports' default (Guide; Limit Stop), and `None` for a blank
# limit stop limit, the technical reference's display of a blank limit, free on that side (Limit Stop).
RIGID_STIFFNESS = "Rigid"
BLANK_LIMIT = "None"
_SUPPORT_DATA = {"GGAP": ("G",), "MU": ("G", "LS"), "STIFF": ("G", "LS"), "DV": ("LS", "K")}
_SUPPORT_NAMES = {"G": "a guide (G)", "LS": "a limit stop (LS)", "K": "a skewed restraint (K)"}
# Support comments whose meaning the public manual leaves open (TBD-17-04-010, Design §20.1).
_TBD_010_COMMENTS = {
    "AXIAL": "AXIAL sets a limit stop along its element's local x, but no public page says which element's axes a "
             "limit stop takes",
    "SHEARY": "SHEARY sets a limit stop along its element's local y, but no public page says which element's axes a "
              "limit stop takes",
    "SHEARZ": "SHEARZ sets a limit stop along its element's local z, but no public page says which element's axes a "
              "limit stop takes",
    "KR": "KR, a rotational stiffness, is in in-lb/deg or N-m/deg on the Format of .MBF page and in inch-lb/radian "
          "on the technical reference's Anchor page; its unit is open"}
_ANCHOR_STIFFNESS = ("TRAK", "ROTK")
SPRING_FAMILY = "spring"
# A stiffness (TRAK, STIFF, K) is in N/mm in SI files and lb/in in English files (Format of .MBF, TRAK, STIFF, K); it
# keeps that unit, which the units catalog reads to N/m by its binary64 factor (core/units: N/mm 1000; lbf/in
# POUND_FORCE_IN_NEWTONS / INCH_IN_METERS).
STIFFNESS_UNIT = {True: "N/mm", False: "lbf/in"}
_N_PER_M_PER_UNIT = {"N/mm": 1000.0, "lbf/in": 4.448_221_615_260_5 / 0.0254}
# The solve reads a document under the checked-JSON profile openpipestress_jcs_ijson_v1 (canonical_json
# MAX_SAFE_INTEGER and `validate_checked_value`): no integer, and no integral float, beyond 2^53 - 1 in magnitude.
MAX_SAFE_INTEGER = 9_007_199_254_740_991
_DOCUMENTED_COMMENTS = {*_FORCE_NAMES, *_MOMENT_NAMES, *_DISPLACEMENT_NAMES, *_ROTATION_NAMES, *_FIXD_NAMES,
                        *_FIXR_NAMES, "FLC", "FF", "BSIF", "LA", "TAG", "LTAG", *_SUPPORT_TYPES, *_SUPPORT_DATA,
                        *_TBD_010_COMMENTS, *_ANCHOR_STIFFNESS,
                        *(name for names in _DOCUMENTED_REFUSED_COMMENTS.values() for name in names)}
_INCH_MARKS = ('"', "\u201d")
_FOOT_SEPARATORS = ("'", "\u2019", "-")
_LENGTH_MARKS = {"\u2019", "\u201d"}

_REAL = re.compile(r"[+-]?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?\Z")
_FEET_ONLY = re.compile(r"([+-]?)(\d+(?:\.\d+)?|\.\d+)\Z")
_FEET_INCH = re.compile(r"([+-]?)(\d+)(['\u2019-])(\d+)(?:-(\d+)/(\d+))?([\"\u201d]?)\Z")
_COMMENT = re.compile(r"([A-Z][A-Z0-9]*)\s*(?:=\s*(.*)|\((.*)\))?\Z", re.S)
_ROW_KEY = re.compile(r"([FTL])(\d+)(\*?)\Z")
_KEYWORD_LIKE = re.compile(r"[A-Z][A-Z0-9]{2,}\Z")


@dataclass(frozen=True)
class MbfImportResult:
    status: str
    model: dict[str, Any] | None
    diagnostics: list[dict[str, Any]] = field(default_factory=list)
    loss_report: list[dict[str, Any]] = field(default_factory=list)
    assumptions: list[str] = field(default_factory=list)
    target_options: dict[str, Any] = field(default_factory=dict)
    id_map: dict[str, Any] = field(default_factory=dict)
    source: dict[str, Any] = field(default_factory=dict)
    summary: dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> dict[str, Any]:
        """The import report; it carries no model values from refused or omitted records."""
        return {
            "document_kind": REPORT_KIND,
            "profile_id": IMPORT_PROFILE_ID,
            "status": self.status,
            "source": dict(self.source),
            "diagnostics": [dict(item) for item in self.diagnostics],
            "loss_report": [dict(item) for item in self.loss_report],
            "assumptions": list(self.assumptions),
            "target_options": json.loads(json.dumps(self.target_options)),
            "summary": dict(self.summary),
        }

    def model_bytes(self) -> bytes | None:
        return None if self.model is None else canonical_bytes(self.model)

    def report_bytes(self) -> bytes:
        return canonical_bytes(self.to_dict())

    def id_map_bytes(self) -> bytes:
        return canonical_bytes(self.id_map)


def canonical_bytes(value: Any) -> bytes:
    return (json.dumps(value, sort_keys=True, indent=2, ensure_ascii=True, allow_nan=False) + "\n").encode("ascii")


def import_caepipe_mbf(
    data: bytes,
    *,
    sidecar: Mapping[str, Any] | bytes | str | None = None,
    material_provenance: Mapping[str, Any] | None = None,
    pressure_closure: Mapping[str, Any] | None = None,
    shear_model: str | None = None,
    rebind_by_node_number: bool = False,
    source_label: str,
) -> MbfImportResult:
    """Import the declared MBF subset into a SWBPIPE model document, or refuse by name.

    `sidecar` is the writer sidecar as parsed JSON, or its JSON text (bytes or str), which the importer parses
    strictly (parse_sidecar_json; Design §15): a repeated member name, a NaN or Infinity, or bytes that are not strict
    UTF-8 are refused by JSON path. A caller passing parsed JSON has resolved its own parse.

    `pressure_closure` is the caller's closure-transfer declaration for the terminals of pressure regions an
    unbound import forms (Design §9.4): {"closure_transfer": "transfers_to_wall" or
    "separately_supported_or_compensated", "provenance": "<basis>", "nodes": {"<CAEPIPE node>": <closure>}}, with
    "nodes" optional. It governs the real ends of regions (free ends, tees, pressure or bore changes); a terminal at a
    straight-to-straight corner takes transfers_to_wall, deduced from the continuous pipe wall, unless "nodes" names
    it. Closure transfer is never inferred from the file: without the declaration, a file with pressure is refused.

    `shear_model` (optional) is the exact contract's shear model for an unbound import with pressure, one of
    SHEAR_MODELS; without it the contract's default timoshenko_annulus_energy_v1 applies. A bound import keeps the
    sidecar's, and a different `shear_model` with it is refused."""

    return _Importer(data, sidecar, material_provenance, rebind_by_node_number, source_label,
                     pressure_closure, shear_model).run()


def documented_orientations(data: bytes) -> dict[tuple[str, str], tuple[float, float, float]] | None:
    """CAEPIPE's documented local y of each straight of the MBF `data`, keyed by its (from, to) CAEPIPE names, as an
    import derives it from the MBF's own positions (Design §10.5); None if the import refuses the file before its
    positions are read. The SI writer records these for straights without an authored y_reference."""
    importer = _Importer(data, None, None, False, "documented orientations", derive_only=True)
    importer.run()
    return importer.derived


# ---------------------------------------------------------------------------
# numbers and lengths
# ---------------------------------------------------------------------------
@dataclass(frozen=True)
class Num:
    """A parsed MBF number: binary64 value, exact rational and the printed text."""
    value: float
    exact: Fraction
    text: str


def half_unit(text: str) -> Fraction:
    """Half a unit in the last printed digit of a decimal (§10.1 conflict bound); 0 for an exact zero."""
    mantissa, _, exponent = text.lstrip("+-").lower().partition("e")
    if Fraction(mantissa or "0") == 0:
        return Fraction(0)
    decimals = len(mantissa.partition(".")[2])
    return Fraction(1, 2) * Fraction(10) ** (int(exponent or 0) - decimals)


def tenth_digit(value: Fraction) -> Fraction:
    """Half a unit in the tenth significant digit of the decimal `value`, 0 for 0: the writer's printing bound. It
    prints ten significant digits correctly rounded (si_writer _num, `.10g`, trailing zeros dropped, so "4000" stands
    for 4000.000000), and a value printed as p lies within this bound of p."""
    if value == 0:
        return Fraction(0)
    size = abs(value)
    exponent = math.floor(math.log10(float(size)))
    while Fraction(10) ** exponent > size:
        exponent -= 1
    while Fraction(10) ** (exponent + 1) <= size:
        exponent += 1
    return Fraction(1, 2) * Fraction(10) ** (exponent - 9)


def parse_english_length(text: str) -> Fraction | None:
    """Exact inches for one documented English length form (§5.4), or None when the form is not documented."""
    match = _FEET_ONLY.match(text)
    if match:
        value = Fraction(match.group(2)) * 12
        return -value if match.group(1) == "-" else value
    match = _FEET_INCH.match(text)
    if not match:
        return None
    sign, feet, _, inches, numerator, denominator, _ = match.groups()
    if int(inches) >= 12:
        return None
    value = Fraction(int(feet) * 12 + int(inches))
    if numerator is not None:
        if int(denominator) == 0 or int(numerator) >= int(denominator):
            return None
        value += Fraction(int(numerator), int(denominator))
    return -value if sign == "-" else value


def english_length_bound(text: str) -> Fraction:
    """Half a unit in the last printed digit of an English length, in inches."""
    match = _FEET_ONLY.match(text)
    if match:
        return half_unit(match.group(2)) * 12
    match = _FEET_INCH.match(text)
    if match and match.group(6):
        return Fraction(1, 2 * int(match.group(6)))
    return Fraction(1, 2)


# CAEPIPE's documented local y axis of a straight element with global Y vertical (Technical Reference, Local
# Coordinate System; DEL-17-01 CAEPIPE-LOCAL-AXES): a non-vertical element's local y lies in the plane of its
# local x and global Y, toward +Y; a vertical element's local y is global -X. The manual gives no angle for
# "vertical"; this profile treats an element as vertical when the horizontal part of its unit direction is at
# most VERTICAL_SINE (selected interpretation, Design §10.5).
VERTICAL_SINE = 1e-6


def local_y_reference(start: Iterable[float], end: Iterable[float], *,
                      z_vertical: bool = False) -> tuple[float, float, float]:
    """The documented local y of a straight from `start` to `end`, in SWBPIPE's Y-vertical axes.

    With `z_vertical`, the file's vertical was Z and the points are already rotated (§5.1): the documented local z
    is then the projected vertical +y and local y = z x x = (x_z, 0, -x_x) / |(x_x, x_z)|, horizontal; a vertical
    element's local y is the file's global Y, which is SWBPIPE's -z."""
    x = _unit(_sub(tuple(end), tuple(start)))
    horizontal = math.hypot(x[0], x[2])
    if horizontal <= VERTICAL_SINE:
        return (0.0, 0.0, -1.0) if z_vertical else (-1.0, 0.0, 0.0)
    if z_vertical:
        return (x[2] / horizontal + 0.0, 0.0, -x[0] / horizontal + 0.0)  # + 0.0 writes no negative zero
    # Y - (Y.x) x, with 1 - x_y^2 written as x_x^2 + x_z^2 so a near-vertical element keeps its precision.
    projected = (-x[1] * x[0], x[0] * x[0] + x[2] * x[2], -x[1] * x[2])
    return tuple(value + 0.0 for value in _unit(projected))  # + 0.0 writes no negative zero


def standard_gravity_as_printed(text: str, standard: Fraction) -> bool:
    """True when `text` is `standard` correctly rounded to its own printed digits, with at least three significant
    digits: |p - g0| <= half a unit in p's last digit."""
    mantissa = text.lstrip("+-").lower().partition("e")[0].replace(".", "").lstrip("0")
    return len(mantissa) >= GRAVITY_MIN_DIGITS and abs(Fraction(text) - standard) <= half_unit(text)


def split_fields(text: str) -> list[str] | None:
    """Comma-separated fields, keeping commas inside parentheses; None when parentheses do not balance."""
    fields, depth, current = [], 0, []
    for char in text:
        if char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth < 0:
                return None
        if char == "," and depth == 0:
            fields.append("".join(current).strip())
            current = []
        else:
            current.append(char)
    if depth != 0:
        return None
    fields.append("".join(current).strip())
    return fields


def _num(value: float) -> str:
    return "0" if value == 0 else format(value, ".10g")


def _unit(vector: Iterable[float]) -> tuple[float, float, float]:
    x, y, z = vector
    length = math.hypot(x, y, z)  # scaled: no overflow or underflow in the squares
    return (x / length, y / length, z / length)


def _unit_exact(vector: Iterable[Fraction]) -> tuple[float, float, float]:
    """The unit vector of an exact non-zero vector: scaled exactly by a power of two so its largest component is
    near 1, then rounded and normalized. A direction component is rounded relative to the vector's length."""
    largest = max(abs(value) for value in vector)
    shift = largest.numerator.bit_length() - largest.denominator.bit_length()
    scale = Fraction(2) ** -shift
    return _unit(tuple(float(value * scale) for value in vector))


def correct_norm3(a: float, b: float, c: float) -> float:
    """RN(sqrt(a^2 + b^2 + c^2)): one rounding of the exact norm, as core/numerics/correct_norm `norm3`."""
    if any(math.isinf(value) for value in (a, b, c)):
        return math.inf
    if any(math.isnan(value) for value in (a, b, c)):
        return math.nan
    square = Fraction(a) ** 2 + Fraction(b) ** 2 + Fraction(c) ** 2
    if square == 0:
        return 0.0
    # Scale by 4^k so the root's integer part r lies in [2^52, 2^53), then round to nearest, ties to even.
    k = (105 - (square.numerator.bit_length() - square.denominator.bit_length())) // 2
    while True:
        scaled = square * Fraction(4) ** k
        root = math.isqrt(scaled.numerator // scaled.denominator)  # floor(sqrt(scaled)), exactly
        if root >= 2 ** 53:
            k -= 1
        elif root < 2 ** 52:
            k += 1
        else:
            break
    if k > 1074:
        # A subnormal result: round once on its own lattice of 2^-1074, not to 53 bits and then again.
        k = 1074
        scaled = square * Fraction(4) ** k
        root = math.isqrt(scaled.numerator // scaled.denominator)
    midpoint = Fraction(root * root + root) + Fraction(1, 4)  # (root + 1/2)^2
    if scaled > midpoint or (scaled == midpoint and root % 2 == 1):
        root += 1
    try:
        return math.ldexp(root, -k)  # exact: root has at most 53 bits, on the lattice of 2^-k
    except OverflowError:
        return math.inf


def runtime_collinear(points: list[tuple[float, float, float]]) -> bool:
    """The exact route's straight-run check (pressure_runtime.rs `collinear_run`), ported operation for operation:
    `points` are the run's nodes in traversal order, in m. Every later node's offset from the first, divided by the
    span, must lie within 64 epsilon of the span direction and advance along it without passing its end."""
    origin, end = points[0], points[-1]
    chord = [end[axis] - origin[axis] for axis in range(3)]
    length = correct_norm3(*chord)
    if not math.isfinite(length) or length <= 0.0:
        return False
    direction = [value / length for value in chord]
    previous = 0.0
    for point in points[1:]:
        delta = [(point[axis] - origin[axis]) / length for axis in range(3)]
        projection = 0.0
        for axis in range(3):
            projection += delta[axis] * direction[axis]
        residual = correct_norm3(*(delta[axis] - projection * direction[axis] for axis in range(3)))
        if (not math.isfinite(projection) or not math.isfinite(residual) or residual > REPRESENTATION_GUARD
                or projection <= previous or projection > 1.0 + REPRESENTATION_GUARD):
            return False
        previous = projection
    return True


def runtime_bore_radius_m(od: float, wall: float, factor: float) -> float:
    """The exact route's pressure bore r_i,r of an imported section (pressure_exact `ExactSectionBases::derive` and
    `SourceAnnulus::from_od_wall`): OD and wall read to m as the catalog does, the reduced wall (t - c_m) - c_a with
    mill tolerance absent (0) and the stated corrosion allowance 0, and r_i = OD/2 - t_r."""
    od_m, wall_m = od * factor + 0.0, wall * factor + 0.0
    reduced = (wall_m - 0.0) - (0.0 * factor + 0.0)
    return od_m * 0.5 - reduced


def runtime_section_valid(od: float, wall: float, factor: float) -> bool:
    """The build's section checks on OD and wall read to m (product_physics `derive_pipe_section`, annulus_geometry
    `validate_wall`, `SourceAnnulus::from_od_wall`): 0 < t, 2t < OD, t <= OD - t and 0 < OD/2 - t < OD/2, compared
    on the binary64 values in m, which a decimal 2t < OD in the file's unit does not settle."""
    od_m, wall_m = od * factor + 0.0, wall * factor + 0.0
    if not (math.isfinite(od_m) and math.isfinite(wall_m)) or od_m <= 0.0 or wall_m <= 0.0 or 2.0 * wall_m >= od_m:
        return False
    if wall_m > od_m or wall_m > od_m - wall_m:
        return False
    outer = od_m * 0.5
    inner = outer - wall_m
    return not (inner <= 0.0 or inner >= outer)


def _dot(a, b) -> float:
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]  # left to right, unfused, as frame_kernel and curved_bend `dot`


def _cross(a, b) -> list[float]:
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def _finite(vector) -> bool:
    return all(math.isfinite(value) for value in vector)


def runtime_frame_axes(start_m, end_m, y_reference) -> list[list[float]] | None:
    """frame_kernel `FrameElement::orientation` on positions in m, ported operation for operation: the local axes
    [x, y, z], or None where the kernel refuses. x = the element axis scaled by 1/|d|, y = the y_reference with its
    x part removed, scaled the same way, z = x x y scaled; each norm must exceed AXIS_TOLERANCE, and then
    `FrameOrientation::new` requires finite unit axes (| |a| - 1 | <= 1e-12), mutually orthogonal (|a.b| <= 1e-12)
    and right-handed (max |x x y - z| <= 1e-12). An element no longer than 1e-12 m fails the first step."""

    def normalize(vector):
        magnitude = math.sqrt(_dot(vector, vector))
        if magnitude <= AXIS_TOLERANCE:
            return None
        inverse = 1.0 / magnitude
        return [vector[0] * inverse, vector[1] * inverse, vector[2] * inverse]

    axis = normalize([end_m[index] - start_m[index] for index in range(3)])
    if axis is None or not _finite(axis) or not _finite(y_reference):
        return None
    x = normalize(axis)
    if x is None:
        return None
    projection = _dot(y_reference, x)
    y = normalize([y_reference[index] - x[index] * projection for index in range(3)])
    if y is None:
        return None
    z = normalize(_cross(x, y))
    if z is None:
        return None
    axes = [x, y, z]
    if not all(_finite(item) for item in axes):
        return None
    if any(abs(math.sqrt(_dot(item, item)) - 1.0) > AXIS_TOLERANCE for item in axes):
        return None
    if abs(_dot(x, y)) > AXIS_TOLERANCE or abs(_dot(x, z)) > AXIS_TOLERANCE or abs(_dot(y, z)) > AXIS_TOLERANCE:
        return None
    right = _cross(x, y)
    if max(abs(right[index] - z[index]) for index in range(3)) > AXIS_TOLERANCE:
        return None
    return axes


def frame_y_reference_usable(start_m, end_m, y_reference) -> bool:
    """Whether the frame kernel forms an orientation from this y_reference on positions in m (runtime_frame_axes)."""
    return runtime_frame_axes(start_m, end_m, y_reference) is not None


_DECIMAL_DIGITS = 90


def _decimal_atan(t: decimal.Decimal) -> decimal.Decimal:
    """atan(t) for 0 <= t <= 1 in the current decimal context: two half-angle reductions, then the Taylor series."""
    doublings = 0
    while t > decimal.Decimal("0.125"):
        t = t / (1 + (1 + t * t).sqrt())
        doublings += 1
    square, term, total, n = t * t, t, t, 1
    while True:
        term = -term * square
        n += 2
        step = term / n
        if step == 0 or abs(step) < abs(total).scaleb(-_DECIMAL_DIGITS - 4):
            break
        total += step
    return total * (2 ** doublings)


def twice_atan2_nonnegative(y: float, x: float) -> float | None:
    """frame_kernel `twice_atan2_nonnegative`: 2 atan2(y, x) for finite y, x >= 0, not both 0, rounded once to
    binary64; +0 for y = 0 and fl(pi) for x = 0. The runtime's p = 128 arctangent is the correctly rounded value
    except within about 2^-120 relative of a rounding boundary (its module documentation); this one evaluates the
    arctangent to 90 significant digits from the exact quotient and rounds it once, so the two agree there too."""
    if not (math.isfinite(y) and math.isfinite(x)) or y < 0.0 or x < 0.0 or (y == 0.0 and x == 0.0):
        return None
    if y == 0.0:
        return 0.0
    if x == 0.0:
        return math.pi
    with decimal.localcontext() as context:
        context.prec, context.Emin, context.Emax = _DECIMAL_DIGITS, -10 ** 6, 10 ** 6
        quotient = decimal.Decimal(y) / decimal.Decimal(x)  # Decimal(float) is exact
        if quotient <= 1:
            angle = _decimal_atan(quotient)
        else:  # atan t = pi/2 - atan(1/t); pi/4 = 4 atan(1/5) - atan(1/239)
            quarter_pi = 4 * _decimal_atan(decimal.Decimal(1) / 5) - _decimal_atan(decimal.Decimal(1) / 239)
            angle = 2 * quarter_pi - _decimal_atan(1 / quotient)
        return float(Fraction(2 * angle))  # Fraction(Decimal) is exact; float(Fraction) rounds once


def runtime_kink(a, t) -> float | None:
    """curved_bend `kink`: the direction change 2 atan2(|a - t|, |a + t|) of the two directions, each divided by its
    norm (refused at or below AXIS_TOLERANCE); None where the runtime refuses."""

    def normalize(vector):
        magnitude = math.sqrt(_dot(vector, vector))
        if magnitude <= AXIS_TOLERANCE:
            return None
        return [vector[0] / magnitude, vector[1] / magnitude, vector[2] / magnitude]

    if not (_finite(a) and _finite(t)):
        return None
    a, t = normalize(a), normalize(t)
    if a is None or t is None:
        return None
    difference = [a[index] - t[index] for index in range(3)]
    total = [a[index] + t[index] for index in range(3)]
    return twice_atan2_nonnegative(math.sqrt(_dot(difference, difference)), math.sqrt(_dot(total, total)))


def runtime_bend_tangents(start_m, end_m, radius_m: float, y_reference) -> tuple[list[float], list[float]] | None:
    """A realized bend's end tangents [t_i, t_j], oriented from its from-node to its to-node, as the runtime builds
    them on positions and radius in m, or None where it does not build the bend or form its tangents. Ported
    operation for operation: product_physics's curved-bend screen (a finite positive chord, R > L/2, the y_reference
    not parallel to the chord within 1e-9), then curved_bend `objective_arc` (|d|^2 and 4R^2 - |d|^2 from exact
    products rounded once, s = L/2R, c = sqrt(4R^2 - L^2)/2R, the included angle 2 atan2(s, c) within [1e-9,
    pi - 1e-9], two Gram-Schmidt passes for the bow normal n, refused at or below 1e-12 |y|), its axes checked by
    `FrameOrientation::new`, and t_i = c d + s n, t_j = c d - s n."""
    chord = [end_m[index] - start_m[index] for index in range(3)]
    y_reference = list(y_reference)
    if not (_finite(chord) and _finite(y_reference) and math.isfinite(radius_m)) or radius_m <= 0.0:
        return None
    # product_physics: the curved-bend screen before the element is formed
    plain = math.sqrt(chord[0] * chord[0] + chord[1] * chord[1] + chord[2] * chord[2])
    if not (math.isfinite(plain) and plain > 0.0) or radius_m <= 0.5 * plain:
        return None
    unit = [chord[0] / plain, chord[1] / plain, chord[2] / plain]
    along = y_reference[0] * unit[0] + y_reference[1] * unit[1] + y_reference[2] * unit[2]
    plane = [y_reference[index] - along * unit[index] for index in range(3)]
    if math.sqrt(plane[0] * plane[0] + plane[1] * plane[1] + plane[2] * plane[2]) <= 1.0e-9:
        return None
    # curved_bend objective_arc
    try:
        length_squared = float(sum(Fraction(value) * Fraction(value) for value in chord))
        four_radius = 4.0 * radius_m
        if not math.isfinite(four_radius):
            return None
        span = float(Fraction(four_radius) * Fraction(radius_m) - sum(Fraction(value) ** 2 for value in chord))
    except OverflowError:
        return None
    chord_length = math.sqrt(length_squared)
    if not (math.isfinite(chord_length) and chord_length > 0.0) or span <= 0.0:
        return None
    diameter = 2.0 * radius_m
    s, c = chord_length / diameter, math.sqrt(span) / diameter
    angle = twice_atan2_nonnegative(s, c)
    if angle is None or not (1.0e-9 <= angle <= math.pi - 1.0e-9):
        return None
    d = [chord[0] / chord_length, chord[1] / chord_length, chord[2] / chord_length]
    reference_magnitude = math.sqrt(_dot(y_reference, y_reference))
    normal = y_reference
    for _ in range(2):
        projection = _dot(normal, d)
        normal = [normal[index] - projection * d[index] for index in range(3)]
    normal_magnitude = math.sqrt(_dot(normal, normal))
    if not normal_magnitude > AXIS_TOLERANCE * reference_magnitude:
        return None
    n = [normal[0] / normal_magnitude, normal[1] / normal_magnitude, normal[2] / normal_magnitude]
    axes = [[-s * d[index] + c * n[index] for index in range(3)], [c * d[index] + s * n[index] for index in range(3)],
            _cross(n, d)]
    if not all(_finite(item) for item in axes) or any(
            abs(math.sqrt(_dot(item, item)) - 1.0) > AXIS_TOLERANCE for item in axes):
        return None
    if (abs(_dot(axes[0], axes[1])) > AXIS_TOLERANCE or abs(_dot(axes[0], axes[2])) > AXIS_TOLERANCE
            or abs(_dot(axes[1], axes[2])) > AXIS_TOLERANCE):
        return None
    right = _cross(axes[0], axes[1])
    if max(abs(right[index] - axes[2][index]) for index in range(3)) > AXIS_TOLERANCE:
        return None
    return axes[1], [c * d[index] - s * n[index] for index in range(3)]


def runtime_blank(text: str) -> bool:
    """Rust `str::trim().is_empty()`: empty or only Unicode White_Space. Python's `str.strip` also removes
    U+001C..U+001F, which Rust keeps, so the set is spelled out."""
    return all(character in _RUST_WHITESPACE for character in text)


_RUST_WHITESPACE = frozenset("\t\n\x0b\x0c\r \x85\xa0\u1680\u2028\u2029\u202f\u205f\u3000"
                             + "".join(chr(code) for code in range(0x2000, 0x200B)))

# ---------------------------------------------------------------------------
# the sidecar (Design §15)
# ---------------------------------------------------------------------------
_SHA256 = re.compile(r"[0-9a-f]{64}\Z")
# The sidecar formats this importer reads (Design §15): 2, and the current writer's 3, which is 2 with its self-digest.
SIDECAR_FORMATS = (2, SIDECAR_FORMAT)
_LOAD_CATEGORIES = ("concentrated_force", "concentrated_moment", "weight", "thermal", "imposed_displacement")
_RIGID_FAMILIES = RESTORABLE_SUPPORT_FAMILIES[1:]
_BAD = object()


def runtime_support_findings(supports: Iterable[tuple[str, str, str | None, list[str]]]) -> list[tuple[str, str]]:
    """What the solve refuses among supports (support_id, node, family, DOFs), in model order, as
    (support_id, reason): product_physics `rigid_linear_support_from_preview` infers an absent family as anchor with
    six DOFs and as guide otherwise, and linear_supports `prepare_rigid_support` requires at least one DOF, admits
    any DOF for an anchor, translations only for a guide or line stop, UZ only for a vertical support, and refuses a
    DOF already restrained at the node (SUPPORT_INPUT_INVALID). A spring (`prepare_spring`) acts on one DOF and
    restrains none; the importer and the writer give it exactly one, the DOF its stiffness names."""
    allowed = {"anchor": DOFS, "guide": DOFS[:3], "line_stop": DOFS[:3], "vertical_support": ("UZ",)}
    restrained: set[tuple[str, str]] = set()
    found = []
    for support_id, node, family, dofs in supports:
        if family == SPRING_FAMILY:
            if len(dofs) != 1:
                found.append((support_id, "a spring acts on exactly one DOF"))
            continue
        family = family if family in _RIGID_FAMILIES else ("anchor" if len(dofs) == len(DOFS) else "guide")
        if not dofs:
            found.append((support_id, "a rigid support restrains no DOF"))
        for dof in dofs:
            if dof not in allowed[family]:
                found.append((support_id, f"{dof} is not a DOF a {family} restrains"))
            elif (node, dof) in restrained:
                found.append((support_id, f"{dof} at its node is already restrained by another support"))
            else:
                restrained.add((node, dof))
    return found


def canonical_sidecar(sidecar: Mapping[str, Any]) -> dict[str, Any]:
    """`sidecar` without its digests (`mbf_sha256`, which decides the route, and the self-digest), in the form the
    regeneration identity compares (Design §14): every number a binary64 (1 and 1.0 are one value), key order
    immaterial, and each load direction as the axis it names through its aliases (`global_y`, `GLOBAL_Y`, `UY`, `uy`:
    UY; `rotation_x`, `rx`: RX)."""
    def walk(value: Any) -> Any:
        if isinstance(value, Mapping):
            return {key: walk(item) for key, item in value.items()}
        if isinstance(value, list):
            return [walk(item) for item in value]
        if type(value) is int and abs(value) <= 2 ** 1023:  # beyond, no binary64 equals it
            return float(value)
        return value

    out = walk({key: value for key, value in sidecar.items() if key not in ("mbf_sha256", SIDECAR_DIGEST_KEY)})
    loads = out.get("load_map")
    for entry in loads.values() if isinstance(loads, dict) else ():
        direction = entry.get("direction") if isinstance(entry, Mapping) else None
        if isinstance(direction, str) and direction in _DIRECTION_AXIS:
            kind, index = _DIRECTION_AXIS[direction]
            entry["direction"] = ("U" if kind == "t" else "R") + "XYZ"[index]
    return out


def summed_entries(id_map: Mapping[str, Any]) -> dict[str, list[str]]:
    """The primitives an import formed from several sidecar entries (REQ-016, reported by name), as
    "<case>/<formed id>" -> the sidecar load_map keys it sums: the identity the regeneration cannot reproduce."""
    return {f"{entry['case']}/{load_id}": list(entry["summed_entries"])
            for load_id, entry in id_map.get("load_map", {}).items() if "summed_entries" in entry}


def sidecar_difference(given: Mapping[str, Any], regenerated: Mapping[str, Any],
                       summed: Mapping[str, Iterable[str]] | None = None) -> tuple[str, str, str] | None:
    """The first JSON path, in sorted key order, at which the canonical forms of `given` and `regenerated` differ,
    with each side's value (JSON text, "absent" where the key is missing); None when they are equal. `summed`
    (summed_entries) maps a primitive's regenerated key to the given keys it sums: only that identity is set aside,
    and each of those entries is compared, field by field, with the regenerated entry."""
    def shown(value: Any) -> str:
        text = "absent" if value is _BAD else json.dumps(value, sort_keys=True)
        return text if len(text) <= 160 else text[:157] + "..."

    def first(a: Any, b: Any, path: str) -> tuple[str, str, str] | None:
        if isinstance(a, dict) and isinstance(b, dict):
            for key in sorted(set(a) | set(b), key=str):
                where = f"{path}[{key}]" if path in _KEYED_MAPS else (f"{path}.{key}" if path else str(key))
                found = first(a.get(key, _BAD), b.get(key, _BAD), where)
                if found is not None:
                    return found
            return None
        if isinstance(a, list) and isinstance(b, list) and len(a) == len(b):
            for index, (left, right) in enumerate(zip(a, b)):
                found = first(left, right, f"{path}[{index}]")
                if found is not None:
                    return found
            return None
        same = type(a) is type(b) and a == b
        return None if same else (path, shown(a), shown(b))

    left, right = canonical_sidecar(given), canonical_sidecar(regenerated)
    loads, again = left.get("load_map"), right.get("load_map")
    for formed, keys in (summed or {}).items():
        if not isinstance(loads, dict) or not isinstance(again, dict):
            break
        keys = list(keys)
        if formed in loads and formed not in keys:  # another entry under the formed id: compared as it stands
            continue
        for key in keys:
            found = first(loads.get(key, _BAD), again.get(formed, _BAD), f"load_map[{key}]")
            if found is not None:
                return found
        merged = [loads.pop(key) for key in keys if key in loads]
        if merged:
            loads[formed] = merged[0]
    return first(left, right, "")


# The target each load category names (si_writer read_load): a node, a pipe, or a support and its DOF.
_TARGET_KIND = {"concentrated_force": "node", "concentrated_moment": "node", "weight": "pipe", "thermal": "pipe",
                "imposed_displacement": "support"}


def primitive_key(entry: Mapping[str, Any]) -> tuple:
    """The file primitive a sidecar `load_map` entry describes: (case, category, target kind, target id, axis). The
    axis reads a direction through its aliases (`global_y`, `GLOBAL_Y`, `UY` and `uy` are one axis): a force's or
    moment's direction, "*" for a legacy one without a direction, a displacement's DOF, and None for a weight or
    thermal load (one per pipe). The writer sums the primitives that share a key into one MBF record."""
    target, category = entry["target"], entry["category"]
    kind = "support" if "support" in target else next(iter(target))
    if category in ("concentrated_force", "concentrated_moment"):
        axis = "*" if entry.get("direction") is None else _DIRECTION_AXIS.get(entry["direction"])
    elif category == "imposed_displacement":
        axis = _DIRECTION_AXIS.get(target.get("dof"))
    else:
        axis = None
    return entry["load_case"], category, kind, target[kind], axis


class _SidecarParser:
    """The strict sidecar parser, run before anything reads the sidecar (Design §15). Every key has an exact JSON
    type (a bool is not a number, null is not a default), an allowed vocabulary, a non-empty value where an id or
    name is required, and finite binary64 range for numbers; unknown keys and wrong shapes are refused by JSON path.
    A sidecar with `sidecar_format` 2 or 3 (the current writer's: 2 with its self-digest `sidecar_sha256`, which
    _Importer.self_digest verifies before this parser runs) must hold every field, each with its effective value
    (never null); one without it is legacy, from main's tranche-1 writer, and holds what that writer recorded
    (some fields absent in its earlier versions) and no pressure_map. `parse` returns the typed copy that the
    importer reads, or None with `errors` [(path, reason)]."""

    def __init__(self) -> None:
        self.errors: list[tuple[str, str]] = []
        self.current = False
        self.format_number: int | None = None  # 2 or 3 once parsed; None for a legacy sidecar

    def fail(self, path: str, reason: str) -> object:
        self.errors.append((path, reason))
        return _BAD

    # -- scalars ------------------------------------------------------
    def text(self, value: Any, path: str) -> Any:
        if type(value) is not str:
            return self.fail(path, "must be a string")
        return value if not runtime_blank(value) else self.fail(path, "must not be empty or only whitespace")

    def free_text(self, value: Any, path: str) -> Any:
        return value if type(value) is str else self.fail(path, "must be a string")

    def number(self, value: Any, path: str, nullable: bool = False) -> Any:
        if value is None and nullable:
            return None
        if type(value) is int:
            try:
                return float(value)
            except OverflowError:
                return self.fail(path, "must be a finite binary64 number")
        if type(value) is float and math.isfinite(value):
            return value
        return self.fail(path, "must be a finite binary64 number" + (" or null" if nullable else ""))

    def count(self, value: Any, path: str) -> Any:
        return value if type(value) is int and 0 <= value <= 2 ** 53 else self.fail(
            path, "must be a non-negative integer no larger than 2^53 (exact in binary64)")

    def flag(self, value: Any, path: str) -> Any:
        return value if type(value) is bool else self.fail(path, "must be true or false")

    def choice(self, allowed: tuple) -> Any:
        def parse(value: Any, path: str) -> Any:
            if (value is None or type(value) is str) and value in allowed:
                return value
            shown = ", ".join("null" if item is None else item for item in allowed)
            return self.fail(path, f"must be one of {shown}")
        return parse

    # -- containers -----------------------------------------------------
    def record(self, value: Any, path: str, required: Mapping[str, Any], optional: Mapping[str, Any] = {}) -> Any:
        if type(value) is not dict:
            return self.fail(path, "must be an object")
        out: dict[str, Any] = {}
        field = lambda key: f"{path}.{key}" if path else str(key)  # noqa: E731
        for key, item in value.items():
            if key not in required and key not in optional:
                self.fail(field(key), "is not a field the writer records")
            else:
                out[key] = (required.get(key) or optional[key])(item, field(key))
        for key in required:
            if key not in value:
                self.fail(field(key), "is missing")
        return out

    def mapping(self, value: Any, path: str, parse: Any) -> Any:
        if type(value) is not dict:
            return self.fail(path, "must be an object")
        out = {}
        for key, item in value.items():
            where = f"{path}[{key}]"
            if type(key) is not str or runtime_blank(key):
                self.fail(where, "an id must be a non-empty string")
                continue
            out[key] = parse(item, where)
        return out

    def items(self, parse: Any, *, length: int | None = None, nonempty: bool = True) -> Any:
        def parse_list(value: Any, path: str) -> Any:
            if type(value) is not list:
                return self.fail(path, "must be a list")
            if (length is not None and len(value) != length) or (nonempty and not value):
                return self.fail(path, f"must hold {length if length is not None else 'at least one'} item(s)")
            return [parse(item, f"{path}[{index}]") for index, item in enumerate(value)]
        return parse_list

    # -- the writer's records ---------------------------------------------
    # Format 2 holds what a bound import restores and the MBF does not carry, every field with its effective value
    # (no null); what the MBF determines (layout order, record names, numbers, bend geometry, sections, CAEPIPE kinds
    # and names) is not repeated. A legacy sidecar requires what main's tranche-1 writer always recorded and allows only
    # its documented omissions: y_reference (an unauthored straight), family, origin and direction (earlier
    # versions), and a null direction or digest.
    def vector(self, value: Any, path: str) -> Any:
        return self.record(value, path, {axis: self.number for axis in "xyz"})

    def bend(self, value: Any, path: str) -> Any:
        return self.record(value, path, {
            "component": self.text, "radius_mm": self.number, "angle_deg": self.number,
            "flexibility_factor": self.number, "tangent_intersection_mm": self.items(self.number, length=3),
            "caepipe_row_node": self.text})

    def element(self, value: Any, path: str) -> Any:
        if self.current:  # a straight records its effective orientation and whether it was authored
            common = {"from": self.text, "to": self.text, "reversed": self.flag}
            if type(value) is dict and "bend" in value:
                return self.record(value, path, {**common, "bend": lambda item, where: self.record(
                    item, where, {"component": self.text})})
            return self.record(value, path, {**common, "y_reference": self.vector, "y_reference_authored": self.flag})
        common = {"from": self.text, "to": self.text, "reversed": self.flag, "layout_order": self.count,
                  "layout_row": self.count, "material": self.text, "section": self.text, "load": self.text}
        if type(value) is dict and "bend" in value:
            return self.record(value, path, {**common, "bend": self.bend})
        return self.record(value, path, common, {"y_reference": self.vector})

    def support(self, value: Any, path: str) -> Any:
        common = {"node": self.text, "restrained": self.items(self.choice(DOFS))}
        if self.current:  # format 3 also records springs (Design §12, tranche 3); format 2 the rigid families only
            families = (*_RIGID_FAMILIES, SPRING_FAMILY) if self.format_number == SIDECAR_FORMAT else _RIGID_FAMILIES
            entry = self.record(value, path, {**common, "family": self.choice(families),
                                              "family_authored": self.flag})
        else:
            entry = self.record(value, path, {**common, "caepipe_kind": self.choice(("anchor", "restraint"))},
                                {"family": self.choice(RESTORABLE_SUPPORT_FAMILIES)})
        if entry is _BAD or any(item is _BAD for item in entry.values()) or "restrained" not in entry:
            return entry
        if len(set(entry["restrained"])) != len(entry["restrained"]):
            return self.fail(f"{path}.restrained", "repeats a DOF")
        if entry.get("family") == "anchor" and len(entry["restrained"]) != len(DOFS):
            return self.fail(f"{path}.family", "an anchor restrains all six DOFs, as the writer exports it")
        if entry.get("family") == SPRING_FAMILY:
            if len(entry["restrained"]) != 1 or entry["restrained"][0] not in DOFS[:3]:
                return self.fail(f"{path}.restrained", "a spring acts on exactly one translation, as the writer "
                                 "exports it (a flexible anchor's TRAK)")
            if entry.get("family_authored") is not True:
                return self.fail(f"{path}.family_authored", "a spring's family is authored: the solve infers none")
        inferred = "anchor" if len(entry["restrained"]) == len(DOFS) else "guide"
        if self.current and entry.get("family_authored") is False and entry.get("family") != inferred:
            return self.fail(f"{path}.family", f"an unauthored family is the solve's inference, {inferred}")
        return entry

    def material(self, value: Any, path: str) -> Any:
        common = {"swbpipe_material": self.text, "weight_density_kgf_per_m3": self.number,
                  "poisson_ratio": self.number, "elastic_modulus_n_per_mm2": self.number,
                  "thermal_expansion_mm_per_mm_per_c": self.number, "temperatures_c": self.items(self.number),
                  "elements": self.items(self.text)}
        if self.current:
            return self.record(value, path, {"swbpipe_material": self.text,
                                             "origin": self.choice((*MATERIAL_ORIGINS, "undeclared"))})
        return self.record(value, path, common, {"origin": self.choice(MATERIAL_ORIGINS)})

    def section(self, value: Any, path: str) -> Any:
        return self.record(value, path, {"outside_diameter_mm": self.number, "wall_thickness_mm": self.number,
                                         "elements": self.items(self.text)})

    def target(self, value: Any, path: str) -> Any:
        if type(value) is dict and set(value) in ({"node"}, {"pipe"}):
            return self.record(value, path, {key: self.text for key in value})
        if type(value) is dict and set(value) == {"support", "dof"}:
            return self.record(value, path, {"support": self.text, "dof": self.choice(DOFS)})
        return self.fail(path, "must be an object naming a node, a pipe, or a support and its DOF")

    def load(self, value: Any, path: str) -> Any:
        if self.current:
            entry = self.record(value, path, {"load_case": self.text, "category": self.choice(_LOAD_CATEGORIES),
                                              "target": self.target, "direction": self.choice(RESTORABLE_DIRECTIONS)})
        else:
            common = {"load_case": self.text, "caepipe_load_case": self.text,
                      "category": self.choice(_LOAD_CATEGORIES), "target": self.target, "mbf": self.text}
            if type(value) is dict and type(value.get("target")) is dict and "pipe" in value["target"]:
                common["caepipe_element"] = self.element
            else:
                common["caepipe_node"] = self.text
            entry = self.record(value, path, common, {"direction": self.choice((None, *RESTORABLE_DIRECTIONS))})
        if entry is _BAD or any(item is _BAD for item in entry.values()) or not {"category", "target"} <= set(entry):
            return entry
        # Each category names one kind of target, and a force or moment a translation or rotation (si_writer
        # read_load); the import matches on both (VER-002 r13).
        category, kind = entry["category"], _TARGET_KIND[entry["category"]]
        if kind not in entry["target"]:
            return self.fail(f"{path}.target", f"must name a {kind}: a {category} load targets a {kind}")
        wanted = {"concentrated_force": "t", "concentrated_moment": "r"}.get(category)
        direction = entry.get("direction")
        if wanted is not None and direction is not None and _DIRECTION_AXIS[direction][0] != wanted:
            return self.fail(f"{path}.direction", f"must be a {'translation' if wanted == 't' else 'rotation'} "
                             f"direction for a {category} load")
        return entry

    def contract(self, value: Any, path: str) -> Any:
        if value == "none":
            return value
        return self.record(value, path, {"version": self.choice((PRESSURE_CONTRACT["version"],)),
                                         "mode": self.choice((PRESSURE_CONTRACT["mode"],))})

    def region(self, value: Any, path: str) -> Any:
        terminal = lambda item, where: self.record(item, where, {  # noqa: E731
            "node_ref": self.text, "closure_transfer": self.choice(CLOSURE_TRANSFERS), "provenance": self.text})
        return self.record(value, path, {"id": self.text, "members": self.items(self.text),
                                         "terminals": self.items(terminal, length=2)})

    def pressure(self, value: Any, path: str) -> Any:
        """Format 2: the effective pressure state of every model: `contract` (the exact contract, or "none"), the
        effective `shear_model` (the route's euler_bernoulli without a contract) and the regions (none without a
        contract). A legacy sidecar has none: main's writer exported no pressure."""
        if not self.current:
            return self.fail(path, "is not a field main's tranche-1 writer records; a sidecar with pressure has "
                             f"sidecar_format {' or '.join(map(str, SIDECAR_FORMATS))}")
        entry = self.record(value, path, {"contract": self.contract, "shear_model": self.choice(SHEAR_MODELS),
                                          "regions": self.items(self.region, nonempty=False)})
        if entry is _BAD or len(entry) != 3 or any(item is _BAD for item in entry.values()):
            return entry
        if entry["contract"] == "none" and (entry["shear_model"] != "euler_bernoulli" or entry["regions"]):
            return self.fail(path, "without a contract the shear model is the route's euler_bernoulli and there "
                             "is no region")
        return entry

    def digest(self, value: Any, path: str) -> Any:
        if (value is None and not self.current) or (type(value) is str and _SHA256.match(value)):
            return value
        return self.fail(path, "must be a lowercase SHA-256 hex digest" + ("" if self.current else " or null"))

    def format(self, value: Any, path: str) -> Any:
        return value if type(value) is int and value in SIDECAR_FORMATS else self.fail(
            path, f"must be {' or '.join(map(str, SIDECAR_FORMATS))}, a format this importer reads (or absent, for a "
            "legacy sidecar)")

    # -- the whole sidecar -------------------------------------------------
    def parse(self, sidecar: Any) -> dict[str, Any] | None:
        maps = {"node_map": lambda value, path: self.mapping(value, path, self.text),
                "element_map": lambda value, path: self.mapping(value, path, self.element),
                "support_map": lambda value, path: self.mapping(value, path, self.support),
                "load_case_map": lambda value, path: self.mapping(value, path, self.text),
                "material_map": lambda value, path: self.mapping(value, path, self.material),
                "section_map": lambda value, path: self.mapping(value, path, self.section),
                "load_map": lambda value, path: self.mapping(value, path, self.load),
                "pressure_map": self.pressure}
        if type(sidecar) is not dict:
            self.fail("sidecar", "must be an object")
            return None
        self.current = "sidecar_format" in sidecar
        self.format_number = sidecar["sidecar_format"] if self.current and type(sidecar["sidecar_format"]) is int \
            else None
        if self.current:  # every map but section_map (the MBF's), the digest and the format; format 3 its self-digest
            current = {name: parse for name, parse in maps.items() if name != "section_map"}
            current.update(mbf_sha256=self.digest, sidecar_format=self.format)
            if type(sidecar["sidecar_format"]) is int and sidecar["sidecar_format"] == SIDECAR_FORMAT:
                current[SIDECAR_DIGEST_KEY] = self.digest
            parsed = self.record(sidecar, "", current)
        else:  # main's run_pilot wrote the seven maps; earlier runs kept the digest in mbf_render.json
            legacy = {name: parse for name, parse in maps.items() if name != "pressure_map"}
            parsed = self.record(sidecar, "", legacy, {"mbf_sha256": self.digest, "pressure_map": self.pressure})
        if self.errors:
            return None
        self.format_number = parsed.get("sidecar_format")
        out = {name: parsed.get(name, {}) for name in maps}
        out["mbf_sha256"] = parsed.get("mbf_sha256")
        # The importer reads the restored pressure state as the contract (with its shear model) and regions, or None.
        state = parsed.get("pressure_map")
        out["pressure_map"] = None if state is None or state["contract"] == "none" else {
            "contract": {**state["contract"], "shear_model": state["shear_model"]}, "regions": state["regions"]}
        # Across entries: each load id is its case's ("<case>/<id>") and unique (DUPLICATE_ID); supports as the
        # solve admits them, together at each node.
        seen: dict[str, str] = {}
        for key, entry in out["load_map"].items():
            prefix = entry["load_case"] + "/"
            suffix = key[len(prefix):] if key.startswith(prefix) else ""
            if runtime_blank(suffix):
                self.fail(f"load_map[{key}]", f"the key must be '{prefix}<load id>' with a non-empty load id")
            elif suffix in seen:
                self.fail(f"load_map[{key}]", f"repeats the primitive load id of {seen[suffix]} (DUPLICATE_ID)")
            else:
                seen[suffix] = key
        for support_id, reason in runtime_support_findings(
                (support_id, entry["node"], entry.get("family"), entry["restrained"])
                for support_id, entry in out["support_map"].items()):
            self.fail(f"support_map[{support_id}]", f"{reason}; the solve refuses it (SUPPORT_INPUT_INVALID)")
        names: dict[str, str] = {}
        for swb_id, name in out["node_map"].items():
            if name in names:
                self.fail(f"node_map[{swb_id}]", f"names CAEPIPE node {name} as {names[name]} does; it must be "
                          "one-to-one")
            names[name] = swb_id
        if not self.current:
            self.legacy_duplicates(out)
        return None if self.errors else out

    def legacy_duplicates(self, out: Mapping[str, Any]) -> None:
        """A legacy primitive repeats what other entries record (main's writer: `caepipe_load_case` the case's label,
        `caepipe_node` the target node's or moved support's CAEPIPE node, `caepipe_element` the pipe's element entry,
        `mbf` the record it wrote); each repetition must be that value (VER-002 r13). The maps it repeats are held
        to the file (completeness, and each element's record names, in the import)."""
        for key, entry in out["load_map"].items():
            where, target = f"load_map[{key}]", entry["target"]
            label = out["load_case_map"].get(entry["load_case"])
            if entry["caepipe_load_case"] != label:
                self.fail(f"{where}.caepipe_load_case", f"must be its case's label in load_case_map ({label!r})")
            element = out["element_map"].get(target["pipe"]) if "pipe" in target else None
            if "pipe" in target:
                if entry["caepipe_element"] != element:
                    self.fail(f"{where}.caepipe_element", f"must be element_map[{target['pipe']}], as the writer "
                              "recorded it")
            else:
                node = (out["node_map"].get(target["node"]) if "node" in target
                        else out["support_map"].get(target["support"], {}).get("node"))
                if entry["caepipe_node"] != node:
                    self.fail(f"{where}.caepipe_node", f"must be the CAEPIPE node of its target ({node!r})")
            mbf = {"concentrated_force": "FOR", "concentrated_moment": "MOM",
                   "imposed_displacement": "D" if str(target.get("dof")).startswith("U") else "ROT"}.get(
                entry["category"])
            if element is not None:
                mbf = (f"LOADS {element.get('load')} Temperature 1" if entry["category"] == "thermal"
                       else f"MATERIAL {element.get('material')} weight density")
            if entry["mbf"] != mbf:
                self.fail(f"{where}.mbf", f"must be the record main's writer wrote for it ({mbf!r})")


def runtime_bores_equal(bores: list[float]) -> bool:
    """The exact route's equal-bore guard (pressure_runtime.rs): every bore within 64 epsilon of max(|first|,
    |bore|) of the first, the region's first member in traversal order."""
    reference = bores[0]
    return all(abs(reference - bore) <= REPRESENTATION_GUARD * max(abs(reference), abs(bore)) for bore in bores[1:])


def _negated(number: "Num") -> "Num":
    """`number` with its sign changed: value, exact rational and printed text (0 stays 0)."""
    text = number.text[1:] if number.text.startswith("-") else "-" + number.text.lstrip("+")
    return Num(0.0 - number.value, -number.exact, text)


def _sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def _norm(a) -> float:
    return math.hypot(a[0], a[1], a[2])


# ---------------------------------------------------------------------------
# the importer
# ---------------------------------------------------------------------------
@dataclass
class _Row:
    line: int
    kind: str
    number: int
    absolute: bool
    keys: dict[str, str]
    comments: list[tuple[int, str]]
    started: bool = False


@dataclass
class _Element:
    line: int
    start: int
    end: int
    material: str
    section: str
    load: str | None
    ti: bool = False
    radius: Num | None = None
    factor: Num | None = None
    sifs: tuple[Num, Num] | None = None  # BSIF(in-plane,out-plane)
    row: int = -1  # its T row's index among the layout's F and T rows (a legacy sidecar's layout_row)


class _Refused(Exception):
    """Stops the import after a refusal that later stages cannot be read past."""


class _Importer:
    def __init__(self, data: Any, sidecar: Any, provenance: Any, rebind: bool, source_label: Any,
                 closure: Any = None, shear_model: Any = None, derive_only: bool = False) -> None:
        self.data = data
        self.derive_only = derive_only
        self.derived: dict[tuple[str, str], tuple[float, float, float]] | None = None
        self.closure = closure
        self.shear_model = shear_model
        self.sidecar_current = False
        self.self_digest_state: str | None = None  # "verified" or "absent" once a given sidecar is read
        self.oppres: tuple[int, list[str], bool] | None = None
        self.pressure_notes: list[str] = []
        self.sidecar = sidecar
        self.provenance = provenance
        self.rebind = rebind
        self.source_label = source_label
        self.diagnostics: list[dict[str, Any]] = []
        self.loss: list[dict[str, Any]] = []
        self.assumptions: list[str] = []
        self.target_options: dict[str, Any] = {}
        self.summary: dict[str, Any] = {"empty_comments": 0, "delegated_sections": {}}
        self.sha = ""
        self.version: str | None = None
        self.si = False
        self.rad = False
        self.z_vertical = False
        self.lines: list[tuple[int, str]] = []
        self.heading: list[tuple[int, str]] = []
        self.sections: dict[str, tuple[int, list[tuple[int, str]]]] = {}
        self.t_ref: Num | None = None
        self.t_ref_line = 0
        self.materials: dict[str, dict[str, Any]] = {}
        self.pipes: dict[str, dict[str, Any]] = {}
        self.loads: dict[str, dict[str, Any]] = {}
        self.notes: list[str] = []
        self.bound = False
        self.rebound = False
        self.maps: dict[str, Any] = {}
        self.node_ids: dict[str, str] = {}
        self.used_conventions: set[tuple[str, str]] = set()
        self.thermal_load_count: Fraction | None = None
        self.unconfirmed: list[str] = []

    # -- reporting -------------------------------------------------------
    def refuse(self, code: str, line: int, location: str, message: str) -> None:
        self.diagnostics.append({"code": code, "severity": "error", "line": line, "location": location,
                                 "message": message})
        self.loss.append({"location": location, "line": line, "category": "unsupported", "code": code,
                          "reason": message})

    def listed(self, category: str, code: str, line: int, location: str, message: str,
               severity: str = "warning") -> None:
        self.diagnostics.append({"code": code, "severity": severity, "line": line, "location": location,
                                 "message": message})
        self.loss.append({"location": location, "line": line, "category": category, "code": code,
                          "reason": message})

    def incomplete(self, map_name: str, in_file: Iterable[Any], in_sidecar: Iterable[Any], what: str) -> None:
        """A byte-bound sidecar was written for exactly this file: each of its entries names something in the file,
        and each such thing has its entry, or the sidecar is refused (Design §15)."""
        in_file, in_sidecar = list(in_file), list(in_sidecar)
        missing = [item for item in in_file if item not in in_sidecar]
        extra = [item for item in in_sidecar if item not in in_file]
        if missing or extra:
            self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", 0, map_name,
                        f"the bound sidecar's {map_name} does not match the file's {what}: "
                        + "; ".join(part for part in (f"no entry for {', '.join(map(str, missing))}" if missing else "",
                                                      f"entries for {', '.join(map(str, extra))}, not in the file"
                                                      if extra else "") if part))

    def loads_line(self) -> int:
        """The LOADS header's line, or 0 for a file without LOADS (a bound pressure state can still name one)."""
        return self.sections["LOADS"][0] if "LOADS" in self.sections else 0

    def has_errors(self) -> bool:
        return any(item["severity"] == "error" for item in self.diagnostics)

    def result(self, model: dict[str, Any] | None, id_map: dict[str, Any]) -> MbfImportResult:
        refused = self.has_errors()
        source = {"mbf_sha256": self.sha, "mbf_version": self.version, "unit_mode": "SI" if self.si else "English",
                  "angles": "rad" if self.rad else "deg", "vertical_axis": "Z" if self.z_vertical else "Y",
                  "line_count": len(self.lines),
                  "source_label": self.source_label,
                  "binding": "bound" if self.bound else ("rebound_unconfirmed" if self.rebound else "unbound"),
                  "sidecar_self_digest": self.self_digest_state}
        return MbfImportResult(
            status="refused" if refused else "imported",
            model=None if refused else model,
            diagnostics=self.diagnostics,
            loss_report=self.loss,
            assumptions=self.assumptions,
            target_options=self.target_options,
            id_map={} if refused else id_map,
            source=source,
            summary=self.summary,
        )

    # -- run -------------------------------------------------------------
    def run(self) -> MbfImportResult:
        if not isinstance(self.data, (bytes, bytearray)):
            raise TypeError("import_caepipe_mbf reads bytes; the importer owns the decoding")
        if not isinstance(self.source_label, str) or not self.source_label.strip():
            raise ValueError("source_label must be a non-empty string")
        self.sha = hashlib.sha256(bytes(self.data)).hexdigest()
        try:
            self.decode()
            self.split_sections()
            self.read_version()
            self.read_options()
            self.bind_sidecar()
            if self.shear_model is not None and not (type(self.shear_model) is str
                                                     and self.shear_model in SHEAR_MODELS):
                self.refuse("MBF-IMPORT-SHEAR-MODEL-INVALID", 0, "shear_model",
                            f"the shear_model argument {self.shear_model!r} is not one of {', '.join(SHEAR_MODELS)}")
            self.read_analysis_options()
            self.read_materials()
            self.read_pipes()
            self.read_loads()
            self.read_other_sections()
            rows = self.read_layout_rows()
            if self.has_errors():
                return self.result(None, {})
            model, id_map = self.build(rows)
        except _Refused:
            return self.result(None, {})
        if self.has_errors():
            return self.result(None, {})
        self.add_assumptions()
        return self.result(model, id_map)

    # -- §4.1 bytes, lines and encoding ----------------------------------
    def decode(self) -> None:
        try:
            text = bytes(self.data).decode("utf-8", errors="strict")
        except UnicodeDecodeError as error:
            self.refuse("MBF-IMPORT-ENCODING-UNSUPPORTED", 0, f"byte {error.start}",
                        f"the MBF is not strict UTF-8: the byte at offset {error.start} cannot be decoded "
                        "(TBD-17-04-006)")
            raise _Refused
        if text.startswith("\ufeff"):
            self.refuse("MBF-IMPORT-ENCODING-UNSUPPORTED", 1, "byte 0",
                        "the MBF starts with a byte-order mark, which no admitted source shows (TBD-17-04-006)")
            raise _Refused
        unified = text.replace("\r\n", "\n")
        if "\r" in unified:
            line = unified[: unified.index("\r")].count("\n") + 1
            self.refuse("MBF-IMPORT-LINE-END-UNSUPPORTED", line, f"line {line}",
                        "a lone carriage return ends a line; only CRLF and LF line ends are read")
            raise _Refused
        parts = unified.split("\n")
        if parts and parts[-1] == "":
            parts.pop()
        self.lines = [(index + 1, part.rstrip(" \t")) for index, part in enumerate(parts)]

    # -- §4.2 heading and sections -----------------------------------------
    def keyword(self, text: str, line: int) -> str | None:
        if text in SECTION_KEYWORDS:
            return text
        if len(text) == 3 and text.isupper() and "," not in text:
            matches = [name for name in SECTION_KEYWORDS if name.startswith(text)]
            if len(matches) == 1:
                return matches[0]
            if len(matches) > 1:
                self.refuse("MBF-IMPORT-SECTION-AMBIGUOUS", line, f"line {line}",
                            f"the three-character keyword {text!r} names {len(matches)} documented sections "
                            f"({', '.join(sorted(matches))}); write the full keyword")
                raise _Refused
        return None

    def split_sections(self) -> None:
        current: str | None = None
        for line, text in self.lines:
            stripped = text.strip()
            name = self.keyword(stripped, line) if stripped else None
            if name is not None:
                if name in self.sections:
                    self.refuse("MBF-IMPORT-SECTION-REPEATED", line, name, f"section {name} appears a second time")
                    raise _Refused
                self.sections[name] = (line, [])
                current = name
                continue
            if current is None:
                self.heading.append((line, text))
                continue
            if (SECTION_KEYWORDS[current] not in ("delegated", "zero") and _KEYWORD_LIKE.match(stripped)
                    and not (current == "LAYOUT" and _ROW_KEY.match(stripped))
                    and not (current == "OPTIONS" and stripped in (*HANGER_OPTIONS, *PIPING_CODE_OPTIONS, "SI", "RAD"))):
                self.refuse("MBF-IMPORT-SECTION-UNKNOWN", line, f"line {line}",
                            f"{stripped!r} is not a documented section keyword and does not parse as a {current} record")
                continue
            free = current == "LAYOUT" and stripped.startswith("*")
            if not free and current != "MATERIAL":
                self.check_characters(line, stripped, allowed=_LENGTH_MARKS if current == "LAYOUT" else set())
            self.sections[current][1].append((line, stripped))

    def check_characters(self, line: int, text: str, allowed: set[str]) -> bool:
        for char in text:
            if ord(char) > 127 and char not in allowed:
                self.refuse("MBF-IMPORT-CHARACTER-UNSUPPORTED", line, f"line {line}",
                            f"character U+{ord(char):04X} outside free text is not read; only ASCII and the "
                            "typographic foot and inch marks in lengths are (TBD-17-04-006)")
                return False
        return True

    def data_lines(self, name: str) -> list[tuple[int, str]]:
        return [(line, text) for line, text in self.sections.get(name, (0, []))[1] if text]

    def fields(self, line: int, text: str, location: str) -> list[str] | None:
        result = split_fields(text)
        if result is None:
            self.refuse("MBF-IMPORT-ROW-SYNTAX-INVALID", line, location, "parentheses do not balance")
        return result

    def too_many(self, fields: list[str], limit: int, line: int, location: str) -> bool:
        if len(fields) > limit:
            self.refuse("MBF-IMPORT-FIELD-UNDOCUMENTED", line, location,
                        f"{location} has {len(fields)} fields; MBF {self.version} documents {limit}, and field "
                        f"{limit + 1} onward has no public description")
            return True
        return False

    def real(self, text: str, line: int, location: str, *, required: bool = True) -> Num | None:
        if text == "":
            if required:
                self.refuse("MBF-IMPORT-FIELD-MISSING", line, location, f"{location} is required and empty")
            return None
        if any(ord(char) > 127 for char in text):
            self.refuse("MBF-IMPORT-CHARACTER-UNSUPPORTED", line, location,
                        f"{location} contains a non-ASCII character; only lengths may carry foot and inch marks")
            return None
        if not _REAL.match(text):
            self.refuse("MBF-IMPORT-NUMBER-INVALID", line, location, f"{location} is not a real number")
            return None
        exact = Fraction(text)
        value = float(text)
        if not math.isfinite(value) or (exact != 0 and value == 0):
            self.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", line, location,
                        f"{location} is outside the binary64 range: it overflows, or a non-zero value underflows to 0")
            return None
        return Num(value, exact, text)

    def binary64(self, value: Fraction | float, line: int, location: str, what: str, *,
                 nonzero: bool = False) -> float:
        """A derived value rounded to binary64; overflow, a non-finite result or a non-zero value that underflows to
        0 is refused by name and stops the import."""
        try:
            result = float(value)
        except OverflowError:
            result = math.inf
        if not math.isfinite(result) or ((value != 0 or nonzero) and result == 0):
            self.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", line, location,
                        f"the derived {what} is outside the binary64 range")
            raise _Refused
        return result

    def is_zero(self, text: str, line: int, location: str) -> bool | None:
        """True for empty or zero, False for a non-zero number, None (refused) for anything else."""
        if text == "":
            return True
        number = self.real(text, line, location)
        return None if number is None else number.exact == 0

    # -- VERSION, OPTIONS ------------------------------------------------
    def read_version(self) -> None:
        if "VERSION" not in self.sections:
            self.refuse("MBF-IMPORT-VERSION-MISSING", 0, "VERSION",
                        "the MBF has no VERSION section; the 6.xx variant without it is not read")
            raise _Refused
        header, _ = self.sections["VERSION"]
        lines = self.data_lines("VERSION")
        if not lines:
            self.refuse("MBF-IMPORT-VERSION-MISSING", header, "VERSION", "the VERSION section gives no version")
            raise _Refused
        if len(lines) > 1:
            self.refuse("MBF-IMPORT-FIELD-UNDOCUMENTED", lines[1][0], "VERSION", "VERSION documents one line")
        line, text = lines[0]
        if text not in ACCEPTED_VERSIONS:
            self.refuse("MBF-IMPORT-VERSION-UNSUPPORTED", line, "VERSION",
                        f"MBF version {text!r} is not read; field layouts depend on the version (TBD-17-01-001) and "
                        f"this importer reads {', '.join(ACCEPTED_VERSIONS)}")
            raise _Refused
        self.version = text

    def read_options(self) -> None:
        options: dict[str, Any] = {"hanger_design": [], "piping_code": []}
        for line, text in self.data_lines("OPTIONS"):
            for token in self.fields(line, text, "OPTIONS") or []:
                if token == "SI":
                    self.si = True
                elif token == "RAD":
                    self.rad = True
                elif token == "Z":
                    self.z_vertical = True
                elif token in HANGER_OPTIONS:
                    options["hanger_design"].append(token)
                elif token in PIPING_CODE_OPTIONS:
                    options["piping_code"].append(token)
                else:
                    self.refuse("MBF-IMPORT-OPTION-UNSUPPORTED", line, "OPTIONS",
                                f"OPTIONS token {token!r} is not a documented option")
        for key in ("hanger_design", "piping_code"):
            if options[key]:
                self.target_options[f"OPTIONS.{key}"] = options[key]
                self.listed("delegated", "MBF-IMPORT-TARGET-OPTION-DELEGATED", self.sections["OPTIONS"][0],
                            f"OPTIONS {key}", f"OPTIONS {key.replace('_', ' ')} is pass-through target configuration "
                            "only; it does not become SWBPIPE rule or code-checking logic")

    # -- sidecar (§15) -----------------------------------------------------
    def bind_sidecar(self) -> None:
        if self.sidecar is None:
            if self.rebind:
                self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, "sidecar", "rebinding was requested without a sidecar")
            return
        if isinstance(self.sidecar, (bytes, bytearray, str)):  # JSON text: parsed strictly, changed in nothing
            try:
                self.sidecar = parse_sidecar_json(self.sidecar)
            except SidecarJsonInvalid as error:
                for path, reason in error.findings:
                    self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, path, f"the sidecar JSON at {path} {reason}")
                return
        if not self.self_digest():  # before anything else reads the sidecar, on every route (Design §15)
            return
        parser = _SidecarParser()  # strict, before anything reads the sidecar (Design §15)
        parsed = parser.parse(self.sidecar)
        if parsed is None:
            for path, reason in parser.errors:
                self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, path, f"the sidecar's {path} {reason}")
            return
        if parser.format_number != SIDECAR_FORMAT:
            self.self_digest_state = "absent"
            self.diagnostics.append({
                "code": "MBF-IMPORT-SIDECAR-NO-SELF-DIGEST", "severity": "info", "line": 0,
                "location": "sidecar_format" if parser.current else "sidecar",
                "message": ("the sidecar is format 2, written before the self-digest" if parser.current else
                            "the sidecar is legacy, from main's tranche-1 writer")
                + ", and carries no self-digest: an accidental or partial edit of it is caught only where the "
                  "agreement rule, the parser or the regeneration identity reads the edited value"})
        digest = parsed.pop("mbf_sha256")
        self.maps = parsed
        self.sidecar_current = parser.current
        if self.rebind:  # the explicit request, whatever the digest: identities by CAEPIPE name, unconfirmed
            self.rebound = True
        elif digest == self.sha:
            self.bound = True
            if not parser.current and (not self.si or self.z_vertical):
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", 0, "sidecar_format",
                            "the sidecar is legacy, from main's tranche-1 writer, which wrote only SI files with Y "
                            "vertical; this file is " + ("Z-vertical" if self.z_vertical else "English"))
        else:
            self.diagnostics.append({"code": "MBF-IMPORT-SIDECAR-NOT-APPLIED", "severity": "info", "line": 0,
                                     "location": "sidecar",
                                     "message": "the sidecar's mbf_sha256 does not equal the input's, so the import "
                                                "is unbound and the sidecar is not applied (no rebinding requested)"})
            self.maps = {}
            return
        self.node_ids = {name: swb_id for swb_id, name in self.maps["node_map"].items()}
        self.ambiguous()

    def self_digest(self) -> bool:
        """Design §15: a sidecar claiming format 3 must carry its self-digest `sidecar_sha256`, the SHA-256 of its
        canonical content without it (si_writer.sidecar_canonical_bytes), and that digest must be the content's, or
        the import is refused, before the parser or any route reads it. It catches an accidental or partial edit; a
        deliberate editor can recompute it, so the parser, the agreement rule and the regeneration identity still
        govern meaning. Format 2 and legacy sidecars carry none: the parser reads them, and bind_sidecar reports it."""
        sidecar = self.sidecar
        if not (type(sidecar) is dict and type(sidecar.get("sidecar_format")) is int
                and sidecar["sidecar_format"] == SIDECAR_FORMAT):
            return True
        path = SIDECAR_DIGEST_KEY
        if path not in sidecar:
            self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, path,
                        f"the sidecar's {path} is missing: a sidecar of format {SIDECAR_FORMAT} carries its "
                        "self-digest, and one without it is not read as an older format")
            return False
        given = sidecar[path]
        if not (type(given) is str and _SHA256.match(given)):
            self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, path,
                        f"the sidecar's {path} must be a lowercase SHA-256 hex digest")
            return False
        try:
            actual = sidecar_self_digest(sidecar)
        except SidecarNotCanonical as error:
            self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, error.path,
                        f"the sidecar's {error.path} {error.reason}, so the sidecar has no canonical form for its "
                        "self-digest")
            return False
        if actual != given:
            self.refuse("MBF-IMPORT-SIDECAR-DIGEST-MISMATCH", 0, path,
                        f"the sidecar's {path} is not the SHA-256 of its canonical content ({actual}): the sidecar "
                        "was changed after it was written, or only part of it was kept. It is not applied on any "
                        "route; restore the written sidecar")
            return False
        self.self_digest_state = "verified"
        return True

    def ambiguous(self) -> None:
        """Refuse, before assembly, a sidecar whose reverse mapping is not a function (VER-002 r13): two elements
        between the same pair of CAEPIPE nodes, or two cases with one label. Several primitives with one key
        (primitive_key) are a many-to-one mapping the MBF sums, reported by name (REQ-016), not refused."""
        def repeated(map_name: str, keyed: Iterable[tuple[str, Any]], what: str) -> None:
            seen: dict[Any, str] = {}
            for name, key in keyed:
                if key in seen:
                    self.refuse("MBF-IMPORT-SIDECAR-AMBIGUOUS", 0, f"{map_name}[{name}]",
                                f"{map_name}[{name}] names {what} that {map_name}[{seen[key]}] names, so the import "
                                "cannot tell which entry is the file's")
                else:
                    seen[key] = name

        repeated("element_map", ((swb_id, frozenset((entry["from"], entry["to"])))
                                 for swb_id, entry in self.maps["element_map"].items()), "the pair of CAEPIPE nodes")
        repeated("load_case_map", self.maps["load_case_map"].items(), "the case label")

    def pd1(self) -> bool:
        return self.version in PD1_VERSIONS

    def convention(self, what: str, line: int, location: str) -> bool:
        """Apply the writer's convention for `what` (TBD-17-04-007) when bound or under PD-1, else refuse."""
        if self.bound:
            self.used_conventions.add(("bound", what))
            return True
        if self.pd1():
            self.used_conventions.add(("PD-1", what))
            return True
        self.refuse("MBF-IMPORT-TBD-17-04-007", line, location,
                    f"{what} rests on TBD-17-04-007 (CQ-17-01-013) and is refused in an unbound import of MBF "
                    f"version {self.version}")
        return False

    # -- §6 analysis options -------------------------------------------------
    def read_analysis_options(self) -> None:
        if "OPTEMP" in self.sections:
            lines = self.data_lines("OPTEMP")
            if len(lines) > 1:
                self.refuse("MBF-IMPORT-FIELD-UNDOCUMENTED", lines[1][0], "OPTEMP", "OPTEMP documents one line")
            if lines:
                line, text = lines[0]
                fields = self.fields(line, text, "OPTEMP") or []
                if not self.too_many(fields, FIELD_COUNTS["OPTEMP"], line, "OPTEMP"):
                    self.t_ref = self.real(fields[0], line, "OPTEMP field 1 (reference temperature)")
                    self.t_ref_line = line
                    if len(fields) > 2 and fields[2] != "":
                        count = self.real(fields[2], line, "OPTEMP field 3 (number of thermal loads)")
                        self.thermal_load_count = count.exact if count is not None else None
                        if count is not None and count.exact != 1:
                            self.refuse("MBF-IMPORT-THERMAL-LOADS-UNSUPPORTED", line, "OPTEMP field 3",
                                        "more than one thermal load is not read in tranche 1")
                    delegated = [value for index, value in enumerate(fields) if index in (1, 3, 4)]
                    self.target_options["OPTEMP"] = {"fields": fields[1:]}
                    if any(delegated):
                        self.listed("delegated", "MBF-IMPORT-TARGET-OPTION-DELEGATED", line, "OPTEMP",
                                    "OPTEMP thermal cycles, solve-thermal and cold-modulus flags are pass-through "
                                    "target configuration")
        for name in ("OPCODE", "OPPRES", "OPMISC"):
            if name not in self.sections:
                continue
            header, entries = self.sections[name]
            if entries and entries[0][1] == "" and name in ("OPPRES", "OPMISC"):
                if name == "OPPRES":
                    self.oppres = (header, list(OPPRES_DEFAULTS), True)
                self.target_options[name] = "section present with CAEPIPE defaults"
                self.listed("delegated", "MBF-IMPORT-TARGET-OPTION-DELEGATED", header, name,
                            f"{name} is present with a blank line (CAEPIPE defaults); pass-through only")
                if self.data_lines(name):
                    self.refuse("MBF-IMPORT-FIELD-UNDOCUMENTED", self.data_lines(name)[0][0], name,
                                f"{name} gives data after the blank defaults line")
                continue
            lines = self.data_lines(name)
            if len(lines) > 1:
                self.refuse("MBF-IMPORT-FIELD-UNDOCUMENTED", lines[1][0], name, f"{name} documents one line")
            if not lines:
                continue
            line, text = lines[0]
            fields = self.fields(line, text, name) or []
            if name == "OPMISC":
                self.read_opmisc(line, fields)
                continue
            if self.too_many(fields, FIELD_COUNTS[name], line, name):
                continue
            if name == "OPMISC" and len(fields) >= 10 and fields[9] != "":
                self.refuse("MBF-IMPORT-TBD-17-04-005", line, "OPMISC field 10 (gravitational direction)",
                            "the gravitational direction field's encoding and default are open (TBD-17-04-005, "
                            "CQ-17-01-011); weight direction is a mechanics input and is not assumed")
                continue
            if name == "OPPRES":
                self.oppres = (line, fields, False)
            self.target_options[name] = {"fields": fields}
            self.listed("delegated", "MBF-IMPORT-TARGET-OPTION-DELEGATED", line, name,
                        f"{name} configures CAEPIPE's own analysis; pass-through target configuration only")

    def read_opmisc(self, line: int, fields: list[str]) -> None:
        """OPMISC (§6): dynamics options pass through; the gravitational setting is read only as the earth default."""
        if len(fields) != OPMISC_EXPORT_FIELDS and self.too_many(fields, FIELD_COUNTS["OPMISC"], line, "OPMISC"):
            return
        self.target_options["OPMISC"] = {"fields": fields[:8]}
        self.listed("delegated", "MBF-IMPORT-TARGET-OPTION-DELEGATED", line, "OPMISC fields 1-8",
                    "OPMISC dynamics and hanger options configure CAEPIPE's own analysis; pass-through only")
        if len(fields) == OPMISC_EXPORT_FIELDS:
            if not self.is_zero(fields[8], line, "OPMISC field 9"):
                self.refuse("MBF-IMPORT-FIELD-UNDOCUMENTED", line, "OPMISC field 9",
                            "OPMISC field 9 has no public description; only the 0 the export writes is read")
                return
            self.listed("delegated", "MBF-IMPORT-TARGET-OPTION-DELEGATED", line, "OPMISC field 9",
                        "OPMISC field 9 (no public description) is read only as 0 and passes through")
            acceleration, direction = fields[9], fields[10:13]
        else:
            if len(fields) == 10 and fields[9] != "":
                self.refuse("MBF-IMPORT-TBD-17-04-005", line, "OPMISC field 10 (gravitational direction)",
                            "a gravitational direction in one field has no documented encoding (TBD-17-04-005, "
                            "CQ-17-01-011); weight direction is a mechanics input and is not assumed")
                return
            acceleration, direction = (fields[8] if len(fields) > 8 else ""), []
        if acceleration == "" and not any(direction):
            return
        g = self.real(acceleration, line, "OPMISC gravitational acceleration")
        components = [self.real(value, line, "OPMISC gravitational direction") for value in direction]
        if g is None or None in components:
            return
        if len(fields) == OPMISC_EXPORT_FIELDS:  # the export's layout: its observed default only
            down = EARTH_GRAVITY_DIRECTION_Z if self.z_vertical else EARTH_GRAVITY[1]
            earth = g.exact == EARTH_GRAVITY[0] and tuple(c.exact for c in components) == down
        else:  # the documented layout: standard gravity in the file's documented unit
            earth = standard_gravity_as_printed(g.text, STANDARD_GRAVITY_BY_MODE[self.si])
        if not earth:
            self.refuse("MBF-IMPORT-GRAVITY-UNSUPPORTED", line, "OPMISC gravitational acceleration and direction",
                        "only earth gravity is read (the export's default setting, or standard gravity in the "
                        "documented layout's unit); another acceleration or direction scales or turns the weights, "
                        "and SWBPIPE has no gravity setting (TBD-17-04-005)")
            return
        self.listed("delegated", "MBF-IMPORT-TARGET-OPTION-DELEGATED", line, "OPMISC gravity",
                    "OPMISC gravitational acceleration and direction are the default earth setting; SWBPIPE has no "
                    "gravity setting and carries weight as explicit primitives along -Y"
                    + (" (the file's -Z, rotated)" if self.z_vertical else ""))
        self.used_conventions.add(("gravity", "the earth gravity setting"))

    # -- §7 MATERIAL -----------------------------------------------------------
    def read_materials(self) -> None:
        order: list[str] = []
        for line, text in self.sections.get("MATERIAL", (0, []))[1]:
            if not text:
                continue
            fields = self.fields(line, text, "MATERIAL")
            if not fields:
                continue
            name = fields[0]
            if name == "":
                self.refuse("MBF-IMPORT-FIELD-MISSING", line, "MATERIAL field 1 (name)", "the material name is empty")
                continue
            if not order or order[-1] != name:
                if name in self.materials:
                    self.refuse("MBF-IMPORT-RECORD-REPEATED", line, f"MATERIAL {name}",
                                f"material {name} is defined a second time")
                    continue
                order.append(name)
                self.material_first(name, fields, line, text)
            else:
                self.material_temperature(name, fields, line)
        for name in order:
            record = self.materials.get(name)
            if record is None or record.get("refused"):
                continue
            rows = record["rows"]
            if not rows:
                self.refuse("MBF-IMPORT-FIELD-MISSING", record["line"], f"MATERIAL {name}",
                            f"material {name} has no temperature line with E and the expansion coefficient")
                record["refused"] = True
            elif any((row[0].value, row[1].value) != (rows[0][0].value, rows[0][1].value) for row in rows):
                self.refuse("MBF-IMPORT-MATERIAL-TEMPERATURE-DEPENDENT", record["line"], f"MATERIAL {name}",
                            f"material {name} varies E or the expansion coefficient with temperature; "
                            "temperature-dependent materials are tranche 2")
                record["refused"] = True

    def material_first(self, name: str, fields: list[str], line: int, text: str) -> None:
        record: dict[str, Any] = {"line": line, "rows": [], "refused": False}
        self.materials[name] = record
        location = f"MATERIAL {name}"
        # Free text: the description (field 7) may hold any character; every other field must be ASCII.
        for index, value in enumerate(fields):
            if index != 6 and not self.check_characters(line, value, set()):
                record["refused"] = True
                return
        if len(fields) >= 6 and fields[5] in _MATERIAL_FRP_TYPES:
            self.refuse("MBF-IMPORT-MATERIAL-FRP-UNSUPPORTED", line, location,
                        f"material {name} has an FRP type, whose line layout is not read")
            record["refused"] = True
            return
        if self.too_many(fields, FIELD_COUNTS["MATERIAL_FIRST"], line, location):
            record["refused"] = True
            return
        fields = fields + [""] * (FIELD_COUNTS["MATERIAL_FIRST"] - len(fields))
        for position, column in _MATERIAL_FIRST_PROTECTED.items():
            if self.protected(fields[position - 1], line, location, column, position):
                record["refused"] = True
                return
        density = self.real(fields[1], line, f"{location} field 2 (weight density)")
        nu = self.real(fields[2], line, f"{location} field 3 (Poisson ratio)")
        if density is not None and density.exact < 0:
            self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, f"{location} field 2",
                        "a weight density must not be negative")
            density = None
        if nu is not None and not 0 < nu.value < 0.5:
            self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, f"{location} field 3",
                        "the Poisson ratio must lie in (0, 0.5)")
            nu = None
        if density is None or nu is None:
            record["refused"] = True
            return
        record.update(density=density, nu=nu, description=fields[6])
        if any(fields[3:6]):
            record["delegated"] = True
            self.listed("delegated", "MBF-IMPORT-TARGET-OPTION-DELEGATED", line, location,
                        f"material {name} joint factors and type are code-check settings; not imported")

    def protected(self, value: str, line: int, location: str, column: str, position: int) -> bool:
        if value == "":
            return False
        if _REAL.match(value) and Fraction(value) == 0:
            return False
        self.refuse("MBF-IMPORT-MATERIAL-PROTECTED-SUSPECTED", line, f"{location} field {position} ({column})",
                    f"{location} carries a value in its {column} column; strength, allowable and fatigue data never "
                    "enter the model and mark the material as suspected library or protected data")
        return True

    def material_temperature(self, name: str, fields: list[str], line: int) -> None:
        record = self.materials[name]
        if record.get("refused"):
            return
        location = f"MATERIAL {name} line {line}"
        if not all(self.check_characters(line, value, set()) for value in fields):
            record["refused"] = True
            return
        if self.too_many(fields, FIELD_COUNTS["MATERIAL_TEMPERATURE"], line, location):
            record["refused"] = True
            return
        fields = fields + [""] * (FIELD_COUNTS["MATERIAL_TEMPERATURE"] - len(fields))
        for position, column in _MATERIAL_TEMPERATURE_PROTECTED.items():
            if self.protected(fields[position - 1], line, location, column, position):
                record["refused"] = True
                return
        temperature = self.real(fields[1], line, f"{location} field 2 (temperature)")
        modulus = self.real(fields[2], line, f"{location} field 3 (E)")
        alpha = self.real(fields[3], line, f"{location} field 4 (expansion coefficient)")
        if modulus is not None and not modulus.value > 0:
            self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, f"{location} field 3", "E must be positive")
            modulus = None
        if temperature is None or modulus is None or alpha is None:
            record["refused"] = True
            return
        record["rows"].append((modulus, alpha, temperature))

    # -- §8 PIPE -------------------------------------------------------------
    def read_pipes(self) -> None:
        for line, text in self.data_lines("PIPE"):
            fields = self.fields(line, text, "PIPE")
            if not fields:
                continue
            name = fields[0]
            location = f"PIPE {name}"
            if name == "":
                self.refuse("MBF-IMPORT-FIELD-MISSING", line, "PIPE field 1 (name)", "the section name is empty")
                continue
            if name in self.pipes:
                self.refuse("MBF-IMPORT-RECORD-REPEATED", line, location, f"section {name} is defined a second time")
                continue
            record: dict[str, Any] = {"line": line, "refused": True}
            self.pipes[name] = record
            if self.too_many(fields, FIELD_COUNTS["PIPE"], line, location):
                continue
            allowance_given = len(fields) > 3 and fields[3] != ""  # field 4 present, for the pressure contract (§9.4)
            fields = fields + [""] * (FIELD_COUNTS["PIPE"] - len(fields))
            od = self.real(fields[1], line, f"{location} field 2 (OD)")
            thk = self.real(fields[2], line, f"{location} field 3 (thickness)")
            ok = od is not None and thk is not None
            if ok and not (0 < thk.value and 2 * thk.exact < od.exact):
                self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, location,
                            f"section {name} needs 0 < thickness < OD/2")
                ok = False
            for position, column in _PIPE_ZERO_FIELDS.items():
                zero = self.is_zero(fields[position - 1], line, f"{location} field {position} ({column})")
                if zero is False:
                    self.refuse("MBF-IMPORT-SECTION-FIELD-UNSUPPORTED", line, f"{location} field {position}",
                                f"section {name} has a non-zero {column}; tranche 1 reads bare pipe only")
                ok = ok and bool(zero)
            if fields[9] != "":
                self.refuse("MBF-IMPORT-SECTION-FIELD-UNSUPPORTED", line, f"{location} field 10",
                            f"section {name} names a soil; buried pipe is tranche 2")
                ok = False
            if ok:
                record.update(refused=False, od=od, thk=thk, allowance_given=allowance_given)

    # -- §9 LOADS ------------------------------------------------------------
    def read_loads(self) -> None:
        for line, text in self.data_lines("LOADS"):
            fields = self.fields(line, text, "LOADS")
            if not fields:
                continue
            name = fields[0]
            location = f"LOADS {name}"
            if name == "":
                self.refuse("MBF-IMPORT-FIELD-MISSING", line, "LOADS field 1 (name)", "the load name is empty")
                continue
            if name in self.loads:
                self.refuse("MBF-IMPORT-RECORD-REPEATED", line, location, f"load {name} is defined a second time")
                continue
            record: dict[str, Any] = {"line": line, "refused": True}
            self.loads[name] = record
            if self.too_many(fields, FIELD_COUNTS["LOADS"], line, location):
                continue
            fields = fields + [""] * (FIELD_COUNTS["LOADS"] - len(fields))
            t1 = self.real(fields[1], line, f"{location} field 2 (temperature 1)")
            p1 = self.real(fields[2], line, f"{location} field 3 (pressure 1)")
            sg = self.real(fields[3], line, f"{location} field 4 (specific gravity)")
            ok = t1 is not None and p1 is not None and sg is not None
            if p1 is not None and p1.exact < 0:
                self.refuse("MBF-IMPORT-PRESSURE-UNSUPPORTED", line, f"{location} field 3",
                            f"load {name} has a negative pressure 1; the manual gives no sign convention for "
                            "external pressure, so it is not read")
                ok = False
            if sg is not None and sg.exact != 0:
                self.refuse("MBF-IMPORT-CONTENTS-UNSUPPORTED", line, f"{location} field 4",
                            f"load {name} has a non-zero specific gravity; contents are tranche 2")
                ok = False
            tail = dict(zip(_LOADS_TAIL, fields[4:]))
            for n in (2, 3, *range(4, 11)):
                if self.thermal_load_count is not None and n > self.thermal_load_count:
                    ok = self.inactive_pair(name, tail[f"T{n}"], tail[f"P{n}"], n, line, location) and ok
                else:
                    ok = self.thermal_pair(name, tail[f"T{n}"], tail[f"P{n}"], n, line, location) and ok
            for label in ("additional weight", "Wind 1", "Wind 2", "Wind 3", "Wind 4", "Snow", "Ice", "Wave",
                          "Wave 2", "Wave 3", "Wave 4"):
                zero = self.is_zero(tail[label], line, f"{location} {label}")
                if zero is False:
                    self.refuse("MBF-IMPORT-LOAD-FIELD-UNSUPPORTED", line, f"{location} {label}",
                                f"load {name} sets {label}; tranche 1 reads it only as zero or empty")
                ok = ok and bool(zero)
            if tail["peak temperature"] or tail["peak pressure"]:
                self.listed("delegated", "MBF-IMPORT-TARGET-OPTION-DELEGATED", line, f"{location} peak values",
                            f"load {name} peak temperature and pressure only feed CAEPIPE's peak case")
            if ok:
                record.update(refused=False, t1=t1, p1=p1)

    def thermal_pair(self, name: str, t_text: str, p_text: str, n: int, line: int, location: str) -> bool:
        p_zero = self.is_zero(p_text, line, f"{location} P{n}")
        if p_zero is None:
            return False
        if t_text == "" and p_zero:
            return True
        temperature = self.real(t_text, line, f"{location} T{n}")
        if temperature is None:
            return False
        if self.t_ref is not None and temperature.exact == self.t_ref.exact and p_zero:
            self.slot_inactive(name, n, line, location, f"T{n} equals the reference temperature and P{n} is 0")
            return True
        self.refuse("MBF-IMPORT-THERMAL-LOADS-UNSUPPORTED", line, f"{location} T{n}",
                    f"load {name} defines thermal load {n}; multiple thermal cases are tranche 2")
        return False

    def inactive_pair(self, name: str, t_text: str, p_text: str, n: int, line: int, location: str) -> bool:
        """A slot beyond OPTEMP's number of thermal loads: no load case uses it; its values must still be numbers."""
        if t_text and self.real(t_text, line, f"{location} T{n}") is None:
            return False
        if p_text and self.real(p_text, line, f"{location} P{n}") is None:
            return False
        if t_text or p_text:
            self.slot_inactive(name, n, line, location, "OPTEMP gives 1 thermal load, so no load case uses it")
        return True

    def slot_inactive(self, name: str, n: int, line: int, location: str, reason: str) -> None:
        self.diagnostics.append({"code": "MBF-IMPORT-LOAD-SLOT-INACTIVE", "severity": "info", "line": line,
                                 "location": f"{location} T{n}/P{n}",
                                 "message": f"load {name}: thermal load {n} (T{n}, P{n}) is inactive and dropped: "
                                            f"{reason}"})

    # -- §14 other sections -------------------------------------------------
    def read_other_sections(self) -> None:
        for name, (header, entries) in self.sections.items():
            kind = SECTION_KEYWORDS[name]
            if kind == "refused":
                self.refuse("MBF-IMPORT-SECTION-UNSUPPORTED", header, name, f"section {name} is not read in tranche 1")
            elif kind == "delegated":
                count = len([text for _, text in entries if text])
                self.summary["delegated_sections"][name] = count
                self.listed("delegated", "MBF-IMPORT-SECTION-DELEGATED", header, name,
                            f"section {name} ({count} records) configures CAEPIPE's own checks or dynamics; its "
                            "values are not copied")
            elif kind == "zero":
                ok = True
                for line, text in entries:
                    if not text:
                        continue
                    for index, value in enumerate(split_fields(text) or ["?"]):
                        if value != "" and not (_REAL.match(value) and Fraction(value) == 0):
                            ok = False
                if ok:
                    self.listed("omitted", "MBF-IMPORT-SECTION-ZERO-OMITTED", header, name,
                                f"section {name} gives only zero load values and is omitted")
                else:
                    self.refuse("MBF-IMPORT-SECTION-UNSUPPORTED", header, name,
                                f"section {name} gives non-zero load values; static equivalents are tranche 2")
        for required in ("MATERIAL", "PIPE", "LAYOUT"):
            if required not in self.sections:
                self.refuse("MBF-IMPORT-SECTION-MISSING", 0, required, f"the MBF has no {required} section")

    # -- §4.4 layout rows -------------------------------------------------------
    def read_layout_rows(self) -> list[_Row | tuple[int, str]]:
        rows: list[_Row | tuple[int, str]] = []
        entries = [(line, text) for line, text in self.sections.get("LAYOUT", (0, []))[1] if text]
        index = 0
        while index < len(entries):
            line, text = entries[index]
            index += 1
            if text.startswith("*"):
                rows.append((line, text))
                continue
            fields = self.fields(line, text, f"LAYOUT line {line}")
            if fields is None:
                continue
            continued = fields[-1] == "" and len(fields) > 1
            if continued:
                fields = fields[:-1]
            row = self.layout_row(line, fields)
            if row is None:
                continue
            if continued and not row.started:
                self.refuse("MBF-IMPORT-ROW-SYNTAX-INVALID", line, f"LAYOUT line {line}",
                            "a row ends with a comma before any comment")
                continue
            while continued:
                if index >= len(entries):
                    self.refuse("MBF-IMPORT-ROW-SYNTAX-INVALID", line, f"LAYOUT line {line}",
                                "the last comment ends with a comma but no continuation line follows")
                    break
                next_line, next_text = entries[index]
                index += 1
                more = self.fields(next_line, next_text, f"LAYOUT line {next_line}") or []
                continued = more[-1:] == [""] and len(more) > 1
                row.comments.extend((next_line, value) for value in (more[:-1] if continued else more))
            rows.append(row)
        return rows

    def layout_row(self, line: int, fields: list[str]) -> _Row | None:
        location = f"LAYOUT line {line}"
        first = fields[0]
        match = _ROW_KEY.match(first)
        if match is None:
            if first[:1] == "H":
                self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", line, location, "row key H (hydrotest) is not read")
            elif first[:1] in "FTL" and "*" in first:
                self.refuse("MBF-IMPORT-TBD-17-04-009", line, location,
                            "the absolute-coordinate marker is read only trailing the node number (TBD-17-04-009, "
                            "CQ-17-01-015); any other placement is refused")
            elif first[:1] in "FTL":
                self.refuse("MBF-IMPORT-NODE-NUMBER-INVALID", line, location,
                            f"node {first[1:]!r} is not a positive integer below 99999; generated suffixes (A, B, C, "
                            "D, J) never appear on input rows")
            else:
                self.refuse("MBF-IMPORT-ROW-SYNTAX-INVALID", line, location,
                            "a layout row starts with F, T or L and a node number")
            return None
        kind, digits, marker = match.groups()
        number = int(digits)
        if not 0 < number < 99999:
            self.refuse("MBF-IMPORT-NODE-NUMBER-INVALID", line, location,
                        "a node number is a positive integer below 99999")
            return None
        if marker and kind != "T":
            self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", line, location,
                        f"the absolute-coordinate marker is read on To rows only, not on a {kind} row")
            return None
        row = _Row(line, kind, number, bool(marker), {}, [])
        started = False
        for position, value in enumerate(fields[1:], start=2):
            if started:
                row.comments.append((line, value))
                continue
            if value == "":
                self.refuse("MBF-IMPORT-ROW-SYNTAX-INVALID", line, f"{location} field {position}",
                            "an empty key-letter field (a trailing comma continues comments only)")
                return None
            letter = value[0]
            if letter == "C":
                started = row.started = True
                row.comments.append((line, value[1:]))
                continue
            if letter not in _ROW_KEY_LETTERS:
                self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", line, f"{location} field {position}",
                            f"key letter {letter!r} is not a documented layout key")
                return None
            if letter in row.keys:
                self.refuse("MBF-IMPORT-ROW-KEY-REPEATED", line, f"{location} field {position}",
                            f"key letter {letter} appears twice in one row")
                return None
            row.keys[letter] = value[1:]
        return row

    # -- §5.4 lengths ------------------------------------------------------------
    def length(self, text: str, line: int, location: str) -> tuple[Fraction, Fraction] | None:
        """(exact value, half-unit bound) in the length unit (in or mm)."""
        if self.si:
            number = self.real(text, line, location)
            return None if number is None else (number.exact, half_unit(text))
        value = parse_english_length(text)
        if value is None:
            self.refuse("MBF-IMPORT-LENGTH-NOTATION-INVALID", line, location,
                        f"{location} is not a documented English length form (feet, or feet-inch with whole inches "
                        "0..11 and an optional fraction)")
            return None
        return value, english_length_bound(text)

    # -- §10-§13 the model -----------------------------------------------------------
    def build(self, rows: list[_Row | tuple[int, str]]) -> tuple[dict[str, Any], dict[str, Any]]:
        self.nodes: dict[int, dict[str, Any]] = {}
        self.node_order: list[int] = []
        self.elements: list[_Element] = []
        self.node_data: dict[int, dict[str, Any]] = {}
        self.run_end_after: set[int] = set()  # element indices after which a run ends
        self.branch_starts: set[int] = set()
        self.location_rows: dict[int, list[int]] = {}
        carried: dict[str, Any] = {"M": None, "P": None, "L": None}
        current: int | None = None
        first_from = True
        run_refused = False
        self.layout_present = "LAYOUT" in self.sections
        layout_index = -1
        for row in rows:
            if isinstance(row, tuple):
                line, text = row
                self.notes.append(f"line {line}: {text[1:].strip()}" if text[1:].strip() else f"line {line}: *")
                continue
            if row.kind == "L":
                if row.number not in self.nodes:
                    self.refuse("MBF-IMPORT-NODE-UNKNOWN", row.line, f"LAYOUT line {row.line}",
                                f"location row L{row.number} names a node no earlier row defines")
                    continue
                if row.keys:
                    self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", row.line, f"LAYOUT line {row.line}",
                                "a location row carries comments only (no offsets, M, P, K, J or B)")
                    continue
                self.location_rows.setdefault(row.number, []).append(row.line)
                self.row_comments(row, row.number, allow_load=False)
                continue
            layout_index += 1
            if self.elements:
                if row.kind == "F":
                    self.run_end_after.add(len(self.elements) - 1)
            self.row_properties(row, carried)
            if row.kind == "F":
                self.from_row(row, first_from)
                first_from = False
                current = row.number if row.number in self.nodes else None
                run_refused = current is None
            else:
                if current is None and run_refused:
                    continue  # the run's From row is already refused
                if current is None:
                    self.refuse("MBF-IMPORT-ROW-SYNTAX-INVALID", row.line, f"LAYOUT line {row.line}",
                                "a To row needs a preceding From row")
                    continue
                if not self.to_row(row, current, carried):
                    current, run_refused = None, True
                    continue
                self.elements[-1].row = layout_index
                current = row.number
            self.row_comments(row, row.number, allow_load=True)
        if self.elements:
            self.run_end_after.add(len(self.elements) - 1)
        if not self.elements and self.layout_present:
            self.refuse("MBF-IMPORT-TOPOLOGY-EMPTY", self.sections["LAYOUT"][0], "LAYOUT", "the layout has no element")
        if self.has_errors():
            raise _Refused
        self.check_supports()
        self.resolve_bends()
        if self.has_errors():
            raise _Refused
        if self.derive_only:  # documented_orientations: the positions are read; nothing is assembled
            self.derived = {(a, b): local_y_reference(start, end, z_vertical=self.z_vertical)
                            for a, b, start, end in self.straight_ends()}
            raise _Refused
        model, id_map = self.assemble()
        self.checked_json_domain(model)
        if self.has_errors():
            raise _Refused
        if self.bound and self.sidecar_current:
            self.regeneration_identity(model, id_map)
        return model, id_map

    def checked_json_domain(self, model: dict[str, Any]) -> None:
        """Design §9.4 guard map: every number of the emitted document lies in the checked-JSON numeric domain the
        solve reads it under (canonical_json `validate_checked_value`, profile openpipestress_jcs_ijson_v1): an
        integer within +-(2^53 - 1) (CHECKED-JSON-UNSAFE-INTEGER), a finite number (CHECKED-JSON-NON-FINITE), and no
        integral float beyond 2^53 - 1 (CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT). The importer writes each number as
        Python's json writes it: a float with a fraction or exponent mark, which the reader parses as f64, and an int
        as an integer. A number outside the domain is refused by name at its JSON path, never changed or dropped."""
        def walk(value: Any, path: str, line: int) -> None:
            if isinstance(value, dict):
                provenance = value.get("provenance")
                found = re.search(r", line (\d+)\Z", provenance) if isinstance(provenance, str) else None
                line = int(found.group(1)) if found else line
                for key, item in value.items():
                    walk(item, f"{path}.{key}" if path else key, line)
            elif isinstance(value, list):
                for index, item in enumerate(value):
                    key = item.get("id") if isinstance(item, dict) and isinstance(item.get("id"), str) else index
                    walk(item, f"{path}[{key}]", line)
            elif isinstance(value, bool) or not isinstance(value, (int, float)):
                return
            elif isinstance(value, int) and abs(value) > MAX_SAFE_INTEGER:
                self.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", line, path,
                            f"{path} is an integer beyond +-(2^53 - 1), which the solve's checked-JSON reader refuses "
                            "(CHECKED-JSON-UNSAFE-INTEGER)")
            elif isinstance(value, float) and not math.isfinite(value):
                self.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", line, path,
                            f"{path} is not finite, which the solve's checked-JSON reader refuses "
                            "(CHECKED-JSON-NON-FINITE)")
            elif isinstance(value, float) and value.is_integer() and abs(value) > MAX_SAFE_INTEGER:
                self.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", line, path,
                            f"{path} is the integral value {value!r} beyond 2^53 - 1, which the solve's checked-JSON "
                            "reader refuses (CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT)")

        walk(model, "", 0)

    def regeneration_identity(self, model: dict[str, Any], id_map: dict[str, Any]) -> None:
        """The exact-digest route's guard of record (Design §14, VER-002 r13): the writer regenerates the sidecar
        from the imported model (no MBF is written), and the format-2 sidecar given must equal it after
        canonicalisation (sidecar_difference); otherwise MBF-IMPORT-SIDECAR-NOT-REPRODUCED names the first differing
        JSON path. The one input beyond the model is the import's record of which straights' y_reference it derived
        (id_map y_reference_derived). Only the identity of a primitive the MBF sums is not compared: each sidecar
        entry this import reported as summed into it is compared, field by field, with its regenerated entry under
        its formed id (summed_entries)."""
        reference = ({} if self.t_ref is None else {"reference_temperature_c": self.t_ref.value} if self.si
                     else {"reference_temperature_f": self.t_ref.value})
        regenerated, refusals = regenerate_caepipe_sidecar(model, mbf_sha256=self.sha,
                                                           derived_y_reference=id_map["y_reference_derived"],
                                                           **reference)
        if regenerated is not None:  # format 2 is format 3 without the self-digest: compared in the given format
            regenerated = {**regenerated, "sidecar_format": self.sidecar["sidecar_format"]}
        if regenerated is None:
            self.refuse("MBF-IMPORT-SIDECAR-NOT-REPRODUCED", 0, "sidecar",
                        "the bound sidecar is not the writer's record of the model it yields: the writer does not "
                        "write the imported model (" + ", ".join(sorted({item["code"] for item in refusals})) + ")")
            return
        found = sidecar_difference(self.sidecar, regenerated, summed=summed_entries(id_map))
        if found is not None:
            path, given, again = found
            self.refuse("MBF-IMPORT-SIDECAR-NOT-REPRODUCED", 0, path,
                        f"the bound sidecar is not the writer's record of the model it yields: at {path} it has "
                        f"{given}, "
                        f"and the writer, regenerating the sidecar from the imported model, records {again}. A "
                        "sidecar whose identities were edited is applied only on request, by CAEPIPE name, every "
                        "restored identity unconfirmed (rebind_by_node_number, Design §15)")

    def straight_ends(self) -> list[tuple[str, str, tuple[float, ...], tuple[float, ...]]]:
        """Each element's straight as (CAEPIPE from name, to name, from position, to position), the positions as the
        import reads them: a bend's tangent nodes nA and nB stand in for its TI (§11)."""
        def end(number: int, near: bool) -> tuple[str, tuple[float, ...]]:
            if number in self.bend_geometry:
                bend = self.bend_geometry[number]
                return (f"{number}A", bend["near"]) if near else (f"{number}B", bend["far"])
            return str(number), self.positions[number]

        found = []
        for element in self.elements:
            (a, start), (b, finish) = end(element.start, False), end(element.end, True)
            found.append((a, b, start, finish))
        return found

    def row_properties(self, row: _Row, carried: dict[str, Any]) -> None:
        location = f"LAYOUT line {row.line}"
        for letter, table, what in (("M", self.materials, "MATERIAL"), ("P", self.pipes, "PIPE")):
            if letter in row.keys:
                name = row.keys[letter]
                if name not in table:
                    self.refuse("MBF-IMPORT-REFERENCE-UNRESOLVED", row.line, f"{location} {letter}",
                                f"{letter}{name} names no {what} record")
                carried[letter] = name
        for comment_line, text in row.comments:
            if text.startswith("L=") or text == "L":
                name = text[2:].strip()
                # A refused L= still names the element's load, so later rows report their own refusals.
                self.convention("the load reference L=", comment_line, f"{location} comment L=")
                if name not in self.loads:
                    self.refuse("MBF-IMPORT-REFERENCE-UNRESOLVED", comment_line, f"{location} comment L=",
                                "L= names no LOADS record")
                carried["L"] = name
        if "J" in row.keys:
            self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", row.line, f"{location} J",
                        f"joint code J{row.keys['J']} is not read in tranche 1")
        if "H" in row.keys:
            self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", row.line, f"{location} H", "hydrotest data are not read")
        code = row.keys.get("K")
        if code is not None and code not in ("A", "I"):
            self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", row.line, f"{location} K",
                        f"node code K{code} is not read in tranche 1 (branch codes set SIFs and flexibility)")
        if code == "I" and row.kind != "T":
            self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", row.line, f"{location} K",
                        "KI (tangent intersection) is valid on To rows only")
        if "B" in row.keys and code != "I":
            self.refuse("MBF-IMPORT-BEND-UNSUPPORTED", row.line, f"{location} B",
                        "a bend radius B is read only with KI")

    def coordinates(self, row: _Row) -> dict[int, tuple[Fraction, Fraction]] | None:
        values: dict[int, tuple[Fraction, Fraction]] = {}
        for index, axis in enumerate(_AXES):
            if axis in row.keys:
                parsed = self.length(row.keys[axis], row.line, f"LAYOUT line {row.line} {axis}")
                if parsed is None:
                    return None
                values[index] = parsed
        return self.to_swb(values)

    def to_swb(self, values: dict[int, Any], *, signed: bool = True) -> dict[int, Any]:
        """Per-axis values keyed by file axis, keyed by SWBPIPE axis: unchanged for Y vertical; for Z vertical
        rotated by _Z_TARGET (§5.1). A signed value is a Num or a (value, bound) pair; a flag only moves."""
        if not self.z_vertical:
            return values
        out: dict[int, Any] = {}
        for axis, value in values.items():
            target, sign = _Z_TARGET[axis]
            if signed and sign < 0:
                value = _negated(value) if isinstance(value, Num) else (-value[0], value[1])
            out[target] = value
        return out

    def file_axis(self, index: int) -> str:
        """The file's name for SWBPIPE axis `index`."""
        return ("XZY" if self.z_vertical else _AXES)[index]

    def from_row(self, row: _Row, first: bool) -> None:
        location = f"LAYOUT line {row.line}"
        values = self.coordinates(row)
        if values is None:
            return
        if row.number in self.nodes:
            self.branch_starts.add(row.number)
            node = self.nodes[row.number]
            for index, (value, bound) in values.items():
                if abs(value - node["exact"][index]) > bound + node["bound"][index]:
                    self.refuse("MBF-IMPORT-FROM-COORDINATES-CONFLICT", row.line, f"{location} {self.file_axis(index)}",
                                f"F{row.number} {self.file_axis(index)} does not equal the node's position within the "
                                "half-unit bounds of the printed digits")
            return
        if values:
            # A missing axis on a From row that gives coordinates is 0: CAEPIPE's MBF export writes only the
            # non-zero absolute coordinates (observed format behaviour, Design §10.1).
            exact = [values.get(index, (Fraction(0), Fraction(0)))[0] for index in range(3)]
            bound = [values.get(index, (Fraction(0), Fraction(0)))[1] for index in range(3)]
        elif first:
            exact, bound = [Fraction(0)] * 3, [Fraction(0)] * 3
        else:
            self.refuse("MBF-IMPORT-NODE-POSITION-MISSING", row.line, location,
                        f"F{row.number} starts a new run and needs its coordinates (only the first From row may omit "
                        "them all)")
            return
        self.add_node(row.number, exact, bound, row.line)

    def add_node(self, number: int, exact: list[Fraction], bound: list[Fraction], line: int) -> None:
        self.nodes[number] = {"exact": exact, "bound": bound, "line": line, "ti": False}
        self.node_order.append(number)

    def to_row(self, row: _Row, start: int, carried: dict[str, Any]) -> bool:
        location = f"LAYOUT line {row.line}"
        if row.number in self.nodes:
            self.refuse("MBF-IMPORT-TOPOLOGY-LOOP", row.line, location,
                        f"T{row.number} names an existing node and would close a loop; loops are tranche 2")
            return False
        values = self.coordinates(row)
        if values is None:
            return False
        origin = self.nodes[start]
        if row.absolute:
            if len(values) != 3:
                self.refuse("MBF-IMPORT-NODE-POSITION-MISSING", row.line, location,
                            "a row with the absolute-coordinate marker needs X, Y and Z")
                return False
            exact = [values[index][0] for index in range(3)]
            bound = [values[index][1] for index in range(3)]
        else:
            exact = [origin["exact"][index] + values.get(index, (Fraction(0), Fraction(0)))[0] for index in range(3)]
            bound = [origin["bound"][index] + values.get(index, (Fraction(0), Fraction(0)))[1] for index in range(3)]
        if exact == origin["exact"]:
            self.refuse("MBF-IMPORT-GEOMETRY-ZERO-LENGTH", row.line, location,
                        f"the element to T{row.number} has zero length; zero-length joint elements are not read")
            return False
        if carried["M"] is None or carried["P"] is None:
            self.refuse("MBF-IMPORT-PROPERTY-MISSING", row.line, location,
                        f"the element to T{row.number} has no material or section in effect")
            return False
        load = carried["L"]
        if load is None and "LOADS" in self.sections:
            self.refuse("MBF-IMPORT-PROPERTY-MISSING", row.line, location,
                        f"the element to T{row.number} has no load reference in effect, and the file has LOADS")
            return False
        self.add_node(row.number, exact, bound, row.line)
        element = _Element(row.line, start, row.number, carried["M"], carried["P"], load)
        if row.keys.get("K") == "I":
            if "B" not in row.keys:
                self.refuse("MBF-IMPORT-BEND-RADIUS-MISSING", row.line, location,
                            "KI needs a bend radius B; the default long radius depends on nominal size")
                return False
            if not self.si and not _REAL.match(row.keys["B"]):
                self.refuse("MBF-IMPORT-LENGTH-NOTATION-INVALID", row.line, f"{location} B",
                            "an English bend radius is a plain decimal in inches")
                return False
            radius = self.real(row.keys["B"], row.line, f"{location} B")
            if radius is None:
                return False
            if not radius.value > 0:
                self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", row.line, f"{location} B", "a bend radius must be positive")
                return False
            element.ti, element.radius = True, radius
            self.nodes[row.number]["ti"] = True
        self.elements.append(element)
        return True

    def node_record(self, number: int) -> dict[str, Any]:
        return self.node_data.setdefault(number, {
            "anchor": [], "la": [], "fixd": {}, "fixr": {}, "forces": {}, "moments": {}, "disp": {}, "rot": {},
            "tag": [], "comment_lines": [], "load_lines": [], "supports": [], "trak": [], "rotk": []})

    def comment_values(self, name: str, args: str | None, value: str | None, axis: int | None, line: int,
                       location: str, *, flag: bool = False) -> dict[int, Num] | None:
        """Per-axis numbers of NAME(a,b,c), NAMEX=v or (for flags) a bare NAMEX."""
        if axis is None:
            if args is None:
                self.refuse("MBF-IMPORT-COMMENT-INVALID", line, location, f"{name} needs three values in parentheses")
                return None
            parts = split_fields(args) or []
            if len(parts) != 3:
                self.refuse("MBF-IMPORT-COMMENT-INVALID", line, location, f"{name} needs exactly three values")
                return None
            out = {}
            for index, part in enumerate(parts):
                number = self.real(part, line, f"{location} value {index + 1}")
                if number is None:
                    return None
                out[index] = number
            return out
        if args is not None:
            self.refuse("MBF-IMPORT-COMMENT-INVALID", line, location, f"{name} takes one value, not a list")
            return None
        if value is None:
            if flag:
                return {axis: Num(1.0, Fraction(1), "1")}
            self.refuse("MBF-IMPORT-COMMENT-INVALID", line, location, f"{name} needs a value")
            return None
        number = self.real(value, line, location)
        return None if number is None else {axis: number}

    def row_comments(self, row: _Row, node: int, *, allow_load: bool) -> None:
        record = self.node_record(node)
        location = f"LAYOUT line {row.line}"
        if row.kind != "L" and row.keys.get("K") == "A":
            record["anchor"].append(row.line)
        flc: tuple[int, Num | None] | None = None
        loads: list[tuple[str, dict[int, Num], int]] = []
        support_data: dict[str, tuple[str | None, str | None, int, str]] = {}
        for line, text in row.comments:
            where = f"{location} comment {text.split('=')[0].split('(')[0] or '?'}"
            if text.startswith("L=") or text == "L":
                if not allow_load:
                    self.refuse("MBF-IMPORT-COMMENT-INVALID", line, where, "L= is not read on a location row")
                continue
            match = _COMMENT.match(text)
            if match is None:
                self.refuse("MBF-IMPORT-COMMENT-INVALID", line, where, "a comment is NAME, NAME=value or NAME(values)")
                continue
            name, value, args = match.groups()
            if name in _DOCUMENTED_COMMENTS and value is not None and value.strip() == "" and args is None:
                self.summary["empty_comments"] += 1  # a documented comment with an empty value carries no data
                continue
            record["comment_lines"].append(line)
            if name in _FORCE_NAMES or name in _MOMENT_NAMES:
                table = _FORCE_NAMES if name in _FORCE_NAMES else _MOMENT_NAMES
                values = self.comment_values(name, args, value, table[name], line, where)
                if values is not None:
                    values = self.to_swb(values)
                    loads.append(("forces" if table is _FORCE_NAMES else "moments", values, line))
            elif name == "FLC":
                number = self.real(value or "", line, where)
                if number is not None and number.exact != 1:
                    self.refuse("MBF-IMPORT-FORCE-CASE-UNSUPPORTED", line, where,
                                "only FLC=1 (sustained) is read in tranche 1")
                    number = None
                flc = (line, number)
            elif name in _DISPLACEMENT_NAMES or name in _ROTATION_NAMES:
                rotation = name in _ROTATION_NAMES
                table = _ROTATION_NAMES if rotation else _DISPLACEMENT_NAMES
                values = self.comment_values(name, args, value, table[name], line, where)
                if values is not None:
                    store = record["rot" if rotation else "disp"]
                    for axis, number in self.to_swb(values).items():
                        if number.exact != 0:
                            store.setdefault(axis, []).append((number, line))
            elif name in _FIXD_NAMES or name in _FIXR_NAMES:
                rotation = name in _FIXR_NAMES
                table = _FIXR_NAMES if rotation else _FIXD_NAMES
                if name == "F" and args is None:
                    self.refuse("MBF-IMPORT-COMMENT-INVALID", line, where, "F needs three values in parentheses")
                    continue
                values = self.comment_values(name, args, value, table[name], line, where, flag=True)
                if values is None:
                    continue
                for axis, number in self.to_swb(values, signed=False).items():
                    if number.exact not in (0, 1):
                        self.refuse("MBF-IMPORT-COMMENT-INVALID", line, where, f"{name} values must be 0 or 1")
                        break
                    if number.exact == 1:
                        record["fixr" if rotation else "fixd"][axis] = line
            elif name == "FF":
                if row.keys.get("K") != "I":
                    self.refuse("MBF-IMPORT-COMMENT-INVALID", line, where, "FF is read on a KI bend row only")
                    continue
                number = self.real(value or "", line, where)
                if number is not None and not number.value > 0:
                    self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, where, "a flexibility factor must be positive")
                    number = None
                if number is not None:
                    self.elements[-1].factor = number
                record["comment_lines"].remove(line)  # FF is bend data, not a node comment
            elif name == "BSIF":  # Format of .MBF, BSIF: a bend's SIFs, in-plane then out-of-plane (§11)
                if row.keys.get("K") != "I":
                    self.refuse("MBF-IMPORT-COMMENT-INVALID", line, where, "BSIF is read on a KI bend row only")
                    continue
                parts = split_fields(args) if args is not None else None
                if parts is None or len(parts) != 2:
                    self.refuse("MBF-IMPORT-COMMENT-INVALID", line, where,
                                "BSIF needs two values in parentheses, in-plane then out-of-plane")
                    continue
                numbers = [self.real(part, line, f"{where} value {index + 1}") for index, part in enumerate(parts)]
                if None in numbers:
                    continue
                if not all(number.value > 0 for number in numbers):
                    self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, where, "a bend SIF must be positive")
                    continue
                self.elements[-1].sifs = (numbers[0], numbers[1])
                record["comment_lines"].remove(line)  # BSIF is bend data, not a node comment
            elif name == "LA":
                number = self.real(value or "", line, where)
                if number is not None and number.exact != 0:
                    self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, where,
                                "an anchor in local axes (LA=1) is not read")
                elif number is not None:
                    record["la"].append(line)
            elif name in _SUPPORT_TYPES or name in _SUPPORT_DATA or name in _TBD_010_COMMENTS:
                if name in support_data:
                    self.refuse("MBF-IMPORT-COMMENT-INVALID", line, where, f"{name} is repeated on one row")
                else:
                    support_data[name] = (value, args, line, where)
            elif name in _ANCHOR_STIFFNESS:
                values = self.comment_values(name, args, value, None, line, where)
                if values is not None:  # magnitudes per axis: they move with their axes, as restraint flags do
                    record[name.lower()].append((self.to_swb(values, signed=False), line))
            elif name == "TAG":
                record["tag"].append(((value or "").strip(), line))
            elif name == "LTAG":
                self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, where, "a non-empty LTAG (spectrum level) is not read")
            else:
                code = next((code for code, names in _DOCUMENTED_REFUSED_COMMENTS.items() if name in names), None)
                if code is None:
                    self.refuse("MBF-IMPORT-COMMENT-UNKNOWN", line, where,
                                f"comment {name} is not in the documented comment list")
                else:
                    self.refuse(code, line, where, f"comment {name} is not read by this profile (Design §12 to §14)")
        for kind, values, line in loads:
            if flc is None:
                if not self.convention(f"a {'force' if kind == 'forces' else 'moment'} without FLC", line,
                                       f"{location} {'FOR' if kind == 'forces' else 'MOM'}"):
                    continue
            elif flc[1] is None:
                continue
            store = record[kind]
            for axis, number in values.items():
                store.setdefault(axis, []).append((number, line))
            record["load_lines"].append(line)
        if flc is not None and not loads:
            self.refuse("MBF-IMPORT-COMMENT-INVALID", flc[0], f"{location} comment FLC", "FLC without FOR or MOM")
        if support_data:
            self.row_supports(record, support_data, location)

    def errors(self) -> int:
        return sum(item["severity"] == "error" for item in self.diagnostics)

    def single_value(self, name: str, entry: tuple[str | None, str | None, int, str]) -> Num | None:
        value, args, line, where = entry
        if args is not None or value is None:
            self.refuse("MBF-IMPORT-COMMENT-INVALID", line, where, f"{name} takes one value, as {name}=value")
            return None
        return self.real(value, line, where)

    def row_supports(self, record: dict[str, Any], data: dict[str, tuple[str | None, str | None, int, str]],
                     location: str) -> None:
        """Design §12, tranche 3: the guide (G), limit stop (LS) or skewed restraint (K) one row carries, with its data
        comments. Each is read only rigid and axis-aligned, or as a spring along one global axis; gaps, friction and
        skewed directions are refused by name."""
        start = self.errors()
        for name in sorted(data.keys() & _TBD_010_COMMENTS):
            self.refuse("MBF-IMPORT-TBD-17-04-010", data[name][2], data[name][3], _TBD_010_COMMENTS[name])
        kinds = [name for name in _SUPPORT_TYPES if name in data]
        if len(kinds) > 1:
            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", data[kinds[1]][2], location,
                        f"{' and '.join(kinds)} on one row: a row carries one guide (G), limit stop (LS) or skewed "
                        "restraint (K), and its other comments are that support's data")
            return
        kind = kinds[0] if kinds else None
        for name, readers in _SUPPORT_DATA.items():
            if name in data and kind not in readers:
                self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", data[name][2], data[name][3],
                            f"{name} is read only as the data of {' or '.join(_SUPPORT_NAMES[r] for r in readers)} "
                            "on its row")
        if kind is None or self.errors() > start:
            return
        value, args, line, where = data[kind]
        stiffness: Num | None = None
        axis: int | None = None
        if "GGAP" in data:
            gap = self.single_value("GGAP", data["GGAP"])
            if gap is not None and gap.exact != 0:
                self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", data["GGAP"][2], data["GGAP"][3],
                            "a guide gap (GGAP) is a clearance the pipe closes before the guide acts, symmetric about "
                            "the guide axis: SWBPIPE has no support member for it; only GGAP=0 is read")
        if "MU" in data:
            mu = self.single_value("MU", data["MU"])
            if mu is not None and mu.exact < 0:
                self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", data["MU"][2], data["MU"][3],
                            "a friction coefficient is not negative")
            elif mu is not None and mu.exact != 0:
                self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", data["MU"][2], data["MU"][3],
                            "a friction coefficient (MU) makes the support nonlinear, its friction opposing motion "
                            "normal to the restraint; only MU=0 is read")
        if "STIFF" in data and data["STIFF"][0] == RIGID_STIFFNESS and data["STIFF"][1] is None:
            pass  # STIFF=Rigid: the documented default, read exactly as a row without STIFF (Design §12)
        elif "STIFF" in data:
            stiffness = self.single_value("STIFF", data["STIFF"])
            if stiffness is not None and not stiffness.exact > 0:
                self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", data["STIFF"][2], data["STIFF"][3],
                            "a stiffness (STIFF) must be positive; without STIFF the support is rigid")
        if "DV" in data:
            dv_value, dv_args, dv_line, dv_where = data["DV"]
            components = self.comment_values("DV", dv_args, dv_value, None, dv_line, dv_where)
            if components is not None:
                along = [index for index, number in self.to_swb(components).items() if number.exact != 0]
                if not along:
                    self.refuse("MBF-IMPORT-COMMENT-INVALID", dv_line, dv_where, "DV(0,0,0) names no direction")
                elif len(along) > 1:
                    self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", dv_line, dv_where,
                                "a direction vector off the global axes is a skewed restraint direction; SWBPIPE's "
                                "supports restrain global DOFs only")
                else:
                    axis = along[0]
        if kind == "G":
            if value is not None or args is not None:
                self.refuse("MBF-IMPORT-COMMENT-INVALID", line, where, "G takes no value")
        elif kind == "LS":
            parts = split_fields(args) if args is not None else None
            if parts is None or len(parts) != 2:
                self.refuse("MBF-IMPORT-COMMENT-INVALID", line, where,
                            "LS needs its two limits, LS(lower,upper)")
            else:
                for label, part in zip(("lower", "upper"), parts):
                    if part in ("", BLANK_LIMIT):
                        self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, f"{where} {label}",
                                    f"a blank {label} limit (empty or None) leaves that side free: a one-way "
                                    "restraint, which is nonlinear; only LS(0,0), stopped both ways at the undeflected "
                                    "position, is read")
                        continue
                    limit = self.real(part, line, f"{where} {label}")
                    if limit is not None and limit.exact != 0:
                        self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, f"{where} {label}",
                                    f"a non-zero {label} limit is a gap the node moves through freely before the stop "
                                    "acts, which is nonlinear; only LS(0,0) is read")
            if "DV" not in data:
                self.refuse("MBF-IMPORT-TBD-17-04-010", line, where,
                            "a limit stop without DV: the Format of .MBF page documents no default direction")
        else:
            stiffness = self.single_value("K", data["K"])
            if stiffness is not None and not stiffness.exact > 0:
                self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, where, "a skewed restraint's stiffness K must be "
                            "positive")
            if "DV" not in data:
                self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, where,
                            "a skewed restraint needs its direction DV (a connected node, CNOD, is not read)")
        if self.errors() == start:
            record["supports"].append({"kind": _SUPPORT_TYPES[kind], "line": line, "axis": axis,
                                       "stiffness": stiffness})

    def check_supports(self) -> None:
        for number, record in self.node_data.items():
            location = f"node {number}"
            if record["la"] and not record["anchor"]:
                self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", record["la"][0], location, "LA is read only with KA")
            self.check_flexible_anchor(number, record)
            for support in record["supports"]:
                if support["kind"] == "guide":
                    support["axis"] = self.guide_axis(number, support["line"])
            for key in ("disp", "rot"):
                if record[key] and not record["anchor"]:
                    line = min(line for items in record[key].values() for _, line in items)
                    self.refuse("MBF-IMPORT-DISPLACEMENT-UNSUPPORTED", line, location,
                                "specified displacements and rotations are read at anchors (KA) only")
            if self.nodes.get(number, {}).get("ti"):
                lines = record["comment_lines"] + record["load_lines"] + record["anchor"] + self.location_rows.get(number, [])
                if lines:
                    self.refuse("MBF-IMPORT-BEND-UNSUPPORTED", min(lines), location,
                                "a location row, support or load at a tangent-intersection node is not read")

    def check_flexible_anchor(self, number: int, record: dict[str, Any]) -> None:
        """Design §12, tranche 3: KA with TRAK(kx,ky,kz) and ROTK(kxx,kyy,kzz) is a flexible anchor, one stiffness per
        global DOF; 0 leaves the DOF free (documented: technical reference, Anchor). Its translational stiffnesses
        become springs; ROTK is read only as all zero, its unit being open (TBD-17-04-010)."""
        location = f"node {number}"
        for name in _ANCHOR_STIFFNESS:
            items = record[name.lower()]
            if len(items) > 1:
                self.refuse("MBF-IMPORT-COMMENT-INVALID", items[1][1], f"{location} {name}",
                            f"{name} is given twice at node {number}")
            if items and not record["anchor"]:
                self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", items[0][1], f"{location} {name}",
                            f"{name} is an anchor's stiffness and is read only with KA")
        if not record["anchor"] or not (record["trak"] or record["rotk"]):
            return
        line = record["anchor"][0]
        if not (record["trak"] and record["rotk"]):
            self.refuse("MBF-IMPORT-TBD-17-04-010", line, location,
                        "a flexible anchor needs both TRAK and ROTK: no public page says whether the DOFs of an absent "
                        "one stay rigid or are free")
            return
        trak, rotk = record["trak"][0][0], record["rotk"][0][0]
        if any(value.exact < 0 for value in (*trak.values(), *rotk.values())):
            self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, location, "an anchor stiffness is not negative")
        elif any(value.exact != 0 for value in rotk.values()):
            self.refuse("MBF-IMPORT-TBD-17-04-010", record["rotk"][0][1], f"{location} ROTK",
                        "a non-zero ROTK: its unit is in-lb/deg or N-m/deg on the Format of .MBF page and inch-lb/radian "
                        "on the technical reference's Anchor page, so it is open; ROTK(0,0,0), free rotations, is read")
        elif all(value.exact == 0 for value in trak.values()):
            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, location,
                        f"the flexible anchor at node {number} has no stiffness in any DOF and restrains nothing")
        if record["fixd"] or record["fixr"] or record["supports"]:
            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, location,
                        "a flexible anchor (KA with TRAK and ROTK) is read alone at its node: no public page says how "
                        "it combines with another restraint there")
        for key in ("disp", "rot"):
            if record[key]:
                moved = min(line for items in record[key].values() for _, line in items)
                self.refuse("MBF-IMPORT-DISPLACEMENT-UNSUPPORTED", moved, location,
                            "a specified displacement at a flexible anchor moves the ground end of its springs; "
                            "SWBPIPE's imposed displacement is a rigid prescribed DOF")

    def guide_axis(self, number: int, line: int) -> int | None:
        """The global axis of the element whose local axes a guide at `number` takes: the preceding element, or with
        none the following one (documented: technical reference, Guide). The guide restrains that element's local y
        and z; it is read only when the element lies exactly along a global axis, whose two lateral translations are
        then exactly those."""
        location = f"node {number} G"
        before = [element for element in self.elements if element.end == number]
        after = [element for element in self.elements if element.start == number]
        if not before and len(after) != 1:
            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, location,
                        f"node {number} has no preceding element and {len(after)} following ones, so the guide's "
                        "element is not determined")
            return None
        element = (before or after)[0]
        start, end = self.nodes[element.start]["exact"], self.nodes[element.end]["exact"]
        along = [index for index in range(3) if end[index] != start[index]]
        if len(along) != 1:
            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, location,
                        f"the guide's element, {element.start} to {element.end}, does not lie along a global axis, so "
                        "its lateral directions are skewed; SWBPIPE's supports restrain global DOFs only")
            return None
        return along[0]

    # -- §11 bends ---------------------------------------------------------------
    def resolve_bends(self) -> None:
        self.positions = {number: tuple(self.binary64(value, node["line"], f"node {number}", "node coordinate")
                                        for value in node["exact"]) for number, node in self.nodes.items()}
        for element in self.elements:  # distinct exact positions may round to one binary64 point
            length = _norm(_sub(self.positions[element.end], self.positions[element.start]))
            if not (0 < length < math.inf):
                self.refuse("MBF-IMPORT-GEOMETRY-ZERO-LENGTH" if length == 0 else "MBF-IMPORT-VALUE-UNREPRESENTABLE",
                            element.line, f"LAYOUT line {element.line}",
                            f"the element to node {element.end} has a length of zero or beyond binary64 once its end "
                            "positions are rounded")
                raise _Refused
        self.bend_geometry: dict[int, dict[str, Any]] = {}
        for index, element in enumerate(self.elements):
            if not element.ti:
                continue
            location = f"LAYOUT line {element.line}"
            n = element.end
            outgoing = self.elements[index + 1] if index + 1 < len(self.elements) else None
            if index in self.run_end_after or outgoing is None or outgoing.start != n:
                self.refuse("MBF-IMPORT-BEND-UNSUPPORTED", element.line, location,
                            f"the run ends at tangent intersection {n}; a bend needs an outgoing leg")
                continue
            if n in self.branch_starts:
                self.refuse("MBF-IMPORT-BEND-UNSUPPORTED", element.line, location,
                            f"a branch starts at tangent intersection {n}")
                continue
            p, ti, q = self.positions[element.start], self.positions[n], self.positions[outgoing.end]
            d_in, d_out = _unit(_sub(ti, p)), _unit(_sub(q, ti))
            cosine = max(-1.0, min(1.0, sum(a * b for a, b in zip(d_in, d_out))))
            theta = math.acos(cosine)
            if theta <= 1e-9 or theta >= math.pi - 1e-9:
                self.refuse("MBF-IMPORT-BEND-GEOMETRY-INVALID", element.line, location,
                            f"the legs at tangent intersection {n} are parallel or reversed; there is no bend angle")
                continue
            t = self.binary64(element.radius.value * math.tan(theta / 2.0), element.line, location, "tangent length",
                              nonzero=True)  # R > 0 and theta > 0: a zero tangent length is an underflow
            self.bend_geometry[n] = {"theta": theta, "t": t, "d_in": d_in, "d_out": d_out, "ti": ti,
                                     "element": index, "start": element.start, "next": outgoing.end}
        tol = _BEND_RELATIVE_TOLERANCE
        for n, bend in self.bend_geometry.items():
            element = self.elements[bend["element"]]
            for leg, other in (("incoming", bend["start"]), ("outgoing", bend["next"])):
                span = _norm(_sub(bend["ti"], self.positions[other]))
                other_t = self.bend_geometry[other]["t"] if other in self.bend_geometry else 0.0
                if span - bend["t"] - other_t <= tol * span:
                    self.refuse("MBF-IMPORT-BEND-GEOMETRY-INVALID", element.line, f"LAYOUT line {element.line}",
                                f"the {leg} leg at tangent intersection {n} is not longer than the tangent length "
                                "R tan(theta/2)" + (" plus the adjacent bend's" if other_t else ""))
        for n, bend in self.bend_geometry.items():
            ti, t = bend["ti"], bend["t"]
            # Each tangent-node coordinate TI -/+ t d is formed exactly from its binary64 operands and rounded once,
            # so a non-zero coordinate below the binary64 range is refused, never rounded to 0 by a product.
            exact = {name: tuple(Fraction(ti[axis]) + sign * Fraction(t) * Fraction(bend[direction][axis])
                                 for axis in range(3))
                     for name, sign, direction in (("near", -1, "d_in"), ("far", 1, "d_out"))}
            line = self.elements[bend["element"]].line
            near = tuple(self.binary64(value, line, f"bend {n}", "tangent node") for value in exact["near"])
            far = tuple(self.binary64(value, line, f"bend {n}", "tangent node") for value in exact["far"])
            toward_ti = tuple(Fraction(ti[axis]) - (Fraction(near[axis]) + Fraction(far[axis])) / 2 for axis in range(3))
            if min(_norm(_sub(ti, near)), _norm(_sub(far, ti)), _norm(_sub(far, near))) == 0 or not any(toward_ti):
                self.refuse("MBF-IMPORT-BEND-GEOMETRY-INVALID", line, f"bend {n}",
                            f"bend {n}'s tangent nodes collapse onto the tangent intersection or each other once "
                            "rounded to binary64")
                raise _Refused
            bend.update(near=near, far=far, y_reference=_unit_exact(toward_ti))
        for element in self.elements:  # the generated straights, from tangent node to tangent node
            start = self.bend_geometry[element.start]["far"] if element.start in self.bend_geometry \
                else self.positions[element.start]
            end = self.bend_geometry[element.end]["near"] if element.end in self.bend_geometry \
                else self.positions[element.end]
            if not (0 < _norm(_sub(end, start)) < math.inf):
                self.refuse("MBF-IMPORT-BEND-GEOMETRY-INVALID", element.line, f"LAYOUT line {element.line}",
                            f"the straight to node {element.end} beside a bend has zero length once its generated "
                            "tangent nodes are rounded to binary64")
                raise _Refused

    # -- assembly (§15, §16) --------------------------------------------------------------
    def node_name_id(self, name: str) -> str:
        formed = f"node:{name}"
        if (self.bound or self.rebound) and name in getattr(self, "node_ids", {}):
            restored = self.node_ids[name]
            if self.rebound:
                self.unconfirmed.append(restored)
            return restored
        return formed

    def assemble(self) -> tuple[dict[str, Any], dict[str, Any]]:
        length_unit = "mm" if self.si else "in"
        angle_unit = "rad" if self.rad else "deg"
        provenance = lambda line: f"imported from MBF sha256:{self.sha}, line {line}"  # noqa: E731
        maps = self.maps
        id_map: dict[str, Any] = {"mbf_sha256": self.sha, "binding": "bound" if self.bound else (
            "rebound_unconfirmed" if self.rebound else "unbound"), "node_map": {}, "element_map": {},
            "support_map": {}, "bend_map": {}, "material_map": {}, "load_case_map": {}, "load_map": {}}
        used_ids: dict[str, str] = {}

        def claim(swb_id: str, what: str, line: int) -> str:
            if swb_id in used_ids and used_ids[swb_id] != what:
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, what,
                            f"{what} would take id {swb_id}, which {used_ids[swb_id]} already has")
            used_ids[swb_id] = what
            return swb_id

        # nodes
        nodes = []
        node_names: dict[int, tuple[str, ...]] = {}
        point_of: dict[str, tuple[float, float, float]] = {}
        for number in self.node_order:
            node = self.nodes[number]
            if number in self.bend_geometry:
                bend = self.bend_geometry[number]
                names = (f"{number}A", f"{number}B")
                points = (bend["near"], bend["far"])
            else:
                names, points = (str(number),), (self.positions[number],)
            node_names[number] = names
            for name, point in zip(names, points):
                point_of[name] = point
                swb_id = claim(self.node_name_id(name), f"CAEPIPE node {name}", node["line"])
                id_map["node_map"][name] = swb_id
                nodes.append({"id": swb_id, "label": name, "position": dict(zip("xyz", point)),
                              "provenance": provenance(node["line"])})
        node_id = {name: id_map["node_map"][name] for names in node_names.values() for name in names}
        if self.bound:
            self.incomplete("node_map", node_id, self.node_ids, "nodes")

        # materials (§7)
        materials, material_ids = self.assemble_materials(id_map, provenance, claim)
        # pipes, weights and temperatures
        factor = _METRES_PER_UNIT[length_unit]
        element_lookup: dict[tuple[str, str], tuple[str, dict]] = {}
        for swb_id, entry in maps.get("element_map", {}).items():
            element_lookup[(entry["from"], entry["to"])] = (swb_id, entry)
        pipes, components, unresolved = [], [], []
        pressures: dict[str, tuple[Num | None, _Element]] = {}
        weights: list[tuple[str, float, int]] = []
        thermals: list[tuple[str, float, int]] = []
        element_pairs: list[tuple[str, str]] = []
        self.carriers: dict[tuple[str, str], list[str]] = {}  # (MATERIAL or PIPE, name) -> pipe ids, layout order
        derived_y: list[str] = []  # straights whose y_reference is CAEPIPE's documented local y, derived
        for element in self.elements:
            start = node_names[element.start][-1]
            end = node_names[element.end][0]
            members = [(start, end, None)]
            if element.ti:
                members.append(node_names[element.end] + (self.bend_geometry[element.end],))
            for a, b, bend in members:
                caepipe_name = f"{a}-{b}"
                element_pairs.append((a, b))
                swb_id, reversed_, entry = f"pipe:{caepipe_name}", False, {}
                if (self.bound or self.rebound) and (a, b) in element_lookup:
                    swb_id, entry = element_lookup[(a, b)]
                    reversed_ = entry["reversed"]
                    if self.rebound:
                        self.unconfirmed.append(swb_id)
                    # The entry's variant is the element's: a bend record for a bend, an orientation for a straight
                    # (VER-002 r13), on both routes.
                    if ("bend" in entry) != (bend is not None):
                        self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", element.line, f"element_map[{swb_id}].bend",
                                    f"the sidecar records {'a' if 'bend' in entry else 'no'} bend for {swb_id}, but "
                                    f"the file's element {caepipe_name} is {'straight' if bend is None else 'a bend'}")
                    if self.bound and not self.sidecar_current:
                        self.legacy_element(swb_id, entry, element, caepipe_name, len(element_pairs) - 1, bend)
                claim(swb_id, f"CAEPIPE element {caepipe_name}", element.line)
                id_map["element_map"][caepipe_name] = swb_id
                section = self.pipes[element.section]
                ends = (node_id[b], node_id[a]) if reversed_ else (node_id[a], node_id[b])
                pipe: dict[str, Any] = {
                    "id": swb_id, "label": caepipe_name, "from": ends[0], "to": ends[1],
                    "material": material_ids[element.material],
                    "section": {"outside_diameter": {"value": section["od"].value, "unit": length_unit},
                                "wall_thickness": {"value": section["thk"].value, "unit": length_unit}},
                    "provenance": provenance(element.line)}
                restored = entry.get("y_reference") if self.bound else None
                # The frame kernel forms the orientation on the positions in m, from the pipe's own from-node.
                ends_m = [[value * factor + 0.0 for value in point_of[name]]
                          for name in ((b, a) if reversed_ else (a, b))]
                if bend is not None:
                    pipe["y_reference"] = dict(zip("xyz", bend["y_reference"]))
                elif restored is not None:  # the sidecar's record: restored as written, or refused
                    usable = runtime_frame_axes(*ends_m, [restored[axis] for axis in "xyz"]) is not None
                    pipe["y_reference"] = {axis: restored[axis] for axis in "xyz"} if usable else None
                    if not usable:
                        self.refuse("MBF-IMPORT-ORIENTATION-UNUSABLE", element.line, swb_id,
                                    f"the sidecar's y_reference for {swb_id} is not one the frame kernel forms an "
                                    "orientation from on the MBF's positions in m (finite, a part perpendicular to "
                                    "the element above 1e-12, and orthonormal right-handed axes within 1e-12); "
                                    "nothing is substituted")
                    elif entry.get("y_reference_authored") is False:
                        derived_y.append(swb_id)
                        # Recorded as derived: it must be the derivation, the documented local y on the MBF's
                        # positions (Design §15, agreement rule).
                        derived = dict(zip("xyz", local_y_reference(point_of[a], point_of[b],
                                                                     z_vertical=self.z_vertical)))
                        if pipe["y_reference"] != derived:
                            self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", element.line,
                                        f"element_map[{swb_id}].y_reference",
                                        f"the sidecar records {swb_id}'s y_reference as derived "
                                        "(y_reference_authored false), but it is not CAEPIPE's documented local y on "
                                        "the MBF's positions")
                        else:
                            self.diagnostics.append({
                                "code": "MBF-IMPORT-ORIENTATION-DEFAULTED", "severity": "info", "line": element.line,
                                "location": swb_id,
                                "message": f"{swb_id} has no authored y_reference; the sidecar records the derived "
                                           "one, CAEPIPE's documented pipe local y (CAEPIPE-LOCAL-AXES), equal to "
                                           "the derivation"})
                else:
                    derived_y.append(swb_id)
                    documented = local_y_reference(point_of[a], point_of[b], z_vertical=self.z_vertical)
                    pipe["y_reference"] = dict(zip("xyz", documented))
                    vertical = ("global Z vertical, in the rotated axes" if self.z_vertical else "global Y vertical")
                    self.diagnostics.append({
                        "code": "MBF-IMPORT-ORIENTATION-DEFAULTED", "severity": "info", "line": element.line,
                        "location": swb_id,
                        "message": f"{swb_id} has no y_reference in the MBF; it takes CAEPIPE's documented pipe "
                                   f"local y axis (CAEPIPE-LOCAL-AXES, {vertical}). A circular section's "
                                   "stiffness does not depend on it; only local-axis results do"})
                    axis = [ends_m[1][index] - ends_m[0][index] for index in range(3)]
                    if runtime_frame_axes(*ends_m, documented) is None and math.sqrt(_dot(axis, axis)) > AXIS_TOLERANCE:
                        self.refuse("MBF-IMPORT-ORIENTATION-UNUSABLE", element.line, swb_id,
                                    f"the frame kernel forms no orientation for {swb_id} from CAEPIPE's documented "
                                    "local y on the MBF's positions in m (they differ in direction from the file's "
                                    "own values), and the MBF states no other")
                if bend is not None:
                    components.append(self.bend_component(element, swb_id, node_id[a], length_unit, provenance,
                                                          unresolved, id_map, claim))
                pipes.append(pipe)
                self.carriers.setdefault(("material", element.material), []).append(swb_id)
                self.carriers.setdefault(("section", element.section), []).append(swb_id)
                pressures[swb_id] = (self.loads[element.load]["p1"] if element.load is not None else None, element)
                weight = self.weight_per_length(element)
                if weight != 0 or self.materials[element.material]["density"].exact != 0:
                    weight = self.binary64(weight, element.line, f"LAYOUT line {element.line}", "weight per length",
                                           nonzero=True)  # a non-zero density must not lose its weight
                    weights.append((swb_id, weight, element.line))
                if element.load is not None:
                    delta = self.binary64(self.loads[element.load]["t1"].exact - self.t_ref.exact, element.line,
                                          f"LOADS {element.load}", "temperature change") \
                        if self.t_ref is not None else None
                    if delta is None:
                        self.refuse("MBF-IMPORT-REFERENCE-TEMPERATURE-MISSING", element.line,
                                    f"LAYOUT line {element.line}",
                                    "a LOADS temperature is read, but OPTEMP gives no reference temperature")
                        raise _Refused
                    if delta != 0:
                        thermals.append((swb_id, delta, element.line))
        if self.bound:
            self.incomplete("element_map", [f"{a}-{b}" for a, b in element_pairs],
                            [f"{a}-{b}" for a, b in element_lookup], "elements")
        # The solve reads positions in m (value x factor + 0.0, core/units): an element whose ends coincide there has
        # zero length to it, however the file's own unit separates them.
        name_of = {swb_id: name for name, swb_id in id_map["node_map"].items()}
        radius_of = {component["geometry"]["bend_pipe_ref"]: component["geometry"]["bend_radius"]["value"]
                     for component in components}
        # What the runtime forms on the positions in m, read by the pressure region checks (Design §9.4): each pipe's
        # frame axis and length, and each realized bend's end tangents; None where the runtime forms none.
        self.frame_x: dict[str, list[float] | None] = {}
        self.length_m: dict[str, float] = {}
        self.arc_tangents: dict[str, tuple[list[float], list[float]] | None] = {}
        for pipe in pipes:
            a, b = ([value * factor + 0.0 for value in point_of[name_of[pipe[end]]]] for end in ("from", "to"))
            if a == b:
                self.refuse("MBF-IMPORT-GEOMETRY-ZERO-LENGTH", pressures[pipe["id"]][1].line,
                            pipe["id"], f"{pipe['id']} has zero length in metres, the unit the solve reads")
                continue
            if pipe["y_reference"] is None:  # refused above (MBF-IMPORT-ORIENTATION-UNUSABLE)
                continue
            y_reference = [pipe["y_reference"][axis] for axis in "xyz"]
            axes = runtime_frame_axes(a, b, y_reference)
            self.frame_x[pipe["id"]] = None if axes is None else axes[0]
            axis = [b[index] - a[index] for index in range(3)]
            self.length_m[pipe["id"]] = math.sqrt(_dot(axis, axis))
            if pipe["id"] in radius_of:
                self.arc_tangents[pipe["id"]] = runtime_bend_tangents(a, b, radius_of[pipe["id"]] * factor + 0.0,
                                                                      y_reference)
        sections_used = dict.fromkeys(element.section for element in self.elements)
        if self.bound and not self.sidecar_current:  # format 2 leaves sections to the MBF
            self.incomplete("section_map", sections_used, self.maps["section_map"], "sections")
            self.legacy_tables()
        for name in sections_used:
            section = self.pipes[name]
            if not runtime_section_valid(section["od"].value, section["thk"].value, factor):
                self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", section["line"], f"PIPE {name}",
                            f"section {name} needs 0 < thickness < OD/2 with a bore radius OD/2 - thickness that is "
                            "positive and below OD/2 on the values in m the solve reads; the file's decimals pass, "
                            "but their binary64 values in m do not")
        if self.has_errors():
            raise _Refused
        if self.elements and all(element.load is None for element in self.elements):
            self.listed("tbd", "MBF-IMPORT-LOADS-ABSENT", self.sections["LAYOUT"][0], "LAYOUT",
                        "the file has no LOADS section; elements are imported with no temperature, pressure or "
                        "contents load, and CAEPIPE's treatment of an element without a load record is not documented")
        self.list_unused(material_ids)
        supports, support_ids = self.assemble_supports(node_id, provenance, id_map, claim)
        for support_id, reason in runtime_support_findings(
                (support["id"], support["node"], support.get("family"), support["restraints"]) for support in supports):
            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", self.support_lines[support_id], support_id,
                        f"support {support_id}: {reason}; the solve refuses it (SUPPORT_INPUT_INVALID)")
        if self.has_errors():
            raise _Refused
        self.labels = {pipe["id"]: pipe["label"] for pipe in pipes}
        arcs = {component["geometry"]["bend_pipe_ref"] for component in components}
        pressure = self.assemble_pressure(pipes, pressures, arcs, point_of, id_map, claim)
        load_cases = self.assemble_loads(node_id, support_ids, weights, thermals, length_unit, angle_unit,
                                         provenance, id_map, claim, None if pressure is None else pressure[1])
        if self.has_errors():
            raise _Refused
        if pressure is not None:
            moved = [load["id"] for case in load_cases for load in case["primitive_loads"]
                     if load["category"] == "imposed_displacement"]
            if moved:
                self.refuse("MBF-IMPORT-PRESSURE-DISPLACEMENT-UNSUPPORTED", 0, ", ".join(moved),
                            "the file has pressure and anchor displacements; the exact pressure route takes boundary "
                            "motion only in load-state form (EXACT_IMPOSED_DISPLACEMENT_PRIMITIVE_UNSUPPORTED), which "
                            "this importer does not write")
                raise _Refused
            # The contract needs an explicit allowance on every section. PIPE field 4 is read only as 0 (§8); the
            # manual documents no default for an omitted field, so a section without it is refused, not zeroed.
            missing = sorted({element.section for element in self.elements
                              if not self.pipes[element.section]["allowance_given"]})
            for name in missing:
                self.refuse("MBF-IMPORT-CORROSION-ALLOWANCE-MISSING", self.pipes[name]["line"], f"PIPE {name} field 4",
                            f"section {name} gives no corrosion allowance; the exact pressure contract needs an "
                            "explicit allowance on every section, and none is supplied")
            if missing:
                raise _Refused
            for pipe in pipes:  # each section's own explicit 0 (field 4)
                pipe["section"]["corrosion_allowance"] = {"value": 0.0, "unit": length_unit}
            # The exact route's stable-suffix check (pressure_runtime.rs check_suffixes, ':' -> '-'), per kind.
            for kind, ids in (("pipe", [pipe["id"] for pipe in pipes]), ("node", [node["id"] for node in nodes]),
                              ("support", [support["id"] for support in supports]),
                              ("load_case", [case["id"] for case in load_cases])):
                seen: dict[str, str] = {}
                for identity in ids:
                    previous = seen.setdefault(identity.replace(":", "-"), identity)
                    if previous != identity:
                        self.refuse("MBF-IMPORT-PRESSURE-ID-COLLISION", 0, f"{kind} {previous}, {identity}",
                                    f"{kind} ids {previous} and {identity} are equal once ':' is read as '-', which "
                                    "the exact route refuses (EXACT_PRESSURE_RESULT_ID_COLLISION)")
            if self.has_errors():
                raise _Refused
        self.report_unrecoverable()

        units = ({"length": "mm", "force": "N", "moment": "N*m", "pressure": "MPa", "stress": "Pa",
                  "temperature": "degC", "angle": angle_unit} if self.si else
                 {"length": "in", "force": "lbf", "moment": "lbf*in", "pressure": "psi", "stress": "psi",
                  "temperature": "degF", "angle": angle_unit})
        title = self.heading[0][1].strip() if self.heading and self.heading[0][1].strip() else self.source_label
        notes = [f"line {line}: {text.strip()}" for line, text in self.heading[1:] if text.strip()] + self.notes
        project: dict[str, Any] = {"id": f"project:mbf-{self.sha[:12]}", "name": title,
                                   "description": f"Imported from MBF sha256:{self.sha}", "units": units}
        if notes:
            project["notes"] = "\n".join(notes)
        binding = "bound to the SWBPIPE writer sidecar" if self.bound else (
            "unbound; sidecar ids rebound by CAEPIPE name, unconfirmed" if self.rebound else "unbound")
        model: dict[str, Any] = {
            "schema_version": current_model_schema_version(),
            "document_kind": MODEL_KIND,
            **({} if pressure is None else {"pressure_contract": pressure[0]}),
            "data_boundary": {**_BLANK_BOUNDARY,
                              "private_data_policy": f"imported from MBF sha256:{self.sha} ({binding}); local user "
                                                     "document not committed to repository"},
            "project": project,
            "analysis_status": dict(_ANALYSIS_STATUS),
            "materials": materials,
            "nodes": nodes,
            "pipe_segments": pipes,
            "supports": supports,
            "components": components,
            "load_cases": load_cases,
            "diagnostics": [],
        }
        if unresolved:
            model["unresolved_assumptions"] = unresolved
        id_map["rebound_unconfirmed"] = sorted(set(self.unconfirmed))
        id_map["y_reference_derived"] = derived_y
        return model, id_map

    def bend_component(self, element: _Element, pipe_id: str, near_id: str, length_unit: str, provenance: Any,
                       unresolved: list[dict[str, Any]], id_map: dict[str, Any], claim: Any) -> dict[str, Any]:
        n = element.end
        component_id = f"component:bend-{n}"
        if self.bound or self.rebound:
            entry = self.maps.get("element_map", {}).get(pipe_id)
            bend = entry.get("bend") if entry is not None else None
            if bend is not None:  # an entry without one is refused with the element (its variant)
                component_id = bend["component"]
                if self.rebound:
                    self.unconfirmed.append(component_id)
            if self.bound:
                self.listed("approximated", "MBF-IMPORT-ID-UNRECOVERABLE", element.line, f"bend {n}",
                            f"the sidecar does not record which tangent node bend {n}'s component names; the "
                            f"importer uses the near tangent node {n}A")
        claim(component_id, f"CAEPIPE bend {n}", element.line)
        id_map["bend_map"][str(n)] = component_id
        component: dict[str, Any] = {
            "id": component_id, "label": f"bend {n}", "kind": "bend", "node": near_id,
            "geometry": {"bend_pipe_ref": pipe_id,
                         "bend_radius": {"value": element.radius.value, "unit": length_unit},
                         "bend_geometry_source_reference": f"MBF sha256:{self.sha}, line {element.line}"},
            "mechanics_interface": {"solver_consumption": "curved_bend_macro_element"},
            "provenance": provenance(element.line),
        }
        comments = "comments FF and BSIF" if element.sifs is not None else "comment FF"
        if element.factor is not None:
            component["modifiers"] = {"flexibility_factor_user_value": {"value": element.factor.value, "unit": "none"},
                                      "source_reference": f"MBF sha256:{self.sha}, line {element.line}, {comments}"}
        else:
            component["modifiers"] = {"source_reference": f"MBF sha256:{self.sha}, line {element.line}: the MBF "
                                                          "supplies no flexibility factor"
                                                          + (", comment BSIF" if element.sifs is not None else "")}
            unresolved.append({"assumption_id": f"assumption:mbf-bend-{n}-flexibility", "kind": "open_issue",
                               "statement": f"bend {n} has no flexibility factor in the MBF; enter a user factor",
                               "status": "unresolved", "affected_refs": [component_id]})
            self.listed("omitted", "MBF-IMPORT-BEND-FLEXIBILITY-MISSING", element.line, f"bend {n}",
                        f"bend {n} has no FF comment; SWBPIPE never supplies a factor, so the bend is incomplete")
        if element.sifs is not None:
            component["modifiers"].update({name: {"value": number.value, "unit": "none"}
                                           for name, number in zip(BEND_SIF_FIELDS, element.sifs)})
        return component

    def weight_per_length(self, element: _Element) -> float:
        """w = -rho A (English, lbf/in) or -rho g0 A (SI, N/m), A the bare-metal area (§7.3)."""
        density = self.materials[element.material]["density"].value
        if density == 0:
            return 0.0
        section = self.pipes[element.section]
        od, thk = section["od"].value, section["thk"].value
        try:
            if self.si:
                od, thk = od / 1000.0, thk / 1000.0
                return -density * (math.pi / 4.0 * (od**2 - (od - 2.0 * thk) ** 2)) * STANDARD_GRAVITY
            return -density * (math.pi / 4.0 * (od**2 - (od - 2.0 * thk) ** 2))
        except OverflowError:
            return -math.inf  # refused as unrepresentable by the caller

    def material_qualifies(self, name: str) -> tuple[str, str, str | None, bool] | None:
        """(origin, source name, redistribution status, recorded) from a bound sidecar's recorded origin or the
        caller's declaration, else None. On a bound import a recorded origin is the material's (the writer read it
        from the source's provenance), and a declaration of another origin for that material is refused; the
        declaration supplies the origin the sidecar records as "undeclared"."""
        declared = None
        if isinstance(self.provenance, Mapping):
            declared = self.provenance.get(name, self.provenance.get("*"))
        entry = self.maps.get("material_map", {}).get(name) if self.bound else None
        recorded = entry.get("origin") if entry is not None else None
        if recorded in MATERIAL_ORIGINS:
            if isinstance(declared, Mapping) and declared.get("origin") != recorded:
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", self.materials[name]["line"],
                            f"material_map[{name}].origin",
                            f"the bound sidecar records material {name}'s origin as {recorded}, and the caller "
                            f"declares {declared.get('origin')!r}; neither is preferred")
                return None
            return recorded, "material origin recorded by the SWBPIPE MBF writer sidecar", (
                "private_only" if recorded == "user_supplied" else "public_permissive"), True
        if isinstance(declared, Mapping):
            origin, source = declared.get("origin"), declared.get("source_name")
            status = declared.get("redistribution_status")
            if origin in MATERIAL_ORIGINS and isinstance(source, str) and source.strip():
                if origin == "user_supplied":
                    return origin, source, "private_only", False
                if status in (None, "public_permissive", "private_only"):
                    return origin, source, status, False
        return None

    def assemble_materials(self, id_map: dict[str, Any], provenance: Any, claim: Any):
        used = []
        for element in self.elements:
            if element.material not in used:
                used.append(element.material)
        materials, material_ids, merged = [], {}, {}
        if self.bound:
            self.incomplete("material_map", used, self.maps["material_map"], "materials")
        e_unit, alpha_unit = ("MPa", "1/degC") if self.si else ("psi", "1/degF")
        for name in used:
            record = self.materials[name]
            if record.get("refused"):
                raise _Refused
            before = len(self.diagnostics)
            qualified = self.material_qualifies(name)
            if qualified is None and len(self.diagnostics) > before:  # refused there (a conflicting declaration)
                continue
            if qualified is None:
                self.refuse("MBF-IMPORT-MATERIAL-PROVENANCE-REQUIRED", record["line"], f"MATERIAL {name}",
                            f"material {name} needs an affirmative project_invented or user_supplied provenance "
                            "declaration; vendor-library, unknown or undeclared origins are refused")
                continue
            modulus, alpha, _ = record["rows"][0]
            nu = record["nu"]
            swb_id = f"material:{name}"
            if self.bound or self.rebound:
                entry = self.maps.get("material_map", {}).get(name)
                if entry is not None:
                    swb_id = entry["swbpipe_material"]
                    if self.rebound:
                        self.unconfirmed.append(swb_id)
            material_ids[name] = swb_id
            id_map["material_map"][name] = swb_id
            key = (modulus.value, nu.value, alpha.value)
            if swb_id in merged:
                if merged[swb_id] != key:
                    self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", record["line"], f"MATERIAL {name}",
                                f"material {name} maps to {swb_id} like another MBF material, but E, nu or the "
                                "expansion coefficient differ")
                continue
            claim(swb_id, f"MBF material {name}", record["line"])
            merged[swb_id] = key
            e_pa = self.binary64(modulus.value * _PA_PER_UNIT[e_unit], record["line"], f"MATERIAL {name}", "E in Pa")
            shear = self.binary64(Fraction(e_pa) / (2 * (1 + Fraction(nu.value))), record["line"], f"MATERIAL {name}",
                                  "shear modulus")
            origin, source, status, recorded = qualified
            note = (f"G derived from E and nu (isotropic): G = E/(2(1+nu)) with E in Pa, from MBF MATERIAL {name} "
                    f"E = {modulus.text} {e_unit} and nu = {nu.text}")
            # A recorded origin keeps the certification the writer read it from (si_writer material_origin), so
            # the writer records it again; a declared one is the caller's token, which the writer does not read
            # as a certification, so a re-export asks for the declaration again.
            certification = {"project_invented": INVENTED_CERTIFICATION,
                             "user_supplied": USER_SUPPLIED_CERTIFICATION}[origin] if recorded else origin
            provenance_record: dict[str, Any] = {"source_name": source, "source_location": f"sha256:{self.sha}",
                                                 "contributor_certification": certification,
                                                 "verification_note": note}
            if status is not None:
                provenance_record["redistribution_status"] = status
            materials.append({
                "id": swb_id, "label": record["description"].strip() or name,
                "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                "elastic_modulus": {"value": modulus.value, "unit": e_unit},
                "poisson_ratio": {"value": nu.value, "unit": "1"},
                "shear_modulus": {"value": shear, "unit": "Pa"},
                "thermal_expansion_coefficient": {"value": alpha.value, "unit": alpha_unit},
                "provenance": provenance(record["line"]),
                "provenance_record": provenance_record,
            })
        if self.has_errors():
            raise _Refused
        return materials, material_ids

    def list_unused(self, material_ids: Mapping[str, str]) -> None:
        used_sections = {element.section for element in self.elements}
        used_loads = {element.load for element in self.elements}
        for table, used, what in ((self.materials, set(material_ids), "MATERIAL"), (self.pipes, used_sections, "PIPE"),
                                  (self.loads, used_loads, "LOADS")):
            for name, record in table.items():
                if name not in used:
                    self.listed("omitted", "MBF-IMPORT-RECORD-UNUSED", record["line"], f"{what} {name}",
                                f"{what} {name} is not referenced by any element and is not imported")

    def assemble_supports(self, node_id: Mapping[str, str], provenance: Any, id_map: dict[str, Any], claim: Any):
        supports, support_ids = [], {}
        self.support_lines: dict[str, int] = {}
        sidecar_by_node: dict[str, list[tuple[str, Mapping[str, Any]]]] = {}
        for swb_id, entry in self.maps.get("support_map", {}).items():
            sidecar_by_node.setdefault(entry["node"], []).append((swb_id, entry))
        matched_sidecar: set[str] = set()
        stiffness_unit = STIFFNESS_UNIT[self.si]
        for number in self.node_order:
            record = self.node_data.get(number)
            if record is None:
                continue
            name = str(number)
            # (id, DOFs, family, line, label): an anchor, a restraint, a guide, limit stops, then springs (§10.4, §12)
            imported: list[tuple[str, list[str], str | None, int, str]] = []
            springs: dict[str, tuple[Num, int]] = {}

            def add(item: tuple[str, list[str], str | None, int, str], what: str) -> None:
                if any(existing[0] == item[0] for existing in imported):
                    self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", item[3], f"node {name}",
                                f"a second {what} at node {name} is not read")
                else:
                    imported.append(item)

            def spring(dof: str, stiffness: Num, line: int, what: str) -> None:
                if dof in springs:
                    self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, f"node {name}",
                                f"{what} adds a second spring on {dof} at node {name}, which is not read")
                else:
                    springs[dof] = (stiffness, line)

            flexible = bool(record["anchor"] and record["trak"])
            if record["anchor"] and not flexible:
                imported.append((f"support:{name}", list(DOFS), "anchor", record["anchor"][0], "anchor"))
            if flexible:
                for axis, stiffness in sorted(record["trak"][0][0].items()):
                    if stiffness.exact != 0:
                        spring(DOFS[axis], stiffness, record["anchor"][0], "TRAK")
            dofs = [DOFS[axis] for axis in sorted(record["fixd"])] + [DOFS[3 + axis] for axis in sorted(record["fixr"])]
            if dofs:
                line = min(list(record["fixd"].values()) + list(record["fixr"].values()))
                imported.append((f"support:{name}-restraint", dofs, None, line, "restraint"))
            for item in record["supports"]:
                axis, line, stiffness = item["axis"], item["line"], item["stiffness"]
                if item["kind"] == "guide":  # restrains its element's two lateral translations (§12)
                    lateral = [DOFS[index] for index in range(3) if index != axis]
                    if stiffness is None:
                        add((f"support:{name}-guide", lateral, "guide", line, "guide"), "guide")
                    for dof in lateral if stiffness is not None else ():
                        spring(dof, stiffness, line, "a flexible guide (G with STIFF)")
                elif item["kind"] == "stop":
                    if stiffness is None:
                        add((f"support:{name}-stop-{'xyz'[axis]}", [DOFS[axis]], "line_stop", line, "limit stop"),
                            f"limit stop along {'xyz'[axis]}")
                    else:
                        spring(DOFS[axis], stiffness, line, "a flexible limit stop (LS with STIFF)")
                else:
                    spring(DOFS[axis], stiffness, line, "a skewed restraint (K)")
            rigid = set().union(*(set(item[1]) for item in imported))
            for dof, (_, line) in springs.items():
                if dof in rigid:
                    self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, f"node {name}",
                                f"a spring on {dof} at node {name}, which a rigid support there already restrains, is "
                                "not read")
            for dof in DOFS:
                if dof in springs:
                    imported.append((f"support:{name}-spring-{dof.lower()}", [dof], SPRING_FAMILY, springs[dof][1],
                                     "spring"))
            tags = record["tag"]
            if not imported:
                for _, line in tags:
                    self.listed("omitted", "MBF-IMPORT-TAG-WITHOUT-SUPPORT", line, f"node {name} TAG",
                                f"TAG at node {name}, which has no support, is not imported")
                continue
            line = imported[0][3]
            restored = sidecar_by_node.get(name, []) if (self.bound or self.rebound) else []
            ids = []
            if self.bound and not restored:
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, f"node {name}",
                            f"the bound sidecar records no support at node {name}, which the file supports")
                continue
            if restored:
                wanted = set().union(*(set(dofs) for _, dofs, family, _, _ in imported if family != SPRING_FAMILY))
                given = set().union(*(set(entry["restrained"]) for _, entry in restored
                                      if entry.get("family") != SPRING_FAMILY))
                given_springs = sorted(dof for _, entry in restored if entry.get("family") == SPRING_FAMILY
                                       for dof in entry["restrained"])
                if wanted != given or given_springs != sorted(springs):
                    self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, f"node {name}",
                                f"the sidecar's supports at node {name} restrain other DOFs, or carry springs on other "
                                "DOFs, than the MBF")
                    continue
                entries = []
                for swb_id, entry in restored:
                    # An unauthored family is recorded as the solve's inference and restored as absent.
                    family = entry.get("family") if entry.get("family_authored", True) else None
                    if "family" not in entry:
                        self.listed("approximated", "MBF-IMPORT-ID-UNRECOVERABLE", line, f"support_map[{swb_id}]",
                                    f"the sidecar entry for {swb_id} carries no family, which is not recoverable")
                    label = ("spring" if family == SPRING_FAMILY else "anchor" if family == "anchor" else "restraint")
                    entries.append((swb_id, [dof for dof in DOFS if dof in entry["restrained"]], family, label, line))
                    kind = "anchor" if record["anchor"] else "restraint"
                    if self.bound and not self.sidecar_current and entry["caepipe_kind"] != kind:
                        self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, f"support_map[{swb_id}].caepipe_kind",
                                    f"the sidecar records {swb_id} as an {entry['caepipe_kind']}, but the file "
                                    f"writes node {name} as {'an anchor (KA)' if kind == 'anchor' else 'a restraint'}")
                    matched_sidecar.add(swb_id)
                    if self.rebound:
                        self.unconfirmed.append(swb_id)
            else:
                entries = [(swb_id, dofs, family, label, own) for swb_id, dofs, family, own, label in imported]
            for swb_id, dofs, family, label, own in entries:
                claim(swb_id, f"support at node {name}", line)
                self.support_lines[swb_id] = own
                support: dict[str, Any] = {"id": swb_id, "label": tags[0][0] if tags else f"{label} at {name}",
                                           "node": node_id[name], "restraints": dofs,
                                           "provenance": provenance(line)}
                if family is not None:
                    support["family"] = family
                if family == SPRING_FAMILY:
                    stiffness, spring_line = springs[dofs[0]]
                    if not (math.isfinite(stiffness.value * _N_PER_M_PER_UNIT[stiffness_unit])
                            and stiffness.value * _N_PER_M_PER_UNIT[stiffness_unit] > 0):
                        self.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", spring_line, swb_id,
                                    f"the stiffness of {swb_id} is outside the binary64 range in N/m, as the solve "
                                    "reads it")
                    support["stiffness"] = {"dof": dofs[0], "value": {"value": stiffness.value,
                                                                      "unit": stiffness_unit}}
                supports.append(support)
                ids.append((swb_id, dofs))
            support_ids[number] = ids
            id_map["support_map"][name] = [swb_id for swb_id, _ in ids]
        if self.bound:
            self.incomplete("support_map", matched_sidecar, self.maps["support_map"], "supports")
        if self.rebound:
            for swb_id in self.maps.get("support_map", {}):
                if swb_id not in matched_sidecar:
                    self.listed("omitted", "MBF-IMPORT-SIDECAR-ENTRY-ABSENT", 0, f"support_map[{swb_id}]",
                                f"sidecar support {swb_id} has no counterpart in this MBF")
        return supports, support_ids

    def case_ids(self, id_map: dict[str, Any]) -> dict[str, str]:
        labels = {"sustained": LOAD_CASE_LABELS["sustained"], "expansion": LOAD_CASE_LABELS["expansion"]}
        ids = {"sustained": SUSTAINED_CASE, "expansion": EXPANSION_CASE}
        if self.bound:
            for swb_id, label in self.maps.get("load_case_map", {}).items():
                for kind, wanted in labels.items():
                    if label == wanted:
                        ids[kind] = swb_id
        return ids

    def assemble_loads(self, node_id, support_ids, weights, thermals, length_unit, angle_unit, provenance,
                       id_map, claim, regions=None):
        cases = self.case_ids(id_map)
        force_unit, moment_unit = ("N", "N*m") if self.si else ("lbf", "lbf*ft")
        primitives: dict[str, list[dict[str, Any]]] = {"sustained": [], "expansion": []}
        for pipe_id, weight, line in weights:
            primitives["sustained"].append({"category": "weight", "target": {"type": "element", "pipe": pipe_id},
                                            "direction": "global_y", "magnitude": weight,
                                            "unit": "N/m" if self.si else "lbf/in",
                                            "dimension": "force_per_length", "line": line, "axis": None})
        for number in self.node_order:
            record = self.node_data.get(number)
            if record is None:
                continue
            name = str(number)
            for kind, category, unit, dimension in (("forces", "concentrated_force", force_unit, "force"),
                                                    ("moments", "concentrated_moment", moment_unit, "moment")):
                for axis in sorted(record[kind]):
                    items = record[kind][axis]
                    total = sum(number_.exact for number_, _ in items)
                    total = self.binary64(total, items[0][1], f"node {number}", "summed load") if total else 0.0
                    if total == 0:
                        continue
                    direction = _FORCE_DIRECTIONS[axis] if kind == "forces" else DOFS[3 + axis]
                    primitives["sustained"].append({"category": category,
                                                    "target": {"type": "node", "node": node_id[name]},
                                                    "direction": direction, "magnitude": total, "unit": unit,
                                                    "dimension": dimension, "line": items[0][1],
                                                    "axis": ("t" if kind == "forces" else "r", axis)})
        for pipe_id, delta, line in thermals:
            primitives["expansion"].append({"category": "thermal", "target": {"type": "element", "pipe": pipe_id},
                                            "direction": _THERMAL_DIRECTION, "magnitude": delta,
                                            "unit": "degC" if self.si else "degF",
                                            "dimension": "temperature_interval", "line": line, "axis": None})
        for number in self.node_order:
            record = self.node_data.get(number)
            if record is None or not record["anchor"]:
                continue
            for kind, offset, unit, dimension in (("disp", 0, length_unit, "displacement"),
                                                  ("rot", 3, angle_unit, "rotation")):
                for axis in sorted(record[kind]):
                    items = record[kind][axis]
                    total = sum(number_.exact for number_, _ in items)
                    total = self.binary64(total, items[0][1], f"node {number}", "summed load") if total else 0.0
                    if total == 0:
                        continue
                    dof = DOFS[offset + axis]
                    candidates = [swb_id for swb_id, dofs in support_ids.get(number, []) if dof in dofs]
                    target = candidates[0]
                    if len(candidates) > 1:
                        target = self.displacement_target(str(number), dof, candidates) or target
                    primitives["expansion"].append({"category": "imposed_displacement",
                                                    "target": {"type": "support", "support": target, "dof": dof},
                                                    "direction": dof, "magnitude": total, "unit": unit,
                                                    "dimension": dimension, "line": items[0][1],
                                                    "axis": _DIRECTION_AXIS[dof]})
        restorable = self.restorable_primitives(cases)
        matched: set[tuple] = set()
        load_cases = []
        ids_seen: dict[str, str] = {}  # primitive id -> what took it, across cases (the solve's DUPLICATE_ID)
        for kind, label_key, case_kind in (("sustained", "sustained", "sustained"),
                                           ("expansion", "expansion", "thermal")):
            if not primitives[kind] and not (regions and kind == "sustained"):
                continue
            if self.bound and LOAD_CASE_LABELS[label_key] not in self.maps["load_case_map"].values():
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", 0, "load_case_map",
                            f"the bound sidecar records no case {LOAD_CASE_LABELS[label_key]!r}, which the file has")
            case_id = claim(cases[kind], f"CAEPIPE load case {LOAD_CASE_LABELS[label_key]}", 0)
            id_map["load_case_map"][LOAD_CASE_LABELS[label_key]] = case_id
            loads = []
            for item in primitives[kind]:
                target = item["target"]
                target_id = target.get("node") or target.get("pipe") or target.get("support")
                formed = f"load:{item['category']}:{target_id}" + (
                    f":{item['direction']}" if item["category"] in ("concentrated_force", "concentrated_moment",
                                                                    "imposed_displacement") else "")
                load_id, direction = formed, item["direction"]
                target_kind = "support" if "support" in target else ("node" if "node" in target else "pipe")
                key = (case_id, item["category"], target_kind, target_id, item["axis"])
                found = [option for option in (key, (*key[:4], "*")) if option in restorable]
                match = restorable[found[0]] if found else None
                matched.update(found[:1])
                if self.bound and match is None:
                    self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", item["line"], formed,
                                f"the bound sidecar's load_map records no primitive for {formed}, which the file has")
                # A recorded direction the MBF determines (all but a thermal one, which the MBF does not carry) must be
                # the MBF's: the gravity axis for a weight, the comment's axis for a force or moment, the moved DOF
                # for a displacement (Design §15, agreement rule).
                for map_key, recorded in (match[3] if self.bound and match is not None else []):
                    if (item["category"] != "thermal" and recorded is not None
                            and _DIRECTION_AXIS.get(recorded) != _DIRECTION_AXIS.get(item["direction"])):
                        self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", item["line"], f"load_map[{map_key}].direction",
                                    f"the sidecar records the direction {recorded} for {formed}, but the MBF gives "
                                    f"{item['direction']}")
                if self.bound and match is not None and item["category"] == "thermal":
                    direction = self.thermal_direction(match[3], formed, item["line"])
                if self.bound and match is not None and match[1] == 1:
                    load_id = match[0]
                elif self.bound and match is not None:  # many-to-one (REQ-016): reported by name, entry by entry
                    keys = [map_key for map_key, _ in match[3]]
                    named = ", ".join(f"load_map[{key}]" for key in keys)
                    self.listed("approximated", "MBF-IMPORT-ID-UNRECOVERABLE", item["line"],
                                f"load_map[{case_id}/{formed}]",
                                f"the MBF holds {named} summed as one primitive; no single sidecar id is its own, so "
                                f"it keeps its formed id {formed}")
                    for map_key in keys:
                        self.listed("approximated", "MBF-IMPORT-ID-UNRECOVERABLE", item["line"], f"load_map[{map_key}]",
                                    f"load_map[{map_key}] is one of the {len(keys)} primitives the MBF sums into "
                                    f"{formed}; its id is not recoverable")
                # Restored and formed primitive ids are unique across cases, or the import is refused by name: two
                # primitives under one id would overwrite each other here and the solve refuses them (DUPLICATE_ID).
                what = f"the primitive {formed} of case {case_id}"
                if load_id in ids_seen:
                    self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", item["line"], f"load_map[{case_id}/{load_id}]",
                                f"{what} would take id {load_id}, which {ids_seen[load_id]} already has; primitive "
                                "ids must be unique (DUPLICATE_ID)")
                    continue
                ids_seen[load_id] = what
                id_map["load_map"][load_id] = {"case": case_id, "category": item["category"], "target": target_id}
                if self.bound and match is not None and match[1] > 1:
                    id_map["load_map"][load_id]["summed_entries"] = [map_key for map_key, _ in match[3]]
                loads.append({"id": load_id, "category": item["category"], "target": target, "direction": direction,
                              "magnitude": {"value": item["magnitude"], "unit": item["unit"]},
                              "dimension": item["dimension"], "provenance": provenance(item["line"])})
            case = {"id": case_id, "label": LOAD_CASE_LABELS[label_key], "kind": case_kind, "status": "draft",
                    "primitive_loads": loads, "provenance":
                    f"imported from MBF sha256:{self.sha}; CAEPIPE load case {LOAD_CASE_LABELS[label_key]}"}
            if regions is not None:  # Pressure 1 acts in the sustained case, not in the expansion case (§9.4)
                case["pressure_regions"] = regions if kind == "sustained" else []
            load_cases.append(case)
        if self.bound:
            self.incomplete("load_map", sorted(matched, key=repr), sorted(restorable, key=repr), "primitives")
        if self.bound:  # a bound sidecar names exactly the file's cases
            self.incomplete("load_case_map", [case["label"] for case in load_cases],
                            list(self.maps["load_case_map"].values()), "load cases")
        return load_cases

    # -- §9.4 pressure ----------------------------------------------------------
    def assemble_pressure(self, pipes, pressures, arcs, point_of, id_map, claim):
        """(contract, regions) from LOADS Pressure 1, or None when the model has no pressure."""
        active = {pipe_id: number for pipe_id, (number, _) in pressures.items()
                  if number is not None and number.exact != 0}
        stored = (self.maps.get("pressure_map") or None) if self.bound else None
        if self.bound and active and stored is None:  # a bound import never falls through to unbound formation
            self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", self.loads_line(), "pressure_map",
                        "the file has Pressure 1, but the bound sidecar records no pressure_map (contract, shear model "
                        "and regions); a bound import restores them and never forms regions or defaults the shear "
                        "model")
            raise _Refused
        if not active and stored is None:
            return None
        # The solve reads Pressure 1 in Pa (value x factor + 0.0, core/units) and refuses a non-finite one
        # (PRESSURE_REGION_PRESSURE_INVALID): a finite bar or psi value can overflow there.
        to_pa = 100_000.0 if self.si else _PA_PER_UNIT["psi"]
        for name in dict.fromkeys(pressures[pipe_id][1].load for pipe_id in active):
            if not math.isfinite(self.loads[name]["p1"].value * to_pa + 0.0):
                self.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", self.loads[name]["line"], f"LOADS {name} Pressure 1",
                            f"Pressure 1 of LOADS {name} is finite in {PRESSURE_UNIT[self.si]} but beyond binary64 in "
                            "Pa, the unit the solve reads")
        if self.has_errors():
            raise _Refused
        contract = dict(PRESSURE_CONTRACT)
        if stored is not None:
            contract["shear_model"] = stored["contract"]["shear_model"]
            if self.shear_model is not None and self.shear_model != contract["shear_model"]:
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", 0, "shear_model",
                            f"the shear_model argument {self.shear_model} differs from the bound sidecar's "
                            f"{contract['shear_model']}, which a bound import keeps")
                raise _Refused
        elif self.shear_model is not None:
            contract["shear_model"] = self.shear_model
        if active:
            self.pressure_effects()
        ends = {pipe["id"]: (pipe["from"], pipe["to"]) for pipe in pipes}
        name_of = {swb_id: name for name, swb_id in id_map["node_map"].items()}
        unit = PRESSURE_UNIT[self.si]
        if stored is not None:
            members = self.restore_regions(stored["regions"], active, ends, name_of)
            if members is not None:
                members = self.check_restored(members, pressures, ends, arcs, point_of, name_of,
                                              {pipe["id"]: pipe["label"] for pipe in pipes})
            how = ("restored from the sidecar's pressure_map (bound import), each checked whole as the exact route "
                   "checks it in its declared order")
        else:
            members = self.form_regions(active, pressures, ends, arcs, point_of, name_of,
                                        {pipe["id"]: pipe["label"] for pipe in pipes})
            how = ("formed as maximal chains of elements with equal Pressure 1 and bore, ended at tees, where "
                   "pressure or bore changes, and at straight-to-straight corners of a run the exact route's "
                   "collinearity check refuses whole; closure transfer at each real end (free end, tee, pressure or "
                   "bore change) is the caller's declaration (pressure_closure), never inferred from the file; a "
                   "corner terminal takes transfers_to_wall, deduced from geometry: the pipe wall is continuous "
                   "there, so it carries the corner's pressure force pA(t_in - t_out), unless pressure_closure.nodes "
                   "names the corner (Agent 0's decision under the owner's delegation, 2026-10-27)")
            self.pressure_notes.append(
                "Shear model: " + (f"the caller's {contract['shear_model']}" if self.shear_model is not None else
                                   "timoshenko_annulus_energy_v1, the exact contract's default (owner D-6: Timoshenko "
                                   "with the energy-matched annular shear factor), as the MBF states none")
                + "; realized curved bends stay Euler-Bernoulli under either model, as the route discloses (Agent "
                "0's decision under the owner's delegation, 2026-10-27).")
        if members is None:
            raise _Refused
        regions, region_map = [], {}
        element_name = {swb_id: name for name, swb_id in id_map["element_map"].items()}
        for region_id, member_ids, terminals in members:
            first = pressures[member_ids[0]][1]
            number = active[member_ids[0]]
            claim(region_id, f"pressure region of {element_name[member_ids[0]]}", first.line)
            regions.append({"id": region_id, "member_pipe_ids": member_ids, "pressure_basis": PRESSURE_BASIS,
                            "pressure": {"value": number.value, "unit": unit}, "terminals": terminals,
                            "provenance": f"imported from MBF sha256:{self.sha}, LOADS {first.load} line "
                                          f"{self.loads[first.load]['line']} (Pressure 1)"})
            region_map[region_id] = [element_name[pipe_id] for pipe_id in member_ids]
        if active:
            id_map["region_map"] = region_map
        self.summary["pressure_regions"] = len(regions)
        self.pressure_notes.append(
            f"Pressure 1 is kept in {unit} as exact pressure regions ({len(regions)}) under the contract "
            f"{contract['version']}/{contract['mode']} with shear_model {contract['shear_model']}; regions are {how}. "
            "Pressure acts in the sustained case; the expansion case states an empty region list. Each section's "
            "corrosion allowance is the 0 its PIPE record states in field 4; a section that omits it is refused.")
        return contract, regions

    def pressure_effects(self) -> None:
        """Lists the CAEPIPE pressure effects OPPRES turns on, or leaves unstated, that SWBPIPE does not model."""
        if self.oppres is None:
            self.listed("omitted", "MBF-IMPORT-PRESSURE-EFFECTS-OMITTED", self.loads_line(), "OPPRES",
                        "the file has pressure and no OPPRES section, so it does not state CAEPIPE's Bourdon effect "
                        "or pressure correction for bends (a blank OPPRES line turns both on); SWBPIPE's exact "
                        "pressure route models neither (DEL-04-07 D-3)")
            return
        line, fields, defaults = self.oppres
        on = []
        for index, effect in OPPRES_EFFECTS.items():
            text = fields[index] if len(fields) > index else ""
            if text == "":
                on.append(f"{effect} (unstated)")
                continue
            number = self.real(text, line, f"OPPRES field {index + 1} ({effect})")
            if number is not None and number.exact not in (0, 1):
                self.refuse("MBF-IMPORT-FIELD-UNDOCUMENTED", line, f"OPPRES field {index + 1}",
                            f"OPPRES {effect} is 0 (off) or 1 (on)")
            elif number is not None and number.exact == 1:
                on.append(f"{effect} (on{' by the blank-line default' if defaults else ''})")
        if on:
            self.listed("omitted", "MBF-IMPORT-PRESSURE-EFFECTS-OMITTED", line, "OPPRES",
                        f"OPPRES gives {', '.join(on)}; SWBPIPE's exact pressure route models neither the Bourdon "
                        "effect nor pressure stiffening of bends (DEL-04-07 D-3)")

    def route_checks(self, pressures, ends, arcs, point_of, name_of):
        """The exact route's whole-region checks, as ported: (runs, collinear, failures). `failures(members, start)`
        gives, for members in traversal order from `start`, the straight runs collinear_run refuses, whether the
        equal-bore guard refuses, and the bend-adjacent junctions the tangency check refuses (Design §9.4)."""
        factor = _METRES_PER_UNIT["mm" if self.si else "in"]

        def far(pipe_id: str, node: str) -> str:
            a, b = ends[pipe_id]
            return b if node == a else a

        def runs(members: list[str], nodes: list[str]) -> list[tuple[int, int]]:
            """(first, past-last) member indices of each straight run: between bends or the piece's ends."""
            found, index = [], 0
            while index < len(members):
                if members[index] in arcs:
                    index += 1
                    continue
                end = index
                while end < len(members) and members[end] not in arcs:
                    end += 1
                found.append((index, end))
                index = end
            return found

        def collinear(nodes: list[str]) -> bool:
            return runtime_collinear([tuple(value * factor + 0.0 for value in point_of[name_of[node]])
                                      for node in nodes])

        def bore_m(pipe_id: str) -> float:
            section = self.pipes[pressures[pipe_id][1].section]
            return runtime_bore_radius_m(section["od"].value, section["thk"].value, factor)

        def directions(pipe_id: str, node: str) -> list[list[float]] | None:
            """The member's directions at its start and end traversed from `node` (pressure_runtime.rs
            `traverse_region`): a bend's end tangents, a straight's frame axis at both ends; reversed and negated
            when traversed against the pipe's own from-to order."""
            if pipe_id in arcs:
                pair = self.arc_tangents.get(pipe_id)
            else:
                axis = self.frame_x.get(pipe_id)
                pair = None if axis is None else (axis, axis)
            if pair is None:
                return None
            if node == ends[pipe_id][0]:
                return [list(pair[0]), list(pair[1])]
            return [[-value for value in pair[1]], [-value for value in pair[0]]]

        def junctions(members: list[str], nodes: list[str]) -> list[tuple[str, float | None]]:
            """What the runtime refuses at a region's bends: (place, theta) for each bend-adjacent junction whose
            theta = kink(t_in, t_out) exceeds alpha_tan (PRESSURE_REGION_MITRE_UNSUPPORTED), and (place, None)
            where a bend's tangents, a junction's angle or a member's frame orientation is not formed
            (PRESSURE_REGION_GEOMETRY_INVALID). A straight no longer than 1e-12 m, which no orientation fits, is left
            to the solve's named refusal (§10.1)."""
            found: list[tuple[str, float | None]] = [
                (f"element {self.labels[member]}", None) for member in members
                if (member in arcs and self.arc_tangents.get(member) is None)
                or (self.frame_x.get(member) is None and self.length_m.get(member, 0.0) > AXIS_TOLERANCE)]
            for index in range(len(members) - 1):
                before, after = members[index], members[index + 1]
                if before not in arcs and after not in arcs:
                    continue
                incoming, outgoing = directions(before, nodes[index]), directions(after, nodes[index + 1])
                theta = None if incoming is None or outgoing is None else runtime_kink(incoming[1], outgoing[0])
                if theta is None or not theta <= TANGENCY_TOLERANCE_RAD:
                    found.append((f"node {name_of[nodes[index + 1]]}", theta))
            return found

        def failures(members: list[str], start: str) -> tuple[list[list[str]], bool, list[tuple[str, float | None]]]:
            nodes = [start]
            for member in members:
                nodes.append(far(member, nodes[-1]))
            bad = [members[low:high] for low, high in runs(members, nodes) if not collinear(nodes[low:high + 1])]
            return bad, not runtime_bores_equal([bore_m(member) for member in members]), junctions(members, nodes)

        return runs, collinear, failures

    def check_restored(self, restored, pressures, ends, arcs, point_of, name_of, label_of):
        """Bound regions checked whole as the route checks them, in their declared traversal order (from the first
        terminal), never cut or reordered; None after a refusal (Design §9.4)."""
        _, _, failures = self.route_checks(pressures, ends, arcs, point_of, name_of)
        line = self.loads_line()
        for region_id, members, terminals in restored:
            start, finish = terminals[0]["node_ref"], terminals[1]["node_ref"]
            ordered, remaining, node = [], set(members), start
            while remaining:
                following = [pipe_id for pipe_id in remaining if node in ends[pipe_id]]
                if len(following) != 1:
                    break
                ordered.append(following[0])
                remaining.discard(following[0])
                a, b = ends[following[0]]
                node = b if node == a else a
            location = f"pressure_map region {region_id}"
            if remaining or node != finish:
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, location,
                            f"the sidecar's region {region_id} is not one chain from its first terminal to its second")
                continue
            inner, node = set(), start
            for pipe_id in ordered[:-1]:
                a, b = ends[pipe_id]
                node = b if node == a else a
                inner.add(node)
            branching = sorted(name_of[node] for node in inner
                               if any(node in pair and pipe_id not in members for pipe_id, pair in ends.items()))
            if branching:
                self.refuse("MBF-IMPORT-PRESSURE-INTERNAL-BRANCH", line, location,
                            f"region {region_id} as the sidecar declares it passes through {', '.join(branching)}, "
                            "where another pipe joins; the exact route admits no internal branch "
                            "(PRESSURE_REGION_INTERNAL_BRANCH), and the region is not cut")
                continue
            runs, bores, bends = failures(ordered, start)
            self.refuse_junctions(bends, line, location, f"region {region_id} as the sidecar declares it",
                                  "the region is not cut or reordered")
            for run in runs:
                names = ", ".join(label_of[pipe_id] for pipe_id in run)
                self.refuse("MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR", line, location,
                            f"region {region_id} as the sidecar declares it (from {name_of[start]}): its straight run "
                            f"{names} fails the exact route's check (64 epsilon of the run's span) on the MBF's "
                            "values; the region is not cut or reordered")
            if bores:
                self.refuse("MBF-IMPORT-PRESSURE-BORE-MISMATCH", line, location,
                            f"region {region_id} as the sidecar declares it (from {name_of[start]}): its bores in "
                            "metres differ by more than the exact route's guard (64 epsilon of the radius) on the "
                            "MBF's values; the region is not cut or reordered")
        return None if self.has_errors() else restored

    def form_regions(self, active, pressures, ends, arcs, point_of, name_of, label_of):
        """Unbound regions: [(id, member pipe ids in chain order, two terminals)], or None after a refusal."""
        incident: dict[str, list[str]] = {}
        for pipe_id, (a, b) in ends.items():
            incident.setdefault(a, []).append(pipe_id)
            incident.setdefault(b, []).append(pipe_id)
        order = {pipe_id: index for index, pipe_id in enumerate(ends)}

        def bore(pipe_id: str) -> Fraction:
            section = self.pipes[pressures[pipe_id][1].section]
            return section["od"].exact - 2 * section["thk"].exact

        def far(pipe_id: str, node: str) -> str:
            a, b = ends[pipe_id]
            return b if node == a else a

        def continues(pipe_id: str, node: str) -> str | None:
            if len(incident[node]) != 2:
                return None  # a free end or a tee ends the region
            other = next(item for item in incident[node] if item != pipe_id)
            if other not in active or active[other].exact != active[pipe_id].exact or bore(other) != bore(pipe_id):
                return None
            return other

        factor = _METRES_PER_UNIT["mm" if self.si else "in"]

        def metres(node: str) -> tuple[float, float, float]:
            return tuple(value * factor + 0.0 for value in point_of[name_of[node]])  # type: ignore[return-value]

        def corner(before: str, node: str, after: str) -> bool:
            a = _unit(_sub(metres(node), metres(before)))
            t = _unit(_sub(metres(after), metres(node)))
            theta = 2 * math.atan2(correct_norm3(*_sub(a, t)), correct_norm3(*(a[i] + t[i] for i in range(3))))
            return theta > TANGENCY_TOLERANCE_RAD

        runs, collinear, failures = self.route_checks(pressures, ends, arcs, point_of, name_of)

        def split(members: list[str], start: str) -> list[tuple[list[str], str, str]]:
            """The chain cut only at its corners: a straight run the runtime's collinear_run admits whole, in either
            traversal order, is never cut; one it refuses both ways is cut at its corners."""
            nodes = [start]
            for member in members:
                nodes.append(far(member, nodes[-1]))
            cuts: set[int] = set()
            for low, high in runs(members, nodes):
                stretch = nodes[low:high + 1]
                if not collinear(stretch) and not collinear(stretch[::-1]):
                    cuts.update(k for k in range(low + 1, high) if corner(nodes[k - 1], nodes[k], nodes[k + 1]))
            corners.update(nodes[index] for index in cuts)
            pieces, current, piece_start = [], [], start
            for index, member in enumerate(members):
                if index in cuts:
                    pieces.append((current, piece_start, nodes[index]))
                    current, piece_start = [], nodes[index]
                current.append(member)
            pieces.append((current, piece_start, nodes[-1]))
            return pieces

        chains, seen, corners = [], set(), set()
        for pipe_id in sorted(active, key=order.__getitem__):
            if pipe_id in seen:
                continue
            seen.add(pipe_id)
            sides = []
            for node in ends[pipe_id]:
                walked, current = [], pipe_id
                while (following := continues(current, node)) is not None and following not in seen:
                    walked.append(following)
                    seen.add(following)
                    node, current = far(following, node), following
                sides.append((walked, node))
            (back, start), (ahead, end) = sides
            members = back[::-1] + [pipe_id] + ahead
            if order[members[-1]] < order[members[0]]:
                members, start, end = members[::-1], end, start
            chains.extend(split(members, start))
        # The runtime traverses a region from its first terminal: its straight-run check and its bore reference (the
        # first member) depend on the order. Each region takes its file order, or else the reversed order; it is
        # refused only when neither is admitted.
        admitted, refused = [], False
        for members, start, end in chains:
            forward = failures(members, start)
            if forward == ([], False, []):
                admitted.append((members, start, end))
                continue
            backward = failures(members[::-1], end)
            if backward == ([], False, []):
                admitted.append((members[::-1], end, start))
                continue
            refused = True
            line = pressures[members[0]][1].line
            names = lambda items: ", ".join(label_of[pipe_id] for pipe_id in items)  # noqa: E731
            # kink(t_in, t_out) is the same both ways (negated, swapped directions), so a junction refused in one
            # order is refused in the other; the region is refused whole, never cut at a bend.
            self.refuse_junctions(forward[2], line, f"elements {names(members)}", f"the region {names(members)}",
                                  "a region is never cut")
            backward_runs = {frozenset(run) for run in backward[0]}
            both = [run for run in forward[0] if frozenset(run) in backward_runs]
            for run in both:
                self.refuse("MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR", line, f"elements {names(run)}",
                            f"the straight run {names(run)} carries one pressure but the exact route's check (64 "
                            "epsilon of the run's span) refuses it in both traversal orders, and it has no corner (a "
                            f"direction change above {TANGENCY_TOLERANCE_RAD} rad) to end a region at; a region is "
                            "never cut inside a straight run")
            if forward[1] and backward[1]:
                self.refuse("MBF-IMPORT-PRESSURE-BORE-MISMATCH", line, f"elements {names(members)}",
                            f"the region {names(members)} has equal bores in the file's decimals, but in metres they "
                            "differ by more than the exact route's guard (64 epsilon of the radius) in both traversal "
                            "orders; a region is never cut to separate them")
            if not both and not (forward[1] and backward[1]) and not forward[2]:
                checks = lambda result: ", ".join(  # noqa: E731
                    [f"the straight-run check on {names(run)}" for run in result[0]] + ["the bore guard"] * result[1])
                self.refuse("MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR", line, f"elements {names(members)}",
                            f"the region {names(members)} is admitted in neither traversal order: in file order "
                            f"{checks(forward)} refuses it, reversed {checks(backward)}")
        if refused:
            return None
        chains = admitted
        declared = self.closure_declaration()
        if declared is None:
            return None
        default, provenance, by_node = declared
        terminal_names = {name_of[node] for _, start, end in chains for node in (start, end)}
        unknown = sorted(set(by_node) - terminal_names)
        if unknown:
            self.refuse("MBF-IMPORT-PRESSURE-CLOSURE-INVALID", 0, "pressure_closure.nodes",
                        f"the closure declaration names {', '.join(unknown)}, which is not a pressure-region "
                        "terminal")
            return None
        formed = []
        wall = ("closure transfer deduced from geometry at import, not inferred from the file: the pipe wall is "
                "continuous through this straight-to-straight corner, so it carries the corner's pressure force "
                "pA(t_in - t_out)")
        for members, start, end in chains:
            terminals = [{"node_ref": node, "closure_transfer": by_node.get(name_of[node], default),
                          "provenance": f"closure transfer declared at import: {provenance}"}
                         if name_of[node] in by_node or node not in corners else
                         {"node_ref": node, "closure_transfer": "transfers_to_wall", "provenance": wall}
                         for node in (start, end)]
            formed.append((f"region:mbf-{name_of[start]}-{name_of[end]}", members, terminals))
        return formed

    def refuse_junctions(self, found, line: int, location: str, subject: str, never: str) -> None:
        """What the exact route refuses at a region's bends (pressure_runtime.rs `traverse_region`), by place."""
        for place, theta in found:
            if theta is None:
                self.refuse("MBF-IMPORT-PRESSURE-GEOMETRY-INVALID", line, location,
                            f"{subject}: at {place} the runtime forms no bend tangent, direction change or frame "
                            "orientation on the MBF's values in m (PRESSURE_REGION_GEOMETRY_INVALID); " + never)
            else:
                self.refuse("MBF-IMPORT-PRESSURE-MITRE", line, location,
                            f"{subject}: at {place} the bend's end tangent, as the runtime forms the arc on "
                            f"the MBF's positions and radius in m, turns {theta:.6g} rad from its neighbour, above "
                            f"the tangency tolerance {TANGENCY_TOLERANCE_RAD} rad: the exact route refuses it as a "
                            "mitre (PRESSURE_REGION_MITRE_UNSUPPORTED); " + never)

    def closure_declaration(self) -> tuple[str, str, dict[str, str]] | None:
        declared = self.closure
        line = self.loads_line()
        if declared is None:
            self.refuse("MBF-IMPORT-PRESSURE-CLOSURE-REQUIRED", line, "LOADS Pressure 1",
                        "the file has pressure; each pressure-region terminal needs a closure transfer "
                        "(transfers_to_wall or separately_supported_or_compensated), which the MBF does not state "
                        "and SWBPIPE never infers: pass pressure_closure")
            return None
        nodes = declared.get("nodes", {}) if isinstance(declared, Mapping) else None
        if not (isinstance(declared, Mapping) and set(declared) <= {"closure_transfer", "provenance", "nodes"}
                and declared.get("closure_transfer") in CLOSURE_TRANSFERS
                and isinstance(declared.get("provenance"), str) and declared["provenance"].strip()
                and isinstance(nodes, Mapping)
                and all(isinstance(name, str) and value in CLOSURE_TRANSFERS for name, value in nodes.items())):
            self.refuse("MBF-IMPORT-PRESSURE-CLOSURE-INVALID", line, "pressure_closure",
                        "the closure declaration needs closure_transfer (transfers_to_wall or "
                        "separately_supported_or_compensated), a non-empty provenance and optionally nodes, a map "
                        "from CAEPIPE node names to a closure transfer")
            return None
        return declared["closure_transfer"], declared["provenance"].strip(), dict(nodes)

    def restore_regions(self, stored, active, ends, name_of):
        """Bound regions from the sidecar, checked against the file's Pressure 1 values; None after a refusal."""
        restored, owned = [], set()
        for entry in stored:  # typed by the sidecar parser; checked here against the file
            members, terminals = entry["members"], entry["terminals"]
            valid = (entry["id"] not in {region_id for region_id, _, _ in restored}
                     and all(item in active and item not in owned for item in members)
                     and len(set(members)) == len(members)
                     and len({active[item].exact for item in members}) == 1
                     and all(item["node_ref"] in name_of for item in terminals))
            if not valid:
                break
            owned.update(members)
            restored.append((entry["id"], list(members), [
                {"node_ref": item["node_ref"], "closure_transfer": item["closure_transfer"],
                 "provenance": item["provenance"]} for item in terminals]))
        else:
            if owned == set(active):
                return restored
        self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", self.loads_line(), "pressure_map",
                    "the sidecar's pressure regions do not match the MBF's Pressure 1 values")
        return None

    def displacement_target(self, name: str, dof: str, candidates: list[str]) -> str | None:
        found = {entry["target"].get("support") for entry in self.maps.get("load_map", {}).values()
                 if entry["category"] == "imposed_displacement" and entry.get("caepipe_node") == name
                 and entry["target"].get("dof") == dof}
        found &= set(candidates)
        return found.pop() if len(found) == 1 else None

    def printed(self, recorded: float, number: Num | None, path: str, line: int) -> None:
        """A legacy record of a number the MBF prints. The writer printed it correctly rounded to ten significant
        digits, so the record lies within half a unit in the tenth significant digit of the printed value
        (tenth_digit, §15), or is refused by path."""
        if number is not None and abs(Fraction(recorded) - number.exact) <= tenth_digit(number.exact):
            return
        shown = "nothing" if number is None else number.text
        self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, path,
                    f"the sidecar records {recorded!r}, but the MBF prints {shown}; a writer's record lies within half a "
                    "unit in the tenth significant digit of the printed value")

    def writer_bound(self, number: int) -> list[Fraction]:
        """How far the writer's position of node `number` can lie from the MBF's, per axis: the printing bound
        (tenth_digit) of each printed offset, or the printed coordinates of a run's From row, summed from the root."""
        parent = {element.end: element.start for element in self.elements}
        bound = [Fraction(0)] * 3
        while True:
            exact = self.nodes[number]["exact"]
            above = parent.get(number)
            base = [Fraction(0)] * 3 if above is None else self.nodes[above]["exact"]
            bound = [bound[axis] + tenth_digit(exact[axis] - base[axis]) for axis in range(3)]
            if above is None:
                return bound
            number = above

    def legacy_element(self, swb_id: str, entry: Mapping[str, Any], element: _Element, name: str, order: int,
                       bend: Mapping[str, Any] | None) -> None:
        """What a legacy element entry repeats of the file (§15): its MBF record names, its member order and row, and
        for a bend its radius and factor (printed), its TI (summed printed offsets, within their accumulated
        printing bounds), its angle (from those positions, within the angle the bounds subtend) and row node."""
        where = f"element_map[{swb_id}]"
        conflict = lambda field, recorded, given: self.refuse(  # noqa: E731
            "MBF-IMPORT-SIDECAR-CONFLICT", element.line, f"{where}.{field}",
            f"the sidecar records {field} {recorded!r} for {swb_id}, but the file's element {name} has {given!r}")
        for field, given in (("material", element.material), ("section", element.section), ("load", element.load),
                             ("layout_order", order), ("layout_row", element.row)):
            if entry[field] != given:
                conflict(field, entry[field], given)
        record = entry.get("bend")
        if record is None or bend is None:
            return
        n = element.end
        if record["caepipe_row_node"] != str(n):
            conflict("bend.caepipe_row_node", record["caepipe_row_node"], str(n))
        self.printed(record["radius_mm"], element.radius, f"{where}.bend.radius_mm", element.line)
        self.printed(record["flexibility_factor"], element.factor, f"{where}.bend.flexibility_factor", element.line)
        node, reach = self.nodes[n], self.writer_bound(n)
        slack = 1 + Fraction(1, 2 ** 20)  # binary64 offsets of binary64 positions: far below a printing bound
        for axis in range(3):
            if abs(Fraction(record["tangent_intersection_mm"][axis]) - node["exact"][axis]) > reach[axis] * slack:
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", element.line, f"{where}.bend.tangent_intersection_mm[{axis}]",
                            f"the sidecar records the TI of bend {n} at {record['tangent_intersection_mm'][axis]!r} on "
                            "this axis, beyond the printing bounds of the offsets that place it")
        geometry = self.bend_geometry[n]

        def subtended(a: int, b: int) -> float:
            spread = sum(math.hypot(*(float(value) for value in self.writer_bound(k))) for k in (a, b))
            spread += 4 * 2.0 ** -52 * (_norm(self.positions[a]) + _norm(self.positions[b]))
            length = _norm(_sub(self.positions[b], self.positions[a]))
            return math.asin(min(1.0, spread / length))

        theta = geometry["theta"]
        allowed = (subtended(geometry["start"], n) + subtended(n, geometry["next"])
                   + 64 * 2.0 ** -52 / max(math.sin(theta), 2.0 ** -26))
        if not abs(math.radians(record["angle_deg"]) - theta) <= allowed:
            self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", element.line, f"{where}.bend.angle_deg",
                        f"the sidecar records bend {n}'s angle as {record['angle_deg']!r} deg, but its legs in the MBF "
                        f"meet at {math.degrees(theta)!r} deg, beyond the {math.degrees(allowed)!r} deg their printed "
                        "positions allow")

    def legacy_tables(self) -> None:
        """What legacy material and section entries repeat of the file (§15): each printed number within its printing
        bound, the temperatures in order, and the elements that carry the record, in layout order."""
        for name, entry in self.maps["material_map"].items():
            record, where = self.materials.get(name), f"material_map[{name}]"
            if record is None or record.get("refused"):
                continue
            line = record["line"]
            self.printed(entry["weight_density_kgf_per_m3"], record["density"], f"{where}.weight_density_kgf_per_m3", line)
            self.printed(entry["poisson_ratio"], record["nu"], f"{where}.poisson_ratio", line)
            for modulus, alpha, _ in record["rows"]:
                self.printed(entry["elastic_modulus_n_per_mm2"], modulus, f"{where}.elastic_modulus_n_per_mm2", line)
                self.printed(entry["thermal_expansion_mm_per_mm_per_c"], alpha,
                             f"{where}.thermal_expansion_mm_per_mm_per_c", line)
            temperatures = [temperature for _, _, temperature in record["rows"]]
            if len(entry["temperatures_c"]) != len(temperatures):
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, f"{where}.temperatures_c",
                            f"the sidecar records {len(entry['temperatures_c'])} temperatures for {name}; the MBF "
                            f"prints {len(temperatures)}")
            else:
                for index, (recorded, printed) in enumerate(zip(entry["temperatures_c"], temperatures)):
                    self.printed(recorded, printed, f"{where}.temperatures_c[{index}]", line)
            if entry["elements"] != self.carriers.get(("material", name), []):
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, f"{where}.elements",
                            f"the sidecar's elements of {name} are not the file's, in layout order")
        for name, entry in self.maps["section_map"].items():
            record, where = self.pipes.get(name), f"section_map[{name}]"
            if record is None:
                continue
            self.printed(entry["outside_diameter_mm"], record["od"], f"{where}.outside_diameter_mm", record["line"])
            self.printed(entry["wall_thickness_mm"], record["thk"], f"{where}.wall_thickness_mm", record["line"])
            if entry["elements"] != self.carriers.get(("section", name), []):
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", record["line"], f"{where}.elements",
                            f"the sidecar's elements of {name} are not the file's, in layout order")

    def thermal_direction(self, entries: list[tuple[str, Any]], formed: str, line: int) -> str:
        """The direction a restored thermal primitive keeps (the MBF carries none): the one its sidecar entries
        record, which must name one axis when the MBF sums several (summing across directions is not meaningful),
        else refused by name; an entry without one (legacy) is reported, and the import's global_x stands."""
        recorded = [(key, direction) for key, direction in entries if direction is not None]
        axes = {_DIRECTION_AXIS[direction] for _, direction in recorded}
        if len(axes) > 1:
            self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, f"load_map[{recorded[-1][0]}].direction",
                        f"the sidecar entries the MBF sums into {formed} record different directions ("
                        + ", ".join(f"{key}: {direction}" for key, direction in recorded) + "); a sum across "
                        "directions is not meaningful")
        effective = recorded[0][1] if recorded else _THERMAL_DIRECTION
        why = ("the direction the other entries it sums record" if recorded else
               "the import's token, as no entry records one")
        for key, direction in entries:
            if direction is None:
                self.listed("approximated", "MBF-IMPORT-DIRECTION-UNRECOVERABLE", line, f"load_map[{key}].direction",
                            f"load_map[{key}] records no direction; {formed} takes {effective}, {why}")
        return effective

    def restorable_primitives(self, cases: Mapping[str, str]) -> dict[tuple, tuple[str, int, Any]]:
        """primitive_key -> (sidecar primitive id, count, direction, [(map key, direction)]) for bound imports; a
        count above 1 is a many-to-one mapping, reported by name."""
        if not self.bound:
            return {}
        counts: dict[tuple, list[tuple[str, Any, str]]] = {}
        for key, entry in self.maps.get("load_map", {}).items():
            load_id = key[len(entry["load_case"]) + 1:]  # the parser holds each key to "<case>/<id>"
            counts.setdefault(primitive_key(entry), []).append((load_id, entry.get("direction"), key))
        return {key: (items[0][0], len(items), items[0][1], [(item[2], item[1]) for item in items])
                for key, items in counts.items()}

    def report_unrecoverable(self) -> None:
        if not self.bound:
            return
        self.listed("approximated", "MBF-IMPORT-ID-UNRECOVERABLE", 0, "unexported members",
                    "members the writer does not export (labels, provenance, combinations, lines, unresolved "
                    "assumptions, constraints, equipment interfaces, rule-pack references, model settings) are not "
                    "recoverable from the MBF")

    def add_assumptions(self) -> None:
        mode = ("SI: lengths in mm, forces in N, moments in N*m, E in MPa (N/mm2), temperatures in degC"
                if self.si else
                "English: lengths in in (feet-inch notation evaluated exactly, 1 ft = 12 in), forces in lbf, "
                "moments in lbf*ft, E in psi, temperatures in degF")
        vertical = ("Z vertical, turned into SWBPIPE's Y-vertical axes by the exact rotation (X, Y, Z) -> (X, Z, -Y) "
                    "of every coordinate, offset, force, moment, displacement, rotation and restraint axis; the SI "
                    "writer re-exports the rotated model Y-vertical" if self.z_vertical else "Y vertical")
        self.assumptions.append(f"Profile {IMPORT_PROFILE_ID}: MBF version {self.version}, unit mode {mode}; angles "
                                f"in {'rad' if self.rad else 'deg'}; {vertical}; quantities keep their MBF unit.")
        self.assumptions.append("Node positions are summed from the printed offsets as exact rationals and rounded "
                                "to binary64 once per node.")
        if not self.si:
            self.assumptions.append("English imports keep their US-customary units; the SI writer re-exports them by "
                                    "its exact normalization to the SI profile (O-2, MBF-SI-UNITS-NORMALIZED), "
                                    "with the reference temperature passed in degF.")
        if self.si and any(record.get("density") is not None and record["density"].exact != 0
                           for record in self.materials.values()):
            self.assumptions.append(f"Weight density in kgf/m3 becomes weight per length w = -rho * "
                                    f"{STANDARD_GRAVITY} * A in N/m along global -Y (1 kgf = {STANDARD_GRAVITY} N), "
                                    "A the bare-metal area pi/4 (OD^2 - ID^2); weight acts on bare metal only.")
        elif not self.si:
            self.assumptions.append("Weight density in lbf/in3 becomes weight per length w = -rho * A in lbf/in along "
                                    "global -Y, A the bare-metal area; weight acts on bare metal only.")
        self.assumptions.append("G is derived as E/(2(1+nu)) with E in Pa; materials are temperature-independent "
                                "(identical E and expansion coefficient on every temperature line).")
        self.assumptions.append("Thermal change per element is T1 - T_ref (OPTEMP); zero changes give no primitive. "
                                "Section corrosion, mill tolerance, insulation and lining fields given as zero are "
                                "written as absent" + (", except the corrosion allowance under pressure."
                                                       if self.pressure_notes else "."))
        self.assumptions.extend(self.pressure_notes)
        kinds = {item["kind"] for record in self.node_data.values() for item in record["supports"]}
        flexible = any(record["anchor"] and record["trak"] for record in self.node_data.values())
        unit = STIFFNESS_UNIT[self.si]
        for kind, note in (
                ("guide", "A guide (G) restrains the two global translations lateral to the element whose local axes "
                          "it takes, the preceding element or else the following one (technical reference, Guide), "
                          "which lies along a global axis: rigid, or with STIFF a spring on each."),
                ("stop", "A limit stop LS(0,0) along an axis-aligned DV is stopped both ways at its undeflected "
                         "position (technical reference, Limit Stop): a rigid line stop on that DOF, or with STIFF a "
                         "spring."),
                ("skew", "A skewed restraint K along an axis-aligned DV is a two-way spring on that DOF (technical "
                         "reference, Skewed Restraint).")):
            if kind in kinds:
                self.assumptions.append(note)
        if flexible:
            self.assumptions.append("A flexible anchor (KA with TRAK and ROTK(0,0,0)) is a spring on each translation "
                                    "with a non-zero stiffness; a zero stiffness leaves its DOF free (technical "
                                    "reference, Anchor).")
        if kinds - {"guide"} or flexible or any(item["stiffness"] is not None for record in self.node_data.values()
                                                for item in record["supports"]):
            self.assumptions.append(f"Spring stiffnesses keep their MBF unit, {unit}.")
        if ("gravity", "the earth gravity setting") in self.used_conventions:
            self.used_conventions.discard(("gravity", "the earth gravity setting"))
            self.assumptions.append("OPMISC gives earth gravity (the export's default setting 386.4 with direction "
                                    "(0, -1, 0), or standard gravity in the documented layout's unit); weights are "
                                    "imported as earth weights along -Y.")
        for basis, what in sorted(self.used_conventions):
            source = ("the writer's convention (bound import)" if basis == "bound" else
                      f"profile decision PD-1 for MBF version {self.version} (TBD-17-04-007 stays open)")
            meaning = ("is carried until changed, as M and P are" if "L=" in what else
                       f"belongs to the sustained case {LOAD_CASE_LABELS['sustained']!r}")
            self.assumptions.append(f"{what[0].upper() + what[1:]} {meaning}; basis: {source}.")
        self.assumptions.append("No combinations are created; CAEPIPE forms its own load combinations.")


# ---------------------------------------------------------------------------
# command line (§3, §16.4)
# ---------------------------------------------------------------------------
def declared_invented(path: Path) -> bool:
    """True when `path` has a provenance sidecar declaring it invented and not produced by CAEPIPE."""
    provenance = path.with_name(path.stem + ".provenance.json")
    try:
        declared = json.loads(provenance.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError):
        return False
    return (isinstance(declared, dict) and declared.get("produced_by_caepipe") is False
            and "invented" in str(declared.get("origin", "")))


def inside_git_work_tree(path: Path) -> bool:
    probe = path
    while not probe.exists() and probe != probe.parent:
        probe = probe.parent
    env = {key: value for key, value in os.environ.items() if not key.startswith("GIT_")}
    try:
        completed = subprocess.run(["git", "-C", str(probe), "rev-parse", "--is-inside-work-tree"],
                                   capture_output=True, text=True, env=env, check=False, timeout=30)
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return True  # cannot confirm: treat as inside, so private outputs are never written there
    return completed.returncode == 0


class SidecarJsonInvalid(ValueError):
    """Sidecar JSON text that parse_sidecar_json refuses; `findings` are (JSON path, reason)."""

    def __init__(self, findings: list[tuple[str, str]]) -> None:
        super().__init__("; ".join(f"{path}: {reason}" for path, reason in findings))
        self.findings = findings


def _json_path(path: str, key: str) -> str:
    return f"{path}[{key}]" if path in _KEYED_MAPS else (f"{path}.{key}" if path else key)


def parse_sidecar_json(text: bytes | bytearray | str) -> Any:
    """Sidecar JSON text, parsed strictly and changed in nothing (Design §15): bytes must be strict UTF-8, the text
    RFC 8259 JSON (no NaN, Infinity or -Infinity), and no object may repeat a member name, which json.loads would
    resolve silently to its last value. Raises SidecarJsonInvalid naming each repeated member by JSON path."""
    if isinstance(text, (bytes, bytearray)):
        try:
            text = bytes(text).decode("utf-8")
        except UnicodeDecodeError as error:
            raise SidecarJsonInvalid([("sidecar", f"is not strict UTF-8: the byte at offset {error.start} cannot "
                                                  "be decoded")]) from None
    repeated: dict[int, tuple[dict, list[str]]] = {}  # holds each such object, so its id is not reused

    def pairs(items: list[tuple[str, Any]]) -> dict[str, Any]:
        out: dict[str, Any] = {}
        for key, value in items:
            if key in out:
                repeated.setdefault(id(out), (out, []))[1].append(key)
            out[key] = value
        return out

    def constant(name: str) -> Any:
        raise ValueError(f"{name} is not a JSON value")

    try:
        value = json.loads(text, object_pairs_hook=pairs, parse_constant=constant)
    except ValueError as error:  # json.JSONDecodeError, or a NaN or Infinity
        raise SidecarJsonInvalid([("sidecar", f"is not JSON: {error}")]) from None
    findings: list[tuple[str, str]] = []

    def walk(item: Any, path: str) -> None:
        if type(item) is dict:
            for key in dict.fromkeys(repeated.get(id(item), ({}, []))[1]):
                findings.append((_json_path(path, key), "repeats a member name; JSON does not define which value "
                                 "counts (RFC 8259 §4), and the sidecar must state each once"))
            for key, child in item.items():
                walk(child, _json_path(path, key))
        elif type(item) is list:
            for index, child in enumerate(item):
                walk(child, f"{path}[{index}]")

    walk(value, "")
    if findings:
        raise SidecarJsonInvalid(findings)
    return value


def load_sidecar(path: Path) -> bytes | dict[str, Any]:
    """The sidecar file as the import reads it (Design §15): its bytes, unchanged, which the importer parses strictly
    and a format-3 sidecar's self-digest then verifies as supplied. The one exception is a legacy sidecar (no
    `sidecar_format`) without `mbf_sha256`, the layout of main's earliest pilot runs, which kept the digest in
    `mbf_render.json` beside it: that sidecar is parsed strictly here and takes its companion's digest. A format-2
    or format-3 sidecar never takes it. Raises SidecarJsonInvalid when that companion is not strict JSON."""
    data = path.read_bytes()
    try:
        sidecar = parse_sidecar_json(data)
    except SidecarJsonInvalid:
        return data  # the import refuses it by path
    render = path.with_name("mbf_render.json")
    if type(sidecar) is dict and "sidecar_format" not in sidecar and "mbf_sha256" not in sidecar and render.is_file():
        companion = parse_sidecar_json(render.read_bytes())
        return {**sidecar, "mbf_sha256": companion.get("mbf_sha256") if type(companion) is dict else None}
    return data


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(prog="python3 -m core.handoff.caepipe_mbf.importer")
    parser.add_argument("mbf", type=Path)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--sidecar", type=Path)
    parser.add_argument("--material-provenance", type=Path)
    parser.add_argument("--pressure-closure", type=Path)
    parser.add_argument("--shear-model")
    parser.add_argument("--rebind-by-node-number", action="store_true")
    parser.add_argument("--source-label")
    args = parser.parse_args(argv)
    if inside_git_work_tree(args.out.resolve()) and not declared_invented(args.mbf):
        print("importer: refused: the output folder is inside a Git work tree and the input MBF is not a declared "
              "invented fixture; import outputs are private", file=sys.stderr)
        return 2
    try:
        sidecar = load_sidecar(args.sidecar) if args.sidecar else None
    except SidecarJsonInvalid as error:
        print(f"importer: refused: mbf_render.json beside the sidecar is not strict JSON ({error})", file=sys.stderr)
        return 2
    result = import_caepipe_mbf(
        args.mbf.read_bytes(),
        sidecar=sidecar,
        material_provenance=json.loads(args.material_provenance.read_text(encoding="utf-8"))
        if args.material_provenance else None,
        pressure_closure=json.loads(args.pressure_closure.read_text(encoding="utf-8"))
        if args.pressure_closure else None,
        shear_model=args.shear_model,
        rebind_by_node_number=args.rebind_by_node_number,
        source_label=args.source_label or args.mbf.name,
    )
    args.out.mkdir(parents=True, exist_ok=True)
    (args.out / "import_report.json").write_bytes(result.report_bytes())
    (args.out / "id_map.json").write_bytes(result.id_map_bytes())
    if result.model is not None:
        (args.out / "model.json").write_bytes(result.model_bytes())
    # Operational status only: the outcome and the paths written.
    print(f"importer: {result.status}; outputs in {args.out}")
    return 0 if result.status == "imported" else 1


if __name__ == "__main__":
    raise SystemExit(main())
