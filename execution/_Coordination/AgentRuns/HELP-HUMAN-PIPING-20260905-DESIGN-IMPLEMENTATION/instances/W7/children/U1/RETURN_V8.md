# U1 return V8 — diagnostic drawer panel sizing

Status: AUTHOR_CANDIDATE_FROZEN. Agent2 TASK; no delegation; actual model unknown. BRIEF_AMENDMENT_V8.md supersedes earlier write authority for this turn.

Changed only apps/desktop/src/styles.css: `.issues-drawer > .panel { flex-shrink: 0; }` beside existing workspace drawer styles. Direct diagnostic/missing-data panels keep their content height instead of shrinking and clipping ordinary pointer targets; the existing outer drawer remains bounded and owns scrolling. No header, global panel, z-index, pointer-event or other CSS change.

SOURCE_FREEZE_V8.json records the exact resulting style SHA256. No test, build or browser execution by U1. V7 ordinary-click test remains unchanged. Previous evidence packets unchanged.

W7 owns targeted diagnostic checks in two viewports, full source28/dist3, desktop748/build, and fresh review of combined three-file delta. Product change invalidates earlier native/source continuity; no new native or verification success claimed by child. Source ownership released to manager.
