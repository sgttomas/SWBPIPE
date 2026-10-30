"""W1 coverage repair D3 (DEL-04-08 Design w1-coverage-repair.md §2.2 and §6): the positive owners the repair adds
outside the chain's TEXT inventory (retained-pricing-rules.md §8.11, W1R-FS).

check_intervals now returns the coupled, floored per-body scales the gate judged every row with (the vector
row_scales_reduced reserves exactly at the source's body_count, capacities[6]), and certify_final moves them into a new
`final_scales: Vec<[f64;4]>` field of CertifiedProductProof instead of dropping them. No allocation is added, but the
backing now lives with the proof until it drops after serialization, and every holder of a proof is one Vec header
(24 B) larger. Both are charged positively per attempted case, with no reuse credit: the existing T14.3 charge of the
transient scales (BodyCoverage) stays.

The owner is bound to the sha256 of the bodies that allocate, move and read it and to its exact statements; every file
is also under an existing full-file pin (w1_member_load_model, b7a_domain_model). A snapshot without the field adds
nothing, so earlier registered trees keep their coefficients; a partly present or changed body stops the run."""
import hashlib
import os
import re

from g3lib import s, Xc

FKF = "core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs"
RW = "core/product_physics/src/retained_wire.rs"
RP = "core/product_physics/src/retained_product.rs"
FIELD = "final_scales:Vec<[f64;4]>,"

# Read at 3bbfdd5080 (the repair's implementation head).
BODY_PINS = {
    # W-7 D3 (retained-pricing-rules.md §8.20): row_scales_reduced admits a not-covered force or moment row in a combination (a predicate); the final scales are unchanged.
    (FKF, "row_scales_reduced"): "3322775097f9f9ab59aac76ff2b38bf3dfa3769a76debebe8b155d9403068d26",
    (FKF, "check_intervals"): "0dbce60eb55b833a7a6f81c9482c460399fe2da02d4f2412f16f14ca7564953a",
    (FKF, "certify_final"): "98872d34ac2b49dc783ca36e096361e0dae251c0a56aafd0e842d1086df6938e",
    (RW, "selection"): "4603dee55a5f4d98bd56d21dc1869bc0c66626d70060ee98ba7e8132e5b616cb",
}
# CertifiedProductProof: its declaration through its accessor impl.
PROOF_START = "pub struct CertifiedProductProof {\n"
PROOF_STOP = "pub struct ProductFinalSpent {"
PROOF_PIN = "9c7e36013efdd161e82112e0e6b364e4e2f5320f8fae85a99a360a828a6d9bab"

# Proof holders, per attempted case: the attempt's held record (FrozenCase, RefusedCase or FrozenCombination, to
# serialization) and the frames that hold a proof by value on the case chain, each counted twice (its local and its
# return slot): certify_final, freeze_case_certify, freeze_case_settle, freeze_case's closure, freeze_case, and the
# attempt loop's closure. The combination chain (certify_final, freeze_combination, its attempt closure) holds fewer.
PROOF_FRAMES = 6
PROOF_HOLDERS = 1 + 2 * PROOF_FRAMES


def _read(snapshot, rel):
    path = os.path.join(snapshot, rel)
    if not os.path.exists(path):
        return None
    with open(path, encoding="utf-8") as stream:
        return stream.read()


def _body(text, name):
    hits = list(re.finditer(r"^([ \t]*)(?:pub(?:\([a-z]+\))?\s+)?fn\s+" + re.escape(name) + r"\b", text, re.M))
    if not hits:
        return None
    if len(hits) != 1:
        raise SystemExit(f"W1R owners: {name} is not one definition; re-review the owner it prices")
    indent = hits[0].group(1)
    end = text.find("\n" + indent + "}\n", hits[0].start())
    if end < 0:
        raise SystemExit(f"W1R owners: no end of fn {name}")
    return text[hits[0].start():end + len(indent) + 3]


def digests(snapshot):
    """The current digests (for review and re-pinning; not a pass condition)."""
    out = {}
    for (rel, name) in BODY_PINS:
        text = _read(snapshot, rel)
        body = _body(text, name) if text is not None else None
        out[f"{rel}:{name}"] = hashlib.sha256(body.encode()).hexdigest() if body else None
    text = _read(snapshot, FKF) or ""
    if text.count(PROOF_START) == 1 and PROOF_STOP in text:
        a = text.index(PROOF_START)
        out["proof"] = hashlib.sha256(text[a:text.index(PROOF_STOP, a)].encode()).hexdigest()
    else:
        out["proof"] = None
    return out


def inspect(snapshot):
    """True when the repair's final-scales owner is present with its reviewed digests; False when the proof has no
    final_scales field (an earlier revision); anything else stops."""
    fkf = _read(snapshot, FKF) or ""
    if FIELD not in fkf:
        if "final_scales" in fkf or "final_scales" in (_read(snapshot, RW) or ""):
            raise SystemExit("W1R-FS: final_scales is partly present; re-review the owner it prices")
        return False
    current = digests(snapshot)
    for (rel, name), pin in BODY_PINS.items():
        if current[f"{rel}:{name}"] != pin:
            raise SystemExit(f"W1R owners: {rel} fn {name} changed or is missing; re-review the owner it prices")
    if current["proof"] != PROOF_PIN:
        raise SystemExit("W1R-FS: CertifiedProductProof changed; re-review the owner it prices")
    # The exact statements the formula prices, each once.
    for rel, needle in (
            (FKF, "let mut scales = reserve(nb)?;"),
            (FKF, "spent.capacities[6] = scales.capacity();"),
            (FKF, "    Ok(scales)\n}"),
            (FKF, "let scales=check_intervals("),
            (FKF, "Ok(final_scales)=>Ok(CertifiedProductProof{work:owned,anchor:self.data.anchor,final_scales}),"),
            (FKF, FIELD),
            (RW, "let scales = certificate.final_scales();"),
            (RP, "pub(super) struct FrozenCase { payload: PreparedPayload, certificate: k::CertifiedProductProof }"),
            (RP, "pub(super) struct FrozenCombination { values: k::FrozenProductValues, certificate: k::CertifiedProductProof }")):
        text = _read(snapshot, rel)
        if text is None or text.count(needle) != 1:
            raise SystemExit(f"W1R owners: {needle!r} is not one site in {rel}; re-review the owner it prices")
    return True


def proof_scales(b):
    """W1R-FS (T14, per attempted case, live through W2-W5 with the other T12-T15 owners): the final scales' backing,
    one [f64;4] per body (exact reserve at body_count <= b = n), held by the certified proof to serialization, and the
    proof's Vec header in each of its holders."""
    return Xc("f64", 4 * b) + s("Vec") * PROOF_HOLDERS
