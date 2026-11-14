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
WIRE_LINE, SCALAR_LINE, HOLDER_LINE, USE_LINE = 10, 15, 3, 1


def entries_for(source):
    """The fixture's two entries, keyed at the lines `source` puts them on: the hash entry names its use item (form
    "use") and the names and module it resolves through it; the scalar names struct Holder."""
    lines = source.split("\n")
    at = lambda text: next(i + 1 for i, l in enumerate(lines) if text in l)
    return [
        {"site": f"{F}:{at('format!(\"{:x}\", hasher.finalize())')}", "row_kind": "format", "spec": "x",
         "arg": "hasher.finalize()", "size": 64, "kind": "hash", "reason": "SHA-256 streaming hasher",
         "premises": ["hasher is bound once, by Sha256::new()"], "names": ["Sha256", "Digest"], "modules": ["sha2"],
         "depends": [{"key": f"{F}:{at('use sha2::')}", "form": "use", "note": "use sha2::{Digest, Sha256};"}], "pin": {}},
        {"site": f"{F}:{at('format!(\"{:016x}\", value)')}", "row_kind": "format", "spec": "016x", "arg": "value",
         "size": 16, "kind": "scalar", "reason": "u64 by Holder.bits", "premises": ["the field bits of struct Holder is a u64"],
         "depends": [{"key": f"{F}:{at('struct Holder')}", "form": "item", "note": "struct Holder"}], "pin": {}},
    ]


ENTRIES = entries_for(SOURCE)


def rows_of(root, files):
    out = subprocess.run([sys.executable, str(PRICING / "base/template_inventory.py"), "."] + files, cwd=root,
                         capture_output=True, text=True, check=True).stdout
    return json.loads(out)["rows"]


def fixture(tmp_path, source=SOURCE, entries=None, revision="0" * 40, record=True):
    """A source tree holding `source` at F and an inventory of `entries` (default: entries_for(source)) with its pins
    recorded from that tree."""
    root = tmp_path / "src"
    (root / F).parent.mkdir(parents=True, exist_ok=True)
    (root / F).write_text(source)
    inv = tmp_path / "hex_inventory.json"
    entries = entries_for(source) if entries is None else entries
    inv.write_text(json.dumps({"about": "fixture", "revision": revision, "entries": json.loads(json.dumps(entries))}))
    if record:
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
    ("use sha2::{Digest, Sha256};", "use sha2::{Digest, Sha256 };", "hex-dependency-pin"),  # one character in the named use item
    ("use sha2::{Digest, Sha256};", "use sha2::{Digest, Sha512 as Sha256};", "hex-name-shadow"),  # an alias in it
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


def test_a_duplicate_entry_stops(tmp_path):
    root, inv = fixture(tmp_path, entries=ENTRIES + [ENTRIES[0]])
    assert "hex-entry-duplicate" in kinds(check(root, inv))


# ------------------------------------------------------------------ the use-item pin: only the named use items

def with_line2(source, text):
    """`source` with its blank second line (after the use item) replaced by `text`: no other line moves."""
    head = "use sha2::{Digest, Sha256};\n\n"
    assert source.count(head) == 1
    return source.replace(head, f"use sha2::{{Digest, Sha256}};\n{text}\n", 1)


@pytest.mark.parametrize("before,after", [("", "use std::fmt;"), ("use std::fmt;", ""), ("use std::fmt;", "use std::fmt::Write;")])
def test_an_unrelated_use_item_added_removed_or_edited_does_not_stop(tmp_path, before, after):
    root, inv = fixture(tmp_path, source=with_line2(SOURCE, before))
    (root / F).write_text(with_line2(SOURCE, after))
    assert check(root, inv).problems == []


def test_the_named_use_item_edited_by_one_character_stops(tmp_path):
    root, inv = fixture(tmp_path)
    (root / F).write_text(SOURCE.replace("use sha2::{Digest, Sha256};", "use sha2::{Digest,Sha256};", 1))
    assert kinds(check(root, inv)) == ["hex-dependency-pin"]


@pytest.mark.parametrize("line", ["use other::Thing as Sha256;", "struct Sha256;", "type Digest = u8;", "mod sha2 {}",
                                  "use other::Sha256;"])
def test_a_file_level_shadow_of_an_audited_name_stops(tmp_path, line):
    """The tripwires: an alias or type definition of an audited name, a local module named like its path root, and a
    second use item naming it (which would shadow a glob import)."""
    root, inv = fixture(tmp_path)
    (root / F).write_text(with_line2(SOURCE, line))
    assert kinds(check(root, inv)) == ["hex-name-shadow"]


# ------------------------------------------------------------------ the function pin's attribute boundary

ATTR = "#[cfg_attr(\n    test, allow(dead_code)\n)]\n"     # a multiline outer attribute, three lines


