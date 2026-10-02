#!/usr/bin/env python3
"""Small exact predicate boundaries and fixed-source interface checks; no solve."""
import hashlib
import json
from pathlib import Path
import sys
import bare_b_compare as c


def main(report):
    q=c.q;checks=[]
    def check(name,condition,details=None):
        assert condition,name
        checks.append({'name':name,'pass':True,'details':details})
    for name,error,bound,expected in [
        ('zero_exact',q.F(0),q.F(0),True),
        ('zero_bound_nonzero_error',q.H/2,q.F(0),False),
        ('subnormal_bound_equality',q.H,q.H,True),
        ('subnormal_bound_just_outside',q.H*(1+q.p2(-64)),q.H,False),
        ('normal_bound_equality',q.F(1),q.F(1),True),
        ('historical_multiplier_cannot_pass',1+q.p2(-23),q.F(1),False),
    ]:
        result=c.predicates(q.F(0),error,'AbsoluteVerified',bound)
        check(name,result['bare_b_pass']==expected,result)
    # These are rational predicate boundaries, not added primitive source cases.
    ae,af,_=c.relative_allowances(q.F(1),q.F(1))
    midpoint=(ae+af)/2
    result=c.predicates(q.F(1),1-midpoint,'RelativeVerified',scale=q.F(1))
    check('A_f64_smaller_only_exact_would_miss',af<midpoint<ae and result['sharper_exact_pass'] and not result['sharper_binary64_pass'],result)
    x=1+q.p2(-52);scale=1+q.p2(-42)
    ae,af,_=c.relative_allowances(x,scale);midpoint=(ae+af)/2
    result=c.predicates(x,x-midpoint,'RelativeVerified',scale=scale)
    check('A_exact_smaller_only_f64_would_miss',ae<midpoint<af and not result['sharper_exact_pass'] and result['sharper_binary64_pass'],result)
    result=c.predicates(q.F(1),1-q.F(1,10**9),'RelativeVerified',scale=q.p2(40))
    check('public_relative_exact_decimal_equality',result['public_pass'] and result['sharper_exact_pass'] and result['sharper_binary64_pass'],result)
    result=c.predicates(q.F(1),1-q.F(1,10**9)-q.p2(-120),'RelativeVerified',scale=q.p2(40))
    check('public_relative_just_outside',not result['public_pass'] and result['sharper_exact_pass'] and result['sharper_binary64_pass'],result)
    # One synthetic B01 publication exercises the strict full-source interface.
    cases,identities=c.fixed_basis();case=cases['B01'];identity=identities['B01']
    prefix=['FORMAT\ta1-public-tsv-v1','CASE\tB01','SOURCE_COMMIT\t'+'d01ad98a754698631f927709d08284c272de85e8',
            'LIMITS\t100000000\t100000000','SOURCE_ENCODING\t'+identity['source_encoding_hex'],
            'STIFFNESS_ENCODING\t'+identity['stiffness_encoding_hex']]
    lines=prefix+['STATUS\tSelected','SELECTED\t128\t256','IDENTITY\t'+c.METHOD+'\t'+c.POLICY+'\t'+c.FLOOR_RATIO,
                  'SOURCE_ENCODING_SELECTED\t'+identity['source_encoding_hex']]
    for r in case['rows']:
        lines.append('\t'.join(['LAYOUT',r['key'],r['kind'],'0',str(r['input_derived']).lower()]))
    for r in case['rows']:
        lines.append('\t'.join(['ROW',r['key'],r['kind'],r['direct_outcome'],r['direct_bits'] or 'none',r['direct_class'],r['direct_bound_bits'] or 'none']))
    lines+=['SCALE\t'+k+'\t'+b for k,b in case['direct_coupled_scale_bits'].items()]
    text='\n'.join(lines)+'\n'
    result,code=c.compare_text(text,hashlib.sha256(text.encode()).hexdigest())
    check('fixed_B01_synthetic_selected',code==0 and result['numeric_accuracy_pass'] and len(result['rows'])==37)
    rejected={
        'missing_source':text.replace('SOURCE_ENCODING\t'+identity['source_encoding_hex']+'\n',''),
        'wrong_source':text.replace('SOURCE_ENCODING\t'+identity['source_encoding_hex'],'SOURCE_ENCODING\t00'),
        'wrong_selected_source':text.replace('SOURCE_ENCODING_SELECTED\t'+identity['source_encoding_hex'],'SOURCE_ENCODING_SELECTED\t00'),
        'historical_policy':text.replace(c.POLICY,'M03-INTEGRITY-MP-v1'),
        'duplicate_selected':text+'SELECTED\t128\t256\n',
        'unknown_status':text.replace('STATUS\tSelected','STATUS\tUnknown'),
        'negative_scale':text.replace('SCALE\tTranslation\t'+case['direct_coupled_scale_bits']['Translation'],'SCALE\tTranslation\tbff0000000000000'),
        'nonfinite_value':text.replace('ROW\tD:0\tTranslation\tValue\t0000000000000000','ROW\tD:0\tTranslation\tValue\t7ff0000000000000'),
        'negative_zero_value':text.replace('ROW\tD:0\tTranslation\tValue\t0000000000000000','ROW\tD:0\tTranslation\tValue\t8000000000000000'),
    }
    p512=text.replace('SELECTED\t128\t256','SELECTED\t512\t1024')
    rejected['missing_floor']=p512
    rejected['negative_floor']=p512+'FLOOR\tForce\tbff0000000000000\nFLOOR\tMoment\t0000000000000000\n'
    rejected['duplicate_floor']=p512+'FLOOR\tForce\t0000000000000000\nFLOOR\tMoment\t0000000000000000\nFLOOR\tForce\t0000000000000000\n'
    for name,malformed in rejected.items():
        try:c.compare_text(malformed,hashlib.sha256(malformed.encode()).hexdigest())
        except ValueError as exc:check(name,True,str(exc))
        else:raise AssertionError('accepted malformed input: '+name)
    for status in ('Refused','Unresolved','SourceRefused'):
        text='\n'.join(prefix+['STATUS\t'+status])+'\n'
        result,code=c.compare_text(text,hashlib.sha256(text.encode()).hexdigest())
        check(status+'_not_accuracy_pass',code==3 and result['numeric_accuracy_pass'] is None and not result['rows'])
    text='\n'.join(prefix[:4]+['STATUS\tSourceRefused'])+'\n'
    try:c.compare_text(text,hashlib.sha256(text.encode()).hexdigest())
    except ValueError as exc:check('unbound_source_refusal_is_invalid',True,str(exc))
    else:raise AssertionError('accepted unbound source refusal')
    result={'scope':'Exact synthetic predicate boundaries and metadata around one existing B01 source; no new primitive cases or repaired outputs',
            'checks':checks,'count':len(checks),'pass':True}
    report.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'checks':len(checks),'pass':True}))


if __name__=='__main__':main(Path(sys.argv[1]))
