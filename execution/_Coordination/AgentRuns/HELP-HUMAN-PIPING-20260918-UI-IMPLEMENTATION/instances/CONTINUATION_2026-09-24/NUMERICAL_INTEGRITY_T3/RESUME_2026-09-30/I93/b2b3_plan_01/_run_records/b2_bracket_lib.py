"""I93: the hybrid-tree rule of b2_bracket.py, shared with b2_mid.py (identical sets and logic)."""
import copy

ALWAYS_C = {"T11", "T11_late_capture", "T11_ordinary_seed", "INVOC", "T19", "STATICS", "T07_moving", "TAV_X"}
ALWAYS_CZ = {"T13", "T14", "T15", "STAGED", "T16_P1", "T16_P2", "T16_P3", "T16_P4", "T16_moving",
             "T17_V1", "T17_V2_clone", "T17_V2_hash", "T17_V3", "T17_V4", "T17_V5", "T17_V6",
             "T17_moving_invocation", "T17_moving_publication", "T17_output", "SUCC", "BODY",
             "TXT_moving", "HELPER_moving"}
BRACKET = {"O_base_sparse", "O_base_dense", "TAV_W", "T12", "NOTICE", "NOTICE_moving"}


def hybrid(t_c, t_cz, low):
    h = copy.deepcopy(t_cz)
    for name in h["forms"]:
        if name.startswith("T25_") or name in ALWAYS_C:
            h["forms"][name] = copy.deepcopy(t_c["forms"][name])
        elif name in ALWAYS_CZ:
            pass
        elif name in BRACKET:
            if low:
                h["forms"][name] = copy.deepcopy(t_c["forms"][name])
        else:
            raise SystemExit("unclassified form " + name)
    return h
