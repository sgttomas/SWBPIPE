#!/usr/bin/env python3
"""Lane-local check that public text added on a branch carries no evaluation of the external prover.

Owner ruling (2026-10-26): "I want: **interoperate openly, evaluate privately**". The product name and the
file-format and interface facts our code needs to interoperate are public. Anything that evaluates the product
stays in the private exchange folder: result values, differences, agreement counts and classifications,
tolerances measured from results; its defects and crashes, the trial's limits, the evaluation stamp, its
performance; screenshots and survey notes. This check screens what a change adds for those kinds of content:

- a result: a comparison or agreement word with a numeric value in one sentence that refers to the target;
- a defect or limit: defect, crash, limit, trial, stamp or performance language in a sentence that refers to
  the target;
- a screenshot reference in a sentence that refers to the target or to the survey;
- a private token (a distinctive string from the private notes), anywhere in prose, owner-words and
  policy-text included, and on every added line of tests and fixtures.

A sentence refers to the target when it, or a line next to the hit, names the product, its vendor or version
(the private terms file's `[names]`), or says "external prover", "target tool" or "the prover" (or, for
screenshots, "survey"). A numeric result is a count (`20 of 35`, `35/35`, `20 matched`, `12 quantities`),
a percentage or ppm, a number with a unit, or scientific notation; ids, dates, section numbers, bare
integers, conversion rates (`25.4 mm per inch`) and the product's version are not. The terms file's sections are `[names]`
and `[tokens]`, one term per line; `#` comments and blank lines are ignored. Terms are never printed; a
finding names the place and the kind.

Prose is every line of a non-code file, and in code files (Python, PowerShell, TypeScript, Rust) only
comments, docstrings and test function names: code may contain whatever it needs. Fixture CSV data counts as
code. Comment extraction outside Python is deliberately over-inclusive (quotes ignored, block nesting
counted). Every prose line, whatever its source, is matched as visible text: links keep their labels, and
blockquote, emphasis and code markers are removed. Consecutive prose lines are read as one text, so a term
wrapped across lines still matches; a change is screened in the context of its whole file, and a finding is
reported when a match involves an added line. Paths and file names are not screened. Commit messages in
`--base..--head` and any `--file` given whole are prose too. In Markdown, the owner's verbatim words between
`<!-- owner-words -->` and `<!-- /owner-words -->`, and text stating the data rule itself between
`<!-- policy-text -->` and `<!-- /policy-text -->`, are not screened; an unpaired marker is an operational
error. The check's own source and tests are screened for private tokens only.

The check is a lexical aid to review, not a proof that text is clean; review of the change remains the control.

Exit 1 on a finding, 2 on an operational error, 0 otherwise.
"""

from __future__ import annotations

import argparse
import ast
import html
import io
import re
import subprocess
import sys
import tokenize
from pathlib import Path

CODE_SUFFIXES = {".py", ".ps1", ".ts", ".tsx", ".js", ".rs"}
SECTIONS = ("names", "tokens")


def load_terms(path: Path) -> dict[str, list[str]]:
    terms: dict[str, list[str]] = {name: [] for name in SECTIONS}
    section = None
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        if line.startswith("[") and line.endswith("]"):
            section = line[1:-1].strip()
            if section not in terms:
                raise ValueError("unknown section in terms file")
            continue
        if section is None:
            raise ValueError("term outside a section in terms file")
        terms[section].append(line)
    return terms


def _normal(text: str) -> str:
    return " ".join(text.split())


def _word(term: str) -> re.Pattern[str]:
    # Identifier characters bound a term, so code paths and identifiers (`caepipe_mbf`) never match a name.
    return re.compile(r"(?<![A-Za-z0-9_])" + re.escape(_normal(term)) + r"(?![A-Za-z0-9_])", re.IGNORECASE)


# Paths and file names (`Design/x-notes.md`) are not prose; they are blanked, offsets kept, before names
# and markers are matched. Only a word that is clearly a path is blanked: after its surrounding punctuation
# is stripped it must end in a separator or a known file extension, or join path segments that hold an
# underscore. `Name/Other`, `Name/9.99` and sentence-final `Name/Other.` stay prose. Observed tokens are
# still matched on the unmasked text.
FILE_EXTENSION = re.compile(r"\.(?:md|py|json|ya?ml|txt|csv|mbf|ps1|rs|tsx?|js|toml|sh)$", re.IGNORECASE)
PATH_WORD = re.compile(r"[\w.-]+(?:[/\\][\w.-]+)*[/\\]?")
EDGE_PUNCTUATION = "`'\"()[]<>{}|,;:!?."


