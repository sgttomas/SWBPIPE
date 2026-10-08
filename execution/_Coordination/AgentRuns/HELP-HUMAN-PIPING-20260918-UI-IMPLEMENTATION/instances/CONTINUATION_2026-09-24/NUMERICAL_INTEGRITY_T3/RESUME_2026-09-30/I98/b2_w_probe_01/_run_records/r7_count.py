"""I98 B2-W, item R-7 (RV115 ADDENDUM_02 NB-2): count the combination rows the ordinary route
publishes on in-domain (W1a) requests, and check 7n + 50m + 8g with n, m and g taken from
distinct entity references (read-only; VENV).

Usage: python r7_count.py <input dir> <plain dir> <label>...
For each label and mode it reads <input dir>/<label>.json (the request) and
<plain dir>/plain_<label>_<mode>.json (the plain envelope the probe wrote).
"""
import collections
import hashlib
import json
import os
import sys

MODES = ["sparse_interactive", "dense_scrutiny"]
DISP = {f"global_nodal_{q}_{a}" for q in ("displacement", "rotation") for a in "xyz"}


def family(kind):
    if kind in DISP:
        return "displacement components"
    if kind == "displacement_magnitude":
        return "displacement magnitudes"
    if kind.startswith("element_local_") and "_stress" in kind:
        return "stress"
    if kind.startswith("element_local_") and ("_force" in kind or "_moment" in kind):
        return "action"
    if kind == "support_reaction_component_v2":
        return "support components"
    if kind in ("support_reaction_force_magnitude_v2", "support_reaction_moment_magnitude_v2"):
        return "support magnitudes"
    return "OTHER:" + kind


def main():
    inputs, plains, labels = sys.argv[1], sys.argv[2], sys.argv[3:]
    bad = 0
    for label in labels:
        with open(os.path.join(inputs, label + ".json"), "rb") as f:
            request = json.loads(f.read())
        model = request["model"]
        combos = [c["id"] for c in model["combinations"]]
        print(f"== {label}: model nodes={len(model['nodes'])} members={len(model['pipe_segments'])} supports={len(model['supports'])} "
              f"cases={[c['id'] for c in model['load_cases']]} combinations={combos}")
        for mode in MODES:
            path = os.path.join(plains, f"plain_{label}_{mode}.json")
            with open(path, "rb") as f:
                raw = f.read()
            env = json.loads(raw)
            rows = env["results"]
            basis = [(r.get("basis_ref") or {}).get("ref_type") for r in rows]
            for cid in combos:
                comb = [r for r in rows if (r.get("basis_ref") or {}) == {"ref_type": "combination", "ref_id": cid}]
                idx = [i for i, r in enumerate(rows) if (r.get("basis_ref") or {}).get("ref_id") == cid]
                contiguous_after_cases = bool(idx) and idx == list(range(idx[0], idx[0] + len(idx))) and all(b == "load_case" for b in basis[:idx[0]])
                fam = collections.Counter(family(r["kind"]) for r in comb)
                kinds = collections.Counter(r["kind"] for r in comb)
                ents = collections.defaultdict(set)
                for r in comb:
                    ents[family(r["kind"])].add(r["entity_ref"])
                n_d = ents["displacement components"] | ents["displacement magnitudes"]
                m_d = ents["action"] | ents["stress"]
                g_d = ents["support components"] | ents["support magnitudes"]
                n, m, g = len(n_d), len(m_d), len(g_d)
                expected = 7 * n + 50 * m + 8 * g
                ids_unique = len({r["id"] for r in comb}) == len(comb)
                keys = [(r["kind"], r["entity_ref"], (r.get("metadata") or {}).get("location"), (r.get("metadata") or {}).get("component")) for r in comb]
                keys_unique = len(set(keys)) == len(keys)
                sites = collections.Counter((r["entity_ref"], (r.get("metadata") or {}).get("location")) for r in comb if family(r["kind"]) in ("action", "stress"))
                site_names = sorted({s for (_, s) in sites})
                per_site = sorted(set(sites.values()))
                per_member = collections.Counter(r["entity_ref"] for r in comb if family(r["kind"]) in ("action", "stress"))
                per_support = collections.Counter(r["entity_ref"] for r in comb if family(r["kind"]) == "support components")
                checks = {
                    "count == 7n+50m+8g": len(comb) == expected,
                    "displacement components == 6n": fam["displacement components"] == 6 * n,
                    "displacement magnitudes == n": fam["displacement magnitudes"] == n,
                    "action == 30m": fam["action"] == 30 * m,
                    "stress == 20m": fam["stress"] == 20 * m,
                    "support components == 6g": fam["support components"] == 6 * g,
                    "support magnitudes == 2g": fam["support magnitudes"] == 2 * g,
                    "no other family": not any(k.startswith("OTHER") for k in fam),
                    "ids unique": ids_unique,
                    "(kind, entity, location, component) unique": keys_unique,
                    "10 rows per member site": per_site == [10],
                    "contiguous after the case rows": contiguous_after_cases,
                }
                bad += sum(not v for v in checks.values())
                case_rows = collections.Counter((r.get("basis_ref") or {}).get("ref_id") for r in rows if (r.get("basis_ref") or {}).get("ref_type") == "load_case")
                print(f"{label} {mode} plain_sha={hashlib.sha256(raw).hexdigest()} rows_total={len(rows)} case_rows={dict(case_rows)} combination={cid} "
                      f"combination_rows={len(comb)} n={n} m={m} g={g} 7n+50m+8g={expected}")
                print(f"  families={dict(sorted(fam.items()))}")
                print(f"  kinds={dict(sorted(kinds.items()))}")
                print(f"  member sites={site_names} rows per member={sorted(set(per_member.values()))} component rows per support={sorted(set(per_support.values()))}")
                print(f"  nodes={sorted(n_d)} members={sorted(m_d)} supports={sorted(g_d)}")
                unreferenced_nodes = sorted({x['id'] for x in model['nodes']} - n_d)
                unreferenced_supports = sorted({x['id'] for x in model['supports']} - g_d)
                print(f"  model nodes without combination rows={unreferenced_nodes} model supports without combination rows={unreferenced_supports}")
                print("  checks: " + ", ".join(f"{k}={'ok' if v else 'FAIL'}" for k, v in checks.items()))
    print(f"R7_SUMMARY failed_checks={bad}")


if __name__ == "__main__":
    main()
