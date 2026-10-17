"""B7a stage-1 ordinary member-load and named-basis positive owner bounds.

The guarded source admits no states/components/partial extents. Existing
positive families remain; these terms widen their storage/multiplicity basis.
"""
import hashlib
import json
import os
import re
import runpy

# B7b D3: the full-file pins are B7b's actual source (retained-pricing-rules.md, B7b section);
# pressure_runtime.rs's later change is inside its `mod tests` only (production prefix identical);
# physics_source.rs adds only the two resolved-branch requires (033626030c) and test include paths;
# ccb19ba0bf adds the readers' zero-corrosion-allowance checks and two guards (retained-pricing-rules.md, B7b-ZA).
# T4-U6/U8 D3 (retained-pricing-rules.md, "T4-U6/U8: actual-source D3"): the changed owner files are re-bound to the
# priced source after their diffs were read; the D-5 and Timoshenko changes add no owner this model prices.
# PP (503180f78a..20c6126a16): lib.rs, certified_loads.rs, retained_product.rs, retained_wire.rs (one hash constant),
# certified_publication.rs, formation_check.rs, certified_recovery.rs, straight_pipe, pressure_runtime.rs and
# structural_adapter.rs change for the Timoshenko admission, the D-5 exact mass and member-identity frame binding:
# frame_members copies are priced by U8-FM; no owner this model prices changes.
# Option A (433fc1f1e4): source_receipt.rs changes only `hash` (U6-SH: a borrowed wrapper through `hash_text`);
# physics_source.rs's actual_materials adds one Err-only `require(.., "ACTUAL_SHEAR_MODEL")` per pipe in its existing
# loop (18 B, inside the 40 B detail class) and a test module.
# Epoch addbdc030e (PP's realized-arc marker): lib.rs's `pipe_sections` moves into `exact_pipe_sections`, which with
# `realized_arc` and `mark_realized_arc` replaces the same expressions in the case solve, `assemble_pressure_case`
# (pressure_runtime.rs, region path only) and T25's `physical_source_built`; the marker is set only on a realized arc
# (none on D1: no components). No owner this model prices changes; the moved keys below are re-keyed.
# Final source 588de796a4 (REG2): lib.rs's withhold_certified_summaries also clears `max_stress_reduced` (PP4: a drop,
# no allocation) and a test; physics_source.rs compares its links and twins by `section_basis::bit_equal` (RD3, RD4:
# a recursive comparison that allocates nothing; `number` errs only on a non-number) and exposes
# `normalized_quantity` to reader_verdict.rs's pressure-1 model binding (unreached from the retained roots, B7b-ZA).
# No owner this model prices changes; no require site is added or removed.
# W1 stage2: the reviewed member capture/wire/reader changes bind the current
# cohort here; their additional owners are positive in w1_member_load_model.
# Current site keys are carried by their unchanged text/function identity.
# Historical BASE_PREIMAGES below remain keyed to the original basis.
SOURCE_FILES = {'core/product_physics/src/lib.rs': 'a5c5b09f31a996c19ff63ace14bd39a07b7a474d6b03556a19825057e90aed74', 'core/product_physics/src/certified_loads.rs': '56f2a4ef24ccca359290d7b25ab2078487d89da908826dd3f3897e14fdd35c8f', 'core/product_physics/src/retained_product.rs': '69a87004ff692dd577c2b4bd0f7a17e621cd5d4ea07b29d55ddc7618200ef025', 'core/product_physics/src/retained_wire.rs': '9d4fbf77f8c800dc0fc0800d3be417be49f1d95c678945b4b0a4557c6515bf13', 'core/product_physics/src/certified_publication.rs': '0008eb462f94640386194afb3659fdc3d74fb2e45ae690b7b2646db5feb8be57', 'core/solver/frame_kernel/src/load_ledger.rs': '8ad71203f34a1c38552530296875c0b051f9a5f4f9990d25f748dac8d3f0584f', 'core/solver/frame_kernel/src/structural/formation_check.rs': '4e2b2aed3a155b3893d9bab353aad112248ca555b4016b59a075cb0ec719cdf2', 'core/solver/frame_kernel/src/structural/certified_recovery.rs': 'e9a0bb4220482e9596e0e11ec698ae2291da5d223865d21ce9d7e2725594f7af', 'core/loads/primitive_loads/src/lib.rs': 'f0536b0c0bfdb0df69e7a8dd251dfb0b15607caf3ad693fa1adc3b912152f121', 'core/solver/straight_pipe/src/lib.rs': 'a375535794d2700ffb08621aebe6f587a2d226f107056f320ee23fef466d1bb7', 'core/product_physics/src/preview_physics.rs': '61b2fb2f3f9bc3aa9803bf6bf5c95cf5e51c841289ef5c415bf86fe00f33b468', 'core/reporting/result_export/src/retained_precision.rs': '7772f539ff2a2a2d75805a1e1a1c634b23fa56fdc63b8596954c89b067d1533e', 'core/product_physics/src/model_format.rs': '5b96cf74cbeb8c4a4fe11509e634525d3384c56cb9ff37b11a3c12d39a1df71f', 'core/product_physics/src/pressure_runtime.rs': 'e0897c0216eb1d774e15d44c1641bc33e2b3b5a01a1da7ec192ea722fc5b8de2', 'core/product_physics/src/source_receipt.rs': 'a74f58877c0fe80bc8b5e36f55af2524c366a85c97a0137e92711539cb23f8c9', 'core/product_physics/src/exact_admission.rs': '493e5876cdc5fbbe593a1435a6c8b235323a2a76f8b339f41fe14e8e541eec7b', 'core/product_physics/src/case_state/mod.rs': '633b46de9fb4fdd5d213efbba1405e65afbc291ad8f9848096596fce27b06fab', 'core/solver/nonlinear_integration/src/structural_adapter.rs': 'c71d4acf1aa3f2d5c048eefe3d559b192d31c2d962083cf89bd4ef978347d9cd', 'core/solver/frame_kernel/src/lib.rs': '76d2b727e5ccaa051ffda7daad7ea0b299741a70b8388a54b310b5570f7cffa9', 'core/reporting/result_export/src/semantic_contract.rs': 'c19cdf808085d9a43ad985ad56146093b4124975a0d764969718f276e480db45', 'core/reporting/result_export/src/physics_source.rs': 'cdf30d5ade495af00456612daec1a85dce6f5d83043174fefe31897484be92e9'}
CAPS_PIN = "e6759d84e480f0480f381933d4d36d9891f103158c35874fc7dd4acae7d9e71d"
# T4-U6/U8 (O4): family_clauses adds one refusal, a nonzero mill tolerance or corrosion allowance on branch E
# (FamilyFact::SectionAllowance), and namespace_branch refuses the Timoshenko exact contract (FamilyFact::ShearModel).
# Both only narrow admission; the caps, census and late admission bodies are unchanged.
FAMILY_PIN = "41b97bbb844b8e25bee51b9252edd1e00565766bc435df1e38c08b0bd7ea7486"
ADMISSION_MARKER = "// ---- D-6: the build status --------------------------------------------------"
ADMISSION_PREFIX_PIN = "1b88ca84e9a98b811f867849e80c0e6ae360cd83dd7f1a4c2832b71da8907656"
ADMISSION_LATE_PINS = {"law_order":"b0f9eae8c2d716f14cfda1ff895a65d7833b086383ec34154b501daa1ac344a3","admit":"67334c60ce1a606b5cd73e2715474cfc40150a11def9143752993af4c79904c8","admission":"7627646ef640f23a095bab2f355c7576e1ff79baeb2a9ea44be4550765f60003"}
MATERIALS = 4
OPERAND_WORDS = 26
MEMBER_SOURCE = 268
FAMILY_BYTES = 16

