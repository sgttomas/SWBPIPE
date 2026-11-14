"""The pressure-elongation inventory's closure (DEL-04-07): every tracked file under the scanned roots that names
`pressure_regions` is claimed exactly once, and each way of breaking that fails by name.

The two misses this closure exists for are re-created by removing their claims: the stress benchmark builds its
documents in Rust (validation/benchmarks/stress/src/lib.rs), and the K-D5 M3-T frozen subset is JSON Lines
(validation/references/kd5_timoshenko/frozen/kd5t_frozen_subset.jsonl). The committed claims are used unchanged
otherwise, over one scan of the repository.
"""
from __future__ import annotations

import ast
import copy
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.append(str(ROOT))
from tools.fixtures import retained_precision_corpus as CORPUS  # noqa: E402
from tools.validation import pressure_elongation as PE  # noqa: E402
from tools.validation import pressure_elongation_inventory as INV  # noqa: E402

STRESS = "validation/benchmarks/stress/src/lib.rs"
KD5 = "validation/references/kd5_timoshenko/frozen/kd5t_frozen_subset.jsonl"
NOT_NAMING = "tools/validation/pressure_elongation_anchor.py"


@pytest.fixture(scope="module")
def scanned():
    return INV.scan()


def problems(scanned, **claims) -> str:
    with pytest.raises(INV.InventoryError) as raised:
        INV.build(scanned=scanned, **claims)
    return str(raised.value)


def without(claims: list[dict], path: str) -> list[dict]:
    kept = [c for c in claims if path not in c["paths"]]
    assert len(kept) == len(claims) - 1, path
    return kept


# ----------------------------------------------------------------------------- the committed inventory
def test_the_inventory_is_current(scanned):
    assert INV.OUTPUT.read_text(encoding="utf-8") == INV.dumps(INV.build(scanned=scanned)), \
        "run python3 tools/validation/pressure_elongation_inventory.py and classify every new file naming pressure_regions"


def test_every_scanned_file_is_claimed_once_and_the_scan_reaches_every_type(scanned):
    files, data = scanned
    inventory = INV.build(scanned=scanned)
    assert inventory["scan"]["files"] == len(files) == sum(inventory["scan"]["claimed_by"].values())
    assert {Path(f).suffix for f in files} >= {".json", ".jsonl", ".rs", ".py", ".ts", ".tsx", ".md"}
    assert not any(f.startswith("execution/") for f in files)
    in_code = {p for entry in inventory["in_code_constructions"] for p in entry["paths"]}
    exempt = {p for entry in inventory["exemptions"] for p in entry["files"]}
    grouped = {entry["path"] for entry in inventory["entries"]}
    assert not (in_code & exempt) and not (in_code & grouped) and not (exempt & grouped)
    assert in_code | exempt | grouped == set(files)


# ----------------------------------------------------------------------------- the two known misses
def test_the_stress_benchmark_built_in_rust_is_caught_when_unclaimed(scanned):
    assert STRESS in scanned[0]
    found = problems(scanned, in_code=without(INV.IN_CODE, STRESS))
    assert f"  unclaimed: {STRESS}" in found


def test_the_kd5_json_lines_subset_is_caught_when_unclaimed(scanned):
    assert KD5 in scanned[0]
    groups = [g for g in INV.GROUPS if g["id"] != "kd5_timoshenko_frozen"]
    assert len(groups) == len(INV.GROUPS) - 1
    found = problems(scanned, groups=groups)
    assert f"  unclaimed: {KD5}" in found
    # Nor can it be exempted: it holds pressurised documents.
    found = problems(scanned, groups=groups, exempt=[*INV.EXEMPT, {"paths": [KD5], "reason": "an attempt"}])
    assert f"{KD5} holds a pressurised document; claim it with a group" in found


def test_the_kd5_subset_is_twelve_pressurised_documents_naming_no_setting(scanned):
    [entry] = [e for e in INV.build(scanned=scanned)["entries"] if e["path"] == KD5]
    assert entry["group"] == "kd5_timoshenko_frozen" and entry["documents"] == []
    assert entry["anchor"]["kind"] == "excluded" and entry["producer"]["status"] == "awaiting_m3_restoration"
    held = entry["held_documents"]
    assert len(held) == 12 and all(len(d["pressurised_load_cases"]) == 1 for d in held)
    lines = (ROOT / KD5).read_text(encoding="utf-8").splitlines()
    nonzero = 0
    for record in held:
        doc = json.loads(lines[record["line"] - 1])
        for part in record["pointer"].split("/")[1:]:
            doc = doc[part]
        assert "pressure_elongation" not in (doc.get("model_settings") or {}), record
        nonzero += sum(1 for case in doc["load_cases"] for region in case.get("pressure_regions") or []
                       if PE._nonzero(region.get("pressure")))
    assert nonzero == 12


