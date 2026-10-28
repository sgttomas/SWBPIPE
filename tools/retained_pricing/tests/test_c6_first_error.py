"""Light C6 source/graph/site refusal controls; no pricing or native jobs."""
import copy
import json
import os
from pathlib import Path
import runpy
import shutil
import subprocess
import sys
import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = Path(os.environ.get("RP_C6_SNAPSHOT", str(PRICING.parents[1])))
MODEL = runpy.run_path(str(PRICING / "base/kd5_first_error_model.py"))
STATE = MODEL["inspect"](SOURCE)
CPP = MODEL["CPP"]

# Compact independent boundary definitions, not saved agent-run artifacts.
GRAPH = {"edges": {k: [] for k in STATE["keys"].values()}, "site_loops": [], "spans": {}}
PAIRS = (("summary_row", "divided"), ("stress_mpa", "divided"),
    ("maximum_summary_mpa", "add"), ("maximum_summary_mpa", "divided"),
    ("fallback_section_errors", "accumulate"), ("fallback_section_errors", "active_length"),
    ("fallback_section_errors", "add"), ("fallback_section_errors", "exact"),
    ("fallback_section_errors", "max"), ("fallback_section_errors", "min"),
    ("fallback_section_errors", "mul"), ("fallback_section_errors", "rounded"),
    ("fallback_circular_maximum", "accumulate"), ("fallback_circular_maximum", "add"),
    ("fallback_circular_maximum", "divided"), ("fallback_circular_maximum", "divided_scaled"),
    ("fallback_circular_maximum", "exact"), ("fallback_circular_maximum", "exact_scaled"),
    ("fallback_circular_maximum", "mul"), ("fallback_circular_maximum", "rounded"),
    ("fallback_circular_maximum", "fallback_section_errors"),
    ("fallback_circular_maximum", "directed_transport_sum"),
    ("straight_stress_spans", "directed_transport_sum"),
    ("maximum_cover", "public_mpa_magnitude_in_pa_upper"), ("circular_maximum", "maximum_cover"))
for caller, target in PAIRS:
    GRAPH["edges"][STATE["keys"][caller]].append(STATE["keys"][target])
# B7a C5's complete member API/body removes nine staging-frame lines from
# the reviewed batch-5 production prefix. Every allocation owner body is
# unchanged; this independent current-site inventory keeps all 14 capped and
# three uncapped sites, including their original negative controls.
# T4-U6/U8: the same sites at the priced source's lines, unchanged text (B7b: 511, 522, 535, 538, 541, 637, 641, 643,
# 646, 648, 651, 716, 765, 978, 990, 1107, 1127); fallback_circular_maximum's two added comment lines move its later
# sites by 2 more. T4-S5 (epoch 4): five lines added above them move every site by 5 (T4-U6/U8:
# 519, 530, 543, 546, 549, 647, 651, 653, 656, 658, 661, 726, 775, 988, 1002, 1119, 1139).
# W-16 stage 2 R-c (§8.29): support/support_record's spring-ground parameter moves every site by 4, unchanged text
# (524, 535, 548, 551, 554, 652, 656, 658, 661, 663, 666, 731, 780, 993, 1007, 1124, 1144).
CHECKED = 735
SITE_INVENTORY = ((528,"format","maximum_cover"), (539,"format","maximum_cover"),
    (552,"to_string","public_mpa_magnitude_in_pa_upper"),
    (555,"to_string","public_mpa_magnitude_in_pa_upper"),
    (558,"to_string","public_mpa_magnitude_in_pa_upper"),
    (656,"into_text","directed_transport_sum"), (660,"to_string","directed_transport_sum"),
    (662,"to_string","directed_transport_sum"), (665,"to_string","directed_transport_sum"),
    (667,"into_text","directed_transport_sum"), (670,"to_string","directed_transport_sum"),
    (CHECKED,"into_text","checked"), (784,"into_text","divided"),
    (997,"into_text","fallback_circular_maximum"), (1011,"into_text","fallback_circular_maximum"),
    (1128,"into_text","fallback_circular_maximum"), (1148,"format","fallback_circular_maximum"))
