import { act, render } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import type { PackageHashEvidence } from "../types";
import { usePackageHash } from "./usePackageHash";

// Every package hash comes from a computation the test resolves by hand.
type Pending = {
  packageId: string;
  packet: unknown;
  payloadExcludes: PackageHashEvidence["payload_excludes"];
  resolve: (hash: PackageHashEvidence) => void;
};
const pending = vi.hoisted((): Pending[] => []);

vi.mock("./hashService", async (importOriginal) => ({
  ...(await importOriginal<typeof import("./hashService")>()),
  computePackageHash: vi.fn(
    (packageId: string, packet: unknown, payloadExcludes: PackageHashEvidence["payload_excludes"]) =>
      new Promise<PackageHashEvidence | null>((resolve) => {
        pending.push({ packageId, packet, payloadExcludes, resolve });
      })
  )
}));

function hashFor(label: string): PackageHashEvidence {
  return {
    algorithm: "sha256",
    canonicalization: "rfc8785_jcs",
    payload_scope: "package_review_payload",
    payload_excludes: "validation_report_package_hash_fields",
    payload_ref: label,
    value: `sha256:${label}`,
    hash_status: "computed_local_preview"
  };
}

type Inputs = {
  label: string;
  packageId: string | null;
  packet: unknown;
  payloadExcludes?: PackageHashEvidence["payload_excludes"];
};

// One entry per render of the hook: which inputs it was given and what it returned.
const seen: Array<{ label: string; value: string | null }> = [];

function Probe({ label, packageId, packet, payloadExcludes }: Inputs) {
  const hash = usePackageHash(packageId, packet, payloadExcludes);
  seen.push({ label, value: hash?.value ?? null });
  return null;
}

/** The latest computation for these exact inputs, resolved with `label`'s hash. */
async function resolveLatest(inputs: Inputs, label: string) {
  const match = [...pending]
    .reverse()
    .find(
      (item) =>
        item.packageId === inputs.packageId &&
        JSON.stringify(item.packet) === JSON.stringify(inputs.packet) &&
        item.payloadExcludes === (inputs.payloadExcludes ?? "validation_report_package_hash_fields")
    );
  expect(match, `a computation for ${inputs.label}`).toBeDefined();
  await act(async () => match!.resolve(hashFor(label)));
}

function rendersOf(label: string) {
  return seen.filter((entry) => entry.label === label).map((entry) => entry.value);
}

const PACKET_A = { package_id: "pkg", members: ["a.json"] };
const PACKET_B = { package_id: "pkg", members: ["b.json"] };
const A: Inputs = { label: "A", packageId: "pkg", packet: PACKET_A };

beforeEach(() => {
  pending.length = 0;
  seen.length = 0;
});

describe("usePackageHash binds a hash to the inputs it was computed from (issue 497)", () => {
  it("returns null, never the previous packet's hash, on the render in which the packet changes", async () => {
    const view = render(<Probe {...A} />);
    await resolveLatest(A, "A");
    expect(seen.at(-1)).toEqual({ label: "A", value: "sha256:A" });

    const B: Inputs = { ...A, label: "B", packet: PACKET_B };
    view.rerender(<Probe {...B} />);
    expect(rendersOf("B").length).toBeGreaterThan(0);
    expect(rendersOf("B")).not.toContain("sha256:A");
    expect(rendersOf("B")[0]).toBeNull();

    await resolveLatest(B, "B");
    expect(rendersOf("B")).toEqual([...rendersOf("B").filter((value) => value === null), "sha256:B"]);
  });

  it.each([
    ["package id", { packageId: "pkg-other" }],
    ["payload excludes", { payloadExcludes: "manifest_and_validation_report_package_hash_carrier_fields" as const }]
  ])("returns null on the render in which the %s changes with the same packet", async (_name, change) => {
    const view = render(<Probe {...A} />);
    await resolveLatest(A, "A");

    const C: Inputs = { ...A, ...change, label: "C" };
    view.rerender(<Probe {...C} />);
    expect(rendersOf("C").length).toBeGreaterThan(0);
    expect(rendersOf("C")).not.toContain("sha256:A");

    await resolveLatest(C, "C");
    expect(rendersOf("C").at(-1)).toBe("sha256:C");
    expect(rendersOf("C").filter((value) => value !== null)).toEqual(["sha256:C"]);
  });

  it("does not surface a computation that resolves after its inputs changed, and does surface the new one", async () => {
    const view = render(<Probe {...A} />);
    const B: Inputs = { ...A, label: "B", packet: PACKET_B };
    view.rerender(<Probe {...B} />);

    await resolveLatest(A, "A");
    expect(rendersOf("B")).not.toContain("sha256:A");
    expect(seen.at(-1)).toEqual({ label: "B", value: null });

    await resolveLatest(B, "B");
    expect(seen.at(-1)).toEqual({ label: "B", value: "sha256:B" });
    expect(rendersOf("B").filter((value) => value !== null)).toEqual(["sha256:B"]);
  });

  it("keeps the new hash when the earlier computation resolves after it", async () => {
    const view = render(<Probe {...A} />);
    const B: Inputs = { ...A, label: "B", packet: PACKET_B };
    view.rerender(<Probe {...B} />);

    await resolveLatest(B, "B");
    expect(seen.at(-1)).toEqual({ label: "B", value: "sha256:B" });
    const rendersBeforeLateA = seen.length;
    await resolveLatest(A, "A");
    expect(seen.slice(rendersBeforeLateA)).toEqual([]);
    view.rerender(<Probe {...B} />);
    expect(seen.at(-1)).toEqual({ label: "B", value: "sha256:B" });
  });

  it.each([
    ["after B's hash resolved", true],
    ["while B's hash is pending", false]
  ])("going back to A %s returns null until A's hash is computed again", async (_name, resolveB) => {
    const view = render(<Probe {...A} />);
    await resolveLatest(A, "A");
    const B: Inputs = { ...A, label: "B", packet: PACKET_B };
    view.rerender(<Probe {...B} />);
    if (resolveB) await resolveLatest(B, "B");

    const backToA: Inputs = { ...A, label: "A again" };
    const computationsBefore = pending.length;
    view.rerender(<Probe {...backToA} />);
    expect(rendersOf("A again").length).toBeGreaterThan(0);
    expect(rendersOf("A again").every((value) => value === null)).toBe(true);
    expect(pending.length).toBe(computationsBefore + 1);

    await resolveLatest(backToA, "A");
    expect(seen.at(-1)).toEqual({ label: "A again", value: "sha256:A" });
    expect(rendersOf("A again")).not.toContain("sha256:B");
  });

  it("returns null and computes nothing without a package id or a packet", () => {
    render(<Probe label="none" packageId={null} packet={PACKET_A} />);
    render(<Probe label="none" packageId="pkg" packet={null} />);
    expect(rendersOf("none").every((value) => value === null)).toBe(true);
    expect(pending).toHaveLength(0);
  });
});
