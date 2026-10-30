"""Source/priority/copy guards for five immediate-return allocations."""
from copy import deepcopy
import json
from pathlib import Path
import runpy
import shutil
import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
MODEL = runpy.run_path(str(PRICING / "base/b7a_returning_errors.py"))
FORMED = runpy.run_path(str(PRICING / "base/b7a_formation_model.py"))
DOMAIN = runpy.run_path(str(PRICING / "base/b7a_domain_model.py"))


def fixture():
    table = {MODEL["MARKER"]: MODEL["VERSION"], "site_from": {}}
    edges = {}
    inventory = []
    for spec in MODEL["SITES"]:
        table["site_from"][spec["short"]] = {
            "from": [spec["from"]], "per": "1", "why": "SQ2 G5: " + MODEL["WHY"]}
        edges.setdefault(spec["owner"], [])
        for parent in spec["parents"]:
            children = edges.setdefault(parent, [])
            if spec["owner"] not in children:
                children.append(spec["owner"])
        inventory.append({k: spec[k] for k in ("file", "line", "kind")})
        inventory[-1]["fn"] = spec["owner"]
    return table, {"edges": edges}, inventory


def test_all_five_owned_allocations_preserve_full_counts_and_content():
    table, graph, inventory = fixture()
    active = MODEL["state"](SOURCE, table, graph, inventory, [])
    assert len(active["sites"]) == 5
    for spec, count, content in zip(MODEL["SITES"], (288, 288, 96, 27, 7), (8208, 8208, 8192, 27, 26)):
        counts = {spec["owner"]: count}
        MODEL["witness"](active, spec["file"], spec["line"], spec["kind"], count * 1536, counts)
        MODEL["witness"](active, spec["file"], spec["line"], spec["kind"], count, counts, rebound=True)
        assert 2 * count * content > 0
    assert sum(2 * n * size for n, size in zip((288, 288, 96, 27, 7), (8208, 8208, 8192, 27, 26))) == 11030302


@pytest.mark.parametrize("index", range(5))
@pytest.mark.parametrize("change", ("per", "owner", "alias_zero", "alias_size"))
def test_each_selected_rule_override_refuses(index, change):
    table, graph, inventory = fixture()
    spec = MODEL["SITES"][index]
    if change == "per":
        table["site_from"][spec["short"]]["per"] = "0"
    elif change == "owner":
        table["site_from"][spec["short"]]["from"] = ["other.rs:1:owner"]
    else:
        family = "site_zero" if change == "alias_zero" else "site_size"
        table[family] = {spec["file"] + ":" + str(spec["line"]) + ":" + spec["kind"]: {"bytes": 0}}
    with pytest.raises(ValueError, match="B7a owned first error"):
        MODEL["state"](SOURCE, table, graph, inventory, [])


@pytest.mark.parametrize("change", ("marker", "caller", "duplicate_edge", "missing_site", "duplicate_site", "kind", "owner"))
def test_exact_guard_graph_inventory_refuses(change):
    table, graph, inventory = fixture()
    spec = MODEL["SITES"][0]
    if change == "marker": del table[MODEL["MARKER"]]
    elif change == "caller": graph["edges"]["other.rs:1:caller"] = [spec["owner"]]
    elif change == "duplicate_edge": graph["edges"][spec["parents"][0]].append(spec["owner"])
    elif change == "missing_site": inventory.pop(0)
    elif change == "duplicate_site": inventory.append(deepcopy(inventory[0]))
    elif change == "kind": inventory[0]["kind"] = "clone_text"
    else: inventory[0]["fn"] = "other.rs:1:owner"
    with pytest.raises(ValueError, match="B7a owned first error"):
        MODEL["state"](SOURCE, table, graph, inventory, [])


@pytest.mark.parametrize("index", range(5))
def test_each_pre_and_post_witness_preserves_positive_calls(index):
    table, graph, inventory = fixture()
    active = MODEL["state"](SOURCE, table, graph, inventory, [])
    spec = MODEL["SITES"][index]
    with pytest.raises(ValueError, match="KD5C6"):
        MODEL["witness"](active, spec["file"], spec["line"], spec["kind"], 0, {spec["owner"]: 288})
    with pytest.raises(ValueError, match="KD5C6"):
        MODEL["witness"](active, spec["file"], spec["line"], spec["kind"], 287, {spec["owner"]: 288}, rebound=True)


