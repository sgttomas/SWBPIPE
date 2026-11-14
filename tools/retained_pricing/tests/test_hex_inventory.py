"""The closed inventory of hex placeholders (base/hex_inventory.py, base/hex_inventory.json) and its reconciliation in
pricing: every one of the priced sources' 47 hex placeholders reconciles; a missing entry, an extra entry, a stale pin, a
one-character change in a pinned function, in a use line or in a named dependency each stop the run; a re-key carried by
the line map with every pin unchanged prices; and text_budget.py sizes hex placeholders only through the inventory.
"""
import ast
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
ROOT = PRICING.parents[1]
sys.path.insert(0, str(PRICING))
sys.path.insert(0, str(PRICING / "base"))
import hex_inventory as HI  # noqa: E402
import price  # noqa: E402
from base.loopscan import off_build_files  # noqa: E402

F = "core/x/src/lib.rs"
SOURCE = """use sha2::{Digest, Sha256};

pub(crate) struct Holder {
    pub bits: u64,
}

fn wire(parts: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    for part in parts { hasher.update(part); }
    format!("{:x}", hasher.finalize())
}

fn scalar(h: &Holder) -> String {
    let value: u64 = h.bits;
    format!("{:016x}", value)
}
"""
WIRE_LINE, SCALAR_LINE, HOLDER_LINE = 10, 15, 3
ENTRIES = [
    {"site": f"{F}:{WIRE_LINE}", "row_kind": "format", "spec": "x", "arg": "hasher.finalize()", "size": 64, "kind": "hash",
     "reason": "SHA-256 streaming hasher", "premises": ["hasher is bound once, by Sha256::new()"], "depends": [], "pin": {}},
    {"site": f"{F}:{SCALAR_LINE}", "row_kind": "format", "spec": "016x", "arg": "value", "size": 16, "kind": "scalar",
     "reason": "u64 by Holder.bits", "premises": ["the field bits of struct Holder is a u64"],
     "depends": [{"key": f"{F}:{HOLDER_LINE}", "form": "item", "note": "struct Holder"}], "pin": {}},
]


def rows_of(root, files):
    out = subprocess.run([sys.executable, str(PRICING / "base/template_inventory.py"), "."] + files, cwd=root,
                         capture_output=True, text=True, check=True).stdout
    return json.loads(out)["rows"]


def fixture(tmp_path, source=SOURCE, entries=ENTRIES, revision="0" * 40):
    """A source tree holding `source` at F and an inventory of `entries` with its pins recorded from that tree."""
    root = tmp_path / "src"
    (root / F).parent.mkdir(parents=True, exist_ok=True)
    (root / F).write_text(source)
    inv = tmp_path / "hex_inventory.json"
    inv.write_text(json.dumps({"about": "fixture", "revision": revision, "entries": json.loads(json.dumps(entries))}))
    HI.record(str(root), str(inv))
    return root, inv


def check(root, inv):
    return HI.check(str(inv), str(root), rows_of(root, [F]))


def kinds(result):
    return sorted({k for k, _ in result.problems})


# ------------------------------------------------------------------ the priced sources

def test_every_hex_placeholder_of_the_priced_sources_reconciles(tmp_path):
    """The run's own placeholder set (the template inventory of the priced crates, as price.py builds it, every row at
    any multiplicity) against the committed inventory carried to HEAD by the line map, as price.py carries it."""
    inv_rows = tmp_path / "inventory.json"
    price.template_inventory(str(ROOT), str(PRICING / "base"), off_build_files(str(ROOT)), out=str(inv_rows))
    rows = json.loads(inv_rows.read_text())["rows"]
    carried, report, ok = price.carry_hex_inventory(str(ROOT), "HEAD", str(tmp_path / "map_hex"))
    assert ok, report
    result = HI.check(carried, str(ROOT), rows)
    assert result.problems == []
    entries = json.loads(Path(carried).read_text())["entries"]
    assert len(HI.hex_placeholders(rows)) == len(entries) == len(result.sizes) == 47
    assert sum(e["kind"] == "hash" for e in entries) == 21 and sum(e["kind"] == "scalar" for e in entries) == 26
    assert all(e["size"] == 64 for e in entries if e["kind"] == "hash")
    assert [e["site"].rsplit("/", 1)[1] for e in entries if e.get("bounded") is False] == ["tests_resource.rs:575"]
    assert result.unbounded == {e["site"] for e in entries if e.get("bounded") is False}


