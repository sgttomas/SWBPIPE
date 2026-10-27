"""Parser for the CSV results file written by CAEPIPE's Print > To File command (DEL-17-05).

The recognized section layout follows the public documentation of the
print-to-file output and private notes; the public fixture that exercises it is
invented. Each recognized section's
column-header and unit rows are verified exactly before any value is read, so no
unit is ever assumed. Unknown sections are skipped with an info diagnostic.

Parsed values are correlation evidence only. Nothing here asserts agreement or
compatibility with CAEPIPE, solver validation, code compliance or professional
reliance.
"""

from __future__ import annotations

import csv
import math
from pathlib import Path
import re
from typing import Any


PARSER_VERSION = "caepipe-print-csv-1"
DOCUMENT_KIND = "openpipestress.caepipe_print_csv.parsed"
PAIRING_BASIS = "row_order_from_to_pairs"
_BASIS = "public CAEPIPE-EXPORT-DATA / Print To File page + invented fixture"
_BLANK_NOTE = "A blank numeric field inside a recognized data row is read as 0.0 (observed export convention)."

UNITS = {"length": "mm", "rotation": "deg", "force": "N", "moment": "N*m"}

_FORCE_KEYS = ("fx", "fy", "fz", "mx", "my", "mz")
_LOCAL_KEYS = ("axial", "shear_y", "shear_z", "torque", "moment_in_plane", "moment_out_of_plane")
_DISPLACEMENT_KEYS = ("ux", "uy", "uz", "rx", "ry", "rz")
_NODE = re.compile(r"^[0-9]+[A-Za-z0-9]*$")
_STRESS_LABELS = ("", "SL", "SE", "Sopr")


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
        notes = f"Output member {kind}. Header and unit rows must match exactly after stripping spaces; " \
                f"otherwise the section is not parsed. {_BLANK_NOTE}"
        if kind == "element_forces_local":
            unit_row = " | ".join(",".join(variant[-1]) for variant in variants)
            notes += (" Accepted stress-column variants: none (Empty Weight), SL, SE, Sopr. Columns read: axial, "
                      "y shear, z shear, torque, in-plane and out-of-plane bending moment; SIF and stress columns are "
                      f"not read. Rows pair as (from node, to node) in layout order ({PAIRING_BASIS}).")
        if kind == "element_forces_global":
            notes += f" Rows pair as (from node, to node) in layout order ({PAIRING_BASIS})."
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
            "load_cases": self.load_cases,
            "pairing_basis": PAIRING_BASIS,
            "sections_parsed": self.sections_parsed,
            "sections_skipped": self.sections_skipped,
            "diagnostics": self.diagnostics,
        }

    def skip(self, title: str, index: int, reason: str, code: str, message: str) -> None:
        self.sections_skipped.append({"title": title, "line": index + 1, "reason": reason})
        self.diag(code, "info", title, index + 1, message)

    def read_header(self) -> int:
        lines = [index for index, row in enumerate(self.rows) if row][:2]
        if len(lines) == 2:
            first, second = self.rows[lines[0]], self.rows[lines[1]]
            if second[0].startswith("Version ") and len(first) >= 1:
                self.header["product_label"] = first[0]
                self.header["model_name"] = ",".join(first[1:]) if len(first) > 1 else None
                self.header["version"] = second[0][len("Version "):].strip()
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
        if tuple(found) not in spec["headers"]:
            expected = " / ".join(",".join(row) for row in spec["headers"][0])
            got = " / ".join(",".join(row) for row in found)
            self.diag("CAEPIPE-PRINT-COLUMN-HEADER-MISMATCH", "error", title, title_line,
                      f"column-header and unit rows of {title!r} do not match the recognized layout "
                      f"(expected {expected!r}, got {got!r}); the section is not parsed")
            self.sections_skipped.append({"title": title, "line": title_line, "reason": "header_mismatch"})
            return index + 1
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
            values[key] = value
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
        self.sections_parsed.append({"title": title, "kind": kind, "load_case": case, "line": line, "rows": len(data)})


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

