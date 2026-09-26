"""V3 step 1: derive every case on the intended and the represented inputs, from inputs and topology only.

Usage: python3 v3_derive.py <references_eload.json> <out v3_values.json>
Reads each case's `inputs` and `model` (topology; derived model fields are only asserted against).
Does not read `expected`, `classes`, `zero_valued`, `negative_controls` or `cancellation`.
Also verifies PI against an independent Gauss-Legendre (AGM) evaluation at 200 digits.
"""
import json
import sys
from decimal import Decimal, getcontext, localcontext
import v3_core as V


def agm_pi(prec):
    with localcontext() as c:
        c.prec = prec + 10
        a, b, t, p = Decimal(1), Decimal(1) / Decimal(2).sqrt(), Decimal(1) / 4, Decimal(1)
        for _ in range(12):
            an = (a + b) / 2
            b = (a * b).sqrt()
            t -= p * (a - an) ** 2
            a = an
            p *= 2
        return (a + b) ** 2 / (4 * t)


def main():
    ref = json.load(open(sys.argv[1]))
    getcontext().prec = 80
    with localcontext() as c:
        c.prec = 210
        d = abs(Decimal(V.PI.numerator) / Decimal(V.PI.denominator) - agm_pi(200))
    out = {'pi': {'digits': V.PI_DIGITS, 'abs_diff_vs_gauss_legendre_200': format(d, '.3e')}, 'cases': {}}
    for case in ref['cases']:
        rec = {}
        for basis in ('int', 'rep'):
            pb, sol, pubd = V.run(case, basis)
            rec[basis] = {k: format(V.to_dec(v, 70), '.60e') for k, v in pubd.items()}
            if basis == 'int':
                rec['derived'] = {
                    'eloads_global_intensity': {el['key']: [str(x) if x.denominator < 10 ** 12 else format(
                        Decimal(x.numerator) / Decimal(x.denominator), '.40e') for x in el['q']] for el in pb.eloads},
                    'eps': {m: str(v) for m, v in pb.eigen.items()},
                    'Fp_over_pi': {m: str(v / V.PI) for m, v in pb.thrust.items()},
                }
        out['cases'][case['id']] = rec
        print(case['id'], len(rec['int']), 'values; equilibrium exact on both bases')
    json.dump(out, open(sys.argv[2], 'w'), indent=0)
    print('pi check', out['pi'])


if __name__ == '__main__':
    main()
