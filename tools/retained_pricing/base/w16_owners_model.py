"""W-16 slice a D3 (DEL-04-03 Design pf-imposed-displacement.md; retained-pricing-rules.md §8.23): the positive owners
imposed displacement on the pressure-free route adds outside the chain's TEXT inventory. D1.7 refuses every
support-targeted primitive (W16-NS), so no admitted case carries motion; what remains on D1 is priced here.

W16-NI (both routes, per case solve, CasePrefix): `imposed_motion::case_prescribed` runs for every case solved without a
load state (the pressure-free route, and route E, where B7b-LS makes `load_state` None) and first collects `node_index`,
a HashMap<&str, usize> over the model's n nodes from an exact-size iterator (one table, no growth), before its loop. It
lives to the function's return, one case at a time. Its `prescribed` BTreeMap is inserted into only after the loop's
let-else on a support target, so it allocates nothing on D1 (`BTreeMap::new` does not allocate); `any` stays false and
the function returns None. Charged as one table of n entries per case, beside the case's own node index.
W16-VF (validation, per ordinary run, BuildScratch): `imposed_motion::validate` collects its findings in `found`, a
push-grown Vec<Diagnostic>, and then moves them into the run's diagnostics (`diagnostics.extend(found)`). On D1 the only
reachable finding is validate_pressure_free's node/element `imposed_displacement` refusal (route L), at most one per
primitive load: <= c*l entries, with the previous backing while it grows. The diagnostics' strings are TEXT (W16-ID) and
their count is in D; only this separate backing is added. Charged on both routes (an over-bound on route E, where the
exact branch pushes nothing on D1), as W7-QF is.
Not owners on D1: validate_pressure_free's per-case `prescribed` HashMap (inserted into only on a support target), the
DOF name `to_ascii_uppercase()` (support branch) and refuse's `support.map(str::to_string)` (None at the one reachable call).

W-16 stage 2 D3 (DEL-04-03 Design pf-nonlinear-imposed-motion.md §7, §10.4, §11.5; retained-pricing-rules.md §8.29):
W16S-SG (both routes, per case solve, CasePrefix): `imposed_motion::case_spring_ground` runs for every case solved
without a load state and allocates `ground`, a vec![0.0; s] (f64), and `moved`, a vec![false; s] (bool), each of the
case's s prepared springs, even without a spring target; `moved` ends at its return and `ground` lives through the case's
recovery. A load-state case allocates the length-s zero vector directly. On D1 every ground is 0.0, so
certified_loads::spring_base_motion pushes no ledger term. Charged as both vectors per case, one case at a time.
Not owners on D1: `case_ground_motion`'s Vec<(String, f64)>, collected from a filter_map that yields nothing without a
support target (Vec's FromIterator allocates only on a first element), and the nonlinear loop's ground and base maps
(D1.6 refuses nonlinear supports).

Bound to imposed_motion.rs's sha256 (W-16's new module) and to the exact statements, whitespace-normalized; a snapshot
without the module adds nothing (earlier registered trees keep their coefficients), a partly present or changed owner stops
the run."""
import hashlib
import os
import re

from g3lib import B, HashReq, s, pushcap

PP = "core/product_physics/src/"
IM = PP + "imposed_motion.rs"
LIB = PP + "lib.rs"
PIN = "7069d94e1590c0ce198d4d4ddfe593d46b2d1644f5937a05abf6cf8040217478"
CL = PP + "certified_loads.rs"

