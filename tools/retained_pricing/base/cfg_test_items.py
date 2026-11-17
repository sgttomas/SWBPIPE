"""CFG-PROD (PR-A, retained-pricing-rules.md §8.62): a Rust file's production text, each exact `#[cfg(test)]` item,
statement or expression-position element removed, everything else kept byte for byte (newlines kept, so line numbers
are unchanged).

The earlier helper (o10_rules `_production` and batch5_model `production`) counted a `{` nested inside parentheses as
the item's own block. A test statement such as `tests::PROBE.with(|probe| { .. });` then never ended at its `;` and
swallowed the production code after it, up to the next block closing at the statement's depth (lib.rs's
KD5_RECOVERY_PROBE statement hid the `if !recovery_records.is_empty() {` branch that follows it). A guard that reads
such text can miss production code; this module replaces both copies.

The lexical rule, on code only (string, char and raw literals and comments are masked by base/rust_literals.py):
- An attribute is exactly `#[cfg(test)]` in code. `#[cfg(not(test))]`, `#[cfg(all(test, ..))]` and every other
  attribute are kept (nothing else is removed here).
- Removal starts at that attribute and runs through any further outer attributes (`#[...]`, bracket-matched) to the
  end of the element they gate. Attributes written before it (`#[allow(..)]\\n#[cfg(test)]`) are kept.
- The element is scanned with a depth over `(` and `[` only:
  - at depth 0, `;` ends it (inclusive);
  - at depth 0, a `{` opens a block, which is brace-matched; after the block closes, the element continues only if the
    next code token is `else`, `.` or `?`, or the element began with `let`; otherwise it ends after the block. So
    `fn`, `mod`, `impl`, `if .. {} else {}`, `match`, `for` and a braced macro call end at their block; `let x = if a
    {..} else {..};` and `{..}.method();` end at their `;`;
  - a `{` inside parentheses or brackets (a closure body, a struct literal argument) is inside the element and never
    ends it;
  - at depth 0, an unmatched `)`, `]` or `}` ends the element before it (the last field, arm or argument of a list
    without a trailing comma);
  - a depth-0 `,` ends a field (`name: Type,`, counted outside the type's angle brackets) or an expression-position
    element (an arm, a list element) inclusively; it never ends an item (`fn`, `struct`, `impl`, `const`, `use`, ...,
    after an optional visibility), whose generics and parameters hold commas, or a `let`.
- An attribute inside an element already removed (a nested `#[cfg(test)]`) is part of that element.
- An element that does not end before the end of the file stops (ValueError), rather than removing an unbounded suffix.
"""
import os
import re
import runpy

ATTR = re.compile(r"#\[cfg\(test\)\]")
ITEM_WORDS = {"fn", "struct", "enum", "union", "mod", "impl", "trait", "type", "const", "static", "use", "extern",
              "unsafe", "async", "macro_rules"}
_LEXER = []


def _lexer():
    if not _LEXER:
        _LEXER.append(runpy.run_path(os.path.join(os.path.dirname(os.path.abspath(__file__)), "rust_literals.py")))
    return _LEXER[0]


def _skip_space(text, mask, i):
    n = len(text)
    while i < n and (text[i].isspace() or mask[i]):
        i += 1
    return i


def _word_at(text, i):
    m = re.match(r"[A-Za-z_][A-Za-z0-9_]*", text[i:i + 64])
    return m.group(0) if m else ""


