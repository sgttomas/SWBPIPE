"""CEF (RT-S3 constant-effort fallback publication; §8.61): the exact reviewed CE delta.

The CE correction adds one branch to `solve_prepared_load_case`'s support loop. A support with no selected source
action, no nonlinear field and `is_constant_effort_support` restates `classify_constant_effort_consumption`'s
normalized force on its one translational DOF; the other components keep +0. The same change adds two tests to
`mod tests`. No other lib.rs byte changes.

CEF-PJ: `s6_text` removes the reviewed branch and the reviewed test block only when both stand at their reviewed
anchors. The existing full-file and `solve_prepared_load_case` body pins then compare S6's reviewed bytes, so their
warrants (KD5T2, KD5C4, KD5C6, B5O1, O4, B7a, W1) bind every other byte. A moved, partial, duplicated or edited delta
is not removed, and the existing pin stops with its own message.

CEF-D1: `inspect` binds the branch's receiver, predicate, classifier, force assignment and the D1.6 admission
premise that makes the branch unreachable on every admitted model. It returns None on a source without the branch
and stops on any other form. These are source warrants, not runtime allocation observations.
"""
import hashlib
import os
import runpy

LIB = "core/product_physics/src/lib.rs"
RM = "core/product_physics/src/retained_memory.rs"

# The branch as reviewed, between its two unchanged S6 neighbours.
BEFORE = ("            if let Some(action) = source_action {\n"
          "                vector = action.values;\n")
BRANCH = ("            } else if support.nonlinear.is_none() && is_constant_effort_support(support) {\n"
          "                // DEC-049: use the same normalized consumption as the case's load ledger.\n"
          "                // The ideal device has no restraint, spring or nonlinear reaction entry.\n"
          "                if let Ok(application) = classify_constant_effort_consumption(model, support) {\n"
          "                    vector[dof_index(application.dof)] = application.force_newtons;\n"
          "                }\n")
AFTER = ("            } else {\n"
         "            if let Some(linear) = built\n")
# The loop head that owns the branch: one known-node support at a time, its six-slot action and its selected action.
LOOP_HEAD = ("    for support in &model.supports {\n"
             "        if let Some(index) = node_index(model, &support.node) {\n"
             "            let mut vector = [0.0; 6];\n"
             "            let source_action = selected_source.as_ref().and_then(|recovery| recovery.support_actions()"
             ".iter().find(|action| action.support_id == support.id));\n" + BEFORE)

# The two added tests (87 lines, test-only): from their comment to the unchanged S6 test that follows them.
TEST_START = "    // RT-S3: exercise the fallback on every axis as well as the certified\n"
TEST_NEXT = "    #[test]\n    fn constant_effort_applies_in_every_solved_load_case() {\n"
TEST_SHA256 = "3e22d2cae60b942d87c5468e9bae564a09f54b3beea7aaa9e06436dda0e56923"

# The receiver, predicate, classifier and force path the branch calls (kd5 `_indented_function` bodies). Each is
# S6's text, so the projected full-file pins bind them too; these name them for the CEF-D1 premise.
HELPER_PINS = {
    "support_hanger_type": "f84634d6256941cdaf60ea643e8b597c8b5409a4d2f25e43cb199f4d576a7179",
    "is_constant_effort_support": "2c8aa6e272faf79f167e2228474e6ab0a53b99d0ac3adbaadc5bb0067aff822b",
    "classify_constant_effort_consumption": "7a67881ff526f37d37c4ada1f3dd1201b1ad4f540348bedf8f9a5e01db3645a6",
    "parse_dof": "6e641e1a3cb5242a9f39ea4a66c7dc4edddfc91f61bdd95656e3baee55923147",
    "dof_index": "d95e6f0214dc5d24dbe7f238c09b4f1a562e616bb2679841130e16ec54b2e378",
    "node_index": "fb37e26530b2704bcbe582139c0b79300564a06faed05981eccccce965935438",
}
# The classifier's scalar success value: no owned field.
APPLICATION = ("struct ConstantEffortApplication {\n    node_index: usize,\n    dof: FrameDof,\n"
               "    force_newtons: f64,\n}\n")
# support_hanger_type reads the hanger's type first and the support family only when no non-empty hanger type exists.
HANGER_TYPE = ("        .and_then(|hanger| hanger.hanger_type.as_deref())\n"
               "        .map(str::trim)\n"
               "        .filter(|value| !value.is_empty())\n"
               "        .or_else(|| support.family.as_deref().map(str::trim))\n")
PREDICATE = '        support_hanger_type(support),\n        Some("constant_effort_support")\n'
# D1.6 on the admitted request: every hanger, then every nonlinear support, is refused before the family test, and the
# family must be one of five exact strings.
D16 = ("    // D1.6: rigid restraints and one scalar spring only; exact family strings.\n"
       "    for support in &m.supports {\n"
       "        if support.hanger.is_some() {\n"
       "            return refuse(C::Supports, F::Hanger);\n"
       "        }\n"
       "        if support.nonlinear.is_some() {\n"
       "            return refuse(C::Supports, F::Nonlinear);\n"
       "        }\n")
D16_FAMILIES = ('            if !matches!(family.as_str(), "anchor" | "guide" | "line_stop" | "vertical_support" | "spring") {\n'
                "                return refuse(C::Supports, F::SupportFamily);\n")
ADMITTED_FAMILIES = ("anchor", "guide", "line_stop", "vertical_support", "spring")


