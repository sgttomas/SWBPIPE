# Slice B2: the manager's finding on the session object's identity

Lane B-SHELL's manager (WORKING_ITEMS, Type 1), 2026-09-19. ROOT asked the manager to read this itself before committing stage 3. It is the second named item for the independent reviewer of slice B2; the first is `B2_RENDER_ORDER_FINDING.md`. Candidate read: the stage-3 tree (`src/App.tsx` `fec6d96f65ef…`, `src/features/workspace/workspaceSession.ts` `d9956a177889…`), committed as stage 3 of the slice. Paths are relative to `{WORKING_ROOT}/apps/desktop`.

## The question

From stage 3, `useWorkspaceSession` builds a new object on every render: `{ model: {…}, selection: {…}, results: {…}, operations: {…}, project: {…}, chrome: {…} }`, and `AppSession` destructures it. The six state hooks each build a new object per render too. At the base none of these objects existed. Does any consumer depend on the identity of one of them (an effect dependency, a memo key, a prop passed whole), so that the new object causes an effect run or a re-render path the base did not have?

## Finding: no

1. **The session object has one consumer and it only destructures.** The identifier `session` occurs seven times in `src/App.tsx`: the call `const session = useWorkspaceSession();` (`:133`) and the six destructurings `} = session.model;` to `} = session.chrome;` (`:143`, `:157`, `:185`, `:215`, `:233`, `:263`). It is in no dependency array, no memo, no JSX attribute and no spread. Each slice object is read once, by its destructuring, and is not kept.
2. **Nothing else can reach it.** `useWorkspaceSession` is imported by `src/App.tsx` alone; the six state hooks are imported by `workspaceSession.ts` alone; the type `WorkspaceSession` has no user yet; the slice adds no React context.
3. **The state hooks' objects are not kept either.** In `workspaceSession.ts` each is destructured in the statement that calls the hook (`} = useChromeSessionState();` and the five like it); no variable holds a hook's return object.
4. **The return is names only.** Its 129 properties are all shorthand (`model,` not `model: something`), in six slices of 8, 14, 28, 28, 16 and 35; the manager checked that mechanically. So every value the view receives is the base's own value: a state value, a React setter (stable), a ref object (stable), a memo's value (same memo cell, same dependencies), or a handler. Handlers were function declarations in the component body at the base and had a new identity on every render; they are function declarations in the hook body now and have a new identity on every render. No identity became less stable, and none became more stable.
5. **Effects and memos.** The 13 effects and the memos are byte for byte the base's, dependency arrays included, and those arrays name cells, never the session object, a slice or a hook's return object.
6. **What re-renders.** `AppSession` is the only component that calls these hooks, so it re-renders on exactly the state changes that re-rendered it at the base; a custom hook adds no component and no subscription. Its JSX (#177) is byte for byte the base's, so every child receives the same props with the same identity behaviour as before.

The only new runtime work per render is allocating seven small objects in the session and six in the state hooks, and the destructurings.

## For B3

B3 will hand slices to shell components. A slice object is new on every render, so a component that takes a whole slice as a prop, or a context whose value is the session object, re-renders whenever `AppSession` does, and an effect that lists a slice as a dependency runs on every render. That is how every child of `AppSession` already behaves where it is not memoized, but B3 decides it per component, and no effect or memo lists a slice or the session as a dependency; they list the cells they read.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
