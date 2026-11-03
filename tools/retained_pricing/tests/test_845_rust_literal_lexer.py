"""§8.45: the Rust literal lexer behind composite_text's static and report literal classes.

Adversarial in both directions (the old quote-pairing regex over- and undercounted), fail-closed
controls, and the actual corpus at the current head. Stdlib + pytest; no product binaries.
"""
import ast
from pathlib import Path
import re
import runpy
import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
R = runpy.run_path(str(PRICING / "base/rust_literals.py"))
literals, string_literals, scan, measure = R["literals"], R["string_literals"], R["scan"], R["measure"]
LexError, debug_escapes, doubling_safe = R["LexError"], R["debug_escapes"], R["doubling_safe"]
OLD = re.compile(r'"((?:\\.|[^"\\])*)"')       # the replaced method, kept here only as a contrast


def longest(text):
    return max((x["nbytes"] for x in string_literals(text)), default=0)


def old_longest(text):
    return max((len(m.group(1).encode()) for m in OLD.finditer(text)), default=0)


def values(text):
    return [x["value"] for x in string_literals(text)]


# ---- overcount: quotes that are not string literals ---------------------------------------------
@pytest.mark.parametrize("src", [
    'let a = "x"; // a "quoted" word and a lone " quote\nlet b = "y";\n',
    'let a = "x"; /* outer " /* inner " */ still " comment */ let b = "y";\n',
    '/// The "doc" comment says "hi" and has a lone "\nfn f() -> &\'static str { "x" }\n',
    '//! crate doc with " quote\n/** block "doc" */ fn g() { let c = \'"\'; let d = "y"; }\n',
    'fn h<\'a>(s: &\'a str) -> char { let q = \'"\'; let e = \'\\\'\'; \'outer: loop { break \'outer; } "x"; q }\n',
])
def test_quotes_outside_literals_are_not_counted(src):
    vs = values(src)
    assert all(len(v.encode()) <= 1 for v in vs), vs
    assert longest(src) == 1
    assert old_longest(src) > 1                  # the regex paired quotes across code here


def test_lifetimes_next_to_quoted_apostrophes():
    src = 'fn k<\'a, \'b: \'a>(x: &\'a str, y: &\'b str) -> &\'static str { if x == "\'" { "y" } else { "\'" } }\n'
    assert [(x["kind"], x["value"]) for x in literals(src)] == [("str", "'"), ("str", "y"), ("str", "'")]


def test_char_literal_forms_and_lifetimes():
    src = ("let a = '\"'; let b = '\\''; let c = '\\u{1F600}'; let d = 'x'; let e = '\\n'; "
           "let f = b'x'; let g = b'\\xFF'; fn l<'a>(x: &'a u8) -> &'static str { \"s\" } let h = 'é';")
    toks = literals(src)
    assert [(x["kind"], x["value"]) for x in toks] == [
        ("char", '"'), ("char", "'"), ("char", "\U0001F600"), ("char", "x"), ("char", "\n"),
        ("byte", ord("x")), ("byte", 0xFF), ("str", "s"), ("char", "é")]


def test_raw_identifier_is_not_a_literal():
    assert values('let r#match = 1; let r#type = "t";') == ["t"]


# ---- undercount: long real literals the regex split or truncated --------------------------------
def test_long_literal_with_escaped_quotes_comment_markers_and_apostrophes():
    body = 'it\\"s // not a comment /* nor this */ and \'quoted\' \\\\ ' * 20
    src = 'let x = \'"\'; let s = "' + body + '"; let y = \'"\';\n'
    expect = ('it"s // not a comment /* nor this */ and \'quoted\' \\ ' * 20)
    assert values(src) == [expect]
    assert longest(src) == len(expect.encode())
    assert old_longest(src) != len(expect.encode())  # the regex measured source width, not the value


def test_long_literal_hidden_by_an_odd_quote_in_a_comment():
    long = "a real literal " * 40
    src = '// it is a "lone quote\nlet s = "' + long + '"; let t = "x";\n'
    assert longest(src) == len(long)
    assert old_longest(src) < len(long)              # the comment's quote shifted the pairing


