# Native quit and cleanup evidence

- Exact app: `OpenPipeStress Technical Preview Implementation Witness 55df51ac 20260908.app`
- Exact bundle identifier: `org.openpipestress.technical-preview.walkthrough-physics-ui-implementation-55df51ac-20260908`
- Native CUA close control invoked after evidence capture.
- Exact executable process check after close: absent.
- The pre-quit inventory contained one file. After native quit, SQLite exposed three owned files: main (`122880` bytes, SHA-256 `98d332e9080da291730fcb2e1dbe370bec20a9d0d53bd4dd43184e25f4ab372f`), shm (`32768` bytes, SHA-256 `fd4c9fda9cd3f9ae7c962b0ddf37232294d55580e1aa165aa06129b8549389eb`), and zero-byte wal (SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`).
- The first cleanup guard held without deleting anything because it expected one file and observed three. The corrected guard required the exact application-support parent, exact isolated identifier, exactly those three basenames, and regular nonsymlink files.
- Cleanup explicitly unlinked wal, shm, then main and removed only the now-empty exact isolated directory. No wildcard or recursive deletion was used.
- Cleanup status: complete; exact dedicated directory absent afterward.
- Normal application store: not enumerated, opened, hashed, moved, modified, or deleted.
