"""§8.43 MODAL-SEAM's standing tuple and §8.49 RTA3NL's reader, bound by exact text where the DEL-04-07
publication-basis decision changed them. Both premises are D1.6's: D1 refuses every nonlinear support.

SEAM-ST (MODAL-SEAM, solve_prepared_load_case). `append_nonlinear_support_loop_results` returns None at its first
statement when `built.nonlinear_supports` is empty, which D1.6 makes it, so the case takes the match's None arm. The
current source returns a fourth member, `nonlinear_staged: Option<StagedConfirmedRows>` (the rows on the
confirmation's basis, staged for the publication-basis decision to release). On D1 the None arm sets it to None; the
arm builds four `None`s and allocates nothing. Its only reads are `released = confirmed.is_some() &&
nonlinear_staged.is_some()` (false), the test-only probe, and `if let Some(staged) = nonlinear_staged.filter(|_|
released)`, which alone calls `release_confirmed_rows` and is not entered. It is never re-assigned. The earlier
three-member form (no staged rows) is accepted unchanged, so earlier sources price as before; any other form stops.

NL-RD (RTA3NL, nonlinear_support_evidence.rs). `check` reads `read`, which maps `read_case` over `case_ids`: on D1
there is no `nonlinear_support_*` row, so the case list is empty. The current `read` binds the list as `cases` and
first runs `require(cases.is_empty() || <mechanics solved>, "ROWS_IN_UNSOLVED_DOCUMENT")`; `read_case` gains
`require(code != INSPECTION_ONLY_CODE, "INSPECTION_ONLY_IN_SOLVED_DOCUMENT")` per case. `require`'s signature is
`fn require(ok: bool, code: &str) -> Check`: the message is a static `&str` passed by reference, and its only
allocation, `Err(format!("SOURCE_PHYSICS_NONLINEAR_{code}"))`, is in the `else` branch, when `ok` is false. On D1
`cases.is_empty()` is true, so `||` short-circuits, `ok` holds and nothing is allocated at the call; `read_case`'s
require is inside the map over the empty list and unreached. The earlier `read` (no require) is accepted unchanged.
NL-RQ binds `require`'s exact signature and body, and each call's message as a string literal.

Each function returns the names of the checks that fail; an empty list means all hold.
"""
import re

OLD_LET = ("let (selected_nonlinear, nonlinear_standing, nonlinear_qualification) =\n"
           "        match append_nonlinear_support_loop_results(")
OLD_ARMS = ("Some((solve, nonlinear, qualification)) => {\n"
            "                (Some(solve), Some(nonlinear), qualification)\n"
            "            }\n"
            "            None => (None, None, None),")
NEW_LET = ("let (selected_nonlinear, nonlinear_standing, nonlinear_qualification, nonlinear_staged) =\n"
           "        match append_nonlinear_support_loop_results(")
NEW_ARMS = ("Some((solve, nonlinear, qualification, staged)) => {\n"
            "                (Some(solve), Some(nonlinear), qualification, staged)\n"
            "            }\n"
            "            None => (None, None, None, None),")
STAGED_READS = [
    "    let released = confirmed.is_some() && nonlinear_staged.is_some();\n",
    "                published: nonlinear_staged.as_ref().filter(|_| released).map_or_else(",
    "    if let Some(staged) = nonlinear_staged.filter(|_| released) {\n        release_confirmed_rows(&mut results, staged);\n    }\n",
]

OLD_READ = ("    let rows = items(&source[\"results\"]);\n"
            "    case_ids(rows)?.into_iter().map(|case| read_case(source, case)).collect()\n")
NEW_READ_HEAD = ("    let rows = items(&source[\"results\"]);\n"
                 "    let cases = case_ids(rows)?;\n")
NEW_READ_TAIL = ("    require(cases.is_empty() || source[\"status\"][\"mechanics\"] == \"MECHANICS_SOLVED\", "
                 "\"ROWS_IN_UNSOLVED_DOCUMENT\")?;\n"
                 "    cases.into_iter().map(|case| read_case(source, case)).collect()\n"
                 "}\n")
