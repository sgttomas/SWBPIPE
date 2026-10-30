"""C7 scoped source and positive-owner proposal controls; no run-artifact inputs."""
import ast, collections, hashlib, json, os, re, runpy, subprocess, sys
from pathlib import Path
import pytest

PRICING=Path(__file__).resolve().parents[1]
SOURCE=Path(os.environ.get('RP_C7_SNAPSHOT',str(PRICING.parents[1])))
MODEL=runpy.run_path(str(PRICING/'base/prescribed_key_model.py'))


def test_positive_full_owner_formula():
    nodes=lambda n:0 if n<=0 else 1+(n-1)//5
    cap=lambda n:max(4,1<<(n-1).bit_length())
    result=MODEL['owned_formula'](3,192,1472,nodes,cap)
    assert result=={'Node((&str,&str),())':412,'s(NumericCaseBorrowed)':4,
      's(BTreeSetBorrowed)':4,'s(Vec)':3,'s(PairBorrowed)':4100,'s(NumericCaseResultBorrowed)':2}
    assert all(v>0 for v in result.values())


def test_source_known_representation():
    # Both current owned and proposed borrowed representations have exact pins.
    assert isinstance(MODEL['borrowed'](SOURCE),bool)


@pytest.mark.parametrize('change',['missing_owner','new_field','query_clone','missing_marker'])
def test_source_owner_query_changes_refuse(tmp_path,change):
    file=tmp_path/MODEL['FILE'];file.parent.mkdir(parents=True)
    source=(SOURCE/MODEL['FILE']).read_text()
    if change=='missing_owner':source=source.replace("struct NumericCase<'a>","struct RenamedCase<'a>",1)
    if change=='new_field':source=source.replace("struct NumericCase<'a> {","struct NumericCase<'a> {\n    extra:String,",1)
    if change=='query_clone':source=source.replace('prescribed.contains(&(', 'prescribed.contains(&(/* changed receiver */',1)
    if change=='missing_marker':source=source.replace('prescribed: BTreeSet<','prescribed: Vec<',1)
    file.write_text(source)
    with pytest.raises(ValueError):MODEL['borrowed'](tmp_path)


@pytest.mark.parametrize('owner',['case','combination'])
def test_duplicate_numeric_owner_refuses(tmp_path,owner):
    file=tmp_path/MODEL['FILE'];file.parent.mkdir(parents=True)
    source=(SOURCE/MODEL['FILE']).read_text()
    call={'case':'out.push(numeric_owner(source, c, s, vec![s], coverage)?);',
          'combination':'out.push(numeric_owner(source, e, s, data, coverage)?);'}[owner]
    assert source.count(call)==1
    source=source.replace(call,call+'\n        '+call,1)
    file.write_text(source)
    with pytest.raises(ValueError,match='owner count'):
        MODEL['borrowed'](tmp_path)


def test_existing_sets_and_global_ownership_not_changed():
    code=(PRICING/'base/g4_caps.py').read_text()
    assert '+ X({"Node((String,String),())": tree_nodes(k)}) + B(k * 2 * (ID + 8))' in code
    assert 'SETS = SETS + _c7' in code
    for name in ['"V4 G3-G6 working sets": SETS','"V5 G7 project + preview evidence": SUCC.tree() + SETS',
                 '"V6 G8 projected + invocation hash + model rebuild": ENVP.tree()']:
        assert name in code
    # New in-build borrowed types are distinct; no global owned atom rewrite.
    native=(PRICING/'base/g5_profile.py').read_text()
    assert '"s((String,String))": sz("(String, String)")' in native
    assert '"Node((String,String),())": node("(String, String)", "()")' in native
    ast.parse(native)


def _audit_namespace():
    rulefile=PRICING/'rules/o10_rules.py'
    code=rulefile.read_text()
    namespace={'_os':os,'_re':re,'_SNAP':str(SOURCE),'__file__':str(rulefile),
               'Resolved':str,'RE_':'core/reporting/result_export/src/','O10_ID_AUDIT':[]}
    definitions=[node for node in ast.parse(code).body
                 if isinstance(node,ast.FunctionDef) and node.name in {'fnkey','site'}]
    exec(compile(ast.Module(body=definitions,type_ignores=[]),str(rulefile),'exec'),namespace)
    # C7 controls are bounded before the independently controlled join and nonlinear rules.
    exec(compile(code[code.index('# C7A1:'):code.index('# ---- The pressure-source join')],str(rulefile),'exec'),namespace)
    return namespace


