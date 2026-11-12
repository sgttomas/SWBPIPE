"""§8.45: reviewed dispositions of the production literals longer than the `static` class.

composite_text sizes the `static` class from the four files that define the &'static str fields it
names; text rules apply that class across the snapshot. Every production literal in the pricing
scope (crate_dirs.txt, test-only modules excluded by rust_literals.production_literals) that is
longer than the class must match exactly one entry here, and every entry exactly one literal
(`head` is the start of the literal's runtime value), else the run stops. Dispositions:
  own-bound   copied into text only where another bound covers it (cited);
  per-site    copied at a site priced at the literal's own lexed length (cited rule);
  never-copied  never copied into text (cited why).
Not covered: a value that reaches a static-class site through computation (format!, concatenation,
a runtime String) rather than as one literal; that premise predates §8.45.
"""
PP = "core/product_physics/src/"
ENTRIES = [
    {"file": PP + "joint.rs", "head": "Objective connectors (expansion joints",
     "disposition": "per-site",
     "cite": "CONNECTOR_LIMITATION; its one copy, exact_admission.rs pressure_v3_formulation_basis_with_source's "
             "`super::joint::CONNECTOR_LIMITATION.to_string()`, is audited at the constant's lexed length "
             "(o10_rules.py SL-LIM)"},
    {"file": PP + "bend_pressure.rs", "head": "Realized curved bends",
     "disposition": "per-site",
     "cite": "BEND_LIMITATION; its one copy, lib.rs formulation_basis_for_model_with_source's "
             "`bend_pressure::BEND_LIMITATION.to_string()`, is audited at the constant's lexed length "
             "(o10_rules.py SL-LIM)"},
    {"file": PP + "exact_admission.rs", "head": "3.0.0/exact_pressure_v3 under pressure-source-1 semantics",
     "disposition": "own-bound",
     "cite": "one of the two `identity` literals; copied only by `identity.to_string()`, audited at the longer "
             "identity literal (o10_rules.py U6-J4). §8.44: unchanged (1,262 B); the pressure-2 literal it is "
             "audited at grows"},
    {"file": PP + "exact_admission.rs", "head": "3.0.0/exact_pressure_v3 under pressure-3 semantics",
     "disposition": "own-bound",
     "cite": "the longer `identity` literal; copied only by `identity.to_string()`, audited at its length "
             "(o10_rules.py U6-J4). §8.44 (RT-A3 increment 3): its constant-effort clause lengthens it to "
             "3,007 B (+540); carried, since U6-J4 measures the literal from source and no copy site is added. "
             "RT-L2: the head names pressure-3 for pressure-2 (same length); no copy site is added"},
    {"file": PP + "exact_admission.rs", "head": "reducer maps a member of this pressure region",
     "disposition": "own-bound",
     "cite": "REDUCER_IN_REGION_TEXT; passed only as pressure_runtime.rs problem()'s `message`, which the "
             "diag row prices at the error_display class (8,192 B; args rule `^(e|error|...|message|...)$`)"},
    {"file": PP + "retained_memory.rs", "head": "v1;Cargo.lock=",
     "disposition": "never-copied",
     "cite": "REGISTERED_PROFILES' reviewed_inputs; read only by bindings_hold, which compares it with "
             "COMPILED_REVIEWED_INPUTS (no copy, no format)"},
]
