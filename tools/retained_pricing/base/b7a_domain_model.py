"""B7a stage-1 ordinary member-load and named-basis positive owner bounds.

The guarded source admits no states/components/partial extents. Existing
positive families remain; these terms widen their storage/multiplicity basis.
"""
import hashlib
import json
import os
import re
import runpy

SOURCE_FILES = {"core/product_physics/src/lib.rs":"d1a3675f3c59630b69bd35de5896f1877264eb484eb980466ca057de315396db","core/product_physics/src/certified_loads.rs":"d695564353f1ce177fcb0f1f91c8ee00ab3bf8ad57bdeb9cf25f5501b11a67b1","core/product_physics/src/retained_product.rs":"e0cf6a0681a7ecfc01a41cf18ef83c4d8789b8f45811c20e9707b3cc8c18a27c","core/product_physics/src/retained_wire.rs":"31484751460173cbefe3c9328efe42d6e2313746988ac7bb5bc836460c959b7f","core/product_physics/src/certified_publication.rs":"635b29d6e52dfeec94812b69a6e60b80fa3ec3630cc76edee799e82f763dc886","core/solver/frame_kernel/src/load_ledger.rs":"8ad71203f34a1c38552530296875c0b051f9a5f4f9990d25f748dac8d3f0584f","core/solver/frame_kernel/src/structural/formation_check.rs":"d6305e0b55708dfd107dc74460d90af46377ccf7106c8d7d74d371baf38419df","core/solver/frame_kernel/src/structural/certified_recovery.rs":"953da59448e8a47653e23b989ab1f46d00dc0a93e15d8152ea890df37ee671fd","core/loads/primitive_loads/src/lib.rs":"f0536b0c0bfdb0df69e7a8dd251dfb0b15607caf3ad693fa1adc3b912152f121","core/solver/straight_pipe/src/lib.rs":"b743d7ac67312253d39bd7fe772fbff3e2b320eeaca6a41dda6d256ed9af4856","core/product_physics/src/preview_physics.rs":"8b24d69c79e7dd0e7fd8d375b74042ea5e22216eaa8fe5449f1eb915eb2b45b1","core/reporting/result_export/src/retained_precision.rs":"241d8db741fba3b7204c3bd5936d4eee58a2cad8a819865fa2d7fd0ca3dd9205","core/product_physics/src/model_format.rs":"a47f529dc22869d3435552337bfe7032f133c654a3a2faeeb1a83fc7059548df","core/product_physics/src/pressure_runtime.rs":"b876348a88047dc8af9354d909a74cbe07db00931cf78215412525357f873fa5","core/product_physics/src/source_receipt.rs":"dc36c7f8a75377ea61abb82322d572798388e632cc087c31b3bf3f642040af0d","core/product_physics/src/exact_admission.rs":"c4beff11ee573e89ce7774a5d46367c2e4026e78542efb4aeaa50172d49791b0","core/product_physics/src/case_state/mod.rs":"17f17f20c6c3556af1c3465418f0fa964fbdf2db4a5ba1b49c5914e90d7c75b5","core/solver/nonlinear_integration/src/structural_adapter.rs":"7405c5cf32f59edf1c590859868e9af76bd08803aee3eea9b99a44be2aa5893b","core/solver/frame_kernel/src/lib.rs":"f21e07046cba2909b67694d429a11dab915a0e27ea28ea79878a18b83bdd2843","core/reporting/result_export/src/semantic_contract.rs":"5f727dce170c2674465a6fb2a5455070c22deff84aafb9996e5d03a00e4f7e1d","core/reporting/result_export/src/physics_source.rs":"12bab9917680ca6bbf2de59ee337459c5e8a2db80eb9fa9edb83963cbae007c1"}
CAPS_PIN = "e6759d84e480f0480f381933d4d36d9891f103158c35874fc7dd4acae7d9e71d"
FAMILY_PIN = "e640cc8365bdd6634734be237345fcb375a3e1ee7a653bb42a6067fa8640bb3a"
ADMISSION_MARKER = "// ---- D-6: the build status --------------------------------------------------"
ADMISSION_PREFIX_PIN = "f4063371220434eaa3f95d4df61354543c40aec4901ce84490a6af78eb730ae7"
ADMISSION_LATE_PINS = {"law_order":"b0f9eae8c2d716f14cfda1ff895a65d7833b086383ec34154b501daa1ac344a3","admit":"67334c60ce1a606b5cd73e2715474cfc40150a11def9143752993af4c79904c8","admission":"7627646ef640f23a095bab2f355c7576e1ff79baeb2a9ea44be4550765f60003"}
MATERIALS = 4
OPERAND_WORDS = 26
MEMBER_SOURCE = 268
FAMILY_BYTES = 16

