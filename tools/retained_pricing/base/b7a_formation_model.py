"""Closed B7a formed-load traversal and finding ownership premises.

This model qualifies the admitted D1 paths, not ordinary fallback on refused
inputs. Historical BASE files remain immutable; every replaced premise is
checked before transforming the copied rules. Formats, copies and JSON owners
remain independent, even where their source-list lengths share one bound.
"""
from copy import deepcopy
import hashlib
import json
import os
import re
import runpy

# W1-L1a: borrow/lifetime annotations only; source traversal bounds unchanged.
SOURCE_FILES = {
    "core/product_physics/src/formation_guard.rs": "9de2c2afdc7350c8928ba341c309a2e47326e2eefeabce1eabed2d6b1f11a1a6",
    # PP (D-5 mass): an exact-contract section's generated self-weight uses EXACT_MASS_METHOD (metal on the nominal
    # wall, contents on the pressure bore, the allowance among its inputs); the added refusals are static `invalid`
    # messages and retained_basis checks one more input key. The formed-load and finding owners are unchanged.
    # W-16 slice a (§8.23): primitive_payload and inspect_applied_self_weight gain the support arm (a payload for a
    # support target, none on D1; `false` for the generated identity). The formed-load and finding owners are unchanged.
    # Nested-record closure (§8.25): the two evidence-reader PreviewPipe literals name the pipe's three
    # declared-and-ignored zero-size members (+3 lines each), old_dependencies_mass removes normalized_dependencies'
    # section_ref before rebuilding the closed section (DEPENDENCY_SECTION_REF), component_basis reads the recorded
    # weight and centre of gravity as closed quantities, and the tests add evidence tests. No formed-load or finding
    # owner changes.
    "core/product_physics/src/self_weight.rs": "e543bf48200f0453a11026f1167d124050ffc21b275c977fc98e52698f85f775",
    "core/product_physics/src/source_receipt/source.rs": "384140db918da65d5e76dd601cd0a2068ae2a098379b41c6907069cf99ced419",
    # T4-U8: retained::assemble's Timoshenko refusal; W1 adds one retained_api
    # export line for member-source types. Existing formation owners stay.
    # T4-S5 (epoch, U5b): structural.rs and sparse.rs move the absorbed-diagonal refusal into ElementAbsorption
    # (per-element groups, one Vec<bool> of the free DOFs per contribution audit; priced by U5B-AB) and add the tie
    # rod's stiffness (components only: P3) and the arc maximum's module; source_recovery.rs adds
    # `contribution_groups: None` and comment/path text. No formed-load traversal or finding owner changes.
    # W1 coverage repair D3 (§8.11): retained_api adds the CENSUS_PROBE const (cfg!, no allocation) and a census_probe
    # re-export under the off feature (W1R-FG). No formed-load traversal or finding owner changes.
    # W1 coverage repair corner 1 (§8.12): read 32e0c3faf0; the same for LOADED_MAXIMUM_HOOKS and the
    # loaded_maximum_hooks re-export under the off `loaded-maximum-hooks` feature.
    "core/solver/frame_kernel/src/structural.rs": "c4d96b2c41ce74044953dc8893b269570809651867ac55fee430ea2d1b475e82",
    # T4-U6/U8 (O4): prepare_sources declines Timoshenko, nonzero mill tolerance and nonzero allowance with static
    # `unsupported(..)` messages after the existing declines; close_load_state names the nominal section's area.
    # No formed-load or finding owner changes.
    # R-A D3 (§8.13): read b903a2d901; `Sources::hint` is B1's index rule (arithmetic, no allocation), which solve's
    # plan (`new_with_hints`) and the replay pass to the kernel as a borrowed closure (the kernel's hint vectors are
    # RA-HS); j7_rf_census declares the #[cfg(test)] gating module. No formed-load or finding owner changes.
    # Pre-0.4 failure unification D3 (§8.14): read b6cc3a792a; the screen's and decline_withheld's doc comments are
    # route-neutral and RF-14's test is restated. No formed-load or finding owner changes.
    "core/product_physics/src/source_recovery.rs": "0c38c5ebd6c51148b5d5d554a11504bbb3273b3e7899657eab88255ddcd2aac8",
    "core/solver/frame_kernel/src/structural/sparse.rs": "e316bc9d2738285ef9a87392436436c667b60b9ab0a744f6650051fcf918f4fb",
}
RANGE_CONTENT = 59
REASON = 165057
FIRED = 165659
JOINED = 993964
SENTENCE = 994321
ZERO_PREIMAGES = {
    "formation_guard.rs:315": "after load_row_finding's has_formation_records() early return (formation_guard.rs:303-305); D1.7 nodal-only loads carry no formation records",
    "formation_guard.rs:320": "same early return",
    "self_weight.rs:814": "D1.7: the deterministic-identity format! is only in the Element-target arm (self_weight.rs:811-821); every D1 load has a Node target",
}
COPY_PREIMAGES = {
    "load_ledger.rs:133": "5*cases*l*2*128",
    "load_ledger.rs:408": "4*cases*l*2*128",
    "load_ledger.rs:670": "5*cases*l*2*128",
}
BASE_LOOP_PREIMAGES = (
    (211, r"^iter:terms\.iter\(\)\.map$", "l", "G4 (R-1, newly reached): structural.rs:1098 row_sources: the force terms of one row <= l", "12*l"),
    (225, r"^for t in &d\.terms$", "144*m+s+l", "G4 (R-1, newly reached on the repaired graph): terms of one descriptor <= contributions + springs + force terms", "144*m+s+12*l"),
    (265, r"^for \(i, t\) in d\.terms\.iter\(\)\.enumerate\(\)$", "144*m+s+l", "terms of one descriptor <= contributions + springs + force terms", "144*m+s+12*l"),
    (276, r"^for &index in indices$|^for \(index, term\) in self\.terms\.iter\(\)\.enumerate\(\)$|^iter:self \.terms \.iter\(\)\.map$", "l", "load ledger terms and per-DOF index lists <= l nodal terms", "12*l"),
    (222, r'^iter:" ":forces\.iter\(\)\.map$', "l", "G4 (R-1, newly reached on the repaired graph): source.rs:376 commitment force_terms: <= l nodal force terms", "12*l"),
    (226, r"^for product in &row\.offset$", "144*m+s+l", "G4 (R-1, newly reached on the repaired graph): source_recovery.rs:1412 append_scaled_products: offset products of one row <= the descriptor bound", "144*m+s+12*l"),
    (219, r"^for &component in scaled_intended$", "l", "G4 (R-1, newly reached on the repaired graph): load_ledger.rs:595 add_defect: the scaled intended components of one DOF row <= its force terms <= l", "max(l,80)"),
)
SPLIT_LOOP_PREIMAGE = dict(re=r"old\.loads\(\)|^for f in forces$", bound="l", why="nodal loads/forces <= l")
FIDELITY_PREIMAGES = {'source_clone': {'from': ['structural.rs:1041:audit_load_fidelity', 'structural/sparse.rs:1815:audit_sparse_load_fidelity'], 'per': '2*l', 'why': 'G4 (R-1, newly reached): row_sources clones one source id per force term of a row; over one audit call, every nodal force term sits on one DOF row, and each row calls at most unaudited_row and audit_load_row (structural.rs:1037-1180), so <= 2l clones per audit call'}, 'rows_join': {'re': '^rows$', 'bytes': 'N*(165+2*20+2*16+2*24+20+24+8+2)+l*(2+6*128+2+2)', 'why': 'lib.rs:1290: the load-fidelity row strings (lib.rs:1266/1270) over N rows with all sources once'}, 'aggregates': {'lib.rs:1273': {'absorbs': 'report \\.rows', 'bytes': 'N*(46+2*20+7+5+8)+l*(2+6*128+2+2)+2*N', 'why': "per call: the unaudited-row strings over all N rows; sum over rows of each row's sources Debug <= all l identified sources once (each nodal force term sits on one DOF row), so the sources part is l*(str Debug 6*128+2 plus separator) in total, not per row"}, 'lib.rs:1277': {'absorbs': 'report \\.rows', 'bytes': 'N*(165+2*20+2*16+2*24+20+24+8)+l*(2+6*128+2+2)+2*N', 'why': 'per call: as lib.rs:1266 for the audited-row template (two {:016x}, three {:?} f64, one usize); sources over all rows <= l'}}}
COPY_OBJECTS = {'load_ledger.rs:133': {'bytes': '5*cases*l*2*128', 'why': "I104 SQ B1 (5*l*2*128 -> 5*cases*l*2*128): the case ledger and the T25 replay's are per requested case, the three prepare_sources folded ledgers per case (5 per case, c cases). Was: LoadLedger::push copies one load id per pushed term; per invocation at most 5 nodal ledgers exist (the case ledger, lib.rs:3221; the T25 replay's case ledger; three prepare_sources folded ledgers, source_recovery.rs:705), each with <= l terms; capacity <= 2x length"}, 'load_ledger.rs:408': {'bytes': '4*cases*l*2*128', 'why': "I104 SQ B1 (4*l*2*128 -> 4*cases*l*2*128): W2's two evaluations run per requested case. Was: force_scaled_term clones each term's source id: at most two W2 evaluations, each with one scaled force plus the reaction generation (I54 W2_RETURN), each <= l terms"}, 'load_ledger.rs:670': {'bytes': '5*cases*l*2*128', 'why': 'I104 SQ B1 (5*l*2*128 -> 5*cases*l*2*128): as LoadLedger::push: 5 ledgers per case. Was: push_unique copies one source id per pushed term: <= 5 ledgers x l terms per invocation (as LoadLedger::push)'}}
O10_PREIMAGES = (
    ("for p in &source.loads", "0", "KD5L3: D1/W1 excludes load-state; source_factor returns None unless is_load_state; nodal uses push input and no load primitive", ("structural/formation_check.rs:load_closure",)),
    ("for (i, (term, record)) in terms.iter().zip(records).enumerate()", "l", "KD5L4: exactly one trusted closure record per nodal ledger term, <= l", ("structural/formation_check.rs:load_closure",)),
    ("for (j, expected) in manifest.iter().enumerate()", "0", "KD5L5: every D1 nodal record is input (formed=false); no manifest is installed; loop only after formed guard", ("structural/formation_check.rs:load_closure",)),
    ("for (i, term) in ledger.terms().iter().enumerate()", "l", "KD5L11: one input term per admitted nodal primitive, <= l", ("structural/formation_check.rs:evaluate",)),
    ("for p in source.loads.iter().filter(|p| p.term_index == i)", "0", "KD5L12: supplied load primitive list is empty by KD5L3", ("structural/formation_check.rs:evaluate",)),
)


