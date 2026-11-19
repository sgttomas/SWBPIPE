"""B1 (DEL-05-02 REQ-012): the static census of applied-load ledger append sites.

It reads the production Rust source of core/product_physics/src, core/solver/nonlinear_integration/src and
core/solver/frame_kernel/src (files named `*_tests.rs` or `tests.rs`, files under a `tests` or `*_tests` directory,
and items under `#[cfg(test)]` are excluded) and lists, each with its enclosing function:
- every call of the four ledger append functions by name and argument count: `.push(` with 3 arguments,
  `.push_product(` with 4, `.push_formed(` with 6 and `.push_formed_product(` with 7, or a path call
  (`LoadLedger::push(` and so on) with one more;
- the term producers that bypass `push`: calls of `.with_appended(` and `.force_scaled(`;
- the non-ledger force writes: an indexed assignment (`=`, `+=`, `-=`, `*=`) to a name containing `force`, and each
  `ForceContribution {` literal.
Each list must equal its census below exactly, so a new, moved-out or removed site fails until it is classified.

The scan covers this named syntax only. A call outside it is not seen: a `.push(` with another argument count (a
`Vec::push` has one), a call through a function pointer or closure, a call inside a macro's own expansion, a
re-export or method under another name, or a write through another binding. A missed append is caught at run time
only on a fixture of `core/product_physics/tests/ledger_site_observer.rs` that reaches it. A `.push(` with 3
arguments, or `.force_scaled(`, is counted whatever its receiver's type: the census names which ones are ledger
or `AssembledForce` calls.

It also holds one pending path (§8.5): the hanger cold load has no ledger site, so no production source names
`spring_preload`, and no function holding an append site reads `cold_load`.
"""
from collections import Counter
import os
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
SCANNED = ("core/product_physics/src", "core/solver/nonlinear_integration/src", "core/solver/frame_kernel/src")
PP = "core/product_physics/src/"
NI = "core/solver/nonlinear_integration/src/"
FK = "core/solver/frame_kernel/src/"

ARITY = {"push": 3, "push_product": 4, "push_formed": 6, "push_formed_product": 7}

# (file, enclosing function, callee): (count, class). The classes are the ledger each append reaches.
APPENDS = {
    (PP + "certified_loads.rs", "spring_base_motion", "push_formed"): (1, "case ledger"),
    (PP + "certified_loads.rs", "nodal", "push_formed"): (1, "case ledger"),
    (PP + "certified_loads.rs", "nodal", "push"): (1, "case ledger"),
    (PP + "lib.rs", "push_nodal_loads", "push"): (1, "retained replay ledger"),
    (PP + "lib.rs", "add_connector_reference_loads", "push_formed"): (1, "case ledger"),
    (PP + "lib.rs", "add_tie_rod_reference_loads", "push_formed"): (1, "case ledger"),
    (PP + "lib.rs", "push_exact_pressure_operands", "push_formed"): (1, "case ledger"),
    (PP + "lib.rs", "add_uniform_element_loads", "push_formed"): (3, "case ledger"),
    (PP + "lib.rs", "add_thermal_equivalent_loads", "push_formed"): (2, "case ledger"),
    (PP + "lib.rs", "add_curved_bend_thermal_equivalent_load", "push_formed_product"): (1, "case ledger"),
    (PP + "lib.rs", "add_curved_bend_pressure_equivalent_load", "push_formed_product"): (1, "case ledger"),
    (PP + "lib.rs", "add_constant_effort_support_loads", "push"): (1, "case ledger"),
    (PP + "source_recovery.rs", "prepare_sources", "push"): (1, "source recovery fold"),
    (PP + "source_recovery.rs", "close_load_state", "push"): (2, "source recovery fold"),
    (NI + "confirmation.rs", "appended_ledger", "push_formed"): (2, "confirmation extra (with_appended)"),
    (NI + "confirmation.rs", "appended_ledger", "push"): (1, "confirmation extra (with_appended)"),
    (NI + "confirmation.rs", "stage_a", "push"): (1, "confirmation influence unit"),
    (FK + "load_ledger.rs", "push_formed", "push"): (1, "kernel wrapper (track_caller)"),
    (FK + "load_ledger.rs", "push_formed_product", "push_product"): (1, "kernel wrapper (track_caller)"),
    (FK + "restraint_basis.rs", "transform_force", "push"): (1, "skew basis, blocked on the exact route"),
    (FK + "restraint_basis.rs", "transform_force", "push_product"): (5, "skew basis, blocked on the exact route"),
}

