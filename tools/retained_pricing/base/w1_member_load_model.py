"""W1 stage 2: source-bound positive owners, not the design surrogates.

The complete source cohort binds member-only custody, the once-formed exact
ledger, recovery/conditioning, both lanes, loaded maxima and the independent
reader. Source-only layout observations live in the native profile; constants
below are source capacity bounds, never measured RSS or allocator usable bytes.
"""
# RT-A3 reducer 2a D3 (§8.38): admission and normalized geometry guards change only in-region reducers,
# excluded by D1.4/D1.5. Retained/source mapped-component gates and existing owner lifetimes are unchanged.
# Resource-only reader digest updates bind the described current tables; their actual bytes are priced separately.
import hashlib
from pathlib import Path
import re
import runpy

# Full raw pins bind the implementation and complete helper call chains.
# T4-S5 D3 (retained-pricing-rules.md §8.8): the epoch's member-path files are re-bound after their diffs were read.
# The member-load custody, ledger, recovery, maxima and reader call chains are unchanged; the additions are the
# not-covered passthrough (retained_product.rs capture, failed manifest and prepared_not_covered; final_case.rs
# ProductRecipe::NotCovered and complete_not_covered; retained_precision.rs derived_kind and route L's G7
# u4b_evidence::preview_model_checks; result_export lib.rs's u4b_evidence module), priced by U4B-W1 and U4B-RD,
# and the -2 identities and table hashes (retained_wire.rs, retained_precision.rs).
# T4-S6 D3 (§8.8, S6-HS): retained_wire.rs's domain_hash streams its digest and retained_precision.rs's G1 hashes the
# publication without cloning the source; neither touches a member-path owner. T4-S7: retained_wire.rs's
# domain_hash doc comment only.
# Stack-headroom repair D3 (b7-plan 5a, retained-pricing-rules.md §8.10): read 27e4dfa972;
# freeze_case's closure is split into six phase functions, with unchanged calls, order, errors,
# drops and allocation. Only stack placement changes; no owner this model prices changes.
# W1 coverage repair D3 (retained-pricing-rules.md §8.11): read 3bbfdd5080. loaded_maximum.rs: search takes
# `driven` and, when set, runs on Enclosure::point(z.lo) with one more point_max per evaluated point for lower_hull,
# inside the unchanged loop and limits (frame growth: loaded_inventory's driven_slots, W1R-LM); recipe_reduced
# passes driven = corner1() && .., false without the census-probe feature. final_case.rs: check_intervals returns
# the scales it used and certify_final moves them into CertifiedProductProof (W1R-FS). retained_wire.rs: selection
# borrows the proof and writes body_scales from them (W1R-L1). origins.rs, source_residual.rs and the rest of
# final_case.rs/loaded_maximum.rs add only census-probe items, compiled out of the registered build (W1R-FG).
# W1 coverage repair corner 1 D3 (§8.12): read 32e0c3faf0. final_case.rs: corner1() is true without census-probe,
# so the driven search above is live in the registered build (priced since §8.11: W1R-LM, the same loop and limits).
# loaded_maximum.rs: the capacity hook module and limits()'s hook read are compiled under any(test,
# loaded-maximum-hooks); final_case.rs, origins.rs and structural.rs re-export it under that off feature, and add a
# census-probe lane hook and work snapshots (W1R-FG). No owner this model prices changes.
# W-16 slice a D3 (§8.23): retained_product.rs's captured-target match gains the support arm (`=>false`: D1.7 refuses
# it before any capture); retained_precision.rs's TABLE_HASH literal follows the preview tables. No owner this model
# prices changes.
# Unit factor-sum gate D3 (§8.24): re-read on the merge with main's W-16 slice a (§8.23). RE retained_precision.rs's
# preview-physics-retained-1 table digest (a constant) moves with the table's restated gate condition. No owner this
# model prices changes.
# Model document 0.8.0 D3 (§8.26): retained_product.rs changes one comment; retained_wire.rs and RE retained_precision.rs
# carry DEF-E's definition hash and the physics-retained-1 table hash, and RE retained_precision.rs's current-version
# gates read 0.8.0 (same length). No owner this model prices changes.
# W-16 slices b and c D3 (§8.27): RE lib.rs declares `mod exact_combination_evidence;` and retained_precision.rs's
# table hash follows the pressure-2 table. No owner this model prices changes.
# Slice 1a D3 (§8.31): reviewed nonlinear operand producer/reader additions; D1.6 empties the producer
# records, OP1A-RD retains the reader scans and OP1A-OWN prices its borrowed gate vector.
# Model document 0.9.0 D3 (§8.36): retained_wire.rs moves DEF-E's hash; RE retained_precision.rs's current-version gates
# read 0.9.0 (same length), its table hash follows physics-retained-1, and G8 adds four allocation-free need sites
# (condition_use, condition sets, spectra, set references) and a test module. No owner this model prices changes.
SOURCE_FILES = {
    "core/product_physics/src/retained_product.rs": 'cf3a1240ee380d8bb4e6999c101e224fe740ea7eb50f24a54f531ab987b9908c',
    "core/product_physics/src/retained_wire.rs": "1e1e60a02279771574f84df2b45628622936774a879a092842faa1e2f5f0f937",
    "core/product_physics/src/retained_receipt.rs": "bf10f382e247fe84164dac5916e0b6fc69134e4b4934fd832132f59e86ca7251",
    "core/reporting/result_export/src/lib.rs": "76c2e7c71c30f2d35c8ca11cb3a8044df675c199a112a9e24f41f28c1e89db55",
    "core/reporting/result_export/src/retained_precision.rs": "1bd0c1b935d3906f67092726e3453ed4c1489a79f447f46cd1bfb1745b7493f5",
    "core/reporting/result_export/src/declared_member_load.rs": '93787b8894f752cd4c577c58513449471672956e934d35157e31178e9c53a52c',
    "core/solver/frame_kernel/src/structural/retained_resource.rs": "ca0516b5b88ed0d55abf9780c0782f42acd68f4f80b73543415c920fd3da5b7b",
    "core/solver/frame_kernel/src/structural/retained/source.rs": "053b59579d6d5a2c18026029d6a8c8648617a341a2787f6f248ce0d5c4379053",
    "core/solver/frame_kernel/src/structural/retained/declared.rs": "867637e55a30fd34b679ad11423ad1d6d58e14f632548b04c4a535c94aa62c77",
    "core/solver/frame_kernel/src/structural/retained/ledger.rs": "0d80f13a76771506c1c81f65e502ce255d648f8aaa6a2eecfd3544bab8af47e9",
    "core/solver/frame_kernel/src/structural/retained/origins.rs": "0d37f9de9291c21180b279ec860cd4762ed172b71e1398642e91ad96365ee562",
    "core/solver/frame_kernel/src/structural/retained/adaptive.rs": "b966f7d77b851da81a630759f04d68f29b92b760972b9d31ebb693475a7e4249",
    "core/solver/frame_kernel/src/structural/retained/assemble.rs": "8b4ab81afc98389e6c46a979ef1ac05acb06bcd3e5ef0aa63fc847f5a897f2c5",
    "core/solver/frame_kernel/src/structural/retained/combine.rs": "85d7908cce9c4989b439db508df25601bf3905c7898245201afb27b4f085d42a",
    "core/solver/frame_kernel/src/structural/retained/recover.rs": "2c4bf28ccafd98204b1c7b0dcf91339254ed13504fe2508e40ab71a65eb30c97",
    "core/solver/frame_kernel/src/structural/retained/verify.rs": "b3de08f43e2578b5083af19a70a0490cb9f155669ad471defb67b55d65357259",
    "core/solver/frame_kernel/src/structural/retained/wide_sum.rs": "15a9d997a57929bb6e8f49ed5565c266b3d7de8207cc9a111adddd3bd6d7ff9e",
    "core/solver/frame_kernel/src/structural/retained/work.rs": "8f2311cab48a6661b132272b176e777ce881b661faf975ae56a0734e4609a51f",
    "core/solver/frame_kernel/src/structural/retained/wide.rs": "b1720097c463461f656398142f1379417f850e058e1f5458821727e8076146a1",
    "core/solver/frame_kernel/src/structural/retained/wide/multi.rs": "d5aef1987ae7f390e085f98a91203cd22b58eaade07f1610e97db8fbd38d8227",
    "core/solver/frame_kernel/src/structural/retained/directed.rs": "031457491dcb9c2ae939de8ed617e7b59d5006debd5ad485a0fc9f8fd49c6b01",
    "core/solver/frame_kernel/src/structural/retained/directed/certificate.rs": "dadbb4d6a37e8a44b95d567494828feb795d805434814b7039c8b925a17e1a80",
    "core/solver/frame_kernel/src/structural/retained/product_certificate.rs": "2481f6038229ca0dfc7e43d4a5f23918d45f373b7afb4480b7d750a9c0cf5121",
    "core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs": "1ff20d1d2806ee1b50abf1950ffc8c33b794f5e5312b1bf243d1e6d39e0b6466",
    "core/solver/frame_kernel/src/structural/retained/product_certificate/loaded_maximum.rs": "288ddb346d23e9737013060eb985b678475cc9b2b7b4251590bea9a972db8917",
    "core/solver/frame_kernel/src/structural/retained/product_certificate/source_residual.rs": "e2ef2ea11b1312d843147b042f4f41b7a7310ce52216ef48abb018ab64f954fd"
}
READER_CASE_CAP = 1024 * 1024
READER_STACK = 128 * 1024
LOADED_SEARCH_CAP = 1024 * 1024


