"""COMPATIBILITY of the shared M3 judge's schema reader (validation/references/m3_judge/m3_view.py) with the
public schemas it reads at judge time.

The judge validates parts of an exact-route record's contract evidence against schemas/ (load/reference
states, exact combinations, and the pressure-source-1 recovery evidence) with a reader that admits a closed
set of keywords, `m3_view.SCHEMA_KEYWORDS`, and refuses the record (SchemaUnsupported) when any keyword
outside it is reachable. That refusal is by design, but it means a schema change can disable part of the
judge without any failure here: 0c480a9e5 added `allOf` to load_reference_state.schema.json and every
exact-route record carrying load/reference states became unreadable, unnoticed, until the reader learnt it.

This test takes the schema entries the judge consumes from the judge's own tables (EXACT_EVIDENCE_SCHEMA
with SHAPE_CHECKED, SOURCE_RECOVERY_SCHEMA with SOURCE_RECOVERY_EVIDENCE), not from a sample record, so
optional evidence is covered whether or not a record carries it. It checks that those tables are the
reader's only call sites, then walks each entry and every `$ref` it reaches, across files, with a walker of
its own that descends wherever JSON Schema places a subschema, and fails naming the schema file, the
JSON pointer and the keyword (or type) the reader does not support. It also runs the reader's own
`schema_supported` on each entry. Pure reading; nothing is judged.
"""
from __future__ import annotations

import ast
import json
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
JUDGE = ROOT / "validation" / "references" / "m3_judge"
SCHEMAS = ROOT / "schemas"
# The entries' $ref targets are named in the schemas' own text, which the CI selector cannot read: a change to
# any schema may change what an entry reaches. The call-site guard scans the judge's modules.
CI_READS = ("schemas/**", "validation/references/m3_judge/**")
sys.path.insert(0, str(JUDGE))
import m3_view as MV  # noqa: E402

# Where JSON Schema (draft-07 to 2020-12) places subschemas: one schema, a map of schemas, or a list of
# schemas. `$defs`/`definitions` hold schemas that apply only through a `$ref`, which the walk follows.
ONE = ("items", "additionalProperties", "not", "if", "then", "else", "contains", "propertyNames", "additionalItems",
       "unevaluatedItems", "unevaluatedProperties")
MAP = ("properties", "patternProperties", "dependentSchemas")
LIST = ("allOf", "anyOf", "oneOf", "prefixItems")
# The judge's self-test modules, which call the reader on schemas they construct; every other module is scanned.
SELF_TEST = {"m3_selftest.py", "m3_regress.py"}
READER = {"schema_problems", "schema_supported"}
# The functions that call the reader, and the table names each must take its entries from.
CALL_SITES = {"_evidence": {"SOURCE_RECOVERY_SCHEMA", "SOURCE_RECOVERY_EVIDENCE"},
              "_schema_checked_members": {"EXACT_EVIDENCE_SCHEMA", "SHAPE_CHECKED"}}


def consumed_entries() -> list[tuple[str, str]]:
    """Every (schema file, JSON pointer) the judge passes to its schema reader, from its own tables."""
    doc, ptr = MV.EXACT_EVIDENCE_SCHEMA
    out = [(doc, f"{ptr}/properties/{member}") for member in MV.SHAPE_CHECKED]
    out += [(MV.SOURCE_RECOVERY_SCHEMA, ptr) for ptr in sorted(set(MV.SOURCE_RECOVERY_EVIDENCE.values()))]
    return out


_DOCS: dict = {}


def _doc(name: str):
    if name not in _DOCS:
        _DOCS[name] = json.loads((SCHEMAS / name).read_text(encoding="utf-8"))
    return _DOCS[name]


def _resolve(name: str, pointer: str):
    node = _doc(name)
    for part in [p for p in pointer.split("/") if p]:
        part = part.replace("~1", "/").replace("~0", "~")
        node = node[int(part)] if isinstance(node, list) else node[part]
    return node