FORMATS, LEX = [], []
for line, kind, owner in SITE_INVENTORY:
    row = {"file": CPP, "line": line, "kind": kind, "fn": STATE["keys"][owner]}
    (FORMATS if kind == "format" else LEX).append(row)


def table():
    return {"classes": {"ident":128}, "first_error_guard":"KD5C6",
            "site_from": dict(MODEL["site_rules"](STATE))}


def snapshot(tmp_path):
    for file in [CPP, MODEL["LIB"], "core/solver/frame_kernel/src/load_ledger.rs"]:
        to = tmp_path / file
        to.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / file, to)
    return tmp_path


def test_actual_all_edge_and_17_site_boundary():
    MODEL["verify_rules"](STATE, table())
    MODEL["verify_graph"](STATE, GRAPH)
    MODEL["verify_inventory"](STATE, FORMATS, LEX)
    assert len(STATE["sites"]) == 14
    assert len(MODEL["PRIVATE"]) == 12


@pytest.mark.parametrize("change", ["missing", "zero", "allzero", "missing_site_witness"])
def test_each_root_must_be_positive(change):
    row = STATE["sites"][11]  # checked: all four preserved entry-root allowances
    counts = {k: 1 for k in row["roots"]}
    mult = 1
    if change == "missing": counts.pop(row["roots"][0])
    if change == "zero": counts[row["roots"][0]] = 0
    if change == "allzero": counts = {k: 0 for k in counts}; mult = 0
    if change == "missing_site_witness": mult = 0
    with pytest.raises(ValueError, match="positive witnessed"):
        MODEL["positive_witness"](STATE, row["file"], row["line"], row["kind"], mult, counts)


@pytest.mark.parametrize("change", ["new_unreached_caller", "duplicate_parent", "missing_parent", "missing_node"])
def test_all_graph_parent_inventory_refuses(change):
    graph = copy.deepcopy(GRAPH)
    keys = STATE["keys"]
    if change == "new_unreached_caller": graph["edges"]["core/product_physics/src/elsewhere.rs:1:new"] = [keys["checked"]]
    if change == "duplicate_parent": graph["edges"][keys["fallback_circular_maximum"]].append(keys["directed_transport_sum"])
    if change == "missing_parent": graph["edges"][keys["straight_stress_spans"]].remove(keys["directed_transport_sum"])
    if change == "missing_node": graph["edges"].pop(keys["checked"])
    with pytest.raises(ValueError): MODEL["verify_graph"](STATE, graph)


@pytest.mark.parametrize("change", ["missing", "duplicate", "new", "wrong_kind", "wrong_owner"])
def test_exact_kind_site_inventory_refuses(change):
    lexical = copy.deepcopy(LEX)
    index = next(i for i,r in enumerate(lexical) if r["file"] == CPP and r["line"] == CHECKED)
    if change == "missing": lexical.pop(index)
    if change == "duplicate": lexical.append(copy.deepcopy(lexical[index]))
    if change == "new": lexical.append(dict(lexical[index], line=CHECKED + 1))
    if change == "wrong_kind": lexical[index]["kind"] = "clone_text"
    if change == "wrong_owner": lexical[index]["fn"] = STATE["keys"]["maximum_summary_mpa"]
    with pytest.raises(ValueError, match="kind\\+site|misqualified"): MODEL["verify_inventory"](STATE, FORMATS, lexical)


@pytest.mark.parametrize("change", ["missing_rule", "per", "from", "no_guard", "no_guard_or_tags"])
def test_rule_and_marker_removal_refuses(change):
    rules = table()
    key = MODEL["site_rules"](STATE)[0][0]
    if change == "missing_rule": rules["site_from"].pop(key)
    if change == "per": rules["site_from"][key]["per"] = "2"
    if change == "from": rules["site_from"][key]["from"] = []
    if change in ["no_guard", "no_guard_or_tags"]: rules.pop("first_error_guard")
    if change == "no_guard_or_tags":
        for value in rules["site_from"].values(): value["why"] = value["why"].replace("KD5C6:", "untagged:")
    # Source detection is unconditional: removing both table markers cannot choose history.
    assert MODEL["inspect"](SOURCE) is not None
    with pytest.raises(ValueError): MODEL["verify_rules"](STATE, rules)


