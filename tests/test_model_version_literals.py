#!/usr/bin/env python3
"""No stray model-document version literal in desktop or applier production code.

DEL-02-01 Design model-document-0.7.0.md §2 ("Guard test"): the current model
document version and the retired list are defined once per language
(`core/product_physics/src/model_format.rs`, which the desktop's Rust side and the
operation applier import, and `apps/desktop/src/services/modelSchemaVersion.ts`).
Production code names a model version only through those constants: comparisons,
messages and names alike. Comments are free text and are not scanned; test code
(`*.test.*`, `test-support/`, `*_tests.rs` and `#[cfg(test)]` modules) is not
production code.

A string literal is refused when it
- contains the current version;
- names a model version in words ("model 0.6.0", "Model document 0.7.0", ...); or
- is exactly a retired model version on a line that gates on the model's
  `schema_version` or names retired versions (a version gate or a copied retired list).
"""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MODEL_FORMAT = ROOT / "core" / "product_physics" / "src" / "model_format.rs"
TS_DEFINITION = ROOT / "apps" / "desktop" / "src" / "services" / "modelSchemaVersion.ts"
DESKTOP_TS = ROOT / "apps" / "desktop" / "src"
DESKTOP_RS = ROOT / "apps" / "desktop" / "src-tauri" / "src"
APPLIER_RS = ROOT / "core" / "model_operations" / "operation_applier" / "src"

MODEL_WORDS = re.compile(r"(?i)\bmodel(?:[ _-]?(?:document|format|version|schema))*\s+v?0\.\d+\.\d+")
VERSION = re.compile(r"^0\.\d+\.\d+$")
# A line that gates on the model's version or lists retired versions.
MODEL_GATE = re.compile(r"""(?i)\bmodel\w*\s*(?:\.|\[\s*["']|\.get\(\s*["'])schema_version|retired[_ ]model|model[\w ]*retired|\bschema\s*[!=]==?""")


def model_versions() -> tuple[str, list[str]]:
    source = MODEL_FORMAT.read_text(encoding="utf-8")
    current = re.search(r'pub const MODEL_SCHEMA_VERSION: &str = "([^"]+)";', source).group(1)
    retired = re.findall(r'"(0\.\d+\.\d+)"', re.search(r"pub const RETIRED_MODEL_SCHEMA_VERSIONS[^=]*=\s*\[([^\]]*)\]",
                                                        source).group(1))
    return current, retired


def literals(text: str, rust: bool):
    """(line number, literal body) of every string literal, skipping comments."""
    out, i, line, n = [], 0, 1, len(text)
    while i < n:
        c = text[i]
        if c == "\n":
            line += 1
            i += 1
        elif text.startswith("//", i):
            i = text.find("\n", i)
            i = n if i < 0 else i
        elif text.startswith("/*", i):
            end = text.find("*/", i + 2)
            end = n if end < 0 else end + 2
            line += text.count("\n", i, end)
            i = end
        elif rust and re.match(r"r#*\"", text[i:i + 8]) and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            hashes = re.match(r"r(#*)\"", text[i:]).group(1)
            start = i + 2 + len(hashes)
            end = text.find('"' + hashes, start)
            out.append((line, text[start:end]))
            line += text.count("\n", i, end)
            i = end + 1 + len(hashes)
        elif c == "'" and rust:
            match = re.match(r"'(\\u\{[0-9a-fA-F]+\}|\\.|[^\\'\n])'", text[i:])
            i += len(match.group(0)) if match else 1
        elif c in "\"'`":
            j = i + 1
            while j < n and text[j] != c:
                j += 2 if text[j] == "\\" else 1
            out.append((line, text[i + 1:j]))
            line += text.count("\n", i, j)
            i = j + 1
        else:
            i += 1
    return out


def without_rust_test_modules(text: str) -> str:
    """Blank `#[cfg(test)]` items: the attribute's following item up to its matching top-level close."""
    lines = text.split("\n")
    out, skipping = [], False
    for index, line in enumerate(lines):
        if not skipping and line.strip() == "#[cfg(test)]":
            following = next((l for l in lines[index + 1:] if l.strip()), "")
            if re.match(r"\s*(pub(\([^)]*\))?\s+)?mod\s+\w+\s*\{\s*$", following):
                skipping = True
                out.append("")
                continue
        if skipping:
            out.append("")
            if line == "}":
                skipping = False
            continue
        out.append(line)
    return "\n".join(out)


def production_files():
    for path in sorted(DESKTOP_TS.rglob("*")):
        if path.suffix not in {".ts", ".tsx"} or path == TS_DEFINITION:
            continue
        relative = path.relative_to(DESKTOP_TS).as_posix()
        if ".test." in path.name or relative.startswith("test-support/") or "/__tests__/" in f"/{relative}":
            continue
        yield path, False
    for directory in (DESKTOP_RS, APPLIER_RS):
        for path in sorted(directory.rglob("*.rs")):
            if not path.name.endswith("_tests.rs"):
                yield path, True


def stray_literals():
    current, retired = model_versions()
    found = []
    for path, rust in production_files():
        text = path.read_text(encoding="utf-8")
        if rust:
            text = without_rust_test_modules(text)
        source_lines = text.split("\n")
        for line, body in literals(text, rust):
            where = f"{path.relative_to(ROOT).as_posix()}:{line}"
            context = source_lines[line - 1] if line - 1 < len(source_lines) else ""
            if current in body:
                found.append((where, "current version", body[:120]))
            elif MODEL_WORDS.search(body):
                found.append((where, "model version in words", body[:120]))
            elif VERSION.match(body) and body in retired and MODEL_GATE.search(context):
                found.append((where, "retired version gate", context.strip()[:120]))
    return found


def test_the_constants_name_the_current_version():
    current, retired = model_versions()
    definition = TS_DEFINITION.read_text(encoding="utf-8")
    assert f'export const MODEL_SCHEMA_VERSION = "{current}";' in definition
    listed = re.findall(r'"(0\.\d+\.\d+)"', re.search(r"RETIRED_MODEL_SCHEMA_VERSIONS[^=]*=\s*\[([^\]]*)\]", definition).group(1))
    assert listed == retired
    assert current not in retired


def test_no_stray_model_version_literal_in_desktop_or_applier_production_code():
    assert production_files()
    found = stray_literals()
    assert not found, "\n".join(f"{where}: {kind}: {text}" for where, kind, text in found)


def test_the_guard_finds_each_kind_of_stray_literal():
    current, retired = model_versions()
    samples = [
        (f'if (model.schema_version === "{current}") return;', False),
        ('const message = "requires model document 0.4.0 or later";', False),
        (f'if model["schema_version"] == "{retired[-1]}" {{ return; }}', True),
        (f'let text = format!("Model {current} only");', True),
    ]
    for source, rust in samples:
        hits = [body for _, body in literals(source, rust)
                if current in body or MODEL_WORDS.search(body) or (VERSION.match(body) and body in retired)]
        assert hits, source
    # Comments, lifetimes and other formats' versions are not literals of a model version.
    quiet = f"// model {current}\nfn f<'a>(x: &'a str) {{ let unit = \"0.1.0\"; }}"
    assert [body for _, body in literals(quiet, True)] == ["0.1.0"]
    assert not MODEL_WORDS.search("0.1.0")
