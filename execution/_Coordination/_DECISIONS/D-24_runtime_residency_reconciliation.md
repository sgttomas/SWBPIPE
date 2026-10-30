---
doc_id: OPS-D-24
doc_kind: governance.decision_packet
status: RULED (substance, 2026-06-21); final wording presented for owner confirmation; CHANGE to publish
created: 2026-06-21
persona: SOFTWARE_DECOMP (Normative governance)
working_root: projects/chirality-piping
refs:
  - rel: reconciles_to
    to: "_DomainEngines/_DECISIONS/D-T0-04_data_residency.md (tier-0 owner ruling)"
  - rel: amends_runtime_clause_of
    to: DEC-017
  - rel: preserves
    to: "OPS-K-IP-1/2/3; IP_AND_DATA_BOUNDARY §§3,5,7,8; PROFESSIONAL_BOUNDARY; OPS-K-PRIV-2 (telemetry)"
---

# D-24 — Runtime data-residency / agent-visibility / model-provider reconciliation to tier-0 `D-T0-04`

**State:** `RULED` (substance) by the owner on 2026-06-21 (four rulings below). Final wording revised to match; presented for owner confirmation before CHANGE publishes.
**Proposed on publish:** `DEC-051` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 + a `D-24` row in `_REGISTER.md` → RULED; CHANGE applies the four diffs below and binds them to a publish SHA (K-AUTH-2).
**Authority discipline:** Agents draft; **only the human project authority rules**; CHANGE publishes. Nothing here is self-applied. No approval/SHA is claimed that is not held.

---

## Rulings (owner, 2026-06-21)

| # | Decision | Ruling |
|---|---|---|
| 1 | Reconciliation basis | **ACCEPTED** — runtime residency / agent-visibility / provider relaxed; public-commit / third-party-IP boundary + professional/APEGA ceiling preserved unchanged. |
| 2 | Egress guard posture | **FULLY LITERAL OPEN** — once the owner configures a model provider, private model/Class-B data may be transmitted to it with **no app-side guard, opt-in gate, or indicator**. The "surface, don't enforce" recommendation was reviewed and **not adopted**; the accidental-egress risk (§5c) is owner-accepted. |
| 3 | Telemetry scope | **TELEMETRY STAYS OFF** — off-by-default preserved as a guarantee; the relaxation covers only the agent / model-provider channel, not telemetry. |
| 4 | Conforming follow-on timing | **HOLD** — secondary-doc edits drafted only after the core wording is accepted. |

---

## 0. Source verification (done cold, before trusting the task prompt)

| Check | Result |
|---|---|
| Cited SHA `6e70b5aace4a3a7c4ebb20490a3bf57bfd912f45` exists | YES — `commit`, author `Ryan Tufts`, 2026-06-21, subject "domain-engine: tier-0 OpenPipeStress bridge prep + owner rulings". |
| Relation to `main`/HEAD | It **is** HEAD of `main`; working tree matches (empty diff on the two ruling files). |
| `_DomainEngines/_DECISIONS/D-T0-04_data_residency.md` | Present; ruling reads identically in the stub, `RULINGS_PUBLISHED.md`, and `_DECISIONS/_REGISTER.md`. |
| `_DomainEngines/RULINGS_PUBLISHED.md` `RES-RECONCILE` section | Present; **explicitly assigns** piping's `OPS-K-PRIV-1` + no-required-network + `IP_AND_DATA_BOUNDARY` to the **piping loop** to reconcile; conflicting-on-record until done (K-CONFLICT-1). |

**Owner's verbatim ruling (D-T0-04, 2026-06-21):** *"It doesn't concern me that the agent sees the private model, for now. The app shouldn't try to enforce privacy either. Local models or Anthropic models or even other providers could all be used."* — framed *"for now"* (current, revisable stance).

**Tier-0 ruling SHA note:** the tier-0 record's own `Ruling SHA` reads `TBD (binds at CHANGE publish)` per K-AUTH-2. Artifacts are *recorded* at commit `6e70b5aac…`; that is the verified source commit cited throughout.

---

## 1. What is being reconciled (and what is NOT)