# ----------------------------------------------------------------------------- each way of breaking the closure
def test_a_stale_claim_fails(scanned):
    assert NOT_NAMING not in scanned[0]
    entry = {"paths": [NOT_NAMING], "expectations": "closed_form", "cases_named": "all", "detail": "a stale claim"}
    assert f"stale: IN_CODE path {NOT_NAMING} is not a tracked file naming pressure_regions" in \
        problems(scanned, in_code=[*INV.IN_CODE, entry])
    assert "stale: exemption path tools/no_such_dir/* matches no tracked file naming pressure_regions" in \
        problems(scanned, exempt=[*INV.EXEMPT, {"paths": ["tools/no_such_dir/*"], "reason": "a stale pattern"}])
    groups = copy.deepcopy(INV.GROUPS)
    groups[0]["paths"].append("validation/references/no_such_reference/*.json")
    assert f"stale: group {groups[0]['id']} path validation/references/no_such_reference/*.json" in \
        problems(scanned, groups=groups)


def test_an_exemption_without_a_reason_fails(scanned):
    exempt = copy.deepcopy(INV.EXEMPT)
    for blank in ("", "  ", None):
        exempt[0]["reason"] = blank
        assert "exemption #0 (" in problems(scanned, exempt=exempt) and ": no reason" in problems(scanned, exempt=exempt)


def test_an_in_code_entry_must_say_what_it_rests_on(scanned):
    in_code = copy.deepcopy(INV.IN_CODE)
    [stress] = [e for e in in_code if STRESS in e["paths"]]
    stress.update(expectations="trust", cases_named="most", detail=" ")
    found = problems(scanned, in_code=in_code)
    assert "expectations must be one of" in found and "cases_named must be one of" in found and "no detail" in found


def test_a_file_claimed_twice_fails(scanned):
    found = problems(scanned, exempt=[*INV.EXEMPT, {"paths": [STRESS], "reason": "a second claim"}])
    assert f"  claimed twice: {STRESS} (IN_CODE {STRESS}; exemption #" in found


def test_an_in_code_entry_cannot_claim_a_data_file(scanned):
    data_file = "validation/references/t4_u5a/requests/J3-0.request.json"
    entry = {"paths": [data_file], "expectations": "reference_file", "cases_named": "all", "detail": "wrong kind"}
    assert f"IN_CODE path {data_file} is a data file" in problems(scanned, in_code=[*INV.IN_CODE, entry])


# ----------------------------------------------------------------------------- documents the rule reads
def test_documents_are_read_in_json_text_json_lines_and_dag_storage():
    doc = {"schema_version": "0.9.0", "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3"},
           "load_cases": [{"id": "case:p", "pressure_regions": [{"id": "r", "pressure": {"value": 1.0, "unit": "Pa"}}]}]}
    record = {"schema_version": "0.9.0", "pressurised_load_cases": ["case:p"]}
    assert INV.data_documents("a.json", json.dumps({"model": doc})) == ([{"pointer": "/model", **record}], [])
    text = json.dumps({"requests": [{"args": {"modelJson": json.dumps({"wrapped": json.dumps(doc)})}}]})
    assert INV.data_documents("a.json", text) == ([], [{"pointer": "/requests/0/args/modelJson",
                                                        "text_pointers": ["/wrapped", ""], **record}])
    lines = "\n".join([json.dumps({"tag": 1, "request": {"model": {**doc, "load_cases": []}}}), "",
                       json.dumps({"tag": 2, "request": {"model": doc}})])
    assert INV.data_documents("a.jsonl", lines) == ([], [{"line": 3, "pointer": "/request/model", **record}])
    dag = CORPUS.compact((json.dumps({"entries": [{"request": {"model": doc}}]}, indent=2) + "\n").encode())
    assert INV.data_documents("a.json", json.dumps(dag)) == \
        ([], [{"pointer": "/entries/0/request/model", **record, "storage": CORPUS.FORMAT}])


def resolve(value, pointer: str):
    for part in pointer.split("/")[1:] if pointer else []:
        part = part.replace("~1", "/").replace("~0", "~")
        value = value[int(part)] if isinstance(value, list) else value[part]
    return value


def held_document(path: str, record: dict) -> dict:
    """The document a held record locates."""
    text = (ROOT / path).read_text(encoding="utf-8")
    container = json.loads(text.splitlines()[record["line"] - 1] if "line" in record else text)
    if "storage" in record:
        container = CORPUS.expand(container)
    value = resolve(container, record["pointer"])
    for pointer in record.get("text_pointers", []):
        value = resolve(json.loads(value), pointer)
    return value


def test_held_documents_found_in_the_repository_are_recorded_where_they_are(scanned):
    """The files whose pressurised documents only this scan reads, beside the JSON Lines subset. Each record locates a
    document with exactly the recorded pressurised cases; each case is named, except the MBF parity vectors', which
    are never solved."""
    entries = {e["path"]: e for e in INV.build(scanned=scanned)["entries"]}
    for path, named in (("apps/desktop/src/test-support/native-replay/native_replay_requests.json", True),
                        ("apps/desktop/src/test-support/analysis-run-v03/analysis_run_v03_cases.json", True),
                        ("fixtures/results/pressure_source_reader_corpus.json", True),
                        ("fixtures/caepipe_mbf/invented/parity/writer_vectors.json", False),
                        ("fixtures/caepipe_mbf/invented/parity/import_vectors.json", False)):
        entry = entries[path]
        assert entry["held_documents"] and entry["anchor"]["kind"] == "excluded", path
        for record in entry["held_documents"]:
            doc = held_document(path, record)
            assert PE.pressurised_case_ids(doc) == record["pressurised_load_cases"], (path, record)
            listed = ((doc.get("model_settings") or {}).get("pressure_elongation") or {}).get("operating_load_cases")
            assert (set(record["pressurised_load_cases"]) <= set(listed or [])) is named, (path, record)