def inspect(snapshot):
    # This binds all 21 existing owners and the full nongenerated admission
    # census/enforcement plus law_order/admit/the genuine permit constructor.
    domain = runpy.run_path(os.path.join(os.path.dirname(__file__), "b7a_domain_model.py"))
    proof = domain["inspect"](snapshot)
    fingerprints = {}
    for file, expected in SOURCE_FILES.items():
        with open(os.path.join(snapshot, file), "rb") as stream:
            raw = stream.read()
        fingerprints[file] = hashlib.sha256(raw).hexdigest()
        if fingerprints[file] != expected:
            raise ValueError("B7a formation owner source changed: " + file)
    proof["formation_raw_fingerprints"] = fingerprints
    proof["formation_model_sha256"] = hashlib.sha256(open(__file__, "rb").read()).hexdigest()
    return proof


def lengths(nodes, loads, ident):
    """Each list generation pays its own escaped source bytes.

    Ledger.finish fixes N=6*nodes rows. Each admitted input emits <=12 terms,
    including zero terms. A term contributes once to each of formed/cannot
    lists, and push_unique removes repeats only within that row. Thus either
    list's total entries <=q=12*loads; one row has <=loads distinct bare load
    IDs (<=ident), not member_source's separately priced 268-byte identity.
    """
    n = 6 * nodes
    q = 12 * loads
    debug_total = 2 * n + (10 * ident + 4) * q
    fired_total = n * (600 + 2 + max(RANGE_CONTENT, 87, 321)) + debug_total
    return dict(N=n, q=q, debug_total=debug_total, fired_total=fired_total,
                reason=REASON, fired=FIRED, joined=JOINED, sentence=SENTENCE)


