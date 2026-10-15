from copy import deepcopy
from pathlib import Path
import runpy
import pytest

P = Path(__file__).resolve().parents[1]
M = runpy.run_path(str(P / 'base/b7a_scoped_text_model.py'))
SOURCE = P.parents[1]
LN = M['LN']   # B7b: the bound sites' snapshot lines (B7a's in the model's comments)


def preimages():
    table = {'id_audit': {k: deepcopy(v) for k,v in M['AUDIT_PREIMAGES'].items() if v is not None},
             'lex_site_size': {}, 'site_total': {
                 'formation_guard.rs:239': {'bytes':'2*cases*N*59','why':'SQ2 G5: B7a exact formed guard owner: full independent generation; <=cases guards, N DOF rows, q<=12*l aggregate escaped source IDs'},
                 'formation_guard.rs:322': {'bytes':'2*cases*(N*(600+2+321)+(2*N+(10*128+4)*(12*l)))','why':'SQ2 G5: B7a exact formed guard owner: full independent generation; <=cases guards, N DOF rows, q<=12*l aggregate escaped source IDs'}}}
    loops = {'loops':[{'re':r'^iter:refs\.iter\(\)\.map$','bound':'4','why':'refs of one diagnostic <= 4 (vec! literals)'}]}
    return loops,table


def test_current_source_and_exact_transformation():
    loops,table=preimages(); old=deepcopy(table)
    current,proof=M['transform'](SOURCE,loops,table)
    assert table==old
    assert loops['loops'][0]['bound']=='4'
    assert M['state'](SOURCE,current,loops)==current
    assert proof['scoped_units_sha256']==M['UNITS_SHA']


@pytest.mark.parametrize('key',list(M['AUDIT_PREIMAGES']))
def test_every_authoritative_preimage_drift_refuses(key):
    loops,table=preimages();table['id_audit'][key]={'unknown':{'bytes':0}}
    with pytest.raises(ValueError,match='preimage changed'):M['transform'](SOURCE,loops,table)


@pytest.mark.parametrize('family',['site_zero','site_total','site_size','site_from','site_aggregate'])
def test_unit_competing_aliases_cannot_erase_copy(family):
    loops,table=preimages();current,_=M['transform'](SOURCE,loops,table)
    current.setdefault(family,{})['core/product_physics/src/lib.rs:%d:into_text'%LN['unit_message']]={'bytes':0}
    with pytest.raises(ValueError,match='competing override'):M['state'](SOURCE,current,loops)


def test_messages_refs_and_helper_delta_are_local_and_positive():
    loops,table=preimages();active,_=M['transform'](SOURCE,loops,table)
    row={'file':M['PP']+'pressure_runtime.rs','line':LN['diag'],'kind':'diag','message_arg':'message'}
    assert M['message'](active,row)==453
    assert M['refs'](active,row['file'],LN['diag'])==760
    assert M['owner_total'](active,M['PROBLEM'],[M['REF_LOOP']],{M['PROBLEM']:14000})==70000
    assert M['extra_requested'](active,M['PP']+'lib.rs',LN['suffix'],'replace',{M['PROBLEM']:14000})==14448000
    assert M['message'](active,dict(row,line=LN['diag']-1)) is None
    assert M['refs'](active,row['file'],LN['diag']-1) is None
    assert M['owner_total'](active,'other.rs:1:other',[M['REF_LOOP']],{M['PROBLEM']:14000}) is None
    assert M['extra_requested'](active,M['PP']+'lib.rs',LN['suffix_fn'],'replace',{M['PROBLEM']:14000})==0


def test_authoritative_unit_audit_and_lexical_class_agree():
    loops,table=preimages();active,_=M['transform'](SOURCE,loops,table)
    unit='lib.rs:%d'%LN['unit_message']
    assert active['id_audit'][M['PP']+unit]['message']['bytes']==600
    assert active['lex_site_size'][unit]['class']==600
    active['id_audit'][M['PP']+unit]['message']['bytes']=8192
    with pytest.raises(ValueError,match='current audit changed'):M['state'](SOURCE,active,loops)


@pytest.mark.parametrize('count',[None,0,-1])
def test_missing_or_nonpositive_owner_refuses(count):
    counts={} if count is None else {M['PROBLEM']:count}
    with pytest.raises(ValueError,match='missing/zero'):
        M['owner_total']({},M['PROBLEM'],[M['REF_LOOP']],counts)
    with pytest.raises(ValueError,match='missing/zero'):
        M['extra_requested']({},M['PP']+'lib.rs',LN['suffix'],'replace',counts)


@pytest.mark.parametrize('line',[239,322])
@pytest.mark.parametrize('family',['site_zero','site_from','site_size','site_aggregate'])
def test_range_competing_alias_refuses(line,family):
    loops,table=preimages();active,_=M['transform'](SOURCE,loops,table)
    active.setdefault(family,{})[M['PP']+'formation_guard.rs:'+str(line)+':format']={'bytes':0}
    with pytest.raises(ValueError,match='competing override'):M['state'](SOURCE,active,loops)


def test_consuming_global_ref_loop_drift_refuses():
    loops,table=preimages();active,_=M['transform'](SOURCE,loops,table)
    loops['loops'][0]['bound']='0'
    with pytest.raises(ValueError,match='consuming global ref loop'):M['state'](SOURCE,active,loops)


@pytest.mark.parametrize('where',[0,1])
def test_consuming_ref_loop_conflict_and_reordering_refuse(where):
    loops,table=preimages();active,_=M['transform'](SOURCE,loops,table)
    earlier={'re':r'^iter:refs','bound':'0','why':'counterfactual earlier shadow'}
    if where:
        loops['loops'].append(earlier)
        loops['loops'].insert(0,loops['loops'].pop())
    else:
        loops['loops'].insert(0,earlier)
    with pytest.raises(ValueError,match='priority changed'):M['state'](SOURCE,active,loops)


@pytest.mark.parametrize('line,kind',M['PROBLEM_SITES'])
def test_positive_problem_pre_and_post_witness(line,kind):
    expected=35 if line==LN['to_string'] else 7
    M['witness']({},M['PP']+'pressure_runtime.rs',line,kind,expected,{M['PROBLEM']:7})
    with pytest.raises(ValueError,match='witness changed'):
        M['witness']({},M['PP']+'pressure_runtime.rs',line,kind,0,{M['PROBLEM']:7})


def test_range_maxima_remain_and_every_copy_uses_same_aggregate():
    formed=runpy.run_path(str(P/'base/b7a_formation_model.py'))
    bound=formed['lengths'](32,128,128)
    assert bound['fired_total']==192*(600+2+321)+(2*192+(10*128+4)*1536)
    assert [bound[k] for k in ('reason','fired','joined','sentence')]==[165057,165659,993964,994321]
