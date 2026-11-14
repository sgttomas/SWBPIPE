"""Source, allocation choice, auxiliary ownership and effective priority."""
from copy import deepcopy
import json
import os
import re
from pathlib import Path
import runpy
import shutil

import pytest

PRICING=Path(__file__).resolve().parents[1]
SOURCE=Path(os.environ.get("B7A_PHASE_TEST_SOURCE",str(PRICING.parents[1])))
MODEL=runpy.run_path(str(PRICING/"base/b7a_owner_choices.py"))


def inputs():
    p=deepcopy(MODEL["PREIMAGES"])
    table={"site_from":{MODEL["SOURCE_RULE"]:p["source_from"]},
           "id_audit":{**p["audit_preimages"],**p["aux_audits"],
                       MODEL["SOURCE_SITE"]:{"t.source":{"bytes":128,"source":"IN128"}}},
           "lex_site_size":p["lex_preimages"]}
    # Absent audit records are represented by absent keys, not null rules.
    table["id_audit"]={k:v for k,v in table["id_audit"].items() if v is not None}
    for family,values in p["aux_overrides"].items():table.setdefault(family,{}).update(values)
    table,_=MODEL["transform"](SOURCE,table)
    loops={"loops":[p["source_loop"]]}
    graph={"edges":{target:[] for target in p["parents"]},"site_loops":p["site_stacks"]}
    for target,parents in p["parents"].items():
        for parent in parents:graph["edges"].setdefault(parent,[]).append(target)
    rows=p["group_rows"]+p["aux_rows"]+p["helper_rows"]+p["error_rows"]
    source=dict(file=MODEL["FK"]+"structural.rs",line=MODEL["SOURCE_LINE"],kind="clone_text",fn=MODEL["SOURCE_OWNER"],receiver="t.source",literal_bytes=None,arg="")
    return table,loops,graph,[r for r in rows if r["kind"]=="format"],[r for r in rows if r["kind"]!="format"]+[source]


def active(projection=""):
    args=inputs()
    return MODEL["state"](SOURCE,*args,projection),args


@pytest.mark.parametrize("actual",[21150,987,1])
def test_complete_choice_once_and_all_auxiliaries_positive(actual):
    context,_=active()
    counts={MODEL["VALIDATOR"]:actual,MODEL["ERROR_OWNER"]:10}
    rows=[]
    for spec in MODEL["PREIMAGES"]["group_rows"]:
        line=spec["line"]
        rows.append(dict(spec,fn=MODEL["VALIDATOR"],loops=[],mult=actual,bytes=4000,req=8000*actual))
    for spec in MODEL["PREIMAGES"]["aux_rows"]+MODEL["PREIMAGES"]["helper_rows"]:
        owner=spec.get("fn",MODEL["VALIDATOR"])
        counts[owner]=actual*3 if owner!=MODEL["VALIDATOR"] else actual
        rows.append(dict(spec,fn=owner,loops=[],mult=counts[owner],req=counts[owner]*20))
    group=MODEL["recipe_group"](context,rows,counts)
    # T4-S5 (U4B-W1): the NotCovered base recipe adds an eleventh mutually exclusive group row; W-7 (§8.20) the global
    # end NotCovered arm's recipe a twelfth.
    assert group["members"]==[r["line"] for r in rows[:12]]
    assert group["bytes"]==338 and group["req"]==676*actual
    assert len(rows)==19 and all(row["req"]>0 for row in rows)
    # Subtract only the mutually exclusive twelve prospective standalone costs.
    total=sum(row["req"] for row in rows)-group["standalone_requested_bytes"]+group["req"]
    assert total==676*actual+sum(row["req"] for row in rows[12:])


# The sites are the model's own rows (one complete group row, every auxiliary and helper row, and the error rows
# after the first), read from PREIMAGES rather than restated, so a carry of the model's line keys carries the test.
ALIAS_SITES=[(MODEL["FK"]+"structural.rs",MODEL["SOURCE_LINE"],"clone_text")]+[
    (row["file"],row["line"],row["kind"]) for row in
    MODEL["PREIMAGES"]["group_rows"][:1]+MODEL["PREIMAGES"]["aux_rows"]+MODEL["PREIMAGES"]["helper_rows"]
    +MODEL["PREIMAGES"]["error_rows"][1:]]
