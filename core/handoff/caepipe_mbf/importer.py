"""CAEPIPE MBF model importer (DEL-17-04; Design `caepipe-mbf-import.md`).

`import_caepipe_mbf` reads the declared first-tranche subset of a CAEPIPE MBF
(model batch file) into a SWBPIPE model document of the current model-document
version, with an import report and an id map. It is the inverse of the SI
writer's profile (`si_writer.py`), extended only where SWBPIPE represents an
MBF construct exactly. It reads model input only, never analysis results.

Every section, record, key letter, comment, field or value outside the subset
is refused by name (the import produces no model) or listed in the import loss
report; nothing is silently dropped, defaulted or inferred. Quantities keep
their MBF unit. The only conversions are the two exact definitions of Design
§5.3: feet and inches to inches, and kilogram-force to newton.

An import is bound when the writer sidecar's `mbf_sha256` equals the SHA-256 of
the input bytes; only then are the writer's own conventions applied (§1).

From the project root:

    python3 -m core.handoff.caepipe_mbf.importer <in.mbf> --out <dir> [--sidecar <id_map.json>]
        [--material-provenance <declaration.json>] [--rebind-by-node-number] [--source-label <text>]
"""

from __future__ import annotations

import argparse
from dataclasses import dataclass, field
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
    LOAD_CASE_LABELS,
    MATERIAL_ORIGINS,
    MBF_VERSION,
    STANDARD_GRAVITY,
    _BEND_RELATIVE_TOLERANCE,
    current_model_schema_version,
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
_FORCE_DIRECTIONS = ("global_x", "global_y", "global_z")
_THERMAL_DIRECTION = "global_x"  # the token the invented correlation cases use; the solve does not read it
# Exact catalog factors to Pa (core/units: MPa = 1e6 Pa; psi = lbf / in^2), used only to derive G in Pa.
_PA_PER_UNIT = {"MPa": 1_000_000.0, "psi": 4.448_221_615_260_5 / 0.000_645_16}

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
        "AXIAL", "CLD", "CNOD", "CS", "DV", "E", "FREE", *_XYZ("FREE"), "G", "GGAP", "GRA1", "GRA2", "GRB1",
        "GRB2", "GRB3", "GRC1", "GRC2", "HTYP", "K", "KR", "LS", "MLV", "MU", "NOD", "NOZZLE", "NTHK", "ROTK",
        "SHEARY", "SHEARZ", "SPIDER", "SR", "STIFF", "TRAK", "US", "VOD", "VTHK", "VS"},
    "MBF-IMPORT-DISPLACEMENT-UNSUPPORTED": {
        "LD", *{f"DIS{n}" for n in range(2, 11)}, *{f"ROT{n}" for n in range(2, 11)}, "DISD", "ROTD", "SETT",
        "STRO", "SEIS", "SEIS2", "SEIS3", "SERO", "SERO2", "SERO3", *{f"WIND{n}" for n in range(1, 5)},
        *{f"WINR{n}" for n in range(1, 5)}},
    "MBF-IMPORT-LOAD-UNSUPPORTED": {"CWGT", "U", "UNIF", "T", "TEMP", "P", "PRES", "SG", "AMB", "HSG", "HPRES",
                                    "FRE", "PH"},
    "MBF-IMPORT-BEND-UNSUPPORTED": {"IN1", "IN2", "BTHK", "BMAT", "BSIF", "WS"},
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
_DOCUMENTED_COMMENTS = {*_FORCE_NAMES, *_MOMENT_NAMES, *_DISPLACEMENT_NAMES, *_ROTATION_NAMES, *_FIXD_NAMES,
                        *_FIXR_NAMES, "FLC", "FF", "LA", "TAG", "LTAG",
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
    sidecar: Mapping[str, Any] | None = None,
    material_provenance: Mapping[str, Any] | None = None,
    rebind_by_node_number: bool = False,
    source_label: str,
) -> MbfImportResult:
    """Import the declared MBF subset into a SWBPIPE model document, or refuse by name."""

    return _Importer(data, sidecar, material_provenance, rebind_by_node_number, source_label).run()


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


