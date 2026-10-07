**piping(T3 B6): the retained-precision readers agree on G7 header codes, Python validates transported successors, and the shared corpus grows to 07m**

B6 settles four items carried from F2a's reader work. It changes reader tests, the TypeScript and Python readers, and the shared corpus. It touches no file in the F2a D1 milestone's build.

## What changes

- **TypeScript's G7 header refusal now carries the same code as Python and Rust.** It used one code for every header refusal. Five classes are aligned, among them the declared N-3 class and four that differed without a declaration. Each is pinned by a new corpus entry in all three languages.
- **Python validates a transported retained successor** with a transport validator, the twin of Rust's and TypeScript's, instead of refusing every one. RV92's ten tampered successors are refused with the same codes in all three languages. Python still refuses successor packages.
- **The shared corpus is now snapshot 07m:** eight entries appended and one per-reader expectation aligned. No existing entry moves. All three harnesses run the 07k and 07m slices and check their ids.
- **Python reads its own per-reader expectation,** as Rust and TypeScript do.
- **The declared differences go from six to five:** F-U6b-2 and the N-3 scope sentence are removed, and the pins assert their absence.

## What stays the same

- **No change to the product physics crate, the Rust reader, any D1 crate's source, a schema, a dependency or a lockfile.** The registered build identity is untouched.
- **Reader eligibility stays closed.** Every input whose code changes is refused before and after.
- **Every count change is an added test or entry.** No assertion is dropped.

## Review and gates

- **Independent review (RV108):** PASS (0 blocking, 0 should-fix, 7 notes).
  - It used its own 1,032 G7 probes, 98 tampered-transport probes, a raw-path differential and 30 mutants.
  - The notes are pre-existing reader differences and wording, routed to the next breadth slice (B1).
  - These are agent reviews, not personal review by the owner.
- **The package:** `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/B6/`, with the change record and a citation index.
- **Gates run before the merge and recorded on the integration branch:** source equality, citations, GEN-8, hosted CI with the full-SHA dispatch, and the Mac DEC-025 against a fresh main baseline.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
