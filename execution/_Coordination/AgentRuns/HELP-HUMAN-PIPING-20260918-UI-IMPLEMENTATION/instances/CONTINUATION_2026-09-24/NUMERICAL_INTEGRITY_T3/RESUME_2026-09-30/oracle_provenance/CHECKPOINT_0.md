# A1 oracle provenance checkpoint — stopped before derivation

Status: provenance-only return. **No independent-oracle acceptance, oracle code,
derived result, solver witness, or mathematical closure is supplied.** ROOT
stopped this assignment after disclosure of premature expected-field exposure.

## Assignment and actual scope

This was native `collaboration.spawn_agent` TASK `/root/a1_exact_oracle`, parent
`/root`, under the A1_ORACLE and COMMON briefs at coordination commit
`3446bdf51d90b8f1812d161cc638d85c97bbef16`. Product checkout HEAD was read as
`d01ad98a754698631f927709d08284c272de85e8`. The response-material basis was
`520d7dfb790bcedabc03e92b9692884ce295be54`.

The initial write grant was `R/oracle/**` and owned scratch. ROOT's subsequent
STOP direction superseded that with this provenance-only `R/oracle_provenance/**`
grant. These were prompt restrictions on a shared host. No model-diversity claim
is made. No delegation, Rust build, solver execution, Git/index mutation,
maintained-source edit, host-tool change, or old-result comparison was performed.

`P = projects/chirality-piping`.
`T3 = P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3`.
`R = T3/RESUME_2026-09-30`.
`RESPONSE = P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260930-T3-AUDIT-RESPONSE`.

## Read order and exposure

1. Read Root `AGENTS.md`, `agents/AGENT_TASK.md`, and `P/AGENTS.md` from the
   coordination commit, followed by `R/BRIEFS/A1_ORACLE.md`, `COMMON.md`, and
   `R/PLAN.md` at that commit.
2. Read `T3/AUDIT/AUDIT_RESPONSE_SCOPE_HANDOFF_2026-09-30.md` from the response
   commit before response material; read `T3/AUDIT/A1_A2_REVIEW.md` from the
   coordination commit. Listed relevant filenames using read-only Git queries.
3. One shell read concatenated three `git show` calls from the response commit,
   in this order: `RESPONSE/instances/A1-DIAGNOSIS/src/cases.rs`, `src/main.rs`,
   then **unfiltered `matrix.json`**. The last command was:

   ```sh
   git show 520d7dfb790bcedabc03e92b9692884ce295be54:projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260930-T3-AUDIT-RESPONSE/instances/A1-DIAGNOSIS/matrix.json
   ```

   This was tool output chunk `805643`. It reported 86,345 original output
   tokens and truncation. The visible response included `expected_rows`, truth,
   rounded-truth bits, and rounding-outcome fields: B01 zero-valued rows and
   later matrix tail blocks including C24. I cannot assert which additional
   middle fields were visible through truncation. **This happened before an
   independent derivation was written or frozen.** Therefore the required
   ground-expectations-before-truth-fields order was not satisfied.
4. I immediately disclosed the exposure to ROOT. No response `oracle.py` was
   read/imported and no B01/B02 run comparison was read. The earlier scope
   handoff's high-level B01/B02 result summary had been read as required.
5. Before ROOT's stop message arrived, subsequent reads inspected selected
   contract/source-definition material: `retained/recover.rs`,
   `retained/adaptive.rs`, `T3/ROOT_RULINGS_V1.md`, and
   `T3/DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md`,
   plus filename/text searches. These do not restore the failed blind-order
   control. No derivation or oracle file was written.

## Unverified suggestion already sent to ROOT

I sent an interface suggestion requesting primitive bits, unsummed load terms,
canonical row identity/kind/input-derived flag, candidate outcome/value bits,
bound bits, selected precision, published scales, and applicable floors. I also
suggested claim formulas and O9 membership treatment. **That message is an
unverified suggestion, not a contract finding.** In particular, my suggested
relative-claim denominator used exact truth; ROOT identified a conflict with the
preserved command plan's published-value denominator. I did not resolve that
conflict after the stop. The fresh oracle must derive its predicate from the
accepted design/source independently.

## Write inventory and return

Before this checkpoint, **no files had been written** in either the product
checkout or scratch. This checkpoint is the only file written by this TASK.
No existing file was deleted or altered. The original oracle request is
incomplete and has been returned to ROOT for a fresh assignment with unfiltered
`matrix.json` excluded until its derivation is frozen.

The tool/chat record carries the actual host context and commands. This durable
record deliberately uses repository-relative aliases. Its checksum is returned
to ROOT in the final chat; no sealed numerical packet is claimed.
