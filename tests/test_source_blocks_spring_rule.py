"""The source-block verifier's support-reaction decisions are the solver's.

`tests/verifiers/source_blocks.py` (`_supports`) checks the published action terms against the authored support. The
solver (`core/product_physics/src/lib.rs`) builds no support from a declaration for which
`is_constant_effort_support` holds, before its spring decision, and source recovery declines any model holding one;
the verifier refuses a plan naming one (CONSTANT_EFFORT_SUPPORT). Otherwise the solver makes a support a ground spring
when its family is exactly `spring` or `is_variable_spring_hanger` holds (ACTUAL_SUPPORT_LAW). Both read
`support_hanger_type`: the explicit `hanger.hanger_type` trimmed by Rust's `str::trim` when that is non-empty,
otherwise the trimmed family. Rust's reader mirrors them (`solver_built_support`, `solver_spring_support`,
`core/reporting/result_export/src/source_blocks.rs`), and so does the verifier. The solver's answers are an explicit
table read from its text, which the first test guards; the grid is the Rust reader's (`solver_spring_rule_tests`):
every schema `SupportFamily`, an absent, null, empty, unknown and padded family, crossed with no hanger record, a null
record, a record without a type or with a null type, every schema `SpringHanger.hanger_type`, and an empty, unknown,
padded and padding-only type. Padding uses ASCII spaces, U+0085 (which `str::trim` removes) and U+FEFF and U+001C
(which it keeps).
"""
from __future__ import annotations

import json
from pathlib import Path
import sys
from types import SimpleNamespace

import pytest

from tests.verifiers import source_blocks as sb

ROOT = Path(__file__).resolve().parents[1]
LAW = "SOURCE_BLOCKS_ACTUAL_SUPPORT_LAW"
CONSTANT_EFFORT = "SOURCE_BLOCKS_CONSTANT_EFFORT_SUPPORT"
NEL, BOM, FS = "\u0085", "﻿", "\u001c"

# The solver's text the tables are read from (`core/product_physics/src/lib.rs`), compared through `_normalized`.
SOLVER_SOURCE = (
    """pub(crate) fn support_hanger_type(support: &PreviewSupport) -> Option<&str> {
    support
        .hanger
        .as_ref()
        .and_then(|hanger| hanger.hanger_type.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| support.family.as_deref().map(str::trim))
}""",
    """pub(crate) fn is_variable_spring_hanger(support: &PreviewSupport) -> bool {
    matches!(
        support_hanger_type(support),
        Some("variable_spring_hanger" | "spring_hanger")
    )
}""",
    """pub(crate) fn is_constant_effort_support(support: &PreviewSupport) -> bool {
    matches!(
        support_hanger_type(support),
        Some("constant_effort_support")
    )
}""",
)
# The linear-support assembly's exclusion of a constant-effort support, which precedes its spring decision in the
# same closure.
EXCLUSION = """            if is_constant_effort_support(support) {
                return None;
            }
            let Some(&node) = node_map.get(support.node.as_str()) else {"""
DECISION = """            if support.family.as_deref() == Some("spring") || is_variable_spring_hanger(support) {
                let stiffness = support_stiffness_input(support).and_then(|input| {"""
ABSENT = object()
HANGER, CONSTANT, OTHER = "hanger", "constant_effort", "other"  # how a declaration reads, by `support_hanger_type`
SPRING, RIGID, NOT_BUILT = "spring", "rigid", "not_built"  # what the solver builds from it


