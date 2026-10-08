#!/usr/bin/env python3
"""RV110: verify sum files against the committed tree (a git archive of the PR head),
and report files in each sealed folder that none of its sum files lists.
Usage: sums_verify.py <copy_T3_root> <folder>[:<sumfile-glob>]...   (folders T3-relative)
A sum file is any file in the folder's top level whose name contains SHA256SUMS."""
import hashlib, os, sys, fnmatch
root=sys.argv[1]
print("folder\tsumfile\tentries\tok\tbad\tmissing\tuncovered\tdetail")
for arg in sys.argv[2:]:
    folder, _, glob_ = arg.partition(":")
    fd=os.path.join(root,folder)
    sums=sorted(f for f in os.listdir(fd) if "SHA256SUMS" in f and os.path.isfile(os.path.join(fd,f)) and (not glob_ or fnmatch.fnmatch(f,glob_)))
    listed=set()
    for sf in sums:
        ok=bad=miss=n=0; det=[]
        for line in open(os.path.join(fd,sf), encoding="utf-8"):
            line=line.rstrip("\n")
            if not line.strip() or line.startswith("#"): continue
            h,name=line.split(None,1); name=name.lstrip("*"); n+=1
            rel=os.path.normpath(name); listed.add(rel)
            p=os.path.join(fd,rel)
            if not os.path.isfile(p): miss+=1; det.append("MISSING "+rel); continue
            if hashlib.sha256(open(p,"rb").read()).hexdigest()==h: ok+=1
            else: bad+=1; det.append("BAD "+rel)
        print(f"{folder}\t{sf}\t{n}\t{ok}\t{bad}\t{miss}\t\t{'; '.join(det)}")
    allf=set()
    for dp,dn,fn in os.walk(fd):
        for f in fn:
            rel=os.path.relpath(os.path.join(dp,f),fd)
            if rel in sums: continue
            allf.add(rel)
    unc=sorted(allf-listed)
    print(f"{folder}\t(coverage, {len(sums)} sum files)\t\t\t\t\t{len(unc)}\t{'; '.join(unc[:12])}")