# ----------------------------------------------------------------------------- what the scan does not reconstruct
PARITY_READERS = r"parity/(writer|import)_vectors|caepipe_mbf_(writer|import)_vectors"
SOLVE_ENTRIES = re.compile(r"run_linear_static_preview|open_pipe_stress_product_physics|open_pipe_stress_headless_runner"
                           r"|openpipestress-runner|OPENPIPESTRESS_RUNNER_BIN")
CODE = ("*.rs", "*.py", "*.ts", "*.tsx", "*.js", "*.mjs")


def test_no_code_naming_the_mbf_parity_vectors_names_a_solve_entry():
    """The writer parity vectors hold many pressurised models as delta-assembled blobs the scan does not reconstruct
    (the module docstring). They are claimed as MBF writer inputs, none of them solved as source-reviewed. This is a
    literal-name guard, not reachability: every tracked code file of the CODE suffixes that names the vectors or their
    harvest modules, and the Cargo manifest of a Rust one's crate, names no mechanics solve entry (the product's
    preview entry, the product or runner crate, or the runner binary). A solve reached without those names is not
    detected."""
    done = subprocess.run(["git", "grep", "-l", "-E", PARITY_READERS, "--", *CODE], cwd=ROOT, capture_output=True,
                          text=True, check=True)
    own = {"tools/validation/pressure_elongation_inventory.py", "tools/validation/test_pressure_elongation_inventory.py"}
    readers = sorted(set(done.stdout.split()) - own)
    assert "tools/fixtures/caepipe_mbf_writer_vectors.py" in readers and \
        "core/handoff/caepipe_mbf_engine/tests/writer_parity.rs" in readers, readers
    for path in readers:
        checked = [path]
        if path.endswith(".rs"):
            crate = (ROOT / path).parent
            while not (crate / "Cargo.toml").exists():
                crate = crate.parent
            checked.append(str((crate / "Cargo.toml").relative_to(ROOT)))
        for name in checked:
            found = SOLVE_ENTRIES.search((ROOT / name).read_text(encoding="utf-8"))
            assert found is None, f"{name} (read for {path}) names {found.group(0)}: the parity vectors may now be solved"


# ----------------------------------------------------------------------------- pins on individual files
# A fixed table in a consumer that requires the current bytes of the files it names to have a recorded SHA-256. Each
# PIN_TABLES row has a reader that extracts every target path the table names by the consumer's own path-construction
# rules: dictionary keys, tuple fields, a mode substituted into a filename template, an include_str! literal, a
# `successor` or `path` value. It never reads, evaluates or compares a digest, so a digest written as a literal, a
# named constant or any other expression names the same targets, and the registration a pinned file needs stays
# required while the consumer's digest is stale (the failure this exists for: the three mixed source requests changed
# and the O4 twins' table did not). A reader recognises one complete construct and refuses, by consumer and table,
# any path-bearing form it does not support (a computed key, an unpacked mapping, a non-literal path field, a changed
# construction rule) rather than dropping it. It is a reader of these registered forms, not an interpreter: nothing in
# a consumer is executed. The tables are those the independent review found on inventoried files; a digest held
# anywhere else is not covered.


class UnsupportedPinForm(AssertionError):
    """A registered table holds a path-bearing form its reader does not support."""


def read_table(consumer: str, table: str, reader, text: str) -> set[str]:
    try:
        return reader(text)
    except UnsupportedPinForm as refused:
        raise UnsupportedPinForm(f"{consumer} ({table}): {refused}") from None


def _require(condition, what: str):
    if not condition:
        raise UnsupportedPinForm(what)


def _block(text: str, start: str) -> str:
    """The text from the line starting `start` to the next line that closes it at the left margin."""
    found = re.search(rf"^{start}.*?^[\]\}}]", text, re.M | re.S)
    _require(found, f"no {start!r} at the left margin")
    return found.group(0)


def _relative(consumer: str, path: str) -> str:
    return str((ROOT / consumer).parent.joinpath(path).resolve().relative_to(ROOT))


# -- Python: dictionary declarations are parsed, never evaluated; only literal string keys are read.
def _py_keys(text: str, name: str) -> list[str]:
    """The literal string keys of the one module-level `name = {...}`. A computed key, an unpacked mapping, a second
    binding or any later change to `name` (item assignment, augmented assignment, a mutating method) is refused."""
    tree = ast.parse(text)
    bound = [n for n in tree.body if isinstance(n, (ast.Assign, ast.AnnAssign))
             and any(isinstance(t, ast.Name) and t.id == name for t in (n.targets if isinstance(n, ast.Assign) else [n.target]))]
    _require(len(bound) == 1, f"{name} is bound {len(bound)} times at module level; one dictionary literal is supported")
    value = bound[0].value
    _require(isinstance(value, ast.Dict), f"{name} is not a dictionary literal")
    for node in ast.walk(tree):
        targets = node.targets if isinstance(node, ast.Assign) else [node.target] if isinstance(node, (ast.AugAssign, ast.AnnAssign)) else []
        for target in targets:
            _require(not (isinstance(target, ast.Subscript) and isinstance(target.value, ast.Name) and target.value.id == name),
                     f"{name} gains an entry by item assignment (line {node.lineno})")
            _require(not (isinstance(node, ast.AugAssign) and isinstance(target, ast.Name) and target.id == name),
                     f"{name} is changed by augmented assignment (line {node.lineno})")
        if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) and isinstance(node.func.value, ast.Name) \
                and node.func.value.id == name:
            _require(node.func.attr not in {"update", "setdefault", "__setitem__", "pop", "popitem", "clear", "__ior__"},
                     f"{name} is changed by .{node.func.attr}() (line {node.lineno})")
    keys = []
    for key in value.keys:
        _require(key is not None, f"{name} unpacks a mapping (**) at line {value.lineno}")
        _require(isinstance(key, ast.Constant) and isinstance(key.value, str),
                 f"{name} has a computed key at line {key.lineno}: {ast.unparse(key)}")
        keys.append(key.value)
    return keys


