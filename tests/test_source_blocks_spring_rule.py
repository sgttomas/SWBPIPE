"""The source-block verifier's support-reaction spring decision is the solver's.

`tests/verifiers/source_blocks.py` (`_supports`, ACTUAL_SUPPORT_LAW) checks the published action terms against the
authored support. The solver (`core/product_physics/src/lib.rs`) makes a support a ground spring when its family is
exactly `spring` or `is_variable_spring_hanger` holds, which reads `support_hanger_type`: the explicit
`hanger.hanger_type` trimmed by Rust's `str::trim` when that is non-empty, otherwise the trimmed family. Rust's
reader mirrors it (`solver_spring_support`, `core/reporting/result_export/src/source_blocks.rs`), and so does the
verifier. The solver's answers are an explicit table read from its text, which the first test guards; the grid is
the Rust reader's (`solver_spring_rule_tests`): every schema `SupportFamily`, an absent, null, empty, unknown and
padded family, crossed with no hanger record, a null record, a record without a type or with a null type, every
schema `SpringHanger.hanger_type`, and an empty, unknown, padded and padding-only type. Padding uses ASCII spaces,
U+0085 (which `str::trim` removes) and U+FEFF and U+001C (which it keeps).
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
NEL, BOM, FS = "\u0085", "﻿", "\u001c"

# The solver's text the tables are read from (`core/product_physics/src/lib.rs`), verbatim.
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
    """            if support.family.as_deref() == Some("spring") || is_variable_spring_hanger(support) {
                let stiffness = support_stiffness_input(support).and_then(|input| {""",
)
ABSENT = object()


def _hanger_cases():
    """(name, hanger record, decision): `None` when the trimmed explicit type is absent or empty and the family
    decides, else whether the type is a variable spring hanger."""
    typed = lambda t, decides: (f"hanger_type {t!r}", {"hanger_type": t}, decides)
    return [
        ("no hanger record", ABSENT, None), ("null hanger record", None, None),
        ("no hanger_type", {}, None), ("null hanger_type", {"hanger_type": None}, None),
        typed("variable_spring_hanger", True), typed("spring_hanger", True), typed("constant_effort_support", False),
        typed("", None), typed("strut", False), typed("spring", False),
        typed(" spring_hanger ", True), typed(f"{NEL}spring_hanger{NEL}", True),
        typed(f"{BOM}spring_hanger{BOM}", False), typed(f"{FS}spring_hanger{FS}", False),
        typed("  ", None), typed(NEL, None), typed(BOM, False), typed(FS, False),
    ]


def _family_cases():
    """(name, family, exactly `spring`, whether the trimmed family is a variable spring hanger)."""
    named = lambda f, spring, hanger: (f"family {f!r}", f, spring, hanger)
    cases = [
        named("anchor", False, False), named("guide", False, False), named("line_stop", False, False),
        named("vertical_support", False, False), named("spring", True, False),
        named("variable_spring_hanger", False, True), named("spring_hanger", False, True),
        named("constant_effort_support", False, False), named("nonlinear", False, False),
        ("no family", ABSENT, False, False), ("null family", None, False, False),
        named("", False, False), named("strut", False, False),
    ]
    for pad in (" ", NEL, BOM, FS):
        cases.append(named(f"{pad}spring_hanger{pad}", False, pad not in (BOM, FS)))
        # `spring` is compared untrimmed: padded, it is no ground spring.
        cases.append(named(f"{pad}spring{pad}", False, False))
    return cases


def _grid():
    for family_name, family, exactly_spring, family_hanger in _family_cases():
        for hanger_name, hanger, decides in _hanger_cases():
            declaration = {}
            if family is not ABSENT:
                declaration["family"] = family
            if hanger is not ABSENT:
                declaration["hanger"] = hanger
            solver = exactly_spring or (family_hanger if decides is None else decides)
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


def test_the_tables_are_read_from_the_solver_as_it_stands():
    solver = (ROOT / "core/product_physics/src/lib.rs").read_text()
    for text in SOLVER_SOURCE:
        assert text in solver
    schema = json.loads((ROOT / "schemas/model.schema.yaml").read_text())
    families = schema["$defs"]["SupportFamily"]["enum"]
    types = schema["$defs"]["SpringHanger"]["properties"]["hanger_type"]["enum"]
    assert [case[1] for case in _family_cases()][:len(families)] == families
    hanger_types = [case[1].get("hanger_type") for case in _hanger_cases() if isinstance(case[1], dict)]
    assert set(types) <= set(hanger_types)
    assert len(list(_grid())) == len(_family_cases()) * len(_hanger_cases())


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
    assert sb._solver_spring_support(declaration) is solver
    _decides(declaration, solver)


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


@pytest.mark.parametrize(("declaration", "spring"), [
    ({"family": "spring_hanger", "hanger": {"hanger_type": "constant_effort_support"}}, False),
    ({"family": "variable_spring_hanger", "hanger": {"hanger_type": "strut"}}, False),
])
def test_an_explicit_hanger_type_decides_over_the_family(declaration, spring):
    _decides(declaration, spring)
