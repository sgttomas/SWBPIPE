# KF3-B2 (T3 KF3 checkpoint D): the TREE frames' measured heap against both
# E_max implementations, and the pass term at the 1024 shift derived from the
# code. Run from the repository root of <wt>/kf3; reads committed records only.
import json

T3 = ('projects/chirality-piping/execution/_Coordination/AgentRuns/'
      'HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/'
      'CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3')
B = T3 + '/IMPLEMENTATION/KF3/_run_records/b'
H = 'projects/chirality-piping/core/solver/performance_harness/observations/k6b/counts.jsonl'
RUNS = {'RF-LARGE-TREE-n10000-AX': B + '/runs/15_RF-LARGE-TREE-n10000-AX_w1a.jsonl',
        'RF-LARGE-TREE-n10000-ROT': B + '/runs/16_RF-LARGE-TREE-n10000-ROT_w1a.jsonl'}

def lines(path):
    return [json.loads(l) for l in open(path) if l.strip()]

vr = {o['case']: o for o in lines(B + '/counts.json')}
h = {o['model']: o for o in lines(H) if o.get('model') in RUNS}

def wide(limbs):  # H counts.rs:224, 8L + 16
    return 8 * limbs + 16

for case, path in RUNS.items():
    measured = [o for o in lines(path) if o['kind'] == 'summary'][0]['repeats_heap_peak']
    o = h[case]
    g = lambda k: o['estimate_w1_' + k]
    fixed_h = g('fixed_bytes')
    kept = (fixed_h + sum(g('shared_%d' % p) for p in (128, 256, 512, 1024))
            + sum(g('state_%d' % p) for p in (128, 256, 512, 1024))
            + sum(g('verify_%d' % p) for p in (256, 512, 1024)))
    e_h = o['estimate_adm_bytes_w1a']
    kernel = e_h - fixed_h
    e_vr = vr[case]['estimate_max_bytes']
    fixed_vr = vr[case]['estimate_fixed_bytes']
    nf, rows, p_entries = o['w1_free_dofs'], o['w1_rows'], o['w1_profile_entries']
    w = wide(16)
    # H counts.rs:511-515 at 1024 (OPTION_EXTRA = 8, VEC_HEADER_BYTES = 24)
    at_shift_h = (3 * rows * (w + 8) + p_entries * w + nf * (24 + 8 + 4)
                  + p_entries * w + nf * ((24 + 8) + (w + 8) + w))
    # the code at bound.rs:1040 -> 525 (inside nl_pass on the shifted factor):
    # e_rows, w, a_s (Option<Wide<16>> is 144 B); the scaled profile; the
    # shifted factor without `work` (freed at bound.rs:914); at, bt, ct.
    at_shift_code = (3 * rows * w + p_entries * w + nf * (24 + 8 + 4)
                     + p_entries * w + nf * ((24 + 8) + w) + 3 * nf * w)
    print(case)
    print('  measured heap peak (vk_scale, K6 allocator)      %13d' % measured)
    print('  VR E_max (VR/src/scale.rs, port of 082990c8d)     %13d  measured - E = %+d  rho %.4f'
          % (e_vr, measured - e_vr, measured / e_vr))
    print('  K6b E_max (H counts.jsonl, H harness)             %13d  measured - E = %+d' % (e_h, measured - e_h))
    print('    H fixed %d, kept before the 1024 pass %d, pass_1024 %d' % (fixed_h, kept, g('pass_1024')))
    print('  K6b kernel terms (E_max - H fixed)                %13d' % kernel)
    print('  K6b kernel terms + VR fixed (%d)             %13d  measured - E = %+d'
          % (fixed_vr, kernel + fixed_vr, measured - (kernel + fixed_vr)))
    print('  1024 shift: H at_shift %d; from the code %d; difference %+d'
          % (at_shift_h, at_shift_code, at_shift_code - at_shift_h))
    print('    of which nl_pass at/bt/ct less work: %+d; Option overcount (3 rows + nf) x 8: %+d'
          % (2 * nf * w, -8 * (3 * rows + nf)))
    print('  K6b E_max with the 1024 shift from the code (H harness) %13d' % (e_h + at_shift_code - at_shift_h))
