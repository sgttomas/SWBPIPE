"""PRA-CF (PR-A, retained-pricing-rules.md §8.62): the corrected displacement's failure texts, priced positively.

DEL-04-07 *Published displacement*: `solve_prepared_load_case` forms each case's corrected displacement
(`corrected_nodal_displacements`) before publication. If one component cannot be certified or represented, the
failure is kept as a `DisplacementCorrectionFailure { dof, kind, error: format!("{error:?}") }`, cloned once into
`correction_failure`, and formatted into the held reason (`displacement_correction_failed`), the integrity sentence
(`displacement_correction_sentence`) and the case's warning (`displacement_correction_warning`: its id, message and
one ref). No premise excludes this failure on D1: every one of these texts is charged at its bound in every case
solve that forms a correction. This module derives the bounds from the source and binds every text they rest on:

- CF-DBG, the kernel error's Debug text. `RecoveryFailure` is `ArcCertificateFailure` (certified_recovery.rs), a
  `#[derive(Debug)]` enum whose fields are `&'static str` sites, an f64, or `WideError` (itself `#[derive(Debug)]`
  over `&'static str`, u32, `WorkFault` and `SumError`, both fieldless `#[derive(Debug)]` enums). Each enum block is
  bound by its exact text. `certify_displacement_step(dof, u, bound)` receives no text (MemberBoundInputs holds only
  f64/i32 vectors and an f64, bound by its exact text), and the frame kernel builds no `&'static str` at run time (no
  `leak(` in its production files), so every `&'static str` in the error is a string literal or const of the frame
  kernel's production source. The bound is the longest variant's Debug text with every such field at the longest
  Debug-escaped literal of those files (`"` quotes added; `\\`, `"`, `\\n`, `\\r`, `\\t`, `\\0` take two bytes; any
  other control or non-ASCII character is charged as a ten-byte `\\u{..}` escape), an f64 at 24 bytes and a u32 at
  10. `format!("{error:?}")` allocates exactly that text (capacity at most twice its length, the chain's capacity
  rule).
- CF-FAIL, `displacement_correction_failed`: its template's literal bytes, `integrity_dof_label` (`{node.id}:{UX..RZ}`,
  at most 128 + 1 + 2 = 131 B, or `global_dof={dof}`, at most 31 B), the step (the longer of its two literals) and
  `failure.error` (CF-DBG).
- CF-SENT and CF-WARN: the sentence's and the warning message's literal bytes plus CF-FAIL (and the warning's case id,
  IN128); the warning id's literal plus IN128; its one ref is `case_id.to_string()` (IN128).
- CF-CLONE: `failure.clone()` copies the one owned String, `error` (dof is usize, kind is Copy): CF-DBG bytes.

`inspect(snapshot)` returns None when lib.rs has no `corrected_nodal_displacements` (the earlier U5B-PD shape, which
drops a failure and allocates nothing), otherwise the bounds; any other change to a bound text stops the run."""
import hashlib
import os
import re
import runpy

LIB = "core/product_physics/src/lib.rs"
FK = "core/solver/frame_kernel/src/"
CR = FK + "structural/certified_recovery.rs"
IN128 = 128     # D1.9 RawTextBytes: an authored id (load case, node)
F64_DEBUG = 24  # the longest binary64 Debug text, e.g. -2.2250738585072014e-308
U32_DEBUG = 10