def reader_inventory():
    """Closed new-domain call-chain upper; old G8 owners stay independent.

    Native sizes dominate the measured wasm pairs. Stored declared dyadics
    have <=1024 bits after RN/normalization; 32 limbs covers their exact
    allocation/rounding-carry staging. Every other integer is capped at 128
    limbs. Sixty-four complete such temporaries is the positive union of the
    D1-D5/net and Nat helper arguments, results and old/new moving owners.
    The encoding and recomputation are separate statements/calls, not live
    together; neither result escapes its scope except the digest/flags.
    """
    declared = 128 * (208 + 6 * 32 * 8)
    # Union, not measured simultaneous liveness: declared()'s x/y/w (9),
    # s/length/g (3), current six entries (6); Dyad add/mul/rn/sqrt/div12
    # operands/results (16); Nat shift/mul/div/isqrt arguments/results,
    # quotient/remainder/midpoint/carry old/new buffers (24); net and its
    # current contribution plus hash/move staging (6). These 64 full-size
    # integers are additional to every stored declaration, so a previous
    # load/current entries overlap is charged twice rather than omitted.
    arithmetic_slots = 9 + 3 + 6 + 16 + 24 + 6
    arithmetic = arithmetic_slots * 128 * 8
    expected = 128 * 40
    auxiliary = 32 * 1024  # SHA, strings/errors, borrowed guard/index/header staging
    recompute = declared + arithmetic + expected + auxiliary + READER_STACK
    # Actual G8 call-chain bound, not the looser standalone native_bytes
    # helper guard. guard_member_g8 bounds invocation IDs to128 and authored
    # terms jointly to128; nodal/member occurrence binding finishes before
    # verify_native_source_hashes. Other encoded dimensions retain their
    # conservative helper supersets, including192 references per support.
    encoded = (6 + 4 + 32 * 24 + 4 + 32 * 84 + 4 + 192 * 17 + 4
               + 4 + 192 * 13 + 4 + 4 + 96 * 16
               + 4 + 32 * (4 + 4 + 6 + 4 + 192 * 4 + 4)
               + 4 + 128 * max(145, 160))
    # No power-of-two capacity assumption: reserve grows to max(2*old,need).
    # Every request need<=encoded, so final capacity<2*encoded and its
    # preceding backing<encoded. Header6 and variable extends are included.
    encoding_moving = 3 * encoded
    encoding = encoding_moving + auxiliary + READER_STACK
    return {"declared": declared, "arithmetic_slots": arithmetic_slots,
            "arithmetic": arithmetic, "expected": expected,
            "auxiliary": auxiliary, "encoded_bytes": encoded,
            "encoding_with_moving_and_stack": encoding,
            "recompute_with_stack": recompute, "upper": max(encoding, recompute),
            "cap": READER_CASE_CAP}


