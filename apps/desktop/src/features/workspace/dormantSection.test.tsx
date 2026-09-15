import { act, fireEvent, render, screen } from "@testing-library/react";
import { useEffect, useState } from "react";
import { describe, expect, it } from "vitest";
import { DormantSection } from "./dormantSection";

describe("DormantSection", () => {
  it("retains drafts, skips inactive reconciliation, refreshes on activation, and advances session guards", () => {
    let renders = 0;
    let publishCompletion: (() => void) | null = null;

    function Probe({ modelRevision, sessionGeneration }: { modelRevision: number; sessionGeneration: number }) {
      renders += 1;
      const [draft, setDraft] = useState("");
      const [completion, setCompletion] = useState("none");
      useEffect(() => {
        let current = true;
        publishCompletion = () => {
          if (current) setCompletion(`session:${sessionGeneration}`);
        };
        setDraft("");
        return () => {
          current = false;
        };
      }, [sessionGeneration]);
      return <>
        <output data-testid="revision">{modelRevision}</output>
        <output data-testid="completion">{completion}</output>
        <input aria-label="Retained draft" value={draft} onChange={(event) => setDraft(event.target.value)} />
      </>;
    }

    const section = (active: boolean, sessionGeneration: number, modelRevision: number) => (
      <DormantSection active={active} sessionGeneration={sessionGeneration}>
        <Probe modelRevision={modelRevision} sessionGeneration={sessionGeneration} />
      </DormantSection>
    );
    const view = render(section(true, 1, 1));
    fireEvent.change(screen.getByLabelText("Retained draft"), { target: { value: "keep me" } });
    const firstSessionCompletion = publishCompletion!;
    const activeRenderCount = renders;

    view.rerender(section(false, 1, 2));
    view.rerender(section(false, 1, 3));
    expect(renders).toBe(activeRenderCount);
    expect(screen.getByTestId("revision")).toHaveTextContent("1");
    expect(screen.getByLabelText("Retained draft")).toHaveValue("keep me");

    view.rerender(section(true, 1, 3));
    expect(screen.getByTestId("revision")).toHaveTextContent("3");
    expect(screen.getByLabelText("Retained draft")).toHaveValue("keep me");

    view.rerender(section(false, 2, 4));
    expect(screen.getByLabelText("Retained draft")).toHaveValue("");
    act(() => firstSessionCompletion());
    expect(screen.getByTestId("completion")).toHaveTextContent("none");
    act(() => publishCompletion!());
    expect(screen.getByTestId("completion")).toHaveTextContent("session:2");
  });
});