def _path(word: str) -> bool:
    if not PATH_WORD.fullmatch(word):
        return False
    separated = "/" in word or "\\" in word
    return bool(FILE_EXTENSION.search(word)) or word.endswith(("/", "\\")) or (separated and "_" in word)


def _unpathed(text: str) -> str:
    out = list(text)
    for match in re.finditer(r"\S+", text):
        word = match.group()
        lead = len(word) - len(word.lstrip(EDGE_PUNCTUATION))
        core = word.strip(EDGE_PUNCTUATION)
        if core and _path(core):
            start = match.start() + lead
            out[start:start + len(core)] = " " * len(core)
    return "".join(out)


COMMENT_CLOSER = re.compile(r"(?:\*+/|#>)\s*$")
COMMENT_OPENER = re.compile(r"^(?:<#|/\*+!?|//+!?|#+|\*+)")


def _strip_comment(text: str) -> str:
    """The words of one comment line, without its comment syntax, so wrapped comments join as prose.
    A closing delimiter is removed before an opening prefix, so a bare `#>` or `*/` leaves nothing."""

    return COMMENT_OPENER.sub("", COMMENT_CLOSER.sub("", text.strip())).strip()


def _phrase(term: str) -> re.Pattern[str]:
    return re.compile(re.escape(_normal(term)), re.IGNORECASE)


def python_prose_lines(text: str) -> dict[int, str]:
    """Comments, docstring-style strings and test function names of a Python source, by line number."""

    prose: dict[int, str] = {}
    try:
        tokens = list(tokenize.generate_tokens(io.StringIO(text).readline))
        ast.parse(text)
    except (tokenize.TokenError, IndentationError, SyntaxError, ValueError):
        return {number: line for number, line in enumerate(text.splitlines(), 1)}
    # Docstrings in any quoting (a one-line single-quoted docstring is not triple-quoted), as the span of
    # their expression, so every implicitly concatenated literal belongs to it. AST columns count UTF-8
    # bytes and tokenizer columns characters; spans are converted to characters.
    source = text.splitlines()

    def position(line: int, byte_column: int) -> tuple[int, int]:
        return line, len(source[line - 1].encode("utf-8")[:byte_column].decode("utf-8", errors="ignore"))

    docstrings = []
    for node in ast.walk(ast.parse(text)):
        if isinstance(node, (ast.Module, ast.ClassDef, ast.FunctionDef, ast.AsyncFunctionDef)) and node.body:
            first = node.body[0]
            if isinstance(first, ast.Expr) and isinstance(first.value, ast.Constant) \
                    and isinstance(first.value.value, str):
                docstrings.append((position(first.lineno, first.col_offset),
                                   position(first.end_lineno, first.end_col_offset)))
    previous = None
    for token in tokens:
        if token.type == tokenize.COMMENT:
            prose[token.start[0]] = prose.get(token.start[0], "") + " " + _strip_comment(token.string)
        elif token.type == tokenize.STRING and (any(low <= token.start and token.end <= high
                                                    for low, high in docstrings)
                                                or token.string.lstrip("rbuRBUfF").startswith(('"""', "'''"))):
            for offset, part in enumerate(_string_lines(token.string)):
                number = token.start[0] + offset
                prose[number] = prose.get(number, "") + ("" if part.startswith(GLUE) else " ") + part
        elif token.type == tokenize.NAME and previous is not None and previous.string == "def" \
                and token.string.startswith("test"):
            prose[token.start[0]] = prose.get(token.start[0], "") + " " + token.string.replace("_", " ")
        if token.type not in (tokenize.NL, tokenize.NEWLINE, tokenize.INDENT, tokenize.DEDENT):
            previous = token
    return prose


GLUE = "\x00"  # marks a prose line that continues the previous one without a separator
STRING_OPENING = re.compile(r"[rbuRBUfF]*(\'\'\'|\"\"\"|\'|\")")


def _string_lines(literal: str) -> list[str]:
    """The text of a string literal by source line, without prefix and delimiters, so implicitly
    concatenated fragments (`'Widget ' 'overflow'`) join as prose."""

    if "\n" not in literal:
        try:
            value = ast.literal_eval(literal)
        except (ValueError, SyntaxError):
            value = None
        if isinstance(value, str):
            return [value]
    raw = literal.splitlines()
    opening = STRING_OPENING.match(literal)
    if opening:
        raw[0] = raw[0][len(opening.group(0)):]
        if raw[-1].endswith(opening.group(1)):
            raw[-1] = raw[-1][:-len(opening.group(1))]
    # An odd run of backslashes at a line end continues the string without a line break: the next line is
    # glued on (GLUE), so a word split across the continuation reads whole.
    parts, glue = [], False
    for part in raw:
        continued = (len(part) - len(part.rstrip("\\"))) % 2 == 1
        text = part[:-1] if continued else part
        parts.append((GLUE if glue else "") + text)
        # Whitespace before the backslash is text, so it still separates the next line's words.
        glue = continued and not text[-1:].isspace()
    return parts


