"""O4 S5 D3 (the Timoshenko and corrosion-allowance source extension): heap owners the O4 source adds outside the chain's
TEXT inventory, priced positively (retained-pricing-rules.md §8.9: O4-PS, O4-PA, O4-ID, O4-RS, O4-NC, O4-PP, O4-KS,
O4-TR, O4-WR, O4-RD, O4-SR, O4-SP). Each owner is bound to the sha256 of the function bodies (text from `fn <name>`
to its closing brace at the same indentation) or the exact source span that allocates it; every file is also under
an existing full-file pin. A snapshot without the O4 functions adds nothing, so earlier registered trees keep their
coefficients; a partly present or changed body stops the run (exit 4) for review. No zero and no reuse credit is
taken: every owner is charged at its push-grown capacity, with the previous backing of its last reallocation, in
every phase where it can be live, at the cap with every member Timoshenko and every member's bases distinct. Inline
growth of the kernel's MemberOperators, BoundedCoefficients and ProductCertificateSpent (the D/D-bar entries, the
reduced capacities) is already priced by their in-build strides (g5_profile.py binds the kernel exports)."""
import hashlib
import json
import os
import re

from g3lib import X, B, s, Xc, V, HashMov, pushcap, tree_nodes, ASSUMED

RP = "core/product_physics/src/retained_product.rs"
RW = "core/product_physics/src/retained_wire.rs"
LIB = "core/product_physics/src/lib.rs"
SRC = "core/product_physics/src/source_receipt/source.rs"
ROWS = "core/product_physics/src/source_receipt/rows.rs"
RPX = "core/reporting/result_export/src/retained_precision.rs"
PHS = "core/reporting/result_export/src/physics_source.rs"
FKS = "core/solver/frame_kernel/src/structural/retained/source.rs"
FKA = "core/solver/frame_kernel/src/structural/retained/adaptive.rs"
FKO = "core/solver/frame_kernel/src/structural/retained/origins.rs"
FKF = "core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs"
CENSUS = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "data", "census_u4b_7c77bf8072.json")

