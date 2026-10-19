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


# S6-HS (T4-S6, retained-pricing-rules.md §8.8): the three publication-hash families stream the checked canonical text
# of the borrowed {domain, payload} into SHA-256 through open_pipe_stress_canonical_json::checked_v1_domain_canonical
# (no wrapper copy, no to_string text, no checked parse, no canonical text). The writer's production code, from
# validate_checked_value to the test module, is bound by its sha256; each site by its exact text.
CJ = "core/serialization/canonical_json/src/lib.rs"
CJ_START = "fn validate_checked_value(value: &Value) -> Result<(), String> {\n"
CJ_STOP = "\n#[cfg(test)]\nmod tests {\n"
CJ_PIN = "381e93064a6cfaab7077fed66849774226fbed5bc5e8c890cb7ea6b6c1327df5"
SB = "core/reporting/result_export/src/source_blocks.rs"
_STREAM_LINE = "&mut ByteSink(|bytes: &[u8]| hasher.update(bytes))"
STREAM_DH = ("pub(super) fn domain_hash(domain: &str, payload: &Value) -> Option<String> {\n"
             "    let mut hasher = Sha256::new();\n"
             "    checked_v1_domain_canonical(domain, DomainPayload::Value(payload), " + _STREAM_LINE + ").ok()?;\n"
             "    Some(format!(\"{:x}\", hasher.finalize()))\n}\n")
STREAM_SH = ("fn hash(domain: &'static str, payload: &Value) -> Result<String, ReceiptError> {\n"
             "    let mut hasher = Sha256::new();\n"
             "    checked_v1_domain_canonical(domain, DomainPayload::Value(payload), " + _STREAM_LINE + ")\n"
             "        .map_err(bad)?;\n"
             "    Ok(format!(\"{:x}\", hasher.finalize()))\n}\n")
STREAM_RS = ("pub fn domain_hash(domain: &str, payload: &Value) -> Result<String, String> {\n"
             "    domain_digest(domain, DomainPayload::Value(payload))\n}\n",
             "pub fn domain_hash_without(domain: &str, object: &serde_json::Map<String, Value>, omit: &str) -> Result<String, String> {\n"
             "    domain_digest(domain, DomainPayload::ObjectWithout(object, omit))\n}\n",
             "fn domain_digest(domain: &str, payload: DomainPayload<'_>) -> Result<String, String> {\n"
             "    let mut hasher = Sha256::new();\n"
             "    checked_v1_domain_canonical(domain, payload, " + _STREAM_LINE + ")?;\n"
             "    Ok(format!(\"{:x}\", hasher.finalize()))\n}\n")
FORMER_RS = ("pub fn domain_hash(domain: &str, payload: &Value) -> Result<String, String> {\n"
             "    let value = json!({\"domain\":domain,\"payload\":payload});\n")
BORROW_G1 = ("        need(\n            hash_without(\n                \"retained_precision_publication_mp_v2\",\n"
             "                source,\n                \"retained_precision\",\n")
HASH_WITHOUT = ("fn hash_without(domain: &str, v: &Value, omit: &str, gate: &'static str, code: &str) -> VResult<String> {\n"
                "    let object = v.as_object().ok_or_else(|| error(gate, code))?;\n"
                "    crate::source_blocks::domain_hash_without(domain, object, omit).map_err(|_| error(gate, code))\n}\n")
FORMER_G1 = "        let mut public = source.clone();\n"


def canonical_writer(snapshot):
    """S6-HS: the canonical writer's production code is the reviewed one (a changed writer stops the run)."""
    import hashlib
    text = _read(snapshot, CJ)
    if text.count(CJ_START) != 1 or text.count(CJ_STOP) != 1:
        raise SystemExit("S6-HS: canonical_json's checked writer moved; re-review the streamed hash route")
    span = text[text.index(CJ_START):text.index(CJ_STOP)]
    if hashlib.sha256(span.encode()).hexdigest() != CJ_PIN:
        raise SystemExit("S6-HS: canonical_json's checked writer changed; re-review the streamed hash route")