# Exact texts. Each block runs from its first line to the first following line that is exactly "}".
BLOCKS = {
    (CR, "pub type RecoveryFailure = ArcCertificateFailure;"): None,
    (FK + "structural/arc_certificate.rs", "#[derive(Debug, Clone, Copy, PartialEq)]\npub enum ArcCertificateFailure {"):
        "ArcCertificateFailure",
    (FK + "structural/retained/wide.rs", "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum WideError {"): "WideError",
    (FK + "exact_sum.rs", "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum SumError {"): "SumError",
    (FK + "structural/retained/work.rs", "#[repr(u8)]\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum WorkFault {"): "WorkFault",
    (FK + "structural/formation_check.rs", "pub struct MemberBoundInputs {"): None,
}
# CF-PIN: the sha256 of each bound text, read before this model was written.
PINS = {
    "ArcCertificateFailure": "2a580c31eccaab3934f57b029dea9a9082b786e8d3f33c6adcb1e25ec6e062b6",
    "WideError": "a2ad5ed03c8baf673b8b24ec3c3563e29cea7f8e5302b69d241873b3f86cbcc5",
    "SumError": "7c9440ffd3e226a1f05a282570f342308e953a8aadf6c28694d976754794b1fe",
    "WorkFault": "36356889a1a547add0271dff83eacdac4618ce49be9b8e9a99ac813b1d37ad54",
    "MemberBoundInputs": "eba0701030b0bb77a6b88d0a6f9fe43589036d5a5bc25321cec4ce704425b787",
    "certify_displacement_step": "88eb14669f260d6e464c4e0ebb83e94f6b17c286357080558bc9c53e70768223",
    "DisplacementCorrectionFailure": "86379f28e18758d7e2840cb34c14d8db26a36b6b183f0b033fd96317be1c603c",
    "corrected_nodal_displacements": "df86a50c0b1bee3edad25f27a4d148b7bbadc609133b3f97842f3ac67b704982",
    "displacement_correction_failed": "7ae069ff4d1ac3ad7d35d266e32f7fdeba24d633992ddc5ab42d4ce42335e520",
    "displacement_correction_sentence": "b100d1d21fd1b8d06ada4c40ec72f86061fe7abaedd81b04df19339031d60480",
    "displacement_correction_warning": "7b238f0a8c2ea17dbd22475cb4018c2de2352e47cf3cdd16b93900710691d2db",
    "integrity_dof_label": "9409dff48618df8c60a4f42866384e14fbd8698d7ee03f411be0010c83a7f7ed",
}


def _read(snapshot, rel):
    path = os.path.join(snapshot, rel)
    if not os.path.exists(path):
        raise SystemExit(f"PRA-CF: {rel} is missing; re-review the failure texts' bounds (§8.62)")
    with open(path, encoding="utf-8") as stream:
        return stream.read()


def _block(text, head, label):
    if text.count(head) != 1:
        raise SystemExit(f"PRA-CF: {label} changed or is not one definition; re-review (§8.62)")
    start = text.index(head)
    if head.endswith(";"):
        return head
    end = text.find("\n}\n", start)
    if end < 0:
        raise SystemExit(f"PRA-CF: no end of {label}")
    return text[start:end + 3]


def _fn(text, name):
    hits = list(re.finditer(r"^(?:pub(?:\([a-z]+\))?\s+)?fn\s+" + re.escape(name) + r"\b", text, re.M))
    if len(hits) != 1:
        raise SystemExit(f"PRA-CF: fn {name} is not one top-level definition; re-review (§8.62)")
    end = text.find("\n}\n", hits[0].start())
    return text[hits[0].start():end + 3]


def _check(label, body):
    digest = hashlib.sha256(body.encode()).hexdigest()
    if digest != PINS[label]:
        raise SystemExit(f"PRA-CF: {label} changed; re-review the failure texts' bounds (§8.62)")
    return digest


def debug_str_bytes(value):
    """Upper bound of `format!("{:?}", value)` for a &str: quotes plus Rust's escape_debug, conservatively."""
    n = 2
    for ch in value:
        if ch in "\\\"\n\r\t\0":
            n += 2
        elif " " <= ch <= "~":
            n += 1
        else:
            n += 10
    return n


def fk_production_files(snapshot):
    root = os.path.join(snapshot, FK)
    out = []
    for d, _, fs in os.walk(root):
        for f in fs:
            rel = os.path.relpath(os.path.join(d, f), snapshot)
            if f.endswith(".rs") and not f.endswith("_tests.rs") and f != "tests.rs" and "/tests/" not in "/" + rel:
                out.append(rel)
    return sorted(out)


def longest_static_text(snapshot):
    """The longest Debug text of any string literal in the frame kernel's production files, and its file."""
    lexer = runpy.run_path(os.path.join(os.path.dirname(os.path.abspath(__file__)), "rust_literals.py"))
    best, at = 0, None
    for rel in fk_production_files(snapshot):
        text = _read(snapshot, rel)
        if re.search(r"\bleak\s*\(", re.sub(r"//[^\n]*", "", text)):
            raise SystemExit(f"PRA-CF: {rel} leaks a String to &'static str; the error's text is not a literal (§8.62)")
        for token in lexer["literals"](text):
            if token.get("kind") in ("str", "raw_str") and isinstance(token.get("value"), str):
                n = debug_str_bytes(token["value"])
                if n > best:
                    best, at = n, rel
    return best, at