def _normalized(text: str) -> str:
    """Rust's `normalized` (`solver_spring_rule_tests`): `text` with comments dropped and whitespace outside string and
    char literals collapsed, one space kept only between two word characters or two operator characters, where
    dropping it could join two tokens. Literals are kept verbatim, so a change inside one still counts."""
    word = lambda c: c.isalnum() or c == "_"
    operator = lambda c: c in "!#$%&*+-./:<=>?@^|~"
    at = lambda i: text[i] if i < len(text) else None
    out, gap, i = [], False, 0
    while i < len(text):
        c = text[i]
        if c.isspace():
            gap, i = True, i + 1
            continue
        if text.startswith("//", i):
            while i < len(text) and text[i] != "\n":
                i += 1
            gap = True
            continue
        if text.startswith("/*", i):
            depth = 0
            while i < len(text):
                if text.startswith("/*", i):
                    depth, i = depth + 1, i + 2
                elif text.startswith("*/", i):
                    depth, i = depth - 1, i + 2
                    if depth == 0:
                        break
                else:
                    i += 1
            gap = True
            continue
        last = out[-1][-1] if out else None
        joined = last is not None and word(last) and not gap
        quote = ('"', "'")
        if gap and last is not None and (
            (word(last) and word(c))
            or (operator(last) and operator(c))
            or (word(last) and c in quote)
            or (last in quote and word(c))
        ):
            out.append(" ")
        gap = False
        prefix = i
        if not joined and c == "b" and at(i + 1) in ('"', "'", "r"):
            prefix += 1
        end = None
        if not joined and at(prefix) == "r" and at(prefix + 1) in ('"', "#"):
            hashes = len(text[prefix + 1:]) - len(text[prefix + 1:].lstrip("#"))
            opening = prefix + 1 + hashes
            if at(opening) == '"':
                close = '"' + "#" * hashes
                found = text.find(close, opening + 1)
                end = len(text) if found < 0 else found + len(close)
        elif at(prefix) == '"':
            j = prefix + 1
            while j < len(text) and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            end = min(j + 1, len(text))
        elif at(prefix) == "'" and at(prefix + 1) == "\\":
            j = prefix + 3
            while j < len(text) and text[j] != "'":
                j += 1
            end = min(j + 1, len(text))
        elif at(prefix) == "'" and at(prefix + 2) == "'":
            end = prefix + 3
        end = i + 1 if end is None else end
        out.append(text[i:end])
        i = end
    return "".join(out)


def _pinned(solver: str) -> bool:
    """The pinned functions are present and the exclusion precedes the decision in the same closure, all read through
    `_normalized`."""
    solver = _normalized(solver)
    if not all(_normalized(text) in solver for text in SOLVER_SOURCE):
        return False
    exclusion = solver.find(_normalized(EXCLUSION))
    decision = solver.find(_normalized(DECISION), max(exclusion, 0))
    return 0 <= exclusion < decision and "fn " not in solver[exclusion:decision]


def _hanger_cases():
    """(name, hanger record, reading): `None` when the trimmed explicit type is absent or empty and the family
    decides, else the type's reading."""
    typed = lambda t, decides: (f"hanger_type {t!r}", {"hanger_type": t}, decides)
    cases = [
        ("no hanger record", ABSENT, None), ("null hanger record", None, None),
        ("no hanger_type", {}, None), ("null hanger_type", {"hanger_type": None}, None),
        typed("variable_spring_hanger", HANGER), typed("spring_hanger", HANGER), typed("constant_effort_support", CONSTANT),
        typed("", None), typed("strut", OTHER), typed("spring", OTHER),
        typed(" spring_hanger ", HANGER), typed(f"{NEL}spring_hanger{NEL}", HANGER),
        typed(f"{BOM}spring_hanger{BOM}", OTHER), typed(f"{FS}spring_hanger{FS}", OTHER),
        typed("  ", None), typed(NEL, None), typed(BOM, OTHER), typed(FS, OTHER),
    ]
    for pad in (" ", NEL, BOM, FS):
        cases.append(typed(f"{pad}constant_effort_support{pad}", OTHER if pad in (BOM, FS) else CONSTANT))
    return cases


