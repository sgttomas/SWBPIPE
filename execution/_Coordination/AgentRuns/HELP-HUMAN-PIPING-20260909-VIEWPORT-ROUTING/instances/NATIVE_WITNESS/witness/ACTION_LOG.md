# Native action record

1. Verified the sealed brief and all candidate record hashes, PID `23199`, exact executable path/hash, exact Info.plist path/hash/identifier, all ten source hashes, and the exact dedicated empty store before any UI write. The normal store was not inspected.
2. Attached to the running macOS application with native CUA. No browser was opened or substituted.
3. Used **Create local** to persist the invented preview model into the dedicated store. Read-only SQLite inspection showed 5 nodes and 4 pipes, with no witness IDs.
4. Armed **Pipe**, resolved From to elevated `node:N-140`, selected **New node**, retained XZ, selected X, and selected endpoint unit `mm`. Native feedback read `Construction plane: XZ · Y=2.4 m · through node:N-140`; Y was disabled and X/Z were applicable.
5. Performed an orbit drag. Native status read `Pointer movement exceeded 4 CSS pixels; no selection or coordinate was captured.` No endpoint fields or model entities were authored.
6. Recorded an attempted click at a blank canvas location. Native status read `The pointer ray has no finite intersection with the selected construction plane.` The ghost cleared and no coordinates or model entities were authored. This expected guard result was retained rather than hidden.
7. Created a finite constrained hover ghost and captured through the canvas in `mm`. The main applied route capture yielded endpoint fields `(2928.619, 2400, 2200) mm`, with captured ghost `(2.929, 2.4, 2.2) m` at displayed precision.
8. Entered stable IDs `node:WIT-N-150` and `pipe:WIT-P-140`, labels, invented provenance, material, `168 mm` OD, `7 mm` wall, y-reference `(0,1,0)`, and enabled continuation.
9. Clicked **Add route**. Native validation passed and froze atomic operations `op:viewport-create-node-node:WIT-N-150-001` then `op:viewport-connect-pipe-pipe:WIT-P-140-001`, with validated model hash `sha256:b2cec8ee447c44fa7efbdf43c0970a5c690b8c63e24577a105ce12e7f613ccc3`. The native model remained at 26 entities and the store remained at 5 nodes/4 pipes.
10. Clicked **Apply**. The native model moved directly to 28 entities and exposed exactly `node:WIT-N-150` and `pipe:WIT-P-140`. No one-entity partial state was observed.
11. Verified continuation moved From to `node:WIT-N-150`; material, dimensions, orientation, pipe provenance, XZ, and X persisted; consumed IDs, labels, endpoint coordinates, and endpoint provenance cleared.
12. Selected the new node and pipe in the native model tree. Node properties showed `x=2.9286190000000003 m, y=2.4 m, z=2.2 m`; pipe properties showed exact connectivity `node:N-140 → node:WIT-N-150`, `168 mm`, `7 mm`, and the invented material.
13. Confirmed the store still held 5 nodes/4 pipes before Save. Clicked **Save local** exactly once. The store then held 6 nodes/5 pipes and the exact node/pipe JSON recorded in `BEFORE_AFTER.json`.
14. Quit the app with Command-Q. The next native CUA observation relaunched the same bundle as PID `30116`; **Open local** reported `Opened local SQLite project snapshot.` The native tree returned 28 entities with both exact IDs and exact coordinates/connectivity.
15. Performed a post-reopen stable-pointer probe at unchanged pointer/camera state: visible hover `(5.66,2.4,2.2) m` captured as `(5659.782,2400,2200) mm`, and the captured ghost remained `(5.66,2.4,2.2) m`. This probe was canceled without Add, Apply, or Save.

CUA-generated screenshots were recovered byte-for-byte from this run's local Codex session record into `screenshots/`. One discarded system-screen capture attempt saw an unrelated foreground application instead of the CUA surface; the two exact generated PNGs were immediately unlinked and are absent from the evidence package.
