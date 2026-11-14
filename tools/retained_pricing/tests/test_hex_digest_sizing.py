"""The text budget's hex sizing of SHA-256 values (text_budget.py `arg_max` and `proved_sha256`).

A SHA-256 digest in lowercase hex is 64 characters, whatever expression produced it. The rule sizes a
hex placeholder at 64 B only when it proves the value is SHA-256: a `NAME.finalize()` of a local
`Sha256::new()` / `Sha256::default()` hasher, an argument whose own text is a `Sha256::digest(...)` call,
or a bare name bound to a `Sha256::digest(...)` call. The proof reads past comments, strings and raw
strings, respects scopes, and refuses shadowing and anything it does not recognise. Not proved leaves the
placeholder unclassified, so the run stops. Other hex arguments keep their sizes."""
import ast
import collections
import importlib.util
import os
import re
from pathlib import Path

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
FUNCTIONS = ("_code_text", "_open_braces", "_sha256_resolves", "proved_sha256", "arg_max")
ASSIGNED = ("_SHA256", "_INITS")


def rust_literals():
    spec = importlib.util.spec_from_file_location("rust_literals", PRICING / "base/rust_literals.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def sizing(root, site, pricing=PRICING):
    """`arg_max` and its proof as written in text_budget.py (under `pricing`), over source `root`, at `site`."""
    module = ast.parse((pricing / "base/text_budget.py").read_text())
    wanted = [n for n in module.body
              if (isinstance(n, ast.FunctionDef) and n.name in FUNCTIONS)
              or (isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id in ASSIGNED for t in n.targets))]
    names = [n.name if isinstance(n, ast.FunctionDef) else n.targets[0].id for n in wanted]
    assert sorted(names) == sorted(FUNCTIONS + ASSIGNED), names
    ns = {
        "re": re, "os": os, "proot": str(root), "filetext": {}, "CUR": [site], "_rl": rust_literals(), "_CODE": {},
        "audited": lambda arg, spec, lex=False: None, "TA": {"classes": {"f64_exp": 24}},
        "arg_rules": [], "ID_CLASSES": set(), "id_unaudited": collections.defaultdict(set),
    }
    exec(compile(ast.Module(body=wanted, type_ignores=[]), "text_budget hex sizing", "exec"), ns)
    return ns["arg_max"]


def write(tmp_path, body, marker):
    path = tmp_path / "src/a.rs"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(body)
    line = next(i for i, text in enumerate(body.split("\n"), 1) if marker in text)
    return f"src/a.rs:{line}"


def size(tmp_path, body, arg, spec="x", marker="format!"):
    return sizing(tmp_path, write(tmp_path, body, marker))(spec, arg)


# ------------------------------------------------------------------ the five real shapes, synthetic

USE = "use sha2::{Digest, Sha256};\n"
WIRE = USE + """fn sha_hex(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
fn sha_streamed(parts: &[&[u8]]) -> Option<String> {
    let mut hasher = Sha256::new();
    for part in parts { hasher.update(part); }
    Some(format!("{:x}", hasher.finalize()))
}
"""
RECEIPT = USE + """pub fn receipt_hash(value: &Value) -> Result<String, String> {
    let mut hasher = Sha256::new();
    write_canonical(value, &mut ByteSink(|bytes| hasher.update(bytes)))?;
    Ok(format!("{:x}", hasher.finalize()))
}
"""
CENSUS = USE + """fn digest_of(values: &[u64]) -> String {
    let mut h = Sha256::new();
    for value in values {
        h.update(value.to_le_bytes());
    }
    format!("{:x}", h.finalize())
}
"""
MEMBER_LOAD = USE + """fn record(rows: &[Row]) -> Record {
    let mut hash = Sha256::new();
    for row in rows {
        hash.update(((row.dof / 6) as u32).to_le_bytes());
        for word in &row.n {
            hash.update(word.to_le_bytes());
        }
    }
    Record {
        count: rows.len(),
        digest: format!("{:x}", hash.finalize()),
    }
}
"""
BLOCKS = USE + """pub fn domain_hash(domain: &str, payload: &Value) -> Result<String, String> {
    let mut hasher = Sha256::new();
    checked_v1_domain_canonical(domain, payload, &mut ByteSink(|bytes| hasher.update(bytes)))?;
    Ok(format!("{:x}", hasher.finalize()))
}
"""
NATIVE = USE + """fn verify(s: &Value) -> Result<(), ()> {
    for (source, expected) in [(true, "kernel"), (false, "stiffness")] {
        let matches = {
            let encoded = native_bytes(s, source)?;
            let digest = Sha256::digest(&encoded);
            s[expected] == format!("{:x}", digest)
        };
        if !matches { return Err(()); }
    }
    Ok(())
}
"""


