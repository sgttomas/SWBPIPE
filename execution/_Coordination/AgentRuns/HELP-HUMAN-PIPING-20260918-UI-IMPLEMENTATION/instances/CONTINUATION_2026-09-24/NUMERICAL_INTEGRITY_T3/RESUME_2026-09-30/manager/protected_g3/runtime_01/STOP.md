# R09 semantic stop

At 2026-10-01 15:50 UTC I22 reported the R09/K4-M2 test failed first on sum_targeted line2 (tail2^-133), before the frozen registration's specifically named line6 (tail2^-428). Manager independently read raw stderr: p128 terms [w:+8p0,w:+8p-128,w:+8p-133], actual +8p0 versus +80000000000000000000000000000001p0 at wide_sum_tests.rs102. This is related sticky-tail behavior but is not the registered witness; no R09 credit.

R01–R08 finished intended assertions and matching untouched controls. R09 build/list/mutant completed, control unrun. R10–R52 remain unrun. The R09 source postimage remains e671336b0dc5dc2eb71825f667bb4fe1926cf93c2f9c4d3f3e5f1518c051752e and its binary1df5cfd15fea7bf89e1e9e539058bdcb193f6af4aa29085f5520f70156556faa. Raw path <wt>/scratch/i22/protected_g3/logs/R09/mutant_01.{stdout,stderr}; exact command/argv/cwd/environment preserved at I22/protected_g3/runtime_01/R09/mutant_01/EVIDENCE.json.

Manager initially messaged that a matching control remained authorized while checking the distinction, then explicitly withdrew that suggestion at the child's stop report: the unexpected-assertion rule requires no further runtime without ROOT disposition. Child had already stopped correctly and no control or later copy ran. This mistaken manager suggestion did not become an execution.

Manager independent ps snapshot at15:50:57 found guard5387 live and no cargo/rustc/FK-test/protected_g3 descendants, excluding the snapshot query itself. The original16:23:26 boundary remains recorded; stopping does not authorize a retry, repair, extension or V-K dispatch. ROOT has been notified; sealed return follows.