def _test_block(text):
    """The reviewed test block's (start, end) when it stands, exact, just before its S6 neighbour; else None."""
    if text.count(TEST_START) != 1 or text.count(TEST_NEXT) != 1:
        return None
    start, end = text.index(TEST_START), text.index(TEST_NEXT)
    if start >= end or hashlib.sha256(text[start:end].encode()).hexdigest() != TEST_SHA256:
        return None
    return start, end


def _branch_present(text):
    return text.count(BRANCH) == 1 and text.count(BEFORE + BRANCH + AFTER) == 1


def s6_text(text):
    """CEF-PJ: `text` without the reviewed CE delta when the whole delta stands exactly at its anchors; else `text`."""
    block = _test_block(text)
    if block is None or not _branch_present(text):
        return text
    start, end = block
    text = text[:start] + text[end:]
    return text.replace(BEFORE + BRANCH + AFTER, BEFORE + AFTER, 1)


def s6_bytes(raw):
    """CEF-PJ over a file's raw bytes (UTF-8 only; any other encoding is returned unchanged)."""
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError:
        return raw
    projected = s6_text(text)
    return raw if projected is text else projected.encode("utf-8")


def s6_body(source, body):
    """CEF-PJ for a pinned `solve_prepared_load_case` body: drop the branch only when its whole file carries the
    reviewed delta and the body holds the branch exactly once at its anchors."""
    if s6_text(source) is source or body.count(BEFORE + BRANCH + AFTER) != 1 or body.count(BRANCH) != 1:
        return body
    return body.replace(BEFORE + BRANCH + AFTER, BEFORE + AFTER, 1)


def _read(snapshot, relative):
    with open(os.path.join(snapshot, relative), encoding="utf-8") as stream:
        return stream.read()


def inspect(snapshot):
    """CEF-D1: None without the branch; the bound facts with the exact reviewed branch; SystemExit otherwise."""
    lib = _read(snapshot, LIB)
    kd5 = runpy.run_path(os.path.join(os.path.dirname(os.path.abspath(__file__)), "kd5_text_model.py"))
    solve, _ = kd5["_indented_function"](lib, "solve_prepared_load_case")
    # S6's case solve calls neither (its dispositions helper already calls the classifier); any CE spelling in the
    # case solve, or the branch anywhere, must be the reviewed delta.
    if ("classify_constant_effort_consumption" not in solve and "is_constant_effort_support" not in solve
            and "application.force_newtons" not in solve and BRANCH not in lib):
        return None
    if s6_text(lib) is lib or s6_body(lib, solve) is solve:
        raise SystemExit("CEF-PJ: the CE fallback branch or its tests are not the reviewed delta at their anchors; "
                         "re-review §8.61")
    if (solve.count(LOOP_HEAD + BRANCH + AFTER) != 1
            or solve.count("classify_constant_effort_consumption(") != 1
            or solve.count("is_constant_effort_support(") != 1
            or solve.count("application.force_newtons") != 1):
        raise SystemExit("CEF-D1: the case solve's CE call, predicate, receiver or force assignment changed; "
                         "re-review §8.61")
    # The branch holds no `?`, return, allocation spelling or fallback-error root (KD5C6, KD5T2): its one Err is
    # discarded by `if let Ok`.
    code = "".join(line for line in BRANCH.splitlines(keepends=True) if not line.lstrip().startswith("//"))
    if any(token in code for token in ("?", "return", "format!", "clone(", "to_string(", "Vec", "push(",
                                       "FallbackError", '"')):
        raise SystemExit("CEF-D1: the CE branch gained an error path, allocation or literal; re-review §8.61")
    keys = {}
    for name, digest in HELPER_PINS.items():
        body, line = kd5["_indented_function"](lib, name)
        if hashlib.sha256(body.encode()).hexdigest() != digest:
            raise SystemExit(f"CEF-D1: {LIB}:{name} changed; re-review the CE receiver, classifier and §8.61")
        keys[name] = f"{LIB}:{line}:{name}"
    hanger_type, _ = kd5["_indented_function"](lib, "support_hanger_type")
    predicate, _ = kd5["_indented_function"](lib, "is_constant_effort_support")
    classifier, _ = kd5["_indented_function"](lib, "classify_constant_effort_consumption")
    if (hanger_type.count(HANGER_TYPE) != 1 or predicate.count(PREDICATE) != 1
            or lib.count(APPLICATION) != 1 or classifier.count("force_newtons: constant_load.value,") != 1):
        raise SystemExit("CEF-D1: the hanger-type receiver, CE predicate or scalar application changed; "
                         "re-review §8.61")
    rm = _read(snapshot, RM)
    family, _ = kd5["_indented_function"](rm, "family_clauses")
    domain, _ = kd5["_indented_function"](rm, "domain_clauses")
    if (domain.count("    family_clauses(request)?;\n") != 1 or family.count("    let m = &request.model;\n") != 1
            or family.count(D16) != 1 or family.count(D16_FAMILIES) != 1
            or family.index(D16) > family.index(D16_FAMILIES)
            or "constant_effort_support" in family
            or any(name.strip() != name for name in ADMITTED_FAMILIES)):
        raise SystemExit("CEF-D1: D1.6's hanger, nonlinear or family refusal or its receiver changed; "
                         "re-review §8.61")
    # The run's solved model is the admitted request's model, moved after the retained assessment.
    run, _ = kd5["_indented_function"](lib, "run_linear_static_preview_observed_inner")
    if run.count("    let mut model = request.model;\n") != 1:
        raise SystemExit("CEF-D1: the solved model is no longer the admitted request's; re-review §8.61")
    return {"helpers": keys, "branch_lines": BRANCH.count("\n"), "test_sha256": TEST_SHA256,
            "admitted_families": list(ADMITTED_FAMILIES)}