@pytest.mark.parametrize("body, arg", [
    (WIRE, "hasher.finalize()"), (RECEIPT, "hasher.finalize()"), (CENSUS, "h.finalize()"),
    (MEMBER_LOAD, "hash.finalize()"), (BLOCKS, "hasher.finalize()"),
], ids=["retained_wire", "source_receipt", "j7_rf_census", "declared_member_load", "source_blocks"])
def test_the_five_real_finalize_shapes_are_proved_64(tmp_path, body, arg):
    marker = arg + ")"
    assert size(tmp_path, body, arg, marker=marker) == 64


@pytest.mark.parametrize("construction", [
    "let mut hasher = Sha256::new();",
    "let mut hasher = sha2::Sha256::new();",
    "let mut hasher: Sha256 = Sha256::new();",
    "let mut hasher = Sha256::default();",
])
def test_a_sha256_hashers_finalize_in_hex_is_64_bytes(tmp_path, construction):
    body = WIRE.replace("let mut hasher = Sha256::new();", construction)
    assert size(tmp_path, body, "hasher.finalize()", marker="hasher.finalize())") == 64


def test_a_bound_sha256_digest_is_proved_64(tmp_path):
    """retained_precision.rs:5935's shape: the name is bound to a Sha256::digest call in an open block."""
    assert size(tmp_path, NATIVE, "digest", marker='format!("{:x}", digest)') == 64


# ------------------------------------------------------------------ not proved (unclassified)

NOT_PROVED_FINALIZE = {
    "shadowed by a Sha512 binding": WIRE.replace(
        "let mut hasher = Sha256::new();", "let mut hasher = Sha256::new();\n    let mut hasher = Sha512::new();"),
    "shadowing even when the later binding is Sha256": WIRE.replace(
        "let mut hasher = Sha256::new();", "let mut hasher = Sha512::new();\n    let mut hasher = Sha256::new();"),
    "a binding in a closed inner block": WIRE.replace(
        "let mut hasher = Sha256::new();", "{ let mut hasher = Sha256::new(); }"),
    "a binding in a nested closed scope (a closure body)": WIRE.replace(
        "let mut hasher = Sha256::new();", "let f = || { let mut hasher = Sha256::new(); hasher };"),
    "a commented-out construction": WIRE.replace(
        "let mut hasher = Sha256::new();", "// let mut hasher = Sha256::new();"),
    "a block-commented construction": WIRE.replace(
        "let mut hasher = Sha256::new();", "/* let mut hasher = Sha256::new(); */"),
    "a construction inside a string": WIRE.replace(
        "let mut hasher = Sha256::new();", 'let s = "{ let mut hasher = Sha256::new(); }";'),
    "a construction inside a raw string": WIRE.replace(
        "let mut hasher = Sha256::new();", 'let s = r#"{ let mut hasher = Sha256::new(); }"#;'),
    "a wider hasher": WIRE.replace("Sha256::new()", "Sha512::new()", 1).replace("Sha256::new()", "Sha512::new()"),
    "a hasher built in another function": WIRE.replace("    let mut hasher = Sha256::new();\n", ""),
    "a hasher passed in": WIRE.replace("fn sha_streamed(parts: &[&[u8]])", "fn sha_streamed(mut hasher: Sha256, parts: &[&[u8]])")
                              .replace("    let mut hasher = Sha256::new();\n", ""),
    "a suggestive name only": WIRE.replace("let mut hasher = Sha256::new();", "let mut hasher = make();"),
    "an unresolved type alias": WIRE.replace(USE, "use sha2::Digest;\ntype Sha256 = sha2::Sha512;\n"),
    "a type not imported from sha2": WIRE.replace(USE, "use other::Sha256;\n"),
    "a rebinding pattern between binding and use": WIRE.replace(
        "for part in parts { hasher.update(part); }", "let g = |hasher: u8| hasher;"),
}


