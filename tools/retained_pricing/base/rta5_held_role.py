"""§8.54 RTA5-HR and RTA5-WE: where lib.rs names a held case role, and the publication-basis withdrawal's D1 zero.

RTA5-DZ's premise is that a held case solve is created only inside the held solve's closure, which
`solve_held_combinations` runs and D1 never reaches. lib.rs names `CaseRole::Held(..)` at exactly three places, each
bound here by kind, exact text and place:

- HR-C, the one construction: the held closure's `solve_load_case_observed(.., held_status::CaseRole::Held(request))`,
  inside `run_linear_static_preview_observed_inner` between the `held_status::solve_held_combinations(` call and
  `let held_sources`, that is, inside the closure the call takes.
- HR-L, a destructuring: the nonlinear loop's held step, right after its D1.6 early return in
  `append_nonlinear_support_loop_results`.
- HR-W, a destructuring: the case's publication-basis withdrawal in `solve_prepared_load_case`, directly inside
  `if confirmed_state.is_some() && confirmed.is_none() {`, which calls `HeldRequest::withdraw_established`.

A destructuring reads a role that already exists and creates none, so RTA5-DZ holds with the two destructurings
present. Each `Held(` in lib.rs, qualified or not, is classed by kind: a line `if let <path>Held(..) = ..`
destructures, and every other occurrence counts as a construction. HR-C admits exactly one construction and HR-L with
HR-W exactly two destructurings, and HR-ALL requires the three to be every `Held(`. So a further construction or
destructuring, or one spelled another way, stops the run, as does moving HR-W out of its condition.

RTA5-WE, the zeroed edge `solve_prepared_load_case -> withdraw_established`, rests on two grounds, each bound here:
- WE-1 (RTA5-DZ): HR-W runs only with a held role, and a held role exists only inside the held closure (HR-C,
  HR-ALL; RTA5-DZ's own guard binds that D1 never runs it).
- WE-2 (D1.6): HR-W runs only with `confirmed_state` set. `confirmed_state` is bound once, from
  `Some(NonlinearStanding::Confirmed(state))` of `nonlinear_standing`, the loop's second result, and the loop returns
  None at its first statement when `built.nonlinear_supports` is empty, which D1.6 makes it (`family_clauses` refuses
  every `support.nonlinear`).
If reached, the edge would cost one allocation that does not scale with the model, per held case: the reason String
(`not_established`'s format of a fixed text and the reason), with the conditions dropped and no loop.

`failures(lib_text, family_clauses_text)` returns the names of the checks that fail; an empty list means all hold.
Comments (`//` to end of line) are removed before matching, as RTA5's other guards do.
"""
import re

CONSTRUCTION = "held_status::CaseRole::Held(request),"
LOOP_STEP = ("    if built.nonlinear_supports.is_empty() {\n"
             "        return None;\n"
             "    }\n"
             "    if let held_status::CaseRole::Held(request) = role {\n"
             "        return held_status::held_solve(request, ")
WITHDRAWAL = ("    if confirmed_state.is_some() && confirmed.is_none() {\n"
              "        if let held_status::CaseRole::Held(request) = &mut role {\n"
              "            let reason = match &correction_failure {\n"
              "                Some(failure) => displacement_correction_failed(model, failure),\n"
              "                None => \"the case's integrity verdict after the publication guards is not "
              "NUMERICAL_INTEGRITY_CHECKS_PASSED\".to_string(),\n"
              "            };\n"
              "            request.withdraw_established(&reason);\n"
              "        }\n"
              "    }\n")
CONFIRMED_FROM_STANDING = ("    let (confirmed_state, nonlinear_sentence) = match nonlinear_standing {\n"
                           "        Some(NonlinearStanding::Confirmed(state)) => (Some(state), None),\n"
                           "        Some(NonlinearStanding::NotEstablished(sentence)) => (None, Some(sentence)),\n"
                           "        None => (None, None),\n"
                           "    };\n")
STANDING_FROM_LOOP = ("    let (selected_nonlinear, nonlinear_standing, nonlinear_qualification, nonlinear_staged) =\n"
                      "        match append_nonlinear_support_loop_results(")
STANDING_ARMS = ("            Some((solve, nonlinear, qualification, staged)) => {\n"
                 "                (Some(solve), Some(nonlinear), qualification, staged)\n"
                 "            }\n"
                 "            None => (None, None, None, None),\n")