def test_the_argument_key_is_g6s_audit_key():
    module = ast.parse((PRICING / "base/text_budget.py").read_text())
    fn = next(n for n in module.body if isinstance(n, ast.FunctionDef) and n.name == "audit_key")
    ns = {"re": __import__("re")}
    exec(compile(ast.Module(body=[fn], type_ignores=[]), "audit_key", "exec"), ns)
    args = [e["arg"] for e in json.loads((PRICING / "base/hex_inventory.json").read_text())["entries"]]
    for arg in args + ["a .b( c ) // note\n", "x"]:
        for spec in ("x", "016x", "x?"):
            assert HI.arg_key(arg, spec) == ns["audit_key"](arg, spec)


# ------------------------------------------------------------------ reconciliation and pins

def test_the_fixture_reconciles(tmp_path):
    result = check(*fixture(tmp_path))
    assert result.problems == []
    assert HI.size(result, f"{F}:{WIRE_LINE}", "hasher.finalize()", "x") == 64
    assert HI.size(result, f"{F}:{SCALAR_LINE}", "value", "016x") == 16


def test_a_placeholder_with_no_entry_stops(tmp_path):
    result = check(*fixture(tmp_path, entries=ENTRIES[:1]))
    assert kinds(result) == ["hex-unregistered"]
    assert HI.size(result, f"{F}:{SCALAR_LINE}", "value", "016x") is None


def test_an_entry_with_no_placeholder_stops(tmp_path):
    extra = dict(ENTRIES[1], site=f"{F}:{WIRE_LINE}", arg="other.bits", depends=[])
    result = check(*fixture(tmp_path, entries=ENTRIES + [extra]))
    assert kinds(result) == ["hex-stale-entry"]


def test_a_stale_pin_stops(tmp_path):
    root, inv = fixture(tmp_path)
    data = json.loads(inv.read_text())
    data["entries"][0]["pin"]["fn_sha256"] = "0" * 64
    inv.write_text(json.dumps(data))
    result = check(root, inv)
    assert kinds(result) == ["hex-pin"]
    assert HI.size(result, f"{F}:{WIRE_LINE}", "hasher.finalize()", "x") is None


@pytest.mark.parametrize("old,new,kind", [
    ("for part in parts {", "for part in partz {", "hex-pin"),                        # one character in the hash's function
    ("let value: u64 = h.bits;", "let value: u64 = h.bitz;", "hex-pin"),               # one character in the scalar's function
    ("use sha2::{Digest, Sha256};", "use sha2::{Digest, Sha256 };", "hex-pin"),        # one character in a use line
    ("use sha2::{Digest, Sha256};", "use sha2::{Digest, Sha512 as Sha256};", "hex-pin"),  # an alias in the use line
    ("    pub bits: u64,", "    pub bits: u128,", "hex-dependency-pin"),              # a named dependency outside the pin
])
def test_a_change_to_audited_text_stops(tmp_path, old, new, kind):
    root, inv = fixture(tmp_path)
    (root / F).write_text(SOURCE.replace(old, new, 1))
    assert kind in kinds(check(root, inv))


def test_a_scalars_binding_changed_to_a_digest_stops(tmp_path):
    """The case that matters most: the placeholder and its argument name are unchanged, the value is now a digest."""
    root, inv = fixture(tmp_path)
    changed = SOURCE.replace("let value: u64 = h.bits;", "let value = Sha256::digest(&h.bits.to_le_bytes());", 1)
    assert 'format!("{:016x}", value)' in changed
    (root / F).write_text(changed)
    result = check(root, inv)
    assert kinds(result) == ["hex-pin"]
    assert HI.size(result, f"{F}:{SCALAR_LINE}", "value", "016x") is None


def test_a_hash_site_changed_to_a_wider_hash_stops(tmp_path):
    root, inv = fixture(tmp_path)
    (root / F).write_text(SOURCE.replace("let mut hasher = Sha256::new();", "let mut hasher = sha2::Sha512::new();", 1))
    result = check(root, inv)
    assert kinds(result) == ["hex-pin"]
    assert HI.size(result, f"{F}:{WIRE_LINE}", "hasher.finalize()", "x") is None


def test_the_identity_is_the_site_not_the_argument_text(tmp_path):
    """A new placeholder whose argument and spec equal an audited entry's, at another site, has no entry."""
    other = "\nfn other(h: &Holder) -> String {\n    let value: u64 = h.bits;\n    format!(\"{:016x}\", value)\n}\n"
    root, inv = fixture(tmp_path)
    (root / F).write_text(SOURCE + other)
    result = check(root, inv)
    assert kinds(result) == ["hex-unregistered"]


def test_a_changed_spec_stops(tmp_path):
    root, inv = fixture(tmp_path)
    (root / F).write_text(SOURCE.replace('format!("{:016x}", value)', 'format!("{:032x}", value)', 1))
    assert kinds(check(root, inv)) == ["hex-stale-entry", "hex-unregistered"]


