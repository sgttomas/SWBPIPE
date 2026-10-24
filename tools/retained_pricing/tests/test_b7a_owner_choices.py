"""Source, allocation choice, auxiliary ownership and effective priority."""
from copy import deepcopy
import json
import os
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
    # T4-S5 (U4B-W1): the NotCovered base recipe adds an eleventh mutually exclusive group row.
    assert group["members"]==[r["line"] for r in rows[:11]]
    assert group["bytes"]==338 and group["req"]==676*actual
    assert len(rows)==18 and all(row["req"]>0 for row in rows)
    # Subtract only the mutually exclusive eleven prospective standalone costs.
    total=sum(row["req"] for row in rows)-group["standalone_requested_bytes"]+group["req"]
    assert total==676*actual+sum(row["req"] for row in rows[11:])


@pytest.mark.parametrize("family",["site_zero","site_from","site_total","site_aggregate","site_size","lex_site_size"])
# T4-U6/U8: the same sites at the priced source's lines, unchanged text (B7b: structural.rs 1124; retained_product.rs
# 3057, 3071, 3073, 3185, 3194, 3217, 3005, 3227; lib.rs helpers 13244/13619; 433fc1f1e4: 13587/13966; addbdc030e:
# 13623/14002; T4-U6/U8 final: 3208, 3222, 3224, 3336, 3345, 3375, 13630, 14009, 3156, 3385; T4-S5 epoch 4 below).
@pytest.mark.parametrize("line",[MODEL["SOURCE_LINE"],3402,3416,3418,3530,3539,3592,14050,14471,3350,3602])   # O4 S5: retained_product.rs +167; pre-0.4 unification: lib.rs +103; W-8: +2; silent-zero guard: +1
def test_complete_group_source_error_and_auxiliary_alias_refusal(family,line):
    args=inputs();table=args[0]
    file=MODEL["FK"]+"structural.rs" if line==MODEL["SOURCE_LINE"] else MODEL["PP"]+("lib.rs" if line in (14050,14471) else "retained_product.rs")
    kind={MODEL["SOURCE_LINE"]:"clone_text",3530:"to_string",14471:"replace"}.get(line,"format")
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


@pytest.mark.parametrize("site",[2787,3350,3602])   # B7b: 2544, 3005, 3227; T4-U6/U8: 2622, 3156, 3385; T4-S5: 2644, 3183, 3435
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