def test_five_replaced_audits_resolve_with_original_classes():
    namespace=_audit_namespace()
    audits=namespace['C7_INPUT_METADATA']
    assert namespace['O10_ID_AUDIT']==([] if MODEL['borrowed'](SOURCE) else audits)
    assert len(audits)==5 and len({key for key,_ in audits})==5
    assert [audit for _,audit in audits]==[
      {'text(&d[""][""])':{'bytes':128,'source':'IN128'},'text(&n[""])':{'bytes':128,'source':'IN128'}},
      {'text(&r[""])':{'bytes':1024,'source':'RES'}},
      {'text(&d[""])':{'bytes':128,'source':'IN128'}},
      {'text(&d[""])':{'bytes':128,'source':'STATIC'}},
      {'text(&row[""])':{'bytes':1024,'source':'RES'}}]
    source=(SOURCE/MODEL['FILE']).read_text().splitlines()
    for (key,_),(name,owned,borrowed,_) in zip(audits,namespace['_c7_audit_sites']):
        file,line=key.rsplit(':',1)
        assert file==MODEL['FILE']
        assert (borrowed if MODEL['borrowed'](SOURCE) else owned) in source[int(line)-1]
        body,_=MODEL['HELPERS']['_indented_function'](chr(10).join(source),name)
        assert source[int(line)-1] in body
    base=json.loads((PRICING/'base/text_args.g4.json').read_text())
    assert all(MODEL['FILE']+':'+str(n) not in base['id_audit'] for n in (3635,3644,4329,4330,4346))


def test_actual_g6_allocation_inventory_accepts_the_disposition(tmp_path):
    source=(SOURCE/MODEL['FILE']).read_text()
    graph={'edges':{},'spans':{}}
    for name in ('numeric_owner','g5c'):
        body,line=MODEL['HELPERS']['_indented_function'](source,name)
        key=MODEL['FILE']+':'+str(line)+':'+name
        graph['edges'][key]=[]
        start=source.index(body)
        graph['spans'][key]=[MODEL['FILE'],start,start+len(body)]
    edges=tmp_path/'edges.json';edges.write_text(json.dumps(graph))
    lex=tmp_path/'lexicon.json'
    subprocess.run([sys.executable,str(PRICING/'base/text_lexicon.py'),str(SOURCE),
                    str(edges),','.join(graph['edges']),str(lex)],check=True,capture_output=True,text=True)
    inv=json.loads(subprocess.check_output([sys.executable,str(PRICING/'base/template_inventory.py'),
                    '.',MODEL['FILE']],cwd=SOURCE,text=True))
    lexical=json.loads(lex.read_text())['rows']
    consumer=(PRICING/'base/text_budget.py').read_text()
    functions=[node for node in ast.parse(consumer).body
               if isinstance(node,ast.FunctionDef) and node.name in {'short','audit_key'}]
    begin=consumer.index('# G6 repair (RV87 SF-2): a site-keyed')
    end=consumer.index('# G7: a cycle',begin)
    def check(audits):
        ns={'re':re,'collections':collections,'inv':inv,'LEX':lexical,
            'TA':{'id_audit':dict(audits)},'ID_AUDIT':dict(audits),
            'unclassified':collections.defaultdict(int)}
        exec(compile(ast.Module(body=functions,type_ignores=[]),'actual G6 functions','exec'),ns)
        exec(compile(consumer[begin:end],'actual G6 inventory consumer','exec'),ns)
        return dict(ns['unclassified'])
    namespace=_audit_namespace()
    assert check(namespace['O10_ID_AUDIT'])=={}
    if MODEL['borrowed'](SOURCE):
        # The previously proposed allocation rows are positively rejected by
        # the actual consumer and actual source inventories, not hidden by zero.
        stale=check(namespace['C7_INPUT_METADATA'])
        assert sum(v for (kind,_),v in stale.items() if kind=='stale-key')==5
        assert sum(v for (kind,_),v in stale.items() if kind=='stale-audit-entry')==6
    else:
        assert len(namespace['O10_ID_AUDIT'])==5
