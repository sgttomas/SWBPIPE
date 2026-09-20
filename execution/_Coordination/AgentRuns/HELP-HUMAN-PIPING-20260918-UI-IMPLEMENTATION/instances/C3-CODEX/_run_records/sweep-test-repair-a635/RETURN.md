# Edge test repair return

Implemented by WORKING_ITEMS /root/i1_manager, actual Codex gpt-6-astra/high, under ROOT
HELP_HUMAN. No child dispatched. ROOT disposition SHA256
3e9d015f7459777c17fe8902da8b9bc8e9006515a170d601f6948aa93289d2d8 authorizes this
single test-block repair. Diagnosis and original failure remain unchanged beside this folder.

Only viewportFigureEdge.test.ts changed among maintained files: its named edge-resource
case's final admission block/comment. All preceding pre-admission assertions and every byte
after that case remain unchanged. Both plain and edged meshes now enter equivalent resources;
each must have exactly the layer/mesh/one-companion graph, shared geometry and inactive count 0
companion. Complete post-admission live ledgers must agree and retain exact common totals:
2 pipe meshes, 1 geometry, 2 materials, 0 textures, 2 matrix buffers, 2 colour buffers.
Thus a resource introduced by the edge option still fails; nothing is arbitrarily ignored.

Validation, sequentially, one worker and Vitest 4.1.10:
- Same affected case: 1 passed, 23 unselected/skipped, exit 0 (after its recorded prior failure).
- Complete edge file: 24 passed, exit 0.
Exact commands, Node version and log hashes are RESULTS.json; raw stdout/stderr is preserved.
The exact 17-line addition/6-line removal diff is test.patch, SHA256
95f60162caf64c88d1f03e99055c17ad7046a29d23a073031d09ab0e3e1a781e.
Final test SHA256 9cad88ad83ce7b88e023b67be72276fe03b686a77f1ba8c1283af031b6fdba24.

PRESERVATION.json binds unaffected prefix/suffix bytes and source identity. All production,
shader, picking, other tests and 83 protected first-profile/picking files remain unchanged.
Before this test-only change, the complete src tree was byte-identical to failed a635; the only
src tree difference now is this authorized test. Native src-tauri tree remains
14d002f968cfcdde0442d9cfdf88c3fa6f684a1e. No performance/resource limit, first-profile oracle
or geometric tolerance changed. The original sweep remains 1488 pass/3 fail;
these focused passes do not establish a complete sweep or repair the two shell-owned failures.

Checkpoint contains only this authorized test plus diagnosis/repair evidence. No push, PR,
browser/native/build/full sweep/timed work was performed. ROOT owns independent bounded
backcheck and a fresh complete clean sweep. Existing native production evidence remains
source-equivalent; no new native or performance qualification is claimed. Future resource
admission impact selection should include the edge test consumer even when edge shaders do
not change. C4 remains separately held. Standard F-PIP-2/DEC-081 fence applies.