def _family_cases():
    """(name, family, exactly `spring`, the trimmed family's reading)."""
    named = lambda f, spring, reading: (f"family {f!r}", f, spring, reading)
    cases = [
        named("anchor", False, OTHER), named("guide", False, OTHER), named("line_stop", False, OTHER),
        named("vertical_support", False, OTHER), named("spring", True, OTHER),
        named("variable_spring_hanger", False, HANGER), named("spring_hanger", False, HANGER),
        named("constant_effort_support", False, CONSTANT), named("nonlinear", False, OTHER),
        ("no family", ABSENT, False, OTHER), ("null family", None, False, OTHER),
        named("", False, OTHER), named("strut", False, OTHER),
    ]
    for pad in (" ", NEL, BOM, FS):
        reading = (lambda r: OTHER) if pad in (BOM, FS) else (lambda r: r)
        cases.append(named(f"{pad}spring_hanger{pad}", False, reading(HANGER)))
        # `spring` is compared untrimmed: padded, it is no ground spring.
        cases.append(named(f"{pad}spring{pad}", False, OTHER))
        cases.append(named(f"{pad}constant_effort_support{pad}", False, reading(CONSTANT)))
    return cases


def _grid():
    """The solver's assembly: a constant-effort reading is excluded first; otherwise family `spring` or a hanger
    reading is a spring."""
    for family_name, family, exactly_spring, family_reading in _family_cases():
        for hanger_name, hanger, decides in _hanger_cases():
            declaration = {}
            if family is not ABSENT:
                declaration["family"] = family
            if hanger is not ABSENT:
                declaration["hanger"] = hanger
            reading = family_reading if decides is None else decides
            solver = NOT_BUILT if reading == CONSTANT else SPRING if exactly_spring or reading == HANGER else RIGID
            yield f"{family_name} | {hanger_name}", declaration, solver


def _law(declaration, spring):
    """`_supports` on one authored support restraining UZ with UZ stiffness, publishing the producer's terms for
    the given decision (`core/product_physics/src/source_receipt/rows.rs`): a ground spring at UZ for a spring, an
    ideal constraint at UZ otherwise, structural zeros elsewhere."""
    authored = {"id": "support:s", "node": "node:0", "restraints": ["UZ"], "stiffness": {"dof": "UZ"}, **declaration}
    model = {"nodes": [{"id": "node:0"}], "supports": [authored]}
    context = SimpleNamespace(SUPPORT_SIGN="sign")
    raw, projections, components = {}, {}, []
    for slot, component in enumerate(sb.COMPONENTS):
        result_id = f"result:{slot}"
        kind = ("ground_spring" if spring else "ideal_constraint") if slot == 2 else "structural_zero"
        raw[result_id] = {"kind": "support_reaction_component_v2", "entity_ref": "support:s", "basis_ref": "basis:b",
                          "unit": "N" if slot < 3 else "N*m", "value": 1.0 if slot == 2 else 0,
                          "metadata": {"component": component, "coordinate_system": "global", "location": "node",
                                       "basis": "recovered_from_assembled_support_law", "sign_convention": "sign"}}
        projections[result_id] = {"result_id": result_id, "functional_id": f"functional:{slot}",
                                  "quantity": "support_action_component", "basis": "exact" if slot == 2 else "exact_zero"}
        components.append({"component": component, "result_id": result_id, "functional_id": f"functional:{slot}",
                           "action_terms": [{"kind": kind, "source_id": "support:s", "global_dof": slot}]})
    case = {"basis_ref": "basis:b", "supports": [{"support_id": "support:s", "node_id": "node:0", "components": components}],
            "source": {"support_ids": ["support:s"], "dof_count": 6, "prescribed_dofs": [] if spring else [2]}}
    sb._supports(case, raw, projections, model, context)


def _decides(declaration, spring):
    _law(declaration, spring)
    with pytest.raises(ValueError, match=f"^{LAW}$"):
        _law(declaration, not spring)