def local_y_reference(start: Iterable[float], end: Iterable[float]) -> tuple[float, float, float]:
    """The documented local y of a straight from `start` to `end` (global Y vertical)."""
    x = _unit(_sub(tuple(end), tuple(start)))
    if math.hypot(x[0], x[2]) <= VERTICAL_SINE:
        return (-1.0, 0.0, 0.0)
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


class _Refused(Exception):
    """Stops the import after a refusal that later stages cannot be read past."""


class _Importer:
    def __init__(self, data: Any, sidecar: Any, provenance: Any, rebind: bool, source_label: Any) -> None:
        self.data = data
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

    def has_errors(self) -> bool:
        return any(item["severity"] == "error" for item in self.diagnostics)

    def result(self, model: dict[str, Any] | None, id_map: dict[str, Any]) -> MbfImportResult:
        refused = self.has_errors()
        source = {"mbf_sha256": self.sha, "mbf_version": self.version, "unit_mode": "SI" if self.si else "English",
                  "angles": "rad" if self.rad else "deg", "line_count": len(self.lines),
                  "source_label": self.source_label,
                  "binding": "bound" if self.bound else ("rebound_unconfirmed" if self.rebound else "unbound")}
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
                    self.refuse("MBF-IMPORT-OPTION-UNSUPPORTED", line, "OPTIONS Z",
                                "vertical axis Z is not read in tranche 1; the writer writes Y vertical")
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
        if not isinstance(self.sidecar, Mapping):
            self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, "sidecar", "the sidecar must be an object")
            return
        names = ("node_map", "element_map", "support_map", "load_case_map", "material_map", "load_map")
        for name in names:
            value = self.sidecar.get(name, {})
            if not isinstance(value, Mapping):
                self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, f"sidecar.{name}", f"sidecar {name} must be an object")
                return
            self.maps[name] = value
        if self.sidecar.get("mbf_sha256") == self.sha:
            self.bound = True
        elif self.rebind:
            self.rebound = True
        else:
            self.diagnostics.append({"code": "MBF-IMPORT-SIDECAR-NOT-APPLIED", "severity": "info", "line": 0,
                                     "location": "sidecar",
                                     "message": "the sidecar's mbf_sha256 does not equal the input's, so the import "
                                                "is unbound and the sidecar is not applied (no rebinding requested)"})
            self.maps = {}
            return
        inverse: dict[str, str] = {}
        for swb_id, name in self.maps["node_map"].items():
            if not isinstance(name, str) or not isinstance(swb_id, str):
                self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, "sidecar.node_map", "node_map maps ids to names")
                return
            if name in inverse:
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", 0, f"sidecar.node_map[{name}]",
                            f"node_map names CAEPIPE node {name} for two SWBPIPE nodes; it must be one-to-one")
                return
            inverse[name] = swb_id
        self.node_ids = inverse

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
            earth = g.exact == EARTH_GRAVITY[0] and tuple(c.exact for c in components) == EARTH_GRAVITY[1]
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
                    "gravity setting and carries weight as explicit primitives along -Y")
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
                record.update(refused=False, od=od, thk=thk)

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
            if p1 is not None and p1.exact != 0:
                self.refuse("MBF-IMPORT-PRESSURE-UNSUPPORTED", line, f"{location} field 3",
                            f"load {name} has a non-zero pressure 1; pressure is tranche 2")
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
                record.update(refused=False, t1=t1)

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
        return self.assemble()

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
        return values

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
                    self.refuse("MBF-IMPORT-FROM-COORDINATES-CONFLICT", row.line, f"{location} {_AXES[index]}",
                                f"F{row.number} {_AXES[index]} does not equal the node's position within the "
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
            "tag": [], "comment_lines": [], "load_lines": []})

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
                    for axis, number in values.items():
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
                for axis, number in values.items():
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
            elif name == "LA":
                number = self.real(value or "", line, where)
                if number is not None and number.exact != 0:
                    self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, where,
                                "an anchor in local axes (LA=1) is not read")
                elif number is not None:
                    record["la"].append(line)
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
                    self.refuse(code, line, where, f"comment {name} is not read in tranche 1")
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

    def check_supports(self) -> None:
        for number, record in self.node_data.items():
            location = f"node {number}"
            if record["la"] and not record["anchor"]:
                self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", record["la"][0], location, "LA is read only with KA")
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

        # materials (§7)
        materials, material_ids = self.assemble_materials(id_map, provenance, claim)
        # pipes, weights and temperatures
        element_lookup: dict[tuple[str, str], tuple[str, dict]] = {}
        for swb_id, entry in maps.get("element_map", {}).items():
            if isinstance(entry, Mapping):
                element_lookup[(str(entry.get("from")), str(entry.get("to")))] = (swb_id, entry)
        pipes, components, unresolved = [], [], []
        weights: list[tuple[str, float, int]] = []
        thermals: list[tuple[str, float, int]] = []
        for element in self.elements:
            start = node_names[element.start][-1]
            end = node_names[element.end][0]
            members = [(start, end, None)]
            if element.ti:
                members.append(node_names[element.end] + (self.bend_geometry[element.end],))
            for a, b, bend in members:
                caepipe_name = f"{a}-{b}"
                swb_id, reversed_, entry = f"pipe:{caepipe_name}", False, {}
                if (self.bound or self.rebound) and (a, b) in element_lookup:
                    swb_id, entry = element_lookup[(a, b)]
                    reversed_ = entry.get("reversed") is True
                    if self.rebound:
                        self.unconfirmed.append(swb_id)
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
                if bend is not None:
                    pipe["y_reference"] = dict(zip("xyz", bend["y_reference"]))
                elif isinstance(restored, Mapping) and all(
                        isinstance(restored.get(axis), (int, float)) and not isinstance(restored.get(axis), bool)
                        and math.isfinite(restored[axis]) for axis in "xyz") and 0 < _norm(
                        tuple(float(restored[axis]) for axis in "xyz")) < math.inf:
                    pipe["y_reference"] = {axis: float(restored[axis]) for axis in "xyz"}
                else:
                    pipe["y_reference"] = dict(zip("xyz", local_y_reference(point_of[a], point_of[b])))
                    self.diagnostics.append({
                        "code": "MBF-IMPORT-ORIENTATION-DEFAULTED", "severity": "info", "line": element.line,
                        "location": swb_id,
                        "message": f"{swb_id} has no y_reference in the MBF; it takes CAEPIPE's documented pipe "
                                   "local y axis (CAEPIPE-LOCAL-AXES, global Y vertical). A circular section's "
                                   "stiffness does not depend on it; only local-axis results do"})
                if bend is not None:
                    components.append(self.bend_component(element, swb_id, node_id[a], length_unit, provenance,
                                                          unresolved, id_map, claim))
                pipes.append(pipe)
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
        if self.elements and all(element.load is None for element in self.elements):
            self.listed("tbd", "MBF-IMPORT-LOADS-ABSENT", self.sections["LAYOUT"][0], "LAYOUT",
                        "the file has no LOADS section; elements are imported with no temperature, pressure or "
                        "contents load, and CAEPIPE's treatment of an element without a load record is not documented")
        self.list_unused(material_ids)
        supports, support_ids = self.assemble_supports(node_id, provenance, id_map, claim)
        load_cases = self.assemble_loads(node_id, support_ids, weights, thermals, length_unit, angle_unit,
                                         provenance, id_map, claim)
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
        return model, id_map

    def bend_component(self, element: _Element, pipe_id: str, near_id: str, length_unit: str, provenance: Any,
                       unresolved: list[dict[str, Any]], id_map: dict[str, Any], claim: Any) -> dict[str, Any]:
        n = element.end
        component_id = f"component:bend-{n}"
        if self.bound or self.rebound:
            entry = self.maps.get("element_map", {}).get(pipe_id)
            bend = entry.get("bend") if isinstance(entry, Mapping) else None
            if isinstance(bend, Mapping) and isinstance(bend.get("component"), str):
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
        if element.factor is not None:
            component["modifiers"] = {"flexibility_factor_user_value": {"value": element.factor.value, "unit": "none"},
                                      "source_reference": f"MBF sha256:{self.sha}, line {element.line}, comment FF"}
        else:
            component["modifiers"] = {"source_reference": f"MBF sha256:{self.sha}, line {element.line}: the MBF "
                                                          "supplies no flexibility factor"}
            unresolved.append({"assumption_id": f"assumption:mbf-bend-{n}-flexibility", "kind": "open_issue",
                               "statement": f"bend {n} has no flexibility factor in the MBF; enter a user factor",
                               "status": "unresolved", "affected_refs": [component_id]})
            self.listed("omitted", "MBF-IMPORT-BEND-FLEXIBILITY-MISSING", element.line, f"bend {n}",
                        f"bend {n} has no FF comment; SWBPIPE never supplies a factor, so the bend is incomplete")
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

    def material_qualifies(self, name: str) -> tuple[str, str, str | None] | None:
        """(origin, source name, redistribution status) from the declaration or a bound sidecar, else None."""
        declared = None
        if isinstance(self.provenance, Mapping):
            declared = self.provenance.get(name, self.provenance.get("*"))
        if isinstance(declared, Mapping):
            origin, source = declared.get("origin"), declared.get("source_name")
            status = declared.get("redistribution_status")
            if origin in MATERIAL_ORIGINS and isinstance(source, str) and source.strip():
                if origin == "user_supplied":
                    return origin, source, "private_only"
                if status in (None, "public_permissive", "private_only"):
                    return origin, source, status
            return None
        if self.bound:
            entry = self.maps.get("material_map", {}).get(name)
            origin = entry.get("origin") if isinstance(entry, Mapping) else None
            if origin in MATERIAL_ORIGINS:
                return origin, "material origin recorded by the SWBPIPE MBF writer sidecar", (
                    "private_only" if origin == "user_supplied" else None)
        return None

    def assemble_materials(self, id_map: dict[str, Any], provenance: Any, claim: Any):
        used = []
        for element in self.elements:
            if element.material not in used:
                used.append(element.material)
        materials, material_ids, merged = [], {}, {}
        e_unit, alpha_unit = ("MPa", "1/degC") if self.si else ("psi", "1/degF")
        for name in used:
            record = self.materials[name]
            if record.get("refused"):
                raise _Refused
            qualified = self.material_qualifies(name)
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
                if isinstance(entry, Mapping) and isinstance(entry.get("swbpipe_material"), str):
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
            origin, source, status = qualified
            note = (f"G derived from E and nu (isotropic): G = E/(2(1+nu)) with E in Pa, from MBF MATERIAL {name} "
                    f"E = {modulus.text} {e_unit} and nu = {nu.text}")
            provenance_record: dict[str, Any] = {"source_name": source, "source_location": f"sha256:{self.sha}",
                                                 "contributor_certification": origin, "verification_note": note}
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
        sidecar_by_node: dict[str, list[tuple[str, Mapping[str, Any]]]] = {}
        for swb_id, entry in self.maps.get("support_map", {}).items():
            if isinstance(entry, Mapping):
                sidecar_by_node.setdefault(str(entry.get("node")), []).append((swb_id, entry))
        matched_sidecar: set[str] = set()
        for number in self.node_order:
            record = self.node_data.get(number)
            if record is None:
                continue
            name = str(number)
            imported: list[tuple[str, list[str], str | None, int]] = []
            if record["anchor"]:
                imported.append((f"support:{name}", list(DOFS), "anchor", record["anchor"][0]))
            dofs = [DOFS[axis] for axis in sorted(record["fixd"])] + [DOFS[3 + axis] for axis in sorted(record["fixr"])]
            if dofs:
                line = min(list(record["fixd"].values()) + list(record["fixr"].values()))
                imported.append((f"support:{name}-restraint", dofs, None, line))
            tags = record["tag"]
            if not imported:
                for _, line in tags:
                    self.listed("omitted", "MBF-IMPORT-TAG-WITHOUT-SUPPORT", line, f"node {name} TAG",
                                f"TAG at node {name}, which has no support, is not imported")
                continue
            line = imported[0][3]
            restored = sidecar_by_node.get(name, []) if (self.bound or self.rebound) else []
            ids = []
            if restored:
                wanted = set().union(*(set(dofs) for _, dofs, _, _ in imported))
                given = set().union(*(set(entry.get("restrained") or []) for _, entry in restored))
                if wanted != given:
                    self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", line, f"node {name}",
                                f"the sidecar's supports at node {name} restrain other DOFs than the MBF")
                    continue
                entries = []
                for swb_id, entry in restored:
                    family = entry.get("family") if "family" in entry else None
                    if "family" not in entry:
                        self.listed("approximated", "MBF-IMPORT-ID-UNRECOVERABLE", line, f"support_map[{swb_id}]",
                                    f"the sidecar entry for {swb_id} carries no family, which is not recoverable")
                    entries.append((swb_id, [dof for dof in DOFS if dof in (entry.get("restrained") or [])], family))
                    matched_sidecar.add(swb_id)
                    if self.rebound:
                        self.unconfirmed.append(swb_id)
            else:
                entries = [(swb_id, dofs, family) for swb_id, dofs, family, _ in imported]
            for swb_id, dofs, family in entries:
                claim(swb_id, f"support at node {name}", line)
                support: dict[str, Any] = {"id": swb_id, "label": tags[0][0] if tags else
                                           f"{'anchor' if family == 'anchor' else 'restraint'} at {name}",
                                           "node": node_id[name], "restraints": dofs,
                                           "provenance": provenance(line)}
                if family is not None:
                    support["family"] = family
                supports.append(support)
                ids.append((swb_id, dofs))
            support_ids[number] = ids
            id_map["support_map"][name] = [swb_id for swb_id, _ in ids]
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
                       id_map, claim):
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
                                                    "dimension": dimension, "line": items[0][1], "axis": axis})
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
                                                    "dimension": dimension, "line": items[0][1], "axis": dof})
        restorable = self.restorable_primitives(cases)
        load_cases = []
        for kind, label_key, case_kind in (("sustained", "sustained", "sustained"),
                                           ("expansion", "expansion", "thermal")):
            if not primitives[kind]:
                continue
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
                key = (case_id, item["category"], target_id, item["axis"])
                match = restorable.get(key) or restorable.get((case_id, item["category"], target_id, "*"))
                if self.bound and match is not None and match[1] == 1:
                    load_id = match[0]
                    if item["category"] == "thermal" and isinstance(match[2], str):
                        direction = match[2]
                elif self.bound:
                    self.listed("approximated", "MBF-IMPORT-ID-UNRECOVERABLE", item["line"], formed,
                                f"no single sidecar primitive matches {formed}; it keeps its formed id")
                id_map["load_map"][load_id] = {"case": case_id, "category": item["category"], "target": target_id}
                loads.append({"id": load_id, "category": item["category"], "target": target, "direction": direction,
                              "magnitude": {"value": item["magnitude"], "unit": item["unit"]},
                              "dimension": item["dimension"], "provenance": provenance(item["line"])})
            load_cases.append({"id": case_id, "label": LOAD_CASE_LABELS[label_key], "kind": case_kind,
                               "status": "draft", "primitive_loads": loads, "provenance":
                               f"imported from MBF sha256:{self.sha}; CAEPIPE load case {LOAD_CASE_LABELS[label_key]}"})
        return load_cases

    def displacement_target(self, name: str, dof: str, candidates: list[str]) -> str | None:
        found = {entry.get("target", {}).get("support") for entry in self.maps.get("load_map", {}).values()
                 if isinstance(entry, Mapping) and entry.get("category") == "imposed_displacement"
                 and entry.get("caepipe_node") == name and entry.get("target", {}).get("dof") == dof}
        found &= set(candidates)
        return found.pop() if len(found) == 1 else None

    def restorable_primitives(self, cases: Mapping[str, str]) -> dict[tuple, tuple[str, int, Any]]:
        """(case, category, target, axis) -> (sidecar primitive id, count, direction) for bound imports."""
        if not self.bound:
            return {}
        counts: dict[tuple, list[tuple[str, Any]]] = {}
        for key, entry in self.maps.get("load_map", {}).items():
            if not isinstance(entry, Mapping):
                continue
            case_id = entry.get("load_case")
            target = entry.get("target") or {}
            target_id = target.get("node") or target.get("pipe") or target.get("support")
            category = entry.get("category")
            direction = entry.get("direction")
            if category in ("concentrated_force", "concentrated_moment"):
                if direction is None:
                    axis = "*"
                elif category == "concentrated_force":
                    axis = {"global_x": 0, "global_y": 1, "global_z": 2}.get(str(direction).lower())
                else:
                    token = str(direction).upper().replace("ROTATION_", "R")
                    axis = {"RX": 0, "RY": 1, "RZ": 2}.get(token)
            elif category == "imposed_displacement":
                axis = target.get("dof")
                axis = None if axis is None else str(axis).upper()
            else:
                axis = None
            load_id = str(key)[len(str(case_id)) + 1:] if str(key).startswith(f"{case_id}/") else str(key)
            counts.setdefault((case_id, category, target_id, axis), []).append((load_id, direction))
        return {key: (items[0][0], len(items), items[0][1]) for key, items in counts.items()}

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
        self.assumptions.append(f"Profile {IMPORT_PROFILE_ID}: MBF version {self.version}, unit mode {mode}; angles "
                                f"in {'rad' if self.rad else 'deg'}; Y vertical; quantities keep their MBF unit.")
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
                                "written as absent.")
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