**Relaxed — runtime residency / agent-visibility / provider posture only (Ruling 1 + 2):**
- An embedded agent worker MAY read the owner's **own** private model and private Class-B inputs (allowables, SIFs, design basis) at runtime.
- Once the owner configures a **selected model provider** (local, Anthropic, or other), private model/Class-B data may be transmitted to it — **with no app-side guard, opt-in gate, or indicator** (Ruling 2: fully literal open).
- The app does **not enforce** privacy/residency on this channel.

**Preserved — explicitly out of scope (the owner cannot waive these by configuration):**
- The **public-commit / third-party-IP boundary**: `OPS-K-IP-1/2/3`, `IP_AND_DATA_BOUNDARY` §§1–5, 7–8 (ASME/B31J/B16/B36/MSS/ISO/EN/CSA text, tables, SIF/flexibility tables, allowable tables, proprietary catalogs), the quarantine rule. Third-party copyrighted standards remain barred from the public repo and from redistribution **regardless of this ruling**.
- The **professional / code-compliance boundary**: `PROFESSIONAL_BOUNDARY`, the APEGA professional ceiling, `OPS-K-AUTH-1`, `OPS-K-MECH-2`.
- **Telemetry** (`OPS-K-PRIV-2`): off by default, preserved as a guarantee (Ruling 3) — the relaxation does **not** touch the telemetry channel.
- The deterministic storage substrate from `DEC-017` (SQLite local store, canonical-JSON truth, no direct adapter/plugin SQL) is **unchanged**; only its "no required network / cloud sync" clause is partially amended (network becomes owner-configurable/optional; the app still requires no network for core operation).

---

## 2. Proposed `DEC-051` entry (DEC-04x style) — to be inserted in §12 by CHANGE on publish

> |DEC-051|D-T0-04 reconciliation (`RES-RECONCILE`) — runtime data-residency / agent-visibility / model-provider posture: adopt the tier-0 owner ruling `D-T0-04` "OPEN RESIDENCY" (`{REPO_ROOT}/_DomainEngines/_DECISIONS/D-T0-04_data_residency.md`; published `_DomainEngines/RULINGS_PUBLISHED.md` §`RES-RECONCILE`; recorded at commit `6e70b5aace4a3a7c4ebb20490a3bf57bfd912f45`, branch `main`) into the OpenPipeStress **runtime** posture. An embedded agent worker may read the owner's own private model and private Class-B inputs (allowables, SIFs, design basis) at runtime; once the owner configures a model provider (local / Anthropic / other), that data may be transmitted to it with **no app-side guard, opt-in gate, or indicator** (owner ruling 2026-06-21: **fully literal open** — the "surface, don't enforce" option was reviewed and declined; the accidental-egress risk was surfaced and is owner-accepted). The app does not enforce privacy/residency on this channel. This relaxes the **runtime** network/egress posture only: amends `OPS-K-PRIV-1` (`docs/CONTRACT.md:41`) and the SPEC §4.4 no-required-network posture (`docs/SPEC.md:376-381`), clarifies `OPS-K-PRIV-2` (`docs/CONTRACT.md:42`) as telemetry-only (substantively preserved), adds `IP_AND_DATA_BOUNDARY` §6.1, and partially amends `DEC-017`'s "no required network / cloud sync" clause (the SQLite local store, canonical-JSON truth, and no-direct-adapter-SQL substrate are unchanged). **Preserved unchanged (explicitly out of scope):** the public-commit / third-party-IP boundary — `OPS-K-IP-1/2/3`, `IP_AND_DATA_BOUNDARY` §§1–5/7–8, `PROFESSIONAL_BOUNDARY`, the APEGA professional ceiling, and telemetry off by default (`OPS-K-PRIV-2`). **The owner cannot waive third parties' copyright by configuration** (open residency concerns the owner's own data). Owner framed the stance "for now" (revisable; revisiting is a human decision). **Conforming follow-on (surfaced per K-CONFLICT-1, HELD per ruling — not applied here):** `PRD §18.1/§18.2`, `docs/PLAN.md:188`, `docs/DIRECTIVE.md:91`, `docs/security/telemetry_policy.md`, `docs/security/redaction_export_controls.md`, and `docs/RELEASE_NOTES_TEMPLATE.md` carry local-first/no-network/private-by-default wording to be reconciled in a separate pass after this entry is accepted.|Tier-0 owner ruling `D-T0-04` (open residency, 2026-06-21) and its `RES-RECONCILE` follow-on assigning piping's `OPS-K-PRIV-1` + no-required-network + `IP_AND_DATA_BOUNDARY` to the piping loop; verified cold at commit `6e70b5aac…` (HEAD of `main`). Owner's words (2026-06-21): "It doesn't concern me that the agent sees the private model, for now. The app shouldn't try to enforce privacy either. Local models or Anthropic models or even other providers could all be used." Four refining rulings recorded 2026-06-21 (basis accepted; egress fully-literal-open; telemetry preserved; follow-on held). Doc-only governance prep consistent with `DEC-042`; `D-21` remains held.|**RULED (substance) by the owner 2026-06-21; final wording presented for confirmation; CHANGE to publish.** On publish: register row `D-24` → RULED with this entry as the pointer; CHANGE applies the four diffs in `D-24_runtime_residency_reconciliation.md` + this entry + the register row and binds them to a publish SHA (K-AUTH-2). No lifecycle, release-readiness, professional approval, certification, sealing, authentication, or code-compliance claim is created; live agent binding remains gated by the four asymmetric conditions (`D-21` held, `DEC-041` automation condition, app-dev fence F3, tier-0 adoption). Residency is no longer a separate blocker (per `RULINGS_PUBLISHED.md`).|