def _source_line(text: str, line: str) -> None:
    _require(re.search(rf"^{re.escape(line)}$", text, re.M), f"the path-construction rule {line!r} is not as registered")


def _twins(text):
    """PINS and PINS_ADMITTED: repository paths as literal keys; check_pins hashes exactly PINS's keys."""
    _require("    if set(PINS) != set(input_files()):\n" in text
             and "if got != PINS[path] and got != PINS_ADMITTED.get(path):" in text,
             "check_pins no longer hashes exactly PINS's keys against PINS and PINS_ADMITTED")
    return set(_py_keys(text, "PINS")) | set(_py_keys(text, "PINS_ADMITTED"))


def _u5a(text):
    """U5A_PINNED: literal keys relative to t4_u5a, which the generator reaches as ROOT.parent / "t4_u5a"."""
    _source_line(text, "ROOT = Path(__file__).resolve().parent.parent")
    _source_line(text, 'U5A = ROOT.parent / "t4_u5a"')
    _require(re.search(r"for rel, h in U5A_PINNED\.items\(\):\n\s+b = \(U5A / rel\)\.read_bytes\(\)", text),
             "U5A_PINNED's keys are no longer read as paths under U5A")
    return {f"validation/references/t4_u5a/{key}" for key in _py_keys(text, "U5A_PINNED")}


def _u4a(text):
    """The U4a cross-check: every os.path.join(U4A_DIR, <literal>) in u4a_cross_check, U4A_DIR being t4_u4a."""
    _source_line(text, "HERE = os.path.dirname(os.path.abspath(__file__))")
    _source_line(text, "ROOT = os.path.dirname(HERE)")
    _source_line(text, 'U4A_DIR = os.path.join(os.path.dirname(ROOT), "t4_u4a")')
    function = [n for n in ast.parse(text).body if isinstance(n, ast.FunctionDef) and n.name == "u4a_cross_check"]
    _require(len(function) == 1, "no single u4a_cross_check")
    names = []
    for node in ast.walk(function[0]):
        if isinstance(node, ast.Call) and any(isinstance(a, ast.Name) and a.id == "U4A_DIR" for a in node.args):
            _require(ast.unparse(node.func) == "os.path.join" and len(node.args) == 2 and isinstance(node.args[0], ast.Name)
                     and node.args[0].id == "U4A_DIR"
                     and isinstance(node.args[1], ast.Constant) and isinstance(node.args[1].value, str),
                     f"U4A_DIR is used in an unsupported path form at line {node.lineno}: {ast.unparse(node)}")
            names.append(node.args[1].value)
    uses = sum(isinstance(n, ast.Name) and n.id == "U4A_DIR" for n in ast.walk(function[0]))
    _require(uses == len(names), "U4A_DIR is used outside os.path.join(U4A_DIR, <literal>)")
    return {f"validation/references/t4_u4a/{name}" for name in names}


def _b3_successors(text):
    """EXACT_PINNED's literal keys are solver modes; m3x_producer substitutes each into its one filename template."""
    modes = _py_keys(text, "EXACT_PINNED")
    function = [n for n in ast.parse(text).body if isinstance(n, ast.FunctionDef) and n.name == "m3x_producer"]
    _require(len(function) == 1, "no single m3x_producer")
    templates = [n for n in ast.walk(function[0]) if isinstance(n, ast.JoinedStr)]
    _require(len(templates) == 1, f"m3x_producer builds {len(templates)} f-strings; one filename template is supported")
    parts = []
    for value in templates[0].values:
        if isinstance(value, ast.Constant):
            parts.append(value.value)
        else:
            _require(isinstance(value, ast.FormattedValue) and isinstance(value.value, ast.Name) and value.value.id == "mode"
                     and value.conversion == -1 and value.format_spec is None,
                     f"the filename template substitutes something other than mode: {ast.unparse(templates[0])}")
            parts.append("{mode}")
    template = "".join(parts)
    _require(template.count("{mode}") == 1 and "EXACT_PINNED[mode]" in ast.unparse(function[0]),
             f"the template {template!r} is not keyed by EXACT_PINNED's mode")
    return {template.replace("{mode}", mode) for mode in modes}