@pytest.mark.parametrize("change", ["catch", "continue", "retry", "second_fallback_retry", "filter_map", "vec_result", "swallow", "new_private_caller", "rename_marker", "remove_cpp"])
def test_source_changes_refuse(tmp_path, change):
    source = snapshot(tmp_path)
    file = source / CPP
    text = file.read_text()
    if change == "remove_cpp":
        file.unlink()
    elif change == "second_fallback_retry":
        file = source / MODEL["LIB"]
        text = file.read_text()
        old = "certified_publication::fallback_circular_maximum(&spans,end_forces,loads,length,section,pressure_state)"
        assert old in text
        file.write_text(text.replace(old, old + ".or_else(|_| " + old + ")"))
    else:
        replacements = {
            "catch": ("fallback_section_errors(ends, loads, length)?", "fallback_section_errors(ends, loads, length).unwrap_or([FallbackError::ZERO;6])"),
            "continue": ("return Err(\"fallback polynomial arithmetic transport range\".into());", "continue;"),
            "retry": ("let rows = fallback_section_errors(ends, loads, length)?;", "let rows = fallback_section_errors(ends, loads, length).or_else(|_|fallback_section_errors(ends,loads,length))?;"),
            "filter_map": ("        .collect()", "        .filter_map(Result::ok).map(Ok).collect()"),
            "vec_result": ("        .collect()", "        .collect::<Vec<Result<_,String>>>()"),
            "swallow": ("        .collect()", "        .collect::<Result<Vec<_>,String>>().or_else(|_|Ok(Vec::new()))"),
            "new_private_caller": ("#[cfg(test)]", "fn added_private_caller()->Result<FallbackError,String>{FallbackError::exact(1.0)}\n#[cfg(test)]"),
            "rename_marker": ("fallback_circular_maximum", "renamed_fallback_circular_maximum"),
        }
        old,new = replacements[change]
        if change in ("filter_map", "vec_result", "swallow"):
            body, _ = MODEL["HELPERS"]["_indented_function"](text, "straight_stress_spans")
            assert old in body
            file.write_text(text.replace(body, body.replace(old,new,1)))
        else:
            assert old in text
            file.write_text(text.replace(old,new,1))
    with pytest.raises(ValueError): MODEL["inspect"](source)


def test_known_old_source_has_no_caps(tmp_path):
    # Three original wrappers are a small source distinction, requiring no Git history.
    source = r"""pub fn product_upward(a: f64, b: f64) -> f64 {
    let p = a * b;
    if p.is_finite() && a.mul_add(b, -p) > 0.0 {
        p.next_up()
    } else if p == 0.0 && a != 0.0 && b != 0.0 {
        // Underflow below the smallest subnormal.
        f64::from_bits(1)
    } else {
        p
    }
}
pub fn product_downward(a: f64, b: f64) -> f64 {
    let p = a * b;
    if p.is_finite() && a.mul_add(b, -p) < 0.0 {
        p.next_down().max(0.0)
    } else {
        p
    }
}
pub fn quotient_downward(a: f64, b: f64) -> f64 {
    let q = a / b;
    if q.is_finite() && q.mul_add(b, -a) > 0.0 {
        q.next_down().max(0.0)
    } else {
        q
    }
}
"""
    old = tmp_path / "core/solver/frame_kernel/src/load_ledger.rs"
    old.parent.mkdir(parents=True)
    old.write_text(source)
    assert MODEL["inspect"](tmp_path) is None
    old.write_text(source.replace("let p = a * b;", "let p = a * b + 1.0;", 1))
    with pytest.raises(ValueError): MODEL["inspect"](tmp_path)


