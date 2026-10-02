#!/usr/bin/env python3
"""Small grammar controls only; these are not numerical solver observations."""
import json
from pathlib import Path
import sys
sys.dontwrite_bytecode = True
from validate_compare import validate

BASE = '\n'.join(['FORMAT\ta1-public-tsv-v1','CASE\tB01','STATUS\tSelected',
                  'SELECTED\t512\t1024','FLOOR\tForce\t0000000000000000',
                  'FLOOR\tMoment\t0000000000000000',
                  *('SCALE\t'+k+'\t0000000000000000' for k in ('Translation','Rotation','Force','Moment')),
                  'ROW\tD:0\tTranslation\tValue\t0000000000000000\tInputDerived\tnone'])+'\n'


def main(out):
    valid = {
        'selected_512': BASE,
        'selected_128': BASE.replace('512\t1024','128\t256').replace('FLOOR\tForce\t0000000000000000\n','').replace('FLOOR\tMoment\t0000000000000000\n',''),
        'unresolved': 'FORMAT\ta1-public-tsv-v1\nCASE\tB01\nSTATUS\tUnresolved\n',
    }
    bad = {
        'unknown_status': BASE.replace('STATUS\tSelected','STATUS\tNotAStatus'),
        'duplicate_status': BASE+'STATUS\tSelected\n',
        'duplicate_selected': BASE+'SELECTED\t512\t1024\n',
        'missing_selected': BASE.replace('SELECTED\t512\t1024\n',''),
        'malformed_selected_fields': BASE.replace('SELECTED\t512\t1024','SELECTED\t512'),
        'malformed_selected_precision': BASE.replace('SELECTED\t512\t1024','SELECTED\t512\t512'),
        'duplicate_floor': BASE+'FLOOR\tForce\t0000000000000000\n',
        'missing_floor': BASE.replace('FLOOR\tForce\t0000000000000000\n',''),
        'negative_floor': BASE.replace('FLOOR\tForce\t0000000000000000','FLOOR\tForce\tbff0000000000000'),
        'nonfinite_floor': BASE.replace('FLOOR\tForce\t0000000000000000','FLOOR\tForce\t7ff0000000000000'),
        'value_without_bits': BASE.replace('Value\t0000000000000000','Value\tnone'),
        'underflow_with_bits': BASE.replace('Value\t0000000000000000','Underflow\t0000000000000000'),
        'overflow_with_class': BASE.replace('Value\t0000000000000000','Overflow\tnone'),
        'noncanonical_bit_width': BASE.replace('Value\t0000000000000000','Value\t0'),
        'uppercase_bits': BASE.replace('Value\t0000000000000000','Value\t3FF0000000000000'),
        'nonfinite_row': BASE.replace('Value\t0000000000000000','Value\t7ff8000000000000'),
        'negative_zero_row': BASE.replace('Value\t0000000000000000','Value\t8000000000000000'),
        'unknown_outcome': BASE.replace('Value\t0000000000000000','Bogus\t0000000000000000'),
        'absolute_missing_bound': BASE.replace('InputDerived\tnone','AbsoluteVerified\tnone'),
        'negative_bound': BASE.replace('InputDerived\tnone','AbsoluteVerified\tbff0000000000000'),
        'duplicate_row': BASE+'ROW\tD:0\tTranslation\tValue\t0000000000000000\tInputDerived\tnone\n',
    }
    result = {'scope':'prevalidation grammar only; no claim of solver output or row completeness', 'valid':[], 'malformed':[]}
    for name,text in valid.items():
        validate(text)
        result['valid'].append({'control':name,'accepted':True})
    for name,text in bad.items():
        try:
            validate(text)
        except ValueError as exc:
            result['malformed'].append({'control':name,'rejected':True,'reason':str(exc)})
        else:
            raise AssertionError('malformed control was accepted: '+name)
    out.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'valid_accepted':len(result['valid']),'malformed_rejected':len(result['malformed'])}))


if __name__ == '__main__':
    main(Path(sys.argv[1]))