def finding_capture(cases, nodes, loads, ident):
    from g3lib import Xc, B
    b = lengths(nodes, loads, ident)
    # ordinary_finding copies the complete N-element Vec, every fired child,
    # and sentence. It stays in ProductCapture.ordinary through publication.
    return (Xc("String", b["N"]) + B(b["fired_total"] + SENTENCE)) * cases


def finding_construction(cases, nodes, loads, ident):
    from g3lib import V, Xc, HashReq, B, s, pushcap, ASSUMED
    # Illustrative only. G5 binds both exact Rust types in the actual build.
    ASSUMED.setdefault("s(FormationRow)", 3 * ASSUMED["s(ExactAccumulator)"] + 128)
    ASSUMED.setdefault("s((usize,[f64;4]))", 40)
    b = lengths(nodes, loads, ident)
    n, q = b["N"], b["q"]
    # FormationRow's native type pays its inline 3 ExactAccumulators and two
    # Vec headers. Both distinct child lists independently hold <=q entries.
    # Sum max(4,2h) across <=N lists for each family, with owned bare IDs.
    rows = (V("FormationRow", n) * 2 + s("String") * (8 * n + 4 * q) * 2
            + B(2 * q * ident))
    maps = (HashReq("usize", n) + HashReq("(usize,[f64;4])", nodes)
            + HashReq("(usize,f64)", n))
    # TAV pays formatted reason/fired/join/sentence children. The cloned six
    # items are not separate lexicon String sites, so pay their actual owner.
    fired = (V("String", n) + s("String") * (pushcap(n) // 2)
             + Xc("String", 6) + B(6 * FIRED))
    # Keep all sequential requested/old table backing allowances positively.
    return (rows + maps * 2 + fired) * cases


def finding_value_facts(cases, nodes, loads, ident, diagnostic_ref):
    b = lengths(nodes, loads, ident)
    return dict(arr=cases * b["N"], obj=cases, ent=4 * cases,
                strb=cases * (SENTENCE + b["fired_total"] + 8 + diagnostic_ref),
                keyb=cases * 4 * 32, nums=0, depth=4)


def rebind_o10(snapshot, rows):
    inspect(snapshot)
    result = deepcopy(rows)
    for header, old_bound, old_why, owners in O10_PREIMAGES:
        hits = [i for i, row in enumerate(result) if row[0] == header]
        expected = (header, old_bound, "O-10 (T4): " + old_why, list(owners))
        if len(hits) != 1 or result[hits[0]] != expected:
            raise ValueError("B7a exact formed O10 preimage changed: " + header)
        # attach_last creates one Summand on a NEW term; attach installs
        # one manifest and one primitive at that term_index. Plain nodal
        # records have no manifest. D1 rejects all other formed constructors;
        # scaling and NI copies preserve indices/cardinality. The supplied
        # outer walk and predicate scans still traverse q, not this body1.
        body_one = header in ("for (j, expected) in manifest.iter().enumerate()",
                              "for p in source.loads.iter().filter(|p| p.term_index == i)")
        result[hits[0]] = ("for p in source.loads.iter()" if header == "for p in &source.loads" else header,
                           "1" if body_one else "12*l",
                           "B7a source-bound formed traversal: q<=12*l; one manifest and matching primitive BODY per admitted uniform term; outer scans remain q",
                           list(owners))
    return result


def decisions(snapshot, resolved):
    inspect(snapshot)
    # Existing function-qualified LOOP_TOTAL bounds the only load-row guard,
    # leaving other owners of the common `for row in &rows` header at P.
    # It also propagates the actual total into decide_row's call multiplicity.
    loop_total = [(resolved("formation_guard.rs:304:load_row_finding"),
                   r"^for row in \&rows$", "cases*N",
                   "Ledger.finish(built.nodes.len()*6), private AssembledForce constructors and one guard call per requested case")]
    pp = "core/product_physics/src/formation_guard.rs:"
    audit = [(resolved(pp + "322"), {"reason": {"bytes": REASON, "source": "COMP"}}),
             (resolved(pp + "327"), {
                 'fired.len()': {"bytes": 20, "source": "NUM"},
                 'fired.iter().take(NAMED).cloned().collect::<Vec<_>>().join(";")': {"bytes": JOINED, "source": "COMP"},
                 'iffired.len()>NAMED{format!(";and{}more",fired.len()-NAMED)}else{String::new()}': {"bytes": 31, "source": "COMP"},
             })]
    # Newly positive identifiers remain site-local: this callback is the
    # guarded integrity_dof_label closure, <=128-byte node ID+3 or a
    # 31-byte global_dof fallback. Keep the existing conservative600 class.
    audit[0][1]["dof_label(row.dof)"] = {"bytes": 600, "source": "COMP"}
    audit[1][1]["case_id"] = {"bytes": 128, "source": "IN128"}
    # T4-U6/U8 snapshot keys (PP head 20c6126a16; same text): self_weight.rs 814 -> 869 (the D-5 exact mass branch
    # above it); formation_check.rs +13 from check_at on (PP's frame-member binding before it).
    # W-15 slice b (§8.22): 869 -> 1281 (the component-weight generator above it; same text). W-16 slice a (§8.23):
    # 1281 -> 1284 (primitive_payload's support arm above it; same text).
    # Nested-record closure (§8.25): 1284 -> 1292 (DEPENDENCY_SECTION_REF's two lines, the first evidence-reader
    # literal's three members and component_basis's three closed reads above it), same text.
    audit.append((resolved("core/product_physics/src/self_weight.rs:1292"), {
        "case.id": {"bytes": 128, "source": "IN128"},
        "pipe": {"bytes": 128, "source": "IN128"},
        "case.id.len()": {"bytes": 20, "source": "NUM"},
        "pipe.len()": {"bytes": 20, "source": "NUM"},
    }))
    fc = "core/solver/frame_kernel/src/structural/formation_check.rs:"
    # ForceTerm.source is the bare admitted load_id, not the separately
    # owned member_source identity in LoadPrimitive/manifest entries.
    for line in (528, 534, 544, 620, 633, 640):
        audit.append((resolved(fc + str(line)), {
            "terms[p.term_index].source": {"bytes": 128, "source": "IN128"},
        }))
    # U5B-TR (T4-U5b): the TieRodReference binding refusal formats the formed term's source,
    # "tie_rod_reference:{len}:{component}:{len}:{rod}" (18 + 4 + 128 + 1 + 4 + 128 = 283 B); tie rods exist only on
    # connectors, so it is unreached on D1 (P3), and it is still charged at its full class.
    audit.append((resolved(fc + "580"), {
        "terms[p.term_index].source": {"bytes": 283, "source": "U5B-TR: formed tie-rod reference source"},
    }))
    for line in (672, 682, 688, 694, FIRST_RETURN_LINE):
        audit.append((resolved(fc + str(line)), {
            "term.source": {"bytes": 128, "source": "IN128"},
        }))
    # Keep the old generic suffix allowance (no length-class reduction).
    # JOINED+SENTENCE must also be supplied as site-specific positive sizes.
    return loop_total, audit


def lex_sizes(snapshot, resolved):
    inspect(snapshot)
    return [(resolved("formation_guard.rs:330"),
             {"class": JOINED, "why": "B7a load-row finding only: six independently cloned fired items plus five separators"}),
            (resolved('retained_product.rs:677'),   # T4-S5: +5 lines (MemberIdentity's poisson_ratio); O4 S5: +61 (the O4 structs), same text
             {"class": SENTENCE, "why": "B7a full load-row finding sentence copied into persistent FindingSeed; fired children paid separately"})]


def prepare(snapshot, inherited, output):
    proof = inspect(snapshot)
    raw = {name: open(inherited[name], "rb").read()
           for name in ("loop_bounds.g4.json", "text_args.g4.json")}
    loops = json.loads(raw["loop_bounds.g4.json"])
    text = json.loads(raw["text_args.g4.json"])
    rf_active = runpy.run_path(os.path.join(os.path.dirname(__file__), "rf_source_model.py"))["active"]
    if rf_active(snapshot):
        # RF-ID: the two member-row ID clones moved from prepare_sources
        # into member_descriptors (with member, not pipe.element_id, as the
        # parameter). Re-add both by name in o10_rules; retain the third,
        # unchanged member_ids clone at basis line 1134.
        for key in ("core/product_physics/src/source_recovery.rs:1099",
                    "core/product_physics/src/source_recovery.rs:1124"):
            if text["id_audit"].pop(key, None) != {"pipe.element_id": {"bytes": 128, "source": "IN128"}}:
                raise ValueError("RF-ID member clone preimage changed: " + key)
    if "b7a_reformation_first_error" in text:
        raise ValueError("B7a first-return guard preimage already present")
    text["b7a_reformation_first_error"] = "FK895-v1"
    owned = runpy.run_path(os.path.join(os.path.dirname(__file__), "b7a_returning_errors.py"))
    if owned["MARKER"] in text:
        raise ValueError("B7a owned first-error guard preimage already present")
    text[owned["MARKER"]] = owned["VERSION"]
    proof["owned_first_error_sources"] = owned["inspect"](snapshot)
    alterations = []
    for index, regex, bound, why, changed in BASE_LOOP_PREIMAGES:
        expected = dict(re=regex, bound=bound, why=why)
        if loops["loops"][index] != expected:
            raise ValueError("B7a exact force-term loop preimage changed: " + str(index))
        loops["loops"][index] = dict(expected, bound=changed,
                                    why="B7a q<=12*l positive force-term upper; input-count owners retain l. Was: " + why)
        alterations.append(dict(family="loops", index=index, before=expected,
                                after=loops["loops"][index]))
    if loops["loops"][256] != SPLIT_LOOP_PREIMAGE:
        raise ValueError("B7a authored/formed split loop preimage changed")
    split = [dict(re=r"old\.loads\(\)", bound="l", why="B7a: PrimitiveSource.loads is the authored nodal input vector; unchanged input-count bound"),
             dict(re=r"^for f in forces$", bound="12*l", why="B7a: emitted AssembledForce terms have q<=12*l; source constructor is guarded")]
    loops["loops"][256:257] = split
    alterations.append(dict(family="loops", index=256, before=SPLIT_LOOP_PREIMAGE, after=split))
    for key, expected in ZERO_PREIMAGES.items():
        if text["site_zero"].get(key) != expected:
            raise ValueError("B7a exact formation/self-weight zero changed: " + key)
        del text["site_zero"][key]
        alterations.append(dict(family="site_zero", key=key, before=expected))
    # Preserve the 17 later self-weight function zeros. Their actual warrant
    # is the guarded nonobject provenance + deterministic identity refusal,
    # followed by continue825; Element identity formatting itself is positive.
    # W-15 slice b (§8.22, W15B-NS): the node arm now tests the generated
    # namespace prefix, and D1.7 refuses a node load with that prefix too.
    old_why = None
    for row in loops["fn_zero"]:
        if row["fn"].startswith("self_weight.rs:"):
            if old_why is None:
                old_why = row["why"]
            if row["why"] != old_why or "false for a Node target" not in old_why:
                raise ValueError("B7a self-weight downstream zero preimage changed")
            row["why"] = "B7a: guarded provenance forbids objects; self_weight_form refuses the generated Element identity and W15B-NS a node load in the generated namespace; inspect_applied_self_weight continues at825 before this owner. Formatting that identity is positively priced. Was: " + old_why
    if sum(row["fn"].startswith("self_weight.rs:") for row in loops["fn_zero"]) != 17:
        raise ValueError("B7a self-weight downstream function inventory changed")
    for key, expected in COPY_PREIMAGES.items():
        entry = text["site_total"].get(key)
        if entry != COPY_OBJECTS[key]:
            raise ValueError("B7a exact ledger copy total changed: " + key)
        multiplier = 10 if key.endswith(":670") else (5 if key.endswith(":133") else 4)
        entry["bytes"] = f"{multiplier}*cases*(12*l)*2*128"
        entry["why"] = "B7a q<=12*l: preserves historical ledger/evaluation count; push_unique pays both formed and cannot source-list generations independently. Was: " + entry["why"]
        alterations.append(dict(family="site_total", key=key, before=expected, after=entry["bytes"]))
    # The normal q loop is not enough: text_budget's site_from override is
    # applied afterward. Keep both audited callers and their two-copy factor.
    key = "structural.rs:1103"
    if text["site_from"].get(key) != FIDELITY_PREIMAGES["source_clone"]:
        raise ValueError("B7a fidelity source-clone override preimage changed")
    prior = deepcopy(text["site_from"][key])
    text["site_from"][key] = dict(prior, per="24*l",
                                  why="B7a: both audited callers retain their complete2*q source-copy upper before row-local dedup; String128/capacity2 remain. Was: " + prior["why"])
    alterations.append(dict(family="site_from", key=key, before=prior, after=text["site_from"][key]))
    for key, expected in FIDELITY_PREIMAGES["aggregates"].items():
        if text["site_aggregate"].get(key) != expected:
            raise ValueError("B7a fidelity source aggregate preimage changed: " + key)
        changed = dict(expected, bytes=expected["bytes"].replace("l*(2+6*128+2+2)", "(12*l)*(2+10*128+2+2)"),
                       why="B7a: <=q source entries across all DOF rows, with independent10-byte Debug escape upper; every fixed scalar/row/bracket term stays. Was: " + expected["why"])
        text["site_aggregate"][key] = changed
        alterations.append(dict(family="site_aggregate", key=key, before=expected, after=changed))
    expected = FIDELITY_PREIMAGES["rows_join"]
    if text["join_rules"].count(expected) != 1:
        raise ValueError("B7a fidelity rows join preimage changed")
    changed = dict(expected, bytes=expected["bytes"].replace("l*(2+6*128+2+2)", "(12*l)*(2+10*128+2+2)") + "+2*N",
                   why="B7a: complete row-string join with q aggregate source occurrences; fixed allowances and independent row brackets remain")
    text["join_rules"][text["join_rules"].index(expected)] = changed
    alterations.append(dict(family="join_rules", before=expected, after=changed))
    # Maintain individual source/debug/string maxima. Aggregate source mass
    # is used only for requested capacities of these exact guarded sites.
    aggregate = "(2*N+(10*128+4)*(12*l))"
    totals = {
        "formation_guard.rs:239": "2*cases*N*59",
        "formation_guard.rs:241": f"2*cases*(87*N+{aggregate})",
        "formation_guard.rs:251": f"2*cases*(321*N+{aggregate})",
        "formation_guard.rs:322": f"2*cases*(N*(600+2+321)+{aggregate})",
    }
    # Current line keys are resolved later in SQ2, not carried from the old
    # basis here; use the exact existing name-qualified decision seam below.
    proof.update(input_files={name: hashlib.sha256(data).hexdigest() for name, data in raw.items()},
                 alterations=alterations, requested_site_totals=totals)
    os.makedirs(output, exist_ok=True)
    paths = {}
    for name, data in (("loop_bounds.g4.json", loops), ("text_args.g4.json", text)):
        path = os.path.join(output, name)
        with open(path, "w", encoding="utf-8") as stream:
            json.dump(data, stream, indent=1)
        paths[name] = path
    proof["prepared_files"] = {name: hashlib.sha256(open(path, "rb").read()).hexdigest()
                               for name, path in paths.items()}
    return paths, proof


def site_totals(snapshot, resolved):
    inspect(snapshot)
    g = "(2*N+(10*128+4)*(12*l))"
    return [(resolved(key), expression,
             "B7a exact formed guard owner: full independent generation; <=cases guards, N DOF rows, q<=12*l aggregate escaped source IDs")
            for key, expression in (
                ("formation_guard.rs:239", "2*cases*N*59"),
                ("formation_guard.rs:241", f"2*cases*(87*N+{g})"),
                ("formation_guard.rs:251", f"2*cases*(321*N+{g})"),
                ("formation_guard.rs:322", f"2*cases*(N*(600+2+321)+{g})"),
            )]



FIRST_RETURN_FILE = "core/solver/frame_kernel/src/structural/formation_check.rs"
FIRST_RETURN_ROOT = FIRST_RETURN_FILE + ":914:evaluate"
FIRST_RETURN_PARENT = FIRST_RETURN_FILE + ":474:check_at"
# T4-U8: evaluate's added `straight_frames_bound(source)?;` (the line after load_closure) moves the reformation
# map_err site from 895 (B7b) to 896 with unchanged text; PP's member-identity binding moves evaluate (850),
# check_at (441) and the site by 13 more (909), text unchanged.
# T4-S5 (epoch): the TieRodReference variant and binding arm move check_at (+19), evaluate and the site (+50), and the
# load_closure sites (+19/+45), text unchanged.
FIRST_RETURN_LINE = 960
FIRST_RETURN_SITE = "structural/formation_check.rs:%d" % FIRST_RETURN_LINE
FIRST_RETURN_WHY = "B7aFK895: map_err formats only on Err and immediate? returns evaluate; preserve full evaluate calls/content/capacity and every helper/loop/copy cost"
FIRST_RETURN_RULE = {"from": ["structural/formation_check.rs:914:evaluate"],
                     "per": "1", "why": FIRST_RETURN_WHY}


def first_return_rules(snapshot, resolved):
    inspect(snapshot)
    return [(resolved(FIRST_RETURN_SITE), {
        "from": [resolved(FIRST_RETURN_RULE["from"][0])],
        "per": "1", "why": FIRST_RETURN_WHY})]


def first_return_state(snapshot, table, graph, formats, lexical):
    inspect(snapshot)
    if table.get("b7a_reformation_first_error") != "FK895-v1":
        raise ValueError("B7aFK895: missing/versioned first-return guard")
    actual = table.get("site_from", {}).get(FIRST_RETURN_SITE)
    expected = dict(FIRST_RETURN_RULE, why="SQ2 G5: " + FIRST_RETURN_WHY)
    if actual != expected:
        raise ValueError("B7aFK895: exact source/per/why rule changed")
    c6 = runpy.run_path(os.path.join(os.path.dirname(__file__), "kd5_first_error_model.py"))
    site = {"file": FIRST_RETURN_FILE, "line": FIRST_RETURN_LINE, "kind": "format",
            "fn": FIRST_RETURN_ROOT, "roots": [FIRST_RETURN_ROOT]}
    for family in ("site_from", "site_zero", "site_total", "site_aggregate", "site_size", "lex_site_size"):
        for alias in c6["override_aliases"](site):
            if family == "site_from" and alias == FIRST_RETURN_SITE:
                continue
            if alias in table.get(family, {}):
                raise ValueError("B7aFK895: conflicting override: " + family + ":" + alias)
    edges = graph["edges"]
    if FIRST_RETURN_ROOT not in edges or FIRST_RETURN_PARENT not in edges:
        raise ValueError("B7aFK895: missing qualified root/caller node")
    parents = [caller for caller, targets in edges.items() for target in targets if target == FIRST_RETURN_ROOT]
    if parents != [FIRST_RETURN_PARENT]:
        raise ValueError("B7aFK895: qualified evaluate caller inventory changed")
    matching = [row for row in formats + lexical if
                (row["file"], row["line"], row["kind"]) == (FIRST_RETURN_FILE, FIRST_RETURN_LINE, "format")]
    if len(matching) != 1 or any(row.get("fn", FIRST_RETURN_ROOT) != FIRST_RETURN_ROOT for row in matching):
        raise ValueError("B7aFK895: missing/duplicate/misqualified format site")
    return {"sites": [site]}


def first_return_witness(state, file, line, kind, mult, counts, rebound=False):
    # Reuse C6's strict positive and exact post-rebind count checks. This
    # targets one allocation only; it never limits helper visits or a graph.
    if (file, line, kind) != (FIRST_RETURN_FILE, FIRST_RETURN_LINE, "format"):
        return
    c6 = runpy.run_path(os.path.join(os.path.dirname(__file__), "kd5_first_error_model.py"))
    check = c6["rebound_witness"] if rebound else c6["positive_witness"]
    check(state, file, line, kind, mult, counts)