def _frozen(text):
    """FROZEN: repository paths as literal keys, whatever each value is (a literal or a named constant), each hashed
    by the loop over FROZEN.items()."""
    _require(re.search(r"^    for path, digest in FROZEN\.items\(\):\n\s+assert hashlib\.sha256\(\(PROJECT / path\)\.read_bytes\(\)\)"
                       r"\.hexdigest\(\) == digest, path$", text, re.M), "FROZEN is no longer hashed by its loop as registered")
    return set(_py_keys(text, "FROZEN"))


# -- Rust and TypeScript: array and object literals are split at their top level; a path-bearing field must be the
# literal form the reader names.
_LITERAL = re.compile(r'"([^"\\]*)"')
# What a scan skips as a unit: string literals and comments. Rust adds character literals (not lifetimes); TypeScript
# adds single-quoted and template strings.
_SKIP = {"rust": re.compile(r'"(?:[^"\\]|\\.)*"|//[^\n]*|/\*.*?\*/|\'(?:[^\'\\\n]|\\.)\'', re.S),
         "ts": re.compile(r'"(?:[^"\\]|\\.)*"|\'(?:[^\'\\]|\\.)*\'|`(?:[^`\\]|\\.)*`|//[^\n]*|/\*.*?\*/', re.S)}


def _scan(text: str, start: int, lang: str):
    """(index, character) of each character from `start` that is outside string literals and comments."""
    i = start
    while i < len(text):
        skipped = _SKIP[lang].match(text, i) if text[i] in "\"'`/" else None
        if skipped and skipped.end() > i:
            i = skipped.end()
            continue
        yield i, text[i]
        i += 1


def _enclosed(text: str, opening: int, lang: str) -> str:
    """The text inside the bracket that opens at `opening`, through its match."""
    pairs, depth = {"[": "]", "(": ")", "{": "}"}, []
    for i, c in _scan(text, opening, lang):
        if c in pairs:
            depth.append(pairs[c])
        elif c in ")]}":
            _require(depth and depth.pop() == c, "unbalanced brackets")
            if not depth:
                return text[opening + 1:i]
    raise UnsupportedPinForm("an unclosed bracket")


def _top_level(inner: str, lang: str) -> list[str]:
    """`inner` split at its top-level commas, comments removed, each part stripped; a trailing comma gives no part."""
    parts, depth, start = [], 0, 0
    for i, c in _scan(inner, 0, lang):
        depth += c in "([{"
        depth -= c in ")]}"
        if c == "," and depth == 0:
            parts.append(inner[start:i])
            start = i + 1
    parts.append(inner[start:])
    cleaned = [_uncomment(part, lang).strip() for part in parts]
    return [part for part in cleaned if part]


def _uncomment(text: str, lang: str) -> str:
    """`text` with its comments removed and its string literals kept."""
    out, i = [], 0
    for found in _SKIP[lang].finditer(text):
        if found.start() < i:
            continue
        out.append(text[i:found.start()])
        if not found.group(0).startswith(("//", "/*")):
            out.append(found.group(0))
        i = found.end()
    out.append(text[i:])
    return "".join(out)


def _rust_array(text: str, opener: str) -> tuple[int | None, list[str]]:
    """(declared length or None, elements) of the array literal whose opening `[` ends the regex `opener`."""
    code = _uncomment(text, "rust")
    found = list(re.finditer(opener, code))
    _require(len(found) == 1, f"{len(found)} matches of {opener!r}; one array literal is supported")
    declared = found[0].groupdict().get("n")
    return (int(declared) if declared else None), _top_level(_enclosed(code, found[0].end() - 1, "rust"), "rust")


def _fields(element: str) -> list[str]:
    _require(element.startswith("(") and element.endswith(")"), f"an element that is not a tuple: {element[:80]}")
    return _top_level(element[1:-1], "rust")


def _macro_literal(field: str, macro: str = "include_str") -> str | None:
    """The path of `macro!("<literal>")`, None when the field names no such macro; any other form of it is refused."""
    if f"{macro}!" not in field:
        return None
    found = re.fullmatch(rf'{macro}!\(\s*"([^"\\]*)"\s*\)', field)
    _require(found, f"{macro}! with a non-literal path: {field[:80]}")
    return found.group(1)


def _const_include(text: str, name: str) -> str:
    """The literal path of the module-level `const NAME: &str = include_str!("...");`."""
    found = re.findall(rf"^const {name}: &str = (.*?);$", _uncomment(text, "rust"), re.M | re.S)
    _require(len(found) == 1, f"const {name} is not declared once")
    path = _macro_literal(found[0].strip())
    _require(path is not None, f"const {name} is not include_str! of a literal path")
    return path


