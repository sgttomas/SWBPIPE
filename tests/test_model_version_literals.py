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

import ast
import re
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
MODEL_FORMAT = ROOT / "core" / "product_physics" / "src" / "model_format.rs"
TS_DEFINITION = ROOT / "apps" / "desktop" / "src" / "services" / "modelSchemaVersion.ts"
DESKTOP_TS = ROOT / "apps" / "desktop" / "src"
DESKTOP_RS = ROOT / "apps" / "desktop" / "src-tauri" / "src"
APPLIER_RS = ROOT / "core" / "model_operations" / "operation_applier" / "src"
# W-15 slice c (DEL-02-01 Design model-document-0.8.0.md §2, §5): the product-physics
# and result-export production sources are scanned too. Their production code names the
# current version through `crate::MODEL_SCHEMA_VERSION`, except the independent readers'
# gates and texts, which the version ripple edits by token (MD8 §2, §7: they are priced
# and do not depend on the producer crate). Those sites are listed here, tied to the
# version they were taken at: the test fails when a file gains or loses one, and when the
# current version moves while the list still names the old one, so the next ripple
# re-checks each site and cannot leave one silently at a retired version.
EXTENDED_RS = (ROOT / "core" / "product_physics" / "src", ROOT / "core" / "reporting" / "result_export" / "src")
# Re-checked at model document 0.9.0 (DEL-02-01 Design model-document-0.9.0.md §3.5): each site below edited by
# token, the counts unchanged.
READER_TOKEN_VERSION = "0.9.0"
READER_TOKEN_SITES = {
    # `physics_source.rs` :32, :838, :1360 (ACTUAL_MODEL_SCHEMA): the captured model's current-version gates.
    "core/reporting/result_export/src/physics_source.rs": 3,
    # `retained_precision.rs` :4866, :4954: the exact and preview namespace gates.
    "core/reporting/result_export/src/retained_precision.rs": 2,
    # `semantic_contract.rs` :508: the retired v2 contract's re-author text.
    "core/reporting/result_export/src/semantic_contract.rs": 1,
}
# The Python verifiers and validation tools that state a model version (MD8 §5). Each
# names the current version once, as a module-level `MODEL_VERSION` definition that every
# gate and message reads; any other literal holding the current version, or naming a
# model version in words, is refused. Docstrings and comments are free text.
PYTHON_VERSION_ROOTS = (ROOT / "tests" / "verifiers", ROOT / "tools" / "validation", ROOT / "tools" / "fixtures",
                        ROOT / "core" / "handoff", ROOT / "core" / "project_persistence", ROOT / "core" / "product_preview")

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


def cfg_test_module_files(directory: Path) -> set[Path]:
    """The files a crate root or module declares as `#[cfg(test)] mod name;` (with an
    optional `#[path = "..."]`), so a test module whose file name does not say so is not
    taken for production code."""
    found = set()
    for path in directory.rglob("*.rs"):
        text = path.read_text(encoding="utf-8")
        for match in re.finditer(r'#\[cfg\(test\)\]\s*(?:#\[path\s*=\s*"([^"]+)"\]\s*)?mod\s+(\w+)\s*;', text):
            base = path.parent if path.name in ("lib.rs", "mod.rs", "main.rs") else path.parent / path.stem
            found.add((base / match.group(1)) if match.group(1) else (base / f"{match.group(2)}.rs"))
            if not match.group(1):
                found.add(base / match.group(2) / "mod.rs")
    return {p.resolve() for p in found}


def extended_production_files():
    """Product-physics and result-export production Rust: test files, test
    directories, `#[cfg(test)]` modules (inline or declared) excluded, and the constants'
    own file."""
    for directory in EXTENDED_RS:
        tests_only = cfg_test_module_files(directory)
        for path in sorted(directory.rglob("*.rs")):
            parts = path.relative_to(directory).parts
            if path == MODEL_FORMAT or path.name == "tests.rs" or path.name.endswith("_tests.rs") \
                    or path.resolve() in tests_only \
                    or any(part.endswith("_tests") or part in ("tests", "retained_tests_hooks") for part in parts[:-1]):
                continue
            yield path


def extended_current_literals() -> dict[str, int]:
    current, _ = model_versions()
    counts: dict[str, int] = {}
    for path in extended_production_files():
        text = without_rust_test_modules(path.read_text(encoding="utf-8"))
        for _, body in literals(text, True):
            if current in body or MODEL_WORDS.search(body):
                relative = path.relative_to(ROOT).as_posix()
                counts[relative] = counts.get(relative, 0) + 1
    return counts


def python_files():
    for directory in PYTHON_VERSION_ROOTS:
        yield from sorted(directory.rglob("*.py"))