def comment_prose_lines(text: str, suffix: str) -> dict[int, str]:
    """Comments of a PowerShell, TypeScript, JavaScript or Rust source, by line number.

    Deliberately over-inclusive rather than parsed: quotes are ignored, so a line is prose from its first
    comment opener on, and block comments count their nesting depth. A string, lifetime or regular
    expression that looks like a comment adds code to the prose; it never hides a comment."""

    prose: dict[int, str] = {}
    depth = 0
    opener, closer, single = ("<#", "#>", "#") if suffix == ".ps1" else ("/*", "*/", "//")
    for number, line in enumerate(text.splitlines(), 1):
        start = 0 if depth else None
        i = 0
        while i < len(line):
            if line.startswith(opener, i):
                start = i if start is None else start
                depth, i = depth + 1, i + len(opener)
            elif depth and line.startswith(closer, i):
                depth, i = depth - 1, i + len(closer)
            elif not depth and line.startswith(single, i):
                start = i if start is None else start
                break
            else:
                i += 1
        if start is not None:
            prose[number] = _strip_comment(line[start:])
    return prose


OWNER_WORDS_OPEN = "<!-- owner-words -->"
OWNER_WORDS_CLOSE = "<!-- /owner-words -->"
POLICY_TEXT_OPEN = "<!-- policy-text -->"
POLICY_TEXT_CLOSE = "<!-- /policy-text -->"


def _without_span(lines: dict[int, str], opener: str, closer: str) -> dict[int, str]:
    """Drop the text between a pair of markers. Text outside a marked span, on the marker lines too, is
    kept; an unpaired marker raises ValueError."""

    kept: dict[int, str] = {}
    inside = False
    for number in sorted(lines):
        line, out = lines[number], ""
        while line:
            marker = closer if inside else opener
            other = opener if inside else closer
            at, stray = line.find(marker), line.find(other)
            if stray != -1 and (at == -1 or stray < at):
                raise ValueError("unpaired span marker")
            if at == -1:
                out += "" if inside else line
                break
            out += "" if inside else line[:at] + " "
            line, inside = line[at + len(marker):], not inside
        if out.strip():
            kept[number] = out
    if inside:
        raise ValueError("unpaired span marker")
    return kept


def _without_owner_words(lines: dict[int, str]) -> dict[int, str]:
    """Drop the owner's verbatim words (quoted, not evaluations) and text that states the data rule itself
    (`<!-- policy-text -->`), which must name the kinds of content it keeps private."""

    return _without_span(_without_span(lines, OWNER_WORDS_OPEN, OWNER_WORDS_CLOSE), POLICY_TEXT_OPEN, POLICY_TEXT_CLOSE)


MARKDOWN_QUOTE = re.compile(r"^(?:\s*>)+")
# Markdown and HTML syntax rewritten to the visible text, in order, over a whole paragraph so that a link or
# tag wrapped across lines is read as written. HTML comments are taken out and read as paragraphs of their
# own, inline links with balanced destinations keep their labels (`_unlinked`), break and block tags become
# spaces, and a rewrite below keeps a link's label (its first group) or substitutes a literal. Every
# character keeps the source line it came from.
HTML_COMMENT = re.compile(r"<!--(.*?)(?:-->|\Z)", re.DOTALL)
LINK_OPEN = re.compile(r"!?\[([^\[\]]*)\]\(")
TAG_BODY = r"""(?:[^<>"']|"[^"]*"|'[^']*')*>"""  # attributes, with `>` allowed inside quoted values
BLOCK_TAG = re.compile(r"</?(?:br|p|div|li|ul|ol|dl|dt|dd|tr|td|th|table|thead|tbody|h[1-6]|hr|blockquote|pre"
                       r"|section|details|summary)\b" + TAG_BODY, re.IGNORECASE)