# W-15 D3 (§8.21): solve_prepared_load_case keeps a body's maxima out of the headline, reads a body's PF nu from the
# unscaled pair and names `component_body: None` on its rows; the second composite maximum it prices (O4-SR) is unchanged.
BODY_PINS = {
    # O4-PS (capture: exact_members, distinct_bases) and O4-PA (prepare: leaves, reduced_facts, reduced preparations,
    # shear associations, the invocation's TimoshenkoLeafSet and reduced facts)
    (RP, "capture_case_structure"): "4988b70e7815c7b9b56c7e78640895d1d782dddd070301084aaeea7e7fc403b8",
    (RP, "prepare_active_case"): "d4b7aae394232031d105f4a5b531c6e0c272b56aa72a857cee5cfd9d05b914ef",
    # O4-RS (the ReducedSectionSet), O4-NC (the reduced operand table and bound bitmap), O4-PP (distinct copy, the
    # reduced maximum outputs and the payload's reduced sections and shear leaves)
    (RP, "reduced_section_set"): "298ecb9242a154133aa1d3a45510999d81e48778bccc53830a9906387b33e6fc",
    (RP, "distinct_copy"): "951b22a65be7c1590c2f01c772234eb22655a865a52fc9068fc977b28205ba6c",
    # W-7 D3 (§8.20): prepared_not_covered adds the end-action table (W7-NC); the reduced table and bound bitmap are unchanged.
    (RP, "prepared_not_covered"): "697dd47f3ab31b2f8b7aca8b1711426dad26cc4411187f30091afa453bec710c",
    (RP, "prepared_maxima"): "10f81029578641e0c05914cab95b3a8cfefc0a45cb9b21fd94fbd27aba8adaa0",
    (RP, "prepared_sections"): "9b5b2b6a6094db92c72790e22d7c956f2a5215ae08e25aa3990f32a38eed9ed2",
    # Stack-headroom repair D3 (b7-plan 5a, retained-pricing-rules.md §8.10): read 27e4dfa972;
    # freeze_case's closure is split into six phase functions, with unchanged calls, order, errors,
    # drops and allocation. Only stack placement changes; no owner this model prices changes.
    (RP, "freeze_case"): "4fd4525e76ae31f8babbe6384cfd56568018a44dddacece26e056070555cd0da",
    (RP, "freeze_case_inputs"): "7cc89da34fefbaff43cc1ab013045dc0756621591cdd4f0f530d3bc8732bb165",
    (RP, "freeze_case_draft"): "ee1cb4dce82b6d404ad0a407efca45baf6a48f91f4c8737d00026faf41ac6b1f",
    (RP, "freeze_case_values"): "4507cd2ad855850250a1b355b7216b778ad8983c957faeb860d5c9ebac0a99ad",
    (RP, "freeze_case_certify"): "6866fbb15b0d5b976c7fa2cb53649dbed24f6fbc2c7f8a4c6b14a5c1998f366c",
    (RP, "freeze_case_settle"): "fbead169403126ea9207e5fc3bba733a3a912324cb3a9c420ae4fa40d00fb430",
    (RP, "freeze_case_commit"): "97088782f6b5a7469ec7cf15198bf18ba3d1dce6670cb0e2f891e50e85ba9924",
    # O4-WR (the receipt's section-term keys and reduced preparation records)
    (RW, "case_source_at"): "28f05373993894b3a4df7664dfdc2f410f80be81fd87604b325dc0a89dc83679",
    # W1 coverage repair D3 (retained-pricing-rules.md §8.11): read 3bbfdd5080; selection borrows the certified proof
    # and writes body_scales from its final scales (W1R-FS, W1R-L1). Its section terms, the keys O4-WR prices, are
    # unchanged.
    (RW, "selection"): "4603dee55a5f4d98bd56d21dc1869bc0c66626d70060ee98ba7e8132e5b616cb",
    (RW, "reduced_preparation_value"): "af3226f2fa85205f1a231d450599252d553a94d68942e6d386e1e8d125e0cb41",
    (RW, "attach_reduced_preparations"): "ae61df5566e415ddce9363e18590b2b93632620e292e2eb4f3c8ab41f00976ea",
    # O4-RD (the reader's distinct-member sets, the Timoshenko leaves and block bytes)
    (RPX, "rows_for"): "0de430a098ccc641c696d953385da2dffb73829a4f7391c825164e6a51ca6f84",
    (RPX, "distinct_members_of"): "598d7ae3646832e6c1ca8e55ff8ff2f4eaa5f80641348ffd5cdd25cf6cdb3304",
    (RPX, "native_bytes"): "b29f2a8e08c9b1129d6c0667216d233f5b15bb5917d2443de415577d2299ca57",
    # W-16 slice c D3 (§8.27, W16C-ES): an ordinary companion's equivalent_static record is checked under
    # `exact.get("equivalent_static")`, absent on D1 (D1.5 refuses the generation); the owner it prices is unchanged.
    (PHS, "validate_transport_metadata_in"): "9000648e78a185e739811f0647aa600cd499ebe1a5419c320decfb0ffb9ce56a",
    # O4-SR (the source route's second maximum) and O4-SP (its payload keys and per-station components)
    # Pre-0.4 failure unification D3 (§8.14): the screen and withheld decline on both exact routes, the support-norm
    # cause String and the route-neutral cause records (PU-TX). The second maximum and the payload keys are unchanged.
    # W-7 D3 (retained-pricing-rules.md §8.20): solve_prepared_load_case appends the member's global end rows or one blocking diagnostic (W7-GE); the second composite maximum it prices is unchanged.
    # W-16 slice a D3 (retained-pricing-rules.md §8.23): a pressure-free case (no load state) forms its support-targeted
    # primitives' prescribed map (imposed_motion::case_prescribed, W16-NI) and returns an empty LoadCaseSolve on a
    # blocking finding there (none on D1: D1.7 refuses the support target, W16-NS); the prescribed values read that
    # map when present. The second composite maximum and its payload keys are unchanged.
    # W-16 slice c D3 (§8.27, W16C-ES): the case evidence's equivalent_static record, formed only under
    # `load_case.equivalent_static.is_some()` (none on D1: D1.5). The second composite maximum and its payload keys are
    # unchanged.
    # W-16 stage 2 D3 (§8.29): the pressure-free case's spring-base vector and formed k·g ledger terms (W16S-SG), the
    # moved nonlinear grounds and their relative-displacement rows (W16S-L1, none on D1), the spring's -k·(u - g)
    # action and the ground vector passed to certified_publication::support. The second composite maximum and its
    # payload keys are unchanged.
    # RT-A3 increment 1 D3 (§8.33): a straight exact member's branch rows (u4b_rows::append_branch_intensified, under
    # `exact && macro_bend.is_none()`; none on D1, D1.4); O4-ID/O4-SR's Timoshenko blocks and maxima are unchanged.
    # RT-A3 increment 1b D3 (§8.36): the case solve's published-row index (u4b_rows::PublishedScope before the
    # member loop, its `begin` at each member's start and a MemberRows passed to the station quantities and branch
    # rows; RTA3B-PS, RTA3B-MR). No `?`, return, record, label, carrier or staging is added. O4-ID/O4-SR's Timoshenko blocks and maxima are unchanged.
    (LIB, "solve_prepared_load_case"): "1e9343f49b5b53d51db544c5148f38e5fb1839954c5c94f377f9a0716bf45327",
    (SRC, "distinct_reduced_section"): "179e447476ae2a7cf8b01d1e5da22bddd66b9057403453ffc79f9e57747fd100",
    (SRC, "stress_operands"): "de7a26eb0574635be8c808bc42070cdb132f8f8b67fee83542bd79f667cb6050",
    (SRC, "commitment"): "279ce112036664d7802e2952d87a67abe6dd7fe04879a4d498fd2544fe86753d",
    (ROWS, "derived_for"): "9b80307c2f233c50c6c9d0cb205d29c774a4550ec676070a7016136d205799f8",
    # O4-KS (kernel sidecar, identity block, ReducedSectionSet maps): the bodies that allocate them
    (FKO, "solve_timoshenko_cases"): "1c949e3cd60a8adb1f46cdb574b241fa3f089b89b1710050152daab82e3f45ae",
    (FKF, "reduced_index"): "0ff5eaec24cb7d64f50ee6155100bd817531481703c71c5a3597a348d2cff16a",
    # W-7 D3 (retained-pricing-rules.md §8.20): row_scales_reduced admits a not-covered force or moment row in a combination (a predicate); no owner changes.
    (FKF, "row_scales_reduced"): "3322775097f9f9ab59aac76ff2b38bf3dfa3769a76debebe8b155d9403068d26",
}
# FK source.rs from the TimoshenkoLeaf doc comment to the Station doc comment: the leaf, its error, the validated set
# (new: one sorted Vec; validate: no allocation; encode_block: tag + count + 12-byte entries) and the two constants.
SIDECAR_START = "/// O4 (DEL-04-08 Design `o4-source-extension.md` §4.2): one Timoshenko"
SIDECAR_STOP = "/// A station on a member at a binary64 fraction of its chord (0 at node i)."
SIDECAR_PIN = "f9ec34fd496c591c5c6d1a73f6d0aab9126175ed72313170b9e8d27723d71c63"
# FK final_case.rs's ReducedSection/ReducedSectionSet declarations and ReducedSectionSet::new (sections sorted by a
# stable sort, the `seen` row-index vector), up to the ProductRecipe layout pin.
RSS_START = "/// O4 (DEL-04-08 Design `o4-source-extension.md` §4.5): one member's reduced"
RSS_STOP = "// O4 §4.5: ProductRecipe carries no basis tag; its receipt-priced layout is"
RSS_PIN = "b4807142c01a110919d2fa175016b4e4a66c75919906e31d5cd14068f8919ac0"
# FK adaptive.rs CasePrep::new_with_shear: the CasePrep clone of the set and the replaced K4SRC identity.
PREP_START = "    pub(crate) fn new_with_shear(\n"
PREP_STOP = "    /// The factor-reuse identity of this case: K4STF with the Timoshenko"
PREP_PIN = "f59add62ed16366651c18b9394da187de9f5bfcae8badb33dfe2ed5fc2bb1d9d"

