"""I65 U4 G3 T08: the text atoms other records use (stdlib only).

Reads text_budget.<which>.out.json and composite_text.<which>.json and writes
text_closure.<which>.json. Every atom is a byte bound on String CONTENT or CAPACITY as
stated; the derivations are in TEXT.md section 5.
Usage: python3 t08_closure.py <caps|milestone>
"""
import json, os, sys
which = sys.argv[1]
H = os.path.dirname(os.path.abspath(__file__))
tb = json.load(open(os.path.join(H, f"text_budget.{which}.out.json")))
# the ordinary envelope's own diagnostics: the same budget without the receipt finalization's
# replays (whose diagnostics go to local vectors that are dropped) and without the W1 root
tbe = json.load(open(os.path.join(H, f"text_budget_env.{which}.out.json")))
assert tbe["complete"], "envelope text budget incomplete"
ct = json.load(open(os.path.join(H, f"composite_text.{which}.json")))
assert tb["complete"], "text budget incomplete"
ERR = ct["classes"]["error_display"]            # every D1-reachable error Display/Debug <= 8192
RESULT_ID = 1024                                 # longest reached result-id template: 872 bytes
STATIC = 426                                     # longest literal used as text on the reached graph
CONST = 339                                      # longest &'static str constant (PP/FK)
IDENT = 128
row = (RESULT_ID            # id
       + RESULT_ID          # entity_ref (identifier or composite reference)
       + max(STATIC, CONST) # kind
       + max(STATIC, IDENT) # unit
       + max(STATIC, CONST) + IDENT        # basis_ref.ref_type, ref_id
       + 2 * max(STATIC, CONST)            # metadata.component, coordinate_system
       + RESULT_ID + RESULT_ID             # metadata.location, metadata.basis
       + RESULT_ID                         # metadata.sign_convention (one format! site <= 600)
       + 4 * RESULT_ID)                    # source_result_refs (endpoint bending refs, <= 4)
F_len = ct["parts"]["F_len"]
msg_int = ct["classes"]["message_int"]
atoms = {
    "Text(row)": row,
    "Text(diag_total)": tb["retained_diagnostic_bytes"],
    "Text(err)": 2 * ERR,
    "Text(sym)": 2 * ct["report_literal_max_bytes"],
    "Text(audit_error)": 2 * ERR,
    "Text(formation_detail)": 2 * (ERR + 21),
    "Text(recovery_finding)": 1022 + 2 * (16 * F_len + 2 * msg_int) + 8 * 24,
    "D": tb["D_diagnostics"],
    "D_env": tbe["D_diagnostics"],
    "Text(diag_env)": tbe["retained_diagnostic_bytes"],
    "TAV_text_requested": tb["total_text_requested_bytes"],
    "TAV_text_moving": tb["total_text_moving_bytes"],
}
json.dump({"which": which, "atoms": atoms,
           "notes": "content bounds except Text(err)/Text(audit_error)/Text(formation_detail)/Text(sym)/"
                    "Text(recovery_finding), which are capacity bounds (2x length)"},
          open(os.path.join(H, f"text_closure.{which}.json"), "w"), indent=1)
print(json.dumps(atoms, indent=1))
