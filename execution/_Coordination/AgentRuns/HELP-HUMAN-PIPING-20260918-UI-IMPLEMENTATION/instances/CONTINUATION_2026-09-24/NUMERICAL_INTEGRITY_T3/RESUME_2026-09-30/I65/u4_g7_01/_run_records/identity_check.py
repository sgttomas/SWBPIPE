"""G7 item 2: the integrated build's compiled identity, reviewed inputs and reader layouts (from the
law-test log) against the registered entry in that tree's retained_memory.rs (stdlib only)."""
import re, sys, json
log, rm = sys.argv[1:3]
L = open(log).read()
src = open(rm).read()
ident = re.search(r"I65_G5_IDENTITY (\S+)", L).group(1)
inputs = re.search(r"I65_G5_REVIEWED_INPUTS (\S+)", L).group(1)
layouts = re.findall(r"I65_G5_READER_LAYOUT (\w+) size=(\d+) align=(\d+)", L)
entry = src[src.index("static REGISTERED_PROFILES"):]
entry = entry[:entry.index("\n}];") + 4]
e_ident = re.search(r'identity: "([^"]+)"', entry).group(1)
e_inputs = re.search(r'reviewed_inputs: "([^"]+)"', entry).group(1)
e_layouts = re.findall(r"TypeLayout \{ size: (\d+), align: (\d+) \}", entry)
e_thr = re.search(r"threshold_bytes: ([\d_]+)", entry).group(1)
out = {
    "identity_equal": ident == e_ident,
    "reviewed_inputs_equal": inputs == e_inputs,
    "reviewed_inputs_count": len(inputs.split(";")) - 1,
    "unavailable_inputs": inputs.count("unavailable"),
    "layouts_compiled": [(n, int(s), int(a)) for n, s, a in layouts],
    "layouts_entry": [(int(s), int(a)) for s, a in e_layouts],
    "layouts_equal": [(int(s), int(a)) for _, s, a in layouts] == [(int(s), int(a)) for s, a in e_layouts],
    "threshold_bytes": int(e_thr.replace("_", "")),
    "registered_tests": {t: (re.search(rf"test retained_memory::law_tests::{t} \.\.\. .*?(ok|FAILED)", L, re.S) or [None, "absent"])[1]
                         for t in ("the_registered_profile_is_the_only_permit_source", "admit_grants_a_permit_for_the_milestone_in_the_registered_build",
                                   "registered_g_c_declines_only_unattempted_solves", "bindings_need_witnesses_inputs_and_reader_layouts",
                                   "reviewed_inputs_bind_the_lock_and_the_reader_statics", "profile_in_build_record", "challenge_bounds_are_the_profile")},
    "summary": re.findall(r"^test result: .*$", L, re.M),
}
out["registered"] = out["identity_equal"] and out["reviewed_inputs_equal"] and out["layouts_equal"] and out["unavailable_inputs"] == 0
print(json.dumps(out, indent=1))