def load_sidecar(path: Path) -> dict[str, Any]:
    sidecar = json.loads(path.read_text(encoding="utf-8"))
    if isinstance(sidecar, dict) and "mbf_sha256" not in sidecar:
        render = path.with_name("mbf_render.json")
        if render.is_file():
            sidecar["mbf_sha256"] = json.loads(render.read_text(encoding="utf-8")).get("mbf_sha256")
    return sidecar


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(prog="python3 -m core.handoff.caepipe_mbf.importer")
    parser.add_argument("mbf", type=Path)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--sidecar", type=Path)
    parser.add_argument("--material-provenance", type=Path)
    parser.add_argument("--rebind-by-node-number", action="store_true")
    parser.add_argument("--source-label")
    args = parser.parse_args(argv)
    if inside_git_work_tree(args.out.resolve()) and not declared_invented(args.mbf):
        print("importer: refused: the output folder is inside a Git work tree and the input MBF is not a declared "
              "invented fixture; import outputs are private", file=sys.stderr)
        return 2
    result = import_caepipe_mbf(
        args.mbf.read_bytes(),
        sidecar=load_sidecar(args.sidecar) if args.sidecar else None,
        material_provenance=json.loads(args.material_provenance.read_text(encoding="utf-8"))
        if args.material_provenance else None,
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