def loaded_inventory():
    """Positive source union for the one live loaded-maximum search.

    No new numeric helper allocates. Charge every endpoint local/argument
    and array in all the new helpers plus their existing corner/norm/scalar
    callees together, rounded to 1024 endpoint slots. Sixteen full 8192-byte
    sum/scratch records, 32 double-width limb arrays and 32KiB of remaining
    headers/indices/work records cover nested numeric staging and moves.
    Returned hulls, boxed refusal diagnostic, PP spans/evidence and caller
    NumericWork records are independent positive rows below.

    W1R-LM (W1 coverage repair §1.2, retained-pricing-rules.md §8.11): the
    driven search's frame keeps the represented hull (hull_z, two endpoints)
    beside its z_lo point, the hull-valid lower (lower_hull) and the named
    point value: four more endpoint slots, added to the rounded union rather
    than assumed inside it. Its second point_max call per evaluated point runs
    after the first returns (sequential callee frames, no new allocation).
    """
    backing = 256 * 2744
    endpoint_slots = 1024
    driven_slots = 4
    sum_buffers = 16 * 8192
    double_buffers = 32 * (2 * 16 * 8)
    auxiliary = 32 * 1024
    upper = backing + (endpoint_slots + driven_slots) * 144 + sum_buffers + double_buffers + auxiliary
    return {"backing": backing, "endpoint_slots": endpoint_slots, "driven_slots": driven_slots,
            "sum_buffers": sum_buffers, "double_buffers": double_buffers,
            "auxiliary": auxiliary, "upper": upper, "cap": LOADED_SEARCH_CAP}