# Rows of one distinct-basis member that the ReducedSectionSet maps: its 20 reduced element-local stress rows (five
# sites x four components) and its reduced maximum (final_case.rs: `n = sections.len() * 21`).
REDUCED_ROWS_PER_MEMBER = 21
# The Timoshenko block of K4SRC/K4STF (FK encode_block, RS native_bytes): tag `TSH\x01` (4) + u32 count (4) + per
# leaf u32 member + u64 shear-area bits (12).
BLOCK_FIXED, BLOCK_PER_LEAF = 8, 12
# One kernel-stack staging allowance for the new per-member transients (MemberEnclosures' Option<ShearEnclosures>
# (2 enclosures + 1 endpoint), source_residual Member's t/tgap (4 enclosures + 4 endpoints), bridge Majorants' t (8
# endpoints), timoshenko_entries' six Wide<L> and their temporaries): 64 Wide<16> slots, no compiler reuse assumed.
KERNEL_STAGING_WIDES = 64

ASSUMED.update({
    # illustrative only: g5_profile binds each to the actual type's size_of in the PP build
    "s(ExactMemberBasis)": 48, "s(ReducedFacts)": 56, "s(ReducedPreparation)": 2048, "s(ShearAssociation)": 24,
    "s(TimoshenkoLeaf)": 16, "s(ReducedSection)": 72, "s(PreparedSectionPatch)": 160, "s((usize,Number))": 40,
    "s(StressComponents)": 64, "s(CompositeMaximum)": 40, "s(O4KernelStaging)": 9216, "s((u64,&Value))": 16,
})


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
        raise SystemExit(f"O4 owners: {name} is not one definition; re-review the owner it prices")
    indent = hits[0].group(1)
    end = text.find("\n" + indent + "}\n", hits[0].start())
    if end < 0:
        raise SystemExit(f"O4 owners: no end of fn {name}")
    return text[hits[0].start():end + len(indent) + 3]


