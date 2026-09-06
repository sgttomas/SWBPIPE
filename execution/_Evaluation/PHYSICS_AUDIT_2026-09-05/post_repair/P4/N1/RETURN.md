# N1 bounded implementation return

Status: IMPLEMENTED; fresh independent review and integrated registered checks pending parent.

DEL-04-04 / SOW-012 / OBJ-003; repair R02/R04. Sealed parent brief N1_BRIEF_V1.md SHA e92be5975a3f24474e30fca0fa6ee5b78c79d93b06887683bf3be582dd1e0279. Root accepted S1 V3 and R1 PASS as stated in brief. Derivative repair packet; baseline audit bytes unchanged. No lifecycle or engineering acceptance claimed.

## Changes

- Gap equality now admits zero bearing reaction in the engaged state. Released-gap existing clearance equality remains unchanged. Thus exact touching converges to Active from either seed. Strict pulling reaction releases and strictly open displacement stays inactive. No contact tolerance or friction/history policy changed. The existing test misnaming zero-reaction touching as separating is corrected. One unrelated pre-existing assertion is mechanically reformatted by rustfmt.
- Public classifier checks every supplied trial numeric field before classification, including optional normal/tangential values, using the existing NonFiniteInput channel.
- Public nonlinear solve revalidates ConvergenceControl with its existing constructor and rejects duplicate nonlinear support IDs through the existing InvalidInput channel before iteration. No public types, fields or error variants added.

## Evidence

EXPECTED_BEFORE_RUN.md gives independent equilibrium calculations. New baseline regressions failed in all four targeted cases before repairs (three integration tests, one classifier test); original source copies and exact encoded logs are retained. Final full crate tests: integration 21 PASS, supports 20 PASS, no failures/ignored. Exact gap checks cover 24 combinations of sign/seed/mode/load, and coupled contacts another 16 combinations using independent two-bar equilibrium. Invalid trial matrix covers all four behavior categories, three prior states, four numeric fields, and NaN/+infinity/-infinity. Existing friction, one-way, capped nonconvergence and sparse/dense tests remain passing. rustfmt check and scoped git diff --check PASS. Build used one sequential /tmp/piping-n1-target allocation; no global build performed.

SOURCE_HASHES.json and FROZEN.patch.json bind the two source files. Raw test logs are encoded losslessly with byte SHA. scope-check.json PASS uses explicit owned paths to avoid attributing concurrent sibling work. affected-checks.json selects piping-pytest/evidence-sweep/harness-self-check for parent integrated execution; these are not claimed run here.

## Handoff

Frozen for fresh read-only review; rerun if either source hash changes. P4 later adapter handoff may modify nonlinear_integration only after parent acceptance of this packet. Friction prior-iterate behavior, inactive-seed contact-only singularity, pivot/convergence policy and other Owner-held matters remain unchanged. No Git mutations, network, user app interaction or sibling writes. No children; native Agent2 role instruction/config asserted; model unknown.