def _refuses(declaration):
    """Whatever terms are published for it, the support is refused by name."""
    for spring in (True, False):
        with pytest.raises(ValueError, match=f"^{CONSTANT_EFFORT}$"):
            _law(declaration, spring)


def test_the_tables_are_read_from_the_solver_as_it_stands():
    assert _pinned((ROOT / "core/product_physics/src/lib.rs").read_text())
    schema = json.loads((ROOT / "schemas/model.schema.yaml").read_text())
    families = schema["$defs"]["SupportFamily"]["enum"]
    types = schema["$defs"]["SpringHanger"]["properties"]["hanger_type"]["enum"]
    assert [case[1] for case in _family_cases()][:len(families)] == families
    hanger_types = [case[1].get("hanger_type") for case in _hanger_cases() if isinstance(case[1], dict)]
    assert set(types) <= set(hanger_types)
    assert len(list(_grid())) == len(_family_cases()) * len(_hanger_cases())
    assert {g[2] for g in _grid()} == {SPRING, RIGID, NOT_BUILT}


def test_the_guard_reads_through_formatting_and_not_through_a_change():
    """The guard reads through formatting, comments included, and fails on a change of meaning in the pinned text,
    inside a string literal included (Rust's `the_guard_reads_through_formatting_and_not_through_a_change`)."""
    solver = (ROOT / "core/product_physics/src/lib.rs").read_text()

    def edit(old, new):
        assert solver.count(old) == 1, old
        return solver.replace(old, new, 1)

    trim = "        .map(str::trim)\n        .filter(|value| !value.is_empty())\n"
    constant = '        Some("constant_effort_support")\n    )\n}'
    exclusion = "            if is_constant_effort_support(support) {\n                return None;\n            }\n            let Some(&node)"
    decision = '== Some("spring") || is_variable_spring_hanger(support) {\n                let stiffness'
    for reformatted in (
        edit(trim, "        .map(str::trim)\n\n            .filter( |value| ! value.is_empty() ) // reformatted\n"),
        edit(constant, '        Some(\n            "constant_effort_support" /* reformatted */\n        )\n    )\n}'),
        edit(exclusion, "            if is_constant_effort_support(support)\n            {\n                return None;\n            }\n\n            let Some(&node)"),
        edit(decision, '== Some("spring")\n                || is_variable_spring_hanger(support)\n            {\n                let stiffness'),
    ):
        assert _pinned(reformatted)
    for changed in (
        edit(trim, "        .map(str::trim_start)\n        .filter(|value| !value.is_empty())\n"),
        edit(trim, "        .map(str::trim)\n        .filter(|value| value.is_empty())\n"),
        edit(constant, '        Some("constant_effort_support ")\n    )\n}'),
        edit(exclusion, "            let Some(&node)"),
        edit(decision, '== Some("spring") && is_variable_spring_hanger(support) {\n                let stiffness'),
        edit(decision, '== Some("spring ") || is_variable_spring_hanger(support) {\n                let stiffness'),
    ):
        assert not _pinned(changed)


def test_normalizing_keeps_token_boundaries_at_quotes():
    # Code review r2 on #1686: each pair is two different token streams (Rust's
    # `normalizing_keeps_token_boundaries_at_quotes`).
    for spaced, joined in (("m!('a 'b)", "m!('a'b)"), ('m!(b "x")', 'm!(b"x")'), ('m!("x" y)', 'm!("x"y)')):
        assert _normalized(spaced) != _normalized(joined), spaced
    assert _normalized("m!( 'a   'b )") == _normalized("m!('a 'b)")


