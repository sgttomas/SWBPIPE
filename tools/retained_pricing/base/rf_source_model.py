"""RF source-method owners, conditional on its declared-congruence source.

Historical sources retain their original formulas. The source-complete B7a
pins and O-10 loop inventory independently guard the production bodies; these
premises select the additional owners, never a smaller historical bound.
"""
import os


def active(snapshot):
    if not snapshot:
        return False
    path = os.path.join(snapshot, "core/solver/frame_kernel/src/structural/exact_boundary.rs")
    source = open(path, encoding="utf-8").read()
    if "pub struct CongruenceMember" not in source:
        return False
    for premise in ("const CONGRUENCE_VALUES: usize = 288;",
                    "const FUNCTIONAL_ORDER_CAP: usize = 4;",
                    "adjugates: Vec<Vec<Expansion>>", "congruence: Vec<CongruenceMember>",
                    "upper: Option<Vec<Vec<f64>>>", "minors: Option<Vec<Vec<f64>>>"):
        if premise not in source:
            raise ValueError("RF source owner premise changed: " + premise)
    return True


def coefficient_max(a, b):
    """Positive union of mutually exclusive source-construction bounds."""
    from g3lib import X
    return X({k: max(a.get(k, 0), b.get(k, 0)) for k in a.keys() | b.keys()})