# The same sites stated independently of the model, by enclosing function and line text (and which occurrence of
# that text in the function), so that a row deleted from the model cannot delete its coverage.
EXPECTED_ALIAS_SITES=[
    ("structural.rs","clone_text","row_sources","let mut sources: Vec<String> = terms.iter().map(|t| t.source.clone()).collect();",0),
    ("retained_product.rs","format","validate_final_metadata",'format!("result:disp:{suffix}")',0),
    ("retained_product.rs","format","validate_final_metadata",'format!("positive value follows the global cartesian {axis} axis displacement of the node")',0),
    ("retained_product.rs","format","validate_final_metadata",'format!("positive value follows the right-hand-rule rotation about the global cartesian {axis} axis")',0),
    ("retained_product.rs","format","validate_final_metadata",'(station_id_location(loc),"recovered_from_open_mechanics_stress_components",format!("{prefix}; j-side section action in the element-local frame (x toward end j); section equilibrium from stiffness-recovered end actions with consistent distributed-load fixed-end correction"))',0),
    ("retained_product.rs","format","validate_final_metadata",'format!("result:combination:{}:{}", stable_suffix(id), super::result_tail(&expected))',0),
    ("retained_product.rs","to_string","validate_final_metadata","STRAIGHT_ENDPOINT_SECTION_SIGN_CONVENTION.to_string(),",0),
    ("lib.rs","format","qualified_load_case_result_id","format!(",0),
    ("lib.rs","replace","stable_suffix","id.replace(':', \"-\")",0),
    ("retained_product.rs","format","validate_final_metadata","return Err(CaptureError::Association(format!(",0),
    ("retained_product.rs","format","validate_final_metadata","return Err(CaptureError::Association(format!(",1),
]


def enclosing_site(file,line):
    """(basename, enclosing fn, stripped text, occurrence of that text within the fn) of a source line."""
    lines=(SOURCE/file).read_bytes().decode().split("\n")
    start=line-1
    while start>=0 and not re.match(r"^\s*(pub(\([^)]*\))?\s+)?fn\s+\w+",lines[start]):start-=1
    assert start>=0,(file,line)
    function=re.match(r"^\s*(pub(\([^)]*\))?\s+)?fn\s+(\w+)",lines[start]).group(3)
    text=lines[line-1].strip()
    occurrence=sum(1 for i in range(start,line-1) if lines[i].strip()==text)
    return file.rsplit("/",1)[1],function,text,occurrence


def test_the_alias_sites_are_exactly_the_expected_owner_sites():
    observed=sorted((name,kind,function,text,occurrence) for file,line,kind in ALIAS_SITES
                    for name,function,text,occurrence in [enclosing_site(file,line)])
    assert observed==sorted(EXPECTED_ALIAS_SITES)


@pytest.mark.parametrize("family",["site_zero","site_from","site_total","site_aggregate","site_size","lex_site_size"])
@pytest.mark.parametrize("site",ALIAS_SITES,ids=lambda site:f"{site[0].rsplit('/',1)[1]}:{site[1]}")
def test_complete_group_source_error_and_auxiliary_alias_refusal(family,site):
    args=inputs();table=args[0]
    file,line,kind=site
    table.setdefault(family,{})[file+":"+str(line)+":"+kind]={"bytes":0}
    with pytest.raises(ValueError):MODEL["state"](SOURCE,*args)


@pytest.mark.parametrize("mutation",["parent","stack","orphan","duplicate","missing","audit","loop","earlier_loop","function_cap","contents"])
def test_actual_current_inventory_priority_and_source_width_refuse(mutation):
    args=inputs();table,loops,cg,formats,lex=args
    target=MODEL["SOURCE_OWNER"]
    if mutation=="parent":cg["edges"]["unknown.rs:1:retry"]=[target]
    if mutation=="stack":next(r for r in cg["site_loops"] if r[1]==target)[2]=[["for retry in retries"]]
    if mutation=="orphan":cg["site_loops"].append(["unknown.rs:1:orphan",target,[[]]])
    if mutation=="duplicate":formats.append(deepcopy(formats[0]))
    if mutation=="missing":formats.pop(0)
    if mutation=="audit":table["id_audit"][MODEL["SOURCE_SITE"]]["t.source"]["bytes"]=0
    if mutation=="loop":loops["loops"][0]["bound"]="l"
    if mutation=="earlier_loop":loops["loops"].insert(0,{"re":"terms","bound":"0","why":"unknown"})
    if mutation=="function_cap":loops["fn_cap"]=[{"fn":target,"cap":"0"}]
    if mutation=="contents":table[MODEL["MARKER"]]["base_contents"]["3314"]=1   # B7b: 3163
    with pytest.raises(ValueError):MODEL["state"](SOURCE,*args)