def _source_basis(text):
    """CURRENT_SOURCE_BASIS: each row's family, name and mode, through Family::request_path and the raw-output format."""
    for rule in ('format!("fixtures/product_preview/pressure_source/{}/{name}.request.json", self.name())',
                 '"fixtures/product_preview/pressure_source/{}/{name}-{}.raw.json",'):
        _require(rule in text, f"the path-construction rule {rule!r} is not as registered")
    _require(re.search(r'if self\.resolved\(\) \{\s*"load_reference_source"\s*\} else \{\s*"physics_source"\s*\}', text)
             and "matches!(self, Self::LoadReference)" in text, "Family's directory names are not as registered")
    n, rows = _rust_array(text, r"const CURRENT_SOURCE_BASIS: \[.*?; (?P<n>\d+)\] = \[")
    _require(n == len(rows), f"declares {n} rows and holds {len(rows)}")
    family = {"Family::Physics": "physics_source", "Family::LoadReference": "load_reference_source"}
    mode = {"PreviewSolverMode::SparseInteractive": "sparse_interactive", "PreviewSolverMode::DenseScrutiny": "dense_scrutiny"}
    base, paths = "fixtures/product_preview/pressure_source", set()
    for row in rows:
        fields = _fields(row)
        _require(len(fields) == 6 and fields[0] in family and _LITERAL.fullmatch(fields[1]) and fields[3] in mode,
                 f"a row not of the form (Family::<variant>, \"<name>\", <digest>, PreviewSolverMode::<mode>, <length>, <digest>): {row[:120]}")
        name = _LITERAL.fullmatch(fields[1]).group(1)
        paths |= {f"{base}/{family[fields[0]]}/{name}.request.json",
                  f"{base}/{family[fields[0]]}/{name}-{mode[fields[3]]}.raw.json"}
    return paths


def _witness_inputs(text):
    """b3b_witness_inputs_and_routes: each (name, CONST, digest) in its loop, CONST an include_str! of a literal path."""
    block = _block(text, "fn b3b_witness_inputs_and_routes")
    _require("assert_eq!(sha(text.as_bytes()), file_sha" in block, "the witness loop no longer hashes the included text")
    _, rows = _rust_array(block, r"for \(name, text, file_sha\) in \[")
    paths = set()
    for row in rows:
        fields = _fields(row)
        _require(len(fields) == 3 and _LITERAL.fullmatch(fields[0]) and re.fullmatch(r"[A-Z0-9_]+", fields[1]),
                 f"a witness row not of the form (\"<name>\", <CONST>, <digest>): {row[:120]}")
        paths.add(_relative(FACADE, _const_include(text, fields[1])))
    return paths


def _include_array(consumer: str, start: str, const: str):
    """The const array `const` inside the block at `start`: each element, bare or a tuple, holds exactly one
    include_str! of a literal path, and the declared length equals the elements."""
    def read(text):
        n, elements = _rust_array(_block(text, start), rf"const {const}: \[.*?; (?P<n>\d+)\] = \[")
        _require(n == len(elements), f"{const} declares {n} elements and holds {len(elements)}")
        paths = set()
        for element in elements:
            fields = _fields(element) if element.startswith("(") else [element]
            found = [p for p in (_macro_literal(f) for f in fields) if p is not None]
            _require(len(found) == 1, f"a {const} element without exactly one include_str! literal: {element[:120]}")
            paths.add(_relative(consumer, found[0]))
        return paths
    return read


def _census(text):
    """INPUTS, the include_str! of a literal path, hashed against j7_rf_census_expected.json's inputs_sha256."""
    _require('pin["inputs_sha256"], sha(INPUTS.as_bytes())' in text, "INPUTS is no longer what inputs_sha256 pins")
    return {_relative(J7_CENSUS, _const_include(text, "INPUTS"))}


def _j7_i12(text):
    """fn request: every include_bytes! of a literal path, hashed as `bytes`."""
    block = _uncomment(_block(text, "fn request"), "rust")
    _require("assert_eq!(sha(bytes)," in block, "fn request no longer hashes the included bytes")
    arguments = [_enclosed(block, m.end() - 1, "rust").strip() for m in re.finditer(r"include_bytes!\s*\(", block)]
    _require(len(arguments) == 1, f"fn request includes {len(arguments)} byte files; one is supported")
    _require(_LITERAL.fullmatch(arguments[0]), f"include_bytes! with a non-literal path: {arguments[0][:80]}")
    return {_relative(J7_I12, _LITERAL.fullmatch(arguments[0]).group(1))}


def _eigen_motion(text):
    """EIGEN_MOTION_REQUEST: concat!(env!("CARGO_MANIFEST_DIR"), "<literal>"), hashed against EIGEN_MOTION_SHA256."""
    _require("hex, EIGEN_MOTION_SHA256" in text, "EIGEN_MOTION_SHA256 no longer pins the request it reads")
    found = re.findall(r"^const EIGEN_MOTION_REQUEST: &str = (.*?);$", _uncomment(text, "rust"), re.M | re.S)
    _require(len(found) == 1, "const EIGEN_MOTION_REQUEST is not declared once")
    path = re.fullmatch(r'concat!\(\s*env!\("CARGO_MANIFEST_DIR"\),\s*"(/[^"\\]*)"\s*,?\s*\)', found[0].strip())
    _require(path, f"EIGEN_MOTION_REQUEST is not concat!(env!(\"CARGO_MANIFEST_DIR\"), \"<literal>\"): {found[0][:120]}")
    return {_relative("core/product_physics/Cargo.toml", path.group(1).lstrip("/"))}


