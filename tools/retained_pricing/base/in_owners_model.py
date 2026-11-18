"""IN-WS (DEL-04-01 REQ-015, the inverse-norm gate; DRAFT, unregistered): the positive heap owner the formation
check's inverse-norm certificate adds, in both modes, and the zero-heap slot each factor object carries.

The gate (`formation_check::verified_inverse_operand`) asks the factor's `InverseNormInput::verified_bound` for a bound
at Q = mul_up(1024, est). The first ask on a factor runs `CertifiedMatrix::certify`, i.e. `run` in
structural/inverse_certificate.rs, on the factor's own scaled matrix (dimension n = F, the free DOFs); every later ask
at the same Q reads the slot. What the run allocates, all freed before `verified_bound` returns except the slot's
`Option<VerifiedBound>` (plain scalars, no heap):

  envelope   order, position, first: three exact usize vectors of n (Natural: two `(0..n).collect()`; Supplied: two
             `to_vec`; `position` a `vec!`).
  blocks     `of` (exact n usize); `members`, push-built, at most n components: headers pushcap(n); each component is
             push-built from empty, pushcap(h) <= 4 + 2h, so <= 4n + 2n = 6n usize in all; the DFS `stack`, one live at a
             time, <= pushcap(n) usize.
  records    one exact `BlockRecord` per block (<= n); each `attempts` push-built to at most three (E sigma1, E sigma2,
             D), capacity 4; each attempt's `figures`: `vec!` of one, `extend` of six (capacity 7), one or two pushes
             (capacity 14), so <= 14 `(&'static str, f64)` per attempt, <= 3n attempts.
  shifts     sigma1, all, sigma2 and D's zero shifts: four exact f64/usize vectors of n (blocks <= n); `retry` and
             `need_d` are filter-collects: pushcap(n) each.
  factor     one `Factored` live at a time (each E/D pass drops it): rows, exact headers n and exact row lengths
             i - first[i] + 1 <= i + 1, so <= H = n(n+1)/2 f64; `shifted`, `work`, the `first` clone: 3n words;
             `failure`: one Option<FactorFailure> per block.
  passes     `mine` (filter-collect, pushcap(n) usize) and a_col, b_col, a, c (four exact f64 vectors of n).
  assemble   on not verified only, while `records` lives: `attempts` push-built to <= 3 (capacity 4) AttemptReasons,
             each with a push-built `reasons` list of distinct reasons (deduplicated by `find`), at most four per kind:
             E can give LemmaENonpositivePivot, ArithmeticNonfinite, LemmaESigmaNonpositive, BoundExceedsExistingQuantity
             and D LemmaDNonpositivePivot, ArithmeticNonfinite, LemmaDContractionNotBelowOne, BoundExceedsExistingQuantity
             (pushcap(4));
             `candidates` push-built to <= 3n CandidateBound.

Every part is summed (no slot reuse is assumed between parts that are not live together), so the formula bounds the
run's peak. The length bounds are properties of `run`'s and `assemble`'s straight-line structure (blocks partition the
n positions; `all`, `retry` and `need_d` each hold distinct block indices and each pass pushes one attempt per listed
block; each attempt forms at most nine figures; `assemble` iterates three kinds and pushes at most one candidate per
attempt); the kernel's `the_record_vectors_stay_within_the_priced_lengths` checks each, with capacities.

Where it is live (LIVE_PHASES). The workspace exists only while `verified_bound` runs, inside one formation check, and
is freed before it returns. The ordinary solve runs on one thread (product_physics `on_reserved_stack` joins its one
scoped worker), and its cases and W2 evaluations call the check one after another, so at most one workspace is live at
a time, and only while an ordinary solve runs: the ordinary spans W1, X1 and X3 (retained_product's
`run_linear_static_preview_observed` call). The retained producer's own phases (T11-T15), publication, validation and
transfer run no formation check on D1 (hanger derived solves and nonlinear confirmation are refused there). It is
therefore charged once, outside O_base, in those three phases only, as T25 is charged in X1 only. The factor objects' `certificate: CertificateSlot` (structural.rs PositiveFactor, sparse.rs
SparsePositiveFactor and SharedSparseFactor) is inline in a stack-held object and owns no heap: it is listed as a
zero-heap owner so that it appears in the tree. Confirmation attempts borrow their factor's slot and add nothing; the
confirmation phases are not on D1 (D1.6: no nonlinear supports).

Every allocating construct in the certificate's production text is counted (cfg(test) items stripped); a count that
moves, or a pinned body that changes, stops the run. A snapshot without the certificate's gate adds nothing, so
earlier registered trees keep their coefficients."""
import hashlib
import os
import re

from g3lib import s, B, X, pushcap, ASSUMED
from cfg_test_items import production as _production

IC = "core/solver/frame_kernel/src/structural/inverse_certificate.rs"
FC = "core/solver/frame_kernel/src/structural/formation_check.rs"