def test_raw_strings_with_inner_quotes_and_hashes():
    inner = 'a "quoted" word, a lone " and "# not the end'
    src = 'let s = r##"' + inner + '"##; let t = r"plain \\n"; let u = br#"b "x""#;'
    assert values(src) == [inner, "plain \\n", b'b "x"']
    assert longest(src) == len(inner.encode())
    assert old_longest(src) < len(inner.encode())


def test_multiline_string_and_line_continuation():
    src = 'let s = "first line\nsecond line\\\n        continued \\\n\n\t  end";'
    assert values(src) == ["first line\nsecond line" + "continued " + "end"]
    two = 'let s = "a\\\n   b"; let t = "c";'
    assert values(two) == ["ab", "c"]
    assert old_longest(two) == len('; let t = ')            # regex `.` does not match the newline


def test_unicode_and_escapes_count_runtime_utf8_bytes():
    src = 'let s = "Δ3 \\u{1F600} \\u{e9} \\x41 \\0 \\t \\\\ \\" \\\'";'
    v = "Δ3 \U0001F600 é A \0 \t \\ \" '"
    assert values(src) == [v]
    assert longest(src) == len(v.encode("utf-8")) == 23       # Δ 2 B, 😀 4 B, é 2 B, the rest 1 B each
    assert values('let c = c"ab\\xFF\\u{e9}";') == [b"ab\xff\xc3\xa9"]
    assert values('let b = b"\\x00\\xFFz";') == [b"\x00\xffz"]


def test_debug_premise_predicates():
    assert not debug_escapes("plain 'apostrophe' text")       # str Debug leaves ' unescaped
    for v in ('a"b', "a\\b", "a\nb", "é", "\x01"):
        assert debug_escapes(v)
    assert doubling_safe('a"b\\c\n\t\0')
    for v in ("é", "\x01", "\u200b"):
        assert not doubling_safe(v)


# ---- fail closed --------------------------------------------------------------------------------
@pytest.mark.parametrize("src,why", [
    ('let s = "never closed;\n', "unterminated str"),
    ('let s = r#"never closed"; \n', "unterminated raw"),
    ('let s = r"never closed;\n', "unterminated raw"),
    ('let s = br##"x"#;\n', "unterminated raw"),
    ('/* outer /* inner */ never closed\n', "unterminated block comment"),
    ('let c = \'"x;\n', "ambiguous char"),
    ("let c = ''';\n", "ambiguous char"),
    ('let c = \'\\q\';\n', "unknown escape"),
    ('let s = "\\q";\n', "unknown escape"),
    ('let s = "\\x80";\n', "above 0x7F"),
    ('let s = "\\u{D800}";\n', "out of range"),
    ('let s = "\\u{1234567}";\n', "malformed"),
    ('let b = b"\\u{41}";\n', "\\u escape in byte_str"),
    ('let b = b"é";\n', "non-ASCII"),
    ('let s = f"x";\n', "unknown literal prefix 'f'"),
    ('let s = foo#bar;\n', "unknown literal prefix 'foo'"),
    ("let s = x'y';\n", "unknown literal prefix 'x'"),
    ("fn f<'ab'>() {}\n", "lifetime/label immediately followed"),
    ('let s = 1"x";\n', "numeric literal immediately followed"),
    ('let s = "x";\r\n', "carriage return"),
])
def test_unsupported_or_ambiguous_constructs_stop(src, why):
    with pytest.raises(LexError, match=re.escape(why)):
        literals(src, "case.rs")


def test_error_names_the_file_and_line():
    with pytest.raises(LexError, match=r"^case\.rs:3: unterminated str literal"):
        literals('let a = 1;\n// "ok"\nlet s = "open\nmore\n', "case.rs")


