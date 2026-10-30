# SWBPIPE repository split

This repository joins the owner-created destination history to the complete
SWBPIPE project extracted from `sgttomas/chirality`. The initial destination
commit `bda65d1adf27f23120ceaaac64a521dc7660f09a` remains an ancestor; its `LICENSE`
is retained. The project's original `LICENSE.md` remains unchanged. Both are
MIT notices for the same owner; their original attribution spelling is preserved.
No force push or original-history rewrite is part of the split.

Rehearsal source: `0828e8166ccefddc5c5b7bd5f75d58c2d656f51a`, subtree
`projects/chirality-piping/`. Unadapted extracted commit
`260eceb30138964d645a964da26926e8999f59aa` has exactly the same tree:
`406dd72fd8942ec7694ad12da2d639c50c96be03`. SWBPIPE Agent 0 selected this same commit as the final execution baseline.
Unmerged RT-E1 lanes are excluded from the import and will be replayed as fresh
branches/PRs by their manager after cutover; their private evidence stays outside
the public repository.

`commit-map.txt` maps original source identities to extracted commits, including
explicit tree-equivalent aliases for original commits that changed no project
content. Historical source citations remain Chirality identities. The owner-created
initial commit is not replaced. `historical-inputs.json` records verified trees
and normalized Git-archive identities for the pricing basis, archive and baseline.
The annotated `archive/piping-retained-corpus-20261011` tag identifies the mapped
archive and records its original tag object/commit. It is not a release.

`tools/repository_history.py` resolves original revisions explicitly (including
`source:<original-sha>`) from the local map, refusing missing or ambiguous entries.
Retained pricing accepts both original monorepo and standalone layouts. It retains
BASIS `2243be380b` as the documentary identity and records the actual mapped input.
The historical corpus generator and tests retain their exact source-byte checks;
missing required historical inputs fail rather than silently skipping tests.

Rehearsal pricing on both source and extracted history completed all four points,
passed both O-10 checks and reproduced the committed 1,695-line GENERATED PROFILE
block with `generate --check`. All 1,586 coefficients and the complete generated
trees matched. The registered budget/threshold, numerical source and expected
engineering results were not changed for relocation.

`shared-inputs.json` lists the bounded Root tools/actions/workflows copied with
original hashes. Local adaptations are maintained here. Ordinary Cargo/npm/Python
build/test inputs are project-local; development roles and methods resolve through
the selected Chirality library in `AGENTS.md`, following the West DOE pattern.
The former Root practitioner-harness registrations were retired development gates
and are not part of SWBPIPE's standalone product verification.

The required `harness` joins repository checks and selected product verification;
failed, cancelled or missing selected work fails it. Shared execution controls
remain explicit after path promotion. Unknown paths select conservative coverage;
complete diffs retain both sides of renames and deletions. Bounded routing tests
run on every PR, including added/deleted Rust files and Cargo manifests.

Owner decisions of 2026-10-29 select public `sgttomas/SWBPIPE`, local
`/Users/ryan/dev/swbpipe`, and replace RT-V8's sanitized publication mechanism with
this independent repository. "Push now, sign before v1.0": the verified history
import may be published before signature. The owner-signed D-20/DEC-058 scan,
including the full private-term scan and finding dispositions, remains a v1.0
release gate. Unsigned dry-run findings are evidence for cleanup, not clearance,
acceptance or a release. No repository protection/settings changes are implicit.
The exporter and its export-only reviewed-lint allowance are retired from this
repository. Historical reproduction remains available in the mapped Git history.
Reviewed binary hashes, the trusted-schema manifest and P7 provenance bindings
remain scan policy inputs. `provenance_scope_members.json` retains the already
reviewed P7 source set; the generator verifies tracking, bytes and exact labels
without importing an export allowlist or adding implicit exemptions. The Windows
startup diagnostic uses the tracked repository snapshot. Signed scan and release
acceptance remain owner acts.
