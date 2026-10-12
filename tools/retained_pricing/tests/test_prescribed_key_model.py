"""C7 scoped source and positive-owner proposal controls; no run-artifact inputs."""
import ast, hashlib, os, runpy
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
