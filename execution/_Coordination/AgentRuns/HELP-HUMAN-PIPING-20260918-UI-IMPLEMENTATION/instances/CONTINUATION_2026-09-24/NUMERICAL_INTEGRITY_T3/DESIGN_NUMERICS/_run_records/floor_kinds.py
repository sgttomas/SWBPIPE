#!/usr/bin/env python3
"""D1 DESIGN revision 3, F2: stop-rule floor with twist and extension kinds (standard library only).

Usage: [FLOOR_R=2^-34] python3 floor_kinds.py <references.json> <v2_compare.json> <out.json>
Without FLOOR_R, R = 10^9 * 2^-64 (V2's constant); with FLOOR_R=2^-34, DESIGN revision 3's constant.
Reads R1's references (not edited) and V2's per-case L_b (v2_compare.json), and counts, per family,
the comparisons whose scale max(|exp|, scale) falls below R*S*, for:
  B  V2's variant B: twist and extension are kinds with S* = their own largest magnitude;
  C  twist S* = max(S(twist), S*(moment) * max(L/GJ)),
     extension S* = max(S(extension), S*(force) * max(L/EA)), maxima over the case's members;
  D  as C with the minimum member flexibility min(L/GJ), min(L/EA);
  E  as C with each member's own flexibility (per-member floor);
  F  the rule adopted in DESIGN revision 3: twist and extension are derived from the published
     torque and axial force, so S*_twist(m) = S*(moment) * (L/GJ)_m and
     S*_extension(m) = S*(force) * (L/EA)_m, with no own-magnitude term;
     translation, rotation, force and moment as DESIGN.md section 4.1.6.
For the seven generated models (n = 10000 and one RF-MECH case) the member length is bounded by
L_b, which can only raise the coupled floor (more flags, never fewer).
S(kind) is taken from R1's published expected rows, not from a complete solution. For cases with
n >= 1000 R1 publishes sampled positions only, so S there is a lower bound; variant B is recomputed
the same way so that B and C are compared on one basis, and B is checked against V2's counts.
RF-CANCEL rows are also compared on the recommended (net-governed) scale, per ROOT's ruling (F_rec).
"""
import json
import sys
from decimal import Decimal as D, getcontext
getcontext().prec = 60
getcontext().Emin = -999999
import os
R = D(2) ** -34 if os.environ.get("FLOOR_R") == "2^-34" else D(2) ** -64 / D("1e-9")


PI = D('3.14159265358979323846264338327950288419716939937510582097494459')


def num(x):
    """R1's exact number strings: decimal, p/q, or m*2^k."""
    x = str(x)
    if '*2^' in x:
        m, k = x.split('*2^')
        return D(m) * D(2) ** int(k)
    if '/' in x:
        a, b = x.split('/')
        return D(a) / D(b)
    return D(x)


def base(c):
    return c.split('@')[0]