def inspect(snapshot):
    if not SOURCE_FILES:
        raise ValueError("W1 stage2 source pins are not yet established")
    root = Path(snapshot)
    for relative, expected in SOURCE_FILES.items():
        if hashlib.sha256((root / relative).read_bytes()).hexdigest() != expected:
            raise ValueError("W1 stage2 source/owner pin changed: " + relative)
    product = (root / "core/product_physics/src/retained_product.rs").read_text()
    if "MEMBER_LOAD_CAUSE" in product or "fn capture_member_load_case(" in product:
        raise ValueError("W1 stage2: retired preparation refusal returned")
    if "member_sources: Option<Vec<MemberCaseSource>>" not in product:
        raise ValueError("W1 stage2: member-only invocation custody changed")
    if reader_inventory()["upper"] > READER_CASE_CAP:
        raise ValueError("W1 stage2: complete reader call-chain exceeds its priced cap")
    if loaded_inventory()["upper"] > LOADED_SEARCH_CAP:
        raise ValueError("W1 stage2: complete loaded search exceeds its priced cap")
    identifiers = member_identifier_warrant(snapshot)
    return {"stage": 2, "success_findings": "lc", "source_sha256": dict(SOURCE_FILES),
            "reader_case_cap": READER_CASE_CAP, "reader_stack": READER_STACK,
            "loaded_search_cap": LOADED_SEARCH_CAP,
            "member_identifier_warrant": identifiers}


def member_identifier_warrant(snapshot):
    """The member-only copy class and real parser specialization, fail closed.

    The complete raw source pins bind callers and the cfg(test) shadow;
    domain.inspect independently binds D1's text cap, typed/raw census and
    admission enforcement. Neither generic copy nor nodal parsing changes.
    """
    base = Path(__file__).parent
    domain = runpy.run_path(str(base / "b7a_domain_model.py"))
    proof = domain["inspect"](snapshot)
    helpers = runpy.run_path(str(base / "kd5_text_model.py"))
    file = "core/product_physics/src/retained_product.rs"
    source = (Path(snapshot) / file).read_text()
    body = helpers["_indented_function"]
    old, _ = body(source, "copy")
    member, header = body(source, "copy_member_id")
    if old.replace("fn copy(", "fn copy_member_id(", 1) != member:
        raise ValueError("W1-MID: member copy allocation/events/errors differ from generic copy")
    capture, _ = body(source, "capture_case_nodal_terms")
    local, _ = body(source, "prepared_member_local_loads")
    shadow, _ = body(source, "test_member_id_copy_equivalence")
    if (capture.count(".copy_member_id(&load.load_id)") != 2
            or local.count(".copy_member_id(&load.source_id)") != 1
            or shadow.count(".copy_member_id(&value)") != 1
            or source.count(".copy_member_id(") != 4):
        raise ValueError("W1-MID: member-only caller/test-shadow inventory changed")
    marker = "for (occurrence, load) in application.element_uniform_loads.iter().enumerate()"
    if capture.count(marker) != 1:
        raise ValueError("W1-MP: member occurrence loop changed")
    member_loop = capture[capture.index(marker):]
    predicate = ('matches!((p.direction.as_str(),axis),("global_x",0)|'
                 '("global_y",1)|("global_z",2))')
    nodal_parser = "parse_direction(&p.direction).is_ok_and(|d|d.dof_index()==l.global_dof%6)"
    if (member_loop.count(predicate) != 1 or "parse_direction(" in member_loop
            or capture.count(nodal_parser) != 1 or capture.count("parse_direction(") != 1):
        raise ValueError("W1-MP: member predicate or preserved nodal parser changed")
    hits = [header + i for i, line in enumerate(member.splitlines()) if "out.push_str(s);" in line]
    if len(hits) != 1:
        raise ValueError("W1-MID: member copy lexical site changed")
    return {"class": 128, "site": file + ":" + str(hits[0]),
            "productive_calls": 3, "test_calls": 1,
            "member_copy_sha256": hashlib.sha256(member.encode()).hexdigest(),
            "domain_caps_sha256": proof["admission_caps_sha256"],
            "domain_enforcement_sha256": proof["admission_prefix_sha256"]}