def _span(text, start, stop, label):
    if text is None or start not in text:
        return None
    if text.count(start) != 1:
        raise SystemExit(f"{label}: the span start is not one site; re-review the owner it prices")
    a = text.index(start)
    e = text.find(stop, a)
    if e < 0:
        raise SystemExit(f"{label}: the span end moved; re-review the owner it prices")
    return text[a:e]


def digests(snapshot):
    """The current digests (for review and re-pinning; not a pass condition)."""
    out = {}
    for (rel, name) in BODY_PINS:
        text = _read(snapshot, rel)
        body = _body(text, name) if text is not None else None
        out[f"{rel}:{name}"] = hashlib.sha256(body.encode()).hexdigest() if body else None
    for label, rel, start, stop in (("sidecar", FKS, SIDECAR_START, SIDECAR_STOP), ("reduced_set", FKF, RSS_START, RSS_STOP),
                                    ("case_prep", FKA, PREP_START, PREP_STOP)):
        span = _span(_read(snapshot, rel), start, stop, label)
        out[label] = hashlib.sha256(span.encode()).hexdigest() if span else None
    return out


def inspect(snapshot):
    """True when every O4 owner body and span exists with its reviewed digest; False when no O4 marker exists (an
    earlier revision); anything else stops."""
    sidecar = _span(_read(snapshot, FKS), SIDECAR_START, SIDECAR_STOP, "O4-KS")
    rp = _read(snapshot, RP) or ""
    present = sidecar is not None or "fn reduced_section_set(" in rp
    if not present:
        return False
    current = digests(snapshot)
    for (rel, name), pin in BODY_PINS.items():
        if current[f"{rel}:{name}"] != pin:
            raise SystemExit(f"O4 owners: {rel} fn {name} changed or is missing; re-review the owner it prices")
    if current["sidecar"] != SIDECAR_PIN:
        raise SystemExit("O4-KS: the Timoshenko sidecar block changed; re-review the owner it prices")
    if current["reduced_set"] != RSS_PIN:
        raise SystemExit("O4-RS: ReducedSectionSet changed; re-review the owner it prices")
    if current["case_prep"] != PREP_PIN:
        raise SystemExit("O4-PA: CasePrep::new_with_shear changed; re-review the owner it prices")
    # The exact allocating statements the formulas below price (each once in its pinned body).
    for rel, needle in (
            (RP, "Ok(v) => exact_members = v,"),
            (RP, "self.adapter.push_room(&mut distinct_bases)?;"),
            (RP, "let mut leaves:Vec<k::TimoshenkoLeaf>=if timoshenko>0 {self.adapter.reserve(timoshenko)?} else {Vec::new()};"),
            (RP, "let mut sections=self.adapter.reserve::<k::ReducedSection>(self.reduced_facts.len())?;"),
            (RP, "let mut indices=self.adapter.reserve::<u32>(21)?;"),
            (RP, "reduced_operands=self.adapter.reserve(self.reduced_facts.len())?;"),
            (RP, "reduced_bound=self.adapter.reserve(rows.len())?;reduced_bound.resize(rows.len(),false);"),
            (FKA, "prep.timoshenko = Some(set.clone());"),
            (FKA, "prep.identity = source.encoding_with(Some(set));"),
            (FKF, "let mut map=reserve::<u32>(n)?;"),
            (FKF, "let mut v = reserve::<bool>(n)?;"),
            (RPX, ".filter_map(|st| st.get(\"shear_area\").map(|a| (u(&st[\"member\"]), a)))"),
            (ROWS, "let mut components = Vec::with_capacity(2);")):
        text = _read(snapshot, rel)
        if text is None or text.count(needle) != 1:
            raise SystemExit(f"O4 owners: {needle!r} is not one site in {rel}; re-review the owner it prices")
    if REDUCED_ROWS_PER_MEMBER != 21 or "checked_mul(21)" not in _read(snapshot, FKF):
        raise SystemExit("O4-RS: the reduced rows per member changed; re-review")
    return True