STATIC_PREIMAGE = ('schemas/physics_source_recovery.schema.json', 'schemas/retained_precision_mp_v2.schema.json', 'fixtures/results/retained_precision_prepared_ordinary_v1.json', 'fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json', 'fixtures/results/semantic_contract_v0_2.json', 'fixtures/results/semantic_contract_v0_3_physics_1.json', 'fixtures/results/semantic_contract_v0_3_load_reference_1.json', 'fixtures/results/semantic_contract_v0_3_load_reference_source_1.json', 'fixtures/results/semantic_contract_v0_3_preview_physics_1.json', 'fixtures/results/semantic_contract_v0_3_physics_source_1.json', 'fixtures/results/semantic_contract_v0_3_source_blocks_1.json', 'schemas/source_block_recovery.schema.json')
STATIC_OWNERS = (('core/reporting/result_export/src/physics_source.rs:schema', 'schemas/physics_source_recovery.schema.json'), ('core/reporting/result_export/src/retained_precision.rs:schema', 'schemas/retained_precision_mp_v2.schema.json'), ('core/reporting/result_export/src/retained_precision.rs:definition', 'fixtures/results/retained_precision_prepared_ordinary_v1.json'), ('core/reporting/result_export/src/retained_precision.rs:table', 'fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:physics_contract', 'fixtures/results/semantic_contract_v0_3_physics_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:load_reference_contract', 'fixtures/results/semantic_contract_v0_3_load_reference_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:load_reference_source_contract', 'fixtures/results/semantic_contract_v0_3_load_reference_source_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:preview_physics_contract', 'fixtures/results/semantic_contract_v0_3_preview_physics_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:physics_source_contract', 'fixtures/results/semantic_contract_v0_3_physics_source_1.json'), ('core/reporting/result_export/src/retained_precision.rs:combination_definition', 'fixtures/results/retained_precision_prepared_combination_v1.json'), ('core/reporting/result_export/src/retained_precision.rs:exact_definition', 'fixtures/results/retained_precision_prepared_exact_v1.json'), ('core/reporting/result_export/src/semantic_contract.rs:physics_retained_contract', 'fixtures/results/semantic_contract_v0_3_physics_retained_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:preview_physics_retained_contract', 'fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json'), ('core/reporting/result_export/src/retained_precision.rs:exact_table', 'fixtures/results/semantic_contract_v0_3_physics_retained_1.json'))


