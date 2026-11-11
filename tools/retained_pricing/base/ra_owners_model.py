"""R-A D3 (DEL-04-08 Design source-work-nondyadic-inputs.md §3.1 and §4 "D3"): the positive owners the source route's
work reduction adds outside the chain's TEXT inventory (retained-pricing-rules.md §8.13, RA-HS, RA-SC).

B1 carries one optional source-row index per functional descriptor (`Vec<Option<u32>>`). product_physics passes only an
index rule (`Sources::hint`, arithmetic on the descriptor index, borrowed as a `&dyn Fn` closure on the stack); the
kernel's `build_hints` charges it and collects `(0..count).map(rule)` at the exact descriptor count, once into the plan
(`FunctionalPlan::plan`, held to the plan's drop) and once in `RetainedFunctionalSet::replay_against` (live through the
replay's evaluation). `evaluate` holds R-A's `Shortcuts` while it runs: three per-DOF vectors (`zero`, `window`,
`products`, n entries each), the push-grown `touched` list (at most one entry per DOF per row, drained per row) and
`source_ok` (one bool per descriptor, reserved at the descriptor count, one push per row). B1's negated Ratio is the
row's value and is priced by T07 F1's per-value expansion charge (its numerator and denominator are exact-size copies
of a source value's, each at most the 256-term expansion cap).

Every owner is bound to the sha256 of the bodies and spans that allocate, move and read it and to its exact
statements. source_recovery.rs is also under b7a_formation_model's full-file pin. A snapshot without R-A adds nothing,
so earlier registered trees keep their coefficients; a partly present or changed body stops the run."""
import hashlib
import os
import re

from g3lib import s, B, Xc, Vbytes, ASSUMED

FN = "core/solver/frame_kernel/src/structural/exact_boundary/functionals.rs"
SR = "core/product_physics/src/source_recovery.rs"

# Read at b903a2d901 (R-A's implementation head after its review moved the hint construction into the kernel); the
# shortcuts span re-read at 2dcba5ab08 (Shortcuts::start charges 3n + |prescribed|; `skipped` is test/exact-operation-count
# gated; no allocation changes).
BODY_PINS = {
    (FN, "build_hints"): "a85dcbfd19c53f75b58d87c72f722ff64c4e2bcc4344dd82a44d27222ee2ef0a",
    (FN, "new_with_hints"): "91646ae623ee538d800b09b662838bdfc7b127ad4c9a6a4173f586e1d72c1a52",
    (FN, "plan"): "ec6b157e3083c04b274e6652cf67af7b5bfcc66d6379c88a6d890bc70d1c8d1f",
    (FN, "evaluate"): "2068c5218ed428b0025bebcb30114c41058e4b81c7cc4cca557eadb1f21af351",
    (FN, "replay_against"): "672c98019b198e1184390f0dedfcc4a906acaee87359d15af5ec3fa13092ec6c",
    (SR, "hint"): "686a2e4ea524eb8cf9151125a23bb40e18ca728314f11ae37afeca0d0cf02e92",
    (SR, "prepare_sources"): "952d5dfb515fe9698e6fca1f3f9484b0ff13528192c2ea075f7ba0b20fbaa1ba",
    (SR, "solve"): "beb56d200e12efe52c78c6947be9774a4ed4d9fec3b66add1b04927c14c342f6",
    (SR, "replay_against"): "e12443fe763e6b44dfc8ce7890c611bf6093d0380bfc1ef743ed4a1ece336467",
    (SR, "check_binding_against"): "8b951ec036368e3993b238a8b8fccae8045ba70a721111150b857ad4885fc4f3",
}
# Spans: FunctionalPlan's declaration to its impl; R-A's `mod shortcuts` to its closing brace; Sources' declaration.
SPANS = {
    "plan": (FN, "pub struct FunctionalPlan<'a> {\n", "impl<'a> FunctionalPlan<'a> {\n", "cd2725b22aa2d15638e232ef9665d777e139eeaf8fbcdc5dcffc27749575e2ef"),
    "shortcuts": (FN, "\nmod shortcuts {\n", "\n}\n", "b0ccae25bfd7f708e6cfe56a26743b69e9668cee454387497b9ee156f64b77dd"),
    "sources": (SR, "\nstruct Sources {\n", "\n}\n", "93d2eb3dae9760b97bca83fdf703dad5e9ba8df4bfd6c135f45426a797bc14ee"),
}
# The exact allocating, moving and borrowing statements the formulas price, each once in its file.
STATEMENTS = (
    (FN, "    hints: Vec<Option<u32>>,\n"),
    (FN, "pub type HintRule<'h> = &'h dyn Fn(usize) -> Option<u32>;"),
    (FN, "Ok((0..count).map(rule).collect())"),
    (FN, "Some(rule) => build_hints(rule, descriptors.len(), w)?,"),
    (FN, "Some(rule) => build_hints(rule, expected_descriptors.len(), w)?,"),
    (FN, "let mut ra = shortcuts::Shortcuts::start(c, ds, hints, shortcuts, w)?;"),
    (FN, "let values = evaluate(self, &plan.descriptors, &plan.hints, true, w)?;"),
    (FN, "let values = evaluate(&response, expected_descriptors, &expected_hints, true, w)?;"),
    (FN, "zero = vec![false; n];"),
    (FN, "window = vec![0; n];"),
    (FN, "products = vec![0; n];"),
    (FN, "touched: Vec::new(),"),
    (FN, "self.touched.push(t.dof);"),
    (FN, "for i in self.touched.drain(..) {"),
    (FN, "source_ok: Vec::with_capacity(if b1 { ds.len() } else { 0 }),"),
    (FN, "self.source_ok.push(false);"),
    (FN, "self.source_ok.push(ok);"),
    (FN, "terms: source.numerator.terms.iter().map(|x| -x).collect(),"),
    (FN, "denominator: source.denominator.clone(),"),
    (SR, "fn hint(&self, index: usize) -> Option<u32> {"),
    (SR, "Some(&|index| source.hint(index)),"),
    (SR, "FunctionalPlan::new_with_hints(&context, &source.descriptors, &|index| source.hint(index), &mut budget)"),
)
# The allocating constructs in `mod shortcuts`, counted, so that an added owner cannot hide behind the span pin's
# re-review: three `vec![..; n]`, one `Vec::with_capacity`, one `.collect()` and one `.clone()` (with its `Vec::new()`
# initialisers and the push sites above).
SHORTCUTS_ALLOCATIONS = {"vec![": 3, "Vec::with_capacity(": 1, ".collect()": 1, ".clone()": 1, ".to_vec()": 0,
                         "String": 0, "format!": 0, "Box::new": 0}