def reachable(name: str, pointer: str) -> list[tuple[str, str, str, object]]:
    """(file, pointer, keyword, value) of every keyword of every schema object reachable from the entry."""
    out, seen, stack = [], set(), [(name, pointer)]
    while stack:
        doc, ptr = stack.pop()
        if (doc, ptr) in seen:
            continue
        seen.add((doc, ptr))
        node = _resolve(doc, ptr)
        if not isinstance(node, dict):
            out.append((doc, ptr, "<a schema that is not an object>", node))
            continue
        for key, value in node.items():
            out.append((doc, ptr, key, value))
            if key == "$ref":
                target, _, frag = value.partition("#")
                stack.append((target or doc, frag))
            elif key in ONE:
                if isinstance(value, list):     # draft-07 tuple items
                    stack += [(doc, f"{ptr}/{key}/{i}") for i in range(len(value))]
                elif isinstance(value, dict):
                    stack.append((doc, f"{ptr}/{key}"))
            elif key in MAP and isinstance(value, dict):
                stack += [(doc, f"{ptr}/{key}/{k.replace('~', '~0').replace('/', '~1')}") for k in value]
            elif key in LIST and isinstance(value, list):
                stack += [(doc, f"{ptr}/{key}/{i}") for i in range(len(value))]
    return out


def unsupported(name: str, pointer: str) -> list[str]:
    found = []
    for doc, ptr, key, value in reachable(name, pointer):
        if key not in MV.SCHEMA_KEYWORDS:
            found.append(f"{doc}#{ptr}: keyword {key!r}")
        elif key == "type":
            for t in value if isinstance(value, list) else [value]:
                if t not in MV._TYPES:
                    found.append(f"{doc}#{ptr}: type {t!r}")
    return found


@pytest.mark.parametrize("entry", consumed_entries(), ids=lambda e: f"{e[0]}#{e[1]}")
def test_every_reachable_keyword_is_one_the_judge_reads(entry):
    name, pointer = entry
    rows = reachable(name, pointer)
    print(f"\n{name}#{pointer}: {len({(d, p) for d, p, _, _ in rows})} schema objects in "
          f"{sorted({d for d, _, _, _ in rows})}; keywords {sorted({k for _, _, k, _ in rows})}")
    found = unsupported(name, pointer)
    assert not found, (f"the M3 judge's schema reader (m3_view.SCHEMA_KEYWORDS) would refuse every record that "
                       f"carries {name}#{pointer}:\n" + "\n".join(found))
    MV.schema_supported(_resolve(name, pointer), name)   # the reader's own check agrees


def test_the_judges_tables_are_its_only_schema_entries():
    """A new call of the reader outside the two known sites would consume an entry this test does not walk."""
    calls = {}
    modules = sorted(path for path in JUDGE.rglob("*.py") if path.name not in SELF_TEST)
    assert JUDGE / "m3_view.py" in modules
    for path in modules:
        module = path.relative_to(JUDGE).as_posix()
        tree = ast.parse(path.read_text(encoding="utf-8"))
        for fn in ast.walk(tree):
            if not isinstance(fn, (ast.FunctionDef, ast.AsyncFunctionDef)) or fn.name in READER:
                continue
            called = {(c.func.id if isinstance(c.func, ast.Name) else getattr(c.func, "attr", None))
                      for c in ast.walk(fn) if isinstance(c, ast.Call)}
            if called & READER:
                calls[(module, fn.name)] = {n.id for n in ast.walk(fn) if isinstance(n, ast.Name)}
    assert set(calls) == {("m3_view.py", f) for f in CALL_SITES}, (
        f"the schema reader is called from {sorted(calls)}; add the new site's entries to consumed_entries()")
    for (_, fn), names in calls.items():
        assert CALL_SITES[fn] <= names, f"{fn} no longer takes its entries from {sorted(CALL_SITES[fn])}"