# (file, enclosing function, callee): (count, what it is). `force_scaled` is also a method of other types.
PRODUCERS = {
    (NI + "confirmation.rs", "confirm_with_applied", "with_appended"): (4, "AssembledForce::with_appended (appended_ledger)"),
    (NI + "structural_adapter.rs", "solve_force_scaled", "force_scaled"): (2, "AssembledForce::force_scaled"),
    (NI + "structural_adapter.rs", "solve_force_scaled_with_formation_check", "force_scaled"): (2, "AssembledForce::force_scaled"),
    (FK + "structural/sparse.rs", "force_scaled_reactions", "force_scaled"): (1, "AssembledForce::force_scaled (reactions)"),
    (FK + "load_ledger.rs", "force_scaled", "force_scaled"): (1, "a load primitive record inside AssembledForce::force_scaled"),
    (NI + "structural_adapter.rs", "force_scaled_primitives", "force_scaled"): (1, "a connector, not a load term"),
    (NI + "structural_adapter.rs", "new", "force_scaled"): (1, "a frame element, not a load term"),
    (FK + "structural/sparse.rs", "force_scaled_inputs", "force_scaled"): (1, "a frame element, not a load term"),
    (FK + "lib.rs", "force_scaled_end_actions", "force_scaled"): (1, "a frame element, not a load term"),
    (FK + "connector.rs", "force_scaled", "force_scaled"): (1, "a tie rod, not a load term"),
    (FK + "structural/formation_check.rs", "load_closure", "force_scaled"): (1, "a tie rod, not a load term"),
    (FK + "structural/formation_check.rs", "primitive_frame_bound", "force_scaled"): (1, "a frame element, not a load term"),
}

# (file, enclosing function, written name or `ForceContribution`): (count, what it is).
FORCE_WRITES = {
    (NI + "lib.rs", "solve_iteration_with_sliding_friction_evidence", "unit_force"): (1, "influence perturbation, auxiliary"),
    (NI + "lib.rs", "add_applied_forces", "force"): (1, "support response: friction on a copy, not observed at run time"),
    (NI + "structural_adapter.rs", "scrutinize_gaps", "ForceContribution"): (2, "gap scrutiny's exact solve input"),
    (PP + "source_recovery.rs", "prepare_sources", "ForceContribution"): (1, "source recovery's exact solve input"),
    (PP + "source_recovery.rs", "close_load_state", "ForceContribution"): (2, "source recovery's exact solve input"),
    (FK + "lib.rs", "reduce_system_for_boundary", "reduced_force"): (1, "the binary64 reduction's reduced right-hand side"),
    (PP + "self_weight.rs", "component_basis", "force_vector"): (1, "a component weight's wrench record, not a term"),
    (PP + "self_weight.rs", "component_primitive", "force_vector"): (1, "a component weight's wrench record, not a term"),
    (PP + "lib.rs", "curved_bend_section_resultants", "node_j_force"): (1, "recovery of section resultants"),
    (PP + "lib.rs", "recover_curved_bend_local_forces", "local_forces"): (1, "recovery of local forces"),
    (FK + "connector.rs", "recover", "force_global"): (1, "connector recovery"),
    (FK + "structural/certified_recovery.rs", "certify_arc_recovery", "force"): (1, "arc recovery certificate"),
}