---

## 3. Proposed `_REGISTER.md` row (append to the table) — to be applied by CHANGE on publish

> | D-24 | Runtime data-residency / agent-visibility / model-provider posture reconciliation to tier-0 `D-T0-04` (OPEN RESIDENCY): relax the **runtime** `OPS-K-PRIV-1` + SPEC §4.4 no-required-network posture to permit owner-configured agent visibility of the owner's own private model / Class-B inputs and networked model providers (fully literal open; no app-side egress guard), preserving the public-commit / third-party-IP boundary, the professional ceiling, and telemetry-off | Embedded-agent live binding (with `D-21`, `DEC-041`, app-dev F3); `RES-RECONCILE` closure | RULED | [D-24_runtime_residency_reconciliation.md](D-24_runtime_residency_reconciliation.md) (2026-06-21) | `DEC-051` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (on CHANGE publish) |

---

## 4. Reviewable doc-edit diffs (BEFORE / AFTER — NOT applied) — revised per the rulings

### Diff 1 — `docs/CONTRACT.md:41` (`OPS-K-PRIV-1`)

**BEFORE**
```
|OPS-K-PRIV-1|Private project, material, component, and rule-pack data must not be transmitted or committed publicly by default.|Storage policy; telemetry config; CI checks|
```
**AFTER**
```
|OPS-K-PRIV-1|Private project, material, component, and rule-pack data must not be committed to the public repository. Per `D-T0-04` (open residency) / `DEC-051`, the app does not enforce data residency on the agent / model-provider channel: when the owner configures a model provider (local, Anthropic, or other), private project/model and Class-B data may be transmitted to it, with no further app-side guard, opt-in gate, or indicator. The public-commit and third-party-IP prohibitions (`OPS-K-IP-1/2/3`, `IP_AND_DATA_BOUNDARY`) and telemetry-off-by-default (`OPS-K-PRIV-2`) are unaffected.|Public-commit CI checks; user-controlled provider configuration; review|
```

### Diff 2 — `docs/CONTRACT.md:42` (`OPS-K-PRIV-2`) — telemetry preserved; clarifying pointer only

**BEFORE**
```
|OPS-K-PRIV-2|Telemetry is off by default and cannot include private engineering/code data.|Security review; telemetry tests|
```
**AFTER**
```
|OPS-K-PRIV-2|Telemetry is off by default and cannot include private engineering/code data. (This telemetry guarantee is preserved under `D-T0-04`/`DEC-051`; the separate agent / model-provider channel is governed by `OPS-K-PRIV-1`, not this invariant.)|Security review; telemetry tests|
```

