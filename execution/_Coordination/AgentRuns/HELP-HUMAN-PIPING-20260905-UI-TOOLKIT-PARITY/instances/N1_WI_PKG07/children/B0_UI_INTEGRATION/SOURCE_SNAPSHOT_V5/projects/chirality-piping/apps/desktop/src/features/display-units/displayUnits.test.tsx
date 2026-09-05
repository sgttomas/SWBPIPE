import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { DisplayUnitsProvider, DisplayUnitSelector, QuantityReadout } from "./index";
import type { DisplayQuantityRequest, DisplayQuantityResult } from "../../services/displayQuantityService";

const length = { value: 1, unit: "m", dimension_id: "length" };
describe("display units presentation", () => {
  it("dispatches the accepted US area alias for volume per length and displays the Rust result", async () => {
    const quantity = Object.freeze({ value: 0.001, unit: "m^3/m", dimension_id: "volume_per_length" });
    const converter = vi.fn(async (items: DisplayQuantityRequest[]): Promise<DisplayQuantityResult[]> => items.map(item => ({ id: item.id, status: "converted", value: 1.5500031000062, unit: item.to_unit })));
    render(<DisplayUnitsProvider initialPreference="US" converter={converter}><QuantityReadout quantity={quantity} /></DisplayUnitsProvider>);
    await screen.findByText("1.5500031000062 in^2");
    expect(converter).toHaveBeenCalledTimes(1);
    expect(converter.mock.calls[0][0]).toEqual([expect.objectContaining({ value: 0.001, from_unit: "m^3/m", to_unit: "in^2", dimension_id: "volume_per_length" })]);
    expect(screen.queryByText(/Entered value shown/)).not.toBeInTheDocument();
    expect(quantity).toEqual({ value: 0.001, unit: "m^3/m", dimension_id: "volume_per_length" });
  });
  it("batches explicit temperature kinds and preserves all source state", async () => {
    const state = Object.freeze({ model: Object.freeze({ quantity: Object.freeze(length) }), hash: "original-hash", draft: "1.000 m" });
    const before = JSON.stringify(state);
    const converter = vi.fn(async (items: DisplayQuantityRequest[]): Promise<DisplayQuantityResult[]> => items.map(item => ({ id: item.id, status: "converted", value: item.dimension_id === "temperature" ? 32 : 18, unit: item.to_unit })));
    render(<DisplayUnitsProvider initialPreference="US" converter={converter}><DisplayUnitSelector />
      <QuantityReadout quantity={{value: 0, unit: "degC", dimension_id: "temperature"}} />
      <QuantityReadout quantity={{value: 10, unit: "degC", dimension_id: "temperature_interval"}} />
      <QuantityReadout quantity={state.model.quantity} />
    </DisplayUnitsProvider>);
    await screen.findByText("32 degF");
    expect(converter).toHaveBeenCalledTimes(1);
    expect(converter.mock.calls[0][0]).toEqual(expect.arrayContaining([
      expect.objectContaining({ dimension_id: "temperature", to_unit: "degF" }),
      expect.objectContaining({ dimension_id: "temperature_interval", to_unit: "degF" })
    ]));
    fireEvent.change(screen.getByLabelText("Display units"), {target: {value: "SI"}});
    await screen.findByText("32 K");
    expect(converter.mock.calls[1][0]).toEqual(expect.arrayContaining([
      expect.objectContaining({dimension_id: "temperature", to_unit: "K"}),
      expect.objectContaining({dimension_id: "temperature_interval", to_unit: "K"})
    ]));
    expect(JSON.stringify(state)).toBe(before);
  });
  it("rejects late preference and late source responses", async () => {
    const pending: {items: DisplayQuantityRequest[]; resolve: (items: DisplayQuantityResult[]) => void}[] = [];
    const converter = vi.fn((items: DisplayQuantityRequest[]) => new Promise<DisplayQuantityResult[]>(resolve => pending.push({items, resolve})));
    const view = (value: number) => <DisplayUnitsProvider initialPreference="US" converter={converter}><DisplayUnitSelector /><QuantityReadout quantity={{...length, value}} /></DisplayUnitsProvider>;
    const result = render(view(1));
    await waitFor(() => expect(pending).toHaveLength(1));
    fireEvent.change(screen.getByLabelText("Display units"), {target: {value: "SI"}});
    await waitFor(() => expect(pending).toHaveLength(2));
    result.rerender(view(2));
    await waitFor(() => expect(pending).toHaveLength(3));
    await act(async () => {
      for (const [index, request] of pending.entries()) request.resolve([{id: request.items[0].id, status:"converted", value:index === 2 ? 2 : 999, unit:request.items[0].to_unit}]);
    });
    expect(screen.getByText("2 m")).toBeInTheDocument();
    expect(screen.queryByText(/999/)).not.toBeInTheDocument();
  });
  it("shows entered quantities and accessible failure notices without inventing dimensions", async () => {
    const converter = vi.fn(async (): Promise<DisplayQuantityResult[]> => {throw new Error("offline");});
    render(<DisplayUnitsProvider initialPreference="US" converter={converter}>
      <QuantityReadout quantity={length} />
      <QuantityReadout quantity={{value:12, unit:"mystery", dimension_id:"TBD"}} />
      <QuantityReadout quantity={{value:"TBD", unit:"m", dimension_id:"length"}} />
    </DisplayUnitsProvider>);
    await screen.findByText(/service unavailable/);
    expect(screen.getByText(/no US catalog target for TBD/)).toHaveAttribute("role", "status");
    expect(screen.getByText(/not a finite numeric value/)).toHaveAttribute("role", "status");
    expect(converter.mock.calls).toHaveLength(1);
  });
  it("does not dispatch conversions in entered mode and uses a safe default without a provider", () => {
    const converter = vi.fn();
    render(<><DisplayUnitsProvider converter={converter}><QuantityReadout quantity={length} /></DisplayUnitsProvider><QuantityReadout quantity={{...length, value:2}} /></>);
    expect(screen.getByText("1 m")).toBeInTheDocument();
    expect(screen.getByText("2 m")).toBeInTheDocument();
    expect(converter).not.toHaveBeenCalled();
  });
  it("deduplicates identical readouts and rejects mismatched converter units", async () => {
    const converter = vi.fn(async (items: DisplayQuantityRequest[]): Promise<DisplayQuantityResult[]> => items.map(item => ({ id:item.id, status:"converted", value:123, unit:"psi" })));
    render(<DisplayUnitsProvider initialPreference="US" converter={converter}><QuantityReadout quantity={length} /><QuantityReadout quantity={length} /></DisplayUnitsProvider>);
    await waitFor(() => expect(screen.getAllByText(/invalid conversion response/)).toHaveLength(2));
    expect(converter.mock.calls[0][0]).toHaveLength(1);
    expect(screen.queryByText(/123/)).not.toBeInTheDocument();
  });
});