def test_the_trim_is_rusts():
    # Rust's `char::is_whitespace` is the Unicode White_Space property; Python's `str.isspace` adds U+001C-U+001F.
    python = {chr(c) for c in range(sys.maxunicode + 1) if chr(c).isspace()}
    assert python - sb.RUST_WHITESPACE == {"\u001c", "\u001d", "\u001e", "\u001f"}
    assert sb.RUST_WHITESPACE <= python
    assert sorted(map(ord, sb.RUST_WHITESPACE)) == [*range(0x09, 0x0E), 0x20, 0x85, 0xA0, 0x1680, *range(0x2000, 0x200B),
                                                     0x2028, 0x2029, 0x202F, 0x205F, 0x3000]
    assert sb._rust_trim(f"{NEL} x　{NEL}") == "x"
    assert sb._rust_trim(f"{BOM}x{FS}") == f"{BOM}x{FS}"


@pytest.mark.parametrize(("name", "declaration", "solver"), list(_grid()), ids=[g[0] for g in _grid()])
def test_the_verifier_decides_every_declaration_as_the_solver_does(name, declaration, solver):
    assert sb._solver_constant_effort_support(declaration) is (solver == NOT_BUILT)
    if solver == NOT_BUILT:
        _refuses(declaration)
        return
    assert sb._solver_spring_support(declaration) is (solver == SPRING)
    _decides(declaration, solver == SPRING)


# Regressions: each failed before the verifier read declarations as the solver does (at `eb9450806e`,
# ACTUAL_SUPPORT_LAW on the solver's terms).
@pytest.mark.parametrize(("declaration", "spring"), [
    ({"family": "guide", "hanger": {"hanger_type": f"{NEL}spring_hanger{NEL}"}}, True),
    ({"family": "guide", "hanger": {"hanger_type": " variable_spring_hanger "}}, True),
    ({"family": f"{NEL}spring_hanger{NEL}"}, True),
    ({"family": " variable_spring_hanger "}, True),
    ({"family": "variable_spring_hanger", "hanger": {"hanger_type": f"{BOM}spring_hanger{BOM}"}}, False),
    ({"family": "variable_spring_hanger", "hanger": {"hanger_type": f"{FS}spring_hanger{FS}"}}, False),
])
def test_a_padded_declaration_is_read_as_the_solver_trims_it(declaration, spring):
    _decides(declaration, spring)


# A non-empty explicit type decides over a hanger family. (A constant-effort type, which decided this way at
# `a58d893712`, is now refused: `test_a_constant_effort_support_is_refused`.)
@pytest.mark.parametrize(("declaration", "spring"), [
    ({"family": "spring_hanger", "hanger": {"hanger_type": "guide"}}, False),
    ({"family": "variable_spring_hanger", "hanger": {"hanger_type": "strut"}}, False),
])
def test_an_explicit_hanger_type_decides_over_the_family(declaration, spring):
    _decides(declaration, spring)


# Regressions (Rust's `a_constant_effort_support_is_refused`): the solver builds no support from a constant-effort
# declaration, so a plan naming one is refused whatever terms it publishes. Each failed at `a58d893712` and at
# `eb9450806e`: one of the two term sets was accepted (the first, Codex r1's reproduction, as a spring).
@pytest.mark.parametrize("declaration", [
    {"family": "spring", "hanger": {"hanger_type": "constant_effort_support"}},
    # By family, the explicit type absent or empty.
    {"family": "constant_effort_support"},
    {"family": "constant_effort_support", "hanger": {"hanger_type": "  "}},
    {"family": "constant_effort_support", "hanger": {"hanger_type": "constant_effort_support"}},
    # By explicit type, over any family.
    {"family": "guide", "hanger": {"hanger_type": "constant_effort_support"}},
    {"family": "spring_hanger", "hanger": {"hanger_type": "constant_effort_support"}},
    # Padded with what `str::trim` removes.
    {"family": " constant_effort_support "},
    {"family": f"{NEL}constant_effort_support{NEL}"},
    {"family": "spring", "hanger": {"hanger_type": f"{NEL}constant_effort_support{NEL}"}},
    {"family": "variable_spring_hanger", "hanger": {"hanger_type": " constant_effort_support "}},
])
def test_a_constant_effort_support_is_refused(declaration):
    _refuses(declaration)
