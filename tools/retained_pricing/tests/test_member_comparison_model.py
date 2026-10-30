"""Bounded C8 source/frame and real allocation-inventory controls."""
import ast, collections, json, os, re, runpy, subprocess, sys
from pathlib import Path
import pytest
P=Path(__file__).resolve().parents[1]
SOURCE=Path(os.environ.get("RP_C8_SNAPSHOT",str(P.parents[1])))
MODEL=runpy.run_path(str(P/"base/member_comparison_model.py"))

def state():return MODEL["inspect"](SOURCE)

def metadata(monkeypatch):
    code=(P/"rules/sq2_decisions_E.py").read_text()
    begin=code.index("# C8: source-bound")
    end=code.index("ID_AUDIT_WHY_E =",begin)
    monkeypatch.setenv("SQ2_SNAP",str(SOURCE))
    ns={"os":os,"runpy":runpy,"H":str(P/"rules"),"Resolved":str,
        "IN":{"bytes":128,"source":"IN128"},"ID_AUDIT":[]}
    exec(compile(code[begin:end],"actual C8 route E audit disposition","exec"),ns)
    return ns

def test_exact_source_and_positive_case_coefficient():
    assert MODEL["coefficient"](3,state())==(3 if state()["borrowed"] else 0)
    assert MODEL["SLOTS"]=={"&'static u8":16,"&'static [u8]":8,"usize":16,
        "u64":4,"f64":4,"u8":4,"bool":16,"i32":4}

@pytest.mark.parametrize("mutation",["rename","extra_binding","changed_guard","duplicate_owner","crlf"])
def test_changed_source_refuses(tmp_path,mutation):
    source=(SOURCE/MODEL["FILE"]).read_bytes().decode("utf-8")
    if mutation=="crlf":
        assert "\r\n" not in source
        source=source.replace("\n","\r\n")
    if mutation=="rename":source=source.replace("fn composite_exact(","fn renamed_comparison(",1)
    if mutation=="extra_binding":
        start=source.index("fn composite_exact(")
        source=source[:start]+source[start:].replace("let descriptor = selected","let extra = &member.member_id;\n                    let descriptor = selected",1)
    if mutation=="changed_guard":
        start=source.index("fn composite_exact(")
        source=source[:start]+source[start:].replace("FunctionalQuantity::MemberSection {","FunctionalQuantity::OtherSection {",1)
    if mutation=="duplicate_owner":source+='\nfn composite_exact() {}\n'
    assert source!=(SOURCE/MODEL["FILE"]).read_text()
    file=tmp_path/MODEL["FILE"];file.parent.mkdir(parents=True);file.write_bytes(source.encode("utf-8"))
    with pytest.raises(ValueError):MODEL["inspect"](tmp_path)

def test_actual_sourceupper_expression_and_all_stage_placement():
    native=(P/"base/g5_profile.py").read_text();tree=ast.parse(native)
    nodes=[node for node in tree.body if
       (isinstance(node,ast.FunctionDef) and node.name in {"upr","align_expr"}) or
       (isinstance(node,ast.Assign) and any(isinstance(t,ast.Name) and t.id.startswith("_C8_") for t in node.targets)) or
       (isinstance(node,ast.Assign) and "MemberComparisonWorkspace" in ast.unparse(node))]
    ns={"BIND":{}};exec(compile(ast.Module(body=nodes,type_ignores=[]),"actual C8 native expression","exec"),ns)
    assert ns["BIND"]["s(MemberComparisonWorkspace)"][0]=="SourceUpper"
    assert dict(zip(ns["_C8_TYPES"],ns["_C8_SLOTS"]))==MODEL["SLOTS"]
    code=(P/"base/t25_g4.py").read_text()
    assert 'stage[_key] = stage[_key] + _c8_workspace' in code
    assert 'inv[_key] = inv[_key] + _c8_workspace' in code
    assert 's("MemberComparisonWorkspace") * _c8_coefficient(CC, _c8_state)' in code
    assert 'carried_case * CC' in code

def test_owned_audit_only_actual_g6_inventory(monkeypatch,tmp_path):
    ns=metadata(monkeypatch);audits=ns["C8_INPUT_METADATA"]
    assert len(audits)==1 and audits[0][1]=={"member.member_id":{"bytes":128,"source":"IN128"}}
    assert ns["ID_AUDIT"]==([] if state()["borrowed"] else audits)
    source=(SOURCE/MODEL["FILE"]).read_text()
    hit=re.search(r"^([ \t]*)(?:pub(?:\([^)]*\))?\s+)?fn composite_exact\b",source,re.M)
    end=re.search(r"^"+re.escape(hit.group(1))+r"}",source[hit.end():],re.M)
    line=source.count(chr(10),0,hit.start())+1
    fn=MODEL["FILE"]+":"+str(line)+":composite_exact"
    graph={"edges":{fn:[]},"spans":{fn:[MODEL["FILE"],hit.start(),hit.end()+end.end()]}}
    edges=tmp_path/"edges.json";edges.write_text(json.dumps(graph));lex=tmp_path/"lex.json"
    subprocess.run([sys.executable,str(P/"base/text_lexicon.py"),str(SOURCE),str(edges),fn,str(lex)],check=True,capture_output=True,text=True)
    inv=json.loads(subprocess.check_output([sys.executable,str(P/"base/template_inventory.py"),".",MODEL["FILE"]],cwd=SOURCE,text=True))
    consumer=(P/"base/text_budget.py").read_text()
    defs=[node for node in ast.parse(consumer).body if isinstance(node,ast.FunctionDef) and node.name in {"short","audit_key"}]
    begin=consumer.index("# G6 repair (RV87 SF-2): a site-keyed");end=consumer.index("# G7: a cycle",begin)
    def check(audits):
        c={"re":re,"collections":collections,"inv":inv,"LEX":json.loads(lex.read_text())["rows"],
           "TA":{"id_audit":dict(audits)},"ID_AUDIT":dict(audits),"unclassified":collections.defaultdict(int)}
        exec(compile(ast.Module(body=defs,type_ignores=[]),"actual G6 functions","exec"),c)
        exec(compile(consumer[begin:end],"actual G6 inventory condition","exec"),c)
        return dict(c["unclassified"])
    assert check(ns["ID_AUDIT"])=={}
    if state()["borrowed"]:
        flags=check(audits)
        assert sum(v for (kind,_),v in flags.items() if kind=="stale-key")==1
        assert sum(v for (kind,_),v in flags.items() if kind=="stale-audit-entry")==1
    route=(P/"rules/sq2_decisions_E.py").read_text()
    assert '(C_ + "869", {"member.member_id": IN})' not in route
    for sibling in ('(C_ + "97",','(C_ + "925",','(C_ + "961",','(C_ + "968",'):
        assert sibling in route