# ---------------------------------------------------------------- producer (T11-T15)
def identity_block(m):
    """O4-ID: the bytes a non-empty Timoshenko sidecar adds to each K4SRC and K4STF identity (E_src, E_stf)."""
    return BLOCK_FIXED + BLOCK_PER_LEAF * m


def capture_sidecars(m, ident):
    """O4-PS (T11, invocation-owned): ProductCapture's exact_members (adapter reserve of m, exact), distinct_bases (<= m
    adapter-copied ids, push-grown with its previous backing), the TimoshenkoLeafSet (m leaves) and reduced_facts (m),
    plus one equal capture-local copy of each (built per case before the equality check against the invocation's)."""
    one = (Xc("ExactMemberBasis", m) + V("String", m) + s("String") * (pushcap(m) // 2) + B(m * ident)
           + Xc("TimoshenkoLeaf", m) + Xc("ReducedFacts", m))
    return one * 2


def prepare_sidecars(m, e_src):
    """O4-PA (T12, per attempted case): prepare_active_case's leaves and reduced_facts locals (each reserved at m), the
    attempt's reduced preparations and shear associations (each reserved at m, held in AttemptParts to serialization),
    the CasePrep's cloned TimoshenkoLeafSet, and the replaced K4SRC identity live while new_with_shear computes its
    successor (one more E_src)."""
    return (Xc("TimoshenkoLeaf", m) + Xc("ReducedFacts", m) + Xc("ReducedPreparation", m) + Xc("ShearAssociation", m)
            + Xc("TimoshenkoLeaf", m) + B(e_src))


def proof_owners(m, p_final, ident):
    """O4-RS, O4-NC, O4-PP, O4-KS (T14, per attempted case), at m distinct-basis members:
    - the ReducedSectionSet: m sections (adapter reserve, exact) with 21 row indices each (exact), the stable sort's
      scratch (<= m sections), and ReducedSectionSet::new's `seen` vector (extend_from_slice growth to <= 21m with the
      previous backing);
    - the reduced-row index maps (reserve::<u32>(n), n = the case's final rows <= P_final): the validity check's,
      the projection's and check_intervals', two live at once at most, charged twice; the reduced-coverage bitmaps
      (21m bools), charged twice;
    - prepared_not_covered's reduced operand tables (m) and its bound-position bitmap (P_final bools);
    - distinct_copy's adapter-copied ids (m, exact, ids <= 128 B);
    - the reduced maxima: m more outputs, slots and patches (MaximumE, the premise m -> 2m);
    - the frozen payload's reduced sections (m patches) and shear leaves (m), held until staging;
    - O4-KS: the kernel's new per-member stack values, one SourceUpper allowance."""
    h = REDUCED_ROWS_PER_MEMBER * m
    rss = (Xc("ReducedSection", m) + B(4 * REDUCED_ROWS_PER_MEMBER * m) + Xc("ReducedSection", m)
           + B(4 * pushcap(h) + 4 * (pushcap(h) // 2)))
    maps = B(2 * 4 * p_final + 2 * h)
    not_covered = Xc("NotCoveredOperands", m) + B(p_final)
    distinct = Xc("String", m) + B(m * ident)
    payload = Xc("PreparedSectionPatch", m) + Xc("(usize,Number)", m)
    return rss + maps + not_covered + distinct + V("MaximumE", m) + payload + s("O4KernelStaging")


def trace_owners(m):
    """O4-TR (T15, per attempted case): a distinct-basis member's reduced annulus preparation is a second preparation
    record, so the trace's per-member premises are charged once more (PreparedMemberEvent m -> 2m, its nine
    ConversionEvents 9m -> 18m)."""
    return V("PreparedMemberEvent", m) + V("ConversionEvent", 9 * m)


# ---------------------------------------------------------------- receipt (T16-T19, X2) and reader (T17)
def receipt_facts(m):
    """O4-WR: per attempted case, (source section terms, selection section terms, class entries, preparation records)
    as rec(count, slots, obj, ent, strb, nums) tuples for g4_caps' grammar:
    - each source section term gains `shear_area` (bits) and `reduced` {area, section_modulus, geometry {route,
      normalized_od, effective_wall, actual_radius, actual_second_moment, actual_polar_moment}}: 2 objects, 10 entries,
      a route token and 8 bits strings;
    - each selection section term gains `shear_area` and `reduced` {area, section_modulus}: 1 object, 4 entries, 3 bits;
    - the distinct members' 21 reduced rows each take a class entry (absolute_verified {result_id, class}) of their
      own; the row premise P already counts them as published rows, so this is a conservative second charge;
    - each member's preparation record gains `reduced`, at most a full second record of the member's grammar (its
      result, work and <= 9 conversions)."""
    return {
        "source_terms": dict(cnt=m, obj=2, ent=10, strb_tok=1, strb_bits=8),
        "selection_terms": dict(cnt=m, obj=1, ent=4, strb_bits=3),
        "class_entries": dict(cnt=REDUCED_ROWS_PER_MEMBER * m, slots=1, ent=2),
        "preparation_members": m,
    }


def reader_sets(m, attempted):
    """O4-RD (T17): rows_for's, g5c's and g5b_exact_evidence's distinct-member BTreeSets (<= m borrowed ids each, three
    charged at once), native_bytes' Timoshenko leaves (filter_map collect: push-grown with its previous backing, per
    source and stiffness encoding) and physics_source's per-case model set (<= m inserts, charged as m)."""
    return (X({"Node(&str,())": 3 * tree_nodes(m)}) + s("(u64,&Value)") * (2 * attempted * (pushcap(m) + pushcap(m) // 2))
            + HashMov("&str", m))


def reader_block_bytes(m):
    """O4-RD (T17 G8): native_bytes appends the same Timoshenko block (tag, count, 12 bytes per leaf) to each rebuilt
    K4SRC and K4STF encoding."""
    return identity_block(m)


# ---------------------------------------------------------------- source route (X phases) and ordinary recovery
# One source-selected maximum's evidence Value (composite.rs CompositeMaximum.evidence): one pipe_stress_extrema entry
# of the D-5 census (data/census_u4b_7c77bf8072.json: per member two entries, one per basis, {arr 74, obj 20, ent 92,
# strb 9728, keyb 1042, nums 82}), halved and rounded up. The chain's work directory holds no census copy, so the facts
# are stated here and tests/test_o4_owners_model.py ties them to the committed census.
MAXIMUM_EVIDENCE = {"arr": 37, "obj": 10, "ent": 46, "strb": 4864, "keyb": 521, "nums": 41}


def maximum_evidence_facts(census=None):
    """MAXIMUM_EVIDENCE, checked against the census when its path is given (tool tests)."""
    if census is not None:
        d = json.load(open(census))
        per_pipe = d["exact_evidence_case_parts"]["per_pipe_by_array"]["pipe_stress_extrema"]
        entries = d["per_pipe_entries"]["pipe_stress_extrema"]
        expect = {k: (v + entries - 1) // entries for k, v in per_pipe.items() if k != "depth"}
        if entries != 2 or expect != MAXIMUM_EVIDENCE:
            raise SystemExit("O4-SR: the census's maximum entry changed; re-review MAXIMUM_EVIDENCE")
    return dict(MAXIMUM_EVIDENCE)


def beam_theory_evidence(m):
    """O4-EV: a Timoshenko member's pipe_sections entry states `beam_theory` {model, shear_factor, shear_area_m2}, two
    number entries (25 key bytes) beyond the Euler-Bernoulli census entry; per case and member. G-C's bounds stay the
    census (the product gate, unchanged; it refuses beyond them); this is a positive charge beside the census owner."""
    return dict(ent=2 * m, keyb=(len("shear_factor") + len("shear_area_m2")) * m, nums=2 * m)


def source_payload_facts(m):
    """O4-SP (T25 commitment): each frame's formation inputs gain `shear_area_bits` and a distinct member's frame gains
    `reduced_recovery_section` {area, section_modulus, torsion_constant, torsion_radius} bits: per member one object,
    six entries, five 16-byte bit strings."""
    return dict(obj=m, ent=6 * m, strb=5 * 16 * m)


def source_plan_facts(m):
    """O4-SP (T25 plan): a distinct member's 21 reduced rows name `reduced_recovery_section` operand paths, longer than
    the nominal paths: one more 64-byte path string per reduced row (conservative)."""
    return dict(strb=REDUCED_ROWS_PER_MEMBER * m * 64)


def derived_components():
    """O4-SP (T25 rows::bind): derived_for's per-station Vec of the two bases' StressComponents (with_capacity(2)),
    one station at a time."""
    return Xc("StressComponents", 2)
