"""Parser for the CSV results file written by CAEPIPE's Print > To File command (DEL-17-05).

The recognized section layout follows the public documentation of the
print-to-file output and private notes; the public fixture that exercises it is
invented. Each recognized section's
column-header and unit rows are verified exactly before any value is read, so no
unit is ever assumed: every unit label must be one the layout prints for that
column's quantity (SI or English; angles in deg or rad), one system and one angle
unit per file. English and radian values are normalized to the parser's output
units (mm, deg, N, N*m) with the unit catalog's exact definitions, and each
section records the label every value was printed in. Unknown sections are skipped
with an info diagnostic.

Parsed values are correlation evidence only. Nothing here asserts agreement or
compatibility with CAEPIPE, solver validation, code compliance or professional
reliance.
"""

from __future__ import annotations

import csv
from fractions import Fraction
import math
from pathlib import Path
import re
from typing import Any


PARSER_VERSION = "caepipe-print-csv-4"
DOCUMENT_KIND = "openpipestress.caepipe_print_csv.parsed"
PAIRING_BASIS = "row_order_from_to_pairs"
BINDING_BASIS = "writer_id_map_node_pairs"
_BASIS = "public CAEPIPE-EXPORT-DATA / Print To File page + invented fixture"
_BLANK_NOTE = "A blank numeric field inside a recognized data row is read as 0.0."

# The target tool's print-to-file layout: units, the version header, stress labels and the sections below.
VERSION_PREFIX = "Version "
UNITS = {"length": "mm", "rotation": "deg", "force": "N", "moment": "N*m"}

# Unit labels the print layout uses, per quantity: the SI labels of the documented layout and the English ones,
# and radians where the model uses them (observed format behaviour: an English model prints "(inch)", "(lb)",
# "(ft-lb)" and "(psi)"; a model with the radian option prints "(rad)" in the rotation columns). Each label's
# system: a file is printed in one system and one angle unit. Stress labels are checked, never read.
_QUANTITY_LABELS = {
    "length": {"mm": "SI", "inch": "English"},
    "rotation": {"deg": None, "rad": None},
    "force": {"N": "SI", "lb": "English"},
    "moment": {"Nm": "SI", "ft-lb": "English"},
    "stress": {"MPa": "SI", "psi": "English"},
}
# The unit catalog's exact definitions (core/units: INCH_IN_METERS 0.0254, FOOT_IN_METERS 0.3048,
# POUND_FORCE_IN_NEWTONS 4.4482216152605, degree = pi/180 rad), combined exactly and rounded once to binary64.
_INCH_IN_METRES = Fraction("0.0254")
_FOOT_IN_METRES = Fraction("0.3048")
_POUND_FORCE_IN_NEWTONS = Fraction("4.4482216152605")
_TO_OUTPUT = {  # label -> factor to the output unit (None: already the output unit)
    "mm": None, "deg": None, "N": None, "Nm": None,
    "inch": float(_INCH_IN_METRES / Fraction("0.001")),
    "lb": float(_POUND_FORCE_IN_NEWTONS),
    "ft-lb": float(_POUND_FORCE_IN_NEWTONS * _FOOT_IN_METRES),
}
_RADIANS_PER_DEGREE = math.pi / 180.0  # the catalog's degree definition; radians are divided by it
# The SI label in the documented layout -> the quantity of that column.
_SI_SLOTS = {"N": "force", "Nm": "moment", "mm": "length", "deg": "rotation", "MPa": "stress"}
_KEY_QUANTITY = {**{key: "force" for key in ("fx", "fy", "fz", "axial", "shear_y", "shear_z")},
                 **{key: "moment" for key in ("mx", "my", "mz", "torque", "moment_in_plane", "moment_out_of_plane")},
                 **{key: "length" for key in ("ux", "uy", "uz")}, **{key: "rotation" for key in ("rx", "ry", "rz")}}
_SLOT = re.compile(r"\((N|Nm|mm|deg|MPa)\)")

_FORCE_KEYS = ("fx", "fy", "fz", "mx", "my", "mz")
_LOCAL_KEYS = ("axial", "shear_y", "shear_z", "torque", "moment_in_plane", "moment_out_of_plane")
_DISPLACEMENT_KEYS = ("ux", "uy", "uz", "rx", "ry", "rz")
_NODE = re.compile(r"^[0-9]+[A-Za-z0-9]*$")
_STRESS_LABELS = ("", "SL", "SE", "Sopr")


