"""The text budget's hex sizing of a streaming hasher's finalize() (text_budget.py `arg_max`, `hasher_type`).

A SHA-256 digest in lowercase hex is 64 characters, whatever expression produced it. A `{:x}` of a
`finalize()` is sized at 64 B when its hasher is built as a `Sha256` in the enclosing function; any other
or unproven hasher (a wider hash among them) is left unclassified, so the run stops instead of sizing it
silently. Other hex arguments keep their sizes."""
import ast
import collections
import os
import re
from pathlib import Path

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]


def sizing(root, site):
    """`arg_max` and `hasher_type` as written in text_budget.py, over source `root`, at `site`."""
    module = ast.parse((PRICING / "base/text_budget.py").read_text())
    wanted = [n for n in module.body if isinstance(n, ast.FunctionDef) and n.name in ("hasher_type", "arg_max")]
    assert [n.name for n in wanted] == ["hasher_type", "arg_max"]
    ns = {
        "re": re, "os": os, "proot": str(root), "filetext": {}, "CUR": [site],
        "audited": lambda arg, spec, lex=False: None, "TA": {"classes": {"f64_exp": 24}},
        "arg_rules": [], "ID_CLASSES": set(), "id_unaudited": collections.defaultdict(set),
    }
    exec(compile(ast.Module(body=wanted, type_ignores=[]), "text_budget hex sizing", "exec"), ns)
    return ns["arg_max"]


def write(tmp_path, body):
    path = tmp_path / "src/a.rs"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(body)
    line = next(i for i, text in enumerate(body.split("\n"), 1) if "finalize()" in text and "format!" in text)
    return f"src/a.rs:{line}"


SHA256_FN = """use sha2::{Digest, Sha256};
fn other() {
    let mut hasher = Sha512::new();
}
pub fn hash(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
"""


@pytest.mark.parametrize("construction", [
    "let mut hasher = Sha256::new();",
    "let mut hasher = sha2::Sha256::new();",
    "let mut hasher: Sha256 = Sha256::new();",
])
def test_a_sha256_hashers_finalize_in_hex_is_64_bytes(tmp_path, construction):
    site = write(tmp_path, SHA256_FN.replace("let mut hasher = Sha256::new();", construction))
    assert sizing(tmp_path, site)("x", "hasher.finalize()") == 64


@pytest.mark.parametrize("spec, arg, size", [
    ("016x", "value.to_bits()", 16),
    ("x", "limb", 16),
    ("02x", "b", 16),
    ("032x", "self.sig()", 32),
    ("x", "row.hash128", 32),
    ("x", "Sha256::digest(bytes)", 64),
    ("x", "digest", 64),
])
def test_an_unrelated_hex_argument_is_sized_as_before(tmp_path, spec, arg, size):
    site = write(tmp_path, SHA256_FN)
    assert sizing(tmp_path, site)(spec, arg) == size


@pytest.mark.parametrize("body", [
    SHA256_FN.replace("let mut hasher = Sha256::new();", "let mut hasher = Sha512::new();"),
    SHA256_FN.replace("let mut hasher = Sha256::new();\n", ""),
    SHA256_FN.replace("pub fn hash(bytes: &[u8]) -> String {\n    let mut hasher = Sha256::new();",
                      "pub fn hash(mut hasher: Sha256, bytes: &[u8]) -> String {"),
], ids=["a wider hasher", "a hasher built in another function", "a hasher passed in"])
def test_a_finalize_of_a_wider_or_unproven_hasher_is_left_unclassified(tmp_path, body):
    site = write(tmp_path, body)
    assert sizing(tmp_path, site)("x", "hasher.finalize()") is None


def test_every_hex_finalize_in_the_priced_crates_is_a_sha256():
    """The premise in the priced sources: every `format!("{:x}", h.finalize())` in the crates the
    retained pricing reads (base/crate_dirs.txt) formats a hasher built as a Sha256 in its function."""
    found = {}
    for crate in (PRICING / "base/crate_dirs.txt").read_text().split():
        for path in sorted((SOURCE / crate).rglob("*.rs")):
            rel = path.relative_to(SOURCE).as_posix()
            for line, text in enumerate(path.read_text(encoding="utf-8").split("\n"), 1):
                m = re.search(r'format!\(\s*"\{:x\}",\s*(\w+)\.finalize\(\)\s*\)', text)
                if m:
                    found[f"{rel}:{line}"] = sizing(SOURCE, f"{rel}:{line}")("x", f"{m.group(1)}.finalize()")
    assert len(found) >= 5, found
    assert all(size == 64 for size in found.values()), found
