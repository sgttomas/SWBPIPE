#!/usr/bin/env python3
"""ROOT's helper for per-lane records. Subcommands:
  seal   : instantiate a brief from a template into lanes/<LANE>/briefs/, hash it, index it, add a graph node
  retain : record a retained return (path relative to the lane folder) on a graph node and in the index
  note   : append a dated line to the lane's HANDOFF.md and set the graph frontier
All writes stay under {RUN}/lanes/<LANE>/ of the given worktree. Nothing absolute is written into a file.
"""
import argparse, hashlib, json, os, re, sys, datetime

RUN_REL = "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION"
FENCE = "Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081)."
_HEADS = ["Users", "private", "home", "tmp", "var/folders"]
MACHINE = re.compile(r"(?:/(?:%s)/[^\s)\"'`<>]+)" % "|".join(_HEADS))

def now():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%MZ")

def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()

def lane_dir(wt, lane):
    d = os.path.join(wt, RUN_REL, "lanes", lane)
    assert os.path.isdir(d), d
    return d

def read(p): return open(p, encoding="utf-8").read()
def write(p, s):
    if MACHINE.search(s):
        sys.exit("absolute machine path in " + p)
    open(p, "w", encoding="utf-8").write(s)

def before_fence(s, block):
    assert s.rstrip("\n").endswith(FENCE), "no claim fence at end"
    i = s.rindex(FENCE)
    head = s[:i].rstrip("\n")
    last = head.split("\n")[-1]
    sep = "\n" if last.startswith("- ") else "\n\n"
    return head + sep + block.rstrip("\n") + "\n\n" + FENCE + "\n"

def load_graph(d):
    return json.loads(read(os.path.join(d, "GRAPH.json")))
def save_graph(d, g):
    write(os.path.join(d, "GRAPH.json"), json.dumps(g, indent=2, ensure_ascii=False) + "\n")

def cmd_seal(a):
    d = lane_dir(a.wt, a.lane)
    t = read(a.template)
    subs = {"DATE": a.date or now()}
    for kv in a.set or []:
        k, v = kv.split("=", 1); subs[k] = v
    for k, v in subs.items():
        t = t.replace("@" + k + "@", v)
    left = re.findall(r"@[A-Z_]+@", t)
    assert not left, "unfilled placeholders: %s" % sorted(set(left))
    if a.node in load_graph(d)["nodes"] and not a.force:
        sys.exit("graph node exists: %s (give the instrument its own node id, or pass --force to merge into it)" % a.node)
    out = os.path.join(d, "briefs", a.name)
    assert not os.path.exists(out) or a.force, "exists: " + out
    write(out, t)
    h = sha(out)
    idx = os.path.join(d, "INDEX.md")
    s = read(idx)
    row = "| `briefs/%s` | `%s` | %s | %s |" % (a.name, h, subs["DATE"], a.note)
    # insert after the last row of the Sealed instructions table
    m = re.search(r"(## Sealed instructions\n\n(?:\|.*\n)+)", s)
    assert m, "no sealed table"
    s = s[:m.end()] + row + "\n" + s[m.end():]
    write(idx, s)
    g = load_graph(d)
    existing = g["nodes"].get(a.node)
    if existing is not None and not a.force:
        sys.exit("graph node exists: %s (give the instrument its own node id, or pass --force to merge into it)" % a.node)
    node = dict(existing or {})
    node.update({"role": a.role, "model": a.model, "status": "sealed", "sealed": subs["DATE"],
                 "briefSha256": h, "brief": "briefs/" + a.name})
    if subs.get("SHA"):
        node["candidate"] = subs["SHA"]
    node.setdefault("candidate", None)
    node.setdefault("returnSha256", None)
    g["nodes"][a.node] = node
    save_graph(d, g)
    print(h, out.replace(a.wt, "{WT}"))

def cmd_retain(a):
    d = lane_dir(a.wt, a.lane)
    p = os.path.join(d, a.path)
    assert os.path.isfile(p), p
    h = sha(p)
    g = load_graph(d)
    n = g["nodes"].setdefault(a.node, {"role": a.role or "", "model": a.model or ""})
    n["status"] = a.status; n["returnSha256"] = h; n["return"] = a.path
    if a.ran: n["modelThatRan"] = a.ran
    if a.candidate: n["candidate"] = a.candidate
    save_graph(d, g)
    idx = os.path.join(d, "INDEX.md")
    s = read(idx)
    line = "- %s: **%s** %s. Return `%s`, SHA-256 `%s`%s. %s" % (
        now(), a.node, a.status, a.path, h, ("; model that ran: " + a.ran) if a.ran else "", a.note or "")
    write(idx, before_fence(s, line))
    print(h)

def cmd_note(a):
    d = lane_dir(a.wt, a.lane)
    hp = os.path.join(d, "HANDOFF.md")
    write(hp, before_fence(read(hp), "- %s: %s" % (now(), a.text)))
    if a.frontier:
        g = load_graph(d); g["current_frontier"] = a.frontier; save_graph(d, g)
    print("ok")

ap = argparse.ArgumentParser(); sub = ap.add_subparsers(dest="cmd", required=True)
s1 = sub.add_parser("seal"); s1.add_argument("--wt", required=True); s1.add_argument("--lane", required=True)
s1.add_argument("--template", required=True); s1.add_argument("--name", required=True); s1.add_argument("--node", required=True)
s1.add_argument("--role", required=True); s1.add_argument("--model", required=True); s1.add_argument("--note", required=True)
s1.add_argument("--date"); s1.add_argument("--set", action="append"); s1.add_argument("--force", action="store_true"); s1.set_defaults(f=cmd_seal)
s2 = sub.add_parser("retain"); s2.add_argument("--wt", required=True); s2.add_argument("--lane", required=True)
s2.add_argument("--node", required=True); s2.add_argument("--path", required=True); s2.add_argument("--status", required=True)
s2.add_argument("--ran"); s2.add_argument("--note"); s2.add_argument("--role"); s2.add_argument("--model"); s2.add_argument("--candidate"); s2.set_defaults(f=cmd_retain)
s3 = sub.add_parser("note"); s3.add_argument("--wt", required=True); s3.add_argument("--lane", required=True)
s3.add_argument("--text", required=True); s3.add_argument("--frontier"); s3.set_defaults(f=cmd_note)
a = ap.parse_args(); a.f(a)
