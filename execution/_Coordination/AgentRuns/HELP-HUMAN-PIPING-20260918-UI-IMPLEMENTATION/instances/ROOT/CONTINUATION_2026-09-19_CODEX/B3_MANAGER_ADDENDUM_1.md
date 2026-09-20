# B3 manager addendum 1 — local integration and narrow coverage

Issued by ROOT under the approved strategy. This clarifies the sealed
B3_MANAGER_BRIEF.md; its original bytes/hash remain historical at c805bcc1a.

1. The phrase "no push/PR/merge" refers to final publication/integration into
   main. Local upstream merges into the shell lane, and local integration of
   child work within scope, are authorized. The brief explicitly requires
   merging origin/main at a clean point; the manager's completed clean merge
   a63607e5c6d805f364c969eb6963f768baefb300 is within that authority. ROOT owns
   push, PR creation and final merge to main. No force operation is authorized.
2. Changing the native minimum to 1280 by 800 must not silently eliminate
   below-1280 browser robustness scenarios. Inspect tests whose viewport matrix
   derives from Tauri config; retain the existing narrow scenario coverage and
   distinguish native minimum verification from browser fallback verification.

No product decision or tolerance changes beyond the adopted package.
