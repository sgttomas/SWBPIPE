# Owner native scroll-axis clarification

CHAT_TRANSCRIPTION — EVIDENCE, NOT RULING. Active-chat supplied question/reply, not transport bytes. Read after OWNER_NATIVE_SCROLL_CORRECTION_2026-09-20.md; preserve both prior observations.

Question:
> When the filter and family buttons moved off screen, were they moving vertically, horizontally, or both? I’m checking which container needs correction.

Owner reply:
> Vertically (up/down)

Question item: ["request_user_input_async","call_LwdiA3tgs96GYxwVpiYFDBz0",0].

Disposition: correct vertical scroll ownership in the existing B4.1 scope. Keep filter/family controls stable and let table content use the remaining space; do not misdiagnose this as failure of the repaired horizontal implicit-track fix. Read-only browser reproduction at compact1280×800 corroborates shell-tree-host scrollTop0→68, moving ModelTree/filter/families while a five-row body reserves360px for180px of rows. The wide1440×920 profile did not overflow. Exact raw diagnosis and proposed implementation remain in the owning B4 instance. The manager returns a bounded layout proposal before source edits. No VirtualList/engine/pane-budget waiver, broader table completion, C4 ruling, CLI activation, usability qualification or release follows from this observation.