# The local element-force section in a second layout (observed format behaviour, with an ASME B31.1 (1980) code):
# columns fx..mz, one SIF column and the case's stress column, as in the other layout. Which bending moment is in
# plane is not documented, so the section is recognized and skipped (info), never read by guess.
_LOCAL_AXES_HEADERS = [
    (("", "fx", "fy", "fz", "mx", "my", "mz", "SIF") + ((label,) if label else ()),
     ("Node", "(N)", "(N)", "(N)", "(Nm)", "(Nm)", "(Nm)") + (("", "(MPa)") if label else ()))
    for label in _STRESS_LABELS]


def _local_headers(label: str) -> tuple[tuple[str, ...], ...]:
    return (
        ("", "", "", "", "", "In plane", "Out of plane", "In plane", "Out of plane", "Torsion"),
        ("", "Axial", "y shear", "z shear", "Torque", "Bend.Moment", "Bend.Moment", "SIF", "SIF", "SIF")
        + ((label,) if label else ()),
        ("Node", "(N)", "(N)", "(N)", "(Nm)", "(Nm)", "(Nm)") + (("", "", "", "(MPa)") if label else ()),
    )


# kind -> title pattern, accepted header-row variants, value columns (index, key), output member.
_SPECS: dict[str, dict[str, Any]] = {
    "min_max_displacements": {
        "pattern": re.compile(r"^Min/Max Displacements:\s*(?P<case>.+)$"),
    },
    "anchor_loads_global": {
        "pattern": re.compile(r"^Loads on Anchors in Global Coordinates:\s*(?P<case>.+)$"),
        "headers": [(("Node", "Tag", "FX (N)", "FY (N)", "FZ (N)", "MX (Nm)", "MY (Nm)", "MZ (Nm)"),)],
        "columns": list(zip(range(2, 8), _FORCE_KEYS)),
    },
    "anchor_loads_local": {
        "pattern": re.compile(r"^Loads on Anchors in Local Coordinates:\s*(?P<case>.+)$"),
        "headers": [(("Node", "Tag", "fx (N)", "fy (N)", "fz (N)", "mx (Nm)", "my (Nm)", "mz (Nm)"),)],
        "columns": list(zip(range(2, 8), _FORCE_KEYS)),
    },
    "element_forces_global": {
        "pattern": re.compile(r"^Element forces in global coordinates:\s*(?P<case>.+)$"),
        "headers": [(("Node", "FX (N)", "FY (N)", "FZ (N)", "MX(Nm)", "MY(Nm)", "MZ(Nm)"),)],
        "columns": list(zip(range(1, 7), _FORCE_KEYS)),
    },
    "restraint_loads_global": {
        "pattern": re.compile(r"^(?P<case>.+?):\s*Loads on restraints$"),
        "headers": [(("Node", "Tag", "FX (N)", "FY (N)", "FZ (N)"),)],
        "columns": list(zip(range(2, 5), _FORCE_KEYS[:3])),
    },
    "element_forces_local": {
        "pattern": re.compile(r"^(?P<case>.+?):\s*Pipe element forces \(local\)$"),
        "headers": [_local_headers(label) for label in _STRESS_LABELS],
        "columns": list(zip(range(1, 7), _LOCAL_KEYS)),
    },
    "displacements": {
        "pattern": re.compile(r"^(?P<case>.+?):\s*Displacements$"),
        "headers": [(
            ("", "", "", "Displacements (global)"),
            ("", "X", "Y", "Z", "XX", "YY", "ZZ"),
            ("Node", "(mm)", "(mm)", "(mm)", "(deg)", "(deg)", "(deg)"),
        )],
        "columns": list(zip(range(1, 7), _DISPLACEMENT_KEYS)),
    },
}


