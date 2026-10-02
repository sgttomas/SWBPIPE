#!/usr/bin/env python3
"""A1 checkpoint E only: verify/apply one frozen unified patch; never run Rust.
Usage: python -B apply_frozen_patch.py MANIFEST ID DISPOSABLE [--apply]
Without --apply, checks entirely in memory. All writes require a later grant.
"""
import sys,json,pathlib,hashlib,re

def digest(b): return hashlib.sha256(b).hexdigest()

def main():
    manifest_path=pathlib.Path(sys.argv[1]).resolve()
    identifier=sys.argv[2]
    root=pathlib.Path(sys.argv[3]).resolve()
    apply=sys.argv[4:]==["--apply"]
    if sys.argv[4:] not in ([],["--apply"]): raise ValueError("unknown arguments")
    # A checkout is not a disposable archive. This guard is not an OS sandbox.
    if (root/".git").exists(): raise ValueError("refuse Git checkout as patch target")
    manifest=json.loads(manifest_path.read_text())
    variants=[v for v in manifest["variants"] if v["id"]==identifier]
    if len(variants)!=1: raise ValueError("exact frozen ID required")
    v=variants[0]
    patch=(manifest_path.parent/v["patch"]).read_bytes()
    if digest(patch)!=v["patch_sha256"]: raise ValueError("patch hash mismatch")
    target=(root/v["source_path"]).resolve()
    target.relative_to(root)
    before=target.read_bytes()
    if digest(before)!=v["before_sha256"]: raise ValueError("source hash mismatch")
    src=before.decode().splitlines(True);lines=patch.decode().splitlines(True)
    if lines[:2]!=["--- a/"+v["source_path"]+"\n","+++ b/"+v["source_path"]+"\n"]:
        raise ValueError("patch target mismatch")
    out=[];cursor=0;i=2
    while i<len(lines):
        m=re.match(r"@@ -(\d+)(?:,\d+)? \+(\d+)(?:,\d+)? @@",lines[i])
        if not m: raise ValueError("bad hunk")
        start=int(m[1])-1
        if start<cursor: raise ValueError("overlapping hunks")
        out+=src[cursor:start];cursor=start;i+=1
        while i<len(lines) and not lines[i].startswith("@@"):
            line=lines[i];i+=1
            if line[0] not in " +-": raise ValueError("unsupported patch row")
            if line[0] in " -":
                if cursor>=len(src) or src[cursor]!=line[1:]: raise ValueError("context mismatch")
                cursor+=1
            if line[0] in " +":out.append(line[1:])
    after=("".join(out+src[cursor:])).encode()
    if digest(after)!=v["after_sha256"]: raise ValueError("postimage hash mismatch")
    if apply:
        target.write_bytes(after)
        if digest(target.read_bytes())!=v["after_sha256"]: raise ValueError("write verification failed")
    print(json.dumps({"id":identifier,"mode":"apply" if apply else "check-only",
        "before_sha256":digest(before),"after_sha256":digest(after),"source_write":apply}))
if __name__=="__main__": main()