def _golden(text):
    """EXACT_GOLDEN: each `<mode>: { ... }` entry's `successor` and `path`, each a plain string literal."""
    _require("expect(sha256(bytes)).toBe(golden.successorSha256)" in text
             and "expect(sha256(goldenText)).toBe(golden.sha256)" in text, "EXACT_GOLDEN's files are no longer hashed")
    code = _uncomment(text, "ts")
    found = list(re.finditer(r"const EXACT_GOLDEN: [^=]*= \{", code))
    _require(len(found) == 1, "EXACT_GOLDEN is not declared once as an object literal")
    paths = set()
    for entry in _top_level(_enclosed(code, found[0].end() - 1, "ts"), "ts"):
        head = re.match(r"(\w+): \{", entry)
        _require(head and entry.endswith("}"), f"an EXACT_GOLDEN entry not of the form <mode>: {{ ... }}: {entry[:80]}")
        properties = {}
        for prop in _top_level(entry[head.end():-1], "ts"):
            named = re.match(r"(\w+):\s*(.*)$", prop, re.S)
            _require(named, f"an EXACT_GOLDEN property not of the form <name>: <value>: {prop[:80]}")
            properties[named.group(1)] = named.group(2).strip()
        for key in ("successor", "path"):
            _require(key in properties and _LITERAL.fullmatch(properties[key]),
                     f"{head.group(1)}.{key} is not a plain string literal: {properties.get(key, '(absent)')[:80]}")
            paths.add(_LITERAL.fullmatch(properties[key]).group(1))
    return paths


TWINS = "validation/references/o4_retained_source/generators/o4_source_twins.py"
J7_CENSUS = "core/product_physics/src/source_recovery/j7_rf_census.rs"
FACADE = "core/product_physics/src/retained_facade_tests.rs"
J7_I12 = "core/product_physics/src/source_recovery/j7_t4_consequences.rs"
CONTRACT = "core/reporting/result_export/tests/retained_precision_contract.rs"
DERIVATIVE = "core/reporting/result_export/tests/retained_precision_derivative_golden.rs"
FROZEN_CONSUMER = "tests/test_load_reference_readers.py"
# (consumer, table, reader, how many paths it names, how many of those the inventory holds). The counts cross-check
# the readings against the tables as committed; they cannot show a reading is complete, which is the readers' job.
PIN_TABLES = [
    # tools/fixtures/retained_precision_corpus.py names no pressure_regions
    (TWINS, "PINS, PINS_ADMITTED", _twins, 14, 13),
    # the 24 committed raw outputs name no pressure_regions
    ("core/product_physics/tests/pressure_source_join.rs", "CURRENT_SOURCE_BASIS", _source_basis, 36, 12),
    (FACADE, "b3b_witness_inputs_and_routes", _witness_inputs, 3, 3),
    ("core/product_physics/tests/load_reference_state_runtime_extension.rs", "EIGEN_MOTION_SHA256", _eigen_motion, 1, 1),
    # oracle.json names no pressure_regions
    ("validation/references/t4_u4b/generators/u4b_reference.py", "U4A_REQUESTS_SHA256", _u4a, 2, 1),
    ("validation/references/t4_u5b/generators/u5b_reference.py", "U5A_PINNED", _u5a, 10, 10),
    (J7_CENSUS, "inputs_sha256", _census, 1, 1),
    (J7_I12, "request", _j7_i12, 1, 1),
    # the four raw outputs, the pressure-3 table and the transport schema name no pressure_regions
    (FROZEN_CONSUMER, "FROZEN", _frozen, 8, 2),
    (FACADE, "EXACT_PINNED", _include_array(FACADE, "fn b3b_exact_successor_fixtures_are_the_live_successors", "FIXTURES"), 2, 2),
    (CONTRACT, "M3X", _include_array(CONTRACT, "const M3X", "M3X"), 2, 2),
    (DERIVATIVE, "EXACT_PINNED", _include_array(DERIVATIVE, "const EXACT_PINNED", "EXACT_PINNED"), 2, 2),
    ("tests/test_retained_precision_b3.py", "EXACT_PINNED", _b3_successors, 2, 2),
    # the two derivative goldens name no pressure_regions
    ("apps/desktop/src/features/result-export/retainedPrecisionResultExport.test.tsx", "EXACT_GOLDEN", _golden, 4, 2),
]


def _names(pin: str, consumer: str) -> bool:
    """A registered pin names `consumer`: the bare path (a group's) or the path and its table."""
    return pin == consumer or pin.startswith(consumer + " ")


def registrations(inventory: dict) -> dict[str, list[str]]:
    """Every inventoried file's registered pins, whether its claim is a group or an exemption."""
    named = {e["path"]: e["pinned_by"] for e in inventory["entries"]}
    for record in inventory["exemptions"]:
        named.update({path: record.get("pinned_by", {}).get(path, []) for path in record["files"]})
    return named


def unregistered(inventory: dict, root: Path = ROOT) -> list[tuple[str, str]]:
    """(file, consumer) for every inventoried file a PIN_TABLES table names whose registration does not name that
    consumer, reading the tables' consumers under `root`. A table its reader refuses raises UnsupportedPinForm."""
    named = registrations(inventory)
    missing = []
    for consumer, table, reader, _, _ in PIN_TABLES:
        for path in sorted(read_table(consumer, table, reader, (root / consumer).read_text(encoding="utf-8")) & set(named)):
            if not any(_names(pin, consumer) for pin in named[path]):
                missing.append((path, consumer))
    return missing