def _coverage() -> list[dict[str, Any]]:
    records = [{
        "section_pattern": "Options,",
        "basis": _BASIS,
        "supported": True,
        "unit_row": None,
        "notes": "Option lines are kept as strings up to the next title line; they are not interpreted.",
    }]
    for kind, spec in _SPECS.items():
        if kind == "min_max_displacements":
            records.append({
                "section_pattern": spec["pattern"].pattern,
                "basis": _BASIS,
                "supported": False,
                "unit_row": None,
                "notes": "Skipped explicitly: its fixed-width fields can run together, and the values repeat the "
                         "Displacements section.",
            })
            continue
        variants = spec["headers"]
        unit_row = ",".join(variants[0][-1])
        notes = f"Output member {kind}. Header and unit rows must match exactly after stripping spaces, each unit " \
                "label being one the layout prints for that quantity (SI or English; deg or rad), one system and " \
                "one angle unit per file; otherwise the section is not parsed. English and radian values are " \
                f"normalized to mm, deg, N and N*m by the unit catalog's exact definitions. {_BLANK_NOTE}"
        if kind == "element_forces_local":
            unit_row = " | ".join(",".join(variant[-1]) for variant in variants)
            notes += (" Accepted stress-column variants: none (Empty Weight), SL, SE, Sopr. Columns read: axial, "
                      "y shear, z shear, torque, in-plane and out-of-plane bending moment; SIF and stress columns are "
                      f"not read. Rows pair as (from node, to node) in layout order ({PAIRING_BASIS}). The fx..mz "
                      "layout with one SIF column is skipped with an info diagnostic: its bending-moment planes are "
                      "not documented.")
        if kind == "element_forces_global":
            notes += f" Rows pair as (from node, to node) in layout order ({PAIRING_BASIS})."
        if kind.startswith("element_forces"):
            notes += (" bind_element_rows binds each pair to a SWBPIPE member through the writer's element_map by "
                      f"its two node labels, not by position ({BINDING_BASIS}); an unbound row is reported, never "
                      "dropped.")
        if kind == "restraint_loads_global":
            notes += " Several rows at one node are summed into that node's entry (info diagnostic)."
        records.append({
            "section_pattern": spec["pattern"].pattern,
            "basis": _BASIS,
            "supported": True,
            "unit_row": unit_row,
            "notes": notes,
        })
    records.append({
        "section_pattern": "any other single-field title line",
        "basis": _BASIS,
        "supported": False,
        "unit_row": None,
        "notes": "Recorded in sections_skipped with an info diagnostic; its rows are not read.",
    })
    return records


PRINT_CSV_COVERAGE = _coverage()


def read_caepipe_print_csv(path: str | Path) -> dict[str, Any]:
    """Read and parse a Print > To File CSV, decoding it as cp1252 (latin-1 if cp1252 cannot decode it)."""

    data = Path(path).read_bytes()
    try:
        return parse_caepipe_print_csv(data.decode("cp1252"))
    except UnicodeDecodeError as error:
        result = parse_caepipe_print_csv(data.decode("latin-1"))
        result["diagnostics"].insert(0, {
            "code": "CAEPIPE-PRINT-DECODE-FALLBACK",
            "severity": "warning",
            "section": None,
            "line": None,
            "message": f"cp1252 could not decode the file ({error.reason} at byte {error.start}); decoded as latin-1",
        })
        return result


def parse_caepipe_print_csv(text: str) -> dict[str, Any]:
    """Parse Print > To File CSV text (already decoded) into per-load-case result tables."""

    return _Parser(text).run()


