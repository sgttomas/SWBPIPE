export default {
  root: "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/apps/desktop",
  test: {
    environment: "jsdom",
    globals: true,
    setupFiles: ["./src/test/setup.ts"],
    include: [
      "/Users/ryan/.codex/worktrees/8728/chirality-physics-ui-implementation-20260908/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-IMPLEMENTATION/instances/RU/successors/RU-F1-F5-BACKCHECK/_run_records/repros/**/*.test.tsx"
    ],
    testTimeout: 30000,
    hookTimeout: 30000
  }
};
