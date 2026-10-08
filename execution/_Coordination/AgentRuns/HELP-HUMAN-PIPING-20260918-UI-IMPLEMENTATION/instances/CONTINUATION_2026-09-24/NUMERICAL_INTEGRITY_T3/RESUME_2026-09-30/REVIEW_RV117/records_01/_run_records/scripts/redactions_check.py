#!/usr/bin/env python3
"""RV117 (from RV110): none of REDACTIONS.json's 13 originals is in the PR head's tree.
Usage: redactions_check.py <repo> <H> <M> <T3 path> <pre-redaction rev>"""
import json, subprocess, sys, hashlib
repo,H,M,T3,OLD=sys.argv[1:6]
def git(*a): return subprocess.run(["git","-C",repo,*a],capture_output=True,check=True).stdout
red=json.loads(git("show",f"{H}:{T3}/IMPLEMENTATION/HANDOFF_2026-10-05/_run_records/REDACTIONS.json"))
entries = red if isinstance(red,list) else red.get("files", red.get("entries", red.get("redactions")))
print("entries:",len(entries))
orig_blobs={}
for e in entries:
    p=e["path"].replace("T3/", T3+"/", 1) if e["path"].startswith("T3/") else e["path"]; o=e["original_sha256"]; r=e.get("redacted_sha256")
    try: b=git("rev-parse",f"{OLD}:{p}").decode().strip(); data=git("cat-file","blob",b)
    except subprocess.CalledProcessError: print("NOT AT OLD",p); continue
    ok=hashlib.sha256(data).hexdigest()==o
    hb=git("rev-parse",f"{H}:{p}").decode().strip() if subprocess.run(["git","-C",repo,"cat-file","-e",f"{H}:{p}"]).returncode==0 else None
    hsha=hashlib.sha256(git("cat-file","blob",hb)).hexdigest() if hb else None
    orig_blobs[b]=p
    print(f"orig-blob-at-OLD-matches-original_sha256={ok}  head-sha==redacted={hsha==r}  path=...{p[-60:]}")
tree=set(l.split()[2] for l in git("ls-tree","-r",H).decode().splitlines())
mtree=set(l.split()[2] for l in git("ls-tree","-r",M).decode().splitlines())
print("H tree distinct blobs:",len(tree)," original blobs in H tree:",len(set(orig_blobs)&tree)," in M tree:",len(set(orig_blobs)&mtree))
origs={e["original_sha256"] for e in entries}
ch=git("diff","--no-renames","--name-only",M,H).decode().split()
hits=[p for p in ch if hashlib.sha256(git("show",f"{H}:{p}")).hexdigest() in origs]
print("changed files whose sha256 equals an original:",len(hits),"of",len(ch))
