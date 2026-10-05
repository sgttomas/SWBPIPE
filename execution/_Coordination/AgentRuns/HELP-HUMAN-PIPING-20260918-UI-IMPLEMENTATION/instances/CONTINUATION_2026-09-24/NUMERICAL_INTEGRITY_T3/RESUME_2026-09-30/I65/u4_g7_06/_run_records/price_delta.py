"""I65 U4 G7: price the U6 delta on T17 at in-build strides (stdlib only).
Inputs: the integrated tree's profile_tree.json (the TEXT chain's, byte-identical to G6's) and the
law-test log of the integrated registered build (I65_G5_ATOM lines: in-build atom values).
The one allocating delta on the D1 graph is F5 in g5_ordinary (retained_precision.rs:4136-4145 at
ba1faa1c): per case, `exact: Vec<&Value>` collected from a filter over the envelope diagnostics
(push growth: capacity pushcap(n) <= pushcap(D_env)) and `list(diagnostic_refs).iter().collect::<Vec<_>>()`
(exact size, <= D_env: the preceding loop requires the refs unique and resolving). Both live
together for the comparison and drop at the end of the case's iteration, so they belong to T17's
V4 stage (G3-G6 working sets); counted there in full, for every stage that could hold them."""
import json, re, sys
tree_p, log_p = sys.argv[1:3]
tree = json.load(open(tree_p))
atoms = {m.group(1): int(m.group(2)) for m in re.finditer(r"I65_G5_ATOM \w+\t([^\t]+)\t(\d+)", open(log_p).read())}
def pushcap(h, minimum=4):
    c = 0
    for length in range(1, h + 1):
        if length > c:
            c = max(2 * c, length, minimum)
    return c
def ev(form):
    return sum(c * (1 if a == "1" else atoms[a]) for a, c in form.items())
D_ENV = tree["text_atoms"]["D_env"]
delta_terms = {"s(&Value)": pushcap(D_ENV) + D_ENV}
delta = ev(delta_terms)
moving = atoms["s(&Value)"] * (pushcap(D_ENV) // 2)
stages = {k: ev(v) for k, v in tree["forms"].items() if re.match(r"T17_V\d|T17_V2_", k)}
top = max(stages, key=stages.get)
out = {"D_env": D_ENV, "s(&Value)": atoms["s(&Value)"], "F5_terms": delta_terms, "F5_requested_bytes": delta,
       "F5_moving_bytes_last_growth": moving, "T17_stages_in_build": stages, "T17_argmax": top,
       "V4_plus_F5": stages["T17_V4"] + delta, "headroom_to_argmax": stages[top] - (stages["T17_V4"] + delta),
       "T17_changes": stages["T17_V4"] + delta > stages[top],
       "moving_max_in_W4": max(ev(tree["forms"][k]) for k in ("T17_moving_publication", "T17_moving_invocation")),
       "moving_changes": moving > max(ev(tree["forms"][k]) for k in ("T17_moving_publication", "T17_moving_invocation"))}
print(json.dumps(out, indent=1))