STATIC_PREIMAGE = ('schemas/physics_source_recovery.schema.json', 'schemas/retained_precision_mp_v2.schema.json', 'fixtures/results/retained_precision_prepared_ordinary_v1.json', 'fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json', 'fixtures/results/semantic_contract_v0_2.json', 'fixtures/results/semantic_contract_v0_3_physics_1.json', 'fixtures/results/semantic_contract_v0_3_load_reference_1.json', 'fixtures/results/semantic_contract_v0_3_load_reference_source_1.json', 'fixtures/results/semantic_contract_v0_3_preview_physics_1.json', 'fixtures/results/semantic_contract_v0_3_physics_source_1.json', 'fixtures/results/semantic_contract_v0_3_source_blocks_1.json', 'schemas/source_block_recovery.schema.json')
# B7b (A4/A5, O-10): physics_contract, load_reference_contract,
# load_reference_source_contract and physics_source_contract are deleted
# with their tables. pressure_rows_contract() (the ordinary pressure-1 table)
# and pressure_source_contract() (the pressure-source-1 table) are distinct
# resident OnceLock<Value> owners and are priced positively in their own right.
# B7b D3 review (b7-plan D2): load_reference::transport_schema()'s
# OnceLock<Result<Value, String>> over the pinned load_reference_state schema
# is a surviving independent process-lifetime owner, priced as its Ok Value.
STATIC_OWNERS = (('core/reporting/result_export/src/physics_source.rs:schema', 'schemas/physics_source_recovery.schema.json'), ('core/reporting/result_export/src/retained_precision.rs:schema', 'schemas/retained_precision_mp_v2.schema.json'), ('core/reporting/result_export/src/retained_precision.rs:definition', 'fixtures/results/retained_precision_prepared_ordinary_v1.json'), ('core/reporting/result_export/src/retained_precision.rs:table', 'fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:preview_physics_contract', 'fixtures/results/semantic_contract_v0_3_preview_physics_1.json'), ('core/reporting/result_export/src/retained_precision.rs:combination_definition', 'fixtures/results/retained_precision_prepared_combination_v1.json'), ('core/reporting/result_export/src/retained_precision.rs:exact_definition', 'fixtures/results/retained_precision_prepared_exact_v1.json'), ('core/reporting/result_export/src/semantic_contract.rs:physics_retained_contract', 'fixtures/results/semantic_contract_v0_3_physics_retained_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:preview_physics_retained_contract', 'fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json'), ('core/reporting/result_export/src/retained_precision.rs:exact_table', 'fixtures/results/semantic_contract_v0_3_physics_retained_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:pressure_source_contract', 'fixtures/results/semantic_contract_v0_3_pressure_source_1.json'), ('core/reporting/result_export/src/semantic_contract.rs:pressure_rows_contract', 'fixtures/results/semantic_contract_v0_3_pressure_1.json'), ('core/reporting/result_export/src/load_reference.rs:transport_schema', 'schemas/load_reference_state.schema.json'))


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

        # B7b: snapshot keys follow B7b's unchanged lines (B7a 10627.. +5).
        # T4-U6/U8 (PP head 20c6126a16): every key below moved with unchanged text (B7b: lib.rs
        # add_uniform_element_loads 10632.., 1054, 3064, 5299, 5970; certified_loads.rs 233/256; retained_product.rs
        # failed_manifest 4298..; retained_precision.rs 5555/5637/5661). Epoch addbdc030e: the lib.rs keys +36
        # (the realized-arc marker's three helpers after `straight_section_resultants`); final source 588de796a4:
        # every lib.rs key after withhold_certified_summaries +5 (PP4's doc lines and reduced-headline clear).
        (pp + "lib.rs:10998", {"load_case_id": inp}),
        (pp + "lib.rs:11012", {"stable_suffix(load_case_id)": inp, "stable_suffix(&load.load_id)": inp}),
        (pp + "lib.rs:11019", {"load.load_id": inp, "bend.pipe_id": inp}),
        (pp + "lib.rs:11024", {"load.load_id": inp}),
        (pp + "lib.rs:11025", {"bend.pipe_id": inp}),
        (pp + "lib.rs:11026", {"bend.component_id": inp}),
        (pp + "lib.rs:11027", {"load_case_id": inp}),
        (pp + "lib.rs:11086", {"stable_suffix(load_case_id)": inp, "stable_suffix(&load.load_id)": inp}),
        (pp + "lib.rs:11093", {"load.load_id": inp, "bend.pipe_id": inp}),
        (pp + "lib.rs:11098", {"load.load_id": inp}),
        (pp + "lib.rs:11099", {"bend.pipe_id": inp}),
        (pp + "lib.rs:11100", {"bend.component_id": inp}),
        (pp + "lib.rs:11101", {"load_case_id": inp}),
        (pp + "lib.rs:11140", {"stable_suffix(&load.load_id)": inp}),
        (pp + "lib.rs:11147", {"load.load_id": inp, "load_case_id": inp}),
        (pp + "certified_loads.rs:236", {"case_id": inp}),
        (pp + "certified_loads.rs:259", {"case_id": inp}),
        (pp + 'retained_product.rs:4509', {"stable_suffix(id)": inp}),
        (pp + 'retained_product.rs:4510', {"id": inp}),
        (pp + 'retained_product.rs:4518', {"id": inp, "id.len()": inp, "component": static}),
        (pp + 'retained_product.rs:4520', {"id": inp}),
        (pp + 'retained_product.rs:4521', {"component": static}),
        (pp + 'retained_product.rs:4538', {"member.id": inp, "member.id.len()": inp}),
        (pp + 'retained_product.rs:4543', {"member.id": inp}),
        (pp + 'retained_product.rs:4569', {"ordinary.document_kind": static}),
        (pp + 'retained_product.rs:4570', {"ordinary.model_ref": inp}),
        (pp + "lib.rs:1058", {"semantic_contract_id": static}),
        (pp + "lib.rs:3102", {"context.semantic_id()": static}),
        (pp + "lib.rs:5385", {"support.id": inp}),
        (pp + "lib.rs:6129", {"pipe.element_id": inp}),
        (pp + "model_format.rs:26", {"version": static}),
        # ccb19ba0bf: +17 lines (section_allowances), same text.
        # W1 G8 scope: +6 lines before these unchanged requested-mode bodies.
        (re_ + 'retained_precision.rs:5848', {"mode": static}),
        (re_ + 'retained_precision.rs:5930', {"text(cid)": inp}),
        (re_ + 'retained_precision.rs:5954', {"text(cid)": inp}),
    ]
    lex = [('retained_product.rs:4567', {"class": 64, "why": "source-bound mechanics schema literal"}),
           ('retained_product.rs:4569', {"class": 2914, "why": "positive formulation content upper; actual struct/Vec storage is separate"}),
           ('retained_product.rs:4573', {"class": 2914, "why": "positive summary-owned text upper; inline header and LocatedQuantity children stay separate"})]
    member = runpy.run_path(os.path.join(os.path.dirname(__file__), "w1_member_load_model.py"))
    audits.append(member["case_clone_audit"](snapshot))
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
    # W1 extracts the submitted-source loop into solve_sources_projected.
    # Keep the complete historical audit preimage, then re-install it at the
    # actual clone statement after line carry. The existing sources.iter()
    # rule still bounds the moved loop by att; no owner or visit is deleted.
    member = runpy.run_path(os.path.join(os.path.dirname(__file__), "w1_member_load_model.py"))
    member["inspect"](snapshot)
    clone_key = "core/solver/frame_kernel/src/structural/retained/adaptive.rs:5082"
    clone_audit = {"source": {"bytes": 128, "source": "STATIC"}}
    if text["id_audit"].get(clone_key) != clone_audit:
        raise ValueError("W1 stage2: historical source-clone audit preimage changed")
    text["id_audit"].pop(clone_key)
    alterations.append({"family": "id_audit", "key": clone_key,
                        "after": member["case_clone_audit"](snapshot)[0],
                        "why": "same source clone in the extracted batch body; full audit retained"})
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
    # B7b: eleven distinct inputs, thirteen actual OnceLock allocations
    # (twelve OnceLock<Value> and transport_schema's OnceLock<Result<Value, String>>).
    # Never deduplicate by path or skip an absent file: g4's unchanged
    # static_tree parser must account every listed allocation positively.
    return [path for _, path in STATIC_OWNERS]