def test_a_second_identical_placeholder_at_a_site_stops(tmp_path):
    root, inv = fixture(tmp_path)
    (root / F).write_text(SOURCE.replace('format!("{:016x}", value)', 'format!("{:016x}{:016x}", value, value)', 1))
    assert kinds(check(root, inv)) == ["hex-count", "hex-pin"]   # the edit also changes the function's text


def test_a_scalar_entry_naming_a_digest_trips_the_second_tripwire(tmp_path):
    source = SOURCE.replace("let value: u64 = h.bits;\n    format!(\"{:016x}\", value)",
                            "let digest_bits: u64 = h.bits;\n    format!(\"{:016x}\", digest_bits)", 1)
    entries = [ENTRIES[0], dict(ENTRIES[1], arg="digest_bits")]
    assert kinds(check(*fixture(tmp_path, source=source, entries=entries))) == ["hex-tripwire"]


def test_an_unbounded_entry_stops_at_a_positive_multiplicity(tmp_path):
    entries = [ENTRIES[0], dict(ENTRIES[1], bounded=False)]
    result = check(*fixture(tmp_path, entries=entries))
    assert result.problems == []
    assert HI.positive_unbounded(result, f"{F}:{SCALAR_LINE}", 0) is None
    assert HI.positive_unbounded(result, f"{F}:{SCALAR_LINE}", 3) == ("hex-unbounded-positive", f"{F}:{SCALAR_LINE}")
    assert HI.positive_unbounded(result, f"{F}:{WIRE_LINE}", 3) is None


# The shapes the two readings named: each new placeholder of such a shape has no entry, so it stops.
READING_SHAPES = {
    "a comment between finalize and its call": "hasher.finalize /* c */ ()",
    "an inline wider hasher": "sha2::Sha512::new().finalize()",
    "a field receiver": "self_digest.finalize()",
    "parentheses": "(hasher.finalize())",
    "whitespace before the call": "hasher .finalize()",
    "a bare digest name": "digest",
    "an aliased direct digest": "Sha256::digest(b\"x\")",
}


@pytest.mark.parametrize("name", sorted(READING_SHAPES))
def test_a_new_placeholder_of_a_shape_the_readings_named_stops(tmp_path, name):
    added = f"\nfn added(hasher: sha2::Sha512, self_digest: sha2::Sha512, digest: u64) -> String {{\n    format!(\"{{:x}}\", {READING_SHAPES[name]})\n}}\n"
    root, inv = fixture(tmp_path)
    (root / F).write_text(SOURCE + added)
    result = check(root, inv)
    assert kinds(result) == ["hex-unregistered"], result.problems


def test_a_same_line_shadowing_has_no_entry(tmp_path):
    added = ("\nfn shadow() -> String {\n    let hasher = Sha256::new();\n"
             "    let _ = format!(\"{:x}\", hasher.finalize()); let hasher = sha2::Sha512::new(); format!(\"{:x}\", hasher.finalize())\n}\n")
    root, inv = fixture(tmp_path)
    (root / F).write_text(SOURCE + added)
    assert "hex-unregistered" in kinds(check(root, inv))


# ------------------------------------------------------------------ re-key through the line map

P = "projects/chirality-piping/"


def git(repo, *a):
    env = dict(os.environ, GIT_AUTHOR_NAME="t", GIT_AUTHOR_EMAIL="t@example.invalid", GIT_COMMITTER_NAME="t",
               GIT_COMMITTER_EMAIL="t@example.invalid", GIT_CONFIG_GLOBAL=os.devnull, GIT_CONFIG_NOSYSTEM="1")
    return subprocess.run(["git", "-C", str(repo), *a], env=env, check=True, capture_output=True, text=True).stdout.strip()


def commit(repo, text):
    (repo / P / F).parent.mkdir(parents=True, exist_ok=True)
    (repo / P / F).write_text(text)
    git(repo, "add", "-A")
    git(repo, "commit", "-q", "-m", "v")
    return git(repo, "rev-parse", "HEAD")


def rekey(tmp_path, new_source):
    """An inventory recorded at commit A, carried to commit B (new_source) by price.py's carry, then reconciled at B."""
    repo = tmp_path / "repo"
    repo.mkdir()
    git(repo, "init", "-q")
    a = commit(repo, SOURCE)
    _, inv = fixture(tmp_path, revision=a)
    b = commit(repo, new_source)
    crates = tmp_path / "crates.txt"
    crates.write_text("core/x/src\n")
    carried, report, ok = price.carry_hex_inventory(str(repo), b, str(tmp_path / "map_hex"), src=str(inv), crates=str(crates))
    result = HI.check(carried, str(repo / P), rows_of(repo / P, [F])) if ok else None
    return report, ok, result


SHIFT = "// one\n// two\n// three\n"


