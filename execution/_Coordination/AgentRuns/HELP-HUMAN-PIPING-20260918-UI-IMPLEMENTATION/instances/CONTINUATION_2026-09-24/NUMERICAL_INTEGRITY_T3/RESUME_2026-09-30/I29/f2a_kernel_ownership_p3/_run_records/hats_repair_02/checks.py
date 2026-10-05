"""Narrow RV52-1 source/ownership repair control; no production execution."""
from pathlib import Path
import hashlib
import json

HERE = Path(__file__).resolve().parent
PACK = HERE.parent.parent
NUM = next(p for p in HERE.parents if (p / "projects/chirality-piping/core").is_dir())

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    correction = json.loads((HERE / "CORRECTION.json").read_text())
    checks = []
    for entry in correction["source_origins"]:
        assert digest(NUM / entry["path"]) == entry["sha256"]
    checks.append({"name": "source_matches_unchanged_fdae294", "pass": True, "files": 3})
    for entry in correction["preserved_original_raw"] + correction["preserved_review"]:
        path = Path(entry["path"])
        assert digest(path) == entry["sha256"] and len(path.read_bytes()) == entry["bytes"]
    checks.append({"name": "original_raw_and_review_preserved", "pass": True,
                   "files": len(correction["preserved_original_raw"])+len(correction["preserved_review"])})
    root = NUM / "projects/chirality-piping/core/solver/frame_kernel/src/structural/retained"
    adaptive = (root / "adaptive.rs").read_text()
    verify = (root / "verify.rs").read_text()
    bound = (root / "bound.rs").read_text()
    rule = adaptive[adaptive.index("fn rule<"):adaptive.index("// ------------------------------------------------------------ states by precision")]
    assert "let hats: Vec<[f64; 2]> = match report" in rule
    assert ".map(|(e, &x)| e_hat(*e, x))" in rule
    assert "let mut phis = Vec::with_capacity(hats.len());" in rule
    assert "floor = Some(phis);" in rule
    assert rule.index("let hats:") < rule.index("let mut phis") < rule.index("let mut trackers:")
    assert rule.index("hat(meta.body, meta.kind)") < rule.index("for ((test, body, kind), tracker)")
    assert "resolution_hats(&resolution, &prep.extents)?;" in verify
    assert "let c = u_pass(" in bound and "let ct = nl_pass(" in bound
    assert "let u = block_max(&c," in bound and "let n_l = block_max(&ct," in bound
    checks.append({"name": "distinct_construction_and_lifetime_source_anchors", "pass": True,
                   "hats": "adaptive.rs:2284–2490", "earlier_drop": "verify.rs:794",
                   "uc": "bound.rs:386,426,493,517,533,691–694"})

    # An independent event ledger: names identify actual allocations, not values.
    # All weights are intentionally synthetic units, not Rust byte coefficients.
    live = {}
    def allocate(token, units):
        assert token not in live and type(units) is int and units >= 0
        live[token] = units
    def release(token):
        del live[token]
    allocate("report_resolution", 9)
    allocate("earlier_resolution_check", 5)
    assert set(live) == {"report_resolution", "earlier_resolution_check"}
    release("earlier_resolution_check")  # standalone verify statement completed
    assert set(live) == {"report_resolution"}
    allocate("skip", 2)
    allocate("coupled_scales", 3)
    allocate("rule_hats_old", 6)
    allocate("rule_hats", 13)
    # Only hats growth has this old backing; no floor exists yet.
    assert set(live) == {"report_resolution","skip","coupled_scales","rule_hats_old","rule_hats"}
    assert sum(live.values()) == 33
    requested_units = sum(v for k,v in live.items() if k != "rule_hats_old")
    assert requested_units == 27
    release("rule_hats_old")
    allocate("optional_floor", 17)
    allocate("trackers", 4)
    allocate("summary", 8)
    assert {"report_resolution","rule_hats","optional_floor"} <= live.keys()
    assert "earlier_resolution_check" not in live and "rule_hats_old" not in live
    assert sum(live[k] for k in ("report_resolution","rule_hats","optional_floor")) == 39
    assert sum(live.values()) == 56
    checks.append({"name": "distinct_report_hats_floor_after_earlier_check_drop", "pass": True,
                   "simultaneous_tokens": ["report_resolution","rule_hats","optional_floor"],
                   "synthetic_live_units": 56})
    checks.append({"name": "hats_moving_collect_old_plus_new", "pass": True,
                   "synthetic_requested_units": requested_units, "synthetic_moving_units": 33})
    # A later summary resize occurs while hats is still a live ordinary owner.
    allocate("summary_old_backing", 7)
    assert "rule_hats" in live and "rule_hats_old" not in live
    assert sum(live.values()) == 63
    release("summary_old_backing")
    checks.append({"name": "hats_overlaps_other_buffer_resize", "pass": True,
                   "synthetic_requested_units": 56, "synthetic_moving_units": 63})
    # Rule closure returns; floor/summary move into StopDecision, report stays.
    for token in ("skip","coupled_scales","rule_hats","trackers"):
        release(token)
    assert set(live) == {"report_resolution","optional_floor","summary"}
    checks.append({"name": "hats_drops_floor_summary_move_report_remains", "pass": True,
                   "remaining_tokens": sorted(live)})

    # RV52 domination is an injection of distinct actual owners into four coarse
    # same-type slots. It neither aliases a/c nor invents a fifth concurrent slot.
    slots = {"wide0","wide1","wide2","wide3"}
    mappings = {"u_pass": {"a":"wide0","c":"wide1"},
                "nl_pass": {"c":"wide0","at":"wide1","bt":"wide2","ct":"wide3"}}
    for phase, mapping in mappings.items():
        assert len(set(mapping.values())) == len(mapping)
        assert set(mapping.values()) <= slots
    assert len(mappings["u_pass"]) == 2 and len(mappings["nl_pass"]) == 4
    checks.append({"name": "uc_existing_four_buffer_domination", "pass": True,
                   "mapping": mappings, "premise": "maximum per-site Wide<L>(F) capacity law",
                   "block_max_owners": ["u(B)", "n_l(B)"], "extra_F_buffer_charge": False})

    output = {"status": "pass", "production_execution": False,
              "scope": "RV52-1 source anchors and abstract lifetime bookkeeping only",
              "numeric_weights": "synthetic units; no Rust profile or allowance",
              "checks": checks}
    (HERE / "checks.json").write_text(json.dumps(output, indent=2)+"\n")
    print(json.dumps({"status":"pass","checks":len(checks)}))

if __name__ == "__main__":
    main()

