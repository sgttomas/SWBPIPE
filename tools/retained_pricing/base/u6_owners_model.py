"""T4-U6/U8 D3: resident owners the D-5/D-6 source adds outside the chain's TEXT inventory, priced positively
(retained-pricing-rules.md, "T4-U6/U8: actual-source D3": U6-ES, U6-WS, U6-RS, U8-FM). Each owner is detected
by its exact source text and a half-changed form stops the run; a snapshot without it adds nothing, so earlier
registered trees keep their coefficients. No zero and no reuse credit is taken."""
import os
import runpy

LIB = "core/product_physics/src/lib.rs"
FC = "core/solver/frame_kernel/src/structural/formation_check.rs"
RPX = "core/reporting/result_export/src/retained_precision.rs"
# U8-MP: lib.rs member_prefix formats "member:{len}:{member}:" of an admitted pipe id: 7 + <= 3 + 1 + 128 + 1 B.
MEMBER_PREFIX = 140
# U6-WS: one stress site (&str, usize, bool, &StressRecoveryResult, Option<&StressRecoveryResult>) <= 48 B, and one
# stress-finding basis entry (&StressRecoveryResult, Option<SectionBasis>) <= 16 B, on a 64-bit target.
STRESS_SITE = 48
FINDING_BASIS = 16


def _wrapper_copies(text, label, name, head, copy_line, borrow_line, tail, helper, helper_text, wrapper_type):
    """True if fn `name` copies its payload into json!'s new Value (head + copy_line + tail), False if it serializes
    the borrowed wrapper (head + borrow_line + tail, the helper's exact text and the one wrapper struct, each name
    used once beside its definition); any other form stops the run."""
    if text.count(f"fn {name}(") != 1:
        raise SystemExit(f"{label}: {name} is not one definition; re-review its hash route")
    if head + copy_line + tail in text:
        return True
    if (head + borrow_line + tail in text and helper_text in text and wrapper_type in text
            and text.count("DomainPayload") == 2 and text.count(helper + "(") == 2):
        return False
    raise SystemExit(f"{label}: {name} changed; re-review its hash route")


def domain_hash_copies(retained_wire_text):
    """U6-DH: retained_wire.rs `domain_hash` (T16's three hashes)."""
    return _wrapper_copies(
        retained_wire_text, "U6-DH", "domain_hash",
        "pub(super) fn domain_hash(domain: &str, payload: &Value) -> Option<String> {\n",
        "    let text = serde_json::to_string(&json!({\"domain\":domain,\"payload\":payload})).ok()?;\n",
        "    let text = domain_text(domain, payload)?;\n",
        "    canonical_json_checked_v1_text(&text).ok().map(|canon| sha_hex(canon.as_bytes()))\n}\n",
        "domain_text",
        "pub(super) fn domain_text(domain: &str, payload: &Value) -> Option<String> {\n"
        "    serde_json::to_string(&DomainPayload { domain, payload }).ok()\n}\n",
        "#[derive(serde::Serialize)]\nstruct DomainPayload<'a> {\n    domain: &'a str,\n    payload: &'a Value,\n}\n")


def receipt_hash_copies(source_receipt_text):
    """U6-SH: source_receipt.rs `hash` (every T25 hash route)."""
    return _wrapper_copies(
        source_receipt_text, "U6-SH", "hash",
        "fn hash(domain: &'static str, payload: &Value) -> Result<String, ReceiptError> {\n",
        "    let text = checked(&json!({\"domain\":domain,\"payload\":payload}))?;\n",
        "    let text = hash_text(domain, payload)?;\n",
        "    Ok(format!(\"{:x}\", Sha256::digest(text.as_bytes())))\n}\n",
        "hash_text",
        "fn hash_text(domain: &str, payload: &Value) -> Result<String, ReceiptError> {\n"
        "    let text = serde_json::to_string(&DomainPayload { domain, payload }).map_err(|e| bad(e.to_string()))?;\n"
        "    canonical_json_checked_v1_text(&text).map_err(bad)\n}\n",
        "#[derive(Serialize)]\nstruct DomainPayload<'a> {\n    domain: &'a str,\n    payload: &'a Value,\n}\n")


def _read(snapshot, rel):
    with open(os.path.join(snapshot, rel), encoding="utf-8") as stream:
        return stream.read()


def _production(text):
    return text.split("#[cfg(test)]", 1)[0]


def _one(flag_texts, text, label):
    """True when every text occurs exactly once, False when none occurs; anything else stops."""
    counts = [text.count(t) for t in flag_texts]
    if all(c == 1 for c in counts):
        return True
    if all(c == 0 for c in counts):
        return False
    raise SystemExit(f"{label}: the source changed; re-review the owner it prices: {counts}")


def inspect(snapshot):
    # lib.rs declares its test modules near the top (`#[cfg(test)] mod ..;`), so its whole text is read; each
    # checked text below is production code and occurs once.
    prod = _read(snapshot, LIB)
    out = {}
    # U6-ES: BuiltModel.exact_sections, one entry per built pipe (build_model_for_members inserts it in its loop over
    # the pipes, under is_exact; U6-L1 checks the two inserts).
    out["exact_sections"] = _one(["    exact_sections: HashMap<String, pressure_exact::ExactSectionBases>,\n"], prod, "U6-ES")
    if out["exact_sections"] and prod.count("exact_sections.insert(") != 2:
        raise SystemExit("U6-ES: build_model_for_members' exact_sections inserts changed; re-review")
    # U6-WS: the case solve's reduced stresses, stress sites and stress-finding bases, and the reduced spans held
    # beside the nominal spans.
    out["reduced_workspace"] = _one([
        "        let reduced_stresses = match &reduced_section {\n",
        "        let mut stress_sites = vec![\n",
        "            let bases = match reduced {\n",
        "                                    reduced_spans = certified_publication::straight_stress_spans(",
    ], prod, "U6-WS")
    # U8-FM: FormationSource.frame_members and the PP identities it is filled from (member prefixes, <= one per frame).
    fc = _production(_read(snapshot, FC))
    out["frame_members"] = _one(["    pub frame_members: Vec<String>,\n"], fc, "U8-FM")
    if out["frame_members"] and (prod.count("fn frame_member_identities(built: &BuiltModel) -> Vec<String> {") != 1
                                 or prod.count("members.push(member_prefix(&pipe.element_id))") != 1
                                 or prod.count("let mut members = Vec::with_capacity(built.frame_elements.len());") != 1
                                 or 'format!("member:{}:{member}:", member.len())' not in prod):
        raise SystemExit("U8-FM: frame_member_identities or member_prefix changed; re-review")
    # U6-RS: NumericCase.all_rows (prescribed_key_model pins the struct, numeric_owner and g5c) and g5c's rebuild.
    keys = runpy.run_path(os.path.join(os.path.dirname(os.path.abspath(__file__)), "prescribed_key_model.py"))
    out["all_rows"] = bool(keys["all_rows_field"](snapshot))
    if out["all_rows"]:
        keys["borrowed"](snapshot)
        rp = _production(_read(snapshot, RPX))
        if rp.count("let bound: Vec<RowClassification> = classes.drain(first..).collect();") != 1 \
                or rp.count("last = Some(class.clone());") != 1:
            raise SystemExit("U6-RS: g5c's twin rebuild changed; re-review")
    return out