def test_a_pure_shift_rekeys_records_and_prices(tmp_path):
    report, ok, result = rekey(tmp_path, SHIFT + SOURCE)
    assert ok, report
    assert result.problems == []
    moved = {(old, new) for old, new, _ in report["rekeyed"]}
    assert (f"{F}:{WIRE_LINE}", f"{F}:{WIRE_LINE + 3}") in moved
    assert (f"{F}:{SCALAR_LINE}", f"{F}:{SCALAR_LINE + 3}") in moved
    assert (f"{F}:{HOLDER_LINE}", f"{F}:{HOLDER_LINE + 3}") in moved
    assert HI.size(result, f"{F}:{WIRE_LINE + 3}", "hasher.finalize()", "x") == 64


def test_a_shift_with_a_change_in_the_function_stops(tmp_path):
    report, ok, result = rekey(tmp_path, SHIFT + SOURCE.replace("for part in parts {", "for part in partz {", 1))
    assert ok, report
    assert kinds(result) == ["hex-pin"]


def test_a_shift_with_a_change_in_a_named_dependency_stops(tmp_path):
    report, ok, result = rekey(tmp_path, SHIFT + SOURCE.replace("    pub bits: u64,", "    pub bits: u128,", 1))
    assert ok, report
    assert kinds(result) == ["hex-dependency-pin"]


def test_a_move_the_line_map_cannot_place_stops(tmp_path):
    report, ok, _ = rekey(tmp_path, SOURCE.replace('format!("{:x}", hasher.finalize())', 'format!("{:x}",  hasher.finalize())', 1))
    assert not ok
    assert any(u[1] == f"{F}:{WIRE_LINE}" for u in report["unmapped"])


# ------------------------------------------------------------------ text_budget.py's wiring

def _text_budget():
    return (PRICING / "base/text_budget.py").read_text(), ast.parse((PRICING / "base/text_budget.py").read_text())


def test_text_budget_sizes_a_hex_placeholder_only_through_the_inventory(tmp_path):
    text, module = _text_budget()
    fn = next(n for n in module.body if isinstance(n, ast.FunctionDef) and n.name == "arg_max")
    root, inv = fixture(tmp_path)
    ns = {"HEXINV": HI, "HEXR": check(root, inv), "CUR": [f"{F}:{WIRE_LINE}"], "audited": lambda a, s, lex=False: None}
    exec(compile(ast.Module(body=[fn], type_ignores=[]), "arg_max", "exec"), ns)
    assert ns["arg_max"]("x", "hasher.finalize()") == 64
    assert ns["arg_max"]("x", "anything.else()") is None
    ns["CUR"][0] = f"{F}:{SCALAR_LINE}"
    assert ns["arg_max"]("016x", "value") == 16
    ns["audited"] = lambda a, s, lex=False: 40
    assert ns["arg_max"]("016x", "value") == 40          # G6: the audit is never lowered
    assert "proved_sha256" not in text and "_INITS" not in text


def test_text_budget_feeds_every_reconciliation_problem_to_unclassified(tmp_path):
    text, module = _text_budget()
    stmts = [n for n in module.body if ast.get_source_segment(text, n).startswith(("HEXR = HEXINV.check(", "for _hex_problem in HEXR.problems:"))]
    assert len(stmts) == 2
    root, inv = fixture(tmp_path, entries=ENTRIES[:1])
    here = tmp_path / "chain"
    here.mkdir()
    shutil.copy(inv, here / "hex_inventory.json")
    ns = {"HEXINV": HI, "os": os, "proot": str(root), "inv": {"rows": rows_of(root, [F])},
          "unclassified": __import__("collections").Counter(), "__file__": str(here / "text_budget.py")}
    exec(compile(ast.Module(body=stmts, type_ignores=[]), "reconcile", "exec"), ns)
    assert [k for k, _ in ns["unclassified"]] == ["hex-unregistered"]


def test_text_budget_stops_an_unbounded_entry_at_a_positive_multiplicity(tmp_path):
    text, module = _text_budget()
    loop = next(n for n in module.body if isinstance(n, ast.For) and ast.get_source_segment(text, n.iter) == 'inv["rows"]'
                and "_hex_unbounded" in ast.get_source_segment(text, n))
    stmts = [n for n in loop.body if "_hex_unbounded" in ast.get_source_segment(text, n)]
    assert len(stmts) == 2
    entries = [ENTRIES[0], dict(ENTRIES[1], bounded=False)]
    result = check(*fixture(tmp_path, entries=entries))
    for mult, expected in ((0, []), (2, ["hex-unbounded-positive"])):
        ns = {"HEXINV": HI, "HEXR": result, "f": F, "line": SCALAR_LINE, "mult": mult,
              "unclassified": __import__("collections").Counter()}
        exec(compile(ast.Module(body=stmts, type_ignores=[]), "unbounded", "exec"), ns)
        assert [k for k, _ in ns["unclassified"]] == expected
