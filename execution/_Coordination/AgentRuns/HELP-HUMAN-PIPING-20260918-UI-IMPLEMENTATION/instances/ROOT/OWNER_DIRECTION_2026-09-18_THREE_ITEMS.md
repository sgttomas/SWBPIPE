# The owner's direction on three open items, 2026-09-18

Recorded by ROOT (HELP_HUMAN, Agent 0). All three texts are extracted from the stored session transcript (stored-transcript custody, not transport bytes).

## The owner's question

2026-09-18T22:18:22.335Z, SHA-256 `c1628640b61eec4bde67ee23092c4b8816cda5f1716f9127cec5598f72f831a0`, verbatim:

~~~
What do you recommend for these:
The acceptance sentence that survives in two emitted data constants, which are schema constants.
The "Evidence" domain word and the "External run evidence" wording.
Whether saved browser preferences reset under the new bundle identifier, which I have not checked.
~~~

## ROOT's recommendation, as the owner read it

2026-09-18T22:23:53.475Z, SHA-256 `055d0aeea3d663a0635a84f725d8a507cae9fb04130b9a907eb7c63eab059074`, verbatim. It is reproduced because the owner's answer refers to it. Its inventory was incomplete; see the correction below.

~~~
I recommend these four things:

- Remove the acceptance clause from both emitted notices now, in a small reviewed change.
- Keep "Evidence".
- Rename the panel to "External prover run".
- Accept a one-time preferences reset.

One correction first: I called the two surviving constants "schema constants", and that was inaccurate.

**1. The acceptance clause in emitted data**
- **Where it is.** Two engine strings carry the short clause. One is the rule-check run notice at [rule_check_runner/src/lib.rs:78](projects/chirality-piping/core/rules/rule_check_runner/src/lib.rs:78). The other is the rule-pack validation notice at [rule_pack_document/src/lib.rs:1207](projects/chirality-piping/core/rules/rule_pack_document/src/lib.rs:1207).
- **What the schema pins.** It pins neither sentence. It pins an older one, the prohibition list that DEC-081 retired. The engine stopped emitting that sentence in July, and the schema was never updated. Nothing validates a real result against it, so nothing fails.
- **Saved files are safe.** The saved-project and model schemas accept any non-empty text in that field.
- **Visibility.** Since Tranche A1 the app displays neither notice. The clause lives only inside exported and saved JSON.
- **Recommendation.** Your ruling said to remove any such mention, and I see no contract reason left to keep it. I would drop the clause and keep the rest, as below. I would also make the schema field plain non-empty text, because pinned prose is what drifted.
- **What it needs.** It touches the engine and one schema. So it needs your yes, an independent review and the full sweep. I would pair it with your new bundle identifier, which needs the same review.

```
Rule-check results are engineering decision-support information computed from user-supplied rules and data. Human review remains required.

Document validation results are engineering decision-support information computed from user-supplied rule-pack content. Human review remains required.
```

Tranche A1 deliberately kept the short clause in four documents: the README, the user guide, the build and release guide and the release notes template. If "any such mention" is literal, I would remove those in the same change, which needs no lint edit. I would leave the ScopeOfWork files.

**2. The two wordings**
- **Keep "Evidence".** The registry's own section is named for evidence-status labels, and it says these two are not authority domains. The labels already name their source. No screen draws either label yet, so this can be revisited when the Review page first shows them.
- **Rename the panel to "External prover run".** "External run evidence" does not say what ran. The PRD already defines the external prover tool, and the token is Prover correlated. The name carries no vendor.

**3. Preferences under the new identifier**
- **They reset once.** On this machine WebKit keeps local storage in one folder per bundle identifier, and the old identifier's folder exists. I did not launch a build to watch it happen.
- **What is lost.** One key holding theme, density, three panel sizes and last open tabs. It holds no engineering data.
- **Recommendation.** Accept it and write no migration. The Tranche B shell replaces the rails and dock those sizes describe. If preferences should outlive an identity change, they belong in the app's own store with the persisted view state gap.
- **Project data carries over.** The old project store exists here, so the carry-forward has real data to move.

