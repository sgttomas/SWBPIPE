import { createContext, useContext } from "react";
import type { ReactNode } from "react";
import type { WorkspaceSession } from "./workspaceSession";

// The session, handed to the shell's components. `AppSession` calls
// `useWorkspaceSession()` once and wraps its JSX in the provider; a component
// reads the slice it needs through one of the hooks below and never calls a
// state hook itself (the import guard test holds that).
//
// A slice object is new on every render of `AppSession`, and so is the session.
// A consumer therefore re-renders whenever `AppSession` does, as every
// unmemoized child of it already did, and no effect or memo lists the session
// or a slice as a dependency: it lists the cells it reads.

const WorkspaceSessionContext = createContext<WorkspaceSession | null>(null);

export function WorkspaceSessionProvider({ session, children }: { session: WorkspaceSession; children: ReactNode }) {
  return <WorkspaceSessionContext.Provider value={session}>{children}</WorkspaceSessionContext.Provider>;
}

/** The whole session. Prefer a slice hook; a component should name what it reads. */
export function useWorkspaceSessionContext(): WorkspaceSession {
  const session = useContext(WorkspaceSessionContext);
  if (!session) throw new Error("WORKSPACE-SESSION-CONTEXT-ABSENT: no WorkspaceSessionProvider above this component");
  return session;
}

export function useSessionModel(): WorkspaceSession["model"] {
  return useWorkspaceSessionContext().model;
}

export function useSessionSelection(): WorkspaceSession["selection"] {
  return useWorkspaceSessionContext().selection;
}

export function useSessionResults(): WorkspaceSession["results"] {
  return useWorkspaceSessionContext().results;
}

export function useSessionOperations(): WorkspaceSession["operations"] {
  return useWorkspaceSessionContext().operations;
}

export function useSessionProject(): WorkspaceSession["project"] {
  return useWorkspaceSessionContext().project;
}

export function useSessionChrome(): WorkspaceSession["chrome"] {
  return useWorkspaceSessionContext().chrome;
}