@pytest.mark.parametrize("consumer, table, reader, total, held", PIN_TABLES,
                         ids=[f"{c.rsplit('/', 1)[1]}:{t}" for c, t, *_ in PIN_TABLES])
def test_every_inventoried_file_a_fixed_pin_table_names_names_that_table(scanned, consumer, table, reader, total, held):
    """A change to a pinned file without moving its pin fails the consumer's check, so every pinned file the inventory
    holds must say which consumer pins it, whether its claim is a group or an exemption. The reader refuses a form it
    does not support; the counts cross-check the reading against the table as committed."""
    pinned = read_table(consumer, table, reader, (ROOT / consumer).read_text(encoding="utf-8"))
    assert len(pinned) == total and all((ROOT / p).exists() for p in pinned), sorted(pinned)
    named = registrations(INV.build(scanned=scanned))
    inventoried = sorted(pinned & set(named))
    assert len(inventoried) == held, inventoried
    for path in inventoried:
        assert any(_names(pin, consumer) for pin in named[path]), (path, consumer, table)


def _scratch_consumers(tmp_path: Path, *also: str) -> None:
    for path in {c for c, *_ in PIN_TABLES} | set(also):
        (tmp_path / path).parent.mkdir(parents=True, exist_ok=True)
        (tmp_path / path).write_bytes((ROOT / path).read_bytes())


def test_a_pinned_file_changed_under_a_stale_table_still_needs_its_registration(scanned, tmp_path):
    """The failure that happened, in a scratch copy: an inventoried input changes and its consumer's table does not, so
    the consumer's digest no longer matches. Its registration is still demanded, and only the registration decides."""
    changed = "fixtures/product_preview/pressure_source/physics_source/mixed.request.json"
    consumer = "core/product_physics/tests/pressure_source_join.rs"
    _scratch_consumers(tmp_path, changed)
    document = json.loads((tmp_path / changed).read_text(encoding="utf-8"))
    del document["model"]["model_settings"]["pressure_elongation"]  # the change that moved it, in reverse
    (tmp_path / changed).write_text(json.dumps(document, indent=1) + "\n", encoding="utf-8")
    recorded = re.search(r'\(Family::Physics, "mixed", "([0-9a-f]{64})"', (tmp_path / consumer).read_text()).group(1)
    assert hashlib.sha256((tmp_path / changed).read_bytes()).hexdigest() != recorded  # the table is now stale
    assert unregistered(INV.build(scanned=scanned), tmp_path) == []
    dropped = [e for e in INV.PINNED_BY if not any(p.startswith(consumer) for p in e["pinned_by"])]
    assert len(dropped) == len(INV.PINNED_BY) - 1
    assert (changed, consumer) in unregistered(INV.build(scanned=scanned, pinned=dropped), tmp_path)


def test_a_target_pinned_through_a_named_constant_still_needs_its_registration(scanned, tmp_path):
    """F5, in a scratch copy: an inventoried file is added to FROZEN with its digest held in a named constant, as two
    of FROZEN's committed entries are. The reader takes the key whatever the value is, so the missing registration is
    named."""
    added = "fixtures/results/j7_rf_census_inputs.json"
    _scratch_consumers(tmp_path)
    text = (tmp_path / FROZEN_CONSUMER).read_text(encoding="utf-8")
    digest = hashlib.sha256((ROOT / added).read_bytes()).hexdigest()
    mutated = text.replace("FROZEN = {\n", f'REVIEW_PIN = "{digest}"\nFROZEN = {{\n    "{added}": REVIEW_PIN,\n', 1)
    assert mutated != text
    (tmp_path / FROZEN_CONSUMER).write_text(mutated, encoding="utf-8")
    assert added in read_table(FROZEN_CONSUMER, "FROZEN", _frozen, mutated)
    assert unregistered(INV.build(scanned=scanned), tmp_path) == [(added, FROZEN_CONSUMER)]


@pytest.mark.parametrize("form, mutate", [
    ("computed key", lambda t: t.replace("FROZEN = {\n", 'FROZEN = {\n    "fixtures/results/" + "j7_rf_census_inputs.json": "0",\n', 1)),
    ("unpacked mapping", lambda t: t.replace("FROZEN = {\n", "MORE = {}\nFROZEN = {\n    **MORE,\n", 1)),
    ("item assignment", lambda t: t + '\nFROZEN["fixtures/results/j7_rf_census_inputs.json"] = "0"\n'),
])
def test_an_unsupported_path_form_is_refused_by_consumer_and_table(form, mutate):
    """A path-bearing form the FROZEN reader does not support refuses, naming the consumer and the table, rather than
    dropping out of the result."""
    text = (ROOT / FROZEN_CONSUMER).read_text(encoding="utf-8")
    mutated = mutate(text)
    assert mutated != text
    with pytest.raises(UnsupportedPinForm, match=rf"^{re.escape(FROZEN_CONSUMER)} \(FROZEN\): "):
        read_table(FROZEN_CONSUMER, "FROZEN", _frozen, mutated)


def test_a_stale_pin_fails(scanned):
    stale = [*INV.PINNED_BY, {"paths": ["fixtures/no_such_fixture.json"], "pinned_by": ["somewhere"]}]
    assert "stale: PINNED_BY path fixtures/no_such_fixture.json matches no tracked file naming pressure_regions" in \
        problems(scanned, pinned=stale)
