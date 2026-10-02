# I23 VK-R02 observable24

An existing test directly checks the **current seed site**:

    structural::retained::verify::tests::e_hat_couples_force_and_moment_through_the_body_extent_in_binary64

FK verify_tests.rs43–49 imports the parent verify module, whose cfg(test) includes this file. Calls enter the actual verify.rs321 function, including active(R02) at324. Thus the FK unit-test build reaches the real environment-selected VK-R02 branch even with features=[]. No helper replica or source patch is involved.

The independent literal assertions are e_hat([1,8],2)==[4,8] and e_hat([5,8],2)==[5,10]. They follow exactly from max(1,8/2), max(8,2*1) and max(5,8/2), max(8,2*5). ActiveR02 returns the input pair. The primary numeric discriminator is line45; a failure there does not also execute/credit line46. The zero-extent control at44 is nondiscriminating by design.

Existing untouched artifact: `<wt>/a1-fk-root-baseline-target/debug/deps/open_pipe_stress_frame_kernel-716dc63302c0c904`, SHA256cd5967f30c1ca9d8957d02126d2639e943435b84f86c5d02c0b21c904af8e00a. Current binary/fingerprint match fk_baseline_03; its old stdout lists this exact test PASS. No build or execution is needed to prepare this pointer. Any later fresh NONE/VK-R02/NONE diagnostic still requires ROOT's grant.

Historical distinction: the relevant protected experiment is **G1/G27 R7-M2**, not G3. G3's52-entry manifest excludes R7-M2 and links the untouched baseline above. G27 patched the later extent guard, then failed line45 with [1,8] versus [4,8]; its control passed. That is useful prior test evidence, not a VK-R02 environment-seeded run or P49 credit. No blanket equivalence is inferred from the fault names.

**Criterion boundary remains:** the original VK-R02 register names TREE100-AX outcome/evidence, and the frozen map asks for a named R7 estimate/charge or acceptance change. The direct scalar test is a complementary numeric seed-site witness; it is not automatically TREE100/P49 evidence. Existing record JSON omits numerical verification_estimate/verification_charge; work counters cannot replace them. A final targeted search also found an existing FK TREE100 test: references_tests::rf_large_tree_ax_at_100_members_is_selected_at_128_and_honest. It checks selected128, then models::g5a reads actual public verification_estimate/verification_charge and checks literal limits0.25/1.0. Its existing baseline passed. Full correspondence of its R1_LARGE/models5a3 inputs to the original VR target was not completed; same name alone is insufficient. Earlier selection or helper checks may prevent reaching those numeric assertions.

No new test or collector is warranted yet: the direct scalar witness exists, and the existing FK TREE100 candidate already exposes fixed numeric summary limits. The remaining bounded task for that candidate is input/criterion correspondence and qualified reach, not an invented oracle. ROOT owns any next grant; neither candidate earns runtime or P49 credit here.

Raw source hashes, exact test/cfg/import chain, original registration, artifact/baseline pointers and historical separation are under `_run_records`. P49 and all tests/builds remain unrun here; S2's sealed preparation is untouched. Start23:10:43UTC; boundary23:20:43UTC. No Git/index, delegation or automatic continuation.

Completed:2026-10-01T23:20:00Z. Final target discovery and its input-correspondence limit are preserved explicitly.
