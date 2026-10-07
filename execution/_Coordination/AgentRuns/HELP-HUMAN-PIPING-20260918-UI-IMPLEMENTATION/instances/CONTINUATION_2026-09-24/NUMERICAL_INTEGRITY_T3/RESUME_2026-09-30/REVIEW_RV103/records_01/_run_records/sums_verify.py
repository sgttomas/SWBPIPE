#!/usr/bin/env python3
"""RV103: verify SHA256SUMS files against the committed tree (a git archive of the PR head),
and report files in each sealed folder that none of its sum files lists.
Usage: sums_verify.py <copy_T3_root> <folder>...   (folders T3-relative)"""
import hashlib, os, sys, glob
root=sys.argv[1]
print("folder\tsumfile\tok\tbad\tmissing\tuncovered\tdetail")
for folder in sys.argv[2:]:
    fd=os.path.join(root,folder)
    sums=sorted(f for f in os.listdir(fd) if "SHA256SUMS" in f)
    listed=set(); 
    for sf in sums:
        ok=bad=miss=0; det=[]
        for line in open(os.path.join(fd,sf)):
            line=line.rstrip("\n")
            if not line.strip(): continue
            h,name=line.split(None,1); name=name.lstrip("*")
            rel=os.path.normpath(name); listed.add(rel)
            p=os.path.join(fd,rel)
            if not os.path.isfile(p): miss+=1; det.append("MISSING "+rel); continue
            if hashlib.sha256(open(p,"rb").read()).hexdigest()==h: ok+=1
            else: bad+=1; det.append("BAD "+rel)
        print(f"{folder}\t{sf}\t{ok}\t{bad}\t{miss}\t\t{'; '.join(det)}")
    allf=set()
    for dp,dn,fn in os.walk(fd):
        for f in fn:
            rel=os.path.relpath(os.path.join(dp,f),fd)
            if "SHA256SUMS" in os.path.basename(rel) and os.path.dirname(rel)=="": continue
            allf.add(rel)
    unc=sorted(allf-listed)
    print(f"{folder}\t(coverage)\t\t\t\t{len(unc)}\t{'; '.join(unc[:10])}")
