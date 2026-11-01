"""Model document 0.9.0 S1 D3 (DEL-02-01 Design model-document-0.9.0.md; retained-pricing-rules.md §8.36): the positive
owners the 0.9.0 lowering and withholding add outside the chain's TEXT inventory, on D1.

D1.3 refuses a non-empty condition_sets or spectra and any pipe's condition_set_ref, D1.4 a difference basis, a
subtrahend support state, an unsigned basis and a term's combination operand, D1.5 a case's condition_use and
test_condition, and D1.6 a support's direction and the snubber family. What the two passes still own on D1:

S1-VW (operating_conditions::lower -> document_members::model_findings, once per ordinary run, transient): the views,
  exact-size collections over the typed model: m `(&str, Option<&str>)` pipes, c `(&str, Option<&ConditionUseInput>)`
  cases, g SupportView with each support's restraint `&str`s (r in all), z CombinationView with each one's <= 3
  TermView and <= 3 operand `&str`s in a four-slot backing (the flattened collect has a zero lower size hint);
  supports_findings' transient one-entry Vec<String> backing; all_findings' c case `&str`s; the findings, a push-grown Vec<Finding> of at most
  4z (S1-L1) with its previous backing and each finding's two-entry refs Vec (its texts are TEXT); and
  combinations_findings' resolution: the Operands set of c case ids and map of z combination ids, the z edge lists
  (<= 3 targets each, push-grown), the z x z reachability rows with one row's work stack (<= 3 targets, extended by
  <= 3 per visited combination), the reported flags, one cycle's member indices (<= z) and its name and ref Vecs (<= z
  Strings each), and one combination's operand references (<= 3 `(String, &str, Operand)`, push-grown) with its
  one-entry refs Vec. The condition-set and spectrum passes' HashSets stay empty (D1.3) and allocate nothing.
S1-WH (document_members::withhold_interim_combinations, once per ordinary run, route L with z >= 1, transient): its own
  Operands, the withheld index set (<= z), and per combination (one at a time) its interim reasons (<= 4
  `(&str, String)`, push-grown), the combination's view (<= 3 TermView, <= 3 operand `&str`s in four slots) and its operand
  references (<= 3, push-grown). The diagnostics' texts are TEXT.

Bound to the three modules' complete bytes and the two call sites; a snapshot without the modules adds nothing (earlier
registered trees keep their coefficients), a changed module or call stops the run."""
import hashlib
import os

from g3lib import B, s, pushcap, HashReq, HashMov

PP = "core/product_physics/src/"
PINS = {PP + "document_members.rs": "6ada6a00c3f8300747dfa3420fd920df595e2e3b1f88d5a4cb0bc8c32e463910",
        PP + "operating_conditions.rs": "81460837757326180e30d9cca33bce6b7c3a2357b403dea37409084dc05bf8ef",
        PP + "spectra.rs": "b1ed575817c3d9ffd4edd16fcfefc1f8e9029c13c2985c9219e868d3e3f69060"}
# The solve's two calls, each once in run_linear_static_preview_observed_inner, and their twins in pre_formation_checks.
CALLS = ((PP + "lib.rs", "operating_conditions::lower(&mut model, &mut diagnostics);", 1),
         (PP + "lib.rs", "document_members::withhold_interim_combinations(&mut model, &mut diagnostics);", 1),
         (PP + "validation/model_checks.rs", "crate::operating_conditions::lower(&mut model, &mut diagnostics);", 1),
         (PP + "validation/model_checks.rs", "crate::document_members::withhold_interim_combinations(&mut model, &mut diagnostics);", 1))
TERMS = 3          # D1.9: a combination's terms and range operands
REASONS = 4        # interim_reasons' four pushes


def inspect(snapshot):
    """True on the reviewed S1 source; False when the modules are absent; anything else stops."""
    present = [os.path.exists(os.path.join(snapshot, f)) for f in PINS]
    if not any(present):
        return False
    for f, digest in PINS.items():
        with open(os.path.join(snapshot, f), "rb") as stream:
            if hashlib.sha256(stream.read()).hexdigest() != digest:
                raise SystemExit(f"S1 owners: {f} changed; re-review §8.36's owners")
    for f, text, count in CALLS:
        with open(os.path.join(snapshot, f), encoding="utf-8") as stream:
            if stream.read().count(text) != count:
                raise SystemExit(f"S1 owners: the call {text!r} in {f} changed; re-review §8.36's owners")
    return True


def _operands(cases, z):
    """Operands::new: a HashSet of the case ids (collected) and a HashMap of the combination ids (inserted)."""
    return HashReq("&str", cases) + HashMov("(&str,usize)", z)


def _operand_refs():
    return s("(String,&str,Operand)") * (pushcap(TERMS) + pushcap(TERMS) // 2)


def _view_children():
    # combination_view's Option<Vec<_>>::iter().flatten().map(...).collect()
    # has a zero lower size hint, so a nonempty operand list grows from the
    # allocator's minimum four slots even when its length is at most three.
    return s("TermView") * TERMS + s("&str") * pushcap(TERMS)


def views_and_findings(m, cases, g, r, z):
    """S1-VW (per ordinary run): model_findings' views, all_findings' findings and combinations_findings' resolution."""
    views = (s("(&str,Option<&str>)") * m + s("(&str,Option<&T>)") * cases + s("SupportView") * g + s("&str") * r
             + (s("CombinationView") + _view_children()) * z + s("&str") * cases
             + s("String"))  # supports_findings' one-entry refs Vec, one support at a time; text is TEXT.
    # Operands::new runs with or without a combination: its case-id set is collected either way.
    if not z:
        return views + HashReq("&str", cases)
    found = 4 * z
    findings = s("Finding") * (pushcap(found) + pushcap(found) // 2) + s("String") * (2 * found)
    stack = TERMS + TERMS * z          # one row's work stack: the start's targets, then <= 3 per visited combination
    resolution = (_operands(cases, z) + s("Vec") * z + B(8 * (pushcap(TERMS) + pushcap(TERMS) // 2) * z)
                  + (s("Vec") + B(z)) * z + B(8 * (pushcap(stack) + pushcap(stack) // 2)) + B(z)
                  + B(8 * z) + s("String") * (2 * z) + _operand_refs() + s("String"))
    return views + findings + resolution


def withholding(cases, z):
    """S1-WH (per ordinary run, z >= 1): the withholding pass, one combination at a time."""
    if not z:
        return HashReq("&str", cases)          # Operands::new's case-id set; the loop does not run
    reasons = s("(&str,String)") * (pushcap(REASONS) + pushcap(REASONS) // 2)
    return _operands(cases, z) + HashMov("usize", z) + reasons + _view_children() + _operand_refs()