def blank(text):
    """Comments, string and char literal contents blanked, length and newlines kept."""
    out = list(text)
    i, n = 0, len(text)

    def wipe(a, b):
        for q in range(a, b):
            if out[q] != "\n":
                out[q] = " "

    while i < n:
        if text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            wipe(i, j)
            i = j
            continue
        if text.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif text.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            wipe(i, j)
            i = j
            continue
        m = re.match(r'b?r(#*)"', text[i:i + 300])
        if m and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            end = text.find('"' + m.group(1), i + len(m.group(0)))
            end = n if end < 0 else end + 1 + len(m.group(1))
            wipe(i + len(m.group(0)), end - 1 - len(m.group(1)))
            i = end
            continue
        if text[i] == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            wipe(i + 1, j)
            i = j + 1
            continue
        if text[i] == "'":
            if text.startswith("\\", i + 1):
                j = text.find("'", i + 3)
                j = n if j < 0 else j
                wipe(i + 1, j)
                i = j + 1
                continue
            if i + 2 < n and text[i + 2] == "'":
                wipe(i + 1, i + 2)
                i += 3
                continue
        i += 1
    return "".join(out)


def matching(code, i, open_, close):
    """The index of the bracket closing the one at `i`."""
    depth = 0
    for j in range(i, len(code)):
        if code[j] == open_:
            depth += 1
        elif code[j] == close:
            depth -= 1
            if depth == 0:
                return j
    raise ValueError("unbalanced brackets")


def blank_test_items(code):
    """Items under `#[cfg(test)]` blanked (the attribute, further attributes, and the item to its `;` or block)."""
    out = list(code)
    for m in re.finditer(r"#\[cfg\(test\)\]", code):
        j = m.end()
        while True:
            while j < len(code) and code[j].isspace():
                j += 1
            if code.startswith("#[", j):
                j = matching(code, j + 1, "[", "]") + 1
                continue
            break
        k, depth = j, 0
        while k < len(code):
            c = code[k]
            if c in "([":
                depth += 1
            elif c in ")]":
                depth -= 1
            elif depth == 0 and c == ";":
                break
            elif depth == 0 and c == "{":
                k = matching(code, k, "{", "}")
                break
            k += 1
        for q in range(m.start(), min(k + 1, len(code))):
            if out[q] != "\n":
                out[q] = " "
    return "".join(out)


def production_files():
    files = {}
    for root in SCANNED:
        for directory, dirs, names in os.walk(ROOT / root):
            dirs[:] = sorted(d for d in dirs if d != "tests" and not d.endswith("_tests"))
            for name in sorted(names):
                if name.endswith(".rs") and not name.endswith("_tests.rs") and name != "tests.rs":
                    path = Path(directory) / name
                    files[path.relative_to(ROOT).as_posix()] = blank_test_items(blank(path.read_text(encoding="utf-8")))
    return files


def functions(code):
    """(name, body start, body end) of every `fn` with a body."""
    out = []
    for m in re.finditer(r"\bfn\s+([A-Za-z_]\w*)", code):
        j, depth = m.end(), 0
        while j < len(code):
            c = code[j]
            if c in "(<[":
                depth += 1
            elif c in ")>]" and not (c == ">" and code[j - 1] == "-"):
                depth -= 1
            elif depth <= 0 and c == ";":
                j = -1
                break
            elif depth <= 0 and c == "{":
                break
            j += 1
        if 0 <= j < len(code):
            out.append((m.group(1), j, matching(code, j, "{", "}")))
    return out


def enclosing(fns, offset):
    inside = [(end - start, name) for name, start, end in fns if start < offset < end]
    return min(inside)[1] if inside else None


def arguments(code, open_paren):
    """The number of top-level arguments of the call whose `(` is at `open_paren`."""
    close = matching(code, open_paren, "(", ")")
    inner = code[open_paren + 1:close]
    if not inner.strip():
        return 0
    count, depth = 1, 0
    for c in inner:
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        elif c == "," and depth == 0:
            count += 1
    return count - (1 if inner.rstrip().endswith(",") else 0)


