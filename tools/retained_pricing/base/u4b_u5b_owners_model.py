"""T4-S5 D3 (shared epoch 4): heap owners the T4-U4b/U5b source adds outside the chain's TEXT inventory, priced
positively (retained-pricing-rules.md §8.8: U5B-PD, U4B-ST, U5B-AB, U4B-W1, U4B-RD, U4B-NQ). Each owner is bound to
the exact function bodies (sha256 of the text from `fn <name>` to its closing brace at the same indentation) or
exact source lines that allocate it. A snapshot without the function adds nothing, so earlier registered trees keep
their coefficients; a changed body stops the run for review. No zero and no reuse credit is taken: every owner below
is charged at its push-grown capacity, with the previous backing of its last reallocation, in every phase where it
can be live. Realized arcs (directional, toroidal and arc-maximum owners) and connectors (tie rods) are absent on D1
by premises P2 and P3; their owners are empty there and are not charged."""
import hashlib
import os
import re

from g3lib import X, B, s, Xc, V, HashMov, pushcap, tree_nodes, ASSUMED

LIB = "core/product_physics/src/lib.rs"
U4B = "core/product_physics/src/u4b_rows.rs"
FKS = "core/solver/frame_kernel/src/structural.rs"
SPARSE = "core/solver/frame_kernel/src/structural/sparse.rs"
RP = "core/product_physics/src/retained_product.rs"
EVID = "core/reporting/result_export/src/u4b_evidence.rs"
PREV = "core/reporting/result_export/src/preview_physics_evidence.rs"
PHYS = "core/reporting/result_export/src/physics_evidence.rs"
ROWS = "core/product_physics/src/source_receipt/rows.rs"
NI = "core/solver/nonlinear_integration/src/structural_adapter.rs"

# O4 S5 (retained-pricing-rules.md §8.9, O4-NC): prepared_not_covered keeps U4B-W1's operand table (type, reservation,
# events) and adds a separate reduced table (one per distinct-basis member) and a bound-position bitmap of the case's
# rows, both only when a member's bases differ; those are O4's owners (o4_owners_model.py), not this model's.
BODY_PINS = {
    (LIB, "published_nodal_displacements"): "83648eba3d0101586f558439d1198591659ecd706db07b5447feed75421dbe02",
    (U4B, "member_bases"): "760afaf0e6145df8f837cb75c6976d87afd98b3d20f873789ab4427fd0c8e372",
    (U4B, "append_station_quantities"): "778a4751ce9402583307ce8ae1b3a153fe9b40ee483b601722d43786fe73f06a",
    (SPARSE, "sparse_contribution_sums"): "3a7bb0911b4646798f6cbc6f104f0b852ba5beaf3ef439d6535004b06c8f4a7a",
    # W-7 D3 (retained-pricing-rules.md §8.20): failed_manifest adds each member's twelve global end placeholders after
    # its maximum (rows inside FAILED_MEMBER_ROWS = 78, W7-RL; their texts are TEXT sites), and prepared_not_covered
    # adds its end-action table (W7-NC, w7_owners_model) and completes the global end rows; the Poisson flags, the
    # grown diagnostics and this operand table are unchanged.
    (RP, "failed_manifest"): "c67b66d832027a5da512947a8bad05355e68136974eb4e03cc0a1e6c462c6b1f",
    (RP, "prepared_not_covered"): "373fc109510882da46338c0a81da6e91b0eb9f5d4803dd2a1954be0a429bcd19",
    (EVID, "member_stations"): "422def17cd2b550f2654109416449ee04276fddd13a42175dcfe5606b7f93cad",
    (EVID, "poisson_exempt"): "087720dfa55869d73d7b277affd80e7dffb34a83f2488b85900387caae4f33d4",
    (EVID, "preview_model_checks"): "5580d56f7b5f41ec230d9b2d02aa84fc9dfcbb2a633f17a594985de9819ff29a",
    (PHYS, "u4b_cases"): "ef840d771fad2b50bef538f1e6b57466bc1902ba4b7c4406de53a9a31b5849ed",
    (ROWS, "not_qualified_for"): "c76bc3bdb40be7ca80e348223b7cf3a7373bd7d05f74013f93567020b0abf222",
    (ROWS, "not_qualified_inputs"): "14c9b4d161963de572fe32bcd9fc1c58191eebea441728054a624b998f97e4e3",
}
# U5B-AB: FK structural.rs from `pub(crate) struct ElementAbsorption<'g> {` to `fn contribution_differences(`.
ABSORPTION_PIN = "2acbe81068e36d74474cb2f7014f9dcbf65c0bdde660c783af7ced048534e5f4"
# U4B-RD: u4b_evidence.rs from `type Slot<'a>` to `/// What a reader states about one member`.
INDEX_PIN = "174a42fbc8929d9f5bb9cd2b5637ec88eb09d15e7969f1363c13e1e3b83fb865"

