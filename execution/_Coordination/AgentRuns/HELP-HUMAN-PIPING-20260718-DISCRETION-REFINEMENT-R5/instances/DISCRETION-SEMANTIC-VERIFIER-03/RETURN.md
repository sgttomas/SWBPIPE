BLOCK

- **Material unique/exclusive-field bypass remains.** The corrected helper
  counts a case-insensitive field prefix only when it begins in column 1:
  `tolower(substr($0, 1, length(prefix))) == prefix_lower`. I executed that
  exact predicate against the valid four-field post-landing text and every
  required mutation class. Missing, replaced/wrong, exact-duplicate,
  column-1 contradictory, differently-cased, and suffixed variants of each of
  `Verdict:`, `Basis-Plan-Blob:`, `Basis-D54-Semantic-SHA256:`, and
  `Basis-DEC087-Semantic-SHA256:` all fell back as intended. However, the
  otherwise valid return plus any of ` Verdict: BLOCK`, a tab-prefixed
  `Verdict: BLOCK`, `- Verdict: BLOCK`, `> Verdict: BLOCK`, or
  ` Basis-Plan-Blob: deadbeef` still returned ACTIVE in the exact parser
  simulation. `Verdict : BLOCK` likewise remained invisible. Thus a
  whitespace- or Markdown-shaped contradictory field can coexist with exactly
  one expected column-1 line and satisfy the guard. This refutes the brief's
  unique/exclusive/otherwise-ambiguous requirement and the plan's claim that
  every ambiguous or contradictory S5 condition falls back.

- **The preserved v2 history is internally contradictory.** The enumerated R5
  tree contains
  `instances/DISCRETION-CARRY-FORWARD-VERIFIER-02/RETURN.md`, and its durable
  verdict is `BLOCK` on the pre-repair ambiguity. At the same time,
  `INTERRUPTION_RECORD.md`, `STATUS.json`, `WORK_GRAPH.json`, D-54 section 4,
  `ORCHESTRATION_PLAN.md`, `S5_REVIEW_PACKET.md`, the top-level `RETURN.md`,
  and the v3 carry-forward brief assert that v2 was interrupted **without a
  return**. The decision-register D-54 row and DEC-087 row still require fresh
  v2 verification, and top-level `RETURN.md` repeats that stale v2 gate,
  despite the corrected plan/graph requiring v3. A late v2 return need not be
  admitted to fan-in, but its existence and `BLOCK` verdict cannot coexist
  with these unqualified `NO_RETURN` claims. The governed tranche therefore
  does not truthfully preserve or enumerate v2 history.

- **Enumeration completed.** Before this authorized return, the tranche was
  exactly the new D-54, one appended D-54 register row, one appended DEC-087
  row, the re-minted plan, and 19 R5 files: seven top-level records; the
  integration brief; v1 semantic/carry briefs and returns; v2 semantic brief
  and `BLOCK` return; v2 carry brief, interruption record, and late `BLOCK`
  return; both v3 briefs; and this v3 semantic brief. No
  `S5_REVIEW_RETURN_02.md` exists. The v1 returns and S5 block return remain
  present, but neither can support landing.

- **Activation checks that pass do not cure the blockers.** Executing the
  exact first fenced shell block in the current tree returned
  `DEC087_GUARD_FALLBACK: use WORKPLAN_2026-07-17_piping_loop.md; DEC-087 prohibited`.
  Static and adversarial review confirms that untracked/absent-HEAD paths,
  dirty worktree blobs, missing exact fields, and wrong content hashes fail;
  the four required artifacts are checked at one HEAD and the bindings use
  the plan blob plus D-54/DEC-087 semantic hashes, not a circular future commit
  SHA. For the current bytes the candidate blob is
  `643a67110db0af7af09c67dad6200f9c67f0af2b`, the D-54 sections 2--3 semantic
  SHA-256 is
  `2b76c43e453d67821c3e35c3cfc5ba1b2d41e123dfbc9192a0089f37e9763b4b`,
  and the DEC-087 description SHA-256 is
  `a1f2c41b5348ba8f7afa3801e342a63702f0fd9a96d9f277b56643065cbdeb0a`.

- **Graph/effect checks pass.** `WORK_GRAPH.json` has 12 unique node IDs,
  every dependency resolves, and N4 depends on `N1_INTEGRATION`; N12 remains
  dependent on both v3 verifiers and `NOT_READY`. Graph effect is
  `HELD_CORRECTION_IN_PROGRESS`; status remains `CORRECTION_IN_PROGRESS` /
  `HELD_REVIEW_PENDING_S5`, with repeat S5 not ready, external effect false,
  no receipt, and DEL-09-04 unaffected. HEAD remains
  `756425eb53814f7a9f154fac5e2c139ef8ed5039` with an empty index.

- **Authority, boundaries, and preservation checks otherwise pass.** D-54's
  owner payload recomputes to 4,632 bytes / SHA-256
  `1cb500ce6a2e41038b1fbf4f2250ef80e650e8de8fb990658c4a68ac3e65fbc0`.
  D-54, DEC-087, and Step 2 retain live accepted-artifact grounding,
  existing-authority-only application, all four lenses, bounded
  reversibility, rationale/material rejected alternatives, independent
  refutation, agent-judgment attribution under standing approval,
  `OwnerCaseSelection=NONE`, owner-only governed-brief adoption, and all
  D-54/D-52/D-49 nondelegable boundaries. Step 3 is byte-identical at 830
  bytes / SHA-256
  `b351d3aacdecb67a595f9450f100b29bac2d39ff4ab954df0333613fc84ad1d9`;
  standing fences through EOF are byte-identical at 3,515 bytes / SHA-256
  `aad0baef3bd24d73aa81a471700564482864582c36b230c07441e10b385da86f`.
  The old plan, LOOP_INIT, receipts, D-49 through D-52, app-dev, and every
  enumerated DEL-09-04 surface show no diff from HEAD. Both Shared-Block v1
  payloads remain 5,108 bytes / SHA-256
  `76438ab0e00dc70e5f6db751a32d0ff07b681c7b7fb12eeda338157c5ebe7668`.
  Instruction-entrypoint and receipt validators passed, JSON parsed, and
  `git diff --check` passed. No refined-discretion exercise, S5 verdict,
  receipt append, DEL-09-04 action, commit, merge, push, or external effect is
  evidenced.