def enum_debug(blocks, name, static):
    """The longest #[derive(Debug)] text of enum `name`, by its exact block's variants."""
    bound = 0
    for raw in blocks[name].split("\n")[1:]:
        line = raw.strip()
        if not line or line.startswith("//") or line.startswith("#[") or line.startswith("pub enum") or line == "}":
            continue
        m = re.fullmatch(r"(\w+)(?:\s*=\s*\d+)?\s*,", line) or re.fullmatch(r"(\w+)\s*\(([^()]*)\)\s*,", line) \
            or re.fullmatch(r"(\w+)\s*\{\s*(\w+)\s*:\s*([^{}]*?)\s*\}\s*,", line)
        if not m:
            raise SystemExit(f"PRA-CF: {name} variant `{line}` is not a reviewed form (§8.62)")
        variant = m.group(1)
        if m.re.pattern.startswith(r"(\w+)(?:"):
            size = len(variant)
        else:
            field_type = m.group(m.lastindex).strip()
            fields = {"&'static str": static, "f64": F64_DEBUG, "u32": U32_DEBUG}
            if field_type in fields:
                inner = fields[field_type]
            elif field_type in blocks:
                inner = enum_debug(blocks, field_type, static)
            elif field_type == "super::work::WorkFault":
                inner = enum_debug(blocks, "WorkFault", static)
            else:
                raise SystemExit(f"PRA-CF: {name}::{variant} holds `{field_type}`, which is not a reviewed field (§8.62)")
            if m.lastindex == 3:   # Name { field: T }
                size = len(variant) + len(" { ") + len(m.group(2)) + len(": ") + inner + len(" }")
            else:                  # Name(T)
                size = len(variant) + 2 + inner
        bound = max(bound, size)
    return bound


def literal_bytes(template):
    if "{{" in template or "}}" in template:
        raise SystemExit("PRA-CF: escaped braces in a failure template; re-review (§8.62)")
    return len(re.sub(r"\{[^{}]*\}", "", template).encode())


def _template(body, label, lexer):
    values = [t["value"] for t in lexer["literals"](body) if t.get("kind") == "str" and "{" in t["value"]]
    if len(values) != 1:
        raise SystemExit(f"PRA-CF: {label} does not hold one format template; re-review (§8.62)")
    return values[0]