@pytest.mark.parametrize("file,mutation", [
    ("core/product_physics/src/lib.rs", "question"),
    ("core/product_physics/src/lib.rs", "retry"),
    ("core/product_physics/src/lib.rs", "receiver"),
    ("core/product_physics/src/retained_product.rs", "return"),
    ("core/solver/frame_kernel/src/exact_sum.rs", "display"),
    ("core/solver/curved_bend/src/lib.rs", "crlf"),
])
def test_actual_return_type_and_source_mutations_refuse(tmp_path, file, mutation):
    files = list(DOMAIN["SOURCE_FILES"]) + list(FORMED["SOURCE_FILES"]) + list(MODEL["SOURCES"]) + ["core/product_physics/src/retained_memory.rs"]
    for name in set(files):
        target = tmp_path / name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / name, target)
    target = tmp_path / file
    raw = target.read_bytes()
    if mutation == "question":
        needle = b'.map_err(|e| format!("span intensity: {e}"))?;'
        assert raw.count(needle) == 2
        raw = raw.replace(needle, needle.replace(b"?;", b";"), 1)
    elif mutation == "retry":
        needle = b'.map_err(|e| format!("span intensity: {e}"))?;'
        assert raw.count(needle) == 2
        raw = raw.replace(needle, needle.replace(b"?;", b".or_else(|_| Ok(())).unwrap();"), 1)
    elif mutation == "receiver":
        needle = b".add(load.force_per_length)"
        assert raw.count(needle) >= 1
        raw = raw.replace(needle, b".different_receiver_method(load.force_per_length)", 1)
    elif mutation == "return":
        needle = b'return Err("canonical spring occurrence".into());'
        assert raw.count(needle) == 1
        raw = raw.replace(needle, b'let _ = "canonical spring occurrence";', 1)
    elif mutation == "display":
        needle = b"exact sum is outside the binary64 range"
        assert raw.count(needle) == 1
        raw = raw.replace(needle, needle + b" widened", 1)
    else:
        raw = raw.replace(b"\n", b"\r\n")
    target.write_bytes(raw)
    with pytest.raises(ValueError, match="source changed"):
        MODEL["state"](tmp_path, *fixture()[:2], fixture()[2], [])


def test_non_target_witness_is_unchanged(monkeypatch):
    def forbidden(_):
        raise AssertionError("C6 load for non-target")
    monkeypatch.setitem(MODEL["witness"].__globals__, "adjacent", forbidden)
    MODEL["witness"]({}, "other.rs", 1, "format", 0, {})


def test_additional_valid_source_allocation_cannot_inherit_owned_guard():
    table, graph, inventory = fixture()
    # This is the actual separate FK returning-format site, not an invented key.
    site = "structural/formation_check.rs:895"
    table["site_from"][site] = {
        "from": ["structural/formation_check.rs:850:evaluate"],
        "per": "1", "why": "SQ2 G5: " + MODEL["WHY"]}
    with pytest.raises(ValueError, match="marked allocation inventory changed"):
        MODEL["state"](SOURCE, table, graph, inventory, [])


def test_exact_envelope_excludes_only_two_owned_rp_allocations():
    table, graph, inventory = fixture()
    active = MODEL["state"](SOURCE, table, graph, inventory, [], MODEL["ENVELOPE_ZERO"])
    for spec in MODEL["SITES"]:
        if spec["file"].endswith("retained_product.rs"):
            MODEL["witness"](active, spec["file"], spec["line"], spec["kind"], 0, {})
            MODEL["witness"](active, spec["file"], spec["line"], spec["kind"], 0, {}, rebound=True)
            with pytest.raises(ValueError, match="became positive"):
                MODEL["witness"](active, spec["file"], spec["line"], spec["kind"], 1, {spec["owner"]: 1})
        else:
            with pytest.raises(ValueError, match="KD5C6"):
                MODEL["witness"](active, spec["file"], spec["line"], spec["kind"], 0, {})


@pytest.mark.parametrize("extra", [
    "core/product_physics/src/retained_product.rs",
    "core/product_physics/src/retained_product.rs,other.rs",
    "unknown",
])
def test_partial_extra_unknown_envelope_exclusion_refuses(extra):
    table, graph, inventory = fixture()
    with pytest.raises(ValueError, match="unknown envelope exclusion"):
        MODEL["state"](SOURCE, table, graph, inventory, [], extra)


def test_envelope_emitter_source_change_refuses(tmp_path, monkeypatch):
    source = (PRICING / "base/sens.py").read_bytes()
    needle = b'if variant == "_env": extra["TB_EXTRA_ZERO"] = ENVZERO'
    assert source.count(needle) == 1
    (tmp_path / "sens.py").write_bytes(source.replace(needle, needle + b",extra_unknown", 1))
    monkeypatch.setitem(MODEL["envelope_projection"].__globals__, "__file__", str(tmp_path / "b7a_returning_errors.py"))
    with pytest.raises(ValueError, match="emitter source changed"):
        MODEL["envelope_projection"](MODEL["ENVELOPE_ZERO"])