REFERENCE_IMAGE = re.compile(r"!\[([^\[\]]*)\](?:\[([^\[\]]*)\])?(?!\()")
REFERENCE_DEFINITION = re.compile(r"^\s*\[([^\[\]]+)\]:\s*<?(\S+?)>?(?:\s.*)?$")
VISIBLE_TEXT = (
    (re.compile(r"\[([^\[\]]*)\]\[[^\[\]]*\]"), 1),
    (re.compile(r"!?\[|\]"), ""),
    (BLOCK_TAG, " "),
    (re.compile(r"</?[A-Za-z]" + TAG_BODY + r"|<[a-z][a-z0-9+.-]*:[^<>\s]*>"), ""),
    (re.compile(r"&(?:#[0-9]+|#[xX][0-9a-fA-F]+|[A-Za-z][A-Za-z0-9]*);"), html.unescape),
    (re.compile(r"\\(?=\s|$)"), " "),  # a Markdown hard line break
    (re.compile(r"[*~`]+|(?<![A-Za-z0-9])_+|_+(?![A-Za-z0-9])"), ""),
    (re.compile(r"\s+"), " "),
)
# A sentence ends at a full stop, question or exclamation mark followed by the end of the text or by a word
# that starts with a capital letter or a digit (after any opening quote, bracket or emphasis).
SENTENCE_END = re.compile(r"[.!?](?=\s+[\"'(\[*_`]*[A-Z0-9]|\s*$)")
# Abbreviations whose full stop does not end a sentence (compared without the final stop, lower case).
ABBREVIATIONS = {"i.e", "e.g", "w.r.t", "ver", "etc", "vs", "cf", "approx", "incl", "excl", "resp", "viz", "al", "fig", "figs",
                 "eq", "eqs", "sec", "no", "nos", "ref", "refs", "vol", "ch", "p", "pp", "dr", "mr", "ms", "st"}
LAST_WORD = re.compile(r"[\w.]+$")


def prose_of(path: str, text: str, exempt: bool = True) -> dict[int, str]:
    """The prose lines of a file; with `exempt`, Markdown owner-words and policy-text spans are dropped."""

    suffix = Path(path).suffix.lower()
    if suffix == ".md":
        lines = {number: line for number, line in enumerate(text.splitlines(), 1)}
        return _without_owner_words(lines) if exempt else lines
    if suffix == ".py":
        return python_prose_lines(text)
    if suffix in CODE_SUFFIXES:
        return comment_prose_lines(text, suffix)
    return {number: line for number, line in enumerate(text.splitlines(), 1)}


def _rewrite(pattern: re.Pattern[str], replacement, text: str, owners: list[int]) -> tuple[str, list[int]]:
    out: list[str] = []
    own: list[int] = []
    position = 0
    for match in pattern.finditer(text):
        out.append(text[position:match.start()])
        own.extend(owners[position:match.start()])
        if isinstance(replacement, int):
            out.append(match.group(replacement))
            own.extend(owners[match.start(replacement):match.end(replacement)])
        else:
            literal = replacement(match.group()) if callable(replacement) else replacement
            out.append(literal)
            own.extend([owners[match.start()]] * len(literal))
        position = match.end()
    out.append(text[position:])
    own.extend(owners[position:])
    return "".join(out), own


def _link_end(text: str, index: int) -> int | None:
    """The index just past an inline link's closing parenthesis, given the index after its opening one:
    an optional `<...>` or balanced destination, then an optional quoted or parenthesized title, with
    backslash escapes honoured. None when the text is not a well-formed inline link."""

    size = len(text)

    def blank(at: int) -> int:
        while at < size and text[at].isspace():
            at += 1
        return at

    index = blank(index)
    if index < size and text[index] == "<":
        index += 1
        while index < size and text[index] != ">":
            index += 2 if text[index] == "\\" else 1
        index += 1
    else:
        depth = 0
        while index < size and not text[index].isspace():
            if text[index] == "\\":
                index += 2
                continue
            if text[index] == "(":
                depth += 1
            elif text[index] == ")":
                if depth == 0:
                    break
                depth -= 1
            index += 1
    index = blank(index)
    if index < size and text[index] in "\"'(":
        close = ")" if text[index] == "(" else text[index]
        index += 1
        while index < size and text[index] != close:
            index += 2 if text[index] == "\\" else 1
        index = blank(index + 1)
    return index + 1 if index < size and text[index] == ")" else None


def _unlinked(text: str, owners: list[int]) -> tuple[str, list[int]]:
    """Inline links and images reduced to their labels (an image also to a screenshot marker); the
    destination's parentheses are balanced."""

    out: list[str] = []
    own: list[int] = []
    position = 0
    for match in LINK_OPEN.finditer(text):
        if match.start() < position:
            continue
        index = _link_end(text, match.end())
        if index is None:
            continue
        out.append(text[position:match.start()])
        own.extend(owners[position:match.start()])
        out.append(match.group(1))
        own.extend(owners[match.start(1):match.end(1)])
        if match.group().startswith("!") and _screenshot_candidate(text[match.start():index]):
            # An image may be a screenshot; the invisible marker lets the screenshot rule judge it in context.
            out.append(IMAGE_MARK)
            own.append(owners[match.start()])
        position = index
    out.append(text[position:])
    own.extend(owners[position:])
    return "".join(out), own


