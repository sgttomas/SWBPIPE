# Native V4 delivery-path clarification

The active FINAL_NATIVE_LEASE_V4.json remains unchanged. Its required actual native export witness necessarily invokes the product's existing Downloads resolver: apps/desktop/src-tauri/src/lib.rs calls app.path().download_dir(), and native_result_download.rs saves collision-safe new files there.

Under the already approved synthetic native delivery workflow, foundation_native may create, read and copy only the new result/stress export files produced by its own V4 UI actions in that resolved directory. Bind each file to the actual V4 action, returned filename/receipt or native download observation, byte count and checksum before using it as evidence. The JSON filename families are openpipestress-preview-results-*.json and openpipestress-preview-stress-neutral-*.json. The owned stress CSV delivery may use its actual product-suggested filename; record it exactly. This is not permission to inspect or alter unrelated Downloads contents.

Preserve existing files and collision-safe behavior. If a CSV save path could replace an existing file, choose a unique owned name through the actual native UI or return the condition to Root. Copy owned completed outputs into the V4 scratch/evidence scope for verification. The original-file versus copied-artifact attribution must remain explicit; a typed receipt and copied bytes are not a claim that an inaccessible original inode was independently read.

This clarification grants no source change, transport change or broad Downloads write scope. Preserve failures and complete normal quit and owned-process release under the original lease.