class _Parser:
    def __init__(self, text: str) -> None:
        if text.startswith("\ufeff"):
            text = text[1:]
        raw = text.replace("\r\n", "\n").replace("\r", "\n").split("\n")
        # `fields` keeps trailing empty cells (blank = 0.0 in data rows); `rows` drops them for layout matching.
        self.fields = [tuple(field.strip() for field in next(csv.reader([line]), [])) for line in raw]
        self.rows = [_normalize(list(fields)) for fields in self.fields]
        self.diagnostics: list[dict[str, Any]] = []
        self.sections_parsed: list[dict[str, Any]] = []
        self.sections_skipped: list[dict[str, Any]] = []
        self.load_cases: dict[str, dict[str, Any]] = {}
        self.options: list[str] = []
        self.header = {"product_label": None, "version": None, "model_name": None, "title": None, "analyzed": None}
        self.system: str | None = None  # "SI" or "English", fixed by the first section read
        self.angle: str | None = None   # "deg" or "rad", likewise
        self.section_units: dict[str, str] = {}

    def diag(self, code: str, severity: str, section: str | None, line: int | None, message: str) -> None:
        self.diagnostics.append({"code": code, "severity": severity, "section": section, "line": line, "message": message})

    def run(self) -> dict[str, Any]:
        index = self.read_header()
        while index < len(self.rows):
            row = self.rows[index]
            if not _is_title(row):
                index += 1
                continue
            title = row[0]
            if title == "Options":
                index = self.read_options(index + 1)
                continue
            kind, case = _match(title)
            if kind == "min_max_displacements":
                self.skip(title, index, "min_max_redundant", "CAEPIPE-PRINT-SECTION-SKIPPED",
                          "Min/Max Displacements block skipped: fixed-width fields can run together and the values "
                          "repeat the Displacements section")
                index += 1
            elif kind is None:
                self.skip(title, index, "unknown_section", "CAEPIPE-PRINT-SECTION-UNKNOWN",
                          f"section {title!r} is not recognized by {PARSER_VERSION}; its rows are not read")
                index += 1
            else:
                index = self.read_section(kind, case, title, index)
        return {
            "document_kind": DOCUMENT_KIND,
            "parser_version": PARSER_VERSION,
            "header": self.header,
            "options": self.options,
            "units": dict(UNITS),
            "source_units": {"system": self.system, "rotation": self.angle},
            "load_cases": self.load_cases,
            "pairing_basis": PAIRING_BASIS,
            "sections_parsed": self.sections_parsed,
            "sections_skipped": self.sections_skipped,
            "diagnostics": self.diagnostics,
        }

    def units_refused(self, matched: dict[int, tuple[str, str]], title: str, title_line: int) -> str | None:
        """Refuses (error, section skipped) a section whose unit labels are not labels of the print layout, or mix
        systems or angle units within it or against the sections read before it; None when they are valid."""
        unknown = [(quantity, label) for quantity, label in matched.values() if label not in _QUANTITY_LABELS[quantity]]
        if unknown:
            quantity, label = unknown[0]
            self.diag("CAEPIPE-PRINT-COLUMN-HEADER-MISMATCH", "error", title, title_line,
                      f"unit label {label!r} in {title!r} is not a {quantity} label of the print layout "
                      f"({', '.join(_QUANTITY_LABELS[quantity])}); the section is not parsed")
            self.sections_skipped.append({"title": title, "line": title_line, "reason": "header_mismatch"})
            return "header_mismatch"
        systems = {_QUANTITY_LABELS[q][label] for q, label in matched.values()} - {None}
        angles = {label for q, label in matched.values() if q == "rotation"}
        if systems:
            systems |= {self.system} - {None}
        if angles and self.angle is not None:
            angles |= {self.angle}
        if len(systems) > 1 or len(angles) > 1:
            self.diag("CAEPIPE-PRINT-UNITS-MIXED", "error", title, title_line,
                      f"{title!r} prints units of more than one system or angle unit "
                      f"({', '.join(sorted(systems | angles))}) within the section or against the sections read "
                      "before it; the section is not parsed")
            self.sections_skipped.append({"title": title, "line": title_line, "reason": "units_mixed"})
            return "units_mixed"
        return None

    def skip(self, title: str, index: int, reason: str, code: str, message: str) -> None:
        self.sections_skipped.append({"title": title, "line": index + 1, "reason": reason})
        self.diag(code, "info", title, index + 1, message)

    def read_header(self) -> int:
        lines = [index for index, row in enumerate(self.rows) if row][:2]
        if len(lines) == 2:
            first, second = self.rows[lines[0]], self.rows[lines[1]]
            if second[0].startswith(VERSION_PREFIX) and len(first) >= 1:
                self.header["product_label"] = first[0]
                self.header["model_name"] = ",".join(first[1:]) if len(first) > 1 else None
                self.header["version"] = second[0][len(VERSION_PREFIX):].strip()
                if len(second) >= 3:
                    self.header["title"] = ",".join(second[1:-1])
                    self.header["analyzed"] = second[-1]
                elif len(second) == 2:
                    self.header["title"] = second[1]
                return lines[1] + 1
        self.diag("CAEPIPE-PRINT-HEADER-UNRECOGNIZED", "warning", None, 1,
                  "the first two lines are not '<product>,<model>,' and 'Version <n>,<title>,<timestamp>,'")
        return 0

    def read_options(self, index: int) -> int:
        while index < len(self.rows):
            row = self.rows[index]
            if row and _is_title(row):
                break
            if row:
                self.options.append(",".join(row))
            index += 1
        return index

    def read_section(self, kind: str, case: str, title: str, index: int) -> int:
        spec = _SPECS[kind]
        title_line = index + 1
        width = len(spec["headers"][0])
        found: list[tuple[str, ...]] = []
        cursor = index + 1
        while cursor < len(self.rows) and len(found) < width:
            if self.rows[cursor]:
                found.append(self.rows[cursor])
            cursor += 1
        matched = _match_headers(spec["headers"], tuple(found))
        if matched is None and kind == "element_forces_local":
            # The fx..mz layout is skipped only when its unit labels are valid and consistent; otherwise refused.
            local = _match_headers(_LOCAL_AXES_HEADERS, tuple(found[:2]))
            if local is not None and self.units_refused(local, title, title_line) is None:
                self.skip(title, index, "local_axes_layout", "CAEPIPE-PRINT-SECTION-SKIPPED",
                          f"{title!r} is printed in the fx..mz layout with one SIF column, whose bending-moment "
                          "planes are not documented; its rows are not read")
                return index + 1
            if local is not None:
                return index + 1
        if matched is None:
            expected = " / ".join(",".join(row) for row in spec["headers"][0])
            got = " / ".join(",".join(row) for row in found)
            self.diag("CAEPIPE-PRINT-COLUMN-HEADER-MISMATCH", "error", title, title_line,
                      f"column-header and unit rows of {title!r} do not match the recognized layout "
                      f"(expected {expected!r}, got {got!r}); the section is not parsed")
            self.sections_skipped.append({"title": title, "line": title_line, "reason": "header_mismatch"})
            return index + 1
        refused = self.units_refused(matched, title, title_line)
        if refused is not None:
            return index + 1
        systems = {_QUANTITY_LABELS[q][label] for q, label in matched.values()} - {None}
        angles = {label for q, label in matched.values() if q == "rotation"}
        self.system = next(iter(systems), self.system)
        self.angle = next(iter(angles), self.angle)
        self.section_units = {key: matched[position][1] for position, key in spec["columns"]}
        header_width = max(len(row) for row in found)
        data: list[tuple[int, str, dict[str, float] | None]] = []
        while cursor < len(self.rows):
            row = self.rows[cursor]
            if not row:
                cursor += 1
                continue
            if not _NODE.match(row[0]):
                break
            values = self.read_row(self.fields[cursor], spec["columns"], header_width, title, cursor + 1)
            data.append((cursor + 1, row[0], values))
            cursor += 1
        self.store(kind, case, title, title_line, data)
        return cursor

    def read_row(self, row: tuple[str, ...], columns: list[tuple[int, str]], width: int, title: str,
                 line: int) -> dict[str, float] | None:
        needed = max(position for position, _ in columns) + 1
        if len(row) < needed:
            self.diag("CAEPIPE-PRINT-ROW-SHORT", "error", title, line,
                      f"data row has {len(row)} fields; at least {needed} are needed")
            return None
        if any(row[width:]):
            self.diag("CAEPIPE-PRINT-ROW-EXTRA-FIELDS", "warning", title, line,
                      f"data row has non-empty fields beyond the {width} header columns; they are not read")
        values: dict[str, float] = {}
        for position, key in columns:
            text = row[position]
            if text == "":
                values[key] = 0.0
                continue
            try:
                value = float(text)
            except ValueError:
                value = math.nan
            if not math.isfinite(value):
                self.diag("CAEPIPE-PRINT-NUMBER-INVALID", "error", title, line,
                          f"field {position + 1} ({key}) {text!r} is not a finite number; the row is not read")
                return None
            converted = _to_output(value, self.section_units.get(key))
            if not math.isfinite(converted):
                self.diag("CAEPIPE-PRINT-VALUE-UNREPRESENTABLE", "error", title, line,
                          f"field {position + 1} ({key}) {text!r} in {self.section_units.get(key)!r} is not finite in "
                          "the output unit; the row is not read")
                return None
            values[key] = converted
        return values

    def case_tables(self, case: str) -> dict[str, Any]:
        return self.load_cases.setdefault(case, {
            "displacements": {},
            "anchor_loads_global": {},
            "anchor_loads_local": {},
            "restraint_loads_global": {},
            "element_forces_local": [],
            "element_forces_global": [],
        })

    def store(self, kind: str, case: str, title: str, line: int,
              data: list[tuple[int, str, dict[str, float] | None]]) -> None:
        if any(item["kind"] == kind and item["load_case"] == case for item in self.sections_parsed):
            self.diag("CAEPIPE-PRINT-SECTION-DUPLICATE", "error", title, line,
                      f"a second {kind} section for {case!r}; it is not parsed")
            self.sections_skipped.append({"title": title, "line": line, "reason": "duplicate_section"})
            return
        if kind.startswith("element_forces"):
            if any(values is None for _, _, values in data) or len(data) % 2:
                self.diag("CAEPIPE-PRINT-ELEMENT-PAIRING", "error", title, line,
                          f"{len(data)} element rows (with any unreadable row) cannot be paired as (from, to) rows; "
                          "the section is not parsed")
                self.sections_skipped.append({"title": title, "line": line, "reason": "element_pairing_error"})
                return
            elements = [
                {
                    "element_index": number,
                    "node_from": data[2 * number][1],
                    "node_to": data[2 * number + 1][1],
                    "from": data[2 * number][2],
                    "to": data[2 * number + 1][2],
                }
                for number in range(len(data) // 2)
            ]
            self.case_tables(case)[kind] = elements
        else:
            table = self.case_tables(case)[kind]
            for row_line, node, values in data:
                if values is None:
                    continue
                if node in table:
                    if kind == "restraint_loads_global":
                        table[node] = {key: table[node][key] + values[key] for key in values}
                        self.diag("CAEPIPE-PRINT-RESTRAINT-ROWS-SUMMED", "info", title, row_line,
                                  f"restraint rows at node {node} are summed")
                        continue
                    self.diag("CAEPIPE-PRINT-DUPLICATE-NODE", "error", title, row_line,
                              f"node {node} appears twice; the later row is not read")
                    continue
                table[node] = values
        self.sections_parsed.append({"title": title, "kind": kind, "load_case": case, "line": line, "rows": len(data),
                                     "source_units": dict(self.section_units)})


def _template(row: tuple[str, ...]) -> tuple[tuple[str, re.Pattern | None, str | None], ...]:
    """A header row of the documented (SI) layout as cells; a cell carrying a unit becomes a slot for its quantity."""
    cells = []
    for cell in row:
        slot = _SLOT.search(cell)
        if slot is None:
            cells.append((cell, None, None))
        else:
            pattern = re.compile(re.escape(cell[:slot.start()]) + r"\((?P<label>[^()]*)\)" + re.escape(cell[slot.end():]) + r"\Z")
            cells.append((cell, pattern, _SI_SLOTS[slot.group(1)]))
    return tuple(cells)


def _match_headers(variants, found: tuple[tuple[str, ...], ...]) -> dict[int, tuple[str, str]] | None:
    """The unit label of each unit-bearing column of the last header row ({position: (quantity, label)}, plus the
    stress columns), for the first variant whose rows match `found` cell by cell with any label in the unit slots;
    None when no variant matches."""
    for variant in variants:
        if len(variant) != len(found) or any(len(a) != len(b) for a, b in zip(variant, found)):
            continue
        labels: dict[int, tuple[str, str]] = {}
        ok = True
        for number, (expected, got) in enumerate(zip(variant, found)):
            for position, ((text, pattern, quantity), cell) in enumerate(zip(_template(expected), got)):
                if pattern is None:
                    ok = ok and cell == text
                    continue
                hit = pattern.match(cell)
                if hit is None:
                    ok = False
                elif number == len(variant) - 1 or quantity != "stress":
                    labels[position if number == len(variant) - 1 else -1 - position] = (quantity, hit.group("label"))
        if ok:
            return labels
    return None


def _to_output(value: float, label: str | None) -> float:
    """A printed value in its column's unit, in the parser's output unit (exact catalog definitions)."""
    if label == "rad":
        return value / _RADIANS_PER_DEGREE
    factor = _TO_OUTPUT.get(label) if label is not None else None
    return value if factor is None else value * factor


def bind_element_rows(elements: list[dict[str, Any]], element_map: dict[str, Any]) -> dict[str, Any]:
    """Bind parsed element-force rows to SWBPIPE members through the MBF writer's id map (DEL-17-05 Design §11.4).

    Each parsed element is a (from, to) pair of rows. It binds to the one member whose element_map entry names the
    same two node labels; each row then belongs to the member end at its node: the CAEPIPE node of SWBPIPE's end_i
    is the entry's `from`, or its `to` when the entry is `reversed` (the layout runs against the member). Binding
    never depends on row position. A pair whose labels name no member, or more than one, a second pair for an
    already bound member, and a member no pair names are each listed as unmatched with the reason; nothing is
    dropped silently. A pair bound in the other row order, or at another position than the member's layout order,
    is bound and noted.
    """
    by_nodes: dict[frozenset[str], list[str]] = {}
    for member, entry in element_map.items():
        if isinstance(entry, dict) and isinstance(entry.get("from"), str) and isinstance(entry.get("to"), str):
            by_nodes.setdefault(frozenset((entry["from"], entry["to"])), []).append(member)
    bound: dict[str, dict[str, Any]] = {}
    unmatched_rows: list[dict[str, Any]] = []
    notes: list[dict[str, Any]] = []
    for element in elements:
        labels = (str(element.get("node_from")), str(element.get("node_to")))
        row = {"element_index": element.get("element_index"), "node_from": labels[0], "node_to": labels[1]}
        members = by_nodes.get(frozenset(labels), []) if labels[0] != labels[1] else []
        if len(members) != 1:
            unmatched_rows.append({**row, "reason": "no member of the writer's element_map joins these nodes"
                                   if not members else f"{len(members)} members join these nodes: {', '.join(members)}"})
            continue
        member = members[0]
        if member in bound:
            unmatched_rows.append({**row, "reason": f"a second row pair for {member}"})
            continue
        entry = element_map[member]
        end_i = entry["to"] if entry.get("reversed") else entry["from"]
        values = {labels[0]: element.get("from"), labels[1]: element.get("to")}
        bound[member] = {
            "element_index": element.get("element_index"),
            "end_i": {"caepipe_node": end_i, "values": values[end_i]},
            "end_j": {"caepipe_node": labels[1] if end_i == labels[0] else labels[0],
                      "values": values[labels[1] if end_i == labels[0] else labels[0]]},
        }
        if labels != (entry["from"], entry["to"]):
            notes.append({**row, "member": member, "note": "rows in the other order than the layout's from and to"})
        if element.get("element_index") != entry.get("layout_order"):
            notes.append({**row, "member": member,
                          "note": f"at position {element.get('element_index')}, layout order {entry.get('layout_order')}"})
    unmatched_members = [{"member": member, "from": entry.get("from"), "to": entry.get("to"),
                          "reason": "no element-force row pair names this member's nodes"}
                         for member, entry in element_map.items() if member not in bound]
    return {"basis": BINDING_BASIS, "bound": bound, "unmatched_rows": unmatched_rows,
            "unmatched_members": unmatched_members, "notes": notes}


def _normalize(fields: list[str]) -> tuple[str, ...]:
    stripped = [field.strip() for field in fields]
    while stripped and stripped[-1] == "":
        stripped.pop()
    return tuple(stripped)


def _is_number(text: str) -> bool:
    try:
        float(text)
    except ValueError:
        return False
    return True


def _is_title(row: tuple[str, ...]) -> bool:
    return len(row) == 1 and bool(row[0]) and "=" not in row[0] and not _is_number(row[0])


def _match(title: str) -> tuple[str | None, str]:
    for kind, spec in _SPECS.items():
        found = spec["pattern"].match(title)
        if found:
            return kind, found.group("case").strip()
    return None, ""