def _streams(text, label, name, streamed, retired):
    """True if fn `name` is the exact streamed form (each `retired` text gone), else None (the caller's former check)."""
    if text.count(f"fn {name}(") != 1:
        raise SystemExit(f"{label}: {name} is not one definition; re-review its hash route")
    if streamed in text:
        if any(r in text for r in retired):
            raise SystemExit(f"{label}: {name} streams but a former wrapper remains; re-review its hash route")
        return True
    return None


def domain_hash_streams(snapshot, retained_wire_text):
    """S6-HS at T16: True when retained_wire.rs `domain_hash` streams; False for U6-DH's former forms; else stops."""
    if _streams(retained_wire_text, "S6-HS", "domain_hash", STREAM_DH, ("fn domain_text(", "struct DomainPayload")):
        canonical_writer(snapshot)
        return True
    domain_hash_copies(retained_wire_text)
    return False


def receipt_hash_streams(snapshot, source_receipt_text):
    """S6-HS at T25: True when source_receipt.rs `hash` streams; False for U6-SH's former forms; else stops."""
    if _streams(source_receipt_text, "S6-HS", "hash", STREAM_SH, ("fn hash_text(", "struct DomainPayload")):
        canonical_writer(snapshot)
        return True
    receipt_hash_copies(source_receipt_text)
    return False


def reader_hash_streams(snapshot):
    """S6-HS at T17: (streams, borrows the publication). result_export source_blocks.rs `domain_hash` streams (the
    former json! copy otherwise), and retained_precision.rs G1 hashes the source less its receipt through
    `hash_without` (the former clone otherwise). Any other form stops."""
    sb = _read(snapshot, SB)
    if all(sb.count(t) == 1 for t in STREAM_RS) and FORMER_RS not in sb:
        canonical_writer(snapshot)
        streams = True
    elif sb.count(FORMER_RS) == 1 and "fn domain_digest(" not in sb:
        streams = False
    else:
        raise SystemExit("S6-HS: result_export domain_hash changed; re-review its hash route")
    rp = _production(_read(snapshot, RPX))
    if rp.count(BORROW_G1) == 1 and rp.count(HASH_WITHOUT) == 1 and FORMER_G1 not in rp and streams:
        borrows = True
    elif rp.count(FORMER_G1) == 1 and "fn hash_without(" not in rp:
        borrows = False
    else:
        raise SystemExit("S6-HS: retained_precision G1's publication hash changed; re-review its hash route")
    return streams, borrows


def stream_route(vf, lmax, eps):
    """S6-HS: what a streamed hash holds beside its borrowed payload: the per-string `serde_json::to_string`
    temporary of the longest string (to_vec starts at 128 B and doubles: <= max(128, 2(eps*lmax + 2)), one at a
    time), the number-rendering temporaries and a retained depth*128 B frame allowance (<= depth*8*16 + 144 + 128 B,
    as the former route charged), and the per-object sorted key vectors (with_capacity(len) of 8-byte references,
    live along the current path; charged for every entry at s(&str) = 16 B). The 16 B per entry covers the 8 B key
    reference and up to 8 B of `keys.sort_by`'s stable-sort scratch: std's stable sort sorts len <= 512 references
    in its 4 KiB stack buffer and otherwise allocates one heap scratch of at most len references (8 B each), freed
    when `sorted_keys` returns, before the writer descends; so at most one scratch is live, and it is within the
    charge of the object being sorted. SHA-256 state is on the stack. Nothing else allocates: no wrapper, text,
    parse tree, seen set, escape scratch or canonical output. No coefficient changes."""
    from g3lib import B, s
    return B(max(128, 2 * (eps * lmax + 2))) + B(vf.depth * 8 * 16 + 144 + 128) + s("&str") * vf.ent


def stream_last_growth(lmax, eps):
    """S6-HS: the largest old backing of a streamed hash, the string temporary's previous backing at its last doubling."""
    from g3lib import B
    return B(max(128, 2 * (eps * lmax + 2)) // 2)


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