I recorded your registration as DEC-106 and changed the identifier in the configuration and its tests, which pass. It is not committed yet. One build-readiness panel literal is still to change, and it waits on the tokens child's browser tests, which are still running. The identifier change goes in its own reviewed pull request, not the tokens one. The findings are in the run's [HANDOFF_STATE.md](projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/HANDOFF_STATE.md). I need a yes or no on items 1 and 2 before I apply them.
~~~

## The owner's answer

2026-09-18T22:29:15.816Z, SHA-256 `1da85bed178b7f2b7eb533541fb5818500673e56ec871bbb7d8b212ad443becd`, verbatim:

~~~
Yes to all three, including the four documents; carry on.
~~~

## ROOT's correction of its own inventory

ROOT told the owner that the acceptance clause survived in two emitted notices and four documents. ROOT had searched for one short variant on unwrapped lines. A search of all five registered texts with whitespace normalized (`../../tools/find_acceptance_texts.py`) finds, outside `execution/**` and ruled history:

- **three** emitted notices, not two: the rule-check run notice, the rule-pack document validation notice, and the report package's first boundary note (`core/reporting/report_package/src/lib.rs` near 600). Nothing pins the third but its own source.
- **thirteen** documents, not four: the four ROOT named (`README.md`, `docs/user_guide/index.md`, `docs/BUILD_AND_RELEASE.md`, `docs/RELEASE_NOTES_TEMPLATE.md`) and nine more (`docs/README.md`, `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, `docs/_Examples/rule_pack_notice.md`, `docs/contributor_guide/index.md`, `docs/local_analysis/local_fea_handoff_guidance.md`, `docs/security/local_first_storage_policy.md`, `docs/security/redaction_export_controls.md`, `docs/validation_manual/index.md`, `docs/validation_manual/headless_runner_reproduction.md`).
- two tests that require a document to carry the sentence: `tests/test_user_guide_status_wording.py` (the user guide) and `tests/test_local_fea_handoff_contract.py` (the local FEA handoff guidance).
- 25 of the 93 live `ScopeOfWork.md` files carry a registered text.

## ROOT's reading, labelled as ROOT's

- The owner's yes covers what ROOT described: the clause leaves the emitted notices; the run-result schema's field becomes a non-empty string; "Evidence" stays; the panel is named "External prover run"; the one-time preference reset is accepted and no migration is written; the sentence leaves the four named documents.
- ROOT put a condition to the owner, "If 'any such mention' is literal, I would remove those", and the owner answered yes. ROOT reads that as confirming the literal reading of the owner's words on D-71 item 2 ("Remove any such mention of "acceptance""), consistent with the owner's earlier correction on the maturity sentence ("in all instances of it"). ROOT therefore applies the direction to the whole class it had under-counted: all three emitted notices and all thirteen documents, with the two document-pinning assertions moved. The third notice is product output and is within `DEC-100` as already ruled; Tranche A1 missed it.
- The nine documents ROOT had not named are committed separately from the owner-named scope, so they can be reverted alone if ROOT has read too much into the answer. This reading is open to correction.
- Left as they are: the PRD, `docs/CONTRACT.md` and `docs/PROFESSIONAL_BOUNDARY.md` (`DEC-100`'s record says untouched; the last is the registry's anchor document); `docs/SPEC.md`, whose sentence is a requirement on report sections in the specification's own words and not a registered text (ROOT's reading: the same class); `docs/claims_registry.md`, which records the texts and their retirement; the live `ScopeOfWork.md` files (ROOT said it would leave them and the owner agreed); ruled history; `docs/validation_manual/cases/**`; test assertions that guard against the sentence's return.
- The claims lint is not edited. It requires the sentence nowhere, so nothing in this change needs an instruction-surface authorization.
- Codified as `DEC-107` (the acceptance text), `DEC-108` (the two wordings) and `DEC-109` (the preference reset).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