# Rows of one member that a §4.3 derived treatment can name: σ⁺/σ⁻ at five stations on two bases (20) and τ_v,max at
# five stations on two bases (10).
NOT_QUALIFIED_PER_MEMBER = 30
STATIONS = 5
BASES = 2

ASSUMED.update({
    # illustrative only: g5_profile binds each to the actual type's size_of in the PP build
    "s(BasisSection)": 56, "s(SectionFactorResult)": 40, "s(StationWitnesses)": 32, "s(StationExtreme)": 64,
    "s(NotCoveredOperands)": 400, "s(U4bSlotEntry)": 104, "s((&str,Vec<f64>))": 40, "s((String,Vec<f64>))": 48,
    "s(U4bCoverEntry)": 80, "s((&str,(f64,f64)))": 32,
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
        raise SystemExit(f"U4B/U5B owners: {name} is not one definition; re-review the owner it prices")
    indent = hits[0].group(1)
    end = text.find("\n" + indent + "}\n", hits[0].start())
    if end < 0:
        raise SystemExit(f"U4B/U5B owners: no end of fn {name}")
    return text[hits[0].start():end + len(indent) + 3]


def _span(text, start, stop, label):
    if text is None or start not in text:
        return None
    a = text.index(start)
    e = text.find(stop, a)
    if e < 0 or text.count(start) != 1:
        raise SystemExit(f"{label}: the source changed; re-review the owner it prices")
    return text[a:e]


def _pinned(snapshot, keys, label):
    """True when every function exists with its reviewed body, False when none exists; anything else stops."""
    found = []
    for rel, name in keys:
        text = _read(snapshot, rel)
        body = _body(text, name) if text is not None else None
        found.append(body)
    if all(b is None for b in found):
        return False
    for (rel, name), body in zip(keys, found):
        if body is None or hashlib.sha256(body.encode()).hexdigest() != BODY_PINS[(rel, name)]:
            raise SystemExit(f"{label}: {rel} fn {name} changed; re-review the owner it prices")
    return True


def _once(text, needle, label):
    if text is None or text.count(needle) != 1:
        raise SystemExit(f"{label}: {needle.strip()!r} is not one site; re-review the owner it prices")


def inspect(snapshot):
    out = {}
    lib = _read(snapshot, LIB)
    # U5B-PD: the case solve's published (corrected) nodal displacements, one Vec<f64> of N per case.
    out["published_displacements"] = _pinned(snapshot, [(LIB, "published_nodal_displacements")], "U5B-PD")
    if out["published_displacements"]:
        _once(lib, "    let published_displacements = published_nodal_displacements(&displacements, member_bound);\n", "U5B-PD")
    # U4B-ST: one member's station quantities (append_station_quantities and the caller's bases).
    out["station_quantities"] = _pinned(snapshot, [(U4B, "member_bases"), (U4B, "append_station_quantities")], "U4B-ST")
    if out["station_quantities"]:
        _once(lib, "            let bases = u4b_rows::member_bases(section, exact_bases);\n", "U4B-ST")
        _once(lib, "                bases: bases.clone(), poisson_ratio, directional, toroidal,\n", "U4B-ST")
    # U5B-AB: the absorbed-diagonal refusal's free-DOF flags, one Vec<bool> of N per contribution-sum call.
    fks = _read(snapshot, FKS)
    block = _span(fks, "pub(crate) struct ElementAbsorption<'g> {", "fn contribution_differences(", "U5B-AB")
    out["absorption"] = block is not None
    if block is not None:
        if hashlib.sha256(block.encode()).hexdigest() != ABSORPTION_PIN:
            raise SystemExit("U5B-AB: ElementAbsorption changed; re-review the owner it prices")
        _once(fks, "    let mut absorption = ElementAbsorption::new(system.contribution_groups, entries.len(), system.free_dofs, n)?;\n", "U5B-AB")
        if not _pinned(snapshot, [(SPARSE, "sparse_contribution_sums")], "U5B-AB"):
            raise SystemExit("U5B-AB: sparse_contribution_sums is missing; re-review the owner it prices")
    # U4B-W1: the failed manifest's Poisson flags and grown diagnostics, and prepared_not_covered's operand table.
    out["not_covered"] = _pinned(snapshot, [(RP, "failed_manifest"), (RP, "prepared_not_covered")], "U4B-W1")
    # U4B-RD: the u4b readers' row index, covers, exemption sets and the preview model checks' array clones.
    evid = _read(snapshot, EVID)
    index = _span(evid, "type Slot<'a> =", "/// What a reader states about one member", "U4B-RD")
    keys = [(EVID, "member_stations"), (EVID, "poisson_exempt"), (EVID, "preview_model_checks"), (PHYS, "u4b_cases")]
    out["readers"] = _pinned(snapshot, keys, "U4B-RD")
    if out["readers"]:
        if index is None or hashlib.sha256(index.encode()).hexdigest() != INDEX_PIN:
            raise SystemExit("U4B-RD: u4b_evidence's Index changed; re-review the owner it prices")
        prev = _read(snapshot, PREV)
        _once(prev, "    let index = crate::u4b_evidence::Index::new(rows_list, crate::u4b_evidence::Route::Preview)?;\n", "U4B-RD")
        _once(prev, "        let mut covers: HashMap<&str, Vec<f64>> = HashMap::new();\n", "U4B-RD")
    elif index is not None:
        raise SystemExit("U4B-RD: u4b_evidence's Index without its readers; re-review the owner it prices")
    # U4B-NQ: bind_for's derived_not_qualified treatments.
    out["not_qualified"] = _pinned(snapshot, [(ROWS, "not_qualified_for"), (ROWS, "not_qualified_inputs")], "U4B-NQ")
    # U5B-CG (T4-S6): the evidence's contribution-group vector, one usize pushed per frame/curved element and per
    # ground spring into a Vec::new() and moved into the dense or sparse AssemblyEvidence.
    ni = _read(snapshot, NI)
    group_lines = ["    groups: Vec<usize>,\n", "            groups: Vec::new(),\n",
                   "        self.groups.push(self.contributions.len());\n",
                   "            result.groups.push(result.contributions.len());\n"]
    counts = [ni.count(line) if ni is not None else 0 for line in group_lines]
    moves = ni.count("            contribution_groups: parts.groups,\n") if ni is not None else 0
    if all(c == 0 for c in counts) and moves == 0:
        out["contribution_groups"] = False
    elif all(c == 1 for c in counts) and moves == 2 and ni.count("groups.push(") == 2:
        out["contribution_groups"] = True
    else:
        raise SystemExit("U5B-CG: structural_adapter.rs's contribution groups changed; re-review the owner it prices")
    return out


def ordinary_published_displacements(N):
    """U5B-PD: `collect::<Result<Vec<_>, _>>()` has size-hint lower bound 0, so the Vec push-grows to pushcap(N)
    slots with its previous backing live during the last reallocation (the `to_vec` fallback is N exactly)."""
    return B(8 * pushcap(N) + 8 * (pushcap(N) // 2))


def ordinary_station_quantities():
    """U4B-ST: one member at a time. The bases (exact collect of <= 2) and their clone into MemberQuantities, the
    shear factors (exact collect of <= 2), the normal and directional witness lists (<= 2 pushes each, each holding
    <= 5 station pairs push-grown), and one station's extremes (<= 2 pushes). Strings are TEXT sites; rows pushed
    into the case results are inside P_final."""
    return (Xc("BasisSection", BASES) * 2 + Xc("SectionFactorResult", BASES)
            + s("StationWitnesses") * (2 * pushcap(BASES)) + B(2 * BASES * 16 * pushcap(STATIONS))
            + s("StationExtreme") * pushcap(BASES))


def contribution_groups(m, springs):
    """U5B-CG: one evidence owner's contribution-group vector, 8·pushcap(m + s) (no curved element or connector on D1:
    P2, P3); the dense and sparse evidences each own one, so it is charged wherever their contributions are."""
    return B(8 * pushcap(m + springs))


def absorption_flags(N):
    """U5B-AB: `vec![false; n]` of the system's N DOFs, live inside one contribution-sum call."""
    return B(N)


def producer_not_covered(m):
    """U4B-W1: prepared_not_covered reserves one [[Option<usize>; 5]; 5] per member (adapter reserve, exact)."""
    return Xc("NotCoveredOperands", m)


def grown_from(capacity, pushes):
    """The capacity of a Vec that starts at exactly `capacity` (a clone) and takes `pushes` pushes: RawVec's amortized
    growth doubles the capacity (at least to the required length, at least 4) whenever a push meets a full Vec."""
    cap = capacity
    for length in range(capacity + 1, capacity + pushes + 1):
        if length > cap:
            cap = max(2 * cap, length, 4)
    return cap


def failed_manifest_additions(m, d_env):
    """U4B-W1: failed_manifest's out_of_domain (adapter reserve of m Option<f64>, exact) and the diagnostics clone
    grown by <= m Poisson warnings. `ordinary.diagnostics.clone()` has exactly d_env slots (already in the staged
    copy), so the first push reallocates to 2·d_env (T4-S6: was pushcap(d_env + m) plus half of it, below 2·d_env
    whenever d_env + m does not reach a power of two); the replaced d_env backing is the staged copy's own capacity.
    Every grown backing is charged here, beside it."""
    grown = grown_from(d_env, m) if m > 0 else 0
    return Xc("Option<f64>", m) + s("Diagnostic") * grown


def reader_working_sets(rows, cases, m, raw_tree):
    """U4B-RD (T17 V5, G7): the projection's base read (preview_physics_evidence or physics_evidence u4b_cases) and
    route L's preview_model_checks, summed although they run in turn. Each builds one Index over the envelope rows
    (<= rows slots, push-grown hash table with its previous table); preview: the Poisson exemption set and the
    per-member station covers (<= m, <= 5 values per basis); route E: the covers of every case, member and basis, the
    case's material index and pressurized arcs (<= m); member_stations' per-member cover map (<= 2 bases) and the
    facts' small vectors. preview_model_checks clones a model array per lookup (`as_array().cloned()`) while the found
    case, pipe, material and point are held: at most two copies of the captured model (raw_tree each), plus its
    member set and exemption set."""
    index = HashMov("U4bSlotEntry", rows)
    cover_values = B(8 * pushcap(STATIONS))
    preview = (index + HashMov("&str", m) + HashMov("(&str,Vec<f64>)", m) + cover_values * m
               + HashMov("(String,Vec<f64>)", BASES) + (B(16) + cover_values) * BASES + B(32 + 16))
    exact = (index + HashMov("U4bCoverEntry", cases * BASES * m) + (B(16) + cover_values) * (cases * BASES * m)
             + HashMov("(&str,&Value)", m) + HashMov("(&str,(f64,f64))", m)
             + HashMov("(String,Vec<f64>)", BASES) + (B(16) + cover_values) * BASES + B(2 * 32 + 2 * 16))
    model_checks = (index + raw_tree * 2 + X({"Node(&str,())": tree_nodes(m)}) + HashMov("&str", m))
    return preview + exact + model_checks


def receipt_not_qualified(m):
    """U4B-NQ (T25.3): not_qualified_for pushes <= 30 m RowTreatments (their strings move into the ledger, which
    already pays each row's id, treatment and four input ids), with the previous backing of its last reallocation,
    and one inputs vector (<= 3 references) at a time."""
    h = NOT_QUALIFIED_PER_MEMBER * m
    return V("RowTreatment", h) + s("RowTreatment") * (pushcap(h) // 2) + B(3 * 8)
