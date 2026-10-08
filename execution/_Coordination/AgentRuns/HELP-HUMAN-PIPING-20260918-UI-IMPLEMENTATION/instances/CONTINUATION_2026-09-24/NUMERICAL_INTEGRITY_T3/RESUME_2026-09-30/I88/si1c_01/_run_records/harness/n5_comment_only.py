#!/usr/bin/env python3
"""I88 T3-SI1c: build commit 1 (N-5, comments only) from main's files.

Applies only the N-5 wording changes (I87 PLAN §5.2) to main's evaluator
source and README, in place, in the worktree. Then `--check A B` compares two
versions of lib.rs with comment lines removed (RV104 ADDENDUM_01's method):
the non-test code must be byte-identical.

Usage: n5_comment_only.py apply <lib.rs> <README.md>
       n5_comment_only.py check <lib.rs A> <lib.rs B>
"""
import hashlib
import re
import sys


def rep(s, old, new):
    assert s.count(old) == 1, old[:80]
    return s.replace(old, new)


def apply(lib, readme):
    s = open(lib).read()
    s = rep(s, '''    /// Boolean conjunction/disjunction. Evaluation is eager: both operands
    /// are always evaluated, so diagnostics in either operand always surface.
''', '''    /// Boolean conjunction/disjunction. Evaluation never short-circuits on a
    /// value: the right operand is evaluated even when the left one decides
    /// the result, so a blocking diagnostic in it still blocks. Operands are
    /// evaluated left to right, and evaluation stops at the first operand
    /// that blocks, so a later operand's diagnostics are not reported.
''')
    s = rep(s, '''    /// Eager conditional: condition, then-branch, and else-branch are all
    /// evaluated (in that fixed order) regardless of the condition value, so
    /// diagnostics in the unselected branch still block. Branches must both
    /// be booleans or both be quantities of the same dimension with matching
    /// unit references.
''', '''    /// Eager conditional: the condition, then-branch and else-branch are
    /// evaluated in that fixed order whatever the condition's value, so a
    /// blocking diagnostic in the unselected branch still blocks. Evaluation
    /// stops at the first of them that blocks, so a later one's diagnostics
    /// are not reported. Branches must both be booleans or both be quantities
    /// of the same dimension with matching unit references.
''')
    s = rep(s, '''            // Eager: both operands always evaluated; no value short-circuit.
''', '''            // No value short-circuit: the right operand is evaluated even
            // when the left decides; evaluation stops at the first that blocks.
''')
    s = rep(s, '''            // Eager: condition, then-branch, else-branch all evaluated in
            // this fixed order regardless of the condition value.
''', '''            // Eager: condition, then-branch, else-branch evaluated in this
            // fixed order whatever the condition's value; evaluation stops at
            // the first that blocks.
''')
    s = rep(s, '''            // Eager, as in the point path: all three are always evaluated.
''', '''            // Eager, as in the point path: all three are evaluated in this
            // fixed order whatever the condition; evaluation stops at the
            // first that blocks.
''')
    open(lib, "w").write(s)
    r = open(readme).read()
    r = rep(r, '''- Boolean `and`/`or`/`not` and the eager `select` conditional (all
  subexpressions always evaluated; diagnostics in unselected branches block).
''', '''- Boolean `and`/`or`/`not` and the eager `select` conditional (evaluation
  never short-circuits on a value, so a blocking diagnostic in an operand the
  result does not need, or in an unselected branch, still blocks; operands
  are evaluated in a fixed order, and evaluation stops at the first that
  blocks, so a later operand's diagnostics are not reported).
''')
    open(readme, "w").write(r)


def code_only(path):
    """The file with `//` comment lines (including `///` and `//!`) removed."""
    lines = open(path).read().splitlines()
    kept = [line for line in lines if not re.match(r"^\s*//", line)]
    return "\n".join(kept) + "\n"


if sys.argv[1] == "apply":
    apply(sys.argv[2], sys.argv[3])
else:
    a, b = code_only(sys.argv[2]), code_only(sys.argv[3])
    ha, hb = hashlib.sha256(a.encode()).hexdigest(), hashlib.sha256(b.encode()).hexdigest()
    print(f"code-only sha256 A {ha} ({len(a.splitlines())} lines)")
    print(f"code-only sha256 B {hb} ({len(b.splitlines())} lines)")
    print("IDENTICAL" if a == b else "DIFFERENT")