def test_actual_entry_gate_refuses_caps_with_all_table_markers_removed(tmp_path):
    rules = table()
    rules.pop("first_error_guard")
    for value in rules["site_from"].values():
        value["why"] = value["why"].replace("KD5C6:", "untagged:")
    target = tmp_path / "table.json"
    target.write_text(json.dumps(rules))
    inputs = {"inventory.json": {"rows": FORMATS}, "graph.json": GRAPH,
              "loops.json": {"counts":{"caps":{}}, "loops":[]}, "lexicon.json":{"rows":LEX}}
    for name, value in inputs.items(): (tmp_path / name).write_text(json.dumps(value))
    env = dict(os.environ, TB_LEXICON=str(tmp_path / "lexicon.json"),
               G4_CAPS='{"l":128,"c":3,"lc":3,"z":0}')
    env.pop("RP_C6_EVIDENCE", None)
    result = subprocess.run([sys.executable, str(PRICING / "base/text_budget.py"),
        str(SOURCE), str(tmp_path / "inventory.json"),
        str(tmp_path / "graph.json"), str(tmp_path / "loops.json"),
        str(target), "run_linear_static_preview_value_with_retained_direct", "caps"],
        env=env, capture_output=True, text=True)
    assert result.returncode != 0
    assert "KD5C6: missing/versioned first-error guard" in result.stderr
    assert not result.stdout.strip()  # refusal precedes pricing/M computation


@pytest.mark.parametrize("family", ["site_from", "site_zero", "site_total", "site_aggregate", "site_size", "lex_site_size"])
@pytest.mark.parametrize("alias", [
    f"certified_publication.rs:{CHECKED}:into_text", f"src/certified_publication.rs:{CHECKED}",
    f"product_physics/src/certified_publication.rs:{CHECKED}", f"core/product_physics/src/certified_publication.rs:{CHECKED}",
])
def test_all_count_and_byte_override_aliases_refuse(family, alias):
    rules = table()
    rules.setdefault(family,{})[alias] = {"from":[],"per":"0","class":0,"bytes":0,"why":"untagged override"}
    with pytest.raises(ValueError, match="conflicting allocation override"):
        MODEL["verify_rules"](STATE,rules)


@pytest.mark.parametrize("mult", [0, 1, 5])
def test_actual_post_rebind_count_must_match_positive_roots(mult):
    row = STATE["sites"][11]
    counts = {k:1 for k in row["roots"]}
    with pytest.raises(ValueError):
        MODEL["rebound_witness"](STATE,row["file"],row["line"],row["kind"],mult,counts)
    MODEL["rebound_witness"](STATE,row["file"],row["line"],row["kind"],4,counts)


@pytest.mark.parametrize("family,alias,override", [
    ("lex_site_size", f"product_physics/src/certified_publication.rs:{CHECKED}", {"class":0}),
    ("lex_site_size", f"src/certified_publication.rs:{CHECKED}", {"class":0}),
    ("site_zero", f"certified_publication.rs:{CHECKED}:into_text", "untagged zero multiplier"),
    ("site_from", f"product_physics/src/certified_publication.rs:{CHECKED}", {"from":[],"per":"0","why":"untagged"}),
])
def test_actual_entry_refuses_zero_byte_and_zero_count_aliases(tmp_path,family,alias,override):
    rules = table()
    rules.setdefault(family,{})[alias] = override
    inputs = {"inventory.json":{"rows":FORMATS},"graph.json":GRAPH,
        "loops.json":{"counts":{"caps":{}},"loops":[]},"lexicon.json":{"rows":LEX},"table.json":rules}
    for name,value in inputs.items():(tmp_path/name).write_text(json.dumps(value))
    env = dict(os.environ,TB_LEXICON=str(tmp_path/"lexicon.json"),G4_CAPS='{"l":128,"c":3,"lc":3,"z":0}')
    env.pop("RP_C6_EVIDENCE",None)
    result = subprocess.run([sys.executable,str(PRICING/"base/text_budget.py"),str(SOURCE),
        str(tmp_path/"inventory.json"),str(tmp_path/"graph.json"),str(tmp_path/"loops.json"),
        str(tmp_path/"table.json"),"run_linear_static_preview_value_with_retained_direct","caps"],
        env=env,capture_output=True,text=True)
    assert result.returncode != 0
    assert "KD5C6: conflicting allocation override" in result.stderr
    assert not result.stdout.strip()  # both suppressions refuse before pricing/M
