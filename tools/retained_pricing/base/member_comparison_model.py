"""C8 exact borrowed MemberSection comparison staging; no other owner removed."""
import hashlib, re
from pathlib import Path
FILE="core/product_physics/src/source_receipt/composite.rs"
OLD="993146b22041d2baa3f969fca92063c5ea13d1299943602238de418d4998eed1"
NEW="8f9f963b4dd1b66691d6b2151fc2818bb1204581282bf6b04396adac33eef718"
# B7a B4/D1 removes only the two obsolete non-composite selector args.
# The borrowed MemberSection comparison and complete positive frame are unchanged.
B7A="b2507ed61be43b58d8696a3f8f137ee551d8e48397ede7bf34a73399c14446c3"
# T4-U6 (D-5): composite_exact publishes the source maximum once per section basis (each after a coincident-section
# check) and binds each basis's evidence; the borrowed MemberSection comparison, its descriptor binding and the
# staging frame are unchanged. The new rows and evidence are priced at their own sites and by the row premise.
U6="39beb234d2eab8ecbf770ac1f8d12dc7661ce94bc34b0d44a1904dda60907f21"
# T4-S5 (U4B-NQ): composite_exact adds one allocation-free check that admits `derived_not_qualified` for the two
# derived kinds only (a static `bad(..)` refusal otherwise); the borrowed comparison and its frame are unchanged.
U4B="f0544806c59835a02dc9c85d92164563d2b958d966ad4a4f050dc5fa93e1335c"
# O4 S5 (retained-pricing-rules.md §8.9, O4-SR): composite_exact evaluates the source maximum on the reduced section
# with a second composite_member_maximum call where the member's stress sections differ (in place of the
# coincident-section refusals) and checks each stress row on its own basis's section. The borrowed MemberSection
# comparison, its descriptor binding and the staging frame are unchanged; the second maximum is O4-SR's owner.
O4="25905ffb6bcf9ad61209fc73ad3675f40851815f78714faa9978e202f5b8cdf5"
# W-15 D3 (retained-pricing-rules.md §8.21): the source maximum's row names `component_body:None` (no allocation);
# the borrowed MemberSection comparison, its descriptor binding and the staging frame are unchanged.
W15="446190d7e867a7141ca19d932e33a9f6a28506e8a421e551a7ae446e01807365"
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
    if digest not in (OLD,NEW,B7A,U6,U4B,O4,W15):raise ValueError("C8: comparison owner/call-chain/lifetime source changed")
    borrowed=digest in (NEW,B7A,U6,U4B,O4,W15)
    needle="member: descriptor_member," if borrowed else "member: member.member_id.clone(),"
    if body.count(needle)!=1:raise ValueError("C8: missing/duplicate metadata expression")
    line=source.count(chr(10),0,hit.start()+body.index(needle))+1
    return {"borrowed":borrowed,"site":FILE+":"+str(line),"body_sha256":digest}

def coefficient(cases,state):
    return cases if state["borrowed"] else 0
