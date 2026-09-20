# Canonical peer MCP compatibility evidence

One canonical packet is owned by the Runtime lead:
projects/chirality-runtime/execution/_Coordination/AgentRuns/MCP_V2_COMPATIBILITY_20260920/

Reviewed source commit:9ecbdecdfc9bb07404fb488836a9c8cf1c40117d.
Merged PR:https://github.com/sgttomas/chirality/pull/827
Observed origin/main merge:55932683916fd1cd0ca047a0a76c32adaf9b65ab.
Only eight Runtime evidence files changed from Piping's prior main88c6c746; no Piping or shared maintained source changed.

- README.md SHA256eccc28401d80ef01547a593827397b91127708e4264018f08a12f4dbbdb06d84.
- result.json SHA256efe9f5f9084ea8cc9826c1ece3327baff886466d9d3dcd0a499c2f158b7da418.
- mcp-wire.jsonl SHA2569e214cc66a68103c3c6ef1b5d8945372091aeee5c1c9eed0fe462ff8005da9a8.

ROOT and the independent Piping reviewer both verified exact result/wire bytes from the original scratch location, then ROOT verified these hashes through git show at the canonical source commit. No probe was repeated or packet copied into Piping.

The actual desktop-bundled Codex0.155.0-alpha.9.2 sent initialize with2025-06-18. The strict2026-07-28 server rejected it; no modern request, tool call, reconnect, isolation or cancellation check was reached. This establishes incompatibility of that tested configuration, not every version/configuration/transport and not hot attachment to this running task.

The peer reports fresh independent evidence review PASS and required hosted checks passed before normal merge. Piping's bounded records reviewer independently checked the raw empirical interpretation. Provider configuration names or App Server dynamicTools do not establish MCP V2 support. This evidence changes no product behavior or live-binding authority.

