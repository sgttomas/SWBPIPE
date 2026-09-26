# Generic change workflow; chirality-change carries the repository rules

Owner-authorized Root tranche `ROOT-CHANGE-GENERIC-20260926` narrows the bundled `change` workflow into a repository-agnostic Git method. Without authorization the user or the repository's instructions actually give, it does not push, open or update a pull request, or merge, and it names how repository conventions specialize it. It drops the `codex/` branch-prefix default and renames the concurrency class `SHARED_MONOREPO` to `SHARED_CHECKOUT`.

The Chirality-specific rules it carried now live in `.agents/skills/chirality-change/SKILL.md`, which this loop's `AGENTS.md` already names for scoped Git closeout: the standing Git grant and its merge conditions (required CI, independent review of the actual candidate, source HEAD verification, PR/source/merge revisions recorded), the `codex/` prefix, no-fast-forward PR integration, high-risk overlap paths, and the grant's limits and destructive-action exclusions. The standing grant itself is unchanged.

Historical run records are not rewritten. This loop decides its own adoption; this source tranche grants no release.