READ_CASE_REQUIRE = ("    require(code != crate::semantic_contract::INSPECTION_ONLY_CODE, "
                     "\"INSPECTION_ONLY_IN_SOLVED_DOCUMENT\")?;\n")
REQUIRE = ("fn require(ok: bool, code: &str) -> Check {\n"
           "    if ok {\n"
           "        Ok(())\n"
           "    } else {\n"
           "        Err(format!(\"SOURCE_PHYSICS_NONLINEAR_{code}\"))\n"
           "    }\n"
           "}\n")
OLD_LOOP, NEW_LOOP = "iter:case_ids(rows)?.into_iter().map", "iter:cases.into_iter().map"


def _call_args(text, name):
    """The top-level arguments of every call `name(..)` in text (not its `fn name` definition), each stripped, with a
    trailing empty argument (a trailing comma) dropped. Parentheses inside string literals are skipped."""
    out = []
    for m in re.finditer(r"(?<!fn )\b" + re.escape(name) + r"\(", text):
        i, depth, args, cur, in_str = m.end(), 1, [], [], False
        while i < len(text) and depth:
            c = text[i]
            if in_str:
                if c == "\\":
                    cur.append(text[i:i + 2]); i += 2; continue
                in_str = c != '"'
            elif c == '"':
                in_str = True
            elif c in "([{":
                depth += 1
            elif c in ")]}":
                depth -= 1
                if not depth:
                    break
            elif c == "," and depth == 1:
                args.append("".join(cur).strip()); cur = []; i += 1; continue
            cur.append(c); i += 1
        args.append("".join(cur).strip())
        out.append([a for a in args if a] if args and not args[-1] else args)
    return out


def standing_failures(case_text):
    """SEAM-ST on solve_prepared_load_case's text."""
    old = case_text.count(OLD_LET) == 1 and case_text.count(OLD_ARMS) == 1 and "nonlinear_staged" not in case_text
    new = (case_text.count(NEW_LET) == 1 and case_text.count(NEW_ARMS) == 1
           and not re.search(r"\bnonlinear_staged\s*=[^=]", case_text)
           and all(case_text.count(r) == 1 for r in STAGED_READS)
           and len(re.findall(r"\bnonlinear_staged\b", case_text)) == 1 + len(STAGED_READS)
           and len(re.findall(r"\brelease_confirmed_rows\(", case_text)) == 1
           and case_text.find(NEW_LET) < case_text.find(STAGED_READS[0]) < case_text.find(STAGED_READS[2]))
    return [] if old or new else ["SEAM-ST (the nonlinear loop's standing tuple and its staged rows)"]


def reader_failures(production_text, read_text):
    """NL-RD and NL-RQ on nonlinear_support_evidence.rs's production text and `read`'s text. Returns
    (failures, loop header of read's case map)."""
    failed = []
    old = production_text.count(OLD_READ) == 1 and read_text.count(OLD_READ) == 1 and "require(" not in read_text
    new = (read_text.count(NEW_READ_HEAD) == 1 and read_text.count(NEW_READ_TAIL) == 1
           and read_text.find(NEW_READ_HEAD) < read_text.find(NEW_READ_TAIL)
           and len(re.findall(r"\bcases\b", read_text)) == 3 and not re.search(r"\bcases\s*=[^=]", read_text.replace("let cases =", ""))
           and len(re.findall(r"\brequire\(", read_text)) == 1
           and production_text.count(READ_CASE_REQUIRE) == 1)
    if not (old or new):
        failed.append("NL-RD (read maps read_case over case_ids, after one require on the empty-case short circuit)")
    # NL-RQ: require takes a static &str and formats only on failure; every call passes a string literal message.
    calls = _call_args(production_text, "require")
    if not (production_text.count(REQUIRE) == 1 and len(re.findall(r"\bfn require\b", production_text)) == 1
            and calls and all(len(a) == 2 and re.fullmatch(r'"[A-Z0-9_]+"', a[1]) for a in calls)):
        failed.append("NL-RQ (require(ok: bool, code: &str) formats only when ok is false; each message a literal)")
    return failed, (NEW_LOOP if new else OLD_LOOP)
