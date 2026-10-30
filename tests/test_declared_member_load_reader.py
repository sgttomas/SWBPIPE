"""Reader arithmetic against maintained third-oracle vectors, never producer calls."""
from fractions import Fraction
from pathlib import Path
import pytest
from tests.verifiers import declared_member_load as reader

ROOT = Path(__file__).resolve().parents[1]


def token(word):
    if word in ("Z+", "Z-"):
        return Fraction()
    significand, exponent = word[1:].split("p")
    exponent = int(exponent) + 1 - 4 * len(significand)
    n = int(significand, 16)
    result = Fraction(n << exponent) if exponent >= 0 else Fraction(n, 1 << -exponent)
    return -result if word[0] == "-" else result


def vectors():
    source = None
    for line in (ROOT / "core/solver/frame_kernel/tests/retained_k4/declared.txt").read_text().splitlines():
        row = line.split()
        if not row:
            continue
        if row[0] == "case":
            name = row[1]
            source = {"id_maps":{"nodes":[{"coordinates":row[2:5]},{"coordinates":row[5:8]}],"members":[{"node_i":0,"node_j":1}]},"nodal_terms":[],"member_load_terms":[]}
            entries = []
        elif row[0] == "load":
            source["member_load_terms"].append({"member":0,"intensity":row[1:4]})
        elif row[0] == "entries":
            entries.append(list(map(token,row[1:])))
        elif row[0] == "nodal":
            dof = int(row[1])
            source["nodal_terms"].append({"dof":{"node":dof//6,"component":reader.COMPONENTS[dof%6]},"value":row[2]})
        elif row[0] == "ledger":
            yield name, source, entries, row[2]


@pytest.mark.parametrize("name,source,entries,digest",list(vectors()))
def test_independent_declared_entries_and_ledger(name,source,entries,digest):
    nodes = source["id_maps"]["nodes"]
    for load, expected in zip(source["member_load_terms"],entries):
        assert reader.declared_entries(nodes[0]["coordinates"],nodes[1]["coordinates"],load["intensity"]) == expected, name
    assert reader.recompute(source)[0] == digest, name


def test_cancelled_contributions_and_zero_occurrence_inventory():
    from copy import deepcopy
    _, source, _, _ = next(vectors())
    source = deepcopy(source)
    reverse = deepcopy(source["member_load_terms"][0])
    reverse["intensity"][1] = "c008000000000000"
    source["member_load_terms"].append(reverse)
    digest, data = reader.recompute(source)
    assert data == {1,5,7,11}
    source["member_load_terms"] = [{"member":0,"intensity":["0000000000000000"]*3}]
    zero_digest, data = reader.recompute(source)
    assert not data and zero_digest == digest  # 12 occurrence DOFs, exact canonical zero nets
    source["nodal_terms"].append({"dof":{"node":0,"component":"UX"},"value":"8000000000000000"})
    assert reader.recompute(source)[0] == zero_digest


def test_reader_refuses_nonfinite_and_outside_bounded_domain():
    from copy import deepcopy
    _, source, _, _ = next(vectors())
    source = deepcopy(source)
    source["member_load_terms"][0]["intensity"][1] = "7ff0000000000000"
    with pytest.raises(ValueError): reader.recompute(source)
    source["member_load_terms"] *= 129
    with pytest.raises(ValueError): reader.recompute(source)


def _binding_fixture():
    from tests.verifiers import retained_precision as rp
    _, source, _, digest = next(vectors())
    source["member_load_terms"][0].update(constructor_ordinal=0,primitive_load_index=0,source_id="l")
    case={"id":"c","primitive_loads":[{"id":"l","target":{"type":"element","pipe":"p"},"category":"distributed_force","dimension":"force_per_length","direction":"global_y","magnitude":{"value":3,"unit":"N/m"}}]}
    return rp,source,case,[{"id":"p"}],digest


def test_selected_member_case_binds_same_invocation_declared_problem_and_digest():
    from copy import deepcopy
    rp,source,case,pipes,digest = _binding_fixture()
    rp._g8_member_terms(source,case,pipes,selected_digest=digest)
    with pytest.raises(rp.RetainedPrecisionError) as error:
        rp._g8_member_terms(source,case,pipes,selected_digest="0"*64)
    assert (error.value.gate,error.value.code)==("G8","RETAINED_PRECISION_PREPARATION_MISMATCH")
    for mutation in ("duplicate","omit","ordinal","intensity","other_case"):
        altered,other = deepcopy(source),deepcopy(case)
        if mutation=="duplicate": altered["member_load_terms"]*=2
        elif mutation=="omit": del altered["member_load_terms"]
        elif mutation=="ordinal": altered["member_load_terms"][0]["constructor_ordinal"]=1
        elif mutation=="intensity": altered["member_load_terms"][0]["intensity"][0]="3ff0000000000000"
        else: other["primitive_loads"][0]["magnitude"]["value"]=-3
        with pytest.raises(rp.RetainedPrecisionError): rp._g8_member_terms(altered,other,pipes,selected_digest=digest)


def test_member_case_declines_and_nodal_only_exclusions():
    from copy import deepcopy
    rp,source,case,pipes,_ = _binding_fixture()
    with pytest.raises(rp.RetainedPrecisionError): rp._g8_member_terms(source,case,pipes,exact=True)
    for mutation in ("partial","generated"):
        other=deepcopy(case)
        if mutation=="partial": other["primitive_loads"][0]["extent"]=[0,1]
        else: other["primitive_loads"][0]["id"]="load:self-weight:1:c:1:p"
        with pytest.raises(rp.RetainedPrecisionError): rp._g8_member_terms(source,other,pipes)
    # No new ledger arithmetic on the established nodal-only path.
    case["primitive_loads"]=[]
    with pytest.raises(rp.RetainedPrecisionError): rp._g8_member_terms(source,case,pipes)
    del source["member_load_terms"]
    rp._g8_member_terms(source,case,pipes,selected_digest="attested nodal ledger unchanged")


def _exhausted_fixture():
    rp,source,_,_,_=_binding_fixture()
    source['owner']={'kind':'case'}
    source['id_maps']['members'][0].update(id='p',kernel_member=0)
    owner={'basis_ref':{'ref_type':'load_case','ref_id':'c'},'status':'unavailable'}
    zero={'negative':False,'exponent':0,'limbs':['0000000000000000']*16}
    upper={'negative':False,'exponent':0,'limbs':['0000000000000000']*15+['8000000000000000']}
    a={'owner_ref':{'kind':'case'},'source_ref':0,'stages':{'proof_start':'completed','projection':'failed',**{k:'not_entered' for k in ('maxima','values','aliases','certificate','observables','g5a')}},'result':{'kind':'unavailable','error':{'kind':'proof','cause':{'kind':'association','detail':'loaded maximum search exhausted'}}},'proof':{'loaded_maximum_diagnostic':{'row':0,'member':0,'lower':zero,'upper':upper}}}
    row={'basis_ref':owner['basis_ref'],'entity_ref':'p','kind':'pipe_elastic_normal_stress_maximum_v2','unit':'MPa','value':1.0}
    return rp,{'sources':[source]},a,owner,[row]


def test_loaded_maximum_projection_uses_exact_activity_and_si_publication():
    from copy import deepcopy
    rp,body,a,owner,rows=_exhausted_fixture();s=body['sources'][0];row=rows[0]
    assert rp._owner_projected(owner,row,s)
    assert not rp._owner_projected(owner,row,s,None)  # no route-specific maximum without a route
    assert rp._projected_publication_value(row,{'kind':'normal','value':rp.bits(1_000_000.)})==1.0
    opposite=deepcopy(s['member_load_terms'][0]);opposite['intensity'][1]='c008000000000000';s['member_load_terms'].append(opposite)
    assert rp._owner_projected(owner,row,s)  # cancelling authored activity remains loaded
    assert not rp._owner_projected(owner,row,s,rp.EXACT_ROUTE)
    assert not rp._owner_projected({'basis_ref':{'ref_type':'combination'}},row,s)
    for t in s['member_load_terms']: t['intensity']=['8000000000000000','0000000000000000','0000000000000000']
    assert not rp._owner_projected(owner,row,s)
    assert not rp._owner_projected(owner,row)


def test_exhaustion_diagnostic_required_associated_and_fail_closed():
    from copy import deepcopy
    rp,body,a,owner,rows=_exhausted_fixture()
    rp._loaded_maximum_diagnostic(body,a,owner,rows,rp.PREVIEW_ROUTE)
    for fault in ('missing','cause','ready','row','member','later','route','nodal','combination'):
        b,x=deepcopy(body),deepcopy(a);route=rp.PREVIEW_ROUTE
        if fault=='missing':del x['proof']['loaded_maximum_diagnostic']
        elif fault=='cause':x['result']['error']['cause']['detail']='other failure'
        elif fault=='ready':x['result']['kind']='ready'
        elif fault in ('row','member'):x['proof']['loaded_maximum_diagnostic'][fault]=1
        elif fault=='later':x['stages']['maxima']='completed'
        elif fault=='route':route=rp.EXACT_ROUTE
        elif fault=='nodal':del b['sources'][0]['member_load_terms']
        else:x['owner_ref']['kind']='combination'
        with pytest.raises(rp.RetainedPrecisionError):rp._loaded_maximum_diagnostic(b,x,owner,rows,route)


def test_endpoint_encoding_and_semantic_canonical_order_are_separate():
    from copy import deepcopy
    rp,body,a,owner,rows=_exhausted_fixture();endpoint=a['proof']['loaded_maximum_diagnostic']['upper'];spec=rp._schema()['$defs']['LoadedMaximumEndpoint']
    for value in (.5,2**53,-0.0):
        v=deepcopy(endpoint);v['exponent']=value
        with pytest.raises(rp.RetainedPrecisionError) as error:rp._encoding(v,spec)
        assert error.value.gate=='G2'
    for word in ('abcdef','FFFFFFFFFFFFFFFF','000000000000000g'):
        v=deepcopy(endpoint);v['limbs'][0]=word
        with pytest.raises(rp.RetainedPrecisionError):rp._encoding(v,spec)
    for fault in ('negative','zero_exp','unnormalized','reversed'):
        x=deepcopy(a);d=x['proof']['loaded_maximum_diagnostic']
        if fault=='negative':d['lower']['negative']=True
        elif fault=='zero_exp':d['lower']['exponent']=1
        elif fault=='unnormalized':d['upper']['limbs'][15]='4000000000000000'
        else:d['lower']=deepcopy(endpoint);d['lower']['exponent']=1
        with pytest.raises(rp.RetainedPrecisionError):rp._loaded_maximum_diagnostic(body,x,owner,rows,rp.PREVIEW_ROUTE)
    x=deepcopy(a);x['proof']['loaded_maximum_diagnostic']['upper']['exponent']=rp.SAFE
    rp._loaded_maximum_diagnostic(body,x,owner,rows,rp.PREVIEW_ROUTE)


def test_early_member_guard_precedes_hash_and_mixed_nodal_staging():
    rp,body,_,_,_=_exhausted_fixture()
    invocation={'request':{'model':{'load_cases':[{'id':'c','primitive_loads':[{'target':{'type':'node'}}]*128+[{'target':{'type':'element'}}]}]}}}
    with pytest.raises(rp.RetainedPrecisionError):rp._guard_member_g8(body,invocation)
    invocation['request']['model']['load_cases'][0]['primitive_loads'][-1]['target']['type']='node'
    del body['sources'][0]['member_load_terms']
    rp._guard_member_g8(body,invocation)  # old nodal admission is unchanged


def test_early_member_guard_scope_preserves_namespace_and_source_forgery_bounds(monkeypatch):
    from copy import deepcopy
    from tests.verifiers import retained_precision as rp
    body={"sources":[],"invocation":{"value":"0"*64}}
    invocation={"request":{"model":{"schema_version":"0.8.0","load_cases":[{"id":"c","primitive_loads":
        [{"target":{"type":"node"}}]*128+[{"target":{"type":"element"},"category":"distributed_force","dimension":"force_per_length"}]}]}},"solver_mode":"sparse_interactive"}
    def no_hash(*args,**kwargs):raise AssertionError("hash reached")
    monkeypatch.setattr(rp,"_hash",no_hash)
    def refuses_before_hash(b,inv,exact=False):
        with pytest.raises(rp.RetainedPrecisionError) as error:rp._g8(b,{},inv,rp.EXACT_ROUTE if exact else rp.PREVIEW_ROUTE)
        assert (error.value.gate,error.value.code)==("G8","RETAINED_PRECISION_PREPARATION_MISMATCH")
    refuses_before_hash(body,invocation)  # source member key omitted
    malformed=deepcopy(invocation)
    malformed['request']['model']['load_cases'][0]['primitive_loads'][-1].update(direction='ux',extent=[0,1],magnitude={'value':1,'unit':'wrong-unit'})
    refuses_before_hash(body,malformed)
    legacy=deepcopy(invocation)
    legacy['request']['model'].update(schema_version='0.3.0',pressure_contract={'version':'1.0.0','mode':'legacy_pressure_v1'})
    pressure=deepcopy(invocation)
    pressure['request']['model']['load_cases'][0]['primitive_loads'][-1].update(category='pressure',dimension='pressure')
    nodal=deepcopy(invocation);nodal['request']['model']['load_cases'][0]['primitive_loads'][-1]['target']['type']='node'
    for inv,exact in ((legacy,False),(pressure,False),(nodal,False),(invocation,True)):
        with pytest.raises(AssertionError,match='hash reached'):rp._g8(body,{},inv,rp.EXACT_ROUTE if exact else rp.PREVIEW_ROUTE)
    for marker in (None,[]):
        forged={"sources":[{"member_load_terms":marker}],"invocation":{"value":"0"*64}}
        refuses_before_hash(forged,legacy)
        refuses_before_hash(forged,invocation,True)
        refuses_before_hash(forged,nodal)


def test_early_member_guard_borrowed_views_match_rust_json_types():
    from copy import deepcopy
    from tests.verifiers import retained_precision as rp
    invocation={"request":{"model":{"schema_version":"0.8.0","load_cases":[{"id":"c","primitive_loads":[]}] }},"solver_mode":"sparse_interactive"}
    variants=[[],{"request":None},{"request":{"model":None}}]
    for key,value in (("load_cases",None),("load_cases",[None]),("pipe_segments",[None]),("pipe_segments",[{"id":[]}]),("materials",{})):
        inv=deepcopy(invocation);inv['request']['model'][key]=value;variants.append(inv)
    for key,value in (("id",[]),("primitive_loads",None),("primitive_loads",[None]),("primitive_loads",[{"target":None}])):
        inv=deepcopy(invocation);inv['request']['model']['load_cases'][0][key]=value;variants.append(inv)
    for sources in ([],[{"member_load_terms":None}],[{"member_load_terms":[]}]):
        body={'sources':sources,'invocation':{'value':'0'*64}}
        for inv in variants:
            for exact in (False,True):
                rp._guard_member_g8(body,inv,exact)
                with pytest.raises(rp.RetainedPrecisionError) as error:rp._g8(body,{},inv,rp.EXACT_ROUTE if exact else rp.PREVIEW_ROUTE)
                assert (error.value.gate,error.value.code)==('G8','RETAINED_PRECISION_INVOCATION_MISMATCH')


def test_genuine_member_product_and_all_attested_hash_forgery():
    import os,json
    from copy import deepcopy
    from tests.verifiers import retained_precision as rp
    directory=os.environ.get('W1_MEMBER_READER_INPUT_DIR')
    if directory is None:pytest.skip('requires maintained product writer outputs; shared corpus remains the always-on consumer')
    for mode in ('sparse_interactive','dense_scrutiny'):
        record=json.loads((Path(directory)/f'w1_member_load_{mode}.json').read_text())
        source,invocation=record['source'],record['invocation']
        assert rp.validate_retained_precision(source,invocation)['numerical_eligible']
        assert rp.validate_retained_precision(source)['standing']=='needs_recompute'
        mutant=deepcopy(source)
        def forge(value):
            if isinstance(value,dict):
                for k,v in value.items():
                    if k=='ledger_sha256':value[k]='0'*64
                    else:forge(v)
            elif isinstance(value,list):
                for v in value:forge(v)
        forge(mutant['retained_precision']['body'])
        mutant['retained_precision']['receipt_sha256']=rp._hash('retained_precision_receipt_mp_v2',mutant['retained_precision']['body'])
        with pytest.raises(rp.RetainedPrecisionError) as error:rp.validate_retained_precision(mutant,invocation)
        assert (error.value.gate,error.value.code)==('G8','RETAINED_PRECISION_PREPARATION_MISMATCH')


def test_genuine_exhaustion_and_diagnostic_encoding_association_mutants():
    """The genuine-exhaustion control's writer outputs, read and mutated.

    Since the W1 coverage repair (DEL-04-08 w1-coverage-repair.md section 1.4, option 2) the
    control is hook-driven: its writer runs only with product_physics'
    --features loaded-maximum-hooks and drives a real member to exhaustion at reduced search
    limits; it claims no production limit.
    """
    import os,json
    from copy import deepcopy
    from tests.verifiers import retained_precision as rp
    directory=os.environ.get('W1_MEMBER_READER_INPUT_DIR')
    if directory is None:pytest.skip('requires the maintained, hook-driven product exhaustion writer outputs (--features loaded-maximum-hooks)')
    for mode in ('sparse_interactive','dense_scrutiny'):
        record=json.loads((Path(directory)/f'w1_member_exhaustion_{mode}.json').read_text());source=record['source'];invocation=record['invocation']
        result=rp.validate_retained_precision(source,invocation)
        assert result['invocation_bound'] and not result['numerical_eligible']
        ai=next(i for i,a in enumerate(source['retained_precision']['body']['product_attempts']) if 'loaded_maximum_diagnostic' in (a['proof'] or {}))
        for fault in ('missing','cause','member','row','negative','zero_exp','unnormalized','reversed','fractional','limbcase'):
            mutant=deepcopy(source);a=mutant['retained_precision']['body']['product_attempts'][ai];d=a['proof']['loaded_maximum_diagnostic']
            if fault=='missing':del a['proof']['loaded_maximum_diagnostic']
            elif fault=='cause':a['result']['error']['cause']['detail']='other failure'
            elif fault=='member':d['member']=31
            elif fault=='row':d['row']=99999
            elif fault=='negative':d['lower']['negative']=True
            elif fault=='zero_exp':d['lower']['limbs']=['0000000000000000']*16;d['lower']['exponent']=1
            elif fault=='unnormalized':d['upper']['limbs'][15]='4000000000000000'
            elif fault=='reversed':d['lower']=deepcopy(d['upper']);d['lower']['exponent']+=1
            elif fault=='fractional':d['upper']['exponent']=.5
            else:d['upper']['limbs'][0]='FFFFFFFFFFFFFFFF'
            mutant['retained_precision']['receipt_sha256']=rp._hash('retained_precision_receipt_mp_v2',mutant['retained_precision']['body'])
            with pytest.raises(rp.RetainedPrecisionError) as error:rp.validate_retained_precision(mutant,invocation)
            assert error.value.gate==('G2' if fault in ('fractional','limbcase') else 'G5'),(fault,error.value.gate,error.value.code)