# (file, function, statement, count) with line breaks and their indentation removed.
STATEMENTS = (
    (IM, "case_prescribed",
     "let node_index = model.nodes.iter().enumerate().map(|(index, node)| (node.id.as_str(), index)).collect::<HashMap<_, _>>();", 1),
    (IM, "case_prescribed", "let mut prescribed = BTreeMap::new();", 1),
    (IM, "case_prescribed", "let LoadTargetInput::Support { support, dof } = &load.target else { continue };", 1),
    (IM, "case_prescribed", "prescribed.insert(global, load.magnitude.value);", 1),
    (IM, "case_prescribed", "any.then_some(prescribed)", 1),
    (IM, "validate", "let mut found = Vec::new();", 1),
    (IM, "validate", "diagnostics.extend(found);", 1),
    (IM, "validate_pressure_free", "let mut prescribed: HashMap<(Base, &str, usize), &str> = HashMap::new();", 1),
    (IM, "validate_rigid_base", "if let Some(first) = prescribed.insert((Base::NodeDof, support.node.as_str(), dof_index(dof)), load.id.as_str()) {", 1),
    (IM, "case_ground_motion", "Some((record, _, Acting::Nonlinear)) => Some((record.id.clone(), load.magnitude.value)),", 1),
    (IM, "case_ground_motion", ".collect()", 1),
    (IM, "case_spring_ground", "let mut ground = vec![0.0; springs.len()];", 1),
    (IM, "case_spring_ground", "let mut moved = vec![false; springs.len()];", 1),
    (IM, "case_spring_ground", "let Some((record, dof, Acting::Spring)) = target_acting(model, load) else { continue };", 1),
    (IM, "target_acting", "let LoadTargetInput::Support { support, dof } = &load.target else { return None };", 1),
    (CL, "spring_base_motion", "if g == 0.0 {continue;}", 1),
    (LIB, "solve_prepared_load_case", "Some(_) => vec![0.0; spring_entries.len()],", 1),
    (LIB, "solve_prepared_load_case",
     "None => imposed_motion::case_spring_ground(model, load_case, spring_entries, diagnostics),", 1),
    (LIB, "solve_prepared_load_case", "certified_loads::spring_base_motion(&mut ledger, spring_entries, &spring_ground);", 1),
    (LIB, "solve_prepared_load_case",
     "None => imposed_motion::case_prescribed(model, load_case, restrained_dofs, diagnostics),", 1),
    (LIB, "run_linear_static_preview_observed_inner", "if imposed_motion::validate(&model, &mut diagnostics) {", 1),
)


def _read(snapshot, rel):
    path = os.path.join(snapshot, rel)
    if not os.path.exists(path):
        return None
    with open(path, encoding="utf-8") as stream:
        return stream.read()


def _body(text, name):
    hits = list(re.finditer(r"^([ \t]*)(?:pub(?:\([a-z]+\))?\s+)?fn\s+" + re.escape(name) + r"\b", text, re.M))
    if len(hits) != 1:
        return ""
    end = text.find("\n" + hits[0].group(1) + "}\n", hits[0].start())
    return text[hits[0].start():end] if end >= 0 else ""


def inspect(snapshot):
    """True when W-16's owners are present as reviewed; False on an earlier revision; anything else stops."""
    texts = {IM: _read(snapshot, IM), LIB: _read(snapshot, LIB) or "", CL: _read(snapshot, CL) or ""}
    if texts[IM] is None:
        if "imposed_motion::" in texts[LIB]:
            raise SystemExit("W16 owners: lib.rs calls imposed_motion without the module; re-review")
        return False
    if hashlib.sha256(texts[IM].encode()).hexdigest() != PIN:
        raise SystemExit(f"W16 owners: {IM} changed; re-review the owners it prices (W16-NI, W16-VF)")
    found = [re.sub(r"\s*\n\s*", "", _body(texts[rel], fn)).count(needle) for rel, fn, needle, _ in STATEMENTS]
    if found != [count for *_, count in STATEMENTS]:
        raise SystemExit(f"W16 owners: an owning statement or call changed; re-review W16-NI/W16-VF/W16S-SG: {found}")
    return True


def node_index(n):
    """W16-NI (per case solve, one case at a time): one exact-size HashMap<&str, usize> of n entries."""
    return HashReq("(&str,usize)", n)


def validation_findings(loads):
    """W16-VF (per ordinary run): validate's push-grown Vec<Diagnostic> of <= one finding per primitive load, with its
    previous backing."""
    return s("Diagnostic") * (pushcap(loads) + pushcap(loads) // 2)


def spring_ground(springs):
    """W16S-SG (per case solve, one case at a time): case_spring_ground's vec![0.0; s] (f64) and vec![false; s] (bool)."""
    return B(8 * springs) + B(springs)