@pytest.mark.parametrize("loads,one_audit",[(128,336),(128,224),(128,112)])
def test_effective_both_audit_counts_source_width_and_capacity_unchanged(loads,one_audit):
    context,_=active()
    counts={MODEL["FK"]+name:one_audit for name in MODEL["PREIMAGES"]["source_from"]["from"]}
    args=(context,MODEL["FK"]+"structural.rs",MODEL["SOURCE_LINE"],"clone_text",2*one_audit*12*loads,counts,loads)
    MODEL["source_witness"](*args,rebound=True,owner=MODEL["SOURCE_OWNER"],stack=["iter:terms.iter().map"])
    assert args[4]*128*2=={336:264241152,224:176160768,112:88080384}[one_audit]
    with pytest.raises(ValueError):MODEL["source_witness"](*args[:4],0,*args[5:],owner=MODEL["SOURCE_OWNER"],stack=["iter:terms.iter().map"])


def test_canonical_envelope_projection_has_positive_audits_and_helpers_only_rp_zero():
    mode=runpy.run_path(str(PRICING/"base/b7a_returning_errors.py"))["ENVELOPE_ZERO"]
    context,_=active(mode)
    counts={MODEL["FK"]+name:336 for name in MODEL["PREIMAGES"]["source_from"]["from"]}
    MODEL["source_witness"](context,MODEL["FK"]+"structural.rs",MODEL["SOURCE_LINE"],"clone_text",1032192,counts,128,
                              rebound=True,owner=MODEL["SOURCE_OWNER"],stack=["iter:terms.iter().map"])
    rows=[]
    for spec in MODEL["PREIMAGES"]["group_rows"]+MODEL["PREIMAGES"]["aux_rows"]+MODEL["PREIMAGES"]["helper_rows"]:
        owner=spec.get("fn",MODEL["VALIDATOR"])
        calls=0 if owner==MODEL["VALIDATOR"] else 123
        counts[owner]=calls
        rows.append(dict(spec,fn=owner,loops=[],mult=calls,req=20*calls))
    assert MODEL["recipe_group"](context,rows,counts)["req"]==0
    with pytest.raises(ValueError):active(mode+",unknown")


def test_saved_rule_json_retains_exact_marker_and_preimages():
    args=inputs()
    saved=tuple(json.loads(json.dumps(x)) for x in args)
    assert MODEL["state"](SOURCE,*saved)==MODEL["state"](SOURCE,*args)


@pytest.mark.parametrize("site",[2803,3366,3631])   # B7b: 2544, 3005, 3227; T4-U6/U8: 2622, 3156, 3385; T4-S5: 2644, 3183, 3435; W-7: 2787, 3350, 3602 before
def test_first_error_keeps_full_validator_and_whole_bind_owner_counts(site):
    context,_=active()
    counts={MODEL["VALIDATOR"]:21150,MODEL["ERROR_OWNER"]:10}
    owner=MODEL["ERROR_OWNER"] if site==MODEL["ROWS_ERROR_LINE"] else MODEL["VALIDATOR"]
    stack=["for observed in view.rows()"] if site==MODEL["ROWS_ERROR_LINE"] else []
    MODEL["error_witness"](context,MODEL["PP"]+"retained_product.rs",site,"format",21150,counts,owner,stack)
    MODEL["error_witness"](context,MODEL["PP"]+"retained_product.rs",site,"format",10,counts,owner,stack,rebound=True)
    with pytest.raises(ValueError):MODEL["error_witness"](context,MODEL["PP"]+"retained_product.rs",site,"format",0,counts,owner,stack,rebound=True)


@pytest.mark.parametrize("file,fragment",[
 ("core/solver/frame_kernel/src/structural.rs",b"let r = residual.round()?;"),
 ("core/solver/frame_kernel/src/structural.rs",b"sources: row_sources(terms),"),
 ("core/solver/frame_kernel/src/structural/sparse.rs",b"for term in force_terms"),
 ("core/product_physics/src/retained_product.rs",b"let expected = match recipe"),
 ("core/product_physics/src/retained_product.rs",b"validate_final_metadata("),
 ("core/product_physics/src/lib.rs",b"fn station_id_location"),
])
def test_partition_terminal_return_recipe_custody_and_helper_source_drift(tmp_path,file,fragment):
    domain=runpy.run_path(str(PRICING/"base/b7a_domain_model.py"))
    formation=runpy.run_path(str(PRICING/"base/b7a_formation_model.py"))
    scoped=runpy.run_path(str(PRICING/"base/b7a_scoped_text_model.py"))
    files=set(domain["SOURCE_FILES"])|set(formation["SOURCE_FILES"])|{scoped["UNITS"],"core/product_physics/src/retained_memory.rs"}
    for relative in files:
        path=tmp_path/relative;path.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(SOURCE/relative,path)
    path=tmp_path/file;data=path.read_bytes();assert fragment in data
    path.write_bytes(data.replace(fragment,fragment+b"/* source drift */",1))
    with pytest.raises(ValueError):MODEL["inspect"](tmp_path)