def inspect(snapshot):
    lib = _read(snapshot, LIB)
    # U5B-PD-RN: without the corrected owner the source must hold U5B-PD's earlier reviewed shape (published), which
    # drops a failure (`.ok()`) and forms no failure text; any other absence stops (u4b_u5b_owners_model).
    shape = runpy.run_path(os.path.join(os.path.dirname(os.path.abspath(__file__)), "u4b_u5b_owners_model.py"))[
        "published_displacement_shape"](snapshot, lib)
    if shape == "published":
        return None
    if shape != "corrected":
        raise SystemExit("PRA-CF: neither reviewed U5B-PD shape is present; the failure texts' owner is missing (§8.62)")
    lexer = runpy.run_path(os.path.join(os.path.dirname(os.path.abspath(__file__)), "rust_literals.py"))
    blocks, digests = {}, {}
    for (rel, head), name in BLOCKS.items():
        body = _block(_read(snapshot, rel), head, name or head)
        if name:
            blocks[name] = body
            digests[name] = _check(name, body)
        elif "MemberBoundInputs" in head:
            digests["MemberBoundInputs"] = _check("MemberBoundInputs", body)
            types = re.findall(r"^\s*pub \w+: ([^\n]+),$", body, re.M)
            if sorted(set(types)) != ["Vec<f64>", "Vec<i32>", "f64"]:
                raise SystemExit(f"PRA-CF: MemberBoundInputs's fields changed ({types}); re-review (§8.62)")
    cr = _read(snapshot, CR)
    step = _fn(cr, "certify_displacement_step")
    digests["certify_displacement_step"] = _check("certify_displacement_step", step)
    # The step's own error sites, by exact text: the two shape/sum_error constructions and evaluate_row/republished.
    for text in ("    let row = evaluate_row(&[Ball::ONE], &[dof], Ball::ZERO, 0.0, u, bound).map_err(|e| (Certify, e))?;\n",
                 "        return Err((Represent, shape(\"recovery publication\")));\n"):
        if step.count(text) != 1:
            raise SystemExit("PRA-CF: certify_displacement_step's error sites changed; re-review (§8.62)")
    static, static_at = longest_static_text(snapshot)
    debug = enum_debug(blocks, "ArcCertificateFailure", static)
    failure_type = _block(lib, "#[derive(Debug, Clone)]\nstruct DisplacementCorrectionFailure {", "DisplacementCorrectionFailure")
    digests["DisplacementCorrectionFailure"] = _check("DisplacementCorrectionFailure", failure_type)
    if re.findall(r"^\s+(\w+):", failure_type, re.M) != ["dof", "kind", "error"] or failure_type.count("String") != 1:
        raise SystemExit("PRA-CF: DisplacementCorrectionFailure's fields changed; re-review (§8.62)")
    bodies = {name: _fn(lib, name) for name in ("corrected_nodal_displacements", "displacement_correction_failed",
                                                "displacement_correction_sentence", "displacement_correction_warning",
                                                "integrity_dof_label")}
    for name, body in bodies.items():
        digests[name] = _check(name, body)
    if bodies["corrected_nodal_displacements"].count(
            ".map_err(|(kind, error)| DisplacementCorrectionFailure { dof, kind, error: format!(\"{error:?}\") })") != 1:
        raise SystemExit("PRA-CF: the failure's Debug text changed; re-review (§8.62)")
    label = bodies["integrity_dof_label"]
    if (label.count('let name = ["UX", "UY", "UZ", "RX", "RY", "RZ"][dof % DOF_PER_NODE];') != 1
            or label.count('|| format!("global_dof={dof}"),') != 1 or label.count('|node| format!("{}:{name}", node.id),') != 1):
        raise SystemExit("PRA-CF: integrity_dof_label changed; re-review (§8.62)")
    dof_label = max(IN128 + 1 + 2, len("global_dof=") + 20)
    failed_body = bodies["displacement_correction_failed"]
    steps = re.findall(r'DisplacementFailureKind::\w+ => "([^"\\]*)",', failed_body)
    if len(steps) != 2 or failed_body.count("DisplacementFailureKind::") != 2:
        raise SystemExit("PRA-CF: the failure step's two literals changed; re-review (§8.62)")
    step_bytes = max(len(s.encode()) for s in steps)
    failed_template = _template(failed_body, "displacement_correction_failed", lexer)
    if failed_template != "the corrected displacement of {} could not be {step} ({})":
        raise SystemExit("PRA-CF: displacement_correction_failed's template changed; re-review (§8.62)")
    failed = literal_bytes(failed_template) + dof_label + step_bytes + debug
    sentence_template = _template(bodies["displacement_correction_sentence"], "displacement_correction_sentence", lexer)
    sentence = literal_bytes(sentence_template) + failed
    warning = bodies["displacement_correction_warning"]
    warning_templates = [t["value"] for t in lexer["literals"](warning) if t.get("kind") == "str" and "{" in t["value"]]
    if warning_templates != ["diagnostic:displacement-correction:{case_id}",
                             "Load case {case_id}: {}; no published result of this case relies on the member-relative formation check."]:
        raise SystemExit("PRA-CF: the warning's templates changed; re-review (§8.62)")
    if warning.count("vec![case_id.to_string()],") != 1 or warning.count("displacement_correction_failed(model, failure)") != 1:
        raise SystemExit("PRA-CF: the warning's ref or message argument changed; re-review (§8.62)")
    return {"debug": debug, "static_literal": static, "static_literal_at": static_at, "dof_label": dof_label,
            "step": step_bytes, "failed": failed, "sentence": sentence,
            "warning_id": literal_bytes(warning_templates[0]) + IN128,
            "warning_message": literal_bytes(warning_templates[1]) + IN128 + failed, "digests": digests}