def scan():
    appends, producers, writes, shapes, lines = Counter(), Counter(), Counter(), [], {}
    call = re.compile(r"(\.|::)\s*(push_formed_product|push_formed|push_product|push|with_appended|force_scaled)\s*\(")
    write = re.compile(r"\b(\w*force\w*)\s*\[")
    for rel, code in production_files().items():
        fns = functions(code)
        for m in call.finditer(code):
            name, path_form = m.group(2), m.group(1) == "::"
            if path_form and name in ("with_appended", "force_scaled"):
                continue
            where = (rel, enclosing(fns, m.start()), name)
            line = code.count("\n", 0, m.start()) + 1
            if name in ("with_appended", "force_scaled"):
                producers[where] += 1
                continue
            args = arguments(code, m.end() - 1) - (1 if path_form else 0)
            if args == ARITY[name]:
                appends[where] += 1
                lines.setdefault(where, []).append(line)
            elif name != "push":
                shapes.append(f"{rel}:{line}: {name} with {args} arguments")
        for m in write.finditer(code):
            close = matching(code, m.end() - 1, "[", "]")
            if re.match(r"\s*([+\-*]?=)(?!=)", code[close + 1:]):
                writes[(rel, enclosing(fns, m.start()), m.group(1))] += 1
        for m in re.finditer(r"(?<!struct )\bForceContribution\s*\{", code):
            writes[(rel, enclosing(fns, m.start()), "ForceContribution")] += 1
    return appends, producers, writes, shapes, lines


SCAN = scan()


def census(table):
    return Counter({key: count for key, (count, _) in table.items()})


def test_every_append_call_is_in_the_census():
    appends, _, _, shapes, lines = SCAN
    assert shapes == [], "an append name called with another argument count; classify it"
    assert appends == census(APPENDS), {k: (v, lines.get(k)) for k, v in (appends - census(APPENDS)).items()} or \
        dict(census(APPENDS) - appends)


def test_every_term_producer_bypassing_push_is_in_the_census():
    _, producers, _, _, _ = SCAN
    assert producers == census(PRODUCERS), (dict(producers - census(PRODUCERS)), dict(census(PRODUCERS) - producers))


def test_every_non_ledger_force_write_is_in_the_census():
    _, _, writes, _, _ = SCAN
    assert writes == census(FORCE_WRITES), (dict(writes - census(FORCE_WRITES)), dict(census(FORCE_WRITES) - writes))


def test_the_hanger_cold_load_has_no_ledger_site():
    files = production_files()
    assert not [rel for rel, code in files.items() if "spring_preload" in code]
    for (rel, function, _), _ in APPENDS.items():
        bodies = [code for name, start, end in functions(files[rel]) for code in [files[rel][start:end]] if name == function]
        assert bodies and not any("cold_load" in body for body in bodies), (rel, function)


def test_the_scan_sees_its_named_shapes_and_not_others():
    code = blank_test_items(blank(
        'fn a(l: &mut LoadLedger) { l.push("s", 0, 1.0); v.push(x); l.push_formed("s", 0, 1.0, f, 0.0, false); }\n'
        'fn b() { LoadLedger::push(&mut l, "s", 0, 1.0); let p = LoadLedger::push; p(&mut l, "s", 0, 1.0); }\n'
        '// l.push("comment", 0, 1.0);\n'
        '#[cfg(test)]\nmod tests { fn c() { l.push("t", 0, 1.0); } }\n'))
    fns = functions(code)
    found = [(enclosing(fns, m.start()), m.group(2), arguments(code, m.end() - 1) - (m.group(1) == "::"))
             for m in re.finditer(r"(\.|::)\s*(push_formed|push)\s*\(", code)]
    assert found == [("a", "push", 3), ("a", "push", 1), ("a", "push_formed", 6), ("b", "push", 3)]
