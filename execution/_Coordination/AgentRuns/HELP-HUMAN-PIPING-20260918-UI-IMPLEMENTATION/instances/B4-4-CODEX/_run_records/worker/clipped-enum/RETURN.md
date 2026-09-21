# Clipped enum diagnostic return

Retained TASK gpt-6-astra/low, no descendants. Diagnosis-only production40e833; test checkpoint1cef35b6456649cd7e89de2a377e22d4c1205e24. Exactly one chromium-desktop test collected and run under canonical lock/5174, pinned Chromium, one worker, no retry. Failed the intended option-hitability oracle in2.9s. No production repair performed.

Observed real wheel scrollTop0→90. Owning input/anchor moved from y339.375–374.375 to249.375–284.375; body begins339.375. Editor/body intersection35→0 and layer clip inset changed from bottom469.625 to559.625. Popup nevertheless moved to y284.375–316.375 with visibility visible, and option center hit-testing remained true over the header area. Screenshot, raw trace/report and decoded observation.json retain evidence.

Exact draft pi and focused input remained unchanged across decisive observation; undo disabled and edited markers0. Intentional external filter focus then closed popup, kept pi without completion/Apply, and stayed externally focused. Cancel cleanup plus visible export established equal canonical model hashes and no history. Those assertions passed before intended failure. Port5174 listener absent and lock released after exit1.

Cause (high confidence): persistent editor positioning clips its layer to the table body, while useEnumEditor.position reads the raw input rect and ignores its clipping intersection; hidden/inert/style visibility checks do not detect clip-path occlusion. The detached portal therefore follows a geometrically live but fully clipped input.

Smallest recommended repair: give popup availability/placement an explicit intersection with its owning table body/grid viewport (both horizontal and vertical), closing passively when the editor has no visible intersection. Prefer sharing the existing table placement visibility result or checking actual body/grid rectangles in the enum helper; do not move input, mutate draft, Apply, or refocus on close. Retain the real-wheel failure as backcheck; add partial clipping/resize and horizontal-edge coverage if the chosen implementation changes those cases. Mere CSS visibility checks cannot fix this measured condition. No product repair is authorized in this diagnosis return. Native behavior remains untested.

Test source frozen and maintained writing ownership returned to parent. Separate Section quantity metadata P2 remains deferred for its own addendum.