ASSUMED_RA = {"s(Option<u32>)": 8}   # illustrative only: g5_profile binds it to size_of::<Option<u32>>() in the PP build


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
        raise SystemExit(f"R-A owners: {name} is not one definition; re-review the owner it prices")
    indent = hits[0].group(1)
    end = text.find("\n" + indent + "}\n", hits[0].start())
    if end < 0:
        raise SystemExit(f"R-A owners: no end of fn {name}")
    return text[hits[0].start():end + len(indent) + 3]


def _span(text, start, stop):
    if text is None or text.count(start) != 1:
        return None
    a = text.index(start)
    b = text.find(stop, a + len(start))
    if b < 0:
        return None
    return text[a:b + len(stop)]


def digests(snapshot):
    """The current digests (for review and re-pinning; not a pass condition)."""
    out = {}
    for (rel, name) in BODY_PINS:
        text = _read(snapshot, rel)
        body = _body(text, name) if text is not None else None
        out[f"{rel}:{name}"] = hashlib.sha256(body.encode()).hexdigest() if body else None
    for key, (rel, start, stop, _) in SPANS.items():
        span = _span(_read(snapshot, rel), start, stop)
        out[key] = hashlib.sha256(span.encode()).hexdigest() if span else None
    return out


def active(snapshot):
    """True when R-A's owners are present with their reviewed digests; False when neither file carries them (an
    earlier revision); anything else stops."""
    if not snapshot:
        return False
    fn = _read(snapshot, FN) or ""
    sr = _read(snapshot, SR) or ""
    marks = ("mod shortcuts {" in fn, "fn build_hints" in fn, "fn hint(&self" in sr)
    if not any(marks):
        return False
    if not all(marks):
        raise SystemExit("R-A owners: B1's hints or B2's shortcuts are partly present; re-review the owners they price")
    current = digests(snapshot)
    for (rel, name), pin in BODY_PINS.items():
        if current[f"{rel}:{name}"] != pin:
            raise SystemExit(f"R-A owners: {rel} fn {name} changed or is missing; re-review the owner it prices")
    for key, (rel, _, _, pin) in SPANS.items():
        if current[key] != pin:
            raise SystemExit(f"R-A owners: the {key} span of {rel} changed; re-review the owner it prices")
    for rel, needle in STATEMENTS:
        text = fn if rel == FN else sr
        if text.count(needle) != 1:
            raise SystemExit(f"R-A owners: {needle!r} is not one site in {rel}; re-review the owner it prices")
    shortcuts = _span(fn, *SPANS["shortcuts"][1:3])
    for construct, count in SHORTCUTS_ALLOCATIONS.items():
        if shortcuts.count(construct) != count:
            raise SystemExit(f"R-A owners: mod shortcuts holds {shortcuts.count(construct)} of {construct!r}, "
                             f"reviewed {count}; re-review the owners it prices")
    ASSUMED.update(ASSUMED_RA)
    return True


def hints(fn_):
    """RA-HS: one B1 hint vector, `build_hints`' `(0..count).map(rule).collect()`: a trusted-length range, so exact
    capacity at the descriptor count, at most Fn = 42m + N + s + 6g (the descriptor bound T07 A3 and E1 already use)."""
    return Xc("Option<u32>", fn_)


def shortcuts(n_dofs, fn_):
    """RA-SC: one evaluation's Shortcuts: `zero` (n bools), `window` (n u8), `products` (n usize), `touched` (push-grown
    usize, at most one entry per DOF in a row: <= n live, drained per row, capacity kept) and `source_ok` (bools
    reserved at the descriptor count, one push per row, so never reallocated). n = c.f.len() is the DOF count."""
    return B(n_dofs) + B(n_dofs) + B(8 * n_dofs) + Vbytes(n_dofs, 8) + B(fn_)


def touched_old(n_dofs):
    return B(Vbytes(n_dofs, 8).get("1", 0) // 2)