def _unimaged(text: str, owners: list[int], references: dict[str, tuple[str, int]]) -> tuple[str, list[int]]:
    """Reference, collapsed and shortcut images reduced to their labels, with the screenshot marker when the
    label and the resolved destination make the image a screenshot candidate."""

    out: list[str] = []
    own: list[int] = []
    position = 0
    for match in REFERENCE_IMAGE.finditer(text):
        label, key = match.group(1), match.group(2) or match.group(1)
        out.append(text[position:match.start()])
        own.extend(owners[position:match.start()])
        out.append(label)
        own.extend(owners[match.start(1):match.end(1)])
        destination, line = references.get(key.lower(), ("", owners[match.start()]))
        if _screenshot_candidate(label + " " + destination):
            # One marker on the image's line keeps its context; one on the definition's line screens a
            # change to the definition alone.
            out.append(IMAGE_MARK)
            own.append(owners[match.start()])
            if line != owners[match.start()]:
                out.append(IMAGE_MARK)
                own.append(line)
        position = match.end()
    out.append(text[position:])
    own.extend(owners[position:])
    return "".join(out), own


def _paragraphs(lines: dict[int, str], blank_breaks: bool = True) -> list[tuple[str, list[int], set[int]]]:
    """Consecutive prose lines read as one visible, whitespace-normalized text; a gap in line numbers ends a
    paragraph, and so does a blank line when `blank_breaks` (text, not code comments, whose delimiter-only
    lines are empty). Each character carries the source line it came from. In text, a table row and a line in
    a fenced code block are each a sentence of their own: the returned set holds the source lines that begin
    a sentence."""

    paragraphs: list[tuple[str, list[int], set[int]]] = []
    text, owners, last, fenced, starts = "", [], None, False, set()
    references = {match.group(1).lower(): (match.group(2), number) for number, line in lines.items()
                  if (match := REFERENCE_DEFINITION.match(line))}

    def flush() -> None:
        nonlocal text, owners, starts
        if text:
            comments = [(match.group(1), owners[match.start(1):match.end(1)]) for match in HTML_COMMENT.finditer(text)]
            text, owners = _rewrite(HTML_COMMENT, " ", text, owners)
            for part, part_owners in [(text, owners)] + comments:
                if not part.strip():
                    continue
                part, part_owners = _unlinked(part, part_owners)
                part, part_owners = _unimaged(part, part_owners, references)
                for pattern, replacement in VISIBLE_TEXT:
                    part, part_owners = _rewrite(pattern, replacement, part, part_owners)
                paragraphs.append((part, part_owners, starts))
        text, owners, starts = "", [], set()

    previous_hard = False
    for number in sorted(lines):
        line = lines[number]
        glue = line.startswith(GLUE)
        line = line[len(GLUE):] if glue else line
        raw = line.strip()
        fence = blank_breaks and raw.startswith(("```", "~~~"))
        hard = blank_breaks and (fenced or fence or raw.startswith("|"))
        if fence:
            fenced = not fenced
        piece = _normal(MARKDOWN_QUOTE.sub("", line))
        if last is not None and number != last + 1 or not piece and blank_breaks:
            flush()
            previous_hard = False
        last = number
        if not piece:
            continue
        if hard or previous_hard:
            starts.add(number)
        previous_hard = hard
        if text and not (glue and not line[:1].isspace()):
            text, owners = text + " ", owners + [number]
        text, owners = text + piece, owners + [number] * len(piece)
    flush()
    return paragraphs


def _span(owners: list[int], start: int, end: int) -> list[int]:
    return list(range(owners[start], owners[max(start, end - 1)] + 1))


def _sentences(text: str, owners: list[int], starts: set[int]) -> list[int]:
    """The sentence number of every character; a sentence ends at a full stop, question or exclamation
    mark, and before every line in `starts`."""

    ends = set()
    for match in SENTENCE_END.finditer(text):
        word = LAST_WORD.search(text, 0, match.start())
        if match.group() == "." and word and word.group().lower().rstrip(".") in ABBREVIATIONS:
            continue
        ends.add(match.end())
    numbers, current = [], 0
    for index in range(len(text)):
        if index in ends or index and owners[index] != owners[index - 1] and owners[index] in starts:
            current += 1
        numbers.append(current)
    return numbers