def python_version_literals():
    """(where, kind, text) of every string literal in the Python verifiers and tools that
    holds the current version outside a module-level `MODEL_VERSION` definition, or names
    a model version in words; and the module-level definitions that are not current."""
    current, _ = model_versions()
    found, definitions = [], []
    for path in python_files():
        tree = ast.parse(path.read_text(encoding="utf-8"))
        allowed, docstrings = set(), set()
        for node in tree.body:
            if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "MODEL_VERSION" for t in node.targets) \
                    and isinstance(node.value, ast.Constant) and isinstance(node.value.value, str):
                allowed.add(id(node.value))
                definitions.append((path.relative_to(ROOT).as_posix(), node.value.value))
        for node in ast.walk(tree):
            if isinstance(node, (ast.Module, ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)) and node.body \
                    and isinstance(node.body[0], ast.Expr) and isinstance(node.body[0].value, ast.Constant):
                docstrings.add(id(node.body[0].value))
        for node in ast.walk(tree):
            if not (isinstance(node, ast.Constant) and isinstance(node.value, str)) or id(node) in allowed | docstrings:
                continue
            where = f"{path.relative_to(ROOT).as_posix()}:{node.lineno}"
            if current in node.value:
                found.append((where, "current version", node.value[:120]))
            elif MODEL_WORDS.search(node.value):
                found.append((where, "model version in words", node.value[:120]))
    found.extend((where, "MODEL_VERSION is not the current version", value)
                 for where, value in definitions if value != current)
    return found, definitions


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


def test_the_extended_scan_finds_exactly_the_listed_reader_token_sites():
    """W-15 slice c: every current-version literal in product-physics and result-export
    production code is a listed independent-reader token site, and the list is tied to
    the version it was taken at."""
    current, _ = model_versions()
    assert list(extended_production_files())
    assert current == READER_TOKEN_VERSION, (
        f"the current version moved to {current} while READER_TOKEN_SITES was taken at {READER_TOKEN_VERSION}: "
        "edit each listed reader site by token, route any other through the constant, then re-take the list")
    assert extended_current_literals() == READER_TOKEN_SITES


def test_a_cfg_test_module_file_is_not_production_code():
    """`membrane_publication_range.rs` is declared `#[cfg(test)] mod` in lib.rs: it is test code."""
    files = {path.relative_to(ROOT).as_posix() for path in extended_production_files()}
    assert "core/product_physics/src/membrane_publication_range.rs" not in files
    assert "core/product_physics/src/rigid_component.rs" in files


def test_python_verifiers_and_tools_name_the_version_only_through_their_definition():
    """MD8 §5: the Python verifiers and validation tools hold the current version only in
    their module-level `MODEL_VERSION` definitions, each current."""
    found, definitions = python_version_literals()
    assert not found, "\n".join(f"{where}: {kind}: {text}" for where, kind, text in found)
    assert {where for where, _ in definitions} >= {
        "tests/verifiers/compatibility.py", "tests/verifiers/physics_source.py", "tests/verifiers/pressure_2.py",
        "tests/verifiers/retained_precision.py", "tools/validation/qualification_physics.py",
        "tools/validation/qualification_physics_structure.py", "tools/fixtures/retained_precision_synthetic_bases.py"}


def test_the_python_scan_finds_a_stray_current_literal(tmp_path, monkeypatch):
    current, _ = model_versions()
    sample = tmp_path / "sample.py"
    sample.write_text(f'"""Docstring naming model {current} is free text."""\nMODEL_VERSION = "{current}"\n'
                      f'TEXT = "requires model {current}"\nGATE = lambda m: m["schema_version"] == "{current}"\nOLD = "0.0.1"\n',
                      encoding="utf-8")
    monkeypatch.setattr(sys.modules[__name__], "PYTHON_VERSION_ROOTS", (tmp_path,))
    monkeypatch.setattr(sys.modules[__name__], "ROOT", tmp_path)
    found, definitions = python_version_literals()
    assert [(where.split(":")[1], kind) for where, kind, _ in found] == [("3", "current version"), ("4", "current version")]
    assert definitions == [("sample.py", current)]
    sample.write_text('MODEL_VERSION = "0.0.1"\n', encoding="utf-8")
    found, _ = python_version_literals()
    assert [kind for _, kind, _ in found] == ["MODEL_VERSION is not the current version"]


def test_every_python_retired_list_is_the_rust_one():
    """r6 (model document 0.8.0): the Python readers that refuse a retired model document by name (project
    persistence, the preview service, the CAEPIPE SI writer, qualification) list exactly model_format.rs's retired
    versions, so the next ripple fails here until each is re-checked."""
    _, retired = model_versions()
    found = {}
    for path in python_files():
        tree = ast.parse(path.read_text(encoding="utf-8"))
        for node in tree.body:
            if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "RETIRED_MODEL_VERSIONS" for t in node.targets) \
                    and isinstance(node.value, ast.Tuple) and all(isinstance(e, ast.Constant) for e in node.value.elts):
                found[path.relative_to(ROOT).as_posix()] = [e.value for e in node.value.elts]
    assert {"core/project_persistence/service.py", "core/product_preview/service.py", "core/handoff/caepipe_mbf/si_writer.py"} <= set(found)
    assert {path: listed for path, listed in found.items() if listed != retired} == {}
