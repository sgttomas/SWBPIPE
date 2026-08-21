# Sealed instance launch brief — N3 CI fragility hardening

- RequestedBy: `HELP_HUMAN`
- ParentInstanceID: `HELP_HUMAN`
- ExecutorInstanceID: `WORKING-ITEMS-CI-HARDENING`
- Role: `WORKING_ITEMS` (Agent 1)
- Node: `N3`
- Model: current `GPT-5.6` Codex runtime; no substitution authorized.
- Basis: branch `codex/piping-sca009-ci-support-20260821` at `b1876a5e0f0083e10c0c18255cd92ed0079b63a2`; observed 2026-08-21 slow-object failures are accepted defect evidence and must not be reproduced.
- Objective: prevent Playwright GitCommitInfo collector merge-base fetches from crashing the desktop E2E lane, choosing and explaining either disabled git-info capture or explicit merge-base prefetch; and make governance-harness Candidate whitespace lazy-fetch hangs fail fast with a step timeout and legible diagnosis.
- DeclaredReads: `.github/workflows/piping-desktop-e2e.yml`; desktop Playwright source/dist configs; root governance-harness workflow; G4 instruction-surface manifest conventions; policy tests pinning touched workflow text.
- AllowedWriteTargets: `.github/workflows/piping-desktop-e2e.yml` only if the selected fix needs it; `projects/chirality-piping/apps/desktop/playwright.config.ts`; corresponding dist config if CI executes it; the root governance-harness workflow explicitly authorized by the owner; required G4 tranche manifest(s); directly coupled policy tests; node-local records in this AgentRuns root.
- Acceptance: no remote merge-base fetch by the Playwright collector during CI startup; Candidate whitespace step has a bounded `timeout-minutes` and legible failure path; all touched instruction surfaces have same-tranche G4 manifests; coupled policy tests pass; no reproduction/retry burn.
- EXCLUSIONS: no product behavior or Node 1/2/4 paths; no slow-serving reproduction; no unsigned-artifact lane; no `artifact-proof` label; no parked holds; no commit/push/PR.
- Return: chosen designs and reasons, files, focused checks, manifests, blockers, and fan-in evidence.