def identifier_lex_sizes(snapshot, resolved):
    inspect(snapshot)
    warrant = member_identifier_warrant(snapshot)
    key = warrant["site"].split("/src/")[-1]
    return [(resolved(key), {"class": warrant["class"],
            "why": "W1-MID: only the three source-pinned member identifier copies; D1 text cap128, full calls/capacity2/events/errors retained"})]


def selected_message_bytes(snapshot):
    text = (Path(snapshot) / "core/product_physics/src/retained_wire.rs").read_text()
    matches = re.findall(r'const SELECTED_MEMBER_LOAD_MESSAGE: &str = "([^"\\]*)";', text)
    if len(matches) != 1:
        raise ValueError("W1 stage2: selected member disclosure changed")
    return len(matches[0].encode())


def case_clone_audit(snapshot):
    """Carry the original source clone audit to the extracted batch body."""
    inspect(snapshot)
    file = "core/solver/frame_kernel/src/structural/retained/adaptive.rs"
    lines = (Path(snapshot) / file).read_text().splitlines()
    # O4 S5 (retained-pricing-rules.md §8.9): the Timoshenko arm `if shear.is_some() { CasePrep::new_with_shear(..) }`
    # precedes the nodal arm, so the same clone statement is the `else if` line; the audit follows it unchanged.
    text = ("let prepared = if composite.member_loads().is_empty() { CasePrep::new(source.clone()) }",
            "else if composite.member_loads().is_empty() { CasePrep::new(source.clone()) }")
    hits = [i + 1 for i, line in enumerate(lines) if line.strip() in text]
    if len(hits) != 1:
        raise ValueError("W1 stage2: extracted source clone site changed")
    return file + ":" + str(hits[0]), {"source": {"bytes": 128, "source": "STATIC"}}


def text_decisions(snapshot, resolved):
    """Exact new headers with positive source bounds and unchanged ID laws."""
    inspect(snapshot)
    why = "W1 member path: exact source-pinned header; D1 and fixed array bounds"
    loops = [
        ('for (ordinal,l) in authored.iter().enumerate().filter(|(_,l)| l[" "][" "]==" ")', "l", ["retained_precision.rs:verify_member_terms"]),
        ("for (canonical,load) in owner.member_loads().iter().enumerate().filter(|(_,load)|load.member==member)", "l", ["retained_product.rs:prepared_member_local_loads"]),
        ("for (occurrence, load) in application.element_uniform_loads.iter().enumerate()", "l", ["retained_product.rs:capture_case_nodal_terms"]),
        ("for load in source.loads.loads()", "l", ["retained_product.rs:prepare_active_case"]),
        ("for term in &member.terms", "l", ["retained_product.rs:check_early_occurrences"]),
        ('for value in list(&term[" "])', "3", ["retained_precision.rs:native_bytes"]),
        ('for (to,from) in limbs.iter_mut().zip(list(&v[" "]))', "16", ["retained_precision.rs:diagnostic_endpoint"]),
        ("for (canonical,load) in member.loads.loads().iter().enumerate()", "l", ["retained_wire.rs:case_source_at"]),
        ('iter:" ":load.intensity.iter().map', "3", ["retained_wire.rs:case_source_at"]),
        ('iter:" ":limbs.iter().map', "16", ["retained_wire.rs:proof_trace"]),
        ("for (canonical,load) in loads.loads().iter().enumerate()", "l", ["retained_product.rs:store_member_source"]),
    ]
    def site(file, text, count=1):
        lines = (Path(snapshot) / file).read_text().splitlines()
        hits = [i + 1 for i, line in enumerate(lines) if text in line]
        if len(hits) != count:
            raise ValueError("W1 stage2: new argument site changed: " + file)
        return [resolved(file + ":" + str(h)) for h in hits] if count != 1 else resolved(file + ":" + str(hits[0]))
    # O4 S5 (retained-pricing-rules.md §8.9): CasePrep::new_with_shear's nodal arm makes the same PrimitiveSource
    # clone as new_member (`Self::new(source.source().clone())`), so the one audit applies at both statements.
    _adaptive = "core/solver/frame_kernel/src/structural/retained/adaptive.rs"
    _clones = 2 if "pub(crate) fn new_with_shear(" in (Path(snapshot) / _adaptive).read_text() else 1
    _clone_sites = site(_adaptive, "Self::new(source.source().clone())", _clones)
    audits = [
        (site("core/product_physics/src/retained_product.rs", "else {k::PreparedCaseSource::new(source.clone())"),
         {"source": {"bytes": 128, "source": "STATIC"}}),
        *[(_s, {"source.source()": {"bytes": 128, "source": "STATIC"}})
          for _s in (_clone_sites if isinstance(_clone_sites, list) else [_clone_sites])],
        (site("core/reporting/result_export/src/retained_precision.rs", 'let generated=format!("load:self-weight:'),
         {'text(&case["id"])': {"bytes": 128, "source": "IN128"},
          'text(&pipes[member]["id"])': {"bytes": 128, "source": "IN128"},
          'text(&case["id"]).len()': {"bytes": 20, "source": "NUM"},
          'text(&pipes[member]["id"]).len()': {"bytes": 20, "source": "NUM"}}),
    ]
    identifier = member_identifier_warrant(snapshot)
    audits.append((resolved(identifier["site"]), {"s": {"bytes": 128, "source": "IN128"}}))
    return [(h, b, why, owners) for h, b, owners in loops], audits