def _token_findings(label: str, lines: dict[int, str], tokens: list[tuple[int, str]], added: set[int] | None,
                    kind: str, blank_breaks: bool = True) -> list[str]:
    findings: list[str] = []
    for text, owners, _ in _paragraphs(lines, blank_breaks):
        text, owners = _rewrite(re.compile(IMAGE_MARK), "", text, owners)
        for index, term in tokens:
            for match in _phrase(term).finditer(text):
                span = _span(owners, match.start(), match.end())
                if added is None or added.intersection(span):
                    findings.append(f"{label}:{span[0]}: {kind} #{index}")
    return findings


def _alternation(*words: str) -> re.Pattern[str]:
    return re.compile(r"(?<![\w-])(?:" + "|".join(words) + r")(?![\w-])", re.IGNORECASE)


TARGET_REFERENCE = _alternation(r"external[ -]provers?", r"external[ -]prover's", r"target[ -]tools?",
                                r"target[ -]tool's", r"the prover", r"the prover's")
COMPARISON = _alternation(r"agree[sd]?", r"agreements?", r"agreeing", r"match(?:e[sd]|ing)?", r"mismatch(?:e[sd]|es)?",
                          r"differ(?:s|ed|ing)?", r"differences?", r"deviat(?:es?|ed|ions?|ing)", r"discrepanc(?:y|ies)",
                          r"exceed(?:s|ed|ing)?", r"residuals?", r"correlat(?:es|ed)", r"off by", r"within")
DEFECT = _alternation(r"crash(?:es|ed|ing)?", r"access violations?", r"defects?", r"bugs?", r"buggy", r"hangs?", r"hung",
                      r"freez(?:es|ing)", r"froze(?:n)?", r"unresponsive", r"workarounds?", r"work around", r"broken",
                      r"fail(?:s|ed)? to", r"does not work", r"did not work", r"unreliable", r"unstable",
                      r"stop(?:s|ped)? early", r"abort(?:s|ed)?", r"quits?", r"terminat(?:es|ed) unexpectedly",
                      r"(?:writes|wrote|produces|produced|leaves|left) no (?:CSV|output|outputs|results?|result files?|files?|records?)",
                      r"(?:row|size|model[ -]size|node|element|layout)[ -]limits?",
                      r"limits? (?:on|of) (?:the )?(?:model|models|rows?|size|nodes?|elements?)",
                      r"limited to (?:\d+|small)", r"(?:more than|at most|up to|maximum(?: of)?|max\.?|over) \d+ (?:layout )?rows?",
                      r"trial", r"evaluation (?:version|build|copy|stamp|period)",
                      r"watermark\w*", r"stamp(?:s|ed)?", r"expir(?:es|ed|y|ation)", r"slow(?:er|ly|s)?", r"performance",
                      r"faster", r"speed")
IMAGE_MARK = "\u2063"  # an invisible separator placed after an image's label
# An image is a screenshot candidate unless its label or file names an original drawing (or is an SVG);
# screenshot cues in either win.
ORIGINAL_DIAGRAM = re.compile(r"diagram|schematic|sketch|drawing|\.svg\b", re.IGNORECASE)
SCREENSHOT_CUE = re.compile(r"screen|window|dialog|capture|snap", re.IGNORECASE)


def _screenshot_candidate(label_and_destination: str) -> bool:
    return bool(SCREENSHOT_CUE.search(label_and_destination)) or not ORIGINAL_DIAGRAM.search(label_and_destination)
SCREENSHOT = re.compile(_alternation(r"screen ?shots?", r"screen (?:captures?|grabs?|recordings?)",
                                     r"screengrabs?").pattern + "|" + IMAGE_MARK, re.IGNORECASE)
SURVEY = _alternation(r"survey(?:s|ed)?")
# "matches ... to <record, field, column, id, axis or a capitalised or slashed name>" maps records.
MAPPING_TO = re.compile(r"[^,;.:]{0,60}?\bto\s+(?:the\s+)?(?!\d)(?!\S*\s*(?:decimal|digit|place|precision|significant))"
                        r"(?:[A-Z][\w/]*|\S*/\S*|(?:(?!decimal|digit|place|precision)[A-Za-z_ ,]){0,60}?\b"
                        r"(?:fields?|records?|columns?|ids?|nodes?|keys?|names?|entries|labels?|axes|axis))\b")