def test_adding_a_multiline_attribute_stops(tmp_path):
    root, inv = fixture(tmp_path, source=SOURCE.replace("fn wire(", "\n\n\nfn wire(", 1))
    (root / F).write_text(SOURCE.replace("fn wire(", ATTR + "fn wire(", 1))
    assert kinds(check(root, inv)) == ["hex-pin"]


def test_changing_only_a_multiline_attributes_contents_stops(tmp_path):
    source = SOURCE.replace("fn wire(", ATTR + "fn wire(", 1)
    root, inv = fixture(tmp_path, source=source)
    (root / F).write_text(source.replace("    test, allow", "    not(test), allow", 1))
    assert kinds(check(root, inv)) == ["hex-pin"]


def test_an_attribute_across_a_comment_is_pinned(tmp_path):
    source = SOURCE.replace("fn wire(", "#[inline]\n// a note\nfn wire(", 1)
    root, inv = fixture(tmp_path, source=source)
    (root / F).write_text(source.replace("#[inline]", "#[inline(always)]", 1))
    assert kinds(check(root, inv)) == ["hex-pin"]


def test_an_unlocatable_function_boundary_is_refused(tmp_path):
    """A qualifier on the line above `fn` is not an item end, an attribute or a block start: the boundary is not
    located safely, so the function is refused, naming the site."""
    source = SOURCE.replace("fn wire(", "pub(crate)\nfn wire(", 1)
    root, inv = fixture(tmp_path, source=source, record=False)
    site = entries_for(source)[0]["site"]
    assert (("hex-pin", f"{site}: its enclosing function or its attribute boundary cannot be located safely")
            in check(root, inv).problems)


# ------------------------------------------------------------------ the impl headers the reasons rely on

IMPL_HEADERS = ["core/product_physics/src/source_recovery.rs:201",
                "core/solver/frame_kernel/src/structural/exact_boundary/functionals.rs:947",
                "core/solver/frame_kernel/src/structural/exact_boundary/functionals.rs:1012",
                "core/solver/frame_kernel/src/structural/retained/wide.rs:246"]


@pytest.mark.parametrize("key", IMPL_HEADERS)
def test_a_change_to_an_impl_header_a_reason_relies_on_stops(tmp_path, key):
    """On the priced sources, carried to HEAD: the entries of the file whose entry names this impl header reconcile;
    one character added in the header gives hex-dependency-pin for it."""
    committed = json.loads((PRICING / "base/hex_inventory.json").read_text())["entries"]
    hits = [(i, j) for i, e in enumerate(committed) for j, d in enumerate(e["depends"]) if d["key"] == key]
    assert len(hits) == 1 and committed[hits[0][0]]["depends"][hits[0][1]]["form"] == "signature"
    carried, report, ok = price.carry_hex_inventory(str(ROOT), "HEAD", str(tmp_path / "map_hex"))
    assert ok, report
    entries = json.loads(Path(carried).read_text())["entries"]
    entry = entries[hits[0][0]]
    header = entry["depends"][hits[0][1]]["key"]
    site_file = entry["site"].rsplit(":", 1)[0]
    subset = [e for e in entries if e["site"].rsplit(":", 1)[0] == site_file]
    root = tmp_path / "src"
    for f in {site_file} | {d["key"].rsplit(":", 1)[0] for e in subset for d in e["depends"]}:
        (root / f).parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / f, root / f)
    inv = tmp_path / "subset.json"
    inv.write_text(json.dumps({"revision": "HEAD", "entries": subset}))
    assert HI.check(str(inv), str(root), rows_of(root, [site_file])).problems == []
    hf, hl = header.rsplit(":", 1)
    lines = (root / hf).read_text().split("\n")
    assert lines[int(hl) - 1].lstrip().startswith("impl")
    lines[int(hl) - 1] = lines[int(hl) - 1].replace("impl", "impl ", 1)
    (root / hf).write_text("\n".join(lines))
    result = HI.check(str(inv), str(root), rows_of(root, [site_file]))
    assert result.problems == [("hex-dependency-pin", f"{entry['site']}: {header} is not as audited")]


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


def test_a_deleted_site_stops_the_carry(tmp_path):
    """A site whose function is deleted is not carried: the line map reports its key deleted, and without pruning it
    stays unmapped, so the carry fails (price.py then stops with exit 4). Pruning it would drop the entry silently."""
    start, end = SOURCE.index("fn wire("), SOURCE.index("fn scalar(")
    report, ok, result = rekey(tmp_path, SOURCE[:start] + SOURCE[end:])
    assert not ok, {"report": report, "problems_after_carry": None if result is None else result.problems}
    assert [(u[1], u[-1]) for u in report["unmapped"]] == [(f"{F}:{WIRE_LINE}", "deleted")]


