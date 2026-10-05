"""RV94's own eligibility oracle (stdlib only; scratch). Written from the contract, not from
any implementer's oracle or test:

- C1 WIRE_CONTRACT.md:160: "Numerically eligible requires all applicable G checks with
  invocation, MECHANICS_SOLVED, requested load-case refs/order and each case selected or
  ordinarily eligible not_required ... An unavailable case, missing invocation or scope yields
  needs_recompute; malformed evidence yields unsupported."
- C1:162: transport is never eligible.
- D2 DESIGN.md 4.9.4 table: numerically_eligible / needs_recompute / unsupported.
- D-U6-1: the reader's own `numerical_eligible` is the flag AND an invocation AND
  MECHANICS_SOLVED AND every case selected or not_required (the reader does not see the
  caller's requested refs); the carriers add requested refs and the not_required ordinary
  eligibility (D2 4.9.4 conjuncts).
- D2 4.9.9: withheld = abs+not_covered when the standing (with the invocation's requested
  cases) is numerically_eligible, else every quantity row (rel+abs+nc+input_derived).
- D-U7-4: TS additionally requires the live native capture for these bytes and the model.

The G outcome (R0) is an input: the record's shared expectation ("pass"/"fail"), or, for
RV94's hostile not_required variants ("observe"), the U7 base's own reader outcome (the flag
does not reach any gate, D-U6-1; gate identity base==candidate is checked separately).
"""


def _ids(source):
    ids = set()
    for key in ("results", "diagnostics"):
        items = source.get(key)
        if not isinstance(items, list):
            return None
        for item in items:
            i = item.get("id") if isinstance(item, dict) else None
            if not isinstance(i, str) or not i or i in ids:
                return None
            ids.add(i)
    return ids


def ordinarily_eligible_not_required(source, index, case):
    """D2 4.9.4: the base identity's ordinary eligibility for one not_required case."""
    q = (source.get("numerical_quality") or {}).get("cases")
    if not isinstance(q, list) or index >= len(q) or not isinstance(q[index], dict):
        return False
    q = q[index]
    ids = _ids(source)
    if ids is None:
        return False
    refs = q.get("evidence_refs")
    return (q.get("basis_ref") == case.get("basis_ref")
            and q.get("solve_quality") == "checks_passed"
            and q.get("structural_status") == "passive_model_basis"
            and q.get("model_matrix_fidelity") == "represented_equations_retained"
            and q.get("accuracy_evidence") in ("not_claimed", "reference_verified")
            and isinstance(refs, list) and len(refs) > 0
            and all(isinstance(r, str) and r in ids for r in refs))


def receipt_cases(source):
    try:
        cases = source["retained_precision"]["body"]["cases"]
        return cases if isinstance(cases, list) else []
    except (KeyError, TypeError):
        return []


def reader_eligible(flag, g_pass, source, invocation):
    if not (flag and g_pass and invocation is not None):
        return False
    if (source.get("status") or {}).get("mechanics") != "MECHANICS_SOLVED":
        return False
    return all(c.get("status") in ("selected", "not_required") for c in receipt_cases(source))


def token(flag, g_pass, source, invocation, requested):
    if not g_pass:
        return "unsupported"
    if not reader_eligible(flag, g_pass, source, invocation):
        return "needs_recompute"
    cases = receipt_cases(source)
    if [c.get("basis_ref") for c in cases] != list(requested):
        return "needs_recompute"
    for i, c in enumerate(cases):
        if c.get("status") == "not_required" and not ordinarily_eligible_not_required(source, i, c):
            return "needs_recompute"
    return "numerically_eligible"


def invocation_refs(invocation):
    if not isinstance(invocation, dict):
        return []
    cases = ((invocation.get("request") or {}).get("model") or {}).get("load_cases")
    return [{"ref_type": "load_case", "ref_id": c.get("id")} for c in cases] if isinstance(cases, list) else []


def expect(flag, g_pass, source, invocation, requested):
    """PY/RS expectations for one record."""
    e = reader_eligible(flag, g_pass, source, invocation)
    return {
        "reader_ok": g_pass,
        "invocation_bound": (invocation is not None) if g_pass else None,
        "numerical_eligible": e if g_pass else None,
        "standing_label": ("eligible" if e else "needs_recompute") if g_pass else None,
        "token": token(flag, g_pass, source, invocation, requested),
        # classification_summary(source, invocation): Current iff the token with the
        # invocation's own requested cases is numerically_eligible (D2 4.9.9).
        "summary_current": token(flag, g_pass, source, invocation, invocation_refs(invocation)) == "numerically_eligible" if g_pass else None,
    }


def ts_seam_expect(flag, g_pass, source, invocation, requested, live):
    """TS standing for a registration of these bytes with this invocation, read with a model whose
    load cases are `requested` (TS requested refs are always load_case typed), where `live` says
    whether the live native capture holds for these bytes and that model (D-U7-4)."""
    if not g_pass:
        return {"standing": "unsupported", "eligible": False, "status": "needs_recompute", "summary": "empty"}
    if token(flag, g_pass, source, invocation, requested) != "numerically_eligible":
        return {"standing": "needs_recompute", "eligible": False, "status": "needs_recompute",
                "finding": "RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE", "summary": "not_current"}
    if not live:
        return {"standing": "needs_recompute", "eligible": False, "status": "needs_recompute",
                "finding": "RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED", "summary": "not_current"}
    return {"standing": "numerically_eligible", "eligible": True, "status": "integrity_checked", "summary": "current"}