def producer_additions(n, m, loads, cases, ident):
    """Per-case additions, except the invocation-owned T11.12 row.

    Existing nodal/adapter reservations stay positive. The old nodal ledger
    remains resident beside a complete formed ledger, rather than taking a
    delta-of-row-count credit. The caller applies its unchanged C/|A| laws.
    """
    from g3lib import B, s, Xc, V
    dofs = 6 * n
    member = Xc("UniformMemberLoad", loads) + B(loads * ident)
    terms = Xc("MemberTermIdentity", loads) + B(loads * ident)
    # The capture owns one vector; each case owns terms and a MemberLoadSet.
    capture = s("Vec") + Xc("MemberCaseSource", cases) + (member + terms + B(ident)) * cases
    # Source validation's stable sort and its occurrence-used bitmap coexist
    # with the owned sidecar. A prepared/rebuilt operand keeps its own clone.
    sort = s("UniformMemberLoad") * max(loads, 48) if loads > 20 else B(0)
    # ExactWideSum's canonical accessor returns at most 128 limbs per net.
    formed = (Xc("DeclaredMemberLoad", loads) + Xc("(usize,LedgerNet)", dofs)
              + Xc("u64", 128 * dofs) + B(dofs))
    # Recovery's Option<LocalLoad> has nine Wide fields plus its tag;
    # E's analogous vector has three Wide fields plus its tag. Exact-capacity
    # vectors are charged independently, with their two headers.
    recovery = s("Wide<16>") * (12 * m + 64) + s("usize") * (2 * m) + s("Vec") * 2
    # One new member-local input list and its returned three-axis spans, held
    # beside the existing PP maximum helper, which is not reduced.
    maximum = (B(LOADED_SEARCH_CAP) + Xc("ElementUniformLoadContribution", loads)
               + B(loads * ident) + Xc("SpannedUniformLocalLoad", 3 * loads)
               + s("MemberValuesWork") * 2 + Xc("LoadedValue", m)
               + s("LoadedMaximumDiagnostic") + Xc("ProductMaximumEvidence", m)
               + s("Vec") * 4)
    return {
        "T11.12 invocation member-only custody": capture,
        # Two old nodal encodings can coexist with the final member encoding
        # in new_member/MemberSourceRef::encoding. Each is <32KiB on D1;
        # final encodings remain charged separately through E_src.
        "T12.11 member capture/validation and prepared-sidecar staging": member + sort + B(2 * loads + 65536),
        "T12.12 prepared/rebuilt operand member clone": member,
        "T12.13 complete declared ledger beside the nodal ledger": formed,
        "T13.4 member-aware submitted views": s("MemberSourceRef") + s("Vec"),
        "T13.5 declared/recovery/E call-chain scratch": recovery + s("ExactWideSum") * 2 + V("usize", dofs) * 2,
        # (u32,[Enclosure;2]): four endpoints, independently on both lanes.
        "T14.4 member curvature vectors on both lanes": (s("Wide<16>") * (4 * m) + B(8 * m) + s("Vec")) * 2,
        "T14.5 loaded search, evidence, PP spans and values completion": maximum,
    }