def test_the_driver_stops_with_exit_4_when_the_carry_fails(tmp_path, monkeypatch):
    """cmd_run's own stop: every stage before the carry is stubbed as successful, the carry reports a key it could not
    place, and the run ends with exit 4 before any chain is built."""
    import importlib.util
    from types import SimpleNamespace
    monkeypatch.syspath_prepend(str(PRICING))
    spec = importlib.util.spec_from_file_location("hex_price_driver_test", PRICING / "price.py")
    driver = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(driver)
    output = tmp_path / "price"
    monkeypatch.setattr(driver, "resolve_revision", lambda repo, rev: "0" * 40)
    monkeypatch.setattr(driver, "snapshot", lambda *args: str(tmp_path / "snap"))
    monkeypatch.setattr(driver, "prepare_b7a_retired_carry", lambda *args: ({}, None))

    def step(args, **kwargs):
        assert str(args[0]).endswith("linemap_rules.py"), args[0]   # the rule tables' carry, stubbed as successful
        mapped = output / "map"
        mapped.mkdir()
        for relative in driver.RULE_FILES:
            shutil.copyfile(PRICING / "base" / relative, mapped / relative)
        (mapped / "premise_pins.json").write_text(json.dumps({"pins": []}))
        return subprocess.CompletedProcess(args, 0, "", "")
    monkeypatch.setattr(driver, "py", step)
    unmapped = [["hex_inventory.json/entries[0]/site", f"{F}:{WIRE_LINE}", "line 10: text not found at NEW", "deleted"]]
    monkeypatch.setattr(driver, "carry_hex_inventory", lambda *args, **kw: (
        str(PRICING / "base" / "hex_inventory.json"), {"rekeyed": [], "unmapped": unmapped}, False))
    with pytest.raises(SystemExit) as stopped:
        driver.cmd_run(SimpleNamespace(repo=str(ROOT), rev="any", out=str(output)))
    assert stopped.value.code == 4
    assert not (output / "chain").exists()


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


# ------------------------------------------------------------------ pricing's own verdict (text_budget.py run whole)

MODEL_STUB = '''"""A pricing model stubbed by tests/test_hex_inventory.py: every witness passes and every hook declines."""
def inspect(*args, **kwargs): return None
def recipe_group(*args, **kwargs): return {"name": "none", "standalone_requested_bytes": 0, "req": 0}
def recipe_witness(*args, **kwargs): return False
def extra_requested(*args, **kwargs): return 0
def __getattr__(name): return lambda *args, **kwargs: None
'''
MODELS = ("kd5_first_error_model", "b7a_formation_model", "b7a_returning_errors", "b7a_scoped_text_model",
          "b7a_phase_model", "b7a_owner_choices")


def priced(tmp_path, extra_rows=()):
    """text_budget.py, as written, run whole on the fixture (its six pricing models stubbed, nothing reached): the run's
    JSON, whose `complete` and `unclassified_args` are pricing's own verdict."""
    root, inv = fixture(tmp_path)
    work = tmp_path / "work"
    work.mkdir()
    for f in (PRICING / "base").glob("*.py"):
        shutil.copy(f, work / f.name)
    for m in MODELS:
        (work / f"{m}.py").write_text(MODEL_STUB)
    shutil.copy(inv, work / "hex_inventory.json")
    inputs = {"inventory.json": {"rows": rows_of(root, [F]) + list(extra_rows)},
              "graph.json": {"edges": {}, "site_loops": [], "spans": {}},
              "loops.json": {"counts": {"milestone": {"l": 1}}, "loops": []},
              "table.json": {"classes": {"ident": 46, "static": 0, "f64_exp": 24, "message_lit_struct": 0}, "args": [],
                             "diag_fixed_bytes": 0, "diag_refs_default_bytes": 0}}
    for name, value in inputs.items():
        (tmp_path / name).write_text(json.dumps(value))
    env = {k: v for k, v in os.environ.items() if not k.startswith(("TB_", "G4_", "RP_"))}
    r = subprocess.run([sys.executable, str(work / "text_budget.py"), str(root)]
                       + [str(tmp_path / n) for n in ("inventory.json", "graph.json", "loops.json", "table.json")]
                       + ["no_root", "milestone"], cwd=root, env=env, capture_output=True, text=True)
    assert r.returncode == 0, r.stderr[-3000:]
    return json.loads(r.stdout)


def test_pricing_completes_when_every_hex_placeholder_reconciles(tmp_path):
    out = priced(tmp_path)
    assert out["complete"] and out["unclassified_args"] == []


def test_a_new_placeholder_that_arg_max_never_sizes_is_refused_by_pricing(tmp_path):
    """The reconciliation is the only refusal for a hex placeholder in a row arg_max never sizes (here a diag row):
    with it skipped, this placeholder would reach pricing unrefused."""
    new = {"file": F, "line": SCALAR_LINE, "kind": "diag", "template": "{:x}", "literal_bytes": 0,
           "message_arg": "\"m\"", "placeholders": [{"spec": "x", "arg": "new_value"}]}
    out = priced(tmp_path, [new])
    assert not out["complete"]
    assert [k for k, _ in out["unclassified_args"]] == [["hex-unregistered", f"{F}:{SCALAR_LINE} {{:x}} new_value"]]
