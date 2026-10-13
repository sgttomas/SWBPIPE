"""C8 exact borrowed MemberSection comparison staging; no other owner removed."""
import hashlib, re
from pathlib import Path
FILE="core/product_physics/src/source_receipt/composite.rs"
OLD="993146b22041d2baa3f969fca92063c5ea13d1299943602238de418d4998eed1"
NEW="8f9f963b4dd1b66691d6b2151fc2818bb1204581282bf6b04396adac33eef718"
# B7a B4/D1 removes only the two obsolete non-composite selector args.
# The borrowed MemberSection comparison and complete positive frame are unchanged.
B7A="b2507ed61be43b58d8696a3f8f137ee551d8e48397ede7bf34a73399c14446c3"
# Source union of bindings, complete operand/argument/result/move slots;
# each is rounded to the maximum native alignment, with no slot reuse.
SLOTS={"&'static u8":16,"&'static [u8]":8,"usize":16,"u64":4,
       "f64":4,"u8":4,"bool":16,"i32":4}

def inspect(snapshot):
    source=(Path(snapshot)/FILE).read_bytes().decode("utf-8")
    hits=list(re.finditer(r"^([ \t]*)(?:pub(?:\([^)]*\))?\s+)?fn composite_exact\b",source,re.M))
    if len(hits)!=1:raise ValueError("C8: missing/duplicate exact comparison owner")
    hit=hits[0];end=re.search(r"^"+re.escape(hit.group(1))+r"}",source[hit.end():],re.M)
    if not end:raise ValueError("C8: missing comparison owner end")
    body=source[hit.start():hit.end()+end.end()]
    digest=hashlib.sha256(body.encode()).hexdigest()
    if digest not in (OLD,NEW,B7A):raise ValueError("C8: comparison owner/call-chain/lifetime source changed")
    borrowed=digest in (NEW,B7A)
    needle="member: descriptor_member," if borrowed else "member: member.member_id.clone(),"
    if body.count(needle)!=1:raise ValueError("C8: missing/duplicate metadata expression")
    line=source.count(chr(10),0,hit.start()+body.index(needle))+1
    return {"borrowed":borrowed,"site":FILE+":"+str(line),"body_sha256":digest}

def coefficient(cases,state):
    return cases if state["borrowed"] else 0
