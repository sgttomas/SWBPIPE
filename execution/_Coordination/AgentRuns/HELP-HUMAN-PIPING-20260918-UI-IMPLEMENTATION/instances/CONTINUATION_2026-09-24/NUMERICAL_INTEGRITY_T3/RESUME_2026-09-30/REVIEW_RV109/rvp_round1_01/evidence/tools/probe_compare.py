#!/usr/bin/env python3
"""RV109: compare the probe's base and candidate lines (byte differential, item 4) and print
the head's outcomes for the item-3 inputs. Also cross-checks base against I81's recorded
probe lines (control).

Usage: probe_compare.py <out_base.jsonl> <out_cand.jsonl> [<I81 probe_run2.log>]
"""
import json
import re
import sys

ITEM3 = ["case_c", "two_body_b", "w6_phys_r4", "w2b_input", "file:b2_k1e3.json"]


def load(path):
    rows = {}
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            row = json.loads(line)
            rows[(row["input"], row["mode"])] = row
    return rows


def pub(row):
    d = row.get("direct", {})
    return (d.get("pub_sha"), d.get("pub_len"))


def main():
    base, cand = load(sys.argv[1]), load(sys.argv[2])
    keys = sorted(set(base) | set(cand))
    print(f"pairs: base {len(base)} cand {len(cand)} union {len(keys)}")
    same_plain = same_pub = same_driver_env = same_successor = 0
    changed = []
    for k in keys:
        b, c = base.get(k), cand.get(k)
        if b is None or c is None:
            changed.append((k, "missing on one side"))
            continue
        assert b["input_sha"] == c["input_sha"], k
        if b.get("plain_sha") == c.get("plain_sha"):
            same_plain += 1
        else:
            changed.append((k, f"PLAIN differs {b.get('plain_sha')} {c.get('plain_sha')}"))
        if pub(b) == pub(c):
            same_pub += 1
        else:
            changed.append((k, f"Direct published differs: base cause={b['direct'].get('cause')} notices={b['direct'].get('notices')} "
                               f"eq_plain+notice={b['direct'].get('pub_eq_plain_plus_notice')} -> cand cause={c['direct'].get('cause')} "
                               f"notices={c['direct'].get('notices')} eq_plain={c['direct'].get('pub_eq_plain')}"))
        bd, cd = b.get("driver", {}), c.get("driver", {})
        if bd.get("env_sha") == cd.get("env_sha"):
            same_driver_env += 1
        else:
            changed.append((k, f"driver envelope differs: base cause={bd.get('cause')} -> cand cause={cd.get('cause')} "
                               f"cand env_eq_plain={cd.get('env_eq_plain')}"))
        if bd.get("successor_sha") == cd.get("successor_sha"):
            same_successor += 1
        else:
            changed.append((k, f"driver successor differs {bd.get('successor_sha')} {cd.get('successor_sha')}"))
        for side, row in (("base", b), ("cand", c)):
            d = row.get("direct", {})
            if d.get("successor") and not d.get("envelope_eq_plain"):
                changed.append((k, f"{side}: successor published but the envelope beside it is not plain"))
    print(f"plain identical: {same_plain}; Direct published identical: {same_pub}; driver envelope identical: {same_driver_env}; "
          f"driver successor identical: {same_successor}")
    print("DIFFERENCES (base -> cand):")
    for k, why in changed:
        print(f"  {k[0]} [{k[1]}]: {why}")
    print("SUCCESSORS (Direct, identical on both sides unless listed above):")
    for k in keys:
        c = cand.get(k)
        if c and c.get("direct", {}).get("successor"):
            print(f"  {k[0]} [{k[1]}] pub_sha={c['direct']['pub_sha']} receipt={c['direct']['receipt_sha']}")
    print("ITEM 3 (candidate):")
    for name in ITEM3:
        for mode in ("sparse_interactive", "dense_scrutiny"):
            c = cand.get((name, mode))
            if not c:
                print(f"  {name} [{mode}]: not run")
                continue
            d, dr = c["direct"], c["driver"]
            s = c.get("driver_sentinel", {})
            print(f"  {name} [{mode}] input={c['input_sha'][:12]} verdicts={dr.get('verdicts')} seeds={dr.get('seeds')}\n"
                  f"     Direct: cause={d.get('cause')} phase={d.get('phase')} notices={d.get('notices')} runs/gc={d.get('runs')}/{d.get('complete_gates')} "
                  f"refusal={d.get('admission', {}).get('refusal')} eq_plain={d.get('pub_eq_plain')} eq_plain+notice={d.get('pub_eq_plain_plus_notice')} "
                  f"armed={d.get('armed_before')}/{d.get('armed_after')}\n"
                  f"     driver(4 MiB): cause={dr.get('cause')} phase={dr.get('phase')} notices={dr.get('notices')} reserved_slot={dr.get('reserved_slot')} "
                  f"env_eq_plain={dr.get('env_eq_plain')} env_eq_plain+notice={dr.get('env_eq_plain_plus_notice')}; sentinel native_reached={s.get('native_reached')}\n"
                  f"     seam (permitted observer): {c.get('permitted_seam')}")
    print("NO-TRIGGER CHECK (candidate rows with cause NoTriggeredCase):")
    for k in keys:
        c = cand.get(k)
        if not c:
            continue
        d, dr, s = c["direct"], c["driver"], c.get("driver_sentinel", {})
        if d.get("cause") == "NoTriggeredCase" or dr.get("cause") == "NoTriggeredCase":
            ok = (d.get("cause") in ("NoTriggeredCase", "None")) and d.get("pub_eq_plain") and d.get("notices") == 0 \
                and dr.get("cause") == "NoTriggeredCase" and dr.get("env_eq_plain") and dr.get("reserved_slot") is False \
                and s.get("native_reached") is False
            print(f"  {k[0]} [{k[1]}]: direct={d.get('cause')} eq_plain={d.get('pub_eq_plain')} notices={d.get('notices')} "
                  f"driver={dr.get('cause')} env_eq_plain={dr.get('env_eq_plain')} reserved_slot={dr.get('reserved_slot')} "
                  f"native_reached={s.get('native_reached')} -> {'OK' if ok else 'CHECK'}")
    print("SEAM (candidate; late_loads_total against the request's load count, admitted inputs):")
    for k in keys:
        c = cand.get(k)
        seam = (c or {}).get("permitted_seam") or {}
        if seam.get("admitted"):
            print(f"  {k[0]} [{k[1]}]: late_loads_total={seam.get('late_loads_total')} loads={seam.get('loads')} late_refused={seam.get('late_refused')}"
                  f" -> {'OK' if seam.get('late_loads_total') == seam.get('loads') else 'CHECK'}")
    if len(sys.argv) > 3:
        print("CONTROL against I81's probe_run2.log (base):")
        names = {
            "u8 first_load_only": "first_load_only", "u8 two_body_case_a": "two_body_a", "u8 two_body_case_b (W-C1)": "two_body_b",
            "u8 l0_isolated_node": "l0_isolated_node", "W2 cap_maximal": "w2_cap_maximal", "W2-deep": "w2_deep",
            "W2b cap_maximal_solvable": "w2b_input", "W6 force_scaled": "w6_phys_r4", "case_c": "case_c",
            "attempted: milestone": "attempted:milestone", "attempted: failed attempt": "attempted:failed attempt",
            "attempted: deferred formation (K2a partial underflow)": "attempted:deferred formation (K2a partial underflow)",
            "attempted: rejected_stress_range sparse": "attempted:rejected_stress_range sparse",
            "attempted: rejected_stress_range dense": "attempted:rejected_stress_range dense",
        }
        pat = re.compile(r"^I81_(BEGIN|ORDINARY|W1) (.*?) (sparse_interactive|dense_scrutiny) (.*)$")
        seen = {}
        with open(sys.argv[3], encoding="utf-8", errors="replace") as fh:
            for line in fh:
                m = pat.match(line.rstrip("\n"))
                if not m:
                    continue
                kind, label, mode, rest = m.groups()
                label = label.strip()
                if label not in names:
                    continue
                key = (names[label], mode)
                rec = seen.setdefault(key, {})
                for field in ("input_sha", "plain_sha", "published_sha", "notices"):
                    mm = re.search(field + r"=([0-9a-f]+|\d+)", rest)
                    if mm:
                        rec[field] = mm.group(1)
                mm = re.search(r"cause=(\S+)", rest)
                if mm and kind == "W1":
                    rec["cause"] = mm.group(1)
        agree = 0
        for key, rec in sorted(seen.items()):
            b = base.get(key)
            if not b:
                print(f"  {key}: not in my base run")
                continue
            mine = {"input_sha": b["input_sha"], "plain_sha": b.get("plain_sha"), "published_sha": b["direct"].get("pub_sha"),
                    "notices": str(b["direct"].get("notices"))}
            diffs = [f for f in ("input_sha", "plain_sha", "published_sha", "notices") if f in rec and rec[f] != mine[f]]
            cause = rec.get("cause", "")
            mine_cause = b["direct"].get("cause")
            cause_ok = (cause == "Ok(successor)" and mine_cause == "Successor") or (
                cause.startswith("Err(") and re.split(r"[ ({)]", cause[4:])[0] == re.split(r"[ ({)]", str(mine_cause))[0])
            if not diffs and cause_ok:
                agree += 1
            else:
                print(f"  {key}: DIFF {diffs} I81 cause={cause} mine={mine_cause}")
        print(f"  agree: {agree} of {len(seen)} I81 (input, mode) pairs (input, plain, published sha, notices, cause)")


if __name__ == "__main__":
    main()