# Read at 0253f475d (the gate's head with its round-2 strengthenings).
BODY_PINS = {
    "envelope": "b40f11c0a594ef6a596d265d98645c8cb54323cf1124cdd920f66f3f79b9b48e",
    "blocks": "f89ab96f2f0945cdf09aa3f37242b5f87286ab3f1b02406d57cdeceb137f554e",
    "factor": "edb76ea65afd4987c24929e9a589c26d6cbe84293347dd69e50bc1fa80698d18",
    "passes": "29b7fcac6a696b26e2465fd0cf7ded0290ca95a46eea571d2760feaf06d99aee",
    "run": "cbe64082ce9fade760a857e52c965faa0ff092f0ca203185ad747d60104becfb",
    "assemble": "1bf42b2eea8f60a861e6fd87c4d5d3e319aa02fb528f424f4d0b4403118899f5",
    "unusable": "9308f2518f671d83e7e642500a4a64d3c4d7f54c212e5c411ee072c37565651c",
    "dense_run": "ecdfc95cbf77039cb9b26b8b859e8ed4bfb1246762e855320543df8542660577",
    "compressed_run": "bef0670f0526b224278f69a9f568b710956487092016dec5ed8f7fc35b40e01c",
    "certify_dense": "96468b8e71ed45b5c81ee1fb7d5982737027acb44399444daabd11f2d93422f8",
    "certify_compressed": "b8e8b2444c3661dadfe2e017bd519f91f6cc2c0f32930692e404652bcbd1d9cb",
    "certify_compressed_bits": "f44784ae0b0d160718ca7307afefaa83cb5cfaba52ff3afccabeab286ff012b5",
    "certify": "7825cd8ba68e469038e859e946b7ce9075a158bf63ee7f313ee080c02966fec3",
    "verified_bound": "6648ae740fcac7ad5b0379c0e2d7f9f06e1fe4f58cf088358d29690173edeccf",
    "validate_dense": "9883cf8165a4d6ca838efb91fc4fe41b160c06c0a52014f09ffe29a2ddb4af18",
    "validate_compressed": "c9c854e3f65f2fd5b3bcb97ffb1bd089a18a8857ac1f745daca7a0d61d6eaf26",
}
# Allocating constructs in the production text (cfg(test) items stripped). Two `.collect()` and one `.extend(` sit in
# `impl NotVerified`, which only tests call (it carries allow(dead_code) outside test builds); one `.clone()` is
# `first.clone()` (priced), two clone an Option<VerifiedBound> (no heap).
CONSTRUCTS = {"vec![": 14, "Vec::new()": 10, ".collect()": 11, ".to_vec()": 2, ".clone()": 3,
              "Vec::with_capacity(": 0, ".push(": 12, ".extend(": 3, "String": 0, "format!": 0, "Box::new": 0,
              "Rc::": 0, "Arc::": 0}
GATE = "let operand = verified_inverse_operand(est, inverse)?;"
LIVE_PHASES = ("W1 ordinary span", "X1 ordinary span", "X3 republication")

# Illustrative only; g5_profile binds each to a source upper the kernel's premise test checks against size_of.
ASSUMED_IN = {"s(CertificateBlockRecord)": 48, "s(CertificateAttempt)": 64, "s(CertificateFigure)": 24,
              "s(CertificateFailure)": 8, "s(CertificateReasons)": 40, "s(CertificateReasonCount)": 16,
              "s(CertificateCandidate)": 40}


def _read(snapshot, rel):
    p = os.path.join(snapshot, rel)
    return open(p, encoding="utf-8").read() if os.path.exists(p) else None


def _body(text, name):
    hits = list(re.finditer(r"^([ \t]*)(?:pub(?:\([a-z]+\))?\s+)?fn\s+" + re.escape(name) + r"\b", text, re.M))
    if len(hits) != 1:
        return None
    ind = hits[0].group(1)
    end = text.find("\n" + ind + "}\n", hits[0].start())
    return text[hits[0].start():end + len(ind) + 3] if end >= 0 else None


def digests(snapshot):
    text = _read(snapshot, IC) or ""
    out = {}
    for name in BODY_PINS:
        body = _body(text, name)
        out[name] = hashlib.sha256(body.encode()).hexdigest() if body else None
    return out


def active(snapshot):
    """True when the gate and its certificate are present with their reviewed digests and construct counts; False
    when neither is (an earlier revision); anything else stops."""
    if not snapshot:
        return False
    ic = _read(snapshot, IC)
    fc = _read(snapshot, FC) or ""
    if ic is None or "verified_inverse_operand" not in fc:
        # before unit 1, or unit 1 alone: the certificate is not on a live path
        return False
    if GATE not in fc:
        raise SystemExit("IN-WS: the gate's helper is present but its call is not the reviewed one; re-review the owner")
    current = digests(snapshot)
    for name, pin in BODY_PINS.items():
        if current[name] != pin:
            raise SystemExit(f"IN-WS: {IC} fn {name} changed or is missing; re-review the owner it prices")
    production = _production(ic, IC)
    for construct, count in CONSTRUCTS.items():
        if production.count(construct) != count:
            raise SystemExit(f"IN-WS: {IC} holds {production.count(construct)} of {construct!r} in production, "
                             f"reviewed {count}; re-review the owner it prices")
    ASSUMED.update(ASSUMED_IN)
    return True


def workspace(F, H):
    """IN-WS: one certificate run on the factor's matrix of dimension F (H = F(F+1)/2), summed over its parts."""
    usize_f64 = 8
    envelope = B(3 * usize_f64 * F)
    blocks = (B(usize_f64 * F) + s("Vec") * pushcap(F) + B(usize_f64 * 6 * F)
              + B(usize_f64 * pushcap(F)))
    records = (s("CertificateBlockRecord") * F + s("CertificateAttempt") * (4 * F)
               + s("CertificateFigure") * (14 * 3 * F))
    shifts = B(4 * usize_f64 * F) + B(2 * usize_f64 * pushcap(F))
    factor = s("Vec") * F + B(usize_f64 * H) + B(3 * usize_f64 * F) + s("CertificateFailure") * F
    passes = B(usize_f64 * pushcap(F)) + B(4 * usize_f64 * F)
    assemble = (s("CertificateReasons") * 4 + s("CertificateReasonCount") * (3 * pushcap(4))
                + s("CertificateCandidate") * pushcap(3 * F))
    return envelope + blocks + records + shifts + factor + passes + assemble


def slot():
    """The CertificateSlot on each factor object: inline, no heap (zero-heap owner, listed for visibility)."""
    return X()