### Diff 3 — `docs/SPEC.md:376-381` (§4.4 storage posture)

**BEFORE**
```
The MVP storage posture is local-only and offline-capable, with no hosted
database, daemon, required network, cloud sync, or telemetry path. Operating
system roots, application data directories, product and DB migration mechanics,
portable export/copy workflows, encryption, key management, secret storage,
cloud exception workflows, and release packaging remain separate `TBD`
decisions.
```
**AFTER**
```
The MVP storage posture is local-first and offline-capable: core model
authoring, solve, rule-check, and reporting require no network, and there is no
hosted database, daemon, cloud sync, or telemetry path by default. Per
`D-T0-04` (open residency) / `DEC-051`, the app does not enforce data
residency: the owner may configure a networked model provider (local,
Anthropic, or other) for an embedded agent, and once configured the agent may
transmit the owner's own private model and Class-B data to that provider with
no further app-side guard. Such transmission is never a public commit
(`OPS-K-IP-1/2/3` unaffected) and is distinct from telemetry (off by default,
`OPS-K-PRIV-2`). Operating system roots, application data directories, product
and DB migration mechanics, portable export/copy workflows, encryption, key
management, secret storage, provider/egress configuration surfaces, cloud
exception workflows, and release packaging remain separate `TBD` decisions.
```

### Diff 4 — `docs/IP_AND_DATA_BOUNDARY.md` §6 (ADD a §6.1; existing §6 text unchanged)

**BEFORE** (insert point: immediately after the current §6 final paragraph, before `## 7. Report boundary`)
```
Importers, examples, reports, telemetry, and issue templates must default to excluding private project, material, component, rule-pack, owner-standard, and company design-basis data unless a user intentionally exports or contributes it with documented redistribution rights.

## 7. Report boundary
```
**AFTER**
```
Importers, examples, reports, telemetry, and issue templates must default to excluding private project, material, component, rule-pack, owner-standard, and company design-basis data unless a user intentionally exports or contributes it with documented redistribution rights.

### 6.1 Runtime residency vs public redistribution (per `D-T0-04` / `DEC-051`)

The default-exclusion rule above governs *public redistribution and committed/exported artifacts* — it is unchanged. It is distinct from *runtime data residency*. Per the owner's tier-0 ruling `D-T0-04` (open residency), an embedded agent worker MAY read the owner's own private model and private Class-B inputs (allowables, SIFs, design basis) at runtime, and once the owner configures a model provider (local, Anthropic, or other) that data MAY be transmitted to it; the app does not enforce privacy/residency on this channel. This is owner-configured runtime handling of the owner's *own* data — it is NOT a public commit, NOT redistribution, and creates NO exception to §3 (public repository must not contain), §5 (quarantine), or the third-party protected-content prohibitions (`OPS-K-IP-1/2/3`). Third-party copyrighted standards content remains barred from the public repository regardless of this ruling; the owner cannot waive third parties' rights by configuration.

## 7. Report boundary
```

---

## 5. Implications note

### 5a. What changes at runtime
- An embedded agent may read the owner's own private model JSON + private Class-B inputs (allowables, SIFs, design basis), and once a provider is configured, send them to it.
- The app **does not enforce** privacy/residency on the agent/provider channel and adds **no opt-in gate, indicator, or default-local guard** (Ruling 2: fully literal open).
- Provider choice (local / Anthropic / other) is owner-configurable. Network is an optional, owner-configured path; the app remains fully functional offline (no *required* network for core operation).
- **Telemetry is unchanged:** off by default, not a private-data egress path (Ruling 3).

### 5b. What stays protected (unchanged)
- **Public-commit boundary:** third-party copyrighted standards (ASME/B31J/B16/B36/MSS/ISO/EN/CSA text, tables, SIF/flexibility tables, allowable tables, proprietary catalogs) — never committed to the public repo or redistributed. `OPS-K-IP-1/2/3`, `IP_AND_DATA_BOUNDARY` §§3,5,7,8 intact.
- The owner **cannot waive third parties' copyright** by configuration. Open residency is about the owner's *own* data.
- **Professional / code-compliance boundary:** `PROFESSIONAL_BOUNDARY`, APEGA ceiling, `OPS-K-AUTH-1`, `OPS-K-MECH-2` — no certification/sealing/approval claims.
- **Telemetry** off by default (`OPS-K-PRIV-2`).
- The deterministic substrate (SQLite local store, canonical-JSON truth, no direct adapter SQL) from `DEC-017`.