_UNIT = r"(?:%|‰|percent|per cent|ppm|parts per million|mm|cm|m|N|kN|N[·*]m|Nm|kN[·*]?m|MPa|rad|deg|°)"
# Numeric results: counts (`20 of 35`, `35/35`, `20 matched`), percentages and ppm, decimals with a unit, and
# scientific notation. Ids, dates, section numbers and bare integers are not results.
_CONVERTED = r"(?:in|inch|inches|ft|feet|foot|lb|lbs|lbf|lbm|kip|kips|psi|ksi|°F|F|°C|C|K|kgf|kg|g|N|kN|mm|cm|m|Pa|kPa|MPa|GPa|rad|deg|°)"
# Unit definitions (`1 in = 25.4 mm`, `1 kgf = 9.80665 N`): one unit equals a number of a different unit.
EQUATION = re.compile(r"(?<![\w.])1\s*(" + _CONVERTED + r")\s*=\s*\d+(?:[.,]\d+)?(?:[eE][-+]?\d+)?\s*("
                      + _CONVERTED + r")(?![\w])")
SIGN = re.compile(r"(?:(?<=^)|(?<=[\s(=:]))[-+−](?=\d)")
_COUNTED = r"(?:quantities|quantity|cases?|values?|checks?|nodes?|results?|records?|reactions?|displacements?|members?)"
RESULT_NUMBER = re.compile(
    r"(?<![\w\-§./])(?:\d+\s*(?:/|of|out of)\s*\d+(?![\w\-]|\.\d)"
    r"|\d+\s+(?:matched|agreed|exceeded|differed|mismatched|deviated)"
    r"|(?:all|only|just|in|for|across)\s+\d+\s+" + _COUNTED + r"(?![\w-])"
    r"|\d+(?:[.,]\d+)?\s*(?:%|‰|percent|per cent|ppm|parts per million)"
    r"|\d+(?:[.,]\d+)?(?:[eE][-+]?\d+)?\s*" + _UNIT + r"(?![\w/])(?!\s*(?:per|/)(?![\w]))"
    r"|\d+(?:\.\d+)?[eE][-+]?\d+)", re.IGNORECASE)


def scan_tokens(label: str, lines: dict[int, str], terms: dict[str, list[str]], added: set[int] | None = None,
                blank_breaks: bool = True) -> list[str]:
    """Private tokens in `lines`; callers pass prose before any owner-words or policy-text exemption."""

    return _token_findings(label, lines, list(enumerate(terms["tokens"], 1)), added, "private token", blank_breaks)


def scan(label: str, lines: dict[int, str], terms: dict[str, list[str]], added: set[int] | None = None,
         blank_breaks: bool = True) -> list[str]:
    """Findings in `lines` (the whole prose); with `added`, only matches that involve an added line."""

    findings = scan_tokens(label, lines, terms, added, blank_breaks)
    names = [_word(term) for term in terms["names"]]
    for text, owners, starts in _paragraphs(lines, blank_breaks):
        text = _unpathed(text)
        sentences = _sentences(text, owners, starts)
        named = [(_span(owners, m.start(), m.end()), sentences[m.start()])
                 for pattern in names + [TARGET_REFERENCE] for m in pattern.finditer(text)]
        surveyed = [(_span(owners, m.start(), m.end()), sentences[m.start()]) for m in SURVEY.finditer(text)]
        if not named and not surveyed:
            continue
        masked = text
        for pattern in names:
            masked = pattern.sub(lambda m: "@" * len(m.group()), masked)
        # Unit conversions (`1 in = 25.4 mm`) are format facts; a sign before a number (`-3 mm`) is kept
        # apart from ids (`REQ-018`) by requiring a space, bracket or start before it.
        masked = EQUATION.sub(lambda m: " " * len(m.group()) if m.group(1) != m.group(2) else m.group(), masked)
        masked = SIGN.sub(" ", masked)
        numbers = [(_span(owners, m.start(), m.end()), sentences[m.start()]) for m in RESULT_NUMBER.finditer(masked)]

        def near(span: list[int], sentence: int, targets: list[tuple[list[int], int]]) -> list[int]:
            """Lines of the target references in the hit's sentence, else of those within one line of it."""

            same = [line for target_span, target_sentence in targets if target_sentence == sentence for line in target_span]
            return same or [line for target_span, _ in targets
                            if not (target_span[-1] < span[0] - 1 or target_span[0] > span[-1] + 1) for line in target_span]

        for kind, pattern in (("defect or limit about the target tool", DEFECT),
                              ("result about the target tool", COMPARISON), ("screenshot reference", SCREENSHOT)):
            for match in pattern.finditer(text):
                span, sentence = _span(owners, match.start(), match.end()), sentences[match.start()]
                # "matches ... to X" maps records; it does not compare results.
                if pattern is COMPARISON and match.group().lower() in ("match", "matches") \
                        and MAPPING_TO.match(text, match.end()):
                    continue
                involved = near(span, sentence, named + surveyed if pattern is SCREENSHOT else named)
                if not involved:
                    continue
                if pattern is COMPARISON:
                    values = [number_span for number_span, number_sentence in numbers if number_sentence == sentence]
                    if not values:
                        continue
                    involved += [line for value in values for line in value]
                if added is None or added.intersection(span + involved):
                    findings.append(f"{label}:{min(span + involved)}: {kind}")
    return list(dict.fromkeys(findings))


