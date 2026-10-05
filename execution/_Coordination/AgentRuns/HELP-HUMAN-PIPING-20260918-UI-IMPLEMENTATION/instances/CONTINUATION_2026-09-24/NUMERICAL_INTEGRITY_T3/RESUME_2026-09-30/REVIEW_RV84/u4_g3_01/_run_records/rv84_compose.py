"""RV84: independent re-derivation of the G3 composition at the D1 caps (stdlib only).

Written from RV84's own reading of source at NUM 5ae5fe4f0f (source_receipt.rs finalize_for
:867-1110, hash/checked :30-37; canonical_json lib.rs :39-162; serde_json 1.0.149 ser.rs
ESCAPE table :2135-2165). Inputs taken from the packet and NOT re-derived here are marked
PACKET: O without T25 (I54 families), text TAV and its atoms (P, D_env, Text(row),
Text(diag_env), message_int), T11-T15, the carried case outputs and the wire entry.
Layout strides are the packet's ILLUSTRATIVE values (ASSUMED); RV84 has no build facts.
Usage: python3 rv84_compose.py > rv84_compose.out.json
"""
import json

M = 4_026_531_840
R = 64 * 2**20
# --- ASSUMED illustrative strides (same values as the packet's g3lib.py) -------------------
sV, NODE_SV, NODE_STR_UNIT, NODE_REF_UNIT, sString = 32, 736, 376, 288, 24
# --- PACKET inputs -------------------------------------------------------------------------
O_base = {"sparse": 245_209_549, "dense": 264_919_997}          # ORDINARY.md, T25 removed
TAV, TAV_mov_extra = 1_540_955_362, 2_599_962                    # TEXT.md
T11, T12_15 = 204_912, 68_293_632                                # COMPOSITION.md section 2
P, D_env, ROW, DIAG = 2_115, 11_030, 11_474, 77_812_844          # text_closure.caps.json
L_MAX = 2_549_385                                                # composite_text.caps.json message_int
CARRIED = 51_639_470          # T25 - I1 in t25_caps.caps.eps*.out.json (the per-case outputs)
TYPED = 16_292_992            # T_resident (= requested() minus the raw clone and Content buffers)
WIRE = 81_938_604 - (P * 296 + P * ROW + (2 * P * sV + 3 * P * NODE_SV + 128 * P + P * ROW))  # T25.9 minus temporaries
RID = 1024

def ceil_div(a, b): return -(-a // b)
def tree(arr, obj, ent, strb, keyb):          # serde_json Value tree, exact strings (to_value / json!)
    return sV * arr + NODE_SV * (obj + ent // 5) + strb + keyb
def parsed(arrays, arr, obj, ent, strb, keyb):  # checked_parse: Vec::new()+push, owned keys + seen clones
    return sV * (4 * arrays + 2 * arr) + NODE_SV * (obj + ent // 5) + strb + 2 * keyb

# --- the publication Value (MechanicsEnvelope, branch X: preview None, contract_evidence None)
# results: P objects {id,kind,value,unit,entity_ref,basis_ref{2},source_result_refs[<=4],metadata{5}}
# diagnostics: D_env objects {id,code,severity,message,source,affected_refs[..]}; refs slots as packet
# scaffolding: 11 objects, 57 entries, 3 arrays, 8 slots, <= 6,144 string bytes, 10 numbers (RV84 count)
l = 192
pub = dict(arr=P + 4 * P + D_env + 4 * D_env + 3 * (1 + l) + 8,
           obj=3 * P + D_env + 11, ent=15 * P + 6 * D_env + 57,
           arrays=P + D_env + 3,
           strb=P * ROW + DIAG + 6_144,
           keyb=15 * P * 24 + 6 * D_env * 16 + 57 * 40, nums=P + 10)
values = pub["arr"] + pub["ent"] + 1
def e_text(eps):
    return eps * pub["strb"] + pub["keyb"] + 8 * values + 24 * pub["nums"]
T_pub = tree(pub["arr"], pub["obj"], pub["ent"], pub["strb"], pub["keyb"])
T_parsed = parsed(pub["arrays"], pub["arr"], pub["obj"], pub["ent"], pub["strb"], pub["keyb"])
ids = NODE_REF_UNIT * ((P + D_env) // 5 + 1)
accounted = NODE_STR_UNIT * (P // 5 + 1) + P * RID
observations = P * sString + P * RID
seen_sets = 6 * (sString * 32 + 32 + 23)      # depth-6 active frames, <= 16 keys each (packet law)

out = {"inputs_from_packet": ["O_base", "TAV", "T11", "T12_15", "P", "D_env", "Text(row)", "Text(diag_env)",
                              "message_int", "carried case outputs", "wire entry"],
       "publication_facts": pub, "publication_tree": T_pub, "modes": {}}
for eps in (6, 2):
    e = e_text(eps)
    escape_temp = max(128, 2 * (eps * L_MAX + 2))   # serde_json::to_string(text) per string (canonical_json:132)
    hash_live = (T_pub + 128) + max(128, 2 * e) + T_parsed + seen_sets + max(8, 2 * e) + escape_temp + 2_080 + 4_096
    I1 = CARRIED + T_pub + TYPED + ids + accounted + observations + WIRE + 4_096 + hash_live
    out[f"EPS{eps}"] = {"publication_text_e": e, "escape_temporary": escape_temp, "hash_route_live": hash_live,
                        "T25_I1_rv84": I1, "T25_packet": {6: 3_074_072_410, 2: 1_439_324_474}[eps],
                        "moving_extra_rv84": max(e, TAV_mov_extra, 131_072 * 64)}
for mode in ("sparse", "dense"):
    for eps in (6, 2):
        t25 = out[f"EPS{eps}"]["T25_I1_rv84"]
        X_req = O_base[mode] + t25 + TAV + T11
        X = X_req + out[f"EPS{eps}"]["moving_extra_rv84"] + R
        X_pkt = O_base[mode] + out[f"EPS{eps}"]["T25_packet"] + TAV + T11 + max(e_text(eps), TAV_mov_extra, 131_072 * 64) + R
        W_req = O_base[mode] + TAV + T11 + T12_15
        W = W_req + max(TAV_mov_extra, 131_072 * 64) + R
        out["modes"].setdefault(mode, {})[f"EPS{eps}"] = {
            "X_rv84_Emov_plus_R": X, "X_rv84_over_M": X / M, "X_fits": X <= M,
            "X_with_packet_T25": X_pkt, "W_G3_part_Emov_plus_R": W, "W_over_M": W / M,
            "headroom_for_G4": M - W, "headroom_to_0.9M_X": int(0.9 * M) - X_pkt}
print(json.dumps(out, indent=1))
