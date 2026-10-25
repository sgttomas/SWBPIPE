"""Selected publication hash source, caller and positive-owner boundaries."""
from pathlib import Path
from copy import deepcopy
import os
import json
import runpy
import shutil
import pytest

PRICING=Path(__file__).resolve().parents[1]
SOURCE=Path(os.environ.get('B7A_PHASE_TEST_SOURCE',str(PRICING.parents[1])))
MODEL=runpy.run_path(str(PRICING/'base/b7a_phase_model.py'))
# T4-U6/U8 snapshot lines, unchanged text (B7b: composite 724/950, source_receipt 646/732/748, lib 2532; B7a:
# composite 726/952, source_receipt 694/780/796, lib 2543; T4-U6/U8 433fc1f1e4: composite 731/982; T4-U6/U8 final:
# composite 977, lib 2553; T4-S5 epoch 4: composite 990, lib 2569; pre-0.4 unification: source_receipt +4, lib 2634; W-8: lib 2635).
CALLERS={
 'core/product_physics/src/source_receipt/composite.rs:740:composite_exact':MODEL['SOLVE'],
 'core/product_physics/src/source_receipt/composite.rs:1018:ordinary_physics':MODEL['SOLVE'],
 'core/product_physics/src/source_receipt.rs:661:failed':MODEL['SOLVE'],
 'core/product_physics/src/source_receipt.rs:747:finalize_composite':'core/product_physics/src/lib.rs:2705:run_linear_static_preview_observed_inner',
 'core/product_physics/src/source_receipt.rs:763:finalize_for':'core/product_physics/src/source_receipt.rs:747:finalize_composite',
}

def inputs(tmp_path,route='L'):
    loop={'loops':[],'loop_total':[deepcopy(MODEL['LOOP_RULE'])]}
    text={'site_total':{f'formation_guard.rs:{k}':{'bytes':v,'why':MODEL['TOTAL_WHY']} for k,v in MODEL['TOTALS'].items()}}
    loop,text,_=MODEL['transform'](SOURCE,route,loop,text)
    for name,data in [('loop_bounds.g4.json',loop),('text_args.g4.json',text)]:
        (tmp_path/name).write_text(json.dumps(data))
    graph={'edges':{key:[] for key in set(CALLERS)|set(CALLERS.values())},'site_loops':[]}
    for target,caller in CALLERS.items():
        graph['edges'][caller].append(target)
        graph['site_loops'].append([caller,target,[[]]])
    return graph

@pytest.mark.parametrize('route',['L','E'])
def test_exact_source_and_full_constructor_finalizer_inventory(tmp_path,route):
    assert MODEL['selected_publication_context'](SOURCE,tmp_path,inputs(tmp_path,route))['route']==route

@pytest.mark.parametrize('target',list(CALLERS))
@pytest.mark.parametrize('mutation',['new_caller','duplicate','stack','orphan_stack','missing_stack'])
def test_every_selected_parent_cut_refuses_drift(tmp_path,target,mutation):
    graph=inputs(tmp_path)
    if mutation=='new_caller':graph['edges']['unknown.rs:1:new_caller']=[target]
    if mutation=='duplicate':graph['edges'][CALLERS[target]].append(target)
    if mutation=='stack':next(r for r in graph['site_loops'] if r[1]==target)[2]=[['for call in calls']]
    if mutation=='orphan_stack':graph['site_loops'].append(['unknown.rs:1:new_caller',target,[[]]])
    if mutation=='missing_stack':graph['site_loops']=[r for r in graph['site_loops'] if r[1]!=target]
    with pytest.raises(ValueError):MODEL['selected_publication_context'](SOURCE,tmp_path,graph)

def test_route_and_context_label_drift_refuse(tmp_path):
    graph=inputs(tmp_path)
    path=tmp_path/'text_args.g4.json'
    table=json.loads(path.read_bytes());table[MODEL['MARKER']]['route']='E'
    path.write_text(json.dumps(table))
    with pytest.raises(ValueError,match='context/route/count drift'):
        MODEL['selected_publication_context'](SOURCE,tmp_path,graph)

@pytest.mark.parametrize('file,fragment',[
 ('core/product_physics/src/lib.rs',b'captured && exact && !nonlinear && !combinations'),
 ('core/product_physics/src/lib.rs',b'(capture, pressure_runtime::is_exact(model))'),
 ('core/product_physics/src/lib.rs',b'(source_selected, composite_context)'),
 ('core/product_physics/src/source_receipt.rs',b'capture.composite_context()?.receipt_policy()'),
])
def test_each_exact_same_capture_gate_source_change_refuses(tmp_path,file,fragment):
    domain=runpy.run_path(str(PRICING/'base/b7a_domain_model.py'))
    formation=runpy.run_path(str(PRICING/'base/b7a_formation_model.py'))
    scoped=runpy.run_path(str(PRICING/'base/b7a_scoped_text_model.py'))
    files=set(domain['SOURCE_FILES'])|set(formation['SOURCE_FILES'])|{scoped['UNITS'],'core/product_physics/src/retained_memory.rs'}
    for relative in files:
        destination=tmp_path/relative;destination.parent.mkdir(parents=True,exist_ok=True)
        shutil.copyfile(SOURCE/relative,destination)
    path=tmp_path/file;raw=path.read_bytes();assert fragment in raw
    path.write_bytes(raw.replace(fragment,fragment+b'/* changed selected gate */',1))
    with pytest.raises(ValueError):MODEL['inspect'](tmp_path)