def _git(repo: Path, *args: str) -> str:
    return subprocess.run(["git", "-C", str(repo), *args], capture_output=True, text=True, check=True).stdout


def added_line_numbers(repo: Path, base: str, head: str) -> dict[str, set[int]]:
    # Paths come NUL-separated and are passed back literally, so quoted or non-ASCII names are never misparsed.
    paths = [path for path in _git(repo, "diff", "-z", "--name-only", "--no-renames", "--diff-filter=d",
                                   f"{base}...{head}").split("\0") if path]
    added: dict[str, set[int]] = {}
    for path in paths:
        diff = _git(repo, "--literal-pathspecs", "diff", "--unified=0", "--no-color", "--no-renames",
                    f"{base}...{head}", "--", path)
        for line in diff.splitlines():
            match = re.match(r"@@ -\S+ \+(\d+)(?:,(\d+))? @@", line)
            if match:
                start, count = int(match.group(1)), int(match.group(2) or "1")
                added.setdefault(path, set()).update(range(start, start + count))
    return {path: numbers for path, numbers in added.items() if numbers}


def scan_data_tokens(label: str, lines: dict[int, str], terms: dict[str, list[str]],
                     added: set[int] | None = None) -> list[str]:
    """Private tokens on every line of test and fixture files, code and data included."""

    return _token_findings(label, lines, list(enumerate(terms["tokens"], 1)), added, "private token")


def _data_path(path: str) -> bool:
    parts = Path(path).parts
    return "tests" in parts or "fixtures" in parts


# The check's own source and tests necessarily name the kinds of content it screens.
SELF_PATHS = ("tools/caepipe_correlation/check_public_observations.py", "tests/test_check_public_observations.py")


def check_range(repo: Path, base: str, head: str, terms: dict[str, list[str]],
                messages_base: str | None = None) -> list[str]:
    findings: list[str] = []
    for path, numbers in sorted(added_line_numbers(repo, base, head).items()):
        text = _git(repo, "show", f"{head}:{path}")
        blank_breaks = Path(path).suffix.lower() not in CODE_SUFFIXES
        # Private tokens are screened in every file, before the owner-words and policy-text exemptions.
        findings += scan_tokens(path, prose_of(path, text, exempt=False), terms, numbers, blank_breaks)
        # A fixture data file (CSV) is consumed by code like code itself; only the private-token rule applies.
        if not path.endswith(SELF_PATHS) and not (_data_path(path) and Path(path).suffix.lower() == ".csv"):
            findings += scan(path, prose_of(path, text), terms, numbers, blank_breaks)
        if _data_path(path):
            findings += scan_data_tokens(path, dict(enumerate(text.splitlines(), 1)), terms, numbers)
    # Commit messages already published cannot change; a pre-push run screens only the new ones.
    for commit in _git(repo, "rev-list", f"{messages_base or base}..{head}").split():
        message = _git(repo, "log", "-1", "--format=%B", commit)
        findings += scan(f"commit {commit[:10]}", dict(enumerate(message.splitlines(), 1)), terms)
    return list(dict.fromkeys(findings))


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--terms-file", required=True, type=Path)
    parser.add_argument("--repo", type=Path, default=Path.cwd())
    parser.add_argument("--base")
    parser.add_argument("--head", default="HEAD")
    parser.add_argument("--messages-base", help="screen commit messages only after this ref (default: --base)")
    parser.add_argument("--file", action="append", default=[], type=Path, help="a whole file to screen as prose")
    args = parser.parse_args(argv)
    try:
        terms = load_terms(args.terms_file)
        findings: list[str] = []
        if args.base:
            findings += check_range(args.repo, args.base, args.head, terms, args.messages_base)
        for path in args.file:
            text = path.read_text(encoding="utf-8")
            blank_breaks = path.suffix.lower() not in CODE_SUFFIXES
            findings += scan_tokens(str(path), prose_of(str(path), text, exempt=False), terms, blank_breaks=blank_breaks)
            findings += [item for item in scan(str(path), prose_of(str(path), text), terms, blank_breaks=blank_breaks)
                         if item not in findings]
    except (OSError, ValueError, UnicodeDecodeError, subprocess.CalledProcessError):
        print("check_public_observations: operational error (terms file, refs or files unreadable)", file=sys.stderr)
        return 2
    for finding in findings:
        print(finding)
    print(f"check_public_observations: {len(findings)} finding(s)")
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