def inspect(snapshot):
    fingerprints = {}
    for file, expected in SOURCE_FILES.items():
        with open(os.path.join(snapshot, file), "rb") as stream:
            raw = stream.read()
        fingerprints[file] = hashlib.sha256(raw).hexdigest()
        if fingerprints[file] != expected:
            raise ValueError("B7a positive-domain owner source changed: " + file)
    with open(os.path.join(snapshot, "core/product_physics/src/retained_memory.rs"), "rb") as stream:
        source = stream.read().decode("utf-8")
    # The whole raw nongenerated prefix binds actual census, capacity and
    # domain enforcement. Mutable registration/generated arrays are excluded.
    if source.count(ADMISSION_MARKER) != 1:
        raise ValueError("B7a positive-domain admission boundary changed")
    if hashlib.sha256(source.split(ADMISSION_MARKER)[0].encode()).hexdigest() != ADMISSION_PREFIX_PIN:
        raise ValueError("B7a positive-domain census/enforcement source changed")
    kd5 = runpy.run_path(os.path.join(os.path.dirname(__file__), "kd5_text_model.py"))
    function_body = kd5["function_body"]
    for name, digest in ADMISSION_LATE_PINS.items():
        body, _ = kd5["_indented_function"](source, name)
        if hashlib.sha256(body.encode()).hexdigest() != digest:
            raise ValueError("B7a positive-domain late admission source changed: " + name)
    caps = re.search(r"^pub\(super\) mod caps \{.*?^\}", source, re.S | re.M)
    if caps is None or hashlib.sha256(caps.group().encode()).hexdigest() != CAPS_PIN:
        raise ValueError("B7a positive-domain admission caps changed")
    if hashlib.sha256(function_body(source, "family_clauses").encode()).hexdigest() != FAMILY_PIN:
        raise ValueError("B7a positive-domain family clauses changed")
    return {"full_raw_source_fingerprints": fingerprints,
            "admission_caps_sha256": CAPS_PIN, "family_clauses_sha256": FAMILY_PIN,
            "admission_prefix_sha256": ADMISSION_PREFIX_PIN, "admission_late_sha256": ADMISSION_LATE_PINS,
            "model_sha256": hashlib.sha256(open(__file__, "rb").read()).hexdigest(),
            "formed_terms_per_input": 12, "manifest_operand_words": OPERAND_WORDS,
            "basis_material_count": MATERIALS, "named_basis_count": 3,
            "reader_static_owners": STATIC_OWNERS,
            "resident_json_inputs": list(dict.fromkeys(path for _, path in STATIC_OWNERS))}


def basis_lengths(ident):
    provenance = 161 + 4 * ident
    joined = MATERIALS * provenance + 3 * (MATERIALS - 1)
    record = 85 + ident + joined
    return provenance, joined, record


def primitive_copy(terms):
    """One complete owned FormationSource.loads/ledger primitive copy."""
    from g3lib import V, B, textcap
    identity = textcap(FAMILY_BYTES) + textcap(MEMBER_SOURCE)
    return V("LoadPrimitive", terms) + B(terms * identity)


def manifest_copy(terms):
    """One independent one-entry manifest for each formed term, including children."""
    from g3lib import s, B, pushcap, textcap
    # Child arrays each have one entry: keep the existing 4q+2x upper
    # for q=terms, x=terms, rather than treating them as one shared Vec.
    identity = textcap(FAMILY_BYTES) + textcap(MEMBER_SOURCE)
    return s("LoadClosureEntry") * (6 * terms) + B(
        terms * (identity + 8 * pushcap(OPERAND_WORDS)))


def formed_constructor(terms):
    """Conservative sum over attach's source, expected and supplied staging."""
    from g3lib import V, B, textcap
    identity = textcap(FAMILY_BYTES) + textcap(MEMBER_SOURCE)
    # These constructors are sequential per term. Reserving all terms'
    # source/supplied/temp primitive backings together is an upper, not a
    # claim that they coexist or that move elision removes storage.
    return (V("CertifiedSummand", terms) + V("LoadPrimitive", terms) * 2
            + B(3 * terms * identity))


def custody(named, ident):
    """Invocation-owned exact-growth bases/selections, with no per-case cloning."""
    from g3lib import s, Xc, B
    _, _, record = basis_lengths(ident)
    requested = (Xc("BasisCustody", named)
                 + (Xc("MaterialSelection", MATERIALS)
                    + B(MATERIALS * 5 * ident + 3 * ident + record)) * named)
    # try_reserve_exact(1), not the geometric Vec.push law: each old
    # exact backing can have n-1 elements while the new backing is live.
    moving = (s("BasisCustody") * max(named - 1, 0)
              + s("MaterialSelection") * (MATERIALS - 1))
    return requested, moving