def main():
    ref = json.load(open(sys.argv[1]))['cases']
    cmpr = json.load(open(sys.argv[2]))['cases']
    out = {'by_family': {}, 'cases': {}}
    for cid, c in ref.items():
        if not c.get('expected'):
            continue
        fam = c['family']
        Lb = D(cmpr[cid]['L_b'])
        secs = c['model']['sections']
        phi_t = phi_e = D(0)
        phin_t = phin_e = None
        phi_m = {}
        if 'members' in c['model']:
            nodes = {k: [num(x) for x in v] for k, v in c['model']['nodes_m'].items()}
            members = [(sum((p - q) ** 2 for p, q in zip(nodes[m[1]], nodes[m[2]])).sqrt(), m[3])
                       for m in c['model']['members']]
        else:  # generated models (n = 10000): member length bounded by L_b, which is conservative here
            members = [(Lb, k) for k in secs]
        for L, sid in members:
            s = secs[sid]
            od, idd = num(s['OD']), num(s['ID'])
            ea = num(s['E']) * PI * (od * od - idd * idd) / 4
            gmod = num(s['G']) if 'G' in s else num(s['E']) / (2 * (1 + num(s['nu'])))
            gj = gmod * PI * (od ** 4 - idd ** 4) / 32
            phi_t, phi_e = max(phi_t, L / gj), max(phi_e, L / ea)
            phin_t = L / gj if phin_t is None else min(phin_t, L / gj)
            phin_e = L / ea if phin_e is None else min(phin_e, L / ea)
            if 'members' in c['model']:
                pass
        if 'members' in c['model']:
            nodes2 = {k: [num(x) for x in v] for k, v in c['model']['nodes_m'].items()}
            for m in c['model']['members']:
                sm = secs[m[3]]
                L = sum((p - q) ** 2 for p, q in zip(nodes2[m[1]], nodes2[m[2]])).sqrt()
                od, idd = num(sm['OD']), num(sm['ID'])
                gmod = num(sm['G']) if 'G' in sm else num(sm['E']) / (2 * (1 + num(sm['nu'])))
                phi_m[m[0]] = (L / (gmod * PI * (od ** 4 - idd ** 4) / 32), L / (num(sm['E']) * PI * (od * od - idd * idd) / 4))
        S = {}
        for key, v, cls in (r[:3] for r in c['expected']):
            S[base(cls)] = max(S.get(base(cls), D(0)), abs(D(v)))
        g = lambda k: S.get(k, D(0))
        tr = max(g('translation'), Lb * g('rotation'))
        ro = max(g('rotation'), g('translation') / Lb)
        fo = max(g('force'), g('moment') / Lb)
        mo = max(g('moment'), Lb * g('force'))
        SB = {'translation': tr, 'rotation': ro, 'force': fo, 'moment': mo, 'twist': g('twist'), 'extension': g('extension')}
        SC = dict(SB)
        SC['twist'] = max(g('twist'), mo * phi_t)
        SC['extension'] = max(g('extension'), fo * phi_e)
        SD = dict(SB)
        SD['twist'] = max(g('twist'), mo * phin_t)
        SD['extension'] = max(g('extension'), fo * phin_e)
        scales = {k: D(v['value']) for k, v in c['scales'].items()}
        r = {'B_scale_below': [], 'C_scale_below': [], 'D_scale_below': [], 'E_scale_below': [], 'F_scale_below': [], 'F_rec_scale_below': []}
        for row in c['expected']:
            key, e, cls = row[0], D(row[1]), row[2]
            k = base(cls)
            sc = max(abs(e), scales[cls])
            SE = dict(SB)
            mem = key.split('.')[1] if key.count('.') >= 1 else None
            if k in ('twist', 'extension') and mem in phi_m:
                SE['twist'] = max(g('twist'), mo * phi_m[mem][0])
                SE['extension'] = max(g('extension'), fo * phi_m[mem][1])
            elif k in ('twist', 'extension'):
                SE = SC
            SF = dict(SB)
            if k in ('twist', 'extension'):
                if mem in phi_m:
                    SF['twist'] = mo * phi_m[mem][0]
                    SF['extension'] = fo * phi_m[mem][1]
                else:
                    SF['twist'] = mo * phi_t
                    SF['extension'] = fo * phi_e
            for var, SS in (('B', SB), ('C', SC), ('D', SD), ('E', SE), ('F', SF)):
                if SS[k] > 0 and sc < R * SS[k]:
                    r[var + '_scale_below'].append((key, cls, format(sc / SS[k], '.2e')))
            if fam == 'RF-CANCEL':
                rs = max(abs(e), D(row[3]))
                if SF[k] > 0 and rs < R * SF[k]:
                    r['F_rec_scale_below'].append((key, cls, format(rs / SF[k], '.2e')))
        out['cases'][cid] = {x: y for x, y in r.items() if y}
        F = out['by_family'].setdefault(fam, {x: 0 for x in r})
        for x in r:
            F[x] += len(r[x])
    json.dump(out, open(sys.argv[3], 'w'), indent=1)
    for f, F in out['by_family'].items():
        print(f, F)


if __name__ == '__main__':
    main()