def element_end(text, mask, start):
    """The end offset (exclusive) of the element gated by the `#[cfg(test)]` at `start`."""
    n = len(text)
    i = start
    # the attribute stack
    while True:
        i = _skip_space(text, mask, i)
        if text.startswith("#[", i) and not mask[i]:
            depth = 0
            while i < n:
                if not mask[i]:
                    if text[i] == "[":
                        depth += 1
                    elif text[i] == "]":
                        depth -= 1
                        if depth == 0:
                            i += 1
                            break
                i += 1
            continue
        break
    # The element's kind, from its first words (after an optional visibility):
    # - item: fn, struct, enum, union, mod, impl, trait, type, const, static, use, extern, unsafe, async, macro_rules;
    #   commas never end it (they are generics, parameters or fields inside its header or block);
    # - let: only a depth-0 `;` ends it;
    # - field: `name:` (not `name::`); a depth-0 `,` outside angle brackets ends it (types may hold `<A, B>`);
    # - expression: anything else (a statement, a match arm, a list element); a depth-0 `,` ends it.
    k = i
    if _word_at(text, k) == "pub":
        k = _skip_space(text, mask, k + 3)
        if text.startswith("(", k) and not mask[k]:
            k = text.index(")", k) + 1
        k = _skip_space(text, mask, k)
    first = _word_at(text, k)
    after = _skip_space(text, mask, k + len(first)) if first else k
    if first in ITEM_WORDS:
        kind = "item"
    elif first == "let":
        kind = "let"
    elif first and text.startswith(":", after) and not text.startswith("::", after):
        kind = "field"
    else:
        kind = "expression"
    depth = angle = 0
    j = i
    while j < n:
        if mask[j]:
            j += 1
            continue
        c = text[j]
        if c in "([":
            depth += 1
        elif c in ")]":
            if depth == 0:
                return j
            depth -= 1
        elif c == "}" and depth == 0:
            return j
        elif kind == "field" and depth == 0 and c == "<":
            angle += 1
        elif kind == "field" and depth == 0 and c == ">" and text[j - 1] != "-" and angle:
            angle -= 1
        elif depth == 0 and c == ";":
            return j + 1
        elif depth == 0 and angle == 0 and c == "," and kind in ("field", "expression"):
            return j + 1
        elif depth == 0 and c == "{":
            k, braces = j, 0
            while k < n:
                if not mask[k]:
                    if text[k] == "{":
                        braces += 1
                    elif text[k] == "}":
                        braces -= 1
                        if braces == 0:
                            break
                k += 1
            if k >= n:
                raise ValueError(f"CFG-PROD: unterminated block of the #[cfg(test)] element at offset {start}")
            j = k + 1
            nxt = _skip_space(text, mask, j)
            if kind == "let" or _word_at(text, nxt) == "else" or text.startswith((".", "?"), nxt):
                continue
            if text.startswith(";", nxt):
                return nxt + 1
            return j
        j += 1
    raise ValueError(f"CFG-PROD: the #[cfg(test)] element at offset {start} does not end")


def production(text, rel="<text>"):
    """`text` without its `#[cfg(test)]` elements (newlines kept)."""
    if "#[cfg(test)]" not in text:
        return text
    lexer = _lexer()
    toks, comments = lexer["scan"](text, rel)
    mask = lexer["_masks"](text, toks, comments)
    out, at = [], 0
    for m in ATTR.finditer(text):
        if m.start() < at or mask[m.start()]:
            continue
        end = element_end(text, mask, m.start())
        out.append(text[at:m.start()])
        out.append("\n" * text.count("\n", m.start(), end))
        at = end
    out.append(text[at:])
    return "".join(out)


def legacy_production(text, rel="<text>"):
    """The earlier helper's exact behaviour, kept only to compare its consumers (CFG-PROD consumer census); never a
    pricing input."""
    if "#[cfg(test)]" not in text:
        return text
    lexer = _lexer()
    toks, comments = lexer["scan"](text, rel)
    mask = lexer["_masks"](text, toks, comments)
    out, at = [], 0
    for m in ATTR.finditer(text):
        if m.start() < at or mask[m.start()]:
            continue
        i, nest, opened = m.end(), 0, False
        while i < len(text):
            ch = text[i]
            if not mask[i]:
                if ch in "([":
                    nest += 1
                elif ch in ")]":
                    nest -= 1
                elif ch == "{":
                    nest += 1
                    opened = True
                elif ch == "}":
                    nest -= 1
                    if opened and nest == 0:
                        i += 1
                        break
                elif ch == ";" and nest == 0 and not opened:
                    i += 1
                    break
            i += 1
        else:
            raise ValueError(f"legacy: {rel}: unterminated #[cfg(test)] item at offset {m.start()}")
        out.append(text[at:m.start()])
        out.append("\n" * text.count("\n", m.start(), i))
        at = i
    out.append(text[at:])
    return "".join(out)