def cached_basis_headers(named, ident):
    from g3lib import V, s, B, pushcap, textcap
    _, _, record = basis_lengths(ident)
    # BASE is retained too; every named state owns its cloned key and
    # formatted record beside its materials/BuiltModel/BasisStiffness.
    count = 1 + named
    requested = V("BasisState", count) + B(
        named * (textcap(6 + ident) + textcap(record)))
    moving = s("BasisState") * (pushcap(count) // 2)
    return requested, moving


def resolver_scratch(members, ident):
    from g3lib import HashReq, V, B, s, textcap
    provenance, joined, record = basis_lengths(ident)
    # HashSet collect sees every pipe in its size_hint, even when only
    # four distinct materials remain. One resolver/build runs at a time.
    requested = (HashReq("&str", members) + V("String", MATERIALS)
                 + B(MATERIALS * textcap(provenance) + textcap(joined)
                     + textcap(6 + ident)))
    moving = (hash_moving("&str", members)
              + s("String") * (pushcap_for_materials() // 2)
              + B(max(textcap(provenance), textcap(joined), textcap(record))))
    return requested, moving


def pushcap_for_materials():
    from g3lib import pushcap
    return pushcap(MATERIALS)


def record_map(cases, ident):
    from g3lib import HashReq, B
    _, _, record = basis_lengths(ident)
    return (HashReq("(String,String)", cases) + B(cases * (ident + record)),
            hash_moving("(String,String)", cases))


def hash_moving(kind, count):
    from g3lib import X, s, B, buckets
    previous = buckets(count) // 2 if buckets(count) > 4 else 0
    return s(kind) * previous + B(previous + 8 + 15) if previous else X()



def formation_children(terms):
    """Original force retains each moved Formation::Exact f64 child."""
    from g3lib import B, pushcap
    return B(8 * terms * pushcap(80))


def formation_constructor():
    """One straight uniform call: conservative sum of distinct construction owners."""
    from g3lib import Xc, B, pushcap
    # All63 expansion_components outputs per single uniform call plus the
    # two initial variable children. Counting them together conservatively
    # covers requested/moving allocation, not just a pair's coexistence.
    # Keeping returnedRows here as well as in resident q is conservative.
    children = 63 + 2
    requested = (Xc("ExactAccumulator", 14)
                 + Xc("Vec", 2 + 8 + 6 + 12 + 12 + 3)
                 + B(children * 8 * pushcap(80))
                 + Xc("Option<FormationRecord>", 12)
                 + Xc("SpannedUniformLocalLoad", 3) + B(3 * 12 * 8))
    # Expansion growth is geometric; at most the old64-f64 backing can
    # remain beside a new128-f64 child. Sequential calls do not multiply it.
    moving = B(8 * (pushcap(80) // 2))
    return requested, moving



TEXT_LOOPS = [

    ("for (local_dof, &global_dof) in dof_map.iter().enumerate()", "12",
     ["lib.rs:add_uniform_element_loads"]),
    ("for ((slot, global), formation) in element_dof_map(pipe.node_i.index, pipe.node_j.index) .iter() .enumerate() .zip(formations)", "12",
     ["lib.rs:add_uniform_element_loads"]),
    ("for load in uniform_loads", "1", ["lib.rs:exact_scaled_intended"]),
    ("iter:variables .iter().map", "2", ["lib.rs:exact_scaled_intended"]),
    ("iter:self.early_sources.iter().filter_map(Option::as_ref).filter_map", "cases",
     ["retained_product.rs:pending_error_bytes"]),
    ("for entry in quality.cases.iter_mut().filter(|entry|entry.basis_ref.ref_id==case)", "cases",
     ["retained_product.rs:failed_manifest"]),
    ("for (index, selector) in selectors.into_iter().enumerate()", "cases",
     ["retained_wire.rs:material_basis"]),
    ("for &i in &used", "4", ["retained_wire.rs:material_basis"]),
    ("for location in FAILED_STATIONS", "3", ["retained_product.rs:failed_manifest"]),
    ("for (index, mut row) in old.into_iter().enumerate()", "Pall", ["preview_physics.rs:render"]),
    ("for term in &self.terms", "l", ["retained_product.rs:check_early_occurrences"]),
    ('for o in list(&b[" "])', "cases", ["retained_precision.rs:g5_requested_mode"]),
    ('for d in list(&source[" "])', "D", ["retained_precision.rs:g5_requested_mode"]),
    ("for (index, (seed, refs)) in pc.ordinary.iter().zip(diagnostic_refs).enumerate()", "cases",
     ["retained_wire.rs:serialize_cases_with"]),
    ("for index in 0..self.cases_seen()", "cases", ["retained_product.rs:settle_ordinary_terminal"]),
    ("for selection in &custody.selections", "4", ["retained_product.rs:validate_selected_case"]),
    ("for count in counts.iter()", "cases", ["retained_product.rs:bind_observations_by_case"]),
    ("for row in &mut rows", "P", ["retained_product.rs:failed_manifest"]),
]


def text_decisions(snapshot, resolved):
    """Site-specific positive classes and O10's existing exact owner checks."""
    inspect(snapshot)
    # Full raw owner guards above bind the functions named by each exact
    # header; O10's standard post-run match check refuses another owner.
    loops = [(header, bound, "B7a source-bound stage-1 owner: " + ", ".join(owners), owners)
             for header, bound, owners in TEXT_LOOPS]
    pp = "core/product_physics/src/"
    re_ = "core/reporting/result_export/src/"
    inp = {"bytes": 128, "source": "IN128"}
    static = {"bytes": 64, "source": "STATIC"}
    audits = [

        (pp + "lib.rs:10627", {"load_case_id": inp}),
        (pp + "lib.rs:10641", {"stable_suffix(load_case_id)": inp, "stable_suffix(&load.load_id)": inp}),
        (pp + "lib.rs:10648", {"load.load_id": inp, "bend.pipe_id": inp}),
        (pp + "lib.rs:10653", {"load.load_id": inp}),
        (pp + "lib.rs:10654", {"bend.pipe_id": inp}),
        (pp + "lib.rs:10655", {"bend.component_id": inp}),
        (pp + "lib.rs:10656", {"load_case_id": inp}),
        (pp + "lib.rs:10715", {"stable_suffix(load_case_id)": inp, "stable_suffix(&load.load_id)": inp}),
        (pp + "lib.rs:10722", {"load.load_id": inp, "bend.pipe_id": inp}),
        (pp + "lib.rs:10727", {"load.load_id": inp}),
        (pp + "lib.rs:10728", {"bend.pipe_id": inp}),
        (pp + "lib.rs:10729", {"bend.component_id": inp}),
        (pp + "lib.rs:10730", {"load_case_id": inp}),
        (pp + "lib.rs:10769", {"stable_suffix(&load.load_id)": inp}),
        (pp + "lib.rs:10776", {"load.load_id": inp, "load_case_id": inp}),
        (pp + "certified_loads.rs:233", {"case_id": inp}),
        (pp + "certified_loads.rs:256", {"case_id": inp}),
        (pp + "retained_product.rs:4298", {"stable_suffix(id)": inp}),
        (pp + "retained_product.rs:4299", {"id": inp}),
        (pp + "retained_product.rs:4307", {"id": inp, "id.len()": inp, "component": static}),
        (pp + "retained_product.rs:4309", {"id": inp}),
        (pp + "retained_product.rs:4310", {"component": static}),
        (pp + "retained_product.rs:4327", {"member.id": inp, "member.id.len()": inp}),
        (pp + "retained_product.rs:4332", {"member.id": inp}),
        (pp + "retained_product.rs:4358", {"ordinary.document_kind": static}),
        (pp + "retained_product.rs:4359", {"ordinary.model_ref": inp}),
        (pp + "lib.rs:1008", {"semantic_contract_id": static}),
        (pp + "lib.rs:3079", {"context.semantic_id()": static}),
        (pp + "lib.rs:5314", {"support.id": inp}),
        (pp + "lib.rs:5985", {"pipe.element_id": inp}),
        (pp + "model_format.rs:27", {"version": static}),
        (re_ + "retained_precision.rs:5536", {"mode": static}),
        (re_ + "retained_precision.rs:5618", {"text(cid)": inp}),
        (re_ + "retained_precision.rs:5642", {"text(cid)": inp}),
    ]
    lex = [("retained_product.rs:4356", {"class": 64, "why": "source-bound mechanics schema literal"}),
           ("retained_product.rs:4358", {"class": 2914, "why": "positive formulation content upper; actual struct/Vec storage is separate"}),
           ("retained_product.rs:4362", {"class": 2914, "why": "positive summary-owned text upper; inline header and LocatedQuantity children stay separate"})]
    return loops, [(resolved(key), values) for key, values in audits], [(resolved(key), value) for key, value in lex]


def failed_envelope_copy(cases, ident):
    from g3lib import s, V, B
    # A failed Preview case is the only admitted owner. Keep the PF9-limit
    # 2035-byte union rather than the tighter blocked8-limit1649 bytes.
    # The complete real envelope atom includes inline basis/Summary headers.
    return (s("MechanicsEnvelope") + V("String", 9) + B(2035)
            + B(2 * (2 * ident + 1024))) * cases

BASE_PREIMAGES = {'loop_bounds_sha256': 'cd0273bc94e521142cc56121171055f9020c167b97caa8efe7fa6ac36c69ba4c', 'loop_rows': {'116': {'re': '^for\\ \\(mi,\\ mb\\)\\ in\\ list\\(\\&b\\["\\ "\\]\\)\\.iter\\(\\)\\.enumerate\\(\\)$', 'bound': '1', 'why': "I104 SQ B1 re-checked, kept (1): RS reader (:747 g3, :3616 g8): body.material_bases: the producer emits one material basis (D1.5's one default basis) whose case_indices name every case (retained_wire.rs material_basis). Was: G4 (T17 precommit reader at b1f80234dc): a receipt body array other than builds: cases, sources, material_bases, calls, groups, ordinary_attempts, product_attempts and legacy_source_work each hold <= 1 entry in D1 (one case, no combinations, one native call and run; retained_wire.rs one_case/one_run, finish)"}, '218': {'re': '^for term in force_terms$|^for force in force_terms \\.iter\\(\\) \\.filter', 'bound': 'l', 'why': 'G4 (R-1, newly reached on the repaired graph): force terms: one per nodal load primitive <= l (D1.7)'}, '313': {'re': 'span\\.start_fraction|uniform_intensities', 'bound': '0', 'why': 'D1.7: no element (distributed) loads, so no load spans (PP lib.rs:9999-10015 at b1f80234dc). G4 (RV84 S-2): bare `intensity` removed (it matched the 3-accumulator zip)'}}, 'fn_zero': {'fn': 'lib.rs:10631:add_uniform_element_loads', 'why': 'D1.7: iterates load_application.element_uniform_loads (lib.rs:3223-3226), empty for nodal-only loads; every text site is inside that loop'}, 'edge_zero': {'caller': 'formation_guard.rs:297:load_row_finding', 'callee': 'formation_guard.rs:216:decide_row', 'why': 'D1.7 nodal-only loads: every formation slot is None, so load_row_finding returns at its has_formation_records() check (formation_guard.rs:303-305; FK load_ledger.rs:307-309; I54 correction ADDENDUM)'}, 'record_audit': {'bytes': 600, 'source': 'COMP'}, 'provenance_join': {'re': '^provenance_records$', 'bytes': '8*(128+200)+8*3', 'why': 'composite_text provenance_join'}, 'copy_lex': {'class': 'result_id', 'why': "adapter copy(): identifiers, units, entity refs, result ids and the basis text (retained_product.rs:247-3452), each <= result_id (G6 repair, RV87 SF-2: re-keyed from b1f80234dc's line 2963 to copy()'s push_str at 1e323058f3, retained_product.rs:3125)"}}

def prepare(snapshot, base, output, inherited):
    """Replay positive source-premise changes; never edit the historical base."""
    proof = inspect(snapshot)
    loop_path = os.path.join(base, "loop_bounds.g4.json")
    raw = open(loop_path, "rb").read()
    if hashlib.sha256(raw).hexdigest() != BASE_PREIMAGES["loop_bounds_sha256"]:
        raise ValueError("B7a positive-domain loop-rule base changed")
    loops = json.loads(raw)
    alterations = []
    for index, bound, why in ((116, "cases", "one material basis per distinct requested selector"),
                              (218, "12*l", "at most twelve formed terms per admitted full-span uniform input"),
                              (313, "l", "at most one full-span uniform contribution per admitted input")):
        expected = BASE_PREIMAGES["loop_rows"][str(index)]
        assert loops["loops"][index] == expected
        changed = dict(expected, bound=bound, why="B7a stage 1: " + why)
        loops["loops"][index] = changed
        alterations.append({"family": "loops", "index": index, "before": expected, "after": changed})
    for family, index in (("fn_zero", 19), ("edge_zero", 0)):
        expected = BASE_PREIMAGES[family]
        if loops[family][index] != expected or loops[family].count(expected) != 1:
            raise ValueError("B7a positive-domain exact zero preimage changed")
        loops[family].pop(index)
        alterations.append({"family": family, "index": index, "before": expected,
                            "why": "already-admitted full-span uniform loads execute this owner"})
    text_path = inherited.get("text_args.g4.json", os.path.join(base, "text_args.g4.json"))
    text_raw = open(text_path, "rb").read()
    text = json.loads(text_raw)
    key = "core/product_physics/src/lib.rs:10230"
    if text["id_audit"][key]["basis_record"] != BASE_PREIMAGES["record_audit"]:
        raise ValueError("B7a positive-domain record audit preimage changed")
    text["id_audit"][key]["basis_record"] = {"bytes": 2914, "source": "COMP"}
    old_join = BASE_PREIMAGES["provenance_join"]
    if text["join_rules"].count(old_join) != 1:
        raise ValueError("B7a positive-domain provenance join preimage changed")
    new_join = dict(old_join, bytes="2701", why="B7a named point: four 673-byte records plus three 3-byte separators")
    text["join_rules"][text["join_rules"].index(old_join)] = new_join
    lex = "retained_product.rs:3775"
    if text["lex_site_size"][lex] != BASE_PREIMAGES["copy_lex"]:
        raise ValueError("B7a positive-domain adapter-copy preimage changed")
    text["lex_site_size"][lex] = {"class": 2914, "why": "B7a named basis record; other original copied identifiers remain bounded"}
    text["args"].insert(0, {"re": r"^(basis_record|metadata\.basis|basis_label)$", "max": 2914,
                            "why": "B7a named point aggregate record: 85+128+4*(161+4*128)+9 bytes"})
    alterations += [{"family": "id_audit", "key": key, "field": "basis_record", "bytes": 2914},
                    {"family": "join_rules", "before": old_join, "after": new_join},
                    {"family": "lex_site_size", "key": lex, "class": 2914},
                    {"family": "args_first", "rule": text["args"][0]}]
    os.makedirs(output, exist_ok=True)
    paths = {}
    for name, data in (("loop_bounds.g4.json", loops), ("text_args.g4.json", text)):
        path = os.path.join(output, name)
        with open(path, "w", encoding="utf-8") as stream:
            json.dump(data, stream, indent=1)
        paths[name] = path
    proof.update({"base_loop_sha256": hashlib.sha256(raw).hexdigest(),
                  "input_text_rules_sha256": hashlib.sha256(text_raw).hexdigest(),
                  "prepared_files": {name: hashlib.sha256(open(path, "rb").read()).hexdigest()
                                     for name, path in paths.items()}, "alterations": alterations})
    return paths, proof

def reader_static_files(snapshot, historical):
    """Closed actual loader ownership, including two independently cached duplicates."""
    inspect(snapshot)
    if tuple(historical) != STATIC_PREIMAGE:
        raise ValueError("B7a historical reader static inventory changed")
    # Twelve distinct inputs, fourteen actual OnceLock<Value> allocations.
    # Never deduplicate by path or skip an absent file: g4's unchanged
    # static_tree parser must account every listed allocation positively.
    return [path for _, path in STATIC_OWNERS]