@pytest.mark.parametrize("name", sorted(NOT_PROVED_FINALIZE))
def test_an_unproven_finalize_is_left_unclassified(tmp_path, name):
    body = NOT_PROVED_FINALIZE[name]
    assert size(tmp_path, body, "hasher.finalize()", marker="hasher.finalize())") is None


@pytest.mark.parametrize("arg", ["self.digest.finalize()", "Sha512::new().finalize()", "(hasher.finalize())", "hasher .finalize()"])
def test_a_finalize_of_another_shape_is_left_unclassified(tmp_path, arg):
    """The reader's argument shapes: a field receiver, an inline construction, parentheses, whitespace."""
    assert size(tmp_path, WIRE, arg, marker="hasher.finalize())") is None


def test_the_finalize_rule_runs_before_the_name_rules(tmp_path):
    """A finalize receiver named `digest` is never sized by its name: it must be proved like any other."""
    body = WIRE.replace("hasher", "digest").replace("let mut digest = Sha256::new();", "let mut digest = Sha512::new();")
    assert size(tmp_path, body, "digest.finalize()", marker="digest.finalize())") is None


NOT_PROVED_DIGEST = {
    "bound to a u64": NATIVE.replace("let digest = Sha256::digest(&encoded);", "let digest: u64 = 5;"),
    "bound to a Sha512::digest call": NATIVE.replace("Sha256::digest(&encoded)", "Sha512::digest(&encoded)"),
    "no binding found": NATIVE.replace("            let digest = Sha256::digest(&encoded);\n", ""),
    "bound in a closed block": NATIVE.replace(
        "            let digest = Sha256::digest(&encoded);\n", "            { let digest = Sha256::digest(&encoded); }\n"),
    "shadowed": NATIVE.replace(
        "let digest = Sha256::digest(&encoded);", "let digest = Sha256::digest(&encoded);\n            let digest = 0u64;"),
}


@pytest.mark.parametrize("name", sorted(NOT_PROVED_DIGEST))
def test_a_name_only_digest_is_left_unclassified(tmp_path, name):
    assert size(tmp_path, NOT_PROVED_DIGEST[name], "digest", marker='format!("{:x}", digest)') is None


# ------------------------------------------------------------------ unchanged sizes

@pytest.mark.parametrize("spec, arg, expected", [
    ("016x", "value.to_bits()", 16),
    ("x", "limb", 16),
    ("02x", "b", 16),
    ("032x", "self.sig()", 32),
    ("x", "row.hash128", 32),
    ("x", "Sha256::digest(bytes)", 64),
    ("x", "Sha256::digest(canonical_json(v).as_bytes())", 64),
])
def test_an_unrelated_hex_argument_is_sized_as_before(tmp_path, spec, arg, expected):
    """Other hex arguments keep their sizes, the fifteen own-text Sha256::digest(...) calls among them."""
    assert size(tmp_path, WIRE, arg, spec=spec, marker="hasher.finalize())") == expected


# ------------------------------------------------------------------ the priced sources

def test_every_hex_sha256_value_in_the_priced_crates_is_proved():
    """In the crates the retained pricing reads (base/crate_dirs.txt): every `format!("{:x}", NAME.finalize())`
    and every `format!("{:x}", NAME)` whose NAME is bound to a Sha256::digest call is proved 64."""
    proved, digests = {}, {}
    for crate in (PRICING / "base/crate_dirs.txt").read_text().split():
        for path in sorted((SOURCE / crate).rglob("*.rs")):
            rel = path.relative_to(SOURCE).as_posix()
            text = path.read_text(encoding="utf-8")
            for line, code in enumerate(text.split("\n"), 1):
                m = re.search(r'format!\(\s*"\{:x\}",\s*(\w+)\.finalize\(\)\s*\)', code)
                if m:
                    proved[f"{rel}:{line}"] = sizing(SOURCE, f"{rel}:{line}")("x", f"{m.group(1)}.finalize()")
                m = re.search(r'format!\(\s*"\{:x\}",\s*(digest)\s*\)', code)
                if m:
                    digests[f"{rel}:{line}"] = sizing(SOURCE, f"{rel}:{line}")("x", m.group(1))
    assert len(proved) >= 5, proved
    assert all(v == 64 for v in proved.values()), proved
    assert "core/reporting/result_export/src/retained_precision.rs:5935" in digests, digests
    assert all(v == 64 for v in digests.values()), digests
