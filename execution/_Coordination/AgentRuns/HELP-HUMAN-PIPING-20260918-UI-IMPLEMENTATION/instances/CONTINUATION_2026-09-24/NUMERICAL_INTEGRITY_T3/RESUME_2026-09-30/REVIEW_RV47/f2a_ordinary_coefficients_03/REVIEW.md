# RV47-C1 — correction confirmed

**RV47-C1 is repaired at `7bca4a0dfd88de4fd3f6d1c59de3c98a189ab7ee`.
No new blocking or non-blocking finding.** Recommend ROOT close this narrow
evidence finding. This confirmation preserves the earlier conditional proof
verdict and all implementing/qualification obligations.

Same native TASK reviewer `/root/rv47_f2a_preview_truth`, continuation under
ROOT `/root`. Brief `9fb9748897071fa8a7dbb33548e802d36c922e77`.
Actual receipt 2026-10-02 22:51:55 UTC; new-check cutoff 23:01:55;
deadline 23:06:55. Checks completed by 22:53:37 UTC.

The correction's `exact_k_products` multiplies independently lifted Fraction
operands without rounding the result. I independently recomputed all four EA,
GJ, EI_z and EI_y products by multiplying their integer numerators and
denominators, then recomputed both historical and corrected endpoint-distance
deltas. Every reported rational value matches; all four products and all four
deltas changed as disclosed.

The new discriminator calls the same constructor. For `x=1+2^-52`, its exact
product is `1+2^-51+2^-104`, while RN64 is `1+2^-51`. With the source singleton
at RN64, the required delta is `2^-104`; the old rounded-product substitution
would give zero. Replacing only the constructor with that faulty substitution
makes the isolated discriminator fail. This is correction-sensitive evidence,
not merely a replay of the original passing test.

Both frozen checkers replay byte-identically: historical 17 groups and corrected
18 groups. The 16 unaffected groups' statements, their original order, 32 named
unaffected exact states and pre-existing JSON fields are preserved. The original
raw script/results/checks/execution/origins and DEPENDENCIES remain byte-equal
to `f5ca39a1a2`. RETURN's theorem sections 1–3 are unchanged; its edits disclose
the defect and correction. All nine files of the preceding RV47 review remain
unchanged.

The candidate commit has exactly the authorized five-file write set: two
modified records (RETURN and inventory) and three new correction records.
All ten inventory payloads, seven correction-origin hashes and three recorded
command hashes verify. The packet tree matches its inventory plus the inventory
itself. Exact checks, commands and hashes are recoverable in `_run_records`;
no copied product tree is included.

This closes only the exact-K control defect. Source/row association, arithmetic
and I35 grammar/cost integration, actual layouts/lifetimes, aggregate accounting,
code review and protected availability remain open as recorded previously. No
source promise, operator, output, predicate, policy, product qualification or
availability claim changes. Only assigned review evidence was written; no
maintained source, compiler, solver, model/product runtime, host probes,
Git/index/API writes or delegation occurred. ROOT owns acceptance and Git.