# ---- the corpus at the current head -------------------------------------------------------------
def composite_lists():
    """composite_text's two file lists, read from its source so the test measures its inventory."""
    tree = ast.parse((PRICING / "base/composite_text.py").read_text(encoding="utf-8"))
    out = {}
    for node in tree.body:
        if isinstance(node, ast.Assign) and isinstance(node.targets[0], ast.Name) and node.targets[0].id in ("lit_files", "rep_files"):
            out[node.targets[0].id] = ast.literal_eval(node.value)
    return out["lit_files"], out["rep_files"]


LIT, REP = composite_lists()
DIRS = (PRICING / "base/crate_dirs.txt").read_text(encoding="utf-8").split()
DISPOSITIONS = runpy.run_path(str(PRICING / "base/static_literal_dispositions.py"))["ENTRIES"]


@pytest.mark.parametrize("files", [LIT, REP], ids=["static", "report"])
def test_corpus_lexes_completely_and_the_class_is_its_longest_token(files):
    """Self-consistency at the head, no pinned value or line: every file lexes (no LexError), and the
    measured class equals the runtime bytes of the reported token, checked independently below."""
    m = measure(SOURCE, files)
    assert m["count"] > 0 and m["at"]
    f, line = m["at"].rsplit(":", 1)
    toks = [x for x in string_literals((SOURCE / f).read_text(encoding="utf-8"), f) if x["line"] == int(line)]
    assert max(x["nbytes"] for x in toks) == m["max_bytes"]
    assert one_token_bytes(m["at"]) == m["max_bytes"]


def one_token_bytes(site):
    """Independent of the lexer's walk: the line holds exactly two quotes, the code before the first has
    no comment marker or char/lifetime quote, and the text between them has no escape; so that text is
    one plain literal, and its UTF-8 length is the literal's runtime size."""
    f, line = site.rsplit(":", 1)
    text = (SOURCE / f).read_text(encoding="utf-8").split("\n")[int(line) - 1]
    m = re.fullmatch(r'([^"]*)"([^"\\]*)"([^"]*)', text)
    assert m and text.count('"') == 2, text[:120]
    assert "//" not in m.group(1) and "/*" not in m.group(1) and "'" not in m.group(1)
    return len(m.group(2).encode("utf-8"))


@pytest.mark.parametrize("f", sorted(set(LIT) | set(REP)))
def test_every_quote_in_the_corpus_is_accounted_for(f):
    """Each `"` lies inside a literal token or a comment, and each literal token's span starts and
    ends with its delimiters: no quote is left over for a mis-pairing."""
    text = (SOURCE / f).read_text(encoding="utf-8")
    toks, comments = scan(text, f)
    covered = bytearray(len(text))
    for s, e in [(x["start"], x["end"]) for x in toks] + comments:
        assert not any(covered[s:e]), f"overlapping spans at {s}"
        covered[s:e] = b"\x01" * (e - s)
    stray = [i for i, ch in enumerate(text) if ch == '"' and not covered[i]]
    assert stray == []
    for x in toks:
        if x["kind"] == "str":
            assert text[x["start"]] == '"' and text[x["end"] - 1] == '"'


def test_every_long_production_literal_has_one_disposition():
    """The run's whole-scope check at the head: every production literal longer than the static class
    matches one reviewed entry and every entry one literal."""
    threshold = max(512, measure(SOURCE, LIT)["max_bytes"])
    matched, problems = R["long_literal_check"](SOURCE, DIRS, threshold, DISPOSITIONS)
    assert problems == []
    assert len(matched) == len(DISPOSITIONS)
    assert {x["disposition"] for x in matched} <= {"own-bound", "per-site", "never-copied"}


# ---- production scope and the disposition check on synthetic trees ------------------------------
LONG = "x" * 40


def tree(tmp_path, files):
    for name, text in files.items():
        p = tmp_path / name
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_text(text, encoding="utf-8")
    return ["src"]


def prod(tmp_path):
    lits, test_files = R["production_literals"](tmp_path, R["scope_files"](tmp_path, ["src"]))
    return sorted((x["file"], x["value"]) for x in lits), test_files


