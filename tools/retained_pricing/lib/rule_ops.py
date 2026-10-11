"""Two steps of sq2_rules.py, as functions so they can be tested alone (stdlib only).

carry_decision_keys: every line key in the decision tables (written at SQ2's basis) is carried to the target by a
linemap_lib.Mapper; keys resolved by name at the snapshot (`Resolved`) are left as resolved. A key that cannot be carried
stops the run unless KEYMAP_ALLOW lists it as {key: ("deleted", why)} and the map reported it "deleted" (the rule then
names nothing at the target). A "replaced" or "fewer_definitions" key is never allowed; the latter is resolved first in
rules/linemap_resolutions.json.

remove_caller_zero_edges: EDGE_ZERO_REMOVE_CALLER removes every zero edge of a caller; the count written at the basis
must match, reduced by exactly the caller's edges the line map's prune dropped.
"""
ALLOWABLE = ("deleted",)
DESCR = ("why", "evidence", "about", "reason")


def carry_decision_keys(D, M):
    """Carry D's upper-case tables through M in place. Returns the unmapped entries KEYMAP_ALLOW does not cover."""
    def km(v, ctx):
        if type(v).__name__ == "Resolved":
            return str(v)
        if isinstance(v, str):
            return M.map_key(v, ctx)
        if isinstance(v, tuple):
            return tuple(km(x, ctx) for x in v)
        if isinstance(v, list):
            return [km(x, f"{ctx}[{i}]") for i, x in enumerate(v)]
        if isinstance(v, dict):
            return {(km(k, ctx + "/key") if isinstance(k, str) else k): (x if k in DESCR else km(x, ctx + "/" + str(k)[:30])) for k, x in v.items()}
        return v
    for k in [k for k in D if k.isupper() and k != "KEYMAP_ALLOW"]:
        D[k] = km(D[k], k)
    allow = dict(D.get("KEYMAP_ALLOW", {}))
    for k, v in allow.items():
        if not (isinstance(v, tuple) and len(v) == 2 and v[0] in ALLOWABLE and v[1]):
            raise SystemExit(f"KEYMAP_ALLOW[{k!r}] must be (kind, why) with kind in {ALLOWABLE}")
    return [u for u in M.unmapped if u[1] not in allow or allow[u[1]][0] != u[-1]]


def remove_caller_zero_edges(edge_zero, entries, pruned):
    """Remove each listed caller's zero edges from edge_zero (in place). entries: (caller, n at the basis, why);
    pruned: the line map's pruned_deleted. Returns log rows; stops when a count does not match."""
    log = []
    for caller, n_expected, why in entries:
        hits = [e for e in edge_zero if e["caller"] == caller]
        gone = [p for p in pruned if p["family"] == "edge_zero" and isinstance(p["rule"], dict) and p["rule"].get("caller") == caller]
        if gone:
            log.append(["edge_zero_remove_caller: pruned at the line map", caller, [g["rule"]["callee"] for g in gone]])
        if len(hits) != n_expected - len(gone):
            raise SystemExit(f"EDGE_ZERO_REMOVE_CALLER {caller}: {len(hits)} != {n_expected} - {len(gone)} pruned")
        for e in hits:
            edge_zero.remove(e)
        log.append(["edge_zero removed (caller)", caller, sorted(e["callee"] for e in hits), why])
    return log
