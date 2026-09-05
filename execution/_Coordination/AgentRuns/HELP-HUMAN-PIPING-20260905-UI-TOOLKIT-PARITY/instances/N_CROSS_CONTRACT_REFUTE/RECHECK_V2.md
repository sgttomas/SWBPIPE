# PKG03 v2 design recheck

Verdict: PASS_DESIGN_ONLY. C1 is closed at the candidate contract level. No new design blocker found. Parent acceptance and implementation validation remain separate.

The v2 replacement wire projects the full existing support configuration into stale-checked change.before and replaces it atomically. Omitted top-level stiffness and nonlinear fields explicitly clear old values, the preview exposes those effects, identity/label/node remain preserved, and restraints require explicit user confirmation. New-support selection also prevents a generic stiffness override. Required regression checks the consumed DOF and stiffness value, stale rejection, repeat selection without inherited fields, and unchanged source/session/checkpoint on failure. This repairs the specific precedence failure in C1 without changing solver precedence itself.

The chosen UX/UY/UZ spelling is accepted by existing product_physics::parse_dof. The v2 invented payload carries UY with N/m and force_per_length and keeps its original quantity/provenance structure.

The evidence binding is now concrete: canonical HangerSelectionEvidence with library metadata and full record snapshot is stored in supported string fields change.source_note and support.provenance; product Quantity remains {value,unit}, and the original magnitude/unit/dimension/per-value provenance remains in the snapshot. The UI has an explicit obligation to render it intelligibly and the selection resolver must revalidate the chosen current record. This avoids unsupported model keys or a live library dependency.

The final clarification explicitly requires all seven provenance fields and disposition/quarantine checks at every numeric value. That addresses the old generic traversal limitation. The implementation must prove these checks in both runtimes; generic library behavior is outside this amendment.

This is a read-only design backcheck, not a frozen-code review, test pass, schema acceptance, lifecycle transition, or source-write grant. No tests or source edits performed. Existing v1 review records remain unchanged. Rerun for contract amendments, interface drift, implementation findings, or changed persistence semantics. Actual inherited model identifier unavailable; direct Agent2, no delegation.

## Reviewed inputs and binding verification

```json
{
  "inputs_sha256": {
    "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N4_WI_PKG03/CONTRACT_PROPOSAL_v2.md": "659a0a479af0c53bdab821ee957f48ae4976e3185f3502a92fdec91edfe44e7e",
    "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N4_WI_PKG03/INVENTED_PAYLOAD_PROPOSAL_v2.json": "3f6484ac41a49cb9af8b350abbe17bdcd9f3a3a7772d6dc1ca8365b45cc4d49d",
    "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N4_WI_PKG03/INTERFACE_BINDING_v2.json": "b9880d95e87b8c361cb234a9716a112a222c6e8757072311de7d51177d3092eb"
  },
  "binding_reference_checks": {
    "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/N2_WI_PKG16/A2_RETURN.md": {
      "expected": "22452007ee29de51d2bc4a8ca29abff3b7d1f1aa49cc55646c8be3699814c0bb",
      "actual": "22452007ee29de51d2bc4a8ca29abff3b7d1f1aa49cc55646c8be3699814c0bb",
      "matches": true
    },
    "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/N2_WI_PKG16/AMENDMENT_B2.md": {
      "expected": "82d75eab37c9acc59352ac894676d84e3622092a0abeccd25a808b87d53d8d1e",
      "actual": "82d75eab37c9acc59352ac894676d84e3622092a0abeccd25a808b87d53d8d1e",
      "matches": true
    },
    "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/N2_WI_PKG16/B2_BRIEF.md": {
      "expected": "5ba253656e60f58c166fa7c8cddd1bfa1739f6599f930d8cd002b9bc06f319fb",
      "actual": "5ba253656e60f58c166fa7c8cddd1bfa1739f6599f930d8cd002b9bc06f319fb",
      "matches": true
    }
  }
}
```
