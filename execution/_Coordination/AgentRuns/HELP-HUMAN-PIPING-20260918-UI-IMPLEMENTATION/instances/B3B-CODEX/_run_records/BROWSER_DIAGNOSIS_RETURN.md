Diagnosis: Open invalidates its own evidence guard after committing the model.

- `PipeViewport` model and selection effects call `invalidateDirectDraftContext`, advancing `requestEpoch`.
- Open’s awaited hash finishes, but `stillCurrent()` rejects its observations because that epoch changed.
- Saved-basis publication survives because it uses generation/sequence guards.
- Project-panel navigation does not itself advance the epoch.

Minimal repair: change only Open’s **post-commit observation guards** to require the same request, project generation, and project identity. Preserve pre-commit model/history adoption guards.

Regression: delay Open hashing, invalidate draft context, then verify both opened-snapshot observations publish without adopting later edits. Rerun the unchanged browser journey afterward.

No edits or tests performed.