### 5c. Security implications of private data reaching a networked provider (surfaced; owner-accepted under Ruling 2)
- **Data leaves the device under third-party terms.** Sending the private model + allowables/SIFs/design basis to a networked LLM API places that proprietary engineering data under the provider's data-handling, retention, and training-use terms — outside the app's control. Default API terms vs zero-retention/enterprise terms differ materially; provider selection IS the security posture.
- **Possible conflict with the owner's *own* third-party obligations.** If private Class-B inputs include client-confidential or NDA-bound design bases, owner-configured egress could collide with the owner's confidentiality/contractual duties to *those* third parties. The ruling waives the **app's** enforcement, not the **owner's** obligations.
- **No app-side safety net (per Ruling 2).** With fully-literal-open and no indicator/opt-in, the only protection against accidental egress of private data is the owner's own configuration discipline. Selecting a networked provider is, in effect, consent for everything the agent reads to leave the device.
- **APEGA / professional angle.** The engineer remains responsible for safeguarding client data; "the app doesn't enforce privacy" places that responsibility wholly on owner configuration discipline.

**Disposition:** The "surface, don't enforce" recommendation (default local / visible indicator / explicit opt-in) was presented and **declined** by the owner in favor of fully-literal-open; the accidental-egress risk above is recorded as **surfaced and owner-accepted**. *(Optional, non-binding: a one-time informational note at provider-configuration time would document this boundary without enforcing it — offered, not required, consistent with the literal-open ruling.)*

---

## 6. CHANGE handoff (for publish on the owner's go — not executed here)

**Created by this packet (PROPOSAL → RULED-substance, in piping write scope):**
- `execution/_Coordination/_DECISIONS/D-24_runtime_residency_reconciliation.md` (this file)

**To apply on the owner's publish go (the four diffs above + the records):**
- `docs/CONTRACT.md` — `OPS-K-PRIV-1` (line 41), `OPS-K-PRIV-2` (line 42)
- `docs/SPEC.md` — §4.4 (lines 376–381)
- `docs/IP_AND_DATA_BOUNDARY.md` — add §6.1
- `execution/_Decomposition/SOFTWARE_DECOMP.md` — §12 `DEC-051`
- `execution/_Coordination/_DECISIONS/_REGISTER.md` — append the `D-24` row (state RULED)

**Conforming follow-on set (HELD per Ruling 4 — separate pass after publish):**
- `docs/PRD.md` §18.1/§18.2; `docs/PLAN.md:188`; `docs/DIRECTIVE.md:91`; `docs/security/telemetry_policy.md`; `docs/security/redaction_export_controls.md`; `docs/RELEASE_NOTES_TEMPLATE.md`.

**Proposed commit note (PROPOSAL — CHANGE to finalize):**
```
piping(governance): reconcile runtime residency to tier-0 D-T0-04 (DEC-051)

Adopt the owner's tier-0 OPEN RESIDENCY ruling (D-T0-04, RES-RECONCILE,
recorded at 6e70b5aac…) into the OpenPipeStress runtime posture: relax
OPS-K-PRIV-1 + SPEC §4.4 no-required-network to permit owner-configured
agent visibility of the owner's own private model/Class-B inputs and
networked model providers (fully literal open; no app-side egress guard).
Public-commit / third-party-IP boundary (OPS-K-IP-1/2/3,
IP_AND_DATA_BOUNDARY §§3/5/7/8), the professional ceiling, and
telemetry-off (OPS-K-PRIV-2) are preserved. No live agent binding
(D-21 held).
```

**Discipline reminders for CHANGE:** stage only the explicit piping paths listed above; never `git add -A`; do **not** edit `{REPO_ROOT}/_DomainEngines/` or the app-dev subtree; behind-guard the push; commit/push only on the human's instruction. No release/lifecycle/professional/compliance claim is created.