D16 = "        if support.nonlinear.is_some() {\n"


def _nc(text):
    return re.sub(r"//[^\n]*", "", text)


def _fn(text, name):
    """The one `fn name`'s text (to the next fn at the same or lower indent), as o10_rules' fn_lines spans it."""
    lines = text.split("\n")
    hits = [i + 1 for i, l in enumerate(lines) if re.match(r"\s*(pub(\([a-z]+\))?\s+)?(const\s+)?fn\s+" + re.escape(name) + r"\b", l)]
    if len(hits) != 1:
        return None
    start = hits[0]
    ind = len(lines[start - 1]) - len(lines[start - 1].lstrip())
    end = next((i + 1 for i in range(start, len(lines)) if re.match(r"\s*(pub(\([a-z]+\))?\s+)?(const\s+)?fn\s", lines[i])
                and len(lines[i]) - len(lines[i].lstrip()) <= ind), len(lines) + 1)
    return "\n".join(lines[start - 1:end - 1])


def failures(lib_text, family_clauses_text):
    failed = []
    lib = _nc(lib_text)
    inner = _nc(_fn(lib_text, "run_linear_static_preview_observed_inner") or "")
    loop = _nc(_fn(lib_text, "append_nonlinear_support_loop_results") or "")
    case = _nc(_fn(lib_text, "solve_prepared_load_case") or "")

    # Each `Held(` by kind: a line `if let <path>Held(..) = ..` destructures; every other occurrence constructs (or
    # matches some other way, which is counted with the constructions so that it stops too).
    held_lines = [l.strip() for l in lib.split("\n") for _ in re.findall(r"\bHeld\(", l)]
    destructurings = [l for l in held_lines if re.match(r"if let [\w:]*\bHeld\([^)]*\) = ", l)]
    constructions = [l for l in held_lines if l not in destructurings]
    # HR-C: one construction, by its exact text, inside the held closure.
    call, end = inner.find("held_status::solve_held_combinations("), inner.find("let held_sources: HashMap<String, serde_json::Value> = model")
    at = inner.find(CONSTRUCTION)
    if not (constructions == [CONSTRUCTION] and inner.count(CONSTRUCTION) == 1 and 0 <= call < at < end):
        failed.append("HR-C (the one construction, inside the held closure)")
    # HR-L: the loop's destructuring, by its exact text, right after its D1.6 early return.
    if not (destructurings.count("if let held_status::CaseRole::Held(request) = role {") == 1 and loop.count(LOOP_STEP) == 1
            and loop.find("{\n" + LOOP_STEP) == loop.find("{\n")):
        failed.append("HR-L (the loop's destructuring after its early return)")
    # HR-W: the withdrawal's destructuring, by its exact text, directly inside its condition, once; with HR-L, the only two.
    if not (destructurings.count("if let held_status::CaseRole::Held(request) = &mut role {") == 1 and len(destructurings) == 2
            and case.count(WITHDRAWAL) == 1
            and lib.count("if confirmed_state.is_some() && confirmed.is_none() {") == 1
            and lib.count("withdraw_established(") == 1):
        failed.append("HR-W (the withdrawal's destructuring inside `confirmed_state.is_some() && confirmed.is_none()`)")
    # HR-ALL: every Held( in lib.rs is one of the three.
    if len(re.findall(r"\bHeld\(", lib)) != 3:
        failed.append("HR-ALL (lib.rs names Held( only at HR-C, HR-L and HR-W)")
    # WE-2: confirmed_state comes only from the loop's Confirmed standing; the loop returns None first without
    # nonlinear supports; D1.6 refuses every nonlinear support.
    if not (case.count(CONFIRMED_FROM_STANDING) == 1 and not re.search(r"\bconfirmed_state\s*=[^=]", case)
            and len(re.findall(r"\blet\s*\(?\s*confirmed_state\b", case)) == 1
            and case.count(STANDING_FROM_LOOP) == 1 and case.count(STANDING_ARMS) == 1
            and not re.search(r"\bnonlinear_standing\s*=[^=]", case)
            and case.find(STANDING_FROM_LOOP) < case.find(CONFIRMED_FROM_STANDING) < case.find(WITHDRAWAL)
            and loop.find("{\n" + LOOP_STEP) == loop.find("{\n")
            and _nc(family_clauses_text).count(D16) == 1):
        failed.append("WE-2 (confirmed_state needs a nonlinear support, which D1.6 refuses)")
    return failed