def test_test_only_modules_are_excluded_and_everything_else_kept(tmp_path):
    tree(tmp_path, {
        "src/lib.rs": ('mod a;\n#[cfg(test)]\nmod t;\n#[cfg(test)]\n#[path = "deep/p.rs"]\npub(crate) mod q;\n'
                       '#[cfg(any(test, feature = "x"))]\nmod w;\n#[cfg(test)]\nmod inline {\n    const S: &str = "in-test {";\n}\n'
                       'const KEEP: &str = "kept";\n// #[cfg(test)] mod c;\nmod c;\n'),
        "src/a.rs": 'const A: &str = "a";\n#[cfg(test)]\nmod at;\n',
        "src/a/at.rs": 'const AT: &str = "a-test";\nmod deeper;\n',
        "src/a/at/deeper.rs": 'const D: &str = "deeper-test";\n',
        "src/t.rs": 'const T: &str = "test";\n',
        "src/deep/p.rs": 'const P: &str = "path-test";\n',
        "src/w.rs": 'const W: &str = "any-test-kept";\n',
        "src/c.rs": 'const C: &str = "c-kept";\n',
    })
    values, test_files = prod(tmp_path)
    lib = [v for f, v in values if f == "src/lib.rs"]
    assert "kept" in lib and "in-test {" not in lib            # the inline test module's body is out
    assert sorted(v for f, v in values if f != "src/lib.rs") == ["a", "any-test-kept", "c-kept"]
    assert test_files == sorted(["src/t.rs", "src/deep/p.rs", "src/a/at.rs", "src/a/at/deeper.rs"])


def test_an_unresolved_test_module_stays_in_scope(tmp_path):
    tree(tmp_path, {"src/lib.rs": '#[cfg(test)]\n#[path = concat!("x", ".rs")]\nmod x;\nmod y { #[cfg(test)] mod z; }\n',
                    "src/x.rs": 'const X: &str = "x";\n', "src/y/z.rs": 'const Z: &str = "z";\n', "src/z.rs": 'const Z: &str = "z2";\n'})
    values, test_files = prod(tmp_path)
    assert test_files == [] and sorted(v for f, v in values if f != "src/lib.rs") == ["x", "z", "z2"]


@pytest.mark.parametrize("entries,problem", [
    ([], "with 0 disposition entries"),
    ([{"file": "src/lib.rs", "head": "x", "disposition": "own-bound", "cite": "c"},
      {"file": "src/lib.rs", "head": "xx", "disposition": "own-bound", "cite": "c"}], "with 2 disposition entries"),
    ([{"file": "src/lib.rs", "head": "x", "disposition": "own-bound", "cite": "c"},
      {"file": "src/lib.rs", "head": "gone", "disposition": "own-bound", "cite": "c"}], "matches 0 long literals"),
])
def test_long_literal_check_stops_on_unlisted_ambiguous_or_stale(tmp_path, entries, problem):
    dirs = tree(tmp_path, {"src/lib.rs": f'const L: &str = "{LONG}";\nconst S: &str = "short";\n'})
    matched, problems = R["long_literal_check"](tmp_path, dirs, 10, entries)
    assert any(problem in p for p in problems), problems


def test_long_literal_check_matches_each_listed_literal_once(tmp_path):
    # t.rs's equal literal is test-only, so the one entry matches lib.rs's literal alone
    dirs = tree(tmp_path, {"src/lib.rs": f'const L: &str = "{LONG}";\n#[cfg(test)]\nmod t;\nconst S: &str = "short";\n',
                           "src/t.rs": f'const T: &str = "{LONG}";\n'})
    entry = {"file": "src/lib.rs", "head": "xxx", "disposition": "never-copied", "cite": "c"}
    matched, problems = R["long_literal_check"](tmp_path, dirs, 10, [entry])
    assert problems == [] and [(m["at"], m["bytes"]) for m in matched] == [("src/lib.rs:1", 40)]
