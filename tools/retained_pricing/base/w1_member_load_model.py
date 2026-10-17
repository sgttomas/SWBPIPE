"""W1 stage 2: source-bound positive owners, not the design surrogates.

The complete source cohort binds member-only custody, the once-formed exact
ledger, recovery/conditioning, both lanes, loaded maxima and the independent
reader. Source-only layout observations live in the native profile; constants
below are source capacity bounds, never measured RSS or allocator usable bytes.
"""
import hashlib
from pathlib import Path
import re
import runpy

# Full raw pins bind the implementation and complete helper call chains.
SOURCE_FILES = {
    "core/product_physics/src/retained_product.rs": '69a87004ff692dd577c2b4bd0f7a17e621cd5d4ea07b29d55ddc7618200ef025',
    "core/product_physics/src/retained_wire.rs": "9d4fbf77f8c800dc0fc0800d3be417be49f1d95c678945b4b0a4557c6515bf13",
    "core/product_physics/src/retained_receipt.rs": "bf10f382e247fe84164dac5916e0b6fc69134e4b4934fd832132f59e86ca7251",
    "core/reporting/result_export/src/lib.rs": "6e3b91d7171483db3d9f88d736405afb581394de7e1d20a21ce4c357451f74da",
    "core/reporting/result_export/src/retained_precision.rs": "e2c6de999d7de423fcf5331cc83948ec79b25955c86d0c6e4823b6667c9943aa",
    "core/reporting/result_export/src/declared_member_load.rs": '93787b8894f752cd4c577c58513449471672956e934d35157e31178e9c53a52c',
    "core/solver/frame_kernel/src/structural/retained_resource.rs": "817de6a975e7a95a9bfe8bbe660cc40807f66e31d923db860615daaf61223909",
    "core/solver/frame_kernel/src/structural/retained/source.rs": "2efa421b47fa147f3233dcdc236f414f0f80a9019b19b4d181782a90d4eb7d98",
    "core/solver/frame_kernel/src/structural/retained/declared.rs": "867637e55a30fd34b679ad11423ad1d6d58e14f632548b04c4a535c94aa62c77",
    "core/solver/frame_kernel/src/structural/retained/ledger.rs": "0d80f13a76771506c1c81f65e502ce255d648f8aaa6a2eecfd3544bab8af47e9",
    "core/solver/frame_kernel/src/structural/retained/origins.rs": "e7fed3bb7bb207aa20231dd2fd4bf7b6debe668b228bb6f2b2ab5cfaad53f6c4",
    "core/solver/frame_kernel/src/structural/retained/adaptive.rs": "d68397d947d9298404d057ddfc772a4ba721757eeddc6e0ce81c7d36834a4179",
    "core/solver/frame_kernel/src/structural/retained/assemble.rs": "360734773e605301bcc3d05356391a910df0e8c909ed912a0c0cdde3f9ec339b",
    "core/solver/frame_kernel/src/structural/retained/combine.rs": "8f2c429096e8a0dd7bbc6788655ab74302f0e1d434839f49e6c7c3a62d11e048",
    "core/solver/frame_kernel/src/structural/retained/recover.rs": "89c53763e499fbadd3ecb273875716f5072a57e545487a2c3de54afbdc8e4b27",
    "core/solver/frame_kernel/src/structural/retained/verify.rs": "b0bc5322aa69da3c44992f08abaac95fcd9c7d650b318672aa741dde8f867b40",
    "core/solver/frame_kernel/src/structural/retained/wide_sum.rs": "15a9d997a57929bb6e8f49ed5565c266b3d7de8207cc9a111adddd3bd6d7ff9e",
    "core/solver/frame_kernel/src/structural/retained/work.rs": "8f2311cab48a6661b132272b176e777ce881b661faf975ae56a0734e4609a51f",
    "core/solver/frame_kernel/src/structural/retained/wide.rs": "b1720097c463461f656398142f1379417f850e058e1f5458821727e8076146a1",
    "core/solver/frame_kernel/src/structural/retained/wide/multi.rs": "d5aef1987ae7f390e085f98a91203cd22b58eaade07f1610e97db8fbd38d8227",
    "core/solver/frame_kernel/src/structural/retained/directed.rs": "031457491dcb9c2ae939de8ed617e7b59d5006debd5ad485a0fc9f8fd49c6b01",
    "core/solver/frame_kernel/src/structural/retained/directed/certificate.rs": "dadbb4d6a37e8a44b95d567494828feb795d805434814b7039c8b925a17e1a80",
    "core/solver/frame_kernel/src/structural/retained/product_certificate.rs": "0f6853a3eda3e92e7dda81bfc11653653c71bd2797026f067be30eff8c2fd363",
    "core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs": "cad7a7f54b234d1cb80dc8ba85dec2312732482415e23c4086d12c5fd437f2c7",
    "core/solver/frame_kernel/src/structural/retained/product_certificate/loaded_maximum.rs": "cc9ad2a9051a354273bd66370fbc57f1eb52e98aacb71b4e3cc75cd72de54900",
    "core/solver/frame_kernel/src/structural/retained/product_certificate/source_residual.rs": "7cd4da7c473cf08dc1462c3ae2b900a1646be57ec76519995dbd33d8126fdcf6"
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
    """
    backing = 256 * 2744
    endpoint_slots = 1024
    sum_buffers = 16 * 8192
    double_buffers = 32 * (2 * 16 * 8)
    auxiliary = 32 * 1024
    upper = backing + endpoint_slots * 144 + sum_buffers + double_buffers + auxiliary
    return {"backing": backing, "endpoint_slots": endpoint_slots,
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
    text = "let prepared = if composite.member_loads().is_empty() { CasePrep::new(source.clone()) }"
    hits = [i + 1 for i, line in enumerate(lines) if line.strip() == text]
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
    def site(file, text):
        lines = (Path(snapshot) / file).read_text().splitlines()
        hits = [i + 1 for i, line in enumerate(lines) if text in line]
        if len(hits) != 1:
            raise ValueError("W1 stage2: new argument site changed: " + file)
        return resolved(file + ":" + str(hits[0]))
    audits = [
        (site("core/product_physics/src/retained_product.rs", "else {k::PreparedCaseSource::new(source.clone())"),
         {"source": {"bytes": 128, "source": "STATIC"}}),
        (site("core/solver/frame_kernel/src/structural/retained/adaptive.rs", "Self::new(source.source().clone())"),
         {"source.source()": {"bytes": 128, "source": "STATIC"}}),
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
