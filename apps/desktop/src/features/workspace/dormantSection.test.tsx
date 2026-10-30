import { act, fireEvent, render, screen } from "@testing-library/react";
import { useEffect, useState } from "react";
import { describe, expect, it } from "vitest";
import { DormantSection } from "./dormantSection";

describe("DormantSection", () => {
  it("retains drafts, skips inactive reconciliation, refreshes on activation, and advances session guards", () => {
    let renders = 0;
    let publishCompletion: (() => void) | null = null;

    function Probe({ modelRevision, sessionGeneration, guardGeneration }: { modelRevision: number; sessionGeneration: number; guardGeneration: number }) {
      renders += 1;
      const [draft, setDraft] = useState("");
      const [completion, setCompletion] = useState("none");
      useEffect(() => {
        let current = true;
        publishCompletion = () => {
          if (current) setCompletion(`session:${sessionGeneration}:guard:${guardGeneration}`);
        };
        setDraft("");
        return () => {
          current = false;
        };
      }, [guardGeneration, sessionGeneration]);
      return <>
        <output data-testid="revision">{modelRevision}</output>
        <output data-testid="completion">{completion}</output>
        <input aria-label="Retained draft" value={draft} onChange={(event) => setDraft(event.target.value)} />
      </>;
    }

    const section = (active: boolean, sessionGeneration: number, modelRevision: number, guardGeneration = 1) => (
      <DormantSection active={active} guardGeneration={guardGeneration} sessionGeneration={sessionGeneration}>
        <Probe guardGeneration={guardGeneration} modelRevision={modelRevision} sessionGeneration={sessionGeneration} />
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

    const preGuardCompletion = publishCompletion!;
    view.rerender(section(false, 1, 4, 2));
    act(() => preGuardCompletion());
    expect(screen.getByTestId("completion")).toHaveTextContent("none");
    act(() => publishCompletion!());
    expect(screen.getByTestId("completion")).toHaveTextContent("session:1:guard:2");

    view.rerender(section(false, 2, 5, 3));
    expect(screen.getByLabelText("Retained draft")).toHaveValue("");
    act(() => firstSessionCompletion());
    expect(screen.getByTestId("completion")).toHaveTextContent("session:1:guard:2");
    act(() => publishCompletion!());
    expect(screen.getByTestId("completion")).toHaveTextContent("session:2:guard:3");
  });

  it("keeps inactive report computation dormant while its owner rejects an obsolete completion", async () => {
    let renders = 0;
    let ownerGeneration = 1;
    let resolveRequest!: () => void;
    const request = new Promise<void>((resolve) => {
      resolveRequest = resolve;
    });

    function ReportProbe({ modelRevision }: { modelRevision: number }) {
      renders += 1;
      const [completion, setCompletion] = useState("none");
      useEffect(() => {
        const capturedGeneration = ownerGeneration;
        void request.then(() => {
          if (capturedGeneration === ownerGeneration) setCompletion(`revision:${modelRevision}`);
        });
      }, [modelRevision]);
      return <output data-testid="report-completion">{completion}</output>;
    }

    const section = (active: boolean, modelRevision: number) => (
      <DormantSection active={active} sessionGeneration={1}>
        <ReportProbe modelRevision={modelRevision} />
      </DormantSection>
    );
    const view = render(section(true, 1));
    const activeRenderCount = renders;
    view.rerender(section(false, 2));
    ownerGeneration = 2;
    view.rerender(section(false, 3));
    expect(renders).toBe(activeRenderCount);

    await act(async () => resolveRequest());
    expect(screen.getByTestId("report-completion")).toHaveTextContent("none");
  });
});
